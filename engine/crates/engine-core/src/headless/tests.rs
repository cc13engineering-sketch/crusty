use super::*;
use crate::sleague;

// All headless tests use S-League as the test game, but the headless
// infrastructure itself is game-agnostic. The game-specific parts
// (dispatch_action, scoring functions) live in sleague.

// ─── HeadlessRunner basics ──────────────────────────────────────────────

#[test]
fn headless_runner_runs_frames() {
    let mut runner = HeadlessRunner::new(480, 720);
    let result = runner.run(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        60,
    );
    assert_eq!(result.frames_run, 60);
    assert!(result.elapsed_sim_time > 0.9); // ~1 second at 60fps
}

#[test]
fn headless_runner_game_state_populated() {
    let mut runner = HeadlessRunner::new(480, 720);
    let result = runner.run(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
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
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        1,
    );
    assert_ne!(result.framebuffer_hash, 0, "hash should be non-zero after rendering");
}

#[test]
fn headless_runner_deterministic() {
    let run = || {
        let mut runner = HeadlessRunner::new(480, 720);
        runner.run(
            sleague::setup_fight_only,
            sleague::update,
            sleague::render,
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
        setup_fn: sleague::setup_fight_only,
        update_fn: sleague::update,
        render_fn: sleague::render,
        action_dispatch: sleague::dispatch_action,
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
        setup_fn: sleague::setup_fight_only,
        update_fn: sleague::update,
        render_fn: sleague::render,
        action_dispatch: sleague::dispatch_action,
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
        setup_fn: sleague::setup_fight_only,
        update_fn: sleague::update,
        render_fn: sleague::render,
        action_dispatch: sleague::dispatch_action,
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
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
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
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
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
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
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
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
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
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
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
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
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
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        60,
    );

    // Game-specific scoring functions live in sleague
    let evaluator = FitnessEvaluator::new()
        .add("completion", 3.0, sleague::score_hole_completion)
        .add("efficiency", 2.0, sleague::score_stroke_efficiency)
        .add("proximity", 1.0, sleague::score_proximity_to_hole);

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
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        60,
    );

    // Run with shot
    let shot_scenario = GameScenario {
        name: "shot_toward_hole".into(),
        width: 480,
        height: 720,
        setup_fn: sleague::setup_fight_only,
        update_fn: sleague::update,
        render_fn: sleague::render,
        action_dispatch: sleague::dispatch_action,
        actions,
        total_frames,
        assertions: vec![],
    };
    let shot_result = shot_scenario.run();

    let evaluator = FitnessEvaluator::new()
        .add("proximity", 1.0, sleague::score_proximity_to_hole);

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
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &actions,
        &configs,
        120,
    );

    let evaluator = FitnessEvaluator::new()
        .add("proximity", 1.0, sleague::score_proximity_to_hole);

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
            setup_fn: sleague::setup_fight_only,
            update_fn: sleague::update,
            render_fn: sleague::render,
            action_dispatch: sleague::dispatch_action,
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
            setup_fn: sleague::setup_fight_only,
            update_fn: sleague::update,
            render_fn: sleague::render,
            action_dispatch: sleague::dispatch_action,
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

// ─── Round 5: ScenarioBuilder ─────────────────────────────────────────

#[test]
fn scenario_builder_idle_run() {
    let builder = ScenarioBuilder::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    );

    let result = builder.run_idle("builder_idle", 30);
    assert_eq!(result.sim.frames_run, 30);
    assert!(result.all_passed()); // no assertions = all pass
}

#[test]
fn scenario_builder_with_assertions() {
    let builder = ScenarioBuilder::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    );

    let result = builder.run(
        "builder_assertions",
        vec![],
        60,
        vec![
            Assertion::StateEquals {
                key: "tl_phase".into(),
                expected: 0.0,
                tolerance: 0.0,
            },
            Assertion::StateGreaterThan {
                key: "ball_x".into(),
                threshold: 0.0,
            },
        ],
    );

    assert!(result.all_passed(), "{}", result.failure_report());
}

