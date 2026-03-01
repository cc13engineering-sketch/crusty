/// SYSTEM: state_machine
/// READS: StateMachine, TimeScale, GameState, SignalEmitter
/// WRITES: StateMachine (state transitions, elapsed time)
/// ORDER: runs after signal, before behavior

use crate::ecs::World;
use crate::components::state_machine::TransitionCondition;

pub fn run(world: &mut World, dt: f64) {
    let entities: Vec<_> = world.state_machines.iter()
        .map(|(e, _)| e)
        .collect();

    // Collect active signal channels for OnSignal checks
    let active_channels: std::collections::HashSet<String> = world.signal_emitters.iter()
        .filter(|(_, em)| em.active)
        .map(|(_, em)| em.channel.clone())
        .collect();

    for entity in entities {
        let effective_dt = if let Some(ts) = world.time_scales.get(entity) {
            ts.apply(dt)
        } else {
            dt
        };

        // Get entity game state for StateCheck conditions
        let entity_state = world.game_states.get(entity);

        if let Some(sm) = world.state_machines.get_mut(entity) {
            // Clear one-frame flags from previous frame
            if !sm.just_entered {
                sm.prev_state = None;
            }

            // Increment elapsed time
            sm.state_elapsed += effective_dt;

            // Check transitions — first matching from current state wins
            let mut new_state: Option<String> = None;
            for transition in &sm.transitions {
                if transition.from != sm.current_state {
                    continue;
                }
                let condition_met = match &transition.condition {
                    TransitionCondition::After(duration) => sm.state_elapsed >= *duration,
                    TransitionCondition::OnSignal(channel) => active_channels.contains(channel),
                    TransitionCondition::StateCheck { key, op, value } => {
                        if let Some(gs) = entity_state {
                            let current = gs.get(key);
                            op.evaluate(current, *value)
                        } else {
                            false
                        }
                    }
                    TransitionCondition::Always => true,
                };
                if condition_met {
                    new_state = Some(transition.to.clone());
                    break;
                }
            }

            // Apply transition
            if let Some(target) = new_state {
                sm.prev_state = Some(std::mem::replace(&mut sm.current_state, target));
                sm.state_elapsed = 0.0;
                sm.just_entered = true;
            } else {
                sm.just_entered = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::World;
    use crate::components::state_machine::{StateMachine, TransitionCondition, CompareOp};
    use crate::components::game_state::GameState;
    use crate::components::signal::SignalEmitter;

    #[test]
    fn after_fires_at_correct_time() {
        let mut world = World::new();
        let e = world.spawn();
        let mut sm = StateMachine::new("idle");
        sm.add_transition("idle", "patrol", TransitionCondition::After(1.0));
        world.state_machines.insert(e, sm);

        run(&mut world, 0.5);
        assert!(world.state_machines.get(e).unwrap().is_in("idle"));

        run(&mut world, 0.6); // total > 1.0
        assert!(world.state_machines.get(e).unwrap().is_in("patrol"));
    }

    #[test]
    fn always_fires_immediately() {
        let mut world = World::new();
        let e = world.spawn();
        let mut sm = StateMachine::new("start");
        sm.add_transition("start", "end", TransitionCondition::Always);
        world.state_machines.insert(e, sm);

        run(&mut world, 0.016);
        assert!(world.state_machines.get(e).unwrap().is_in("end"));
    }

    #[test]
    fn state_check_evaluates() {
        let mut world = World::new();
        let e = world.spawn();
        let mut sm = StateMachine::new("alive");
        sm.add_transition("alive", "dead", TransitionCondition::StateCheck {
            key: "health".to_string(),
            op: CompareOp::Lte,
            value: 0.0,
        });
        let mut gs = GameState::new();
        gs.set("health", 100.0);
        world.game_states.insert(e, gs);
        world.state_machines.insert(e, sm);

        run(&mut world, 0.016);
        assert!(world.state_machines.get(e).unwrap().is_in("alive"));

        world.game_states.get_mut(e).unwrap().set("health", 0.0);
        run(&mut world, 0.016);
        assert!(world.state_machines.get(e).unwrap().is_in("dead"));
    }

    #[test]
    fn on_signal_transitions() {
        let mut world = World::new();
        let e = world.spawn();
        let mut sm = StateMachine::new("waiting");
        sm.add_transition("waiting", "triggered", TransitionCondition::OnSignal("alarm".to_string()));
        world.state_machines.insert(e, sm);

        run(&mut world, 0.016);
        assert!(world.state_machines.get(e).unwrap().is_in("waiting"));

        // Activate signal
        let sig = world.spawn();
        world.signal_emitters.insert(sig, SignalEmitter::new("alarm", true));
        run(&mut world, 0.016);
        assert!(world.state_machines.get(e).unwrap().is_in("triggered"));
    }

    #[test]
    fn just_entered_true_on_first_frame() {
        let mut world = World::new();
        let e = world.spawn();
        let mut sm = StateMachine::new("a");
        sm.add_transition("a", "b", TransitionCondition::Always);
        world.state_machines.insert(e, sm);

        run(&mut world, 0.016);
        let sm = world.state_machines.get(e).unwrap();
        assert!(sm.just_entered);
        assert!(sm.just_entered_state("b"));
    }

    #[test]
    fn just_entered_clears_on_second_frame() {
        let mut world = World::new();
        let e = world.spawn();
        let mut sm = StateMachine::new("a");
        sm.add_transition("a", "b", TransitionCondition::Always);
        world.state_machines.insert(e, sm);

        run(&mut world, 0.016); // transitions to b, just_entered=true
        run(&mut world, 0.016); // no transition, just_entered=false
        assert!(!world.state_machines.get(e).unwrap().just_entered);
    }

    #[test]
    fn prev_state_tracks_last() {
        let mut world = World::new();
        let e = world.spawn();
        let mut sm = StateMachine::new("idle");
        sm.add_transition("idle", "walk", TransitionCondition::Always);
        world.state_machines.insert(e, sm);

        run(&mut world, 0.016);
        let sm = world.state_machines.get(e).unwrap();
        assert_eq!(sm.prev_state.as_deref(), Some("idle"));
    }

    #[test]
    fn first_matching_transition_wins() {
        let mut world = World::new();
        let e = world.spawn();
        let mut sm = StateMachine::new("start");
        sm.add_transition("start", "first", TransitionCondition::Always);
        sm.add_transition("start", "second", TransitionCondition::Always);
        world.state_machines.insert(e, sm);

        run(&mut world, 0.016);
        assert!(world.state_machines.get(e).unwrap().is_in("first"));
    }

    #[test]
    fn respects_time_scale() {
        let mut world = World::new();
        let e = world.spawn();
        let mut sm = StateMachine::new("idle");
        sm.add_transition("idle", "done", TransitionCondition::After(1.0));
        world.state_machines.insert(e, sm);
        world.time_scales.insert(e, crate::components::time_scale::TimeScale { scale: 0.5 });

        run(&mut world, 1.0); // effective dt = 0.5, should not fire
        assert!(world.state_machines.get(e).unwrap().is_in("idle"));

        run(&mut world, 1.0); // effective dt = 0.5, total = 1.0, should fire
        assert!(world.state_machines.get(e).unwrap().is_in("done"));
    }

    #[test]
    fn elapsed_resets_on_transition() {
        let mut world = World::new();
        let e = world.spawn();
        let mut sm = StateMachine::new("a");
        sm.add_transition("a", "b", TransitionCondition::After(0.5));
        world.state_machines.insert(e, sm);

        run(&mut world, 0.6);
        assert_eq!(world.state_machines.get(e).unwrap().state_elapsed, 0.0);
    }
}
