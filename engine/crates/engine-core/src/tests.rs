/// Integration tests for engine systems.
///
/// These tests exercise the full Engine through its public API and verify that
/// the internal systems (physics, collision, events, input, scripting) behave
/// correctly when composed together.

use crate::engine::Engine;
use crate::components::*;
use crate::components::state_machine::{TransitionCondition, CompareOp};
use crate::components::property_tween::{Tween, TweenTarget, EasingFn};
use crate::components::waypoint_path::{Waypoint, WaypointMode};
use crate::events::EventKind;
use crate::rendering::color::Color;
use crate::tilemap::{TileMap, Tile, TileType};

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

// (tests continue below — end-to-end tests are appended after test 12)

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

// ═══════════════════════════════════════════════════════════════════════════
// END-TO-END TESTS — Innovation Games Round 5
//
// Each test exercises one or more complete engine systems in coordination.
// Tests use the public Engine API and verify observable state changes.
// ═══════════════════════════════════════════════════════════════════════════

// ─── E2E-1. Full tick cycle ─────────────────────────────────────────────────
//
// Spawn entities via the world, run multiple full engine ticks, and verify
// that time advances, entities survive, and the entity count is stable.

#[test]
fn e2e_full_tick_cycle_entities_survive_60_frames() {
    let mut engine = Engine::new(320, 240);

    // Spawn a handful of entities with basic components
    for i in 0..5 {
        let e = engine.world.spawn();
        engine.world.transforms.insert(e, Transform {
            x: i as f64 * 50.0, y: 100.0,
            rotation: 0.0, scale: 1.0,
        });
        engine.world.rigidbodies.insert(e, RigidBody {
            vx: 10.0 * i as f64, vy: 0.0,
            mass: 1.0,
            damping: 0.01,
            ..RigidBody::default()
        });
    }

    let initial_count = engine.world.entity_count();
    assert_eq!(initial_count, 5);

    // Tick 60 frames (one second at 60 Hz)
    for _ in 0..60 {
        engine.tick(DT);
    }

    assert_eq!(engine.frame, 60, "frame counter should reach 60");
    assert!(
        approx_eq(engine.time, DT * 60.0, 1e-6),
        "engine time should be ~1 second, got {}",
        engine.time,
    );
    assert_eq!(
        engine.world.entity_count(), 5,
        "entities without lifetimes should not be removed",
    );

    // Verify that physics actually moved entities with non-zero velocity
    let fast_entity = engine.world.transforms.iter()
        .map(|(_, t)| t.x)
        .fold(f64::NEG_INFINITY, f64::max);
    assert!(
        fast_entity > 0.0,
        "at least one entity should have moved right, max_x={}",
        fast_entity,
    );
}

// ─── E2E-2. Hierarchy propagation through tick() ────────────────────────────
//
// Parent entity moves; after tick(), the child's WorldTransform should
// reflect the parent's new position plus the child's local offset.

#[test]
fn e2e_hierarchy_parent_move_propagates_to_child_world_transform() {
    let mut engine = Engine::new(320, 240);

    let parent = engine.world.spawn();
    engine.world.transforms.insert(parent, Transform {
        x: 100.0, y: 100.0,
        rotation: 0.0, scale: 1.0,
    });

    let child = engine.world.spawn();
    // Child is 20 units to the right of the parent in local space
    engine.world.transforms.insert(child, Transform {
        x: 20.0, y: 0.0,
        rotation: 0.0, scale: 1.0,
    });
    engine.world.parents.insert(child, Parent::new(parent));

    // Run one tick so the hierarchy system fires
    engine.tick(DT);

    let child_wt = engine.world.world_transforms.get(child)
        .expect("child should have WorldTransform after tick");

    // World position of child = parent(100,100) + local(20,0) = (120,100)
    assert!(
        approx_eq(child_wt.x, 120.0, 0.1),
        "child world_x should be 120 after hierarchy propagation, got {}",
        child_wt.x,
    );
    assert!(
        approx_eq(child_wt.y, 100.0, 0.1),
        "child world_y should be 100, got {}",
        child_wt.y,
    );

    // Now move the parent and tick again
    if let Some(t) = engine.world.transforms.get_mut(parent) {
        t.x = 200.0;
        t.y = 50.0;
    }
    engine.tick(DT);

    let child_wt2 = engine.world.world_transforms.get(child)
        .expect("child should still have WorldTransform");

    // New world position = parent(200,50) + local(20,0) = (220,50)
    assert!(
        approx_eq(child_wt2.x, 220.0, 0.1),
        "child world_x should update to 220 after parent moved, got {}",
        child_wt2.x,
    );
    assert!(
        approx_eq(child_wt2.y, 50.0, 0.1),
        "child world_y should update to 50 after parent moved, got {}",
        child_wt2.y,
    );
}

// ─── E2E-3. State machine transitions over multiple ticks ───────────────────
//
// An entity starts in "idle", transitions to "walk" after 0.5s, then
// to "run" after another 0.3s.  Verify timing through engine tick().

