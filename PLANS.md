# Crusty Engine — Implementation Plans

This document contains the two implementation plans for transforming Crusty from a game engine with no game into a deterministic simulation platform that AI agents can drive.

---

# Plan 1: Main Implementation Plan (Phases 1–8)

## Context

Crusty is a 38K-line Rust game engine with a mature ECS, 21 systems, 32 components, a software renderer, and a sophisticated headless testing layer (18 modules). The demo game (`sleague.rs`) has been archived to a separate branch. There are no users and no working games on the engine yet. Backward compatibility is not a concern — we can break any API freely.

This plan takes the engine from "proven loop with no game on it" to "deterministic simulation platform that AI agents can drive."

-----

## Strategic Framing

Crusty's interesting problem is the engine-as-platform: a deterministic simulation core that AI agents can explore, diagnose, and iterate on at high throughput. The future vision — `engine-cli sweep`, `engine-cli analyze`, `engine-cli sandbox` — requires three things the engine doesn't fully have yet:

1. **Genuine determinism** — same seed + same inputs = identical state, always, everywhere
1. **A formal simulation boundary** — games implement a trait, the engine enforces the contract
1. **Throughput tooling** — batch runs, replays, golden tests, all driven from the CLI

The headless infrastructure is surprisingly mature (replays, golden tests, sweeps, anomaly detection, hill climbing, fitness evaluation) but it was built around function pointers and in-memory-only data. This plan formalizes what exists, fills the real gaps, and cuts everything that no longer serves the mission.

-----

## Phase 1 — Clean Slate

With sleague archived and no backward compatibility concerns, start by stripping the codebase down to the engine platform.

### 1.1 Remove Dead Code

1. Delete `sleague.rs` from engine-core (if not already done)
1. Delete all `sleague_*` WASM bindings from `lib.rs`
1. Delete the `simulate` subcommand from `engine-cli` (it hardcodes sleague)
1. Delete `site/s-league/` and `game-concept/DESIGN.md`
1. Delete sleague-specific tests from `headless/tests.rs`
1. Audit remaining `headless/tests.rs` — any test that calls sleague functions is dead. Strip it. Keep pure engine tests.

### 1.2 Remove Legacy Patterns

With no users to break:

1. Delete the function-pointer API on `HeadlessRunner` (`run`, `run_with_frame_cb`). These will be replaced by the `Simulation` trait.
1. Delete `GameScenario`'s function-pointer fields. Same reason.
1. Delete `record_replay`'s function-pointer signature.
1. Delete `ShotBuilder` (sleague-specific action builder).
1. Replace `ScheduledAction` entirely with `InputFrame` (Phase 2).
1. Delete `run_spawners`, `spawn_wave`, `try_fire_bullet`, `random_edge_spawn` from `engine.rs` — these are game logic that leaked into the engine.

### 1.3 Consolidate RNG

The codebase has three RNG implementations:

|Location                |Type                    |Verdict                                  |
|------------------------|------------------------|-----------------------------------------|
|`procedural_gen.rs`     |`SeededRng` (xorshift64)|Keep — promote to engine-level           |
|`headless/action_gen.rs`|`LcgRng`                |Keep — test infra, intentionally separate|
|`rendering/particles.rs`|`SimpleRng`             |Delete — replace with `SeededRng`        |

Actions:

1. Move `SeededRng` out of `procedural_gen.rs` into a new top-level `rng.rs` module
1. Add `pub rng: SeededRng` to `Engine`
1. Seed it in `Engine::new()` with a default seed (e.g., 42)
1. Delete `SimpleRng` from `rendering/particles.rs` — particles and starfield take `&mut SeededRng` from the engine
1. Add a CI grep (or clippy config) that fails on `thread_rng`, `rand::random`, or the `sin(`-based RNG pattern anywhere in engine-core

**Deliverable:** `main` builds, all surviving tests pass, no dead code, one canonical RNG.

-----

## Phase 2 — Deterministic Core

