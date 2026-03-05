//! Music Theory Simulation
//!
//! An interactive music theory learning app where players complete chord
//! progressions, melodies, identify intervals, and classify chord qualities.
//! Correct answers chain into a growing musical phrase played with
//! synthesized tones.

pub mod theory;
pub mod srs;
pub mod persist;

use crate::engine::Engine;
use crate::simulation::Simulation;
use crate::rendering::color::Color;
use crate::rendering::shapes;
use crate::rendering::text;
use crate::sound::{SoundCommand, Waveform};
use persist::PersistCommand;
use srs::SrsState;

use theory::{*, select_content, ContentEntry};

// ─── Layout Constants ───────────────────────────────────────────────
// Reference design: 600×900. Actual screen_w/screen_h come from the
// framebuffer at runtime.  Y-positions are stored as fractions of the
// reference height (900) and scaled by `ys = screen_h / 900.0`.

const REF_H: f64 = 900.0;

// Y-position ratios (fraction of REF_H)
const HEADER_H: f64 = 60.0;
const CHALLENGE_Y: f64 = 70.0;
const OPTIONS_Y: f64 = 310.0;
const OPTIONS_H: f64 = 70.0;
const FEEDBACK_Y: f64 = 410.0;
const FEEDBACK_H: f64 = 100.0;
const PIANO_Y: f64 = 540.0;
const WHITE_KEY_H: f64 = 130.0;
const BLACK_KEY_H: f64 = 85.0;
const TIMELINE_Y: f64 = 690.0;
const TIMELINE_H: f64 = 140.0;
const FOOTER_Y: f64 = 850.0;

// Piano geometry
const NUM_WHITE_KEYS: u8 = 15; // C3 to C5
const MIDI_LOW: u8 = 48;  // C3
const MIDI_HIGH: u8 = 72; // C5
const NUM_NOTES: usize = 25; // MIDI 48–72 inclusive

// Mobile piano (touch devices) — zoomed keys with horizontal scrolling
const MOBILE_ZOOM: f64 = 2.0;

// Timing
const FEEDBACK_CORRECT_DUR: f64 = 1.0;
const FEEDBACK_WRONG_DUR: f64 = 0.6;
const KEY_FLASH_DUR: f64 = 0.5;
const RHYTHM_ON_DUR: f64 = 0.5;
const RHYTHM_OFF_DUR: f64 = 0.3;

// Colors
const BG_COLOR: Color = Color { r: 10, g: 10, b: 30, a: 255 };
const ACCENT_TEAL: Color = Color { r: 0, g: 212, b: 170, a: 255 };
const ACCENT_CYAN: Color = Color { r: 0, g: 255, b: 204, a: 255 };
const ACCENT_PINK: Color = Color { r: 255, g: 102, b: 178, a: 255 };
const ACCENT_RED: Color = Color { r: 255, g: 80, b: 80, a: 255 };
const CORRECT_COLOR: Color = Color { r: 0, g: 255, b: 180, a: 255 };
const WRONG_COLOR: Color = Color { r: 255, g: 68, b: 102, a: 255 };
const WHITE_KEY_CLR: Color = Color { r: 232, g: 232, b: 240, a: 255 };
const BLACK_KEY_CLR: Color = Color { r: 26, g: 26, b: 46, a: 255 };
const OPTION_BG: Color = Color { r: 20, g: 30, b: 60, a: 255 };
const OPTION_HOVER: Color = Color { r: 30, g: 50, b: 90, a: 255 };
const DIM_TEXT: Color = Color { r: 100, g: 100, b: 136, a: 255 };
const OPTION_BORDER: Color = Color { r: 60, g: 70, b: 120, a: 255 };
const DIVIDER: Color = Color { r: 40, g: 40, b: 80, a: 255 };
const ACCENT_GOLD: Color = Color { r: 255, g: 215, b: 0, a: 255 };
// Piano key active states — toggled keys that produce sound
const KEY_TOGGLED: Color = Color { r: 200, g: 20, b: 20, a: 255 };     // dark red — toggled/active key

// Vowel sample names for instrument sounds
const VOWEL_SAMPLES: &[&str] = &["a", "i", "u", "e", "o"];

// Correct/wrong messages
const CORRECT_MESSAGES: &[&str] = &[
    "Correct!", "Perfect!", "Great!", "Nice!", "Well done!",
    "Excellent!", "Spot on!", "Nailed it!", "Right!",
];
const WRONG_MESSAGES: &[&str] = &[
    "Try again!", "Not quite!", "Keep trying!", "Almost!",
    "Close!", "One more try!",
];

// ─── Types ──────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub enum MusicConcept {
    ScaleDegree,          // Hear note in context → identify which degree
    RomanNumeral,         // Hear chord in context → identify which numeral
    IntervalRecognition,  // Hear two notes → name the interval
    ChordQuality,         // Hear chord → Major/Minor/Dim/Aug
    Cadence,              // Hear 2-chord cadence → Authentic/Plagal/Half/Deceptive
}

#[derive(Clone, Debug)]
pub struct MusicChallenge {
    concept: MusicConcept,
    key_root: u8,      // MIDI root of the key (e.g. 48 = C3)
    sequence: Vec<u8>, // scale degrees (chords/melody) or MIDI notes (intervals)
    answer: u8,        // correct answer (degree, semitone count, or quality index)
    options: Vec<u8>,  // 4 choices including answer
    solved: bool,
    // Chord quality challenge fields
    quality_root: u8,           // MIDI root of the played chord
    quality: Option<ChordQuality>, // the quality being tested
}

#[derive(Clone, Debug)]
struct ScheduledSound {
    play_at: f64,
    frequency: f64,
    duration: f64,
    volume: f64,
    waveform: Waveform,
    attack: f64,
    decay: f64,
}

#[derive(Clone, Debug)]
struct ScheduledSample {
    play_at: f64,
    name: String,
    volume: f64,
    pitch: f64,
    duration: f64,
}

#[derive(Clone, Debug, PartialEq)]
enum FeedbackState {
    Neutral,
    Correct,
    Wrong,
}

#[derive(Clone, Debug, Copy, PartialEq)]
enum SparkleShape {
    Circle,
    Star,
    Note,
}

#[derive(Clone, Debug)]
struct Sparkle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    life: f64,
    max_life: f64,
    color: Color,
    shape: SparkleShape,
}

#[derive(Clone, Debug)]
struct BackgroundStar {
    x: f64,
    y: f64,
    speed: f64,
    phase: f64,
    is_note: bool,
    size: f64,
}

// ─── MusicTheorySim ─────────────────────────────────────────────────

pub struct MusicTheorySim {
    streak: u8,
    max_streak: u8,
    difficulty: u8,
    challenge: MusicChallenge,
    feedback: FeedbackState,
    feedback_timer: f64,
    highlighted_keys: [bool; NUM_NOTES],
    key_flash_timers: [f64; NUM_NOTES],
    key_flash_colors: [Color; NUM_NOTES],
    pulse_time: f64,
    scheduled_sounds: Vec<ScheduledSound>,
    total_time: f64,
    challenges_completed: u32,
    sparkles: Vec<Sparkle>,
    background_stars: Vec<BackgroundStar>,
    scheduled_samples: Vec<ScheduledSample>,
    combo_pulse: f64,
    toggled_keys: [bool; NUM_NOTES],
    rhythm_timer: f64,
    rhythm_on: bool,
    last_hovered_option: Option<usize>,
    piano_scroll: f64,
    slider_dragging: bool,
    sound_energy: f64,
    screen_w: f64,
    screen_h: f64,
    current_insight: String,
    started: bool,
    // SRS state
    srs: SrsState,
    // Hint system
    hint_used: bool,
    eliminated_options: [bool; 4],
    // Content / affiliate system
    current_content: Option<&'static ContentEntry>,
    show_product: bool,
}

impl MusicTheorySim {
    pub fn new() -> Self {
        Self {
            streak: 0,
            max_streak: 0,
            difficulty: 1,
            challenge: MusicChallenge {
                concept: MusicConcept::ScaleDegree,
                key_root: 60,
                sequence: vec![],
                answer: 0,
                options: vec![],
                solved: false,
                quality_root: 60,
                quality: None,
            },
            feedback: FeedbackState::Neutral,
            feedback_timer: 0.0,
            highlighted_keys: [false; NUM_NOTES],
            key_flash_timers: [0.0; NUM_NOTES],
            key_flash_colors: [Color::WHITE; NUM_NOTES],
            pulse_time: 0.0,
            scheduled_sounds: Vec::new(),
            total_time: 0.0,
            challenges_completed: 0,
            sparkles: Vec::new(),
            background_stars: Vec::new(),
            scheduled_samples: Vec::new(),
            combo_pulse: 0.0,
            toggled_keys: [false; NUM_NOTES],
            rhythm_timer: 0.0,
            rhythm_on: false,
            last_hovered_option: None,
            piano_scroll: 0.0,
            slider_dragging: false,
            sound_energy: 0.0,
            screen_w: 600.0,
            screen_h: 900.0,
            current_insight: String::new(),
            started: false,
            srs: SrsState::new(),
            hint_used: false,
            eliminated_options: [false; 4],
            current_content: None,
            show_product: false,
        }
    }

    // ─── Computed Layout Helpers ─────────────────────────────
    /// Scale a reference-coordinate Y value to actual screen pixels.
    fn sy(&self, v: f64) -> f64 { v * self.screen_h / REF_H }
    fn ys(&self) -> f64 { self.screen_h / REF_H }
    fn white_key_w(&self) -> f64 { self.screen_w / NUM_WHITE_KEYS as f64 }
    fn black_key_w(&self) -> f64 { self.white_key_w() * 0.6 }
    fn mobile_piano_total_w(&self) -> f64 { self.screen_w * MOBILE_ZOOM }
    fn mobile_max_scroll(&self) -> f64 { self.mobile_piano_total_w() - self.screen_w }
    fn slider_track_y(&self) -> f64 { (PIANO_Y + WHITE_KEY_H + 20.0) * self.ys() }
    fn opt_btn_w(&self) -> f64 { self.screen_w * 0.2 }
    fn opt_btn_gap(&self) -> f64 { self.screen_w * 0.027 }
    fn opt_total_w(&self) -> f64 { 4.0 * self.opt_btn_w() + 3.0 * self.opt_btn_gap() }
    fn opt_x_start(&self) -> f64 { (self.screen_w - self.opt_total_w()) / 2.0 }

    // ─── Challenge Generation ────────────────────────────────

    fn generate_challenge(&mut self, engine: &mut Engine) {
        self.current_insight.clear();
        self.hint_used = false;
        self.eliminated_options = [false; 4];
        self.current_content = None;
        self.show_product = false;

        let key_roots = [48u8, 50, 52, 53, 55, 57];
        let key_root = key_roots[engine.rng.range_i32(0, key_roots.len() as i32 - 1) as usize];

        // Use SRS to pick the next card
        let seed = engine.rng.next_u64();
        if let Some((concept, variant)) = self.srs.select_next_card(seed) {
            match concept {
                0 => self.generate_chord_challenge_for(key_root, variant, engine),
                1 => self.generate_note_challenge_for(key_root, variant, engine),
                2 => self.generate_interval_challenge_for(key_root, variant, engine),
                3 => self.generate_quality_challenge_for(variant, engine),
                _ => self.generate_cadence_challenge_for(key_root, variant, engine),
            }
        } else {
            // Fallback: generate a random ScaleDegree challenge
            self.generate_chord_challenge(key_root, engine);
        }
        self.update_highlights();
    }

