# Crusty Engine Changelog

All notable changes to the Crusty game engine, organized by Innovation Games round.

---

## Round 11 — Physics & Core Engine Improvements

**4 engine upgrades, 31 new tests (1063 → 1094 total)**

### Engine Upgrades
- **Restitution Override** — `PhysicsMaterial.restitution_override` wired into CCD collision response. Changed combination from `min()` to `max()` enabling bumper entities with restitution >1.0 to amplify bounces. (5 tests)
- **Scene Isolation** — World snapshot save/restore for fight scene physics isolation. Added `push_with_snapshot()`/`pop_with_restore()` to SceneManager. Clone derives on World, ComponentStore, NameMap, SpawnQueue. (8 tests)
- **Multi-layer TileMap** — `TileLayer` struct with per-layer name, visibility, collidability, opacity. All existing APIs delegate to layer 0 for backward compatibility. New: `add_layer()`, `get_on_layer()`, `set_on_layer()`, `fill_rect_on_layer()`, `layer_count()`, `layer_index()`. `is_solid()` checks all collidable layers. (8 tests)
- **AimPreview** (`aim_preview.rs`) — New module for trajectory simulation. `compute_arc()` uses Euler integration with configurable drag, gravity, restitution, ball_radius. Detects wall collisions via closure, reflects velocity, produces `Vec<ArcPoint>` for rendering ghost dots. `compute_arc_with_hazards()` variant for hazard zone detection. (10 tests)

### Performance
- Removed `sorted_entities()` from collision hot path in `collision.rs` — eliminates per-frame Vec allocation + sort. Iteration order doesn't affect results because each entity's sweep is independent.

---

## Round 10 — Trap Links Game Design

**Game concept document: 622 lines, synthesized from 3 competing proposals**

### Trap Links: A Minigolf RPG
- **Concept**: Tile-based RPG where trap encounters open minigolf fight scenes (Pokémon meets mini-golf)
- **Core Loop**: Explore overworld → trigger trap → iris transition → solve minigolf puzzle → earn XP/gold/caddies
- **Fight Mechanic**: Slingshot aim model, par-based damage system, 8 obstacle types
- **5 Biomes**: Meadow Links, The Sandtraps, Frostway Greens, Cursed Caverns, The Final Course
- **Capture System**: Catch enemies as Caddies (passive fight bonuses), Shiny (1/50) and Gilded (1/200) variants
- **Equipment**: 8 Ball types, 12 Clubs, 10 Charms, 9 consumable items, 3 Signature Shots
- **Procedural Courses**: Seeded generation via cellular automata + A* validation per enemy type
- **Engine Gaps Identified**: 8 gaps for Rounds 5-6 (multi-layer TileMap, physics isolation, restitution, dialogue branching, UI tap detection, aim preview, sprite tiles, one-shot SFX)
- **Estimated Play Time**: ~7-8 hours first playthrough, infinite via Daily Links endgame

---

## Round 9 — Game Creation Platform

**4 new engine modules, 109 new tests (954 → 1063 total)**

### New Engine Modules
- **GameFlow** (`game_flow.rs`) — Declarative game lifecycle FSM with FlowState (Title/Playing/Paused/GameOver/Victory/Custom), FlowCondition (StateReaches/EventFired/ButtonTapped/AfterSeconds), CompareOp, pause/unpause, sound-on-enter, JSON export. (33 tests)
- **CameraDirector** (`camera_director.rs`) — Cinematic camera orchestration via mode stack. CameraMode (Follow/Pan/Zoom/Shake/Letterbox), CameraRule (EventBus→push_mode with auto-pop), 4 Easing curves (Linear/EaseIn/EaseOut/EaseInOut), dead-zone, letterbox rendering. (24 tests)
- **ColorPalette** (`color_palette.rs`) — Procedural art identity system. HSL color theory generation with 4 ColorSchemes, 7 semantic PaletteRoles, 6 built-in palettes (neon_cave, sunset, ice_dungeon, toxic_swamp, ocean_depths, volcanic), deterministic from_seed. (26 tests)
- **LevelCurve** (`level_curve.rs`) — Difficulty progression with named Curves (Linear/EaseIn/EaseOut/Step), keyframe interpolation with sorted auto-ordering, clamp_min/clamp_max, 3 DifficultyPresets (Casual=0.7x, Standard=1.0x, Hardcore=1.5x) with 5 pre-built curves each (enemy_speed, spawn_rate, enemy_health, player_damage, score_mult). (26 tests)

