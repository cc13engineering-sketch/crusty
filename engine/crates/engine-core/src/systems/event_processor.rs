/// SYSTEM: event_processor
/// READS: EventQueue, Tags, World.names
/// WRITES: (various — depends on event handlers)
/// ORDER: runs once per frame after all physics steps

use std::collections::HashSet;
use crate::ecs::{World, Entity};
use crate::events::{EventQueue, EventKind};

pub fn run(world: &mut World, events: &EventQueue) {
    let mut seen_triggers: HashSet<(Entity, Entity)> = HashSet::new();

    for event in events.iter() {
        match &event.kind {
            EventKind::TriggerEnter { trigger, visitor } => {
                if !seen_triggers.insert((*trigger, *visitor)) {
                    continue;
                }
                let trigger_is_goal = world.tags.get(*trigger)
                    .map_or(false, |t| t.has("goal"));
                let visitor_is_player = world.tags.get(*visitor)
                    .map_or(false, |t| t.has("player"));
                let trigger_is_encounter = world.tags.get(*trigger)
                    .map_or(false, |t| t.has("encounter_zone"));
                let trigger_is_pickup = world.tags.get(*trigger)
                    .map_or(false, |t| t.has("pickup"));
                let trigger_is_transition = world.tags.get(*trigger)
                    .map_or(false, |t| t.has("transition"));

                if trigger_is_goal && visitor_is_player {
                    crate::log::log("🎯 Goal reached!");
                }
                if trigger_is_encounter && visitor_is_player {
                    crate::log::log("🌿 Wild encounter!");
                }
                if trigger_is_pickup && visitor_is_player {
                    let name = world.names.get_name(*trigger).unwrap_or("item");
                    crate::log::log(&format!("✨ Picked up {}!", name));
                    // Make pickup invisible
                    if let Some(r) = world.renderables.get_mut(*trigger) {
                        r.visible = false;
                    }
                }
                if trigger_is_transition && visitor_is_player {
                    crate::log::log("🚪 Map transition!");
                }
            }
            EventKind::Collision { entity_a, entity_b, .. } => {
                // Check for trainer collision
                let a_is_trainer = world.tags.get(*entity_a)
                    .map_or(false, |t| t.has("trainer"));
                let b_is_trainer = world.tags.get(*entity_b)
                    .map_or(false, |t| t.has("trainer"));
                let a_is_player = world.tags.get(*entity_a)
                    .map_or(false, |t| t.has("player"));
                let b_is_player = world.tags.get(*entity_b)
                    .map_or(false, |t| t.has("player"));

                if (a_is_trainer && b_is_player) || (b_is_trainer && a_is_player) {
                    crate::log::log("⚔️ Trainer encounter!");
                }
            }
            EventKind::Interaction { .. } => {
                // Future: NPC dialogue
            }
        }
    }
}
