# Crusty Engine: Headless Testing Architecture

## Overview

The Crusty Engine headless testing system is a 22-module infrastructure that enables AI agents to autonomously run, analyze, optimize, and improve games — all without a browser, display, or human intervention. It runs entirely via `cargo test` and the `engine-cli` tool.

This document covers the technical architecture: how the modules compose, what data flows between them, and how to extend the system for new games.

## Core Abstractions

### Simulation Trait

The contract between a game and the engine. Games implement `setup`, `step`, and `render`:

```rust
pub trait Simulation {
    fn setup(&mut self, engine: &mut Engine);
    fn step(&mut self, engine: &mut Engine);
    fn render(&self, engine: &mut Engine);
    fn variants(&self) -> Vec<ParamSet> { vec![] }
}
```

### InputFrame

The canonical representation of one frame of player input:

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

### Policy Trait

A pluggable input generator. Takes an `Observation`, produces an `InputFrame`:

```rust
pub trait Policy {
    fn next_input(&mut self, obs: &Observation) -> InputFrame;
}
```

Built-in implementations: `NullPolicy`, `RandomPolicy`, `ScriptedPolicy`.

### Observation

A lightweight view into engine state after each frame:

```rust
pub struct Observation<'a> {
    pub frame: u64,
    pub state_hash: u64,
    pub game_state: &'a GameState,
    pub entity_count: usize,
    pub framebuffer: Option<&'a [u8]>,
}
```

## Module Dependency Graph

```
                        ┌──────────────┐
                        │  InputFrame  │  Canonical input
                        └──────┬───────┘
                               │
                         ┌─────▼──────┐        ┌────────────┐
                         │  Headless  │◄───────│  Policy    │
                         │  Runner    │        │  Trait     │
                         └─────┬──────┘        └────────────┘
                               │
                         ┌─────▼──────┐
                         │  SimResult │  state_hash, game_state, fb_hash
                         └─────┬──────┘
                               │
        ┌──────────────────────┼──────────────────────┐
        │                      │                      │
  ┌─────▼──────┐        ┌─────▼──────┐        ┌─────▼──────┐
  │  Fitness   │        │  Regression │        │  Replay    │
  │  Evaluator │        │  Suite      │        │  Recording │
  └─────┬──────┘        └─────┬──────┘        └─────┬──────┘
        │                      │                      │
        │                      │               ┌─────▼──────┐
  ┌─────▼──────────────────────▼──────┐        │  Compare   │
  │           Experiment              │        │  Replays   │
  └─────┬─────────────────────────────┘        └────────────┘
        │
  ┌─────▼──────┐     ┌────────────┐     ┌────────────┐
  │ Hill Climber│     │  Anomaly   │     │  Divergence│
  └────────────┘     │  Detector  │     │  Replay    │
                     └────────────┘     └────────────┘

  ┌────────────┐     ┌────────────┐     ┌────────────┐
  │  Strategy  │     │  Test      │     │  Golden    │
  │  Playbook  │     │  Harness   │     │  Test      │
  └────────────┘     └────────────┘     └────────────┘

  ┌────────────┐     ┌────────────┐     ┌────────────┐
  │  Death     │     │  Highlights│     │  Ablation  │
  │  Classify  │     │  Scanner   │     │  Study     │
  └────────────┘     └────────────┘     └────────────┘

                     ┌────────────┐
                     │  Dashboard │  Integrates all above
                     └────────────┘
```

## Core Layer

### HeadlessRunner

The foundation. Creates an `Engine` instance with a specified viewport, runs simulations via the `Simulation` trait, and returns a `SimResult`.

```rust
let mut runner = HeadlessRunner::new(480, 720);

// Run with fixed inputs
let result = runner.run_sim(&mut sim, seed, &inputs, config);

// Run with a policy
let result = runner.run_with_policy(&mut sim, &mut policy, seed, frames, config);
```

### SimResult

The universal output of every simulation:
- `frames_run: u64`
- `game_state: HashMap<String, StateValue>`
- `framebuffer_hash: u64`
- `state_hash: u64`
- `state_hashes: Vec<u64>` (optional per-frame)
- `elapsed_sim_time: f64`

### PlaythroughFile

Serializable replay format:
- Seed, inputs, frame count, final state/framebuffer hashes
- Per-frame state hashes for determinism verification
- Metadata key-value pairs

## Analysis Layer

### Parameter Sweep
`run_sweep()` runs the same scenario with different parameter configurations. Returns a `SweepReport` that can be queried for min/max by any state key.

### State Timeline
`record_timeline()` captures specific state keys at every frame. `StateTimeline` supports `series()`, `stats()`, and `first_frame_where()` queries.

### Fitness Evaluator
Composable weighted scoring: define criteria like proximity, efficiency, completion. Each criterion maps to [0, 1]. The evaluator produces a `FitnessResult` with letter grades (A+ through F) and can rank entire sweeps.