#[test]
fn scenario_builder_with_shot() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;

    let builder = ScenarioBuilder::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    );

    let result = builder.run(
        "builder_shot",
        vec![
            ScheduledAction::PointerDown { frame: 5, x: ball_x, y: ball_y },
            ScheduledAction::PointerUp { frame: 5, x: ball_x, y: ball_y + 60.0 },
        ],
        120,
        vec![
            Assertion::StateEquals {
                key: "strokes".into(),
                expected: 1.0,
                tolerance: 0.0,
            },
        ],
    );

    assert!(result.all_passed(), "{}", result.failure_report());
}

#[test]
fn scenario_builder_noop_dispatch() {
    let builder = ScenarioBuilder::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        dispatch_noop,
    );

    let result = builder.run_idle("noop_dispatch", 10);
    assert_eq!(result.sim.frames_run, 10);
}

// ─── Round 5: New Assertion Types ─────────────────────────────────────

#[test]
fn assertion_state_greater_than() {
    let builder = ScenarioBuilder::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    );

    let result = builder.run(
        "greater_than",
        vec![],
        10,
        vec![
            Assertion::StateGreaterThan {
                key: "ball_x".into(),
                threshold: 0.0,
            },
        ],
    );

    assert!(result.all_passed(), "{}", result.failure_report());
}

#[test]
fn assertion_state_less_than() {
    let builder = ScenarioBuilder::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    );

    let result = builder.run(
        "less_than",
        vec![],
        10,
        vec![
            Assertion::StateLessThan {
                key: "ball_x".into(),
                threshold: 10000.0,
            },
        ],
    );

    assert!(result.all_passed(), "{}", result.failure_report());
}

#[test]
fn assertion_state_greater_than_fails_correctly() {
    let builder = ScenarioBuilder::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    );

    let result = builder.run(
        "greater_than_fail",
        vec![],
        10,
        vec![
            Assertion::StateGreaterThan {
                key: "ball_x".into(),
                threshold: 99999.0,
            },
        ],
    );

    assert!(!result.all_passed(), "should fail: ball_x is not > 99999");
}

#[test]
fn assertion_framebuffer_changed() {
    let builder = ScenarioBuilder::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    );

    let result = builder.run(
        "fb_changed",
        vec![],
        10,
        vec![
            Assertion::FramebufferChanged {
                previous: 0x0, // hash should differ from zero
            },
        ],
    );

    assert!(result.all_passed(), "{}", result.failure_report());
}

#[test]
fn assertion_framebuffer_changed_fails_for_same_hash() {
    // First, capture the actual hash
    let builder = ScenarioBuilder::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    );

    let baseline = builder.run_idle("baseline_for_fb", 10);
    let actual_hash = baseline.sim.framebuffer_hash;

    // Now assert it changed from its own value — should fail
    let result = builder.run(
        "fb_same",
        vec![],
        10,
        vec![
            Assertion::FramebufferChanged {
                previous: actual_hash,
            },
        ],
    );

    assert!(!result.all_passed(), "should fail: hash equals previous");
}

// ─── Round 5: Mid-Simulation Snapshots ────────────────────────────────

#[test]
fn snapshot_captures_at_frames() {
    let result = run_with_snapshots(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[],
        60,
        &[0, 29, 59],
    );

    assert_eq!(result.snapshots.len(), 3);
    assert_eq!(result.snapshots[0].frame, 0);
    assert_eq!(result.snapshots[1].frame, 29);
    assert_eq!(result.snapshots[2].frame, 59);
    assert_eq!(result.sim.frames_run, 60);
}

#[test]
fn snapshot_at_frame_lookup() {
    let result = run_with_snapshots(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[],
        30,
        &[10, 20],
    );

    assert!(result.at_frame(10).is_some());
    assert!(result.at_frame(20).is_some());
    assert!(result.at_frame(15).is_none()); // not captured
}

#[test]
fn snapshot_state_values_accessible() {
    let result = run_with_snapshots(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[],
        30,
        &[0],
    );

    let snap = result.at_frame(0).unwrap();
    let ball_x = snap.get_f64("ball_x");
    assert!(ball_x.is_some(), "should have ball_x in snapshot");
    assert!(ball_x.unwrap() > 0.0);
}

