use crate::engine::Engine;
use super::scenario::{GameScenario, ScheduledAction, Assertion};
use super::fitness::{FitnessEvaluator, FitnessResult};

/// A test case in a test harness: a scenario with optional fitness evaluation.
struct HarnessCase {
    scenario: GameScenario,
    fitness: Option<FitnessEvaluator>,
}

/// Individual test result within a harness run.
#[derive(Debug)]
pub struct HarnessEntry {
    pub name: String,
    pub passed: bool,
    pub assertion_failures: usize,
    pub fitness: Option<FitnessResult>,
    pub detail: String,
}

/// Consolidated quality report from running a battery of test cases.
#[derive(Debug)]
pub struct HarnessReport {
    pub entries: Vec<HarnessEntry>,
}

impl HarnessReport {
    /// Number of passing tests.
    pub fn passed(&self) -> usize {
        self.entries.iter().filter(|e| e.passed).count()
    }

    /// Number of failing tests.
    pub fn failed(&self) -> usize {
        self.entries.iter().filter(|e| !e.passed).count()
    }

    /// All tests passed?
    pub fn all_passed(&self) -> bool {
        self.entries.iter().all(|e| e.passed)
    }

    /// Average fitness across all entries with fitness evaluations.
    pub fn avg_fitness(&self) -> Option<f64> {
        let scores: Vec<f64> = self.entries.iter()
            .filter_map(|e| e.fitness.as_ref().map(|f| f.total))
            .collect();
        if scores.is_empty() { None }
        else { Some(scores.iter().sum::<f64>() / scores.len() as f64) }
    }

    /// Machine-readable summary.
    pub fn summary(&self) -> String {
        let total = self.entries.len();
        let pass = self.passed();
        let fail = self.failed();
        let verdict = if self.all_passed() { "PASS" } else { "FAIL" };

        let mut out = format!("Harness: {} ({}/{} passed, {} failed)\n", verdict, pass, total, fail);

        for e in &self.entries {
            let status = if e.passed { "OK" } else { "FAIL" };
            let fitness_str = e.fitness.as_ref()
                .map(|f| format!(" fitness={:.3} [{}]", f.total, f.grade()))
                .unwrap_or_default();
            out.push_str(&format!("  [{}] {}{} — {}\n", status, e.name, fitness_str, e.detail));
        }

        if let Some(avg) = self.avg_fitness() {
            out.push_str(&format!("  Average fitness: {:.3}\n", avg));
        }

        out
    }
}

/// A test harness: run a battery of scenarios and produce a quality report.
///
/// Designed for Claude to quickly assess game quality across multiple
/// test cases with a single call.
pub struct TestHarness {
    setup_fn: fn(&mut Engine),
    update_fn: fn(&mut Engine, f64),
    render_fn: fn(&mut Engine),
    action_dispatch: fn(&mut Engine, &ScheduledAction),
    cases: Vec<HarnessCase>,
}

impl TestHarness {
    pub fn new(
        setup_fn: fn(&mut Engine),
        update_fn: fn(&mut Engine, f64),
        render_fn: fn(&mut Engine),
        action_dispatch: fn(&mut Engine, &ScheduledAction),
    ) -> Self {
        Self {
            setup_fn,
            update_fn,
            render_fn,
            action_dispatch,
            cases: Vec::new(),
        }
    }

    /// Add a test case with actions and assertions.
    pub fn add(
        mut self,
        name: &str,
        actions: Vec<ScheduledAction>,
        frames: u64,
        assertions: Vec<Assertion>,
    ) -> Self {
        self.cases.push(HarnessCase {
            scenario: GameScenario {
                name: name.into(),
                width: 480,
                height: 720,
                setup_fn: self.setup_fn,
                update_fn: self.update_fn,
                render_fn: self.render_fn,
                action_dispatch: self.action_dispatch,
                actions,
                total_frames: frames,
                assertions,
            },
            fitness: None,
        });
        self
    }

    /// Add a test case with fitness evaluation.
    pub fn add_with_fitness(
        mut self,
        name: &str,
        actions: Vec<ScheduledAction>,
        frames: u64,
        assertions: Vec<Assertion>,
        evaluator: FitnessEvaluator,
    ) -> Self {
        self.cases.push(HarnessCase {
            scenario: GameScenario {
                name: name.into(),
                width: 480,
                height: 720,
                setup_fn: self.setup_fn,
                update_fn: self.update_fn,
                render_fn: self.render_fn,
                action_dispatch: self.action_dispatch,
                actions,
                total_frames: frames,
                assertions,
            },
            fitness: Some(evaluator),
        });
        self
    }

    /// Run all cases and produce a consolidated report.
    pub fn run(self) -> HarnessReport {
        let entries = self.cases.into_iter().map(|case| {
            let result = case.scenario.run();
            let assertion_failures = result.outcomes.iter().filter(|o| !o.passed).count();
            let fitness = case.fitness.map(|f| f.evaluate(&result.sim));
            let passed = result.all_passed();

            let detail = if passed {
                format!("{} assertions passed", result.outcomes.len())
            } else {
                let fails: Vec<String> = result.outcomes.iter()
                    .filter(|o| !o.passed)
                    .map(|o| o.detail.clone())
                    .collect();
                fails.join("; ")
            };

            HarnessEntry {
                name: result.name,
                passed,
                assertion_failures,
                fitness,
                detail,
            }
        }).collect();

        HarnessReport { entries }
    }
}
