/// ENGINE MODULE: ProceduralGen
/// Seeded RNG, 2D value noise, cellular automata, and dungeon generation.

use crate::tilemap::{TileMap, Tile, TileType};
use crate::rendering::color::Color;

/// Deterministic seeded RNG (xorshift64).
#[derive(Clone, Debug)]
pub struct SeededRng {
    state: u64,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() as f64) / (u64::MAX as f64)
    }

    pub fn range_f64(&mut self, min: f64, max: f64) -> f64 {
        min + self.next_f64() * (max - min)
    }

    pub fn range_i32(&mut self, min: i32, max: i32) -> i32 {
        if max <= min { return min; }
        let range = (max as i64 - min as i64 + 1) as u64;
        (min as i64 + (self.next_u64() % range) as i64) as i32
    }

    pub fn chance(&mut self, probability: f64) -> bool {
        self.next_f64() < probability
    }
}

/// 2D value noise with configurable octaves.
#[derive(Clone, Debug)]
pub struct Noise2D {
    pub seed: u64,
    pub octaves: u32,
    pub frequency: f64,
    pub persistence: f64,
    pub lacunarity: f64,
}

impl Noise2D {
    pub fn new(seed: u64) -> Self {
        Self {
            seed, octaves: 4, frequency: 0.05,
            persistence: 0.5, lacunarity: 2.0,
        }
    }

    /// Hash-based value noise at integer coordinates.
    fn hash(&self, ix: i64, iy: i64, seed: u64) -> f64 {
        let mut h = seed;
        h = h.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        h ^= ix as u64;
        h = h.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        h ^= iy as u64;
        h = h.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (h as f64) / (u64::MAX as f64)
    }

    /// Bilinear interpolation of value noise.
    fn noise_single(&self, x: f64, y: f64, seed: u64) -> f64 {
        let ix = x.floor() as i64;
        let iy = y.floor() as i64;
        let fx = x - ix as f64;
        let fy = y - iy as f64;

        let v00 = self.hash(ix, iy, seed);
        let v10 = self.hash(ix + 1, iy, seed);
        let v01 = self.hash(ix, iy + 1, seed);
        let v11 = self.hash(ix + 1, iy + 1, seed);

        let sx = fx * fx * (3.0 - 2.0 * fx); // smoothstep
        let sy = fy * fy * (3.0 - 2.0 * fy);

        let a = v00 + sx * (v10 - v00);
        let b = v01 + sx * (v11 - v01);
        a + sy * (b - a)
    }

    /// Sample multi-octave noise at (x, y), returns 0.0..1.0.
    pub fn sample(&self, x: f64, y: f64) -> f64 {
        let mut total = 0.0;
        let mut amplitude = 1.0;
        let mut freq = self.frequency;
        let mut max_amplitude = 0.0;

        for octave in 0..self.octaves {
            total += self.noise_single(x * freq, y * freq, self.seed.wrapping_add(octave as u64)) * amplitude;
            max_amplitude += amplitude;
            amplitude *= self.persistence;
            freq *= self.lacunarity;
        }

        total / max_amplitude
    }

    /// Fill a TileMap using a threshold: values below threshold → Empty, above → Solid.
    pub fn fill_tilemap(&self, tilemap: &mut TileMap, threshold: f64, solid_color: Color) {
        for ty in 0..tilemap.height {
            for tx in 0..tilemap.width {
                let value = self.sample(tx as f64, ty as f64);
                if value > threshold {
                    tilemap.set(tx, ty, Tile::solid(solid_color));
                }
            }
        }
    }
}