#[test]
fn e2e_state_machine_advances_states_over_ticks() {
    let mut engine = Engine::new(320, 240);

    let e = engine.world.spawn();
    let mut sm = StateMachine::new("idle");
    sm.add_transition("idle", "walk", TransitionCondition::After(0.5));
    sm.add_transition("walk", "run",  TransitionCondition::After(0.3));
    engine.world.state_machines.insert(e, sm);

    // Tick for 0.4s — still in idle
    let ticks_04s = (0.4 / DT).ceil() as u32;
    for _ in 0..ticks_04s {
        engine.tick(DT);
    }
    assert!(
        engine.world.state_machines.get(e).unwrap().is_in("idle"),
        "should be in idle at 0.4s",
    );

    // Tick until we cross 0.5s
    let ticks_02s = (0.2 / DT).ceil() as u32;
    for _ in 0..ticks_02s {
        engine.tick(DT);
    }
    assert!(
        engine.world.state_machines.get(e).unwrap().is_in("walk"),
        "should transition to walk after 0.5s total",
    );

    // Tick another 0.35s to cross the 0.3s walk → run threshold
    let ticks_035s = (0.35 / DT).ceil() as u32;
    for _ in 0..ticks_035s {
        engine.tick(DT);
    }
    assert!(
        engine.world.state_machines.get(e).unwrap().is_in("run"),
        "should transition to run after walk threshold",
    );
}

// ─── E2E-4. Coroutine execution across multiple ticks ───────────────────────
//
// A coroutine waits 0.5s, sets a game state value, waits another 0.5s,
// then sets a second value.  Verify the sequencing via engine tick().

#[test]
fn e2e_coroutine_multi_step_executes_in_sequence() {
    let mut engine = Engine::new(320, 240);

    let e = engine.world.spawn();
    engine.world.game_states.insert(e, GameState::new());
    engine.world.coroutines.insert(e, Coroutine::new("sequence")
        .then_wait(0.5)
        .then_set_state("phase", 1.0)
        .then_wait(0.5)
        .then_set_state("phase", 2.0));

    // Before 0.5s: phase should not be set
    for _ in 0..(0.45 / DT) as u32 {
        engine.tick(DT);
    }
    let phase_early = engine.world.game_states.get(e)
        .map(|gs| gs.get("phase"))
        .unwrap_or(0.0);
    assert!(
        approx_eq(phase_early, 0.0, 1e-6),
        "phase should still be 0 before first wait completes, got {}",
        phase_early,
    );

    // Tick past 0.5s mark — phase 1 should be set
    for _ in 0..(0.2 / DT) as u32 {
        engine.tick(DT);
    }
    let phase_mid = engine.world.game_states.get(e)
        .map(|gs| gs.get("phase"))
        .unwrap_or(0.0);
    assert!(
        approx_eq(phase_mid, 1.0, 1e-6),
        "phase should be 1.0 after first wait, got {}",
        phase_mid,
    );

    // Tick past second 0.5s — phase 2 should be set, coroutine removed
    for _ in 0..(0.6 / DT) as u32 {
        engine.tick(DT);
    }
    let phase_final = engine.world.game_states.get(e)
        .map(|gs| gs.get("phase"))
        .unwrap_or(0.0);
    assert!(
        approx_eq(phase_final, 2.0, 1e-6),
        "phase should be 2.0 after second wait, got {}",
        phase_final,
    );
    assert!(
        engine.world.coroutines.get(e).is_none(),
        "completed coroutine should be removed",
    );
}

// ─── E2E-5. Signal → StateMachine integration ───────────────────────────────
//
// An entity's FSM listens for "alarm" signal. A separate emitter entity
// activates the signal, which should cause the FSM to transition.

#[test]
fn e2e_signal_triggers_state_machine_transition() {
    let mut engine = Engine::new(320, 240);

    // Guard entity with FSM listening for a signal
    let guard = engine.world.spawn();
    let mut sm = StateMachine::new("patrol");
    sm.add_transition("patrol", "alert", TransitionCondition::OnSignal("alarm".to_string()));
    engine.world.state_machines.insert(guard, sm);

    // Alarm emitter (starts inactive)
    let alarm = engine.world.spawn();
    engine.world.signal_emitters.insert(alarm, SignalEmitter::new("alarm", false));

    // Several ticks with inactive alarm — guard stays in patrol
    for _ in 0..10 {
        engine.tick(DT);
    }
    assert!(
        engine.world.state_machines.get(guard).unwrap().is_in("patrol"),
        "guard should remain in patrol while alarm is inactive",
    );

    // Activate the alarm
    engine.world.signal_emitters.get_mut(alarm).unwrap().active = true;

    // One tick — signal system fires, FSM system sees active signal
    engine.tick(DT);

    assert!(
        engine.world.state_machines.get(guard).unwrap().is_in("alert"),
        "guard should transition to alert when alarm activates",
    );
}

// ─── E2E-6. Tween completion: X moves from A to B over N frames ─────────────
//
// Create a PropertyTween that moves an entity's X from 0 to 100 in 1 second.
// After exactly that duration, the entity should be at X=100 and the tween
// component should be removed.

