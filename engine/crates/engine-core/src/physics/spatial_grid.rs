use std::collections::HashMap;

/// Uniform spatial grid for broad-phase collision detection.
/// Entities are inserted by their AABB; queries return candidate indices
/// for any cell overlapping a given AABB.
pub struct SpatialGrid {
    inv_cell_size: f64,
    cells: HashMap<(i32, i32), Vec<usize>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f64) -> Self {
        Self {
            inv_cell_size: 1.0 / cell_size,
            cells: HashMap::new(),
        }
    }

    fn cell_coord(&self, v: f64) -> i32 {
        (v * self.inv_cell_size).floor() as i32
    }

    /// Insert an entity index with the given axis-aligned bounding box.
    pub fn insert(&mut self, index: usize, min_x: f64, min_y: f64, max_x: f64, max_y: f64) {
        let cx0 = self.cell_coord(min_x);
        let cy0 = self.cell_coord(min_y);
        let cx1 = self.cell_coord(max_x);
        let cy1 = self.cell_coord(max_y);
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                self.cells.entry((cx, cy)).or_default().push(index);
            }
        }
    }

    /// Query all entity indices whose AABB overlaps the given region.
    /// Returns indices with possible duplicates removed.
    pub fn query(&self, min_x: f64, min_y: f64, max_x: f64, max_y: f64, out: &mut Vec<usize>) {
        out.clear();
        let cx0 = self.cell_coord(min_x);
        let cy0 = self.cell_coord(min_y);
        let cx1 = self.cell_coord(max_x);
        let cy1 = self.cell_coord(max_y);
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                if let Some(indices) = self.cells.get(&(cx, cy)) {
                    for &idx in indices {
                        out.push(idx);
                    }
                }
            }
        }
        out.sort_unstable();
        out.dedup();
    }
}

/// Compute the AABB half-extents for a collider shape.
pub fn collider_aabb_half(shape: &super::super::components::ColliderShape) -> (f64, f64) {
    use super::super::components::ColliderShape;
    match shape {
        ColliderShape::Circle { radius } => (*radius, *radius),
        ColliderShape::Rect { half_width, half_height } => (*half_width, *half_height),
    }
}
