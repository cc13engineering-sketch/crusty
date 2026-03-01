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

### 1. Compilation Error — Private Module (Severity: Critical)
`engine-core/src/lib.rs:8` declares `mod scripting;` (private), but `engine-cli/src/main.rs:14,23,25` access it. The CLI crate does not compile.

### 2. Debug Toggle Fires Every Frame (Severity: Medium)
`engine.rs:77` — `self.input.keys_pressed.contains("KeyD")` toggles debug mode, but `keys_pressed` is only cleared in `end_frame()` which runs at the end of `tick()`. Since the physics loop runs multiple sub-steps before `end_frame()`, the toggle fires once per frame, not once per sub-step — so this is actually fine per-frame. However, if the user holds 'D', `keys_pressed` won't re-fire (good), but there's a subtlety: `keys_pressed` uses the raw event code string `"KeyD"` while `key_down` receives `e.code` from JS. This works, but if a different keyboard layout maps 'D' elsewhere, the code could be different. Consider documenting this as a known limitation.

### 3. CCD Unused `denom` Variable (Severity: Low)
`ccd.rs:77` — `let denom = dot(move_d, seg_n);` is computed but never used. This suggests an incomplete optimization (early-out when the circle moves parallel to the line). Either use it or remove it.

### 4. World `despawn()` Doesn't Remove from NameMap (Severity: Medium)
`world.rs:81-89` — `despawn()` removes the entity from all component stores and `alive`, but doesn't remove it from `names`. A despawned entity's name will persist and could be returned by `get_by_name()`, pointing to a dead entity.

### 5. Collision: Rect-vs-Rect Not Supported (Severity: Low, Documented)
Only circle movers support CCD. Non-circle movers log a warning and are skipped. This is documented, but means rect-on-rect collision doesn't exist — moving rect entities will pass through everything.

### 6. `sweep_circle_vs_aabb` Corner Tests Are Redundant (Severity: Negligible)
`ccd.rs:148-162` — The 4 corner point tests are already covered by `sweep_circle_vs_line_segment` which tests both endpoints of each segment. This doubles the corner work but won't cause incorrect results — just wastes ~8 extra sweep tests per AABB.

---

## Warnings (from `cargo check`)

| File | Warning | Suggested Fix |
|------|---------|---------------|
| `ecs/mod.rs:6` | Unused import `ComponentStore` | Remove or use in a public API |
| `ecs/mod.rs:7` | Unused import `NameMap` | Remove or use in a public API |
| `scripting/loader.rs:5` | Unused import `EntityDef` | Remove from import |
| `systems/collision.rs:32` | Unused variable `tags` | Change to `tags: _` |
| `physics/ccd.rs:77` | Unused variable `denom` | Prefix with `_` or remove |
| `physics/math.rs:10-11,21` | Dead code: `distance`, `distance_sq`, `clamp_f64` | Keep (utility functions for future use) or gate with `#[allow(dead_code)]` |
| `scripting/parser.rs:35,38,40` | Never-read fields in `Value::Str`, `Value::Vec2`, `Value::Array` | These are parsed but not yet consumed by the loader — expected for a v1 where not all value types are used yet |

---

## Performance Observations

1. **O(n^2) collision detection** (`collision.rs:97`) — Every moving entity checks against every other entity every sub-step. Fine for <100 entities, will need spatial partitioning (grid/quadtree) for more.

2. **`sorted_entities()` allocates every call** (`component_store.rs:63`) — Collects keys into a Vec and sorts. Called multiple times per physics step. For determinism this is necessary with HashMap, but consider switching to a `BTreeMap` or caching the sorted order.

3. **`Visual::clone()` in renderer** (`renderer.rs:25`) — Every visible entity clones its `Visual` enum into the drawables list every frame. Since `Visual` contains `Color` (which is `Copy`), the clone is cheap, but collecting into a Vec and sorting by layer every frame could be avoided with a dirty flag or layer-bucketed storage.

4. **Thick line drawing** (`shapes.rs:113-150`) — Iterates every pixel in the bounding box. For long, thin lines this is mostly wasted iteration. A scanline approach would be faster.

5. **Alpha blending** (`framebuffer.rs:50-58`) — Uses integer division (`/ 255`). The standard trick `(x + 128) / 255` or `(x * 257 + 256) >> 16` gives more accurate rounding. Current approach is fine for a game though.

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
