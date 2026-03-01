use super::SchemaInfo;

#[derive(Clone, Debug)]
pub struct ZoneEffect {
    pub effects: Vec<ZoneEffectKind>,
}

#[derive(Clone, Debug)]
pub enum ZoneEffectKind {
    Wind { dx: f64, dy: f64, strength: f64 },
    Drag { coefficient: f64 },
    SpeedMultiplier { factor: f64 },
    Conveyor { dx: f64, dy: f64, speed: f64 },
}

impl SchemaInfo for ZoneEffect {
    fn schema_name() -> &'static str { "ZoneEffect" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "effects": {
                    "type": "Vec<ZoneEffectKind>",
                    "variants": [
                        "Wind { dx, dy, strength }",
                        "Drag { coefficient }",
                        "SpeedMultiplier { factor }",
                        "Conveyor { dx, dy, speed }"
                    ]
                }
            }
        })
    }
}
