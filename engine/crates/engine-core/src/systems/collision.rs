/// SYSTEM: collision
/// READS: Transform, Collider, RigidBody, Tags
/// WRITES: Transform.x, Transform.y, RigidBody.vx, RigidBody.vy
/// PUSHES: Collision and TriggerEnter events
/// ORDER: 3rd in physics step (after integrator)

use crate::ecs::{World, Entity};
use crate::events::{EventQueue, EventKind};
use crate::physics::{math, ccd};
use crate::components::ColliderShape;

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
    let World { transforms, colliders, rigidbodies, tags, .. } = world;

    // SNAPSHOT all collidable entities
    let mut snaps: Vec<EntitySnap> = Vec::new();
    let entities = colliders.sorted_entities();
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
        let radius = match &c.shape {
            ColliderShape::Circle { radius } => *radius,
            ColliderShape::Rect { .. } => 0.0,
        };
        snaps.push(EntitySnap {
            entity: *entity,
            pos: (t.x, t.y),
            vel: rb.map_or((0.0, 0.0), |r| (r.vx, r.vy)),
            collider: c.shape.clone(),
            radius_for_sweep: radius,
            restitution: rb.map_or(0.5, |r| r.restitution),
            is_static: rb.map_or(true, |r| r.is_static),
            is_trigger: c.is_trigger,
            has_rigidbody: rb.is_some(),
        });
    }

    let mut results: Vec<MoveResult> = Vec::new();
    let mut new_events: Vec<EventKind> = Vec::new();

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

            // Find earliest hit
            let mut best_hit: Option<(ccd::SweepHit, usize)> = None;
            for j in 0..snaps.len() {
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

                    let e = snap.restitution.min(other.restitution);
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
    let rb_entities: Vec<Entity> = rigidbodies.sorted_entities();
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
