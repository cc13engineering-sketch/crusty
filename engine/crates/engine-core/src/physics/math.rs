#![allow(dead_code)]

/// 2D vector. Type alias keeps things simple — destructures naturally, zero-cost.
pub type Vec2 = (f64, f64);

pub fn add(a: Vec2, b: Vec2) -> Vec2 { (a.0 + b.0, a.1 + b.1) }
pub fn sub(a: Vec2, b: Vec2) -> Vec2 { (a.0 - b.0, a.1 - b.1) }
pub fn scale(v: Vec2, s: f64) -> Vec2 { (v.0 * s, v.1 * s) }
pub fn dot(a: Vec2, b: Vec2) -> f64 { a.0 * b.0 + a.1 * b.1 }
pub fn length(v: Vec2) -> f64 { dot(v, v).sqrt() }
pub fn length_sq(v: Vec2) -> f64 { dot(v, v) }
pub fn distance(a: Vec2, b: Vec2) -> f64 { length(sub(a, b)) }
pub fn distance_sq(a: Vec2, b: Vec2) -> f64 { length_sq(sub(a, b)) }
pub fn normalize(v: Vec2) -> Vec2 {
    let len = length(v);
    if len < 1e-10 { (0.0, 0.0) } else { (v.0 / len, v.1 / len) }
}
pub fn reflect(v: Vec2, normal: Vec2) -> Vec2 {
    let d = 2.0 * dot(v, normal);
    (v.0 - d * normal.0, v.1 - d * normal.1)
}
pub fn perpendicular(v: Vec2) -> Vec2 { (-v.1, v.0) }
pub fn clamp_f64(val: f64, min: f64, max: f64) -> f64 { val.max(min).min(max) }
