# Crusty UI Upgrades — Modern Visual Design

## Problem Statement

Every game built with Crusty uses a **5x7 bitmap pixel font** hardcoded in `engine/crates/engine-core/src/rendering/text.rs`. This gives everything a retro look. We want the ability to create UIs with **modern, clean text** like excellent web apps — while keeping the retro option for in-world game elements.

---

## Current Architecture

| Component | How It Works |
|-----------|-------------|
| **Text rendering** | 5x7 bitmap font, 95 ASCII chars, integer scaling only |
| **Framebuffer** | Software-rendered RGBA `Vec<u8>`, blitted to canvas via `putImageData()` |
| **UI system** | `ui_canvas.rs` — anchor-based widgets (Label, Bar, Button) drawn into the framebuffer |
| **Web shell** | WASM exports framebuffer pointer; JS reads pixels and draws to `<canvas>` |
| **State bridge** | `get_game_state_f64(key)` / `set_game_state_f64(key, val)` for Rust-to-JS data flow |
| **Existing HTML HUD** | Gravity-pong already has a CSS pill-based HUD overlay (`index.html` lines 28-104), currently **hidden** |

The gravity-pong game already has the skeleton of the hybrid architecture — it just needs to be promoted to a first-class engine feature.

---

## Recommended Strategy: Hybrid Rendering

**Keep the framebuffer for game-world rendering. Use HTML/CSS overlays for all UI.**

This is the dominant pattern used by production browser games (agar.io, slither.io, etc.):

```
┌─────────────────────────────────────┐
│  HTML/CSS UI Layer (z-index: 10+)   │  ← Modern fonts, CSS layout, glassmorphism
│  - HUD (scores, health bars)        │
│  - Menus / modals                   │
│  - Tooltips, notifications          │
├─────────────────────────────────────┤
│  <canvas> Game Layer (z-index: 0)   │  ← Framebuffer blit, bitmap font for
│  - Entities, particles, effects     │     in-world labels, debug text
│  - Physics debug overlays           │
└─────────────────────────────────────┘
```

### Why This Works

- **Native font rendering** — sub-pixel anti-aliasing, kerning, ligatures for free
- **Web fonts** (Inter, system stack) render at full quality at any size
- **CSS handles** hover states, transitions, responsive sizing with zero custom code
- **Performance** — 100 DOM buttons maintain 60+ FPS; DOM text updates are negligible vs framebuffer blit
- **Accessibility** — screen readers, text selection built in
- **Dramatically less code** — 530 bytes for a DOM button vs 1.4KB canvas equivalent

---

## Implementation Plan

### Phase 1: HTML Overlay Infrastructure

**Goal:** Create a reusable HTML/CSS overlay system that any Crusty game can use.

#### 1a. Standard HTML Shell Template

Every game gets a standard HTML structure:

```html
<body>
  <!-- Game canvas -->
  <canvas id="game-canvas"></canvas>

  <!-- HUD overlay: pointer-events:none so clicks pass to canvas -->
  <div id="hud" style="pointer-events:none; z-index:10;">
    <!-- Interactive elements get pointer-events:auto -->
  </div>

  <!-- Menu overlay: blocks input when visible -->
  <div id="menu-overlay" style="z-index:20;"></div>
</body>
```

#### 1b. CSS Grid HUD Layout

Replace manual coordinate math with CSS grid anchoring:

```css
#hud {
  position: fixed;
  inset: 0;
  display: grid;
  grid-template-areas:
    "topleft  topcenter  topright"
    ".        center     ."
    "botleft  botcenter  botright";
  grid-template-rows: auto 1fr auto;
  grid-template-columns: auto 1fr auto;
  padding: 16px;
  pointer-events: none;
}
.hud-topleft   { grid-area: topleft; }
.hud-topcenter { grid-area: topcenter; justify-self: center; }
.hud-topright  { grid-area: topright; justify-self: end; }
.hud-center    { grid-area: center; align-self: center; justify-self: center; }
.hud-botleft   { grid-area: botleft; align-self: end; }
.hud-botcenter { grid-area: botcenter; align-self: end; justify-self: center; }
.hud-botright  { grid-area: botright; align-self: end; justify-self: end; }
```

#### 1c. State Bridge Enhancement

Add string state export so Rust can send text to JS HUD:

```rust
#[wasm_bindgen]
pub fn get_game_state_str(key: String) -> String { ... }
```

JS update loop reads state and sets `textContent` on HUD elements each frame.

---

### Phase 2: Modern Typography

#### Font Stack

```css
:root {
  --font-ui: 'Inter', 'Roboto', 'Helvetica Neue', 'Arial Nova', sans-serif;
  --font-mono: 'JetBrains Mono', 'Fira Code', 'SF Mono', monospace;
}
```

- **Inter** — primary UI font (tall x-height, 9 weights, designed for screens)
- **JetBrains Mono** — scores, timers, debug info (tabular numbers)
- Load via Google Fonts or self-host for zero-latency

#### Responsive Typography

```css
.hud-label { font-size: clamp(0.65rem, 1.5vw, 0.85rem); color: #888; }
.hud-value { font-size: clamp(0.85rem, 2vw, 1.2rem); color: #e0e0e0; }
.hud-title { font-size: clamp(1.5rem, 4vw, 3rem); font-weight: 700; }
```

Use `font-variant-numeric: tabular-nums` for all numeric displays.

---

### Phase 3: Modern UI Components (CSS)

#### Glassmorphism Panels

```css
.glass-panel {
  background: rgba(255, 255, 255, 0.06);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  border: 1px solid rgba(255, 255, 255, 0.12);
  box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
  border-radius: 16px;
  padding: 16px;
}
```

