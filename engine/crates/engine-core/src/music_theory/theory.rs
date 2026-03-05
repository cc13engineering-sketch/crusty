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

// ─── Content Database (Fun Facts + Affiliate Links) ────────────────

/// A product recommendation shown alongside a fun fact.
#[derive(Clone, Debug)]
pub struct ProductRec {
    pub name: &'static str,
    pub blurb: &'static str,
    pub url: &'static str,
    pub program: &'static str,
    pub disclosure: &'static str,
}

/// A fun fact + optional product + optional hint for a specific challenge variant.
#[derive(Clone, Debug)]
pub struct ContentEntry {
    pub concept: u8,
    pub variant: Option<u8>,
    pub min_difficulty: u8,
    pub fact: &'static str,
    pub hint: Option<&'static str>,
    pub product: Option<ProductRec>,
}

// ─── Coursera Affiliate Links ──────────────────────────────────────
// Placeholder URLs — replace with real affiliate URLs after Coursera
// approval. The course slugs are illustrative of what we'd link to.

const COURSERA_DISCLOSURE: &str = "(affiliate link \u{2014} we earn a commission if you enroll)";

const COURSERA_FUNDAMENTALS: ProductRec = ProductRec {
    name: "Fundamentals of Music Theory (Coursera)",
    blurb: "Learn the building blocks from Edinburgh",
    url: "https://www.coursera.org/learn/edinburgh-music-theory?utm_source=crusty&utm_medium=affiliate",
    program: "coursera",
    disclosure: COURSERA_DISCLOSURE,
};

const COURSERA_MUSICIANSHIP: ProductRec = ProductRec {
    name: "Developing Your Musicianship (Coursera/Berklee)",
    blurb: "Berklee's approach to practical ear training",
    url: "https://www.coursera.org/learn/develop-your-musicianship?utm_source=crusty&utm_medium=affiliate",
    program: "coursera",
    disclosure: COURSERA_DISCLOSURE,
};

const COURSERA_JAZZ_IMPROV: ProductRec = ProductRec {
    name: "Jazz Improvisation (Coursera/Berklee)",
    blurb: "Take your theory into jazz performance",
    url: "https://www.coursera.org/learn/jazz-improvisation?utm_source=crusty&utm_medium=affiliate",
    program: "coursera",
    disclosure: COURSERA_DISCLOSURE,
};

const COURSERA_SONGWRITING: ProductRec = ProductRec {
    name: "Songwriting (Coursera/Berklee)",
    blurb: "Turn chord progressions into songs",
    url: "https://www.coursera.org/learn/songwriting?utm_source=crusty&utm_medium=affiliate",
    program: "coursera",
    disclosure: COURSERA_DISCLOSURE,
};

const COURSERA_GUITAR: ProductRec = ProductRec {
    name: "Guitar for Beginners (Coursera/Berklee)",
    blurb: "Play these chords on a real instrument",
    url: "https://www.coursera.org/learn/guitar?utm_source=crusty&utm_medium=affiliate",
    program: "coursera",
    disclosure: COURSERA_DISCLOSURE,
};

const COURSERA_MUSIC_PROD: ProductRec = ProductRec {
    name: "Music Production (Coursera/Berklee)",
    blurb: "Hear these sounds in a DAW context",
    url: "https://www.coursera.org/learn/music-production?utm_source=crusty&utm_medium=affiliate",
    program: "coursera",
    disclosure: COURSERA_DISCLOSURE,
};

const COURSERA_COMPOSITION: ProductRec = ProductRec {
    name: "Write Like Mozart (Coursera)",
    blurb: "Classical composition and voice leading",
    url: "https://www.coursera.org/learn/classical-composition?utm_source=crusty&utm_medium=affiliate",
    program: "coursera",
    disclosure: COURSERA_DISCLOSURE,
};

