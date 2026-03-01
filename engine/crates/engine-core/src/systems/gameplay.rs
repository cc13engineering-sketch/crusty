/// SYSTEM: gameplay
/// The gameplay rules engine. Processes collision events and applies game logic:
/// - bullet + enemy/asteroid → damage, score, despawn
/// - enemy/asteroid + player → damage to player, screen shake
/// - pickup + player → heal, despawn pickup
/// READS: EventQueue, Tags, GameState
/// WRITES: GameState, SpawnQueue (despawns), PostFxConfig (screen shake), ParticlePool (explosions)

use crate::ecs::{World, Entity};
use crate::events::{EventQueue, EventKind};
use crate::spawn_queue::SpawnQueue;
use crate::rendering::post_fx::PostFxConfig;
use crate::rendering::particles::ParticlePool;
use crate::rendering::color::Color;

pub fn run(
    world: &mut World,
    events: &EventQueue,
    spawn_queue: &mut SpawnQueue,
    post_fx: &mut PostFxConfig,
    particles: &mut ParticlePool,
    frame: u64,
) {
    for event in events.iter() {
        match &event.kind {
            EventKind::Collision { entity_a, entity_b, contact, .. } => {
                handle_collision(world, *entity_a, *entity_b, *contact, spawn_queue, post_fx, particles, frame);
            }
            EventKind::TriggerEnter { trigger, visitor } => {
                handle_trigger(world, *trigger, *visitor, spawn_queue, particles, frame);
            }
            EventKind::Interaction { .. } => {}
        }
    }
}

fn has_tag(world: &World, entity: Entity, tag: &str) -> bool {
    world.tags.get(entity).map_or(false, |t| t.has(tag))
}

fn handle_collision(
    world: &mut World,
    a: Entity, b: Entity,
    contact: (f64, f64),
    spawn_queue: &mut SpawnQueue,
    post_fx: &mut PostFxConfig,
    particles: &mut ParticlePool,
    frame: u64,
) {
    let a_bullet = has_tag(world, a, "bullet");
    let b_bullet = has_tag(world, b, "bullet");
    let a_enemy = has_tag(world, a, "enemy") || has_tag(world, a, "asteroid");
    let b_enemy = has_tag(world, b, "enemy") || has_tag(world, b, "asteroid");
    let a_player = has_tag(world, a, "player");
    let b_player = has_tag(world, b, "player");

    // Bullet hits enemy/asteroid
    if a_bullet && b_enemy {
        bullet_hits_target(world, a, b, contact, spawn_queue, particles, frame);
    } else if b_bullet && a_enemy {
        bullet_hits_target(world, b, a, contact, spawn_queue, particles, frame);
    }

    // Enemy/asteroid hits player
    if a_enemy && b_player {
        enemy_hits_player(world, a, b, contact, spawn_queue, post_fx, particles, frame);
    } else if b_enemy && a_player {
        enemy_hits_player(world, b, a, contact, spawn_queue, post_fx, particles, frame);
    }
}

fn bullet_hits_target(
    world: &mut World,
    bullet: Entity, target: Entity,
    contact: (f64, f64),
    spawn_queue: &mut SpawnQueue,
    particles: &mut ParticlePool,
    frame: u64,
) {
    // Get bullet damage (default 1)
    let damage = world.game_states.get(bullet)
        .map_or(1.0, |gs| gs.get("damage").max(1.0));

    // Get points from target
    let points = world.game_states.get(target)
        .map_or(100.0, |gs| gs.get("points"));

    // Apply damage to target
    let target_dead = if let Some(gs) = world.game_states.get_mut(target) {
        gs.subtract_check_zero("health", damage)
    } else {
        true // no health = one-shot
    };

    // Always despawn bullet
    spawn_queue.despawn(bullet);

    if target_dead {
        spawn_queue.despawn(target);

        // Add score to player
        for (e, tag) in world.tags.iter() {
            if tag.has("player") {
                if let Some(gs) = world.game_states.get_mut(e) {
                    gs.add("score", points);
                    gs.add("kills", 1.0);
                }
            }
        }

        // Spawn explosion particles
        let is_asteroid = has_tag(world, target, "asteroid");
        let color = if is_asteroid {
            Color::from_rgba(200, 160, 100, 255)
        } else {
            Color::from_rgba(255, 100, 50, 255)
        };
        particles.spawn_burst(
            contact.0, contact.1,
            25, 50.0, 200.0,
            0.6, 3.0, 0.5,
            color,
            Color::from_rgba(color.r / 2, color.g / 2, color.b / 2, 0),
            frame,
        );
    } else {
        // Hit spark
        particles.spawn_burst(
            contact.0, contact.1,
            8, 30.0, 100.0,
            0.3, 2.0, 0.5,
            Color::from_rgba(255, 255, 200, 255),
            Color::from_rgba(255, 100, 0, 0),
            frame.wrapping_add(1),
        );
    }
}

fn enemy_hits_player(
    world: &mut World,
    enemy: Entity, player: Entity,
    contact: (f64, f64),
    spawn_queue: &mut SpawnQueue,
    post_fx: &mut PostFxConfig,
    particles: &mut ParticlePool,
    frame: u64,
) {
    let damage = world.game_states.get(enemy)
        .map_or(10.0, |gs| gs.get("damage").max(1.0));

    // Apply damage to player's shield first, then health
    if let Some(gs) = world.game_states.get_mut(player) {
        let shield = gs.get("shield");
        if shield > 0.0 {
            let absorbed = damage.min(shield);
            gs.add("shield", -absorbed);
            let remaining = damage - absorbed;
            if remaining > 0.0 {
                gs.add("health", -remaining);
            }
        } else {
            gs.add("health", -damage);
        }
    }

    // Despawn the enemy on contact
    spawn_queue.despawn(enemy);

    // Screen shake
    post_fx.shake_remaining = 0.15;
    post_fx.shake_intensity = 6.0;

    // Impact particles
    particles.spawn_burst(
        contact.0, contact.1,
        20, 40.0, 150.0,
        0.5, 2.5, 0.5,
        Color::from_rgba(255, 200, 50, 255),
        Color::from_rgba(255, 50, 0, 0),
        frame.wrapping_add(2),
    );
}

fn handle_trigger(
    world: &mut World,
    trigger: Entity, visitor: Entity,
    spawn_queue: &mut SpawnQueue,
    particles: &mut ParticlePool,
    frame: u64,
) {
    let is_pickup = has_tag(world, trigger, "pickup");
    let is_player = has_tag(world, visitor, "player");

    if is_pickup && is_player {
        // Heal the player
        if let Some(gs) = world.game_states.get_mut(visitor) {
            gs.add("health", 25.0);
            // Clamp health to max
            let health = gs.get("health");
            if health > 100.0 { gs.set("health", 100.0); }
        }

        spawn_queue.despawn(trigger);

        // Pickup sparkle
        if let Some(t) = world.transforms.get(trigger) {
            particles.spawn_burst(
                t.x, t.y,
                15, 20.0, 80.0,
                0.5, 2.0, 0.5,
                Color::from_rgba(50, 255, 100, 255),
                Color::from_rgba(0, 200, 50, 0),
                frame.wrapping_add(3),
            );
        }
    }
}
