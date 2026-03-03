# The Crusty Engine — A Complete Codebase Walkthrough

This document explains the entire Crusty engine codebase for senior engineers coming from TypeScript who may be unfamiliar with Rust or game engine development. It covers what every layer does, why it's designed that way, and how the pieces connect.

## What Is This Thing?

Crusty is a deterministic 2D simulation engine written in Rust that compiles to WebAssembly. It's designed so that AI agents can build, test, and iterate on games without a human in the loop. The key properties are:

- **Deterministic**: Given the same seed and inputs, the engine produces the exact same output every time, on every platform.
- **Headless-first**: The engine can run thousands of simulations without rendering a single pixel.
- **AI-observable**: Every aspect of game state can be inspected, recorded, and compared programmatically.

Think of it as a game engine where testing and automation are first-class features, not afterthoughts.

## Rust for TypeScript Engineers

Before diving into the codebase, here are the Rust concepts you'll encounter mapped to TypeScript equivalents.

### Ownership and Borrowing

In TypeScript, every object is reference-counted and garbage-collected. In Rust, every value has exactly one owner, and when the owner goes out of scope, the value is destroyed. You can lend a value out temporarily via references:

- `&T` — a shared (read-only) reference. Like `Readonly<T>` but enforced at compile time.
- `&mut T` — an exclusive (read-write) reference. Only one `&mut` can exist at a time. This prevents data races by construction.

