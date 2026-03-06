use super::SchemaInfo;

/// Phase of a launch/sling mechanic.
#[derive(Clone, Debug, PartialEq)]
pub enum LaunchPhase {
    /// Waiting for input
    Idle,
    /// Player is dragging to aim
    Aiming {
        /// Where the drag started (world coords)
        origin_x: f64,
        origin_y: f64,
    },
    /// Entity has been launched
    Launched {
        /// Time since launch (seconds)
        elapsed: f64,
        /// Force immunity factor (1.0 = full immunity, decays toward 0)
        immunity: f64,
    },
}

/// Sling/launch mechanic state for an entity.
/// Tracks the aiming phase, launch parameters, and post-launch timers.
///
/// This is deliberately minimal -- games will iterate on specifics.
/// Breaking changes are expected as more games adopt this component.
#[derive(Clone, Debug)]
pub struct LaunchState {
    pub phase: LaunchPhase,
    /// Maximum pull distance (pixels). Pull is clamped to this.
    pub max_pull: f64,
    /// Power multiplier: final velocity = (pull_distance / max_pull) * power
    pub power: f64,
    /// Post-launch force immunity duration (seconds). Entity ignores force fields.
    pub immunity_duration: f64,
    /// Immunity decay rate (units per second). Factor decreases at this rate.
    pub immunity_decay_rate: f64,
    /// Hard stop: zero velocity after this many seconds of flight. 0 = disabled.
    pub hard_stop_time: f64,
}

impl Default for LaunchState {
    fn default() -> Self {
        Self {
            phase: LaunchPhase::Idle,
            max_pull: 150.0,
            power: 500.0,
            immunity_duration: 1.0,
            immunity_decay_rate: 2.5,
            hard_stop_time: 0.0,
        }
    }
}

impl SchemaInfo for LaunchState {
    fn schema_name() -> &'static str { "LaunchState" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "phase": { "type": "LaunchPhase", "default": "Idle" },
                "max_pull": { "type": "f64", "default": 150.0 },
                "power": { "type": "f64", "default": 500.0 },
                "immunity_duration": { "type": "f64", "default": 1.0 },
                "immunity_decay_rate": { "type": "f64", "default": 2.5 },
                "hard_stop_time": { "type": "f64", "default": 0.0 }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-6;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    // ── Default values ──────────────────────────────────────────────

    #[test]
    fn default_phase_is_idle() {
        let state = LaunchState::default();
        assert_eq!(state.phase, LaunchPhase::Idle);
    }

    #[test]
    fn default_max_pull() {
        let state = LaunchState::default();
        assert!(approx_eq(state.max_pull, 150.0));
    }

    #[test]
    fn default_power() {
        let state = LaunchState::default();
        assert!(approx_eq(state.power, 500.0));
    }

    #[test]
    fn default_immunity_duration() {
        let state = LaunchState::default();
        assert!(approx_eq(state.immunity_duration, 1.0));
    }

    #[test]
    fn default_immunity_decay_rate() {
        let state = LaunchState::default();
        assert!(approx_eq(state.immunity_decay_rate, 2.5));
    }

    #[test]
    fn default_hard_stop_time_disabled() {
        let state = LaunchState::default();
        assert!(approx_eq(state.hard_stop_time, 0.0));
    }

    // ── Phase transitions ───────────────────────────────────────────

    #[test]
    fn transition_idle_to_aiming() {
        let mut state = LaunchState::default();
        assert_eq!(state.phase, LaunchPhase::Idle);

        state.phase = LaunchPhase::Aiming {
            origin_x: 100.0,
            origin_y: 200.0,
        };

        match &state.phase {
            LaunchPhase::Aiming { origin_x, origin_y } => {
                assert!(approx_eq(*origin_x, 100.0));
                assert!(approx_eq(*origin_y, 200.0));
            }
            _ => panic!("expected Aiming phase"),
        }
    }

    #[test]
    fn transition_aiming_to_launched() {
        let mut state = LaunchState::default();
        state.phase = LaunchPhase::Aiming {
            origin_x: 50.0,
            origin_y: 75.0,
        };

        state.phase = LaunchPhase::Launched {
            elapsed: 0.0,
            immunity: 1.0,
        };

        match &state.phase {
            LaunchPhase::Launched { elapsed, immunity } => {
                assert!(approx_eq(*elapsed, 0.0));
                assert!(approx_eq(*immunity, 1.0));
            }
            _ => panic!("expected Launched phase"),
        }
    }

    #[test]
    fn transition_launched_back_to_idle() {
        let mut state = LaunchState::default();
        state.phase = LaunchPhase::Launched {
            elapsed: 2.0,
            immunity: 0.0,
        };

        state.phase = LaunchPhase::Idle;
        assert_eq!(state.phase, LaunchPhase::Idle);
    }

    // ── Schema ──────────────────────────────────────────────────────

    #[test]
    fn schema_name_is_launch_state() {
        assert_eq!(LaunchState::schema_name(), "LaunchState");
    }

    #[test]
    fn schema_contains_expected_fields() {
        let schema = LaunchState::schema();
        let fields = schema.get("fields").expect("schema should have fields");
        assert!(fields.get("phase").is_some());
        assert!(fields.get("max_pull").is_some());
        assert!(fields.get("power").is_some());
        assert!(fields.get("immunity_duration").is_some());
        assert!(fields.get("immunity_decay_rate").is_some());
        assert!(fields.get("hard_stop_time").is_some());
    }
}
