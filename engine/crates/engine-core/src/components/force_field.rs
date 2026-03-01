use super::SchemaInfo;

#[derive(Clone, Debug)]
pub struct ForceField {
    pub field_type: FieldType,
    pub strength: f64,
    pub radius: f64,
    pub falloff: Falloff,
}

#[derive(Clone, Debug)]
pub enum FieldType {
    Attract,
    Repel,
    Directional { dx: f64, dy: f64 },
    Vortex,
}

#[derive(Clone, Debug)]
pub enum Falloff {
    Constant,
    Linear,
    InverseSquare,
}

impl Default for ForceField {
    fn default() -> Self {
        Self {
            field_type: FieldType::Attract,
            strength: 100.0,
            radius: 200.0,
            falloff: Falloff::InverseSquare,
        }
    }
}

impl SchemaInfo for ForceField {
    fn schema_name() -> &'static str { "ForceField" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "field_type": { "type": "enum", "variants": ["Attract", "Repel", "Directional", "Vortex"] },
                "strength": { "type": "f64", "default": 100.0 },
                "radius": { "type": "f64", "default": 200.0 },
                "falloff": { "type": "enum", "variants": ["Constant", "Linear", "InverseSquare"] }
            }
        })
    }
}
