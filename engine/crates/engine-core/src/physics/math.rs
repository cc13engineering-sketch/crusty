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

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    fn vec2_approx_eq(a: Vec2, b: Vec2) -> bool {
        approx_eq(a.0, b.0) && approx_eq(a.1, b.1)
    }

    // ---- add ----

    #[test]
    fn test_add_basic() {
        let result = add((1.0, 2.0), (3.0, 4.0));
        assert!(vec2_approx_eq(result, (4.0, 6.0)));
    }

    #[test]
    fn test_add_negative_numbers() {
        let result = add((-1.0, -2.0), (-3.0, -4.0));
        assert!(vec2_approx_eq(result, (-4.0, -6.0)));
    }

    #[test]
    fn test_add_zeros() {
        let result = add((0.0, 0.0), (0.0, 0.0));
        assert!(vec2_approx_eq(result, (0.0, 0.0)));

        let result2 = add((5.0, -3.0), (0.0, 0.0));
        assert!(vec2_approx_eq(result2, (5.0, -3.0)));
    }

    // ---- sub ----

    #[test]
    fn test_sub_basic() {
        let result = sub((5.0, 7.0), (2.0, 3.0));
        assert!(vec2_approx_eq(result, (3.0, 4.0)));
    }

    #[test]
    fn test_sub_self_subtraction_is_zero() {
        let v = (42.0, -17.5);
        let result = sub(v, v);
        assert!(vec2_approx_eq(result, (0.0, 0.0)));
    }

    // ---- scale ----

    #[test]
    fn test_scale_positive() {
        let result = scale((2.0, 3.0), 4.0);
        assert!(vec2_approx_eq(result, (8.0, 12.0)));
    }

    #[test]
    fn test_scale_negative() {
        let result = scale((2.0, 3.0), -1.0);
        assert!(vec2_approx_eq(result, (-2.0, -3.0)));
    }

    #[test]
    fn test_scale_zero_multiplier() {
        let result = scale((100.0, -200.0), 0.0);
        assert!(vec2_approx_eq(result, (0.0, 0.0)));
    }

    // ---- dot ----

    #[test]
    fn test_dot_perpendicular_vectors_is_zero() {
        let a = (1.0, 0.0);
        let b = (0.0, 1.0);
        assert!(approx_eq(dot(a, b), 0.0));

        // Another perpendicular pair
        let c = (3.0, 4.0);
        let d = (-4.0, 3.0);
        assert!(approx_eq(dot(c, d), 0.0));
    }

    #[test]
    fn test_dot_parallel_vectors_product_of_lengths() {
        let a = (3.0, 0.0);
        let b = (5.0, 0.0);
        assert!(approx_eq(dot(a, b), 15.0));

        // Same direction, different magnitudes
        let c = (1.0, 1.0);
        let d = (2.0, 2.0);
        assert!(approx_eq(dot(c, d), 4.0));
    }

    // ---- length ----

    #[test]
    fn test_length_3_4_5_triangle() {
        assert!(approx_eq(length((3.0, 4.0)), 5.0));
    }

    #[test]
    fn test_length_zero_vector() {
        assert!(approx_eq(length((0.0, 0.0)), 0.0));
    }

    // ---- length_sq ----

    #[test]
    fn test_length_sq_3_4() {
        assert!(approx_eq(length_sq((3.0, 4.0)), 25.0));
    }

    // ---- distance ----

    #[test]
    fn test_distance_between_two_points() {
        let a = (1.0, 1.0);
        let b = (4.0, 5.0);
        // distance = sqrt(9 + 16) = 5
        assert!(approx_eq(distance(a, b), 5.0));
    }

    #[test]
    fn test_distance_same_point_is_zero() {
        let a = (7.0, -3.0);
        assert!(approx_eq(distance(a, a), 0.0));
    }

    // ---- distance_sq ----

    #[test]
    fn test_distance_sq_between_two_points() {
        let a = (1.0, 1.0);
        let b = (4.0, 5.0);
        assert!(approx_eq(distance_sq(a, b), 25.0));
    }

    // ---- normalize ----

    #[test]
    fn test_normalize_produces_unit_vector() {
        let v = (3.0, 4.0);
        let n = normalize(v);
        assert!(approx_eq(length(n), 1.0));
        assert!(approx_eq(n.0, 3.0 / 5.0));
        assert!(approx_eq(n.1, 4.0 / 5.0));
    }

    #[test]
    fn test_normalize_axis_aligned() {
        let n = normalize((0.0, 7.0));
        assert!(vec2_approx_eq(n, (0.0, 1.0)));

        let n2 = normalize((5.0, 0.0));
        assert!(vec2_approx_eq(n2, (1.0, 0.0)));
    }

    #[test]
    fn test_normalize_zero_vector_returns_zero() {
        let n = normalize((0.0, 0.0));
        assert!(vec2_approx_eq(n, (0.0, 0.0)));
    }

    // ---- reflect ----

    #[test]
    fn test_reflect_off_horizontal_surface() {
        // Ball moving down-right, reflecting off floor (normal pointing up)
        let v = (1.0, -1.0);
        let normal = (0.0, 1.0);
        let r = reflect(v, normal);
        // reflect formula: v - 2*dot(v,n)*n
        // dot = -1, so r = (1, -1) - 2*(-1)*(0,1) = (1, -1 + 2) = (1, 1)
        assert!(vec2_approx_eq(r, (1.0, 1.0)));
    }

    #[test]
    fn test_reflect_off_vertical_surface() {
        // Ball moving right, reflecting off right wall (normal pointing left)
        let v = (1.0, 0.0);
        let normal = (-1.0, 0.0);
        let r = reflect(v, normal);
        // dot = -1, r = (1,0) - 2*(-1)*(-1,0) = (1-2, 0) = (-1, 0)
        assert!(vec2_approx_eq(r, (-1.0, 0.0)));
    }

    #[test]
    fn test_reflect_off_diagonal_surface() {
        // Moving straight down, reflecting off a 45-degree surface
        let v = (0.0, -1.0);
        let normal = normalize((1.0, 1.0));
        let r = reflect(v, normal);
        // dot(v,n) = -1/sqrt(2)
        // r = (0, -1) - 2 * (-1/sqrt(2)) * (1/sqrt(2), 1/sqrt(2))
        //   = (0, -1) + (1, 1) = (1, 0)
        assert!(vec2_approx_eq(r, (1.0, 0.0)));
    }

    // ---- perpendicular ----

    #[test]
    fn test_perpendicular_x_axis() {
        let p = perpendicular((1.0, 0.0));
        assert!(vec2_approx_eq(p, (0.0, 1.0)));
    }

    #[test]
    fn test_perpendicular_y_axis() {
        let p = perpendicular((0.0, 1.0));
        assert!(vec2_approx_eq(p, (-1.0, 0.0)));
    }

    #[test]
    fn test_perpendicular_is_perpendicular() {
        let v = (3.0, 7.0);
        let p = perpendicular(v);
        assert!(approx_eq(dot(v, p), 0.0));
    }

    // ---- clamp_f64 ----

    #[test]
    fn test_clamp_below_min() {
        assert!(approx_eq(clamp_f64(-5.0, 0.0, 10.0), 0.0));
    }

    #[test]
    fn test_clamp_above_max() {
        assert!(approx_eq(clamp_f64(15.0, 0.0, 10.0), 10.0));
    }

    #[test]
    fn test_clamp_in_range() {
        assert!(approx_eq(clamp_f64(5.0, 0.0, 10.0), 5.0));
    }

    #[test]
    fn test_clamp_at_boundaries() {
        assert!(approx_eq(clamp_f64(0.0, 0.0, 10.0), 0.0));
        assert!(approx_eq(clamp_f64(10.0, 0.0, 10.0), 10.0));
    }
}
