/// Lightweight per-frame performance telemetry.
///
/// This is a structural seam for observability — not a full profiler.
/// The engine updates these values each tick so that external tooling
/// (dev-tools overlay, WASM host, etc.) can read them cheaply.

#[derive(Clone, Debug)]
pub struct FrameMetrics {
    /// Wall-clock frame time in milliseconds (clamped dt passed to tick).
    pub frame_time_ms: f64,
    /// Time spent in the fixed-step physics loop, in milliseconds.
    pub physics_time_ms: f64,
    /// Number of live entities at the end of the frame.
    pub entity_count: usize,
    /// Monotonically increasing frame counter.
    pub frame_number: u64,
}

impl Default for FrameMetrics {
    fn default() -> Self {
        Self {
            frame_time_ms: 0.0,
            physics_time_ms: 0.0,
            entity_count: 0,
            frame_number: 0,
        }
    }
}

impl FrameMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    /// Serialize to a minimal JSON string without pulling in serde.
    pub fn to_json(&self) -> String {
        format!(
            "{{\"frame_time_ms\":{:.3},\"physics_time_ms\":{:.3},\"entity_count\":{},\"frame_number\":{}}}",
            self.frame_time_ms,
            self.physics_time_ms,
            self.entity_count,
            self.frame_number,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_metrics_are_zeroed() {
        let m = FrameMetrics::new();
        assert_eq!(m.frame_time_ms, 0.0);
        assert_eq!(m.physics_time_ms, 0.0);
        assert_eq!(m.entity_count, 0);
        assert_eq!(m.frame_number, 0);
    }

    #[test]
    fn to_json_round_trips_values() {
        let m = FrameMetrics {
            frame_time_ms: 16.667,
            physics_time_ms: 4.123,
            entity_count: 42,
            frame_number: 100,
        };
        let json = m.to_json();
        assert!(json.contains("\"frame_time_ms\":16.667"));
        assert!(json.contains("\"physics_time_ms\":4.123"));
        assert!(json.contains("\"entity_count\":42"));
        assert!(json.contains("\"frame_number\":100"));
    }
}
