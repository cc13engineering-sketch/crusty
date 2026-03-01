use crate::ecs::Entity;
use crate::components::SchemaInfo;

/// Marks an entity as a child of another entity.
#[derive(Clone, Debug)]
pub struct Parent {
    pub entity: Entity,
}

impl Parent {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

impl SchemaInfo for Parent {
    fn schema_name() -> &'static str { "Parent" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "description": "Marks entity as child of another entity",
            "fields": { "entity": "Entity ID of parent" }
        })
    }
}

/// Tracks the children of an entity.
#[derive(Clone, Debug)]
pub struct Children {
    pub entities: Vec<Entity>,
}

impl Default for Children {
    fn default() -> Self {
        Self { entities: Vec::new() }
    }
}

impl Children {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(entities: Vec<Entity>) -> Self {
        Self { entities }
    }

    pub fn add(&mut self, child: Entity) {
        if !self.entities.contains(&child) {
            self.entities.push(child);
        }
    }

    pub fn remove(&mut self, child: Entity) {
        self.entities.retain(|&e| e != child);
    }

    pub fn contains(&self, child: Entity) -> bool {
        self.entities.contains(&child)
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

impl SchemaInfo for Children {
    fn schema_name() -> &'static str { "Children" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "description": "List of child entities",
            "fields": { "entities": "Vec<Entity>" }
        })
    }
}

/// Computed world-space transform (after parent propagation).
/// If entity has no parent, this equals its local Transform.
#[derive(Clone, Debug)]
pub struct WorldTransform {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub scale: f64,
}

impl Default for WorldTransform {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0 }
    }
}

impl WorldTransform {
    pub fn new(x: f64, y: f64, rotation: f64, scale: f64) -> Self {
        Self { x, y, rotation, scale }
    }
}

impl SchemaInfo for WorldTransform {
    fn schema_name() -> &'static str { "WorldTransform" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "description": "Computed world-space position after hierarchy propagation",
            "fields": {
                "x": "f64", "y": "f64",
                "rotation": "f64 (radians)",
                "scale": "f64"
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::Entity;

    #[test]
    fn parent_new() {
        let p = Parent::new(Entity(5));
        assert_eq!(p.entity, Entity(5));
    }

    #[test]
    fn children_new_empty() {
        let c = Children::new();
        assert!(c.is_empty());
        assert_eq!(c.len(), 0);
    }

    #[test]
    fn children_add_and_contains() {
        let mut c = Children::new();
        c.add(Entity(1));
        c.add(Entity(2));
        assert!(c.contains(Entity(1)));
        assert!(c.contains(Entity(2)));
        assert!(!c.contains(Entity(3)));
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn children_no_duplicates() {
        let mut c = Children::new();
        c.add(Entity(1));
        c.add(Entity(1));
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn children_remove() {
        let mut c = Children::with(vec![Entity(1), Entity(2), Entity(3)]);
        c.remove(Entity(2));
        assert_eq!(c.len(), 2);
        assert!(!c.contains(Entity(2)));
    }

    #[test]
    fn world_transform_default() {
        let wt = WorldTransform::default();
        assert_eq!(wt.x, 0.0);
        assert_eq!(wt.y, 0.0);
        assert_eq!(wt.rotation, 0.0);
        assert_eq!(wt.scale, 1.0);
    }

    #[test]
    fn world_transform_new() {
        let wt = WorldTransform::new(10.0, 20.0, 1.5, 2.0);
        assert_eq!(wt.x, 10.0);
        assert_eq!(wt.y, 20.0);
        assert_eq!(wt.rotation, 1.5);
        assert_eq!(wt.scale, 2.0);
    }

    #[test]
    fn parent_schema_info() {
        assert_eq!(Parent::schema_name(), "Parent");
        let s = Parent::schema();
        assert!(s.is_object());
    }

    #[test]
    fn children_schema_info() {
        assert_eq!(Children::schema_name(), "Children");
    }

    #[test]
    fn world_transform_schema_info() {
        assert_eq!(WorldTransform::schema_name(), "WorldTransform");
    }

    #[test]
    fn children_with_constructor() {
        let c = Children::with(vec![Entity(10), Entity(20)]);
        assert_eq!(c.len(), 2);
        assert!(c.contains(Entity(10)));
    }
}
