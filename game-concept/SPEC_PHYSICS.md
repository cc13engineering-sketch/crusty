# SPEC_PHYSICS: Physics Isolation & Ball-Specific Collision Response

**Gaps addressed**: Gap 4 (Physics isolation per scene), Gap 7 (Ball-specific collision response)
**Target engine**: Crusty (Rust -> WASM -> Canvas 2D)
**Depends on**: `ecs/world.rs`, `scene_manager.rs`, `systems/collision.rs`, `components/rigidbody.rs`, `components/physics_material.rs`, `components/collider.rs`, `physics/ccd.rs`, `engine.rs`

---

## Gap 4: Physics Isolation Per Scene

### Problem Statement

When the Trap Links fight scene is pushed via `SceneManager::push()`, the overworld's physics entities (player position, NPC velocities, force fields, zone effects) remain live in the single `World` instance. The fight scene spawns its own ball, walls, and bumpers into the same `World`. When the fight ends and `SceneManager::pop()` fires, the overworld entities may have drifted, collided with fight-scene entities, or had their velocities corrupted by fight-scene forces.

The current `SceneManager` stores only a `SceneStackEntry { name: String }` -- it carries zero world state. There is no snapshot, no isolation, and no restore mechanism tied to push/pop.

### Design Goal

When `SceneManager::push("fight_scene")` is called:
1. The entire current `World` is snapshotted and stored on the scene stack.
2. The `World` is cleared for the incoming scene.
3. When `SceneManager::pop()` is called, the snapshotted `World` is restored wholesale.
4. The fight scene's entities are discarded entirely on pop.

This gives each scene a fully isolated physics sandbox.

### 1. Struct/Enum/Trait Additions

#### 1a. `PhysicsWorldSnapshot` (new struct in `scene_manager.rs`)

```rust
/// A lightweight clone of all physics-relevant World state.
/// Stores the complete entity set and all component stores so that
/// push/pop can fully save and restore world state.
#[derive(Clone)]
pub struct PhysicsWorldSnapshot {
    pub next_id: u64,
    pub alive: HashSet<Entity>,
    pub names: NameMapSnapshot,
    pub transforms: Vec<(Entity, Transform)>,
    pub rigidbodies: Vec<(Entity, RigidBody)>,
    pub colliders: Vec<(Entity, Collider)>,
    pub renderables: Vec<(Entity, Renderable)>,
    pub force_fields: Vec<(Entity, ForceField)>,
    pub tags: Vec<(Entity, Tags)>,
    pub roles: Vec<(Entity, Role)>,
    pub lifetimes: Vec<(Entity, Lifetime)>,
    pub game_states: Vec<(Entity, GameState)>,
    pub behaviors: Vec<(Entity, Behavior)>,
    pub physics_materials: Vec<(Entity, PhysicsMaterial)>,
    pub impulses: Vec<(Entity, Impulse)>,
    pub motion_constraints: Vec<(Entity, MotionConstraint)>,
    pub zone_effects: Vec<(Entity, ZoneEffect)>,
    pub property_tweens: Vec<(Entity, PropertyTween)>,
    pub entity_flashes: Vec<(Entity, EntityFlash)>,
    pub ghost_trails: Vec<(Entity, GhostTrail)>,
    pub time_scales: Vec<(Entity, TimeScale)>,
    pub actives: Vec<(Entity, Active)>,
    pub waypoint_paths: Vec<(Entity, WaypointPath)>,
    pub signal_emitters: Vec<(Entity, SignalEmitter)>,
    pub signal_receivers: Vec<(Entity, SignalReceiver)>,
    pub parents: Vec<(Entity, Parent)>,
    pub children: Vec<(Entity, Children)>,
    pub world_transforms: Vec<(Entity, WorldTransform)>,
    pub state_machines: Vec<(Entity, StateMachine)>,
    pub coroutines: Vec<(Entity, Coroutine)>,
    pub sprite_animators: Vec<(Entity, SpriteAnimator)>,
    pub physics_joints: Vec<(Entity, PhysicsJoint)>,
    pub resource_inventories: Vec<(Entity, ResourceInventory)>,
    pub graph_nodes: Vec<(Entity, GraphNode)>,
    pub visual_connections: Vec<(Entity, VisualConnection)>,
}
```

**Rationale**: Using `Vec<(Entity, T)>` rather than cloning `ComponentStore` directly avoids exposing `ComponentStore`'s internal `HashMap` and keeps the snapshot type `Clone`-friendly. Every component type already derives `Clone + Debug`, so collecting into vecs is straightforward.

**Why not reuse `save_load::WorldSnapshot`?** That struct uses JSON serialization and only captures a subset of component types (Transform, RigidBody, GameState, Tags, Role, StateMachine). It is designed for save files, not in-memory round-tripping. `PhysicsWorldSnapshot` captures ALL component stores with zero serialization overhead.

#### 1b. `NameMapSnapshot` (new struct in `scene_manager.rs`)