The borrow checker (Rust's compiler) guarantees at compile time that you never have a dangling reference or a data race. There's no garbage collector and no runtime cost.

### Structs and Enums

Rust structs are like TypeScript interfaces with concrete fields:

```
// Rust                          // TypeScript equivalent
struct Transform {               interface Transform {
    x: f64,                          x: number;
    y: f64,                          y: number;
}                                }
```

Rust enums are tagged unions — like TypeScript discriminated unions but enforced by the compiler:

```
// Rust                          // TypeScript equivalent
enum ColliderShape {             type ColliderShape =
    Circle { radius: f64 },          | { kind: 'circle'; radius: number }
    Rect { half_w: f64 },            | { kind: 'rect'; half_w: number };
}
```

The compiler forces you to handle every variant. You cannot forget one.

### Traits

Traits are Rust's version of interfaces, but they can also provide default implementations:

```
// Rust                          // TypeScript equivalent
trait Simulation {               interface Simulation {
    fn setup(&mut self, e: &mut Engine);    setup(e: Engine): void;
    fn step(&mut self, e: &mut Engine);     step(e: Engine): void;
    fn render(&self, e: &mut Engine);       render(e: Engine): void;
}                                }
```

### Generics

Work the same as TypeScript generics. `ComponentStore<T>` is like `Map<Entity, T>`.

### Macros

Rust macros are compile-time code generators. They take source code as input and produce source code as output. Think of them as sophisticated template literals that run during compilation. The codebase uses one key macro (`component_stores!`) to eliminate boilerplate — explained in detail below.

### Option and Result

Rust has no `null` or `undefined`. Instead, it uses `Option<T>` (either `Some(value)` or `None`) and `Result<T, E>` (either `Ok(value)` or `Err(error)`). The compiler forces you to handle both cases. This eliminates null pointer exceptions by design.

### cfg(test) and cfg(target_arch)

The `#[cfg(...)]` attribute is conditional compilation. `#[cfg(test)]` means "only compile this code when running tests" — test code is literally absent from production builds. `#[cfg(target_arch = "wasm32")]` means "only compile this for WebAssembly."

## Architecture Overview

The codebase is a Cargo workspace with two crates:

```
engine/
  crates/
    engine-core/     The engine library (compiles to WASM + native)
    engine-cli/      Command-line tool for headless analysis (native only)
```

Engine-core is structured in layers:

```
ECS Core          Entity, ComponentStore, World
Components        32 data types (Transform, RigidBody, Collider, ...)
Systems           21 functions that process components each frame
Physics           Vec2 math, continuous collision detection, spatial grid
Rendering         Software framebuffer, shapes, text, particles, post-fx
Platform          Input, events, sound commands, logging
Game Framework    Timers, templates, behavior rules, game flow, camera
Headless          22 modules for testing, analysis, and AI optimization
WASM Bridge       lib.rs — the thin glue between JavaScript and Rust
```

## The ECS Core

ECS stands for Entity-Component-System. It's an alternative to object-oriented inheritance that game engines use because it's faster and more flexible.

In OOP, you'd have a class hierarchy: `GameObject > PhysicsObject > Enemy`. In ECS, there are no classes. Instead:

- An **Entity** is just a numeric ID — a handle, not an object.
- **Components** are plain data structs attached to entities. An entity's "type" is defined by which components it has.
- **Systems** are functions that process entities with specific component combinations.

This means you can create an entity with `Transform + Collider` (a wall), or `Transform + RigidBody + Collider + Renderable` (a bouncing ball), or `Transform + Behavior + StateMachine` (an NPC) — all without any class hierarchy.

### Entity (ecs/entity.rs)

```
pub struct Entity(pub u64);
```

An entity is a 64-bit integer wrapped in a named type for type safety. It derives `Copy` (free to pass around — no ownership transfer), `Hash` (usable as a HashMap key), `Eq` (comparable), `Ord` (sortable for determinism), and `Serialize/Deserialize` (JSON-compatible for replays).

Entity IDs start at 1 and never get recycled. Entity(0) is reserved as a sentinel. This prevents a class of bugs where a despawned entity's old ID accidentally references a newly-spawned entity.

### ComponentStore (ecs/component_store.rs)

```
pub struct ComponentStore<T> {
    data: HashMap<Entity, T>,
}
```

A generic container mapping entities to components. One store per component type. The API is intentionally minimal: `insert`, `get`, `get_mut`, `remove`, `has`, `iter`, `iter_mut`, `len`, `clear`, plus `sorted_entities()` which returns entity IDs in sorted order for deterministic iteration (HashMap order is random).

The trait implementations are manual rather than derived. This is a deliberate Rust pattern: `impl<T: Clone> Clone for ComponentStore<T>` means "you can clone this store, but only if the component type is cloneable." A derived `Clone` would require `T: Clone` always, even when you never clone the store. Manual impls keep the type constraints minimal.

### World (ecs/world.rs)

The World struct holds all entities and all 31 component stores. It's the single source of truth for the simulation.

The key innovation here is the `component_stores!` macro. Without it, adding a new component type required editing four places: the struct definition, the constructor, the `despawn()` method, and the `clear()` method. With the macro, you add one line:

```
component_stores! {
    transforms: Transform,
    rigidbodies: RigidBody,
    colliders: Collider,
    // ... 31 total — one line each
}
```

The macro expands this single list into the struct fields, the `new()` initializations, a `despawn_components()` method that calls `.remove(entity)` on every store, and a `clear_components()` method that calls `.clear()` on every store. It's code generation at compile time — zero runtime cost.

The World also includes a NameMap for bidirectional name-to-entity lookups (like a phonebook) and a SpawnQueue for deferred entity creation.

## The Engine (engine.rs)

The Engine struct is the central orchestrator. It owns:

- The **World** (all entities and components)
- A **Framebuffer** (the pixel array that becomes the canvas)
- **Input** state (keyboard, mouse, touch)
- **30+ subsystems** (timers, templates, particles, camera, sound queue, diagnostics, etc.)
- A **SeededRng** (the single source of randomness)

### The Tick Loop

Every frame, JavaScript calls `tick(dt)` where `dt` is the wall-clock time since the last frame (typically ~16ms at 60fps). The engine then executes five phases in a fixed order:

**Phase 0 — Input**: Process debug toggles, advance gesture recognition timers, drain recognized gestures (tap, swipe, pinch) and publish them to the EventBus.

**Phase 1 — Simulation (fixed dt = 1/60s)**: Run 13 systems in strict order: lifecycle (spawn/despawn/timers/behavior rules), hierarchy (parent-child transforms), signals (emitter-receiver wiring), state machines (FSM transitions), coroutines (async step sequences), environment clock, flow network, sprite animation, behavior AI (chase/flee/orbit), tweens (easing curves), flash effects, and waypoint path-following.

**Phase 2 — Physics (fixed dt, 0-N iterations)**: Uses a physics accumulator to decouple physics from frame rate. Each iteration runs: force accumulation, semi-implicit Euler integration, collision detection (broadphase spatial grid + narrowphase CCD), and joint constraints.

**Phase 3 — PostPhysics**: Ghost trail capture, particle updates, scene transitions, dialogue timers, and camera follow/smooth/clamp.

**Phase 4 — RenderingPrep**: Clear framebuffer, draw entities by layer, draw particles, draw debug overlays, apply screen effects (tint, flash, desaturate), apply transitions, apply post-processing (CRT, scanlines), then clean up (clear events, reset per-frame input, advance counters).

### Fixed Timestep (Why It Matters)

The physics accumulator is a pattern that prevents a subtle but devastating bug. If you just multiply velocity by `dt`, then on a slow frame (33ms) the ball moves twice as far as on a fast frame (16ms). Worse, it might skip past a wall entirely. The accumulator ensures physics always advances in fixed 1/60s steps, regardless of frame rate.

### Determinism

`Engine::reset(seed)` is the single entry point for reproducible simulation. It clears all world state, resets all subsystems, and reseeds the RNG. `Engine::state_hash()` computes a deterministic FNV-1a hash of the simulation state (transforms, rigidbodies, global state, frame counter, RNG state) — deliberately excluding rendering state so the hash reflects simulation truth only. Two runs with the same seed and inputs produce the same hash on every platform.

## The Simulation Trait

Games implement the `Simulation` trait to plug into the engine:

```
pub trait Simulation {
    fn setup(&mut self, engine: &mut Engine);   // Initialize game state
    fn step(&mut self, engine: &mut Engine);    // Advance one frame of logic
    fn render(&self, engine: &mut Engine);      // Draw into framebuffer
    fn variants(&self) -> Vec<ParamSet>;        // Declare tuning variants
}
```

The engine owns timing, input application, and determinism. The game provides the three hooks. `render` is separated from `step` so that turbo mode (headless, no rendering) can skip it entirely while still advancing the simulation.

`DemoBall` is the reference implementation — a bouncing ball demo that implements `Simulation` in about 300 lines.

## Physics

### Vec2 Math (physics/math.rs)

Vectors are represented as `(f64, f64)` tuples, not a custom struct. Functions like `add`, `sub`, `scale`, `dot`, `length`, `normalize`, `perpendicular`, and `rotate` operate on these tuples. This is intentionally simple — no operator overloading, no methods, just functions. Everything uses `f64` (never `f32`) for precision consistency across platforms.

### Continuous Collision Detection (physics/ccd.rs)

Standard collision detection checks if objects overlap each frame. But a fast-moving bullet can pass straight through a thin wall between frames — the "tunneling" problem. CCD solves this by sweeping the moving circle's path and finding the exact moment of first contact.

Three sweep functions handle all cases: circle-vs-circle, circle-vs-line-segment (used for rectangle edges), and circle-vs-AABB (axis-aligned bounding box, which decomposes into 4 edges + 4 corners).

### Spatial Grid (physics/spatial_grid.rs)

Checking every entity against every other entity for collisions is O(n^2). The spatial grid divides the world into cells and only checks entities in the same or neighboring cells. This is the broadphase — it quickly eliminates pairs that can't possibly collide before the expensive narrowphase CCD runs.

## Rendering

### Framebuffer (rendering/framebuffer.rs)

There is no GPU. The engine renders directly to a pixel array in memory — a flat `Vec<u8>` of RGBA values (4 bytes per pixel). The `ptr()` method exposes the raw memory address so JavaScript can create an `ImageData` from it and blit it to a canvas. This is the WASM shared memory bridge: Rust writes pixels, JavaScript reads them, zero copies.

### Software Rendering

The rendering module implements everything from scratch: filled and stroked rectangles, circles (Bresenham's algorithm), lines (with thickness), text (using a built-in bitmap font), particles, sprites from sprite sheets, a layer system for depth sorting, screen-wide effects (tint, flash, desaturate), scene transition overlays (fade, wipe), and post-processing (CRT scanlines, chromatic aberration, vignette).

All drawing functions operate on the Framebuffer's pixel array with manual alpha blending. There's no WebGL, no Canvas 2D API — just direct pixel manipulation. This makes the rendering entirely deterministic and platform-independent.

## Components (32 Types)

Components are plain data structs. Here are the key ones:

**Transform** — Position (x, y). Rotation and scale fields exist but are reserved for v2.

**RigidBody** — Physics state: mass, velocity (vx, vy), acceleration (ax, ay), restitution (bounciness), damping, and a `is_static` flag for immovable objects.

**Collider** — Shape (Circle or Rect) plus an `is_trigger` flag. Triggers detect overlaps without physics response — useful for pickup zones and event areas.

**Renderable** — Visual representation: Circle, Rect, Line, or Sprite, plus layer depth and visibility.

**Lifetime** — Auto-despawn countdown. Tracks `duration` and `remaining`, provides `fraction_elapsed()` for fade effects.

**StateMachine** — Finite state machine with condition-based transitions. Conditions include `After(seconds)`, `OnSignal(channel)`, `StateCheck { key, op, value }`, and `Always`. Tracks current state, elapsed time in state, and `just_entered` flag for one-shot enter logic.

**Coroutine** — Sequential async behavior as a step queue. Steps include `WaitSeconds`, `WaitSignal`, `WaitUntil { key, op, value }`, `SetState`, `AddState`, `SpawnTemplate`, and `Log`. Non-wait steps cascade in a single frame; wait steps block until satisfied. This is like an async function expressed as data.

**Behavior** — Autonomous movement modes: Drift, Chase, Flee, Seek, Orbit. Each mode takes parameters (speed, target tag, orbit radius) and the behavior system updates velocity accordingly.

**SignalEmitter / SignalReceiver** — Cross-entity event channels. An emitter broadcasts on a named channel; receivers listen. The signal system wires them together each frame.

**Parent / Children / WorldTransform** — Scene hierarchy. The hierarchy system propagates parent transforms to children, computing world-space positions.

**PropertyTween** — Easing-curve animation for numeric properties. Supports Linear, EaseIn, EaseOut, EaseInOut, Bounce, and Elastic easing functions.

Other components: Tags, Role, GameState (per-entity key-value), PhysicsMaterial, Impulse, MotionConstraint, ZoneEffect, EntityFlash, GhostTrail, TimeScale, Active, WaypointPath, SpriteAnimator, PhysicsJoint, ResourceInventory, GraphNode, VisualConnection.

## Systems (21 Functions)

Systems are stateless functions that process components. Each system declares what it reads and writes, runs in a fixed position in the tick order, and operates on the World by iterating component stores.

The execution order matters. For example, `lifecycle` runs before `hierarchy` because newly-spawned entities need to exist before parent-child transforms are propagated. `force_accumulator` runs before `integrator` because forces must be summed before velocity is updated. This ordering is locked down in the `SystemPhase` enum and documented extensively.

Key systems:

**lifecycle** — Processes the spawn queue, ticks Lifetime components (despawning expired entities), ticks timers, and evaluates behavior rules (condition-action pairs).

**collision** — Builds a spatial grid, runs broadphase to find candidate pairs, runs CCD narrowphase, resolves collisions (reflect velocity, separate positions), and emits collision events.

**integrator** — Semi-implicit Euler: `velocity += acceleration * dt`, then `position += velocity * dt`. Simple, stable, fast.

**renderer** — Sorts entities by Renderable layer, converts world coordinates to screen coordinates via the camera, and draws each entity's visual into the framebuffer.

**state_machine** — Ticks elapsed time, checks transition conditions in priority order, fires the first matching transition. Tracks `just_entered` for one-frame enter logic.

**coroutine** — Pops steps from the queue. Wait steps block; action steps (SetState, SpawnTemplate) execute and cascade to the next step in the same frame.

## Input and Events

### Input (input.rs)

Tracks keyboard state (`keys_held`, `keys_pressed`, `keys_released` per frame), mouse position, mouse button state, and drag detection. `end_frame()` clears the per-frame sets at the end of each tick.

### InputFrame (input_frame.rs)

A serializable snapshot of one frame's input: `keys_held`, `keys_pressed`, `keys_released`, `pointer` position, `pointer_down`, `pointer_up`. This is the canonical format for replays and policy-driven simulation.

### EventQueue (events.rs)

Frame-local events emitted by systems (e.g., `Collision { entity_a, entity_b }`). Cleared every frame. Used by the lifecycle system to trigger behavior rules.

### EventBus (event_bus.rs)

A named-channel publish-subscribe system. Events carry typed payloads (`f64`, `bool`, `str`, `entity`). Systems publish events; other systems and the behavior rule engine subscribe. This decouples systems that need to communicate without direct references.

## Game Framework

### Timers (timers.rs)

One-shot and repeating timers. Ticked each frame; fires are detected by the lifecycle system and can trigger behavior rules.

### Templates (templates.rs)

Entity blueprints. A template defines which components an entity should have (position, collider shape, visual style, etc.) with 23 optional fields. `TemplateRegistry` stores named templates; `spawn()` instantiates them.

### BehaviorRules (behavior.rs)

A declarative rule engine. Rules have a condition (collision, trigger enter, timer fired, state check, key pressed, always) and a list of actions (despawn, set state, spawn template, start timer, log). Rules are evaluated each frame; the first matching rule fires. One-shot rules disable after firing.

### GameFlow (game_flow.rs)

Top-level game state machine: Title, Playing, Paused, GameOver, Victory, Custom. Transitions are condition-driven (state reaches threshold, event fires, button tapped, timer expires). Pausing freezes elapsed time and condition evaluation.

### CameraDirector (camera_director.rs)

Stack-based camera control: Follow (track an entity by tag), Pan (smooth move to a point), Zoom (scale with easing), Shake (procedural screen shake), Letterbox (cinematic bars). Effects layer on top of each other and auto-pop after their duration.

### AutoJuice (auto_juice.rs)

One-stop game feel system. Register trigger-effect pairs: "on collision between ball and wall, play particles + screen shake + sound." Triggers fire automatically; you define them once.

### Sound (sound.rs)

Command buffer pattern. Rust enqueues sound commands (PlayTone, PlayNoise, StartLoop, StopLoop, SetVolume); JavaScript drains them as JSON each frame and executes them via Web Audio. The engine never touches audio hardware directly.

## Headless Testing Infrastructure

This is the most distinctive part of the engine — 22 modules for running simulations without a display.

### HeadlessRunner (headless/runner.rs)

Runs a Simulation with a given seed and input sequence, returns a `SimResult` containing: frames run, final metrics, game state snapshot, framebuffer hash, state hash, and optional per-frame state hashes. Two modes: replay-driven (fixed input sequence) and policy-driven (an AI decides each frame's input).

### Policy Trait (policy.rs)

The AI interface. A Policy receives an `Observation` (read-only view of the engine state) and returns an `InputFrame` (what buttons to press). Built-in policies: `NullPolicy` (no input), `RandomPolicy` (random keys from a given set, seeded for determinism), `ReplayPolicy` (replays a recorded input sequence).

### PlaythroughFile (headless/playthrough.rs)

Records a complete simulation run: seed, inputs, frame count, final state hash, optional per-frame hashes. Can be serialized to JSON, stored on disk, and verified later. This is snapshot testing for game runs.

### Parameter Sweeps (headless/sweep.rs)

Vary game state parameters (gravity, friction, speed) across multiple configurations, run each, compare outcomes. Used for tuning and sensitivity analysis.

### FitnessEvaluator (headless/fitness.rs)

Score simulation outcomes against weighted criteria. Produces grades (A+, A, B, C, D, F). Used by the hill climber and the dashboard.

### GoldenTest (headless/golden.rs)

Record a "golden" baseline of correct behavior. After code changes, replay and compare against the golden. If the hashes diverge, the test fails. This catches unintended behavior regressions.

### HillClimber (headless/hill_climb.rs)

Automated parameter optimization. Defines ranges for game parameters, runs simulations, evaluates fitness, and hill-climbs toward better outcomes.

### Other Headless Modules

- **Anomaly Detection** — Flags statistical outliers across batches of runs.
- **Death Classification** — Categorizes how entities die (timeout, collision, out-of-bounds).
- **Divergence Analysis** — Compares two runs frame-by-frame to find exactly when they diverge.
- **Ablation Study** — Disables game mechanics one at a time to measure their individual impact.
- **Highlight Detection** — Finds "interesting moments" (score spikes, near-deaths, combos).
- **Variant Runner** — Sweeps across parameter variants declared by the Simulation trait.
- **Dashboard Data** — Generates JSON for the web dashboard visualization.

## The WASM Bridge (lib.rs)

`lib.rs` is the thin glue layer between JavaScript and Rust. It uses `thread_local!` to store the Engine and Simulation instances (WASM is single-threaded), and exposes `#[wasm_bindgen]` functions that JavaScript calls:

- `init(width, height)` — Create the engine.
- `setup_demo_ball()` — Initialize the demo game.
- `tick(dt_ms)` — Advance one frame.
- `framebuffer_ptr()` / `framebuffer_len()` — Get the raw pixel array for `ImageData`.
- `key_down(code)` / `key_up(code)` / `mouse_move(x, y)` / `mouse_down(x, y, btn)` / `mouse_up(x, y, btn)` — Forward input events.
- `touch_start(id, x, y)` / `touch_move(id, x, y)` / `touch_end(id, x, y)` — Forward touch events (routed through gesture recognition).
- `get_game_state()` / `set_game_state_f64(key, value)` — Read/write global state as JSON.
- `spawn_template(name, x, y)` — Spawn an entity from a template.
- `drain_sound_commands()` — Get queued sound commands as JSON.
- `get_diagnostics()` / `get_frame_metrics()` — Performance telemetry as JSON.
- Timer management functions.

The pattern is: JavaScript owns the animation loop and the DOM. Rust owns the simulation and rendering. They communicate through shared WASM memory (the framebuffer) and JSON strings (sound, diagnostics, game state).

## The CLI (engine-cli)

A native binary with 15 commands for headless analysis. All output is JSON/JSONL for piping into analysis tools:

- `schema` — Dump the engine's type schema.
- `record` / `replay` — Record and verify playthroughs.
- `batch` — Run simulations across a seed range.
- `sweep` — Parameter sweep with policy-driven input.
- `golden` — Golden baseline recording and verification.
- `deaths` — Classify terminal states.
- `divergence` — Find where two runs diverge.
- `ablation` — Mechanic ablation study.
- `variants` / `variant-sweep` — Variant declaration and sweeping.
- `highlights` — Detect interesting moments.
- `dashboard-data` — Generate visualization JSON.
- `preset` — Manage physics feel presets (loaded from TOML).
- `info` — Print engine information.

## Build and Deploy

### Compilation

The engine compiles to two targets: WASM (for the browser) via `wasm-pack build --target web`, and native (for the CLI) via `cargo build`. Dependencies are minimal: `wasm-bindgen`, `serde`, `serde_json`, optional `toml` for presets, and WASM-only `console_error_panic_hook` and `web-sys`.

### Site Build

`scripts/build-docs.sh` converts `docs/*.md` to HTML using a Python-based markdown converter that produces dark-themed pages with navigation. `build-site.sh` assembles the final `_site/` directory: copies site pages, WASM artifacts, and generated docs. A SHA256 hash of the WASM binary is stamped into HTML for cache busting.

### CI/CD

GitHub Actions (`.github/workflows/deploy.yml`) triggers on pushes to `main` or `claude/**` branches. It installs Rust, builds WASM, generates HTML docs, assembles the site, and deploys to GitHub Pages.

## Why This Design?

**Why ECS instead of OOP?** Cache-friendly data layout, easy composition (no diamond inheritance), and systems can be trivially parallelized later since they operate on disjoint data.

**Why a fixed timestep?** Reproducibility. Variable timesteps make simulations non-deterministic — the same game plays differently at 30fps vs 60fps. Fixed timesteps guarantee identical behavior.

**Why software rendering?** Determinism again. GPU rendering varies by driver and hardware. A CPU framebuffer produces identical pixels everywhere. It also means headless runs need no display server.

**Why command buffers for sound?** The engine runs in WASM where there's no direct audio hardware access. Queueing commands as JSON and letting JavaScript execute them via Web Audio is the cleanest cross-boundary pattern.

**Why a single RNG?** One canonical `SeededRng` (xorshift64) owned by the Engine means every random decision is reproducible from the seed. No thread-local RNGs, no OS entropy, no non-determinism.

**Why no unsafe code?** The codebase has zero `unsafe` blocks. Rust's safe abstractions (bounds-checked array access, reference counting, the borrow checker) provide sufficient performance for a 2D simulation engine. Avoiding `unsafe` means the entire codebase benefits from Rust's memory safety guarantees.