---

## Round 8 — Mobile & Game Feel

**6 new engine modules, 90 new tests (864 → 954 total)**

### New Engine Modules
- **GestureRecognizer** (`gesture.rs`) — Touch gesture recognition: tap, double-tap, long-press, swipe (with direction/velocity), pinch. Primary touch → mouse forwarding for backwards compatibility. EventBus integration publishes `gesture:tap`, `gesture:swipe`, etc. (18 tests)
- **SoundScape** (`sound.rs`) — Command-buffer audio system. `SoundCommandQueue` accumulates commands, `drain_json()` WASM binding lets JS Web Audio consume them each frame. `SoundPalette` provides 6 named presets (impact, pickup, explosion, ui_click, ambient_wind, game_over). (13 tests)
- **AutoJuice** (`auto_juice.rs`) — Declarative game-feel pipeline. Maps `JuiceTrigger` (OnCollision, OnSpawn, OnDespawn, OnEvent) to `JuiceEffect` (particles, shake, flash, sound, freeze frames). Builder pattern, symmetric collision matching. (10 tests)
- **UiCanvas** (`ui_canvas.rs`) — Anchor-based UI widget system. 9 anchor positions, `ValueBinding` for data-bound labels/bars/buttons. Framebuffer rendering + hit-testing for interactive overlays. (20 tests)
- **DiagnosticBus** (`diagnostics.rs`) — Structured runtime error reporting. Runs NaN transform, out-of-bounds, and entity count threshold checks each frame. JSON export via `get_diagnostics()` WASM binding. (14 tests)
- **WorldLint** (`world_lint.rs`) — Static analysis for .world files. Levenshtein-based "did you mean?" suggestions. Checks: W001 (physics without collider), E002 (unknown template), W003 (unknown tag), E005 (undefined timer), H006 (no player). (15 tests)

### WASM Bindings (4 new exports)
| Function | Description |
|----------|-------------|
| `touch_start/move/end(id, x, y)` | Touch input forwarded to gesture recognizer |
| `drain_sound_commands()` | Drain audio command queue as JSON |
| `get_diagnostics()` | Get runtime diagnostics as JSON |

---

## Engine Refactor — S-League Focus

**Major cleanup: removed scripting, old demos, mycelia; added headless testing**

### Removed
- **Scripting system**: `.world` file parser (pest grammar), loader, world_lint — games are now Rust crates only
- **Mycelia game module**: Replaced by S-League demo
- **Old demos**: game-1, game-2, game-3, innovations-1 removed from site
- **Stale docs**: PLAN.md, REVIEW.md, PROCESS.md, IMPLEMENTATION_PLAN.md, SPEC_*.md, SPORELINGS.md

### Added — S-League Demo (`trap_links_demo.rs`)
- Single minigolf hole: slingshot aim, shoot ball, sink in hole
- TileMap course with L-shaped wall, sand traps, border walls
- Physics: sub-stepping (4 per frame), wall collision reflection, drag, restitution
- AimPreview trajectory dots, power bar, HUD with stroke/par display
- Mobile-first 480x720 portrait with new site/s-league/ web deployment

### Added — Portability Infrastructure
- **SystemPhase enum**: Documents canonical tick execution order (Input → Simulation → Physics → PostPhysics → RenderingPrep)
- **FrameMetrics**: Lightweight per-frame telemetry (frame_time_ms, physics_time_ms, entity_count)
- **ENGINE_BOUNDARIES.md**: Documents no-dependency boundary (no windowing, graphics APIs, filesystem, audio)
- **RENDERER_FUTURE.md**: Software framebuffer today, WebGPU/wgpu migration path for later

### WASM Bindings (6 new S-League exports)
| Function | Description |
|----------|-------------|
| `sleague_init()` | Initialize S-League demo |
| `sleague_pointer_down/move/up(x, y)` | Aiming input |
| `sleague_update(dt_ms)` | Game logic tick |
| `sleague_render()` | Custom render pass |
| `get_frame_metrics()` | Performance telemetry JSON |

---

## Innovation Games: Headless Testing — Round 1

**Theme: Headless engine testing for AI feedback loops**

