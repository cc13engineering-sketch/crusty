use super::framebuffer::Framebuffer;
use super::color::Color;

/// Darkens edges of screen in a circular gradient.
pub fn vignette(fb: &mut Framebuffer, strength: f64) {
    let cx = fb.width as f64 / 2.0;
    let cy = fb.height as f64 / 2.0;
    let max_dist = (cx * cx + cy * cy).sqrt();
    for y in 0..fb.height {
        let dy = y as f64 - cy;
        for x in 0..fb.width {
            let dx = x as f64 - cx;
            let dist = (dx * dx + dy * dy).sqrt() / max_dist;
            let factor = 1.0 - (dist * dist * strength).min(1.0);
            let idx = ((y * fb.width + x) * 4) as usize;
            fb.pixels[idx] = (fb.pixels[idx] as f64 * factor) as u8;
            fb.pixels[idx + 1] = (fb.pixels[idx + 1] as f64 * factor) as u8;
            fb.pixels[idx + 2] = (fb.pixels[idx + 2] as f64 * factor) as u8;
        }
    }
}

/// Retro scanline effect: dims every Nth row.
pub fn scanlines(fb: &mut Framebuffer, spacing: u32, darkness: f64) {
    let factor = 1.0 - darkness.min(1.0).max(0.0);
    for y in (0..fb.height).step_by(spacing.max(1) as usize) {
        for x in 0..fb.width {
            let idx = ((y * fb.width + x) * 4) as usize;
            fb.pixels[idx] = (fb.pixels[idx] as f64 * factor) as u8;
            fb.pixels[idx + 1] = (fb.pixels[idx + 1] as f64 * factor) as u8;
            fb.pixels[idx + 2] = (fb.pixels[idx + 2] as f64 * factor) as u8;
        }
    }
}

/// Shifts all pixels by (dx, dy). Used for screen shake.
/// Zero-allocation: uses row-by-row copy_within, iterating in the
/// correct order to avoid overwriting source data.
pub fn shift(fb: &mut Framebuffer, dx: i32, dy: i32) {
    if dx == 0 && dy == 0 { return; }
    let w = fb.width as i32;
    let h = fb.height as i32;
    let stride = (w * 4) as usize;

    // Determine valid source/dest row ranges
    let (y_start, y_end, y_step): (i32, i32, i32) = if dy >= 0 {
        (h - 1, -1, -1) // bottom-up to avoid overwriting
    } else {
        (0, h, 1) // top-down
    };

    // Horizontal pixel range that survives the shift (computed in signed i32
    // to avoid wrapping when |dx| >= w).
    let src_x0_i = 0i32.max(-dx);
    let src_x1_i = w.min(w - dx);
    if src_x1_i <= src_x0_i { fb.clear(Color::BLACK); return; }
    let src_x0 = src_x0_i as usize;
    let src_x1 = src_x1_i as usize;
    let dst_x0 = 0i32.max(dx) as usize;
    let row_bytes = (src_x1 - src_x0) * 4;

    let mut y = y_start;
    while y != y_end {
        let src_y = y - dy;
        let dy_idx = y as usize * stride;
        if src_y >= 0 && src_y < h {
            let sy_idx = src_y as usize * stride;
            let src_start = sy_idx + src_x0 * 4;
            let dst_start = dy_idx + dst_x0 * 4;
            fb.pixels.copy_within(src_start..src_start + row_bytes, dst_start);
            // Clear exposed pixels on left (opaque black to maintain alpha=255 convention)
            if dst_x0 > 0 {
                for chunk in fb.pixels[dy_idx..dy_idx + dst_x0 * 4].chunks_exact_mut(4) {
                    chunk.copy_from_slice(&[0, 0, 0, 255]);
                }
            }
            // Clear exposed pixels on right
            let right_start = dy_idx + (dst_x0 + src_x1 - src_x0) * 4;
            let row_end = dy_idx + stride;
            if right_start < row_end {
                for chunk in fb.pixels[right_start..row_end].chunks_exact_mut(4) {
                    chunk.copy_from_slice(&[0, 0, 0, 255]);
                }
            }
        } else {
            // Row is entirely outside source — clear to opaque black
            for chunk in fb.pixels[dy_idx..dy_idx + stride].chunks_exact_mut(4) {
                chunk.copy_from_slice(&[0, 0, 0, 255]);
            }
        }
        y += y_step;
    }
}

/// Configuration for active post-processing effects.
pub struct PostFxConfig {
    pub vignette_strength: f64,
    pub scanline_spacing: u32,
    pub scanline_darkness: f64,
    pub shake_remaining: f64,
    pub shake_intensity: f64,
}

impl Default for PostFxConfig {
    fn default() -> Self {
        Self {
            vignette_strength: 0.0,
            scanline_spacing: 0,
            scanline_darkness: 0.3,
            shake_remaining: 0.0,
            shake_intensity: 5.0,
        }
    }
}

pub fn apply(fb: &mut Framebuffer, config: &mut PostFxConfig, dt: f64, frame: u64) {
    if config.vignette_strength > 0.0 {
        vignette(fb, config.vignette_strength);
    }
    if config.scanline_spacing > 0 {
        scanlines(fb, config.scanline_spacing, config.scanline_darkness);
    }
    if config.shake_remaining > 0.0 {
        let intensity = config.shake_intensity;
        let seed = frame.wrapping_mul(2654435761);
        let dx = ((seed % 100) as f64 / 100.0 * 2.0 - 1.0) * intensity;
        let dy = ((seed.wrapping_mul(31) % 100) as f64 / 100.0 * 2.0 - 1.0) * intensity;
        shift(fb, dx as i32, dy as i32);
        config.shake_remaining -= dt;
        if config.shake_remaining < 0.0 {
            config.shake_remaining = 0.0;
        }
    }
}
