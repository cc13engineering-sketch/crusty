use super::color::Color;
use super::framebuffer::Framebuffer;

/// Types of screen effects that can be applied.
#[derive(Clone, Debug)]
pub enum ScreenEffect {
    /// Full-screen color tint overlay.
    Tint { color: Color, intensity: f64 },
    /// Desaturate/grayscale (0.0 = full color, 1.0 = full grayscale).
    Desaturate { amount: f64 },
    /// Brief flash that fades out quadratically.
    Flash { color: Color, intensity: f64 },
}

/// A timed screen effect with duration tracking.
#[derive(Clone, Debug)]
pub struct TimedEffect {
    pub effect: ScreenEffect,
    pub duration: f64,
    pub elapsed: f64,
}

impl TimedEffect {
    /// Returns the time factor (1.0 at start, 0.0 at end).
    pub fn time_factor(&self) -> f64 {
        if self.duration <= 0.0 { return 0.0; }
        (1.0 - self.elapsed / self.duration).max(0.0).min(1.0)
    }

    /// Returns true if this effect has expired.
    pub fn is_expired(&self) -> bool {
        self.elapsed >= self.duration
    }
}

/// Composable stack of timed screen effects applied in order.
#[derive(Clone, Debug)]
pub struct ScreenFxStack {
    pub effects: Vec<TimedEffect>,
}

impl Default for ScreenFxStack {
    fn default() -> Self {
        Self { effects: Vec::new() }
    }
}

impl ScreenFxStack {
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a new timed effect onto the stack.
    pub fn push(&mut self, effect: ScreenEffect, duration: f64) {
        self.effects.push(TimedEffect { effect, duration, elapsed: 0.0 });
    }

    /// Advance all timers and remove expired effects.
    pub fn tick(&mut self, dt: f64) {
        for eff in &mut self.effects {
            eff.elapsed += dt;
        }
        self.effects.retain(|e| !e.is_expired());
    }

