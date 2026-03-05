# Chord Reps Audio Overhaul — Research & Plan

## Executive Summary

The current audio is bare-bones oscillator synthesis (single sine/triangle waves per note) plus leftover Kasane Teto vocaloid samples. Every chord sounds like a 1980s electronic tutor. This doc covers what exists, what's available, and the best path to rich, realistic instrument audio.

---

## 1. Current State (What We're Replacing)

### Synthesis System
- **Single oscillator per note** — sine wave dominant, occasional triangle
- Signal chain: `OscillatorNode → BiquadFilter (lowpass) → GainNode → DynamicsCompressor → destination`
- Per-MIDI-note envelope table (`MIDI_NOTE_ENVELOPES`, MIDI 45-84) overrides Rust-provided ADSR values
- Lowpass filter cutoff: `min(8000, 2000 + (84 - midi) * 150)` Hz
- Compressor: threshold -12dB, ratio 4:1, knee 10dB

### Vocaloid Samples (to remove entirely)
- 45 Kasane Teto UTAU voicebank samples (Japanese syllables: a, i, u, e, o, ka, ki, etc.)
- Stored as **base64-encoded OGG** inline in HTML — ~350KB of the 372KB file
- Used only for post-solve piano key toggle and rhythm loop
- Formant synthesis fallback: sawtooth through two bandpass filters for vowel approximation
- **This entire system should be replaced with sampled piano sounds**

### Communication Architecture (keep this)
- Rust pushes `SoundCommand` variants (`PlayTone`, `PlaySample`) onto `engine.sound_queue`
- `drain_sound_commands()` serializes queue as JSON, JS parses and dispatches
- Scheduled sounds use two-phase: pushed to `self.scheduled_sounds` with `play_at` timestamp, promoted to queue when `total_time >= play_at`
- ~16.6ms scheduling jitter (frame-quantized), but Web Audio's `currentTime` provides sample-accurate playback

### MIDI Range
- MIDI 48 (C3) to 72 (C5), 25 notes
- Key roots: `[48, 50, 52, 53, 55, 57]` (C3, D3, E3, F3, G3, A3)
- Standard 12-TET tuning, A4=440Hz

### Current Limitations
1. No harmonic richness — single oscillator per note
2. No detuning/chorus for warmth
3. Piano keys play Japanese syllables instead of piano sounds
4. No reverb or spatial effects — dry and clinical
5. No stereo imaging — everything mono center
6. Context/question/answer notes all sound identical (no timbral differentiation)
7. 350KB of base64 vocaloid data serves no useful purpose
8. No polyphonic voice management

---

## 2. Available Technologies (Ranked)

### Tier 1: Best Fit for Chord Reps

#### WebAudioFont (RECOMMENDED)
- **What**: Wavetable synthesis with pre-converted GM instrument samples
- **Size**: ~20KB player + ~1.2MB per instrument JS file
- **API**: `player.queueWaveTable()`, `player.queueChord()`, `player.queueStrumUp/Down()`
- **Instruments**: ~2000 variations across all 128 GM programs (5-10 per program from FluidR3, GeneralUser GS, MusyngKite, etc.)
- **Integration**: Zero dependencies, pure JS, `<script>` tag — matches current architecture perfectly
- **Built-in**: Reverb, 10-band EQ, chord playback
- **GitHub**: https://github.com/surikov/webaudiofont

**Why it wins**: Zero build step, built-in `queueChord()` for ear training, on-demand instrument loading, sample-accurate timing via Web Audio scheduling. Drop-in replacement for current oscillator system.

#### Tone.js Sampler + Salamander Piano
- **What**: Full Web Audio framework with sample-based instruments
- **Size**: ~150KB Tone.js + ~1.5MB for 30 Salamander MP3 samples (every minor third)
- **API**: `sampler.triggerAttackRelease("C4", "8n")`, polyphonic by default
- **Quality**: Excellent — Yamaha C5 Grand Piano, public domain
- **Integration**: Requires ES modules or bundler
- **GitHub**: https://github.com/Tonejs/Tone.js, https://github.com/tambien/Piano

**Why it's strong**: Best scheduling/timing API, richest ecosystem, proven in OpenEar (open-source ear training app). But heavier dependency.

### Tier 2: Good Alternatives

#### smplr (Modern soundfont-player successor)
- **Size**: ~15KB + samples loaded from CDN
- **API**: Clean, Promise-based, TypeScript support
- **Instruments**: All 128 GM via MusyngKite or FluidR3_GM soundfonts
- **GitHub**: https://github.com/danigb/smplr

#### Touch Pianist Approach (DIY minimal)
- **What**: One MP3 per key (88 files), lowpass filter simulates velocity
- **Size**: ~2-3MB total
- **Quality**: Proven production-quality in Touch Pianist app
- **Complexity**: No library needed, works with existing Web Audio code
- **Source**: https://earslap.com/article/the-browser-sound-engine-behind-touch-pianist.html

