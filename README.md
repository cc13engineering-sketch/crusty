# Crusty Engine

Deterministic simulation engine in Rust, designed for AI-driven game development.

---

## What This Is

Crusty is a modular Rust game engine built around:

- **Deterministic simulation** — same seed + same inputs = identical state, always
- **Headless-first execution** — thousands of runs in seconds, no browser needed
- **AI-driven iteration** — structured observation, sweep analysis, and automated optimization
- **The Simulation trait** — clean boundary between engine and game logic

Games implement `setup`, `step`, and `render`. The engine owns timing, input, RNG, and determinism.

---

## Repository Structure

```
engine/
  crates/
    engine-core/     Main engine (ECS, physics, rendering, headless)
    engine-cli/      CLI tool (14 commands)
docs/                Technical documentation
site/                Static site + web demo
```

---

## Quick Start

```bash
cd engine
cargo test              # 1157+ tests
cargo run -p engine-cli -- info
cargo run -p engine-cli -- batch --seed-range 0..10 --frames 600 --turbo
```

---

## Core Architecture

- **ECS**: 32 component types, 21 systems, fixed execution order
- **Physics**: Fixed 60Hz timestep, CCD, spatial grid broadphase
- **Rendering**: Software framebuffer (shapes, text, particles, post-fx)
- **RNG**: Single `SeededRng` (xorshift64) owned by engine
- **State hashing**: `Engine::state_hash()` for determinism verification

### Simulation Trait

```rust
pub trait Simulation {
    fn setup(&mut self, engine: &mut Engine);
    fn step(&mut self, engine: &mut Engine);
    fn render(&self, engine: &mut Engine);
}
```

### Headless Infrastructure (22 modules)

Replay recording, golden tests, parameter sweeps, fitness evaluation, hill climbing, anomaly detection, death classification, divergence replay, interesting moment detection, mechanic ablation, variant branching, dashboard generation.

### CLI Commands

```
record / replay / batch / sweep / golden / deaths / divergence
preset / variants / variant-sweep / highlights / ablation
dashboard-data / info / schema
```

All data-producing commands emit JSON/JSONL.

---

## Documentation

| Document | Contents |
|----------|----------|
| `docs/getting-started.md` | Quick-start tutorial |
| `docs/architecture.md` | Headless testing architecture |
| `docs/engine.md` | Engine technical reference |
| `docs/ai-iteration.md` | AI-driven development guide |
| `docs/api-reference.md` | API reference |
| `engine/ARCHITECTURE.md` | Engine internals (for contributors) |
| `engine/CLAUDE.md` | Conventions for AI code generation |
| `ENGINE_BOUNDARIES.md` | Platform separation rules |
| `RENDERER_FUTURE.md` | Rendering layer roadmap |

---

## Project Status

**What is solid:**

- ECS foundation (32 components, 21 systems)
- Deterministic simulation (seeded RNG, fixed dt, state hashing)
- Simulation trait boundary (InputFrame, Policy, Observation)
- Headless infrastructure (22 modules, 14 CLI commands)
- 1157+ passing tests
- WASM builds for browser deployment

**What is evolving:**

- Validation games beyond DemoBall
- Declarative game definition format
- Cross-platform determinism (native vs WASM)
- Production ergonomics

---

## Design Goals

Optimizing for:

- Deterministic reproducibility
- AI-assisted development workflows
- Fast iteration cycles
- Clear system boundaries

Not optimizing for:

- GPU rendering
- Editor tooling
- Network multiplayer
- Asset pipelines

---

## Contributing

Before making changes:

1. Read `engine/CLAUDE.md` (coding conventions)
2. Read `engine/ARCHITECTURE.md` (system architecture)
3. Read `ENGINE_BOUNDARIES.md` (platform separation)
4. Preserve determinism — use `Engine.rng`, not external RNG
5. All simulation-phase systems use `FIXED_DT`

---

## License

Check the repository root for license details.
