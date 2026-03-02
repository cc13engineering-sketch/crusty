use super::scenario::ScheduledAction;

/// High-level builder for constructing slingshot-style input sequences.
///
/// Abstracts a drag-and-release mechanic: pointer_down at origin, then release
/// at an offset computed from angle and power. The drag distance normalization
/// is configurable via `drag_scale` (default 120.0).
///
/// Works for any game that uses a slingshot/pull-back aiming mechanic.
pub struct ShotBuilder {
    actions: Vec<ScheduledAction>,
    next_frame: u64,
    /// Pixels of drag that correspond to power=1.0.
    drag_scale: f64,
    /// Frames to wait after each shot for the simulation to settle.
    settle_frames: u64,
}

impl Default for ShotBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ShotBuilder {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            next_frame: 5, // small delay after setup
            drag_scale: 120.0,
            settle_frames: 180,
        }
    }

    /// Start building from a specific frame.
    pub fn at_frame(frame: u64) -> Self {
        Self {
            actions: Vec::new(),
            next_frame: frame,
            drag_scale: 120.0,
            settle_frames: 180,
        }
    }

    /// Set the drag distance that corresponds to power=1.0.
    /// Default is 120.0 pixels.
    pub fn with_drag_scale(mut self, scale: f64) -> Self {
        self.drag_scale = scale;
        self
    }

    /// Set how many frames to wait after each shot for the simulation to settle.
    /// Default is 180 (3 seconds at 60fps).
    pub fn with_settle_frames(mut self, frames: u64) -> Self {
        self.settle_frames = frames;
        self
    }

    /// Queue a shot aimed at `angle_deg` (0=right, 90=down, 180=left, 270=up)
    /// with `power` in 0.0..1.0.
    ///
    /// Computes the slingshot drag vector: pointer_down at origin,
    /// pointer_up at `origin - direction * (power * drag_scale)`.
    pub fn aim_and_shoot(mut self, origin_x: f64, origin_y: f64, angle_deg: f64, power: f64) -> Self {
        let angle_rad = angle_deg.to_radians();
        let drag_dist = power.clamp(0.0, 1.0) * self.drag_scale;

        // Slingshot: we drag OPPOSITE to desired direction
        let up_x = origin_x - angle_rad.cos() * drag_dist;
        let up_y = origin_y - angle_rad.sin() * drag_dist;

        self.actions.push(ScheduledAction::PointerDown {
            frame: self.next_frame,
            x: origin_x,
            y: origin_y,
        });
        self.actions.push(ScheduledAction::PointerUp {
            frame: self.next_frame,
            x: up_x,
            y: up_y,
        });

        self.next_frame += self.settle_frames;
        self
    }

    /// Wait additional frames before the next action.
    pub fn wait(mut self, frames: u64) -> Self {
        self.next_frame += frames;
        self
    }

    /// Build the action list and suggested total frame count.
    pub fn build(self) -> (Vec<ScheduledAction>, u64) {
        let total = self.next_frame + 60; // buffer
        (self.actions, total)
    }
}