### Tier 3: Too Heavy / Niche

| Option | Why Not |
|--------|---------|
| SpessaSynth | Full SF2 synth, 30-148MB soundfont files |
| @tonejs/piano | 3-8MB for velocity layers we don't need |
| MIDI.js | Abandoned, dated API |
| Web MIDI API | Safari doesn't support it (no iOS) |
| Raw FM/additive synthesis | Can't convincingly do piano |

---

## 3. Free Sample Sources

| Source | What | Size | License | URL |
|--------|------|------|---------|-----|
| **Salamander Grand Piano V3** | Yamaha C5, 16 velocity layers | 1.12GB full / ~1.5MB web-optimized | CC-BY / Public Domain | [Archive.org](https://archive.org/details/SalamanderGrandPianoV3) |
| **Tonejs Salamander subset** | 30 MP3s (every minor third) | ~1.5MB | Public Domain | [GitHub](https://github.com/Tonejs/audio/tree/master/salamander) |
| **midi-js-soundfonts** | All 128 GM instruments, pre-rendered | ~500KB-2MB per instrument | MIT | [GitHub](https://github.com/gleitz/midi-js-soundfonts) |
| **WebAudioFont presets** | 2000+ instrument variations | 50KB-1.5MB each | MIT | [GitHub](https://github.com/surikov/webaudiofont) |
| **Google Magenta SGM Plus** | Piano: pitches 21-108, 8 velocity layers | Moderate | Apache 2.0 | [Magenta](https://magenta.github.io/magenta-js/music/) |
| **Univ. of Iowa MIS** | Academic instrument recordings | Large (raw AIFF) | Free | [UIowa](https://theremin.music.uiowa.edu/mis.html) |
| **FreePats** | FLAC samples, SoundFont files | Large | Various open | [FreePats](https://freepats.zenvoid.org/) |

---

## 4. Chord Voicing Best Practices

### For Ear Training Specifically

1. **Close position in mid-register (C3-C5)** — sweet spot for hearing chord quality. Too low = muddy, too high = thin.
2. **Root position first, then inversions** — root position is stable/grounded, 1st inversion is lighter, 2nd inversion is "floating"
3. **Double the root for 4-note triads** — C3-C4-E4-G4 gives fuller sound while anchoring the root (this is what ToneDear and musictheory.net do)
4. **Bass note slightly before upper voices** — 50-100ms offset simulates natural piano playing and helps the ear parse harmony
5. **Consistent register with random transposition** — forces ear to focus on interval relationships, not absolute pitch
6. **Proper voice leading for cadences/progressions** — move each voice to nearest available note in next chord

### Voicing Types

| Type | Description | Example (Cmaj) | Best For |
|------|------------|-----------------|----------|
| Close position | All tones within one octave | C4-E4-G4 | Chord quality ID, beginners |
| Open position | Spread beyond octave | C3-G3-E4 | Fuller sound, advanced |
| Drop 2 | 2nd-from-top dropped an octave | G3-C4-E4-B4 | Jazz, natural balance |
| Root doubled | Root + close triad | C3-C4-E4-G4 | Clear root, standard for training |

### How Competitors Voice Chords
- **ToneDear**: Close position triads, mid-register, single piano timbre
- **Teoria**: Root position with optional inversions, piano sound
- **EarMaster**: Multiple voicings, multiple instruments, MIDI keyboard input
- **ToneGym**: Piano-based, gamified, close position primarily
- **OpenEar**: Tone.js + piano samples, composable exercise pattern

---

## 5. Recommended Implementation Plan

### Phase 1: Replace Oscillators with Sampled Piano (~1.2MB)

**Goal**: Everything sounds like a real piano instead of sine waves.

1. Add WebAudioFont player + Acoustic Grand Piano preset to `index.html`
2. Add new `SoundCommand` variant in Rust:
   ```rust
   PlaySampledNote { instrument: u8, note: u8, duration: f64, volume: f64, delay: f64 }
   PlaySampledChord { instrument: u8, notes: Vec<u8>, duration: f64, volume: f64 }
   ```
3. Replace all `PlayTone` calls in challenge generators with `PlaySampledNote`
4. Replace piano key toggle from Teto samples to piano samples
5. **Remove all Kasane Teto base64 data** — saves 350KB from HTML

**Result**: The app immediately sounds 10x better. ~1.2MB download for piano instrument.

### Phase 2: Bass + Upper Voice Separation (~400KB more)

**Goal**: Chords have depth with a distinct bass voice.

1. Load Electric Bass preset (GM Program 33)
2. For triads: bass plays root an octave below, upper voices on piano
3. Bass note plays 50ms before upper voices for natural feel
4. Apply to: RomanNumeral, Cadence, ChordQuality challenges

**Result**: Chords sound like a real ensemble. ~1.6MB total.

### Phase 3: Reverb + Spatial Effects

**Goal**: No more dry, clinical sound.

1. WebAudioFont has built-in reverb — enable it on the master bus
2. Add subtle stereo panning for arpeggiated notes (slight left-right spread)
3. Differentiate timbral roles: context notes slightly softer/warmer, mystery notes brighter

**Result**: Professional, polished sound. No additional download.

### Phase 4: Multi-Instrument Variety (~800KB-1.2MB more)

**Goal**: Timbral variety keeps the ear engaged.

1. Add instrument selection: Piano, Vibraphone, Organ, Strings
2. Different default instruments per challenge type:
   - Chord Quality: piano
   - Cadences: piano + bass
   - Intervals: vibraphone (clear, beautiful sustain)
   - Scale Degrees: piano
3. Lazy-load instruments on demand with loading indicator

**File Size Budget**:

| Instrument | Size | Load Timing |
|-----------|------|-------------|
| Acoustic Grand Piano | ~1.2MB | Immediate (first interaction) |
| Electric Bass | ~400KB | On cadence/progression exercises |
| Vibraphone | ~200KB | On interval exercises |
| String Ensemble | ~800KB | On advanced mode |
| **Total max** | **~2.6MB** | Spread across sessions |

### Phase 5: Advanced Polish

- Velocity-sensitive chord playback (louder = brighter via WebAudioFont volume)
- Chord strumming option via `queueStrumDown()`
- Sustain pedal simulation for richer resonance
- User preference for instrument (save to localStorage)

---

## 6. Integration Architecture

### Minimal Changes to Existing System

The Rust → JS sound pipeline stays the same. We just add new command types:

```
Rust (mod.rs)                    JS (index.html)
─────────────                    ───────────────
PlaySampledNote {                WebAudioFont
  instrument: 0,       ──JSON──▶  player.queueWaveTable(
  note: 60,                          ctx, dest, pianoPreset,
  duration: 2.0,                     ctx.currentTime + delay,
  volume: 0.7,                       note, duration, volume
  delay: 0.0                      )
}

PlaySampledChord {               WebAudioFont
  instrument: 0,       ──JSON──▶  player.queueChord(
  notes: [60,64,67],                 ctx, dest, pianoPreset,
  duration: 2.0,                     ctx.currentTime,
  volume: 0.6                        notes, duration, volume
}                                  )
```

### Loading Strategy

```javascript
// On first user tap (ensureAudio):
const player = new WebAudioFontPlayer();
player.loader.startLoad(audioCtx, '_tone_0000_FluidR3_GM_sf2_file');

// When piano is loaded:
player.loader.waitLoad(() => { pianoReady = true; });

// Lazy-load bass on first cadence exercise:
function loadBass() {
    player.loader.startLoad(audioCtx, '_tone_0330_FluidR3_GM_sf2_file');
}
```

---

## 7. Key Sources

### Libraries
- WebAudioFont: https://github.com/surikov/webaudiofont
- Tone.js: https://tonejs.github.io/
- @tonejs/piano: https://github.com/tambien/Piano
- tonejs-instruments: https://github.com/nbrosowsky/tonejs-instruments
- smplr: https://github.com/danigb/smplr
- soundfont-player: https://github.com/danigb/soundfont-player
- SpessaSynth: https://github.com/spessasus/SpessaSynth

### Samples
- Salamander Grand Piano: https://archive.org/details/SalamanderGrandPianoV3
- Tonejs Salamander: https://github.com/Tonejs/audio/tree/master/salamander
- midi-js-soundfonts: https://github.com/gleitz/midi-js-soundfonts
- Salamander MP3 (npm/CDN): https://github.com/darosh/samples-piano-mp3

### Techniques
- Touch Pianist sound engine: https://earslap.com/article/the-browser-sound-engine-behind-touch-pianist.html
- Greg Jopa piano pitch-shifting: https://www.gregjopa.com/2023/03/piano-sounds-with-web-audio-api/
- FM synthesis in Rust/WASM: https://cprimozic.net/blog/fm-synth-rust-wasm-simd/
- Additive synthesis guide: https://teropa.info/blog/2016/09/20/additive-synthesis.html
- LeafWindow digital piano tutorial: https://www.leafwindow.com/en/digital-piano-with-web-audio-api-6-en/

### Competitors / Reference Apps
- OpenEar (Tone.js + piano samples): https://github.com/ShacharHarshuv/open-ear
- Solfetta (webaudio-tinysynth): https://github.com/tomcookedeveloper/solfetta
- ToneGym: https://www.tonegym.co/
- EarMaster: https://www.earmaster.com/
- ToneDear: https://tonedear.com/ear-training/chord-identification

### Specs
- General MIDI instrument list: https://www.earmaster.com/wiki/music-technology/list-of-general-midi-instruments.html
- Web MIDI API (80% browser support, no Safari): https://caniuse.com/midi
- Web Audio API: https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API
