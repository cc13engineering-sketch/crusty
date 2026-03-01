/// Integration tests for engine systems.
///
/// These tests exercise the full Engine through its public API and verify that
/// the internal systems (physics, collision, events, input, scripting) behave
/// correctly when composed together.

use crate::engine::Engine;
use crate::components::*;
use crate::events::EventKind;

// ─── Helpers ────────────────────────────────────────────────────────────────

const DT: f64 = 1.0 / 60.0;

fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() < eps
}

// ─── 1. Engine creation ────────────────────────────────────────────────────

#[test]
fn test_engine_creation() {
    let engine = Engine::new(800, 600);
    assert_eq!(engine.width, 800);
    assert_eq!(engine.height, 600);
    assert_eq!(engine.time, 0.0);
    assert_eq!(engine.frame, 0);
    assert_eq!(engine.world.entity_count(), 0);
    assert!(engine.events.events.is_empty());
}

// ─── 2. Basic tick ─────────────────────────────────────────────────────────

#[test]
fn test_basic_tick() {
    let mut engine = Engine::new(320, 240);
    engine.tick(DT);
    assert!(engine.time > 0.0, "time should advance after tick");
    assert_eq!(engine.frame, 1);
}

// ─── 3. Multiple ticks ────────────────────────────────────────────────────

#[test]
fn test_multiple_ticks() {
    let mut engine = Engine::new(320, 240);
    for _ in 0..10 {
        engine.tick(DT);
    }
    assert_eq!(engine.frame, 10);
    assert!(engine.time > 0.0);
}

// ─── 4. Physics: ball falls under directional force field (gravity) ─────────

#[test]
fn test_physics_ball_falls_with_gravity() {
    let mut engine = Engine::new(400, 400);

    // Create a gravity-like force field: directional, pointing downward,
    // with constant falloff and a large radius so it affects everything.
    let gravity_source = engine.world.spawn();
    engine.world.transforms.insert(gravity_source, Transform {
        x: 200.0, y: 200.0,
        rotation: 0.0, scale: 1.0,
    });
    engine.world.force_fields.insert(gravity_source, ForceField {
        field_type: FieldType::Directional { dx: 0.0, dy: 1.0 },
        strength: 200.0,
        radius: 10000.0,
        falloff: Falloff::Constant,
    });

    // Ball that should fall
    let ball = engine.world.spawn();
    engine.world.transforms.insert(ball, Transform {
        x: 100.0, y: 100.0,
        rotation: 0.0, scale: 1.0,
    });
    engine.world.rigidbodies.insert(ball, RigidBody {
        mass: 1.0,
        vx: 0.0, vy: 0.0,
        ax: 0.0, ay: 0.0,
        restitution: 0.5,
        friction: 0.3,
        is_static: false,
        damping: 0.0,
    });
    // No collider -- the collision system moves bodies without colliders
    // via the "position update for entities with RigidBody but NO Collider" path.

    let start_y = 100.0;

    // Run several physics steps. The force_accumulator sets ay from the
    // force field, the integrator updates velocity, and the collision system
    // moves the position for collider-less bodies.
    for _ in 0..60 {
        engine.physics_step(DT);
    }

    let t = engine.world.transforms.get(ball).expect("transform should exist");
    assert!(
        t.y > start_y,
        "ball should have fallen: start_y={}, current_y={}",
        start_y, t.y,
    );

    let rb = engine.world.rigidbodies.get(ball).expect("rigidbody should exist");
    assert!(
        rb.vy > 0.0,
        "ball should have positive vy from gravity: vy={}",
        rb.vy,
    );
}

// ─── 5. Collision: circle bounces off wall ─────────────────────────────────

