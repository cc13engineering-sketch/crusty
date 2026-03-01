/// GameFlow — Declarative game lifecycle state machine
///
/// Manages high-level game states (Title, Playing, Paused, GameOver, Victory,
/// or any custom state) with declarative transition rules. Conditions are
/// evaluated each frame; the first matching transition fires automatically.
///
/// # Example
/// ```ignore
/// let mut flow = GameFlow::new();
/// flow.add_transition(FlowTransition {
///     from: FlowState::Playing,
///     to: FlowState::GameOver,
///     condition: FlowCondition::StateReaches {
///         key: "health".into(),
///         op: CompareOp::Le,
///         value: 0.0,
///     },
///     sound_on_enter: Some("game_over".into()),
/// });
/// // In update loop:
/// if let Some(new_state) = flow.update(dt, &game_state, &event_bus) {
///     // React to state change
/// }
/// ```

use crate::game_state::GameState;
use crate::event_bus::EventBus;

// ─── FlowState ──────────────────────────────────────────────────────────────

/// A top-level game lifecycle state.
#[derive(Clone, Debug, PartialEq)]
pub enum FlowState {
    /// The title/main-menu screen.
    Title,
    /// Active gameplay.
    Playing,
    /// Game is paused (gameplay suspended).
    Paused,
    /// The game-over screen.
    GameOver,
    /// The victory/win screen.
    Victory,
    /// Any user-defined custom state.
    Custom(String),
}

impl FlowState {
    /// Return a static string name for this state (used for JSON / display).
    pub fn name(&self) -> &str {
        match self {
            FlowState::Title => "Title",
            FlowState::Playing => "Playing",
            FlowState::Paused => "Paused",
            FlowState::GameOver => "GameOver",
            FlowState::Victory => "Victory",
            FlowState::Custom(s) => s.as_str(),
        }
    }

    /// Returns true when states are the same variant (and same custom tag).
    pub fn matches(&self, other: &FlowState) -> bool {
        self == other
    }
}

// ─── CompareOp ──────────────────────────────────────────────────────────────

/// Numeric comparison operator used in `FlowCondition::StateReaches`.
#[derive(Clone, Debug, PartialEq)]
pub enum CompareOp {
    /// Equal (within 1e-9 tolerance for f64).
    Eq,
    /// Strictly less than.
    Lt,
    /// Less than or equal.
    Le,
    /// Strictly greater than.
    Gt,
    /// Greater than or equal.
    Ge,
}

impl CompareOp {
    /// Evaluate `lhs <op> rhs`.
    pub fn evaluate(&self, lhs: f64, rhs: f64) -> bool {
        match self {
            CompareOp::Eq => (lhs - rhs).abs() < 1e-9,
            CompareOp::Lt => lhs < rhs,
            CompareOp::Le => lhs <= rhs,
            CompareOp::Gt => lhs > rhs,
            CompareOp::Ge => lhs >= rhs,
        }
    }
}

// ─── FlowCondition ──────────────────────────────────────────────────────────

/// The condition that must be true for a transition to fire.
#[derive(Clone, Debug)]
pub enum FlowCondition {
    /// A named key in `GameState` satisfies a numeric comparison.
    StateReaches { key: String, op: CompareOp, value: f64 },
    /// An event was published on the named EventBus channel this frame.
    EventFired { channel: String },
    /// An input action was activated (checked via `EventBus` channel
    /// `"action:<name>"` so it integrates with `InputMap` event bridging).
    ButtonTapped { action: String },
    /// Automatically transition after this many seconds have elapsed in the
    /// current state.
    AfterSeconds(f64),
}

// ─── FlowTransition ─────────────────────────────────────────────────────────

/// A single edge in the state machine: from → to, triggered by a condition.
#[derive(Clone, Debug)]
pub struct FlowTransition {
    /// The state this transition departs from.
    pub from: FlowState,
    /// The state this transition arrives at.
    pub to: FlowState,
    /// Condition evaluated each frame while `current == from`.
    pub condition: FlowCondition,
    /// Optional sound palette name played when entering `to`.
    pub sound_on_enter: Option<String>,
}