    /// Generate ScaleDegree challenge for a specific degree (SRS-driven).
    fn generate_chord_challenge_for(&mut self, key_root: u8, degree: u8, engine: &mut Engine) {
        // Temporarily set answer to the SRS-selected degree
        let saved_answer = degree;
        self.generate_chord_challenge_inner(key_root, saved_answer, engine);
    }

    /// Generate RomanNumeral challenge for a specific degree (SRS-driven).
    fn generate_note_challenge_for(&mut self, key_root: u8, degree: u8, engine: &mut Engine) {
        self.generate_note_challenge_inner(key_root, degree, engine);
    }

    /// Generate IntervalRecognition challenge for specific semitones (SRS-driven).
    fn generate_interval_challenge_for(&mut self, key_root: u8, semitones: u8, engine: &mut Engine) {
        self.generate_interval_challenge_inner(key_root, semitones, engine);
    }

    /// Generate ChordQuality challenge for specific quality index (SRS-driven).
    fn generate_quality_challenge_for(&mut self, quality_idx: u8, engine: &mut Engine) {
        self.generate_quality_challenge_inner(quality_idx, engine);
    }

    /// Generate Cadence challenge for specific cadence type (SRS-driven).
    fn generate_cadence_challenge_for(&mut self, key_root: u8, cadence_idx: u8, engine: &mut Engine) {
        self.generate_cadence_challenge_inner(key_root, cadence_idx, engine);
    }

    fn generate_chord_challenge(&mut self, key_root: u8, engine: &mut Engine) {
        let answer = engine.rng.range_i32(0, 6) as u8;
        self.generate_chord_challenge_inner(key_root, answer, engine);
    }

    fn generate_chord_challenge_inner(&mut self, key_root: u8, answer: u8, engine: &mut Engine) {

        let pool: Vec<u8> = if self.difficulty <= 3 {
            vec![0, 2, 4, 5] // I, iii, V, vi — easiest to distinguish
        } else if self.difficulty <= 6 {
            vec![0, 1, 3, 4, 5, 6] // add ii, IV, vii
        } else {
            (0..7).collect()
        };

        let options = generate_options(answer, &pool, 4, engine.rng.next_u64());

        // Context: ascending tonic triad (I-III-V) to establish the key
        let context_degrees: [u8; 3] = [0, 2, 4]; // scale degrees 1, 3, 5
        for (i, &deg) in context_degrees.iter().enumerate() {
            let midi = degree_to_midi(key_root, deg);
            self.schedule_sound(ScheduledSound {
                play_at: self.total_time + i as f64 * 0.3,
                frequency: midi_to_freq(midi),
                duration: 0.25,
                volume: 0.45,
                waveform: Waveform::Sine,
                attack: 0.02,
                decay: 0.2,
            });
        }

        // Mystery note after a brief pause
        let mystery_midi = degree_to_midi(key_root, answer);
        self.schedule_sound(ScheduledSound {
            play_at: self.total_time + 1.2,
            frequency: midi_to_freq(mystery_midi),
            duration: 0.5,
            volume: 0.7,
            waveform: Waveform::Triangle,
            attack: 0.01,
            decay: 0.4,
        });

        // sequence stores context degrees + answer for replay
        let seq = vec![0, 2, 4, answer];

        self.challenge = MusicChallenge {
            concept: MusicConcept::ScaleDegree,
            key_root,
            sequence: seq,
            answer,
            options,
            solved: false,
            quality_root: 60,
            quality: None,
        };
    }

    fn generate_note_challenge(&mut self, key_root: u8, engine: &mut Engine) {
        let answer = engine.rng.range_i32(0, 6) as u8;
        self.generate_note_challenge_inner(key_root, answer, engine);
    }

    fn generate_note_challenge_inner(&mut self, key_root: u8, answer: u8, engine: &mut Engine) {

        let pool: Vec<u8> = if self.difficulty <= 3 {
            vec![0, 3, 4, 5] // I, IV, V, vi — most distinct
        } else if self.difficulty <= 6 {
            vec![0, 1, 3, 4, 5]
        } else {
            (0..7).collect()
        };

        let options = generate_options(answer, &pool, 4, engine.rng.next_u64());

        // Reference: play I chord (arpeggiated)
        let i_chord = chord_intervals(0);
        for (i, &iv) in i_chord.iter().enumerate() {
            let midi = key_root + iv;
            self.schedule_sound(ScheduledSound {
                play_at: self.total_time + i as f64 * 0.15,
                frequency: midi_to_freq(midi),
                duration: 0.4,
                volume: 0.4,
                waveform: Waveform::Sine,
                attack: 0.02,
                decay: 0.3,
            });
        }

        // Mystery chord after a pause — play all notes together
        let mystery_root = key_root + MAJOR_SCALE[(answer % 7) as usize];
        let mystery_intervals = chord_intervals(answer);
        for &iv in &mystery_intervals {
            let midi = mystery_root + iv;
            self.schedule_sound(ScheduledSound {
                play_at: self.total_time + 0.8,
                frequency: midi_to_freq(midi),
                duration: 0.6,
                volume: 0.5,
                waveform: Waveform::Sine,
                attack: 0.02,
                decay: 0.5,
            });
        }
        // Also arpeggiate the mystery chord for clarity
        for (i, &iv) in mystery_intervals.iter().enumerate() {
            let midi = mystery_root + iv;
            self.schedule_sound(ScheduledSound {
                play_at: self.total_time + 1.5 + i as f64 * 0.15,
                frequency: midi_to_freq(midi),
                duration: 0.35,
                volume: 0.45,
                waveform: Waveform::Sine,
                attack: 0.02,
                decay: 0.3,
            });
        }

        // sequence = [answer degree] (for highlight/replay)
        let seq = vec![answer];

        self.challenge = MusicChallenge {
            concept: MusicConcept::RomanNumeral,
            key_root,
            sequence: seq,
            answer,
            options,
            solved: false,
            quality_root: 60,
            quality: None,
        };
    }

    fn generate_interval_challenge(&mut self, key_root: u8, engine: &mut Engine) {
        let intervals: Vec<u8> = if self.difficulty <= 4 {
            vec![0, 2, 4, 7, 12]
        } else {
            (0..=12).collect()
        };
        let idx = engine.rng.range_i32(0, intervals.len() as i32 - 1) as usize;
        let interval = intervals[idx];
        self.generate_interval_challenge_inner(key_root, interval, engine);
    }

    fn generate_interval_challenge_inner(&mut self, key_root: u8, interval: u8, engine: &mut Engine) {
        let base_offset = engine.rng.range_i32(0, 11) as u8;
        let base_midi = key_root + base_offset;
        let top_midi = base_midi + interval;

        let pool: Vec<u8> = if self.difficulty <= 4 {
            vec![0, 2, 4, 7, 12]
        } else {
            (0..=12).collect()
        };
        let options = generate_options(interval, &pool, 4, engine.rng.next_u64());

        // Play the two notes
        self.schedule_sound(ScheduledSound {
            play_at: self.total_time + 0.1,
            frequency: midi_to_freq(base_midi),
            duration: 0.4,
            volume: 0.7,
            waveform: Waveform::Sine,
            attack: 0.02,
            decay: 0.3,
        });
        self.schedule_sound(ScheduledSound {
            play_at: self.total_time + 0.6,
            frequency: midi_to_freq(top_midi),
            duration: 0.4,
            volume: 0.7,
            waveform: Waveform::Sine,
            attack: 0.02,
            decay: 0.3,
        });

        self.challenge = MusicChallenge {
            concept: MusicConcept::IntervalRecognition,
            key_root,
            sequence: vec![base_midi, top_midi],
            answer: interval,
            options,
            solved: false,
            quality_root: 60,
            quality: None,
        };
    }

    fn generate_quality_challenge(&mut self, engine: &mut Engine) {
        let qi = engine.rng.range_i32(0, CHORD_QUALITIES.len() as i32 - 1) as usize;
        self.generate_quality_challenge_inner(qi as u8, engine);
    }

    fn generate_quality_challenge_inner(&mut self, answer: u8, engine: &mut Engine) {
        let root_midi = 48 + engine.rng.range_i32(0, 12) as u8;
        let quality = CHORD_QUALITIES[answer as usize % CHORD_QUALITIES.len()];

        // Options: all 4 qualities (0=Major, 1=Minor, 2=Dim, 3=Aug)
        let pool: Vec<u8> = (0..4).collect();
        let options = generate_options(answer, &pool, 4, engine.rng.next_u64());

        // Play the chord (arpeggiated then together)
        let intervals = quality_intervals(quality);
        // Arpeggiate up
        for (i, &iv) in intervals.iter().enumerate() {
            let midi = root_midi + iv;
            self.schedule_sound(ScheduledSound {
                play_at: self.total_time + i as f64 * 0.2,
                frequency: midi_to_freq(midi),
                duration: 0.5,
                volume: 0.5,
                waveform: Waveform::Sine,
                attack: 0.02,
                decay: 0.4,
            });
        }
        // Then play all together
        for &iv in &intervals {
            let midi = root_midi + iv;
            self.schedule_sound(ScheduledSound {
                play_at: self.total_time + 0.7,
                frequency: midi_to_freq(midi),
                duration: 0.6,
                volume: 0.4,
                waveform: Waveform::Sine,
                attack: 0.02,
                decay: 0.5,
            });
        }

        self.challenge = MusicChallenge {
            concept: MusicConcept::ChordQuality,
            key_root: root_midi,
            sequence: intervals.iter().map(|&iv| root_midi + iv).collect(),
            answer,
            options,
            solved: false,
            quality_root: root_midi,
            quality: Some(quality),
        };
    }

    fn generate_cadence_challenge(&mut self, key_root: u8, engine: &mut Engine) {
        let ci = engine.rng.range_i32(0, CADENCE_TYPES.len() as i32 - 1) as usize;
        self.generate_cadence_challenge_inner(key_root, ci as u8, engine);
    }

    fn generate_cadence_challenge_inner(&mut self, key_root: u8, answer: u8, engine: &mut Engine) {
        let cadence = CADENCE_TYPES[answer as usize % CADENCE_TYPES.len()];
        let (setup_deg, resolve_deg) = cadence_chords(cadence);

        let pool: Vec<u8> = (0..4).collect();
        let options = generate_options(answer, &pool, 4, engine.rng.next_u64());

        // Context: play I chord to establish key
        let i_chord = chord_intervals(0);
        for (i, &iv) in i_chord.iter().enumerate() {
            let midi = key_root + iv;
            self.schedule_sound(ScheduledSound {
                play_at: self.total_time + i as f64 * 0.12,
                frequency: midi_to_freq(midi),
                duration: 0.5,
                volume: 0.4,
                waveform: Waveform::Sine,
                attack: 0.02,
                decay: 0.4,
            });
        }

        // Setup chord of the cadence
        let setup_root = key_root + MAJOR_SCALE[(setup_deg % 7) as usize];
        let setup_iv = chord_intervals(setup_deg);
        for &iv in &setup_iv {
            let midi = setup_root + iv;
            self.schedule_sound(ScheduledSound {
                play_at: self.total_time + 0.8,
                frequency: midi_to_freq(midi),
                duration: 0.5,
                volume: 0.5,
                waveform: Waveform::Sine,
                attack: 0.02,
                decay: 0.4,
            });
        }

        // Resolution chord of the cadence
        let resolve_root = key_root + MAJOR_SCALE[(resolve_deg % 7) as usize];
        let resolve_iv = chord_intervals(resolve_deg);
        for &iv in &resolve_iv {
            let midi = resolve_root + iv;
            self.schedule_sound(ScheduledSound {
                play_at: self.total_time + 1.5,
                frequency: midi_to_freq(midi),
                duration: 0.7,
                volume: 0.5,
                waveform: Waveform::Sine,
                attack: 0.02,
                decay: 0.6,
            });
        }

        // sequence stores [setup_deg, resolve_deg] for replay
        let seq = vec![setup_deg, resolve_deg];

        self.challenge = MusicChallenge {
            concept: MusicConcept::Cadence,
            key_root,
            sequence: seq,
            answer,
            options,
            solved: false,
            quality_root: 60,
            quality: None,
        };
    }

