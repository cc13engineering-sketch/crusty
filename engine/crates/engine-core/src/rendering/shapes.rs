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
    // Ensure at least center pixel is drawn for tiny radii
    if radius < 1.0 {
        draw_px(fb, cx.round() as i32, cy.round() as i32, color);
        return;
    }
    let r2 = radius * radius;
    let x0 = (cx - radius).floor() as i32;
    let y0 = (cy - radius).floor() as i32;
    let x1 = (cx + radius).ceil() as i32;
    let y1 = (cy + radius).ceil() as i32;
    for py in y0..=y1 {
        let dy = py as f64 + 0.5 - cy;
        for px in x0..=x1 {
            let dx = px as f64 + 0.5 - cx;
            if dx * dx + dy * dy <= r2 {
                draw_px(fb, px, py, color);
            }
        }
    }
}

pub fn draw_circle(fb: &mut Framebuffer, cx: f64, cy: f64, radius: f64, color: Color) {
    if radius <= 0.0 {
        return;
    }
    let r = radius;
    let x0 = (cx - r - 1.0).floor() as i32;
    let y0 = (cy - r - 1.0).floor() as i32;
    let x1 = (cx + r + 1.0).ceil() as i32;
    let y1 = (cy + r + 1.0).ceil() as i32;
    for py in y0..=y1 {
        let dy = py as f64 + 0.5 - cy;
        for px in x0..=x1 {
            let dx = px as f64 + 0.5 - cx;
            let dist = (dx * dx + dy * dy).sqrt();
            if (dist - r).abs() < 0.7 {
                draw_px(fb, px, py, color);
            }
        }
    }
}

pub fn draw_line(fb: &mut Framebuffer, x0: f64, y0: f64, x1: f64, y1: f64, color: Color) {
    // Bresenham's line algorithm
    let mut sx = x0.round() as i32;
    let mut sy = y0.round() as i32;
    let ex = x1.round() as i32;
    let ey = y1.round() as i32;
    let dx = (ex - sx).abs();
    let dy = -(ey - sy).abs();
    let step_x: i32 = if sx < ex { 1 } else { -1 };
    let step_y: i32 = if sy < ey { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        draw_px(fb, sx, sy, color);
        if sx == ex && sy == ey {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            sx += step_x;
        }
        if e2 <= dx {
            err += dx;
            sy += step_y;
        }
    }
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
    let bx0 = x0.min(x1) - half_t;
    let by0 = y0.min(y1) - half_t;
    let bx1 = x0.max(x1) + half_t;
    let by1 = y0.max(y1) + half_t;
    for py in (by0.floor() as i32)..=(by1.ceil() as i32) {
        for px in (bx0.floor() as i32)..=(bx1.ceil() as i32) {
            let px_f = px as f64 + 0.5;
            let py_f = py as f64 + 0.5;
            // Perpendicular distance to line segment
            let t = ((px_f - x0) * dx + (py_f - y0) * dy) / (len * len);
            let t_clamped = t.max(0.0).min(1.0);
            let closest_x = x0 + t_clamped * dx;
            let closest_y = y0 + t_clamped * dy;
            let dist_sq =
                (px_f - closest_x) * (px_f - closest_x) + (py_f - closest_y) * (py_f - closest_y);
            if dist_sq <= half_t * half_t {
                draw_px(fb, px, py, color);
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
