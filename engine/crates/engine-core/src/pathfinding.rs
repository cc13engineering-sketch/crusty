/// A* pathfinding on a grid.
///
/// Works with TileMap or any grid where cells can be solid/walkable.
/// Returns a path as a sequence of (col, row) tile coordinates.

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

/// A position on the grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// A node in the A* open set.
#[derive(Clone, Debug)]
struct AStarNode {
    pos: GridPos,
    g_cost: f64,
    f_cost: f64,
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_cost == other.f_cost
    }
}

impl Eq for AStarNode {}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap (BinaryHeap is max by default)
        other.f_cost.partial_cmp(&self.f_cost)
            .unwrap_or(Ordering::Equal)
    }
}

/// Configuration for pathfinding.
#[derive(Clone, Debug)]
pub struct PathConfig {
    /// Allow diagonal movement.
    pub allow_diagonal: bool,
    /// Cost of diagonal movement (typically sqrt(2) ≈ 1.414).
    pub diagonal_cost: f64,
    /// Cost of cardinal movement.
    pub cardinal_cost: f64,
    /// Maximum number of nodes to explore before giving up.
    pub max_iterations: usize,
}

impl Default for PathConfig {
    fn default() -> Self {
        Self {
            allow_diagonal: true,
            diagonal_cost: 1.414,
            cardinal_cost: 1.0,
            max_iterations: 10_000,
        }
    }
}

/// Heuristic function: octile distance (supports diagonal).
fn heuristic(a: GridPos, b: GridPos, config: &PathConfig) -> f64 {
    let dx = (a.x - b.x).unsigned_abs() as f64;
    let dy = (a.y - b.y).unsigned_abs() as f64;
    if config.allow_diagonal {
        let min = dx.min(dy);
        let max = dx.max(dy);
        config.diagonal_cost * min + config.cardinal_cost * (max - min)
    } else {
        config.cardinal_cost * (dx + dy)
    }
}

/// Neighbors for a grid position.
fn neighbors(pos: GridPos, allow_diagonal: bool) -> Vec<(GridPos, bool)> {
    let mut result = vec![
        (GridPos::new(pos.x + 1, pos.y), false),
        (GridPos::new(pos.x - 1, pos.y), false),
        (GridPos::new(pos.x, pos.y + 1), false),
        (GridPos::new(pos.x, pos.y - 1), false),
    ];
    if allow_diagonal {
        result.push((GridPos::new(pos.x + 1, pos.y + 1), true));
        result.push((GridPos::new(pos.x - 1, pos.y + 1), true));
        result.push((GridPos::new(pos.x + 1, pos.y - 1), true));
        result.push((GridPos::new(pos.x - 1, pos.y - 1), true));
    }
    result
}

/// Find the shortest path from `start` to `goal` on a grid.
///
/// `is_walkable` returns true if the given grid position is passable.
/// Returns `None` if no path exists or the search exceeds max iterations.
pub fn find_path<F>(
    start: GridPos,
    goal: GridPos,
    config: &PathConfig,
    is_walkable: F,
) -> Option<Vec<GridPos>>
where
    F: Fn(GridPos) -> bool,
{
    if start == goal {
        return Some(vec![start]);
    }

    if !is_walkable(start) || !is_walkable(goal) {
        return None;
    }

    let mut open = BinaryHeap::new();
    let mut came_from: HashMap<GridPos, GridPos> = HashMap::new();
    let mut g_scores: HashMap<GridPos, f64> = HashMap::new();
    let mut iterations = 0;

    g_scores.insert(start, 0.0);
    open.push(AStarNode {
        pos: start,
        g_cost: 0.0,
        f_cost: heuristic(start, goal, config),
    });

    while let Some(current) = open.pop() {
        iterations += 1;
        if iterations > config.max_iterations {
            return None; // Search exhausted
        }

        if current.pos == goal {
            // Reconstruct path
            let mut path = Vec::new();
            let mut p = goal;
            path.push(p);
            while let Some(&prev) = came_from.get(&p) {
                path.push(prev);
                p = prev;
            }
            path.reverse();
            return Some(path);
        }

        let current_g = g_scores.get(&current.pos).copied().unwrap_or(f64::INFINITY);

        // Skip if we've already found a better path to this node
        if current.g_cost > current_g {
            continue;
        }

        for (neighbor, is_diagonal) in neighbors(current.pos, config.allow_diagonal) {
            if !is_walkable(neighbor) {
                continue;
            }

            // For diagonal moves, check that both cardinal neighbors are walkable
            // (prevents cutting corners through walls)
            if is_diagonal {
                let dx = neighbor.x - current.pos.x;
                let dy = neighbor.y - current.pos.y;
                if !is_walkable(GridPos::new(current.pos.x + dx, current.pos.y))
                    || !is_walkable(GridPos::new(current.pos.x, current.pos.y + dy))
                {
                    continue;
                }
            }

            let move_cost = if is_diagonal {
                config.diagonal_cost
            } else {
                config.cardinal_cost
            };

            let tentative_g = current_g + move_cost;
            let existing_g = g_scores.get(&neighbor).copied().unwrap_or(f64::INFINITY);

            if tentative_g < existing_g {
                came_from.insert(neighbor, current.pos);
                g_scores.insert(neighbor, tentative_g);
                open.push(AStarNode {
                    pos: neighbor,
                    g_cost: tentative_g,
                    f_cost: tentative_g + heuristic(neighbor, goal, config),
                });
            }
        }
    }

    None // No path found
}

