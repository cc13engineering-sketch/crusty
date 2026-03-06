# Chord Reps Enhancements: New Music Theory Topics

_Plan created 2026-03-05. Based on open-source education research + engine architecture analysis._

## Context

The app currently has 35 cards across 5 ear-training concepts (scale degrees, Roman numerals, intervals, chord quality, cadences). All audio-based, no notation. The proposed expansion adds visual/knowledge-based challenges covering clefs, staff reading, circle of fifths, triad/seventh chord construction, diatonic harmony, modal mixture, and pivot chords.

### Engine Constraints That Shape This Plan

- **Software rasterizer** — no WebGL. Staff notation must be drawn pixel-by-pixel via `Framebuffer` + `shapes.rs` primitives (fill_circle, fill_triangle, fill_tapered_trail). No VexFlow/SVG/Canvas2D available.
- **Deterministic** — all generation uses `engine.rng` (SeededRng xorshift64). No `rand` crate, no JS Math.random.
- **Command-buffer audio** — `SoundCommand::PlayNote { note, duration, volume, instrument }` is the only audio primitive. Notes are MIDI numbers. JS-side WebAudioFont handles synthesis.
- **SRS cards keyed by `"concept:variant"`** — stored in `BTreeMap<String, CardState>`. Adding new concepts means new concept IDs (5, 6, 7...) with their own variant ranges. Backward-compatible with existing localStorage data.
- **Persistence is JSON via `PersistCommand::Set`** — single `srs_state` key in localStorage. Adding cards doesn't break deserialization (unknown keys in `from_json()` are ignored, new keys start as fresh cards).
- **Layout is relative to 600x900 reference frame** — `sy(v)` scales everything. New UI sections must fit this grid or extend it.
- **No external assets at runtime** — everything is compiled into WASM or drawn procedurally. Staff notation images can't be loaded; they must be rendered.

---

## Phase 1: Seventh Chord Recognition (Ear Training)

**Rationale:** Lowest-hanging fruit. Reuses existing audio infrastructure, challenge flow, and UI. The `ChordQuality` concept already plays arpeggiated chords — seventh chords just add one more note.

### New Concept: `5` (SeventhChordQuality)

**theory.rs changes:**
```
enum SeventhChordType { Maj7, Dom7, Min7, HalfDim7, Dim7 }

// Interval patterns (semitones from root):
Maj7:     [0, 4, 7, 11]  — root + M3 + P5 + M7
Dom7:     [0, 4, 7, 10]  — root + M3 + P5 + m7
Min7:     [0, 3, 7, 10]  — root + m3 + P5 + m7
HalfDim7: [0, 3, 6, 10]  — root + m3 + d5 + m7
Dim7:     [0, 3, 6, 9]   — root + m3 + d5 + d7
```

**5 variants** (one per type), 5 new SRS cards: `"5:0"` through `"5:4"`.

**mod.rs changes:**
- Add `generate_seventh_challenge()` — same structure as `generate_quality_challenge()` but uses 4-note chords and 5 options
- Audio scheduling: arpeggiate 4 notes (0.15s spacing), then play together at 0.7s
- Option labels: `["Maj7", "Dom7", "Min7", "m7♭5", "dim7"]`
- The 5th option requires either: (a) a wider option button layout, or (b) a 2-row option layout for challenges with >4 options
- Add `seventh_insight()` function with educational facts per type

**Difficulty gating:**
- Difficulty 1-3: pool = `[Maj7, Dom7, Min7]` (most distinct sounds)
- Difficulty 4-6: add HalfDim7
- Difficulty 7+: add Dim7 (similar to HalfDim7, hardest distinction)

**CONTENT_DB additions:** ~10 entries covering each seventh chord type + generic facts about seventh chord usage in jazz, pop, classical.

**SRS compatibility:** New cards start as unseen. Existing `srs_state` JSON deserializes fine — `from_json()` only reads known keys, and `select_next_card()` already iterates a hardcoded card list that we extend.

### Risk: Option Layout for 5 Choices

Current layout: 4 option buttons at `OPTIONS_Y = 310`, each `OPTIONS_H = 70` tall, spaced evenly across screen width. With 5 options, each button is narrower.

**Approach:** Keep 4-wide layout but make it concept-dependent. For 5-option concepts, use slightly narrower buttons with smaller font. The buttons already have `min_w` logic — just need to adjust the grid calculation when `options.len() == 5`.

---

## Phase 2: Circle of Fifths Enumeration

**Rationale:** Pure knowledge challenge — no audio or notation rendering needed. User sees a starting note and direction, must enumerate the next N notes in circle-of-fifths order.

### New Concept: `6` (CircleOfFifths)

