use super::SchemaInfo;

/// Per-entity time dilation. Systems that consume dt should multiply by this value.
/// time_scale=1.0 is normal speed, 0.5 is half speed, 2.0 is double, 0.0 is frozen.
#[derive(Clone, Debug)]
pub struct TimeScale {
    pub scale: f64,
}

impl TimeScale {
    pub fn new(scale: f64) -> Self {
        Self { scale }
    }

    pub fn normal() -> Self {
        Self { scale: 1.0 }
    }

    pub fn frozen() -> Self {
        Self { scale: 0.0 }
    }

    pub fn slow_mo(factor: f64) -> Self {
        Self { scale: factor.max(0.0) }
    }

    /// Returns the effective dt for this entity.
    /// Negative scale values are clamped to 0.0.
    pub fn apply(&self, dt: f64) -> f64 {
        dt * self.scale.max(0.0)
    }
}

impl SchemaInfo for TimeScale {
    fn schema_name() -> &'static str { "TimeScale" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "scale": { "type": "f64", "default": 1.0, "note": "1.0=normal, 0.0=frozen, 0.5=half speed, 2.0=double" }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ts = TimeScale::new(1.5);
        assert_eq!(ts.scale, 1.5);
    }

    #[test]
    fn test_normal() {
        let ts = TimeScale::normal();
        assert_eq!(ts.scale, 1.0);
    }

    #[test]
    fn test_frozen() {
        let ts = TimeScale::frozen();
        assert_eq!(ts.scale, 0.0);
    }

    #[test]
    fn test_slow_mo_positive() {
        let ts = TimeScale::slow_mo(0.5);
        assert_eq!(ts.scale, 0.5);
    }

    #[test]
    fn test_slow_mo_clamps_negative_to_zero() {
        let ts = TimeScale::slow_mo(-1.0);
        assert_eq!(ts.scale, 0.0);
    }

    #[test]
    fn test_slow_mo_zero() {
        let ts = TimeScale::slow_mo(0.0);
        assert_eq!(ts.scale, 0.0);
    }

    #[test]
    fn test_apply_normal_speed() {
        let ts = TimeScale::normal();
        let dt = 0.016;
        let result = ts.apply(dt);
        assert!((result - 0.016).abs() < f64::EPSILON);
    }

    #[test]
    fn test_apply_half_speed() {
        let ts = TimeScale::new(0.5);
        let dt = 0.016;
        let result = ts.apply(dt);
        assert!((result - 0.008).abs() < f64::EPSILON);
    }

    #[test]
    fn test_apply_double_speed() {
        let ts = TimeScale::new(2.0);
        let dt = 0.016;
        let result = ts.apply(dt);
        assert!((result - 0.032).abs() < f64::EPSILON);
    }

    #[test]
    fn test_apply_frozen() {
        let ts = TimeScale::frozen();
        let dt = 0.016;
        let result = ts.apply(dt);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_apply_negative_scale_clamped_to_zero() {
        let ts = TimeScale::new(-2.0);
        let dt = 0.016;
        let result = ts.apply(dt);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_apply_zero_dt() {
        let ts = TimeScale::new(5.0);
        let result = ts.apply(0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_apply_large_scale() {
        let ts = TimeScale::new(100.0);
        let dt = 0.016;
        let result = ts.apply(dt);
        assert!((result - 1.6).abs() < 1e-10);
    }

    #[test]
    fn test_apply_very_small_scale() {
        let ts = TimeScale::new(0.001);
        let dt = 1.0;
        let result = ts.apply(dt);
        assert!((result - 0.001).abs() < f64::EPSILON);
    }

    #[test]
    fn test_clone() {
        let ts = TimeScale::new(0.75);
        let cloned = ts.clone();
        assert_eq!(cloned.scale, 0.75);
    }

    #[test]
    fn test_debug() {
        let ts = TimeScale::new(1.0);
        let debug_str = format!("{:?}", ts);
        assert!(debug_str.contains("TimeScale"));
        assert!(debug_str.contains("1.0"));
    }

    #[test]
    fn test_schema_name() {
        assert_eq!(TimeScale::schema_name(), "TimeScale");
    }

    #[test]
    fn test_schema_is_valid_json() {
        let schema = TimeScale::schema();
        assert!(schema.get("fields").is_some());
        let fields = schema.get("fields").unwrap();
        assert!(fields.get("scale").is_some());
    }

    #[test]
    fn test_new_allows_storing_negative() {
        // new() does not clamp; only apply() and slow_mo() clamp
        let ts = TimeScale::new(-5.0);
        assert_eq!(ts.scale, -5.0);
    }

    #[test]
    fn test_apply_preserves_precision() {
        let ts = TimeScale::new(1.0 / 3.0);
        let dt = 0.03;
        let result = ts.apply(dt);
        assert!((result - 0.01).abs() < 1e-10);
    }

    #[test]
    fn test_scale_field_is_public() {
        let mut ts = TimeScale::normal();
        ts.scale = 3.0;
        assert_eq!(ts.scale, 3.0);
    }
}
