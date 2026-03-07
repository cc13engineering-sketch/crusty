use super::SchemaInfo;

#[derive(Clone, Debug)]
pub struct RigidBody {
    pub mass: f64,
    pub vx: f64,
    pub vy: f64,
    pub ax: f64, // accumulated acceleration, reset each frame
    pub ay: f64,
    pub restitution: f64,
    pub friction: f64, // RESERVED in v1 — CCD response is frictionless
    pub is_static: bool,
    pub damping: f64,
}

impl Default for RigidBody {
    fn default() -> Self {
        Self {
            mass: 1.0, vx: 0.0, vy: 0.0, ax: 0.0, ay: 0.0,
            restitution: 0.5, friction: 0.3, is_static: false, damping: 0.01,
        }
    }
}

impl SchemaInfo for RigidBody {
    fn schema_name() -> &'static str { "RigidBody" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "mass": { "type": "f64", "default": 1.0 },
                "vx": { "type": "f64", "default": 0.0 },
                "vy": { "type": "f64", "default": 0.0 },
                "ax": { "type": "f64", "default": 0.0, "note": "set by force_accumulator" },
                "ay": { "type": "f64", "default": 0.0, "note": "set by force_accumulator" },
                "restitution": { "type": "f64", "default": 0.5, "range": [0.0, 1.0] },
                "friction": { "type": "f64", "default": 0.3, "note": "RESERVED" },
                "is_static": { "type": "bool", "default": false },
                "damping": { "type": "f64", "default": 0.01, "range": [0.0, 1.0] }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_mass_is_one() {
        let rb = RigidBody::default();
        assert_eq!(rb.mass, 1.0);
    }

    #[test]
    fn default_velocity_is_zero() {
        let rb = RigidBody::default();
        assert_eq!(rb.vx, 0.0);
        assert_eq!(rb.vy, 0.0);
    }

    #[test]
    fn default_acceleration_is_zero() {
        let rb = RigidBody::default();
        assert_eq!(rb.ax, 0.0);
        assert_eq!(rb.ay, 0.0);
    }

    #[test]
    fn default_restitution_is_half() {
        let rb = RigidBody::default();
        assert_eq!(rb.restitution, 0.5);
    }

    #[test]
    fn default_friction_is_point_three() {
        let rb = RigidBody::default();
        assert_eq!(rb.friction, 0.3);
    }

    #[test]
    fn default_is_static_is_false() {
        let rb = RigidBody::default();
        assert_eq!(rb.is_static, false);
    }

    #[test]
    fn default_damping_is_point_zero_one() {
        let rb = RigidBody::default();
        assert_eq!(rb.damping, 0.01);
    }

    #[test]
    fn clone_produces_equal_values() {
        let rb = RigidBody {
            mass: 5.0,
            vx: 1.0,
            vy: 2.0,
            ax: 3.0,
            ay: 4.0,
            restitution: 0.8,
            friction: 0.2,
            is_static: true,
            damping: 0.05,
        };
        let rb2 = rb.clone();
        assert_eq!(rb2.mass, 5.0);
        assert_eq!(rb2.vx, 1.0);
        assert_eq!(rb2.vy, 2.0);
        assert_eq!(rb2.ax, 3.0);
        assert_eq!(rb2.ay, 4.0);
        assert_eq!(rb2.restitution, 0.8);
        assert_eq!(rb2.friction, 0.2);
        assert_eq!(rb2.is_static, true);
        assert_eq!(rb2.damping, 0.05);
    }

    #[test]
    fn clone_is_independent() {
        let mut rb = RigidBody::default();
        let rb2 = rb.clone();
        rb.mass = 99.0;
        let _ = rb; // suppress unused assignment warning
        assert_eq!(rb2.mass, 1.0); // clone is unaffected
    }

    #[test]
    fn schema_name_returns_rigidbody() {
        assert_eq!(RigidBody::schema_name(), "RigidBody");
    }
}
