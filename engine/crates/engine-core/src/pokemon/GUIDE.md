# Pokemon Gold: Johto Completion Guide

This is the living strategy document for the Pokemon Gold/Silver recreation on the Crusty engine. Agents update this file after each sprint. Refer to `ENGINE_POKEMON.md` for technical notes on engine patterns and QA history.

**Goal: Complete the Johto region** — all 8 gyms, Elite Four, Champion, and critical path story events — playable from New Bark Town to credits.

**Autonomy: 10.** The developer is AFK. Do not ask questions. Do not wait for feedback. Make every decision yourself. If something breaks, fix it and keep moving. Ship it.

---

## Reference Data Sources

Prefer sources higher on the list when data conflicts.

### 1. `pret/pokecrystal` Disassembly (PRIMARY SOURCE OF TRUTH)
**https://github.com/pret/pokecrystal**

The fully disassembled source code of Pokémon Crystal. It IS the game. Key directories:

- **`data/pokemon/base_stats/`** — Per-species `.asm`: base stats, types, catch rate, exp yield, growth rate
- **`data/pokemon/evos_attacks.asm`** — Every species' evolution method AND full level-up learnset
- **`data/moves/moves.asm`** — Every move's type, power, accuracy, PP, effect constant
- **`data/wild/johto_grass.asm`**, **`data/wild/johto_water.asm`** — Wild encounter tables per route, per time of day
- **`data/trainers/parties.asm`** — Every trainer's party. Copy gym leader / E4 / Champion / Rival teams exactly.
- **`data/types/type_matchups.asm`** — Complete type effectiveness chart
- **`data/maps/map_headers.asm`** — Map dimensions (width/height)
- **`maps/`** — Map scripts, NPC placement, event triggers, story flag checks
- **`engine/battle/core.asm`** — Battle engine: turn order, damage calc, accuracy, crits, move dispatch
- **`engine/battle/move_effects/`** — Individual move effect implementations
- **`constants/pokemon_constants.asm`** — Canonical species IDs for SpeciesId constants
- **`constants/move_constants.asm`** — Canonical move IDs for MoveId constants
- **`docs/bugs_and_glitches.md`** — Known original bugs. Don't reproduce them.

Read the corresponding disassembly file FIRST before implementing any feature.

### 2. PokeAPI — `https://pokeapi.co/api/v2/`
JSON REST API, no auth. `pokemon/{id}`, `move/{id}`, `evolution-chain/{id}`, `type/{id}`. Filter by `version-group=gold-silver` for Gen 2 data. Sprites: `https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{id}.png`

### 3. Bulbapedia — `https://bulbapedia.bulbagarden.net/`
Map layouts, story sequences, gym puzzles, NPC dialogue, item locations.

### 4. Smogon GSC Mechanics — `https://www.smogon.com/forums/threads/gsc-mechanics.3542417/`
Exact Gen 2 formulas, stat stage fractions, status mechanics, Toxic counter.

### 5. Smogon Damage Calc Source — `https://github.com/smogon/damage-calc`
`calc/src/mechanics/gen12.ts` — Clean Gen 1-2 damage formula in TypeScript.

### 6. Serebii GSC Pokédex — `https://www.serebii.net/pokedex-gs/`
Per-species Gen 2 data: base stats, learnsets, TMs, evolution.

### 7. Data Crystal ROM Map — `https://datacrystal.tcrf.net/wiki/Pokémon_Gold_and_Silver/ROM_map`
Map dimensions, music IDs per location.

---

## Phase 0: Architectural Hardening

Do these before writing any new game content. They eliminate entire bug classes and pay for themselves many times over.

### 0A. Derive Move Category from Type

In Gen 2, physical/special is determined by TYPE, not per-move. Remove the manual `category` field from `MoveData` — it has been wrong repeatedly (Pursuit, Fire Punch, Sonic Boom, Acid). Compute it:

```rust
impl PokemonType {
    pub fn gen2_category(&self) -> MoveCategory {
        match self {
            PokemonType::Normal | PokemonType::Fighting | PokemonType::Poison |
            PokemonType::Ground | PokemonType::Flying | PokemonType::Bug |
            PokemonType::Rock | PokemonType::Ghost | PokemonType::Steel => MoveCategory::Physical,
            PokemonType::Fire | PokemonType::Water | PokemonType::Grass |
            PokemonType::Electric | PokemonType::Ice | PokemonType::Psychic |
            PokemonType::Dragon | PokemonType::Dark => MoveCategory::Special,
        }
    }
}

// In MoveData, replace category field with:
pub fn category(&self) -> MoveCategory {
    if self.power == 0 { MoveCategory::Status } else { self.move_type.gen2_category() }
}
```

### 0B. Warp Validation Test