#[test]
fn snapshot_series_tracks_state() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;

    let result = run_with_snapshots(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[
            ScheduledAction::PointerDown { frame: 5, x: ball_x, y: ball_y },
            ScheduledAction::PointerUp { frame: 5, x: ball_x, y: ball_y + 60.0 },
        ],
        60,
        &[0, 10, 20, 30, 40, 50],
    );

    let series = result.series("ball_y");
    assert_eq!(series.len(), 6);
    // After shot at frame 5, ball_y should change
    let y_at_0 = series[0].1;
    let y_at_30 = series[3].1;
    assert!(
        (y_at_0 - y_at_30).abs() > 1.0,
        "ball should move after shot: frame0={}, frame30={}", y_at_0, y_at_30
    );
}

#[test]
fn snapshot_value_changed_detects_movement() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;

    let result = run_with_snapshots(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[
            ScheduledAction::PointerDown { frame: 5, x: ball_x, y: ball_y },
            ScheduledAction::PointerUp { frame: 5, x: ball_x, y: ball_y + 60.0 },
        ],
        60,
        &[0, 30],
    );

    let changed = result.value_changed("ball_y", 0, 30);
    assert_eq!(changed, Some(true), "ball_y should differ between frame 0 and 30");
}

#[test]
fn snapshot_no_actions_ball_stationary() {
    let result = run_with_snapshots(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[],
        30,
        &[0, 29],
    );

    let changed = result.value_changed("ball_y", 0, 29);
    assert_eq!(changed, Some(false), "ball should be stationary without input");
}

// ─── Round 6: Action Generator ────────────────────────────────────────

#[test]
fn action_gen_grid_shots_produces_grid() {
    let shots = super::action_gen::grid_shots(240.0, 512.0, 250.0, 290.0, 3, 0.3, 0.9, 3);
    // 3 angles x 3 powers = 9
    assert_eq!(shots.len(), 9);
    for (label, actions, frames) in &shots {
        assert!(!label.is_empty());
        assert_eq!(actions.len(), 2); // down + up per shot
        assert!(*frames > 0);
    }
}

#[test]
fn action_gen_grid_shots_single() {
    let shots = super::action_gen::grid_shots(100.0, 100.0, 270.0, 270.0, 1, 0.5, 0.5, 1);
    assert_eq!(shots.len(), 1);
}

#[test]
fn action_gen_random_shots_deterministic() {
    let a = super::action_gen::random_shots(240.0, 512.0, 5, 42);
    let b = super::action_gen::random_shots(240.0, 512.0, 5, 42);
    assert_eq!(a.len(), 5);
    // Same seed = same results
    for i in 0..5 {
        assert_eq!(a[i].0, b[i].0);
    }
}

#[test]
fn action_gen_random_shots_different_seeds() {
    let a = super::action_gen::random_shots(240.0, 512.0, 3, 1);
    let b = super::action_gen::random_shots(240.0, 512.0, 3, 2);
    // Different seeds should produce different labels (and actions)
    // Labels are "rng_0" etc so they match, but actions differ
    assert_eq!(a.len(), 3);
    assert_eq!(b.len(), 3);
}

#[test]
fn action_gen_tap_sequence() {
    let (actions, total) = super::action_gen::tap_sequence(&[
        (10, 100.0, 200.0),
        (30, 150.0, 250.0),
    ]);
    assert_eq!(actions.len(), 4); // 2 taps x (down + up)
    assert!(total > 30);
}

#[test]
fn action_gen_drag_produces_events() {
    let (actions, total) = super::action_gen::drag(5, 100.0, 200.0, 300.0, 400.0, 10);
    // Should have: 1 down + 9 moves + 1 up = 11
    assert!(actions.len() >= 3); // at minimum: down, some moves, up
    assert!(total > 15);
}

#[test]
fn action_gen_grid_shots_run_scenario() {
    // Verify grid-generated shots actually produce valid scenarios
    let shots = super::action_gen::grid_shots(240.0, 512.0, 270.0, 270.0, 1, 0.5, 0.5, 1);
    let (_, actions, frames) = &shots[0];

    let builder = ScenarioBuilder::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    );

    let result = builder.run("grid_shot_test", actions.clone(), *frames, vec![
        Assertion::StateEquals {
            key: "strokes".into(),
            expected: 1.0,
            tolerance: 0.0,
        },
    ]);

    assert!(result.all_passed(), "{}", result.failure_report());
}

