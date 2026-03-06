# Engine Architecture

## Overview

2D deterministic simulation engine. Rust → WASM → shared-memory framebuffer → JS canvas. Games implement the `Simulation` trait; the engine owns timing, input, RNG, and determinism.

## Layer Structure

```
engine-core/
  ecs/           Entity, World, ComponentStore (HashMap-backed, no archetypes)
  systems/       17 systems, fixed execution order per tick
  components/    32 component types (Clone + Debug, no Serialize required)
  rendering/     Software framebuffer, SDF shapes, bitmap text, particles, post-fx
  physics/       Semi-implicit Euler, CCD, spatial grid broadphase
  headless/      26 modules — replay, sweep, golden tests, ablation, hill-climb

  engine.rs      Main loop, accumulator-based fixed timestep
  lib.rs         WASM bindings (thread-local ENGINE singleton)

engine-cli/      15 CLI commands for headless analysis
```

## System Execution Order

```
INPUT PHASE
  debug toggle → gesture recognition → gesture→EventBus

SIMULATION PHASE (fixed dt = 1/60s)
  lifecycle → hierarchy → signal → state_machine → coroutine
  → behavior → tween → flash → waypoint

PHYSICS PHASE (fixed dt, accumulator loop)
  force_accumulator → integrator → collision → physics_joint

POST-PHYSICS PHASE (variable dt)
  ghost_trail → particles → transition → dialogue → camera

RENDER PHASE
  clear → entities → particles → debug
  → screen_fx → transition_overlay → post_fx
  → events.clear → input.end_frame → frame_metrics
```

## ECS Design

- **Entity**: `u64` ID, monotonically increasing, never recycled. Entity(0) reserved.
- **ComponentStore<T>**: `HashMap<Entity, T>`. Independent stores, no archetype coupling.
- **World**: Owns all component stores. `component_stores!` macro generates `new/despawn/clear`.
- **Pattern**: Destructure `World` at top of system functions to avoid borrow conflicts.

Components: Transform, RigidBody, Collider, Renderable, ForceField, Tags, Role, Lifetime,
GameState, Behavior, PhysicsMaterial, Impulse, MotionConstraint, ZoneEffect, PropertyTween,
EntityFlash, GhostTrail, TimeScale, Active, WaypointPath, SignalEmitter, SignalReceiver,
Parent, Children, WorldTransform, StateMachine, Coroutine, SpriteAnimator, PhysicsJoint,
ResourceInventory, GraphNode, VisualConnection, LaunchState

## Physics

**Integration**: Semi-implicit (symplectic) Euler. Velocity updated from acceleration first, then position from new velocity during collision phase. Energy-conserving for orbital systems.

**Fixed timestep**: Accumulator-based at 60Hz. Wall-clock dt clamped to 50ms. Accumulator clamped to 5×FIXED_DT to prevent spiral-of-death.

**CCD**: Circle vs circle, line segment, and AABB. Quadratic ray-sphere intersection with Minkowski sum. Handles edge cases: zero-length segments, stationary objects, initial overlap.

**Force fields**: Plummer-softened gravity (smooth at r=0, 1/r² at large r). InverseSquare with min-distance clamping. Zone effects: Wind, Drag, SpeedMultiplier, Conveyor.

**Collision response**: Iterative bounce loop (max 4 bounces) with remaining-time tracking. Specular reflection scaled by restitution. Epsilon separation prevents surface sinking.

**Spatial grid**: Uniform grid with pre-computed inverse cell size. Rebuilt each frame. Sort+dedup for candidate filtering.

## Rendering

**Framebuffer**: Linear RGBA `Vec<u8>`. JS reads via shared WASM memory pointer. `set_pixel_blended` with alpha=0 skip and alpha=255 fast paths.

**Anti-aliasing**: SDF-based 1px feather on all shape primitives (circles, triangles, pills, thick lines, tapered trails). Pixel center offset (+0.5) for sub-pixel accuracy. Exception: `fill_rect` has no AA.

**Shape primitives**: fill_circle, draw_circle, fill_rect, draw_rect, draw_line_thick, fill_pill (stadium SDF), fill_triangle (edge-function SDF), fill_tapered_trail (multi-segment polyline, per-pixel best-segment coverage).

**Text**: 5×7 bitmap font, printable ASCII, integer scaling. No AA, no UTF-8.

**Particles**: 2048 hard cap. `retain_mut` single-pass update. Small particles (≤1.5px) render as single blended pixel.

**Post-fx**: Vignette, scanlines, screen shake (buffer clone + offset), tint, desaturate (BT.601 luminance), flash (quadratic fade). ScreenFxStack for layered composition.