### New Module: `headless/` (6 files, 12 tests)
- **HeadlessRunner**: Wraps `Engine` for native `cargo test` simulation — no browser required
- **GameScenario**: Declarative test cases with scheduled input injection (PointerDown/Move/Up at specific frames) and post-run assertions (StateEquals, StateInRange, FramebufferHash)
- **ShotBuilder**: High-level API for constructing minigolf shots by angle (degrees) and power (0.0-1.0), abstracts slingshot drag math
- **framebuffer_hash**: FNV-1a hash of pixel buffer for deterministic visual regression detection
- **CLI `simulate` command**: `engine-cli simulate [frames] --json` outputs machine-readable game state

### Value
Claude Code can now write tests that run the full game loop natively, inject shots, assert on game state, and detect visual regressions — all via `cargo test`. The CLI command enables shell-level simulation with structured JSON output.

---

## Foundation (Pre-Innovation Games)

**Commits**: `first` through `Add GitHub Pages deployment`

### Core Engine
- **ECS Architecture**: Entity-Component-System with `ComponentStore<T>` (HashMap-backed), `Entity(u64)` IDs, `World` struct holding all stores
- **Fixed Timestep Physics**: `FIXED_DT = 1/60s` accumulator pattern in `Engine::tick()`
- **Software Renderer**: RGBA framebuffer shared with JS via WASM linear memory
- **WASM Bindings**: `wasm_bindgen` API for init, tick, input, framebuffer access

### Components (6)
| Component | Description |
|-----------|-------------|
| `Transform` | Position (x, y), rotation, scale |
| `RigidBody` | Velocity, acceleration, mass, body type (dynamic/static/kinematic), damping |
| `Collider` | Shape (Circle/Rect/Line), is_trigger flag, collision layer/mask |
| `Renderable` | Visual type (Rect/Circle/Ellipse/Polygon), color, z-order, visibility |
| `ForceField` | Field type (gravity/repulsion/vortex), strength, radius, falloff (linear/quadratic/none) |
| `Tags` | String tag set for entity classification |

### Systems (6)
| System | Description |
|--------|-------------|
| `force_accumulator` | Applies ForceField effects to nearby RigidBodies |
| `integrator` | Semi-implicit Euler integration (velocity → position) |
| `collision` | Spatial grid broad-phase + narrow-phase (circle-circle, circle-rect, rect-rect, circle-line), collision response with restitution |
| `event_processor` | Processes collision events for gameplay rules |
| `input_gameplay` | Maps keyboard/mouse input to player entity actions |
| `renderer` | Draws all Renderable entities to framebuffer |

### Rendering
- `color`: RGBA color with hex parsing, blend, lerp
- `framebuffer`: RGBA pixel buffer with clear, set_pixel, draw primitives
- `shapes`: Circle, rectangle, line, filled/outlined, polygon fill
- `text`: 5x7 bitmap font renderer (uppercase, digits, symbols)

### Infrastructure
- Spatial grid collision broad-phase (replaces O(n^2))
- GitHub Pages deployment with demo games
- Two demos: bouncing balls, arrow-key walker
- Comprehensive code review and cleanup
- 244 initial unit tests

---

## Innovation Games Round 1 — Space Survival
**Commit**: `eb94682` | **Theme**: Space Survival (asteroid dodging, wave spawning)

### New Components (4)
| Component | Description |
|-----------|-------------|
| `Role` | Entity role enum (Player/Enemy/Projectile/Pickup/Obstacle/Decoration/UI) |
| `Lifetime` | TTL timer — entity auto-despawns when expired |
| `GameState` | Per-entity key-value state (f64 + string maps) |
| `Behavior` | AI behavior with BehaviorMode (Idle/Chase/Flee/Wander/Patrol/Orbit), action queue |

### New Systems (3)
| System | Description |
|--------|-------------|
| `lifecycle` | Processes SpawnQueue, ticks Lifetime, despawns expired entities |
| `behavior` | Evaluates BehaviorRules against GameState conditions, executes actions |
| `gameplay` | Collision-triggered gameplay (damage, pickups, scoring) |

### New Engine Modules
- **SpawnQueue**: Deferred entity spawning with full component specification
- **GameState** (global): Engine-wide key-value store (f64/string) with WASM API
- **Timers**: One-shot and repeating named timers with fire count tracking
- **Templates**: Named entity template registry with `spawn()` at position
- **Behavior Rules**: Condition→action rules (state comparisons, timer checks, spawning)

