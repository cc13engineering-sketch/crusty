# Crusty Engine — Consistency & Developer Experience Review Plan

**Date**: 2026-03-03
**Scope**: Code consistency, developer experience, maintainability
**Method**: 5 parallel analysis agents examined structure, consistency, testing, API ergonomics, and code smells

---

## Executive Summary

The codebase is **well-architected** (8.5/10 API ergonomics score) with strong fundamentals: excellent naming conventions, clean module organization, determinism-first design, and comprehensive test coverage (1157+ tests). The issues found are primarily about **consistency gaps** and **maintainability friction** rather than architectural problems.

---

## Findings by Priority

### P0 — High-Impact Consistency Fixes

#### 1. World Component Store Boilerplate (DRY violation)
**File**: `engine/crates/engine-core/src/ecs/world.rs`
**Problem**: Adding a new component requires updating **3 separate locations** with identical boilerplate:
- `World::new()` — 31 manual initializations (lines ~97-130)
- `World::despawn()` — 31 manual `.remove(entity)` calls (lines ~150-181)
- `World::clear()` — 31 manual `.clear()` calls (lines ~195-226)

**Impact**: Every new component creates maintenance burden and risk of missed locations.
**Proposed Fix**: Introduce a `component_stores!` macro that generates all three methods from a single declaration list. This keeps the explicit store-per-field pattern (no dynamic dispatch overhead) while eliminating triple-duplication.

#### 2. SchemaInfo Implementation Duplication
**Files**: All 30 files in `engine/crates/engine-core/src/components/`
**Problem**: Every component has a near-identical `impl SchemaInfo` block producing JSON field metadata. ~30 copy-paste implementations.
**Impact**: Boilerplate-heavy, error-prone when adding components.
**Proposed Fix**: Create a `derive(SchemaInfo)` proc macro or a declarative `schema!` macro to auto-generate implementations from struct field definitions.

#### 3. Error Handling Inconsistency (Violates Own CLAUDE.md Rules)
**Files**: Multiple system files, `physics/ccd.rs`, `systems/ghost_trail.rs`
**Problem**: CLAUDE.md states *"No unwrap() in systems"* but:
- `physics/ccd.rs` has **10 unwrap() calls** in critical collision detection paths
- `systems/ghost_trail.rs` has **5 expect() calls** (`expect("trail")`, `expect("trail should exist")`)
- 223 total `unwrap()` calls across 31 files (many in tests, which is fine)

**Impact**: Panics in production physics code; inconsistency with documented conventions.
**Proposed Fix**:
- Replace all non-test `unwrap()` in systems/physics with `if let Some(...)` or early returns
- Audit and fix the ~15-20 non-test occurrences that violate CLAUDE.md

---

### P1 — Developer Experience Improvements

#### 4. Add Explicit Linting Configuration
**Current State**: No `rustfmt.toml`, no `clippy.toml` — relying on Rust defaults.
**Problem**: Formatting and lint rules are implicit; contributors must guess conventions.
**Proposed Fix**:
- Add `rustfmt.toml` codifying the current style (4-space indent, ~100 char lines)
- Add `clippy.toml` with project-specific settings
- Add a `cargo clippy` step to CI

#### 5. Add CI Test Job (Separate from Deploy)
**Current State**: `.github/workflows/deploy.yml` builds WASM but has no explicit `cargo test` step.
**Problem**: Tests only run locally; no CI gate prevents merging broken code.
**Proposed Fix**: Add a `test` job to the workflow (or a separate `ci.yml`) that runs:
- `cargo test --all`
- `cargo clippy -- -D warnings`
- `cargo fmt --check`
- The existing `rng_lint.sh` script

#### 6. Documentation Coverage Gaps
**Current State**: ~60% of components have doc comments; rest have none. Some enums documented, others not.
**Specific Gaps**:
- Components without module docs: `impulse.rs`, `coroutine.rs`, `signal.rs`, `graph_node.rs`, `collider.rs`, `renderable.rs` (~15 files)
- `EventKind` enum variants have no docs (contrast: `BehaviorMode` variants are documented)
- CLI entry point (`main.rs`) has minimal docs
**Proposed Fix**: Add `/// ...` doc comments to all public types and enum variants in components. Focus on the "what and why" — one sentence per item.

