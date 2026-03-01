use super::color::Color;
use super::framebuffer::Framebuffer;
use super::shapes;
use super::particles::SimpleRng;
use crate::engine::Camera;

pub struct Star {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub brightness: u8,
    pub color: Color,
    pub depth: f64,
}

pub struct Starfield {
    pub stars: Vec<Star>,
    pub twinkle: bool,
}

impl Starfield {
    pub fn generate(seed: u64, world_w: f64, world_h: f64, density: u32) -> Self {
        let mut stars = Vec::new();
        let mut rng = SimpleRng::new(seed);

        // Layer 1: Far (dim, single pixel)
        for _ in 0..density {
            stars.push(Star {
                x: rng.next_f64() * world_w * 3.0 - world_w,
                y: rng.next_f64() * world_h * 3.0 - world_h,
                size: 0.5,
                brightness: 40 + (rng.next_f64() * 40.0) as u8,
                color: Color::WHITE,
                depth: 0.05 + rng.next_f64() * 0.1,
            });
        }
        // Layer 2: Mid (medium brightness)
        for _ in 0..density / 2 {
            let tint = if rng.next_f64() < 0.2 {
                Color::from_rgba(180, 200, 255, 255)
            } else {
                Color::WHITE
            };
            stars.push(Star {
                x: rng.next_f64() * world_w * 2.0 - world_w * 0.5,
                y: rng.next_f64() * world_h * 2.0 - world_h * 0.5,
                size: 1.0 + rng.next_f64() * 0.5,
                brightness: 100 + (rng.next_f64() * 60.0) as u8,
                color: tint,
                depth: 0.2 + rng.next_f64() * 0.2,
            });
        }
        // Layer 3: Near (bright, some colored)
        for _ in 0..density / 4 {
            let tint = match (rng.next_f64() * 3.0) as u32 {
                0 => Color::from_rgba(255, 240, 200, 255),
                1 => Color::from_rgba(200, 220, 255, 255),
                _ => Color::WHITE,
            };
            stars.push(Star {
                x: rng.next_f64() * world_w * 1.5 - world_w * 0.25,
                y: rng.next_f64() * world_h * 1.5 - world_h * 0.25,
                size: 1.5 + rng.next_f64() * 1.5,
                brightness: 180 + (rng.next_f64() * 75.0).min(75.0) as u8,
                color: tint,
                depth: 0.5 + rng.next_f64() * 0.5,
            });
        }

        Starfield { stars, twinkle: true }
    }

    pub fn render(&self, fb: &mut Framebuffer, camera: &Camera, frame: u64) {
        let sw = fb.width as f64;
        let sh = fb.height as f64;

        for star in &self.stars {
            let parallax = 1.0 - star.depth;
            let mut sx = star.x - camera.x * parallax;
            let mut sy = star.y - camera.y * parallax;

            // Wrap stars
            sx = ((sx % sw) + sw) % sw;
            sy = ((sy % sh) + sh) % sh;

            let alpha = if self.twinkle {
                let hash = simple_hash(star.x.to_bits() ^ star.y.to_bits() ^ frame);
                let flicker = 0.7 + 0.3 * ((hash % 100) as f64 / 100.0);
                (star.brightness as f64 * flicker).min(255.0) as u8
            } else {
                star.brightness
            };

            let color = Color::from_rgba(star.color.r, star.color.g, star.color.b, alpha);

            if star.size <= 1.0 {
                fb.set_pixel_blended(sx as i32, sy as i32, color);
            } else {
                shapes::fill_circle(fb, sx, sy, star.size, color);
            }
        }
    }
}

fn simple_hash(v: u64) -> u64 {
    let mut x = v;
    x ^= x >> 16;
    x = x.wrapping_mul(0x45d9f3b);
    x ^= x >> 16;
    x
}
