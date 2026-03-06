/// Delta-compressed game state telemetry.
///
/// Games call `set_*()` to update tracked values and `commit()` to checkpoint.
/// The recorder diffs against the previous snapshot and stores only deltas.
/// A command-queue-based API lets JS POST recorded state to external endpoints.
///
/// BTreeMap is used throughout for deterministic key ordering, which produces
/// reproducible JSON output (important for testing and diffing).
///
/// JSON is hand-rolled (not serde) to avoid pulling serde into the WASM binary.

use std::collections::BTreeMap;
use crate::command_queue::ToJson;

// ─── StateValue ─────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum StateValue {
    F64(f64),
    Bool(bool),
    Str(String),
    /// Pre-serialized JSON for arrays/nested objects.
    Json(String),
}

impl StateValue {
    /// Compare two StateValues for equality (f64 uses bitwise comparison
    /// so NaN == NaN returns true, avoiding unbounded delta growth).
    fn same_as(&self, other: &StateValue) -> bool {
        match (self, other) {
            (StateValue::F64(a), StateValue::F64(b)) => a.to_bits() == b.to_bits(),
            (StateValue::Bool(a), StateValue::Bool(b)) => a == b,
            (StateValue::Str(a), StateValue::Str(b)) => a == b,
            (StateValue::Json(a), StateValue::Json(b)) => a == b,
            _ => false,
        }
    }

    fn to_json_value(&self) -> String {
        match self {
            StateValue::F64(v) => format_f64(*v),
            StateValue::Bool(v) => if *v { "true".to_string() } else { "false".to_string() },
            StateValue::Str(v) => format!("\"{}\"", escape_json(v)),
            StateValue::Json(v) => v.clone(),
        }
    }
}

// ─── StateDelta ─────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct StateDelta {
    frame: u64,
    time_s: f64,
    changes: BTreeMap<String, StateValue>,
    removed: Vec<String>,
}

impl StateDelta {
    fn to_json(&self) -> String {
        let mut s = String::from("{");
        s.push_str(&format!("\"frame\":{},\"time_s\":{},\"changes\":{{", self.frame, format_f64(self.time_s)));
        for (i, (k, v)) in self.changes.iter().enumerate() {
            if i > 0 { s.push(','); }
            s.push_str(&format!("\"{}\":{}", escape_json(k), v.to_json_value()));
        }
        s.push_str("},\"removed\":[");
        for (i, k) in self.removed.iter().enumerate() {
            if i > 0 { s.push(','); }
            s.push_str(&format!("\"{}\"", escape_json(k)));
        }
        s.push_str("]}");
        s
    }
}

// ─── TelemetryRecorder ─────────────────────────────────────────────

pub struct TelemetryRecorder {
    session_id: u64,
    current_snapshot: BTreeMap<String, StateValue>,
    pending: BTreeMap<String, StateValue>,
    pending_removes: Vec<String>,
    initial_snapshot: Option<BTreeMap<String, StateValue>>,
    deltas: Vec<StateDelta>,
    enabled: bool,
    last_frame: u64,
    last_time_s: f64,
}

impl TelemetryRecorder {
    pub fn new() -> Self {
        Self {
            session_id: 0,
            current_snapshot: BTreeMap::new(),
            pending: BTreeMap::new(),
            pending_removes: Vec::new(),
            initial_snapshot: None,
            deltas: Vec::new(),
            enabled: true,
            last_frame: 0,
            last_time_s: 0.0,
        }
    }

