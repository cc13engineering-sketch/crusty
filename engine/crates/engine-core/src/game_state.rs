use std::collections::HashMap;

/// A typed value in the game state store.
#[derive(Clone, Debug, PartialEq)]
pub enum StateValue {
    F64(f64),
    Bool(bool),
    Str(String),
}

impl StateValue {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            StateValue::F64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            StateValue::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            StateValue::Str(v) => Some(v.as_str()),
            _ => None,
        }
    }
}

/// Simple key-value store for game state (score, lives, health, level, etc.).
///
/// Designed for game-level state that does not belong to any single entity.
/// Accessible from any system via `&Engine.state` or `&mut Engine.state`.
///
/// # Example
/// ```ignore
/// state.set_f64("score", 0.0);
/// state.add_f64("score", 10.0);  // score is now 10
/// state.set_f64("lives", 3.0);
/// state.add_f64("lives", -1.0);  // lives is now 2
/// ```
#[derive(Default, Clone, Debug)]
pub struct GameState {
    values: HashMap<String, StateValue>,
}

impl GameState {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    // ─── Generic set/get ───────────────────────────────────────────

    pub fn set(&mut self, key: &str, value: StateValue) {
        self.values.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&StateValue> {
        self.values.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<StateValue> {
        self.values.remove(key)
    }

    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    // ─── Typed convenience methods ─────────────────────────────────

    pub fn set_f64(&mut self, key: &str, value: f64) {
        self.values.insert(key.to_string(), StateValue::F64(value));
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.values.get(key).and_then(|v| v.as_f64())
    }

    /// Add to a numeric value (creates it with the given value if missing).
    pub fn add_f64(&mut self, key: &str, delta: f64) {
        let current = self.get_f64(key).unwrap_or(0.0);
        self.set_f64(key, current + delta);
    }

    pub fn set_bool(&mut self, key: &str, value: bool) {
        self.values.insert(key.to_string(), StateValue::Bool(value));
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.values.get(key).and_then(|v| v.as_bool())
    }

    pub fn set_str(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), StateValue::Str(value.to_string()));
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.values.get(key).and_then(|v| v.as_str())
    }

    // ─── Iteration / bulk ──────────────────────────────────────────

