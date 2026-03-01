use crate::ecs::{Entity, World};
use crate::components::collider::ColliderShape;

/// Result of a ray hitting an entity's collider.
#[derive(Clone, Debug)]
pub struct RayHit {
    pub entity: Entity,
    pub point: (f64, f64),
    pub normal: (f64, f64),
    pub distance: f64,
}

/// A ray defined by origin and direction.
#[derive(Clone, Debug)]
pub struct Ray {
    pub origin_x: f64,
    pub origin_y: f64,
    pub dir_x: f64,
    pub dir_y: f64,
}

impl Ray {
    /// Create a ray with a normalized direction.
    pub fn new(ox: f64, oy: f64, dx: f64, dy: f64) -> Self {
        let len = (dx * dx + dy * dy).sqrt();
        if len < 1e-12 {
            Self { origin_x: ox, origin_y: oy, dir_x: 1.0, dir_y: 0.0 }
        } else {
            Self { origin_x: ox, origin_y: oy, dir_x: dx / len, dir_y: dy / len }
        }
    }

    /// Create a ray from two points.
    pub fn from_points(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self::new(x1, y1, x2 - x1, y2 - y1)
    }
}

/// Ray-circle intersection. Returns (distance, hit_point, normal) or None.
fn ray_vs_circle(ray: &Ray, cx: f64, cy: f64, radius: f64) -> Option<(f64, (f64, f64), (f64, f64))> {
    let dx = ray.origin_x - cx;
    let dy = ray.origin_y - cy;
    // Quadratic: t^2 + 2*b*t + c = 0 (since dir is normalized, a=1)
    let b = dx * ray.dir_x + dy * ray.dir_y;
    let c = dx * dx + dy * dy - radius * radius;
    let discriminant = b * b - c;
    if discriminant < 0.0 {
        return None;
    }
    let sqrt_d = discriminant.sqrt();
    let t = -b - sqrt_d;
    if t < 0.0 {
        // Try the other root (ray starts inside circle)
        let t2 = -b + sqrt_d;
        if t2 < 0.0 {
            return None;
        }
        let hx = ray.origin_x + ray.dir_x * t2;
        let hy = ray.origin_y + ray.dir_y * t2;
        let nx = (hx - cx) / radius;
        let ny = (hy - cy) / radius;
        return Some((t2, (hx, hy), (nx, ny)));
    }
    let hx = ray.origin_x + ray.dir_x * t;
    let hy = ray.origin_y + ray.dir_y * t;
    let nx = (hx - cx) / radius;
    let ny = (hy - cy) / radius;
    Some((t, (hx, hy), (nx, ny)))
}

/// Ray-AABB intersection using slab method. Returns (distance, hit_point, normal) or None.
fn ray_vs_aabb(ray: &Ray, cx: f64, cy: f64, half_w: f64, half_h: f64) -> Option<(f64, (f64, f64), (f64, f64))> {
    let min_x = cx - half_w;
    let max_x = cx + half_w;
    let min_y = cy - half_h;
    let max_y = cy + half_h;

    let (mut tmin, mut tmax, mut normal_min) = if ray.dir_x.abs() < 1e-12 {
        if ray.origin_x < min_x || ray.origin_x > max_x {
            return None;
        }
        (f64::NEG_INFINITY, f64::INFINITY, (0.0, 0.0))
    } else {
        let inv_dx = 1.0 / ray.dir_x;
        let mut t1 = (min_x - ray.origin_x) * inv_dx;
        let mut t2 = (max_x - ray.origin_x) * inv_dx;
        let mut n = (-1.0, 0.0);
        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
            n = (1.0, 0.0);
        }
        (t1, t2, n)
    };

    let (ty_min, ty_max, ny_min) = if ray.dir_y.abs() < 1e-12 {
        if ray.origin_y < min_y || ray.origin_y > max_y {
            return None;
        }
        (f64::NEG_INFINITY, f64::INFINITY, (0.0, 0.0))
    } else {
        let inv_dy = 1.0 / ray.dir_y;
        let mut t1 = (min_y - ray.origin_y) * inv_dy;
        let mut t2 = (max_y - ray.origin_y) * inv_dy;
        let mut n = (0.0, -1.0);
        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
            n = (0.0, 1.0);
        }
        (t1, t2, n)
    };

    if ty_min > tmin {
        tmin = ty_min;
        normal_min = ny_min;
    }
    if ty_max < tmax {
        tmax = ty_max;
    }

    if tmin > tmax || tmax < 0.0 {
        return None;
    }

    let t = if tmin >= 0.0 { tmin } else { tmax };
    if t < 0.0 {
        return None;
    }

    let hx = ray.origin_x + ray.dir_x * t;
    let hy = ray.origin_y + ray.dir_y * t;
    Some((t, (hx, hy), normal_min))
}

/// Cast a ray against all colliders in the world, return closest hit.
pub fn raycast(world: &World, ray: &Ray, max_distance: f64) -> Option<RayHit> {
    let hits = raycast_all(world, ray, max_distance);
    hits.first().cloned()
}

