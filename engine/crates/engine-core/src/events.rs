use crate::ecs::Entity;
use crate::physics::math::Vec2;

#[derive(Clone, Debug)]
pub struct Event {
    pub kind: EventKind,
}

#[derive(Clone, Debug)]
pub enum EventKind {
    Collision {
        entity_a: Entity,
        entity_b: Entity,
        normal: Vec2,
        contact: Vec2,
    },
    TriggerEnter {
        trigger: Entity,
        visitor: Entity,
    },
    Interaction {
        actor: Entity,
        target: Entity,
    },
}

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

impl EventQueue {
    pub fn push(&mut self, kind: EventKind) {
        self.events.push(Event { kind });
    }
    pub fn clear(&mut self) {
        self.events.clear();
    }
    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collision_event() -> EventKind {
        EventKind::Collision {
            entity_a: Entity(1),
            entity_b: Entity(2),
            normal: (0.0, 1.0),
            contact: (5.0, 10.0),
        }
    }

    fn trigger_event() -> EventKind {
        EventKind::TriggerEnter {
            trigger: Entity(10),
            visitor: Entity(20),
        }
    }

    fn interaction_event() -> EventKind {
        EventKind::Interaction {
            actor: Entity(100),
            target: Entity(200),
        }
    }

    // ─── Default ────────────────────────────────────────────────────

    #[test]
    fn default_creates_empty_queue() {
        let queue = EventQueue::default();
        assert_eq!(queue.iter().count(), 0);
        assert!(queue.events.is_empty());
    }

    // ─── push and iter ──────────────────────────────────────────────

    #[test]
    fn push_single_event_and_iter() {
        let mut queue = EventQueue::default();
        queue.push(collision_event());
        let events: Vec<&Event> = queue.iter().collect();
        assert_eq!(events.len(), 1);
        match &events[0].kind {
            EventKind::Collision { entity_a, entity_b, .. } => {
                assert_eq!(*entity_a, Entity(1));
                assert_eq!(*entity_b, Entity(2));
            }
            _ => panic!("Expected Collision event"),
        }
    }

    #[test]
    fn push_multiple_events_preserves_order() {
        let mut queue = EventQueue::default();
        queue.push(collision_event());
        queue.push(trigger_event());
        queue.push(interaction_event());

        let events: Vec<&Event> = queue.iter().collect();
        assert_eq!(events.len(), 3);

        assert!(matches!(&events[0].kind, EventKind::Collision { .. }));
        assert!(matches!(&events[1].kind, EventKind::TriggerEnter { .. }));
        assert!(matches!(&events[2].kind, EventKind::Interaction { .. }));
    }

    #[test]
    fn push_same_kind_multiple_times() {
        let mut queue = EventQueue::default();
        queue.push(collision_event());
        queue.push(collision_event());
        queue.push(collision_event());
        assert_eq!(queue.iter().count(), 3);
    }

    // ─── clear ──────────────────────────────────────────────────────

    #[test]
    fn clear_empties_queue() {
        let mut queue = EventQueue::default();
        queue.push(collision_event());
        queue.push(trigger_event());
        queue.push(interaction_event());
        queue.clear();
        assert_eq!(queue.iter().count(), 0);
        assert!(queue.events.is_empty());
    }

    #[test]
    fn clear_on_empty_queue_is_safe() {
        let mut queue = EventQueue::default();
        queue.clear();
        assert_eq!(queue.iter().count(), 0);
    }

    #[test]
    fn push_after_clear_works() {
        let mut queue = EventQueue::default();
        queue.push(collision_event());
        queue.clear();
        queue.push(trigger_event());
        let events: Vec<&Event> = queue.iter().collect();
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0].kind, EventKind::TriggerEnter { .. }));
    }

    // ─── iter on empty ──────────────────────────────────────────────

    #[test]
    fn iter_on_empty_returns_nothing() {
        let queue = EventQueue::default();
        let events: Vec<&Event> = queue.iter().collect();
        assert!(events.is_empty());
    }

    // ─── Event fields are accessible ────────────────────────────────

    #[test]
    fn collision_event_fields_are_correct() {
        let mut queue = EventQueue::default();
        queue.push(EventKind::Collision {
            entity_a: Entity(5),
            entity_b: Entity(6),
            normal: (1.0, 0.0),
            contact: (3.0, 4.0),
        });
        let event = queue.iter().next().unwrap();
        match &event.kind {
            EventKind::Collision { entity_a, entity_b, normal, contact } => {
                assert_eq!(*entity_a, Entity(5));
                assert_eq!(*entity_b, Entity(6));
                assert_eq!(*normal, (1.0, 0.0));
                assert_eq!(*contact, (3.0, 4.0));
            }
            _ => panic!("Expected Collision"),
        }
    }

    #[test]
    fn trigger_event_fields_are_correct() {
        let mut queue = EventQueue::default();
        queue.push(EventKind::TriggerEnter {
            trigger: Entity(10),
            visitor: Entity(20),
        });
        let event = queue.iter().next().unwrap();
        match &event.kind {
            EventKind::TriggerEnter { trigger, visitor } => {
                assert_eq!(*trigger, Entity(10));
                assert_eq!(*visitor, Entity(20));
            }
            _ => panic!("Expected TriggerEnter"),
        }
    }

    #[test]
    fn interaction_event_fields_are_correct() {
        let mut queue = EventQueue::default();
        queue.push(EventKind::Interaction {
            actor: Entity(100),
            target: Entity(200),
        });
        let event = queue.iter().next().unwrap();
        match &event.kind {
            EventKind::Interaction { actor, target } => {
                assert_eq!(*actor, Entity(100));
                assert_eq!(*target, Entity(200));
            }
            _ => panic!("Expected Interaction"),
        }
    }

    // ─── Event is Clone + Debug ─────────────────────────────────────

    #[test]
    fn event_is_cloneable() {
        let mut queue = EventQueue::default();
        queue.push(collision_event());
        let event = queue.iter().next().unwrap();
        let cloned = event.clone();
        // Verify the clone has the same shape
        assert!(matches!(&cloned.kind, EventKind::Collision { .. }));
    }

    #[test]
    fn event_is_debug_printable() {
        let mut queue = EventQueue::default();
        queue.push(trigger_event());
        let event = queue.iter().next().unwrap();
        let dbg = format!("{:?}", event);
        assert!(dbg.contains("TriggerEnter"));
    }
}
