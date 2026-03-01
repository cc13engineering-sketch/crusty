/// General-purpose event bus for decoupled inter-system communication.
///
/// Events are typed by channel name and can carry an optional payload.
/// Published events persist for one frame and are cleared at the end of each tick.
/// Systems can subscribe by checking for events on specific channels.

use std::collections::HashMap;
use crate::ecs::Entity;

/// A single event on the bus.
#[derive(Clone, Debug)]
pub struct BusEvent {
    /// The channel/topic name.
    pub channel: String,
    /// The source entity that emitted the event (if any).
    pub source: Option<Entity>,
    /// The target entity (if any).
    pub target: Option<Entity>,
    /// Key-value payload data.
    pub data: HashMap<String, EventValue>,
}

/// Event payload value types.
#[derive(Clone, Debug, PartialEq)]
pub enum EventValue {
    Float(f64),
    Int(i64),
    Text(String),
    Bool(bool),
}

impl EventValue {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            EventValue::Float(v) => Some(*v),
            EventValue::Int(v) => Some(*v as f64),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            EventValue::Int(v) => Some(*v),
            EventValue::Float(v) => Some(*v as i64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            EventValue::Text(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            EventValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

impl BusEvent {
    /// Create a simple event with no payload.
    pub fn new(channel: &str) -> Self {
        Self {
            channel: channel.to_string(),
            source: None,
            target: None,
            data: HashMap::new(),
        }
    }

    /// Create an event with a source entity.
    pub fn from_entity(channel: &str, source: Entity) -> Self {
        Self {
            channel: channel.to_string(),
            source: Some(source),
            target: None,
            data: HashMap::new(),
        }
    }

    /// Builder: set target entity.
    pub fn with_target(mut self, target: Entity) -> Self {
        self.target = Some(target);
        self
    }

    /// Builder: add a float payload value.
    pub fn with_f64(mut self, key: &str, value: f64) -> Self {
        self.data.insert(key.to_string(), EventValue::Float(value));
        self
    }

    /// Builder: add an int payload value.
    pub fn with_i64(mut self, key: &str, value: i64) -> Self {
        self.data.insert(key.to_string(), EventValue::Int(value));
        self
    }

    /// Builder: add a text payload value.
    pub fn with_text(mut self, key: &str, value: &str) -> Self {
        self.data.insert(key.to_string(), EventValue::Text(value.to_string()));
        self
    }

    /// Builder: add a bool payload value.
    pub fn with_bool(mut self, key: &str, value: bool) -> Self {
        self.data.insert(key.to_string(), EventValue::Bool(value));
        self
    }

    /// Get a payload value by key.
    pub fn get(&self, key: &str) -> Option<&EventValue> {
        self.data.get(key)
    }

    /// Get a float from the payload.
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.data.get(key).and_then(|v| v.as_f64())
    }

    /// Get a string from the payload.
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.data.get(key).and_then(|v| v.as_str())
    }
}

/// The event bus: collects events during a frame, provides query APIs.
#[derive(Default, Debug)]
pub struct EventBus {
    events: Vec<BusEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Publish an event to the bus.
    pub fn publish(&mut self, event: BusEvent) {
        self.events.push(event);
    }

    /// Shorthand: publish a simple named event.
    pub fn emit(&mut self, channel: &str) {
        self.publish(BusEvent::new(channel));
    }

    /// Shorthand: publish an event from an entity.
    pub fn emit_from(&mut self, channel: &str, source: Entity) {
        self.publish(BusEvent::from_entity(channel, source));
    }

    /// Get all events on a specific channel.
    pub fn on<'a>(&'a self, channel: &'a str) -> impl Iterator<Item = &'a BusEvent> + 'a {
        self.events.iter().filter(move |e| e.channel == channel)
    }

    /// Check if any event was published on a channel this frame.
    pub fn has(&self, channel: &str) -> bool {
        self.events.iter().any(|e| e.channel == channel)
    }

