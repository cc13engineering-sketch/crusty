//! Music theory utilities for the Crusty game engine.
//!
//! Pure functions for MIDI note conversion, chord/scale construction,
//! and interval naming. No state — everything is computed from inputs.

/// The twelve chromatic note names in ascending semitone order.
pub const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

/// Semitone offsets for a major scale relative to the root.
pub const MAJOR_SCALE: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];

/// Quality / type of a chord.
#[derive(Clone, Debug, PartialEq)]
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Major7,
    Minor7,
    Dominant7,
}

/// Type of a musical scale.
#[derive(Clone, Debug, PartialEq)]
pub enum ScaleType {
    Major,
    NaturalMinor,
    HarmonicMinor,
    MelodicMinor,
    Pentatonic,
    Blues,
    Chromatic,
}

/// Converts a MIDI note number to its frequency in Hz.
///
/// Uses the standard tuning where A4 (MIDI 69) = 440 Hz.
pub fn note_freq(midi_note: u8) -> f64 {
    440.0 * 2.0_f64.powf((midi_note as f64 - 69.0) / 12.0)
}

/// Returns the note name (e.g. "C", "F#") for a MIDI note number.
pub fn note_name(midi_note: u8) -> &'static str {
    NOTE_NAMES[(midi_note % 12) as usize]
}

/// Returns the octave number for a MIDI note number.
///
/// MIDI 60 = C4, so the formula is `midi_note / 12 - 1`.
pub fn note_octave(midi_note: u8) -> i32 {
    (midi_note as i32) / 12 - 1
}

/// Returns the semitone intervals that define a chord quality.
pub fn chord_intervals(quality: &ChordQuality) -> &'static [u8] {
    match quality {
        ChordQuality::Major => &[0, 4, 7],
        ChordQuality::Minor => &[0, 3, 7],
        ChordQuality::Diminished => &[0, 3, 6],
        ChordQuality::Augmented => &[0, 4, 8],
        ChordQuality::Major7 => &[0, 4, 7, 11],
        ChordQuality::Minor7 => &[0, 3, 7, 10],
        ChordQuality::Dominant7 => &[0, 4, 7, 10],
    }
}

/// Returns the semitone intervals that define a scale type.
pub fn scale_intervals(scale_type: &ScaleType) -> &'static [u8] {
    match scale_type {
        ScaleType::Major => &[0, 2, 4, 5, 7, 9, 11],
        ScaleType::NaturalMinor => &[0, 2, 3, 5, 7, 8, 10],
        ScaleType::HarmonicMinor => &[0, 2, 3, 5, 7, 8, 11],
        ScaleType::MelodicMinor => &[0, 2, 3, 5, 7, 9, 11],
        ScaleType::Pentatonic => &[0, 2, 4, 7, 9],
        ScaleType::Blues => &[0, 3, 5, 6, 7, 10],
        ScaleType::Chromatic => &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    }
}

/// Returns the MIDI note numbers for a chord built on `root`.
pub fn chord_notes(root: u8, quality: &ChordQuality) -> Vec<u8> {
    chord_intervals(quality)
        .iter()
        .map(|interval| root + interval)
        .collect()
}

