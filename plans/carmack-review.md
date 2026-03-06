# Carmack Review — Engine Audit Findings

Five independent reviews of `engine/crates/engine-core/src/` on branch `gravity-pong`.
Each seeded with a different obsession. Unfiltered.

---

## Carmack #1 — Performance & Memory Layout

### The HashMap ECS is a cache miss catastrophe

Every `ComponentStore<T>` is a `HashMap<Entity, T>`. When a system needs Transform AND RigidBody for the same entity, it's two separate hash probes into unrelated memory. For 36 component stores, despawning one entity is 36 hash lookups.

### `sorted_entities()` allocates and sorts every call

It's called in the physics hot path — `force_accumulator` (every physics step), `compute_substep_count` (every tick). That's a heap alloc + O(n log n) sort inside the inner physics loop, potentially 4x per frame with substeps. `compute_substep_count` doesn't even need sorted order — it's finding a max. Pure waste.

### The physics tick allocates ~15 Vecs per step

- `collision.rs` creates 5 fresh Vecs every frame
- `integrator.rs` creates 3 more, cloning `MotionConstraint` for every constrained entity
- `force_accumulator.rs` clones every `ForceField` and every `Vec<ZoneEffectKind>` — heap allocs inside heap allocs — 60+ times per second
- With 4 substeps that's 60 heap allocs per frame

### The integrator has an O(N*M) drag lookup

Lines 40-44 do `.any()` then `.find()` on the same list — two linear scans per rigidbody. Should be a single lookup.

### SpatialGrid uses HashMap for cells

For a uniform grid with known dimensions, a flat array indexed by `cy * width + cx` would be zero-cost. The HashMap hashes `(i32, i32)` pairs for no reason.

### Gravity Pong particle trails heap-allocate per particle

50 particles x separate `Vec<(f64, f64)>` allocations, pushed and drained every frame. Should be a fixed-size circular buffer.

### Top 5 fixes by impact

1. Replace `sorted_entities()` with `.iter()` where sort order doesn't matter (5-min fix)
2. Pre-allocate scratch Vecs on Engine, `.clear()` per frame instead of re-allocating (~15 allocs eliminated)
3. Fix the O(N*M) drag lookup — just call `world.continuous_drags.get(entity)` directly
4. Flat array for SpatialGrid cells
5. Fixed-size circular buffer for particle trails

---

## Carmack #2 — Physics & Numerics

### Collision restitution scales the ENTIRE velocity vector

`collision.rs:174` does `math::scale(reflected, e)` — this drains tangential momentum on angled hits. Balls feel "sticky" on glancing blows. Physically wrong. Restitution should only affect the normal component.

### No NaN/Inf guards anywhere

`force_accumulator.rs:97` divides by `rb.mass`. Mass = 0 -> Inf acceleration -> NaN position -> every distance calc that touches it is poisoned forever. InverseSquare at close range produces enormous forces even with the min_dist clamp. **First production crash.**

### Edge bounce double-reflects near world boundaries

CCD resolves a wall bounce and reflects velocity. Then `run_edge_bounce` sees the entity near the boundary and reflects velocity AGAIN. Two systems, contradictory, no coordination. Balls occasionally go through walls.

### Adaptive substeps use stale acceleration data

`compute_substep_count` reads `rb.ax`/`rb.ay` from the PREVIOUS frame. If an entity enters a strong force field, the first frame uses 1 substep, accumulates explosive velocity, and only increases substeps on the NEXT frame. One frame too late.

### Spring joints explode under substeps

Joint solver runs once per outer step with `FIXED_DT`, but physics bodies integrated with `sub_dt`. At 4 substeps, springs are effectively 4x too stiff. They will oscillate and diverge.

### The 4-bounce limit silently drops remaining motion

A ball in a tight corner exhausts all 4 bounces, keeps full velocity, gets stuck inside geometry. Next frame: t=0 overlap, arbitrary normal, vibration.

### Failure modes ranked by likelihood

1. NaN from zero mass or InverseSquare singularity
2. Edge bounce double-reflection near boundaries
3. Sticky glancing blows from full-vector restitution scaling
4. Velocity explosion from stale substep heuristic
5. Spring joint oscillation under adaptive substeps
6. Corner vibration from bounce exhaustion

---

## Carmack #3 — Architecture & Bloat

