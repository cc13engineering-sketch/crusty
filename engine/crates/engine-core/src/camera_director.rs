/// CameraDirector — Cinematic camera orchestration
///
/// Maintains a stack of camera modes (Follow, Pan, Zoom, Shake, Letterbox)
/// and a rule table that maps EventBus events to mode pushes with optional
/// auto-pop timers. Call `update()` once per frame to advance state and
/// write the result into the live Camera.

use crate::engine::Camera;
use crate::event_bus::EventBus;
use crate::ecs::World;
use crate::rendering::framebuffer::Framebuffer;
use crate::rendering::color::Color;

// ─── Easing ─────────────────────────────────────────────────────────────────

/// Easing curve applied to timed camera transitions.
#[derive(Clone, Debug, PartialEq)]
pub enum Easing {
    /// Constant rate — t.
    Linear,
    /// Quadratic ease-in — t².
    EaseIn,
    /// Quadratic ease-out — t·(2−t).
    EaseOut,
    /// Smooth-step ease-in-out — 3t²−2t³.
    EaseInOut,
}

impl Easing {
    /// Evaluate the easing function at normalised time `t` ∈ [0, 1].
    pub fn apply(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => t * (2.0 - t),
            Easing::EaseInOut => t * t * (3.0 - 2.0 * t),
        }
    }
}

// ─── CameraMode ──────────────────────────────────────────────────────────────

/// A discrete camera behaviour pushed onto the director stack.
#[derive(Clone, Debug)]
pub enum CameraMode {
    /// Continuously follow the first entity whose tags include `tag`.
    /// `lead` pushes the camera ahead of movement; `dead_zone` is the radius
    /// around the target in which the camera does not move.
    Follow {
        tag: String,
        lead: f64,
        dead_zone: f64,
    },
    /// Animate the camera position to (`to_x`, `to_y`) over `duration` seconds.
    /// `from_x`/`from_y` are captured automatically on first application.
    Pan {
        to_x: f64,
        to_y: f64,
        duration: f64,
        easing: Easing,
        from_x: Option<f64>,
        from_y: Option<f64>,
    },
    /// Animate camera zoom to `scale` over `duration` seconds.
    /// `from_scale` is captured automatically on first application.
    Zoom {
        scale: f64,
        duration: f64,
        easing: Easing,
        from_scale: Option<f64>,
    },
    /// Procedural shake around the camera position at push time.
    /// `base_x`/`base_y` are captured automatically on first application.
    Shake {
        intensity: f64,
        frequency: f64,
        duration: f64,
        base_x: Option<f64>,
        base_y: Option<f64>,
    },
    /// Draw horizontal letterbox bars (cinematic bars) over the framebuffer.
    Letterbox {
        bar_height: f64,
        fade_duration: f64,
    },
}

// ─── CameraRule ──────────────────────────────────────────────────────────────

/// Declarative rule: when the EventBus publishes `on_event`, push `push_mode`
/// onto the stack. If `pop_after` is set, the mode is automatically popped
/// after that many seconds.
#[derive(Clone, Debug)]
pub struct CameraRule {
    /// EventBus channel name that triggers this rule.
    pub on_event: String,
    /// The camera mode to push when the event fires.
    pub push_mode: CameraMode,
    /// If `Some(t)`, auto-pop the mode after `t` seconds.
    pub pop_after: Option<f64>,
}

// ─── CameraDirector ──────────────────────────────────────────────────────────

/// Orchestrates cinematic camera modes via a stack + rule table.
///
/// # Lifecycle
/// 1. Register rules with [`add_rule`].
/// 2. Call [`update`] once per frame (after gameplay, before rendering).
/// 3. The director modifies `camera.x`, `camera.y`, `camera.zoom` in-place.
/// 4. Call [`apply_letterbox`] after the main render pass to draw cinematic bars.
/// 5. Call [`clear`] (or let `Engine::reset` do so) between levels.
#[derive(Clone, Debug, Default)]
pub struct CameraDirector {
    /// Active mode stack. Each entry is `(mode, elapsed_seconds)`.
    pub stack: Vec<(CameraMode, f64)>,
    /// Declarative event → mode push rules.
    pub rules: Vec<CameraRule>,
    /// Pending auto-pop timers: `(stack_index, remaining_seconds)`.
    pub auto_pops: Vec<(usize, f64)>,
}

