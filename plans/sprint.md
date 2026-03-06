# Crusty Engine Sprint Plan: Game-to-Engine Extraction

**Date:** 2026-03-05
**Scope:** Extract reusable concepts from gravity-pong and chord-reps back into engine-core
**Status:** Planning

---

## 1. Executive Summary

Crusty is a deterministic 2D game engine with two production games built on it: **gravity-pong** (orbital physics puzzle) and **chord-reps** (music theory spaced repetition trainer). Both games have invented significant systems outside the engine to solve problems the engine does not yet address. This document catalogs every concept worth extracting, prioritizes them, and lays out an implementation roadmap.

The goal is not to make the engine game-specific. It is to identify **generic, reusable primitives** that both games needed (and future games will need) and bring them into engine-core where they belong. Each concept is evaluated on three criteria:

1. **Reusability** -- Would a third game likely need this?
2. **Complexity of re-implementation** -- How painful is it to rebuild from scratch each time?
3. **Engine fit** -- Does it belong in the engine layer, or is it game-specific logic?

This plan covers 35 extractable concepts across 7 domains, organized into 3 priority tiers, with a suggested 8-sprint implementation roadmap.

---

## 2. Gap Analysis

### What the engine provides well
- ECS with 32 components and 21 systems across 5 execution phases
- Physics: semi-implicit Euler, CCD, spatial grid, circle/rect colliders, joints, force fields (including Plummer falloff)
- Rendering: framebuffer, anti-aliased shapes, particles, post-FX, screen effects, layers, transitions
- Input: keyboard, mouse, touch, gestures, input mapping
- Events: frame events + pub/sub EventBus
- Sound: command-buffer queue with tone/noise/sample/note support
- High-level: GameFlow, AutoJuice, CameraDirector, UiCanvas, SceneManager, Timers, Templates

### What the games had to reinvent

| Gap | gravity-pong | chord-reps | Impact |
|-----|:---:|:---:|--------|
| Plasma corridor / directional linear force | Invented from scratch | -- | Any game with currents, conveyor streams, wind corridors |
| Wormhole / teleportation | Invented from scratch | -- | Puzzle games, platformers, any portal mechanic |
| Adaptive substep integration | Invented from scratch | -- | Any game with high-acceleration fields |
| Sling / launch mechanics | Invented from scratch | -- | Angry Birds-style, golf, any projectile game |
| Tapered trail rendering (game-level) | Custom two-pass system | -- | Universal visual effect |
| Field visualization / potential grid | Custom 50x50 grid | -- | Any game exposing force fields to the player |
| Aim preview / trajectory simulation | Custom 120-step sim | -- | Projectile aiming in any game |
| Spaced repetition (SM-2) | -- | Invented from scratch | Any learning/quiz/flashcard game |
| PersistQueue for localStorage | -- | Invented from scratch | Every WASM game needs save/load |
| Responsive reference-coord layout | -- | Invented from scratch | Every game targeting phone + desktop |
| Audio scheduling with timing | -- | Invented from scratch | Music games, rhythm games, sequencers |
| Sound-reactive visual effects | -- | Invented from scratch | Any game with audio-visual coupling |
| Particle effects beyond pool | Both games | Both games | Sparkles, dust motes, ambient particles |
| Music theory utilities | -- | Invented from scratch | Music games (niche but complete) |
| Streak / combo tracking | -- | Invented from scratch | Any game with scoring progression |
| Content database with difficulty gating | -- | Invented from scratch | Quiz, education, RPG dialogue |
| Queue-based I/O pattern | -- | Invented from scratch | General WASM architecture pattern |

---

## 3. Concept Catalog

### 3.1 Physics Domain

#### P1: Plasma Current (Linear Corridor Force)
- **Source:** gravity-pong
- **What it does:** Applies directional force along a line segment A to B, with Gaussian falloff perpendicular to the corridor axis. Velocity coupling reduces force by 50% when entity moves against the current. Strength maps 1-30 to 100-200 force units.
- **Why valuable:** Wind tunnels, conveyor streams, river currents, jet streams. Any game with directional environmental forces confined to a region. The existing `ZoneEffect::Conveyor` is rectangular and axis-aligned -- plasma currents are arbitrarily oriented line segments with smooth perpendicular falloff.
- **Complexity:** M
- **Dependencies:** None (builds on existing force accumulator)
- **Engine fit:** New `FieldType::LinearCorridor { ax, ay, bx, by, falloff_width }` variant on ForceField, or a new component `CorridorForce`. The velocity-coupling factor could be a parameter.

#### P2: Wormhole / Portal Teleportation
- **Source:** gravity-pong
- **What it does:** Two linked spatial mouths. When an entity enters mouth A, it exits mouth B with preserved velocity. Per-mouth cooldown timer prevents oscillation (entity bouncing back and forth).
- **Why valuable:** Portals are a fundamental puzzle/platformer mechanic. Pac-Man wrapping, Portal-style wormholes, teleporters in strategy games.
- **Complexity:** M
- **Dependencies:** None
- **Engine fit:** New component `Portal { partner: Entity, cooldown: f64, cooldown_timer: f64 }`. New system `portal` in PostPhysics phase that checks overlap and teleports. Requires trigger collider on each mouth.

