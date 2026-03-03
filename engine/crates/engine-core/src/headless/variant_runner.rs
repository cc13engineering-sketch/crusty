//! Variant runner: execute simulations with different parameter configurations.
//!
//! Provides `run_variant` for a single variant run and `sweep_variants` for
//! batch execution across multiple variants and seeds.

use crate::headless::runner::{HeadlessRunner, RunConfig, SimResult};
use crate::input_frame::InputFrame;
use crate::simulation::Simulation;
use crate::variant::ParamSet;

/// Result of running a single variant configuration.
#[derive(Clone, Debug)]
pub struct VariantResult {
    /// Name of the variant (from `ParamSet::display_name()`).
    pub variant_name: String,
    /// The parameter set that was applied.
    pub param_set: ParamSet,
    /// The simulation result.
    pub result: SimResult,
}

/// Run a single variant: create a fresh game, set up on engine, apply params, run.
///
/// The `game_factory` closure creates a fresh simulation instance for each run,
/// ensuring no state leaks between variant runs.
pub fn run_variant<S: Simulation>(
    runner: &mut HeadlessRunner,
    game_factory: &dyn Fn() -> S,
    seed: u64,
    frames: u64,
    config: RunConfig,
    param_set: &ParamSet,
) -> VariantResult {
    let mut game = game_factory();
    let dt = 1.0 / 60.0;

    runner.engine.reset(seed);
    game.setup(&mut runner.engine);

    // Apply variant parameters after setup
    param_set.apply_to(&mut runner.engine);

    let mut state_hashes = if config.capture_state_hashes {
        Vec::with_capacity(frames as usize)
    } else {
        Vec::new()
    };

    let empty = InputFrame::default();
    for _ in 0..frames {
        runner.engine.tick(dt);
        runner.engine.apply_input(&empty);
        game.step(&mut runner.engine);
        if !config.turbo {
            game.render(&mut runner.engine);
        }
        if config.capture_state_hashes {
            state_hashes.push(runner.engine.state_hash());
        }
    }

    let result = snapshot_result(runner, frames, state_hashes);

    VariantResult {
        variant_name: param_set.display_name().to_string(),
        param_set: param_set.clone(),
        result,
    }
}

/// Run each variant across each seed, collecting all results.
///
/// Total runs = `variants.len() * seeds.len()`.
pub fn sweep_variants<S: Simulation>(
    game_factory: &dyn Fn() -> S,
    seeds: &[u64],
    frames: u64,
    config: RunConfig,
    variants: &[ParamSet],
) -> Vec<VariantResult> {
    let mut results = Vec::with_capacity(variants.len() * seeds.len());

    for variant in variants {
        for &seed in seeds {
            let mut runner = HeadlessRunner::new(480, 270);
            let vr = run_variant(&mut runner, game_factory, seed, frames, config.clone(), variant);
            results.push(vr);
        }
    }

    results
}

/// Summary report for a variant sweep.
#[derive(Clone, Debug)]
pub struct VariantSweepReport {
    /// All individual variant results.
    pub results: Vec<VariantResult>,
}

impl VariantSweepReport {
    /// Build a report from sweep results.
    pub fn new(results: Vec<VariantResult>) -> Self {
        Self { results }
    }

    /// Generate a human-readable summary.
    pub fn summary(&self) -> String {
        use std::collections::BTreeMap;

        let mut by_variant: BTreeMap<String, Vec<&VariantResult>> = BTreeMap::new();
        for r in &self.results {
            by_variant.entry(r.variant_name.clone()).or_default().push(r);
        }

        let mut out = format!("Variant Sweep: {} total runs\n", self.results.len());
        for (name, runs) in &by_variant {
            let scores: Vec<f64> = runs.iter()
                .filter_map(|r| r.result.get_f64("score"))
                .collect();
            let n = scores.len();
            if n > 0 {
                let mean = scores.iter().sum::<f64>() / n as f64;
                let min = scores.iter().cloned().fold(f64::INFINITY, f64::min);
                let max = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                out.push_str(&format!(
                    "  {}: {} runs, score mean={:.2} min={:.2} max={:.2}\n",
                    name, n, mean, min, max,
                ));
            } else {
                out.push_str(&format!("  {}: {} runs (no score data)\n", name, runs.len()));
            }
        }
        out
    }
}

