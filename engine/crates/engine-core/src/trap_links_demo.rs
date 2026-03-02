/// DEMO MODULE: S-League — Minigolf RPG (tiny slice)
/// Shows a single fight scene: a minigolf hole where you aim and shoot a ball.
/// Demonstrates: TileMap, AimPreview, physics ball movement, collision with walls,
/// hole target, stroke counter, HUD. Mobile-first 480x720 portrait.

use crate::engine::Engine;
use crate::tilemap::{TileMap, Tile};
use crate::rendering::color::Color;
use crate::rendering::shapes;
use crate::rendering::text;
use crate::aim_preview::{AimConfig, compute_arc};

// ─── Constants ──────────────────────────────────────────────────────────

const TILE_SIZE: f64 = 16.0;
const COURSE_W: usize = 30; // tiles
const COURSE_H: usize = 37; // tiles (leaves room for HUD)
const _WORLD_W: f64 = COURSE_W as f64 * TILE_SIZE; // 480
const WORLD_H: f64 = COURSE_H as f64 * TILE_SIZE; // 592

const WIDTH: f64 = 480.0;
const HEIGHT: f64 = 720.0;
const HUD_H: f64 = HEIGHT - WORLD_H; // 128px HUD at bottom

// Colors — earthy meadow palette
const COL_BG: Color = Color { r: 34, g: 51, b: 34, a: 255 };
const COL_FAIRWAY: Color = Color { r: 58, g: 94, b: 48, a: 255 };
const COL_WALL: Color = Color { r: 72, g: 56, b: 40, a: 255 };
const COL_WALL_LIGHT: Color = Color { r: 88, g: 68, b: 48, a: 255 };
const COL_BALL: Color = Color { r: 245, g: 245, b: 240, a: 255 };
const COL_BALL_SHADOW: Color = Color { r: 30, g: 46, b: 30, a: 120 };
const COL_HOLE: Color = Color { r: 20, g: 20, b: 20, a: 255 };
const COL_HOLE_RIM: Color = Color { r: 45, g: 35, b: 25, a: 255 };
const COL_AIM_DOT: Color = Color { r: 255, g: 255, b: 200, a: 160 };
const COL_POWER_BAR: Color = Color { r: 255, g: 200, b: 60, a: 255 };
const COL_POWER_BG: Color = Color { r: 40, g: 40, b: 40, a: 200 };
const COL_HUD_BG: Color = Color { r: 28, g: 28, b: 36, a: 255 };
const COL_TEXT: Color = Color { r: 230, g: 230, b: 220, a: 255 };
const COL_TEXT_DIM: Color = Color { r: 140, g: 140, b: 130, a: 255 };
const COL_SAND: Color = Color { r: 194, g: 170, b: 120, a: 255 };
const COL_SUCCESS: Color = Color { r: 80, g: 220, b: 100, a: 255 };
const COL_FLAG: Color = Color { r: 220, g: 50, b: 50, a: 255 };

// Physics
const BALL_RADIUS: f64 = 5.0;
const HOLE_RADIUS: f64 = 8.0;
const MAX_POWER: f64 = 400.0;
const DRAG: f64 = 1.8;
const RESTITUTION: f64 = 0.65;
const STOP_THRESHOLD: f64 = 8.0;
const PAR: i32 = 3;

// ─── Game State Keys ────────────────────────────────────────────────────

const K_BALL_X: &str = "ball_x";
const K_BALL_Y: &str = "ball_y";
const K_BALL_VX: &str = "ball_vx";
const K_BALL_VY: &str = "ball_vy";
const K_STROKES: &str = "strokes";
const K_PHASE: &str = "tl_phase"; // 0=aiming, 1=ball moving, 2=sunk, 3=fail
const K_AIM_X: &str = "aim_x";
const K_AIM_Y: &str = "aim_y";
const K_AIM_ACTIVE: &str = "aim_active";
const K_RESULT_TIMER: &str = "result_timer";
const K_HOLE_X: &str = "hole_x";
const K_HOLE_Y: &str = "hole_y";
const K_START_X: &str = "start_x";
const K_START_Y: &str = "start_y";

