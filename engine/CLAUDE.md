# Engine Conventions — Claude Code reads this automatically

## When modifying or adding engine crate modules

- always make sure there is an up to date "AI-INSTRUCTIONS" labelled comment at the top of the .mod file. This must be up to date because it allows our assistants to more quickly reason about existing code.

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

## Rendering Rules
- All visual primitives MUST be anti-aliased (1px feather on edges). No hard-edged shapes.
- Use `fill_circle` (AA), `fill_tapered_trail`, and `fill_triangle` (AA) from `shapes.rs`.
- Trails/streaks use `fill_tapered_trail` (single-pass distance-field polyline) — never raw `draw_line`.
- Glow effects: render a wider, low-alpha pass underneath the bright core pass.

## Pokemon Generation 2 Source of Truth
Whenever we need to lookup, verify, or research pokemon generation 2 related data, refer to `<crusty>/data`. Ignore `<crusty>/data/_pipeline`.

## Pokemon Version 2 Rewrite
If you are an agent working on them rewrite (engine/crates/engine-core/src/pokemonv2) note the following important rule: you can reference the previous pokemon version's code (engine/crates/engine-core/src/pokemon) - but know this - we'd prefer to work from first principles - using our new sprints to guide our architecture, patterns, compiler usage, abstractions, etc, etc, etc. Study the old version to help make better decisions. But do not blindly follow your previous patterns - only where it makes sense. 

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

## Special Commands

### `ship <game-name>`

Build a self-contained, static-host-ready deployment folder. Does NOT touch `site/`, `_site/`, or any existing infrastructure.

**Steps:**

1. **Resolve the game.** Match `<game-name>` (case-insensitive) to a directory under `site/` containing `index.html`.

2. **Build WASM.** From `engine/`:
   ```bash
   wasm-pack build crates/engine-core --target web --out-dir "$ROOT/_pkg" -- --no-default-features
   ```
   Skip if the user says to skip or a recent build exists. When in doubt, rebuild.

3. **Determine version.** The user specifies one of: `patch`, `minor`, `major`, or `canary`. If not specified, ask. Read the current version from `engine/crates/engine-core/Cargo.toml` and increment accordingly (`canary` appends `-canary.<git-short-hash>` without bumping the number). Update `Cargo.toml` with the new version. Deployment folder: `deployments/<game-name>-<version>/`.

4. **Assemble the folder.** Copy into `deployments/<game-name>-<version>/`:
   - `site/<game-name>/index.html` → root
   - `_pkg/engine_core.js` and `engine_core_bg.wasm` → `pkg/`
   - Any parent-directory JS/CSS imports (e.g. `../browser-state.js`) → root
   - Asset subdirectories from `site/<game-name>/` (skip base64-embedded assets)
   - `games/<game-name>/public/` → `public/` (if it exists — OG images, favicons, robots.txt, etc.)

5. **Rewrite paths in `index.html`.** Change `../pkg/` → `./pkg/`, `../` imports → `./`, etc. All references must be self-contained (no parent paths).

6. **Inject SEO metadata.** If `games/<game-name>/seo.jsonld` exists: extract the `_seo_meta` key for HTML metadata (title, description, theme-color, canonical, og:*, twitter:*); inject the rest as `<script type="application/ld+json">`. If missing, warn and continue.

7. **Validate.** Scan deployed HTML for asset references. Warn about any missing files and fix mismatches in the deployment folder.

8. **Report.** Print deployment path, WASM size, total size, SEO status. Remind: "Ready to deploy to any static host."

**Rules:**
- Fixing files in the deployment folder is always allowed — the goal is zero broken links
- Never modify `site/`, `_site/`, or existing build output
- `deployments/` is gitignored
- If the deployment folder already exists, warn before overwriting
- Per-game config (seo.jsonld, public/) lives at `games/<game-name>/` (project root)

## Autonomy Level

The assistant operates on an autonomy scale from 1–10 that controls how often it asks for human input vs. making decisions independently:

| Level | Behavior |
|-------|----------|
| 10 | Fully autonomous — never asks for feedback, makes all decisions |
| 9 | Near-full autonomy — only asks when a decision is truly irreversible and high-risk |
| 8 | Minimal questions — asks only for major architectural or scope-changing decisions |
| 7 | **Default** — assumes autonomy in grey areas, asks when genuinely uncertain about intent |
| 6 | Moderate — checks in on ambiguous requirements but handles implementation details solo |
| 5 | Balanced — asks about approach before starting, then executes independently |
| 4 | Collaborative — frequent check-ins on direction and approach |
| 3 | Cautious — asks before most non-trivial decisions |
| 2 | Very cautious — asks before nearly every action |
| 1 | Full confirmation — asks about everything |

**Default: 7.** The user can override by saying e.g. "autonomy 9" or "set autonomy to 4" at any point in the session. In grey areas — where the right call isn't obvious — the assistant should bias toward acting over asking, unless the current level is 5 or below.
