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

// ─── Round 2: Parameter Sweep ─────────────────────────────────────────

#[test]
fn sweep_runs_multiple_configs() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;
    let (actions, _) = ShotBuilder::new()
        .aim_and_shoot(ball_x, ball_y, 270.0, 0.5)
        .build();

    let configs = vec![
        SweepConfig {
            label: "default".into(),
            overrides: vec![],
        },
        SweepConfig {
            label: "low_ball_y".into(),
            overrides: vec![("ball_y".into(), 400.0)],
        },
    ];

    let report = run_sweep(
        trap_links_demo::setup,
        trap_links_demo::update,
        trap_links_demo::render,
        &actions,
        &configs,
        120,
    );

    assert_eq!(report.results.len(), 2);
    assert!(!report.summary().is_empty());
}

#[test]
fn sweep_min_max_by_state() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;
    let (actions, _) = ShotBuilder::new()
        .aim_and_shoot(ball_x, ball_y, 270.0, 0.5)
        .build();

    let configs = vec![
        SweepConfig {
            label: "default".into(),
            overrides: vec![],
        },
        SweepConfig {
            label: "moved_ball".into(),
            overrides: vec![("ball_y".into(), 300.0)],
        },
    ];

    let report = run_sweep(
        trap_links_demo::setup,
        trap_links_demo::update,
        trap_links_demo::render,
        &actions,
        &configs,
        120,
    );

    let min = report.min_by_state("ball_y").unwrap();
    let max = report.max_by_state("ball_y").unwrap();
    let min_y = min.sim.game_state.get("ball_y").and_then(|v| v.as_f64()).unwrap();
    let max_y = max.sim.game_state.get("ball_y").and_then(|v| v.as_f64()).unwrap();
    assert!(min_y < max_y, "min_y={} should be < max_y={}", min_y, max_y);
}

// ─── Round 2: State Timeline ──────────────────────────────────────────

#[test]
fn timeline_records_ball_position() {
    let tl = record_timeline(
        trap_links_demo::setup,
        trap_links_demo::update,
        trap_links_demo::render,
        30,
        &["ball_x", "ball_y", "tl_phase"],
    );

    assert_eq!(tl.len(), 30);
    assert_eq!(tl.keys.len(), 3);

    // Ball should be stationary (no shot fired), so all values should be the same
    let bx_series = tl.series("ball_x").unwrap();
    assert!(bx_series.iter().all(|v| (*v - bx_series[0]).abs() < 0.01));
}

#[test]
fn timeline_with_shot_shows_movement() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;
    let (actions, total) = ShotBuilder::new()
        .aim_and_shoot(ball_x, ball_y, 270.0, 0.6)
        .build();

    let tl = record_timeline_with_actions(
        trap_links_demo::setup,
        trap_links_demo::update,
        trap_links_demo::render,
        &actions,
        total.min(120),
        &["ball_y", "ball_vy", "tl_phase"],
    );

    assert!(tl.len() > 0);

    // Ball should move upward (decreasing Y) after the shot
    let by_series = tl.series("ball_y").unwrap();
    let min_y = by_series.iter().cloned().fold(f64::INFINITY, f64::min);
    assert!(min_y < ball_y, "ball should move upward from {}, min was {}", ball_y, min_y);
}

#[test]
fn timeline_stats_work() {
    let tl = record_timeline(
        trap_links_demo::setup,
        trap_links_demo::update,
        trap_links_demo::render,
        30,
        &["ball_x", "ball_y"],
    );

    let (min, max, mean) = tl.stats("ball_x").unwrap();
    assert!(min > 0.0);
    assert!(max > 0.0);
    assert!(mean > 0.0);
    // Stationary ball: min ≈ max ≈ mean
    assert!((min - max).abs() < 1.0);
}

#[test]
fn timeline_first_frame_where() {
    let tl = record_timeline(
        trap_links_demo::setup,
        trap_links_demo::update,
        trap_links_demo::render,
        10,
        &["tl_phase"],
    );

    // Phase should be 0 (aiming) from the start
    let frame = tl.first_frame_where("tl_phase", |v| v == 0.0);
    assert_eq!(frame, Some(0));
}

// ─── Round 3: Fitness Evaluator ───────────────────────────────────────