### The numbers

- **153 source files, 48,572 lines of Rust**
- **82% infrastructure, 18% game code**
- Three games: a bouncing ball demo, a pong game, and a music quiz

### 29 engine systems have ZERO consumers

FlowNetwork, EnvironmentClock, CameraDirector, LevelCurve, GameFlow, SceneManager, EntityPool, TemplateRegistry, DialogueQueue, AutoJuice, TileMap, Pathfinding, SaveLoad, DensityField, ProceduralGen, InputMap, EventBus, SpatialQuery, ColorPalette, SchemaInfo, FeelPreset, Raycast — all unused. All maintained. All compiled into the WASM binary.

### The two real games don't even use the ECS

Gravity Pong defines its own `Particle`, `GravityWell`, `Target` structs internally. It touches 10 fields out of 42 on the Engine struct. The World carries 36 component stores; Gravity Pong touches zero of them. Every frame, the engine ticks 17 systems over empty stores for nothing.

### The headless analysis framework: 28 modules for a pong game

`AblationStudy`, `HillClimber`, `DeathClassification`, `DivergenceReport`, `AnomalyDetector`, `Dashboard`, `FitnessEvaluator`, `RegressionSuite` — for a pong game. A telescope pointed at your own backyard.

### The core question

> "Is this engine serving the games, or are the games serving the engine?"

The games bypass the ECS entirely because the engine's abstractions don't fit the actual problem. The engine was built for an imagined RPG with tilemaps, pathfinding, day/night cycles, resource networks, dialogue trees, cinematic cameras, and procedural dungeons. That game doesn't exist.

### What this engine should be

~4,000 lines. A framebuffer, input, seeded RNG, sound queue, persistence, global state, screen shake, post-fx, and a `Simulation` trait. Everything else is the game's problem.

### Modules to delete

**Zero-consumer infrastructure (delete immediately):**
1. `flow_network.rs`
2. `environment_clock.rs`
3. `camera_director.rs` (738 lines, zero callers)
4. `level_curve.rs` (609 lines)
5. `game_flow.rs` (817 lines)
6. `scene_manager.rs`
7. `entity_pool.rs` (477 lines)
8. `templates.rs` (427 lines)
9. `dialogue.rs`
10. `auto_juice.rs`
11. `tilemap.rs`
12. `pathfinding.rs`
13. `save_load.rs`
14. `density_field.rs`
15. `procedural_gen.rs`
16. `input_map.rs`
17. `event_bus.rs`
18. `spatial_query.rs`
19. `color_palette.rs`
20. `schema.rs`
21. `feel_preset.rs`
22. `raycast.rs`

**Zero-consumer components:**
23. `resource_inventory.rs`
24. `graph_node.rs`
25. `visual_connection.rs`
26. `signal.rs`
27. `state_machine.rs`
28. `coroutine.rs`
29. `waypoint_path.rs`
30. `ghost_trail.rs`
31. `physics_joint.rs`
32. `zone_effect.rs`
33. `time_scale.rs`
34. `motion_constraint.rs`

**Headless framework — keep 3, delete 25:**
Keep `HeadlessRunner`, `Scenario`, `Replay`. Kill the rest.

---

## Carmack #4 — Rendering Pipeline

### Per-pixel bounds checking everywhere

`set_pixel` does 4 comparisons. `set_pixel_blended` calls `set_pixel` which does them AGAIN for opaque pixels — 8 bounds checks for one pixel. `blit_rect` checks bounds per pixel inside the inner loop. Standard fix: clip before the loop, not inside it.

### sqrt per pixel in every AA shape

`fill_circle` computes `(dx*dx + dy*dy).sqrt()` for every pixel in the bounding box. Radius-50 circle: ~10,000 sqrts. 90% can be resolved with a `dist_sq` comparison — only the 1-pixel feather band needs the actual sqrt.

### `fill_tapered_trail` is O(pixels x segments)

30-segment trail over a 200x400 bbox = 2.4 million iterations, each with a sqrt. For one trail. The "best coverage wins" heuristic produces brightness halos at joints — it selects by alpha, not by geometric distance.

### Post-processing uses f64 per-pixel math on u8 data

`apply_tint` does `chunk[0] as f64 * inv + tr * intensity` — 64-bit float ops on 8-bit values. Should be pure integer: `pixel + ((target - pixel) * intensity_u8) >> 8`.