```rust
/// Snapshot of bidirectional name mapping.
#[derive(Clone)]
pub struct NameMapSnapshot {
    pub name_to_entity: HashMap<String, Entity>,
    pub entity_to_name: HashMap<Entity, String>,
}
```

#### 1c. `EngineSnapshot` (new struct in `scene_manager.rs`)

```rust
/// Captures engine-level state that must be preserved across scene push/pop.
/// This includes the world snapshot plus engine subsystems that are
/// scene-specific (camera, config, particles, timers, global state, etc.).
#[derive(Clone)]
pub struct EngineSnapshot {
    pub world: PhysicsWorldSnapshot,
    pub config: WorldConfig,
    pub camera: Camera,
    pub global_state: GlobalGameState,
    pub timers: TimerQueue,
    pub rules: BehaviorRules,
    pub game_over: bool,
    pub tilemap: Option<TileMap>,
}
```

**Rationale**: The fight scene needs its own camera (static, unzoomed), its own world config (480x600 bounds), and its own tilemap (the golf course). The overworld's camera position, target tag, zoom level, and tilemap must be preserved.

#### 1d. Modified `SceneStackEntry`

```rust
/// Entry in the scene stack. Now carries a full engine snapshot
/// so that push/pop achieves complete isolation.
#[derive(Clone)]
pub struct SceneStackEntry {
    pub name: String,
    pub snapshot: Option<EngineSnapshot>,
}
```

The `snapshot` is `Option` so that the bottom of the stack (the initial scene) can be `None` -- there is nothing to restore below it.

### 2. Functions to Modify

#### 2a. `SceneManager::push()` -- Major rewrite

**Current signature**: `pub fn push(&mut self, name: &str) -> bool`

**New signature**: `pub fn push_with_snapshot(&mut self, name: &str, snapshot: EngineSnapshot) -> bool`

The old `push()` remains for backward compatibility but calls `push_with_snapshot` with `snapshot: None` in the stack entry. The new method:

1. Verifies the scene name is registered.
2. Creates a `SceneStackEntry` with the provided `EngineSnapshot`.
3. Pushes it onto the stack.
4. Returns `true`.

The *caller* (engine-level code) is responsible for capturing the snapshot from the current engine state before calling push, and then clearing/loading the new scene's world afterward. This keeps `SceneManager` decoupled from `Engine`.

```rust
impl SceneManager {
    /// Push a scene with an associated engine snapshot for later restoration.
    pub fn push_with_snapshot(&mut self, name: &str, snapshot: EngineSnapshot) -> bool {
        if !self.registry.contains_key(name) {
            return false;
        }
        self.stack.push(SceneStackEntry {
            name: name.to_string(),
            snapshot: Some(snapshot),
        });
        true
    }

    /// Pop the top scene and return its snapshot (if any) for restoration.
    pub fn pop_with_snapshot(&mut self) -> Option<(String, Option<EngineSnapshot>)> {
        self.stack.pop().map(|entry| (entry.name, entry.snapshot))
    }
}
```

#### 2b. `PhysicsWorldSnapshot::capture()` (new method)

```rust
impl PhysicsWorldSnapshot {
    /// Snapshot the entire World into a PhysicsWorldSnapshot.
    pub fn capture(world: &World) -> Self {
        Self {
            next_id: world.next_id,
            alive: world.alive.clone(),
            names: NameMapSnapshot {
                name_to_entity: world.names.name_to_entity.clone(),
                entity_to_name: world.names.entity_to_name.clone(),
            },
            transforms: world.transforms.iter().map(|(e, c)| (e, c.clone())).collect(),
            rigidbodies: world.rigidbodies.iter().map(|(e, c)| (e, c.clone())).collect(),
            colliders: world.colliders.iter().map(|(e, c)| (e, c.clone())).collect(),
            renderables: world.renderables.iter().map(|(e, c)| (e, c.clone())).collect(),
            // ... (one line per component store, same pattern)
        }
    }
}
```

**Note on `NameMap` access**: The current `NameMap` struct has private fields (`name_to_entity`, `entity_to_name`). Two options:

- **Option A (preferred)**: Add `pub fn snapshot(&self) -> NameMapSnapshot` and `pub fn restore(&mut self, snap: NameMapSnapshot)` methods to `NameMap` in `world.rs`. This preserves encapsulation.
- **Option B**: Make the fields `pub(crate)`.

We choose Option A.

**Note on `World::next_id` access**: This field is currently private. Add a getter `pub fn next_id(&self) -> u64` and a setter `pub fn set_next_id(&mut self, id: u64)` to `World`.

#### 2c. `PhysicsWorldSnapshot::restore()` (new method)

```rust
impl PhysicsWorldSnapshot {
    /// Restore a World from this snapshot, fully replacing all state.
    pub fn restore(self, world: &mut World) {
        world.clear();
        world.set_next_id(self.next_id);
        world.alive = self.alive;
        world.names.restore(self.names);

        for (entity, component) in self.transforms {
            world.transforms.insert(entity, component);
        }
        for (entity, component) in self.rigidbodies {
            world.rigidbodies.insert(entity, component);
        }
        // ... (one block per component store, same pattern)
    }
}
```

