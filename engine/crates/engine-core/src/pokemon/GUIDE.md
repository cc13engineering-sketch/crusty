# Pokemon Gold: Johto Completion Guide

This is a **true 1:1 clone** of Pokemon Gold/Silver/Crystal, rebuilt from scratch on the Crusty engine. Every mechanic, formula, data value, and behavior is sourced directly from the `pret/pokecrystal` disassembly — the actual game ROM. This is not an "inspired-by" or a simplified recreation. It is a faithful reproduction of Gen 2 Pokemon down to the damage formula, stat calculation, type chart, experience curves, encounter tables, trainer parties, and battle engine behavior.

Agents update this file after each sprint. Refer to `ENGINE_POKEMON.md` for engine patterns and QA history.

**Goal: Complete the Johto region** — all 8 gyms, Elite Four, Champion, and critical path story events — playable from New Bark Town to credits. Every mechanic must match the original game.

**Autonomy: 10.** The developer is AFK. Do not ask questions. Do not wait for feedback. Make every decision yourself. If something breaks, fix it and keep moving. Ship it.

---

## MUST USE SOON

This is our master source for how to do tile art and maps for the game - https://github.com/nikouu/Pokemon-gen-2-style-tilemap

## Reference Data Sources

Prefer sources higher on the list when data conflicts.

### 1. `pret/pokecrystal` Disassembly (PRIMARY SOURCE OF TRUTH)

**LOCAL COPY: `engine/crates/engine-core/src/pokemon/pokecrystal-master/`**
Upstream: https://github.com/pret/pokecrystal

The fully disassembled source code of Pokémon Crystal. It IS the game. This is our **ultimate source of truth** for all data, mechanics, and design patterns. Everything else is secondary. Always read the corresponding `.asm` file FIRST before implementing any feature.

**IMPORTANT: All paths below are relative to `pokecrystal-master/`.**

#### Data Files — Species, Moves, Types

| File | What it contains | Format |
|------|-----------------|--------|
| `data/pokemon/base_stats/*.asm` | Per-species base stats, types, catch rate, exp yield, growth rate, TM/HM learnset, gender ratio, egg groups. 251 files, one per species. | `db HP, ATK, DEF, SPD, SAT, SDF` then metadata fields |
| `data/pokemon/evos_attacks.asm` | Every species' evolution method(s) AND full level-up learnset (3,357 lines). | `db EVOLVE_LEVEL, level, species` then `db level, move` pairs |
| `data/pokemon/egg_moves.asm` | Egg moves per species | `db move1, move2, ...` |
| `data/moves/moves.asm` | All 251 moves: animation, effect, power, type, accuracy, PP, effect chance (268 lines). | `move NAME, EFFECT, power, TYPE, acc, pp, chance` |
| `data/moves/effects.asm` | Move effect scripts — the step-by-step execution order for each effect type (e.g. NormalHit: checkobedience → usedmovetext → doturn → critical → damagestats → damagecalc → stab → damagevariation → checkhit → moveanim → failuretext → applydamage → criticaltext → supereffectivetext → checkfaint → buildopponentrage → kingsrock → endmove). **Critical for implementing move execution order correctly.** | Macro sequence per effect |
| `data/moves/critical_hit_moves.asm` | Moves with high crit ratio | List of move constants |
| `data/types/type_matchups.asm` | Complete type chart: every super-effective, not-very-effective, and immune matchup | `db ATTACKER, DEFENDER, EFFECTIVENESS` |
| `data/types/type_boost_items.asm` | Which held items boost which types (Charcoal→Fire, etc.) | `db item, type` |
| `data/types/badge_type_boosts.asm` | Which badge boosts which stat | `db badge, stat` |
| `data/growth_rates.asm` | EXP curve coefficients for all 6 growth rates | polynomial coefficients |
| `data/wild/johto_grass.asm` | Wild grass encounters per map, 3 time-of-day slots (morn/day/nite), 7 slots each | `def_grass_wildmons MAP` then `db level, species` × 7 × 3 |
| `data/wild/johto_water.asm` | Wild surfing encounters per map | Same format as grass |
| `data/wild/fish.asm` | Fishing encounters per group (Old/Good/Super Rod) | `db encounter_chance, species, level` |
| `data/items/attributes.asm` | Item effects, prices, pockets, parameters | Per-item attribute block |
| `data/items/marts.asm` | What each Poké Mart sells | `db item1, item2, ...` per mart |

#### Data Files — Trainers & Maps

| File | What it contains | Format |
|------|-----------------|--------|
| `data/trainers/parties.asm` | Every trainer's team (3,497 lines). Gym leaders, E4, Champion, Rival, all route trainers. | `db "NAME@", TYPE` then `db level, species[, item][, 4 moves]` |
| `data/trainers/attributes.asm` | Trainer AI flags per trainer class | AI layer bitmask |
| `data/maps/maps.asm` | Map metadata: tileset, environment type, landmark, music, time palette, fishing group | `map NAME, TILESET, ENV, LANDMARK, MUSIC, PHONE, PALETTE, FISHGROUP` |
| `data/maps/map_data.asm` | Map dimensions (width × height in blocks), tileset, connections | Per-map struct |
| `data/maps/spawn_points.asm` | Fly/respawn destinations | `spawn_point MAP, x, y` |
| `data/maps/flypoints.asm` | Fly menu destinations | Ordered list |
| `maps/*.asm` | Per-map script files: NPCs, events, warps, sign text, story flag checks, scene scripts. **Read these to understand how the original game gates progression.** | ASM script commands |
| `maps/*.blk` | Map block data (binary tile layout) | Binary |

#### Engine Code — Battle System Design Patterns

| File | What it contains | Lines |
|------|-----------------|-------|
| `engine/battle/core.asm` | **The entire battle engine.** Turn order, switching, fainting, EXP award, evolution trigger, wild vs trainer flow, linked battles. Read this to understand the battle state machine. | 9,147 |
| `engine/battle/effect_commands.asm` | Implementations of every effect macro (checkhit, damagecalc, critical, stab, applydamage, etc.). **The actual damage formula lives here.** | |
| `engine/battle/move_effects/*.asm` | 58 individual move effect files (curse, baton_pass, encore, attract, etc.). Each handles its unique mechanic. | 58 files |
| `engine/battle/ai/scoring.asm` | AI move scoring — how trainers pick moves. Layers: Basic (dismiss useless), Setup, Risky, Aggressive, Cautious. | |
| `engine/battle/ai/move.asm` | AI move selection orchestration | |
| `engine/battle/ai/items.asm` | AI item usage (gym leaders use potions, etc.) | |
| `engine/battle/ai/switch.asm` | AI switching logic | |
| `engine/battle/menu.asm` | Battle menu: Fight/Item/Switch/Run selection | |
| `engine/battle/start_battle.asm` | Battle initialization, intro animations | |

#### Engine Code — Overworld Design Patterns

| File | What it contains |
|------|-----------------|
| `engine/overworld/overworld.asm` | Main overworld loop, step processing |
| `engine/overworld/player_movement.asm` | Player movement, collision, tile interaction |
| `engine/overworld/map_objects.asm` | NPC object management, sprite loading |
| `engine/overworld/npc_movement.asm` | NPC pathfinding, random walk, scripted movement |
| `engine/overworld/events.asm` | Event processing, script triggers |
| `engine/overworld/scripting.asm` | Script command interpreter |
| `engine/overworld/warp_connection.asm` | Map transitions, warp logic, connections |
| `engine/overworld/wildmons.asm` | Wild encounter trigger logic (step counter, rates) |
| `engine/overworld/time.asm` | Time-of-day system (morn/day/nite transitions) |
| `engine/pokemon/evolve.asm` | Evolution execution, animation trigger |
| `engine/pokemon/move_mon.asm` | Move/delete moves, party management |

#### Constants — IDs & Flags

| File | What it contains |
|------|-----------------|
| `constants/pokemon_constants.asm` | Canonical species IDs (BULBASAUR=1 through CELEBI=251) |
| `constants/move_constants.asm` | Canonical move IDs (POUND=1 through BEAT_UP=251) |
| `constants/item_constants.asm` | Item IDs |
| `constants/type_constants.asm` | Type IDs (NORMAL=0, FIRE=1, ... STEEL=8, DARK=17) |
| `constants/event_flags.asm` | All story/event flags — gym badges, item pickups, story progression gates |
| `constants/engine_flags.asm` | Engine flags — badges, pokegear, fly points |
| `constants/battle_constants.asm` | Battle state constants |
| `constants/map_constants.asm` | Map group/number constants |
| `constants/trainer_constants.asm` | Trainer class + ID constants |

#### How To Use pokecrystal For Implementation

1. **Adding a new species**: Read `data/pokemon/base_stats/{name}.asm` for stats, `data/pokemon/evos_attacks.asm` for learnset and evolution, check `data/wild/johto_grass.asm` for where it appears.

2. **Adding a new move**: Read `data/moves/moves.asm` for stats, `data/moves/effects.asm` for execution sequence, check `engine/battle/move_effects/{name}.asm` if it has a unique effect.

3. **Implementing a trainer battle**: Read `data/trainers/parties.asm` for the exact team, `data/trainers/attributes.asm` for AI flags, and `maps/{Location}.asm` for the trigger script.

4. **Implementing a map/route**: Read `maps/{MapName}.asm` for events/NPCs/warps, `data/wild/johto_grass.asm` for encounters, `data/maps/map_data.asm` for dimensions and connections.

5. **Verifying battle mechanics**: Read `engine/battle/effect_commands.asm` for the actual damage formula implementation, `engine/battle/core.asm` for turn flow, and `data/moves/effects.asm` for the move execution pipeline.

6. **Understanding story gates**: Read `constants/event_flags.asm` for the flag name, then grep `maps/` for `checkevent EVENT_*` and `setevent EVENT_*` to see where flags are checked and set.

7. **Designing simulation tests**: Use the data files as ground truth. Parse `data/moves/moves.asm` to verify our move stats match. Parse `data/trainers/parties.asm` to verify trainer teams. Parse `data/types/type_matchups.asm` to verify our type chart. Parse `data/pokemon/evos_attacks.asm` to verify learnsets and evolution levels.

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

### 8. Tile Art Sources
The engine renders 16×16 tiles from 4-color indexed strings (`0`=transparent, `1`=light, `2`=medium, `3`=dark) with palettes applied at render time in `render.rs`. Current tiles are hand-coded in `sprites.rs`. For higher-quality tile art:

- **nikouu/Pokemon-gen-2-style-tilemap** — `https://github.com/nikouu/Pokemon-gen-2-style-tilemap` — Free, open source Gen 2 style tileset made from scratch. 8×8 base tiles, 4-color GBC palette. Includes grass, trees, paths, buildings, water, roofs, doors, flowers, signs. Comes with a full Johto reference map and detailed notes on how original tiles work. MIT-style license.
- **GibbonGL GB Crystal packs** — `https://gibbongl.itch.io/gbcrystal-base-tilesets` ($5 base, $8 bundle with trainer sprites). Made from scratch, 4-color, no Nintendo copyrighted materials. Commercial-safe. Covers caves, grass, water, trees, buildings with variety for different city styles. Companion character pack has Gen 2 style trainer sprites and portraits.
- **The Pixel Nook** — Free GB Studio overworld tiles and building packs in Pokémon style on itch.io.

**Tile art integration pipeline**: To replace the hand-coded `sprites.rs` tiles with art pack PNGs:
1. Load the PNG spritesheet
2. For each 16×16 (or 8×8) tile region, map pixel colors to the 4-color palette index (`0`/`1`/`2`/`3`)
3. Output a Rust `pub const TILE_XXX: &str = "..."` string (256 chars for 16×16, row-major)
4. The existing `decode_sprite()` function and palette system in `render.rs` handle the rest — no rendering changes needed

This conversion tool can be a standalone Rust script or a Python script. Run it once per art pack update. The engine's palette system (`PAL_OVERWORLD`, `PAL_PATH`, `PAL_BUILDING`, `PAL_WATER`, etc. in `render.rs`) maps the 4 indices to actual RGBA colors at render time, so the same tile data can be recolored by swapping palettes.

### 9. Music Sources

The engine currently has SFX only (`SoundCommand::PlayTone` — simple waveform blips pushed to JS-side Web Audio). Each map has a `music_id: u8` field that isn't yet wired to actual music playback. Here are the resources and integration approaches:

**MIDI files (free):**
- **VGMusic.com** — Fan-sequenced MIDIs for every Pokémon Gold/Silver track. Free. `https://www.vgmusic.com` (search "Pokemon Gold")
- **BitMidi** — Hosts GSC MIDIs with browser preview. Free. `https://bitmidi.com` (search "Pokemon Gold Silver Crystal")
- **KHInsider MIDI collection** — Full sets for Pokemon Gold and Gold/Silver/Crystal. Free download. `https://www.khinsider.com/midi/gameboy/pokemon-gold` and `https://www.khinsider.com/midi/gameboy/pok-mon-gold-silver`

**pokecrystal audio source (most accurate):**
- **`pret/pokecrystal/audio/music/`** — The actual music data in ASM macro format. Each `.asm` file is one track with 4 Game Boy channels. Can be compiled to a `.gbs` file (Game Boy Sound format).
- **Crystal Tracker** (`https://github.com/dannye/crystal-tracker`) — Desktop editor that reads/writes pokecrystal ASM music files with real-time playback. Can export tracks.
- **GF2MID** (`https://github.com/turboboy215/GF2MID`) — Converts Game Boy ROM music directly to MIDI.
- **gbs2midi** (`https://github.com/Thysbelon/gbs2midi`) — Converts GBS files to MIDI.

**Integration approaches (pick one):**

**Approach A — MIDI playback via JS (recommended for this sprint)**
Load MIDI files as static assets. Use a JS MIDI player library (Tone.js, MIDI.js, or a lightweight custom parser) to play them through Web Audio with square/triangle oscillators for authentic chiptune sound. On map transition, JS reads `music_id` from `global_state`, loads the corresponding MIDI, loops it. On battle start, crossfade to battle music. Simple, lightweight, good enough.

**Approach B — Pre-rendered audio loops**
Convert MIDIs to chiptune-style OGG/MP3 offline (using a tracker or DAW with Game Boy soundfonts). Host as static files. Play via `<audio>` element or `AudioBufferSourceNode`. Simplest JS integration, largest file sizes (~50KB-200KB per track).

**Approach C — GBS playback (most authentic, most complex)**
Compile pokecrystal's audio to a `.gbs` file. Use Game Music Emu compiled to WASM (`https://www.wothke.ch/webGSF/`) to play it in-browser. Authentic hardware sound. Most complex integration.

**Music-to-map mapping**: Reference `data/maps/map_headers.asm` in pokecrystal and the Data Crystal ROM map (`https://datacrystal.tcrf.net/wiki/Pokémon_Gold_and_Silver/ROM_map`) for which music_id plays where. Key mappings: New Bark Town, Route 29, Violet City, Azalea Town, Goldenrod City, Ecruteak City, Olivine City, battle themes (wild, trainer, gym leader, rival, E4/Champion), Pokemon Center, title screen.

---

## Remaining Work (Phase Checklist)

### Phase 0: Architectural Hardening
_Items not yet implemented are marked with ☐. Completed items marked ✓._

- ☐ **0A. Derive Move Category from Type** — Remove manual `category` field from `MoveData`. In Gen 2, category is determined by type. Add `PokemonType::gen2_category()` method. `MoveData::category()` returns Status if power==0, else derives from type. Eliminates the recurring physical/special misclassification bug.
- ✓ **0B. Warp Validation Test** — `test_all_warps_valid()` implemented Sprint 44. Catches warp destinations on bad tiles. Update the map list whenever adding new `MapId` variants.
- ☐ **0C. Story Flags + NPC Gating** — Add `StoryFlag` enum, `story_flags: u64` to `PokemonSim`, `has_flag()`/`set_flag()` helpers. Add `requires_flag`/`hidden_by_flag` to `NpcDef`. Blocking NPCs become data, not scattered if-statements.
- ☐ **0D. NPC Action as Data** — Add `NpcAction` enum (`Talk`, `Heal`, `Mart`, `GiveBadge`, `SetFlag`, `ConditionalDialogue`, etc.) to `NpcDef`. Replace `match (map_id, npc_idx)` blocks with `match npc.action`. Adding a gym leader never requires touching `mod.rs`.
- ✓ **0E. Debug State Export** — Implemented Sprint 58. Exports `player_x`, `player_y`, `current_map`, `badges`, `party_size`, `step_count`, `defeated_count`, `money`, `lead_hp`, `lead_level`, `lead_species` to `global_state` every `step()`. 8 headless integration tests verify title screen, starter selection, walking, determinism.
- ☐ **0F. File Splits** — Split `maps.rs` (currently >7,000 lines): keep types/enum/tests, move `build_*` to `maps_early.rs` and `maps_late.rs`. Split `mod.rs` when >4,000 lines: extract `battle.rs` and `menus.rs`.

### Phase 1: Data Tables ✓ (ongoing)
Species and moves added as needed per sprint. Currently ~125 species, ~147 moves. Continue adding for E4/Champion teams and remaining encounter tables.

### Phase 2: Maps — Olivine, Cianwood, Mahogany (Gyms 5-7) ✓
All maps built through Lake of Rage and Mahogany Gym. RocketHQ still needed for story gating.

### Phase 3: Maps — Blackthorn through Victory Road (Gym 8 + E4) ✓
All maps complete: Route 44, Ice Path, Blackthorn, Route 45, Route 46, Route 27, Route 26, Victory Road, Indigo Plateau, E4 rooms (Will/Koga/Bruno/Karen), Champion Lance.

### Phase 4: Trainer Teams ✓
All 8 gym leaders + all 5 E4/Champion wired with correct canonical teams. Verified against pret/pokecrystal Sprint 57.

**Rival** at Victory Road: starter final form lv36 + Haunter lv35 + Sneasel lv34 + Magneton lv34 + Golbat lv36 (not yet implemented)

### Phase 5: Move Effects & Battle Polish
**Priority 1 — Secondary effects**: Thunderbolt 10% paralysis, Ice Beam 10% freeze, Fire Blast 10% burn, Dragon Breath 30% paralysis, Crunch 20% def drop, Psychic 10% sp.def drop.

**Priority 2 — Status moves**: Haze (reset stages), Self-Destruct (user faints), Toxic (escalating), Confuse Ray/Swagger/Hypnosis (add `confused: u8`), Mean Look (add `trapped: bool`).

**Priority 3 — Multi-turn**: Fly, Dig, SolarBeam — only if E4 uses them.

### Phase 6: Story Events & Gating
Requires Phase 0C (story flags). No story gating exists yet.

**Critical gates**:
- **Olivine Gym**: Jasmine locked until `DeliveredMedicine` flag chain
- **Lake of Rage**: Forced Red Gyarados → `RedGyarados` → unblocks Mahogany Gym
- **Rocket HQ** (not yet built): Clear trainers → `RocketMahogany` → unblocks Route 44
- **Route blocks**: NPCs with `hidden_by_flag` at choke points

### Phase 7: Save System ✓
Implemented Sprint 61. Auto-saves on map transition. CONTINUE/NEW GAME title menu. Full state serialization including RNG. Read the traps below before modifying.

#### Trap #1: Persist command JSON format vs Sound command JSON format
Both `drain_persist_commands()` and `drain_sound_commands()` return JSON arrays from Rust. Both use a `"type"` field to identify command types. But the JS sound handler currently uses `if (cmd.PlayTone)` (checking for a nested object key) — this is WRONG for the flat `{"type":"PlayTone",...}` format that `to_json()` actually produces. The correct pattern for BOTH sound and persist is `cmd.type === "Set"` / `cmd.type === "PlayTone"`. When you implement persist handling, use `cmd.type`, not `cmd.Set`.

The persist JSON format from Rust is:
```json
[{"type":"Set","key":"pokemon_save","value":"{...escaped json...}"}]
```

The correct JS handler:
```javascript
// Replace the bare drain_persist_commands() call with:
try {
    const persistJson = drain_persist_commands();
    if (persistJson && persistJson !== '[]') {
        const cmds = JSON.parse(persistJson);
        for (const cmd of cmds) {
            if (cmd.type === 'Set') localStorage.setItem(cmd.key, cmd.value);
            else if (cmd.type === 'Remove') localStorage.removeItem(cmd.key);
            else if (cmd.type === 'Clear') {
                Object.keys(localStorage).filter(k => k.startsWith('pokemon_')).forEach(k => localStorage.removeItem(k));
            }
        }
    }
} catch(e) { console.warn('persist error', e); }
```