**24 variants:** 12 notes × 2 directions (clockwise/counter-clockwise). Card keys: `"6:0"` through `"6:23"` (variants 0-11 = clockwise from C,C#,...,B; 12-23 = counter-clockwise).

**Challenge format:**
- Prompt: "Starting from **G**, name the next 4 notes going **clockwise** (fifths)"
- Answer: text input or sequential note selection on piano keyboard
- The piano keyboard is already rendered — repurpose it as an input device

**Implementation approach — Piano as Input:**
- Challenge shows prompt text in the challenge area
- User taps notes on the piano in order (4-6 notes)
- Each correct tap highlights the key green and advances
- Wrong tap flashes red, resets sequence
- This reuses the existing piano rendering and `check_piano_click()` infrastructure

**theory.rs additions:**
```rust
const FIFTHS_ORDER: [u8; 12] = [0, 7, 2, 9, 4, 11, 6, 1, 8, 3, 10, 5];
// C, G, D, A, E, B, F#/Gb, Db, Ab, Eb, Bb, F

fn circle_sequence(start_note: u8, clockwise: bool, count: usize) -> Vec<u8> {
    // Returns the next `count` notes in circle order
}
```

**mod.rs changes:**
- New `generate_circle_challenge()` — sets up expected sequence
- New input mode: sequential piano taps (vs. current single-option-click mode)
- Need a `ChallengeInputMode` enum: `SingleOption` (current) vs. `SequentialPiano` (new)
- Render a progress indicator showing how many notes entered so far

**SRS integration:** Quality = 5 if all notes correct on first try, 4 if ≤2 mistakes, 1 if abandoned.

### Risk: New Input Mode

The biggest architectural change here. Current flow assumes 4 clickable option buttons → instant answer. Sequential piano input is fundamentally different. Needs:
- A `pending_sequence: Vec<u8>` field tracking entered notes
- A `expected_sequence: Vec<u8>` field for validation
- Modified `step()` to handle sequential input state machine
- Modified `render()` to show progress (e.g., "3/6 notes entered")

**Mitigation:** Wrap in a match on `ChallengeInputMode` so existing option-click flow is untouched.

---

## Phase 3: Triad/Seventh Chord Construction (Note Deduction)

**Rationale:** "Given Cm7, what are the notes?" — tests knowledge of chord intervals. Uses piano as answer input (same sequential mode from Phase 2).

### New Concepts: `7` (TriadDeduction), `8` (SeventhDeduction)

**Concept 7 — Triad Deduction:**
- 60 variants: 12 roots × 5 types (Maj, Min, Dim, Aug, Power)
- Card keys: `"7:0"` through `"7:59"` (variant = root*5 + type)
- Prompt: "Spell the notes of **E minor**"
- Answer: tap 3 notes on piano in any order
- Validation: check if tapped set matches expected set (order-independent)

**Concept 8 — Seventh Chord Deduction:**
- 60 variants: 12 roots × 5 types (Maj7, Dom7, Min7, HalfDim7, Dim7)
- Prompt: "Spell the notes of **Dmaj7**"
- Answer: tap 4 notes on piano

**theory.rs additions:**
```rust
fn triad_notes(root: u8, quality: TriadType) -> [u8; 3]
fn seventh_notes(root: u8, quality: SeventhChordType) -> [u8; 4]

// Power chord: root + P5 (2 notes only)
// Handled as special case: [root, root+7]
```

**New input mode:** `SetPiano` — user taps N notes in any order, confirms with a "Check" button. Different from `SequentialPiano` (Phase 2) which requires order.

**Rendering:**
- Show chord name prominently in challenge area
- Piano highlights tapped notes in a "pending" color (amber?)
- "Check" button appears when enough notes are tapped
- On check: green flash if correct, red flash + show correct notes if wrong
- A "Clear" button to reset tapped notes

**Difficulty gating:**
- Difficulty 1-3: only Major and Minor triads
- Difficulty 4-6: add Diminished and Augmented
- Difficulty 7+: add Power chords (trivial but reinforces)
- Seventh chords: Difficulty 1-3 = Maj7/Dom7/Min7, 4+ = HalfDim/Dim7

---

## Phase 4: Chord Identification from Notes (Induction)

### New Concepts: `9` (TriadInduction), `10` (SeventhInduction)

**Concept 9 — Triad Induction (with inversions):**
- 180 variants: 12 roots × 5 types × 3 inversions
- Variant encoding: `root * 15 + type * 3 + inversion`
- Prompt: shows 3 highlighted notes on piano → "Name this chord"
- Answer: select from 4-5 text options (e.g., "C major", "A minor", "F# dim")

**Concept 10 — Seventh Induction (root position only):**
- 60 variants: 12 roots × 5 types
- Same format but with 4 highlighted notes

**This reuses the existing option-click UI** — no new input modes needed.

**mod.rs changes:**
- `generate_triad_induction()` — highlight 3 piano keys, generate text options
- Piano keys highlighted in a neutral "question" color (distinct from correct/wrong)
- Option labels are chord names, not numbers/abbreviations
- Generate distractors that share notes with the correct answer (harder)

**theory.rs additions:**
```rust
fn invert_triad(notes: [u8; 3], inversion: u8) -> [u8; 3] {
    match inversion {
        0 => notes,                           // root position
        1 => [notes[1], notes[2], notes[0]+12], // 1st inversion
        2 => [notes[2], notes[0]+12, notes[1]+12], // 2nd inversion
    }
}

fn identify_triad(notes: &[u8]) -> Option<(u8, TriadType, u8)> {
    // Returns (root, type, inversion) or None
    // Try all 12 roots × 5 types × 3 inversions
}
```

---

## Phase 5: Diatonic Harmony

### New Concepts: `11` (ScaleTriads), `12` (ParallelMinor), `13` (PivotChords)

**Concept 11 — Triads in Each Scale:**
- 24 variants: 12 notes × 2 (major/minor)
- Prompt: "What are the diatonic triads in **G major**?"
- Answer: sequential selection of 7 chords (reuse SequentialPiano? Or 7-option text layout?)
- Alternative simpler format: show Roman numerals I-vii, ask "What quality is the **iii** chord in G major?" → option-click

**Recommend the simpler format** — asking to enumerate all 7 is too much for one card. Instead, 7 sub-cards per key, each asking about one degree's chord quality. That's 24 × 7 = 168 variants, but many are identical across keys (the quality pattern is the same in every major key). Reduce to 14 unique questions: 7 degrees × 2 (major/minor key pattern).

**Concept 12 — Parallel Minor Borrowing:**
- 12 variants (one per major key)
- Prompt: "Which chords can be borrowed from **C minor** into **C major**?"
- Answer: option-click from list of borrowed chords (iv, ♭VI, ♭VII, etc.)
- Simpler format: "What is the borrowed **iv** chord in C major?" → option-click

**Concept 13 — Pivot Chords:**
- 36 variants: combinations of closely related keys
- Prompt: "Name a pivot chord between **C major** and **G major**"
- Answer: option-click from possible pivot chords
- This is the most advanced topic — gate behind difficulty ≥ 6

**theory.rs additions:**
```rust
fn diatonic_triads(root: u8, is_minor: bool) -> [(u8, TriadType); 7]
fn parallel_minor_borrowings(root: u8) -> Vec<(u8, TriadType)>  // borrowed chords
fn pivot_chords(key1_root: u8, key2_root: u8) -> Vec<(u8, TriadType)>
```

---

## Phase 6: Staff Notation (Deferred — Highest Effort)

### Why This Is Last

Staff rendering requires drawing from scratch in our software rasterizer:
- 5 horizontal staff lines per system
- Clef symbols (treble, bass, alto, tenor) — complex curves
- Noteheads (filled/open ovals at precise positions)
- Ledger lines above/below staff
- Accidental symbols (sharp, flat, natural)

**This is a significant rendering project.** The clef symbols alone are complex vector shapes that must be rasterized. Unlike circles and rectangles, clefs are irregular glyphs.

### Approach If Implemented

**Option A: Bitmap glyphs (recommended)**
- Pre-render clef symbols as small pixel arrays compiled into Rust `const` data
- Blit them onto the framebuffer using `blit_rect()` or similar
- Staff lines are trivial (horizontal `fill_rect`)
- Noteheads are filled ovals (already have `fill_circle`)
- Ledger lines are short horizontal lines
- Total bitmap data: ~4 clef glyphs × ~40x80px × 1 byte = ~12KB compiled in

**Option B: Vector rendering**
- Define clef shapes as polylines/bezier curves
- Rasterize using existing AA primitives
- More code, better scaling, but much more complex

**New Concepts (if implemented):**

**Concept 14 — Clef Identification:**
- 4 variants (treble, bass, alto, tenor)
- Render clef on staff → option-click to name it
- Trivial once bitmap glyphs exist

**Concept 15 — Note Reading:**
- 92 variants: 4 clefs × 23 notes (3 ledger lines each direction)
- Render note on staff in given clef → identify as "B3", "C4", etc.
- Uses option-click with 4 note name options

**Recommend deferring to a future sprint.** The effort is disproportionate to the value vs. Phases 1-5 which reuse existing infrastructure.

---

## Implementation Priorities

| Phase | Concept IDs | New Cards | Effort | Prereqs |
|-------|-------------|-----------|--------|---------|
| 1: Seventh Chord Ear Training | 5 | 5 | Small | None — pure extension of existing pattern |
| 2: Circle of Fifths | 6 | 24 | Medium | New SequentialPiano input mode |
| 3: Chord Construction | 7, 8 | 120 | Medium | New SetPiano input mode (reuses Phase 2 piano infra) |
| 4: Chord Identification | 9, 10 | 240 | Small | Reuses option-click, needs piano highlight mode |
| 5: Diatonic Harmony | 11, 12, 13 | ~60 | Medium | New theory functions, option-click |
| 6: Staff Notation | 14, 15 | 96 | Large | Bitmap glyph system, staff renderer |

**Total new cards:** ~545 (6× current deck size)
**Recommended sprint order:** Phase 1 → 4 → 5 → 2 → 3 → 6

Rationale for reordering:
- Phase 1 is trivial and validates the "add a new concept" pattern
- Phase 4 before Phase 3 because option-click is simpler than piano-input
- Phase 2-3 grouped because they share the new piano-input infrastructure
- Phase 6 deferred indefinitely unless explicitly requested

---

## Architecture Changes Required

### 1. Concept Registry (theory.rs)

Currently, concepts are hardcoded as magic numbers (0-4) throughout the codebase. Before adding 10+ new concepts, refactor to:

```rust
pub enum Concept {
    ScaleDegree = 0,
    RomanNumeral = 1,
    IntervalRecognition = 2,
    ChordQuality = 3,
    Cadence = 4,
    SeventhChordQuality = 5,
    CircleOfFifths = 6,
    TriadDeduction = 7,
    SeventhDeduction = 8,
    TriadInduction = 9,
    SeventhInduction = 10,
    ScaleTriads = 11,
    ParallelMinor = 12,
    PivotChords = 13,
    // Phase 6
    ClefIdentification = 14,
    NoteReading = 15,
}
```

Replace all `concept: u8` with `concept: Concept` in SRS, challenge generation, and content DB.

### 2. Challenge Input Modes (mod.rs)

```rust
enum ChallengeInputMode {
    SingleOption,        // Current: click 1 of 4-5 buttons
    SequentialPiano,     // Phase 2: tap notes in order on piano
    SetPiano,            // Phase 3: tap N notes in any order, then confirm
}
```

The `step()` main loop branches on this mode for input handling. `render()` adjusts UI accordingly (show/hide option buttons, show piano progress indicator, show Check/Clear buttons).

### 3. SRS Card Registry (srs.rs)

Currently `select_next_card()` has a hardcoded list of all (concept, variant) pairs. Replace with a registry:

```rust
fn all_cards() -> Vec<(u8, u8)> {
    let mut cards = vec![];
    // Concept 0: 7 variants
    for v in 0..7 { cards.push((0, v)); }
    // Concept 1: 7 variants
    for v in 0..7 { cards.push((1, v)); }
    // ... etc for all concepts
    cards
}
```

This is already close to what exists but should be made more maintainable as concept count grows.

### 4. Variable Option Count

Current UI hardcodes 4 options. Seventh chord quality needs 5. Diatonic harmony questions might need 7. Abstract the option layout:

```rust
struct OptionLayout {
    count: usize,         // 4, 5, or 7
    cols: usize,          // 4 or 5 per row
    rows: usize,          // 1 or 2
    button_w: f64,        // computed from count
    button_h: f64,
}
```

### 5. Piano Highlight Modes

Currently piano highlights are binary (correct answer flash). Need:
- `PianoMode::Passive` — current behavior, highlights show after answer
- `PianoMode::Input` — keys are tappable input, highlight on tap
- `PianoMode::Display` — keys highlighted as part of the question (induction challenges)

---

## Content Attribution

All educational content derived from these sources requires attribution:

| Source | License | Attribution |
|--------|---------|-------------|
| Open Music Theory v2 | CC BY-SA 4.0 | "Open Music Theory, Version 2, Chelsey Hamm et al." |
| seancolsen/music-theory-data | CC BY-SA 4.0 | "Sean Colsen, music-theory-data" |
| Wikipedia | CC BY-SA 4.0 | Standard Wikipedia attribution |
| Tonal.js (logic reference) | MIT | No attribution required but good practice |

Add an attribution section to the app's info/about area or footer.

---

## Open Source Data We Can Use Directly

**For programmatic card generation** (all interval/chord/scale permutations):
- `seancolsen/music-theory-data` YAML — 19 chord types, 399 scales, binary encoding scheme
- Port Tonal.js logic to Rust for chord detection, key enumeration, pivot chord finding
- `pybuche/anki-music-theory` deck structure as reference for difficulty tiering

**For CONTENT_DB educational facts:**
- OMT2 chapter content (CC BY-SA 4.0) — definitions, aural descriptors, pedagogical explanations
- Adapt for brevity (current facts are 1-2 sentences)
