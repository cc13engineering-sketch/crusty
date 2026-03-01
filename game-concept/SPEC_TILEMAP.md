# SPEC_TILEMAP: Tilemap and Rendering Implementation Specification

**Gaps covered**: Gap 1 (Multi-layer TileMap), Gap 2 (Sprite sheet tile rendering), Gap 6 (Circular arc aim line)
**Target game**: Trap Links (Minigolf RPG)
**Engine**: Crusty (Rust -> WASM -> Canvas 2D)

---

## Gap 1: Multi-layer TileMap

### Problem Statement

The current `TileMap` struct stores a single flat `Vec<Tile>` in row-major order. Trap Links requires at minimum two layers for the overworld:
- **Background layer** (terrain: grass, stone, water, ice, sand) -- always drawn first.
- **Object layer** (trap markers, save points, exit arrows, decorative props) -- drawn on top of terrain.

Fight scenes also benefit from layering: a floor layer beneath obstacle entities. The single-layer design forces game code to conflate terrain data with object data in one grid cell, making collision queries ambiguous (is this tile "grass with a trap on it" or "just a trap"?).

### Current Code Under Modification

**File**: `/home/user/crusty/engine/crates/engine-core/src/tilemap.rs`

Current `TileMap` struct (lines 58-66):
```rust
pub struct TileMap {
    pub width: usize,
    pub height: usize,
    pub tile_size: f64,
    pub tiles: Vec<Tile>,    // single flat buffer
    pub origin_x: f64,
    pub origin_y: f64,
}
```

Current `Engine` field (in `engine.rs` line 189):
```rust
pub tilemap: Option<TileMap>,
```

### Design Decisions

**Approach chosen**: Internal layered storage within `TileMap`, NOT multiple separate `TileMap` instances.

Rationale: Keeping layers inside one struct preserves the existing API surface (`world_to_tile`, `is_solid`, `fill_rect`) as thin wrappers over a default layer, and guarantees all layers share the same grid dimensions, tile size, and origin. Game code that only needs one layer (fight scenes) uses the existing API unchanged.

**Layer model**: A `TileLayer` struct wraps a `Vec<Tile>` plus per-layer metadata. `TileMap` holds a `Vec<TileLayer>` instead of a bare `Vec<Tile>`.

**Default layer**: Layer index 0 is always the "terrain" layer and is auto-created in `TileMap::new`. All existing single-layer APIs operate on layer 0 for backward compatibility.

### Struct / Enum / Trait Additions

```rust
// ---- tilemap.rs additions ----

/// Metadata and tile storage for a single tilemap layer.
#[derive(Clone, Debug)]
pub struct TileLayer {
    /// Human-readable name for debugging and lookup ("terrain", "objects", etc.).
    pub name: String,
    /// Row-major tile storage, length = width * height. Same dimensions as parent TileMap.
    pub tiles: Vec<Tile>,
    /// Whether this layer participates in collision queries (is_solid, is_solid_at_world).
    pub collidable: bool,
    /// Whether this layer is rendered. Set to false to hide a layer without destroying data.
    pub visible: bool,
    /// Opacity multiplier applied to every tile color on this layer during rendering.
    /// 1.0 = fully opaque (tile alpha unchanged), 0.0 = fully transparent.
    pub opacity: f64,
}

impl TileLayer {
    /// Create a new layer filled with empty tiles.
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        let tiles = (0..width * height).map(|_| Tile::empty()).collect();
        Self {
            name: name.to_string(),
            tiles,
            collidable: true,
            visible: true,
            opacity: 1.0,
        }
    }
}
```

### Modified `TileMap` Struct

```rust
#[derive(Clone, Debug)]
pub struct TileMap {
    pub width: usize,
    pub height: usize,
    pub tile_size: f64,
    pub layers: Vec<TileLayer>,   // replaces `pub tiles: Vec<Tile>`
    pub origin_x: f64,
    pub origin_y: f64,
}
```

### Functions to Modify

#### `TileMap::new` -- add default layer

```rust
pub fn new(width: usize, height: usize, tile_size: f64) -> Self {
    let default_layer = TileLayer::new("terrain", width, height);
    Self {
        width,
        height,
        tile_size,
        layers: vec![default_layer],
        origin_x: 0.0,
        origin_y: 0.0,
    }
}
```

#### New layer management methods

```rust
impl TileMap {
    /// Add a new named layer. Returns its index. Layer is appended (drawn on top
    /// of existing layers).
    pub fn add_layer(&mut self, name: &str) -> usize {
        let layer = TileLayer::new(name, self.width, self.height);
        self.layers.push(layer);
        self.layers.len() - 1
    }

    /// Get a layer by index. Returns None if out of bounds.
    pub fn layer(&self, index: usize) -> Option<&TileLayer> {
        self.layers.get(index)
    }

    /// Get a mutable layer by index.
    pub fn layer_mut(&mut self, index: usize) -> Option<&mut TileLayer> {
        self.layers.get_mut(index)
    }

    /// Find a layer index by name. Returns the first match.
    pub fn layer_index(&self, name: &str) -> Option<usize> {
        self.layers.iter().position(|l| l.name == name)
    }

    /// Total number of layers.
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
}
```

#### Backward-compatible single-layer accessors (operate on layer 0)

All existing `get`, `get_mut`, `set`, `is_solid`, `fill_rect`, `clear`, `solid_count` methods are updated to delegate to layer 0. A `tiles` accessor is also provided for migration.

