# Crusty Engine — Implementation Progress

## Current Phase: Phase 8 — Documentation (remaining)

## Phase 1 — Clean Slate — COMPLETE

### Phase 1.1: Remove Dead Code — DONE
- [x] Delete `game-concept/DESIGN.md`
- [x] Rewrite `engine/CLAUDE.md` for simulation-platform identity
- [x] Update `engine/ARCHITECTURE.md` overview
- [x] Remove S-League game (archived in prior commits)

### Phase 1.2b: Remove Leaked Game Logic from engine.rs — DONE
- [x] Remove `game_over`, `spawn_timer`, `fire_cooldown` fields from Engine struct
- [x] Remove `run_spawners()`, `spawn_wave()`, `try_fire_bullet()` methods
- [x] Remove `check_game_over()`, `render_hud()` methods
- [x] Remove `random_edge_spawn()` free function
- [x] Remove calls in `tick()`: `self.run_spawners(dt)`, `self.check_game_over()`, `self.render_hud()`
- [x] Update tick() doc comments
- [x] All 1036 tests pass

### Phase 1.3: Consolidate RNG — DONE
- [x] Created `rng.rs` module with `SeededRng` (xorshift64) + tests
- [x] Added `pub rng: SeededRng` field to Engine, seeded with 42
- [x] Updated `procedural_gen.rs` to re-export from `rng.rs`
- [x] Deleted `SimpleRng` from `rendering/particles.rs`
- [x] Updated `particles.rs` and `starfield.rs` to use `SeededRng`
- [x] All 1037 tests pass

## Phase 2 — Deterministic Core — COMPLETE

### Phase 2.1: State Hashing — DONE
- [x] Added `PartialOrd, Ord` derives to Entity for deterministic sorted iteration
- [x] Implemented `Engine::state_hash() -> u64` using FNV-1a
- [x] Hashes: entity count, transforms (x,y,rotation,scale), rigidbodies (vx,vy,mass), global_state (sorted by key with type tags), frame counter, rng state
- [x] Excludes rendering state (framebuffer, particles, starfield) — simulation truth only
- [x] 5 tests: deterministic, changes-after-tick, differs-with-state, differs-with-entities, sensitive-to-rng
- [x] All 1042 tests pass

### Phase 2.2: Fixed DT Everywhere — DONE
- [x] Changed all simulation-phase systems from variable `dt` to `FIXED_DT`
- [x] Rendering/UI systems keep variable dt: particles, transition, dialogue, camera, screen_fx, post_fx
- [x] Updated SystemPhase and tick() doc comments to reflect fixed-dt simulation
- [x] All 1042 tests pass

### Phase 2.3: Engine::reset(seed) — DONE
- [x] Replaced `reset_game_state()` with `reset(seed: u64)`
- [x] Clears all world state, subsystems, reseeds RNG, resets frame/time/accumulator
- [x] 7 new tests
- [x] All 1049 tests pass

## Phase 3 — Simulation Trait and InputFrame — COMPLETE

### Phase 3.1-3.2: InputFrame and Simulation trait — DONE
- [x] `InputFrame` struct with keys_pressed, keys_released, keys_held, pointer events
- [x] `Engine::apply_input(&mut self, input: &InputFrame)` implemented
- [x] `Simulation` trait defined (setup, step, render)

### Phase 3.3: HeadlessRunner rewrite — DONE
- [x] HeadlessRunner rewritten with `run_sim`, `run_sim_frames`, `run_with_policy`
- [x] RunConfig and SimResult with state hash capture

### Phase 3.4: demo_ball minimal game — DONE
- [x] `DemoBall` implements `Simulation`
- [x] Ball bouncing, tap-to-launch, score tracking
- [x] 5 tests: deterministic, different seeds diverge, tap launches, turbo mode, state hash capture

## Phase 4 — Replay and Golden Tests — COMPLETE

### Phase 4.1: PlaythroughFile — DONE
- [x] Serializable replay format with seed, inputs, hashes, metadata

### Phase 4.2: Clean up legacy — DONE
- [x] Delete ShotBuilder, clean up action_gen

### Phase 4.3: Golden replay test CI gate — DONE
- [x] Golden test stored and verified in CI

## Phase 5 — Observation and Policy — COMPLETE

- [x] Observation struct and `Engine::observe()`
- [x] Policy trait with NullPolicy, RandomPolicy, ScriptedPolicy
- [x] Policy-driven simulation runner

## Phase 6 — CLI as Control Surface — COMPLETE

- [x] `engine-cli record`, `replay`, `batch`, `sweep`, `golden`, `info` commands
- [x] JSONL output format

## Phase 7 — Determinism Hardening — COMPLETE

- [x] Fuzz tests for determinism
- [x] RNG lint in CI

## Phase 8 — Documentation — IN PROGRESS

- [ ] Update site for new architecture (bouncing ball demo, remove S-League references)
- [ ] Deploy workflow fixed for GitHub Pages

## GitHub Pages Deployment — IN PROGRESS

- [x] WASM bindings for demo_ball (`setup_demo_ball` + sim step/render in tick)
- [x] `site/demo-ball/index.html` — interactive bouncing ball demo
- [x] `site/index.html` — updated landing page (Crusty Engine, no S-League)
- [x] `deploy.yml` — fixed: removed s-league references, added demo-ball

## Risk Mitigation Follow-Ups (after Definition of Done)
- Follow-Up A: Validation Game
- Follow-Up B: HashMap Iteration Audit
- Follow-Up C: Cross-Platform Determinism
- Follow-Up D: Scope Boundary Enforcement (FUTURE.md)
