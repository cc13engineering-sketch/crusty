use crate::engine::Engine;
use crate::frame_metrics::FrameMetrics;
use std::collections::HashMap;
use crate::game_state::StateValue;

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
/// Runs the full game loop (setup → tick → game_update → render) without
/// any browser or WASM dependencies. Designed for `cargo test` and CLI use.
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

    /// Run a complete simulation.
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
        self.snapshot(frames)
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
        self.snapshot(frames)
    }

    fn snapshot(&self, frames_run: u64) -> SimResult {
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
        }
    }
}