#### Trap #2: The value field is double-escaped JSON
When you `persist_set(queue, "pokemon_save", &json_blob)`, the `escape_json_string()` in `PersistCommand::to_json()` will escape the quotes inside your JSON blob. So the value arrives in JS as a string, not a parsed object. On the JS load side, you need `JSON.parse(localStorage.getItem("pokemon_save"))` — the value is a JSON string that needs a second parse.

#### Trap #3: Load must happen BEFORE setup_test_pokemon()
The init sequence in `index.html` is: `init(W, H)` → `set_url_param()` → `setup_test_pokemon()`. The `setup_test_pokemon()` call creates `PokemonSim::new()`. Your save-load check must be readable inside `PokemonSim::new()`. The approach:

1. **JS side (before `setup_test_pokemon()`)**: Read localStorage and push into WASM:
```javascript
const saveData = localStorage.getItem('pokemon_save');
if (saveData) {
    set_game_state_str('pokemon_save', saveData);
}
setup_test_pokemon();
```

2. **Rust side (inside `PokemonSim::new()`)**: Check if save data exists in global_state:
```rust
// At the start of PokemonSim::new(), check for saved game
let save_str = engine.global_state.get_str("pokemon_save");
if !save_str.is_empty() {
    // Parse JSON and reconstruct state
    return Self::load_from_save(&save_str);
}
// Otherwise, normal new game initialization...
```

But note: `PokemonSim::new()` doesn't take `engine` as a parameter — it creates a fresh sim. You'll need to either pass the save string into `new()`, or check global_state after construction in the `setup_test_pokemon` WASM binding in `lib.rs`.

#### Trap #4: MapId serialization/deserialization
You'll serialize `current_map_id` as `format!("{:?}", self.current_map_id)` which produces strings like `"EcruteakCity"`. To deserialize, you need a `MapId::from_str()` that matches these debug strings back to enum variants. This doesn't exist yet — you'll need to add it. A match block with every variant, or use the `strum` crate. Don't forget to handle the case where a save references a MapId that doesn't exist (return a default).

#### Trap #5: RNG state access
The save needs `engine.rng.state`. But the `Simulation::step()` signature gives you `&mut Engine`, not during save. You'll need to either:
- Save the rng state into a field on PokemonSim during each step (e.g., `self.last_rng_state = engine.rng.state`)
- Or access it during the persist command push (which happens inside step)

#### Recommended single-blob save format
```json
{
  "map": "EcruteakCity",
  "x": 5, "y": 8, "facing": 2,
  "money": 3000, "badges": 15, "flags": 0,
  "time": 12345.0, "rng": 8675309,
  "party": [
    {"species":155,"level":18,"hp":52,"max_hp":52,"exp":1200,"moves":[33,52,108,45],"pp":[35,25,20,40],"status":0}
  ],
  "pc": [],
  "defeated": [["VioletGym",0],["AzaleaGym",0]],
  "seen": [152,155,158,16,19],
  "caught": [155]
}
```

Build the JSON with `format!()` — don't pull in serde_json for this. The hand-built JSON in `persist.rs` and `sound.rs` shows the established pattern. Parse it back with a simple state machine or manual string splitting — the format is controlled by you so it doesn't need to be robust against arbitrary input.

**Auto-save**: Call `serialize_save()` and push to persist queue in `change_map()`. This means every map transition auto-saves, protecting against browser crashes.

**Title screen**: Add CONTINUE/NEW GAME options. If `global_state.get_str("pokemon_save")` is non-empty, show CONTINUE highlighted. CONTINUE calls `load_from_save()`. NEW GAME clears the save key and starts fresh.

### Phase 8: Credits ✓
Implemented Sprint 61. `GamePhase::Credits { scroll_y: f64 }`. Scrolling text on framebuffer. Triggered after Champion Lance. Returns to title.

---

## Headless Testing Gaps

The headless infrastructure (`headless/runner.rs`, `headless/scenario.rs`, `headless/replay.rs`) was built for pointer-driven physics games, not keyboard-driven RPGs. Sprint 58 proved `HeadlessRunner::run_sim()` works with `PokemonSim` via `InputFrame` — but several gaps remain before this game can be a full stress test for Crusty's deterministic tooling.

### Gap 1: ScheduledAction has no keyboard events (CRITICAL)
`GameScenario` drives input via `ScheduledAction`, which only has `PointerDown`, `PointerMove`, `PointerUp`. Pokemon is 100% keyboard-driven (`ArrowUp/Down/Left/Right`, `KeyZ`, `KeyX`). You cannot write a scenario that picks a starter, walks a route, or fights a battle using the current scenario system.

**Workaround (current)**: The Sprint 58 headless tests use `HeadlessRunner::run_sim()` directly with `InputFrame` arrays — this works because `InputFrame` has `keys_pressed`/`keys_held`. But you lose the `GameScenario` assertion framework.

**Fix**: Add `KeyDown { frame: u64, key: String }` and `KeyUp { frame: u64, key: String }` variants to `ScheduledAction`. Wire the dispatch in `GameScenario::run()` to push keys into `engine.input.keys_pressed`. This lets scenarios drive the full game: title screen → starter select → walk to gym → battle → verify badge earned.

### Gap 2: `state_hash()` doesn't capture game-specific state (CRITICAL)
`Engine::state_hash()` hashes ECS world state (transforms, rigidbodies, input keys). Pokemon doesn't use the ECS — it's a standalone `PokemonSim` struct. So `state_hash()` returns essentially the same value regardless of whether the player has 0 badges or 8. The `PlaythroughFile` and `DivergenceReport` systems rely on `state_hash` for determinism verification — but for Pokemon, they're comparing meaningless hashes.

**Fix**: Add `fn state_hash(&self) -> u64` to the `Simulation` trait (default returns 0). Have `Engine::state_hash()` XOR in the simulation's hash. Implement on `PokemonSim` to hash: `current_map_id`, `player.x`, `player.y`, `badges`, `story_flags`, `money`, `party.len()`, each party Pokemon's `species_id`/`level`/`hp`, `defeated_trainers.len()`, `phase` discriminant, and `battle` state if in battle. This makes `PlaythroughFile` verification and `DivergenceReport` actually detect Pokemon-specific determinism breaks.

### Gap 3: `GameScenario` uses legacy API, not `Simulation` trait
`GameScenario` takes raw function pointers (`setup_fn`, `update_fn`, `render_fn`), not a `Simulation` impl. Pokemon implements `Simulation`. No bridge exists. The Sprint 58 tests work around this by using `HeadlessRunner::run_sim()` directly.

**Fix**: Either migrate `GameScenario` to accept `&mut dyn Simulation`, or build a `PokemonScenario` wrapper that combines `run_sim()` with the `Assertion` framework from `scenario.rs`. The assertion types (`StateEquals`, `StateInRange`, `StateGreaterThan`) are reusable — they just need to be decoupled from the legacy `GameScenario` runner.

### Gap 4: No "observable state" contract on Simulation trait
The headless tools read `global_state` for metrics, but there's no trait method ensuring simulations export their state. Phase 0E added `export_debug_state()` called manually inside `step()`. If it's forgotten, headless tools see stale data.

**Fix**: Add `fn export_state(&self, engine: &mut Engine)` to the `Simulation` trait (default no-op). Have the runner call it after every `step()`. Move `export_debug_state()` into this method.

### Gap 5: Death classification doesn't map to RPG concepts
`DeathClass` (Cliff, Attrition, Blowout, CloseCall) was designed for games where a single metric declines to zero. In Pokemon, relevant terminal states are: party wipe (WhiteOut), softlock (position unchanged for N frames), all badges earned, E4 defeated, credits reached. The anomaly detector could catch party wipes via `lead_hp` drops but can't distinguish "lost to wild Rattata" from "lost to Champion Lance."

**Fix (future)**: Add Pokemon-specific classifications or export richer state (e.g., `game_outcome` key: "playing"/"whiteout"/"credits"/"stuck"). The anomaly detector's spike/plateau detection on `badges` and `defeated_count` already provides some regression value.

### Gap 6: No goal-seeking Policy for RPGs
`RandomPolicy` picks random keys. An RPG policy needs multi-step plans: walk to (5,10), press Z, navigate dialogue, select moves in battle. This requires map pathfinding, menu state machines, and battle AI — essentially an automated player.

**Fix (future, post-Johto)**: Build a `PokemonPolicy` that reads `player_x`/`player_y`/`current_map` from observations and follows a scripted waypoint list. This would enable fully automated regression playthroughs: "start new game, pick Cyndaquil, walk to Violet City, beat Falkner, verify Zephyr Badge." Major project but the ultimate stress test for deterministic replay.

### Test Authoring Infrastructure (Gaps 7-11)

Gaps 1-6 are engine-level changes. Gaps 7-11 are game-level additions that let agents write efficient, expressive headless tests without engine changes.

#### Gap 7: No InputFrame builder API (BLOCKS TEST AUTHORING)
Writing headless tests currently requires manually constructing `InputFrame` structs with `keys_pressed: vec!["ArrowRight".into()]` for every frame. A test that walks from Elm Lab to Violet City needs hundreds of frames of boilerplate. This makes agents reluctant to write complex integration tests.

**Fix**: Add a `test_helpers` module (or functions at the top of the headless_tests module) with ergonomic builders:

```rust
fn press(key: &str) -> InputFrame {
    InputFrame { keys_pressed: vec![key.into()], ..Default::default() }
}
fn hold(key: &str, frames: usize) -> Vec<InputFrame> {
    (0..frames).map(|_| InputFrame { keys_held: vec![key.into()], ..Default::default() }).collect()
}
fn wait(frames: usize) -> Vec<InputFrame> {
    vec![InputFrame::default(); frames]
}
fn walk_right(tiles: usize) -> Vec<InputFrame> {
    // Each tile = press + hold for walk animation frames + release
    let mut frames = Vec::new();
    for _ in 0..tiles {
        frames.push(press("ArrowRight"));
        frames.extend(hold("ArrowRight", 7)); // 8 frames per tile at walk speed
    }
    frames
}
fn sequence(steps: &[(&str, usize)]) -> Vec<InputFrame> {
    steps.iter().flat_map(|(key, count)| {
        if key.is_empty() { wait(*count) } else { hold(key, *count) }
    }).collect()
}
```

~40 lines. Without this, every test is 50+ lines of InputFrame construction and agents won't write complex scenarios.

#### Gap 8: No mid-run assertions (LIMITS TEST EXPRESSIVENESS)
`run_sim()` returns `SimResult` with final state only. You can't assert "badges == 0 at frame 50 AND badges == 1 at frame 200." The legacy `run_with_frame_cb` has a per-frame callback, but the `Simulation` trait path (`run_sim`/`run_sim_frames`) does not.

**Fix**: Add `run_sim_with_checkpoints` to `HeadlessRunner`:

```rust
pub fn run_sim_with_checkpoints<S: Simulation>(
    &mut self,
    sim: &mut S,
    seed: u64,
    inputs: &[InputFrame],
    frames: u64,
    checkpoints: &[(u64, &str, f64, f64)], // (frame, key, expected, tolerance)
    config: RunConfig,
) -> (SimResult, Vec<(u64, String, bool, String)>) // (frame, key, passed, detail)
```

This lets a single test verify a multi-step sequence (walk to gym → fight → verify badge → walk out) without splitting into separate runs that each need fresh setup. Alternatively, accept `Vec<(u64, Box<dyn Fn(&Engine) -> Result<(), String>>)>` for arbitrary predicates.

#### Gap 9: No direct PokemonSim state access from HeadlessRunner tests
Headless tests using `HeadlessRunner` can only read `global_state` f64 values. The exported keys are limited — no `current_phase` discriminant, no `battle.enemy.species_id`, no `party[1].hp`. The existing `headless_tests` module inside `mod.rs` can access `PokemonSim` fields directly (private access within the same module), but `HeadlessRunner`-based integration tests cannot.

**Fix**: Add `pub fn test_snapshot(&self) -> PokemonTestSnapshot` to `PokemonSim`:

```rust
#[derive(Debug, Clone)]
pub struct PokemonTestSnapshot {
    pub map_id: MapId,
    pub x: i32, pub y: i32,
    pub badges: u32, pub money: u32,
    pub story_flags: u64,
    pub phase: String, // discriminant name
    pub party: Vec<(SpeciesId, u8, u16, u16)>, // (species, level, hp, max_hp)
    pub in_battle: bool,
    pub enemy_species: Option<SpeciesId>,
    pub defeated_count: usize,
}
```

This gives integration tests rich assertions without bloating `global_state` exports on every frame.

#### Gap 10: No test-mode initial state constructor (BLOCKS EFFICIENT TESTING)
Every gym test currently needs to replay from the title screen — picking a starter, walking through dialogue, navigating to the gym. What you actually want: construct a `PokemonSim` at a specific game state and run from there.

**Fix**: Add `PokemonSim::with_state()` — a test constructor that skips the title screen:

```rust
impl PokemonSim {
    #[cfg(test)]
    pub fn with_state(
        map: MapId, x: i32, y: i32,
        party: Vec<Pokemon>, badges: u32,
        flags: u64, money: u32,
    ) -> Self {
        let mut sim = Self::new();
        sim.change_map(map, x, y);
        sim.party = party;
        sim.badges = badges;
        sim.story_flags = flags;
        sim.money = money;
        sim.has_starter = true;
        sim.phase = GamePhase::Overworld;
        sim
    }
}
```

This is the difference between a 30-frame test and a 3000-frame test. A gym badge test becomes: construct sim with 7 badges and a L100 Typhlosion at the gym entrance, run 200 frames of "walk to leader, press Z, spam attack," assert `badges == 8`.

#### Gap 11: No RNG call counting for determinism auditing
The GUIDE's determinism section requires stable RNG call order and dummy calls in branches, but there's no way to verify compliance. If someone adds a conditional `rng.next_f64()` that only fires during battle, replays silently desync with no test failure.

**Fix**: Add `call_count: u64` to `SeededRng` that increments on every `next_f64()` / `next_u64()` call. Export it to `global_state` as `rng_call_count`. Then determinism tests can assert:

```rust
// Run same inputs twice with same seed
let r1 = runner.run_sim(&mut sim1, 42, &inputs, config.clone());
let r2 = runner.run_sim(&mut sim2, 42, &inputs, config);
// RNG was called the same number of times
assert_eq!(
    r1.get_f64("rng_call_count"),
    r2.get_f64("rng_call_count"),
);
// State hashes match (requires Gap 2 fix)
assert_eq!(r1.state_hash, r2.state_hash);
```

If a code change adds a conditional RNG call, the count diverges between runs that take different branches, and the determinism test catches it before it becomes a replay desync bug in production.

---

## Technical Notes

**Tile rendering**: 16×16 tiles, 4-color indexed strings in `sprites.rs`, palettes in `render.rs`. `decode_sprite()` converts string → pixel buffer. See "Tile Art Sources" section above for upgrade path to proper art packs.

**Battle sprites**: JS overlay loads from CDN via `global_state` bridge. No Rust-side battle sprites needed.

**Overworld sprites**: Reuse existing `sprite_id` palette (0-7). Tile constants for gym interiors: BLACK + FLOOR pattern.

**HMs**: Surf — don't implement, design around water. Fly — menu warp to visited cities. Cut/Strength/Whirlpool/Waterfall — skip, design maps without them.

**Pokemon Center**: Shared `MapId::PokemonCenter`. Exit returns to source city via `last_pokecenter_map`.

**Compilation**: `cargo check` after every unit. `cargo test` after every map batch (catches warp bugs). WASM build only for browser testing.

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

## Missing Transition Screens & UX Sequences

A complete catalogue of screens, animations, and text sequences that the original Pokemon Gold/Silver presents to the player but are currently missing or incomplete in the Crusty recreation. Organized by when they occur during gameplay. Reference: `pret/pokecrystal` engine source, specifically `engine/battle/`, `engine/pokemon/`, `engine/menus/`, and `data/text/`.

### Battle Start Transitions

| # | Transition | Current State | What Original Does |
|---|-----------|---------------|-------------------|
| 1 | **Wild encounter screen flash** | ✓ Exists (`EncounterTransition`) | Screen flashes black/white 3-4 times, then wipes to battle screen |
| 2 | **Trainer encounter screen flash** | ✓ Exists (same as wild) | Different flash pattern — diagonal wipe for trainers vs vertical for wild |
| 3 | **Trainer intro slide-in** | ❌ Missing | "You are challenged by BUGCATCHER WADE!" text, trainer sprite slides in from right |
| 4 | **Wild Pokemon intro** | Partial | "Wild PIDGEY appeared!" — text exists but no sprite slide-in animation |
| 5 | **Player Pokemon send-out** | ❌ Missing | "Go! CYNDAQUIL!" with pokeball throw arc animation, Pokemon materializes from ball |
| 6 | **Enemy trainer send-out** | ❌ Missing | Trainer sprite slides out, Pokemon materializes. "FALKNER sent out PIDGEY!" |

### During Battle — Attack Sequences

| # | Transition | Current State | What Original Does |
|---|-----------|---------------|-------------------|
| 7 | **Move name announcement** | ✓ Exists | "CYNDAQUIL used EMBER!" text |
| 8 | **Move animation** | ❌ Missing | Each move has a unique screen animation — Ember shows fire particles, Surf shows a wave, Thunderbolt shows lightning. ~250 unique animations in Gen 2. Minimum viable: screen flash + sprite shake for physical, screen flash for special |
| 9 | **Damage number/HP bar drain** | Partial | HP bar exists but drains instantly. Original drains smoothly over ~0.5s with a ticking sound |
| 10 | **Critical hit text** | ✓ Exists | "A critical hit!" |
| 11 | **Effectiveness text** | ✓ Exists | "It's super effective!" / "It's not very effective..." / "It had no effect!" |
| 12 | **Multi-hit display** | ❌ Missing | "Hit 2 times!" / "Hit 3 times!" for moves like Fury Swipes, Double Kick |
| 13 | **Recoil text** | Partial | Struggle recoil exists. Missing: "POKEMON is hit with recoil!" for Take Down, Double-Edge |
| 14 | **Miss text** | ✓ Exists | "POKEMON's attack missed!" |

### During Battle — Status & Effects

| # | Transition | Current State | What Original Does |
|---|-----------|---------------|-------------------|
| 15 | **Status infliction text** | ✓ Exists | "PIDGEY was burned!" / "PIDGEY was poisoned!" etc. |
| 16 | **Status condition activation text** | Partial | Sleep/freeze/para skip text exists. Missing: burn damage text ("PIDGEY is hurt by its burn!"), poison damage text ("PIDGEY is hurt by poison!") per turn |
| 17 | **Stat change text** | ✓ Exists | "PIDGEY's ATTACK fell!" / "CYNDAQUIL's SPEED rose!" |
| 18 | **Stat change sharply text** | ❌ Missing | "PIDGEY's DEFENSE sharply fell!" for -2 stages, "PIDGEY's ATTACK rose sharply!" for +2 |
| 19 | **Stat won't go higher/lower** | ❌ Missing | "PIDGEY's ATTACK won't go any higher!" when at +6, "...won't go any lower!" at -6 |
| 20 | **Weather text** | ❌ Missing (no weather) | "Rain continues to fall." / "The sandstorm rages." per turn |
| 21 | **Wrap/Bind/Fire Spin damage** | ❌ Missing | "PIDGEY is hurt by WRAP!" trapping damage text each turn |
| 22 | **Leech Seed drain** | ❌ Missing (no Leech Seed) | "PIDGEY's health is sapped by LEECH SEED!" |
| 23 | **Nightmare damage** | ❌ Missing | "PIDGEY is locked in a NIGHTMARE!" |
| 24 | **Curse damage (Ghost)** | ❌ Missing | "PIDGEY is afflicted by the CURSE!" |

### During Battle — Fainting & Rewards

| # | Transition | Current State | What Original Does |
|---|-----------|---------------|-------------------|
| 25 | **Faint animation** | Partial | HP reaches 0. Missing: sprite drops off screen with a cry, brief pause |
| 26 | **Faint text** | ✓ Exists | "Wild PIDGEY fainted!" / "Enemy PIDGEY fainted!" |
| 27 | **EXP gain text** | ❌ Missing | "CYNDAQUIL gained 120 EXP. Points!" — this is a key feel moment |
| 28 | **EXP bar fill animation** | ❌ Missing | EXP bar fills smoothly. If it wraps around (level up), it fills to max, resets, fills to new amount |
| 29 | **Money gain text (trainer)** | ❌ Missing | "Player got $1200 for winning!" after trainer battle |

