use crate::rendering::color::Color;

/// The type of a tile — determines collision behavior.
#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    Empty,
    Solid,
    Platform,    // solid from above only
    Custom(u16), // user-defined type ID
}

/// A single cell in a TileMap grid.
#[derive(Clone, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub color: Color,
    pub sprite_index: Option<u16>, // optional sprite sheet frame
}

impl Tile {
    pub fn empty() -> Self {
        Self {
            tile_type: TileType::Empty,
            color: Color::TRANSPARENT,
            sprite_index: None,
        }
    }

    pub fn solid(color: Color) -> Self {
        Self {
            tile_type: TileType::Solid,
            color,
            sprite_index: None,
        }
    }

    pub fn platform(color: Color) -> Self {
        Self {
            tile_type: TileType::Platform,
            color,
            sprite_index: None,
        }
    }

    pub fn custom(id: u16, color: Color) -> Self {
        Self {
            tile_type: TileType::Custom(id),
            color,
            sprite_index: None,
        }
    }
}

/// A 2D grid of tiles stored in row-major order.
///
/// Tile `(x, y)` is stored at index `y * width + x`.
/// The grid's world-space top-left corner is `(origin_x, origin_y)`.
#[derive(Clone, Debug)]
pub struct TileMap {
    pub width: usize,    // grid width in tiles
    pub height: usize,   // grid height in tiles
    pub tile_size: f64,  // pixel size of each (square) tile
    pub tiles: Vec<Tile>, // row-major: tiles[y * width + x]
    pub origin_x: f64,   // world-space left edge
    pub origin_y: f64,   // world-space top edge
}

impl TileMap {
    /// Create a new TileMap filled with empty tiles.
    pub fn new(width: usize, height: usize, tile_size: f64) -> Self {
        let tiles = (0..width * height).map(|_| Tile::empty()).collect();
        Self {
            width,
            height,
            tile_size,
            tiles,
            origin_x: 0.0,
            origin_y: 0.0,
        }
    }

