use super::color::Color;
use super::framebuffer::Framebuffer;
use super::shapes;
use crate::engine::Camera;

#[derive(Clone, Debug)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub life: f64,
    pub max_life: f64,
    pub size_start: f64,
    pub size_end: f64,
    pub color_start: Color,
    pub color_end: Color,
}

pub struct ParticlePool {
    pub particles: Vec<Particle>,
}

impl ParticlePool {
    pub fn new() -> Self {
        Self { particles: Vec::with_capacity(512) }
    }

    pub fn spawn(&mut self, p: Particle) {
        if self.particles.len() < 2048 {
            self.particles.push(p);
        }
    }

    pub fn spawn_burst(
        &mut self,
        x: f64, y: f64,
        count: u32,
        speed_min: f64, speed_max: f64,
        life: f64,
        size_start: f64, size_end: f64,
        color_start: Color, color_end: Color,
        seed: u64,
    ) {
        let mut rng = SimpleRng::new(seed);
        for _ in 0..count {
            let angle = rng.next_f64() * std::f64::consts::TAU;
            let speed = speed_min + rng.next_f64() * (speed_max - speed_min);
            let life_var = life * (0.5 + rng.next_f64() * 0.5);
            self.spawn(Particle {
                x, y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                life: life_var,
                max_life: life_var,
                size_start,
                size_end,
                color_start,
                color_end,
            });
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.particles.retain_mut(|p| {
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            // Slight drag
            p.vx *= 0.99;
            p.vy *= 0.99;
            p.life -= dt;
            p.life > 0.0
        });
    }

    pub fn render(&self, fb: &mut Framebuffer, camera: &Camera) {
        for p in &self.particles {
            let t = 1.0 - (p.life / p.max_life).max(0.0);
            let size = p.size_start + (p.size_end - p.size_start) * t;
            let color = Color::lerp(p.color_start, p.color_end, t);
            let (sx, sy) = camera.world_to_screen(p.x, p.y);
            if size <= 1.5 {
                fb.set_pixel_blended(sx, sy, color);
            } else {
                shapes::fill_circle(fb, sx as f64, sy as f64, size, color);
            }
        }
    }

    pub fn count(&self) -> usize {
        self.particles.len()
    }
}

/// Simple deterministic PRNG (xorshift64).
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() % 10000) as f64 / 10000.0
    }
}