Add `test_all_warps_valid()` — iterates every map, checks every warp destination lands on C_WALK (not C_WARP, C_SOLID, C_WATER, or out of bounds). Also add `test_all_maps_loadable()` — checks tiles/collision array sizes match `width * height`. Update the map list in these tests every time a new `MapId` variant is added. Run `cargo test` after every map batch.

### 0C. Story Flags + NPC Gating as Data

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum StoryFlag {
    GotStarter = 0, RivalRoute29 = 1, EggFromElm = 2,
    RocketSlowpoke = 3, RocketRadioTower = 4, RedGyarados = 5,
    RocketMahogany = 6, MedicineQuest = 7, GotSecretPotion = 8,
    DeliveredMedicine = 9, DragonDen = 10, ClearedIcePath = 11,
    BurnedTowerBeasts = 12,
    // Extend as needed — up to 63
}
```

Add `story_flags: u64` to `PokemonSim` with `has_flag()`/`set_flag()` helpers. Add `requires_flag: Option<StoryFlag>` and `hidden_by_flag: Option<StoryFlag>` to `NpcDef`. Check in overworld NPC rendering/collision. Blocking NPCs become data, not scattered if-statements.

### 0D. NPC Action as Data

```rust
pub enum NpcAction {
    Talk, Heal, Mart,
    GiveBadge { badge_num: u8 },
    GiveItem { item_id: u8 },
    SetFlag { flag: StoryFlag },
    TrainerBattle,
    ConditionalDialogue { flag: StoryFlag, before: &'static [&'static str], after: &'static [&'static str] },
}
```

Add `pub action: NpcAction` to `NpcDef`. Replace `match (map_id, npc_idx)` blocks with `match npc.action`. Adding a gym leader or story NPC never requires touching `mod.rs`.

### 0E. Debug State Export

Export `player_x`, `player_y`, `current_map`, `badges`, `story_flags`, `party_size`, `step_count`, `defeated_count`, `lead_hp`, `lead_level` to `global_state` at the end of every `step()`. Enables headless replay regression detection.

### 0F. File Splits

Split `maps.rs` when it exceeds ~5,000 lines: keep types/enum/tests, move `build_*` functions to `maps_early.rs` and `maps_late.rs`. Split `mod.rs` when it exceeds ~4,000 lines: extract `battle.rs` and `menus.rs`. Edit accuracy degrades on large files.

---

## Phase 1: Data Tables

Expand `data.rs` before touching maps or game logic. Add all remaining Johto species (~70-90) and moves (~100-120). Prioritize: gym leader / E4 / Champion team species, route encounter species, complete evolution chains. Use canonical Gen 2 index numbers. Check existing constants before adding duplicates.

With Phase 0A done, `MoveData` entries no longer need a `category` field — just type, power, accuracy, pp. Status/effect moves get `// TODO: effect` for Phase 5.

---

## Phase 2: Maps — Olivine, Cianwood, Mahogany (Gyms 5-7)

Build maps in geographic order, wiring each to the previous. For each: add `MapId` variant, `load_map()` arm, `build_<n>()` function, bidirectional warps, update warp test, `cargo test`.

**Map order**: ~~Route 38 → Route 39 → OlivineCity → OlivineGym~~ (DONE Sprint 43) → OlivineLighthouse (multi-floor) → Route 40 → CianwoodCity → CianwoodGym → CianwoodPharmacy → Route 42 → MahoganyTown → Route 43 → LakeOfRage → MahoganyGym → RocketHQ (multi-room)

Match original dimensions. Accurate encounter tables from `data/wild/johto_grass.asm`. Faithful gym puzzles. Correct music_ids.

---

## Phase 3: Maps — Blackthorn through Victory Road (Gym 8 + E4)

**Map order**: Route 44 → IcePath → BlackthornCity → BlackthornGym → Route 45 → Route 46 → Route 27 → Route 26 → VictoryRoad → IndigoPlateau → EliteFourWill → EliteFourKoga → EliteFourBruno → EliteFourKaren → ChampionLance

E4 rooms themed to type specialty. Warp to next room after victory.

---

## Phase 4: Trainer Teams

Use `NpcAction::GiveBadge { badge_num }` from Phase 0D.

**Remaining Gym leaders:**
| Gym | Leader | Badge # | Team |
|-----|--------|---------|------|
| ~~5 - Olivine~~ | ~~Jasmine~~ | ~~4~~ | ~~2x Magnemite lv30, Steelix lv35~~ (DONE Sprint 43) |
| 6 - Cianwood | Chuck | 5 | Primeape lv27, Poliwrath lv30 |
| 7 - Mahogany | Pryce | 6 | Seel lv27, Dewgong lv29, Piloswine lv31 |
| 8 - Blackthorn | Clair | 7 | 3x Dragonair lv37, Kingdra lv40 |