#### P3: Adaptive Substep Integration
- **Source:** gravity-pong
- **What it does:** Dynamically scales physics substeps based on maximum acceleration: `substeps = min(cap, ceil(max_accel / threshold))`. Prevents tunneling and instability in high-gravity scenarios without wasting cycles in calm areas.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** Parameter on the physics step configuration. The engine already runs fixed-dt; this adds an inner subdivision loop. Add `adaptive_substeps: Option<AdaptiveSubstep>` to Engine config, where `AdaptiveSubstep { threshold: f64, max_substeps: u8 }`.

#### P4: Sling / Launch Mechanics
- **Source:** gravity-pong
- **What it does:** Two-stage input: place waypoint, then drag-to-sling. Delta-based input (pull = mouse_current - mouse_pressed). Power = pull_distance / max_pull. Post-launch immunity window (1.0s, decays at 2.5/s) prevents immediate recapture by force fields. Hard stop after 2.5s zeroes velocity to prevent infinite drifting.
- **Why valuable:** Angry Birds, golf games, pinball launchers, any projectile-aiming mechanic. The immunity-window pattern applies whenever you launch something near attractors.
- **Complexity:** M
- **Dependencies:** Aim Preview (R4) for visual feedback
- **Engine fit:** `LaunchState` component tracking phase (idle/aiming/launched), immunity timer, and hard-stop timer. A `launch` system in Simulation phase. The drag input pattern could be a reusable `DragGesture` utility (engine already has gesture recognition).

#### P5: Gravity Immunity / Force Shield
- **Source:** gravity-pong (sling immunity)
- **What it does:** Temporary per-entity immunity to ForceField effects. Decays over time. Prevents launched projectiles from being immediately recaptured.
- **Why valuable:** Shield powerups, spawn protection, launch windows, temporary invulnerability to environmental forces.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** New component `ForceImmunity { factor: f64, decay_rate: f64 }` where factor 1.0 = full immunity, 0.0 = no immunity. Force accumulator multiplies forces by `(1.0 - immunity.factor)`. Decay system reduces factor each frame.

#### P6: Exponential Drag with Immunity Factor
- **Source:** gravity-pong
- **What it does:** `v *= exp(-drag_coeff * speed * dt)` with a sling-immunity multiplier. Frame-rate independent, creates natural terminal velocity.
- **Why valuable:** Already partially covered by `ContinuousDrag` component. The immunity factor integration is the missing piece.
- **Complexity:** S
- **Dependencies:** P5 (ForceImmunity)
- **Engine fit:** Extend existing `ContinuousDrag` to check for `ForceImmunity` component and modulate drag coefficient accordingly. Small change to integrator system.

#### P7: Hard Stop / Velocity Timeout
- **Source:** gravity-pong (2.5s flight hard stop)
- **What it does:** After a configurable duration of flight, velocity is zeroed. Prevents infinite drifting in low-drag environments.
- **Why valuable:** Any game where projectiles should not fly forever. Turn-based physics games, puzzle games.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** Could be a variant of `MotionConstraint` or a new `VelocityTimeout { max_flight_time: f64, timer: f64 }` component. System decrements timer and zeroes velocity when expired.

### 3.2 Rendering Domain

#### R1: Tapered Trail Rendering (Enhanced)
- **Source:** gravity-pong
- **What it does:** Two-pass rendering: wide low-alpha glow pass underneath, narrow bright core pass on top. Width varies with entity speed. Alpha gradient uses `frac^0.6` curve for natural fade. The engine has `GhostTrail` (position snapshots) and `fill_tapered_trail` (shape primitive), but games still do custom trail rendering with speed-dependent width and two-pass glow.
- **Why valuable:** Trails are one of the most common visual effects in 2D games. Speed-dependent width and glow passes should be engine-level options, not game-level reimplementation.
- **Complexity:** S
- **Dependencies:** None (enhances existing GhostTrail + shapes)
- **Engine fit:** Extend `GhostTrail` component with `glow_enabled: bool`, `glow_alpha_multiplier: f64`, `glow_width_multiplier: f64`, `speed_width_scale: bool`. Extend `ghost_trail` system to capture speed alongside position. Extend renderer to do two-pass rendering when glow is enabled.

#### R2: Field Visualization Grid
- **Source:** gravity-pong
- **What it does:** Precomputes a 50x50 grid of force potential values across the screen. Renders as colored dots: blue for attractive potential, amber for repulsive. Updates each frame based on active ForceField entities.
- **Why valuable:** Any game that exposes force fields to the player needs visualization. Strategy games showing influence zones, physics sandboxes, educational simulations.
- **Complexity:** M
- **Dependencies:** None
- **Engine fit:** The engine already has `DensityField` module. This could be an extension or companion: `ForceFieldVisualizer` that samples all ForceFields on a grid and renders to the framebuffer. New rendering module `field_viz.rs`. Configurable grid resolution, color mapping, and alpha.

#### R3: Ambient Particle Layer (Dust Motes)
- **Source:** gravity-pong (300 dust motes responding to 30% gravity)
- **What it does:** Large population of tiny ambient particles that respond to environmental forces at a reduced strength. Gravity-tinted colors. Purely decorative, no collision.
- **Why valuable:** Adds atmosphere and life to any scene. Snow, dust, fireflies, pollen, space debris. The existing particle pool is burst-oriented (spawn N, fade, die). Ambient particles are persistent and force-responsive.
- **Complexity:** M
- **Dependencies:** None
- **Engine fit:** New system `ambient_particles` with its own pool (separate from burst particles). Configuration: count, force_response_factor, color_from_field, base_color, size_range. Rendered in RenderingPrep phase before entities.

