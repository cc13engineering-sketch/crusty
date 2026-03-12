// AI-INSTRUCTIONS: pokemonv2/overworld.rs — Player movement, collision, warps, camera, NPC wandering.
// Sprint 2: Ledge movement, wild encounter checks, map connection transitions, new step_overworld signature.
// Sprint 4: TrainerBattle OverworldResult variant, is_in_sight() helper, trainer sight detection
//           in step_overworld after each player step. Uses NpcDef.trainer_range + event flags.
// Review #3: step_overworld takes time_of_day, rng_enc, rng_slot as parameters (caller extracts).
// Review #7: is_walkable_with_direction imported from maps.rs, not defined here.
// Import graph: overworld.rs <- data.rs, maps.rs, events.rs(EventFlags, SceneState)

use super::data::{CameraState, Direction, NpcState, PlayerState, SpeciesId, TimeOfDay};
use super::events::{EventFlags, SceneState};
use super::maps::{
    find_bg_event, find_coord_event, find_warp,
    is_walkable, is_walkable_with_direction,
    MapData, MapId, NpcMoveType,
    C_GRASS,
};
use crate::engine::Engine;

// ── Constants ────────────────────────────────────────────────────────────────

pub const TILE_PX: i32 = 16;
pub const VIEW_TILES_X: i32 = 10;
pub const VIEW_TILES_Y: i32 = 9;
pub const WALK_SPEED: f64 = 8.0;
pub const CAMERA_LERP: f64 = 0.2;
pub const NPC_WANDER_INTERVAL: f64 = 2.0;

// ── Result Type ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum OverworldResult {
    Nothing,
    WarpTo { dest_map: MapId, dest_warp_id: u8 },
    TriggerScript { script_id: u16, npc_idx: Option<u8> },
    TriggerCoordEvent { script_id: u16 },
    WildEncounter { species: SpeciesId, level: u8 },
    MapConnection { direction: Direction, dest_map: MapId, offset: i8 },
    /// Sprint 4: a trainer spotted the player via line-of-sight
    TrainerBattle { npc_idx: u8, script_id: u16 },
}

// ── Main Step ─────────────────────────────────────────────────────────────────

