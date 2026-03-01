use super::SchemaInfo;

#[derive(Clone, Debug)]
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub rotation: f64, // radians — RESERVED, not used by v1 systems
    pub scale: f64,    // RESERVED, not used by v1 systems
}

impl Default for Transform {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0 }
    }
}

impl SchemaInfo for Transform {
    fn schema_name() -> &'static str { "Transform" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "x": { "type": "f64", "default": 0.0 },
                "y": { "type": "f64", "default": 0.0 },
                "rotation": { "type": "f64", "default": 0.0, "note": "RESERVED" },
                "scale": { "type": "f64", "default": 1.0, "note": "RESERVED" }
            }
        })
    }
}
