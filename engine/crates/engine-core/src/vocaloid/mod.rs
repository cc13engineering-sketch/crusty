//! Vocaloid Music Theory Simulation — featuring Kasane Teto
//!
//! A music theory discovery game where players complete chord progressions,
//! melodies, identify intervals, and learn Japanese phonemes from the
//! Teto UTAU voicebank. Correct answers chain into a growing musical phrase
//! played with synthesized tones and Teto vocal samples.

pub mod theory;

use crate::engine::Engine;
use crate::simulation::Simulation;
use crate::rendering::color::Color;
use crate::rendering::shapes;
use crate::rendering::text;
use crate::sound::{SoundCommand, Waveform};

use theory::*;

// ─── Layout Constants ───────────────────────────────────────────────

const SCREEN_W: f64 = 600.0;
const SCREEN_H: f64 = 900.0;

const HEADER_H: f64 = 60.0;
const CHALLENGE_Y: f64 = 70.0;
const OPTIONS_Y: f64 = 310.0;
const OPTIONS_H: f64 = 70.0;
const CHARACTER_Y: f64 = 410.0;
const CHARACTER_H: f64 = 100.0;
const PIANO_Y: f64 = 540.0;
const WHITE_KEY_H: f64 = 130.0;
const BLACK_KEY_H: f64 = 85.0;
const TIMELINE_Y: f64 = 690.0;
const TIMELINE_H: f64 = 140.0;
const FOOTER_Y: f64 = 850.0;

// Piano geometry
const NUM_WHITE_KEYS: u8 = 15; // C3 to C5
const WHITE_KEY_W: f64 = SCREEN_W / NUM_WHITE_KEYS as f64; // 40.0
const BLACK_KEY_W: f64 = 24.0;
const MIDI_LOW: u8 = 48;  // C3
const MIDI_HIGH: u8 = 72; // C5
const NUM_NOTES: usize = 25; // MIDI 48–72 inclusive

// Mobile piano (touch devices) — zoomed keys with horizontal scrolling
const MOBILE_ZOOM: f64 = 2.0;
const MOBILE_PIANO_TOTAL_W: f64 = NUM_WHITE_KEYS as f64 * WHITE_KEY_W * MOBILE_ZOOM;
const MOBILE_MAX_SCROLL: f64 = MOBILE_PIANO_TOTAL_W - SCREEN_W;
const SLIDER_TRACK_Y: f64 = PIANO_Y + WHITE_KEY_H + 6.0;

// Option button geometry
const OPT_BTN_W: f64 = 120.0;
const OPT_BTN_GAP: f64 = 16.0;
const OPT_TOTAL_W: f64 = 4.0 * OPT_BTN_W + 3.0 * OPT_BTN_GAP; // 528
const OPT_X_START: f64 = (SCREEN_W - OPT_TOTAL_W) / 2.0; // 36

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

// Teto voice sample names for celebrations
const TETO_VOWELS: &[&str] = &["a", "i", "u", "e", "o"];
const TETO_KA_ROW: &[&str] = &["ka", "ki", "ku", "ke", "ko"];
const TETO_SA_ROW: &[&str] = &["sa", "shi", "su", "se", "so"];
const TETO_TA_ROW: &[&str] = &["ta", "chi", "tsu", "te", "to"];
const TETO_NA_ROW: &[&str] = &["na", "ni", "nu", "ne", "no"];
const TETO_HA_ROW: &[&str] = &["ha", "hi", "fu", "he", "ho"];
const TETO_MA_ROW: &[&str] = &["ma", "mi", "mu", "me", "mo"];
const TETO_RA_ROW: &[&str] = &["ra", "ri", "ru", "re", "ro"];
const TETO_CV_ROWS: &[&[&str]] = &[
    TETO_KA_ROW, TETO_SA_ROW, TETO_TA_ROW, TETO_NA_ROW,
    TETO_HA_ROW, TETO_MA_ROW, TETO_RA_ROW,
];

// Correct/wrong messages with Japanese expressions
const CORRECT_MESSAGES: &[&str] = &[
    "Correct!", "Sugoi!", "Yatta!", "Perfect!", "Subarashii!",
    "Great!", "Kanpeki!", "Nice!", "Ii ne!",
];
const WRONG_MESSAGES: &[&str] = &[
    "Try again!", "Ganbare!", "Mou ikkai!", "Not quite!",
    "Oshii!", "Keep trying!",
];

// ─── Types ──────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub enum MusicConcept {
    ChordProgression,
    NextNote,
    IntervalRecognition,
    PhonemeRecognition,
}

#[derive(Clone, Debug)]
pub struct MusicChallenge {
    concept: MusicConcept,
    key_root: u8,      // MIDI root of the key (e.g. 48 = C3)
    sequence: Vec<u8>, // scale degrees (chords/melody) or MIDI notes (intervals)
    answer: u8,        // correct answer (degree, semitone count, or phoneme index)
    options: Vec<u8>,  // 4 choices including answer
    solved: bool,
    // Phoneme challenge fields
    phoneme_prompt: Option<&'static str>, // romaji prompt for phoneme challenges
    phoneme_options_idx: Vec<usize>,      // indices into GOJUUON for phoneme options
}

#[derive(Clone, Debug)]
pub struct PhraseNote {
    midi: u8,
    lyric: Option<&'static str>, // hiragana syllable from Teto voicebank
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
    Heart,
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

#[derive(Clone, Debug)]
struct ScheduledSample {
    play_at: f64,
    name: String,
    volume: f64,
    pitch: f64,
    duration: f64,
}

// ─── VocaloidSim ────────────────────────────────────────────────────

pub struct VocaloidSim {
    score: u32,
    streak: u8,
    max_streak: u8,
    difficulty: u8,
    challenge: MusicChallenge,
    phrase_notes: Vec<PhraseNote>,
    feedback: FeedbackState,
    feedback_timer: f64,
    highlighted_keys: [bool; NUM_NOTES],
    key_flash_timers: [f64; NUM_NOTES],
    key_flash_colors: [Color; NUM_NOTES],
    pulse_time: f64,
    scheduled_sounds: Vec<ScheduledSound>,
    total_time: f64,
    challenges_completed: u32,
    last_concept_idx: u8,
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
    phrase_playing: bool,
    phrase_play_start: f64,
    last_hovered_phrase_note: Option<usize>,
    /// Sound energy level (0.0–1.0). Spikes on sound events, decays smoothly.
    /// Higher pitch sounds contribute more energy. Drives star glow intensity.
    sound_energy: f64,
}

impl VocaloidSim {
    pub fn new() -> Self {
        Self {
            score: 0,
            streak: 0,
            max_streak: 0,
            difficulty: 1,
            challenge: MusicChallenge {
                concept: MusicConcept::ChordProgression,
                key_root: 60,
                sequence: vec![],
                answer: 0,
                options: vec![],
                solved: false,
                phoneme_prompt: None,
                phoneme_options_idx: vec![],
            },
            phrase_notes: Vec::new(),
            feedback: FeedbackState::Neutral,
            feedback_timer: 0.0,
            highlighted_keys: [false; NUM_NOTES],
            key_flash_timers: [0.0; NUM_NOTES],
            key_flash_colors: [Color::WHITE; NUM_NOTES],
            pulse_time: 0.0,
            scheduled_sounds: Vec::new(),
            total_time: 0.0,
            challenges_completed: 0,
            last_concept_idx: 3, // so first challenge wraps to 0 (ChordProgression)
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
            phrase_playing: false,
            phrase_play_start: 0.0,
            last_hovered_phrase_note: None,
            sound_energy: 0.0,
        }
    }