#### R4: Aim Preview / Trajectory Simulation
- **Source:** gravity-pong
- **What it does:** Runs a mini physics simulation (120 steps) from the launch point with the launch velocity, applying drag but intentionally omitting gravity (so the player must read the field). Renders dotted trajectory. Stops on hazard or target hit.
- **Why valuable:** Any projectile game needs trajectory preview. The "omit some forces" pattern lets designers choose what the player must predict vs. what is shown.
- **Complexity:** M
- **Dependencies:** None
- **Engine fit:** The engine already has `AimPreview` module. Needs extension to support configurable force inclusion/exclusion and collision termination conditions. The mini-simulation should be a utility function: `simulate_trajectory(start, velocity, steps, dt, forces, colliders) -> Vec<Vec2>`.

#### R5: Sound-Reactive Visual Effects
- **Source:** chord-reps (stars pulse based on sound energy)
- **What it does:** Tracks a "sound energy" scalar that spikes on audio events and decays over time. Visual elements (background stars, glow intensity, particle emission rate) modulate based on this value.
- **Why valuable:** Audio-visual coupling makes any game feel more alive. Rhythm games, music visualizers, any game with sound feedback.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** New lightweight module `sound_energy.rs` with `SoundEnergy { level: f64, decay_rate: f64 }`. Expose `spike(amount)` and `tick(dt)`. Games query the level to drive visuals. Could live alongside SoundCommandQueue.

#### R6: Border Glow Animation
- **Source:** chord-reps (3px animated border pulse tied to feedback state)
- **What it does:** Renders a pulsing glow border around the screen or a widget, color and intensity driven by game state (correct = green pulse, wrong = red pulse, neutral = subtle).
- **Why valuable:** Universal feedback mechanism for any interactive application. Quiz games, notification systems, alert states.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** Extension to `screen_fx.rs`. New effect type `BorderGlow { color: Color, width: f64, pulse_speed: f64, intensity: f64 }`. Rendered in the screen_fx pass.

#### R7: Sparkle / Celebration Particles
- **Source:** chord-reps (circle/star/note shapes, gravity-affected, spawn on correct)
- **What it does:** Burst of shaped particles (circles, stars, custom shapes) with gravity, spawned at a point. Different from engine particle pool which is circle-only.
- **Why valuable:** Celebration effects, collectible pickup feedback, level completion confetti. Every game with positive feedback moments.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** Extend existing `particles.rs` to support `ParticleShape { Circle, Star { points: u8 }, Triangle, Square }`. Add shape field to `Particle` struct. Extend particle renderer to draw each shape.

### 3.3 UI/UX Domain

#### U1: Responsive Reference-Coord Layout System
- **Source:** chord-reps
- **What it does:** Defines a reference coordinate space (e.g., 600x900) and scales uniformly by `screen_h / reference_h`. All UI positions and sizes are specified in reference coords. Works seamlessly from phone to desktop.
- **Why valuable:** Every game targeting multiple screen sizes needs this. The current UiCanvas uses pixel offsets from anchors, which breaks across resolutions.
- **Complexity:** M
- **Dependencies:** None
- **Engine fit:** Extension to `UiCanvas`. Add `reference_size: Option<(f64, f64)>` to canvas config. When set, all widget positions/sizes are in reference coords, auto-scaled at render time. The anchor system still works but coordinates are reference-relative.

#### U2: SRS Pill Badges / Status Indicators
- **Source:** chord-reps (Mature/Learning/Due counts)
- **What it does:** Small colored pill-shaped badges showing counts or status. Rendered inline in headers or widget rows.
- **Why valuable:** Status indicators are universal UI: notification counts, health bars, resource counters, achievement badges.
- **Complexity:** S
- **Dependencies:** U1 (Reference-Coord Layout) for proper scaling
- **Engine fit:** New widget type in `UiCanvas`: `Widget::Badge { text: String, bg_color: Color, text_color: Color, corner_radius: f64 }`. Auto-sized to text content.

#### U3: Mobile-Friendly Touch Zones
- **Source:** chord-reps (60px touch zones, 2x zoom piano, scroll slider)
- **What it does:** Enlarged hit areas for touch interaction, with scroll/pan support for content wider than the screen. Minimum 44px touch targets (Apple HIG).
- **Why valuable:** Any game targeting mobile must handle touch ergonomics. The engine's gesture system handles raw gestures but not touch-zone layout.
- **Complexity:** M
- **Dependencies:** U1 (Reference-Coord Layout)
- **Engine fit:** Extension to `UiCanvas` hit-testing. Add `min_touch_size: f64` to widget config. Add `ScrollRegion` widget type that clips content and responds to drag/swipe gestures within a defined viewport.

#### U4: Combo / Streak Display
- **Source:** chord-reps (duration x color ramp: teal to pink to gold)
- **What it does:** Displays current streak count with escalating visual intensity. Color ramps through a palette as streak grows. Pulse animation on increment.
- **Why valuable:** Streaks and combos appear in almost every score-driven game. Fighting games, rhythm games, quiz games, endless runners.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** New component `StreakTracker { count: u32, max: u32, display_timer: f64 }`. Rendering utility function `render_combo(fb, streak, position, color_ramp)`. Could also be a UiCanvas widget.

### 3.4 Audio Domain