    /// Apply all active effects to the framebuffer.
    pub fn apply(&self, fb: &mut Framebuffer) {
        for timed in &self.effects {
            let tf = timed.time_factor();
            match &timed.effect {
                ScreenEffect::Tint { color, intensity } => {
                    let effective = intensity * tf;
                    if effective <= 0.0 { continue; }
                    apply_tint(fb, *color, effective);
                }
                ScreenEffect::Desaturate { amount } => {
                    let effective = amount * tf;
                    if effective <= 0.0 { continue; }
                    apply_desaturate(fb, effective);
                }
                ScreenEffect::Flash { color, intensity } => {
                    // Flash fades out quadratically
                    let effective = intensity * tf * tf;
                    if effective <= 0.0 { continue; }
                    apply_tint(fb, *color, effective);
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }

    pub fn clear(&mut self) {
        self.effects.clear();
    }
}

fn apply_tint(fb: &mut Framebuffer, color: Color, intensity: f64) {
    let intensity = intensity.max(0.0).min(1.0);
    let inv = 1.0 - intensity;
    let tr = color.r as f64;
    let tg = color.g as f64;
    let tb = color.b as f64;

    for chunk in fb.pixels.chunks_exact_mut(4) {
        chunk[0] = (chunk[0] as f64 * inv + tr * intensity) as u8;
        chunk[1] = (chunk[1] as f64 * inv + tg * intensity) as u8;
        chunk[2] = (chunk[2] as f64 * inv + tb * intensity) as u8;
    }
}

fn apply_desaturate(fb: &mut Framebuffer, amount: f64) {
    let amount = amount.max(0.0).min(1.0);
    let inv = 1.0 - amount;

    for chunk in fb.pixels.chunks_exact_mut(4) {
        let gray = 0.299 * chunk[0] as f64 + 0.587 * chunk[1] as f64 + 0.114 * chunk[2] as f64;
        chunk[0] = (chunk[0] as f64 * inv + gray * amount) as u8;
        chunk[1] = (chunk[1] as f64 * inv + gray * amount) as u8;
        chunk[2] = (chunk[2] as f64 * inv + gray * amount) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_empty() {
        let stack = ScreenFxStack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn push_adds_effect() {
        let mut stack = ScreenFxStack::new();
        stack.push(ScreenEffect::Tint { color: Color::RED, intensity: 0.5 }, 1.0);
        assert!(!stack.is_empty());
        assert_eq!(stack.effects.len(), 1);
    }

    #[test]
    fn tick_advances_elapsed() {
        let mut stack = ScreenFxStack::new();
        stack.push(ScreenEffect::Tint { color: Color::RED, intensity: 0.5 }, 2.0);
        stack.tick(0.5);
        assert!((stack.effects[0].elapsed - 0.5).abs() < 1e-10);
    }

    #[test]
    fn tick_removes_expired() {
        let mut stack = ScreenFxStack::new();
        stack.push(ScreenEffect::Tint { color: Color::RED, intensity: 0.5 }, 1.0);
        stack.tick(1.5);
        assert!(stack.is_empty());
    }

    #[test]
    fn clear_empties_stack() {
        let mut stack = ScreenFxStack::new();
        stack.push(ScreenEffect::Tint { color: Color::RED, intensity: 0.5 }, 1.0);
        stack.push(ScreenEffect::Flash { color: Color::WHITE, intensity: 1.0 }, 0.5);
        stack.clear();
        assert!(stack.is_empty());
    }

    #[test]
    fn time_factor_decreases() {
        let eff = TimedEffect {
            effect: ScreenEffect::Tint { color: Color::RED, intensity: 1.0 },
            duration: 1.0,
            elapsed: 0.0,
        };
        assert!((eff.time_factor() - 1.0).abs() < 1e-10);

        let eff2 = TimedEffect {
            effect: ScreenEffect::Tint { color: Color::RED, intensity: 1.0 },
            duration: 1.0,
            elapsed: 0.5,
        };
        assert!((eff2.time_factor() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn tint_modifies_pixels() {
        let mut fb = Framebuffer::new(2, 2);
        fb.clear(Color::BLACK);

        let stack = ScreenFxStack {
            effects: vec![TimedEffect {
                effect: ScreenEffect::Tint { color: Color::RED, intensity: 1.0 },
                duration: 1.0,
                elapsed: 0.0,
            }],
        };
        stack.apply(&mut fb);

        // Should be fully red
        assert_eq!(fb.pixels[0], 255);
        assert_eq!(fb.pixels[1], 0);
        assert_eq!(fb.pixels[2], 0);
    }

    #[test]
    fn desaturate_makes_gray() {
        let mut fb = Framebuffer::new(2, 2);
        fb.clear(Color::RED);

        let stack = ScreenFxStack {
            effects: vec![TimedEffect {
                effect: ScreenEffect::Desaturate { amount: 1.0 },
                duration: 1.0,
                elapsed: 0.0,
            }],
        };
        stack.apply(&mut fb);

        // Red (255,0,0) desaturated should give gray = 0.299*255 ≈ 76
        let gray = (0.299 * 255.0) as u8;
        assert!((fb.pixels[0] as i32 - gray as i32).abs() <= 1);
        assert!((fb.pixels[1] as i32 - gray as i32).abs() <= 1);
        assert!((fb.pixels[2] as i32 - gray as i32).abs() <= 1);
    }

    #[test]
    fn flash_fades_quadratically() {
        let eff = TimedEffect {
            effect: ScreenEffect::Flash { color: Color::WHITE, intensity: 1.0 },
            duration: 1.0,
            elapsed: 0.5,
        };
        // time_factor = 0.5, effective = 1.0 * 0.5 * 0.5 = 0.25
        let tf = eff.time_factor();
        assert!((tf - 0.5).abs() < 1e-10);
    }

    #[test]
    fn multiple_effects_stack() {
        let mut stack = ScreenFxStack::new();
        stack.push(ScreenEffect::Tint { color: Color::RED, intensity: 0.5 }, 2.0);
        stack.push(ScreenEffect::Desaturate { amount: 0.3 }, 1.0);
        assert_eq!(stack.effects.len(), 2);

        stack.tick(1.5);
        // Desaturate (1.0 duration) should have expired
        assert_eq!(stack.effects.len(), 1);
    }

    #[test]
    fn clone_and_debug() {
        let mut stack = ScreenFxStack::new();
        stack.push(ScreenEffect::Tint { color: Color::RED, intensity: 0.5 }, 1.0);
        let cloned = stack.clone();
        let debug = format!("{:?}", cloned);
        assert!(debug.contains("Tint"));
    }
}
