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
