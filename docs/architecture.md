# Crusty Engine: Headless Testing Architecture

## Overview

The Crusty Engine headless testing system is a 15-module infrastructure that enables AI agents (Claude Code) to autonomously run, analyze, optimize, and improve games — all without a browser, display, or human intervention. It runs entirely via `cargo test` and the CLI `simulate` subcommand.

This document covers the technical architecture: how the modules compose, what data flows between them, and how to extend the system for new games.

## Module Dependency Graph

```
                              ┌──────────────┐
                              │  action_gen  │ Generates input sequences
                              └──────┬───────┘
                                     │
         ┌────────────┐        ┌─────▼──────┐
         │ ShotBuilder │───────►│  Scheduled │
         └────────────┘        │  Action    │
                               └─────┬──────┘
                                     │
    ┌────────────┐             ┌─────▼──────┐        ┌────────────┐
    │  Assertion  │────────────►│   Game     │───────►│  Headless  │
    └────────────┘             │  Scenario  │        │  Runner    │
                               └─────┬──────┘        └─────┬──────┘
                                     │                      │
                               ┌─────▼──────┐        ┌─────▼──────┐
                               │  Scenario  │        │  SimResult │
                               │  Result    │        │  + fb_hash │
                               └─────┬──────┘        └─────┬──────┘
                                     │                      │
              ┌──────────────────────┼──────────────────────┤
              │                      │                      │
        ┌─────▼──────┐        ┌─────▼──────┐        ┌─────▼──────┐
        │  Fitness   │        │  Regression │        │  Timeline  │
        │  Evaluator │        │  Suite      │        │            │
        └─────┬──────┘        └─────┬──────┘        └────────────┘
              │                      │
              │                      │
        ┌─────▼──────────────────────▼──────┐
        │           Experiment              │  Combines sweep + fitness + regression
        └─────┬─────────────────────────────┘
              │
        ┌─────▼──────┐
        │ Hill Climber│  Iterative parameter optimization
        └────────────┘

    ┌────────────┐     ┌────────────┐     ┌────────────┐
    │   Replay   │────►│  Compare   │     │  Anomaly   │
    │  Recording │     │  Replays   │     │  Detector  │
    └─────┬──────┘     └────────────┘     └────────────┘
          │
    ┌─────▼──────┐     ┌────────────┐     ┌────────────┐
    │  Strategy  │     │  Test      │     │  Golden    │
    │  Playbook  │     │  Harness   │     │  Test      │
    └────────────┘     └────────────┘     └────────────┘
```

## Core Layer (Rounds 1-2)

### HeadlessRunner
The foundation. Creates an `Engine` instance with a specified viewport (default 480x720), runs the game loop (tick → update → render) for N frames, and returns a `SimResult`.

```rust
let mut runner = HeadlessRunner::new(480, 720);
let result = runner.run(setup, update, render, 60);
// result.frames_run, result.game_state, result.framebuffer_hash
```

**Key insight**: The runner is game-agnostic. It takes function pointers for setup/update/render, so any game module can plug in.

### SimResult
The universal output of every simulation:
- `frames_run: u64` — how many frames were simulated
- `game_state: HashMap<String, StateValue>` — all key-value game state at final frame
- `framebuffer_hash: u64` — FNV-1a hash of the rendered pixels
- `elapsed_sim_time: f64` — total simulated time in seconds

### ScheduledAction
Timed input events injected during simulation:
- `PointerDown { frame, x, y }`
- `PointerMove { frame, x, y }`
- `PointerUp { frame, x, y }`

These are dispatched to the game via an `action_dispatch: fn(&mut Engine, &ScheduledAction)` callback.

### GameScenario
Declarative test: setup + actions + frame count + assertions = structured result.

### ShotBuilder
High-level builder for constructing shot input sequences from angle + power.

## Analysis Layer (Rounds 2-3)

### Parameter Sweep
`run_sweep()` runs the same scenario with different `SweepConfig` overrides (game state modifications applied after setup). Returns a `SweepReport` that can be queried for min/max by any state key.

