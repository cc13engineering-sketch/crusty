/// ENGINE MODULE: DeathClassify
/// Terminal state classification for simulation sweep results.
/// Analyzes time-series metric data to classify how a run ended:
/// sudden cliff, slow attrition, blowout, close call, or unclassified.

use serde::{Serialize, Deserialize};

// ─── Death Class Enum ───────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeathClass {
    CloseCall,
    Blowout,
    Cliff,
    Attrition,
    Unclassified,
}

impl DeathClass {
    pub fn label(&self) -> &'static str {
        match self {
            DeathClass::CloseCall => "close_call",
            DeathClass::Blowout => "blowout",
            DeathClass::Cliff => "cliff",
            DeathClass::Attrition => "attrition",
            DeathClass::Unclassified => "unclassified",
        }
    }
}

// ─── Classifier Configuration ───────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ClassifierConfig {
    pub window_size: usize,
    pub metric_key: String,
    pub terminal_threshold: f64,
    pub cliff_spike_fraction: f64,
    pub close_call_band: f64,
    pub linearity_threshold: f64,
    pub blowout_slope_threshold: f64,
}

impl Default for ClassifierConfig {
    fn default() -> Self {
        Self {
            window_size: 120,
            metric_key: "health".to_string(),
            terminal_threshold: 0.0,
            cliff_spike_fraction: 0.5,
            close_call_band: 10.0,
            linearity_threshold: 0.7,
            blowout_slope_threshold: 1.0,
        }
    }
}

impl ClassifierConfig {
    pub fn with_window_size(mut self, window_size: usize) -> Self {
        self.window_size = window_size;
        self
    }

    pub fn with_metric_key(mut self, metric_key: &str) -> Self {
        self.metric_key = metric_key.to_string();
        self
    }

    pub fn with_terminal_threshold(mut self, terminal_threshold: f64) -> Self {
        self.terminal_threshold = terminal_threshold;
        self
    }

    pub fn with_cliff_spike_fraction(mut self, cliff_spike_fraction: f64) -> Self {
        self.cliff_spike_fraction = cliff_spike_fraction;
        self
    }

    pub fn with_close_call_band(mut self, close_call_band: f64) -> Self {
        self.close_call_band = close_call_band;
        self
    }

    pub fn with_linearity_threshold(mut self, linearity_threshold: f64) -> Self {
        self.linearity_threshold = linearity_threshold;
        self
    }

    pub fn with_blowout_slope_threshold(mut self, blowout_slope_threshold: f64) -> Self {
        self.blowout_slope_threshold = blowout_slope_threshold;
        self
    }
}

// ─── Classification Result ──────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeathClassification {
    pub class: DeathClass,
    pub trajectory: Vec<f64>,
    pub slope: f64,
    pub residual_variance: f64,
    pub r_squared: f64,
    pub max_drop: f64,
    pub max_drop_frame: usize,
    pub close_call_fraction: f64,
    pub confidence: f64,
}

// ─── Statistical Helpers ────────────────────────────────────────────

/// Compute linear regression on a sequence of values.
/// Returns (slope, intercept, r_squared).
/// x values are assumed to be 0, 1, 2, ..., n-1.
fn linear_regression(values: &[f64]) -> (f64, f64, f64) {
    let n = values.len() as f64;
    if n < 2.0 {
        return (0.0, values.first().copied().unwrap_or(0.0), 0.0);
    }

    let x_mean = (n - 1.0) / 2.0;
    let y_mean = values.iter().sum::<f64>() / n;

    let mut ss_xy = 0.0;
    let mut ss_xx = 0.0;
    let mut ss_yy = 0.0;

    for (i, &y) in values.iter().enumerate() {
        let x = i as f64;
        let dx = x - x_mean;
        let dy = y - y_mean;
        ss_xy += dx * dy;
        ss_xx += dx * dx;
        ss_yy += dy * dy;
    }

    if ss_xx.abs() < 1e-15 {
        return (0.0, y_mean, 0.0);
    }

    let slope = ss_xy / ss_xx;
    let intercept = y_mean - slope * x_mean;

    let r_squared = if ss_yy.abs() < 1e-15 {
        // All y values are the same — perfect "fit" with slope 0
        if slope.abs() < 1e-15 { 1.0 } else { 0.0 }
    } else {
        (ss_xy * ss_xy) / (ss_xx * ss_yy)
    };

    (slope, intercept, r_squared)
}