// ─── Setup ──────────────────────────────────────────────────────────────

pub fn setup(engine: &mut Engine) {
    engine.debug_mode = false;
    engine.config.name = "S-League".into();
    engine.config.bounds = (WIDTH, HEIGHT);
    engine.config.background = COL_BG;

    // Build the course
    let mut tilemap = TileMap::new(COURSE_W, COURSE_H, TILE_SIZE);

    // Fill everything with fairway
    for y in 0..COURSE_H {
        for x in 0..COURSE_W {
            tilemap.set(x, y, Tile::custom(0, COL_FAIRWAY));
        }
    }

    // Border walls
    for x in 0..COURSE_W {
        tilemap.set(x, 0, Tile::solid(COL_WALL));
        tilemap.set(x, 1, Tile::solid(COL_WALL));
        tilemap.set(x, COURSE_H - 1, Tile::solid(COL_WALL));
        tilemap.set(x, COURSE_H - 2, Tile::solid(COL_WALL));
    }
    for y in 0..COURSE_H {
        tilemap.set(0, y, Tile::solid(COL_WALL));
        tilemap.set(1, y, Tile::solid(COL_WALL));
        tilemap.set(COURSE_W - 1, y, Tile::solid(COL_WALL));
        tilemap.set(COURSE_W - 2, y, Tile::solid(COL_WALL));
    }

    // Interior obstacles — an L-shaped wall
    for x in 8..16 {
        tilemap.set(x, 12, Tile::solid(COL_WALL_LIGHT));
        tilemap.set(x, 13, Tile::solid(COL_WALL_LIGHT));
    }
    for y in 12..22 {
        tilemap.set(14, y, Tile::solid(COL_WALL_LIGHT));
        tilemap.set(15, y, Tile::solid(COL_WALL_LIGHT));
    }

    // A small wall on the right
    for y in 18..26 {
        tilemap.set(22, y, Tile::solid(COL_WALL_LIGHT));
        tilemap.set(23, y, Tile::solid(COL_WALL_LIGHT));
    }

    // Sand traps (visual — slow the ball via drag)
    for y in 24..28 {
        for x in 6..12 {
            tilemap.set(x, y, Tile::custom(1, COL_SAND));
        }
    }

    engine.tilemap = Some(tilemap);

    // Ball start position (bottom area)
    let start_x = 15.0 * TILE_SIZE;
    let start_y = 32.0 * TILE_SIZE;

    // Hole position (upper area)
    let hole_x = 20.0 * TILE_SIZE;
    let hole_y = 7.0 * TILE_SIZE;

    engine.global_state.set_f64(K_BALL_X, start_x);
    engine.global_state.set_f64(K_BALL_Y, start_y);
    engine.global_state.set_f64(K_BALL_VX, 0.0);
    engine.global_state.set_f64(K_BALL_VY, 0.0);
    engine.global_state.set_f64(K_STROKES, 0.0);
    engine.global_state.set_f64(K_PHASE, 0.0);
    engine.global_state.set_f64(K_AIM_X, 0.0);
    engine.global_state.set_f64(K_AIM_Y, 0.0);
    engine.global_state.set_f64(K_AIM_ACTIVE, 0.0);
    engine.global_state.set_f64(K_RESULT_TIMER, 0.0);
    engine.global_state.set_f64(K_HOLE_X, hole_x);
    engine.global_state.set_f64(K_HOLE_Y, hole_y);
    engine.global_state.set_f64(K_START_X, start_x);
    engine.global_state.set_f64(K_START_Y, start_y);

    // Camera centered on course
    engine.camera.x = 0.0;
    engine.camera.y = 0.0;
    engine.camera.zoom = 1.0;

    // Subtle vignette
    engine.post_fx.vignette_strength = 0.3;
}

// ─── Input Handling ─────────────────────────────────────────────────────

pub fn on_pointer_down(engine: &mut Engine, x: f64, y: f64) {
    let phase = engine.global_state.get_f64(K_PHASE).unwrap_or(0.0);
    if phase == 2.0 || phase == 3.0 {
        // Tap to restart after result
        let timer = engine.global_state.get_f64(K_RESULT_TIMER).unwrap_or(0.0);
        if timer > 1.0 {
            setup(engine);
        }
        return;
    }
    if phase != 0.0 { return; } // only aim when in aiming phase

    engine.global_state.set_f64(K_AIM_ACTIVE, 1.0);
    engine.global_state.set_f64(K_AIM_X, x);
    engine.global_state.set_f64(K_AIM_Y, y);
}

