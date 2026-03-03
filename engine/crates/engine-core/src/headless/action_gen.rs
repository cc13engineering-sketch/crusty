use super::scenario::ScheduledAction;

/// Generates input action sequences programmatically for automated testing.
///
/// Game-agnostic: produces ScheduledAction sequences that can be dispatched
/// by any game's action_dispatch function.

/// Generate a tap sequence at specific coordinates and frames.
///
/// Each entry is a (frame, x, y) producing PointerDown+PointerUp on that frame.
/// Useful for testing UI buttons, menu interactions, etc.
pub fn tap_sequence(taps: &[(u64, f64, f64)]) -> (Vec<ScheduledAction>, u64) {
    let mut actions = Vec::new();
    let mut max_frame = 0u64;

    for &(frame, x, y) in taps {
        actions.push(ScheduledAction::PointerDown { frame, x, y });
        actions.push(ScheduledAction::PointerUp { frame: frame + 1, x, y });
        max_frame = max_frame.max(frame + 1);
    }

    (actions, max_frame + 30) // 30 frames settle time
}

/// Generate a drag gesture from one point to another.
pub fn drag(
    start_frame: u64,
    from_x: f64,
    from_y: f64,
    to_x: f64,
    to_y: f64,
    drag_frames: u64,
) -> (Vec<ScheduledAction>, u64) {
    let mut actions = Vec::new();
    actions.push(ScheduledAction::PointerDown {
        frame: start_frame,
        x: from_x,
        y: from_y,
    });

    // Interpolate move events
    let steps = drag_frames.max(1);
    for i in 1..steps {
        let t = i as f64 / steps as f64;
        let x = from_x + (to_x - from_x) * t;
        let y = from_y + (to_y - from_y) * t;
        actions.push(ScheduledAction::PointerMove {
            frame: start_frame + i,
            x,
            y,
        });
    }

    actions.push(ScheduledAction::PointerUp {
        frame: start_frame + steps,
        x: to_x,
        y: to_y,
    });

    (actions, start_frame + steps + 30)
}

// ─── Deterministic PRNG ─────────────────────────────────────────────

/// Simple LCG for deterministic random generation without external deps.
pub(crate) struct LcgRng {
    state: u64,
}

impl LcgRng {
    pub(crate) fn new(seed: u64) -> Self {
        Self { state: seed.wrapping_add(1) }
    }

    pub(crate) fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }

    pub(crate) fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}
