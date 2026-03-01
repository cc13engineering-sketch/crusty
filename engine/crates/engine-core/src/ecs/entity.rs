use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Entity(pub u64);

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashSet, HashMap};

    #[test]
    fn equality_same_id() {
        let a = Entity(1);
        let b = Entity(1);
        assert_eq!(a, b);
    }

    #[test]
    fn inequality_different_id() {
        let a = Entity(1);
        let b = Entity(2);
        assert_ne!(a, b);
    }

    #[test]
    fn equality_zero() {
        assert_eq!(Entity(0), Entity(0));
    }

    #[test]
    fn hash_set_insert_and_lookup() {
        let mut set = HashSet::new();
        set.insert(Entity(1));
        set.insert(Entity(2));
        set.insert(Entity(1)); // duplicate
        assert_eq!(set.len(), 2);
        assert!(set.contains(&Entity(1)));
        assert!(set.contains(&Entity(2)));
        assert!(!set.contains(&Entity(3)));
    }

    #[test]
    fn hash_map_as_key() {
        let mut map = HashMap::new();
        map.insert(Entity(10), "ten");
        map.insert(Entity(20), "twenty");
        assert_eq!(map.get(&Entity(10)), Some(&"ten"));
        assert_eq!(map.get(&Entity(20)), Some(&"twenty"));
        assert_eq!(map.get(&Entity(30)), None);
    }

    #[test]
    fn copy_semantics() {
        let a = Entity(42);
        let b = a; // Copy, not move
        assert_eq!(a, b);
        // Both a and b are still usable — this line proves `a` was not moved
        assert_eq!(a.0, 42);
        assert_eq!(b.0, 42);
    }

    #[test]
    fn clone_semantics() {
        let a = Entity(99);
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn debug_formatting() {
        let e = Entity(7);
        let dbg = format!("{:?}", e);
        assert!(dbg.contains("Entity"));
        assert!(dbg.contains("7"));
    }

    #[test]
    fn debug_formatting_zero() {
        let e = Entity(0);
        let dbg = format!("{:?}", e);
        assert!(dbg.contains("0"));
    }

    #[test]
    fn serialize_json() {
        let e = Entity(123);
        let json = serde_json::to_string(&e).expect("serialize");
        assert_eq!(json, "123");
    }

    #[test]
    fn deserialize_json() {
        let e: Entity = serde_json::from_str("456").expect("deserialize");
        assert_eq!(e, Entity(456));
    }

    #[test]
    fn serialize_deserialize_roundtrip() {
        let original = Entity(u64::MAX);
        let json = serde_json::to_string(&original).expect("serialize");
        let restored: Entity = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original, restored);
    }

    #[test]
    fn public_field_access() {
        let e = Entity(55);
        assert_eq!(e.0, 55);
    }

    #[test]
    fn hash_set_remove() {
        let mut set = HashSet::new();
        set.insert(Entity(1));
        set.insert(Entity(2));
        assert!(set.remove(&Entity(1)));
        assert!(!set.contains(&Entity(1)));
        assert!(set.contains(&Entity(2)));
    }
}
