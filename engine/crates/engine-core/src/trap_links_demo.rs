/// DEMO MODULE: S-League — Minigolf RPG
///
/// Two modes:
/// - OVERWORLD: Pokémon-like tile world. Tap to move. Wild areas trigger encounters.
/// - FIGHT: Minigolf battle vs monsters. Slingshot aiming. Sink the ball to win.
///
/// Mobile-first 480x720 portrait. Tap anywhere to move in the overworld.
/// Encounters happen in tall grass (wild areas). Each monster has its own course.

use crate::engine::Engine;
use crate::tilemap::{TileMap, Tile, TileType};
use crate::rendering::color::Color;
use crate::rendering::shapes;
use crate::rendering::text;
use crate::aim_preview::{AimConfig, compute_arc};
use crate::rendering::screen_fx::ScreenEffect;

// ═══════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════

const TILE_SIZE: f64 = 16.0;
const MAP_W: usize = 30;
const MAP_H: usize = 37;
const COURSE_W: usize = 30;
const COURSE_H: usize = 37;

const WIDTH: f64 = 480.0;
const HEIGHT: f64 = 720.0;
const WORLD_H: f64 = COURSE_H as f64 * TILE_SIZE; // 592
const HUD_H: f64 = HEIGHT - WORLD_H; // 128

// Tilemap render camera: positions tiles so world coords = screen coords.
const TILEMAP_CAM_X: f64 = 240.0;
const TILEMAP_CAM_Y: f64 = 360.0;

// Game modes
const MODE_OVERWORLD: f64 = 0.0;
const MODE_FIGHT: f64 = 1.0;

// Overworld tile custom IDs
const TILE_GRASS: u16 = 0;
const _TILE_SAND_ID: u16 = 1;
const TILE_PATH: u16 = 2;
const TILE_WILD: u16 = 3;
const TILE_WATER: u16 = 4;

// Colors — overworld
const COL_BG: Color = Color { r: 34, g: 51, b: 34, a: 255 };
const COL_GRASS: Color = Color { r: 58, g: 94, b: 48, a: 255 };
const COL_GRASS_DARK: Color = Color { r: 48, g: 78, b: 38, a: 255 };
const COL_WILD_GRASS: Color = Color { r: 68, g: 120, b: 52, a: 255 };
const COL_WILD_GRASS2: Color = Color { r: 52, g: 100, b: 42, a: 255 };
const COL_PATH_COL: Color = Color { r: 180, g: 165, b: 130, a: 255 };
const COL_PATH_LIGHT: Color = Color { r: 195, g: 178, b: 140, a: 255 };
const COL_TREE: Color = Color { r: 36, g: 60, b: 28, a: 255 };
const COL_WATER_COL: Color = Color { r: 40, g: 90, b: 150, a: 255 };
const COL_WATER_LIGHT: Color = Color { r: 50, g: 110, b: 170, a: 255 };
const COL_PLAYER: Color = Color { r: 245, g: 245, b: 240, a: 255 };
const COL_PLAYER_RING: Color = Color { r: 60, g: 100, b: 180, a: 255 };

// Colors — fight
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
const COL_HP: Color = Color { r: 220, g: 60, b: 60, a: 255 };

// Physics
const BALL_RADIUS: f64 = 5.0;
const HOLE_RADIUS: f64 = 8.0;
const MAX_POWER: f64 = 400.0;
const DRAG: f64 = 1.8;
const RESTITUTION: f64 = 0.65;
const STOP_THRESHOLD: f64 = 8.0;

// Player movement
const MOVE_DURATION: f64 = 0.15;
const ENCOUNTER_CHANCE: f64 = 0.25;

// ═══════════════════════════════════════════════════════════════════════
// GAME STATE KEYS
// ═══════════════════════════════════════════════════════════════════════

const K_MODE: &str = "game_mode";

// Overworld
const K_PLAYER_TX: &str = "player_tx";
const K_PLAYER_TY: &str = "player_ty";
const K_PLAYER_WX: &str = "player_wx";
const K_PLAYER_WY: &str = "player_wy";
const K_PLAYER_TARGET_TX: &str = "player_target_tx";
const K_PLAYER_TARGET_TY: &str = "player_target_ty";
const K_PLAYER_MOVING: &str = "player_moving";
const K_PLAYER_MOVE_T: &str = "player_move_t";
const K_PLAYER_HP: &str = "player_hp";
const K_PLAYER_MAX_HP: &str = "player_max_hp";
const K_PLAYER_LVL: &str = "player_level";
const K_PLAYER_XP: &str = "player_xp";
const K_ENCOUNTERS_WON: &str = "encounters_won";
const K_STEPS: &str = "steps";

// Fight
const K_BALL_X: &str = "ball_x";
const K_BALL_Y: &str = "ball_y";
const K_BALL_VX: &str = "ball_vx";
const K_BALL_VY: &str = "ball_vy";
const K_STROKES: &str = "strokes";
const K_PHASE: &str = "tl_phase"; // 0=aiming, 1=moving, 2=sunk, 3=fail
const K_AIM_X: &str = "aim_x";
const K_AIM_Y: &str = "aim_y";
const K_AIM_ACTIVE: &str = "aim_active";
const K_RESULT_TIMER: &str = "result_timer";
const K_HOLE_X: &str = "hole_x";
const K_HOLE_Y: &str = "hole_y";
const K_START_X: &str = "start_x";
const K_START_Y: &str = "start_y";
const K_MONSTER_ID: &str = "monster_id";

// ═══════════════════════════════════════════════════════════════════════
// MONSTER DATA
// ═══════════════════════════════════════════════════════════════════════

struct MonsterInfo {
    name: &'static str,
    par: i32,
    color: Color,
    max_strokes: i32,
}

fn get_monster(id: i32) -> MonsterInfo {
    match id {
        0 => MonsterInfo { name: "MOLE", par: 3, color: Color { r: 139, g: 90, b: 43, a: 255 }, max_strokes: 6 },
        1 => MonsterInfo { name: "RABBIT", par: 2, color: Color { r: 200, g: 200, b: 200, a: 255 }, max_strokes: 5 },
        2 => MonsterInfo { name: "SCORPION", par: 4, color: Color { r: 180, g: 60, b: 60, a: 255 }, max_strokes: 7 },
        _ => MonsterInfo { name: "SLIME", par: 3, color: Color { r: 80, g: 180, b: 80, a: 255 }, max_strokes: 6 },
    }
}

// ═══════════════════════════════════════════════════════════════════════
// SETUP
// ═══════════════════════════════════════════════════════════════════════

