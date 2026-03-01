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
- Adding a .world property: extend loader.rs mapping. Grammar's catch-all handles new ident:value properties automatically.

## Parser Gotchas
- `string` rule includes quotes — strip with `&s[1..s.len()-1]`
- `number` rule yields string — parse with `.parse::<f64>()`
- `color_value` includes `#` — pass directly to `Color::from_hex()`
- pest grammar path in `#[grammar = "..."]` is relative to src/

## Innovation Games (Development Methodology)

Innovation Games drive this engine's development. Each round follows a structured process with mandatory pre-work, competitive proposals, and changelog publishing.

### Pre-Work: Codebase Audit (mandatory before each round)
Before starting proposals, spawn agents with different technical backgrounds to:
1. **Rust Engine Expert** — review engine code for dead code, unused imports, API drift, unfixed bugs, test gaps.
2. **Game Developer** — verify all demos compile and run against current WASM build, check .world files use current grammar.
3. **Documentation Reviewer** — audit CLAUDE.md, PROCESS.md, REVIEW.md, CHANGELOG.md, ARCHITECTURE.md for accuracy against actual source.
4. Fix all issues found, update docs, commit as "pre-work: codebase audit".
This ensures we build cohesively and don't accumulate drift between docs, code, and demos.

### Innovation Round Steps
1. **Pre-work audit** (see above)
2. **Spawn competing agents** (4 agents) — each proposes features/ideas independently
3. **Theme-driven ideation** — proposals validated against the round's theme
4. **Cross-pollinate** — review agents select best ideas across all competitors
5. **Integrate** — winning features implemented with tests
6. **Demo** — build/update demo games showcasing new features
7. **Changelog publish** — update CHANGELOG.md, publish changelog page to GitHub Pages with "how to run" info for all demos

Key principle: features are designed for *engine generality*, not just the theme game. The theme game validates that features compose well together.

### Round Schedule
- **Rounds 1-2**: Focus on innovations that allow Claude Code to design high-quality immersive mobile games with minimal human input. Think about every aspect of the codebase.
- **Rounds 3-4**: In-depth game concept competition. Theme: large-map tile-based RPG, Pokémon-style, but instead of fighting trainers, players encounter traps that open a "fight puzzle scene" (like Pokémon fight scenes) where they must use minigolf-like mechanics to solve/fight through the encounter. Must be a rich, ready-to-build concept.
- **Rounds 5-6**: Code and engine improvements specifically to make building the agreed-upon game feasible.
- **Rounds 7+**: Open feature competition. Continue until told to stop.

### Past Rounds
- **Round 1**: Space Survival theme. Added: SpawnQueue, GameState, Behavior AI, Lifetime/Lifecycle, Particle System, Bitmap Text, Starfield, Post-FX (vignette/scanlines/shake), HUD rendering, Gameplay collision rules, Wave spawning. Demo: game-3 (Space Survival).
- **Round 2**: Minigolf Tile Art RPG theme. Added: Camera follow/zoom with smooth lerp, Render layer stack with parallax, Sprite sheet renderer, Scene transitions (fade/iris/pixelate), PhysicsMaterial (friction/drag), Impulse component, MotionConstraint (speed cap/axis lock), ZoneEffect (wind/drag/conveyor), DialogueQueue (dialogue/notification/floating text).
- **Round 3**: Puzzle Platformer with Time Mechanics theme. Added: PropertyTween (9 easing curves), EntityFlash (hit flash/blink/color pulse), GhostTrail (fading afterimage ring buffer), Per-Entity TimeScale, Active component, WaypointPath (once/loop/ping-pong), SignalEmitter/SignalReceiver (wired logic gates with AND/OR/edge detection), ScreenFxStack (composable timed effects), SceneManager (push/pop scene stack).
- **Round 4**: Signal Breach (Tactical Stealth-Puzzle) theme. Added: Parent/Children/WorldTransform (entity hierarchy with transform propagation), StateMachine (data-driven FSM with transitions), Coroutine (scripted async step sequences), TileMap (row-major grid with viewport culling), Raycast (circle/AABB/DDA grid), SpatialHashGrid (cell-bucketed spatial index), EntityPool (pre-warmed recycling).
- **Round 5**: Expert Review & E2E Testing. Allocation optimizations (HashSet<&str>), ghost trail ring buffer fix, raycast DDA fix, flash/tween system optimizations, Default impls. 22 comprehensive E2E integration tests.
- **Round 6**: Feature Bonanza. Added: SpriteAnimator (named clips with frame sequences), PhysicsJoint (distance/spring/rope/hinge with break force), EventBus (channel-based typed events), InputMap (abstract input layer), A* Pathfinding (octile heuristic, diagonal movement), Save/Load (world snapshot to JSON).
- **Round 7**: Ecosystem Infrastructure. Added: ResourceInventory (bounded multi-resource container), GraphNode (entity-to-entity graph edges), VisualConnection (visual links with styles), FlowNetwork (directed resource flow), ProceduralGen (noise + cellular automata + dungeon gen), EnvironmentClock (cyclical time phases), DensityField (2D scalar field with diffusion).
- **Demo Build**: Mycelia: Ascent — procedural cave game showcasing 8+ engine systems composing together. Custom Rust module with WASM bindings, mobile-first 480x720.

## Demo Game Building Process

When building a demo game:
1. **Read `PROCESS.md`** at repo root for lessons learned, API pitfalls, and patterns from prior builds. Use as helper info — first principles are king.
2. **Research the engine API** before writing code. Read the actual source for components, systems, and rendering functions you plan to use. Don't assume signatures.
3. **Decide architecture**: `.world` file for simple games, custom Rust module for anything procedural or with custom mechanics.
4. **Build iteratively**: Get compilation passing first, then tests, then web integration.
5. **After each build**: Update `PROCESS.md` with new findings, API corrections, and patterns discovered. Every build attempt teaches something — capture it.
