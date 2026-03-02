use super::replay::Replay;

/// Side-by-side comparison of two simulation replays.
///
/// Produces structured diffs of state trajectories, identifies divergence
/// points, and generates human/AI-readable reports.
#[derive(Clone, Debug)]
pub struct Comparison {
    pub name_a: String,
    pub name_b: String,
    /// Per-key comparison results.
    pub diffs: Vec<KeyDiff>,
    /// Frame where framebuffers first diverge (None if always identical or always different).
    pub first_visual_divergence: Option<usize>,
}

/// Comparison result for a single state key across two replays.
#[derive(Clone, Debug)]
pub struct KeyDiff {
    pub key: String,
    /// Absolute max difference across all frames.
    pub max_delta: f64,
    /// Frame where max difference occurs.
    pub max_delta_frame: usize,
    /// Mean absolute difference across all frames.
    pub mean_delta: f64,
    /// Frame where values first diverge (delta > tolerance).
    pub first_divergence: Option<usize>,
    /// Series of per-frame deltas (a[i] - b[i]).
    pub deltas: Vec<f64>,
}

impl KeyDiff {
    /// True if this key was identical (within tolerance) across all frames.
    pub fn is_identical(&self, tolerance: f64) -> bool {
        self.max_delta <= tolerance
    }
}

impl Comparison {
    /// True if all tracked keys are identical within tolerance.
    pub fn is_identical(&self, tolerance: f64) -> bool {
        self.diffs.iter().all(|d| d.is_identical(tolerance))
    }

    /// Get the diff for a specific key.
    pub fn key_diff(&self, key: &str) -> Option<&KeyDiff> {
        self.diffs.iter().find(|d| d.key == key)
    }

    /// Machine-readable summary for AI consumption.
    pub fn summary(&self) -> String {
        let mut out = format!("Compare: {} vs {}\n", self.name_a, self.name_b);

        if let Some(frame) = self.first_visual_divergence {
            out.push_str(&format!("  Visual divergence at frame {}\n", frame));
        }

        for diff in &self.diffs {
            let status = if diff.is_identical(0.01) { "identical" } else { "differs" };
            out.push_str(&format!(
                "  {}: {} (max_delta={:.4} at frame {}, mean={:.4})\n",
                diff.key, status, diff.max_delta, diff.max_delta_frame, diff.mean_delta
            ));
            if let Some(frame) = diff.first_divergence {
                out.push_str(&format!("    first divergence at frame {}\n", frame));
            }
        }

        out
    }
}

/// Compare two replays side-by-side across the specified keys.
///
/// Both replays must track the same keys. Comparison runs across the
/// minimum frame count of the two replays.
pub fn compare_replays(a: &Replay, b: &Replay, keys: &[&str], tolerance: f64) -> Comparison {
    let frame_count = a.len().min(b.len());
    let mut diffs = Vec::new();

    for key in keys {
        let series_a = a.series(key);
        let series_b = b.series(key);

        let mut deltas = Vec::with_capacity(frame_count);
        let mut max_delta = 0.0f64;
        let mut max_delta_frame = 0;
        let mut sum_delta = 0.0;
        let mut first_divergence = None;

        for i in 0..frame_count {
            let va = series_a.get(i).copied().unwrap_or(0.0);
            let vb = series_b.get(i).copied().unwrap_or(0.0);
            let delta = (va - vb).abs();
            deltas.push(va - vb);
            sum_delta += delta;

            if delta > max_delta {
                max_delta = delta;
                max_delta_frame = i;
            }

            if delta > tolerance && first_divergence.is_none() {
                first_divergence = Some(i);
            }
        }

        diffs.push(KeyDiff {
            key: key.to_string(),
            max_delta,
            max_delta_frame,
            mean_delta: if frame_count > 0 { sum_delta / frame_count as f64 } else { 0.0 },
            first_divergence,
            deltas,
        });
    }

    // Visual divergence: first frame where framebuffer hashes differ
    let first_visual_divergence = (0..frame_count).find(|&i| {
        let ha = a.at(i).map(|f| f.fb_hash).unwrap_or(0);
        let hb = b.at(i).map(|f| f.fb_hash).unwrap_or(0);
        ha != hb
    });

    Comparison {
        name_a: a.name.clone(),
        name_b: b.name.clone(),
        diffs,
        first_visual_divergence,
    }
}
