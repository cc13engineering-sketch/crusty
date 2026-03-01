use crate::ecs::World;
use crate::engine::WorldConfig;
use crate::rendering::color::Color;
use crate::components::*;
use super::parser::{WorldFile, Value};

struct WorldDefaults {
    damping: f64,
    restitution: f64,
    friction: f64,
}

impl Default for WorldDefaults {
    fn default() -> Self {
        Self { damping: 0.01, restitution: 0.5, friction: 0.3 }
    }
}

fn get_f64(map: &std::collections::HashMap<String, Value>, key: &str) -> Option<f64> {
    match map.get(key) {
        Some(Value::Number(n)) => Some(*n),
        _ => None,
    }
}

fn get_bool(map: &std::collections::HashMap<String, Value>, key: &str) -> Option<bool> {
    match map.get(key) {
        Some(Value::Bool(b)) => Some(*b),
        _ => None,
    }
}

fn get_ident(map: &std::collections::HashMap<String, Value>, key: &str) -> Option<String> {
    match map.get(key) {
        Some(Value::Ident(s)) => Some(s.clone()),
        _ => None,
    }
}

fn get_color(map: &std::collections::HashMap<String, Value>, key: &str) -> Option<Color> {
    match map.get(key) {
        Some(Value::Color(s)) => Color::from_hex(s),
        _ => None,
    }
}

