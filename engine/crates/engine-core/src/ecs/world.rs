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
        self.transforms.remove(entity);
        self.rigidbodies.remove(entity);
        self.colliders.remove(entity);
        self.renderables.remove(entity);
        self.force_fields.remove(entity);
        self.tags.remove(entity);
        self.roles.remove(entity);
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
        self.next_id = 1;
    }
}
