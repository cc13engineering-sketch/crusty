# Implementation Research: Features 5-7

## Feature 5: Interesting Moment Detection (~535 lines, 1 new file + 2 modified)

### New Files
- `engine/crates/engine-core/src/headless/highlights.rs` (~450 lines) — `HighlightScanner`, `HighlightReport`, `Highlight`, `HighlightClip`, `MomentKind`, `DetectorConfig`, `ClipConfig`

### Modified Files
- `headless/mod.rs` — Register highlights module
- CLI `main.rs` — Add `highlights` subcommand (~80 lines)

### Detector Types
- **NearMiss**: metric approaches threshold without crossing (e.g., health drops to 1 then recovers)
- **Reversal**: metric reverses direction sharply after sustained trend
- **RareEvent**: state value in bottom/top percentile across seed population (two-pass detection)
- **DecisionPoint**: high variance across keys indicating branching
- **StateNovelty**: state hash enters a region not seen in previous seeds
- **Custom**: game-defined predicate function

### Key Design Decisions
1. **Two-pass detection for RareEvent** — pass 1 collects distributions across all seeds, pass 2 flags outliers
2. **Optional `HighlightConfig` trait** (not modifying Simulation) — games optionally implement to provide detector configs
3. **Factory closures `Fn() -> S`** instead of `Clone` on Simulation — matches existing CLI pattern
4. **Clips store state snapshots only** (not framebuffers) for memory efficiency
5. **ClipConfig defaults**: 90 frames before + 90 frames after (1.5s each at 60fps)

### CLI Command
```
engine-cli highlights [--seed-range 0..1000] [--frames 600]
                      [--detectors near-miss,reversal,rare,novelty]
                      [--top N] [--clips] [--out FILE]
```

### Testing Strategy
- NearMiss on synthetic series approaching then recovering from threshold
- Reversal on series with known direction changes
- RareEvent with known distribution + one outlier seed
- StateNovelty with duplicate vs unique hash sequences
- HighlightReport::top() ranking
- Serde roundtrip for Highlight, HighlightReport

---

## Feature 6: Mechanic Ablation (~400 lines, 1 new file + 2 modified)

### New Files
- `engine/crates/engine-core/src/headless/ablation.rs` (~400 lines) — `AblationRunner`, `AblationReport`, `AblationConfig`, `AblationDelta`, `AblationRunResult`, `AggregateMetric`, optional `Ablatable` trait

### Modified Files
- `headless/mod.rs` — Register ablation module
- CLI `main.rs` — Add `ablation` subcommand

### Core Algorithm
1. **Baseline run**: sweep N seeds with all mechanics enabled, collect metric aggregates
2. **Per-ablation run**: for each mechanic, sweep N seeds with that mechanic disabled
3. **Delta computation**: compare each ablation's aggregates to baseline
4. **Ranking**: sort mechanics by impact score (sum of |delta_pct| across metrics)
5. **Significance**: simple z-test to flag statistically significant differences

### Ablation Mechanism
- **Via GameState flags** — ablation sets `global_state.set_f64("friction_enabled", 0.0)`, game checks during step()
- Piggybacks on existing `SweepConfig::overrides` pattern
- No changes to Simulation trait — uses separate optional `Ablatable` trait

### Output Format
```
Ablation Report: 4 mechanics, 100 seeds, 600 frames

Impact Ranking (highest first):
  1. combat (impact=34.50)
     score: 450.2 -> 290.1 (-35.5%)*
     game_length: 380.0 -> 520.3 (+36.9%)*
  2. resource_management (impact=12.10)
     score: 450.2 -> 398.5 (-11.5%)*
  3. level_variety (impact=4.20)
     score: 450.2 -> 440.8 (-2.1%)
  4. powerups (impact=1.80)
     score: 450.2 -> 446.3 (-0.9%)
```

### CLI Command
```
engine-cli ablation [--seeds N] [--frames N] [--metrics key1,key2]
                    [--out FILE] [--pretty]
```