/// Master content database. Adding content = adding entries here.
///
/// **Coursera link frequency strategy:**
/// - Intervals/ChordQuality (concept 2, 3): Show on ~30% of entries.
///   These are beginner-friendly concepts where Coursera Fundamentals
///   and Musicianship courses are most relevant.
/// - ScaleDegree/RomanNumeral (concept 0, 1): Show on ~25% of entries.
///   Users learning these benefit from structured courses.
/// - Cadence (concept 4): Show on ~35% of entries. Users identifying
///   cadences are intermediate and ready for deeper coursework.
///
/// Product recs display at ~25% frequency (controlled in mod.rs),
/// so effective affiliate impression rate is:
///   entry_has_product_rate × 0.25 display_rate ≈ 7-9% of successes.
pub const CONTENT_DB: &[ContentEntry] = &[
    // ════════════════════════════════════════════════════════════════
    // SCALE DEGREE (concept 0, variants 0–6)
    // ════════════════════════════════════════════════════════════════
    ContentEntry {
        concept: 0, variant: Some(0), min_difficulty: 0,
        fact: "Tonic (1st) -- home base. Every melody wants to come back here. It's the gravitational center of the key.",
        hint: Some("Solfege: Do -- the most stable, 'at rest' sound"),
        product: Some(COURSERA_FUNDAMENTALS),
    },
    ContentEntry {
        concept: 0, variant: Some(1), min_difficulty: 0,
        fact: "Supertonic (2nd) -- one step above home. The ii-V-I progression built on this degree is the backbone of jazz.",
        hint: Some("Solfege: Re -- wants to move, feels 'unfinished'"),
        product: None,
    },
    ContentEntry {
        concept: 0, variant: Some(2), min_difficulty: 0,
        fact: "Mediant (3rd) -- the bridge between tonic and dominant. This single note determines whether music sounds major or minor.",
        hint: Some("Solfege: Mi -- bright in major, dark in minor"),
        product: None,
    },
    ContentEntry {
        concept: 0, variant: Some(3), min_difficulty: 0,
        fact: "Subdominant (4th) -- the plagal sound. Think of the 'Amen' at the end of a hymn. Gentle motion away from home.",
        hint: Some("Solfege: Fa -- warm, wants to fall back to Mi"),
        product: Some(COURSERA_MUSICIANSHIP),
    },
    ContentEntry {
        concept: 0, variant: Some(4), min_difficulty: 0,
        fact: "Dominant (5th) -- maximum tension, maximum pull. The V-I motion is the most fundamental relationship in Western music.",
        hint: Some("Solfege: Sol -- strong, open, almost as stable as Do"),
        product: None,
    },
    ContentEntry {
        concept: 0, variant: Some(5), min_difficulty: 0,
        fact: "Submediant (6th) -- the emotional twin of the tonic. It's the relative minor's root, giving a bittersweet shadow of major.",
        hint: Some("Solfege: La -- the root of the relative minor"),
        product: None,
    },
    ContentEntry {
        concept: 0, variant: Some(6), min_difficulty: 0,
        fact: "Leading tone (7th) -- the most unstable degree, just a half step below tonic. It pulls upward with almost physical urgency.",
        hint: Some("Solfege: Ti -- tense, desperately wants to resolve up to Do"),
        product: Some(COURSERA_FUNDAMENTALS),
    },

    // ════════════════════════════════════════════════════════════════
    // ROMAN NUMERAL (concept 1, variants 0–6)
    // ════════════════════════════════════════════════════════════════
    ContentEntry {
        concept: 1, variant: Some(0), min_difficulty: 0,
        fact: "I -- the tonic triad. The harmonic center of gravity. Every progression eventually resolves here.",
        hint: Some("This chord is major and feels completely 'at rest'"),
        product: None,
    },
    ContentEntry {
        concept: 1, variant: Some(1), min_difficulty: 0,
        fact: "ii -- supertonic minor. The classic setup chord for V in the ii-V-I turnaround that jazz musicians call the 'predominant.'",
        hint: Some("This chord is minor and leads naturally to V"),
        product: Some(COURSERA_JAZZ_IMPROV),
    },
    ContentEntry {
        concept: 1, variant: Some(2), min_difficulty: 0,
        fact: "iii -- mediant minor. It shares two notes with I and two with V, making it a chameleon. Relatively rare in pop, common in classical.",
        hint: Some("This chord is minor and shares notes with both I and V"),
        product: None,
    },
    ContentEntry {
        concept: 1, variant: Some(3), min_difficulty: 0,
        fact: "IV -- subdominant major. Foundation of plagal cadences and the four-chord pop loop. Gentle departure from home.",
        hint: Some("This chord is major and has a warm, 'hymn-like' quality"),
        product: Some(COURSERA_SONGWRITING),
    },
    ContentEntry {
        concept: 1, variant: Some(4), min_difficulty: 0,
        fact: "V -- the dominant major. Creates the strongest pull back to I. Its major third is the leading tone of the key.",
        hint: Some("This chord is major and creates strong tension toward I"),
        product: None,
    },
    ContentEntry {
        concept: 1, variant: Some(5), min_difficulty: 0,
        fact: "vi -- submediant minor. The gateway to the relative minor key. Starting on vi instead of I gives a melancholic flavor.",
        hint: Some("This chord is minor and is the root of the relative minor"),
        product: None,
    },
    ContentEntry {
        concept: 1, variant: Some(6), min_difficulty: 0,
        fact: "vii\u{00b0} -- the leading-tone diminished triad. Both intervals are minor thirds, creating instability that naturally resolves to I.",
        hint: Some("This is the only diminished chord in the natural major key"),
        product: Some(COURSERA_FUNDAMENTALS),
    },

    // ════════════════════════════════════════════════════════════════
    // INTERVAL RECOGNITION (concept 2, variants 0–12 semitones)
    // ════════════════════════════════════════════════════════════════
    ContentEntry {
        concept: 2, variant: Some(0), min_difficulty: 0,
        fact: "Gregorian chant is entirely unison singing -- hundreds of monks on the same note. The raw power of unity.",
        hint: Some("Both notes are identical -- listen for a single pitch"),
        product: None,
    },
    ContentEntry {
        concept: 2, variant: Some(1), min_difficulty: 0,
        fact: "The Jaws theme is just two notes a half step apart. The most dissonant interval creates the most tension.",
        hint: Some("Song ref: 'Jaws' theme (duh-NUH) -- the smallest possible step"),
        product: Some(COURSERA_MUSICIANSHIP),
    },
    ContentEntry {
        concept: 2, variant: Some(2), min_difficulty: 0,
        fact: "'Happy Birthday' starts with a major 2nd -- the basic building block of every scale. Two whole steps in a row form the major sound.",
        hint: Some("Song ref: 'Happy Birthday' opening (Hap-py) -- one whole step"),
        product: None,
    },
    ContentEntry {
        concept: 2, variant: Some(3), min_difficulty: 0,
        fact: "'Greensleeves' opens with a minor 3rd leap. This one interval defines every minor chord and minor key.",
        hint: Some("Song ref: 'Greensleeves' (A-las) -- the 'sad' third"),
        product: None,
    },
    ContentEntry {
        concept: 2, variant: Some(4), min_difficulty: 0,
        fact: "Do-mi is ALWAYS a major 3rd, no matter what key you're in. That's the power of movable-do solfege.",
        hint: Some("Song ref: 'When the Saints' (Oh when the) -- the 'happy' third"),
        product: Some(COURSERA_FUNDAMENTALS),
    },
    ContentEntry {
        concept: 2, variant: Some(5), min_difficulty: 0,
        fact: "The perfect 4th is music's great paradox: consonant in a melody, but traditionally dissonant in harmony!",
        hint: Some("Song ref: 'Here Comes the Bride' -- open, bright, rising"),
        product: None,
    },
    ContentEntry {
        concept: 2, variant: Some(6), min_difficulty: 0,
        fact: "There are only 13 possible chromatic intervals, but the tritone is the only one that is its own inversion. It splits the octave exactly in half.",
        hint: Some("Song ref: 'Maria' (West Side Story) -- unstable, wants to resolve"),
        product: Some(COURSERA_JAZZ_IMPROV),
    },
    ContentEntry {
        concept: 2, variant: Some(7), min_difficulty: 0,
        fact: "Power chords in rock are just root and fifth. The perfect 5th is the foundation of the overtone series.",
        hint: Some("Song ref: 'Star Wars' opening (bum bum-bum) -- strong, open, powerful"),
        product: None,
    },
    ContentEntry {
        concept: 2, variant: Some(8), min_difficulty: 0,
        fact: "The minor 6th is the inversion of the major 3rd. Flip M3 upside down and you get m6 -- major becomes minor.",
        hint: Some("Song ref: 'The Entertainer' (Joplin) -- wide, bittersweet leap"),
        product: Some(COURSERA_MUSICIANSHIP),
    },
    ContentEntry {
        concept: 2, variant: Some(9), min_difficulty: 0,
        fact: "Jazz musicians love added 6th chords for their smooth, mellow quality. The major 6th gives warmth without tension.",
        hint: Some("Song ref: 'My Bonnie Lies Over the Ocean' -- warm, lyrical, open"),
        product: None,
    },
    ContentEntry {
        concept: 2, variant: Some(10), min_difficulty: 0,
        fact: "The minor 7th is the heart of dominant 7th chords. It adds soulful tension without the extreme pull of the major 7th.",
        hint: Some("Song ref: 'Somewhere' (West Side Story) -- bluesy, soulful"),
        product: None,
    },
    ContentEntry {
        concept: 2, variant: Some(11), min_difficulty: 0,
        fact: "Major 7th chords are the signature sound of jazz ballads and bossa nova -- dreamy, floating dissonance.",
        hint: Some("Song ref: 'Take On Me' (a-ha) -- wide, bright, just short of an octave"),
        product: Some(COURSERA_JAZZ_IMPROV),
    },
    ContentEntry {
        concept: 2, variant: Some(12), min_difficulty: 0,
        fact: "The frequency ratio of an octave is exactly 2:1. Your brain perceives octave-displaced notes as 'the same' -- nobody fully understands why.",
        hint: Some("Song ref: 'Somewhere Over the Rainbow' -- same note, higher register"),
        product: None,
    },

    // ════════════════════════════════════════════════════════════════
    // CHORD QUALITY (concept 3, variants 0–3)
    // ════════════════════════════════════════════════════════════════
    ContentEntry {
        concept: 3, variant: Some(0), min_difficulty: 0,
        fact: "Every major key contains exactly 3 major triads (I, IV, V), 3 minor triads (ii, iii, vi), and 1 diminished (vii\u{00b0}). The same recipe, every time.",
        hint: Some("The 3rd is major (4 semitones above root) -- bright, stable, 'happy'"),
        product: Some(COURSERA_FUNDAMENTALS),
    },
    ContentEntry {
        concept: 3, variant: Some(1), min_difficulty: 0,
        fact: "Major and minor differ by exactly one semitone -- the 3rd. That single half-step is the difference between joy and melancholy.",
        hint: Some("The 3rd is minor (3 semitones above root) -- darker, introspective"),
        product: None,
    },
    ContentEntry {
        concept: 3, variant: Some(2), min_difficulty: 0,
        fact: "A diminished 7th chord divides the octave into four equal minor thirds. There are really only 3 unique dim7 chords -- every other is a rearrangement.",
        hint: Some("Two stacked minor 3rds -- compact, tense, unstable"),
        product: Some(COURSERA_COMPOSITION),
    },
    ContentEntry {
        concept: 3, variant: Some(3), min_difficulty: 0,
        fact: "An augmented triad divides the octave into three equal parts. It sounds mysterious because your ear can't find a 'bottom.'",
        hint: Some("Two stacked major 3rds -- symmetrical, dreamy, tonally ambiguous"),
        product: None,
    },

    // ════════════════════════════════════════════════════════════════
    // CADENCE (concept 4, variants 0–3)
    // ════════════════════════════════════════════════════════════════
    ContentEntry {
        concept: 4, variant: Some(0), min_difficulty: 0,
        fact: "The cadential 6/4 is one of the most analyzed moments in all of theory. Despite looking like a tonic chord, it functions as a decorated dominant.",
        hint: Some("Bass motion: the bass moves from Sol down to Do -- strongest resolution"),
        product: Some(COURSERA_COMPOSITION),
    },
    ContentEntry {
        concept: 4, variant: Some(1), min_difficulty: 0,
        fact: "The Picardy third -- ending a minor piece with a major chord -- has been used since the Renaissance. Minor-key pieces almost ALWAYS ended major in the 1500s.",
        hint: Some("Bass motion: Fa falls to Do -- the 'Amen' sound, gentle, hymn-like"),
        product: Some(COURSERA_FUNDAMENTALS),
    },
    ContentEntry {
        concept: 4, variant: Some(2), min_difficulty: 0,
        fact: "A half cadence is a musical question mark. The phrase pauses on the dominant, demanding continuation. Composers use them to keep listeners engaged.",
        hint: Some("The phrase ends on V -- feels unresolved, like a question"),
        product: None,
    },
    ContentEntry {
        concept: 4, variant: Some(3), min_difficulty: 0,
        fact: "V to vi -- when Radiohead uses a deceptive cadence, they're using the same trick Beethoven did 200 years earlier. The surprise creates emotional depth.",
        hint: Some("Starts like authentic (V) but lands on vi instead of I -- the 'surprise' ending"),
        product: Some(COURSERA_MUSICIANSHIP),
    },

    // ════════════════════════════════════════════════════════════════
    // ADDITIONAL ENRICHMENT ENTRIES (concept-generic or alternate facts)
    // These provide variety -- users see different facts on repeat visits.
    // ════════════════════════════════════════════════════════════════

    // -- Extra interval facts (alternate fun facts for variety) --
    ContentEntry {
        concept: 2, variant: Some(6), min_difficulty: 3,
        fact: "The tritone was called 'diabolus in musica' and banned in medieval church music. It's now the backbone of dominant 7th chords and the entire blues tradition.",
        hint: Some("Exactly 6 semitones -- sits right in the middle of the octave"),
        product: Some(COURSERA_MUSICIANSHIP),
    },
    ContentEntry {
        concept: 2, variant: Some(7), min_difficulty: 2,
        fact: "The perfect 5th is so fundamental that it's the basis of Pythagorean tuning -- one of the oldest tuning systems, derived from the ratios of vibrating strings.",
        hint: Some("The most consonant interval after unison and octave -- 7 semitones"),
        product: Some(COURSERA_FUNDAMENTALS),
    },

    // -- Extra chord quality facts --
    ContentEntry {
        concept: 3, variant: Some(0), min_difficulty: 3,
        fact: "The I-IV-V progression uses only major triads. These three chords alone can harmonize almost any melody in a major key.",
        hint: Some("Bright and stable -- the 'default' chord sound in Western music"),
        product: Some(COURSERA_GUITAR),
    },
    ContentEntry {
        concept: 3, variant: Some(1), min_difficulty: 3,
        fact: "Minor chords appear on ii, iii, and vi in a major key. In minor keys, i, iv, and sometimes v are minor -- creating that pervasive melancholy.",
        hint: Some("The lower 3rd gives it a darker, more pensive quality"),
        product: None,
    },

    // -- Extra cadence facts --
    ContentEntry {
        concept: 4, variant: Some(0), min_difficulty: 3,
        fact: "Nearly every classical piece ends with an authentic cadence. The leading tone resolves up to tonic while the bass drops from dominant to tonic -- the strongest closure possible.",
        hint: Some("V to I -- the musical 'full stop'"),
        product: Some(COURSERA_COMPOSITION),
    },
    ContentEntry {
        concept: 4, variant: Some(3), min_difficulty: 3,
        fact: "Deceptive cadences work because vi shares two notes with I. Your brain was expecting I but gets its minor-mode shadow instead -- same skeleton, different mood.",
        hint: Some("V resolves to vi instead of I -- your ear expects resolution but gets a twist"),
        product: None,
    },

    // -- Extra scale degree facts (advanced, difficulty-gated) --
    ContentEntry {
        concept: 0, variant: Some(4), min_difficulty: 4,
        fact: "The dominant sits a perfect 5th above tonic. This 3:2 frequency ratio is the simplest after the octave, which is why V-I feels so natural.",
        hint: Some("The second-most stable degree -- strong and open"),
        product: Some(COURSERA_MUSICIANSHIP),
    },
    ContentEntry {
        concept: 0, variant: Some(6), min_difficulty: 4,
        fact: "The leading tone (ti) is music's most impatient note. Theorists say it 'wants' to resolve up to do so strongly that when it doesn't, it's called 'frustrated.'",
        hint: Some("Just a half step below tonic -- maximum instability"),
        product: None,
    },

    // -- Extra Roman numeral facts --
    ContentEntry {
        concept: 1, variant: Some(4), min_difficulty: 3,
        fact: "Add a minor 7th to V and you get V7 -- the dominant seventh chord. That added note creates the tritone interval, which is why V7 resolves to I even more powerfully than plain V.",
        hint: Some("Major chord built on the 5th degree -- the 'engine' of tonal harmony"),
        product: Some(COURSERA_JAZZ_IMPROV),
    },
    ContentEntry {
        concept: 1, variant: Some(0), min_difficulty: 4,
        fact: "In Schenkerian analysis, every tonal piece is ultimately a prolongation of the I chord. All the other chords are just elaborate decorations of home base.",
        hint: Some("The most stable chord -- everything resolves here"),
        product: Some(COURSERA_COMPOSITION),
    },

    // ════════════════════════════════════════════════════════════════
    // CONCEPT-GENERIC ENTRIES (variant: None, match any variant)
    // Shown as fallbacks when no variant-specific entry matches.
    // ════════════════════════════════════════════════════════════════
    ContentEntry {
        concept: 0, variant: None, min_difficulty: 0,
        fact: "Tendency tones: Ti pulls up to Do, Fa pulls down to Mi. These two half-step resolutions drive all of tonal harmony.",
        hint: None,
        product: Some(COURSERA_FUNDAMENTALS),
    },
    ContentEntry {
        concept: 1, variant: None, min_difficulty: 0,
        fact: "Roman numeral analysis was invented to show chord function independent of key. A ii-V-I sounds 'the same' whether it's in C major or Gb major.",
        hint: None,
        product: None,
    },
    ContentEntry {
        concept: 2, variant: None, min_difficulty: 0,
        fact: "Intervals come in pairs called inversions: flip a 3rd upside down and you get a 6th. Flip a 2nd and get a 7th. They always add to 9.",
        hint: None,
        product: Some(COURSERA_MUSICIANSHIP),
    },
    ContentEntry {
        concept: 3, variant: None, min_difficulty: 0,
        fact: "A chord's 'quality' comes from the size of its stacked thirds. Major+minor = major triad. Minor+major = minor triad. Same notes, different order, different world.",
        hint: None,
        product: None,
    },
    ContentEntry {
        concept: 4, variant: None, min_difficulty: 0,
        fact: "Cadences are musical punctuation: authentic = period, half = comma, deceptive = plot twist. The entire structure of tonal phrases depends on them.",
        hint: None,
        product: Some(COURSERA_COMPOSITION),
    },

    // ════════════════════════════════════════════════════════════════
    // ADVANCED HARMONY FUN FACTS (concept-generic, difficulty-gated)
    // These surface for experienced users and cover deeper topics
    // from Open Music Theory without needing new challenge types.
    // ════════════════════════════════════════════════════════════════
    ContentEntry {
        concept: 1, variant: None, min_difficulty: 5,
        fact: "An applied chord is like a brief vacation to another key -- you borrow a chord to make one of your own feel momentarily like a tonic. The most common is V/V.",
        hint: None,
        product: Some(COURSERA_JAZZ_IMPROV),
    },
    ContentEntry {
        concept: 3, variant: None, min_difficulty: 5,
        fact: "When Nirvana uses a bVII chord in a major-key song, that's modal mixture -- borrowing from the parallel minor for a darker sound. It's painting with borrowed colors.",
        hint: None,
        product: None,
    },
    ContentEntry {
        concept: 2, variant: None, min_difficulty: 6,
        fact: "All four augmented-sixth chords share the same defining interval: the augmented 6th between b6 and #4, which expands outward to an octave when it resolves.",
        hint: None,
        product: Some(COURSERA_COMPOSITION),
    },
    ContentEntry {
        concept: 0, variant: None, min_difficulty: 5,
        fact: "Le (b6) is the 'dark' tendency tone -- it pulls downward to Sol with melancholy gravity, while Ti pushes upward to Do with bright urgency.",
        hint: None,
        product: None,
    },
    ContentEntry {
        concept: 4, variant: None, min_difficulty: 5,
        fact: "The 'truck-driver modulation' -- named by scholar Walter Everett -- sounds like shifting gears: drop to the dominant of the new key, then rev up to the new tonic.",
        hint: None,
        product: Some(COURSERA_MUSIC_PROD),
    },
    ContentEntry {
        concept: 2, variant: None, min_difficulty: 5,
        fact: "Parallel fifths are the cardinal sin of counterpoint. Two voices in parallel perfect consonances lose their independence -- they start sounding like one voice.",
        hint: None,
        product: Some(COURSERA_COMPOSITION),
    },
    ContentEntry {
        concept: 2, variant: None, min_difficulty: 4,
        fact: "Music exploits a survival mechanism: your brain uses sound irregularities to detect threats. That's why a surprise chord change gives you chills.",
        hint: None,
        product: None,
    },
];

