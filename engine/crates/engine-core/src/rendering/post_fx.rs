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
pub fn shift(fb: &mut Framebuffer, dx: i32, dy: i32) {
    if dx == 0 && dy == 0 { return; }
    let w = fb.width as i32;
    let h = fb.height as i32;
    let old = fb.pixels.clone();
    fb.clear(Color::BLACK);
    for y in 0..h {
        let src_y = y - dy;
        if src_y < 0 || src_y >= h { continue; }
        for x in 0..w {
            let src_x = x - dx;
            if src_x < 0 || src_x >= w { continue; }
            let si = ((src_y * w + src_x) * 4) as usize;
            let di = ((y * w + x) * 4) as usize;
            fb.pixels[di..di + 4].copy_from_slice(&old[si..si + 4]);
        }
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
