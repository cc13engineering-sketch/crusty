/// ENGINE MODULE: Dashboard
/// Generates structured JSON data for the continuous design dashboard.
///
/// Runs a suite of analyses (sweep, death classification, highlights,
/// ablation) and assembles the results into a single `DashboardData`
/// struct that the static HTML dashboard consumes.

use serde::{Serialize, Deserialize};
use crate::simulation::Simulation;
use crate::input_frame::InputFrame;
use super::runner::{HeadlessRunner, RunConfig};
use super::death_classify::{ClassifierConfig, classify};
use super::death_report::{DeathReport, classify_batch};
use super::highlights::{HighlightConfig, HighlightReport, scan_for_highlights_report};
use super::ablation::{AblationConfig, AblationReport, run_ablation_study};

// ─── Configuration ──────────────────────────────────────────────────

/// Configuration for dashboard data generation.
#[derive(Clone, Debug)]
pub struct DashboardConfig {
    /// Range of seeds to sweep. Default: (0, 100).
    pub seed_range: (u64, u64),
    /// Frames per run. Default: 600.
    pub frames: u64,
    /// Metric key to track. Default: "score".
    pub metric_key: String,
    /// Ablation configurations (if empty, uses defaults).
    pub ablation_config: Option<AblationConfig>,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            seed_range: (0, 100),
            frames: 600,
            metric_key: "score".to_string(),
            ablation_config: None,
        }
    }
}

// ─── Data Types ─────────────────────────────────────────────────────

/// Complete dashboard data, serializable to JSON for the HTML dashboard.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardData {
    /// Engine version string.
    pub engine_version: String,
    /// Timestamp of generation (ISO 8601).
    pub generated_at: String,
    /// Sweep summary statistics.
    pub sweep: SweepSummary,
    /// Death classification breakdown.
    pub deaths: DeathSummary,
    /// Top highlights across all seeds.
    pub highlights: Vec<HighlightEntry>,
    /// Ablation impact ranking.
    pub ablation: Option<AblationSummary>,
    /// Score distribution histogram.
    pub histogram: Vec<HistogramBin>,
}

/// Summary statistics from the sweep.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SweepSummary {
    pub total_seeds: u64,
    pub frames_per_run: u64,
    pub metric_key: String,
    pub mean: f64,
    pub median: f64,
    pub min: f64,
    pub max: f64,
    pub stddev: f64,
}

/// Summary of death classifications.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeathSummary {
    pub total: usize,
    pub close_call: usize,
    pub blowout: usize,
    pub cliff: usize,
    pub attrition: usize,
    pub unclassified: usize,
}

/// A single highlight entry for the dashboard.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HighlightEntry {
    pub seed: u64,
    pub frame: u64,
    pub kind: String,
    pub value: f64,
    pub delta: f64,
    pub description: String,
}

/// Summary of ablation results for dashboard rendering.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AblationSummary {
    pub baseline_mean: f64,
    pub ablations: Vec<AblationEntry>,
}

/// A single ablation entry for the dashboard.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AblationEntry {
    pub name: String,
    pub mean_score: f64,
    pub delta_percent: f64,
}

/// A histogram bin for score distribution.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistogramBin {
    pub low: f64,
    pub high: f64,
    pub count: usize,
}

// ─── MetricStats ────────────────────────────────────────────────────

/// Compute basic statistics for a slice of f64 values.
pub fn compute_stats(values: &[f64]) -> (f64, f64, f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0, 0.0, 0.0);
    }
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let variance = if values.len() > 1 {
        values.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / (n - 1.0)
    } else {
        0.0
    };
    let stddev = variance.sqrt();

    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median = if sorted.len() % 2 == 0 {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
    } else {
        sorted[sorted.len() / 2]
    };

    (mean, median, min, max, stddev)
}

/// Build a histogram from values with a given number of bins.
pub fn build_histogram(values: &[f64], num_bins: usize) -> Vec<HistogramBin> {
    if values.is_empty() || num_bins == 0 {
        return Vec::new();
    }

    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;

    if range < f64::EPSILON {
        return vec![HistogramBin { low: min, high: max, count: values.len() }];
    }

    let bin_width = range / num_bins as f64;
    let mut bins: Vec<HistogramBin> = (0..num_bins).map(|i| {
        HistogramBin {
            low: min + i as f64 * bin_width,
            high: min + (i + 1) as f64 * bin_width,
            count: 0,
        }
    }).collect();

    for &v in values {
        let idx = ((v - min) / bin_width).floor() as usize;
        let idx = idx.min(num_bins - 1); // clamp last value into last bin
        bins[idx].count += 1;
    }

    bins
}

