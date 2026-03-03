# Implementation Research: Features 3-4

## Feature 3: Divergence Replay (~625 lines, 1 new file + 2 modified)

### New Files
- `engine/crates/engine-core/src/headless/divergence.rs` (~350 lines) — `DivergenceReport`, `DivergenceContext`, `ContextFrame`, `SweepDivergenceReport`, `SeedDivergence`, `DivergenceClip`, `compare_playthroughs()`, `compare_sweep_results()`, `generate_divergence_clip()`

### Modified Files
- `headless/mod.rs` — Register divergence module
- CLI `main.rs` — Add `divergence` subcommand with `files` and `sweep` sub-commands

### Core Algorithm: `compare_playthroughs`
1. Check if both PlaythroughFiles have non-empty `state_hashes`
2. Iterate `min(a.len, b.len)` frames, comparing state hashes
3. Find first frame `i` where `a.state_hashes[i] != b.state_hashes[i]`
4. Build `DivergenceContext` window `[max(0, i-radius), min(len, i+radius)]`
5. Count total divergent frames, package into `DivergenceReport`

### Core Algorithm: `compare_sweep_results`
1. Parse both JSONL files into `Vec<SweepEntry>`
2. Build `HashMap<u64, SweepEntry>` keyed by seed for file B
3. For each entry in A, look up matching seed in B, compute `|outcome_a - outcome_b|`
4. Sort by delta descending
5. Compute summary stats (mean, max, count with divergence)

### Core Algorithm: `generate_divergence_clip`
1. Create HeadlessRunner, reset with playthrough's seed
2. Run simulation frame by frame
3. Only capture full state snapshots in `[center - radius, center + radius]`
4. Return `DivergenceClip` with captured snapshots

### CLI Commands
```
engine-cli divergence files <A.json> <B.json> [--context N] [--json]
engine-cli divergence sweep <A.jsonl> <B.jsonl> [--key score] [--top N] [--json]
```

### Key Design Decisions
1. **Cheap because state hashing already exists** — comparing two replays is comparing two sequences of u64
2. **Builds on PlaythroughFile** with `state_hashes` field (already recorded)
3. **Clip generation replays from scratch** — leverages determinism, no snapshot serialization needed

### Testing Strategy (8-10 tests, ~150 lines)
- Identical playthroughs: no divergence
- Different seeds: diverge at frame 0-1
- Different inputs at frame 10: diverge around frame 10
- Context window correct size
- Sweep comparison ranks by delta
- Clip captures correct frame range
- Report JSON roundtrip
- Empty state_hashes fallback to final hash

---

## Feature 4: Feel Presets (~900 lines, 1 new file + 4 modified)

### New Files
- `engine/crates/engine-core/src/feel_preset.rs` (~500 lines) — `FeelPreset`, `FeelPresetLibrary`, 6 built-in presets, TOML/JSON serialization

### Modified Files
- `lib.rs` — Add `pub mod feel_preset`
- `engine-core/Cargo.toml` — Add `toml = "0.8"` dependency
- `engine-cli/Cargo.toml` — Add `toml = "0.8"` dependency
- CLI `main.rs` — Add `preset` subcommand with 5 sub-commands (~200 lines)

### Built-in Presets (6 presets)
1. **tight_platformer** — high gravity (980), high acceleration (2000), high friction (0.95)
2. **floaty_astronaut** — low gravity (60), low friction (0.02), high air control (1.0)
3. **heavy_tank** — high mass (10), slow turn (1.5), low acceleration (300)
4. **snappy_cursor** — zero gravity, instant acceleration (10000), high damping (0.99)
5. **underwater** — high drag (0.3), buoyancy (-150), low max speed (100)
6. **ice_skating** — very low friction (0.01), low damping (0.001), slow turning (0.5)

### Parameter Convention
- Dotted key names: `physics.gravity`, `player.max_speed`, etc.
- Applied via `engine.global_state.set_f64()` — completely game-agnostic
- Games read from `global_state` during step() to configure physics

### Key Design Decisions
1. **Presets use `global_state`, same as Variant Branching** — unified mechanism
2. **BTreeMap<String, f64>** for deterministic iteration
3. **TOML format** for human-authored presets (new `toml` crate dependency)
4. **Bridge to existing sweep infra** via `to_sweep_configs()` method
5. **Bridge to hill climbing** via `to_param_ranges(margin)` method
6. **Export workflow**: apply preset → tune → `export_from_state()` → save as new preset

### CLI Commands
```
engine-cli preset list [--json] [--category <cat>]
engine-cli preset show <name>
engine-cli preset apply <name> [--frames N] [--seed N] [--override key=val]
engine-cli preset export [--name N] [--keys K1,K2,...] [--format toml|json] [--out FILE]
engine-cli preset sweep [--frames N] [--seed-range S..E] [--key score]
```

### TOML File Format Example
```toml
name = "my_custom_platformer"
description = "Tuned for my specific game"
category = "platformer"

[params]
"physics.gravity" = 850.0
"physics.restitution" = 0.05
"player.max_speed" = 280.0
```

### Testing Strategy (12-14 tests, ~200 lines)
- Built-in library has all 6 presets
- Apply sets global_state correctly
- Apply with overrides works
- TOML roundtrip
- JSON roundtrip
- Export from state captures keys
- to_sweep_configs produces one per preset
- Merge overrides values correctly
- to_param_ranges centered on values
- Load TOML adds to library
- Different presets produce different simulations (smoke test)

---

## Cross-Feature Integration (Features 3 + 4)

These compose through the filesystem — no additional code needed:

1. **Divergence + Presets**: Apply two presets, record both, run divergence to see where physics feel caused different outcomes
2. **Preset Sweep + Divergence**: Sweep all presets, find which causes most divergent outcome from baseline

### Workflow Example
```bash
engine-cli preset apply tight_platformer --out run_a.json
engine-cli preset apply floaty_astronaut --out run_b.json
engine-cli divergence files run_a.json run_b.json
```

---

## Implementation Order Recommendation
1. **Feature 4 first** (Feel Presets) — self-contained, adds `toml` dependency cleanly, immediately useful for experimentation
2. **Feature 3 second** (Divergence Replay) — benefits from having interesting runs to compare; with presets available, compelling use case immediately

## Dependencies
- Feature 3: No new crate dependencies. Relies on existing PlaythroughFile, RunConfig.capture_state_hashes
- Feature 4: Requires `toml = "0.8"` crate added to Cargo.toml files
- Neither depends on the other
- Both integrate with existing sweep infrastructure
