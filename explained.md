# How the Crusty ECS Core Works — A Plain-English Explainer

This document explains, in depth and in plain English, how the Entity Component System (ECS) core of the Crusty game engine works. It covers four files: `entity.rs`, `component_store.rs`, `world.rs`, and `schema.rs`.

---

## Part 1: Entity — The Universal ID Badge

**File:** `engine/crates/engine-core/src/ecs/entity.rs`

An `Entity` is the simplest possible thing: a number wrapped in a named type.

```rust
pub struct Entity(pub u64);
```

That's it. An entity is just a 64-bit unsigned integer (0 through 18 quintillion). The `pub` inside the parentheses means anyone can read the number directly with `entity.0`.

### Why wrap a number in a struct?

Type safety. If entities were raw `u64` values, you could accidentally pass a framebuffer width where an entity ID was expected and the compiler wouldn't catch it. By wrapping it in `Entity(...)`, the compiler enforces that you can only use an `Entity` where an `Entity` is expected.

### The derive line

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
```

This single line asks the Rust compiler to automatically generate a bunch of standard behaviors:

- **Clone** and **Copy**: You can duplicate an entity freely. `let b = a;` makes a copy — it doesn't move ownership. This is critical because entity IDs get passed around constantly and you don't want the original to become unusable.
- **Debug**: You can print it for debugging (`Entity(42)`).
- **PartialEq** and **Eq**: You can compare two entities (`a == b`). Two entities are equal if and only if they hold the same number.
- **PartialOrd** and **Ord**: You can sort entities by their ID, which matters for deterministic iteration (always processing entities in the same order).
- **Hash**: You can use entities as keys in HashMaps and HashSets, which is how component storage works (more on that below).
- **Serialize** and **Deserialize**: You can convert entities to/from JSON for save files and replays.

### The reserved zero

Entity(0) is never assigned to anything. The World starts its counter at 1. This means code can use `Entity(0)` as a sentinel value meaning "no entity" without conflicting with a real one.

---

## Part 2: ComponentStore — The Filing Cabinet

**File:** `engine/crates/engine-core/src/ecs/component_store.rs`

A `ComponentStore<T>` is a generic container that maps entities to components of a single type. Think of it as a filing cabinet where each drawer is labeled with an entity ID and contains exactly one item of type `T`.

```rust
pub struct ComponentStore<T> {
    data: HashMap<Entity, T>,
}
```

Under the hood, it's a HashMap — a hash table that lets you look up, insert, and remove items in roughly constant time. The `data` field is private (no `pub`), so the outside world must use the provided methods.

### The generic `<T>`

The `T` means "any type." The same ComponentStore code works for storing Transform components, RigidBody components, Tags, or anything else. You don't write separate storage code for each component type — you get one implementation that works for all of them.

### Manual trait implementations

Most Rust types use `#[derive(...)]` to automatically implement standard traits. ComponentStore does it by hand for a good reason:

```rust
impl<T: Clone> Clone for ComponentStore<T> { ... }
```

This says "ComponentStore can be cloned, but only if the thing it stores can also be cloned." This is a *conditional* implementation. If someone tried to store a non-cloneable type, they'd still be able to use ComponentStore for everything except cloning.

```rust
impl<T> Default for ComponentStore<T> {
    fn default() -> Self {
        Self { data: HashMap::new() }
    }
}
```

This says "you can create an empty ComponentStore for any type T, even if T itself doesn't have a default value." The derive macro would have required `T: Default`, which is an unnecessary restriction since an empty HashMap doesn't need to know anything about T.

### The core operations

The methods are intentionally simple — thin wrappers over HashMap:

