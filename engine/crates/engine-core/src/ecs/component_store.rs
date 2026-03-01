use std::collections::HashMap;
use super::entity::Entity;

/// Typed storage for a single component type. One per component in World.
pub struct ComponentStore<T> {
    data: HashMap<Entity, T>,
}

// Manual Default — no T: Default bound required.
impl<T> Default for ComponentStore<T> {
    fn default() -> Self {
        Self { data: HashMap::new() }
    }
}

impl<T> ComponentStore<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, entity: Entity, component: T) {
        self.data.insert(entity, component);
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.data.get(&entity)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.data.get_mut(&entity)
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        self.data.remove(&entity)
    }

    pub fn has(&self, entity: Entity) -> bool {
        self.data.contains_key(&entity)
    }

    /// Note: HashMap iter yields (&K, &V). We dereference the key since Entity is Copy.
    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.data.iter().map(|(k, v)| (*k, v))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.data.iter_mut().map(|(k, v)| (*k, v))
    }

    pub fn entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.data.keys().copied()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns entities sorted by ID for deterministic iteration.
    pub fn sorted_entities(&self) -> Vec<Entity> {
        let mut v: Vec<Entity> = self.data.keys().copied().collect();
        v.sort_by_key(|e| e.0);
        v
    }
}