/// Cellular automata cave generation.
/// Fills the tilemap randomly, then iterates smoothing rules.
pub fn cellular_automata(
    tilemap: &mut TileMap,
    rng: &mut SeededRng,
    fill_chance: f64,
    iterations: u32,
    birth_threshold: u32,
    death_threshold: u32,
    solid_color: Color,
) {
    let w = tilemap.width;
    let h = tilemap.height;

    // Random fill
    for y in 0..h {
        for x in 0..w {
            if rng.chance(fill_chance) {
                tilemap.set(x, y, Tile::solid(solid_color));
            }
        }
    }

    // Automata iterations
    for _ in 0..iterations {
        let mut new_solid = vec![false; w * h];

        for y in 0..h {
            for x in 0..w {
                let neighbors = count_solid_neighbors(tilemap, x, y);
                let is_solid = tilemap.is_solid(x, y);

                new_solid[y * w + x] = if is_solid {
                    neighbors >= death_threshold
                } else {
                    neighbors >= birth_threshold
                };
            }
        }

        for y in 0..h {
            for x in 0..w {
                if new_solid[y * w + x] {
                    tilemap.set(x, y, Tile::solid(solid_color));
                } else {
                    tilemap.set(x, y, Tile { tile_type: TileType::Empty, color: Color::BLACK, sprite_index: None });
                }
            }
        }
    }
}

fn count_solid_neighbors(tilemap: &TileMap, x: usize, y: usize) -> u32 {
    let mut count = 0u32;
    for dy in -1i32..=1 {
        for dx in -1i32..=1 {
            if dx == 0 && dy == 0 { continue; }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 || nx >= tilemap.width as i32 || ny >= tilemap.height as i32 {
                count += 1; // Treat out-of-bounds as solid
            } else if tilemap.is_solid(nx as usize, ny as usize) {
                count += 1;
            }
        }
    }
    count
}

