/// Behavior Rules: Declarative condition -> action rules evaluated each frame.
///
/// Rules are the bridge between events/state and runtime actions. They let
/// .world files define game logic without custom Rust code.
///
/// Example rule: "when an entity tagged 'bullet' collides with an entity
/// tagged 'asteroid', despawn both and add 10 to score."

use crate::ecs::Entity;

/// A condition that must be met for a rule to fire.
#[derive(Clone, Debug)]
pub enum Condition {
    /// Fires when two entities with the given tags collide.
    /// (tag_a, tag_b) — order does not matter.
    Collision { tag_a: String, tag_b: String },

    /// Fires when an entity with `visitor_tag` enters a trigger with `trigger_tag`.
    TriggerEnter { trigger_tag: String, visitor_tag: String },

    /// Fires when a named timer fires.
    TimerFired { timer_name: String },

    /// Fires when a game state key meets a numeric comparison.
    StateCheck { key: String, op: CompareOp, value: f64 },

    /// Fires when a key is pressed this frame.
    KeyPressed { key_code: String },

    /// Always fires (every frame). Use with caution.
    Always,
}

/// Comparison operators for StateCheck conditions.
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
            CompareOp::Eq => (lhs - rhs).abs() < 1e-9,
            CompareOp::Neq => (lhs - rhs).abs() >= 1e-9,
            CompareOp::Lt => lhs < rhs,
            CompareOp::Lte => lhs <= rhs,
            CompareOp::Gt => lhs > rhs,
            CompareOp::Gte => lhs >= rhs,
        }
    }

    /// Parse from a string like "==", "!=", "<", "<=", ">", ">=".
    pub fn from_str_op(s: &str) -> Option<Self> {
        match s {
            "==" | "eq" => Some(CompareOp::Eq),
            "!=" | "neq" => Some(CompareOp::Neq),
            "<" | "lt" => Some(CompareOp::Lt),
            "<=" | "lte" => Some(CompareOp::Lte),
            ">" | "gt" => Some(CompareOp::Gt),
            ">=" | "gte" => Some(CompareOp::Gte),
            _ => None,
        }
    }
}

/// An action to perform when a rule's condition is met.
#[derive(Clone, Debug)]
pub enum Action {
    /// Despawn the entity that matched `entity_ref` in the condition.
    /// EntityRef::A means the first entity, EntityRef::B means the second.
    Despawn { entity_ref: EntityRef },

    /// Add a delta to a game state key.
    AddState { key: String, delta: f64 },

    /// Set a game state key to a value.
    SetState { key: String, value: f64 },

    /// Set a boolean game state flag.
    SetFlag { key: String, value: bool },

    /// Spawn an entity from a named template at an optional position.
    SpawnTemplate { template_name: String, at: SpawnAt },

    /// Start a timer.
    StartTimer { name: String, delay: f64, interval: Option<f64>, max_fires: u64 },

    /// Cancel a timer by name.
    CancelTimer { name: String },

    /// Log a message.
    Log { message: String },
}

/// Reference to which entity in a two-entity condition (collision/trigger).
#[derive(Clone, Debug)]
pub enum EntityRef {
    /// The first entity (entity_a / trigger).
    A,
    /// The second entity (entity_b / visitor).
    B,
    /// Both entities.
    Both,
}

/// Where to spawn an entity from a template.
#[derive(Clone, Debug)]
pub enum SpawnAt {
    /// Spawn at the position of entity A from the triggering condition.
    EntityA,
    /// Spawn at the position of entity B from the triggering condition.
    EntityB,
    /// Spawn at a fixed world position.
    Position(f64, f64),
    /// Spawn at a random position within bounds.
    Random,
}

/// A complete rule: one condition and a list of actions.
#[derive(Clone, Debug)]
pub struct BehaviorRule {
    /// Human-readable name for debugging.
    pub name: String,
    /// The condition that triggers this rule.
    pub condition: Condition,
    /// Actions to execute when the condition is met.
    pub actions: Vec<Action>,
    /// Whether this rule is active.
    pub enabled: bool,
    /// If true, this rule fires only once and then disables itself.
    pub once: bool,
}

impl BehaviorRule {
    pub fn new(name: &str, condition: Condition, actions: Vec<Action>) -> Self {
        Self {
            name: name.to_string(),
            condition,
            actions,
            enabled: true,
            once: false,
        }
    }

    pub fn one_shot(name: &str, condition: Condition, actions: Vec<Action>) -> Self {
        Self {
            name: name.to_string(),
            condition,
            actions,
            enabled: true,
            once: true,
        }
    }
}

