/// LevelCurve — Progression and Difficulty Scaling
///
/// Manages named numeric curves that interpolate values based on the current
/// game level. Supports multiple interpolation shapes (linear, ease-in, ease-out,
/// step) and a global difficulty multiplier that scales the effective level when
/// sampling curves.

use std::collections::HashMap;

// ─── CurveShape ─────────────────────────────────────────────────────────────

/// Shape of the interpolation used between keyframes.
#[derive(Clone, Debug, PartialEq)]
pub enum CurveShape {
    /// Uniform rate of change from one keyframe to the next.
    Linear,
    /// Slow start, fast end (quadratic ease-in: t²).
    EaseIn,
    /// Fast start, slow end (quadratic ease-out: t*(2-t)).
    EaseOut,
    /// Staircase — value is held at the previous keyframe until the next is
    /// reached (floor to nearest keyframe).
    Step,
}

// ─── Curve ──────────────────────────────────────────────────────────────────

/// A named numeric curve defined by a set of (level, value) keyframes.
///
/// Keyframes are kept sorted by level. Sampling outside the keyframe range
/// clamps to the first or last value, then additionally clamps to
/// [`clamp_min`] / [`clamp_max`].
#[derive(Clone, Debug)]
pub struct Curve {
    pub name: String,
    pub shape: CurveShape,
    /// Sorted list of `(level, value)` pairs.
    pub keyframes: Vec<(f64, f64)>,
    pub clamp_min: f64,
    pub clamp_max: f64,
}

impl Curve {
    /// Create a new curve. `keyframes` will be sorted by level automatically.
    /// Default clamp range is `[f64::NEG_INFINITY, f64::INFINITY]` (no clamping).
    pub fn new(name: &str, shape: CurveShape, keyframes: &[(f64, f64)]) -> Self {
        let mut kf: Vec<(f64, f64)> = keyframes.to_vec();
        kf.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        Self {
            name: name.to_string(),
            shape,
            keyframes: kf,
            clamp_min: f64::NEG_INFINITY,
            clamp_max: f64::INFINITY,
        }
    }

    /// Convenience constructor for a [`CurveShape::Linear`] curve.
    pub fn new_linear(name: &str, keyframes: &[(f64, f64)]) -> Self {
        Self::new(name, CurveShape::Linear, keyframes)
    }

    /// Sample the curve at the given `level`, returning an interpolated value.
    ///
    /// - If there are no keyframes, returns `0.0`.
    /// - If `level` is below the first keyframe, returns the first value.
    /// - If `level` is above the last keyframe, returns the last value.
    /// - Otherwise interpolates between the surrounding keyframes using the
    ///   curve's [`CurveShape`].
    /// - The result is clamped to `[clamp_min, clamp_max]`.
    pub fn sample(&self, level: f64) -> f64 {
        if self.keyframes.is_empty() {
            return 0.0;
        }

        let first = self.keyframes[0];
        let last = *self.keyframes.last().expect("keyframes non-empty");

        // Clamp level to keyframe range
        if level <= first.0 {
            return self.apply_clamp(first.1);
        }
        if level >= last.0 {
            return self.apply_clamp(last.1);
        }

        // Find the surrounding pair
        let mut lo_idx = 0usize;
        for (i, kf) in self.keyframes.iter().enumerate() {
            if kf.0 <= level {
                lo_idx = i;
            } else {
                break;
            }
        }

        let (l0, v0) = self.keyframes[lo_idx];
        let (l1, v1) = self.keyframes[lo_idx + 1];

        let raw = match self.shape {
            CurveShape::Step => v0,
            CurveShape::Linear => {
                let t = normalize(level, l0, l1);
                lerp(v0, v1, t)
            }
            CurveShape::EaseIn => {
                let t = normalize(level, l0, l1);
                let t2 = t * t; // quadratic ease-in
                lerp(v0, v1, t2)
            }
            CurveShape::EaseOut => {
                let t = normalize(level, l0, l1);
                let t2 = t * (2.0 - t); // quadratic ease-out
                lerp(v0, v1, t2)
            }
        };

        self.apply_clamp(raw)
    }

    /// Clamp a raw value to `[clamp_min, clamp_max]`.
    #[inline]
    fn apply_clamp(&self, v: f64) -> f64 {
        v.max(self.clamp_min).min(self.clamp_max)
    }
}

// ─── DifficultyPreset ────────────────────────────────────────────────────────

