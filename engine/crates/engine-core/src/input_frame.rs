use serde::{Serialize, Deserialize};

/// A single frame of input, serializable for replay and policy-driven simulation.
///
/// This is the canonical input representation for the engine. Headless runs,
/// replays, and policies all produce `InputFrame`s. The engine applies them
/// via [`Engine::apply_input`] before each simulation step.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InputFrame {
    /// Keys pressed this frame (e.g., "KeyA", "Space").
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys_pressed: Vec<String>,
    /// Keys released this frame.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys_released: Vec<String>,
    /// Keys held down (carried from previous frames).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys_held: Vec<String>,
    /// Current pointer/mouse position, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pointer: Option<(f64, f64)>,
    /// Pointer pressed this frame at (x, y).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pointer_down: Option<(f64, f64)>,
    /// Pointer released this frame at (x, y).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pointer_up: Option<(f64, f64)>,
}

impl InputFrame {
    /// Returns true if this frame has no input at all.
    pub fn is_empty(&self) -> bool {
        self.keys_pressed.is_empty()
            && self.keys_released.is_empty()
            && self.keys_held.is_empty()
            && self.pointer.is_none()
            && self.pointer_down.is_none()
            && self.pointer_up.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_empty() {
        let frame = InputFrame::default();
        assert!(frame.is_empty());
    }

    #[test]
    fn with_key_press_not_empty() {
        let frame = InputFrame {
            keys_pressed: vec!["Space".into()],
            ..Default::default()
        };
        assert!(!frame.is_empty());
    }

    #[test]
    fn serde_roundtrip() {
        let frame = InputFrame {
            keys_pressed: vec!["KeyA".into()],
            keys_held: vec!["KeyA".into(), "ShiftLeft".into()],
            pointer: Some((100.0, 200.0)),
            pointer_down: Some((100.0, 200.0)),
            ..Default::default()
        };
        let json = serde_json::to_string(&frame).unwrap();
        let back: InputFrame = serde_json::from_str(&json).unwrap();
        assert_eq!(back.keys_pressed, frame.keys_pressed);
        assert_eq!(back.keys_held, frame.keys_held);
        assert_eq!(back.pointer, frame.pointer);
        assert_eq!(back.pointer_down, frame.pointer_down);
        assert!(back.keys_released.is_empty());
        assert!(back.pointer_up.is_none());
    }

    #[test]
    fn empty_frame_compact_json() {
        let frame = InputFrame::default();
        let json = serde_json::to_string(&frame).unwrap();
        // Empty fields should be skipped
        assert_eq!(json, "{}");
    }
}