/// Accumulated action commands produced by rule evaluation.
/// Collected during evaluation, then executed against the world/state/timers.
#[derive(Default, Debug)]
pub struct ActionQueue {
    pub despawns: Vec<Entity>,
    pub state_adds: Vec<(String, f64)>,
    pub state_sets: Vec<(String, f64)>,
    pub flag_sets: Vec<(String, bool)>,
    pub spawns: Vec<(String, f64, f64)>, // (template_name, x, y)
    pub timer_starts: Vec<(String, f64, Option<f64>, u64)>, // (name, delay, interval, max_fires)
    pub timer_cancels: Vec<String>,
    pub logs: Vec<String>,
}

impl ActionQueue {
    pub fn new() -> Self { Self::default() }

    pub fn is_empty(&self) -> bool {
        self.despawns.is_empty()
            && self.state_adds.is_empty()
            && self.state_sets.is_empty()
            && self.flag_sets.is_empty()
            && self.spawns.is_empty()
            && self.timer_starts.is_empty()
            && self.timer_cancels.is_empty()
            && self.logs.is_empty()
    }

    pub fn clear(&mut self) {
        self.despawns.clear();
        self.state_adds.clear();
        self.state_sets.clear();
        self.flag_sets.clear();
        self.spawns.clear();
        self.timer_starts.clear();
        self.timer_cancels.clear();
        self.logs.clear();
    }
}

/// The collection of all behavior rules for the current world.
#[derive(Default, Clone, Debug)]
pub struct BehaviorRules {
    pub rules: Vec<BehaviorRule>,
}

impl BehaviorRules {
    pub fn new() -> Self { Self { rules: Vec::new() } }

    pub fn add(&mut self, rule: BehaviorRule) {
        self.rules.push(rule);
    }

    pub fn clear(&mut self) {
        self.rules.clear();
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_op_eq() {
        assert!(CompareOp::Eq.evaluate(5.0, 5.0));
        assert!(!CompareOp::Eq.evaluate(5.0, 6.0));
    }

    #[test]
    fn compare_op_neq() {
        assert!(CompareOp::Neq.evaluate(5.0, 6.0));
        assert!(!CompareOp::Neq.evaluate(5.0, 5.0));
    }

    #[test]
    fn compare_op_lt() {
        assert!(CompareOp::Lt.evaluate(3.0, 5.0));
        assert!(!CompareOp::Lt.evaluate(5.0, 5.0));
    }

    #[test]
    fn compare_op_lte() {
        assert!(CompareOp::Lte.evaluate(5.0, 5.0));
        assert!(CompareOp::Lte.evaluate(3.0, 5.0));
        assert!(!CompareOp::Lte.evaluate(6.0, 5.0));
    }

    #[test]
    fn compare_op_gt() {
        assert!(CompareOp::Gt.evaluate(6.0, 5.0));
        assert!(!CompareOp::Gt.evaluate(5.0, 5.0));
    }

    #[test]
    fn compare_op_gte() {
        assert!(CompareOp::Gte.evaluate(5.0, 5.0));
        assert!(CompareOp::Gte.evaluate(6.0, 5.0));
        assert!(!CompareOp::Gte.evaluate(4.0, 5.0));
    }

    #[test]
    fn compare_op_from_str() {
        assert!(CompareOp::from_str_op("==").is_some());
        assert!(CompareOp::from_str_op("!=").is_some());
        assert!(CompareOp::from_str_op("<").is_some());
        assert!(CompareOp::from_str_op("<=").is_some());
        assert!(CompareOp::from_str_op(">").is_some());
        assert!(CompareOp::from_str_op(">=").is_some());
        assert!(CompareOp::from_str_op("eq").is_some());
        assert!(CompareOp::from_str_op("nope").is_none());
    }

    #[test]
    fn behavior_rule_new() {
        let rule = BehaviorRule::new(
            "test",
            Condition::Always,
            vec![Action::Log { message: "hello".into() }],
        );
        assert_eq!(rule.name, "test");
        assert!(rule.enabled);
        assert!(!rule.once);
    }

    #[test]
    fn behavior_rule_one_shot() {
        let rule = BehaviorRule::one_shot(
            "test",
            Condition::Always,
            vec![Action::Log { message: "hello".into() }],
        );
        assert!(rule.once);
    }

    #[test]
    fn action_queue_starts_empty() {
        let aq = ActionQueue::new();
        assert!(aq.is_empty());
    }

    #[test]
    fn action_queue_not_empty_after_push() {
        let mut aq = ActionQueue::new();
        aq.despawns.push(Entity(1));
        assert!(!aq.is_empty());
    }

    #[test]
    fn action_queue_clear() {
        let mut aq = ActionQueue::new();
        aq.despawns.push(Entity(1));
        aq.logs.push("test".into());
        aq.clear();
        assert!(aq.is_empty());
    }

    #[test]
    fn behavior_rules_collection() {
        let mut rules = BehaviorRules::new();
        assert!(rules.is_empty());
        rules.add(BehaviorRule::new("r1", Condition::Always, vec![]));
        assert_eq!(rules.len(), 1);
        rules.clear();
        assert!(rules.is_empty());
    }
}
