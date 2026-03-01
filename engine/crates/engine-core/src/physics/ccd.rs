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
