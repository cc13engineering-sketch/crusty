/// Entity Templates: blueprints for spawning entities at runtime.
///
/// A template stores all the component data needed to create an entity.
/// Templates are defined in .world files and stored by name. Runtime code
/// can spawn from a template by name, optionally overriding the position.

use std::collections::HashMap;
use crate::components::*;
use crate::components::lifetime::Lifetime;
use crate::ecs::{World, Entity};

/// A complete blueprint for an entity.
#[derive(Clone, Debug)]
pub struct EntityTemplate {
    /// Name of this template (used for lookup).
    pub name: String,
    /// Optional transform (position can be overridden at spawn time).
    pub transform: Option<Transform>,
    /// Optional rigidbody.
    pub rigidbody: Option<RigidBody>,
    /// Optional collider.
    pub collider: Option<Collider>,
    /// Optional renderable.
    pub renderable: Option<Renderable>,
    /// Optional force field.
    pub force_field: Option<ForceField>,
    /// Optional tags.
    pub tags: Option<Tags>,
    /// Optional role.
    pub role: Option<Role>,
    /// Optional lifetime (auto-despawn).
    pub lifetime: Option<Lifetime>,
    /// Optional per-entity game state (health, ammo, etc.).
    pub game_state: Option<GameState>,
    /// Optional autonomous behavior.
    pub behavior: Option<Behavior>,
}

impl EntityTemplate {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            transform: None,
            rigidbody: None,
            collider: None,
            renderable: None,
            force_field: None,
            tags: None,
            role: None,
            lifetime: None,
            game_state: None,
            behavior: None,
        }
    }

    /// Spawn this template into the world, returning the new Entity.
    /// If `position` is Some, override the template's transform position.
    pub fn spawn_into(&self, world: &mut World, position: Option<(f64, f64)>) -> Entity {
        let entity = world.spawn();

        // Transform
        if let Some(ref t) = self.transform {
            let mut transform = t.clone();
            if let Some((x, y)) = position {
                transform.x = x;
                transform.y = y;
            }
            world.transforms.insert(entity, transform);
        } else if let Some((x, y)) = position {
            world.transforms.insert(entity, Transform { x, y, ..Default::default() });
        }

        // RigidBody
        if let Some(ref rb) = self.rigidbody {
            world.rigidbodies.insert(entity, rb.clone());
        }

        // Collider
        if let Some(ref col) = self.collider {
            world.colliders.insert(entity, col.clone());
        }

        // Renderable
        if let Some(ref rend) = self.renderable {
            world.renderables.insert(entity, rend.clone());
        }

        // ForceField
        if let Some(ref ff) = self.force_field {
            world.force_fields.insert(entity, ff.clone());
        }

        // Tags
        if let Some(ref tags) = self.tags {
            world.tags.insert(entity, tags.clone());
        }

        // Role
        if let Some(ref role) = self.role {
            world.roles.insert(entity, role.clone());
        }

        // Lifetime
        if let Some(ref lt) = self.lifetime {
            world.lifetimes.insert(entity, lt.clone());
        }

        // Per-entity game state
        if let Some(ref gs) = self.game_state {
            world.game_states.insert(entity, gs.clone());
        }

        // Behavior
        if let Some(ref beh) = self.behavior {
            world.behaviors.insert(entity, beh.clone());
        }

        entity
    }
}

