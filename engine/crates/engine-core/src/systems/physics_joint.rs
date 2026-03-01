/// SYSTEM: physics_joint
/// READS: PhysicsJoint, Transform, RigidBody
/// WRITES: Transform (position correction), RigidBody (velocity adjustment)
/// ORDER: runs after integrator, before collision

use crate::ecs::{World, Entity};

fn is_dynamic(world: &World, entity: Entity) -> bool {
    world.rigidbodies.get(entity)
        .map(|rb| !rb.is_static)
        .unwrap_or(false)
}

pub fn run(world: &mut World, dt: f64) {
    let joints: Vec<_> = world.physics_joints.iter()
        .map(|(e, j)| (e, j.clone()))
        .collect();

    let mut broken = Vec::new();

    for (owner, joint) in &joints {
        if joint.broken {
            broken.push(*owner);
            continue;
        }

        let a = joint.entity_a;
        let b = joint.entity_b;

        let (ax, ay) = match world.transforms.get(a) {
            Some(t) => (t.x, t.y),
            None => continue,
        };
        let (bx, by) = match world.transforms.get(b) {
            Some(t) => (t.x, t.y),
            None => continue,
        };

        let dx = bx - ax;
        let dy = by - ay;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < 1e-10 {
            continue;
        }

        let nx = dx / dist;
        let ny = dy / dist;

        match &joint.joint_type {
            crate::components::physics_joint::JointType::Distance { length, stiffness, damping } => {
                let error = dist - length;
                let correction = error * stiffness;

                if let Some(max_force) = joint.break_force {
                    if correction.abs() > max_force {
                        broken.push(*owner);
                        continue;
                    }
                }

                let a_dyn = is_dynamic(world, a);
                let b_dyn = is_dynamic(world, b);

                let (a_ratio, b_ratio) = match (a_dyn, b_dyn) {
                    (true, true) => (0.5, 0.5),
                    (true, false) => (1.0, 0.0),
                    (false, true) => (0.0, 1.0),
                    (false, false) => continue,
                };

                if let Some(ta) = world.transforms.get_mut(a) {
                    ta.x += nx * correction * a_ratio;
                    ta.y += ny * correction * a_ratio;
                }
                if let Some(tb) = world.transforms.get_mut(b) {
                    tb.x -= nx * correction * b_ratio;
                    tb.y -= ny * correction * b_ratio;
                }

                if *damping > 0.0 {
                    let a_vel = world.rigidbodies.get(a).map(|rb| (rb.vx, rb.vy)).unwrap_or((0.0, 0.0));
                    let b_vel = world.rigidbodies.get(b).map(|rb| (rb.vx, rb.vy)).unwrap_or((0.0, 0.0));
                    let rel_vx = b_vel.0 - a_vel.0;
                    let rel_vy = b_vel.1 - a_vel.1;
                    let rel_v_along = rel_vx * nx + rel_vy * ny;
                    let damp_force = rel_v_along * damping;

                    if a_dyn {
                        if let Some(rb) = world.rigidbodies.get_mut(a) {
                            rb.vx += nx * damp_force * a_ratio;
                            rb.vy += ny * damp_force * a_ratio;
                        }
                    }
                    if b_dyn {
                        if let Some(rb) = world.rigidbodies.get_mut(b) {
                            rb.vx -= nx * damp_force * b_ratio;
                            rb.vy -= ny * damp_force * b_ratio;
                        }
                    }
                }
            }
            crate::components::physics_joint::JointType::Spring { rest_length, k, damping } => {
                let displacement = dist - rest_length;
                let force = displacement * k;

                if let Some(max_force) = joint.break_force {
                    if force.abs() > max_force {
                        broken.push(*owner);
                        continue;
                    }
                }

                let fx = nx * force * dt;
                let fy = ny * force * dt;

                let a_dyn = is_dynamic(world, a);
                let b_dyn = is_dynamic(world, b);

                // Read velocities before modification for damping calculation
                let a_vel = world.rigidbodies.get(a).map(|rb| (rb.vx, rb.vy)).unwrap_or((0.0, 0.0));
                let b_vel = world.rigidbodies.get(b).map(|rb| (rb.vx, rb.vy)).unwrap_or((0.0, 0.0));

                if a_dyn {
                    if let Some(rb) = world.rigidbodies.get_mut(a) {
                        let inv_mass = if rb.mass > 0.0 { 1.0 / rb.mass } else { 0.0 };
                        rb.vx += fx * inv_mass;
                        rb.vy += fy * inv_mass;
                    }
                }
                if b_dyn {
                    if let Some(rb) = world.rigidbodies.get_mut(b) {
                        let inv_mass = if rb.mass > 0.0 { 1.0 / rb.mass } else { 0.0 };
                        rb.vx -= fx * inv_mass;
                        rb.vy -= fy * inv_mass;
                    }
                }

                if *damping > 0.0 {
                    let rel_vx = b_vel.0 - a_vel.0;
                    let rel_vy = b_vel.1 - a_vel.1;
                    let rel_v_along = rel_vx * nx + rel_vy * ny;
                    let damp = rel_v_along * damping * dt;

                    if a_dyn {
                        if let Some(rb) = world.rigidbodies.get_mut(a) {
                            rb.vx += nx * damp * 0.5;
                            rb.vy += ny * damp * 0.5;
                        }
                    }
                    if b_dyn {
                        if let Some(rb) = world.rigidbodies.get_mut(b) {
                            rb.vx -= nx * damp * 0.5;
                            rb.vy -= ny * damp * 0.5;
                        }
                    }
                }
            }
            crate::components::physics_joint::JointType::Rope { max_length } => {
                if dist <= *max_length {
                    continue;
                }

                let overshoot = dist - max_length;

                if let Some(max_force) = joint.break_force {
                    if overshoot > max_force {
                        broken.push(*owner);
                        continue;
                    }
                }

                let a_dyn = is_dynamic(world, a);
                let b_dyn = is_dynamic(world, b);

                let (a_ratio, b_ratio) = match (a_dyn, b_dyn) {
                    (true, true) => (0.5, 0.5),
                    (true, false) => (1.0, 0.0),
                    (false, true) => (0.0, 1.0),
                    (false, false) => continue,
                };

                if let Some(ta) = world.transforms.get_mut(a) {
                    ta.x += nx * overshoot * a_ratio;
                    ta.y += ny * overshoot * a_ratio;
                }
                if let Some(tb) = world.transforms.get_mut(b) {
                    tb.x -= nx * overshoot * b_ratio;
                    tb.y -= ny * overshoot * b_ratio;
                }

                if a_dyn {
                    if let Some(rb) = world.rigidbodies.get_mut(a) {
                        let v_along = rb.vx * nx + rb.vy * ny;
                        if v_along < 0.0 {
                            rb.vx -= nx * v_along;
                            rb.vy -= ny * v_along;
                        }
                    }
                }
                if b_dyn {
                    if let Some(rb) = world.rigidbodies.get_mut(b) {
                        let v_along = rb.vx * (-nx) + rb.vy * (-ny);
                        if v_along < 0.0 {
                            rb.vx -= (-nx) * v_along;
                            rb.vy -= (-ny) * v_along;
                        }
                    }
                }
            }
            crate::components::physics_joint::JointType::Hinge { radius, angular_velocity, angle, .. } => {
                let new_angle = angle + angular_velocity * dt;

                let Some(joint_mut) = world.physics_joints.get_mut(*owner) else { continue; };
                if let crate::components::physics_joint::JointType::Hinge {
                    angle: ref mut a_angle, min_angle, max_angle, ..
                } = joint_mut.joint_type {
                    *a_angle = new_angle;
                    if let Some(min) = min_angle {
                        if *a_angle < min { *a_angle = min; }
                    }
                    if let Some(max) = max_angle {
                        if *a_angle > max { *a_angle = max; }
                    }
                }

                let final_angle = if let Some(j) = world.physics_joints.get(*owner) {
                    match &j.joint_type {
                        crate::components::physics_joint::JointType::Hinge { angle, .. } => *angle,
                        _ => 0.0,
                    }
                } else {
                    0.0
                };

                if let Some(tb) = world.transforms.get_mut(b) {
                    tb.x = ax + radius * final_angle.cos();
                    tb.y = ay + radius * final_angle.sin();
                }
            }
        }
    }

    for entity in broken {
        if let Some(j) = world.physics_joints.get_mut(entity) {
            j.broken = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::World;
    use crate::components::transform::Transform;
    use crate::components::rigidbody::RigidBody;
    use crate::components::physics_joint::{PhysicsJoint, JointType};

    fn setup_two_dynamic_entities(world: &mut World) -> (Entity, Entity) {
        let a = world.spawn();
        let b = world.spawn();
        world.transforms.insert(a, Transform { x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.transforms.insert(b, Transform { x: 100.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        // Dynamic (is_static = false, which is the default)
        world.rigidbodies.insert(a, RigidBody::default());
        world.rigidbodies.insert(b, RigidBody::default());
        (a, b)
    }

    #[test]
    fn distance_joint_corrects_position() {
        let mut world = World::new();
        let (a, b) = setup_two_dynamic_entities(&mut world);
        world.physics_joints.insert(a, PhysicsJoint::distance(a, b, 50.0));

        run(&mut world, 1.0 / 60.0);
        let dist = {
            let ta = world.transforms.get(a).unwrap();
            let tb = world.transforms.get(b).unwrap();
            ((tb.x - ta.x).powi(2) + (tb.y - ta.y).powi(2)).sqrt()
        };
        assert!(dist < 100.0);
    }

    #[test]
    fn spring_joint_applies_force() {
        let mut world = World::new();
        let (a, b) = setup_two_dynamic_entities(&mut world);
        world.physics_joints.insert(a, PhysicsJoint::spring(a, b, 50.0, 100.0, 0.5));

        run(&mut world, 1.0 / 60.0);
        let rb_a = world.rigidbodies.get(a).unwrap();
        assert!(rb_a.vx > 0.0);
        let rb_b = world.rigidbodies.get(b).unwrap();
        assert!(rb_b.vx < 0.0);
    }

    #[test]
    fn rope_joint_slack_no_correction() {
        let mut world = World::new();
        let (a, b) = setup_two_dynamic_entities(&mut world);
        world.physics_joints.insert(a, PhysicsJoint::rope(a, b, 200.0));

        run(&mut world, 1.0 / 60.0);
        assert_eq!(world.transforms.get(a).unwrap().x, 0.0);
        assert_eq!(world.transforms.get(b).unwrap().x, 100.0);
    }

    #[test]
    fn rope_joint_taut_corrects() {
        let mut world = World::new();
        let (a, b) = setup_two_dynamic_entities(&mut world);
        world.physics_joints.insert(a, PhysicsJoint::rope(a, b, 50.0));

        run(&mut world, 1.0 / 60.0);
        let dist = {
            let ta = world.transforms.get(a).unwrap();
            let tb = world.transforms.get(b).unwrap();
            ((tb.x - ta.x).powi(2) + (tb.y - ta.y).powi(2)).sqrt()
        };
        assert!((dist - 50.0).abs() < 1.0);
    }

    #[test]
    fn hinge_joint_orbits() {
        let mut world = World::new();
        let (a, b) = setup_two_dynamic_entities(&mut world);
        let mut joint = PhysicsJoint::hinge(a, b, 50.0);
        if let JointType::Hinge { ref mut angular_velocity, .. } = joint.joint_type {
            *angular_velocity = std::f64::consts::PI;
        }
        world.physics_joints.insert(a, joint);

        run(&mut world, 0.5);
        let tb = world.transforms.get(b).unwrap();
        assert!((tb.x - 0.0).abs() < 1.0);
        assert!((tb.y - 50.0).abs() < 1.0);
    }

    #[test]
    fn break_force_breaks_joint() {
        let mut world = World::new();
        let (a, b) = setup_two_dynamic_entities(&mut world);
        let joint = PhysicsJoint::distance(a, b, 10.0).with_break_force(5.0);
        world.physics_joints.insert(a, joint);

        run(&mut world, 1.0 / 60.0);
        assert!(world.physics_joints.get(a).unwrap().broken);
    }

    #[test]
    fn static_body_doesnt_move() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();
        world.transforms.insert(a, Transform { x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.transforms.insert(b, Transform { x: 100.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        let mut rb_a = RigidBody::default();
        rb_a.is_static = true;
        world.rigidbodies.insert(a, rb_a);
        world.rigidbodies.insert(b, RigidBody::default());
        world.physics_joints.insert(a, PhysicsJoint::distance(a, b, 50.0));

        run(&mut world, 1.0 / 60.0);
        assert_eq!(world.transforms.get(a).unwrap().x, 0.0);
        assert!(world.transforms.get(b).unwrap().x < 100.0);
    }

    #[test]
    fn missing_transform_safe() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();
        world.physics_joints.insert(a, PhysicsJoint::distance(a, b, 50.0));
        run(&mut world, 1.0 / 60.0);
    }
}
