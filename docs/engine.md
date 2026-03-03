# Crusty Engine Architecture

Complete technical reference for the Crusty Engine: ECS, rendering, physics, systems, and runtime.

## Overview

Crusty Engine is a Rust-native deterministic simulation engine that compiles to WebAssembly (browser) and native targets (headless testing, CLI). Features a custom ECS, software framebuffer renderer, fixed-timestep physics (60Hz), seeded RNG, and a 5-phase system execution pipeline.

All math uses `f64`. The framebuffer is a flat `Vec<u8>` in RGBA format, shared with JavaScript via WASM linear memory (zero-copy).

## Workspace

```
/engine
  /crates
    /engine-core    cdylib + rlib (WASM + native)
    /engine-cli     Native binary (14 CLI commands)
  Cargo.toml        Workspace root (opt-level="s", lto=true)
```

## The Engine Struct

Central object. Every subsystem is a field:

- **Core**: `world` (ECS), `framebuffer`, `input`, `events`, `config`, `camera`, `time`, `frame`, `rng` (SeededRng)
- **Rendering**: `particles`, `starfield`, `post_fx`, `layers`, `sprite_sheets`, `screen_fx`, `transition`
- **Gameplay**: `global_state`, `timers`, `templates`, `rules`, `game_flow`, `scene_manager`
- **Spatial**: `tilemap`, `pool_registry`
- **Events/Input**: `event_bus`, `input_map`, `gestures`
- **Advanced**: `flow_network`, `environment_clock`, `sound_queue`, `auto_juice`, `camera_director`, `level_curve`, `color_palette`, `ui_canvas`, `diagnostic_bus`, `frame_metrics`

## Simulation Trait

Games implement the `Simulation` trait — the formal boundary between engine and game logic:

```rust
pub trait Simulation {
    fn setup(&mut self, engine: &mut Engine);
    fn step(&mut self, engine: &mut Engine);
    fn render(&self, engine: &mut Engine);
    fn variants(&self) -> Vec<ParamSet> { vec![] }
}
```

The engine owns timing, input application, RNG, and determinism. Games read `engine.input` and `engine.global_state` during `step()`.

## 5-Phase Tick Loop

| Phase | Name | Runs | Key Systems |
|-------|------|------|-------------|
| 0 | Input | Once | Debug toggle, gesture recognition, gesture -> EventBus |
| 1 | Simulation | Fixed dt | lifecycle, hierarchy, signal, state_machine, coroutine, environment_clock, flow_network, sprite_animator, behavior, tween, flash, waypoint |
| 2 | Physics | Fixed 60Hz | force_accumulator, integrator, collision, physics_joint |
| 3 | PostPhysics | Once | gameplay, event_processor, input_gameplay, spawners, ghost_trail, particles, transition, dialogue, camera |
| 4 | RenderingPrep | Once | clear, starfield, entities, particles, debug, HUD, screen_fx, transition, post_fx, cleanup |

All simulation-phase systems receive `FIXED_DT` (1/60s). Variable dt from the host is used only for the physics accumulator.

## ECS

- `Entity` — `u64` newtype, `Entity(0)` is null
- `ComponentStore<T>` — `HashMap<Entity, T>` with ergonomic API
- `World` — holds all entities + 32 component stores

### 32 Components

Transform, RigidBody, Collider, Renderable, ForceField, Tags, Role, Lifetime, GameState, Behavior, PhysicsMaterial, Impulse, MotionConstraint, ZoneEffect, PropertyTween, EntityFlash, GhostTrail, TimeScale, Active, WaypointPath, SignalEmitter, SignalReceiver, Parent, Children, WorldTransform, StateMachine, Coroutine, SpriteAnimator, PhysicsJoint, ResourceInventory, GraphNode, VisualConnection

### 21 Systems

lifecycle, hierarchy, signal, state_machine, coroutine, sprite_animator, behavior, tween, flash, ghost_trail, waypoint, force_accumulator, integrator, collision, physics_joint, gameplay, event_processor, input_gameplay, renderer, debug_render

## Rendering (12 modules)

Pure software. No GPU. Modules: framebuffer, color, shapes, text, particles, starfield, post_fx, layers, sprite, transition, screen_fx.

Post-processing: vignette, CRT scanlines, screen shake, bloom.

## Physics

Fixed timestep at 60Hz. Semi-implicit Euler. Circle-circle and circle-rect collision. Continuous collision detection (CCD). Spatial grid broadphase. Joint constraints: distance, spring, rope, hinge.

## Key Subsystems

- **GameState** — `HashMap<String, StateValue>` (F64/Bool/Str). Primary game-testing interface.
- **EventBus** — Frame-scoped pub/sub. Central integration point for cross-system communication.
- **Sound** — Command-buffer pattern. Rust queues, JS polls via `drain_sound_commands()`.
- **Tilemap** — Multi-layer grid. Tile types: Empty, Solid, Platform, Custom(u16).
- **GameFlow** — Declarative lifecycle FSM (Title, Playing, Paused, GameOver, Victory).
- **AutoJuice** — Trigger-based automatic game feel (particles, shake, flash, sound on collision/spawn/despawn).
- **CameraDirector** — Stack-based cinematic camera (Follow, Pan, Zoom, Shake, Letterbox).
- **GestureRecognizer** — Touch gesture recognition (Tap, DoubleTap, LongPress, Swipe, Pinch).

## Determinism

- **SeededRng** (xorshift64) owned by Engine — the single canonical RNG
- **State hashing** via `Engine::state_hash()` — deterministic u64 independent of rendering
- **Seeded reset** via `Engine::reset(seed)` — single entry point for reproducible simulation
- **Fixed DT** for all simulation systems
- **InputFrame** as canonical input representation for replays and policies

## WASM API

Thread-local `Engine` singleton. Key exports: `init`, `tick`, `framebuffer_ptr/len`, keyboard/mouse/touch input, `setup_demo_ball`, `get_game_state`, `drain_sound_commands`, `get_diagnostics`, `get_frame_metrics`.

## Design Decisions

1. Single cdylib+rlib crate — same code for WASM and native
2. Zero-copy framebuffer — JS reads directly from WASM linear memory
3. All f64 math — no f32 anywhere
4. Sound as JSON drain — no JS callbacks from Rust
5. Fixed 60Hz physics — deterministic, independent of display rate
6. Simulation trait boundary — games implement setup/step/render, engine owns timing and input
7. Deferred mutation — SpawnQueue, SoundQueue, EventBus stage changes for phase boundaries
8. EventBus as integration point — nearly all cross-system communication flows through it
