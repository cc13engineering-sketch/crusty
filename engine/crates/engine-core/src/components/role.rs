use super::SchemaInfo;

#[derive(Clone, Debug, Default)]
pub struct Role {
    pub name: String,
    pub intent: String,
    pub group: Option<String>,
}

impl SchemaInfo for Role {
    fn schema_name() -> &'static str { "Role" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "name": { "type": "String" },
                "intent": { "type": "String" },
                "group": { "type": "Option<String>" }
            }
        })
    }
}