    pub fn begin_session(&mut self, session_id: u64) {
        self.clear();
        self.session_id = session_id;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_f64(&mut self, key: &str, value: f64) {
        if !self.enabled { return; }
        self.pending_removes.retain(|k| k != key);
        self.pending.insert(key.to_string(), StateValue::F64(value));
    }

    pub fn set_bool(&mut self, key: &str, value: bool) {
        if !self.enabled { return; }
        self.pending_removes.retain(|k| k != key);
        self.pending.insert(key.to_string(), StateValue::Bool(value));
    }

    pub fn set_str(&mut self, key: &str, value: &str) {
        if !self.enabled { return; }
        self.pending_removes.retain(|k| k != key);
        self.pending.insert(key.to_string(), StateValue::Str(value.to_string()));
    }

    pub fn set_json(&mut self, key: &str, json: String) {
        if !self.enabled { return; }
        self.pending_removes.retain(|k| k != key);
        self.pending.insert(key.to_string(), StateValue::Json(json));
    }

    pub fn remove(&mut self, key: &str) {
        if !self.enabled { return; }
        self.pending.remove(key);
        self.pending_removes.push(key.to_string());
    }

    /// Commit pending changes as a delta. Returns true if a delta was recorded.
    pub fn commit(&mut self, frame: u64, time_s: f64) -> bool {
        if !self.enabled { return false; }

        let pending = std::mem::take(&mut self.pending);
        let pending_removes = std::mem::take(&mut self.pending_removes);

        // Capture initial snapshot on first commit
        if self.initial_snapshot.is_none() {
            for (k, v) in pending {
                self.current_snapshot.insert(k, v);
            }
            for k in pending_removes {
                self.current_snapshot.remove(&k);
            }
            self.initial_snapshot = Some(self.current_snapshot.clone());
            self.last_frame = frame;
            self.last_time_s = time_s;
            return true;
        }

        // Diff pending against current_snapshot
        let mut changes = BTreeMap::new();
        for (k, v) in pending {
            let changed = match self.current_snapshot.get(&k) {
                Some(existing) => !existing.same_as(&v),
                None => true,
            };
            if changed {
                self.current_snapshot.insert(k.clone(), v.clone());
                changes.insert(k, v);
            }
        }

        let mut removed = Vec::new();
        for k in pending_removes {
            if self.current_snapshot.remove(&k).is_some() {
                removed.push(k);
            }
        }

        if changes.is_empty() && removed.is_empty() {
            return false;
        }

        self.deltas.push(StateDelta {
            frame,
            time_s,
            changes,
            removed,
        });
        self.last_frame = frame;
        self.last_time_s = time_s;
        true
    }

    pub fn delta_count(&self) -> usize {
        self.deltas.len()
    }

    /// Clear all recorded data. Does not reset session_id or enabled.
    pub fn clear(&mut self) {
        self.current_snapshot.clear();
        self.pending.clear();
        self.pending_removes.clear();
        self.initial_snapshot = None;
        self.deltas.clear();
        self.last_frame = 0;
        self.last_time_s = 0.0;
    }

    pub fn to_json(&self) -> String {
        let initial = match &self.initial_snapshot {
            Some(snap) => map_to_json(snap),
            None => "{}".to_string(),
        };

        let mut deltas_json = String::from("[");
        for (i, d) in self.deltas.iter().enumerate() {
            if i > 0 { deltas_json.push(','); }
            deltas_json.push_str(&d.to_json());
        }
        deltas_json.push(']');

        let final_state = map_to_json(&self.current_snapshot);

        format!(
            "{{\"session_id\":{},\"engine_version\":\"{}\",\"initial_state\":{},\"deltas\":{},\"final_state\":{},\"total_frames\":{},\"total_time_s\":{},\"delta_count\":{}}}",
            self.session_id,
            env!("CARGO_PKG_VERSION"),
            initial,
            deltas_json,
            final_state,
            self.last_frame,
            format_f64(self.last_time_s),
            self.deltas.len()
        )
    }

    /// Called by JS via WASM when a POST request completes.
    pub fn on_post_result(&mut self, request_id: u64, success: bool, response_body: &str) {
        if success {
            crate::log::log(&format!("telemetry POST {} succeeded", request_id));
        } else {
            crate::log::warn(&format!("telemetry POST {} failed: {}", request_id, response_body));
        }
    }
}

impl Default for TelemetryRecorder {
    fn default() -> Self {
        Self::new()
    }
}

// ─── TelemetryPostCommand ───────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct TelemetryPostCommand {
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub request_id: u64,
}