pub fn on_pointer_move(engine: &mut Engine, x: f64, y: f64) {
    let active = engine.global_state.get_f64(K_AIM_ACTIVE).unwrap_or(0.0);
    if active > 0.5 {
        engine.global_state.set_f64(K_AIM_X, x);
        engine.global_state.set_f64(K_AIM_Y, y);
    }
}

pub fn on_pointer_up(engine: &mut Engine, x: f64, y: f64) {
    let active = engine.global_state.get_f64(K_AIM_ACTIVE).unwrap_or(0.0);
    if active < 0.5 { return; }

    engine.global_state.set_f64(K_AIM_ACTIVE, 0.0);

    let phase = engine.global_state.get_f64(K_PHASE).unwrap_or(0.0);
    if phase != 0.0 { return; }

    let ball_x = engine.global_state.get_f64(K_BALL_X).unwrap_or(0.0);
    let ball_y = engine.global_state.get_f64(K_BALL_Y).unwrap_or(0.0);

    // Slingshot: drag away from ball, velocity goes opposite direction
    let dx = ball_x - x;
    let dy = ball_y - y;
    let dist = (dx * dx + dy * dy).sqrt();

    if dist < 5.0 { return; } // too small, ignore

    let power = (dist / 120.0).min(1.0) * MAX_POWER;
    let angle = dy.atan2(dx);

    let vx = angle.cos() * power;
    let vy = angle.sin() * power;

    engine.global_state.set_f64(K_BALL_VX, vx);
    engine.global_state.set_f64(K_BALL_VY, vy);
    engine.global_state.set_f64(K_PHASE, 1.0);

    let strokes = engine.global_state.get_f64(K_STROKES).unwrap_or(0.0);
    engine.global_state.set_f64(K_STROKES, strokes + 1.0);
}

// ─── Update ─────────────────────────────────────────────────────────────

