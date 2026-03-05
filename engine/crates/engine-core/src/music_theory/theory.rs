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

// ─── Cadence Types ──────────────────────────────────────────────────

/// Cadence types for the cadence identification challenge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CadenceType {
    Authentic,  // V → I
    Plagal,     // IV → I
    Half,       // X → V
    Deceptive,  // V → vi
}

/// All cadence types used in challenges.
pub const CADENCE_TYPES: [CadenceType; 4] = [
    CadenceType::Authentic,
    CadenceType::Plagal,
    CadenceType::Half,
    CadenceType::Deceptive,
];

/// Display name for a cadence type.
pub fn cadence_name(c: CadenceType) -> &'static str {
    match c {
        CadenceType::Authentic => "Authentic",
        CadenceType::Plagal    => "Plagal",
        CadenceType::Half      => "Half",
        CadenceType::Deceptive => "Deceptive",
    }
}

/// The two chord degrees that define a cadence (setup, resolution).
pub fn cadence_chords(c: CadenceType) -> (u8, u8) {
    match c {
        CadenceType::Authentic => (4, 0),  // V → I
        CadenceType::Plagal    => (3, 0),  // IV → I
        CadenceType::Half      => (1, 4),  // ii → V
        CadenceType::Deceptive => (4, 5),  // V → vi
    }
}

/// Insight text for a correctly identified cadence type.
pub fn cadence_insight(c: CadenceType) -> &'static str {
    match c {
        CadenceType::Authentic => "V to I -- the strongest resolution in tonal music. The leading tone (7th degree) pulls up a half step to the tonic, while the 5th of V drops to the root. This is the musical equivalent of a full stop. Nearly every classical piece ends this way.",
        CadenceType::Plagal    => "IV to I -- the 'Amen' cadence, heard at the end of hymns for centuries. The warm subdominant settles gently onto the tonic without the urgency of V. Common in gospel, folk, and rock. The Beatles loved it.",
        CadenceType::Half      => "Ending on V -- a musical question mark. The phrase pauses on the dominant, leaving maximum tension. It demands a continuation. Composers use half cadences mid-phrase to keep the listener engaged and anticipating resolution.",
        CadenceType::Deceptive => "V to vi -- the great harmonic bait-and-switch. Your ear expects the tonic but lands on its relative minor instead. The surprise creates emotional depth. Beethoven and Radiohead both use deceptive cadences to subvert expectations.",
    }
}

// ─── Learning Resources ─────────────────────────────────────────────

/// A learning resource link for a music theory concept.
pub struct LearnLink {
    pub label: &'static str,
    pub url: &'static str,
}

/// Learning resources keyed by challenge concept index:
/// 0 = Scale Degree, 1 = Roman Numeral, 2 = Intervals, 3 = Chord Quality, 4 = Cadences
pub fn learning_resources(concept_idx: u8) -> &'static [LearnLink] {
    match concept_idx {
        0 => &[
            LearnLink { label: "musictheory.net: Scale Degrees", url: "https://www.musictheory.net/lessons/21" },
            LearnLink { label: "Open Music Theory: Scales", url: "https://viva.pressbooks.pub/openmusictheory/chapter/scales-and-scale-degrees/" },
        ],
        1 => &[
            LearnLink { label: "musictheory.net: Diatonic Chords", url: "https://www.musictheory.net/lessons/43" },
            LearnLink { label: "Open Music Theory: Harmony", url: "https://viva.pressbooks.pub/openmusictheory/chapter/introduction-to-harmony-chords-and-basic-tonal-progressions/" },
        ],
        2 => &[
            LearnLink { label: "musictheory.net: Intervals", url: "https://www.musictheory.net/lessons/30" },
            LearnLink { label: "teoria.com: Interval Ear Training", url: "https://www.teoria.com/en/exercises/ie.php" },
        ],
        3 => &[
            LearnLink { label: "musictheory.net: Triads", url: "https://www.musictheory.net/lessons/40" },
            LearnLink { label: "Open Music Theory: Triads", url: "https://viva.pressbooks.pub/openmusictheory/chapter/triads/" },
        ],
        4 => &[
            LearnLink { label: "musictheory.net: Cadences", url: "https://www.musictheory.net/lessons/55" },
            LearnLink { label: "Open Music Theory: Cadences", url: "https://viva.pressbooks.pub/openmusictheory/chapter/cadences/" },
        ],
        _ => &[],
    }
}

// ─── Educational Insights ──────────────────────────────────────────

/// Insight text for a correctly identified scale degree.
pub fn degree_insight(degree: u8) -> &'static str {
    match degree % 7 {
        0 => "Tonic (1st) -- home base. Every melody wants to come back here. It is the gravitational center of the key. When you hear a song 'end,' it almost always lands on the tonic.",
        1 => "Supertonic (2nd) -- one step above home. It has a 'pre-dominant' function, meaning it naturally leads toward the dominant. The ii-V-I progression is the backbone of jazz.",
        2 => "Mediant (3rd) -- the bridge between tonic and dominant. It defines whether the key sounds major (bright) or minor (dark). This single note shapes the entire mood.",
        3 => "Subdominant (4th) -- the plagal sound. Think of the 'Amen' at the end of a hymn. It creates gentle motion away from the tonic without the urgency of the dominant.",
        4 => "Dominant (5th) -- maximum tension, maximum pull. It yearns to resolve back to the tonic. The V-I motion is the most fundamental harmonic relationship in Western music.",
        5 => "Submediant (6th) -- the emotional twin of the tonic. It is the relative minor's root, so shifting to vi gives a bittersweet shadow of the major key. Pop songs use the I-V-vi-IV loop constantly.",
        6 => "Leading tone (7th) -- the most unstable degree, just a half step below tonic. It pulls upward with an almost physical urgency. Without it, dominant chords would lose their drive.",
        _ => "",
    }
}