/// Full game setup — starts in overworld mode.
pub fn setup(engine: &mut Engine) {
    engine.debug_mode = false;
    engine.config.name = "S-League".into();
    engine.config.bounds = (WIDTH, HEIGHT);
    engine.config.background = COL_BG;

    // Player stats
    engine.global_state.set_f64(K_MODE, MODE_OVERWORLD);
    engine.global_state.set_f64(K_PLAYER_HP, 5.0);
    engine.global_state.set_f64(K_PLAYER_MAX_HP, 5.0);
    engine.global_state.set_f64(K_PLAYER_LVL, 1.0);
    engine.global_state.set_f64(K_PLAYER_XP, 0.0);
    engine.global_state.set_f64(K_ENCOUNTERS_WON, 0.0);
    engine.global_state.set_f64(K_STEPS, 0.0);
    engine.global_state.set_f64(K_PLAYER_MOVING, 0.0);
    engine.global_state.set_f64(K_PLAYER_MOVE_T, 0.0);
    engine.global_state.set_f64(K_MONSTER_ID, 0.0);

    // Initialize fight keys (headless tests may read these)
    engine.global_state.set_f64(K_BALL_X, 0.0);
    engine.global_state.set_f64(K_BALL_Y, 0.0);
    engine.global_state.set_f64(K_BALL_VX, 0.0);
    engine.global_state.set_f64(K_BALL_VY, 0.0);
    engine.global_state.set_f64(K_STROKES, 0.0);
    engine.global_state.set_f64(K_PHASE, 0.0);
    engine.global_state.set_f64(K_AIM_ACTIVE, 0.0);
    engine.global_state.set_f64(K_RESULT_TIMER, 0.0);
    engine.global_state.set_f64(K_HOLE_X, 0.0);
    engine.global_state.set_f64(K_HOLE_Y, 0.0);
    engine.global_state.set_f64(K_START_X, 0.0);
    engine.global_state.set_f64(K_START_Y, 0.0);

    build_overworld_map(engine);

    // Player starts at town center
    let start_tx = 15usize;
    let start_ty = 18usize;
    let (wx, wy) = tile_center(start_tx, start_ty);
    engine.global_state.set_f64(K_PLAYER_TX, start_tx as f64);
    engine.global_state.set_f64(K_PLAYER_TY, start_ty as f64);
    engine.global_state.set_f64(K_PLAYER_WX, wx);
    engine.global_state.set_f64(K_PLAYER_WY, wy);
    engine.global_state.set_f64(K_PLAYER_TARGET_TX, start_tx as f64);
    engine.global_state.set_f64(K_PLAYER_TARGET_TY, start_ty as f64);

    engine.camera.x = TILEMAP_CAM_X;
    engine.camera.y = TILEMAP_CAM_Y;
    engine.camera.zoom = 1.0;
    engine.post_fx.vignette_strength = 0.2;
}

/// Fight-only setup — for headless tests and CLI simulate.
/// Starts directly in a fight (Mole, par 3) without overworld.
pub fn setup_fight_only(engine: &mut Engine) {
    engine.debug_mode = false;
    engine.config.name = "S-League".into();
    engine.config.bounds = (WIDTH, HEIGHT);
    engine.config.background = COL_BG;

    engine.global_state.set_f64(K_MODE, MODE_FIGHT);
    engine.global_state.set_f64(K_MONSTER_ID, 0.0);
    engine.global_state.set_f64(K_PLAYER_HP, 5.0);
    engine.global_state.set_f64(K_PLAYER_MAX_HP, 5.0);
    engine.global_state.set_f64(K_PLAYER_LVL, 1.0);
    engine.global_state.set_f64(K_PLAYER_XP, 0.0);
    engine.global_state.set_f64(K_ENCOUNTERS_WON, 0.0);

    build_fight_course(engine, 0);

    engine.camera.x = TILEMAP_CAM_X;
    engine.camera.y = TILEMAP_CAM_Y;
    engine.camera.zoom = 1.0;
    engine.post_fx.vignette_strength = 0.3;
}

// ═══════════════════════════════════════════════════════════════════════
// OVERWORLD MAP
// ═══════════════════════════════════════════════════════════════════════

fn build_overworld_map(engine: &mut Engine) {
    let mut tm = TileMap::new(MAP_W, MAP_H, TILE_SIZE);

    // Fill with checkerboard grass
    for y in 0..MAP_H {
        for x in 0..MAP_W {
            let col = if (x + y) % 2 == 0 { COL_GRASS } else { COL_GRASS_DARK };
            tm.set(x, y, Tile::custom(TILE_GRASS, col));
        }
    }

    // Border trees (2 tiles thick)
    for x in 0..MAP_W {
        for d in 0..2 {
            tm.set(x, d, Tile::solid(COL_TREE));
            tm.set(x, MAP_H - 1 - d, Tile::solid(COL_TREE));
        }
    }
    for y in 0..MAP_H {
        for d in 0..2 {
            tm.set(d, y, Tile::solid(COL_TREE));
            tm.set(MAP_W - 1 - d, y, Tile::solid(COL_TREE));
        }
    }

    // ── Town center (path area, 10x8 in the middle) ──
    let town_x = 10;
    let town_y = 14;
    let town_w = 10;
    let town_h = 8;

    for y in town_y..town_y + town_h {
        for x in town_x..town_x + town_w {
            let col = if (x + y) % 2 == 0 { COL_PATH_COL } else { COL_PATH_LIGHT };
            tm.set(x, y, Tile::custom(TILE_PATH, col));
        }
    }

    // Town buildings
    for y in town_y + 1..town_y + 3 {
        for x in town_x + 1..town_x + 3 {
            tm.set(x, y, Tile::solid(COL_WALL));
        }
    }
    for y in town_y + 1..town_y + 3 {
        for x in town_x + town_w - 3..town_x + town_w - 1 {
            tm.set(x, y, Tile::solid(COL_WALL_LIGHT));
        }
    }

    // ── North route (vertical path with wild grass on sides) ──
    let route_x = 14;
    for y in 3..town_y {
        tm.set(route_x, y, Tile::custom(TILE_PATH, COL_PATH_COL));
        tm.set(route_x + 1, y, Tile::custom(TILE_PATH, COL_PATH_LIGHT));
        for &dx in &[-2i32, -1, 2, 3] {
            let wx = (route_x as i32 + dx) as usize;
            if wx >= 2 && wx < MAP_W - 2 {
                let col = if (wx + y) % 2 == 0 { COL_WILD_GRASS } else { COL_WILD_GRASS2 };
                tm.set(wx, y, Tile::custom(TILE_WILD, col));
            }
        }
    }

    // ── South route ──
    for y in town_y + town_h..MAP_H - 3 {
        tm.set(route_x, y, Tile::custom(TILE_PATH, COL_PATH_COL));
        tm.set(route_x + 1, y, Tile::custom(TILE_PATH, COL_PATH_LIGHT));
        for &dx in &[-2i32, -1, 2, 3] {
            let wx = (route_x as i32 + dx) as usize;
            if wx >= 2 && wx < MAP_W - 2 {
                let col = if (wx + y) % 2 == 0 { COL_WILD_GRASS } else { COL_WILD_GRASS2 };
                tm.set(wx, y, Tile::custom(TILE_WILD, col));
            }
        }
    }

    // ── East route (horizontal) ──
    let route_y = 17;
    for x in town_x + town_w..MAP_W - 3 {
        tm.set(x, route_y, Tile::custom(TILE_PATH, COL_PATH_COL));
        tm.set(x, route_y + 1, Tile::custom(TILE_PATH, COL_PATH_LIGHT));
        for &dy in &[-2i32, -1, 2, 3] {
            let wy = (route_y as i32 + dy) as usize;
            if wy >= 2 && wy < MAP_H - 2 {
                let col = if (x + wy) % 2 == 0 { COL_WILD_GRASS } else { COL_WILD_GRASS2 };
                tm.set(x, wy, Tile::custom(TILE_WILD, col));
            }
        }
    }

    // ── West route ──
    for x in 3..town_x {
        tm.set(x, route_y, Tile::custom(TILE_PATH, COL_PATH_COL));
        tm.set(x, route_y + 1, Tile::custom(TILE_PATH, COL_PATH_LIGHT));
        for &dy in &[-2i32, -1, 2, 3] {
            let wy = (route_y as i32 + dy) as usize;
            if wy >= 2 && wy < MAP_H - 2 {
                let col = if (x + wy) % 2 == 0 { COL_WILD_GRASS } else { COL_WILD_GRASS2 };
                tm.set(x, wy, Tile::custom(TILE_WILD, col));
            }
        }
    }

    // ── Pond (northeast) ──
    for y in 4..8 {
        for x in 22..27 {
            let dx = x as f64 - 24.0;
            let dy = y as f64 - 5.5;
            if dx * dx + dy * dy < 6.0 {
                let col = if (x + y) % 2 == 0 { COL_WATER_COL } else { COL_WATER_LIGHT };
                tm.set(x, y, Tile::custom(TILE_WATER, col));
            }
        }
    }

    // ── Extra tree clusters ──
    for &(tx, ty) in &[(6, 6), (7, 6), (6, 7), (24, 30), (25, 30), (25, 31), (8, 28), (9, 28)] {
        tm.set(tx, ty, Tile::solid(COL_TREE));
    }

    engine.tilemap = Some(tm);
}

