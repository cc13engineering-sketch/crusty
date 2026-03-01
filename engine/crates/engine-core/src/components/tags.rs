use super::SchemaInfo;

#[derive(Clone, Debug, Default)]
pub struct Tags {
    pub values: Vec<String>,
}

impl Tags {
    pub fn has(&self, tag: &str) -> bool {
        self.values.iter().any(|t| t == tag)
    }
    pub fn new(tags: &[&str]) -> Self {
        Self { values: tags.iter().map(|s| s.to_string()).collect() }
    }
}

impl SchemaInfo for Tags {
    fn schema_name() -> &'static str { "Tags" }
    fn schema() -> serde_json::Value {
        serde_json::json!({ "fields": { "values": { "type": "Vec<String>" } } })
    }
}