#[test]
fn e2e_tween_moves_entity_x_from_a_to_b_and_completes() {
    let mut engine = Engine::new(320, 240);

    let e = engine.world.spawn();
    engine.world.transforms.insert(e, Transform {
        x: 0.0, y: 50.0, rotation: 0.0, scale: 1.0,
    });
    engine.world.property_tweens.insert(e, PropertyTween::new()
        .with_tween(Tween::new(TweenTarget::X, 0.0, 100.0, 1.0, EasingFn::Linear)));

    // Tick for half the duration — X should be near 50
    let half_ticks = (0.5 / DT) as u32;
    for _ in 0..half_ticks {
        engine.tick(DT);
    }
    let x_mid = engine.world.transforms.get(e).map(|t| t.x).unwrap_or(0.0);
    assert!(
        x_mid > 30.0 && x_mid < 70.0,
        "X should be near 50 at midpoint, got {}",
        x_mid,
    );

    // Tick until past the 1.0s duration
    let remaining_ticks = (0.6 / DT) as u32;
    for _ in 0..remaining_ticks {
        engine.tick(DT);
    }
    let x_final = engine.world.transforms.get(e).map(|t| t.x).unwrap_or(0.0);
    assert!(
        x_final >= 99.0,
        "X should be at or near 100 after tween completes, got {}",
        x_final,
    );
    assert!(
        !engine.world.property_tweens.has(e),
        "PropertyTween should be removed after all tweens complete",
    );
}

// ─── E2E-7. Waypoint following with TimeScale ───────────────────────────────
//
// Entity has a WaypointPath at half speed (TimeScale 0.5). A second entity
// with normal speed follows the same path. After equal time, the slow
// entity should be closer to the start than the normal entity.

#[test]
fn e2e_waypoint_with_time_scale_half_speed_moves_less() {
    let mut engine = Engine::new(320, 240);

    // Normal-speed entity: no TimeScale component (defaults to 1.0)
    let normal_speed = engine.world.spawn();
    engine.world.transforms.insert(normal_speed, Transform {
        x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0,
    });
    engine.world.waypoint_paths.insert(normal_speed, WaypointPath::new(
        vec![Waypoint::new(500.0, 0.0)],
        200.0,
        WaypointMode::Once,
    ));

    // Half-speed entity
    let half_speed = engine.world.spawn();
    engine.world.transforms.insert(half_speed, Transform {
        x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0,
    });
    engine.world.waypoint_paths.insert(half_speed, WaypointPath::new(
        vec![Waypoint::new(500.0, 0.0)],
        200.0,
        WaypointMode::Once,
    ));
    engine.world.time_scales.insert(half_speed, TimeScale::new(0.5));

    // Run 0.5s of ticks
    for _ in 0..(0.5 / DT) as u32 {
        engine.tick(DT);
    }

    let x_normal = engine.world.transforms.get(normal_speed).map(|t| t.x).unwrap_or(0.0);
    let x_half   = engine.world.transforms.get(half_speed).map(|t| t.x).unwrap_or(0.0);

    assert!(
        x_normal > 0.0,
        "normal-speed entity should have moved forward, x={}",
        x_normal,
    );
    assert!(
        x_half > 0.0,
        "half-speed entity should have moved forward too, x={}",
        x_half,
    );
    assert!(
        x_normal > x_half,
        "normal-speed entity ({}) should be ahead of half-speed entity ({})",
        x_normal, x_half,
    );
    // The ratio should be approximately 2:1
    let ratio = x_normal / x_half.max(1.0);
    assert!(
        ratio > 1.5 && ratio < 2.5,
        "velocity ratio should be near 2:1, got {}",
        ratio,
    );
}

// ─── E2E-8. EntityFlash with Active flag ────────────────────────────────────
//
// An entity has a blink flash. When Active is disabled, the flash system
// does not advance (the flash timer should not tick). When re-enabled, it
// resumes. Verify by checking that flash is still alive after a long wait
// while disabled.

#[test]
fn e2e_entity_flash_respects_active_flag_via_time_scale() {
    let mut engine = Engine::new(320, 240);

    let e = engine.world.spawn();
    engine.world.renderables.insert(e, Renderable {
        visual: Visual::Circle { radius: 10.0, color: Color::RED, filled: true },
        layer: 0,
        visible: true,
    });
    // Short blink: total duration 0.3s, so it should expire after 0.3s
    engine.world.entity_flashes.insert(e, EntityFlash::blink(0.05, 0.05, 0.3));

    // Freeze the entity's time — the flash system respects TimeScale
    engine.world.time_scales.insert(e, TimeScale::frozen());

    // Tick for 1 second with frozen time — flash should NOT expire
    for _ in 0..(1.0 / DT) as u32 {
        engine.tick(DT);
    }
    assert!(
        engine.world.entity_flashes.has(e),
        "flash should still be alive while entity time is frozen",
    );

    // Unfreeze
    engine.world.time_scales.get_mut(e).unwrap().scale = 1.0;

    // Tick 0.4s — now the flash should expire (0.3s duration)
    for _ in 0..(0.4 / DT) as u32 {
        engine.tick(DT);
    }
    assert!(
        !engine.world.entity_flashes.has(e),
        "flash should have expired after unfreezing and ticking past its duration",
    );
    // Visibility should be restored after blink expires
    assert!(
        engine.world.renderables.get(e).map_or(false, |r| r.visible),
        "entity should be visible after blink expires",
    );
}