// ─── Generator ──────────────────────────────────────────────────────

/// Generate complete dashboard data by running all analyses.
pub fn generate_dashboard_data<S: Simulation>(
    runner_factory: impl Fn() -> HeadlessRunner,
    game_factory: impl Fn() -> S,
    config: &DashboardConfig,
) -> DashboardData {
    let (seed_start, seed_end) = config.seed_range;
    let seeds: Vec<u64> = (seed_start..seed_end).collect();

    // 1. Run sweep: collect per-seed scores and per-frame metric trajectories
    let run_config = RunConfig { turbo: true, capture_state_hashes: false };
    let capture_keys = vec![config.metric_key.clone()];
    let mut scores = Vec::with_capacity(seeds.len());
    let mut trajectories: Vec<(u64, Vec<f64>)> = Vec::with_capacity(seeds.len());
    let mut all_highlights = Vec::new();

    let highlight_config = HighlightConfig {
        spike_threshold: 2.0,
        near_death_threshold: 5.0,
        milestone_values: vec![100.0, 500.0, 1000.0],
        tracked_metrics: vec![config.metric_key.clone()],
        window_size: 30,
    };

    for &seed in &seeds {
        let mut runner = runner_factory();
        let mut game = game_factory();
        let inputs: Vec<InputFrame> = vec![InputFrame::default(); config.frames as usize];

        let (result, captured) = runner.run_with_capture(
            &mut game, seed, &inputs, config.frames,
            run_config.clone(), &capture_keys,
        );

        let score = result.game_state.get(&config.metric_key)
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        scores.push(score);

        // Collect per-frame metric values for death classification
        let series: Vec<f64> = captured.iter().map(|row| {
            row.iter()
                .find(|(k, _)| k == &config.metric_key)
                .map(|(_, v)| *v)
                .unwrap_or(0.0)
        }).collect();
        trajectories.push((seed, series.clone()));

        // Highlights for this seed
        let frame_values: Vec<(u64, f64)> = series.iter().enumerate()
            .map(|(i, &v)| (i as u64, v))
            .collect();
        let report = scan_for_highlights_report(&config.metric_key, &frame_values, &highlight_config);
        for h in report.highlights {
            all_highlights.push(HighlightEntry {
                seed,
                frame: h.frame,
                kind: format!("{:?}", h.kind),
                value: h.value,
                delta: h.delta,
                description: h.description,
            });
        }
    }

    // 2. Compute sweep statistics
    let (mean, median, min, max, stddev) = compute_stats(&scores);
    let sweep = SweepSummary {
        total_seeds: seeds.len() as u64,
        frames_per_run: config.frames,
        metric_key: config.metric_key.clone(),
        mean, median, min, max, stddev,
    };

    // 3. Death classification
    let classifier_config = ClassifierConfig::default()
        .with_metric_key(&config.metric_key);
    let death_report = classify_batch(&trajectories, &classifier_config);
    let deaths = DeathSummary {
        total: death_report.classifications.len(),
        close_call: death_report.classifications.iter()
            .filter(|c| format!("{:?}", c.classification.class) == "CloseCall").count(),
        blowout: death_report.classifications.iter()
            .filter(|c| format!("{:?}", c.classification.class) == "Blowout").count(),
        cliff: death_report.classifications.iter()
            .filter(|c| format!("{:?}", c.classification.class) == "Cliff").count(),
        attrition: death_report.classifications.iter()
            .filter(|c| format!("{:?}", c.classification.class) == "Attrition").count(),
        unclassified: death_report.classifications.iter()
            .filter(|c| format!("{:?}", c.classification.class) == "Unclassified").count(),
    };

    // 4. Sort highlights by absolute delta and take top 20
    all_highlights.sort_by(|a, b| b.delta.abs().partial_cmp(&a.delta.abs()).unwrap_or(std::cmp::Ordering::Equal));
    all_highlights.truncate(20);

    // 5. Histogram
    let histogram = build_histogram(&scores, 20);

    // 6. Ablation (optional, can be expensive)
    let ablation = config.ablation_config.as_ref().map(|ab_config| {
        let report = run_ablation_study(
            &runner_factory,
            &game_factory,
            ab_config,
        );
        AblationSummary {
            baseline_mean: report.baseline.mean_score,
            ablations: report.ablations.iter().map(|a| AblationEntry {
                name: a.ablation_name.clone(),
                mean_score: a.mean_score,
                delta_percent: a.delta_percent,
            }).collect(),
        }
    });

    DashboardData {
        engine_version: env!("CARGO_PKG_VERSION").to_string(),
        generated_at: "".to_string(), // CLI fills this in
        sweep,
        deaths,
        highlights: all_highlights,
        ablation,
        histogram,
    }
}