fn tile_center(tx: usize, ty: usize) -> (f64, f64) {
    (tx as f64 * TILE_SIZE + TILE_SIZE * 0.5, ty as f64 * TILE_SIZE + TILE_SIZE * 0.5)
}

fn is_walkable_tile(tm: &TileMap, tx: usize, ty: usize) -> bool {
    if let Some(tile) = tm.get(tx, ty) {
        !matches!(tile.tile_type, TileType::Solid | TileType::Custom(TILE_WATER))
    } else {
        false
    }
}

fn is_wild_grass(tm: &TileMap, tx: usize, ty: usize) -> bool {
    tm.get(tx, ty).map_or(false, |t| matches!(t.tile_type, TileType::Custom(TILE_WILD)))
}

fn pseudo_random(seed: u64) -> f64 {
    let h = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    ((h >> 33) as u32) as f64 / u32::MAX as f64
}

// ═══════════════════════════════════════════════════════════════════════
// FIGHT COURSE
// ═══════════════════════════════════════════════════════════════════════

fn build_fight_course(engine: &mut Engine, monster_id: i32) {
    let mut tm = TileMap::new(COURSE_W, COURSE_H, TILE_SIZE);

    // Fairway fill
    for y in 0..COURSE_H {
        for x in 0..COURSE_W {
            tm.set(x, y, Tile::custom(0, COL_FAIRWAY));
        }
    }

    // Border walls
    for x in 0..COURSE_W {
        tm.set(x, 0, Tile::solid(COL_WALL));
        tm.set(x, 1, Tile::solid(COL_WALL));
        tm.set(x, COURSE_H - 1, Tile::solid(COL_WALL));
        tm.set(x, COURSE_H - 2, Tile::solid(COL_WALL));
    }
    for y in 0..COURSE_H {
        tm.set(0, y, Tile::solid(COL_WALL));
        tm.set(1, y, Tile::solid(COL_WALL));
        tm.set(COURSE_W - 1, y, Tile::solid(COL_WALL));
        tm.set(COURSE_W - 2, y, Tile::solid(COL_WALL));
    }

    // Monster-specific obstacles
    match monster_id {
        0 => {
            // Mole: L-shaped wall + right wall
            for x in 8..16 { for y in 12..14 { tm.set(x, y, Tile::solid(COL_WALL_LIGHT)); } }
            for y in 12..22 { for x in 14..16 { tm.set(x, y, Tile::solid(COL_WALL_LIGHT)); } }
            for y in 18..26 { for x in 22..24 { tm.set(x, y, Tile::solid(COL_WALL_LIGHT)); } }
            for y in 24..28 { for x in 6..12 { tm.set(x, y, Tile::custom(1, COL_SAND)); } }
        }
        1 => {
            // Rabbit: Wide open, few small obstacles
            for x in 12..14 { for y in 18..20 { tm.set(x, y, Tile::solid(COL_WALL_LIGHT)); } }
            for x in 18..20 { for y in 10..12 { tm.set(x, y, Tile::solid(COL_WALL_LIGHT)); } }
        }
        2 => {
            // Scorpion: Corridor walls + heavy sand
            for y in 8..30 { tm.set(10, y, Tile::solid(COL_WALL_LIGHT)); tm.set(20, y, Tile::solid(COL_WALL_LIGHT)); }
            // Openings
            for y in 18..20 { tm.set(10, y, Tile::custom(0, COL_FAIRWAY)); }
            for y in 14..16 { tm.set(20, y, Tile::custom(0, COL_FAIRWAY)); }
            // Sand traps
            for y in 10..16 { for x in 4..9 { tm.set(x, y, Tile::custom(1, COL_SAND)); } }
            for y in 20..28 { for x in 22..27 { tm.set(x, y, Tile::custom(1, COL_SAND)); } }
            for y in 12..20 { for x in 12..18 { tm.set(x, y, Tile::custom(1, COL_SAND)); } }
        }
        _ => {
            // Default: simple wall
            for y in 14..20 { tm.set(15, y, Tile::solid(COL_WALL_LIGHT)); }
        }
    }

    engine.tilemap = Some(tm);

    // Ball start (bottom area)
    let start_x = 15.0 * TILE_SIZE;
    let start_y = 32.0 * TILE_SIZE;

    // Hole position varies by monster
    let (hole_x, hole_y) = match monster_id {
        0 => (20.0 * TILE_SIZE, 7.0 * TILE_SIZE),
        1 => (15.0 * TILE_SIZE, 5.0 * TILE_SIZE),
        2 => (15.0 * TILE_SIZE, 4.0 * TILE_SIZE),
        _ => (18.0 * TILE_SIZE, 8.0 * TILE_SIZE),
    };

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
}

