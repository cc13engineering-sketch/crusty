/// UiCanvas — Declarative UI Widget System
///
/// Provides anchor-based positioning, data-bound widgets (labels, bars, buttons),
/// and hit-testing for interactive UI overlays rendered directly to the framebuffer.

use crate::rendering::color::Color;
use crate::rendering::framebuffer::Framebuffer;
use crate::rendering::text;
use crate::rendering::shapes;
use crate::game_state::GameState;

// ─── Anchor ─────────────────────────────────────────────────────────────

/// Screen anchor point for widget positioning.
#[derive(Clone, Debug, PartialEq)]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

/// Resolve an anchor + pixel offset to an absolute screen position.
///
/// Returns the top-left corner of a widget placed at the anchor with the given offset.
/// `screen_w` and `screen_h` are the framebuffer dimensions.
pub fn resolve_position(
    anchor: &Anchor,
    offset: (f64, f64),
    screen_w: f64,
    screen_h: f64,
) -> (i32, i32) {
    let (ax, ay) = match anchor {
        Anchor::TopLeft      => (0.0,              0.0),
        Anchor::TopCenter    => (screen_w / 2.0,   0.0),
        Anchor::TopRight     => (screen_w,         0.0),
        Anchor::CenterLeft   => (0.0,              screen_h / 2.0),
        Anchor::Center       => (screen_w / 2.0,   screen_h / 2.0),
        Anchor::CenterRight  => (screen_w,         screen_h / 2.0),
        Anchor::BottomLeft   => (0.0,              screen_h),
        Anchor::BottomCenter => (screen_w / 2.0,   screen_h),
        Anchor::BottomRight  => (screen_w,         screen_h),
    };
    ((ax + offset.0).round() as i32, (ay + offset.1).round() as i32)
}

// ─── Value binding ──────────────────────────────────────────────────────

/// How a widget value is sourced.
#[derive(Clone, Debug, PartialEq)]
pub enum ValueBinding {
    /// Read a numeric value from the global GameState by key.
    GameState(String),
    /// A fixed numeric literal.
    Fixed(f64),
    /// A fixed text string.
    Text(String),
}

impl ValueBinding {
    /// Resolve this binding to a f64, falling back to `fallback` if the key is missing.
    pub fn resolve_f64(&self, state: &GameState, fallback: f64) -> f64 {
        match self {
            ValueBinding::Fixed(v) => *v,
            ValueBinding::GameState(key) => state.get_f64(key).unwrap_or(fallback),
            ValueBinding::Text(_) => fallback,
        }
    }

    /// Resolve this binding to a display string.
    pub fn resolve_text(&self, state: &GameState) -> String {
        match self {
            ValueBinding::Text(s) => s.clone(),
            ValueBinding::Fixed(v) => {
                // Display as integer if whole, otherwise with 1 decimal
                if (*v - v.round()).abs() < 1e-9 {
                    format!("{}", *v as i64)
                } else {
                    format!("{:.1}", v)
                }
            }
            ValueBinding::GameState(key) => {
                if let Some(v) = state.get_f64(key) {
                    if (v - v.round()).abs() < 1e-9 {
                        format!("{}", v as i64)
                    } else {
                        format!("{:.1}", v)
                    }
                } else if let Some(s) = state.get_str(key) {
                    s.to_string()
                } else {
                    String::new()
                }
            }
        }
    }
}

// ─── Widget kinds ───────────────────────────────────────────────────────

/// The visual type and configuration of a UI widget.
#[derive(Clone, Debug)]
pub enum WidgetKind {
    /// Text label.
    Label {
        text: ValueBinding,
        scale: u32,
        color: Color,
    },
    /// Horizontal bar (health, mana, XP, etc.).
    Bar {
        value: ValueBinding,
        max: ValueBinding,
        fill_color: Color,
        bg_color: Color,
        width: f64,
        height: f64,
    },
    /// Clickable button.
    Button {
        label: String,
        action: String,
        width: f64,
        height: f64,
        color: Color,
        text_color: Color,
    },
}

