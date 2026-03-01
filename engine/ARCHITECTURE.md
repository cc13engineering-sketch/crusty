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
  1. force_accumulator — reset + accumulate accelerations
  2. integrator — update velocities (NOT positions)
  3. collision — CCD sweep, update positions, push events

Per-frame (once):
  4. event_processor — handle collision/trigger events
  5. input_gameplay — drag-to-launch
  6. renderer — clear + draw all entities
  7. debug_render — velocity vectors, force field radii, collider wireframes
```

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
