# ENGINE_POKEMON.md — Engine Notes from 42 Sprints of Pokemon Development

## Overview

Pokemon Gold/Silver/Crystal recreation built on the Crusty engine. ~11,800 lines of Rust across 5 files, implementing ~42 species, 29 maps (Johto through Ecruteak City), turn-based battles with stat stages, and a full overworld with trainer battles. All rendering is Rust-side framebuffer at GBC resolution (160x144), with a JavaScript overlay canvas for battle sprites loaded from the web. Scope: Johto region only (no Kanto post-game).

## Engine Observations

### What Worked Well

**The Simulation Trait Pattern**
The `Simulation::step()` + `Simulation::render()` split was perfect for a game this complex. Battle logic stays in `step()`, visual-only state (camera lerp, screen flash timers) updates in `render()`. The engine's determinism guarantee means replays would work out of the box if we added InputFrame recording.

**Take-Put-Back for Borrow Management**
Rust's borrow checker fights you hard with game state. The pattern of `let mut battle = self.battle.take().unwrap()` → process → `self.battle = Some(battle)` cleanly avoids simultaneous `&self` and `&mut self.battle` borrows. This pattern scaled well from simple wild encounters to multi-Pokemon trainer battles with party switching.

**The Framebuffer Renderer**
Pixel-level control at 160x144 was ideal for GBC-style games. The `fill_rect`, `draw_text_scaled`, and `draw_sprite` primitives composed naturally into tile grids, HP bars, and menu boxes. No abstraction overhead — just writing pixels.

**SoundCommand Queue**
Rust pushes `SoundCommand { waveform, frequency, duration, volume }` to a queue; JavaScript drains it with Web Audio. Clean separation, no web-sys in game code. Added battle hit sounds, level-up fanfares, and menu blips with minimal code.

**Global State Bridge**
`engine.global_state.set_str("enemy_pokemon", "totodile")` in Rust → `get_game_state_str("enemy_pokemon")` in JavaScript. This let us load Pokemon sprites from a CDN without any image handling in Rust. The bridge is also how we signal battle start/end, healing animations, and evolution sequences to the JS overlay.

**Stat Stage System (Sprint 36)**
Implemented all 7 stat stages (Atk, Def, SpAtk, SpDef, Speed, Accuracy, Evasion) with Gen 2 multiplier tables. The `[i8; 7]` array per battler is compact, and stage-modified stats feed cleanly into damage calc. Moves like Growl, Defense Curl, and Scary Face "just work" once the stage infrastructure exists.

### Pain Points & Lessons

**File Size**
`mod.rs` hit ~3430 lines and `maps.rs` ~5020 lines. Both are well past comfortable navigation. maps.rs especially — each new city adds ~300-400 lines of tile/collision/warp data. A future refactor could split map data into per-region files or generate it from a tilemap editor.

**State Machine Complexity**
GamePhase has 18+ variants, BattlePhase has 15+. Nested state transitions (e.g., PlayerAttack → Text → EnemyAttack → Text → ActionSelect) require careful phase chain construction. The `BattlePhase::Text { message, next_phase }` pattern handles this cleanly but deeply nested Box<BattlePhase> chains are hard to read.

**Warp Destination Bugs**
The #1 recurring QA issue across all sprints. Warp destinations must land on C_WALK tiles, not C_WARP tiles (the engine places the player exactly at dest coordinates; if that's a warp tile, they immediately warp back). Every map connection needs manual verification: find the destination map's collision array, compute `y * width + x`, check the tile type. Automating this check would eliminate the most common bug class.

**No Asset Pipeline**
Sprites are hardcoded as `const` arrays of palette indices in `sprites.rs` (1475 lines). This is correct for the engine's "zero runtime dependencies" philosophy but painful to author. A future tool could convert indexed PNGs to the const array format.

**Gen 2 Physical/Special Split**
In Gen 2, move category is determined by TYPE, not per-move. Physical types: Normal, Fighting, Poison, Ground, Flying, Bug, Rock, Ghost, Steel. Special types: Fire, Water, Grass, Electric, Ice, Psychic, Dragon, Dark. This caught us multiple times — Pursuit (Dark=Special), Fire Punch (Fire=Special), Acid (Poison=Physical), Sonic Boom (Normal=Physical) all needed corrections.

### Architecture Decisions Worth Noting

1. **Input helpers** (`is_confirm`, `is_up`, `held_left`, etc.): Centralized input mapping eliminated 30+ inline string comparisons. If we ever add gamepad support, only these 10 functions change.