// ─── UiWidget ───────────────────────────────────────────────────────────

/// A single positioned widget in the UI canvas.
#[derive(Clone, Debug)]
pub struct UiWidget {
    pub id: String,
    pub kind: WidgetKind,
    pub anchor: Anchor,
    pub offset: (f64, f64),
    pub visible: bool,
}

impl UiWidget {
    /// Create a new widget.
    pub fn new(id: &str, kind: WidgetKind, anchor: Anchor, offset: (f64, f64)) -> Self {
        Self {
            id: id.to_string(),
            kind,
            anchor,
            offset,
            visible: true,
        }
    }

    /// Get the bounding rect (x, y, w, h) at the resolved screen position.
    fn bounds(&self, screen_w: f64, screen_h: f64) -> (i32, i32, f64, f64) {
        let (x, y) = resolve_position(&self.anchor, self.offset, screen_w, screen_h);
        let (w, h) = match &self.kind {
            WidgetKind::Label { text, scale, .. } => {
                // Approximate width from text — use fixed text for sizing
                let display = match text {
                    ValueBinding::Text(s) => s.clone(),
                    ValueBinding::Fixed(v) => format!("{}", *v as i64),
                    ValueBinding::GameState(k) => k.clone(), // placeholder width
                };
                let tw = crate::rendering::text::text_width(&display, *scale) as f64;
                let th = 7.0 * (*scale).max(1) as f64; // CHAR_H = 7
                (tw, th)
            }
            WidgetKind::Bar { width, height, .. } => (*width, *height),
            WidgetKind::Button { width, height, .. } => (*width, *height),
        };
        (x, y, w, h)
    }
}

// ─── UiCanvas ───────────────────────────────────────────────────────────

/// Collection of UI widgets rendered as an overlay.
#[derive(Clone, Debug, Default)]
pub struct UiCanvas {
    pub widgets: Vec<UiWidget>,
}

impl UiCanvas {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a widget to the canvas.
    pub fn add(&mut self, widget: UiWidget) {
        self.widgets.push(widget);
    }

    /// Remove a widget by id. Returns true if found and removed.
    pub fn remove(&mut self, id: &str) -> bool {
        let before = self.widgets.len();
        self.widgets.retain(|w| w.id != id);
        self.widgets.len() < before
    }

    /// Find a widget by id.
    pub fn get(&self, id: &str) -> Option<&UiWidget> {
        self.widgets.iter().find(|w| w.id == id)
    }

