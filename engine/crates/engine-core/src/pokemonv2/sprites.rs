// AI-INSTRUCTIONS: pokemonv2/sprites.rs — Procedural sprite/tile generation.
// No game assets — everything drawn with colored rectangles and simple shapes.
// Import graph: sprites.rs <- data.rs only

use super::data::{Direction, Emote};
use crate::engine::Engine;
use crate::rendering::color::Color;

// ── Sprite ID Constants ───────────────────────────────────────────────────────

pub const SPRITE_PLAYER: u8 = 0;
pub const SPRITE_MOM: u8 = 1;
pub const SPRITE_ELM: u8 = 2;
pub const SPRITE_TEACHER: u8 = 3;
pub const SPRITE_FISHER: u8 = 4;
pub const SPRITE_RIVAL: u8 = 5;
pub const SPRITE_POKE_BALL: u8 = 6;
pub const SPRITE_SCIENTIST: u8 = 7;
pub const SPRITE_OFFICER: u8 = 8;
pub const SPRITE_POKEFAN_F: u8 = 9;
pub const SPRITE_CONSOLE: u8 = 10;
pub const SPRITE_DOLL_1: u8 = 11;
pub const SPRITE_DOLL_2: u8 = 12;
pub const SPRITE_BIG_DOLL: u8 = 13;

// ── Tile Colors ───────────────────────────────────────────────────────────────

/// Map tile visual ID to color. Procedural — no image assets needed.
pub fn tile_color(tile_id: u8) -> Color {
    match tile_id {
        0 => Color::from_rgba(72, 140, 56, 255),   // grass green
        1 => Color::from_rgba(160, 148, 96, 255),  // path tan
        2 => Color::from_rgba(48, 32, 16, 255),    // wall dark brown
        3 => Color::from_rgba(100, 80, 60, 255),   // door/mat brown
        4 => Color::from_rgba(200, 184, 160, 255), // floor beige
        5 => Color::from_rgba(140, 100, 60, 255),  // wood/furniture brown
        6 => Color::from_rgba(200, 60, 40, 255),   // stairs/roof red
        7 => Color::from_rgba(80, 120, 200, 255),  // water blue
        _ => Color::from_rgba(128, 128, 128, 255), // unknown grey
    }
}

/// NPC sprite color by sprite_id for body color.
fn sprite_body_color(sprite_id: u8) -> Color {
    match sprite_id {
        SPRITE_PLAYER    => Color::from_rgba(80, 120, 200, 255),  // blue jacket
        SPRITE_MOM       => Color::from_rgba(220, 120, 160, 255), // pink dress
        SPRITE_ELM       => Color::from_rgba(80, 160, 80, 255),   // green lab coat
        SPRITE_TEACHER   => Color::from_rgba(200, 160, 80, 255),  // tan
        SPRITE_FISHER    => Color::from_rgba(60, 100, 60, 255),   // dark green vest
        SPRITE_RIVAL     => Color::from_rgba(200, 80, 80, 255),   // red shirt
        SPRITE_POKE_BALL => Color::from_rgba(220, 60, 60, 255),   // red pokeball top
        SPRITE_SCIENTIST => Color::from_rgba(200, 200, 220, 255), // white coat
        SPRITE_OFFICER   => Color::from_rgba(60, 80, 140, 255),   // blue uniform
        SPRITE_POKEFAN_F => Color::from_rgba(200, 140, 200, 255), // purple
        SPRITE_CONSOLE   => Color::from_rgba(60, 60, 80, 255),    // dark grey
        SPRITE_DOLL_1    => Color::from_rgba(240, 200, 120, 255), // plush yellow
        SPRITE_DOLL_2    => Color::from_rgba(200, 120, 240, 255), // plush purple
        SPRITE_BIG_DOLL  => Color::from_rgba(240, 120, 120, 255), // big plush pink
        _                => Color::from_rgba(180, 180, 180, 255),
    }
}

// ── Sprite Drawing ────────────────────────────────────────────────────────────

