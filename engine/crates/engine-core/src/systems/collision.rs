/// SYSTEM: collision
/// READS: Transform, Collider, RigidBody, Tags
/// WRITES: Transform.x, Transform.y, RigidBody.vx, RigidBody.vy
/// PUSHES: Collision and TriggerEnter events
/// ORDER: 3rd in physics step (after integrator)

use crate::ecs::{World, Entity};
use crate::events::{EventQueue, EventKind};
use crate::physics::{math, ccd, spatial_grid};
use crate::components::ColliderShape;

const GRID_CELL_SIZE: f64 = 128.0;

#[derive(Clone)]
struct EntitySnap {
    entity: Entity,
    pos: (f64, f64),
    vel: (f64, f64),
    collider: ColliderShape,
    radius_for_sweep: f64, // circle radius, or 0 for rect
    restitution: f64,
    is_static: bool,
    is_trigger: bool,
    has_rigidbody: bool,
}

struct MoveResult {
    entity: Entity,
    new_pos: (f64, f64),
    new_vel: (f64, f64),
}

pub fn run(world: &mut World, events: &mut EventQueue, dt: f64) {
    let World { transforms, colliders, rigidbodies, physics_materials, tags: _, .. } = world;

    // SNAPSHOT all collidable entities
    let mut snaps: Vec<EntitySnap> = Vec::new();
    let entities: Vec<Entity> = colliders.entities().collect();
    for entity in &entities {
        let t = match transforms.get(*entity) {
            Some(t) => t,
            None => continue,
        };
        let c = match colliders.get(*entity) {
            Some(c) => c,
            None => continue,
        };
        let rb = rigidbodies.get(*entity);
        let pm = physics_materials.get(*entity);
        let radius = match &c.shape {
            ColliderShape::Circle { radius } => *radius,
            ColliderShape::Rect { .. } => 0.0,
        };
        let restitution = pm
            .and_then(|m| m.restitution_override)
            .unwrap_or_else(|| rb.map_or(0.5, |r| r.restitution));
        snaps.push(EntitySnap {
            entity: *entity,
            pos: (t.x, t.y),
            vel: rb.map_or((0.0, 0.0), |r| (r.vx, r.vy)),
            collider: c.shape.clone(),
            radius_for_sweep: radius,
            restitution,
            is_static: rb.map_or(true, |r| r.is_static),
            is_trigger: c.is_trigger,
            has_rigidbody: rb.is_some(),
        });
    }

    // BUILD spatial grid from all entity snapshots
    let mut grid = spatial_grid::SpatialGrid::new(GRID_CELL_SIZE);
    for (idx, snap) in snaps.iter().enumerate() {
        let (hx, hy) = spatial_grid::collider_aabb_half(&snap.collider);
        grid.insert(
            idx,
            snap.pos.0 - hx, snap.pos.1 - hy,
            snap.pos.0 + hx, snap.pos.1 + hy,
        );
    }

    let mut results: Vec<MoveResult> = Vec::new();
    let mut new_events: Vec<EventKind> = Vec::new();
    let mut candidates: Vec<usize> = Vec::new();

    // PROCESS each moving circle entity
    for i in 0..snaps.len() {
        let snap = &snaps[i];
        if snap.is_static || !snap.has_rigidbody {
            continue;
        }
        // Only Circle colliders support CCD as moving body
        if snap.radius_for_sweep <= 0.0 {
            crate::log::warn("Non-circle mover skipped for CCD");
            continue;
        }

        let r = snap.radius_for_sweep;
        let mut current_pos = snap.pos;
        let mut current_vel = snap.vel;
        let mut remaining_t = 1.0_f64;
        let max_bounces = 4;
        let mut excluded: Vec<Entity> = Vec::new();

        for _ in 0..max_bounces {
            if remaining_t < 1e-6 {
                break;
            }
            let target_pos = math::add(
                current_pos,
                math::scale(current_vel, dt * remaining_t),
            );

            // Compute swept AABB covering the entire trajectory + radius
            let sweep_min_x = current_pos.0.min(target_pos.0) - r;
            let sweep_min_y = current_pos.1.min(target_pos.1) - r;
            let sweep_max_x = current_pos.0.max(target_pos.0) + r;
            let sweep_max_y = current_pos.1.max(target_pos.1) + r;

            // Query spatial grid for candidates in the swept region
            grid.query(sweep_min_x, sweep_min_y, sweep_max_x, sweep_max_y, &mut candidates);

            // Find earliest hit among candidates
            let mut best_hit: Option<(ccd::SweepHit, usize)> = None;
            for &j in &candidates {
                if i == j { continue; }
                if excluded.contains(&snaps[j].entity) { continue; }
                let other = &snaps[j];
                let hit = match &other.collider {
                    ColliderShape::Circle { radius } => {
                        ccd::sweep_circle_vs_circle(current_pos, target_pos, r, other.pos, *radius)
                    }
                    ColliderShape::Rect { half_width, half_height } => {
                        ccd::sweep_circle_vs_aabb(
                            current_pos, target_pos, r,
                            other.pos, *half_width, *half_height,
                        )
                    }
                };
                if let Some(h) = hit {
                    if h.t >= 0.0 && best_hit.as_ref().map_or(true, |(bh, _)| h.t < bh.t) {
                        best_hit = Some((h, j));
                    }
                }
            }

            match best_hit {
                None => {
                    current_pos = target_pos;
                    break;
                }
                Some((hit, j)) => {
                    let other = &snaps[j];

                    if snap.is_trigger || other.is_trigger {
                        // Push trigger event
                        let (trigger, visitor) = if other.is_trigger {
                            (other.entity, snap.entity)
                        } else {
                            (snap.entity, other.entity)
                        };
                        new_events.push(EventKind::TriggerEnter { trigger, visitor });
                        // Exclude this trigger and continue sweep
                        excluded.push(other.entity);
                        continue;
                    }

                    // Physical collision
                    let contact_pos = math::add(
                        hit.contact,
                        math::scale(hit.normal, 0.01), // epsilon separation
                    );

                    let e = snap.restitution.max(other.restitution);
                    let reflected = math::reflect(current_vel, hit.normal);
                    let new_vel = math::scale(reflected, e);

                    new_events.push(EventKind::Collision {
                        entity_a: snap.entity,
                        entity_b: other.entity,
                        normal: hit.normal,
                        contact: hit.contact,
                    });

                    current_pos = contact_pos;
                    current_vel = new_vel;
                    remaining_t *= 1.0 - hit.t;
                }
            }
        }

        results.push(MoveResult {
            entity: snap.entity,
            new_pos: current_pos,
            new_vel: current_vel,
        });
    }

    // Position update for entities with RigidBody but NO Collider
    let rb_entities: Vec<Entity> = rigidbodies.entities().collect();
    for entity in &rb_entities {
        if colliders.has(*entity) { continue; }
        let rb = match rigidbodies.get(*entity) {
            Some(r) if !r.is_static => r,
            _ => continue,
        };
        if let Some(t) = transforms.get(*entity) {
            results.push(MoveResult {
                entity: *entity,
                new_pos: (t.x + rb.vx * dt, t.y + rb.vy * dt),
                new_vel: (rb.vx, rb.vy),
            });
        }
    }

    // COMMIT results
    for res in results {
        if let Some(t) = transforms.get_mut(res.entity) {
            t.x = res.new_pos.0;
            t.y = res.new_pos.1;
        }
        if let Some(rb) = rigidbodies.get_mut(res.entity) {
            rb.vx = res.new_vel.0;
            rb.vy = res.new_vel.1;
        }
    }

    for event in new_events {
        events.push(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Transform, RigidBody, Collider, ColliderShape, PhysicsMaterial};
    use crate::events::EventQueue;

    /// Helper: create a world with a moving ball at (-100, 0) heading right
    /// toward a static wall (rect) at (0, 0). Ball radius 10, wall 50x200.
    /// Returns (world, events) ready to call `run()`.
    fn ball_and_wall(
        ball_restitution: f64,
        ball_material: Option<PhysicsMaterial>,
        wall_material: Option<PhysicsMaterial>,
    ) -> (World, EventQueue) {
        let mut world = World::new();
        let events = EventQueue::default();

        // Ball entity — moving right
        let ball = world.spawn();
        world.transforms.insert(ball, Transform { x: -100.0, y: 0.0, ..Transform::default() });
        world.colliders.insert(ball, Collider {
            shape: ColliderShape::Circle { radius: 10.0 },
            is_trigger: false,
        });
        world.rigidbodies.insert(ball, RigidBody {
            vx: 200.0, vy: 0.0,
            restitution: ball_restitution,
            is_static: false,
            ..RigidBody::default()
        });
        if let Some(pm) = ball_material {
            world.physics_materials.insert(ball, pm);
        }

        // Wall entity — static rect
        let wall = world.spawn();
        world.transforms.insert(wall, Transform { x: 0.0, y: 0.0, ..Transform::default() });
        world.colliders.insert(wall, Collider {
            shape: ColliderShape::Rect { half_width: 50.0, half_height: 200.0 },
            is_trigger: false,
        });
        world.rigidbodies.insert(wall, RigidBody {
            restitution: 0.5,
            is_static: true,
            ..RigidBody::default()
        });
        if let Some(pm) = wall_material {
            world.physics_materials.insert(wall, pm);
        }

        (world, events)
    }

    // ─── Test 1: Default restitution uses RigidBody value ───────────

    #[test]
    fn default_restitution_uses_rigidbody_value() {
        // Ball has restitution 0.8 on RigidBody, no PhysicsMaterial
        let (mut world, mut events) = ball_and_wall(0.8, None, None);
        run(&mut world, &mut events, 1.0);

        // Ball should have bounced; wall rb has 0.5, ball rb has 0.8
        // max(0.8, 0.5) = 0.8 → velocity scaled by 0.8
        let ball = Entity(1);
        let rb = world.rigidbodies.get(ball).unwrap();
        // Ball was going right (+200), hit wall, reflected to left
        assert!(rb.vx < 0.0, "ball should bounce left, got vx={}", rb.vx);
        // Speed should be approximately 200 * 0.8 = 160
        let speed = rb.vx.abs();
        assert!((speed - 160.0).abs() < 1.0,
            "speed should be ~160 (200*0.8), got {}", speed);
    }

    // ─── Test 2: PhysicsMaterial override takes precedence ──────────

    #[test]
    fn physics_material_override_takes_precedence() {
        // Ball rb.restitution = 0.3, but PhysicsMaterial overrides to 0.9
        let ball_pm = PhysicsMaterial {
            restitution_override: Some(0.9),
            ..PhysicsMaterial::default()
        };
        let (mut world, mut events) = ball_and_wall(0.3, Some(ball_pm), None);
        run(&mut world, &mut events, 1.0);

        let ball = Entity(1);
        let rb = world.rigidbodies.get(ball).unwrap();
        assert!(rb.vx < 0.0, "ball should bounce left");
        // max(0.9, 0.5) = 0.9 → speed ~200*0.9 = 180
        let speed = rb.vx.abs();
        assert!((speed - 180.0).abs() < 1.0,
            "speed should be ~180 (200*0.9), got {}", speed);
    }

    // ─── Test 3: max() allows amplification (e > 1.0) ──────────────

    #[test]
    fn max_combination_allows_amplification() {
        // Wall has restitution_override = 1.5 (bumper)
        let wall_pm = PhysicsMaterial {
            restitution_override: Some(1.5),
            ..PhysicsMaterial::default()
        };
        let (mut world, mut events) = ball_and_wall(0.5, None, Some(wall_pm));
        run(&mut world, &mut events, 1.0);

        let ball = Entity(1);
        let rb = world.rigidbodies.get(ball).unwrap();
        assert!(rb.vx < 0.0, "ball should bounce left");
        // max(0.5, 1.5) = 1.5 → speed ~200*1.5 = 300
        let speed = rb.vx.abs();
        assert!((speed - 300.0).abs() < 1.0,
            "speed should be ~300 (200*1.5), got {}", speed);
    }

    // ─── Test 4: Two entities with overrides use max of both ────────

    #[test]
    fn two_overrides_use_max() {
        // Ball override = 0.6, wall override = 0.9
        let ball_pm = PhysicsMaterial {
            restitution_override: Some(0.6),
            ..PhysicsMaterial::default()
        };
        let wall_pm = PhysicsMaterial {
            restitution_override: Some(0.9),
            ..PhysicsMaterial::default()
        };
        let (mut world, mut events) = ball_and_wall(0.3, Some(ball_pm), Some(wall_pm));
        run(&mut world, &mut events, 1.0);

        let ball = Entity(1);
        let rb = world.rigidbodies.get(ball).unwrap();
        assert!(rb.vx < 0.0, "ball should bounce left");
        // max(0.6, 0.9) = 0.9 → speed ~200*0.9 = 180
        let speed = rb.vx.abs();
        assert!((speed - 180.0).abs() < 1.0,
            "speed should be ~180 (200*max(0.6,0.9)), got {}", speed);
    }

    // ─── Test 5: Restitution scales reflected velocity correctly ────

    #[test]
    fn restitution_scales_reflected_velocity() {
        // Test with e=1.0 (perfect elastic) — speed should be preserved
        let ball_pm = PhysicsMaterial {
            restitution_override: Some(1.0),
            ..PhysicsMaterial::default()
        };
        let (mut world, mut events) = ball_and_wall(0.5, Some(ball_pm), None);
        run(&mut world, &mut events, 1.0);

        let ball = Entity(1);
        let rb = world.rigidbodies.get(ball).unwrap();
        assert!(rb.vx < 0.0, "ball should bounce left");
        // max(1.0, 0.5) = 1.0 → speed = 200 * 1.0 = 200
        let speed = rb.vx.abs();
        assert!((speed - 200.0).abs() < 1.0,
            "perfect elastic bounce should preserve speed ~200, got {}", speed);

        // Now test with e=0.0 (perfectly inelastic) on both
        let ball_pm2 = PhysicsMaterial {
            restitution_override: Some(0.0),
            ..PhysicsMaterial::default()
        };
        let wall_pm2 = PhysicsMaterial {
            restitution_override: Some(0.0),
            ..PhysicsMaterial::default()
        };
        let (mut world2, mut events2) = ball_and_wall(0.5, Some(ball_pm2), Some(wall_pm2));
        run(&mut world2, &mut events2, 1.0);

        let rb2 = world2.rigidbodies.get(ball).unwrap();
        // max(0.0, 0.0) = 0.0 → speed = 200 * 0.0 = 0
        let speed2 = (rb2.vx * rb2.vx + rb2.vy * rb2.vy).sqrt();
        assert!(speed2 < 1.0,
            "perfectly inelastic bounce should stop ball, got speed {}", speed2);
    }
}