#### 2d. `EngineSnapshot::capture()` and `EngineSnapshot::restore()` (new methods)

```rust
impl EngineSnapshot {
    /// Capture engine-level scene state.
    pub fn capture(engine: &Engine) -> Self {
        Self {
            world: PhysicsWorldSnapshot::capture(&engine.world),
            config: engine.config.clone(),
            camera: engine.camera.clone(),
            global_state: engine.global_state.clone(),
            timers: engine.timers.clone(),
            rules: engine.rules.clone(),
            game_over: engine.game_over,
            tilemap: engine.tilemap.clone(),
        }
    }

    /// Restore engine-level scene state. Clears current world first.
    pub fn restore(self, engine: &mut Engine) {
        self.world.restore(&mut engine.world);
        engine.config = self.config;
        engine.camera = self.camera;
        engine.global_state = self.global_state;
        engine.timers = self.timers;
        engine.rules = self.rules;
        engine.game_over = self.game_over;
        engine.tilemap = self.tilemap;
    }
}
```

#### 2e. Engine-level convenience methods (in `engine.rs`)

```rust
impl Engine {
    /// Push a new scene with physics isolation.
    /// Snapshots current world/config/camera state, clears world,
    /// and prepares for the new scene to be loaded.
    pub fn push_scene(&mut self, scene_name: &str) -> bool {
        let snapshot = EngineSnapshot::capture(self);
        if !self.scene_manager.push_with_snapshot(scene_name, snapshot) {
            return false;
        }
        // Clear world for the incoming scene
        self.world.clear();
        self.reset_game_state();
        // Load the new scene's .world source if available
        if let Some(source) = self.scene_manager.get_source(scene_name) {
            let source = source.to_string();
            match crate::scripting::parser::parse_world(&source) {
                Ok(world_file) => {
                    crate::scripting::loader::load_world_full(
                        &world_file, &mut self.world, &mut self.config,
                        &mut self.global_state, &mut self.timers,
                        &mut self.templates, &mut self.rules,
                    );
                }
                Err(e) => {
                    crate::log::error(&format!("Scene load error: {}", e));
                }
            }
        }
        true
    }

    /// Pop the current scene and restore the previous scene's state.
    /// Returns the name of the popped scene, or None if stack was empty.
    pub fn pop_scene(&mut self) -> Option<String> {
        if let Some((name, snapshot)) = self.scene_manager.pop_with_snapshot() {
            if let Some(snap) = snapshot {
                snap.restore(self);
            }
            Some(name)
        } else {
            None
        }
    }
}
```

#### 2f. Required changes to existing types for `Clone`

Several engine subsystems stored in `EngineSnapshot` must derive or implement `Clone`:

- `WorldConfig` -- already `Clone`.
- `Camera` -- already `Clone` (has `#[derive(Clone, Debug)]`).
- `GlobalGameState` (the `game_state::GameState` type) -- verify it is `Clone`. If not, add `#[derive(Clone)]`.
- `TimerQueue` -- must add `Clone` if not present.
- `BehaviorRules` -- must add `Clone` if not present.
- `TileMap` -- must add `Clone` if not present.

For any type that cannot easily derive `Clone` (e.g., contains closures or `Rc`), we either:
- Wrap the field in `Option` and skip it in snapshot (losing that subsystem state on restore, which is acceptable for things like particle pools).
- Add manual `Clone` implementation that does a logical clone.

#### 2g. `NameMap` additions (in `world.rs`)

```rust
impl NameMap {
    /// Capture current state as a snapshot.
    pub fn snapshot(&self) -> NameMapSnapshot {
        NameMapSnapshot {
            name_to_entity: self.name_to_entity.clone(),
            entity_to_name: self.entity_to_name.clone(),
        }
    }

    /// Restore state from a snapshot, replacing all current mappings.
    pub fn restore(&mut self, snap: NameMapSnapshot) {
        self.name_to_entity = snap.name_to_entity;
        self.entity_to_name = snap.entity_to_name;
    }
}
```

#### 2h. `World` accessor additions (in `world.rs`)

```rust
impl World {
    /// Get the next entity ID (for snapshot purposes).
    pub fn next_id(&self) -> u64 {
        self.next_id
    }

    /// Set the next entity ID (for restore purposes).
    /// SAFETY: Caller must ensure no ID collisions.
    pub fn set_next_id(&mut self, id: u64) {
        self.next_id = id;
    }
}
```

### 3. Integration with Existing Patterns

#### `with_engine` pattern
No changes needed. The `with_engine` closure accesses `&mut Engine`, and the new `push_scene`/`pop_scene` methods are on `Engine`. Game code calls them like:

```rust
// In a Trap Links game module:
with_engine(|eng| {
    eng.push_scene("fight_mole_3");
    // Fight scene is now active, overworld is frozen on the stack
});

// After the fight resolves:
with_engine(|eng| {
    eng.pop_scene();
    // Overworld is fully restored
});
```

#### `ComponentStore` pattern
The snapshot iterates each store with `.iter()` (returns `(Entity, &T)`) and collects `(Entity, T)` via `.clone()`. Restore calls `.insert(entity, component)` for each pair. This follows exactly how all systems already read and write component stores.

#### `SceneManager` existing API
The original `push()` and `pop()` remain functional for non-isolated scene transitions (e.g., switching between overworld rooms). The new `push_with_snapshot()` / `pop_with_snapshot()` are additive. `push()` still works -- it just creates a stack entry with `snapshot: None`.

### 4. Test Cases

#### Test 4.1: `PhysicsWorldSnapshot::capture` preserves entity count
```
Setup: Create World with 5 entities, each with Transform + RigidBody.
Action: Call PhysicsWorldSnapshot::capture(&world).
Assert: snapshot.alive.len() == 5.
Assert: snapshot.transforms.len() == 5.
Assert: snapshot.rigidbodies.len() == 5.
Assert: snapshot.next_id == world.next_id().
```

#### Test 4.2: `PhysicsWorldSnapshot::restore` restores exact positions
```
Setup: Create World with 2 entities. Entity(1) at (10.0, 20.0), Entity(2) at (30.0, 40.0).
Action: Capture snapshot. Modify Entity(1) position to (999.0, 999.0). Call snapshot.restore(&mut world).
Assert: world.transforms.get(Entity(1)).x == 10.0.
Assert: world.transforms.get(Entity(1)).y == 20.0.
Assert: world.transforms.get(Entity(2)).x == 30.0.
Assert: world.transforms.get(Entity(2)).y == 40.0.
```

#### Test 4.3: `PhysicsWorldSnapshot::restore` restores velocities
```
Setup: Create World with 1 entity. RigidBody with vx=100.0, vy=-50.0.
Action: Capture snapshot. Set RigidBody vx=0.0, vy=0.0. Restore.
Assert: rb.vx == 100.0, rb.vy == -50.0.
```

#### Test 4.4: Push/pop isolation -- fight scene entities do not persist
```
Setup: Create World with 3 overworld entities (tagged "player", "npc1", "npc2").
Action: Capture EngineSnapshot. Clear world. Spawn 10 fight-scene entities. Then restore from snapshot.
Assert: world.entity_count() == 3.
Assert: world.tags.iter() finds "player", "npc1", "npc2".
Assert: No fight-scene entities remain.
```

#### Test 4.5: Push/pop isolation -- overworld velocities are frozen
```
Setup: Create World with "player" entity, RigidBody vx=50, vy=0.
Action: Capture snapshot. Clear world. Spawn fight-scene ball, run 100 physics steps.
Then restore snapshot.
Assert: player RigidBody vx == 50.0, vy == 0.0 (unchanged by fight-scene physics).
```

#### Test 4.6: Named entities survive push/pop
```
Setup: Create World. spawn_named("hero"), spawn_named("boss_npc").
Action: Capture snapshot, clear, restore.
Assert: world.names.get_by_name("hero") returns the original Entity.
Assert: world.names.get_by_name("boss_npc") returns the original Entity.
```

#### Test 4.7: `next_id` continuity after restore
```
Setup: Create World, spawn 5 entities (next_id == 6).
Action: Capture snapshot. Clear world (resets next_id to 1). Restore.
Assert: world.next_id() == 6.
Action: Spawn new entity.
Assert: new entity == Entity(6), not Entity(1).
```

#### Test 4.8: Double push/pop (nested scenes)
```
Setup: World with 2 entities. Push scene A (snapshot has 2 entities).
In scene A: spawn 5 entities. Push scene B (snapshot has 5 entities).
In scene B: spawn 3 entities.
Action: Pop scene B.
Assert: world.entity_count() == 5 (scene A's entities).
Action: Pop scene A.
Assert: world.entity_count() == 2 (original entities).
```

#### Test 4.9: Push unregistered scene returns false, no snapshot created
```
Setup: SceneManager with no scenes registered.
Action: push_with_snapshot("nonexistent", snapshot).
Assert: returns false.
Assert: stack depth == 0.
Assert: World unchanged.
```

#### Test 4.10: Empty world snapshot/restore round-trip
```
Setup: Empty World (no entities).
Action: Capture snapshot. Spawn 3 entities. Restore.
Assert: world.entity_count() == 0.
Assert: world.next_id() == 1.
```

### 5. Estimated Lines of Code

