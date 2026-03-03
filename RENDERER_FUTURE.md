# Renderer Future — Intent Document

Current state and planned direction for the rendering layer.

## Current Architecture
- **Software framebuffer**: CPU-side RGBA pixel buffer, blitted to HTML canvas via `putImageData`
- **Resolution**: Configurable per game (e.g., 480x720 portrait)
- **Drawing primitives**: fill_rect, fill_circle, draw_line, draw_text (5x7 bitmap font)
- **Layers**: RenderLayerStack with parallax support
- **Effects**: Post-FX (vignette, scanlines, shake), ScreenFxStack (flash, tint)
- **Sprites**: SpriteSheet with frame-based animation
- **TileMap**: Viewport-culled tile rendering with multi-layer support
- **Particles**: Dense Vec pool, not ECS

## Target Entity Count Ceiling
- ~100-300 active entities per game
- Particles use dense Vec (not ECS) — can handle thousands
- TileMap tiles are not entities — grid-based, viewport-culled

## Long-term Target: WebGPU / wgpu
- When performance demands it, migrate from software framebuffer to wgpu
- Shaders will be authored in WGSL
- The framebuffer API will become a thin wrapper over GPU texture writes
- Current `shapes::fill_circle`, `text::draw_text` etc. become GPU draw calls

## Batching Strategy (future)
- Not needed today at 480x720 with <300 entities
- When adopted: batch by texture atlas, minimize state changes
- Sprite atlas strategy: single atlas per biome/region

## Migration Path
1. Keep software framebuffer as fallback (WebGL not available, tests, headless)
2. Add wgpu backend behind feature flag
3. Abstract drawing primitives behind a `Renderer` trait
4. Game code never calls GPU APIs directly — always through engine rendering layer

## No action required now
This document captures intent. Implementation happens when performance demands it.