#### 7. Missing Standard Trait Implementations
**Current State**: Clone/Debug on everything (good), but no Display implementations anywhere.
**Problem**: Debugging engine state requires manual formatting; `println!("{}", transform)` doesn't work.
**Proposed Fix**: Add `impl Display` for core types:
- `Transform` → `"(x, y) rot=R scale=S"`
- `Color` → `"#RRGGBB"` or `"rgba(r,g,b,a)"`
- `Entity` → `"Entity(id)"`
- `Camera` → `"Camera(x,y zoom=Z)"`

---

### P2 — Consistency Polish

#### 8. Inconsistent Field Visibility in Input
**File**: `engine/crates/engine-core/src/input.rs` line 14
**Problem**: `drag_threshold: f64` is private while all adjacent fields (`keys_held`, `mouse_x`, `is_dragging`) are `pub`.
**Proposed Fix**: Make `drag_threshold` pub to match the pattern, or add a comment explaining why it's intentionally private.

#### 9. Inconsistent Method Naming for State Advancement
**Problem**: Some modules use `tick()`, others `update()`, others `apply()` for the same conceptual operation (advancing state by one step).
**Examples**: Engine uses `tick()`, systems use function calls, some subsystems use `update()`.
**Proposed Fix**: Audit and standardize. Recommendation: `tick()` for frame-level advancement, `update()` for subsystem state changes, `apply()` for one-shot mutations.

#### 10. Large Files That Could Benefit from Modularization
**Files over 700 lines**:
- `game_flow.rs` (816 lines)
- `tilemap.rs` (781 lines)
- `camera_director.rs` (738 lines)
- `gesture.rs` (737 lines)
**Proposed Fix**: Consider splitting into submodules where natural seams exist. Not urgent — these files are cohesive — but worth reviewing if they continue growing.

---

### P3 — Nice-to-Have Enhancements

#### 11. EntityTemplate Builder Pattern
**File**: `engine/crates/engine-core/src/templates.rs`
**Problem**: `EntityTemplate` has 23 `Option<T>` fields, making construction verbose.
**Proposed Fix**: Add a builder API: `EntityTemplate::builder().with_transform(...).with_collider(...).build()`

#### 12. Color Conversion Ergonomics
**Problem**: `Color::from_hex("#FF0000")` works but `"#FF0000".into()` doesn't.
**Proposed Fix**: Add `impl From<&str> for Color` alongside existing `from_hex()`.

#### 13. Feature Flag Expansion
**Current State**: Only one feature flag (`toml-presets`).
**Proposed Fix**: As the engine grows, consider `headless`, `rendering`, `physics` feature gates to allow slimmer builds for specific use cases.

#### 14. VISION.md is Empty
**File**: `/home/user/crusty/VISION.md` (0 bytes)
**Proposed Fix**: Either populate it with the project vision or remove it to avoid confusion.

---

## What's Already Excellent (No Changes Needed)

- **Naming conventions**: Consistent snake_case/PascalCase throughout
- **Module organization**: Clean filename.rs pattern with explicit re-exports
- **Import style**: Consistent `crate::` absolute paths, `super::` for siblings
- **Code formatting**: Consistent 4-space indent, K&R braces, clean whitespace
- **Determinism architecture**: SeededRng, state hashing, golden replay tests
- **Test coverage**: 1157+ tests, inline unit tests in 67 modules, fuzz testing
- **Dependency hygiene**: Minimal deps (5 direct), all current versions
- **Platform abstraction**: `crate::log` cleanly abstracts WASM vs native
- **Zero unsafe code**: No `unsafe` blocks anywhere
- **Zero TODO/FIXME/HACK**: Clean codebase with no deferred work markers
- **Component design**: Small, focused types — no God structs

---

## Proposed Execution Order

If approved, I'd execute in this order:

1. **P0-3**: Fix error handling violations (unwrap→if-let in systems/physics)
2. **P0-1**: Create `component_stores!` macro for World boilerplate
3. **P1-5**: Add CI test/lint job
4. **P1-4**: Add rustfmt.toml + clippy.toml
5. **P1-6**: Add doc comments to undocumented public types
6. **P0-2**: Create SchemaInfo derive/macro (if time permits — larger effort)
7. **P1-7**: Add Display impls for core types
8. **P2 items**: Polish passes

---

## Questions for Review

1. **Macro approach for P0-1**: Do you prefer a declarative `macro_rules!` approach (simpler, in-crate) or a proc macro (cleaner syntax, separate crate)?
2. **CI scope for P1-5**: Should the test job block deployment, or run as a separate check?
3. **Which P2/P3 items matter most to you?** I can prioritize accordingly.
4. **Are any items here out of scope or unwanted?** Happy to trim the list.
