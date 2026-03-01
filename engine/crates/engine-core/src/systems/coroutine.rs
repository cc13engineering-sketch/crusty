/// SYSTEM: coroutine
/// READS: Coroutine, TimeScale, GameState, SignalEmitter
/// WRITES: Coroutine (advance steps), GameState (set/add), SpawnQueue
/// ORDER: runs after state_machine, before physics

use crate::ecs::World;
use crate::components::coroutine::CoroutineStep;

pub fn run(world: &mut World, dt: f64) {
    let entities: Vec<_> = world.coroutines.iter()
        .map(|(e, _)| e)
        .collect();

    // Collect active signal channels
    let active_channels: std::collections::HashSet<String> = world.signal_emitters.iter()
        .filter(|(_, em)| em.active)
        .map(|(_, em)| em.channel.clone())
        .collect();

    // Collect spawn commands to defer
    let mut spawns = Vec::new();
    let mut completed = Vec::new();

    for entity in entities {
        let effective_dt = if let Some(ts) = world.time_scales.get(entity) {
            ts.apply(dt)
        } else {
            dt
        };

        if let Some(co) = world.coroutines.get_mut(entity) {
            if co.paused || co.is_done() {
                if co.is_done() {
                    completed.push(entity);
                }
                continue;
            }

            // Process steps — cascade through non-blocking steps in one frame
            loop {
                if co.steps.is_empty() {
                    completed.push(entity);
                    break;
                }

                match co.steps.front().cloned() {
                    Some(CoroutineStep::WaitSeconds(duration)) => {
                        co.wait_timer += effective_dt;
                        if co.wait_timer >= duration {
                            co.wait_timer -= duration;
                            co.steps.pop_front();
                            // Continue to next step
                        } else {
                            break; // Still waiting
                        }
                    }
                    Some(CoroutineStep::WaitSignal(ref channel)) => {
                        if active_channels.contains(channel) {
                            co.steps.pop_front();
                            // Continue to next step
                        } else {
                            break; // Still waiting
                        }
                    }
                    Some(CoroutineStep::WaitUntil { ref key, ref op, value }) => {
                        let current = world.game_states.get(entity)
                            .map(|gs| gs.get(key))
                            .unwrap_or(0.0);
                        if op.evaluate(current, value) {
                            co.steps.pop_front();
                        } else {
                            break; // Still waiting
                        }
                    }
                    Some(CoroutineStep::SetState { ref key, value }) => {
                        let key = key.clone();
                        co.steps.pop_front();
                        // Apply to entity's GameState
                        if let Some(gs) = world.game_states.get_mut(entity) {
                            gs.set(&key, value);
                        } else {
                            let mut gs = crate::components::game_state::GameState::new();
                            gs.set(&key, value);
                            world.game_states.insert(entity, gs);
                        }
                        // Continue cascade
                    }
                    Some(CoroutineStep::AddState { ref key, delta }) => {
                        let key = key.clone();
                        co.steps.pop_front();
                        if let Some(gs) = world.game_states.get_mut(entity) {
                            let current = gs.get(&key);
                            gs.set(&key, current + delta);
                        } else {
                            let mut gs = crate::components::game_state::GameState::new();
                            gs.set(&key, delta);
                            world.game_states.insert(entity, gs);
                        }
                    }
                    Some(CoroutineStep::SpawnTemplate { ref name, x, y }) => {
                        spawns.push((name.clone(), x, y));
                        co.steps.pop_front();
                    }
                    Some(CoroutineStep::Log(ref msg)) => {
                        crate::log::log(&format!("[Coroutine {}] {}", co.label, msg));
                        co.steps.pop_front();
                    }
                    None => {
                        completed.push(entity);
                        break;
                    }
                }
            }
        }
    }

    // Queue spawns
    for (name, x, y) in spawns {
        let mut cmd = crate::spawn_queue::SpawnCommand::at(x, y);
        cmd.name = Some(name);
        world.spawn_queue.spawn(cmd);
    }

    // Remove completed coroutines
    for entity in completed {
        world.coroutines.remove(entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::World;
    use crate::components::coroutine::Coroutine;
    use crate::components::signal::SignalEmitter;

    #[test]
    fn wait_seconds_delays() {
        let mut world = World::new();
        let e = world.spawn();
        world.coroutines.insert(e, Coroutine::new("test")
            .then_wait(1.0)
            .then_log("done"));

        run(&mut world, 0.5);
        assert_eq!(world.coroutines.get(e).unwrap().remaining_steps(), 2);

        run(&mut world, 0.6);
        // Wait completed, log cascaded, coroutine done
        assert!(world.coroutines.get(e).is_none());
    }

    #[test]
    fn set_state_executes_immediately() {
        let mut world = World::new();
        let e = world.spawn();
        world.coroutines.insert(e, Coroutine::new("test")
            .then_set_state("score", 42.0));

        run(&mut world, 0.016);
        assert_eq!(world.game_states.get(e).unwrap().get("score"), 42.0);
        // Coroutine should be done and removed
        assert!(world.coroutines.get(e).is_none());
    }

    #[test]
    fn multiple_non_wait_steps_cascade() {
        let mut world = World::new();
        let e = world.spawn();
        world.coroutines.insert(e, Coroutine::new("test")
            .then_set_state("a", 1.0)
            .then_set_state("b", 2.0)
            .then_add_state("a", 10.0));

        run(&mut world, 0.016);
        let gs = world.game_states.get(e).unwrap();
        assert_eq!(gs.get("a"), 11.0);
        assert_eq!(gs.get("b"), 2.0);
        assert!(world.coroutines.get(e).is_none());
    }

    #[test]
    fn wait_signal_blocks_until_active() {
        let mut world = World::new();
        let e = world.spawn();
        world.coroutines.insert(e, Coroutine::new("test")
            .then_wait_signal("door_open")
            .then_set_state("entered", 1.0));

        run(&mut world, 0.016);
        assert!(world.coroutines.get(e).is_some()); // still waiting

        // Activate signal
        let sig = world.spawn();
        world.signal_emitters.insert(sig, SignalEmitter::new("door_open", true));
        run(&mut world, 0.016);
        assert!(world.coroutines.get(e).is_none()); // completed
        assert_eq!(world.game_states.get(e).unwrap().get("entered"), 1.0);
    }

    #[test]
    fn paused_coroutine_doesnt_advance() {
        let mut world = World::new();
        let e = world.spawn();
        let mut co = Coroutine::new("test").then_set_state("x", 1.0);
        co.paused = true;
        world.coroutines.insert(e, co);

        run(&mut world, 0.016);
        assert!(world.coroutines.get(e).is_some());
        assert!(world.game_states.get(e).is_none());
    }

    #[test]
    fn spawn_template_queues_spawn() {
        let mut world = World::new();
        let e = world.spawn();
        world.coroutines.insert(e, Coroutine::new("test")
            .then_spawn("bullet", 10.0, 20.0));

        run(&mut world, 0.016);
        assert_eq!(world.spawn_queue.spawns.len(), 1);
        assert!((world.spawn_queue.spawns[0].transform.x - 10.0).abs() < 1e-10);
    }

    #[test]
    fn add_state_creates_if_missing() {
        let mut world = World::new();
        let e = world.spawn();
        world.coroutines.insert(e, Coroutine::new("test")
            .then_add_state("count", 5.0));

        run(&mut world, 0.016);
        assert_eq!(world.game_states.get(e).unwrap().get("count"), 5.0);
    }

    #[test]
    fn completed_coroutine_removed() {
        let mut world = World::new();
        let e = world.spawn();
        world.coroutines.insert(e, Coroutine::new("test")
            .then_log("bye"));

        run(&mut world, 0.016);
        assert!(world.coroutines.get(e).is_none());
    }

    #[test]
    fn wait_then_action_pattern() {
        let mut world = World::new();
        let e = world.spawn();
        world.coroutines.insert(e, Coroutine::new("test")
            .then_wait(0.5)
            .then_set_state("phase", 1.0)
            .then_wait(0.5)
            .then_set_state("phase", 2.0));

        run(&mut world, 0.3);
        assert!(world.game_states.get(e).is_none()); // still waiting

        run(&mut world, 0.3); // total 0.6 > 0.5, first wait done
        assert_eq!(world.game_states.get(e).unwrap().get("phase"), 1.0);

        run(&mut world, 0.6); // second wait done
        assert_eq!(world.game_states.get(e).unwrap().get("phase"), 2.0);
    }
}
