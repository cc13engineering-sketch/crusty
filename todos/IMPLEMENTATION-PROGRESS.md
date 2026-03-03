# Design Acceleration — Implementation Progress

## Status: COMPLETE (All 7 phases implemented)

## Phase A: Death Classification — COMPLETE
- [x] `headless/death_classify.rs` — DeathClass enum, ClassifierConfig, linear regression, classify()
- [x] `headless/death_report.rs` — DeathReport, classify_batch()
- [x] Register in `headless/mod.rs`
- [x] CLI `deaths` command in `engine-cli/src/main.rs`
- [x] 15 new tests pass (10 death_classify + 5 death_report)
- [x] Committed and pushed (1be9d4e)

## Phase B: Divergence Replay — COMPLETE
- [x] `headless/divergence.rs` — DivergenceReport, compare_hash_sequences(), compare_sweep_outcomes()
- [x] Register in `headless/mod.rs`
- [x] CLI `divergence` command (files/sweep subcommands) in `engine-cli/src/main.rs`
- [x] 7 new tests pass
- [x] Committed and pushed (1be9d4e)

## Phase C: Feel Presets — COMPLETE
- [x] `feel_preset.rs` — FeelPreset, FeelPresetLibrary, 6 built-in presets
- [x] Add `toml` dependency to Cargo.toml files
- [x] Register in `lib.rs`
- [x] CLI `preset` command (list/show/apply) in `engine-cli/src/main.rs`
- [x] 8 new tests pass
- [x] Committed and pushed (1be9d4e)

## Phase D: Variant Branching — COMPLETE
- [x] `variant.rs` — ParamSet with BTreeMap, builder pattern, serde, apply_to()
- [x] `headless/variant_runner.rs` — run_variant(), sweep_variants(), VariantSweepReport
- [x] `headless/variant_rewind.rs` — rewind_and_branch(), multi_branch(), BranchResult
- [x] Extend Simulation trait with `variants()` default method
- [x] Update DemoBall: derive Clone, add variants(), read from global_state
- [x] Register in `headless/mod.rs` and `lib.rs`
- [x] CLI `variants` and `variant-sweep` commands
- [x] 18 tests pass (8 variant + 5 variant_runner + 5 variant_rewind)

## Phase E: Interesting Moment Detection — COMPLETE
- [x] `headless/highlights.rs` — HighlightScanner, spike/drop/near-death/milestone detectors, HighlightReport
- [x] Add `run_with_capture` to HeadlessRunner
- [x] Register in `headless/mod.rs`
- [x] CLI `highlights` command
- [x] 14 tests pass (statistical helpers + detection + integration)

## Phase F: Mechanic Ablation — COMPLETE
- [x] `headless/ablation.rs` — run_ablation_study(), AblationReport, ranked(), summary()
- [x] Register in `headless/mod.rs`
- [x] CLI `ablation` command
- [x] 10 tests pass (delta computation, ranking, serialization, integration)

## Phase G: Dashboard — COMPLETE
- [x] `headless/dashboard.rs` — DashboardData, generate_dashboard_data(), compute_stats(), build_histogram()
- [x] `site/dashboard/index.html` — static HTML dashboard with sweep stats, deaths, histogram, highlights, ablation
- [x] Register in `headless/mod.rs`
- [x] CLI `dashboard-data` command
- [x] Update deploy.yml
- [x] 7 tests pass (stats, histogram, serde, integration)

## Additional Fixes
- [x] Rebuilt WASM package (`_pkg/`) — `setup_demo_ball` now properly exported
- [x] Fixed ablation test (zero-baseline delta_percent edge case)

## Running Totals
- Total new tests: 82+ across all phases
- Total test count: 1157 (all passing)
- New files: 9 (death_classify.rs, death_report.rs, divergence.rs, feel_preset.rs, variant.rs, variant_runner.rs, variant_rewind.rs, highlights.rs, ablation.rs, dashboard.rs, site/dashboard/index.html)
- Modified files: 7 (headless/mod.rs, lib.rs, simulation.rs, demo_ball.rs, main.rs, 2x Cargo.toml, deploy.yml)

## New CLI Commands
| Command | Phase | Purpose |
|---------|-------|---------|
| `deaths` | A | Death classification report |
| `divergence files` | B | Compare two playthroughs |
| `divergence sweep` | B | Compare two sweep results |
| `preset list/show/apply` | C | Physics feel presets |
| `variants` | D | List game variants |
| `variant-sweep` | D | Sweep across variants |
| `highlights` | E | Interesting moment detection |
| `ablation` | F | Mechanic ablation study |
| `dashboard-data` | G | Generate dashboard JSON |
