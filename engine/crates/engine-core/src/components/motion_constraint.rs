use super::SchemaInfo;

#[derive(Clone, Debug)]
pub struct MotionConstraint {
    pub max_speed: Option<f64>,
    pub min_speed: Option<f64>,
    pub lock_x: bool,
    pub lock_y: bool,
}

impl Default for MotionConstraint {
    fn default() -> Self {
        Self {
            max_speed: None,
            min_speed: None,
            lock_x: false,
            lock_y: false,
        }
    }
}

impl SchemaInfo for MotionConstraint {
    fn schema_name() -> &'static str { "MotionConstraint" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "max_speed": { "type": "Option<f64>", "default": "None", "note": "clamp speed to this max" },
                "min_speed": { "type": "Option<f64>", "default": "None", "note": "speeds below this snap to zero" },
                "lock_x": { "type": "bool", "default": false, "note": "prevent x movement" },
                "lock_y": { "type": "bool", "default": false, "note": "prevent y movement" }
            }
        })
    }
}
