use std::collections::HashSet;

pub struct Input {
    pub keys_held: HashSet<String>,
    pub keys_pressed: HashSet<String>,
    pub keys_released: HashSet<String>,
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub mouse_buttons_held: HashSet<u32>,
    pub mouse_buttons_pressed: HashSet<u32>,
    pub mouse_buttons_released: HashSet<u32>,
    pub drag_start: Option<(f64, f64)>,
    pub is_dragging: bool,
    drag_threshold: f64,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys_held: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_buttons_held: HashSet::new(),
            mouse_buttons_pressed: HashSet::new(),
            mouse_buttons_released: HashSet::new(),
            drag_start: None,
            is_dragging: false,
            drag_threshold: 5.0,
        }
    }

    pub fn on_key_down(&mut self, code: String) {
        if !self.keys_held.contains(&code) {
            self.keys_pressed.insert(code.clone());
        }
        self.keys_held.insert(code);
    }

    pub fn on_key_up(&mut self, code: String) {
        self.keys_held.remove(&code);
        self.keys_released.insert(code);
    }

    pub fn on_mouse_move(&mut self, x: f64, y: f64) {
        self.mouse_x = x;
        self.mouse_y = y;
        if !self.mouse_buttons_held.is_empty() {
            if let Some((sx, sy)) = self.drag_start {
                let dx = x - sx;
                let dy = y - sy;
                if (dx * dx + dy * dy).sqrt() > self.drag_threshold {
                    self.is_dragging = true;
                }
            }
        }
    }

    pub fn on_mouse_down(&mut self, x: f64, y: f64, button: u32) {
        self.mouse_x = x;
        self.mouse_y = y;
        self.mouse_buttons_held.insert(button);
        self.mouse_buttons_pressed.insert(button);
        self.drag_start = Some((x, y));
        self.is_dragging = false;
    }

    pub fn on_mouse_up(&mut self, x: f64, y: f64, button: u32) {
        self.mouse_x = x;
        self.mouse_y = y;
        self.mouse_buttons_held.remove(&button);
        self.mouse_buttons_released.insert(button);
    }

    pub fn end_frame(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_buttons_pressed.clear();
        self.mouse_buttons_released.clear();
        if self.mouse_buttons_held.is_empty() {
            self.drag_start = None;
            self.is_dragging = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── Keyboard: on_key_down ──────────────────────────────────────

    #[test]
    fn on_key_down_adds_to_held_and_pressed() {
        let mut input = Input::new();
        input.on_key_down("KeyA".to_string());
        assert!(input.keys_held.contains("KeyA"));
        assert!(input.keys_pressed.contains("KeyA"));
    }

    #[test]
    fn on_key_down_twice_does_not_double_press() {
        let mut input = Input::new();
        input.on_key_down("KeyA".to_string());
        input.on_key_down("KeyA".to_string());
        assert!(input.keys_held.contains("KeyA"));
        // pressed should contain it once (HashSet deduplicates)
        assert!(input.keys_pressed.contains("KeyA"));
        // But the second call should NOT have re-inserted into pressed
        // because the key was already held. We verify by checking that
        // the first call inserted and the second did not trigger the
        // `if !self.keys_held.contains` branch.
        // The best we can test is that pressed still only has 1 entry for KeyA.
        assert_eq!(input.keys_pressed.len(), 1);
    }

    #[test]
    fn on_key_down_second_call_while_held_does_not_add_to_pressed() {
        let mut input = Input::new();
        input.on_key_down("Space".to_string());
        // Clear pressed to simulate frame boundary
        input.keys_pressed.clear();
        // Second key-down event (auto-repeat)
        input.on_key_down("Space".to_string());
        // Should NOT appear in pressed again because it's already held
        assert!(!input.keys_pressed.contains("Space"));
        assert!(input.keys_held.contains("Space"));
    }

    #[test]
    fn on_key_down_multiple_keys() {
        let mut input = Input::new();
        input.on_key_down("KeyA".to_string());
        input.on_key_down("KeyB".to_string());
        input.on_key_down("KeyC".to_string());
        assert_eq!(input.keys_held.len(), 3);
        assert_eq!(input.keys_pressed.len(), 3);
    }

    // ─── Keyboard: on_key_up ────────────────────────────────────────

    #[test]
    fn on_key_up_removes_from_held_adds_to_released() {
        let mut input = Input::new();
        input.on_key_down("KeyA".to_string());
        input.on_key_up("KeyA".to_string());
        assert!(!input.keys_held.contains("KeyA"));
        assert!(input.keys_released.contains("KeyA"));
    }

    #[test]
    fn on_key_up_without_prior_down() {
        let mut input = Input::new();
        // Should not panic; just adds to released
        input.on_key_up("KeyX".to_string());
        assert!(input.keys_released.contains("KeyX"));
        assert!(!input.keys_held.contains("KeyX"));
    }

    // ─── Keyboard: end_frame ────────────────────────────────────────

    #[test]
    fn end_frame_clears_pressed_and_released() {
        let mut input = Input::new();
        input.on_key_down("KeyA".to_string());
        input.on_key_up("KeyB".to_string());
        input.end_frame();
        assert!(input.keys_pressed.is_empty());
        assert!(input.keys_released.is_empty());
    }

    #[test]
    fn end_frame_preserves_held_keys() {
        let mut input = Input::new();
        input.on_key_down("KeyA".to_string());
        input.end_frame();
        assert!(input.keys_held.contains("KeyA"));
    }

    // ─── Mouse: on_mouse_down ───────────────────────────────────────

    #[test]
    fn on_mouse_down_sets_drag_start() {
        let mut input = Input::new();
        input.on_mouse_down(100.0, 200.0, 0);
        assert_eq!(input.drag_start, Some((100.0, 200.0)));
    }

    #[test]
    fn on_mouse_down_adds_to_held_and_pressed() {
        let mut input = Input::new();
        input.on_mouse_down(0.0, 0.0, 0);
        assert!(input.mouse_buttons_held.contains(&0));
        assert!(input.mouse_buttons_pressed.contains(&0));
    }

    #[test]
    fn on_mouse_down_resets_is_dragging() {
        let mut input = Input::new();
        input.is_dragging = true;
        input.on_mouse_down(10.0, 10.0, 0);
        assert!(!input.is_dragging);
    }

    #[test]
    fn on_mouse_down_updates_position() {
        let mut input = Input::new();
        input.on_mouse_down(50.0, 75.0, 0);
        assert_eq!(input.mouse_x, 50.0);
        assert_eq!(input.mouse_y, 75.0);
    }

    // ─── Mouse: dragging with threshold ─────────────────────────────

    #[test]
    fn mouse_move_with_button_held_triggers_dragging_after_threshold() {
        let mut input = Input::new();
        input.on_mouse_down(100.0, 100.0, 0);
        // Move beyond the 5.0 threshold
        input.on_mouse_move(110.0, 100.0); // dx=10, well above 5.0
        assert!(input.is_dragging);
    }

    #[test]
    fn mouse_move_below_threshold_does_not_trigger_dragging() {
        let mut input = Input::new();
        input.on_mouse_down(100.0, 100.0, 0);
        // Move less than 5.0 distance
        input.on_mouse_move(102.0, 101.0); // distance ~ 2.24
        assert!(!input.is_dragging);
    }

    #[test]
    fn mouse_move_exactly_at_threshold_does_not_trigger_dragging() {
        let mut input = Input::new();
        input.on_mouse_down(100.0, 100.0, 0);
        // Move exactly 5.0 (3-4-5 triangle: dx=3, dy=4 => dist=5)
        input.on_mouse_move(103.0, 104.0);
        // threshold check is strictly greater than 5.0, so exactly 5.0 should not trigger
        assert!(!input.is_dragging);
    }

    #[test]
    fn mouse_move_without_button_held_does_not_trigger_dragging() {
        let mut input = Input::new();
        // Set drag_start manually but no buttons held
        input.drag_start = Some((100.0, 100.0));
        input.on_mouse_move(200.0, 200.0);
        assert!(!input.is_dragging);
    }

    #[test]
    fn mouse_move_updates_position() {
        let mut input = Input::new();
        input.on_mouse_move(42.0, 84.0);
        assert_eq!(input.mouse_x, 42.0);
        assert_eq!(input.mouse_y, 84.0);
    }

    // ─── Mouse: on_mouse_up ─────────────────────────────────────────

    #[test]
    fn on_mouse_up_removes_from_held() {
        let mut input = Input::new();
        input.on_mouse_down(0.0, 0.0, 0);
        input.on_mouse_up(0.0, 0.0, 0);
        assert!(!input.mouse_buttons_held.contains(&0));
    }

    #[test]
    fn on_mouse_up_adds_to_released() {
        let mut input = Input::new();
        input.on_mouse_down(0.0, 0.0, 2);
        input.on_mouse_up(0.0, 0.0, 2);
        assert!(input.mouse_buttons_released.contains(&2));
    }

    #[test]
    fn on_mouse_up_updates_position() {
        let mut input = Input::new();
        input.on_mouse_down(0.0, 0.0, 0);
        input.on_mouse_up(55.0, 66.0, 0);
        assert_eq!(input.mouse_x, 55.0);
        assert_eq!(input.mouse_y, 66.0);
    }

    // ─── Mouse: end_frame drag state ────────────────────────────────

    #[test]
    fn end_frame_with_no_buttons_held_clears_drag_state() {
        let mut input = Input::new();
        input.on_mouse_down(10.0, 10.0, 0);
        input.on_mouse_move(100.0, 100.0); // triggers dragging
        input.on_mouse_up(100.0, 100.0, 0);
        input.end_frame();
        assert_eq!(input.drag_start, None);
        assert!(!input.is_dragging);
    }

    #[test]
    fn end_frame_with_button_still_held_preserves_drag_state() {
        let mut input = Input::new();
        input.on_mouse_down(10.0, 10.0, 0);
        input.on_mouse_move(100.0, 100.0); // triggers dragging
        // Button 0 is still held
        input.end_frame();
        assert!(input.drag_start.is_some());
        assert!(input.is_dragging);
    }

    #[test]
    fn end_frame_clears_mouse_pressed_and_released() {
        let mut input = Input::new();
        input.on_mouse_down(0.0, 0.0, 0);
        input.on_mouse_up(0.0, 0.0, 0);
        input.end_frame();
        assert!(input.mouse_buttons_pressed.is_empty());
        assert!(input.mouse_buttons_released.is_empty());
    }

    // ─── Full frame cycle ───────────────────────────────────────────

    #[test]
    fn full_frame_cycle() {
        let mut input = Input::new();

        // Frame 1: press A
        input.on_key_down("KeyA".to_string());
        assert!(input.keys_pressed.contains("KeyA"));
        assert!(input.keys_held.contains("KeyA"));

        input.end_frame();
        assert!(!input.keys_pressed.contains("KeyA"));
        assert!(input.keys_held.contains("KeyA"));

        // Frame 2: A still held (auto-repeat), press B
        input.on_key_down("KeyA".to_string()); // repeat
        input.on_key_down("KeyB".to_string());
        assert!(!input.keys_pressed.contains("KeyA")); // already held
        assert!(input.keys_pressed.contains("KeyB"));

        input.end_frame();

        // Frame 3: release A
        input.on_key_up("KeyA".to_string());
        assert!(!input.keys_held.contains("KeyA"));
        assert!(input.keys_released.contains("KeyA"));
        assert!(input.keys_held.contains("KeyB")); // B still held

        input.end_frame();
        assert!(!input.keys_released.contains("KeyA"));
    }
}
