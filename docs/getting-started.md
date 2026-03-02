# Getting Started with Headless Testing

A quick-start guide for using the Crusty Engine's headless testing infrastructure. From zero to running your first AI-driven analysis in minutes.

## Prerequisites

```bash
cd /home/user/crusty/engine
cargo test  # Verify everything compiles and passes
```

The headless modules live in `engine/crates/engine-core/src/headless/`. Tests are in `headless/tests.rs`.

## Your First Headless Test

The simplest test: run the game idle for 60 frames and check that it initializes correctly.

```rust
use crate::headless::*;
use crate::sleague;

#[test]
fn game_initializes_correctly() {
    let result = ScenarioBuilder::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    )
    .run(
        "init check",
        vec![],  // no input
        60,      // 1 second at 60fps
        vec![
            Assertion::StateEquals {
                key: "stroke_count".into(),
                expected: 0.0,
                tolerance: 0.1,
            },
        ],
    );

    assert!(result.all_passed(), "{}", result.failure_report());
}
```

## Simulating Player Input

Use `ShotBuilder` to construct slingshot-style drag-and-release inputs:

```rust
#[test]
fn shot_moves_ball() {
    let (actions, frames) = ShotBuilder::new()
        .aim_and_shoot(240.0, 500.0, 270.0, 0.8) // aim up, 80% power
        .build();

    let result = ScenarioBuilder::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    )
    .run(
        "shot moves ball",
        actions,
        frames,
        vec![
            Assertion::StateGreaterThan {
                key: "stroke_count".into(),
                threshold: 0.0,
            },
        ],
    );

    assert!(result.all_passed());
}
```

## Recording and Analyzing Trajectories

Use `record_timeline` to capture per-frame state:

```rust
#[test]
fn ball_trajectory_analysis() {
    let (actions, frames) = ShotBuilder::new()
        .aim_and_shoot(240.0, 500.0, 270.0, 0.8)
        .build();

    let timeline = record_timeline_with_actions(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &actions,
        frames,
        &["ball_x", "ball_y"],
    );

    // Analyze the trajectory
    let (min_y, max_y, _mean_y) = timeline.stats("ball_y").unwrap();
    assert!(min_y < max_y, "Ball should move vertically");

    // Find when ball starts falling
    let peak = timeline.first_frame_where("ball_y", |y| y < 300.0);
    println!("Ball reached y < 300 at frame {:?}", peak);
}
```

## Detecting Physics Anomalies

```rust
#[test]
fn no_physics_anomalies() {
    let (actions, frames) = ShotBuilder::new()
        .aim_and_shoot(240.0, 500.0, 270.0, 1.0)
        .build();

    let replay = record_replay(
        "physics_check",
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        &actions,
        frames,
        &["ball_x", "ball_y"],
    );

    let anomalies = AnomalyDetector::new()
        .with_spike_threshold(50.0)  // flag jumps > 50px
        .with_bounds(0.0, 720.0)     // ball should stay on screen
        .scan(&replay, &["ball_x", "ball_y"]);

    let spikes: Vec<_> = anomalies.iter()
        .filter(|a| a.kind == AnomalyKind::Spike)
        .collect();
    assert!(spikes.is_empty(), "Physics spikes detected: {:?}", spikes);
}
```

## Parameter Optimization

Find optimal game parameters with a hill climber:

```rust
#[test]
fn optimize_drag_coefficient() {
    let (actions, _) = ShotBuilder::new()
        .aim_and_shoot(240.0, 500.0, 270.0, 0.8)
        .build();

    let result = HillClimber::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
        |sim| {
            // Fitness: closer to hole = better
            let dist = sim.get_f64("dist_to_hole").unwrap_or(999.0);
            (1.0 - dist / 500.0).max(0.0)
        },
    )
        .with_actions(actions)
        .with_frames(300)
        .with_param(ParamRange::new("drag", 0.5, 5.0, 0.5))
        .with_max_iterations(10)
        .run();

    println!("{}", result.summary());
    assert!(result.best.fitness > 0.0, "Should find a non-zero fitness");
}
```

## Quality Gate: Test Harness

Run a battery of tests in one call:

```rust
#[test]
fn quality_gate() {
    let report = TestHarness::new(
        sleague::setup_fight_only,
        sleague::update,
        sleague::render,
        sleague::dispatch_action,
    )
    .add("idle_stable", vec![], 60, vec![
        Assertion::StateEquals { key: "stroke_count".into(), expected: 0.0, tolerance: 0.1 },
    ])
    .add("shot_registers", {
        let (a, _) = ShotBuilder::new().aim_and_shoot(240.0, 500.0, 270.0, 0.8).build();
        a
    }, 300, vec![
        Assertion::StateGreaterThan { key: "stroke_count".into(), threshold: 0.0 },
    ])
    .run();

    println!("{}", report.summary());
    assert!(report.all_passed());
}
```

## Module Cheat Sheet

| Task | Module | One-liner |
|------|--------|-----------|
| Run N frames | `HeadlessRunner` | `runner.run(setup, update, render, 60)` |
| Declarative test | `GameScenario` | `scenario.run()` |
| Multiple tests | `ScenarioBuilder` | `builder.run(name, actions, frames, asserts)` |
| Slingshot input | `ShotBuilder` | `ShotBuilder::new().aim_and_shoot(...)` |
| Parameter sweep | `run_sweep` | `run_sweep(s, u, r, d, &acts, &cfgs, 120)` |
| Trajectory data | `StateTimeline` | `record_timeline(s, u, r, 120, &keys)` |
| Quality scoring | `FitnessEvaluator` | `evaluator.evaluate(&sim)` |
| Regression check | `RegressionSuite` | `suite.diff_against(&baselines)` |
| Snapshot frames | `run_with_snapshots` | `run_with_snapshots(s, u, r, d, &a, 120, &[30,60])` |
| Full replay | `record_replay` | `record_replay(name, s, u, r, d, &a, 120, &keys)` |
| Compare replays | `compare_replays` | `compare_replays(&a, &b, &keys, 0.01)` |
| Anomaly scan | `AnomalyDetector` | `detector.scan(&replay, &keys)` |
| Sweep + fitness | `Experiment` | `experiment.run()` |
| Hill climbing | `HillClimber` | `climber.run()` |
| Generate shots | `action_gen` | `grid_shots(...)`, `random_shots(...)` |
| Multi-step flow | `Strategy` | `strategy.record().compare().assert_state().run()` |
| Battery test | `TestHarness` | `harness.add(...).run()` |
| Golden file | `GoldenTest` | `test.record_golden()` / `test.compare_against()` |
