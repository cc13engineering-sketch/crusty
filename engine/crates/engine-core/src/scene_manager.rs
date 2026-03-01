use std::collections::HashMap;
use crate::ecs::World;

/// A cloned snapshot of the entire World, used to preserve state
/// when pushing into a sub-scene (e.g. fight encounter) so the
/// overworld can be restored exactly when the sub-scene pops.
#[derive(Clone, Debug)]
pub struct WorldSnapshot {
    world: World,
}

impl WorldSnapshot {
    /// Capture a snapshot by cloning the given world.
    pub fn capture(world: &World) -> Self {
        Self { world: world.clone() }
    }

    /// Consume the snapshot and return the cloned world.
    pub fn restore(self) -> World {
        self.world
    }

    /// Peek at the entity count without consuming the snapshot.
    pub fn entity_count(&self) -> usize {
        self.world.entity_count()
    }
}

/// Entry in the scene stack.
#[derive(Clone, Debug)]
pub struct SceneStackEntry {
    pub name: String,
    pub world_snapshot: Option<WorldSnapshot>,
}

/// Named scene registry with push/pop stack semantics.
/// Scenes are stored as .world file source strings.
#[derive(Clone, Debug)]
pub struct SceneManager {
    registry: HashMap<String, String>,
    stack: Vec<SceneStackEntry>,
}

impl Default for SceneManager {
    fn default() -> Self {
        Self { registry: HashMap::new(), stack: Vec::new() }
    }
}

