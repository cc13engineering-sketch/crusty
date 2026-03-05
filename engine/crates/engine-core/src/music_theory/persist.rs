/// Persistence command queue — mirrors the SoundCommandQueue pattern.
///
/// Rust code pushes `PersistCommand`s into a `PersistQueue`. Each frame
/// the JS side drains the queue (via a WASM binding that returns JSON)
/// and writes values to localStorage.

// ─── PersistCommand ─────────────────────────────────────────────────

/// A single persistence command to be consumed by the JS persistence driver.
#[derive(Clone, Debug)]
pub enum PersistCommand {
    /// Store a key-value pair in persistent storage.
    Store { key: String, value: String },
}

impl PersistCommand {
    fn to_json(&self) -> String {
        match self {
            PersistCommand::Store { key, value } => {
                format!(
                    "{{\"type\":\"Store\",\"key\":\"{}\",\"value\":\"{}\"}}",
                    escape_json(key),
                    escape_json(value)
                )
            }
        }
    }
}

fn escape_json(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

// ─── PersistQueue ───────────────────────────────────────────────────

/// Accumulates `PersistCommand`s during a frame. The JS side drains the
/// queue each frame via `drain_json()`.
#[derive(Clone, Debug, Default)]
pub struct PersistQueue {
    commands: Vec<PersistCommand>,
}

impl PersistQueue {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn push(&mut self, cmd: PersistCommand) {
        self.commands.push(cmd);
    }

    /// Drain all queued commands, returning them as a JSON array string.
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

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_queue_drains_empty_array() {
        let mut q = PersistQueue::new();
        assert_eq!(q.drain_json(), "[]");
    }

    #[test]
    fn store_command_produces_valid_json() {
        let mut q = PersistQueue::new();
        q.push(PersistCommand::Store {
            key: "srs_state".to_string(),
            value: "{\"cards\":{}}".to_string(),
        });
        let json = q.drain_json();
        assert!(q.is_empty());
        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("must produce valid JSON");
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["type"], "Store");
        assert_eq!(arr[0]["key"], "srs_state");
    }
}
