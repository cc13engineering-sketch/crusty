use crate::engine::Engine;
use super::runner::{HeadlessRunner, SimResult};

/// A single parameter configuration for a sweep run.
#[derive(Clone, Debug)]
pub struct SweepConfig {
    /// Human-readable label for this config (e.g. "drag=2.0, restitution=0.5").
    pub label: String,
    /// Game state overrides to apply after setup.
    pub overrides: Vec<(String, f64)>,
}

/// Result of a single sweep run, pairing config with simulation output.
#[derive(Clone, Debug)]
pub struct SweepResult {
    pub config: SweepConfig,
    pub sim: SimResult,
}

/// Result of an entire parameter sweep.
#[derive(Clone, Debug)]
pub struct SweepReport {
    pub results: Vec<SweepResult>,
}

impl SweepReport {
    /// Find the result where a game state key has the minimum value.
    pub fn min_by_state(&self, key: &str) -> Option<&SweepResult> {
        self.results.iter().min_by(|a, b| {
            let va = a.sim.game_state.get(key).and_then(|v| v.as_f64()).unwrap_or(f64::MAX);
            let vb = b.sim.game_state.get(key).and_then(|v| v.as_f64()).unwrap_or(f64::MAX);
            va.partial_cmp(&vb).unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Find the result where a game state key has the maximum value.
    pub fn max_by_state(&self, key: &str) -> Option<&SweepResult> {
        self.results.iter().max_by(|a, b| {
            let va = a.sim.game_state.get(key).and_then(|v| v.as_f64()).unwrap_or(f64::MIN);
            let vb = b.sim.game_state.get(key).and_then(|v| v.as_f64()).unwrap_or(f64::MIN);
            va.partial_cmp(&vb).unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Generate a compact summary for AI consumption.
    ///
    /// Lists each configuration with its framebuffer hash and any overridden
    /// state keys. Game-agnostic: reports only what was configured in the sweep.
    pub fn summary(&self) -> String {
        let mut out = format!("Sweep: {} configurations\n", self.results.len());
        for r in &self.results {
            let overrides_str: String = r.config.overrides.iter()
                .map(|(k, v)| format!("{}={:.2}", k, v))
                .collect::<Vec<_>>()
                .join(", ");
            let info = if overrides_str.is_empty() { "baseline".to_string() } else { overrides_str };
            out.push_str(&format!(
                "  [{}] {} fb={:#x}\n",
                r.config.label, info, r.sim.framebuffer_hash
            ));
        }
        out
    }

    /// Generate a summary showing specific state keys.
    pub fn summary_with_keys(&self, keys: &[&str]) -> String {
        let mut out = format!("Sweep: {} configurations\n", self.results.len());
        for r in &self.results {
            let vals: String = keys.iter()
                .map(|k| {
                    let v = r.sim.game_state.get(*k)
                        .and_then(|v| v.as_f64())
                        .map_or("?".to_string(), |v| format!("{:.2}", v));
                    format!("{}={}", k, v)
                })
                .collect::<Vec<_>>()
                .join(" ");
            out.push_str(&format!(
                "  [{}] {} fb={:#x}\n",
                r.config.label, vals, r.sim.framebuffer_hash
            ));
        }
        out
    }
}

/// Run a parameter sweep: execute the same scenario with different configs.
///
/// Game-agnostic: supply your own `action_dispatch` to route ScheduledActions
/// to your game's input handlers.
pub fn run_sweep(
    setup_fn: fn(&mut Engine),
    update_fn: fn(&mut Engine, f64),
    render_fn: fn(&mut Engine),
    action_dispatch: fn(&mut Engine, &super::scenario::ScheduledAction),
    actions: &[super::scenario::ScheduledAction],
    configs: &[SweepConfig],
    frames: u64,
) -> SweepReport {
    let mut results = Vec::with_capacity(configs.len());

    for config in configs {
        let mut runner = HeadlessRunner::new(480, 720);
        let mut sorted_actions: Vec<_> = actions.to_vec();
        sorted_actions.sort_by_key(|a| a.frame());

        let overrides = config.overrides.clone();
        let sim = runner.run_with_frame_cb(
            |engine| {
                setup_fn(engine);
                // Apply overrides after setup
                for (key, value) in &overrides {
                    engine.global_state.set_f64(key, *value);
                }
            },
            |engine, frame, dt| {
                for action in &sorted_actions {
                    if action.frame() == frame {
                        action_dispatch(engine, action);
                    }
                }
                update_fn(engine, dt);
                render_fn(engine);
            },
            frames,
        );

        results.push(SweepResult {
            config: config.clone(),
            sim,
        });
    }

    SweepReport { results }
}
