//! Pure music theory functions — no side effects, no World access.
//!
//! All functions operate on scale degrees (0–6), MIDI note numbers (0–127),
//! and chord quality identifiers. Uses the major scale exclusively.

/// Semitone offsets for each degree of the major scale.
pub const MAJOR_SCALE: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];

/// Roman numeral labels for each scale degree.
pub const DEGREE_NAMES: [&str; 7] = ["I", "ii", "iii", "IV", "V", "vi", "vii"];

/// Interval names for 0–12 semitones.
pub const INTERVAL_NAMES: [&str; 13] = [
    "P1", "m2", "M2", "m3", "M3", "P4", "TT", "P5", "m6", "M6", "m7", "M7", "P8",
];

/// Note names within an octave (sharp notation).
pub const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

// ─── Chord Quality ──────────────────────────────────────────────────

/// Chord quality types for the chord quality identification challenge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Augmented,
}

/// All chord qualities used in challenges.
pub const CHORD_QUALITIES: [ChordQuality; 4] = [
    ChordQuality::Major,
    ChordQuality::Minor,
    ChordQuality::Diminished,
    ChordQuality::Augmented,
];

/// Display name for a chord quality.
pub fn chord_quality_name(q: ChordQuality) -> &'static str {
    match q {
        ChordQuality::Major => "Major",
        ChordQuality::Minor => "Minor",
        ChordQuality::Diminished => "Dim",
        ChordQuality::Augmented => "Aug",
    }
}

/// Triad intervals (semitones above root) for a chord quality.
pub fn quality_intervals(q: ChordQuality) -> [u8; 3] {
    match q {
        ChordQuality::Major      => [0, 4, 7],
        ChordQuality::Minor      => [0, 3, 7],
        ChordQuality::Diminished => [0, 3, 6],
        ChordQuality::Augmented  => [0, 4, 8],
    }
}

// ─── Learning Resources ─────────────────────────────────────────────

/// A learning resource link for a music theory concept.
pub struct LearnLink {
    pub label: &'static str,
    pub url: &'static str,
}

/// Learning resources keyed by challenge concept index:
/// 0 = Chord Progressions, 1 = Melody, 2 = Intervals, 3 = Chord Quality
pub fn learning_resources(concept_idx: u8) -> &'static [LearnLink] {
    match concept_idx {
        0 => &[
            LearnLink { label: "musictheory.net: Progressions", url: "https://www.musictheory.net/lessons/57" },
            LearnLink { label: "Open Music Theory: Harmony", url: "https://viva.pressbooks.pub/openmusictheory/chapter/introduction-to-harmony-chords-and-basic-tonal-progressions/" },
        ],
        1 => &[
            LearnLink { label: "musictheory.net: Major Scale", url: "https://www.musictheory.net/lessons/21" },
            LearnLink { label: "Open Music Theory: Melody", url: "https://viva.pressbooks.pub/openmusictheory/chapter/melody/" },
        ],
        2 => &[
            LearnLink { label: "musictheory.net: Intervals", url: "https://www.musictheory.net/lessons/30" },
            LearnLink { label: "teoria.com: Interval Ear Training", url: "https://www.teoria.com/en/exercises/ie.php" },
        ],
        3 => &[
            LearnLink { label: "musictheory.net: Triads", url: "https://www.musictheory.net/lessons/40" },
            LearnLink { label: "Open Music Theory: Triads", url: "https://viva.pressbooks.pub/openmusictheory/chapter/triads/" },
        ],
        _ => &[],
    }
}

// ─── Diatonic Theory ────────────────────────────────────────────────

/// Triad intervals (semitones above chord root) for each scale degree in a major key.
pub fn chord_intervals(degree: u8) -> [u8; 3] {
    match degree % 7 {
        0 => [0, 4, 7],  // I   major
        1 => [0, 3, 7],  // ii  minor
        2 => [0, 3, 7],  // iii minor
        3 => [0, 4, 7],  // IV  major
        4 => [0, 4, 7],  // V   major
        5 => [0, 3, 7],  // vi  minor
        6 => [0, 3, 6],  // vii° diminished
        _ => [0, 4, 7],
    }
}

/// Common chord progressions: which degrees typically follow a given degree.
pub fn likely_next_degrees(degree: u8) -> &'static [u8] {
    match degree % 7 {
        0 => &[3, 4, 5, 1],  // I  → IV, V, vi, ii
        1 => &[4, 6],         // ii → V, vii°
        2 => &[5, 3],         // iii→ vi, IV
        3 => &[0, 4, 1],      // IV → I, V, ii
        4 => &[0, 5],         // V  → I, vi
        5 => &[1, 3, 4],      // vi → ii, IV, V
        6 => &[0],            // vii°→ I
        _ => &[0],
    }
}

/// Convert MIDI note number to frequency in Hz (A4 = 440 Hz).
pub fn midi_to_freq(midi: u8) -> f64 {
    440.0 * 2.0_f64.powf((midi as f64 - 69.0) / 12.0)
}

