# Game State Telemetry Module

## Goal

Record complete playthroughs as compressed delta streams. Let games decide what state to track and when. Provide a command-queue-based API to POST recorded state to an external endpoint.

---

## Core Concept: Delta-Compressed State Snapshots

Don't record every frame — record **what changed**. A playthrough becomes a sequence of timestamped deltas against the initial state. Games control granularity: a turn-based game posts once per turn, a physics game might post every N frames, a music game posts per challenge.

```
Playthrough = InitialSnapshot + [Delta(t=0.5s), Delta(t=1.2s), Delta(t=3.0s), ...]
```

Each delta stores only keys that changed since the last snapshot. Unchanged state costs zero bytes.

---

## Architecture

### New Files

```
engine/crates/engine-core/src/
├── telemetry.rs          # Core module: TelemetryRecorder + TelemetryPoster
```

One file. One module. Two responsibilities separated by struct.

### Module Registration

1. Add `pub mod telemetry;` to `lib.rs`
2. Add `pub telemetry: TelemetryRecorder` field to `Engine`
3. Initialize in `Engine::new()`, clear in `Engine::reset()`
4. **No automatic per-frame calls** — games drive recording explicitly

---

## API Design

### 1. TelemetryRecorder — State Tracking

```rust
/// Delta-compressed game state recorder.
/// Games call `update()` whenever meaningful state changes.
/// The recorder diffs against the previous snapshot and stores only deltas.
pub struct TelemetryRecorder {
    /// Unique ID for this playthrough (set on reset)
    session_id: u64,
    /// The "current known state" — full key-value map
    current_snapshot: BTreeMap<String, StateValue>,
    /// Initial state captured on first update
    initial_snapshot: Option<BTreeMap<String, StateValue>>,
    /// Ordered list of deltas since initial snapshot
    deltas: Vec<StateDelta>,
    /// Whether recording is active
    enabled: bool,
}

#[derive(Clone, Debug)]
pub enum StateValue {
    F64(f64),
    Bool(bool),
    Str(String),
    /// For arrays/nested objects — store as pre-serialized JSON
    Json(String),
}

#[derive(Clone, Debug)]
pub struct StateDelta {
    /// Engine frame number when this delta was recorded
    frame: u64,
    /// Elapsed simulation time in seconds
    time_s: f64,
    /// Only the keys that changed (or were added)
    changes: BTreeMap<String, StateValue>,
    /// Keys that were removed
    removed: Vec<String>,
}
```

**Game-facing API:**

```rust
impl TelemetryRecorder {
    /// Start a new recording session. Called automatically by Engine::reset().
    pub fn begin_session(&mut self, session_id: u64)

    /// Update a single key. Recorder diffs internally.
    /// No-op if value hasn't changed (dedup at write time).
    pub fn set_f64(&mut self, key: &str, value: f64)
    pub fn set_bool(&mut self, key: &str, value: bool)
    pub fn set_str(&mut self, key: &str, value: String)
    pub fn set_json(&mut self, key: &str, json: String)

    /// Remove a key from tracked state.
    pub fn remove(&mut self, key: &str)

    /// Commit current pending changes as a delta at the given frame/time.
    /// Games call this when they want to "checkpoint" — e.g. end of turn,
    /// after a scoring event, every N frames, etc.
    /// Returns true if a delta was actually recorded (i.e., something changed).
    pub fn commit(&mut self, frame: u64, time_s: f64) -> bool

    /// Enable/disable recording.
    pub fn set_enabled(&mut self, enabled: bool)

    /// Serialize entire playthrough (initial + deltas) to JSON.
    /// Used by TelemetryPoster to build request body.
    pub fn to_json(&self) -> String

    /// Number of deltas recorded so far.
    pub fn delta_count(&self) -> usize

    /// Clear all recorded data.
    pub fn clear(&mut self)
}
```

**Usage pattern in a Simulation:**

```rust
fn step(&mut self, engine: &mut Engine) {
    // Game logic runs...
    self.score += 10;
    self.level = 3;

    // Update telemetry state (cheap — just BTreeMap inserts)
    engine.telemetry.set_f64("score", self.score as f64);
    engine.telemetry.set_f64("level", self.level as f64);
    engine.telemetry.set_str("last_action", "cleared_row".into());

    // Commit a checkpoint (only when game decides it's meaningful)
    engine.telemetry.commit(engine.frame, engine.elapsed_time);
}
```

### 2. TelemetryPoster — External Dispatch

Follows the existing **command queue pattern** (like `SoundCommand`, `PersistCommand`). The engine queues "post" commands; JS drains and executes them via `fetch()`.

```rust
/// Command to POST telemetry data to an external endpoint.
#[derive(Clone, Debug)]
pub struct TelemetryPostCommand {
    /// Target URL to POST to
    pub url: String,
    /// Additional headers (e.g., auth tokens, content-type overrides)
    pub headers: Vec<(String, String)>,
    /// The body payload — typically from TelemetryRecorder::to_json()
    pub body: String,
    /// Opaque request ID so the callback can correlate responses
    pub request_id: u64,
}

impl ToJson for TelemetryPostCommand {
    fn to_json(&self) -> String { /* serialize to JSON */ }
}
```

**Engine-side API:**

```rust
impl Engine {
    /// Queue a telemetry POST. JS will drain and execute via fetch().
    /// The `request_id` is echoed back in the callback.
    pub fn post_telemetry(&mut self, url: &str, headers: Vec<(String, String)>, request_id: u64) {
        let body = self.telemetry.to_json();
        self.telemetry_post_queue.push(TelemetryPostCommand {
            url: url.to_string(),
            headers,
            body,
            request_id,
        });
    }
}
```

