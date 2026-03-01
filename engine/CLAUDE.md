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
Innovation Games drive much of this engine's development. The process:
1. **Spawn competing agents** — each proposes novel engine features independently.
2. **Theme-driven ideation** — each round has a game concept theme (e.g., space survival, minigolf RPG) used as an ideation seed. Agents validate feature proposals against the theme game's needs.
3. **Cross-pollinate** — after proposals, review agents select the best ideas across all competitors.
4. **Integrate** — winning features get implemented into the engine with tests.
5. **Demo** — optionally build a demo game showcasing the new features.

Past rounds:
- **Round 1**: Space Survival theme. Added: SpawnQueue, GameState, Behavior AI, Lifetime/Lifecycle, Particle System, Bitmap Text, Starfield, Post-FX (vignette/scanlines/shake), HUD rendering, Gameplay collision rules, Wave spawning. Demo: game-3 (Space Survival).
- **Round 2**: Minigolf Tile Art RPG theme. Added: Camera follow/zoom with smooth lerp, Render layer stack with parallax, Sprite sheet renderer, Scene transitions (fade/iris/pixelate), PhysicsMaterial (friction/drag), Impulse component, MotionConstraint (speed cap/axis lock), ZoneEffect (wind/drag/conveyor), DialogueQueue (dialogue/notification/floating text).

- **Round 3**: Puzzle Platformer with Time Mechanics theme. Added: PropertyTween (9 easing curves for any numeric property), EntityFlash (hit flash/blink/color pulse), GhostTrail (fading afterimage ring buffer), Per-Entity TimeScale (local time multiplier), Active component (entity enable/disable), WaypointPath (once/loop/ping-pong path following), SignalEmitter/SignalReceiver (wired logic gate system with AND/OR/edge detection), ScreenFxStack (composable timed tint/desaturate/flash effects), SceneManager (named scene registry with push/pop stack). World Snapshot deferred.

Key principle: features are designed for *engine generality*, not just the theme game. The theme game validates that features compose well together.

## Demo Game Building Process

When building a demo game:
1. **Read `PROCESS.md`** at repo root for lessons learned, API pitfalls, and patterns from prior builds. Use as helper info — first principles are king.
2. **Research the engine API** before writing code. Read the actual source for components, systems, and rendering functions you plan to use. Don't assume signatures.
3. **Decide architecture**: `.world` file for simple games, custom Rust module for anything procedural or with custom mechanics.
4. **Build iteratively**: Get compilation passing first, then tests, then web integration.
5. **After each build**: Update `PROCESS.md` with new findings, API corrections, and patterns discovered. Every build attempt teaches something — capture it.
