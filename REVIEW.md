# Crusty Engine — Expert Review

Multi-domain expert review of architecture, physics, rendering, games, web integration, and advanced systems. March 2026.

**Codebase**: ~12K LOC Rust engine-core, 3 games, 26 headless modules, 1200+ tests.

---

## Scorecard

| Domain | Area | Score |
|--------|------|-------|
| **Physics** | CCD implementation | 9/10 |
| | Fixed timestep handling | 9/10 |
| | Numerical stability | 8/10 |
| | Force accumulation (Plummer softening) | 8/10 |
| | Integration method (symplectic Euler) | 7/10 |
| | Collision response | 7/10 |
| | Spatial grid | 7/10 |
| **Rendering** | Anti-aliasing (SDF feather) | 8/10 |
| | Shape primitives (tapered trail standout) | 8/10 |
| | Renderer architecture | 7/10 |
| | Post-processing pipeline | 7/10 |
| | Text rendering | 6/10 |
| | Particle system | 6/10 |
| **Games** | Gravity Pong mechanics & originality | 9/10 |
| | Chord Reps music theory correctness | 9/10 |
| | Chord Reps SRS algorithm | 8/10 |
| | Gravity Pong game feel | 7/10 |
| | Gravity Pong progression | 7/10 |
| | Chord Reps learning flow | 7/10 |
| | Gravity Pong code organization | 6/10 |
| **Headless** | Testing infrastructure | 9/10 |
| | Determinism verification | 9/10 |
| | Ablation/hill-climb framework | 8/10 |
| **Advanced** | Behavior/FSM systems | 8/10 |
| | Tween system (9 easings) | 8/10 |
| | Event bus | 8/10 |
| | Telemetry (delta-compressed, RFC 8259) | 8/10 |
| | Coroutine system | 8/10 |
| | Pathfinding (A* with octile heuristic) | 8/10 |
| | Spatial/environment systems | 7/10 |
| **Web** | JS interop (shared-memory bridge) | 8/10 |
| | Input handling | 8/10 |
| | WASM binary size optimization | 8/10 |
| | Docs quality | 8/10 |
| | wasm-bindgen usage | 7/10 |
| | Canvas/rendering bridge | 7/10 |
| | Audio bridge | 7/10 |
| | HTML/CSS quality | 7/10 |
| | Responsive design | 6/10 |
| | Loading UX | 5/10 |
| | Accessibility | 3/10 |

**Overall: 7.5/10** — Well-engineered core with standout CCD, determinism, and headless testing. Main gaps: rendering performance, web polish, and game-engine integration.

---

## Standout Strengths

### 1. CCD is mathematically excellent
Circle vs circle/segment/AABB with correct quadratic ray-sphere intersection, Minkowski sum, endpoint fallbacks, and 17 edge-case tests. Zero-length segments, stationary objects, and initial overlap all handled. Epsilon constants well-chosen for f64.

### 2. Headless testing is best-in-class for an indie engine
Ablation studies with baseline + variants across multiple seeds. Hill climbing with coordinate descent and convergence detection. Statistical highlight detection with rolling mean/stddev and spike/drop/near-death/milestone classification. Golden test regression framework. All game-agnostic via Simulation trait.

### 3. Determinism guarantees are sound
Single xorshift64 RNG owned by Engine. Fixed 60Hz timestep with accumulator clamping. Monotonic entity IDs. Per-frame state hashing for verification. BTreeMap throughout headless for deterministic iteration. Replay recording with frame-level hash comparison.

### 4. Gravity Pong is genuinely original
Not Pong at all — orbital mechanics puzzle game with gravity wells, repulsors, black holes, wormholes, plasma currents, supernovas. Waypoint-then-sling two-step input. Dust motes as gravitational field visualization. Plummer softening prevents singularities, adaptive sub-stepping handles high accelerations.

### 5. Chord Reps music theory is excellent
Faithful SM-2 SRS with thoughtful modifications (hint-used = 0.5 day, fail = 0.1 day). Theory module is musically correct (scales, diatonic chords, cadences, MIDI). Educational insight texts include historical context, song references, and harmonic analysis.

### 6. Plummer-softened force fields
`F(r) = strength * r / (r² + ε²)^(3/2)` — standard N-body softening kernel. Smooth at r=0, peaks at r = ε/√2, falls off as 1/r² at large distances. Eliminates singularities without ad-hoc clamping.

### 7. Telemetry with delta compression
Only records changed values. Bitwise NaN comparison prevents spurious deltas. JSON serialization handles RFC 8259 control characters correctly. Most hand-rolled JSON serializers miss this.

---

## Critical Issues

### 1. Gravity Pong has zero audio
Rich visual juice (particles, screen shake, flash, trails, dust motes) but absolutely no sound. Launch, impact, scoring, black hole kills — all silent. The engine has a full sound command system that the game doesn't use.

### 2. Neither game uses engine support systems
AutoJuice, CameraDirector, GameFlow, FeelPreset, ContentDeck — all well-designed, all well-tested, none used by either game. Gravity Pong hand-rolls its own juice. Chord Reps manages its own state inline. Significant built infrastructure sitting idle.

### 3. fill_rect has no anti-aliasing
Engine convention: "All visual primitives MUST be anti-aliased (1px feather on edges)." fill_rect and draw_rect break this rule — they round coordinates and fill solid pixels. Every other shape primitive uses SDF-based AA.