| Item | LOC |
|------|-----|
| `PhysicsWorldSnapshot` struct definition | ~45 |
| `NameMapSnapshot` struct definition | ~6 |
| `EngineSnapshot` struct definition | ~12 |
| `PhysicsWorldSnapshot::capture()` | ~40 |
| `PhysicsWorldSnapshot::restore()` | ~45 |
| `EngineSnapshot::capture()` | ~15 |
| `EngineSnapshot::restore()` | ~15 |
| `SceneStackEntry` modification | ~5 |
| `SceneManager::push_with_snapshot()` | ~12 |
| `SceneManager::pop_with_snapshot()` | ~8 |
| `Engine::push_scene()` | ~25 |
| `Engine::pop_scene()` | ~12 |
| `NameMap::snapshot()` / `restore()` | ~12 |
| `World::next_id()` / `set_next_id()` | ~8 |
| Clone derivations on engine subsystems | ~10 |
| Test cases (10 tests) | ~200 |
| **Total** | **~470** |

---

## Gap 7: Ball-Specific Collision Response

### Problem Statement

The current collision system in `systems/collision.rs` (line 169) computes the restitution coefficient for a collision pair as:

```rust
let e = snap.restitution.min(other.restitution);
```

This takes the **minimum** restitution of the two colliding bodies. The `restitution` value comes from the `RigidBody` component (default 0.5).

The problem is twofold:

1. **The min-rule is wrong for the game design.** Trap Links needs bumpers with `restitution > 1.0` (amplifying bounces). With a min-rule, a ball with `restitution = 0.9` hitting a bumper with `restitution = 1.4` would resolve at `min(0.9, 1.4) = 0.9` -- the bumper's amplifying property is entirely ignored.

2. **`PhysicsMaterial.restitution_override` is never consulted.** The `PhysicsMaterial` component has a `restitution_override: Option<f64>` field that exists precisely for this purpose, but the collision system ignores it completely. It only reads `RigidBody.restitution`.

3. **No collision pair formula.** Different games need different combination rules. Minigolf needs `max(a, b)` so that a bouncy bumper always dominates. A realistic physics sim might want `sqrt(a * b)` (geometric mean). The engine should support configurable combination rules.

### Design Goal

- The collision system respects per-entity `PhysicsMaterial.restitution_override` when present, falling back to `RigidBody.restitution`.
- A configurable `RestitutionRule` controls how two restitution values combine.
- Bumpers with `restitution = 1.4` actually amplify the ball's speed.
- The combination rule is set at the engine level (one rule per scene, not per-entity).

### 1. Struct/Enum/Trait Additions

#### 1a. `RestitutionRule` (new enum in `components/physics_material.rs`)

```rust
/// Rule for combining two restitution coefficients in a collision pair.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RestitutionRule {
    /// Use the smaller of the two values: min(a, b).
    /// Conservative, prevents amplification. Current default behavior.
    Min,
    /// Use the larger of the two values: max(a, b).
    /// Lets bouncy surfaces dominate. Ideal for bumper mechanics.
    Max,
    /// Arithmetic mean: (a + b) / 2.0.
    /// Balanced middle ground.
    Average,
    /// Geometric mean: sqrt(a * b).
    /// Physically motivated. Preserves energy better.
    Geometric,
    /// Multiply the two values: a * b.
    /// High restitution on both sides compounds. Low values suppress each other.
    Multiply,
}

impl RestitutionRule {
    /// Combine two restitution coefficients using this rule.
    pub fn combine(self, a: f64, b: f64) -> f64 {
        match self {
            RestitutionRule::Min => a.min(b),
            RestitutionRule::Max => a.max(b),
            RestitutionRule::Average => (a + b) * 0.5,
            RestitutionRule::Geometric => (a * b).abs().sqrt(),
            RestitutionRule::Multiply => a * b,
        }
    }
}

impl Default for RestitutionRule {
    fn default() -> Self {
        RestitutionRule::Max // Default to Max for Trap Links bumper gameplay
    }
}
```

**Rationale for `Max` as default**: The game design document explicitly lists bumpers with `restitution = 1.4` and describes amplifying bounces. The `Max` rule is the simplest formulation that enables this. For a realistic physics game, developers would switch to `Average` or `Geometric`.

#### 1b. `CollisionConfig` (new struct in `systems/collision.rs` or new file)

```rust
/// Configuration for the collision resolution system.
/// Set at the engine level; applies to all collisions in the current scene.
#[derive(Clone, Debug)]
pub struct CollisionConfig {
    /// How to combine restitution coefficients from two colliding entities.
    pub restitution_rule: RestitutionRule,
    /// Maximum allowed combined restitution.
    /// Prevents infinite energy gain from extreme amplification.
    /// Default: 2.0 (ball can at most double its speed on a single bounce).
    pub max_restitution: f64,
    /// Minimum restitution floor. Prevents zero-bounce dead stops when unintended.
    /// Default: 0.0 (no floor).
    pub min_restitution: f64,
}

impl Default for CollisionConfig {
    fn default() -> Self {
        Self {
            restitution_rule: RestitutionRule::default(),
            max_restitution: 2.0,
            min_restitution: 0.0,
        }
    }
}
```

### 2. Functions to Modify

