use std::collections::VecDeque;
use crate::components::SchemaInfo;
use crate::components::state_machine::CompareOp;

/// A single step in a coroutine sequence.
#[derive(Clone, Debug)]
pub enum CoroutineStep {
    /// Wait for N seconds.
    WaitSeconds(f64),
    /// Wait until a signal channel is active.
    WaitSignal(String),
    /// Wait until a game state condition is met.
    WaitUntil { key: String, op: CompareOp, value: f64 },
    /// Set a game state value on the entity.
    SetState { key: String, value: f64 },
    /// Add to a game state value on the entity.
    AddState { key: String, delta: f64 },
    /// Spawn a template at position.
    SpawnTemplate { name: String, x: f64, y: f64 },
    /// Log a message.
    Log(String),
}

/// A coroutine is a sequence of steps executed over time.
/// Wait steps block until their condition is met.
/// Non-wait steps execute immediately and cascade in one frame.
#[derive(Clone, Debug)]
pub struct Coroutine {
    pub steps: VecDeque<CoroutineStep>,
    pub wait_timer: f64,
    pub paused: bool,
    pub label: String,
}

impl Coroutine {
    pub fn new(label: &str) -> Self {
        Self {
            steps: VecDeque::new(),
            wait_timer: 0.0,
            paused: false,
            label: label.to_string(),
        }
    }

    pub fn then_wait(mut self, seconds: f64) -> Self {
        self.steps.push_back(CoroutineStep::WaitSeconds(seconds));
        self
    }

    pub fn then_wait_signal(mut self, channel: &str) -> Self {
        self.steps.push_back(CoroutineStep::WaitSignal(channel.to_string()));
        self
    }

    pub fn then_wait_until(mut self, key: &str, op: CompareOp, value: f64) -> Self {
        self.steps.push_back(CoroutineStep::WaitUntil {
            key: key.to_string(), op, value
        });
        self
    }

    pub fn then_set_state(mut self, key: &str, value: f64) -> Self {
        self.steps.push_back(CoroutineStep::SetState {
            key: key.to_string(), value
        });
        self
    }

    pub fn then_add_state(mut self, key: &str, delta: f64) -> Self {
        self.steps.push_back(CoroutineStep::AddState {
            key: key.to_string(), delta
        });
        self
    }

    pub fn then_spawn(mut self, template: &str, x: f64, y: f64) -> Self {
        self.steps.push_back(CoroutineStep::SpawnTemplate {
            name: template.to_string(), x, y,
        });
        self
    }

    pub fn then_log(mut self, msg: &str) -> Self {
        self.steps.push_back(CoroutineStep::Log(msg.to_string()));
        self
    }

    pub fn is_done(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn remaining_steps(&self) -> usize {
        self.steps.len()
    }
}

impl SchemaInfo for Coroutine {
    fn schema_name() -> &'static str { "Coroutine" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "description": "Sequential async behavior: wait, set state, spawn, log",
            "fields": {
                "steps": "VecDeque<CoroutineStep>",
                "wait_timer": "f64",
                "paused": "bool",
                "label": "String"
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_pattern() {
        let co = Coroutine::new("test")
            .then_wait(1.0)
            .then_set_state("score", 10.0)
            .then_log("hello")
            .then_spawn("bullet", 5.0, 10.0)
            .then_wait_signal("door_open")
            .then_add_state("count", 1.0);
        assert_eq!(co.remaining_steps(), 6);
        assert_eq!(co.label, "test");
        assert!(!co.is_done());
    }

    #[test]
    fn empty_coroutine_is_done() {
        let co = Coroutine::new("empty");
        assert!(co.is_done());
        assert_eq!(co.remaining_steps(), 0);
    }

    #[test]
    fn new_not_paused() {
        let co = Coroutine::new("test");
        assert!(!co.paused);
    }

    #[test]
    fn wait_timer_starts_zero() {
        let co = Coroutine::new("test").then_wait(5.0);
        assert_eq!(co.wait_timer, 0.0);
    }

    #[test]
    fn then_wait_until_creates_step() {
        let co = Coroutine::new("test")
            .then_wait_until("health", CompareOp::Lte, 0.0);
        assert_eq!(co.remaining_steps(), 1);
    }

    #[test]
    fn clone_independence() {
        let co = Coroutine::new("test").then_wait(1.0).then_log("msg");
        let mut co2 = co.clone();
        co2.steps.pop_front();
        assert_eq!(co.remaining_steps(), 2);
        assert_eq!(co2.remaining_steps(), 1);
    }

    #[test]
    fn schema_info() {
        assert_eq!(Coroutine::schema_name(), "Coroutine");
    }
}
