use super::SchemaInfo;
use crate::rendering::color::Color;

/// A single position snapshot in a ghost trail.
#[derive(Clone, Debug)]
pub struct GhostSnapshot {
    pub x: f64,
    pub y: f64,
    pub age: f64,
}

/// Ghost trail afterimage component. Captures position snapshots at intervals
/// and renders fading afterimages behind the entity.
#[derive(Clone, Debug)]
pub struct GhostTrail {
    pub snapshots: Vec<GhostSnapshot>,
    pub max_snapshots: usize,
    pub snapshot_interval: f64,
    pub timer: f64,
    pub fade_duration: f64,
    pub color: Color,
}

impl GhostTrail {
    pub fn new(max_snapshots: usize, snapshot_interval: f64, fade_duration: f64, color: Color) -> Self {
        Self {
            snapshots: Vec::new(),
            max_snapshots,
            snapshot_interval,
            timer: 0.0,
            fade_duration,
            color,
        }
    }

    /// Tick the trail timer and capture a snapshot if interval has elapsed.
    /// Also ages existing snapshots and removes expired ones.
    pub fn tick(&mut self, dt: f64, x: f64, y: f64) {
        // Age existing snapshots
        for snap in &mut self.snapshots {
            snap.age += dt;
        }

        // Remove expired snapshots
        self.snapshots.retain(|s| s.age < self.fade_duration);

        // Capture new snapshot if interval reached
        self.timer += dt;
        if self.timer >= self.snapshot_interval {
            self.timer -= self.snapshot_interval;
            if self.snapshots.len() < self.max_snapshots {
                self.snapshots.push(GhostSnapshot { x, y, age: 0.0 });
            } else if !self.snapshots.is_empty() {
                // Snapshots are kept in insertion order; index 0 is always the
                // oldest because every existing entry receives the same +dt and
                // new entries are appended with age 0.0.
                self.snapshots[0] = GhostSnapshot { x, y, age: 0.0 };
                // Rotate so the newly-reset entry moves to the back, restoring
                // chronological order (oldest first, newest last).
                self.snapshots.rotate_left(1);
            }
        }
    }

    /// Get the alpha for a snapshot based on its age.
    pub fn alpha_for_snapshot(&self, snapshot: &GhostSnapshot) -> u8 {
        if self.fade_duration <= 0.0 {
            return 0;
        }
        let t = (1.0 - snapshot.age / self.fade_duration).max(0.0).min(1.0);
        (t * self.color.a as f64) as u8
    }
}

impl SchemaInfo for GhostTrail {
    fn schema_name() -> &'static str { "GhostTrail" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "max_snapshots": { "type": "usize", "default": 8 },
                "snapshot_interval": { "type": "f64", "default": 0.05 },
                "fade_duration": { "type": "f64", "default": 0.5 },
                "color": { "type": "Color" }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_trail() {
        let trail = GhostTrail::new(8, 0.05, 0.5, Color::WHITE);
        assert!(trail.snapshots.is_empty());
        assert_eq!(trail.max_snapshots, 8);
        assert_eq!(trail.snapshot_interval, 0.05);
        assert_eq!(trail.fade_duration, 0.5);
    }

    #[test]
    fn tick_captures_snapshot_at_interval() {
        let mut trail = GhostTrail::new(8, 0.1, 1.0, Color::WHITE);
        trail.tick(0.1, 10.0, 20.0);
        assert_eq!(trail.snapshots.len(), 1);
        assert_eq!(trail.snapshots[0].x, 10.0);
        assert_eq!(trail.snapshots[0].y, 20.0);
    }

    #[test]
    fn tick_does_not_capture_before_interval() {
        let mut trail = GhostTrail::new(8, 0.1, 1.0, Color::WHITE);
        trail.tick(0.05, 10.0, 20.0);
        assert!(trail.snapshots.is_empty());
    }

    #[test]
    fn tick_ages_snapshots() {
        let mut trail = GhostTrail::new(8, 0.1, 1.0, Color::WHITE);
        trail.tick(0.1, 10.0, 20.0);
        trail.tick(0.1, 15.0, 25.0);
        assert!(trail.snapshots[0].age > 0.0);
    }

    #[test]
    fn tick_removes_expired_snapshots() {
        let mut trail = GhostTrail::new(8, 0.05, 0.2, Color::WHITE);
        trail.tick(0.05, 10.0, 20.0);
        assert_eq!(trail.snapshots.len(), 1);
        trail.tick(0.25, 15.0, 25.0); // ages first snapshot past fade_duration
        // First snapshot should be removed, second one captured
        assert!(trail.snapshots.iter().all(|s| s.age < trail.fade_duration));
    }

    #[test]
    fn max_snapshots_respected() {
        let mut trail = GhostTrail::new(3, 0.01, 10.0, Color::WHITE);
        for i in 0..10 {
            trail.tick(0.01, i as f64, 0.0);
        }
        assert!(trail.snapshots.len() <= 3);
    }

    #[test]
    fn alpha_for_snapshot_fresh() {
        let trail = GhostTrail::new(8, 0.05, 1.0, Color::from_rgba(255, 255, 255, 200));
        let snap = GhostSnapshot { x: 0.0, y: 0.0, age: 0.0 };
        assert_eq!(trail.alpha_for_snapshot(&snap), 200);
    }

    #[test]
    fn alpha_for_snapshot_half_aged() {
        let trail = GhostTrail::new(8, 0.05, 1.0, Color::from_rgba(255, 255, 255, 200));
        let snap = GhostSnapshot { x: 0.0, y: 0.0, age: 0.5 };
        assert_eq!(trail.alpha_for_snapshot(&snap), 100);
    }

    #[test]
    fn alpha_for_snapshot_expired() {
        let trail = GhostTrail::new(8, 0.05, 1.0, Color::WHITE);
        let snap = GhostSnapshot { x: 0.0, y: 0.0, age: 1.0 };
        assert_eq!(trail.alpha_for_snapshot(&snap), 0);
    }

    #[test]
    fn clone_and_debug() {
        let trail = GhostTrail::new(4, 0.1, 0.5, Color::RED);
        let cloned = trail.clone();
        let debug = format!("{:?}", cloned);
        assert!(debug.contains("GhostTrail"));
    }

    #[test]
    fn schema_info() {
        assert_eq!(GhostTrail::schema_name(), "GhostTrail");
        let schema = GhostTrail::schema();
        assert!(schema.get("fields").is_some());
    }
}