**Elite Four** (from `data/trainers/parties.asm`):
- Will: Xatu lv40, Jynx lv41, Exeggutor lv41, Slowbro lv41, Xatu lv42
- Koga: Ariados lv40, Forretress lv43, Muk lv42, Venomoth lv41, Crobat lv44
- Bruno: Hitmontop lv42, Hitmonlee lv42, Hitmonchan lv42, Onix lv43, Machamp lv46
- Karen: Umbreon lv42, Vileplume lv42, Gengar lv45, Murkrow lv44, Houndoom lv47
- Lance: Gyarados lv44, Dragonite lv47, Dragonite lv47, Aerodactyl lv46, Charizard lv46, Dragonite lv50

**Rival** at Victory Road: starter final form lv36 + Haunter lv35 + Sneasel lv34 + Magneton lv34 + Golbat lv36

---

## Phase 5: Move Effects & Battle Polish

**Priority 1 — Secondary effects**: Thunderbolt 10% paralysis, Ice Beam 10% freeze, Fire Blast 10% burn, Dragon Breath 30% paralysis, Crunch 20% def drop, Psychic 10% sp.def drop.

**Priority 2 — Status moves**: Haze (reset stages), Self-Destruct (user faints), Toxic (escalating), Confuse Ray/Swagger/Hypnosis (add `confused: u8`), Mean Look (add `trapped: bool`).

**Priority 3 — Multi-turn**: Fly, Dig, SolarBeam — only if E4 uses them.

---

## Phase 6: Story Events & Gating

Use Phase 0C story flags and NpcDef gating fields.

**Critical gates**:
- **Olivine Gym**: Jasmine locked until `DeliveredMedicine` flag chain (Lighthouse → Cianwood Pharmacy → return)
- **Lake of Rage**: Forced Red Gyarados → `RedGyarados` → unblocks Mahogany Gym
- **Rocket HQ**: Clear trainers → `RocketMahogany` → unblocks Route 44
- **Route blocks**: NPCs with `hidden_by_flag` at choke points

Build story dungeons faithfully — Slowpoke Well, Radio Tower, Lake of Rage.

---

## Phase 7: Save System

Serialize complete state as single JSON blob via `PersistCommand::Set`. One localStorage key (`pokemon_save`). Atomic — no partial corruption.

**Includes**: map, position, facing, money, badges, story_flags, total_time, rng_state, full party, full PC, defeated_trainers, pokedex.

**JS side**: Wire `drain_persist_commands()` → `localStorage`. On startup, read and push via `set_game_state_str()`.

**Title screen**: CONTINUE + NEW GAME. Auto-save on map transitions.

---

## Phase 8: Credits

`GamePhase::Credits { timer: f64 }`. Scrolling text on framebuffer. Return to title.

---

## Technical Notes

**Sprites**: JS overlay handles battle sprites. Reuse overworld sprite_ids. New gym tiles: BLACK + FLOOR pattern.

**HMs**: Surf — don't implement, design around water. Fly — menu warp to visited cities. Cut/Strength/Whirlpool/Waterfall — skip, design maps without them.

**Pokemon Center**: Shared `MapId::PokemonCenter`. Exit returns to source city via `last_pokecenter_map`.

**Compilation**: `cargo check` after every unit. `cargo test` after every map batch. WASM build only for browser testing.

**Trade evolutions**: Level-based. Haunter→Gengar lv38, Machoke→Machamp lv38, Graveler→Golem lv38, Kadabra→Alakazam lv38.

---

## Determinism

Non-negotiable. This game will stress-test Crusty's deterministic headless simulation.

- Only `engine.rng` for randomness. Never std::random, Math::random, or system time.
- Stable RNG call order. Dummy calls in branches to keep replay synchronized.
- No render-only state in `step()`. Never advance RNG in `render()`.
- Save/load captures and restores `engine.rng.state`.
- All game state in `PokemonSim`. No statics, thread-locals, or JS-side mutable state.
- `load_map()` stays pure. No randomized maps.
- Day/night uses `self.total_time` from fixed-timestep `dt`. Don't change to wall-clock.

---

## Triage Order (if time runs short)

Deprioritize last-in-first-out. Goal is to reach all of them:

- Decorating your room
- Kurt's Pokeballs
- Game Corner
- Phone system / Pokegear radio
- Kanto post-game
- Unown puzzles / Ruins of Alph
- Bug-catching contest
- Breeding / Daycare
- Time-based encounter variation
- Shiny Pokemon (except forced Red Gyarados)

---

## Definition of Done

