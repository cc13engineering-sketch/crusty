/// SYSTEM: input_gameplay
/// READS: Input, Tags, Transform, Collider, RigidBody, Renderable
/// WRITES: RigidBody.vx, RigidBody.vy, RigidBody.is_static, Renderable.visual color
/// ORDER: runs once per frame after event_processor

use crate::ecs::World;
use crate::input::Input;
use crate::events::EventQueue;
use crate::components::{ColliderShape, Visual};
use crate::rendering::color::Color;

const MAX_DRAG_DISTANCE: f64 = 200.0;
const MAX_LAUNCH_SPEED: f64 = 1000.0;

pub fn run(world: &mut World, input: &Input, _events: &mut EventQueue) {
    // Slingshot launch on drag release
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
        return;
    }

    // Click-to-freeze: on click (non-drag), find ball under cursor and toggle freeze
    if input.mouse_buttons_released.contains(&0) && !input.is_dragging {
        let click_x = input.mouse_x;
        let click_y = input.mouse_y;

        // Find the entity under the click
        let mut hit_entity = None;
        let mut best_dist_sq = f64::MAX;

        let ball_entities: Vec<_> = world.tags.iter()
            .filter(|(_, tag)| tag.has("ball"))
            .map(|(e, _)| e)
            .collect();

        for entity in ball_entities {
            let t = match world.transforms.get(entity) {
                Some(t) => t,
                None => continue,
            };
            let col = match world.colliders.get(entity) {
                Some(c) => c,
                None => continue,
            };

            let dx = click_x - t.x;
            let dy = click_y - t.y;
            let dist_sq = dx * dx + dy * dy;

            let inside = match &col.shape {
                ColliderShape::Circle { radius } => dist_sq <= radius * radius,
                ColliderShape::Rect { half_width, half_height } => {
                    dx.abs() <= *half_width && dy.abs() <= *half_height
                }
            };

            if inside && dist_sq < best_dist_sq {
                best_dist_sq = dist_sq;
                hit_entity = Some(entity);
            }
        }

        if let Some(entity) = hit_entity {
            if let Some(rb) = world.rigidbodies.get_mut(entity) {
                let freezing = !rb.is_static;
                rb.is_static = freezing;
                if freezing {
                    rb.vx = 0.0;
                    rb.vy = 0.0;
                }

                // Update visual to indicate frozen/unfrozen state
                if let Some(rend) = world.renderables.get_mut(entity) {
                    match &mut rend.visual {
                        Visual::Circle { ref mut color, .. } => {
                            if freezing {
                                // Dim the color to indicate frozen
                                *color = Color::from_rgba(
                                    color.r / 3,
                                    color.g / 3,
                                    color.b / 3,
                                    color.a,
                                );
                            } else {
                                // Brighten back (multiply by 3, clamp to 255)
                                *color = Color::from_rgba(
                                    (color.r as u16 * 3).min(255) as u8,
                                    (color.g as u16 * 3).min(255) as u8,
                                    (color.b as u16 * 3).min(255) as u8,
                                    color.a,
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