/// Compute the residual variance of the data around the regression line.
fn residual_variance(values: &[f64], slope: f64, intercept: f64) -> f64 {
    let n = values.len() as f64;
    if n < 2.0 {
        return 0.0;
    }

    let ss_res: f64 = values.iter().enumerate().map(|(i, &y)| {
        let predicted = slope * i as f64 + intercept;
        let residual = y - predicted;
        residual * residual
    }).sum();

    ss_res / n
}

// ─── Main Classification Function ───────────────────────────────────

/// Classify a time-series of metric values into a death class.
///
/// The series should contain per-frame values of the tracked metric
/// (e.g., health over time). The classifier examines the last `window_size`
/// frames to determine how the run ended.
pub fn classify(series: &[f64], config: &ClassifierConfig) -> DeathClassification {
    // Extract the analysis window
    let start = if series.len() > config.window_size {
        series.len() - config.window_size
    } else {
        0
    };
    let trajectory: Vec<f64> = series[start..].to_vec();

    // Too few frames to classify
    if trajectory.len() < 3 {
        return DeathClassification {
            class: DeathClass::Unclassified,
            trajectory,
            slope: 0.0,
            residual_variance: 0.0,
            r_squared: 0.0,
            max_drop: 0.0,
            max_drop_frame: 0,
            close_call_fraction: 0.0,
            confidence: 0.0,
        };
    }

    // Linear regression
    let (slope, intercept, r_squared) = linear_regression(&trajectory);
    let res_var = residual_variance(&trajectory, slope, intercept);

    // Find maximum single-frame drop
    let mut max_drop = 0.0_f64;
    let mut max_drop_frame = 0usize;
    for i in 1..trajectory.len() {
        let drop = trajectory[i - 1] - trajectory[i];
        if drop > max_drop {
            max_drop = drop;
            max_drop_frame = i;
        }
    }

    // Trajectory range
    let traj_min = trajectory.iter().cloned().fold(f64::INFINITY, f64::min);
    let traj_max = trajectory.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let traj_range = (traj_max - traj_min).abs();

    // Close-call fraction: frames within close_call_band of terminal_threshold
    let close_call_count = trajectory.iter()
        .filter(|&&v| (v - config.terminal_threshold).abs() <= config.close_call_band)
        .count();
    let close_call_fraction = close_call_count as f64 / trajectory.len() as f64;

    // Classification logic
    let half_point = trajectory.len() / 2;

    let class = if close_call_fraction > 0.5 {
        DeathClass::CloseCall
    } else if traj_range > 0.0
        && max_drop > traj_range * config.cliff_spike_fraction
        && max_drop_frame >= half_point
    {
        DeathClass::Cliff
    } else if r_squared >= config.linearity_threshold
        && slope < 0.0
        && slope.abs() >= config.blowout_slope_threshold
    {
        DeathClass::Blowout
    } else if r_squared >= config.linearity_threshold && slope < 0.0 {
        DeathClass::Attrition
    } else {
        DeathClass::Unclassified
    };

    // Compute confidence based on how strongly the pattern matches
    let confidence = match &class {
        DeathClass::Cliff => {
            if traj_range > 0.0 {
                (max_drop / traj_range).min(1.0)
            } else {
                0.0
            }
        }
        DeathClass::CloseCall => close_call_fraction.min(1.0),
        DeathClass::Blowout => r_squared.min(1.0),
        DeathClass::Attrition => r_squared.min(1.0),
        DeathClass::Unclassified => 0.0,
    };

    DeathClassification {
        class,
        trajectory,
        slope,
        residual_variance: res_var,
        r_squared,
        max_drop,
        max_drop_frame,
        close_call_fraction,
        confidence,
    }
}

