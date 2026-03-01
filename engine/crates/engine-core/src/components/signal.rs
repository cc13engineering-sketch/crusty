use super::SchemaInfo;

/// A signal emitter. When `active` is true, the named channel is considered "on."
#[derive(Clone, Debug)]
pub struct SignalEmitter {
    pub channel: String,
    pub active: bool,
}

impl SignalEmitter {
    pub fn new(channel: &str, active: bool) -> Self {
        Self { channel: channel.to_string(), active }
    }
}

impl SchemaInfo for SignalEmitter {
    fn schema_name() -> &'static str { "SignalEmitter" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "channel": { "type": "string" },
                "active": { "type": "bool", "default": false }
            }
        })
    }
}

/// A signal receiver. Watches one or more channels and updates its triggered state.
#[derive(Clone, Debug)]
pub struct SignalReceiver {
    pub channels: Vec<String>,
    pub require_all: bool,
    pub triggered: bool,
    pub prev_triggered: bool,
}

impl SignalReceiver {
    pub fn new(channels: Vec<String>, require_all: bool) -> Self {
        Self {
            channels,
            require_all,
            triggered: false,
            prev_triggered: false,
        }
    }

    /// Returns true on the frame when triggered transitions from false to true.
    pub fn just_triggered(&self) -> bool {
        self.triggered && !self.prev_triggered
    }

    /// Returns true on the frame when triggered transitions from true to false.
    pub fn just_released(&self) -> bool {
        !self.triggered && self.prev_triggered
    }
}

impl SchemaInfo for SignalReceiver {
    fn schema_name() -> &'static str { "SignalReceiver" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "channels": { "type": "Vec<String>" },
                "require_all": { "type": "bool", "default": true },
                "triggered": { "type": "bool", "read_only": true },
                "prev_triggered": { "type": "bool", "read_only": true }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- SignalEmitter ---

    #[test]
    fn emitter_new() {
        let e = SignalEmitter::new("door_switch", false);
        assert_eq!(e.channel, "door_switch");
        assert!(!e.active);
    }

    #[test]
    fn emitter_active() {
        let e = SignalEmitter::new("lever", true);
        assert!(e.active);
    }

    #[test]
    fn emitter_clone() {
        let e = SignalEmitter::new("test", true);
        let c = e.clone();
        assert_eq!(c.channel, "test");
        assert!(c.active);
    }

    #[test]
    fn emitter_debug() {
        let e = SignalEmitter::new("ch1", false);
        let d = format!("{:?}", e);
        assert!(d.contains("ch1"));
    }

    #[test]
    fn emitter_schema() {
        assert_eq!(SignalEmitter::schema_name(), "SignalEmitter");
        let s = SignalEmitter::schema();
        assert!(s.get("fields").is_some());
    }

    // --- SignalReceiver ---

    #[test]
    fn receiver_new() {
        let r = SignalReceiver::new(vec!["a".into(), "b".into()], true);
        assert_eq!(r.channels.len(), 2);
        assert!(r.require_all);
        assert!(!r.triggered);
        assert!(!r.prev_triggered);
    }

    #[test]
    fn receiver_just_triggered_false_initially() {
        let r = SignalReceiver::new(vec!["a".into()], true);
        assert!(!r.just_triggered());
    }

    #[test]
    fn receiver_just_triggered_true() {
        let mut r = SignalReceiver::new(vec!["a".into()], true);
        r.prev_triggered = false;
        r.triggered = true;
        assert!(r.just_triggered());
    }

    #[test]
    fn receiver_just_triggered_false_when_sustained() {
        let mut r = SignalReceiver::new(vec!["a".into()], true);
        r.prev_triggered = true;
        r.triggered = true;
        assert!(!r.just_triggered());
    }

    #[test]
    fn receiver_just_released() {
        let mut r = SignalReceiver::new(vec!["a".into()], true);
        r.prev_triggered = true;
        r.triggered = false;
        assert!(r.just_released());
    }

    #[test]
    fn receiver_just_released_false_when_sustained_off() {
        let mut r = SignalReceiver::new(vec!["a".into()], true);
        r.prev_triggered = false;
        r.triggered = false;
        assert!(!r.just_released());
    }

    #[test]
    fn receiver_clone() {
        let r = SignalReceiver::new(vec!["x".into()], false);
        let c = r.clone();
        assert_eq!(c.channels, vec!["x".to_string()]);
        assert!(!c.require_all);
    }

    #[test]
    fn receiver_debug() {
        let r = SignalReceiver::new(vec!["sig".into()], true);
        let d = format!("{:?}", r);
        assert!(d.contains("sig"));
    }

    #[test]
    fn receiver_schema() {
        assert_eq!(SignalReceiver::schema_name(), "SignalReceiver");
        let s = SignalReceiver::schema();
        assert!(s.get("fields").is_some());
    }
}