    #[inline]
    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    /// Get a reference to the tile at grid position `(x, y)`.
    /// Returns `None` if the coordinates are out of bounds.
    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.index(x, y).map(|i| &self.tiles[i])
    }

    /// Get a mutable reference to the tile at grid position `(x, y)`.
    /// Returns `None` if the coordinates are out of bounds.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.index(x, y).map(|i| &mut self.tiles[i])
    }

    /// Set the tile at grid position `(x, y)`. Silently ignored if out of bounds.
    pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
        if let Some(i) = self.index(x, y) {
            self.tiles[i] = tile;
        }
    }

    /// Returns true if the tile at `(x, y)` is `Solid` (not Platform, not Custom, not Empty).
    pub fn is_solid(&self, x: usize, y: usize) -> bool {
        match self.get(x, y) {
            Some(tile) => tile.tile_type == TileType::Solid,
            None => false,
        }
    }

    /// Convert a world-space position to tile grid coordinates.
    /// Returns `None` if the point is outside the map bounds.
    pub fn world_to_tile(&self, wx: f64, wy: f64) -> Option<(usize, usize)> {
        let lx = wx - self.origin_x;
        let ly = wy - self.origin_y;
        if lx < 0.0 || ly < 0.0 {
            return None;
        }
        let tx = (lx / self.tile_size) as usize;
        let ty = (ly / self.tile_size) as usize;
        if tx < self.width && ty < self.height {
            Some((tx, ty))
        } else {
            None
        }
    }

    /// Convert a tile grid position to the world-space center of that tile.
    pub fn tile_to_world(&self, tx: usize, ty: usize) -> (f64, f64) {
        let half = self.tile_size * 0.5;
        let wx = self.origin_x + tx as f64 * self.tile_size + half;
        let wy = self.origin_y + ty as f64 * self.tile_size + half;
        (wx, wy)
    }

    /// Returns true if the tile at the world-space position is `Solid`.
    pub fn is_solid_at_world(&self, wx: f64, wy: f64) -> bool {
        match self.world_to_tile(wx, wy) {
            Some((tx, ty)) => self.is_solid(tx, ty),
            None => false,
        }
    }

    /// Fill a rectangular region with copies of `tile`.
    /// Clamps to map bounds; silently ignores out-of-bounds regions.
    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, tile: Tile) {
        let x_end = (x + w).min(self.width);
        let y_end = (y + h).min(self.height);
        for ty in y..y_end {
            for tx in x..x_end {
                let i = ty * self.width + tx;
                self.tiles[i] = tile.clone();
            }
        }
    }

    /// Reset all tiles to `Empty`.
    pub fn clear(&mut self) {
        for tile in self.tiles.iter_mut() {
            *tile = Tile::empty();
        }
    }

    /// Count the number of `Solid` tiles in the map.
    pub fn solid_count(&self) -> usize {
        self.tiles.iter().filter(|t| t.tile_type == TileType::Solid).count()
    }

    /// Render visible tiles to a framebuffer using camera position and zoom.
    ///
    /// `camera_x` / `camera_y`: world-space coordinates of the screen center.
    /// `zoom`: pixels per world unit (> 1.0 zooms in).
    /// Only tiles whose screen rectangles overlap the viewport are drawn.
    pub fn render(
        &self,
        fb: &mut crate::rendering::framebuffer::Framebuffer,
        camera_x: f64,
        camera_y: f64,
        zoom: f64,
        screen_width: u32,
        screen_height: u32,
    ) {
        let sw = screen_width as f64;
        let sh = screen_height as f64;
        let half_sw = sw * 0.5;
        let half_sh = sh * 0.5;

        // Pixel size of one tile at this zoom level.
        let tile_px = self.tile_size * zoom;

        // World-space rectangle visible on screen.
        let world_left   = camera_x - half_sw / zoom;
        let world_top    = camera_y - half_sh / zoom;
        let world_right  = camera_x + half_sw / zoom;
        let world_bottom = camera_y + half_sh / zoom;

        // Tile range that overlaps the visible area.
        let tile_x_min = {
            let v = ((world_left - self.origin_x) / self.tile_size).floor() as i64;
            v.max(0) as usize
        };
        let tile_y_min = {
            let v = ((world_top - self.origin_y) / self.tile_size).floor() as i64;
            v.max(0) as usize
        };
        let tile_x_max = {
            let v = ((world_right - self.origin_x) / self.tile_size).ceil() as i64;
            (v.max(0) as usize).min(self.width)
        };
        let tile_y_max = {
            let v = ((world_bottom - self.origin_y) / self.tile_size).ceil() as i64;
            (v.max(0) as usize).min(self.height)
        };

        for ty in tile_y_min..tile_y_max {
            for tx in tile_x_min..tile_x_max {
                let tile = &self.tiles[ty * self.width + tx];
                if tile.tile_type == TileType::Empty {
                    continue;
                }

                // Top-left world-space corner of this tile.
                let world_tx = self.origin_x + tx as f64 * self.tile_size;
                let world_ty = self.origin_y + ty as f64 * self.tile_size;

                // Convert to screen space.
                let screen_x = (world_tx - camera_x) * zoom + half_sw;
                let screen_y = (world_ty - camera_y) * zoom + half_sh;

                crate::rendering::shapes::fill_rect(
                    fb,
                    screen_x,
                    screen_y,
                    tile_px,
                    tile_px,
                    tile.color,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendering::color::Color;

    // ── construction ──────────────────────────────────────────────────────────

    #[test]
    fn new_creates_correct_dimensions() {
        let tm = TileMap::new(10, 8, 32.0);
        assert_eq!(tm.width, 10);
        assert_eq!(tm.height, 8);
        assert_eq!(tm.tile_size, 32.0);
        assert_eq!(tm.tiles.len(), 80);
    }

    #[test]
    fn new_all_empty() {
        let tm = TileMap::new(5, 5, 16.0);
        for tile in &tm.tiles {
            assert_eq!(tile.tile_type, TileType::Empty);
        }
    }

    // ── get / set ─────────────────────────────────────────────────────────────

    #[test]
    fn get_set_round_trip() {
        let mut tm = TileMap::new(4, 4, 32.0);
        tm.set(2, 3, Tile::solid(Color::RED));
        let t = tm.get(2, 3).unwrap();
        assert_eq!(t.tile_type, TileType::Solid);
        assert_eq!(t.color, Color::RED);
    }

    #[test]
    fn get_out_of_bounds_returns_none() {
        let tm = TileMap::new(4, 4, 32.0);
        assert!(tm.get(4, 0).is_none());
        assert!(tm.get(0, 4).is_none());
        assert!(tm.get(100, 100).is_none());
    }

    #[test]
    fn set_out_of_bounds_is_silent() {
        let mut tm = TileMap::new(4, 4, 32.0);
        // Should not panic.
        tm.set(99, 99, Tile::solid(Color::RED));
    }

    #[test]
    fn get_mut_allows_mutation() {
        let mut tm = TileMap::new(4, 4, 32.0);
        tm.set(1, 1, Tile::solid(Color::BLUE));
        if let Some(tile) = tm.get_mut(1, 1) {
            tile.color = Color::GREEN;
        }
        assert_eq!(tm.get(1, 1).unwrap().color, Color::GREEN);
    }

    // ── is_solid ──────────────────────────────────────────────────────────────

    #[test]
    fn empty_tile_is_not_solid() {
        let tm = TileMap::new(4, 4, 32.0);
        assert!(!tm.is_solid(0, 0));
    }

    #[test]
    fn solid_tile_is_solid() {
        let mut tm = TileMap::new(4, 4, 32.0);
        tm.set(2, 2, Tile::solid(Color::WHITE));
        assert!(tm.is_solid(2, 2));
    }

    #[test]
    fn platform_tile_is_not_solid() {
        let mut tm = TileMap::new(4, 4, 32.0);
        tm.set(1, 1, Tile::platform(Color::GREEN));
        assert!(!tm.is_solid(1, 1));
    }

    #[test]
    fn is_solid_out_of_bounds_returns_false() {
        let tm = TileMap::new(4, 4, 32.0);
        assert!(!tm.is_solid(10, 10));
    }

    // ── coordinate conversion ────────────────────────────────────────────────

    #[test]
    fn world_to_tile_basic() {
        let tm = TileMap::new(10, 10, 32.0);
        // Point (16, 16) is inside tile (0, 0).
        assert_eq!(tm.world_to_tile(16.0, 16.0), Some((0, 0)));
        // Point (32, 0) is inside tile (1, 0).
        assert_eq!(tm.world_to_tile(32.0, 0.0), Some((1, 0)));
        // Point (64, 96) -> tile (2, 3).
        assert_eq!(tm.world_to_tile(64.0, 96.0), Some((2, 3)));
    }

    #[test]
    fn world_to_tile_with_origin() {
        let mut tm = TileMap::new(5, 5, 32.0);
        tm.origin_x = 100.0;
        tm.origin_y = 200.0;
        assert_eq!(tm.world_to_tile(100.0, 200.0), Some((0, 0)));
        assert_eq!(tm.world_to_tile(132.0, 200.0), Some((1, 0)));
    }

    #[test]
    fn world_to_tile_out_of_bounds() {
        let tm = TileMap::new(4, 4, 32.0);
        // Negative coords.
        assert!(tm.world_to_tile(-1.0, 0.0).is_none());
        assert!(tm.world_to_tile(0.0, -1.0).is_none());
        // Beyond map edge (4 tiles * 32 px = 128).
        assert!(tm.world_to_tile(128.0, 0.0).is_none());
        assert!(tm.world_to_tile(0.0, 128.0).is_none());
    }

    #[test]
    fn tile_to_world_center() {
        let tm = TileMap::new(10, 10, 32.0);
        // Tile (0,0) center should be (16, 16).
        let (wx, wy) = tm.tile_to_world(0, 0);
        assert!((wx - 16.0).abs() < 1e-10);
        assert!((wy - 16.0).abs() < 1e-10);
        // Tile (2, 3) center: x=2*32+16=80, y=3*32+16=112.
        let (wx, wy) = tm.tile_to_world(2, 3);
        assert!((wx - 80.0).abs() < 1e-10);
        assert!((wy - 112.0).abs() < 1e-10);
    }

    // ── is_solid_at_world ────────────────────────────────────────────────────

    #[test]
    fn is_solid_at_world_hit() {
        let mut tm = TileMap::new(5, 5, 32.0);
        tm.set(1, 1, Tile::solid(Color::WHITE));
        // Center of tile (1,1) = (48, 48).
        assert!(tm.is_solid_at_world(48.0, 48.0));
    }

    #[test]
    fn is_solid_at_world_miss() {
        let tm = TileMap::new(5, 5, 32.0);
        assert!(!tm.is_solid_at_world(48.0, 48.0));
    }

    // ── fill_rect ────────────────────────────────────────────────────────────

    #[test]
    fn fill_rect_sets_region() {
        let mut tm = TileMap::new(8, 8, 16.0);
        tm.fill_rect(2, 2, 3, 2, Tile::solid(Color::RED));
        for ty in 2..4 {
            for tx in 2..5 {
                assert_eq!(tm.get(tx, ty).unwrap().tile_type, TileType::Solid,
                    "Expected solid at ({}, {})", tx, ty);
            }
        }
        // Adjacent cells should still be empty.
        assert_eq!(tm.get(1, 2).unwrap().tile_type, TileType::Empty);
        assert_eq!(tm.get(5, 2).unwrap().tile_type, TileType::Empty);
    }

    #[test]
    fn fill_rect_clamps_to_bounds() {
        let mut tm = TileMap::new(4, 4, 16.0);
        // Should not panic even though rect extends past the edge.
        tm.fill_rect(3, 3, 10, 10, Tile::solid(Color::BLUE));
        assert_eq!(tm.get(3, 3).unwrap().tile_type, TileType::Solid);
    }

    // ── clear ────────────────────────────────────────────────────────────────

    #[test]
    fn clear_resets_all_tiles() {
        let mut tm = TileMap::new(4, 4, 32.0);
        tm.fill_rect(0, 0, 4, 4, Tile::solid(Color::GREEN));
        assert_eq!(tm.solid_count(), 16);
        tm.clear();
        assert_eq!(tm.solid_count(), 0);
        for tile in &tm.tiles {
            assert_eq!(tile.tile_type, TileType::Empty);
        }
    }

    // ── solid_count ──────────────────────────────────────────────────────────

    #[test]
    fn solid_count_accurate() {
        let mut tm = TileMap::new(5, 5, 16.0);
        assert_eq!(tm.solid_count(), 0);
        tm.set(0, 0, Tile::solid(Color::WHITE));
        tm.set(1, 1, Tile::solid(Color::WHITE));
        tm.set(2, 2, Tile::platform(Color::GREEN)); // platform, not solid
        assert_eq!(tm.solid_count(), 2);
    }

    // ── Tile constructors ─────────────────────────────────────────────────────

    #[test]
    fn tile_custom_has_correct_id() {
        let t = Tile::custom(42, Color::BLUE);
        assert_eq!(t.tile_type, TileType::Custom(42));
    }
}
