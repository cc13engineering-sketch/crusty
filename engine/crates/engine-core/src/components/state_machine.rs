use super::SchemaInfo;

/// Comparison operator for state-check transition conditions.
#[derive(Clone, Debug)]
pub enum CompareOp {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
}

impl CompareOp {
    pub fn evaluate(&self, lhs: f64, rhs: f64) -> bool {
        match self {
            CompareOp::Eq  => (lhs - rhs).abs() < f64::EPSILON,
            CompareOp::Neq => (lhs - rhs).abs() >= f64::EPSILON,
            CompareOp::Lt  => lhs < rhs,
            CompareOp::Lte => lhs <= rhs,
            CompareOp::Gt  => lhs > rhs,
            CompareOp::Gte => lhs >= rhs,
        }
    }
}

/// Condition that must be satisfied for a state transition to fire.
#[derive(Clone, Debug)]
pub enum TransitionCondition {
    /// Transition when the entity has been in the current state for at least `duration` seconds.
    After(f64),
    /// Transition when the named signal channel is active in the world's signal emitters.
    OnSignal(String),
    /// Transition when the entity's GameState value at `key` satisfies `op` against `value`.
    StateCheck { key: String, op: CompareOp, value: f64 },
    /// Transition unconditionally (first matching rule wins).
    Always,
}

/// A single edge in a finite state machine: from one named state to another under a condition.
#[derive(Clone, Debug)]
pub struct StateTransition {
    pub from: String,
    pub to: String,
    pub condition: TransitionCondition,
}

/// Finite state machine component. Tracks the current state, elapsed time in that state,
/// and the previous state for one-frame transition detection.
#[derive(Clone, Debug)]
pub struct StateMachine {
    pub current_state: String,
    pub transitions: Vec<StateTransition>,
    /// Seconds spent in the current state (reset on each transition).
    pub state_elapsed: f64,
    /// The state that was active before the most recent transition (cleared after one frame).
    pub prev_state: Option<String>,
    /// True only on the first frame of a new state.
    pub just_entered: bool,
}

impl StateMachine {
    /// Create a new state machine starting in `initial_state`.
    pub fn new(initial_state: &str) -> Self {
        Self {
            current_state: initial_state.to_string(),
            transitions: Vec::new(),
            state_elapsed: 0.0,
            prev_state: None,
            just_entered: true,
        }
    }

    /// Add a transition edge. Returns `&mut Self` for chaining.
    pub fn add_transition(
        &mut self,
        from: &str,
        to: &str,
        condition: TransitionCondition,
    ) -> &mut Self {
        self.transitions.push(StateTransition {
            from: from.to_string(),
            to: to.to_string(),
            condition,
        });
        self
    }

    /// Manually jump to a new state, resetting elapsed time and setting edge-detection fields.
    pub fn transition_to(&mut self, new_state: &str) {
        self.prev_state = Some(self.current_state.clone());
        self.current_state = new_state.to_string();
        self.state_elapsed = 0.0;
        self.just_entered = true;
    }

    /// Returns true if the machine is currently in `state`.
    pub fn is_in(&self, state: &str) -> bool {
        self.current_state == state
    }

    /// Returns true if the machine just entered `state` this frame.
    pub fn just_entered_state(&self, state: &str) -> bool {
        self.just_entered && self.current_state == state
    }

    /// Returns true if the machine just left `state` this frame (it was the previous state).
    pub fn just_left_state(&self, state: &str) -> bool {
        self.prev_state.as_deref() == Some(state)
    }
}

impl SchemaInfo for StateMachine {
    fn schema_name() -> &'static str { "StateMachine" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "current_state": { "type": "string" },
                "transitions": { "type": "Vec<StateTransition>", "note": "from/to/condition edge list" },
                "state_elapsed": { "type": "f64", "read_only": true, "note": "seconds in current state" },
                "prev_state": { "type": "Option<string>", "read_only": true, "note": "set for one frame on transition" },
                "just_entered": { "type": "bool", "read_only": true, "note": "true on first frame of new state" }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── StateMachine::new ───────────────────────────────────────────

    #[test]
    fn new_sets_initial_state() {
        let sm = StateMachine::new("idle");
        assert_eq!(sm.current_state, "idle");
        assert_eq!(sm.state_elapsed, 0.0);
        assert!(sm.prev_state.is_none());
        assert!(sm.just_entered);
    }

    #[test]
    fn new_starts_with_no_transitions() {
        let sm = StateMachine::new("walk");
        assert!(sm.transitions.is_empty());
    }

    // ─── is_in ───────────────────────────────────────────────────────

    #[test]
    fn is_in_returns_true_for_current_state() {
        let sm = StateMachine::new("idle");
        assert!(sm.is_in("idle"));
    }

    #[test]
    fn is_in_returns_false_for_other_state() {
        let sm = StateMachine::new("idle");
        assert!(!sm.is_in("run"));
    }

    // ─── transition_to ───────────────────────────────────────────────

    #[test]
    fn transition_to_changes_current_state() {
        let mut sm = StateMachine::new("idle");
        sm.transition_to("run");
        assert!(sm.is_in("run"));
    }

    #[test]
    fn transition_to_sets_prev_state() {
        let mut sm = StateMachine::new("idle");
        sm.transition_to("run");
        assert_eq!(sm.prev_state, Some("idle".to_string()));
    }

