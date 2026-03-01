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
