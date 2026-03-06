# Code Review: March 5, 2026

Multi-expert review of all changes from the last 5 hours on the `gravity-pong` branch.
Covers ~3,300 lines added across 15 modified files and 9 new files.

---

## CRITICAL

### 1. SRS reviews the wrong card for ~35% of concept types
**File:** `engine/crates/engine-core/src/chord_reps/mod.rs:1378-1397`

For TriadInduction, SeventhInduction, ScaleTriads, ParallelMinor, and PivotChords, `self.challenge.answer` is the option display index (0-3), not the actual SRS variant. The spaced repetition system records reviews against the wrong cards, making scheduling incoherent over time.

**Fix:** Store the original SRS variant separately in `MusicChallenge` and use that for the `srs.review_card()` call instead of `self.challenge.answer`.

### 2. WASM bindings out of sync with HTML pages
**Files:** `site/chord-reps/index.html`, `site/gravity-pong/index.html`, `site/demo-ball/index.html`

- chord-reps imports 8 functions (`setup_chord_reps`, `drain_persist_commands`, `set_game_state_str`, `get_game_state_str`, `clear_game_state`, `set_url_param`, `browser_state_ptr`, `browser_state_len`) that don't exist in the built WASM. Page crashes on load.
- gravity-pong and demo-ball both import `set_url_param` which also doesn't exist -- crashes if loaded with query parameters.

**Fix:** Rebuild WASM (`wasm-pack build`) to generate exports matching the current Rust source.

---

## MEDIUM

### 3. `next_f64()` returns `(0, 1]` instead of `[0, 1)`
**File:** `engine/crates/engine-core/src/rng.rs:24`

Divides by `u64::MAX` so can return exactly 1.0 but never 0.0. This means `chance(1.0)` can return false, and `range_f64(min, max)` can return `max` (exclusive upper bound violated).

**Fix:** Use `(self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)` for proper `[0, 1)`.

### 4. Deduction progress counter always shows "0 / 3 notes selected"
**File:** `engine/crates/engine-core/src/chord_reps/mod.rs:2625-2635`

TriadDeduction and SeventhDeduction use `SetPiano` mode (toggle keys), but the progress counter reads `pending_notes.len()` which is only populated by `SequentialPiano` mode.

**Fix:** Count from `self.toggled_keys.iter().filter(|&&t| t).count()` for SetPiano mode challenges.

### 5. Ambient particle forces push in wrong directions
**File:** `engine/crates/engine-core/src/gravity_pong/mod.rs:1015-1016`

`collect_ambient_forces` pushes two entries per gravity well with forces in fixed +X and +Y directions, rather than radially toward/away from the well. Ambient particles swirl incorrectly around gravity sources.

**Fix:** Compute radial force direction from each ambient particle's position relative to the well.

### 6. Wall restitution applies to tangential velocity (unintended friction)
**File:** `engine/crates/engine-core/src/gravity_pong/mod.rs:3163-3164`

`(vx - 2.0 * dot * nx) * wall.restitution` scales the entire reflected vector, not just the normal component. Glancing bounces lose tangential speed, making walls feel "sticky."

**Fix:** Use `vx - (1.0 + wall.restitution) * dot * nx` to only affect the normal component.

### 7. Aim preview overestimates trajectory
**File:** `engine/crates/engine-core/src/gravity_pong/mod.rs:2718`

Preview uses constant low drag (sling immunity) for 120 steps, but real physics decays immunity at 2.0/sec, ramping drag from 0.06 to 0.3. Preview shows particles going much further than reality.

**Fix:** Simulate immunity decay in the preview loop to match actual physics.

### 8. `sound_energy` never ticked by engine
**File:** `engine/crates/engine-core/src/engine.rs`

Engine owns `sound_energy` but `tick()` never calls `sound_energy.tick(dt)`. Only works because gravity_pong manually calls it. Other game modes would see energy levels that never decay.

**Fix:** Add `self.sound_energy.tick(dt)` to `Engine::tick()`.

### 9. Simulation systems run at variable rate despite fixed-timestep docs
**File:** `engine/crates/engine-core/src/engine.rs:780-823`

