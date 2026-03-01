use std::collections::HashMap;
use super::SchemaInfo;

/// Generic per-entity numeric property bag.
/// Stores arbitrary named f64 values for gameplay logic (health, score, ammo, etc).
#[derive(Clone, Debug, Default)]
pub struct GameState {
    pub values: HashMap<String, f64>,
}

impl GameState {
    pub fn new() -> Self { Self::default() }

    pub fn set(&mut self, key: &str, val: f64) {
        self.values.insert(key.to_string(), val);
    }

    pub fn get(&self, key: &str) -> f64 {
        self.values.get(key).copied().unwrap_or(0.0)
    }

    pub fn add(&mut self, key: &str, delta: f64) -> f64 {
        let entry = self.values.entry(key.to_string()).or_insert(0.0);
        *entry += delta;
        *entry
    }

    /// Returns true if value reaches zero or below after subtraction.
    pub fn subtract_check_zero(&mut self, key: &str, amount: f64) -> bool {
        let entry = self.values.entry(key.to_string()).or_insert(0.0);
        *entry -= amount;
        *entry <= 0.0
    }

    pub fn from_pairs(pairs: &[(&str, f64)]) -> Self {
        let mut gs = Self::new();
        for (k, v) in pairs {
            gs.set(k, *v);
        }
        gs
    }
}

impl SchemaInfo for GameState {
    fn schema_name() -> &'static str { "GameState" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "values": { "type": "HashMap<String, f64>", "note": "arbitrary key-value numeric pairs" }
            }
        })
    }
}
