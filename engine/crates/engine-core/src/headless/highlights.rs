//! Interesting moment detection (highlights) for simulation metric series.
//!
//! Given a per-frame time series of a metric (e.g. score), this module
//! identifies statistically interesting moments: spikes, drops, milestone
//! crossings, and near-death events. It uses a rolling window to compute
//! local mean and standard deviation, then flags frames whose values
//! deviate significantly from the local statistics.
//!
//! # Example
//!
//! ```ignore
//! use engine_core::headless::highlights::*;
//!
//! let data: Vec<(u64, f64)> = (0..100).map(|i| (i, i as f64)).collect();
//! let config = HighlightConfig::default();
//! let highlights = scan_for_highlights(&data, &config);
//! ```

use serde::{Serialize, Deserialize};

// ─── Types ───────────────────────────────────────────────────────────

/// The kind of interesting moment detected.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HighlightKind {
    /// A sudden large increase in the metric.
    ScoreSpike,
    /// A sudden large decrease in the metric.
    ScoreDrop,
    /// A state-machine-level change (detected via value transitions).
    StateChange,
    /// The metric dropped dangerously close to zero.
    NearDeath,
    /// The metric crossed a notable milestone value.
    Milestone,
    /// A user-defined highlight kind.
    Custom(String),
}

/// A single detected interesting moment in a metric series.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Highlight {
    /// Frame number where the highlight was detected.
    pub frame: u64,
    /// The type of interesting moment.
    pub kind: HighlightKind,
    /// Which metric key this highlight refers to.
    pub metric_key: String,
    /// The metric value at the frame.
    pub value: f64,
    /// The change (delta) from the previous frame.
    pub delta: f64,
    /// Human-readable description of what happened.
    pub description: String,
}

/// Configuration controlling what counts as an "interesting moment".
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HighlightConfig {
    /// Detect frame-to-frame changes exceeding this many standard deviations
    /// from the rolling mean of deltas. Default: 2.0.
    pub spike_threshold: f64,
    /// Detect metric values that drop within this distance of zero. Default: 5.0.
    pub near_death_threshold: f64,
    /// Notable milestone values to detect crossings for. Default: [100, 500, 1000].
    pub milestone_values: Vec<f64>,
    /// Which metric keys to track. Default: ["score"].
    pub tracked_metrics: Vec<String>,
    /// Rolling window size (in frames) for computing local statistics. Default: 30.
    pub window_size: usize,
}

impl Default for HighlightConfig {
    fn default() -> Self {
        Self {
            spike_threshold: 2.0,
            near_death_threshold: 5.0,
            milestone_values: vec![100.0, 500.0, 1000.0],
            tracked_metrics: vec!["score".to_string()],
            window_size: 30,
        }
    }
}

/// A report summarizing all highlights found in a simulation run.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HighlightReport {
    /// All detected highlights, in frame order.
    pub highlights: Vec<Highlight>,
    /// Total number of frames analyzed.
    pub total_frames: u64,
    /// Human-readable summary of the report.
    pub summary: String,
}

impl HighlightReport {
    /// Build a report from a list of highlights and total frame count.
    pub fn new(mut highlights: Vec<Highlight>, total_frames: u64) -> Self {
        highlights.sort_by_key(|h| h.frame);
        let summary = Self::build_summary(&highlights, total_frames);
        Self {
            highlights,
            total_frames,
            summary,
        }
    }

    /// Generate a human-readable summary of the highlights.
    pub fn summary(&self) -> &str {
        &self.summary
    }

    fn build_summary(highlights: &[Highlight], total_frames: u64) -> String {
        if highlights.is_empty() {
            return format!(
                "Highlight report: {} frames analyzed, 0 interesting moments found.",
                total_frames,
            );
        }

        let spike_count = highlights.iter().filter(|h| h.kind == HighlightKind::ScoreSpike).count();
        let drop_count = highlights.iter().filter(|h| h.kind == HighlightKind::ScoreDrop).count();
        let near_death_count = highlights.iter().filter(|h| h.kind == HighlightKind::NearDeath).count();
        let milestone_count = highlights.iter().filter(|h| h.kind == HighlightKind::Milestone).count();
        let state_change_count = highlights.iter().filter(|h| h.kind == HighlightKind::StateChange).count();
        let custom_count = highlights.iter().filter(|h| matches!(h.kind, HighlightKind::Custom(_))).count();

        let mut parts = Vec::new();
        if spike_count > 0 { parts.push(format!("{} spike(s)", spike_count)); }
        if drop_count > 0 { parts.push(format!("{} drop(s)", drop_count)); }
        if near_death_count > 0 { parts.push(format!("{} near-death", near_death_count)); }
        if milestone_count > 0 { parts.push(format!("{} milestone(s)", milestone_count)); }
        if state_change_count > 0 { parts.push(format!("{} state change(s)", state_change_count)); }
        if custom_count > 0 { parts.push(format!("{} custom", custom_count)); }

        format!(
            "Highlight report: {} frames analyzed, {} interesting moments found ({})",
            total_frames,
            highlights.len(),
            parts.join(", "),
        )
    }
}

