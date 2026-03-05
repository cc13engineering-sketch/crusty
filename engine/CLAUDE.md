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

## Rendering Rules
- All visual primitives MUST be anti-aliased (1px feather on edges). No hard-edged shapes.
- Use `fill_circle` (AA), `fill_tapered_trail`, and `fill_triangle` (AA) from `shapes.rs`.
- Trails/streaks use `fill_tapered_trail` (single-pass distance-field polyline) — never raw `draw_line`.
- Glow effects: render a wider, low-alpha pass underneath the bright core pass.

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

When the user says `ship <game-name>`, build a fully self-contained, static-host-ready deployment folder. This is an "eject" mechanism — it does NOT touch GitHub Pages, `_site/`, or any existing deployment infrastructure.

**Steps:**

1. **Resolve the game.** Match `<game-name>` (case-insensitive, hyphen/space flexible) to a directory under `site/`. Known games and their aliases:
   - `music-theory` — aka "chord reps", "chordreps", "music theory"
   - `gravity-pong` — aka "gravity pong"
   - `demo-ball` — aka "demo ball"
   - Any other directory under `site/` that contains an `index.html`

2. **Build WASM.** From the `engine/` directory:
   ```bash
   wasm-pack build crates/engine-core --target web --out-dir "$ROOT/_pkg" -- --no-default-features
   ```
   Skip this step if `_pkg/engine_core_bg.wasm` already exists and the user says to skip the build, or if a recent build is already available. When in doubt, rebuild.

3. **Determine version.** Read the version from `engine/crates/engine-core/Cargo.toml` (currently `0.1.0`). Compute WASM hash: first 12 chars of SHA-256 of `_pkg/engine_core_bg.wasm`. The deployment folder name is: `deployments/<game-name>-<version>` (e.g., `deployments/gravity-pong-0.1.0`).

4. **Assemble the deployment folder.** Create `deployments/<game-name>-<version>/` at the project root:
   ```
   deployments/<game-name>-<version>/
   ├── index.html              # Game HTML (paths rewritten)
   ├── pkg/
   │   ├── engine_core.js      # WASM JS glue
   │   └── engine_core_bg.wasm # WASM binary
   └── browser-state.js        # Only if the game imports it
   ```

5. **Copy files:**
   - Copy `site/<game-name>/index.html` to the deployment root
   - Copy `_pkg/engine_core.js` and `_pkg/engine_core_bg.wasm` into `pkg/`
   - If the game's HTML imports `../browser-state.js`, copy `site/browser-state.js` to the deployment root
   - Copy any other asset subdirectories from `site/<game-name>/` (but NOT `samples/` if its contents are already base64-embedded in the HTML)
   - If `<game-name>/public/` exists at the project root, copy its contents directly into the deployment root (files land at the top level, not inside a `public/` subdirectory). This is for static assets like Open Graph images, favicons, `robots.txt`, etc.

6. **Inject SEO from JSON-LD.** Look for `<game-name>/seo.jsonld` at the project root (i.e., `crusty/<game-name>/seo.jsonld`). This file is the **single source of truth** for all SEO metadata. If it exists:

   **a) Inject structured data:** Strip the `_seo_meta` key (it's not valid schema.org), then inject the remaining JSON as a `<script type="application/ld+json">` block into `<head>` (before `</head>`).

   **b) Set HTML `<title>`:** Read `_seo_meta.title` and set/replace the page's `<title>` tag.

   **c) Inject meta tags:** From `_seo_meta`, inject or replace these in `<head>`:
   - `<meta name="description" content="...">` from `meta_description`
   - `<meta name="theme-color" content="...">` from `theme_color`
   - `<link rel="canonical" href="...">` from `canonical`

   **d) Inject Open Graph tags:** From `_seo_meta`, inject:
   - `<meta property="og:title" content="...">` from `og_title`
   - `<meta property="og:description" content="...">` from `og_description`
   - `<meta property="og:type" content="...">` from `og_type`
   - `<meta property="og:url" content="...">` from `og_url`
   - `<meta property="og:image" content="...">` from `og_image`

   **e) Inject Twitter card tags:** From `_seo_meta`, inject:
   - `<meta name="twitter:card" content="...">` from `twitter_card`
   - `<meta name="twitter:title" content="...">` from `og_title`
   - `<meta name="twitter:description" content="...">` from `og_description`
   - `<meta name="twitter:image" content="...">` from `og_image`

   If the file doesn't exist, warn the user: "No seo.jsonld found at `<game-name>/seo.jsonld` — deploy will proceed without SEO metadata. Create one for better SEO."

7. **Rewrite paths in `index.html`:**
   - `../pkg/` → `./pkg/` (WASM imports)
   - `../browser-state.js` → `./browser-state.js` (if present)
   - `__WASM_HASH__` → the 12-char SHA-256 hash computed in step 3

8. **Report results.** Print:
   - Deployment path
   - WASM binary size
   - Total folder size
   - Whether SEO JSON-LD was injected
   - Remind the user: "Ready to deploy — drag this folder into Cloudflare Pages, Netlify, Vercel, or any static host."

**Important rules:**
- Never modify files under `site/`, `_site/`, or any existing build output
- The `deployments/` directory is gitignored (add to `.gitignore` if not already)
- If the deployment folder already exists, warn the user and ask before overwriting
- Each game deployment is fully self-contained — no parent directory references
- SEO JSON-LD source files live at `crusty/<game-name>/seo.jsonld` (project root, NOT inside `site/` or `engine/`)
- Static deploy assets (OG images, favicons, `robots.txt`, etc.) live at `crusty/<game-name>/public/`. When creating or updating `seo.jsonld`, reference OG images and icon files from this directory (e.g., `og_image` should point to a file that exists in `<game-name>/public/`)

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
