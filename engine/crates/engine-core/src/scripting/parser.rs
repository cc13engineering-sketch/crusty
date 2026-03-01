use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

// pest_derive resolves grammar paths relative to the crate's src/ directory.
#[derive(Parser)]
#[grammar = "scripting/world_lang.pest"]
pub struct WorldParser;

pub struct WorldFile {
    pub name: String,
    pub bounds: Option<(f64, f64)>,
    pub background: Option<String>,
    pub feel: Vec<String>,
    pub entities: Vec<EntityDef>,
}

pub struct EntityDef {
    pub id: String,
    pub role: Option<String>,
    pub intent: Option<String>,
    pub group: Option<String>,
    pub position: Option<(f64, f64)>,
    pub physics: Option<HashMap<String, Value>>,
    pub collider: Option<HashMap<String, Value>>,
    pub visual: Option<HashMap<String, Value>>,
    pub force_field: Option<HashMap<String, Value>>,
    pub tags: Vec<String>,
    pub extra: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Color(String),
    Vec2(f64, f64),
    Ident(String),
    Array(Vec<Value>),
}

/// Strip enclosing quotes from a pest string match.
fn strip_quotes(s: &str) -> &str {
    if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

fn parse_value(pair: pest::iterators::Pair<Rule>) -> Value {
    match pair.as_rule() {
        Rule::value => parse_value(pair.into_inner().next().unwrap()),
        Rule::number => Value::Number(pair.as_str().parse::<f64>().unwrap_or(0.0)),
        Rule::string => Value::Str(strip_quotes(pair.as_str()).to_string()),
        Rule::bool_val => Value::Bool(pair.as_str() == "true"),
        Rule::color_value => Value::Color(pair.as_str().to_string()),
        Rule::vec2 => {
            let mut inner = pair.into_inner();
            let x = inner.next().unwrap().as_str().parse::<f64>().unwrap_or(0.0);
            let y = inner.next().unwrap().as_str().parse::<f64>().unwrap_or(0.0);
            Value::Vec2(x, y)
        }
        Rule::ident => Value::Ident(pair.as_str().to_string()),
        Rule::array => {
            let inner = pair.into_inner().next();
            match inner {
                Some(value_list) => {
                    Value::Array(value_list.into_inner().map(parse_value).collect())
                }
                None => Value::Array(Vec::new()),
            }
        }
        Rule::object => {
            // Objects parsed separately; this shouldn't be hit from value context
            Value::Str("object".to_string())
        }
        _ => Value::Str(pair.as_str().to_string()),
    }
}

fn parse_object(pair: pest::iterators::Pair<Rule>) -> HashMap<String, Value> {
    let mut map = HashMap::new();
    for field in pair.into_inner() {
        if field.as_rule() == Rule::object_field {
            let mut inner = field.into_inner();
            let key = inner.next().unwrap().as_str().to_string();
            let val = parse_value(inner.next().unwrap());
            map.insert(key, val);
        }
    }
    map
}

pub fn parse_world(source: &str) -> Result<WorldFile, String> {
    let pairs = WorldParser::parse(Rule::world_file, source)
        .map_err(|e| format!("Parse error: {}", e))?;

    let mut world_file = WorldFile {
        name: String::new(),
        bounds: None,
        background: None,
        feel: Vec::new(),
        entities: Vec::new(),
    };

    for pair in pairs.into_iter().next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::declaration => {
                let inner = pair.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::world_decl => {
                        let mut inner_pairs = inner.into_inner();
                        world_file.name = strip_quotes(inner_pairs.next().unwrap().as_str()).to_string();
                        for prop in inner_pairs {
                            if prop.as_rule() == Rule::world_prop {
                                let mut parts = prop.into_inner();
                                let first = parts.next().unwrap();
                                match first.as_rule() {
                                    Rule::number => {
                                        // bounds: num x num
                                        let w = first.as_str().parse::<f64>().unwrap_or(960.0);
                                        let h = parts.next().unwrap().as_str().parse::<f64>().unwrap_or(540.0);
                                        world_file.bounds = Some((w, h));
                                    }
                                    Rule::color_value => {
                                        world_file.background = Some(first.as_str().to_string());
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Rule::feel_decl => {
                        let ident_list = inner.into_inner().next().unwrap();
                        for ident in ident_list.into_inner() {
                            world_file.feel.push(ident.as_str().to_string());
                        }
                    }
                    Rule::entity_decl => {
                        let mut inner_pairs = inner.into_inner();
                        let id = inner_pairs.next().unwrap().as_str().to_string();
                        let mut entity = EntityDef {
                            id,
                            role: None, intent: None, group: None,
                            position: None, physics: None, collider: None,
                            visual: None, force_field: None,
                            tags: Vec::new(), extra: HashMap::new(),
                        };

                        for prop in inner_pairs {
                            if prop.as_rule() != Rule::entity_prop { continue; }
                            let prop_str = prop.as_str();
                            let mut parts = prop.into_inner();
                            let first = parts.next().unwrap();

                            match first.as_rule() {
                                Rule::string if prop_str.starts_with("role") => {
                                    entity.role = Some(strip_quotes(first.as_str()).to_string());
                                }
                                Rule::string if prop_str.starts_with("intent") => {
                                    entity.intent = Some(strip_quotes(first.as_str()).to_string());
                                }
                                Rule::string if prop_str.starts_with("group") => {
                                    entity.group = Some(strip_quotes(first.as_str()).to_string());
                                }
                                Rule::vec2 => {
                                    let mut coords = first.into_inner();
                                    let x = coords.next().unwrap().as_str().parse::<f64>().unwrap_or(0.0);
                                    let y = coords.next().unwrap().as_str().parse::<f64>().unwrap_or(0.0);
                                    entity.position = Some((x, y));
                                }
                                Rule::object if prop_str.starts_with("physics") => {
                                    entity.physics = Some(parse_object(first));
                                }
                                Rule::object if prop_str.starts_with("collider") => {
                                    entity.collider = Some(parse_object(first));
                                }
                                Rule::object if prop_str.starts_with("visual") => {
                                    entity.visual = Some(parse_object(first));
                                }
                                Rule::object if prop_str.starts_with("force_field") => {
                                    entity.force_field = Some(parse_object(first));
                                }
                                Rule::string_list => {
                                    for s in first.into_inner() {
                                        entity.tags.push(strip_quotes(s.as_str()).to_string());
                                    }
                                }
                                Rule::ident => {
                                    let key = first.as_str().to_string();
                                    if let Some(val_pair) = parts.next() {
                                        entity.extra.insert(key, parse_value(val_pair));
                                    }
                                }
                                _ => {
                                    // Catch-all for ident: value properties
                                    if let Some(val_pair) = parts.next() {
                                        let key = first.as_str().to_string();
                                        entity.extra.insert(key, parse_value(val_pair));
                                    }
                                }
                            }
                        }
                        world_file.entities.push(entity);
                    }
                    _ => {}
                }
            }
            Rule::EOI => {}
            _ => {}
        }
    }

    Ok(world_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // 1. Parse minimal world
    // -----------------------------------------------------------------------
    #[test]
    fn parse_minimal_world() {
        let wf = parse_world(r#"world "Test" {}"#).unwrap();
        assert_eq!(wf.name, "Test");
        assert!(wf.entities.is_empty());
        assert!(wf.bounds.is_none());
        assert!(wf.background.is_none());
        assert!(wf.feel.is_empty());
    }

    // -----------------------------------------------------------------------
    // 2. Parse world with bounds
    // -----------------------------------------------------------------------
    #[test]
    fn parse_world_with_bounds() {
        let wf = parse_world(r#"world "Test" { bounds: 800 x 600 }"#).unwrap();
        assert_eq!(wf.name, "Test");
        let (w, h) = wf.bounds.unwrap();
        assert_eq!(w, 800.0);
        assert_eq!(h, 600.0);
    }

    // -----------------------------------------------------------------------
    // 3. Parse world with background
    // -----------------------------------------------------------------------
    #[test]
    fn parse_world_with_background() {
        let wf = parse_world(r#"world "Test" { background: #ff0000 }"#).unwrap();
        assert_eq!(wf.background.as_deref(), Some("#ff0000"));
    }

    // -----------------------------------------------------------------------
    // 4. Parse feel declaration
    // -----------------------------------------------------------------------
    #[test]
    fn parse_feel_declaration() {
        let src = r#"
            world "Test" {}
            feel: [bouncy, floaty]
        "#;
        let wf = parse_world(src).unwrap();
        assert_eq!(wf.feel, vec!["bouncy", "floaty"]);
    }

    // -----------------------------------------------------------------------
    // 5. Parse simple entity with position
    // -----------------------------------------------------------------------
    #[test]
    fn parse_simple_entity_with_position() {
        let src = r#"
            world "Test" {}
            entity ball { position: (100, 200) }
        "#;
        let wf = parse_world(src).unwrap();
        assert_eq!(wf.entities.len(), 1);
        let e = &wf.entities[0];
        assert_eq!(e.id, "ball");
        let (x, y) = e.position.unwrap();
        assert_eq!(x, 100.0);
        assert_eq!(y, 200.0);
    }

    // -----------------------------------------------------------------------
    // 6. Parse entity with physics
    // -----------------------------------------------------------------------
    #[test]
    fn parse_entity_with_physics() {
        let src = r#"
            world "Test" {}
            entity ball { physics: { mass: 2.0, vx: 100 } }
        "#;
        let wf = parse_world(src).unwrap();
        let e = &wf.entities[0];
        let phys = e.physics.as_ref().unwrap();
        match phys.get("mass") {
            Some(Value::Number(n)) => assert_eq!(*n, 2.0),
            other => panic!("expected Number(2.0), got {:?}", other),
        }
        match phys.get("vx") {
            Some(Value::Number(n)) => assert_eq!(*n, 100.0),
            other => panic!("expected Number(100.0), got {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // 7. Parse entity with collider (rect)
    // -----------------------------------------------------------------------
    #[test]
    fn parse_entity_with_collider() {
        let src = r#"
            world "Test" {}
            entity wall { collider: { shape: rect, half_width: 50, half_height: 10 } }
        "#;
        let wf = parse_world(src).unwrap();
        let e = &wf.entities[0];
        let col = e.collider.as_ref().unwrap();
        match col.get("shape") {
            Some(Value::Ident(s)) => assert_eq!(s, "rect"),
            other => panic!("expected Ident(\"rect\"), got {:?}", other),
        }
        match col.get("half_width") {
            Some(Value::Number(n)) => assert_eq!(*n, 50.0),
            other => panic!("expected Number(50.0), got {:?}", other),
        }
        match col.get("half_height") {
            Some(Value::Number(n)) => assert_eq!(*n, 10.0),
            other => panic!("expected Number(10.0), got {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // 8. Parse entity with tags
    // -----------------------------------------------------------------------
    #[test]
    fn parse_entity_with_tags() {
        let src = r#"
            world "Test" {}
            entity ball { tags: ["player", "ball"] }
        "#;
        let wf = parse_world(src).unwrap();
        let e = &wf.entities[0];
        assert_eq!(e.tags, vec!["player", "ball"]);
    }

    // -----------------------------------------------------------------------
    // 9. Parse entity with visual
    // -----------------------------------------------------------------------
    #[test]
    fn parse_entity_with_visual() {
        let src = r#"
            world "Test" {}
            entity ball { visual: { shape: circle, radius: 15, color: #ff0000, filled: true } }
        "#;
        let wf = parse_world(src).unwrap();
        let e = &wf.entities[0];
        let vis = e.visual.as_ref().unwrap();
        match vis.get("shape") {
            Some(Value::Ident(s)) => assert_eq!(s, "circle"),
            other => panic!("expected Ident(\"circle\"), got {:?}", other),
        }
        match vis.get("radius") {
            Some(Value::Number(n)) => assert_eq!(*n, 15.0),
            other => panic!("expected Number(15.0), got {:?}", other),
        }
        match vis.get("color") {
            Some(Value::Color(c)) => assert_eq!(c, "#ff0000"),
            other => panic!("expected Color(\"#ff0000\"), got {:?}", other),
        }
        match vis.get("filled") {
            Some(Value::Bool(b)) => assert_eq!(*b, true),
            other => panic!("expected Bool(true), got {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // 10. Parse entity with role, intent, and group
    // -----------------------------------------------------------------------
    #[test]
    fn parse_entity_with_role_intent_group() {
        let src = r#"
            world "Test" {}
            entity hero {
                role: "player"
                intent: "wander"
                group: "team_a"
            }
        "#;
        let wf = parse_world(src).unwrap();
        let e = &wf.entities[0];
        assert_eq!(e.role.as_deref(), Some("player"));
        assert_eq!(e.intent.as_deref(), Some("wander"));
        assert_eq!(e.group.as_deref(), Some("team_a"));
    }

    // -----------------------------------------------------------------------
    // 11. Parse comments — lines starting with // are ignored
    // -----------------------------------------------------------------------
    #[test]
    fn parse_comments_are_ignored() {
        let src = r#"
            // This is a comment
            world "Test" {
                // Another comment
                bounds: 640 x 480
            }
            // Yet another comment
            entity ball {
                position: (10, 20)
            }
        "#;
        let wf = parse_world(src).unwrap();
        assert_eq!(wf.name, "Test");
        let (w, h) = wf.bounds.unwrap();
        assert_eq!(w, 640.0);
        assert_eq!(h, 480.0);
        assert_eq!(wf.entities.len(), 1);
        assert_eq!(wf.entities[0].id, "ball");
    }

    // -----------------------------------------------------------------------
    // 12. Parse empty input — grammar allows zero declarations, yields empty world
    // -----------------------------------------------------------------------
    #[test]
    fn parse_empty_input_yields_empty_world() {
        let wf = parse_world("").unwrap();
        assert_eq!(wf.name, "");
        assert!(wf.entities.is_empty());
        assert!(wf.bounds.is_none());
        assert!(wf.background.is_none());
        assert!(wf.feel.is_empty());
    }

    // -----------------------------------------------------------------------
    // 13. Parse invalid syntax → error
    // -----------------------------------------------------------------------
    #[test]
    fn parse_invalid_syntax_is_error() {
        let result = parse_world("this is not valid world syntax {{{");
        assert!(result.is_err(), "invalid syntax should fail to parse");
    }

    // -----------------------------------------------------------------------
    // 14. Parse multiple entities
    // -----------------------------------------------------------------------
    #[test]
    fn parse_multiple_entities() {
        let src = r#"
            world "Test" {}
            entity ball1 { position: (10, 20) }
            entity ball2 { position: (30, 40) }
            entity ball3 { position: (50, 60) }
        "#;
        let wf = parse_world(src).unwrap();
        assert_eq!(wf.entities.len(), 3);
        assert_eq!(wf.entities[0].id, "ball1");
        assert_eq!(wf.entities[1].id, "ball2");
        assert_eq!(wf.entities[2].id, "ball3");
        assert_eq!(wf.entities[0].position, Some((10.0, 20.0)));
        assert_eq!(wf.entities[1].position, Some((30.0, 40.0)));
        assert_eq!(wf.entities[2].position, Some((50.0, 60.0)));
    }

    // -----------------------------------------------------------------------
    // 15. Parse entity with force_field
    // -----------------------------------------------------------------------
    #[test]
    fn parse_entity_with_force_field() {
        let src = r#"
            world "Test" {}
            entity attractor {
                force_field: { type: attract, strength: 500, radius: 300, falloff: linear }
            }
        "#;
        let wf = parse_world(src).unwrap();
        let e = &wf.entities[0];
        let ff = e.force_field.as_ref().unwrap();
        match ff.get("type") {
            Some(Value::Ident(s)) => assert_eq!(s, "attract"),
            other => panic!("expected Ident(\"attract\"), got {:?}", other),
        }
        match ff.get("strength") {
            Some(Value::Number(n)) => assert_eq!(*n, 500.0),
            other => panic!("expected Number(500.0), got {:?}", other),
        }
        match ff.get("radius") {
            Some(Value::Number(n)) => assert_eq!(*n, 300.0),
            other => panic!("expected Number(300.0), got {:?}", other),
        }
        match ff.get("falloff") {
            Some(Value::Ident(s)) => assert_eq!(s, "linear"),
            other => panic!("expected Ident(\"linear\"), got {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // 16. Parse the actual bouncing_balls.world file content
    // -----------------------------------------------------------------------
    #[test]
    fn parse_bouncing_balls_world() {
        let src = r#"// Bouncing Balls — click any ball to freeze it in place.
// Press D to toggle debug visualization.

world "Bouncing Balls" {
    bounds: 960 x 540
    background: #111118
}

feel: [bouncy]

// --- Walls ---

entity wall_top {
    position: (480, 0)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 480, half_height: 6 }
    visual: { shape: rect, width: 960, height: 12, color: #2a2a3a, filled: true }
}

entity wall_bottom {
    position: (480, 540)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 480, half_height: 6 }
    visual: { shape: rect, width: 960, height: 12, color: #2a2a3a, filled: true }
}

entity wall_left {
    position: (0, 270)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 6, half_height: 270 }
    visual: { shape: rect, width: 12, height: 540, color: #2a2a3a, filled: true }
}

entity wall_right {
    position: (960, 270)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 6, half_height: 270 }
    visual: { shape: rect, width: 12, height: 540, color: #2a2a3a, filled: true }
}

// --- Balls ---

entity ball_1 {
    position: (120, 100)
    physics: { mass: 1.0, vx: 280.0, vy: 190.0, restitution: 0.95, damping: 0.001 }
    collider: { shape: circle, radius: 18 }
    visual: { shape: circle, radius: 18, color: #e84040, filled: true }
    tags: ["ball"]
}

entity ball_2 {
    position: (300, 400)
    physics: { mass: 1.5, vx: -200.0, vy: 150.0, restitution: 0.95, damping: 0.001 }
    collider: { shape: circle, radius: 22 }
    visual: { shape: circle, radius: 22, color: #40a0e8, filled: true }
    tags: ["ball"]
}

entity ball_3 {
    position: (700, 150)
    physics: { mass: 0.8, vx: -150.0, vy: 260.0, restitution: 0.95, damping: 0.001 }
    collider: { shape: circle, radius: 14 }
    visual: { shape: circle, radius: 14, color: #50d860, filled: true }
    tags: ["ball"]
}

entity ball_4 {
    position: (500, 300)
    physics: { mass: 2.0, vx: 180.0, vy: -220.0, restitution: 0.95, damping: 0.001 }
    collider: { shape: circle, radius: 26 }
    visual: { shape: circle, radius: 26, color: #e8c840, filled: true }
    tags: ["ball"]
}

entity ball_5 {
    position: (200, 250)
    physics: { mass: 1.0, vx: 250.0, vy: -180.0, restitution: 0.95, damping: 0.001 }
    collider: { shape: circle, radius: 16 }
    visual: { shape: circle, radius: 16, color: #c050d8, filled: true }
    tags: ["ball"]
}

entity ball_6 {
    position: (800, 400)
    physics: { mass: 1.2, vx: -220.0, vy: -160.0, restitution: 0.95, damping: 0.001 }
    collider: { shape: circle, radius: 20 }
    visual: { shape: circle, radius: 20, color: #e87830, filled: true }
    tags: ["ball"]
}

entity ball_7 {
    position: (600, 80)
    physics: { mass: 0.7, vx: -300.0, vy: 200.0, restitution: 0.95, damping: 0.001 }
    collider: { shape: circle, radius: 12 }
    visual: { shape: circle, radius: 12, color: #40e8d0, filled: true }
    tags: ["ball"]
}

entity ball_8 {
    position: (400, 470)
    physics: { mass: 1.8, vx: 160.0, vy: -240.0, restitution: 0.95, damping: 0.001 }
    collider: { shape: circle, radius: 24 }
    visual: { shape: circle, radius: 24, color: #e8e8e8, filled: true }
    tags: ["ball"]
}

entity ball_9 {
    position: (150, 450)
    physics: { mass: 0.9, vx: 270.0, vy: -100.0, restitution: 0.95, damping: 0.001 }
    collider: { shape: circle, radius: 15 }
    visual: { shape: circle, radius: 15, color: #d84080, filled: true }
    tags: ["ball"]
}

entity ball_10 {
    position: (750, 280)
    physics: { mass: 1.3, vx: -190.0, vy: 230.0, restitution: 0.95, damping: 0.001 }
    collider: { shape: circle, radius: 19 }
    visual: { shape: circle, radius: 19, color: #80d840, filled: true }
    tags: ["ball"]
}
"#;
        let wf = parse_world(src).unwrap();
        assert_eq!(wf.name, "Bouncing Balls");
        assert_eq!(wf.bounds, Some((960.0, 540.0)));
        assert_eq!(wf.background.as_deref(), Some("#111118"));
        assert_eq!(wf.feel, vec!["bouncy"]);
        // 4 walls + 10 balls = 14 entities
        assert_eq!(wf.entities.len(), 14);

        // Verify first wall
        let wall_top = &wf.entities[0];
        assert_eq!(wall_top.id, "wall_top");
        assert_eq!(wall_top.position, Some((480.0, 0.0)));
        let phys = wall_top.physics.as_ref().unwrap();
        match phys.get("is_static") {
            Some(Value::Bool(b)) => assert_eq!(*b, true),
            other => panic!("expected Bool(true), got {:?}", other),
        }

        // Verify first ball
        let ball_1 = &wf.entities[4];
        assert_eq!(ball_1.id, "ball_1");
        assert_eq!(ball_1.position, Some((120.0, 100.0)));
        assert_eq!(ball_1.tags, vec!["ball"]);
        let phys = ball_1.physics.as_ref().unwrap();
        match phys.get("mass") {
            Some(Value::Number(n)) => assert_eq!(*n, 1.0),
            other => panic!("expected Number(1.0), got {:?}", other),
        }
        match phys.get("vx") {
            Some(Value::Number(n)) => assert_eq!(*n, 280.0),
            other => panic!("expected Number(280.0), got {:?}", other),
        }

        // Verify last ball
        let ball_10 = &wf.entities[13];
        assert_eq!(ball_10.id, "ball_10");
        assert_eq!(ball_10.position, Some((750.0, 280.0)));
    }

    // -----------------------------------------------------------------------
    // 17. Parse the actual walker.world file content
    // -----------------------------------------------------------------------
    #[test]
    fn parse_walker_world() {
        let src = r#"// Walker — move the character with arrow keys. You cannot walk through shapes.

world "Walker" {
    bounds: 960 x 540
    background: #0d0d14
}

// --- Player ---

entity player {
    position: (100, 270)
    physics: { mass: 1.0, restitution: 0.0, damping: 0.001 }
    collider: { shape: circle, radius: 12 }
    visual: { shape: circle, radius: 12, color: #f0e060, filled: true }
    tags: ["player"]
}

// --- Walls ---

entity wall_top {
    position: (480, 0)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 480, half_height: 6 }
    visual: { shape: rect, width: 960, height: 12, color: #2a2a3a, filled: true }
}

entity wall_bottom {
    position: (480, 540)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 480, half_height: 6 }
    visual: { shape: rect, width: 960, height: 12, color: #2a2a3a, filled: true }
}

entity wall_left {
    position: (0, 270)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 6, half_height: 270 }
    visual: { shape: rect, width: 12, height: 540, color: #2a2a3a, filled: true }
}

entity wall_right {
    position: (960, 270)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 6, half_height: 270 }
    visual: { shape: rect, width: 12, height: 540, color: #2a2a3a, filled: true }
}

// --- Rectangular obstacles ---

entity block_1 {
    position: (250, 150)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 60, half_height: 20 }
    visual: { shape: rect, width: 120, height: 40, color: #3a5a8a, filled: true }
}

entity block_2 {
    position: (400, 350)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 20, half_height: 80 }
    visual: { shape: rect, width: 40, height: 160, color: #3a5a8a, filled: true }
}

entity block_3 {
    position: (650, 200)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 80, half_height: 15 }
    visual: { shape: rect, width: 160, height: 30, color: #3a5a8a, filled: true }
}

entity block_4 {
    position: (550, 450)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 50, half_height: 25 }
    visual: { shape: rect, width: 100, height: 50, color: #3a5a8a, filled: true }
}

entity block_5 {
    position: (200, 400)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 40, half_height: 40 }
    visual: { shape: rect, width: 80, height: 80, color: #4a3a6a, filled: true }
}

entity block_6 {
    position: (780, 380)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 70, half_height: 12 }
    visual: { shape: rect, width: 140, height: 24, color: #3a5a8a, filled: true }
}

entity block_7 {
    position: (350, 100)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 15, half_height: 50 }
    visual: { shape: rect, width: 30, height: 100, color: #4a3a6a, filled: true }
}

// --- Circular pillars ---

entity pillar_1 {
    position: (500, 270)
    physics: { is_static: true }
    collider: { shape: circle, radius: 30 }
    visual: { shape: circle, radius: 30, color: #6a4a3a, filled: true }
}

entity pillar_2 {
    position: (150, 270)
    physics: { is_static: true }
    collider: { shape: circle, radius: 20 }
    visual: { shape: circle, radius: 20, color: #6a4a3a, filled: true }
}

entity pillar_3 {
    position: (720, 120)
    physics: { is_static: true }
    collider: { shape: circle, radius: 25 }
    visual: { shape: circle, radius: 25, color: #6a4a3a, filled: true }
}

entity pillar_4 {
    position: (850, 270)
    physics: { is_static: true }
    collider: { shape: circle, radius: 35 }
    visual: { shape: circle, radius: 35, color: #5a3a3a, filled: true }
}

entity pillar_5 {
    position: (300, 480)
    physics: { is_static: true }
    collider: { shape: circle, radius: 18 }
    visual: { shape: circle, radius: 18, color: #6a4a3a, filled: true }
}

entity pillar_6 {
    position: (680, 460)
    physics: { is_static: true }
    collider: { shape: circle, radius: 22 }
    visual: { shape: circle, radius: 22, color: #6a4a3a, filled: true }
}

// --- Small scatter ---

entity crate_1 {
    position: (440, 120)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 12, half_height: 12 }
    visual: { shape: rect, width: 24, height: 24, color: #5a6a3a, filled: true }
}

entity crate_2 {
    position: (600, 340)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 12, half_height: 12 }
    visual: { shape: rect, width: 24, height: 24, color: #5a6a3a, filled: true }
}

entity crate_3 {
    position: (880, 480)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 12, half_height: 12 }
    visual: { shape: rect, width: 24, height: 24, color: #5a6a3a, filled: true }
}

entity pebble_1 {
    position: (530, 80)
    physics: { is_static: true }
    collider: { shape: circle, radius: 8 }
    visual: { shape: circle, radius: 8, color: #555555, filled: true }
}

entity pebble_2 {
    position: (760, 500)
    physics: { is_static: true }
    collider: { shape: circle, radius: 10 }
    visual: { shape: circle, radius: 10, color: #555555, filled: true }
}

entity pebble_3 {
    position: (100, 100)
    physics: { is_static: true }
    collider: { shape: circle, radius: 7 }
    visual: { shape: circle, radius: 7, color: #555555, filled: true }
}
"#;
        let wf = parse_world(src).unwrap();
        assert_eq!(wf.name, "Walker");
        assert_eq!(wf.bounds, Some((960.0, 540.0)));
        assert_eq!(wf.background.as_deref(), Some("#0d0d14"));
        assert!(wf.feel.is_empty());

        // 1 player + 4 walls + 7 blocks + 6 pillars + 3 crates + 3 pebbles = 24 entities
        assert_eq!(wf.entities.len(), 24);

        // Verify player
        let player = &wf.entities[0];
        assert_eq!(player.id, "player");
        assert_eq!(player.position, Some((100.0, 270.0)));
        assert_eq!(player.tags, vec!["player"]);

        // Verify a block
        let block_1 = &wf.entities[5];
        assert_eq!(block_1.id, "block_1");
        assert_eq!(block_1.position, Some((250.0, 150.0)));

        // Verify a pillar
        let pillar_1 = &wf.entities[12];
        assert_eq!(pillar_1.id, "pillar_1");
        assert_eq!(pillar_1.position, Some((500.0, 270.0)));

        // Verify last entity
        let pebble_3 = &wf.entities[23];
        assert_eq!(pebble_3.id, "pebble_3");
        assert_eq!(pebble_3.position, Some((100.0, 100.0)));
    }
}