#### A1: Audio Scheduling with Precise Timing
- **Source:** chord-reps
- **What it does:** Queues notes with absolute timing offsets (e.g., note at t=0.0, t=0.3, t=0.6 for arpeggiation). The existing `SoundCommandQueue` fires everything immediately when drained. This adds a `delay` or `at_time` field.
- **Why valuable:** Music games, rhythm games, sequenced sound effects, any game needing timed audio sequences (e.g., a melody on level complete).
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** Add `ScheduledCommand { command: SoundCommand, delay: f64 }` to `SoundCommandQueue`. The drain_json output includes the delay field. JS audio driver schedules using `audioContext.currentTime + delay`.

#### A2: Layered Audio Playback
- **Source:** chord-reps (context chord, pause, mystery sound at different volumes)
- **What it does:** Plays multiple audio elements in sequence with controlled timing gaps and independent volume levels. A "sequence" is an ordered list of (command, delay_after) pairs.
- **Why valuable:** Tutorials (play instruction, pause, play example), multi-part sound effects, musical phrases.
- **Complexity:** S
- **Dependencies:** A1 (Audio Scheduling)
- **Engine fit:** Utility function `queue_sequence(queue, steps: &[(SoundCommand, f64)])` that computes absolute delays and pushes `ScheduledCommand`s. Pure helper, no new component needed.

#### A3: Music Theory Utilities
- **Source:** chord-reps
- **What it does:** MAJOR_SCALE intervals, chord quality intervals (major/minor/dim/aug/7th), cadence chord pairs, note names, interval names. Functions: note_to_frequency, chord_notes, interval_name, scale_degree.
- **Why valuable:** Niche but complete. Any music-related game or audio tool. Could also support procedural music generation for non-music games.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** New module `music_theory.rs` alongside `sound.rs`. Pure functions, no state. Provides `note_freq(midi_note) -> f64`, `chord_intervals(quality) -> &[u8]`, `scale_intervals(scale_type) -> &[u8]`, `note_name(midi_note) -> &str`.

### 3.5 Learning / Progression Domain

#### L1: Spaced Repetition System (SM-2)
- **Source:** chord-reps
- **What it does:** Full SM-2 algorithm. Card state: easiness_factor (starts 2.5), interval_days, repetitions, next_review_day, last_reviewed_day. Quality scores 1-5. Interval calculation: rep 1 = 1 day, rep 2 = 6 days, thereafter = prev_interval * easiness. Easiness adjusted by: `ef + (0.1 - (5 - q) * (0.08 + (5 - q) * 0.02))`, clamped to minimum 1.3.
- **Why valuable:** Any educational game, flashcard app, language learning tool, or quiz game. Spaced repetition is the gold standard for learning retention.
- **Complexity:** M
- **Dependencies:** L3 (PersistQueue) for saving state
- **Engine fit:** New module `srs.rs` at engine-core top level. `SrsCard { id: String, easiness: f64, interval: f64, reps: u32, next_review: f64, last_review: f64 }`. `SrsDeck` struct managing a collection of cards. Pure algorithm, no rendering. `fn review(card: &mut SrsCard, quality: u8, current_day: f64)`.

#### L2: Card Deck / Content Database System
- **Source:** chord-reps
- **What it does:** Content entries with variant-specific and generic fallbacks. Difficulty-gated content (advanced content requires difficulty >= threshold). Card ID format with concept:variant naming. Daily new-card limits. Priority selection: most overdue, then new, then least recently reviewed.
- **Why valuable:** Any game with unlockable content, progressive difficulty, or a library of challenges. RPG ability decks, quiz pools, procedural content selection.
- **Complexity:** M
- **Dependencies:** L1 (SRS) for scheduling, though the deck system is useful independently
- **Engine fit:** New module `content_deck.rs`. `ContentEntry { id: String, difficulty_gate: f64, tags: Vec<String>, data: serde_json::Value }`. `ContentDeck` with selection strategies (most_overdue, random_weighted, difficulty_filtered). Separate from SRS -- SRS schedules reviews, ContentDeck manages the content pool.

#### L3: PersistQueue for localStorage
- **Source:** chord-reps
- **What it does:** Accumulates key-value pairs to persist, serializes as compact JSON with abbreviated keys. JS side drains the queue and writes to localStorage. Decouples Rust save logic from JS storage.
- **Why valuable:** Every WASM game needs save/load. The engine has `save_load.rs` but it requires the game to manage serialization manually. A queue-based approach matches the existing SoundCommandQueue pattern.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** New module `persist_queue.rs` mirroring `SoundCommandQueue` pattern. `PersistQueue { commands: Vec<PersistCommand> }` where `PersistCommand` is `Set { key, value_json }`, `Remove { key }`, or `Clear`. `drain_json()` returns the batch. JS side applies to localStorage.

#### L4: Difficulty Scaling System
- **Source:** chord-reps (1-10 range, increases every 5 correct)
- **What it does:** Tracks player performance and adjusts difficulty. Configurable: threshold for increase/decrease, min/max range, what difficulty controls (option pool size, speed, complexity).
- **Why valuable:** Dynamic difficulty adjustment is standard in modern games. Any game can benefit: enemy count, speed multiplier, puzzle complexity.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** New component or GameState-based system. `DifficultyController { level: f64, min: f64, max: f64, increase_threshold: u32, decrease_threshold: u32, correct_streak: u32, wrong_streak: u32 }`. Pure logic, games query the level to parameterize their systems.

#### L5: Streak / Combo Tracker
- **Source:** chord-reps (consecutive correct, max streak, combo display at 3+)
- **What it does:** Tracks consecutive successes, records maximum streak, fires events at milestone thresholds (3, 5, 10, etc.). Resets on failure.
- **Why valuable:** Universal game mechanic. Score multipliers, visual escalation, achievement triggers.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** Utility struct `StreakTracker { current: u32, max: u32, milestones: Vec<u32> }` with `hit() -> Option<u32>` (returns milestone if reached) and `miss()`. Could be a component or standalone.

