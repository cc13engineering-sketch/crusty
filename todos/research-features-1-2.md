# Implementation Research: Features 1-2

## Feature 1: Variant Branching (~800 lines, 6 files modified, 3 files created)

### New Files
- `engine/crates/engine-core/src/variant.rs` (~80 lines) — `ParamSet` (BTreeMap<String, f64>), builder pattern, `apply_to(global_state)`
- `engine/crates/engine-core/src/headless/variant_runner.rs` (~200 lines) — `run_variant()`, `sweep_variants()`, `sweep_variants_with_policy()`, `VariantResult`, `VariantSweepReport`
- `engine/crates/engine-core/src/headless/variant_rewind.rs` (~180 lines) — `rewind_and_branch()`, `multi_branch()`, `BranchResult`

### Modified Files
- `simulation.rs` — Add default method `fn variants(&self) -> Vec<ParamSet> { vec![] }`
- `demo_ball.rs` — Derive Clone, add `variants()` returning 3 presets (default/fast/slow), refactor step() to read tuning from global_state
- `headless/mod.rs` — Register new modules
- `lib.rs` — `pub mod variant`
- CLI `main.rs` — Add `variants` and `variant-sweep` subcommands

### Key Design Decisions
1. **Variants use `global_state`, not a separate mechanism.** Games already read tuning from `engine.global_state`. Variants override after `setup()`.
2. **`ParamSet` uses `BTreeMap`** for deterministic iteration (Follow-Up B alignment).
3. **Rewind is replay-based, not snapshot-based.** Leverages determinism: rewind to frame N = replay from 0 to N. Slower but zero new infrastructure.
4. **`Simulation::variants()` has a default impl** — backward compatible.
5. **`Simulation + Clone` required for sweeps** — each seed/variant combo needs a fresh instance.

### CLI Commands
```
engine-cli variants                                    # List variants declared by the game
engine-cli variant-sweep [--seed-range S..E] [--frames N] [--turbo] [--out FILE]
```

### Testing Strategy
- ParamSet: construction, builder, apply_to, serde roundtrip
- variant_runner: different variants produce different results, same variant+seed deterministic
- variant_rewind: branch at frame 0 = run_variant, branch_point_hash matches straight run
- DemoBall integration: fast variant > slow variant score over multiple seeds
- CLI: `variants` outputs valid JSON, `variant-sweep` outputs valid JSONL

---

## Feature 2: Death Classification (~770 lines, 2 files modified, 2 files created)

### New Files
- `engine/crates/engine-core/src/headless/death_classify.rs` (~300 lines) — `DeathClass` enum (CloseCall, Blowout, Cliff, Attrition, Unclassified), `ClassifierConfig`, `classify()`, linear regression math
- `engine/crates/engine-core/src/headless/death_report.rs` (~150 lines) — `DeathReport`, `RunClassification`, `classify_batch()`, `run_and_classify()`

### Modified Files
- `headless/mod.rs` — Register new modules
- CLI `main.rs` — Add `deaths` subcommand

### Classification Algorithm (pure math, no ML)
1. Extract last N frames (window_size, default 120 = 2s at 60fps) of tracked metric
2. Run linear regression to get slope, intercept, R²
3. Find max single-frame drop and its position
4. Compute close-call fraction (frames within band of terminal threshold)
5. Classify:
   - **Cliff**: large drop (>50% of range) in latter half of window
   - **Close call**: >50% of frames within band of threshold
   - **Blowout**: high R² + steep negative slope
   - **Attrition**: high R² + moderate negative slope
   - **Unclassified**: fallback

### CLI Command
```
engine-cli deaths [--metric KEY] [--window N] [--seed-range S..E] [--frames N]
                  [--threshold T] [--out FILE] [--pretty]
```

### Output Example
```
Death Report: 95/100 runs classified
  close_call: 57 (60.0%)
  blowout: 19 (20.0%)
  cliff: 14 (14.7%)
  attrition: 5 (5.3%)
```

### Testing Strategy
- Math: linear_regression on known data, constant data, noisy data
- Classification: construct synthetic series for each class, assert correct classification
- Batch: mixed series returns correct breakdown counts, percentages sum to ~100%
- Integration: run DemoBall with null policy, classify by "score"
- CLI: `deaths --seed-range 0..5 --frames 100 --metric score` produces valid JSON

---

## Implementation Order Recommendation
1. **Feature 2 (Death Classification) first** — simpler, self-contained, no trait changes
2. **Feature 1 (Variant Branching) second** — trait change is more significant; having death classification already means variant sweep output can be classified immediately

## Shared Dependencies
- Both independent of each other, can be parallelized
- Both build on existing: HeadlessRunner, SimResult, global_state, Simulation trait, sweep infra
- Both use BTreeMap for deterministic iteration
- Both output JSON/JSONL via serde
