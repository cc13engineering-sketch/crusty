/// SYSTEM: force_accumulator
/// READS: Transform, ForceField, RigidBody, Collider
/// WRITES: RigidBody.ax, RigidBody.ay (resets then accumulates)
/// ORDER: 1st in physics step

use crate::ecs::World;
use crate::physics::math::{self, Vec2};
use crate::components::{FieldType, Falloff, ColliderShape};

pub fn run(world: &mut World) {
    let World { transforms, rigidbodies, force_fields, colliders, .. } = world;

    // Collect force field data: (position, field_ref, min_distance)
    let sources: Vec<(Vec2, crate::components::ForceField, f64)> = force_fields
        .iter()
        .filter_map(|(entity, ff)| {
            let t = transforms.get(entity)?;
            let min_dist = colliders.get(entity).map_or(1.0, |c| match &c.shape {
                ColliderShape::Circle { radius } => *radius,
                ColliderShape::Rect { half_width, half_height } => half_width.min(*half_height),
            });
            Some(((t.x, t.y), ff.clone(), min_dist))
        })
        .collect();

    // For each dynamic body, accumulate forces
    let body_entities: Vec<crate::ecs::Entity> = rigidbodies.sorted_entities();
    for entity in body_entities {
        let rb = match rigidbodies.get_mut(entity) {
            Some(rb) if !rb.is_static => rb,
            _ => continue,
        };
        let pos = match transforms.get(entity) {
            Some(t) => (t.x, t.y),
            None => continue,
        };

        rb.ax = 0.0;
        rb.ay = 0.0;

        for (src_pos, ff, min_dist) in &sources {
            let diff = math::sub(pos, *src_pos);
            let mut dist = math::length(diff);
            if dist > ff.radius {
                continue;
            }
            dist = dist.max(*min_dist);

            let magnitude = match ff.falloff {
                Falloff::Constant => ff.strength,
                Falloff::Linear => ff.strength * (1.0 - dist / ff.radius),
                Falloff::InverseSquare => ff.strength / (dist * dist),
            };

            let dir: Vec2 = match &ff.field_type {
                FieldType::Attract => math::normalize(math::sub(*src_pos, pos)),
                FieldType::Repel => math::normalize(diff),
                FieldType::Directional { dx, dy } => math::normalize((*dx, *dy)),
                FieldType::Vortex => {
                    let radial = math::normalize(diff);
                    math::perpendicular(radial) // clockwise
                }
            };

            let accel = magnitude / rb.mass;
            rb.ax += dir.0 * accel;
            rb.ay += dir.1 * accel;
        }
    }
}
