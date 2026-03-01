use super::SchemaInfo;

/// Autonomous entity movement behavior.
#[derive(Clone, Debug)]
pub struct Behavior {
    pub mode: BehaviorMode,
    pub speed: f64,
    pub turn_rate: f64,
    pub target_tag: Option<String>,
}

#[derive(Clone, Debug)]
pub enum BehaviorMode {
    /// Move in a straight line at constant velocity (velocity set at spawn)
    Drift,
    /// Seek toward the nearest entity with target_tag
    Chase,
    /// Move away from nearest entity with target_tag
    Flee,
    /// Move toward a fixed point
    Seek { target_x: f64, target_y: f64 },
    /// Move in a circle of given radius around current position
    Orbit { radius: f64, angle: f64 },
}

impl Default for Behavior {
    fn default() -> Self {
        Self {
            mode: BehaviorMode::Drift,
            speed: 100.0,
            turn_rate: 3.0,
            target_tag: None,
        }
    }
}

impl SchemaInfo for Behavior {
    fn schema_name() -> &'static str { "Behavior" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "mode": { "type": "enum", "variants": ["Drift", "Chase", "Flee", "Seek", "Orbit"] },
                "speed": { "type": "f64", "default": 100.0 },
                "turn_rate": { "type": "f64", "default": 3.0 },
                "target_tag": { "type": "Option<String>" }
            }
        })
    }
}
