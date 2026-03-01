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
