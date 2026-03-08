// AI-INSTRUCTIONS: pokemonv2/overworld.rs — Player movement, collision, warps, camera, NPC wandering.
// Import graph: overworld.rs <- data.rs, maps.rs, events.rs(EventFlags, SceneState)

use super::data::{CameraState, Direction, NpcState, PlayerState};
use super::events::{EventFlags, SceneState};
use super::maps::{find_bg_event, find_coord_event, find_warp, is_walkable, MapData, MapId, NpcMoveType};
use crate::engine::Engine;

// ── Constants ────────────────────────────────────────────────────────────────

pub const TILE_PX: i32 = 16;
pub const VIEW_TILES_X: i32 = 10;
pub const VIEW_TILES_Y: i32 = 9;
pub const WALK_SPEED: f64 = 8.0;      // pixels per frame at 60fps
pub const CAMERA_LERP: f64 = 0.2;
pub const NPC_WANDER_INTERVAL: f64 = 2.0; // seconds between random NPC wander steps

// ── Result Type ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum OverworldResult {
    Nothing,
    WarpTo { dest_map: MapId, dest_warp_id: u8 },
    TriggerScript { script_id: u16, npc_idx: Option<u8> },
    TriggerCoordEvent { script_id: u16 },
}

// ── Main Step ─────────────────────────────────────────────────────────────────

/// Main overworld update — called each frame when GamePhase == Overworld.
pub fn step_overworld(
    player: &mut PlayerState,
    camera: &mut CameraState,
    map: &MapData,
    npc_states: &mut Vec<NpcState>,
    _flags: &EventFlags,
    scenes: &SceneState,
    engine: &Engine,
) -> OverworldResult {
    // ── 1. Advance player walk ────────────────────────────────────────────
    if player.is_walking {
        player.walk_offset += WALK_SPEED;
        player.frame_timer += 1.0;
        if player.frame_timer >= 4.0 {
            player.frame_timer = 0.0;
            player.walk_frame = (player.walk_frame + 1) % 4;
        }

        if player.walk_offset >= TILE_PX as f64 {
            // Snap to destination
            player.walk_offset = 0.0;
            player.is_walking = false;
            match player.facing {
                Direction::Up    => player.y -= 1,
                Direction::Down  => player.y += 1,
                Direction::Left  => player.x -= 1,
                Direction::Right => player.x += 1,
            }

            // Check warp at new position
            if let Some(warp) = find_warp(map, player.x, player.y) {
                let result = OverworldResult::WarpTo {
                    dest_map: warp.dest_map,
                    dest_warp_id: warp.dest_warp_id,
                };
                update_camera(camera, player);
                return result;
            }

            // Check coord events at new position
            let scene_id = scenes.get(map.id);
            if let Some(evt) = find_coord_event(map, player.x, player.y, scene_id) {
                let script_id = evt.script_id;
                update_camera(camera, player);
                return OverworldResult::TriggerCoordEvent { script_id };
            }

            update_camera(camera, player);
            return OverworldResult::Nothing;
        }

        update_camera(camera, player);
        return OverworldResult::Nothing;
    }

    // ── 2. Process directional input ─────────────────────────────────────
    let input = &engine.input;
    let up    = input.keys_held.contains("ArrowUp")    || input.keys_held.contains("KeyW");
    let down  = input.keys_held.contains("ArrowDown")  || input.keys_held.contains("KeyS");
    let left  = input.keys_held.contains("ArrowLeft")  || input.keys_held.contains("KeyA");
    let right = input.keys_held.contains("ArrowRight") || input.keys_held.contains("KeyD");

    let maybe_dir = if up    { Some(Direction::Up) }
                    else if down  { Some(Direction::Down) }
                    else if left  { Some(Direction::Left) }
                    else if right { Some(Direction::Right) }
                    else { None };

    if let Some(dir) = maybe_dir {
        player.facing = dir;
        let (tx, ty) = target_tile(player.x, player.y, dir);

        if is_walkable(map, tx, ty) && npc_at(npc_states, tx, ty).is_none() {
            player.is_walking = true;
            player.walk_offset = 0.0;
        }
    }

    // ── 3. Interaction (confirm button) ───────────────────────────────────
    let confirm = input.keys_pressed.contains("KeyZ")
        || input.keys_pressed.contains("Space")
        || input.keys_pressed.contains("Enter");

    if confirm {
        let (fx, fy) = target_tile(player.x, player.y, player.facing);

        // Check for interactable NPC
        if let Some(npc_idx) = npc_at(npc_states, fx, fy) {
            let script_id = map.npcs[npc_idx].script_id;
            if script_id != 0 {
                return OverworldResult::TriggerScript {
                    script_id,
                    npc_idx: Some(npc_idx as u8),
                };
            }
        }

        // Check for bg_event
        if let Some(evt) = find_bg_event(map, fx, fy, player.facing) {
            let script_id = evt.script_id;
            if script_id != 0 {
                return OverworldResult::TriggerScript {
                    script_id,
                    npc_idx: None,
                };
            }
        }
    }

    // ── 4. Tick NPC wandering ─────────────────────────────────────────────
    tick_npc_wander(npc_states, map, engine);

    update_camera(camera, player);
    OverworldResult::Nothing
}

// ── Camera ────────────────────────────────────────────────────────────────────

