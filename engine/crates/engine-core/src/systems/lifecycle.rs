/// SYSTEM: lifecycle
/// READS: Lifetime, Transform, SpawnQueue, EventQueue, TimerQueue, BehaviorRules
/// WRITES: World (spawn/despawn entities), SpawnQueue (drain), GameState (global),
///         TimerQueue (start/cancel), TemplateRegistry (spawn from templates)
/// ORDER: runs at start of tick, before physics
///
/// This system is the heart of runtime entity management. Each frame it:
/// 1. Processes the SpawnQueue (deferred spawns/despawns from other systems).
/// 2. Ticks all Lifetime components and despawns expired entities.
/// 3. Ticks all Timers and collects fired timer names.
/// 4. Evaluates all BehaviorRules against current events, timers, and state.
/// 5. Executes the resulting action queue (despawn, spawn, state changes, etc.).
/// 6. Auto-despawns out-of-bounds dynamic entities.

use crate::ecs::{World, Entity};
use crate::spawn_queue::SpawnQueue;
use crate::events::{EventQueue, EventKind};
use crate::game_state::GameState as GlobalGameState;
use crate::timers::TimerQueue;
use crate::templates::TemplateRegistry;
use crate::behavior::{
    BehaviorRules, ActionQueue, Condition, Action, EntityRef, SpawnAt,
};
use crate::engine::WorldConfig;

