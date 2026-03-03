/// Divergence analysis between simulation runs.
///
/// Compares per-frame state hashes or sweep outcomes to find where
/// two simulation runs diverge and produce a human-readable report.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;

// ── Per-frame hash divergence ───────────────────────────────────────

/// Full report from comparing two sequences of per-frame state hashes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DivergenceReport {
    pub run_a: String,
    pub run_b: String,
    pub first_divergence_frame: Option<u64>,
    pub frames_compared: u64,
    pub final_hash_match: bool,
    pub divergent_frame_count: u64,
    pub context: Option<DivergenceContext>,
}

/// A window of frames around the first divergence point.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DivergenceContext {
    pub start_frame: u64,
    pub end_frame: u64,
    pub frames: Vec<ContextFrame>,
}

/// One frame within a divergence context window.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContextFrame {
    pub frame: u64,
    pub hash_a: u64,
    pub hash_b: u64,
    pub matches: bool,
}

impl DivergenceReport {
    /// Produce a human-readable summary string.
    pub fn summary(&self) -> String {
        let mut s = String::new();
        let _ = writeln!(s, "Divergence Report: {} vs {}", self.run_a, self.run_b);
        let _ = writeln!(s, "  Frames compared: {}", self.frames_compared);
        if let Some(frame) = self.first_divergence_frame {
            let _ = writeln!(s, "  First divergence at frame: {}", frame);
            let _ = writeln!(
                s,
                "  Total divergent frames: {}",
                self.divergent_frame_count
            );
        } else {
            let _ = writeln!(s, "  No divergence detected.");
        }
        let _ = writeln!(
            s,
            "  Final hash match: {}",
            if self.final_hash_match { "yes" } else { "no" }
        );
        s
    }
}

/// Compare two sets of per-frame state hashes and produce a [`DivergenceReport`].
///
/// `context_radius` controls how many frames before/after the first divergence
/// to include in the context window.
pub fn compare_hash_sequences(
    hashes_a: &[u64],
    hashes_b: &[u64],
    name_a: &str,
    name_b: &str,
    context_radius: usize,
) -> DivergenceReport {
    let compare_len = hashes_a.len().min(hashes_b.len());

    let mut first_divergence: Option<u64> = None;
    let mut divergent_count: u64 = 0;

    for i in 0..compare_len {
        if hashes_a[i] != hashes_b[i] {
            if first_divergence.is_none() {
                first_divergence = Some(i as u64);
            }
            divergent_count += 1;
        }
    }

    let final_hash_match = if hashes_a.is_empty() && hashes_b.is_empty() {
        true
    } else if hashes_a.is_empty() || hashes_b.is_empty() {
        false
    } else {
        hashes_a[hashes_a.len() - 1] == hashes_b[hashes_b.len() - 1]
    };

    let context = first_divergence.map(|div_frame| {
        let div_idx = div_frame as usize;
        let start = if div_idx >= context_radius {
            div_idx - context_radius
        } else {
            0
        };
        let end = (div_idx + context_radius + 1).min(compare_len);
        let frames = (start..end)
            .map(|i| ContextFrame {
                frame: i as u64,
                hash_a: hashes_a[i],
                hash_b: hashes_b[i],
                matches: hashes_a[i] == hashes_b[i],
            })
            .collect();
        DivergenceContext {
            start_frame: start as u64,
            end_frame: (end.saturating_sub(1)) as u64,
            frames,
        }
    });

    DivergenceReport {
        run_a: name_a.to_string(),
        run_b: name_b.to_string(),
        first_divergence_frame: first_divergence,
        frames_compared: compare_len as u64,
        final_hash_match,
        divergent_frame_count: divergent_count,
        context,
    }
}

// ── Sweep / batch outcome divergence ────────────────────────────────

/// Report from comparing two sets of sweep/batch results.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SweepDivergenceReport {
    pub pairs_compared: usize,
    pub ranked_seeds: Vec<SeedDivergence>,
    pub summary: DivergenceSummary,
}

/// One seed's divergence between two sweep runs.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeedDivergence {
    pub seed: u64,
    pub outcome_a: f64,
    pub outcome_b: f64,
    pub delta: f64,
}

/// Aggregate statistics for sweep divergence.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DivergenceSummary {
    pub total_seeds: usize,
    pub seeds_with_divergence: usize,
    pub max_delta: f64,
    pub mean_delta: f64,
}

impl SweepDivergenceReport {
    /// Produce a human-readable summary string.
    pub fn summary(&self) -> String {
        let mut s = String::new();
        let _ = writeln!(s, "Sweep Divergence Report");
        let _ = writeln!(s, "  Pairs compared: {}", self.pairs_compared);
        let _ = writeln!(
            s,
            "  Seeds with divergence: {} / {}",
            self.summary.seeds_with_divergence, self.summary.total_seeds
        );
        let _ = writeln!(s, "  Max delta: {:.6}", self.summary.max_delta);
        let _ = writeln!(s, "  Mean delta: {:.6}", self.summary.mean_delta);
        if !self.ranked_seeds.is_empty() {
            let _ = writeln!(s, "  Top divergent seeds:");
            for entry in self.ranked_seeds.iter().take(5) {
                let _ = writeln!(
                    s,
                    "    seed {}: a={:.4} b={:.4} delta={:.6}",
                    entry.seed, entry.outcome_a, entry.outcome_b, entry.delta
                );
            }
        }
        s
    }
}