// ─── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_stats_empty() {
        let (mean, median, min, max, stddev) = compute_stats(&[]);
        assert_eq!(mean, 0.0);
        assert_eq!(median, 0.0);
        assert_eq!(min, 0.0);
        assert_eq!(max, 0.0);
        assert_eq!(stddev, 0.0);
    }

    #[test]
    fn compute_stats_known_values() {
        let values = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let (mean, median, min, max, stddev) = compute_stats(&values);
        assert!((mean - 30.0).abs() < 1e-10);
        assert!((median - 30.0).abs() < 1e-10);
        assert!((min - 10.0).abs() < 1e-10);
        assert!((max - 50.0).abs() < 1e-10);
        // Sample stddev of [10,20,30,40,50]: variance = 1000/4 = 250, stddev = sqrt(250)
        assert!((stddev - (250.0_f64).sqrt()).abs() < 1e-10);
    }

    #[test]
    fn compute_stats_single_value() {
        let (mean, median, min, max, stddev) = compute_stats(&[42.0]);
        assert!((mean - 42.0).abs() < 1e-10);
        assert!((median - 42.0).abs() < 1e-10);
        assert!((min - 42.0).abs() < 1e-10);
        assert!((max - 42.0).abs() < 1e-10);
        assert_eq!(stddev, 0.0);
    }

    #[test]
    fn histogram_empty() {
        assert!(build_histogram(&[], 10).is_empty());
    }

    #[test]
    fn histogram_single_value() {
        let bins = build_histogram(&[42.0], 10);
        assert_eq!(bins.len(), 1);
        assert_eq!(bins[0].count, 1);
    }

    #[test]
    fn histogram_uniform() {
        let values: Vec<f64> = (0..100).map(|i| i as f64).collect();
        let bins = build_histogram(&values, 10);
        assert_eq!(bins.len(), 10);
        let total: usize = bins.iter().map(|b| b.count).sum();
        assert_eq!(total, 100);
    }

    #[test]
    fn histogram_bins_cover_range() {
        let values = vec![0.0, 25.0, 50.0, 75.0, 100.0];
        let bins = build_histogram(&values, 4);
        assert_eq!(bins.len(), 4);
        assert!((bins[0].low - 0.0).abs() < 1e-10);
        assert!((bins[3].high - 100.0).abs() < 1e-10);
    }

    #[test]
    fn dashboard_data_serde_roundtrip() {
        let data = DashboardData {
            engine_version: "0.1.0".to_string(),
            generated_at: "2025-01-01T00:00:00Z".to_string(),
            sweep: SweepSummary {
                total_seeds: 10,
                frames_per_run: 600,
                metric_key: "score".to_string(),
                mean: 42.0,
                median: 40.0,
                min: 10.0,
                max: 80.0,
                stddev: 15.0,
            },
            deaths: DeathSummary {
                total: 10,
                close_call: 2,
                blowout: 3,
                cliff: 1,
                attrition: 2,
                unclassified: 2,
            },
            highlights: vec![HighlightEntry {
                seed: 42,
                frame: 100,
                kind: "ScoreSpike".to_string(),
                value: 50.0,
                delta: 30.0,
                description: "test".to_string(),
            }],
            ablation: Some(AblationSummary {
                baseline_mean: 42.0,
                ablations: vec![AblationEntry {
                    name: "no_gravity".to_string(),
                    mean_score: 30.0,
                    delta_percent: -28.6,
                }],
            }),
            histogram: vec![HistogramBin { low: 0.0, high: 50.0, count: 5 }],
        };

        let json = serde_json::to_string(&data).expect("serialize");
        let back: DashboardData = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.engine_version, "0.1.0");
        assert_eq!(back.sweep.total_seeds, 10);
        assert_eq!(back.deaths.total, 10);
        assert_eq!(back.highlights.len(), 1);
        assert!(back.ablation.is_some());
        assert_eq!(back.histogram.len(), 1);
    }

    #[test]
    fn generate_dashboard_data_integration() {
        use crate::demo_ball::DemoBall;

        let config = DashboardConfig {
            seed_range: (0, 5),
            frames: 60,
            metric_key: "score".to_string(),
            ablation_config: None,
        };

        let data = generate_dashboard_data(
            || HeadlessRunner::new(480, 270),
            DemoBall::new,
            &config,
        );

        assert_eq!(data.sweep.total_seeds, 5);
        assert_eq!(data.sweep.frames_per_run, 60);
        assert_eq!(data.deaths.total, 5);
        assert!(!data.histogram.is_empty());
        assert!(data.ablation.is_none());
    }
}
