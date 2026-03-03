# Crusty Engine: API Reference

API reference for the core simulation infrastructure and headless testing modules.

## Core Types

### `Simulation` Trait

The contract between a game and the engine.

```rust
pub trait Simulation {
    fn setup(&mut self, engine: &mut Engine);
    fn step(&mut self, engine: &mut Engine);
    fn render(&self, engine: &mut Engine);
    fn variants(&self) -> Vec<ParamSet> { vec![] }
}
```

### `InputFrame`

Canonical input representation. Serializable for replays.

```rust
pub struct InputFrame {
    pub keys_pressed: Vec<String>,
    pub keys_released: Vec<String>,
    pub keys_held: Vec<String>,
    pub pointer: Option<(f64, f64)>,
    pub pointer_down: Option<(f64, f64)>,
    pub pointer_up: Option<(f64, f64)>,
}
```

**Methods:** `is_empty() -> bool`

### `Observation`

Zero-allocation view into engine state.

```rust
pub struct Observation<'a> {
    pub frame: u64,
    pub state_hash: u64,
    pub game_state: &'a GameState,
    pub entity_count: usize,
    pub framebuffer: Option<&'a [u8]>,
}
```

### `Policy` Trait

Pluggable input generator for automated simulation.

```rust
pub trait Policy {
    fn next_input(&mut self, obs: &Observation) -> InputFrame;
}
```

**Built-in implementations:**
- `NullPolicy` — empty InputFrame every tick
- `RandomPolicy::new(seed)` — random events from a separate seeded RNG
- `ScriptedPolicy::new(inputs)` — replay a fixed `Vec<InputFrame>`

---

## Headless Runner

### `HeadlessRunner`

The foundation. Wraps an `Engine` instance and drives simulations.

```rust
let mut runner = HeadlessRunner::new(480, 720);

// Run with fixed inputs
let result = runner.run_sim(&mut sim, seed, &inputs, config);

// Run for N frames with fixed inputs
let result = runner.run_sim_frames(&mut sim, seed, &inputs, frames, config);

// Run with a policy
let result = runner.run_with_policy(&mut sim, &mut policy, seed, frames, config);
```

### `RunConfig`

```rust
pub struct RunConfig {
    pub turbo: bool,              // Skip rendering
    pub capture_state_hashes: bool, // Per-frame hashes
}
```

### `SimResult`

Universal output of every simulation run.

```rust
pub struct SimResult {
    pub frames_run: u64,
    pub final_metrics: FrameMetrics,
    pub game_state: HashMap<String, StateValue>,
    pub framebuffer_hash: u64,
    pub elapsed_sim_time: f64,
    pub state_hash: u64,
    pub state_hashes: Vec<u64>,
}
```

**Methods:** `get_f64(key: &str) -> Option<f64>`

### `PlaythroughFile`

Serializable replay format.

```rust
pub struct PlaythroughFile {
    pub engine_version: String,
    pub seed: u64,
    pub inputs: Vec<InputFrame>,
    pub frame_count: u64,
    pub final_state_hash: u64,
    pub final_fb_hash: u64,
    pub state_hashes: Vec<u64>,
    pub metadata: HashMap<String, String>,
}
```

---

## Analysis Layer

### `FitnessEvaluator`

Composable weighted scoring system.

```rust
let evaluator = FitnessEvaluator::new()
    .add("proximity", 3.0, |sim| { /* -> f64 in [0, 1] */ })
    .add("efficiency", 1.0, |sim| { /* -> f64 in [0, 1] */ });

let fitness: FitnessResult = evaluator.evaluate(&sim_result);
// fitness.total, fitness.grade(), fitness.summary()
```

**Scoring helpers:**
- `score_distance(x1, y1, x2, y2, max_dist, sim)` — 1.0 at distance 0
- `score_state_match(key, target, tolerance, sim)` — 1.0 if key equals target
- `score_ratio(key, target, sim)` — target/actual, capped at 1.0

**Grade scale:** A+ (>=0.95), A (>=0.85), B (>=0.70), C (>=0.50), D (>=0.30), F (<0.30)

### `RegressionSuite`

Capture baselines and diff against future runs.

```rust
let suite = RegressionSuite::new(&["score", "game_length"])
    .with_tolerance(0.5)
    .with_classifier(classify_lower_is_better);

let baselines = suite.capture_baseline();
// ... make changes ...
let diff = suite.diff_against(&baselines);
assert!(!diff.has_regressions());
```

### `AnomalyDetector`

Scans replay data for anomalies.