/// Compare two sets of sweep results by matching seeds.
///
/// Each entry is `(seed, outcome_value)`. Seeds present in A but not B
/// (or vice-versa) are silently skipped.
pub fn compare_sweep_outcomes(
    results_a: &[(u64, f64)],
    results_b: &[(u64, f64)],
) -> SweepDivergenceReport {
    let map_b: HashMap<u64, f64> = results_b.iter().copied().collect();

    let mut ranked: Vec<SeedDivergence> = Vec::new();
    for &(seed, outcome_a) in results_a {
        if let Some(&outcome_b) = map_b.get(&seed) {
            let delta = (outcome_a - outcome_b).abs();
            ranked.push(SeedDivergence {
                seed,
                outcome_a,
                outcome_b,
                delta,
            });
        }
    }

    // Sort by delta descending (largest divergence first).
    ranked.sort_by(|a, b| b.delta.partial_cmp(&a.delta).unwrap_or(std::cmp::Ordering::Equal));

    let total_seeds = ranked.len();
    let seeds_with_divergence = ranked.iter().filter(|e| e.delta > 0.0).count();
    let max_delta = ranked
        .first()
        .map(|e| e.delta)
        .unwrap_or(0.0);
    let mean_delta = if total_seeds > 0 {
        ranked.iter().map(|e| e.delta).sum::<f64>() / total_seeds as f64
    } else {
        0.0
    };

    SweepDivergenceReport {
        pairs_compared: total_seeds,
        ranked_seeds: ranked,
        summary: DivergenceSummary {
            total_seeds,
            seeds_with_divergence,
            max_delta,
            mean_delta,
        },
    }
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_sequences_no_divergence() {
        let hashes = vec![10, 20, 30, 40, 50];
        let report = compare_hash_sequences(&hashes, &hashes, "a", "b", 2);
        assert_eq!(report.first_divergence_frame, None);
        assert_eq!(report.frames_compared, 5);
        assert!(report.final_hash_match);
        assert_eq!(report.divergent_frame_count, 0);
        assert!(report.context.is_none());
    }

    #[test]
    fn different_at_frame_5() {
        let a = vec![1, 2, 3, 4, 5, 100, 7, 8, 9, 10];
        let b = vec![1, 2, 3, 4, 5, 200, 7, 8, 9, 10];
        let report = compare_hash_sequences(&a, &b, "run_a", "run_b", 2);
        assert_eq!(report.first_divergence_frame, Some(5));
        assert_eq!(report.divergent_frame_count, 1);
        assert!(report.final_hash_match);
        assert!(report.context.is_some());
        let ctx = report.context.as_ref().unwrap();
        // context should cover frames 3..=7 (radius=2 around frame 5)
        assert_eq!(ctx.start_frame, 3);
        assert_eq!(ctx.end_frame, 7);
    }

    #[test]
    fn context_window_correct_size() {
        // Divergence at frame 1 with radius 3 — start clamped to 0.
        let a = vec![0, 99, 2, 3, 4, 5];
        let b = vec![0, 88, 2, 3, 4, 5];
        let report = compare_hash_sequences(&a, &b, "a", "b", 3);
        assert_eq!(report.first_divergence_frame, Some(1));
        let ctx = report.context.as_ref().unwrap();
        assert_eq!(ctx.start_frame, 0); // clamped
        assert_eq!(ctx.end_frame, 4); // 1+3 = 4
        assert_eq!(ctx.frames.len(), 5); // frames 0,1,2,3,4
        // frame 0 matches, frame 1 does not
        assert!(ctx.frames[0].matches);
        assert!(!ctx.frames[1].matches);
    }

    #[test]
    fn sweep_comparison_ranks_by_delta() {
        let a = vec![(1, 10.0), (2, 20.0), (3, 30.0)];
        let b = vec![(1, 10.0), (2, 25.0), (3, 33.0)];
        let report = compare_sweep_outcomes(&a, &b);
        assert_eq!(report.pairs_compared, 3);
        // Ranked by delta desc: seed 2 (delta=5), seed 3 (delta=3), seed 1 (delta=0)
        assert_eq!(report.ranked_seeds[0].seed, 2);
        assert_eq!(report.ranked_seeds[0].delta, 5.0);
        assert_eq!(report.ranked_seeds[1].seed, 3);
        assert_eq!(report.ranked_seeds[1].delta, 3.0);
        assert_eq!(report.ranked_seeds[2].seed, 1);
        assert_eq!(report.ranked_seeds[2].delta, 0.0);
        assert_eq!(report.summary.seeds_with_divergence, 2);
        assert!((report.summary.max_delta - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn empty_sequences() {
        let report = compare_hash_sequences(&[], &[], "empty_a", "empty_b", 2);
        assert_eq!(report.frames_compared, 0);
        assert_eq!(report.first_divergence_frame, None);
        assert!(report.final_hash_match);
        assert!(report.context.is_none());
    }

    #[test]
    fn report_summary_non_empty() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 99];
        let report = compare_hash_sequences(&a, &b, "run1", "run2", 1);
        let text = report.summary();
        assert!(!text.is_empty());
        assert!(text.contains("run1"));
        assert!(text.contains("run2"));
        assert!(text.contains("First divergence at frame: 2"));
    }

    #[test]
    fn serde_roundtrip_divergence_report() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![1, 2, 99, 4, 5];
        let report = compare_hash_sequences(&a, &b, "alpha", "beta", 1);
        let json = serde_json::to_string(&report).expect("serialize");
        let restored: DivergenceReport = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored.run_a, report.run_a);
        assert_eq!(restored.run_b, report.run_b);
        assert_eq!(
            restored.first_divergence_frame,
            report.first_divergence_frame
        );
        assert_eq!(restored.frames_compared, report.frames_compared);
        assert_eq!(restored.final_hash_match, report.final_hash_match);
        assert_eq!(
            restored.divergent_frame_count,
            report.divergent_frame_count
        );
    }
}
