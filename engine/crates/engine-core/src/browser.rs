/// Fixed-layout shared memory buffer for browser → WASM data.
/// JS writes via Float64Array view; WASM reads by slot index.
/// Mirrors the framebuffer pattern but flows in reverse.

pub const SLOT_COUNT: usize = 32;

// Slot indices — keep in sync with JS constants
pub const VIEWPORT_W: usize = 0;
pub const VIEWPORT_H: usize = 1;
pub const CANVAS_CSS_W: usize = 2;
pub const CANVAS_CSS_H: usize = 3;
pub const DEVICE_PIXEL_RATIO: usize = 4;
pub const IS_TOUCH_DEVICE: usize = 5;
pub const MAX_TOUCH_POINTS: usize = 6;
pub const ORIENTATION: usize = 7;
pub const DOCUMENT_VISIBLE: usize = 8;
pub const DOCUMENT_FOCUSED: usize = 9;
pub const AUDIO_UNLOCKED: usize = 10;
pub const JS_FPS: usize = 11;
pub const JS_FRAME_TIME_MS: usize = 12;
pub const ONLINE: usize = 13;
pub const WALL_CLOCK_S: usize = 14;
pub const SCROLL_Y: usize = 15;

pub struct BrowserState {
    slots: Vec<f64>,
}

impl BrowserState {
    pub fn new() -> Self {
        Self { slots: vec![0.0; SLOT_COUNT] }
    }

    pub fn get(&self, slot: usize) -> f64 {
        self.slots.get(slot).copied().unwrap_or(0.0)
    }

    pub fn ptr(&self) -> usize {
        self.slots.as_ptr() as usize
    }

    pub fn byte_len(&self) -> usize {
        self.slots.len() * std::mem::size_of::<f64>()
    }

    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }

    // Convenience readers
    pub fn viewport_w(&self) -> f64 { self.get(VIEWPORT_W) }
    pub fn viewport_h(&self) -> f64 { self.get(VIEWPORT_H) }
    pub fn canvas_css_w(&self) -> f64 { self.get(CANVAS_CSS_W) }
    pub fn canvas_css_h(&self) -> f64 { self.get(CANVAS_CSS_H) }
    pub fn device_pixel_ratio(&self) -> f64 { self.get(DEVICE_PIXEL_RATIO) }
    pub fn is_touch_device(&self) -> bool { self.get(IS_TOUCH_DEVICE) > 0.5 }
    pub fn is_visible(&self) -> bool { self.get(DOCUMENT_VISIBLE) > 0.5 }
    pub fn is_focused(&self) -> bool { self.get(DOCUMENT_FOCUSED) > 0.5 }
    pub fn is_online(&self) -> bool { self.get(ONLINE) > 0.5 }
    pub fn audio_unlocked(&self) -> bool { self.get(AUDIO_UNLOCKED) > 0.5 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_zeros_all_slots() {
        let bs = BrowserState::new();
        for i in 0..SLOT_COUNT {
            assert_eq!(bs.get(i), 0.0, "slot {} should be zero", i);
        }
    }

    #[test]
    fn slot_count_matches() {
        let bs = BrowserState::new();
        assert_eq!(bs.slot_count(), SLOT_COUNT);
    }

    #[test]
    fn byte_len_correct() {
        let bs = BrowserState::new();
        assert_eq!(bs.byte_len(), SLOT_COUNT * 8);
    }

    #[test]
    fn out_of_bounds_returns_zero() {
        let bs = BrowserState::new();
        assert_eq!(bs.get(SLOT_COUNT), 0.0);
        assert_eq!(bs.get(999), 0.0);
    }

    #[test]
    fn ptr_is_stable_across_reads() {
        let bs = BrowserState::new();
        let p1 = bs.ptr();
        let _ = bs.get(0);
        let _ = bs.get(SLOT_COUNT - 1);
        let p2 = bs.ptr();
        assert_eq!(p1, p2);
    }

    #[test]
    fn direct_write_then_read() {
        let mut bs = BrowserState::new();
        bs.slots[VIEWPORT_W] = 1920.0;
        bs.slots[VIEWPORT_H] = 1080.0;
        bs.slots[DEVICE_PIXEL_RATIO] = 2.0;
        bs.slots[IS_TOUCH_DEVICE] = 1.0;
        bs.slots[DOCUMENT_VISIBLE] = 1.0;
        bs.slots[ONLINE] = 1.0;

        assert_eq!(bs.viewport_w(), 1920.0);
        assert_eq!(bs.viewport_h(), 1080.0);
        assert_eq!(bs.device_pixel_ratio(), 2.0);
        assert!(bs.is_touch_device());
        assert!(bs.is_visible());
        assert!(bs.is_online());
        assert!(!bs.is_focused());
        assert!(!bs.audio_unlocked());
    }

    #[test]
    fn bool_threshold_at_half() {
        let mut bs = BrowserState::new();
        bs.slots[IS_TOUCH_DEVICE] = 0.5;
        assert!(!bs.is_touch_device(), "exactly 0.5 should be false (> 0.5 required)");
        bs.slots[IS_TOUCH_DEVICE] = 0.51;
        assert!(bs.is_touch_device());
    }
}
