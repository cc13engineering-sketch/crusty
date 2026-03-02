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
    pub fn summary(&self) -> String {
        let mut out = format!("Sweep: {} configurations\n", self.results.len());
        for r in &self.results {
            let phase = r.sim.game_state.get("tl_phase").and_then(|v| v.as_f64()).unwrap_or(-1.0);
            let strokes = r.sim.game_state.get("strokes").and_then(|v| v.as_f64()).unwrap_or(0.0);
            out.push_str(&format!(
                "  [{}] phase={} strokes={} fb={:#x}\n",
                r.config.label, phase, strokes, r.sim.framebuffer_hash
            ));
        }
        out
    }
}

/// Run a parameter sweep: execute the same scenario with different configs.
///
/// `setup_fn` initializes the game, `actions_fn` provides the input sequence,
/// `configs` specifies the parameter variations. After each setup, the
/// overrides are applied to GameState before the simulation runs.
pub fn run_sweep(
    setup_fn: fn(&mut Engine),
    update_fn: fn(&mut Engine, f64),
    render_fn: fn(&mut Engine),
    actions: &[super::scenario::ScheduledAction],
    configs: &[SweepConfig],
    frames: u64,
) -> SweepReport {
    let mut results = Vec::with_capacity(configs.len());

    for config in configs {
        let mut runner = HeadlessRunner::new(480, 720);
        let sorted_actions: Vec<_> = {
            let mut a = actions.to_vec();
            a.sort_by_key(|a| match a {
                super::scenario::ScheduledAction::PointerDown { frame, .. } => *frame,
                super::scenario::ScheduledAction::PointerMove { frame, .. } => *frame,
                super::scenario::ScheduledAction::PointerUp { frame, .. } => *frame,
            });
            a
        };

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
                    let af = match action {
                        super::scenario::ScheduledAction::PointerDown { frame, .. } => *frame,
                        super::scenario::ScheduledAction::PointerMove { frame, .. } => *frame,
                        super::scenario::ScheduledAction::PointerUp { frame, .. } => *frame,
                    };
                    if af == frame {
                        super::scenario::dispatch_action_pub(engine, action);
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