    pub fn iter(&self) -> impl Iterator<Item = (&str, &StateValue)> {
        self.values.iter().map(|(k, v)| (k.as_str(), v))
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Serialize the entire state to a JSON string (for WASM export).
    pub fn to_json(&self) -> String {
        let mut map = serde_json::Map::new();
        for (key, val) in &self.values {
            let json_val = match val {
                StateValue::F64(v) => serde_json::Value::from(*v),
                StateValue::Bool(v) => serde_json::Value::from(*v),
                StateValue::Str(v) => serde_json::Value::from(v.clone()),
            };
            map.insert(key.clone(), json_val);
        }
        serde_json::Value::Object(map).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_empty() {
        let gs = GameState::new();
        assert!(gs.is_empty());
        assert_eq!(gs.len(), 0);
    }

    #[test]
    fn set_and_get_f64() {
        let mut gs = GameState::new();
        gs.set_f64("score", 100.0);
        assert_eq!(gs.get_f64("score"), Some(100.0));
    }

    #[test]
    fn set_and_get_bool() {
        let mut gs = GameState::new();
        gs.set_bool("game_over", false);
        assert_eq!(gs.get_bool("game_over"), Some(false));
    }

    #[test]
    fn set_and_get_str() {
        let mut gs = GameState::new();
        gs.set_str("player_name", "Hero");
        assert_eq!(gs.get_str("player_name"), Some("Hero"));
    }

    #[test]
    fn get_missing_key_returns_none() {
        let gs = GameState::new();
        assert_eq!(gs.get_f64("nothing"), None);
        assert_eq!(gs.get_bool("nothing"), None);
        assert_eq!(gs.get_str("nothing"), None);
    }

    #[test]
    fn get_wrong_type_returns_none() {
        let mut gs = GameState::new();
        gs.set_f64("score", 10.0);
        assert_eq!(gs.get_bool("score"), None);
        assert_eq!(gs.get_str("score"), None);
    }

    #[test]
    fn add_f64_creates_if_missing() {
        let mut gs = GameState::new();
        gs.add_f64("score", 10.0);
        assert_eq!(gs.get_f64("score"), Some(10.0));
    }

    #[test]
    fn add_f64_accumulates() {
        let mut gs = GameState::new();
        gs.set_f64("score", 50.0);
        gs.add_f64("score", 25.0);
        assert_eq!(gs.get_f64("score"), Some(75.0));
    }

    #[test]
    fn add_f64_negative() {
        let mut gs = GameState::new();
        gs.set_f64("lives", 3.0);
        gs.add_f64("lives", -1.0);
        assert_eq!(gs.get_f64("lives"), Some(2.0));
    }

    #[test]
    fn has_returns_true_for_existing_key() {
        let mut gs = GameState::new();
        gs.set_f64("x", 1.0);
        assert!(gs.has("x"));
    }

    #[test]
    fn has_returns_false_for_missing_key() {
        let gs = GameState::new();
        assert!(!gs.has("x"));
    }

    #[test]
    fn remove_returns_value() {
        let mut gs = GameState::new();
        gs.set_f64("x", 42.0);
        let removed = gs.remove("x");
        assert_eq!(removed, Some(StateValue::F64(42.0)));
        assert!(!gs.has("x"));
    }

    #[test]
    fn remove_missing_returns_none() {
        let mut gs = GameState::new();
        assert_eq!(gs.remove("nothing"), None);
    }

    #[test]
    fn clear_empties_state() {
        let mut gs = GameState::new();
        gs.set_f64("a", 1.0);
        gs.set_bool("b", true);
        gs.clear();
        assert!(gs.is_empty());
    }

    #[test]
    fn len_tracks_entries() {
        let mut gs = GameState::new();
        gs.set_f64("a", 1.0);
        gs.set_bool("b", true);
        assert_eq!(gs.len(), 2);
    }

    #[test]
    fn overwrite_replaces_value() {
        let mut gs = GameState::new();
        gs.set_f64("score", 10.0);
        gs.set_f64("score", 20.0);
        assert_eq!(gs.get_f64("score"), Some(20.0));
        assert_eq!(gs.len(), 1);
    }

    #[test]
    fn overwrite_changes_type() {
        let mut gs = GameState::new();
        gs.set_f64("val", 10.0);
        gs.set_str("val", "hello");
        assert_eq!(gs.get_str("val"), Some("hello"));
        assert_eq!(gs.get_f64("val"), None);
    }

    #[test]
    fn to_json_produces_valid_json() {
        let mut gs = GameState::new();
        gs.set_f64("score", 100.0);
        gs.set_bool("alive", true);
        gs.set_str("name", "Test");
        let json = gs.to_json();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["score"], 100.0);
        assert_eq!(parsed["alive"], true);
        assert_eq!(parsed["name"], "Test");
    }

    #[test]
    fn iter_yields_all_entries() {
        let mut gs = GameState::new();
        gs.set_f64("a", 1.0);
        gs.set_f64("b", 2.0);
        let items: Vec<_> = gs.iter().collect();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn state_value_as_accessors() {
        assert_eq!(StateValue::F64(1.0).as_f64(), Some(1.0));
        assert_eq!(StateValue::F64(1.0).as_bool(), None);
        assert_eq!(StateValue::Bool(true).as_bool(), Some(true));
        assert_eq!(StateValue::Bool(true).as_f64(), None);
        assert_eq!(StateValue::Str("hi".into()).as_str(), Some("hi"));
        assert_eq!(StateValue::Str("hi".into()).as_f64(), None);
    }

    #[test]
    fn default_is_empty() {
        let gs = GameState::default();
        assert!(gs.is_empty());
    }

    #[test]
    fn clone_is_independent() {
        let mut gs = GameState::new();
        gs.set_f64("score", 10.0);
        let gs2 = gs.clone();
        gs.set_f64("score", 999.0);
        assert_eq!(gs2.get_f64("score"), Some(10.0));
    }
}