impl ToJson for TelemetryPostCommand {
    fn to_json(&self) -> String {
        let mut headers_json = String::from("[");
        for (i, (k, v)) in self.headers.iter().enumerate() {
            if i > 0 { headers_json.push(','); }
            headers_json.push_str(&format!("[\"{}\",\"{}\"]", escape_json(k), escape_json(v)));
        }
        headers_json.push(']');

        format!(
            "{{\"url\":\"{}\",\"headers\":{},\"body\":{},\"request_id\":{}}}",
            escape_json(&self.url),
            headers_json,
            // body is already JSON, embed directly
            &self.body,
            self.request_id
        )
    }
}

// ─── Helpers ────────────────────────────────────────────────────────

fn escape_json(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                // RFC 8259: all control chars U+0000-U+001F must be escaped
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out
}

fn format_f64(v: f64) -> String {
    if v.is_nan() {
        "null".to_string()
    } else if v.is_infinite() {
        if v.is_sign_positive() { "1e308".to_string() } else { "-1e308".to_string() }
    } else if v.fract() == 0.0 {
        format!("{:.1}", v)
    } else {
        format!("{}", v)
    }
}

fn map_to_json(map: &BTreeMap<String, StateValue>) -> String {
    let mut s = String::from("{");
    for (i, (k, v)) in map.iter().enumerate() {
        if i > 0 { s.push(','); }
        s.push_str(&format!("\"{}\":{}", escape_json(k), v.to_json_value()));
    }
    s.push('}');
    s
}