pub fn update(engine: &mut Engine, dt: f64) {
    let phase = engine.global_state.get_f64(K_PHASE).unwrap_or(0.0);

    if phase == 2.0 || phase == 3.0 {
        // Result screen timer
        let t = engine.global_state.get_f64(K_RESULT_TIMER).unwrap_or(0.0) + dt;
        engine.global_state.set_f64(K_RESULT_TIMER, t);
        return;
    }

    if phase != 1.0 { return; } // only simulate when ball is moving

    let mut bx = engine.global_state.get_f64(K_BALL_X).unwrap_or(0.0);
    let mut by = engine.global_state.get_f64(K_BALL_Y).unwrap_or(0.0);
    let mut vx = engine.global_state.get_f64(K_BALL_VX).unwrap_or(0.0);
    let mut vy = engine.global_state.get_f64(K_BALL_VY).unwrap_or(0.0);
    let hole_x = engine.global_state.get_f64(K_HOLE_X).unwrap_or(0.0);
    let hole_y = engine.global_state.get_f64(K_HOLE_Y).unwrap_or(0.0);

    // Sub-stepping for stability
    let steps = 4;
    let sub_dt = dt / steps as f64;

    for _ in 0..steps {
        // Apply drag (higher in sand)
        let mut drag = DRAG;
        if let Some(ref tm) = engine.tilemap {
            if let Some((tx, ty)) = tm.world_to_tile(bx, by) {
                if let Some(tile) = tm.get(tx, ty) {
                    if let crate::tilemap::TileType::Custom(1) = tile.tile_type {
                        drag = DRAG * 3.0; // sand = heavy drag
                    }
                }
            }
        }

        vx -= vx * drag * sub_dt;
        vy -= vy * drag * sub_dt;

        let new_x = bx + vx * sub_dt;
        let new_y = by + vy * sub_dt;

        // Wall collision
        let solid_at = |wx: f64, wy: f64| -> bool {
            if let Some(ref tm) = engine.tilemap {
                // Check at ball radius offsets
                tm.is_solid_at_world(wx, wy)
                    || tm.is_solid_at_world(wx - BALL_RADIUS, wy)
                    || tm.is_solid_at_world(wx + BALL_RADIUS, wy)
                    || tm.is_solid_at_world(wx, wy - BALL_RADIUS)
                    || tm.is_solid_at_world(wx, wy + BALL_RADIUS)
            } else {
                false
            }
        };

        if solid_at(new_x, new_y) {
            let sx = solid_at(new_x, by);
            let sy = solid_at(bx, new_y);
            match (sx, sy) {
                (true, true) => {
                    vx = -vx * RESTITUTION;
                    vy = -vy * RESTITUTION;
                }
                (true, false) => {
                    vx = -vx * RESTITUTION;
                    by = new_y;
                }
                (false, true) => {
                    bx = new_x;
                    vy = -vy * RESTITUTION;
                }
                (false, false) => {
                    // Corner graze — reflect both
                    vx = -vx * RESTITUTION;
                    vy = -vy * RESTITUTION;
                }
            }
        } else {
            bx = new_x;
            by = new_y;
        }

        // Check if ball reached the hole
        let dx = bx - hole_x;
        let dy = by - hole_y;
        let dist = (dx * dx + dy * dy).sqrt();
        let speed = (vx * vx + vy * vy).sqrt();

        if dist < HOLE_RADIUS && speed < 250.0 {
            // Sunk!
            engine.global_state.set_f64(K_PHASE, 2.0);
            engine.global_state.set_f64(K_RESULT_TIMER, 0.0);
            engine.global_state.set_f64(K_BALL_X, hole_x);
            engine.global_state.set_f64(K_BALL_Y, hole_y);
            engine.global_state.set_f64(K_BALL_VX, 0.0);
            engine.global_state.set_f64(K_BALL_VY, 0.0);

            // Screen shake for celebration
            engine.post_fx.shake_remaining = 0.3;
            engine.post_fx.shake_intensity = 4.0;
            return;
        }
    }

    // Check if ball stopped
    let speed = (vx * vx + vy * vy).sqrt();
    if speed < STOP_THRESHOLD {
        vx = 0.0;
        vy = 0.0;
        engine.global_state.set_f64(K_PHASE, 0.0); // back to aiming

        let strokes = engine.global_state.get_f64(K_STROKES).unwrap_or(0.0) as i32;
        if strokes >= PAR + 3 {
            // Too many strokes — fail
            engine.global_state.set_f64(K_PHASE, 3.0);
            engine.global_state.set_f64(K_RESULT_TIMER, 0.0);
        }
    }

    engine.global_state.set_f64(K_BALL_X, bx);
    engine.global_state.set_f64(K_BALL_Y, by);
    engine.global_state.set_f64(K_BALL_VX, vx);
    engine.global_state.set_f64(K_BALL_VY, vy);
}

// ─── Render ─────────────────────────────────────────────────────────────