// ─── E2E-9. Tilemap queries ──────────────────────────────────────────────────
//
// Build a 10x10 tilemap, paint a solid tile at (3, 5), then verify that
// world_to_tile and is_solid_at_world work correctly through the engine.

#[test]
fn e2e_tilemap_world_to_tile_and_is_solid_at_world() {
    let mut engine = Engine::new(640, 480);

    // Create a 10x10 tilemap with 32px tiles at origin (0, 0)
    let mut tilemap = TileMap::new(10, 10, 32.0);

    // Set tile (3, 5) to solid
    tilemap.set(3, 5, Tile::solid(Color::WHITE));

    // Set tile (1, 2) to platform (not solid by is_solid definition)
    tilemap.set(1, 2, Tile::platform(Color::GREEN));

    engine.tilemap = Some(tilemap);

    let tm = engine.tilemap.as_ref().unwrap();

    // --- world_to_tile ---
    // Tile (3,5): world center = (3*32 + 16, 5*32 + 16) = (112, 176)
    let coords = tm.world_to_tile(112.0, 176.0);
    assert_eq!(
        coords, Some((3, 5)),
        "world point (112, 176) should map to tile (3, 5), got {:?}",
        coords,
    );

    // Corner of tile (0,0)
    assert_eq!(tm.world_to_tile(0.0, 0.0), Some((0, 0)));
    // Outside map
    assert_eq!(tm.world_to_tile(-1.0, 0.0), None);
    assert_eq!(tm.world_to_tile(0.0, 10.0 * 32.0), None); // exactly at boundary

    // --- is_solid_at_world ---
    assert!(
        tm.is_solid_at_world(112.0, 176.0),
        "tile (3,5) world center should be solid",
    );
    assert!(
        !tm.is_solid_at_world(32.0 + 16.0, 64.0 + 16.0), // tile (1,2) = platform, not solid
        "tile (1,2) is a platform, is_solid_at_world should be false",
    );
    assert!(
        !tm.is_solid_at_world(0.0, 0.0), // tile (0,0) = empty
        "tile (0,0) should be empty",
    );
}

// ─── E2E-10. Raycasting hits entity via engine's World ──────────────────────
//
// Spawn a circle collider entity in the world and verify that a ray fired
// directly at it returns a hit with the correct entity ID and distance.

#[test]
fn e2e_raycast_hits_entity_through_world() {
    let mut engine = Engine::new(320, 240);

    // Spawn a circle obstacle at (100, 0) with radius 10
    let obstacle = engine.world.spawn();
    engine.world.transforms.insert(obstacle, Transform {
        x: 100.0, y: 0.0, rotation: 0.0, scale: 1.0,
    });
    engine.world.colliders.insert(obstacle, Collider {
        shape: ColliderShape::Circle { radius: 10.0 },
        is_trigger: false,
    });

    // Also spawn an entity not on the ray path
    let off_path = engine.world.spawn();
    engine.world.transforms.insert(off_path, Transform {
        x: 50.0, y: 999.0, rotation: 0.0, scale: 1.0,
    });
    engine.world.colliders.insert(off_path, Collider {
        shape: ColliderShape::Circle { radius: 10.0 },
        is_trigger: false,
    });

    // Fire ray from origin along +X axis
    let ray = crate::raycast::Ray::new(0.0, 0.0, 1.0, 0.0);
    let hit = crate::raycast::raycast(&engine.world, &ray, 200.0);

    assert!(hit.is_some(), "ray should hit the obstacle circle");
    let hit = hit.unwrap();
    assert_eq!(
        hit.entity, obstacle,
        "hit entity should be the obstacle, got {:?}",
        hit.entity,
    );
    // Distance to circle edge = 100 - 10 = 90
    assert!(
        approx_eq(hit.distance, 90.0, 0.5),
        "hit distance should be ~90, got {}",
        hit.distance,
    );

    // Verify off-path entity is not hit
    let hits_all = crate::raycast::raycast_all(&engine.world, &ray, 200.0);
    assert!(
        !hits_all.iter().any(|h| h.entity == off_path),
        "off-path entity should not appear in raycast results",
    );
}

// ─── E2E-11. Entity Pool lifecycle: prewarm, acquire, release ───────────────
//
// Create a pool in the engine's PoolRegistry, prewarm it, acquire entities,
// and release them.  Verify counts at each stage.