    fn update_highlights(&mut self) {
        self.highlighted_keys = [false; NUM_NOTES];
        match &self.challenge.concept {
            MusicConcept::ScaleDegree => {
                // Highlight the mystery note (last in sequence)
                if let Some(&deg) = self.challenge.sequence.last() {
                    let midi = degree_to_midi(self.challenge.key_root, deg);
                    if midi >= MIDI_LOW && midi <= MIDI_HIGH {
                        self.highlighted_keys[(midi - MIDI_LOW) as usize] = true;
                    }
                }
            }
            MusicConcept::RomanNumeral => {
                // Highlight the mystery chord notes
                if let Some(&deg) = self.challenge.sequence.first() {
                    let root = self.challenge.key_root + MAJOR_SCALE[(deg % 7) as usize];
                    for &interval in &chord_intervals(deg) {
                        let midi = root + interval;
                        if midi >= MIDI_LOW && midi <= MIDI_HIGH {
                            self.highlighted_keys[(midi - MIDI_LOW) as usize] = true;
                        }
                    }
                }
            }
            MusicConcept::IntervalRecognition => {
                for &midi in &self.challenge.sequence {
                    if midi >= MIDI_LOW && midi <= MIDI_HIGH {
                        self.highlighted_keys[(midi - MIDI_LOW) as usize] = true;
                    }
                }
            }
            MusicConcept::ChordQuality => {
                for &midi in &self.challenge.sequence {
                    if midi >= MIDI_LOW && midi <= MIDI_HIGH {
                        self.highlighted_keys[(midi - MIDI_LOW) as usize] = true;
                    }
                }
            }
            MusicConcept::Cadence => {
                // Highlight both cadence chords
                for &deg in &self.challenge.sequence {
                    let root = self.challenge.key_root + MAJOR_SCALE[(deg % 7) as usize];
                    for &interval in &chord_intervals(deg) {
                        let midi = root + interval;
                        if midi >= MIDI_LOW && midi <= MIDI_HIGH {
                            self.highlighted_keys[(midi - MIDI_LOW) as usize] = true;
                        }
                    }
                }
            }
        }
    }

    // ─── Input Handling ──────────────────────────────────────

    fn handle_option_click(&mut self, option_idx: usize, engine: &mut Engine) {
        if self.challenge.solved || self.feedback != FeedbackState::Neutral {
            return;
        }
        if option_idx >= self.challenge.options.len() {
            return;
        }

        let selected = self.challenge.options[option_idx];
        if selected == self.challenge.answer {
            self.on_correct(engine);
        } else {
            self.on_wrong(selected, engine);
        }
    }

    fn on_correct(&mut self, engine: &mut Engine) {
        self.challenge.solved = true;
        self.feedback = FeedbackState::Correct;
        self.feedback_timer = FEEDBACK_CORRECT_DUR;
        self.streak += 1;
        if self.streak > self.max_streak {
            self.max_streak = self.streak;
        }
        self.challenges_completed += 1;

        if self.challenges_completed % 5 == 0 && self.difficulty < 10 {
            self.difficulty += 1;
        }

        // Play the answer
        match &self.challenge.concept {
            MusicConcept::ScaleDegree => {
                let deg = self.challenge.answer;
                let midi = degree_to_midi(self.challenge.key_root, deg);
                engine.sound_queue.push(SoundCommand::PlayTone {
                    frequency: midi_to_freq(midi),
                    duration: 0.5,
                    volume: 0.7,
                    waveform: Waveform::Triangle,
                    attack: 0.01,
                    decay: 0.4,
                });
                self.flash_key(midi, CORRECT_COLOR);

                self.current_insight = degree_insight(deg).to_string();
            }
            MusicConcept::RomanNumeral => {
                let deg = self.challenge.answer;
                let root = self.challenge.key_root + MAJOR_SCALE[(deg % 7) as usize];
                let intervals = chord_intervals(deg);
                for (i, &interval) in intervals.iter().enumerate() {
                    let midi = root + interval;
                    engine.sound_queue.push(SoundCommand::PlayTone {
                        frequency: midi_to_freq(midi),
                        duration: 0.5,
                        volume: 0.5 - i as f64 * 0.1,
                        waveform: Waveform::Sine,
                        attack: 0.02,
                        decay: 0.4,
                    });
                    self.flash_key(midi, CORRECT_COLOR);
                }

                self.current_insight = numeral_insight(deg).to_string();
            }
            MusicConcept::IntervalRecognition => {
                if self.challenge.sequence.len() >= 2 {
                    let base = self.challenge.sequence[0];
                    let top = self.challenge.sequence[1];
                    engine.sound_queue.push(SoundCommand::PlayTone {
                        frequency: midi_to_freq(base),
                        duration: 0.3,
                        volume: 0.6,
                        waveform: Waveform::Sine,
                        attack: 0.02,
                        decay: 0.25,
                    });
                    self.schedule_sound(ScheduledSound {
                        play_at: self.total_time + 0.3,
                        frequency: midi_to_freq(top),
                        duration: 0.4,
                        volume: 0.6,
                        waveform: Waveform::Sine,
                        attack: 0.02,
                        decay: 0.3,
                    });
                    self.flash_key(base, CORRECT_COLOR);
                    self.flash_key(top, CORRECT_COLOR);

                    self.current_insight = interval_insight(self.challenge.answer).to_string();
                }
            }
            MusicConcept::ChordQuality => {
                // Replay the chord with correct answer flash
                let root = self.challenge.quality_root;
                if let Some(q) = self.challenge.quality {
                    let intervals = quality_intervals(q);
                    for &iv in &intervals {
                        let midi = root + iv;
                        engine.sound_queue.push(SoundCommand::PlayTone {
                            frequency: midi_to_freq(midi),
                            duration: 0.5,
                            volume: 0.45,
                            waveform: Waveform::Sine,
                            attack: 0.02,
                            decay: 0.4,
                        });
                        self.flash_key(midi, CORRECT_COLOR);
                    }
    
                    self.current_insight = quality_insight(q).to_string();
                }
            }
            MusicConcept::Cadence => {
                // Replay the cadence chords
                if self.challenge.sequence.len() >= 2 {
                    let setup_deg = self.challenge.sequence[0];
                    let resolve_deg = self.challenge.sequence[1];
                    // Flash setup chord
                    let sr = self.challenge.key_root + MAJOR_SCALE[(setup_deg % 7) as usize];
                    for &iv in &chord_intervals(setup_deg) {
                        self.flash_key(sr + iv, CORRECT_COLOR);
                    }
                    // Flash resolve chord
                    let rr = self.challenge.key_root + MAJOR_SCALE[(resolve_deg % 7) as usize];
                    for &iv in &chord_intervals(resolve_deg) {
                        engine.sound_queue.push(SoundCommand::PlayTone {
                            frequency: midi_to_freq(rr + iv),
                            duration: 0.5,
                            volume: 0.45,
                            waveform: Waveform::Sine,
                            attack: 0.02,
                            decay: 0.4,
                        });
                        self.flash_key(rr + iv, CORRECT_COLOR);
                    }

                }
                let ci = self.challenge.answer as usize;
                if ci < CADENCE_TYPES.len() {
                    self.current_insight = cadence_insight(CADENCE_TYPES[ci]).to_string();
                }
            }
        }

        // Select enriched content (fun fact + optional product) from CONTENT_DB
        let content_seed = engine.rng.next_u64();
        let concept_for_content = self.concept_idx();
        let variant_for_content = self.challenge.answer;
        if let Some(entry) = select_content(concept_for_content, variant_for_content, self.difficulty, content_seed) {
            self.current_insight = entry.fact.to_string();
            // Show product ~25% of the time when one exists
            self.show_product = entry.product.is_some()
                && engine.rng.next_u64() % 4 == 0;
            self.current_content = Some(entry);
            // Use enriched hint if available (for future hint presses on similar cards)
        } else {
            self.current_content = None;
            self.show_product = false;
            // Keep the existing insight from degree_insight/etc. as fallback
        }

        // Correct chime
        engine.sound_queue.push(SoundCommand::PlayTone {
            frequency: 1320.0,
            duration: 0.12,
            volume: 0.25,
            waveform: Waveform::Triangle,
            attack: 0.005,
            decay: 0.1,
        });

        self.combo_pulse = 1.0;

        // SRS review — determine quality
        let quality = if self.hint_used {
            2 // hint used
        } else if self.streak >= 3 {
            5 // easy (streak >= 3)
        } else {
            4 // good
        };
        let concept_idx = self.concept_idx();
        let variant = self.challenge.answer;
        self.srs.review_card(concept_idx, variant, quality);

        // Persist SRS state
        engine.persist_queue.push(PersistCommand::Store {
            key: "srs_state".to_string(),
            value: self.srs.to_json(),
        });
    }

    fn on_wrong(&mut self, _selected: u8, engine: &mut Engine) {
        self.feedback = FeedbackState::Wrong;
        self.feedback_timer = FEEDBACK_WRONG_DUR;
        self.streak = 0;

        // SRS review — fail
        let concept_idx = self.concept_idx();
        let variant = self.challenge.answer;
        self.srs.review_card(concept_idx, variant, 1);

        // Persist SRS state
        engine.persist_queue.push(PersistCommand::Store {
            key: "srs_state".to_string(),
            value: self.srs.to_json(),
        });

        // Flash wrong notes on piano
        match &self.challenge.concept {
            MusicConcept::ScaleDegree => {
                let midi = degree_to_midi(self.challenge.key_root, _selected);
                self.flash_key(midi, WRONG_COLOR);
            }
            MusicConcept::RomanNumeral => {
                let root = self.challenge.key_root + MAJOR_SCALE[(_selected % 7) as usize];
                for &interval in &chord_intervals(_selected) {
                    self.flash_key(root + interval, WRONG_COLOR);
                }
            }
            MusicConcept::ChordQuality => {
                // Flash the chord notes with wrong color
                let seq = self.challenge.sequence.clone();
                for &midi in &seq {
                    self.flash_key(midi, WRONG_COLOR);
                }
            }
            MusicConcept::IntervalRecognition => {}
            MusicConcept::Cadence => {
                // Flash cadence chord notes with wrong color
                for &deg in &self.challenge.sequence.clone() {
                    let root = self.challenge.key_root + MAJOR_SCALE[(deg % 7) as usize];
                    for &iv in &chord_intervals(deg) {
                        self.flash_key(root + iv, WRONG_COLOR);
                    }
                }
            }
        }

        // Gentle wrong sound
        engine.sound_queue.push(SoundCommand::PlayTone {
            frequency: 180.0,
            duration: 0.12,
            volume: 0.15,
            waveform: Waveform::Sine,
            attack: 0.01,
            decay: 0.1,
        });

    }