#[test]
fn test_collision_circle_vs_wall() {
    let mut engine = Engine::new(800, 600);

    // Moving circle, starting at x=100, moving right at vx=200
    let circle = engine.world.spawn();
    engine.world.transforms.insert(circle, Transform {
        x: 100.0, y: 300.0,
        rotation: 0.0, scale: 1.0,
    });
    engine.world.rigidbodies.insert(circle, RigidBody {
        mass: 1.0,
        vx: 200.0, vy: 0.0,
        ax: 0.0, ay: 0.0,
        restitution: 1.0,
        friction: 0.0,
        is_static: false,
        damping: 0.0,
    });
    engine.world.colliders.insert(circle, Collider {
        shape: ColliderShape::Circle { radius: 10.0 },
        is_trigger: false,
    });

    // Static wall at x=200 (rect collider standing in the way)
    let wall = engine.world.spawn();
    engine.world.transforms.insert(wall, Transform {
        x: 200.0, y: 300.0,
        rotation: 0.0, scale: 1.0,
    });
    engine.world.rigidbodies.insert(wall, RigidBody {
        is_static: true,
        ..RigidBody::default()
    });
    engine.world.colliders.insert(wall, Collider {
        shape: ColliderShape::Rect { half_width: 10.0, half_height: 200.0 },
        is_trigger: false,
    });

    // Run enough physics steps for the circle to reach the wall and bounce.
    // At vx=200 px/s, crossing ~80px gap takes ~0.4s = ~24 steps at 60Hz.
    for _ in 0..60 {
        engine.physics_step(DT);
    }

    let rb = engine.world.rigidbodies.get(circle).expect("rigidbody should exist");
    assert!(
        rb.vx < 0.0,
        "circle should have bounced; vx should be negative but got vx={}",
        rb.vx,
    );
}

// ─── 6. Collision emits Collision events ───────────────────────────────────

#[test]
fn test_collision_emits_events() {
    let mut engine = Engine::new(800, 600);

    // Fast circle heading right
    let circle = engine.world.spawn();
    engine.world.transforms.insert(circle, Transform {
        x: 50.0, y: 300.0,
        rotation: 0.0, scale: 1.0,
    });
    engine.world.rigidbodies.insert(circle, RigidBody {
        mass: 1.0,
        vx: 500.0, vy: 0.0,
        ax: 0.0, ay: 0.0,
        restitution: 0.8,
        friction: 0.0,
        is_static: false,
        damping: 0.0,
    });
    engine.world.colliders.insert(circle, Collider {
        shape: ColliderShape::Circle { radius: 10.0 },
        is_trigger: false,
    });

    // Static wall right in front
    let wall = engine.world.spawn();
    engine.world.transforms.insert(wall, Transform {
        x: 100.0, y: 300.0,
        rotation: 0.0, scale: 1.0,
    });
    engine.world.rigidbodies.insert(wall, RigidBody {
        is_static: true,
        ..RigidBody::default()
    });
    engine.world.colliders.insert(wall, Collider {
        shape: ColliderShape::Rect { half_width: 10.0, half_height: 100.0 },
        is_trigger: false,
    });

    // Single physics step: at vx=500, the circle travels ~8.3px in one step,
    // and starts 30px from the wall edge (50+10 = 60, wall starts at 90).
    // Run a few steps to guarantee a hit.
    for _ in 0..10 {
        engine.physics_step(DT);
    }

    let has_collision = engine.events.events.iter().any(|e| {
        matches!(&e.kind, EventKind::Collision { .. })
    });
    assert!(has_collision, "expected a Collision event after circle hits wall");
}

// ─── 7. Trigger emits TriggerEnter events ──────────────────────────────────

