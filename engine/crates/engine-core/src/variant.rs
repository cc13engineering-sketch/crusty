//! Variant parameter sets for branching simulation experiments.
//!
//! A `ParamSet` is an ordered collection of `(key, f64)` pairs that can be
//! applied to an engine's `global_state`. Games declare variants via the
//! `Simulation::variants()` method; the headless variant runner applies them
//! before (or during) a run to explore different tuning configurations.

use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use crate::engine::Engine;

/// An ordered set of named f64 parameters.
///
/// Uses `BTreeMap` for deterministic iteration order, which is critical
/// for reproducible simulation: applying params in a consistent order
/// ensures the same global_state regardless of platform or HashMap seed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParamSet {
    /// Optional human-readable name (e.g. "fast", "slow", "default").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// The parameter key-value pairs, ordered by key.
    params: BTreeMap<String, f64>,
}

impl ParamSet {
    /// Create an empty param set with no name.
    pub fn new() -> Self {
        Self {
            name: None,
            params: BTreeMap::new(),
        }
    }

    /// Builder method: insert a key-value pair and return self.
    pub fn with(mut self, key: &str, value: f64) -> Self {
        self.params.insert(key.to_string(), value);
        self
    }

    /// Write all parameters into `engine.global_state`.
    pub fn apply_to(&self, engine: &mut Engine) {
        for (key, value) in &self.params {
            engine.global_state.set_f64(key, *value);
        }
    }

    /// Get the optional name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name.
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    /// Builder method: set the name and return self.
    pub fn named(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Number of parameters in the set.
    pub fn len(&self) -> usize {
        self.params.len()
    }

    /// Whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    /// Iterate over the parameters in key order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, f64)> {
        self.params.iter().map(|(k, v)| (k.as_str(), *v))
    }

    /// Get a display name, falling back to "(unnamed)" if no name is set.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("(unnamed)")
    }
}

impl Default for ParamSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::Engine;

    #[test]
    fn new_is_empty() {
        let ps = ParamSet::new();
        assert!(ps.is_empty());
        assert_eq!(ps.len(), 0);
        assert!(ps.name().is_none());
    }

    #[test]
    fn builder_pattern() {
        let ps = ParamSet::new()
            .named("test")
            .with("speed", 100.0)
            .with("gravity", 9.8);
        assert_eq!(ps.name(), Some("test"));
        assert_eq!(ps.len(), 2);
        assert!(!ps.is_empty());
    }

    #[test]
    fn apply_to_engine() {
        let ps = ParamSet::new()
            .with("ball_speed", 300.0)
            .with("gravity", 5.0);
        let mut engine = Engine::new(100, 100);
        ps.apply_to(&mut engine);
        assert_eq!(engine.global_state.get_f64("ball_speed"), Some(300.0));
        assert_eq!(engine.global_state.get_f64("gravity"), Some(5.0));
    }

    #[test]
    fn set_name() {
        let mut ps = ParamSet::new();
        assert!(ps.name().is_none());
        ps.set_name("my_variant");
        assert_eq!(ps.name(), Some("my_variant"));
    }

    #[test]
    fn display_name_fallback() {
        let unnamed = ParamSet::new();
        assert_eq!(unnamed.display_name(), "(unnamed)");

        let named = ParamSet::new().named("fast");
        assert_eq!(named.display_name(), "fast");
    }

    #[test]
    fn deterministic_iteration_order() {
        let ps = ParamSet::new()
            .with("z_param", 1.0)
            .with("a_param", 2.0)
            .with("m_param", 3.0);
        let keys: Vec<&str> = ps.iter().map(|(k, _)| k).collect();
        assert_eq!(keys, vec!["a_param", "m_param", "z_param"]);
    }

    #[test]
    fn serde_roundtrip() {
        let ps = ParamSet::new()
            .named("fast")
            .with("speed", 300.0)
            .with("friction", 0.95);
        let json = serde_json::to_string(&ps).unwrap();
        let back: ParamSet = serde_json::from_str(&json).unwrap();
        assert_eq!(back.name(), Some("fast"));
        assert_eq!(back.len(), 2);
        let vals: Vec<(String, f64)> = back.iter().map(|(k, v)| (k.to_string(), v)).collect();
        assert_eq!(vals, vec![
            ("friction".to_string(), 0.95),
            ("speed".to_string(), 300.0),
        ]);
    }

    #[test]
    fn serde_roundtrip_unnamed() {
        let ps = ParamSet::new().with("x", 1.0);
        let json = serde_json::to_string(&ps).unwrap();
        assert!(!json.contains("name"));
        let back: ParamSet = serde_json::from_str(&json).unwrap();
        assert!(back.name().is_none());
        assert_eq!(back.len(), 1);
    }

    #[test]
    fn default_is_empty() {
        let ps = ParamSet::default();
        assert!(ps.is_empty());
    }
}