// ─── GameFlow ────────────────────────────────────────────────────────────────

/// Top-level game lifecycle state machine.
///
/// Add transitions with `add_transition`, then call `update` every frame.
/// On a transition `update` returns `Some(new_state)` and the internal state
/// is already updated — callers just react to the signal.
#[derive(Clone, Debug)]
pub struct GameFlow {
    /// The current lifecycle state.
    pub current: FlowState,
    /// All registered transitions.
    pub transitions: Vec<FlowTransition>,
    /// Seconds elapsed in the current state (reset on every state change).
    pub state_elapsed: f64,
    /// The state that was active before a `pause()` call.
    pub paused_state: Option<FlowState>,
    /// Sound palette name queued by the most recent transition (drained by
    /// `take_pending_sound()`).
    pending_sound: Option<String>,
}

impl GameFlow {
    /// Create a new `GameFlow` starting in `FlowState::Title`.
    pub fn new() -> Self {
        Self {
            current: FlowState::Title,
            transitions: Vec::new(),
            state_elapsed: 0.0,
            paused_state: None,
            pending_sound: None,
        }
    }

    // ─── Mutation helpers ────────────────────────────────────────────────

    /// Register a transition rule.
    pub fn add_transition(&mut self, t: FlowTransition) {
        self.transitions.push(t);
    }

    /// Unconditionally jump to a new state, resetting the elapsed timer.
    pub fn set_state(&mut self, state: FlowState) {
        self.current = state;
        self.state_elapsed = 0.0;
    }

    /// Returns `true` when the current state is `Playing`.
    pub fn is_playing(&self) -> bool {
        self.current == FlowState::Playing
    }

    /// Pause the game: saves the current state and moves to `Paused`.
    ///
    /// Calling `pause` while already `Paused` is a no-op.
    pub fn pause(&mut self) {
        if self.current == FlowState::Paused {
            return;
        }
        self.paused_state = Some(self.current.clone());
        self.set_state(FlowState::Paused);
    }

    /// Unpause: restores the state saved by the last `pause()` call.
    ///
    /// Calling `unpause` when not paused is a no-op.
    pub fn unpause(&mut self) {
        if self.current != FlowState::Paused {
            return;
        }
        if let Some(prev) = self.paused_state.take() {
            self.set_state(prev);
        }
    }

    // ─── Query helpers ──────────────────────────────────────────────────

    /// Seconds spent in the current state.
    pub fn elapsed(&self) -> f64 {
        self.state_elapsed
    }

    /// The string name of the current state (cheap, no allocation for built-ins).
    pub fn state_name(&self) -> &str {
        self.current.name()
    }

    // ─── Frame update ───────────────────────────────────────────────────

    /// Advance the state machine by `dt` seconds.
    ///
    /// Evaluates all transitions whose `from` matches the current state.
    /// The **first** matching transition fires: the state changes and
    /// `Some(new_state)` is returned so the caller can react (e.g. play a
    /// sound, trigger a UI change).  Returns `None` when no transition fires.
    ///
    /// Note: While `Paused`, `state_elapsed` does **not** advance and
    /// condition evaluation is skipped, because gameplay conditions should
    /// not fire during a pause.
    pub fn update(&mut self, dt: f64, state: &GameState, bus: &EventBus) -> Option<FlowState> {
        // Do not advance timers or evaluate transitions while paused.
        if self.current == FlowState::Paused {
            return None;
        }

        self.state_elapsed += dt;

        // Snapshot the current state to avoid mutation while iterating.
        let current_snapshot = self.current.clone();

        // Find the first matching transition.
        let mut matched_to: Option<FlowState> = None;
        let mut matched_sound: Option<String> = None;

        for transition in &self.transitions {
            if !transition.from.matches(&current_snapshot) {
                continue;
            }

            let fires = Self::evaluate_condition(
                &transition.condition,
                self.state_elapsed,
                state,
                bus,
            );

            if fires {
                matched_to = Some(transition.to.clone());
                matched_sound = transition.sound_on_enter.clone();
                break;
            }
        }

        if let Some(new_state) = matched_to {
            // Store any queued sound so the caller can retrieve it via
            // `take_pending_sound()` after this call.
            self.pending_sound = matched_sound;
            self.set_state(new_state.clone());
            return Some(new_state);
        }

        None
    }