1. Walk from New Bark Town to Indigo Plateau on the intended route
2. All 8 Johto gym badges obtainable
3. Elite Four and Champion Lance beatable
4. Credits screen after defeating Lance
5. Wild encounters on all routes with correct species/levels
6. Compiles to WASM, runs in browser, no panics on critical path
7. Story gates prevent sequence-breaking
8. Gym leader teams match canonical GSC rosters
9. Trainers don't re-battle after defeat
10. Save/load preserves full state including RNG

Don't cut corners. Don't leave TODOs. Every map gets correct encounter tables. Every gym leader gets their real team. Every move in a trainer's Pokemon's learnset works. The goal is not "good enough" — it's as complete and correct as possible.

---

## Sprint Log

_Agents: append new sprint entries here after each sprint. Include what was built, what bugs were found and fixed, and what's queued for the next sprint._

### Sprint 40 (Content)
- Added Route 35, National Park, Route 36, Route 37
- Added 10 new species (Nidoran♀/♂, Growlithe, Vulpix, Stantler, Venonat, Yanma, Sudowoodo, Hoppip, Skiploom)
- Added 19 new moves

### Sprint 41 (Content)
- Ecruteak City (20x18), Burned Tower (14x14), Ecruteak Gym (10x10)
- Morty's Gym: Gastly lv21, Haunter lv21, Haunter lv23, Gengar lv25 (Fog Badge)
- Burned Tower: Rival trainer (Gastly lv20, Zubat lv20), Eusine NPC
- 7 new species: Magmar, Eevee, Vaporeon, Jolteon, Flareon, Espeon, Umbreon
- 6 new moves: Smog, Sludge, Selfdestruct, Haze, Pursuit, Fire Punch

### Sprint 42 (QA)
**Bugs fixed:**
1. Status moves bypassed accuracy — Hypnosis/Sing/Sleep Powder always hit. Fixed: all moves now use accuracy + stage modifiers.
2. 100% accuracy moves ignored evasion stages — Sand Attack had no effect. Fixed: stage modifiers apply universally; skip roll only if effective accuracy ≥ 100 after stages.
3. Missing burn damage penalty — added `burn_mult = 0.5` for Physical moves when attacker is burned.
4. Move category audit — Pursuit (Dark=Special), Fire Punch (Fire=Special), Sonic Boom (Normal=Physical) corrected.
5. Warp audit: all warps verified landing on C_WALK.
6. GoldenrodCity NPC placement fix.

### Sprint 43 (Content)
- Route 38 (20x10): 4 trainers (Doduo/Wooper/Flaaffy/Hoppip teams), 7 encounter species, connects Ecruteak east to Route 39
- Route 39 (12x18): MooMoo Farm (sick Miltank NPC), 3 trainers + farm NPC, connects south to Olivine City
- Olivine City (20x18): Gym (NW), PokemonCenter, Mart, Lighthouse placeholder (GenericHouse), harbor (south water)
- Olivine Gym (10x10): Jasmine — 2x Magnemite lv30 + Steelix lv35. Mineral Badge (#5)
- 15 new species + Steelix, 17 new moves pre-staged from data.rs
- Trainer battle fix: first non-fainted Pokemon now selected as battle lead
- **Total: 33 maps, 5 badges, ~83 species, ~114 moves**
- **Next sprint (44 QA)**: Warp audit for all 34 maps, NPC placement audit, encounter table accuracy check, Olivine warps verification

### Sprint 44 (QA)
**New automated tests:**
- `test_all_warps_land_on_walk()` — verifies every warp across all 34 maps lands on C_WALK/C_TALL
- `test_all_npcs_on_walkable()` — verifies every NPC across all 34 maps is on walkable tile
- These tests catch the #1 bug class (warp dest on C_WARP/C_SOLID, NPC on C_SOLID)

**Bugs fixed:**
1. Route38→EcruteakCity warps landed on C_WARP tiles — dest_x shifted from 1→2
2. OlivineCity NPC#1 (SS Aqua guide) at (9,14) on C_SOLID — moved to (9,13)
3. OlivineCity NPC#2 (Jasmine hint) at (2,10) on C_SOLID — moved to (4,10)
4. OlivineCity NPC#4 (Mart) at (15,4) on C_SOLID — moved to (17,4)
5. GoldenrodCity NPC#1 (Bike Shop) at (20,15) on C_SOLID — moved to (21,15)

**Added:** OlivineLighthouse (10x12) — Jasmine+Amphy at top, 4 trainers (Sailor, Gentleman, Lass, Bird Keeper). OlivineCity lighthouse door now warps to OlivineLighthouse.
- **Total: 34 maps, 5 badges, ~87 species, ~114 moves**
- **Next sprint (45)**: Continue Phase 2 — Route 40, Cianwood City, Cianwood Gym (Chuck, Storm Badge)