### New Rendering
- **Particles**: Particle emitter system with burst/continuous modes, gravity, fade, size curves
- **Bitmap Text (HUD)**: Score, lives, wave display rendered at fixed screen positions
- **Starfield**: Parallax scrolling star background (seed-based generation)
- **Post-FX**: Vignette darkening, scanline overlay, screen shake

### Scripting
- **`.world` file format**: PEG grammar (pest) for declarative world definition
- **Parser**: Parses entities, components, properties, templates, timers, rules
- **Loader**: Maps parsed AST to ECS components and engine state

### Stats
- Tests: 285 (added scripting parser + system integration tests)
- Demo: game-3 (Space Survival) with asteroid waves, shooting, scoring

---

## Innovation Games Round 2 — Minigolf RPG
**Commit**: `18d3c31` | **Theme**: Minigolf tile-art RPG (precision physics, world traversal)

### New Components (5)
| Component | Description |
|-----------|-------------|
| `PhysicsMaterial` | Friction coefficient, drag, bounciness override per-entity |
| `Impulse` | One-shot force application (consumed after apply), with optional direction |
| `MotionConstraint` | Speed cap (max velocity), axis lock (X-only, Y-only, or free) |
| `ZoneEffect` | Area effect zones: Wind (directional force), Drag (slowdown), Conveyor (constant push) |
| — | (DialogueQueue is engine-level, not a component) |

### New Rendering
- **Camera**: Follow target entity with smooth lerp, configurable zoom, deadzone
- **Render Layers**: Ordered layer stack with per-layer parallax factor and offset
- **Sprite Renderer**: Sprite sheet support with frame selection, scale, flip
- **Scene Transitions**: Fade, iris (circle wipe), pixelate effects with configurable duration

### New Engine Modules
- **DialogueQueue**: Three display modes — dialogue box (bottom), notification (top toast), floating text (world-space above entity). Auto-advance with timers.

### Systems Enhanced
- `integrator`: Now applies PhysicsMaterial friction/drag, MotionConstraint speed caps
- `force_accumulator`: Now processes ZoneEffect forces
- `renderer`: Layer-aware rendering with camera transform and sprite support
- `lifecycle`: Handles all new component types in spawn processing

### Stats
- Tests: 378 (93 new tests for Round 2 features)

---

## Innovation Games Round 3 — Puzzle Platformer with Time Mechanics
**Commit**: `b60d519` | **Theme**: Puzzle platformer with temporal mechanics

### New Components (8)
| Component | Description |
|-----------|-------------|
| `PropertyTween` | Easing-curve animation for any numeric property. 9 easing functions: Linear, QuadIn/Out, CubicIn/Out, BounceOut, ElasticOut, SineInOut, ExpoOut. Supports looping and ping-pong. Target properties: X, Y, Rotation, Scale, VelocityX, VelocityY, Opacity. |
| `EntityFlash` | Visual flash effects — HitFlash (solid color overlay), Blink (visibility toggle), ColorPulse (sinusoidal intensity). Duration-based with automatic expiry. |
| `GhostTrail` | Fading afterimage trail using position snapshot ring buffer. Configurable interval, duration, max snapshots. Alpha fades based on age. |
| `TimeScale` | Per-entity time multiplier (0.0=frozen, 1.0=normal, 2.0=double speed). All time-aware systems respect this. Constructors: `normal()`, `frozen()`, `slow_mo(factor)`. |
| `Active` | Entity enable/disable flag. When `enabled=false`, systems skip the entity entirely. Constructors: `enabled()`, `disabled()`. |
| `WaypointPath` | Path-following along ordered waypoint sequences. Modes: Once (stop at end), Loop (wrap around), PingPong (reverse at ends). Configurable speed and pause-at-waypoint duration. |
| `SignalEmitter` | Named signal channel broadcaster. When `active=true`, emits on its channel name. |
| `SignalReceiver` | Multi-channel signal listener with AND/OR logic. Edge detection via `just_triggered()` and `just_released()`. Tracks previous frame state for rising/falling edges. |

### New Systems (5)
| System | Description |
|--------|-------------|
| `tween` | Ticks PropertyTween components, applies eased values to Transform/RigidBody/Renderable. Handles looping, ping-pong, completion removal. Respects per-entity TimeScale. |
| `flash` | Processes EntityFlash: ticks timers, toggles visibility for Blink mode, removes expired flashes, restores original visibility. Respects TimeScale. |
| `ghost_trail` | Captures position snapshots into ring buffer, ages existing snapshots. Respects TimeScale. |
| `waypoint` | Moves entities toward current waypoint at speed×dt. Handles Once/Loop/PingPong mode transitions and pause timers. Respects Active flag and TimeScale. |
| `signal` | Two-phase: (1) collects active SignalEmitter channels, (2) updates SignalReceiver triggered state with AND/OR logic and edge detection. |