impl CameraDirector {
    /// Create an empty director with no rules or active modes.
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a new camera mode onto the top of the stack.
    pub fn push(&mut self, mode: CameraMode) {
        self.stack.push((mode, 0.0));
    }

    /// Pop the top-most camera mode (no-op on empty stack).
    pub fn pop(&mut self) {
        if self.stack.is_empty() {
            return;
        }
        let removed_idx = self.stack.len() - 1;
        self.stack.pop();
        // Drop any auto-pop timers that referenced the removed index.
        self.auto_pops.retain(|(idx, _)| *idx != removed_idx);
        // Shift remaining timers whose indices are above the removed entry.
        for (idx, _) in &mut self.auto_pops {
            if *idx > removed_idx {
                *idx -= 1;
            }
        }
    }

    /// Add a declarative rule.
    pub fn add_rule(&mut self, rule: CameraRule) {
        self.rules.push(rule);
    }

    /// Number of entries currently on the stack.
    pub fn stack_depth(&self) -> usize {
        self.stack.len()
    }

    /// Clear the stack, rules, and pending auto-pop timers.
    pub fn clear(&mut self) {
        self.stack.clear();
        self.rules.clear();
        self.auto_pops.clear();
    }

    /// Update the director for one frame.
    ///
    /// 1. Scans the EventBus for rule triggers and pushes matching modes.
    /// 2. Ticks elapsed time on every stack entry.
    /// 3. Ticks and fires auto-pop timers.
    /// 4. Applies the **top-most** mode to the camera (modes below it on the
    ///    stack are dormant but preserved).
    pub fn update(&mut self, dt: f64, camera: &mut Camera, bus: &EventBus, world: &World) {
        // ── Rule evaluation ───────────────────────────────────────────────
        // Collect rule indices that match a fired event to avoid borrowing
        // self.rules while mutating self.stack.
        let mut triggers: Vec<usize> = Vec::new();
        for (i, rule) in self.rules.iter().enumerate() {
            if bus.has(&rule.on_event) {
                triggers.push(i);
            }
        }
        for rule_idx in triggers {
            // Clone so we can call push() without holding a reference into self.rules.
            let (mode, pop_after) = {
                let rule = &self.rules[rule_idx];
                (rule.push_mode.clone(), rule.pop_after)
            };
            self.push(mode);
            if let Some(duration) = pop_after {
                let stack_idx = self.stack.len() - 1;
                self.auto_pops.push((stack_idx, duration));
            }
        }

        // ── Tick elapsed on every stack entry ─────────────────────────────
        for (_, elapsed) in &mut self.stack {
            *elapsed += dt;
        }

        // ── Auto-pop timers ───────────────────────────────────────────────
        // Tick each timer; collect the ones that have expired.
        let mut expired_stack_indices: Vec<usize> = Vec::new();
        for (stack_idx, remaining) in &mut self.auto_pops {
            *remaining -= dt;
            if *remaining <= 0.0 {
                expired_stack_indices.push(*stack_idx);
            }
        }
        // Remove expired auto-pops and their stack entries.
        // Sort descending so removing by index doesn't shift lower entries.
        expired_stack_indices.sort_unstable_by(|a, b| b.cmp(a));
        expired_stack_indices.dedup();
        for idx in &expired_stack_indices {
            if *idx < self.stack.len() {
                self.stack.remove(*idx);
            }
        }
        // Rebuild auto_pops, dropping entries that expired and adjusting indices.
        self.auto_pops.retain(|(_, remaining)| *remaining > 0.0);
        // Fix indices after removal (simple O(n²) pass; stacks are tiny).
        for removed_idx in &expired_stack_indices {
            for (idx, _) in &mut self.auto_pops {
                if *idx > *removed_idx {
                    *idx = idx.saturating_sub(1);
                }
            }
        }

        // ── Apply top-most mode to camera ─────────────────────────────────
        if let Some((mode, elapsed)) = self.stack.last_mut() {
            let elapsed = *elapsed;
            match mode {
                CameraMode::Follow { tag, lead: _, dead_zone } => {
                    // Locate the first entity with the requested tag.
                    // Use sorted_entities for deterministic selection when
                    // multiple entities share the same tag.
                    let mut target: Option<(f64, f64)> = None;
                    for entity in world.tags.sorted_entities() {
                        if let Some(tags) = world.tags.get(entity) {
                            if tags.has_str(tag) {
                                if let Some(t) = world.transforms.get(entity) {
                                    target = Some((t.x, t.y));
                                    break;
                                }
                            }
                        }
                    }
                    if let Some((tx, ty)) = target {
                        let dx = tx - camera.x;
                        let dy = ty - camera.y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        // Only move camera if target is outside the dead zone.
                        if dist > *dead_zone {
                            camera.x = tx;
                            camera.y = ty;
                        }
                    }
                }
                CameraMode::Pan { to_x, to_y, duration, easing, from_x, from_y } => {
                    if *duration > 0.0 {
                        // Capture start position on first application
                        let sx = *from_x.get_or_insert(camera.x);
                        let sy = *from_y.get_or_insert(camera.y);
                        let t = easing.apply((elapsed / *duration).min(1.0));
                        camera.x = sx + (*to_x - sx) * t;
                        camera.y = sy + (*to_y - sy) * t;
                    } else {
                        camera.x = *to_x;
                        camera.y = *to_y;
                    }
                }
                CameraMode::Zoom { scale, duration, easing, from_scale } => {
                    if *duration > 0.0 {
                        // Capture start scale on first application
                        let ss = *from_scale.get_or_insert(camera.zoom);
                        let t = easing.apply((elapsed / *duration).min(1.0));
                        camera.zoom = ss + (*scale - ss) * t;
                    } else {
                        camera.zoom = *scale;
                    }
                }
                CameraMode::Shake { intensity, frequency, duration, base_x, base_y } => {
                    // Capture base position on first application
                    let bx = *base_x.get_or_insert(camera.x);
                    let by = *base_y.get_or_insert(camera.y);
                    if elapsed < *duration {
                        // Decay intensity linearly toward zero as the shake expires.
                        let remaining = (*duration - elapsed).max(0.0);
                        let factor = remaining / *duration;
                        let decayed = *intensity * factor;
                        // Use elapsed time + frequency to produce deterministic
                        // pseudo-random offsets without requiring a PRNG state.
                        let phase_x = elapsed * *frequency * std::f64::consts::TAU;
                        let phase_y = elapsed * *frequency * std::f64::consts::TAU + 1.5707963;
                        camera.x = bx + phase_x.sin() * decayed;
                        camera.y = by + phase_y.sin() * decayed;
                    } else {
                        // Shake expired — restore base position
                        camera.x = bx;
                        camera.y = by;
                    }
                }
                CameraMode::Letterbox { .. } => {
                    // Letterbox does not modify camera coordinates; it is
                    // rendered in apply_letterbox() instead.
                }
            }
        }
    }

