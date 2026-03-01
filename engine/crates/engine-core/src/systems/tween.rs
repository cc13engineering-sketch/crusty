/// SYSTEM: tween
/// READS: PropertyTween, TimeScale
/// WRITES: Transform (x, y, rotation, scale), RigidBody (vx, vy), Renderable (alpha)
/// ORDER: runs before physics, after lifecycle

use crate::ecs::World;
use crate::components::property_tween::{TweenTarget, PropertyTween};

pub fn run(world: &mut World, dt: f64) {
    // Collect entity IDs that have tweens (snapshot to avoid borrow conflict)
    let entities: Vec<_> = world.property_tweens.iter()
        .map(|(e, _)| e)
        .collect();

    for entity in entities {
        // Get per-entity time scale
        let effective_dt = if let Some(ts) = world.time_scales.get(entity) {
            ts.apply(dt)
        } else {
            dt
        };

        if let Some(pt) = world.property_tweens.get_mut(entity) {
            // Tick all tweens
            for tween in &mut pt.tweens {
                tween.elapsed += effective_dt;

                // Handle looping
                if tween.elapsed >= tween.duration {
                    if tween.looping {
                        if tween.ping_pong {
                            tween.forward = !tween.forward;
                        }
                        tween.elapsed -= tween.duration;
                    }
                }
            }

            // Collect values to apply (snapshot to release borrow)
            let values: Vec<_> = pt.tweens.iter()
                .map(|tw| (tw.target.clone(), tw.current_value()))
                .collect();

            // Remove completed tweens
            pt.tweens.retain(|tw| !tw.is_complete());

            // Apply values to target components
            for (target, value) in values {
                match target {
                    TweenTarget::X => {
                        if let Some(t) = world.transforms.get_mut(entity) {
                            t.x = value;
                        }
                    }
                    TweenTarget::Y => {
                        if let Some(t) = world.transforms.get_mut(entity) {
                            t.y = value;
                        }
                    }
                    TweenTarget::Rotation => {
                        if let Some(t) = world.transforms.get_mut(entity) {
                            t.rotation = value;
                        }
                    }
                    TweenTarget::Scale => {
                        if let Some(t) = world.transforms.get_mut(entity) {
                            t.scale = value;
                        }
                    }
                    TweenTarget::VelocityX => {
                        if let Some(rb) = world.rigidbodies.get_mut(entity) {
                            rb.vx = value;
                        }
                    }
                    TweenTarget::VelocityY => {
                        if let Some(rb) = world.rigidbodies.get_mut(entity) {
                            rb.vy = value;
                        }
                    }
                    TweenTarget::Alpha => {
                        // Alpha is 0..255, value is 0.0..255.0
                        if let Some(r) = world.renderables.get_mut(entity) {
                            let alpha = (value.max(0.0).min(255.0)) as u8;
                            match &mut r.visual {
                                crate::components::Visual::Circle { ref mut color, .. } => {
                                    *color = color.with_alpha(alpha);
                                }
                                crate::components::Visual::Rect { ref mut color, .. } => {
                                    *color = color.with_alpha(alpha);
                                }
                                crate::components::Visual::Line { ref mut color, .. } => {
                                    *color = color.with_alpha(alpha);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    // Remove PropertyTween components with no remaining tweens
    let empty: Vec<_> = world.property_tweens.iter()
        .filter(|(_, pt)| pt.tweens.is_empty())
        .map(|(e, _)| e)
        .collect();
    for entity in empty {
        world.property_tweens.remove(entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::World;
    use crate::components::*;
    use crate::components::property_tween::*;

    #[test]
    fn tween_moves_x() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.property_tweens.insert(e, PropertyTween::new()
            .with_tween(Tween::new(TweenTarget::X, 0.0, 100.0, 1.0, EasingFn::Linear)));

        run(&mut world, 0.5);
        let x = world.transforms.get(e).map(|t| t.x).unwrap_or(0.0);
        assert!((x - 50.0).abs() < 1.0);
    }

    #[test]
    fn tween_moves_y() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0 });
        world.property_tweens.insert(e, PropertyTween::new()
            .with_tween(Tween::new(TweenTarget::Y, 0.0, 200.0, 2.0, EasingFn::Linear)));

        run(&mut world, 1.0);
        let y = world.transforms.get(e).map(|t| t.y).unwrap_or(0.0);
        assert!((y - 100.0).abs() < 1.0);
    }

    #[test]
    fn tween_completes_and_removes() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform::default());
        world.property_tweens.insert(e, PropertyTween::new()
            .with_tween(Tween::new(TweenTarget::X, 0.0, 10.0, 0.5, EasingFn::Linear)));

        run(&mut world, 0.6);
        assert!(!world.property_tweens.has(e));
    }

    #[test]
    fn looping_tween_does_not_complete() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform::default());
        let mut tw = Tween::new(TweenTarget::X, 0.0, 10.0, 0.5, EasingFn::Linear);
        tw.looping = true;
        world.property_tweens.insert(e, PropertyTween::new().with_tween(tw));

        run(&mut world, 0.6);
        assert!(world.property_tweens.has(e));
    }

    #[test]
    fn time_scale_affects_tween_speed() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform::default());
        world.property_tweens.insert(e, PropertyTween::new()
            .with_tween(Tween::new(TweenTarget::X, 0.0, 100.0, 1.0, EasingFn::Linear)));
        world.time_scales.insert(e, crate::components::TimeScale::new(0.5));

        run(&mut world, 1.0); // effective dt = 0.5
        let x = world.transforms.get(e).map(|t| t.x).unwrap_or(0.0);
        assert!((x - 50.0).abs() < 1.0);
    }

    #[test]
    fn frozen_time_scale_stops_tween() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform::default());
        world.property_tweens.insert(e, PropertyTween::new()
            .with_tween(Tween::new(TweenTarget::X, 0.0, 100.0, 1.0, EasingFn::Linear)));
        world.time_scales.insert(e, crate::components::TimeScale::frozen());

        run(&mut world, 1.0);
        let x = world.transforms.get(e).map(|t| t.x).unwrap_or(-1.0);
        assert!((x - 0.0).abs() < 1.0);
    }
}