pub fn run(
    world: &mut World,
    queue: &mut SpawnQueue,
    config: &WorldConfig,
    events: &EventQueue,
    state: &mut GlobalGameState,
    timers: &mut TimerQueue,
    templates: &TemplateRegistry,
    rules: &mut BehaviorRules,
    dt: f64,
) {
    // --- Phase 1: Process deferred spawns from SpawnQueue ---
    for cmd in queue.spawns.drain(..) {
        let entity = match &cmd.name {
            Some(n) => world.spawn_named(n),
            None => world.spawn(),
        };
        world.transforms.insert(entity, cmd.transform);
        if let Some(rb) = cmd.rigidbody { world.rigidbodies.insert(entity, rb); }
        if let Some(col) = cmd.collider { world.colliders.insert(entity, col); }
        if let Some(rend) = cmd.renderable { world.renderables.insert(entity, rend); }
        if let Some(tags) = cmd.tags { world.tags.insert(entity, tags); }
        if let Some(lt) = cmd.lifetime { world.lifetimes.insert(entity, lt); }
        if let Some(gs) = cmd.game_state { world.game_states.insert(entity, gs); }
        if let Some(beh) = cmd.behavior { world.behaviors.insert(entity, beh); }
    }

    // --- Phase 2: Process deferred despawns ---
    for entity in queue.despawns.drain(..) {
        world.despawn(entity);
    }

    // --- Phase 3: Tick lifetimes and auto-despawn expired entities ---
    let mut expired = Vec::new();
    for (entity, lt) in world.lifetimes.iter_mut() {
        if lt.tick(dt) {
            expired.push(entity);
        }
    }
    for e in expired {
        world.despawn(e);
    }

    // --- Phase 4: Tick timers ---
    timers.tick(dt);

    // --- Phase 5: Evaluate behavior rules ---
    let world_bounds = config.bounds;
    let mut action_queue = ActionQueue::new();
    let mut rules_to_disable: Vec<usize> = Vec::new();

    for (rule_idx, rule) in rules.rules.iter().enumerate() {
        if !rule.enabled { continue; }

        match &rule.condition {
            Condition::Collision { tag_a, tag_b } => {
                for event in events.iter() {
                    if let EventKind::Collision { entity_a, entity_b, .. } = &event.kind {
                        let a_has_tag_a = world.tags.get(*entity_a)
                            .map_or(false, |t| t.has(tag_a));
                        let b_has_tag_b = world.tags.get(*entity_b)
                            .map_or(false, |t| t.has(tag_b));
                        let a_has_tag_b = world.tags.get(*entity_a)
                            .map_or(false, |t| t.has(tag_b));
                        let b_has_tag_a = world.tags.get(*entity_b)
                            .map_or(false, |t| t.has(tag_a));

                        if (a_has_tag_a && b_has_tag_b) || (a_has_tag_b && b_has_tag_a) {
                            let (ea, eb) = if a_has_tag_a && b_has_tag_b {
                                (*entity_a, *entity_b)
                            } else {
                                (*entity_b, *entity_a)
                            };
                            execute_actions(&rule.actions, Some(ea), Some(eb), world, &mut action_queue, world_bounds);
                            if rule.once { rules_to_disable.push(rule_idx); }
                        }
                    }
                }
            }

            Condition::TriggerEnter { trigger_tag, visitor_tag } => {
                for event in events.iter() {
                    if let EventKind::TriggerEnter { trigger, visitor } = &event.kind {
                        let trigger_matches = world.tags.get(*trigger)
                            .map_or(false, |t| t.has(trigger_tag));
                        let visitor_matches = world.tags.get(*visitor)
                            .map_or(false, |t| t.has(visitor_tag));

                        if trigger_matches && visitor_matches {
                            execute_actions(&rule.actions, Some(*trigger), Some(*visitor), world, &mut action_queue, world_bounds);
                            if rule.once { rules_to_disable.push(rule_idx); }
                        }
                    }
                }
            }

            Condition::TimerFired { timer_name } => {
                if timers.fired.iter().any(|n| n == timer_name) {
                    execute_actions(&rule.actions, None, None, world, &mut action_queue, world_bounds);
                    if rule.once { rules_to_disable.push(rule_idx); }
                }
            }

            Condition::StateCheck { key, op, value } => {
                let current = state.get_f64(key).unwrap_or(0.0);
                if op.evaluate(current, *value) {
                    execute_actions(&rule.actions, None, None, world, &mut action_queue, world_bounds);
                    if rule.once { rules_to_disable.push(rule_idx); }
                }
            }

            Condition::KeyPressed { key_code: _ } => {
                // Handled by input_gameplay; reserved for future use.
            }

            Condition::Always => {
                execute_actions(&rule.actions, None, None, world, &mut action_queue, world_bounds);
                if rule.once { rules_to_disable.push(rule_idx); }
            }
        }
    }

    // Disable one-shot rules that fired
    for idx in rules_to_disable.into_iter().rev() {
        if idx < rules.rules.len() {
            rules.rules[idx].enabled = false;
        }
    }

    // --- Phase 6: Execute the action queue ---

    // Despawn from rules (deduplicate)
    let mut despawned: Vec<Entity> = Vec::new();
    for entity in &action_queue.despawns {
        if !despawned.contains(entity) && world.is_alive(*entity) {
            world.despawn(*entity);
            despawned.push(*entity);
        }
    }

    // State changes (global state)
    for (key, value) in &action_queue.state_sets {
        state.set_f64(key, *value);
    }
    for (key, delta) in &action_queue.state_adds {
        state.add_f64(key, *delta);
    }
    for (key, value) in &action_queue.flag_sets {
        state.set_bool(key, *value);
    }

    // Spawn from templates
    for (template_name, x, y) in &action_queue.spawns {
        if let Some(template) = templates.get(template_name) {
            template.spawn_into(world, Some((*x, *y)));
        } else {
            crate::log::warn(&format!("Template '{}' not found for spawn action", template_name));
        }
    }

    // Timer management from rules
    for (name, delay, interval, max_fires) in &action_queue.timer_starts {
        let timer = if let Some(iv) = interval {
            if *max_fires > 0 {
                crate::timers::Timer::repeating_n(name, *delay, *iv, *max_fires)
            } else {
                crate::timers::Timer::repeating(name, *delay, *iv)
            }
        } else {
            crate::timers::Timer::one_shot(name, *delay)
        };
        timers.add(timer);
    }
    for name in &action_queue.timer_cancels {
        timers.cancel(name);
    }

    // Logs
    for msg in &action_queue.logs {
        crate::log::log(msg);
    }

    // --- Phase 7: Auto-despawn out-of-bounds dynamic entities ---
    let margin = 200.0;
    let (bw, bh) = config.bounds;
    let oob: Vec<_> = world.transforms.iter()
        .filter(|(e, t)| {
            let is_dynamic = world.rigidbodies.get(*e)
                .map_or(false, |rb| !rb.is_static);
            is_dynamic && (t.x < -margin || t.x > bw + margin
                || t.y < -margin || t.y > bh + margin)
        })
        .map(|(e, _)| e)
        .collect();
    for e in oob {
        world.despawn(e);
    }
}