### Regression Suite
Capture baselines, diff against future runs. Pluggable classifiers determine what counts as regression vs. improvement.

## Replay Layer

### Replay
`record_replay()` captures specified state keys at every frame. Lightweight and efficient for full-run analysis.

### Comparison
`compare_replays()` produces a structural diff: per-key `max_delta`, `mean_delta`, `first_divergence`, and full delta series.

### Anomaly Detection
`AnomalyDetector` scans replay series for spikes, plateaus, and out-of-bounds values.

## Optimization Layer

### Experiment
Combines sweep + fitness + optional regression into a single declarative call.

### Hill Climber
Coordinate-descent optimizer for game parameters. Tries +/-step for each parameter, keeps improvements, shrinks step when stuck.

### Action Generator
Programmatic input generation: `grid_shots()`, `random_shots()`, `tap_sequence()`, `drag()`.

## Orchestration Layer

### Strategy
Multi-step playbook: chain Record, Compare, DetectAnomalies, and AssertState steps.

### Test Harness
Battery testing: run multiple scenarios, produce a consolidated report with pass/fail counts and average fitness.

### Golden Test
Reference replay comparison. Record a golden replay when behavior is correct, then diff future runs against it.

## Design Acceleration Layer

### Death Classification
Classifies terminal states by trajectory shape: CloseCall, Blowout, Cliff, Attrition, Unclassified. Pure math — slope and variance calculations on the last N frames.

### Divergence Replay
Compares two runs frame-by-frame via state hash comparison. Finds the exact frame where behavior diverges.

### Feel Presets
Library of named physics profiles (tight_platformer, floaty_astronaut, heavy_tank, snappy_cursor, underwater, ice_skating). Applied via `global_state`. TOML format for custom presets.

### Variant Branching
Games declare parameter variants via `Simulation::variants()`. The engine can sweep all variants across seed ranges and compare outcomes.

### Interesting Moment Detection
Scans batch runs for notable events: spikes, drops, near-death experiences, milestones. Produces ranked highlight reports.

### Mechanic Ablation
Runs baseline vs. mechanic-disabled sweeps, computes impact deltas, and ranks mechanics by contribution.

### Dashboard
Generates structured JSON combining sweep stats, death classification, highlights, ablation results, and golden test status. Static HTML frontend renders the data.

## Game Integration

Every headless module is game-agnostic. Games integrate by implementing the `Simulation` trait:

```rust
pub struct MyGame { ... }

impl Simulation for MyGame {
    fn setup(&mut self, engine: &mut Engine) { /* initialize */ }
    fn step(&mut self, engine: &mut Engine) { /* game logic */ }
    fn render(&self, engine: &mut Engine) { /* draw */ }
}
```

The built-in `DemoBall` demo (`demo_ball.rs`) validates the full pipeline.

## Data Flow: A Typical AI Iteration

```
1. Create Simulation implementation
2. Run sweep across seed range with policy
3. Evaluate fitness, classify deaths
4. Use hill climber to fine-tune parameters
5. Record golden replay of improved behavior
6. Verify no regressions against golden baseline
7. Run highlights to find interesting moments
8. Commit with structured test evidence
```

## File Inventory

| File | Layer | Purpose |
|------|-------|---------|
| `runner.rs` | Core | HeadlessRunner, SimResult |
| `scenario.rs` | Core | GameScenario, ScheduledAction, ScenarioBuilder |
| `fb_hash.rs` | Core | FNV-1a framebuffer hashing |
| `sweep.rs` | Analysis | Parameter sweep |
| `timeline.rs` | Analysis | Per-frame state recording |
| `fitness.rs` | Analysis | Weighted scoring, ranking |
| `regression.rs` | Analysis | Baseline diff, classifiers |
| `snapshot.rs` | Snapshot | Mid-simulation state capture |
| `replay.rs` | Replay | Full per-frame recording |
| `compare.rs` | Replay | Side-by-side diff |
| `anomaly.rs` | Replay | Spike/plateau/OOB detection |
| `experiment.rs` | Optimization | Sweep + fitness + regression |
| `hill_climb.rs` | Optimization | Coordinate descent optimizer |
| `action_gen.rs` | Optimization | Input sequence generation |
| `strategy.rs` | Orchestration | Multi-step playbooks |
| `harness.rs` | Orchestration | Battery testing |
| `golden.rs` | Orchestration | Reference replay comparison |
| `death_classify.rs` | Acceleration | Terminal state classification |
| `death_report.rs` | Acceleration | Batch death classification |
| `divergence.rs` | Acceleration | Run comparison/diffing |
| `highlights.rs` | Acceleration | Interesting moment detection |
| `ablation.rs` | Acceleration | Mechanic ablation study |
| `dashboard.rs` | Acceleration | Dashboard data generation |
| `variant_runner.rs` | Acceleration | Variant sweep runner |
| `variant_rewind.rs` | Acceleration | Replay-based rewind + branch |
| `tests.rs` | Testing | Tests covering all modules |
