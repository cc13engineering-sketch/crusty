use super::scenario::ScheduledAction;
use super::shot_builder::ShotBuilder;

/// Generates input action sequences programmatically for automated testing.
///
/// Game-agnostic: produces ScheduledAction sequences that can be dispatched
/// by any game's action_dispatch function.

/// Generate a grid of shots varying angle and power.
///
/// Produces one (actions, total_frames) pair per grid cell. Useful for
/// systematic exploration of the input space.
pub fn grid_shots(
    origin_x: f64,
    origin_y: f64,
    angle_min: f64,
    angle_max: f64,
    angle_steps: usize,
    power_min: f64,
    power_max: f64,
    power_steps: usize,
) -> Vec<(String, Vec<ScheduledAction>, u64)> {
    let mut results = Vec::new();
    let angle_step = if angle_steps > 1 {
        (angle_max - angle_min) / (angle_steps - 1) as f64
    } else {
        0.0
    };
    let power_step = if power_steps > 1 {
        (power_max - power_min) / (power_steps - 1) as f64
    } else {
        0.0
    };

    for ai in 0..angle_steps {
        let angle = angle_min + ai as f64 * angle_step;
        for pi in 0..power_steps {
            let power = power_min + pi as f64 * power_step;
            let (actions, frames) = ShotBuilder::new()
                .aim_and_shoot(origin_x, origin_y, angle, power)
                .build();
            let label = format!("a{:.0}_p{:.0}", angle, (power * 100.0));
            results.push((label, actions, frames));
        }
    }
    results
}

/// Generate a sequence of random shots using a deterministic seed.
///
/// Uses a simple LCG PRNG for reproducibility without external dependencies.
pub fn random_shots(
    origin_x: f64,
    origin_y: f64,
    count: usize,
    seed: u64,
) -> Vec<(String, Vec<ScheduledAction>, u64)> {
    let mut rng = LcgRng::new(seed);
    let mut results = Vec::new();

    for i in 0..count {
        let angle = rng.next_f64() * 360.0;
        let power = 0.1 + rng.next_f64() * 0.9; // [0.1, 1.0]
        let (actions, frames) = ShotBuilder::new()
            .aim_and_shoot(origin_x, origin_y, angle, power)
            .build();
        let label = format!("rng_{}", i);
        results.push((label, actions, frames));
    }
    results
}

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
struct LcgRng {
    state: u64,
}

impl LcgRng {
    fn new(seed: u64) -> Self {
        Self { state: seed.wrapping_add(1) }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}