### Level Up Sequence

| # | Transition | Current State | What Original Does |
|---|-----------|---------------|-------------------|
| 30 | **Level up fanfare** | Partial | SFX exists. Missing: the full sequence below |
| 31 | **Level up text** | ✓ Exists | "CYNDAQUIL grew to LV. 14!" |
| 32 | **Stat change display** | ❌ Missing | Full stat screen showing old → new stats with +N for each stat. This is an entire screen the player sees every level up. Shows HP, Attack, Defense, Sp.Atk, Sp.Def, Speed with the gains highlighted |

### New Move Learning Sequence

This is the biggest missing UX flow. In the original, this is a multi-screen interactive sequence that happens every time a Pokemon reaches a level where it learns a new move.

| # | Transition | Current State | What Original Does |
|---|-----------|---------------|-------------------|
| 33 | **Wants to learn text** | ❌ Missing | "CYNDAQUIL is trying to learn FLAME WHEEL." |
| 34 | **But can't learn more text** | ❌ Missing | "But, CYNDAQUIL can't learn more than four moves." |
| 35 | **Delete a move prompt** | ❌ Missing | "Delete an older move to make room for FLAME WHEEL?" YES/NO |
| 36 | **Move select screen** | ❌ Missing | If YES: shows all 4 current moves with PP, player picks which to forget. Full move summary screen with type/PP/power |
| 37 | **Forget confirmation** | ❌ Missing | "1, 2, and... Poof! CYNDAQUIL forgot TACKLE." |
| 38 | **Move learned text** | ❌ Missing | "And... CYNDAQUIL learned FLAME WHEEL!" |
| 39 | **Give up learning text** | ❌ Missing | If NO or cancel: "Stop learning FLAME WHEEL?" YES/NO. If YES: "CYNDAQUIL did not learn FLAME WHEEL." |
| 40 | **Auto-learn (< 4 moves)** | Partial | Currently auto-fills empty slots. Missing: "CYNDAQUIL learned EMBER!" text confirmation |

Currently the game silently fills empty move slots and never prompts the player when all 4 slots are full. This means Pokemon can never replace old moves with new ones — a fundamental gameplay mechanic.

### Evolution Sequence

| # | Transition | Current State | What Original Does |
|---|-----------|---------------|-------------------|
| 41 | **Evolution trigger text** | ❌ Missing | "What? CYNDAQUIL is evolving!" |
| 42 | **Evolution animation** | ❌ Missing | Screen goes dark, sprite morphs/flashes between old and new forms for ~5 seconds with a distinctive sound effect |
| 43 | **Evolution complete text** | ❌ Missing | "Congratulations! Your CYNDAQUIL evolved into QUILAVA!" |
| 44 | **Evolution cancel** | ❌ Missing | Player can press B during the animation to cancel. "Huh? CYNDAQUIL stopped evolving!" |
| 45 | **Post-evolution move learn** | ❌ Missing | Some Pokemon learn a move upon evolving (e.g., Butterfree learns Confusion). This triggers the full move learning sequence above |

Currently evolution happens silently — `pending_evolution` is set, species changes, stats recalc. No text, no animation, no player interaction, no cancel option.

### Catching Sequence

| # | Transition | Current State | What Original Does |
|---|-----------|---------------|-------------------|
| 46 | **Ball throw animation** | ❌ Missing | Pokeball arc from player to enemy Pokemon |
| 47 | **Ball shake animation** | ❌ Missing | Ball lands, shakes 0-3 times. Each shake has a wobble + pause. Huge tension moment |
| 48 | **Catch success text** | Partial | "Gotcha! PIDGEY was caught!" — may exist but no shake animation leads into it |
| 49 | **Nickname prompt** | ❌ Missing | "Give a nickname to the caught PIDGEY?" YES/NO → keyboard if yes |
| 50 | **Pokedex registration** | ❌ Missing | If new species: "PIDGEY's data was added to the POKEDEX." Brief Pokedex entry screen with species art, type, height, weight, description |
| 51 | **Party full — PC transfer** | ❌ Missing | "PIDGEY was transferred to BILL's PC." if party has 6 Pokemon |

### Trainer Battle Bookend Sequences

| # | Transition | Current State | What Original Does |
|---|-----------|---------------|-------------------|
| 52 | **Trainer defeat text** | ❌ Missing | "BUGCATCHER WADE was defeated!" |
| 53 | **Trainer post-battle dialogue** | ❌ Missing | Each trainer has a defeat quote: "Whoa! You're something else!" |
| 54 | **Badge acquisition screen** | ❌ Missing | Gym leader: "<PLAYER> received the ZEPHYR BADGE!" + badge icon display + brief explanation of badge effect ("Pokemon up to Lv. 20 will obey you.") |
| 55 | **TM received text** | ❌ Missing | "FALKNER: Here, take this." "Received TM31 - MUD-SLAP!" |
| 56 | **Trainer next Pokemon text** | Partial | When trainer sends next: "FALKNER is about to use PIDGEOTTO. Will you change POKEMON?" YES/NO prompt. Currently just sends without asking |

### Overworld Transitions

| # | Transition | Current State | What Original Does |
|---|-----------|---------------|-------------------|
| 57 | **Map transition fade** | ❌ Missing | Screen fades to black on map change, fades back in on new map. Currently instant cut |
| 58 | **Building enter/exit** | ❌ Missing | Brief fade to black when entering a door, fade in inside. Same when exiting |
| 59 | **Cave enter darkness** | ❌ Missing | Entering a cave: brief flash, possibly darker palette |
| 60 | **Repel wore off** | ❌ Missing (no repel system) | "REPEL's effect wore off!" — text pops up in overworld |
| 61 | **Poison overworld damage** | ❌ Missing | In Gen 2, poisoned Pokemon take 1 HP damage every 4 steps in the overworld. Screen flashes, "CYNDAQUIL is hurt by poison!" If HP reaches 1, poison is cured |
| 62 | **Egg hatch sequence** | ❌ Missing (no eggs) | "Huh?" → "..." → "Your EGG is hatching!" → animation → "TOGEPI hatched from the EGG!" |

### Pokemon Center & Healing

| # | Transition | Current State | What Original Does |
|---|-----------|---------------|-------------------|
| 63 | **Nurse Joy dialogue** | Partial | Dialogue exists. Missing: the full "We'll take your Pokemon for a few seconds." → pokeball placement animation → jingle → "Your Pokemon are fully healed!" |
| 64 | **Healing jingle** | ❌ Missing | Distinctive 6-note healing melody that plays while balls are on the machine |
| 65 | **Ball placement animation** | ❌ Missing | Player's pokeballs slide onto the healing machine tray |

### Menu & Item Use

| # | Transition | Current State | What Original Does |
|---|-----------|---------------|-------------------|
| 66 | **Item use in battle text** | Partial | Potion exists. Missing: "PLAYER used POTION! CYNDAQUIL's HP was restored by 20 points." |
| 67 | **Item use outside battle** | ❌ Missing | "PLAYER used POTION on CYNDAQUIL. CYNDAQUIL's HP was restored." |
| 68 | **Antidote/status heal text** | ❌ Missing | "CYNDAQUIL was cured of poisoning!" |
| 69 | **Revive text** | ❌ Missing | "CYNDAQUIL was revived!" |
| 70 | **PP restore text** | ❌ Missing | "CYNDAQUIL's EMBER PP was restored." (Ether/Elixir) |
| 71 | **Rare Candy level up** | ❌ Missing | Uses Rare Candy → triggers full level up sequence (stat screen + possible move learn + possible evolution) |
| 72 | **TM/HM use sequence** | ❌ Missing | "Booted up a TM!" → "It contained MUDSLAP." → "Teach MUDSLAP to CYNDAQUIL?" → move replace flow if 4 moves |
| 73 | **Evolution stone use** | ❌ Missing | Same as evolution sequence but triggered by item |

### Whiteout / Game Over

| # | Transition | Current State | What Original Does |
|---|-----------|---------------|-------------------|
| 74 | **Whiteout text** | Partial | Money loss shown. Missing: "PLAYER is out of usable POKEMON!" → screen fades → "PLAYER whited out!" → fade to PokeCenter |
| 75 | **Whiteout fade sequence** | ❌ Missing | Distinctive slow fade to white (not black), then wake up in PokeCenter |

### Save & Title Screen

| # | Transition | Current State | What Original Does |
|---|-----------|---------------|-------------------|
| 76 | **Save confirmation** | Partial | "Game saved!" exists. Missing: "Would you like to save the game?" YES/NO → "Saving... Don't turn off the power." → brief pause → "Player saved the game." |
| 77 | **Title screen Pokemon cry** | ❌ Missing | Ho-Oh (Gold) or Lugia (Silver) cry plays on title, legendary sprite animates |
| 78 | **Continue screen stats** | ❌ Missing | CONTINUE option shows: Player name, badges, Pokedex count, time played |
| 79 | **New Game overwrite warning** | Partial | Warning may exist. Original: "There is already a save file. Is it OK to overwrite?" → "The previously saved file will be lost." confirmation |

### Pokedex Screens

| # | Transition | Current State | What Original Does |
|---|-----------|---------------|-------------------|
| 80 | **Pokedex entry seen** | ❌ Missing | When viewing a seen-but-not-caught Pokemon: shows silhouette, type, area |
| 81 | **Pokedex entry caught** | ❌ Missing | Full entry: sprite, name, type, height, weight, description text that scrolls, area map, cry playback |
| 82 | **Pokedex completion count** | ❌ Missing | "SEEN: 45 OWN: 12" at top of Pokedex list |

---

### Priority Ranking for Implementation

**Must-have (game feels broken without these):**
- #33-40: New move learning sequence 
- #41-45: Evolution sequence (evolution is silent/invisible)
- #27-28: EXP gain text + bar (player has no feedback on progress)
- #32: Level up stat display (key RPG feedback loop)
- #57-58: Map transition fades (jarring instant cuts)
- #54: Badge acquisition screen (reward moment is invisible)

**Should-have (game feels incomplete without these):**
- #9: Smooth HP bar drain (most visible animation in battles)
- #46-48: Catch ball shake sequence (catching has no tension)
- #29: Money gain text
- #52-53: Trainer defeat text + post-battle quote
- #56: Trainer switch prompt ("Will you change POKEMON?")
- #74-75: Whiteout fade sequence
- #5-6: Pokemon send-out animation

**Nice-to-have (polish):**
- #8: Move animations (massive scope — start with screen flash categories)
- #50: Pokedex registration on catch
- #63-65: Pokemon Center healing animation
- #76: Save confirmation dialogue
- #78: Continue screen stats display
- #80-82: Full Pokedex entry screens

## Definition of Done

Don't cut corners. Don't leave TODOs. Every map gets correct encounter tables. Every gym leader gets their real team. Every move in a trainer's Pokemon's learnset works. The goal is not "good enough" — it's as complete and correct as possible.

## Test Reliability Warning

**Headless simulation tests are NOT a source of truth.** They verify structural properties (warp existence, field values, data integrity) but CANNOT fully simulate the real browser environment. Known gaps:
- Tests don't exercise the JS rendering layer, sprite loading, or localStorage
- HeadlessRunner skips input timing nuances (frame-perfect presses, held keys vs taps)
- Save/load tests verify serialization round-trip but not the WASM↔JS persist pipeline
- Battle flow tests verify phase transitions but may miss render-side bugs
- Tests can pass with green checkmarks while the real game has obvious broken behavior

**Always verify fixes by playing the actual game in the browser.** Tests catch regressions and structural errors but are no substitute for real QA.

---

## Crucial Abstractions for Pokemon Gold Recreation

Five abstractions that would most increase the odds of completing a faithful Pokemon Gold recreation on Crusty. Ordered by impact — each one eliminates a class of problems that compounds over time and across sprints.

---

### 1. Event Script System (replaces imperative story logic)

**The problem it solves**: Every story event is currently a bespoke function in `mod.rs` — `check_rival_battle()`, `check_victory_road_rival()`, `check_sprout_tower_elder()`, `check_red_gyarados()`, `check_sudowoodo()`. Each one is ~30-50 lines of Rust with hardcoded map IDs, coordinates, dialogue strings, and flag checks. Adding the Jasmine medicine chain requires writing another custom function. Adding Team Rocket Radio Tower requires another. Adding the Dragon's Den sequence requires another. Every function touches `mod.rs`, which is already 5000+ lines. The agents can write these functions, but they can't refactor them, and they accumulate coupling to `PokemonSim` internals that makes future changes fragile.

The original game solved this with a script interpreter. Every map has a script file (`maps/MahoganyTown.asm`) containing event commands — `checkevent`, `setevent`, `writetext`, `startbattle`, `warp`, `disappear`, `appear`, etc. NPCs aren't "trainers" or "healers" by struct field — they're objects with attached scripts that run when interacted with. The script system is the single most important architectural decision in pokecrystal. It's what lets one game have 250+ maps with hundreds of unique story events without the engine code growing proportionally.

**The abstraction**: A data-driven event system. Not a full scripting language — just a `Vec<EventStep>` on each NPC or map trigger.

```rust
#[derive(Clone, Debug)]
pub enum EventStep {
    // Dialogue
    Text(Vec<String>),
    YesNo { yes: Vec<EventStep>, no: Vec<EventStep> },
    
    // Conditions
    RequireFlag(u64),         // skip rest if flag not set
    RequireNoFlag(u64),       // skip rest if flag IS set
    RequireBadges(u32),       // skip rest if badges < N
    RequireItem(u8),          // skip rest if item not in bag
    
    // Effects
    SetFlag(u64),
    GiveBadge(u8),
    GiveItem(u8),
    GivePokemon(SpeciesId, u8),    // species, level
    TakeMoney(u32),
    GiveMoney(u32),
    Heal,
    OpenMart,
    
    // Battle
    StartWildBattle(SpeciesId, u8), // forced encounter
    StartTrainerBattle,             // uses NPC's trainer_team
    
    // Map/NPC manipulation  
    Warp(MapId, i32, i32),
    HideNpc(MapId, u8),       // permanently hide NPC by index
    ShowNpc(MapId, u8),
    
    // Transitions
    FadeOut,
    FadeIn,
    PlaySfx(u8),
    Wait(f64),                // seconds
}
```

Then `NpcDef` gets `pub on_interact: &'static [EventStep]` instead of `dialogue` + `is_mart` + `is_trainer` + the `sprite_id == 4` nurse hack. The NPC interaction handler becomes a 50-line `execute_event_steps()` loop that replaces 500+ lines of `match (map_id, npc_idx)` blocks.

**Why this is the highest-impact abstraction**: It's the difference between O(N) code growth (one function per story event) and O(1) code growth (one interpreter, N data entries). The Jasmine medicine chain becomes data, not code:

```rust
// Lighthouse Ampharos NPC (sick)
on_interact: &[
    EventStep::RequireNoFlag(FLAG_DELIVERED_MEDICINE),
    EventStep::Text(vec!["AMPHY looks very sick...".into(), "It needs medicine from CIANWOOD.".into()]),
    EventStep::SetFlag(FLAG_MEDICINE),
],

// Cianwood Pharmacy NPC
on_interact: &[
    EventStep::RequireFlag(FLAG_MEDICINE),
    EventStep::RequireNoFlag(FLAG_GOT_SECRETPOTION),
    EventStep::Text(vec!["Here's the SECRETPOTION.".into()]),
    EventStep::GiveItem(ITEM_SECRETPOTION),
    EventStep::SetFlag(FLAG_GOT_SECRETPOTION),
],

// Lighthouse Ampharos NPC (with medicine)
on_interact: &[
    EventStep::RequireFlag(FLAG_GOT_SECRETPOTION),
    EventStep::RequireNoFlag(FLAG_DELIVERED_MEDICINE),
    EventStep::Text(vec!["You used the SECRETPOTION!".into(), "AMPHY is feeling better!".into()]),
    EventStep::SetFlag(FLAG_DELIVERED_MEDICINE),
],
```

No new functions in mod.rs. No new `check_*` methods. The agent just adds data to maps.rs.

**Sprint cost**: ~2 sprints to implement the interpreter and migrate existing events. Saves ~1 sprint per 10 story events after that.

---

### 2. Battle Phase Sequencer (replaces nested Box<BattlePhase> chains)

**The problem it solves**: Every battle action is a manually constructed chain of `BattlePhase::Text { message, timer, next_phase: Box::new(BattlePhase::Text { ... Box::new(BattlePhase::EnemyAttack { ... }) }) }`. The confusion implementation alone has 3 nested phases. Self-Destruct mutual KO required careful phase ordering across 4 different code paths. The Sprint 64 status move fix was a one-line conceptual change (`damage > 0` → `damage > 0 || is_status_move`) but touched 6 different phase chain constructions.

This nesting pattern is the #1 source of battle bugs. The agent builds a chain, gets one link wrong, and the battle state machine goes to the wrong place. You can't unit-test individual links — you can only test the whole chain end-to-end.

**The abstraction**: A battle event queue instead of nested phases.

```rust
struct BattleSequence {
    steps: VecDeque<BattleStep>,
}

enum BattleStep {
    Text(String),
    AnimateAttack { attacker: Side, move_id: MoveId },
    ApplyDamage { target: Side, amount: u16 },
    DrainHpBar { target: Side, to: u16, duration: f64 },
    CheckFaint { target: Side },
    GainExp { amount: u32 },
    LevelUp,
    LearnMove { move_id: MoveId },     // triggers move learn flow
    Evolve { into: SpeciesId },         // triggers evolution flow
    InflictStatus { target: Side, status: StatusCondition },
    StatChange { target: Side, stat: usize, stages: i8 },
    Weather(WeatherType),
    Pause(f64),
    FadeOut,
    FadeIn,
    SwitchPrompt,
    ReturnToActionSelect,
}
```

Then `execute_attack()` pushes steps onto the queue:

```rust
fn execute_attack(&self, attacker: Side, move_id: MoveId, ...) -> Vec<BattleStep> {
    let mut steps = vec![];
    steps.push(BattleStep::Text(format!("{} used {}!", name, move_name)));
    if missed {
        steps.push(BattleStep::Text(format!("{}'s attack missed!", name)));
        return steps;
    }
    steps.push(BattleStep::AnimateAttack { attacker, move_id });
    steps.push(BattleStep::ApplyDamage { target, amount: damage });
    steps.push(BattleStep::DrainHpBar { target, to: new_hp, duration: 0.5 });
    if crit { steps.push(BattleStep::Text("A critical hit!".into())); }
    if super_effective { steps.push(BattleStep::Text("It's super effective!".into())); }
    // Secondary effect
    if rng < burn_chance { 
        steps.push(BattleStep::InflictStatus { target, status: Burn });
        steps.push(BattleStep::Text(format!("{} was burned!", target_name)));
    }
    steps.push(BattleStep::CheckFaint { target });
    steps
}
```

