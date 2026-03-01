use std::collections::HashMap;
use crate::ecs::Entity;

/// Spatial hash grid for efficient proximity queries.
/// Entities are inserted by position; queries return entities in overlapping cells.
#[derive(Clone, Debug)]
pub struct SpatialHashGrid {
    /// The size of each grid cell in world units. Retained for introspection and debug output.
    pub cell_size: f64,
    inv_cell_size: f64,
    cells: HashMap<(i32, i32), Vec<Entity>>,
}

impl SpatialHashGrid {
    pub fn new(cell_size: f64) -> Self {
        assert!(cell_size > 0.0, "cell_size must be positive");
        Self {
            cell_size,
            inv_cell_size: 1.0 / cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    fn cell_key(&self, x: f64, y: f64) -> (i32, i32) {
        ((x * self.inv_cell_size).floor() as i32,
         (y * self.inv_cell_size).floor() as i32)
    }

    /// Insert an entity at a single point.
    pub fn insert(&mut self, entity: Entity, x: f64, y: f64) {
        let key = self.cell_key(x, y);
        self.cells.entry(key).or_default().push(entity);
    }

    /// Insert an entity that spans an AABB (inserted into all overlapping cells).
    pub fn insert_aabb(&mut self, entity: Entity, min_x: f64, min_y: f64, max_x: f64, max_y: f64) {
        let (cx0, cy0) = self.cell_key(min_x, min_y);
        let (cx1, cy1) = self.cell_key(max_x, max_y);
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                self.cells.entry((cx, cy)).or_default().push(entity);
            }
        }
    }

    /// Query all entities within cells that overlap the given radius from (cx, cy).
    /// Note: returns candidates — caller should do fine distance check if needed.
    pub fn query_radius(&self, cx: f64, cy: f64, radius: f64) -> Vec<Entity> {
        self.query_aabb(cx - radius, cy - radius, cx + radius, cy + radius)
    }

