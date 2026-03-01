use std::collections::{HashSet, HashMap};
use super::entity::Entity;
use super::component_store::ComponentStore;
use crate::components::*;

/// Bidirectional name↔entity mapping.
#[derive(Default)]
pub struct NameMap {
    name_to_entity: HashMap<String, Entity>,
    entity_to_name: HashMap<Entity, String>,
}

impl NameMap {
    pub fn insert(&mut self, name: String, entity: Entity) {
        if self.name_to_entity.contains_key(&name) {
            crate::log::warn(&format!("Duplicate entity name '{}' — overwriting", name));
        }
        self.name_to_entity.insert(name.clone(), entity);
        self.entity_to_name.insert(entity, name);
    }

    pub fn get_by_name(&self, name: &str) -> Option<Entity> {
        self.name_to_entity.get(name).copied()
    }

    pub fn get_name(&self, entity: Entity) -> Option<&str> {
        self.entity_to_name.get(&entity).map(|s| s.as_str())
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        if let Some(name) = self.entity_to_name.remove(&entity) {
            self.name_to_entity.remove(&name);
        }
    }

    pub fn clear(&mut self) {
        self.name_to_entity.clear();
        self.entity_to_name.clear();
    }
}

/// The world holds all entities and all component stores.
pub struct World {
    next_id: u64,
    pub alive: HashSet<Entity>,
    pub names: NameMap,

    pub transforms: ComponentStore<Transform>,
    pub rigidbodies: ComponentStore<RigidBody>,
    pub colliders: ComponentStore<Collider>,
    pub renderables: ComponentStore<Renderable>,
    pub force_fields: ComponentStore<ForceField>,
    pub tags: ComponentStore<Tags>,
    pub roles: ComponentStore<Role>,
    pub lifetimes: ComponentStore<Lifetime>,
    pub game_states: ComponentStore<GameState>,
    pub behaviors: ComponentStore<Behavior>,
}

impl World {
    pub fn new() -> Self {
        Self {
            next_id: 1, // Entity(0) is reserved
            alive: HashSet::new(),
            names: NameMap::default(),
            transforms: ComponentStore::new(),
            rigidbodies: ComponentStore::new(),
            colliders: ComponentStore::new(),
            renderables: ComponentStore::new(),
            force_fields: ComponentStore::new(),
            tags: ComponentStore::new(),
            roles: ComponentStore::new(),
            lifetimes: ComponentStore::new(),
            game_states: ComponentStore::new(),
            behaviors: ComponentStore::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        let id = self.next_id;
        self.next_id += 1;
        let entity = Entity(id);
        self.alive.insert(entity);
        entity
    }

    pub fn spawn_named(&mut self, name: &str) -> Entity {
        let entity = self.spawn();
        self.names.insert(name.to_string(), entity);
        entity
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.alive.remove(&entity);
        self.names.remove_entity(entity);
        self.transforms.remove(entity);
        self.rigidbodies.remove(entity);
        self.colliders.remove(entity);
        self.renderables.remove(entity);
        self.force_fields.remove(entity);
        self.tags.remove(entity);
        self.roles.remove(entity);
        self.lifetimes.remove(entity);
        self.game_states.remove(entity);
        self.behaviors.remove(entity);
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.alive.contains(&entity)
    }

    pub fn entity_count(&self) -> usize {
        self.alive.len()
    }

    pub fn clear(&mut self) {
        self.alive.clear();
        self.names.clear();
        self.transforms.clear();
        self.rigidbodies.clear();
        self.colliders.clear();
        self.renderables.clear();
        self.force_fields.clear();
        self.tags.clear();
        self.roles.clear();
        self.lifetimes.clear();
        self.game_states.clear();
        self.behaviors.clear();
        self.next_id = 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── World::spawn ───────────────────────────────────────────────

    #[test]
    fn spawn_returns_entity_starting_at_1() {
        let mut world = World::new();
        let e1 = world.spawn();
        assert_eq!(e1, Entity(1));
    }

    #[test]
    fn spawn_increments_ids() {
        let mut world = World::new();
        let e1 = world.spawn();
        let e2 = world.spawn();
        let e3 = world.spawn();
        assert_eq!(e1, Entity(1));
        assert_eq!(e2, Entity(2));
        assert_eq!(e3, Entity(3));
    }

    #[test]
    fn spawn_marks_entity_alive() {
        let mut world = World::new();
        let e = world.spawn();
        assert!(world.is_alive(e));
    }

    // ─── World::spawn_named ─────────────────────────────────────────

    #[test]
    fn spawn_named_adds_to_name_map() {
        let mut world = World::new();
        let e = world.spawn_named("player");
        assert_eq!(world.names.get_by_name("player"), Some(e));
        assert_eq!(world.names.get_name(e), Some("player"));
    }

    #[test]
    fn spawn_named_entity_is_alive() {
        let mut world = World::new();
        let e = world.spawn_named("npc");
        assert!(world.is_alive(e));
    }

    #[test]
    fn spawn_named_increments_ids_like_spawn() {
        let mut world = World::new();
        let e1 = world.spawn();
        let e2 = world.spawn_named("hero");
        assert_eq!(e1, Entity(1));
        assert_eq!(e2, Entity(2));
    }

    // ─── World::despawn ─────────────────────────────────────────────

    #[test]
    fn despawn_removes_from_alive() {
        let mut world = World::new();
        let e = world.spawn();
        world.despawn(e);
        assert!(!world.is_alive(e));
    }

    #[test]
    fn despawn_removes_components_from_all_stores() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform::default());
        world.tags.insert(e, Tags::new(&["enemy"]));
        world.despawn(e);
        assert!(!world.transforms.has(e));
        assert!(!world.tags.has(e));
    }

