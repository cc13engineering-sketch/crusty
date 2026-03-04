/// SYSTEM: force_accumulator
/// READS: Transform, ForceField, RigidBody, Collider, ZoneEffect
/// WRITES: RigidBody.ax, RigidBody.ay (resets then accumulates)
/// ORDER: 1st in physics step

use crate::ecs::World;
use crate::physics::math::{self, Vec2};
use crate::components::{FieldType, Falloff, ColliderShape, ZoneEffectKind};

pub fn run(world: &mut World) {
    let World {
        transforms, rigidbodies, force_fields, colliders,
        zone_effects, ..
    } = world;

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

    // Collect zone effect data: (position, collider_shape, effects)
    let zones: Vec<(Vec2, ColliderShape, Vec<ZoneEffectKind>)> = zone_effects
        .iter()
        .filter_map(|(entity, ze)| {
            let t = transforms.get(entity)?;
            let col = colliders.get(entity)?;
            if !col.is_trigger {
                return None;
            }
            Some(((t.x, t.y), col.shape.clone(), ze.effects.clone()))
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

        // --- ForceField accumulation ---
        for (src_pos, ff, min_dist) in &sources {
            let diff = math::sub(pos, *src_pos);
            let raw_dist = math::length(diff);

            // Plummer falloff extends to infinity (no radius cutoff needed);
            // other falloffs cut off at the configured radius.
            let is_plummer = matches!(ff.falloff, Falloff::Plummer { .. });
            if !is_plummer && raw_dist > ff.radius {
                continue;
            }

            let magnitude = match ff.falloff {
                Falloff::Constant => ff.strength,
                Falloff::Linear => {
                    let dist = raw_dist.max(*min_dist);
                    ff.strength * (1.0 - dist / ff.radius)
                }
                Falloff::InverseSquare => {
                    let dist = raw_dist.max(*min_dist);
                    ff.strength / (dist * dist)
                }
                Falloff::Plummer { epsilon } => {
                    // F(r) = strength * r / (r^2 + epsilon^2)^(3/2)
                    // Smooth everywhere, zero at center, peaks at r = epsilon/sqrt(2)
                    let r2_eps2 = raw_dist * raw_dist + epsilon * epsilon;
                    ff.strength * raw_dist / (r2_eps2 * r2_eps2.sqrt())
                }
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

        // --- ZoneEffect accumulation ---
        for (zone_pos, zone_shape, effects) in &zones {
            if !point_in_shape(pos, *zone_pos, zone_shape) {
                continue;
            }

            for effect in effects {
                match effect {
                    ZoneEffectKind::Wind { dx, dy, strength } => {
                        // Wind applies as acceleration (force / mass, but
                        // strength is already in acceleration units)
                        let wind_dir = math::normalize((*dx, *dy));
                        rb.ax += wind_dir.0 * strength;
                        rb.ay += wind_dir.1 * strength;
                    }
                    ZoneEffectKind::Drag { coefficient } => {
                        // Drag opposes current velocity: a = -coefficient * v
                        rb.ax -= coefficient * rb.vx;
                        rb.ay -= coefficient * rb.vy;
                    }
                    ZoneEffectKind::SpeedMultiplier { factor } => {
                        // Apply as a per-frame velocity scale via acceleration.
                        // The target velocity is v * factor, so the needed
                        // acceleration nudge is v * (factor - 1).
                        rb.ax += rb.vx * (factor - 1.0);
                        rb.ay += rb.vy * (factor - 1.0);
                    }
                    ZoneEffectKind::Conveyor { dx, dy, speed } => {
                        // Conveyor pushes toward the conveyor direction at a
                        // fixed speed. Apply as acceleration toward target vel.
                        let dir = math::normalize((*dx, *dy));
                        let target_vx = dir.0 * speed;
                        let target_vy = dir.1 * speed;
                        // Acceleration proportional to difference from target
                        rb.ax += (target_vx - rb.vx) * 5.0;
                        rb.ay += (target_vy - rb.vy) * 5.0;
                    }
                }
            }
        }
    }
}

/// Simple point-in-shape test. Checks if `point` is inside the shape
/// centered at `center`.
fn point_in_shape(point: Vec2, center: Vec2, shape: &ColliderShape) -> bool {
    match shape {
        ColliderShape::Circle { radius } => {
            let dist_sq = math::distance_sq(point, center);
            dist_sq <= radius * radius
        }
        ColliderShape::Rect { half_width, half_height } => {
            let dx = (point.0 - center.0).abs();
            let dy = (point.1 - center.1).abs();
            dx <= *half_width && dy <= *half_height
        }
    }
}
