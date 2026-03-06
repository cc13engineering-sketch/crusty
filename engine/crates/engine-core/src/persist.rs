/// Generic persistence command queue — engine-level key-value storage.
///
/// Rust code pushes `PersistCommand`s into a `PersistQueue` (a type alias
/// for `CommandQueue<PersistCommand>`). Each frame the JS side drains the
/// queue via a WASM binding that returns JSON and writes values to
/// browser localStorage (or any other storage backend).
///
/// This replaces the game-specific `chord_reps::persist` module with a
/// generic, reusable engine primitive.

use crate::command_queue::{CommandQueue, ToJson};

// ─── PersistCommand ─────────────────────────────────────────────────

/// A single persistence command to be consumed by the JS persistence driver.
#[derive(Clone, Debug)]
pub enum PersistCommand {
    /// Store a key-value pair in persistent storage.
    Set { key: String, value: String },
    /// Remove a key from persistent storage.
    Remove { key: String },
    /// Clear all persistent storage.
    Clear,
}

impl ToJson for PersistCommand {
    fn to_json(&self) -> String {
        match self {
            PersistCommand::Set { key, value } => {
                format!(
                    "{{\"type\":\"Set\",\"key\":\"{}\",\"value\":\"{}\"}}",
                    escape_json_string(key),
                    escape_json_string(value)
                )
            }
            PersistCommand::Remove { key } => {
                format!(
                    "{{\"type\":\"Remove\",\"key\":\"{}\"}}",
                    escape_json_string(key)
                )
            }
            PersistCommand::Clear => {
                "{\"type\":\"Clear\"}".to_string()
            }
        }
    }
}

// ─── PersistQueue ───────────────────────────────────────────────────

/// Queue of persistence commands, backed by the generic `CommandQueue`.
pub type PersistQueue = CommandQueue<PersistCommand>;

// ─── Convenience Helpers ────────────────────────────────────────────

/// Push a Set command onto the persist queue.
pub fn persist_set(queue: &mut PersistQueue, key: &str, value: &str) {
    queue.push(PersistCommand::Set {
        key: key.to_string(),
        value: value.to_string(),
    });
}

/// Push a Remove command onto the persist queue.
pub fn persist_remove(queue: &mut PersistQueue, key: &str) {
    queue.push(PersistCommand::Remove {
        key: key.to_string(),
    });
}

/// Push a Clear command onto the persist queue.
pub fn persist_clear(queue: &mut PersistQueue) {
    queue.push(PersistCommand::Clear);
}

// ─── JSON Escaping ──────────────────────────────────────────────────

/// Minimal JSON string escaping for values embedded in hand-built JSON.
fn escape_json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            // RFC 8259: all control chars U+0000–U+001F must be escaped
            c if c < '\u{0020}' => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out
}

// ─── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_queue_drains_to_empty_array() {
        let mut queue = PersistQueue::new();
        assert_eq!(queue.drain_json(), "[]");
    }

    #[test]
    fn set_command_produces_valid_json() {
        let mut queue = PersistQueue::new();
        persist_set(&mut queue, "player_name", "Alice");

        let json = queue.drain_json();
        assert!(queue.is_empty());

        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("drain_json must produce valid JSON");
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["type"], "Set");
        assert_eq!(arr[0]["key"], "player_name");
        assert_eq!(arr[0]["value"], "Alice");
    }

    #[test]
    fn remove_command_produces_valid_json() {
        let mut queue = PersistQueue::new();
        persist_remove(&mut queue, "old_key");

        let json = queue.drain_json();
        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("drain_json must produce valid JSON");
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["type"], "Remove");
        assert_eq!(arr[0]["key"], "old_key");
    }

    #[test]
    fn clear_command_produces_valid_json() {
        let mut queue = PersistQueue::new();
        persist_clear(&mut queue);

        let json = queue.drain_json();
        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("drain_json must produce valid JSON");
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["type"], "Clear");
    }

    #[test]
    fn multiple_commands_drain_correctly() {
        let mut queue = PersistQueue::new();
        persist_set(&mut queue, "score", "100");
        persist_set(&mut queue, "level", "3");
        persist_remove(&mut queue, "temp");
        persist_clear(&mut queue);

        assert_eq!(queue.len(), 4);

        let json = queue.drain_json();
        assert!(queue.is_empty());

        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("drain_json must produce valid JSON");
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr.len(), 4);

        assert_eq!(arr[0]["type"], "Set");
        assert_eq!(arr[0]["key"], "score");
        assert_eq!(arr[0]["value"], "100");

        assert_eq!(arr[1]["type"], "Set");
        assert_eq!(arr[1]["key"], "level");
        assert_eq!(arr[1]["value"], "3");

        assert_eq!(arr[2]["type"], "Remove");
        assert_eq!(arr[2]["key"], "temp");

        assert_eq!(arr[3]["type"], "Clear");
    }

    #[test]
    fn json_escaping_special_characters() {
        let mut queue = PersistQueue::new();
        persist_set(
            &mut queue,
            "key\"with\\special",
            "value\nwith\ttabs\rand\"quotes\\",
        );

        let json = queue.drain_json();
        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("JSON with escaped characters must parse");
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr[0]["key"], "key\"with\\special");
        assert_eq!(arr[0]["value"], "value\nwith\ttabs\rand\"quotes\\");
    }

    #[test]
    fn set_command_with_json_value() {
        let mut queue = PersistQueue::new();
        persist_set(
            &mut queue,
            "srs_state",
            "{\"cards\":{},\"count\":0}",
        );

        let json = queue.drain_json();
        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("JSON with embedded JSON value must parse");
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr[0]["type"], "Set");
        assert_eq!(arr[0]["key"], "srs_state");
        // The value should be a string containing JSON
        assert_eq!(arr[0]["value"], "{\"cards\":{},\"count\":0}");
    }

    #[test]
    fn drain_clears_queue() {
        let mut queue = PersistQueue::new();
        persist_set(&mut queue, "a", "1");
        persist_set(&mut queue, "b", "2");

        let first = queue.drain_json();
        assert!(!first.is_empty());
        assert!(queue.is_empty());

        // Second drain should return empty array
        let second = queue.drain_json();
        assert_eq!(second, "[]");
    }

    #[test]
    fn direct_push_works() {
        let mut queue = PersistQueue::new();
        queue.push(PersistCommand::Set {
            key: "direct".to_string(),
            value: "push".to_string(),
        });
        assert_eq!(queue.len(), 1);

        let json = queue.drain_json();
        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("direct push must produce valid JSON");
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr[0]["type"], "Set");
        assert_eq!(arr[0]["key"], "direct");
        assert_eq!(arr[0]["value"], "push");
    }
}