Only physics systems run inside the accumulator loop. Simulation systems (lifecycle, tween, waypoint, etc.) run once per `tick()` call at whatever frame rate the browser delivers, but receive `FIXED_DT` as their dt.

**Fix:** Move simulation systems inside the accumulator loop, or update docs/dt to reflect actual behavior.

### 10. `frame_metrics` not reset in `Engine::reset()`
**File:** `engine/crates/engine-core/src/engine.rs:529-573`

Every other subsystem is reset, but `frame_metrics` retains stale data. JS consumers reading metrics between reset and first tick see old values.

**Fix:** Add `self.frame_metrics = FrameMetrics::default();` to `reset()`.

### 11. `Won` phase only lasts one frame
**File:** `engine/crates/engine-core/src/gravity_pong/mod.rs:2931-2934`

`check_win_loss()` sets `Won`, then `update_phase()` immediately replaces it with `LevelTransition` on the same frame. JS HUD only sees `phase=1.0` for a single frame -- victory UI may flicker or be missed.

**Fix:** Add a minimum dwell time on `Won` before transitioning, or have JS check for `LevelTransition` as the victory indicator.

---

## LOW

### 12. `WAYPOINT_MIN_SPEED = 0.0` prevents natural capture
**File:** `engine/crates/engine-core/src/gravity_pong/mod.rs:1300-1312`

Waypoints only capture particles with exactly zero velocity. Particles that slow through drag have floating-point residual velocity (~1e-15) and are never captured -- only those hit by the hard-stop timer.

**Fix:** Set `WAYPOINT_MIN_SPEED` to a small epsilon like `0.5` or `1.0`.

### 13. Duplicate/conflicting modules
- `music_theory.rs` vs `chord_reps/theory.rs` -- both define `ChordQuality` with different variants (7 vs 4).
- `srs.rs` vs `chord_reps/srs.rs` -- different SM-2 interval schedules (6-day vs 3-day second rep). Only `chord_reps/srs.rs` is used.

**Fix:** Consolidate into single modules. Delete unused standalone versions or merge them.

### 14. Debug logging in hot path
**File:** `engine/crates/engine-core/src/gravity_pong/mod.rs:1512-1521`

Logs particle state once/sec per particle to browser console. Comment says "Debug" -- left in by mistake.

**Fix:** Remove or gate behind `engine.debug_mode`.

### 15. Dead CSS in chord-reps page
**File:** `site/chord-reps/index.html`

Styles for `#attribution`, `h1`, `.info`, `.version` -- none of these elements exist in the HTML. Copy-paste artifact from gravity-pong page.

**Fix:** Remove dead CSS rules.

### 16. `crusty-ui.js` is never imported
**File:** `site/crusty-ui.js`

113-line shared JS module created but no page loads it. Dead code.

**Fix:** Either wire it into pages or remove it.

### 17. ENGINE RefCell held across sim calls (latent panic)
**File:** `engine/crates/engine-core/src/lib.rs:97-119`

ENGINE's `RefCell` borrow_mut is held for the entire `tick()` duration including `sim.step(eng)` and `sim.render(eng)`. If simulation code ever re-enters the WASM API, it will panic with `BorrowMutError`. Not triggered today but a trap for future code.

**Fix:** Drop the ENGINE borrow before calling sim methods, passing owned/cloned data instead, or document the constraint.

### 18. Interval notes can exceed piano display range
**File:** `engine/crates/engine-core/src/chord_reps/mod.rs:539-541`

Top note of an interval challenge can reach MIDI 80, but the piano keyboard only renders MIDI 48-72. The interval is audible but not visible.

**Fix:** Clamp `base_offset` so that `base_midi + max_interval <= MIDI_HIGH`.

---

## Recommended Fix Priority

1. Rebuild WASM (unblocks chord-reps page entirely)
2. Fix SRS variant tracking (core learning feature silently broken)
3. Fix `next_f64()` range (affects all randomness downstream)
4. Fix deduction progress counter (visible UI bug)
5. Fix ambient particle force directions (visual artifact)
6. Remove debug logging (console spam)
7. Everything else as time permits