### 3.6 Game Design Domain

#### G1: Progressive Mechanic Introduction Pattern
- **Source:** gravity-pong (each level introduces one mechanic, capstone combines)
- **What it does:** Framework for introducing game mechanics one at a time across levels, then combining them in challenge levels. Level metadata tags which mechanics are active.
- **Why valuable:** This is game design best practice. Any multi-level game benefits from structured introduction flow. The engine's `LevelCurve` handles difficulty curves but not mechanic introduction sequencing.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** Extension to `LevelCurve` or a companion `MechanicProgression` struct. Maps level numbers to sets of active mechanics. `fn active_mechanics(level: u32) -> Vec<MechanicId>`. Pure data, games interpret the mechanic IDs.

#### G2: Level Data String Format / Level Loader
- **Source:** gravity-pong (`type:x:y:param1:param2` per line)
- **What it does:** Simple text-based level format. Each line describes an entity with type and positional parameters. Easy to edit by hand, easy to generate procedurally.
- **Why valuable:** Simple level formats reduce the barrier to creating content. The engine has `Templates` for entity spawning and `ProceduralGen` for generation, but no standard level serialization format.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** New module `level_format.rs`. Parser function `parse_level(text: &str) -> Vec<LevelEntry>` where `LevelEntry { entity_type: String, x: f64, y: f64, params: Vec<f64> }`. Games register type handlers. Could integrate with Templates.

#### G3: Game Phase State Machine (with Timed Transitions)
- **Source:** gravity-pong (Playing -> Won/Lost -> LevelTransition(2s) -> next)
- **What it does:** State machine for game flow with automatic timed transitions between phases. The engine has `GameFlow` but it handles broader flow (menu/playing/paused). This is the in-game phase within "playing."
- **Why valuable:** Almost every level-based game needs: intro, playing, success, failure, transition. Standardizing the pattern prevents each game from reinventing it.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** Extension to `GameFlow`. Add `SubPhase` concept: `GameFlow` handles top-level (Menu/Playing/Paused), games define sub-phases within Playing that auto-advance on timers. Or: a new `PhaseStateMachine` component that games attach to a "game controller" entity.

#### G4: Deterministic Shuffle Utility
- **Source:** chord-reps (xorshift64 with seed parameter)
- **What it does:** Fisher-Yates shuffle using a provided seed, without requiring an RNG state object. Pure function: `shuffle(slice, seed) -> shuffled slice`.
- **Why valuable:** Reproducible randomization for level generation, card dealing, enemy placement. The engine has `SeededRng` but it requires carrying state. A pure-function shuffle is more convenient for one-shot randomization.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** Add `pub fn seeded_shuffle<T>(slice: &mut [T], seed: u64)` to `rng.rs`. Uses internal temporary xorshift state, does not affect the engine's canonical RNG.

#### G5: Challenge Generation Pipeline
- **Source:** chord-reps (concept -> variant -> option pool -> difficulty filter -> shuffle -> present)
- **What it does:** Multi-stage pipeline for generating a challenge/question from a content database. Each stage narrows and transforms the candidate pool.
- **Why valuable:** Any quiz, trivia, or challenge-based game. Also applicable to procedural encounter generation in RPGs.
- **Complexity:** M
- **Dependencies:** L2 (Content Database), G4 (Deterministic Shuffle)
- **Engine fit:** Generic `Pipeline<T>` utility with composable filter/transform stages. Or: a `ChallengeGenerator` trait that games implement. Given the diversity of challenge types, a trait-based approach may be more flexible than a rigid pipeline struct.

### 3.7 Architecture Domain

#### A4: Queue-Based I/O Pattern (Generic)
- **Source:** chord-reps (sound commands + persist commands accumulated, JSON-serialized, JS-consumed)
- **What it does:** Generic pattern for accumulating commands in Rust during a frame, serializing to JSON, and having JS drain and execute. Already used for sound. Could be generalized.
- **Why valuable:** The pattern is used for SoundCommandQueue and should be used for PersistQueue, analytics, telemetry, external API calls. A generic implementation avoids duplicating the drain/serialize logic.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** Generic `CommandQueue<T: ToJson>` struct with `push`, `drain_json`, `len`, `is_empty`, `clear`. SoundCommandQueue and PersistQueue become type aliases or thin wrappers. New module `command_queue.rs`.

#### A5: Supernova Countdown / Timed Event Visualization
- **Source:** gravity-pong (spinning spokes, shrinking ring, accelerating flash)
- **What it does:** Multi-stage visual countdown for dramatic timed events. Configurable phases with escalating visual intensity.
- **Why valuable:** Boss warnings, bomb timers, round countdowns, any timed event with visual urgency. The engine has `Timers` but no visual countdown system.
- **Complexity:** M
- **Dependencies:** None
- **Engine fit:** New component `Countdown { total_duration: f64, elapsed: f64, phases: Vec<CountdownPhase> }` where each phase defines visual parameters (spoke count, ring radius, flash frequency). System ticks the timer and emits events at phase boundaries. Rendering handled by game code querying the component state.