#[test]
fn test_trigger_emits_trigger_event() {
    let mut engine = Engine::new(800, 600);

    // Moving circle
    let mover = engine.world.spawn();
    engine.world.transforms.insert(mover, Transform {
        x: 50.0, y: 300.0,
        rotation: 0.0, scale: 1.0,
    });
    engine.world.rigidbodies.insert(mover, RigidBody {
        mass: 1.0,
        vx: 500.0, vy: 0.0,
        ax: 0.0, ay: 0.0,
        restitution: 0.5,
        friction: 0.0,
        is_static: false,
        damping: 0.0,
    });
    engine.world.colliders.insert(mover, Collider {
        shape: ColliderShape::Circle { radius: 10.0 },
        is_trigger: false,
    });

    // Trigger zone (static circle with is_trigger=true)
    let trigger = engine.world.spawn();
    engine.world.transforms.insert(trigger, Transform {
        x: 100.0, y: 300.0,
        rotation: 0.0, scale: 1.0,
    });
    engine.world.rigidbodies.insert(trigger, RigidBody {
        is_static: true,
        ..RigidBody::default()
    });
    engine.world.colliders.insert(trigger, Collider {
        shape: ColliderShape::Circle { radius: 30.0 },
        is_trigger: true,
    });

    // Run physics steps until the circle enters the trigger zone
    for _ in 0..10 {
        engine.physics_step(DT);
    }

    let has_trigger = engine.events.events.iter().any(|e| {
        matches!(&e.kind, EventKind::TriggerEnter { .. })
    });
    assert!(has_trigger, "expected a TriggerEnter event when circle enters trigger zone");
}

// ─── 8. Static body does not move ──────────────────────────────────────────

#[test]
fn test_static_body_does_not_move() {
    let mut engine = Engine::new(400, 400);

    let entity = engine.world.spawn();
    engine.world.transforms.insert(entity, Transform {
        x: 200.0, y: 200.0,
        rotation: 0.0, scale: 1.0,
    });
    engine.world.rigidbodies.insert(entity, RigidBody {
        mass: 1.0,
        vx: 999.0, vy: 999.0,
        ax: 500.0, ay: 500.0,
        restitution: 0.5,
        friction: 0.0,
        is_static: true,
        damping: 0.0,
    });
    engine.world.colliders.insert(entity, Collider {
        shape: ColliderShape::Circle { radius: 10.0 },
        is_trigger: false,
    });

    for _ in 0..30 {
        engine.physics_step(DT);
    }

    let t = engine.world.transforms.get(entity).expect("transform should exist");
    assert!(
        approx_eq(t.x, 200.0, 0.001) && approx_eq(t.y, 200.0, 0.001),
        "static body should not move: x={}, y={}",
        t.x, t.y,
    );
}

// ─── 9. Damping reduces velocity ───────────────────────────────────────────

#[test]
fn test_damping_reduces_velocity() {
    let mut engine = Engine::new(400, 400);

    let entity = engine.world.spawn();
    engine.world.transforms.insert(entity, Transform {
        x: 200.0, y: 200.0,
        rotation: 0.0, scale: 1.0,
    });
    engine.world.rigidbodies.insert(entity, RigidBody {
        mass: 1.0,
        vx: 500.0, vy: 0.0,
        ax: 0.0, ay: 0.0,
        restitution: 0.5,
        friction: 0.0,
        is_static: false,
        damping: 0.5, // aggressive damping
    });
    // No collider -- velocity update still happens through integrator

    let initial_vx = 500.0;

    // Run integrator via physics_step
    engine.physics_step(DT);

    let rb = engine.world.rigidbodies.get(entity).expect("rigidbody should exist");
    assert!(
        rb.vx.abs() < initial_vx,
        "damping should reduce velocity: initial={}, current={}",
        initial_vx, rb.vx.abs(),
    );
    assert!(
        rb.vx > 0.0,
        "velocity should still be positive (same direction): vx={}",
        rb.vx,
    );
}

// ─── 10. World load and tick ───────────────────────────────────────────────