/// Cast a ray, return ALL hits sorted by distance.
pub fn raycast_all(world: &World, ray: &Ray, max_distance: f64) -> Vec<RayHit> {
    let mut hits = Vec::new();

    for (entity, collider) in world.colliders.iter() {
        let transform = match world.transforms.get(entity) {
            Some(t) => t,
            None => continue,
        };

        let result = match &collider.shape {
            ColliderShape::Circle { radius } => {
                ray_vs_circle(ray, transform.x, transform.y, *radius)
            }
            ColliderShape::Rect { half_width, half_height } => {
                ray_vs_aabb(ray, transform.x, transform.y, *half_width, *half_height)
            }
        };

        if let Some((dist, point, normal)) = result {
            if dist <= max_distance && dist >= 0.0 {
                hits.push(RayHit { entity, point, normal, distance: dist });
            }
        }
    }

    hits.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal));
    hits
}

/// Check line of sight between two points. Returns true if unobstructed.
pub fn line_of_sight(world: &World, x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let distance = (dx * dx + dy * dy).sqrt();
    if distance < 1e-12 {
        return true;
    }
    let ray = Ray::new(x1, y1, dx, dy);
    raycast(world, &ray, distance).is_none()
}

/// Cast ray against a TileMap using DDA grid traversal.
/// Returns the world-space point where the ray hits a solid tile.
pub fn raycast_tilemap(tilemap: &crate::tilemap::TileMap, ray: &Ray, max_distance: f64) -> Option<(f64, f64)> {
    if ray.dir_x.abs() < 1e-12 && ray.dir_y.abs() < 1e-12 {
        return None;
    }

    let ts = tilemap.tile_size;
    let inv_ts = 1.0 / ts;

    // Starting tile
    let mut tile_x = ((ray.origin_x - tilemap.origin_x) * inv_ts).floor() as i64;
    let mut tile_y = ((ray.origin_y - tilemap.origin_y) * inv_ts).floor() as i64;

    // Step direction
    let step_x: i64 = if ray.dir_x >= 0.0 { 1 } else { -1 };
    let step_y: i64 = if ray.dir_y >= 0.0 { 1 } else { -1 };

    // Distance to next cell boundary
    let next_x = if step_x > 0 {
        tilemap.origin_x + (tile_x + 1) as f64 * ts
    } else {
        tilemap.origin_x + tile_x as f64 * ts
    };
    let next_y = if step_y > 0 {
        tilemap.origin_y + (tile_y + 1) as f64 * ts
    } else {
        tilemap.origin_y + tile_y as f64 * ts
    };

    let mut t_max_x = if ray.dir_x.abs() > 1e-12 {
        (next_x - ray.origin_x) / ray.dir_x
    } else {
        f64::INFINITY
    };
    let mut t_max_y = if ray.dir_y.abs() > 1e-12 {
        (next_y - ray.origin_y) / ray.dir_y
    } else {
        f64::INFINITY
    };

    let t_delta_x = if ray.dir_x.abs() > 1e-12 { (ts / ray.dir_x).abs() } else { f64::INFINITY };
    let t_delta_y = if ray.dir_y.abs() > 1e-12 { (ts / ray.dir_y).abs() } else { f64::INFINITY };

    let max_steps = ((max_distance * inv_ts).ceil() as usize + 2) * 2;

    // `t_entry` tracks the parametric distance at which the ray *entered* the
    // current cell. For the origin cell this is 0 (or could even be negative
    // when the origin is inside the map, which we clamp to 0 below).
    let mut t_entry: f64 = 0.0;

    for _ in 0..max_steps {
        // Check if current tile is valid and solid
        if tile_x >= 0 && tile_y >= 0
            && (tile_x as usize) < tilemap.width
            && (tile_y as usize) < tilemap.height
        {
            if tilemap.is_solid(tile_x as usize, tile_y as usize) {
                // Use the entry t so the hit point is on the near face of the
                // tile, not the far face.
                let t = t_entry.max(0.0);
                if t <= max_distance {
                    let hit_x = ray.origin_x + ray.dir_x * t;
                    let hit_y = ray.origin_y + ray.dir_y * t;
                    return Some((hit_x, hit_y));
                }
            }
        }

        // Step to the next cell, recording the current t_max as the next entry t.
        if t_max_x < t_max_y {
            if t_max_x > max_distance { break; }
            t_entry = t_max_x;
            tile_x += step_x;
            t_max_x += t_delta_x;
        } else {
            if t_max_y > max_distance { break; }
            t_entry = t_max_y;
            tile_y += step_y;
            t_max_y += t_delta_y;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::World;
    use crate::components::transform::Transform;
    use crate::components::collider::{Collider, ColliderShape};
    use crate::tilemap::TileMap;
    use crate::rendering::color::Color;

    #[test]
    fn ray_creation_normalizes() {
        let r = Ray::new(0.0, 0.0, 3.0, 4.0);
        let len = (r.dir_x * r.dir_x + r.dir_y * r.dir_y).sqrt();
        assert!((len - 1.0).abs() < 1e-10);
        assert!((r.dir_x - 0.6).abs() < 1e-10);
        assert!((r.dir_y - 0.8).abs() < 1e-10);
    }

    #[test]
    fn ray_from_points() {
        let r = Ray::from_points(0.0, 0.0, 10.0, 0.0);
        assert!((r.dir_x - 1.0).abs() < 1e-10);
        assert!(r.dir_y.abs() < 1e-10);
    }

    #[test]
    fn raycast_hits_circle() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 10.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.colliders.insert(e, Collider { shape: ColliderShape::Circle { radius: 2.0 }, is_trigger: false });

        let ray = Ray::new(0.0, 0.0, 1.0, 0.0);
        let hit = raycast(&world, &ray, 100.0);
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert_eq!(hit.entity, e);
        assert!((hit.distance - 8.0).abs() < 1e-10); // 10 - 2 = 8
    }

    #[test]
    fn raycast_misses_when_off_target() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 10.0, y: 100.0, rotation: 0.0, scale: 1.0 });
        world.colliders.insert(e, Collider { shape: ColliderShape::Circle { radius: 2.0 }, is_trigger: false });

        let ray = Ray::new(0.0, 0.0, 1.0, 0.0);
        let hit = raycast(&world, &ray, 100.0);
        assert!(hit.is_none());
    }

    #[test]
    fn raycast_all_returns_sorted() {
        let mut world = World::new();
        let e1 = world.spawn();
        let e2 = world.spawn();
        world.transforms.insert(e1, Transform { x: 20.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.transforms.insert(e2, Transform { x: 10.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.colliders.insert(e1, Collider { shape: ColliderShape::Circle { radius: 2.0 }, is_trigger: false });
        world.colliders.insert(e2, Collider { shape: ColliderShape::Circle { radius: 2.0 }, is_trigger: false });

        let ray = Ray::new(0.0, 0.0, 1.0, 0.0);
        let hits = raycast_all(&world, &ray, 100.0);
        assert_eq!(hits.len(), 2);
        assert!(hits[0].distance < hits[1].distance);
        assert_eq!(hits[0].entity, e2); // closer
    }

    #[test]
    fn raycast_returns_closest() {
        let mut world = World::new();
        let e1 = world.spawn();
        let e2 = world.spawn();
        world.transforms.insert(e1, Transform { x: 20.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.transforms.insert(e2, Transform { x: 10.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.colliders.insert(e1, Collider { shape: ColliderShape::Circle { radius: 2.0 }, is_trigger: false });
        world.colliders.insert(e2, Collider { shape: ColliderShape::Circle { radius: 2.0 }, is_trigger: false });

        let hit = raycast(&world, &Ray::new(0.0, 0.0, 1.0, 0.0), 100.0);
        assert_eq!(hit.unwrap().entity, e2);
    }

    #[test]
    fn line_of_sight_clear() {
        let world = World::new();
        assert!(line_of_sight(&world, 0.0, 0.0, 100.0, 100.0));
    }

    #[test]
    fn line_of_sight_blocked() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 50.0, y: 50.0, rotation: 0.0, scale: 1.0 });
        world.colliders.insert(e, Collider { shape: ColliderShape::Circle { radius: 10.0 }, is_trigger: false });

        assert!(!line_of_sight(&world, 0.0, 50.0, 100.0, 50.0));
    }

    #[test]
    fn ray_vs_aabb_hit() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 10.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.colliders.insert(e, Collider {
            shape: ColliderShape::Rect { half_width: 3.0, half_height: 3.0 },
            is_trigger: false,
        });

        let ray = Ray::new(0.0, 0.0, 1.0, 0.0);
        let hit = raycast(&world, &ray, 100.0);
        assert!(hit.is_some());
        assert!((hit.unwrap().distance - 7.0).abs() < 1e-10);
    }

    #[test]
    fn raycast_tilemap_hits_solid() {
        let mut tm = TileMap::new(10, 10, 32.0);
        tm.set(3, 0, crate::tilemap::Tile::solid(Color::WHITE));

        let ray = Ray::new(0.0, 16.0, 1.0, 0.0);
        let hit = raycast_tilemap(&tm, &ray, 500.0);
        assert!(hit.is_some());
    }

    #[test]
    fn raycast_tilemap_misses_empty() {
        let tm = TileMap::new(10, 10, 32.0);
        let ray = Ray::new(0.0, 16.0, 1.0, 0.0);
        let hit = raycast_tilemap(&tm, &ray, 500.0);
        assert!(hit.is_none());
    }

    #[test]
    fn max_distance_limits_results() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 100.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.colliders.insert(e, Collider { shape: ColliderShape::Circle { radius: 2.0 }, is_trigger: false });

        let ray = Ray::new(0.0, 0.0, 1.0, 0.0);
        let hit = raycast(&world, &ray, 50.0);
        assert!(hit.is_none()); // 100 - 2 = 98 > 50
    }
}
