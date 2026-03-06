# Pokemon Gold: Johto Completion Sprint

You are working on a Pokemon Gold/Silver recreation built on the Crusty engine (Rust/WASM). The game already has ~11,800 lines of working Rust in `engine/crates/engine-core/src/pokemon/` implementing:

**Current inventory (as of Sprint 42 QA):**
- **68 species** with full base stats, learnsets, evolution data
- **97 moves** with type/category/power/accuracy/pp
- **29 maps** (New Bark Town through Ecruteak City + Burned Tower + Ecruteak Gym)
- **4 gym leaders** operational (Violet/Falkner, Azalea/Bugsy, Goldenrod/Whitney, Ecruteak/Morty)
- Gen 2 battle system: damage formula, status conditions (PSN/BRN/PAR/SLP/FRZ), stat stages (7 stats), critical hits, PP tracking, accuracy/evasion stage modifiers on ALL moves including status, burn physical damage reduction
- Evolution, PC storage, Pokedex, Poke Mart (9 items), save system, day/night cycle
- Multi-Pokemon trainer battles with smart AI (50% best move)
- JS overlay sprite system for battle Pokemon (loaded from CDN via global_state bridge)
- Badge bitmask system, rival battle trigger
- Burned Tower with Rival encounter + Eusine NPC
- Eevee + all five Eeveelutions already in species data (Kimono Girls ready)
- New tile types: BLACK, FLOOR (for gym interiors like Morty's dark gym)
- 15 new species pre-staged for Routes 38-39/Olivine (Pidgeotto, Noctowl, Farfetch'd, Tauros, Magnemite, Doduo, Flaaffy, Psyduck, Mr. Mime, Skiploom, Corsola, Slowpoke, Pikachu, Poliwhirl, Krabby)
- 17 new moves pre-staged for Routes 38-39/Olivine (Thunder Wave, Bubble, Harden, Barrier, Meditate, ViceGrip, Double Team, Recover, Swords Dance, Whirlwind, Wing Attack, Tri Attack, Drill Peck, BubbleBeam, Thunderbolt, Faint Attack, Pay Day)

**Your goal: complete the Johto region** — all 8 gyms, Elite Four, Champion, and the critical path story events — so the game is playable from New Bark Town to credits. You have up to one week of continuous work.

---

## Reference Data Sources

Use these sources to ensure accuracy. They are listed in priority order — prefer sources higher on the list when data conflicts.

### 1. `pret/pokecrystal` Disassembly (PRIMARY SOURCE OF TRUTH)
**https://github.com/pret/pokecrystal**

This is the fully disassembled source code of Pokémon Crystal. It IS the game. When in doubt, this is what's correct. Key directories:

- **`data/pokemon/base_stats/`** — One `.asm` file per species with base stats, types, catch rate, exp yield, growth rate, gender ratio. Example: `data/pokemon/base_stats/cyndaquil.asm`
- **`data/pokemon/evos_attacks.asm`** — Every species' evolution method AND full level-up learnset in one file. This is the single source of truth for learnsets.
- **`data/moves/moves.asm`** — Every move's type, power, accuracy, PP, and effect constant. The effect constants map to battle engine routines.
- **`data/wild/johto_grass.asm`**, **`data/wild/johto_water.asm`** — Wild encounter tables per route, per time of day (morn/day/nite), with species and levels. Use these verbatim for encounter tables.
- **`data/trainers/parties.asm`** — Every trainer's party definition, including gym leaders, Elite Four, Champion, and rival at every encounter. Copy these teams exactly.
- **`data/trainers/attributes.asm`** — Trainer AI flags and item usage.
- **`data/types/type_matchups.asm`** — The complete type effectiveness chart.
- **`data/maps/`** — Map dimensions, connections, and warp definitions. Use `data/maps/map_headers.asm` for width/height of every map.
- **`maps/`** — Map scripts, NPC placement, event triggers, sign text, and story flag checks. Example: `maps/EcruteakCity.asm` contains every NPC's position, dialogue, and event scripting.
- **`engine/battle/core.asm`** — The battle engine. This is where turn order, damage calculation, accuracy checks, critical hit formula, and move effect dispatch all live.
- **`engine/battle/move_effects/`** — Individual move effect implementations (e.g., `teleport.asm`, `leech_seed.asm`). Reference these when implementing move effects.
- **`constants/pokemon_constants.asm`** — Canonical species IDs (Dex numbers). Use these for your SpeciesId constants.
- **`constants/move_constants.asm`** — Canonical move IDs. Use these for your MoveId constants.
- **`docs/bugs_and_glitches.md`** — Known bugs in the original game. Don't reproduce these bugs; this file tells you what to avoid.

When implementing a feature, read the corresponding disassembly file FIRST, then code it. Don't rely on memory or wiki summaries when the actual source code is available.

### 2. PokeAPI (MACHINE-READABLE DATA)
**https://pokeapi.co/api/v2/**

Free REST API returning JSON. No auth required. Use this for quick bulk data lookups when you need structured data fast. Key endpoints:

- `https://pokeapi.co/api/v2/pokemon/{id}` — Base stats, types, abilities, sprites URLs, moves with learn methods
- `https://pokeapi.co/api/v2/pokemon-species/{id}` — Growth rate, capture rate, base happiness, evolution chain reference
- `https://pokeapi.co/api/v2/move/{id}` — Power, accuracy, PP, type, damage class, effect description, effect chance
- `https://pokeapi.co/api/v2/type/{id}` — Damage relations (super effective, not effective, immune)
- `https://pokeapi.co/api/v2/evolution-chain/{id}` — Full evolution chain with triggers and level requirements
- `https://pokeapi.co/api/v2/location-area/{id}` — Encounter data per location including version-specific rates

Filter moves by `version-group` = `gold-silver` or `crystal` to get Gen 2 learnsets. The API returns moves across all generations by default — you must filter.

**Sprite URLs**: The API provides sprite URLs at `https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{id}.png` (front) and `.../back/{id}.png` (back). These are the same URLs the JS overlay sprite system can load from if the existing CDN doesn't have a species.

### 3. Bulbapedia (HUMAN-READABLE REFERENCE)
**https://bulbapedia.bulbagarden.net/**

Best for narrative/qualitative information that the disassembly and API don't convey well:

- Map layout descriptions and screenshots (search "X (location)" for any city/route)
- Story event sequences and NPC dialogue summaries
- Gym puzzle descriptions
- Item locations on routes
- Trainer class descriptions and which sprites they use
- Move effect descriptions in plain English (search "X (move)")

### 4. Smogon GSC Mechanics Thread
**https://www.smogon.com/forums/threads/gsc-mechanics.3542417/**

Deep technical reference for Gen 2 battle mechanics edge cases. The community has reverse-engineered exact formulas from the assembly. Use this for:

- Exact Gen 2 damage formula with integer truncation details
- Critical hit formula (Gen 2 uses base speed, not a flat rate — but your current 1/16 flat rate is an acceptable simplification)
- Stat stage multiplier table with the exact fractions
- Status condition mechanics (sleep counter range, freeze thaw chance, burn damage formula, poison damage formula, Toxic counter)
- Belly Drum glitch, Reflect overflow, and other known GSC quirks (avoid reproducing these)

### 5. Smogon Damage Calculator Source
**https://github.com/smogon/damage-calc**

The `calc/src/mechanics/gen12.ts` file contains the Gen 1-2 damage formula implemented in TypeScript. This is a clean, well-tested reference implementation you can translate to Rust. It handles STAB, type effectiveness, critical hits, random factor, stat stages, and badge boosts all in one readable function.

### 6. Serebii GSC Pokédex
**https://www.serebii.net/pokedex-gs/**

Quick reference for per-species data formatted for Gen 2 specifically. Each species page shows base stats, Gen 2 learnset (level-up moves with levels), TM/HM compatibility, and evolution conditions — all scoped to Gold/Silver/Crystal without needing to filter out later-gen data.

### 7. Data Crystal ROM Map
**https://datacrystal.tcrf.net/wiki/Pokémon_Gold_and_Silver/ROM_map**

Contains map dimensions, music IDs per location, and memory addresses for all game data. Useful for verifying map sizes and getting the correct `music_id` values for each MapData struct.

---

Do NOT try to implement everything breadth-first. Work in geographic phases that produce a testable, playable expansion each time. Each phase should leave the game in a compilable, playable state.

### Phase 1: Data Tables (do this FIRST, all at once)
Expand `data.rs` before touching any other file. This is pure tabular work and benefits from being done in one concentrated push.

**Species** — Add all remaining Johto-obtainable species (~70-90 more needed). Follow the exact `SpeciesData` struct pattern already established. Reference Bulbapedia for base stats, growth rates, catch rates, learnsets, and evolution data. Prioritize:
- All species needed for gym leader / E4 / Champion teams (Poliwrath, Steelix, Piloswine, Kingdra, Xatu, Slowbro, Machamp, Houndoom, Murkrow, Dragonite, Gyarados, Charizard, Aerodactyl)
- All species appearing in route encounter tables you'll be building
- Complete evolution chains (don't add a middle stage without its final form)
- Note: Eevee line is DONE. Haunter/Gengar are referenced in maps.rs gym trainers — verify they have SpeciesData entries.

**Moves** — Add all moves used by species learnsets and trainer teams you're adding (~100-120 more). Follow the `MoveData` struct pattern. The current `MoveData` struct only has: id, name, type, category, power, accuracy, pp, description. Status/effect moves still need their effects handled in `mod.rs` battle logic — flag these with a `// TODO: effect` comment for now. Handle them in Phase 5.

**Key constraint**: Every SpeciesId and MoveId must use the canonical Gen 2 Pokedex/move index numbers. This is already the convention (CHIKORITA = 152, MOVE_TACKLE = 33). Check existing constants before adding duplicates. When in doubt, add more species and moves rather than fewer — completeness is the goal.

### Phase 2: Maps — Olivine, Cianwood, Mahogany (Gyms 5-7)
You're starting from Ecruteak, which already has east exit warps stubbed to self-loop at (18,8)/(18,9). Build maps in route order, wiring each one to the previous.

**Map order**: Route 38 → Route 39 → OlivineCity → OlivineGym → OlivineLighthouse (multi-floor: ground floor + top floor with sick Ampharos) → Route 40 → CianwoodCity → CianwoodGym → CianwoodPharmacy (interior, gives SecretPotion) → Route 42 → MahoganyTown → Route 43 → LakeOfRage (outdoor area with forced Red Gyarados encounter) → MahoganyGym → RocketHQ (multi-room base with Rocket trainers and executive battles)

For each map, follow the exact pattern in `maps.rs`:
1. Add variant to `MapId` enum
2. Add match arm in `load_map()`
3. Write `build_<map_name>()` function with tiles, collision, NPCs, warps, encounters
4. Wire warps bidirectionally
5. Add the map to the `all_map_ids()` list at the bottom of maps.rs

**Map fidelity target**: Match the original game's map dimensions as closely as possible. Key buildings should be correctly placed. Encounter tables should be accurate per-route per Bulbapedia. Gym puzzles should be implemented faithfully — Pryce's ice sliding puzzle, Chuck's waterfall room, Clair's lava/dragon room. Use existing tile constants (TREE_TOP, TREE_BOTTOM, GRASS, PATH, BUILDING_ROOF, BUILDING_WALL, DOOR, FLOOR, BLACK, WATER, POKECENTER_ROOF, etc). Add new tile constants as needed (ICE_FLOOR, LAVA, etc.) for gym-specific interiors. Every map should have the correct music_id for its location type.

**Ecruteak east exit wiring**: The current east warps at (18,8) and (18,9) in Ecruteak need to be updated to point to Route 38 once you build it.

### Phase 3: Maps — Blackthorn through Victory Road (Gym 8 + E4)
**Map order**: Route 44 → IcePath (multi-room cave with ice floor tiles and trainers) → BlackthornCity → BlackthornGym → Route 45 → Route 46 (connects back to Route 29) → Route 27 → Route 26 → VictoryRoad (full multi-room cave with trainers and puzzles) → IndigoPlateau → EliteFourWill → EliteFourKoga → EliteFourBruno → EliteFourKaren → ChampionLance

The E4 rooms should each feel distinct — themed to their type specialty with appropriate tile palettes. Each room has the leader NPC, and a warp at the top leading to the next room after victory.

### Phase 4: Trainer Teams & Gym Leaders
Add all gym leader teams, Elite Four teams, Champion Lance's team, and Rival encounters. The trainer battle system already works (see Morty's gym for the pattern) — you just need NPC entries with `trainer_team` fields and a `GiveBadge` dialogue action for gym leaders.

**Remaining Gym leaders** (add to their gym map NPCs, NPC index 0 for badge-giving logic):
| Gym | Leader | Badge # | Key Pokemon |
|-----|--------|---------|-------------|
| 5 - Cianwood | Chuck | 4 | Primeape lv27, Poliwrath lv30 |
| 6 - Olivine | Jasmine | 5 | 2x Magnemite lv30, Steelix lv35 |
| 7 - Mahogany | Pryce | 6 | Seel lv27, Dewgong lv29, Piloswine lv31 |
| 8 - Blackthorn | Clair | 7 | 3x Dragonair lv37, Kingdra lv40 |

Badge-giving: follow the existing pattern in `mod.rs` — match `(MapId::XxxGym, 0)` in the badge_action block and add the badge name to the match in `DialogueAction::GiveBadge`.

**Elite Four** (use canonical GSC teams):
- Will: Xatu lv40, Jynx lv41, Exeggutor lv41, Slowbro lv41, Xatu lv42
- Koga: Ariados lv40, Forretress lv43, Muk lv42, Venomoth lv41, Crobat lv44
- Bruno: Hitmontop lv42, Hitmonlee lv42, Hitmonchan lv42, Onix lv43, Machamp lv46
- Karen: Umbreon lv42, Vileplume lv42, Gengar lv45, Murkrow lv44, Houndoom lv47
- Lance: Gyarados lv44, Dragonite lv47, Dragonite lv47, Aerodactyl lv46, Charizard lv46, Dragonite lv50

**Rival encounters**: Currently one trigger on Route 29 + one in Burned Tower. Add one more encounter:
- Victory Road or Indigo Plateau entrance — starter final form lv36 + Haunter lv35 + Sneasel lv34 + Magneton lv34 + Golbat lv36

### Phase 5: Move Effects & Battle Polish
Now implement status/effect moves that gym leaders and E4 actually use. Prioritize by what will cause visible bugs in important fights.

**How move effects currently work**: The `status_move_stage_effect()` function in `mod.rs` dispatches stat stage changes for status moves. The `try_inflict_status()` function handles PSN/BRN/PAR/SLP/FRZ infliction. Damaging moves just deal damage via the Gen 2 formula. To add secondary effects on damaging moves, add a post-damage roll in the PlayerAttack/EnemyAttack phase handlers (around lines 983-1000 for player, 1130-1155 for enemy).

**Priority 1 — Damaging moves E4/Champion use**: Surf, Ice Beam, Earthquake, Thunderbolt, Psychic, Crunch, Hyper Beam, Dragon Breath, Fire Blast, Shadow Ball. Most of these are simple power/type/category — they'll "just work" once added to the move DB. Add secondary effect rolls where applicable (Thunderbolt 10% paralysis, Ice Beam 10% freeze, Fire Blast 10% burn, Dragon Breath 30% paralysis, Crunch 20% def drop, Psychic 10% sp.def drop).

**Priority 2 — Status moves with unique effects**:
- Haze (already in move DB): reset all stat stages — add to battle phase handler
- Self-Destruct (already in move DB): user faints after dealing damage — add faint-self logic
- Toxic: escalating poison counter (track turns poisoned, damage = max_hp * turns / 16)
- Confuse Ray / Swagger / Hypnosis: confusion not yet implemented — add a `confused: u8` field to Pokemon struct, roll for self-hit each turn
- Mean Look / trapping: prevent fleeing — add `trapped: bool` to BattleState

**Priority 3 — Multi-turn moves**: Fly, Dig, SolarBeam — these need a new BattlePhase variant like `Charging { move_id, timer }`. Skip these unless a gym leader or E4 member specifically uses them.

**Skip only if time is critically short**: Weather (Rain Dance/Sunny Day), Baton Pass, Perish Song, Destiny Bond. Implement everything else — if a move appears in any trainer's Pokemon's learnset at the level they're encountered, it should work correctly in battle.

### Phase 6: Story Events & Gating
The current flag system is minimal (badges bitmask, rival_battle_done bool). Add a `story_flags: u64` bitmask field to PokemonState:

```rust
const FLAG_GOT_STARTER: u64        = 1 << 0;
const FLAG_RIVAL_ROUTE29: u64      = 1 << 1;
const FLAG_EGG_FROM_ELM: u64       = 1 << 2;
const FLAG_ROCKET_SLOWPOKE: u64    = 1 << 3;
const FLAG_ROCKET_RADIO_TOWER: u64 = 1 << 4;
const FLAG_RED_GYARADOS: u64       = 1 << 5;
const FLAG_ROCKET_MAHOGANY: u64    = 1 << 6;
const FLAG_MEDICINE_QUEST: u64     = 1 << 7;  // visited sick Ampharos
const FLAG_GOT_SECRETPOTION: u64   = 1 << 8;  // got medicine from Cianwood
const FLAG_DELIVERED_MEDICINE: u64  = 1 << 9;  // Jasmine can now battle
const FLAG_DRAGON_DEN: u64         = 1 << 10;
```

**Critical story gates to implement**:
- **Olivine Gym lock**: Jasmine won't battle until FLAG_DELIVERED_MEDICINE is set. Visiting Lighthouse sets FLAG_MEDICINE_QUEST. Getting SecretPotion from Cianwood Pharmacy sets FLAG_GOT_SECRETPOTION. Returning to Lighthouse NPC when you have the potion sets FLAG_DELIVERED_MEDICINE and unlocks Jasmine.
- **Lake of Rage → Mahogany**: Red Gyarados is a forced encounter (lv 30 shiny Gyarados). Defeating/catching it sets FLAG_RED_GYARADOS, which unblocks Mahogany Gym.
- **Rocket HQ**: Clearing 3 Rocket trainers in MahoganyRocketHQ sets FLAG_ROCKET_MAHOGANY. This gates access to Blackthorn (Route 44 NPC blocks until flag is set).
- **Route gating NPCs**: Place a blocking NPC at choke points who check flags. Pattern: `if self.story_flags & FLAG_X == 0 { show blocking dialogue }` else NPC steps aside or is absent.

**Implementation standard**: Story events should faithfully represent the original game's progression logic. If the original has a multi-floor dungeon, build it out — multiple rooms with trainers, items, and correct warp connections. The Slowpoke Well should feel like a real dungeon. The Radio Tower takeover should have the correct number of Rocket encounters across multiple floors. The Lake of Rage should be a proper outdoor area with the Red Gyarados encounter at water's edge. Don't shortcut these — they're memorable moments in the game and worth getting right.

### Phase 7: Credits & Endgame
After defeating Lance, trigger a simple credits sequence. Add a new `GamePhase::Credits { timer: f64 }` variant. Render scrolling text on the framebuffer: "POKEMON GOLD — Crusty Engine Recreation" + "THE END" + return to title screen. This is ~50 lines of code in the render path.

---

## Critical Technical Notes

### File Size Management
`mod.rs` is at 3,427 lines and `maps.rs` is at 5,024 lines. They will grow significantly. **Split files proactively when they cross ~5,000 lines:**
- `mod.rs` → extract `battle.rs` (all BattlePhase handling, roughly lines 800-1700) and `menus.rs` (PokemonMenu, Bag, Mart, PC, Pokedex phases)
- `maps.rs` → split into `maps_early.rs` (New Bark → Goldenrod, already ~3500 lines) and `maps_late.rs` (Ecruteak onward)

**This is worth investing 30 minutes in.** Claude Code's edit accuracy degrades on files >4000 lines. A clean split early saves hours of mis-targeted edits later. When splitting, keep all `pub` exports in the original module file and `pub use` from sub-modules.

### Sprites
Don't try to add Rust-side sprites for new Pokemon. The JS overlay sprite system already handles battle sprites. For overworld trainer sprites, reuse the existing `sprite_id` palette (0-6 already in use). New tile types for gym interiors should follow the Ecruteak Gym precedent — BLACK and FLOOR tiles work well, add ICE_FLOOR (light blue) for Pryce's gym if needed as a new `const` in maps.rs.

### HMs — Do the Minimum
- **Surf**: Don't implement. Design maps so water is never on the critical path. Lake of Rage encounter should be on a land-adjacent tile.
- **Fly**: Add as a menu option that warps to any visited city's Pokemon Center. ~30 lines in the Menu phase handler. High QoL for a week-long playtest.
- **Cut / Strength / Whirlpool / Waterfall**: Skip entirely. Design maps without these obstacles on the critical path.

### Save System — Full localStorage Persistence + Load Game

The engine already has the infrastructure for this: `PersistCommand::Set { key, value }` in Rust → `drain_persist_commands()` in JS. The JS side just needs to wire `drain_persist_commands()` output to `localStorage`. Currently the drain happens every frame but nothing is stored. Here's what needs to happen on both sides:

**JS side (index.html)**: After calling `drain_persist_commands()`, parse the returned JSON array and execute each command against `localStorage`:
```javascript
const cmds = JSON.parse(drain_persist_commands());
for (const cmd of cmds) {
    if (cmd.type === 'Set') localStorage.setItem('pokemon_' + cmd.key, cmd.value);
    if (cmd.type === 'Remove') localStorage.removeItem('pokemon_' + cmd.key);
    if (cmd.type === 'Clear') { /* clear pokemon_ prefixed keys */ }
}
```
Prefix all keys with `pokemon_` to avoid collisions with other Crusty games.

**JS side — load on startup**: Before calling `setup_test_pokemon()`, check if `localStorage.getItem('pokemon_save_exists')` is truthy. If so, read all `pokemon_*` keys and push them into WASM via `set_game_state_str(key, value)` for each one. This makes the saved state available to Rust before the game initializes.

**Rust side — saving**: When the player saves at a Pokemon Center PC (or any save trigger), serialize the COMPLETE game state to the persist queue. This means:
- `save_exists` = "1"
- `save_map_id` = current map enum as string (e.g., "GoldenrodCity")
- `save_player_x`, `save_player_y` = grid coordinates
- `save_player_facing` = direction
- `save_money`, `save_badges`, `save_story_flags`
- `save_party_count` + for each party member: `save_party_N_species`, `save_party_N_level`, `save_party_N_hp`, `save_party_N_max_hp`, `save_party_N_exp`, `save_party_N_move0`..`save_party_N_move3`, `save_party_N_pp0`..`save_party_N_pp3`, `save_party_N_status`
- `save_pc_count` + PC box Pokemon in same format
- `save_pokedex_seen` + `save_pokedex_caught` as comma-separated species ID lists
- `save_defeated_trainers` as comma-separated "mapid:npcidx" pairs
- `save_rng_state` = current `engine.rng.state` value (critical for determinism — see below)

**Rust side — loading**: In `PokemonState::new()` or a `load_from_state()` constructor, check `engine.global_state.get_str("save_exists")`. If "1", reconstruct the full game state from the saved values, set `current_map` to the saved map, position the player, restore party/PC/flags/badges/money, and restore the RNG state. Start in `GamePhase::Overworld` instead of `GamePhase::TitleScreen`.

**Title screen — Continue vs New Game**: The original Pokemon Gold has three options on the title screen: CONTINUE, NEW GAME, OPTIONS. Implement at minimum CONTINUE and NEW GAME. If a save exists, show CONTINUE as the first option (highlighted by default). CONTINUE loads from localStorage. NEW GAME warns "This will overwrite your save" and starts fresh. This is essential for multi-session play.

**Auto-save consideration**: In addition to manual Pokemon Center saves, consider auto-saving whenever the player enters a new map. This protects against browser crashes during long play sessions. Use the same persist queue mechanism.

### Compilation Checks
Run `cargo check` (not full `cargo build`) after every logical unit of work — after adding a batch of species, after each new map, after each battle logic change. The WASM build is slow; use native target for iteration speed. Only do a full `wasm-pack build` when you want to browser-test.

### Pokemon Center Reuse
The existing `MapId::PokemonCenter` is a generic interior that heals your team. Every city already warps to this shared instance. Keep reusing it — don't build unique Pokemon Center interiors per city. The warp-back works because each city's PokemonCenter door warp is set to return to that city.

### Trainer Defeated Tracking
Currently there's no system to track which trainers have been defeated (gym leaders re-battle on re-entry). Add a `defeated_trainers: HashSet<(MapId, u8)>` to PokemonState (map + NPC index). Check it before triggering trainer battles. This prevents softlocks from repeatedly re-battling gym leaders and re-triggering badge dialogues.

---

## Determinism is Non-Negotiable

This game will be used to stress-test Crusty's deterministic headless simulation infrastructure after completion. Every design decision must preserve determinism. The engine already enforces this at the architecture level — one canonical `SeededRng` (xorshift64), fixed timestep, `InputFrame`-based replay — but the Pokemon game logic must not break these guarantees.

### Rules for Determinism

**Only use `engine.rng` for randomness.** Never use `std::random`, `js_sys::Math::random`, system time, or any other entropy source in game logic. Every random outcome — wild encounter rolls, damage variance, critical hit checks, catch rate calculations, accuracy rolls, status infliction chances, AI move selection — must flow through `engine.rng.next_f64()` or its derivatives (`range_i32`, `chance`, etc.).

**RNG call order must be stable.** If you add a new random call (e.g., confusion self-hit check), it must happen at a deterministic point in the step sequence. Don't conditionally skip RNG calls based on visual state — if a branch might call `rng.next_f64()`, the other branch should too (use a dummy call if needed) so that the RNG sequence stays synchronized across replay. This is the most common source of determinism bugs.

**No floating-point-dependent branching on render-only state.** The `Simulation::step()` vs `Simulation::render()` split exists for a reason. Step handles game logic (deterministic). Render handles visual-only state (camera lerp, screen flash timers, sprite animation). Never read render-only state in step logic. Never advance RNG in render.

**Save/load must capture RNG state.** When saving, persist `engine.rng.state` (a `u64`). When loading, restore it before any game logic runs. This means a save-load-continue sequence produces identical outcomes to an uninterrupted session. See the save system section above — `save_rng_state` is listed as a required field.

**Keep all game state in `PokemonState`.** Don't stash mutable game state in statics, thread-locals, or the JS layer. Everything that affects game outcomes must live in the `PokemonState` struct so that `state_hash()` captures it and headless replay can reproduce it.

**Map loading must be pure.** `load_map()` is already a pure function of `MapId` — it returns the same `MapData` every time. Keep it that way. Don't add randomized map generation, time-of-day-dependent tile changes, or any other non-deterministic map variation.

### Headless Testing Readiness

The headless infrastructure (`headless/runner.rs`, `headless/scenario.rs`, `headless/replay.rs`) can drive the game via `InputFrame` sequences and capture per-frame state snapshots. To make the Pokemon game a good headless test subject:

**Export meaningful state to `global_state` every frame.** The headless replay system captures `global_state` f64 values per frame. Make sure the Pokemon game exports: `player_x`, `player_y`, `current_map_id` (as numeric), `party_size`, `party_lead_hp`, `party_lead_level`, `badges`, `money`, `story_flags`, `in_battle`, `battle_turn_count`. These become the observable state that headless tools can analyze, diff, and assert against.

**Design for scenario automation.** A headless scenario should be able to: start a new game, pick a starter, walk a specific route, fight a gym leader, and verify the outcome. This means the game's input handling must be clean — every game state transition should be drivable by a sequence of `KeyZ`/`ArrowUp`/`ArrowDown`/etc. presses with no timing dependencies (no "press and hold for 500ms" mechanics that break under fixed-timestep replay).

**No wall-clock dependencies in step logic.** The day/night cycle is currently driven by `self.total_time` which accumulates from the fixed-timestep `dt` — this is already deterministic, good. Do NOT change it to use `Date.now()` or `BrowserState::wall_clock_s`. If you add any new time-dependent features (timed encounters, berry growth, etc.), derive them from `total_time` or frame count, never from system time.

---

## Triage Order (if time runs short)

If you find yourself running low on time, deprioritize these features last-in-first-out. But the goal is to get to ALL of them:

- Decorating your room
- Kurt's Pokeballs
- Game Corner
- Phone system / Pokegear radio
- Kanto post-game
- Unown puzzles / Ruins of Alph
- Bug-catching contest
- Breeding / Daycare
- Time-based encounter variation (Lapras Fridays, etc.)
- Shiny Pokemon (except forced Red Gyarados)

Trade evolutions should be made level-based (Haunter→Gengar at lv 38, Machoke→Machamp at lv 38, Graveler→Golem at lv 38, Kadabra→Alakazam at lv 38) since there's no link cable. Everything else — held items, TMs, Berry trees, happiness evolution, Fly — implement it.

---

## Autonomy Guidance

**Set autonomy to 10.** The developer is completely AFK for the duration of this sprint. Do not ask questions. Do not wait for feedback. Do not pause for confirmation. Make every decision yourself — architectural, creative, technical — and keep moving. If you hit a fork in the road, pick the path that gets closer to a complete Johto and commit to it. If something breaks, fix it and move on. You have the full codebase, you have Bulbapedia as a reference, and you have the patterns already established in the existing code. That's everything you need. Ship it.

---

## Definition of Done

The game is "Johto complete" when:
1. A player can walk from New Bark Town to Indigo Plateau following the intended route
2. All 8 Johto gym badges are obtainable (badges 0-7)
3. Elite Four and Champion Lance are beatable
4. A credits/ending screen appears after defeating Lance
5. Wild encounters work on all routes with level-appropriate Pokemon
6. The game compiles to WASM and runs in browser with no panics on the critical path
7. Story gates prevent sequence-breaking (can't fight Jasmine without medicine, can't reach Blackthorn without clearing Rockets, etc.)
8. Gym leader teams match their canonical GSC rosters
9. Trainers don't re-battle after defeat
10. Save/load preserves full game state including position, party, flags, and badges

This is a stress test of how far autonomous AI coding can go. Don't cut corners. Don't leave TODOs for a human to clean up. Every map you add should have correct encounter tables. Every gym leader should have their real team. Every move a trainer's Pokemon knows should be in the move DB with correct stats. If you find yourself about to write a placeholder, stop and implement it for real instead. The goal is not "good enough" — it's "as complete and correct as you can possibly make it."

# Architectural Changes to Eliminate Implementation Risk

These are concrete code changes to add to the sprint prompt. Each one eliminates a class of bugs rather than individual bugs, which is the only way to survive a week of autonomous coding at this scale.

---

## 1. Derive Move Category from Type (Eliminate the Physical/Special Bug Class)

**The problem**: Every new MoveData entry requires manually setting `category: MoveCategory::Physical` or `MoveCategory::Special`. The ENGINE_POKEMON.md documents this as a recurring bug — Pursuit, Fire Punch, Sonic Boom, and Acid all had wrong categories. With 100+ moves to add, this will happen again and again.

**The fix**: In Gen 2, category is determined entirely by type. Remove the `category` field from `MoveData` and compute it:

```rust
impl PokemonType {
    pub fn gen2_category(&self) -> MoveCategory {
        match self {
            // Physical types
            PokemonType::Normal | PokemonType::Fighting | PokemonType::Poison |
            PokemonType::Ground | PokemonType::Flying | PokemonType::Bug |
            PokemonType::Rock | PokemonType::Ghost | PokemonType::Steel => MoveCategory::Physical,
            // Special types
            PokemonType::Fire | PokemonType::Water | PokemonType::Grass |
            PokemonType::Electric | PokemonType::Ice | PokemonType::Psychic |
            PokemonType::Dragon | PokemonType::Dark => MoveCategory::Special,
        }
    }
}
```

Then in `MoveData`, replace the `category` field with a method:

```rust
pub fn category(&self) -> MoveCategory {
    if self.power == 0 { MoveCategory::Status } else { self.move_type.gen2_category() }
}
```

This makes it **impossible** to get the physical/special split wrong. Every move's category is now a function of its type, which is the Gen 2 rule. Status moves (power == 0) are automatically classified. Remove the `category` field from every `MoveData` literal — that's ~80 deletions that each eliminate a potential bug.

**Risk eliminated**: Every future move added with the wrong category. This was the #1 data correctness bug across sprints 39-42.

---

## 2. Compile-Time Warp Validation (Eliminate the Warp Destination Bug Class)

**The problem**: Warp destinations landing on C_WARP or C_SOLID tiles is the #1 QA bug across all sprints. Each warp is a hand-typed `(dest_x, dest_y)` coordinate that must be manually verified against the destination map's collision array. With 30+ new maps and 100+ new warps, this is untenable without human QA.

**The fix**: Add a `validate_all_warps()` function that runs as a `#[test]` and also runs once at startup in debug builds:

```rust
pub fn validate_all_warps() -> Vec<String> {
    let mut errors = Vec::new();
    let all_maps = [
        MapId::NewBarkTown, MapId::Route29, /* ... all MapId variants ... */
    ];
    for &map_id in &all_maps {
        let map = load_map(map_id);
        for (i, warp) in map.warps.iter().enumerate() {
            let dest = load_map(warp.dest_map);
            let idx = warp.dest_y as usize * dest.width + warp.dest_x as usize;
            if idx >= dest.collision.len() {
                errors.push(format!(
                    "{:?} warp {} → {:?} ({},{}) OUT OF BOUNDS (map is {}x{})",
                    map_id, i, warp.dest_map, warp.dest_x, warp.dest_y, dest.width, dest.height
                ));
                continue;
            }
            let tile = dest.collision[idx];
            if tile == C_WARP || tile == C_SOLID || tile == C_WATER {
                errors.push(format!(
                    "{:?} warp {} → {:?} ({},{}) lands on {:?} (should be walkable)",
                    map_id, i, warp.dest_map, warp.dest_x, warp.dest_y,
                    CollisionType::from_u8(tile)
                ));
            }
        }
    }
    errors
}

#[test]
fn test_all_warps_valid() {
    let errors = validate_all_warps();
    assert!(errors.is_empty(), "Warp validation errors:\n{}", errors.join("\n"));
}
```

Call `validate_all_warps()` in PokemonSim::new() during debug builds and log any errors. This means every `cargo check` or `cargo test` immediately catches warp bugs **before** they require browser testing.

**Also add**: Bidirectional warp validation. For every warp A→B, check that B has a warp back to A (or at least to the same region). This catches "one-way door" bugs where you can enter a building but can never leave.

**Risk eliminated**: The entire warp destination bug class. `cargo test` becomes the QA tool.

---

## 3. Story Flag System as a First-Class Type (Eliminate Flag Spaghetti)

**The problem**: There are currently zero story flags. The only progression gates are `badges` (bitmask), `has_starter` (bool), and `rival_battle_done` (bool). Every new story event needs its own field, and NPC gating logic will be scattered across `step_overworld` as ad-hoc if-statements.

**The fix**: Add story flags as a proper subsystem with a typed enum, not a raw u64:

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum StoryFlag {
    GotStarter = 0,
    RivalRoute29 = 1,
    EggFromElm = 2,
    RocketSlowpoke = 3,
    RocketRadioTower = 4,
    RedGyarados = 5,
    RocketMahogany = 6,
    MedicineQuest = 7,
    GotSecretPotion = 8,
    DeliveredMedicine = 9,
    DragonDen = 10,
    ClearedIcePath = 11,
    BurnedTowerBeasts = 12,
    // Add as needed — up to 63
}

impl PokemonSim {
    fn has_flag(&self, flag: StoryFlag) -> bool {
        self.story_flags & (1u64 << flag as u8) != 0
    }
    fn set_flag(&mut self, flag: StoryFlag) {
        self.story_flags |= 1u64 << flag as u8;
    }
}
```

Add `story_flags: u64` to PokemonSim. Then add a new `DialogueAction` variant:

```rust
DialogueAction::SetFlag { flag: StoryFlag },
DialogueAction::GiveItem { item_id: u8 },
```

And add a flag field to `NpcDef` for conditional visibility:

```rust
pub struct NpcDef {
    // ... existing fields ...
    pub requires_flag: Option<StoryFlag>,    // NPC only appears if flag IS set
    pub hidden_by_flag: Option<StoryFlag>,   // NPC disappears once flag IS set
}
```

This lets you define gating NPCs declaratively in map data rather than imperatively in game logic:

```rust
// Blocking NPC on Route 44 — disappears after clearing Rockets
NpcDef {
    x: 5, y: 10, sprite_id: 2, facing: Direction::Down,
    dialogue: &["The road ahead is", "closed right now."],
    hidden_by_flag: Some(StoryFlag::RocketMahogany),
    requires_flag: None,
    // ...
}
```

The NPC visibility check goes in one place — the overworld NPC rendering/collision loop — instead of being scattered as per-NPC if-statements throughout the code.

**Risk eliminated**: Story gating bugs from ad-hoc conditional logic. Flags become data, not code.

---

## 4. Full Save Serialization as a Single JSON Blob (Eliminate Partial Save Bugs)

**The problem**: The current save system pushes individual key-value pairs to `global_state`. Saving 6 party Pokemon means 6×10+ separate `set_f64` calls. If the save is interrupted midway (browser tab closed), you get a corrupted half-save. Loading requires reading dozens of individual keys and hoping they're all consistent.

**The fix**: Serialize the entire save state as a single JSON string and push it in one `PersistCommand::Set`:

```rust
fn serialize_save(&self) -> String {
    let mut save = String::from("{");
    // Single atomic blob
    save += &format!("\"map\":\"{:?}\",", self.current_map_id);
    save += &format!("\"x\":{},\"y\":{},", self.player.x, self.player.y);
    save += &format!("\"facing\":{},", self.player.facing as u8);
    save += &format!("\"money\":{},", self.money);
    save += &format!("\"badges\":{},", self.badges);
    save += &format!("\"flags\":{},", self.story_flags);
    save += &format!("\"time\":{},", self.total_time);
    save += &format!("\"rng\":{},", /* engine.rng.state */);
    save += "\"party\":[";
    for (i, p) in self.party.iter().enumerate() {
        if i > 0 { save += ","; }
        save += &serialize_pokemon(p);
    }
    save += "],\"pc\":[";
    for (i, p) in self.pc_boxes.iter().enumerate() {
        if i > 0 { save += ","; }
        save += &serialize_pokemon(p);
    }
    save += "],";
    // Defeated trainers
    save += "\"defeated\":[";
    for (i, (map, npc)) in self.defeated_trainers.iter().enumerate() {
        if i > 0 { save += ","; }
        save += &format!("[\"{:?}\",{}]", map, npc);
    }
    save += "],";
    // Pokedex
    save += &format!("\"seen\":{:?},", self.pokedex_seen);
    save += &format!("\"caught\":{:?}", self.pokedex_caught);
    save += "}";
    save
}
```

One `persist_set(queue, "pokemon_save", &self.serialize_save())` call. One localStorage key. Atomic write. On the JS side:

```javascript
const cmds = JSON.parse(drain_persist_commands());
for (const cmd of cmds) {
    if (cmd.type === 'Set') localStorage.setItem(cmd.key, cmd.value);
}
```

On load, read the single JSON blob and parse it. If the key doesn't exist, there's no save. If it does, it's guaranteed to be a complete snapshot.

**Risk eliminated**: Partial saves, key-name typos, inconsistent state from interrupted saves.

---

## 5. Export Overworld State Every Frame (Enable Headless Debugging)

**The problem**: When a bug happens during autonomous coding, there's no human to notice it in the browser. The headless infrastructure can capture `global_state` per frame — but the Pokemon game barely exports anything to it. Currently only `in_battle`, `game_phase`, and battle-specific values are exported.

**The fix**: Add a `export_debug_state()` call at the end of every `step()`:

```rust
fn export_debug_state(&self, engine: &mut Engine) {
    engine.global_state.set_f64("player_x", self.player.x as f64);
    engine.global_state.set_f64("player_y", self.player.y as f64);
    engine.global_state.set_f64("current_map", self.current_map_id as u8 as f64);
    engine.global_state.set_f64("badges", self.badges as f64);
    engine.global_state.set_f64("money", self.money as f64);
    engine.global_state.set_f64("story_flags", self.story_flags as f64);
    engine.global_state.set_f64("party_size", self.party.len() as f64);
    if let Some(p) = self.party.first() {
        engine.global_state.set_f64("lead_hp", p.hp as f64);
        engine.global_state.set_f64("lead_level", p.level as f64);
        engine.global_state.set_f64("lead_species", p.species_id as f64);
    }
    engine.global_state.set_f64("step_count", self.step_count as f64);
    engine.global_state.set_f64("defeated_count", self.defeated_trainers.len() as f64);
}
```

This makes every frame observable by headless tools. A post-sprint headless scenario can walk from New Bark to a gym and assert that `badges` increments, `current_map` changes at the right times, and `lead_hp` never hits zero unexpectedly.

**Risk eliminated**: Silent bugs that only surface during human playtesting. Enables automated regression detection.

---

## 6. MapId Exhaustive Match Enforcement (Eliminate Missing-Map Panics)

**The problem**: `load_map()` uses a match on `MapId`. If a new `MapId` variant is added to the enum but the match arm is forgotten, the code panics at runtime. This is a guaranteed bug for every new map — add the enum variant, wire the warps, forget the match arm, crash.

**The fix**: This should already be caught by the compiler IF `load_map` has no wildcard (`_`) arm. Check that it doesn't. If it does, remove it. Rust's exhaustive matching will then produce a compile error for every `MapId` variant without a `load_map` arm. This turns a runtime panic into a compile error.

Also add to `all_map_ids()` or equivalent: a `#[test]` that iterates every `MapId` variant and calls `load_map()` on it, asserting the returned `MapData` has the correct `id` field:

```rust
#[test]
fn test_all_maps_loadable() {
    // Use strum or manually list all variants
    let maps = vec![MapId::NewBarkTown, MapId::Route29, /* ... */];
    for id in maps {
        let data = load_map(id);
        assert_eq!(data.id, id, "load_map({:?}) returned wrong id", id);
        assert!(data.width > 0 && data.height > 0);
        assert_eq!(data.tiles.len(), data.width * data.height);
        assert_eq!(data.collision.len(), data.width * data.height);
    }
}
```

**Risk eliminated**: Forgotten match arms, tile/collision array size mismatches, wrong map ID in MapData.

---

## 7. NPC Dialogue as Data, Not Hardcoded Branches (Eliminate Dialogue Spaghetti)

**The problem**: The current NPC interaction code in `step_overworld` has hardcoded checks like `match (map_id, npc_idx)` to determine badge rewards, healing behavior, and mart behavior. With 50+ maps and hundreds of NPCs, this match block becomes enormous and fragile.

**The fix**: Extend `NpcDef` with an `action` field that declaratively specifies what happens on interaction:

```rust
#[derive(Clone, Debug)]
pub enum NpcAction {
    Talk,                                    // just show dialogue
    Heal,                                    // Pokemon Center nurse
    Mart,                                    // open shop
    GiveBadge { badge_num: u8 },            // gym leader
    GiveItem { item_id: u8 },               // story item handoff
    SetFlag { flag: StoryFlag },            // set a story flag on interaction
    TrainerBattle,                           // already handled by is_trainer, but explicit
    ConditionalDialogue {                    // different dialogue based on flags
        flag: StoryFlag,
        before: &'static [&'static str],
        after: &'static [&'static str],
    },
}

pub struct NpcDef {
    // ... existing fields ...
    pub action: NpcAction,
}
```

Then the NPC interaction handler becomes a simple match on `npc.action` instead of matching on `(map_id, npc_idx)`. The badge-giving, healing, mart, and flag-setting logic all collapse into a single dispatch:

```rust
match npc.action {
    NpcAction::Heal => { /* heal party, show dialogue */ },
    NpcAction::GiveBadge { badge_num } => { /* give badge, show dialogue */ },
    NpcAction::SetFlag { flag } => { self.set_flag(flag); /* show dialogue */ },
    NpcAction::ConditionalDialogue { flag, before, after } => {
        let lines = if self.has_flag(flag) { after } else { before };
        /* show dialogue */
    },
    // ...
}
```

This moves NPC behavior into map data where it belongs. Adding a gym leader to a new map doesn't require touching `mod.rs` at all — you just set `action: NpcAction::GiveBadge { badge_num: 5 }` in the NpcDef.

**Risk eliminated**: Forgotten badge logic, wrong `(map_id, npc_idx)` tuples, growing match blocks that Claude Code edits incorrectly in large files.

---

## Summary: What These Changes Do to the Odds

These seven changes target the specific bug classes documented across 42 sprints of development:

| Change | Bug class eliminated | Estimated time saved |
|--------|---------------------|---------------------|
| Derive category from type | Physical/special misclassification | 2-3 hours of debugging |
| Warp validation tests | Warp destination landing on wrong tile | 4-6 hours of manual QA |
| Typed story flags + NPC visibility | Story gating logic errors | 3-4 hours of ad-hoc code |
| Single-blob save | Partial/corrupted saves | 2 hours of debugging |
| Debug state export | Silent bugs invisible without playtesting | Unbounded (enables automated QA) |
| Exhaustive match enforcement | Missing map load arms | 30 min per incident |
| NPC action as data | Forgotten badge/heal/mart logic | 2-3 hours of match block surgery |

Implementing all seven takes ~2-3 hours upfront. They collectively save an estimated 15-20 hours of debugging over the remaining sprint — and more importantly, they prevent the cascading failure mode where one silent bug (wrong damage from wrong category, broken warp, missing story gate) compounds into an unplayable game that Claude Code can't diagnose without human eyes.

**Revised odds with these architectural changes: 45-50%.** Up from 30-35%. The single biggest mover is warp validation — it turns the #1 bug class into a compile-time error.

---

## Sprint Log

### Sprint 40 (Content)
- Added Route 35, National Park, Route 36, Route 37
- Added 10 new species (Nidoran♀/♂, Growlithe, Vulpix, Stantler, Venonat, Yanma, Sudowoodo, Hoppip, Skiploom)
- Added 19 new moves

### Sprint 41 (Content)
- Ecruteak City (20x18), Burned Tower (14x14), Ecruteak Gym (10x10) fully implemented
- Morty's Gym: Gastly lv21, Haunter lv21, Haunter lv23, Gengar lv25 (Fog Badge)
- Burned Tower: Rival trainer (Gastly lv20, Zubat lv20), Eusine NPC
- 7 new species: Magmar, Eevee, Vaporeon, Jolteon, Flareon, Espeon, Umbreon
- 6 new moves: Smog, Sludge, Selfdestruct, Haze, Pursuit, Fire Punch
- Burned Tower encounters: Koffing (35), Rattata (30), Zubat (15), Raticate (10), Magmar (10)

### Sprint 42 (QA)
**Bugs found and fixed:**
1. **Status moves bypassed accuracy** — Hypnosis (60%), Sing (55%), Sleep Powder (75%) always hit. Fixed: all moves now use accuracy + stage modifiers.
2. **100% accuracy moves ignored evasion stages** — Sand Attack had no effect on 100-accuracy moves. Fixed: stage modifiers apply to all moves; only skip roll if effective accuracy ≥ 100 after stages.
3. **Missing burn damage penalty** — Burned Pokemon should deal half Physical damage. Fixed: added `burn_mult = 0.5` in calc_damage for Physical moves when attacker is burned.
4. **Move category audit** — Verified Pursuit (Dark=Special), Fire Punch (Fire=Special), Sonic Boom (Normal=Physical) all correct per Gen 2 type-based split.
5. **GoldenrodCity NPC#1** at (20,15) on C_SOLID — moved to (21,15) on C_WALK.
6. **Warp audit**: All 112 warps verified landing on C_WALK. No bugs.
7. **IlexForest→Route34 warp** landing on C_SOLID — fixed in earlier QA commit.

**Next sprint (43)**: Route 38, Route 39, Olivine City. Wire Ecruteak east exit warps to Route 38. Species and moves for these routes already pre-staged in data.rs.