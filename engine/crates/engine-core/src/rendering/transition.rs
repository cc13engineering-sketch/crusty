use super::color::Color;
use super::framebuffer::Framebuffer;

/// The kind of visual transition effect.
#[derive(Clone, Debug)]
pub enum TransitionKind {
    /// Fade the entire screen toward a solid color.
    FadeToColor { color: Color },
    /// Circular iris wipe centered at (cx, cy) in normalized [0..1] screen coords.
    IrisWipe { cx: f64, cy: f64 },
    /// Pixelation effect that increases block size up to `block_size_max`.
    Pixelate { block_size_max: u32 },
}

/// Which half of the transition we are in.
#[derive(Clone, Debug)]
pub enum TransitionPhase {
    /// No transition active.
    Idle,
    /// Transitioning out (obscuring the screen). `t` goes from 0.0 to 1.0.
    Out { t: f64, duration: f64 },
    /// Transitioning in (revealing the new scene). `t` goes from 1.0 to 0.0.
    In { t: f64, duration: f64 },
}

/// Manages scene transition effects. Captures a snapshot of the framebuffer at
/// start, then applies a visual effect over time (out phase -> midpoint -> in phase).
pub struct TransitionManager {
    pub phase: TransitionPhase,
    pub kind: TransitionKind,
    snapshot: Vec<u8>,
    snapshot_width: u32,
    snapshot_height: u32,
}

impl TransitionManager {
    pub fn new() -> Self {
        Self {
            phase: TransitionPhase::Idle,
            kind: TransitionKind::FadeToColor { color: Color::BLACK },
            snapshot: Vec::new(),
            snapshot_width: 0,
            snapshot_height: 0,
        }
    }

    /// Begin a transition. Captures the current framebuffer as a snapshot and
    /// enters the Out phase.
    pub fn start(&mut self, kind: TransitionKind, duration: f64, fb: &Framebuffer) {
        self.snapshot = fb.pixels.clone();
        self.snapshot_width = fb.width;
        self.snapshot_height = fb.height;
        self.kind = kind;
        // Each half gets half the total duration
        let half = (duration / 2.0).max(0.001);
        self.phase = TransitionPhase::Out { t: 0.0, duration: half };
    }

    /// Advance the transition by `dt` seconds. Returns true when the full
    /// transition (out + in) is complete.
    pub fn update(&mut self, dt: f64) -> bool {
        match self.phase {
            TransitionPhase::Idle => true,
            TransitionPhase::Out { ref mut t, duration } => {
                *t += dt / duration;
                if *t >= 1.0 {
                    // Switch to the In phase
                    self.phase = TransitionPhase::In { t: 1.0, duration };
                }
                false
            }
            TransitionPhase::In { ref mut t, duration } => {
                *t -= dt / duration;
                if *t <= 0.0 {
                    self.phase = TransitionPhase::Idle;
                    self.snapshot.clear();
                    return true;
                }
                false
            }
        }
    }

    /// Apply the transition effect onto the framebuffer.
    pub fn apply(&self, fb: &mut Framebuffer) {
        let t = match self.phase {
            TransitionPhase::Idle => return,
            TransitionPhase::Out { t, .. } => t.max(0.0).min(1.0),
            TransitionPhase::In { t, .. } => t.max(0.0).min(1.0),
        };

        match self.kind {
            TransitionKind::FadeToColor { color } => {
                self.apply_fade(fb, color, t);
            }
            TransitionKind::IrisWipe { cx, cy } => {
                self.apply_iris(fb, cx, cy, t);
            }
            TransitionKind::Pixelate { block_size_max } => {
                self.apply_pixelate(fb, block_size_max, t);
            }
        }
    }

    /// Returns true if a transition is currently running.
    pub fn is_active(&self) -> bool {
        !matches!(self.phase, TransitionPhase::Idle)
    }

