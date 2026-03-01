use super::SchemaInfo;

#[derive(Clone, Debug)]
pub struct Collider {
    pub shape: ColliderShape,
    pub is_trigger: bool,
}

#[derive(Clone, Debug)]
pub enum ColliderShape {
    Circle { radius: f64 },
    Rect { half_width: f64, half_height: f64 },
}

impl Default for Collider {
    fn default() -> Self {
        Self { shape: ColliderShape::Circle { radius: 10.0 }, is_trigger: false }
    }
}

impl SchemaInfo for Collider {
    fn schema_name() -> &'static str { "Collider" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "shape": { "type": "enum", "variants": ["Circle { radius }", "Rect { half_width, half_height }"] },
                "is_trigger": { "type": "bool", "default": false }
            }
        })
    }
}
