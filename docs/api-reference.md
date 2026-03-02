# Crusty Engine: Headless API Reference

Complete API reference for all 18 headless testing modules. Every type, function, and method available for AI-driven game testing and optimization.

## Core Layer

### `HeadlessRunner`

The foundation. Wraps an `Engine` instance and drives the game loop without a browser.

```rust
// Create with viewport dimensions (default for S-League: 480x720)
let mut runner = HeadlessRunner::new(480, 720);

// Run with setup/update/render callbacks
let result = runner.run(setup_fn, update_fn, render_fn, 120);

// Run with per-frame callback (receives frame number and dt)
let result = runner.run_with_frame_cb(
    |engine| { /* setup */ },
    |engine, frame, dt| { /* per-frame logic */ },
    120,
);
```

**Fields:**
- `engine: Engine` — direct access to the underlying engine

### `SimResult`

Universal output of every simulation run.

```rust
pub struct SimResult {
    pub frames_run: u64,
    pub final_metrics: FrameMetrics,
    pub game_state: HashMap<String, StateValue>,
    pub framebuffer_hash: u64,
    pub elapsed_sim_time: f64,
}
```

**Methods:**
- `get_f64(key: &str) -> Option<f64>` — convenience accessor for numeric state values

**Key insight**: `framebuffer_hash` is an FNV-1a hash of all rendered pixels. Identical rendering always produces identical hashes. Use this for visual regression detection.

### `ScheduledAction`

Timed input events injected during simulation.

```rust
pub enum ScheduledAction {
    PointerDown { frame: u64, x: f64, y: f64 },
    PointerMove { frame: u64, x: f64, y: f64 },
    PointerUp   { frame: u64, x: f64, y: f64 },
}
```

**Methods:**
- `frame() -> u64` — get the frame number
- `coords() -> (f64, f64)` — get the (x, y) coordinates

### `GameScenario`

Declarative test: setup + actions + frame count + assertions = structured result.

```rust
let scenario = GameScenario {
    name: "my test".into(),
    width: 480, height: 720,
    setup_fn, update_fn, render_fn, action_dispatch,
    actions: vec![...],
    total_frames: 120,
    assertions: vec![
        Assertion::StateEquals { key: "score".into(), expected: 1.0, tolerance: 0.1 },
        Assertion::StateInRange { key: "ball_x".into(), min: 0.0, max: 480.0 },
    ],
};
let result = scenario.run();
assert!(result.all_passed(), "{}", result.failure_report());
```

### `ScenarioBuilder`

Reduces boilerplate when running multiple scenarios with the same game functions.

```rust
let builder = ScenarioBuilder::new(setup_fn, update_fn, render_fn, dispatch_fn);

// Build and run in one call
let result = builder.run("test name", actions, 120, assertions);

// Or build a GameScenario for later use
let scenario = builder.build("name", actions, 120, assertions);

// Run an idle baseline (no input, no assertions)
let baseline = builder.run_idle("baseline", 60);
```

### `Assertion`

Available assertion types:

| Variant | Description |
|---------|------------|
| `StateEquals { key, expected, tolerance }` | f64 value equals expected (±tolerance) |
| `StateInRange { key, min, max }` | f64 value within [min, max] |
| `StateGreaterThan { key, threshold }` | f64 value > threshold |
| `StateLessThan { key, threshold }` | f64 value < threshold |
| `FramebufferHash { expected }` | Exact framebuffer hash match |
| `FramebufferChanged { previous }` | Framebuffer hash differs from previous |

### `ShotBuilder`

High-level builder for slingshot-style input sequences.

```rust
let (actions, total_frames) = ShotBuilder::new()
    .with_drag_scale(120.0)      // pixels per power=1.0
    .with_settle_frames(180)     // 3 seconds settle time
    .aim_and_shoot(240.0, 500.0, 270.0, 0.8) // origin, angle, power
    .aim_and_shoot(240.0, 500.0, 45.0, 0.5)  // second shot
    .wait(60)
    .build();
```

**Angle convention**: 0=right, 90=down, 180=left, 270=up. Slingshot drag is computed as the opposite direction.