#[test]
fn e2e_entity_pool_prewarm_acquire_release_cycle() {
    let mut engine = Engine::new(320, 240);

    // Create a pool of 5 "bullet" entities
    {
        let pool = engine.pool_registry.create_pool("bullet", 5);
        pool.prewarm(&mut engine.world);
    }

    // All 5 should be in the world and available in the pool
    assert_eq!(
        engine.world.entity_count(), 5,
        "prewarm should have spawned 5 entities into the world",
    );
    assert_eq!(
        engine.pool_registry.get("bullet").unwrap().available_count(), 5,
    );
    assert_eq!(
        engine.pool_registry.get("bullet").unwrap().active_count(), 0,
    );

    // Acquire 3 entities
    let b1 = engine.pool_registry.acquire("bullet").expect("should acquire bullet 1");
    let b2 = engine.pool_registry.acquire("bullet").expect("should acquire bullet 2");
    let b3 = engine.pool_registry.acquire("bullet").expect("should acquire bullet 3");

    assert_eq!(engine.pool_registry.get("bullet").unwrap().available_count(), 2);
    assert_eq!(engine.pool_registry.get("bullet").unwrap().active_count(), 3);

    // Release 2 of them
    assert!(engine.pool_registry.release("bullet", b1), "b1 release should succeed");
    assert!(engine.pool_registry.release("bullet", b2), "b2 release should succeed");

    assert_eq!(engine.pool_registry.get("bullet").unwrap().available_count(), 4);
    assert_eq!(engine.pool_registry.get("bullet").unwrap().active_count(), 1);

    // b3 is still active; verify it's tracked correctly
    assert!(engine.pool_registry.get("bullet").unwrap().contains(b3));

    // Release b3 — pool should be full again
    engine.pool_registry.release("bullet", b3);
    assert_eq!(engine.pool_registry.get("bullet").unwrap().available_count(), 5);
    assert_eq!(engine.pool_registry.get("bullet").unwrap().active_count(), 0);
    assert!(!engine.pool_registry.get("bullet").unwrap().is_empty());

    // A released entity can be re-acquired
    let re_acquired = engine.pool_registry.acquire("bullet");
    assert!(re_acquired.is_some(), "should be able to re-acquire after release");
}

// ─── E2E-12. Spatial query: insert entities, query radius ───────────────────
//
// Build a SpatialHashGrid with entities at known positions.  Query a radius
// and verify that only entities within range are returned.

#[test]
fn e2e_spatial_query_radius_returns_correct_entities() {
    let mut engine = Engine::new(320, 240);

    // Spawn entities at various positions
    let near1 = engine.world.spawn();
    let near2 = engine.world.spawn();
    let far   = engine.world.spawn();

    engine.world.transforms.insert(near1, Transform { x: 10.0, y: 10.0, rotation: 0.0, scale: 1.0 });
    engine.world.transforms.insert(near2, Transform { x: 20.0, y: 5.0,  rotation: 0.0, scale: 1.0 });
    engine.world.transforms.insert(far,   Transform { x: 500.0, y: 500.0, rotation: 0.0, scale: 1.0 });

    // Build spatial grid from engine world transforms
    let mut grid = crate::spatial_query::SpatialHashGrid::new(64.0);
    for (entity, transform) in engine.world.transforms.iter() {
        grid.insert(entity, transform.x, transform.y);
    }

    // Query 50-unit radius around (15, 8)
    let candidates = grid.query_radius(15.0, 8.0, 50.0);

    assert!(
        candidates.contains(&near1),
        "near1 at (10,10) should be within 50 units of (15,8)",
    );
    assert!(
        candidates.contains(&near2),
        "near2 at (20,5) should be within 50 units of (15,8)",
    );
    assert!(
        !candidates.contains(&far),
        "far at (500,500) should not be within 50 units of (15,8)",
    );

    // Nearest entity from origin (0,0) should be near1 or near2
    let nearest = grid.nearest(0.0, 0.0, 100.0, |e| {
        engine.world.transforms.get(e).map(|t| (t.x, t.y))
    });
    assert!(
        nearest == Some(near1) || nearest == Some(near2),
        "nearest to (0,0) should be near1 or near2, got {:?}",
        nearest,
    );
}

// ─── E2E-13. Multi-system: FSM + Coroutine + Tween all active simultaneously ─
//
// One entity simultaneously has:
//   - A StateMachine cycling idle→active after 0.2s
//   - A Coroutine that waits 0.3s then sets "coroutine_done"
//   - A PropertyTween moving X from 0→50 over 0.4s
//
// After ticking through all relevant durations, verify all three systems
// completed their work correctly without interfering with each other.

