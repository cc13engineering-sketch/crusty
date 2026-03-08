// AI-INSTRUCTIONS: pokemonv2/render.rs — All rendering. Reads state, never mutates it.
// Tile rendering, sprites, text boxes, menus, battle screen.
// Sprint 2: battle render, grass/ledge tile colors, updated dispatch.
// Import graph: render.rs <- data.rs, maps.rs, overworld.rs(constants), events.rs, sprites.rs, battle.rs

use super::data::{self as data, Direction};
use super::maps::{C_GRASS, C_LEDGE_D};
use super::sprites::{draw_sprite, draw_emote, tile_color};
use super::overworld::{TILE_PX, VIEW_TILES_X, VIEW_TILES_Y};
use super::{GamePhase, PokemonV2Sim};
use crate::engine::Engine;
use crate::rendering::color::Color;

// ── Constants ────────────────────────────────────────────────────────────────

const SCREEN_W: i32 = VIEW_TILES_X * TILE_PX; // 160
const SCREEN_H: i32 = VIEW_TILES_Y * TILE_PX; // 144
const CHAR_W: i32 = 5;
const CHAR_H: i32 = 7;
const TEXT_BOX_Y: i32 = SCREEN_H - 42;
const TEXT_MAX_CHARS: usize = 20;

// ── Top-Level Dispatch ────────────────────────────────────────────────────────

pub fn render_game(sim: &PokemonV2Sim, engine: &mut Engine) {
    match sim.phase {
        GamePhase::TitleScreen => render_title(sim, engine),
        GamePhase::Overworld | GamePhase::Script => render_overworld(sim, engine),
        GamePhase::Dialogue => render_dialogue_phase(sim, engine),
        GamePhase::StarterSelect { cursor } => render_starter_select(sim, cursor, engine),
        GamePhase::MapTransition { timer } => render_transition(sim, timer, engine),
        GamePhase::Battle => render_battle(sim, engine),
        GamePhase::Menu => render_overworld(sim, engine), // stub
    }
}

// ── Overworld Render ──────────────────────────────────────────────────────────

