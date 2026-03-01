use super::SchemaInfo;

#[derive(Clone, Debug)]
pub struct PhysicsMaterial {
    pub static_friction: f64,
    pub dynamic_friction: f64,
    pub drag: f64,
    pub restitution_override: Option<f64>,
}

impl Default for PhysicsMaterial {
    fn default() -> Self {
        Self {
            static_friction: 0.3,
            dynamic_friction: 0.2,
            drag: 0.0,
            restitution_override: None,
        }
    }
}

impl SchemaInfo for PhysicsMaterial {
    fn schema_name() -> &'static str { "PhysicsMaterial" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "static_friction": { "type": "f64", "default": 0.3, "range": [0.0, 1.0] },
                "dynamic_friction": { "type": "f64", "default": 0.2, "range": [0.0, 1.0] },
                "drag": { "type": "f64", "default": 0.0, "range": [0.0, 1.0] },
                "restitution_override": { "type": "Option<f64>", "default": "None" }
            }
        })
    }
}