/// Main overworld update. Review #3: caller extracts time_of_day, rng_enc, rng_slot.
pub fn step_overworld(
    player: &mut PlayerState,
    camera: &mut CameraState,
    map: &MapData,
    npc_states: &mut Vec<NpcState>,
    flags: &EventFlags,
    scenes: &SceneState,
    engine: &Engine,
    time_of_day: TimeOfDay,
    rng_enc: u8,
    rng_slot: u8,
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
            player.walk_offset = 0.0;
            player.is_walking = false;
            match player.facing {
                Direction::Up    => player.y -= 1,
                Direction::Down  => player.y += 1,
                Direction::Left  => player.x -= 1,
                Direction::Right => player.x += 1,
            }

            // Check grass encounter BEFORE warps
            if let Some((species, level)) = check_wild_encounter(map, player.x, player.y, time_of_day, rng_enc, rng_slot) {
                update_camera(camera, player);
                return OverworldResult::WildEncounter { species, level };
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

            // Check coord events
            let scene_id = scenes.get(map.id);
            if let Some(evt) = find_coord_event(map, player.x, player.y, scene_id) {
                let script_id = evt.script_id;
                update_camera(camera, player);
                return OverworldResult::TriggerCoordEvent { script_id };
            }

            // Sprint 4: check trainer sight lines after each step
            for (i, npc_def) in map.npcs.iter().enumerate() {
                if let Some(range) = npc_def.trainer_range {
                    // Skip trainers whose beaten event flag is already set
                    if let Some(flag) = npc_def.event_flag {
                        if flags.has(flag) { continue; }
                    }
                    if let Some(npc_state) = npc_states.get(i) {
                        if !npc_state.visible { continue; }
                        if is_in_sight(npc_state.x, npc_state.y, npc_def.facing, player.x, player.y, range) {
                            update_camera(camera, player);
                            return OverworldResult::TrainerBattle {
                                npc_idx: i as u8,
                                script_id: npc_def.script_id,
                            };
                        }
                    }
                }
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

        // Review #7: use direction-aware walkability for ledge support
        if is_walkable_with_direction(map, tx, ty, dir) && npc_at(npc_states, tx, ty).is_none() {
            player.is_walking = true;
            player.walk_offset = 0.0;
        } else if tx < 0 || ty < 0 || tx >= map.width || ty >= map.height {
            // Check for map connection at edge
            let conn = match dir {
                Direction::Left  => &map.connections.west,
                Direction::Right => &map.connections.east,
                Direction::Up    => &map.connections.north,
                Direction::Down  => &map.connections.south,
            };
            if let Some(connection) = conn {
                return OverworldResult::MapConnection {
                    direction: dir,
                    dest_map: connection.dest_map,
                    offset: connection.offset,
                };
            }
        }
    }

    // ── 3. Interaction (confirm button) ───────────────────────────────────
    let confirm = input.keys_pressed.contains("KeyZ")
        || input.keys_pressed.contains("Space")
        || input.keys_pressed.contains("Enter");

    if confirm {
        let (fx, fy) = target_tile(player.x, player.y, player.facing);

        if let Some(npc_idx) = npc_at(npc_states, fx, fy) {
            let script_id = map.npcs[npc_idx].script_id;
            if script_id != 0 {
                return OverworldResult::TriggerScript {
                    script_id,
                    npc_idx: Some(npc_idx as u8),
                };
            }
        }

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

// ── Wild Encounter Check ──────────────────────────────────────────────────────

/// Check for a wild encounter after stepping onto a grass tile.
/// Uses pokecrystal probability distribution: slot selected by rng_slot,
/// encounter triggered if rng_encounter < encounter_rate.
pub fn check_wild_encounter(
    map: &MapData,
    x: i32,
    y: i32,
    time_of_day: TimeOfDay,
    rng_encounter: u8,
    rng_slot: u8,
) -> Option<(SpeciesId, u8)> {
    if x < 0 || y < 0 || x >= map.width || y >= map.height {
        return None;
    }
    let idx = (y * map.width + x) as usize;
    if map.collision[idx] != C_GRASS {
        return None;
    }

    if let Some(ref table) = map.wild_encounters {
        if rng_encounter >= table.encounter_rate {
            return None;
        }

        let slots = match time_of_day {
            TimeOfDay::Morning => &table.morning,
            TimeOfDay::Day => &table.day,
            TimeOfDay::Night => &table.night,
        };

        if slots.is_empty() {
            return None;
        }

        // Pokecrystal probability: slots 0-1=30%, 2=20%, 3=10%, 4=5%, 5-6=2.5%
        let slot_idx = match rng_slot {
            0..=76  => 0,
            77..=153 => 1,
            154..=204 => 2,
            205..=229 => 3,
            230..=242 => 4,
            243..=248 => 5,
            _ => 6,
        };

        let slot_idx = slot_idx.min(slots.len() - 1);
        let slot = &slots[slot_idx];
        Some((slot.species, slot.level))
    } else {
        None
    }
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

pub fn snap_camera(camera: &mut CameraState, player: &PlayerState) {
    camera.x = (player.x * TILE_PX) as f64;
    camera.y = (player.y * TILE_PX) as f64;
}

// ── NPC Helpers ───────────────────────────────────────────────────────────────

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
    let rng_val = engine.rng.state;

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

            NpcMoveType::WalkLeftRight => {
                state.wander_timer += dt;
                if state.wander_timer >= NPC_WANDER_INTERVAL {
                    state.wander_timer = 0.0;
                    let mixed = rng_val.wrapping_add(i as u64 * 0xd1b54a32d192ed03);
                    let try_dir = if (mixed >> 32) % 2 == 0 { Direction::Left } else { Direction::Right };
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
                            Direction::Left  => state.x -= 1,
                            Direction::Right => state.x += 1,
                            _ => {}
                        }
                    }
                }
            }

            _ => {}
        }
    }
}

// ── Trainer Sight ─────────────────────────────────────────────────────────────

