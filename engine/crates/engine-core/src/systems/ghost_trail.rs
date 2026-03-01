/// SYSTEM: ghost_trail
/// READS: Transform, GhostTrail, TimeScale
/// WRITES: GhostTrail (tick/capture snapshots)
/// ORDER: runs after physics, before rendering

use crate::ecs::World;

pub fn run(world: &mut World, dt: f64) {
    // Collect entities with ghost trails
    let entities: Vec<_> = world.ghost_trails.iter()
        .map(|(e, _)| e)
        .collect();

    for entity in entities {
        let effective_dt = if let Some(ts) = world.time_scales.get(entity) {
            ts.apply(dt)
        } else {
            dt
        };

        // Get current position
        let pos = world.transforms.get(entity).map(|t| (t.x, t.y));

        if let Some((x, y)) = pos {
            if let Some(trail) = world.ghost_trails.get_mut(entity) {
                trail.tick(effective_dt, x, y);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::World;
    use crate::components::*;
    use crate::components::ghost_trail::GhostTrail;
    use crate::rendering::color::Color;

    #[test]
    fn captures_snapshots() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 10.0, y: 20.0, rotation: 0.0, scale: 1.0 });
        world.ghost_trails.insert(e, GhostTrail::new(8, 0.1, 1.0, Color::WHITE));

        run(&mut world, 0.1);
        let trail = world.ghost_trails.get(e).expect("trail should exist");
        assert_eq!(trail.snapshots.len(), 1);
    }

    #[test]
    fn snapshots_use_current_position() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 42.0, y: 99.0, rotation: 0.0, scale: 1.0 });
        world.ghost_trails.insert(e, GhostTrail::new(8, 0.05, 1.0, Color::WHITE));

        run(&mut world, 0.05);
        let trail = world.ghost_trails.get(e).expect("trail");
        assert_eq!(trail.snapshots[0].x, 42.0);
        assert_eq!(trail.snapshots[0].y, 99.0);
    }

    #[test]
    fn no_transform_no_capture() {
        let mut world = World::new();
        let e = world.spawn();
        world.ghost_trails.insert(e, GhostTrail::new(8, 0.05, 1.0, Color::WHITE));

        run(&mut world, 0.1);
        let trail = world.ghost_trails.get(e).expect("trail");
        assert!(trail.snapshots.is_empty());
    }

    #[test]
    fn time_scale_affects_capture() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.ghost_trails.insert(e, GhostTrail::new(8, 0.1, 1.0, Color::WHITE));
        world.time_scales.insert(e, TimeScale::frozen());

        run(&mut world, 0.2);
        let trail = world.ghost_trails.get(e).expect("trail");
        assert!(trail.snapshots.is_empty()); // frozen, no dt passes
    }

    #[test]
    fn expired_snapshots_removed() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.ghost_trails.insert(e, GhostTrail::new(8, 0.05, 0.2, Color::WHITE));

        run(&mut world, 0.05); // capture
        run(&mut world, 0.25); // age past fade_duration
        let trail = world.ghost_trails.get(e).expect("trail");
        // Older snapshots should be gone
        for snap in &trail.snapshots {
            assert!(snap.age < trail.fade_duration);
        }
    }
}
