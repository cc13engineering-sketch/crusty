# Architecture

## Overview
2D physics game engine purpose-built for the S-League minigolf RPG. Rust → WASM → shared memory framebuffer → JS canvas. Games are defined as Rust modules, not spec files.

## Data Flow
```
Input (JS → WASM) → Systems → Framebuffer (WASM memory → JS ImageData → Canvas)
Sound commands queued in Rust → drained as JSON by JS
Diagnostics/metrics → drained as JSON by JS
```

## System Execution Order (per tick)
See `SystemPhase` enum in engine.rs for authoritative documentation.
```
Input:
  debug toggle, gesture recognition, gesture→EventBus

Simulation (variable dt):
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