    /// Query all entities in cells overlapping the given AABB. Deduplicates results.
    pub fn query_aabb(&self, min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Vec<Entity> {
        let (cx0, cy0) = self.cell_key(min_x, min_y);
        let (cx1, cy1) = self.cell_key(max_x, max_y);
        let mut results = Vec::new();
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                if let Some(entities) = self.cells.get(&(cx, cy)) {
                    results.extend(entities);
                }
            }
        }
        // Deduplicate: sort by Entity ID then dedup
        results.sort_by_key(|e: &Entity| e.0);
        results.dedup();
        results
    }

    /// Find the nearest entity to (x, y) within max_radius.
    /// `positions` is a callback that returns the position of an entity.
    pub fn nearest<F>(&self, x: f64, y: f64, max_radius: f64, positions: F) -> Option<Entity>
    where
        F: Fn(Entity) -> Option<(f64, f64)>,
    {
        let candidates = self.query_radius(x, y, max_radius);
        let mut best: Option<(Entity, f64)> = None;
        for entity in candidates {
            if let Some((ex, ey)) = positions(entity) {
                let dx = ex - x;
                let dy = ey - y;
                let dist_sq = dx * dx + dy * dy;
                let max_r_sq = max_radius * max_radius;
                if dist_sq <= max_r_sq {
                    match best {
                        None => best = Some((entity, dist_sq)),
                        Some((_, bd)) if dist_sq < bd => best = Some((entity, dist_sq)),
                        _ => {}
                    }
                }
            }
        }
        best.map(|(e, _)| e)
    }

    /// Number of cells currently occupied.
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Total entity references stored across all cells.
    pub fn total_entries(&self) -> usize {
        self.cells.values().map(|v| v.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_grid() {
        let grid = SpatialHashGrid::new(100.0);
        assert_eq!(grid.cell_count(), 0);
        assert_eq!(grid.total_entries(), 0);
    }

    #[test]
    fn insert_and_query_radius() {
        let mut grid = SpatialHashGrid::new(100.0);
        grid.insert(Entity(1), 50.0, 50.0);
        let results = grid.query_radius(50.0, 50.0, 10.0);
        assert!(results.contains(&Entity(1)));
    }

    #[test]
    fn insert_and_query_aabb() {
        let mut grid = SpatialHashGrid::new(100.0);
        grid.insert(Entity(1), 50.0, 50.0);
        grid.insert(Entity(2), 150.0, 150.0);
        let results = grid.query_aabb(0.0, 0.0, 99.0, 99.0);
        assert!(results.contains(&Entity(1)));
        assert!(!results.contains(&Entity(2)));
    }

    #[test]
    fn query_radius_empty_for_distant() {
        let mut grid = SpatialHashGrid::new(50.0);
        grid.insert(Entity(1), 0.0, 0.0);
        let results = grid.query_radius(500.0, 500.0, 10.0);
        assert!(!results.contains(&Entity(1)));
    }

    #[test]
    fn multiple_entities_query_returns_subset() {
        let mut grid = SpatialHashGrid::new(100.0);
        grid.insert(Entity(1), 10.0, 10.0);
        grid.insert(Entity(2), 20.0, 20.0);
        grid.insert(Entity(3), 500.0, 500.0);
        let results = grid.query_radius(15.0, 15.0, 50.0);
        assert!(results.contains(&Entity(1)));
        assert!(results.contains(&Entity(2)));
        assert!(!results.contains(&Entity(3)));
    }

    #[test]
    fn deduplication_works() {
        let mut grid = SpatialHashGrid::new(50.0);
        // Entity spans multiple cells
        grid.insert_aabb(Entity(1), 0.0, 0.0, 200.0, 200.0);
        let results = grid.query_aabb(0.0, 0.0, 200.0, 200.0);
        // Should appear exactly once despite being in many cells
        assert_eq!(results.iter().filter(|&&e| e == Entity(1)).count(), 1);
    }

    #[test]
    fn insert_aabb_large_entity() {
        let mut grid = SpatialHashGrid::new(50.0);
        grid.insert_aabb(Entity(1), 0.0, 0.0, 150.0, 150.0);
        // Should be findable from various query locations
        assert!(!grid.query_radius(25.0, 25.0, 10.0).is_empty());
        assert!(!grid.query_radius(125.0, 125.0, 10.0).is_empty());
    }

    #[test]
    fn nearest_returns_closest() {
        let mut grid = SpatialHashGrid::new(100.0);
        grid.insert(Entity(1), 10.0, 10.0);
        grid.insert(Entity(2), 5.0, 5.0);
        grid.insert(Entity(3), 50.0, 50.0);

        let positions = |e: Entity| -> Option<(f64, f64)> {
            match e.0 {
                1 => Some((10.0, 10.0)),
                2 => Some((5.0, 5.0)),
                3 => Some((50.0, 50.0)),
                _ => None,
            }
        };
        let result = grid.nearest(0.0, 0.0, 200.0, positions);
        assert_eq!(result, Some(Entity(2)));
    }

    #[test]
    fn nearest_returns_none_when_empty() {
        let grid = SpatialHashGrid::new(100.0);
        let result = grid.nearest(0.0, 0.0, 100.0, |_| None);
        assert!(result.is_none());
    }

    #[test]
    fn nearest_respects_max_radius() {
        let mut grid = SpatialHashGrid::new(100.0);
        grid.insert(Entity(1), 50.0, 50.0);
        let result = grid.nearest(0.0, 0.0, 10.0, |e| {
            if e == Entity(1) { Some((50.0, 50.0)) } else { None }
        });
        assert!(result.is_none());
    }

    #[test]
    fn clear_empties_grid() {
        let mut grid = SpatialHashGrid::new(100.0);
        grid.insert(Entity(1), 10.0, 10.0);
        grid.insert(Entity(2), 20.0, 20.0);
        assert!(grid.cell_count() > 0);
        grid.clear();
        assert_eq!(grid.cell_count(), 0);
        assert_eq!(grid.total_entries(), 0);
    }

    #[test]
    fn different_cell_sizes() {
        let mut small = SpatialHashGrid::new(10.0);
        let mut large = SpatialHashGrid::new(1000.0);
        small.insert(Entity(1), 5.0, 5.0);
        small.insert(Entity(2), 15.0, 15.0);
        large.insert(Entity(1), 5.0, 5.0);
        large.insert(Entity(2), 15.0, 15.0);

        // With small cells, entities are in different cells
        // With large cells, both are in the same cell
        let small_results = small.query_aabb(0.0, 0.0, 9.0, 9.0);
        let large_results = large.query_aabb(0.0, 0.0, 9.0, 9.0);
        assert_eq!(small_results.len(), 1);
        assert_eq!(large_results.len(), 2);
    }

    #[test]
    fn entity_on_cell_boundary() {
        let mut grid = SpatialHashGrid::new(100.0);
        grid.insert(Entity(1), 100.0, 100.0); // exactly on boundary
        // Should be findable from the cell it falls into
        let results = grid.query_radius(100.0, 100.0, 1.0);
        assert!(results.contains(&Entity(1)));
    }
}