```rust
impl TileMap {
    // --- Convenience: layer-0 delegation ---

    /// Access the flat tile vec of layer 0. Equivalent to the old `self.tiles`.
    pub fn tiles(&self) -> &[Tile] {
        &self.layers[0].tiles
    }

    /// Mutable access to the flat tile vec of layer 0.
    pub fn tiles_mut(&mut self) -> &mut Vec<Tile> {
        &mut self.layers[0].tiles
    }

    // get, get_mut, set -- unchanged signatures, internally index layer 0:
    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.get_on_layer(0, x, y)
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.get_mut_on_layer(0, x, y)
    }
    pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.set_on_layer(0, x, y, tile);
    }

    // --- Layer-specific accessors ---

    pub fn get_on_layer(&self, layer: usize, x: usize, y: usize) -> Option<&Tile> {
        let l = self.layers.get(layer)?;
        self.index(x, y).map(|i| &l.tiles[i])
    }

    pub fn get_mut_on_layer(&mut self, layer: usize, x: usize, y: usize) -> Option<&mut Tile> {
        let l = self.layers.get_mut(layer)?;
        let idx = if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        };
        idx.map(|i| &mut l.tiles[i])
    }

    pub fn set_on_layer(&mut self, layer: usize, x: usize, y: usize, tile: Tile) {
        if let Some(l) = self.layers.get_mut(layer) {
            if let Some(i) = self.index(x, y) {
                l.tiles[i] = tile;
            }
        }
    }

    pub fn fill_rect_on_layer(
        &mut self, layer: usize,
        x: usize, y: usize, w: usize, h: usize, tile: Tile,
    ) {
        if let Some(l) = self.layers.get_mut(layer) {
            let x_end = (x + w).min(self.width);
            let y_end = (y + h).min(self.height);
            for ty in y..y_end {
                for tx in x..x_end {
                    l.tiles[ty * self.width + tx] = tile.clone();
                }
            }
        }
    }
}
```

#### `is_solid` / `is_solid_at_world` -- query ALL collidable layers

```rust
/// Returns true if ANY collidable layer has a Solid tile at (x, y).
pub fn is_solid(&self, x: usize, y: usize) -> bool {
    if let Some(idx) = self.index(x, y) {
        self.layers.iter()
            .filter(|l| l.collidable)
            .any(|l| l.tiles[idx].tile_type == TileType::Solid)
    } else {
        false
    }
}

/// Returns true if ANY collidable layer has a Solid tile at world position (wx, wy).
pub fn is_solid_at_world(&self, wx: f64, wy: f64) -> bool {
    match self.world_to_tile(wx, wy) {
        Some((tx, ty)) => self.is_solid(tx, ty),
        None => false,
    }
}
```

#### `solid_count` -- count across all collidable layers

```rust
pub fn solid_count(&self) -> usize {
    self.layers.iter()
        .filter(|l| l.collidable)
        .flat_map(|l| l.tiles.iter())
        .filter(|t| t.tile_type == TileType::Solid)
        .count()
}
```

#### `clear` -- clear ALL layers

```rust
pub fn clear(&mut self) {
    for layer in self.layers.iter_mut() {
        for tile in layer.tiles.iter_mut() {
            *tile = Tile::empty();
        }
    }
}
```

#### `fill_rect` -- operates on layer 0

```rust
pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, tile: Tile) {
    self.fill_rect_on_layer(0, x, y, w, h, tile);
}
```

#### `render` -- iterate layers in order, skip invisible layers

```rust
pub fn render(
    &self,
    fb: &mut crate::rendering::framebuffer::Framebuffer,
    camera_x: f64,
    camera_y: f64,
    zoom: f64,
    screen_width: u32,
    screen_height: u32,
) {
    // Compute visible tile range once (same for all layers).
    let sw = screen_width as f64;
    let sh = screen_height as f64;
    let half_sw = sw * 0.5;
    let half_sh = sh * 0.5;
    let tile_px = self.tile_size * zoom;

    let world_left   = camera_x - half_sw / zoom;
    let world_top    = camera_y - half_sh / zoom;
    let world_right  = camera_x + half_sw / zoom;
    let world_bottom = camera_y + half_sh / zoom;

    let tile_x_min = ((world_left - self.origin_x) / self.tile_size).floor().max(0.0) as usize;
    let tile_y_min = ((world_top - self.origin_y) / self.tile_size).floor().max(0.0) as usize;
    let tile_x_max = ((world_right - self.origin_x) / self.tile_size).ceil().max(0.0) as usize;
    let tile_y_max = ((world_bottom - self.origin_y) / self.tile_size).ceil().max(0.0) as usize;
    let tile_x_max = tile_x_max.min(self.width);
    let tile_y_max = tile_y_max.min(self.height);

    // Draw each layer in order (index 0 = furthest back).
    for layer in &self.layers {
        if !layer.visible {
            continue;
        }
        for ty in tile_y_min..tile_y_max {
            for tx in tile_x_min..tile_x_max {
                let tile = &layer.tiles[ty * self.width + tx];
                if tile.tile_type == TileType::Empty {
                    continue;
                }

                let world_tx = self.origin_x + tx as f64 * self.tile_size;
                let world_ty = self.origin_y + ty as f64 * self.tile_size;
                let screen_x = (world_tx - camera_x) * zoom + half_sw;
                let screen_y = (world_ty - camera_y) * zoom + half_sh;

                // Apply layer opacity to tile color.
                let mut color = tile.color;
                if layer.opacity < 1.0 {
                    color = color.with_alpha(
                        (color.a as f64 * layer.opacity).round() as u8
                    );
                }

                crate::rendering::shapes::fill_rect(
                    fb,
                    screen_x,
                    screen_y,
                    tile_px,
                    tile_px,
                    color,
                );
            }
        }
    }
}
```

### Migration Notes for Existing Code

Any code that accesses `tilemap.tiles` directly must be updated:
- `tilemap.tiles[i]` becomes `tilemap.tiles()[i]` or `tilemap.layers[0].tiles[i]`
- `tilemap.tiles.len()` becomes `tilemap.tiles().len()`
- `tilemap.tiles.iter()` becomes `tilemap.tiles().iter()`

