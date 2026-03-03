# Design Acceleration — Top Implementation Plan

## Overview

Seven features that compress the design iteration loop. Total estimated effort: ~4,930 lines across 12 new files and 14 file modifications. All features build on the existing deterministic simulation infrastructure (Phases 1-7 complete).

---

## Execution Order

| Phase | Feature | Effort | New Files | Impact | Depends On |
|-------|---------|--------|-----------|--------|------------|
| A | 2. Death Classification | ~770 LOC | 2 | High | Nothing |
| B | 3. Divergence Replay | ~625 LOC | 1 | Medium | Nothing |
| C | 4. Feel Presets | ~900 LOC | 1 | Medium | Nothing (adds `toml` crate) |
| D | 1. Variant Branching | ~800 LOC | 3 | High | Nothing |
| E | 5. Interesting Moments | ~535 LOC | 1 | High | Nothing |
| F | 6. Mechanic Ablation | ~400 LOC | 1 | Medium | Nothing |
| G | 7. Dashboard | ~700 LOC | 2 | High | Features 2, 5, 6 |

**Phases A-F are independent** — can be implemented in any order or in parallel. Phase G (Dashboard) integrates everything and must come last.

**Recommended serial order**: A → B → C → D → E → F → G

- Start with Death Classification (A) — lowest effort, highest immediate value, pure math
- Divergence Replay (B) next — cheap to build, essential for regression debugging
- Feel Presets (C) — self-contained utility, useful for every new game
- Variant Branching (D) — extends Simulation trait, benefits from having A+C already
- Interesting Moments (E) — medium effort, high impact, uses batch infrastructure
- Mechanic Ablation (F) — medium effort, most useful mid-development
- Dashboard (G) last — ties everything together

---

## Architectural Decisions (Cross-Cutting)

### 1. All features use `global_state` as the parameter mechanism
Variants, presets, and ablations all apply parameters via `engine.global_state.set_f64()`. Games read tuning constants from `global_state` during `step()`. No new parameter storage layer needed.

### 2. `BTreeMap` everywhere for determinism
All parameter maps use `BTreeMap<String, f64>` for deterministic iteration order, consistent with the engine's determinism principles.

### 3. No breaking changes to Simulation trait
- Feature 1 adds `variants()` with a default empty implementation
- Features 5 and 6 use separate optional traits (`HighlightConfig`, `Ablatable`)
- All backward compatible

### 4. Factory closures for batch runs
Batch operations (sweeps, highlights, ablation) use `Fn() -> S` factory closures rather than requiring `Clone` on `Simulation`. Matches the existing CLI pattern where `DemoBall::new()` is called fresh per seed.

### 5. JSONL output format for all data-producing commands
Consistent with existing CLI commands. Enables `jq` piping, Python consumption, and downstream analysis.

### 6. Dashboard is static HTML, no frameworks
Vanilla JS + canvas charts. Same pattern as the existing `site/demo-ball/index.html`. Auto-refresh via JSON polling — no WebSocket complexity.

---

## Phase A: Death Classification

**Files to create:**
- `engine/crates/engine-core/src/headless/death_classify.rs` (~300 LOC)
- `engine/crates/engine-core/src/headless/death_report.rs` (~150 LOC)

**Files to modify:**
- `engine/crates/engine-core/src/headless/mod.rs`
- `engine/crates/engine-cli/src/main.rs`

**Key types:** `DeathClass` (CloseCall/Blowout/Cliff/Attrition/Unclassified), `ClassifierConfig`, `DeathClassification`, `DeathReport`

**Algorithm:** Linear regression on metric trajectory over last N frames. Classify by slope magnitude, R², max single-frame drop position, and close-call fraction. Pure math — no ML.

**CLI:** `engine-cli deaths [--metric KEY] [--window N] [--seed-range S..E] [--frames N]`

**Tests:** ~250 LOC — synthetic series per class, batch aggregation, JSON roundtrip

---

## Phase B: Divergence Replay

**Files to create:**
- `engine/crates/engine-core/src/headless/divergence.rs` (~350 LOC)

