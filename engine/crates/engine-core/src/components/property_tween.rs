use super::SchemaInfo;

/// Easing functions for tween interpolation.
/// Each variant maps to a pure math function on a normalized t value (0.0..=1.0).
#[derive(Clone, Debug, PartialEq)]
pub enum EasingFn {
    Linear,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    BounceOut,
    ElasticOut,
}

/// Which property on an entity to animate.
#[derive(Clone, Debug, PartialEq)]
pub enum TweenTarget {
    X,
    Y,
    Rotation,
    Scale,
    VelocityX,
    VelocityY,
    Alpha,
}

/// A single active tween that interpolates a target property over time.
#[derive(Clone, Debug)]
pub struct Tween {
    pub target: TweenTarget,
    pub from: f64,
    pub to: f64,
    pub duration: f64,
    pub elapsed: f64,
    pub easing: EasingFn,
    pub looping: bool,
    pub ping_pong: bool,
    pub forward: bool,
}

impl Tween {
    /// Create a new tween with sensible defaults.
    pub fn new(target: TweenTarget, from: f64, to: f64, duration: f64, easing: EasingFn) -> Self {
        Self {
            target,
            from,
            to,
            duration,
            elapsed: 0.0,
            easing,
            looping: false,
            ping_pong: false,
            forward: true,
        }
    }

    /// Returns the normalized progress (0.0..=1.0) accounting for ping-pong direction.
    pub fn normalized_t(&self) -> f64 {
        if self.duration <= 0.0 {
            return 1.0;
        }
        let raw = (self.elapsed / self.duration).min(1.0).max(0.0);
        if self.ping_pong && !self.forward {
            1.0 - raw
        } else {
            raw
        }
    }

    /// Returns the current eased value.
    pub fn current_value(&self) -> f64 {
        let t = self.normalized_t();
        let eased = apply_easing(&self.easing, t);
        self.from + (self.to - self.from) * eased
    }

    /// Returns true if this tween has completed (elapsed >= duration and not looping).
    pub fn is_complete(&self) -> bool {
        self.elapsed >= self.duration && !self.looping
    }
}

/// A collection of active tweens on an entity.
#[derive(Clone, Debug)]
pub struct PropertyTween {
    pub tweens: Vec<Tween>,
}

impl PropertyTween {
    pub fn new() -> Self {
        Self { tweens: Vec::new() }
    }

    pub fn with_tween(mut self, tween: Tween) -> Self {
        self.tweens.push(tween);
        self
    }
}

impl SchemaInfo for PropertyTween {
    fn schema_name() -> &'static str {
        "PropertyTween"
    }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "tweens": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "target": { "type": "enum", "variants": ["X", "Y", "Rotation", "Scale", "VelocityX", "VelocityY", "Alpha"] },
                            "from": { "type": "f64" },
                            "to": { "type": "f64" },
                            "duration": { "type": "f64" },
                            "elapsed": { "type": "f64", "default": 0.0 },
                            "easing": { "type": "enum", "variants": ["Linear", "QuadIn", "QuadOut", "QuadInOut", "CubicIn", "CubicOut", "CubicInOut", "BounceOut", "ElasticOut"] },
                            "looping": { "type": "bool", "default": false },
                            "ping_pong": { "type": "bool", "default": false },
                            "forward": { "type": "bool", "default": true }
                        }
                    }
                }
            }
        })
    }
}

/// Apply an easing function to a normalized t value (0.0..=1.0).
/// Returns the eased value, also in range 0.0..=1.0 (approximately, for elastic/bounce).
pub fn apply_easing(easing: &EasingFn, t: f64) -> f64 {
    let t = t.max(0.0).min(1.0);
    match easing {
        EasingFn::Linear => t,
        EasingFn::QuadIn => t * t,
        EasingFn::QuadOut => t * (2.0 - t),
        EasingFn::QuadInOut => {
            if t < 0.5 {
                2.0 * t * t
            } else {
                -1.0 + (4.0 - 2.0 * t) * t
            }
        }
        EasingFn::CubicIn => t * t * t,
        EasingFn::CubicOut => {
            let u = t - 1.0;
            u * u * u + 1.0
        }
        EasingFn::CubicInOut => {
            if t < 0.5 {
                4.0 * t * t * t
            } else {
                let u = 2.0 * t - 2.0;
                0.5 * u * u * u + 1.0
            }
        }
        EasingFn::BounceOut => bounce_out(t),
        EasingFn::ElasticOut => elastic_out(t),
    }
}

fn bounce_out(t: f64) -> f64 {
    if t < 1.0 / 2.75 {
        7.5625 * t * t
    } else if t < 2.0 / 2.75 {
        let t = t - 1.5 / 2.75;
        7.5625 * t * t + 0.75
    } else if t < 2.5 / 2.75 {
        let t = t - 2.25 / 2.75;
        7.5625 * t * t + 0.9375
    } else {
        let t = t - 2.625 / 2.75;
        7.5625 * t * t + 0.984375
    }
}

