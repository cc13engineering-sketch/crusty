# AI-Driven Iterative Game Development

How AI agents work iteratively against the Crusty Engine's headless mode to autonomously analyze, optimize, and improve games — without a browser, display, or human intervention.

## The Core Insight

Traditional game development requires a human in the loop: play the game, observe behavior, hypothesize improvements, implement changes, play again. The headless testing infrastructure eliminates this bottleneck by giving an AI agent the same observational and analytical capabilities — but faster, more systematic, and with perfect reproducibility.

AI agents don't "play" the game visually. They run simulations via the `Simulation` trait, read state data through `Observation`, score outcomes with `FitnessEvaluator`, and make evidence-based decisions about what to change. Every iteration is backed by quantitative analysis, not subjective assessment.

## The Feedback Loop

```
  1. ANALYZE: What does the game currently do?
     - Sweep across seeds with random/null policies
     - Record replays for trajectory analysis
     - Anomaly detection for physics bugs
     - Death classification for difficulty diagnosis

  2. MEASURE: How good is it?
     - Fitness evaluation with weighted criteria
     - Regression suite against known-good baselines
     - Highlight detection for interesting moments

  3. OPTIMIZE: What parameters improve it?
     - Parameter sweep to explore variations
     - Hill climbing for fine-tuning
     - Variant branching to compare mechanic alternatives
     - Mechanic ablation to find what matters

  4. VERIFY: Did the change actually help?
     - Golden test for before/after comparison
     - Divergence replay to pinpoint where behavior changed
     - Regression suite to protect existing quality

  5. IMPLEMENT: Apply the improvement
     - Modify game code with optimized values
     - Write tests that encode the improvement
     - Commit with structured evidence
```

## Concrete Example

### Step 1: Baseline Analysis

```rust
use crate::headless::runner::{HeadlessRunner, RunConfig};
use crate::policy::RandomPolicy;
use crate::demo_ball::DemoBall;

let mut runner = HeadlessRunner::new(480, 720);
let config = RunConfig { turbo: true, capture_state_hashes: true };

// Sweep 100 seeds with random input
for seed in 0..100 {
    let mut sim = DemoBall::new();
    let mut policy = RandomPolicy::new(seed);
    let result = runner.run_with_policy(&mut sim, &mut policy, seed, 600, config);
    // Collect results for analysis
}
```

### Step 2: Death Classification

```bash
cargo run -p engine-cli -- deaths --seed-range 0..100 --frames 600 --metric score --pretty
```

Output reveals: "60% close calls, 20% blowouts, 15% cliffs." The blowouts suggest an early-game problem.

### Step 3: Parameter Optimization

```bash
# Sweep variants
cargo run -p engine-cli -- variant-sweep --seed-range 0..100 --frames 600 --turbo --pretty

# Check which mechanics carry the fun
cargo run -p engine-cli -- ablation --seeds 50 --frames 600 --pretty
```

### Step 4: Verification

```bash
# Record golden before changes
cargo run -p engine-cli -- golden record --seed 42 --frames 600 --out golden_pre.json

# ... apply changes ...

# Verify no regressions
cargo run -p engine-cli -- golden check golden_pre.json

# Find where behavior diverged
cargo run -p engine-cli -- divergence files golden_pre.json golden_post.json
```

## Why This Works

### Determinism
Every simulation with the same seed + inputs produces identical outputs. No timing jitter, no frame drops, no OS differences. Every comparison is meaningful and every optimization is reproducible.

### Quantification
Instead of "the ball feels wrong," the system produces concrete data: death classifications, fitness scores, state hash divergence points. AI agents work best with numbers, not vibes.

### Speed
A full 600-frame simulation takes milliseconds in turbo mode. A sweep of 1000 seeds completes in seconds. This enables rapid iteration cycles that would take hours of manual playtesting.

### Composability
The modules compose naturally: run sweeps, score with fitness, classify deaths, detect highlights, compare with golden tests. Each module does one thing well and chains with the others through the CLI or Rust API.

## The Autonomous Loop

The most powerful pattern emerges when an AI agent uses headless outputs to inform its next code change, then validates with the same tools:

```
Agent reads game code
  -> identifies a physics parameter (gravity = 500.0)
  -> runs variant sweep with gravity = [300, 400, 500, 600, 700]
  -> evaluates fitness: gravity=400 scores 15% better
  -> runs death classification: blowout rate dropped from 20% to 8%
  -> modifies source code: let gravity = 400.0;
  -> runs golden test: no regressions
  -> runs highlight detection: more interesting moments found
  -> commits with structured evidence
```

## Design Principles

### Game-Agnostic Infrastructure
Every module works through the `Simulation` trait. The infrastructure knows nothing about specific game mechanics. It runs frames via `step()` and observes state.

### AI-Readable Output
Every CLI command outputs JSON/JSONL. Result types have `.summary()` methods producing compact structured text. No verbose logs — just the signal an agent needs to make decisions.

### Progressive Complexity
- **Quick check**: `engine-cli batch` with a few seeds
- **Deep analysis**: `engine-cli deaths` + `engine-cli highlights`
- **Full optimization**: variant sweep + ablation + golden test

### Evidence-Based Development
Every change is justified by data. The test suite doesn't just prevent regressions — it provides the quantitative evidence that drove the improvement.