/// Draw a 16x16 sprite at screen position (sx, sy).
/// Procedurally generated with simple geometric shapes.
pub fn draw_sprite(
    engine: &mut Engine,
    sprite_id: u8,
    sx: i32,
    sy: i32,
    direction: Direction,
    walk_frame: u8,
) {
    if sprite_id == SPRITE_POKE_BALL {
        draw_pokeball(engine, sx, sy);
        return;
    }

    if sprite_id >= SPRITE_CONSOLE {
        draw_furniture(engine, sprite_id, sx, sy);
        return;
    }

    let body_color = sprite_body_color(sprite_id);
    let skin_color = Color::from_rgba(240, 200, 160, 255);

    // Simple walk bob — shift body 1px down on walk frames 1 and 3
    let bob = if walk_frame == 1 || walk_frame == 3 { 1i32 } else { 0i32 };

    // Head: 8x8 at (sx+4, sy+1)
    fill_rect(engine, sx + 4, sy + 1 + bob, 8, 8, skin_color);

    // Eyes based on direction (simple dots)
    match direction {
        Direction::Down => {
            fill_rect(engine, sx + 5, sy + 4 + bob, 2, 2, Color::from_rgba(40, 40, 40, 255));
            fill_rect(engine, sx + 9, sy + 4 + bob, 2, 2, Color::from_rgba(40, 40, 40, 255));
        }
        Direction::Up => {
            // Back of head — no eyes
        }
        Direction::Left => {
            fill_rect(engine, sx + 5, sy + 4 + bob, 2, 2, Color::from_rgba(40, 40, 40, 255));
        }
        Direction::Right => {
            fill_rect(engine, sx + 9, sy + 4 + bob, 2, 2, Color::from_rgba(40, 40, 40, 255));
        }
    }

    // Body: 10x6 at (sx+3, sy+9)
    fill_rect(engine, sx + 3, sy + 9 + bob, 10, 6, body_color);

    // Legs (walk animation)
    let leg_offset = match walk_frame {
        0 => (0i32, 0i32),
        1 => (-1,  1),
        2 => (0,   0),
        _ => (1,   1),
    };
    fill_rect(engine, sx + 4 + leg_offset.0, sy + 14 + bob, 3, 2, body_color);
    fill_rect(engine, sx + 9 - leg_offset.0, sy + 14 + bob + leg_offset.1, 3, 2, body_color);
}

fn draw_pokeball(engine: &mut Engine, sx: i32, sy: i32) {
    let red   = Color::from_rgba(220, 60, 60, 255);
    let white = Color::from_rgba(240, 240, 240, 255);
    let black = Color::from_rgba(40, 40, 40, 255);

    // Top half red, bottom half white
    fill_rect(engine, sx + 4, sy + 4, 8, 4, red);
    fill_rect(engine, sx + 4, sy + 8, 8, 4, white);
    // Center band
    fill_rect(engine, sx + 4, sy + 7, 8, 2, black);
    // Center button
    fill_rect(engine, sx + 7, sy + 6, 2, 4, white);
}

fn draw_furniture(engine: &mut Engine, sprite_id: u8, sx: i32, sy: i32) {
    let color = sprite_body_color(sprite_id);
    fill_rect(engine, sx + 2, sy + 2, 12, 12, color);
}

fn fill_rect(engine: &mut Engine, x: i32, y: i32, w: i32, h: i32, color: Color) {
    let fw = engine.framebuffer.width as i32;
    let fh = engine.framebuffer.height as i32;
    for dy in 0..h {
        for dx in 0..w {
            let px = x + dx;
            let py = y + dy;
            if px >= 0 && py >= 0 && px < fw && py < fh {
                engine.framebuffer.set_pixel(px, py, color);
            }
        }
    }
}

// ── Emote Drawing ─────────────────────────────────────────────────────────────

/// Draw emote bubble above NPC at screen position (sx, sy).
pub fn draw_emote(engine: &mut Engine, emote: Emote, sx: i32, sy: i32) {
    let bubble_x = sx + 10;
    let bubble_y = sy - 12;
    let bg = Color::from_rgba(240, 240, 200, 255);
    let border = Color::from_rgba(80, 80, 80, 255);

    // Bubble background
    fill_rect(engine, bubble_x, bubble_y, 10, 10, border);
    fill_rect(engine, bubble_x + 1, bubble_y + 1, 8, 8, bg);

    // Symbol based on emote type
    match emote {
        Emote::Shock => {
            // Draw "!" in the bubble
            let gold = Color::from_rgba(240, 180, 40, 255);
            fill_rect(engine, bubble_x + 4, bubble_y + 2, 2, 4, gold);
            fill_rect(engine, bubble_x + 4, bubble_y + 7, 2, 2, gold);
        }
        Emote::Question => {
            // Draw "?" in the bubble
            let blue = Color::from_rgba(80, 120, 220, 255);
            fill_rect(engine, bubble_x + 3, bubble_y + 2, 4, 2, blue);
            fill_rect(engine, bubble_x + 5, bubble_y + 4, 2, 2, blue);
            fill_rect(engine, bubble_x + 4, bubble_y + 7, 2, 2, blue);
        }
        Emote::Happy => {
            // Draw wavy line
            let green = Color::from_rgba(60, 180, 80, 255);
            fill_rect(engine, bubble_x + 2, bubble_y + 5, 6, 2, green);
        }
    }
}
