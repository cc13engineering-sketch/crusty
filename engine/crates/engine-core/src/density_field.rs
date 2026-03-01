/// ENGINE MODULE: DensityField
/// Continuous 2D scalar field with diffusion, decay, gradient sampling,
/// and bilinear interpolation. Used for pheromones, nutrients, heat maps, etc.

/// A 2D scalar field stored on a regular grid.
#[derive(Clone, Debug)]
pub struct DensityField {
    pub width: usize,
    pub height: usize,
    pub cell_size: f64,
    pub origin_x: f64,
    pub origin_y: f64,
    data: Vec<f64>,
    scratch: Vec<f64>,
    pub diffusion_rate: f64,
    pub decay_rate: f64,
    pub clamp_min: f64,
    pub clamp_max: f64,
}

impl DensityField {
    pub fn new(width: usize, height: usize, cell_size: f64) -> Self {
        let size = width * height;
        Self {
            width, height, cell_size,
            origin_x: 0.0, origin_y: 0.0,
            data: vec![0.0; size],
            scratch: vec![0.0; size],
            diffusion_rate: 0.1,
            decay_rate: 0.99,
            clamp_min: 0.0,
            clamp_max: f64::MAX,
        }
    }

    pub fn with_origin(mut self, x: f64, y: f64) -> Self {
        self.origin_x = x;
        self.origin_y = y;
        self
    }

    fn world_to_cell(&self, wx: f64, wy: f64) -> (f64, f64) {
        ((wx - self.origin_x) / self.cell_size, (wy - self.origin_y) / self.cell_size)
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// Get the raw value at a cell coordinate.
    pub fn get_cell(&self, x: usize, y: usize) -> f64 {
        if x >= self.width || y >= self.height { return 0.0; }
        self.data[self.idx(x, y)]
    }

    /// Set the raw value at a cell coordinate.
    pub fn set_cell(&mut self, x: usize, y: usize, value: f64) {
        if x >= self.width || y >= self.height { return; }
        let idx = self.idx(x, y);
        self.data[idx] = value.clamp(self.clamp_min, self.clamp_max);
    }

    /// Sample the field at a world-space position using bilinear interpolation.
    pub fn sample(&self, wx: f64, wy: f64) -> f64 {
        let (fx, fy) = self.world_to_cell(wx, wy);
        let x0 = fx.floor() as i64;
        let y0 = fy.floor() as i64;
        let tx = fx - x0 as f64;
        let ty = fy - y0 as f64;

        let get = |x: i64, y: i64| -> f64 {
            if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
                0.0
            } else {
                self.data[y as usize * self.width + x as usize]
            }
        };

        let v00 = get(x0, y0);
        let v10 = get(x0 + 1, y0);
        let v01 = get(x0, y0 + 1);
        let v11 = get(x0 + 1, y0 + 1);

        let a = v00 + tx * (v10 - v00);
        let b = v01 + tx * (v11 - v01);
        a + ty * (b - a)
    }

    /// Deposit a value at a world-space position, distributed to nearest cells.
    pub fn deposit(&mut self, wx: f64, wy: f64, amount: f64) {
        let (fx, fy) = self.world_to_cell(wx, wy);
        let x0 = fx.floor() as i64;
        let y0 = fy.floor() as i64;
        let tx = fx - x0 as f64;
        let ty = fy - y0 as f64;

        let contributions = [
            (x0, y0, (1.0 - tx) * (1.0 - ty)),
            (x0 + 1, y0, tx * (1.0 - ty)),
            (x0, y0 + 1, (1.0 - tx) * ty),
            (x0 + 1, y0 + 1, tx * ty),
        ];

        for &(cx, cy, w) in &contributions {
            if cx >= 0 && cy >= 0 && (cx as usize) < self.width && (cy as usize) < self.height {
                let idx = cy as usize * self.width + cx as usize;
                self.data[idx] = (self.data[idx] + amount * w).min(self.clamp_max);
            }
        }
    }

    /// Consume (subtract) a value at a world-space position.
    /// Returns the amount actually consumed.
    pub fn consume(&mut self, wx: f64, wy: f64, amount: f64) -> f64 {
        let (fx, fy) = self.world_to_cell(wx, wy);
        let cx = fx.round() as i64;
        let cy = fy.round() as i64;
        if cx < 0 || cy < 0 || cx as usize >= self.width || cy as usize >= self.height {
            return 0.0;
        }
        let idx = cy as usize * self.width + cx as usize;
        let available = self.data[idx];
        let consumed = amount.min(available);
        self.data[idx] -= consumed;
        consumed
    }

    /// Get the gradient (direction of steepest increase) at a world position.
    pub fn gradient(&self, wx: f64, wy: f64) -> (f64, f64) {
        let h = self.cell_size * 0.5;
        let dx = self.sample(wx + h, wy) - self.sample(wx - h, wy);
        let dy = self.sample(wx, wy + h) - self.sample(wx, wy - h);
        (dx / (2.0 * h), dy / (2.0 * h))
    }