    #[test]
    fn transition_to_resets_state_elapsed() {
        let mut sm = StateMachine::new("idle");
        sm.state_elapsed = 2.5;
        sm.transition_to("run");
        assert_eq!(sm.state_elapsed, 0.0);
    }

    #[test]
    fn transition_to_sets_just_entered() {
        let mut sm = StateMachine::new("idle");
        sm.just_entered = false;
        sm.transition_to("run");
        assert!(sm.just_entered);
    }

    // ─── just_entered_state ──────────────────────────────────────────

    #[test]
    fn just_entered_state_true_on_first_frame() {
        let mut sm = StateMachine::new("idle");
        sm.transition_to("run");
        assert!(sm.just_entered_state("run"));
    }

    #[test]
    fn just_entered_state_false_after_just_entered_cleared() {
        let mut sm = StateMachine::new("idle");
        sm.transition_to("run");
        sm.just_entered = false; // simulates second frame
        assert!(!sm.just_entered_state("run"));
    }

    #[test]
    fn just_entered_state_false_for_wrong_state() {
        let mut sm = StateMachine::new("idle");
        sm.transition_to("run");
        assert!(!sm.just_entered_state("idle"));
    }

    // ─── just_left_state ─────────────────────────────────────────────

    #[test]
    fn just_left_state_true_on_transition_frame() {
        let mut sm = StateMachine::new("idle");
        sm.transition_to("run");
        assert!(sm.just_left_state("idle"));
    }

    #[test]
    fn just_left_state_false_for_non_previous_state() {
        let mut sm = StateMachine::new("idle");
        sm.transition_to("run");
        assert!(!sm.just_left_state("attack"));
    }

    #[test]
    fn just_left_state_false_when_no_prev_state() {
        let sm = StateMachine::new("idle");
        assert!(!sm.just_left_state("idle"));
    }

    // ─── prev_state clears ───────────────────────────────────────────

    #[test]
    fn prev_state_tracks_last_state() {
        let mut sm = StateMachine::new("idle");
        sm.transition_to("run");
        assert_eq!(sm.prev_state, Some("idle".to_string()));
        sm.prev_state = None; // cleared by system on next frame
        sm.transition_to("attack");
        assert_eq!(sm.prev_state, Some("run".to_string()));
    }

    // ─── add_transition / chaining ───────────────────────────────────

    #[test]
    fn add_transition_appends_to_list() {
        let mut sm = StateMachine::new("idle");
        sm.add_transition("idle", "run", TransitionCondition::Always);
        assert_eq!(sm.transitions.len(), 1);
        assert_eq!(sm.transitions[0].from, "idle");
        assert_eq!(sm.transitions[0].to, "run");
    }

    #[test]
    fn add_transition_chaining_works() {
        let mut sm = StateMachine::new("idle");
        sm.add_transition("idle", "run", TransitionCondition::Always)
          .add_transition("run", "idle", TransitionCondition::After(2.0));
        assert_eq!(sm.transitions.len(), 2);
    }

    // ─── CompareOp::evaluate ─────────────────────────────────────────

    #[test]
    fn compare_op_eq() {
        assert!(CompareOp::Eq.evaluate(1.0, 1.0));
        assert!(!CompareOp::Eq.evaluate(1.0, 2.0));
    }

    #[test]
    fn compare_op_neq() {
        assert!(CompareOp::Neq.evaluate(1.0, 2.0));
        assert!(!CompareOp::Neq.evaluate(1.0, 1.0));
    }

    #[test]
    fn compare_op_lt() {
        assert!(CompareOp::Lt.evaluate(1.0, 2.0));
        assert!(!CompareOp::Lt.evaluate(2.0, 1.0));
        assert!(!CompareOp::Lt.evaluate(1.0, 1.0));
    }

    #[test]
    fn compare_op_lte() {
        assert!(CompareOp::Lte.evaluate(1.0, 2.0));
        assert!(CompareOp::Lte.evaluate(1.0, 1.0));
        assert!(!CompareOp::Lte.evaluate(2.0, 1.0));
    }

    #[test]
    fn compare_op_gt() {
        assert!(CompareOp::Gt.evaluate(2.0, 1.0));
        assert!(!CompareOp::Gt.evaluate(1.0, 2.0));
        assert!(!CompareOp::Gt.evaluate(1.0, 1.0));
    }

    #[test]
    fn compare_op_gte() {
        assert!(CompareOp::Gte.evaluate(2.0, 1.0));
        assert!(CompareOp::Gte.evaluate(1.0, 1.0));
        assert!(!CompareOp::Gte.evaluate(1.0, 2.0));
    }

    // ─── Clone / Debug ───────────────────────────────────────────────

    #[test]
    fn state_machine_clone_and_debug() {
        let sm = StateMachine::new("idle");
        let cloned = sm.clone();
        assert_eq!(cloned.current_state, "idle");
        let debug_str = format!("{:?}", sm);
        assert!(debug_str.contains("idle"));
    }

    #[test]
    fn schema_name_and_fields() {
        assert_eq!(StateMachine::schema_name(), "StateMachine");
        let schema = StateMachine::schema();
        assert!(schema.get("fields").is_some());
    }
}