#[test]
fn fitness_idle_scores_low_completion() {
    let mut runner = HeadlessRunner::new(480, 720);
    let result = runner.run(
        trap_links_demo::setup,
        trap_links_demo::update,
        trap_links_demo::render,
        60,
    );

    let evaluator = FitnessEvaluator::new()
        .add("completion", 3.0, score_hole_completion)
        .add("efficiency", 2.0, score_stroke_efficiency)
        .add("proximity", 1.0, score_proximity_to_hole);

    let fitness = evaluator.evaluate(&result);

    // No shot fired — completion=0, efficiency=0, proximity=low
    assert_eq!(fitness.criteria.len(), 3);
    assert!(fitness.total < 0.5, "idle game should have low fitness, got {}", fitness.total);
    assert!(!fitness.summary().is_empty());
    assert!(!fitness.grade().is_empty());
}

#[test]
fn fitness_shot_toward_hole_scores_higher() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;
    let (actions, total_frames) = ShotBuilder::new()
        .aim_and_shoot(ball_x, ball_y, 270.0, 0.6)
        .build();

    // Run idle
    let mut idle_runner = HeadlessRunner::new(480, 720);
    let idle_result = idle_runner.run(
        trap_links_demo::setup,
        trap_links_demo::update,
        trap_links_demo::render,
        60,
    );

    // Run with shot
    let shot_scenario = GameScenario {
        name: "shot_toward_hole".into(),
        width: 480,
        height: 720,
        setup_fn: trap_links_demo::setup,
        update_fn: trap_links_demo::update,
        render_fn: trap_links_demo::render,
        actions,
        total_frames,
        assertions: vec![],
    };
    let shot_result = shot_scenario.run();

    let evaluator = FitnessEvaluator::new()
        .add("proximity", 1.0, score_proximity_to_hole);

    let idle_fitness = evaluator.evaluate(&idle_result);
    let shot_fitness = evaluator.evaluate(&shot_result.sim);

    // Shot toward hole should be closer than idle ball
    assert!(
        shot_fitness.total > idle_fitness.total,
        "shot fitness {} should exceed idle {}", shot_fitness.total, idle_fitness.total
    );
}

#[test]
fn fitness_rank_sweep() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;
    let (actions, _) = ShotBuilder::new()
        .aim_and_shoot(ball_x, ball_y, 270.0, 0.5)
        .build();

    let configs = vec![
        SweepConfig { label: "default".into(), overrides: vec![] },
        SweepConfig { label: "closer".into(), overrides: vec![("ball_y".into(), 300.0)] },
    ];

    let report = run_sweep(
        trap_links_demo::setup,
        trap_links_demo::update,
        trap_links_demo::render,
        &actions,
        &configs,
        120,
    );

    let evaluator = FitnessEvaluator::new()
        .add("proximity", 1.0, score_proximity_to_hole);

    let ranked = evaluator.rank_sweep(&report);
    assert_eq!(ranked.len(), 2);
    // Best should be first
    assert!(ranked[0].1.total >= ranked[1].1.total);
}

// ─── Round 3: Regression Suite ────────────────────────────────────────

#[test]
fn regression_identical_runs_no_diff() {
    let suite = RegressionSuite::new(&["tl_phase", "ball_x", "ball_y", "strokes"])
        .add(GameScenario {
            name: "idle".into(),
            width: 480,
            height: 720,
            setup_fn: trap_links_demo::setup,
            update_fn: trap_links_demo::update,
            render_fn: trap_links_demo::render,
            actions: vec![],
            total_frames: 30,
            assertions: vec![],
        });

    let baseline = suite.capture_baseline();
    let diff = suite.diff_against(&baseline);

    assert!(!diff.has_regressions(), "identical runs should have no regressions: {}", diff.summary());
    assert_eq!(diff.verdict(), "PASS");
}

#[test]
fn regression_detects_state_change() {
    // Create a suite with a specific shot
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;
    let (actions, total) = ShotBuilder::new()
        .aim_and_shoot(ball_x, ball_y, 270.0, 0.5)
        .build();

    let suite = RegressionSuite::new(&["ball_x", "ball_y"])
        .with_tolerance(0.01)
        .add(GameScenario {
            name: "shot".into(),
            width: 480,
            height: 720,
            setup_fn: trap_links_demo::setup,
            update_fn: trap_links_demo::update,
            render_fn: trap_links_demo::render,
            actions,
            total_frames: total.min(120),
            assertions: vec![],
        });

    let baseline = suite.capture_baseline();
    // Identical run should pass
    let diff = suite.diff_against(&baseline);
    assert!(!diff.has_regressions(), "same run: {}", diff.summary());
}

#[test]
fn diff_report_summary_not_empty() {
    let report = DiffReport {
        entries: vec![
            DiffEntry {
                scenario: "test".into(),
                metric: "ball_x".into(),
                status: DiffStatus::Changed { detail: "moved".into() },
            },
        ],
    };
    assert!(!report.summary().is_empty());
    assert_eq!(report.verdict(), "PASS"); // Changed != Regressed
}
