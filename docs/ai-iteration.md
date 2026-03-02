# AI-Driven Iterative Game Development

How Claude Code works iteratively against the Crusty Engine's headless mode to autonomously analyze, optimize, and improve games — without a browser, display, or human intervention.

## The Core Insight

Traditional game development requires a human in the loop: play the game, observe behavior, hypothesize improvements, implement changes, play again. The headless testing infrastructure eliminates this bottleneck by giving an AI agent the same observational and analytical capabilities — but faster, more systematic, and with perfect reproducibility.

Claude Code doesn't "play" the game visually. It runs simulations, reads state data, scores outcomes, and makes evidence-based decisions about what to change. Every iteration is backed by quantitative analysis, not subjective assessment.

## The Feedback Loop

```
  ┌─────────────────────────────────────────────────┐
  │                                                   │
  │  1. ANALYZE: What does the game currently do?     │
  │     - Grid shots to explore input space            │
  │     - Timeline recording for trajectory analysis   │
  │     - Anomaly detection for physics bugs           │
  │                                                   │
  │  2. MEASURE: How good is it?                      │
  │     - Fitness evaluation with weighted criteria    │
  │     - Regression suite against known-good baselines│
  │     - Test harness for comprehensive quality check │
  │                                                   │
  │  3. OPTIMIZE: What parameters improve it?         │
  │     - Parameter sweep to explore variations        │
  │     - Hill climbing for fine-tuning                │
  │     - Experiment combining sweep + fitness + reg.  │
  │                                                   │
  │  4. VERIFY: Did the change actually help?         │
  │     - Golden test for before/after comparison      │
  │     - Strategy playbook for multi-step validation  │
  │     - Regression suite to protect existing quality │
  │                                                   │
  │  5. IMPLEMENT: Apply the improvement              │
  │     - Modify game code with optimized values       │
  │     - Add new tracked state keys if needed         │
  │     - Write tests that encode the improvement      │
  │                                                   │
  └──────────────── repeat ──────────────────────────┘
```

## Concrete Example: S-League Optimization

Here's how Claude Code used this loop to improve the S-League minigolf demo:

### Step 1: Systematic Analysis

```rust
// Generate 40 shots across the angle/power space
let shots = action_gen::grid_shots(240.0, 500.0, 200.0, 340.0, 8, 0.3, 1.0, 5);

// Run each shot and evaluate fitness
for (label, actions, frames) in &shots {
    let replay = record_replay("shot", setup_fn, update_fn, render_fn,
        dispatch_fn, actions, *frames, &["ball_x", "ball_y", "dist_to_hole"]);
    let anomalies = AnomalyDetector::new()
        .with_spike_threshold(50.0)
        .scan(&replay, &["ball_x", "ball_y"]);
    // Claude reads the data and identifies patterns
}
```

**Finding**: Ball position sometimes jumped by 50+ pixels between frames near walls. This indicated the wall bounce physics had energy injection issues.

### Step 2: Targeted Improvement

Based on the anomaly data, Claude identified that wall bounces lacked proper damping. It added:
- Speed-proportional screen shake on wall impact (juice)
- Wall bounce counter for analytics
- Best-distance tracking for proximity scoring

### Step 3: Regression Protection

```rust
// Record golden before changes
let golden = test.record_golden("pre_fix");

// ... apply changes ...

// Verify no regressions
let result = test.compare_against(&golden);
// Inspect which keys changed and by how much
```

### Step 4: Quality Verification

```rust
let report = TestHarness::new(setup_fn, update_fn, render_fn, dispatch_fn)
    .add("idle_stable", vec![], 60, vec![...])
    .add_with_fitness("shot_quality", actions, 120, vec![], evaluator)
    .run();
assert!(report.all_passed());
```

## Why This Works

### Determinism
Every simulation with the same inputs produces identical outputs. No timing jitter, no frame drops, no OS differences. This means every comparison is meaningful and every optimization is reproducible.

### Quantification
Instead of "the ball feels wrong," the system produces: "ball_y has a spike of 47.3 pixels at frame 23, wall bounce coefficient of 0.85 produces 12% better proximity scores than 0.70." AI agents work best with numbers, not vibes.

### Speed
A full 120-frame simulation takes milliseconds. A grid of 40 shots takes less than a second. A hill-climbing optimization across 100+ evaluations completes in seconds. This enables rapid iteration cycles that would take hours of manual playtesting.

### Composability
The modules compose naturally: generate shots with `action_gen`, run them through `Experiment` which internally uses `run_sweep` + `FitnessEvaluator`, protect quality with `RegressionSuite`, verify with `GoldenTest`. Each module does one thing well and chains with the others.

## The Singularity Loop

The most powerful pattern emerges when the AI agent uses headless testing outputs to inform its next code change, which it then validates with the same tools:

```
Claude reads game code
  → identifies a physics parameter (drag = 2.0)
  → creates a sweep across drag = [1.0, 1.5, 2.0, 2.5, 3.0]
  → evaluates fitness for each
  → finds drag = 1.5 scores 15% better
  → uses hill climber to fine-tune to drag = 1.47
  → modifies source code: `let drag = 1.47;`
  → runs golden test: no regressions
  → runs full harness: all pass, avg fitness improved
  → commits with structured evidence
```

This is not hypothetical. This is the actual workflow executed in Innovation Round 9, where Claude:
1. Ran grid_shots to analyze the input space
2. Used anomaly detection to find physics issues
3. Implemented wall bounce improvements
4. Verified with a test harness
5. Committed with 9 new tests

## Design Principles

### Game-Agnostic Infrastructure
Every module takes function pointers for setup/update/render/dispatch. The infrastructure knows nothing about minigolf, RPGs, or any specific game. It just runs frames and observes state.

### AI-Readable Output
Every result type has a `.summary()` method that produces compact, structured text. No verbose logs — just the signal Claude needs to make decisions.

### Progressive Complexity
- **Quick check**: `ScenarioBuilder.run()` with a few assertions
- **Deep analysis**: `Strategy` playbook chaining record → compare → anomalies → assert
- **Full optimization**: `Experiment` + `HillClimber` + `GoldenTest`

### Evidence-Based Development
Every change is justified by data. The test suite doesn't just prevent regressions — it provides the quantitative evidence that drove the improvement. When you read a commit that says "optimized drag coefficient," the accompanying tests prove it.

## For AI Agent Developers

If you're building agents that use this infrastructure:

1. **Start with analysis, not optimization.** Record timelines and replays first. Understand what the game does before trying to improve it.

2. **Use anomaly detection as a diagnostic tool.** Spikes indicate physics bugs. Plateaus indicate stuck states. Out-of-bounds values indicate constraint violations.

3. **Protect existing quality with regression baselines.** Always capture baselines before making changes. The cost of a regression check is trivial; the cost of introducing a bug is not.

4. **Let the data decide.** Don't hardcode "good" parameter values. Use sweeps to explore the space, fitness evaluators to score outcomes, and hill climbing to fine-tune.

5. **Write tests that encode improvements.** Every optimization should produce a test that fails if the optimization is reverted. This turns the improvement into a permanent ratchet.