This is a compile-time-caught change; `tiles` changes from a `pub Vec<Tile>` field to a method returning `&[Tile]`.

### Integration with Existing Patterns

- **Engine field** `pub tilemap: Option<TileMap>` -- unchanged type. Backward compatible.
- **Collision queries** (`is_solid`, `is_solid_at_world`) automatically scan all collidable layers. Game code that marks the "objects" layer as `collidable: false` gets terrain-only collision, which is the correct behavior for Trap Links (traps should not block movement).
- **RenderLayerStack** -- `TileMap` layers are internal (not to be confused with the entity `RenderLayerStack`). The tilemap renders as a single unit, typically before entities. Game code calls `tilemap.render(...)` once, and all internal layers draw in order.
- **SceneManager** -- when pushing a fight scene, the engine replaces `tilemap` with a new one-layer `TileMap` for the golf course. On pop, the overworld multi-layer tilemap is restored.

### Test Cases (8 tests)

1. **`new_creates_single_default_layer`**: `TileMap::new(10, 8, 32.0)` has `layer_count() == 1`, layer name is `"terrain"`, and `tiles().len() == 80`.

2. **`add_layer_returns_correct_index`**: After `new(5, 5, 16.0)`, calling `add_layer("objects")` returns `1`. Calling `add_layer("overlay")` returns `2`. `layer_count() == 3`.

3. **`layer_index_finds_by_name`**: After adding `"objects"` layer, `layer_index("objects") == Some(1)`, `layer_index("missing") == None`.

4. **`get_set_on_layer_round_trip`**: Set a solid tile on layer 1. Read it back via `get_on_layer(1, x, y)`. Confirm layer 0 at the same position is still empty.

5. **`is_solid_queries_all_collidable_layers`**: Place a Solid tile at (2,2) on layer 1 only. `is_solid(2, 2)` returns `true`. Mark layer 1 as `collidable: false`. Now `is_solid(2, 2)` returns `false`.

6. **`backward_compat_get_set_uses_layer_0`**: `set(3, 3, Tile::solid(RED))` via the non-layered API writes to layer 0. `get(3, 3)` reads from layer 0.

7. **`clear_resets_all_layers`**: Fill both layers with solid tiles. Call `clear()`. All layers should contain only Empty tiles.

8. **`fill_rect_on_layer_does_not_affect_other_layers`**: `fill_rect_on_layer(1, 0, 0, 3, 3, Tile::solid(RED))` fills layer 1. Layer 0 at (0,0) is still empty.

### Estimated Lines of Code

- `TileLayer` struct + impl: ~25 lines
- `TileMap` struct changes: ~5 lines (field rename)
- `TileMap::new` changes: ~5 lines
- New layer management methods (`add_layer`, `layer`, `layer_mut`, `layer_index`, `layer_count`): ~25 lines
- Layer-specific accessors (`get_on_layer`, `get_mut_on_layer`, `set_on_layer`, `fill_rect_on_layer`): ~35 lines
- Backward-compat wrappers (`tiles()`, `tiles_mut()`, modified `get`/`set`/`fill_rect`/`clear`/`solid_count`/`is_solid`): ~40 lines
- Modified `render`: ~55 lines (replaces existing ~65 lines)
- Tests: ~120 lines
- Migration fixups in other files referencing `.tiles`: ~10 lines

**Total: ~320 lines** (net new + modified)

---

## Gap 2: Sprite Sheet Support for Overworld Tiles

### Problem Statement

Each `Tile` already carries an `Option<u16>` field called `sprite_index`. However, `TileMap::render` ignores this field entirely and always draws a flat-color `fill_rect`. For Trap Links, the overworld needs visually rich terrain -- grass with variation, stone floor patterns, water animation frames, etc. -- all sourced from a sprite sheet.

The entity renderer (`systems/renderer.rs`) already draws `Visual::Sprite { sheet_id, tile_index }` using `SpriteSheet::draw_tile`. The tilemap needs a parallel path: when a tile has `sprite_index = Some(idx)`, render the sprite instead of the flat color.

### Current Code Under Modification

**File**: `/home/user/crusty/engine/crates/engine-core/src/tilemap.rs` -- the `Tile` struct and `TileMap::render` method.

**File**: `/home/user/crusty/engine/crates/engine-core/src/rendering/sprite.rs` -- `SpriteSheet::draw_tile` and `SpriteSheet::draw_tile_flipped`.

**File**: `/home/user/crusty/engine/crates/engine-core/src/engine.rs` -- where `tilemap.render(...)` is called (or where it should be called; currently tilemap rendering is not wired into the main render loop).

Current `Tile` (lines 13-18):
```rust
pub struct Tile {
    pub tile_type: TileType,
    pub color: Color,
    pub sprite_index: Option<u16>,
}
```

Current `SpriteSheet::draw_tile` signature (line 32):
```rust
pub fn draw_tile(&self, fb: &mut Framebuffer, index: u32, dx: i32, dy: i32)
```

Note: `SpriteSheet::draw_tile` has no scaling support. It blits at 1:1 pixel ratio. If the sprite sheet's `tile_w`/`tile_h` differ from `TileMap::tile_size * zoom`, tiles will be the wrong size. This must be addressed.

### Design Decisions

**Sprite sheet binding**: The `TileMap` struct gets an optional reference to a `SpriteSheet` (by index into `Engine::sprite_sheets`). This is a `u32` index, matching the pattern used by `Visual::Sprite { sheet_id }`.

**Per-layer sprite sheets**: Each `TileLayer` can optionally reference a different sprite sheet (e.g., terrain sheet vs. objects sheet). This is stored as `pub sprite_sheet_id: Option<u32>` on `TileLayer`.