### 4. Particle drag is frame-rate dependent
`p.vx *= 0.99` applied per frame, but `update()` receives variable dt. Should be `(0.99f64).powf(dt * 60.0)` for frame-rate independence.

### 5. Non-RNG random spawn in lifecycle.rs
`lifecycle.rs` uses `world.entity_count()` as a pseudo-seed for random spawn positions instead of the canonical `SeededRng`. Violates the "one canonical RNG" determinism rule.

### 6. No loading indicators on game pages
WASM modules take seconds to download and compile. Users see a blank page during this time. No loading spinners, no error fallbacks, no progressive enhancement.

### 7. Accessibility is minimal (3/10)
No ARIA labels, no landmark elements, no focus styles, no keyboard navigation in games. Canvas games are inherently limited, but docs and landing pages could easily be more accessible.

---

## Notable Issues

### Physics
- **No dynamic-dynamic circle collisions.** CCD only sweeps a moving circle against static obstacles. Two fast-moving circles approaching each other are not handled correctly.
- **No friction or mass-ratio in collision response.** Purely specular reflection scaled by max(restitution_a, restitution_b). Fine for circles vs walls, insufficient for dynamic-dynamic.
- **No render interpolation.** Fixed timestep without interpolation causes visual stuttering at low frame rates.
- **Damping NaN edge case.** `(1.0 - damping).powf(dt)` produces NaN when damping ≥ 1.0. No runtime clamp.

### Rendering
- **Per-pixel function call overhead.** Every shape calls draw_px → set_pixel_blended → bounds check per pixel. Scanline batching would significantly improve performance.
- **fill_tapered_trail is O(pixels × segments).** Every pixel in the bounding box tests against every segment. A trail with 20 segments evaluates hundreds of thousands of segment tests.
- **Post-fx are separate full-buffer passes.** Vignette + scanlines + tint + desaturate = 4 passes over ~2MB. A fused single-pass would cut memory bandwidth 4×.
- **Vignette recomputes sqrt per pixel per frame.** Should be cached since it never changes for a fixed resolution.
- **Screen shake clones entire framebuffer** (~2MB allocation per frame during shake).

### Games
- **Gravity Pong is one 2000+ line file.** Should be split into sub-modules (physics, entities, levels, rendering) following the Chord Reps pattern.
- **Chord Reps shows all 35 cards from the start.** A beginner might face cadence identification before mastering scale degrees. Needs concept-type unlocking gates.
- **No tutorial or onboarding.** Player must discover waypoint/sling mechanics by experimentation. Level 2 removes gravity wells, potentially confusing players who haven't found the sling.
- **SRS persistence asymmetry.** `from_json` uses serde_json::Value, `to_json` uses manual formatting. Could drift.

### Web
- **WebAudioFont loaded from third-party CDN** (`surikov.github.io`) with no SRI hash, no fallback, no local copy. CDN outage = chord-reps audio completely broken.
- **No resize handling.** Gravity Pong computes resolution at load time only. Window resize or phone rotation produces stretched canvas.
- **wasm-bindgen is unconditional dependency.** Compiled for native builds even though it's WASM-only. Pragmatic but impure.
- **CLI hardcoded to DemoBall.** No `--game` flag to run gravity-pong or chord-reps headless.
- **Empty catch blocks** in audio code swallow errors silently.

### Architecture
- **Duplicated CompareOp.** Two separate enums — behavior.rs (ε = 1e-9) and state_machine.rs (ε = f64::EPSILON). Should be unified.
- **save_load.rs captures only 6 of 32+ component types.** Restoring a snapshot loses colliders, renderables, behaviors, tweens, lifetimes, and more. SceneManager does it correctly (full World clone).
- **Coroutine SpawnTemplate uses name field as entity name**, not as template lookup key. Differs from lifecycle.rs behavior. Potentially surprising.
- **String allocations in StateMachine.** Every `transition_to` allocates via `to_string()`. Consider `&'static str` or interning.

---

## Recommendations (Priority Order)

### Ship-blocking
1. Add loading indicators to all game pages (CSS spinner, hide on WASM init)
2. Fix particle drag frame-rate dependence (`powf(dt * 60.0)`)
3. Fix lifecycle.rs non-RNG spawn (use `SeededRng`)
4. Add AA to fill_rect (1px SDF feather, matching all other primitives)

### High Impact
5. Add sound to Gravity Pong (launch whoosh, impact, scoring, black hole, ambient)
6. Bundle WebAudioFont locally or use SRI hash on CDN script
7. Add canvas resize handler to Gravity Pong
8. Split Gravity Pong into sub-modules (physics.rs, entities.rs, levels.rs, rendering.rs)
9. Add `--game` flag to CLI for multi-game headless testing
10. Add concept-type unlocking to Chord Reps (scale degrees first, then intervals, etc.)

### Polish
11. Cache vignette mask (compute once, reuse every frame)
12. Add `<meta name="description">` and OG tags to all pages
13. Add back-to-hub navigation on game pages
14. Unify CompareOp into a single canonical implementation
15. Add runtime clamp for damping (0.0..1.0)

### Future
16. Fuse post-fx passes into single buffer iteration
17. Add render interpolation for sub-60Hz displays
18. Scanline batching for shape rendering performance
19. Adopt AutoJuice/CameraDirector/GameFlow in games
20. Expand save_load.rs to capture all component types

---

*Review conducted by 5 specialist agents: Rust architecture, physics/rendering, game UX, WASM/web, advanced systems. Each read and analyzed 15-40 source files in depth.*
