use crate::engine::Engine;

/// The contract between a game and the engine.
///
/// Games implement this trait. The engine owns timing, input application,
/// and determinism. A `Simulation` provides three hooks:
///
/// - **`setup`** — called once after `Engine::reset(seed)` to initialize
///   game state (spawn entities, configure world, etc.).
/// - **`step`** — called once per frame to advance game logic. No `dt`
///   argument: the simulation always advances by one fixed tick (1/60s).
///   Input has already been applied via `Engine::apply_input`.
/// - **`render`** — called once per frame to draw into the framebuffer.
///   Separated from `step` so that turbo mode can skip rendering.
pub trait Simulation {
    /// Initialize game state for a new run. The engine is already reset and seeded.
    fn setup(&mut self, engine: &mut Engine);

    /// Advance one frame of game logic.
    fn step(&mut self, engine: &mut Engine);

    /// Render the current state into the engine's framebuffer.
    fn render(&self, engine: &mut Engine);
}