// ─── Statistical Helpers ─────────────────────────────────────────────

/// Compute the rolling mean and standard deviation of values in a window
/// centered around `index`.
///
/// The window extends `window/2` frames in each direction, clamped to
/// array bounds. Returns `(mean, stddev)`. If fewer than 2 values are
/// in the window, stddev is 0.0.
pub fn rolling_mean_stddev(values: &[f64], window: usize, index: usize) -> (f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0);
    }

    let half = window / 2;
    let start = if index >= half { index - half } else { 0 };
    let end = (index + half + 1).min(values.len());

    let slice = &values[start..end];
    let n = slice.len() as f64;

    if n < 1.0 {
        return (0.0, 0.0);
    }

    let mean = slice.iter().sum::<f64>() / n;

    if n < 2.0 {
        return (mean, 0.0);
    }

    let variance = slice.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / (n - 1.0);
    let stddev = variance.sqrt();

    (mean, stddev)
}

// ─── Core Detection ──────────────────────────────────────────────────

/// Scan a series of per-frame metric values for interesting moments.
///
/// `frame_values` is a slice of `(frame_number, value)` pairs, expected
/// in ascending frame order. The function applies several detection
/// passes using the rolling window statistics configured in `config`.
///
/// Returns a list of `Highlight`s (unsorted; caller may sort).
pub fn scan_for_highlights(frame_values: &[(u64, f64)], config: &HighlightConfig) -> Vec<Highlight> {
    if frame_values.len() < 2 {
        return Vec::new();
    }

    let mut highlights = Vec::new();

    // Extract raw values for statistical analysis
    let values: Vec<f64> = frame_values.iter().map(|&(_, v)| v).collect();

    // Compute frame-to-frame deltas
    let deltas: Vec<f64> = values.windows(2).map(|w| w[1] - w[0]).collect();

    // ── Spike and drop detection ─────────────────────────────────
    // For each delta, compare against the rolling mean+stddev of deltas.
    // If the absolute delta exceeds mean + spike_threshold * stddev, flag it.
    for i in 0..deltas.len() {
        let (mean, stddev) = rolling_mean_stddev(&deltas, config.window_size, i);
        let delta = deltas[i];
        let threshold = if stddev > f64::EPSILON {
            stddev * config.spike_threshold
        } else {
            // If there is no variance, use a minimum sensitivity based on
            // the absolute mean to avoid flagging everything or nothing.
            // If deltas are all zero, any non-zero change is interesting.
            let abs_mean = mean.abs();
            if abs_mean > f64::EPSILON {
                abs_mean * config.spike_threshold
            } else {
                // All deltas are zero; any non-zero delta is a spike
                f64::EPSILON
            }
        };

        let deviation = (delta - mean).abs();
        if deviation > threshold {
            let frame_idx = i + 1; // delta[i] is between frame i and i+1
            let (frame_num, value) = frame_values[frame_idx];
            if delta > 0.0 {
                highlights.push(Highlight {
                    frame: frame_num,
                    kind: HighlightKind::ScoreSpike,
                    metric_key: String::new(), // filled in by caller or below
                    value,
                    delta,
                    description: format!(
                        "Spike at frame {}: value={:.2}, delta={:.2} (mean_delta={:.2}, stddev={:.2})",
                        frame_num, value, delta, mean, stddev,
                    ),
                });
            } else {
                highlights.push(Highlight {
                    frame: frame_num,
                    kind: HighlightKind::ScoreDrop,
                    metric_key: String::new(),
                    value,
                    delta,
                    description: format!(
                        "Drop at frame {}: value={:.2}, delta={:.2} (mean_delta={:.2}, stddev={:.2})",
                        frame_num, value, delta, mean, stddev,
                    ),
                });
            }
        }
    }

    // ── Near-death detection ─────────────────────────────────────
    // Flag frames where the metric is positive but very close to zero
    // (and was previously higher).
    for i in 1..values.len() {
        let v = values[i];
        let prev = values[i - 1];
        if v >= 0.0 && v <= config.near_death_threshold && prev > config.near_death_threshold {
            let (frame_num, value) = frame_values[i];
            let delta = v - prev;
            highlights.push(Highlight {
                frame: frame_num,
                kind: HighlightKind::NearDeath,
                metric_key: String::new(),
                value,
                delta,
                description: format!(
                    "Near-death at frame {}: value={:.2} (was {:.2}), threshold={:.2}",
                    frame_num, value, prev, config.near_death_threshold,
                ),
            });
        }
    }

    // ── Milestone detection ──────────────────────────────────────
    // Detect when the metric crosses a milestone value from below.
    for &milestone in &config.milestone_values {
        for i in 1..values.len() {
            let prev = values[i - 1];
            let curr = values[i];
            if prev < milestone && curr >= milestone {
                let (frame_num, value) = frame_values[i];
                let delta = curr - prev;
                highlights.push(Highlight {
                    frame: frame_num,
                    kind: HighlightKind::Milestone,
                    metric_key: String::new(),
                    value,
                    delta,
                    description: format!(
                        "Milestone {:.0} reached at frame {}: value={:.2}",
                        milestone, frame_num, value,
                    ),
                });
            }
        }
    }

    highlights
}