#### 2a. `Engine` struct -- add `collision_config` field

In `engine.rs`, add to the `Engine` struct:

```rust
pub struct Engine {
    // ... existing fields ...

    /// Configuration for collision response (restitution combination rule).
    pub collision_config: CollisionConfig,
}
```

Initialize in `Engine::new()`:
```rust
collision_config: CollisionConfig::default(),
```

#### 2b. `Engine::reset_game_state()` -- reset collision config

Add to `reset_game_state()`:
```rust
self.collision_config = CollisionConfig::default();
```

#### 2c. `systems::collision::run()` -- accept and use `CollisionConfig`

**Current signature**:
```rust
pub fn run(world: &mut World, events: &mut EventQueue, dt: f64)
```

**New signature**:
```rust
pub fn run(world: &mut World, events: &mut EventQueue, dt: f64, config: &CollisionConfig)
```

#### 2d. `EntitySnap` -- use effective restitution from `PhysicsMaterial`

Modify the snapshot collection to prefer `PhysicsMaterial.restitution_override` over `RigidBody.restitution`:

**Current code (line 53-63)**:
```rust
snaps.push(EntitySnap {
    entity: *entity,
    pos: (t.x, t.y),
    vel: rb.map_or((0.0, 0.0), |r| (r.vx, r.vy)),
    collider: c.shape.clone(),
    radius_for_sweep: radius,
    restitution: rb.map_or(0.5, |r| r.restitution),
    is_static: rb.map_or(true, |r| r.is_static),
    is_trigger: c.is_trigger,
    has_rigidbody: rb.is_some(),
});
```

**New code**:
```rust
// Determine effective restitution: PhysicsMaterial override > RigidBody value > default
let base_restitution = rb.map_or(0.5, |r| r.restitution);
let effective_restitution = physics_materials
    .get(*entity)
    .and_then(|pm| pm.restitution_override)
    .unwrap_or(base_restitution);

snaps.push(EntitySnap {
    entity: *entity,
    pos: (t.x, t.y),
    vel: rb.map_or((0.0, 0.0), |r| (r.vx, r.vy)),
    collider: c.shape.clone(),
    radius_for_sweep: radius,
    restitution: effective_restitution,
    is_static: rb.map_or(true, |r| r.is_static),
    is_trigger: c.is_trigger,
    has_rigidbody: rb.is_some(),
});
```

This requires adding `physics_materials` to the destructured `World` at the top of `run()`:

**Current**:
```rust
let World { transforms, colliders, rigidbodies, tags: _, .. } = world;
```

**New**:
```rust
let World { transforms, colliders, rigidbodies, physics_materials, tags: _, .. } = world;
```

#### 2e. Collision resolution -- use `RestitutionRule` instead of `min`

**Current code (line 169-171)**:
```rust
let e = snap.restitution.min(other.restitution);
let reflected = math::reflect(current_vel, hit.normal);
let new_vel = math::scale(reflected, e);
```

**New code**:
```rust
let combined_e = config.restitution_rule.combine(
    snap.restitution,
    other.restitution,
);
let e = combined_e.clamp(config.min_restitution, config.max_restitution);
let reflected = math::reflect(current_vel, hit.normal);
let new_vel = math::scale(reflected, e);
```

#### 2f. `Engine::physics_step()` -- pass collision config

**Current**:
```rust
pub fn physics_step(&mut self, dt: f64) {
    crate::systems::force_accumulator::run(&mut self.world);
    crate::systems::integrator::run(&mut self.world, dt);
    crate::systems::collision::run(&mut self.world, &mut self.events, dt);
}
```

**New**:
```rust
pub fn physics_step(&mut self, dt: f64) {
    crate::systems::force_accumulator::run(&mut self.world);
    crate::systems::integrator::run(&mut self.world, dt);
    crate::systems::collision::run(
        &mut self.world, &mut self.events, dt, &self.collision_config,
    );
}
```

#### 2g. `EngineSnapshot` -- include `CollisionConfig`

Add to `EngineSnapshot` (from Gap 4):

```rust
pub struct EngineSnapshot {
    // ... existing fields ...
    pub collision_config: CollisionConfig,
}
```

This ensures the overworld's collision rules (probably `Min` or `Average`) are preserved when pushing a fight scene that uses `Max`.

### 3. Integration with Existing Patterns

#### `ComponentStore` / `with_engine` pattern
No new component stores needed. `CollisionConfig` lives on `Engine`, not in the ECS. This follows the same pattern as `PostFxConfig`, `WorldConfig`, and `Camera` -- engine-level configuration structs that affect system behavior globally.

#### `PhysicsMaterial` component
Already exists with `restitution_override: Option<f64>`. The only change is that the collision system now actually reads it. No changes to the component itself.

#### Snapshot-then-commit pattern in collision.rs
The change is entirely within the snapshot phase (reading `PhysicsMaterial`) and the resolution phase (using `RestitutionRule`). The commit phase (writing back positions/velocities) is unchanged. This respects the established pattern documented in `CLAUDE.md`: "Snapshot-then-commit in collision.rs: collect data into Vec, process, write back in separate loop."

