/// SYSTEM: behavior
/// READS: Behavior, Transform (all entities for target resolution), Tags
/// WRITES: RigidBody.vx, RigidBody.vy
/// ORDER: runs before force_accumulator in physics step

use crate::ecs::{World, Entity};
use crate::components::BehaviorMode;
use crate::physics::math;

pub fn run(world: &mut World, dt: f64) {
    // Collect target positions by tag
    let player_positions: Vec<(Entity, (f64, f64))> = world.tags.iter()
        .filter(|(_, t)| t.has(crate::components::Tag::Player))
        .filter_map(|(e, _)| world.transforms.get(e).map(|t| (e, (t.x, t.y))))
        .collect();

    let entities: Vec<Entity> = world.behaviors.sorted_entities();
    for entity in entities {
        let behavior = match world.behaviors.get(entity) {
            Some(b) => b.clone(),
            None => continue,
        };
        let pos = match world.transforms.get(entity) {
            Some(t) => (t.x, t.y),
            None => continue,
        };

        let desired_vel = match &behavior.mode {
            BehaviorMode::Drift => continue,
            BehaviorMode::Chase => {
                if let Some(tag) = &behavior.target_tag {
                    if let Some(target_pos) = find_nearest_with_tag(tag, pos, &player_positions, world) {
                        let dir = math::normalize(math::sub(target_pos, pos));
                        math::scale(dir, behavior.speed)
                    } else { continue }
                } else { continue }
            }
            BehaviorMode::Flee => {
                if let Some(tag) = &behavior.target_tag {
                    if let Some(target_pos) = find_nearest_with_tag(tag, pos, &player_positions, world) {
                        let dir = math::normalize(math::sub(pos, target_pos));
                        math::scale(dir, behavior.speed)
                    } else { continue }
                } else { continue }
            }
            BehaviorMode::Seek { target_x, target_y } => {
                let dir = math::normalize(math::sub((*target_x, *target_y), pos));
                math::scale(dir, behavior.speed)
            }
            BehaviorMode::Orbit { radius, angle } => {
                let new_angle = angle + behavior.speed / radius.max(1.0) * dt;
                // Update the angle
                if let Some(b) = world.behaviors.get_mut(entity) {
                    if let BehaviorMode::Orbit { angle: ref mut a, .. } = b.mode {
                        *a = new_angle;
                    }
                }
                let target_x = pos.0 + new_angle.cos() * radius;
                let target_y = pos.1 + new_angle.sin() * radius;
                let dir = math::normalize(math::sub((target_x, target_y), pos));
                math::scale(dir, behavior.speed)
            }
        };

        if let Some(rb) = world.rigidbodies.get_mut(entity) {
            // Smooth steering
            let steer_factor = (behavior.turn_rate * dt).min(1.0);
            rb.vx += (desired_vel.0 - rb.vx) * steer_factor;
            rb.vy += (desired_vel.1 - rb.vy) * steer_factor;
        }
    }
}

fn find_nearest_with_tag(
    tag: &crate::components::Tag,
    from: (f64, f64),
    cached_players: &[(Entity, (f64, f64))],
    world: &World,
) -> Option<(f64, f64)> {
    // Find the nearest cached player position
    if *tag == crate::components::Tag::Player {
        return cached_players.iter()
            .min_by(|(_, a_pos), (_, b_pos)| {
                math::distance_sq(from, *a_pos)
                    .partial_cmp(&math::distance_sq(from, *b_pos))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(_, pos)| *pos);
    }
    // General lookup for other tags
    let mut best_pos = None;
    let mut best_dist = f64::MAX;
    for (e, t) in world.tags.iter() {
        if t.has(*tag) {
            if let Some(transform) = world.transforms.get(e) {
                let dist = math::distance_sq(from, (transform.x, transform.y));
                if dist < best_dist {
                    best_dist = dist;
                    best_pos = Some((transform.x, transform.y));
                }
            }
        }
    }
    best_pos
}