The sequencer pops one step at a time, renders it, and advances. No nesting. No Box. Each step is independently testable. The missing transitions (#27 EXP text, #32 stat display, #33-40 move learn flow, #41-45 evolution flow) become new `BattleStep` variants — they slot into the queue at the right position without restructuring existing chains.

**Why this is high-impact**: Every item on the Missing Transitions list that involves battle (items #5-56) becomes trivial to add. You just push more steps onto the queue. Currently, adding EXP text + bar animation requires restructuring the `EnemyFainted` phase handler's 80-line block of nested phase construction. With a sequencer, it's `steps.push(BattleStep::Text(format!("Gained {} EXP!", exp))); steps.push(BattleStep::GainExp { amount: exp });`.

**Sprint cost**: ~3 sprints to implement and migrate. This is the most expensive abstraction but has the highest long-term payoff — every future battle feature is 5x easier.

---

### 3. Transition System (replaces instant state changes)

**The problem it solves**: The Missing Transitions document lists 82 items. Most of them are variations of "X happens instantly but should have a text/animation/fade/sound before and after." Map changes are instant cuts. Evolution is silent. Move learning auto-fills slots. Badge rewards are invisible. Every one of these requires the agent to find the exact line where the state change happens and wrap it in dialogue/phase construction. There's no shared infrastructure for "do a thing with a transition."

**The abstraction**: A transition queue that sits between game logic and rendering.

```rust
enum Transition {
    Fade { direction: FadeDir, duration: f64 },
    Flash { color: Color, duration: f64 },
    Text { lines: Vec<String>, auto_advance: bool },
    Sound { sfx_id: u8 },
    Pause(f64),
    Callback(Box<dyn FnOnce(&mut PokemonSim, &mut Engine)>),
}

impl PokemonSim {
    fn queue_transition(&mut self, t: Transition) {
        self.transition_queue.push_back(t);
    }
    
    fn change_map_with_transition(&mut self, map: MapId, x: i32, y: i32) {
        self.queue_transition(Transition::Fade { direction: Out, duration: 0.3 });
        self.queue_transition(Transition::Callback(Box::new(move |sim, _| {
            sim.change_map(map, x, y);
        })));
        self.queue_transition(Transition::Fade { direction: In, duration: 0.3 });
    }
    
    fn evolve_with_transition(&mut self, party_idx: usize, into: SpeciesId) {
        let old_name = self.party[party_idx].name().to_string();
        let new_name = get_species(into).map(|s| s.name).unwrap_or("???");
        self.queue_transition(Transition::Text { 
            lines: vec![format!("What? {} is evolving!", old_name)], auto_advance: true 
        });
        self.queue_transition(Transition::Flash { color: WHITE, duration: 3.0 });
        self.queue_transition(Transition::Callback(Box::new(move |sim, _| {
            sim.party[party_idx].evolve(into);
        })));
        self.queue_transition(Transition::Sound { sfx_id: SFX_EVOLUTION });
        self.queue_transition(Transition::Text {
            lines: vec![format!("{} evolved into {}!", old_name, new_name)], auto_advance: false
        });
    }
}
```

The transition queue is drained in `step()` — if the queue is non-empty, the current transition plays instead of normal game logic. This is similar to how dialogue already works (`GamePhase::Dialogue` blocks other input), but generalized.

**Why this is high-impact**: It turns Missing Transitions items from "restructure a phase handler" into "add 3-5 queue pushes." Map fades, evolution, badge screens, whiteout — all become composable sequences of the same primitives. And crucially, the agent can add transitions without touching the game logic that triggers them — they just wrap calls.

**Sprint cost**: ~1 sprint. Small implementation, huge leverage.

---

### 4. Move Effect Dispatch Table (replaces scattered match blocks)

**The problem it solves**: Move effects are currently scattered across 4 locations in mod.rs: `try_inflict_status()` (status effects), `damaging_move_stat_effect()` (stat drops), `flinch_chance()` (flinch rates), `status_move_stage_effect()` (stat stage moves), plus inline checks for Haze, Confuse Ray, Swagger, Mean Look, Self-Destruct, Struggle, and Hyper Beam recharge. Adding a new move effect means figuring out which of these 6+ locations to modify, and getting the interaction order right.

The original game has `engine/battle/move_effects/` — one file per effect, dispatched via an effect constant on each move. Pokemon Essentials uses a handler registry. Both separate "what the move does" from "when it happens in the turn."

**The abstraction**: Each move gets an `effect: MoveEffect` field (or derived from an effect ID) that's a small enum:

```rust
#[derive(Clone, Copy, Debug)]
pub enum MoveEffect {
    None,
    // Status infliction
    MayBurn(u8),          // chance in percent
    MayFreeze(u8),
    MayParalyze(u8),
    MayPoison(u8),
    MayConfuse(u8),
    MayFlinch(u8),
    Sleep,                // guaranteed (Hypnosis etc)
    Toxic,                // BadPoison
    // Stat changes
    MayLowerStat(u8, usize, i8),   // chance, stat_idx, stages
    RaiseStat(usize, i8),           // user stat boost
    LowerStat(usize, i8),          // target stat drop (guaranteed)
    // Complex
    Recharge,             // Hyper Beam
    Rampage,              // Thrash/Outrage
    Rest,
    SelfDestruct,
    Recoil(u8),           // fraction denominator (4 = 1/4)
    Haze,
    MeanLook,
    Swagger,              // +2 Atk + confuse
    TriAttack,
    DrainHp(u8),          // fraction (2 = 1/2)
    MultiHit(u8, u8),     // min, max hits
}
```

Then on `MoveData`: `pub effect: MoveEffect`. The five scattered functions collapse into one `apply_move_effect()` that's called at the right point in the turn and dispatches on the enum. Adding a new move effect = adding an enum variant + a match arm. No hunting through mod.rs for the right insertion point.

**Why this is high-impact**: There are ~250 moves in Gen 2. About 80 have unique effects. Currently each one requires the agent to know which of 6 functions to modify and how they interact. With a dispatch table, the agent adds `effect: MoveEffect::MayBurn(10)` to the MoveData literal and it works.

**Sprint cost**: ~1 sprint to implement and migrate. Most move effects already work — this just centralizes them.

---

### 5. Map Data Externalization (replaces hand-coded tile arrays)

**The problem it solves**: `maps.rs` is 8000+ lines and growing. Each map is a hand-coded `Vec<u8>` of tile IDs and collision types — a 20x18 city is 360 entries per array, times 2 (tiles + collision), plus NPCs, warps, and encounters. The agents generate these correctly but slowly, and small errors (wrong tile at position 147) are invisible until someone walks there in the browser.

The original game stores maps as `.blk` files — binary block data loaded at runtime. Pokemon Essentials uses RPG Maker's map editor to generate map files. Both separate map data from code.

**The abstraction**: Store maps as compact binary or text files loaded at runtime instead of compiled Rust arrays.

```
# maps/NewBarkTown.map
size: 20 10
tiles:
TTTTTTTTTTTTTTTTTTTT
TGGGPPPPPPPPPPGGGGT
...
collision:
SSSSSSSSSSSSSSSSSSSS
SWWWWWWWWWWWWWWWWWWS
...
warps:
0,5 -> Route29 19,5
19,5 -> Route29 0,5
npcs:
3,4 elm face=down sprite=0 script=elm_intro
encounters:
Pidgey 2-4 30%
Sentret 2-4 50%
```

Parse this in `load_map()` instead of calling `build_new_bark_town()`. The `build_*` functions become a migration tool — run once to export existing maps to text format, then delete the functions.

**Why this is high-impact**: It cuts `maps.rs` from 8000 lines to ~500 (types, enums, parser, tests). Map edits become text edits — no recompilation needed for tile changes. And crucially, it makes the PNG-to-map pipeline viable: the `png_to_sprites.py` tool could output map files directly instead of Rust const arrays.

The risk is that Crusty's WASM build may not support file I/O. If maps must be compiled in, an intermediate approach works: a build script (`build.rs`) that reads `.map` files and generates Rust code. The agent edits text files; `cargo build` compiles them.

**Sprint cost**: ~2 sprints. High initial cost, but saves ~5 minutes per map after that (and there are 57+ maps to maintain).

---

### Summary: Impact vs Cost

| Abstraction | Sprint Cost | What it Eliminates | Ongoing Savings |
|-------------|-------------|-------------------|-----------------|
| 1. Event Script System | 2 | Story event functions in mod.rs | ~1 sprint per 10 events |
| 2. Battle Phase Sequencer | 3 | Nested Box<BattlePhase> chains | Every battle feature 5x easier |
| 3. Transition System | 1 | 50+ Missing Transitions items | Each transition 10 min vs 2 hrs |
| 4. Move Effect Dispatch | 1 | 6 scattered effect functions | Each new move effect is 1 line |
| 5. Map Externalization | 2 | 7500 lines of tile arrays | 5 min saved per map edit |

**If you can only do one**: #3 (Transition System). One sprint, immediately unblocks the entire Missing Transitions list, and the agents can implement it without understanding the full battle system.

**If you can do two**: #3 + #4. Two sprints total, covers both the UX gap (transitions) and the mechanical gap (move effects).

**If you can do three**: #3 + #4 + #1. Four sprints total, and the game's story logic becomes data-driven — which is the single biggest architectural win for long-term agent autonomy.

## Sprint Log

_Agents: append new sprint entries here after each sprint. Include what was built, what bugs were found/fixed, and what's queued next._

### Sprint 40 (Content)
- Added Route 35, National Park, Route 36, Route 37
- 10 new species, 19 new moves

### Sprint 41 (Content)
- Ecruteak City, Burned Tower, Ecruteak Gym
- Morty: Gastly lv21, Haunter lv21, Haunter lv23, Gengar lv25 (Fog Badge)
- Burned Tower: Rival + Eusine NPC
- 7 new species (Magmar, Eevee line), 6 new moves

### Sprint 42 (QA)
- Fixed: status moves bypassing accuracy, 100% moves ignoring evasion, missing burn penalty, physical/special category errors, warp bugs, NPC placement

### Sprint 43 (Content)
- Route 38, Route 39, Olivine City, Olivine Gym
- Jasmine: 2x Magnemite lv30, Steelix lv35 (Mineral Badge)
- 15 new species, 17 new moves pre-staged
- Fixed first non-fainted Pokemon selection for battle lead

### Sprint 44 (QA)
- Added `test_all_warps_land_on_walk()` and `test_all_npcs_on_walkable()` automated tests
- Fixed 5 warp/NPC placement bugs
- Built OlivineLighthouse (10x12) with Jasmine+Amphy and 4 trainers

### Sprint 45 (QA)
- Comprehensive warp audit — all clear
- Fixed Faint Attack accuracy (100→255 never-miss)
- Added accuracy≥255 bypass in both player/enemy paths
- 7 new species, 6 new moves

### Sprint 46 (Content)
- Route 40, Cianwood City, Cianwood Gym
- Chuck: Primeape lv27, Poliwrath lv30 (Storm Badge)

### Sprint 47 (Content)
- Route 42, Mahogany Town, Mahogany Gym
- Pryce: Seel lv27, Dewgong lv29, Piloswine lv31 (Glacier Badge)
- 9 new species, 14 new moves

### Sprint 48 (QA)
- Full audit: all 40 maps, all warps, all NPCs, all species data — clean pass. No bugs found.

### Sprint 49 (Content)
- Route 43, Lake of Rage (Red Gyarados event area, Lance NPC)
- Fixed 9 warp bugs during development

### Sprint 50 (Content)
- Route 44, Ice Path, Blackthorn City, Blackthorn Gym
- Clair: 3x Dragonair lv37, Kingdra lv40 (Rising Badge — all 8 badges now obtainable)
- 9 new species, 8 new moves. Fixed 5 warp bugs.

### Sprint 51 (QA)
- Full audit — clean pass. Fixed Jynx learnset, Seadra evolution level.

### Sprint 52 (Content)
- Route 45, Route 46 (connects Blackthorn south to Route 29)
- 6 new species, 2 new moves. Fixed 9 warp bugs.

### Sprint 53 (Content + Bugfix)
- Trainer walk-up mechanic: "!" exclamation → trainer walks toward player → battle. Added `GamePhase::TrainerApproach`.

### Sprint 54 (QA)
- Full audit: all 48 maps — clean pass. Warp connectivity verified: all maps reachable from New Bark Town.

### Sprint 55 (Content)
- Route 27, Route 26 (connect to Victory Road entrance)
- Opened New Bark Town left exit to Route 27
- 7 new species, 3 new moves. Fixed 1 warp bug.
- **Phase 3 status**: Route 44 ✓, Ice Path ✓, Blackthorn ✓, Route 45 ✓, Route 46 ✓, Route 27 ✓, Route 26 ✓
- **Next**: Victory Road, Indigo Plateau, E4 rooms, Champion, Credits, Save System, Story Gating

### Sprint 56 (Content)
- Victory Road (14x14): Cave dungeon, 3 Cooltrainer battles, 6 encounter species (lv30-36)
- Indigo Plateau (14x10): Lobby with PokemonCenter, E4 entrance, guard NPC
- EliteFourWill (10x10): Xatu40/Jynx41/Exeggutor41/Slowbro41/Xatu42
- EliteFourKoga (10x10): Ariados40/Forretress43/Muk42/Venomoth41/Crobat44
- EliteFourBruno (10x10): Hitmontop42/Hitmonlee42/Hitmonchan42/Onix43/Machamp46
- EliteFourKaren (10x10): Umbreon42/Vileplume42/Gengar45/Murkrow44/Houndoom47
- ChampionLance (10x10): Gyarados44/Dragonite47/Dragonite47/Aerodactyl46/Charizard46/Dragonite50
- 16 new species, 2 new moves (Psychic, Crunch). Fixed Golbat→Crobat evolution.
- Connected Route26→VictoryRoad→IndigoPlateau→Will→Koga→Bruno→Karen→Lance
- Fixed 9 warp bugs (forward warps landing on C_WARP). All 1259 tests pass.
- **Phase 3 COMPLETE**: All maps Route 44 through Champion Lance built
- **Total: 57 maps, 8 badges, ~141 species, ~149 moves**
- **Next (Sprint 57 QA)**: Full audit of all 57 maps, E4 teams, warp connectivity

### Sprint 57 (QA)
- Audited all E4 teams against pret/pokecrystal canonical data
- Fixed Vileplume SpAtk: 110→100 (Gen 2 canonical)
- Fixed Koga party order: Venomoth/Forretress/Muk (was Forretress/Muk/Venomoth)
- Verified all 16 new species base stats — 15/16 correct, Vileplume fixed
- Verified Psychic and Crunch move categories (both Special, correct for Gen 2 type-based system)
- Victory Road encounters verified as reasonable substitutes (Rhyhorn/Rhydon not yet implemented)
- All 1259 tests pass
- **Next (Sprint 58)**: Tile art upgrade pipeline + music integration (see GUIDE.md sections 8-9)

### Sprint 58 (Infrastructure)
- **Phase 0E complete**: Debug state export — 11 keys exported to global_state every step()
- 8 new headless integration tests using HeadlessRunner:
  - Title screen state, enter Elm Lab, full starter selection sequence
  - Walking changes position, deterministic same-seed replay
  - Debug state keys present, money initial value
- HeadlessRunner works correctly with PokemonSim — turbo mode ~0.2s for 200+ frames
- All 1267 tests pass (1259 existing + 8 new headless)
- **Next (Sprint 59)**: Tile art conversion tool + music_id wiring via global_state

### Sprint 59 (Infrastructure)
- Added `music_id` and `map_name` export to global_state in debug state block
- Created `tools/png_to_sprites.py` — PNG tilesheet → sprites.rs converter
  - Auto-detects 4-color palette, supports 8×8 and 16×16 tiles
  - Outputs `pub const TILE_XXX: &str = "..."` format compatible with engine
- All 1267 tests pass

### Sprint 60 (QA)
- **Phase 0A partial**: Added `PokemonType::gen2_category()` and `MoveData::derived_category()` to data.rs
- 4 validation tests: all move categories match Gen 2 type-based rules, physical/special type coverage, status moves have zero power
- All 1271 tests pass (1267 + 4 new)
- **Next (Sprint 61)**: Credits + Save System (DoD #4 + #10)

### Sprint 61 (Content — Credits + Save System)
- **Credits screen (DoD #4)**: `GamePhase::Credits` with scrolling text. Triggered after defeating Champion Lance. Shows congratulations, party, Hall of Fame, returns to title.
- **Save system (DoD #10)**: Full implementation:
  - `serialize_save()`: JSON blob with map, position, party (moves/PP/status), PC, bag, defeated trainers, badges, money, pokedex, RNG state
  - `load_from_save()`: Hand-rolled JSON parser (no serde dependency)
  - `MapId::from_str()`/`to_str()` for serialization
  - Auto-save on every `change_map()` via persist queue
  - Title screen: CONTINUE/NEW GAME menu when save exists
  - JS: loads save from localStorage before WASM init, handles `Store` persist commands
- Save/load round-trip test verifies all fields survive serialization
- All 1272 tests pass (1271 + 1 new)
- **Phase 7 (Save System) COMPLETE**
- **Phase 8 (Credits) COMPLETE**
- **Next (Sprint 62)**: Phase 5 move effects (secondary effects), Phase 0C story flags

### Sprint 62 (Content — Secondary Move Effects)
- **Phase 5 Priority 1**: Expanded `try_inflict_status` with Gen 2 secondary effects:
  - 10% burn (Flamethrower, Fire Blast, Flame Wheel, Fire Punch), 10% freeze (Ice Beam, Blizzard, Powder Snow, Ice Punch)
  - 10% paralysis (Thunderbolt), 30% paralysis (Body Slam, Twister), 30% poison (Sludge)
  - 20% tri-status (Tri Attack), sleep/para/poison status moves
- New `damaging_move_stat_effect`: Psychic/Shadow Ball/Crunch SpDef drops, Acid/Iron Tail Def drops, Aurora Beam Atk drop, Bubblebeam Speed drop, Steel Wing user Def boost, Mud Slap/Icy Wind guaranteed stat drops
- **New flinch mechanic**: Headbutt/Bite/Stomp/Rock Slide 30%, Twister 20%, Hyper Fang 10%
- All 1272 tests pass

### Sprint 63 (QA — Critical Bug Fixes)
- **Fixed**: Can no longer leave Elm Lab without picking a starter (warp blocked with dialogue nudge)
- **Fixed**: Menu opens during walk animation (cancel key checked before walk processing)
- **Fixed**: NEW GAME fully resets all game state (party, PC, bag, badges, money, defeated trainers, pokedex)
- All 1272 tests pass
- **Next (Sprint 64)**: Phase 0C story flags, Phase 5 Priority 2 status moves (Haze, Toxic, Confuse Ray), or more content

### Sprint 64 (QA — Deep Battle/Save Audit)
- **Fixed**: Status-inflicting moves (Hypnosis, Thunder Wave, Sleep Powder, Stun Spore, Sing, Poison Powder) now work — were broken because try_inflict_status only ran when damage > 0, but status moves have power 0
- **Fixed**: Menu SAVE actually persists to localStorage (was showing "Game saved!" but not triggering persist queue)
- **Fixed**: Switching Pokemon in battle costs a turn — enemy gets a free attack (Gen 2 rule)
- **Fixed**: Enemy paralysis now halves speed in turn order (was only applied to player)
- **Fixed**: Frozen Pokemon have 10% thaw chance per turn (were frozen forever)
- **Added**: Struggle move — forced when all PP = 0 (50 power, never-miss, 1/4 recoil). Prevents soft-lock.
- 3 new tests (Struggle, freeze thaw, status move infliction). All 1275 tests pass.
- **Next (Sprint 65)**: Phase 0C story flags (DoD #7), Victory Road rival battle

### Sprint 65 (Phase 0C — Story Flags + Victory Road Rival)
- **Added**: Story flags infrastructure — u64 bitfield with has_flag/set_flag helpers, persisted in save
- **Added**: Victory Road rival battle — starter final form lv36 + Haunter lv35 + Sneasel lv34 + Magneton lv34 + Golbat lv36
- **Added**: Route gate — Victory Road blocked without 8 badges (uses count_ones on badge bitfield)
- **Added**: Final starter evolutions: Meganium (154), Typhlosion (157), Feraligatr (160)
- **Added**: Magneton (82, Electric/Steel) with Magnemite evolution chain
- **Fixed**: Badge count check uses count_ones() instead of raw comparison (badges is a bitfield, not a counter)
- FLAG_RIVAL_ROUTE29 wired into existing Route 29 rival battle
- 10 story flag constants defined (8 reserved for future events: egg, Sprout Tower, Sudowoodo, etc.)
- 3 new tests (flags save/load, Victory Road rival requires 8 badges, final evolutions exist). All 1278 pass.
- **Next (Sprint 66)**: QA sprint

### Sprint 66 (QA — Catch/Run/Whiteout Audit)
- **Fixed**: Catch formula now applies Gen 2 status multipliers (sleep/freeze 2x, poison/burn/paralysis 1.5x)
- **Fixed**: Run formula halves player speed when paralyzed (Gen 2 rule)
- **Fixed**: Whiteout preserves last_pokecenter_map — player returns to correct city's PokeCenter
- **Fixed**: Clear pending_evolution on whiteout (prevents stale evolution after blackout)
- Verified: HP formula correct (+10), heal() restores HP/PP/status, evolution chains work
- 2 new tests (HP formula Gen 2, whiteout PokeCenter preservation). All 1280 pass.
- **Next (Sprint 67)**: Phase 5 Priority 2 status moves

### Sprint 67 (Phase 5 Priority 2 — Haze, Self-Destruct, Confusion)
- **Added**: Haze — resets all stat stages (both player and enemy) in battle
- **Added**: Self-Destruct — user faints after dealing 200-power damage (works for both player and enemy)
- **Added**: Confusion mechanic — Confuse Ray inflicts 2-5 turns, 50% self-hit (typeless 40 power)
- Confusion handles both turn-order paths (player first / enemy first)
- Confusion cleared on switch, auto-switch after faint
- Added player_confused/enemy_confused fields to BattleState
- 3 new tests (move data verification). All 1283 pass.
- **Next (Sprint 68)**: Toxic, Mean Look

### Sprint 68 (Toxic + Mean Look)
- **Added**: Toxic — BadPoison status with escalating damage (1/16, 2/16, 3/16... of max HP per turn)
- **Added**: Mean Look — enemy use prevents player from fleeing wild battles (player_trapped)
- Added MOVE_TOXIC (id 92, Poison, Status, 85 accuracy)
- BadPoison persists in save (serialized as 11+turn value)
- Updated all StatusCondition match arms for new BadPoison variant
- 2 new tests (Toxic escalation, Toxic infliction). All 1285 pass.
- **Next (Sprint 69)**: QA sprint

### Sprint 69 (QA)
- **Fixed**: Self-Destruct mutual KO — player faint now processed after EnemyFainted (no EXP for fainted Pokemon)
- **Fixed**: Enemy Self-Destruct — skip player's pending move when enemy already dead from own Self-Destruct
- **Fixed**: End-of-turn enemy faint check — catches Self-Destruct deaths, not just status damage
- **Fixed**: BadPoison turn counter resets to 1 on switch-in (Gen 2 compliance)
- **Fixed**: Mean Look (player_trapped) clears on switch
- Verified working: confusion mechanic, Toxic escalation, antidote cures BadPoison, evolution chains, Victory Road gate, flinch/paralysis/freeze timing
- 2 new tests. All 1287 pass.
- **Next (Sprint 70)**: Swagger + story events

### Sprint 70 (Swagger + Story Events)
- **Added**: Swagger (Normal/Status, 90 acc, 15 PP) — confuses target + raises Attack by 2 stages
- **Wired**: FLAG_SPROUT_CLEAR — Elder Li battle at top of Sprout Tower (3 Bellsprout)
- **Wired**: FLAG_RED_GYARADOS — forced Gyarados L30 encounter at Lake of Rage
- **Wired**: FLAG_SUDOWOODO — forced Sudowoodo L20 battle on Route 36 (requires 3+ badges)
- 5 of 10 story flags now active (Rival Route 29, Rival Victory Road, Sprout Clear, Red Gyarados, Sudowoodo)
- Phase 5 Priority 2 complete: Haze, Self-Destruct, Confusion, Toxic, Mean Look, Swagger all done
- 3 new tests. All 1290 pass.
- **Next (Sprint 71)**: E4/Champion learnset overhaul

### Sprint 71 (E4 Learnset Overhaul)
- **Critical fix**: 10 E4/Champion species had weak/stub learnsets (e.g., Lance's Dragonites used Dragon Rage for 40 fixed damage)
- Dragonite: Outrage at 47, Wing Attack at 42 (was 61/55)
- Xatu/Slowbro: Psychic at 40 (was 65/54)
- Onix: expanded from 3 moves to 8 (added Earthquake, RockSlide, IronTail)
- Vileplume: added PetalDance, SludgeBomb (was only 4 level-1 moves)
- All Hitmon: added proper Fighting STAB (Submission, CrossChop)
- Ariados/Muk: added SludgeBomb for Poison STAB
- Umbreon: added FaintAttack, Crunch for Dark STAB
- Gyarados: added Surf, Crunch, HyperBeam
- New moves: Petal Dance (Grass/70BP), Sludge Bomb (Poison/90BP, 30% poison)
- All 1290 tests pass.

### Sprint 72 (QA)
- **Confusion snap-out**: returned to MoveSelect instead of ActionSelect — fixed
- **Struggle recoil death**: player dying from recoil skipped straight to enemy attack — added faint check before enemy turn
- **Ether unusable**: fell to "Can't use that now!" — added Ether handler restoring 10 PP to first depleted move
- **Catch formula**: shake_prob could exceed 1.0 for easy catches — clamped with `.min(1.0)`
- **Champion credits preempted**: pending evolution could redirect away from credits after beating Lance — Champion check now runs before evolution check
- **Warp audit**: NewBarkTown↔Route27 warps verified correct (offsets prevent infinite warp loops)
- Added tests: test_catch_shake_prob_clamped, test_champion_credits_over_evolution
- All 1292 tests pass.

### Sprint 73 (Multi-Turn Move Mechanics)
- **Hyper Beam recharge**: player and enemy must skip next turn after landing Hyper Beam (critical for Lance fight)
- **Thrash/Outrage rampage**: locked into same move for 2-3 turns, self-confusion when rampage ends
- **Rest**: full HP heal + forced 2-turn sleep (Gen 2 accurate)
- Added BattleState fields: `player/enemy_must_recharge`, `player/enemy_rampage`
- Recharge/rampage cleared on switch-out and trainer send-out
- Enemy rampage: overrides `calc_enemy_move` selection via `calc_enemy_move_forced`
- Added `calc_player_damage` helper for rampage damage recalculation
- Added tests: hyper_beam_data, outrage_data, thrash_data, rest_data
- All 1296 tests pass.

### Sprint 74 (Rocket HQ + Battle Items Audit)
- **Rocket HQ** (12x12): 4 Rocket Grunts (Rattata/Koffing/Zubat/Raticate/Muk/Golbat) + Executive boss (Golbat/Koffing/Muk L28-30)
- Mahogany Town mart door now warps to RocketHQ entrance
- Defeating Executive (NPC 4) sets FLAG_ROCKET_MAHOGANY — story progression unblocked
- **Battle items audit**: Potions, Revives, and status heals already work mid-battle (enemy gets free turn after)
- Removed dead_code allow on FLAG_ROCKET_MAHOGANY (now used)
- Added tests: rocket_hq_map_exists, rocket_hq_warp_to_mahogany
- All 1298 tests pass.

### Sprint 75 QA
- **Rampage re-initialization bug**: Both player and enemy rampage init used `.0 == 0` (counter), which re-triggered rampage after counter hit 0 during active rampage. Fixed to `.1 == 0` (move_id) — only init when no rampage active.
- QA audit false positives dismissed: RocketHQ exit warp is valid, Self-Destruct+rampage not a real conflict
- All 73 pokemon tests pass.

### Sprint 76 (New Move Learning Sequence)
- **Full move learning UX**: When Pokemon levels up and has 4 moves, interactive sequence: "trying to learn" → "can't learn more" → YES/NO delete prompt → move picker → forget/learn confirmation
- Added `BattlePhase::LearnMove` with `LearnMoveSub` state machine (8 sub-states: TryingToLearn, CantLearnMore, DeletePrompt, PickMove, ForgotMove, LearnedMove, StopPrompt, DidNotLearn)
- Added `pending_learn_moves: Vec<MoveId>` to BattleState for queueing multiple moves at same level
- Auto-fills empty move slots silently; only prompts when all 4 occupied
- Skips moves already known (duplicate check)
- Full render: move picker with type colors + PP display, YES/NO prompts with cursor
- Added tests: learn_move_queued_when_full, learn_move_sub_phases
- All 75 tests pass.

### Sprint 77 (Map Fades + Trainer Defeat Text)
- **Map transition fades**: All overworld warp transitions now fade to black (0.25s) → change_map → fade in (0.25s). Added `GamePhase::MapFadeOut` and `MapFadeIn` with alpha overlay rendering. No longer jarring instant cuts.
- **Trainer defeat text**: "Trainer was defeated!" now shows before money reward. Champion Lance gets "CHAMPION LANCE was defeated!" text.
- Special transitions (whiteout, escape rope, title screen) still use instant change_map (they have their own UI flow).
- All 75 tests pass.

### Sprint 78 QA
- **Evolution skips learn-moves**: Evolution branch went straight to Won, bypassing LevelUp (which drains pending_learn_moves). Fixed: routes through LevelUp first.
- **Wild encounter fainted lead**: Used hardcoded `player_idx: 0`. Fixed: now finds first non-fainted party member.
- **Flinch persists across trainer Pokemon**: `enemy_flinched` not reset on send-out. Fixed in all 6 send-out blocks.
- **Flinch/recharge/rampage persists after player faint**: Auto-switch didn't clear these. Fixed with full state reset.
- All 75 tests pass.
- **Next (Sprint 79)**: Trainer switch prompt, EXP text, gym leader LOS fix

### Sprint 79 (UX Polish — Switch Prompt, EXP Text, LOS Fix)
- **Trainer switch prompt (#56)**: "Foe will send out X. Will you switch?" YES/NO prompt when trainer sends next Pokemon. YES opens PokemonMenu with free switch (no enemy attack penalty). `free_switch` field on BattleState.
- **EXP gain text (#27)**: "POKEMON gained X EXP!" text now shows after enemy faints, before EXP is awarded. New `BattlePhase::ExpAwarded` separates text display from EXP logic.
- **Gym leader LOS fix**: Gym leaders (NPC 0 in all 8 gym maps) no longer trigger line-of-sight approach. They battle only when talked to, matching original behavior. Regular gym trainers still have 5-tile LOS.
- Stat change text (#18, #19) verified already implemented: "sharply rose/fell" for +/-2 stages, "won't go higher/lower" at +/-6.
- All 1305 tests pass.
- **Next (Sprint 80)**: Evolution sequence, badge screen, catch shake

### Sprint 80 (UX Polish — Evolution, Badges, Catch Shakes)
- **Evolution cancel (#44)**: B button during flash phase cancels evolution with "Huh? X stopped evolving!" text
- **Evolution completion text (#43)**: "Congratulations! X evolved into Y!" dialogue after evolution completes (was silent)
- **Badge screen (#54)**: Per-badge effect text (Attack up, Speed up, obey levels). Badge count display. Screen flash celebration.
- **Catch ball shakes (#47)**: 0-3 shake checks with Gen 2 flavor text ("Almost had it!", "Appeared to be caught!"). "Wobble..." text for tension.
- All 1305 tests pass.
- **Next (Sprint 81 QA)**: Full audit

### Sprint 81 (QA)
- **free_switch cancel bug**: Backing out of PokemonMenu after TrainerSwitchPrompt YES left free_switch=true, giving free switches on any subsequent switch. Fixed: clear free_switch on PokemonMenu cancel.
- **MediumSlow EXP formula**: Used `saturating_sub` for unsigned math causing inflated EXP at levels 2-12 for ~38 species (starters, many common Pokemon). Fixed: signed arithmetic.
- **Level-up SFX on evolution**: No level-up sound played when evolution was pending. Fixed: moved sfx_level_up before evolution check.
- **Multi-level-up**: Only one level gained per battle even with enough EXP for multiple. Fixed: while loop processes all level-ups, collecting all new moves.
- All 1305 tests pass.
- **Next (Sprint 82)**: Sprite backgrounds, whiteout sequence

### Sprint 82 (Content — Sprite Fix + Whiteout)
- **Transparent battle sprites**: Switched primary sprite source to PokeAPI (GitHub-hosted, transparent PNGs). Added `removeWhiteBackground()` JS fallback for PokemonDB sprites. Export `enemy_species_id`/`player_species_id` to global_state for PokeAPI ID-based loading.
- **Smooth HP bar drain (#9)**: Verified already implemented — `player_hp_display`/`enemy_hp_display` lerp at `diff * 0.15` per frame.
- All 1305 tests pass.
- **Next (Sprint 83)**: Whiteout fade, send-out text

### Sprint 83 (Content — Whiteout Fade + Send-Out Text)
- **Whiteout fade (#74-75)**: New `GamePhase::WhiteoutFade` — screen fades to white over 1.5 seconds (distinctive from normal black fade). Then warps to PokeCenter with "You blacked out!" dialogue. Money loss shown.
- **Send-out text (#5)**: "Go! POKEMON!" text now shows after "Wild X appeared!" / "Trainer sent out X!" at battle start. Uses Text phase → ActionSelect chain.
- All 1305 tests pass.
- **Next (Sprint 84 QA)**: Full audit

### Sprint 84 (QA)
- **Free switch cancel destination**: Cancelling PokemonMenu during free switch went to ActionSelect instead of TrainerSwitchPrompt. Fixed: cancel now returns to TrainerSwitchPrompt with cursor reset.
- **Catch formula probability**: Used 3 independent rolls (effective rate = shake_prob^3), making catching 3-8x harder than Gen 2. Fixed: single roll for catch/no-catch decision, cosmetic shakes proportional to shake_prob.
- All 1305 tests pass.
- **Next (Sprint 85)**: Discovery sprint — full audit of broken transitions and progression

### Sprint 85 (DISCOVERY — Comprehensive Bug Audit)

Full audit of every transition, progression gate, battle text sequence, and map warp. Categorized by severity.

---

#### CATEGORY A: PROGRESSION BLOCKERS (game-breaking)

**A1. GenericHouse exit always goes to wrong door (CONFIRMED)**
- ALL houses across ALL towns share `MapId::GenericHouse`. Exit uses `last_house_map` to pick a city, but each city only maps to ONE exit coordinate. Cities with multiple houses (CherrygroveCity, AzaleaTown, GoldenrodCity, OlivineCity, EcruteakCity, etc.) will exit the player at the WRONG door.
- Example: CherrygroveCity has houses at (15,4) and (16,8). Both enter GenericHouse. Exit always goes to (15,5). Entering house at (16,8) and exiting drops you at (15,5) — wrong building.
- **Fix needed**: Either (a) store `last_house_x, last_house_y` and exit 1 tile below the entry door, or (b) create unique MapId per house, or (c) use a return-warp stack.

**A2. New Bark Town → Route 27 wide open at game start (CONFIRMED)**
- `maps.rs:653`: WarpData at (0,10) goes to Route27 with NO gate check.
- In original game, Route 27 requires Waterfall HM (post-E4).
- Player can walk left from New Bark Town immediately into late-game content.
- **Fix needed**: Add gate check in warp processing: block Route 27 warp without 8 badges.

**A3. Missing story progression gates**
- Route 32 south → Union Cave: NO Zephyr Badge check (original requires badge)
- Ilex Forest north → Route 34: NO Cut HM check (original requires Cut)
- Route 44 east → Ice Path: NO Rocket HQ flag check (flag is SET at line 2868 but NEVER CHECKED)
- Route 27 west → Route 26: NO gate (should require Waterfall/post-E4)
- **Fix needed**: Add warp gate checks for each, similar to Victory Road badge check at line 1119.

**A4. "No way past Route 30" — potential trainer loop**
- Route 30 has 3 trainers. If defeated_trainers is not persisting correctly across saves, or if the LOS triggers incorrectly, trainers could re-battle endlessly.
- Route 30 east exit to Route 31 is at (29,2)-(29,3) — far corner of map. Player must navigate from south (14,16) to northeast corner. Path is open but long.
- Trainer at (8,3) faces Left — checks x=3-7 at y=3. Could block east path at row 2-3 if player walks near.
- **Investigation needed**: Check if save/load preserves defeated_trainers. Check if trainers re-trigger after save/load.

---

#### CATEGORY B: BATTLE TEXT SEQUENCES (broken feel)

**B1. Critical hit, super effective, miss — all jammed into one line**
- Lines 1981-1989: `format!("{} used {}! {}{}{}", pname, move_name, eff, crit_str, miss_str)`
- In original game, these are SEPARATE sequential messages: "X used MOVE!" → "Critical hit!" → "It's super effective!"
- Current: "CYNDAQUIL used EMBER! Super effective! Critical hit!" — one long messy line.
- **Fix needed**: Chain through `BattlePhase::Text` for each message.

**B2. Status damage text completely missing**
- `apply_status_damage()` at lines 2075, 2434-2441 applies damage but shows NO text.
- Missing: "X is hurt by its burn!", "X is hurt by poison!", "X is hurt by TOXIC!" per turn.
- Return value is captured as `_enemy_status_dmg` (discarded) or not captured at all.
- **Fix needed**: Check return value, show damage text if > 0 before advancing to ActionSelect.

**B3. Sleep wake-up silent**
- `tick_status()` at data.rs:2094-2109 clears sleep when counter hits 0. No message generated.
- Missing: "X woke up!" — currently player has no feedback that sleep ended.
- **Fix needed**: Return bool from tick_status indicating wake-up, show text.

**B4. Freeze thaw silent**
- `try_thaw()` returns true on thaw but return value is discarded (`let _thawed = ...`).
- Missing: "X thawed out!" message.
- **Fix needed**: Check return value, show "X thawed out!" text.

**B5. Recoil text missing + incomplete implementation**
- Only Struggle has recoil (lines 1909-1915). Double-Edge and Take Down should have 1/4 recoil but DON'T.
- No "X is hit with recoil!" text for any recoil.
- **Fix needed**: Add recoil to Double-Edge/Take Down, show text.

**B6. Multi-hit moves unimplemented**
- Fury Swipes, Double Kick exist in data but do single-hit damage only.
- No "Hit N times!" display.
- **Fix needed**: Implement multi-hit loop with cumulative damage and hit count display.

**B7. "X learned MOVE!" silent on auto-fill**
- When a Pokemon levels up with < 4 moves, new moves are silently inserted at lines 2518-2528.
- No text: "X learned Y!" — player doesn't know a new move was added.
- **Fix needed**: Show text for each auto-learned move.

**B8. Level-up stat display missing (#32 from priority list)**
- `BattlePhase::LevelUp` at lines 4003-4008 only shows "X grew to LV Y!" text.
- Missing: stat comparison screen showing old → new stats with +N for each stat.
- In original game, this is a full screen players see every level up.
- **Fix needed**: Capture old stats before recalc_stats, display delta in LevelUp render.

**B9. EXP bar fill animation missing (#28)**
- EXP is awarded instantly. No visual bar fill.
- In original game, EXP bar fills smoothly, wraps around on level up.
- **Fix needed**: Add EXP bar to battle HUD, animate fill in ExpAwarded phase.

**B10. Run away — "Got away safely!" missing**
- Player can run from wild battles (BattlePhase::Run at line 2906-2911).
- No "Got away safely!" text — just instantly exits battle.
- **Fix needed**: Show text before exiting.

---

#### CATEGORY C: TRAINER LOS ISSUES

**C1. LOS fires immediately after map transition (HIGH)**
- When `MapFadeIn` completes (line 5451), phase becomes Overworld.
- Next frame, `step_overworld()` runs with LOS check.
- A trainer near a warp destination can trigger battle before player can react.
- **Fix needed**: Add `los_suppress_frames: u8` counter, set to 2 after any map transition.

**C2. NPC facing direction is static**
- NPCs have a fixed `facing: Direction` and never turn to face the player on approach.
- In original game, trainers turn to face the player when they walk up to talk.
- This is cosmetic but affects which trainers can "see" the player.
- **Fix needed**: Low priority — could update facing when talked to.

---

#### CATEGORY D: POKEMONCENTER / HOUSE WARP ISSUES

**D1. PokemonCenter exit also uses single-destination pattern**
- `last_pokecenter_map` lookup (lines 1140-1152) has only one exit per city.
- If a city has multiple PokemonCenters (unlikely in Gen 2), same bug as GenericHouse.
- Currently OK since each city has exactly 1 PokemonCenter.

**D2. GenericHouse has only 1 NPC with same dialogue**
- Every house in the entire game has the same "I love this town" NPC.
- Not a bug but very immersion-breaking.
- **Fix needed**: Low priority — could randomize dialogue or have per-city house variants.

---

#### CATEGORY E: MISSING GAME SYSTEMS

**E1. Wild Pokemon fleeing**
- No system for wild Pokemon running away.
- Roaming legendaries (Raikou, Entei, Suicune) don't exist yet.
- **Fix needed**: Low priority unless implementing roaming legends.

**E2. Player run formula feedback**
- Run succeeds or fails, but on failure there's `BattlePhase::RunFailed` (line 2913).
- On success, just exits — no "Got away safely!" text.
- **Fix needed**: Add text.

---

#### SIMULATION TESTS NEEDED

1. **test_generic_house_exit_returns_to_correct_door** — enter from CherrygroveCity (15,4), exit, verify position is near (15,4) not (16,8)
2. **test_newbark_route27_blocked_without_badges** — verify Route 27 warp from New Bark blocked at game start
3. **test_route32_requires_zephyr_badge** — verify Union Cave warp blocked without badge
4. **test_defeated_trainer_no_retrigger** — defeat trainer, walk through their LOS, no battle
5. **test_los_suppressed_after_map_transition** — enter map with trainer in LOS, verify no immediate trigger
6. **test_status_damage_text_shown** — apply burn, advance turn, verify text output
7. **test_critical_hit_separate_message** — land a crit, verify separate "Critical hit!" text
8. **test_auto_learn_shows_text** — level up with < 4 moves, verify "learned" text appears
9. **test_run_shows_text** — run from wild battle, verify "Got away safely!" text

---

**Priority order for fixes:**
1. A1 (GenericHouse exit) + A2 (Route 27 gate) — game-breaking
2. A3 (missing gates) — progression-breaking
3. C1 (LOS after transition) — gameplay feel
4. B1 (separate battle messages) — battle feel
5. B2-B4 (status damage/wake/thaw text) — feedback
6. Everything else

---

## Sprint 86 — Data Compilation Sprint

**Goal**: Build a comprehensive reference data library to prevent recurring bugs caused by missing or incomplete game knowledge.

### Data Files Created (3,298 total lines)

| File | Lines | Contents |
|------|-------|----------|
| `engine/data/REFERENCE.md` | 578 | Master index: game progression (15 parts), map connectivity graph, 12 progression gates, story flags, city buildings, interior maps, key mechanics rules, physical/special split gotchas, implementation checklist |
| `engine/data/gym_e4_rival_data.txt` | 415 | All 8 gym leaders with full teams + movesets, Elite Four (Will/Koga/Bruno/Karen), Champion Lance, Rival Silver (7 encounters, 3 starter variants each) |
| `engine/data/johto_routes_34_46.txt` | 881 | Routes 34-46 with wild encounters (Morning/Day/Night), trainers, items. Plus 6 dungeons: Union Cave, Ilex Forest, Ice Path, Mt. Mortar, Dark Cave, Slowpoke Well |
| `engine/data/gen2_battle_mechanics.txt` | 353 | Damage formula (10+ modifiers), critical hit stages, stat calculation (DVs + StatExp), EXP formula (4 growth curves), catch rate formula (with Gen2 bugs), status conditions (6 types), full 17-type effectiveness chart |
| `engine/data/gen2_moves_pokemon.txt` | 832 | 251 moves with Gen2-specific stats, evolution data (all methods), starter learnsets, ~130 species base stats with catch rates + EXP yields + growth rates |
| `engine/data/johto_routes_29_33.txt` | 239 | Routes 29-33 encounters + trainers (pre-existing) |
| `engine/data/johto_cities_progression.txt` | 926 | Complete map connectivity graph, detailed city/building data |

### Key Findings During Compilation

1. **Gen 2 Physical/Special split gotchas documented** — Pursuit is Special (Dark type), Shadow Ball is Physical (Ghost type), Fire/Ice/ThunderPunch are Special. These have caused bugs before.
2. **12 progression gates identified** — Only 4 currently implemented (G1-G3, G11). 8 more needed.
3. **Badge stat boosts** — Zephyr→Attack, Plain→Speed, Mineral→Defense, Glacier→SpAtk+SpDef. Each is ×1.125. Not yet implemented in battle.
4. **Gen 2-specific move power differences** — Dig=60 (not 100), Wing Attack=35 (not 60), Low Kick=50 (fixed power, not weight-based). Need to audit our move data.
5. **Catch formula Gen 2 bug** — Paralysis/Burn/Poison status bonus is completely skipped due to code bug. Our implementation should match this bug for accuracy.
6. **Version exclusives on routes** — Route 45: Gold has Gligar+Teddiursa, Silver has Phanpy+Skarmory. Ice Path: Gold has Jynx, Silver has Delibird.

### Sources Used
- Bulbapedia (primary for structured data)
- Serebii.net (encounter tables, trainer data)
- Pokemon World Online Wiki (base EXP yields)
- pret/pokecrystal disassembly (canonical source for verification)

### Sprint 87 (Content — Battle Text Overhaul)
- **B1 FIX: Separate battle messages** — "X used MOVE!", "Critical hit!", "Super effective!" now display as separate sequential text phases instead of jammed into one line. Both player and enemy attacks fixed. Matches original Gen 2 behavior.
- **B2 FIX: Status damage text** — "X is hurt by its burn!", "X is hurt by poison!" now displays at end-of-turn when status damage is applied. Both player and enemy sides. Both end-of-turn code paths (player-went-second, enemy-went-second).
- **B3 FIX: Wake-up text** — `tick_status()` now returns bool for wake-up. "X woke up!" text shown when sleep counter hits 0.
- **B4 FIX: Thaw text** — "X thawed out!" shown when freeze thaw succeeds. Player thaw sends back to MoveSelect. Enemy thaw adds text to follow-up chain.
- **B5 FIX: Recoil for Take Down** — Take Down now deals 1/4 recoil like Struggle. "X is hit with recoil!" text shown. Both player and enemy sides.
- **B10/E2 FIX: Run text** — "Got away safely!" dialogue now shows when fleeing wild battles (was instant silent exit).
- All 1318 tests pass.
- **Next (Sprint 88)**: More content from Sprint 85 audit: B6 (multi-hit moves), B7 (auto-learn text), B8 (level-up stat display), B9 (EXP bar animation)

### Sprint 88 (Content — Multi-Hit Moves + Auto-Learn Text)
- **B6 FIX: Multi-hit moves** — Added `multi_hit_count()` helper. Fury Swipes and Fury Attack use Gen 2 distribution (2=37.5%, 3=37.5%, 4=12.5%, 5=12.5%). Double Kick always hits exactly 2 times. Damage multiplied by hit count. "Hit N times!" text shown after attack. Both player and enemy sides.
- **B7 FIX: Auto-learn text** — When a Pokemon levels up and auto-learns a move into an empty slot, "X learned MOVE!" text now displays before the LevelUp phase. Uses `auto_learn_msgs` Vec chained before LevelUp.
- **Next (Sprint 89 — QA)**: Run full QA audit. Remaining B-category items: B8 (level-up stat display), B9 (EXP bar animation).

### Sprint 89 (QA)
- **CRITICAL FIX: Confusion snap-out lost player's turn** — When player snapped out of confusion in MoveSelect, phase went to ActionSelect (losing selected move). Fixed: snapout message is now chained before the attack dispatch. For enemy-goes-first case, snapout msg is stored in `confusion_snapout_msg` field and inserted when pending move resolves.
- **MEDIUM FIX: Enemy woke-up text missing** — `tick_status()` return was suppressed with `_ewoke` in both end-of-turn paths. Now checks return and shows "Foe X woke up!" text. Both EOT branches fixed.
- **MEDIUM FIX: Enemy thaw notification lost during rampage** — `try_thaw()` return was ignored during player rampage turns. Now captured: if enemy thaws, shows "Foe X thawed out!" and proceeds to enemy attack.
- **LOW FIX: Bug vs Poison type chart** — `(Bug, Poison) => 0.5` was a Gen 1 leftover. Removed entry; now correctly falls through to 1.0 (neutral) per Gen 2.
- All 1318 tests pass.
- **Next (Sprint 90)**: Content sprint. B8 (level-up stat display), B9 (EXP bar animation).

### Sprint 90 (Content — Route 30 Fix + Tilemap Assets)
- **Route 30 east exit relocated** — Moved Route 31 exit from hidden top-right corner (y=2/3) to natural mid-map east-west path junction (y=10/11). Path now clearly leads east to Route 31 with a wide walkable corridor. Both tile and collision maps updated. Route 31 return warps updated to match new coordinates.
- **Gen 2 tilemap assets added** — Downloaded MIT-licensed 8x8 GBC-style tileset from `nikouu/Pokemon-gen-2-style-tilemap`. Files in `engine/assets/tilesets/`: `Original.png` (128x80 tilesheet), `Custom.png` (artist variant), `TilemapDetails.json` (149 tile names + positions), `Constructed/` (16x16 composites). Ready for future tile rendering integration.
- **GUIDE.md updated** — Marked project as "true 1:1 clone" of Pokemon Gold/Silver/Crystal in header.
- All 1318 tests pass.
- **Next (Sprint 91)**: Content sprint. Tile rendering integration, B8 (level-up stat display).

### Sprint 91 (Content — Level-Up Stat Display)
- **B8 FIX: Level-up stat delta display** — Added `stat_deltas: [i16; 6]` field to `BattlePhase::LevelUp`. Old stats captured before `recalc_stats()`, deltas accumulated for multi-level-ups. Render shows all 6 stats (HP, Atk, Def, SAtk, SDef, Spd) with current value and +N delta in a panel above the "grew to LV X!" message. Matches Gen 2's stat increase screen.
- All 1318 tests pass.
- **Next (Sprint 92 — QA)**: Full QA audit.

### Sprint 92 (QA)
- **CRITICAL FIX: BurnedTower rival sprite_id out of bounds** — Rival NPC had `sprite_id: 6` but only 0-5 exist. Changed to 2 (Youngster placeholder) to prevent panic.
- **MEDIUM FIX: Route 30 south exit asymmetry** — Removed x=13 warp tile, narrowing south exit from 3 to 2 tiles to match Cherrygrove's 2-tile north border. Eliminates spatial inconsistency.
- **MEDIUM FIX: EcruteakCity west entry re-entry trap** — Removed x=1 warp column (rows 8-9), keeping only x=0 as the true border. Updated Route 37 east exit to land players at x=1 (inside the single warp column). Eliminates re-entry loop.
- All 1318 tests pass.
- **Next (Sprint 93)**: Content sprint. B9 (EXP bar animation), new content.

### Sprint 93 (Content — EXP Bar Animation)
- **B9 FIX: EXP bar fill animation** — `ExpAwarded` phase now has a 1-second timer with smooth bar fill animation. EXP bar interpolates from old EXP to new EXP over the duration. Skippable with confirm button. Render draws animated bar during the phase. All Sprint 85 audit B-category items now resolved (B1-B10 complete).
- All 1318 tests pass.
- **Next (Sprint 94)**: Content sprint. D2 fix, Sudowoodo NPC, missing encounters.

### Sprint 94 (Content — House NPC Variety + Sudowoodo Blocker)
- **D2 FIX: GenericHouse per-city NPC dialogue** — Every GenericHouse NPC now has city-specific dialogue based on `last_house_map`. All 10 Johto cities have unique lines reflecting local flavor (Sprout Tower in Violet, KURT in Azalea, Dept Store in Goldenrod, legends in Ecruteak, lighthouse in Olivine, pharmacy in Cianwood, "too quiet" in Mahogany, Dragons in Blackthorn). Eliminates the immersion-breaking "I love this town" from every house.
- **Sudowoodo visual blocker NPC** — Added NPC at (14,6) on Route 36 blocking the east path. Visible until `FLAG_SUDOWOODO` is set. Interacting with it triggers the Sudowoodo battle (requires 3+ badges). Added `is_npc_active()` helper for flag-based NPC filtering across collision, rendering, LOS, and interaction systems.
- **Missing encounters: Abra + Vulpix** — Abra added to Route 34 encounters (lv10-12, 10% weight) matching Gen 2. Vulpix added to Route 37 encounters (lv14-16, 10% weight) matching Gen 2 Silver/Crystal.
- All 1318 tests pass.
- **Next (Sprint 95)**: Content sprint. Tile art + NPC wandering.

### Sprint 95 (Content — Gen 2 Tile Art + NPC Wandering)
- **Tile art overhaul** — Updated 7 core tile sprites to match Gen 2 GBC aesthetics. Grass uses subtle checkerboard pattern. Tall grass has uniform blade texture. Trees have proper light/dark shading with round canopy and trunk detail. Water has diagonal wave crests instead of diagonal lines. Path has cleaner grain. Flowers have two-flower pattern on grass base. All tiles maintain the indexed-color format (0/1/2/3) with existing palette system.
- **NPC wander logic** — Implemented random movement for NPCs with `wanders: true`. Every 2 seconds, wandering NPCs take a random step in any direction if the target tile is walkable and unoccupied (no player, no other NPC). NPCs update facing direction on each move. Uses `npc_wander_timer` field. Collision-checked against map bounds, collision types, player position, and other active NPCs.
- All 1318 tests pass.
- **Next (Sprint 96 — QA)**: Full QA audit.

### Sprint 96 (QA)
- **Full audit of Sprints 94-95** — All NPC wandering logic, Sudowoodo blocker, GenericHouse dialogue, tile sprites, and encounter tables audited.
- **LOW FIX: Dead SUDOWOODO constant** — Removed unused `const SUDOWOODO` from maps.rs (actual species ID imported from data.rs).
- All 1318 tests pass. Zero functional bugs found.
- **Next (Sprint 97)**: Content sprint.

### Sprint 97 (Content — Fishing System)
- **Fishing mechanic** — Face a water tile and press A to fish. 70% chance of bite ("Oh! A bite!" dialogue → encounter transition), 30% "Not even a nibble..." dialogue. Uses new `water_encounters` field on MapData for separate fishing encounter tables.
- **Water encounter data** — Added fishing encounters to 8 maps: Route 32 (Tentacool/Quagsire/Magikarp/Goldeen), Route 35 (Poliwag/Poliwhirl/Magikarp), Route 40 (Tentacool/Tentacruel/Magikarp/Krabby), Olivine City (Tentacool/Krabby/Magikarp), Cianwood City (Tentacool/Krabby/Magikarp), Lake of Rage (Magikarp/Gyarados), Route 44 (Poliwag/Poliwhirl/Magikarp), Ice Path (Magikarp/Seel/Shellder), Route 27 (Tentacool/Tentacruel/Magikarp/Shellder).
- **New DialogueAction::StartFishBattle** — Dialogue "Oh! A bite!" triggers battle on completion via dedicated dialogue action, ensuring text displays before encounter flash.
- **New species constants**: SHELLDER (90).
- All tests pass.
- **Next (Sprint 98)**: Content sprint.

### Sprint 98 (Content — Bicycle + Escape Rope Fix)
- **Bicycle mechanic** — Obtained from Bike Shop owner NPC in Goldenrod City (NPC index 1). Press C or Shift to toggle in overworld. Doubles movement speed (WALK_SPEED / 2). Auto-dismounts on map transition. Indoor maps block mounting. `has_bicycle` and `on_bicycle` state persisted in save data.
- **Escape Rope fix** — Previously warped to hardcoded PokemonCenter coordinates. Now uses MapFadeOut transition to PokemonCenter map, leveraging existing `last_pokecenter_map` for correct city-based exit. Also blocks use when already in PokemonCenter.
- **is_select input helper** — New input function for Select button (KeyC / ShiftLeft).
- All tests pass.
- **Next (Sprint 99 — QA)**: Full QA audit of Sprints 97-98.

### Sprint 99 (QA)
- **Full audit of Sprints 97-98** — Fishing system, bicycle mechanic, Escape Rope fix audited.
- **CRITICAL FIX: Escape Rope text not displaying** — MapFadeOut was set immediately, skipping the "Used an ESCAPE ROPE!" dialogue. Added `DialogueAction::EscapeRope` so text displays first, then fade triggers on_complete.
- **MEDIUM FIX: Fishing level range overflow** — `min_level + rng * (range+1)` could exceed max_level when rng ≈ 1.0. Added `.min(max_level)` clamp matching the grass encounter logic in maps.rs.
- PokemonCenter Escape Rope block confirmed working (line 4808).
- Bike Shop NPC index verified correct (index 1 = Bike Shop owner in Goldenrod).
- Old save backward compatibility: `has_bike` field defaults to false (0.0) for missing field, acceptable.
- All tests pass. 2 bugs fixed, 0 remaining.
- **Next (Sprint 100)**: Content sprint.

### Sprint 100 (Content — NPC Sprites + Tile Art)
- **New NPC sprites: Rocket Grunt + Fisherman** — Added NPC_ROCKET (sprite_id 6, black uniform with R on chest) and NPC_FISHER (sprite_id 7, hat and overalls with rod). Rocket HQ grunts now use Rocket sprite instead of out-of-bounds index. Lake of Rage fisherman and Route 44 fisher trainer now use Fisher sprite.
- **Sprite ID corrections** — Pryce (Mahogany Gym leader) changed to sprite_id 5 (OldMan, fits his character). Lance (Lake of Rage) changed to sprite_id 2 (Youngster, generic male). Previously both used sprite_id 7 which was out of bounds.
- **Tile art improvements** — Building roof redesigned from triangular peak to Gen 2-style layered shingles. Ledge tile improved with visible grassy cliff-top transitioning to shaded drop.
- NPC sprite cache now holds 8 sprites (indices 0-7).
- All tests pass.
- **Next (Sprint 101)**: Content sprint.

### Sprint 101 (Content — Squirtbottle + Sudowoodo Refactor)
- **Squirtbottle item flow** — Goldenrod Flower Shop lady (NPC 0) gives Squirtbottle after player earns Plain Badge (badge 2, Whitney). Uses FLAG_SQUIRTBOTTLE (bit 10). Without Squirtbottle, interacting with Sudowoodo shows "A weird tree is blocking the path" dialogue.
- **Sudowoodo refactored to dialogue-action pattern** — Removed legacy `check_sudowoodo_battle()`. Sudowoodo NPC interaction now shows "Used the SQUIRTBOTTLE!" dialogue, then `DialogueAction::StartSudowoodoBattle` triggers the Lv20 wild Sudowoodo battle on_complete. Fixes old bug where battle dialogue was skipped.
- **Legacy `check_sudowoodo()` stubbed** — Always returns false; position-based check removed since NPC interaction handles everything. Test updated to verify Squirtbottle flag system.
- All tests pass.
- **Next (Sprint 102 — QA)**: Full QA audit of Sprints 100-101.

### Sprint 102 (QA)
- **Full audit of Sprints 100-101** — NPC sprites, Squirtbottle flow, Sudowoodo refactor, tile art, sprite_id bounds all verified.
- **Zero bugs found.** All NPC sprite dimensions valid (256 chars), all sprite_id references in-bounds (0-7), Squirtbottle flag logic correct, Sudowoodo dialogue-action pattern working, tile art properly formatted.
- All tests pass. 0 bugs fixed, 0 remaining.
- **Next (Sprint 103)**: Content sprint.

### Sprint 103 (Content — SecretPotion / Lighthouse Quest)
- **Amphy lighthouse quest** — Full quest chain: Cianwood Pharmacist (NPC 5) gives SecretPotion (FLAG_MEDICINE). Deliver to Jasmine (NPC 0) at Olivine Lighthouse (FLAG_DELIVERED_MEDICINE). Jasmine disappears from Lighthouse via `is_npc_active` and returns to gym. Olivine Gym Jasmine blocks battle until medicine delivered.
- **Cianwood Pharmacist NPC** — Added at (11,6) in Cianwood City. Gives SecretPotion on first interaction; shows default pharmacy dialogue after.
- **Olivine Gym gate** — Jasmine (NPC 0) shows "GYM LEADER isn't here" dialogue if FLAG_DELIVERED_MEDICINE not set. After delivery, normal trainer battle proceeds.
- **Flags activated**: FLAG_MEDICINE (bit 7) and FLAG_DELIVERED_MEDICINE (bit 8) removed #[allow(dead_code)].
- All tests pass.

### Sprint 104 (Content — Red Gyarados Event)
- **Red Gyarados dialogue fix** — Old implementation set dialogue + battle simultaneously, skipping text. Now uses `DialogueAction::StartRedGyaradosBattle` so "The lake is churning!" text displays before Lv30 Gyarados battle begins.
- **Red Gyarados NPC** — Added visual NPC (index 3) at Lake of Rage water's edge (4,2). Hidden via `is_npc_active` after FLAG_RED_GYARADOS is set.
- **Lance post-event dialogue** — After Red Gyarados battle, Lance (NPC 0) mentions Team Rocket's suspicious shop in Mahogany Town, pointing player to Rocket HQ quest.
- All tests pass (1313).

### Sprint 105 (Content — Mystery Egg + Rocket HQ Gate)
- **Mystery Egg quest** — After getting Zephyr Badge, visiting Elm Lab triggers egg event. Elm gives Togepi Lv5 (FLAG_GOT_EGG). Elm has post-egg encouragement dialogue.
- **Togepi + Togetic species** — Added to SPECIES_DB. Normal type, Metronome/Growl/Encore learnset.
- **Rocket HQ entrance gate** — Mahogany "mart" door blocked until FLAG_RED_GYARADOS set. Shows "Just a souvenir shop" dialogue before event.
- **FLAG_GOT_EGG activated** — Removed dead_code annotation, now used in Elm Lab interaction.
- All tests pass (1313).
### Sprint 106 (QA + Tile Art Overhaul)
- **QA audit (Sprints 104-105)**: Zero bugs found. All dialogue-action patterns verified, NPC indices correct, flag sequencing valid, warp gates working.
- **Major tile art overhaul** — Rewrote 15 tile sprites for authentic Gen 2 GBC feel:
  - **Grass**: Sparse organic field (was busy checkerboard)
  - **Tall grass**: Distinct upward blade tufts (was uniform repeating pattern)
  - **Path**: Packed earth with scattered pebble grain
  - **Trees**: Fuller canopy with light dapple, trunk with bark detail and root flare
  - **Water**: Rolling waves with foam crests at peaks
  - **Building**: Brick texture with window and mortar lines
  - **Roof**: Overlapping offset shingle rows
  - **Door**: Recessed entry with handle detail
  - **PokemonCenter**: Tiled roof ridges, P logo on wall, cross emblem on entrance
  - **Sign**: Clear post with text lines on board
  - **Ledge**: Grass top with layered gradient cliff face
  - **Fence**: Wooden slat fence with spacer detail
  - **Floor**: Wood plank pattern with grain lines
  - **Heal machine**: Counter with ball slots and screen
  - **Flower**: Clear 3-petal scattered flowers
- All tests pass (1313).

### Sprint 107 (Content — Day/Night Encounters)
- **Night encounter system** — Added `night_encounters` field to MapData. New `roll_encounter_timed(roll, level_roll, is_night)` method uses night table when available, falls back to day table.
- **Night encounter data** — Routes 29, 30, 31, 38, 39 now have unique night encounter tables: Hoothoot/Noctowl replace Pidgey/Sentret, Gastly/Zubat/Spinarak/Meowth appear at night.
- **Time integration** — Overworld wild encounter check now passes `is_night` (time < 5 or >= 19) to the encounter roller.
- All tests pass (1313).

### Sprint 108 (QA)
- **Full audit of Sprints 106-107** — Tile art dimensions, night encounter system, species validity, time calculation consistency all verified.
- **Zero bugs found.** All 19 tile sprites are exactly 256 chars (16x16). Night encounter tables have valid species. `is_night` calculation matches day_night_tint definition. All 59 maps have the night_encounters field.
- All tests pass. 0 bugs fixed, 0 remaining.
- **Next (Sprint 109)**: Content sprint.

### Sprint 109 (Content — Cave/Ice/Gym Tiles)
- **4 new tile sprites** added to sprites.rs: TILE_CAVE_WALL (dark rocky surface), TILE_CAVE_FLOOR (stone with cracks), TILE_ICE_FLOOR (icy surface with highlights), TILE_GYM_FLOOR (patterned arena floor).
- **4 new tile IDs** in maps.rs: CAVE_WALL=25, CAVE_FLOOR=26, ICE_FLOOR=27, GYM_FLOOR=28.
- **3 new palettes** in render.rs: PAL_CAVE (brown-gray stone), PAL_ICE (cool blue-white), PAL_GYM (warm arena tones). Wired into tile_palette().
- **Cave maps overhauled**: Union Cave, Ice Path, Victory Road, Rocket HQ — all now use CAVE_WALL/CAVE_FLOOR instead of BLACK/FLOOR for visually distinct cave environments.
- **Ice Path** uses ICE_FLOOR for ice patches instead of WATER, giving proper icy appearance.
- **Pryce's Mahogany Gym** uses ICE_FLOOR — fitting for the ice-type gym leader.
- **All 8 gym maps** updated to use GYM_FLOOR: Violet, Azalea, Goldenrod, Ecruteak, Olivine, Cianwood, Mahogany (ICE_FLOOR), Blackthorn.
- Sprite cache expanded to include all 29 tile types (IDs 0-28).
- All 1313 tests pass. 0 bugs.
- **Next (Sprint 110)**: Content sprint.

### Sprint 110 (Content — Trainer Card Screen)
- **Trainer Card**: New `GamePhase::TrainerCard` with player name, money, Pokedex seen/caught, play time, and 8-badge grid. Accessible from pause menu (new TRAINER option between POKEDEX and SAVE). Menu expanded from 5 to 6 items.
- All 1313 tests pass.

### Sprint 111 (Content — Party Management)
- **Party swap/reorder**: PokemonMenu expanded with 3 action modes: browse, sub-menu (SUMMARY/SWAP/CANCEL), and swap selection. Players select two Pokemon to swap positions. Yellow highlight on swap source, "SWAP TO?" prompt. Battle switch path unchanged. Swap correctly updates battle player_idx if active Pokemon involved.
- All 1313 tests pass.

### Sprint 112 (Content — Progression Gates + Mobile Bike + Gym Audit)

- **`check_warp_gate` helper** — Extracted all 6 inline warp gate checks (Route27, UnionCave, IlexForest->Route34, RocketHQ, IcePath, VictoryRoad) into a single `fn check_warp_gate(&self, dest: MapId) -> Option<&'static [&'static str]>` method. Returns gate dialogue lines if blocked, None if passable. Reduces the warp processing code from ~90 lines of repetitive if-blocks to a single 10-line call site. All gate logic is now centralized and easy to audit/extend.
- **Mobile bike access via BAG** — When `has_bicycle` is true and not in battle, BICYCLE appears as the first item in the Bag menu. Selecting it toggles `on_bicycle` with feedback dialogue ("Got on the BICYCLE!" / "Got off the BICYCLE."). Indoor maps show "Can't use that here!" message. The bicycle shows "ON" status when riding, "x1" otherwise. Mobile/touch players who cannot press C/Shift can now access the bicycle. Keyboard shortcut still works too.
- **Bike Shop dialogue updated** — Now says "Use it from your BAG or press C/SHIFT!" instead of just "Press C or SHIFT to ride it!" to inform all players.
- **Gym availability audit verified**:
  - All 8 gym leaders (NPC 0 in each gym) battle on talk, not line-of-sight (line 1372).
  - `defeated_trainers` check prevents re-triggering defeated gym leaders (line 1581).
  - Jasmine (OlivineGym) correctly requires FLAG_DELIVERED_MEDICINE (line 1562).
  - Other gyms are freely fightable when reached.
- **GenericHouse exit (A1) verified** — `last_house_x`/`last_house_y` already implemented in prior sprint, storing exact door position before warp and returning player to (last_house_x, last_house_y) on exit. Serialized in save data.
- All 1313 tests pass. 0 bugs.

### Sprint 113 (QA — Audit of Sprints 110-112)

**Code audit results:**

1. **`step_pokemon_menu` swap logic** — Verified all edge cases:
   - Swap with self: guarded by `src != dst` check (line 4080), no-op. Correct.
   - Swap fainted Pokemon in overworld: allowed (matches original game behavior).
   - Swap during battle: impossible path. Swap mode (action 2) is only reachable from the sub-menu (action 1), which is only available in overworld mode (line 4018 gates on `self.battle.is_none()`). During battle, confirm goes directly to switch logic.
   - Battle `player_idx` tracking: correctly updates when either src or dst matches `player_idx` (lines 4086-4090).
   - Sub-menu cursor wraps: modulo 3 for 3 items (SUMMARY/SWAP/CANCEL). Correct.
   - Cancel from sub-menu and swap mode: both return cleanly to action 0 (browse). No state leaks.

2. **`check_warp_gate`** — All 6 gates verified:
   - Route27 from NewBarkTown: requires `badges.count_ones() >= 8`. Correct.
   - UnionCave: requires Zephyr Badge (bit 0). Correct.
   - IlexForest to Route34: requires Hive Badge (bit 1). Correct.
   - RocketHQ: requires FLAG_RED_GYARADOS. Correct.
   - IcePath: requires FLAG_ROCKET_MAHOGANY. Correct.
   - VictoryRoad: requires `badges.count_ones() >= 8`. Correct.
   - Gate nudge-back logic (lines 1187-1192): pushes player 1 tile opposite to facing direction, preventing softlock on warp tile. Correct.

3. **Bag BICYCLE integration** — Verified:
   - `bike_offset` correctly set to 1 only when `has_bicycle && battle.is_none()`.
   - Cursor arithmetic: BICYCLE at cursor 0, real items at `cursor - bike_offset`. No OOB risk.
   - `total_count` includes bike_offset for wrap math. Correct.
   - Indoor check covers all gym, cave, tower, and building MapIds. Complete.
   - Render function row offset (`row = 1` when bike shown) produces correct y-positions without overlap.

4. **TrainerCard render** — Verified:
   - Badge grid: 2 rows of 4, correct x/y layout with all 8 badge names.
   - Play time: `hours = (total_time / 3600.0) as u32`, `minutes = ((total_time % 3600.0) / 60.0) as u32`. No overflow for realistic values. For extreme values (>u32::MAX seconds), Rust's saturating f64-to-u32 cast prevents panic.
   - No panic paths: early return on missing ctx, all other ops are formatting.

**Tests added (6 new, total now 1319):**

1. `test_party_swap_basic` — 3-member party, swap index 0 and 2, verify species exchanged.
2. `test_party_swap_preserves_hp` — Swap two damaged Pokemon, verify HP/level/moves/attack preserved.
3. `test_check_warp_gate_route27` — Route27 blocked with 0 and 7 badges, passable with 8.
4. `test_check_warp_gate_union_cave` — UnionCave blocked without Zephyr Badge, passable with it.
5. `test_check_warp_gate_victory_road` — VictoryRoad blocked without 8 badges, passable with 8.
6. `test_trainer_card_time_display` — Hours/minutes calculation for normal, large, edge, and extreme total_time values.

**Bugs found: 0.** Code from Sprints 110-112 is clean. All 1319 tests pass.

### Sprint 114 (Content — Daycare System + Ice Path Sliding Puzzle)

**Ice Path Sliding Puzzle:**
- **New collision type `C_ICE` (8)** — Added `CollisionType::Ice` to maps.rs. Ice tiles are walkable but trigger sliding: player automatically continues moving in their current direction until hitting a non-ice tile (wall, rock, regular floor) or map edge. Walk speed doubled on ice for a faster slide feel.
- **`ice_sliding: Option<Direction>`** — New field on `PokemonSim`. When `Some(dir)`, the player is mid-slide; input is ignored, menu blocked. Cleared on map transition, trainer approach, or landing on non-ice.
- **Ice Path map redesign** — Replaced placeholder `C_WATER` ice tiles with proper `C_ICE` collision. Added strategic rock walls (`C_SOLID` at (7,3), (7,4), (7,6), (6,9)) to create a solvable sliding puzzle requiring 3+ direction changes. Player enters west, slides through ice patches, and must navigate rocks to reach east exit. Trainers repositioned to walkable floor tiles.

**Daycare System:**
- **`daycare_pokemon: Option<Pokemon>` + `daycare_steps: u32`** — New fields on `PokemonSim`, initialized to `None`/0.
- **Route 34 Day-Care Man (NPC 0)** — Special interaction: if no Pokemon in daycare, offers deposit with `DaycareDeposit` dialogue action. If Pokemon present, shows level and offers return with `DaycareReturn` action + YES/NO prompt.
- **Daycare deposit screen (`GamePhase::DaycareDeposit`)** — Shows party list with cursor, confirms deposit. Blocks deposit if party has only 1 Pokemon. Removes from party, stores in `daycare_pokemon`, resets `daycare_steps`.
- **Daycare return prompt (`GamePhase::DaycarePrompt`)** — YES/NO over overworld. YES: calculates cost ($100 + $100 * levels_gained), checks money, returns Pokemon to party. Blocks if party full. NO: "I'll keep raising it."
- **Step counting** — Every overworld step adds 1 EXP to daycare Pokemon. Auto-levels when EXP threshold reached (using `exp_for_level` with species growth rate). Gen 2 move replacement: new moves shift into last slot, oldest move drops out.
- **Save/load** — Daycare Pokemon serialized as JSON object in save data (`"daycare":{...},"daycare_steps":N`). Deserialized with balanced-brace parser matching the existing party format.

All 1319 tests pass.

### Sprint 115 (Content — NPC Behavior Overhaul + Headless Test Infrastructure)

**Part 1: NPC Dialogue Overhaul**
Updated NPC dialogue across 18+ locations to be meaningful and city-specific:
- **NewBarkTown**: Youngster references Prof. Elm's lab; Lass points toward Cherrygrove
- **Route 29**: Guide Gent mentions Cherrygrove City ahead
- **CherrygroveCity**: Guide Gent references MR. POKEMON; Youngster hints at Route 30
- **Route 30**: Added hint NPC about MR. POKEMON's house
- **Route 31**: Added hint NPC about Violet City gym
- **VioletCity**: Old man references Sprout Tower; Youngster warns about Falkner's Pidgeotto
- **Route 32**: Added hint NPC about Union Cave to Azalea Town
- **AzaleaTown**: NPC updated to mention Kurt and Bugsy
- **Route 34**: Added daycare hint NPC (placed on walkable tile at 5,14)
- **GoldenrodCity**: City guide mentions Dept Store and Whitney
- **Route 35**: Added National Park hint NPC
- **Route 36**: Updated weird tree NPC with water hint
- **Route 37**: Added Ecruteak legends hint NPC
- **EcruteakCity**: Old man updated about Burned Tower and Tin Tower
- **Route 39**: NPC updated about Olivine Lighthouse
- **OlivineCity**: Town guide references Jasmine and Ampharos
- **MahoganyTown**: Shop NPC now warns about suspicious shop (Rocket HQ entrance)
- **BlackthornCity**: All 3 NPCs updated with Clair/Dragon references, last JOHTO badge hint
- **Route 42**: Non-trainer NPC updated with Lake of Rage hint
- **Route 44**: Added Ice Path warning hint NPC
- **Route 46**: Added Route 29/New Bark Town connection hint NPC

**Part 2: Headless Test Infrastructure**

*Input builder helpers* (in `#[cfg(test)]` module):
- `press(key)` -- single key-press frame
- `hold(key, n)` -- hold a key for N frames (keys_held repeated)
- `wait(n)` -- N empty frames
- `walk_dir(dir, gap)` -- press direction + gap empty frames (one tile movement)
- `sequence(seqs)` -- concatenate multiple input sequences

*Test constructor*:
- `PokemonSim::with_state(map, x, y, party, badges)` -- creates a sim already in Overworld, skipping title screen. Sets has_starter, party, badges, and calls change_map.

*5 new integration tests*:
1. `test_input_builder_helpers` -- validates all helper functions produce correct InputFrame sequences
2. `test_with_state_overworld` -- verifies with_state places player at correct map/position with correct state
3. `test_daycare_deposit_withdraw` -- tests deposit, step-counting level gain, and withdrawal
4. `test_ice_sliding_basic` -- tests ice_sliding field and IcePath map structure
5. `test_warp_gate_progression` -- comprehensive gate test across Union Cave, Ilex Forest, Route 27, and Victory Road

All 99 Pokemon module tests pass.

---

### Sprint 116 — QA Audit (Sprints 114-115)

**Scope**: Audit Ice Path sliding, Daycare system, NPC dialogue, and test infrastructure from Sprints 114-115.

**Audit Results**:

1. **Ice Path Sliding (Sprint 114)** -- PASS
   - Sliding starts correctly when stepping onto C_ICE tiles (line 1481)
   - Sliding continues through ice and stops on first non-ice tile (Walkable/Solid/edge)
   - `ice_sliding` cleared on map transitions (MapFadeOut handler, line 6751)
   - Menu blocked during sliding (line 1205 checks `ice_sliding.is_none()`)
   - Ice Path collision map has correct C_ICE placement across rows 2-9
   - Puzzle is solvable: enter at (0,7), walk right to (3,7), slide right to (7,7), step right and slide to exit warp at (13,7)
   - Trainer approach correctly stops ice sliding (line 1467)

2. **Daycare System (Sprint 114)** -- BUG FIXED
   - Deposit correctly removes from party (`party.remove(idx)`)
   - Can't deposit with only 1 Pokemon (checked at `party.len() <= 1`)
   - Step counting increments `daycare_steps`, adds 1 EXP/step, auto-levels with Gen 2 move replacement
   - Withdrawal blocked if party full (6 Pokemon) or insufficient money
   - Save/load correctly serializes daycare_pokemon as JSON object
   - **BUG FOUND**: Withdrawal cost calculated as `pkmn.level - 1` instead of actual levels gained in daycare. Fixed by adding `daycare_deposit_level` field to track level at deposit time. Cost is now correctly `$100 + $100 * (current_level - deposit_level)`. Field persisted in save/load as `daycare_dlvl`.

3. **NPC Dialogue (Sprint 115)** -- PASS
   - Spot-checked 5 maps: Route 34 (4 NPCs), Route 44 (4 NPCs), Ice Path (2 NPCs), Blackthorn City (NPCs), and verified:
     - All NPCs placed on walkable tiles (C_WALK)
     - No NPC position conflicts with warps or other NPCs
     - Dialogue text is appropriate and contextual

4. **Test Infrastructure (Sprint 115)** -- PASS
   - All helper functions present: `press`, `empty`, `hold`, `wait`, `walk_dir`, `sequence`
   - `PokemonSim::with_state` exists and works correctly
   - Added `#[allow(dead_code)]` to `hold`, `wait`, `walk_dir`, `sequence`, `with_state` to suppress future warnings

**Changes Made**:
- Added `daycare_deposit_level: u8` field to PokemonSim (tracks level at deposit time)
- Fixed daycare withdrawal cost: `$100 + $100 * (current_level - deposit_level)` instead of `$100 + $100 * (current_level - 1)`
- Updated save format to include `daycare_dlvl` field, load correctly restores it
- Added `#[allow(dead_code)]` to 5 test helper functions

**Test Results**: All 1329 tests pass (99 Pokemon module tests). Clean compilation.

### Sprint 117: Gen 2 Tilemap Art Integration (2026-03-07)

**Goal**: Convert open-source MIT-licensed Gen 2 tilemap art to engine format and replace hand-coded tile sprites.

**Assets Source**: [Pokemon Gen 2 Style Tilemap](https://github.com/nikouu/Pokemon-gen-2-style-tilemap) - MIT licensed. Local copy at `engine/assets/tilesets/`.

**What was done**:

1. **Analyzed all Constructed/ PNGs** - inspected pixel dimensions, color counts, and visual content of 20+ pre-composed 16x16 tiles and multi-tile composites.

2. **Tested png_to_sprites.py converter** - verified the PNG-to-sprite converter works correctly on 4-color indexed PNGs. The Constructed/ folder PNGs (16x16, exactly 4 colors) convert cleanly.

3. **Replaced 5 existing tile sprites** with converted versions from Constructed/ PNGs:
   - `TILE_TREE_TOP` - from Tree-short.png (richer round canopy with leaf detail)
   - `TILE_TREE_BOTTOM` - from Tree-tall.png bottom half (better trunk with bark texture)
   - `TILE_SIGN` - from Sign.png (detailed signpost with frame and text area)
   - `TILE_BOOKSHELF` - from Bookshelf-type-01.png top half (book rows with shelf detail)
   - `TILE_PC` - from Tv-type-01.png (screen with frame, replaces simple PC)
   - `TILE_LAB_WALL` - from Back-wall-window.png (interior wall with window panes)

4. **Added 9 new tile types** (tile IDs 29-37) from Constructed/ PNGs:
   - `TILE_STOVE` (29) - kitchen stove with burners and oven door
   - `TILE_WALL_ART` (30) - wall painting/picture frame
   - `TILE_STOOL` (31) - pink stool/chair
   - `TILE_BED_TOP` (32) - bed upper half with pillow
   - `TILE_BED_BTM` (33) - bed lower half with blanket
   - `TILE_FRIDGE` (34) - refrigerator with shelves
   - `TILE_EXIT_MAT` (35) - indoor exit mat (red)
   - `TILE_OUTDOOR_MAT` (36) - outdoor door mat (red checkerboard)
   - `TILE_TREE_TALL_TOP` (37) - tall tree upper canopy section

5. **Also tried full 8x8 tilesheet** (Original.png) - extracted 160 tiles but most were too blocky when upscaled 2x. The Constructed/ 16x16 PNGs are superior quality.

6. **Kept existing sprites** where hand-coded versions were already good or 8x8 source was too limited:
   - TILE_GRASS, TILE_TALL_GRASS, TILE_PATH (subtle detail lost at 8x8)
   - TILE_WATER, TILE_WATER2 (hand-coded wave animation looks good)
   - TILE_FLOOR, TILE_DOOR, TILE_BUILDING_WALL, TILE_BUILDING_ROOF (existing quality adequate)

**Files changed**:
- `sprites.rs` - replaced 6 tile constants, added 9 new tile constants + compile-time assertions
- `mod.rs` - added new tiles to `init_sprite_caches()` tile_strs array
- `render.rs` - added palette mappings for tile IDs 29-37

**Test Results**: All 1324 tests pass + 2 fuzz + 3 golden replay. Clean compilation.

---

## Sprint 118: Fly system + overworld item use + repel system

### 1. FLY SYSTEM

**Visited cities tracking**: Added `visited_cities: Vec<MapId>` to PokemonSim. When the player enters a city (tracked in `change_map()`), it's added to the list if not already present. 10 flyable cities: NewBarkTown, CherrygroveCity, VioletCity, AzaleaTown, GoldenrodCity, EcruteakCity, OlivineCity, CianwoodCity, MahoganyTown, BlackthornCity.

**Helper methods**:
- `is_fly_city(MapId) -> bool` - checks if a map is a flyable city
- `fly_spawn(MapId) -> (u8, u8)` - returns spawn coordinates for each city (from pokecrystal-master/data/maps/spawn_points.asm)
- `city_name(MapId) -> &str` - display name for the fly menu

**FlyMenu phase**: `GamePhase::FlyMenu { cursor: u8 }` shows a list of visited cities. Navigate with up/down, confirm with Z to fly, cancel with X.

**Access from bag**: FLY appears as a virtual item at the top of the bag menu when the player has any badges and is not in battle. Selecting it opens the FlyMenu. Indoor locations block FLY use.

**Fly execution**: When a city is selected, auto-dismounts bicycle, clears ice_sliding, and triggers MapFadeOut to the city's spawn point.

**Persistence**: `visited_cities` is serialized as a JSON array of map name strings in save data, and parsed back on load.

### 2. OVERWORLD ITEM USE

**New items added to data.rs** (8 new item types, IDs 12-19):
- Hyper Potion (ID 12) - restores 200 HP, $1200
- Max Potion (ID 13) - fully restores HP, $2500
- Full Restore (ID 14) - fully restores HP + cures status, $3000
- Rare Candy (ID 15) - raises Pokemon by 1 level, $4800
- Awakening (ID 16) - cures sleep, $250
- Ice Heal (ID 17) - cures freeze, $250
- Super Repel (ID 18) - 200 steps, $500
- Max Repel (ID 19) - 250 steps, $700

**ItemData struct extended** with two new fields:
- `is_rare_candy: bool` - identifies Rare Candy for level-up handling
- `repel_steps: u32` - number of repel steps (0 = not a repel item)

**Full Restore handling**: Detects items with both `heal_amount > 0` and `is_status_heal`, heals HP fully and cures status in one use.

**Rare Candy handling**: From BagUseItem target selection:
- Blocked for fainted or LV100 Pokemon
- Increments level, sets EXP to level threshold
- Recalcs stats (HP difference added to current HP)
- Auto-learns moves at new level (or shows "slots full" message)
- Checks evolution and triggers via `DialogueAction::CheckEvolution`

**New DialogueAction::CheckEvolution**: After Rare Candy dialogue completes, checks `pending_evolution` global state and triggers Evolution phase if needed.

### 3. REPEL SYSTEM

**Generic repel handling**: All repel items (Repel/Super Repel/Max Repel) now use the `repel_steps` field from ItemData. Single code path handles all three.

**Repel wore off dialogue**: When `repel_steps` decrements to 0 during a step, shows "REPEL's effect wore off!" dialogue and transitions to Dialogue phase.

**Gen 2 encounter suppression**: Wild encounter check now implements the Gen 2 rule: if repel is active AND the wild Pokemon's level is less than the lead Pokemon's level, the encounter is suppressed. Wild Pokemon at or above the lead's level can still appear even with repel active.

**Repel activation message**: "REPEL's effect lingered!" shown when using any repel item (matches Gen 2).

**Files changed**:
- `data.rs` - 8 new item constants (IDs 12-19), ItemData struct extended with `is_rare_candy` and `repel_steps` fields, 8 new ITEM_DB entries
- `mod.rs` - FlyMenu GamePhase, visited_cities field + save/load, fly helper methods, step_fly_menu/render_fly_menu, FLY virtual item in bag, generic repel handling, Rare Candy + Full Restore in BagUseItem, Gen 2 repel encounter suppression, repel wore-off dialogue, CheckEvolution DialogueAction, AI-INSTRUCTIONS updated

---

## Sprint 119 — QA Audit of Sprints 117-118

**Type**: QA audit of Sprint 117 (tile art) and Sprint 118 (fly system, overworld items, repel).

### Audit Results

#### 1. Tile Art (Sprint 117) -- PASS
- All 38 tile constants in sprites.rs verified at exactly 256 characters via compile-time assertions (lines 1703-1740)
- Tile IDs 29-37 all have palette mappings in render.rs tile_palette() (lines 331-340)
- init_sprite_caches includes all 38 tiles including the 9 new ones (lines 559-569)
- Spot-checked new tiles: all have 4 unique color indices, none are all-zeros or all-same-char

#### 2. Fly System (Sprint 118) -- BUGS FOUND AND FIXED
- visited_cities populated on city entry via change_map() -- PASS
- FlyMenu step/render wired into main step()/render() match arms -- PASS
- Save/load preserves visited_cities (JSON array, round-trip verified) -- PASS
- Fly clears ice_sliding and bicycle (on_bicycle=false, ice_sliding=None) -- PASS
- FLY shown in bag when badges > 0 and not in battle -- PASS
- **BUG FOUND**: BlackthornCity fly spawn at (8, 8) landed on a SOLID tile (house roof). Fixed to (3, 7) -- walkable path below PokemonCenter.

#### 3. Overworld Item Use (Sprint 118) -- BUG FOUND AND FIXED
- Potion (20 HP), Super Potion (50 HP), Hyper Potion (200 HP) heal correct amounts -- PASS
- Max Potion heals to max (heal_amount: 9999, capped by min()) -- PASS
- Rare Candy: levels up, recalcs stats, checks evolution, auto-learns moves -- PASS
- Status heals (Antidote, Awakening, Ice Heal, Paralyze Heal, Full Heal) work -- PASS
- Item consumption via bag.use_item() after use -- PASS
- **BUG FOUND**: Full Restore was unreachable! The status heal check (`is_status_heal`) fired before the Full Restore check (`heal_amount > 0 && is_status_heal`), so Full Restore only cured status without healing HP. Fixed by moving the Full Restore check before the plain status heal check and removing the dead duplicate block.

#### 4. Repel System (Sprint 118) -- PASS
- Repel/Super Repel/Max Repel step values: 100/200/250 -- correct per Gen 2
- Step decrement in walk completion handler -- PASS
- "REPEL's effect wore off!" dialogue triggers when steps hit 0 -- PASS
- Gen 2 suppression rule: wild encounter blocked only if wild level < lead Pokemon level -- PASS

#### 5. Tests Added
- `test_fly_destinations` -- verifies all 10 fly cities have valid, in-bounds, non-solid spawn points
- `test_repel_steps` -- verifies Repel (100), Super Repel (200), Max Repel (250) step values
- `test_item_data_completeness` -- verifies all 19 item IDs have valid ITEM_DB entries with correct names, prices, and heal amounts

### Bugs Fixed
1. **BlackthornCity fly spawn on solid tile** -- spawn (8,8) was on a house roof tile. Changed to (3,7), walkable path below PokemonCenter.
2. **Full Restore unreachable code path** -- The item use ordering in step_bag_use_item checked `is_status_heal` before `heal_amount > 0 && is_status_heal`, causing Full Restore to only cure status without healing HP. Reordered checks so Full Restore is handled first, then plain status heals.

### Files Changed
- `mod.rs` -- Fixed Full Restore item use ordering (moved combined HP+status check before plain status check, removed dead duplicate block), fixed BlackthornCity fly spawn coordinates (8,8 -> 3,7), added 3 new tests (test_fly_destinations, test_repel_steps, test_item_data_completeness)

**102 tests passing. cargo check clean.**

---

## Sprint 120: Battle Phase Sequencer -- Phase 1 (Infrastructure + Run Migration)

**Objective:** Add queue-based battle step infrastructure alongside the existing phase system, then migrate one flow (Run/escape) to demonstrate the pattern.

### What Was Added

#### 1. BattleStep Enum
New `BattleStep` enum with 8 step types for queue-based battle sequencing:
- `Text(String)` -- display text, auto-advance after 1.5s or on confirm press
- `ApplyDamage { is_player, amount }` -- deduct HP from target Pokemon immediately
- `DrainHp { is_player, to_hp, duration }` -- smooth HP bar interpolation animation
- `InflictStatus { is_player, status }` -- set StatusCondition on target
- `StatChange { is_player, stat, stages }` -- apply stat stage modification (-6 to +6)
- `CheckFaint { is_player }` -- check if target fainted, transition to faint handling
- `Pause(f64)` -- wait N seconds with no text
- `GoToPhase(Box<BattlePhase>)` -- escape hatch to transition to any existing BattlePhase

#### 2. BattleState Queue Fields
- `battle_queue: VecDeque<BattleStep>` -- FIFO queue of steps to process
- `queue_timer: f64` -- timer for current queue step (text display, HP drain, pause)

#### 3. BattlePhase::ExecuteQueue
New phase variant that drives the queue. When active, `step_execute_queue()` pops and processes steps from the front of `battle_queue`. When queue empties, defaults to `ActionSelect`.

#### 4. step_execute_queue() Function
Static method `Self::step_execute_queue(battle, party, engine)` handles all 8 step types:
- Text: renders via render_battle, advances on timer or confirm
- ApplyDamage: immediate HP deduction
- DrainHp: exponential decay interpolation toward target HP
- InflictStatus: sets status on target Pokemon
- StatChange: modifies stat stages with clamping
- CheckFaint: transitions to PlayerFainted or EnemyFainted (with EXP calc)
- Pause: simple timer wait
- GoToPhase: sets battle.phase to the boxed phase

#### 5. queue_attack_sequence() Helper
Builds a standard attack flow: "X used Y!" text, pause, apply damage, drain HP, optional crit/effectiveness text, faint check. Ready for Sprint 121 migration.

#### 6. Render Support
`ExecuteQueue` render arm peeks at queue front -- if it's a `Text` step, renders the text box. Otherwise renders the normal battle scene (sprites + HP bars already drawn by shared render code above the match).

### Migration: Run Escape Flow
**Before:** Run phase immediately exited to `GamePhase::Dialogue` with "Got away safely!" text, then returned to Overworld.
**After:** Run success enqueues `[Text("Got away safely!"), GoToPhase(Run)]` and enters `ExecuteQueue`. The text displays within the battle scene with the proper 1.5s timer / confirm advancement. Then `GoToPhase(Run)` transitions to the Run handler which does cleanup (clear battle, return to Overworld).

### Design Decisions
- Queue system is **additive** -- all existing BattlePhase handlers remain untouched (except Run, which was migrated)
- `step_execute_queue` is a static method taking `&mut BattleState` directly (avoids the take-put-back pattern since it doesn't need `self`)
- `BattleStep` derives `Clone + Debug` to match component conventions
- No `unwrap()` anywhere in the new code -- all party access uses `get()`/`get_mut()` with fallbacks
- All math uses `f64`

### Files Changed
- `mod.rs` -- Added VecDeque import, BattleStep enum (8 variants), ExecuteQueue phase variant, battle_queue/queue_timer fields on BattleState (all 5 constructor sites), step_execute_queue() function, queue_attack_sequence() helper, ExecuteQueue render arm, migrated Run escape to use queue, updated AI-INSTRUCTIONS comment

**1332 tests passing (1327 unit + 2 fuzz + 3 golden replay). cargo check clean. No warnings.**

**Test Results**: All 1324 tests pass + 2 fuzz + 3 golden replay. Clean compilation.

## Sprint 121: Battle Sequencer Migration -- Phase 2 (RunFailed, Intro, Won)

**Objective:** Migrate 3 additional battle flows from direct BattlePhase transitions to the queue-based BattleStep system, plus add unit tests for the queue itself.

### Migrations

#### 1. RunFailed Flow (was: dedicated RunFailed { timer } phase)
**Before:** Failed escape set `BattlePhase::RunFailed { timer: 0.0 }`. RunFailed handler showed "Can't escape!" for 1.0s, then called `calc_enemy_move()` and transitioned to `EnemyAttack`.
**After:** Failed escape now calculates the enemy move immediately, then enqueues `[Text("Can't escape!"), GoToPhase(EnemyAttack { ... })]` and enters `ExecuteQueue`. Text displays with the standard 1.5s timer / confirm advancement, then GoToPhase transitions to EnemyAttack with pre-calculated move data. The `RunFailed` enum variant is now dead code (kept for exhaustive matching, suppressed with `#[allow(dead_code)]` on BattlePhase).

#### 2. Intro Flow (was: timer-based Intro -> Text -> ActionSelect chain)
**Before:** Intro phase ran a 1.5s timer showing "Wild X appeared!" / "Trainer sent out X!" in render code, then created a `BattlePhase::Text` for "Go! POKEMON!" which chained to `ActionSelect`.
**After:** After the 1.5s intro animation completes, the phase enqueues `[Text("Go! POKEMON!"), GoToPhase(ActionSelect { cursor: 0 })]` and enters `ExecuteQueue`. The intro fade-in animation is unchanged (still uses the Intro { timer } phase for the first 1.5s). Only the "Go! X!" text display and ActionSelect transition use the queue.

#### 3. Won Flow (was: Won { timer: 0.0 } with 1s delay showing "You won!")
**Before:** Four code paths set `BattlePhase::Won { timer: 0.0 }` (from EXP award, LevelUp, LearnMove::LearnedMove, LearnMove::DidNotLearn). Won handler waited 1.0s showing "You won!", then did complex cleanup (money, badges, evolution, trainer tracking).
**After:** All 4 code paths now enqueue `[Text("You won!"), GoToPhase(Won { timer: 2.0 })]` and enter `ExecuteQueue`. Text displays with standard queue timing. GoToPhase transitions to Won with timer pre-set to 2.0, which immediately triggers the `t > 1.0` cleanup branch on the next frame. All cleanup logic (money awards, badge grants, evolution checks, champion detection) is unchanged.

### New Tests

#### test_battle_queue_drains_correctly
Creates a BattleState with a pre-populated queue of `[Text, Pause(0.1), ApplyDamage, GoToPhase(ActionSelect)]`. Steps through the queue by calling `step_execute_queue()` for the correct number of frames:
- Text: ~91 frames (1.5s at 60fps)
- Pause(0.1): ~7 frames
- ApplyDamage: 1 frame (instant)
- GoToPhase: 1 frame (instant)
Verifies queue length decreases after each step, enemy HP decreases after ApplyDamage, and final phase is ActionSelect.

#### test_battle_queue_check_faint_transitions
Creates a BattleState with an enemy at 0 HP, enqueues `CheckFaint { is_player: false }`, and verifies that stepping the queue transitions to `BattlePhase::EnemyFainted`.

### Design Decisions
- **Enemy move pre-calculation for RunFailed**: The enemy move is calculated at the point of failure (in the ActionSelect Run handler) rather than after the text display. This avoids needing `self` inside the queue processor. Since the RNG state is consumed at the same logical point, this is mechanically identical.
- **Won timer skip**: GoToPhase(Won { timer: 2.0 }) re-enters the Won handler with timer > 1.0, immediately triggering cleanup. This avoids duplicating the complex cleanup logic or adding a new phase variant.
- **RunFailed variant kept**: The enum variant is retained (with `#[allow(dead_code)]`) rather than removed, since removing it would require editing match arms in both step and render functions. It can be cleaned up in a future sprint.

### Files Changed
- `mod.rs` -- Migrated RunFailed creation to queue (ActionSelect Run handler), migrated Intro post-animation to queue, migrated all 4 Won { timer: 0.0 } sites to queue, added `#[allow(dead_code)]` on BattlePhase enum, added 2 queue unit tests (test_battle_queue_drains_correctly, test_battle_queue_check_faint_transitions), updated AI-INSTRUCTIONS comment

**Test Results**: All 1329 unit tests pass + 2 fuzz + 3 golden replay. Clean compilation (0 warnings in pokemon/mod.rs).

## Sprint 122: QA Audit of Battle Sequencer (Sprints 120-121)

**Objective:** Thorough QA audit of the queue-based battle sequencer introduced in Sprints 120-121. Find and fix bugs, remove dead code, add comprehensive tests.

### Audit Findings

#### Bug Fix: GoToPhase did not clear remaining queue items
**Problem:** `BattleStep::GoToPhase` only called `pop_front()` to remove itself, leaving any subsequent steps in the queue. If `ExecuteQueue` was later re-entered (via another queue-building flow), stale steps from a previous sequence could execute unexpectedly.
**Fix:** Changed GoToPhase handler to call `battle_queue.clear()` instead of `pop_front()`. GoToPhase is a terminal step -- any steps after it in the queue are unreachable and should be discarded.

#### Dead Code Removal: RunFailed variant fully removed
**Problem:** Sprint 121 migrated RunFailed to the queue system but kept the `BattlePhase::RunFailed { timer }` variant, its step handler (lines 3822-3832), and its render handler as dead code with `#[allow(dead_code)]`.
**Fix:** Removed the RunFailed variant from the BattlePhase enum, its step handler, and its render handler. The run-failed flow is now exclusively handled via `[Text("Can't escape!"), GoToPhase(EnemyAttack)]` in the queue. No external code transitions to RunFailed.

#### Safety: Empty queue fallback now logs a warning
**Problem:** When `step_execute_queue` was called with an empty queue, it silently fell back to ActionSelect. This could mask bugs where queue sequences were missing their terminal GoToPhase step.
**Fix:** Added a `crate::log::warn()` call before the ActionSelect fallback, making the safety net observable in logs.

#### Updated AI-INSTRUCTIONS comment
Updated the header comment to reflect RunFailed removal and GoToPhase queue-clearing behavior.

### New Tests (14 tests added)

| Test | What it verifies |
|------|-----------------|
| `test_goto_phase_clears_remaining_queue` | GoToPhase discards all remaining queue items after transition |
| `test_empty_queue_fallback_to_action_select` | Empty queue safely falls back to ActionSelect |
| `test_intro_sequence_via_queue` | Intro flow: "Go! CYNDAQUIL!" text (1.5s) then ActionSelect |
| `test_won_flow_via_queue` | Won flow: "You won!" text then Won { timer: 2.0 } for instant cleanup |
| `test_run_success_flow_via_queue` | Run success: "Got away safely!" text then Run phase |
| `test_run_failed_flow_via_queue` | Run failed: "Can't escape!" text then EnemyAttack phase |
| `test_drain_hp_animation_completes` | DrainHp animates HP display from current to target over duration |
| `test_inflict_status_step` | InflictStatus correctly applies Poison to enemy |
| `test_stat_change_step` | StatChange lowers enemy ATK by 1 stage |
| `test_stat_change_clamps_at_bounds` | StatChange clamps at -6/+6 boundaries |
| `test_check_faint_no_faint_continues_queue` | CheckFaint on non-fainted Pokemon continues queue |
| `test_check_faint_player_transitions_to_player_fainted` | CheckFaint on fainted player transitions to PlayerFainted |
| `test_apply_damage_to_player` | ApplyDamage correctly subtracts HP from player Pokemon |
| `test_apply_damage_saturates_at_zero` | ApplyDamage does not underflow past 0 HP |
| `test_text_step_advances_on_confirm` | Text step advances early when confirm button is pressed |
| `test_full_attack_queue_sequence` | Complete 6-step attack sequence (Text → Pause → ApplyDamage → DrainHp → CheckFaint → GoToPhase) |

### Test Infrastructure
Added two shared helpers to reduce boilerplate in queue tests:
- `make_test_battle(party, enemy, is_wild)` — builds a minimal BattleState in ExecuteQueue phase
- `step_until_phase_change(battle, party, engine, max_frames)` — steps queue until phase changes from ExecuteQueue

### Files Changed
- `mod.rs` — Removed RunFailed variant + handler + render handler, fixed GoToPhase to clear queue, added empty-queue warning, added 14 new tests + 2 test helpers, updated AI-INSTRUCTIONS comment

**Test Results**: All 1345 unit tests pass + 2 fuzz + 3 golden replay. Clean compilation.