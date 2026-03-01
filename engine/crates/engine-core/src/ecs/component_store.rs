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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct TestComponent {
        val: i32,
    }

    fn tc(val: i32) -> TestComponent {
        TestComponent { val }
    }

    #[test]
    fn new_creates_empty_store() {
        let store = ComponentStore::<TestComponent>::new();
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn default_creates_empty_store() {
        let store = ComponentStore::<TestComponent>::default();
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn insert_and_get() {
        let mut store = ComponentStore::new();
        let e = Entity(1);
        store.insert(e, tc(42));
        assert_eq!(store.get(e), Some(&tc(42)));
    }

    #[test]
    fn get_nonexistent_returns_none() {
        let store = ComponentStore::<TestComponent>::new();
        assert_eq!(store.get(Entity(999)), None);
    }

    #[test]
    fn insert_overwrites_existing() {
        let mut store = ComponentStore::new();
        let e = Entity(1);
        store.insert(e, tc(10));
        store.insert(e, tc(20));
        assert_eq!(store.get(e), Some(&tc(20)));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn get_mut_modifies_in_place() {
        let mut store = ComponentStore::new();
        let e = Entity(1);
        store.insert(e, tc(5));
        if let Some(comp) = store.get_mut(e) {
            comp.val = 99;
        }
        assert_eq!(store.get(e), Some(&tc(99)));
    }

    #[test]
    fn get_mut_nonexistent_returns_none() {
        let mut store = ComponentStore::<TestComponent>::new();
        assert!(store.get_mut(Entity(1)).is_none());
    }

    #[test]
    fn remove_returns_old_value() {
        let mut store = ComponentStore::new();
        let e = Entity(1);
        store.insert(e, tc(42));
        let removed = store.remove(e);
        assert_eq!(removed, Some(tc(42)));
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn remove_nonexistent_returns_none() {
        let mut store = ComponentStore::<TestComponent>::new();
        assert_eq!(store.remove(Entity(1)), None);
    }

    #[test]
    fn remove_makes_get_return_none() {
        let mut store = ComponentStore::new();
        let e = Entity(1);
        store.insert(e, tc(10));
        store.remove(e);
        assert_eq!(store.get(e), None);
    }

    #[test]
    fn has_true_when_present() {
        let mut store = ComponentStore::new();
        let e = Entity(1);
        store.insert(e, tc(1));
        assert!(store.has(e));
    }

    #[test]
    fn has_false_when_absent() {
        let store = ComponentStore::<TestComponent>::new();
        assert!(!store.has(Entity(1)));
    }

    #[test]
    fn has_false_after_remove() {
        let mut store = ComponentStore::new();
        let e = Entity(1);
        store.insert(e, tc(1));
        store.remove(e);
        assert!(!store.has(e));
    }

    #[test]
    fn len_reflects_operations() {
        let mut store = ComponentStore::new();
        assert_eq!(store.len(), 0);
        store.insert(Entity(1), tc(1));
        assert_eq!(store.len(), 1);
        store.insert(Entity(2), tc(2));
        assert_eq!(store.len(), 2);
        // Overwrite does not change len
        store.insert(Entity(1), tc(10));
        assert_eq!(store.len(), 2);
        store.remove(Entity(1));
        assert_eq!(store.len(), 1);
        store.remove(Entity(2));
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn clear_empties_everything() {
        let mut store = ComponentStore::new();
        store.insert(Entity(1), tc(1));
        store.insert(Entity(2), tc(2));
        store.insert(Entity(3), tc(3));
        store.clear();
        assert_eq!(store.len(), 0);
        assert!(!store.has(Entity(1)));
        assert!(!store.has(Entity(2)));
        assert!(!store.has(Entity(3)));
    }

    #[test]
    fn clear_on_empty_store_is_fine() {
        let mut store = ComponentStore::<TestComponent>::new();
        store.clear();
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn iter_yields_all_pairs() {
        let mut store = ComponentStore::new();
        store.insert(Entity(1), tc(10));
        store.insert(Entity(2), tc(20));
        store.insert(Entity(3), tc(30));

        let mut pairs: Vec<(Entity, i32)> = store.iter().map(|(e, c)| (e, c.val)).collect();
        pairs.sort_by_key(|(e, _)| e.0);

        assert_eq!(pairs, vec![
            (Entity(1), 10),
            (Entity(2), 20),
            (Entity(3), 30),
        ]);
    }

    #[test]
    fn iter_empty_store() {
        let store = ComponentStore::<TestComponent>::new();
        let count = store.iter().count();
        assert_eq!(count, 0);
    }

    #[test]
    fn iter_mut_modifies_all() {
        let mut store = ComponentStore::new();
        store.insert(Entity(1), tc(1));
        store.insert(Entity(2), tc(2));

        for (_e, comp) in store.iter_mut() {
            comp.val *= 10;
        }

        assert_eq!(store.get(Entity(1)), Some(&tc(10)));
        assert_eq!(store.get(Entity(2)), Some(&tc(20)));
    }

    #[test]
    fn entities_returns_all_keys() {
        let mut store = ComponentStore::new();
        store.insert(Entity(5), tc(50));
        store.insert(Entity(3), tc(30));
        store.insert(Entity(7), tc(70));

        let mut ents: Vec<Entity> = store.entities().collect();
        ents.sort_by_key(|e| e.0);
        assert_eq!(ents, vec![Entity(3), Entity(5), Entity(7)]);
    }

    #[test]
    fn entities_empty_store() {
        let store = ComponentStore::<TestComponent>::new();
        let ents: Vec<Entity> = store.entities().collect();
        assert!(ents.is_empty());
    }

    #[test]
    fn sorted_entities_returns_sorted_by_id() {
        let mut store = ComponentStore::new();
        store.insert(Entity(100), tc(1));
        store.insert(Entity(3), tc(2));
        store.insert(Entity(50), tc(3));
        store.insert(Entity(1), tc(4));

        let sorted = store.sorted_entities();
        assert_eq!(sorted, vec![Entity(1), Entity(3), Entity(50), Entity(100)]);
    }

    #[test]
    fn sorted_entities_empty_store() {
        let store = ComponentStore::<TestComponent>::new();
        let sorted = store.sorted_entities();
        assert!(sorted.is_empty());
    }

    #[test]
    fn sorted_entities_single_element() {
        let mut store = ComponentStore::new();
        store.insert(Entity(42), tc(1));
        let sorted = store.sorted_entities();
        assert_eq!(sorted, vec![Entity(42)]);
    }

    #[test]
    fn insert_multiple_distinct_entities() {
        let mut store = ComponentStore::new();
        for i in 0..100 {
            store.insert(Entity(i), tc(i as i32));
        }
        assert_eq!(store.len(), 100);
        for i in 0..100 {
            assert_eq!(store.get(Entity(i)), Some(&tc(i as i32)));
        }
    }
}
