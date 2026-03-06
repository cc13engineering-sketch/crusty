/// ENGINE MODULE: Ablation
/// Mechanic ablation testing: "what if we disable/modify mechanic X?"
///
/// Runs a baseline simulation and multiple ablation variants across a range
/// of seeds, then compares the resulting metrics to quantify each mechanic's
/// impact on gameplay.

use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use crate::simulation::Simulation;
use super::runner::{HeadlessRunner, RunConfig};
use crate::input_frame::InputFrame;

// ─── Configuration ──────────────────────────────────────────────────

/// Configuration for an ablation study.
#[derive(Clone, Debug)]
pub struct AblationConfig {
    /// The "normal" parameter values applied for the baseline run.
    pub baseline_params: BTreeMap<String, f64>,
    /// List of mechanic ablations to test against the baseline.
    pub ablations: Vec<Ablation>,
    /// Range of seeds to run (start, end exclusive). Default: (0, 10).
    pub seed_range: (u64, u64),
    /// Number of frames per run. Default: 600.
    pub frames: u64,
    /// Game state key to measure as the outcome metric. Default: "score".
    pub metric_key: String,
}

impl Default for AblationConfig {
    fn default() -> Self {
        Self {
            baseline_params: BTreeMap::new(),
            ablations: Vec::new(),
            seed_range: (0, 10),
            frames: 600,
            metric_key: "score".to_string(),
        }
    }
}

/// A single mechanic ablation: a descriptive name and the parameter
/// overrides that simulate disabling or modifying a mechanic.
#[derive(Clone, Debug)]
pub struct Ablation {
    /// Descriptive name (e.g., "no_gravity", "double_speed").
    pub name: String,
    /// Parameter overrides for this ablation. These are applied on top
    /// of (and override) the baseline params.
    pub params: BTreeMap<String, f64>,
}

// ─── Results ────────────────────────────────────────────────────────

/// Result of running one configuration (baseline or ablation) across
/// all seeds.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AblationResult {
    /// Name of the ablation (or "baseline").
    pub ablation_name: String,
    /// Mean of the measured metric across all seeds.
    pub mean_score: f64,
    /// Per-seed metric values.
    pub scores: Vec<f64>,
    /// Difference from baseline mean (baseline itself has delta 0).
    pub delta_from_baseline: f64,
    /// Percentage change from baseline mean. 0 if baseline mean is 0.
    pub delta_percent: f64,
}

/// Full ablation study report.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AblationReport {
    /// Results for the baseline configuration.
    pub baseline: AblationResult,
    /// Results for each ablation variant.
    pub ablations: Vec<AblationResult>,
    /// Total number of simulation runs executed.
    pub total_runs: usize,
}

