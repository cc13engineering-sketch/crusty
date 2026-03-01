/// SYSTEM: waypoint
/// READS: WaypointPath, Transform, TimeScale, Active
/// WRITES: Transform (move toward waypoints), WaypointPath (advance index)
/// ORDER: runs before physics

use crate::ecs::World;
use crate::components::waypoint_path::WaypointMode;

pub fn run(world: &mut World, dt: f64) {
    let entities: Vec<_> = world.waypoint_paths.iter()
        .map(|(e, _)| e)
        .collect();

    for entity in entities {
        // Skip inactive entities
        if let Some(active) = world.actives.get(entity) {
            if !active.enabled { continue; }
        }

        let effective_dt = if let Some(ts) = world.time_scales.get(entity) {
            ts.apply(dt)
        } else {
            dt
        };

        if effective_dt <= 0.0 { continue; }

        // Get current position
        let pos = if let Some(t) = world.transforms.get(entity) {
            (t.x, t.y)
        } else {
            continue;
        };

        if let Some(wp) = world.waypoint_paths.get_mut(entity) {
            if wp.waypoints.is_empty() { continue; }

            // Handle pause at waypoint
            if wp.pause_timer > 0.0 {
                wp.pause_timer -= effective_dt;
                continue;
            }

            // Get target waypoint
            let idx = wp.current_index.min(wp.waypoints.len() - 1);
            let target_x = wp.waypoints[idx].x;
            let target_y = wp.waypoints[idx].y;

            // Calculate movement
            let dx = target_x - pos.0;
            let dy = target_y - pos.1;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 1.0 {
                // Arrived at waypoint — snap to it
                if let Some(t) = world.transforms.get_mut(entity) {
                    t.x = target_x;
                    t.y = target_y;
                }

                // Set pause timer
                wp.pause_timer = wp.pause_at_waypoint;

                // Advance index based on mode
                match wp.mode {
                    WaypointMode::Once => {
                        if wp.current_index < wp.waypoints.len() - 1 {
                            wp.current_index += 1;
                        }
                        // else stay at last waypoint
                    }
                    WaypointMode::Loop => {
                        wp.current_index = (wp.current_index + 1) % wp.waypoints.len();
                    }
                    WaypointMode::PingPong => {
                        if wp.forward {
                            if wp.current_index >= wp.waypoints.len() - 1 {
                                wp.forward = false;
                                if wp.current_index > 0 {
                                    wp.current_index -= 1;
                                }
                            } else {
                                wp.current_index += 1;
                            }
                        } else {
                            if wp.current_index == 0 {
                                wp.forward = true;
                                if wp.waypoints.len() > 1 {
                                    wp.current_index = 1;
                                }
                            } else {
                                wp.current_index -= 1;
                            }
                        }
                    }
                }
            } else {
                // Move toward waypoint
                let move_dist = wp.speed * effective_dt;
                let ratio = (move_dist / dist).min(1.0);
                if let Some(t) = world.transforms.get_mut(entity) {
                    t.x += dx * ratio;
                    t.y += dy * ratio;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::World;
    use crate::components::*;
    use crate::components::waypoint_path::{WaypointPath, Waypoint, WaypointMode};

    fn setup_entity_with_path(world: &mut World, waypoints: Vec<Waypoint>, speed: f64, mode: WaypointMode) -> crate::ecs::Entity {
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.waypoint_paths.insert(e, WaypointPath::new(waypoints, speed, mode));
        e
    }

    #[test]
    fn moves_toward_first_waypoint() {
        let mut world = World::new();
        let e = setup_entity_with_path(&mut world,
            vec![Waypoint::new(100.0, 0.0)], 50.0, WaypointMode::Once);

        run(&mut world, 1.0);
        let t = world.transforms.get(e).unwrap();
        assert!(t.x > 0.0);
        assert!(t.x <= 50.0);
    }

    #[test]
    fn reaches_waypoint_and_stops_once_mode() {
        let mut world = World::new();
        let e = setup_entity_with_path(&mut world,
            vec![Waypoint::new(10.0, 0.0)], 1000.0, WaypointMode::Once);

        run(&mut world, 1.0);
        let t = world.transforms.get(e).unwrap();
        assert!((t.x - 10.0).abs() < 1.0);
    }

    #[test]
    fn loop_mode_wraps_around() {
        let mut world = World::new();
        let e = setup_entity_with_path(&mut world,
            vec![Waypoint::new(5.0, 0.0), Waypoint::new(10.0, 0.0)], 1000.0, WaypointMode::Loop);

        run(&mut world, 0.1); // reach first
        let wp = world.waypoint_paths.get(e).unwrap();
        // After reaching first waypoint, should advance
        assert!(wp.current_index <= 2);
    }

    #[test]
    fn ping_pong_reverses() {
        let mut world = World::new();
        let e = setup_entity_with_path(&mut world,
            vec![Waypoint::new(5.0, 0.0), Waypoint::new(10.0, 0.0)], 1000.0, WaypointMode::PingPong);

        // Tick enough times to move past both waypoints and trigger reversal
        // Tick 1: move from (0,0) toward (5,0) — arrives
        // Tick 2: detect arrival, advance to index 1
        // Tick 3: move from (5,0) toward (10,0) — arrives
        // Tick 4: detect arrival at last waypoint, reverse direction
        for _ in 0..4 {
            run(&mut world, 0.1);
        }
        let wp = world.waypoint_paths.get(e).unwrap();
        // Should have reversed direction after reaching last waypoint
        assert!(!wp.forward, "Expected forward=false after reaching last waypoint in PingPong mode");
    }

    #[test]
    fn pause_at_waypoint() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        let mut path = WaypointPath::new(
            vec![Waypoint::new(5.0, 0.0), Waypoint::new(50.0, 0.0)],
            1000.0, WaypointMode::Once,
        );
        path.pause_at_waypoint = 1.0;
        world.waypoint_paths.insert(e, path);

        run(&mut world, 0.1); // reach first waypoint, start pause
        let t1 = world.transforms.get(e).unwrap().x;
        run(&mut world, 0.5); // still pausing
        let t2 = world.transforms.get(e).unwrap().x;
        assert!((t1 - t2).abs() < 1.0); // should not have moved
    }

    #[test]
    fn inactive_entity_skipped() {
        let mut world = World::new();
        let e = setup_entity_with_path(&mut world,
            vec![Waypoint::new(100.0, 0.0)], 50.0, WaypointMode::Once);
        world.actives.insert(e, Active::disabled());

        run(&mut world, 1.0);
        let t = world.transforms.get(e).unwrap();
        assert_eq!(t.x, 0.0); // didn't move
    }

    #[test]
    fn empty_waypoints_does_nothing() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform::default());
        world.waypoint_paths.insert(e, WaypointPath::new(vec![], 100.0, WaypointMode::Once));

        run(&mut world, 1.0); // should not panic
    }
}
