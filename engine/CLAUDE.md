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
- Games are defined as Rust modules (like `trap_links_demo.rs` or `mycelia.rs`), never as spec files.

## Development Methodology

This engine is purpose-built for the **S-League** minigolf RPG. The game is always defined as a Rust crate — not using the world spec language. Development is driven by game needs: build what the RPG requires, harden what exists, avoid feature sprawl.

### Innovation Games Process

Innovation Games drive this engine's development. Each round follows a structured process with mandatory pre-work, competitive proposals, and changelog publishing.

#### Pre-Work: Codebase Audit (mandatory before each round)
Before starting proposals, spawn agents with different technical backgrounds to:
1. **Rust Engine Expert** — review engine code for dead code, unused imports, API drift, unfixed bugs, test gaps.
2. **Game Developer** — verify demos compile and run against current WASM build.
3. **Documentation Reviewer** — audit all .md files for accuracy against actual source.
4. Fix all issues found, update docs, commit as "pre-work: codebase audit".

#### Innovation Round Steps
1. **Pre-work audit** (see above)
2. **Spawn competing agents** (4 agents) — each proposes features/ideas independently
3. **Theme-driven ideation** — proposals validated against the round's theme
4. **Cross-pollinate** — review agents select best ideas across all competitors
5. **Integrate** — winning features implemented with tests
6. **Demo** — build/update demo games showcasing new features
7. **Changelog publish** — MANDATORY after every round: update CHANGELOG.md AND site/changelog/index.html, commit, push, and deploy

## Demo Game Building Process

When building a demo game:
1. **Read `PROCESS.md`** at repo root for lessons learned, API pitfalls, and patterns from prior builds. Use as helper info — first principles are king.
2. **Research the engine API** before writing code. Read the actual source for components, systems, and rendering functions you plan to use. Don't assume signatures.
3. **Architecture**: Custom Rust crate/module. The game is defined in code, not spec files.
4. **Build iteratively**: Get compilation passing first, then tests, then web integration.
5. **After each build**: Update `PROCESS.md` with new findings, API corrections, and patterns discovered.
