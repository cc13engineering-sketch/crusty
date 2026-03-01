use super::SchemaInfo;
use crate::rendering::color::Color;

/// Flash effect mode.
#[derive(Clone, Debug)]
pub enum FlashMode {
    /// Solid color overlay for a duration.
    HitFlash { color: Color, duration: f64, elapsed: f64 },
    /// Toggles visibility at a rate.
    Blink { on_time: f64, off_time: f64, remaining: f64, timer: f64, is_on: bool },
    /// Pulsing color overlay (sinusoidal).
    ColorPulse { color: Color, frequency: f64, elapsed: f64, duration: f64 },
}

/// Lightweight visual effect component for hit flashes, blinks, and color pulses.
#[derive(Clone, Debug)]
pub struct EntityFlash {
    pub mode: FlashMode,
}

impl EntityFlash {
    pub fn hit_flash(color: Color, duration: f64) -> Self {
        Self {
            mode: FlashMode::HitFlash { color, duration, elapsed: 0.0 },
        }
    }

    pub fn blink(on_time: f64, off_time: f64, total_duration: f64) -> Self {
        Self {
            mode: FlashMode::Blink {
                on_time, off_time, remaining: total_duration, timer: 0.0, is_on: true,
            },
        }
    }

    pub fn color_pulse(color: Color, frequency: f64, duration: f64) -> Self {
        Self {
            mode: FlashMode::ColorPulse { color, frequency, elapsed: 0.0, duration },
        }
    }

    /// Returns true if this flash effect has expired.
    pub fn is_expired(&self) -> bool {
        match &self.mode {
            FlashMode::HitFlash { duration, elapsed, .. } => *elapsed >= *duration,
            FlashMode::Blink { remaining, .. } => *remaining <= 0.0,
            FlashMode::ColorPulse { elapsed, duration, .. } => *elapsed >= *duration,
        }
    }

    /// Advance the flash timer by dt. Returns true if expired.
    pub fn tick(&mut self, dt: f64) -> bool {
        match &mut self.mode {
            FlashMode::HitFlash { elapsed, .. } => {
                *elapsed += dt;
            }
            FlashMode::Blink { on_time, off_time, remaining, timer, is_on } => {
                *remaining -= dt;
                *timer += dt;
                let threshold = if *is_on { *on_time } else { *off_time };
                if *timer >= threshold {
                    *timer -= threshold;
                    *is_on = !*is_on;
                }
            }
            FlashMode::ColorPulse { elapsed, .. } => {
                *elapsed += dt;
            }
        }
        self.is_expired()
    }

    /// Returns the flash intensity (0.0..=1.0) for color mixing.
    pub fn intensity(&self) -> f64 {
        match &self.mode {
            FlashMode::HitFlash { duration, elapsed, .. } => {
                if *duration <= 0.0 { return 0.0; }
                (1.0 - *elapsed / *duration).max(0.0)
            }
            FlashMode::Blink { is_on, .. } => if *is_on { 0.0 } else { 1.0 },
            FlashMode::ColorPulse { frequency, elapsed, .. } => {
                let phase = *elapsed * *frequency * 2.0 * std::f64::consts::PI;
                (phase.sin() * 0.5 + 0.5).max(0.0).min(1.0)
            }
        }
    }
}

impl SchemaInfo for EntityFlash {
    fn schema_name() -> &'static str { "EntityFlash" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "mode": {
                    "type": "enum",
                    "variants": ["HitFlash", "Blink", "ColorPulse"]
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_flash_creates_correctly() {
        let f = EntityFlash::hit_flash(Color::WHITE, 0.5);
        match &f.mode {
            FlashMode::HitFlash { duration, elapsed, .. } => {
                assert_eq!(*duration, 0.5);
                assert_eq!(*elapsed, 0.0);
            }
            _ => panic!("Expected HitFlash"),
        }
    }

    #[test]
    fn blink_creates_correctly() {
        let f = EntityFlash::blink(0.1, 0.1, 2.0);
        match &f.mode {
            FlashMode::Blink { on_time, off_time, remaining, is_on, .. } => {
                assert_eq!(*on_time, 0.1);
                assert_eq!(*off_time, 0.1);
                assert_eq!(*remaining, 2.0);
                assert!(*is_on);
            }
            _ => panic!("Expected Blink"),
        }
    }

    #[test]
    fn color_pulse_creates_correctly() {
        let f = EntityFlash::color_pulse(Color::RED, 2.0, 3.0);
        match &f.mode {
            FlashMode::ColorPulse { frequency, duration, elapsed, .. } => {
                assert_eq!(*frequency, 2.0);
                assert_eq!(*duration, 3.0);
                assert_eq!(*elapsed, 0.0);
            }
            _ => panic!("Expected ColorPulse"),
        }
    }

    #[test]
    fn hit_flash_not_expired_initially() {
        let f = EntityFlash::hit_flash(Color::WHITE, 1.0);
        assert!(!f.is_expired());
    }

    #[test]
    fn hit_flash_expires_after_duration() {
        let mut f = EntityFlash::hit_flash(Color::WHITE, 0.5);
        f.tick(0.5);
        assert!(f.is_expired());
    }

    #[test]
    fn hit_flash_intensity_decreases() {
        let mut f = EntityFlash::hit_flash(Color::WHITE, 1.0);
        let i0 = f.intensity();
        f.tick(0.5);
        let i1 = f.intensity();
        assert!(i0 > i1);
    }

    #[test]
    fn blink_toggles_visibility() {
        let mut f = EntityFlash::blink(0.1, 0.1, 2.0);
        match &f.mode {
            FlashMode::Blink { is_on, .. } => assert!(*is_on),
            _ => panic!(),
        }
        f.tick(0.15); // past on_time
        match &f.mode {
            FlashMode::Blink { is_on, .. } => assert!(!*is_on),
            _ => panic!(),
        }
    }

    #[test]
    fn blink_expires() {
        let mut f = EntityFlash::blink(0.1, 0.1, 0.3);
        assert!(!f.is_expired());
        f.tick(0.35);
        assert!(f.is_expired());
    }

    #[test]
    fn color_pulse_intensity_oscillates() {
        let f = EntityFlash::color_pulse(Color::RED, 1.0, 5.0);
        // At elapsed=0, sin(0)=0, intensity=0.5
        let i = f.intensity();
        assert!((i - 0.5).abs() < 0.01);
    }

    #[test]
    fn tick_returns_true_when_expired() {
        let mut f = EntityFlash::hit_flash(Color::WHITE, 0.1);
        assert!(!f.tick(0.05));
        assert!(f.tick(0.06));
    }

    #[test]
    fn clone_and_debug() {
        let f = EntityFlash::hit_flash(Color::RED, 1.0);
        let cloned = f.clone();
        let debug = format!("{:?}", cloned);
        assert!(debug.contains("HitFlash"));
    }

    #[test]
    fn schema_info() {
        assert_eq!(EntityFlash::schema_name(), "EntityFlash");
        let schema = EntityFlash::schema();
        assert!(schema.get("fields").is_some());
    }
}