**Fallback**: If a tile has `sprite_index = Some(idx)` but the layer has no sprite sheet (or the sheet ID is invalid), fall back to flat-color rendering. If `sprite_index = None`, always use flat-color rendering regardless of sprite sheet availability.

**Zoom scaling**: `SpriteSheet::draw_tile` blits at 1:1. When `zoom != 1.0`, tiles must be scaled. Two approaches:
- (a) Add a `draw_tile_scaled` method that performs nearest-neighbor scaling during blit.
- (b) Pre-scale the sprite sheet on load.

Approach (a) is chosen because zoom can change dynamically (CameraDirector zoom in/out) and because pre-scaling wastes memory for unused zoom levels.

### Struct / Enum / Trait Additions

#### Extended `TileLayer` (addition to Gap 1)

```rust
#[derive(Clone, Debug)]
pub struct TileLayer {
    pub name: String,
    pub tiles: Vec<Tile>,
    pub collidable: bool,
    pub visible: bool,
    pub opacity: f64,
    /// Index into Engine::sprite_sheets. If Some, tiles with sprite_index
    /// will be rendered from this sheet instead of as flat color.
    pub sprite_sheet_id: Option<u32>,
}
```

`TileLayer::new` sets `sprite_sheet_id: None` by default.

#### New method on `SpriteSheet`: `draw_tile_scaled`

```rust
// ---- rendering/sprite.rs additions ----

impl SpriteSheet {
    /// Draw a tile from the sprite sheet, scaled to fit a destination rectangle
    /// of `dest_w x dest_h` pixels using nearest-neighbor interpolation.
    /// Supports alpha blending. Skips fully transparent source pixels.
    pub fn draw_tile_scaled(
        &self,
        fb: &mut Framebuffer,
        index: u32,
        dx: i32,
        dy: i32,
        dest_w: u32,
        dest_h: u32,
    ) {
        if self.columns == 0 || dest_w == 0 || dest_h == 0 {
            return;
        }

        // If destination matches source dimensions exactly, use fast path.
        if dest_w == self.tile_w && dest_h == self.tile_h {
            self.draw_tile(fb, index, dx, dy);
            return;
        }

        let col = index % self.columns;
        let row = index / self.columns;
        let sx = col * self.tile_w;
        let sy = row * self.tile_h;

        if sx + self.tile_w > self.width || sy + self.tile_h > self.height {
            return;
        }

        // Nearest-neighbor scaling
        for py in 0..dest_h {
            let dest_y = dy + py as i32;
            // Map destination row to source row
            let src_row = (py as u64 * self.tile_h as u64 / dest_h as u64) as u32;
            for px in 0..dest_w {
                let dest_x = dx + px as i32;
                // Map destination col to source col
                let src_col = (px as u64 * self.tile_w as u64 / dest_w as u64) as u32;

                let src_x = sx + src_col;
                let src_y = sy + src_row;
                let src_idx = ((src_y * self.width + src_x) * 4) as usize;

                if src_idx + 3 >= self.pixels.len() {
                    continue;
                }

                let a = self.pixels[src_idx + 3];
                if a == 0 {
                    continue;
                }

                let color = Color::from_rgba(
                    self.pixels[src_idx],
                    self.pixels[src_idx + 1],
                    self.pixels[src_idx + 2],
                    a,
                );
                fb.set_pixel_blended(dest_x, dest_y, color);
            }
        }
    }
}
```

### Modified `TileMap::render` -- sprite-aware rendering

The `render` method signature is extended to accept `sprite_sheets`:

```rust
pub fn render(
    &self,
    fb: &mut crate::rendering::framebuffer::Framebuffer,
    camera_x: f64,
    camera_y: f64,
    zoom: f64,
    screen_width: u32,
    screen_height: u32,
    sprite_sheets: &[crate::rendering::sprite::SpriteSheet],
) {
    let sw = screen_width as f64;
    let sh = screen_height as f64;
    let half_sw = sw * 0.5;
    let half_sh = sh * 0.5;
    let tile_px = self.tile_size * zoom;
    let tile_px_u32 = tile_px.round() as u32;

    // Compute visible tile range (unchanged from current)
    let world_left   = camera_x - half_sw / zoom;
    let world_top    = camera_y - half_sh / zoom;
    let world_right  = camera_x + half_sw / zoom;
    let world_bottom = camera_y + half_sh / zoom;

    let tile_x_min = ((world_left - self.origin_x) / self.tile_size).floor().max(0.0) as usize;
    let tile_y_min = ((world_top - self.origin_y) / self.tile_size).floor().max(0.0) as usize;
    let tile_x_max = ((world_right - self.origin_x) / self.tile_size)
        .ceil().max(0.0) as usize;
    let tile_y_max = ((world_bottom - self.origin_y) / self.tile_size)
        .ceil().max(0.0) as usize;
    let tile_x_max = tile_x_max.min(self.width);
    let tile_y_max = tile_y_max.min(self.height);

    for layer in &self.layers {
        if !layer.visible {
            continue;
        }

        // Resolve sprite sheet for this layer (if any).
        let sheet: Option<&SpriteSheet> = layer.sprite_sheet_id
            .and_then(|id| sprite_sheets.get(id as usize));

        for ty in tile_y_min..tile_y_max {
            for tx in tile_x_min..tile_x_max {
                let tile = &layer.tiles[ty * self.width + tx];
                if tile.tile_type == TileType::Empty {
                    continue;
                }

                let world_tx = self.origin_x + tx as f64 * self.tile_size;
                let world_ty = self.origin_y + ty as f64 * self.tile_size;
                let screen_x = (world_tx - camera_x) * zoom + half_sw;
                let screen_y = (world_ty - camera_y) * zoom + half_sh;

                // Try sprite rendering first.
                let mut drew_sprite = false;
                if let (Some(sprite_idx), Some(s)) = (tile.sprite_index, sheet) {
                    s.draw_tile_scaled(
                        fb,
                        sprite_idx as u32,
                        screen_x.round() as i32,
                        screen_y.round() as i32,
                        tile_px_u32,
                        tile_px_u32,
                    );
                    drew_sprite = true;
                }

                // Fall back to flat color if no sprite was drawn.
                if !drew_sprite {
                    let mut color = tile.color;
                    if layer.opacity < 1.0 {
                        color = color.with_alpha(
                            (color.a as f64 * layer.opacity).round() as u8
                        );
                    }
                    crate::rendering::shapes::fill_rect(
                        fb, screen_x, screen_y, tile_px, tile_px, color,
                    );
                }
            }
        }
    }
}
```

