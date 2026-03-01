use super::color::Color;

/// RGBA pixel buffer. Rust writes pixels here; JS reads via shared WASM memory.
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>, // length = width * height * 4 (RGBA)
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let len = (width as usize) * (height as usize) * 4;
        let mut pixels = vec![0u8; len];
        // Set all alpha bytes to 255 (opaque black, not transparent)
        for i in (3..len).step_by(4) {
            pixels[i] = 255;
        }
        Self { width, height, pixels }
    }

    pub fn clear(&mut self, color: Color) {
        let pattern = [color.r, color.g, color.b, color.a];
        for chunk in self.pixels.chunks_exact_mut(4) {
            chunk.copy_from_slice(&pattern);
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        let idx = ((y as u32 * self.width + x as u32) * 4) as usize;
        self.pixels[idx] = color.r;
        self.pixels[idx + 1] = color.g;
        self.pixels[idx + 2] = color.b;
        self.pixels[idx + 3] = color.a;
    }

    pub fn set_pixel_blended(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        if color.a == 0 {
            return;
        }
        if color.a == 255 {
            self.set_pixel(x, y, color);
            return;
        }
        let idx = ((y as u32 * self.width + x as u32) * 4) as usize;
        let sa = color.a as u16;
        let da = 255 - sa;
        self.pixels[idx] = ((color.r as u16 * sa + self.pixels[idx] as u16 * da) / 255) as u8;
        self.pixels[idx + 1] =
            ((color.g as u16 * sa + self.pixels[idx + 1] as u16 * da) / 255) as u8;
        self.pixels[idx + 2] =
            ((color.b as u16 * sa + self.pixels[idx + 2] as u16 * da) / 255) as u8;
        self.pixels[idx + 3] = 255; // result is always opaque
    }

    /// Copy a rectangular region of raw RGBA pixels onto the framebuffer.
    /// Foundation for all sprite/tilemap rendering.
    pub fn blit_rect(
        &mut self,
        src: &[u8],
        src_width: u32,
        sx: u32,
        sy: u32,
        sw: u32,
        sh: u32,
        dx: i32,
        dy: i32,
        use_alpha: bool,
    ) {
        for row in 0..sh {
            let dst_y = dy + row as i32;
            if dst_y < 0 || dst_y >= self.height as i32 {
                continue;
            }
            for col in 0..sw {
                let dst_x = dx + col as i32;
                if dst_x < 0 || dst_x >= self.width as i32 {
                    continue;
                }
                let src_idx = (((sy + row) * src_width + (sx + col)) * 4) as usize;
                if src_idx + 3 >= src.len() {
                    continue;
                }
                let color = Color::from_rgba(
                    src[src_idx],
                    src[src_idx + 1],
                    src[src_idx + 2],
                    src[src_idx + 3],
                );
                if use_alpha {
                    if color.a == 0 {
                        continue;
                    }
                    self.set_pixel_blended(dst_x, dst_y, color);
                } else {
                    self.set_pixel(dst_x, dst_y, color);
                }
            }
        }
    }

    pub fn ptr(&self) -> usize {
        self.pixels.as_ptr() as usize
    }

    pub fn len(&self) -> usize {
        self.pixels.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- new ---

    #[test]
    fn new_creates_correct_size_buffer() {
        let fb = Framebuffer::new(10, 20);
        assert_eq!(fb.pixels.len(), 10 * 20 * 4);
        assert_eq!(fb.width, 10);
        assert_eq!(fb.height, 20);
    }

    #[test]
    fn new_creates_correct_size_1x1() {
        let fb = Framebuffer::new(1, 1);
        assert_eq!(fb.pixels.len(), 4);
    }

    #[test]
    fn new_initializes_alpha_to_255() {
        let fb = Framebuffer::new(4, 4);
        for i in (3..fb.pixels.len()).step_by(4) {
            assert_eq!(fb.pixels[i], 255, "alpha byte at index {} should be 255", i);
        }
    }

    #[test]
    fn new_initializes_rgb_to_zero() {
        let fb = Framebuffer::new(3, 3);
        for pixel in fb.pixels.chunks_exact(4) {
            assert_eq!(pixel[0], 0, "r should be 0");
            assert_eq!(pixel[1], 0, "g should be 0");
            assert_eq!(pixel[2], 0, "b should be 0");
        }
    }

    // --- clear ---

    #[test]
    fn clear_fills_all_pixels() {
        let mut fb = Framebuffer::new(4, 4);
        let color = Color::from_rgba(10, 20, 30, 40);
        fb.clear(color);
        for pixel in fb.pixels.chunks_exact(4) {
            assert_eq!(pixel[0], 10);
            assert_eq!(pixel[1], 20);
            assert_eq!(pixel[2], 30);
            assert_eq!(pixel[3], 40);
        }
    }

    #[test]
    fn clear_with_white() {
        let mut fb = Framebuffer::new(2, 2);
        fb.clear(Color::WHITE);
        for pixel in fb.pixels.chunks_exact(4) {
            assert_eq!(pixel, [255, 255, 255, 255]);
        }
    }

    #[test]
    fn clear_overwrites_previous_data() {
        let mut fb = Framebuffer::new(2, 2);
        fb.clear(Color::RED);
        fb.clear(Color::BLUE);
        for pixel in fb.pixels.chunks_exact(4) {
            assert_eq!(pixel, [0, 0, 255, 255]);
        }
    }

    // --- set_pixel ---

    #[test]
    fn set_pixel_writes_correct_rgba() {
        let mut fb = Framebuffer::new(4, 4);
        let color = Color::from_rgba(100, 150, 200, 250);
        fb.set_pixel(2, 3, color);
        let idx = (3 * 4 + 2) * 4; // y=3, x=2, width=4
        assert_eq!(fb.pixels[idx], 100);
        assert_eq!(fb.pixels[idx + 1], 150);
        assert_eq!(fb.pixels[idx + 2], 200);
        assert_eq!(fb.pixels[idx + 3], 250);
    }

    #[test]
    fn set_pixel_at_origin() {
        let mut fb = Framebuffer::new(4, 4);
        fb.set_pixel(0, 0, Color::RED);
        assert_eq!(fb.pixels[0], 255);
        assert_eq!(fb.pixels[1], 0);
        assert_eq!(fb.pixels[2], 0);
        assert_eq!(fb.pixels[3], 255);
    }

    #[test]
    fn set_pixel_at_last_position() {
        let mut fb = Framebuffer::new(4, 4);
        fb.set_pixel(3, 3, Color::GREEN);
        let idx = (3 * 4 + 3) * 4;
        assert_eq!(fb.pixels[idx], 0);
        assert_eq!(fb.pixels[idx + 1], 255);
        assert_eq!(fb.pixels[idx + 2], 0);
        assert_eq!(fb.pixels[idx + 3], 255);
    }

    // --- set_pixel out of bounds ---

    #[test]
    fn set_pixel_out_of_bounds_positive_x() {
        let mut fb = Framebuffer::new(4, 4);
        fb.clear(Color::BLACK);
        fb.set_pixel(4, 0, Color::RED); // x == width, out of bounds
        // buffer should be unchanged (all black)
        for pixel in fb.pixels.chunks_exact(4) {
            assert_eq!(pixel, [0, 0, 0, 255]);
        }
    }

    #[test]
    fn set_pixel_out_of_bounds_positive_y() {
        let mut fb = Framebuffer::new(4, 4);
        fb.clear(Color::BLACK);
        fb.set_pixel(0, 4, Color::RED); // y == height, out of bounds
        for pixel in fb.pixels.chunks_exact(4) {
            assert_eq!(pixel, [0, 0, 0, 255]);
        }
    }

    #[test]
    fn set_pixel_negative_x_no_panic() {
        let mut fb = Framebuffer::new(4, 4);
        fb.set_pixel(-1, 0, Color::RED); // should silently do nothing
    }

    #[test]
    fn set_pixel_negative_y_no_panic() {
        let mut fb = Framebuffer::new(4, 4);
        fb.set_pixel(0, -1, Color::RED); // should silently do nothing
    }

    #[test]
    fn set_pixel_both_negative_no_panic() {
        let mut fb = Framebuffer::new(4, 4);
        fb.set_pixel(-100, -100, Color::RED);
    }

    #[test]
    fn set_pixel_large_out_of_bounds_no_panic() {
        let mut fb = Framebuffer::new(4, 4);
        fb.set_pixel(i32::MAX, i32::MAX, Color::RED);
    }

    // --- set_pixel_blended ---

    #[test]
    fn set_pixel_blended_alpha_255_acts_like_set_pixel() {
        let mut fb1 = Framebuffer::new(4, 4);
        let mut fb2 = Framebuffer::new(4, 4);
        fb1.clear(Color::from_rgba(50, 60, 70, 255));
        fb2.clear(Color::from_rgba(50, 60, 70, 255));

        let color = Color::from_rgba(200, 100, 50, 255);
        fb1.set_pixel(1, 1, color);
        fb2.set_pixel_blended(1, 1, color);

        assert_eq!(fb1.pixels, fb2.pixels);
    }

    #[test]
    fn set_pixel_blended_alpha_0_does_nothing() {
        let mut fb = Framebuffer::new(4, 4);
        fb.clear(Color::from_rgba(50, 60, 70, 255));
        let original = fb.pixels.clone();

        fb.set_pixel_blended(1, 1, Color::from_rgba(200, 100, 50, 0));
        assert_eq!(fb.pixels, original);
    }

    #[test]
    fn set_pixel_blended_alpha_128_blends_approximately() {
        let mut fb = Framebuffer::new(4, 4);
        fb.clear(Color::BLACK); // background (0, 0, 0, 255)

        // Blend white at ~50% alpha
        fb.set_pixel_blended(0, 0, Color::from_rgba(255, 255, 255, 128));

        // Expected: (255 * 128 + 0 * 127) / 255 ~= 128
        let r = fb.pixels[0];
        let g = fb.pixels[1];
        let b = fb.pixels[2];
        let a = fb.pixels[3];

        // Allow a tolerance of +/- 2 for integer rounding
        assert!((r as i32 - 128).abs() <= 2, "r={} expected ~128", r);
        assert!((g as i32 - 128).abs() <= 2, "g={} expected ~128", g);
        assert!((b as i32 - 128).abs() <= 2, "b={} expected ~128", b);
        assert_eq!(a, 255, "blended alpha should always be 255");
    }

    #[test]
    fn set_pixel_blended_onto_colored_background() {
        let mut fb = Framebuffer::new(4, 4);
        fb.clear(Color::from_rgba(100, 0, 0, 255)); // red background

        // Blend blue at 50%
        fb.set_pixel_blended(0, 0, Color::from_rgba(0, 0, 200, 128));

        let r = fb.pixels[0];
        let b = fb.pixels[2];

        // Red channel: (0 * 128 + 100 * 127) / 255 ~= 49-50
        assert!((r as i32 - 50).abs() <= 2, "r={} expected ~50", r);
        // Blue channel: (200 * 128 + 0 * 127) / 255 ~= 100
        assert!((b as i32 - 100).abs() <= 2, "b={} expected ~100", b);
    }

    #[test]
    fn set_pixel_blended_out_of_bounds_no_panic() {
        let mut fb = Framebuffer::new(4, 4);
        fb.set_pixel_blended(-1, -1, Color::from_rgba(255, 0, 0, 128));
        fb.set_pixel_blended(100, 100, Color::from_rgba(255, 0, 0, 128));
    }

    // --- len ---

    #[test]
    fn len_returns_correct_value() {
        let fb = Framebuffer::new(10, 20);
        assert_eq!(fb.len(), 10 * 20 * 4);
    }

    #[test]
    fn len_matches_pixels_vec_len() {
        let fb = Framebuffer::new(7, 13);
        assert_eq!(fb.len(), fb.pixels.len());
    }

    #[test]
    fn len_1x1() {
        let fb = Framebuffer::new(1, 1);
        assert_eq!(fb.len(), 4);
    }

    // --- blit_rect basic ---

    #[test]
    fn blit_rect_copies_pixels_without_alpha() {
        let mut fb = Framebuffer::new(4, 4);
        fb.clear(Color::BLACK);

        // 2x2 red source
        let src = vec![
            255, 0, 0, 255,  255, 0, 0, 255,
            255, 0, 0, 255,  255, 0, 0, 255,
        ];
        fb.blit_rect(&src, 2, 0, 0, 2, 2, 0, 0, false);

        // Check top-left 2x2 is red
        for y in 0..2 {
            for x in 0..2 {
                let idx = (y * 4 + x) * 4;
                assert_eq!(fb.pixels[idx], 255, "pixel ({},{}) r", x, y);
                assert_eq!(fb.pixels[idx + 1], 0);
                assert_eq!(fb.pixels[idx + 2], 0);
            }
        }
    }

    // --- Negative coordinates ---

    #[test]
    fn negative_coords_set_pixel_does_not_panic() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_pixel(-5, -5, Color::RED);
        fb.set_pixel(-1, 5, Color::RED);
        fb.set_pixel(5, -1, Color::RED);
        // If we reach here without panic, the test passes
    }

    #[test]
    fn negative_coords_set_pixel_blended_does_not_panic() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_pixel_blended(-5, -5, Color::from_rgba(255, 0, 0, 128));
        fb.set_pixel_blended(-1, 5, Color::from_rgba(255, 0, 0, 128));
        fb.set_pixel_blended(5, -1, Color::from_rgba(255, 0, 0, 128));
    }
}
