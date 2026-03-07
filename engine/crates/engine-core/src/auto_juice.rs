/// AutoJuice — Automatic Game Feel Pipeline
///
/// Declaratively maps game events (collisions, spawns, despawns, bus events)
/// to juicy effects (particles, screen shake, flash, sound cues, time pause).
/// Add rules once; the system fires effects automatically each frame.

use crate::ecs::Entity;
use crate::rendering::color::Color;
use crate::rendering::particles::ParticlePool;
use crate::rendering::post_fx::PostFxConfig;
use crate::rendering::screen_fx::{ScreenEffect, ScreenFxStack};
use crate::events::{EventQueue, EventKind};
use crate::event_bus::EventBus;

// ─── Trigger ────────────────────────────────────────────────────────────

/// What game event should fire a juice effect.
#[derive(Clone, Debug, PartialEq)]
pub enum JuiceTrigger {
    /// Two tagged entities collide.
    OnCollision { tag_a: String, tag_b: String },
    /// An entity with the given tag is despawned.
    OnDespawn { tag: String },
    /// An entity with the given tag is spawned.
    OnSpawn { tag: String },
    /// An event is published on the named EventBus channel.
    OnEvent { channel: String },
}

// ─── Effect configuration ───────────────────────────────────────────────

/// Which entity/target a flash should apply to.
#[derive(Clone, Debug, PartialEq)]
pub enum FlashTarget {
    /// Flash a specific entity (unused until per-entity flash wiring exists).
    Entity(Entity),
    /// Full-screen flash via ScreenFxStack.
    Screen,
}

/// Configuration for a particle burst.
#[derive(Clone, Debug)]
pub struct ParticleConfig {
    pub count: u32,
    pub color: Color,
    pub speed: f64,
    pub life: f64,
}

/// Configuration for screen/camera shake.
#[derive(Clone, Debug)]
pub struct ShakeConfig {
    pub intensity: f64,
    pub duration: f64,
}

/// Configuration for a visual flash.
#[derive(Clone, Debug)]
pub struct FlashConfig {
    pub target: FlashTarget,
    pub color: Color,
    pub duration: f64,
}

/// A composite juice effect. All fields are optional; only set fields fire.
#[derive(Clone, Debug, Default)]
pub struct JuiceEffect {
    pub particles: Option<ParticleConfig>,
    pub shake: Option<ShakeConfig>,
    pub flash: Option<FlashConfig>,
    pub sound: Option<String>,
    pub time_pause: Option<f64>,
}

impl JuiceEffect {
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder: attach a particle burst.
    pub fn with_particles(mut self, count: u32, color: Color, speed: f64, life: f64) -> Self {
        self.particles = Some(ParticleConfig { count, color, speed, life });
        self
    }

    /// Builder: attach screen shake.
    pub fn with_shake(mut self, intensity: f64, duration: f64) -> Self {
        self.shake = Some(ShakeConfig { intensity, duration });
        self
    }

    /// Builder: attach a screen flash.
    pub fn with_screen_flash(mut self, color: Color, duration: f64) -> Self {
        self.flash = Some(FlashConfig {
            target: FlashTarget::Screen,
            color,
            duration,
        });
        self
    }

    /// Builder: attach a sound palette name.
    pub fn with_sound(mut self, palette: &str) -> Self {
        self.sound = Some(palette.to_string());
        self
    }

    /// Builder: attach a time-pause (freeze frames).
    pub fn with_time_pause(mut self, freeze_duration: f64) -> Self {
        self.time_pause = Some(freeze_duration);
        self
    }
}

// ─── AutoJuice system ──────────────────────────────────────────────────

/// Collects trigger → effect rules and applies them each frame.
#[derive(Clone, Debug, Default)]
pub struct AutoJuiceSystem {
    pub rules: Vec<(JuiceTrigger, JuiceEffect)>,
    /// Accumulated time-pause remaining (freeze frames).
    pub freeze_remaining: f64,
    /// Queue of sound palette names to play this frame.
    pub sound_queue: Vec<String>,
}