**Engine fields added:**

```rust
pub struct Engine {
    // ... existing fields ...
    pub telemetry: TelemetryRecorder,
    pub telemetry_post_queue: CommandQueue<TelemetryPostCommand>,
}
```

### 3. WASM Bindings

```rust
// In lib.rs — drain command queue for JS to execute
#[wasm_bindgen]
pub fn drain_telemetry_commands() -> String {
    ENGINE.with(|e| {
        let mut eng = e.borrow_mut();
        let eng = eng.as_mut().unwrap();
        eng.telemetry_post_queue.drain_json()
    })
}

// JS calls this when a POST succeeds/fails
#[wasm_bindgen]
pub fn telemetry_post_callback(request_id: u64, success: bool, response_body: &str) {
    ENGINE.with(|e| {
        let mut eng = e.borrow_mut();
        let eng = eng.as_mut().unwrap();
        eng.telemetry.on_post_result(request_id, success, response_body);
    });
}
```

### 4. JS-Side Integration (sketch — not in engine crate)

```javascript
// In the game's tick loop, after engine.tick():
const cmds = JSON.parse(Module.drain_telemetry_commands());
for (const cmd of cmds) {
    const headers = { 'Content-Type': 'application/json' };
    for (const [k, v] of cmd.headers) headers[k] = v;

    fetch(cmd.url, { method: 'POST', headers, body: cmd.body })
        .then(r => r.text().then(body =>
            Module.telemetry_post_callback(cmd.request_id, r.ok, body)
        ))
        .catch(err =>
            Module.telemetry_post_callback(cmd.request_id, false, err.message)
        );
}
```

---

## Data Format — Serialized Playthrough JSON

```json
{
    "session_id": 12345,
    "engine_version": "0.1.0",
    "initial_state": {
        "score": 0.0,
        "level": 1.0,
        "lives": 3.0,
        "game": "gravity-pong"
    },
    "deltas": [
        {
            "frame": 120,
            "time_s": 2.0,
            "changes": { "score": 10.0 },
            "removed": []
        },
        {
            "frame": 300,
            "time_s": 5.0,
            "changes": { "score": 25.0, "level": 2.0 },
            "removed": []
        }
    ],
    "final_state": {
        "score": 25.0,
        "level": 2.0,
        "lives": 3.0,
        "game": "gravity-pong"
    },
    "total_frames": 3600,
    "total_time_s": 60.0,
    "delta_count": 2
}
```

The `final_state` is always a full snapshot (initial + all deltas applied) so consumers don't need to replay deltas to know end state.

---

## Implementation Steps

### Step 1: `telemetry.rs` — Core structs and recording logic
- `StateValue` enum
- `StateDelta` struct
- `TelemetryRecorder` with set/commit/clear/to_json
- `TelemetryPostCommand` with `ToJson` impl
- Internal diff logic: compare pending writes against `current_snapshot`, emit only changes
- BTreeMap for deterministic key ordering in JSON output

### Step 2: Wire into Engine
- Add `pub mod telemetry;` to `lib.rs`
- Add `telemetry: TelemetryRecorder` and `telemetry_post_queue: CommandQueue<TelemetryPostCommand>` to Engine
- `Engine::new()` — initialize both
- `Engine::reset()` — call `telemetry.begin_session(rng.next_u64())`, clear post queue
- Add `post_telemetry()` convenience method on Engine

### Step 3: WASM bindings
- `drain_telemetry_commands()` — mirrors existing drain patterns
- `telemetry_post_callback()` — JS → WASM callback for post results

### Step 4: Integration test
- Unit test: set values, commit, verify delta only contains changes
- Unit test: unchanged values between commits produce empty delta (commit returns false)
- Unit test: to_json produces valid JSON with initial + deltas + final state
- Unit test: clear resets everything

---

## Design Decisions

| Decision | Rationale |
|----------|-----------|
| **Games drive recording, not engine** | Different games have wildly different "meaningful state change" granularity. A physics sim might commit every 60 frames; a card game commits per turn. |
| **BTreeMap for state** | Deterministic key ordering → reproducible JSON output → easier diffing/debugging. |
| **StateValue::Json variant** | Lets games store nested/complex state (e.g., full board state, entity positions) without the engine needing to understand the schema. |
| **Command queue for POST** | Follows existing crusty pattern (sound, persist). Engine stays pure; JS handles async I/O. |
| **request_id + callback** | Games need to know if their POST succeeded (e.g., to retry, show confirmation, unlock cloud save). Opaque u64 lets games correlate. |
| **No auto-commit** | Avoids recording noise. Games know when state changes meaningfully. |
| **Session ID from RNG** | Deterministic per seed → same seed produces same session ID → useful for replay correlation. |

## What This Does NOT Cover (intentionally deferred)

- **Storage backend** — Where the POST endpoint lives, database schema, costs. User said "we'll consider cost and security later."
- **Compression** — The delta format is already compact. Could add zlib/brotli later if payload size matters.
- **Replay** — Reading deltas back and reconstructing state. Natural extension but not in scope.
- **Privacy/PII** — Games control what keys they track. No automatic entity/component scraping.
- **Rate limiting** — JS-side concern, not engine-side.
- **Retry logic** — JS-side. The callback gives games enough info to implement their own retry.
