use crate::game_state::GameState;

/// A zero-allocation observation of the engine's current state.
///
/// Borrows from the engine. Used by policies to decide what input to
/// produce next. Intentionally read-only — policies observe, they
/// don't mutate.
pub struct Observation<'a> {
    /// Current frame number.
    pub frame: u64,
    /// Simulation state hash at this frame.
    pub state_hash: u64,
    /// Reference to the global game state key-value store.
    pub game_state: &'a GameState,
    /// Number of alive entities.
    pub entity_count: usize,
    /// Framebuffer pixel data (None in turbo mode).
    pub framebuffer: Option<&'a [u8]>,
}

impl crate::engine::Engine {
    /// Create a read-only observation of the current engine state.
    pub fn observe(&self) -> Observation<'_> {
        Observation {
            frame: self.frame,
            state_hash: self.state_hash(),
            game_state: &self.global_state,
            entity_count: self.world.entity_count(),
            framebuffer: Some(&self.framebuffer.pixels),
        }
    }

    /// Create an observation without framebuffer data (for turbo mode).
    pub fn observe_turbo(&self) -> Observation<'_> {
        Observation {
            frame: self.frame,
            state_hash: self.state_hash(),
            game_state: &self.global_state,
            entity_count: self.world.entity_count(),
            framebuffer: None,
        }
    }
}
