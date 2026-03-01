/// SYSTEM: flash
/// READS: EntityFlash, TimeScale
/// WRITES: EntityFlash (tick), Renderable (visibility for blink)
/// ORDER: runs after tween, before rendering

use crate::ecs::World;
use crate::components::entity_flash::FlashMode;

pub fn run(world: &mut World, dt: f64) {
    // Collect entities with flashes (snapshot avoids aliasing the store).
    let entities: Vec<_> = world.entity_flashes.iter()
        .map(|(e, _)| e)
        .collect();

    // Entities whose flash expired this tick — gathered to avoid a second iteration.
    let mut expired: Vec<crate::ecs::Entity> = Vec::new();

    for entity in entities {
        let effective_dt = if let Some(ts) = world.time_scales.get(entity) {
            ts.apply(dt)
        } else {
            dt
        };

        if let Some(flash) = world.entity_flashes.get_mut(entity) {
            let now_expired = flash.tick(effective_dt);

            if now_expired {
                expired.push(entity);
            } else {
                // For Blink mode, keep renderable visibility in sync with the blink state.
                if let FlashMode::Blink { is_on, .. } = &flash.mode {
                    if let Some(r) = world.renderables.get_mut(entity) {
                        r.visible = *is_on;
                    }
                }
            }
        }
    }

    // Remove expired flashes and restore visibility for any that were blinking.
    for entity in expired {
        if let Some(r) = world.renderables.get_mut(entity) {
            r.visible = true;
        }
        world.entity_flashes.remove(entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::World;
    use crate::components::*;
    use crate::components::entity_flash::EntityFlash;
    use crate::rendering::color::Color;

    #[test]
    fn hit_flash_expires_and_is_removed() {
        let mut world = World::new();
        let e = world.spawn();
        world.entity_flashes.insert(e, EntityFlash::hit_flash(Color::WHITE, 0.2));

        run(&mut world, 0.3);
        assert!(!world.entity_flashes.has(e));
    }

    #[test]
    fn blink_toggles_visibility() {
        let mut world = World::new();
        let e = world.spawn();
        world.renderables.insert(e, Renderable {
            visual: Visual::Circle { radius: 5.0, color: Color::RED, filled: true },
            layer: 0, visible: true,
        });
        world.entity_flashes.insert(e, EntityFlash::blink(0.1, 0.1, 2.0));

        // After 0.15s, should have toggled from on to off
        run(&mut world, 0.15);
        assert!(world.entity_flashes.has(e));
    }

    #[test]
    fn blink_restores_visibility_on_expire() {
        let mut world = World::new();
        let e = world.spawn();
        world.renderables.insert(e, Renderable {
            visual: Visual::Circle { radius: 5.0, color: Color::RED, filled: true },
            layer: 0, visible: true,
        });
        world.entity_flashes.insert(e, EntityFlash::blink(0.1, 0.1, 0.2));

        run(&mut world, 0.3);
        assert!(!world.entity_flashes.has(e));
        assert!(world.renderables.get(e).map_or(false, |r| r.visible));
    }

    #[test]
    fn color_pulse_ticks() {
        let mut world = World::new();
        let e = world.spawn();
        world.entity_flashes.insert(e, EntityFlash::color_pulse(Color::RED, 2.0, 1.0));

        run(&mut world, 0.5);
        assert!(world.entity_flashes.has(e));
    }

    #[test]
    fn color_pulse_expires() {
        let mut world = World::new();
        let e = world.spawn();
        world.entity_flashes.insert(e, EntityFlash::color_pulse(Color::RED, 2.0, 0.5));

        run(&mut world, 0.6);
        assert!(!world.entity_flashes.has(e));
    }

    #[test]
    fn time_scale_affects_flash() {
        let mut world = World::new();
        let e = world.spawn();
        world.entity_flashes.insert(e, EntityFlash::hit_flash(Color::WHITE, 1.0));
        world.time_scales.insert(e, TimeScale::new(2.0));

        // 0.6 real seconds * 2.0 scale = 1.2 effective → expired
        run(&mut world, 0.6);
        assert!(!world.entity_flashes.has(e));
    }
}