    // ─── Challenge Generation ────────────────────────────────

    fn generate_challenge(&mut self, engine: &mut Engine) {
        self.last_concept_idx = (self.last_concept_idx + 1) % 4;
        let key_roots = [48u8, 50, 52, 53, 55, 57];
        let key_root = key_roots[engine.rng.range_i32(0, key_roots.len() as i32 - 1) as usize];

        match self.last_concept_idx {
            0 => self.generate_chord_challenge(key_root, engine),
            1 => self.generate_note_challenge(key_root, engine),
            2 => self.generate_interval_challenge(key_root, engine),
            _ => self.generate_phoneme_challenge(engine),
        }
        self.update_highlights();
    }

    fn generate_chord_challenge(&mut self, key_root: u8, engine: &mut Engine) {
        let mut seq = vec![0u8]; // Start on I
        for _ in 0..2 {
            let last = *seq.last().unwrap_or(&0);
            let nexts = likely_next_degrees(last);
            let idx = engine.rng.range_i32(0, nexts.len() as i32 - 1) as usize;
            seq.push(nexts[idx]);
        }

        let last = *seq.last().unwrap_or(&0);
        let nexts = likely_next_degrees(last);
        let idx = engine.rng.range_i32(0, nexts.len() as i32 - 1) as usize;
        let answer = nexts[idx];

        let pool: Vec<u8> = if self.difficulty <= 3 {
            vec![0, 3, 4, 5]
        } else if self.difficulty <= 7 {
            vec![0, 1, 2, 3, 4, 5]
        } else {
            (0..7).collect()
        };

        let options = generate_options(answer, &pool, 4, engine.rng.next_u64());

        // Play the existing chord sequence
        for (i, &deg) in seq.iter().enumerate() {
            let root = key_root + MAJOR_SCALE[(deg % 7) as usize];
            let freq = midi_to_freq(root);
            self.schedule_sound(ScheduledSound {
                play_at: self.total_time + i as f64 * 0.5,
                frequency: freq,
                duration: 0.4,
                volume: 0.5,
                waveform: Waveform::Sine,
                attack: 0.02,
                decay: 0.3,
            });
        }

        self.challenge = MusicChallenge {
            concept: MusicConcept::ChordProgression,
            key_root,
            sequence: seq,
            answer,
            options,
            solved: false,
            phoneme_prompt: None,
            phoneme_options_idx: vec![],
        };
    }

    fn generate_note_challenge(&mut self, key_root: u8, engine: &mut Engine) {
        let start_degree = engine.rng.range_i32(0, 4) as u8;
        let mut seq = vec![start_degree];
        for _ in 0..3 {
            let last = *seq.last().unwrap_or(&0) as i8;
            let step = if engine.rng.chance(0.65) { 1i8 } else { -1 };
            let next = (last + step).max(0).min(6) as u8;
            seq.push(next);
        }

        let last = *seq.last().unwrap_or(&0) as i8;
        let prev = seq.get(seq.len().wrapping_sub(2)).copied().unwrap_or(0) as i8;
        let direction = if last >= prev { 1i8 } else { -1 };
        let answer = (last + direction).max(0).min(6) as u8;

        let pool: Vec<u8> = (0..7).collect();
        let options = generate_options(answer, &pool, 4, engine.rng.next_u64());

        // Play the melody
        for (i, &deg) in seq.iter().enumerate() {
            let midi = degree_to_midi(key_root, deg);
            self.schedule_sound(ScheduledSound {
                play_at: self.total_time + i as f64 * 0.35,
                frequency: midi_to_freq(midi),
                duration: 0.3,
                volume: 0.6,
                waveform: Waveform::Triangle,
                attack: 0.01,
                decay: 0.25,
            });
        }

        self.challenge = MusicChallenge {
            concept: MusicConcept::NextNote,
            key_root,
            sequence: seq,
            answer,
            options,
            solved: false,
            phoneme_prompt: None,
            phoneme_options_idx: vec![],
        };
    }

    fn generate_interval_challenge(&mut self, key_root: u8, engine: &mut Engine) {
        let base_offset = engine.rng.range_i32(0, 11) as u8;
        let base_midi = key_root + base_offset;

        let intervals: Vec<u8> = if self.difficulty <= 4 {
            vec![0, 2, 4, 7, 12]
        } else {
            (0..=12).collect()
        };

        let idx = engine.rng.range_i32(0, intervals.len() as i32 - 1) as usize;
        let interval = intervals[idx];
        let top_midi = base_midi + interval;

        let options = generate_options(interval, &intervals, 4, engine.rng.next_u64());

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
            phoneme_prompt: None,
            phoneme_options_idx: vec![],
        };
    }

    fn generate_phoneme_challenge(&mut self, engine: &mut Engine) {
        // Pick a random phoneme from the gojuuon table
        let max_idx = if self.difficulty <= 3 {
            10  // vowels + ka-row only
        } else if self.difficulty <= 6 {
            25  // through na-row
        } else {
            GOJUUON.len()
        };

        let answer_idx = engine.rng.range_i32(0, max_idx as i32 - 1) as usize;
        let phoneme = &GOJUUON[answer_idx];

        // Generate 4 options (indices into GOJUUON)
        let opts = generate_phoneme_options(answer_idx, 4, engine.rng.next_u64());

        // The answer position in the options list (for the u8-based system)
        // We'll store the answer as the position and use phoneme_options_idx for lookup
        let answer_pos = opts.iter().position(|&i| i == answer_idx).unwrap_or(0) as u8;

        // Play the Teto sample for this phoneme
        engine.sound_queue.push(SoundCommand::PlaySample {
            name: phoneme.sample.to_string(),
            volume: 0.8,
            pitch: 1.0,
            duration: 0.5,
        });

        self.challenge = MusicChallenge {
            concept: MusicConcept::PhonemeRecognition,
            key_root: 60,
            sequence: vec![answer_idx as u8],
            answer: answer_pos,
            options: (0..opts.len() as u8).collect(),
            solved: false,
            phoneme_prompt: Some(phoneme.romaji),
            phoneme_options_idx: opts,
        };
    }

