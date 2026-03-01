/// SYSTEM: signal
/// READS: SignalEmitter, SignalReceiver
/// WRITES: SignalReceiver (triggered, prev_triggered)
/// ORDER: runs early in tick, after lifecycle

use std::collections::HashSet;
use crate::ecs::World;

pub fn run(world: &mut World) {
    // Phase 1: Collect all active signal channels
    let active_channels: HashSet<String> = world.signal_emitters.iter()
        .filter(|(_, emitter)| emitter.active)
        .map(|(_, emitter)| emitter.channel.clone())
        .collect();

    // Phase 2: Update all receivers
    for (_, receiver) in world.signal_receivers.iter_mut() {
        receiver.prev_triggered = receiver.triggered;

        if receiver.channels.is_empty() {
            receiver.triggered = false;
            continue;
        }

        if receiver.require_all {
            // AND logic: all channels must be active
            receiver.triggered = receiver.channels.iter()
                .all(|ch| active_channels.contains(ch));
        } else {
            // OR logic: any channel must be active
            receiver.triggered = receiver.channels.iter()
                .any(|ch| active_channels.contains(ch));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::World;
    use crate::components::signal::{SignalEmitter, SignalReceiver};

    #[test]
    fn emitter_activates_receiver() {
        let mut world = World::new();

        let switch = world.spawn();
        world.signal_emitters.insert(switch, SignalEmitter::new("door_channel", true));

        let door = world.spawn();
        world.signal_receivers.insert(door, SignalReceiver::new(vec!["door_channel".into()], true));

        run(&mut world);
        let recv = world.signal_receivers.get(door).unwrap();
        assert!(recv.triggered);
    }

    #[test]
    fn inactive_emitter_does_not_trigger() {
        let mut world = World::new();

        let switch = world.spawn();
        world.signal_emitters.insert(switch, SignalEmitter::new("ch", false));

        let door = world.spawn();
        world.signal_receivers.insert(door, SignalReceiver::new(vec!["ch".into()], true));

        run(&mut world);
        let recv = world.signal_receivers.get(door).unwrap();
        assert!(!recv.triggered);
    }

    #[test]
    fn and_logic_requires_all_channels() {
        let mut world = World::new();

        let s1 = world.spawn();
        world.signal_emitters.insert(s1, SignalEmitter::new("a", true));
        let s2 = world.spawn();
        world.signal_emitters.insert(s2, SignalEmitter::new("b", false)); // inactive

        let receiver = world.spawn();
        world.signal_receivers.insert(receiver,
            SignalReceiver::new(vec!["a".into(), "b".into()], true));

        run(&mut world);
        assert!(!world.signal_receivers.get(receiver).unwrap().triggered);
    }

    #[test]
    fn or_logic_any_channel_suffices() {
        let mut world = World::new();

        let s1 = world.spawn();
        world.signal_emitters.insert(s1, SignalEmitter::new("a", true));
        let s2 = world.spawn();
        world.signal_emitters.insert(s2, SignalEmitter::new("b", false));

        let receiver = world.spawn();
        world.signal_receivers.insert(receiver,
            SignalReceiver::new(vec!["a".into(), "b".into()], false));

        run(&mut world);
        assert!(world.signal_receivers.get(receiver).unwrap().triggered);
    }

    #[test]
    fn just_triggered_detects_edge() {
        let mut world = World::new();

        let switch = world.spawn();
        world.signal_emitters.insert(switch, SignalEmitter::new("ch", true));

        let door = world.spawn();
        world.signal_receivers.insert(door, SignalReceiver::new(vec!["ch".into()], true));

        run(&mut world);
        assert!(world.signal_receivers.get(door).unwrap().just_triggered());

        run(&mut world);
        assert!(!world.signal_receivers.get(door).unwrap().just_triggered());
    }

    #[test]
    fn just_released_detects_deactivation() {
        let mut world = World::new();

        let switch = world.spawn();
        world.signal_emitters.insert(switch, SignalEmitter::new("ch", true));

        let door = world.spawn();
        world.signal_receivers.insert(door, SignalReceiver::new(vec!["ch".into()], true));

        run(&mut world); // triggered
        assert!(!world.signal_receivers.get(door).unwrap().just_released());

        // Deactivate
        world.signal_emitters.get_mut(switch).unwrap().active = false;
        run(&mut world);
        assert!(world.signal_receivers.get(door).unwrap().just_released());
    }

    #[test]
    fn no_emitters_means_not_triggered() {
        let mut world = World::new();

        let door = world.spawn();
        world.signal_receivers.insert(door, SignalReceiver::new(vec!["ch".into()], true));

        run(&mut world);
        assert!(!world.signal_receivers.get(door).unwrap().triggered);
    }

    #[test]
    fn empty_channels_not_triggered() {
        let mut world = World::new();

        let door = world.spawn();
        world.signal_receivers.insert(door, SignalReceiver::new(vec![], true));

        run(&mut world);
        assert!(!world.signal_receivers.get(door).unwrap().triggered);
    }

    #[test]
    fn multiple_emitters_same_channel() {
        let mut world = World::new();

        let s1 = world.spawn();
        world.signal_emitters.insert(s1, SignalEmitter::new("ch", true));
        let s2 = world.spawn();
        world.signal_emitters.insert(s2, SignalEmitter::new("ch", true));

        let door = world.spawn();
        world.signal_receivers.insert(door, SignalReceiver::new(vec!["ch".into()], true));

        run(&mut world);
        assert!(world.signal_receivers.get(door).unwrap().triggered);
    }
}
