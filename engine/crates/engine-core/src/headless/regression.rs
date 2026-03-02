use super::scenario::GameScenario;
use std::collections::HashMap;

/// A captured baseline from a known-good run.
#[derive(Clone, Debug)]
pub struct RegressionBaseline {
    pub name: String,
    pub state_snapshot: HashMap<String, f64>,
    pub fb_hash: u64,
}

/// Classification of a single metric comparison.
#[derive(Clone, Debug, PartialEq)]
pub enum DiffStatus {
    Unchanged,
    Improved { delta: f64, detail: String },
    Regressed { delta: f64, detail: String },
    Changed { detail: String },
}

/// One line item in the diff report.
#[derive(Clone, Debug)]
pub struct DiffEntry {
    pub scenario: String,
    pub metric: String,
    pub status: DiffStatus,
}

/// The full before/after comparison report.
#[derive(Clone, Debug)]
pub struct DiffReport {
    pub entries: Vec<DiffEntry>,
}

impl DiffReport {
    pub fn has_regressions(&self) -> bool {
        self.entries.iter().any(|e| matches!(e.status, DiffStatus::Regressed { .. }))
    }

    pub fn regressions(&self) -> Vec<&DiffEntry> {
        self.entries.iter().filter(|e| matches!(e.status, DiffStatus::Regressed { .. })).collect()
    }

    pub fn improvements(&self) -> Vec<&DiffEntry> {
        self.entries.iter().filter(|e| matches!(e.status, DiffStatus::Improved { .. })).collect()
    }

    pub fn verdict(&self) -> &'static str {
        if self.has_regressions() { "FAIL" } else { "PASS" }
    }

    /// Generate a compact summary for AI consumption.
    pub fn summary(&self) -> String {
        let reg = self.regressions().len();
        let imp = self.improvements().len();
        let mut out = format!("[{}] {} regressions, {} improvements\n", self.verdict(), reg, imp);
        for e in &self.entries {
            match &e.status {
                DiffStatus::Unchanged => {}
                DiffStatus::Improved { detail, .. } => {
                    out.push_str(&format!("  + [{}] {}: {}\n", e.scenario, e.metric, detail));
                }
                DiffStatus::Regressed { detail, .. } => {
                    out.push_str(&format!("  ! [{}] {}: {}\n", e.scenario, e.metric, detail));
                }
                DiffStatus::Changed { detail } => {
                    out.push_str(&format!("  ~ [{}] {}: {}\n", e.scenario, e.metric, detail));
                }
            }
        }
        out
    }
}

/// A regression suite: run scenarios, capture baselines, diff against them.
pub struct RegressionSuite {
    scenarios: Vec<GameScenario>,
    /// Keys to track for f64 comparisons.
    watch_keys: Vec<String>,
    /// Tolerance for f64 comparisons.
    pub tolerance: f64,
}

impl RegressionSuite {
    pub fn new(watch_keys: &[&str]) -> Self {
        Self {
            scenarios: Vec::new(),
            watch_keys: watch_keys.iter().map(|s| s.to_string()).collect(),
            tolerance: 0.5,
        }
    }

    pub fn add(mut self, scenario: GameScenario) -> Self {
        self.scenarios.push(scenario);
        self
    }

    pub fn with_tolerance(mut self, tol: f64) -> Self {
        self.tolerance = tol;
        self
    }

    /// Run all scenarios and capture baselines.
    pub fn capture_baseline(&self) -> Vec<RegressionBaseline> {
        self.scenarios.iter().map(|s| {
            let result = s.run();
            let state_snapshot: HashMap<String, f64> = self.watch_keys.iter().filter_map(|k| {
                result.sim.game_state.get(k)
                    .and_then(|v| v.as_f64())
                    .map(|v| (k.clone(), v))
            }).collect();
            RegressionBaseline {
                name: result.name,
                state_snapshot,
                fb_hash: result.sim.framebuffer_hash,
            }
        }).collect()
    }

    /// Run all scenarios and diff against baselines.
    pub fn diff_against(&self, baselines: &[RegressionBaseline]) -> DiffReport {
        let current = self.capture_baseline();
        let mut entries = Vec::new();

        for (base, curr) in baselines.iter().zip(current.iter()) {
            // Framebuffer hash
            if base.fb_hash != curr.fb_hash {
                entries.push(DiffEntry {
                    scenario: base.name.clone(),
                    metric: "framebuffer".into(),
                    status: DiffStatus::Changed {
                        detail: format!("{:#x} -> {:#x}", base.fb_hash, curr.fb_hash),
                    },
                });
            }

            // State key comparisons
            for key in &self.watch_keys {
                let base_val = base.state_snapshot.get(key).copied();
                let curr_val = curr.state_snapshot.get(key).copied();

                if let (Some(b), Some(c)) = (base_val, curr_val) {
                    let delta = c - b;
                    if delta.abs() > self.tolerance {
                        entries.push(DiffEntry {
                            scenario: base.name.clone(),
                            metric: key.clone(),
                            status: classify_delta(key, delta),
                        });
                    }
                }
            }
        }

        DiffReport { entries }
    }
}

/// Domain-aware classification of deltas.
fn classify_delta(key: &str, delta: f64) -> DiffStatus {
    // Keys where LOWER is better
    let lower_is_better = ["strokes", "dist_to_hole"];

    if lower_is_better.iter().any(|k| key.contains(k)) {
        if delta < 0.0 {
            DiffStatus::Improved {
                delta,
                detail: format!("{} decreased by {:.2}", key, delta.abs()),
            }
        } else {
            DiffStatus::Regressed {
                delta,
                detail: format!("{} increased by {:.2}", key, delta.abs()),
            }
        }
    } else {
        DiffStatus::Changed {
            detail: format!("{} delta={:.2}", key, delta),
        }
    }
}