/// Built-in difficulty presets for common game parameters.
#[derive(Clone, Debug, PartialEq)]
pub enum DifficultyPreset {
    /// Relaxed difficulty — gentler curves, more player-friendly values.
    Casual,
    /// Balanced, moderate difficulty ramp.
    Standard,
    /// Aggressive scaling — high enemy stats, fast spawn rates.
    Hardcore,
}

// ─── LevelCurve ─────────────────────────────────────────────────────────────

/// Manages a collection of named [`Curve`]s and the current progression level.
///
/// When sampling a curve via [`value`], the effective level is
/// `current_level * difficulty_multiplier`, so the multiplier acts as a global
/// scaling knob on top of the level progression.
#[derive(Clone, Debug)]
pub struct LevelCurve {
    pub curves: HashMap<String, Curve>,
    /// Current game level (e.g. wave number, elapsed time, depth, etc.).
    pub current_level: f64,
    /// Global difficulty scaling factor applied to `current_level` on every
    /// [`value`] sample. `1.0` = no scaling.
    pub difficulty_multiplier: f64,
}

impl LevelCurve {
    /// Create a new, empty `LevelCurve` at level 0 with no multiplier scaling.
    pub fn new() -> Self {
        Self {
            curves: HashMap::new(),
            current_level: 0.0,
            difficulty_multiplier: 1.0,
        }
    }

    /// Register a [`Curve`]. If a curve with the same name already exists it
    /// is replaced.
    pub fn add_curve(&mut self, curve: Curve) {
        self.curves.insert(curve.name.clone(), curve);
    }

    /// Sample the named curve at `current_level * difficulty_multiplier`.
    ///
    /// Returns `0.0` if the curve does not exist.
    pub fn value(&self, name: &str) -> f64 {
        let effective_level = self.current_level * self.difficulty_multiplier;
        if let Some(curve) = self.curves.get(name) {
            curve.sample(effective_level)
        } else {
            0.0
        }
    }

    /// Increment `current_level` by 1 and return the new level.
    pub fn advance_level(&mut self) -> f64 {
        self.current_level += 1.0;
        self.current_level
    }

    /// Set `current_level` to an explicit value.
    pub fn set_level(&mut self, level: f64) {
        self.current_level = level;
    }

    /// Clear all curves and reset level and multiplier to defaults.
    pub fn clear(&mut self) {
        self.curves.clear();
        self.current_level = 0.0;
        self.difficulty_multiplier = 1.0;
    }

    /// Build a `LevelCurve` pre-loaded with common game-parameter curves for
    /// the given [`DifficultyPreset`].
    ///
    /// Curves created:
    /// - `"enemy_speed"`    — how fast enemies move
    /// - `"spawn_rate"`     — enemies spawned per wave
    /// - `"enemy_health"`   — enemy hit-points
    /// - `"player_damage"`  — base player damage output
    /// - `"score_mult"`     — score multiplier per kill
    pub fn from_preset(preset: DifficultyPreset) -> Self {
        let mut lc = LevelCurve::new();

        match preset {
            DifficultyPreset::Casual => {
                lc.difficulty_multiplier = 0.7;

                lc.add_curve(Curve::new_linear("enemy_speed", &[
                    (0.0, 60.0),
                    (5.0, 90.0),
                    (10.0, 120.0),
                    (20.0, 150.0),
                ]));
                lc.add_curve(Curve::new_linear("spawn_rate", &[
                    (0.0, 2.0),
                    (5.0, 4.0),
                    (10.0, 6.0),
                    (20.0, 8.0),
                ]));
                lc.add_curve(Curve::new("enemy_health", CurveShape::EaseIn, &[
                    (0.0, 1.0),
                    (5.0, 2.0),
                    (10.0, 4.0),
                    (20.0, 7.0),
                ]));
                lc.add_curve(Curve::new_linear("player_damage", &[
                    (0.0, 25.0),
                    (10.0, 35.0),
                    (20.0, 50.0),
                ]));
                lc.add_curve(Curve::new_linear("score_mult", &[
                    (0.0, 1.0),
                    (10.0, 1.5),
                    (20.0, 2.0),
                ]));
            }

            DifficultyPreset::Standard => {
                lc.difficulty_multiplier = 1.0;

                lc.add_curve(Curve::new_linear("enemy_speed", &[
                    (0.0, 80.0),
                    (5.0, 120.0),
                    (10.0, 170.0),
                    (20.0, 250.0),
                ]));
                lc.add_curve(Curve::new("spawn_rate", CurveShape::EaseIn, &[
                    (0.0, 3.0),
                    (5.0, 6.0),
                    (10.0, 10.0),
                    (20.0, 16.0),
                ]));
                lc.add_curve(Curve::new("enemy_health", CurveShape::EaseIn, &[
                    (0.0, 2.0),
                    (5.0, 4.0),
                    (10.0, 8.0),
                    (20.0, 18.0),
                ]));
                lc.add_curve(Curve::new_linear("player_damage", &[
                    (0.0, 20.0),
                    (10.0, 30.0),
                    (20.0, 45.0),
                ]));
                lc.add_curve(Curve::new_linear("score_mult", &[
                    (0.0, 1.0),
                    (10.0, 2.0),
                    (20.0, 4.0),
                ]));
            }

            DifficultyPreset::Hardcore => {
                lc.difficulty_multiplier = 1.5;

                lc.add_curve(Curve::new("enemy_speed", CurveShape::EaseIn, &[
                    (0.0, 120.0),
                    (5.0, 200.0),
                    (10.0, 300.0),
                    (20.0, 500.0),
                ]));
                lc.add_curve(Curve::new("spawn_rate", CurveShape::EaseIn, &[
                    (0.0, 5.0),
                    (5.0, 10.0),
                    (10.0, 18.0),
                    (20.0, 30.0),
                ]));
                lc.add_curve(Curve::new("enemy_health", CurveShape::EaseIn, &[
                    (0.0, 3.0),
                    (5.0, 8.0),
                    (10.0, 20.0),
                    (20.0, 50.0),
                ]));
                lc.add_curve(Curve::new_linear("player_damage", &[
                    (0.0, 15.0),
                    (10.0, 25.0),
                    (20.0, 40.0),
                ]));
                lc.add_curve(Curve::new("score_mult", CurveShape::EaseOut, &[
                    (0.0, 2.0),
                    (10.0, 5.0),
                    (20.0, 10.0),
                ]));
            }
        }

        lc
    }
}