pub fn render(engine: &mut Engine) {
    let fb = &mut engine.framebuffer;
    let w = fb.width;
    let h = fb.height;

    // Clear
    fb.clear(COL_BG);

    // Render tilemap (course)
    if let Some(ref tm) = engine.tilemap {
        tm.render(fb, 0.0, 0.0, 1.0, w, h);
    }

    let ball_x = engine.global_state.get_f64(K_BALL_X).unwrap_or(0.0);
    let ball_y = engine.global_state.get_f64(K_BALL_Y).unwrap_or(0.0);
    let hole_x = engine.global_state.get_f64(K_HOLE_X).unwrap_or(0.0);
    let hole_y = engine.global_state.get_f64(K_HOLE_Y).unwrap_or(0.0);
    let phase = engine.global_state.get_f64(K_PHASE).unwrap_or(0.0);
    let strokes = engine.global_state.get_f64(K_STROKES).unwrap_or(0.0) as i32;

    // Draw hole (target)
    shapes::fill_circle(fb, hole_x, hole_y, HOLE_RADIUS + 3.0, COL_HOLE_RIM);
    shapes::fill_circle(fb, hole_x, hole_y, HOLE_RADIUS, COL_HOLE);

    // Flag pole
    shapes::draw_line(fb, hole_x + 2.0, hole_y, hole_x + 2.0, hole_y - 20.0, COL_TEXT_DIM);
    // Flag triangle
    for i in 0..8 {
        let fy = hole_y - 20.0 + i as f64;
        let fw = 8.0 - i as f64;
        shapes::draw_line(fb, hole_x + 3.0, fy, hole_x + 3.0 + fw, fy, COL_FLAG);
    }

    // Draw aim preview when dragging
    let aim_active = engine.global_state.get_f64(K_AIM_ACTIVE).unwrap_or(0.0);
    if aim_active > 0.5 && phase == 0.0 {
        let aim_x = engine.global_state.get_f64(K_AIM_X).unwrap_or(0.0);
        let aim_y = engine.global_state.get_f64(K_AIM_Y).unwrap_or(0.0);

        // Compute shot direction (slingshot)
        let dx = ball_x - aim_x;
        let dy = ball_y - aim_y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > 5.0 {
            let power = (dist / 120.0).min(1.0) * MAX_POWER;
            let angle = dy.atan2(dx);
            let vx = angle.cos() * power;
            let vy = angle.sin() * power;

            // Use AimPreview to compute trajectory
            let config = AimConfig {
                dot_count: 8,
                time_step: 0.08,
                drag: DRAG * 0.08,
                gravity_y: 0.0,
                ball_radius: BALL_RADIUS,
                restitution: RESTITUTION,
            };

            // Tilemap solid check
            let tm_ref = engine.tilemap.as_ref();
            let points = compute_arc(ball_x, ball_y, vx, vy, &config, |wx, wy| {
                if let Some(tm) = tm_ref {
                    tm.is_solid_at_world(wx, wy)
                } else {
                    false
                }
            });

            // Draw ghost dots with fading alpha
            for (i, pt) in points.iter().enumerate() {
                let alpha = 200 - (i as u8 * 20).min(180);
                let dot_color = Color::from_rgba(COL_AIM_DOT.r, COL_AIM_DOT.g, COL_AIM_DOT.b, alpha);
                let r = 3.0 - (i as f64 * 0.3).min(2.0);
                shapes::fill_circle(fb, pt.x, pt.y, r, dot_color);
            }

            // Draw power indicator line from ball to cursor
            let line_alpha = 120;
            let line_color = Color::from_rgba(255, 255, 200, line_alpha);
            shapes::draw_line(fb, ball_x, ball_y, ball_x + dx * 0.3, ball_y + dy * 0.3, line_color);
        }
    }

    // Draw ball shadow
    if phase != 2.0 {
        shapes::fill_circle(fb, ball_x + 2.0, ball_y + 2.0, BALL_RADIUS, COL_BALL_SHADOW);
    }

    // Draw ball
    if phase != 2.0 {
        shapes::fill_circle(fb, ball_x, ball_y, BALL_RADIUS, COL_BALL);
        // Highlight
        shapes::fill_circle(fb, ball_x - 1.5, ball_y - 1.5, 1.5, Color::from_rgba(255, 255, 255, 180));
    }

    // ─── HUD ────────────────────────────────────────────────────────────
    let hud_y = WORLD_H;
    shapes::fill_rect(fb, 0.0, hud_y, WIDTH, HUD_H, COL_HUD_BG);
    // Divider line
    shapes::draw_line(fb, 0.0, hud_y, WIDTH, hud_y, Color::from_rgba(80, 80, 90, 255));

    // Title
    text::draw_text(fb, 16, (hud_y + 12.0) as i32, "S-LEAGUE", COL_TEXT, 2);

    // Strokes
    let stroke_text = format!("STROKE {}", strokes);
    text::draw_text(fb, 16, (hud_y + 40.0) as i32, &stroke_text, COL_TEXT_DIM, 1);

    // Par
    let par_text = format!("PAR {}", PAR);
    text::draw_text(fb, 16, (hud_y + 56.0) as i32, &par_text, COL_TEXT_DIM, 1);

    // Score relative to par
    if strokes > 0 {
        let diff = strokes - PAR;
        let score_text = if diff < 0 {
            format!("{}", diff)
        } else if diff == 0 {
            "E".to_string()
        } else {
            format!("+{}", diff)
        };
        let score_col = if diff <= 0 { COL_SUCCESS } else { COL_FLAG };
        text::draw_text(fb, 380, (hud_y + 12.0) as i32, &score_text, score_col, 3);
    }

    // Power bar (when aiming)
    if aim_active > 0.5 && phase == 0.0 {
        let aim_x = engine.global_state.get_f64(K_AIM_X).unwrap_or(0.0);
        let aim_y = engine.global_state.get_f64(K_AIM_Y).unwrap_or(0.0);
        let dx = ball_x - aim_x;
        let dy = ball_y - aim_y;
        let dist = (dx * dx + dy * dy).sqrt();
        let power_pct = (dist / 120.0).min(1.0);

        let bar_x = 150.0;
        let bar_y = hud_y + 48.0;
        let bar_w = 200.0;
        let bar_h = 14.0;

        shapes::fill_rect(fb, bar_x, bar_y, bar_w, bar_h, COL_POWER_BG);
        shapes::fill_rect(fb, bar_x, bar_y, bar_w * power_pct, bar_h, COL_POWER_BAR);
        shapes::draw_rect(fb, bar_x, bar_y, bar_w, bar_h, COL_TEXT_DIM);
        text::draw_text(fb, (bar_x) as i32, (bar_y - 12.0) as i32, "POWER", COL_TEXT_DIM, 1);
    }

    // Phase-specific messages
    if phase == 0.0 && strokes == 0 {
        text::draw_text_centered(fb, 240, (hud_y + 96.0) as i32, "DRAG TO AIM - RELEASE TO SHOOT", COL_TEXT_DIM, 1);
    }

    // Result overlay
    if phase == 2.0 {
        // Sunk!
        let result_timer = engine.global_state.get_f64(K_RESULT_TIMER).unwrap_or(0.0);
        let alpha = ((result_timer * 3.0).min(1.0) * 200.0) as u8;
        shapes::fill_rect(fb, 0.0, 200.0, WIDTH, 180.0, Color::from_rgba(0, 0, 0, alpha));

        if result_timer > 0.3 {
            text::draw_text_centered(fb, 240, 260, "HOLE IN", COL_SUCCESS, 3);
            let stroke_label = if strokes == 1 { "ONE!" } else { &format!("{}", strokes) };
            text::draw_text_centered(fb, 240, 300, stroke_label, COL_SUCCESS, 4);

            let diff = strokes - PAR;
            let label = match diff {
                d if d <= -2 => "EAGLE!",
                -1 => "BIRDIE!",
                0 => "PAR",
                1 => "BOGEY",
                _ => "KEEP PRACTICING",
            };
            text::draw_text_centered(fb, 240, 340, label, COL_TEXT, 2);
        }

        if result_timer > 1.5 {
            text::draw_text_centered(fb, 240, 370, "TAP TO PLAY AGAIN", COL_TEXT_DIM, 1);
        }
    }

    if phase == 3.0 {
        let result_timer = engine.global_state.get_f64(K_RESULT_TIMER).unwrap_or(0.0);
        let alpha = ((result_timer * 3.0).min(1.0) * 200.0) as u8;
        shapes::fill_rect(fb, 0.0, 200.0, WIDTH, 180.0, Color::from_rgba(0, 0, 0, alpha));

        if result_timer > 0.3 {
            text::draw_text_centered(fb, 240, 270, "TOO MANY STROKES!", COL_FLAG, 2);
            text::draw_text_centered(fb, 240, 310, "YOU TAKE DAMAGE", COL_TEXT_DIM, 2);
        }

        if result_timer > 1.5 {
            text::draw_text_centered(fb, 240, 350, "TAP TO RETRY", COL_TEXT_DIM, 1);
        }
    }

    // Screen effects
    engine.screen_fx.tick(0.016);
    engine.screen_fx.apply(fb);

    // Post-FX
    crate::rendering::post_fx::apply(
        fb, &mut engine.post_fx, 0.016, engine.frame,
    );
}
