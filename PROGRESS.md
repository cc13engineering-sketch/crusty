# Crusty Engine — Implementation Progress

## Current Phase: Phase 2 — Deterministic Core

## Phase 1 — Clean Slate — COMPLETE

### Phase 1.1: Remove Dead Code — DONE
- [x] Delete `game-concept/DESIGN.md`
- [x] Rewrite `engine/CLAUDE.md` for simulation-platform identity
- [x] Update `engine/ARCHITECTURE.md` overview

### Phase 1.2b: Remove Leaked Game Logic from engine.rs — DONE
- [x] Remove `game_over`, `spawn_timer`, `fire_cooldown` fields from Engine struct
- [x] Remove `run_spawners()`, `spawn_wave()`, `try_fire_bullet()` methods
- [x] Remove `check_game_over()`, `render_hud()` methods
- [x] Remove `random_edge_spawn()` free function
- [x] Remove calls in `tick()`: `self.run_spawners(dt)`, `self.check_game_over()`, `self.render_hud()`
- [x] Update tick() doc comments
- [x] All 1036 tests pass

### Phase 1.2: Remove Legacy Function-Pointer APIs — IN PROGRESS
- [ ] Delete `ShotBuilder` (headless/shot_builder.rs)
- [ ] Note: `HeadlessRunner`, `GameScenario`, `ScheduledAction`, `record_replay` use function pointers. These will be replaced by the Simulation trait in Phase 3. For now, keep them — they are still used by surviving headless modules. Will be rewritten in Phase 3/4.

### Phase 1.3: Consolidate RNG — DONE
- [x] Created `rng.rs` module with `SeededRng` (xorshift64) + tests
- [x] Added `pub rng: SeededRng` field to Engine, seeded with 42
- [x] Updated `procedural_gen.rs` to re-export from `rng.rs`
- [x] Deleted `SimpleRng` from `rendering/particles.rs`
- [x] Updated `particles.rs` and `starfield.rs` to use `SeededRng`
- [x] All 1037 tests pass

## Phase 1 COMPLETE. Moving to Phase 2.

## Phase 2 — Deterministic Core — IN PROGRESS

### Phase 2.1: State Hashing — DONE
- [x] Added `PartialOrd, Ord` derives to Entity for deterministic sorted iteration
- [x] Implemented `Engine::state_hash() -> u64` using FNV-1a
- [x] Hashes: entity count, transforms (x,y,rotation,scale), rigidbodies (vx,vy,mass), global_state (sorted by key with type tags), frame counter, rng state
- [x] Excludes rendering state (framebuffer, particles, starfield) — simulation truth only
- [x] 5 tests: deterministic, changes-after-tick, differs-with-state, differs-with-entities, sensitive-to-rng
- [x] All 1042 tests pass

### Phase 2.2: Fixed DT Everywhere
- [ ] All simulation-phase systems receive FIXED_DT instead of variable dt
- [ ] Affected: state_machine, coroutine, sprite_animator, behavior, tween, flash, waypoint
- [ ] Also: EnvironmentClock::tick, FlowNetwork::solve

### Phase 2.3: Engine::reset(seed)
- [ ] Replace `reset_game_state()` with `reset(seed: u64)`
- [ ] Clear all state, reseed rng, reset frame/time to 0

## Phases 3-8 — Not Yet Started
See main implementation plan for details.

## Risk Mitigation Follow-Ups (after Definition of Done)
- Follow-Up A: Validation Game
- Follow-Up B: HashMap Iteration Audit
- Follow-Up C: Cross-Platform Determinism
- Follow-Up D: Scope Boundary Enforcement (FUTURE.md)