### State Timeline
`record_timeline()` captures specific state keys at every frame. `StateTimeline` supports `series()`, `stats()`, and `first_frame_where()` queries.

### Fitness Evaluator
Composable weighted scoring: define criteria like proximity, efficiency, completion. Each criterion is a `fn(&SimResult) -> f64` mapping to [0, 1]. The evaluator produces a `FitnessResult` with letter grades (A+ through F) and can rank entire sweeps.

### Regression Suite
Capture baselines, diff against future runs. Pluggable classifiers determine what counts as regression vs. improvement.

## Snapshot & Replay Layer (Rounds 5, 7)

### Snapshots
`run_with_snapshots()` captures full game state at specific frames (vs timeline which captures specific keys at all frames). Best for debugging specific moments.

### Replay
`record_replay()` captures specified state keys at every frame. More lightweight than snapshots (only f64 values, not full StateValue). Supports `series()`, `get()`, `first_frame_where()`.

### Comparison
`compare_replays()` produces a structured diff: per-key `max_delta`, `mean_delta`, `first_divergence`, and full delta series. Also detects visual (framebuffer hash) divergence.

### Anomaly Detection
`AnomalyDetector` scans replay series for three anomaly types:
- **Spike**: sudden large change between consecutive frames
- **Plateau**: value stuck constant for too many frames
- **OutOfBounds**: value outside expected range

## Optimization Layer (Round 6)

### Experiment
Combines sweep + fitness + optional regression into a single `Experiment::new().with_*().run()` call. Returns rankings, regression verdicts, and summaries.

### Hill Climber
Coordinate-descent optimizer. Given `ParamRange` definitions (key, min, max, step), tries +/-step for each parameter, keeps improvements, shrinks step when stuck. Returns best parameters, convergence history, and total evaluations.

### Action Generator
Programmatic input generation:
- `grid_shots()` — systematic angle/power grid
- `random_shots()` — deterministic PRNG exploration
- `tap_sequence()` — UI interaction testing
- `drag()` — interpolated drag gestures

## Orchestration Layer (Round 8)

### Strategy
Multi-step playbook: chain Record → Compare → DetectAnomalies → AssertState steps. Each step operates on named replays captured in earlier steps.

### Test Harness
Battery testing: run multiple scenarios with optional fitness evaluation, produce a consolidated `HarnessReport` with pass/fail counts and average fitness.

### Golden Test
Reference replay comparison: record a "golden" replay when behavior is correct, then diff future runs against it. The headless equivalent of visual regression testing.

## Game Integration

Every headless module is game-agnostic. Games integrate by providing:

1. `setup_fn: fn(&mut Engine)` — initialize game state, tilemap, entities
2. `update_fn: fn(&mut Engine, f64)` — game logic per frame
3. `render_fn: fn(&mut Engine)` — draw to framebuffer
4. `action_dispatch: fn(&mut Engine, &ScheduledAction)` — route inputs to game handlers

The S-League demo provides these as:
- `sleague::setup_fight_only` / `sleague::setup`
- `sleague::update`
- `sleague::render`
- `sleague::dispatch_action`

Game-specific scoring functions (`score_hole_completion`, etc.) live in the game module, not the engine.

## Data Flow: A Typical AI Iteration

```
1. Claude reads game code, identifies physics parameters
2. Creates SweepConfig[] with parameter variations
3. Runs Experiment with fitness evaluator
4. Inspects ExperimentResult.rankings — finds best config
5. Uses HillClimber to fine-tune the best parameters
6. Records golden replay of improved behavior
7. Modifies game code with optimized values
8. Runs GoldenTest to verify no regressions
9. Runs TestHarness for comprehensive quality check
10. Commits with structured test evidence
```

## File Inventory

| File | Module | Purpose |
|------|--------|---------|
| `runner.rs` | Core | HeadlessRunner, SimResult |
| `scenario.rs` | Core | GameScenario, Assertion, ScenarioBuilder |
| `shot_builder.rs` | Core | ShotBuilder |
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
| `tests.rs` | Testing | 1116 tests covering all modules |