### Engine Integration

**File**: `/home/user/crusty/engine/crates/engine-core/src/engine.rs`

In the rendering section of `tick()` (around line 466), add tilemap rendering before entity rendering:

```rust
// --- RENDERING ---
self.framebuffer.clear(self.config.background);
if let Some(ref starfield) = self.starfield {
    starfield.render(&mut self.framebuffer, &self.camera, self.frame);
}

// TileMap rendering (before entities so entities draw on top)
if let Some(ref tilemap) = self.tilemap {
    tilemap.render(
        &mut self.framebuffer,
        self.camera.x, self.camera.y,
        self.camera.zoom,
        self.width, self.height,
        &self.sprite_sheets,
    );
}

// Entity rendering
crate::systems::renderer::run_entities_only(
    &self.world, &mut self.framebuffer, &self.input, &self.camera,
    &self.sprite_sheets,
);
```

### Tile Sprite Construction Helpers

Add builder-style methods on `Tile` for sprite-bearing tiles:

```rust
// ---- tilemap.rs, impl Tile additions ----

impl Tile {
    /// Create a tile with a sprite index. Color is used as fallback if no
    /// sprite sheet is bound to the layer.
    pub fn with_sprite(mut self, index: u16) -> Self {
        self.sprite_index = Some(index);
        self
    }

    /// Convenience: create a walkable (Custom) tile with sprite.
    pub fn terrain(custom_id: u16, fallback_color: Color, sprite_index: u16) -> Self {
        Self {
            tile_type: TileType::Custom(custom_id),
            color: fallback_color,
            sprite_index: Some(sprite_index),
        }
    }
}
```

### Test Cases (7 tests)

1. **`tile_with_sprite_builder`**: `Tile::solid(Color::RED).with_sprite(42)` has `sprite_index == Some(42)`, `tile_type == Solid`, `color == RED`.

2. **`tile_terrain_constructor`**: `Tile::terrain(0, Color::GREEN, 5)` has `tile_type == Custom(0)`, `color == GREEN`, `sprite_index == Some(5)`.

3. **`render_falls_back_to_color_when_no_sheet`**: Create a tilemap with one solid tile having `sprite_index = Some(0)`. Render with an empty `sprite_sheets` slice. The framebuffer should contain the tile's flat color at the expected pixel, not be blank.

4. **`render_draws_sprite_when_sheet_available`**: Create a 1x1 tilemap at zoom=1.0 with `sprite_index = Some(0)` and a sprite sheet whose tile 0 is solid red. Render. The framebuffer should show red pixels at the tile position (from the sprite, not from `tile.color`).

5. **`draw_tile_scaled_1x_matches_draw_tile`**: Create a SpriteSheet (4x4 tiles, 8x8 tile size). Call `draw_tile(fb1, 0, 0, 0)` and `draw_tile_scaled(fb2, 0, 0, 0, 8, 8)` on separate framebuffers. Pixels should match exactly.

6. **`draw_tile_scaled_2x_doubles_dimensions`**: A 2x2 sprite tile (index 0, all red) scaled to 4x4. The 4x4 destination region should be entirely red.

7. **`draw_tile_scaled_skips_transparent_pixels`**: A 2x2 sprite tile where top-left pixel is transparent. Scaled to 4x4. The framebuffer background (black) should show through in the top-left quadrant.

### Estimated Lines of Code

- `SpriteSheet::draw_tile_scaled`: ~45 lines
- `Tile::with_sprite`, `Tile::terrain`: ~15 lines
- `TileLayer::sprite_sheet_id` field: ~3 lines
- Modified `TileMap::render` (sprite-aware): ~70 lines (replaces existing ~65 lines)
- Engine `tick()` tilemap render call: ~10 lines
- Tests: ~100 lines

**Total: ~243 lines** (net new + modified)

---

## Gap 6: Circular Arc Aim Line

### Problem Statement

In the Trap Links fight scene, when the player drags to aim a shot, a **trajectory preview** should appear showing the projected parabolic arc of the golf ball. The design document specifies (Section 4):

> "3 ghost dots spaced 0.1 s of simulated travel time apart. Each dot sized by predicted velocity at that moment (large=fast, small=slowing). Dots fade out (alpha: 255 -> 60) to show uncertainty at range."

The current renderer has a straight-line aim indicator (renderer.rs lines 117-133) that draws a simple `draw_line` from the player position along the drag vector. This is inadequate for a physics-driven game where the ball curves under gravity and drag.

The existing `GhostTrail` component is entity-attached -- it records the *past* trail of a moving entity's positions. What we need is a *forward-looking* trajectory preview that simulates future physics without spawning a real entity.

### Design Decisions

**Approach**: A standalone rendering utility that takes initial conditions (position, velocity, gravity, drag) and draws a series of dots along the predicted arc. This is a pure rendering function, not a component or system -- it lives in the `rendering` module.

**Physics model for preview**: Simplified Euler integration matching the engine's actual physics. Each step: `vel += gravity * dt`, `vel *= (1.0 - drag * dt)`, `pos += vel * dt`. This mirrors the integrator system.

