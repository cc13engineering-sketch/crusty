# Design Acceleration — Implementation Progress

## Status: IN PROGRESS

## Phase A: Death Classification — NOT STARTED
- [ ] `headless/death_classify.rs` — DeathClass enum, ClassifierConfig, linear regression, classify()
- [ ] `headless/death_report.rs` — DeathReport, classify_batch(), run_and_classify()
- [ ] Register in `headless/mod.rs`
- [ ] CLI `deaths` command in `engine-cli/src/main.rs`
- [ ] Tests pass
- [ ] Committed and pushed

## Phase B: Divergence Replay — NOT STARTED
- [ ] `headless/divergence.rs` — DivergenceReport, compare_playthroughs(), compare_sweep_results()
- [ ] Register in `headless/mod.rs`
- [ ] CLI `divergence` command in `engine-cli/src/main.rs`
- [ ] Tests pass
- [ ] Committed and pushed

## Phase C: Feel Presets — NOT STARTED
- [ ] `feel_preset.rs` — FeelPreset, FeelPresetLibrary, 6 built-in presets
- [ ] Add `toml` dependency to Cargo.toml files
- [ ] Register in `lib.rs`
- [ ] CLI `preset` command in `engine-cli/src/main.rs`
- [ ] Tests pass
- [ ] Committed and pushed

## Phase D: Variant Branching — NOT STARTED
- [ ] `variant.rs` — ParamSet with BTreeMap
- [ ] `headless/variant_runner.rs` — run_variant(), sweep_variants()
- [ ] `headless/variant_rewind.rs` — rewind_and_branch(), multi_branch()
- [ ] Extend Simulation trait with `variants()` default method
- [ ] Update DemoBall: derive Clone, add variants(), read from global_state
- [ ] Register in `headless/mod.rs` and `lib.rs`
- [ ] CLI `variants` and `variant-sweep` commands
- [ ] Tests pass
- [ ] Committed and pushed

## Phase E: Interesting Moment Detection — NOT STARTED
- [ ] `headless/highlights.rs` — HighlightScanner, detectors, HighlightReport
- [ ] Add `run_with_capture` to HeadlessRunner
- [ ] Register in `headless/mod.rs`
- [ ] CLI `highlights` command
- [ ] Tests pass
- [ ] Committed and pushed

## Phase F: Mechanic Ablation — NOT STARTED
- [ ] `headless/ablation.rs` — AblationRunner, AblationReport, AblationConfig
- [ ] Register in `headless/mod.rs`
- [ ] CLI `ablation` command
- [ ] Tests pass
- [ ] Committed and pushed

## Phase G: Dashboard — NOT STARTED
- [ ] `headless/dashboard.rs` — DashboardData, generate_dashboard_data()
- [ ] `site/dashboard/index.html` — static HTML dashboard
- [ ] Register in `headless/mod.rs`
- [ ] CLI `dashboard-data` command
- [ ] Update deploy.yml
- [ ] Tests pass
- [ ] Committed and pushed
