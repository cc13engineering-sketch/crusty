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