    /// Returns true when the transition has just crossed from Out to In,
    /// i.e. we are at the midpoint where the scene swap should happen.
    pub fn midpoint_reached(&self) -> bool {
        matches!(self.phase, TransitionPhase::In { t, .. } if t >= 0.99)
    }

    // ── Private effect implementations ───────────────────────────────

    fn apply_fade(&self, fb: &mut Framebuffer, color: Color, t: f64) {
        let w = fb.width;
        let h = fb.height;
        let len = (w as usize) * (h as usize);
        for i in 0..len {
            let idx = i * 4;
            if idx + 3 >= fb.pixels.len() {
                break;
            }
            let r = fb.pixels[idx] as f64;
            let g = fb.pixels[idx + 1] as f64;
            let b = fb.pixels[idx + 2] as f64;
            let inv = 1.0 - t;
            fb.pixels[idx] = (r * inv + color.r as f64 * t) as u8;
            fb.pixels[idx + 1] = (g * inv + color.g as f64 * t) as u8;
            fb.pixels[idx + 2] = (b * inv + color.b as f64 * t) as u8;
        }
    }

    fn apply_iris(&self, fb: &mut Framebuffer, cx: f64, cy: f64, t: f64) {
        let w = fb.width as f64;
        let h = fb.height as f64;
        // Maximum radius is the diagonal of the screen
        let max_radius = (w * w + h * h).sqrt();
        // During Out: radius shrinks from max to 0 (t goes 0->1, so radius = max*(1-t))
        // During In: radius grows from 0 to max (t goes 1->0, so radius = max*(1-t))
        // In both cases, radius = max_radius * (1.0 - t) works:
        //   Out: t=0 -> full screen visible, t=1 -> nothing visible
        //   In:  t=1 -> nothing visible, t=0 -> full screen visible
        let radius = max_radius * (1.0 - t);
        let r2 = radius * radius;
        let center_x = cx * w;
        let center_y = cy * h;

        let fw = fb.width;
        let fh = fb.height;
        for y in 0..fh {
            let dy = y as f64 - center_y;
            for x in 0..fw {
                let dx = x as f64 - center_x;
                let dist2 = dx * dx + dy * dy;
                if dist2 > r2 {
                    // Outside the iris circle: black it out
                    let idx = ((y * fw + x) * 4) as usize;
                    if idx + 3 < fb.pixels.len() {
                        fb.pixels[idx] = 0;
                        fb.pixels[idx + 1] = 0;
                        fb.pixels[idx + 2] = 0;
                    }
                }
            }
        }
    }

    fn apply_pixelate(&self, fb: &mut Framebuffer, block_size_max: u32, t: f64) {
        if block_size_max <= 1 {
            return;
        }
        // Block size scales from 1 (no effect) up to block_size_max
        let block = (1.0 + (block_size_max as f64 - 1.0) * t).round() as u32;
        if block <= 1 {
            return;
        }
        let w = fb.width;
        let h = fb.height;

        // Process each block: sample center pixel, fill entire block
        let mut by = 0u32;
        while by < h {
            let mut bx = 0u32;
            while bx < w {
                // Sample the center of the block
                let sample_x = (bx + block / 2).min(w - 1);
                let sample_y = (by + block / 2).min(h - 1);
                let si = ((sample_y * w + sample_x) * 4) as usize;

                if si + 3 >= fb.pixels.len() {
                    bx += block;
                    continue;
                }

                let r = fb.pixels[si];
                let g = fb.pixels[si + 1];
                let b = fb.pixels[si + 2];
                let a = fb.pixels[si + 3];

                // Fill the block
                let end_y = (by + block).min(h);
                let end_x = (bx + block).min(w);
                for py in by..end_y {
                    for px in bx..end_x {
                        let di = ((py * w + px) * 4) as usize;
                        if di + 3 < fb.pixels.len() {
                            fb.pixels[di] = r;
                            fb.pixels[di + 1] = g;
                            fb.pixels[di + 2] = b;
                            fb.pixels[di + 3] = a;
                        }
                    }
                }
                bx += block;
            }
            by += block;
        }
    }
}