### Screen shake clones the entire framebuffer

1.92MB allocation at 800x600, clears it, copies pixels back one at a time with per-pixel bounds checking. For a shift. Two `copy_from_slice` calls per row with no allocation would suffice.

### Vignette: 480,000 sqrts per frame

Distance from center doesn't change unless resolution changes. Precompute a lookup table.

### The rendering budget at 60fps (16.67ms)

| Resolution | Estimated Frame Time | Verdict |
|---|---|---|
| 800x600 | 8-12ms | On the edge |
| 1280x720 | 12-18ms | Over budget |
| 1920x1080 | 25-35ms | Not a chance (30fps) |

### Three fixes that buy 2x headroom

1. Clip bounding boxes once, never bounds-check inside loops
2. Replace f64 pixel math with integer math in all post-processing
3. Use `dist_sq` to reject pixels, only sqrt in the feather band

---

## Carmack #5 — Determinism & State Integrity

### Cross-run determinism is broken

Rust's `HashMap` uses randomized SipHash seeds. `collision.rs:38` collects entities via `colliders.entities().collect()` — HashMap order. Processing order determines collision resolution priority. Two runs with the same seed and inputs WILL produce different collision outcomes.

**One-line fix:** `colliders.sorted_entities()` instead of `colliders.entities().collect()`. The method already exists. The force_accumulator already uses it. The collision system doesn't.

### `powf()` and `exp()` break cross-platform determinism

The integrator uses `(1.0 - damping).powf(dt)`. The drag system uses `(-drag * dt).exp()`. These transcendental functions differ between x86_64 and WASM by 1-2 ULP. In a chaotic simulation, those ULPs compound into macroscopic divergence.

### The accumulator breaks replay if dt isn't captured

`InputFrame` doesn't include the `dt` passed to `tick()`. If replay uses different dt values, the accumulator produces different substep counts.

### State hash misses upstream causes

Covers positions and velocities (symptoms) but not acceleration, collider state, lifetimes, timers, or events (causes). Desync detection is delayed by one frame.

### What's actually safe

- RNG (xorshift64, pure integer ops) — deterministic and portable
- `sorted_entities()` sort — deterministic (stable sort on u64)
- `GameState` — accessed by key, never iterated in simulation
- Telemetry — purely observational, doesn't touch RNG or simulation state
- Gravity Pong specifically — bypasses ECS physics entirely, uses only basic arithmetic and sqrt (correctly rounded)

### Determinism verdict

| Scope | Deterministic? |
|---|---|
| Same process | Yes |
| Cross-run (same platform) | **No** — HashMap ordering |
| Cross-platform (native vs WASM) | **No** — transcendental floats |

---

## The Synthesis

Five angles, one recurring theme: **the engine is overbuilt for games that don't use it, and underbuilt where it matters.**

The games bypass the ECS. The ECS has HashMap determinism bugs. The physics has NaN traps and double-reflection. The renderer does per-pixel bounds checks and sqrt where it shouldn't. And 29 unused systems burn compile time and binary size for a pong game.

The foundational instincts are right: semi-implicit Euler, CCD for tunneling, SDF anti-aliasing, Plummer softening, command-buffer sound, seeded xorshift. The bones are solid. The problem is all the speculative muscle bolted on before the skeleton was finished.

### Priority action items

**Do today (< 1 hour):**
- [ ] Replace `sorted_entities()` with `.iter()` in `compute_substep_count` and other non-order-dependent paths
- [ ] Use `sorted_entities()` in `collision.rs:38` (determinism fix)
- [ ] Add `mass <= 0` guard in force_accumulator (NaN prevention)

**Do this week:**
- [ ] Pre-allocate scratch Vecs, stop cloning components in physics tick
- [ ] Fix restitution to only affect normal component
- [ ] Clip bounding boxes before pixel loops in shapes/framebuffer
- [ ] Replace f64 post-processing math with integer ops

**Do before the next game:**
- [ ] Delete the 22 unused infrastructure modules
- [ ] Delete the 12 unused component types
- [ ] Trim headless framework from 28 to 3 modules
- [ ] Resolve edge bounce vs CCD contradiction
- [ ] Add dt to InputFrame for replay fidelity