    /// Evaluate a single condition given the current snapshot.
    fn evaluate_condition(
        condition: &FlowCondition,
        elapsed: f64,
        state: &GameState,
        bus: &EventBus,
    ) -> bool {
        match condition {
            FlowCondition::StateReaches { key, op, value } => {
                if let Some(v) = state.get_f64(key) {
                    op.evaluate(v, *value)
                } else {
                    false
                }
            }
            FlowCondition::EventFired { channel } => bus.has(channel),
            FlowCondition::ButtonTapped { action } => {
                // ButtonTapped checks for a bus event on "action:<name>" so it
                // integrates naturally with the InputMap → EventBus bridge.
                let channel = format!("action:{}", action);
                bus.has(&channel)
            }
            FlowCondition::AfterSeconds(secs) => elapsed >= *secs,
        }
    }

    /// Drain any sound name queued by the last `update()` call.
    ///
    /// Returns `None` after being called once (consumed).
    pub fn take_pending_sound(&mut self) -> Option<String> {
        self.pending_sound.take()
    }

    /// Number of registered transitions.
    pub fn transition_count(&self) -> usize {
        self.transitions.len()
    }

    /// Remove all transitions, resetting to a clean slate (state is kept).
    pub fn clear_transitions(&mut self) {
        self.transitions.clear();
    }

    /// Full reset: back to `Title`, all timers and paused-state cleared.
    pub fn clear(&mut self) {
        self.current = FlowState::Title;
        self.transitions.clear();
        self.state_elapsed = 0.0;
        self.paused_state = None;
        self.pending_sound = None;
    }

    /// Serialize the current state to a JSON object string.
    ///
    /// No serde dependency — built manually per engine conventions.
    pub fn to_json(&self) -> String {
        let paused_str = match &self.paused_state {
            Some(s) => format!("\"{}\"", s.name()),
            None => "null".to_string(),
        };
        format!(
            "{{\"current\":\"{}\",\"state_elapsed\":{},\"paused_state\":{},\"transitions\":{}}}",
            self.current.name(),
            self.state_elapsed,
            paused_str,
            self.transitions.len(),
        )
    }
}

impl Default for GameFlow {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::GameState;
    use crate::event_bus::EventBus;

    // ── Helpers ──────────────────────────────────────────────────────────

    fn empty_state() -> GameState {
        GameState::new()
    }

    fn empty_bus() -> EventBus {
        EventBus::new()
    }

    fn state_with(key: &str, val: f64) -> GameState {
        let mut gs = GameState::new();
        gs.set_f64(key, val);
        gs
    }

    fn bus_with_event(channel: &str) -> EventBus {
        let mut bus = EventBus::new();
        bus.emit(channel);
        bus
    }

    // ── new() / defaults ─────────────────────────────────────────────────

    #[test]
    fn new_starts_in_title() {
        let flow = GameFlow::new();
        assert_eq!(flow.current, FlowState::Title);
        assert_eq!(flow.state_elapsed, 0.0);
        assert!(flow.paused_state.is_none());
        assert_eq!(flow.transition_count(), 0);
    }

    #[test]
    fn default_is_title() {
        let flow = GameFlow::default();
        assert_eq!(flow.current, FlowState::Title);
    }

    // ── set_state / elapsed ───────────────────────────────────────────────

    #[test]
    fn set_state_resets_elapsed() {
        let mut flow = GameFlow::new();
        flow.update(0.5, &empty_state(), &empty_bus());
        assert!((flow.elapsed() - 0.5).abs() < 1e-9);
        flow.set_state(FlowState::Playing);
        assert_eq!(flow.elapsed(), 0.0);
    }