    fn activate_hint(&mut self, engine: &mut Engine) {
        if self.hint_used || self.challenge.solved {
            return;
        }
        self.hint_used = true;

        // Eliminate 2 wrong options
        let mut wrong_indices: Vec<usize> = self.challenge.options.iter().enumerate()
            .filter(|(_, &opt)| opt != self.challenge.answer)
            .map(|(i, _)| i)
            .collect();
        // Shuffle using a simple deterministic method
        let seed = self.pulse_time.to_bits();
        if wrong_indices.len() > 1 {
            let swap = (seed as usize) % wrong_indices.len();
            wrong_indices.swap(0, swap);
        }
        for &idx in wrong_indices.iter().take(2) {
            self.eliminated_options[idx] = true;
        }

        // Subtle chime
        engine.sound_queue.push(SoundCommand::PlayTone {
            frequency: 880.0,
            duration: 0.1,
            volume: 0.08,
            waveform: Waveform::Sine,
            attack: 0.005,
            decay: 0.08,
        });

        // Auto-replay the challenge
        self.replay_challenge(engine);
    }

    // ─── Helpers ─────────────────────────────────────────────

    fn flash_key(&mut self, midi: u8, color: Color) {
        if midi >= MIDI_LOW && midi <= MIDI_HIGH {
            let idx = (midi - MIDI_LOW) as usize;
            self.key_flash_timers[idx] = KEY_FLASH_DUR;
            self.key_flash_colors[idx] = color;
        }
    }

    fn schedule_sound(&mut self, sound: ScheduledSound) {
        self.scheduled_sounds.push(sound);
    }

    fn process_scheduled_sounds(&mut self, engine: &mut Engine) {
        let now = self.total_time;
        let mut i = 0;
        while i < self.scheduled_sounds.len() {
            if self.scheduled_sounds[i].play_at <= now {
                let s = self.scheduled_sounds.remove(i);
                engine.sound_queue.push(SoundCommand::PlayTone {
                    frequency: s.frequency,
                    duration: s.duration,
                    volume: s.volume,
                    waveform: s.waveform,
                    attack: s.attack,
                    decay: s.decay,
                });
            } else {
                i += 1;
            }
        }
    }

    fn midi_to_sample(midi: u8) -> (&'static str, f64) {
        let vowel_idx = (midi % 5) as usize;
        let sample = VOWEL_SAMPLES[vowel_idx];
        let pitch = 2.0_f64.powf((midi as f64 - 60.0) / 12.0);
        (sample, pitch)
    }

    fn process_scheduled_samples(&mut self, engine: &mut Engine) {
        let now = self.total_time;
        let mut i = 0;
        while i < self.scheduled_samples.len() {
            if self.scheduled_samples[i].play_at <= now {
                let s = self.scheduled_samples.remove(i);
                engine.sound_queue.push(SoundCommand::PlaySample {
                    name: s.name,
                    volume: s.volume,
                    pitch: s.pitch,
                    duration: s.duration,
                });
            } else {
                i += 1;
            }
        }
    }

    fn init_background_stars(&mut self, engine: &mut Engine) {
        self.background_stars.clear();
        for _ in 0..22 {
            self.background_stars.push(BackgroundStar {
                x: engine.rng.range_f64(0.0, self.screen_w),
                y: engine.rng.range_f64(0.0, self.screen_h),
                speed: engine.rng.range_f64(8.0, 30.0),
                phase: engine.rng.range_f64(0.0, std::f64::consts::TAU),
                is_note: engine.rng.chance(0.3),
                size: engine.rng.range_f64(1.5, 3.5),
            });
        }
    }

    fn spawn_sparkles(&mut self, cx: f64, cy: f64, count: usize, color: Color, engine: &mut Engine) {
        let shapes = [SparkleShape::Circle, SparkleShape::Star, SparkleShape::Note];
        for _ in 0..count {
            let angle = engine.rng.range_f64(0.0, std::f64::consts::TAU);
            let speed = engine.rng.range_f64(40.0, 180.0);
            let shape_idx = engine.rng.range_i32(0, shapes.len() as i32 - 1) as usize;
            let palette = [color, ACCENT_PINK, ACCENT_RED, ACCENT_TEAL];
            let color_idx = engine.rng.range_i32(0, palette.len() as i32 - 1) as usize;
            self.sparkles.push(Sparkle {
                x: cx,
                y: cy,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                life: engine.rng.range_f64(0.3, 0.8),
                max_life: 0.8,
                color: palette[color_idx],
                shape: shapes[shape_idx],
            });
        }
    }

    fn option_rect(&self, idx: usize) -> (f64, f64, f64, f64) {
        let ys = self.ys();
        let x = self.opt_x_start() + idx as f64 * (self.opt_btn_w() + self.opt_btn_gap());
        (x, OPTIONS_Y * ys, self.opt_btn_w(), OPTIONS_H * ys)
    }

    fn check_option_click(&self, mx: f64, my: f64) -> Option<usize> {
        for i in 0..self.challenge.options.len() {
            if self.eliminated_options[i] { continue; }
            let (x, y, w, h) = self.option_rect(i);
            if mx >= x && mx <= x + w && my >= y && my <= y + h {
                return Some(i);
            }
        }
        None
    }

    fn option_label(&self, value: u8) -> String {
        match &self.challenge.concept {
            MusicConcept::ScaleDegree => {
                DEGREE_NAMES[(value % 7) as usize].to_string()
            }
            MusicConcept::RomanNumeral => {
                DEGREE_NAMES[(value % 7) as usize].to_string()
            }
            MusicConcept::IntervalRecognition => {
                if (value as usize) < INTERVAL_NAMES.len() {
                    INTERVAL_NAMES[value as usize].to_string()
                } else {
                    format!("{}st", value)
                }
            }
            MusicConcept::ChordQuality => {
                if (value as usize) < CHORD_QUALITIES.len() {
                    chord_quality_name(CHORD_QUALITIES[value as usize]).to_string()
                } else {
                    "?".to_string()
                }
            }
            MusicConcept::Cadence => {
                if (value as usize) < CADENCE_TYPES.len() {
                    cadence_name(CADENCE_TYPES[value as usize]).to_string()
                } else {
                    "?".to_string()
                }
            }
        }
    }

    // ─── Piano Interaction ──────────────────────────────────

    fn check_piano_click(&self, mx: f64, my: f64, zoom: f64, scroll: f64) -> Option<u8> {
        let ys = self.ys();
        let piano_y = PIANO_Y * ys;
        let white_key_h = WHITE_KEY_H * ys;
        let black_key_h = BLACK_KEY_H * ys;
        if my < piano_y || my > piano_y + white_key_h {
            return None;
        }
        let vx = mx + scroll; // virtual X in zoomed coordinate space
        let key_w = self.white_key_w() * zoom;
        let black_w = self.black_key_w() * zoom;
        // Black keys first (rendered on top, overlap whites)
        if my < piano_y + black_key_h {
            for i in 0..NUM_WHITE_KEYS - 1 {
                let white_midi = white_key_to_midi(i);
                let black_midi = white_midi + 1;
                if !is_white_key(black_midi) && black_midi <= MIDI_HIGH {
                    let x = (i as f64 + 1.0) * key_w - black_w / 2.0;
                    if vx >= x && vx <= x + black_w {
                        return Some(black_midi);
                    }
                }
            }
        }
        // White keys
        for i in 0..NUM_WHITE_KEYS {
            let x = i as f64 * key_w;
            if vx >= x && vx <= x + key_w {
                return Some(white_key_to_midi(i));
            }
        }
        None
    }

    fn play_option_preview(&self, idx: usize, engine: &mut Engine) {
        if idx >= self.challenge.options.len() {
            return;
        }
        let opt = self.challenge.options[idx];
        match &self.challenge.concept {
            MusicConcept::ScaleDegree => {
                // Preview: play the single note for this degree
                let midi = degree_to_midi(self.challenge.key_root, opt);
                engine.sound_queue.push(SoundCommand::PlayTone {
                    frequency: midi_to_freq(midi),
                    duration: 0.3,
                    volume: 0.35,
                    waveform: Waveform::Triangle,
                    attack: 0.01,
                    decay: 0.25,
                });
            }
            MusicConcept::RomanNumeral => {
                // Preview: play the diatonic chord for this degree
                let deg = opt;
                let root = self.challenge.key_root + MAJOR_SCALE[(deg % 7) as usize];
                for &interval in &chord_intervals(deg) {
                    let midi = root + interval;
                    engine.sound_queue.push(SoundCommand::PlayTone {
                        frequency: midi_to_freq(midi),
                        duration: 0.3,
                        volume: 0.3,
                        waveform: Waveform::Sine,
                        attack: 0.02,
                        decay: 0.25,
                    });
                }
            }
            MusicConcept::IntervalRecognition => {
                if !self.challenge.sequence.is_empty() {
                    let base = self.challenge.sequence[0];
                    let top = base + opt;
                    engine.sound_queue.push(SoundCommand::PlayTone {
                        frequency: midi_to_freq(base),
                        duration: 0.2,
                        volume: 0.3,
                        waveform: Waveform::Sine,
                        attack: 0.02,
                        decay: 0.15,
                    });
                    engine.sound_queue.push(SoundCommand::PlayTone {
                        frequency: midi_to_freq(top),
                        duration: 0.3,
                        volume: 0.3,
                        waveform: Waveform::Sine,
                        attack: 0.02,
                        decay: 0.25,
                    });
                }
            }
            MusicConcept::ChordQuality => {
                // Preview: play the chord with the hovered quality
                let root = self.challenge.quality_root;
                if (opt as usize) < CHORD_QUALITIES.len() {
                    let q = CHORD_QUALITIES[opt as usize];
                    let intervals = quality_intervals(q);
                    for &iv in &intervals {
                        let midi = root + iv;
                        engine.sound_queue.push(SoundCommand::PlayTone {
                            frequency: midi_to_freq(midi),
                            duration: 0.3,
                            volume: 0.3,
                            waveform: Waveform::Sine,
                            attack: 0.02,
                            decay: 0.25,
                        });
                    }
                }
            }
            MusicConcept::Cadence => {
                // Preview: play the cadence represented by this option
                if (opt as usize) < CADENCE_TYPES.len() {
                    let c = CADENCE_TYPES[opt as usize];
                    let (setup_deg, resolve_deg) = cadence_chords(c);
                    let sr = self.challenge.key_root + MAJOR_SCALE[(setup_deg % 7) as usize];
                    for &iv in &chord_intervals(setup_deg) {
                        engine.sound_queue.push(SoundCommand::PlayTone {
                            frequency: midi_to_freq(sr + iv),
                            duration: 0.25,
                            volume: 0.3,
                            waveform: Waveform::Sine,
                            attack: 0.02,
                            decay: 0.2,
                        });
                    }
                    let rr = self.challenge.key_root + MAJOR_SCALE[(resolve_deg % 7) as usize];
                    for &iv in &chord_intervals(resolve_deg) {
                        engine.sound_queue.push(SoundCommand::PlayTone {
                            frequency: midi_to_freq(rr + iv),
                            duration: 0.3,
                            volume: 0.3,
                            waveform: Waveform::Sine,
                            attack: 0.02,
                            decay: 0.25,
                        });
                    }
                }
            }
        }
    }

