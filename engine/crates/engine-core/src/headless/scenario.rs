use crate::engine::Engine;
use super::runner::HeadlessRunner;

/// A timed input action to inject during simulation.
#[derive(Clone, Debug)]
pub enum ScheduledAction {
    PointerDown { frame: u64, x: f64, y: f64 },
    PointerMove { frame: u64, x: f64, y: f64 },
    PointerUp { frame: u64, x: f64, y: f64 },
}

impl ScheduledAction {
    fn frame(&self) -> u64 {
        match self {
            Self::PointerDown { frame, .. } => *frame,
            Self::PointerMove { frame, .. } => *frame,
            Self::PointerUp { frame, .. } => *frame,
        }
    }
}

/// An assertion to check after simulation completes.
#[derive(Clone, Debug)]
pub enum Assertion {
    /// Assert a game state f64 equals expected (within tolerance).
    StateEquals { key: String, expected: f64, tolerance: f64 },
    /// Assert a game state f64 is within a range.
    StateInRange { key: String, min: f64, max: f64 },
    /// Assert the framebuffer hash matches exactly.
    FramebufferHash { expected: u64 },
}

/// Outcome of a single assertion.
#[derive(Clone, Debug)]
pub struct AssertionOutcome {
    pub assertion: Assertion,
    pub passed: bool,
    pub detail: String,
}

/// Result of running a GameScenario.
#[derive(Clone, Debug)]
pub struct ScenarioResult {
    pub name: String,
    pub sim: super::runner::SimResult,
    pub outcomes: Vec<AssertionOutcome>,
}

impl ScenarioResult {
    pub fn all_passed(&self) -> bool {
        self.outcomes.iter().all(|o| o.passed)
    }

    pub fn failure_report(&self) -> String {
        let failures: Vec<&AssertionOutcome> = self.outcomes.iter().filter(|o| !o.passed).collect();
        if failures.is_empty() {
            return format!("[{}] all assertions passed", self.name);
        }
        let mut report = format!("[{}] {} assertion(s) failed:\n", self.name, failures.len());
        for f in &failures {
            report.push_str(&format!("  - {}\n", f.detail));
        }
        report
    }
}

/// A declarative test scenario: setup, scheduled inputs, assertions.
pub struct GameScenario {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub setup_fn: fn(&mut Engine),
    pub update_fn: fn(&mut Engine, f64),
    pub render_fn: fn(&mut Engine),
    pub actions: Vec<ScheduledAction>,
    pub total_frames: u64,
    pub assertions: Vec<Assertion>,
}

impl GameScenario {
    pub fn run(&self) -> ScenarioResult {
        let mut runner = HeadlessRunner::new(self.width, self.height);
        let mut sorted_actions = self.actions.clone();
        sorted_actions.sort_by_key(|a| a.frame());

        let update_fn = self.update_fn;
        let render_fn = self.render_fn;

        let sim = runner.run_with_frame_cb(
            self.setup_fn,
            |engine, frame, dt| {
                // Inject any actions scheduled for this frame
                for action in &sorted_actions {
                    if action.frame() == frame {
                        dispatch_action(engine, action);
                    }
                }
                update_fn(engine, dt);
                render_fn(engine);
            },
            self.total_frames,
        );

        // Evaluate assertions
        let outcomes = self.assertions.iter().map(|a| evaluate(a, &sim)).collect();

        ScenarioResult {
            name: self.name.clone(),
            sim,
            outcomes,
        }
    }
}

fn dispatch_action(engine: &mut Engine, action: &ScheduledAction) {
    use crate::trap_links_demo;
    match action {
        ScheduledAction::PointerDown { x, y, .. } => {
            trap_links_demo::on_pointer_down(engine, *x, *y);
        }
        ScheduledAction::PointerMove { x, y, .. } => {
            trap_links_demo::on_pointer_move(engine, *x, *y);
        }
        ScheduledAction::PointerUp { x, y, .. } => {
            trap_links_demo::on_pointer_up(engine, *x, *y);
        }
    }
}

fn evaluate(assertion: &Assertion, sim: &super::runner::SimResult) -> AssertionOutcome {
    match assertion {
        Assertion::StateEquals { key, expected, tolerance } => {
            let actual = sim.game_state.get(key)
                .and_then(|v| v.as_f64())
                .unwrap_or(f64::NAN);
            let passed = (actual - expected).abs() <= *tolerance;
            AssertionOutcome {
                assertion: assertion.clone(),
                passed,
                detail: format!(
                    "StateEquals(\"{}\") expected={} actual={} tol={}",
                    key, expected, actual, tolerance
                ),
            }
        }
        Assertion::StateInRange { key, min, max } => {
            let actual = sim.game_state.get(key)
                .and_then(|v| v.as_f64())
                .unwrap_or(f64::NAN);
            let passed = actual >= *min && actual <= *max;
            AssertionOutcome {
                assertion: assertion.clone(),
                passed,
                detail: format!(
                    "StateInRange(\"{}\") expected=[{}, {}] actual={}",
                    key, min, max, actual
                ),
            }
        }
        Assertion::FramebufferHash { expected } => {
            let passed = sim.framebuffer_hash == *expected;
            AssertionOutcome {
                assertion: assertion.clone(),
                passed,
                detail: format!(
                    "FramebufferHash expected={:#x} actual={:#x}",
                    expected, sim.framebuffer_hash
                ),
            }
        }
    }
}