    /// Find a mutable widget by id.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut UiWidget> {
        self.widgets.iter_mut().find(|w| w.id == id)
    }

    /// Number of widgets.
    pub fn len(&self) -> usize {
        self.widgets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.widgets.is_empty()
    }

    /// Clear all widgets.
    pub fn clear(&mut self) {
        self.widgets.clear();
    }

    /// Render all visible widgets onto the framebuffer.
    pub fn render(
        &self,
        fb: &mut Framebuffer,
        global_state: &GameState,
        width: f64,
        height: f64,
    ) {
        for widget in &self.widgets {
            if !widget.visible {
                continue;
            }
            let (x, y) = resolve_position(&widget.anchor, widget.offset, width, height);
            match &widget.kind {
                WidgetKind::Label { text: binding, scale, color } => {
                    let display = binding.resolve_text(global_state);
                    text::draw_text(fb, x, y, &display, *color, *scale);
                }
                WidgetKind::Bar { value, max, fill_color, bg_color, width: bw, height: bh } => {
                    let val = value.resolve_f64(global_state, 0.0);
                    let max_val = max.resolve_f64(global_state, 100.0);
                    let fraction = if max_val > 0.0 { (val / max_val).max(0.0).min(1.0) } else { 0.0 };

                    // Background
                    shapes::fill_rect(fb, x as f64, y as f64, *bw, *bh, *bg_color);
                    // Fill
                    let fill_w = *bw * fraction;
                    if fill_w > 0.0 {
                        shapes::fill_rect(fb, x as f64, y as f64, fill_w, *bh, *fill_color);
                    }
                }
                WidgetKind::Button { label, width: bw, height: bh, color, text_color, .. } => {
                    // Background rect
                    shapes::fill_rect(fb, x as f64, y as f64, *bw, *bh, *color);
                    // Border
                    shapes::draw_rect(fb, x as f64, y as f64, *bw, *bh, *text_color);
                    // Centered label text
                    let tw = text::text_width(label, 1);
                    let text_x = x + (*bw as i32 - tw) / 2;
                    let text_y = y + (*bh as i32 - 7) / 2; // CHAR_H=7
                    text::draw_text(fb, text_x, text_y, label, *text_color, 1);
                }
            }
        }
    }

    /// Hit-test a screen coordinate against all visible button widgets.
    /// Returns the `action` string of the first button hit, or None.
    pub fn hit_test(&self, x: f64, y: f64, width: f64, height: f64) -> Option<String> {
        for widget in &self.widgets {
            if !widget.visible {
                continue;
            }
            if let WidgetKind::Button { action, width: bw, height: bh, .. } = &widget.kind {
                let (wx, wy) = resolve_position(&widget.anchor, widget.offset, width, height);
                let wx = wx as f64;
                let wy = wy as f64;
                if x >= wx && x < wx + bw && y >= wy && y < wy + bh {
                    return Some(action.clone());
                }
            }
        }
        None
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_position_top_left() {
        let (x, y) = resolve_position(&Anchor::TopLeft, (10.0, 20.0), 800.0, 600.0);
        assert_eq!((x, y), (10, 20));
    }

    #[test]
    fn resolve_position_center() {
        let (x, y) = resolve_position(&Anchor::Center, (0.0, 0.0), 800.0, 600.0);
        assert_eq!((x, y), (400, 300));
    }

    #[test]
    fn resolve_position_bottom_right() {
        let (x, y) = resolve_position(&Anchor::BottomRight, (-10.0, -10.0), 800.0, 600.0);
        assert_eq!((x, y), (790, 590));
    }

    #[test]
    fn resolve_position_all_anchors() {
        let w = 100.0;
        let h = 100.0;
        let off = (0.0, 0.0);

        assert_eq!(resolve_position(&Anchor::TopLeft, off, w, h), (0, 0));
        assert_eq!(resolve_position(&Anchor::TopCenter, off, w, h), (50, 0));
        assert_eq!(resolve_position(&Anchor::TopRight, off, w, h), (100, 0));
        assert_eq!(resolve_position(&Anchor::CenterLeft, off, w, h), (0, 50));
        assert_eq!(resolve_position(&Anchor::Center, off, w, h), (50, 50));
        assert_eq!(resolve_position(&Anchor::CenterRight, off, w, h), (100, 50));
        assert_eq!(resolve_position(&Anchor::BottomLeft, off, w, h), (0, 100));
        assert_eq!(resolve_position(&Anchor::BottomCenter, off, w, h), (50, 100));
        assert_eq!(resolve_position(&Anchor::BottomRight, off, w, h), (100, 100));
    }

    #[test]
    fn value_binding_fixed_f64() {
        let state = GameState::new();
        let binding = ValueBinding::Fixed(42.0);
        assert!((binding.resolve_f64(&state, 0.0) - 42.0).abs() < 1e-10);
    }

    #[test]
    fn value_binding_game_state_lookup() {
        let mut state = GameState::new();
        state.set_f64("health", 75.0);
        let binding = ValueBinding::GameState("health".into());
        assert!((binding.resolve_f64(&state, 0.0) - 75.0).abs() < 1e-10);
    }

    #[test]
    fn value_binding_game_state_missing_uses_fallback() {
        let state = GameState::new();
        let binding = ValueBinding::GameState("missing".into());
        assert!((binding.resolve_f64(&state, 99.0) - 99.0).abs() < 1e-10);
    }

    #[test]
    fn value_binding_text_resolve() {
        let state = GameState::new();
        let binding = ValueBinding::Text("Hello".into());
        assert_eq!(binding.resolve_text(&state), "Hello");
    }

    #[test]
    fn value_binding_fixed_resolve_text() {
        let state = GameState::new();
        // Whole number → display as integer
        let binding = ValueBinding::Fixed(100.0);
        assert_eq!(binding.resolve_text(&state), "100");
        // Fractional → display with 1 decimal
        let binding2 = ValueBinding::Fixed(3.5);
        assert_eq!(binding2.resolve_text(&state), "3.5");
    }

    #[test]
    fn canvas_add_and_get() {
        let mut canvas = UiCanvas::new();
        canvas.add(UiWidget::new(
            "hp_label",
            WidgetKind::Label {
                text: ValueBinding::Text("HP".into()),
                scale: 1,
                color: Color::WHITE,
            },
            Anchor::TopLeft,
            (10.0, 10.0),
        ));
        assert_eq!(canvas.len(), 1);
        assert!(canvas.get("hp_label").is_some());
        assert!(canvas.get("missing").is_none());
    }

    #[test]
    fn canvas_remove() {
        let mut canvas = UiCanvas::new();
        canvas.add(UiWidget::new(
            "w1",
            WidgetKind::Label {
                text: ValueBinding::Text("A".into()),
                scale: 1,
                color: Color::WHITE,
            },
            Anchor::TopLeft,
            (0.0, 0.0),
        ));
        canvas.add(UiWidget::new(
            "w2",
            WidgetKind::Label {
                text: ValueBinding::Text("B".into()),
                scale: 1,
                color: Color::WHITE,
            },
            Anchor::TopRight,
            (0.0, 0.0),
        ));
        assert!(canvas.remove("w1"));
        assert_eq!(canvas.len(), 1);
        assert!(canvas.get("w1").is_none());
        assert!(canvas.get("w2").is_some());
        // Removing nonexistent returns false
        assert!(!canvas.remove("w1"));
    }

    #[test]
    fn hit_test_button() {
        let mut canvas = UiCanvas::new();
        canvas.add(UiWidget::new(
            "start_btn",
            WidgetKind::Button {
                label: "Start".into(),
                action: "start_game".into(),
                width: 80.0,
                height: 30.0,
                color: Color::from_rgba(50, 50, 50, 255),
                text_color: Color::WHITE,
            },
            Anchor::TopLeft,
            (100.0, 100.0),
        ));

        // Inside button
        let result = canvas.hit_test(120.0, 110.0, 800.0, 600.0);
        assert_eq!(result, Some("start_game".into()));

        // Outside button
        let result = canvas.hit_test(50.0, 50.0, 800.0, 600.0);
        assert!(result.is_none());

        // Just at the edge (right boundary)
        let result = canvas.hit_test(180.0, 110.0, 800.0, 600.0);
        assert!(result.is_none()); // x=180 is at 100+80 = boundary, not included

        // Just inside the right edge
        let result = canvas.hit_test(179.0, 110.0, 800.0, 600.0);
        assert_eq!(result, Some("start_game".into()));
    }

    #[test]
    fn hit_test_invisible_button_ignored() {
        let mut canvas = UiCanvas::new();
        let mut widget = UiWidget::new(
            "hidden_btn",
            WidgetKind::Button {
                label: "Hidden".into(),
                action: "secret".into(),
                width: 100.0,
                height: 40.0,
                color: Color::RED,
                text_color: Color::WHITE,
            },
            Anchor::TopLeft,
            (0.0, 0.0),
        );
        widget.visible = false;
        canvas.add(widget);

        let result = canvas.hit_test(50.0, 20.0, 800.0, 600.0);
        assert!(result.is_none());
    }

    #[test]
    fn render_does_not_panic() {
        let mut canvas = UiCanvas::new();
        let state = GameState::new();
        let mut fb = Framebuffer::new(100, 100);

        // Add one of each widget kind
        canvas.add(UiWidget::new(
            "label",
            WidgetKind::Label {
                text: ValueBinding::Text("Score: 0".into()),
                scale: 1,
                color: Color::WHITE,
            },
            Anchor::TopLeft,
            (5.0, 5.0),
        ));
        canvas.add(UiWidget::new(
            "bar",
            WidgetKind::Bar {
                value: ValueBinding::Fixed(50.0),
                max: ValueBinding::Fixed(100.0),
                fill_color: Color::GREEN,
                bg_color: Color::from_rgba(40, 40, 40, 200),
                width: 60.0,
                height: 8.0,
            },
            Anchor::BottomLeft,
            (5.0, -15.0),
        ));
        canvas.add(UiWidget::new(
            "btn",
            WidgetKind::Button {
                label: "OK".into(),
                action: "confirm".into(),
                width: 40.0,
                height: 20.0,
                color: Color::from_rgba(80, 80, 80, 255),
                text_color: Color::WHITE,
            },
            Anchor::Center,
            (-20.0, -10.0),
        ));

        // Invisible widget should be skipped
        let mut hidden = UiWidget::new(
            "hidden",
            WidgetKind::Label {
                text: ValueBinding::Text("HIDDEN".into()),
                scale: 2,
                color: Color::RED,
            },
            Anchor::Center,
            (0.0, 0.0),
        );
        hidden.visible = false;
        canvas.add(hidden);

        // This should not panic
        canvas.render(&mut fb, &state, 100.0, 100.0);
    }

    #[test]
    fn bar_renders_correct_fill_fraction() {
        let mut canvas = UiCanvas::new();
        let mut state = GameState::new();
        state.set_f64("hp", 25.0);
        state.set_f64("max_hp", 100.0);

        canvas.add(UiWidget::new(
            "hp_bar",
            WidgetKind::Bar {
                value: ValueBinding::GameState("hp".into()),
                max: ValueBinding::GameState("max_hp".into()),
                fill_color: Color::GREEN,
                bg_color: Color::from_rgba(40, 40, 40, 255),
                width: 100.0,
                height: 10.0,
            },
            Anchor::TopLeft,
            (0.0, 0.0),
        ));

        let mut fb = Framebuffer::new(200, 100);
        fb.clear(Color::BLACK);
        canvas.render(&mut fb, &state, 200.0, 100.0);

        // The bar is 100px wide at TopLeft(0,0). Fill should be 25% = 25px.
        // Check pixel at x=10 (should be green fill)
        let idx_filled = (0 * 200 + 10) * 4; // y=0, x=10
        assert_eq!(fb.pixels[idx_filled as usize + 1], 255, "Green channel in filled area");

        // Check pixel at x=50 (should be bg gray, not green)
        let idx_bg = (0 * 200 + 50) * 4;
        assert_eq!(fb.pixels[idx_bg as usize], 40, "R channel in bg area");
        assert_eq!(fb.pixels[idx_bg as usize + 1], 40, "G channel in bg area");
    }

    #[test]
    fn canvas_clear() {
        let mut canvas = UiCanvas::new();
        canvas.add(UiWidget::new(
            "a",
            WidgetKind::Label {
                text: ValueBinding::Text("X".into()),
                scale: 1,
                color: Color::WHITE,
            },
            Anchor::TopLeft,
            (0.0, 0.0),
        ));
        canvas.clear();
        assert!(canvas.is_empty());
    }

    #[test]
    fn widget_visibility_toggle() {
        let mut canvas = UiCanvas::new();
        canvas.add(UiWidget::new(
            "toggle_me",
            WidgetKind::Label {
                text: ValueBinding::Text("Visible".into()),
                scale: 1,
                color: Color::WHITE,
            },
            Anchor::TopLeft,
            (0.0, 0.0),
        ));
        assert!(canvas.get("toggle_me").map_or(false, |w| w.visible));
        if let Some(w) = canvas.get_mut("toggle_me") {
            w.visible = false;
        }
        assert!(!canvas.get("toggle_me").map_or(true, |w| w.visible));
    }

    #[test]
    fn value_binding_game_state_str_resolve() {
        let mut state = GameState::new();
        state.set_str("player_name", "Hero");
        let binding = ValueBinding::GameState("player_name".into());
        assert_eq!(binding.resolve_text(&state), "Hero");
    }
}
