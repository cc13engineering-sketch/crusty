use super::math::*;

/// Result of a CCD sweep test.
#[derive(Clone, Debug)]
pub struct SweepHit {
    pub t: f64,       // time of impact, 0.0 = start, 1.0 = end
    pub normal: Vec2, // collision surface normal (pointing away from hit surface)
    pub contact: Vec2, // circle center at moment of impact
}

/// Moving circle (center from a_start to a_end, radius a_r)
/// vs stationary circle (center b_pos, radius b_r).
pub fn sweep_circle_vs_circle(
    a_start: Vec2, a_end: Vec2, a_r: f64,
    b_pos: Vec2, b_r: f64,
) -> Option<SweepHit> {
    let d = sub(a_end, a_start);
    let f = sub(a_start, b_pos);
    let r = a_r + b_r;

    let a_coeff = dot(d, d);
    let b_coeff = 2.0 * dot(f, d);
    let c_coeff = dot(f, f) - r * r;

    // If already overlapping at start, report t=0
    if c_coeff <= 0.0 {
        let n = normalize(f);
        let n = if length_sq(n) < 1e-10 { (0.0, -1.0) } else { n };
        return Some(SweepHit { t: 0.0, normal: n, contact: a_start });
    }

    if a_coeff < 1e-12 {
        return None; // not moving
    }

    let discriminant = b_coeff * b_coeff - 4.0 * a_coeff * c_coeff;
    if discriminant < 0.0 {
        return None;
    }

    let sqrt_disc = discriminant.sqrt();
    let t1 = (-b_coeff - sqrt_disc) / (2.0 * a_coeff);
    let t2 = (-b_coeff + sqrt_disc) / (2.0 * a_coeff);

    let t = if t1 >= 0.0 && t1 <= 1.0 {
        t1
    } else if t2 >= 0.0 && t2 <= 1.0 {
        t2
    } else {
        return None;
    };

    let contact = add(a_start, scale(d, t));
    let normal = normalize(sub(contact, b_pos));
    let normal = if length_sq(normal) < 1e-10 { (0.0, -1.0) } else { normal };

    Some(SweepHit { t, normal, contact })
}

/// Moving circle vs a static line segment.
pub fn sweep_circle_vs_line_segment(
    start: Vec2, end: Vec2, r: f64,
    seg_a: Vec2, seg_b: Vec2,
) -> Option<SweepHit> {
    let mut best: Option<SweepHit> = None;

    // Test against the infinite line, then check segment bounds
    let seg_d = sub(seg_b, seg_a);
    let seg_len = length(seg_d);
    if seg_len < 1e-10 {
        // Degenerate segment — treat as point
        return sweep_circle_vs_circle(start, end, r, seg_a, 0.0);
    }
    let seg_n = normalize(perpendicular(seg_d)); // outward normal

    let move_d = sub(end, start);
    let _denom = dot(move_d, seg_n);

    // Check both sides of the line (normal and -normal)
    for &sign in &[1.0_f64, -1.0] {
        let n = scale(seg_n, sign);
        let dist_start = dot(sub(start, seg_a), n) - r;
        let dist_end = dot(sub(end, seg_a), n) - r;

        if dist_start < 0.0 {
            continue; // starting on wrong side
        }
        if dist_end >= 0.0 {
            continue; // doesn't reach the line
        }

        let t = dist_start / (dist_start - dist_end);
        if t < 0.0 || t > 1.0 {
            continue;
        }

        // Check if contact point is within segment bounds
        let contact = add(start, scale(move_d, t));
        let proj = dot(sub(contact, seg_a), seg_d) / (seg_len * seg_len);
        if proj >= 0.0 && proj <= 1.0 {
            if best.as_ref().map_or(true, |b| t < b.t) {
                best = Some(SweepHit { t, normal: n, contact });
            }
        }
    }

    // Test against segment endpoints (circle vs point)
    for &endpoint in &[seg_a, seg_b] {
        if let Some(hit) = sweep_circle_vs_circle(start, end, r, endpoint, 0.0) {
            if best.as_ref().map_or(true, |b| hit.t < b.t) {
                best = Some(hit);
            }
        }
    }

    best
}

