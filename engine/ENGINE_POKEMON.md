# ENGINE_POKEMON.md — Engine Notes from 30 Sprints of Pokemon Development

## Overview

Pokemon Gold/Silver/Crystal recreation built on the Crusty engine. ~8700 lines of Rust across 5 files, implementing 23 species, 14 maps, turn-based battles, and a full overworld. All rendering is Rust-side framebuffer at GBC resolution (160x144), with a JavaScript overlay canvas for battle sprites loaded from the web.

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

### Pain Points & Lessons

**File Size**
`mod.rs` hit ~2900 lines and `maps.rs` ~2800 lines. Both are at the edge of comfortable navigation. A future refactor could split battle logic into `battle.rs`, menu logic into `menus.rs`, and map data into per-map files. The current monolith works but makes targeted edits slower.

**State Machine Complexity**
GamePhase has 18 variants, BattlePhase has 15. Nested state transitions (e.g., PlayerAttack → Text → EnemyAttack → Text → ActionSelect) require careful phase chain construction. The `BattlePhase::Text { message, next_phase }` pattern handles this cleanly but deeply nested Box<BattlePhase> chains are hard to read.

**No Asset Pipeline**
Sprites are hardcoded as `const` arrays of palette indices in `sprites.rs` (1475 lines). This is correct for the engine's "zero runtime dependencies" philosophy but painful to author. A future tool could convert indexed PNGs to the const array format.

**Paralysis Check Duplication**
The paralysis immobility check appears in two places (player can't move, enemy can't move) with identical logic. Extracted to a constant (`PARALYSIS_SKIP_CHANCE`) but the check pattern itself is still duplicated. Could be a method on `Pokemon`.

### Architecture Decisions Worth Noting

1. **Input helpers** (`is_confirm`, `is_up`, `held_left`, etc.): Centralized input mapping eliminated 30+ inline string comparisons. If we ever add gamepad support, only these 10 functions change.

2. **NPC `is_mart: bool` field**: Initially mart detection used `dialogue.contains("buy")` — fragile. A boolean field on NpcDef is robust and zero-cost.

3. **Camera lerp with snap**: `update_camera()` uses `CAMERA_LERP = 0.2` for smooth following, but snaps instantly on `change_map()`. This prevents the camera from "sliding in" when entering a new area.

4. **Enemy AI**: 50% random move / 50% best move (effectiveness × power). Simple but produces noticeably smarter behavior — trainers actually exploit type matchups half the time.

5. **Critical hits**: Gen 2 formula (1/16 chance, 2x damage). The `is_crit: bool` field on BattlePhase variants carries through to the text renderer for "Critical hit!" messages.

6. **PC deposit safety**: Can't deposit your last Pokemon. Simple guard but easy to forget.

## File Layout

| File | Lines | Purpose |
|------|-------|---------|
| `mod.rs` | 2965 | Game logic: phases, battle, menus, overworld, camera |
| `maps.rs` | 2811 | 14 maps with tiles, collision, NPCs, warps, encounters |
| `sprites.rs` | 1475 | Tile sprite data (indexed palette, 16x16 tiles) |
| `data.rs` | 838 | Species DB, move DB, item DB, type chart, damage calc |
| `render.rs` | 608 | Rendering helpers: tiles, HP bars, menu boxes, text |

## Species Implemented (23)

Chikorita/Bayleef, Cyndaquil/Quilava, Totodile/Croconaw, Pidgey, Rattata, Sentret, Hoothoot, Caterpie, Weedle, Geodude, Zubat, Bellsprout, Gastly, Onix, Magikarp, Ledyba, Spinarak, Mareep, Wooper, Hoppip

## Maps Implemented (14)

NewBarkTown, Route29, CherrygroveCity, Route30, Route31, VioletCity, VioletGym, SproutTower, PlayerHouse1F/2F, ElmLab, PokemonCenter, Route32, UnionCave