### 2.1 State Hashing

**Problem:** `framebuffer_hash` hashes rendered pixels. A color tweak breaks the hash even though simulation state is identical. There's no way to verify determinism independently of rendering.

**Solution:**

1. Implement `fn state_hash(&self) -> u64` on `Engine`
1. Hash: entity count, all transform (x, y, rotation), all rigidbody (vx, vy), `global_state` contents, `frame` counter, `rng.state`
1. Use FNV-1a (same algorithm as `framebuffer_hash`) for consistency
1. Keep `framebuffer_hash` as a separate visual regression tool — it's still useful, just not for determinism verification

**Acceptance:** Two runs with the same seed + inputs produce identical `state_hash` at every frame. Changing a render color doesn't change `state_hash`. Changing a force value does.

### 2.2 Fixed DT Everywhere

**Problem:** The simulation phase runs at variable dt. Camera smoothing uses raw dt. Same inputs at different frame rates produce different results.

**Solution:**

1. All simulation-phase systems receive `FIXED_DT` instead of variable `dt`. Affected systems: `state_machine::run`, `coroutine::run`, `sprite_animator::run`, `behavior::run`, `tween::run`, `flash::run`, `waypoint::run`
1. Camera update uses accumulated fixed-step time
1. `EnvironmentClock::tick` and `FlowNetwork::solve` also get `FIXED_DT`
1. The variable `dt` from the host is used *only* for the physics accumulator — which already works correctly

**Acceptance:** Simulation produces identical state hashes regardless of how dt is distributed across calls. One call with dt=0.1 and six calls with dt=1/60 produce the same final state.

### 2.3 Engine::reset(seed)

Replace the current `Engine::reset()` (no args) with `Engine::reset(seed: u64)`:

1. Clear all world state (already done)
1. Clear all subsystems (already done)
1. Reseed `self.rng` with the given seed
1. Reset `self.frame` to 0 and `self.time` to 0.0

This is the single entry point for reproducible simulation. Every headless run starts here.

**Deliverable:** Deterministic engine with state hashing, fixed dt, and seeded reset.

-----

## Phase 3 — Simulation Trait and InputFrame

### 3.1 InputFrame

```rust
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InputFrame {
    pub keys_pressed: Vec<String>,
    pub keys_released: Vec<String>,
    pub keys_held: Vec<String>,
    pub pointer: Option<(f64, f64)>,
    pub pointer_down: Option<(f64, f64)>,
    pub pointer_up: Option<(f64, f64)>,
}
```

Add `Engine::apply_input(&mut self, input: &InputFrame)` that maps an `InputFrame` into the existing `Input` struct's internal state.

### 3.2 Simulation Trait

```rust
pub trait Simulation {
    /// Reset game state for a new run. Engine is already reset and seeded.
    fn setup(&mut self, engine: &mut Engine);

    /// Advance one frame of game logic.
    fn step(&mut self, engine: &mut Engine);

    /// Render the current state into the engine's framebuffer.
    fn render(&self, engine: &mut Engine);
}
```

No `dt` argument on `step` — the simulation always advances by one fixed tick. The engine owns timing.

No `input` argument — the engine has already applied the `InputFrame` before calling `step`. The game reads `engine.input` as it always did.

### 3.3 HeadlessRunner Rewrite

Delete the old function-pointer API entirely. The new runner:

```rust
pub struct HeadlessRunner {
    pub engine: Engine,
}

impl HeadlessRunner {
    pub fn new(width: u32, height: u32) -> Self;

    pub fn run<S: Simulation>(
        &mut self,
        sim: &mut S,
        seed: u64,
        inputs: &[InputFrame],
        config: RunConfig,
    ) -> SimResult;

    pub fn run_with_policy<S: Simulation, P: Policy>(
        &mut self,
        sim: &mut S,
        policy: &mut P,
        seed: u64,
        frames: u64,
        config: RunConfig,
    ) -> SimResult;
}

pub struct RunConfig {
    pub turbo: bool,
    pub capture_state_hashes: bool,
}
```

