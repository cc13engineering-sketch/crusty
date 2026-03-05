use super::color::Color;
use super::framebuffer::Framebuffer;

fn draw_px(fb: &mut Framebuffer, x: i32, y: i32, color: Color) {
    if color.a == 255 {
        fb.set_pixel(x, y, color);
    } else if color.a > 0 {
        fb.set_pixel_blended(x, y, color);
    }
}

pub fn fill_rect(fb: &mut Framebuffer, x: f64, y: f64, w: f64, h: f64, color: Color) {
    let x0 = x.round() as i32;
    let y0 = y.round() as i32;
    let x1 = (x + w).round() as i32;
    let y1 = (y + h).round() as i32;
    for py in y0..y1 {
        for px in x0..x1 {
            draw_px(fb, px, py, color);
        }
    }
}

pub fn draw_rect(fb: &mut Framebuffer, x: f64, y: f64, w: f64, h: f64, color: Color) {
    let x0 = x.round() as i32;
    let y0 = y.round() as i32;
    let x1 = (x + w).round() as i32 - 1;
    let y1 = (y + h).round() as i32 - 1;
    for px in x0..=x1 {
        draw_px(fb, px, y0, color);
        draw_px(fb, px, y1, color);
    }
    for py in y0..=y1 {
        draw_px(fb, x0, py, color);
        draw_px(fb, x1, py, color);
    }
}

pub fn fill_circle(fb: &mut Framebuffer, cx: f64, cy: f64, radius: f64, color: Color) {
    if radius <= 0.0 {
        return;
    }
    if radius < 1.0 {
        draw_px(fb, cx.round() as i32, cy.round() as i32, color);
        return;
    }
    let feather = 1.0;
    let outer = radius + feather;
    let x0 = (cx - outer).floor() as i32;
    let y0 = (cy - outer).floor() as i32;
    let x1 = (cx + outer).ceil() as i32;
    let y1 = (cy + outer).ceil() as i32;
    for py in y0..=y1 {
        let dy = py as f64 + 0.5 - cy;
        for px in x0..=x1 {
            let dx = px as f64 + 0.5 - cx;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist <= radius {
                draw_px(fb, px, py, color);
            } else if dist < outer {
                let t = 1.0 - (dist - radius) / feather;
                let aa_alpha = (color.a as f64 * t).round() as u8;
                if aa_alpha > 0 {
                    draw_px(fb, px, py, color.with_alpha(aa_alpha));
                }
            }
        }
    }
}

pub fn draw_circle(fb: &mut Framebuffer, cx: f64, cy: f64, radius: f64, color: Color) {
    if radius <= 0.0 {
        return;
    }
    let r = radius;
    let half_w = 0.5; // ring half-thickness
    let feather = 1.0;
    let outer = r + half_w + feather;
    let inner = (r - half_w - feather).max(0.0);
    let x0 = (cx - outer).floor() as i32;
    let y0 = (cy - outer).floor() as i32;
    let x1 = (cx + outer).ceil() as i32;
    let y1 = (cy + outer).ceil() as i32;
    for py in y0..=y1 {
        let dy = py as f64 + 0.5 - cy;
        for px in x0..=x1 {
            let dx = px as f64 + 0.5 - cx;
            let dist = (dx * dx + dy * dy).sqrt();
            let d = (dist - r).abs(); // distance from ring center
            if d <= half_w {
                draw_px(fb, px, py, color);
            } else if d < half_w + feather {
                let t = 1.0 - (d - half_w) / feather;
                let aa_alpha = (color.a as f64 * t).round() as u8;
                if aa_alpha > 0 {
                    draw_px(fb, px, py, color.with_alpha(aa_alpha));
                }
            }
        }
    }
    let _ = inner; // suppress unused
}

pub fn draw_line(fb: &mut Framebuffer, x0: f64, y0: f64, x1: f64, y1: f64, color: Color) {
    draw_line_thick(fb, x0, y0, x1, y1, 1.0, color);
}

