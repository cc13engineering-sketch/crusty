use crate::engine::Engine;
use super::scenario::ScheduledAction;
use super::replay::{Replay, record_replay};
use super::compare::{compare_replays, Comparison};

/// Result of a golden file comparison.
#[derive(Clone, Debug)]
pub struct GoldenResult {
    /// Whether the current run matches the golden reference.
    pub matches: bool,
    /// The comparison details.
    pub comparison: Comparison,
    /// Summary message.
    pub detail: String,
}

impl GoldenResult {
    pub fn summary(&self) -> String {
        let verdict = if self.matches { "MATCH" } else { "MISMATCH" };
        format!("Golden test: {} — {}", verdict, self.detail)
    }
}

/// A golden reference: a recorded replay used for comparison.
///
/// Workflow:
/// 1. Record a golden replay (once, when behavior is correct)
/// 2. After changes, run the same scenario and compare against golden
/// 3. If they match (within tolerance), the change is safe
///
/// This is the headless equivalent of visual regression testing.
pub struct GoldenTest {
    setup_fn: fn(&mut Engine),
    update_fn: fn(&mut Engine, f64),
    render_fn: fn(&mut Engine),
    action_dispatch: fn(&mut Engine, &ScheduledAction),
    actions: Vec<ScheduledAction>,
    frames: u64,
    keys: Vec<String>,
    tolerance: f64,
}

impl GoldenTest {
    pub fn new(
        setup_fn: fn(&mut Engine),
        update_fn: fn(&mut Engine, f64),
        render_fn: fn(&mut Engine),
        action_dispatch: fn(&mut Engine, &ScheduledAction),
    ) -> Self {
        Self {
            setup_fn,
            update_fn,
            render_fn,
            action_dispatch,
            actions: Vec::new(),
            frames: 60,
            keys: Vec::new(),
            tolerance: 0.01,
        }
    }

    pub fn with_actions(mut self, actions: Vec<ScheduledAction>) -> Self {
        self.actions = actions;
        self
    }

    pub fn with_frames(mut self, frames: u64) -> Self {
        self.frames = frames;
        self
    }

    pub fn with_keys(mut self, keys: &[&str]) -> Self {
        self.keys = keys.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Record the golden reference replay.
    pub fn record_golden(&self, name: &str) -> Replay {
        let keys_ref: Vec<&str> = self.keys.iter().map(|s| s.as_str()).collect();
        record_replay(
            name,
            self.setup_fn,
            self.update_fn,
            self.render_fn,
            self.action_dispatch,
            &self.actions,
            self.frames,
            &keys_ref,
        )
    }

    /// Compare a current run against a golden reference.
    pub fn compare_against(&self, golden: &Replay) -> GoldenResult {
        let keys_ref: Vec<&str> = self.keys.iter().map(|s| s.as_str()).collect();
        let current = record_replay(
            "current",
            self.setup_fn,
            self.update_fn,
            self.render_fn,
            self.action_dispatch,
            &self.actions,
            self.frames,
            &keys_ref,
        );

        let comparison = compare_replays(golden, &current, &keys_ref, self.tolerance);
        let matches = comparison.is_identical(self.tolerance);

        let detail = if matches {
            format!("all {} keys match within tolerance {}", self.keys.len(), self.tolerance)
        } else {
            let diverged: Vec<String> = comparison.diffs.iter()
                .filter(|d| !d.is_identical(self.tolerance))
                .map(|d| format!("{} (max_delta={:.4})", d.key, d.max_delta))
                .collect();
            format!("diverged: {}", diverged.join(", "))
        };

        GoldenResult {
            matches,
            comparison,
            detail,
        }
    }
}