/// Convenience: find path on a TileMap (uses tilemap solid checks).
pub fn find_path_on_tilemap(
    tilemap: &crate::tilemap::TileMap,
    start_x: usize,
    start_y: usize,
    goal_x: usize,
    goal_y: usize,
    config: &PathConfig,
) -> Option<Vec<GridPos>> {
    let start = GridPos::new(start_x as i32, start_y as i32);
    let goal = GridPos::new(goal_x as i32, goal_y as i32);
    let w = tilemap.width as i32;
    let h = tilemap.height as i32;

    find_path(start, goal, config, |pos| {
        pos.x >= 0 && pos.y >= 0 && pos.x < w && pos.y < h
            && !tilemap.is_solid(pos.x as usize, pos.y as usize)
    })
}

/// Convert a grid path to world coordinates using a tilemap's tile_to_world.
pub fn path_to_world(
    tilemap: &crate::tilemap::TileMap,
    path: &[GridPos],
) -> Vec<(f64, f64)> {
    path.iter()
        .map(|p| tilemap.tile_to_world(p.x as usize, p.y as usize))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open_grid(pos: GridPos) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < 10 && pos.y < 10
    }

    #[test]
    fn same_start_and_goal() {
        let config = PathConfig::default();
        let path = find_path(GridPos::new(5, 5), GridPos::new(5, 5), &config, open_grid);
        assert_eq!(path, Some(vec![GridPos::new(5, 5)]));
    }

    #[test]
    fn straight_line_path() {
        let config = PathConfig { allow_diagonal: false, ..Default::default() };
        let path = find_path(GridPos::new(0, 0), GridPos::new(3, 0), &config, open_grid).unwrap();
        assert_eq!(path.len(), 4);
        assert_eq!(path[0], GridPos::new(0, 0));
        assert_eq!(path[3], GridPos::new(3, 0));
    }

    #[test]
    fn diagonal_shortcut() {
        let config = PathConfig::default();
        let path = find_path(GridPos::new(0, 0), GridPos::new(3, 3), &config, open_grid).unwrap();
        // Diagonal should be shorter than cardinal-only
        assert_eq!(path.len(), 4); // 3 diagonal steps + start
    }

    #[test]
    fn cardinal_only_longer() {
        let config_diag = PathConfig::default();
        let config_card = PathConfig { allow_diagonal: false, ..Default::default() };

        let path_diag = find_path(GridPos::new(0, 0), GridPos::new(3, 3), &config_diag, open_grid).unwrap();
        let path_card = find_path(GridPos::new(0, 0), GridPos::new(3, 3), &config_card, open_grid).unwrap();

        assert!(path_card.len() > path_diag.len());
    }

    #[test]
    fn path_around_wall() {
        // Wall at x=2, y=0..3
        let is_walkable = |pos: GridPos| -> bool {
            pos.x >= 0 && pos.y >= 0 && pos.x < 10 && pos.y < 10
                && !(pos.x == 2 && pos.y < 3)
        };
        let config = PathConfig { allow_diagonal: false, ..Default::default() };
        let path = find_path(GridPos::new(0, 0), GridPos::new(4, 0), &config, is_walkable);
        assert!(path.is_some());
        let path = path.unwrap();
        // Path must go around the wall
        assert!(path.len() > 5);
        assert_eq!(*path.first().unwrap(), GridPos::new(0, 0));
        assert_eq!(*path.last().unwrap(), GridPos::new(4, 0));
        // Verify no position crosses the wall
        for p in &path {
            assert!(is_walkable(*p));
        }
    }

    #[test]
    fn no_path_when_blocked() {
        // Completely surround the goal
        let is_walkable = |pos: GridPos| -> bool {
            pos.x >= 0 && pos.y >= 0 && pos.x < 5 && pos.y < 5
                && !(pos.x == 3 && pos.y >= 0 && pos.y <= 4) // wall column
        };
        let config = PathConfig::default();
        let path = find_path(GridPos::new(0, 0), GridPos::new(4, 2), &config, is_walkable);
        assert!(path.is_none());
    }

    #[test]
    fn unwalkable_start_returns_none() {
        let config = PathConfig::default();
        let path = find_path(GridPos::new(0, 0), GridPos::new(5, 5), &config, |_| false);
        assert!(path.is_none());
    }

    #[test]
    fn max_iterations_limits_search() {
        let config = PathConfig {
            max_iterations: 5,
            allow_diagonal: false,
            ..Default::default()
        };
        // Long path that requires many iterations
        let path = find_path(GridPos::new(0, 0), GridPos::new(9, 9), &config, open_grid);
        assert!(path.is_none()); // should exhaust iterations
    }

    #[test]
    fn path_on_tilemap() {
        let mut tm = crate::tilemap::TileMap::new(10, 10, 32.0);
        // Set a wall
        let wall = crate::tilemap::Tile::solid(crate::rendering::color::Color::WHITE);
        tm.set(2, 0, wall.clone());
        tm.set(2, 1, wall.clone());
        tm.set(2, 2, wall);

        let config = PathConfig { allow_diagonal: false, ..Default::default() };
        let path = find_path_on_tilemap(&tm, 0, 0, 4, 0, &config);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(*path.first().unwrap(), GridPos::new(0, 0));
        assert_eq!(*path.last().unwrap(), GridPos::new(4, 0));
    }

    #[test]
    fn path_to_world_converts() {
        let tm = crate::tilemap::TileMap::new(10, 10, 32.0);
        let path = vec![GridPos::new(0, 0), GridPos::new(1, 0), GridPos::new(2, 0)];
        let world_path = path_to_world(&tm, &path);
        assert_eq!(world_path.len(), 3);
        // Each world pos should be at tile center
        assert!((world_path[1].0 - 48.0).abs() < 1e-6); // 1*32 + 16
    }

    #[test]
    fn diagonal_avoids_corner_cutting() {
        // Wall at (1,0) and (0,1) — diagonal (1,1) should not be reachable
        let is_walkable = |pos: GridPos| -> bool {
            pos.x >= 0 && pos.y >= 0 && pos.x < 5 && pos.y < 5
                && !(pos.x == 1 && pos.y == 0)
                && !(pos.x == 0 && pos.y == 1)
        };
        let config = PathConfig::default();
        let path = find_path(GridPos::new(0, 0), GridPos::new(1, 1), &config, is_walkable);
        // Should still find a path but not via diagonal from (0,0)
        if let Some(path) = path {
            // First step from (0,0) should NOT be (1,1) directly
            if path.len() > 1 {
                assert_ne!(path[1], GridPos::new(1, 1));
            }
        }
    }

    #[test]
    fn large_open_grid() {
        let config = PathConfig::default();
        let is_walkable = |pos: GridPos| -> bool {
            pos.x >= 0 && pos.y >= 0 && pos.x < 50 && pos.y < 50
        };
        let path = find_path(GridPos::new(0, 0), GridPos::new(49, 49), &config, is_walkable);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(*path.first().unwrap(), GridPos::new(0, 0));
        assert_eq!(*path.last().unwrap(), GridPos::new(49, 49));
    }
}
