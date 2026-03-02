use super::replay::Replay;

/// A detected anomaly in a state value series.
#[derive(Clone, Debug)]
pub struct Anomaly {
    /// The state key where the anomaly was found.
    pub key: String,
    /// Frame where the anomaly occurs.
    pub frame: usize,
    /// The type of anomaly.
    pub kind: AnomalyKind,
    /// Human-readable description.
    pub detail: String,
}

/// Types of detectable anomalies.
#[derive(Clone, Debug, PartialEq)]
pub enum AnomalyKind {
    /// Sudden large change between consecutive frames.
    Spike,
    /// Value remains constant for an unexpectedly long stretch.
    Plateau,
    /// Value goes outside expected bounds.
    OutOfBounds,
}

/// Configuration for anomaly detection.
pub struct AnomalyDetector {
    /// Threshold for spike detection: delta between consecutive frames.
    spike_threshold: f64,
    /// Minimum frames of constant value to flag as plateau.
    plateau_min_frames: usize,
    /// Optional bounds for out-of-bounds detection.
    bounds: Option<(f64, f64)>,
}

impl AnomalyDetector {
    /// Create a detector with default settings.
    pub fn new() -> Self {
        Self {
            spike_threshold: 100.0,
            plateau_min_frames: 30,
            bounds: None,
        }
    }

    /// Set spike detection threshold (max allowed frame-to-frame delta).
    pub fn with_spike_threshold(mut self, threshold: f64) -> Self {
        self.spike_threshold = threshold;
        self
    }

    /// Set minimum plateau duration in frames.
    pub fn with_plateau_min_frames(mut self, frames: usize) -> Self {
        self.plateau_min_frames = frames;
        self
    }

    /// Set expected value bounds for out-of-bounds detection.
    pub fn with_bounds(mut self, min: f64, max: f64) -> Self {
        self.bounds = Some((min, max));
        self
    }

    /// Scan a replay for anomalies in the specified keys.
    pub fn scan(&self, replay: &Replay, keys: &[&str]) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        for key in keys {
            let series = replay.series(key);
            if series.is_empty() {
                continue;
            }

            // Spike detection
            for i in 1..series.len() {
                let delta = (series[i] - series[i - 1]).abs();
                if delta > self.spike_threshold {
                    anomalies.push(Anomaly {
                        key: key.to_string(),
                        frame: i,
                        kind: AnomalyKind::Spike,
                        detail: format!(
                            "Spike in '{}' at frame {}: delta={:.2} (threshold={:.2}), {} -> {}",
                            key, i, delta, self.spike_threshold, series[i - 1], series[i]
                        ),
                    });
                }
            }

            // Plateau detection
            let mut plateau_start = 0;
            for i in 1..series.len() {
                if (series[i] - series[plateau_start]).abs() > f64::EPSILON {
                    let run_len = i - plateau_start;
                    if run_len >= self.plateau_min_frames {
                        anomalies.push(Anomaly {
                            key: key.to_string(),
                            frame: plateau_start,
                            kind: AnomalyKind::Plateau,
                            detail: format!(
                                "Plateau in '{}' at frames {}..{}: value={:.2} for {} frames",
                                key, plateau_start, i, series[plateau_start], run_len
                            ),
                        });
                    }
                    plateau_start = i;
                }
            }
            // Check final run
            let final_len = series.len() - plateau_start;
            if final_len >= self.plateau_min_frames {
                anomalies.push(Anomaly {
                    key: key.to_string(),
                    frame: plateau_start,
                    kind: AnomalyKind::Plateau,
                    detail: format!(
                        "Plateau in '{}' at frames {}..{}: value={:.2} for {} frames",
                        key, plateau_start, series.len(), series[plateau_start], final_len
                    ),
                });
            }

            // Out-of-bounds detection
            if let Some((min, max)) = self.bounds {
                for (i, &v) in series.iter().enumerate() {
                    if v < min || v > max {
                        anomalies.push(Anomaly {
                            key: key.to_string(),
                            frame: i,
                            kind: AnomalyKind::OutOfBounds,
                            detail: format!(
                                "Out of bounds in '{}' at frame {}: value={:.2} (expected [{:.2}, {:.2}])",
                                key, i, v, min, max
                            ),
                        });
                    }
                }
            }
        }

        anomalies
    }

    /// Scan and produce a summary report.
    pub fn report(&self, replay: &Replay, keys: &[&str]) -> String {
        let anomalies = self.scan(replay, keys);
        if anomalies.is_empty() {
            return format!("Anomaly scan: {} keys, 0 anomalies found", keys.len());
        }

        let mut out = format!("Anomaly scan: {} anomalies found\n", anomalies.len());
        for a in &anomalies {
            out.push_str(&format!("  [{:?}] {}\n", a.kind, a.detail));
        }
        out
    }
}