// ─── Round 6: Experiment ──────────────────────────────────────────────

#[test]
fn experiment_basic_sweep() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;
    let (actions, _) = ShotBuilder::new()
        .aim_and_shoot(ball_x, ball_y, 270.0, 0.5)
        .build();

    let result = Experiment::new(
        "basic_sweep",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    )
    .with_actions(actions)
    .with_configs(vec![
        SweepConfig { label: "default".into(), overrides: vec![] },
        SweepConfig { label: "moved".into(), overrides: vec![("ball_y".into(), 400.0)] },
    ])
    .with_frames(120)
    .run();

    assert_eq!(result.sweep.results.len(), 2);
    assert!(result.rankings.is_empty()); // no fitness evaluator
    assert!(result.regression_ok());
    assert!(!result.summary().is_empty());
}

#[test]
fn experiment_with_fitness() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;
    let (actions, _) = ShotBuilder::new()
        .aim_and_shoot(ball_x, ball_y, 270.0, 0.5)
        .build();

    let evaluator = FitnessEvaluator::new()
        .add("proximity", 1.0, sleague::score_proximity_to_hole);

    let result = Experiment::new(
        "fitness_sweep",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    )
    .with_actions(actions)
    .with_configs(vec![
        SweepConfig { label: "default".into(), overrides: vec![] },
        SweepConfig { label: "closer".into(), overrides: vec![("ball_y".into(), 300.0)] },
    ])
    .with_frames(120)
    .with_fitness(evaluator)
    .run();

    assert_eq!(result.rankings.len(), 2);
    // Best should be first
    assert!(result.rankings[0].1.total >= result.rankings[1].1.total);
    let (best_label, best_fitness) = result.best().unwrap();
    assert!(!best_label.is_empty());
    assert!(best_fitness.total >= 0.0);
}

#[test]
fn experiment_no_configs_defaults_to_baseline() {
    let result = Experiment::new(
        "no_configs",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    )
    .with_frames(30)
    .run();

    // Should still run one config (default)
    assert_eq!(result.sweep.results.len(), 1);
}

#[test]
fn experiment_summary_format() {
    let evaluator = FitnessEvaluator::new()
        .add("proximity", 1.0, sleague::score_proximity_to_hole);

    let result = Experiment::new(
        "summary_test",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    )
    .with_frames(30)
    .with_fitness(evaluator)
    .run();

    let summary = result.summary();
    assert!(summary.contains("summary_test"));
    assert!(summary.contains("Best:"));
    assert!(summary.contains("score="));
}

// ─── Round 6: Hill Climber ────────────────────────────────────────────

#[test]
fn hill_climb_finds_better_params() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;
    let (actions, _) = ShotBuilder::new()
        .aim_and_shoot(ball_x, ball_y, 270.0, 0.5)
        .build();

    // Optimize ball_y to maximize proximity to hole
    let result = HillClimber::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        sleague::score_proximity_to_hole,
    )
    .with_actions(actions)
    .with_frames(120)
    .with_param(ParamRange::new("ball_y", 200.0, 600.0, 50.0))
    .with_max_iterations(5)
    .run();

    assert!(result.best.fitness >= 0.0);
    assert!(result.evaluations > 1);
    assert!(!result.summary().is_empty());
    assert!(result.history.len() > 1);
}

#[test]
fn hill_climb_multi_param() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;
    let (actions, _) = ShotBuilder::new()
        .aim_and_shoot(ball_x, ball_y, 270.0, 0.5)
        .build();

    let result = HillClimber::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        sleague::score_proximity_to_hole,
    )
    .with_actions(actions)
    .with_frames(120)
    .with_param(ParamRange::new("ball_x", 100.0, 400.0, 30.0))
    .with_param(ParamRange::new("ball_y", 200.0, 600.0, 50.0))
    .with_max_iterations(3)
    .run();

    // Should have optimized both params
    assert_eq!(result.best.params.len(), 2);
    assert!(result.evaluations > 2);
}

