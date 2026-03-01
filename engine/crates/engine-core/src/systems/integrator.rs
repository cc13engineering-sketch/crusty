/// SYSTEM: integrator
/// READS: RigidBody
/// WRITES: RigidBody.vx, RigidBody.vy (velocity update only!)
/// NOTE: Does NOT update position — collision system does that after CCD.
/// ORDER: 2nd in physics step

use crate::ecs::World;

pub fn run(world: &mut World, dt: f64) {
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
}
