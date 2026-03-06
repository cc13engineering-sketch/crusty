//! Ambient particle layer — persistent, force-responsive particles that drift
//! around the screen. Unlike the burst-oriented `ParticlePool`, these particles
//! stay alive indefinitely and wrap at screen edges. Think: dust motes, snow,
//! fireflies, musical notes floating in the background.

use crate::rendering::color::Color;
use crate::rendering::framebuffer::Framebuffer;
use crate::rendering::shapes;
use crate::engine::Camera;
use crate::rng::SeededRng;

// ─── Particle Type ──────────────────────────────────────────────────────────

/// Visual shape for ambient particles.
#[derive(Clone, Debug, PartialEq)]
pub enum AmbientParticleType {
    /// Simple dot/circle.
    Dot,
    /// Small diamond shape.
    Diamond,
    /// Musical note shape (for music games) — filled circle with a stem line.
    Note,
    /// Snowflake / star shape (4-pointed).
    Star,
}

// ─── Internal Particle ──────────────────────────────────────────────────────

/// A single ambient particle. Not an ECS component — lives in its own pool.
#[derive(Clone, Debug)]
struct AmbientParticle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    size: f64,
    alpha: f64,
    particle_type: AmbientParticleType,
    /// Cached color index: false = base_color, true = alt_color (if available).
    use_alt_color: bool,
}

// ─── Config ─────────────────────────────────────────────────────────────────

/// Configuration for an ambient particle layer.
#[derive(Clone, Debug)]
pub struct AmbientParticleConfig {
    /// Number of ambient particles to maintain.
    pub count: usize,
    /// How much particles respond to force fields (0.0 = ignore, 1.0 = full).
    pub force_response: f64,
    /// Base color for particles.
    pub base_color: Color,
    /// Optional second color — particles randomly choose between base and alt.
    pub alt_color: Option<Color>,
    /// Size range (min, max) in pixels.
    pub size_range: (f64, f64),
    /// Base speed for random drift.
    pub drift_speed: f64,
    /// Type of particles to spawn.
    pub particle_type: AmbientParticleType,
    /// Alpha range (min, max), each in 0.0..=1.0.
    pub alpha_range: (f64, f64),
}

impl Default for AmbientParticleConfig {
    fn default() -> Self {
        Self {
            count: 100,
            force_response: 0.3,
            base_color: Color::from_rgba(200, 200, 200, 255),
            alt_color: None,
            size_range: (1.0, 3.0),
            drift_speed: 15.0,
            particle_type: AmbientParticleType::Dot,
            alpha_range: (0.15, 0.45),
        }
    }
}

// ─── Layer ──────────────────────────────────────────────────────────────────

/// A persistent layer of ambient particles that drift and respond to forces.
#[derive(Clone, Debug)]
pub struct AmbientParticleLayer {
    pub config: AmbientParticleConfig,
    particles: Vec<AmbientParticle>,
    initialized: bool,
}

impl AmbientParticleLayer {
    pub fn new(config: AmbientParticleConfig) -> Self {
        Self {
            particles: Vec::with_capacity(config.count),
            config,
            initialized: false,
        }
    }

    /// Seed particles across the screen. Uses `seed` for deterministic placement.
    pub fn initialize(&mut self, width: f64, height: f64, seed: u64) {
        let mut rng = SeededRng::new(seed);
        self.particles.clear();
        for _ in 0..self.config.count {
            self.particles.push(spawn_particle(
                &self.config,
                &mut rng,
                width,
                height,
            ));
        }
        self.initialized = true;
    }