**Dot rendering**: Each dot is a filled circle (`shapes::fill_circle`) with radius proportional to speed and alpha fading over distance.

**Bounce preview** (optional, noted in design doc): "Dots skip through Solid tiles (showing the ball WILL bounce there)." Bounce prediction requires a tilemap reference. The API supports an optional tilemap for bounce simulation. Without a tilemap, the arc is drawn without collision checks.

**Wall bounce model**: When a predicted position enters a solid tile, the velocity is reflected off the estimated wall normal (based on which face of the tile the trajectory crossed). This is a simplified approximation -- the real CCD collision system is more precise, but for a preview line, visual approximation is acceptable.

### Struct / Enum / Trait Additions

```rust
// ---- New file: rendering/aim_arc.rs ----

use super::color::Color;
use super::framebuffer::Framebuffer;
use super::shapes;

/// Configuration for drawing a trajectory preview arc.
#[derive(Clone, Debug)]
pub struct AimArcConfig {
    /// Number of dots to draw along the arc.
    pub dot_count: u32,
    /// Simulated time step between dots, in seconds.
    /// Default: 0.1 (each dot represents 0.1 s of travel).
    pub dot_interval: f64,
    /// Gravity acceleration vector (world units per second^2).
    /// For a top-down game: (0.0, 0.0). For side-view: (0.0, 600.0).
    pub gravity: (f64, f64),
    /// Linear drag coefficient. Applied as `vel *= (1.0 - drag * dt)` each step.
    /// Matches PhysicsMaterial.drag semantics.
    pub drag: f64,
    /// Minimum dot radius in pixels.
    pub min_dot_radius: f64,
    /// Maximum dot radius in pixels.
    pub max_dot_radius: f64,
    /// Color of the first dot (full alpha).
    pub color_start: Color,
    /// Color of the last dot (faded).
    pub color_end: Color,
    /// Number of sub-steps per dot interval for more accurate simulation.
    /// Higher = more accurate but slower. Default: 4.
    pub sub_steps: u32,
    /// If true, draw thin connecting lines between consecutive dots.
    pub draw_connecting_lines: bool,
    /// Thickness of connecting lines (if enabled). Default: 1.0.
    pub line_thickness: f64,
    /// Restitution (bounciness) for wall bounces. 0.0 = no bounce, 1.0 = perfect.
    pub restitution: f64,
    /// Maximum number of wall bounces to simulate. 0 = no bounce simulation.
    pub max_bounces: u32,
}

impl Default for AimArcConfig {
    fn default() -> Self {
        Self {
            dot_count: 3,
            dot_interval: 0.1,
            gravity: (0.0, 0.0),
            drag: 0.04,
            min_dot_radius: 2.0,
            max_dot_radius: 6.0,
            color_start: Color::from_rgba(255, 255, 255, 255),
            color_end: Color::from_rgba(255, 255, 255, 60),
            sub_steps: 4,
            draw_connecting_lines: false,
            line_thickness: 1.0,
            restitution: 0.8,
            max_bounces: 2,
        }
    }
}
```

### Core Rendering Function