pub fn update_camera(camera: &mut CameraState, player: &PlayerState) {
    let walk_dx = match player.facing {
        Direction::Right if player.is_walking => player.walk_offset,
        Direction::Left  if player.is_walking => -player.walk_offset,
        _ => 0.0,
    };
    let walk_dy = match player.facing {
        Direction::Down if player.is_walking => player.walk_offset,
        Direction::Up   if player.is_walking => -player.walk_offset,
        _ => 0.0,
    };

    let target_x = (player.x * TILE_PX) as f64 + walk_dx;
    let target_y = (player.y * TILE_PX) as f64 + walk_dy;

    camera.x += (target_x - camera.x) * CAMERA_LERP;
    camera.y += (target_y - camera.y) * CAMERA_LERP;
}

/// Snap camera to player — used on map transitions.
pub fn snap_camera(camera: &mut CameraState, player: &PlayerState) {
    camera.x = (player.x * TILE_PX) as f64;
    camera.y = (player.y * TILE_PX) as f64;
}

// ── NPC Helpers ───────────────────────────────────────────────────────────────

/// Return the NPC index at tile (x, y) if any visible NPC occupies it.
pub fn npc_at(npc_states: &[NpcState], x: i32, y: i32) -> Option<usize> {
    npc_states.iter().enumerate().find_map(|(i, npc)| {
        if npc.visible && npc.x == x && npc.y == y { Some(i) } else { None }
    })
}

fn target_tile(x: i32, y: i32, dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Up    => (x, y - 1),
        Direction::Down  => (x, y + 1),
        Direction::Left  => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn tick_npc_wander(npc_states: &mut Vec<NpcState>, map: &MapData, engine: &Engine) {
    let dt = 1.0 / 60.0;
    let rng_val = engine.rng.state; // use engine RNG state for determinism

    for (i, npc_def) in map.npcs.iter().enumerate() {
        let state = match npc_states.get_mut(i) {
            Some(s) => s,
            None => continue,
        };
        if !state.visible || state.is_walking { continue; }

        match npc_def.move_type {
            NpcMoveType::SpinRandom => {
                state.wander_timer += dt;
                if state.wander_timer >= NPC_WANDER_INTERVAL {
                    state.wander_timer = 0.0;
                    // Deterministic spin using rng state mixed with NPC index
                    let mixed = rng_val.wrapping_add(i as u64 * 0x9e3779b97f4a7c15);
                    state.facing = match (mixed >> 32) % 4 {
                        0 => Direction::Up,
                        1 => Direction::Down,
                        2 => Direction::Left,
                        _ => Direction::Right,
                    };
                }
            }

            NpcMoveType::WalkUpDown => {
                state.wander_timer += dt;
                if state.wander_timer >= NPC_WANDER_INTERVAL {
                    state.wander_timer = 0.0;
                    let mixed = rng_val.wrapping_add(i as u64 * 0x6c62272e07bb0142);
                    let try_dir = if (mixed >> 32) % 2 == 0 { Direction::Up } else { Direction::Down };
                    let (tx, ty) = target_tile(state.x, state.y, try_dir);
                    if is_walkable(map, tx, ty) {
                        state.facing = try_dir;
                        state.is_walking = true;
                        state.walk_offset = 0.0;
                    }
                }
                if state.is_walking {
                    state.walk_offset += WALK_SPEED;
                    if state.walk_offset >= TILE_PX as f64 {
                        state.walk_offset = 0.0;
                        state.is_walking = false;
                        match state.facing {
                            Direction::Up   => state.y -= 1,
                            Direction::Down => state.y += 1,
                            _ => {}
                        }
                    }
                }
            }

            _ => {} // Still, Standing: no movement
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::maps::load_map;
    use super::super::data::PlayerState;

    #[test]
    fn test_snap_camera() {
        let player = PlayerState {
            x: 5, y: 3, facing: Direction::Down,
            walk_offset: 0.0, is_walking: false,
            walk_frame: 0, frame_timer: 0.0,
            name: "TEST".to_string(),
        };
        let mut camera = CameraState { x: 0.0, y: 0.0 };
        snap_camera(&mut camera, &player);
        assert_eq!(camera.x, (5 * TILE_PX) as f64);
        assert_eq!(camera.y, (3 * TILE_PX) as f64);
    }

    #[test]
    fn test_npc_at_finds_npc() {
        let map = load_map(MapId::NewBarkTown);
        let npc_states = super::super::maps::init_npc_states(&map);
        // Teacher is at (6, 8)
        let found = npc_at(&npc_states, 6, 8);
        assert!(found.is_some(), "Teacher should be at (6,8)");
    }

    #[test]
    fn test_npc_at_empty_tile() {
        let npc_states: Vec<NpcState> = Vec::new();
        assert!(npc_at(&npc_states, 5, 5).is_none());
    }

    #[test]
    fn test_is_walkable_floor() {
        let map = load_map(MapId::PlayersHouse2F);
        assert!(is_walkable(&map, 3, 3));
    }

    #[test]
    fn test_is_walkable_out_of_bounds() {
        let map = load_map(MapId::PlayersHouse2F);
        assert!(!is_walkable(&map, -1, 0));
        assert!(!is_walkable(&map, map.width, 0));
    }
}