fn start_fight(engine: &mut Engine, monster_id: i32) {
    engine.global_state.set_f64(K_MODE, MODE_FIGHT);
    engine.global_state.set_f64(K_MONSTER_ID, monster_id as f64);
    build_fight_course(engine, monster_id);
    engine.post_fx.vignette_strength = 0.3;
    engine.screen_fx.push(ScreenEffect::Flash { color: Color::WHITE, intensity: 1.0 }, 0.3);
}

fn end_fight(engine: &mut Engine, won: bool) {
    let monster_id = engine.global_state.get_f64(K_MONSTER_ID).unwrap_or(0.0) as i32;
    let monster = get_monster(monster_id);
    let strokes = engine.global_state.get_f64(K_STROKES).unwrap_or(0.0) as i32;

    if won {
        let wins = engine.global_state.get_f64(K_ENCOUNTERS_WON).unwrap_or(0.0);
        engine.global_state.set_f64(K_ENCOUNTERS_WON, wins + 1.0);

        let diff = strokes - monster.par;
        let xp_mult = match diff { d if d <= -2 => 3.0, -1 => 2.0, 0 => 1.0, 1 => 0.5, _ => 0.25 };
        let base_xp = match monster_id { 0 => 10.0, 1 => 8.0, 2 => 15.0, _ => 10.0 };
        let xp = engine.global_state.get_f64(K_PLAYER_XP).unwrap_or(0.0);
        engine.global_state.set_f64(K_PLAYER_XP, xp + base_xp * xp_mult);

        // HP effect based on performance
        let hp = engine.global_state.get_f64(K_PLAYER_HP).unwrap_or(5.0);
        if diff > 0 {
            engine.global_state.set_f64(K_PLAYER_HP, (hp - diff.min(3) as f64).max(0.0));
        } else if diff <= -2 {
            let max_hp = engine.global_state.get_f64(K_PLAYER_MAX_HP).unwrap_or(5.0);
            engine.global_state.set_f64(K_PLAYER_HP, (hp + 1.0).min(max_hp));
        }
    } else {
        let hp = engine.global_state.get_f64(K_PLAYER_HP).unwrap_or(5.0);
        engine.global_state.set_f64(K_PLAYER_HP, (hp - 3.0).max(0.0));
    }

    // Level up check
    let xp = engine.global_state.get_f64(K_PLAYER_XP).unwrap_or(0.0);
    let level = engine.global_state.get_f64(K_PLAYER_LVL).unwrap_or(1.0);
    if xp >= level * level * 50.0 {
        engine.global_state.set_f64(K_PLAYER_LVL, level + 1.0);
        if (level as i32 + 1) % 2 == 0 {
            let max_hp = engine.global_state.get_f64(K_PLAYER_MAX_HP).unwrap_or(5.0);
            engine.global_state.set_f64(K_PLAYER_MAX_HP, max_hp + 1.0);
        }
    }

    // HP death check → full heal (roguelike mercy)
    let hp = engine.global_state.get_f64(K_PLAYER_HP).unwrap_or(0.0);
    if hp <= 0.0 {
        let max_hp = engine.global_state.get_f64(K_PLAYER_MAX_HP).unwrap_or(5.0);
        engine.global_state.set_f64(K_PLAYER_HP, max_hp);
    }

    // Save and restore player position across map rebuild
    let saved_tx = engine.global_state.get_f64(K_PLAYER_TX).unwrap_or(15.0) as usize;
    let saved_ty = engine.global_state.get_f64(K_PLAYER_TY).unwrap_or(18.0) as usize;

    engine.global_state.set_f64(K_MODE, MODE_OVERWORLD);
    build_overworld_map(engine);

    // Restore player position
    let (wx, wy) = tile_center(saved_tx, saved_ty);
    engine.global_state.set_f64(K_PLAYER_TX, saved_tx as f64);
    engine.global_state.set_f64(K_PLAYER_TY, saved_ty as f64);
    engine.global_state.set_f64(K_PLAYER_WX, wx);
    engine.global_state.set_f64(K_PLAYER_WY, wy);
    engine.global_state.set_f64(K_PLAYER_MOVING, 0.0);

    engine.post_fx.vignette_strength = 0.2;
    engine.screen_fx.push(ScreenEffect::Flash { color: Color::WHITE, intensity: 1.0 }, 0.3);
}

// ═══════════════════════════════════════════════════════════════════════
// INPUT
// ═══════════════════════════════════════════════════════════════════════

pub fn on_pointer_down(engine: &mut Engine, x: f64, y: f64) {
    let mode = engine.global_state.get_f64(K_MODE).unwrap_or(0.0);
    if mode == MODE_OVERWORLD {
        overworld_pointer_down(engine, x, y);
    } else {
        fight_pointer_down(engine, x, y);
    }
}

pub fn on_pointer_move(engine: &mut Engine, x: f64, y: f64) {
    let mode = engine.global_state.get_f64(K_MODE).unwrap_or(0.0);
    if mode == MODE_FIGHT {
        let active = engine.global_state.get_f64(K_AIM_ACTIVE).unwrap_or(0.0);
        if active > 0.5 {
            engine.global_state.set_f64(K_AIM_X, x);
            engine.global_state.set_f64(K_AIM_Y, y);
        }
    }
}

pub fn on_pointer_up(engine: &mut Engine, x: f64, y: f64) {
    let mode = engine.global_state.get_f64(K_MODE).unwrap_or(0.0);
    if mode == MODE_FIGHT {
        fight_pointer_up(engine, x, y);
    }
}

// ── Overworld input: tap to move one tile in cardinal direction ──

fn overworld_pointer_down(engine: &mut Engine, x: f64, y: f64) {
    let moving = engine.global_state.get_f64(K_PLAYER_MOVING).unwrap_or(0.0);
    if moving > 0.5 { return; }

    let ptx = engine.global_state.get_f64(K_PLAYER_TX).unwrap_or(0.0) as usize;
    let pty = engine.global_state.get_f64(K_PLAYER_TY).unwrap_or(0.0) as usize;
    let (pwx, pwy) = tile_center(ptx, pty);

    // Pointer coords = screen coords = world coords in our setup
    let dx = x - pwx;
    let dy = y - pwy;

    let (target_tx, target_ty) = if dx.abs() > dy.abs() {
        if dx > 0.0 { (ptx + 1, pty) }
        else if ptx > 0 { (ptx - 1, pty) }
        else { return; }
    } else {
        if dy > 0.0 { (ptx, pty + 1) }
        else if pty > 0 { (ptx, pty - 1) }
        else { return; }
    };

    let walkable = engine.tilemap.as_ref().map_or(false, |tm| is_walkable_tile(tm, target_tx, target_ty));
    if !walkable { return; }

    engine.global_state.set_f64(K_PLAYER_TARGET_TX, target_tx as f64);
    engine.global_state.set_f64(K_PLAYER_TARGET_TY, target_ty as f64);
    engine.global_state.set_f64(K_PLAYER_MOVING, 1.0);
    engine.global_state.set_f64(K_PLAYER_MOVE_T, 0.0);
}