fn render_overworld(sim: &PokemonV2Sim, engine: &mut Engine) {
    let fw = engine.framebuffer.width as i32;
    let fh = engine.framebuffer.height as i32;

    // Clear to black
    for y in 0..fh {
        for x in 0..fw {
            engine.framebuffer.set_pixel(x, y, Color::from_rgba(0, 0, 0, 255));
        }
    }

    let cam_ox = (sim.camera.x as i32) - SCREEN_W / 2;
    let cam_oy = (sim.camera.y as i32) - SCREEN_H / 2;

    // ── Draw tiles ──────────────────────────────────────────────────────
    let map = &sim.current_map;
    let start_tx = cam_ox / TILE_PX - 1;
    let end_tx   = start_tx + VIEW_TILES_X + 2;
    let start_ty = cam_oy / TILE_PX - 1;
    let end_ty   = start_ty + VIEW_TILES_Y + 2;

    for ty in start_ty..end_ty {
        for tx in start_tx..end_tx {
            if tx < 0 || ty < 0 || tx >= map.width || ty >= map.height {
                continue;
            }
            let idx = (ty * map.width + tx) as usize;
            let tile_id = map.tiles[idx];
            let color = tile_color(tile_id);
            let sx = tx * TILE_PX - cam_ox;
            let sy = ty * TILE_PX - cam_oy;
            fill_rect(engine, sx, sy, TILE_PX, TILE_PX, color);

            // Warp tile overlay (slightly lighter)
            if map.collision[idx] == 3 { // C_WARP
                let overlay = Color::from_rgba(
                    color.r.saturating_add(20),
                    color.g.saturating_add(20),
                    color.b.saturating_add(20),
                    200,
                );
                fill_rect(engine, sx + 2, sy + 2, TILE_PX - 4, TILE_PX - 4, overlay);
            }

            // Grass tile overlay (green tint)
            if map.collision[idx] == C_GRASS {
                let grass = Color::from_rgba(40, 120, 40, 160);
                fill_rect(engine, sx, sy, TILE_PX, TILE_PX, grass);
                // Draw grass blades: two vertical lines
                let blade = Color::from_rgba(50, 160, 50, 200);
                fill_rect(engine, sx + 4, sy + 4, 2, 6, blade);
                fill_rect(engine, sx + 10, sy + 4, 2, 6, blade);
            }

            // Ledge tile overlay (brown stripe at bottom)
            if map.collision[idx] == C_LEDGE_D {
                let ledge = Color::from_rgba(100, 60, 20, 200);
                fill_rect(engine, sx, sy + TILE_PX - 4, TILE_PX, 4, ledge);
            }
        }
    }

    // ── Draw NPCs ───────────────────────────────────────────────────────
    for (i, npc) in sim.npc_states.iter().enumerate() {
        if !npc.visible { continue; }
        let npc_def = match sim.current_map.npcs.get(i) {
            Some(d) => d,
            None => continue,
        };

        let walk_dx = match npc.facing {
            Direction::Right if npc.is_walking => npc.walk_offset as i32,
            Direction::Left  if npc.is_walking => -(npc.walk_offset as i32),
            _ => 0,
        };
        let walk_dy = match npc.facing {
            Direction::Down if npc.is_walking => npc.walk_offset as i32,
            Direction::Up   if npc.is_walking => -(npc.walk_offset as i32),
            _ => 0,
        };

        let sx = npc.x * TILE_PX - cam_ox + walk_dx;
        let sy = npc.y * TILE_PX - cam_oy + walk_dy;

        draw_sprite(engine, npc_def.sprite_id, sx, sy, npc.facing, 0);

        // Draw emote if active
        if let Some((emote, _frames)) = npc.emote {
            draw_emote(engine, emote, sx, sy);
        }
    }

    // ── Draw player ─────────────────────────────────────────────────────
    let walk_dx = match sim.player.facing {
        Direction::Right if sim.player.is_walking => sim.player.walk_offset as i32,
        Direction::Left  if sim.player.is_walking => -(sim.player.walk_offset as i32),
        _ => 0,
    };
    let walk_dy = match sim.player.facing {
        Direction::Down if sim.player.is_walking => sim.player.walk_offset as i32,
        Direction::Up   if sim.player.is_walking => -(sim.player.walk_offset as i32),
        _ => 0,
    };

    let player_sx = sim.player.x * TILE_PX - cam_ox + walk_dx;
    let player_sy = sim.player.y * TILE_PX - cam_oy + walk_dy;
    draw_sprite(engine, 0, player_sx, player_sy, sim.player.facing, sim.player.walk_frame);

    // ── Draw script text box (if active) ────────────────────────────────
    if let Some(ref script) = sim.script {
        if let Some(ref text) = script.text_buffer {
            draw_text_box(engine, text);
        }
        if script.showing_yesno {
            draw_yesno_box(engine, script.yesno_cursor);
        }
    }

    // ── Draw dialogue box (if active) ─────────────────────────────────
    if let Some(ref dlg) = sim.dialogue {
        draw_text_box(engine, dlg.visible_text());
    }
}

// ── Text Box ──────────────────────────────────────────────────────────────────

fn draw_text_box(engine: &mut Engine, text: &str) {
    let box_h = 40;
    let box_x = 2;
    let box_y = TEXT_BOX_Y;
    let box_w = SCREEN_W - 4;

    // Semi-transparent dark background
    let bg = Color::from_rgba(10, 10, 10, 220);
    fill_rect(engine, box_x, box_y, box_w, box_h, bg);

    // White border
    let border = Color::from_rgba(240, 240, 240, 255);
    draw_rect_border(engine, box_x, box_y, box_w, box_h, border);

    // Text, word-wrapped
    let white = Color::from_rgba(240, 240, 240, 255);
    let wrapped = wrap_text(text, TEXT_MAX_CHARS);
    for (i, line) in wrapped.iter().take(4).enumerate() {
        draw_text(engine, line, box_x + 4, box_y + 4 + (i as i32) * (CHAR_H + 2), white);
    }
}