    #[test]
    fn despawn_removes_from_name_map() {
        let mut world = World::new();
        let e = world.spawn_named("boss");
        world.despawn(e);
        assert_eq!(world.names.get_by_name("boss"), None);
        assert_eq!(world.names.get_name(e), None);
    }

    #[test]
    fn despawn_nonexistent_entity_is_safe() {
        let mut world = World::new();
        // Should not panic
        world.despawn(Entity(999));
    }

    #[test]
    fn despawn_does_not_affect_other_entities() {
        let mut world = World::new();
        let e1 = world.spawn();
        let e2 = world.spawn();
        world.transforms.insert(e1, Transform::default());
        world.transforms.insert(e2, Transform { x: 5.0, ..Transform::default() });
        world.despawn(e1);
        assert!(world.is_alive(e2));
        assert!(world.transforms.has(e2));
        assert_eq!(world.transforms.get(e2).unwrap().x, 5.0);
    }

    // ─── World::is_alive ────────────────────────────────────────────

    #[test]
    fn is_alive_false_for_never_spawned() {
        let world = World::new();
        assert!(!world.is_alive(Entity(1)));
        assert!(!world.is_alive(Entity(0)));
    }

    #[test]
    fn is_alive_true_after_spawn_false_after_despawn() {
        let mut world = World::new();
        let e = world.spawn();
        assert!(world.is_alive(e));
        world.despawn(e);
        assert!(!world.is_alive(e));
    }

    // ─── World::entity_count ────────────────────────────────────────

    #[test]
    fn entity_count_starts_at_zero() {
        let world = World::new();
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    fn entity_count_reflects_spawns() {
        let mut world = World::new();
        world.spawn();
        assert_eq!(world.entity_count(), 1);
        world.spawn();
        assert_eq!(world.entity_count(), 2);
        world.spawn_named("x");
        assert_eq!(world.entity_count(), 3);
    }

    #[test]
    fn entity_count_reflects_despawns() {
        let mut world = World::new();
        let e1 = world.spawn();
        let _e2 = world.spawn();
        world.despawn(e1);
        assert_eq!(world.entity_count(), 1);
    }

    // ─── World::clear ───────────────────────────────────────────────

    #[test]
    fn clear_resets_entity_count_to_zero() {
        let mut world = World::new();
        world.spawn();
        world.spawn();
        world.clear();
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    fn clear_resets_id_counter() {
        let mut world = World::new();
        world.spawn();
        world.spawn();
        world.clear();
        let e = world.spawn();
        assert_eq!(e, Entity(1));
    }

    #[test]
    fn clear_removes_all_components() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform::default());
        world.tags.insert(e, Tags::new(&["a"]));
        world.clear();
        assert_eq!(world.transforms.len(), 0);
        assert_eq!(world.tags.len(), 0);
    }