// ── Fight input ──

fn fight_pointer_down(engine: &mut Engine, x: f64, y: f64) {
    let phase = engine.global_state.get_f64(K_PHASE).unwrap_or(0.0);
    if phase == 2.0 || phase == 3.0 {
        let timer = engine.global_state.get_f64(K_RESULT_TIMER).unwrap_or(0.0);
        if timer > 1.5 {
            end_fight(engine, phase == 2.0);
        }
        return;
    }
    if phase != 0.0 { return; }

    engine.global_state.set_f64(K_AIM_ACTIVE, 1.0);
    engine.global_state.set_f64(K_AIM_X, x);
    engine.global_state.set_f64(K_AIM_Y, y);
}

fn fight_pointer_up(engine: &mut Engine, x: f64, y: f64) {
    let active = engine.global_state.get_f64(K_AIM_ACTIVE).unwrap_or(0.0);
    if active < 0.5 { return; }
    engine.global_state.set_f64(K_AIM_ACTIVE, 0.0);

    let phase = engine.global_state.get_f64(K_PHASE).unwrap_or(0.0);
    if phase != 0.0 { return; }

    let ball_x = engine.global_state.get_f64(K_BALL_X).unwrap_or(0.0);
    let ball_y = engine.global_state.get_f64(K_BALL_Y).unwrap_or(0.0);

    let dx = ball_x - x;
    let dy = ball_y - y;
    let dist = (dx * dx + dy * dy).sqrt();
    if dist < 5.0 { return; }

    let power = (dist / 120.0).min(1.0) * MAX_POWER;
    let angle = dy.atan2(dx);

    engine.global_state.set_f64(K_BALL_VX, angle.cos() * power);
    engine.global_state.set_f64(K_BALL_VY, angle.sin() * power);
    engine.global_state.set_f64(K_PHASE, 1.0);

    let strokes = engine.global_state.get_f64(K_STROKES).unwrap_or(0.0);
    engine.global_state.set_f64(K_STROKES, strokes + 1.0);
}

// ═══════════════════════════════════════════════════════════════════════
// UPDATE
// ═══════════════════════════════════════════════════════════════════════

pub fn update(engine: &mut Engine, dt: f64) {
    let mode = engine.global_state.get_f64(K_MODE).unwrap_or(0.0);
    if mode == MODE_OVERWORLD { overworld_update(engine, dt); }
    else { fight_update(engine, dt); }
}

fn overworld_update(engine: &mut Engine, dt: f64) {
    let moving = engine.global_state.get_f64(K_PLAYER_MOVING).unwrap_or(0.0);
    if moving < 0.5 { return; }

    let t = engine.global_state.get_f64(K_PLAYER_MOVE_T).unwrap_or(0.0) + dt;
    let progress = (t / MOVE_DURATION).min(1.0);

    let from_tx = engine.global_state.get_f64(K_PLAYER_TX).unwrap_or(0.0) as usize;
    let from_ty = engine.global_state.get_f64(K_PLAYER_TY).unwrap_or(0.0) as usize;
    let to_tx = engine.global_state.get_f64(K_PLAYER_TARGET_TX).unwrap_or(0.0) as usize;
    let to_ty = engine.global_state.get_f64(K_PLAYER_TARGET_TY).unwrap_or(0.0) as usize;

    let (from_wx, from_wy) = tile_center(from_tx, from_ty);
    let (to_wx, to_wy) = tile_center(to_tx, to_ty);

    // Ease in-out
    let ease = if progress < 0.5 { 2.0 * progress * progress }
               else { 1.0 - (-2.0 * progress + 2.0).powi(2) / 2.0 };

    engine.global_state.set_f64(K_PLAYER_WX, from_wx + (to_wx - from_wx) * ease);
    engine.global_state.set_f64(K_PLAYER_WY, from_wy + (to_wy - from_wy) * ease);
    engine.global_state.set_f64(K_PLAYER_MOVE_T, t);

    if progress >= 1.0 {
        engine.global_state.set_f64(K_PLAYER_TX, to_tx as f64);
        engine.global_state.set_f64(K_PLAYER_TY, to_ty as f64);
        engine.global_state.set_f64(K_PLAYER_WX, to_wx);
        engine.global_state.set_f64(K_PLAYER_WY, to_wy);
        engine.global_state.set_f64(K_PLAYER_MOVING, 0.0);

        let steps = engine.global_state.get_f64(K_STEPS).unwrap_or(0.0);
        engine.global_state.set_f64(K_STEPS, steps + 1.0);

        // Encounter check on wild grass
        let is_wild = engine.tilemap.as_ref().map_or(false, |tm| is_wild_grass(tm, to_tx, to_ty));
        if is_wild {
            let seed = (steps as u64 + 1) ^ ((to_tx as u64) << 16 | to_ty as u64) ^ 0xDEAD_BEEF;
            if pseudo_random(seed) < ENCOUNTER_CHANCE {
                let monster_id = ((steps as u64 + 1) % 3) as i32;
                start_fight(engine, monster_id);
            }
        }
    }
}