    fn replay_challenge(&mut self, _engine: &mut Engine) {
        let seq = self.challenge.sequence.clone();
        let key_root = self.challenge.key_root;
        let concept = self.challenge.concept.clone();
        let now = self.total_time;
        match concept {
            MusicConcept::ScaleDegree => {
                // Replay context triad (first 3 entries) then mystery note (last)
                let context_len = seq.len().saturating_sub(1);
                for (i, &deg) in seq[..context_len].iter().enumerate() {
                    let midi = degree_to_midi(key_root, deg);
                    self.scheduled_sounds.push(ScheduledSound {
                        play_at: now + i as f64 * 0.3,
                        frequency: midi_to_freq(midi),
                        duration: 0.25,
                        volume: 0.45,
                        waveform: Waveform::Sine,
                        attack: 0.02,
                        decay: 0.2,
                    });
                }
                if let Some(&mystery_deg) = seq.last() {
                    let midi = degree_to_midi(key_root, mystery_deg);
                    self.scheduled_sounds.push(ScheduledSound {
                        play_at: now + 1.2,
                        frequency: midi_to_freq(midi),
                        duration: 0.5,
                        volume: 0.7,
                        waveform: Waveform::Triangle,
                        attack: 0.01,
                        decay: 0.4,
                    });
                }
            }
            MusicConcept::RomanNumeral => {
                // Replay I chord then mystery chord
                let i_chord = chord_intervals(0);
                for (i, &iv) in i_chord.iter().enumerate() {
                    let midi = key_root + iv;
                    self.scheduled_sounds.push(ScheduledSound {
                        play_at: now + i as f64 * 0.15,
                        frequency: midi_to_freq(midi),
                        duration: 0.4,
                        volume: 0.4,
                        waveform: Waveform::Sine,
                        attack: 0.02,
                        decay: 0.3,
                    });
                }
                if let Some(&deg) = seq.first() {
                    let mystery_root = key_root + MAJOR_SCALE[(deg % 7) as usize];
                    let mystery_iv = chord_intervals(deg);
                    for &iv in &mystery_iv {
                        let midi = mystery_root + iv;
                        self.scheduled_sounds.push(ScheduledSound {
                            play_at: now + 0.8,
                            frequency: midi_to_freq(midi),
                            duration: 0.6,
                            volume: 0.5,
                            waveform: Waveform::Sine,
                            attack: 0.02,
                            decay: 0.5,
                        });
                    }
                    for (i, &iv) in mystery_iv.iter().enumerate() {
                        let midi = mystery_root + iv;
                        self.scheduled_sounds.push(ScheduledSound {
                            play_at: now + 1.5 + i as f64 * 0.15,
                            frequency: midi_to_freq(midi),
                            duration: 0.35,
                            volume: 0.45,
                            waveform: Waveform::Sine,
                            attack: 0.02,
                            decay: 0.3,
                        });
                    }
                }
            }
            MusicConcept::IntervalRecognition => {
                if seq.len() >= 2 {
                    self.scheduled_sounds.push(ScheduledSound {
                        play_at: now + 0.1,
                        frequency: midi_to_freq(seq[0]),
                        duration: 0.4,
                        volume: 0.7,
                        waveform: Waveform::Sine,
                        attack: 0.02,
                        decay: 0.3,
                    });
                    self.scheduled_sounds.push(ScheduledSound {
                        play_at: now + 0.6,
                        frequency: midi_to_freq(seq[1]),
                        duration: 0.4,
                        volume: 0.7,
                        waveform: Waveform::Sine,
                        attack: 0.02,
                        decay: 0.3,
                    });
                }
            }
            MusicConcept::ChordQuality => {
                // Replay: arpeggiate then chord
                let root = self.challenge.quality_root;
                if let Some(q) = self.challenge.quality {
                    let intervals = quality_intervals(q);
                    for (i, &iv) in intervals.iter().enumerate() {
                        let midi = root + iv;
                        self.scheduled_sounds.push(ScheduledSound {
                            play_at: now + i as f64 * 0.2,
                            frequency: midi_to_freq(midi),
                            duration: 0.5,
                            volume: 0.5,
                            waveform: Waveform::Sine,
                            attack: 0.02,
                            decay: 0.4,
                        });
                    }
                    for &iv in &intervals {
                        let midi = root + iv;
                        self.scheduled_sounds.push(ScheduledSound {
                            play_at: now + 0.7,
                            frequency: midi_to_freq(midi),
                            duration: 0.6,
                            volume: 0.4,
                            waveform: Waveform::Sine,
                            attack: 0.02,
                            decay: 0.5,
                        });
                    }
                }
            }
            MusicConcept::Cadence => {
                // Replay: I chord context, then cadence (setup → resolve)
                let i_chord = chord_intervals(0);
                for (i, &iv) in i_chord.iter().enumerate() {
                    let midi = key_root + iv;
                    self.scheduled_sounds.push(ScheduledSound {
                        play_at: now + i as f64 * 0.12,
                        frequency: midi_to_freq(midi),
                        duration: 0.5,
                        volume: 0.4,
                        waveform: Waveform::Sine,
                        attack: 0.02,
                        decay: 0.4,
                    });
                }
                if seq.len() >= 2 {
                    let setup_root = key_root + MAJOR_SCALE[(seq[0] % 7) as usize];
                    for &iv in &chord_intervals(seq[0]) {
                        self.scheduled_sounds.push(ScheduledSound {
                            play_at: now + 0.8,
                            frequency: midi_to_freq(setup_root + iv),
                            duration: 0.5,
                            volume: 0.5,
                            waveform: Waveform::Sine,
                            attack: 0.02,
                            decay: 0.4,
                        });
                    }
                    let resolve_root = key_root + MAJOR_SCALE[(seq[1] % 7) as usize];
                    for &iv in &chord_intervals(seq[1]) {
                        self.scheduled_sounds.push(ScheduledSound {
                            play_at: now + 1.5,
                            frequency: midi_to_freq(resolve_root + iv),
                            duration: 0.7,
                            volume: 0.5,
                            waveform: Waveform::Sine,
                            attack: 0.02,
                            decay: 0.6,
                        });
                    }
                }
            }
        }
    }

    /// Get the concept index for learning resource lookup.
    fn concept_idx(&self) -> u8 {
        match &self.challenge.concept {
            MusicConcept::ScaleDegree => 0,
            MusicConcept::RomanNumeral => 1,
            MusicConcept::IntervalRecognition => 2,
            MusicConcept::ChordQuality => 3,
            MusicConcept::Cadence => 4,
        }
    }
}

// ─── Simulation Trait ───────────────────────────────────────────────

impl Simulation for MusicTheorySim {
    fn setup(&mut self, engine: &mut Engine) {
        self.screen_w = engine.framebuffer.width as f64;
        self.screen_h = engine.framebuffer.height as f64;
        engine.config.bounds = (self.screen_w, self.screen_h);
        engine.config.background = BG_COLOR;
        self.init_background_stars(engine);

        // Restore SRS state from persisted data (set by JS before setup)
        if let Some(json) = engine.global_state.get_str("srs_state") {
            if let Some(restored) = SrsState::from_json(json) {
                self.srs = restored;
            }
        }

        engine.global_state.set_f64("streak", 0.0);
        engine.global_state.set_f64("difficulty", 1.0);
    }