/// Convert scale degree (0–6) to MIDI note number given a root MIDI note.
pub fn degree_to_midi(root_midi: u8, degree: u8) -> u8 {
    let octave_offset = (degree / 7) * 12;
    root_midi + octave_offset + MAJOR_SCALE[(degree % 7) as usize]
}

/// Check if a MIDI note is a white key on a standard piano.
pub fn is_white_key(midi: u8) -> bool {
    matches!(midi % 12, 0 | 2 | 4 | 5 | 7 | 9 | 11)
}

/// Get the note name for a MIDI number.
pub fn note_name(midi: u8) -> &'static str {
    NOTE_NAMES[(midi % 12) as usize]
}

/// Map white key index (0–14 for C3–C5) to MIDI note number.
pub fn white_key_to_midi(index: u8) -> u8 {
    let octave = index / 7;
    let degree = index % 7;
    48 + octave * 12 + MAJOR_SCALE[degree as usize]
}

/// Generate distractor options for a challenge.
/// Returns a shuffled Vec containing the answer and (count-1) distractors.
pub fn generate_options(answer: u8, pool: &[u8], count: usize, seed: u64) -> Vec<u8> {
    let mut options = vec![answer];
    let mut s = seed;

    // Collect pool items that aren't the answer
    let mut distractors: Vec<u8> = pool.iter().copied().filter(|&x| x != answer).collect();

    // Fisher-Yates shuffle the distractors using our seed
    for i in (1..distractors.len()).rev() {
        s = xorshift(s);
        let j = (s % (i as u64 + 1)) as usize;
        distractors.swap(i, j);
    }

    // Take (count-1) distractors
    for &d in distractors.iter().take(count - 1) {
        options.push(d);
    }

    // Pad with random values if not enough distractors
    while options.len() < count {
        s = xorshift(s);
        let v = (s % 7) as u8;
        if !options.contains(&v) {
            options.push(v);
        }
    }

    // Shuffle the final options
    for i in (1..options.len()).rev() {
        s = xorshift(s);
        let j = (s % (i as u64 + 1)) as usize;
        options.swap(i, j);
    }

    options
}

/// Deterministic xorshift64 for shuffling.
pub fn xorshift(mut s: u64) -> u64 {
    if s == 0 {
        s = 1;
    }
    s ^= s << 13;
    s ^= s >> 7;
    s ^= s << 17;
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn major_scale_correct() {
        assert_eq!(MAJOR_SCALE, [0, 2, 4, 5, 7, 9, 11]);
    }

    #[test]
    fn midi_to_freq_a4() {
        let freq = midi_to_freq(69);
        assert!((freq - 440.0).abs() < 0.01);
    }

    #[test]
    fn midi_to_freq_c4() {
        let freq = midi_to_freq(60);
        assert!((freq - 261.63).abs() < 0.1);
    }

    #[test]
    fn degree_to_midi_c_major() {
        assert_eq!(degree_to_midi(48, 0), 48);
        assert_eq!(degree_to_midi(48, 4), 55);
        assert_eq!(degree_to_midi(48, 6), 59);
    }

    #[test]
    fn white_key_mapping() {
        assert_eq!(white_key_to_midi(0), 48);
        assert_eq!(white_key_to_midi(7), 60);
        assert_eq!(white_key_to_midi(14), 72);
    }

    #[test]
    fn is_white_key_correct() {
        assert!(is_white_key(48));
        assert!(!is_white_key(49));
        assert!(is_white_key(50));
        assert!(!is_white_key(51));
        assert!(is_white_key(52));
        assert!(is_white_key(53));
        assert!(!is_white_key(54));
    }

    #[test]
    fn generate_options_includes_answer() {
        let opts = generate_options(3, &[0, 1, 2, 3, 4, 5, 6], 4, 42);
        assert_eq!(opts.len(), 4);
        assert!(opts.contains(&3));
    }

    #[test]
    fn generate_options_deterministic() {
        let a = generate_options(2, &[0, 1, 2, 3, 4, 5], 4, 100);
        let b = generate_options(2, &[0, 1, 2, 3, 4, 5], 4, 100);
        assert_eq!(a, b);
    }

    #[test]
    fn likely_next_degrees_non_empty() {
        for deg in 0..7 {
            assert!(!likely_next_degrees(deg).is_empty());
        }
    }

    #[test]
    fn chord_intervals_valid() {
        for deg in 0..7 {
            let ci = chord_intervals(deg);
            assert_eq!(ci[0], 0);
            assert!(ci[1] == 3 || ci[1] == 4);
            assert!(ci[2] == 6 || ci[2] == 7);
        }
    }

    #[test]
    fn chord_quality_intervals_valid() {
        assert_eq!(quality_intervals(ChordQuality::Major), [0, 4, 7]);
        assert_eq!(quality_intervals(ChordQuality::Minor), [0, 3, 7]);
        assert_eq!(quality_intervals(ChordQuality::Diminished), [0, 3, 6]);
        assert_eq!(quality_intervals(ChordQuality::Augmented), [0, 4, 8]);
    }

    #[test]
    fn xorshift_nonzero() {
        let mut s = 1u64;
        for _ in 0..100 {
            s = xorshift(s);
            assert_ne!(s, 0);
        }
    }
}