fn fight_update(engine: &mut Engine, dt: f64) {
    let phase = engine.global_state.get_f64(K_PHASE).unwrap_or(0.0);
    let monster_id = engine.global_state.get_f64(K_MONSTER_ID).unwrap_or(0.0) as i32;
    let monster = get_monster(monster_id);

    if phase == 2.0 || phase == 3.0 {
        let t = engine.global_state.get_f64(K_RESULT_TIMER).unwrap_or(0.0) + dt;
        engine.global_state.set_f64(K_RESULT_TIMER, t);
        return;
    }

    if phase != 1.0 { return; }

    let mut bx = engine.global_state.get_f64(K_BALL_X).unwrap_or(0.0);
    let mut by = engine.global_state.get_f64(K_BALL_Y).unwrap_or(0.0);
    let mut vx = engine.global_state.get_f64(K_BALL_VX).unwrap_or(0.0);
    let mut vy = engine.global_state.get_f64(K_BALL_VY).unwrap_or(0.0);
    let hole_x = engine.global_state.get_f64(K_HOLE_X).unwrap_or(0.0);
    let hole_y = engine.global_state.get_f64(K_HOLE_Y).unwrap_or(0.0);

    let steps = 4;
    let sub_dt = dt / steps as f64;

    for _ in 0..steps {
        let mut drag = DRAG;
        if let Some(ref tm) = engine.tilemap {
            if let Some((tx, ty)) = tm.world_to_tile(bx, by) {
                if let Some(tile) = tm.get(tx, ty) {
                    if let TileType::Custom(1) = tile.tile_type { drag = DRAG * 3.0; }
                }
            }
        }

        vx -= vx * drag * sub_dt;
        vy -= vy * drag * sub_dt;
        let new_x = bx + vx * sub_dt;
        let new_y = by + vy * sub_dt;

        let solid_at = |wx: f64, wy: f64| -> bool {
            if let Some(ref tm) = engine.tilemap {
                tm.is_solid_at_world(wx, wy)
                    || tm.is_solid_at_world(wx - BALL_RADIUS, wy)
                    || tm.is_solid_at_world(wx + BALL_RADIUS, wy)
                    || tm.is_solid_at_world(wx, wy - BALL_RADIUS)
                    || tm.is_solid_at_world(wx, wy + BALL_RADIUS)
            } else { false }
        };

        if solid_at(new_x, new_y) {
            let sx = solid_at(new_x, by);
            let sy = solid_at(bx, new_y);
            match (sx, sy) {
                (true, true) | (false, false) => { vx = -vx * RESTITUTION; vy = -vy * RESTITUTION; }
                (true, false) => { vx = -vx * RESTITUTION; by = new_y; }
                (false, true) => { bx = new_x; vy = -vy * RESTITUTION; }
            }
        } else {
            bx = new_x;
            by = new_y;
        }

        let dx = bx - hole_x;
        let dy = by - hole_y;
        let dist = (dx * dx + dy * dy).sqrt();
        let speed = (vx * vx + vy * vy).sqrt();

        if dist < HOLE_RADIUS && speed < 250.0 {
            engine.global_state.set_f64(K_PHASE, 2.0);
            engine.global_state.set_f64(K_RESULT_TIMER, 0.0);
            engine.global_state.set_f64(K_BALL_X, hole_x);
            engine.global_state.set_f64(K_BALL_Y, hole_y);
            engine.global_state.set_f64(K_BALL_VX, 0.0);
            engine.global_state.set_f64(K_BALL_VY, 0.0);
            engine.post_fx.shake_remaining = 0.3;
            engine.post_fx.shake_intensity = 4.0;
            return;
        }
    }

    let speed = (vx * vx + vy * vy).sqrt();
    if speed < STOP_THRESHOLD {
        vx = 0.0;
        vy = 0.0;
        engine.global_state.set_f64(K_PHASE, 0.0);
        let strokes = engine.global_state.get_f64(K_STROKES).unwrap_or(0.0) as i32;
        if strokes >= monster.max_strokes {
            engine.global_state.set_f64(K_PHASE, 3.0);
            engine.global_state.set_f64(K_RESULT_TIMER, 0.0);
        }
    }

    engine.global_state.set_f64(K_BALL_X, bx);
    engine.global_state.set_f64(K_BALL_Y, by);
    engine.global_state.set_f64(K_BALL_VX, vx);
    engine.global_state.set_f64(K_BALL_VY, vy);
}

// ═══════════════════════════════════════════════════════════════════════
// RENDER
// ═══════════════════════════════════════════════════════════════════════

pub fn render(engine: &mut Engine) {
    let mode = engine.global_state.get_f64(K_MODE).unwrap_or(0.0);
    if mode == MODE_OVERWORLD { render_overworld(engine); }
    else { render_fight(engine); }
}

fn render_overworld(engine: &mut Engine) {
    let fb = &mut engine.framebuffer;
    let w = fb.width;
    let h = fb.height;
    fb.clear(COL_BG);

    if let Some(ref tm) = engine.tilemap {
        tm.render(fb, TILEMAP_CAM_X, TILEMAP_CAM_Y, 1.0, w, h);
    }

    let wx = engine.global_state.get_f64(K_PLAYER_WX).unwrap_or(0.0);
    let wy = engine.global_state.get_f64(K_PLAYER_WY).unwrap_or(0.0);

    // Draw tall grass blade decorations on wild tiles
    if let Some(ref tm) = engine.tilemap {
        let ptx = engine.global_state.get_f64(K_PLAYER_TX).unwrap_or(0.0) as i32;
        let pty = engine.global_state.get_f64(K_PLAYER_TY).unwrap_or(0.0) as i32;
        let blade_col = Color::from_rgba(35, 70, 30, 200);
        for dy in -14i32..15 {
            for dx in -16i32..17 {
                let tx = ptx + dx;
                let ty = pty + dy;
                if tx < 0 || ty < 0 || tx >= MAP_W as i32 || ty >= MAP_H as i32 { continue; }
                if is_wild_grass(tm, tx as usize, ty as usize) {
                    let (twx, twy) = tile_center(tx as usize, ty as usize);
                    shapes::draw_line(fb, twx - 3.0, twy + 4.0, twx - 1.0, twy - 2.0, blade_col);
                    shapes::draw_line(fb, twx + 2.0, twy + 4.0, twx + 4.0, twy - 1.0, blade_col);
                }
            }
        }
    }

    // Player shadow + body + highlight
    shapes::fill_circle(fb, wx + 1.0, wy + 2.0, 7.0, Color::from_rgba(0, 0, 0, 60));
    shapes::fill_circle(fb, wx, wy, 7.0, COL_PLAYER_RING);
    shapes::fill_circle(fb, wx, wy, 5.5, COL_PLAYER);
    shapes::fill_circle(fb, wx - 1.5, wy - 2.0, 2.0, Color::from_rgba(255, 255, 255, 140));

    // ── HUD ──
    let hud_y = WORLD_H;
    shapes::fill_rect(fb, 0.0, hud_y, WIDTH, HUD_H, COL_HUD_BG);
    shapes::draw_line(fb, 0.0, hud_y, WIDTH, hud_y, Color::from_rgba(80, 80, 90, 255));

    text::draw_text(fb, 16, (hud_y + 12.0) as i32, "S-LEAGUE", COL_TEXT, 2);

    // HP pips
    let hp = engine.global_state.get_f64(K_PLAYER_HP).unwrap_or(5.0) as i32;
    let max_hp = engine.global_state.get_f64(K_PLAYER_MAX_HP).unwrap_or(5.0) as i32;
    text::draw_text(fb, 16, (hud_y + 42.0) as i32, "HP", COL_TEXT_DIM, 1);
    for i in 0..max_hp {
        let px = 50.0 + i as f64 * 16.0;
        let col = if i < hp { COL_HP } else { Color::from_rgba(60, 60, 60, 255) };
        shapes::fill_rect(fb, px, hud_y + 42.0, 12.0, 10.0, col);
        shapes::draw_rect(fb, px, hud_y + 42.0, 12.0, 10.0, COL_TEXT_DIM);
    }

    // Level + XP
    let level = engine.global_state.get_f64(K_PLAYER_LVL).unwrap_or(1.0) as i32;
    let xp = engine.global_state.get_f64(K_PLAYER_XP).unwrap_or(0.0) as i32;
    text::draw_text(fb, 16, (hud_y + 60.0) as i32, &format!("LV {} XP {}", level, xp), COL_TEXT_DIM, 1);

    // Encounters won + steps
    let wins = engine.global_state.get_f64(K_ENCOUNTERS_WON).unwrap_or(0.0) as i32;
    let steps = engine.global_state.get_f64(K_STEPS).unwrap_or(0.0) as i32;
    text::draw_text(fb, 330, (hud_y + 12.0) as i32, &format!("WINS {}", wins), COL_SUCCESS, 1);
    text::draw_text(fb, 330, (hud_y + 28.0) as i32, &format!("STEPS {}", steps), COL_TEXT_DIM, 1);

    // Instructions
    text::draw_text_centered(fb, 240, (hud_y + 100.0) as i32, "TAP TO MOVE - WILD GRASS = ENCOUNTERS!", COL_TEXT_DIM, 1);

    engine.screen_fx.tick(0.016);
    engine.screen_fx.apply(fb);
    crate::rendering::post_fx::apply(fb, &mut engine.post_fx, 0.016, engine.frame);
}