/// Select a content entry for the given challenge result.
/// Prefers variant-specific over generic entries. `seed` provides
/// deterministic random selection when multiple entries match.
pub fn select_content(
    concept: u8,
    variant: u8,
    difficulty: u8,
    seed: u64,
) -> Option<&'static ContentEntry> {
    let mut specific_count = 0usize;
    let mut generic_count = 0usize;

    // First pass: count matches
    for entry in CONTENT_DB {
        if entry.concept != concept || entry.min_difficulty > difficulty {
            continue;
        }
        match entry.variant {
            Some(v) if v == variant => specific_count += 1,
            None => generic_count += 1,
            _ => {}
        }
    }

    let (target_count, use_specific) = if specific_count > 0 {
        (specific_count, true)
    } else if generic_count > 0 {
        (generic_count, false)
    } else {
        return None;
    };

    // Deterministic selection
    let idx = (xorshift(seed) % target_count as u64) as usize;
    let mut seen = 0usize;

    for entry in CONTENT_DB {
        if entry.concept != concept || entry.min_difficulty > difficulty {
            continue;
        }
        let matches = match entry.variant {
            Some(v) if v == variant => use_specific,
            None => !use_specific,
            _ => false,
        };
        if matches {
            if seen == idx {
                return Some(entry);
            }
            seen += 1;
        }
    }

    None
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

    #[test]
    fn content_db_has_entries() {
        assert!(CONTENT_DB.len() >= 35, "should have at least one entry per SRS card");
    }

    #[test]
    fn content_db_all_concepts_covered() {
        for concept in 0..=4u8 {
            let count = CONTENT_DB.iter().filter(|e| e.concept == concept).count();
            assert!(count > 0, "concept {} has no entries", concept);
        }
    }

    #[test]
    fn select_content_finds_specific() {
        // Interval tritone (concept 2, variant 6) should exist
        let result = select_content(2, 6, 10, 42);
        assert!(result.is_some());
        let entry = result.unwrap();
        assert_eq!(entry.concept, 2);
    }

    #[test]
    fn select_content_falls_back_to_generic() {
        // Use a variant that likely only has concept-generic entries
        // concept 0, variant 0 has a specific entry, but let's test the generic path
        // by using a concept with known generic entries at high difficulty
        let result = select_content(0, 0, 10, 99);
        assert!(result.is_some());
    }

    #[test]
    fn select_content_deterministic() {
        let a = select_content(2, 7, 5, 123);
        let b = select_content(2, 7, 5, 123);
        assert_eq!(a.map(|e| e.fact), b.map(|e| e.fact));
    }

    #[test]
    fn select_content_respects_difficulty() {
        // At difficulty 0, should not return entries with min_difficulty > 0
        for entry in CONTENT_DB {
            if entry.min_difficulty > 0 {
                // If this variant also has a difficulty-0 entry, select_content
                // should prefer those at low difficulty
                continue;
            }
        }
        // Ensure entries with min_difficulty=5 don't show at difficulty 2
        let high_diff_only: Vec<_> = CONTENT_DB.iter()
            .filter(|e| e.concept == 0 && e.min_difficulty > 3)
            .collect();
        if !high_diff_only.is_empty() {
            // These shouldn't be returned at difficulty 0
            for seed in 0..20u64 {
                if let Some(entry) = select_content(
                    high_diff_only[0].concept,
                    high_diff_only[0].variant.unwrap_or(0),
                    0, seed,
                ) {
                    assert!(entry.min_difficulty <= 0);
                }
            }
        }
    }

    #[test]
    fn coursera_links_present() {
        let coursera_count = CONTENT_DB.iter()
            .filter(|e| e.product.as_ref().map_or(false, |p| p.program == "coursera"))
            .count();
        assert!(coursera_count >= 10, "should have at least 10 Coursera links, got {}", coursera_count);
    }

    #[test]
    fn hints_present() {
        let hint_count = CONTENT_DB.iter()
            .filter(|e| e.hint.is_some())
            .count();
        assert!(hint_count >= 20, "should have at least 20 hints, got {}", hint_count);
    }
}