fn draw_yesno_box(engine: &mut Engine, cursor: u8) {
    let box_x = SCREEN_W - 50;
    let box_y = TEXT_BOX_Y - 30;
    let box_w = 46;
    let box_h = 28;

    let bg = Color::from_rgba(10, 10, 10, 220);
    fill_rect(engine, box_x, box_y, box_w, box_h, bg);
    draw_rect_border(engine, box_x, box_y, box_w, box_h, Color::from_rgba(240, 240, 240, 255));

    let white = Color::from_rgba(240, 240, 240, 255);
    let yellow = Color::from_rgba(240, 220, 40, 255);

    let yes_color = if cursor == 0 { yellow } else { white };
    let no_color  = if cursor == 1 { yellow } else { white };

    draw_text(engine, "YES", box_x + 8, box_y + 4, yes_color);
    draw_text(engine, "NO",  box_x + 8, box_y + 4 + CHAR_H + 4, no_color);
}

fn draw_rect_border(engine: &mut Engine, x: i32, y: i32, w: i32, h: i32, color: Color) {
    fill_rect(engine, x,         y,         w, 2, color); // top
    fill_rect(engine, x,         y + h - 2, w, 2, color); // bottom
    fill_rect(engine, x,         y,         2, h, color); // left
    fill_rect(engine, x + w - 2, y,         2, h, color); // right
}

// ── Title Screen ──────────────────────────────────────────────────────────────

fn render_title(sim: &PokemonV2Sim, engine: &mut Engine) {
    let fw = engine.framebuffer.width as i32;
    let fh = engine.framebuffer.height as i32;

    // Dark blue-black background
    for y in 0..fh {
        for x in 0..fw {
            let shade = ((y as f64 / fh as f64) * 30.0) as u8;
            engine.framebuffer.set_pixel(x, y, Color::from_rgba(0, 0, shade, 255));
        }
    }

    // Pulsing title text
    let pulse = ((sim.title_timer * 2.0 * std::f64::consts::PI).sin() * 0.5 + 0.5) as f64;
    let brightness = (160.0 + pulse * 80.0) as u8;
    let title_color = Color::from_rgba(brightness, brightness, brightness, 255);

    draw_text_centered(engine, "POKEMON CRYSTAL", fh / 2 - 20, title_color);
    draw_text_centered(engine, "v2", fh / 2 - 10, Color::from_rgba(100, 200, 240, 255));

    // Press start prompt (blink)
    if (sim.title_timer * 2.0) as u32 % 2 == 0 {
        let prompt_color = Color::from_rgba(200, 200, 200, 255);
        draw_text_centered(engine, "Press Z to Start", fh / 2 + 20, prompt_color);
    }
}

// ── Starter Select ────────────────────────────────────────────────────────────

fn render_starter_select(sim: &PokemonV2Sim, cursor: u8, engine: &mut Engine) {
    let fw = engine.framebuffer.width as i32;
    let fh = engine.framebuffer.height as i32;

    // Background
    for y in 0..fh {
        for x in 0..fw {
            engine.framebuffer.set_pixel(x, y, Color::from_rgba(20, 40, 80, 255));
        }
    }

    let white  = Color::from_rgba(240, 240, 240, 255);
    let yellow = Color::from_rgba(240, 220, 40, 255);

    draw_text_centered(engine, "Choose your Pokemon!", 10, white);

    let starters = [
        ("CYNDAQUIL", Color::from_rgba(220, 80, 40, 255)),
        ("TOTODILE",  Color::from_rgba(60, 120, 200, 255)),
        ("CHIKORITA", Color::from_rgba(80, 180, 80, 255)),
    ];

    for (i, (name, color)) in starters.iter().enumerate() {
        let sx = 20 + (i as i32) * 46;
        let sy = 50;
        let selected = cursor as usize == i;

        // Pokeball icon
        let ball_color = if selected { *color } else { Color::from_rgba(100, 100, 100, 255) };
        fill_rect(engine, sx + 10, sy + 10, 20, 20, ball_color);
        fill_rect(engine, sx + 14, sy + 20, 12, 2, Color::from_rgba(240, 240, 240, 255));

        // Name
        let text_color = if selected { yellow } else { white };
        draw_text(engine, name, sx, sy + 34, text_color);

        // Cursor arrow
        if selected {
            fill_rect(engine, sx + 14, sy + 2, 4, 6, yellow);
            fill_rect(engine, sx + 12, sy + 4, 8, 2, yellow);
        }
    }

    let _ = sim; // sim not needed beyond cursor in this view
}