fn render_fight(engine: &mut Engine) {
    let fb = &mut engine.framebuffer;
    let w = fb.width;
    let h = fb.height;
    fb.clear(COL_BG);

    if let Some(ref tm) = engine.tilemap {
        tm.render(fb, TILEMAP_CAM_X, TILEMAP_CAM_Y, 1.0, w, h);
    }

    let ball_x = engine.global_state.get_f64(K_BALL_X).unwrap_or(0.0);
    let ball_y = engine.global_state.get_f64(K_BALL_Y).unwrap_or(0.0);
    let hole_x = engine.global_state.get_f64(K_HOLE_X).unwrap_or(0.0);
    let hole_y = engine.global_state.get_f64(K_HOLE_Y).unwrap_or(0.0);
    let phase = engine.global_state.get_f64(K_PHASE).unwrap_or(0.0);
    let strokes = engine.global_state.get_f64(K_STROKES).unwrap_or(0.0) as i32;
    let monster_id = engine.global_state.get_f64(K_MONSTER_ID).unwrap_or(0.0) as i32;
    let monster = get_monster(monster_id);

    // Hole + monster indicator
    shapes::fill_circle(fb, hole_x, hole_y, HOLE_RADIUS + 3.0, COL_HOLE_RIM);
    shapes::fill_circle(fb, hole_x, hole_y, HOLE_RADIUS, COL_HOLE);
    shapes::fill_circle(fb, hole_x, hole_y - 22.0, 8.0, monster.color);
    shapes::draw_circle(fb, hole_x, hole_y - 22.0, 8.0, COL_TEXT_DIM);

    // Flag
    shapes::draw_line(fb, hole_x + 2.0, hole_y, hole_x + 2.0, hole_y - 20.0, COL_TEXT_DIM);
    for i in 0..8 {
        let fy = hole_y - 20.0 + i as f64;
        shapes::draw_line(fb, hole_x + 3.0, fy, hole_x + 3.0 + (8.0 - i as f64), fy, COL_FLAG);
    }

    // Aim preview
    let aim_active = engine.global_state.get_f64(K_AIM_ACTIVE).unwrap_or(0.0);
    if aim_active > 0.5 && phase == 0.0 {
        let aim_x = engine.global_state.get_f64(K_AIM_X).unwrap_or(0.0);
        let aim_y = engine.global_state.get_f64(K_AIM_Y).unwrap_or(0.0);
        let dx = ball_x - aim_x;
        let dy = ball_y - aim_y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > 5.0 {
            let power = (dist / 120.0).min(1.0) * MAX_POWER;
            let angle = dy.atan2(dx);
            let vx = angle.cos() * power;
            let vy = angle.sin() * power;

            let config = AimConfig {
                dot_count: 8, time_step: 0.08, drag: DRAG * 0.08,
                gravity_y: 0.0, ball_radius: BALL_RADIUS, restitution: RESTITUTION,
            };

            let tm_ref = engine.tilemap.as_ref();
            let points = compute_arc(ball_x, ball_y, vx, vy, &config, |wx, wy| {
                tm_ref.map_or(false, |tm| tm.is_solid_at_world(wx, wy))
            });

            for (i, pt) in points.iter().enumerate() {
                let alpha = 200 - (i as u8 * 20).min(180);
                let dot_col = Color::from_rgba(COL_AIM_DOT.r, COL_AIM_DOT.g, COL_AIM_DOT.b, alpha);
                shapes::fill_circle(fb, pt.x, pt.y, 3.0 - (i as f64 * 0.3).min(2.0), dot_col);
            }

            shapes::draw_line(fb, ball_x, ball_y, ball_x + dx * 0.3, ball_y + dy * 0.3,
                Color::from_rgba(255, 255, 200, 120));
        }
    }

    // Ball
    if phase != 2.0 {
        shapes::fill_circle(fb, ball_x + 2.0, ball_y + 2.0, BALL_RADIUS, COL_BALL_SHADOW);
        shapes::fill_circle(fb, ball_x, ball_y, BALL_RADIUS, COL_BALL);
        shapes::fill_circle(fb, ball_x - 1.5, ball_y - 1.5, 1.5, Color::from_rgba(255, 255, 255, 180));
    }

    // ── Fight HUD ──
    let hud_y = WORLD_H;
    shapes::fill_rect(fb, 0.0, hud_y, WIDTH, HUD_H, COL_HUD_BG);
    shapes::draw_line(fb, 0.0, hud_y, WIDTH, hud_y, Color::from_rgba(80, 80, 90, 255));

    // Monster name with color pip
    shapes::fill_circle(fb, 24.0, hud_y + 18.0, 8.0, monster.color);
    text::draw_text(fb, 38, (hud_y + 12.0) as i32, monster.name, COL_TEXT, 2);

    // Strokes + par
    text::draw_text(fb, 16, (hud_y + 40.0) as i32, &format!("STROKE {}", strokes), COL_TEXT_DIM, 1);
    text::draw_text(fb, 16, (hud_y + 56.0) as i32, &format!("PAR {}", monster.par), COL_TEXT_DIM, 1);

    // HP pips in fight
    let hp = engine.global_state.get_f64(K_PLAYER_HP).unwrap_or(5.0) as i32;
    let max_hp = engine.global_state.get_f64(K_PLAYER_MAX_HP).unwrap_or(5.0) as i32;
    text::draw_text(fb, 16, (hud_y + 74.0) as i32, "HP", COL_TEXT_DIM, 1);
    for i in 0..max_hp {
        let col = if i < hp { COL_HP } else { Color::from_rgba(60, 60, 60, 255) };
        shapes::fill_rect(fb, 50.0 + i as f64 * 14.0, hud_y + 74.0, 10.0, 8.0, col);
    }

    // Score vs par
    if strokes > 0 {
        let diff = strokes - monster.par;
        let score_text = if diff < 0 { format!("{}", diff) }
                         else if diff == 0 { "E".to_string() }
                         else { format!("+{}", diff) };
        let score_col = if diff <= 0 { COL_SUCCESS } else { COL_FLAG };
        text::draw_text(fb, 380, (hud_y + 12.0) as i32, &score_text, score_col, 3);
    }

    // Power bar
    if aim_active > 0.5 && phase == 0.0 {
        let aim_x = engine.global_state.get_f64(K_AIM_X).unwrap_or(0.0);
        let aim_y = engine.global_state.get_f64(K_AIM_Y).unwrap_or(0.0);
        let dist = ((ball_x - aim_x).powi(2) + (ball_y - aim_y).powi(2)).sqrt();
        let pct = (dist / 120.0).min(1.0);
        let bar_x = 150.0;
        let bar_y = hud_y + 48.0;
        shapes::fill_rect(fb, bar_x, bar_y, 200.0, 14.0, COL_POWER_BG);
        shapes::fill_rect(fb, bar_x, bar_y, 200.0 * pct, 14.0, COL_POWER_BAR);
        shapes::draw_rect(fb, bar_x, bar_y, 200.0, 14.0, COL_TEXT_DIM);
        text::draw_text(fb, bar_x as i32, (bar_y - 12.0) as i32, "POWER", COL_TEXT_DIM, 1);
    }

    if phase == 0.0 && strokes == 0 {
        text::draw_text_centered(fb, 240, (hud_y + 100.0) as i32, "DRAG TO AIM - RELEASE TO SHOOT", COL_TEXT_DIM, 1);
    }

    // Result overlays
    if phase == 2.0 {
        let rt = engine.global_state.get_f64(K_RESULT_TIMER).unwrap_or(0.0);
        let alpha = ((rt * 3.0).min(1.0) * 200.0) as u8;
        shapes::fill_rect(fb, 0.0, 200.0, WIDTH, 200.0, Color::from_rgba(0, 0, 0, alpha));

        if rt > 0.3 {
            text::draw_text_centered(fb, 240, 230, &format!("{} DEFEATED!", monster.name), COL_SUCCESS, 2);
            text::draw_text_centered(fb, 240, 270, "HOLE IN", COL_SUCCESS, 3);
            let label = if strokes == 1 { "ONE!" } else { &format!("{}", strokes) };
            text::draw_text_centered(fb, 240, 310, label, COL_SUCCESS, 4);
            let diff = strokes - monster.par;
            let perf = match diff {
                d if d <= -2 => "EAGLE! +HEAL",  -1 => "BIRDIE!",  0 => "PAR",
                1 => "BOGEY -1HP",  2 => "DOUBLE BOGEY -2HP",  _ => "-3HP",
            };
            text::draw_text_centered(fb, 240, 350, perf, COL_TEXT, 2);
        }
        if rt > 1.5 { text::draw_text_centered(fb, 240, 380, "TAP TO RETURN", COL_TEXT_DIM, 1); }
    }

    if phase == 3.0 {
        let rt = engine.global_state.get_f64(K_RESULT_TIMER).unwrap_or(0.0);
        let alpha = ((rt * 3.0).min(1.0) * 200.0) as u8;
        shapes::fill_rect(fb, 0.0, 200.0, WIDTH, 200.0, Color::from_rgba(0, 0, 0, alpha));

        if rt > 0.3 {
            text::draw_text_centered(fb, 240, 260, "TOO MANY STROKES!", COL_FLAG, 2);
            text::draw_text_centered(fb, 240, 300, &format!("{} ESCAPES!", monster.name), COL_TEXT_DIM, 2);
            text::draw_text_centered(fb, 240, 330, "-3HP", COL_FLAG, 1);
        }
        if rt > 1.5 { text::draw_text_centered(fb, 240, 360, "TAP TO RETURN", COL_TEXT_DIM, 1); }
    }

    engine.screen_fx.tick(0.016);
    engine.screen_fx.apply(fb);
    crate::rendering::post_fx::apply(fb, &mut engine.post_fx, 0.016, engine.frame);
}

