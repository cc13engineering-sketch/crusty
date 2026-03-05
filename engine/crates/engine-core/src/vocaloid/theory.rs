//! Pure music theory functions — no side effects, no World access.
//!
//! All functions operate on scale degrees (0–6) and MIDI note numbers (0–127).
//! Uses the major scale exclusively.

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

// ─── Japanese Phoneme Data (Kasane Teto CV voicebank) ─────────────

/// A Japanese CV phoneme entry: (hiragana, romaji, sample_name, vowel_class).
/// vowel_class maps to the base vowel for pitch reference: 0=a, 1=i, 2=u, 3=e, 4=o, 5=n.
#[derive(Clone, Debug)]
pub struct Phoneme {
    pub kana: &'static str,
    pub romaji: &'static str,
    pub sample: &'static str,
    pub vowel: u8,
}

/// The 5 Japanese vowels.
pub const VOWELS: [&str; 5] = ["a", "i", "u", "e", "o"];

/// Hiragana vowels.
pub const VOWELS_KANA: [&str; 5] = ["あ", "い", "う", "え", "お"];

/// Core CV phoneme table — the gojuuon (50 sounds) from Teto's voicebank.
/// Each row: consonant group + vowel variants.
pub const GOJUUON: &[Phoneme] = &[
    // Vowels (a-row)
    Phoneme { kana: "あ", romaji: "a",   sample: "a",   vowel: 0 },
    Phoneme { kana: "い", romaji: "i",   sample: "i",   vowel: 1 },
    Phoneme { kana: "う", romaji: "u",   sample: "u",   vowel: 2 },
    Phoneme { kana: "え", romaji: "e",   sample: "e",   vowel: 3 },
    Phoneme { kana: "お", romaji: "o",   sample: "o",   vowel: 4 },
    // Ka-row
    Phoneme { kana: "か", romaji: "ka",  sample: "ka",  vowel: 0 },
    Phoneme { kana: "き", romaji: "ki",  sample: "ki",  vowel: 1 },
    Phoneme { kana: "く", romaji: "ku",  sample: "ku",  vowel: 2 },
    Phoneme { kana: "け", romaji: "ke",  sample: "ke",  vowel: 3 },
    Phoneme { kana: "こ", romaji: "ko",  sample: "ko",  vowel: 4 },
    // Sa-row
    Phoneme { kana: "さ", romaji: "sa",  sample: "sa",  vowel: 0 },
    Phoneme { kana: "し", romaji: "shi", sample: "shi", vowel: 1 },
    Phoneme { kana: "す", romaji: "su",  sample: "su",  vowel: 2 },
    Phoneme { kana: "せ", romaji: "se",  sample: "se",  vowel: 3 },
    Phoneme { kana: "そ", romaji: "so",  sample: "so",  vowel: 4 },
    // Ta-row
    Phoneme { kana: "た", romaji: "ta",  sample: "ta",  vowel: 0 },
    Phoneme { kana: "ち", romaji: "chi", sample: "chi", vowel: 1 },
    Phoneme { kana: "つ", romaji: "tsu", sample: "tsu", vowel: 2 },
    Phoneme { kana: "て", romaji: "te",  sample: "te",  vowel: 3 },
    Phoneme { kana: "と", romaji: "to",  sample: "to",  vowel: 4 },
    // Na-row
    Phoneme { kana: "な", romaji: "na",  sample: "na",  vowel: 0 },
    Phoneme { kana: "に", romaji: "ni",  sample: "ni",  vowel: 1 },
    Phoneme { kana: "ぬ", romaji: "nu",  sample: "nu",  vowel: 2 },
    Phoneme { kana: "ね", romaji: "ne",  sample: "ne",  vowel: 3 },
    Phoneme { kana: "の", romaji: "no",  sample: "no",  vowel: 4 },
    // Ha-row
    Phoneme { kana: "は", romaji: "ha",  sample: "ha",  vowel: 0 },
    Phoneme { kana: "ひ", romaji: "hi",  sample: "hi",  vowel: 1 },
    Phoneme { kana: "ふ", romaji: "fu",  sample: "fu",  vowel: 2 },
    Phoneme { kana: "へ", romaji: "he",  sample: "he",  vowel: 3 },
    Phoneme { kana: "ほ", romaji: "ho",  sample: "ho",  vowel: 4 },
    // Ma-row
    Phoneme { kana: "ま", romaji: "ma",  sample: "ma",  vowel: 0 },
    Phoneme { kana: "み", romaji: "mi",  sample: "mi",  vowel: 1 },
    Phoneme { kana: "む", romaji: "mu",  sample: "mu",  vowel: 2 },
    Phoneme { kana: "め", romaji: "me",  sample: "me",  vowel: 3 },
    Phoneme { kana: "も", romaji: "mo",  sample: "mo",  vowel: 4 },
    // Ya-row
    Phoneme { kana: "や", romaji: "ya",  sample: "ya",  vowel: 0 },
    Phoneme { kana: "ゆ", romaji: "yu",  sample: "yu",  vowel: 2 },
    Phoneme { kana: "よ", romaji: "yo",  sample: "yo",  vowel: 4 },
    // Ra-row
    Phoneme { kana: "ら", romaji: "ra",  sample: "ra",  vowel: 0 },
    Phoneme { kana: "り", romaji: "ri",  sample: "ri",  vowel: 1 },
    Phoneme { kana: "る", romaji: "ru",  sample: "ru",  vowel: 2 },
    Phoneme { kana: "れ", romaji: "re",  sample: "re",  vowel: 3 },
    Phoneme { kana: "ろ", romaji: "ro",  sample: "ro",  vowel: 4 },
    // Wa-row + n
    Phoneme { kana: "わ", romaji: "wa",  sample: "wa",  vowel: 0 },
    Phoneme { kana: "ん", romaji: "n",   sample: "n",   vowel: 5 },
];