### `framebuffer_hash(fb: &Framebuffer) -> u64`

FNV-1a hash of framebuffer pixel data. Deterministic for identical content.

---

## Analysis Layer

### `run_sweep()`

Run the same scenario with different parameter configurations.

```rust
let configs = vec![
    SweepConfig { label: "low_drag".into(), overrides: vec![("drag".into(), 1.0)] },
    SweepConfig { label: "high_drag".into(), overrides: vec![("drag".into(), 3.0)] },
];

let report: SweepReport = run_sweep(
    setup_fn, update_fn, render_fn, dispatch_fn,
    &actions, &configs, 120,
);

// Query results
let best = report.min_by_state("dist_to_hole");
let worst = report.max_by_state("dist_to_hole");
println!("{}", report.summary());
println!("{}", report.summary_with_keys(&["dist_to_hole", "ball_x"]));
```

### `StateTimeline`

Per-frame state recording for trajectory and value analysis.

```rust
// Without actions
let timeline = record_timeline(setup_fn, update_fn, render_fn, 120, &["ball_x", "ball_y"]);

// With actions
let timeline = record_timeline_with_actions(
    setup_fn, update_fn, render_fn, dispatch_fn, &actions, 120,
    &["ball_x", "ball_y"],
);

// Query
let x_series: Vec<f64> = timeline.series("ball_x").unwrap();
let (min, max, mean) = timeline.stats("ball_x").unwrap();
let landing = timeline.first_frame_where("ball_vy", |vy| vy >= 0.0);
assert!(!timeline.is_empty());
```

### `FitnessEvaluator`

Composable weighted scoring system.

```rust
let evaluator = FitnessEvaluator::new()
    .add("proximity", 3.0, |sim| {
        score_distance("ball_x", "ball_y", "hole_x", "hole_y", 300.0, sim)
    })
    .add("efficiency", 1.0, |sim| {
        score_ratio("stroke_count", 1.0, sim)
    })
    .add("completion", 2.0, |sim| {
        score_state_match("in_hole", 1.0, 0.5, sim)
    });

let fitness: FitnessResult = evaluator.evaluate(&sim_result);
println!("Grade: {} Score: {:.3}", fitness.grade(), fitness.total);
println!("{}", fitness.summary());

// Rank an entire sweep
let ranked: Vec<(String, FitnessResult)> = evaluator.rank_sweep(&sweep_report);
```

**Scoring helpers:**
- `score_distance(x1, y1, x2, y2, max_dist, sim)` — 1.0 at distance 0, 0.0 at max_dist
- `score_state_match(key, target, tolerance, sim)` — 1.0 if key equals target
- `score_ratio(key, target, sim)` — target/actual, capped at 1.0

**Grade scale:** A+ (≥0.95), A (≥0.85), B (≥0.70), C (≥0.50), D (≥0.30), F (<0.30)

### `RegressionSuite`

Capture baselines and diff against future runs.

```rust
let suite = RegressionSuite::new(&["ball_x", "ball_y", "stroke_count"])
    .with_tolerance(0.5)
    .with_classifier(|key, delta| {
        classify_lower_is_better(&["stroke_count", "dist"], key, delta)
    })
    .add(scenario_a)
    .add(scenario_b);

// Capture
let baselines: Vec<RegressionBaseline> = suite.capture_baseline();

// ... make changes ...

// Diff
let diff: DiffReport = suite.diff_against(&baselines);
println!("{}", diff.summary());
assert!(!diff.has_regressions(), "Regressions found:\n{}", diff.summary());
```

**Built-in classifiers:**
- `classify_any_change(key, delta)` — marks all deltas as Changed (default)
- `classify_lower_is_better(lower_keys, key, delta)` — decrease = Improved, increase = Regressed

---

## Snapshot & Replay Layer

### `run_with_snapshots()`

Capture full game state at specific frames. Best for debugging specific moments.