/// Returns the conventional name for an interval given in semitones.
pub fn interval_name(semitones: u8) -> &'static str {
    match semitones {
        0 => "Unison",
        1 => "Minor 2nd",
        2 => "Major 2nd",
        3 => "Minor 3rd",
        4 => "Major 3rd",
        5 => "Perfect 4th",
        6 => "Tritone",
        7 => "Perfect 5th",
        8 => "Minor 6th",
        9 => "Major 6th",
        10 => "Minor 7th",
        11 => "Major 7th",
        12 => "Octave",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── note_freq ──────────────────────────────────────────────

    #[test]
    fn a4_is_440_hz() {
        let freq = note_freq(69);
        assert!((freq - 440.0).abs() < 1e-10, "A4 should be 440 Hz, got {freq}");
    }

    #[test]
    fn middle_c_frequency() {
        // C4 = MIDI 60, expected ~261.626 Hz
        let freq = note_freq(60);
        assert!(
            (freq - 261.625_565_300_6).abs() < 0.001,
            "Middle C should be ~261.626 Hz, got {freq}"
        );
    }

    #[test]
    fn octave_doubles_frequency() {
        let f_low = note_freq(60);
        let f_high = note_freq(72);
        assert!(
            (f_high / f_low - 2.0).abs() < 1e-10,
            "Going up 12 semitones should double the frequency"
        );
    }

    // ── note_name ──────────────────────────────────────────────

    #[test]
    fn note_names_for_all_12() {
        let expected = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        for (i, name) in expected.iter().enumerate() {
            assert_eq!(
                note_name(i as u8 + 60),
                *name,
                "MIDI {} should be {}",
                i + 60,
                name
            );
        }
    }

    #[test]
    fn note_name_wraps_across_octaves() {
        assert_eq!(note_name(0), "C");
        assert_eq!(note_name(12), "C");
        assert_eq!(note_name(24), "C");
        assert_eq!(note_name(69), "A");
    }

    // ── note_octave ────────────────────────────────────────────

    #[test]
    fn c4_is_octave_4() {
        assert_eq!(note_octave(60), 4);
    }

    #[test]
    fn a4_is_octave_4() {
        assert_eq!(note_octave(69), 4);
    }

    #[test]
    fn c0_is_octave_negative_1() {
        // MIDI 0 = C(-1)
        assert_eq!(note_octave(0), -1);
    }

    // ── chord_intervals ────────────────────────────────────────

    #[test]
    fn major_chord_intervals() {
        assert_eq!(chord_intervals(&ChordQuality::Major), &[0, 4, 7]);
    }

    #[test]
    fn minor_chord_intervals() {
        assert_eq!(chord_intervals(&ChordQuality::Minor), &[0, 3, 7]);
    }

    #[test]
    fn diminished_chord_intervals() {
        assert_eq!(chord_intervals(&ChordQuality::Diminished), &[0, 3, 6]);
    }

    #[test]
    fn augmented_chord_intervals() {
        assert_eq!(chord_intervals(&ChordQuality::Augmented), &[0, 4, 8]);
    }

    #[test]
    fn major7_chord_intervals() {
        assert_eq!(chord_intervals(&ChordQuality::Major7), &[0, 4, 7, 11]);
    }

    #[test]
    fn minor7_chord_intervals() {
        assert_eq!(chord_intervals(&ChordQuality::Minor7), &[0, 3, 7, 10]);
    }

    #[test]
    fn dominant7_chord_intervals() {
        assert_eq!(chord_intervals(&ChordQuality::Dominant7), &[0, 4, 7, 10]);
    }

    // ── scale_intervals ────────────────────────────────────────

    #[test]
    fn major_scale_intervals() {
        assert_eq!(scale_intervals(&ScaleType::Major), &[0, 2, 4, 5, 7, 9, 11]);
    }

    #[test]
    fn natural_minor_scale_intervals() {
        assert_eq!(
            scale_intervals(&ScaleType::NaturalMinor),
            &[0, 2, 3, 5, 7, 8, 10]
        );
    }

    #[test]
    fn harmonic_minor_scale_intervals() {
        assert_eq!(
            scale_intervals(&ScaleType::HarmonicMinor),
            &[0, 2, 3, 5, 7, 8, 11]
        );
    }

    #[test]
    fn melodic_minor_scale_intervals() {
        assert_eq!(
            scale_intervals(&ScaleType::MelodicMinor),
            &[0, 2, 3, 5, 7, 9, 11]
        );
    }

    #[test]
    fn pentatonic_scale_intervals() {
        assert_eq!(scale_intervals(&ScaleType::Pentatonic), &[0, 2, 4, 7, 9]);
    }

    #[test]
    fn blues_scale_intervals() {
        assert_eq!(scale_intervals(&ScaleType::Blues), &[0, 3, 5, 6, 7, 10]);
    }

    #[test]
    fn chromatic_scale_intervals() {
        assert_eq!(
            scale_intervals(&ScaleType::Chromatic),
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        );
    }

    // ── chord_notes ────────────────────────────────────────────

    #[test]
    fn c_major_chord_notes() {
        // C4 major = C4, E4, G4 → MIDI 60, 64, 67
        assert_eq!(chord_notes(60, &ChordQuality::Major), vec![60, 64, 67]);
    }

    #[test]
    fn a_minor_chord_notes() {
        // A3 minor = A3, C4, E4 → MIDI 57, 60, 64
        assert_eq!(chord_notes(57, &ChordQuality::Minor), vec![57, 60, 64]);
    }

    #[test]
    fn g_dominant7_chord_notes() {
        // G3 dom7 = G3, B3, D4, F4 → MIDI 55, 59, 62, 65
        assert_eq!(
            chord_notes(55, &ChordQuality::Dominant7),
            vec![55, 59, 62, 65]
        );
    }

    // ── interval_name ──────────────────────────────────────────

    #[test]
    fn common_interval_names() {
        assert_eq!(interval_name(0), "Unison");
        assert_eq!(interval_name(1), "Minor 2nd");
        assert_eq!(interval_name(2), "Major 2nd");
        assert_eq!(interval_name(3), "Minor 3rd");
        assert_eq!(interval_name(4), "Major 3rd");
        assert_eq!(interval_name(5), "Perfect 4th");
        assert_eq!(interval_name(6), "Tritone");
        assert_eq!(interval_name(7), "Perfect 5th");
        assert_eq!(interval_name(8), "Minor 6th");
        assert_eq!(interval_name(9), "Major 6th");
        assert_eq!(interval_name(10), "Minor 7th");
        assert_eq!(interval_name(11), "Major 7th");
        assert_eq!(interval_name(12), "Octave");
    }

    #[test]
    fn unknown_interval_beyond_octave() {
        assert_eq!(interval_name(13), "Unknown");
        assert_eq!(interval_name(255), "Unknown");
    }

    // ── constants ──────────────────────────────────────────────

    #[test]
    fn major_scale_constant_matches_scale_intervals() {
        assert_eq!(&MAJOR_SCALE[..], scale_intervals(&ScaleType::Major));
    }

    #[test]
    fn note_names_constant_has_12_entries() {
        assert_eq!(NOTE_NAMES.len(), 12);
        assert_eq!(NOTE_NAMES[0], "C");
        assert_eq!(NOTE_NAMES[9], "A");
        assert_eq!(NOTE_NAMES[11], "B");
    }
}
