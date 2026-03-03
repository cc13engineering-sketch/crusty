# Architecture

## Overview
2D deterministic simulation engine. Rust → WASM → shared memory framebuffer → JS canvas. Games implement the `Simulation` trait.

## Executive Intent
Crusty is evolving into a:
- General-purpose game engine core
- Optimized for deterministic simulation
- Designed for AI-assisted iterative development
- With future parallelism in mind

## Guiding Principles (Invariants)
- Deterministic simulation by default
- No hidden global mutable state
- Hot paths avoid heap allocation
- Engine should not become a god object
- ECS boundaries must remain clean
- Systems written to be parallelizable later
- AI observability must not pollute runtime hot paths

## Layer Structure
```
engine-core
  ├─ ecs           (Entity, World, ComponentStore)
  ├─ systems       (21 system modules, ordered execution)
  ├─ components    (32 component types)
  ├─ rendering     (framebuffer, shapes, text, particles, post-fx)
  ├─ physics       (math, CCD, spatial grid)
  └─ platform      (input, events, sound, diagnostics)

engine-runtime (WASM/native glue — lib.rs thread-local)
engine-tools   (headless testing, AI instrumentation)
```

## Threading Stance
- Currently **single-threaded**
- All systems must be **parallel-safe in design** (no hidden shared state)
- Rendering assumed **main-thread only**
- No thread-local global state in engine-core (thread-local ENGINE is runtime shim only)
- The `ENGINE` thread-local in `lib.rs` is a WASM runtime shim, not core engine state

## Data Flow
```
Input (JS → WASM) → Systems → Framebuffer (WASM memory → JS ImageData → Canvas)
Sound commands queued in Rust → drained as JSON by JS
Diagnostics/metrics → drained as JSON by JS
```

## Frame Lifecycle
```
1. Input arrives via WASM bindings (key_down, mouse_move, etc.)
2. tick(dt) called from JS requestAnimationFrame
3. Systems execute in phase order (see below)
4. Framebuffer pixels ready for JS to blit
5. Sound/diagnostic queues ready for JS to drain
```

## System Execution Order (per tick)
See `SystemPhase` enum in engine.rs for authoritative documentation.
```
Input:
  debug toggle, gesture recognition, gesture→EventBus

Simulation (fixed dt):
  lifecycle → hierarchy → signal → state_machine → coroutine
  → environment_clock → flow_network → sprite_animator
  → behavior → tween → flash → waypoint

Physics (fixed dt, 60Hz):
  force_accumulator → integrator → collision → physics_joint

PostPhysics:
  gameplay → event_processor → input_gameplay → spawners
  → ghost_trail → particles → transition → dialogue → camera

RenderingPrep:
  clear → starfield → entities → particles → debug → HUD
  → screen_fx → transition_overlay → post_fx
  → events.clear → input.end_frame → frame_metrics
```

## Determinism Guarantees
- Fixed-timestep physics at 60Hz
- Deterministic entity ID assignment (monotonic u64)
- No wall-clock dependencies in simulation
- Headless runner produces identical results for same inputs
- RNG via deterministic pseudo_random(seed) functions

## ECS Invariants
- Entity(0) is reserved, never assigned
- IDs are monotonically increasing, never recycled
- Despawn atomically removes from all component stores
- World::clear() resets ID counter to 1
- Component stores are independent HashMaps (no archetype coupling)

## Ownership Model
- Engine owns World, all subsystems, and rendering state
- Systems borrow World mutably during their phase
- No shared ownership between systems
- Thread-local ENGINE wrapper is runtime-only (not part of core)

## Components (32)
Transform, RigidBody, Collider, Renderable, ForceField, Tags, Role, Lifetime,
GameState, Behavior, PhysicsMaterial, Impulse, MotionConstraint, ZoneEffect,
PropertyTween, EntityFlash, GhostTrail, TimeScale, Active, WaypointPath,
SignalEmitter, SignalReceiver, Parent, Children, WorldTransform, StateMachine,
Coroutine, SpriteAnimator, PhysicsJoint, ResourceInventory, GraphNode, VisualConnection

## Systems (21)
lifecycle, hierarchy, signal, state_machine, coroutine, behavior, tween, flash,
ghost_trail, waypoint, force_accumulator, integrator, collision, gameplay,
event_processor, input_gameplay, renderer, debug_render, sprite_animator,
physics_joint, (camera integrated in engine)

## Engine Modules
SceneManager, GameState (global), Timers, Templates, Behavior Rules, DialogueQueue,
SpawnQueue, Camera, TileMap, Raycast, SpatialHashGrid, EntityPool, EventBus,
InputMap, Pathfinding, Save/Load, FlowNetwork, ProceduralGen, EnvironmentClock,
DensityField, DiagnosticBus, GestureRecognizer, SoundCommandQueue, AutoJuice,
GameFlow, CameraDirector, ColorPalette, LevelCurve, UiCanvas, AimPreview, FrameMetrics

## Rendering Modules (12)
color, framebuffer, shapes, text, particles, starfield, post_fx, layers,
sprite, transition, screen_fx, (HUD in renderer)

## Import Patterns
Every system needs: `use crate::ecs::World;`
Logging: `use crate::log;` then `crate::log::log("msg")`
Math: `use crate::physics::math::{self, Vec2};`
Colors: `use crate::rendering::color::Color;`

## Adding Components Checklist
1. Create `components/my_component.rs`
2. Add `pub mod my_component;` to `components/mod.rs`
3. Add `pub use my_component::MyComponent;` to `components/mod.rs`
4. Add `pub my_components: ComponentStore<MyComponent>` to `World` in `ecs/world.rs`
5. Initialize in `World::new()`, add to `despawn()` and `clear()`
6. Implement `SchemaInfo` on the component
7. Add to `schema.rs` component list

## Adding Systems Checklist
1. Create `systems/my_system.rs`
2. Add structured comment: READS, WRITES, ORDER
3. Add `pub mod my_system;` to `systems/mod.rs`
4. Add call in `engine.rs` `tick()` at the correct position

## Explicit "Not Yet" List
These are premature for the current stage:
- Full renderer abstraction layer
- Archetype ECS rewrite
- Job system / thread pool
- Plugin architecture
- Diff-based snapshots
- Fully typed event bus
- Editor tooling
- Network sync layer