#### A6: Three-Body / Multi-Source Indicator
- **Source:** gravity-pong (trail tinted orange when particle near 2+ gravity sources)
- **What it does:** Detects when an entity is significantly influenced by 2 or more force sources simultaneously. Changes visual state (trail color, glow) to indicate chaotic dynamics.
- **Why valuable:** Useful in any multi-body physics game. Also applicable as a general "danger zone" indicator when multiple forces compete.
- **Complexity:** S
- **Dependencies:** None
- **Engine fit:** Utility function `count_significant_forces(entity_pos, force_fields, threshold) -> usize` in physics/math. Games query this to drive visuals. Could also be a system that tags entities with a `MultiForceInfluence { source_count: u32 }` component.

---

## 4. Priority Tiers

### Tier 1: Foundation
Core primitives that multiple games need and are painful to reimplement. These are the highest-leverage extractions.

| ID | Concept | Complexity | Rationale |
|----|---------|:---:|-----------|
| L3 | PersistQueue | S | Every WASM game needs save/load. Pattern already proven by SoundCommandQueue. |
| A4 | Generic CommandQueue | S | Foundation for PersistQueue and future queues. Extract once, use everywhere. |
| P3 | Adaptive Substeps | S | Safety net for any physics-heavy game. Small change, large stability gain. |
| P5 | Force Immunity | S | Enables clean launch mechanics, shield powerups, spawn protection. |
| R1 | Enhanced Tapered Trails | S | Most common visual effect. Engine has pieces, just needs parameterization. |
| U1 | Reference-Coord Layout | M | Required for any multi-device game. Foundation for all UI work. |
| G4 | Deterministic Shuffle | S | Tiny utility, large convenience. Used by card games, level gen, content selection. |
| A1 | Audio Scheduling | S | Adds delay field to existing sound system. Minimal change, enables music games. |
| L5 | Streak Tracker | S | Universal game mechanic. Tiny implementation, used by almost every score game. |
| R5 | Sound-Reactive Effects | S | Small utility with outsized visual impact. |

**Tier 1 total: ~7 small + 1 medium = approximately 2 sprints**

### Tier 2: Power
Higher-level systems that enable new game genres and richer experiences.

| ID | Concept | Complexity | Rationale |
|----|---------|:---:|-----------|
| P1 | Plasma Corridor Force | M | Enables current/wind corridor mechanics. Goes beyond axis-aligned zones. |
| P2 | Wormhole / Portal | M | Classic puzzle mechanic. Moderate complexity but high reuse potential. |
| P4 | Sling / Launch Mechanics | M | Projectile games are a major genre. Complex but highly reusable. |
| R2 | Field Visualization | M | Any game exposing force fields needs this. Leverages existing DensityField. |
| R4 | Aim Preview (Enhanced) | M | Projectile aiming with configurable force inclusion. |
| L1 | Spaced Repetition (SM-2) | M | Enables entire educational game genre. Well-defined algorithm. |
| L2 | Content Database / Deck | M | Content management for quiz/RPG/card games. |
| L4 | Difficulty Scaling | S | Dynamic difficulty is standard practice. Small but impactful. |
| R3 | Ambient Particle Layer | M | Persistent force-responsive particles for atmosphere. |
| U3 | Mobile Touch Zones | M | Required for serious mobile targeting. |

**Tier 2 total: ~1 small + 8 medium = approximately 4 sprints**

### Tier 3: Polish
Nice-to-have improvements that enhance specific game types.

| ID | Concept | Complexity | Rationale |
|----|---------|:---:|-----------|
| A2 | Layered Audio Playback | S | Convenience on top of A1. |
| A3 | Music Theory Utilities | S | Niche but complete. Only needed for music games. |
| U2 | SRS Pill Badges | S | UI widget addition. |
| U4 | Combo Display | S | Rendering utility for streaks. |
| R6 | Border Glow Animation | S | Screen effect extension. |
| R7 | Sparkle Particles | S | Particle shape extension. |
| G1 | Mechanic Progression | S | Design pattern, minimal code. |
| G2 | Level Format | S | Simple parser utility. |
| G3 | Phase State Machine | S | GameFlow extension. |
| G5 | Challenge Pipeline | M | Specific to quiz/challenge games. |
| A5 | Countdown Visualization | M | Timed event feedback. |
| A6 | Multi-Source Indicator | S | Physics utility function. |

**Tier 3 total: ~10 small + 2 medium = approximately 2 sprints**

---

## 5. Implementation Roadmap

### Sprint 1: Infrastructure & Primitives (Foundation Core)
**Theme:** Lay the groundwork that everything else builds on.

| Task | Concept | Est. |
|------|---------|------|
| Generic CommandQueue<T> | A4 | 2h |
| PersistQueue (on top of CommandQueue) | L3 | 2h |
| Deterministic shuffle utility | G4 | 1h |
| Audio scheduling (delay field) | A1 | 2h |
| Sound energy tracker | R5 | 1h |

**Deliverables:** `command_queue.rs`, `persist_queue.rs`, updated `rng.rs`, updated `sound.rs`, `sound_energy.rs`
**Tests:** Unit tests for each new module. Integration test: PersistQueue drain/serialize round-trip.

### Sprint 2: Physics Safety & Force Extensions
**Theme:** Make the physics engine robust and expressive.

| Task | Concept | Est. |
|------|---------|------|
| Adaptive substep integration | P3 | 3h |
| ForceImmunity component + integrator changes | P5 | 2h |
| Exponential drag with immunity | P6 | 1h |
| Velocity timeout component | P7 | 1h |
| Streak tracker utility | L5 | 1h |