pub fn draw_line_thick(
    fb: &mut Framebuffer,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    thickness: f64,
    color: Color,
) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.001 {
        fill_circle(fb, x0, y0, thickness / 2.0, color);
        return;
    }
    let half_t = thickness / 2.0;
    let feather = 1.0;
    let pad = half_t + feather;
    let bx0 = x0.min(x1) - pad;
    let by0 = y0.min(y1) - pad;
    let bx1 = x0.max(x1) + pad;
    let by1 = y0.max(y1) + pad;
    let inv_len_sq = 1.0 / (len * len);
    for py in (by0.floor() as i32)..=(by1.ceil() as i32) {
        let py_f = py as f64 + 0.5;
        for px in (bx0.floor() as i32)..=(bx1.ceil() as i32) {
            let px_f = px as f64 + 0.5;
            let t = ((px_f - x0) * dx + (py_f - y0) * dy) * inv_len_sq;
            let t_clamped = t.max(0.0).min(1.0);
            let closest_x = x0 + t_clamped * dx;
            let closest_y = y0 + t_clamped * dy;
            let ddx = px_f - closest_x;
            let ddy = py_f - closest_y;
            let dist = (ddx * ddx + ddy * ddy).sqrt();
            if dist <= half_t {
                draw_px(fb, px, py, color);
            } else if dist < half_t + feather {
                let edge = 1.0 - (dist - half_t) / feather;
                let aa_alpha = (color.a as f64 * edge).round() as u8;
                if aa_alpha > 0 {
                    draw_px(fb, px, py, color.with_alpha(aa_alpha));
                }
            }
        }
    }
}

pub fn draw_dashed_circle(
    fb: &mut Framebuffer,
    cx: f64,
    cy: f64,
    radius: f64,
    color: Color,
    dash_len: f64,
) {
    if radius <= 0.0 {
        return;
    }
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let step = 1.0 / radius; // angular step ≈ 1 pixel
    let mut angle = 0.0_f64;
    let mut arc = 0.0_f64;
    let mut drawing = true;
    while angle < 2.0 * std::f64::consts::PI {
        if drawing {
            let px = cx + angle.cos() * radius;
            let py = cy + angle.sin() * radius;
            draw_px(fb, px.round() as i32, py.round() as i32, color);
        }
        angle += step;
        arc += radius * step;
        if arc > dash_len {
            arc = 0.0;
            drawing = !drawing;
        }
    }
    let _ = circumference; // suppress unused warning
}

/// Fill a pill / stadium shape (rounded rectangle with semicircle caps).
/// Anti-aliased edges via signed-distance-field, same approach as `fill_circle`.
pub fn fill_pill(fb: &mut Framebuffer, x: f64, y: f64, w: f64, h: f64, color: Color) {
    if w <= 0.0 || h <= 0.0 {
        return;
    }
    let r = h / 2.0;
    let feather = 1.0;
    let outer = r + feather;
    let x0 = (x - feather).floor() as i32;
    let y0 = (y - feather).floor() as i32;
    let x1 = (x + w + feather).ceil() as i32;
    let y1 = (y + h + feather).ceil() as i32;
    let cy = y + r;
    let left_cx = x + r;
    let right_cx = x + w - r;
    for py in y0..=y1 {
        let dy = py as f64 + 0.5 - cy;
        for px in x0..=x1 {
            let pxf = px as f64 + 0.5;
            // SDF for stadium: clamp x to the straight segment, then circle distance
            let nearest_x = pxf.max(left_cx).min(right_cx);
            let dx = pxf - nearest_x;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist <= r {
                draw_px(fb, px, py, color);
            } else if dist < outer {
                let t = 1.0 - (dist - r) / feather;
                let aa_alpha = (color.a as f64 * t).round() as u8;
                if aa_alpha > 0 {
                    draw_px(fb, px, py, color.with_alpha(aa_alpha));
                }
            }
        }
    }
}

/// Fill a triangle defined by three vertices with a solid color (AA edges).
pub fn fill_triangle(
    fb: &mut Framebuffer,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    color: Color,
) {
    let min_x = x0.min(x1).min(x2).floor() as i32;
    let max_x = x0.max(x1).max(x2).ceil() as i32;
    let min_y = y0.min(y1).min(y2).floor() as i32;
    let max_y = y0.max(y1).max(y2).ceil() as i32;

    #[inline]
    fn edge(ax: f64, ay: f64, bx: f64, by: f64, px: f64, py: f64) -> f64 {
        (bx - ax) * (py - ay) - (by - ay) * (px - ax)
    }

    let area = edge(x0, y0, x1, y1, x2, y2);
    if area.abs() < 0.001 {
        return;
    }
    let sign = area.signum();

    for py in min_y..=max_y {
        for px in min_x..=max_x {
            let pxf = px as f64 + 0.5;
            let pyf = py as f64 + 0.5;
            let e0 = edge(x0, y0, x1, y1, pxf, pyf) * sign;
            let e1 = edge(x1, y1, x2, y2, pxf, pyf) * sign;
            let e2 = edge(x2, y2, x0, y0, pxf, pyf) * sign;
            if e0 >= -0.5 && e1 >= -0.5 && e2 >= -0.5 {
                let min_e = e0.min(e1).min(e2);
                if min_e >= 0.5 {
                    draw_px(fb, px, py, color);
                } else {
                    let t = (min_e + 0.5).max(0.0);
                    let aa_alpha = (color.a as f64 * t).round() as u8;
                    if aa_alpha > 0 {
                        draw_px(fb, px, py, color.with_alpha(aa_alpha));
                    }
                }
            }
        }
    }
}

