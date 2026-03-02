use crate::engine::Engine;
use super::scenario::ScheduledAction;
use super::replay::{Replay, record_replay};
use super::compare::compare_replays;
use super::anomaly::AnomalyDetector;

/// A step in a multi-step strategy.
pub enum StrategyStep {
    /// Record a replay with given actions and tracked keys.
    Record {
        name: String,
        actions: Vec<ScheduledAction>,
        frames: u64,
        keys: Vec<String>,
    },
    /// Compare two named replays.
    Compare {
        replay_a: String,
        replay_b: String,
        keys: Vec<String>,
        tolerance: f64,
    },
    /// Run anomaly detection on a named replay.
    DetectAnomalies {
        replay_name: String,
        keys: Vec<String>,
        spike_threshold: f64,
        plateau_min_frames: usize,
    },
    /// Assert a condition on a named replay's final state.
    AssertState {
        replay_name: String,
        key: String,
        predicate: StatePredicate,
    },
}

/// A predicate for asserting state values.
#[derive(Clone, Debug)]
pub enum StatePredicate {
    GreaterThan(f64),
    LessThan(f64),
    Equals(f64, f64), // (expected, tolerance)
    InRange(f64, f64),
}

impl StatePredicate {
    fn check(&self, value: f64) -> bool {
        match self {
            Self::GreaterThan(t) => value > *t,
            Self::LessThan(t) => value < *t,
            Self::Equals(expected, tol) => (value - expected).abs() <= *tol,
            Self::InRange(min, max) => value >= *min && value <= *max,
        }
    }

    fn describe(&self) -> String {
        match self {
            Self::GreaterThan(t) => format!("> {}", t),
            Self::LessThan(t) => format!("< {}", t),
            Self::Equals(e, t) => format!("== {} (±{})", e, t),
            Self::InRange(a, b) => format!("in [{}, {}]", a, b),
        }
    }
}

/// Outcome of a single strategy step.
#[derive(Clone, Debug)]
pub struct StepOutcome {
    pub step_name: String,
    pub passed: bool,
    pub detail: String,
}

/// Result of running a complete strategy.
#[derive(Clone, Debug)]
pub struct StrategyResult {
    pub name: String,
    pub outcomes: Vec<StepOutcome>,
}

impl StrategyResult {
    pub fn all_passed(&self) -> bool {
        self.outcomes.iter().all(|o| o.passed)
    }

    pub fn failures(&self) -> Vec<&StepOutcome> {
        self.outcomes.iter().filter(|o| !o.passed).collect()
    }

    pub fn summary(&self) -> String {
        let passed = self.outcomes.iter().filter(|o| o.passed).count();
        let total = self.outcomes.len();
        let verdict = if self.all_passed() { "PASS" } else { "FAIL" };
        let mut out = format!("Strategy '{}': {} ({}/{})\n", self.name, verdict, passed, total);
        for o in &self.outcomes {
            let status = if o.passed { "OK" } else { "FAIL" };
            out.push_str(&format!("  [{}] {} — {}\n", status, o.step_name, o.detail));
        }
        out
    }
}

/// A multi-step strategy that chains headless operations.
///
/// Strategies let Claude express high-level intent: "record a baseline,
/// record with changes, compare them, check for anomalies, assert state."
pub struct Strategy {
    pub name: String,
    setup_fn: fn(&mut Engine),
    update_fn: fn(&mut Engine, f64),
    render_fn: fn(&mut Engine),
    action_dispatch: fn(&mut Engine, &ScheduledAction),
    steps: Vec<StrategyStep>,
}

impl Strategy {
    pub fn new(
        name: &str,
        setup_fn: fn(&mut Engine),
        update_fn: fn(&mut Engine, f64),
        render_fn: fn(&mut Engine),
        action_dispatch: fn(&mut Engine, &ScheduledAction),
    ) -> Self {
        Self {
            name: name.to_string(),
            setup_fn,
            update_fn,
            render_fn,
            action_dispatch,
            steps: Vec::new(),
        }
    }

    pub fn record(mut self, name: &str, actions: Vec<ScheduledAction>, frames: u64, keys: &[&str]) -> Self {
        self.steps.push(StrategyStep::Record {
            name: name.to_string(),
            actions,
            frames,
            keys: keys.iter().map(|s| s.to_string()).collect(),
        });
        self
    }

    pub fn compare(mut self, a: &str, b: &str, keys: &[&str], tolerance: f64) -> Self {
        self.steps.push(StrategyStep::Compare {
            replay_a: a.to_string(),
            replay_b: b.to_string(),
            keys: keys.iter().map(|s| s.to_string()).collect(),
            tolerance,
        });
        self
    }

