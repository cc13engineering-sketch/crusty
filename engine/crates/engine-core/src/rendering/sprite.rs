/// Sprite sheet renderer: slices a raw RGBA pixel buffer into tiles
/// and blits them to a Framebuffer.

use super::framebuffer::Framebuffer;
use super::color::Color;

#[derive(Clone, Debug)]
pub struct SpriteSheet {
    pub pixels: Vec<u8>,    // raw RGBA data
    pub width: u32,         // full sheet width in pixels
    pub height: u32,        // full sheet height in pixels
    pub tile_w: u32,        // width of each tile
    pub tile_h: u32,        // height of each tile
    pub columns: u32,       // number of tile columns in the sheet
}

impl SpriteSheet {
    pub fn new(pixels: Vec<u8>, width: u32, height: u32, tile_w: u32, tile_h: u32) -> Self {
        let columns = if tile_w > 0 { width / tile_w } else { 0 };
        Self {
            pixels,
            width,
            height,
            tile_w,
            tile_h,
            columns,
        }
    }

    /// Draw a tile from the sprite sheet at the given destination position.
    /// Uses `Framebuffer::blit_rect` with alpha blending.
    pub fn draw_tile(&self, fb: &mut Framebuffer, index: u32, dx: i32, dy: i32) {
        if self.columns == 0 {
            return;
        }
        let col = index % self.columns;
        let row = index / self.columns;
        let sx = col * self.tile_w;
        let sy = row * self.tile_h;

        // Bounds check: make sure the tile region fits within the sheet
        if sx + self.tile_w > self.width || sy + self.tile_h > self.height {
            return;
        }

        fb.blit_rect(
            &self.pixels,
            self.width,
            sx,
            sy,
            self.tile_w,
            self.tile_h,
            dx,
            dy,
            true, // use alpha blending
        );
    }

    /// Draw a tile with optional horizontal and/or vertical flipping.
    /// When neither flip is set, this falls back to `draw_tile`.
    /// When flipping, pixels are manually blitted with coordinate mirroring.
    pub fn draw_tile_flipped(
        &self,
        fb: &mut Framebuffer,
        index: u32,
        dx: i32,
        dy: i32,
        flip_x: bool,
        flip_y: bool,
    ) {
        if !flip_x && !flip_y {
            self.draw_tile(fb, index, dx, dy);
            return;
        }

        if self.columns == 0 {
            return;
        }
        let col = index % self.columns;
        let row = index / self.columns;
        let sx = col * self.tile_w;
        let sy = row * self.tile_h;

        // Bounds check
        if sx + self.tile_w > self.width || sy + self.tile_h > self.height {
            return;
        }

        for py in 0..self.tile_h {
            for px in 0..self.tile_w {
                // Source pixel in sheet coordinates
                let src_x = sx + px;
                let src_y = sy + py;
                let src_idx = ((src_y * self.width + src_x) * 4) as usize;

                if src_idx + 3 >= self.pixels.len() {
                    continue;
                }

                let r = self.pixels[src_idx];
                let g = self.pixels[src_idx + 1];
                let b = self.pixels[src_idx + 2];
                let a = self.pixels[src_idx + 3];

                if a == 0 {
                    continue;
                }

                // Destination pixel with flipping
                let dest_px = if flip_x { self.tile_w - 1 - px } else { px };
                let dest_py = if flip_y { self.tile_h - 1 - py } else { py };
                let dest_x = dx + dest_px as i32;
                let dest_y = dy + dest_py as i32;

                let color = Color::from_rgba(r, g, b, a);
                fb.set_pixel_blended(dest_x, dest_y, color);
            }
        }
    }
}