impl Default for LevelCurve {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Normalize `v` to `[0, 1]` within `[lo, hi]`. Returns 0 when `lo == hi`.
#[inline]
fn normalize(v: f64, lo: f64, hi: f64) -> f64 {
    let span = hi - lo;
    if span == 0.0 {
        0.0
    } else {
        ((v - lo) / span).max(0.0).min(1.0)
    }
}

/// Linear interpolation between `a` and `b` by `t ∈ [0, 1]`.
#[inline]
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Curve::sample — CurveShape::Linear ──────────────────────────────────

    #[test]
    fn linear_sample_midpoint() {
        let c = Curve::new_linear("speed", &[(0.0, 10.0), (10.0, 20.0)]);
        let v = c.sample(5.0);
        assert!((v - 15.0).abs() < 1e-10, "expected 15.0, got {}", v);
    }

    #[test]
    fn linear_sample_at_first_keyframe() {
        let c = Curve::new_linear("speed", &[(0.0, 10.0), (10.0, 20.0)]);
        assert!((c.sample(0.0) - 10.0).abs() < 1e-10);
    }

    #[test]
    fn linear_sample_at_last_keyframe() {
        let c = Curve::new_linear("speed", &[(0.0, 10.0), (10.0, 20.0)]);
        assert!((c.sample(10.0) - 20.0).abs() < 1e-10);
    }

    // ── Curve::sample — CurveShape::EaseIn ──────────────────────────────────

    #[test]
    fn ease_in_sample_midpoint_less_than_linear() {
        // ease-in (t²): at t=0.5, curve value = 0.25 vs linear 0.5
        // So midpoint value should be below the linear midpoint.
        let linear = Curve::new_linear("x", &[(0.0, 0.0), (10.0, 100.0)]);
        let ease = Curve::new("x", CurveShape::EaseIn, &[(0.0, 0.0), (10.0, 100.0)]);
        assert!(ease.sample(5.0) < linear.sample(5.0));
    }

    #[test]
    fn ease_in_sample_endpoints_match_keyframes() {
        let c = Curve::new("x", CurveShape::EaseIn, &[(0.0, 0.0), (10.0, 100.0)]);
        assert!((c.sample(0.0) - 0.0).abs() < 1e-10);
        assert!((c.sample(10.0) - 100.0).abs() < 1e-10);
    }

    // ── Curve::sample — CurveShape::EaseOut ─────────────────────────────────

    #[test]
    fn ease_out_sample_midpoint_greater_than_linear() {
        // ease-out (t*(2-t)): at t=0.5, curve value = 0.75 vs linear 0.5
        let linear = Curve::new_linear("x", &[(0.0, 0.0), (10.0, 100.0)]);
        let ease = Curve::new("x", CurveShape::EaseOut, &[(0.0, 0.0), (10.0, 100.0)]);
        assert!(ease.sample(5.0) > linear.sample(5.0));
    }