**Files to modify:**
- `engine/crates/engine-core/src/headless/mod.rs`
- `engine/crates/engine-cli/src/main.rs`

**Key types:** `DivergenceReport`, `DivergenceContext`, `SweepDivergenceReport`, `SeedDivergence`, `DivergenceClip`

**Algorithm:** Compare two PlaythroughFiles frame-by-frame via state hash comparison. First mismatch = divergence point. For sweeps: match seeds across two result sets, rank by outcome delta.

**CLI:**
- `engine-cli divergence files <A.json> <B.json> [--context N]`
- `engine-cli divergence sweep <A.jsonl> <B.jsonl> [--key score] [--top N]`

**Tests:** ~150 LOC — identical runs, different seeds, different inputs, context window sizing

---

## Phase C: Feel Presets

**Files to create:**
- `engine/crates/engine-core/src/feel_preset.rs` (~500 LOC)

**Files to modify:**
- `engine/crates/engine-core/src/lib.rs`
- `engine/crates/engine-core/Cargo.toml` (add `toml = "0.8"`)
- `engine/crates/engine-cli/Cargo.toml` (add `toml = "0.8"`)
- `engine/crates/engine-cli/src/main.rs`

**Key types:** `FeelPreset`, `FeelPresetLibrary`

**Built-in presets (6):**
1. `tight_platformer` — high gravity, high acceleration, strong friction
2. `floaty_astronaut` — low gravity, low friction, high inertia
3. `heavy_tank` — high mass, slow turn, strong momentum
4. `snappy_cursor` — zero gravity, instant acceleration, high damping
5. `underwater` — high drag, buoyancy, floaty movement
6. `ice_skating` — very low friction, momentum-based

**Integration:** `to_sweep_configs()` bridges to existing sweep infra. `to_param_ranges(margin)` bridges to hill climbing. TOML format for human-authored presets.

**CLI:**
- `engine-cli preset list`
- `engine-cli preset show <name>`
- `engine-cli preset apply <name> [--frames N] [--override key=val]`
- `engine-cli preset export [--keys K1,K2] [--out FILE]`
- `engine-cli preset sweep [--frames N] [--key score]`

**Tests:** ~200 LOC — library completeness, apply/override, TOML/JSON roundtrip, export, merge

---

## Phase D: Variant Branching

**Files to create:**
- `engine/crates/engine-core/src/variant.rs` (~80 LOC)
- `engine/crates/engine-core/src/headless/variant_runner.rs` (~200 LOC)
- `engine/crates/engine-core/src/headless/variant_rewind.rs` (~180 LOC)

**Files to modify:**
- `engine/crates/engine-core/src/simulation.rs` — add `variants()` default method
- `engine/crates/engine-core/src/demo_ball.rs` — derive Clone, add variants(), refactor to read from global_state
- `engine/crates/engine-core/src/headless/mod.rs`
- `engine/crates/engine-core/src/lib.rs`
- `engine/crates/engine-cli/src/main.rs`

**Key types:** `ParamSet`, `VariantResult`, `VariantSweepReport`, `BranchResult`

**Rewind mechanism:** Replay-based (not snapshot-based). Rewind to frame N = replay from frame 0 to N. Slower but zero new infrastructure needed.

**CLI:**
- `engine-cli variants`
- `engine-cli variant-sweep [--seed-range S..E] [--frames N] [--turbo]`

**Tests:** ~200 LOC — variant produces different results, same variant deterministic, rewind correctness, multi-branch

---

## Phase E: Interesting Moment Detection

**Files to create:**
- `engine/crates/engine-core/src/headless/highlights.rs` (~450 LOC)

**Files to modify:**
- `engine/crates/engine-core/src/headless/mod.rs`
- `engine/crates/engine-core/src/headless/runner.rs` — add `run_with_capture` method
- `engine/crates/engine-cli/src/main.rs`

**Key types:** `HighlightScanner`, `MomentKind`, `DetectorConfig`, `Highlight`, `HighlightClip`, `HighlightReport`

