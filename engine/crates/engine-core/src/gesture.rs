/// Touch gesture recognition system.
///
/// Detects tap, double-tap, long-press, swipe (with direction and velocity),
/// and pinch gestures from raw touch events. Forwards the primary touch to
/// Input mouse position for backwards compatibility with mouse-based gameplay.

use std::collections::HashMap;

/// A tracked touch point.
#[derive(Clone, Debug)]
pub struct TouchPoint {
    /// Platform touch identifier.
    pub id: u32,
    /// X coordinate when the touch started.
    pub start_x: f64,
    /// Y coordinate when the touch started.
    pub start_y: f64,
    /// Current X coordinate.
    pub current_x: f64,
    /// Current Y coordinate.
    pub current_y: f64,
    /// Elapsed time since touch started (in seconds).
    pub start_time: f64,
}

/// Direction of a swipe gesture.
#[derive(Clone, Debug, PartialEq)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Recognized gesture types.
#[derive(Clone, Debug)]
pub enum Gesture {
    /// A quick tap (touch down then up within tap thresholds).
    Tap { x: f64, y: f64 },
    /// Two taps in quick succession at roughly the same location.
    DoubleTap { x: f64, y: f64 },
    /// A touch held in place beyond the long-press duration.
    LongPress { x: f64, y: f64 },
    /// A directional swipe with velocity in pixels per second.
    Swipe { direction: SwipeDirection, velocity: f64, start_x: f64, start_y: f64, end_x: f64, end_y: f64 },
    /// A two-finger pinch/spread. `scale_delta` > 1.0 means spreading apart,
    /// < 1.0 means pinching together.
    Pinch { scale_delta: f64 },
}

/// Configuration thresholds for gesture recognition.
#[derive(Clone, Debug)]
pub struct GestureConfig {
    /// Maximum duration (seconds) for a touch to count as a tap.
    pub tap_max_duration: f64,
    /// Maximum distance (pixels) the finger can move and still count as a tap.
    pub tap_max_distance: f64,
    /// Maximum time window (seconds) between two taps to form a double-tap.
    pub double_tap_window: f64,
    /// Minimum hold duration (seconds) to trigger a long-press.
    pub long_press_duration: f64,
    /// Minimum distance (pixels) for a swipe to be recognized.
    pub swipe_min_distance: f64,
}

impl Default for GestureConfig {
    fn default() -> Self {
        Self {
            tap_max_duration: 0.3,
            tap_max_distance: 10.0,
            double_tap_window: 0.4,
            long_press_duration: 0.5,
            swipe_min_distance: 30.0,
        }
    }
}

/// Tracks the last tap for double-tap detection.
#[derive(Clone, Debug)]
struct LastTap {
    x: f64,
    y: f64,
    time: f64,
}

/// The main gesture recognizer. Feed it raw touch events and call `update(dt)`
/// each frame, then `drain_gestures()` to consume the recognized gestures.
#[derive(Clone, Debug)]
pub struct GestureRecognizer {
    /// Configuration thresholds.
    pub config: GestureConfig,
    /// Currently active touch points, keyed by touch id.
    active_touches: HashMap<u32, TouchPoint>,
    /// Queue of recognized gestures ready to be consumed.
    pending_gestures: Vec<Gesture>,
    /// Tracks the last tap for double-tap detection.
    last_tap: Option<LastTap>,
    /// Accumulated time since initialization (seconds).
    elapsed: f64,
    /// Previous distance between two fingers for pinch tracking.
    prev_pinch_distance: Option<f64>,
    /// Whether a long-press was already fired for the current single touch.
    long_press_fired: bool,
    /// The id of the primary (first) touch, used for mouse forwarding.
    primary_touch_id: Option<u32>,
}

impl GestureRecognizer {
    pub fn new() -> Self {
        Self {
            config: GestureConfig::default(),
            active_touches: HashMap::new(),
            pending_gestures: Vec::new(),
            last_tap: None,
            elapsed: 0.0,
            prev_pinch_distance: None,
            long_press_fired: false,
            primary_touch_id: None,
        }
    }

