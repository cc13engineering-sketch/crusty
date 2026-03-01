use crate::ecs::World;
use crate::engine::WorldConfig;
use crate::rendering::color::Color;
use crate::components::*;
use crate::game_state::GameState as GlobalGameState;
use crate::timers::{TimerQueue, Timer};
use crate::templates::{EntityTemplate, TemplateRegistry};
use crate::behavior::*;
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

        // GameState — collect any extra numeric properties prefixed with "state_"
        // Also check tags for "has_state" to create an empty GameState
        {
            let mut gs = GameState::new();
            let mut has_any = false;
            for (key, val) in &edef.extra {
                if let Value::Number(n) = val {
                    // Properties like "health", "damage", "score" etc.
                    gs.set(key, *n);
                    has_any = true;
                }
            }
            if has_any {
                world.game_states.insert(entity, gs);
            }
        }

        // Lifetime — load from extra property
        if let Some(Value::Number(secs)) = edef.extra.get("lifetime") {
            world.lifetimes.insert(entity, Lifetime::new(*secs));
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

/// Extended world loading that also populates templates, global state, timers, and rules.
pub fn load_world_full(
    wf: &WorldFile,
    world: &mut World,
    config: &mut WorldConfig,
    state: &mut GlobalGameState,
    timers: &mut TimerQueue,
    templates: &mut TemplateRegistry,
    rules: &mut BehaviorRules,
) {
    // Load entities and config using the existing function
    load_world_file(wf, world, config);

    // Clear previous runtime state
    state.clear();
    timers.clear();
    templates.clear();
    rules.clear();

    let defaults = WorldDefaults::default();

    // Load initial game state
    for (key, val) in &wf.initial_state {
        match val {
            Value::Number(n) => state.set_f64(key, *n),
            Value::Bool(b) => state.set_bool(key, *b),
            Value::Str(s) => state.set_str(key, s),
            _ => {}
        }
    }

    // Load templates
    for tdef in &wf.templates {
        let mut template = EntityTemplate::new(&tdef.id);

        // Transform from position
        if let Some((x, y)) = tdef.position {
            template.transform = Some(Transform { x, y, ..Default::default() });
        } else {
            template.transform = Some(Transform::default());
        }

        // Physics -> RigidBody
        if let Some(ref phys) = tdef.physics {
            template.rigidbody = Some(RigidBody {
                mass: get_f64(phys, "mass").unwrap_or(1.0),
                vx: get_f64(phys, "vx").unwrap_or(0.0),
                vy: get_f64(phys, "vy").unwrap_or(0.0),
                restitution: get_f64(phys, "restitution").unwrap_or(defaults.restitution),
                friction: get_f64(phys, "friction").unwrap_or(defaults.friction),
                damping: get_f64(phys, "damping").unwrap_or(defaults.damping),
                is_static: get_bool(phys, "is_static").unwrap_or(false),
                ..Default::default()
            });
        }

        // Collider
        if let Some(ref col) = tdef.collider {
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
            template.collider = Some(Collider { shape, is_trigger });
        }

        // Visual -> Renderable
        if let Some(ref vis) = tdef.visual {
            let shape_name = get_ident(vis, "shape").unwrap_or_else(|| "circle".to_string());
            let color = get_color(vis, "color").unwrap_or(Color::WHITE);
            let filled = get_bool(vis, "filled").unwrap_or(true);
            let layer = get_f64(vis, "layer").map(|n| n as i32).unwrap_or(0);
            let visual = match shape_name.as_str() {
                "rect" => Visual::Rect {
                    width: get_f64(vis, "width").unwrap_or(20.0),
                    height: get_f64(vis, "height").unwrap_or(20.0),
                    color, filled,
                },
                _ => Visual::Circle {
                    radius: get_f64(vis, "radius").unwrap_or(10.0),
                    color, filled,
                },
            };
            template.renderable = Some(Renderable { visual, layer, visible: true });
        }

        // Tags
        if !tdef.tags.is_empty() {
            template.tags = Some(Tags { values: tdef.tags.clone() });
        }

        // Lifetime
        if let Some(Value::Number(secs)) = tdef.extra.get("lifetime") {
            template.lifetime = Some(Lifetime::new(*secs));
        }

        templates.register(template);
    }

    // Load timers
    for tdef in &wf.timers {
        let delay = get_f64(&tdef.props, "delay").unwrap_or(0.0);
        let interval = get_f64(&tdef.props, "interval");
        let max_fires = get_f64(&tdef.props, "max_fires").unwrap_or(0.0) as u64;

        let timer = if let Some(iv) = interval {
            if max_fires > 0 {
                Timer::repeating_n(&tdef.name, delay, iv, max_fires)
            } else {
                Timer::repeating(&tdef.name, delay, iv)
            }
        } else {
            Timer::one_shot(&tdef.name, delay)
        };
        timers.add(timer);
    }

    // Load behavior rules
    for rdef in &wf.rules {
        let condition = parse_rule_condition(&rdef.condition_name, &rdef.condition_args);
        let actions = rdef.actions.iter()
            .filter_map(|(name, args)| parse_rule_action(name, args))
            .collect();
        let once = rdef.props.get("once")
            .and_then(|v| if let Value::Bool(b) = v { Some(*b) } else { None })
            .unwrap_or(false);

        if let Some(cond) = condition {
            let mut rule = BehaviorRule::new(&rdef.name, cond, actions);
            if once { rule.once = true; }
            rules.add(rule);
        }
    }
}

fn parse_rule_condition(name: &str, args: &[Value]) -> Option<Condition> {
    match name {
        "collision" => {
            let tag_a = args.first().and_then(|v| match v {
                Value::Str(s) => Some(s.clone()),
                Value::Ident(s) => Some(s.clone()),
                _ => None,
            })?;
            let tag_b = args.get(1).and_then(|v| match v {
                Value::Str(s) => Some(s.clone()),
                Value::Ident(s) => Some(s.clone()),
                _ => None,
            })?;
            Some(Condition::Collision { tag_a, tag_b })
        }
        "trigger_enter" => {
            let trigger_tag = args.first().and_then(|v| match v {
                Value::Str(s) => Some(s.clone()),
                Value::Ident(s) => Some(s.clone()),
                _ => None,
            })?;
            let visitor_tag = args.get(1).and_then(|v| match v {
                Value::Str(s) => Some(s.clone()),
                Value::Ident(s) => Some(s.clone()),
                _ => None,
            })?;
            Some(Condition::TriggerEnter { trigger_tag, visitor_tag })
        }
        "timer" => {
            let timer_name = args.first().and_then(|v| match v {
                Value::Str(s) => Some(s.clone()),
                Value::Ident(s) => Some(s.clone()),
                _ => None,
            })?;
            Some(Condition::TimerFired { timer_name })
        }
        "state" => {
            let key = args.first().and_then(|v| match v {
                Value::Str(s) => Some(s.clone()),
                Value::Ident(s) => Some(s.clone()),
                _ => None,
            })?;
            let op_str = args.get(1).and_then(|v| match v {
                Value::Str(s) => Some(s.clone()),
                Value::Ident(s) => Some(s.clone()),
                _ => None,
            })?;
            let value = args.get(2).and_then(|v| match v {
                Value::Number(n) => Some(*n),
                _ => None,
            })?;
            let op = CompareOp::from_str_op(&op_str)?;
            Some(Condition::StateCheck { key, op, value })
        }
        "always" => Some(Condition::Always),
        _ => None,
    }
}

fn parse_rule_action(name: &str, args: &[Value]) -> Option<Action> {
    match name {
        "despawn" => {
            let entity_ref = args.first().and_then(|v| match v {
                Value::Ident(s) | Value::Str(s) => match s.as_str() {
                    "a" | "A" | "first" => Some(EntityRef::A),
                    "b" | "B" | "second" => Some(EntityRef::B),
                    "both" | "all" => Some(EntityRef::Both),
                    _ => None,
                },
                _ => None,
            }).unwrap_or(EntityRef::Both);
            Some(Action::Despawn { entity_ref })
        }
        "add_state" => {
            let key = args.first().and_then(|v| match v {
                Value::Str(s) | Value::Ident(s) => Some(s.clone()),
                _ => None,
            })?;
            let delta = args.get(1).and_then(|v| match v {
                Value::Number(n) => Some(*n),
                _ => None,
            })?;
            Some(Action::AddState { key, delta })
        }
        "set_state" => {
            let key = args.first().and_then(|v| match v {
                Value::Str(s) | Value::Ident(s) => Some(s.clone()),
                _ => None,
            })?;
            let value = args.get(1).and_then(|v| match v {
                Value::Number(n) => Some(*n),
                _ => None,
            })?;
            Some(Action::SetState { key, value })
        }
        "set_flag" => {
            let key = args.first().and_then(|v| match v {
                Value::Str(s) | Value::Ident(s) => Some(s.clone()),
                _ => None,
            })?;
            let value = args.get(1).and_then(|v| match v {
                Value::Bool(b) => Some(*b),
                _ => None,
            }).unwrap_or(true);
            Some(Action::SetFlag { key, value })
        }
        "spawn" => {
            let template_name = args.first().and_then(|v| match v {
                Value::Str(s) | Value::Ident(s) => Some(s.clone()),
                _ => None,
            })?;
            let at = if args.len() >= 3 {
                let x = args.get(1).and_then(|v| match v {
                    Value::Number(n) => Some(*n),
                    _ => None,
                }).unwrap_or(0.0);
                let y = args.get(2).and_then(|v| match v {
                    Value::Number(n) => Some(*n),
                    _ => None,
                }).unwrap_or(0.0);
                SpawnAt::Position(x, y)
            } else {
                let at_str = args.get(1).and_then(|v| match v {
                    Value::Ident(s) => Some(s.clone()),
                    _ => None,
                });
                match at_str.as_deref() {
                    Some("random") => SpawnAt::Random,
                    Some("entity_a") => SpawnAt::EntityA,
                    Some("entity_b") => SpawnAt::EntityB,
                    _ => SpawnAt::Random,
                }
            };
            Some(Action::SpawnTemplate { template_name, at })
        }
        "start_timer" => {
            let name = args.first().and_then(|v| match v {
                Value::Str(s) | Value::Ident(s) => Some(s.clone()),
                _ => None,
            })?;
            let delay = args.get(1).and_then(|v| match v {
                Value::Number(n) => Some(*n),
                _ => None,
            }).unwrap_or(0.0);
            let interval = args.get(2).and_then(|v| match v {
                Value::Number(n) => Some(*n),
                _ => None,
            });
            let max_fires = args.get(3).and_then(|v| match v {
                Value::Number(n) => Some(*n as u64),
                _ => None,
            }).unwrap_or(0);
            Some(Action::StartTimer { name, delay, interval, max_fires })
        }
        "cancel_timer" => {
            let name = args.first().and_then(|v| match v {
                Value::Str(s) | Value::Ident(s) => Some(s.clone()),
                _ => None,
            })?;
            Some(Action::CancelTimer { name })
        }
        "log" => {
            let message = args.first().and_then(|v| match v {
                Value::Str(s) => Some(s.clone()),
                _ => None,
            })?;
            Some(Action::Log { message })
        }
        _ => None,
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

    // -----------------------------------------------------------------------
    // 13. Full load with templates, state, timers, and rules
    // -----------------------------------------------------------------------
    #[test]
    fn load_world_full_populates_all_systems() {
        let src = r#"
            world "Test Full" {}

            state {
                score: 0
                lives: 3
            }

            template bullet {
                physics: { mass: 0.1, vy: -500 }
                collider: { shape: circle, radius: 3 }
                visual: { shape: circle, radius: 3, color: #ffffff, filled: true }
                tags: ["bullet"]
                lifetime: 2.0
            }

            timer wave_timer {
                delay: 5.0
                interval: 10.0
            }

            rule "Score on collision" {
                when: collision(bullet, asteroid)
                then: [
                    despawn(both),
                    add_state(score, 10)
                ]
            }

            entity player {
                position: (100, 200)
                tags: ["player"]
            }
        "#;
        let wf = parse_world(src).unwrap();
        let mut world = World::new();
        let mut config = WorldConfig::default();
        let mut state = GlobalGameState::new();
        let mut timers = TimerQueue::new();
        let mut templates = TemplateRegistry::new();
        let mut rules = BehaviorRules::new();

        load_world_full(&wf, &mut world, &mut config, &mut state, &mut timers, &mut templates, &mut rules);

        // Check config
        assert_eq!(config.name, "Test Full");

        // Check entities
        assert_eq!(world.entity_count(), 1);

        // Check initial state
        assert_eq!(state.get_f64("score"), Some(0.0));
        assert_eq!(state.get_f64("lives"), Some(3.0));

        // Check templates
        assert!(templates.has("bullet"));
        assert_eq!(templates.len(), 1);

        // Check timers
        assert_eq!(timers.len(), 1);
        assert!(timers.has("wave_timer"));

        // Check rules
        assert_eq!(rules.len(), 1);
        assert_eq!(rules.rules[0].name, "Score on collision");
    }

    // -----------------------------------------------------------------------
    // 14. Full load - template spawns correctly
    // -----------------------------------------------------------------------
    #[test]
    fn load_world_full_template_spawn_works() {
        let src = r#"
            world "Test" {}
            template bullet {
                physics: { mass: 0.1, vy: -500 }
                collider: { shape: circle, radius: 3 }
                tags: ["bullet"]
                lifetime: 2.0
            }
        "#;
        let wf = parse_world(src).unwrap();
        let mut world = World::new();
        let mut config = WorldConfig::default();
        let mut state = GlobalGameState::new();
        let mut timers = TimerQueue::new();
        let mut templates = TemplateRegistry::new();
        let mut rules = BehaviorRules::new();

        load_world_full(&wf, &mut world, &mut config, &mut state, &mut timers, &mut templates, &mut rules);

        // Spawn from template
        let entity = templates.spawn("bullet", &mut world, Some((100.0, 200.0))).unwrap();
        assert!(world.is_alive(entity));
        let t = world.transforms.get(entity).unwrap();
        assert_eq!(t.x, 100.0);
        assert_eq!(t.y, 200.0);
        assert!(world.rigidbodies.has(entity));
        assert!(world.colliders.has(entity));
        assert!(world.tags.has(entity));
        assert!(world.lifetimes.has(entity));
        let lt = world.lifetimes.get(entity).unwrap();
        assert_eq!(lt.duration, 2.0);
    }

    // -----------------------------------------------------------------------
    // 15. Full load of space_survival_v2.world
    // -----------------------------------------------------------------------
    #[test]
    fn load_space_survival_v2() {
        let src = include_str!("../../../../worlds/space_survival_v2.world");
        let wf = parse_world(src).unwrap();
        let mut world = World::new();
        let mut config = WorldConfig::default();
        let mut state = GlobalGameState::new();
        let mut timers = TimerQueue::new();
        let mut templates = TemplateRegistry::new();
        let mut rules = BehaviorRules::new();

        load_world_full(&wf, &mut world, &mut config, &mut state, &mut timers, &mut templates, &mut rules);

        assert_eq!(config.name, "Space Survival v2");

        // State initialized
        assert_eq!(state.get_f64("score"), Some(0.0));
        assert_eq!(state.get_f64("lives"), Some(3.0));
        assert_eq!(state.get_f64("wave"), Some(0.0));

        // Templates loaded
        assert!(templates.has("bullet"));
        assert!(templates.has("asteroid_small"));
        assert!(templates.has("asteroid_large"));
        assert!(templates.has("pickup_health"));
        assert_eq!(templates.len(), 4);

        // Timers loaded
        assert!(timers.has("asteroid_wave"));
        assert!(timers.has("pickup_spawn"));
        assert_eq!(timers.len(), 2);

        // Rules loaded
        assert_eq!(rules.len(), 6);

        // Entities loaded (player, spawner, 4 walls)
        assert_eq!(world.entity_count(), 6);
    }
}