// ─── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> ClassifierConfig {
        ClassifierConfig::default()
    }

    #[test]
    fn linear_regression_perfect_line() {
        let values = vec![0.0, 1.0, 2.0, 3.0];
        let (slope, _intercept, r_squared) = linear_regression(&values);
        assert!((slope - 1.0).abs() < 1e-10, "slope should be 1.0, got {}", slope);
        assert!((r_squared - 1.0).abs() < 1e-10, "R^2 should be 1.0, got {}", r_squared);
    }

    #[test]
    fn linear_regression_constant_data() {
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let (slope, intercept, _r_squared) = linear_regression(&values);
        assert!(slope.abs() < 1e-10, "slope should be 0.0, got {}", slope);
        assert!((intercept - 5.0).abs() < 1e-10, "intercept should be 5.0, got {}", intercept);
    }

    #[test]
    fn classify_cliff() {
        // Flat at 100 for 100 frames, then sudden drop to 0
        let mut series: Vec<f64> = vec![100.0; 100];
        series.push(0.0);

        let config = default_config().with_window_size(120);
        let result = classify(&series, &config);
        assert_eq!(result.class, DeathClass::Cliff,
            "Expected Cliff, got {:?}. max_drop={}, traj_range={}, max_drop_frame={}, len={}",
            result.class, result.max_drop, 100.0, result.max_drop_frame, result.trajectory.len());
    }

    #[test]
    fn classify_close_call() {
        // Values oscillating between 2 and 8, ending at 0
        let mut series: Vec<f64> = Vec::new();
        for i in 0..100 {
            if i % 2 == 0 {
                series.push(2.0);
            } else {
                series.push(8.0);
            }
        }
        series.push(0.0);

        let config = default_config().with_window_size(120);
        let result = classify(&series, &config);
        assert_eq!(result.class, DeathClass::CloseCall,
            "Expected CloseCall, got {:?}. close_call_fraction={}",
            result.class, result.close_call_fraction);
    }

    #[test]
    fn classify_blowout() {
        // Linearly declining 100 -> 0 over 60 frames (steep slope)
        let mut series: Vec<f64> = Vec::new();
        for i in 0..61 {
            series.push(100.0 - (100.0 / 60.0) * i as f64);
        }

        let config = default_config()
            .with_window_size(120)
            .with_blowout_slope_threshold(1.0);
        let result = classify(&series, &config);
        assert_eq!(result.class, DeathClass::Blowout,
            "Expected Blowout, got {:?}. slope={}, r_squared={}",
            result.class, result.slope, result.r_squared);
    }

    #[test]
    fn classify_attrition() {
        // Gently declining 100 -> 70 over 300 frames
        let mut series: Vec<f64> = Vec::new();
        for i in 0..301 {
            series.push(100.0 - (30.0 / 300.0) * i as f64);
        }

        let config = default_config()
            .with_window_size(300)
            .with_blowout_slope_threshold(1.0);
        let result = classify(&series, &config);
        assert_eq!(result.class, DeathClass::Attrition,
            "Expected Attrition, got {:?}. slope={}, r_squared={}",
            result.class, result.slope, result.r_squared);
    }

    #[test]
    fn classify_too_few_frames() {
        let series = vec![100.0, 50.0];
        let config = default_config();
        let result = classify(&series, &config);
        assert_eq!(result.class, DeathClass::Unclassified);
    }

    #[test]
    fn death_class_labels() {
        assert_eq!(DeathClass::CloseCall.label(), "close_call");
        assert_eq!(DeathClass::Blowout.label(), "blowout");
        assert_eq!(DeathClass::Cliff.label(), "cliff");
        assert_eq!(DeathClass::Attrition.label(), "attrition");
        assert_eq!(DeathClass::Unclassified.label(), "unclassified");
    }

    #[test]
    fn config_builder_methods() {
        let config = ClassifierConfig::default()
            .with_window_size(60)
            .with_metric_key("score")
            .with_terminal_threshold(10.0)
            .with_cliff_spike_fraction(0.3)
            .with_close_call_band(5.0)
            .with_linearity_threshold(0.8)
            .with_blowout_slope_threshold(2.0);
        assert_eq!(config.window_size, 60);
        assert_eq!(config.metric_key, "score");
        assert!((config.terminal_threshold - 10.0).abs() < 1e-10);
        assert!((config.cliff_spike_fraction - 0.3).abs() < 1e-10);
        assert!((config.close_call_band - 5.0).abs() < 1e-10);
        assert!((config.linearity_threshold - 0.8).abs() < 1e-10);
        assert!((config.blowout_slope_threshold - 2.0).abs() < 1e-10);
    }

    #[test]
    fn residual_variance_perfect_fit() {
        let values = vec![0.0, 1.0, 2.0, 3.0];
        let rv = residual_variance(&values, 1.0, 0.0);
        assert!(rv.abs() < 1e-10, "residual variance should be ~0, got {}", rv);
    }
}
