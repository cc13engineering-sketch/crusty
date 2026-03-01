# Architecture

## Overview
2D physics game engine. Rust → WASM → shared memory framebuffer → JS canvas. One HTML file, one JS file, one WASM binary.

## Data Flow
```
Input (JS → WASM strings) → Systems → Framebuffer (WASM memory → JS ImageData → Canvas)
```

## System Execution Order (per tick)
```
Physics (N times per frame at 60Hz fixed dt):
  1. force_accumulator — reset + accumulate accelerations (includes ZoneEffect)
  2. integrator — update velocities, apply friction/drag/constraints
  3. collision — CCD sweep, update positions, push events

Per-frame (once):
  lifecycle → signal → state_machine → coroutine → behavior
  → tween → flash → waypoint → ghost_trail
  → sprite_animator → physics_joint
  → gameplay → event_processor → input_gameplay
  → RENDER(clear → starfield → entities → particles → debug → HUD
           → screen_fx → transition_overlay → post_fx)
  → camera → events.clear → input.end_frame
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

## Engine Modules (20)
SceneManager, GameState (global), Timers, Templates, Behavior Rules, DialogueQueue,
SpawnQueue, Camera, TileMap, Raycast, SpatialHashGrid, EntityPool, EventBus,
InputMap, Pathfinding, Save/Load, FlowNetwork, ProceduralGen, EnvironmentClock,
DensityField

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
