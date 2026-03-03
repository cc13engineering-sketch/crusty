# Engine Conventions — Claude Code reads this automatically

## Rust Patterns
- Use `f64` for ALL math. No `f32` anywhere.
- Destructure `World` at the top of every system function to access multiple stores without borrow conflicts.
- Use `crate::log::{log, warn, error}` for all logging. NEVER import `web_sys` directly.
- Snapshot-then-commit in collision.rs: collect data into Vec, process, write back in separate loop.
- No `unwrap()` in systems. Use `if let Some(...)` or `match`.
- Components: Clone + Debug. No Serialize/Deserialize needed in v1.
- ComponentStore has manual Default impl — no `T: Default` bound.

## Platform Rules
- `console_error_panic_hook` and `web-sys` are WASM-only deps (cfg-gated in Cargo.toml).
- `#[cfg(target_arch = "wasm32")]` guards on panic hook setup in init().
- CLI must compile for native target — never reference wasm-only crates unconditionally.

## File Conventions
- Adding a component: create file in components/, add to components/mod.rs, add store to world.rs, add to World::new/despawn/clear, add SchemaInfo impl.
- Adding a system: create file in systems/, add to systems/mod.rs, add call in engine.rs tick().
- Games implement the `Simulation` trait. The engine owns timing and input application.

## Development Methodology

This is a deterministic simulation platform. Games implement the `Simulation` trait; the engine enforces determinism, fixed timesteps, and seeded RNG. AI agents can drive simulations through the `Policy` trait.

### Key Architecture Decisions
- **One canonical RNG**: `SeededRng` (xorshift64) owned by `Engine`. No other RNG sources in engine-core.
- **Fixed DT**: All simulation-phase systems receive `FIXED_DT`. Variable dt is only for the physics accumulator.
- **State hashing**: `Engine::state_hash()` produces a deterministic u64 independent of rendering.
- **Seeded reset**: `Engine::reset(seed)` is the single entry point for reproducible simulation.
- **InputFrame**: Canonical input representation for replays and policy-driven simulation.