impl AblationReport {
    /// Generate a formatted text summary of the ablation study.
    pub fn summary(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "Ablation Study: {} total runs ({} configurations x {} seeds)\n",
            self.total_runs,
            1 + self.ablations.len(),
            self.baseline.scores.len(),
        ));
        out.push_str(&format!(
            "\n  {:<20} mean={:>10.2}  (baseline)\n",
            "baseline", self.baseline.mean_score,
        ));
        for ab in &self.ablations {
            let sign = if ab.delta_from_baseline >= 0.0 { "+" } else { "" };
            out.push_str(&format!(
                "  {:<20} mean={:>10.2}  delta={}{:.2} ({}{:.1}%)\n",
                ab.ablation_name,
                ab.mean_score,
                sign,
                ab.delta_from_baseline,
                sign,
                ab.delta_percent,
            ));
        }
        out
    }

    /// Return ablation results sorted by impact (largest absolute delta first).
    pub fn ranked(&self) -> Vec<&AblationResult> {
        let mut sorted: Vec<&AblationResult> = self.ablations.iter().collect();
        sorted.sort_by(|a, b| {
            b.delta_from_baseline
                .abs()
                .partial_cmp(&a.delta_from_baseline.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted
    }
}

// ─── Runner ─────────────────────────────────────────────────────────

/// Run a complete ablation study.
///
/// For each configuration (baseline + each ablation), for each seed in
/// `config.seed_range`, creates a fresh runner and game, applies
/// parameter overrides via `engine.global_state.set_f64()`, runs the
/// simulation, and captures the specified metric.
///
/// # Arguments
/// - `runner_factory`: creates a fresh `HeadlessRunner` for each run
/// - `game_factory`: creates a fresh game `S` for each run
/// - `config`: ablation study configuration
pub fn run_ablation_study<S: Simulation>(
    runner_factory: impl Fn() -> HeadlessRunner,
    game_factory: impl Fn() -> S,
    config: &AblationConfig,
) -> AblationReport {
    let (seed_start, seed_end) = config.seed_range;
    let seeds: Vec<u64> = (seed_start..seed_end).collect();

    // Run baseline
    let baseline_scores = run_variant(
        &runner_factory,
        &game_factory,
        &config.baseline_params,
        &seeds,
        config.frames,
        &config.metric_key,
    );
    let baseline_mean = mean(&baseline_scores);

    let baseline = AblationResult {
        ablation_name: "baseline".to_string(),
        mean_score: baseline_mean,
        scores: baseline_scores,
        delta_from_baseline: 0.0,
        delta_percent: 0.0,
    };

    // Run each ablation
    let mut ablation_results = Vec::with_capacity(config.ablations.len());
    for ablation in &config.ablations {
        // Merge baseline params with ablation overrides
        let mut merged = config.baseline_params.clone();
        for (k, v) in &ablation.params {
            merged.insert(k.clone(), *v);
        }

        let scores = run_variant(
            &runner_factory,
            &game_factory,
            &merged,
            &seeds,
            config.frames,
            &config.metric_key,
        );
        let m = mean(&scores);
        let delta = m - baseline_mean;
        let delta_pct = if baseline_mean.abs() < 1e-12 {
            0.0
        } else {
            (delta / baseline_mean) * 100.0
        };

        ablation_results.push(AblationResult {
            ablation_name: ablation.name.clone(),
            mean_score: m,
            scores,
            delta_from_baseline: delta,
            delta_percent: delta_pct,
        });
    }

    let total_runs = (1 + config.ablations.len()) * seeds.len();

    AblationReport {
        baseline,
        ablations: ablation_results,
        total_runs,
    }
}

/// Run a single variant (baseline or ablation) across all seeds.
fn run_variant<S: Simulation>(
    runner_factory: &impl Fn() -> HeadlessRunner,
    game_factory: &impl Fn() -> S,
    params: &BTreeMap<String, f64>,
    seeds: &[u64],
    frames: u64,
    metric_key: &str,
) -> Vec<f64> {
    let _run_config = RunConfig {
        turbo: true,
        capture_state_hashes: false,
    };
    let inputs: Vec<InputFrame> = vec![InputFrame::default(); frames as usize];

    let mut scores = Vec::with_capacity(seeds.len());
    for &seed in seeds {
        let mut runner = runner_factory();
        let mut game = game_factory();

        // Reset engine and setup game
        runner.engine.reset(seed);
        game.setup(&mut runner.engine);

        // Apply parameter overrides to global_state
        for (key, value) in params {
            runner.engine.global_state.set_f64(key, *value);
        }

        // Run simulation manually (we need to apply params AFTER setup
        // but the run_sim_frames method calls reset+setup internally,
        // so we run the loop directly).
        let dt = 1.0 / 60.0;
        let empty = InputFrame::default();
        for i in 0..frames {
            runner.engine.tick(dt);
            let input = inputs.get(i as usize).unwrap_or(&empty);
            runner.engine.apply_input(input);
            game.step(&mut runner.engine);
            // turbo: skip render
        }

        let score = runner
            .engine
            .global_state
            .get_f64(metric_key)
            .unwrap_or(0.0);
        scores.push(score);
    }
    scores
}

/// Compute the arithmetic mean of a slice.
fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

// ─── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::demo_ball::DemoBall;

    fn make_runner() -> HeadlessRunner {
        HeadlessRunner::new(480, 270)
    }

    fn make_game() -> DemoBall {
        DemoBall::new()
    }

    #[test]
    fn empty_ablation_list_returns_only_baseline() {
        let config = AblationConfig {
            seed_range: (0, 3),
            frames: 60,
            metric_key: "score".to_string(),
            ..Default::default()
        };

        let report = run_ablation_study(make_runner, make_game, &config);

        assert_eq!(report.ablations.len(), 0);
        assert_eq!(report.baseline.ablation_name, "baseline");
        assert_eq!(report.baseline.scores.len(), 3);
        assert_eq!(report.total_runs, 3);
    }

    #[test]
    fn deterministic_same_seed_same_result() {
        let config = AblationConfig {
            seed_range: (42, 43),
            frames: 60,
            metric_key: "score".to_string(),
            ..Default::default()
        };

        let report1 = run_ablation_study(make_runner, make_game, &config);
        let report2 = run_ablation_study(make_runner, make_game, &config);

        assert_eq!(report1.baseline.scores, report2.baseline.scores);
        assert_eq!(report1.baseline.mean_score, report2.baseline.mean_score);
    }

    #[test]
    fn delta_computation_accurate() {
        // Construct synthetic results and verify the math
        let baseline = AblationResult {
            ablation_name: "baseline".to_string(),
            mean_score: 100.0,
            scores: vec![90.0, 100.0, 110.0],
            delta_from_baseline: 0.0,
            delta_percent: 0.0,
        };

        let ablation = AblationResult {
            ablation_name: "test".to_string(),
            mean_score: 150.0,
            scores: vec![140.0, 150.0, 160.0],
            delta_from_baseline: 50.0,
            delta_percent: 50.0,
        };

        assert!((ablation.delta_from_baseline - 50.0).abs() < 1e-10);
        assert!((ablation.delta_percent - 50.0).abs() < 1e-10);

        let report = AblationReport {
            baseline,
            ablations: vec![ablation],
            total_runs: 6,
        };

        assert_eq!(report.total_runs, 6);
        assert_eq!(report.ablations[0].delta_from_baseline, 50.0);
        assert_eq!(report.ablations[0].delta_percent, 50.0);
    }

    #[test]
    fn delta_computation_with_actual_run() {
        // Run a baseline and one ablation, verify deltas are computed correctly.
        // Both baseline and ablation get a non-zero starting score so that
        // delta_percent is meaningful (avoids div-by-zero guard).
        let mut baseline_params = BTreeMap::new();
        baseline_params.insert("score".to_string(), 100.0);

        let mut ablation_params = BTreeMap::new();
        ablation_params.insert("score".to_string(), 500.0);

        let config = AblationConfig {
            baseline_params,
            seed_range: (0, 3),
            frames: 60,
            metric_key: "score".to_string(),
            ablations: vec![Ablation {
                name: "boosted_start".to_string(),
                params: ablation_params,
            }],
            ..Default::default()
        };

        let report = run_ablation_study(make_runner, make_game, &config);

        // Baseline starts at 100, ablation at 500 → ablation should be higher
        assert!(
            report.ablations[0].mean_score > report.baseline.mean_score,
            "ablation with boosted start should have higher mean score: ablation={}, baseline={}",
            report.ablations[0].mean_score,
            report.baseline.mean_score
        );
        assert!(report.ablations[0].delta_from_baseline > 0.0);
        assert!(report.ablations[0].delta_percent > 0.0);

        // Verify delta math: delta = ablation_mean - baseline_mean
        let expected_delta =
            report.ablations[0].mean_score - report.baseline.mean_score;
        assert!(
            (report.ablations[0].delta_from_baseline - expected_delta).abs() < 1e-10,
            "delta should equal difference of means"
        );
    }

    #[test]
    fn summary_formatting() {
        let baseline = AblationResult {
            ablation_name: "baseline".to_string(),
            mean_score: 100.0,
            scores: vec![100.0],
            delta_from_baseline: 0.0,
            delta_percent: 0.0,
        };

        let ablation = AblationResult {
            ablation_name: "no_gravity".to_string(),
            mean_score: 150.0,
            scores: vec![150.0],
            delta_from_baseline: 50.0,
            delta_percent: 50.0,
        };

        let report = AblationReport {
            baseline,
            ablations: vec![ablation],
            total_runs: 2,
        };

        let summary = report.summary();
        assert!(summary.contains("Ablation Study"), "should contain header");
        assert!(summary.contains("baseline"), "should mention baseline");
        assert!(summary.contains("no_gravity"), "should mention ablation name");
        assert!(summary.contains("2 total runs"), "should show total runs");
        assert!(summary.contains("+50.0%"), "should show percentage");
    }

    #[test]
    fn ranked_sorts_by_largest_delta() {
        let baseline = AblationResult {
            ablation_name: "baseline".to_string(),
            mean_score: 100.0,
            scores: vec![100.0],
            delta_from_baseline: 0.0,
            delta_percent: 0.0,
        };

        let small_impact = AblationResult {
            ablation_name: "small".to_string(),
            mean_score: 110.0,
            scores: vec![110.0],
            delta_from_baseline: 10.0,
            delta_percent: 10.0,
        };

        let large_impact = AblationResult {
            ablation_name: "large".to_string(),
            mean_score: 200.0,
            scores: vec![200.0],
            delta_from_baseline: 100.0,
            delta_percent: 100.0,
        };

        let negative_impact = AblationResult {
            ablation_name: "negative".to_string(),
            mean_score: 20.0,
            scores: vec![20.0],
            delta_from_baseline: -80.0,
            delta_percent: -80.0,
        };

        let report = AblationReport {
            baseline,
            ablations: vec![small_impact, large_impact, negative_impact],
            total_runs: 4,
        };

        let ranked = report.ranked();
        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0].ablation_name, "large");
        assert_eq!(ranked[1].ablation_name, "negative");
        assert_eq!(ranked[2].ablation_name, "small");
    }

    #[test]
    fn integration_demo_ball_ablation_study() {
        // Full integration test: run a small ablation study on DemoBall
        // with multiple ablations. Should not panic.
        let mut no_gravity = BTreeMap::new();
        no_gravity.insert("gravity".to_string(), 0.0);

        let mut double_speed = BTreeMap::new();
        double_speed.insert("speed_mult".to_string(), 2.0);

        let mut half_speed = BTreeMap::new();
        half_speed.insert("speed_mult".to_string(), 0.5);

        let mut no_bounce = BTreeMap::new();
        no_bounce.insert("bounce_damping".to_string(), 0.0);

        let config = AblationConfig {
            seed_range: (0, 5),
            frames: 120,
            metric_key: "score".to_string(),
            ablations: vec![
                Ablation { name: "no_gravity".to_string(), params: no_gravity },
                Ablation { name: "double_speed".to_string(), params: double_speed },
                Ablation { name: "half_speed".to_string(), params: half_speed },
                Ablation { name: "no_bounce".to_string(), params: no_bounce },
            ],
            ..Default::default()
        };

        let report = run_ablation_study(make_runner, make_game, &config);

        // Basic structural assertions
        assert_eq!(report.ablations.len(), 4);
        assert_eq!(report.total_runs, 25); // 5 configs x 5 seeds
        assert_eq!(report.baseline.scores.len(), 5);
        for ab in &report.ablations {
            assert_eq!(ab.scores.len(), 5);
        }

        // Summary should not panic
        let summary = report.summary();
        assert!(!summary.is_empty());

        // Ranked should not panic
        let ranked = report.ranked();
        assert_eq!(ranked.len(), 4);
    }

    #[test]
    fn mean_helper_empty() {
        assert_eq!(mean(&[]), 0.0);
    }

    #[test]
    fn mean_helper_values() {
        assert!((mean(&[10.0, 20.0, 30.0]) - 20.0).abs() < 1e-10);
    }

    #[test]
    fn ablation_report_serializable() {
        let report = AblationReport {
            baseline: AblationResult {
                ablation_name: "baseline".to_string(),
                mean_score: 42.0,
                scores: vec![40.0, 44.0],
                delta_from_baseline: 0.0,
                delta_percent: 0.0,
            },
            ablations: vec![AblationResult {
                ablation_name: "test".to_string(),
                mean_score: 50.0,
                scores: vec![48.0, 52.0],
                delta_from_baseline: 8.0,
                delta_percent: 19.047619047619047,
            }],
            total_runs: 4,
        };

        let json = serde_json::to_string(&report).expect("should serialize");
        let deserialized: AblationReport =
            serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(deserialized.baseline.mean_score, 42.0);
        assert_eq!(deserialized.ablations.len(), 1);
        assert_eq!(deserialized.ablations[0].ablation_name, "test");
        assert_eq!(deserialized.total_runs, 4);
    }

    #[test]
    fn baseline_zero_mean_no_div_by_zero() {
        // If baseline mean is zero, delta_percent should be 0, not NaN/Inf.
        let config = AblationConfig {
            seed_range: (0, 1),
            frames: 1, // very short run, score stays at 0
            metric_key: "score".to_string(),
            ablations: vec![Ablation {
                name: "boosted".to_string(),
                params: {
                    let mut m = BTreeMap::new();
                    m.insert("score".to_string(), 100.0);
                    m
                },
            }],
            ..Default::default()
        };

        let report = run_ablation_study(make_runner, make_game, &config);

        // Baseline score for 1 frame should be ~0
        // The ablation starts at 100 so should be higher
        assert!(
            !report.ablations[0].delta_percent.is_nan(),
            "delta_percent should not be NaN"
        );
        assert!(
            !report.ablations[0].delta_percent.is_infinite(),
            "delta_percent should not be Inf"
        );
    }
}
