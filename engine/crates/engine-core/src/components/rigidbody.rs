use super::SchemaInfo;

#[derive(Clone, Debug)]
pub struct RigidBody {
    pub mass: f64,
    pub vx: f64,
    pub vy: f64,
    pub ax: f64, // accumulated acceleration, reset each frame
    pub ay: f64,
    pub restitution: f64,
    pub friction: f64, // RESERVED in v1 — CCD response is frictionless
    pub is_static: bool,
    pub damping: f64,
}

impl Default for RigidBody {
    fn default() -> Self {
        Self {
            mass: 1.0, vx: 0.0, vy: 0.0, ax: 0.0, ay: 0.0,
            restitution: 0.5, friction: 0.3, is_static: false, damping: 0.01,
        }
    }
}

impl SchemaInfo for RigidBody {
    fn schema_name() -> &'static str { "RigidBody" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "mass": { "type": "f64", "default": 1.0 },
                "vx": { "type": "f64", "default": 0.0 },
                "vy": { "type": "f64", "default": 0.0 },
                "ax": { "type": "f64", "default": 0.0, "note": "set by force_accumulator" },
                "ay": { "type": "f64", "default": 0.0, "note": "set by force_accumulator" },
                "restitution": { "type": "f64", "default": 0.5, "range": [0.0, 1.0] },
                "friction": { "type": "f64", "default": 0.3, "note": "RESERVED" },
                "is_static": { "type": "bool", "default": false },
                "damping": { "type": "f64", "default": 0.01, "range": [0.0, 1.0] }
            }
        })
    }
}