    /// Draw cinematic letterbox bars if the top-most mode is `Letterbox`.
    ///
    /// Call this **after** all scene rendering but **before** post-FX so that
    /// the bars cover gameplay pixels but sit under post-processing overlays.
    pub fn apply_letterbox(&self, fb: &mut Framebuffer) {
        // Find the topmost Letterbox entry.
        let letterbox = self.stack.iter().rev().find_map(|(mode, elapsed)| {
            if let CameraMode::Letterbox { bar_height, fade_duration } = mode {
                Some((*bar_height, *fade_duration, *elapsed))
            } else {
                None
            }
        });

        let (bar_height, fade_duration, elapsed) = match letterbox {
            Some(v) => v,
            None => return,
        };

        // Fade alpha: ramp from 0 to 255 over `fade_duration` seconds.
        let alpha = if fade_duration <= 0.0 {
            255u8
        } else {
            let t = (elapsed / fade_duration).clamp(0.0, 1.0);
            (t * 255.0) as u8
        };

        let bar_px = bar_height as i32;
        let w = fb.width as i32;
        let h = fb.height as i32;
        let bar_color = Color::from_rgba(0, 0, 0, alpha);

        // Top bar
        for y in 0..bar_px.min(h) {
            for x in 0..w {
                fb.set_pixel_blended(x, y, bar_color);
            }
        }

        // Bottom bar
        let bottom_start = (h - bar_px).max(0);
        for y in bottom_start..h {
            for x in 0..w {
                fb.set_pixel_blended(x, y, bar_color);
            }
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_bus::EventBus;
    use crate::engine::Camera;
    use crate::ecs::World;
    use crate::rendering::framebuffer::Framebuffer;

    fn make_camera() -> Camera {
        Camera::default()
    }

    fn make_world() -> World {
        World::new()
    }

    fn make_bus() -> EventBus {
        EventBus::new()
    }

    // ── Stack push / pop ──────────────────────────────────────────────────

    #[test]
    fn new_director_is_empty() {
        let d = CameraDirector::new();
        assert_eq!(d.stack_depth(), 0);
        assert!(d.rules.is_empty());
        assert!(d.auto_pops.is_empty());
    }

    #[test]
    fn push_increases_depth() {
        let mut d = CameraDirector::new();
        d.push(CameraMode::Shake { intensity: 5.0, frequency: 10.0, duration: 0.5, base_x: None, base_y: None });
        assert_eq!(d.stack_depth(), 1);
        d.push(CameraMode::Zoom { scale: 2.0, duration: 1.0, easing: Easing::Linear, from_scale: None });
        assert_eq!(d.stack_depth(), 2);
    }

    #[test]
    fn pop_decreases_depth() {
        let mut d = CameraDirector::new();
        d.push(CameraMode::Shake { intensity: 5.0, frequency: 10.0, duration: 0.5, base_x: None, base_y: None });
        d.push(CameraMode::Zoom { scale: 2.0, duration: 1.0, easing: Easing::Linear, from_scale: None });
        d.pop();
        assert_eq!(d.stack_depth(), 1);
        d.pop();
        assert_eq!(d.stack_depth(), 0);
    }

    #[test]
    fn pop_on_empty_stack_is_noop() {
        let mut d = CameraDirector::new();
        d.pop(); // should not panic
        assert_eq!(d.stack_depth(), 0);
    }

    #[test]
    fn clear_resets_everything() {
        let mut d = CameraDirector::new();
        d.push(CameraMode::Shake { intensity: 1.0, frequency: 5.0, duration: 1.0, base_x: None, base_y: None });
        d.add_rule(CameraRule {
            on_event: "boom".into(),
            push_mode: CameraMode::Shake { intensity: 8.0, frequency: 12.0, duration: 0.3, base_x: None, base_y: None },
            pop_after: Some(0.3),
        });
        d.auto_pops.push((0, 0.5));
        d.clear();
        assert_eq!(d.stack_depth(), 0);
        assert!(d.rules.is_empty());
        assert!(d.auto_pops.is_empty());
    }

    // ── Easing curves ──────────────────────────────────────────────────────

    #[test]
    fn easing_linear_at_boundaries() {
        let e = Easing::Linear;
        assert!((e.apply(0.0) - 0.0).abs() < 1e-12);
        assert!((e.apply(0.5) - 0.5).abs() < 1e-12);
        assert!((e.apply(1.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn easing_ease_in_at_boundaries() {
        let e = Easing::EaseIn;
        assert!((e.apply(0.0) - 0.0).abs() < 1e-12);
        // t=0.5 → 0.25
        assert!((e.apply(0.5) - 0.25).abs() < 1e-12);
        assert!((e.apply(1.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn easing_ease_out_at_boundaries() {
        let e = Easing::EaseOut;
        assert!((e.apply(0.0) - 0.0).abs() < 1e-12);
        // t=0.5 → 0.5*(2-0.5) = 0.75
        assert!((e.apply(0.5) - 0.75).abs() < 1e-12);
        assert!((e.apply(1.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn easing_ease_in_out_at_boundaries() {
        let e = Easing::EaseInOut;
        assert!((e.apply(0.0) - 0.0).abs() < 1e-12);
        // smoothstep(0.5) = 3*0.25 - 2*0.125 = 0.75 - 0.25 = 0.5
        assert!((e.apply(0.5) - 0.5).abs() < 1e-12);
        assert!((e.apply(1.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn easing_clamps_out_of_range_t() {
        for e in &[Easing::Linear, Easing::EaseIn, Easing::EaseOut, Easing::EaseInOut] {
            assert!((e.apply(-0.5) - e.apply(0.0)).abs() < 1e-12);
            assert!((e.apply(1.5) - e.apply(1.0)).abs() < 1e-12);
        }
    }

    // ── Dead-zone calculation (Follow mode) ───────────────────────────────

    #[test]
    fn follow_dead_zone_prevents_camera_move_when_inside() {
        let mut d = CameraDirector::new();
        let dead_zone = 50.0;
        d.push(CameraMode::Follow { tag: "player".into(), lead: 0.0, dead_zone });

        let mut world = make_world();
        // Spawn an entity tagged "player" at (10, 10) — well within dead_zone of origin.
        let e = world.spawn();
        world.transforms.insert(e, crate::components::Transform { x: 10.0, y: 10.0, ..Default::default() });
        world.tags.insert(e, crate::components::Tags::new(&["player"]));

        let mut camera = make_camera(); // camera at (0, 0)
        let bus = make_bus();
        d.update(0.016, &mut camera, &bus, &world);

        // Camera should not have moved because distance (≈14.1) < dead_zone (50).
        assert!((camera.x - 0.0).abs() < 1e-6, "camera.x should not move inside dead zone");
        assert!((camera.y - 0.0).abs() < 1e-6, "camera.y should not move inside dead zone");
    }

    #[test]
    fn follow_moves_camera_when_outside_dead_zone() {
        let mut d = CameraDirector::new();
        let dead_zone = 5.0;
        d.push(CameraMode::Follow { tag: "player".into(), lead: 0.0, dead_zone });

        let mut world = make_world();
        let e = world.spawn();
        // Target at (200, 100) — far outside any reasonable dead zone.
        world.transforms.insert(e, crate::components::Transform { x: 200.0, y: 100.0, ..Default::default() });
        world.tags.insert(e, crate::components::Tags::new(&["player"]));

        let mut camera = make_camera();
        let bus = make_bus();
        d.update(0.016, &mut camera, &bus, &world);

        // Camera should snap to target position.
        assert!((camera.x - 200.0).abs() < 1e-6);
        assert!((camera.y - 100.0).abs() < 1e-6);
    }

    // ── Shake decay ───────────────────────────────────────────────────────

    #[test]
    fn shake_applies_nonzero_offset_early_in_duration() {
        let mut d = CameraDirector::new();
        d.push(CameraMode::Shake { intensity: 20.0, frequency: 5.0, duration: 1.0, base_x: None, base_y: None });

        let mut camera = make_camera();
        let bus = make_bus();
        let world = make_world();
        // Tick slightly into the shake.
        d.update(0.05, &mut camera, &bus, &world);

        let moved = camera.x.abs() > 1e-6 || camera.y.abs() > 1e-6;
        assert!(moved, "Shake should offset camera position");
    }

    #[test]
    fn shake_has_no_effect_after_duration() {
        let mut d = CameraDirector::new();
        d.push(CameraMode::Shake { intensity: 20.0, frequency: 5.0, duration: 0.5, base_x: None, base_y: None });

        let mut camera = make_camera();
        let bus = make_bus();
        let world = make_world();
        // Tick past the full duration. The elapsed in the stack entry will be > 0.5.
        d.update(0.6, &mut camera, &bus, &world);

        // At elapsed > duration the shake branch should not fire offsets.
        // (camera starts at 0,0 and no other modes are active)
        assert!((camera.x).abs() < 1e-6, "No shake offset after duration expires");
        assert!((camera.y).abs() < 1e-6);
    }

    // ── Letterbox bar height ──────────────────────────────────────────────

    #[test]
    fn letterbox_renders_bars_at_full_alpha_after_fade() {
        let mut d = CameraDirector::new();
        let bar_height = 40.0;
        d.push(CameraMode::Letterbox { bar_height, fade_duration: 0.0 });

        let mut fb = Framebuffer::new(320, 240);
        // Fill framebuffer with white so bars are visible.
        for p in fb.pixels.chunks_exact_mut(4) {
            p[0] = 255; p[1] = 255; p[2] = 255; p[3] = 255;
        }

        d.apply_letterbox(&mut fb);

        // The first row of pixels should now be black (letterbox bar).
        let r = fb.pixels[0];
        let g = fb.pixels[1];
        let b = fb.pixels[2];
        assert_eq!(r, 0, "Top bar row 0 should be black (r)");
        assert_eq!(g, 0, "Top bar row 0 should be black (g)");
        assert_eq!(b, 0, "Top bar row 0 should be black (b)");
    }

    #[test]
    fn letterbox_bar_height_covers_correct_rows() {
        let mut d = CameraDirector::new();
        let bar_height = 20.0;
        d.push(CameraMode::Letterbox { bar_height, fade_duration: 0.0 });

        let w = 100u32;
        let h = 100u32;
        let mut fb = Framebuffer::new(w, h);
        for p in fb.pixels.chunks_exact_mut(4) {
            p[0] = 200; p[1] = 200; p[2] = 200; p[3] = 255;
        }

        d.apply_letterbox(&mut fb);

        // Row 19 (last bar row) should be black.
        let idx_19 = (19 * w as usize) * 4;
        assert_eq!(fb.pixels[idx_19], 0, "Row 19 should be covered by top bar");

        // Row 20 should still be the original grey.
        let idx_20 = (20 * w as usize) * 4;
        assert_eq!(fb.pixels[idx_20], 200, "Row 20 should NOT be covered by top bar");
    }

    #[test]
    fn no_letterbox_mode_applies_nothing() {
        let d = CameraDirector::new(); // empty stack
        let mut fb = Framebuffer::new(64, 64);
        for p in fb.pixels.chunks_exact_mut(4) {
            p[0] = 128; p[1] = 128; p[2] = 128; p[3] = 255;
        }
        d.apply_letterbox(&mut fb);
        // First pixel should still be grey.
        assert_eq!(fb.pixels[0], 128);
    }

    // ── Rule event matching ───────────────────────────────────────────────

    #[test]
    fn rule_fires_on_matching_event() {
        let mut d = CameraDirector::new();
        d.add_rule(CameraRule {
            on_event: "explosion".into(),
            push_mode: CameraMode::Shake { intensity: 10.0, frequency: 8.0, duration: 0.5, base_x: None, base_y: None },
            pop_after: None,
        });

        let mut bus = make_bus();
        bus.emit("explosion");

        let mut camera = make_camera();
        let world = make_world();
        assert_eq!(d.stack_depth(), 0);
        d.update(0.016, &mut camera, &bus, &world);
        assert_eq!(d.stack_depth(), 1, "Rule should have pushed Shake onto the stack");
    }

    #[test]
    fn rule_does_not_fire_on_non_matching_event() {
        let mut d = CameraDirector::new();
        d.add_rule(CameraRule {
            on_event: "explosion".into(),
            push_mode: CameraMode::Shake { intensity: 10.0, frequency: 8.0, duration: 0.5, base_x: None, base_y: None },
            pop_after: None,
        });

        let mut bus = make_bus();
        bus.emit("other_event");

        let mut camera = make_camera();
        let world = make_world();
        d.update(0.016, &mut camera, &bus, &world);
        assert_eq!(d.stack_depth(), 0, "Non-matching event should not trigger rule");
    }

    // ── Rule auto-pop ─────────────────────────────────────────────────────

    #[test]
    fn rule_auto_pop_removes_mode_after_timeout() {
        let mut d = CameraDirector::new();
        d.add_rule(CameraRule {
            on_event: "cutscene".into(),
            push_mode: CameraMode::Zoom { scale: 2.0, duration: 0.0, easing: Easing::Linear, from_scale: None },
            pop_after: Some(0.1),
        });

        let mut bus = make_bus();
        bus.emit("cutscene");

        let mut camera = make_camera();
        let world = make_world();

        // First update: event fires → push occurs.
        d.update(0.016, &mut camera, &bus, &world);
        assert_eq!(d.stack_depth(), 1, "Mode should be pushed after event");

        // Subsequent updates with a silent bus: let the auto-pop timer expire.
        let silent_bus = make_bus();
        d.update(0.05, &mut camera, &silent_bus, &world);
        assert_eq!(d.stack_depth(), 1, "Mode should still be present after 66ms total");

        d.update(0.05, &mut camera, &silent_bus, &world);
        // Total elapsed on timer ≈ 0.116s > 0.1s threshold → should be popped.
        assert_eq!(d.stack_depth(), 0, "Mode should be auto-popped after timeout");
    }

    #[test]
    fn manual_push_without_auto_pop_persists() {
        let mut d = CameraDirector::new();
        d.push(CameraMode::Zoom { scale: 1.5, duration: 1.0, easing: Easing::EaseInOut, from_scale: None });

        let bus = make_bus();
        let world = make_world();
        let mut camera = make_camera();

        // Advance well past any reasonable auto-pop; the mode should remain.
        for _ in 0..60 {
            d.update(0.016, &mut camera, &bus, &world);
        }
        assert_eq!(d.stack_depth(), 1, "Manual push with no auto-pop should persist");
    }

    // ── Zoom + Pan mode application ───────────────────────────────────────

    #[test]
    fn zoom_mode_instant_sets_zoom() {
        let mut d = CameraDirector::new();
        d.push(CameraMode::Zoom { scale: 3.0, duration: 0.0, easing: Easing::Linear, from_scale: None });

        let mut camera = make_camera();
        let bus = make_bus();
        let world = make_world();
        d.update(0.016, &mut camera, &bus, &world);

        assert!((camera.zoom - 3.0).abs() < 1e-6, "Instant zoom should set camera.zoom");
    }

    #[test]
    fn pan_mode_instant_sets_position() {
        let mut d = CameraDirector::new();
        d.push(CameraMode::Pan { to_x: 400.0, to_y: 300.0, duration: 0.0, easing: Easing::Linear, from_x: None, from_y: None });

        let mut camera = make_camera();
        let bus = make_bus();
        let world = make_world();
        d.update(0.016, &mut camera, &bus, &world);

        assert!((camera.x - 400.0).abs() < 1e-6);
        assert!((camera.y - 300.0).abs() < 1e-6);
    }

    #[test]
    fn multiple_rules_can_fire_in_same_frame() {
        let mut d = CameraDirector::new();
        d.add_rule(CameraRule {
            on_event: "a".into(),
            push_mode: CameraMode::Shake { intensity: 5.0, frequency: 5.0, duration: 0.2, base_x: None, base_y: None },
            pop_after: None,
        });
        d.add_rule(CameraRule {
            on_event: "b".into(),
            push_mode: CameraMode::Zoom { scale: 2.0, duration: 0.5, easing: Easing::EaseOut, from_scale: None },
            pop_after: None,
        });

        let mut bus = make_bus();
        bus.emit("a");
        bus.emit("b");

        let mut camera = make_camera();
        let world = make_world();
        d.update(0.016, &mut camera, &bus, &world);

        assert_eq!(d.stack_depth(), 2, "Both rules should have fired");
    }
}