    fn step(&mut self, engine: &mut Engine) {
        let dt = 1.0 / 60.0;
        self.total_time += dt;
        self.pulse_time += dt;

        // "Tap to Begin" gate — wait for first tap so AudioContext is initialized
        if !self.started {
            if engine.input.mouse_buttons_pressed.contains(&0)
                || !engine.input.keys_pressed.is_empty()
            {
                self.started = true;
                self.generate_challenge(engine);
            }
            // Still update background stars while waiting
            for star in self.background_stars.iter_mut() {
                star.y -= star.speed * dt;
                if star.y < -10.0 {
                    star.y = self.screen_h + 10.0;
                }
            }
            return;
        }

        // Process scheduled sounds and samples
        let sound_count_before = engine.sound_queue.len();
        self.process_scheduled_sounds(engine);
        self.process_scheduled_samples(engine);

        // Decay sound energy smoothly; spike on new sounds
        self.sound_energy = (self.sound_energy - dt * 2.5).max(0.0);

        // Update combo pulse
        if self.combo_pulse > 0.0 {
            self.combo_pulse = (self.combo_pulse - dt * 3.0).max(0.0);
        }

        // Update background stars
        for star in self.background_stars.iter_mut() {
            star.y -= star.speed * dt;
            if star.y < -10.0 {
                star.y = self.screen_h + 10.0;
            }
        }

        // Update feedback timer — only auto-dismiss Wrong; Correct stays until "Next"
        if self.feedback_timer > 0.0 && self.feedback == FeedbackState::Wrong {
            self.feedback_timer -= dt;
            if self.feedback_timer <= 0.0 {
                self.feedback = FeedbackState::Neutral;
                self.feedback_timer = 0.0;
            }
        }

        // Update key flash timers
        for t in self.key_flash_timers.iter_mut() {
            if *t > 0.0 {
                *t = (*t - dt).max(0.0);
            }
        }

        // Update sparkles
        let mut i = 0;
        while i < self.sparkles.len() {
            self.sparkles[i].life -= dt;
            if self.sparkles[i].life <= 0.0 {
                self.sparkles.swap_remove(i);
            } else {
                self.sparkles[i].x += self.sparkles[i].vx * dt;
                self.sparkles[i].y += self.sparkles[i].vy * dt;
                self.sparkles[i].vy += 120.0 * dt; // gravity
                self.sparkles[i].vx *= 0.98;
                i += 1;
            }
        }

        // Handle mouse click on options and piano
        let is_mobile = engine.browser_state.is_touch_device();
        let mx = engine.input.mouse_x;
        let my = engine.input.mouse_y;

        // Mobile slider dragging — generous 60px touch zone centered on slider
        if is_mobile {
            let slider_center = self.slider_track_y() + 3.0;
            let slider_hit_top = slider_center - 30.0;
            let slider_hit_bot = slider_center + 30.0;
            if engine.input.mouse_buttons_pressed.contains(&0)
                && my > slider_hit_top && my < slider_hit_bot
            {
                self.slider_dragging = true;
            }
            if self.slider_dragging {
                if engine.input.mouse_buttons_held.contains(&0) {
                    let ratio = (mx / self.screen_w).max(0.0).min(1.0);
                    self.piano_scroll = ratio * self.mobile_max_scroll();
                }
                if engine.input.mouse_buttons_released.contains(&0) {
                    self.slider_dragging = false;
                }
            }
        }

        if engine.input.mouse_buttons_pressed.contains(&0) && !self.slider_dragging {
            // "Next" text tap zone — advance after correct feedback
            if self.feedback == FeedbackState::Correct {
                let btn_w = 120.0;
                let btn_h = 40.0;
                let btn_x = (self.screen_w - btn_w) / 2.0;
                let btn_y = self.sy(PIANO_Y - 55.0);
                if mx >= btn_x && mx <= btn_x + btn_w && my >= btn_y && my <= btn_y + btn_h {
                    self.feedback = FeedbackState::Neutral;
                    self.feedback_timer = 0.0;
                    self.generate_challenge(engine);
                }

                // Affiliate product tap zone (product name area)
                if self.show_product {
                    if let Some(entry) = self.current_content {
                        if let Some(product) = &entry.product {
                            // Match the layout from render_feedback:
                            // Insight capped at 4 lines when product shown
                            let max_w_px = self.screen_w as i32 - 40;
                            let all_lines = wrap_text(&self.current_insight, max_w_px, 2);
                            let n_lines = all_lines.len().min(4);
                            let start_y = self.sy(OPTIONS_Y + 40.0);
                            let prod_top = start_y + (n_lines as f64) * 18.0 + 4.0;
                            let prod_bot = prod_top + 52.0; // blurb + name + disclosure
                            // Full-width tap zone
                            if my >= prod_top && my <= prod_bot {
                                engine.persist_queue.push(PersistCommand::OpenUrl {
                                    url: product.url.to_string(),
                                });
                            }
                        }
                    }
                }
            }

            // "Replay" and "Hint" tap zones (in challenge area)
            if !self.challenge.solved && self.feedback == FeedbackState::Neutral {
                let cx = self.screen_w / 2.0;
                let btn_w = 80.0;
                let btn_h = 30.0;
                let btn_y = self.sy(CHALLENGE_Y + 188.0);
                // Replay button (left)
                let replay_x = cx - 50.0 - btn_w / 2.0;
                if mx >= replay_x && mx <= replay_x + btn_w && my >= btn_y && my <= btn_y + btn_h {
                    self.replay_challenge(engine);
                }
                // Hint button (right)
                if !self.hint_used {
                    let hint_x = cx + 50.0 - btn_w / 2.0;
                    if mx >= hint_x && mx <= hint_x + btn_w && my >= btn_y && my <= btn_y + btn_h {
                        self.activate_hint(engine);
                    }
                }
            }

            // Replay zone tap (animated wave icon area)
            if !self.challenge.solved {
                let zone_y = self.sy(TIMELINE_Y);
                let zone_h = self.sy(80.0);
                if my >= zone_y && my <= zone_y + zone_h {
                    self.replay_challenge(engine);
                }
            }

            if let Some(idx) = self.check_option_click(mx, my) {
                self.handle_option_click(idx, engine);
            }
            // Piano click → toggle key (zoom-aware)
            let (zoom, scroll) = if is_mobile {
                (MOBILE_ZOOM, self.piano_scroll)
            } else {
                (1.0, 0.0)
            };
            if let Some(midi) = self.check_piano_click(mx, my, zoom, scroll) {
                let idx = (midi - MIDI_LOW) as usize;
                if idx < NUM_NOTES {
                    self.toggled_keys[idx] = !self.toggled_keys[idx];
                    if self.toggled_keys[idx] {
                        let (sample, pitch) = Self::midi_to_sample(midi);
                        engine.sound_queue.push(SoundCommand::PlaySample {
                            name: sample.to_string(),
                            volume: 0.5,
                            pitch,
                            duration: 0.4,
                        });
                        self.flash_key(midi, ACCENT_PINK);
                    }
                }
            }
        }

        // Rhythm loop for toggled keys
        let any_toggled = self.toggled_keys.iter().any(|&t| t);
        if any_toggled {
            self.rhythm_timer += dt;
            let cycle = RHYTHM_ON_DUR + RHYTHM_OFF_DUR;
            let phase = self.rhythm_timer % cycle;
            let now_on = phase < RHYTHM_ON_DUR;
            if now_on && !self.rhythm_on {
                // Spike sound energy so background stars pulse with the beat
                let toggled_count = self.toggled_keys.iter().filter(|&&t| t).count();
                self.sound_energy = (self.sound_energy + 0.3 + toggled_count as f64 * 0.1).min(1.0);
                // Transition to ON — play all toggled keys
                for k in 0..NUM_NOTES {
                    if self.toggled_keys[k] {
                        let midi = MIDI_LOW + k as u8;
                        let (sample, pitch) = Self::midi_to_sample(midi);
                        engine.sound_queue.push(SoundCommand::PlaySample {
                            name: sample.to_string(),
                            volume: 0.45,
                            pitch,
                            duration: RHYTHM_ON_DUR * 0.8,
                        });
                    }
                }
            }
            self.rhythm_on = now_on;
        } else {
            self.rhythm_timer = 0.0;
            self.rhythm_on = false;
        }

        // Option hover preview (uses unified hover — works on desktop + touch)
        // On mobile, skip hover preview when tapping to avoid overlapping sounds
        let skip_hover_preview = is_mobile && engine.input.mouse_buttons_pressed.contains(&0);
        let hx = engine.input.hover_x;
        let hy = engine.input.hover_y;
        let hovered = if engine.input.hover_active && !skip_hover_preview {
            self.check_option_click(hx, hy)
        } else {
            None
        };
        if hovered != self.last_hovered_option
            && self.feedback == FeedbackState::Neutral
            && !self.challenge.solved
        {
            if let Some(idx) = hovered {
                self.play_option_preview(idx, engine);
            }
            self.last_hovered_option = hovered;
        }
        // Clear stale hover state after mobile tap
        if skip_hover_preview {
            self.last_hovered_option = None;
        }

        // Handle keyboard shortcuts (1-4)
        if engine.input.keys_pressed.contains("Digit1") {
            self.handle_option_click(0, engine);
        } else if engine.input.keys_pressed.contains("Digit2") {
            self.handle_option_click(1, engine);
        } else if engine.input.keys_pressed.contains("Digit3") {
            self.handle_option_click(2, engine);
        } else if engine.input.keys_pressed.contains("Digit4") {
            self.handle_option_click(3, engine);
        }

        // "Next" shortcut (Space or Enter) when showing correct feedback
        if self.feedback == FeedbackState::Correct
            && (engine.input.keys_pressed.contains("Space")
                || engine.input.keys_pressed.contains("Enter"))
        {
            self.feedback = FeedbackState::Neutral;
            self.feedback_timer = 0.0;
            self.generate_challenge(engine);
        }
        // Replay challenge (Space or R) — only when neutral and unsolved
        else if !self.challenge.solved && self.feedback == FeedbackState::Neutral
            && (engine.input.keys_pressed.contains("Space")
                || engine.input.keys_pressed.contains("KeyR"))
        {
            self.replay_challenge(engine);
        }

        // Hint shortcut (H)
        if engine.input.keys_pressed.contains("KeyH")
            && self.feedback == FeedbackState::Neutral
            && !self.challenge.solved
        {
            self.activate_hint(engine);
        }

        // Spike sound energy based on new sounds queued this frame
        let new_sounds = engine.sound_queue.len().saturating_sub(sound_count_before);
        if new_sounds > 0 {
            self.sound_energy = (self.sound_energy + new_sounds as f64 * 0.25).min(1.0);
        }

        // Export state for JS HUD
        engine.global_state.set_f64("streak", self.streak as f64);
        engine.global_state.set_f64("difficulty", self.difficulty as f64);
        engine.global_state.set_f64("challenges_completed", self.challenges_completed as f64);
    }

    fn render(&self, engine: &mut Engine) {
        let fb = &mut engine.framebuffer;

        // "Tap to Begin" screen
        if !self.started {
            self.render_background_stars(fb);
            self.render_border_glow(fb);
            let cx = (self.screen_w / 2.0) as i32;
            let cy = (self.screen_h / 2.0) as i32;
            text::draw_text_centered(fb, cx, cy - 40, "MUSIC THEORY", ACCENT_TEAL, 3);
            text::draw_text_centered(fb, cx, cy, "Interactive Ear Training", ACCENT_PINK, 2);
            let blink = ((self.pulse_time * 2.5).sin() * 0.3 + 0.7) * 255.0;
            text::draw_text_centered(fb, cx, cy + 50, "Tap to Begin",
                Color::WHITE.with_alpha(blink as u8), 2);
            return;
        }

        // Use hover position for visual hover effects (works on desktop + touch)
        let hx = if engine.input.hover_active { engine.input.hover_x } else { -1000.0 };
        let hy = if engine.input.hover_active { engine.input.hover_y } else { -1000.0 };
        let is_mobile = engine.browser_state.is_touch_device();

        self.render_background_stars(fb);
        self.render_border_glow(fb);
        self.render_header(fb);
        self.render_challenge(fb);
        self.render_combo(fb);
        self.render_options(fb, hx, hy);
        self.render_feedback(fb);
        self.render_piano(fb, is_mobile);
        self.render_replay_zone(fb);
        self.render_footer(fb);
        self.render_sparkles(fb);
    }
}

// ─── Rendering ──────────────────────────────────────────────────────

impl MusicTheorySim {
    fn render_header(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let ys = self.ys();
        text::draw_text(fb, 16, self.sy(14.0) as i32, "MUSIC THEORY", ACCENT_TEAL, 2);
        let sub = "Interactive Ear Training";
        text::draw_text(fb, 16, self.sy(38.0) as i32, sub, ACCENT_PINK, 1);

        // SRS stats (right-aligned)
        let due = self.srs.due_count();
        let due_str = format!("{} due", due);
        let dw = text::text_width(&due_str, 1);
        text::draw_text(fb, self.screen_w as i32 - dw - 16, self.sy(14.0) as i32,
            &due_str, ACCENT_TEAL, 1);

        let learned = self.srs.total_seen();
        let learned_str = format!("{} learned", learned);
        let lw = text::text_width(&learned_str, 1);
        text::draw_text(fb, self.screen_w as i32 - lw - 16, self.sy(30.0) as i32,
            &learned_str, DIM_TEXT, 1);

        shapes::draw_line(fb, 0.0, HEADER_H * ys - 1.0, self.screen_w, HEADER_H * ys - 1.0, DIVIDER);
    }

