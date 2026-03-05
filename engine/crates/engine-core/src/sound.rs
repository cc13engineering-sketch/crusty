/// SoundScape — command-buffer audio system.
///
/// Rust code pushes `SoundCommand`s into a `SoundCommandQueue`. Each frame the
/// JS side drains the queue (via a WASM binding that returns JSON) and drives
/// the Web Audio API accordingly.
///
/// `SoundPalette` provides named preset profiles so game code can trigger
/// sounds by event name ("impact", "pickup", etc.) without knowing the exact
/// parameters.

use std::collections::HashMap;

// ─── Waveform ────────────────────────────────────────────────────────

/// Oscillator waveform type, maps directly to Web Audio OscillatorNode types.
#[derive(Clone, Debug, PartialEq)]
pub enum Waveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

impl Waveform {
    /// JSON-friendly lowercase string.
    fn as_str(&self) -> &'static str {
        match self {
            Waveform::Sine => "sine",
            Waveform::Square => "square",
            Waveform::Triangle => "triangle",
            Waveform::Sawtooth => "sawtooth",
        }
    }
}

// ─── SoundCommand ────────────────────────────────────────────────────

/// A single audio command to be consumed by the JS audio driver.
#[derive(Clone, Debug)]
pub enum SoundCommand {
    /// Play a single tone with envelope shaping.
    PlayTone {
        frequency: f64,
        duration: f64,
        volume: f64,
        waveform: Waveform,
        attack: f64,
        decay: f64,
    },
    /// Play a burst of noise (e.g. explosions, static).
    PlayNoise {
        duration: f64,
        volume: f64,
        filter_freq: f64,
    },
    /// Start a continuously looping oscillator with an identifier.
    StartLoop {
        id: String,
        frequency: f64,
        volume: f64,
        waveform: Waveform,
    },
    /// Stop a looping oscillator, optionally fading out.
    StopLoop {
        id: String,
        fade_out: f64,
    },
    /// Set the global master volume (0.0 .. 1.0).
    SetVolume {
        master_volume: f64,
    },
    /// Play a named audio sample with pitch shifting.
    /// The `name` field identifies a pre-loaded sample (e.g. "ka", "a").
    /// `pitch` is a playback rate multiplier (1.0 = original pitch).
    PlaySample {
        name: String,
        volume: f64,
        pitch: f64,
        duration: f64,
    },
}

impl SoundCommand {
    /// Serialize a single command to a JSON string.
    fn to_json(&self) -> String {
        match self {
            SoundCommand::PlayTone {
                frequency, duration, volume, waveform, attack, decay,
            } => {
                format!(
                    "{{\"type\":\"PlayTone\",\"frequency\":{},\"duration\":{},\"volume\":{},\"waveform\":\"{}\",\"attack\":{},\"decay\":{}}}",
                    frequency, duration, volume, waveform.as_str(), attack, decay
                )
            }
            SoundCommand::PlayNoise {
                duration, volume, filter_freq,
            } => {
                format!(
                    "{{\"type\":\"PlayNoise\",\"duration\":{},\"volume\":{},\"filter_freq\":{}}}",
                    duration, volume, filter_freq
                )
            }
            SoundCommand::StartLoop {
                id, frequency, volume, waveform,
            } => {
                format!(
                    "{{\"type\":\"StartLoop\",\"id\":\"{}\",\"frequency\":{},\"volume\":{},\"waveform\":\"{}\"}}",
                    escape_json_string(id), frequency, volume, waveform.as_str()
                )
            }
            SoundCommand::StopLoop { id, fade_out } => {
                format!(
                    "{{\"type\":\"StopLoop\",\"id\":\"{}\",\"fade_out\":{}}}",
                    escape_json_string(id), fade_out
                )
            }
            SoundCommand::SetVolume { master_volume } => {
                format!(
                    "{{\"type\":\"SetVolume\",\"master_volume\":{}}}",
                    master_volume
                )
            }
            SoundCommand::PlaySample {
                name, volume, pitch, duration,
            } => {
                format!(
                    "{{\"type\":\"PlaySample\",\"name\":\"{}\",\"volume\":{},\"pitch\":{},\"duration\":{}}}",
                    escape_json_string(name), volume, pitch, duration
                )
            }
        }
    }
}

