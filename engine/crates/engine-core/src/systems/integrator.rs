/// SYSTEM: integrator
/// READS: RigidBody, Impulse, MotionConstraint
/// WRITES: RigidBody.vx, RigidBody.vy (velocity update only!)
/// NOTE: Does NOT update position — collision system does that after CCD.
/// ORDER: 2nd in physics step

use crate::ecs::World;

pub fn run(world: &mut World, dt: f64) {
    // --- Phase 1: Apply impulses (instant velocity deltas) ---
    let impulse_entities: Vec<_> = world.impulses.iter()
        .map(|(entity, imp)| (entity, imp.dvx, imp.dvy))
        .collect();
    for (entity, dvx, dvy) in &impulse_entities {
        if let Some(rb) = world.rigidbodies.get_mut(*entity) {
            if !rb.is_static {
                rb.vx += dvx;
                rb.vy += dvy;
            }
        }
    }
    for (entity, _, _) in impulse_entities {
        world.impulses.remove(entity);
    }

    // --- Phase 2: Velocity integration + damping ---
    for (_, rb) in world.rigidbodies.iter_mut() {
        if rb.is_static {
            continue;
        }
        rb.vx += rb.ax * dt;
        rb.vy += rb.ay * dt;
        // Framerate-independent damping
        let factor = (1.0 - rb.damping).powf(dt);
        rb.vx *= factor;
        rb.vy *= factor;
    }

    // --- Phase 3: Apply motion constraints ---
    let constrained: Vec<_> = world.motion_constraints.iter()
        .map(|(entity, mc)| (entity, mc.clone()))
        .collect();
    for (entity, mc) in constrained {
        if let Some(rb) = world.rigidbodies.get_mut(entity) {
            if rb.is_static {
                continue;
            }

            // Lock axes
            if mc.lock_x {
                rb.vx = 0.0;
            }
            if mc.lock_y {
                rb.vy = 0.0;
            }

            let speed = (rb.vx * rb.vx + rb.vy * rb.vy).sqrt();

            // Min speed: snap to zero if below threshold
            if let Some(min_speed) = mc.min_speed {
                if speed > 0.0 && speed < min_speed {
                    rb.vx = 0.0;
                    rb.vy = 0.0;
                    continue; // Already zeroed, skip max_speed check
                }
            }

            // Max speed: clamp magnitude
            if let Some(max_speed) = mc.max_speed {
                if speed > max_speed && speed > 0.0 {
                    let scale = max_speed / speed;
                    rb.vx *= scale;
                    rb.vy *= scale;
                }
            }
        }
    }
}