    /// Advance particles by `dt` seconds. `forces` is a slice of
    /// `(fx, fy, x, y)` force vectors at world positions — particles within
    /// range are nudged by `(fx, fy)` scaled by `config.force_response` and
    /// inverse distance.
    pub fn update(
        &mut self,
        dt: f64,
        width: f64,
        height: f64,
        forces: &[(f64, f64, f64, f64)],
    ) {
        if !self.initialized {
            return;
        }

        let response = self.config.force_response;

        for p in &mut self.particles {
            // Apply radial force fields.
            // Each entry is (radial_strength, _, center_x, center_y).
            // Positive strength pushes away from center; negative pulls toward.
            for &(strength, _unused, force_x, force_y) in forces {
                let dx = p.x - force_x;
                let dy = p.y - force_y;
                let dist_sq = dx * dx + dy * dy;
                let dist_sq_clamped = dist_sq.max(100.0);
                let dist = dist_sq_clamped.sqrt();
                let influence = response * 1000.0 / dist_sq_clamped;
                // Radial direction: dx/dist points away from center
                p.vx += (dx / dist) * strength * influence * dt;
                p.vy += (dy / dist) * strength * influence * dt;
            }

            // Integrate position.
            p.x += p.vx * dt;
            p.y += p.vy * dt;

            // Light damping so particles don't accelerate forever from forces.
            let damping = (-2.0 * dt).exp();
            p.vx *= damping;
            p.vy *= damping;

            // Wrap at screen edges.
            if p.x < -p.size {
                p.x += width + p.size * 2.0;
            } else if p.x > width + p.size {
                p.x -= width + p.size * 2.0;
            }
            if p.y < -p.size {
                p.y += height + p.size * 2.0;
            } else if p.y > height + p.size {
                p.y -= height + p.size * 2.0;
            }
        }
    }

    /// Render all particles into the framebuffer.
    pub fn render(&self, fb: &mut Framebuffer, camera: &Camera) {
        if !self.initialized {
            return;
        }

        for p in &self.particles {
            let color = if p.use_alt_color {
                self.config.alt_color.unwrap_or(self.config.base_color)
            } else {
                self.config.base_color
            };

            // Scale alpha: particle alpha (0..1) * color alpha channel.
            let final_alpha = (p.alpha * color.a as f64).round().max(0.0).min(255.0) as u8;
            if final_alpha == 0 {
                continue;
            }
            let c = color.with_alpha(final_alpha);

            let (sx, sy) = camera.world_to_screen(p.x, p.y);
            let sx_f = sx as f64;
            let sy_f = sy as f64;

            match p.particle_type {
                AmbientParticleType::Dot => {
                    if p.size < 1.5 {
                        // Very small: single blended pixel is sufficient.
                        fb.set_pixel_blended(sx, sy, c);
                    } else {
                        shapes::fill_circle(fb, sx_f, sy_f, p.size / 2.0, c);
                    }
                }
                AmbientParticleType::Diamond => {
                    render_diamond(fb, sx_f, sy_f, p.size, c);
                }
                AmbientParticleType::Note => {
                    render_note(fb, sx_f, sy_f, p.size, c);
                }
                AmbientParticleType::Star => {
                    render_star(fb, sx_f, sy_f, p.size, c);
                }
            }
        }
    }

    /// Number of live particles.
    pub fn count(&self) -> usize {
        self.particles.len()
    }

    /// Remove all particles and mark the layer as uninitialized.
    pub fn clear(&mut self) {
        self.particles.clear();
        self.initialized = false;
    }
}

// ─── Helpers ────────────────────────────────────────────────────────────────

/// Create a single randomly-placed ambient particle.
fn spawn_particle(
    config: &AmbientParticleConfig,
    rng: &mut SeededRng,
    width: f64,
    height: f64,
) -> AmbientParticle {
    let angle = rng.next_f64() * std::f64::consts::TAU;
    let speed = config.drift_speed * (0.3 + rng.next_f64() * 0.7);
    let (min_size, max_size) = config.size_range;
    let size = min_size + rng.next_f64() * (max_size - min_size);
    let (min_alpha, max_alpha) = config.alpha_range;
    let alpha = min_alpha + rng.next_f64() * (max_alpha - min_alpha);
    let use_alt = if config.alt_color.is_some() {
        rng.chance(0.5)
    } else {
        false
    };

    AmbientParticle {
        x: rng.next_f64() * width,
        y: rng.next_f64() * height,
        vx: angle.cos() * speed,
        vy: angle.sin() * speed,
        size,
        alpha,
        particle_type: config.particle_type.clone(),
        use_alt_color: use_alt,
    }
}