impl SceneManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a scene by name with its .world source.
    pub fn register(&mut self, name: &str, source: &str) {
        self.registry.insert(name.to_string(), source.to_string());
    }

    /// Remove a scene from the registry.
    pub fn unregister(&mut self, name: &str) -> bool {
        self.registry.remove(name).is_some()
    }

    /// Check if a scene is registered.
    pub fn has(&self, name: &str) -> bool {
        self.registry.contains_key(name)
    }

    /// Get the source for a named scene.
    pub fn get_source(&self, name: &str) -> Option<&str> {
        self.registry.get(name).map(|s| s.as_str())
    }

    /// Push a scene onto the stack. Returns false if scene not registered.
    pub fn push(&mut self, name: &str) -> bool {
        if self.registry.contains_key(name) {
            self.stack.push(SceneStackEntry {
                name: name.to_string(),
                world_snapshot: None,
            });
            true
        } else {
            false
        }
    }

    /// Push a scene onto the stack while capturing a snapshot of the current
    /// world state. When this scene is later popped with `pop_with_restore`,
    /// the snapshot is returned so the caller can restore the world.
    /// Returns false if the scene is not registered.
    pub fn push_with_snapshot(&mut self, name: &str, world: &World) -> bool {
        if self.registry.contains_key(name) {
            self.stack.push(SceneStackEntry {
                name: name.to_string(),
                world_snapshot: Some(WorldSnapshot::capture(world)),
            });
            true
        } else {
            false
        }
    }

    /// Pop the top scene. Returns the name of the popped scene.
    pub fn pop(&mut self) -> Option<String> {
        self.stack.pop().map(|e| e.name)
    }

    /// Pop the top scene, returning its name and any stored world snapshot.
    /// If the scene was pushed with `push_with_snapshot`, the second tuple
    /// element contains the cloned world to restore. Otherwise it is None.
    pub fn pop_with_restore(&mut self) -> Option<(String, Option<World>)> {
        self.stack.pop().map(|entry| {
            let restored = entry.world_snapshot.map(|snap| snap.restore());
            (entry.name, restored)
        })
    }

    /// Get the current (top) scene name.
    pub fn current(&self) -> Option<&str> {
        self.stack.last().map(|e| e.name.as_str())
    }

    /// Get the current scene's source.
    pub fn current_source(&self) -> Option<&str> {
        self.current().and_then(|name| self.get_source(name))
    }

    /// Get the stack depth.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// Replace the current scene with a new one.
    pub fn replace(&mut self, name: &str) -> bool {
        if !self.registry.contains_key(name) {
            return false;
        }
        self.stack.pop();
        self.stack.push(SceneStackEntry { name: name.to_string(), world_snapshot: None });
        true
    }

    /// Clear the entire stack.
    pub fn clear_stack(&mut self) {
        self.stack.clear();
    }

    /// Clear everything (registry + stack).
    pub fn clear(&mut self) {
        self.registry.clear();
        self.stack.clear();
    }

    /// List all registered scene names.
    pub fn scene_names(&self) -> Vec<&str> {
        self.registry.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_empty() {
        let sm = SceneManager::new();
        assert_eq!(sm.depth(), 0);
        assert!(sm.current().is_none());
    }

    #[test]
    fn register_and_has() {
        let mut sm = SceneManager::new();
        sm.register("level1", "world \"Level 1\" {}");
        assert!(sm.has("level1"));
        assert!(!sm.has("level2"));
    }

    #[test]
    fn get_source() {
        let mut sm = SceneManager::new();
        sm.register("test", "world \"Test\" {}");
        assert_eq!(sm.get_source("test"), Some("world \"Test\" {}"));
        assert_eq!(sm.get_source("missing"), None);
    }

    #[test]
    fn unregister() {
        let mut sm = SceneManager::new();
        sm.register("level1", "...");
        assert!(sm.unregister("level1"));
        assert!(!sm.has("level1"));
        assert!(!sm.unregister("level1"));
    }

    #[test]
    fn push_registered_scene() {
        let mut sm = SceneManager::new();
        sm.register("main", "...");
        assert!(sm.push("main"));
        assert_eq!(sm.depth(), 1);
        assert_eq!(sm.current(), Some("main"));
    }

    #[test]
    fn push_unregistered_fails() {
        let mut sm = SceneManager::new();
        assert!(!sm.push("nonexistent"));
        assert_eq!(sm.depth(), 0);
    }

    #[test]
    fn pop_returns_scene_name() {
        let mut sm = SceneManager::new();
        sm.register("a", "...");
        sm.push("a");
        assert_eq!(sm.pop(), Some("a".to_string()));
        assert_eq!(sm.depth(), 0);
    }

    #[test]
    fn pop_empty_returns_none() {
        let mut sm = SceneManager::new();
        assert_eq!(sm.pop(), None);
    }

    #[test]
    fn push_pop_stack_semantics() {
        let mut sm = SceneManager::new();
        sm.register("a", "...");
        sm.register("b", "...");
        sm.push("a");
        sm.push("b");
        assert_eq!(sm.current(), Some("b"));
        assert_eq!(sm.depth(), 2);
        sm.pop();
        assert_eq!(sm.current(), Some("a"));
        assert_eq!(sm.depth(), 1);
    }

    #[test]
    fn current_source_returns_world_data() {
        let mut sm = SceneManager::new();
        sm.register("level1", "world \"Level 1\" { bounds: 800 x 600 }");
        sm.push("level1");
        assert_eq!(sm.current_source(), Some("world \"Level 1\" { bounds: 800 x 600 }"));
    }

    #[test]
    fn replace_scene() {
        let mut sm = SceneManager::new();
        sm.register("a", "...");
        sm.register("b", "...");
        sm.push("a");
        assert!(sm.replace("b"));
        assert_eq!(sm.current(), Some("b"));
        assert_eq!(sm.depth(), 1);
    }

    #[test]
    fn replace_unregistered_fails() {
        let mut sm = SceneManager::new();
        sm.register("a", "...");
        sm.push("a");
        assert!(!sm.replace("missing"));
        assert_eq!(sm.current(), Some("a"));
    }

    #[test]
    fn clear_stack() {
        let mut sm = SceneManager::new();
        sm.register("a", "...");
        sm.register("b", "...");
        sm.push("a");
        sm.push("b");
        sm.clear_stack();
        assert_eq!(sm.depth(), 0);
        assert!(sm.has("a")); // registry unaffected
    }

    #[test]
    fn clear_everything() {
        let mut sm = SceneManager::new();
        sm.register("a", "...");
        sm.push("a");
        sm.clear();
        assert_eq!(sm.depth(), 0);
        assert!(!sm.has("a"));
    }

    #[test]
    fn scene_names_lists_all() {
        let mut sm = SceneManager::new();
        sm.register("alpha", "...");
        sm.register("beta", "...");
        let mut names = sm.scene_names();
        names.sort();
        assert_eq!(names, vec!["alpha", "beta"]);
    }

    #[test]
    fn clone_and_debug() {
        let mut sm = SceneManager::new();
        sm.register("x", "...");
        sm.push("x");
        let cloned = sm.clone();
        let debug = format!("{:?}", cloned);
        assert!(debug.contains("SceneManager"));
    }

    // ─── World snapshot save/restore ───────────────────────────────

    #[test]
    fn push_with_snapshot_stores_world_state() {
        use crate::ecs::World;
        use crate::components::Transform;

        let mut sm = SceneManager::new();
        sm.register("fight", "...");

        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 10.0, y: 20.0, rotation: 0.0, scale: 1.0 });
        world.spawn();
        world.spawn();

        assert!(sm.push_with_snapshot("fight", &world));
        assert_eq!(sm.depth(), 1);
        assert_eq!(sm.current(), Some("fight"));

        // The snapshot should be present in the stack entry
        let entry = sm.stack.last().expect("stack should not be empty");
        assert!(entry.world_snapshot.is_some());
        assert_eq!(entry.world_snapshot.as_ref().map(|s| s.entity_count()), Some(3));
    }

    #[test]
    fn push_with_snapshot_unregistered_fails() {
        use crate::ecs::World;

        let mut sm = SceneManager::new();
        let world = World::new();

        assert!(!sm.push_with_snapshot("missing", &world));
        assert_eq!(sm.depth(), 0);
    }

    #[test]
    fn pop_with_restore_returns_cloned_world() {
        use crate::ecs::World;
        use crate::components::Transform;

        let mut sm = SceneManager::new();
        sm.register("fight", "...");

        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 42.0, y: 99.0, rotation: 0.0, scale: 1.0 });

        sm.push_with_snapshot("fight", &world);

        let result = sm.pop_with_restore();
        assert!(result.is_some());

        let (name, restored_world) = result.expect("pop should succeed");
        assert_eq!(name, "fight");
        assert!(restored_world.is_some());

        let restored = restored_world.expect("snapshot should be present");
        assert_eq!(restored.entity_count(), 1);
        assert!(restored.is_alive(e));

        if let Some(t) = restored.transforms.get(e) {
            assert!((t.x - 42.0).abs() < f64::EPSILON);
            assert!((t.y - 99.0).abs() < f64::EPSILON);
        } else {
            panic!("Transform should be restored");
        }
    }

    #[test]
    fn entity_counts_preserved_across_snapshot_restore() {
        use crate::ecs::World;
        use crate::components::{Transform, Tags};

        let mut sm = SceneManager::new();
        sm.register("overworld", "...");
        sm.register("fight", "...");

        // Build an overworld with 5 entities
        let mut world = World::new();
        for i in 0..5 {
            let e = world.spawn();
            world.transforms.insert(e, Transform {
                x: i as f64 * 10.0,
                y: 0.0,
                rotation: 0.0,
                scale: 1.0,
            });
            world.tags.insert(e, Tags::new(&["npc"]));
        }
        assert_eq!(world.entity_count(), 5);

        // Push into fight scene, snapshot overworld
        sm.push_with_snapshot("fight", &world);

        // Mutate the live world (simulate fight scene clearing overworld)
        world.clear();
        let fight_entity = world.spawn();
        world.transforms.insert(fight_entity, Transform::default());
        assert_eq!(world.entity_count(), 1);

        // Pop fight scene, restore overworld
        let (_, restored) = sm.pop_with_restore().expect("pop should succeed");
        let restored_world = restored.expect("should have snapshot");

        // Restored world should have original 5 entities
        assert_eq!(restored_world.entity_count(), 5);
        // And the tags should be intact
        assert_eq!(restored_world.tags.len(), 5);
    }

    #[test]
    fn nested_push_pop_with_snapshots() {
        use crate::ecs::World;

        let mut sm = SceneManager::new();
        sm.register("overworld", "...");
        sm.register("dungeon", "...");
        sm.register("fight", "...");

        // Overworld: 3 entities
        let mut world = World::new();
        world.spawn();
        world.spawn();
        world.spawn();
        assert_eq!(world.entity_count(), 3);

        // Push overworld -> dungeon
        sm.push_with_snapshot("dungeon", &world);

        // Dungeon: different world with 2 entities
        world.clear();
        world.spawn();
        world.spawn();
        assert_eq!(world.entity_count(), 2);

        // Push dungeon -> fight
        sm.push_with_snapshot("fight", &world);
        assert_eq!(sm.depth(), 2);

        // Fight: clear and add 1
        world.clear();
        world.spawn();
        assert_eq!(world.entity_count(), 1);

        // Pop fight -> restore dungeon (2 entities)
        let (name, restored) = sm.pop_with_restore().expect("pop fight");
        assert_eq!(name, "fight");
        let dungeon_world = restored.expect("dungeon snapshot");
        assert_eq!(dungeon_world.entity_count(), 2);
        assert_eq!(sm.depth(), 1);

        // Pop dungeon -> restore overworld (3 entities)
        let (name, restored) = sm.pop_with_restore().expect("pop dungeon");
        assert_eq!(name, "dungeon");
        let overworld_world = restored.expect("overworld snapshot");
        assert_eq!(overworld_world.entity_count(), 3);
        assert_eq!(sm.depth(), 0);
    }

    #[test]
    fn pop_without_snapshot_returns_none_for_world() {
        let mut sm = SceneManager::new();
        sm.register("menu", "...");
        sm.push("menu"); // plain push, no snapshot

        let result = sm.pop_with_restore();
        assert!(result.is_some());

        let (name, restored_world) = result.expect("pop should succeed");
        assert_eq!(name, "menu");
        assert!(restored_world.is_none());
    }

    #[test]
    fn pop_with_restore_empty_returns_none() {
        let mut sm = SceneManager::new();
        assert!(sm.pop_with_restore().is_none());
    }

    #[test]
    fn snapshot_is_independent_of_original_world() {
        use crate::ecs::World;
        use crate::components::Transform;

        let mut sm = SceneManager::new();
        sm.register("fight", "...");

        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 1.0, y: 2.0, rotation: 0.0, scale: 1.0 });

        sm.push_with_snapshot("fight", &world);

        // Modify the original world after snapshotting
        if let Some(t) = world.transforms.get_mut(e) {
            t.x = 999.0;
            t.y = 888.0;
        }
        world.spawn();
        world.spawn();

        // Snapshot should not be affected by subsequent mutations
        let (_, restored) = sm.pop_with_restore().expect("pop");
        let snap_world = restored.expect("snapshot");
        assert_eq!(snap_world.entity_count(), 1);
        if let Some(t) = snap_world.transforms.get(e) {
            assert!((t.x - 1.0).abs() < f64::EPSILON);
            assert!((t.y - 2.0).abs() < f64::EPSILON);
        } else {
            panic!("Transform should exist in snapshot");
        }
    }
}
