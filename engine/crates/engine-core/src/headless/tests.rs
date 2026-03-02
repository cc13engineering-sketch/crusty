use super::*;
use crate::trap_links_demo;

// ─── HeadlessRunner basics ──────────────────────────────────────────────

#[test]
fn headless_runner_runs_frames() {
    let mut runner = HeadlessRunner::new(480, 720);
    let result = runner.run(
        trap_links_demo::setup,
        trap_links_demo::update,
        trap_links_demo::render,
        60,
    );
    assert_eq!(result.frames_run, 60);
    assert!(result.elapsed_sim_time > 0.9); // ~1 second at 60fps
}

#[test]
fn headless_runner_game_state_populated() {
    let mut runner = HeadlessRunner::new(480, 720);
    let result = runner.run(
        trap_links_demo::setup,
        trap_links_demo::update,
        trap_links_demo::render,
        1,
    );
    // After setup, ball position should be set
    let ball_x = result.game_state.get("ball_x").and_then(|v| v.as_f64());
    assert!(ball_x.is_some(), "ball_x should be in game state");
    assert!(ball_x.unwrap() > 0.0, "ball_x should be positive");
}

#[test]
fn headless_runner_framebuffer_hash_nonzero() {
    let mut runner = HeadlessRunner::new(480, 720);
    let result = runner.run(
        trap_links_demo::setup,
        trap_links_demo::update,
        trap_links_demo::render,
        1,
    );
    assert_ne!(result.framebuffer_hash, 0, "hash should be non-zero after rendering");
}

#[test]
fn headless_runner_deterministic() {
    let run = || {
        let mut runner = HeadlessRunner::new(480, 720);
        runner.run(
            trap_links_demo::setup,
            trap_links_demo::update,
            trap_links_demo::render,
            30,
        )
    };
    let r1 = run();
    let r2 = run();
    assert_eq!(r1.framebuffer_hash, r2.framebuffer_hash, "deterministic runs should produce identical framebuffers");
    assert_eq!(
        r1.game_state.get("ball_x").and_then(|v| v.as_f64()),
        r2.game_state.get("ball_x").and_then(|v| v.as_f64()),
    );
}

// ─── GameScenario ──────────────────────────────────────────────────────

#[test]
fn scenario_no_input_stays_in_aiming_phase() {
    let result = GameScenario {
        name: "idle".into(),
        width: 480,
        height: 720,
        setup_fn: trap_links_demo::setup,
        update_fn: trap_links_demo::update,
        render_fn: trap_links_demo::render,
        actions: vec![],
        total_frames: 60,
        assertions: vec![
            Assertion::StateEquals {
                key: "tl_phase".into(),
                expected: 0.0,
                tolerance: 0.0,
            },
            Assertion::StateEquals {
                key: "strokes".into(),
                expected: 0.0,
                tolerance: 0.0,
            },
        ],
    }.run();

    assert!(result.all_passed(), "{}", result.failure_report());
}

#[test]
fn scenario_shoot_increments_strokes() {
    // Shoot by tapping at ball then releasing offset
    let ball_x = 15.0 * 16.0; // 240.0
    let ball_y = 32.0 * 16.0; // 512.0

    let result = GameScenario {
        name: "shoot_once".into(),
        width: 480,
        height: 720,
        setup_fn: trap_links_demo::setup,
        update_fn: trap_links_demo::update,
        render_fn: trap_links_demo::render,
        actions: vec![
            ScheduledAction::PointerDown { frame: 5, x: ball_x, y: ball_y },
            ScheduledAction::PointerUp { frame: 5, x: ball_x, y: ball_y + 60.0 },
        ],
        total_frames: 120,
        assertions: vec![
            Assertion::StateEquals {
                key: "strokes".into(),
                expected: 1.0,
                tolerance: 0.0,
            },
        ],
    }.run();

    assert!(result.all_passed(), "{}", result.failure_report());
}

// ─── ShotBuilder ───────────────────────────────────────────────────────

#[test]
fn shot_builder_produces_actions() {
    let (actions, total_frames) = ShotBuilder::new()
        .aim_and_shoot(240.0, 512.0, 270.0, 0.8) // shoot upward
        .build();

    assert_eq!(actions.len(), 2); // pointer_down + pointer_up
    assert!(total_frames > 60);
}

#[test]
fn shot_builder_multi_shot() {
    let (actions, _) = ShotBuilder::new()
        .aim_and_shoot(240.0, 512.0, 270.0, 0.5)
        .wait(60)
        .aim_and_shoot(240.0, 400.0, 270.0, 0.3)
        .build();

    assert_eq!(actions.len(), 4); // 2 per shot
}

#[test]
fn shot_builder_shoot_upward_moves_ball() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;

    let (actions, total_frames) = ShotBuilder::new()
        .aim_and_shoot(ball_x, ball_y, 270.0, 0.6)
        .build();

    let result = GameScenario {
        name: "shoot_upward".into(),
        width: 480,
        height: 720,
        setup_fn: trap_links_demo::setup,
        update_fn: trap_links_demo::update,
        render_fn: trap_links_demo::render,
        actions,
        total_frames,
        assertions: vec![
            Assertion::StateEquals {
                key: "strokes".into(),
                expected: 1.0,
                tolerance: 0.0,
            },
        ],
    }.run();

    assert!(result.all_passed(), "{}", result.failure_report());

    // Verify ball has moved upward from starting position
    let final_y = result.sim.game_state.get("ball_y").and_then(|v| v.as_f64()).unwrap();
    assert!(final_y < ball_y, "ball should have moved up, was {} now {}", ball_y, final_y);
}