**Deliverables:** Updated `engine.rs` (substep config), new `force_immunity.rs` component, updated `integrator.rs`, `velocity_timeout.rs` component, `streak.rs`
**Tests:** Physics stability test with high-acceleration ForceFields. Immunity decay test. Streak milestone test.

### Sprint 3: Trail & Rendering Enhancements
**Theme:** Make things look good out of the box.

| Task | Concept | Est. |
|------|---------|------|
| Enhanced GhostTrail (glow, speed-width) | R1 | 3h |
| Particle shape extension (star, triangle) | R7 | 2h |
| Border glow screen effect | R6 | 1h |
| Sparkle particle burst utility | R7 | 1h |

**Deliverables:** Updated `ghost_trail.rs` component + system, updated `particles.rs`, updated `screen_fx.rs`
**Tests:** Trail rendering with various parameter combinations. Particle shape rendering correctness.

### Sprint 4: UI Layout System
**Theme:** Make every game work on every screen.

| Task | Concept | Est. |
|------|---------|------|
| Reference-coord layout system | U1 | 4h |
| Badge widget | U2 | 1h |
| Combo display widget/renderer | U4 | 2h |
| Difficulty scaling controller | L4 | 2h |

**Deliverables:** Updated `ui_canvas.rs` with reference coords, new widget types, `difficulty.rs`
**Tests:** Reference-coord scaling at various screen sizes. Badge rendering. Difficulty adjustment thresholds.

### Sprint 5: Advanced Physics Mechanics
**Theme:** Enable new game genres through physics primitives.

| Task | Concept | Est. |
|------|---------|------|
| Plasma corridor force (LinearCorridor) | P1 | 4h |
| Wormhole / portal system | P2 | 4h |
| Multi-source force indicator | A6 | 1h |

**Deliverables:** Updated `force_field.rs` + `force_accumulator.rs`, new `portal.rs` component + system, utility in `physics/math.rs`
**Tests:** Corridor force direction and falloff. Portal teleportation with cooldown. Multi-source detection.

### Sprint 6: Projectile & Visualization
**Theme:** Complete the projectile game toolkit.

| Task | Concept | Est. |
|------|---------|------|
| Sling / launch mechanics | P4 | 4h |
| Enhanced aim preview | R4 | 3h |
| Field visualization grid | R2 | 4h |

**Deliverables:** `launch_state.rs` component + system, updated `aim_preview.rs`, new `field_viz.rs`
**Tests:** Launch state transitions. Trajectory simulation accuracy. Field grid potential computation.

### Sprint 7: Learning & Content Systems
**Theme:** Enable educational and quiz games.

| Task | Concept | Est. |
|------|---------|------|
| SM-2 spaced repetition | L1 | 4h |
| Content database / deck | L2 | 4h |
| Challenge generation pipeline | G5 | 3h |
| Music theory utilities | A3 | 2h |

**Deliverables:** `srs.rs`, `content_deck.rs`, `challenge.rs`, `music_theory.rs`
**Tests:** SM-2 interval calculations against reference implementation. Deck selection priority. Shuffle determinism.

### Sprint 8: Game Design Patterns & Polish
**Theme:** Design pattern utilities and remaining polish.

| Task | Concept | Est. |
|------|---------|------|
| Mechanic progression framework | G1 | 1h |
| Level data format parser | G2 | 2h |
| Phase state machine extension | G3 | 2h |
| Ambient particle layer | R3 | 3h |
| Layered audio playback | A2 | 1h |
| Mobile touch zones | U3 | 3h |
| Countdown visualization | A5 | 2h |

**Deliverables:** Updated `level_curve.rs`, `level_format.rs`, updated `game_flow.rs`, `ambient_particles.rs`, updated `ui_canvas.rs`
**Tests:** Level format parse/roundtrip. Phase transition timing. Ambient particle force response.

### Roadmap Summary

```
Sprint 1  [Foundation]     Infrastructure & Primitives        ~8h
Sprint 2  [Foundation]     Physics Safety & Extensions         ~8h
Sprint 3  [Foundation]     Trail & Rendering Enhancements      ~7h
Sprint 4  [Foundation]     UI Layout System                    ~9h
Sprint 5  [Power]          Advanced Physics Mechanics           ~9h
Sprint 6  [Power]          Projectile & Visualization          ~11h
Sprint 7  [Power]          Learning & Content Systems           ~13h
Sprint 8  [Polish]         Game Design Patterns & Polish        ~14h
                                                      Total: ~79h
```

---

## 6. Architecture Notes

### How Concepts Map to Engine Structure

#### New Components (added to `components/`)

| Component | File | Store in World |
|-----------|------|---------------|
| `ForceImmunity` | `force_immunity.rs` | `force_immunities: ComponentStore<ForceImmunity>` |
| `Portal` | `portal.rs` | `portals: ComponentStore<Portal>` |
| `VelocityTimeout` | `velocity_timeout.rs` | `velocity_timeouts: ComponentStore<VelocityTimeout>` |
| `LaunchState` | `launch_state.rs` | `launch_states: ComponentStore<LaunchState>` |
| `Countdown` | `countdown.rs` | `countdowns: ComponentStore<Countdown>` |

Each follows the standard checklist: create file in `components/`, add to `mod.rs`, add store to `World`, add to `new()`/`despawn()`/`clear()`, implement `SchemaInfo`.

#### New Systems (added to `systems/`)