    #[test]
    fn clear_removes_names() {
        let mut world = World::new();
        world.spawn_named("foo");
        world.clear();
        assert_eq!(world.names.get_by_name("foo"), None);
    }

    #[test]
    fn clear_makes_previous_entities_not_alive() {
        let mut world = World::new();
        let e = world.spawn();
        world.clear();
        assert!(!world.is_alive(e));
    }

    // ─── NameMap ────────────────────────────────────────────────────

    #[test]
    fn name_map_insert_and_get_by_name() {
        let mut nm = NameMap::default();
        nm.insert("hero".to_string(), Entity(1));
        assert_eq!(nm.get_by_name("hero"), Some(Entity(1)));
    }

    #[test]
    fn name_map_get_name() {
        let mut nm = NameMap::default();
        nm.insert("villain".to_string(), Entity(2));
        assert_eq!(nm.get_name(Entity(2)), Some("villain"));
    }

    #[test]
    fn name_map_get_by_name_missing_returns_none() {
        let nm = NameMap::default();
        assert_eq!(nm.get_by_name("nobody"), None);
    }

    #[test]
    fn name_map_get_name_missing_returns_none() {
        let nm = NameMap::default();
        assert_eq!(nm.get_name(Entity(99)), None);
    }

    #[test]
    fn name_map_remove_entity() {
        let mut nm = NameMap::default();
        nm.insert("player".to_string(), Entity(1));
        nm.remove_entity(Entity(1));
        assert_eq!(nm.get_by_name("player"), None);
        assert_eq!(nm.get_name(Entity(1)), None);
    }

    #[test]
    fn name_map_remove_nonexistent_is_safe() {
        let mut nm = NameMap::default();
        // Should not panic
        nm.remove_entity(Entity(42));
    }

    #[test]
    fn name_map_clear() {
        let mut nm = NameMap::default();
        nm.insert("a".to_string(), Entity(1));
        nm.insert("b".to_string(), Entity(2));
        nm.clear();
        assert_eq!(nm.get_by_name("a"), None);
        assert_eq!(nm.get_by_name("b"), None);
        assert_eq!(nm.get_name(Entity(1)), None);
        assert_eq!(nm.get_name(Entity(2)), None);
    }

    #[test]
    fn name_map_overwrite_duplicate_name() {
        let mut nm = NameMap::default();
        nm.insert("shared".to_string(), Entity(1));
        nm.insert("shared".to_string(), Entity(2));
        // The name should now point to Entity(2)
        assert_eq!(nm.get_by_name("shared"), Some(Entity(2)));
        assert_eq!(nm.get_name(Entity(2)), Some("shared"));
    }

    #[test]
    fn name_map_multiple_entities() {
        let mut nm = NameMap::default();
        nm.insert("alpha".to_string(), Entity(10));
        nm.insert("beta".to_string(), Entity(20));
        nm.insert("gamma".to_string(), Entity(30));
        assert_eq!(nm.get_by_name("alpha"), Some(Entity(10)));
        assert_eq!(nm.get_by_name("beta"), Some(Entity(20)));
        assert_eq!(nm.get_by_name("gamma"), Some(Entity(30)));
    }

    // ─── Integration: despawn cleans up everything ──────────────────

    #[test]
    fn despawn_full_cleanup() {
        let mut world = World::new();
        let e = world.spawn_named("target");
        world.transforms.insert(e, Transform { x: 1.0, y: 2.0, rotation: 0.0, scale: 1.0 });
        world.tags.insert(e, Tags::new(&["enemy", "boss"]));

        world.despawn(e);

        assert!(!world.is_alive(e));
        assert!(!world.transforms.has(e));
        assert!(!world.tags.has(e));
        assert_eq!(world.names.get_by_name("target"), None);
        assert_eq!(world.names.get_name(e), None);
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    fn spawn_after_despawn_uses_new_id() {
        let mut world = World::new();
        let e1 = world.spawn();
        world.despawn(e1);
        let e2 = world.spawn();
        // IDs keep incrementing, not recycled
        assert_eq!(e2, Entity(2));
        assert!(!world.is_alive(e1));
        assert!(world.is_alive(e2));
    }
}
