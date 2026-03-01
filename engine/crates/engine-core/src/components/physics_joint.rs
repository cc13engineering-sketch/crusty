use super::SchemaInfo;
use crate::ecs::Entity;

/// Types of physics joints connecting two entities.
#[derive(Clone, Debug)]
pub enum JointType {
    /// Distance joint: maintains a fixed distance between two entities.
    /// Like a rigid rod.
    Distance {
        /// Rest length of the joint.
        length: f64,
        /// Stiffness (0..1): how rigidly the length is enforced.
        /// 1.0 = perfectly rigid, 0.1 = very springy.
        stiffness: f64,
        /// Damping (0..1): how quickly oscillations die.
        damping: f64,
    },
    /// Spring joint: like distance but with spring dynamics.
    Spring {
        rest_length: f64,
        /// Spring constant (higher = stiffer).
        k: f64,
        /// Damping coefficient.
        damping: f64,
    },
    /// Rope joint: prevents entities from being farther than max_length apart,
    /// but allows them to be closer.
    Rope {
        max_length: f64,
    },
    /// Hinge joint: constrains entity B to orbit around entity A at a fixed radius.
    /// Useful for doors, swinging platforms, etc.
    Hinge {
        radius: f64,
        /// Angular velocity in radians/second.
        angular_velocity: f64,
        /// Current angle in radians.
        angle: f64,
        /// Min/max angle limits (None = unconstrained).
        min_angle: Option<f64>,
        max_angle: Option<f64>,
    },
}

/// A physics joint connecting entity_a and entity_b.
#[derive(Clone, Debug)]
pub struct PhysicsJoint {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub joint_type: JointType,
    /// If true, the joint has broken and should be removed.
    pub broken: bool,
    /// Optional break force threshold — if tension exceeds this, joint breaks.
    pub break_force: Option<f64>,
}

impl PhysicsJoint {
    pub fn distance(a: Entity, b: Entity, length: f64) -> Self {
        Self {
            entity_a: a,
            entity_b: b,
            joint_type: JointType::Distance {
                length,
                stiffness: 1.0,
                damping: 0.5,
            },
            broken: false,
            break_force: None,
        }
    }

    pub fn spring(a: Entity, b: Entity, rest_length: f64, k: f64, damping: f64) -> Self {
        Self {
            entity_a: a,
            entity_b: b,
            joint_type: JointType::Spring { rest_length, k, damping },
            broken: false,
            break_force: None,
        }
    }

    pub fn rope(a: Entity, b: Entity, max_length: f64) -> Self {
        Self {
            entity_a: a,
            entity_b: b,
            joint_type: JointType::Rope { max_length },
            broken: false,
            break_force: None,
        }
    }

    pub fn hinge(a: Entity, b: Entity, radius: f64) -> Self {
        Self {
            entity_a: a,
            entity_b: b,
            joint_type: JointType::Hinge {
                radius,
                angular_velocity: 0.0,
                angle: 0.0,
                min_angle: None,
                max_angle: None,
            },
            broken: false,
            break_force: None,
        }
    }

    pub fn with_break_force(mut self, force: f64) -> Self {
        self.break_force = Some(force);
        self
    }
}

impl SchemaInfo for PhysicsJoint {
    fn schema_name() -> &'static str { "PhysicsJoint" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "description": "Physics joint connecting two entities",
            "properties": {
                "entity_a": { "type": "integer" },
                "entity_b": { "type": "integer" },
                "joint_type": {
                    "enum": ["Distance", "Spring", "Rope", "Hinge"]
                },
                "break_force": { "type": "number", "nullable": true }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_distance_joint() {
        let j = PhysicsJoint::distance(Entity(1), Entity(2), 50.0);
        assert_eq!(j.entity_a, Entity(1));
        assert_eq!(j.entity_b, Entity(2));
        assert!(!j.broken);
        match j.joint_type {
            JointType::Distance { length, .. } => assert_eq!(length, 50.0),
            _ => panic!("wrong type"),
        }
    }

    #[test]
    fn create_spring_joint() {
        let j = PhysicsJoint::spring(Entity(1), Entity(2), 30.0, 100.0, 0.5);
        match j.joint_type {
            JointType::Spring { rest_length, k, damping } => {
                assert_eq!(rest_length, 30.0);
                assert_eq!(k, 100.0);
                assert_eq!(damping, 0.5);
            }
            _ => panic!("wrong type"),
        }
    }

    #[test]
    fn create_rope_joint() {
        let j = PhysicsJoint::rope(Entity(3), Entity(4), 100.0);
        match j.joint_type {
            JointType::Rope { max_length } => assert_eq!(max_length, 100.0),
            _ => panic!("wrong type"),
        }
    }

    #[test]
    fn create_hinge_joint() {
        let j = PhysicsJoint::hinge(Entity(5), Entity(6), 25.0);
        match j.joint_type {
            JointType::Hinge { radius, angular_velocity, .. } => {
                assert_eq!(radius, 25.0);
                assert_eq!(angular_velocity, 0.0);
            }
            _ => panic!("wrong type"),
        }
    }

    #[test]
    fn break_force_builder() {
        let j = PhysicsJoint::distance(Entity(1), Entity(2), 50.0)
            .with_break_force(500.0);
        assert_eq!(j.break_force, Some(500.0));
    }

    #[test]
    fn default_not_broken() {
        let j = PhysicsJoint::rope(Entity(1), Entity(2), 10.0);
        assert!(!j.broken);
        assert!(j.break_force.is_none());
    }
}
