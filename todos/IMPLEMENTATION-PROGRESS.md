# Design Acceleration — Implementation Progress

## Status: IN PROGRESS (Phases A-C complete, D-F in progress)

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

## Phase D: Variant Branching — IN PROGRESS (agent running)
- [ ] `variant.rs` — ParamSet with BTreeMap
- [ ] `headless/variant_runner.rs` — run_variant(), sweep_variants()
- [ ] `headless/variant_rewind.rs` — rewind_and_branch(), multi_branch()
- [ ] Extend Simulation trait with `variants()` default method
- [ ] Update DemoBall: derive Clone, add variants(), read from global_state
- [ ] Register in `headless/mod.rs` and `lib.rs`
- [ ] CLI `variants` and `variant-sweep` commands
- [ ] Tests pass
- [ ] Committed and pushed

## Phase E: Interesting Moment Detection — IN PROGRESS (agent running)
- [ ] `headless/highlights.rs` — HighlightScanner, detectors, HighlightReport
- [ ] Add `run_with_capture` to HeadlessRunner
- [ ] Register in `headless/mod.rs`
- [ ] CLI `highlights` command
- [ ] Tests pass
- [ ] Committed and pushed

## Phase F: Mechanic Ablation — IN PROGRESS (agent running)
- [ ] `headless/ablation.rs` — AblationRunner, AblationReport, AblationConfig
- [ ] Register in `headless/mod.rs`
- [ ] CLI `ablation` command
- [ ] Tests pass
- [ ] Committed and pushed

## Phase G: Dashboard — NOT STARTED (depends on A, E, F)
- [ ] `headless/dashboard.rs` — DashboardData, generate_dashboard_data()
- [ ] `site/dashboard/index.html` — static HTML dashboard
- [ ] Register in `headless/mod.rs`
- [ ] CLI `dashboard-data` command
- [ ] Update deploy.yml
- [ ] Tests pass
- [ ] Committed and pushed

## Running Totals
- Total new tests: 30 (A:15 + B:7 + C:8)
- Total test count: 1105 (all passing)
- New files: 4 (death_classify.rs, death_report.rs, divergence.rs, feel_preset.rs)
- Modified files: 5 (headless/mod.rs, lib.rs, main.rs, 2x Cargo.toml)