    fn render_challenge(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let cx = (self.screen_w / 2.0) as i32;
        let cy = |off: f64| -> i32 { self.sy(CHALLENGE_Y + off) as i32 };

        match &self.challenge.concept {
            MusicConcept::ScaleDegree => {
                text::draw_text_centered(fb, cx, cy(12.0), "SCALE DEGREE", DIM_TEXT, 1);
                text::draw_text_centered(fb, cx, cy(36.0), "Which degree is this note?", Color::WHITE, 2);

                // Show the mystery note name
                if let Some(&deg) = self.challenge.sequence.last() {
                    let midi = degree_to_midi(self.challenge.key_root, deg);
                    let display = format!("{}  -->  ?", note_name(midi));
                    text::draw_text_centered(fb, cx, cy(90.0), &display, ACCENT_TEAL, 3);
                }

                let key_str = format!("Key: {} Major", note_name(self.challenge.key_root));
                text::draw_text_centered(fb, cx, cy(150.0), &key_str, DIM_TEXT, 1);
                text::draw_text_centered(fb, cx, cy(170.0), "Listen: tonic triad, then mystery note", DIM_TEXT, 1);
            }
            MusicConcept::RomanNumeral => {
                text::draw_text_centered(fb, cx, cy(12.0), "ROMAN NUMERAL", DIM_TEXT, 1);
                text::draw_text_centered(fb, cx, cy(36.0), "Identify this chord", Color::WHITE, 2);

                // Show a ? for the mystery chord
                let display = "I  -->  ?";
                text::draw_text_centered(fb, cx, cy(90.0), display, ACCENT_TEAL, 3);

                let key_str = format!("Key: {} Major", note_name(self.challenge.key_root));
                text::draw_text_centered(fb, cx, cy(150.0), &key_str, DIM_TEXT, 1);
                text::draw_text_centered(fb, cx, cy(170.0), "Listen: I chord, then mystery chord", DIM_TEXT, 1);
            }
            MusicConcept::IntervalRecognition => {
                text::draw_text_centered(fb, cx, cy(12.0), "INTERVAL", DIM_TEXT, 1);
                text::draw_text_centered(fb, cx, cy(36.0), "Name this interval", Color::WHITE, 2);

                if self.challenge.sequence.len() >= 2 {
                    let note1 = note_name(self.challenge.sequence[0]);
                    let note2 = note_name(self.challenge.sequence[1]);
                    let display = format!("{} --> {}", note1, note2);
                    text::draw_text_centered(fb, cx, cy(90.0), &display, ACCENT_TEAL, 3);
                }
            }
            MusicConcept::ChordQuality => {
                text::draw_text_centered(fb, cx, cy(12.0), "CHORD QUALITY", DIM_TEXT, 1);
                text::draw_text_centered(fb, cx, cy(36.0), "What type of triad is this?", Color::WHITE, 2);

                let root_name = note_name(self.challenge.quality_root);
                let display = format!("{} triad", root_name);
                text::draw_text_centered(fb, cx, cy(90.0), &display, ACCENT_TEAL, 3);

                text::draw_text_centered(fb, cx, cy(150.0), "Listen: arpeggiated then together", DIM_TEXT, 1);
            }
            MusicConcept::Cadence => {
                text::draw_text_centered(fb, cx, cy(12.0), "CADENCE", DIM_TEXT, 1);
                text::draw_text_centered(fb, cx, cy(36.0), "What type of cadence is this?", Color::WHITE, 2);

                // Show the two chords as Roman numerals
                if self.challenge.sequence.len() >= 2 {
                    let s = DEGREE_NAMES[(self.challenge.sequence[0] % 7) as usize];
                    let r = DEGREE_NAMES[(self.challenge.sequence[1] % 7) as usize];
                    let display = format!("{}  -->  {}", s, r);
                    text::draw_text_centered(fb, cx, cy(90.0), &display, ACCENT_TEAL, 3);
                }

                let key_str = format!("Key: {} Major", note_name(self.challenge.key_root));
                text::draw_text_centered(fb, cx, cy(150.0), &key_str, DIM_TEXT, 1);
                text::draw_text_centered(fb, cx, cy(170.0), "Listen: I chord, then two-chord cadence", DIM_TEXT, 1);
            }
        }

        // Replay + Hint buttons — small tappable text below challenge info
        if !self.challenge.solved && self.feedback == FeedbackState::Neutral {
            let btn_y = self.sy(CHALLENGE_Y + 195.0) as i32;
            let blink = ((self.pulse_time * 2.0).sin() * 0.2 + 0.8) * 255.0;
            // Replay button (left of center)
            text::draw_text_centered(fb, cx - 50, btn_y,
                "[ Replay ]", ACCENT_TEAL.with_alpha(blink as u8), 1);
            // Hint button (right of center)
            if !self.hint_used {
                let hint_color = Color::from_rgba(180, 160, 100, (blink * 0.8) as u8);
                text::draw_text_centered(fb, cx + 50, btn_y,
                    "[ Hint ]", hint_color, 1);
            } else {
                text::draw_text_centered(fb, cx + 50, btn_y,
                    "[ Hint ]", DIM_TEXT.with_alpha(80), 1);
            }

            // Show content hint text when hint was used
            if self.hint_used {
                let concept_for_hint = self.concept_idx();
                let variant_for_hint = self.challenge.answer;
                // Try to find a content entry with a hint for this card
                if let Some(entry) = select_content(concept_for_hint, variant_for_hint, self.difficulty, 0) {
                    if let Some(hint_text) = entry.hint {
                        let hint_y = btn_y + 18;
                        let max_w = self.screen_w as i32 - 60;
                        let lines = wrap_text(hint_text, max_w, 1);
                        // Cap at 2 lines to avoid overlapping the options zone
                        for (li, line) in lines.iter().take(2).enumerate() {
                            text::draw_text_centered(fb, cx, hint_y + (li as i32) * 12, line,
                                ACCENT_GOLD.with_alpha(180), 1);
                        }
                    }
                }
            }
        }
    }

    fn render_options(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, mx: f64, my: f64) {
        // Hide options during correct feedback — that space shows the insight text
        if self.feedback == FeedbackState::Correct {
            return;
        }
        for (i, &opt) in self.challenge.options.iter().enumerate() {
            let (x, y, w, h) = self.option_rect(i);

            if self.eliminated_options[i] {
                // Eliminated option: very dark background, ghosted text, diagonal strikethrough
                shapes::fill_rect(fb, x, y, w, h, Color::from_rgba(15, 15, 25, 255));
                shapes::draw_rect(fb, x, y, w, h, Color::from_rgba(30, 30, 50, 255));
                let label = self.option_label(opt);
                text::draw_text_centered(fb, (x + w / 2.0) as i32, (y + h / 2.0 - 5.0) as i32,
                    &label, Color::from_rgba(50, 50, 70, 255), 2);
                // Diagonal strikethrough
                shapes::draw_line(fb, x + 4.0, y + h - 4.0, x + w - 4.0, y + 4.0,
                    Color::from_rgba(80, 40, 40, 180));
                continue;
            }

            let bg = if self.challenge.solved && opt == self.challenge.answer {
                CORRECT_COLOR
            } else if mx >= x && mx <= x + w && my >= y && my <= y + h
                && self.feedback == FeedbackState::Neutral && !self.challenge.solved
            {
                OPTION_HOVER
            } else {
                OPTION_BG
            };

            shapes::fill_rect(fb, x, y, w, h, bg);

            let border_color = if self.challenge.solved && opt == self.challenge.answer {
                ACCENT_CYAN
            } else {
                OPTION_BORDER
            };
            shapes::draw_rect(fb, x, y, w, h, border_color);

            let label = self.option_label(opt);
            let text_color = if self.challenge.solved && opt == self.challenge.answer {
                Color::from_rgba(0, 30, 20, 255)
            } else {
                Color::WHITE
            };
            text::draw_text_centered(fb, (x + w / 2.0) as i32, (y + h / 2.0 - 5.0) as i32,
                &label, text_color, 2);

            // Number hint
            let num = format!("{}", i + 1);
            text::draw_text(fb, (x + 4.0) as i32, (y + 4.0) as i32, &num, DIM_TEXT, 1);
        }
    }

    fn render_feedback(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let cx = (self.screen_w / 2.0) as i32;
        let char_y = self.sy(FEEDBACK_Y);
        let char_h = self.sy(FEEDBACK_H);
        let cy = (char_y + char_h / 2.0) as i32;

        let (msg, color) = match &self.feedback {
            FeedbackState::Correct => {
                let idx = ((self.pulse_time * 2.0) as usize) % CORRECT_MESSAGES.len();
                (CORRECT_MESSAGES[idx], CORRECT_COLOR)
            }
            FeedbackState::Wrong => {
                let idx = ((self.pulse_time * 2.0) as usize) % WRONG_MESSAGES.len();
                (WRONG_MESSAGES[idx], WRONG_COLOR)
            }
            FeedbackState::Neutral => ("", ACCENT_TEAL),
        };

        if self.feedback == FeedbackState::Correct {
            // Options are hidden — use the full OPTIONS_Y..PIANO_Y zone for layout.
            // "Correct!" at top of options zone
            let msg_y = self.sy(OPTIONS_Y + 10.0) as i32;
            text::draw_text_centered(fb, cx, msg_y, msg, color, 2);

            // Insight text below the message
            if !self.current_insight.is_empty() {
                let max_w = self.screen_w as i32 - 40;
                let all_lines = wrap_text(&self.current_insight, max_w, 2);
                let line_h = 18;
                let start_y = self.sy(OPTIONS_Y + 40.0) as i32;
                // Limit insight lines when product is shown to avoid overflow
                let max_lines = if self.show_product { 4 } else { 8 };
                let lines: Vec<_> = all_lines.into_iter().take(max_lines).collect();
                for (i, line) in lines.iter().enumerate() {
                    let alpha = if i == 0 { 220u8 } else { 180 };
                    text::draw_text_centered(fb, cx, start_y + (i as i32) * line_h, line,
                        ACCENT_TEAL.with_alpha(alpha), 2);
                }

                // Affiliate recommendation (below insight, above [ Next ])
                if self.show_product {
                    if let Some(entry) = self.current_content {
                        if let Some(product) = &entry.product {
                            let prod_y = start_y + (lines.len() as i32) * line_h + 10;

                            // Blurb line (scale 1, gold)
                            text::draw_text_centered(fb, cx, prod_y,
                                product.blurb, ACCENT_GOLD, 1);

                            // Product name — use scale 1 on narrow screens
                            let name_scale = if self.screen_w < 500.0 { 1 } else { 2 };
                            let name_y = prod_y + 14;
                            text::draw_text_centered(fb, cx, name_y,
                                product.name, ACCENT_CYAN, name_scale);

                            // FTC disclosure (scale 1, dim)
                            let disc_y = name_y + if name_scale == 2 { 20 } else { 12 };
                            text::draw_text_centered(fb, cx, disc_y,
                                product.disclosure, DIM_TEXT, 1);
                        }
                    }
                }
            }

            // "[ Next ]" anchored above the piano
            let next_y = self.sy(PIANO_Y - 40.0) as i32;
            let blink = ((self.pulse_time * 2.0).sin() * 0.2 + 0.8) * 255.0;
            text::draw_text_centered(fb, cx, next_y,
                "[ Next ]", CORRECT_COLOR.with_alpha(blink as u8), 2);
        } else if !msg.is_empty() {
            // Wrong / Neutral — render in the normal feedback zone
            text::draw_text_centered(fb, cx, cy, msg, color, 2);
        }

        // Decorative accents
        let bob = (self.pulse_time * 2.0).sin() * 4.0;
        let sparkle_alpha = ((self.pulse_time * 3.0).sin() * 0.3 + 0.5) * 255.0;
        shapes::fill_circle(fb, 80.0, char_y + 20.0 + bob,
            2.0, ACCENT_PINK.with_alpha(sparkle_alpha as u8));
        shapes::fill_circle(fb, self.screen_w - 80.0, char_y + 30.0 - bob,
            2.0, ACCENT_TEAL.with_alpha(sparkle_alpha as u8));
    }