/// Minimal JSON string escaping for identifiers embedded in JSON values.
fn escape_json_string(s: &str) -> String {
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

// ─── SoundCommandQueue ───────────────────────────────────────────────

/// Accumulates `SoundCommand`s during a frame. The JS side drains the queue
/// each frame via `drain_json()`.
#[derive(Clone, Debug, Default)]
pub struct SoundCommandQueue {
    commands: Vec<SoundCommand>,
}

impl SoundCommandQueue {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Push a command onto the queue.
    pub fn push(&mut self, cmd: SoundCommand) {
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

// ─── SoundPalette ────────────────────────────────────────────────────

/// Named sound profiles. Each profile is a list of `SoundCommand`s that are
/// pushed into the queue when the profile is triggered.
#[derive(Clone, Debug)]
pub struct SoundPalette {
    profiles: HashMap<String, Vec<SoundCommand>>,
}

impl Default for SoundPalette {
    fn default() -> Self {
        Self::default_palette()
    }
}

impl SoundPalette {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }

    /// Register a named profile (replaces any existing profile with the same name).
    pub fn register(&mut self, name: &str, commands: Vec<SoundCommand>) {
        self.profiles.insert(name.to_string(), commands);
    }

    /// Check whether a profile with the given name exists.
    pub fn has(&self, name: &str) -> bool {
        self.profiles.contains_key(name)
    }

    /// Play a named sound profile by pushing its commands into the queue.
    /// Returns `true` if the profile was found.
    pub fn play(&self, event_name: &str, queue: &mut SoundCommandQueue) -> bool {
        if let Some(commands) = self.profiles.get(event_name) {
            for cmd in commands {
                queue.push(cmd.clone());
            }
            true
        } else {
            false
        }
    }

    /// Number of registered profiles.
    pub fn len(&self) -> usize {
        self.profiles.len()
    }

    /// Whether there are no registered profiles.
    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }

    /// Build the default palette with common game sound profiles.
    pub fn default_palette() -> Self {
        let mut palette = Self::new();

        // "impact" — short punchy hit sound
        palette.register("impact", vec![
            SoundCommand::PlayTone {
                frequency: 150.0,
                duration: 0.1,
                volume: 0.6,
                waveform: Waveform::Square,
                attack: 0.005,
                decay: 0.08,
            },
            SoundCommand::PlayNoise {
                duration: 0.08,
                volume: 0.3,
                filter_freq: 2000.0,
            },
        ]);

        // "pickup" — bright ascending chime
        palette.register("pickup", vec![
            SoundCommand::PlayTone {
                frequency: 880.0,
                duration: 0.15,
                volume: 0.4,
                waveform: Waveform::Sine,
                attack: 0.01,
                decay: 0.12,
            },
            SoundCommand::PlayTone {
                frequency: 1320.0,
                duration: 0.15,
                volume: 0.3,
                waveform: Waveform::Sine,
                attack: 0.05,
                decay: 0.1,
            },
        ]);

        // "explosion" — low rumble with noise burst
        palette.register("explosion", vec![
            SoundCommand::PlayTone {
                frequency: 60.0,
                duration: 0.4,
                volume: 0.8,
                waveform: Waveform::Sawtooth,
                attack: 0.01,
                decay: 0.35,
            },
            SoundCommand::PlayNoise {
                duration: 0.5,
                volume: 0.7,
                filter_freq: 800.0,
            },
        ]);

        // "ui_click" — short, clean click
        palette.register("ui_click", vec![
            SoundCommand::PlayTone {
                frequency: 600.0,
                duration: 0.05,
                volume: 0.3,
                waveform: Waveform::Triangle,
                attack: 0.002,
                decay: 0.04,
            },
        ]);

        // "ambient_wind" — looping low-frequency hum
        palette.register("ambient_wind", vec![
            SoundCommand::StartLoop {
                id: "ambient_wind".to_string(),
                frequency: 80.0,
                volume: 0.15,
                waveform: Waveform::Sine,
            },
        ]);

        // "game_over" — descending ominous tone
        palette.register("game_over", vec![
            SoundCommand::PlayTone {
                frequency: 440.0,
                duration: 0.3,
                volume: 0.5,
                waveform: Waveform::Sawtooth,
                attack: 0.01,
                decay: 0.25,
            },
            SoundCommand::PlayTone {
                frequency: 220.0,
                duration: 0.5,
                volume: 0.6,
                waveform: Waveform::Sawtooth,
                attack: 0.05,
                decay: 0.4,
            },
            SoundCommand::PlayTone {
                frequency: 110.0,
                duration: 0.8,
                volume: 0.7,
                waveform: Waveform::Square,
                attack: 0.1,
                decay: 0.6,
            },
        ]);

        palette
    }
}

// ─── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_starts_empty() {
        let queue = SoundCommandQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn push_increments_len() {
        let mut queue = SoundCommandQueue::new();
        queue.push(SoundCommand::SetVolume { master_volume: 0.5 });
        assert_eq!(queue.len(), 1);
        assert!(!queue.is_empty());

        queue.push(SoundCommand::PlayNoise {
            duration: 0.1,
            volume: 0.3,
            filter_freq: 1000.0,
        });
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn drain_json_empty_returns_empty_array() {
        let mut queue = SoundCommandQueue::new();
        assert_eq!(queue.drain_json(), "[]");
    }

    #[test]
    fn drain_json_produces_valid_json_and_clears() {
        let mut queue = SoundCommandQueue::new();
        queue.push(SoundCommand::PlayTone {
            frequency: 440.0,
            duration: 0.5,
            volume: 0.8,
            waveform: Waveform::Sine,
            attack: 0.01,
            decay: 0.3,
        });
        queue.push(SoundCommand::SetVolume { master_volume: 0.7 });

        let json = queue.drain_json();

        // Queue should be empty after drain
        assert!(queue.is_empty());

        // Should parse as valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("drain_json must produce valid JSON");
        let arr = parsed.as_array().expect("should be a JSON array");
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0]["type"], "PlayTone");
        assert_eq!(arr[0]["frequency"], 440.0);
        assert_eq!(arr[0]["waveform"], "sine");
        assert_eq!(arr[1]["type"], "SetVolume");
        assert_eq!(arr[1]["master_volume"], 0.7);
    }

    #[test]
    fn drain_json_all_command_types() {
        let mut queue = SoundCommandQueue::new();

        queue.push(SoundCommand::PlayTone {
            frequency: 220.0,
            duration: 0.2,
            volume: 0.5,
            waveform: Waveform::Square,
            attack: 0.01,
            decay: 0.15,
        });
        queue.push(SoundCommand::PlayNoise {
            duration: 0.3,
            volume: 0.4,
            filter_freq: 1500.0,
        });
        queue.push(SoundCommand::StartLoop {
            id: "wind".to_string(),
            frequency: 80.0,
            volume: 0.2,
            waveform: Waveform::Triangle,
        });
        queue.push(SoundCommand::StopLoop {
            id: "wind".to_string(),
            fade_out: 0.5,
        });
        queue.push(SoundCommand::SetVolume { master_volume: 1.0 });

        let json = queue.drain_json();
        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("drain_json must produce valid JSON");
        let arr = parsed.as_array().expect("should be a JSON array");
        assert_eq!(arr.len(), 5);

        assert_eq!(arr[0]["type"], "PlayTone");
        assert_eq!(arr[0]["waveform"], "square");

        assert_eq!(arr[1]["type"], "PlayNoise");
        assert_eq!(arr[1]["filter_freq"], 1500.0);

        assert_eq!(arr[2]["type"], "StartLoop");
        assert_eq!(arr[2]["id"], "wind");
        assert_eq!(arr[2]["waveform"], "triangle");

        assert_eq!(arr[3]["type"], "StopLoop");
        assert_eq!(arr[3]["id"], "wind");
        assert_eq!(arr[3]["fade_out"], 0.5);

        assert_eq!(arr[4]["type"], "SetVolume");
    }

    #[test]
    fn default_palette_has_all_profiles() {
        let palette = SoundPalette::default_palette();
        assert!(palette.has("impact"));
        assert!(palette.has("pickup"));
        assert!(palette.has("explosion"));
        assert!(palette.has("ui_click"));
        assert!(palette.has("ambient_wind"));
        assert!(palette.has("game_over"));
        assert_eq!(palette.len(), 6);
    }

    #[test]
    fn palette_play_pushes_commands() {
        let palette = SoundPalette::default_palette();
        let mut queue = SoundCommandQueue::new();

        let found = palette.play("pickup", &mut queue);
        assert!(found);
        // "pickup" has 2 tones
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn palette_play_unknown_returns_false() {
        let palette = SoundPalette::default_palette();
        let mut queue = SoundCommandQueue::new();

        let found = palette.play("nonexistent_sound", &mut queue);
        assert!(!found);
        assert!(queue.is_empty());
    }

    #[test]
    fn palette_register_custom_profile() {
        let mut palette = SoundPalette::new();
        assert!(palette.is_empty());

        palette.register("laser", vec![
            SoundCommand::PlayTone {
                frequency: 1000.0,
                duration: 0.1,
                volume: 0.5,
                waveform: Waveform::Sawtooth,
                attack: 0.005,
                decay: 0.08,
            },
        ]);

        assert!(palette.has("laser"));
        assert_eq!(palette.len(), 1);

        let mut queue = SoundCommandQueue::new();
        palette.play("laser", &mut queue);
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn queue_clear_discards_commands() {
        let mut queue = SoundCommandQueue::new();
        queue.push(SoundCommand::SetVolume { master_volume: 0.5 });
        queue.push(SoundCommand::SetVolume { master_volume: 0.8 });
        assert_eq!(queue.len(), 2);

        queue.clear();
        assert!(queue.is_empty());
        assert_eq!(queue.drain_json(), "[]");
    }

    #[test]
    fn waveform_as_str() {
        assert_eq!(Waveform::Sine.as_str(), "sine");
        assert_eq!(Waveform::Square.as_str(), "square");
        assert_eq!(Waveform::Triangle.as_str(), "triangle");
        assert_eq!(Waveform::Sawtooth.as_str(), "sawtooth");
    }

    #[test]
    fn json_escaping_in_loop_id() {
        let mut queue = SoundCommandQueue::new();
        queue.push(SoundCommand::StartLoop {
            id: "test\"loop".to_string(),
            frequency: 100.0,
            volume: 0.5,
            waveform: Waveform::Sine,
        });

        let json = queue.drain_json();
        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("JSON with escaped characters must parse");
        let arr = parsed.as_array().expect("should be array");
        assert_eq!(arr[0]["id"], "test\"loop");
    }

    #[test]
    fn palette_default_trait_matches_default_palette() {
        let from_default: SoundPalette = Default::default();
        let from_fn = SoundPalette::default_palette();
        assert_eq!(from_default.len(), from_fn.len());
        // Both should have the same profile names
        assert!(from_default.has("impact"));
        assert!(from_default.has("game_over"));
    }
}
