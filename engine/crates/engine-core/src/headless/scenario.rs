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
    pub fn frame(&self) -> u64 {
        match self {
            Self::PointerDown { frame, .. } => *frame,
            Self::PointerMove { frame, .. } => *frame,
            Self::PointerUp { frame, .. } => *frame,
        }
    }

    /// Get the (x, y) coordinates of this action.
    pub fn coords(&self) -> (f64, f64) {
        match self {
            Self::PointerDown { x, y, .. } => (*x, *y),
            Self::PointerMove { x, y, .. } => (*x, *y),
            Self::PointerUp { x, y, .. } => (*x, *y),
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
    /// Assert a game state f64 is greater than a threshold.
    StateGreaterThan { key: String, threshold: f64 },
    /// Assert a game state f64 is less than a threshold.
    StateLessThan { key: String, threshold: f64 },
    /// Assert the framebuffer hash matches exactly.
    FramebufferHash { expected: u64 },
    /// Assert the framebuffer hash differs from a previous value.
    FramebufferChanged { previous: u64 },
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
///
/// Game-agnostic: supply your own `action_dispatch` to route ScheduledActions
/// to your game's input handlers. Use [`dispatch_noop`] if the scenario has no actions.
pub struct GameScenario {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub setup_fn: fn(&mut Engine),
    pub update_fn: fn(&mut Engine, f64),
    pub render_fn: fn(&mut Engine),
    /// Routes a ScheduledAction to the game's input handlers.
    pub action_dispatch: fn(&mut Engine, &ScheduledAction),
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
        let action_dispatch = self.action_dispatch;

        let sim = runner.run_with_frame_cb(
            self.setup_fn,
            |engine, frame, dt| {
                // Inject any actions scheduled for this frame
                for action in &sorted_actions {
                    if action.frame() == frame {
                        action_dispatch(engine, action);
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

/// No-op action dispatcher for scenarios without input.
pub fn dispatch_noop(_engine: &mut Engine, _action: &ScheduledAction) {}

/// Builder pattern for GameScenario to reduce boilerplate.
///
/// Requires game functions (setup, update, render, action_dispatch) to be set
/// once, then allows building multiple scenarios with different inputs and
/// assertions without repeating the function pointers.
pub struct ScenarioBuilder {
    width: u32,
    height: u32,
    setup_fn: fn(&mut Engine),
    update_fn: fn(&mut Engine, f64),
    render_fn: fn(&mut Engine),
    action_dispatch: fn(&mut Engine, &ScheduledAction),
}

impl ScenarioBuilder {
    /// Create a builder with game function pointers.
    pub fn new(
        setup_fn: fn(&mut Engine),
        update_fn: fn(&mut Engine, f64),
        render_fn: fn(&mut Engine),
        action_dispatch: fn(&mut Engine, &ScheduledAction),
    ) -> Self {
        Self {
            width: 480,
            height: 720,
            setup_fn,
            update_fn,
            render_fn,
            action_dispatch,
        }
    }

    /// Override viewport dimensions (default 480x720).
    pub fn viewport(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Build a scenario with the given name, actions, frames, and assertions.
    pub fn build(
        &self,
        name: &str,
        actions: Vec<ScheduledAction>,
        total_frames: u64,
        assertions: Vec<Assertion>,
    ) -> GameScenario {
        GameScenario {
            name: name.into(),
            width: self.width,
            height: self.height,
            setup_fn: self.setup_fn,
            update_fn: self.update_fn,
            render_fn: self.render_fn,
            action_dispatch: self.action_dispatch,
            actions,
            total_frames,
            assertions,
        }
    }

    /// Build and immediately run a scenario. Returns the ScenarioResult.
    pub fn run(
        &self,
        name: &str,
        actions: Vec<ScheduledAction>,
        total_frames: u64,
        assertions: Vec<Assertion>,
    ) -> ScenarioResult {
        self.build(name, actions, total_frames, assertions).run()
    }

    /// Build and run an idle scenario (no input, no assertions).
    /// Useful for capturing baselines.
    pub fn run_idle(&self, name: &str, total_frames: u64) -> ScenarioResult {
        self.build(name, vec![], total_frames, vec![]).run()
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
        Assertion::StateGreaterThan { key, threshold } => {
            let actual = sim.game_state.get(key)
                .and_then(|v| v.as_f64())
                .unwrap_or(f64::NAN);
            let passed = actual > *threshold;
            AssertionOutcome {
                assertion: assertion.clone(),
                passed,
                detail: format!(
                    "StateGreaterThan(\"{}\") threshold={} actual={}",
                    key, threshold, actual
                ),
            }
        }
        Assertion::StateLessThan { key, threshold } => {
            let actual = sim.game_state.get(key)
                .and_then(|v| v.as_f64())
                .unwrap_or(f64::NAN);
            let passed = actual < *threshold;
            AssertionOutcome {
                assertion: assertion.clone(),
                passed,
                detail: format!(
                    "StateLessThan(\"{}\") threshold={} actual={}",
                    key, threshold, actual
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
        Assertion::FramebufferChanged { previous } => {
            let passed = sim.framebuffer_hash != *previous;
            AssertionOutcome {
                assertion: assertion.clone(),
                passed,
                detail: format!(
                    "FramebufferChanged previous={:#x} actual={:#x}",
                    previous, sim.framebuffer_hash
                ),
            }
        }
    }
}
