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

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Basic insert and query ----

    #[test]
    fn insert_single_entity_query_returns_it() {
        let mut grid = SpatialGrid::new(10.0);
        // Insert entity 0 with AABB from (2,2) to (4,4)
        grid.insert(0, 2.0, 2.0, 4.0, 4.0);

        let mut results = Vec::new();
        // Query a region that overlaps the entity
        grid.query(1.0, 1.0, 5.0, 5.0, &mut results);
        assert_eq!(results, vec![0]);
    }

    #[test]
    fn insert_multiple_entities_query_returns_correct_subset() {
        let mut grid = SpatialGrid::new(10.0);
        // Entity 0 at (0,0)-(5,5) -> cell (0,0)
        grid.insert(0, 0.0, 0.0, 5.0, 5.0);
        // Entity 1 at (15,15)-(18,18) -> cell (1,1)
        grid.insert(1, 15.0, 15.0, 18.0, 18.0);
        // Entity 2 at (2,2)-(4,4) -> cell (0,0)
        grid.insert(2, 2.0, 2.0, 4.0, 4.0);

        let mut results = Vec::new();

        // Query region covering cell (0,0) only
        grid.query(0.0, 0.0, 5.0, 5.0, &mut results);
        assert!(results.contains(&0));
        assert!(results.contains(&2));
        assert!(!results.contains(&1));

        // Query region covering cell (1,1) only
        grid.query(15.0, 15.0, 18.0, 18.0, &mut results);
        assert_eq!(results, vec![1]);
    }

    #[test]
    fn query_empty_region_returns_nothing() {
        let mut grid = SpatialGrid::new(10.0);
        // Insert entity at (0,0)-(5,5)
        grid.insert(0, 0.0, 0.0, 5.0, 5.0);

        let mut results = Vec::new();
        // Query a region far away
        grid.query(100.0, 100.0, 105.0, 105.0, &mut results);
        assert!(results.is_empty());
    }

    #[test]
    fn entity_spanning_multiple_cells_appears_in_all_cell_queries() {
        let mut grid = SpatialGrid::new(10.0);
        // Entity spans from (-5, -5) to (5, 5), crossing four cells:
        // cell (-1,-1), (-1,0), (0,-1), (0,0)
        grid.insert(0, -5.0, -5.0, 5.0, 5.0);

        let mut results = Vec::new();

        // Query only cell (-1,-1): region (-10,-10) to (-1,-1)
        grid.query(-10.0, -10.0, -1.0, -1.0, &mut results);
        assert_eq!(results, vec![0]);

        // Query only cell (0,0): region (1,1) to (9,9)
        grid.query(1.0, 1.0, 9.0, 9.0, &mut results);
        assert_eq!(results, vec![0]);
    }

    #[test]
    fn deduplication_entity_in_overlapping_cells_appears_once() {
        let mut grid = SpatialGrid::new(10.0);
        // Entity spans multiple cells
        grid.insert(0, -5.0, -5.0, 5.0, 5.0);

        let mut results = Vec::new();
        // Query a region that overlaps multiple cells where entity 0 is stored
        grid.query(-10.0, -10.0, 10.0, 10.0, &mut results);
        // Entity should appear exactly once thanks to dedup
        assert_eq!(results.iter().filter(|&&x| x == 0).count(), 1);
    }

    #[test]
    fn deduplication_multiple_entities_all_unique() {
        let mut grid = SpatialGrid::new(10.0);
        // Two entities both spanning multiple cells
        grid.insert(0, -5.0, -5.0, 5.0, 5.0);
        grid.insert(1, -3.0, -3.0, 3.0, 3.0);

        let mut results = Vec::new();
        grid.query(-10.0, -10.0, 10.0, 10.0, &mut results);
        // Both should appear exactly once
        assert_eq!(results.iter().filter(|&&x| x == 0).count(), 1);
        assert_eq!(results.iter().filter(|&&x| x == 1).count(), 1);
        assert_eq!(results.len(), 2);
    }

    // ---- collider_aabb_half ----

    #[test]
    fn collider_aabb_half_circle() {
        use super::super::super::components::ColliderShape;
        let shape = ColliderShape::Circle { radius: 15.0 };
        let (hw, hh) = collider_aabb_half(&shape);
        assert!((hw - 15.0).abs() < 1e-10);
        assert!((hh - 15.0).abs() < 1e-10);
    }

    #[test]
    fn collider_aabb_half_rect() {
        use super::super::super::components::ColliderShape;
        let shape = ColliderShape::Rect { half_width: 20.0, half_height: 30.0 };
        let (hw, hh) = collider_aabb_half(&shape);
        assert!((hw - 20.0).abs() < 1e-10);
        assert!((hh - 30.0).abs() < 1e-10);
    }

    // ---- Cell size behavior ----

    #[test]
    fn large_cell_size_fewer_cells() {
        let mut grid = SpatialGrid::new(100.0);
        // Entity at (5,5)-(15,15): with cell_size=100, all in cell (0,0)
        grid.insert(0, 5.0, 5.0, 15.0, 15.0);
        // Entity at (50,50)-(60,60): also in cell (0,0) since 60 < 100
        grid.insert(1, 50.0, 50.0, 60.0, 60.0);

        let mut results = Vec::new();
        // Query a small region in cell (0,0) — both entities are in this cell
        grid.query(0.0, 0.0, 1.0, 1.0, &mut results);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&0));
        assert!(results.contains(&1));
    }

    #[test]
    fn small_cell_size_more_precise() {
        let mut grid = SpatialGrid::new(1.0);
        // Entity at (5,5)-(5.5,5.5): fits in cell (5,5)
        grid.insert(0, 5.0, 5.0, 5.5, 5.5);
        // Entity at (8,8)-(8.5,8.5): fits in cell (8,8)
        grid.insert(1, 8.0, 8.0, 8.5, 8.5);

        let mut results = Vec::new();
        // Query only around entity 0
        grid.query(5.0, 5.0, 5.5, 5.5, &mut results);
        assert_eq!(results, vec![0]);

        // Query only around entity 1
        grid.query(8.0, 8.0, 8.5, 8.5, &mut results);
        assert_eq!(results, vec![1]);
    }

    #[test]
    fn small_cell_size_entity_spans_many_cells() {
        let mut grid = SpatialGrid::new(1.0);
        // Entity from (0,0) to (3,3) spans cells (0,0) through (3,3) = 16 cells
        grid.insert(0, 0.0, 0.0, 3.0, 3.0);

        let mut results = Vec::new();
        // Query just cell (2,2)
        grid.query(2.0, 2.0, 2.5, 2.5, &mut results);
        assert_eq!(results, vec![0]);

        // Query just cell (0,0)
        grid.query(0.0, 0.0, 0.5, 0.5, &mut results);
        assert_eq!(results, vec![0]);
    }

    #[test]
    fn query_clears_output_vector() {
        let mut grid = SpatialGrid::new(10.0);
        grid.insert(0, 0.0, 0.0, 5.0, 5.0);

        let mut results = vec![99, 100, 101]; // pre-filled with junk
        grid.query(0.0, 0.0, 5.0, 5.0, &mut results);
        // Should have cleared old data and only contain entity 0
        assert_eq!(results, vec![0]);
    }

    #[test]
    fn empty_grid_query_returns_nothing() {
        let grid = SpatialGrid::new(10.0);
        let mut results = Vec::new();
        grid.query(0.0, 0.0, 100.0, 100.0, &mut results);
        assert!(results.is_empty());
    }

    #[test]
    fn negative_coordinates() {
        let mut grid = SpatialGrid::new(10.0);
        grid.insert(0, -15.0, -15.0, -5.0, -5.0);

        let mut results = Vec::new();
        grid.query(-20.0, -20.0, -3.0, -3.0, &mut results);
        assert_eq!(results, vec![0]);

        // Query positive region should miss
        grid.query(0.0, 0.0, 10.0, 10.0, &mut results);
        assert!(results.is_empty());
    }
}
