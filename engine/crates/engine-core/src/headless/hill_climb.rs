use crate::engine::Engine;
use super::runner::{HeadlessRunner, SimResult};
use super::scenario::ScheduledAction;

/// A parameter range for hill climbing.
#[derive(Clone, Debug)]
pub struct ParamRange {
    /// Game state key to override.
    pub key: String,
    /// Minimum value.
    pub min: f64,
    /// Maximum value.
    pub max: f64,
    /// Initial step size for exploration.
    pub step: f64,
}

impl ParamRange {
    pub fn new(key: &str, min: f64, max: f64, step: f64) -> Self {
        Self {
            key: key.to_string(),
            min,
            max,
            step,
        }
    }

    fn clamp(&self, v: f64) -> f64 {
        v.clamp(self.min, self.max)
    }

    fn midpoint(&self) -> f64 {
        (self.min + self.max) / 2.0
    }
}

/// A single evaluated candidate in the search.
#[derive(Clone, Debug)]
pub struct Candidate {
    /// Parameter values for this candidate.
    pub params: Vec<(String, f64)>,
    /// Fitness score (higher is better).
    pub fitness: f64,
    /// The simulation result.
    pub sim: SimResult,
}

/// Result of hill climbing optimization.
#[derive(Clone, Debug)]
pub struct ClimbResult {
    /// Best candidate found.
    pub best: Candidate,
    /// Number of iterations performed.
    pub iterations: usize,
    /// Total simulations run.
    pub evaluations: usize,
    /// History of best fitness per iteration.
    pub history: Vec<f64>,
}

impl ClimbResult {
    /// Compact summary for AI consumption.
    pub fn summary(&self) -> String {
        let params: Vec<String> = self.best.params.iter()
            .map(|(k, v)| format!("{}={:.4}", k, v))
            .collect();
        format!(
            "HillClimb: fitness={:.4} after {} iterations ({} evals) | {}",
            self.best.fitness,
            self.iterations,
            self.evaluations,
            params.join(", ")
        )
    }

    /// True if fitness improved over the course of optimization.
    pub fn improved(&self) -> bool {
        self.history.len() >= 2 && self.history.last() > self.history.first()
    }
}

/// Iterative hill-climbing optimizer for game parameters.
///
/// Given parameter ranges and a fitness function, searches for the parameter
/// combination that maximizes fitness. Game-agnostic: supply your own game
/// functions and scoring.
///
/// Algorithm: coordinate descent with shrinking step sizes. Each iteration
/// tries +step and -step for each parameter, keeping the better value.
/// Step size halves when no improvement is found.
pub struct HillClimber {
    setup_fn: fn(&mut Engine),
    update_fn: fn(&mut Engine, f64),
    render_fn: fn(&mut Engine),
    action_dispatch: fn(&mut Engine, &ScheduledAction),
    actions: Vec<ScheduledAction>,
    frames: u64,
    params: Vec<ParamRange>,
    fitness_fn: fn(&SimResult) -> f64,
    max_iterations: usize,
    min_step: f64,
}

impl HillClimber {
    pub fn new(
        setup_fn: fn(&mut Engine),
        update_fn: fn(&mut Engine, f64),
        render_fn: fn(&mut Engine),
        action_dispatch: fn(&mut Engine, &ScheduledAction),
        fitness_fn: fn(&SimResult) -> f64,
    ) -> Self {
        Self {
            setup_fn,
            update_fn,
            render_fn,
            action_dispatch,
            actions: Vec::new(),
            frames: 120,
            params: Vec::new(),
            fitness_fn,
            max_iterations: 20,
            min_step: 0.01,
        }
    }

    pub fn with_actions(mut self, actions: Vec<ScheduledAction>) -> Self {
        self.actions = actions;
        self
    }

    pub fn with_frames(mut self, frames: u64) -> Self {
        self.frames = frames;
        self
    }

    pub fn with_param(mut self, param: ParamRange) -> Self {
        self.params.push(param);
        self
    }

    pub fn with_max_iterations(mut self, n: usize) -> Self {
        self.max_iterations = n;
        self
    }

    pub fn with_min_step(mut self, s: f64) -> Self {
        self.min_step = s;
        self
    }

    /// Run the optimization. Returns the best parameters found.
    pub fn run(&self) -> ClimbResult {
        // Start at midpoints
        let mut current: Vec<f64> = self.params.iter().map(|p| p.midpoint()).collect();
        let mut steps: Vec<f64> = self.params.iter().map(|p| p.step).collect();
        let mut best_sim = self.evaluate(&current);
        let mut best_fitness = (self.fitness_fn)(&best_sim);
        let mut evaluations = 1usize;
        let mut history = vec![best_fitness];

        for iter in 0..self.max_iterations {
            let mut improved = false;

            // Check if all steps are below minimum
            if steps.iter().all(|s| *s < self.min_step) {
                return self.make_result(current, best_fitness, best_sim, iter, evaluations, history);
            }

            for i in 0..self.params.len() {
                let original = current[i];

                // Try +step
                current[i] = self.params[i].clamp(original + steps[i]);
                let sim_plus = self.evaluate(&current);
                let fit_plus = (self.fitness_fn)(&sim_plus);
                evaluations += 1;

                // Try -step
                current[i] = self.params[i].clamp(original - steps[i]);
                let sim_minus = self.evaluate(&current);
                let fit_minus = (self.fitness_fn)(&sim_minus);
                evaluations += 1;

                // Keep the best
                if fit_plus > best_fitness && fit_plus >= fit_minus {
                    current[i] = self.params[i].clamp(original + steps[i]);
                    best_fitness = fit_plus;
                    best_sim = sim_plus;
                    improved = true;
                } else if fit_minus > best_fitness {
                    current[i] = self.params[i].clamp(original - steps[i]);
                    best_fitness = fit_minus;
                    best_sim = sim_minus;
                    improved = true;
                } else {
                    current[i] = original;
                }
            }

            history.push(best_fitness);

            if !improved {
                // Shrink step sizes
                for s in &mut steps {
                    *s *= 0.5;
                }
            }
        }

        self.make_result(current, best_fitness, best_sim, self.max_iterations, evaluations, history)
    }

    fn evaluate(&self, values: &[f64]) -> SimResult {
        let mut runner = HeadlessRunner::new(480, 720);
        let mut sorted_actions = self.actions.clone();
        sorted_actions.sort_by_key(|a| a.frame());

        let setup_fn = self.setup_fn;
        let update_fn = self.update_fn;
        let render_fn = self.render_fn;
        let action_dispatch = self.action_dispatch;
        let params = self.params.clone();
        let vals = values.to_vec();

        runner.run_with_frame_cb(
            |engine| {
                setup_fn(engine);
                for (param, value) in params.iter().zip(vals.iter()) {
                    engine.global_state.set_f64(&param.key, *value);
                }
            },
            |engine, frame, dt| {
                for action in &sorted_actions {
                    if action.frame() == frame {
                        action_dispatch(engine, action);
                    }
                }
                update_fn(engine, dt);
                render_fn(engine);
            },
            self.frames,
        )
    }

    fn make_result(
        &self,
        values: Vec<f64>,
        fitness: f64,
        sim: SimResult,
        iterations: usize,
        evaluations: usize,
        history: Vec<f64>,
    ) -> ClimbResult {
        let params: Vec<(String, f64)> = self.params.iter()
            .zip(values.iter())
            .map(|(p, v)| (p.key.clone(), *v))
            .collect();

        ClimbResult {
            best: Candidate { params, fitness, sim },
            iterations,
            evaluations,
            history,
        }
    }
}