**Detectors:**
- **NearMiss** — single-pass, metric approaches then recovers from threshold
- **Reversal** — single-pass, sharp direction change after sustained trend
- **RareEvent** — two-pass (collect distributions, then flag outliers)
- **StateNovelty** — accumulates hash buckets across seeds, flags new regions
- **DecisionPoint** — variance-based
- **Custom** — game-defined predicate function

**CLI:** `engine-cli highlights [--seed-range 0..1000] [--frames 600] [--detectors near-miss,reversal,novelty] [--top N] [--clips]`

**Tests:** synthetic series per detector type, ranking, serde roundtrip

---

## Phase F: Mechanic Ablation

**Files to create:**
- `engine/crates/engine-core/src/headless/ablation.rs` (~400 LOC)

**Files to modify:**
- `engine/crates/engine-core/src/headless/mod.rs`
- `engine/crates/engine-cli/src/main.rs`

**Key types:** `AblationRunner`, `AblationConfig`, `AblationReport`, `AblationDelta`, `AblationRunResult`, `AggregateMetric`, optional `Ablatable` trait

**Algorithm:** Baseline sweep → per-mechanic ablation sweeps → delta computation → impact ranking. Significance via z-test. Mechanics disabled via GameState flag overrides.

**CLI:** `engine-cli ablation [--seeds N] [--frames N] [--metrics key1,key2]`

**Tests:** known baseline + known ablated = correct deltas, ranking order, significance

---

## Phase G: Continuous Dashboard

**Files to create:**
- `engine/crates/engine-core/src/headless/dashboard.rs` (~300 LOC)
- `site/dashboard/index.html` (~400 LOC)

**Files to modify:**
- `engine/crates/engine-core/src/headless/mod.rs`
- `engine/crates/engine-cli/src/main.rs`
- `.github/workflows/deploy.yml` — add `_site/dashboard/` to assembly

**Key types:** `DashboardData`, `DashboardConfig`, `SweepSummary`, `MetricStats`, `OutcomeClassification`, `GoldenStatus`, `TrendData`

**Architecture:**
1. `engine-cli dashboard-data` — runs analysis suite, writes `dashboard.json`
2. `site/dashboard/index.html` — reads JSON, renders with vanilla JS + canvas
3. `engine-cli dashboard --watch` — monitors binary, re-generates on change

**Dashboard sections:** Sweep summary, outcome classification, highlight reel, ablation ranking, golden status, metric trends

**Tests:** DashboardData serde roundtrip, MetricStats computation, histogram binning

---

## New Dependencies

| Crate | Version | Used By | Purpose |
|-------|---------|---------|---------|
| `toml` | 0.8 | Phase C (Feel Presets) | TOML file parsing for preset files |

No other new external dependencies. All features use existing serde, serde_json.

---

## New CLI Commands Summary

| Command | Phase | Purpose |
|---------|-------|---------|
| `deaths` | A | Death classification report |
| `divergence files` | B | Compare two playthroughs |
| `divergence sweep` | B | Compare two sweep results |
| `preset list/show/apply/export/sweep` | C | Physics feel presets |
| `variants` | D | List game variants |
| `variant-sweep` | D | Sweep across variants |
| `highlights` | E | Interesting moment detection |
| `ablation` | F | Mechanic ablation study |
| `dashboard-data` | G | Generate dashboard JSON |
| `dashboard` | G | Serve dashboard with watch mode |

---

## Definition of Done

- [ ] Death classification produces correct categorization on synthetic series
- [ ] Divergence replay finds exact frame of hash mismatch between two runs
- [ ] Feel presets library has 6+ built-in profiles, TOML roundtrip works
- [ ] Variant branching produces different results per variant, rewind is deterministic
- [ ] Highlight scanner detects near-miss, reversal, and novelty moments
- [ ] Ablation report correctly ranks mechanics by impact with significance testing
- [ ] Dashboard renders all sections from generated JSON
- [ ] All new CLI commands output valid JSON/JSONL
- [ ] All existing 1049+ tests continue to pass
- [ ] No modifications to Simulation trait break existing implementors