#[test]
fn e2e_multi_system_fsm_coroutine_tween_all_active_simultaneously() {
    let mut engine = Engine::new(320, 240);

    let e = engine.world.spawn();
    engine.world.transforms.insert(e, Transform {
        x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0,
    });
    engine.world.game_states.insert(e, GameState::new());

    // FSM: idle → active after 0.2s
    let mut sm = StateMachine::new("idle");
    sm.add_transition("idle", "active", TransitionCondition::After(0.2));
    engine.world.state_machines.insert(e, sm);

    // Coroutine: wait 0.3s then set game state
    engine.world.coroutines.insert(e, Coroutine::new("test")
        .then_wait(0.3)
        .then_set_state("coroutine_done", 1.0));

    // Tween: X from 0 to 50 over 0.4s
    engine.world.property_tweens.insert(e, PropertyTween::new()
        .with_tween(Tween::new(TweenTarget::X, 0.0, 50.0, 0.4, EasingFn::Linear)));

    // After 0.25s: FSM should be in "active", coroutine still waiting, tween in progress
    for _ in 0..(0.25 / DT) as u32 {
        engine.tick(DT);
    }
    assert!(
        engine.world.state_machines.get(e).unwrap().is_in("active"),
        "FSM should be in active at 0.25s",
    );
    let x_mid = engine.world.transforms.get(e).map(|t| t.x).unwrap_or(-1.0);
    assert!(
        x_mid > 5.0 && x_mid < 45.0,
        "tween should be mid-progress at 0.25s, x={}",
        x_mid,
    );
    // Coroutine waits 0.3s, so at 0.25s it's still pending
    assert!(
        engine.world.coroutines.get(e).is_some(),
        "coroutine should not be done at 0.25s",
    );

    // After 0.5s total: all three should have completed/settled
    for _ in 0..(0.3 / DT) as u32 {
        engine.tick(DT);
    }
    // FSM: still in active (no further transitions defined)
    assert!(
        engine.world.state_machines.get(e).unwrap().is_in("active"),
        "FSM should remain in active",
    );
    // Coroutine: done (removed) after 0.3s wait
    assert!(
        engine.world.coroutines.get(e).is_none(),
        "coroutine should be removed after completing at 0.3s",
    );
    let done_val = engine.world.game_states.get(e)
        .map(|gs| gs.get("coroutine_done"))
        .unwrap_or(0.0);
    assert!(
        approx_eq(done_val, 1.0, 1e-6),
        "coroutine should have set coroutine_done=1.0, got {}",
        done_val,
    );
    // Tween: done (removed) after 0.4s, X should be at ~50
    assert!(
        !engine.world.property_tweens.has(e),
        "tween should be removed after completing at 0.4s",
    );
    let x_final = engine.world.transforms.get(e).map(|t| t.x).unwrap_or(-1.0);
    assert!(
        x_final >= 49.0,
        "tween should have brought X to ~50, got {}",
        x_final,
    );
}

// ─── E2E-14. Stress test: spawn 1000 entities and tick ──────────────────────
//
// Spawn 1000 entities with transforms and rigidbodies, then run 5 ticks.
// Must not panic and must complete in a reasonable time.

#[test]
fn e2e_stress_test_1000_entities_tick_no_panic() {
    let mut engine = Engine::new(320, 240);

    for i in 0..1000u32 {
        let e = engine.world.spawn();
        engine.world.transforms.insert(e, Transform {
            x: (i % 100) as f64 * 3.0,
            y: (i / 100) as f64 * 3.0,
            rotation: 0.0,
            scale: 1.0,
        });
        engine.world.rigidbodies.insert(e, RigidBody {
            vx: (i % 7) as f64 - 3.0,
            vy: (i % 5) as f64 - 2.0,
            mass: 1.0,
            damping: 0.05,
            ..RigidBody::default()
        });
    }

    assert_eq!(engine.world.entity_count(), 1000);

    // Run 5 full ticks — must not panic
    for _ in 0..5 {
        engine.tick(DT);
    }

    // Verify engine is still in a consistent state
    assert_eq!(engine.frame, 5);
    assert!(engine.time > 0.0);
    // All 1000 entities should still be alive (no lifetimes set)
    assert_eq!(
        engine.world.entity_count(), 1000,
        "all 1000 entities should still be alive after 5 ticks",
    );
}

// ─── E2E-15. Tween Y-axis: entity moves from top to bottom ─────────────────
//
// Verify that TweenTarget::Y works correctly end-to-end through engine tick.

#[test]
fn e2e_tween_y_axis_moves_entity_top_to_bottom() {
    let mut engine = Engine::new(320, 240);

    let e = engine.world.spawn();
    engine.world.transforms.insert(e, Transform {
        x: 160.0, y: 0.0, rotation: 0.0, scale: 1.0,
    });
    engine.world.property_tweens.insert(e, PropertyTween::new()
        .with_tween(Tween::new(TweenTarget::Y, 0.0, 240.0, 1.0, EasingFn::Linear)));

    // After 0.25s, Y should be near 60
    for _ in 0..(0.25 / DT) as u32 {
        engine.tick(DT);
    }
    let y_quarter = engine.world.transforms.get(e).map(|t| t.y).unwrap_or(-1.0);
    assert!(
        y_quarter > 40.0 && y_quarter < 80.0,
        "Y should be near 60 at t=0.25s, got {}",
        y_quarter,
    );

    // After full duration
    for _ in 0..(0.85 / DT) as u32 {
        engine.tick(DT);
    }
    let y_final = engine.world.transforms.get(e).map(|t| t.y).unwrap_or(-1.0);
    assert!(
        y_final >= 235.0,
        "Y should be near 240 at tween completion, got {}",
        y_final,
    );
    assert!(
        !engine.world.property_tweens.has(e),
        "PropertyTween should be removed after Y tween completes",
    );
}

// ─── E2E-16. Waypoint Once mode: entity stops at final waypoint ─────────────
//
// Entity follows a two-waypoint path in Once mode. After reaching the second
// waypoint it should remain there, not overshoot or restart.