#### Game-level usage in Trap Links

```rust
// In fight scene setup:
eng.collision_config = CollisionConfig {
    restitution_rule: RestitutionRule::Max,
    max_restitution: 2.0,
    min_restitution: 0.0,
};

// Bumper entity setup:
let bumper = world.spawn();
world.transforms.insert(bumper, Transform { x: 200.0, y: 300.0, ..Default::default() });
world.colliders.insert(bumper, Collider {
    shape: ColliderShape::Circle { radius: 18.0 },
    is_trigger: false,
});
world.rigidbodies.insert(bumper, RigidBody {
    is_static: true,
    restitution: 1.4,   // amplifying bounce
    ..Default::default()
});
// OR use PhysicsMaterial for the override:
world.physics_materials.insert(bumper, PhysicsMaterial {
    restitution_override: Some(1.4),
    ..Default::default()
});

// Ball entity setup:
let ball = world.spawn();
world.rigidbodies.insert(ball, RigidBody {
    mass: 1.0,
    restitution: 0.85,  // slightly bouncy
    ..Default::default()
});

// Collision: Max(0.85, 1.4) = 1.4
// Ball bounces with 140% of incoming speed -- amplified!
```

### 4. Test Cases

#### Test 7.1: `RestitutionRule::Min` gives minimum
```
Assert: RestitutionRule::Min.combine(0.3, 0.8) == 0.3
Assert: RestitutionRule::Min.combine(1.0, 0.5) == 0.5
Assert: RestitutionRule::Min.combine(0.0, 1.0) == 0.0
```

#### Test 7.2: `RestitutionRule::Max` gives maximum
```
Assert: RestitutionRule::Max.combine(0.3, 0.8) == 0.8
Assert: RestitutionRule::Max.combine(1.4, 0.9) == 1.4
Assert: RestitutionRule::Max.combine(0.0, 0.0) == 0.0
```

#### Test 7.3: `RestitutionRule::Average` gives arithmetic mean
```
Assert: RestitutionRule::Average.combine(0.4, 0.6) == 0.5
Assert: RestitutionRule::Average.combine(1.0, 0.0) == 0.5
Assert: RestitutionRule::Average.combine(1.4, 0.8) == 1.1
```

#### Test 7.4: `RestitutionRule::Geometric` gives geometric mean
```
Assert: RestitutionRule::Geometric.combine(0.25, 1.0) == 0.5
Assert: RestitutionRule::Geometric.combine(4.0, 1.0) == 2.0
Assert: (RestitutionRule::Geometric.combine(0.5, 0.5) - 0.5).abs() < 1e-10
```

#### Test 7.5: `RestitutionRule::Multiply` gives product
```
Assert: RestitutionRule::Multiply.combine(0.5, 0.5) == 0.25
Assert: RestitutionRule::Multiply.combine(2.0, 0.5) == 1.0
Assert: RestitutionRule::Multiply.combine(0.0, 999.0) == 0.0
```

#### Test 7.6: `PhysicsMaterial.restitution_override` takes precedence over `RigidBody.restitution`
```
Setup: World with entity E.
  - RigidBody { restitution: 0.5, ... }
  - PhysicsMaterial { restitution_override: Some(1.2), ... }
  - Collider (Circle, radius 10)
  - Transform at (0, 0)
Setup: Static wall entity W at (20, 0), Rect collider, RigidBody is_static=true, restitution=0.3.
Action: Move E toward W with vx=100. Run collision system with RestitutionRule::Max.
Assert: Effective restitution for E is 1.2 (from PhysicsMaterial override, not 0.5 from RigidBody).
Assert: Combined restitution = max(1.2, 0.3) = 1.2.
Assert: Post-collision speed is approximately 1.2 * 100 = 120 (amplified).
```

#### Test 7.7: `restitution_override = None` falls back to `RigidBody.restitution`
```
Setup: Entity with RigidBody { restitution: 0.7, ... }, PhysicsMaterial { restitution_override: None, ... }.
Action: Snapshot the entity in collision system.
Assert: EntitySnap.restitution == 0.7 (from RigidBody).
```

#### Test 7.8: Entity without PhysicsMaterial uses RigidBody restitution
```
Setup: Entity with RigidBody { restitution: 0.6 }, NO PhysicsMaterial component.
Action: Snapshot the entity in collision system.
Assert: EntitySnap.restitution == 0.6.
```

#### Test 7.9: `max_restitution` clamp prevents infinite energy
```
Setup: Ball with restitution 1.5. Bumper with restitution 1.8.
Config: CollisionConfig { restitution_rule: Max, max_restitution: 2.0, ... }
Action: Collision occurs.
Assert: Combined restitution = max(1.5, 1.8) = 1.8 (under cap, no clamp).

Setup: Ball restitution 2.5, Bumper restitution 3.0.
Config: Same (max_restitution: 2.0).
Assert: Combined restitution = max(2.5, 3.0) = 3.0, clamped to 2.0.
```