### New Rendering
- **ScreenFxStack**: Composable stack of timed screen effects applied to the framebuffer. Effect types: Tint (color overlay with alpha), Desaturate (luminance-based grayscale blend), Flash (bright white burst). Effects auto-expire and are removed.

### New Engine Modules
- **SceneManager**: Named scene registry storing `.world` source strings. Push/pop stack semantics for scene navigation. Methods: register, push, pop, replace, current, depth, clear.

### System Execution Order (Final)
```
lifecycle → signal → behavior → tween → flash → waypoint
→ physics_loop(force_acc → integrator → collision)
→ gameplay → event_processor → input → spawners → ghost_trail
→ particles → transition → dialogue → camera
→ RENDER(clear → starfield → entities → particles → debug → HUD
         → screen_fx → transition_overlay → post_fx)
→ events.clear → input.end_frame
```

### Stats
- Tests: 544 (166 new tests for Round 3 features)
- New files: 14 (7 components, 5 systems, 1 rendering, 1 engine module)
- Modified files: 10 (integration across ECS, loader, schema, engine tick)

---

## Innovation Games Round 4 — Signal Breach (Tactical Stealth-Puzzle)
**Commit**: `3365f4a` | **Theme**: Signal Breach — tactical puzzle-stealth with hierarchy, FSM, scripted sequences

### New Components (5)
| Component | Description |
|-----------|-------------|
| `Parent` | Entity hierarchy parent reference. Points to parent Entity. |
| `Children` | Entity hierarchy children list. Deduplicating add/remove. |
| `WorldTransform` | Computed world-space transform (x, y, rotation, scale). Propagated from hierarchy. |
| `StateMachine` | Data-driven finite state machine. Current state, transitions with conditions (After/OnSignal/StateCheck/Always), elapsed timer, edge detection (just_entered, prev_state). |
| `Coroutine` | Scripted async step sequences using VecDeque. Steps: WaitSeconds, WaitSignal, WaitUntil, SetState, AddState, SpawnTemplate, Log. Builder pattern API. Non-blocking steps cascade in one frame. |

### New Systems (3)
| System | Description |
|--------|-------------|
| `hierarchy` | Two-phase transform propagation: roots get identity WorldTransform, children get parent transform applied with rotation and scale. Iterative multi-pass with convergence check. |
| `state_machine` | Ticks FSM elapsed timers, evaluates transition conditions (time-based, signal-based, state-check, always). First matching transition wins. Respects TimeScale. |
| `coroutine` | Processes coroutine step queues. Blocking steps (WaitSeconds, WaitSignal, WaitUntil) pause until condition met. Non-blocking steps (SetState, AddState, SpawnTemplate, Log) cascade instantly. Completed coroutines auto-removed. Respects TimeScale. |

### New Engine Modules (4)
| Module | Description |
|--------|-------------|
| `TileMap` | Row-major grid with tile types (Empty/Solid/Platform/Custom). World↔tile coordinate conversion, viewport-culled rendering, fill_rect operations. |
| `Raycast` | Ray-circle (quadratic), ray-AABB (slab method), DDA grid traversal for tilemaps. Functions: raycast (closest hit), raycast_all (sorted), line_of_sight (clear path check). |
| `SpatialHashGrid` | Cell-bucketed spatial index. Insert point/AABB, query by radius/AABB/nearest. Automatic deduplication for multi-cell entities. |
| `EntityPool` | Pre-warmed entity recycling with acquire/release pattern. PoolRegistry manages multiple named pools. |

### Stats
- Tests: 681 (137 new: 24 StateMachine, 7 Coroutine, 12 Raycast, 12 SpatialHashGrid, 20 TileMap, 18 EntityPool, 11 Hierarchy, 9 Hierarchy system, 10 SM system, 8 Coroutine system)
- New files: 10 (3 components, 3 systems, 4 engine modules)
- Modified files: 7 (world.rs, engine.rs, schema.rs, lib.rs, mod.rs files, spawn_queue.rs)

