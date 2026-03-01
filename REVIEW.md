# Crusty Game Engine - Code Review

## Overall Impression

This is a well-structured 2D physics game engine prototype with a clean Rust→WASM→Canvas pipeline. The architecture is thoughtful — fixed-timestep physics with CCD, a simple but functional ECS, a custom DSL for world definitions, and a shared-memory framebuffer approach that avoids serialization overhead. For a v1 prototype, this is solid work.

---

## Build Status: BROKEN

The CLI crate fails to compile due to a visibility error.

**`engine-core/src/lib.rs:8`** — `mod scripting` is private, but `engine-cli/src/main.rs` accesses `engine_core::scripting::parser::parse_world` and `engine_core::scripting::loader::load_world_file` directly. Fix: change `mod scripting;` to `pub mod scripting;` in lib.rs. There are also 11 compiler warnings (unused imports, unused variables, dead code) — see the Warnings section below.

---

## Architecture

**Strengths:**
- Clean separation: ECS core, components, systems, rendering, physics, scripting are all isolated modules
- Fixed-timestep physics loop with accumulator capping (`5.0 * FIXED_DT`) prevents spiral-of-death
- CCD (continuous collision detection) for circles — rare to see in a prototype and prevents tunneling
- Snapshot-then-commit pattern in collision.rs avoids borrow conflicts and makes the physics step deterministic
- `thread_local!` singleton for WASM is pragmatic and correct for single-threaded WASM
- The `.world` DSL with pest grammar is a nice touch — declarative scene definition with zero boilerplate
- Dual-target logging (`log.rs`) keeps the crate clean for both WASM and native
- The CLI's `test` command that simulates 60 ticks and checks for NaN/escape is a good smoke test pattern

**Concerns:**
- No unit tests anywhere in the project. The CLI smoke test is useful but doesn't replace component-level tests
- The `World` struct uses `pub` fields for all component stores, which means any code can reach in and mutate anything — fine for a prototype, but will become a maintenance burden

---

## Bugs & Correctness Issues

### 1. ~~Compilation Error — Private Module (Severity: Critical)~~ FIXED
~~`engine-core/src/lib.rs:8` declares `mod scripting;` (private).~~ Now `pub mod scripting;`.

### 2. Debug Toggle Fires Every Frame (Severity: Medium) — KNOWN LIMITATION
`engine.rs` uses raw `"KeyD"` event code for debug toggle. Works on standard QWERTY layouts but may not map correctly on others. Documented as known limitation.

### 3. CCD Unused `denom` Variable (Severity: Low) — SUPPRESSED
`ccd.rs:77` — Renamed to `_denom` to suppress warning. Underlying early-out optimization still not implemented.

### 4. ~~World `despawn()` Doesn't Remove from NameMap (Severity: Medium)~~ FIXED
~~Missing NameMap cleanup.~~ `world.rs` now calls `self.names.remove_entity(entity)` in `despawn()`. Test added.

### 5. Collision: Rect-vs-Rect Not Supported (Severity: Low, Documented) — STILL OPEN
Only circle movers support CCD. Rect-on-rect collision still not implemented.

### 6. `sweep_circle_vs_aabb` Corner Tests Are Redundant (Severity: Negligible) — STILL OPEN
Redundant corner tests remain. No functional impact.

---

## Warnings (from `cargo check`) — Updated Status

Most original warnings have been fixed. Current state:

| File | Warning | Status |
|------|---------|--------|
| `ecs/mod.rs:6-7` | Unused imports `ComponentStore`, `NameMap` | **FIXED** — removed |
| `scripting/loader.rs:5` | Unused import `EntityDef` | **FIXED** |
| `systems/collision.rs:32` | Unused variable `tags` | **FIXED** |
| `physics/ccd.rs:77` | Unused variable `denom` | **SUPPRESSED** — renamed to `_denom` |
| `physics/math.rs:10-11,21` | Dead code: utility functions | **SUPPRESSED** — `#![allow(dead_code)]` at module level |
| `scripting/parser.rs:35,38,40` | Never-read fields in `Value` variants | **SUPPRESSED** — `#[allow(dead_code)]` on enum |
| `systems/renderer.rs:26` | Unused function `run()` | **NEW** — `run_entities_only()` replaced it; `run()` is dead code |

---

## Performance Observations — Updated Status

1. ~~**O(n^2) collision detection**~~ — **FIXED** in Round 4. Collision system now uses `SpatialGrid` for broad-phase spatial partitioning.

2. **`sorted_entities()` allocates every call** (`component_store.rs`) — STILL OPEN. Still collects keys into Vec and sorts each call.

3. **`Visual::clone()` in renderer** — STILL OPEN. Collect-sort-draw pattern remains.

4. **Thick line drawing** (`shapes.rs`) — STILL OPEN. Bounding-box iteration approach unchanged.

5. **Alpha blending** (`framebuffer.rs`) — STILL OPEN. Uses simple integer division.

---

## Design Suggestions

1. **Add `#[allow(dead_code)]` on math utilities** — `distance`, `distance_sq`, and `clamp_f64` are clearly useful utility functions. Suppress the warnings rather than removing them.

2. **Consider a `PhysicsConfig` struct** — The "feel" system (`loader.rs:62-71`) is a creative idea. It would be even more powerful if exposed at runtime (not just load-time) so feel could change per-level or per-zone.

3. **The `Visual::Line` rendering has a camera bug** — In `renderer.rs:73-74`, `x2`/`y2` from the `Visual::Line` variant are treated as world coordinates and passed through `camera.world_to_screen()`, but the line's start point (`*x`/`*y`) has already been converted to screen space. If `x2`/`y2` are meant to be world-space endpoints, the start should also use world coords. If they're offsets from the entity position, they shouldn't go through the camera transform.

4. **Entity ID recycling** — `World.next_id` only ever increments. In a long-running game with lots of spawn/despawn, IDs will grow unboundedly. A free list would allow ID reuse, though u64 overflow is practically unreachable.

5. **The web frontend hardcodes 960x540** — Both `index.html` and `game.js` hardcode dimensions. Consider reading them from the world config or making the canvas responsive.

---

## Summary

| Category | Rating |
|----------|--------|
| Architecture | Strong — clean module boundaries, good separation of concerns |
| Correctness | Good — one build-breaking bug, one minor despawn bug, CCD math is solid |
| Code Quality | Good — consistent style, follows its own conventions, good documentation in CLAUDE.md |
| Performance | Adequate for prototype — known O(n^2) collision, no major algorithmic issues |
| Completeness | v1-appropriate — rect movers, friction, rotation are marked RESERVED |

The foundation is solid. Fix the `mod scripting` visibility, the despawn/NameMap gap, and the `Visual::Line` camera issue, and this is ready for iteration.
