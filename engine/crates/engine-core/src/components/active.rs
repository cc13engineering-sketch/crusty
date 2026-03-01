use super::SchemaInfo;

/// Active flag for entities. When enabled=false, most systems skip this entity.
/// This allows temporarily disabling entities without despawning them.
#[derive(Clone, Debug)]
pub struct Active {
    pub enabled: bool,
}

impl Active {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    pub fn enabled() -> Self {
        Self { enabled: true }
    }

    pub fn disabled() -> Self {
        Self { enabled: false }
    }
}

impl SchemaInfo for Active {
    fn schema_name() -> &'static str { "Active" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "enabled": { "type": "bool", "default": true }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_enabled() {
        let a = Active::new(true);
        assert!(a.enabled);
    }

    #[test]
    fn test_new_disabled() {
        let a = Active::new(false);
        assert!(!a.enabled);
    }

    #[test]
    fn test_enabled_constructor() {
        let a = Active::enabled();
        assert!(a.enabled);
    }

    #[test]
    fn test_disabled_constructor() {
        let a = Active::disabled();
        assert!(!a.enabled);
    }

    #[test]
    fn test_clone_enabled() {
        let a = Active::enabled();
        let cloned = a.clone();
        assert!(cloned.enabled);
    }

    #[test]
    fn test_clone_disabled() {
        let a = Active::disabled();
        let cloned = a.clone();
        assert!(!cloned.enabled);
    }

    #[test]
    fn test_debug_enabled() {
        let a = Active::enabled();
        let debug_str = format!("{:?}", a);
        assert!(debug_str.contains("Active"));
        assert!(debug_str.contains("true"));
    }

    #[test]
    fn test_debug_disabled() {
        let a = Active::disabled();
        let debug_str = format!("{:?}", a);
        assert!(debug_str.contains("Active"));
        assert!(debug_str.contains("false"));
    }

    #[test]
    fn test_schema_name() {
        assert_eq!(Active::schema_name(), "Active");
    }

    #[test]
    fn test_schema_is_valid_json() {
        let schema = Active::schema();
        assert!(schema.get("fields").is_some());
        let fields = schema.get("fields").unwrap();
        assert!(fields.get("enabled").is_some());
    }

    #[test]
    fn test_schema_enabled_field_type() {
        let schema = Active::schema();
        let fields = schema.get("fields").unwrap();
        let enabled_field = fields.get("enabled").unwrap();
        assert_eq!(enabled_field.get("type").unwrap(), "bool");
    }

    #[test]
    fn test_schema_enabled_field_default() {
        let schema = Active::schema();
        let fields = schema.get("fields").unwrap();
        let enabled_field = fields.get("enabled").unwrap();
        assert_eq!(enabled_field.get("default").unwrap(), true);
    }

    #[test]
    fn test_field_is_public_and_mutable() {
        let mut a = Active::enabled();
        assert!(a.enabled);
        a.enabled = false;
        assert!(!a.enabled);
        a.enabled = true;
        assert!(a.enabled);
    }

    #[test]
    fn test_toggle_behavior() {
        let mut a = Active::enabled();
        a.enabled = !a.enabled;
        assert!(!a.enabled);
        a.enabled = !a.enabled;
        assert!(a.enabled);
    }

    #[test]
    fn test_clone_independence() {
        let a = Active::enabled();
        let mut cloned = a.clone();
        cloned.enabled = false;
        // Original should be unaffected
        assert!(a.enabled);
        assert!(!cloned.enabled);
    }
}