    pub fn detect_anomalies(mut self, replay: &str, keys: &[&str], spike_threshold: f64, plateau_min: usize) -> Self {
        self.steps.push(StrategyStep::DetectAnomalies {
            replay_name: replay.to_string(),
            keys: keys.iter().map(|s| s.to_string()).collect(),
            spike_threshold,
            plateau_min_frames: plateau_min,
        });
        self
    }

    pub fn assert_state(mut self, replay: &str, key: &str, predicate: StatePredicate) -> Self {
        self.steps.push(StrategyStep::AssertState {
            replay_name: replay.to_string(),
            key: key.to_string(),
            predicate,
        });
        self
    }

    /// Execute all steps in order.
    pub fn run(self) -> StrategyResult {
        let mut replays: Vec<Replay> = Vec::new();
        let mut outcomes = Vec::new();

        for step in &self.steps {
            match step {
                StrategyStep::Record { name, actions, frames, keys } => {
                    let keys_ref: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
                    let replay = record_replay(
                        name,
                        self.setup_fn,
                        self.update_fn,
                        self.render_fn,
                        self.action_dispatch,
                        actions,
                        *frames,
                        &keys_ref,
                    );
                    outcomes.push(StepOutcome {
                        step_name: format!("Record '{}'", name),
                        passed: true,
                        detail: format!("{} frames, {} keys", replay.len(), keys.len()),
                    });
                    replays.push(replay);
                }

                StrategyStep::Compare { replay_a, replay_b, keys, tolerance } => {
                    let a = replays.iter().find(|r| &r.name == replay_a);
                    let b = replays.iter().find(|r| &r.name == replay_b);
                    match (a, b) {
                        (Some(a), Some(b)) => {
                            let keys_ref: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
                            let cmp = compare_replays(a, b, &keys_ref, *tolerance);
                            let identical = cmp.is_identical(*tolerance);
                            outcomes.push(StepOutcome {
                                step_name: format!("Compare '{}' vs '{}'", replay_a, replay_b),
                                passed: !identical, // we expect differences when comparing different runs
                                detail: if identical {
                                    "replays are identical".into()
                                } else {
                                    let divergent: Vec<_> = cmp.diffs.iter()
                                        .filter(|d| !d.is_identical(*tolerance))
                                        .map(|d| format!("{} (max_delta={:.2})", d.key, d.max_delta))
                                        .collect();
                                    format!("differs: {}", divergent.join(", "))
                                },
                            });
                        }
                        _ => {
                            outcomes.push(StepOutcome {
                                step_name: format!("Compare '{}' vs '{}'", replay_a, replay_b),
                                passed: false,
                                detail: "replay not found".into(),
                            });
                        }
                    }
                }

                StrategyStep::DetectAnomalies { replay_name, keys, spike_threshold, plateau_min_frames } => {
                    let replay = replays.iter().find(|r| &r.name == replay_name);
                    match replay {
                        Some(r) => {
                            let keys_ref: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
                            let detector = AnomalyDetector::new()
                                .with_spike_threshold(*spike_threshold)
                                .with_plateau_min_frames(*plateau_min_frames);
                            let anomalies = detector.scan(r, &keys_ref);
                            let spike_count = anomalies.iter().filter(|a| a.kind == super::anomaly::AnomalyKind::Spike).count();
                            outcomes.push(StepOutcome {
                                step_name: format!("Anomalies in '{}'", replay_name),
                                passed: spike_count == 0,
                                detail: format!("{} anomalies ({} spikes)", anomalies.len(), spike_count),
                            });
                        }
                        None => {
                            outcomes.push(StepOutcome {
                                step_name: format!("Anomalies in '{}'", replay_name),
                                passed: false,
                                detail: "replay not found".into(),
                            });
                        }
                    }
                }

                StrategyStep::AssertState { replay_name, key, predicate } => {
                    let replay = replays.iter().find(|r| &r.name == replay_name);
                    match replay {
                        Some(r) => {
                            let last_frame = r.len().saturating_sub(1);
                            let value = r.get(last_frame, key);
                            match value {
                                Some(v) => {
                                    let passed = predicate.check(v);
                                    outcomes.push(StepOutcome {
                                        step_name: format!("Assert '{}' in '{}'", key, replay_name),
                                        passed,
                                        detail: format!("value={:.2}, expected {}", v, predicate.describe()),
                                    });
                                }
                                None => {
                                    outcomes.push(StepOutcome {
                                        step_name: format!("Assert '{}' in '{}'", key, replay_name),
                                        passed: false,
                                        detail: format!("key '{}' not found in replay", key),
                                    });
                                }
                            }
                        }
                        None => {
                            outcomes.push(StepOutcome {
                                step_name: format!("Assert '{}' in '{}'", key, replay_name),
                                passed: false,
                                detail: "replay not found".into(),
                            });
                        }
                    }
                }
            }
        }

        StrategyResult {
            name: self.name,
            outcomes,
        }
    }
}
