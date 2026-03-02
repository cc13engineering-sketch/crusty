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

// ─── Built-in S-League scoring functions ────────────────────────────

/// 1.0 if ball sunk (phase==2), 0.0 otherwise.
pub fn score_hole_completion(sim: &SimResult) -> f64 {
    let phase = sim.game_state.get("tl_phase").and_then(|v| v.as_f64()).unwrap_or(0.0);
    if (phase - 2.0).abs() < 0.01 { 1.0 } else { 0.0 }
}

/// 1.0 for hole-in-one at par 3, scales down as strokes increase.
pub fn score_stroke_efficiency(sim: &SimResult) -> f64 {
    let strokes = sim.game_state.get("strokes").and_then(|v| v.as_f64()).unwrap_or(0.0);
    if strokes <= 0.0 { return 0.0; }
    let par = 3.0;
    let efficiency = par / strokes;
    efficiency.min(1.0)
}

/// 1.0 when ball is at the hole, degrades linearly with distance (up to 500px).
pub fn score_proximity_to_hole(sim: &SimResult) -> f64 {
    let bx = sim.game_state.get("ball_x").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let by = sim.game_state.get("ball_y").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let hx = sim.game_state.get("hole_x").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let hy = sim.game_state.get("hole_y").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let dx = bx - hx;
    let dy = by - hy;
    let dist = (dx * dx + dy * dy).sqrt();
    (1.0 - dist / 500.0).max(0.0)
}
