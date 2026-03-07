use super::SchemaInfo;

#[derive(Clone, Debug)]
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub rotation: f64, // radians — RESERVED, not used by v1 systems
    pub scale: f64,    // RESERVED, not used by v1 systems
}

impl Default for Transform {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0 }
    }
}

impl SchemaInfo for Transform {
    fn schema_name() -> &'static str { "Transform" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "x": { "type": "f64", "default": 0.0 },
                "y": { "type": "f64", "default": 0.0 },
                "rotation": { "type": "f64", "default": 0.0, "note": "RESERVED" },
                "scale": { "type": "f64", "default": 1.0, "note": "RESERVED" }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_x_is_zero() {
        let t = Transform::default();
        assert_eq!(t.x, 0.0);
    }

    #[test]
    fn default_y_is_zero() {
        let t = Transform::default();
        assert_eq!(t.y, 0.0);
    }

    #[test]
    fn default_rotation_is_zero() {
        let t = Transform::default();
        assert_eq!(t.rotation, 0.0);
    }

    #[test]
    fn default_scale_is_one() {
        let t = Transform::default();
        assert_eq!(t.scale, 1.0);
    }

    #[test]
    fn clone_produces_equal_values() {
        let t = Transform { x: 5.0, y: 10.0, rotation: 1.5, scale: 2.0 };
        let t2 = t.clone();
        assert_eq!(t2.x, 5.0);
        assert_eq!(t2.y, 10.0);
        assert_eq!(t2.rotation, 1.5);
        assert_eq!(t2.scale, 2.0);
    }

    #[test]
    fn clone_is_independent() {
        let mut t = Transform { x: 1.0, y: 2.0, rotation: 0.0, scale: 1.0 };
        let t2 = t.clone();
        t.x = 99.0;
        let _ = t; // suppress unused assignment warning
        assert_eq!(t2.x, 1.0); // clone is unaffected
    }

    #[test]
    fn schema_name_returns_transform() {
        assert_eq!(Transform::schema_name(), "Transform");
    }
}
