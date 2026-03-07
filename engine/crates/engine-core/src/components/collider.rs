use super::SchemaInfo;

#[derive(Clone, Debug)]
pub struct Collider {
    pub shape: ColliderShape,
    pub is_trigger: bool,
}

#[derive(Clone, Debug)]
pub enum ColliderShape {
    Circle { radius: f64 },
    Rect { half_width: f64, half_height: f64 },
}

impl Default for Collider {
    fn default() -> Self {
        Self { shape: ColliderShape::Circle { radius: 10.0 }, is_trigger: false }
    }
}

impl SchemaInfo for Collider {
    fn schema_name() -> &'static str { "Collider" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "shape": { "type": "enum", "variants": ["Circle { radius }", "Rect { half_width, half_height }"] },
                "is_trigger": { "type": "bool", "default": false }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Default ---

    #[test]
    fn default_shape_is_circle_with_radius_10() {
        let c = Collider::default();
        match c.shape {
            ColliderShape::Circle { radius } => assert_eq!(radius, 10.0),
            _ => panic!("expected Circle, got Rect"),
        }
    }

    #[test]
    fn default_is_trigger_is_false() {
        let c = Collider::default();
        assert_eq!(c.is_trigger, false);
    }

    // --- Clone ---

    #[test]
    fn clone_circle_collider() {
        let c = Collider {
            shape: ColliderShape::Circle { radius: 25.0 },
            is_trigger: true,
        };
        let c2 = c.clone();
        match c2.shape {
            ColliderShape::Circle { radius } => assert_eq!(radius, 25.0),
            _ => panic!("expected Circle after clone"),
        }
        assert_eq!(c2.is_trigger, true);
    }

    #[test]
    fn clone_rect_collider() {
        let c = Collider {
            shape: ColliderShape::Rect { half_width: 16.0, half_height: 32.0 },
            is_trigger: false,
        };
        let c2 = c.clone();
        match c2.shape {
            ColliderShape::Rect { half_width, half_height } => {
                assert_eq!(half_width, 16.0);
                assert_eq!(half_height, 32.0);
            }
            _ => panic!("expected Rect after clone"),
        }
        assert_eq!(c2.is_trigger, false);
    }

    #[test]
    fn clone_is_independent() {
        let mut c = Collider::default();
        let c2 = c.clone();
        c.is_trigger = true;
        let _ = c; // suppress unused assignment warning
        assert_eq!(c2.is_trigger, false); // clone is unaffected
    }

    // --- SchemaInfo ---

    #[test]
    fn schema_name_returns_collider() {
        assert_eq!(Collider::schema_name(), "Collider");
    }
}
