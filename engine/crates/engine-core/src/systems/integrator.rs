/// SYSTEM: integrator
/// READS: RigidBody, Impulse, MotionConstraint, ContinuousDrag
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
    // Collect ContinuousDrag data before iterating rigidbodies
    let drag_data: Vec<_> = world.continuous_drags.iter()
        .map(|(entity, cd)| (entity, cd.base_drag, cd.speed_drag, cd.rest_threshold))
        .collect();

    for (entity, rb) in world.rigidbodies.iter_mut() {
        if rb.is_static {
            continue;
        }
        rb.vx += rb.ax * dt;
        rb.vy += rb.ay * dt;

        // Check for ContinuousDrag component on this entity
        let has_continuous_drag = drag_data.iter().any(|(e, _, _, _)| *e == entity);
        if has_continuous_drag {
            // ContinuousDrag replaces standard damping for this entity
            if let Some((_, base_drag, speed_drag, rest_threshold)) =
                drag_data.iter().find(|(e, _, _, _)| *e == entity)
            {
                let speed = (rb.vx * rb.vx + rb.vy * rb.vy).sqrt();
                let effective_drag = base_drag + speed_drag * speed;
                let factor = (-effective_drag * dt).exp();
                rb.vx *= factor;
                rb.vy *= factor;
                if speed * factor < *rest_threshold {
                    rb.vx = 0.0;
                    rb.vy = 0.0;
                }
            }
        } else {
            // Standard framerate-independent damping (original behavior)
            let factor = (1.0 - rb.damping).powf(dt);
            rb.vx *= factor;
            rb.vy *= factor;
        }
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

/// System: edge_bounce
/// READS: Transform, RigidBody, EdgeBounce
/// WRITES: Transform.x, Transform.y, RigidBody.vx, RigidBody.vy
///
/// Reflects entities off world boundaries. Run after collision system
/// (or as part of post-physics) to keep entities within bounds.
pub fn run_edge_bounce(world: &mut World, bounds: (f64, f64)) {
    let bounce_entities: Vec<_> = world.edge_bounces.iter()
        .map(|(entity, eb)| (entity, eb.restitution, eb.margin))
        .collect();

    for (entity, restitution, margin) in bounce_entities {
        let (bw, bh) = bounds;
        let (mut bounce_x, mut bounce_y) = (false, false);
        let (mut new_x, mut new_y);

        if let Some(t) = world.transforms.get(entity) {
            new_x = t.x;
            new_y = t.y;

            if new_x - margin < 0.0 { new_x = margin; bounce_x = true; }
            if new_x + margin > bw { new_x = bw - margin; bounce_x = true; }
            if new_y - margin < 0.0 { new_y = margin; bounce_y = true; }
            if new_y + margin > bh { new_y = bh - margin; bounce_y = true; }
        } else {
            continue;
        }

        if bounce_x || bounce_y {
            if let Some(rb) = world.rigidbodies.get_mut(entity) {
                if bounce_x { rb.vx = -rb.vx * restitution; }
                if bounce_y { rb.vy = -rb.vy * restitution; }
            }
            if let Some(t) = world.transforms.get_mut(entity) {
                t.x = new_x;
                t.y = new_y;
            }
        }
    }
}