#[test]
fn e2e_waypoint_once_mode_entity_stops_at_final_waypoint() {
    let mut engine = Engine::new(320, 240);

    let e = engine.world.spawn();
    engine.world.transforms.insert(e, Transform {
        x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0,
    });
    engine.world.waypoint_paths.insert(e, WaypointPath::new(
        vec![
            Waypoint::new(50.0, 0.0),
            Waypoint::new(100.0, 0.0),
        ],
        500.0,   // very fast so we overshoot easily if not clamped
        WaypointMode::Once,
    ));

    // Run long enough to definitely reach all waypoints
    for _ in 0..(2.0 / DT) as u32 {
        engine.tick(DT);
    }

    let x_final = engine.world.transforms.get(e).map(|t| t.x).unwrap_or(-1.0);
    assert!(
        x_final >= 99.0 && x_final <= 101.0,
        "entity should be stopped at the final waypoint x=100, got {}",
        x_final,
    );
}

// ─── E2E-17. Signal receiver triggered/released edge detection via tick ──────
//
// Activate a signal, tick, verify receiver just_triggered; deactivate,
// tick, verify just_released.  This exercises the full signal → receiver
// pipeline through engine tick.

#[test]
fn e2e_signal_receiver_edge_detection_through_tick() {
    let mut engine = Engine::new(320, 240);

    let emitter_entity = engine.world.spawn();
    engine.world.signal_emitters.insert(
        emitter_entity,
        SignalEmitter::new("switch", false),
    );

    let receiver_entity = engine.world.spawn();
    engine.world.signal_receivers.insert(
        receiver_entity,
        SignalReceiver::new(vec!["switch".into()], true),
    );

    // Initial tick — no signal active
    engine.tick(DT);
    {
        let recv = engine.world.signal_receivers.get(receiver_entity).unwrap();
        assert!(!recv.triggered, "should not be triggered initially");
        assert!(!recv.just_triggered(), "should not report just_triggered initially");
    }

    // Activate the signal and tick
    engine.world.signal_emitters.get_mut(emitter_entity).unwrap().active = true;
    engine.tick(DT);
    {
        let recv = engine.world.signal_receivers.get(receiver_entity).unwrap();
        assert!(recv.triggered, "should be triggered after signal activates");
        assert!(recv.just_triggered(), "just_triggered should be true on first triggered frame");
    }

    // Second tick with signal still active — just_triggered should clear
    engine.tick(DT);
    {
        let recv = engine.world.signal_receivers.get(receiver_entity).unwrap();
        assert!(recv.triggered, "still triggered");
        assert!(!recv.just_triggered(), "just_triggered should clear on subsequent frame");
    }

    // Deactivate signal and tick
    engine.world.signal_emitters.get_mut(emitter_entity).unwrap().active = false;
    engine.tick(DT);
    {
        let recv = engine.world.signal_receivers.get(receiver_entity).unwrap();
        assert!(!recv.triggered, "should not be triggered after signal deactivates");
        assert!(recv.just_released(), "just_released should be true on deactivation frame");
    }
}

// ─── E2E-18. Tilemap: fill rect and solid count round-trip ──────────────────
//
// Build a tilemap via the engine, fill a region, verify solid_count,
// then clear and verify it returns to zero.

#[test]
fn e2e_tilemap_fill_rect_and_solid_count_round_trip() {
    let mut engine = Engine::new(640, 480);

    let mut tilemap = TileMap::new(20, 15, 16.0);
    assert_eq!(tilemap.solid_count(), 0, "fresh tilemap should have no solid tiles");

    // Fill a 5×3 region with solid tiles at (2, 2)
    tilemap.fill_rect(2, 2, 5, 3, Tile::solid(Color::WHITE));
    assert_eq!(
        tilemap.solid_count(), 15,
        "5*3 = 15 solid tiles expected after fill_rect",
    );

    // Setting one tile inside to empty should reduce count
    tilemap.set(2, 2, Tile::empty());
    assert_eq!(tilemap.solid_count(), 14);

    // Clear everything
    tilemap.clear();
    assert_eq!(tilemap.solid_count(), 0, "solid count should be 0 after clear");
    for tile in &tilemap.tiles {
        assert_eq!(
            tile.tile_type, TileType::Empty,
            "all tiles should be empty after clear",
        );
    }

    engine.tilemap = Some(tilemap);
    // Engine should tick cleanly with a tilemap loaded
    engine.tick(DT);
    assert_eq!(engine.frame, 1, "engine should tick normally with tilemap loaded");
}

// ─── E2E-19. Hierarchy: 3-level deep grandparent chain ──────────────────────
//
// Grandparent → Parent → Child, all in a line on X.
// After tick(), the child's world transform should be the sum of all offsets.