    fn update_highlights(&mut self) {
        self.highlighted_keys = [false; NUM_NOTES];
        match &self.challenge.concept {
            MusicConcept::ChordProgression => {
                if let Some(&last_deg) = self.challenge.sequence.last() {
                    let root = self.challenge.key_root + MAJOR_SCALE[(last_deg % 7) as usize];
                    for &interval in &chord_intervals(last_deg) {
                        let midi = root + interval;
                        if midi >= MIDI_LOW && midi <= MIDI_HIGH {
                            self.highlighted_keys[(midi - MIDI_LOW) as usize] = true;
                        }
                    }
                }
            }
            MusicConcept::NextNote => {
                for &deg in &self.challenge.sequence {
                    let midi = degree_to_midi(self.challenge.key_root, deg);
                    if midi >= MIDI_LOW && midi <= MIDI_HIGH {
                        self.highlighted_keys[(midi - MIDI_LOW) as usize] = true;
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
            MusicConcept::PhonemeRecognition => {
                // No piano highlights for phoneme challenges
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
        self.score += 10 + self.streak as u32 * 5;
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
            MusicConcept::ChordProgression => {
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
                self.phrase_notes.push(PhraseNote { midi: root, lyric: None });
            }
            MusicConcept::NextNote => {
                let deg = self.challenge.answer;
                let midi = degree_to_midi(self.challenge.key_root, deg);
                engine.sound_queue.push(SoundCommand::PlayTone {
                    frequency: midi_to_freq(midi),
                    duration: 0.4,
                    volume: 0.7,
                    waveform: Waveform::Triangle,
                    attack: 0.01,
                    decay: 0.3,
                });
                self.flash_key(midi, CORRECT_COLOR);
                self.phrase_notes.push(PhraseNote { midi, lyric: None });
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
                    self.phrase_notes.push(PhraseNote { midi: top, lyric: None });
                }
            }
            MusicConcept::PhonemeRecognition => {
                // Play the correct Teto sample
                if let Some(answer_global) = self.challenge.sequence.first() {
                    let idx = *answer_global as usize;
                    if idx < GOJUUON.len() {
                        let p = &GOJUUON[idx];
                        engine.sound_queue.push(SoundCommand::PlaySample {
                            name: p.sample.to_string(),
                            volume: 0.9,
                            pitch: 1.0,
                            duration: 0.5,
                        });
                        // Add to phrase with lyric
                        let midi = 60 + (p.vowel as u8) * 2; // map vowel class to pitch
                        self.phrase_notes.push(PhraseNote {
                            midi,
                            lyric: Some(p.kana),
                        });
                    }
                }
            }
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

        // Teto celebration: play a random vowel sample on ALL correct answers
        let vowel_idx = engine.rng.range_i32(0, TETO_VOWELS.len() as i32 - 1) as usize;
        let pitch = 1.0 + (self.streak.min(20) as f64) * 0.05;
        engine.sound_queue.push(SoundCommand::PlaySample {
            name: TETO_VOWELS[vowel_idx].to_string(),
            volume: 0.4,
            pitch,
            duration: 0.4,
        });

        // Streak milestone: every 5, play ascending CV sequence
        if self.streak > 0 && self.streak % 5 == 0 {
            let row_idx = engine.rng.range_i32(0, TETO_CV_ROWS.len() as i32 - 1) as usize;
            let row = TETO_CV_ROWS[row_idx];
            for (i, &sample) in row.iter().enumerate() {
                self.scheduled_samples.push(ScheduledSample {
                    play_at: self.total_time + 0.15 + i as f64 * 0.12,
                    name: sample.to_string(),
                    volume: 0.35,
                    pitch: 1.0 + i as f64 * 0.08,
                    duration: 0.3,
                });
            }
            // Big sparkle burst for milestone
            for x_off in 0..6 {
                let bx = 50.0 + x_off as f64 * 100.0;
                self.spawn_sparkles_shaped(bx, OPTIONS_Y, 8, ACCENT_PINK, engine);
            }
        }

        self.combo_pulse = 1.0;

        // Spawn sparkles with varied shapes
        self.spawn_sparkles_shaped(SCREEN_W / 2.0, OPTIONS_Y + OPTIONS_H / 2.0, 25, ACCENT_CYAN, engine);
    }

    fn on_wrong(&mut self, _selected: u8, engine: &mut Engine) {
        self.feedback = FeedbackState::Wrong;
        self.feedback_timer = FEEDBACK_WRONG_DUR;
        self.streak = 0;

        // Flash wrong notes on piano (only for music challenges)
        match &self.challenge.concept {
            MusicConcept::ChordProgression => {
                let root = self.challenge.key_root + MAJOR_SCALE[(_selected % 7) as usize];
                for &interval in &chord_intervals(_selected) {
                    self.flash_key(root + interval, WRONG_COLOR);
                }
            }
            MusicConcept::NextNote => {
                let midi = degree_to_midi(self.challenge.key_root, _selected);
                self.flash_key(midi, WRONG_COLOR);
            }
            MusicConcept::IntervalRecognition | MusicConcept::PhonemeRecognition => {}
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

        // Teto soft "n" sample
        engine.sound_queue.push(SoundCommand::PlaySample {
            name: "n".to_string(),
            volume: 0.25,
            pitch: 0.8,
            duration: 0.4,
        });

        // Sad sparkles
        self.spawn_sparkles_shaped(SCREEN_W / 2.0, OPTIONS_Y + OPTIONS_H / 2.0, 8, WRONG_COLOR, engine);
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
                x: engine.rng.range_f64(0.0, SCREEN_W),
                y: engine.rng.range_f64(0.0, SCREEN_H),
                speed: engine.rng.range_f64(8.0, 30.0),
                phase: engine.rng.range_f64(0.0, std::f64::consts::TAU),
                is_note: engine.rng.chance(0.3),
                size: engine.rng.range_f64(1.5, 3.5),
            });
        }
    }

    fn spawn_sparkles_shaped(&mut self, cx: f64, cy: f64, count: usize, color: Color, engine: &mut Engine) {
        let shapes = [SparkleShape::Circle, SparkleShape::Star, SparkleShape::Heart, SparkleShape::Note];
        for _ in 0..count {
            let angle = engine.rng.range_f64(0.0, std::f64::consts::TAU);
            let speed = engine.rng.range_f64(40.0, 180.0);
            let shape_idx = engine.rng.range_i32(0, shapes.len() as i32 - 1) as usize;
            // Teto pink/red palette variation
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

    fn option_rect(idx: usize) -> (f64, f64, f64, f64) {
        let x = OPT_X_START + idx as f64 * (OPT_BTN_W + OPT_BTN_GAP);
        (x, OPTIONS_Y, OPT_BTN_W, OPTIONS_H)
    }

    fn check_option_click(&self, mx: f64, my: f64) -> Option<usize> {
        for i in 0..self.challenge.options.len() {
            let (x, y, w, h) = Self::option_rect(i);
            if mx >= x && mx <= x + w && my >= y && my <= y + h {
                return Some(i);
            }
        }
        None
    }

    fn option_label(&self, value: u8) -> String {
        match &self.challenge.concept {
            MusicConcept::ChordProgression => {
                DEGREE_NAMES[(value % 7) as usize].to_string()
            }
            MusicConcept::NextNote => {
                let midi = degree_to_midi(self.challenge.key_root, value);
                note_name(midi).to_string()
            }
            MusicConcept::IntervalRecognition => {
                if (value as usize) < INTERVAL_NAMES.len() {
                    INTERVAL_NAMES[value as usize].to_string()
                } else {
                    format!("{}st", value)
                }
            }
            MusicConcept::PhonemeRecognition => {
                // value is the option index (0-3)
                let gojuuon_idx = self.challenge.phoneme_options_idx
                    .get(value as usize)
                    .copied()
                    .unwrap_or(0);
                if gojuuon_idx < GOJUUON.len() {
                    GOJUUON[gojuuon_idx].kana.to_string()
                } else {
                    "?".to_string()
                }
            }
        }
    }

    // ─── Piano Interaction ──────────────────────────────────

    fn check_piano_click(&self, mx: f64, my: f64, zoom: f64, scroll: f64) -> Option<u8> {
        if my < PIANO_Y || my > PIANO_Y + WHITE_KEY_H {
            return None;
        }
        let vx = mx + scroll; // virtual X in zoomed coordinate space
        let key_w = WHITE_KEY_W * zoom;
        let black_w = BLACK_KEY_W * zoom;
        // Black keys first (rendered on top, overlap whites)
        if my < PIANO_Y + BLACK_KEY_H {
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

    fn midi_to_sample(midi: u8) -> (&'static str, f64) {
        // Map MIDI note to Teto vowel sample with pitch shifting
        // Cycle vowels: a, i, u, e, o based on note
        let vowel_idx = (midi % 5) as usize;
        let sample = TETO_VOWELS[vowel_idx];
        // Pitch relative to C4 (MIDI 60)
        let pitch = 2.0_f64.powf((midi as f64 - 60.0) / 12.0);
        (sample, pitch)
    }

    fn play_option_preview(&self, idx: usize, engine: &mut Engine) {
        if idx >= self.challenge.options.len() {
            return;
        }
        let opt = self.challenge.options[idx];
        match &self.challenge.concept {
            MusicConcept::ChordProgression => {
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
            MusicConcept::NextNote => {
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
            MusicConcept::PhonemeRecognition => {
                let gojuuon_idx = self.challenge.phoneme_options_idx
                    .get(opt as usize)
                    .copied()
                    .unwrap_or(0);
                if gojuuon_idx < GOJUUON.len() {
                    engine.sound_queue.push(SoundCommand::PlaySample {
                        name: GOJUUON[gojuuon_idx].sample.to_string(),
                        volume: 0.5,
                        pitch: 1.0,
                        duration: 0.4,
                    });
                }
            }
        }
    }

    fn replay_challenge(&mut self, engine: &mut Engine) {
        let seq = self.challenge.sequence.clone();
        let key_root = self.challenge.key_root;
        let concept = self.challenge.concept.clone();
        let now = self.total_time;
        match concept {
            MusicConcept::ChordProgression => {
                for (i, &deg) in seq.iter().enumerate() {
                    let root = key_root + MAJOR_SCALE[(deg % 7) as usize];
                    self.scheduled_sounds.push(ScheduledSound {
                        play_at: now + i as f64 * 0.5,
                        frequency: midi_to_freq(root),
                        duration: 0.4,
                        volume: 0.5,
                        waveform: Waveform::Sine,
                        attack: 0.02,
                        decay: 0.3,
                    });
                }
            }
            MusicConcept::NextNote => {
                for (i, &deg) in seq.iter().enumerate() {
                    let midi = degree_to_midi(key_root, deg);
                    self.scheduled_sounds.push(ScheduledSound {
                        play_at: now + i as f64 * 0.35,
                        frequency: midi_to_freq(midi),
                        duration: 0.3,
                        volume: 0.6,
                        waveform: Waveform::Triangle,
                        attack: 0.01,
                        decay: 0.25,
                    });
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
            MusicConcept::PhonemeRecognition => {
                if let Some(&answer_idx) = seq.first() {
                    let idx = answer_idx as usize;
                    if idx < GOJUUON.len() {
                        engine.sound_queue.push(SoundCommand::PlaySample {
                            name: GOJUUON[idx].sample.to_string(),
                            volume: 0.8,
                            pitch: 1.0,
                            duration: 0.5,
                        });
                    }
                }
            }
        }
    }

    // ─── Phrase Playback ────────────────────────────────────

    fn play_phrase(&mut self, _engine: &mut Engine) {
        if self.phrase_notes.is_empty() {
            return;
        }
        self.phrase_playing = true;
        self.phrase_play_start = self.total_time;
        let note_spacing = 0.3;
        for (i, note) in self.phrase_notes.iter().enumerate() {
            let (sample, pitch) = Self::midi_to_sample(note.midi);
            self.scheduled_samples.push(ScheduledSample {
                play_at: self.total_time + i as f64 * note_spacing,
                name: sample.to_string(),
                volume: 0.5,
                pitch,
                duration: note_spacing * 0.9,
            });
        }
    }

    fn check_phrase_note_hover(&self, mx: f64, my: f64) -> Option<usize> {
        if self.phrase_notes.is_empty() {
            return None;
        }
        if my < TIMELINE_Y + 18.0 || my > TIMELINE_Y + TIMELINE_H - 8.0 {
            return None;
        }
        let note_w = 30.0_f64;
        let total_width = self.phrase_notes.len() as f64 * note_w;
        let x_offset = if total_width > SCREEN_W - 40.0 {
            SCREEN_W - 40.0 - total_width
        } else {
            20.0
        };
        for (i, _) in self.phrase_notes.iter().enumerate() {
            let x = x_offset + i as f64 * note_w;
            if mx >= x && mx <= x + note_w {
                return Some(i);
            }
        }
        None
    }
}

// ─── Simulation Trait ───────────────────────────────────────────────

impl Simulation for VocaloidSim {
    fn setup(&mut self, engine: &mut Engine) {
        engine.config.bounds = (SCREEN_W, SCREEN_H);
        engine.config.background = BG_COLOR;
        self.init_background_stars(engine);
        self.generate_challenge(engine);
        engine.global_state.set_f64("score", 0.0);
        engine.global_state.set_f64("streak", 0.0);
        engine.global_state.set_f64("difficulty", 1.0);
    }

    fn step(&mut self, engine: &mut Engine) {
        let dt = 1.0 / 60.0;
        self.total_time += dt;
        self.pulse_time += dt;

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
                star.y = SCREEN_H + 10.0;
            }
        }

        // Update feedback timer
        if self.feedback_timer > 0.0 {
            self.feedback_timer -= dt;
            if self.feedback_timer <= 0.0 {
                if self.feedback == FeedbackState::Correct {
                    self.generate_challenge(engine);
                }
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

        // Mobile slider dragging
        if is_mobile {
            if engine.input.mouse_buttons_pressed.contains(&0)
                && my > PIANO_Y + WHITE_KEY_H && my < TIMELINE_Y
            {
                self.slider_dragging = true;
            }
            if self.slider_dragging {
                if engine.input.mouse_buttons_held.contains(&0) {
                    let ratio = (mx / SCREEN_W).max(0.0).min(1.0);
                    self.piano_scroll = ratio * MOBILE_MAX_SCROLL;
                }
                if engine.input.mouse_buttons_released.contains(&0) {
                    self.slider_dragging = false;
                }
            }
        }

        if engine.input.mouse_buttons_pressed.contains(&0) && !self.slider_dragging {
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
            // Timeline click → play phrase from start
            if my >= TIMELINE_Y && my <= TIMELINE_Y + TIMELINE_H
                && !self.phrase_notes.is_empty()
            {
                self.play_phrase(engine);
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
                // Transition to ON — play all toggled keys
                for i in 0..NUM_NOTES {
                    if self.toggled_keys[i] {
                        let midi = MIDI_LOW + i as u8;
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
        let hx = engine.input.hover_x;
        let hy = engine.input.hover_y;
        let hovered = if engine.input.hover_active {
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

        // Phrase note hover preview (uses unified hover)
        let hovered_phrase = if engine.input.hover_active {
            self.check_phrase_note_hover(hx, hy)
        } else {
            None
        };
        if hovered_phrase != self.last_hovered_phrase_note {
            if let Some(idx) = hovered_phrase {
                if idx < self.phrase_notes.len() {
                    let midi = self.phrase_notes[idx].midi;
                    let (sample, pitch) = Self::midi_to_sample(midi);
                    engine.sound_queue.push(SoundCommand::PlaySample {
                        name: sample.to_string(),
                        volume: 0.35,
                        pitch,
                        duration: 0.3,
                    });
                }
            }
            self.last_hovered_phrase_note = hovered_phrase;
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

        // Replay challenge (Space or R)
        if engine.input.keys_pressed.contains("Space")
            || engine.input.keys_pressed.contains("KeyR")
        {
            if !self.challenge.solved && self.feedback == FeedbackState::Neutral {
                self.replay_challenge(engine);
            }
        }

        // Spike sound energy based on new sounds queued this frame
        let new_sounds = engine.sound_queue.len().saturating_sub(sound_count_before);
        if new_sounds > 0 {
            self.sound_energy = (self.sound_energy + new_sounds as f64 * 0.25).min(1.0);
        }

        // Export state for JS HUD
        engine.global_state.set_f64("score", self.score as f64);
        engine.global_state.set_f64("streak", self.streak as f64);
        engine.global_state.set_f64("difficulty", self.difficulty as f64);
        engine.global_state.set_f64("challenges_completed", self.challenges_completed as f64);
        engine.global_state.set_f64("phrase_len", self.phrase_notes.len() as f64);
    }

    fn render(&self, engine: &mut Engine) {
        // Use hover position for visual hover effects (works on desktop + touch)
        let hx = if engine.input.hover_active { engine.input.hover_x } else { -1000.0 };
        let hy = if engine.input.hover_active { engine.input.hover_y } else { -1000.0 };
        let is_mobile = engine.browser_state.is_touch_device();
        let fb = &mut engine.framebuffer;

        self.render_background_stars(fb);
        self.render_border_glow(fb);
        self.render_header(fb);
        self.render_challenge(fb);
        self.render_combo(fb);
        self.render_options(fb, hx, hy);
        self.render_character(fb);
        self.render_piano(fb, is_mobile);
        self.render_timeline(fb, hx, hy);
        self.render_footer(fb);
        self.render_sparkles(fb);
    }
}

// ─── Rendering ──────────────────────────────────────────────────────

impl VocaloidSim {
    fn render_header(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        // Teto branding
        text::draw_text(fb, 16, 14, "TETO THEORY", ACCENT_RED, 2);
        let sub = "Kasane Teto";
        text::draw_text(fb, 16, 38, sub, ACCENT_PINK, 1);

        let score_str = format!("Score: {}", self.score);
        let sw = text::text_width(&score_str, 2);
        text::draw_text(fb, 600 - sw - 16, 14, &score_str, Color::WHITE, 2);

        if self.streak > 0 {
            let stars: String = (0..self.streak.min(10)).map(|_| '*').collect();
            let streak_str = format!("{}", stars);
            let stw = text::text_width(&streak_str, 1);
            text::draw_text(fb, 600 - stw - 16, 38, &streak_str, ACCENT_PINK, 1);
        }

        shapes::draw_line(fb, 0.0, HEADER_H - 1.0, SCREEN_W, HEADER_H - 1.0, DIVIDER);
    }

    fn render_challenge(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let cx = (SCREEN_W / 2.0) as i32;

        match &self.challenge.concept {
            MusicConcept::ChordProgression => {
                text::draw_text_centered(fb, cx, (CHALLENGE_Y + 12.0) as i32,
                    "CHORD PROGRESSION", DIM_TEXT, 1);
                text::draw_text_centered(fb, cx, (CHALLENGE_Y + 36.0) as i32,
                    "What comes next?", Color::WHITE, 2);

                let mut prog = String::new();
                for (i, &deg) in self.challenge.sequence.iter().enumerate() {
                    if i > 0 { prog.push_str(" > "); }
                    prog.push_str(DEGREE_NAMES[(deg % 7) as usize]);
                }
                prog.push_str(" > ?");

                text::draw_text_centered(fb, cx, (CHALLENGE_Y + 90.0) as i32,
                    &prog, ACCENT_TEAL, 3);

                let key_str = format!("Key: {} Major", note_name(self.challenge.key_root));
                text::draw_text_centered(fb, cx, (CHALLENGE_Y + 150.0) as i32,
                    &key_str, DIM_TEXT, 1);
            }
            MusicConcept::NextNote => {
                text::draw_text_centered(fb, cx, (CHALLENGE_Y + 12.0) as i32,
                    "MELODY", DIM_TEXT, 1);
                text::draw_text_centered(fb, cx, (CHALLENGE_Y + 36.0) as i32,
                    "What note comes next?", Color::WHITE, 2);

                let mut melody = String::new();
                for (i, &deg) in self.challenge.sequence.iter().enumerate() {
                    if i > 0 { melody.push_str("  "); }
                    let midi = degree_to_midi(self.challenge.key_root, deg);
                    melody.push_str(note_name(midi));
                }
                melody.push_str("  ?");

                text::draw_text_centered(fb, cx, (CHALLENGE_Y + 90.0) as i32,
                    &melody, ACCENT_TEAL, 3);

                let key_str = format!("Key: {} Major", note_name(self.challenge.key_root));
                text::draw_text_centered(fb, cx, (CHALLENGE_Y + 150.0) as i32,
                    &key_str, DIM_TEXT, 1);
            }
            MusicConcept::IntervalRecognition => {
                text::draw_text_centered(fb, cx, (CHALLENGE_Y + 12.0) as i32,
                    "INTERVAL", DIM_TEXT, 1);
                text::draw_text_centered(fb, cx, (CHALLENGE_Y + 36.0) as i32,
                    "Name this interval", Color::WHITE, 2);

                if self.challenge.sequence.len() >= 2 {
                    let note1 = note_name(self.challenge.sequence[0]);
                    let note2 = note_name(self.challenge.sequence[1]);
                    let display = format!("{} --> {}", note1, note2);
                    text::draw_text_centered(fb, cx, (CHALLENGE_Y + 90.0) as i32,
                        &display, ACCENT_TEAL, 3);
                }
            }
            MusicConcept::PhonemeRecognition => {
                text::draw_text_centered(fb, cx, (CHALLENGE_Y + 12.0) as i32,
                    "TETO PHONEME", DIM_TEXT, 1);
                text::draw_text_centered(fb, cx, (CHALLENGE_Y + 36.0) as i32,
                    "Which kana matches this sound?", Color::WHITE, 2);

                // Show romaji prompt
                if let Some(romaji) = self.challenge.phoneme_prompt {
                    text::draw_text_centered(fb, cx, (CHALLENGE_Y + 80.0) as i32,
                        romaji, ACCENT_PINK, 3);
                }

                // Show consonant row hint
                if let Some(&answer_idx) = self.challenge.sequence.first() {
                    let idx = answer_idx as usize;
                    if idx < GOJUUON.len() {
                        let row = phoneme_row(idx);
                        if row < CONSONANT_ROWS.len() {
                            let row_label = format!("Row: {}", CONSONANT_ROWS[row]);
                            text::draw_text_centered(fb, cx, (CHALLENGE_Y + 130.0) as i32,
                                &row_label, DIM_TEXT, 1);
                        }
                    }
                }

                // "Listen" hint
                text::draw_text_centered(fb, cx, (CHALLENGE_Y + 160.0) as i32,
                    "(click to hear again)", DIM_TEXT, 1);
            }
        }
    }

    fn render_options(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, mx: f64, my: f64) {
        for (i, &opt) in self.challenge.options.iter().enumerate() {
            let (x, y, w, h) = Self::option_rect(i);

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
            } else if self.challenge.concept == MusicConcept::PhonemeRecognition {
                ACCENT_PINK.with_alpha(80)
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

    fn render_character(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let cx = (SCREEN_W / 2.0) as i32;
        let cy = (CHARACTER_Y + CHARACTER_H / 2.0) as i32;

        // Character area is now rendered by HTML overlay (official Teto art)
        // We only render the feedback message text below the art area
        let (msg, color) = match &self.feedback {
            FeedbackState::Correct => {
                let idx = ((self.pulse_time * 2.0) as usize) % CORRECT_MESSAGES.len();
                (CORRECT_MESSAGES[idx], CORRECT_COLOR)
            }
            FeedbackState::Wrong => {
                let idx = ((self.pulse_time * 2.0) as usize) % WRONG_MESSAGES.len();
                (WRONG_MESSAGES[idx], WRONG_COLOR)
            }
            FeedbackState::Neutral => ("", ACCENT_RED),
        };

        if !msg.is_empty() {
            text::draw_text_centered(fb, cx, cy + 35, msg, color, 2);
        }

        // Subtle floating decorations around art area
        let bob = (self.pulse_time * 2.0).sin() * 4.0;
        let sparkle_alpha = ((self.pulse_time * 3.0).sin() * 0.3 + 0.5) * 255.0;
        shapes::fill_circle(fb, 80.0, CHARACTER_Y + 20.0 + bob,
            2.0, ACCENT_PINK.with_alpha(sparkle_alpha as u8));
        shapes::fill_circle(fb, SCREEN_W - 80.0, CHARACTER_Y + 30.0 - bob,
            2.0, ACCENT_RED.with_alpha(sparkle_alpha as u8));
    }

    fn render_piano(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, is_mobile: bool) {
        let (zoom, scroll) = if is_mobile {
            (MOBILE_ZOOM, self.piano_scroll)
        } else {
            (1.0, 0.0)
        };
        let key_w = WHITE_KEY_W * zoom;
        let black_w = BLACK_KEY_W * zoom;
        let toggle_pulse = (self.rhythm_timer * 8.0).sin() * 0.3 + 0.7;

        // White keys
        for i in 0..NUM_WHITE_KEYS {
            let midi = white_key_to_midi(i);
            let note_idx = (midi - MIDI_LOW) as usize;
            let x = i as f64 * key_w - scroll;

            // Cull off-screen keys
            if x + key_w < 0.0 || x > SCREEN_W { continue; }

            let mut color = WHITE_KEY_CLR;
            if self.highlighted_keys[note_idx] {
                color = Color::from_rgba(120, 220, 200, 255);
            }
            if self.toggled_keys[note_idx] {
                let glow_alpha = (toggle_pulse * 255.0) as u8;
                color = Color::lerp(color, ACCENT_PINK, toggle_pulse * 0.6);
                shapes::fill_rect(fb, x, PIANO_Y - 2.0, key_w, WHITE_KEY_H + 4.0,
                    ACCENT_PINK.with_alpha(glow_alpha / 4));
            }
            if self.key_flash_timers[note_idx] > 0.0 {
                let t = self.key_flash_timers[note_idx] / KEY_FLASH_DUR;
                color = Color::lerp(color, self.key_flash_colors[note_idx], t);
            }

            shapes::fill_rect(fb, x + 1.0, PIANO_Y, key_w - 2.0, WHITE_KEY_H, color);

            // Note name at bottom of key
            let name = note_name(midi);
            let text_scale = if is_mobile { 2 } else { 1 };
            let name_w = text::text_width(name, text_scale);
            let name_x = (x + key_w / 2.0) as i32 - name_w / 2;
            let name_y = (PIANO_Y + WHITE_KEY_H - if is_mobile { 22.0 } else { 14.0 }) as i32;
            text::draw_text(fb, name_x, name_y,
                name, Color::from_rgba(80, 80, 100, 255), text_scale);
        }

        // Black keys on top
        for i in 0..NUM_WHITE_KEYS - 1 {
            let white_midi = white_key_to_midi(i);
            let black_midi = white_midi + 1;
            if !is_white_key(black_midi) && black_midi <= MIDI_HIGH {
                let note_idx = (black_midi - MIDI_LOW) as usize;
                let x = (i as f64 + 1.0) * key_w - black_w / 2.0 - scroll;

                if x + black_w < 0.0 || x > SCREEN_W { continue; }

                let mut color = BLACK_KEY_CLR;
                if self.highlighted_keys[note_idx] {
                    color = Color::from_rgba(0, 160, 130, 255);
                }
                if self.toggled_keys[note_idx] {
                    color = Color::lerp(color, ACCENT_PINK, toggle_pulse * 0.5);
                }
                if self.key_flash_timers[note_idx] > 0.0 {
                    let t = self.key_flash_timers[note_idx] / KEY_FLASH_DUR;
                    color = Color::lerp(color, self.key_flash_colors[note_idx], t);
                }

                shapes::fill_rect(fb, x, PIANO_Y, black_w, BLACK_KEY_H, color);
            }
        }

        // Piano border
        shapes::draw_rect(fb, 0.0, PIANO_Y, SCREEN_W, WHITE_KEY_H, DIVIDER);

        if is_mobile {
            // Scroll slider
            let track_h = 3.0;
            shapes::fill_rect(fb, 12.0, SLIDER_TRACK_Y, SCREEN_W - 24.0, track_h,
                Color::from_rgba(30, 30, 60, 255));

            let visible_ratio = SCREEN_W / MOBILE_PIANO_TOTAL_W;
            let track_inner = SCREEN_W - 24.0;
            let thumb_w = (visible_ratio * track_inner).max(24.0);
            let max_thumb_travel = track_inner - thumb_w;
            let scroll_pct = if MOBILE_MAX_SCROLL > 0.0 {
                self.piano_scroll / MOBILE_MAX_SCROLL
            } else { 0.0 };
            let thumb_x = 12.0 + scroll_pct * max_thumb_travel;

            // Thumb glow
            shapes::fill_rect(fb, thumb_x - 1.0, SLIDER_TRACK_Y - 4.0,
                thumb_w + 2.0, track_h + 8.0, ACCENT_TEAL.with_alpha(30));
            // Thumb body
            shapes::fill_rect(fb, thumb_x, SLIDER_TRACK_Y - 2.0,
                thumb_w, track_h + 4.0, ACCENT_TEAL.with_alpha(180));
        } else {
            // Desktop hint
            let hint = "R / Space: replay   Click keys to loop";
            let hw = text::text_width(hint, 1);
            text::draw_text(fb, (SCREEN_W as i32 - hw) / 2, (PIANO_Y + WHITE_KEY_H + 4.0) as i32,
                hint, DIM_TEXT, 1);
        }
    }

    fn render_timeline(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, mx: f64, my: f64) {
        shapes::fill_rect(fb, 0.0, TIMELINE_Y, SCREEN_W, TIMELINE_H,
            Color::from_rgba(8, 8, 24, 255));

        // Header with play hint
        let header = if self.phrase_notes.is_empty() {
            "COMPOSED PHRASE"
        } else {
            "COMPOSED PHRASE (tap to play all)"
        };
        text::draw_text(fb, 8, (TIMELINE_Y + 6.0) as i32, header, DIM_TEXT, 1);

        if self.phrase_notes.is_empty() {
            text::draw_text_centered(fb, 300, (TIMELINE_Y + TIMELINE_H / 2.0) as i32,
                "Complete challenges to build your phrase!",
                Color::from_rgba(50, 50, 80, 255), 1);
            return;
        }

        let note_w = 30.0_f64;
        let total_width = self.phrase_notes.len() as f64 * note_w;
        let x_offset = if total_width > SCREEN_W - 40.0 {
            SCREEN_W - 40.0 - total_width
        } else {
            20.0
        };

        let min_midi = self.phrase_notes.iter().map(|n| n.midi).min().unwrap_or(48);
        let max_midi = self.phrase_notes.iter().map(|n| n.midi).max().unwrap_or(72);
        let range = (max_midi - min_midi).max(12) as f64;
        let usable_h = TIMELINE_H - 40.0;

        // Check which note is hovered
        let hovered_idx = self.check_phrase_note_hover(mx, my);

        for (i, note) in self.phrase_notes.iter().enumerate() {
            let x = x_offset + i as f64 * note_w;
            let y_ratio = (note.midi - min_midi) as f64 / range;
            let y = TIMELINE_Y + 22.0 + usable_h * (1.0 - y_ratio);

            let is_hovered = hovered_idx == Some(i);

            // Glow under (brighter on hover)
            let glow_alpha = if is_hovered { 100 } else { 40 };
            shapes::fill_rect(fb, x, y - 2.0, note_w - 4.0, 12.0,
                ACCENT_TEAL.with_alpha(glow_alpha));

            // Note block with color gradient
            let t = i as f64 / self.phrase_notes.len().max(1) as f64;
            let note_color = if note.lyric.is_some() {
                Color::lerp(ACCENT_RED, ACCENT_PINK, t)
            } else {
                Color::lerp(ACCENT_TEAL, ACCENT_PINK, t)
            };
            let block_color = if is_hovered {
                Color::lerp(note_color, Color::WHITE, 0.4)
            } else {
                note_color
            };
            let block_h = if is_hovered { 10.0 } else { 8.0 };
            let block_y = if is_hovered { y - 1.0 } else { y };
            shapes::fill_rect(fb, x + 1.0, block_y, note_w - 6.0, block_h, block_color);

            // Draw lyric label above note block if present
            if let Some(lyric) = note.lyric {
                let lw = text::text_width(lyric, 1);
                text::draw_text(fb, (x + note_w / 2.0) as i32 - lw / 2,
                    (y - 14.0) as i32, lyric, ACCENT_PINK, 1);
            }

            // Show note name on hover
            if is_hovered {
                let name = note_name(note.midi);
                let nw = text::text_width(name, 1);
                text::draw_text(fb, (x + note_w / 2.0) as i32 - nw / 2,
                    (y + block_h + 2.0) as i32, name, Color::WHITE, 1);
            }
        }

        // Playhead cursor
        let cursor_x = x_offset + self.phrase_notes.len() as f64 * note_w;
        let blink = ((self.pulse_time * 3.0).sin() * 0.5 + 0.5) * 255.0;
        shapes::draw_line(fb, cursor_x, TIMELINE_Y + 20.0, cursor_x,
            TIMELINE_Y + TIMELINE_H - 8.0,
            ACCENT_CYAN.with_alpha(blink as u8));
    }

    fn render_footer(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        shapes::draw_line(fb, 0.0, FOOTER_Y, SCREEN_W, FOOTER_Y, DIVIDER);

        let diff_str = format!("Difficulty: {}/10", self.difficulty);
        text::draw_text(fb, 16, (FOOTER_Y + 20.0) as i32, &diff_str, DIM_TEXT, 1);

        let concept_str = match &self.challenge.concept {
            MusicConcept::ChordProgression => "Mode: Chords",
            MusicConcept::NextNote => "Mode: Melody",
            MusicConcept::IntervalRecognition => "Mode: Intervals",
            MusicConcept::PhonemeRecognition => "Mode: Phonemes",
        };
        let cw = text::text_width(concept_str, 1);
        text::draw_text(fb, 600 - cw - 16, (FOOTER_Y + 20.0) as i32, concept_str, DIM_TEXT, 1);

        let phrase_str = format!("Phrase: {} notes", self.phrase_notes.len());
        text::draw_text_centered(fb, 300, (FOOTER_Y + 20.0) as i32, &phrase_str, DIM_TEXT, 1);

        // Attribution — fan tribute
        let attr = "Kasane Teto (c) TWINDRILL - kasaneteto.jp";
        text::draw_text_centered(fb, (SCREEN_W / 2.0) as i32, (FOOTER_Y + 36.0) as i32, attr,
            ACCENT_PINK.with_alpha(120), 1);
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
                    // 4-point star: two crossed lines
                    let half = size * 1.5;
                    shapes::draw_line(fb, s.x - half, s.y, s.x + half, s.y, color);
                    shapes::draw_line(fb, s.x, s.y - half, s.x, s.y + half, color);
                    shapes::fill_circle(fb, s.x, s.y, size * 0.5, color);
                }
                SparkleShape::Heart => {
                    // Simple heart: two overlapping circles + triangle
                    let r = size * 0.6;
                    shapes::fill_circle(fb, s.x - r * 0.5, s.y - r * 0.3, r, color);
                    shapes::fill_circle(fb, s.x + r * 0.5, s.y - r * 0.3, r, color);
                    shapes::fill_triangle(fb,
                        s.x - r * 1.2, s.y,
                        s.x + r * 1.2, s.y,
                        s.x, s.y + r * 1.5, color);
                }
                SparkleShape::Note => {
                    // Music note: circle head + stem line
                    shapes::fill_circle(fb, s.x, s.y, size * 0.7, color);
                    shapes::draw_line(fb, s.x + size * 0.6, s.y,
                        s.x + size * 0.6, s.y - size * 2.0, color);
                }
            }
        }
    }

    fn render_background_stars(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        // Sound energy drives stars toward white/bright glow
        let energy = self.sound_energy;

        for star in &self.background_stars {
            let twinkle = ((self.pulse_time * 1.5 + star.phase).sin() * 0.4 + 0.6) * 255.0;
            let base_alpha = twinkle.max(30.0).min(200.0);
            // Boost alpha and blend toward white when sound is playing
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
        // Top
        shapes::fill_rect(fb, 0.0, 0.0, SCREEN_W, strip, glow_color);
        // Bottom
        shapes::fill_rect(fb, 0.0, SCREEN_H - strip, SCREEN_W, strip, glow_color);
        // Left
        shapes::fill_rect(fb, 0.0, 0.0, strip, SCREEN_H, glow_color);
        // Right
        shapes::fill_rect(fb, SCREEN_W - strip, 0.0, strip, SCREEN_H, glow_color);
    }

    fn render_combo(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        if self.streak < 3 {
            return;
        }
        let cx = (SCREEN_W / 2.0) as i32;
        let combo_str = format!("{}x COMBO", self.streak);

        // Color escalation: teal (3+) → pink (5+) → gold (10+)
        let color = if self.streak >= 10 {
            ACCENT_GOLD
        } else if self.streak >= 5 {
            ACCENT_PINK
        } else {
            ACCENT_TEAL
        };

        // Pulsing glow effect
        let pulse = (self.pulse_time * 4.0).sin() * 0.3 + 0.7;
        let alpha = (pulse * 255.0) as u8;
        let glow_color = color.with_alpha(alpha / 3);

        // Draw glow behind
        text::draw_text_centered(fb, cx + 1, (CHALLENGE_Y - 8.0) as i32 + 1,
            &combo_str, glow_color, 2);
        // Draw main text
        text::draw_text_centered(fb, cx, (CHALLENGE_Y - 8.0) as i32,
            &combo_str, color.with_alpha(alpha), 2);
    }
}