// ── Map Transition ────────────────────────────────────────────────────────────

fn render_transition(sim: &PokemonV2Sim, timer: f64, engine: &mut Engine) {
    // First render the underlying scene
    render_overworld(sim, engine);

    // Then overlay black fade
    let fw = engine.framebuffer.width as i32;
    let fh = engine.framebuffer.height as i32;

    let alpha = if timer < 0.5 {
        // Fade to black: 0 -> 255
        (timer / 0.5 * 255.0) as u8
    } else {
        // Fade from black: 255 -> 0
        ((1.0 - (timer - 0.5) / 0.5) * 255.0) as u8
    };

    for y in 0..fh {
        for x in 0..fw {
            let base = ((y * fw + x) * 4) as usize;
            let r = engine.framebuffer.pixels[base];
            let g = engine.framebuffer.pixels[base + 1];
            let b = engine.framebuffer.pixels[base + 2];
            let blended_r = lerp_u8(r, 0, alpha);
            let blended_g = lerp_u8(g, 0, alpha);
            let blended_b = lerp_u8(b, 0, alpha);
            engine.framebuffer.set_pixel(x, y, Color::from_rgba(blended_r, blended_g, blended_b, 255));
        }
    }
}

fn lerp_u8(a: u8, b: u8, t: u8) -> u8 {
    let at = t as u16;
    ((a as u16 * (255 - at) + b as u16 * at) / 255) as u8
}

// ── Dialogue Phase ────────────────────────────────────────────────────────────

fn render_dialogue_phase(sim: &PokemonV2Sim, engine: &mut Engine) {
    render_overworld(sim, engine);
    if let Some(ref dlg) = sim.dialogue {
        draw_text_box(engine, dlg.visible_text());
    }
}

// ── Battle Screen ─────────────────────────────────────────────────────────────

fn render_battle(sim: &PokemonV2Sim, engine: &mut Engine) {
    fill_rect_full(engine, Color::from_rgba(248, 248, 248, 255));

    if let Some(ref battle) = sim.battle {
        if let Some(ref player_mon) = sim.party.first() {
            let player_sdata = data::species_data(player_mon.species);
            let enemy_sdata = data::species_data(battle.current_enemy().species);

            // Enemy info top-left (Review #12: draw_text arg order: engine, text, x, y, color)
            draw_text(engine, enemy_sdata.name, 8, 8, Color::from_rgba(0, 0, 0, 255));
            draw_text(engine, &format!("Lv{}", battle.current_enemy().level), 8, 18, Color::from_rgba(0, 0, 0, 255));
            let enemy_hp_pct = if battle.current_enemy().max_hp > 0 {
                battle.current_enemy().hp as f64 / battle.current_enemy().max_hp as f64
            } else { 0.0 };
            draw_hp_bar(engine, 8, 28, enemy_hp_pct);

            // Player info bottom-right
            let py = SCREEN_H - 50;
            draw_text(engine, player_sdata.name, 80, py, Color::from_rgba(0, 0, 0, 255));
            draw_text(engine, &format!("Lv{}", player_mon.level), 80, py + 10, Color::from_rgba(0, 0, 0, 255));
            draw_text(engine, &format!("HP {}/{}", player_mon.hp, player_mon.max_hp),
                80, py + 20, Color::from_rgba(0, 0, 0, 255));
            let player_hp_pct = if player_mon.max_hp > 0 {
                player_mon.hp as f64 / player_mon.max_hp as f64
            } else { 0.0 };
            draw_hp_bar(engine, 80, py + 30, player_hp_pct);
        }

        if let Some(ref msg) = battle.message {
            draw_text_box(engine, msg);
        }
    }
}

fn draw_hp_bar(engine: &mut Engine, x: i32, y: i32, pct: f64) {
    let bar_w = 60i32;
    let bar_h = 4i32;
    fill_rect(engine, x, y, bar_w, bar_h, Color::from_rgba(64, 64, 64, 255));
    let fill_w = (pct * bar_w as f64) as i32;
    let hp_color = if pct > 0.5 {
        Color::from_rgba(0, 200, 0, 255)
    } else if pct > 0.2 {
        Color::from_rgba(200, 200, 0, 255)
    } else {
        Color::from_rgba(200, 0, 0, 255)
    };
    if fill_w > 0 {
        fill_rect(engine, x, y, fill_w, bar_h, hp_color);
    }
}

