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
