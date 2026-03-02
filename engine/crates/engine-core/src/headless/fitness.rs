use super::runner::SimResult;

/// A single named scoring criterion.
pub struct FitnessCriterion {
    pub name: String,
    pub weight: f64,
    /// Scoring function: SimResult -> f64 in [0.0, 1.0] (higher = better).
    pub score_fn: fn(&SimResult) -> f64,
}

/// Breakdown of one criterion's contribution.
#[derive(Clone, Debug)]
pub struct CriterionResult {
    pub name: String,
    pub raw_score: f64,
    pub weight: f64,
    pub weighted: f64,
}

/// Full evaluation result with composite score.
#[derive(Clone, Debug)]
pub struct FitnessResult {
    /// Weighted average score, normalized to [0.0, 1.0].
    pub total: f64,
    /// Per-criterion breakdown.
    pub criteria: Vec<CriterionResult>,
}

impl FitnessResult {
    /// Machine-readable one-line summary for AI consumption.
    pub fn summary(&self) -> String {
        let parts: Vec<String> = self.criteria.iter().map(|c| {
            format!("{}={:.2}(x{:.0})", c.name, c.raw_score, c.weight)
        }).collect();
        format!("total={:.2} | {}", self.total, parts.join(" "))
    }

    /// Letter grade from composite score.
    pub fn grade(&self) -> &'static str {
        match self.total {
            s if s >= 0.95 => "A+",
            s if s >= 0.85 => "A",
            s if s >= 0.70 => "B",
            s if s >= 0.50 => "C",
            s if s >= 0.30 => "D",
            _ => "F",
        }
    }
}

/// Composable evaluator: a collection of weighted criteria.
pub struct FitnessEvaluator {
    criteria: Vec<FitnessCriterion>,
}

impl FitnessEvaluator {
    pub fn new() -> Self {
        Self { criteria: Vec::new() }
    }

    pub fn add(mut self, name: &str, weight: f64, f: fn(&SimResult) -> f64) -> Self {
        self.criteria.push(FitnessCriterion {
            name: name.to_string(),
            weight,
            score_fn: f,
        });
        self
    }

    /// Evaluate a single SimResult.
    pub fn evaluate(&self, sim: &SimResult) -> FitnessResult {
        let mut total_weight = 0.0;
        let mut weighted_sum = 0.0;
        let mut criteria = Vec::new();

        for c in &self.criteria {
            let raw = (c.score_fn)(sim).clamp(0.0, 1.0);
            let weighted = raw * c.weight;
            total_weight += c.weight;
            weighted_sum += weighted;
            criteria.push(CriterionResult {
                name: c.name.clone(),
                raw_score: raw,
                weight: c.weight,
                weighted,
            });
        }

        FitnessResult {
            total: if total_weight > 0.0 { weighted_sum / total_weight } else { 0.0 },
            criteria,
        }
    }

    /// Evaluate every result in a sweep report, return sorted best-first.
    pub fn rank_sweep(&self, report: &super::sweep::SweepReport) -> Vec<(String, FitnessResult)> {
        let mut ranked: Vec<(String, FitnessResult)> = report.results.iter().map(|r| {
            let fitness = self.evaluate(&r.sim);
            (r.config.label.clone(), fitness)
        }).collect();
        ranked.sort_by(|a, b| b.1.total.partial_cmp(&a.1.total).unwrap_or(std::cmp::Ordering::Equal));
        ranked
    }
}

// ─── Generic scoring helpers ────────────────────────────────────────

/// 1.0 when distance between two (x,y) state key pairs is 0, degrades linearly
/// to 0.0 at `max_dist`. Useful for proximity-based objectives in any game.
pub fn score_distance(
    x1_key: &str, y1_key: &str,
    x2_key: &str, y2_key: &str,
    max_dist: f64,
    sim: &SimResult,
) -> f64 {
    let x1 = sim.game_state.get(x1_key).and_then(|v| v.as_f64()).unwrap_or(0.0);
    let y1 = sim.game_state.get(y1_key).and_then(|v| v.as_f64()).unwrap_or(0.0);
    let x2 = sim.game_state.get(x2_key).and_then(|v| v.as_f64()).unwrap_or(0.0);
    let y2 = sim.game_state.get(y2_key).and_then(|v| v.as_f64()).unwrap_or(0.0);
    let dist = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
    (1.0 - dist / max_dist).max(0.0)
}

/// 1.0 when `key` equals `target` (within tolerance), 0.0 otherwise.
pub fn score_state_match(key: &str, target: f64, tolerance: f64, sim: &SimResult) -> f64 {
    let actual = sim.game_state.get(key).and_then(|v| v.as_f64()).unwrap_or(f64::NAN);
    if (actual - target).abs() <= tolerance { 1.0 } else { 0.0 }
}

/// Score based on ratio: `target / actual`. 1.0 when actual == target,
/// <1.0 when actual > target. Useful for efficiency metrics.
pub fn score_ratio(key: &str, target: f64, sim: &SimResult) -> f64 {
    let actual = sim.game_state.get(key).and_then(|v| v.as_f64()).unwrap_or(0.0);
    if actual <= 0.0 { return 0.0; }
    (target / actual).min(1.0)
}
