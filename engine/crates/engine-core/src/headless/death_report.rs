/// ENGINE MODULE: DeathReport
/// Batch classification and reporting for terminal states across multiple
/// simulation runs. Aggregates individual DeathClassification results into
/// summary statistics.

use super::death_classify::{DeathClass, DeathClassification, ClassifierConfig, classify};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

// ─── Per-Run Classification ─────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunClassification {
    pub seed: u64,
    pub classification: DeathClassification,
}

// ─── Aggregate Report ───────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeathReport {
    pub total_runs: usize,
    pub classified_runs: usize,
    pub breakdown: BTreeMap<String, usize>,
    pub classifications: Vec<RunClassification>,
}

impl DeathReport {
    /// Return percentages for each death class as a fraction of total runs.
    pub fn percentages(&self) -> BTreeMap<String, f64> {
        let mut pcts = BTreeMap::new();
        if self.total_runs == 0 {
            return pcts;
        }
        for (class, &count) in &self.breakdown {
            pcts.insert(class.clone(), count as f64 / self.total_runs as f64 * 100.0);
        }
        pcts
    }

    /// Generate a human-readable summary string.
    pub fn summary(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "Death Report: {} total runs, {} classified",
            self.total_runs, self.classified_runs
        ));

        let pcts = self.percentages();
        // Sort by count descending for readability
        let mut entries: Vec<_> = self.breakdown.iter().collect();
        entries.sort_by(|a, b| b.1.cmp(a.1));

        for (class, count) in entries {
            let pct = pcts.get(class).copied().unwrap_or(0.0);
            lines.push(format!("  {}: {} ({:.1}%)", class, count, pct));
        }

        lines.join("\n")
    }
}

// ─── Batch Classification ───────────────────────────────────────────

/// Classify a batch of runs. Each run is a (seed, metric_values) pair.
pub fn classify_batch(runs: &[(u64, Vec<f64>)], config: &ClassifierConfig) -> DeathReport {
    let mut breakdown: BTreeMap<String, usize> = BTreeMap::new();
    let mut classifications = Vec::with_capacity(runs.len());
    let mut classified_runs = 0usize;

    for &(seed, ref series) in runs {
        let classification = classify(series, config);
        let label = classification.class.label().to_string();

        if classification.class != DeathClass::Unclassified {
            classified_runs += 1;
        }

        *breakdown.entry(label).or_insert(0) += 1;
        classifications.push(RunClassification { seed, classification });
    }

    DeathReport {
        total_runs: runs.len(),
        classified_runs,
        breakdown,
        classifications,
    }
}

// ─── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cliff_series() -> Vec<f64> {
        let mut s: Vec<f64> = vec![100.0; 100];
        s.push(0.0);
        s
    }

    fn make_blowout_series() -> Vec<f64> {
        (0..61).map(|i| 100.0 - (100.0 / 60.0) * i as f64).collect()
    }

    #[test]
    fn classify_batch_basic() {
        let runs = vec![
            (1, make_cliff_series()),
            (2, make_cliff_series()),
            (3, make_blowout_series()),
        ];
        let config = ClassifierConfig::default().with_window_size(120);
        let report = classify_batch(&runs, &config);

        assert_eq!(report.total_runs, 3);
        assert_eq!(report.classified_runs, 3);
        assert_eq!(report.breakdown.get("cliff"), Some(&2));
        assert_eq!(report.breakdown.get("blowout"), Some(&1));
    }

    #[test]
    fn percentages_computation() {
        let runs = vec![
            (1, make_cliff_series()),
            (2, make_blowout_series()),
        ];
        let config = ClassifierConfig::default().with_window_size(120);
        let report = classify_batch(&runs, &config);
        let pcts = report.percentages();

        assert!((pcts["cliff"] - 50.0).abs() < 1e-10);
        assert!((pcts["blowout"] - 50.0).abs() < 1e-10);
    }

    #[test]
    fn summary_output() {
        let runs = vec![
            (42, make_cliff_series()),
        ];
        let config = ClassifierConfig::default().with_window_size(120);
        let report = classify_batch(&runs, &config);
        let summary = report.summary();

        assert!(summary.contains("Death Report"));
        assert!(summary.contains("1 total runs"));
        assert!(summary.contains("cliff"));
    }

    #[test]
    fn empty_batch() {
        let runs: Vec<(u64, Vec<f64>)> = vec![];
        let config = ClassifierConfig::default();
        let report = classify_batch(&runs, &config);

        assert_eq!(report.total_runs, 0);
        assert_eq!(report.classified_runs, 0);
        assert!(report.breakdown.is_empty());
        assert!(report.percentages().is_empty());
    }

    #[test]
    fn unclassified_runs_not_counted() {
        let runs = vec![
            (1, vec![50.0, 50.0]),  // too few frames -> Unclassified
        ];
        let config = ClassifierConfig::default();
        let report = classify_batch(&runs, &config);

        assert_eq!(report.total_runs, 1);
        assert_eq!(report.classified_runs, 0);
        assert_eq!(report.breakdown.get("unclassified"), Some(&1));
    }
}
