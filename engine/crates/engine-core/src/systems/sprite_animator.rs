/// SYSTEM: sprite_animator
/// READS: SpriteAnimator, TimeScale
/// WRITES: SpriteAnimator (advance frame, update current_tile)
/// ORDER: runs after state_machine, before renderer

use crate::ecs::World;

pub fn run(world: &mut World, dt: f64) {
    let entities: Vec<_> = world.sprite_animators.iter()
        .map(|(e, _)| e)
        .collect();

    for entity in entities {
        let effective_dt = if let Some(ts) = world.time_scales.get(entity) {
            ts.apply(dt)
        } else {
            dt
        };

        if let Some(anim) = world.sprite_animators.get_mut(entity) {
            // Clear one-frame flag
            anim.just_finished = false;

            if !anim.playing {
                continue;
            }

            // Find the current clip
            let clip_idx = anim.clips.iter().position(|c| c.name == anim.current_clip);
            let clip_idx = match clip_idx {
                Some(i) => i,
                None => continue,
            };

            let frame_duration = anim.clips[clip_idx].frame_duration;
            let num_frames = anim.clips[clip_idx].frames.len();
            let looping = anim.clips[clip_idx].looping;

            if num_frames == 0 || frame_duration <= 0.0 {
                continue;
            }

            // Advance elapsed time
            anim.elapsed += effective_dt * anim.speed;

            // Calculate frame index
            let total_duration = num_frames as f64 * frame_duration;
            if anim.elapsed >= total_duration {
                if looping {
                    anim.elapsed %= total_duration;
                } else {
                    // Clamp to last frame
                    anim.elapsed = total_duration;
                    anim.current_frame_index = num_frames - 1;
                    anim.current_tile = anim.clips[clip_idx].frames[num_frames - 1];
                    anim.just_finished = true;
                    anim.playing = false;
                    continue;
                }
            }

            let raw_index = (anim.elapsed / frame_duration).floor();
            let frame_index = (raw_index.max(0.0) as usize).min(num_frames - 1);
            anim.current_frame_index = frame_index;
            anim.current_tile = anim.clips[clip_idx].frames[frame_index];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::World;
    use crate::components::sprite_animator::{SpriteAnimator, AnimationClip};

    #[test]
    fn advances_frames() {
        let mut world = World::new();
        let e = world.spawn();
        world.sprite_animators.insert(e, SpriteAnimator::new("walk")
            .with_clip(AnimationClip::new("walk", vec![0, 1, 2, 3], 0.25, true)));

        run(&mut world, 0.0);
        assert_eq!(world.sprite_animators.get(e).unwrap().current_tile, 0);

        run(&mut world, 0.3);
        assert_eq!(world.sprite_animators.get(e).unwrap().current_tile, 1);

        run(&mut world, 0.25);
        assert_eq!(world.sprite_animators.get(e).unwrap().current_tile, 2);
    }

    #[test]
    fn loops_back_to_start() {
        let mut world = World::new();
        let e = world.spawn();
        world.sprite_animators.insert(e, SpriteAnimator::new("walk")
            .with_clip(AnimationClip::new("walk", vec![0, 1, 2, 3], 0.25, true)));

        // total duration = 1.0
        run(&mut world, 1.1);
        let anim = world.sprite_animators.get(e).unwrap();
        assert!(anim.playing);
        assert_eq!(anim.current_tile, 0); // wrapped back
    }

    #[test]
    fn non_looping_stops_at_end() {
        let mut world = World::new();
        let e = world.spawn();
        world.sprite_animators.insert(e, SpriteAnimator::new("attack")
            .with_clip(AnimationClip::new("attack", vec![10, 11, 12], 0.1, false)));

        run(&mut world, 0.35);
        let anim = world.sprite_animators.get(e).unwrap();
        assert!(!anim.playing);
        assert!(anim.just_finished);
        assert_eq!(anim.current_tile, 12);
        assert_eq!(anim.current_frame_index, 2);
    }

    #[test]
    fn just_finished_clears_next_frame() {
        let mut world = World::new();
        let e = world.spawn();
        world.sprite_animators.insert(e, SpriteAnimator::new("die")
            .with_clip(AnimationClip::new("die", vec![5, 6], 0.1, false)));

        run(&mut world, 0.25); // finishes
        assert!(world.sprite_animators.get(e).unwrap().just_finished);

        run(&mut world, 0.01); // next frame clears it
        assert!(!world.sprite_animators.get(e).unwrap().just_finished);
    }

    #[test]
    fn stopped_animator_doesnt_advance() {
        let mut world = World::new();
        let e = world.spawn();
        let mut anim = SpriteAnimator::new("walk")
            .with_clip(AnimationClip::new("walk", vec![0, 1, 2, 3], 0.25, true));
        anim.stop();
        world.sprite_animators.insert(e, anim);

        run(&mut world, 1.0);
        assert_eq!(world.sprite_animators.get(e).unwrap().current_tile, 0);
    }

    #[test]
    fn respects_time_scale() {
        let mut world = World::new();
        let e = world.spawn();
        world.sprite_animators.insert(e, SpriteAnimator::new("walk")
            .with_clip(AnimationClip::new("walk", vec![0, 1, 2, 3], 0.25, true)));
        world.time_scales.insert(e, crate::components::time_scale::TimeScale { scale: 2.0 });

        // dt=0.25 but time scale 2x → effective 0.5 → frame 2
        run(&mut world, 0.25);
        assert_eq!(world.sprite_animators.get(e).unwrap().current_tile, 2);
    }

    #[test]
    fn speed_multiplier() {
        let mut world = World::new();
        let e = world.spawn();
        let mut anim = SpriteAnimator::new("walk")
            .with_clip(AnimationClip::new("walk", vec![0, 1, 2, 3], 0.25, true));
        anim.speed = 2.0;
        world.sprite_animators.insert(e, anim);

        // dt=0.25, speed=2 → elapsed=0.5 → frame 2
        run(&mut world, 0.25);
        assert_eq!(world.sprite_animators.get(e).unwrap().current_tile, 2);
    }

    #[test]
    fn missing_clip_is_safe() {
        let mut world = World::new();
        let e = world.spawn();
        world.sprite_animators.insert(e, SpriteAnimator::new("nonexistent"));

        run(&mut world, 1.0); // should not panic
    }

    #[test]
    fn switch_clip_mid_animation() {
        let mut world = World::new();
        let e = world.spawn();
        world.sprite_animators.insert(e, SpriteAnimator::new("walk")
            .with_clip(AnimationClip::new("walk", vec![0, 1, 2, 3], 0.25, true))
            .with_clip(AnimationClip::new("idle", vec![10], 1.0, true)));

        run(&mut world, 0.3);
        assert_eq!(world.sprite_animators.get(e).unwrap().current_tile, 1);

        world.sprite_animators.get_mut(e).unwrap().play("idle");
        run(&mut world, 0.1);
        assert_eq!(world.sprite_animators.get(e).unwrap().current_tile, 10);
    }
}
