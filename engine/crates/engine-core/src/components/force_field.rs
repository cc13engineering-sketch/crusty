use super::SchemaInfo;

#[derive(Clone, Debug)]
pub struct ForceField {
    pub field_type: FieldType,
    pub strength: f64,
    pub radius: f64,
    pub falloff: Falloff,
}

#[derive(Clone, Debug)]
pub enum FieldType {
    Attract,
    Repel,
    Directional { dx: f64, dy: f64 },
    Vortex,
}

#[derive(Clone, Debug)]
pub enum Falloff {
    Constant,
    Linear,
    InverseSquare,
    /// Plummer-softened gravitational falloff: F(r) = strength * r / (r^2 + epsilon^2)^(3/2)
    /// Smooth everywhere (no singularity at r=0), peaks at r = epsilon / sqrt(2),
    /// then falls off as 1/r^2 at large distances. Ideal for gravitational wells,
    /// black holes, and any attractor/repulsor that needs smooth, stable behavior.
    Plummer { epsilon: f64 },
}

impl Default for ForceField {
    fn default() -> Self {
        Self {
            field_type: FieldType::Attract,
            strength: 100.0,
            radius: 200.0,
            falloff: Falloff::InverseSquare,
        }
    }
}

impl SchemaInfo for ForceField {
    fn schema_name() -> &'static str { "ForceField" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "field_type": { "type": "enum", "variants": ["Attract", "Repel", "Directional", "Vortex"] },
                "strength": { "type": "f64", "default": 100.0 },
                "radius": { "type": "f64", "default": 200.0 },
                "falloff": { "type": "enum", "variants": ["Constant", "Linear", "InverseSquare", "Plummer"] }
            }
        })
    }
}

/// Continuous velocity-dependent drag model.
///
/// Applies frame-rate-independent drag using an exponential decay:
///   effective_drag = base_drag + speed_drag * |velocity|
///   velocity *= exp(-effective_drag * dt)
///
/// This replaces per-frame multiplier-based damping with a physically-motivated
/// model that naturally creates terminal velocities. Use `base_drag` for
/// constant air resistance and `speed_drag` for turbulent/quadratic drag.
///
/// If `rest_threshold` > 0, velocities below this magnitude snap to zero.
#[derive(Clone, Debug)]
pub struct ContinuousDrag {
    /// Constant drag term (linear with velocity). Higher = more base resistance.
    pub base_drag: f64,
    /// Speed-proportional drag term (quadratic effect). Higher = stronger drag at high speeds.
    pub speed_drag: f64,
    /// Velocities below this magnitude snap to zero. Set to 0 to disable.
    pub rest_threshold: f64,
}

impl Default for ContinuousDrag {
    fn default() -> Self {
        Self {
            base_drag: 0.5,
            speed_drag: 0.03,
            rest_threshold: 0.3,
        }
    }
}

impl SchemaInfo for ContinuousDrag {
    fn schema_name() -> &'static str { "ContinuousDrag" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "base_drag": { "type": "f64", "default": 0.5 },
                "speed_drag": { "type": "f64", "default": 0.03 },
                "rest_threshold": { "type": "f64", "default": 0.3 }
            }
        })
    }
}

/// Edge bounce: reflects entities off world boundaries.
///
/// When an entity with this component and a RigidBody moves outside the
/// world bounds, its velocity is reflected and position clamped.
/// The `restitution` controls energy retention (0 = stop, 1 = perfect bounce,
/// >1 = speed boost on bounce).
///
/// The `margin` field offsets the bounce boundary inward (e.g., for particle
/// radius). Useful when the entity's visual or collision radius should not
/// cross the screen edge.
#[derive(Clone, Debug)]
pub struct EdgeBounce {
    /// Bounce energy retention. 0.8 = lose 20% speed on each bounce.
    pub restitution: f64,
    /// Inward margin from world bounds (e.g., entity radius).
    pub margin: f64,
}

impl Default for EdgeBounce {
    fn default() -> Self {
        Self {
            restitution: 0.8,
            margin: 0.0,
        }
    }
}

impl SchemaInfo for EdgeBounce {
    fn schema_name() -> &'static str { "EdgeBounce" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "restitution": { "type": "f64", "default": 0.8 },
                "margin": { "type": "f64", "default": 0.0 }
            }
        })
    }
}