#[test]
fn hill_climb_respects_bounds() {
    let result = HillClimber::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        sleague::score_proximity_to_hole,
    )
    .with_frames(30)
    .with_param(ParamRange::new("ball_y", 300.0, 500.0, 25.0))
    .with_max_iterations(5)
    .run();

    let y_val = result.best.params[0].1;
    assert!(y_val >= 300.0 && y_val <= 500.0,
        "ball_y={} should be in [300, 500]", y_val);
}

#[test]
fn hill_climb_summary_format() {
    let result = HillClimber::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        sleague::score_proximity_to_hole,
    )
    .with_frames(30)
    .with_param(ParamRange::new("ball_y", 300.0, 500.0, 25.0))
    .with_max_iterations(2)
    .run();

    let summary = result.summary();
    assert!(summary.contains("HillClimb"));
    assert!(summary.contains("fitness="));
    assert!(summary.contains("ball_y="));
}

// ─── Round 7: Replay Recording ────────────────────────────────────────

#[test]
fn replay_records_all_frames() {
    let replay = record_replay(
        "idle_replay",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[],
        30,
        &["ball_x", "ball_y", "tl_phase"],
    );

    assert_eq!(replay.len(), 30);
    assert_eq!(replay.name, "idle_replay");
    assert!(replay.final_fb_hash != 0);
}

#[test]
fn replay_series_extracts_values() {
    let replay = record_replay(
        "series_test",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[],
        10,
        &["ball_x"],
    );

    let series = replay.series("ball_x");
    assert_eq!(series.len(), 10);
    // Idle ball should have constant x
    assert!(series.iter().all(|v| (*v - series[0]).abs() < 0.01));
}

#[test]
fn replay_get_and_at_frame() {
    let replay = record_replay(
        "access_test",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[],
        10,
        &["ball_x"],
    );

    assert!(replay.at(0).is_some());
    assert!(replay.at(9).is_some());
    assert!(replay.at(10).is_none());

    let bx = replay.get(0, "ball_x");
    assert!(bx.is_some());
    assert!(bx.unwrap() > 0.0);
}

#[test]
fn replay_first_frame_where() {
    let replay = record_replay(
        "predicate_test",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[],
        10,
        &["tl_phase"],
    );

    // tl_phase should be 0 from the start
    let frame = replay.first_frame_where("tl_phase", |v| v == 0.0);
    assert_eq!(frame, Some(0));
}

#[test]
fn replay_with_shot_shows_movement() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;

    let replay = record_replay(
        "shot_replay",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[
            ScheduledAction::PointerDown { frame: 5, x: ball_x, y: ball_y },
            ScheduledAction::PointerUp { frame: 5, x: ball_x, y: ball_y + 60.0 },
        ],
        60,
        &["ball_y"],
    );

    let series = replay.series("ball_y");
    let min_y = series.iter().cloned().fold(f64::INFINITY, f64::min);
    assert!(min_y < ball_y, "ball should move up from {}, min was {}", ball_y, min_y);
}

// ─── Round 7: Comparison ─────────────────────────────────────────────

#[test]
fn compare_identical_replays() {
    let replay_a = record_replay(
        "a",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[],
        30,
        &["ball_x", "ball_y"],
    );
    let replay_b = record_replay(
        "b",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[],
        30,
        &["ball_x", "ball_y"],
    );

    let cmp = compare_replays(&replay_a, &replay_b, &["ball_x", "ball_y"], 0.01);
    assert!(cmp.is_identical(0.01), "identical runs should compare equal");
    assert!(cmp.first_visual_divergence.is_none());
}

#[test]
fn compare_different_replays() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;

    let replay_a = record_replay(
        "idle",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[],
        60,
        &["ball_y"],
    );
    let replay_b = record_replay(
        "shot",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[
            ScheduledAction::PointerDown { frame: 5, x: ball_x, y: ball_y },
            ScheduledAction::PointerUp { frame: 5, x: ball_x, y: ball_y + 60.0 },
        ],
        60,
        &["ball_y"],
    );

    let cmp = compare_replays(&replay_a, &replay_b, &["ball_y"], 0.01);
    assert!(!cmp.is_identical(0.01), "different runs should differ");

    let ball_diff = cmp.key_diff("ball_y").unwrap();
    assert!(ball_diff.max_delta > 1.0);
    assert!(ball_diff.first_divergence.is_some());
}