- **`insert(entity, component)`** — Put a component into the drawer labeled with that entity. If there was already one there, it gets replaced silently.
- **`get(entity)`** — Look in the drawer. Returns `Some(&component)` if found, `None` if that entity doesn't have this component type.
- **`get_mut(entity)`** — Same as get, but returns a mutable reference so you can modify the component in place.
- **`remove(entity)`** — Pull the component out of the drawer and hand it back. The drawer is now empty.
- **`has(entity)`** — Just checks if the drawer exists, without opening it.
- **`iter()` / `iter_mut()`** — Walk through every entity-component pair. The `iter()` line does a small transformation: HashMap's iterator gives `(&Entity, &T)` (references to both), but since Entity is Copy, it dereferences the key to give `(Entity, &T)` — a plain entity value and a reference to the component. This is more ergonomic because you don't need to deal with `&Entity` everywhere.
- **`sorted_entities()`** — Collects all entity keys, sorts them by ID, and returns the sorted list. This exists for determinism: HashMap iteration order is random, but game systems need to process entities in the same order every time for reproducible simulations.
- **`len()` / `clear()`** — Count and empty the store, respectively.

---

## Part 3: World — The Entire Game Universe

**File:** `engine/crates/engine-core/src/ecs/world.rs`

The World struct is the central container that holds every entity and every component in the game. It's the single source of truth for the entire simulation state.

### NameMap: The Phonebook

Before we get to World itself, the file defines NameMap — a bidirectional lookup between string names and entities.

```rust
pub struct NameMap {
    name_to_entity: HashMap<String, Entity>,
    entity_to_name: HashMap<Entity, String>,
}
```

It maintains two HashMaps that mirror each other. When you insert a name-entity pair, both maps get updated. This lets you look up in either direction:

- "What entity is called 'player'?" → `get_by_name("player")` returns the Entity.
- "What's the name of Entity(7)?" → `get_name(Entity(7))` returns the string.

