/// SYSTEM: input_gameplay
/// READS: Input, Tags
/// WRITES: RigidBody.vx, RigidBody.vy (on the player entity)
/// ORDER: runs once per frame after event_processor

use crate::ecs::World;
use crate::input::Input;
use crate::events::EventQueue;

const MAX_DRAG_DISTANCE: f64 = 200.0;
const MAX_LAUNCH_SPEED: f64 = 1000.0;

pub fn run(world: &mut World, input: &Input, _events: &mut EventQueue) {
    if input.mouse_buttons_released.contains(&0) && input.is_dragging {
        if let Some((start_x, start_y)) = input.drag_start {
            let end_x = input.mouse_x;
            let end_y = input.mouse_y;

            // Slingshot direction: opposite to drag
            let dx = start_x - end_x;
            let dy = start_y - end_y;
            let drag_dist = (dx * dx + dy * dy).sqrt();

            if drag_dist > 1.0 {
                let power_frac = (drag_dist / MAX_DRAG_DISTANCE).min(1.0);
                let speed = MAX_LAUNCH_SPEED * power_frac * power_frac;
                let dir = (dx / drag_dist, dy / drag_dist);

                let World { tags, rigidbodies, .. } = world;
                for (entity, tag) in tags.iter() {
                    if tag.has("player") {
                        if let Some(rb) = rigidbodies.get_mut(entity) {
                            rb.vx = dir.0 * speed;
                            rb.vy = dir.1 * speed;
                        }
                    }
                }
            }
        }
    }
}