```rust
let result: SnapshotResult = run_with_snapshots(
    setup_fn, update_fn, render_fn, dispatch_fn,
    &actions, 120, &[0, 30, 60, 90, 119],
);

let snap = result.at_frame(30).unwrap();
let ball_x = snap.get_f64("ball_x").unwrap();
let mode = snap.get_str("game_mode");

// Value change detection
let changed = result.value_changed("ball_x", 0, 60).unwrap();

// Extract series across snapshot frames
let trajectory: Vec<(u64, f64)> = result.series("ball_x");
```

### `Replay` and `record_replay()`

Lightweight per-frame recording of specific keys. More efficient than snapshots for full-run analysis.

```rust
let replay: Replay = record_replay(
    "baseline",
    setup_fn, update_fn, render_fn, dispatch_fn,
    &actions, 120, &["ball_x", "ball_y", "ball_vx"],
);

// Query
let x_values: Vec<f64> = replay.series("ball_x");
let frame_30 = replay.at(30).unwrap();
let val = replay.get(30, "ball_x").unwrap();
let landing = replay.first_frame_where("ball_vy", |vy| vy >= 0.0);
assert!(!replay.is_empty());
```

### `compare_replays()`

Structural diff of two replays.

```rust
let cmp: Comparison = compare_replays(&replay_a, &replay_b, &["ball_x", "ball_y"], 0.01);

// Overall check
assert!(cmp.is_identical(0.01), "Replays diverged");

// Per-key analysis
let diff = cmp.key_diff("ball_x").unwrap();
println!("max_delta: {} at frame {}", diff.max_delta, diff.max_delta_frame);
println!("mean_delta: {}", diff.mean_delta);
if let Some(frame) = diff.first_divergence {
    println!("First divergence at frame {}", frame);
}

// Visual divergence
if let Some(frame) = cmp.first_visual_divergence {
    println!("Framebuffers first differ at frame {}", frame);
}

println!("{}", cmp.summary());
```

### `AnomalyDetector`

Scans replay data for three anomaly types.

```rust
let detector = AnomalyDetector::new()
    .with_spike_threshold(50.0)       // max frame-to-frame delta
    .with_plateau_min_frames(20)      // min frames for plateau
    .with_bounds(0.0, 720.0);         // expected value range

let anomalies: Vec<Anomaly> = detector.scan(&replay, &["ball_x", "ball_y"]);

for a in &anomalies {
    match a.kind {
        AnomalyKind::Spike => println!("Spike at frame {}: {}", a.frame, a.detail),
        AnomalyKind::Plateau => println!("Plateau at frame {}: {}", a.frame, a.detail),
        AnomalyKind::OutOfBounds => println!("OOB at frame {}: {}", a.frame, a.detail),
    }
}

println!("{}", detector.report(&replay, &["ball_x", "ball_y"]));
```

---

## Optimization Layer

### `Experiment`

Combines sweep + fitness + optional regression into a single call.

```rust
let result: ExperimentResult = Experiment::new(
    "drag optimization",
    setup_fn, update_fn, render_fn, dispatch_fn,
)
    .with_actions(actions)
    .with_configs(configs)
    .with_frames(120)
    .with_fitness(evaluator)
    .with_baseline(baseline, &["ball_x", "ball_y"])
    .run();

// Inspect results
let (label, fitness) = result.best().unwrap();
println!("Best: {} ({})", label, fitness.grade());
assert!(result.regression_ok());
println!("{}", result.summary());
```

### `HillClimber`

Coordinate-descent optimizer for game parameters.

```rust
let result: ClimbResult = HillClimber::new(
    setup_fn, update_fn, render_fn, dispatch_fn, fitness_fn,
)
    .with_actions(actions)
    .with_frames(120)
    .with_param(ParamRange::new("drag", 0.5, 5.0, 0.5))
    .with_param(ParamRange::new("restitution", 0.1, 0.9, 0.1))
    .with_max_iterations(20)
    .with_min_step(0.01)
    .run();

// Results
println!("{}", result.summary());
for (key, value) in &result.best.params {
    println!("{} = {:.4}", key, value);
}
println!("Fitness: {:.4}", result.best.fitness);
println!("Improved: {}", result.improved());
println!("Evaluations: {}", result.evaluations);
```