---

## Innovation Games Round 5 — Expert Review & E2E Testing
**Commits**: `47b7930` (review), `17f3be8` (E2E tests) | **Focus**: Code quality + integration test coverage

### Expert Rust Review Fixes
- **Allocation optimization**: Replaced `HashSet<String>` with `HashSet<&str>` in signal, state_machine, and coroutine systems (avoids per-frame string cloning)
- **Ghost trail ring buffer**: Replaced O(n) max-age search with O(1) `rotate_left(1)` for oldest snapshot replacement
- **Raycast DDA fix**: Track entry t-value for correct hit point on near face of tile (was using far face)
- **Raycast cleanup**: Removed unreachable wildcard arm in collider shape match
- **Flash system optimization**: Collect expired entities during main loop instead of second iteration pass
- **Tween system optimization**: Track empty PropertyTween components during iteration instead of second pass
- **Default impls**: Added `Default` for Children, PropertyTween, ScreenFxStack, SceneManager (idiomatic Rust)
- **Visibility**: Made `SpatialHashGrid.cell_size` public for debug introspection

### E2E Integration Tests (22 new tests)
| Test | Coverage |
|------|----------|
| E2E-1 | Full tick cycle: physics + collision end-to-end |
| E2E-2 | Hierarchy transform propagation through engine tick |
| E2E-3 | State machine transitions over multiple ticks |
| E2E-4 | Coroutine execution across multiple ticks with cascading |
| E2E-5 | Signal → StateMachine integration |
| E2E-6 | Tween X-axis completion over N frames |
| E2E-7 | Waypoint following with TimeScale |
| E2E-8 | EntityFlash with Active flag |
| E2E-9 | Tilemap queries end-to-end |
| E2E-10 | Raycasting hits entity via World |
| E2E-11 | Entity Pool lifecycle (prewarm, acquire, release) |
| E2E-12 | Spatial query radius lookups |
| E2E-13 | Multi-system concurrent: FSM + Coroutine + Tween |
| E2E-14 | Stress test: 1000 entities |
| E2E-15 | Tween Y-axis movement |
| E2E-16 | Waypoint Once mode: stop at final waypoint |
| E2E-17 | Signal receiver edge detection via tick |
| E2E-18 | Tilemap fill rect round-trip |
| E2E-19 | 3-level deep hierarchy chain |
| E2E-20 | StateCheck condition via GameState |
| E2E-21 | Stress: 100 parent-child hierarchy pairs |
| E2E-22 | Coroutine cascade: multiple set_state in one tick |

### Stats
- Tests: 703 (22 new E2E integration tests)
- Modified files: 13 (8 system/component optimizations, 1 test file)

---

## Innovation Games Round 6 — Feature Bonanza (6 New Modules)
**Commit**: `011a311` | **Theme**: Cherry-picked features from 4 competing game pitches