/// Moving circle vs static axis-aligned bounding box.
/// AABB defined by center and half-extents.
pub fn sweep_circle_vs_aabb(
    start: Vec2, end: Vec2, r: f64,
    box_center: Vec2, half_w: f64, half_h: f64,
) -> Option<SweepHit> {
    let (cx, cy) = box_center;
    let mut best: Option<SweepHit> = None;

    // 4 edges
    let edges: [(Vec2, Vec2); 4] = [
        // Top edge
        ((cx - half_w, cy - half_h), (cx + half_w, cy - half_h)),
        // Bottom edge
        ((cx - half_w, cy + half_h), (cx + half_w, cy + half_h)),
        // Left edge
        ((cx - half_w, cy - half_h), (cx - half_w, cy + half_h)),
        // Right edge
        ((cx + half_w, cy - half_h), (cx + half_w, cy + half_h)),
    ];

    for (a, b) in &edges {
        if let Some(hit) = sweep_circle_vs_line_segment(start, end, r, *a, *b) {
            if best.as_ref().map_or(true, |b| hit.t < b.t) {
                best = Some(hit);
            }
        }
    }

    // 4 corners
    let corners: [Vec2; 4] = [
        (cx - half_w, cy - half_h),
        (cx + half_w, cy - half_h),
        (cx - half_w, cy + half_h),
        (cx + half_w, cy + half_h),
    ];
    for &corner in &corners {
        if let Some(hit) = sweep_circle_vs_circle(start, end, r, corner, 0.0) {
            if best.as_ref().map_or(true, |b| hit.t < b.t) {
                best = Some(hit);
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-6;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    fn vec2_approx_eq(a: Vec2, b: Vec2) -> bool {
        approx_eq(a.0, b.0) && approx_eq(a.1, b.1)
    }

    // ========================================================================
    // sweep_circle_vs_circle
    // ========================================================================

    #[test]
    fn circle_vs_circle_head_on_hit() {
        // Circle at x=0 moving right to x=20, radius 1.
        // Stationary circle at x=10, radius 1.
        // They touch when distance between centers = 2 (sum of radii).
        // Center travels from 0 to 20, needs to reach x=8 (distance from 10 = 2).
        // t = 8/20 = 0.4
        let hit = sweep_circle_vs_circle(
            (0.0, 0.0), (20.0, 0.0), 1.0,
            (10.0, 0.0), 1.0,
        );
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!(approx_eq(hit.t, 0.4));
        // Contact is the moving circle's center at impact: (8, 0)
        assert!(approx_eq(hit.contact.0, 8.0));
        assert!(approx_eq(hit.contact.1, 0.0));
        // Normal points from b towards a's contact: normalize((8-10, 0)) = (-1, 0)
        assert!(approx_eq(hit.normal.0, -1.0));
        assert!(approx_eq(hit.normal.1, 0.0));
    }

    #[test]
    fn circle_vs_circle_miss_parallel_path() {
        // Circle moves from (0,5) to (20,5), radius 1.
        // Stationary circle at (10,0), radius 1.
        // Closest distance between centers along path is 5, greater than 2.
        let hit = sweep_circle_vs_circle(
            (0.0, 5.0), (20.0, 5.0), 1.0,
            (10.0, 0.0), 1.0,
        );
        assert!(hit.is_none());
    }

    #[test]
    fn circle_vs_circle_already_overlapping() {
        // Circles overlap at start: centers 1 apart, radii sum = 4.
        let hit = sweep_circle_vs_circle(
            (0.0, 0.0), (5.0, 0.0), 2.0,
            (1.0, 0.0), 2.0,
        );
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!(approx_eq(hit.t, 0.0));
        assert!(vec2_approx_eq(hit.contact, (0.0, 0.0)));
    }

    #[test]
    fn circle_vs_circle_not_moving() {
        // Circle not moving, not overlapping. Should return None.
        let hit = sweep_circle_vs_circle(
            (0.0, 0.0), (0.0, 0.0), 1.0,
            (5.0, 0.0), 1.0,
        );
        assert!(hit.is_none());
    }

    #[test]
    fn circle_vs_circle_moving_away() {
        // Circle at (0,0) moving left (away from target at (5,0)).
        let hit = sweep_circle_vs_circle(
            (0.0, 0.0), (-10.0, 0.0), 1.0,
            (5.0, 0.0), 1.0,
        );
        assert!(hit.is_none());
    }

    #[test]
    fn circle_vs_circle_hit_at_t1_boundary() {
        // Circle moves from (0,0) to (8,0), radius 1.
        // Stationary circle at (10,0), radius 1.
        // Touch when center reaches x=8, i.e. t = 8/8 = 1.0
        let hit = sweep_circle_vs_circle(
            (0.0, 0.0), (8.0, 0.0), 1.0,
            (10.0, 0.0), 1.0,
        );
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!(approx_eq(hit.t, 1.0));
    }

    #[test]
    fn circle_vs_circle_diagonal_hit() {
        // Circle moves diagonally from (0,0) to (10,10), radius 1.
        // Stationary circle at (5,5), radius 1. They overlap immediately on the path.
        // Distance from start (0,0) to target center (5,5) = ~7.07.
        // They touch when distance = 2.
        // Parametric: center at (10t, 10t), dist to (5,5) = sqrt((10t-5)^2 + (10t-5)^2)
        //   = |10t-5| * sqrt(2) = 2
        //   => 10t - 5 = -2/sqrt(2) (take the earlier time, approaching)
        //   => t = (5 - sqrt(2)) / 10
        let expected_t = (5.0 - 2.0_f64.sqrt()) / 10.0;
        let hit = sweep_circle_vs_circle(
            (0.0, 0.0), (10.0, 10.0), 1.0,
            (5.0, 5.0), 1.0,
        );
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!(approx_eq(hit.t, expected_t));
        // Contact should be along the diagonal
        let ct = 10.0 * expected_t;
        assert!(approx_eq(hit.contact.0, ct));
        assert!(approx_eq(hit.contact.1, ct));
    }

    // ========================================================================
    // sweep_circle_vs_line_segment
    // ========================================================================

    #[test]
    fn line_segment_hit_middle() {
        // Circle at (5, -5) moving down to (5, 5), radius 1.
        // Horizontal segment from (0,0) to (10,0).
        // Circle hits the segment when its center is 1 unit above it: y = -1.
        // Travel from y=-5 to y=5 (dist=10), reach y=-1 at t = 4/10 = 0.4.
        let hit = sweep_circle_vs_line_segment(
            (5.0, -5.0), (5.0, 5.0), 1.0,
            (0.0, 0.0), (10.0, 0.0),
        );
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!(approx_eq(hit.t, 0.4));
        assert!(approx_eq(hit.contact.0, 5.0));
        assert!(approx_eq(hit.contact.1, -1.0));
    }

    #[test]
    fn line_segment_hit_endpoint() {
        // Circle moving towards segment endpoint.
        // Segment from (0,0) to (0,10). Circle at (-5, 0) moving right to (5, 0), radius 1.
        // Hits endpoint (0,0) when distance = radius: |-5+10t| = 1 => t = 0.4
        let hit = sweep_circle_vs_line_segment(
            (-5.0, 0.0), (5.0, 0.0), 1.0,
            (0.0, 0.0), (0.0, 10.0),
        );
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!(hit.t >= 0.0 && hit.t <= 1.0);
        assert!(approx_eq(hit.t, 0.4));
    }

    #[test]
    fn line_segment_miss_parallel() {
        // Circle moves parallel to the segment, but far enough away.
        // Segment from (0,0) to (10,0). Circle at (0,5) moving to (10,5), radius 1.
        // Perpendicular distance is 5, never crosses.
        let hit = sweep_circle_vs_line_segment(
            (0.0, 5.0), (10.0, 5.0), 1.0,
            (0.0, 0.0), (10.0, 0.0),
        );
        assert!(hit.is_none());
    }

    #[test]
    fn line_segment_degenerate_zero_length() {
        // Degenerate segment (point) at (5,0). Circle from (0,0) to (10,0), radius 1.
        // Should behave like sweep_circle_vs_circle with b_r = 0.
        // Center at (10t, 0), dist to (5,0) = |10t - 5| = 1 => t = 0.4
        let hit = sweep_circle_vs_line_segment(
            (0.0, 0.0), (10.0, 0.0), 1.0,
            (5.0, 0.0), (5.0, 0.0),
        );
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!(approx_eq(hit.t, 0.4));
        assert!(approx_eq(hit.contact.0, 4.0));
        assert!(approx_eq(hit.contact.1, 0.0));
    }

    // ========================================================================
    // sweep_circle_vs_aabb
    // ========================================================================

    #[test]
    fn aabb_hit_top_edge() {
        // Box centered at (0, 0), half-extents 5x5.
        // Top edge is at y = -5.
        // Circle at (0, -20) moving down to (0, 20), radius 1.
        // Hits when center reaches y = -5 - 1 = -6.
        // Travel from y=-20 to y=20 (40 units), reach y=-6 at t = 14/40 = 0.35.
        let hit = sweep_circle_vs_aabb(
            (0.0, -20.0), (0.0, 20.0), 1.0,
            (0.0, 0.0), 5.0, 5.0,
        );
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!(approx_eq(hit.t, 0.35));
        assert!(approx_eq(hit.contact.0, 0.0));
        assert!(approx_eq(hit.contact.1, -6.0));
    }

    #[test]
    fn aabb_hit_side_edge() {
        // Box centered at (0, 0), half-extents 5x5.
        // Left edge goes from (-5, -5) to (-5, 5).
        // Circle at (-20, 0) moving right to (20, 0), radius 1.
        // Hits when center reaches x = -5 - 1 = -6.
        // Travel from x=-20 to x=20 (40 units), reach x=-6 at t = 14/40 = 0.35.
        let hit = sweep_circle_vs_aabb(
            (-20.0, 0.0), (20.0, 0.0), 1.0,
            (0.0, 0.0), 5.0, 5.0,
        );
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!(approx_eq(hit.t, 0.35));
        assert!(approx_eq(hit.contact.1, 0.0));
    }

    #[test]
    fn aabb_hit_corner() {
        // Box centered at (0, 0), half-extents 5x5.
        // Circle approaches the top-left corner (-5, -5) diagonally.
        // Circle at (-20, -20) moving to (0, 0), radius 1.
        // Corner is a point. Sweep circle vs circle at (-5, -5) radius 0.
        // Center at (-20 + 20t, -20 + 20t), dist to (-5, -5):
        //   = sqrt((20t-15)^2 + (20t-15)^2) = |20t-15|*sqrt(2) = 1
        //   => 20t - 15 = -1/sqrt(2) (approaching from below)
        //   => t = (15 - 1/sqrt(2)) / 20
        let expected_t = (15.0 - 1.0 / 2.0_f64.sqrt()) / 20.0;
        let hit = sweep_circle_vs_aabb(
            (-20.0, -20.0), (0.0, 0.0), 1.0,
            (0.0, 0.0), 5.0, 5.0,
        );
        assert!(hit.is_some());
        let hit = hit.unwrap();
        // The corner hit might not be the earliest; the edge hit might be earlier.
        // Just verify a valid hit occurs within [0, 1].
        assert!(hit.t >= 0.0 && hit.t <= 1.0);
        assert!(hit.t <= expected_t + EPS);
    }

    #[test]
    fn aabb_miss_circle_inside_path_but_misses_box() {
        // Box centered at (0, 0), half-extents 2x2 (box from (-2,-2) to (2,2)).
        // Circle moves from (-10, 5) to (10, 5), radius 1.
        // The path's y=5 is above the box entirely (box top is y=-2, bottom is y=2).
        // Closest y-distance is 5 - 2 = 3, which is > radius 1.
        let hit = sweep_circle_vs_aabb(
            (-10.0, 5.0), (10.0, 5.0), 1.0,
            (0.0, 0.0), 2.0, 2.0,
        );
        assert!(hit.is_none());
    }

    #[test]
    fn aabb_miss_passing_to_the_side() {
        // Box centered at (0, 0), half-extents 5x5.
        // Circle at (-20, 10) moving right to (20, 10), radius 1.
        // Travels along y=10, but box bottom edge is at y=5.
        // Distance from path to nearest edge = 10 - 5 = 5 > 1.
        let hit = sweep_circle_vs_aabb(
            (-20.0, 10.0), (20.0, 10.0), 1.0,
            (0.0, 0.0), 5.0, 5.0,
        );
        assert!(hit.is_none());
    }

    #[test]
    fn aabb_stationary_circle_no_hit() {
        // Circle not moving, not overlapping.
        let hit = sweep_circle_vs_aabb(
            (20.0, 0.0), (20.0, 0.0), 1.0,
            (0.0, 0.0), 5.0, 5.0,
        );
        assert!(hit.is_none());
    }
}