| System | Phase | After | Reads | Writes |
|--------|-------|-------|-------|--------|
| `force_immunity` | Simulation | behavior | ForceImmunity | ForceImmunity.factor (decay) |
| `portal` | PostPhysics | collision | Portal, Transform, Collider | Transform (teleport position) |
| `velocity_timeout` | PostPhysics | integrator | VelocityTimeout, RigidBody | VelocityTimeout.timer, RigidBody.vx/vy |
| `launch` | Simulation | behavior | LaunchState, Transform | LaunchState, RigidBody, Impulse |
| `countdown` | Simulation | tween | Countdown | Countdown.elapsed, EventBus (phase events) |
| `ambient_particles` | RenderingPrep | starfield | ForceField, Transform | Internal particle pool |

#### Modified Existing Systems

| System | Change |
|--------|--------|
| `force_accumulator` | Check `ForceImmunity` component; multiply forces by `(1.0 - immunity.factor)`. Add `LinearCorridor` handling for plasma currents. |
| `integrator` | Check `ForceImmunity` for drag modulation. No structural change. |
| `ghost_trail` (system) | Capture speed alongside position. Two-pass rendering with glow when enabled. |
| `renderer` | Call field_viz rendering. Support particle shapes. |

#### New Top-Level Modules (alongside existing modules)

| Module | Purpose |
|--------|---------|
| `command_queue.rs` | Generic `CommandQueue<T>` replacing ad-hoc queue patterns |
| `persist_queue.rs` | PersistQueue built on CommandQueue |
| `sound_energy.rs` | Sound energy level tracker for audio-visual coupling |
| `srs.rs` | SM-2 spaced repetition algorithm |
| `content_deck.rs` | Content database with difficulty gating and selection |
| `music_theory.rs` | Note/chord/scale/interval utilities |
| `level_format.rs` | Simple text-based level parser |
| `difficulty.rs` | Dynamic difficulty controller |
| `streak.rs` | Streak/combo tracking utility |

#### Extended Existing Modules

| Module | Extension |
|--------|-----------|
| `components/force_field.rs` | Add `FieldType::LinearCorridor { ax, ay, bx, by, falloff_width, velocity_coupling }` |
| `components/ghost_trail.rs` | Add `glow_enabled`, `glow_alpha_multiplier`, `glow_width_multiplier`, `speed_width_scale`, capture speed in `GhostSnapshot` |
| `rendering/particles.rs` | Add `ParticleShape` enum (Circle, Star, Triangle, Square), add shape field to Particle |
| `rendering/screen_fx.rs` | Add `BorderGlow` effect type |
| `ui_canvas.rs` | Add `reference_size` for reference-coord scaling, add `Badge` and `ScrollRegion` widget types, add `min_touch_size` for touch zones |
| `rng.rs` | Add `seeded_shuffle<T>(slice, seed)` utility function |
| `sound.rs` | Add `delay: f64` field to command serialization (default 0.0). Add `queue_sequence()` helper. |
| `aim_preview.rs` | Add force inclusion/exclusion config, collision termination, trajectory utility function |
| `game_flow.rs` | Add sub-phase concept with timed auto-transitions |
| `density_field.rs` | Add force-potential sampling and color-mapped rendering |

### ECS Pattern for New Components

All new components follow the existing convention:

```rust
// components/force_immunity.rs
use super::SchemaInfo;

#[derive(Clone, Debug)]
pub struct ForceImmunity {
    pub factor: f64,       // 1.0 = full immunity, 0.0 = none
    pub decay_rate: f64,   // units per second
}

impl SchemaInfo for ForceImmunity {
    fn schema_name() -> &'static str { "ForceImmunity" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "factor": { "type": "f64", "default": 1.0 },
                "decay_rate": { "type": "f64", "default": 1.0 }
            }
        })
    }
}
```

### Dependency Graph (Simplified)

```
A4 (CommandQueue) ─── L3 (PersistQueue)
                  └── (future queues)

A1 (Audio Scheduling) ─── A2 (Layered Audio)

P5 (ForceImmunity) ─── P6 (Drag + Immunity)
                   └── P4 (Sling/Launch) ─── R4 (Aim Preview)

U1 (Ref-Coord Layout) ─── U2 (Badges)
                       └── U3 (Touch Zones)

L1 (SRS) ─── L2 (Content Deck) ─── G5 (Challenge Pipeline)

G4 (Deterministic Shuffle) ─── G5 (Challenge Pipeline)
                            └── L2 (Content Deck)

Everything else is independent.
```

### Testing Strategy

Each concept gets:
1. **Unit tests** in the module file (`#[cfg(test)] mod tests`)
2. **Integration test** in `tests.rs` if it touches multiple systems
3. **No game-specific tests** -- test the engine primitive in isolation

The engine's determinism guarantee means all tests are reproducible. Use `SeededRng` for any test requiring randomness.

### Migration Path for Existing Games

Once concepts are extracted to the engine:

1. **gravity-pong** can replace its custom physics (plasma current, gravity immunity, adaptive substeps, trail rendering, field visualization, aim preview) with engine components and systems. Estimated: ~40% code reduction in game module.

2. **chord-reps** can replace its custom SRS, persist queue, layout system, and sound scheduling with engine modules. Estimated: ~50% code reduction by deleting `srs.rs`, `persist.rs`, `theory.rs` game modules and using engine equivalents.

Games keep their unique logic (level design, specific game rules, content data) but shed infrastructure.

---

*End of sprint plan. This document should be updated as concepts are implemented -- mark each concept with its completion status and any design decisions made during implementation.*
