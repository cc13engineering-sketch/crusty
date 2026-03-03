//! Feel Presets — A library of named physics feel profiles.
//!
//! Each `FeelPreset` is a named collection of physics parameters (gravity, friction,
//! damping, max_speed, etc.) that can be applied to the engine's global state to
//! instantly change how the game "feels". Presets can be serialized to/from TOML
//! and JSON for sharing and tweaking.

use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// A named physics feel profile containing key-value physics parameters.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeelPreset {
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub category: String,
    pub params: BTreeMap<String, f64>,
}

impl FeelPreset {
    /// Create a new empty preset with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            category: String::new(),
            params: BTreeMap::new(),
        }
    }

    /// Set a parameter value. Returns `&mut Self` for chaining.
    pub fn set(&mut self, key: &str, value: f64) -> &mut Self {
        self.params.insert(key.to_string(), value);
        self
    }

    /// Get a parameter value by key.
    pub fn get(&self, key: &str) -> Option<f64> {
        self.params.get(key).copied()
    }

    /// Apply all preset parameters to the engine's global state.
    pub fn apply(&self, engine: &mut crate::engine::Engine) {
        for (key, value) in &self.params {
            engine.global_state.set_f64(key, *value);
        }
    }

    /// Apply preset parameters with specific overrides.
    /// The preset values are applied first, then overrides replace any matching keys.
    pub fn apply_with_overrides(
        &self,
        engine: &mut crate::engine::Engine,
        overrides: &[(String, f64)],
    ) {
        // Apply base preset
        self.apply(engine);
        // Apply overrides on top
        for (key, value) in overrides {
            engine.global_state.set_f64(key, *value);
        }
    }

    /// Export a preset from the current engine state by capturing specified keys.
    pub fn export_from_state(
        name: &str,
        keys: &[&str],
        engine: &crate::engine::Engine,
    ) -> Self {
        let mut preset = Self::new(name);
        for key in keys {
            if let Some(value) = engine.global_state.get_f64(key) {
                preset.params.insert(key.to_string(), value);
            }
        }
        preset
    }

    /// Merge another preset's parameters into this one.
    /// Values from `other` override existing values in `self`.
    pub fn merge(&mut self, other: &FeelPreset) {
        for (key, value) in &other.params {
            self.params.insert(key.clone(), *value);
        }
    }

    /// Serialize this preset to a TOML string.
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    /// Deserialize a preset from a TOML string.
    pub fn from_toml(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }

    /// Serialize this preset to a JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize a preset from a JSON string.
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

/// A collection of named feel presets.
pub struct FeelPresetLibrary {
    presets: BTreeMap<String, FeelPreset>,
}

impl FeelPresetLibrary {
    /// Create a new empty library.
    pub fn new() -> Self {
        Self {
            presets: BTreeMap::new(),
        }
    }

    /// Create a library pre-loaded with all built-in presets.
    pub fn with_builtins() -> Self {
        let mut lib = Self::new();

        // 1. Tight Platformer
        let mut p = FeelPreset::new("tight_platformer");
        p.description = "Responsive platformer with snappy controls and no float".to_string();
        p.category = "platformer".to_string();
        p.set("physics.gravity", 980.0);
        p.set("physics.restitution", 0.0);
        p.set("physics.friction", 0.95);
        p.set("physics.damping", 0.05);
        p.set("player.max_speed", 300.0);
        p.set("player.acceleration", 2000.0);
        p.set("player.jump_force", 450.0);
        p.set("player.air_control", 0.8);
        lib.register(p);

        // 2. Floaty Astronaut
        let mut p = FeelPreset::new("floaty_astronaut");
        p.description = "Low-gravity space movement with full air control".to_string();
        p.category = "space".to_string();
        p.set("physics.gravity", 60.0);
        p.set("physics.restitution", 0.3);
        p.set("physics.friction", 0.02);
        p.set("physics.damping", 0.005);
        p.set("player.max_speed", 150.0);
        p.set("player.acceleration", 200.0);
        p.set("player.jump_force", 100.0);
        p.set("player.air_control", 1.0);
        lib.register(p);

        // 3. Heavy Tank
        let mut p = FeelPreset::new("heavy_tank");
        p.description = "Slow, weighty vehicle with high mass and momentum".to_string();
        p.category = "vehicle".to_string();
        p.set("physics.gravity", 980.0);
        p.set("physics.restitution", 0.1);
        p.set("physics.friction", 0.7);
        p.set("physics.damping", 0.15);
        p.set("player.max_speed", 120.0);
        p.set("player.acceleration", 300.0);
        p.set("physics.mass", 10.0);
        p.set("player.turn_speed", 1.5);
        lib.register(p);

        // 4. Snappy Cursor
        let mut p = FeelPreset::new("snappy_cursor");
        p.description = "Zero-gravity instant-response cursor for UI or puzzle games".to_string();
        p.category = "cursor".to_string();
        p.set("physics.gravity", 0.0);
        p.set("physics.restitution", 0.0);
        p.set("physics.friction", 0.0);
        p.set("physics.damping", 0.99);
        p.set("player.max_speed", 800.0);
        p.set("player.acceleration", 10000.0);
        lib.register(p);

        // 5. Underwater
        let mut p = FeelPreset::new("underwater");
        p.description = "Submerged movement with buoyancy and heavy drag".to_string();
        p.category = "environment".to_string();
        p.set("physics.gravity", 200.0);
        p.set("physics.restitution", 0.05);
        p.set("physics.friction", 0.4);
        p.set("physics.damping", 0.3);
        p.set("physics.buoyancy", -150.0);
        p.set("player.max_speed", 100.0);
        p.set("player.acceleration", 500.0);
        p.set("physics.drag_coefficient", 2.0);
        lib.register(p);

        // 6. Ice Skating
        let mut p = FeelPreset::new("ice_skating");
        p.description = "Slippery surface with minimal friction and slow turning".to_string();
        p.category = "environment".to_string();
        p.set("physics.gravity", 980.0);
        p.set("physics.restitution", 0.2);
        p.set("physics.friction", 0.01);
        p.set("physics.damping", 0.001);
        p.set("player.max_speed", 400.0);
        p.set("player.acceleration", 800.0);
        p.set("player.turn_speed", 0.5);
        lib.register(p);

        lib
    }

