use super::scenario::ScheduledAction;

/// High-level builder for constructing minigolf shot input sequences.
///
/// Abstracts the slingshot mechanic so that test code can specify shots by
/// angle and power instead of manually computing pointer coordinates.
///
/// The slingshot mechanic: pointer_down at ball position, then drag AWAY from
/// the desired shot direction. The velocity goes opposite to the drag vector.
/// `on_pointer_up` computes: `velocity = (ball_pos - pointer_pos).normalized() * power`
///
/// ShotBuilder inverts this: given a desired shot angle and power fraction,
/// it computes the pointer_up position that produces that shot.
pub struct ShotBuilder {
    actions: Vec<ScheduledAction>,
    next_frame: u64,
}

impl ShotBuilder {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            next_frame: 5, // small delay after setup
        }
    }

    /// Start building from a specific frame.
    pub fn at_frame(frame: u64) -> Self {
        Self {
            actions: Vec::new(),
            next_frame: frame,
        }
    }

    /// Queue a shot aimed at `angle_deg` (0=right, 90=down, 180=left, 270=up)
    /// with `power` in 0.0..1.0.
    ///
    /// Internally computes the slingshot drag vector: pointer_down at ball,
    /// pointer_up at `ball - direction * (power * 120px)`.
    /// The `120px` matches the `dist / 120.0` normalization in trap_links_demo.
    pub fn aim_and_shoot(mut self, ball_x: f64, ball_y: f64, angle_deg: f64, power: f64) -> Self {
        let angle_rad = angle_deg.to_radians();
        let drag_dist = power.clamp(0.0, 1.0) * 120.0;

        // Slingshot: we drag OPPOSITE to desired direction
        // So pointer_up = ball - desired_direction * drag_dist
        // But the game's atan2(ball - pointer) gives the direction,
        // so we put the pointer in the opposite direction of where we want to go.
        let up_x = ball_x - angle_rad.cos() * drag_dist;
        let up_y = ball_y - angle_rad.sin() * drag_dist;

        self.actions.push(ScheduledAction::PointerDown {
            frame: self.next_frame,
            x: ball_x,
            y: ball_y,
        });
        self.actions.push(ScheduledAction::PointerUp {
            frame: self.next_frame,
            x: up_x,
            y: up_y,
        });

        // Wait ~3 seconds for ball to settle
        self.next_frame += 180;
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