/// Returns true if (px, py) is within `range` tiles in the NPC's facing direction.
/// Mirrors pokecrystal's SeeYouScript: exact column/row alignment, range in tiles.
pub fn is_in_sight(npc_x: i32, npc_y: i32, npc_facing: Direction, px: i32, py: i32, range: u8) -> bool {
    let range = range as i32;
    match npc_facing {
        Direction::Up    => px == npc_x && py < npc_y  && (npc_y - py) <= range,
        Direction::Down  => px == npc_x && py > npc_y  && (py - npc_y) <= range,
        Direction::Left  => py == npc_y && px < npc_x  && (npc_x - px) <= range,
        Direction::Right => py == npc_y && px > npc_x  && (px - npc_x) <= range,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::maps::{load_map, is_walkable_with_direction, C_LEDGE_D};
    use super::super::data::{PlayerState, TimeOfDay};

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

    #[test]
    fn test_ledge_only_walkable_facing_down() {
        let map = load_map(MapId::Route29);
        // Find a C_LEDGE_D tile
        let ledge_pos = map.collision.iter().enumerate().find(|(_, &c)| c == C_LEDGE_D);
        if let Some((idx, _)) = ledge_pos {
            let x = (idx as i32) % map.width;
            let y = (idx as i32) / map.width;
            assert!(is_walkable_with_direction(&map, x, y, Direction::Down), "Ledge should be walkable going Down");
            assert!(!is_walkable_with_direction(&map, x, y, Direction::Up),   "Ledge should NOT be walkable going Up");
            assert!(!is_walkable_with_direction(&map, x, y, Direction::Left),  "Ledge should NOT be walkable going Left");
            assert!(!is_walkable_with_direction(&map, x, y, Direction::Right), "Ledge should NOT be walkable going Right");
        }
    }

    #[test]
    fn test_wild_encounter_on_grass() {
        let map = load_map(MapId::Route29);
        let grass_pos = map.collision.iter().enumerate().find(|(_, &c)| c == super::super::maps::C_GRASS);
        if let Some((idx, _)) = grass_pos {
            let x = (idx as i32) % map.width;
            let y = (idx as i32) / map.width;
            // rng_encounter=0 (< encounter_rate=10): should trigger
            let result = check_wild_encounter(&map, x, y, TimeOfDay::Day, 0, 0);
            assert!(result.is_some(), "Should encounter on grass with rng=0");
            // rng_encounter=255 (>= encounter_rate=10): should not trigger
            let result = check_wild_encounter(&map, x, y, TimeOfDay::Day, 255, 0);
            assert!(result.is_none(), "Should not encounter with rng=255");
        }
    }

    // ── Sprint 3 QA: Group 5 — Wild Encounter Slot Distribution ────────

    #[test]
    fn test_wild_encounter_slot_distribution() {
        let map = load_map(MapId::Route29);
        let grass_pos = map.collision.iter().enumerate()
            .find(|(_, &c)| c == super::super::maps::C_GRASS).unwrap().0;
        let x = (grass_pos as i32) % map.width;
        let y = (grass_pos as i32) / map.width;

        // Pokecrystal slot probabilities: 0=30%, 1=30%, 2=20%, 3=10%, 4=5%, 5=2.5%, 6=2.5%
        let test_cases: Vec<(u8, usize)> = vec![
            (0, 0),     // rng_slot=0 -> slot 0
            (77, 1),    // rng_slot=77 -> slot 1
            (154, 2),   // rng_slot=154 -> slot 2
            (205, 3),   // rng_slot=205 -> slot 3
            (230, 4),   // rng_slot=230 -> slot 4
            (243, 5),   // rng_slot=243 -> slot 5
            (249, 6),   // rng_slot=249 -> slot 6
        ];
        let table = map.wild_encounters.as_ref().unwrap();
        for (rng_slot, expected_slot) in test_cases {
            let result = check_wild_encounter(&map, x, y, TimeOfDay::Day, 0, rng_slot);
            assert!(result.is_some(), "rng_slot={} should trigger encounter", rng_slot);
            let (species, level) = result.unwrap();
            assert_eq!(species, table.day[expected_slot].species,
                "rng_slot={} -> slot {}: species mismatch", rng_slot, expected_slot);
            assert_eq!(level, table.day[expected_slot].level,
                "rng_slot={} -> slot {}: level mismatch", rng_slot, expected_slot);
        }
    }

    #[test]
    fn test_wild_encounter_not_on_floor() {
        let map = load_map(MapId::Route29);
        // Find a floor tile (not grass)
        let floor_pos = map.collision.iter().enumerate()
            .find(|(_, &c)| c == super::super::maps::C_FLOOR);
        if let Some((idx, _)) = floor_pos {
            let x = (idx as i32) % map.width;
            let y = (idx as i32) / map.width;
            let result = check_wild_encounter(&map, x, y, TimeOfDay::Day, 0, 0);
            assert!(result.is_none(), "Should NOT encounter on floor tiles");
        }
    }

    #[test]
    fn test_wild_encounter_time_of_day_matters() {
        let map = load_map(MapId::Route29);
        let grass_pos = map.collision.iter().enumerate()
            .find(|(_, &c)| c == super::super::maps::C_GRASS).unwrap().0;
        let x = (grass_pos as i32) % map.width;
        let y = (grass_pos as i32) / map.width;

        let morning = check_wild_encounter(&map, x, y, TimeOfDay::Morning, 0, 0).unwrap();
        let night = check_wild_encounter(&map, x, y, TimeOfDay::Night, 0, 0).unwrap();
        // Morning slot 0 is Pidgey, Night slot 0 is Hoothoot -- different species
        assert_ne!(morning.0, night.0, "Morning and Night slot 0 should have different species");
    }
}