pub fn load_world_file(wf: &WorldFile, world: &mut World, config: &mut WorldConfig) {
    world.clear();

    config.name = wf.name.clone();
    if let Some((w, h)) = wf.bounds {
        config.bounds = (w, h);
    }
    if let Some(ref hex) = wf.background {
        if let Some(color) = Color::from_hex(hex) {
            config.background = color;
        }
    }

    // Apply feel keywords
    let mut defaults = WorldDefaults::default();
    for feel in &wf.feel {
        match feel.as_str() {
            "floaty" => defaults.damping = 0.001,
            "bouncy" => defaults.restitution = 0.9,
            "snappy" => defaults.damping = 0.1,
            "sticky" => { defaults.restitution = 0.1; defaults.friction = 0.8; }
            "frictionless" => defaults.friction = 0.0,
            _ => {}
        }
    }

    // Spawn entities
    for edef in &wf.entities {
        let entity = world.spawn_named(&edef.id);

        // Transform
        let (x, y) = edef.position.unwrap_or((0.0, 0.0));
        world.transforms.insert(entity, Transform { x, y, ..Default::default() });

        // Role
        if edef.role.is_some() || edef.intent.is_some() {
            world.roles.insert(entity, Role {
                name: edef.role.clone().unwrap_or_default(),
                intent: edef.intent.clone().unwrap_or_default(),
                group: edef.group.clone(),
            });
        }

        // Tags
        if !edef.tags.is_empty() {
            world.tags.insert(entity, Tags { values: edef.tags.clone() });
        }

        // Physics → RigidBody
        if let Some(ref phys) = edef.physics {
            let rb = RigidBody {
                mass: get_f64(phys, "mass").unwrap_or(1.0),
                vx: get_f64(phys, "vx").unwrap_or(0.0),
                vy: get_f64(phys, "vy").unwrap_or(0.0),
                restitution: get_f64(phys, "restitution").unwrap_or(defaults.restitution),
                friction: get_f64(phys, "friction").unwrap_or(defaults.friction),
                damping: get_f64(phys, "damping").unwrap_or(defaults.damping),
                is_static: get_bool(phys, "is_static").unwrap_or(false),
                ..Default::default()
            };
            world.rigidbodies.insert(entity, rb);
        }

        // Collider
        if let Some(ref col) = edef.collider {
            let shape_name = get_ident(col, "shape").unwrap_or_else(|| "circle".to_string());
            let shape = match shape_name.as_str() {
                "rect" => ColliderShape::Rect {
                    half_width: get_f64(col, "half_width").unwrap_or(10.0),
                    half_height: get_f64(col, "half_height").unwrap_or(10.0),
                },
                _ => ColliderShape::Circle {
                    radius: get_f64(col, "radius").unwrap_or(10.0),
                },
            };
            let is_trigger = get_bool(col, "is_trigger").unwrap_or(false);
            world.colliders.insert(entity, Collider { shape, is_trigger });
        }

        // Visual → Renderable
        if let Some(ref vis) = edef.visual {
            let shape_name = get_ident(vis, "shape").unwrap_or_else(|| "circle".to_string());
            let color = get_color(vis, "color").unwrap_or(Color::WHITE);
            let filled = get_bool(vis, "filled").unwrap_or(true);
            let layer = get_f64(vis, "layer").map(|n| n as i32).unwrap_or(0);

            let visual = match shape_name.as_str() {
                "rect" => Visual::Rect {
                    width: get_f64(vis, "width").unwrap_or(20.0),
                    height: get_f64(vis, "height").unwrap_or(20.0),
                    color,
                    filled,
                },
                "line" => Visual::Line {
                    x2: get_f64(vis, "x2").unwrap_or(0.0),
                    y2: get_f64(vis, "y2").unwrap_or(0.0),
                    color,
                    thickness: get_f64(vis, "thickness").unwrap_or(1.0),
                },
                _ => Visual::Circle {
                    radius: get_f64(vis, "radius").unwrap_or(10.0),
                    color,
                    filled,
                },
            };
            world.renderables.insert(entity, Renderable { visual, layer, visible: true });
        }

        // ForceField
        if let Some(ref ff) = edef.force_field {
            let type_name = get_ident(ff, "type").unwrap_or_else(|| "attract".to_string());
            let field_type = match type_name.as_str() {
                "repel" => FieldType::Repel,
                "directional" => FieldType::Directional {
                    dx: get_f64(ff, "dx").unwrap_or(0.0),
                    dy: get_f64(ff, "dy").unwrap_or(1.0),
                },
                "vortex" => FieldType::Vortex,
                _ => FieldType::Attract,
            };
            let falloff_name = get_ident(ff, "falloff").unwrap_or_else(|| "inverse_square".to_string());
            let falloff = match falloff_name.as_str() {
                "constant" => Falloff::Constant,
                "linear" => Falloff::Linear,
                _ => Falloff::InverseSquare,
            };
            world.force_fields.insert(entity, ForceField {
                field_type,
                strength: get_f64(ff, "strength").unwrap_or(100.0),
                radius: get_f64(ff, "radius").unwrap_or(200.0),
                falloff,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::World;
    use crate::engine::WorldConfig;
    use super::super::parser::parse_world;

    /// Helper: parse source and load into a fresh world + config.
    fn load_from_source(src: &str) -> (World, WorldConfig) {
        let wf = parse_world(src).unwrap();
        let mut world = World::new();
        let mut config = WorldConfig::default();
        load_world_file(&wf, &mut world, &mut config);
        (world, config)
    }

    // -----------------------------------------------------------------------
    // 1. Load minimal world — config name set, world cleared
    // -----------------------------------------------------------------------
    #[test]
    fn load_minimal_world_sets_config_name() {
        let (world, config) = load_from_source(r#"world "Hello" {}"#);
        assert_eq!(config.name, "Hello");
        assert_eq!(world.entity_count(), 0);
    }

    // -----------------------------------------------------------------------
    // 2. Load world with bounds — config.bounds updated
    // -----------------------------------------------------------------------
    #[test]
    fn load_world_with_bounds_updates_config() {
        let (_, config) = load_from_source(r#"world "Test" { bounds: 800 x 600 }"#);
        assert_eq!(config.bounds, (800.0, 600.0));
    }

    // -----------------------------------------------------------------------
    // 3. Load world with background — config.background updated
    // -----------------------------------------------------------------------
    #[test]
    fn load_world_with_background_updates_config() {
        let (_, config) = load_from_source(r#"world "Test" { background: #ff0000 }"#);
        assert_eq!(config.background, Color::from_hex("#ff0000").unwrap());
    }

    // -----------------------------------------------------------------------
    // 4. Load entity with position — Transform.x/y set
    // -----------------------------------------------------------------------
    #[test]
    fn load_entity_with_position_sets_transform() {
        let src = r#"
            world "Test" {}
            entity ball { position: (150, 250) }
        "#;
        let (world, _) = load_from_source(src);
        assert_eq!(world.entity_count(), 1);
        let entity = world.names.get_by_name("ball").unwrap();
        let transform = world.transforms.get(entity).unwrap();
        assert_eq!(transform.x, 150.0);
        assert_eq!(transform.y, 250.0);
    }

    // -----------------------------------------------------------------------
    // 5. Load entity with physics — RigidBody created with correct values
    // -----------------------------------------------------------------------
    #[test]
    fn load_entity_with_physics_creates_rigidbody() {
        let src = r#"
            world "Test" {}
            entity ball { physics: { mass: 2.0, vx: 100, vy: -50 } }
        "#;
        let (world, _) = load_from_source(src);
        let entity = world.names.get_by_name("ball").unwrap();
        let rb = world.rigidbodies.get(entity).unwrap();
        assert_eq!(rb.mass, 2.0);
        assert_eq!(rb.vx, 100.0);
        assert_eq!(rb.vy, -50.0);
    }

    // -----------------------------------------------------------------------
    // 6. Load entity with collider circle — Collider with Circle shape
    // -----------------------------------------------------------------------
    #[test]
    fn load_entity_with_collider_circle() {
        let src = r#"
            world "Test" {}
            entity ball { collider: { shape: circle, radius: 25 } }
        "#;
        let (world, _) = load_from_source(src);
        let entity = world.names.get_by_name("ball").unwrap();
        let collider = world.colliders.get(entity).unwrap();
        match collider.shape {
            ColliderShape::Circle { radius } => assert_eq!(radius, 25.0),
            _ => panic!("expected Circle shape"),
        }
        assert_eq!(collider.is_trigger, false);
    }

    // -----------------------------------------------------------------------
    // 7. Load entity with collider rect — Collider with Rect shape
    // -----------------------------------------------------------------------
    #[test]
    fn load_entity_with_collider_rect() {
        let src = r#"
            world "Test" {}
            entity wall { collider: { shape: rect, half_width: 50, half_height: 10 } }
        "#;
        let (world, _) = load_from_source(src);
        let entity = world.names.get_by_name("wall").unwrap();
        let collider = world.colliders.get(entity).unwrap();
        match collider.shape {
            ColliderShape::Rect { half_width, half_height } => {
                assert_eq!(half_width, 50.0);
                assert_eq!(half_height, 10.0);
            }
            _ => panic!("expected Rect shape"),
        }
    }

    // -----------------------------------------------------------------------
    // 8. Load entity with tags — Tags component added
    // -----------------------------------------------------------------------
    #[test]
    fn load_entity_with_tags() {
        let src = r#"
            world "Test" {}
            entity ball { tags: ["player", "ball"] }
        "#;
        let (world, _) = load_from_source(src);
        let entity = world.names.get_by_name("ball").unwrap();
        let tags = world.tags.get(entity).unwrap();
        assert_eq!(tags.values, vec!["player", "ball"]);
    }

    // -----------------------------------------------------------------------
    // 9. Load entity with role/intent — Role component added
    // -----------------------------------------------------------------------
    #[test]
    fn load_entity_with_role_intent() {
        let src = r#"
            world "Test" {}
            entity hero {
                role: "player"
                intent: "wander"
                group: "team_a"
            }
        "#;
        let (world, _) = load_from_source(src);
        let entity = world.names.get_by_name("hero").unwrap();
        let role = world.roles.get(entity).unwrap();
        assert_eq!(role.name, "player");
        assert_eq!(role.intent, "wander");
        assert_eq!(role.group.as_deref(), Some("team_a"));
    }

    // -----------------------------------------------------------------------
    // 10. Feel "bouncy" sets restitution to 0.9
    // -----------------------------------------------------------------------
    #[test]
    fn feel_bouncy_sets_restitution() {
        let src = r#"
            world "Test" {}
            feel: [bouncy]
            entity ball { physics: { mass: 1.0 } }
        "#;
        let (world, _) = load_from_source(src);
        let entity = world.names.get_by_name("ball").unwrap();
        let rb = world.rigidbodies.get(entity).unwrap();
        // When feel is bouncy, default restitution should be 0.9
        assert_eq!(rb.restitution, 0.9);
    }

    // -----------------------------------------------------------------------
    // 11. Feel "floaty" sets damping to 0.001
    // -----------------------------------------------------------------------
    #[test]
    fn feel_floaty_sets_damping() {
        let src = r#"
            world "Test" {}
            feel: [floaty]
            entity ball { physics: { mass: 1.0 } }
        "#;
        let (world, _) = load_from_source(src);
        let entity = world.names.get_by_name("ball").unwrap();
        let rb = world.rigidbodies.get(entity).unwrap();
        // When feel is floaty, default damping should be 0.001
        assert_eq!(rb.damping, 0.001);
    }

    // -----------------------------------------------------------------------
    // 12. load_world_file clears existing entities first
    // -----------------------------------------------------------------------
    #[test]
    fn load_world_file_clears_existing_entities() {
        let mut world = World::new();
        let mut config = WorldConfig::default();

        // Pre-populate the world with some entities
        let e1 = world.spawn_named("old_entity_1");
        world.transforms.insert(e1, Transform { x: 1.0, y: 2.0, ..Default::default() });
        let e2 = world.spawn_named("old_entity_2");
        world.transforms.insert(e2, Transform { x: 3.0, y: 4.0, ..Default::default() });
        assert_eq!(world.entity_count(), 2);

        // Now load a new world file
        let wf = parse_world(r#"
            world "Fresh" {}
            entity new_one { position: (10, 20) }
        "#).unwrap();
        load_world_file(&wf, &mut world, &mut config);

        // Old entities should be gone, only the new one remains
        assert_eq!(world.entity_count(), 1);
        assert!(world.names.get_by_name("old_entity_1").is_none());
        assert!(world.names.get_by_name("old_entity_2").is_none());
        let new_entity = world.names.get_by_name("new_one").unwrap();
        let t = world.transforms.get(new_entity).unwrap();
        assert_eq!(t.x, 10.0);
        assert_eq!(t.y, 20.0);
        assert_eq!(config.name, "Fresh");
    }
}
