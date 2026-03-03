use crate::engine::Engine;
use crate::frame_metrics::FrameMetrics;
use crate::input_frame::InputFrame;
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
            let input = inputs.get(i as usize).unwrap_or(&empty);
            self.engine.apply_input(input);
            self.engine.tick(dt);
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