### Key Design Decisions
1. **Separate `Ablatable` trait** — does NOT modify `Simulation` trait
2. **Factory closures** — same pattern as Feature 5
3. **Z-test significance** — simple, no external stats dependencies
4. **Impact score = sum of |delta_pct|** — intuitive ranking metric

### Testing Strategy
- Baseline aggregation: known series produces correct mean/std_dev/median
- Delta computation: known baseline + known ablated = correct deltas
- Impact ranking: verify sort order
- Significance: z-test with known distributions
- Integration: run DemoBall with friction ablated, verify different outcomes

---

## Feature 7: Continuous Playtesting Dashboard (~700 lines, 2 new files + 2 modified)

### New Files
- `engine/crates/engine-core/src/headless/dashboard.rs` (~300 lines) — `DashboardData`, `DashboardConfig`, `SweepSummary`, `MetricStats`, `DistBucket`, `OutcomeClassification`, `GoldenStatus`, `TrendData`, `generate_dashboard_data()`
- `site/dashboard/index.html` (~400 lines) — Single-page static HTML dashboard with vanilla JS

### Modified Files
- `headless/mod.rs` — Register dashboard module
- CLI `main.rs` — Add `dashboard-data` and `dashboard` subcommands
- `deploy.yml` — Add `_site/dashboard/` to site assembly

### Architecture: Two Parts
1. **Data generation** (`engine-cli dashboard-data`) — runs full analysis suite, writes `dashboard.json`
2. **Frontend** (`site/dashboard/index.html`) — reads `dashboard.json`, renders with vanilla JS + canvas charts

### Dashboard Sections
- **Header**: game name, engine version, timestamp, freshness indicator
- **Sweep Summary**: metric stats table + canvas-rendered histograms
- **Outcome Classification**: bar chart of end states
- **Interesting Moments**: table of top highlights (seed, frame, kind, score)
- **Mechanic Ablation**: impact ranking with delta bars
- **Golden Status**: pass/fail badge
- **Trends**: up/down arrows vs previous run

### Key Design Decisions
1. **Static HTML, no frameworks** — matches existing site pattern (vanilla JS)
2. **Auto-refresh via polling** — dashboard polls `dashboard.json` timestamp, refreshes when changed
3. **No WebSocket infrastructure** — keeps complexity low
4. **Canvas 2D for charts** — no chart library dependency
5. **Build last** — integrates all other features; each feature works standalone via CLI first

### CLI Commands
```
engine-cli dashboard-data [--seeds N] [--frames N] [--out DIR]
                          [--highlights] [--ablation] [--golden PATH]
engine-cli dashboard [--watch] [--port 8080]
```

### Watch Mode
- Monitors game binary mtime
- On change: re-runs `dashboard-data`, writes new `dashboard.json`
- Browser polls and auto-refreshes

### Testing Strategy
- DashboardData serde roundtrip
- MetricStats computation on known data
- DistBucket histogram binning
- Integration: generate_dashboard_data with DemoBall, verify all sections populated
- Frontend: manual visual testing (no automated browser tests)

---

## Cross-Feature Dependencies

```
Feature 5 (Highlights) ─────┐
Feature 6 (Ablation)  ──────┼──> Feature 7 (Dashboard)
Features 1-4 (prior)  ──────┘
```

- Features 5 and 6 are independent of each other
- Feature 7 integrates outputs from Features 2, 5, and 6
- None of Features 5-7 modify the `Simulation` trait (use optional separate traits)
- All use factory closures `Fn() -> S` pattern for batch runs

## Implementation Order
1. Feature 5 (Highlights) — independent, high design impact
2. Feature 6 (Ablation) — independent, medium design impact
3. Feature 7 (Dashboard) — last, integrates everything

## Total Estimated Complexity
| Feature | New Files | Modified Files | Lines |
|---------|-----------|----------------|-------|
| 5. Highlights | 1 | 2 | ~535 |
| 6. Ablation | 1 | 2 | ~400 |
| 7. Dashboard | 2 | 3 | ~700 |
| **Total** | **4** | **7** | **~1,635** |