## Determinism

- Single `SeededRng` (xorshift64) owned by Engine. No other RNG sources.
- `FIXED_DT = 1.0/60.0` for all simulation-phase systems.
- `Engine::state_hash()` produces deterministic u64 independent of rendering.
- `Engine::reset(seed)` for reproducible simulation.
- `InputFrame` captures canonical input for replays.
- Entity IDs monotonically assigned, never recycled.
- `BTreeMap` used throughout headless infrastructure for deterministic iteration.

## WASM / Browser Integration

**Bindings**: Flat `#[wasm_bindgen]` functions in `lib.rs`. Thread-local `ENGINE` singleton.

**Browser state**: Shared-memory `Float64Array` buffer. JS writes viewport/DPR/touch/focus/online slots; Rust reads by index. Zero-copy, zero-allocation per frame.

**Sound**: Command-buffer pattern. Rust queues `SoundCommand` variants (PlayTone, PlayNoise, PlayNote, StartLoop, etc.). JS drains as JSON each frame, dispatches to Web Audio API.

**Persistence**: Same command-buffer pattern. `PersistCommand::Store { key, value }` drained by JS, written to localStorage.

**Rendering bridge**: Two paths. Canvas 2D: `framebuffer_ptr()` → `Uint8ClampedArray` → `ImageData` → `putImageData()`. WebGL2 (chord-reps): Framebuffer uploaded as texture, processed through multi-pass bloom/composite pipeline.

## Headless Infrastructure

26 modules providing game-agnostic automated analysis:

| Module | Purpose |
|--------|---------|
| runner | HeadlessRunner with fixed/policy/capture execution modes |
| harness | Battery testing with assertions and fitness evaluation |
| replay | Record/replay deterministic playthroughs |
| golden | Regression testing via recorded baselines |
| sweep | Parameter range exploration across seed ranges |
| ablation | A/B testing mechanic contributions with delta analysis |
| hill_climb | Coordinate-descent optimizer with shrinking step sizes |
| fitness | Composable weighted scoring with letter grades |
| highlights | Statistical spike/drop/near-death/milestone detection |
| death_classify | CloseCall/Blowout/Cliff/Attrition classification |
| divergence | Frame-level determinism break detection |
| timeline | Per-frame metric capture and analysis |
| scenario | Declarative test scenarios with scheduled inputs |
| dashboard | Integrated analysis JSON output |

All modules produce serde-serializable results. `RunConfig.turbo = true` skips rendering for 10-100× throughput.

## Game Support Systems

| System | Design |
|--------|--------|
| CameraDirector | Stack-based camera modes (Follow, Pan, Zoom, Shake, Letterbox) with easing |
| AutoJuice | Declarative game feel (particles, shake, flash, sound, freeze on collision/despawn/spawn/event) |
| FeelPreset | Named physics profiles (tight_platformer, floaty_astronaut, etc.) with JSON/TOML export |
| GameFlow | Declarative state machine (Title→Playing→Paused→GameOver→Victory) with conditions |
| EventBus | One-frame typed/string event channels with entity source/target filtering |
| StateMachine | FSM component with timed transitions, one-frame edge detection (`just_entered`) |
| Coroutine | Step-based async simulation (Wait, SetState, Log, SpawnTemplate) without Rust async |
| GestureRecognizer | Tap, double-tap, long-press, swipe (direction+velocity), pinch |
| ContentDeck | Generic content pool with difficulty gating, 4 selection strategies |
| EntityPool | Acquire/release object pooling with FIFO ordering |
| SceneManager | Stack-based push/pop with full World snapshot/restore |

## Ownership Model

- Engine owns World, all subsystems, and rendering state
- Systems borrow World mutably during their phase
- No shared ownership between systems
- Thread-local ENGINE in `lib.rs` is WASM runtime shim only

## Adding Components

1. Create `components/my_component.rs`
2. Add `pub mod` + `pub use` to `components/mod.rs`
3. Add field to `component_stores!` macro in `ecs/world.rs`
4. Implement `SchemaInfo`
5. Add to `schema.rs` component list

## Adding Systems

1. Create `systems/my_system.rs` with READS/WRITES/ORDER comment
2. Add `pub mod` to `systems/mod.rs`
3. Add call in `engine.rs tick()` at correct phase position

## Not Yet (premature for current stage)

- Archetype ECS rewrite
- Job system / thread pool
- Plugin architecture
- Fully typed event bus
- Editor tooling
- Network sync layer
- GPU rendering path
