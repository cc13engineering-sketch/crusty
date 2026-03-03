# Getting Started with Crusty Engine

From zero to running your first simulation and headless analysis.

## Prerequisites

```bash
cd /home/user/crusty/engine
cargo test  # Verify everything compiles and passes
```

The engine lives in `engine/crates/engine-core/`. The CLI is in `engine/crates/engine-cli/`.

## Writing a Simulation

Games implement the `Simulation` trait:

```rust
use crate::engine::Engine;
use crate::simulation::Simulation;

pub struct MyGame {
    score: f64,
}

impl Simulation for MyGame {
    fn setup(&mut self, engine: &mut Engine) {
        engine.reset(42);
        // Create entities, set initial state
        engine.global_state.set_f64("score", 0.0);
    }

    fn step(&mut self, engine: &mut Engine) {
        // Game logic runs every fixed tick (1/60s)
        // Read input from engine.input
        // Update entities via engine.world
    }

    fn render(&self, engine: &mut Engine) {
        // Draw to engine.framebuffer
    }
}
```

The built-in `DemoBall` (`demo_ball.rs`) is a minimal reference implementation.

## Running Headless

Use `HeadlessRunner` to drive simulations without a browser:

```rust
use crate::headless::runner::{HeadlessRunner, RunConfig};
use crate::demo_ball::DemoBall;

let mut runner = HeadlessRunner::new(480, 720);
let mut sim = DemoBall::new();
let config = RunConfig { turbo: true, capture_state_hashes: true };

// Run with no input for 120 frames
let result = runner.run_sim(&mut sim, 42, &[], config);
println!("Frames: {}, State hash: {}", result.frames_run, result.state_hash);
```

## Using Policies

Policies generate input automatically:

```rust
use crate::policy::{RandomPolicy, NullPolicy};

// Random input exploration
let mut policy = RandomPolicy::new(123);
let result = runner.run_with_policy(&mut sim, &mut policy, 42, 600, config);

// No input (idle baseline)
let mut null = NullPolicy;
let result = runner.run_with_policy(&mut sim, &mut null, 42, 60, config);
```

## CLI Commands

The `engine-cli` tool exposes all headless capabilities:

```bash
# Record a playthrough
cargo run -p engine-cli -- record --seed 42 --frames 600 --out play.json

# Replay and verify determinism
cargo run -p engine-cli -- replay play.json --verify

# Batch run across seeds
cargo run -p engine-cli -- batch --seed-range 0..100 --frames 600 --turbo

# Policy-driven sweep
cargo run -p engine-cli -- sweep --policy random --seed-range 0..1000 --frames 600 --turbo

# Golden test
cargo run -p engine-cli -- golden record --seed 42 --frames 600 --out golden.json
cargo run -p engine-cli -- golden check golden.json

# Death classification
cargo run -p engine-cli -- deaths --seed-range 0..100 --frames 600 --metric score

# Interesting moments
cargo run -p engine-cli -- highlights --seed-range 0..100 --frames 600

# Mechanic ablation
cargo run -p engine-cli -- ablation --seeds 50 --frames 600

# Dashboard data
cargo run -p engine-cli -- dashboard-data --seeds 100 --frames 600

# Engine info
cargo run -p engine-cli -- info
```

## Determinism Verification

Every simulation with the same seed + inputs produces identical state hashes:

```rust
let r1 = runner.run_sim(&mut DemoBall::new(), 42, &inputs, config);
let r2 = runner.run_sim(&mut DemoBall::new(), 42, &inputs, config);
assert_eq!(r1.state_hash, r2.state_hash);
assert_eq!(r1.state_hashes, r2.state_hashes);
```

## Module Cheat Sheet

| Task | Module | One-liner |
|------|--------|-----------|
| Run N frames | `HeadlessRunner` | `runner.run_sim(&mut sim, seed, &inputs, config)` |
| Policy-driven run | `HeadlessRunner` | `runner.run_with_policy(&mut sim, &mut policy, seed, frames, config)` |
| Parameter sweep | `run_sweep` | `run_sweep(configs, ...)` |
| Quality scoring | `FitnessEvaluator` | `evaluator.evaluate(&sim_result)` |
| Regression check | `RegressionSuite` | `suite.diff_against(&baselines)` |
| Full replay | `record_replay` | `record_replay(...)` |
| Compare replays | `compare_replays` | `compare_replays(&a, &b, &keys, tolerance)` |
| Anomaly scan | `AnomalyDetector` | `detector.scan(&replay, &keys)` |
| Hill climbing | `HillClimber` | `climber.run()` |
| Golden file | `GoldenTest` | `test.record_golden() / test.compare_against()` |
| Death classification | `classify_batch` | `classify_batch(results, config)` |
| Interesting moments | `HighlightScanner` | `scanner.scan(...)` |
| Mechanic ablation | `run_ablation_study` | `run_ablation_study(...)` |
| Variant sweep | `sweep_variants` | `sweep_variants(...)` |