fn elastic_out(t: f64) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }
    let p = 0.3;
    let s = p / 4.0;
    2.0_f64.powf(-10.0 * t) * ((t - s) * (2.0 * std::f64::consts::PI) / p).sin() + 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_easing_boundaries() {
        assert!((apply_easing(&EasingFn::Linear, 0.0) - 0.0).abs() < 1e-10);
        assert!((apply_easing(&EasingFn::Linear, 1.0) - 1.0).abs() < 1e-10);
        assert!((apply_easing(&EasingFn::Linear, 0.5) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_quad_in_easing() {
        assert!((apply_easing(&EasingFn::QuadIn, 0.0) - 0.0).abs() < 1e-10);
        assert!((apply_easing(&EasingFn::QuadIn, 1.0) - 1.0).abs() < 1e-10);
        assert!((apply_easing(&EasingFn::QuadIn, 0.5) - 0.25).abs() < 1e-10);
        // QuadIn should be slower at the start
        assert!(apply_easing(&EasingFn::QuadIn, 0.25) < 0.25);
    }

    #[test]
    fn test_quad_out_easing() {
        assert!((apply_easing(&EasingFn::QuadOut, 0.0) - 0.0).abs() < 1e-10);
        assert!((apply_easing(&EasingFn::QuadOut, 1.0) - 1.0).abs() < 1e-10);
        // QuadOut should be faster at the start
        assert!(apply_easing(&EasingFn::QuadOut, 0.25) > 0.25);
    }

    #[test]
    fn test_quad_in_out_easing() {
        assert!((apply_easing(&EasingFn::QuadInOut, 0.0) - 0.0).abs() < 1e-10);
        assert!((apply_easing(&EasingFn::QuadInOut, 1.0) - 1.0).abs() < 1e-10);
        assert!((apply_easing(&EasingFn::QuadInOut, 0.5) - 0.5).abs() < 1e-10);
        // First half should be slow (under linear)
        assert!(apply_easing(&EasingFn::QuadInOut, 0.25) < 0.25);
        // Second half should be fast (over linear)
        assert!(apply_easing(&EasingFn::QuadInOut, 0.75) > 0.75);
    }

    #[test]
    fn test_cubic_in_easing() {
        assert!((apply_easing(&EasingFn::CubicIn, 0.0) - 0.0).abs() < 1e-10);
        assert!((apply_easing(&EasingFn::CubicIn, 1.0) - 1.0).abs() < 1e-10);
        assert!((apply_easing(&EasingFn::CubicIn, 0.5) - 0.125).abs() < 1e-10);
    }

    #[test]
    fn test_cubic_out_easing() {
        assert!((apply_easing(&EasingFn::CubicOut, 0.0) - 0.0).abs() < 1e-10);
        assert!((apply_easing(&EasingFn::CubicOut, 1.0) - 1.0).abs() < 1e-10);
        // CubicOut should be faster at start than linear
        assert!(apply_easing(&EasingFn::CubicOut, 0.5) > 0.5);
    }

    #[test]
    fn test_cubic_in_out_easing() {
        assert!((apply_easing(&EasingFn::CubicInOut, 0.0) - 0.0).abs() < 1e-10);
        assert!((apply_easing(&EasingFn::CubicInOut, 1.0) - 1.0).abs() < 1e-10);
        assert!((apply_easing(&EasingFn::CubicInOut, 0.5) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_bounce_out_easing() {
        assert!((apply_easing(&EasingFn::BounceOut, 0.0) - 0.0).abs() < 1e-10);
        assert!((apply_easing(&EasingFn::BounceOut, 1.0) - 1.0).abs() < 1e-10);
        // Bounce should always be >= 0
        for i in 0..=100 {
            let t = i as f64 / 100.0;
            assert!(apply_easing(&EasingFn::BounceOut, t) >= 0.0);
        }
    }

    #[test]
    fn test_elastic_out_easing() {
        assert!((apply_easing(&EasingFn::ElasticOut, 0.0) - 0.0).abs() < 1e-10);
        assert!((apply_easing(&EasingFn::ElasticOut, 1.0) - 1.0).abs() < 1e-10);
        // Elastic should overshoot slightly then settle
        let mid = apply_easing(&EasingFn::ElasticOut, 0.3);
        assert!(mid > 0.0, "Elastic should have started moving by t=0.3");
    }

    #[test]
    fn test_easing_clamps_input() {
        // Values outside 0..1 should be clamped
        assert!((apply_easing(&EasingFn::Linear, -0.5) - 0.0).abs() < 1e-10);
        assert!((apply_easing(&EasingFn::Linear, 1.5) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tween_new() {
        let tw = Tween::new(TweenTarget::X, 0.0, 100.0, 2.0, EasingFn::Linear);
        assert_eq!(tw.target, TweenTarget::X);
        assert!((tw.from - 0.0).abs() < 1e-10);
        assert!((tw.to - 100.0).abs() < 1e-10);
        assert!((tw.duration - 2.0).abs() < 1e-10);
        assert!((tw.elapsed - 0.0).abs() < 1e-10);
        assert!(!tw.looping);
        assert!(!tw.ping_pong);
        assert!(tw.forward);
    }

    #[test]
    fn test_tween_current_value_linear() {
        let mut tw = Tween::new(TweenTarget::Y, 10.0, 50.0, 1.0, EasingFn::Linear);
        assert!((tw.current_value() - 10.0).abs() < 1e-10);

        tw.elapsed = 0.5;
        assert!((tw.current_value() - 30.0).abs() < 1e-10);

        tw.elapsed = 1.0;
        assert!((tw.current_value() - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_tween_current_value_quad_in() {
        let mut tw = Tween::new(TweenTarget::Scale, 0.0, 100.0, 1.0, EasingFn::QuadIn);
        tw.elapsed = 0.5;
        // QuadIn at t=0.5 -> 0.25, so value = 25.0
        assert!((tw.current_value() - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_tween_is_complete() {
        let mut tw = Tween::new(TweenTarget::X, 0.0, 100.0, 1.0, EasingFn::Linear);
        assert!(!tw.is_complete());

        tw.elapsed = 1.0;
        assert!(tw.is_complete());

        tw.looping = true;
        assert!(!tw.is_complete());
    }

    #[test]
    fn test_tween_normalized_t_ping_pong() {
        let mut tw = Tween::new(TweenTarget::X, 0.0, 100.0, 1.0, EasingFn::Linear);
        tw.ping_pong = true;
        tw.forward = true;
        tw.elapsed = 0.5;
        assert!((tw.normalized_t() - 0.5).abs() < 1e-10);

        tw.forward = false;
        tw.elapsed = 0.5;
        // Reverse direction: 1.0 - 0.5 = 0.5
        assert!((tw.normalized_t() - 0.5).abs() < 1e-10);

        tw.forward = false;
        tw.elapsed = 0.0;
        // Reverse direction, at start: 1.0 - 0.0 = 1.0
        assert!((tw.normalized_t() - 1.0).abs() < 1e-10);

        tw.forward = false;
        tw.elapsed = 1.0;
        // Reverse direction, at end: 1.0 - 1.0 = 0.0
        assert!((tw.normalized_t() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_tween_zero_duration() {
        let tw = Tween::new(TweenTarget::X, 0.0, 100.0, 0.0, EasingFn::Linear);
        // Zero duration should immediately complete
        assert!((tw.normalized_t() - 1.0).abs() < 1e-10);
        assert!((tw.current_value() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_property_tween_new() {
        let pt = PropertyTween::new();
        assert!(pt.tweens.is_empty());
    }

    #[test]
    fn test_property_tween_with_tween() {
        let pt = PropertyTween::new()
            .with_tween(Tween::new(TweenTarget::X, 0.0, 10.0, 1.0, EasingFn::Linear))
            .with_tween(Tween::new(TweenTarget::Y, 0.0, 20.0, 2.0, EasingFn::QuadIn));
        assert_eq!(pt.tweens.len(), 2);
    }

    #[test]
    fn test_schema_info() {
        assert_eq!(PropertyTween::schema_name(), "PropertyTween");
        let schema = PropertyTween::schema();
        assert!(schema.get("fields").is_some());
    }

    #[test]
    fn test_all_easings_start_at_zero_end_at_one() {
        let easings = [
            EasingFn::Linear,
            EasingFn::QuadIn,
            EasingFn::QuadOut,
            EasingFn::QuadInOut,
            EasingFn::CubicIn,
            EasingFn::CubicOut,
            EasingFn::CubicInOut,
            EasingFn::BounceOut,
            EasingFn::ElasticOut,
        ];
        for easing in &easings {
            let at_zero = apply_easing(easing, 0.0);
            let at_one = apply_easing(easing, 1.0);
            assert!(
                (at_zero - 0.0).abs() < 1e-10,
                "{:?} at t=0 should be 0, got {}",
                easing,
                at_zero
            );
            assert!(
                (at_one - 1.0).abs() < 1e-10,
                "{:?} at t=1 should be 1, got {}",
                easing,
                at_one
            );
        }
    }

    #[test]
    fn test_tween_negative_range() {
        let mut tw = Tween::new(TweenTarget::X, 100.0, -100.0, 1.0, EasingFn::Linear);
        tw.elapsed = 0.5;
        assert!((tw.current_value() - 0.0).abs() < 1e-10);

        tw.elapsed = 1.0;
        assert!((tw.current_value() - (-100.0)).abs() < 1e-10);
    }
}