The `insert` method handles edge cases carefully. If you assign a name that already belongs to a different entity, it warns (via the engine's logging system) and overwrites. If an entity already had a different name, the old name gets cleaned up from the forward map. This prevents stale entries from accumulating.

When an entity is removed, `remove_entity` cleans up both directions. When the world resets, `clear` empties everything.

### The component_stores! Macro: Write Once, Use Everywhere

This is the most architecturally interesting part of the file. The engine has 31 different component types (Transform, RigidBody, Collider, etc.), and each one needs its own ComponentStore field in World. Without the macro, adding a new component type meant editing three separate places:

1. The struct definition (add the field)
2. The `new()` constructor (initialize it)
3. The `despawn()` method (remove the entity from it)
4. The `clear()` method (empty it)

Forgetting any one of these would cause bugs. The macro eliminates this by letting you declare all component stores in a single list:

```rust
macro_rules! component_stores {
    ($($field:ident : $Type:ty),* $(,)?) => {
        ...
    };
}
```

Let's break down the macro pattern:

- `$($field:ident : $Type:ty),*` — This matches a comma-separated list of `name: Type` pairs. `$field` captures the field name (like `transforms`), `$Type` captures the type (like `Transform`). The `*` means "zero or more repetitions."
- `$(,)?` — Allows an optional trailing comma, which is a Rust style convention.

When the macro runs, the `$( ... )*` sections inside the body expand once per pair. So this:

```rust
component_stores! {
    transforms: Transform,
    rigidbodies: RigidBody,
}
```

Expands into a struct with `pub transforms: ComponentStore<Transform>` and `pub rigidbodies: ComponentStore<RigidBody>` as fields, a constructor that initializes both to `ComponentStore::new()`, a `despawn_components` method that calls `.remove(entity)` on both, and a `clear_components` method that calls `.clear()` on both.

The actual invocation lists all 31 component types. Adding a 32nd component means adding one line to this list — the macro handles the rest.

### The World struct itself

After the macro expands, World has these fields:

- **`next_id: u64`** — A counter for the next entity ID to assign. Starts at 1 (since 0 is reserved). Private, so only World can increment it.
- **`alive: HashSet<Entity>`** — The set of all currently living entities. A HashSet gives O(1) lookup for "is this entity alive?"
- **`names: NameMap`** — The bidirectional name-entity mapping described above.
- **31 component stores** — One `ComponentStore<T>` for each component type, all public so systems can directly access them.
- **`spawn_queue: SpawnQueue`** — A queue for deferred entity creation (entities scheduled to be spawned next frame).

### Entity lifecycle: spawn, live, despawn, clear

**Spawning** is a two-step operation: increment the counter and register the new entity as alive.

```rust
pub fn spawn(&mut self) -> Entity {
    let id = self.next_id;
    self.next_id += 1;
    let entity = Entity(id);
    self.alive.insert(entity);
    entity
}
```

IDs never get recycled. If you spawn entity 5, despawn it, and spawn another, the new one gets entity 6, not 5. This prevents a category of bugs where old references to entity 5 accidentally point to the new occupant. The counter only resets when the entire world is cleared (game reset).

`spawn_named` is a convenience that spawns an entity and immediately registers a name for it.

**Despawning** removes the entity from three places: the alive set, the name map, and every component store. The `despawn_components` method (generated by the macro) handles the 31 component stores in one sweep. Each `.remove()` call is harmless if the entity didn't have that component — it just returns `None` and moves on.

**Clearing** is a total reset. It empties the alive set, the name map, all 31 component stores, and the spawn queue, then resets the ID counter back to 1. This is used when restarting a game or loading a new scene.

### How systems use World

Game systems (physics, collision, rendering, etc.) receive `&mut World` and directly access the component stores they need. For example, a physics system might do:

```rust
for (entity, rb) in world.rigidbodies.iter_mut() {
    if let Some(transform) = world.transforms.get_mut(entity) {
        transform.x += rb.vx * dt;
        transform.y += rb.vy * dt;
    }
}
```

This iterates all entities with a RigidBody, checks if they also have a Transform, and applies velocity. Entities without both components are naturally skipped — no special filtering needed.

---

## Part 4: Schema — Self-Describing the Engine

**File:** `engine/crates/engine-core/src/schema.rs`

This file has a single function that produces a JSON description of the entire engine's structure. It's used by tooling (the CLI `info` and `schema` commands) to inspect what components, systems, and engine subsystems exist.

```rust
pub fn generate_schema() -> String {
```

It builds a JSON object with three sections:

1. **`components`** — Lists all 31 component types with their names and field schemas. Each component type implements a `SchemaInfo` trait that provides `schema_name()` (the type name as a string) and `schema()` (a JSON description of its fields). Three newer components (ResourceInventory, GraphNode, VisualConnection) use inline descriptions instead of the trait.

2. **`systems`** — Lists all 17 ECS systems by name. These are the functions that run each frame to update the simulation.

3. **`engine_state`** — Lists all the engine-level subsystems (game state, timers, templates, scene manager, etc.) that sit alongside the ECS.

The final line serializes this JSON object into a pretty-printed string:

```rust
serde_json::to_string_pretty(&schema).unwrap_or_else(|e| {
    crate::log::error(&format!("schema serialization failed: {}", e));
    "{}".to_string()
})
```

The `unwrap_or_else` is a safety pattern. `serde_json::to_string_pretty` returns a `Result` — it could theoretically fail (though in practice it won't for a `serde_json::Value`). Rather than calling `.unwrap()` which would crash the program on failure, this code logs an error and returns an empty JSON object as a fallback. This follows the project convention of never using bare `.unwrap()` in production code.

---

## How It All Fits Together

The flow is:

1. **World is created** → all 31 component stores start empty, the ID counter is at 1.
2. **Entities are spawned** → each gets a unique incrementing ID, gets added to the alive set.
3. **Components are attached** → game code calls `world.transforms.insert(entity, Transform { ... })` to give an entity a position, `world.colliders.insert(entity, Collider { ... })` to give it collision, etc. An entity's "type" is defined by which components it has, not by any class hierarchy.
4. **Systems run each frame** → they iterate over component stores, read and write the data. A collision system looks at entities with both transforms and colliders. A rendering system looks at entities with both transforms and renderables. Each system only touches the stores it cares about.
5. **Entities are despawned** → the macro-generated method sweeps all 31 stores, removing that entity's data from each one. The entity ID is never reused.
6. **World is cleared** → everything resets. Fresh start. The cycle begins again.

The macro at the heart of `world.rs` ensures that this 31-store sweep stays in sync automatically. One list of component types, four generated operations, zero chance of forgetting one.