/// Translate rule Actions into ActionQueue entries.
fn execute_actions(
    actions: &[Action],
    entity_a: Option<Entity>,
    entity_b: Option<Entity>,
    world: &World,
    queue: &mut ActionQueue,
    world_bounds: (f64, f64),
) {
    for action in actions {
        match action {
            Action::Despawn { entity_ref } => {
                match entity_ref {
                    EntityRef::A => {
                        if let Some(e) = entity_a { queue.despawns.push(e); }
                    }
                    EntityRef::B => {
                        if let Some(e) = entity_b { queue.despawns.push(e); }
                    }
                    EntityRef::Both => {
                        if let Some(e) = entity_a { queue.despawns.push(e); }
                        if let Some(e) = entity_b { queue.despawns.push(e); }
                    }
                }
            }
            Action::AddState { key, delta } => {
                queue.state_adds.push((key.clone(), *delta));
            }
            Action::SetState { key, value } => {
                queue.state_sets.push((key.clone(), *value));
            }
            Action::SetFlag { key, value } => {
                queue.flag_sets.push((key.clone(), *value));
            }
            Action::SpawnTemplate { template_name, at } => {
                let (x, y) = match at {
                    SpawnAt::EntityA => {
                        if let Some(e) = entity_a {
                            world.transforms.get(e).map_or((0.0, 0.0), |t| (t.x, t.y))
                        } else { (0.0, 0.0) }
                    }
                    SpawnAt::EntityB => {
                        if let Some(e) = entity_b {
                            world.transforms.get(e).map_or((0.0, 0.0), |t| (t.x, t.y))
                        } else { (0.0, 0.0) }
                    }
                    SpawnAt::Position(x, y) => (*x, *y),
                    SpawnAt::Random => {
                        let seed = world.entity_count() as f64;
                        let x = ((seed * 7919.0) % world_bounds.0).abs();
                        let y = ((seed * 6271.0) % world_bounds.1).abs();
                        (x, y)
                    }
                };
                queue.spawns.push((template_name.clone(), x, y));
            }
            Action::StartTimer { name, delay, interval, max_fires } => {
                queue.timer_starts.push((name.clone(), *delay, *interval, *max_fires));
            }
            Action::CancelTimer { name } => {
                queue.timer_cancels.push(name.clone());
            }
            Action::Log { message } => {
                queue.logs.push(message.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::*;
    use crate::behavior::*;

    fn setup_world_with_lifetime_entity(seconds: f64) -> (World, Entity) {
        let mut world = World::new();
        let entity = world.spawn();
        world.transforms.insert(entity, Transform { x: 10.0, y: 20.0, ..Default::default() });
        world.lifetimes.insert(entity, Lifetime::new(seconds));
        (world, entity)
    }

    fn empty_context() -> (SpawnQueue, EventQueue, GlobalGameState, TimerQueue, TemplateRegistry, BehaviorRules, WorldConfig) {
        (
            SpawnQueue::default(),
            EventQueue::default(),
            GlobalGameState::new(),
            TimerQueue::new(),
            TemplateRegistry::new(),
            BehaviorRules::new(),
            WorldConfig::default(),
        )
    }

    // --- Lifetime auto-despawn tests ---

    #[test]
    fn lifetime_entity_survives_before_expiry() {
        let (mut world, entity) = setup_world_with_lifetime_entity(2.0);
        let (mut sq, events, mut state, mut timers, templates, mut rules, config) = empty_context();
        run(&mut world, &mut sq, &config, &events, &mut state, &mut timers, &templates, &mut rules, 1.0);
        assert!(world.is_alive(entity));
    }

    #[test]
    fn lifetime_entity_despawns_on_expiry() {
        let (mut world, entity) = setup_world_with_lifetime_entity(1.0);
        let (mut sq, events, mut state, mut timers, templates, mut rules, config) = empty_context();
        run(&mut world, &mut sq, &config, &events, &mut state, &mut timers, &templates, &mut rules, 1.0);
        assert!(!world.is_alive(entity));
    }

    #[test]
    fn lifetime_entity_despawns_after_accumulation() {
        let (mut world, entity) = setup_world_with_lifetime_entity(2.0);
        let (mut sq, events, mut state, mut timers, templates, mut rules, config) = empty_context();
        run(&mut world, &mut sq, &config, &events, &mut state, &mut timers, &templates, &mut rules, 1.0);
        assert!(world.is_alive(entity));
        run(&mut world, &mut sq, &config, &events, &mut state, &mut timers, &templates, &mut rules, 1.0);
        assert!(!world.is_alive(entity));
    }

    // --- Behavior rule: collision despawn ---

    #[test]
    fn collision_rule_despawns_both_and_adds_score() {
        let mut world = World::new();
        let bullet = world.spawn();
        world.transforms.insert(bullet, Transform { x: 100.0, y: 100.0, ..Default::default() });
        world.tags.insert(bullet, Tags::new(&["bullet"]));

        let asteroid = world.spawn();
        world.transforms.insert(asteroid, Transform { x: 100.0, y: 100.0, ..Default::default() });
        world.tags.insert(asteroid, Tags::new(&["asteroid"]));

        let mut events = EventQueue::default();
        events.push(EventKind::Collision {
            entity_a: bullet,
            entity_b: asteroid,
            normal: (0.0, 1.0),
            contact: (100.0, 100.0),
        });

        let mut sq = SpawnQueue::default();
        let mut state = GlobalGameState::new();
        let mut timers = TimerQueue::new();
        let templates = TemplateRegistry::new();
        let config = WorldConfig::default();
        let mut rules = BehaviorRules::new();
        rules.add(BehaviorRule::new(
            "bullet_hits_asteroid",
            Condition::Collision { tag_a: "bullet".into(), tag_b: "asteroid".into() },
            vec![
                Action::Despawn { entity_ref: EntityRef::Both },
                Action::AddState { key: "score".into(), delta: 10.0 },
            ],
        ));

        run(&mut world, &mut sq, &config, &events, &mut state, &mut timers, &templates, &mut rules, 0.016);

        assert!(!world.is_alive(bullet));
        assert!(!world.is_alive(asteroid));
        assert_eq!(state.get_f64("score"), Some(10.0));
    }

    // --- Behavior rule: timer-triggered spawn ---

    #[test]
    fn timer_rule_triggers_template_spawn() {
        let mut world = World::new();
        let events = EventQueue::default();
        let mut sq = SpawnQueue::default();
        let mut state = GlobalGameState::new();
        let config = WorldConfig::default();

        // Pre-tick the timer so it fires
        let mut timers = TimerQueue::new();
        timers.add(crate::timers::Timer::one_shot("spawn_wave", 0.0));

        let mut templates = TemplateRegistry::new();
        let mut tmpl = crate::templates::EntityTemplate::new("enemy");
        tmpl.transform = Some(Transform::default());
        tmpl.tags = Some(Tags::new(&["enemy"]));
        templates.register(tmpl);

        let mut rules = BehaviorRules::new();
        rules.add(BehaviorRule::new(
            "spawn_enemy_wave",
            Condition::TimerFired { timer_name: "spawn_wave".into() },
            vec![
                Action::SpawnTemplate {
                    template_name: "enemy".into(),
                    at: SpawnAt::Position(480.0, 50.0),
                },
            ],
        ));

        // Timer will be ticked inside run(), firing immediately
        run(&mut world, &mut sq, &config, &events, &mut state, &mut timers, &templates, &mut rules, 0.016);

        assert_eq!(world.entity_count(), 1);
    }

    // --- Behavior rule: state check ---

    #[test]
    fn state_check_rule_fires_when_condition_met() {
        let mut world = World::new();
        let events = EventQueue::default();
        let mut sq = SpawnQueue::default();
        let mut state = GlobalGameState::new();
        state.set_f64("lives", 0.0);
        let mut timers = TimerQueue::new();
        let templates = TemplateRegistry::new();
        let config = WorldConfig::default();

        let mut rules = BehaviorRules::new();
        rules.add(BehaviorRule::one_shot(
            "game_over",
            Condition::StateCheck { key: "lives".into(), op: crate::behavior::CompareOp::Lte, value: 0.0 },
            vec![
                Action::SetFlag { key: "game_over".into(), value: true },
                Action::Log { message: "Game Over!".into() },
            ],
        ));

        run(&mut world, &mut sq, &config, &events, &mut state, &mut timers, &templates, &mut rules, 0.016);

        assert_eq!(state.get_bool("game_over"), Some(true));
        assert!(!rules.rules[0].enabled);
    }

    // --- One-shot rule only fires once ---

    #[test]
    fn one_shot_rule_disables_after_firing() {
        let mut world = World::new();
        let events = EventQueue::default();
        let mut sq = SpawnQueue::default();
        let mut state = GlobalGameState::new();
        let mut timers = TimerQueue::new();
        let templates = TemplateRegistry::new();
        let config = WorldConfig::default();

        let mut rules = BehaviorRules::new();
        rules.add(BehaviorRule::one_shot(
            "init",
            Condition::Always,
            vec![Action::SetState { key: "initialized".into(), value: 1.0 }],
        ));

        run(&mut world, &mut sq, &config, &events, &mut state, &mut timers, &templates, &mut rules, 0.016);
        assert_eq!(state.get_f64("initialized"), Some(1.0));
        assert!(!rules.rules[0].enabled);

        state.set_f64("initialized", 99.0);
        run(&mut world, &mut sq, &config, &events, &mut state, &mut timers, &templates, &mut rules, 0.016);
        assert_eq!(state.get_f64("initialized"), Some(99.0));
    }

    // --- Out-of-bounds despawn ---

    #[test]
    fn oob_entity_is_despawned() {
        let mut world = World::new();
        let entity = world.spawn();
        world.transforms.insert(entity, Transform { x: -500.0, y: 0.0, ..Default::default() });
        world.rigidbodies.insert(entity, RigidBody { is_static: false, ..Default::default() });

        let (mut sq, events, mut state, mut timers, templates, mut rules, config) = empty_context();
        run(&mut world, &mut sq, &config, &events, &mut state, &mut timers, &templates, &mut rules, 0.016);
        assert!(!world.is_alive(entity));
    }

    #[test]
    fn static_entity_not_despawned_when_oob() {
        let mut world = World::new();
        let entity = world.spawn();
        world.transforms.insert(entity, Transform { x: -500.0, y: 0.0, ..Default::default() });
        world.rigidbodies.insert(entity, RigidBody { is_static: true, ..Default::default() });

        let (mut sq, events, mut state, mut timers, templates, mut rules, config) = empty_context();
        run(&mut world, &mut sq, &config, &events, &mut state, &mut timers, &templates, &mut rules, 0.016);
        assert!(world.is_alive(entity));
    }
}