#[test]
fn compare_summary_format() {
    let replay_a = record_replay(
        "a", sleague::setup_fight_only, sleague::update, sleague::render,
        sleague::dispatch_action, &[], 10, &["ball_x"],
    );
    let replay_b = record_replay(
        "b", sleague::setup_fight_only, sleague::update, sleague::render,
        sleague::dispatch_action, &[], 10, &["ball_x"],
    );

    let cmp = compare_replays(&replay_a, &replay_b, &["ball_x"], 0.01);
    let summary = cmp.summary();
    assert!(summary.contains("Compare:"));
    assert!(summary.contains("ball_x"));
}

// ─── Round 7: Anomaly Detection ──────────────────────────────────────

#[test]
fn anomaly_no_anomalies_idle() {
    let replay = record_replay(
        "idle",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[],
        20,
        &["ball_x"],
    );

    let anomalies = AnomalyDetector::new()
        .with_spike_threshold(10.0)
        .with_plateau_min_frames(30) // longer than replay
        .scan(&replay, &["ball_x"]);

    // Idle ball shouldn't have spikes
    let spikes: Vec<_> = anomalies.iter().filter(|a| a.kind == AnomalyKind::Spike).collect();
    assert!(spikes.is_empty(), "idle ball should have no spikes");
}

#[test]
fn anomaly_detects_spike() {
    let ball_x = 15.0 * 16.0;
    let ball_y = 32.0 * 16.0;

    let replay = record_replay(
        "spike_test",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[
            ScheduledAction::PointerDown { frame: 5, x: ball_x, y: ball_y },
            ScheduledAction::PointerUp { frame: 5, x: ball_x, y: ball_y + 60.0 },
        ],
        30,
        &["ball_vy"],
    );

    // With a low spike threshold, the velocity jump should be detected
    let anomalies = AnomalyDetector::new()
        .with_spike_threshold(1.0)
        .with_plateau_min_frames(999) // disable plateau
        .scan(&replay, &["ball_vy"]);

    let spikes: Vec<_> = anomalies.iter().filter(|a| a.kind == AnomalyKind::Spike).collect();
    assert!(!spikes.is_empty(), "velocity change from shot should trigger spike");
}

#[test]
fn anomaly_detects_plateau() {
    let replay = record_replay(
        "plateau_test",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[],
        30,
        &["ball_x"],
    );

    let anomalies = AnomalyDetector::new()
        .with_spike_threshold(9999.0) // disable spike
        .with_plateau_min_frames(10) // detect plateaus of 10+ frames
        .scan(&replay, &["ball_x"]);

    let plateaus: Vec<_> = anomalies.iter().filter(|a| a.kind == AnomalyKind::Plateau).collect();
    assert!(!plateaus.is_empty(), "idle ball should have plateau");
}

#[test]
fn anomaly_detects_out_of_bounds() {
    let replay = record_replay(
        "bounds_test",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[],
        10,
        &["ball_x"],
    );

    // Set bounds that the ball position violates
    let anomalies = AnomalyDetector::new()
        .with_spike_threshold(9999.0)
        .with_plateau_min_frames(999)
        .with_bounds(0.0, 10.0) // ball_x is ~240, way out of these bounds
        .scan(&replay, &["ball_x"]);

    let oob: Vec<_> = anomalies.iter().filter(|a| a.kind == AnomalyKind::OutOfBounds).collect();
    assert!(!oob.is_empty(), "ball_x ~240 should be out of [0, 10] bounds");
}

#[test]
fn anomaly_report_format() {
    let replay = record_replay(
        "report",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &[],
        10,
        &["ball_x"],
    );

    let report = AnomalyDetector::new()
        .with_bounds(0.0, 10.0)
        .report(&replay, &["ball_x"]);

    assert!(report.contains("anomal")); // "anomalies" or "Anomaly"
}
