use crate::engine::Engine;
use crate::frame_metrics::FrameMetrics;
use crate::input_frame::InputFrame;
use crate::policy::Policy;
use crate::simulation::Simulation;
use std::collections::HashMap;
use crate::game_state::StateValue;

/// Configuration for a headless simulation run.
#[derive(Clone, Debug)]
pub struct RunConfig {
    /// Skip rendering for faster throughput.
    pub turbo: bool,
    /// Capture per-frame state hashes.
    pub capture_state_hashes: bool,
}

impl Default for RunConfig {
    fn default() -> Self {
        Self {
            turbo: false,
            capture_state_hashes: false,
        }
    }
}

/// Result of running a headless simulation.
#[derive(Clone, Debug)]
pub struct SimResult {
    /// Number of frames that were simulated.
    pub frames_run: u64,
    /// Final frame metrics snapshot.
    pub final_metrics: FrameMetrics,
    /// Snapshot of all game state key-value pairs.
    pub game_state: HashMap<String, StateValue>,
    /// FNV-1a hash of the final framebuffer pixels.
    pub framebuffer_hash: u64,
    /// Total simulated time in seconds.
    pub elapsed_sim_time: f64,
    /// Final simulation state hash.
    pub state_hash: u64,
    /// Per-frame state hashes (only populated if RunConfig::capture_state_hashes).
    pub state_hashes: Vec<u64>,
}

impl SimResult {
    /// Convenience accessor for a numeric game state value.
    ///
    /// Shorthand for `self.game_state.get(key).and_then(|v| v.as_f64())`.
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.game_state.get(key).and_then(|v| v.as_f64())
    }
}

/// Headless simulation wrapper around Engine.
///
/// Runs the full game loop without any browser or WASM dependencies.
/// Designed for `cargo test` and CLI use.
///
/// # New API (Simulation trait)
///
/// ```ignore
/// let mut runner = HeadlessRunner::new(480, 270);
/// let mut game = MyGame::new();
/// let result = runner.run_sim(&mut game, 42, &[], RunConfig::default());
/// ```
///
/// # Legacy API (function pointers)
///
/// The old `run()` and `run_with_frame_cb()` methods are preserved for
/// backward compatibility with existing headless modules. They will be
/// removed once all modules are migrated to the Simulation trait.
pub struct HeadlessRunner {
    pub engine: Engine,
}