#[test]
fn test_world_load_and_tick() {
    let source = r#"
world "Test World" {
    bounds: 400 x 300
    background: #222233
}

entity ball {
    position: (100, 150)
    physics: { mass: 1.0, vx: 50.0, vy: 0.0, restitution: 0.9, damping: 0.01 }
    collider: { shape: circle, radius: 10 }
    visual: { shape: circle, radius: 10, color: #ff0000, filled: true }
    tags: ["ball"]
}

entity wall_right {
    position: (390, 150)
    physics: { is_static: true }
    collider: { shape: rect, half_width: 10, half_height: 150 }
    visual: { shape: rect, width: 20, height: 300, color: #333333, filled: true }
}
"#;

    let wf = crate::scripting::parser::parse_world(source)
        .expect("world should parse without error");

    let mut engine = Engine::new(400, 300);
    crate::scripting::loader::load_world_file(&wf, &mut engine.world, &mut engine.config);

    assert_eq!(engine.config.name, "Test World");
    assert!(
        engine.world.entity_count() >= 2,
        "expected at least 2 entities, got {}",
        engine.world.entity_count(),
    );

    // Tick several times -- should not panic
    for _ in 0..30 {
        engine.tick(DT);
    }

    // Verify entities still alive
    assert!(
        engine.world.entity_count() >= 2,
        "entities should persist after ticks",
    );
}

// ─── 11. Input affects player ──────────────────────────────────────────────

#[test]
fn test_input_affects_player() {
    let source = r#"
world "Input Test" {
    bounds: 400 x 300
}

entity player {
    position: (200, 150)
    physics: { mass: 1.0, restitution: 0.0, damping: 0.01 }
    collider: { shape: circle, radius: 10 }
    tags: ["player"]
}
"#;

    let wf = crate::scripting::parser::parse_world(source)
        .expect("world should parse without error");

    let mut engine = Engine::new(400, 300);
    crate::scripting::loader::load_world_file(&wf, &mut engine.world, &mut engine.config);

    let player = engine.world.names.get_by_name("player")
        .expect("player entity should exist");

    // Simulate pressing the right arrow key
    engine.input.on_key_down("ArrowRight".to_string());

    // Run a tick -- the input_gameplay system should set the player's velocity
    engine.tick(DT);

    let rb = engine.world.rigidbodies.get(player)
        .expect("player should have rigidbody");

    // The input_gameplay system sets vx = WALK_SPEED (200.0) when ArrowRight held
    assert!(
        rb.vx > 0.0,
        "player vx should be positive after pressing ArrowRight: vx={}",
        rb.vx,
    );
}

// ─── 12. Despawn removes from all stores ───────────────────────────────────

#[test]
fn test_despawn_removes_from_all_stores() {
    let mut engine = Engine::new(400, 400);

    let entity = engine.world.spawn_named("doomed");

    // Add every component type
    engine.world.transforms.insert(entity, Transform {
        x: 50.0, y: 50.0,
        rotation: 0.0, scale: 1.0,
    });
    engine.world.rigidbodies.insert(entity, RigidBody::default());
    engine.world.colliders.insert(entity, Collider::default());
    engine.world.renderables.insert(entity, Renderable::default());
    engine.world.force_fields.insert(entity, ForceField::default());
    engine.world.tags.insert(entity, Tags::new(&["test", "doomed"]));
    engine.world.roles.insert(entity, Role {
        name: "victim".to_string(),
        intent: "be removed".to_string(),
        group: Some("tests".to_string()),
    });

    // Verify all components exist before despawn
    assert!(engine.world.transforms.has(entity));
    assert!(engine.world.rigidbodies.has(entity));
    assert!(engine.world.colliders.has(entity));
    assert!(engine.world.renderables.has(entity));
    assert!(engine.world.force_fields.has(entity));
    assert!(engine.world.tags.has(entity));
    assert!(engine.world.roles.has(entity));
    assert!(engine.world.is_alive(entity));
    assert_eq!(engine.world.names.get_by_name("doomed"), Some(entity));

    // Despawn
    engine.world.despawn(entity);

    // Verify everything is gone
    assert!(!engine.world.transforms.has(entity));
    assert!(!engine.world.rigidbodies.has(entity));
    assert!(!engine.world.colliders.has(entity));
    assert!(!engine.world.renderables.has(entity));
    assert!(!engine.world.force_fields.has(entity));
    assert!(!engine.world.tags.has(entity));
    assert!(!engine.world.roles.has(entity));
    assert!(!engine.world.is_alive(entity));
    assert_eq!(engine.world.names.get_by_name("doomed"), None);
    assert_eq!(engine.world.names.get_name(entity), None);
    assert_eq!(engine.world.entity_count(), 0);
}