/// Room-and-corridor dungeon generation.
/// Returns a list of room rects (x, y, w, h).
pub fn dungeon_rooms(
    tilemap: &mut TileMap,
    rng: &mut SeededRng,
    min_rooms: u32,
    max_rooms: u32,
    min_room_size: u32,
    max_room_size: u32,
    solid_color: Color,
) -> Vec<(usize, usize, usize, usize)> {
    let tw = tilemap.width;
    let th = tilemap.height;

    // Fill everything solid
    for y in 0..th {
        for x in 0..tw {
            tilemap.set(x, y, Tile::solid(solid_color));
        }
    }

    let num_rooms = rng.range_i32(min_rooms as i32, max_rooms as i32) as u32;
    let mut rooms = Vec::new();

    for _ in 0..num_rooms * 3 {
        if rooms.len() >= num_rooms as usize { break; }

        let rw = rng.range_i32(min_room_size as i32, max_room_size as i32) as usize;
        let rh = rng.range_i32(min_room_size as i32, max_room_size as i32) as usize;
        if rw + 2 >= tw || rh + 2 >= th { continue; }

        let rx = rng.range_i32(1, (tw - rw - 1) as i32) as usize;
        let ry = rng.range_i32(1, (th - rh - 1) as i32) as usize;

        // Check overlap with existing rooms (with 1-tile margin)
        let overlaps = rooms.iter().any(|&(ex, ey, ew, eh): &(usize, usize, usize, usize)| {
            rx < ex + ew + 1 && rx + rw + 1 > ex && ry < ey + eh + 1 && ry + rh + 1 > ey
        });
        if overlaps { continue; }

        // Carve room
        let empty = Tile { tile_type: TileType::Empty, color: Color::BLACK, sprite_index: None };
        for y in ry..ry + rh {
            for x in rx..rx + rw {
                tilemap.set(x, y, empty.clone());
            }
        }

        // Connect to previous room with L-shaped corridor
        if let Some(&(px, py, pw, ph)) = rooms.last() {
            let cx1 = rx + rw / 2;
            let cy1 = ry + rh / 2;
            let cx2 = px + pw / 2;
            let cy2 = py + ph / 2;

            // Horizontal then vertical
            let (start_x, end_x) = if cx1 < cx2 { (cx1, cx2) } else { (cx2, cx1) };
            for x in start_x..=end_x {
                if x < tw { tilemap.set(x, cy1, empty.clone()); }
            }
            let (start_y, end_y) = if cy1 < cy2 { (cy1, cy2) } else { (cy2, cy1) };
            for y in start_y..=end_y {
                if y < th { tilemap.set(cx2, y, empty.clone()); }
            }
        }

        rooms.push((rx, ry, rw, rh));
    }

    rooms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_rng_deterministic() {
        let mut a = SeededRng::new(42);
        let mut b = SeededRng::new(42);
        for _ in 0..10 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn seeded_rng_range() {
        let mut rng = SeededRng::new(123);
        for _ in 0..100 {
            let v = rng.range_f64(0.0, 1.0);
            assert!(v >= 0.0 && v <= 1.0);
        }
    }

    #[test]
    fn seeded_rng_range_i32() {
        let mut rng = SeededRng::new(456);
        for _ in 0..100 {
            let v = rng.range_i32(0, 10);
            assert!(v >= 0 && v <= 10);
        }
    }

    #[test]
    fn seeded_rng_chance() {
        let mut rng = SeededRng::new(789);
        let mut trues = 0;
        for _ in 0..1000 {
            if rng.chance(0.5) { trues += 1; }
        }
        // Should be roughly 500 ± 100
        assert!(trues > 300 && trues < 700);
    }

    #[test]
    fn noise_in_range() {
        let noise = Noise2D::new(42);
        for x in 0..20 {
            for y in 0..20 {
                let v = noise.sample(x as f64, y as f64);
                assert!(v >= 0.0 && v <= 1.0, "Noise out of range: {}", v);
            }
        }
    }

    #[test]
    fn noise_deterministic() {
        let noise = Noise2D::new(42);
        let v1 = noise.sample(5.5, 3.7);
        let v2 = noise.sample(5.5, 3.7);
        assert_eq!(v1, v2);
    }

    #[test]
    fn noise_varies() {
        let noise = Noise2D::new(42);
        let v1 = noise.sample(0.0, 0.0);
        let v2 = noise.sample(10.0, 10.0);
        assert!((v1 - v2).abs() > 0.001, "Noise should vary across space");
    }

    #[test]
    fn fill_tilemap_with_noise() {
        let mut tm = TileMap::new(20, 20, 16.0);
        let noise = Noise2D::new(42);
        noise.fill_tilemap(&mut tm, 0.5, Color::WHITE);
        // Some tiles should be solid, some empty
        let solid = tm.solid_count();
        assert!(solid > 0 && solid < 400);
    }

    #[test]
    fn cellular_automata_generates_caves() {
        let mut tm = TileMap::new(20, 20, 16.0);
        let mut rng = SeededRng::new(42);
        cellular_automata(&mut tm, &mut rng, 0.45, 4, 5, 4, Color::WHITE);
        let solid = tm.solid_count();
        assert!(solid > 0 && solid < 400);
    }

    #[test]
    fn dungeon_rooms_generates_rooms() {
        let mut tm = TileMap::new(40, 30, 16.0);
        let mut rng = SeededRng::new(42);
        let rooms = dungeon_rooms(&mut tm, &mut rng, 3, 6, 3, 6, Color::WHITE);
        assert!(rooms.len() >= 2, "Should generate at least 2 rooms, got {}", rooms.len());
        // Rooms should have carved empty space
        let solid = tm.solid_count();
        assert!(solid < 40 * 30, "Dungeon should have some empty space");
    }

    #[test]
    fn dungeon_rooms_corridors_connected() {
        let mut tm = TileMap::new(50, 50, 16.0);
        let mut rng = SeededRng::new(100);
        let rooms = dungeon_rooms(&mut tm, &mut rng, 4, 8, 4, 8, Color::WHITE);
        // Each room center should be walkable
        for &(rx, ry, rw, rh) in &rooms {
            let cx = rx + rw / 2;
            let cy = ry + rh / 2;
            assert!(!tm.is_solid(cx, cy), "Room center at ({}, {}) should be walkable", cx, cy);
        }
    }
}