/// Insight text for a correctly identified Roman numeral chord.
pub fn numeral_insight(degree: u8) -> &'static str {
    match degree % 7 {
        0 => "I -- the tonic triad. The harmonic center of gravity. Every progression eventually resolves here. It is stable, restful, and final. Major key songs often begin and end on I.",
        1 => "ii -- supertonic minor. The classic setup chord for V in the ii-V-I turnaround. Jazz musicians call it the 'predominant.' Its minor quality adds a touch of tension before the dominant.",
        2 => "iii -- mediant minor. It shares two notes with I and two with V, making it a chameleon chord. It can substitute for I or V depending on context. Relatively rare in pop, common in classical.",
        3 => "IV -- subdominant major. Foundation of plagal (IV-I) cadences and the four-chord pop loop. It provides harmonic motion without the intensity of V. Think of it as gentle departure from home.",
        4 => "V -- the dominant major. Creates the strongest gravitational pull back to I. Its major third is the leading tone of the key, which resolves up by half step. The engine of tonal harmony.",
        5 => "vi -- submediant minor. The gateway to the relative minor key. Starting a progression on vi instead of I gives a melancholic flavor. The 'sad' version of the I-V-vi-IV loop starts here.",
        6 => "vii* -- the leading-tone diminished triad. Both intervals are minor thirds, creating instability from every angle. It naturally resolves to I and often functions as a rootless V7 chord.",
        _ => "",
    }
}

/// Insight text for a correctly identified interval.
pub fn interval_insight(semitones: u8) -> &'static str {
    match semitones {
        0  => "Unison (P1) -- the same pitch. Two voices on the same note create power through unity. Gregorian chant and unison riffs in rock both rely on this raw, reinforcing effect.",
        1  => "Minor 2nd (m2) -- one half step. The 'Jaws' theme interval. It is the most dissonant interval, creating grinding tension. In melody, it drives chromatic approach tones and suspense.",
        2  => "Major 2nd (M2) -- one whole step. 'Happy Birthday' starts with this. It is the basic building block of scales. Two whole steps in a row form the major sound; the lack of one signals minor.",
        3  => "Minor 3rd (m3) -- the interval that defines minor chords and minor keys. 'Greensleeves' opens with this leap. It sounds darker and more introspective than its major cousin.",
        4  => "Major 3rd (M3) -- the interval that defines major chords. 'When the Saints Go Marching In' opens with this bright, confident leap. The difference between m3 and M3 is just one half step, but it changes everything.",
        5  => "Perfect 4th (P4) -- 'Here Comes the Bride.' Perfectly consonant and open. It is the inversion of the perfect 5th. In medieval music it was considered a consonance; in common practice, context matters.",
        6  => "Tritone (A4/d5) -- historically called 'diabolus in musica.' 'Maria' from West Side Story starts here. Exactly half an octave, it divides the scale symmetrically. It creates the tension in dominant 7th chords.",
        7  => "Perfect 5th (P5) -- 'Star Wars' theme opening. The most consonant interval after unison and octave. Power chords in rock are just root and fifth. It is the foundation of the overtone series.",
        8  => "Minor 6th (m6) -- 'The Entertainer' by Joplin. A wide, bittersweet leap. It is the inversion of the major 3rd. In a minor key, this interval appears naturally between the root and the 6th degree.",
        9  => "Major 6th (M6) -- 'My Bonnie Lies Over the Ocean.' Warm, lyrical, and open. It is the inversion of the minor 3rd. Jazz musicians love added 6th chords for their smooth, mellow quality.",
        10 => "Minor 7th (m7) -- 'Somewhere' from West Side Story. This bluesy interval is the heart of dominant 7th and minor 7th chords. It adds soulful tension without the extreme pull of the major 7th.",
        11 => "Major 7th (M7) -- 'Take On Me' by a-ha. Just one half step short of an octave, it creates a dreamy, floating dissonance. Major 7th chords are the signature sound of jazz ballads and bossa nova.",
        12 => "Octave (P8) -- 'Somewhere Over the Rainbow.' The purest interval besides unison. The frequency ratio is exactly 2:1. Every note in every octave shares the same letter name for this reason.",
        _  => "",
    }
}

/// Insight text for a correctly identified chord quality.
pub fn quality_insight(q: ChordQuality) -> &'static str {
    match q {
        ChordQuality::Major      => "Major triad: root + major 3rd + perfect 5th. The bright, stable, 'happy' sound. It appears naturally on scale degrees I, IV, and V in any major key. The foundation of Western harmony.",
        ChordQuality::Minor      => "Minor triad: root + minor 3rd + perfect 5th. The darker, more introspective sound. Only one note differs from major (the 3rd is lowered by a half step), but the emotional shift is dramatic.",
        ChordQuality::Diminished => "Diminished triad: root + minor 3rd + diminished 5th. Two stacked minor thirds create maximum instability. It naturally occurs on the 7th degree of major keys and desperately wants to resolve outward.",
        ChordQuality::Augmented  => "Augmented triad: root + major 3rd + augmented 5th. Two stacked major thirds create a symmetrical, dreamlike chord. It divides the octave into three equal parts, making it tonally ambiguous and mysterious.",
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