### New Components (2)
| Component | Description |
|-----------|-------------|
| `SpriteAnimator` | Named animation clip controller. Clips define frame sequences with per-frame duration. Supports play/stop/resume, looping, speed multiplier, `just_finished` edge detection for chaining. |
| `PhysicsJoint` | Constraint between two entities. Joint types: Distance (stiffness+damping), Spring (Hooke's law+damping), Rope (slack/taut with velocity clamping), Hinge (orbital with angular limits). Break force support. |

### New Systems (2)
| System | Description |
|--------|-------------|
| `sprite_animator` | Advances animation frame timers, handles looping/non-looping clips, sets `just_finished` flag, respects TimeScale. |
| `physics_joint` | Processes all joint types with mass-aware position correction and velocity adjustment. Handles static vs dynamic body ratios. Marks broken joints when break force exceeded. |

### New Engine Modules (4)
| Module | Description |
|--------|-------------|
| `EventBus` | Channel-based typed event system. Events carry optional source/target entities and key-value payloads (Float/Int/Text/Bool). Query by channel, source entity, or target entity. Auto-cleared each frame. |
| `InputMap` | Abstract input layer mapping raw keys/mouse buttons to named actions and axes. `is_action_held/pressed/released()`, `axis_value()`. Default WASD+arrows+space preset. |
| `Pathfinding` | A* grid pathfinding with octile heuristic. Diagonal movement with corner-cutting prevention. `PathConfig` for cost tuning and iteration limits. TileMap integration via `find_path_on_tilemap()`. |
| `Save/Load` | World state serialization to JSON. `WorldSnapshot` captures entities (transforms, game states, tags, names) plus global state and camera. Selective restore for transforms, game states, and global state. |

### Stats
- Tests: 786 (83 new: 10 SpriteAnimator component, 9 SpriteAnimator system, 6 PhysicsJoint component, 8 PhysicsJoint system, 11 EventBus, 16 InputMap, 11 Pathfinding, 12 Save/Load)
- New files: 8 (2 components, 2 systems, 4 engine modules)
- Modified files: 6 (world.rs, engine.rs, schema.rs, lib.rs, components/mod.rs, systems/mod.rs)

---

## Innovation Games Round 7 — Ecosystem Infrastructure
**Commit**: `6cde1bd` | **Theme**: Colony/ecosystem simulation infrastructure (4 agents independently pitched "Mycelium")

### New Components (3)
| Component | Description |
|-----------|-------------|
| `ResourceInventory` | Bounded multi-resource container. Slots with capacity, production/consumption rates. `deposit()/withdraw()` with overflow/underflow handling. Fill ratio tracking. Builder pattern. |
| `GraphNode` | Arbitrary entity-to-entity graph edges. Typed labels, weights, bidirectional flag. Query by edge type, find strongest edge, total weight. Group assignment for clustering. |
| `VisualConnection` | Visual link between two entities. Styles: Line, Dashed, Catenary (drooping curve), FlowLine (animated dots). Gradient color, layer ordering, flow intensity modulation. |

### New Engine Modules (4)
| Module | Description |
|--------|-------------|
| `FlowNetwork` | Directed graph of resource flow edges. Priority-based transfer solving each frame. Transfers resources between entity `ResourceInventory` slots respecting capacity. Enable/disable edges. |
| `ProceduralGen` | Seeded xorshift64 RNG. Multi-octave 2D value noise with smoothstep interpolation. Cellular automata cave generation (configurable birth/death thresholds). Room-and-corridor dungeon generator with L-shaped corridors. |
| `EnvironmentClock` | Global cyclical time system. Multiple named cycles (day/night, seasons). Per-cycle speed, phase queries, normalized progress, sine value for smooth transitions. Pause support. |
| `DensityField` | Continuous 2D scalar field on a regular grid. Bilinear interpolation sampling, weighted deposit/consume, central-difference gradient. Jacobi diffusion + multiplicative decay per timestep. Clamping. |

### Stats
- Tests: 850 (64 new: 11 ResourceInventory, 7 GraphNode, 4 VisualConnection, 7 FlowNetwork, 12 ProceduralGen, 11 EnvironmentClock, 12 DensityField)
- New files: 7 (3 components, 4 engine modules)
- Modified files: 5 (world.rs, engine.rs, schema.rs, lib.rs, components/mod.rs)

---

## Engine Summary (Current State)

### Component Count: 32
Transform, RigidBody, Collider, Renderable, ForceField, Tags, Role, Lifetime, GameState, Behavior, PhysicsMaterial, Impulse, MotionConstraint, ZoneEffect, PropertyTween, EntityFlash, GhostTrail, TimeScale, Active, WaypointPath, SignalEmitter, SignalReceiver, Parent, Children, WorldTransform, StateMachine, Coroutine, SpriteAnimator, PhysicsJoint, ResourceInventory, GraphNode, VisualConnection

### System Count: 21
lifecycle, hierarchy, signal, state_machine, coroutine, behavior, tween, flash, ghost_trail, waypoint, force_accumulator, integrator, collision, gameplay, event_processor, input_gameplay, renderer, debug_render, sprite_animator, physics_joint, (camera integrated in engine)

### Rendering Modules: 12
color, framebuffer, shapes, text, particles, starfield, post_fx, layers, sprite, transition, screen_fx, (HUD in renderer)

### Engine Modules: 21
SceneManager, GameState (global), Timers, Templates, Behavior Rules, DialogueQueue, SpawnQueue, Camera, TileMap, Raycast, SpatialHashGrid, EntityPool, EventBus, InputMap, Pathfinding, Save/Load, FlowNetwork, ProceduralGen, EnvironmentClock, DensityField, Headless

### Test Count: 1042+ (12 headless tests, scripting tests removed)
### Game: S-League minigolf RPG demo (trap_links_demo.rs)
### Headless: Native `cargo test` simulation, CLI simulate, visual regression hashing
