# Building Demo Games with Crusty Engine

Lessons learned from building demo games. LLM-optimized reference — use as helper info alongside first principles.

---

## Architecture Decision: .world File vs Custom Module

**Use `.world` files** for simple games that fit the declarative DSL (static layouts, standard components, built-in systems). See `site/game-1/`, `site/game-2/` for examples.

**Use a custom Rust module** when you need:
- Procedural generation (caves, terrain, levels)
- Custom game logic beyond built-in systems
- Non-standard input handling (tap-to-grow, gesture-based)
- Custom render pipeline ordering

Pattern for custom modules:
1. Create `engine/crates/engine-core/src/your_game.rs`
2. Add `pub mod your_game;` to `lib.rs`
3. Add `#[wasm_bindgen]` exports in `lib.rs` that call `with_engine(|eng| your_game::func(eng))`
4. Create `your-game-dir/game.js` that imports your custom WASM functions

## API Pitfalls (Most Common Build Errors)

These caused real compilation failures. Memorize them:

### TimeCycle / EnvironmentClock
```rust
// CORRECT:
engine.environment_clock.add_cycle(TimeCycle::new(
    "pulse",
    vec![("surge", 4.0), ("rest", 6.0)],  // Vec of (&str, f64) tuples
));
let is_surge = engine.environment_clock.phase("pulse")
    .map_or(false, |p| p == "surge");

// WRONG: ClockPhase::new doesn't exist
// WRONG: TimeCycle::new takes 2 args, not 3 (no speed param)
// WRONG: .current_phase() — it's .phase() on EnvironmentClock
```

### GraphNode::add_edge
```rust
// CORRECT:
graph.add_edge(target_entity, "connection", 1.0, true);
// Args: (Entity, &str edge_type, f64 weight, bool bidirectional)

// WRONG: add_edge(GraphEdge { ... }) — no GraphEdge struct param
```

### TileMap::render Camera Coordinates
```rust
// CORRECT: pass camera CENTER position
let (sw, sh) = (engine.framebuffer.width as f64, engine.framebuffer.height as f64);
tm.render(&mut engine.framebuffer, cam.x + sw/2.0, cam.y + sh/2.0, cam.zoom, sw as usize, sh as usize);

// WRONG: passing cam.x, cam.y directly (those are top-left corner)
```

### Drawing Primitives
```rust
use crate::rendering::shapes::{fill_rect, draw_rect, fill_circle, draw_circle, draw_line, draw_line_thick};
use crate::rendering::text::{draw_text, draw_text_centered, text_width};
// All draw functions take &mut Framebuffer as first arg
// fill_rect(fb, x, y, w, h, color)
// fill_circle(fb, cx, cy, radius, color)
// draw_text(fb, text, x, y, color) — returns width
// draw_text_centered(fb, text, cx, y, color)
```

### Entity Creation Pattern
```rust
let entity = engine.world.spawn();
engine.world.transforms.insert(entity, Transform::new(x, y));
engine.world.tags.insert(entity, Tags::new(&["node", "root"]));
engine.world.graph_nodes.insert(entity, GraphNode::new());
```

### Global State for Game Variables
```rust
engine.global_state.set_f64("energy", 100.0);
let energy = engine.global_state.get_f64("energy").unwrap_or(0.0);
// Use string keys. get_f64 returns Option<f64>.
```

## Mobile-First Design Rules

1. **Portrait resolution**: 480x720 works well for mobile
2. **Tap-only input**: No drag, swipe, or multi-touch — tap is the universal mobile action
3. **Touch handling**: Use `passive: false` and `e.preventDefault()` on touch events
4. **Tap vs drag detection**: Track pointer start position, use 15px threshold
5. **CSS requirements**: `touch-action: none`, `user-select: none`, `-webkit-user-select: none`
6. **Viewport meta**: `<meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">`

## Game Loop Pattern (JS Side)

```javascript
async function main() {
    const wasm = await initWasm();
    init(WIDTH, HEIGHT);
    your_game_init(seed);

    // ... canvas setup, input handlers ...

    let lastTime = performance.now();
    function frame(now) {
        const dt = Math.min(now - lastTime, 50); // cap at 50ms
        lastTime = now;
        your_game_update(dt);
        tick(dt);                // engine systems
        your_game_render();      // custom render
        // framebuffer → canvas via putImageData
        requestAnimationFrame(frame);
    }
    requestAnimationFrame(frame);
}
```

## Custom Render Pipeline (Rust Side)

Order matters. Typical layer stack:
1. `framebuffer.clear(background_color)`
2. TileMap render (terrain/cave/world)
3. Game-specific elements (connections, nodes, entities)
4. Overlay effects (blight, fog, weather)
5. Particles (`engine.particles.render()`)
6. HUD (score, energy, status text)
7. Screen effects (`engine.screen_fx.apply()`)
8. Post-FX (`rendering::post_fx::apply()`)

## Testing Strategy

- Test setup/init functions verify entity creation and state initialization
- Test game mechanics (tap, energy, scoring) with controlled seeds
- For borrow checker issues in tests: collect data in a block scope, drop borrows, then call mutable methods
- Use `engine.global_state.get_f64()` to assert game state changes
- Avoid noise-based procedural generation in tests — use simple `rng.chance()` for predictable results

## Procedural Generation Tips

- Cellular automata works well for caves: random fill → iterate smoothing rules → carve guaranteed spaces
- Always carve a starting area so the player has room
- For seeded randomness: use `Rng::new(seed)` from the engine's rng module
- Noise2D with tight thresholds can produce zero results for certain seeds — prefer `rng.chance(probability)` for sparse placement

## Project Structure Convention

```
innovations-N/          # Demo game N
  index.html            # Mobile-optimized HTML shell
  game.js               # WASM loader + input + game loop
  PROPOSAL.md           # Original game proposal
engine/crates/engine-core/src/
  your_game.rs           # Custom game module
site/
  index.html            # Updated with game card link
```