// ─── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_commit_records_initial_snapshot() {
        let mut rec = TelemetryRecorder::new();
        rec.set_f64("score", 0.0);
        rec.set_str("game", "test");
        let recorded = rec.commit(0, 0.0);
        assert!(recorded);
        assert!(rec.initial_snapshot.is_some());
        assert_eq!(rec.delta_count(), 0); // initial snapshot, no deltas yet
    }

    #[test]
    fn delta_only_contains_changes() {
        let mut rec = TelemetryRecorder::new();
        rec.set_f64("score", 0.0);
        rec.set_f64("level", 1.0);
        rec.commit(0, 0.0);

        // Change only score
        rec.set_f64("score", 10.0);
        rec.set_f64("level", 1.0); // unchanged
        let recorded = rec.commit(60, 1.0);
        assert!(recorded);
        assert_eq!(rec.delta_count(), 1);
        assert!(rec.deltas[0].changes.contains_key("score"));
        assert!(!rec.deltas[0].changes.contains_key("level"));
    }

    #[test]
    fn unchanged_values_produce_no_delta() {
        let mut rec = TelemetryRecorder::new();
        rec.set_f64("score", 0.0);
        rec.commit(0, 0.0);

        // Same value
        rec.set_f64("score", 0.0);
        let recorded = rec.commit(60, 1.0);
        assert!(!recorded);
        assert_eq!(rec.delta_count(), 0);
    }

    #[test]
    fn to_json_produces_valid_json() {
        let mut rec = TelemetryRecorder::new();
        rec.begin_session(12345);
        rec.set_f64("score", 0.0);
        rec.set_bool("alive", true);
        rec.set_str("game", "pong");
        rec.commit(0, 0.0);

        rec.set_f64("score", 25.0);
        rec.commit(300, 5.0);

        let json = rec.to_json();
        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("to_json must produce valid JSON");

        assert_eq!(parsed["session_id"], 12345);
        assert_eq!(parsed["delta_count"], 1);

        // initial_state should have all three keys
        let initial = &parsed["initial_state"];
        assert_eq!(initial["score"], 0.0);
        assert_eq!(initial["alive"], true);
        assert_eq!(initial["game"], "pong");

        // final_state should reflect changes
        let final_s = &parsed["final_state"];
        assert_eq!(final_s["score"], 25.0);
        assert_eq!(final_s["alive"], true);
        assert_eq!(final_s["game"], "pong");

        // deltas
        let deltas = parsed["deltas"].as_array().unwrap();
        assert_eq!(deltas.len(), 1);
        assert_eq!(deltas[0]["frame"], 300);
        assert_eq!(deltas[0]["changes"]["score"], 25.0);
    }

    #[test]
    fn clear_resets_everything() {
        let mut rec = TelemetryRecorder::new();
        rec.set_f64("score", 100.0);
        rec.commit(0, 0.0);
        rec.set_f64("score", 200.0);
        rec.commit(60, 1.0);
        assert_eq!(rec.delta_count(), 1);

        rec.clear();
        assert_eq!(rec.delta_count(), 0);
        assert!(rec.initial_snapshot.is_none());
        assert!(rec.current_snapshot.is_empty());
    }

    #[test]
    fn multiple_commits_accumulate_deltas() {
        let mut rec = TelemetryRecorder::new();
        rec.set_f64("score", 0.0);
        rec.commit(0, 0.0);

        rec.set_f64("score", 10.0);
        rec.commit(60, 1.0);

        rec.set_f64("score", 25.0);
        rec.commit(120, 2.0);

        rec.set_f64("score", 50.0);
        rec.commit(180, 3.0);

        assert_eq!(rec.delta_count(), 3);
    }

    #[test]
    fn remove_key_appears_in_delta() {
        let mut rec = TelemetryRecorder::new();
        rec.set_f64("score", 0.0);
        rec.set_str("powerup", "shield");
        rec.commit(0, 0.0);

        rec.remove("powerup");
        let recorded = rec.commit(60, 1.0);
        assert!(recorded);
        assert_eq!(rec.deltas[0].removed, vec!["powerup"]);

        let json = rec.to_json();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(!parsed["final_state"].as_object().unwrap().contains_key("powerup"));
    }

    #[test]
    fn disabled_recorder_ignores_everything() {
        let mut rec = TelemetryRecorder::new();
        rec.set_enabled(false);
        rec.set_f64("score", 100.0);
        assert!(!rec.commit(0, 0.0));
        assert!(rec.initial_snapshot.is_none());
    }

    #[test]
    fn post_command_to_json() {
        let cmd = TelemetryPostCommand {
            url: "https://example.com/telemetry".to_string(),
            headers: vec![("Authorization".to_string(), "Bearer tok".to_string())],
            body: "{\"test\":true}".to_string(),
            request_id: 42,
        };
        let json = cmd.to_json();
        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("TelemetryPostCommand::to_json must produce valid JSON");
        assert_eq!(parsed["url"], "https://example.com/telemetry");
        assert_eq!(parsed["request_id"], 42);
        assert_eq!(parsed["body"]["test"], true);
    }

    #[test]
    fn json_value_type_preserved() {
        let mut rec = TelemetryRecorder::new();
        rec.set_json("board", "[1,2,3]".into());
        rec.commit(0, 0.0);

        let json = rec.to_json();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let board = &parsed["initial_state"]["board"];
        assert!(board.is_array());
        assert_eq!(board[0], 1);
    }

    #[test]
    fn nan_and_infinity_produce_valid_json() {
        let mut rec = TelemetryRecorder::new();
        rec.set_f64("nan_val", f64::NAN);
        rec.set_f64("inf_val", f64::INFINITY);
        rec.set_f64("neg_inf", f64::NEG_INFINITY);
        rec.commit(0, 0.0);

        let json = rec.to_json();
        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("non-finite f64 values must produce valid JSON");
        assert!(parsed["initial_state"]["nan_val"].is_null());
    }

    #[test]
    fn nan_does_not_cause_spurious_deltas() {
        let mut rec = TelemetryRecorder::new();
        rec.set_f64("v", f64::NAN);
        rec.commit(0, 0.0);

        rec.set_f64("v", f64::NAN);
        let recorded = rec.commit(60, 1.0);
        assert!(!recorded, "NaN == NaN should not report a change");
    }

    #[test]
    fn control_chars_escaped_in_json() {
        let mut rec = TelemetryRecorder::new();
        rec.set_str("name", "a\x00b\x1Fc");
        rec.commit(0, 0.0);

        let json = rec.to_json();
        serde_json::from_str::<serde_json::Value>(&json)
            .expect("control characters must be escaped to valid JSON");
    }

    #[test]
    fn set_after_remove_wins() {
        let mut rec = TelemetryRecorder::new();
        rec.set_f64("x", 1.0);
        rec.commit(0, 0.0);

        // remove then set in same frame — set should win
        rec.remove("x");
        rec.set_f64("x", 99.0);
        let recorded = rec.commit(60, 1.0);
        assert!(recorded);

        let json = rec.to_json();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["final_state"]["x"], 99.0);
    }
}