/// Scan for highlights and return a full report, filling in the metric key.
pub fn scan_for_highlights_report(
    metric_key: &str,
    frame_values: &[(u64, f64)],
    config: &HighlightConfig,
) -> HighlightReport {
    let mut highlights = scan_for_highlights(frame_values, config);
    for h in &mut highlights {
        h.metric_key = metric_key.to_string();
    }
    let total_frames = frame_values.last().map(|&(f, _)| f + 1).unwrap_or(0);
    HighlightReport::new(highlights, total_frames)
}

// ─── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── rolling_mean_stddev tests ────────────────────────────────

    #[test]
    fn rolling_stats_empty() {
        let (mean, stddev) = rolling_mean_stddev(&[], 10, 0);
        assert_eq!(mean, 0.0);
        assert_eq!(stddev, 0.0);
    }

    #[test]
    fn rolling_stats_single_value() {
        let (mean, stddev) = rolling_mean_stddev(&[5.0], 10, 0);
        assert!((mean - 5.0).abs() < f64::EPSILON);
        assert_eq!(stddev, 0.0);
    }

    #[test]
    fn rolling_stats_known_data() {
        // Values: [2, 4, 4, 4, 5, 5, 7, 9]
        // Full-window mean = 5.0, stddev (sample) = 2.0 (approximately)
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let (mean, stddev) = rolling_mean_stddev(&data, 100, 4);
        // Mean = 40/8 = 5.0
        assert!((mean - 5.0).abs() < 1e-10, "mean={}", mean);
        // Sample variance = ((2-5)^2 + 3*(4-5)^2 + 2*(5-5)^2 + (7-5)^2 + (9-5)^2) / 7
        //                 = (9 + 3 + 0 + 4 + 16) / 7 = 32/7 ~= 4.571
        // Sample stddev ~= 2.138
        assert!((stddev - (32.0_f64 / 7.0).sqrt()).abs() < 1e-10, "stddev={}", stddev);
    }

    #[test]
    fn rolling_stats_small_window() {
        // Window=3, centered on index=2 => indices 1..=3
        let data = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let (mean, stddev) = rolling_mean_stddev(&data, 3, 2);
        // Window: indices 1,2,3 => [20, 30, 40], mean=30
        assert!((mean - 30.0).abs() < 1e-10, "mean={}", mean);
        // Sample stddev: sqrt(((20-30)^2 + 0 + (40-30)^2) / 2) = sqrt(200/2) = 10
        assert!((stddev - 10.0).abs() < 1e-10, "stddev={}", stddev);
    }

    #[test]
    fn rolling_stats_edge_of_array() {
        let data = vec![1.0, 2.0, 3.0];
        // Index 0, window=10 => start=0, end=min(0+5+1,3)=3 => whole array
        let (mean, _stddev) = rolling_mean_stddev(&data, 10, 0);
        assert!((mean - 2.0).abs() < 1e-10);
    }

    // ── scan_for_highlights tests ────────────────────────────────

    #[test]
    fn empty_data_returns_empty() {
        let config = HighlightConfig::default();
        let highlights = scan_for_highlights(&[], &config);
        assert!(highlights.is_empty());
    }

    #[test]
    fn single_data_point_returns_empty() {
        let config = HighlightConfig::default();
        let highlights = scan_for_highlights(&[(0, 42.0)], &config);
        assert!(highlights.is_empty());
    }

    #[test]
    fn detect_spike_in_synthetic_data() {
        // Flat data with one big jump
        let mut data: Vec<(u64, f64)> = (0..50).map(|i| (i, 10.0)).collect();
        // Inject a spike at frame 25
        data[25] = (25, 100.0);
        // Return to normal after
        for i in 26..50 {
            data[i] = (i as u64, 10.0);
        }

        let config = HighlightConfig {
            spike_threshold: 2.0,
            near_death_threshold: 5.0,
            milestone_values: vec![],
            tracked_metrics: vec!["score".into()],
            window_size: 10,
        };

        let highlights = scan_for_highlights(&data, &config);
        let spikes: Vec<_> = highlights.iter().filter(|h| h.kind == HighlightKind::ScoreSpike).collect();
        assert!(!spikes.is_empty(), "should detect at least one spike");
        // The spike should be around frame 25
        assert!(spikes.iter().any(|h| h.frame == 25), "spike should be at frame 25");
    }

    #[test]
    fn detect_drop_in_synthetic_data() {
        // Steady value with one big drop
        let mut data: Vec<(u64, f64)> = (0..50).map(|i| (i, 100.0)).collect();
        // Inject a drop at frame 25
        data[25] = (25, 10.0);
        // Return to normal after
        for i in 26..50 {
            data[i] = (i as u64, 100.0);
        }

        let config = HighlightConfig {
            spike_threshold: 2.0,
            near_death_threshold: 5.0,
            milestone_values: vec![],
            tracked_metrics: vec!["score".into()],
            window_size: 10,
        };

        let highlights = scan_for_highlights(&data, &config);
        let drops: Vec<_> = highlights.iter().filter(|h| h.kind == HighlightKind::ScoreDrop).collect();
        assert!(!drops.is_empty(), "should detect at least one drop");
        assert!(drops.iter().any(|h| h.frame == 25), "drop should be at frame 25");
    }

    #[test]
    fn detect_milestone_crossing() {
        // Linearly increasing data crossing 100
        let data: Vec<(u64, f64)> = (0..200).map(|i| (i, i as f64)).collect();

        let config = HighlightConfig {
            spike_threshold: 2.0,
            near_death_threshold: 5.0,
            milestone_values: vec![100.0],
            tracked_metrics: vec!["score".into()],
            window_size: 30,
        };

        let highlights = scan_for_highlights(&data, &config);
        let milestones: Vec<_> = highlights.iter().filter(|h| h.kind == HighlightKind::Milestone).collect();
        assert_eq!(milestones.len(), 1, "should detect exactly one milestone crossing");
        assert_eq!(milestones[0].frame, 100);
    }

    #[test]
    fn detect_multiple_milestones() {
        let data: Vec<(u64, f64)> = (0..1200).map(|i| (i, i as f64)).collect();

        let config = HighlightConfig {
            spike_threshold: 2.0,
            near_death_threshold: 5.0,
            milestone_values: vec![100.0, 500.0, 1000.0],
            tracked_metrics: vec!["score".into()],
            window_size: 30,
        };

        let highlights = scan_for_highlights(&data, &config);
        let milestones: Vec<_> = highlights.iter().filter(|h| h.kind == HighlightKind::Milestone).collect();
        assert_eq!(milestones.len(), 3, "should detect 3 milestones at 100, 500, 1000");
    }

    #[test]
    fn detect_near_death() {
        // Value drops from 50 to near zero
        let mut data: Vec<(u64, f64)> = (0..20).map(|i| (i, 50.0)).collect();
        // Drop to 3.0 (within default threshold of 5.0)
        data.push((20, 3.0));
        data.push((21, 2.0));
        data.push((22, 50.0));

        let config = HighlightConfig {
            spike_threshold: 2.0,
            near_death_threshold: 5.0,
            milestone_values: vec![],
            tracked_metrics: vec!["score".into()],
            window_size: 10,
        };

        let highlights = scan_for_highlights(&data, &config);
        let near_deaths: Vec<_> = highlights.iter().filter(|h| h.kind == HighlightKind::NearDeath).collect();
        assert!(!near_deaths.is_empty(), "should detect near-death event");
        assert!(near_deaths.iter().any(|h| h.frame == 20), "near-death at frame 20");
    }

    #[test]
    fn near_death_not_triggered_when_already_low() {
        // Value is always near zero -- should not trigger near-death every frame
        let data: Vec<(u64, f64)> = (0..50).map(|i| (i, 2.0)).collect();

        let config = HighlightConfig {
            spike_threshold: 2.0,
            near_death_threshold: 5.0,
            milestone_values: vec![],
            tracked_metrics: vec!["score".into()],
            window_size: 10,
        };

        let highlights = scan_for_highlights(&data, &config);
        let near_deaths: Vec<_> = highlights.iter().filter(|h| h.kind == HighlightKind::NearDeath).collect();
        assert!(near_deaths.is_empty(), "should not flag near-death when values are consistently low");
    }

    // ── HighlightReport tests ────────────────────────────────────

    #[test]
    fn report_summary_empty() {
        let report = HighlightReport::new(vec![], 100);
        assert!(report.summary().contains("0 interesting moments"));
        assert!(report.summary().contains("100 frames"));
    }

    #[test]
    fn report_summary_with_highlights() {
        let highlights = vec![
            Highlight {
                frame: 10,
                kind: HighlightKind::ScoreSpike,
                metric_key: "score".into(),
                value: 50.0,
                delta: 40.0,
                description: "test spike".into(),
            },
            Highlight {
                frame: 20,
                kind: HighlightKind::Milestone,
                metric_key: "score".into(),
                value: 100.0,
                delta: 5.0,
                description: "test milestone".into(),
            },
        ];
        let report = HighlightReport::new(highlights, 200);
        let s = report.summary();
        assert!(s.contains("2 interesting moments"), "summary: {}", s);
        assert!(s.contains("1 spike"), "summary: {}", s);
        assert!(s.contains("1 milestone"), "summary: {}", s);
    }

    #[test]
    fn report_highlights_sorted_by_frame() {
        let highlights = vec![
            Highlight {
                frame: 50,
                kind: HighlightKind::ScoreSpike,
                metric_key: "score".into(),
                value: 10.0,
                delta: 5.0,
                description: "later".into(),
            },
            Highlight {
                frame: 10,
                kind: HighlightKind::ScoreDrop,
                metric_key: "score".into(),
                value: 5.0,
                delta: -5.0,
                description: "earlier".into(),
            },
        ];
        let report = HighlightReport::new(highlights, 100);
        assert_eq!(report.highlights[0].frame, 10);
        assert_eq!(report.highlights[1].frame, 50);
    }

    // ── scan_for_highlights_report test ──────────────────────────

    #[test]
    fn report_fills_metric_key() {
        let mut data: Vec<(u64, f64)> = (0..50).map(|i| (i, 10.0)).collect();
        data[25] = (25, 100.0);

        let config = HighlightConfig {
            spike_threshold: 2.0,
            near_death_threshold: 5.0,
            milestone_values: vec![],
            tracked_metrics: vec!["score".into()],
            window_size: 10,
        };

        let report = scan_for_highlights_report("score", &data, &config);
        assert!(report.highlights.iter().all(|h| h.metric_key == "score"));
        assert!(report.total_frames > 0);
    }

    // ── Integration: DemoBall highlight detection ────────────────

    #[test]
    fn demo_ball_produces_highlights() {
        use crate::demo_ball::DemoBall;
        use crate::headless::{HeadlessRunner, RunConfig};
        use crate::input_frame::InputFrame;

        // Run DemoBall with some taps to create score movement
        let mut inputs: Vec<InputFrame> = vec![InputFrame::default(); 300];
        // Inject taps to make the ball move and generate score
        for i in [10, 30, 60, 100, 150, 200] {
            if i < inputs.len() {
                inputs[i] = InputFrame {
                    pointer_down: Some((400.0, 50.0)),
                    ..Default::default()
                };
            }
        }

        let mut runner = HeadlessRunner::new(480, 270);
        let mut game = DemoBall::new();
        let capture_keys = vec!["score".to_string()];
        let (_result, captured) = runner.run_with_capture(
            &mut game,
            42,
            &inputs,
            300,
            RunConfig { turbo: true, capture_state_hashes: false },
            &capture_keys,
        );

        // Build frame_values from captured data
        let frame_values: Vec<(u64, f64)> = captured
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let val = row.iter()
                    .find(|(k, _)| k == "score")
                    .map(|(_, v)| *v)
                    .unwrap_or(0.0);
                (i as u64, val)
            })
            .collect();

        let config = HighlightConfig {
            spike_threshold: 2.0,
            near_death_threshold: 5.0,
            milestone_values: vec![10.0, 50.0], // lower milestones for a short run
            tracked_metrics: vec!["score".into()],
            window_size: 20,
        };

        let report = scan_for_highlights_report("score", &frame_values, &config);
        // With taps creating ball movement and thus score changes, we should
        // get at least some highlights (milestones and/or spikes).
        assert!(
            !report.highlights.is_empty(),
            "DemoBall with taps should produce at least some highlights. Frames: {}, Report: {}",
            frame_values.len(),
            report.summary(),
        );
    }
}
