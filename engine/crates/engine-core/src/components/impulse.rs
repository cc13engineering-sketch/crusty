use super::SchemaInfo;

#[derive(Clone, Debug)]
pub struct Impulse {
    pub dvx: f64,
    pub dvy: f64,
}

impl SchemaInfo for Impulse {
    fn schema_name() -> &'static str { "Impulse" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "dvx": { "type": "f64", "note": "instantaneous velocity delta X" },
                "dvy": { "type": "f64", "note": "instantaneous velocity delta Y" }
            }
        })
    }
}
