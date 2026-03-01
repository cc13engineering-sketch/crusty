use super::SchemaInfo;

/// Tracks the remaining lifetime of an entity. When `remaining` reaches 0,
/// the entity is despawned by the lifecycle system.
#[derive(Clone, Debug)]
pub struct Lifetime {
    /// Total lifetime in seconds (for reference / resetting).
    pub duration: f64,
    /// Seconds remaining before auto-despawn.
    pub remaining: f64,
}

impl Lifetime {
    /// Create a new Lifetime that will expire after `seconds`.
    pub fn new(seconds: f64) -> Self {
        Self { duration: seconds, remaining: seconds }
    }

    /// Tick down by `dt` seconds. Returns true if expired (remaining <= 0).
    pub fn tick(&mut self, dt: f64) -> bool {
        self.remaining -= dt;
        self.remaining <= 0.0
    }

    /// Fraction of lifetime elapsed, clamped to [0, 1].
    pub fn fraction_elapsed(&self) -> f64 {
        if self.duration <= 0.0 { return 1.0; }
        ((self.duration - self.remaining) / self.duration).clamp(0.0, 1.0)
    }

    /// Fraction of lifetime remaining, clamped to [0, 1].
    pub fn fraction_remaining(&self) -> f64 {
        if self.duration <= 0.0 { return 0.0; }
        (self.remaining / self.duration).clamp(0.0, 1.0)
    }
}

impl SchemaInfo for Lifetime {
    fn schema_name() -> &'static str { "Lifetime" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "duration": { "type": "f64", "note": "total lifetime in seconds" },
                "remaining": { "type": "f64", "note": "seconds remaining" }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_duration_and_remaining() {
        let lt = Lifetime::new(5.0);
        assert_eq!(lt.duration, 5.0);
        assert_eq!(lt.remaining, 5.0);
    }

    #[test]
    fn tick_decreases_remaining() {
        let mut lt = Lifetime::new(3.0);
        assert!(!lt.tick(1.0));
        assert_eq!(lt.remaining, 2.0);
    }

    #[test]
    fn tick_returns_true_when_expired() {
        let mut lt = Lifetime::new(1.0);
        assert!(!lt.tick(0.5));
        assert!(lt.tick(0.5));
    }

    #[test]
    fn tick_returns_true_when_over_expired() {
        let mut lt = Lifetime::new(1.0);
        assert!(lt.tick(2.0));
    }

    #[test]
    fn fraction_elapsed_at_start() {
        let lt = Lifetime::new(4.0);
        assert_eq!(lt.fraction_elapsed(), 0.0);
    }

    #[test]
    fn fraction_elapsed_halfway() {
        let mut lt = Lifetime::new(4.0);
        lt.tick(2.0);
        assert!((lt.fraction_elapsed() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn fraction_remaining_at_start() {
        let lt = Lifetime::new(4.0);
        assert_eq!(lt.fraction_remaining(), 1.0);
    }

    #[test]
    fn fraction_remaining_at_end() {
        let mut lt = Lifetime::new(2.0);
        lt.tick(2.0);
        assert_eq!(lt.fraction_remaining(), 0.0);
    }

    #[test]
    fn fraction_clamps_beyond_bounds() {
        let mut lt = Lifetime::new(1.0);
        lt.tick(5.0);
        assert_eq!(lt.fraction_elapsed(), 1.0);
        assert_eq!(lt.fraction_remaining(), 0.0);
    }

    #[test]
    fn zero_duration_lifetime() {
        let lt = Lifetime::new(0.0);
        assert_eq!(lt.fraction_elapsed(), 1.0);
        assert_eq!(lt.fraction_remaining(), 0.0);
    }

    #[test]
    fn schema_name_returns_lifetime() {
        assert_eq!(Lifetime::schema_name(), "Lifetime");
    }
}