```rust
// ---- rendering/aim_arc.rs continued ----

/// A single sampled point on the predicted trajectory.
#[derive(Clone, Debug)]
pub struct ArcPoint {
    /// World-space position.
    pub x: f64,
    pub y: f64,
    /// Speed at this point (world units per second).
    pub speed: f64,
    /// Fractional progress along the arc (0.0 = start, 1.0 = last dot).
    pub t: f64,
}

/// Compute the predicted trajectory arc points without rendering.
/// Useful for game logic (hazard detection, bounce analysis).
///
/// `start_x`, `start_y`: initial world-space position.
/// `vel_x`, `vel_y`: initial velocity in world units per second.
/// `config`: arc simulation parameters.
/// `tilemap`: optional tilemap for wall bounce simulation.
///
/// Returns a Vec of `ArcPoint`s, one per dot.
pub fn compute_arc(
    start_x: f64,
    start_y: f64,
    vel_x: f64,
    vel_y: f64,
    config: &AimArcConfig,
    tilemap: Option<&crate::tilemap::TileMap>,
) -> Vec<ArcPoint> {
    let mut points = Vec::with_capacity(config.dot_count as usize);
    let mut px = start_x;
    let mut py = start_y;
    let mut vx = vel_x;
    let mut vy = vel_y;
    let mut bounces_remaining = config.max_bounces;

    let sub_dt = config.dot_interval / config.sub_steps.max(1) as f64;

    for dot_i in 0..config.dot_count {
        // Simulate sub_steps of physics to reach the next dot position.
        for _ in 0..config.sub_steps.max(1) {
            // Apply gravity
            vx += config.gravity.0 * sub_dt;
            vy += config.gravity.1 * sub_dt;

            // Apply drag
            let drag_factor = 1.0 - config.drag * sub_dt;
            let drag_factor = drag_factor.max(0.0);
            vx *= drag_factor;
            vy *= drag_factor;

            // Predict next position
            let next_x = px + vx * sub_dt;
            let next_y = py + vy * sub_dt;

            // Wall bounce check (if tilemap provided and bounces remain)
            if let Some(tm) = tilemap {
                if bounces_remaining > 0 && tm.is_solid_at_world(next_x, next_y) {
                    // Determine bounce normal by checking adjacent cells
                    let (nx, ny) = estimate_bounce_normal(tm, px, py, next_x, next_y);
                    // Reflect velocity: v' = v - 2*(v . n)*n
                    let dot = vx * nx + vy * ny;
                    vx = (vx - 2.0 * dot * nx) * config.restitution;
                    vy = (vy - 2.0 * dot * ny) * config.restitution;
                    bounces_remaining -= 1;
                    // Don't update position into the wall; keep current position
                } else {
                    px = next_x;
                    py = next_y;
                }
            } else {
                px = next_x;
                py = next_y;
            }
        }

        let speed = (vx * vx + vy * vy).sqrt();
        let t = if config.dot_count > 1 {
            dot_i as f64 / (config.dot_count - 1) as f64
        } else {
            0.0
        };

        points.push(ArcPoint { x: px, y: py, speed, t });
    }

    points
}

/// Estimate the wall normal when a predicted position enters a solid tile.
/// Uses axis-aligned checks: test X and Y movement independently to determine
/// which axis caused the penetration.
fn estimate_bounce_normal(
    tm: &crate::tilemap::TileMap,
    from_x: f64, from_y: f64,
    to_x: f64, to_y: f64,
) -> (f64, f64) {
    let solid_x = tm.is_solid_at_world(to_x, from_y);
    let solid_y = tm.is_solid_at_world(from_x, to_y);

    match (solid_x, solid_y) {
        (true, false) => {
            // Hit a vertical wall (normal is horizontal)
            if to_x > from_x { (-1.0, 0.0) } else { (1.0, 0.0) }
        }
        (false, true) => {
            // Hit a horizontal wall (normal is vertical)
            if to_y > from_y { (0.0, -1.0) } else { (0.0, 1.0) }
        }
        (true, true) => {
            // Corner hit -- reflect both axes
            let nx = if to_x > from_x { -1.0 } else { 1.0 };
            let ny = if to_y > from_y { -1.0 } else { 1.0 };
            let len = (nx * nx + ny * ny).sqrt();
            (nx / len, ny / len)
        }
        (false, false) => {
            // Shouldn't happen (caller verified solid at to_x,to_y), but fallback
            (0.0, -1.0)
        }
    }
}

/// Render a trajectory preview arc onto the framebuffer.
///
/// `start_x`, `start_y`: world-space starting position of the arc (ball position).
/// `vel_x`, `vel_y`: initial velocity (world units/sec).
/// `config`: rendering and simulation parameters.
/// `tilemap`: optional tilemap reference for bounce simulation.
/// `camera_x`, `camera_y`, `zoom`: camera transform for world-to-screen conversion.
/// `screen_width`, `screen_height`: viewport dimensions.
pub fn draw_aim_arc(
    fb: &mut Framebuffer,
    start_x: f64,
    start_y: f64,
    vel_x: f64,
    vel_y: f64,
    config: &AimArcConfig,
    tilemap: Option<&crate::tilemap::TileMap>,
    camera_x: f64,
    camera_y: f64,
    zoom: f64,
    screen_width: u32,
    screen_height: u32,
) {
    let points = compute_arc(start_x, start_y, vel_x, vel_y, config, tilemap);

    if points.is_empty() {
        return;
    }

    let half_sw = screen_width as f64 * 0.5;
    let half_sh = screen_height as f64 * 0.5;

    // Find max speed for radius normalization.
    let max_speed = points.iter()
        .map(|p| p.speed)
        .fold(0.0_f64, f64::max)
        .max(0.001); // avoid division by zero

    let mut prev_sx: Option<f64> = None;
    let mut prev_sy: Option<f64> = None;

    for point in &points {
        // World to screen
        let sx = (point.x - camera_x) * zoom + half_sw;
        let sy = (point.y - camera_y) * zoom + half_sh;

        // Interpolate color
        let color = Color::lerp(config.color_start, config.color_end, point.t);

        // Interpolate radius based on speed
        let speed_ratio = point.speed / max_speed;
        let radius = config.min_dot_radius
            + (config.max_dot_radius - config.min_dot_radius) * speed_ratio;

        // Draw connecting line to previous dot
        if config.draw_connecting_lines {
            if let (Some(px), Some(py)) = (prev_sx, prev_sy) {
                let line_color = color.with_alpha(color.a / 2);
                if config.line_thickness <= 1.0 {
                    shapes::draw_line(fb, px, py, sx, sy, line_color);
                } else {
                    shapes::draw_line_thick(
                        fb, px, py, sx, sy, config.line_thickness, line_color,
                    );
                }
            }
        }

        // Draw dot
        shapes::fill_circle(fb, sx, sy, radius, color);

        prev_sx = Some(sx);
        prev_sy = Some(sy);
    }
}
```

### Module Registration

**File**: `/home/user/crusty/engine/crates/engine-core/src/rendering/mod.rs`

Add:
```rust
pub mod aim_arc;
```

### Integration with Fight Scene Renderer

The fight scene game code (custom Rust module) calls `draw_aim_arc` each frame during the aim phase. This replaces the straight-line aim indicator currently in `renderer.rs` (lines 117-133).

The existing aim line code in `renderer.rs` should be gated behind a feature check or removed entirely when the fight scene is active. The fight scene's custom render callback would call:

```rust
use crate::rendering::aim_arc::{draw_aim_arc, AimArcConfig};

// During aim phase:
if is_aiming {
    let power = (drag_distance / 120.0).min(1.0);
    let max_force = 900.0;
    let vel_x = aim_direction_x * power * max_force;
    let vel_y = aim_direction_y * power * max_force;

    let config = AimArcConfig {
        dot_count: 3,
        dot_interval: 0.1,
        gravity: (0.0, 0.0),  // top-down: no gravity
        drag: 0.04,           // match ball's PhysicsMaterial.drag
        min_dot_radius: 3.0,
        max_dot_radius: 8.0,
        color_start: Color::WHITE,
        color_end: Color::from_rgba(255, 255, 255, 60),
        sub_steps: 4,
        draw_connecting_lines: false,
        line_thickness: 1.0,
        restitution: 0.8,
        max_bounces: 2,
    };

    draw_aim_arc(
        &mut fb,
        ball_x, ball_y,
        vel_x, vel_y,
        &config,
        tilemap.as_ref(),  // pass tilemap for bounce preview
        camera_x, camera_y, zoom,
        screen_width, screen_height,
    );
}
```