/// Render a diamond shape centered at (cx, cy) with the given size.
/// Uses pixel-level SDF with 1px anti-aliased feather.
fn render_diamond(fb: &mut Framebuffer, cx: f64, cy: f64, size: f64, color: Color) {
    let half = size / 2.0;
    let feather = 1.0;
    let outer = half + feather;
    let x0 = (cx - outer).floor() as i32;
    let y0 = (cy - outer).floor() as i32;
    let x1 = (cx + outer).ceil() as i32;
    let y1 = (cy + outer).ceil() as i32;

    for py in y0..=y1 {
        let dy = (py as f64 + 0.5 - cy).abs();
        for px in x0..=x1 {
            let dx = (px as f64 + 0.5 - cx).abs();
            // Diamond SDF: |x| + |y| <= half
            let dist = dx + dy;
            if dist <= half {
                fb.set_pixel_blended(px, py, color);
            } else if dist < half + feather {
                let t = 1.0 - (dist - half) / feather;
                let aa_alpha = (color.a as f64 * t).round() as u8;
                if aa_alpha > 0 {
                    fb.set_pixel_blended(px, py, color.with_alpha(aa_alpha));
                }
            }
        }
    }
}

/// Render a musical note (quarter note: filled circle head + vertical stem).
fn render_note(fb: &mut Framebuffer, cx: f64, cy: f64, size: f64, color: Color) {
    let head_r = size / 2.0;
    // Note head — slightly below center so the stem goes up.
    let head_cy = cy + head_r * 0.5;
    shapes::fill_circle(fb, cx, head_cy, head_r, color);

    // Stem — thin vertical line going up from the right side of the head.
    let stem_x = cx + head_r * 0.7;
    let stem_bottom = head_cy - head_r * 0.3;
    let stem_top = head_cy - size * 1.8;
    shapes::draw_line_thick(fb, stem_x, stem_bottom, stem_x, stem_top, 1.0, color);
}

