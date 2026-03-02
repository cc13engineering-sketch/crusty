use crate::engine::Engine;
use super::scenario::ScheduledAction;
use super::sweep::{SweepConfig, SweepReport, run_sweep};
use super::fitness::{FitnessEvaluator, FitnessResult};
use super::regression::{RegressionSuite, RegressionBaseline, DiffReport};
/// A complete experiment: sweep + fitness evaluation + optional regression check.
///
/// Designed for AI-driven iteration: define parameters to explore, fitness
/// criteria to optimize, and regression baselines to protect. Run once,
/// get a structured verdict.
pub struct Experiment {
    pub name: String,
    setup_fn: fn(&mut Engine),
    update_fn: fn(&mut Engine, f64),
    render_fn: fn(&mut Engine),
    action_dispatch: fn(&mut Engine, &ScheduledAction),
    actions: Vec<ScheduledAction>,
    configs: Vec<SweepConfig>,
    frames: u64,
    evaluator: Option<FitnessEvaluator>,
    baseline: Option<RegressionBaseline>,
    regression_keys: Vec<String>,
}

/// Result of running an experiment.
#[derive(Debug)]
pub struct ExperimentResult {
    pub name: String,
    /// The sweep report with all simulation results.
    pub sweep: SweepReport,
    /// Fitness rankings (best first). Empty if no evaluator was set.
    pub rankings: Vec<(String, FitnessResult)>,
    /// Regression diff against baseline. None if no baseline was set.
    pub regression: Option<DiffReport>,
}

impl ExperimentResult {
    /// Get the best-performing configuration label and fitness.
    pub fn best(&self) -> Option<(&str, &FitnessResult)> {
        self.rankings.first().map(|(l, f)| (l.as_str(), f))
    }

    /// Get the worst-performing configuration label and fitness.
    pub fn worst(&self) -> Option<(&str, &FitnessResult)> {
        self.rankings.last().map(|(l, f)| (l.as_str(), f))
    }

    /// True if no regressions were detected (or no baseline was set).
    pub fn regression_ok(&self) -> bool {
        self.regression.as_ref().map_or(true, |d| !d.has_regressions())
    }

    /// Compact machine-readable summary for AI consumption.
    pub fn summary(&self) -> String {
        let mut out = format!("Experiment: {}\n", self.name);
        out.push_str(&format!("  Configurations: {}\n", self.sweep.results.len()));

        if let Some((label, fitness)) = self.best() {
            out.push_str(&format!(
                "  Best: [{}] score={:.3} grade={}\n",
                label, fitness.total, fitness.grade()
            ));
        }
        if let Some((label, fitness)) = self.worst() {
            out.push_str(&format!(
                "  Worst: [{}] score={:.3} grade={}\n",
                label, fitness.total, fitness.grade()
            ));
        }

        if let Some(diff) = &self.regression {
            out.push_str(&format!("  Regression: {}\n", diff.verdict()));
        }

        out
    }
}

impl Experiment {
    /// Create a new experiment with game functions and a shared action sequence.
    pub fn new(
        name: &str,
        setup_fn: fn(&mut Engine),
        update_fn: fn(&mut Engine, f64),
        render_fn: fn(&mut Engine),
        action_dispatch: fn(&mut Engine, &ScheduledAction),
    ) -> Self {
        Self {
            name: name.to_string(),
            setup_fn,
            update_fn,
            render_fn,
            action_dispatch,
            actions: Vec::new(),
            configs: Vec::new(),
            frames: 120,
            evaluator: None,
            baseline: None,
            regression_keys: Vec::new(),
        }
    }

    /// Set the input actions for the experiment.
    pub fn with_actions(mut self, actions: Vec<ScheduledAction>) -> Self {
        self.actions = actions;
        self
    }

    /// Set the parameter configurations to sweep.
    pub fn with_configs(mut self, configs: Vec<SweepConfig>) -> Self {
        self.configs = configs;
        self
    }

    /// Set the number of frames per simulation.
    pub fn with_frames(mut self, frames: u64) -> Self {
        self.frames = frames;
        self
    }

    /// Set the fitness evaluator for ranking results.
    pub fn with_fitness(mut self, evaluator: FitnessEvaluator) -> Self {
        self.evaluator = Some(evaluator);
        self
    }

    /// Set a regression baseline to diff against.
    pub fn with_baseline(mut self, baseline: RegressionBaseline, keys: &[&str]) -> Self {
        self.baseline = Some(baseline);
        self.regression_keys = keys.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Run the experiment: sweep, evaluate fitness, check regression.
    pub fn run(self) -> ExperimentResult {
        let configs = if self.configs.is_empty() {
            vec![SweepConfig {
                label: "default".into(),
                overrides: vec![],
            }]
        } else {
            self.configs
        };

        let sweep = run_sweep(
            self.setup_fn,
            self.update_fn,
            self.render_fn,
            self.action_dispatch,
            &self.actions,
            &configs,
            self.frames,
        );

        let rankings = if let Some(evaluator) = &self.evaluator {
            evaluator.rank_sweep(&sweep)
        } else {
            Vec::new()
        };

        let regression = self.baseline.map(|baseline| {
            // Build a lightweight regression suite from the first sweep config
            let keys_ref: Vec<&str> = self.regression_keys.iter().map(|s| s.as_str()).collect();
            let suite = RegressionSuite::new(&keys_ref)
                .add(super::scenario::GameScenario {
                    name: self.name.clone(),
                    width: 480,
                    height: 720,
                    setup_fn: self.setup_fn,
                    update_fn: self.update_fn,
                    render_fn: self.render_fn,
                    action_dispatch: self.action_dispatch,
                    actions: self.actions.clone(),
                    total_frames: self.frames,
                    assertions: vec![],
                });
            suite.diff_against(&[baseline])
        });

        ExperimentResult {
            name: self.name,
            sweep,
            rankings,
            regression,
        }
    }
}