    /// Run one diffusion + decay step.
    pub fn step(&mut self, dt: f64) {
        let w = self.width;
        let h = self.height;
        let diff = self.diffusion_rate * dt;

        // Diffusion: average with neighbors
        self.scratch.copy_from_slice(&self.data);
        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                let center = self.scratch[idx];

                let mut neighbor_sum = 0.0;
                let mut neighbor_count = 0u32;

                if x > 0 { neighbor_sum += self.scratch[idx - 1]; neighbor_count += 1; }
                if x + 1 < w { neighbor_sum += self.scratch[idx + 1]; neighbor_count += 1; }
                if y > 0 { neighbor_sum += self.scratch[idx - w]; neighbor_count += 1; }
                if y + 1 < h { neighbor_sum += self.scratch[idx + w]; neighbor_count += 1; }

                if neighbor_count > 0 {
                    let avg = neighbor_sum / neighbor_count as f64;
                    self.data[idx] = center + diff * (avg - center);
                }
            }
        }

        // Decay
        let decay = self.decay_rate.powf(dt);
        for v in &mut self.data {
            *v *= decay;
            if *v < self.clamp_min { *v = self.clamp_min; }
        }
    }

    /// Total value across all cells.
    pub fn total(&self) -> f64 {
        self.data.iter().sum()
    }

    /// Maximum value in any cell.
    pub fn max_value(&self) -> f64 {
        self.data.iter().cloned().fold(0.0, f64::max)
    }

    /// Clear all values to zero.
    pub fn clear(&mut self) {
        self.data.fill(0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_field_all_zeros() {
        let field = DensityField::new(10, 10, 1.0);
        assert_eq!(field.total(), 0.0);
    }

    #[test]
    fn set_and_get_cell() {
        let mut field = DensityField::new(10, 10, 1.0);
        field.set_cell(5, 5, 42.0);
        assert_eq!(field.get_cell(5, 5), 42.0);
        assert_eq!(field.get_cell(0, 0), 0.0);
    }

    #[test]
    fn deposit_at_cell_center() {
        let mut field = DensityField::new(10, 10, 1.0);
        field.deposit(5.0, 5.0, 100.0);
        // Depositing at exact cell center should put most in cell (5,5)
        assert!(field.get_cell(5, 5) > 0.0);
        assert!(field.total() > 90.0); // most of 100 should be deposited
    }

    #[test]
    fn deposit_between_cells() {
        let mut field = DensityField::new(10, 10, 1.0);
        field.deposit(5.5, 5.5, 100.0);
        // Should be distributed among 4 cells
        assert!(field.get_cell(5, 5) > 0.0);
        assert!(field.get_cell(6, 5) > 0.0);
        assert!(field.get_cell(5, 6) > 0.0);
        assert!(field.get_cell(6, 6) > 0.0);
    }

    #[test]
    fn sample_bilinear() {
        let mut field = DensityField::new(10, 10, 1.0);
        field.set_cell(0, 0, 100.0);
        field.set_cell(1, 0, 0.0);
        let mid = field.sample(0.5, 0.0);
        assert!((mid - 50.0).abs() < 1.0); // halfway between 100 and 0
    }

    #[test]
    fn consume_reduces_value() {
        let mut field = DensityField::new(10, 10, 1.0);
        field.set_cell(3, 3, 50.0);
        let consumed = field.consume(3.0, 3.0, 20.0);
        assert_eq!(consumed, 20.0);
        assert_eq!(field.get_cell(3, 3), 30.0);
    }

    #[test]
    fn consume_limited_by_available() {
        let mut field = DensityField::new(10, 10, 1.0);
        field.set_cell(3, 3, 5.0);
        let consumed = field.consume(3.0, 3.0, 20.0);
        assert_eq!(consumed, 5.0);
        assert_eq!(field.get_cell(3, 3), 0.0);
    }

    #[test]
    fn gradient_points_toward_higher() {
        let mut field = DensityField::new(10, 10, 1.0);
        // Place value at (7,5), sample gradient at (6.0,5.0) where bilinear
        // interpolation can detect the difference between left and right neighbors
        field.set_cell(7, 5, 100.0);
        let (gx, _gy) = field.gradient(6.5, 5.0);
        assert!(gx > 0.0, "Gradient should point toward higher value (right), got gx={}", gx);
    }

    #[test]
    fn step_diffuses_values() {
        let mut field = DensityField::new(10, 10, 1.0);
        field.diffusion_rate = 1.0;
        field.decay_rate = 1.0; // no decay
        field.set_cell(5, 5, 100.0);

        let initial_max = field.max_value();
        field.step(1.0);
        let after_max = field.max_value();

        assert!(after_max < initial_max, "Peak should decrease after diffusion");
        // Neighbors should have gained value
        assert!(field.get_cell(4, 5) > 0.0 || field.get_cell(6, 5) > 0.0);
    }

    #[test]
    fn step_decay_reduces_total() {
        let mut field = DensityField::new(5, 5, 1.0);
        field.diffusion_rate = 0.0;
        field.decay_rate = 0.5;
        field.set_cell(2, 2, 100.0);

        field.step(1.0);
        assert!(field.get_cell(2, 2) < 100.0);
        assert!(field.get_cell(2, 2) > 0.0);
    }

    #[test]
    fn clear_resets_all() {
        let mut field = DensityField::new(5, 5, 1.0);
        field.set_cell(1, 1, 50.0);
        field.set_cell(3, 3, 30.0);
        field.clear();
        assert_eq!(field.total(), 0.0);
    }

    #[test]
    fn out_of_bounds_safe() {
        let mut field = DensityField::new(5, 5, 1.0);
        assert_eq!(field.get_cell(100, 100), 0.0);
        field.set_cell(100, 100, 50.0); // should be silently ignored
        assert_eq!(field.sample(-10.0, -10.0), 0.0);
    }

    #[test]
    fn clamp_max_respected() {
        let mut field = DensityField::new(5, 5, 1.0);
        field.clamp_max = 50.0;
        field.set_cell(2, 2, 100.0);
        assert_eq!(field.get_cell(2, 2), 50.0);
    }
}
