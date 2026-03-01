use std::collections::{VecDeque, HashMap};
use crate::ecs::Entity;

/// A pool of pre-allocated entities for a specific template type.
/// Instead of spawning/despawning, entities are acquired/released.
/// Released entities have their components reset but the Entity ID is reused.
#[derive(Clone, Debug)]
pub struct EntityPool {
    pub template_name: String,
    available: VecDeque<Entity>,
    active: Vec<Entity>,
    capacity: usize,
}

impl EntityPool {
    /// Create a new pool for the given template name.
    pub fn new(template_name: &str, capacity: usize) -> Self {
        Self {
            template_name: template_name.to_string(),
            available: VecDeque::with_capacity(capacity),
            active: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// Pre-warm the pool by creating entities via world.spawn().
    /// Returns the list of entities that were spawned and placed in the available queue.
    /// The caller is responsible for setting up (and deactivating) each returned entity.
    pub fn prewarm(&mut self, world: &mut crate::ecs::World) -> Vec<Entity> {
        let slots_needed = self.capacity.saturating_sub(self.available.len() + self.active.len());
        let mut spawned = Vec::with_capacity(slots_needed);
        for _ in 0..slots_needed {
            let entity = world.spawn();
            self.available.push_back(entity);
            spawned.push(entity);
        }
        spawned
    }

    /// Acquire an entity from the pool (moves from available to active).
    /// Returns None if the pool is exhausted.
    pub fn acquire(&mut self) -> Option<Entity> {
        let entity = self.available.pop_front()?;
        self.active.push(entity);
        Some(entity)
    }

    /// Release an entity back to the pool (moves from active to available).
    /// Returns true if the entity was found in the active list and released,
    /// false if the entity is not managed by this pool or was not active.
    pub fn release(&mut self, entity: Entity) -> bool {
        if let Some(pos) = self.active.iter().position(|&e| e == entity) {
            self.active.swap_remove(pos);
            self.available.push_back(entity);
            true
        } else {
            false
        }
    }

    /// How many entities are available to acquire.
    pub fn available_count(&self) -> usize {
        self.available.len()
    }

    /// How many entities are currently active (acquired).
    pub fn active_count(&self) -> usize {
        self.active.len()
    }

    /// Total pool capacity (the pre-warmed size, not a hard ceiling).
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Is a specific entity managed by this pool (either available or active)?
    pub fn contains(&self, entity: Entity) -> bool {
        self.active.contains(&entity) || self.available.contains(&entity)
    }

    /// Is the pool exhausted (no entities available to acquire)?
    pub fn is_empty(&self) -> bool {
        self.available.is_empty()
    }

    /// Release all active entities back to the pool.
    pub fn release_all(&mut self) {
        let active = std::mem::take(&mut self.active);
        for entity in active {
            self.available.push_back(entity);
        }
    }
}

/// Registry of multiple entity pools, keyed by template name.
#[derive(Clone, Debug, Default)]
pub struct PoolRegistry {
    pools: HashMap<String, EntityPool>,
}

impl PoolRegistry {
    pub fn new() -> Self {
        Self {
            pools: HashMap::new(),
        }
    }

    /// Create a new pool for the given template name and return a mutable reference to it.
    /// If a pool with that name already exists, it is replaced.
    pub fn create_pool(&mut self, template_name: &str, capacity: usize) -> &mut EntityPool {
        self.pools.insert(template_name.to_string(), EntityPool::new(template_name, capacity));
        self.pools.get_mut(template_name).expect("just inserted")
    }

    /// Get a shared reference to a pool by template name.
    pub fn get(&self, template_name: &str) -> Option<&EntityPool> {
        self.pools.get(template_name)
    }

    /// Get a mutable reference to a pool by template name.
    pub fn get_mut(&mut self, template_name: &str) -> Option<&mut EntityPool> {
        self.pools.get_mut(template_name)
    }

    /// Acquire an entity from the named pool.
    /// Returns None if the pool does not exist or is exhausted.
    pub fn acquire(&mut self, template_name: &str) -> Option<Entity> {
        self.pools.get_mut(template_name)?.acquire()
    }

    /// Release an entity back to the named pool.
    /// Returns false if the pool does not exist or the entity is not active in it.
    pub fn release(&mut self, template_name: &str, entity: Entity) -> bool {
        match self.pools.get_mut(template_name) {
            Some(pool) => pool.release(entity),
            None => false,
        }
    }

    /// Check whether a pool with the given name exists in this registry.
    pub fn has_pool(&self, template_name: &str) -> bool {
        self.pools.contains_key(template_name)
    }

    /// Return the names of all registered pools.
    pub fn pool_names(&self) -> Vec<&str> {
        self.pools.keys().map(|s| s.as_str()).collect()
    }

    /// Sum of available counts across all pools.
    pub fn total_available(&self) -> usize {
        self.pools.values().map(|p| p.available_count()).sum()
    }

    /// Sum of active counts across all pools.
    pub fn total_active(&self) -> usize {
        self.pools.values().map(|p| p.active_count()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::World;

    // -----------------------------------------------------------------------
    // EntityPool tests
    // -----------------------------------------------------------------------

    #[test]
    fn entity_pool_new_creates_correct_state() {
        let pool = EntityPool::new("bullet", 10);
        assert_eq!(pool.template_name, "bullet");
        assert_eq!(pool.capacity(), 10);
        assert_eq!(pool.available_count(), 0);
        assert_eq!(pool.active_count(), 0);
        assert!(pool.is_empty());
    }

    #[test]
    fn prewarm_creates_entities_in_world() {
        let mut world = World::new();
        let mut pool = EntityPool::new("bullet", 5);
        let spawned = pool.prewarm(&mut world);
        assert_eq!(spawned.len(), 5);
        assert_eq!(pool.available_count(), 5);
        assert_eq!(pool.active_count(), 0);
        for entity in &spawned {
            assert!(world.is_alive(*entity));
        }
    }

    #[test]
    fn prewarm_does_not_exceed_capacity() {
        let mut world = World::new();
        let mut pool = EntityPool::new("enemy", 3);
        let first = pool.prewarm(&mut world);
        // Calling prewarm again should spawn nothing new (capacity already filled)
        let second = pool.prewarm(&mut world);
        assert_eq!(first.len(), 3);
        assert_eq!(second.len(), 0);
        assert_eq!(pool.available_count(), 3);
    }

    #[test]
    fn acquire_returns_entity_and_moves_to_active() {
        let mut world = World::new();
        let mut pool = EntityPool::new("bullet", 3);
        pool.prewarm(&mut world);

        let entity = pool.acquire();
        assert!(entity.is_some());
        assert_eq!(pool.available_count(), 2);
        assert_eq!(pool.active_count(), 1);
    }

    #[test]
    fn acquire_from_empty_pool_returns_none() {
        let mut pool = EntityPool::new("bullet", 5);
        // pool not pre-warmed, no available entities
        let result = pool.acquire();
        assert!(result.is_none());
    }

    #[test]
    fn acquire_exhausts_pool() {
        let mut world = World::new();
        let mut pool = EntityPool::new("bullet", 3);
        pool.prewarm(&mut world);

        pool.acquire();
        pool.acquire();
        pool.acquire();
        assert!(pool.is_empty());
        assert_eq!(pool.available_count(), 0);
        assert_eq!(pool.active_count(), 3);

        let result = pool.acquire();
        assert!(result.is_none());
    }

    #[test]
    fn release_returns_entity_to_available() {
        let mut world = World::new();
        let mut pool = EntityPool::new("bullet", 3);
        pool.prewarm(&mut world);

        let entity = pool.acquire().unwrap();
        assert_eq!(pool.active_count(), 1);
        assert_eq!(pool.available_count(), 2);

        let released = pool.release(entity);
        assert!(released);
        assert_eq!(pool.active_count(), 0);
        assert_eq!(pool.available_count(), 3);
    }

    #[test]
    fn release_unknown_entity_returns_false() {
        let mut world = World::new();
        let mut pool = EntityPool::new("bullet", 3);
        pool.prewarm(&mut world);

        // An entity that does not belong to this pool
        let foreign = Entity(9999);
        let result = pool.release(foreign);
        assert!(!result);
        // Pool state unchanged
        assert_eq!(pool.available_count(), 3);
        assert_eq!(pool.active_count(), 0);
    }

    #[test]
    fn available_count_tracks_correctly() {
        let mut world = World::new();
        let mut pool = EntityPool::new("particle", 4);
        pool.prewarm(&mut world);

        assert_eq!(pool.available_count(), 4);
        pool.acquire();
        assert_eq!(pool.available_count(), 3);
        pool.acquire();
        assert_eq!(pool.available_count(), 2);
    }

    #[test]
    fn active_count_tracks_correctly() {
        let mut world = World::new();
        let mut pool = EntityPool::new("particle", 4);
        pool.prewarm(&mut world);

        assert_eq!(pool.active_count(), 0);
        let e1 = pool.acquire().unwrap();
        assert_eq!(pool.active_count(), 1);
        let _e2 = pool.acquire().unwrap();
        assert_eq!(pool.active_count(), 2);
        pool.release(e1);
        assert_eq!(pool.active_count(), 1);
    }

    #[test]
    fn contains_checks_both_active_and_available() {
        let mut world = World::new();
        let mut pool = EntityPool::new("gem", 3);
        let spawned = pool.prewarm(&mut world);

        let e0 = spawned[0];
        let e1 = spawned[1];

        // Both should be contained while still available
        assert!(pool.contains(e0));
        assert!(pool.contains(e1));

        // Acquire one — it should still be contained (now active)
        pool.acquire();
        assert!(pool.contains(e0));

        // A foreign entity must NOT be contained
        assert!(!pool.contains(Entity(9999)));
    }

    #[test]
    fn is_empty_when_all_acquired() {
        let mut world = World::new();
        let mut pool = EntityPool::new("bullet", 2);
        pool.prewarm(&mut world);

        assert!(!pool.is_empty());
        pool.acquire();
        assert!(!pool.is_empty());
        pool.acquire();
        assert!(pool.is_empty());
    }

    #[test]
    fn release_all_returns_everything_to_available() {
        let mut world = World::new();
        let mut pool = EntityPool::new("enemy", 4);
        pool.prewarm(&mut world);

        pool.acquire();
        pool.acquire();
        pool.acquire();
        assert_eq!(pool.active_count(), 3);
        assert_eq!(pool.available_count(), 1);

        pool.release_all();
        assert_eq!(pool.active_count(), 0);
        assert_eq!(pool.available_count(), 4);
        assert!(!pool.is_empty());
    }

    // -----------------------------------------------------------------------
    // PoolRegistry tests
    // -----------------------------------------------------------------------

    #[test]
    fn pool_registry_create_and_get() {
        let mut registry = PoolRegistry::new();
        registry.create_pool("bullet", 8);

        assert!(registry.has_pool("bullet"));
        let pool = registry.get("bullet").unwrap();
        assert_eq!(pool.template_name, "bullet");
        assert_eq!(pool.capacity(), 8);
    }

    #[test]
    fn pool_registry_acquire_and_release() {
        let mut world = World::new();
        let mut registry = PoolRegistry::new();
        {
            let pool = registry.create_pool("bullet", 5);
            pool.prewarm(&mut world);
        }

        let entity = registry.acquire("bullet");
        assert!(entity.is_some());
        let entity = entity.unwrap();

        assert_eq!(registry.get("bullet").unwrap().active_count(), 1);

        let released = registry.release("bullet", entity);
        assert!(released);
        assert_eq!(registry.get("bullet").unwrap().active_count(), 0);
    }

    #[test]
    fn pool_registry_acquire_from_missing_pool_returns_none() {
        let mut registry = PoolRegistry::new();
        let result = registry.acquire("nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn pool_registry_release_to_missing_pool_returns_false() {
        let mut registry = PoolRegistry::new();
        let result = registry.release("nonexistent", Entity(1));
        assert!(!result);
    }

    #[test]
    fn pool_registry_total_counts() {
        let mut world = World::new();
        let mut registry = PoolRegistry::new();

        {
            let p1 = registry.create_pool("bullet", 4);
            p1.prewarm(&mut world);
        }
        {
            let p2 = registry.create_pool("enemy", 3);
            p2.prewarm(&mut world);
        }

        assert_eq!(registry.total_available(), 7);
        assert_eq!(registry.total_active(), 0);

        registry.acquire("bullet");
        registry.acquire("bullet");
        registry.acquire("enemy");

        assert_eq!(registry.total_available(), 4);
        assert_eq!(registry.total_active(), 3);
    }

    #[test]
    fn multiple_pools_work_independently() {
        let mut world = World::new();
        let mut registry = PoolRegistry::new();

        {
            let bullets = registry.create_pool("bullet", 3);
            bullets.prewarm(&mut world);
        }
        {
            let enemies = registry.create_pool("enemy", 2);
            enemies.prewarm(&mut world);
        }

        let b1 = registry.acquire("bullet").unwrap();
        let _b2 = registry.acquire("bullet").unwrap();
        let e1 = registry.acquire("enemy").unwrap();

        // Bullet pool: 1 available, 2 active
        assert_eq!(registry.get("bullet").unwrap().available_count(), 1);
        assert_eq!(registry.get("bullet").unwrap().active_count(), 2);

        // Enemy pool: 1 available, 1 active
        assert_eq!(registry.get("enemy").unwrap().available_count(), 1);
        assert_eq!(registry.get("enemy").unwrap().active_count(), 1);

        // Releasing a bullet entity into enemy pool should fail
        let cross_release = registry.release("enemy", b1);
        assert!(!cross_release);

        // Proper releases succeed
        assert!(registry.release("bullet", b1));
        assert!(registry.release("enemy", e1));

        assert_eq!(registry.get("bullet").unwrap().active_count(), 1);
        assert_eq!(registry.get("enemy").unwrap().active_count(), 0);
    }

    #[test]
    fn pool_registry_pool_names() {
        let mut registry = PoolRegistry::new();
        registry.create_pool("bullet", 4);
        registry.create_pool("enemy", 2);
        registry.create_pool("particle", 10);

        let mut names = registry.pool_names();
        names.sort();
        assert_eq!(names, vec!["bullet", "enemy", "particle"]);
    }
}
