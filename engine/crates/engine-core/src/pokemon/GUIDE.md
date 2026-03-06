# Pokemon Gold: Johto Completion Guide

This is the living strategy document for the Pokemon Gold/Silver recreation on the Crusty engine. Agents update this file after each sprint. Refer to `ENGINE_POKEMON.md` for engine patterns and QA history.

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
- ☐ **0E. Debug State Export** — Export `player_x`, `player_y`, `current_map`, `badges`, `story_flags`, `party_size`, `step_count`, `defeated_count`, `lead_hp`, `lead_level` to `global_state` every `step()`. Enables headless regression detection.
- ☐ **0F. File Splits** — Split `maps.rs` (currently >7,000 lines): keep types/enum/tests, move `build_*` to `maps_early.rs` and `maps_late.rs`. Split `mod.rs` when >4,000 lines: extract `battle.rs` and `menus.rs`.

### Phase 1: Data Tables ✓ (ongoing)
Species and moves added as needed per sprint. Currently ~125 species, ~147 moves. Continue adding for E4/Champion teams and remaining encounter tables.

### Phase 2: Maps — Olivine, Cianwood, Mahogany (Gyms 5-7) ✓
All maps built through Lake of Rage and Mahogany Gym. RocketHQ still needed for story gating.

### Phase 3: Maps — Blackthorn through Victory Road (Gym 8 + E4)
_In progress._ Route 44 ✓, Ice Path ✓, Blackthorn ✓, Route 45 ✓, Route 46 ✓, Route 27 ✓, Route 26 ✓.

**Remaining**: VictoryRoad → IndigoPlateau → EliteFourWill → EliteFourKoga → EliteFourBruno → EliteFourKaren → ChampionLance

E4 rooms themed to type specialty. Warp to next room after victory.

### Phase 4: Trainer Teams
All 8 gym leaders wired with correct teams and badge rewards.

**Remaining — Elite Four** (from `data/trainers/parties.asm`):
- Will: Xatu lv40, Jynx lv41, Exeggutor lv41, Slowbro lv41, Xatu lv42
- Koga: Ariados lv40, Forretress lv43, Muk lv42, Venomoth lv41, Crobat lv44
- Bruno: Hitmontop lv42, Hitmonlee lv42, Hitmonchan lv42, Onix lv43, Machamp lv46
- Karen: Umbreon lv42, Vileplume lv42, Gengar lv45, Murkrow lv44, Houndoom lv47
- Lance: Gyarados lv44, Dragonite lv47, Dragonite lv47, Aerodactyl lv46, Charizard lv46, Dragonite lv50

**Rival** at Victory Road: starter final form lv36 + Haunter lv35 + Sneasel lv34 + Magneton lv34 + Golbat lv36

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

### Phase 7: Save System
Not yet implemented. Serialize complete state as single JSON blob via `PersistCommand::Set`. One localStorage key (`pokemon_save`). Atomic write.

**Includes**: map, position, facing, money, badges, story_flags, total_time, rng_state, full party, full PC, defeated_trainers, pokedex.

**JS side**: Wire `drain_persist_commands()` → `localStorage`. On startup, read and push via `set_game_state_str()`.

**Title screen**: CONTINUE + NEW GAME. Auto-save on map transitions.

### Phase 8: Credits
Not yet implemented. `GamePhase::Credits { timer: f64 }`. Scrolling text on framebuffer. Return to title.

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