```rust
let detector = AnomalyDetector::new()
    .with_spike_threshold(50.0)
    .with_plateau_min_frames(20)
    .with_bounds(0.0, 720.0);

let anomalies = detector.scan(&replay, &["ball_x", "ball_y"]);
```

**Anomaly types:** Spike, Plateau, OutOfBounds

---

## Optimization Layer

### `HillClimber`

Coordinate-descent optimizer for game parameters.

```rust
let result = HillClimber::new(setup_fn, update_fn, render_fn, dispatch_fn, fitness_fn)
    .with_param(ParamRange::new("gravity", 200.0, 800.0, 50.0))
    .with_max_iterations(20)
    .run();

println!("{}", result.summary());
println!("Best fitness: {:.4}", result.best.fitness);
```

### Action Generators (`action_gen`)

Programmatic input sequence generation.

```rust
// Systematic grid search
let shots = action_gen::grid_shots(origin_x, origin_y, angle_min, angle_max, angle_steps, power_min, power_max, power_steps);

// Deterministic random exploration
let shots = action_gen::random_shots(x, y, count, seed);

// UI interaction testing
let (actions, frames) = action_gen::tap_sequence(&[(frame, x, y), ...]);

// Drag gestures
let (actions, frames) = action_gen::drag(start_frame, from_x, from_y, to_x, to_y, duration);
```

---

## Orchestration Layer

### `GoldenTest`

Reference replay comparison.

```rust
let test = GoldenTest::new(...)
    .with_keys(&["score", "ball_x"])
    .with_tolerance(0.01);

let golden = test.record_golden("reference");
let result = test.compare_against(&golden);
assert!(result.matches, "{}", result.summary());
```

### `TestHarness`

Battery testing across multiple scenarios.

```rust
let report = TestHarness::new(...)
    .add("idle test", vec![], 60, assertions)
    .add_with_fitness("quality", actions, 120, assertions, evaluator)
    .run();

assert!(report.all_passed());
```

---

## Design Acceleration Layer

### Death Classification

```rust
use crate::headless::death_classify::{classify, ClassifierConfig, DeathClass};
use crate::headless::death_report::classify_batch;

let report = classify_batch(results, config);
// report.breakdown: HashMap<DeathClass, usize>
// DeathClass: CloseCall, Blowout, Cliff, Attrition, Unclassified
```

### Divergence Replay

```rust
use crate::headless::divergence::compare_hash_sequences;

let report = compare_hash_sequences(&hashes_a, &hashes_b);
// report.divergence_frame, report.total_divergent
```

### Feel Presets

```rust
use crate::feel_preset::FeelPresetLibrary;

let lib = FeelPresetLibrary::built_in();
let preset = lib.get("tight_platformer").unwrap();
preset.apply(&mut engine.global_state);
```

**Built-in presets:** tight_platformer, floaty_astronaut, heavy_tank, snappy_cursor, underwater, ice_skating

### Variant Branching

```rust
use crate::headless::variant_runner::sweep_variants;

// Games declare variants via Simulation::variants()
let report = sweep_variants(sim_factory, seeds, frames, config);
```

### Interesting Moments

```rust
use crate::headless::highlights::HighlightScanner;

let report = scanner.scan(sim_factory, seeds, frames, config);
// report.highlights: Vec<Highlight> with kind, seed, frame, score
```

### Mechanic Ablation

```rust
use crate::headless::ablation::run_ablation_study;

let report = run_ablation_study(sim_factory, ablations, seeds, frames, metrics, config);
// report.ranked(): Vec sorted by impact
```

### Dashboard

```rust
use crate::headless::dashboard::generate_dashboard_data;

let data = generate_dashboard_data(sim_factory, config);
// Writes dashboard.json for the static HTML frontend
```

---

## CLI Command Reference

| Command | Purpose |
|---------|---------|
| `record` | Record a playthrough to JSON |
| `replay` | Replay and optionally verify determinism |
| `batch` | Run seed range with no input |
| `sweep` | Policy-driven seed sweep |
| `golden record/check` | Golden baseline testing |
| `deaths` | Death classification report |
| `divergence files/sweep` | Compare playthroughs or sweeps |
| `preset list/show/apply` | Physics feel presets |
| `variants` | List declared game variants |
| `variant-sweep` | Sweep across variants and seeds |
| `highlights` | Interesting moment detection |
| `ablation` | Mechanic ablation study |
| `dashboard-data` | Generate dashboard JSON |
| `info` | Print engine info |
| `schema` | Generate JSON schema |