impl HeadlessRunner {
    /// Create a runner with the given viewport dimensions.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            engine: Engine::new(width, height),
        }
    }

    // ─── New Simulation-trait API ────────────────────────────────────

    /// Run a simulation with a fixed sequence of input frames.
    ///
    /// Resets the engine with the given seed, calls `sim.setup()`, then
    /// runs one frame per input (applying input → tick → step → render).
    /// If `inputs` is shorter than the desired frame count, remaining
    /// frames get empty input. Use `run_sim_frames` if you want to
    /// specify a frame count independently of input length.
    pub fn run_sim<S: Simulation>(
        &mut self,
        sim: &mut S,
        seed: u64,
        inputs: &[InputFrame],
        config: RunConfig,
    ) -> SimResult {
        self.run_sim_frames(sim, seed, inputs, inputs.len().max(1) as u64, config)
    }

    /// Run a simulation for a specific number of frames.
    ///
    /// Input frames are consumed in order; frames beyond the input
    /// length receive empty input.
    pub fn run_sim_frames<S: Simulation>(
        &mut self,
        sim: &mut S,
        seed: u64,
        inputs: &[InputFrame],
        frames: u64,
        config: RunConfig,
    ) -> SimResult {
        let dt = 1.0 / 60.0;
        self.engine.reset(seed);
        sim.setup(&mut self.engine);

        let mut state_hashes = if config.capture_state_hashes {
            Vec::with_capacity(frames as usize)
        } else {
            Vec::new()
        };

        let empty = InputFrame::default();
        for i in 0..frames {
            self.engine.tick(dt);
            // Apply input AFTER tick (which clears previous transient input
            // via end_frame), so that sim.step() sees fresh input.
            let input = inputs.get(i as usize).unwrap_or(&empty);
            self.engine.apply_input(input);
            sim.step(&mut self.engine);
            if !config.turbo {
                sim.render(&mut self.engine);
            }
            if config.capture_state_hashes {
                state_hashes.push(self.engine.state_hash());
            }
        }

        self.snapshot(frames, state_hashes)
    }

    /// Run a simulation driven by a policy for a fixed number of frames.
    ///
    /// Each frame: tick → observe → policy.next_input → apply_input → step → render.
    /// The policy sees the observation *before* its input is applied,
    /// matching how a real agent would perceive-then-act.
    pub fn run_with_policy<S: Simulation, P: Policy>(
        &mut self,
        sim: &mut S,
        policy: &mut P,
        seed: u64,
        frames: u64,
        config: RunConfig,
    ) -> SimResult {
        let dt = 1.0 / 60.0;
        self.engine.reset(seed);
        sim.setup(&mut self.engine);

        let mut state_hashes = if config.capture_state_hashes {
            Vec::with_capacity(frames as usize)
        } else {
            Vec::new()
        };

        for _ in 0..frames {
            self.engine.tick(dt);

            let obs = if config.turbo {
                self.engine.observe_turbo()
            } else {
                self.engine.observe()
            };
            let input = policy.next_input(&obs);
            self.engine.apply_input(&input);

            sim.step(&mut self.engine);
            if !config.turbo {
                sim.render(&mut self.engine);
            }
            if config.capture_state_hashes {
                state_hashes.push(self.engine.state_hash());
            }
        }

        self.snapshot(frames, state_hashes)
    }

    /// Run a simulation for a specific number of frames, capturing
    /// specified game-state keys at every frame.
    ///
    /// Returns `(SimResult, captured)` where `captured` is a `Vec` with
    /// one entry per frame. Each entry is a `Vec<(String, f64)>` of the
    /// requested keys and their values at that frame (keys that are
    /// missing or non-numeric are omitted from that frame's entry).
    pub fn run_with_capture<S: Simulation>(
        &mut self,
        sim: &mut S,
        seed: u64,
        inputs: &[InputFrame],
        frames: u64,
        config: RunConfig,
        capture_keys: &[String],
    ) -> (SimResult, Vec<Vec<(String, f64)>>) {
        let dt = 1.0 / 60.0;
        self.engine.reset(seed);
        sim.setup(&mut self.engine);

        let mut state_hashes = if config.capture_state_hashes {
            Vec::with_capacity(frames as usize)
        } else {
            Vec::new()
        };

        let mut captured: Vec<Vec<(String, f64)>> = Vec::with_capacity(frames as usize);

        let empty = InputFrame::default();
        for i in 0..frames {
            self.engine.tick(dt);
            let input = inputs.get(i as usize).unwrap_or(&empty);
            self.engine.apply_input(input);
            sim.step(&mut self.engine);
            if !config.turbo {
                sim.render(&mut self.engine);
            }
            if config.capture_state_hashes {
                state_hashes.push(self.engine.state_hash());
            }

            // Capture requested keys from global game state
            let mut frame_capture = Vec::with_capacity(capture_keys.len());
            for key in capture_keys {
                if let Some(val) = self.engine.global_state.get_f64(key) {
                    frame_capture.push((key.clone(), val));
                }
            }
            captured.push(frame_capture);
        }

        (self.snapshot(frames, state_hashes), captured)
    }

    // ─── Legacy function-pointer API ─────────────────────────────────

    /// Run a complete simulation (legacy API).
    ///
    /// - `setup`: called once to initialize the game state
    /// - `game_update`: called each frame with (engine, dt_seconds)
    /// - `game_render`: called each frame to draw
    /// - `frames`: total number of frames to simulate (at 60 fps)
    pub fn run<S, U, R>(
        &mut self,
        setup: S,
        game_update: U,
        game_render: R,
        frames: u64,
    ) -> SimResult
    where
        S: FnOnce(&mut Engine),
        U: Fn(&mut Engine, f64),
        R: Fn(&mut Engine),
    {
        setup(&mut self.engine);
        let dt = 1.0 / 60.0;
        for _ in 0..frames {
            self.engine.tick(dt);
            game_update(&mut self.engine, dt);
            game_render(&mut self.engine);
        }
        self.snapshot(frames, Vec::new())
    }

    /// Run with frame-by-frame callbacks that receive the current frame number.
    /// This is used by GameScenario to inject actions at specific frames.
    pub fn run_with_frame_cb<S, F>(
        &mut self,
        setup: S,
        per_frame: F,
        frames: u64,
    ) -> SimResult
    where
        S: FnOnce(&mut Engine),
        F: Fn(&mut Engine, u64, f64),
    {
        setup(&mut self.engine);
        let dt = 1.0 / 60.0;
        for frame in 0..frames {
            self.engine.tick(dt);
            per_frame(&mut self.engine, frame, dt);
        }
        self.snapshot(frames, Vec::new())
    }

    // ─── Internal ────────────────────────────────────────────────────

    fn snapshot(&self, frames_run: u64, state_hashes: Vec<u64>) -> SimResult {
        let game_state: HashMap<String, StateValue> = self
            .engine
            .global_state
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();

        SimResult {
            frames_run,
            final_metrics: self.engine.frame_metrics.clone(),
            game_state,
            framebuffer_hash: super::framebuffer_hash(&self.engine.framebuffer),
            elapsed_sim_time: self.engine.time,
            state_hash: self.engine.state_hash(),
            state_hashes,
        }
    }
}
