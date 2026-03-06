/// Generic command queue — replaces the ad-hoc queue pattern used by
/// `SoundCommandQueue` and `PersistQueue`.
///
/// Any type implementing `ToJson` can be accumulated in a `CommandQueue`
/// and drained as a JSON array string for consumption by the JS side.

use std::fmt;

// ─── ToJson Trait ───────────────────────────────────────────────────

/// Trait for types that can serialize themselves to a JSON object string.
pub trait ToJson {
    /// Serialize this value to a JSON string (typically a single JSON object).
    fn to_json(&self) -> String;
}

// ─── CommandQueue ───────────────────────────────────────────────────

/// Accumulates commands of type `T` during a frame. The JS side drains the
/// queue each frame via `drain_json()`, which returns a JSON array string.
pub struct CommandQueue<T: ToJson> {
    commands: Vec<T>,
}

impl<T: ToJson> CommandQueue<T> {
    /// Create a new, empty command queue.
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Push a command onto the queue.
    pub fn push(&mut self, cmd: T) {
        self.commands.push(cmd);
    }

    /// Drain all queued commands, returning them as a JSON array string.
    /// The internal queue is emptied after this call.
    pub fn drain_json(&mut self) -> String {
        if self.commands.is_empty() {
            return "[]".to_string();
        }

        let mut json = String::from("[");
        for (i, cmd) in self.commands.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }
            json.push_str(&cmd.to_json());
        }
        json.push(']');
        self.commands.clear();
        json
    }

    /// Number of queued commands.
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Clear without serializing.
    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

impl<T: ToJson + Clone> Clone for CommandQueue<T> {
    fn clone(&self) -> Self {
        Self {
            commands: self.commands.clone(),
        }
    }
}

impl<T: ToJson + fmt::Debug> fmt::Debug for CommandQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandQueue")
            .field("commands", &self.commands)
            .finish()
    }
}

impl<T: ToJson> Default for CommandQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct TestCmd {
        value: String,
    }

    impl ToJson for TestCmd {
        fn to_json(&self) -> String {
            format!("{{\"v\":\"{}\"}}", self.value)
        }
    }

    #[test]
    fn empty_queue_drains_to_empty_array() {
        let mut queue = CommandQueue::<TestCmd>::new();
        assert_eq!(queue.drain_json(), "[]");
    }

    #[test]
    fn push_increments_len() {
        let mut queue = CommandQueue::new();
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());

        queue.push(TestCmd { value: "a".to_string() });
        assert_eq!(queue.len(), 1);
        assert!(!queue.is_empty());

        queue.push(TestCmd { value: "b".to_string() });
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn drain_json_produces_valid_json_and_clears() {
        let mut queue = CommandQueue::new();
        queue.push(TestCmd { value: "first".to_string() });
        queue.push(TestCmd { value: "second".to_string() });

        let json = queue.drain_json();

        // Queue should be empty after drain
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);

        // Should parse as valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("drain_json must produce valid JSON");
        let arr = parsed.as_array().expect("should be a JSON array");
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0]["v"], "first");
        assert_eq!(arr[1]["v"], "second");
    }

    #[test]
    fn clear_discards_commands() {
        let mut queue = CommandQueue::new();
        queue.push(TestCmd { value: "x".to_string() });
        queue.push(TestCmd { value: "y".to_string() });
        assert_eq!(queue.len(), 2);

        queue.clear();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.drain_json(), "[]");
    }

    #[test]
    fn default_is_empty() {
        let queue: CommandQueue<TestCmd> = CommandQueue::default();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn clone_produces_independent_copy() {
        let mut original = CommandQueue::new();
        original.push(TestCmd { value: "cloned".to_string() });

        let mut cloned = original.clone();
        assert_eq!(cloned.len(), 1);

        // Mutating clone should not affect original
        cloned.push(TestCmd { value: "extra".to_string() });
        assert_eq!(original.len(), 1);
        assert_eq!(cloned.len(), 2);
    }

    #[test]
    fn debug_format_includes_commands() {
        let mut queue = CommandQueue::new();
        queue.push(TestCmd { value: "dbg".to_string() });

        let debug_str = format!("{:?}", queue);
        assert!(debug_str.contains("CommandQueue"));
        assert!(debug_str.contains("dbg"));
    }

    #[test]
    fn drain_json_single_command() {
        let mut queue = CommandQueue::new();
        queue.push(TestCmd { value: "solo".to_string() });

        let json = queue.drain_json();
        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("drain_json must produce valid JSON");
        let arr = parsed.as_array().expect("should be a JSON array");
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["v"], "solo");
    }
}