#### Test 7.10: `min_restitution` floor prevents dead stops
```
Setup: Ball restitution 0.0. Wall restitution 0.0.
Config: CollisionConfig { restitution_rule: Min, min_restitution: 0.1, ... }
Action: Collision occurs.
Assert: Combined = min(0.0, 0.0) = 0.0, clamped up to 0.1.
Assert: Ball still has 10% of reflected speed (not dead stop).
```

#### Test 7.11: Bumper amplification end-to-end
```
Setup: Ball at (0, 0) moving right at vx=100, vy=0, radius=5, restitution=0.9.
  Static bumper at (50, 0), Circle radius=10, restitution=1.4.
Config: RestitutionRule::Max, max_restitution=2.0.
Action: Run collision system for one step (dt large enough for ball to reach bumper).
Assert: Ball's vx is approximately -140 (reflected and amplified by 1.4).
Assert: Ball's vy is approximately 0 (head-on collision, no y-component change).
Assert: Collision event emitted with correct entities and normal.
```

#### Test 7.12: Multiple bounces with different restitutions
```
Setup: Ball at (100, 0) moving right at vx=200.
  Wall A at (200, 0), restitution=0.5. Wall B at (0, 0), restitution=1.2.
Config: RestitutionRule::Max.
Action: Ball hits wall A. Expected combined_e = max(ball_rest, 0.5).
  Assume ball_rest=0.8. combined = 0.8. Ball speed after bounce = 0.8 * 200 = 160.
  Ball now moves left at vx=-160.
  Ball hits wall B. combined = max(0.8, 1.2) = 1.2. Speed = 1.2 * 160 = 192.
Assert: After 2 bounces, ball moves right at approximately vx=192.
```

#### Test 7.13: `CollisionConfig::default()` values are correct
```
Assert: CollisionConfig::default().restitution_rule == RestitutionRule::Max.
Assert: CollisionConfig::default().max_restitution == 2.0.
Assert: CollisionConfig::default().min_restitution == 0.0.
```

### 5. Estimated Lines of Code

| Item | LOC |
|------|-----|
| `RestitutionRule` enum + `combine()` + `Default` | ~35 |
| `CollisionConfig` struct + `Default` | ~18 |
| `Engine` struct: add `collision_config` field | ~3 |
| `Engine::new()`: initialize field | ~1 |
| `Engine::reset_game_state()`: reset field | ~1 |
| `Engine::physics_step()`: pass config | ~2 |
| `collision::run()`: signature change | ~2 |
| `collision::run()`: snapshot `PhysicsMaterial` lookup | ~6 |
| `collision::run()`: use `RestitutionRule` + clamp | ~5 |
| `collision::run()`: destructure `physics_materials` | ~1 |
| `EngineSnapshot`: add `collision_config` field | ~2 |
| SchemaInfo for `RestitutionRule` (if desired) | ~15 |
| Test cases (13 tests) | ~250 |
| **Total** | **~341** |

---

## Combined Summary

| Gap | Core Change | Files Modified | New Files | Est. LOC |
|-----|------------|---------------|-----------|----------|
| Gap 4 | Scene push/pop carries full world snapshot | `scene_manager.rs`, `world.rs`, `engine.rs` | None (all in existing files) | ~470 |
| Gap 7 | Per-entity restitution + configurable combination rule | `collision.rs`, `engine.rs`, `components/physics_material.rs` | None | ~341 |
| **Total** | | | | **~811** |

### Implementation Order

1. **Gap 7 first.** It is self-contained: add `RestitutionRule`, modify `collision::run()`, add `CollisionConfig` to `Engine`. No structural changes to other systems. Tests are pure-logic (no engine setup required for rule tests). Can ship and validate immediately.

2. **Gap 4 second.** It depends on Gap 7 because `EngineSnapshot` needs to capture `CollisionConfig`. It also has broader reach (touching `World`, `SceneManager`, `Engine`, and requiring `Clone` on subsystems). The testing is more complex (requires multi-system setup). But the actual logic is straightforward -- it's mostly mechanical `clone` / `insert` loops.

### Risk Assessment

- **Gap 7 risk: LOW.** The change is additive (new enum, new struct, one modified function). The only breaking change is `collision::run()`'s signature gaining a `&CollisionConfig` parameter, which requires updating the one call site in `Engine::physics_step()`.

- **Gap 4 risk: MEDIUM.** The `Clone` requirement on engine subsystems (`TimerQueue`, `BehaviorRules`, `TileMap`) may surface types that do not trivially clone. If a subsystem contains closures or non-Clone types, we must either skip it in the snapshot or refactor. The `NameMap` field exposure also requires a minor API addition. The snapshot size could be significant for large worlds -- but Trap Links overworld is ~60x60 tiles with <100 entities, so memory is not a concern.