`SimResult` gains `state_hash: u64` (final) and optional `state_hashes: Vec<u64>` (per-frame).

### 3.4 Minimal Demo Game

A trivial game that validates the full pipeline:

```rust
// demo_ball.rs — ~150 lines
// A ball on a bounded surface. Tap to launch. Score = distance. Bounces off walls.
pub struct DemoBall { ... }
impl Simulation for DemoBall { ... }
```

This is the engine's built-in test game. It replaces sleague's role as "the thing that proves the engine works" with something two orders of magnitude simpler.

**Deliverable:** Simulation trait defined, HeadlessRunner rewritten, demo_ball passes determinism tests.

-----

## Phase 4 — Replay and Golden Tests

### 4.1 PlaythroughFile

```rust
#[derive(Serialize, Deserialize)]
pub struct PlaythroughFile {
    pub engine_version: String,
    pub seed: u64,
    pub inputs: Vec<InputFrame>,
    pub frame_count: u64,
    pub final_state_hash: u64,
    pub final_fb_hash: u64,
    pub metadata: HashMap<String, String>,
}
```

Write: `serde_json::to_writer` to JSONL.
Read: `serde_json::from_reader`.
Verify: replay inputs with same seed, compare `state_hash`. Hard error on mismatch.

### 4.2 Retrofit Headless Modules

The existing headless modules (`replay`, `golden`, `sweep`, `compare`, `anomaly`, `fitness`, `harness`, `strategy`, `experiment`, `hill_climb`) are solid but built around the old function-pointer API. Rewrite them against `Simulation`:

1. `Replay` stores `Vec<InputFrame>` instead of `Vec<ScheduledAction>` + `HashMap<String, f64>`
1. `GoldenTest` takes `&mut impl Simulation` instead of four function pointers
1. `run_sweep` takes `Simulation + Clone` and runs copies
1. `TestHarness` and `Strategy` updated similarly
1. All modules gain serde support for disk persistence

This is mechanical refactoring. The logic and algorithms stay the same.

### 4.3 Golden Test CI Gate

1. Store one golden playthrough in `tests/golden/baseline.jsonl` (recorded from `demo_ball`)
1. Add `tests/golden_replay.rs`:
- Load the file
- Replay with same seed + inputs
- Assert `state_hash` matches
- Hard fail on mismatch
1. This runs in CI on every commit

**Deliverable:** Replays persist to disk, golden test gates CI, all headless modules use the new API.

-----

## Phase 5 — Observation and Policy

### 5.1 Observation

```rust
pub struct Observation<'a> {
    pub frame: u64,
    pub state_hash: u64,
    pub game_state: &'a GameState,
    pub entity_count: usize,
    pub framebuffer: Option<&'a [u8]>,
}
```

Zero allocation. Borrows from engine. `Engine::observe(&self) -> Observation`.

### 5.2 Policy Trait

```rust
pub trait Policy {
    fn next_input(&mut self, obs: &Observation) -> InputFrame;
}
```

Built-in policies:

- `NullPolicy` — empty InputFrame every tick
- `RandomPolicy` — random pointer/key events from a separate seeded RNG (not the engine's — policy randomness must not affect simulation determinism when replayed)
- `ScriptedPolicy` — replay a fixed `Vec<InputFrame>`

**Deliverable:** Agents can drive simulations through the Policy trait. The engine records what the policy does for replay.

-----

## Phase 6 — CLI as Control Surface

### 6.1 Command Structure

```
engine-cli record    --seed 42 --frames 600 --out playthrough.jsonl
engine-cli replay    playthrough.jsonl --verify
engine-cli batch     --seed-range 0..10000 --frames 600 --turbo --out results.jsonl
engine-cli sweep     --policy random --seed-range 0..1000 --frames 600 --turbo --out sweep.jsonl
engine-cli golden record  --seed 42 --frames 600 --out golden/baseline.jsonl
engine-cli golden check   golden/baseline.jsonl
engine-cli info           # engine version, module count, feature flags
engine-cli schema         # (already exists) generate JSON schema
```

### 6.2 Design Principle

Every CLI command is a thin wrapper around a library function. The CLI does argument parsing, file I/O, and formatting. The real work lives in engine-core, testable via `cargo test` without the CLI.

### 6.3 Output Format

All data-producing commands emit JSONL by default. Human-readable summaries via `--pretty`. Downstream consumers (jq, Python, future analysis agents) read JSONL natively.

**Deliverable:** The CLI is the primary interface for engine interaction.

-----

## Phase 7 — Determinism Hardening (Ongoing)

Not a discrete phase. Runs continuously alongside all other work.

### CI Checks

1. **Golden replay gate** — blocks merge on state hash mismatch
1. **RNG lint** — CI grep fails on forbidden RNG patterns in engine-core
1. **Determinism fuzz** — nightly: 100 random seeds × 2 runs each, assert identical state hashes
1. **Platform parity** — when WASM builds are ready, run golden test both native and WASM, compare

### Debug Tooling

```
engine-cli replay playthrough.jsonl --trace-determinism
```

Per-frame output: frame number, state hash, rng state, entity count. On divergence, dump the first divergent frame's full state.

-----

## Phase 8 — Documentation

### Rewrite

- `README.md` — engine-as-platform, no game references, how to write a Simulation
- `ARCHITECTURE.md` — add Simulation trait, InputFrame, Policy, state hashing
- `ENGINE_BOUNDARIES.md` — add: "Games implement the Simulation trait. Engine owns timing, input application, and determinism."

### New

- `docs/writing-a-game.md` — implement Simulation, wire to CLI, run headless, write golden tests
- `docs/determinism-guide.md` — rules: use engine RNG, no wall clock, no HashMap iteration for logic, fixed dt only
- `docs/replay-format.md` — PlaythroughFile schema, versioning, verification
- `docs/cli-reference.md` — all commands, all flags, examples

-----

## Phase 9 — Toward the Vision (Future)

After Phases 1–8, the minimal autonomous loop is achievable:

```
engine-cli sweep → analyze output → propose changes → engine-cli batch → compare
```

### Likely next steps

- **Analysis commands** — `engine-cli analyze difficulty`, `analyze softlocks`. JSONL post-processing. Could start as standalone Python scripts, promote to CLI when patterns stabilize.
- **Curiosity policy** — maximizes state-space coverage. Requires defining "novelty" over Observations.
- **Design brief** — structured file (TOML/YAML) defining success criteria. Games ship with one. Agents optimize against it.
- **Sandbox mode** — git branch + batch run + compare. Orchestration, not architecture.

### Needs more design

- **`engine-cli agent propose-fixes`** — the interface between analysis and code changes is where complexity lives. Do the loop manually 20 times before automating.
- **Nightly evolution** — each piece (generate → evaluate → rank) must work independently first.
- **Content generation** — game-specific, not engine-level. Lives in the game's crate.

### Out of scope

Multi-agent orchestration. ML model integration. Asset pipelines. ECS archetype rewrite. Network sync. Editor tooling.

-----

## Execution Summary

|Phase|What                                                           |Depends On|Effort  |
|-----|---------------------------------------------------------------|----------|--------|
|1    |Clean slate (strip dead code, consolidate RNG)                 |Nothing   |2–3 days|
|2    |Deterministic core (state hash, fixed dt, seeded reset)        |Phase 1   |3–4 days|
|3    |Simulation trait, InputFrame, HeadlessRunner rewrite, demo_ball|Phase 2   |4–5 days|
|4    |Replay serialization, headless module retrofit, golden CI gate |Phase 3   |3–4 days|
|5    |Observation, Policy trait, policy-driven runner                |Phase 3   |3–4 days|
|6    |CLI buildout                                                   |Phases 4–5|2–3 days|
|7    |Determinism hardening                                          |Phase 2   |Ongoing |
|8    |Documentation                                                  |Phases 1–6|2–3 days|

**Total: ~4 weeks of focused work.** First two weeks are the critical path (Phases 1–3). Everything after is incremental. Phases 4 and 5 can run in parallel.

-----

## Definition of Done

- [ ] No dead code from sleague or legacy function-pointer APIs
- [ ] One canonical `SeededRng` owned by engine, no other RNG sources in engine-core
- [ ] `Engine::state_hash()` is deterministic and rendering-independent
- [ ] All simulation-phase systems use fixed dt
- [ ] `Engine::reset(seed)` is the single entry point for reproducible simulation
- [ ] `Simulation` trait defined, `demo_ball` implements it
- [ ] `InputFrame` is the canonical input representation
- [ ] `HeadlessRunner` consumes `Simulation` and `Policy`
- [ ] Replays serialize to/from JSONL with hash verification
- [ ] Golden replay test gates CI
- [ ] Turbo mode skips rendering, ≥5x faster
- [ ] Batch runner executes 10,000 runs unattended
- [ ] `Policy` trait with random and scripted implementations
- [ ] CLI exposes record, replay, batch, sweep, golden commands
- [ ] Documentation reflects the new architecture

---

# Plan 2: Risk Mitigation Follow-Up Plan

## Context

The main implementation plan (Phases 1–8) is in progress. This document defines work to be done after the Definition of Done checklist is complete. It addresses four risks identified during planning that the main plan doesn't fully cover.

Do not start this work until the main plan's Definition of Done is 100% complete.

-----

## Follow-Up A — Validation Game

### Problem

`demo_ball` validates the pipeline but doesn't stress it. A ball bouncing off walls exercises physics and input, but not: multiple interacting entity types, meaningful branching game state, RNG-dependent outcomes, or win/lose conditions an agent could optimize against.

### What to Build

A small but real game. 500–1000 lines. Best candidate: **Gravity puzzle** — launch projectiles to hit targets on a 2D field with gravity wells. Multiple levels via seed. Win = all targets hit under par.

### What It Must Do

1. Implement `Simulation` without modifications to the trait
1. Run headless in turbo mode at ≥5x realtime
1. Produce meaningfully different outcomes for different policy inputs
1. Have at least one metric an agent could optimize
1. Pass determinism: same seed + same inputs = identical state hashes
1. Have a golden test stored in CI

-----

## Follow-Up B — HashMap Iteration Audit

### Problem

Rust's `HashMap` uses randomized hashing. If any game logic iterates a `HashMap` and the iteration order affects behavior, the simulation is nondeterministic.

### Recommendation

Switch component stores from `HashMap<Entity, T>` to `BTreeMap<Entity, T>` for deterministic iteration order. One-line change per store. Negligible performance impact at crusty's scale.

### Verification

Determinism fuzz test (100 seeds × 2 runs, compare hashes) passes 10 consecutive runs.

-----

## Follow-Up C — Cross-Platform Determinism (Native vs WASM)

### Problem

Native and WASM builds may produce different floating-point results. Don't assume the problem exists — prove it first by running golden tests on both platforms and comparing state hashes.

-----

## Follow-Up D — Scope Boundary Enforcement

### Rules

1. No Phase 9 work until Definition of Done is 100% complete
1. No abstraction without two concrete users
1. Scripts before commands
1. Manual before automatic
1. Track what's deferred in `FUTURE.md`

-----

## Execution Order

|Follow-Up            |When                                   |Effort                        |
|---------------------|---------------------------------------|------------------------------|
|D (scope enforcement)|Start immediately                      |Zero — it's discipline        |
|A (validation game)  |First thing after Definition of Done   |1 week                        |
|B (HashMap audit)    |During or immediately after Follow-Up A|2–3 days                      |
|C (cross-platform)   |When WASM builds are restored          |2–5 days depending on findings|