**Performance notes:**
- Limit glassmorphic elements to 2-3 per viewport
- Reduce blur to 6-8px on mobile
- Never animate elements with `backdrop-filter`

#### Stat Pills (already prototyped in gravity-pong)

```css
.hud-stat {
  background: rgba(64, 255, 192, 0.06);
  border: 1px solid rgba(64, 255, 192, 0.12);
  border-radius: 20px;
  padding: 6px 14px;
  font-family: var(--font-ui);
  pointer-events: auto;
}
```

#### Modern Buttons

```css
.btn {
  font-family: var(--font-ui);
  font-size: clamp(0.75rem, 1.8vw, 1rem);
  font-weight: 600;
  padding: 10px 24px;
  border-radius: 8px;
  border: none;
  cursor: pointer;
  transition: transform 0.1s, box-shadow 0.15s;
  pointer-events: auto;
}
.btn:hover { transform: translateY(-1px); box-shadow: 0 4px 12px rgba(0,0,0,0.3); }
.btn:active { transform: translateY(0); }
.btn-primary { background: #40ffc0; color: #060612; }
.btn-ghost { background: transparent; border: 1px solid rgba(255,255,255,0.2); color: #e0e0e0; }
```

---

### Phase 4: Color System

The existing gravity-pong palette is already well-aligned with modern dark-mode design:

```
Base:      #060612  (deep navy-black)
Accent:    #40ffc0  (neon mint)
Text:      #e0e0e0  (primary), #888 (secondary)
```

**Additions for variety across games:**

```css
:root {
  /* Primary accents (pick one per game) */
  --accent-mint:   #40ffc0;
  --accent-cyan:   #00CAFF;
  --accent-pink:   #F6287D;
  --accent-purple: #B915CC;
  --accent-gold:   #FFD700;

  /* Semantic colors */
  --color-success: #22C55E;
  --color-warning: #F59E0B;
  --color-danger:  #EF4444;
  --color-info:    #3B82F6;

  /* Surfaces */
  --surface-0: #060612;
  --surface-1: rgba(255, 255, 255, 0.04);
  --surface-2: rgba(255, 255, 255, 0.08);
  --border: rgba(255, 255, 255, 0.12);
}
```

Follow the **60-30-10 rule**: 60% dark base, 30% muted grays, 10% accent color.

---

### Phase 5: Declarative HUD from Rust (Engine Feature)

Long-term, let game scripts declare HUD layouts that the JS shell renders:

```rust
// In game setup
hud.add("topcenter", HudWidget::Label {
    key: "score",
    label: "SCORE",
    style: "stat-pill",
});
hud.add("botleft", HudWidget::Bar {
    value_key: "health",
    max_key: "max_health",
    style: "health-bar",
});
```

Rust serializes the HUD layout to a JSON string. JS reads it once at init and creates DOM elements. Each frame, JS reads game state values and updates text content.

This keeps the game script as the single source of truth while leveraging HTML/CSS for rendering.

---

## What to Keep in the Framebuffer

Not everything should move to HTML. Keep these in the software renderer:

- **In-world text** — entity labels, floating damage numbers, name tags
- **Debug overlays** — FPS counter, physics wireframes, collision boxes
- **Particle effects** — text that moves/rotates with the game world
- **Retro-style games** — when the bitmap aesthetic is intentional

The 5x7 bitmap font stays as-is for these use cases.

---

## Migration Path for Existing Games

1. **Gravity Pong** — already has hidden HTML HUD. Un-hide it, remove the framebuffer-rendered score/labels, wire up state bridge
2. **New games** — start with the HTML overlay template from day one
3. **Engine core** — no breaking changes needed; this is purely additive

---

## Priority Order

| Priority | Task | Effort |
|----------|------|--------|
| P0 | Activate gravity-pong's existing HTML HUD, remove framebuffer UI text | Small |
| P1 | Create reusable CSS component library (pills, buttons, panels, grid layout) | Medium |
| P1 | Add `get_game_state_str` export for string data | Small |
| P2 | Add Inter + JetBrains Mono fonts | Small |
| P2 | Create standard HTML shell template for new games | Medium |
| P3 | Declarative HUD-from-Rust system (JSON serialization) | Large |
| P3 | Glassmorphism panels with perf-safe defaults | Small |

---

## Sources

- [HTML5 Game UI: Canvas vs DOM Performance](https://blog.sklambert.com/html5-game-tutorial-game-ui-canvas-vs-dom/)
- [Kaplay: HTML UI on Top of Canvas Games](https://jslegenddev.substack.com/p/how-to-display-an-html-based-ui-on)
- [Rust WASM Canvas Games — Lessons Learned](https://dev.to/fallenstedt/making-a-canvas-based-game-with-rust-and-webassembly-2l46)
- [Agar.io Clone Architecture](https://github.com/owenashurst/agar.io-clone/wiki/Game-Architecture)
- [Glassmorphism in 2025 UI Design](https://www.atvoid.com/blog/what-is-glassmorphism-the-transparent-trend-defining-2025-ui-design)
- [Dark Mode Color Palettes 2025](https://colorhero.io/blog/dark-mode-color-palettes-2025)
- [Best Fonts for UI Design 2026](https://www.untitledui.com/blog/best-free-fonts)
- [Modern Font Stacks](https://modernfontstacks.com)
- [Canvas Text Performance via Offscreen Caching](https://www.mirkosertic.de/blog/2015/03/tuning-html5-canvas-filltext/)
- [MDN: Optimizing Canvas](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Optimizing_canvas)