// ═══════════════════════════════════════════════════════════════════════
// HEADLESS TEST SUPPORT
// ═══════════════════════════════════════════════════════════════════════

/// Action dispatcher for headless testing. Routes ScheduledActions to
/// the appropriate S-League input handlers.
pub fn dispatch_action(engine: &mut Engine, action: &crate::headless::ScheduledAction) {
    match action {
        crate::headless::ScheduledAction::PointerDown { x, y, .. } => {
            on_pointer_down(engine, *x, *y);
        }
        crate::headless::ScheduledAction::PointerMove { x, y, .. } => {
            on_pointer_move(engine, *x, *y);
        }
        crate::headless::ScheduledAction::PointerUp { x, y, .. } => {
            on_pointer_up(engine, *x, *y);
        }
    }
}

/// S-League scoring: 1.0 if ball sunk (phase==2), 0.0 otherwise.
pub fn score_hole_completion(sim: &crate::headless::SimResult) -> f64 {
    let phase = sim.game_state.get("tl_phase").and_then(|v| v.as_f64()).unwrap_or(0.0);
    if (phase - 2.0).abs() < 0.01 { 1.0 } else { 0.0 }
}

/// S-League scoring: 1.0 for hole-in-one at par 3, scales down as strokes increase.
pub fn score_stroke_efficiency(sim: &crate::headless::SimResult) -> f64 {
    let strokes = sim.game_state.get("strokes").and_then(|v| v.as_f64()).unwrap_or(0.0);
    if strokes <= 0.0 { return 0.0; }
    let par = 3.0;
    (par / strokes).min(1.0)
}

/// S-League scoring: 1.0 at hole, degrades linearly with distance (up to 500px).
pub fn score_proximity_to_hole(sim: &crate::headless::SimResult) -> f64 {
    let bx = sim.game_state.get("ball_x").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let by = sim.game_state.get("ball_y").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let hx = sim.game_state.get("hole_x").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let hy = sim.game_state.get("hole_y").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let dist = ((bx - hx).powi(2) + (by - hy).powi(2)).sqrt();
    (1.0 - dist / 500.0).max(0.0)
}
