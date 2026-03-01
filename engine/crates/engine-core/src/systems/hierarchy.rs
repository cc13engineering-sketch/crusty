/// SYSTEM: hierarchy
/// READS: Parent, Transform, Children
/// WRITES: WorldTransform (computed world-space positions)
/// ORDER: runs before rendering, after behavior/movement

use crate::ecs::World;
use crate::components::hierarchy::WorldTransform;

/// Propagate transforms from parents to children.
/// Entities without a Parent get WorldTransform = local Transform.
/// Children's local transforms are offset/rotated/scaled by parent's world transform.
pub fn run(world: &mut World) {
    // Collect all entities that have transforms
    let entities: Vec<_> = world.transforms.iter()
        .map(|(e, _)| e)
        .collect();

    // Phase 1: Set WorldTransform for root entities (no parent)
    for &entity in &entities {
        if world.parents.get(entity).is_none() {
            if let Some(t) = world.transforms.get(entity) {
                world.world_transforms.insert(entity, WorldTransform {
                    x: t.x,
                    y: t.y,
                    rotation: t.rotation,
                    scale: t.scale,
                });
            }
        }
    }

    // Phase 2: Propagate to children (iterative — handles multi-level)
    // We do multiple passes until no more updates are needed.
    // Max depth is bounded by entity count.
    let max_depth = entities.len();
    for _ in 0..max_depth {
        let mut any_updated = false;

        for &entity in &entities {
            if let Some(parent_comp) = world.parents.get(entity) {
                let parent_entity = parent_comp.entity;

                // Only propagate if parent's world transform is already computed
                if let Some(parent_wt) = world.world_transforms.get(parent_entity) {
                    let pwx = parent_wt.x;
                    let pwy = parent_wt.y;
                    let prot = parent_wt.rotation;
                    let pscale = parent_wt.scale;

                    if let Some(local) = world.transforms.get(entity) {
                        // Rotate child's local offset by parent's rotation
                        let cos_r = prot.cos();
                        let sin_r = prot.sin();
                        let scaled_x = local.x * pscale;
                        let scaled_y = local.y * pscale;
                        let world_x = pwx + scaled_x * cos_r - scaled_y * sin_r;
                        let world_y = pwy + scaled_x * sin_r + scaled_y * cos_r;

                        let new_wt = WorldTransform {
                            x: world_x,
                            y: world_y,
                            rotation: prot + local.rotation,
                            scale: pscale * local.scale,
                        };

                        // Check if this is a new or changed value
                        let needs_update = match world.world_transforms.get(entity) {
                            None => true,
                            Some(existing) => {
                                (existing.x - new_wt.x).abs() > 1e-12
                                || (existing.y - new_wt.y).abs() > 1e-12
                                || (existing.rotation - new_wt.rotation).abs() > 1e-12
                                || (existing.scale - new_wt.scale).abs() > 1e-12
                            }
                        };

                        if needs_update {
                            world.world_transforms.insert(entity, new_wt);
                            any_updated = true;
                        }
                    }
                }
            }
        }

        if !any_updated {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::World;
    use crate::components::transform::Transform;
    use crate::components::hierarchy::{Parent, Children};

    fn setup_parent_child(world: &mut World) -> (crate::ecs::Entity, crate::ecs::Entity) {
        let parent = world.spawn();
        let child = world.spawn();
        world.transforms.insert(parent, Transform { x: 100.0, y: 200.0, rotation: 0.0, scale: 1.0 });
        world.transforms.insert(child, Transform { x: 10.0, y: 20.0, rotation: 0.0, scale: 1.0 });
        world.parents.insert(child, Parent::new(parent));
        world.children.insert(parent, Children::with(vec![child]));
        (parent, child)
    }

    #[test]
    fn root_entity_gets_identity_world_transform() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 50.0, y: 75.0, rotation: 1.0, scale: 2.0 });

        run(&mut world);

        let wt = world.world_transforms.get(e).unwrap();
        assert!((wt.x - 50.0).abs() < 1e-10);
        assert!((wt.y - 75.0).abs() < 1e-10);
        assert!((wt.rotation - 1.0).abs() < 1e-10);
        assert!((wt.scale - 2.0).abs() < 1e-10);
    }

    #[test]
    fn parent_child_position_offset() {
        let mut world = World::new();
        let (_, child) = setup_parent_child(&mut world);

        run(&mut world);

        let wt = world.world_transforms.get(child).unwrap();
        assert!((wt.x - 110.0).abs() < 1e-10);
        assert!((wt.y - 220.0).abs() < 1e-10);
    }

    #[test]
    fn parent_rotation_rotates_child_offset() {
        let mut world = World::new();
        let parent = world.spawn();
        let child = world.spawn();
        world.transforms.insert(parent, Transform {
            x: 0.0, y: 0.0,
            rotation: std::f64::consts::FRAC_PI_2, // 90 degrees
            scale: 1.0,
        });
        world.transforms.insert(child, Transform { x: 10.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.parents.insert(child, Parent::new(parent));

        run(&mut world);

        let wt = world.world_transforms.get(child).unwrap();
        // (10, 0) rotated 90 degrees = (0, 10)
        assert!(wt.x.abs() < 1e-10, "x should be ~0, got {}", wt.x);
        assert!((wt.y - 10.0).abs() < 1e-10, "y should be ~10, got {}", wt.y);
    }

    #[test]
    fn parent_scale_scales_child_offset() {
        let mut world = World::new();
        let parent = world.spawn();
        let child = world.spawn();
        world.transforms.insert(parent, Transform { x: 0.0, y: 0.0, rotation: 0.0, scale: 2.0 });
        world.transforms.insert(child, Transform { x: 5.0, y: 3.0, rotation: 0.0, scale: 1.0 });
        world.parents.insert(child, Parent::new(parent));

        run(&mut world);

        let wt = world.world_transforms.get(child).unwrap();
        assert!((wt.x - 10.0).abs() < 1e-10);
        assert!((wt.y - 6.0).abs() < 1e-10);
        assert!((wt.scale - 2.0).abs() < 1e-10);
    }

    #[test]
    fn grandchild_inherits_hierarchy() {
        let mut world = World::new();
        let grandparent = world.spawn();
        let parent = world.spawn();
        let child = world.spawn();

        world.transforms.insert(grandparent, Transform { x: 100.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.transforms.insert(parent, Transform { x: 50.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.transforms.insert(child, Transform { x: 25.0, y: 0.0, rotation: 0.0, scale: 1.0 });

        world.parents.insert(parent, Parent::new(grandparent));
        world.parents.insert(child, Parent::new(parent));

        run(&mut world);

        let wt = world.world_transforms.get(child).unwrap();
        // 100 + 50 + 25 = 175
        assert!((wt.x - 175.0).abs() < 1e-10);
    }

    #[test]
    fn child_rotation_additive() {
        let mut world = World::new();
        let parent = world.spawn();
        let child = world.spawn();
        world.transforms.insert(parent, Transform { x: 0.0, y: 0.0, rotation: 1.0, scale: 1.0 });
        world.transforms.insert(child, Transform { x: 0.0, y: 0.0, rotation: 0.5, scale: 1.0 });
        world.parents.insert(child, Parent::new(parent));

        run(&mut world);

        let wt = world.world_transforms.get(child).unwrap();
        assert!((wt.rotation - 1.5).abs() < 1e-10);
    }

    #[test]
    fn child_scale_multiplicative() {
        let mut world = World::new();
        let parent = world.spawn();
        let child = world.spawn();
        world.transforms.insert(parent, Transform { x: 0.0, y: 0.0, rotation: 0.0, scale: 3.0 });
        world.transforms.insert(child, Transform { x: 0.0, y: 0.0, rotation: 0.0, scale: 2.0 });
        world.parents.insert(child, Parent::new(parent));

        run(&mut world);

        let wt = world.world_transforms.get(child).unwrap();
        assert!((wt.scale - 6.0).abs() < 1e-10);
    }

    #[test]
    fn multiple_children_of_same_parent() {
        let mut world = World::new();
        let parent = world.spawn();
        let c1 = world.spawn();
        let c2 = world.spawn();

        world.transforms.insert(parent, Transform { x: 10.0, y: 10.0, rotation: 0.0, scale: 1.0 });
        world.transforms.insert(c1, Transform { x: 5.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.transforms.insert(c2, Transform { x: 0.0, y: 5.0, rotation: 0.0, scale: 1.0 });
        world.parents.insert(c1, Parent::new(parent));
        world.parents.insert(c2, Parent::new(parent));

        run(&mut world);

        let wt1 = world.world_transforms.get(c1).unwrap();
        let wt2 = world.world_transforms.get(c2).unwrap();
        assert!((wt1.x - 15.0).abs() < 1e-10);
        assert!((wt2.y - 15.0).abs() < 1e-10);
    }

    #[test]
    fn entity_without_transform_is_skipped() {
        let mut world = World::new();
        let e = world.spawn();
        // No transform inserted
        run(&mut world);
        assert!(world.world_transforms.get(e).is_none());
    }
}
