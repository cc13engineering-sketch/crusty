# Crusty Engine

Deterministic 2D simulation engine in Rust. Compiles to WASM for browser play, runs headless for automated testing and AI-driven optimization.

## Games

**Gravity Pong** — Physics puzzle game. Guide particles into targets using gravity wells, repulsors, black holes, wormholes, and plasma currents. 10 levels with progressive mechanic introduction. Dust motes visualize gravitational field lines.

**Chord Reps** — Music theory ear training with spaced repetition. Identify scale degrees, intervals, chord qualities, roman numerals, and cadences. SM-2 SRS algorithm with WebAudio synthesis. Piano visualization with touch support.

**Demo Ball** — Minimal bouncing ball simulation. Smoke test for the Simulation trait and ECS.

## Architecture

```
engine/
  crates/
    engine-core/     ECS, physics, rendering, headless testing (~12K LOC)
    engine-cli/      15 CLI commands for headless analysis
site/                Static site, game pages, docs
```

**Core design**: Games implement `Simulation { setup, step, render }`. Engine owns timing, input, RNG, and determinism. Software framebuffer renders to WASM shared memory; JS blits to canvas.

**ECS**: 32 component types, 17 systems, fixed execution order. HashMap-backed stores, monotonic entity IDs.

**Physics**: Semi-implicit Euler at 60Hz fixed timestep. CCD (circle vs circle/segment/AABB). Plummer-softened force fields. Spatial grid broadphase.

**Rendering**: CPU framebuffer with SDF-based anti-aliasing. Shapes, bitmap text, particles, post-fx (vignette, scanlines, screen shake, tint). Chord Reps adds a WebGL2 bloom pipeline on the JS side.

**Determinism**: Single `SeededRng` (xorshift64). `Engine::state_hash()` for verification. `Engine::reset(seed)` for reproducible runs.

**Headless infrastructure** (26 modules): Replay recording, golden tests, parameter sweeps, fitness evaluation, hill climbing, death classification, highlight detection, ablation studies, variant branching, dashboard generation.

## Quick Start

```bash
cd engine
cargo test                                              # 1200+ tests
cargo run -p engine-cli -- info                         # engine metadata
cargo run -p engine-cli -- batch --seed-range 0..10     # headless batch
```

WASM build:
```bash
wasm-pack build crates/engine-core --target web --out-dir ../../_pkg -- --no-default-features
```

## CLI Commands

```
record / replay          Record and replay deterministic playthroughs
batch / sweep            Run seeds in bulk, sweep parameter ranges
golden record / check    Regression testing via recorded baselines
deaths / highlights      Classify game-overs, detect interesting moments
ablation                 A/B test mechanic contributions
hill-climb               Coordinate-descent parameter optimization
divergence               Pinpoint frame-level determinism breaks
preset / variants        Manage feel presets and parameter variants
dashboard-data           Generate full analysis JSON
info / schema            Engine metadata and component schema
```

## Documentation

| Document | Purpose |
|----------|---------|
| [engine/ARCHITECTURE.md](engine/ARCHITECTURE.md) | Engine internals, system execution, ECS design |
| [engine/CLAUDE.md](engine/CLAUDE.md) | Coding conventions for AI-assisted development |
| [ENGINE_BOUNDARIES.md](ENGINE_BOUNDARIES.md) | Platform separation rules |
| [REVIEW.md](REVIEW.md) | Expert review with ratings and findings |
| [site/docs/](site/docs/) | Web-hosted docs (getting started, API reference) |

## Design Priorities

Optimizing for: deterministic reproducibility, headless-first testing, AI-assisted iteration, fast development cycles.

Not optimizing for: GPU rendering, editor tooling, network multiplayer, asset pipelines.

## License

Check the repository root for license details.