    /// Create a recognizer with custom configuration.
    pub fn with_config(config: GestureConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    /// Called when a new touch begins.
    /// Returns the touch position for optional mouse-forwarding by the caller.
    pub fn on_touch_start(&mut self, id: u32, x: f64, y: f64) -> (f64, f64) {
        let touch = TouchPoint {
            id,
            start_x: x,
            start_y: y,
            current_x: x,
            current_y: y,
            start_time: self.elapsed,
        };
        self.active_touches.insert(id, touch);

        // Track the primary touch (first finger down)
        if self.primary_touch_id.is_none() {
            self.primary_touch_id = Some(id);
        }

        // Reset long-press state when a new touch begins
        if self.active_touches.len() == 1 {
            self.long_press_fired = false;
        }

        // Reset pinch tracking when we get a second finger
        if self.active_touches.len() == 2 {
            self.prev_pinch_distance = Some(self.two_finger_distance());
        }

        (x, y)
    }

    /// Called when an active touch moves.
    /// Returns the touch position for optional mouse-forwarding by the caller.
    pub fn on_touch_move(&mut self, id: u32, x: f64, y: f64) -> Option<(f64, f64)> {
        if let Some(touch) = self.active_touches.get_mut(&id) {
            touch.current_x = x;
            touch.current_y = y;
        }

        // Handle pinch updates when two fingers are active
        if self.active_touches.len() == 2 {
            let new_dist = self.two_finger_distance();
            if let Some(prev_dist) = self.prev_pinch_distance {
                if prev_dist > 0.0 {
                    let scale_delta = new_dist / prev_dist;
                    // Only emit pinch if there's a meaningful change
                    if (scale_delta - 1.0).abs() > 0.005 {
                        self.pending_gestures.push(Gesture::Pinch { scale_delta });
                    }
                }
            }
            self.prev_pinch_distance = Some(new_dist);
        }

        // Return primary touch position for mouse forwarding
        if self.primary_touch_id == Some(id) {
            Some((x, y))
        } else {
            None
        }
    }

    /// Called when a touch ends (finger lifted).
    /// Returns the touch position for optional mouse-forwarding by the caller.
    pub fn on_touch_end(&mut self, id: u32, x: f64, y: f64) -> Option<(f64, f64)> {
        // Update final position before processing
        if let Some(touch) = self.active_touches.get_mut(&id) {
            touch.current_x = x;
            touch.current_y = y;
        }

        let result = if let Some(touch) = self.active_touches.get(&id) {
            let dx = touch.current_x - touch.start_x;
            let dy = touch.current_y - touch.start_y;
            let distance = (dx * dx + dy * dy).sqrt();
            let duration = self.elapsed - touch.start_time;

            // Only recognize single-finger gestures when there's one touch
            if self.active_touches.len() == 1 && !self.long_press_fired {
                if distance >= self.config.swipe_min_distance {
                    // Swipe gesture
                    let direction = if dx.abs() > dy.abs() {
                        if dx > 0.0 { SwipeDirection::Right } else { SwipeDirection::Left }
                    } else {
                        if dy > 0.0 { SwipeDirection::Down } else { SwipeDirection::Up }
                    };
                    let velocity = if duration > 0.0 { distance / duration } else { 0.0 };
                    self.pending_gestures.push(Gesture::Swipe {
                        direction,
                        velocity,
                        start_x: touch.start_x,
                        start_y: touch.start_y,
                        end_x: touch.current_x,
                        end_y: touch.current_y,
                    });
                } else if duration <= self.config.tap_max_duration
                    && distance <= self.config.tap_max_distance
                {
                    // Potential tap — check for double-tap first
                    let is_double = if let Some(ref last) = self.last_tap {
                        let tap_dx = touch.start_x - last.x;
                        let tap_dy = touch.start_y - last.y;
                        let tap_dist = (tap_dx * tap_dx + tap_dy * tap_dy).sqrt();
                        let tap_dt = self.elapsed - last.time;
                        tap_dt <= self.config.double_tap_window
                            && tap_dist <= self.config.tap_max_distance * 2.0
                    } else {
                        false
                    };

                    if is_double {
                        self.pending_gestures.push(Gesture::DoubleTap {
                            x: touch.current_x,
                            y: touch.current_y,
                        });
                        self.last_tap = None;
                    } else {
                        self.pending_gestures.push(Gesture::Tap {
                            x: touch.current_x,
                            y: touch.current_y,
                        });
                        self.last_tap = Some(LastTap {
                            x: touch.current_x,
                            y: touch.current_y,
                            time: self.elapsed,
                        });
                    }
                }
            }

            // Return primary touch position for mouse forwarding
            if self.primary_touch_id == Some(id) {
                Some((touch.current_x, touch.current_y))
            } else {
                None
            }
        } else {
            None
        };

        self.active_touches.remove(&id);

        // Clear primary touch tracking when that finger lifts
        if self.primary_touch_id == Some(id) {
            self.primary_touch_id = None;
        }

        // Reset pinch state when we drop below 2 fingers
        if self.active_touches.len() < 2 {
            self.prev_pinch_distance = None;
        }

        result
    }

    /// Called each frame with the delta time. Checks for time-based gestures
    /// such as long-press.
    pub fn update(&mut self, dt: f64) {
        self.elapsed += dt;

        // Check for long-press on a single stationary touch
        if self.active_touches.len() == 1 && !self.long_press_fired {
            // Get the single active touch
            let touch_data = self.active_touches.values().next().map(|t| {
                let dx = t.current_x - t.start_x;
                let dy = t.current_y - t.start_y;
                let distance = (dx * dx + dy * dy).sqrt();
                let duration = self.elapsed - t.start_time;
                (t.current_x, t.current_y, distance, duration)
            });

            if let Some((cx, cy, distance, duration)) = touch_data {
                if duration >= self.config.long_press_duration
                    && distance <= self.config.tap_max_distance
                {
                    self.pending_gestures.push(Gesture::LongPress { x: cx, y: cy });
                    self.long_press_fired = true;
                }
            }
        }

        // Expire old last_tap if the double-tap window has passed
        if let Some(ref last) = self.last_tap {
            if self.elapsed - last.time > self.config.double_tap_window {
                self.last_tap = None;
            }
        }
    }

    /// Drain all recognized gestures. Returns them and clears the internal queue.
    pub fn drain_gestures(&mut self) -> Vec<Gesture> {
        std::mem::take(&mut self.pending_gestures)
    }

    /// Returns the number of currently active touches.
    pub fn active_touch_count(&self) -> usize {
        self.active_touches.len()
    }

    /// Returns the primary touch id, if any finger is currently down.
    pub fn primary_touch(&self) -> Option<u32> {
        self.primary_touch_id
    }

    /// Compute the distance between two active fingers (for pinch).
    fn two_finger_distance(&self) -> f64 {
        let touches: Vec<&TouchPoint> = self.active_touches.values().collect();
        if touches.len() < 2 {
            return 0.0;
        }
        let dx = touches[1].current_x - touches[0].current_x;
        let dy = touches[1].current_y - touches[0].current_y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── Tap ──────────────────────────────────────────────────────────

    #[test]
    fn tap_recognized_on_quick_touch() {
        let mut rec = GestureRecognizer::new();
        rec.on_touch_start(1, 100.0, 200.0);
        rec.update(0.1); // 100ms — within tap_max_duration (300ms)
        rec.on_touch_end(1, 102.0, 201.0); // small movement within tap_max_distance

        let gestures = rec.drain_gestures();
        assert_eq!(gestures.len(), 1);
        match &gestures[0] {
            Gesture::Tap { x, y } => {
                assert!((*x - 102.0).abs() < f64::EPSILON);
                assert!((*y - 201.0).abs() < f64::EPSILON);
            }
            other => panic!("Expected Tap, got {:?}", other),
        }
    }

    #[test]
    fn tap_not_recognized_if_held_too_long() {
        let mut rec = GestureRecognizer::new();
        rec.on_touch_start(1, 100.0, 200.0);
        rec.update(0.35); // 350ms — exceeds tap_max_duration (300ms)
        rec.on_touch_end(1, 100.0, 200.0);

        let gestures = rec.drain_gestures();
        // Should not produce a tap (held too long but not long enough for long-press)
        for g in &gestures {
            assert!(!matches!(g, Gesture::Tap { .. }));
        }
    }

    #[test]
    fn tap_not_recognized_if_moved_too_far() {
        let mut rec = GestureRecognizer::new();
        rec.on_touch_start(1, 100.0, 200.0);
        rec.update(0.1);
        rec.on_touch_end(1, 100.0, 250.0); // 50px — exceeds tap_max_distance (10px)

        let gestures = rec.drain_gestures();
        // Should be a swipe, not a tap
        assert!(!gestures.is_empty());
        for g in &gestures {
            assert!(!matches!(g, Gesture::Tap { .. }));
        }
    }

    // ─── Double Tap ──────────────────────────────────────────────────

    #[test]
    fn double_tap_recognized_within_window() {
        let mut rec = GestureRecognizer::new();

        // First tap
        rec.on_touch_start(1, 100.0, 200.0);
        rec.update(0.05);
        rec.on_touch_end(1, 100.0, 200.0);
        rec.update(0.1); // gap between taps

        // Second tap at roughly the same location
        rec.on_touch_start(2, 102.0, 201.0);
        rec.update(0.05);
        rec.on_touch_end(2, 102.0, 201.0);

        let gestures = rec.drain_gestures();
        // Should contain: Tap (first), DoubleTap (second)
        let double_taps: Vec<_> = gestures.iter().filter(|g| matches!(g, Gesture::DoubleTap { .. })).collect();
        assert_eq!(double_taps.len(), 1);
        match double_taps[0] {
            Gesture::DoubleTap { x, y } => {
                assert!((*x - 102.0).abs() < f64::EPSILON);
                assert!((*y - 201.0).abs() < f64::EPSILON);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn double_tap_not_recognized_outside_window() {
        let mut rec = GestureRecognizer::new();

        // First tap
        rec.on_touch_start(1, 100.0, 200.0);
        rec.update(0.05);
        rec.on_touch_end(1, 100.0, 200.0);
        rec.update(0.5); // 500ms gap — exceeds double_tap_window (400ms)
        // Expire the last_tap
        rec.update(0.0);

        // Second tap
        rec.on_touch_start(2, 100.0, 200.0);
        rec.update(0.05);
        rec.on_touch_end(2, 100.0, 200.0);

        let gestures = rec.drain_gestures();
        let double_taps: Vec<_> = gestures.iter().filter(|g| matches!(g, Gesture::DoubleTap { .. })).collect();
        assert_eq!(double_taps.len(), 0);
    }

    // ─── Long Press ──────────────────────────────────────────────────

    #[test]
    fn long_press_recognized_after_hold_duration() {
        let mut rec = GestureRecognizer::new();
        rec.on_touch_start(1, 100.0, 200.0);

        // Tick past the long-press threshold (0.5s)
        rec.update(0.2);
        assert!(rec.drain_gestures().is_empty());
        rec.update(0.2);
        assert!(rec.drain_gestures().is_empty());
        rec.update(0.15); // total: 0.55s

        let gestures = rec.drain_gestures();
        assert_eq!(gestures.len(), 1);
        match &gestures[0] {
            Gesture::LongPress { x, y } => {
                assert!((*x - 100.0).abs() < f64::EPSILON);
                assert!((*y - 200.0).abs() < f64::EPSILON);
            }
            other => panic!("Expected LongPress, got {:?}", other),
        }
    }

    #[test]
    fn long_press_not_fired_if_finger_moves() {
        let mut rec = GestureRecognizer::new();
        rec.on_touch_start(1, 100.0, 200.0);
        rec.update(0.1);
        rec.on_touch_move(1, 150.0, 250.0); // moved 70+ px
        rec.update(0.5);

        let gestures = rec.drain_gestures();
        for g in &gestures {
            assert!(!matches!(g, Gesture::LongPress { .. }));
        }
    }

    // ─── Swipe ───────────────────────────────────────────────────────

    #[test]
    fn swipe_right_recognized() {
        let mut rec = GestureRecognizer::new();
        rec.on_touch_start(1, 50.0, 100.0);
        rec.update(0.1);
        rec.on_touch_move(1, 80.0, 102.0);
        rec.update(0.05);
        rec.on_touch_end(1, 120.0, 103.0); // 70px right, 3px down

        let gestures = rec.drain_gestures();
        assert_eq!(gestures.len(), 1);
        match &gestures[0] {
            Gesture::Swipe { direction, velocity, start_x, start_y, end_x, end_y } => {
                assert_eq!(*direction, SwipeDirection::Right);
                assert!(*velocity > 0.0);
                assert!((*start_x - 50.0).abs() < f64::EPSILON);
                assert!((*start_y - 100.0).abs() < f64::EPSILON);
                assert!((*end_x - 120.0).abs() < f64::EPSILON);
                assert!((*end_y - 103.0).abs() < f64::EPSILON);
            }
            other => panic!("Expected Swipe, got {:?}", other),
        }
    }

    #[test]
    fn swipe_up_recognized() {
        let mut rec = GestureRecognizer::new();
        rec.on_touch_start(1, 100.0, 200.0);
        rec.update(0.1);
        rec.on_touch_end(1, 103.0, 150.0); // 3px right, 50px up (negative Y)

        let gestures = rec.drain_gestures();
        assert_eq!(gestures.len(), 1);
        match &gestures[0] {
            Gesture::Swipe { direction, .. } => {
                assert_eq!(*direction, SwipeDirection::Up);
            }
            other => panic!("Expected Swipe Up, got {:?}", other),
        }
    }

    // ─── Pinch ──────────────────────────────────────────────────────

    #[test]
    fn pinch_spread_detected() {
        let mut rec = GestureRecognizer::new();

        // Two fingers start close together
        rec.on_touch_start(1, 100.0, 200.0);
        rec.on_touch_start(2, 120.0, 200.0); // 20px apart

        // Move them apart
        rec.on_touch_move(1, 80.0, 200.0);
        rec.on_touch_move(2, 140.0, 200.0); // now 60px apart

        let gestures = rec.drain_gestures();
        let pinches: Vec<_> = gestures.iter().filter(|g| matches!(g, Gesture::Pinch { .. })).collect();
        assert!(!pinches.is_empty());

        // At least one pinch should have scale_delta > 1.0 (spreading)
        let has_spread = pinches.iter().any(|g| {
            if let Gesture::Pinch { scale_delta } = g {
                *scale_delta > 1.0
            } else {
                false
            }
        });
        assert!(has_spread);
    }

    // ─── Primary touch forwarding ───────────────────────────────────

    #[test]
    fn primary_touch_tracked_correctly() {
        let mut rec = GestureRecognizer::new();

        // First touch becomes primary
        let pos = rec.on_touch_start(1, 100.0, 200.0);
        assert_eq!(pos, (100.0, 200.0));
        assert_eq!(rec.primary_touch(), Some(1));

        // Second touch does not change primary
        rec.on_touch_start(2, 300.0, 400.0);
        assert_eq!(rec.primary_touch(), Some(1));

        // Moving primary returns position
        let move_pos = rec.on_touch_move(1, 110.0, 210.0);
        assert_eq!(move_pos, Some((110.0, 210.0)));

        // Moving non-primary returns None
        let move_pos2 = rec.on_touch_move(2, 310.0, 410.0);
        assert_eq!(move_pos2, None);

        // Ending primary clears it
        rec.on_touch_end(1, 110.0, 210.0);
        assert_eq!(rec.primary_touch(), None);
    }

    // ─── drain_gestures ─────────────────────────────────────────────

    #[test]
    fn drain_gestures_empties_queue() {
        let mut rec = GestureRecognizer::new();
        rec.on_touch_start(1, 100.0, 200.0);
        rec.update(0.05);
        rec.on_touch_end(1, 100.0, 200.0);

        let first = rec.drain_gestures();
        assert!(!first.is_empty());

        let second = rec.drain_gestures();
        assert!(second.is_empty());
    }

    // ─── GestureConfig defaults ─────────────────────────────────────

    #[test]
    fn gesture_config_defaults_are_correct() {
        let config = GestureConfig::default();
        assert!((config.tap_max_duration - 0.3).abs() < f64::EPSILON);
        assert!((config.tap_max_distance - 10.0).abs() < f64::EPSILON);
        assert!((config.double_tap_window - 0.4).abs() < f64::EPSILON);
        assert!((config.long_press_duration - 0.5).abs() < f64::EPSILON);
        assert!((config.swipe_min_distance - 30.0).abs() < f64::EPSILON);
    }

    // ─── Active touch count ─────────────────────────────────────────

    #[test]
    fn active_touch_count_tracks_fingers() {
        let mut rec = GestureRecognizer::new();
        assert_eq!(rec.active_touch_count(), 0);

        rec.on_touch_start(1, 0.0, 0.0);
        assert_eq!(rec.active_touch_count(), 1);

        rec.on_touch_start(2, 50.0, 50.0);
        assert_eq!(rec.active_touch_count(), 2);

        rec.on_touch_end(1, 0.0, 0.0);
        assert_eq!(rec.active_touch_count(), 1);

        rec.on_touch_end(2, 50.0, 50.0);
        assert_eq!(rec.active_touch_count(), 0);
    }

    // ─── Clone + Debug ──────────────────────────────────────────────

    #[test]
    fn types_are_clone_and_debug() {
        let rec = GestureRecognizer::new();
        let _cloned = rec.clone();
        let _dbg = format!("{:?}", rec);

        let config = GestureConfig::default();
        let _cloned_config = config.clone();
        let _dbg_config = format!("{:?}", config);

        let tp = TouchPoint {
            id: 1, start_x: 0.0, start_y: 0.0,
            current_x: 0.0, current_y: 0.0, start_time: 0.0,
        };
        let _cloned_tp = tp.clone();
        let _dbg_tp = format!("{:?}", tp);

        let gesture = Gesture::Tap { x: 0.0, y: 0.0 };
        let _cloned_g = gesture.clone();
        let _dbg_g = format!("{:?}", gesture);

        let dir = SwipeDirection::Up;
        let _cloned_d = dir.clone();
        let _dbg_d = format!("{:?}", dir);
    }

    // ─── Swipe velocity calculation ─────────────────────────────────

    #[test]
    fn swipe_velocity_proportional_to_speed() {
        // Fast swipe
        let mut rec_fast = GestureRecognizer::new();
        rec_fast.on_touch_start(1, 0.0, 0.0);
        rec_fast.update(0.1); // 100ms
        rec_fast.on_touch_end(1, 100.0, 0.0); // 100px in 100ms = 1000 px/s

        let fast_gestures = rec_fast.drain_gestures();
        let fast_vel = match &fast_gestures[0] {
            Gesture::Swipe { velocity, .. } => *velocity,
            other => panic!("Expected Swipe, got {:?}", other),
        };

        // Slow swipe: move the finger early so long-press is not triggered,
        // then wait and release. Total elapsed = 0.4s, distance = 100px.
        let mut rec_slow = GestureRecognizer::new();
        rec_slow.on_touch_start(1, 0.0, 0.0);
        rec_slow.update(0.1);
        rec_slow.on_touch_move(1, 40.0, 0.0); // move past tap_max_distance to prevent long-press
        rec_slow.update(0.3); // total: 0.4s
        rec_slow.on_touch_end(1, 100.0, 0.0); // 100px in 400ms = 250 px/s

        let slow_gestures = rec_slow.drain_gestures();
        let slow_vel = match &slow_gestures[0] {
            Gesture::Swipe { velocity, .. } => *velocity,
            other => panic!("Expected Swipe, got {:?}", other),
        };

        assert!(fast_vel > slow_vel);
        // Verify approximate values
        assert!((fast_vel - 1000.0).abs() < 1.0);
        assert!((slow_vel - 250.0).abs() < 1.0);
    }

    // ─── Long-press fires only once ─────────────────────────────────

    #[test]
    fn long_press_fires_only_once() {
        let mut rec = GestureRecognizer::new();
        rec.on_touch_start(1, 100.0, 200.0);

        rec.update(0.6); // past long-press threshold
        let g1 = rec.drain_gestures();
        assert_eq!(g1.len(), 1);
        assert!(matches!(&g1[0], Gesture::LongPress { .. }));

        // Further updates should not produce another long-press
        rec.update(0.5);
        let g2 = rec.drain_gestures();
        assert!(g2.is_empty());
    }

    // ─── No gesture on touch end after long-press ───────────────────

    #[test]
    fn no_tap_after_long_press() {
        let mut rec = GestureRecognizer::new();
        rec.on_touch_start(1, 100.0, 200.0);
        rec.update(0.6);
        rec.drain_gestures(); // consume long-press

        // Lifting the finger should NOT produce a tap
        rec.on_touch_end(1, 100.0, 200.0);
        let gestures = rec.drain_gestures();
        for g in &gestures {
            assert!(!matches!(g, Gesture::Tap { .. }));
        }
    }
}