impl AutoJuiceSystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a trigger → effect rule.
    pub fn add_rule(&mut self, trigger: JuiceTrigger, effect: JuiceEffect) {
        self.rules.push((trigger, effect));
    }

    /// Number of registered rules.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Clear all rules.
    pub fn clear(&mut self) {
        self.rules.clear();
        self.freeze_remaining = 0.0;
        self.sound_queue.clear();
    }

    /// Evaluate all rules against the current frame's events and fire effects.
    ///
    /// Call this once per frame after collision detection but before rendering.
    /// `collision_events` is the physics EventQueue; `bus` is the EventBus.
    /// `tags_lookup` maps Entity → list of tag strings (snapshot from World.tags).
    /// `spawn_tags` / `despawn_tags` are tags of entities spawned/despawned this frame.
    pub fn apply(
        &mut self,
        collision_events: &EventQueue,
        bus: &EventBus,
        tags_lookup: &dyn Fn(Entity) -> Vec<String>,
        spawn_tags: &[String],
        despawn_tags: &[String],
        particles: &mut ParticlePool,
        post_fx: &mut PostFxConfig,
        screen_fx: &mut ScreenFxStack,
        frame: u64,
    ) {
        self.sound_queue.clear();

        // Snapshot collision pairs with their tags for matching.
        let mut collision_pairs: Vec<(Vec<String>, Vec<String>, f64, f64)> = Vec::new();
        for event in collision_events.iter() {
            if let EventKind::Collision { entity_a, entity_b, contact, .. } = &event.kind {
                let tags_a = tags_lookup(*entity_a);
                let tags_b = tags_lookup(*entity_b);
                collision_pairs.push((tags_a, tags_b, contact.0, contact.1));
            }
        }

        // Collect effects to fire (avoids borrow conflict with self.rules + self.fire_effect)
        let mut to_fire: Vec<(usize, f64, f64)> = Vec::new();
        for (idx, (trigger, _)) in self.rules.iter().enumerate() {
            match trigger {
                JuiceTrigger::OnCollision { tag_a, tag_b } => {
                    for (ta, tb, cx, cy) in &collision_pairs {
                        let matched = (ta.iter().any(|t| t == tag_a) && tb.iter().any(|t| t == tag_b))
                            || (ta.iter().any(|t| t == tag_b) && tb.iter().any(|t| t == tag_a));
                        if matched {
                            to_fire.push((idx, *cx, *cy));
                        }
                    }
                }
                JuiceTrigger::OnSpawn { tag } => {
                    for st in spawn_tags {
                        if st == tag {
                            to_fire.push((idx, 0.0, 0.0));
                        }
                    }
                }
                JuiceTrigger::OnDespawn { tag } => {
                    for dt_tag in despawn_tags {
                        if dt_tag == tag {
                            to_fire.push((idx, 0.0, 0.0));
                        }
                    }
                }
                JuiceTrigger::OnEvent { channel } => {
                    if bus.has(channel) {
                        to_fire.push((idx, 0.0, 0.0));
                    }
                }
            }
        }

        // Fire collected effects
        for (idx, x, y) in to_fire {
            let effect = self.rules[idx].1.clone();
            Self::fire_effect_inner(
                &effect, x, y,
                &mut self.sound_queue, &mut self.freeze_remaining,
                particles, post_fx, screen_fx, frame,
            );
        }
    }

    /// Fire a single effect at the given world position.
    fn fire_effect_inner(
        effect: &JuiceEffect,
        x: f64,
        y: f64,
        sound_queue: &mut Vec<String>,
        freeze_remaining: &mut f64,
        particles: &mut ParticlePool,
        post_fx: &mut PostFxConfig,
        screen_fx: &mut ScreenFxStack,
        frame: u64,
    ) {
        // Particles
        if let Some(ref pc) = effect.particles {
            particles.spawn_burst(
                x, y,
                pc.count,
                pc.speed * 0.3, pc.speed,
                pc.life,
                2.0, 0.5,
                pc.color,
                pc.color.with_alpha(0),
                frame,
            );
        }

        // Screen shake
        if let Some(ref sc) = effect.shake {
            post_fx.shake_intensity = sc.intensity;
            post_fx.shake_remaining = sc.duration;
        }

        // Flash
        if let Some(ref fc) = effect.flash {
            match fc.target {
                FlashTarget::Screen => {
                    screen_fx.push(
                        ScreenEffect::Flash {
                            color: fc.color,
                            intensity: 1.0,
                        },
                        fc.duration,
                    );
                }
                FlashTarget::Entity(_) => {
                    // Per-entity flash would go through world.flashes —
                    // log a note that this path is a stub.
                    crate::log::log("[AutoJuice] Per-entity flash not yet wired");
                }
            }
        }

        // Sound
        if let Some(ref snd) = effect.sound {
            sound_queue.push(snd.clone());
        }

        // Time pause (freeze frames)
        if let Some(pause) = effect.time_pause {
            *freeze_remaining = freeze_remaining.max(pause);
        }
    }

    /// Tick down the freeze-frame timer. Returns true when the game should be frozen.
    pub fn tick_freeze(&mut self, dt: f64) -> bool {
        if self.freeze_remaining > 0.0 {
            self.freeze_remaining -= dt;
            if self.freeze_remaining < 0.0 {
                self.freeze_remaining = 0.0;
            }
            true
        } else {
            false
        }
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventKind;

    fn empty_tags(_e: Entity) -> Vec<String> {
        Vec::new()
    }

    fn make_tags_fn(pairs: Vec<(Entity, Vec<String>)>) -> impl Fn(Entity) -> Vec<String> {
        move |e: Entity| {
            for (ent, tags) in &pairs {
                if *ent == e {
                    return tags.clone();
                }
            }
            Vec::new()
        }
    }

    #[test]
    fn new_system_has_no_rules() {
        let sys = AutoJuiceSystem::new();
        assert_eq!(sys.rule_count(), 0);
        assert!(sys.sound_queue.is_empty());
    }

    #[test]
    fn add_rule_increments_count() {
        let mut sys = AutoJuiceSystem::new();
        sys.add_rule(
            JuiceTrigger::OnEvent { channel: "test".into() },
            JuiceEffect::new(),
        );
        assert_eq!(sys.rule_count(), 1);
        sys.add_rule(
            JuiceTrigger::OnDespawn { tag: "enemy".into() },
            JuiceEffect::new().with_shake(5.0, 0.2),
        );
        assert_eq!(sys.rule_count(), 2);
    }

    #[test]
    fn clear_removes_all_rules() {
        let mut sys = AutoJuiceSystem::new();
        sys.add_rule(
            JuiceTrigger::OnEvent { channel: "x".into() },
            JuiceEffect::new(),
        );
        sys.freeze_remaining = 1.0;
        sys.clear();
        assert_eq!(sys.rule_count(), 0);
        assert!((sys.freeze_remaining - 0.0).abs() < 1e-10);
    }

    #[test]
    fn collision_trigger_fires_particles() {
        let mut sys = AutoJuiceSystem::new();
        sys.add_rule(
            JuiceTrigger::OnCollision {
                tag_a: "bullet".into(),
                tag_b: "enemy".into(),
            },
            JuiceEffect::new().with_particles(10, Color::RED, 100.0, 0.5),
        );

        let mut events = EventQueue::default();
        events.push(EventKind::Collision {
            entity_a: Entity(1),
            entity_b: Entity(2),
            normal: (0.0, 1.0),
            contact: (50.0, 60.0),
        });

        let tags_fn = make_tags_fn(vec![
            (Entity(1), vec!["bullet".into()]),
            (Entity(2), vec!["enemy".into()]),
        ]);

        let bus = EventBus::new();
        let mut particles = ParticlePool::new();
        let mut post_fx = PostFxConfig::default();
        let mut screen_fx = ScreenFxStack::new();

        sys.apply(
            &events, &bus, &tags_fn,
            &[], &[],
            &mut particles, &mut post_fx, &mut screen_fx, 1,
        );

        assert!(particles.count() > 0, "Particles should have been spawned");
    }

    #[test]
    fn event_trigger_fires_shake_and_sound() {
        let mut sys = AutoJuiceSystem::new();
        sys.add_rule(
            JuiceTrigger::OnEvent { channel: "explosion".into() },
            JuiceEffect::new()
                .with_shake(8.0, 0.3)
                .with_sound("boom"),
        );

        let events = EventQueue::default();
        let mut bus = EventBus::new();
        bus.emit("explosion");

        let mut particles = ParticlePool::new();
        let mut post_fx = PostFxConfig::default();
        let mut screen_fx = ScreenFxStack::new();

        sys.apply(
            &events, &bus, &empty_tags,
            &[], &[],
            &mut particles, &mut post_fx, &mut screen_fx, 1,
        );

        assert!((post_fx.shake_intensity - 8.0).abs() < 1e-10);
        assert!((post_fx.shake_remaining - 0.3).abs() < 1e-10);
        assert_eq!(sys.sound_queue, vec!["boom".to_string()]);
    }

    #[test]
    fn despawn_trigger_fires_screen_flash() {
        let mut sys = AutoJuiceSystem::new();
        sys.add_rule(
            JuiceTrigger::OnDespawn { tag: "boss".into() },
            JuiceEffect::new().with_screen_flash(Color::WHITE, 0.5),
        );

        let events = EventQueue::default();
        let bus = EventBus::new();
        let mut particles = ParticlePool::new();
        let mut post_fx = PostFxConfig::default();
        let mut screen_fx = ScreenFxStack::new();

        sys.apply(
            &events, &bus, &empty_tags,
            &[], &["boss".to_string()],
            &mut particles, &mut post_fx, &mut screen_fx, 1,
        );

        assert!(!screen_fx.is_empty(), "Screen flash should have been pushed");
    }

    #[test]
    fn tick_freeze_counts_down() {
        let mut sys = AutoJuiceSystem::new();
        sys.freeze_remaining = 0.1;

        assert!(sys.tick_freeze(0.05));
        assert!((sys.freeze_remaining - 0.05).abs() < 1e-10);

        assert!(sys.tick_freeze(0.06));
        // Should clamp to 0
        assert!((sys.freeze_remaining - 0.0).abs() < 1e-10);

        // No longer frozen
        assert!(!sys.tick_freeze(0.01));
    }

    #[test]
    fn collision_trigger_symmetric_matching() {
        // tag_a=bullet, tag_b=enemy should also match if entity_a has "enemy"
        // and entity_b has "bullet"
        let mut sys = AutoJuiceSystem::new();
        sys.add_rule(
            JuiceTrigger::OnCollision {
                tag_a: "bullet".into(),
                tag_b: "enemy".into(),
            },
            JuiceEffect::new().with_particles(5, Color::GREEN, 50.0, 0.3),
        );

        let mut events = EventQueue::default();
        // Reversed: entity_a is enemy, entity_b is bullet
        events.push(EventKind::Collision {
            entity_a: Entity(10),
            entity_b: Entity(20),
            normal: (1.0, 0.0),
            contact: (30.0, 40.0),
        });

        let tags_fn = make_tags_fn(vec![
            (Entity(10), vec!["enemy".into()]),
            (Entity(20), vec!["bullet".into()]),
        ]);

        let bus = EventBus::new();
        let mut particles = ParticlePool::new();
        let mut post_fx = PostFxConfig::default();
        let mut screen_fx = ScreenFxStack::new();

        sys.apply(
            &events, &bus, &tags_fn,
            &[], &[],
            &mut particles, &mut post_fx, &mut screen_fx, 1,
        );

        assert!(particles.count() > 0, "Symmetric matching should trigger particles");
    }

    #[test]
    fn spawn_trigger_fires_time_pause() {
        let mut sys = AutoJuiceSystem::new();
        sys.add_rule(
            JuiceTrigger::OnSpawn { tag: "powerup".into() },
            JuiceEffect::new().with_time_pause(0.1),
        );

        let events = EventQueue::default();
        let bus = EventBus::new();
        let mut particles = ParticlePool::new();
        let mut post_fx = PostFxConfig::default();
        let mut screen_fx = ScreenFxStack::new();

        sys.apply(
            &events, &bus, &empty_tags,
            &["powerup".to_string()], &[],
            &mut particles, &mut post_fx, &mut screen_fx, 1,
        );

        assert!((sys.freeze_remaining - 0.1).abs() < 1e-10);
    }

    #[test]
    fn no_matching_events_does_nothing() {
        let mut sys = AutoJuiceSystem::new();
        sys.add_rule(
            JuiceTrigger::OnEvent { channel: "nope".into() },
            JuiceEffect::new().with_shake(10.0, 1.0),
        );

        let events = EventQueue::default();
        let bus = EventBus::new(); // no events
        let mut particles = ParticlePool::new();
        let mut post_fx = PostFxConfig::default();
        let mut screen_fx = ScreenFxStack::new();

        sys.apply(
            &events, &bus, &empty_tags,
            &[], &[],
            &mut particles, &mut post_fx, &mut screen_fx, 1,
        );

        assert!((post_fx.shake_remaining - 0.0).abs() < 1e-10);
        assert!(particles.count() == 0);
    }

    #[test]
    fn juice_effect_builder_chain() {
        let effect = JuiceEffect::new()
            .with_particles(20, Color::BLUE, 200.0, 1.0)
            .with_shake(6.0, 0.4)
            .with_screen_flash(Color::WHITE, 0.2)
            .with_sound("hit_01")
            .with_time_pause(0.05);

        assert!(effect.particles.is_some());
        assert!(effect.shake.is_some());
        assert!(effect.flash.is_some());
        assert_eq!(effect.sound, Some("hit_01".to_string()));
        assert_eq!(effect.time_pause, Some(0.05));

        let pc = effect.particles.as_ref().expect("particles");
        assert_eq!(pc.count, 20);
        assert_eq!(pc.color, Color::BLUE);
    }
}