    fn render_piano(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, is_mobile: bool) {
        let piano_y = self.sy(PIANO_Y);
        let wk_h = self.sy(WHITE_KEY_H);
        let bk_h = self.sy(BLACK_KEY_H);

        let (zoom, scroll) = if is_mobile {
            (MOBILE_ZOOM, self.piano_scroll)
        } else {
            (1.0, 0.0)
        };
        let key_w = self.white_key_w() * zoom;
        let black_w = self.black_key_w() * zoom;

        // White keys
        for i in 0..NUM_WHITE_KEYS {
            let midi = white_key_to_midi(i);
            let note_idx = (midi - MIDI_LOW) as usize;
            let x = i as f64 * key_w - scroll;
            if x + key_w < 0.0 || x > self.screen_w { continue; }

            let mut color = WHITE_KEY_CLR;
            if self.highlighted_keys[note_idx] {
                color = Color::from_rgba(120, 220, 200, 255);
            }
            if self.toggled_keys[note_idx] {
                color = KEY_TOGGLED;
                shapes::fill_rect(fb, x - 1.0, piano_y - 2.0, key_w + 2.0, wk_h + 4.0,
                    KEY_TOGGLED.with_alpha(80));
            }
            if self.key_flash_timers[note_idx] > 0.0 {
                let t = self.key_flash_timers[note_idx] / KEY_FLASH_DUR;
                color = Color::lerp(color, self.key_flash_colors[note_idx], t);
            }

            shapes::fill_rect(fb, x + 1.0, piano_y, key_w - 2.0, wk_h, color);

            let name = note_name(midi);
            let text_scale = if is_mobile { 2 } else { 1 };
            let name_w = text::text_width(name, text_scale);
            let name_x = (x + key_w / 2.0) as i32 - name_w / 2;
            let name_y = (piano_y + wk_h - if is_mobile { 22.0 } else { 14.0 }) as i32;
            let key_text_color = if self.toggled_keys[note_idx] {
                Color::WHITE
            } else {
                Color::from_rgba(80, 80, 100, 255)
            };
            text::draw_text(fb, name_x, name_y, name, key_text_color, text_scale);
        }

        // Black keys on top
        for i in 0..NUM_WHITE_KEYS - 1 {
            let white_midi = white_key_to_midi(i);
            let black_midi = white_midi + 1;
            if !is_white_key(black_midi) && black_midi <= MIDI_HIGH {
                let note_idx = (black_midi - MIDI_LOW) as usize;
                let x = (i as f64 + 1.0) * key_w - black_w / 2.0 - scroll;
                if x + black_w < 0.0 || x > self.screen_w { continue; }

                let mut color = BLACK_KEY_CLR;
                if self.highlighted_keys[note_idx] {
                    color = Color::from_rgba(0, 160, 130, 255);
                }
                if self.toggled_keys[note_idx] {
                    color = KEY_TOGGLED;
                    shapes::fill_rect(fb, x - 1.0, piano_y - 1.0, black_w + 2.0, bk_h + 2.0,
                        KEY_TOGGLED.with_alpha(80));
                }
                if self.key_flash_timers[note_idx] > 0.0 {
                    let t = self.key_flash_timers[note_idx] / KEY_FLASH_DUR;
                    color = Color::lerp(color, self.key_flash_colors[note_idx], t);
                }

                shapes::fill_rect(fb, x, piano_y, black_w, bk_h, color);
            }
        }

        shapes::draw_rect(fb, 0.0, piano_y, self.screen_w, wk_h, DIVIDER);

        if is_mobile {
            let track_h = 16.0;
            let track_y = self.slider_track_y();
            shapes::fill_rect(fb, 28.0, track_y, self.screen_w - 56.0, track_h,
                Color::from_rgba(30, 30, 60, 255));

            let visible_ratio = self.screen_w / self.mobile_piano_total_w();
            let track_inner = self.screen_w - 56.0;
            let thumb_w = (visible_ratio * track_inner).max(40.0);
            let max_thumb_travel = track_inner - thumb_w;
            let scroll_pct = if self.mobile_max_scroll() > 0.0 {
                self.piano_scroll / self.mobile_max_scroll()
            } else { 0.0 };
            let thumb_x = 28.0 + scroll_pct * max_thumb_travel;

            shapes::fill_rect(fb, thumb_x - 2.0, track_y - 6.0,
                thumb_w + 4.0, track_h + 12.0, ACCENT_TEAL.with_alpha(30));
            shapes::fill_rect(fb, thumb_x, track_y - 3.0,
                thumb_w, track_h + 8.0, ACCENT_TEAL.with_alpha(200));
        } else {
            let hint = "R / Space: replay   Click keys to loop";
            let hw = text::text_width(hint, 2);
            text::draw_text(fb, (self.screen_w as i32 - hw) / 2, (piano_y + wk_h + 6.0) as i32,
                hint, Color::from_rgba(180, 180, 210, 255), 2);
        }
    }

    fn render_replay_zone(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let zone_y = self.sy(TIMELINE_Y);
        let zone_h = self.sy(80.0); // compact zone (80px ref vs old 140px)
        let cx = self.screen_w / 2.0;
        let cy = zone_y + zone_h / 2.0;

        // 5 animated wave bars
        let bar_w = 4.0;
        let bar_gap = 6.0;
        let total_bars_w = 5.0 * bar_w + 4.0 * bar_gap;
        let bar_start_x = cx - total_bars_w / 2.0;

        for i in 0..5 {
            let phase = self.pulse_time * 2.5 + i as f64 * 0.6;
            let wave = (phase.sin() * 0.4 + 0.6) * zone_h * 0.4;
            let bx = bar_start_x + i as f64 * (bar_w + bar_gap);
            let by = cy - wave / 2.0;
            let t = i as f64 / 4.0;
            let bar_color = Color::lerp(ACCENT_TEAL, ACCENT_PINK, t);
            let pulse_alpha = ((self.pulse_time * 1.5).sin() * 0.2 + 0.8) * 255.0;
            shapes::fill_rect(fb, bx, by, bar_w, wave, bar_color.with_alpha(pulse_alpha as u8));
        }

        // "Listen Again" text below bars
        let text_y = (cy + zone_h * 0.3) as i32;
        let blink = ((self.pulse_time * 2.0).sin() * 0.15 + 0.85) * 255.0;
        text::draw_text_centered(fb, cx as i32, text_y,
            "Listen Again", ACCENT_TEAL.with_alpha(blink as u8), 1);
    }

    fn render_footer(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let ft_y = self.sy(FOOTER_Y);
        shapes::draw_line(fb, 0.0, ft_y, self.screen_w, ft_y, DIVIDER);

        let due = self.srs.due_count();
        let learning = self.srs.learning_count();
        let mature = self.srs.mature_count();
        let stats = format!("Due: {}  |  Learning: {}  |  Mature: {}", due, learning, mature);
        text::draw_text_centered(fb, (self.screen_w / 2.0) as i32, (ft_y + 16.0) as i32,
            &stats, DIM_TEXT, 1);

        text::draw_text_centered(fb, (self.screen_w / 2.0) as i32, (ft_y + 32.0) as i32,
            "Spaced repetition: harder cards appear more often",
            Color::from_rgba(60, 60, 90, 255), 1);
    }

    fn render_sparkles(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        for s in &self.sparkles {
            let t = s.life / s.max_life;
            let alpha = (t * 255.0) as u8;
            let size = 2.0 + t * 3.0;
            let color = s.color.with_alpha(alpha);
            match s.shape {
                SparkleShape::Circle => {
                    shapes::fill_circle(fb, s.x, s.y, size, color);
                }
                SparkleShape::Star => {
                    let half = size * 1.5;
                    shapes::draw_line(fb, s.x - half, s.y, s.x + half, s.y, color);
                    shapes::draw_line(fb, s.x, s.y - half, s.x, s.y + half, color);
                    shapes::fill_circle(fb, s.x, s.y, size * 0.5, color);
                }
                SparkleShape::Note => {
                    shapes::fill_circle(fb, s.x, s.y, size * 0.7, color);
                    shapes::draw_line(fb, s.x + size * 0.6, s.y,
                        s.x + size * 0.6, s.y - size * 2.0, color);
                }
            }
        }
    }

    fn render_background_stars(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let energy = self.sound_energy;

        for star in &self.background_stars {
            let twinkle = ((self.pulse_time * 1.5 + star.phase).sin() * 0.4 + 0.6) * 255.0;
            let base_alpha = twinkle.max(30.0).min(200.0);
            let boosted_alpha = (base_alpha + energy * 120.0).min(255.0) as u8;

            if star.is_note {
                let base_color = ACCENT_PINK.with_alpha(boosted_alpha / 2);
                let color = Color::lerp(base_color, Color::WHITE.with_alpha(boosted_alpha), energy * 0.6);
                let size = star.size * 0.8 + energy * 1.5;
                shapes::fill_circle(fb, star.x, star.y, size, color);
                shapes::draw_line(fb, star.x + star.size * 0.7, star.y,
                    star.x + star.size * 0.7, star.y - star.size * 2.5, color);
            } else {
                let base_color = ACCENT_TEAL.with_alpha(boosted_alpha / 3);
                let glow_color = Color::lerp(base_color, Color::WHITE.with_alpha(boosted_alpha / 2), energy * 0.5);
                let size = star.size + 1.0 + energy * 2.0;
                shapes::fill_circle(fb, star.x, star.y, size, glow_color);
                let core_color = Color::lerp(
                    Color::WHITE.with_alpha(boosted_alpha / 2),
                    Color::WHITE.with_alpha(boosted_alpha),
                    energy,
                );
                shapes::fill_circle(fb, star.x, star.y, star.size * 0.5 + energy, core_color);
            }
        }
    }

    fn render_border_glow(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let intensity = if self.feedback == FeedbackState::Correct {
            0.6
        } else if self.feedback == FeedbackState::Wrong {
            0.4
        } else {
            0.15 + (self.pulse_time * 0.8).sin().abs() * 0.1
        };

        let alpha = (intensity * 255.0).min(255.0) as u8;
        let glow_color = match &self.feedback {
            FeedbackState::Correct => CORRECT_COLOR.with_alpha(alpha),
            FeedbackState::Wrong => WRONG_COLOR.with_alpha(alpha / 2),
            FeedbackState::Neutral => ACCENT_PINK.with_alpha(alpha),
        };

        let strip = 3.0;
        shapes::fill_rect(fb, 0.0, 0.0, self.screen_w, strip, glow_color);
        shapes::fill_rect(fb, 0.0, self.screen_h - strip, self.screen_w, strip, glow_color);
        shapes::fill_rect(fb, 0.0, 0.0, strip, self.screen_h, glow_color);
        shapes::fill_rect(fb, self.screen_w - strip, 0.0, strip, self.screen_h, glow_color);
    }

    fn render_combo(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        if self.streak < 3 {
            return;
        }
        let cx = (self.screen_w / 2.0) as i32;
        let combo_str = format!("{}x COMBO", self.streak);

        let color = if self.streak >= 10 {
            ACCENT_GOLD
        } else if self.streak >= 5 {
            ACCENT_PINK
        } else {
            ACCENT_TEAL
        };

        let pulse = (self.pulse_time * 4.0).sin() * 0.3 + 0.7;
        let alpha = (pulse * 255.0) as u8;
        let glow_color = color.with_alpha(alpha / 3);

        let combo_y = self.sy(CHALLENGE_Y - 8.0) as i32;
        text::draw_text_centered(fb, cx + 1, combo_y + 1, &combo_str, glow_color, 2);
        text::draw_text_centered(fb, cx, combo_y, &combo_str, color.with_alpha(alpha), 2);
    }
}

/// Word-wrap text to fit within `max_w` pixels at the given scale.
fn wrap_text(s: &str, max_w: i32, scale: u32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in s.split_whitespace() {
        let candidate = if current.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current, word)
        };
        if text::text_width(&candidate, scale) <= max_w {
            current = candidate;
        } else {
            if !current.is_empty() {
                lines.push(current);
            }
            current = word.to_string();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}