**Algorithm**: Starts at parameter midpoints. Each iteration tries +step and -step for each parameter, keeping improvements. Step size halves when no improvement is found. Stops when all steps < min_step or max_iterations reached.

### Action Generators (`action_gen`)

Programmatic input sequence generation.

```rust
// Systematic grid search
let shots: Vec<(String, Vec<ScheduledAction>, u64)> = action_gen::grid_shots(
    240.0, 500.0,           // origin
    200.0, 340.0, 8,        // angle: min, max, steps
    0.3, 1.0, 5,            // power: min, max, steps
);

// Deterministic random exploration
let shots = action_gen::random_shots(240.0, 500.0, 20, 42);

// UI interaction testing
let (actions, frames) = action_gen::tap_sequence(&[
    (10, 240.0, 360.0),  // (frame, x, y)
    (30, 100.0, 600.0),
]);

// Drag gestures
let (actions, frames) = action_gen::drag(
    5,                    // start frame
    100.0, 500.0,         // from
    300.0, 200.0,         // to
    15,                   // drag duration in frames
);
```

---

## Orchestration Layer

### `Strategy`

Multi-step playbooks for complex analysis workflows.

```rust
let result: StrategyResult = Strategy::new(
    "physics validation",
    setup_fn, update_fn, render_fn, dispatch_fn,
)
    .record("baseline", vec![], 120, &["ball_x", "ball_y"])
    .record("with_shot", actions.clone(), 120, &["ball_x", "ball_y"])
    .compare("baseline", "with_shot", &["ball_x", "ball_y"], 0.01)
    .detect_anomalies("with_shot", &["ball_x", "ball_y"], 50.0, 30)
    .assert_state("with_shot", "stroke_count", StatePredicate::GreaterThan(0.0))
    .assert_state("with_shot", "ball_x", StatePredicate::InRange(0.0, 480.0))
    .run();

assert!(result.all_passed(), "{}", result.summary());
```

**Available predicates:**
- `StatePredicate::GreaterThan(f64)`
- `StatePredicate::LessThan(f64)`
- `StatePredicate::Equals(expected, tolerance)`
- `StatePredicate::InRange(min, max)`

### `TestHarness`

Battery testing across multiple scenarios.

```rust
let report: HarnessReport = TestHarness::new(
    setup_fn, update_fn, render_fn, dispatch_fn,
)
    .add("idle test", vec![], 60, vec![
        Assertion::StateEquals { key: "stroke_count".into(), expected: 0.0, tolerance: 0.1 },
    ])
    .add_with_fitness("shot quality", actions, 120, vec![], evaluator)
    .run();

println!("{}", report.summary());
assert!(report.all_passed());
println!("Average fitness: {:?}", report.avg_fitness());
```

### `GoldenTest`

Reference replay comparison — the headless equivalent of visual regression testing.

```rust
let test = GoldenTest::new(setup_fn, update_fn, render_fn, dispatch_fn)
    .with_actions(actions)
    .with_frames(120)
    .with_keys(&["ball_x", "ball_y", "stroke_count"])
    .with_tolerance(0.01);

// Step 1: Record the golden reference (once, when behavior is correct)
let golden: Replay = test.record_golden("reference_v1");

// Step 2: After changes, compare against golden
let result: GoldenResult = test.compare_against(&golden);
assert!(result.matches, "{}", result.summary());
```

---

## Game Integration Contract

Every headless module is game-agnostic. Games integrate by providing 4 function pointers:

```rust
fn setup(engine: &mut Engine);                           // Initialize state
fn update(engine: &mut Engine, dt: f64);                 // Game logic per frame
fn render(engine: &mut Engine);                          // Draw to framebuffer
fn action_dispatch(engine: &mut Engine, action: &ScheduledAction); // Route inputs
```

The S-League demo provides:
- `sleague::setup_fight_only` / `sleague::setup`
- `sleague::update`
- `sleague::render`
- `sleague::dispatch_action`

Game-specific scoring functions (like `score_hole_completion`) live in the game module, not the engine.