    #[test]
    fn ease_out_sample_endpoints_match_keyframes() {
        let c = Curve::new("x", CurveShape::EaseOut, &[(0.0, 0.0), (10.0, 100.0)]);
        assert!((c.sample(0.0) - 0.0).abs() < 1e-10);
        assert!((c.sample(10.0) - 100.0).abs() < 1e-10);
    }

    // ── Curve::sample — CurveShape::Step ────────────────────────────────────

    #[test]
    fn step_holds_value_until_next_keyframe() {
        let c = Curve::new("tier", CurveShape::Step, &[
            (0.0, 1.0),
            (5.0, 2.0),
            (10.0, 3.0),
        ]);
        // Before second keyframe
        assert!((c.sample(4.9) - 1.0).abs() < 1e-10);
        // At second keyframe
        assert!((c.sample(5.0) - 2.0).abs() < 1e-10);
        // Slightly after second keyframe but before third
        assert!((c.sample(7.0) - 2.0).abs() < 1e-10);
        // At third keyframe
        assert!((c.sample(10.0) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn step_below_first_keyframe_returns_first_value() {
        let c = Curve::new("tier", CurveShape::Step, &[(5.0, 10.0), (10.0, 20.0)]);
        assert!((c.sample(0.0) - 10.0).abs() < 1e-10);
    }

    // ── Keyframe interpolation ───────────────────────────────────────────────

    #[test]
    fn multi_keyframe_interpolation_selects_correct_segment() {
        // Three keyframes: [0→10, 5→20, 10→30]
        // Sampling at 7.5 should interpolate between (5,20) and (10,30) at t=0.5
        let c = Curve::new_linear("x", &[(0.0, 10.0), (5.0, 20.0), (10.0, 30.0)]);
        let expected = 25.0; // midpoint between 20 and 30
        assert!((c.sample(7.5) - expected).abs() < 1e-10);
    }

    #[test]
    fn keyframes_are_sorted_on_construction() {
        // Pass keyframes out of order — should still sample correctly.
        let c = Curve::new_linear("x", &[(10.0, 100.0), (0.0, 0.0), (5.0, 50.0)]);
        assert!((c.sample(0.0) - 0.0).abs() < 1e-10);
        assert!((c.sample(5.0) - 50.0).abs() < 1e-10);
        assert!((c.sample(10.0) - 100.0).abs() < 1e-10);
    }

    // ── Boundary clamping ────────────────────────────────────────────────────

    #[test]
    fn clamp_min_caps_output() {
        let mut c = Curve::new_linear("x", &[(0.0, 0.0), (10.0, 100.0)]);
        c.clamp_min = 50.0;
        // At level 0 the raw value is 0.0, but clamped to 50.0
        assert!((c.sample(0.0) - 50.0).abs() < 1e-10);
        // At level 10 value is 100.0, above clamp_min
        assert!((c.sample(10.0) - 100.0).abs() < 1e-10);
    }

    #[test]
    fn clamp_max_caps_output() {
        let mut c = Curve::new_linear("x", &[(0.0, 0.0), (10.0, 100.0)]);
        c.clamp_max = 60.0;
        // At level 10 the raw value is 100.0, clamped to 60.0
        assert!((c.sample(10.0) - 60.0).abs() < 1e-10);
        // At level 0 value is 0.0, below clamp_max
        assert!((c.sample(0.0) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn level_below_first_keyframe_clamps_to_first_value() {
        let c = Curve::new_linear("x", &[(5.0, 10.0), (10.0, 20.0)]);
        assert!((c.sample(-100.0) - 10.0).abs() < 1e-10);
    }

    #[test]
    fn level_above_last_keyframe_clamps_to_last_value() {
        let c = Curve::new_linear("x", &[(0.0, 0.0), (10.0, 100.0)]);
        assert!((c.sample(9999.0) - 100.0).abs() < 1e-10);
    }

    // ── Empty keyframes ──────────────────────────────────────────────────────

    #[test]
    fn empty_keyframes_returns_zero() {
        let c = Curve::new_linear("empty", &[]);
        assert!((c.sample(0.0) - 0.0).abs() < 1e-10);
        assert!((c.sample(100.0) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn single_keyframe_always_returns_that_value() {
        let c = Curve::new_linear("x", &[(5.0, 42.0)]);
        assert!((c.sample(0.0) - 42.0).abs() < 1e-10);
        assert!((c.sample(5.0) - 42.0).abs() < 1e-10);
        assert!((c.sample(100.0) - 42.0).abs() < 1e-10);
    }

    // ── Preset generation ────────────────────────────────────────────────────

    #[test]
    fn preset_casual_has_expected_curves() {
        let lc = LevelCurve::from_preset(DifficultyPreset::Casual);
        for name in &["enemy_speed", "spawn_rate", "enemy_health", "player_damage", "score_mult"] {
            assert!(lc.curves.contains_key(*name), "missing curve: {}", name);
        }
        assert!((lc.difficulty_multiplier - 0.7).abs() < 1e-10);
    }

    #[test]
    fn preset_standard_has_expected_curves() {
        let lc = LevelCurve::from_preset(DifficultyPreset::Standard);
        for name in &["enemy_speed", "spawn_rate", "enemy_health", "player_damage", "score_mult"] {
            assert!(lc.curves.contains_key(*name), "missing curve: {}", name);
        }
        assert!((lc.difficulty_multiplier - 1.0).abs() < 1e-10);
    }

    #[test]
    fn preset_hardcore_has_expected_curves() {
        let lc = LevelCurve::from_preset(DifficultyPreset::Hardcore);
        for name in &["enemy_speed", "spawn_rate", "enemy_health", "player_damage", "score_mult"] {
            assert!(lc.curves.contains_key(*name), "missing curve: {}", name);
        }
        assert!((lc.difficulty_multiplier - 1.5).abs() < 1e-10);
    }

    #[test]
    fn hardcore_enemy_speed_exceeds_casual_at_same_level() {
        let mut casual = LevelCurve::from_preset(DifficultyPreset::Casual);
        let mut hardcore = LevelCurve::from_preset(DifficultyPreset::Hardcore);

        // Disable the multiplier so we compare raw level values.
        casual.difficulty_multiplier = 1.0;
        hardcore.difficulty_multiplier = 1.0;
        casual.set_level(10.0);
        hardcore.set_level(10.0);

        assert!(
            hardcore.value("enemy_speed") > casual.value("enemy_speed"),
            "hardcore speed should exceed casual speed at level 10"
        );
    }

    // ── advance_level ────────────────────────────────────────────────────────

    #[test]
    fn advance_level_increments_by_one() {
        let mut lc = LevelCurve::new();
        assert!((lc.current_level - 0.0).abs() < 1e-10);
        let new_level = lc.advance_level();
        assert!((new_level - 1.0).abs() < 1e-10);
        assert!((lc.current_level - 1.0).abs() < 1e-10);
        lc.advance_level();
        lc.advance_level();
        assert!((lc.current_level - 3.0).abs() < 1e-10);
    }

    // ── difficulty_multiplier effect ─────────────────────────────────────────

    #[test]
    fn difficulty_multiplier_scales_effective_level() {
        let mut lc = LevelCurve::new();
        lc.add_curve(Curve::new_linear("speed", &[(0.0, 0.0), (10.0, 100.0)]));

        lc.set_level(5.0);
        lc.difficulty_multiplier = 1.0;
        let base = lc.value("speed"); // effective level = 5.0 → 50.0

        lc.difficulty_multiplier = 2.0;
        let harder = lc.value("speed"); // effective level = 10.0 → 100.0

        assert!((base - 50.0).abs() < 1e-10, "base was {}", base);
        assert!((harder - 100.0).abs() < 1e-10, "harder was {}", harder);
    }

    // ── add_curve and clear ──────────────────────────────────────────────────

    #[test]
    fn add_curve_and_value_lookup() {
        let mut lc = LevelCurve::new();
        lc.add_curve(Curve::new_linear("health", &[(0.0, 1.0), (10.0, 10.0)]));
        lc.set_level(5.0);
        let v = lc.value("health");
        assert!((v - 5.5).abs() < 1e-10, "expected 5.5, got {}", v);
    }

    #[test]
    fn value_returns_zero_for_missing_curve() {
        let lc = LevelCurve::new();
        assert!((lc.value("nonexistent") - 0.0).abs() < 1e-10);
    }

    #[test]
    fn clear_resets_all_state() {
        let mut lc = LevelCurve::from_preset(DifficultyPreset::Standard);
        lc.set_level(15.0);
        lc.difficulty_multiplier = 3.0;
        lc.clear();
        assert!(lc.curves.is_empty());
        assert!((lc.current_level - 0.0).abs() < 1e-10);
        assert!((lc.difficulty_multiplier - 1.0).abs() < 1e-10);
    }
}