fn fill_rect_full(engine: &mut Engine, color: Color) {
    let fw = engine.framebuffer.width as i32;
    let fh = engine.framebuffer.height as i32;
    for y in 0..fh {
        for x in 0..fw {
            engine.framebuffer.set_pixel(x, y, color);
        }
    }
}

// ── Primitive Helpers ─────────────────────────────────────────────────────────

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

fn draw_text(engine: &mut Engine, text: &str, x: i32, y: i32, color: Color) {
    for (i, ch) in text.chars().enumerate() {
        draw_char(engine, ch, x + (i as i32) * (CHAR_W + 1), y, color);
    }
}

fn draw_text_centered(engine: &mut Engine, text: &str, y: i32, color: Color) {
    let fw = engine.framebuffer.width as i32;
    let text_w = text.len() as i32 * (CHAR_W + 1);
    let x = (fw - text_w) / 2;
    draw_text(engine, text, x, y, color);
}

fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for raw_line in text.split('\n') {
        let words: Vec<&str> = raw_line.split_whitespace().collect();
        if words.is_empty() {
            lines.push(String::new());
            continue;
        }
        let mut current = String::new();
        for word in words {
            if current.is_empty() {
                current.push_str(word);
            } else if current.len() + 1 + word.len() <= max_chars {
                current.push(' ');
                current.push_str(word);
            } else {
                lines.push(current.clone());
                current = word.to_string();
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
    }
    lines
}

/// Minimal 5x7 bitmap font — enough for ASCII text display.
/// Each character is stored as 5 bytes, one per column, LSB = top row.
fn char_bitmap(ch: char) -> [u8; 5] {
    match ch {
        'A' => [0x7E, 0x11, 0x11, 0x11, 0x7E],
        'B' => [0x7F, 0x49, 0x49, 0x49, 0x36],
        'C' => [0x3E, 0x41, 0x41, 0x41, 0x22],
        'D' => [0x7F, 0x41, 0x41, 0x22, 0x1C],
        'E' => [0x7F, 0x49, 0x49, 0x49, 0x41],
        'F' => [0x7F, 0x09, 0x09, 0x09, 0x01],
        'G' => [0x3E, 0x41, 0x49, 0x49, 0x7A],
        'H' => [0x7F, 0x08, 0x08, 0x08, 0x7F],
        'I' => [0x00, 0x41, 0x7F, 0x41, 0x00],
        'J' => [0x20, 0x40, 0x41, 0x3F, 0x01],
        'K' => [0x7F, 0x08, 0x14, 0x22, 0x41],
        'L' => [0x7F, 0x40, 0x40, 0x40, 0x40],
        'M' => [0x7F, 0x02, 0x0C, 0x02, 0x7F],
        'N' => [0x7F, 0x04, 0x08, 0x10, 0x7F],
        'O' => [0x3E, 0x41, 0x41, 0x41, 0x3E],
        'P' => [0x7F, 0x09, 0x09, 0x09, 0x06],
        'Q' => [0x3E, 0x41, 0x51, 0x21, 0x5E],
        'R' => [0x7F, 0x09, 0x19, 0x29, 0x46],
        'S' => [0x46, 0x49, 0x49, 0x49, 0x31],
        'T' => [0x01, 0x01, 0x7F, 0x01, 0x01],
        'U' => [0x3F, 0x40, 0x40, 0x40, 0x3F],
        'V' => [0x1F, 0x20, 0x40, 0x20, 0x1F],
        'W' => [0x7F, 0x20, 0x18, 0x20, 0x7F],
        'X' => [0x63, 0x14, 0x08, 0x14, 0x63],
        'Y' => [0x03, 0x04, 0x78, 0x04, 0x03],
        'Z' => [0x61, 0x51, 0x49, 0x45, 0x43],
        'a' => [0x20, 0x54, 0x54, 0x54, 0x78],
        'b' => [0x7F, 0x48, 0x44, 0x44, 0x38],
        'c' => [0x38, 0x44, 0x44, 0x44, 0x20],
        'd' => [0x38, 0x44, 0x44, 0x48, 0x7F],
        'e' => [0x38, 0x54, 0x54, 0x54, 0x18],
        'f' => [0x08, 0x7E, 0x09, 0x01, 0x02],
        'g' => [0x0C, 0x52, 0x52, 0x52, 0x3E],
        'h' => [0x7F, 0x08, 0x04, 0x04, 0x78],
        'i' => [0x00, 0x44, 0x7D, 0x40, 0x00],
        'j' => [0x20, 0x40, 0x44, 0x3D, 0x00],
        'k' => [0x7F, 0x10, 0x28, 0x44, 0x00],
        'l' => [0x00, 0x41, 0x7F, 0x40, 0x00],
        'm' => [0x7C, 0x04, 0x18, 0x04, 0x78],
        'n' => [0x7C, 0x08, 0x04, 0x04, 0x78],
        'o' => [0x38, 0x44, 0x44, 0x44, 0x38],
        'p' => [0x7C, 0x14, 0x14, 0x14, 0x08],
        'q' => [0x08, 0x14, 0x14, 0x18, 0x7C],
        'r' => [0x7C, 0x08, 0x04, 0x04, 0x08],
        's' => [0x48, 0x54, 0x54, 0x54, 0x20],
        't' => [0x04, 0x3F, 0x44, 0x40, 0x20],
        'u' => [0x3C, 0x40, 0x40, 0x20, 0x7C],
        'v' => [0x1C, 0x20, 0x40, 0x20, 0x1C],
        'w' => [0x3C, 0x40, 0x30, 0x40, 0x3C],
        'x' => [0x44, 0x28, 0x10, 0x28, 0x44],
        'y' => [0x0C, 0x50, 0x50, 0x50, 0x3C],
        'z' => [0x44, 0x64, 0x54, 0x4C, 0x44],
        '0' => [0x3E, 0x51, 0x49, 0x45, 0x3E],
        '1' => [0x00, 0x42, 0x7F, 0x40, 0x00],
        '2' => [0x42, 0x61, 0x51, 0x49, 0x46],
        '3' => [0x21, 0x41, 0x45, 0x4B, 0x31],
        '4' => [0x18, 0x14, 0x12, 0x7F, 0x10],
        '5' => [0x27, 0x45, 0x45, 0x45, 0x39],
        '6' => [0x3C, 0x4A, 0x49, 0x49, 0x30],
        '7' => [0x01, 0x71, 0x09, 0x05, 0x03],
        '8' => [0x36, 0x49, 0x49, 0x49, 0x36],
        '9' => [0x06, 0x49, 0x49, 0x29, 0x1E],
        '.' => [0x00, 0x60, 0x60, 0x00, 0x00],
        ',' => [0x00, 0x50, 0x30, 0x00, 0x00],
        '!' => [0x00, 0x00, 0x7D, 0x00, 0x00],
        '?' => [0x02, 0x01, 0x51, 0x09, 0x06],
        ':' => [0x00, 0x36, 0x36, 0x00, 0x00],
        '\'' => [0x00, 0x03, 0x00, 0x00, 0x00],
        '-' => [0x08, 0x08, 0x08, 0x08, 0x08],
        '>' => [0x41, 0x22, 0x14, 0x08, 0x00],
        '<' => [0x00, 0x08, 0x14, 0x22, 0x41],
        '#' => [0x14, 0x7F, 0x14, 0x7F, 0x14],
        '/' => [0x20, 0x10, 0x08, 0x04, 0x02],
        ' ' => [0x00, 0x00, 0x00, 0x00, 0x00],
        _   => [0x7F, 0x41, 0x41, 0x41, 0x7F], // fallback: box
    }
}

fn draw_char(engine: &mut Engine, ch: char, px: i32, py: i32, color: Color) {
    let bitmap = char_bitmap(ch);
    let fw = engine.framebuffer.width as i32;
    let fh = engine.framebuffer.height as i32;
    for (col, &byte) in bitmap.iter().enumerate() {
        for row in 0..7i32 {
            if byte & (1 << row) != 0 {
                let x = px + col as i32;
                let y = py + row;
                if x >= 0 && y >= 0 && x < fw && y < fh {
                    engine.framebuffer.set_pixel(x, y, color);
                }
            }
        }
    }
}