### Hazard Detection via `compute_arc`

The design document states: "Color: white dots normally; red dots when any dot lands in a hazard zone." Game code can use `compute_arc` to check each point against hazard zones:

```rust
let points = aim_arc::compute_arc(
    ball_x, ball_y, vel_x, vel_y, &config, tilemap.as_ref(),
);

let any_in_hazard = points.iter().any(|p| {
    // Check if point is in a sand/water zone
    if let Some(tm) = tilemap.as_ref() {
        if let Some((tx, ty)) = tm.world_to_tile(p.x, p.y) {
            if let Some(tile) = tm.get(tx, ty) {
                matches!(tile.tile_type,
                    TileType::Custom(4) | TileType::Custom(3) // sand, water
                )
            } else { false }
        } else { false }
    } else { false }
});

let mut config = config.clone();
if any_in_hazard {
    config.color_start = Color::RED;
    config.color_end = Color::from_rgba(255, 0, 0, 60);
}
```

### Test Cases (8 tests)

1. **`compute_arc_no_gravity_no_drag_straight_line`**: Initial velocity (100, 0), gravity (0, 0), drag 0.0, 3 dots at 0.1s intervals. Points should be at approximately (10, 0), (20, 0), (30, 0) offset from start. Verify positions within epsilon.

2. **`compute_arc_with_gravity_curves_downward`**: Initial velocity (100, 0), gravity (0, 600), drag 0.0, 3 dots. Each successive point should have increasing Y coordinate. The Y delta between dot 2 and dot 3 should be greater than between dot 1 and dot 2 (acceleration).

3. **`compute_arc_with_drag_decelerates`**: Initial velocity (100, 0), gravity (0, 0), drag 0.5, 3 dots. Each point's `speed` field should be less than the previous. The spacing between successive dots should decrease.

4. **`compute_arc_bounces_off_solid_tile`**: Create a 10x10 tilemap with a solid wall at column 5. Fire a ball from (16, 80) with velocity (200, 0) (heading right). With max_bounces=1 and restitution=1.0, the arc should show the ball reversing X direction after reaching column 5. At least one dot should have `x < start_x + 5*tile_size`.

5. **`compute_arc_respects_max_bounces`**: Same setup as test 4 but with `max_bounces = 0`. The arc should NOT bounce -- dots continue into/past the wall.

6. **`compute_arc_single_dot`**: `dot_count = 1`. Should return exactly one `ArcPoint` with `t = 0.0`.

7. **`aim_arc_config_default_sensible`**: `AimArcConfig::default()` should have `dot_count = 3`, `drag = 0.04`, `sub_steps = 4`, `min_dot_radius = 2.0`, `max_dot_radius = 6.0`.

8. **`draw_aim_arc_renders_dots_to_framebuffer`**: Create a 100x100 framebuffer cleared to black. Call `draw_aim_arc` with a rightward velocity, no gravity, zoom=1.0, camera at origin. Verify that at least 3 non-black pixels exist along the expected trajectory Y-row (confirming dots were drawn).

### Estimated Lines of Code

- `AimArcConfig` struct + Default impl: ~50 lines
- `ArcPoint` struct: ~10 lines
- `compute_arc` function: ~55 lines
- `estimate_bounce_normal` helper: ~25 lines
- `draw_aim_arc` function: ~55 lines
- Module registration (`mod.rs`): ~1 line
- Tests: ~150 lines

**Total: ~346 lines**

---

## Cross-Gap Integration Summary

### Render Order in Engine `tick()`

The final render order after all three gaps are implemented:

```
1. framebuffer.clear(background)
2. starfield.render(...)                    // optional parallax stars
3. tilemap.render(..., sprite_sheets)       // Gap 1+2: multi-layer, sprite-aware
4. renderer::run_entities_only(...)         // entity sprites/shapes
5. aim_arc::draw_aim_arc(...)               // Gap 6: fight scene only
6. particles.render(...)                    // particle effects
7. debug_render::run(...)                   // debug overlays (if enabled)
8. render_hud()                             // HUD elements
9. screen_fx.apply(...)                     // screen effects
10. transition.apply(...)                    // scene transitions
11. post_fx::apply(...)                      // post-processing
```

Steps 3 and 5 are new. Step 5 is conditional (fight scene only, during aim phase).

### Shared Dependencies

- Gaps 1 and 2 both modify `tilemap.rs` and `TileMap::render`. They should be implemented together in a single PR.
- Gap 2 depends on Gap 1 (`TileLayer::sprite_sheet_id` requires the `TileLayer` struct from Gap 1).
- Gap 6 depends on Gap 1 indirectly: `compute_arc` accepts `Option<&TileMap>` for bounce simulation, which uses `is_solid_at_world`. The multi-layer `is_solid` change in Gap 1 means bounces correctly query all collidable layers.

### Files Modified

| File | Gap 1 | Gap 2 | Gap 6 |
|------|-------|-------|-------|
| `tilemap.rs` | Major rewrite | Modify render, add Tile methods | Read-only (used by compute_arc) |
| `rendering/sprite.rs` | -- | Add `draw_tile_scaled` | -- |
| `rendering/aim_arc.rs` | -- | -- | New file |
| `rendering/mod.rs` | -- | -- | Add `pub mod aim_arc` |
| `engine.rs` | -- | Add tilemap render call in tick() | -- |

### Total Estimated Lines of Code

| Gap | New/Modified | Tests | Total |
|-----|-------------|-------|-------|
| Gap 1: Multi-layer TileMap | ~200 | ~120 | ~320 |
| Gap 2: Sprite sheet tiles | ~143 | ~100 | ~243 |
| Gap 6: Arc aim line | ~196 | ~150 | ~346 |
| **Grand Total** | **~539** | **~370** | **~909** |