/// Render a 4-pointed star centered at (cx, cy) with the given size.
/// Uses pixel-level SDF with anti-aliased feather.
fn render_star(fb: &mut Framebuffer, cx: f64, cy: f64, size: f64, color: Color) {
    let outer_r = size / 2.0;
    let inner_r = outer_r * 0.4;
    let feather = 1.0;
    let bound = outer_r + feather;
    let x0 = (cx - bound).floor() as i32;
    let y0 = (cy - bound).floor() as i32;
    let x1 = (cx + bound).ceil() as i32;
    let y1 = (cy + bound).ceil() as i32;

    for py in y0..=y1 {
        let dy = py as f64 + 0.5 - cy;
        for px in x0..=x1 {
            let dx = px as f64 + 0.5 - cx;
            // 4-pointed star SDF: use the min of two diamond SDFs rotated 45 degrees.
            let adx = dx.abs();
            let ady = dy.abs();
            // Axis-aligned diamond.
            let d1 = adx / outer_r + ady / inner_r;
            // Rotated diamond.
            let d2 = adx / inner_r + ady / outer_r;
            let dist = d1.min(d2);

            if dist <= 1.0 {
                fb.set_pixel_blended(px, py, color);
            } else if dist < 1.0 + feather / outer_r {
                let t = 1.0 - (dist - 1.0) / (feather / outer_r);
                let aa_alpha = (color.a as f64 * t).round() as u8;
                if aa_alpha > 0 {
                    fb.set_pixel_blended(px, py, color.with_alpha(aa_alpha));
                }
            }
        }
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_defaults_are_sensible() {
        let cfg = AmbientParticleConfig::default();
        assert_eq!(cfg.count, 100);
        assert_eq!(cfg.particle_type, AmbientParticleType::Dot);
        assert!(cfg.drift_speed > 0.0);
        assert!(cfg.force_response >= 0.0 && cfg.force_response <= 1.0);
        assert!(cfg.size_range.0 > 0.0);
        assert!(cfg.size_range.1 >= cfg.size_range.0);
        assert!(cfg.alpha_range.0 >= 0.0 && cfg.alpha_range.0 <= 1.0);
        assert!(cfg.alpha_range.1 >= cfg.alpha_range.0 && cfg.alpha_range.1 <= 1.0);
        assert!(cfg.alt_color.is_none());
    }

    #[test]
    fn initialize_creates_correct_count() {
        let cfg = AmbientParticleConfig {
            count: 50,
            ..AmbientParticleConfig::default()
        };
        let mut layer = AmbientParticleLayer::new(cfg);
        assert_eq!(layer.count(), 0);

        layer.initialize(800.0, 600.0, 42);
        assert_eq!(layer.count(), 50);
    }

    #[test]
    fn initialize_is_deterministic() {
        let cfg = AmbientParticleConfig::default();
        let mut a = AmbientParticleLayer::new(cfg.clone());
        let mut b = AmbientParticleLayer::new(cfg);
        a.initialize(800.0, 600.0, 123);
        b.initialize(800.0, 600.0, 123);

        assert_eq!(a.count(), b.count());
        for (pa, pb) in a.particles.iter().zip(b.particles.iter()) {
            assert_eq!(pa.x, pb.x);
            assert_eq!(pa.y, pb.y);
            assert_eq!(pa.vx, pb.vx);
            assert_eq!(pa.vy, pb.vy);
            assert_eq!(pa.size, pb.size);
            assert_eq!(pa.alpha, pb.alpha);
        }
    }

    #[test]
    fn update_wraps_particles_at_edges() {
        let cfg = AmbientParticleConfig {
            count: 1,
            drift_speed: 0.0,
            force_response: 0.0,
            size_range: (2.0, 2.0),
            ..AmbientParticleConfig::default()
        };
        let mut layer = AmbientParticleLayer::new(cfg);
        layer.initialize(100.0, 100.0, 1);

        // Force particle to the right edge.
        layer.particles[0].x = 105.0;
        layer.particles[0].vx = 0.0;
        layer.particles[0].vy = 0.0;

        layer.update(0.016, 100.0, 100.0, &[]);

        // Should have wrapped to the left side.
        assert!(
            layer.particles[0].x < 10.0,
            "particle x={} should have wrapped to left side",
            layer.particles[0].x
        );
    }

    #[test]
    fn update_wraps_negative() {
        let cfg = AmbientParticleConfig {
            count: 1,
            drift_speed: 0.0,
            force_response: 0.0,
            size_range: (2.0, 2.0),
            ..AmbientParticleConfig::default()
        };
        let mut layer = AmbientParticleLayer::new(cfg);
        layer.initialize(100.0, 100.0, 1);

        // Force particle past the left edge.
        layer.particles[0].x = -5.0;
        layer.particles[0].y = -5.0;
        layer.particles[0].vx = 0.0;
        layer.particles[0].vy = 0.0;

        layer.update(0.016, 100.0, 100.0, &[]);

        assert!(
            layer.particles[0].x > 90.0,
            "particle x={} should have wrapped to right side",
            layer.particles[0].x
        );
        assert!(
            layer.particles[0].y > 90.0,
            "particle y={} should have wrapped to bottom",
            layer.particles[0].y
        );
    }

    #[test]
    fn clear_empties_the_layer() {
        let cfg = AmbientParticleConfig::default();
        let mut layer = AmbientParticleLayer::new(cfg);
        layer.initialize(800.0, 600.0, 42);
        assert!(layer.count() > 0);

        layer.clear();
        assert_eq!(layer.count(), 0);
        assert!(!layer.initialized);
    }

    #[test]
    fn forces_influence_velocity() {
        let cfg = AmbientParticleConfig {
            count: 1,
            drift_speed: 0.0,
            force_response: 1.0,
            size_range: (2.0, 2.0),
            ..AmbientParticleConfig::default()
        };
        let mut layer = AmbientParticleLayer::new(cfg);
        layer.initialize(800.0, 600.0, 42);

        // Place particle at a known position.
        layer.particles[0].x = 400.0;
        layer.particles[0].y = 300.0;
        layer.particles[0].vx = 0.0;
        layer.particles[0].vy = 0.0;

        // Apply a force nearby pushing right.
        let forces = [(100.0, 0.0, 400.0, 300.0)];
        layer.update(0.016, 800.0, 600.0, &forces);

        // Particle should have moved right.
        assert!(
            layer.particles[0].x > 400.0,
            "particle should have moved right, x={}",
            layer.particles[0].x
        );
    }

    #[test]
    fn alt_color_distributes_roughly_evenly() {
        let cfg = AmbientParticleConfig {
            count: 200,
            alt_color: Some(Color::RED),
            ..AmbientParticleConfig::default()
        };
        let mut layer = AmbientParticleLayer::new(cfg);
        layer.initialize(800.0, 600.0, 42);

        let alt_count = layer.particles.iter().filter(|p| p.use_alt_color).count();
        // With 200 particles and 50% chance, expect roughly 100. Allow wide margin.
        assert!(alt_count > 50, "too few alt-colored particles: {}", alt_count);
        assert!(alt_count < 150, "too many alt-colored particles: {}", alt_count);
    }

    #[test]
    fn render_does_not_panic_on_empty() {
        let cfg = AmbientParticleConfig::default();
        let layer = AmbientParticleLayer::new(cfg);
        let mut fb = Framebuffer::new(100, 100);
        let cam = Camera::default();
        layer.render(&mut fb, &cam);
    }

    #[test]
    fn render_dot_does_not_panic() {
        let cfg = AmbientParticleConfig {
            count: 5,
            particle_type: AmbientParticleType::Dot,
            ..AmbientParticleConfig::default()
        };
        let mut layer = AmbientParticleLayer::new(cfg);
        layer.initialize(100.0, 100.0, 42);
        let mut fb = Framebuffer::new(100, 100);
        let cam = Camera::default();
        layer.render(&mut fb, &cam);
    }

    #[test]
    fn render_diamond_does_not_panic() {
        let cfg = AmbientParticleConfig {
            count: 5,
            particle_type: AmbientParticleType::Diamond,
            ..AmbientParticleConfig::default()
        };
        let mut layer = AmbientParticleLayer::new(cfg);
        layer.initialize(100.0, 100.0, 42);
        let mut fb = Framebuffer::new(100, 100);
        let cam = Camera::default();
        layer.render(&mut fb, &cam);
    }

    #[test]
    fn render_note_does_not_panic() {
        let cfg = AmbientParticleConfig {
            count: 5,
            particle_type: AmbientParticleType::Note,
            ..AmbientParticleConfig::default()
        };
        let mut layer = AmbientParticleLayer::new(cfg);
        layer.initialize(100.0, 100.0, 42);
        let mut fb = Framebuffer::new(100, 100);
        let cam = Camera::default();
        layer.render(&mut fb, &cam);
    }

    #[test]
    fn render_star_does_not_panic() {
        let cfg = AmbientParticleConfig {
            count: 5,
            particle_type: AmbientParticleType::Star,
            ..AmbientParticleConfig::default()
        };
        let mut layer = AmbientParticleLayer::new(cfg);
        layer.initialize(100.0, 100.0, 42);
        let mut fb = Framebuffer::new(100, 100);
        let cam = Camera::default();
        layer.render(&mut fb, &cam);
    }
}