#[test]
fn e2e_hierarchy_three_level_grandparent_chain() {
    let mut engine = Engine::new(320, 240);

    let grandparent = engine.world.spawn();
    engine.world.transforms.insert(grandparent, Transform {
        x: 100.0, y: 0.0, rotation: 0.0, scale: 1.0,
    });

    let parent = engine.world.spawn();
    engine.world.transforms.insert(parent, Transform {
        x: 50.0, y: 0.0, rotation: 0.0, scale: 1.0,
    });
    engine.world.parents.insert(parent, Parent::new(grandparent));

    let child = engine.world.spawn();
    engine.world.transforms.insert(child, Transform {
        x: 25.0, y: 0.0, rotation: 0.0, scale: 1.0,
    });
    engine.world.parents.insert(child, Parent::new(parent));

    engine.tick(DT);

    // Grandparent world: x=100
    // Parent world: x=100+50=150
    // Child world: x=150+25=175
    let child_wt = engine.world.world_transforms.get(child)
        .expect("child should have world transform");
    assert!(
        approx_eq(child_wt.x, 175.0, 0.1),
        "child world X should be 175 (100+50+25), got {}",
        child_wt.x,
    );

    let parent_wt = engine.world.world_transforms.get(parent)
        .expect("parent should have world transform");
    assert!(
        approx_eq(parent_wt.x, 150.0, 0.1),
        "parent world X should be 150 (100+50), got {}",
        parent_wt.x,
    );
}

// ─── E2E-20. State machine StateCheck condition via game state ───────────────
//
// Entity starts "alive". Health decreases via game_state. Once health ≤ 0,
// the FSM should transition to "dead" via StateCheck condition.

#[test]
fn e2e_state_machine_state_check_health_goes_to_dead() {
    let mut engine = Engine::new(320, 240);

    let e = engine.world.spawn();
    let mut gs = GameState::new();
    gs.set("health", 100.0);
    engine.world.game_states.insert(e, gs);

    let mut sm = StateMachine::new("alive");
    sm.add_transition("alive", "dead", TransitionCondition::StateCheck {
        key: "health".to_string(),
        op: CompareOp::Lte,
        value: 0.0,
    });
    engine.world.state_machines.insert(e, sm);

    // Tick once — still alive
    engine.tick(DT);
    assert!(engine.world.state_machines.get(e).unwrap().is_in("alive"));

    // Set health to zero
    engine.world.game_states.get_mut(e).unwrap().set("health", 0.0);

    // One more tick — FSM should transition to dead
    engine.tick(DT);
    assert!(
        engine.world.state_machines.get(e).unwrap().is_in("dead"),
        "entity should transition to dead when health <= 0",
    );
}

// ─── E2E-21. Stress test with hierarchy: 100 parent-child pairs ─────────────
//
// Create 100 parent-child pairs and tick. Verify that all children have
// their WorldTransform computed correctly and no panic occurs.

#[test]
fn e2e_stress_100_parent_child_pairs_all_get_world_transforms() {
    let mut engine = Engine::new(320, 240);

    let mut children = Vec::with_capacity(100);

    for i in 0..100u32 {
        let parent = engine.world.spawn();
        engine.world.transforms.insert(parent, Transform {
            x: i as f64 * 10.0, y: 0.0, rotation: 0.0, scale: 1.0,
        });

        let child = engine.world.spawn();
        engine.world.transforms.insert(child, Transform {
            x: 5.0, y: 0.0, rotation: 0.0, scale: 1.0,
        });
        engine.world.parents.insert(child, Parent::new(parent));
        children.push((child, i as f64 * 10.0 + 5.0));
    }

    engine.tick(DT);

    // Every child should have WorldTransform = parent.x + 5.0
    let mut pass = true;
    for (child, expected_x) in &children {
        if let Some(wt) = engine.world.world_transforms.get(*child) {
            if !approx_eq(wt.x, *expected_x, 0.1) {
                pass = false;
                break;
            }
        } else {
            pass = false;
            break;
        }
    }
    assert!(pass, "all 100 children should have correct world transforms");
}

// ─── E2E-22. Coroutine cascade: multiple set_state steps in one tick ─────────
//
// A coroutine with no wait steps (just set/add state) should execute all
// steps in a single engine tick due to cascade behavior.

#[test]
fn e2e_coroutine_all_non_wait_steps_cascade_in_single_tick() {
    let mut engine = Engine::new(320, 240);

    let e = engine.world.spawn();
    engine.world.game_states.insert(e, GameState::new());
    engine.world.coroutines.insert(e, Coroutine::new("cascade")
        .then_set_state("a", 10.0)
        .then_set_state("b", 20.0)
        .then_add_state("a", 5.0)   // a becomes 15
        .then_set_state("c", 99.0));

    // Only one tick needed for all cascading steps
    engine.tick(DT);

    let gs = engine.world.game_states.get(e).expect("entity should have game state");
    assert!(
        approx_eq(gs.get("a"), 15.0, 1e-6),
        "a should be 15 (10 + 5 from add_state), got {}",
        gs.get("a"),
    );
    assert!(
        approx_eq(gs.get("b"), 20.0, 1e-6),
        "b should be 20, got {}",
        gs.get("b"),
    );
    assert!(
        approx_eq(gs.get("c"), 99.0, 1e-6),
        "c should be 99, got {}",
        gs.get("c"),
    );
    assert!(
        engine.world.coroutines.get(e).is_none(),
        "coroutine should be removed after all steps cascade in one tick",
    );
}