/// Consonant group names in Japanese order (for display).
pub const CONSONANT_ROWS: &[&str] = &[
    "あ行", "か行", "さ行", "た行", "な行",
    "は行", "ま行", "や行", "ら行", "わ行",
];

/// Get the consonant row index for a gojuuon phoneme index.
pub fn phoneme_row(idx: usize) -> usize {
    if idx < 5 { 0 }       // a-row
    else if idx < 10 { 1 } // ka
    else if idx < 15 { 2 } // sa
    else if idx < 20 { 3 } // ta
    else if idx < 25 { 4 } // na
    else if idx < 30 { 5 } // ha
    else if idx < 35 { 6 } // ma
    else if idx < 38 { 7 } // ya (3)
    else if idx < 43 { 8 } // ra
    else { 9 }             // wa+n
}

/// Generate phoneme distractor options.
/// Returns indices into GOJUUON including the answer.
pub fn generate_phoneme_options(answer_idx: usize, count: usize, seed: u64) -> Vec<usize> {
    let mut options = vec![answer_idx];
    let mut s = seed;
    let total = GOJUUON.len();

    // Prefer distractors from the same vowel class or consonant row
    let answer_vowel = GOJUUON[answer_idx].vowel;
    let answer_row = phoneme_row(answer_idx);

    // Collect same-vowel and same-row candidates
    let mut candidates: Vec<usize> = (0..total)
        .filter(|&i| i != answer_idx)
        .filter(|&i| GOJUUON[i].vowel == answer_vowel || phoneme_row(i) == answer_row)
        .collect();

    // Shuffle candidates
    for i in (1..candidates.len()).rev() {
        s = xorshift(s);
        let j = (s % (i as u64 + 1)) as usize;
        candidates.swap(i, j);
    }

    for &c in candidates.iter().take(count - 1) {
        options.push(c);
    }

    // Fill remaining from full pool if needed
    if options.len() < count {
        let mut all: Vec<usize> = (0..total)
            .filter(|i| !options.contains(i))
            .collect();
        for i in (1..all.len()).rev() {
            s = xorshift(s);
            let j = (s % (i as u64 + 1)) as usize;
            all.swap(i, j);
        }
        for &a in all.iter().take(count - options.len()) {
            options.push(a);
        }
    }

    // Truncate and shuffle
    options.truncate(count);
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
        // C3 = 48, degree 0 (tonic) = C3
        assert_eq!(degree_to_midi(48, 0), 48);
        // degree 4 (dominant) = G3 = 55
        assert_eq!(degree_to_midi(48, 4), 55);
        // degree 6 (leading tone) = B3 = 59
        assert_eq!(degree_to_midi(48, 6), 59);
    }

    #[test]
    fn white_key_mapping() {
        assert_eq!(white_key_to_midi(0), 48);  // C3
        assert_eq!(white_key_to_midi(7), 60);  // C4
        assert_eq!(white_key_to_midi(14), 72); // C5
    }

    #[test]
    fn is_white_key_correct() {
        assert!(is_white_key(48));   // C
        assert!(!is_white_key(49));  // C#
        assert!(is_white_key(50));   // D
        assert!(!is_white_key(51));  // D#
        assert!(is_white_key(52));   // E
        assert!(is_white_key(53));   // F
        assert!(!is_white_key(54));  // F#
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
            assert_eq!(ci[0], 0); // root always 0
            assert!(ci[1] == 3 || ci[1] == 4); // minor or major third
            assert!(ci[2] == 6 || ci[2] == 7); // diminished or perfect fifth
        }
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