    /// Get all events from a specific source entity.
    pub fn from_entity(&self, entity: Entity) -> impl Iterator<Item = &BusEvent> + '_ {
        self.events.iter().filter(move |e| e.source == Some(entity))
    }

    /// Get all events targeting a specific entity.
    pub fn targeting(&self, entity: Entity) -> impl Iterator<Item = &BusEvent> + '_ {
        self.events.iter().filter(move |e| e.target == Some(entity))
    }

    /// Get all events this frame.
    pub fn all(&self) -> &[BusEvent] {
        &self.events
    }

    /// Number of events this frame.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Clear all events. Called at the end of each frame.
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Drain all events, consuming them.
    pub fn drain(&mut self) -> Vec<BusEvent> {
        std::mem::take(&mut self.events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_bus_is_empty() {
        let bus = EventBus::new();
        assert!(bus.is_empty());
        assert_eq!(bus.len(), 0);
    }

    #[test]
    fn emit_and_has() {
        let mut bus = EventBus::new();
        bus.emit("player_died");
        assert!(bus.has("player_died"));
        assert!(!bus.has("player_won"));
    }

    #[test]
    fn emit_from_entity() {
        let mut bus = EventBus::new();
        let e = Entity(42);
        bus.emit_from("attack", e);
        let events: Vec<_> = bus.from_entity(e).collect();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].channel, "attack");
        assert_eq!(events[0].source, Some(e));
    }

    #[test]
    fn publish_with_payload() {
        let mut bus = EventBus::new();
        bus.publish(
            BusEvent::new("damage")
                .with_f64("amount", 25.0)
                .with_text("type", "fire")
                .with_bool("critical", true)
        );

        let events: Vec<_> = bus.on("damage").collect();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].get_f64("amount"), Some(25.0));
        assert_eq!(events[0].get_str("type"), Some("fire"));
        assert_eq!(events[0].get("critical").and_then(|v| v.as_bool()), Some(true));
    }

    #[test]
    fn targeting_filters_correctly() {
        let mut bus = EventBus::new();
        let player = Entity(1);
        let enemy = Entity(2);
        bus.publish(BusEvent::new("heal").with_target(player));
        bus.publish(BusEvent::new("damage").with_target(enemy));

        let player_events: Vec<_> = bus.targeting(player).collect();
        assert_eq!(player_events.len(), 1);
        assert_eq!(player_events[0].channel, "heal");
    }

    #[test]
    fn on_filters_by_channel() {
        let mut bus = EventBus::new();
        bus.emit("a");
        bus.emit("b");
        bus.emit("a");

        let a_events: Vec<_> = bus.on("a").collect();
        assert_eq!(a_events.len(), 2);
    }

    #[test]
    fn clear_removes_all() {
        let mut bus = EventBus::new();
        bus.emit("test");
        bus.emit("test2");
        assert_eq!(bus.len(), 2);
        bus.clear();
        assert!(bus.is_empty());
    }

    #[test]
    fn drain_consumes_events() {
        let mut bus = EventBus::new();
        bus.emit("a");
        bus.emit("b");
        let drained = bus.drain();
        assert_eq!(drained.len(), 2);
        assert!(bus.is_empty());
    }

    #[test]
    fn event_value_conversions() {
        assert_eq!(EventValue::Float(3.14).as_f64(), Some(3.14));
        assert_eq!(EventValue::Int(42).as_i64(), Some(42));
        assert_eq!(EventValue::Int(42).as_f64(), Some(42.0));
        assert_eq!(EventValue::Text("hello".to_string()).as_str(), Some("hello"));
        assert_eq!(EventValue::Bool(true).as_bool(), Some(true));
        assert_eq!(EventValue::Float(1.0).as_str(), None);
    }

    #[test]
    fn multiple_events_same_channel() {
        let mut bus = EventBus::new();
        for i in 0..5 {
            bus.publish(BusEvent::new("tick").with_i64("n", i));
        }
        let events: Vec<_> = bus.on("tick").collect();
        assert_eq!(events.len(), 5);
    }

    #[test]
    fn complex_event_builder() {
        let e1 = Entity(10);
        let e2 = Entity(20);
        let event = BusEvent::from_entity("transfer", e1)
            .with_target(e2)
            .with_f64("gold", 100.0)
            .with_text("reason", "trade")
            .with_i64("count", 3)
            .with_bool("confirmed", false);

        assert_eq!(event.source, Some(e1));
        assert_eq!(event.target, Some(e2));
        assert_eq!(event.get_f64("gold"), Some(100.0));
        assert_eq!(event.get_str("reason"), Some("trade"));
        assert_eq!(event.data.len(), 4);
    }
}