    /// Register a preset in the library. Overwrites any existing preset with the same name.
    pub fn register(&mut self, preset: FeelPreset) {
        self.presets.insert(preset.name.clone(), preset);
    }

    /// Get a preset by name.
    pub fn get(&self, name: &str) -> Option<&FeelPreset> {
        self.presets.get(name)
    }

    /// List all preset names in sorted order.
    pub fn list(&self) -> Vec<&str> {
        self.presets.keys().map(|s| s.as_str()).collect()
    }

    /// Return the number of presets in the library.
    pub fn len(&self) -> usize {
        self.presets.len()
    }

    /// Check if the library is empty.
    pub fn is_empty(&self) -> bool {
        self.presets.is_empty()
    }

    /// Load a preset from a TOML string and add it to the library.
    /// Returns the preset name on success, or an error message on failure.
    pub fn load_toml(&mut self, toml_str: &str) -> Result<String, String> {
        match FeelPreset::from_toml(toml_str) {
            Ok(preset) => {
                let name = preset.name.clone();
                self.register(preset);
                Ok(name)
            }
            Err(e) => Err(format!("Failed to parse TOML preset: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_library_has_all_presets() {
        let lib = FeelPresetLibrary::with_builtins();
        assert!(lib.len() >= 6, "Expected at least 6 built-in presets, got {}", lib.len());
        assert!(lib.get("tight_platformer").is_some());
        assert!(lib.get("floaty_astronaut").is_some());
        assert!(lib.get("heavy_tank").is_some());
        assert!(lib.get("snappy_cursor").is_some());
        assert!(lib.get("underwater").is_some());
        assert!(lib.get("ice_skating").is_some());
    }

    #[test]
    fn preset_apply_sets_global_state() {
        let lib = FeelPresetLibrary::with_builtins();
        let preset = lib.get("tight_platformer").unwrap();
        let mut engine = crate::engine::Engine::new(1, 1);

        preset.apply(&mut engine);

        assert_eq!(engine.global_state.get_f64("physics.gravity"), Some(980.0));
        assert_eq!(engine.global_state.get_f64("physics.restitution"), Some(0.0));
        assert_eq!(engine.global_state.get_f64("physics.friction"), Some(0.95));
        assert_eq!(engine.global_state.get_f64("player.max_speed"), Some(300.0));
        assert_eq!(engine.global_state.get_f64("player.jump_force"), Some(450.0));
    }

    #[test]
    fn preset_apply_with_overrides() {
        let lib = FeelPresetLibrary::with_builtins();
        let preset = lib.get("tight_platformer").unwrap();
        let mut engine = crate::engine::Engine::new(1, 1);

        let overrides = vec![
            ("physics.gravity".to_string(), 500.0),
            ("player.max_speed".to_string(), 999.0),
        ];
        preset.apply_with_overrides(&mut engine, &overrides);

        // Overridden values
        assert_eq!(engine.global_state.get_f64("physics.gravity"), Some(500.0));
        assert_eq!(engine.global_state.get_f64("player.max_speed"), Some(999.0));
        // Non-overridden values from preset
        assert_eq!(engine.global_state.get_f64("physics.friction"), Some(0.95));
        assert_eq!(engine.global_state.get_f64("player.jump_force"), Some(450.0));
    }

    #[test]
    fn preset_toml_roundtrip() {
        let lib = FeelPresetLibrary::with_builtins();
        let original = lib.get("floaty_astronaut").unwrap();

        let toml_str = original.to_toml().expect("TOML serialization failed");
        let restored = FeelPreset::from_toml(&toml_str).expect("TOML deserialization failed");

        assert_eq!(restored.name, original.name);
        assert_eq!(restored.description, original.description);
        assert_eq!(restored.category, original.category);
        assert_eq!(restored.params.len(), original.params.len());
        for (key, value) in &original.params {
            assert_eq!(restored.get(key), Some(*value), "Mismatch on key '{}'", key);
        }
    }

    #[test]
    fn preset_json_roundtrip() {
        let lib = FeelPresetLibrary::with_builtins();
        let original = lib.get("underwater").unwrap();

        let json_str = original.to_json().expect("JSON serialization failed");
        let restored = FeelPreset::from_json(&json_str).expect("JSON deserialization failed");

        assert_eq!(restored.name, original.name);
        assert_eq!(restored.description, original.description);
        assert_eq!(restored.category, original.category);
        assert_eq!(restored.params.len(), original.params.len());
        for (key, value) in &original.params {
            assert_eq!(restored.get(key), Some(*value), "Mismatch on key '{}'", key);
        }
    }

    #[test]
    fn export_from_state_captures_keys() {
        let mut engine = crate::engine::Engine::new(1, 1);
        engine.global_state.set_f64("physics.gravity", 123.0);
        engine.global_state.set_f64("physics.friction", 0.5);
        engine.global_state.set_f64("player.max_speed", 250.0);
        engine.global_state.set_f64("unrelated.key", 999.0);

        let exported = FeelPreset::export_from_state(
            "captured",
            &["physics.gravity", "physics.friction", "player.max_speed", "missing.key"],
            &engine,
        );

        assert_eq!(exported.name, "captured");
        assert_eq!(exported.get("physics.gravity"), Some(123.0));
        assert_eq!(exported.get("physics.friction"), Some(0.5));
        assert_eq!(exported.get("player.max_speed"), Some(250.0));
        // Missing keys should not appear
        assert_eq!(exported.get("missing.key"), None);
        // Unrelated keys should not appear
        assert_eq!(exported.get("unrelated.key"), None);
        assert_eq!(exported.params.len(), 3);
    }

    #[test]
    fn merge_overrides_values() {
        let mut base = FeelPreset::new("base");
        base.set("physics.gravity", 980.0);
        base.set("physics.friction", 0.5);
        base.set("player.max_speed", 300.0);

        let mut override_preset = FeelPreset::new("override");
        override_preset.set("physics.friction", 0.01);
        override_preset.set("player.max_speed", 800.0);
        override_preset.set("player.jump_force", 600.0);

        base.merge(&override_preset);

        // Original value unchanged
        assert_eq!(base.get("physics.gravity"), Some(980.0));
        // Overridden values
        assert_eq!(base.get("physics.friction"), Some(0.01));
        assert_eq!(base.get("player.max_speed"), Some(800.0));
        // New value from merge
        assert_eq!(base.get("player.jump_force"), Some(600.0));
    }

    #[test]
    fn load_toml_adds_to_library() {
        let mut lib = FeelPresetLibrary::new();
        assert!(lib.is_empty());

        let toml_str = r#"
name = "custom_feel"
description = "A custom feel preset"
category = "custom"

[params]
"physics.gravity" = 500.0
"physics.friction" = 0.3
"player.max_speed" = 200.0
"#;

        let result = lib.load_toml(toml_str);
        assert!(result.is_ok(), "load_toml failed: {:?}", result);
        assert_eq!(result.unwrap(), "custom_feel");
        assert_eq!(lib.len(), 1);

        let preset = lib.get("custom_feel").unwrap();
        assert_eq!(preset.description, "A custom feel preset");
        assert_eq!(preset.category, "custom");
        assert_eq!(preset.get("physics.gravity"), Some(500.0));
        assert_eq!(preset.get("physics.friction"), Some(0.3));
        assert_eq!(preset.get("player.max_speed"), Some(200.0));
    }
}