/// Registry of named entity templates.
#[derive(Default, Clone, Debug)]
pub struct TemplateRegistry {
    templates: HashMap<String, EntityTemplate>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        Self { templates: HashMap::new() }
    }

    pub fn register(&mut self, template: EntityTemplate) {
        self.templates.insert(template.name.clone(), template);
    }

    pub fn get(&self, name: &str) -> Option<&EntityTemplate> {
        self.templates.get(name)
    }

    pub fn has(&self, name: &str) -> bool {
        self.templates.contains_key(name)
    }

    pub fn remove(&mut self, name: &str) -> Option<EntityTemplate> {
        self.templates.remove(name)
    }

    /// Spawn a named template into the world. Returns None if template not found.
    pub fn spawn(&self, name: &str, world: &mut World, position: Option<(f64, f64)>) -> Option<Entity> {
        let template = self.templates.get(name)?;
        Some(template.spawn_into(world, position))
    }

    pub fn clear(&mut self) {
        self.templates.clear();
    }

    pub fn len(&self) -> usize {
        self.templates.len()
    }

    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.templates.keys().map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendering::color::Color;

    fn bullet_template() -> EntityTemplate {
        let mut t = EntityTemplate::new("bullet");
        t.transform = Some(Transform { x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        t.rigidbody = Some(RigidBody {
            mass: 0.1, vx: 0.0, vy: -500.0, damping: 0.0,
            restitution: 0.0, is_static: false,
            ..Default::default()
        });
        t.collider = Some(Collider {
            shape: ColliderShape::Circle { radius: 3.0 },
            is_trigger: false,
        });
        t.renderable = Some(Renderable {
            visual: Visual::Circle { radius: 3.0, color: Color::WHITE, filled: true },
            layer: 5,
            visible: true,
        });
        t.tags = Some(Tags::new(&["bullet"]));
        t.lifetime = Some(Lifetime::new(3.0));
        t
    }

    #[test]
    fn template_new_has_name() {
        let t = EntityTemplate::new("test");
        assert_eq!(t.name, "test");
        assert!(t.transform.is_none());
        assert!(t.rigidbody.is_none());
    }

    #[test]
    fn spawn_into_creates_entity() {
        let mut world = World::new();
        let template = bullet_template();
        let entity = template.spawn_into(&mut world, None);
        assert!(world.is_alive(entity));
    }

    #[test]
    fn spawn_into_applies_all_components() {
        let mut world = World::new();
        let template = bullet_template();
        let entity = template.spawn_into(&mut world, None);

        assert!(world.transforms.has(entity));
        assert!(world.rigidbodies.has(entity));
        assert!(world.colliders.has(entity));
        assert!(world.renderables.has(entity));
        assert!(world.tags.has(entity));
        assert!(world.lifetimes.has(entity));
    }

    #[test]
    fn spawn_into_uses_template_position_when_no_override() {
        let mut world = World::new();
        let mut template = EntityTemplate::new("test");
        template.transform = Some(Transform { x: 100.0, y: 200.0, ..Default::default() });
        let entity = template.spawn_into(&mut world, None);

        let t = world.transforms.get(entity).unwrap();
        assert_eq!(t.x, 100.0);
        assert_eq!(t.y, 200.0);
    }

    #[test]
    fn spawn_into_overrides_position() {
        let mut world = World::new();
        let mut template = EntityTemplate::new("test");
        template.transform = Some(Transform { x: 100.0, y: 200.0, ..Default::default() });
        let entity = template.spawn_into(&mut world, Some((50.0, 75.0)));

        let t = world.transforms.get(entity).unwrap();
        assert_eq!(t.x, 50.0);
        assert_eq!(t.y, 75.0);
    }

    #[test]
    fn spawn_into_creates_transform_from_position_when_no_template_transform() {
        let mut world = World::new();
        let template = EntityTemplate::new("bare");
        let entity = template.spawn_into(&mut world, Some((10.0, 20.0)));

        let t = world.transforms.get(entity).unwrap();
        assert_eq!(t.x, 10.0);
        assert_eq!(t.y, 20.0);
    }

    #[test]
    fn registry_register_and_get() {
        let mut reg = TemplateRegistry::new();
        reg.register(bullet_template());
        assert!(reg.has("bullet"));
        assert_eq!(reg.get("bullet").unwrap().name, "bullet");
    }

    #[test]
    fn registry_spawn() {
        let mut reg = TemplateRegistry::new();
        reg.register(bullet_template());
        let mut world = World::new();
        let entity = reg.spawn("bullet", &mut world, Some((100.0, 200.0))).unwrap();
        assert!(world.is_alive(entity));
        let t = world.transforms.get(entity).unwrap();
        assert_eq!(t.x, 100.0);
        assert_eq!(t.y, 200.0);
    }

    #[test]
    fn registry_spawn_unknown_returns_none() {
        let reg = TemplateRegistry::new();
        let mut world = World::new();
        assert!(reg.spawn("nonexistent", &mut world, None).is_none());
    }

    #[test]
    fn registry_remove() {
        let mut reg = TemplateRegistry::new();
        reg.register(bullet_template());
        let removed = reg.remove("bullet");
        assert!(removed.is_some());
        assert!(!reg.has("bullet"));
    }

    #[test]
    fn registry_clear() {
        let mut reg = TemplateRegistry::new();
        reg.register(bullet_template());
        reg.clear();
        assert!(reg.is_empty());
    }

    #[test]
    fn registry_len() {
        let mut reg = TemplateRegistry::new();
        assert_eq!(reg.len(), 0);
        reg.register(bullet_template());
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn registry_names() {
        let mut reg = TemplateRegistry::new();
        reg.register(bullet_template());
        let mut t2 = EntityTemplate::new("asteroid");
        t2.transform = Some(Transform::default());
        reg.register(t2);
        let mut names: Vec<&str> = reg.names().collect();
        names.sort();
        assert_eq!(names, vec!["asteroid", "bullet"]);
    }

    #[test]
    fn multiple_spawns_get_unique_entities() {
        let mut world = World::new();
        let template = bullet_template();
        let e1 = template.spawn_into(&mut world, Some((10.0, 10.0)));
        let e2 = template.spawn_into(&mut world, Some((20.0, 20.0)));
        assert_ne!(e1, e2);
        assert!(world.is_alive(e1));
        assert!(world.is_alive(e2));
    }
}
