# Bilbo's Senior Sprint Engineer Review: Sprint 1 Implementation Plan

**Verdict: ACCEPT WITH MODIFICATIONS**

The plan is solid, well-structured, and covers all 8 abstract tasks from the sprint. The architecture is a genuine improvement over v1. However, I have identified several issues that would cause compilation failures, borrow checker problems, or runtime bugs if implemented as-written. These must be fixed before Mary/Pippin/Sam start coding.

---

## Critical Issues (Must Fix)

### 1. Circular Dependency Between maps.rs and events.rs

**Phase**: 2 (maps.rs) and 3 (events.rs)
**Problem**: The plan acknowledges this in the "Circular Dependency Note" at the bottom but doesn't commit to a solution in the actual phase specs. maps.rs has `NpcDef.script: Vec<ScriptStep>` and `CoordEvent.script: Vec<ScriptStep>` which depend on events.rs. But events.rs needs `MapId` and `NpcState` from maps.rs. This is a hard circular `use` dependency that will not compile.

**Fix**: Adopt Option B as the plan recommends, but make it concrete:
- Move `ScriptStep`, `Emote`, `ScriptState`, `EventFlags`, `SceneState`, and all flag constants into events.rs (it remains the canonical owner).
- Move `NpcState` into overworld.rs (it's runtime state, not map definition).
- `maps.rs` imports from `events.rs` for `ScriptStep`. No reverse dependency needed.
- `events.rs` imports `MapId` from `maps.rs` only for `SetScene` — **but actually, `SetScene(u8)` in the ScriptStep enum takes only a u8, not a MapId. So events.rs does NOT need to import from maps.rs at all.** The `current_map_id` parameter to `step_script()` comes from the caller (mod.rs), not from events.rs internals.
- `NpcState` in overworld.rs. events.rs imports `NpcState` from overworld.rs.

**Import graph after fix:**
```
data.rs       <- leaf (no imports from sibling modules)
events.rs     <- imports data (Direction, SpeciesId, Emote), overworld (PlayerState, NpcState)
maps.rs       <- imports data (Direction, SpeciesId), events (ScriptStep)
overworld.rs  <- imports data (Direction), maps (MapData, is_walkable, find_warp, etc.), events (ScriptStep, EventFlags, SceneState)
render.rs     <- imports data, maps, overworld, events
dialogue.rs   <- leaf
sprites.rs    <- imports data (Direction, Emote)
mod.rs        <- imports everything
```

Wait -- there's still a cycle: events.rs imports overworld.rs (for PlayerState, NpcState), and overworld.rs imports events.rs (for EventFlags, SceneState, ScriptStep). This IS a circular dependency.

**Revised fix**: Move `PlayerState` and `NpcState` into data.rs (they're pure data structs). Or better: create a minimal `types.rs` module that holds `PlayerState`, `NpcState`, `Direction`, `Emote`, and other shared types. But that adds a module. Simplest: put PlayerState and NpcState in data.rs alongside Direction, since they're just data containers.

**Final recommended import graph:**
```
data.rs       <- leaf: Direction, Emote, SpeciesId, MoveId, Pokemon, PlayerState, NpcState, etc.
events.rs     <- imports data (Direction, SpeciesId, Emote, PlayerState, NpcState)
maps.rs       <- imports data (Direction, SpeciesId), events (ScriptStep)
overworld.rs  <- imports data (Direction, PlayerState, NpcState, CameraState), maps (*), events (EventFlags, SceneState, ScriptStep)
render.rs     <- imports data, maps, overworld (constants), events
dialogue.rs   <- leaf
sprites.rs    <- imports data (Direction, Emote)
mod.rs        <- imports everything
```

No cycles. maps.rs -> events.rs is one-way. events.rs -> data.rs is one-way.

### 2. `step_script()` Signature Has Borrow Checker Issues

**Phase**: 3 (events.rs) and 8 (mod.rs)
**Problem**: The `step_script()` function takes `&mut PlayerState`, `&mut Vec<NpcState>`, `&mut EventFlags`, `&mut SceneState`, `&mut Vec<Pokemon>`, and `&mut Vec<(u8,u8)>` — all of which are fields of `PokemonV2Sim`. In mod.rs, calling this from `step_script_phase()` requires splitting borrows:

```rust
fn step_script_phase(&mut self, engine: &Engine) {
    if let Some(ref mut script) = self.script {
        step_script(script, &mut self.player, &mut self.npc_states, ...);
    }
}
```

This actually works because `self.script` is borrowed mutably through `ref mut`, and the other fields are borrowed separately. Rust's borrow checker can handle disjoint field borrows within the same method. **This is fine as-is.** No change needed.

BUT: The `step_starter_select()` method has a problem:

```rust
fn step_starter_select(&mut self, cursor: u8, engine: &Engine) {
    // ...
    if self.phase == GamePhase::StarterSelect { .. } {  // <-- SYNTAX ERROR
        self.phase = GamePhase::StarterSelect { cursor: c };
    }
}
```

The `{ .. }` pattern in `==` is not valid Rust. Use `matches!()`:

```rust
if matches!(self.phase, GamePhase::StarterSelect { .. }) {
    self.phase = GamePhase::StarterSelect { cursor: c };
}
```

### 3. MapData Uses Mixed Static/Heap: Inconsistency

**Phase**: 2 (maps.rs)
**Problem**: The architecture doc says `tiles: &'static [u8]`, `collision: &'static [u8]`, `warps: &'static [WarpDef]` (static slices). But the implementation plan says `tiles: Vec<u8>`, `collision: Vec<u8>`, `warps: Vec<Warp>` (all Vec). The plan version is simpler and more practical for Sprint 1 — static data requires const arrays or lazy_static, which adds complexity. **Keep the plan's Vec approach.** It matches v1 and avoids premature optimization. We can optimize to static slices in a future sprint if needed.

**Decision**: Use Vec for all MapData fields as the plan specifies. This is correct for Sprint 1.

### 4. MapData width/height Type Mismatch

**Phase**: 2 (maps.rs)
**Problem**: Architecture says `width: u8, height: u8`. Plan says `width: i32, height: i32`. Player position, warp coordinates, NPC positions are all `i32` in the plan. Using `i32` for dimensions avoids constant `as i32` casts when doing `map.tiles[y * map.width + x]` indexing. **Use i32 for consistency** as the plan specifies.

### 5. `SceneState` Uses HashMap — Violates Determinism

**Phase**: 3 (events.rs)
**Problem**: `SceneState { scenes: HashMap<u16, u8> }`. Engine conventions say deterministic simulation is required. HashMap iteration order is non-deterministic. While SceneState.get() and .set() are deterministic (key lookup/insert), if we ever iterate over scenes (e.g., for serialization), it breaks determinism.

**Fix**: Use a small Vec or fixed array instead. There are only ~87 map scenes in the full game. A `Vec<(u16, u8)>` with linear search or a fixed `[u8; 128]` indexed by MapId (as u8) would be deterministic and faster.

Recommended:
```rust
pub struct SceneState {
    scenes: [u8; 128], // indexed by MapId as usize, 0 = default
}
impl SceneState {
    pub fn new() -> Self { Self { scenes: [0u8; 128] } }
    pub fn get(&self, map_id: u16) -> u8 {
        if (map_id as usize) < self.scenes.len() { self.scenes[map_id as usize] } else { 0 }
    }
    pub fn set(&mut self, map_id: u16, scene: u8) {
        if (map_id as usize) < self.scenes.len() { self.scenes[map_id as usize] = scene; }
    }
}
```

### 6. ElmsLab Dimensions Discrepancy

**Phase**: 2 (maps.rs)
**Problem**: Gimli's reference says "30 bytes, 10 wide x 3 tall" for ElmsLab.blk. The implementation plan says "10 tiles wide x 12 tiles tall" — that's 10 block-width * 2 = 20 tile-width? No, it says 10 tiles, which would be 5 blocks wide.

Actually, looking at pokecrystal's standard lab tileset: ElmsLab is width 10 (blocks), height 3 (blocks). Each block = 4x4 tiles for interior tilesets? No — standard Gen 2 blocks are 2x2 metatiles, each metatile is 2x2 tiles (8x8 pixels each). So 1 block = 4x4 tiles = 32x32 pixels.

But wait — the plan uses "tiles" as 16px units (TILE_PX = 16). In pokecrystal terms, 1 block = 2x2 tiles at 16px each = 32px. So: ElmsLab.blk = 30 bytes. If width is stored in map_attributes, we need to check that.

The implementation plan says 10x12 tiles (at 16px each), which is 5x6 blocks. 5*6 = 30 blocks = 30 bytes. **This is correct.** The plan is right: 10 tiles wide, 12 tiles tall.

However, Gimli's table says "10 wide x 3 tall" blocks. That would be 10*3 = 30 bytes but 20x6 tiles. This is contradictory. Let me trust the implementation plan's interpretation (5 blocks x 6 blocks = 10 tiles x 12 tiles) since it aligns with 30 bytes and standard lab dimensions.

**Decision**: Trust the plan's 10x12 tile interpretation. But verify: pokecrystal's `map_const ELMS_LAB, 5, 6` would mean 5 blocks wide, 6 blocks tall = 10 tiles x 12 tiles = 30 blocks. The Gimli reference table may have the width/height mixed up or be using a different interpretation. **Implementation should use 10 wide x 12 tall.**

### 7. Emote Enum Location

**Phase**: 1 (data.rs) / 3 (events.rs)
**Problem**: `Emote` is defined in maps.rs (in the plan's enums section) but used in events.rs's ScriptStep and sprites.rs. It should be in data.rs since it's a shared type.

**Fix**: Define `Emote` in data.rs alongside Direction.

---

## Minor Issues (Should Fix)

### 8. `MoveData.is_special` is Oversimplified

**Phase**: 1 (data.rs)
**Problem**: Gen 2 doesn't have a per-move physical/special split. Instead, the move's TYPE determines physical vs special (all Fire/Water/Grass/etc. moves are special; Normal/Fighting/etc. are physical). The `is_special` field on MoveData is technically redundant — you can derive it from `move_type`. However, having it pre-computed is a minor convenience and doesn't hurt. **Keep it but document that it's derived from type in Gen 2.**

### 9. Test `test_warp_bedroom_to_1f` May Be Fragile

**Phase**: 9 (tests)
**Problem**: The test tries to navigate to (7,0) by walking right then up from (6,1). But the test only runs 4 frames per direction. At 8px/frame, a full tile walk (16px) takes 2 frames. So 4 frames = 2 tiles of movement. From (6,1), walking right 1 tile reaches (7,1), then up 1 tile reaches (7,0) — but there may be collision or warp-tile interaction. The test asserts a warp occurred OR a map transition started, which is good. But the frame counts may need adjustment depending on exact collision map at the start position.

**Fix**: Use `with_state` to start closer to the warp tile, or increase frame counts to be safe. The exact collision map may block some paths.

### 10. Missing `Route29` and `Route27` MapId Handling

**Phase**: 2 (maps.rs)
**Problem**: `load_map()` needs to handle `MapId::Route29` and `MapId::Route27` since they're in the enum. These should return minimal stub maps (even a 1x1 map) to avoid panics. The plan doesn't specify this.

**Fix**: Add Route29/Route27 stub handling in `load_map()` — return minimal maps like the ElmsHouse/NeighborsHouse stubs.

### 11. `DialogueAction::StartScript(usize)` Won't Compile

**Phase**: 6 (dialogue.rs)
**Problem**: `DialogueAction` derives `Copy` but `StartScript(usize)` is fine for Copy. Actually wait — the issue is: this enum stores a `usize` but how does it reference a script? It would need the script Vec, not a usize index. For Sprint 1, dialogue is only used for standalone text (bg_events, simple NPC talk). The `StartScript` variant is dead code.

**Fix**: Remove `StartScript` variant for Sprint 1. Just have `Resume`. Re-add when needed.

### 12. Test References HeadlessRunner but Tests Use Direct Engine

**Phase**: 9 (tests)
**Problem**: The test helper section imports `HeadlessRunner` but none of the tests actually use it. All tests use `Engine::new(160, 144)` directly. Remove the unused import.

**Fix**: Remove `use crate::headless::runner::HeadlessRunner;` from test imports.

---

## Architecture Praise (What's Good)

1. **Event flag bitfield** — The `[u64; 32]` approach is excellent. Compact, deterministic, maps directly to pokecrystal.

2. **ScriptStep enum** — Declarative scripting is a massive improvement over v1's ad-hoc if chains. The program-counter model is clean.

3. **OverworldResult enum** — Returning a result from step_overworld() instead of mutating phase directly is good separation of concerns.

4. **NPC visibility via event flags** — The `event_flag: Option<u16>` + `event_flag_show: bool` pattern is elegant and matches pokecrystal exactly.

5. **Scene system** — Per-map scene IDs matching pokecrystal's `checkscene`/`setscene` is exactly right.

6. **Module split** — 8 focused modules instead of 1 monolith is the right call.

7. **Test coverage** — 15 tests covering creation, title-to-overworld, spawning, map loading, warp consistency, walking, collision, warping, coord events, script engine, NPC collision, NPC data accuracy, starter stats, event flags, and map dimensions. This is good Sprint 1 coverage.

---

## Summary of Required Modifications

| # | Phase | Change | Priority |
|---|-------|--------|----------|
| 1 | All | Resolve circular deps: put PlayerState, NpcState, CameraState, Emote in data.rs | CRITICAL |
| 2 | 8 | Fix `{ .. }` syntax error in `step_starter_select` — use `matches!()` | CRITICAL |
| 5 | 3 | Replace SceneState HashMap with `[u8; 128]` array for determinism | HIGH |
| 7 | 1 | Move Emote enum to data.rs | MEDIUM |
| 10 | 2 | Add Route29/Route27 stub maps in load_map() | MEDIUM |
| 11 | 6 | Remove DialogueAction::StartScript variant | LOW |
| 12 | 9 | Remove unused HeadlessRunner import | LOW |

I am now updating the SPRINT1_IMPLEMENTATION_PLAN.md with these modifications incorporated.