/// Internal helper: create a SimResult snapshot from the current engine state.
fn snapshot_result(
    runner: &HeadlessRunner,
    frames_run: u64,
    state_hashes: Vec<u64>,
) -> SimResult {
    use std::collections::HashMap;
    use crate::game_state::StateValue;

    let game_state: HashMap<String, StateValue> = runner
        .engine
        .global_state
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect();

    SimResult {
        frames_run,
        final_metrics: runner.engine.frame_metrics.clone(),
        game_state,
        framebuffer_hash: super::framebuffer_hash(&runner.engine.framebuffer),
        elapsed_sim_time: runner.engine.time,
        state_hash: runner.engine.state_hash(),
        state_hashes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::demo_ball::DemoBall;

    fn demo_factory() -> DemoBall {
        DemoBall::new()
    }

    #[test]
    fn run_variant_returns_result() {
        let mut runner = HeadlessRunner::new(480, 270);
        let ps = ParamSet::new().named("default");
        let vr = run_variant(
            &mut runner,
            &demo_factory,
            42,
            60,
            RunConfig { turbo: true, capture_state_hashes: false },
            &ps,
        );
        assert_eq!(vr.variant_name, "default");
        assert_eq!(vr.result.frames_run, 60);
    }

    #[test]
    fn different_variants_produce_different_results() {
        let config = RunConfig { turbo: true, capture_state_hashes: false };

        let default_ps = ParamSet::new().named("default");
        let fast_ps = ParamSet::new()
            .named("fast")
            .with("ball_speed", 400.0)
            .with("ball_friction", 0.99);

        let mut r1 = HeadlessRunner::new(480, 270);
        let vr1 = run_variant(&mut r1, &demo_factory, 42, 60, config.clone(), &default_ps);

        let mut r2 = HeadlessRunner::new(480, 270);
        let vr2 = run_variant(&mut r2, &demo_factory, 42, 60, config.clone(), &fast_ps);

        // Different params in global_state should produce different state hashes
        assert_ne!(vr1.result.state_hash, vr2.result.state_hash,
            "different variants must produce different state hashes");
    }

    #[test]
    fn same_variant_same_seed_is_deterministic() {
        let config = RunConfig { turbo: true, capture_state_hashes: false };
        let ps = ParamSet::new()
            .named("fast")
            .with("ball_speed", 400.0);

        let mut r1 = HeadlessRunner::new(480, 270);
        let vr1 = run_variant(&mut r1, &demo_factory, 42, 60, config.clone(), &ps);

        let mut r2 = HeadlessRunner::new(480, 270);
        let vr2 = run_variant(&mut r2, &demo_factory, 42, 60, config.clone(), &ps);

        assert_eq!(vr1.result.state_hash, vr2.result.state_hash,
            "same variant + same seed must be deterministic");
    }

    #[test]
    fn sweep_variants_collects_all_results() {
        let variants = vec![
            ParamSet::new().named("default"),
            ParamSet::new().named("fast").with("ball_speed", 400.0),
        ];
        let seeds = vec![1, 2, 3];
        let config = RunConfig { turbo: true, capture_state_hashes: false };

        let results = sweep_variants(&demo_factory, &seeds, 30, config, &variants);
        assert_eq!(results.len(), 6); // 2 variants * 3 seeds
    }

    #[test]
    fn variant_sweep_report_summary() {
        let variants = vec![
            ParamSet::new().named("default"),
            ParamSet::new().named("fast").with("ball_speed", 400.0),
        ];
        let seeds = vec![1, 2];
        let config = RunConfig { turbo: true, capture_state_hashes: false };

        let results = sweep_variants(&demo_factory, &seeds, 30, config, &variants);
        let report = VariantSweepReport::new(results);
        let summary = report.summary();
        assert!(summary.contains("default"));
        assert!(summary.contains("fast"));
        assert!(summary.contains("Variant Sweep"));
    }
}