/// Draw a smooth anti-aliased tapered trail along an entire polyline in one
/// pass. Each pixel is evaluated against ALL segments and only the best
/// (closest) match determines its color/alpha — no seams, no double-blending.
///
/// * `points` — screen-space (x,y) positions along the trail
/// * `widths` — half-width at each point (tapers to 0 at the tail)
/// * `alphas` — alpha (0.0–255.0) at each point
/// * `color`  — base RGB color
pub fn fill_tapered_trail(
    fb: &mut Framebuffer,
    points: &[(f64, f64)],
    widths: &[f64],
    alphas: &[f64],
    color: Color,
) {
    let n = points.len();
    if n < 2 {
        return;
    }

    // Precompute per-segment data
    struct Seg {
        x0: f64, y0: f64,
        dx: f64, dy: f64, inv_len_sq: f64,
        r0: f64, r1: f64, a0: f64, a1: f64,
    }
    let feather = 1.0;
    let mut segs: Vec<Seg> = Vec::with_capacity(n - 1);
    let mut bbox_x0 = f64::MAX;
    let mut bbox_y0 = f64::MAX;
    let mut bbox_x1 = f64::MIN;
    let mut bbox_y1 = f64::MIN;

    for i in 0..n - 1 {
        let (x0, y0) = points[i];
        let (x1, y1) = points[i + 1];
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len_sq = dx * dx + dy * dy;
        if len_sq < 0.01 {
            continue;
        }
        let max_r = widths[i].max(widths[i + 1]);
        let pad = max_r + feather + 1.0;
        bbox_x0 = bbox_x0.min(x0.min(x1) - pad);
        bbox_y0 = bbox_y0.min(y0.min(y1) - pad);
        bbox_x1 = bbox_x1.max(x0.max(x1) + pad);
        bbox_y1 = bbox_y1.max(y0.max(y1) + pad);
        segs.push(Seg {
            x0, y0, dx, dy,
            inv_len_sq: 1.0 / len_sq,
            r0: widths[i], r1: widths[i + 1],
            a0: alphas[i], a1: alphas[i + 1],
        });
    }
    if segs.is_empty() {
        return;
    }

    for py in (bbox_y0.floor() as i32)..=(bbox_y1.ceil() as i32) {
        let pyf = py as f64 + 0.5;
        for px in (bbox_x0.floor() as i32)..=(bbox_x1.ceil() as i32) {
            let pxf = px as f64 + 0.5;

            // Find the segment where this pixel is most "inside"
            // (greatest ratio of radius to distance = best coverage)
            let mut best_coverage: f64 = 0.0; // 1.0 = fully inside, 0..1 = feather
            let mut best_alpha: f64 = 0.0;

            for s in &segs {
                let t = ((pxf - s.x0) * s.dx + (pyf - s.y0) * s.dy) * s.inv_len_sq;
                let tc = t.max(0.0).min(1.0);
                let cx = s.x0 + tc * s.dx;
                let cy = s.y0 + tc * s.dy;
                let ddx = pxf - cx;
                let ddy = pyf - cy;
                let dist = (ddx * ddx + ddy * ddy).sqrt();
                let r = s.r0 + (s.r1 - s.r0) * tc;
                let a = s.a0 + (s.a1 - s.a0) * tc;

                if dist <= r {
                    // Fully inside this segment
                    if a > best_alpha || (a == best_alpha && 1.0 > best_coverage) {
                        best_coverage = 1.0;
                        best_alpha = a;
                    }
                } else if dist < r + feather {
                    let edge_t = 1.0 - (dist - r) / feather;
                    let coverage = edge_t;
                    let effective_a = a * coverage;
                    if effective_a > best_alpha * best_coverage {
                        best_coverage = coverage;
                        best_alpha = a;
                    }
                }
            }

            if best_coverage > 0.0 {
                let final_alpha = (best_alpha * best_coverage).round() as u8;
                if final_alpha > 0 {
                    draw_px(fb, px, py, color.with_alpha(final_alpha));
                }
            }
        }
    }
}