    #[test]
    fn elapsed_accumulates_over_frames() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        flow.update(0.1, &empty_state(), &empty_bus());
        flow.update(0.2, &empty_state(), &empty_bus());
        flow.update(0.3, &empty_state(), &empty_bus());
        assert!((flow.elapsed() - 0.6).abs() < 1e-9);
    }

    // ── is_playing ────────────────────────────────────────────────────────

    #[test]
    fn is_playing_true_when_playing() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        assert!(flow.is_playing());
    }

    #[test]
    fn is_playing_false_when_paused() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        flow.pause();
        assert!(!flow.is_playing());
    }

    // ── state_name ────────────────────────────────────────────────────────

    #[test]
    fn state_name_returns_correct_strings() {
        let mut flow = GameFlow::new();
        assert_eq!(flow.state_name(), "Title");
        flow.set_state(FlowState::Playing);
        assert_eq!(flow.state_name(), "Playing");
        flow.set_state(FlowState::Paused);
        assert_eq!(flow.state_name(), "Paused");
        flow.set_state(FlowState::GameOver);
        assert_eq!(flow.state_name(), "GameOver");
        flow.set_state(FlowState::Victory);
        assert_eq!(flow.state_name(), "Victory");
        flow.set_state(FlowState::Custom("BossIntro".into()));
        assert_eq!(flow.state_name(), "BossIntro");
    }

    // ── pause / unpause ───────────────────────────────────────────────────

    #[test]
    fn pause_moves_to_paused_state() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        flow.pause();
        assert_eq!(flow.current, FlowState::Paused);
        assert_eq!(flow.paused_state, Some(FlowState::Playing));
    }

    #[test]
    fn unpause_restores_previous_state() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        flow.pause();
        flow.unpause();
        assert_eq!(flow.current, FlowState::Playing);
        assert!(flow.paused_state.is_none());
    }

    #[test]
    fn pause_while_already_paused_is_noop() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        flow.pause();
        flow.pause(); // second call — must not clobber paused_state
        assert_eq!(flow.paused_state, Some(FlowState::Playing));
    }

    #[test]
    fn unpause_when_not_paused_is_noop() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        flow.unpause(); // should not crash or change state
        assert_eq!(flow.current, FlowState::Playing);
    }

    #[test]
    fn elapsed_does_not_advance_while_paused() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        flow.update(0.5, &empty_state(), &empty_bus());
        flow.pause();
        let elapsed_before = flow.elapsed();
        flow.update(1.0, &empty_state(), &empty_bus()); // paused — should not count
        flow.update(1.0, &empty_state(), &empty_bus());
        assert!((flow.elapsed() - elapsed_before).abs() < 1e-9);
    }

    // ── CompareOp evaluation ──────────────────────────────────────────────

    #[test]
    fn compare_op_eq() {
        assert!(CompareOp::Eq.evaluate(5.0, 5.0));
        assert!(!CompareOp::Eq.evaluate(5.0, 5.1));
        // Tolerance
        assert!(CompareOp::Eq.evaluate(5.0, 5.0 + 1e-10));
    }

    #[test]
    fn compare_op_lt() {
        assert!(CompareOp::Lt.evaluate(3.0, 5.0));
        assert!(!CompareOp::Lt.evaluate(5.0, 5.0));
        assert!(!CompareOp::Lt.evaluate(6.0, 5.0));
    }

    #[test]
    fn compare_op_le() {
        assert!(CompareOp::Le.evaluate(4.0, 5.0));
        assert!(CompareOp::Le.evaluate(5.0, 5.0));
        assert!(!CompareOp::Le.evaluate(6.0, 5.0));
    }

    #[test]
    fn compare_op_gt() {
        assert!(CompareOp::Gt.evaluate(6.0, 5.0));
        assert!(!CompareOp::Gt.evaluate(5.0, 5.0));
        assert!(!CompareOp::Gt.evaluate(4.0, 5.0));
    }

    #[test]
    fn compare_op_ge() {
        assert!(CompareOp::Ge.evaluate(5.0, 5.0));
        assert!(CompareOp::Ge.evaluate(6.0, 5.0));
        assert!(!CompareOp::Ge.evaluate(4.0, 5.0));
    }

    // ── StateReaches condition ────────────────────────────────────────────

    #[test]
    fn state_reaches_fires_when_condition_met() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        flow.add_transition(FlowTransition {
            from: FlowState::Playing,
            to: FlowState::GameOver,
            condition: FlowCondition::StateReaches {
                key: "health".into(),
                op: CompareOp::Le,
                value: 0.0,
            },
            sound_on_enter: None,
        });

        let gs = state_with("health", 0.0);
        let result = flow.update(0.016, &gs, &empty_bus());
        assert_eq!(result, Some(FlowState::GameOver));
        assert_eq!(flow.current, FlowState::GameOver);
    }

    #[test]
    fn state_reaches_does_not_fire_when_not_met() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        flow.add_transition(FlowTransition {
            from: FlowState::Playing,
            to: FlowState::GameOver,
            condition: FlowCondition::StateReaches {
                key: "health".into(),
                op: CompareOp::Le,
                value: 0.0,
            },
            sound_on_enter: None,
        });

        let gs = state_with("health", 50.0);
        let result = flow.update(0.016, &gs, &empty_bus());
        assert_eq!(result, None);
        assert_eq!(flow.current, FlowState::Playing);
    }

    #[test]
    fn state_reaches_missing_key_does_not_fire() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        flow.add_transition(FlowTransition {
            from: FlowState::Playing,
            to: FlowState::GameOver,
            condition: FlowCondition::StateReaches {
                key: "nonexistent".into(),
                op: CompareOp::Eq,
                value: 0.0,
            },
            sound_on_enter: None,
        });

        let result = flow.update(0.016, &empty_state(), &empty_bus());
        assert_eq!(result, None);
    }

    // ── EventFired condition ──────────────────────────────────────────────

    #[test]
    fn event_fired_condition_fires_on_matching_channel() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        flow.add_transition(FlowTransition {
            from: FlowState::Playing,
            to: FlowState::Victory,
            condition: FlowCondition::EventFired { channel: "player_won".into() },
            sound_on_enter: None,
        });

        let bus = bus_with_event("player_won");
        let result = flow.update(0.016, &empty_state(), &bus);
        assert_eq!(result, Some(FlowState::Victory));
    }

    #[test]
    fn event_fired_condition_does_not_fire_on_wrong_channel() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        flow.add_transition(FlowTransition {
            from: FlowState::Playing,
            to: FlowState::Victory,
            condition: FlowCondition::EventFired { channel: "player_won".into() },
            sound_on_enter: None,
        });

        let bus = bus_with_event("player_died");
        let result = flow.update(0.016, &empty_state(), &bus);
        assert_eq!(result, None);
    }

    // ── ButtonTapped condition ────────────────────────────────────────────

    #[test]
    fn button_tapped_fires_when_action_event_present() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Title);
        flow.add_transition(FlowTransition {
            from: FlowState::Title,
            to: FlowState::Playing,
            condition: FlowCondition::ButtonTapped { action: "start".into() },
            sound_on_enter: None,
        });

        let bus = bus_with_event("action:start");
        let result = flow.update(0.016, &empty_state(), &bus);
        assert_eq!(result, Some(FlowState::Playing));
    }

    #[test]
    fn button_tapped_does_not_fire_without_event() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Title);
        flow.add_transition(FlowTransition {
            from: FlowState::Title,
            to: FlowState::Playing,
            condition: FlowCondition::ButtonTapped { action: "start".into() },
            sound_on_enter: None,
        });

        let result = flow.update(0.016, &empty_state(), &empty_bus());
        assert_eq!(result, None);
    }

    // ── AfterSeconds condition ────────────────────────────────────────────

    #[test]
    fn after_seconds_fires_once_elapsed() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::GameOver);
        flow.add_transition(FlowTransition {
            from: FlowState::GameOver,
            to: FlowState::Title,
            condition: FlowCondition::AfterSeconds(3.0),
            sound_on_enter: None,
        });

        // Not yet
        assert_eq!(flow.update(1.0, &empty_state(), &empty_bus()), None);
        assert_eq!(flow.update(1.0, &empty_state(), &empty_bus()), None);
        // Third frame pushes elapsed to 3.0
        let result = flow.update(1.0, &empty_state(), &empty_bus());
        assert_eq!(result, Some(FlowState::Title));
    }

    #[test]
    fn after_seconds_resets_on_state_change() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::GameOver);
        flow.add_transition(FlowTransition {
            from: FlowState::GameOver,
            to: FlowState::Title,
            condition: FlowCondition::AfterSeconds(2.0),
            sound_on_enter: None,
        });

        flow.update(1.5, &empty_state(), &empty_bus());
        // Manually change state before condition fires
        flow.set_state(FlowState::Playing);
        assert_eq!(flow.elapsed(), 0.0, "Elapsed must reset after set_state");
    }

    // ── Transition only fires from the correct from-state ─────────────────

    #[test]
    fn transition_does_not_fire_from_wrong_state() {
        let mut flow = GameFlow::new();
        // Transition is from Playing → GameOver, but we start in Title
        flow.add_transition(FlowTransition {
            from: FlowState::Playing,
            to: FlowState::GameOver,
            condition: FlowCondition::StateReaches {
                key: "health".into(),
                op: CompareOp::Le,
                value: 0.0,
            },
            sound_on_enter: None,
        });

        let gs = state_with("health", 0.0);
        let result = flow.update(0.016, &gs, &empty_bus());
        assert_eq!(result, None);
        assert_eq!(flow.current, FlowState::Title);
    }

    // ── sound_on_enter / pending_sound ────────────────────────────────────

    #[test]
    fn sound_on_enter_is_queued_on_transition() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        flow.add_transition(FlowTransition {
            from: FlowState::Playing,
            to: FlowState::GameOver,
            condition: FlowCondition::EventFired { channel: "die".into() },
            sound_on_enter: Some("game_over_sting".into()),
        });

        let bus = bus_with_event("die");
        flow.update(0.016, &empty_state(), &bus);
        assert_eq!(flow.take_pending_sound(), Some("game_over_sting".to_string()));
        // Consumed
        assert_eq!(flow.take_pending_sound(), None);
    }

    // ── clear ─────────────────────────────────────────────────────────────

    #[test]
    fn clear_resets_everything() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        flow.add_transition(FlowTransition {
            from: FlowState::Playing,
            to: FlowState::GameOver,
            condition: FlowCondition::AfterSeconds(1.0),
            sound_on_enter: Some("boom".into()),
        });
        flow.update(0.5, &empty_state(), &empty_bus());
        flow.pause();
        flow.clear();

        assert_eq!(flow.current, FlowState::Title);
        assert_eq!(flow.transition_count(), 0);
        assert_eq!(flow.elapsed(), 0.0);
        assert!(flow.paused_state.is_none());
    }

    // ── Custom state ──────────────────────────────────────────────────────

    #[test]
    fn custom_state_transition() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Custom("Cutscene".into()));
        flow.add_transition(FlowTransition {
            from: FlowState::Custom("Cutscene".into()),
            to: FlowState::Playing,
            condition: FlowCondition::AfterSeconds(5.0),
            sound_on_enter: None,
        });

        assert_eq!(flow.update(4.9, &empty_state(), &empty_bus()), None);
        let result = flow.update(0.2, &empty_state(), &empty_bus());
        assert_eq!(result, Some(FlowState::Playing));
    }

    // ── to_json ───────────────────────────────────────────────────────────

    #[test]
    fn to_json_contains_state_name() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        let json = flow.to_json();
        assert!(json.contains("\"current\":\"Playing\""), "JSON: {}", json);
    }

    #[test]
    fn to_json_paused_state_is_null_when_not_paused() {
        let flow = GameFlow::new();
        let json = flow.to_json();
        assert!(json.contains("\"paused_state\":null"), "JSON: {}", json);
    }

    #[test]
    fn to_json_paused_state_shows_name_when_paused() {
        let mut flow = GameFlow::new();
        flow.set_state(FlowState::Playing);
        flow.pause();
        let json = flow.to_json();
        assert!(json.contains("\"paused_state\":\"Playing\""), "JSON: {}", json);
    }
}