2. **NPC `is_mart: bool` field**: Initially mart detection used `dialogue.contains("buy")` — fragile. A boolean field on NpcDef is robust and zero-cost.

3. **Camera lerp with snap**: `update_camera()` uses `CAMERA_LERP = 0.2` for smooth following, but snaps instantly on `change_map()`. This prevents the camera from "sliding in" when entering a new area.

4. **Enemy AI**: 50% random move / 50% best move (effectiveness × power). Simple but produces noticeably smarter behavior — trainers actually exploit type matchups half the time.

5. **Critical hits**: Gen 2 formula (1/16 chance, 2x damage). The `is_crit: bool` field on BattlePhase variants carries through to the text renderer for "Critical hit!" messages.

6. **PC deposit safety**: Can't deposit your last Pokemon. Simple guard but easy to forget.

7. **Shared interior maps**: PokemonCenter and GenericHouse are single map instances with dynamic exits tracked by `last_pokecenter_map` / `last_house_map`. Saves ~200 lines per city vs. unique interiors.

8. **Trainer battle system (Sprint 34)**: NPC `trainer_pokemon` field triggers battles on dialogue end. Party switching, forced switches on faint, and money rewards all work through the existing battle phase chain.

9. **Stat stages in damage calc**: Stage multipliers apply at damage calculation time, not as permanent stat changes. This means stat stage effects are automatically cleared on battle end without any cleanup code.

## File Layout

| File | Lines | Purpose |
|------|-------|---------|
| `mod.rs` | 3428 | Game logic: phases, battle, menus, overworld, camera, stat stages |
| `maps.rs` | 5024 | 29 maps with tiles, collision, NPCs, warps, encounters |
| `sprites.rs` | 1475 | Tile sprite data (indexed palette, 16x16 tiles) |
| `data.rs` | 1248 | ~42 species, ~60 moves, item DB, type chart, damage calc |
| `render.rs` | 608 | Rendering helpers: tiles, HP bars, menu boxes, text |

## Species Implemented (~42)

**Starters**: Chikorita/Bayleef, Cyndaquil/Quilava, Totodile/Croconaw
**Early routes**: Pidgey, Rattata/Raticate, Sentret, Hoothoot, Caterpie, Weedle, Ledyba, Spinarak, Mareep
**Caves/special**: Geodude, Zubat, Bellsprout, Gastly, Onix, Magikarp, Wooper, Hoppip
**Gym Pokemon**: Metapod/Butterfree, Kakuna/Beedrill, Scyther (Bugsy), Miltank/Clefairy (Whitney)
**Route 35-37**: Nidoran♀, Nidoran♂, Growlithe, Vulpix, Stantler
**Ecruteak**: Koffing, Magmar, Eevee/Vaporeon/Jolteon/Flareon/Espeon/Umbreon

## Maps Implemented (29)

**New Bark → Violet**: NewBarkTown, Route29, CherrygroveCity, Route30, Route31, VioletCity, VioletGym, SproutTower
**Interiors**: PlayerHouse1F/2F, ElmLab, PokemonCenter, GenericHouse
**Violet → Azalea**: Route32, UnionCave, Route33, AzaleaTown, AzaleaGym, IlexForest
**Azalea → Goldenrod**: Route34, GoldenrodCity, GoldenrodGym
**Goldenrod → Ecruteak**: Route35, NationalPark, Route36, Route37, EcruteakCity, BurnedTower, EcruteakGym

## Badges Implemented (4/8)

| Badge | Gym | Leader | Type |
|-------|-----|--------|------|
| Zephyr Badge | VioletGym | Falkner | Flying |
| Hive Badge | AzaleaGym | Bugsy | Bug |
| Plain Badge | GoldenrodGym | Whitney | Normal |
| Fog Badge | EcruteakGym | Morty | Ghost |

## QA Sprint History

**Sprint 33**: First QA sweep. Fixed warp destinations landing on C_WARP tiles across early maps.
**Sprint 36**: Stat stage system + accuracy/evasion modifiers.
**Sprint 39**: Comprehensive QA. Fixed building entrance warps (PokemonCenter, PlayerHouse, ElmLab, all gyms). Fixed Acid/Miltank learnset. Fixed all route connection warps.
**Sprint 42**: Move category audit (Pursuit→Special, Fire Punch→Special, Sonic Boom→Physical). Fixed IlexForest→Route34 warp landing on C_SOLID. Fixed EcruteakCity building entrance warps. Fixed accuracy check to apply stage modifiers to all moves including status.
