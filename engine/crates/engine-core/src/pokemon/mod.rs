// AI-INSTRUCTIONS: Pokemon Gold/Silver/Crystal recreation implementing the Simulation trait.
// State machine: TitleScreen -> StarterSelect -> Overworld <-> Battle <-> Menu <-> Dialogue.
// Grid-based overworld with tile rendering, wild encounters in tall grass, turn-based battles.
// Battle Pokemon sprites loaded from web in JS layer; overworld tiles/characters rendered in Rust.
// Features: multi-Pokemon trainer battles, status conditions (PSN/BRN/PAR/SLP/FRZ), paralysis
// speed/skip checks, PP tracking, Pokedex (seen/caught), PC storage (Bill's PC with deposit/withdraw),
// Poke Mart (9 items incl. Repel/Escape Rope), badge system, day/night cycle, evolution,
// SFX via SoundCommand, whiteout with money loss, critical hits (1/16), smart enemy AI (50% best move).
// Key pattern: battle uses take-put-back for BattleState borrow management.
// NPC trainers have trainer_team field with typed Pokemon lists (TrainerPokemon in maps.rs).
// Input helpers: is_confirm/is_cancel/is_up/is_down/is_left/is_right + held_ variants.
// Camera: smooth lerp (CAMERA_LERP), snaps on map transition. Ledge tiles: south-only traversal.

pub mod data;
pub mod sprites;
pub mod maps;
pub mod render;

use crate::engine::Engine;
use crate::simulation::Simulation;
use crate::rendering::color::Color;
use crate::sound::{SoundCommand, Waveform};
use data::*;
use maps::*;
use render::*;
use sprites::*;

// ─── Input Helpers ──────────────────────────────────────

fn is_confirm(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("KeyZ")
        || engine.input.keys_pressed.contains("Space")
        || engine.input.keys_pressed.contains("Enter")
}

fn is_cancel(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("KeyX")
        || engine.input.keys_pressed.contains("Escape")
}

fn is_up(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("ArrowUp")
        || engine.input.keys_pressed.contains("KeyW")
}

fn is_down(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("ArrowDown")
        || engine.input.keys_pressed.contains("KeyS")
}

fn is_left(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("ArrowLeft")
        || engine.input.keys_pressed.contains("KeyA")
}

fn is_right(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("ArrowRight")
        || engine.input.keys_pressed.contains("KeyD")
}

fn held_up(engine: &Engine) -> bool {
    engine.input.keys_held.contains("ArrowUp")
        || engine.input.keys_held.contains("KeyW")
}

fn held_down(engine: &Engine) -> bool {
    engine.input.keys_held.contains("ArrowDown")
        || engine.input.keys_held.contains("KeyS")
}

fn held_left(engine: &Engine) -> bool {
    engine.input.keys_held.contains("ArrowLeft")
        || engine.input.keys_held.contains("KeyA")
}

fn held_right(engine: &Engine) -> bool {
    engine.input.keys_held.contains("ArrowRight")
        || engine.input.keys_held.contains("KeyD")
}

// ─── Constants ──────────────────────────────────────────

const TILE_PX: i32 = 16;
const VIEW_TILES_X: i32 = 10;
const VIEW_TILES_Y: i32 = 9;
const WALK_SPEED: f64 = 8.0;
const ENCOUNTER_RATE: f64 = 0.15;

const MART_INVENTORY: [(u8, u16); 9] = [
    (ITEM_POKE_BALL, 200),
    (ITEM_POTION, 300),
    (ITEM_SUPER_POTION, 700),
    (ITEM_ANTIDOTE, 100),
    (ITEM_PARALYZE_HEAL, 200),
    (ITEM_REVIVE, 1500),
    (ITEM_GREAT_BALL, 600),
    (ITEM_REPEL, 350),
    (ITEM_ESCAPE_ROPE, 550),
];

// ─── Battle Constants ───────────────────────────────────
const CRIT_CHANCE: u64 = 16; // 1/16 base crit rate (Gen 2)
const PARALYSIS_SKIP_CHANCE: f64 = 0.25; // 25% chance to be fully paralyzed
const DAMAGE_ROLL_MIN: f64 = 0.85;
const DAMAGE_ROLL_RANGE: f64 = 0.15;
const CAMERA_LERP: f64 = 0.2;

// ─── Game Phase ─────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
enum GamePhase {
    TitleScreen,
    StarterSelect { cursor: u8 },
    Overworld,
    EncounterTransition { timer: f64 },
    Battle,
    Dialogue,
    Menu,
    PokemonMenu { cursor: u8 },
    BagMenu { cursor: u8 },
    BagUseItem { item_id: u8, target_cursor: u8 },
    PokeMart { cursor: u8 },
    PokemonSummary { index: u8 },
    Pokedex { cursor: u8, scroll: u8 },
    PCMenu { mode: u8, cursor: u8 }, // mode: 0=select, 1=withdraw, 2=deposit
    Healing { timer: f64 },
    Evolution { timer: f64, new_species: SpeciesId },
}

// ─── Battle Phase ───────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
enum BattlePhase {
    Intro { timer: f64 },
    ActionSelect { cursor: u8 },
    MoveSelect { cursor: u8 },
    PlayerAttack { timer: f64, move_id: MoveId, damage: u16, effectiveness: f64, is_crit: bool, from_pending: bool },
    EnemyAttack { timer: f64, move_id: MoveId, damage: u16, effectiveness: f64, is_crit: bool },
    Text { message: String, timer: f64, next_phase: Box<BattlePhase> },
    PlayerFainted,
    EnemyFainted { exp_gained: u32 },
    LevelUp { timer: f64 },
    Won { timer: f64 },
    Run,
    RunFailed { timer: f64 },
}

// ─── Battle State ───────────────────────────────────────

// Stat stage indices: ATK=0, DEF=1, SPA=2, SPD=3, SPE=4, ACC=5, EVA=6
const STAGE_ATK: usize = 0;
const STAGE_DEF: usize = 1;
const STAGE_SPA: usize = 2;
const STAGE_SPD: usize = 3;
const STAGE_SPE: usize = 4;
const STAGE_ACC: usize = 5;
const STAGE_EVA: usize = 6;

/// Convert stat stage (-6 to +6) to a multiplier (Gen 2 formula)
fn stage_multiplier(stage: i8) -> f64 {
    let s = stage.max(-6).min(6);
    if s >= 0 {
        (2 + s as i32) as f64 / 2.0
    } else {
        2.0 / (2 - s as i32) as f64
    }
}

/// Convert accuracy/evasion stage to hit rate multiplier (Gen 2: uses 3-based formula)
fn accuracy_stage_multiplier(stage: i8) -> f64 {
    let s = stage.max(-6).min(6);
    if s >= 0 {
        (3 + s as i32) as f64 / 3.0
    } else {
        3.0 / (3 - s as i32) as f64
    }
}

/// Determine what stat stage change a status move applies.
/// Returns (target_is_enemy, stat_index, delta). None if not a stat move.
fn status_move_stage_effect(move_id: MoveId) -> Option<(bool, usize, i8)> {
    match move_id {
        MOVE_GROWL => Some((true, STAGE_ATK, -1)),
        MOVE_LEER => Some((true, STAGE_DEF, -1)),
        MOVE_TAIL_WHIP => Some((true, STAGE_DEF, -1)),
        MOVE_SAND_ATTACK => Some((true, STAGE_ACC, -1)),
        MOVE_SMOKESCREEN => Some((true, STAGE_ACC, -1)),
        MOVE_STRING_SHOT => Some((true, STAGE_SPE, -1)),
        MOVE_SCARY_FACE => Some((true, STAGE_SPE, -2)),
        MOVE_DEFENSE_CURL => Some((false, STAGE_DEF, 1)),
        _ => None,
    }
}

#[derive(Clone, Debug)]
struct BattleState {
    phase: BattlePhase,
    enemy: Pokemon,
    player_idx: usize,
    is_wild: bool,
    player_hp_display: f64,
    enemy_hp_display: f64,
    turn_count: u32,
    trainer_team: Vec<Pokemon>,
    trainer_team_idx: usize,
    pending_player_move: Option<(MoveId, u16, f64, bool)>,
    // Stat stages: [ATK, DEF, SPA, SPD, SPE, ACC, EVA] — range -6 to +6, reset per battle
    player_stages: [i8; 7],
    enemy_stages: [i8; 7],
}

// ─── Dialogue State ─────────────────────────────────────

#[derive(Clone, Debug)]
struct DialogueState {
    lines: Vec<String>,
    current_line: usize,
    char_index: usize,
    timer: f64,
    on_complete: DialogueAction,
}

#[derive(Clone, Debug)]
enum DialogueAction {
    None,
    Heal,
    GiveStarter,
    StartTrainerBattle { team: Vec<(SpeciesId, u8)> },
    OpenMart,
    GiveBadge { badge_num: u8 },
}

// ─── Player State ───────────────────────────────────────

#[derive(Clone, Debug)]
struct PlayerState {
    x: i32,
    y: i32,
    facing: Direction,
    walk_offset: f64,
    is_walking: bool,
    walk_frame: u8,
    frame_timer: f64,
}

// ─── Main Game Struct ───────────────────────────────────

#[derive(Clone)]
pub struct PokemonSim {
    phase: GamePhase,
    player: PlayerState,
    party: Vec<Pokemon>,
    current_map_id: MapId,
    current_map: MapData,
    bag: Bag,
    battle: Option<BattleState>,
    dialogue: Option<DialogueState>,
    ctx: Option<RenderContext>,
    frame_count: u64,
    title_blink_timer: f64,
    has_starter: bool,
    menu_cursor: u8,
    badges: u8,
    water_frame: u8,
    water_timer: f64,
    screen_flash: f64,
    screen_shake: f64,
    screen_shake_x: f64,
    screen_shake_y: f64,
    camera_x: f64,
    camera_y: f64,
    encounter_flash_count: u8,
    day_night_tint: f64,
    time_of_day: f64,
    total_time: f64,
    step_count: u32,
    tile_cache: Vec<Vec<u8>>,
    player_sprite_cache: Vec<Vec<u8>>,
    npc_sprite_cache: Vec<Vec<u8>>,
    money: u32,
    defeated_trainers: Vec<(MapId, u8)>,
    trainer_battle_npc: Option<(MapId, u8)>,
    pokedex_seen: Vec<SpeciesId>,
    pokedex_caught: Vec<SpeciesId>,
    pc_boxes: Vec<Pokemon>,
    repel_steps: u32,
    last_pokecenter_map: MapId, // tracks which city's pokecenter door to exit to
    last_house_map: MapId, // tracks which city's generic house door to exit to
    rival_starter: SpeciesId, // rival picks type-advantaged starter
    rival_battle_done: bool,
}

impl PokemonSim {
    pub fn new() -> Self {
        let start_map = load_map(MapId::NewBarkTown);
        PokemonSim {
            phase: GamePhase::TitleScreen,
            player: PlayerState {
                x: 5, y: 8,
                facing: Direction::Down,
                walk_offset: 0.0,
                is_walking: false,
                walk_frame: 1,
                frame_timer: 0.0,
            },
            party: Vec::new(),
            current_map_id: MapId::NewBarkTown,
            current_map: start_map,
            bag: Bag::new(),
            battle: None,
            dialogue: None,
            ctx: None,
            frame_count: 0,
            title_blink_timer: 0.0,
            has_starter: false,
            menu_cursor: 0,
            badges: 0,
            water_frame: 0,
            water_timer: 0.0,
            screen_flash: 0.0,
            screen_shake: 0.0,
            screen_shake_x: 0.0,
            screen_shake_y: 0.0,
            camera_x: 0.0,
            camera_y: 0.0,
            encounter_flash_count: 0,
            day_night_tint: 0.0,
            time_of_day: 12.0,
            total_time: 0.0,
            step_count: 0,
            tile_cache: Vec::new(),
            player_sprite_cache: Vec::new(),
            npc_sprite_cache: Vec::new(),
            money: 3000,
            defeated_trainers: Vec::new(),
            trainer_battle_npc: None,
            pokedex_seen: Vec::new(),
            pokedex_caught: Vec::new(),
            pc_boxes: Vec::new(),
            repel_steps: 0,
            last_pokecenter_map: MapId::CherrygroveCity,
            last_house_map: MapId::NewBarkTown,
            rival_starter: 0, // set when player picks starter
            rival_battle_done: false,
        }
    }

    fn init_sprite_caches(&mut self) {
        let tile_strs = [
            TILE_GRASS, TILE_TALL_GRASS, TILE_PATH, TILE_TREE_TOP, TILE_TREE_BOTTOM,
            TILE_WATER, TILE_WATER2, TILE_BUILDING_WALL, TILE_BUILDING_ROOF, TILE_DOOR,
            TILE_FENCE_H, TILE_FLOWER, TILE_POKECENTER_ROOF, TILE_POKECENTER_WALL,
            TILE_POKECENTER_DOOR, TILE_LAB_WALL, TILE_LAB_ROOF, TILE_SIGN, TILE_LEDGE,
            TILE_FLOOR, TILE_TABLE, TILE_BOOKSHELF, TILE_PC, TILE_HEAL_MACHINE, TILE_BLACK,
        ];
        self.tile_cache = tile_strs.iter().map(|s| decode_sprite(s)).collect();

        let player_strs = [
            PLAYER_DOWN_1, PLAYER_DOWN_2, PLAYER_DOWN_3,
            PLAYER_UP_1, PLAYER_UP_2, PLAYER_UP_3,
            PLAYER_LEFT_1, PLAYER_LEFT_2, PLAYER_LEFT_3,
            PLAYER_RIGHT_1, PLAYER_RIGHT_2, PLAYER_RIGHT_3,
        ];
        self.player_sprite_cache = player_strs.iter().map(|s| decode_sprite(s)).collect();

        let npc_strs = [
            NPC_ELM, NPC_MOM, NPC_YOUNGSTER, NPC_LASS, NPC_NURSE, NPC_OLD_MAN,
        ];
        self.npc_sprite_cache = npc_strs.iter().map(|s| decode_sprite(s)).collect();
    }

    fn change_map(&mut self, map_id: MapId, dest_x: u8, dest_y: u8) {
        // Track which city the player came from when entering shared interiors
        if map_id == MapId::PokemonCenter {
            self.last_pokecenter_map = self.current_map_id;
        }
        if map_id == MapId::GenericHouse {
            self.last_house_map = self.current_map_id;
        }
        self.current_map_id = map_id;
        self.current_map = load_map(map_id);
        self.player.x = dest_x as i32;
        self.player.y = dest_y as i32;
        self.player.is_walking = false;
        self.player.walk_offset = 0.0;
        // Snap camera to new position (no lerp on map transitions)
        let target_x = dest_x as f64 * TILE_PX as f64 + TILE_PX as f64 / 2.0 - (VIEW_TILES_X * TILE_PX / 2) as f64;
        let target_y = dest_y as f64 * TILE_PX as f64 + TILE_PX as f64 / 2.0 - (VIEW_TILES_Y * TILE_PX / 2) as f64;
        let map_pw = (self.current_map.width as i32 * TILE_PX) as f64;
        let map_ph = (self.current_map.height as i32 * TILE_PX) as f64;
        let vw = (VIEW_TILES_X * TILE_PX) as f64;
        let vh = (VIEW_TILES_Y * TILE_PX) as f64;
        self.camera_x = target_x.max(0.0).min((map_pw - vw).max(0.0));
        self.camera_y = target_y.max(0.0).min((map_ph - vh).max(0.0));
    }

    /// Trigger rival battle event (called from step_overworld)
    fn check_rival_battle(&mut self) -> bool {
        if self.has_starter && !self.rival_battle_done
            && self.current_map_id == MapId::Route29
            && self.rival_starter > 0
        {
            self.rival_battle_done = true;
            self.dialogue = Some(DialogueState {
                lines: vec![
                    "???: Hey, wait!".to_string(),
                    "I just got a POKEMON".to_string(),
                    "from the LAB too!".to_string(),
                    "I'll show you how".to_string(),
                    "strong I already am!".to_string(),
                ],
                current_line: 0, char_index: 0, timer: 0.0,
                on_complete: DialogueAction::StartTrainerBattle {
                    team: vec![(self.rival_starter, 5)],
                },
            });
            self.phase = GamePhase::Dialogue;
            return true;
        }
        false
    }

    // ─── Overworld Logic ───────────────────────────────

    fn step_overworld(&mut self, engine: &mut Engine) {
        let dt = 1.0 / 60.0;
        self.total_time += dt;
        self.update_camera();

        // Update day/night cycle (1 game-hour = 10 real seconds, full day = 240 seconds)
        self.time_of_day = (self.total_time / 10.0) % 24.0;

        // Calculate tint based on time of day
        // Night: 0-5, Dawn: 5-7, Day: 7-17, Dusk: 17-19, Night: 19-24
        self.day_night_tint = if self.time_of_day < 5.0 {
            0.5 // Night
        } else if self.time_of_day < 7.0 {
            0.5 - (self.time_of_day - 5.0) / 4.0 // Dawn
        } else if self.time_of_day < 17.0 {
            0.0 // Day
        } else if self.time_of_day < 19.0 {
            (self.time_of_day - 17.0) / 4.0 // Dusk
        } else {
            0.5 // Night
        };

        self.water_timer += dt;
        if self.water_timer > 0.5 {
            self.water_timer = 0.0;
            self.water_frame = 1 - self.water_frame;
        }

        if self.player.is_walking {
            self.player.walk_offset += 1.0 / WALK_SPEED;
            self.player.frame_timer += dt;
            if self.player.frame_timer > 0.12 {
                self.player.frame_timer = 0.0;
                self.player.walk_frame = (self.player.walk_frame + 1) % 3;
            }

            if self.player.walk_offset >= 1.0 {
                self.player.walk_offset = 0.0;
                self.player.is_walking = false;
                self.player.walk_frame = 1;
                self.step_count += 1;

                // Decrement repel
                if self.repel_steps > 0 {
                    self.repel_steps -= 1;
                }

                match self.player.facing {
                    Direction::Up => self.player.y -= 1,
                    Direction::Down => self.player.y += 1,
                    Direction::Left => self.player.x -= 1,
                    Direction::Right => self.player.x += 1,
                }

                // Check warps
                let px = self.player.x as u8;
                let py = self.player.y as u8;
                let warp_data = self.current_map.warp_at(px, py).cloned();
                if let Some(warp) = warp_data {
                    // PokemonCenter: dynamic exit based on which city we entered from
                    if self.current_map_id == MapId::PokemonCenter {
                        let (dest_map, dx, dy) = match self.last_pokecenter_map {
                            MapId::VioletCity => (MapId::VioletCity, 5, 12),
                            MapId::AzaleaTown => (MapId::AzaleaTown, 6, 13),
                            MapId::GoldenrodCity => (MapId::GoldenrodCity, 10, 15),
                            MapId::EcruteakCity => (MapId::EcruteakCity, 15, 13),
                            MapId::OlivineCity => (MapId::OlivineCity, 4, 8),
                            _ => (MapId::CherrygroveCity, 7, 5),
                        };
                        self.change_map(dest_map, dx, dy);
                    } else if self.current_map_id == MapId::GenericHouse {
                        // GenericHouse: dynamic exit based on which city we entered from
                        let (dest_map, dx, dy) = match self.last_house_map {
                            MapId::NewBarkTown => (MapId::NewBarkTown, 12, 5),
                            MapId::CherrygroveCity => (MapId::CherrygroveCity, 15, 5),
                            MapId::VioletCity => (MapId::VioletCity, 15, 12),
                            MapId::AzaleaTown => (MapId::AzaleaTown, 8, 5),
                            MapId::GoldenrodCity => (MapId::GoldenrodCity, 11, 9),
                            MapId::EcruteakCity => (MapId::EcruteakCity, 4, 13),
                            MapId::OlivineCity => (MapId::OlivineCity, 16, 5),
                            MapId::Route39 => (MapId::Route39, 4, 5),
                            _ => (MapId::NewBarkTown, 12, 5),
                        };
                        self.change_map(dest_map, dx, dy);
                    } else {
                        self.change_map(warp.dest_map, warp.dest_x, warp.dest_y);
                    }
                    return;
                }

                // Check wild encounter (blocked by repel)
                if self.current_map.is_tall_grass(self.player.x as usize, self.player.y as usize)
                    && !self.party.is_empty()
                    && self.repel_steps == 0
                {
                    let roll = engine.rng.next_f64();
                    if roll < ENCOUNTER_RATE {
                        let r1 = engine.rng.next_f64();
                        let r2 = engine.rng.next_f64();
                        if let Some((species_id, level)) = self.current_map.roll_encounter(r1, r2) {
                            self.register_seen(species_id);
                            let enemy = Pokemon::new(species_id, level);
                            let player_hp = self.party.first().map(|p| p.hp as f64).unwrap_or(0.0);
                            self.battle = Some(BattleState {
                                phase: BattlePhase::Intro { timer: 0.0 },
                                enemy,
                                player_idx: 0,
                                is_wild: true,
                                player_hp_display: player_hp,
                                enemy_hp_display: 0.0,
                                turn_count: 0,
                                trainer_team: Vec::new(),
                                trainer_team_idx: 0,
                                pending_player_move: None,
                                player_stages: [0; 7],
                                enemy_stages: [0; 7],
                            });
                            // Trigger encounter transition flash instead of going directly to battle
                            self.encounter_flash_count = 0;
                            self.phase = GamePhase::EncounterTransition { timer: 0.0 };
                            return;
                        }
                    }
                }

                // Check for rival battle event
                if self.check_rival_battle() { return; }

                // Check trainer line-of-sight (5 tiles in their facing direction)
                if self.party.iter().any(|p| !p.is_fainted()) {
                    let px = self.player.x;
                    let py = self.player.y;
                    for (npc_idx, npc) in self.current_map.npcs.iter().enumerate() {
                        if !npc.is_trainer || npc.trainer_team.is_empty() { continue; }
                        // Skip already defeated trainers
                        let key = (self.current_map_id, npc_idx as u8);
                        if self.defeated_trainers.contains(&key) { continue; }
                        // Check if player is in this trainer's line of sight
                        let (dx, dy) = match npc.facing {
                            Direction::Up => (0i32, -1i32),
                            Direction::Down => (0, 1),
                            Direction::Left => (-1, 0),
                            Direction::Right => (1, 0),
                        };
                        let mut in_sight = false;
                        for dist in 1..=5 {
                            let check_x = npc.x as i32 + dx * dist;
                            let check_y = npc.y as i32 + dy * dist;
                            // Stop at walls
                            if check_x < 0 || check_y < 0
                                || (check_x as usize) >= self.current_map.width
                                || (check_y as usize) >= self.current_map.height
                            { break; }
                            let col = self.current_map.collision_at(check_x as usize, check_y as usize);
                            if matches!(col, CollisionType::Solid) { break; }
                            if check_x == px && check_y == py {
                                in_sight = true;
                                break;
                            }
                        }
                        if in_sight {
                            // Trainer spotted player! Start battle.
                            let team: Vec<(SpeciesId, u8)> = npc.trainer_team
                                .iter().map(|tp| (tp.species_id, tp.level)).collect();
                            let lines: Vec<String> = npc.dialogue.iter().map(|s| s.to_string()).collect();
                            self.trainer_battle_npc = Some(key);
                            self.dialogue = Some(DialogueState {
                                lines, current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::StartTrainerBattle { team },
                            });
                            self.phase = GamePhase::Dialogue;
                            self.battle = None; // ensure clean state
                            return;
                        }
                    }
                }
            }
            return;
        }

        // Movement input
        let mut new_dir: Option<Direction> = None;
        if held_up(engine) {
            new_dir = Some(Direction::Up);
        } else if held_down(engine) {
            new_dir = Some(Direction::Down);
        } else if held_left(engine) {
            new_dir = Some(Direction::Left);
        } else if held_right(engine) {
            new_dir = Some(Direction::Right);
        }

        if let Some(dir) = new_dir {
            self.player.facing = dir;
            let (nx, ny) = match dir {
                Direction::Up => (self.player.x, self.player.y - 1),
                Direction::Down => (self.player.x, self.player.y + 1),
                Direction::Left => (self.player.x - 1, self.player.y),
                Direction::Right => (self.player.x + 1, self.player.y),
            };

            if nx >= 0 && ny >= 0
                && (nx as usize) < self.current_map.width
                && (ny as usize) < self.current_map.height
            {
                let col = self.current_map.collision_at(nx as usize, ny as usize);
                let can_walk = match col {
                    CollisionType::Walkable | CollisionType::TallGrass | CollisionType::Warp => true,
                    CollisionType::Ledge => dir == Direction::Down, // ledges: south only
                    _ => false,
                };
                if can_walk {
                    let npc_blocking = self.current_map.npcs.iter().any(|npc| npc.x as i32 == nx && npc.y as i32 == ny);
                    if !npc_blocking {
                        self.player.is_walking = true;
                        self.player.walk_offset = 0.0;
                    }
                }
            }
        }

        // Interact (A button)
        if is_confirm(engine) {
            self.try_interact();
        }

        // Start menu (B/Escape)
        if is_cancel(engine) {
            if self.has_starter {
                self.phase = GamePhase::Menu;
                self.menu_cursor = 0;
            }
        }
    }

    fn try_interact(&mut self) {
        let (fx, fy) = match self.player.facing {
            Direction::Up => (self.player.x, self.player.y - 1),
            Direction::Down => (self.player.x, self.player.y + 1),
            Direction::Left => (self.player.x - 1, self.player.y),
            Direction::Right => (self.player.x + 1, self.player.y),
        };

        // NPC interaction
        let npc_info = self.current_map.npcs.iter().enumerate()
            .find(|(_, npc)| npc.x as i32 == fx && npc.y as i32 == fy)
            .map(|(idx, npc)| (idx as u8, npc.clone()));

        if let Some((npc_idx, npc)) = npc_info {
            // Trainer NPC check
            if npc.is_trainer && self.has_starter {
                let already_defeated = self.defeated_trainers.contains(&(self.current_map_id, npc_idx));
                if already_defeated {
                    self.dialogue = Some(DialogueState {
                        lines: vec!["I already lost...".to_string(), "You're pretty tough!".to_string()],
                        current_line: 0, char_index: 0, timer: 0.0, on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                } else {
                    // Build team from NPC data, fallback to generic if empty
                    let team: Vec<(SpeciesId, u8)> = if !npc.trainer_team.is_empty() {
                        npc.trainer_team.iter().map(|tp| (tp.species_id, tp.level)).collect()
                    } else {
                        vec![(RATTATA, 6)]
                    };
                    let lines: Vec<String> = npc.dialogue.iter().map(|s| s.to_string()).collect();
                    self.trainer_battle_npc = Some((self.current_map_id, npc_idx));
                    self.dialogue = Some(DialogueState {
                        lines, current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::StartTrainerBattle { team },
                    });
                    self.phase = GamePhase::Dialogue;
                }
                return;
            }

            let lines: Vec<String> = npc.dialogue.iter().map(|s| s.to_string()).collect();
            let action = if npc.sprite_id == 0 && !self.has_starter {
                DialogueAction::GiveStarter
            } else if npc.sprite_id == 4 {
                DialogueAction::Heal
            } else if npc.is_mart {
                DialogueAction::OpenMart
            } else {
                DialogueAction::None
            };
            self.dialogue = Some(DialogueState {
                lines, current_line: 0, char_index: 0, timer: 0.0, on_complete: action,
            });
            self.phase = GamePhase::Dialogue;
            return;
        }

        // PC interaction (tile id 22 = PC)
        if fx >= 0 && fy >= 0 && (fx as usize) < self.current_map.width && (fy as usize) < self.current_map.height {
            let tile = self.current_map.tiles[fy as usize * self.current_map.width + fx as usize];
            if tile == 22 { // PC tile
                self.phase = GamePhase::PCMenu { mode: 0, cursor: 0 };
                return;
            }
        }

        // Sign interaction
        if fx >= 0 && fy >= 0 {
            let col = self.current_map.collision_at(fx as usize, fy as usize);
            if col == CollisionType::Sign {
                let text = match self.current_map_id {
                    MapId::NewBarkTown => "NEW BARK TOWN\nWinds of a new beginning.",
                    MapId::CherrygroveCity => "CHERRYGROVE CITY\nThe city of fragrant flowers.",
                    MapId::Route29 => "ROUTE 29",
                    MapId::Route30 => "ROUTE 30",
                    MapId::Route31 => "ROUTE 31",
                    MapId::VioletCity => "VIOLET CITY\nThe city of nostalgic scents.",
                    MapId::VioletGym => "VIOLET CITY GYM\nLeader: FALKNER",
                    MapId::SproutTower => "SPROUT TOWER\nA tower of swaying pillars.",
                    MapId::Route32 => "ROUTE 32\nConnects Violet to Union Cave.",
                    MapId::UnionCave => "UNION CAVE\nA natural cave formation.",
                    _ => "...",
                };
                let lines: Vec<String> = text.split('\n').map(|s| s.to_string()).collect();
                self.dialogue = Some(DialogueState {
                    lines, current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
                self.phase = GamePhase::Dialogue;
            }
        }
    }

    // ─── Battle Logic ──────────────────────────────────

    fn step_battle(&mut self, engine: &mut Engine) {
        let dt = 1.0 / 60.0;

        // Take battle out to avoid borrow issues
        let mut battle = match self.battle.take() {
            Some(b) => b,
            None => { self.phase = GamePhase::Overworld; return; }
        };

        // Animate HP bars
        if let Some(player_pkmn) = self.party.get(battle.player_idx) {
            let target = player_pkmn.hp as f64;
            let diff = target - battle.player_hp_display;
            battle.player_hp_display += diff * 0.15;
            if diff.abs() < 0.5 { battle.player_hp_display = target; }
        }
        {
            let target = battle.enemy.hp as f64;
            let diff = target - battle.enemy_hp_display;
            battle.enemy_hp_display += diff * 0.15;
            if diff.abs() < 0.5 { battle.enemy_hp_display = target; }
        }

        // Export battle state to JS
        self.export_battle_state_from(&battle, engine);

        let phase = battle.phase.clone();
        match phase {
            BattlePhase::Intro { timer } => {
                let t = timer + dt;
                if t > 1.5 {
                    battle.enemy_hp_display = battle.enemy.hp as f64;
                    battle.phase = BattlePhase::ActionSelect { cursor: 0 };
                } else {
                    battle.phase = BattlePhase::Intro { timer: t };
                }
            }

            BattlePhase::ActionSelect { cursor } => {
                if is_down(engine) {
                    battle.phase = BattlePhase::ActionSelect { cursor: (cursor + 1) % 4 };
                } else if is_up(engine) {
                    battle.phase = BattlePhase::ActionSelect { cursor: if cursor == 0 { 3 } else { cursor - 1 } };
                } else if is_right(engine) {
                    battle.phase = BattlePhase::ActionSelect { cursor: (cursor + 2) % 4 };
                } else if is_left(engine) {
                    battle.phase = BattlePhase::ActionSelect { cursor: if cursor < 2 { cursor + 2 } else { cursor - 2 } };
                }

                if is_confirm(engine) {
                    match cursor {
                        0 => battle.phase = BattlePhase::MoveSelect { cursor: 0 },
                        1 => {
                            self.battle = Some(battle);
                            self.phase = GamePhase::BagMenu { cursor: 0 };
                            return;
                        }
                        2 => {
                            self.battle = Some(battle);
                            self.phase = GamePhase::PokemonMenu { cursor: 0 };
                            return;
                        }
                        3 => {
                            if battle.is_wild {
                                let pspeed = self.party.get(battle.player_idx).map(|p| p.speed).unwrap_or(50);
                                let espeed = battle.enemy.speed;
                                let chance = (pspeed as f64 * 128.0 / espeed as f64 + 30.0 * battle.turn_count as f64) / 256.0;
                                if engine.rng.next_f64() < chance || battle.turn_count > 3 {
                                    battle.phase = BattlePhase::Run;
                                } else {
                                    battle.phase = BattlePhase::RunFailed { timer: 0.0 };
                                }
                            } else {
                                battle.phase = BattlePhase::Text {
                                    message: "Can't run from trainer!".to_string(), timer: 0.0,
                                    next_phase: Box::new(BattlePhase::ActionSelect { cursor: 0 }),
                                };
                            }
                        }
                        _ => {}
                    }
                }
            }

            BattlePhase::MoveSelect { cursor } => {
                let move_count = self.party.get(battle.player_idx)
                    .map(|p| p.moves.iter().filter(|m| m.is_some()).count() as u8)
                    .unwrap_or(1).max(1);

                if is_down(engine) {
                    battle.phase = BattlePhase::MoveSelect { cursor: (cursor + 1) % move_count };
                } else if is_up(engine) {
                    battle.phase = BattlePhase::MoveSelect { cursor: if cursor == 0 { move_count - 1 } else { cursor - 1 } };
                }

                if is_cancel(engine) {
                    battle.phase = BattlePhase::ActionSelect { cursor: 0 };
                } else if is_confirm(engine) {
                    // Check if player Pokemon can move (sleep/freeze)
                    let can_move = self.party.get(battle.player_idx).map(|p| p.can_move()).unwrap_or(true);
                    // Paralysis: 25% chance to be fully paralyzed
                    let paralyzed = if let Some(p) = self.party.get(battle.player_idx) {
                        matches!(p.status, StatusCondition::Paralysis) && engine.rng.next_f64() < PARALYSIS_SKIP_CHANCE
                    } else { false };

                    if !can_move || paralyzed {
                        let reason = if paralyzed {
                            format!("{} is paralyzed! It can't move!", self.party.get(battle.player_idx).map(|p| p.name()).unwrap_or("???"))
                        } else {
                            format!("{} is fast asleep!", self.party.get(battle.player_idx).map(|p| p.name()).unwrap_or("???"))
                        };
                        let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
                        battle.phase = BattlePhase::Text {
                            message: reason, timer: 0.0,
                            next_phase: Box::new(BattlePhase::EnemyAttack {
                                timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                            }),
                        };
                        self.battle = Some(battle);
                        return;
                    }

                    // Check PP
                    let has_pp = self.party.get(battle.player_idx)
                        .map(|p| (cursor as usize) < 4 && p.move_pp[cursor as usize] > 0)
                        .unwrap_or(false);
                    if !has_pp {
                        // No PP for this move
                        battle.phase = BattlePhase::Text {
                            message: "No PP left for this move!".to_string(),
                            timer: 0.0,
                            next_phase: Box::new(BattlePhase::MoveSelect { cursor }),
                        };
                        self.battle = Some(battle);
                        return;
                    }

                    // Get player move
                    let move_id = self.party.get(battle.player_idx)
                        .and_then(|p| p.moves.get(cursor as usize).copied().flatten())
                        .unwrap_or(MOVE_TACKLE);

                    // Consume PP
                    if let Some(p) = self.party.get_mut(battle.player_idx) {
                        p.move_pp[cursor as usize] -= 1;
                    }

                    // Accuracy check (apply accuracy/evasion stages)
                    // Gen 2: all moves use accuracy + stage modifiers, including status
                    let accuracy_ok = if let Some(move_data) = get_move(move_id) {
                        if move_data.accuracy >= 255 {
                            true // Never-miss moves (Faint Attack, Swift)
                        } else {
                            let acc_mult = accuracy_stage_multiplier(battle.player_stages[STAGE_ACC]);
                            let eva_mult = accuracy_stage_multiplier(battle.enemy_stages[STAGE_EVA]);
                            let effective_acc = (move_data.accuracy as f64 * acc_mult / eva_mult).min(100.0);
                            if effective_acc >= 100.0 {
                                true
                            } else {
                                (engine.rng.next_u64() % 100) < effective_acc as u64
                            }
                        }
                    } else { true };

                    // Calc player damage (1/16 crit chance, Gen 2)
                    let p_crit = accuracy_ok && (engine.rng.next_u64() % CRIT_CHANCE) == 0;
                    let (p_damage, p_eff) = if !accuracy_ok {
                        (0, 1.0)
                    } else if let Some(move_data) = get_move(move_id) {
                        let species = get_species(battle.enemy.species_id);
                        let dt1 = species.map(|s| s.type1).unwrap_or(PokemonType::Normal);
                        let dt2 = species.and_then(|s| s.type2);
                        let rng = DAMAGE_ROLL_MIN + engine.rng.next_f64() * DAMAGE_ROLL_RANGE;
                        // Use Defense for Physical moves, Sp.Defense for Special moves
                        let def_stat = match move_data.category {
                            MoveCategory::Physical => battle.enemy.defense,
                            _ => battle.enemy.sp_defense,
                        };
                        // Stat stage multipliers (player attacking, enemy defending)
                        let atk_stage = match move_data.category {
                            MoveCategory::Physical => battle.player_stages[STAGE_ATK],
                            _ => battle.player_stages[STAGE_SPA],
                        };
                        let def_stage = match move_data.category {
                            MoveCategory::Physical => battle.enemy_stages[STAGE_DEF],
                            _ => battle.enemy_stages[STAGE_SPD],
                        };
                        // Critical hits ignore negative atk stages and positive def stages (Gen 2)
                        let atk_mult = if p_crit { stage_multiplier(atk_stage.max(0)) } else { stage_multiplier(atk_stage) };
                        let def_mult = if p_crit { stage_multiplier(def_stage.min(0)) } else { stage_multiplier(def_stage) };
                        if let Some(atk) = self.party.get(battle.player_idx) {
                            calc_damage(atk, def_stat, dt1, dt2, move_data, rng, p_crit, atk_mult, def_mult)
                        } else {
                            (0, 1.0)
                        }
                    } else {
                        (0, 1.0)
                    };

                    // Check speed for turn order (paralysis halves speed, apply speed stages)
                    let player_spd_stage = stage_multiplier(battle.player_stages[STAGE_SPE]);
                    let enemy_spd_stage = stage_multiplier(battle.enemy_stages[STAGE_SPE]);
                    let mut player_speed = (self.party.get(battle.player_idx).map(|p| p.speed).unwrap_or(0) as f64 * player_spd_stage) as u16;
                    if let Some(p) = self.party.get(battle.player_idx) {
                        if matches!(p.status, StatusCondition::Paralysis) {
                            player_speed /= 2;
                        }
                    }
                    let enemy_speed = (battle.enemy.speed as f64 * enemy_spd_stage) as u16;
                    if player_speed >= enemy_speed {
                        // Player goes first
                        battle.pending_player_move = None;
                        battle.phase = BattlePhase::PlayerAttack {
                            timer: 0.0, move_id, damage: p_damage, effectiveness: p_eff, is_crit: p_crit, from_pending: false,
                        };
                    } else {
                        // Enemy goes first — store player's move for after enemy's turn
                        battle.pending_player_move = Some((move_id, p_damage, p_eff, p_crit));
                        let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
                        battle.phase = BattlePhase::EnemyAttack {
                            timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                        };
                    }
                }
            }

            BattlePhase::PlayerAttack { timer, move_id, damage, effectiveness, is_crit, from_pending } => {
                let t = timer + dt;
                // Flash effect on hit at t=0.3
                if timer < 0.3 && t >= 0.3 && damage > 0 {
                    self.screen_flash = 0.6;
                    if effectiveness > 1.5 { self.screen_flash = 0.9; }
                    sfx_hit(engine, effectiveness > 1.5);
                }
                if t > 0.8 {
                    battle.enemy.hp = battle.enemy.hp.saturating_sub(damage);

                    // Check for status infliction from move
                    if damage > 0 {
                        let roll = engine.rng.next_f64();
                        try_inflict_status(&mut battle.enemy, move_id, roll);
                    }

                    let move_data_ref = get_move(move_id);
                    let move_name = move_data_ref.map(|m| m.name).unwrap_or("???");
                    let pname = self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default();
                    // Detect miss: damage=0 on a move with power, non-zero effectiveness
                    let is_miss = damage == 0
                        && move_data_ref.map(|m| m.power > 0 && m.category != MoveCategory::Status).unwrap_or(false)
                        && effectiveness > 0.0;
                    let eff = eff_text(effectiveness);
                    let crit_str = if is_crit { " Critical hit!" } else { "" };
                    let miss_str = if is_miss { " Attack missed!" } else { "" };
                    let msg = if !eff.is_empty() {
                        format!("{} used {}! {}{}{}", pname, move_name, eff, crit_str, miss_str)
                    } else if !miss_str.is_empty() {
                        format!("{} used {}!{}", pname, move_name, miss_str)
                    } else {
                        format!("{} used {}!{}", pname, move_name, crit_str)
                    };

                    // Apply stat stage effects for player's status moves
                    let stage_msg = if !is_miss {
                        if let Some((target_enemy, stat_idx, delta)) = status_move_stage_effect(move_id) {
                            let stages = if target_enemy { &mut battle.enemy_stages } else { &mut battle.player_stages };
                            let old = stages[stat_idx];
                            stages[stat_idx] = (old + delta).max(-6).min(6);
                            if stages[stat_idx] != old {
                                let stat_name = match stat_idx {
                                    STAGE_ATK => "Attack", STAGE_DEF => "Defense",
                                    STAGE_SPA => "Sp. Atk", STAGE_SPD => "Sp. Def",
                                    STAGE_SPE => "Speed", STAGE_ACC => "accuracy",
                                    _ => "evasion",
                                };
                                let target_name = if target_enemy {
                                    let prefix = if battle.is_wild { "Wild " } else { "Foe " };
                                    format!("{}{}", prefix, battle.enemy.name())
                                } else {
                                    pname.clone()
                                };
                                let change = if delta > 1 { "sharply rose!" } else if delta > 0 { "rose!" }
                                    else if delta < -1 { "sharply fell!" } else { "fell!" };
                                Some(format!("{}'s {} {}", target_name, stat_name, change))
                            } else {
                                let dir = if delta > 0 { "go any higher!" } else { "go any lower!" };
                                Some(format!("{} won't {}", match stat_idx {
                                    STAGE_ATK => "Attack", STAGE_DEF => "Defense",
                                    STAGE_SPA => "Sp. Atk", STAGE_SPD => "Sp. Def",
                                    STAGE_SPE => "Speed", STAGE_ACC => "accuracy",
                                    _ => "evasion",
                                }, dir))
                            }
                        } else { None }
                    } else { None };

                    // Helper: wrap next_phase with stat change text if present
                    let wrap_stat = |next: BattlePhase, sm: &Option<String>| -> Box<BattlePhase> {
                        if let Some(ref s) = sm {
                            Box::new(BattlePhase::Text { message: s.clone(), timer: 0.0, next_phase: Box::new(next) })
                        } else {
                            Box::new(next)
                        }
                    };

                    if battle.enemy.is_fainted() {
                        let exp = get_species(battle.enemy.species_id)
                            .map(|sp| exp_gained(sp, battle.enemy.level, battle.is_wild))
                            .unwrap_or(10);
                        battle.phase = BattlePhase::Text {
                            message: msg, timer: 0.0,
                            next_phase: wrap_stat(BattlePhase::EnemyFainted { exp_gained: exp }, &stage_msg),
                        };
                    } else if from_pending {
                        // Player's turn came from pending (enemy already attacked this turn)
                        // End-of-turn: apply status damage, tick status, return to ActionSelect
                        if let Some(p) = self.party.get_mut(battle.player_idx) {
                            p.apply_status_damage();
                            p.tick_status();
                        }
                        battle.enemy.apply_status_damage();
                        battle.enemy.tick_status();
                        battle.turn_count += 1;
                        if self.party.get(battle.player_idx).map(|p| p.is_fainted()).unwrap_or(false) {
                            battle.phase = BattlePhase::Text {
                                message: msg, timer: 0.0,
                                next_phase: wrap_stat(BattlePhase::PlayerFainted, &stage_msg),
                            };
                        } else if battle.enemy.is_fainted() {
                            let exp = get_species(battle.enemy.species_id)
                                .map(|sp| exp_gained(sp, battle.enemy.level, battle.is_wild))
                                .unwrap_or(10);
                            battle.phase = BattlePhase::Text {
                                message: msg, timer: 0.0,
                                next_phase: wrap_stat(BattlePhase::EnemyFainted { exp_gained: exp }, &stage_msg),
                            };
                        } else {
                            battle.phase = BattlePhase::Text {
                                message: msg, timer: 0.0,
                                next_phase: wrap_stat(BattlePhase::ActionSelect { cursor: 0 }, &stage_msg),
                            };
                        }
                    } else {
                        // Player went first — enemy gets to attack now
                        let enemy_can_move = battle.enemy.can_move();
                        let enemy_paralyzed = matches!(battle.enemy.status, StatusCondition::Paralysis) && engine.rng.next_f64() < PARALYSIS_SKIP_CHANCE;
                        if !enemy_can_move || enemy_paralyzed {
                            let prefix = if battle.is_wild { "Wild " } else { "Foe " };
                            let reason = if enemy_paralyzed {
                                format!("{}{} is paralyzed!", prefix, battle.enemy.name())
                            } else {
                                format!("{}{} is fast asleep!", prefix, battle.enemy.name())
                            };
                            battle.phase = BattlePhase::Text {
                                message: msg, timer: 0.0,
                                next_phase: wrap_stat(BattlePhase::Text {
                                    message: reason, timer: 0.0,
                                    next_phase: Box::new(BattlePhase::ActionSelect { cursor: 0 }),
                                }, &stage_msg),
                            };
                        } else {
                            let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
                            battle.phase = BattlePhase::Text {
                                message: msg, timer: 0.0,
                                next_phase: wrap_stat(BattlePhase::EnemyAttack {
                                    timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                                }, &stage_msg),
                            };
                        }
                    }
                } else {
                    battle.phase = BattlePhase::PlayerAttack { timer: t, move_id, damage, effectiveness, is_crit, from_pending };
                }
            }

            BattlePhase::EnemyAttack { timer, move_id, damage, effectiveness, is_crit } => {
                let t = timer + dt;
                // Screen shake on hit at t=0.3
                if timer < 0.3 && t >= 0.3 && damage > 0 {
                    self.screen_shake = if effectiveness > 1.5 { 6.0 } else { 3.0 };
                    sfx_hit(engine, effectiveness > 1.5);
                }
                if t > 0.8 {
                    if let Some(p) = self.party.get_mut(battle.player_idx) {
                        p.hp = p.hp.saturating_sub(damage);
                        if damage > 0 {
                            let roll = engine.rng.next_f64();
                            try_inflict_status(p, move_id, roll);
                        }
                    }

                    let move_data_ref = get_move(move_id);
                    let move_name = move_data_ref.map(|m| m.name).unwrap_or("???");
                    let ename = battle.enemy.name().to_string();
                    let is_miss = damage == 0
                        && move_data_ref.map(|m| m.power > 0 && m.category != MoveCategory::Status).unwrap_or(false)
                        && effectiveness > 0.0;
                    let eff = eff_text(effectiveness);
                    let prefix = if battle.is_wild { "Wild " } else { "Foe " };
                    let crit_str = if is_crit { " Critical hit!" } else { "" };
                    let miss_str = if is_miss { " Attack missed!" } else { "" };
                    let msg = if !eff.is_empty() {
                        format!("{}{} used {}! {}{}{}", prefix, ename, move_name, eff, crit_str, miss_str)
                    } else if !miss_str.is_empty() {
                        format!("{}{} used {}!{}", prefix, ename, move_name, miss_str)
                    } else {
                        format!("{}{} used {}!{}", prefix, ename, move_name, crit_str)
                    };

                    // Apply stat stage effects for enemy's status moves
                    let e_stage_msg = if !is_miss {
                        if let Some((target_enemy, stat_idx, delta)) = status_move_stage_effect(move_id) {
                            // For enemy's move: target_enemy means target the "enemy" from the move's perspective,
                            // but the enemy's enemy is the player
                            let stages = if target_enemy { &mut battle.player_stages } else { &mut battle.enemy_stages };
                            let old = stages[stat_idx];
                            stages[stat_idx] = (old + delta).max(-6).min(6);
                            if stages[stat_idx] != old {
                                let stat_name = match stat_idx {
                                    STAGE_ATK => "Attack", STAGE_DEF => "Defense",
                                    STAGE_SPA => "Sp. Atk", STAGE_SPD => "Sp. Def",
                                    STAGE_SPE => "Speed", STAGE_ACC => "accuracy",
                                    _ => "evasion",
                                };
                                let target_name = if target_enemy {
                                    // Enemy targets player
                                    self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default()
                                } else {
                                    let pfx = if battle.is_wild { "Wild " } else { "Foe " };
                                    format!("{}{}", pfx, battle.enemy.name())
                                };
                                let change = if delta > 1 { "sharply rose!" } else if delta > 0 { "rose!" }
                                    else if delta < -1 { "sharply fell!" } else { "fell!" };
                                Some(format!("{}'s {} {}", target_name, stat_name, change))
                            } else {
                                let dir = if delta > 0 { "go any higher!" } else { "go any lower!" };
                                Some(format!("{} won't {}", match stat_idx {
                                    STAGE_ATK => "Attack", STAGE_DEF => "Defense",
                                    STAGE_SPA => "Sp. Atk", STAGE_SPD => "Sp. Def",
                                    STAGE_SPE => "Speed", STAGE_ACC => "accuracy",
                                    _ => "evasion",
                                }, dir))
                            }
                        } else { None }
                    } else { None };

                    let fainted = self.party.get(battle.player_idx).map(|p| p.is_fainted()).unwrap_or(true);

                    // If player has a pending move, execute it next (enemy went first)
                    let has_pending = battle.pending_player_move.is_some();

                    let wrap_estat = |next: BattlePhase, sm: &Option<String>| -> Box<BattlePhase> {
                        if let Some(ref s) = sm {
                            Box::new(BattlePhase::Text { message: s.clone(), timer: 0.0, next_phase: Box::new(next) })
                        } else {
                            Box::new(next)
                        }
                    };

                    let next = if fainted {
                        BattlePhase::PlayerFainted
                    } else if has_pending {
                        // Player's turn now — extract pending move
                        let (pm_id, pm_dmg, pm_eff, pm_crit) = battle.pending_player_move.take().unwrap();
                        BattlePhase::PlayerAttack {
                            timer: 0.0, move_id: pm_id, damage: pm_dmg,
                            effectiveness: pm_eff, is_crit: pm_crit, from_pending: true,
                        }
                    } else {
                        // End-of-turn: apply status damage and tick status for both sides
                        let player_fainted_from_status;
                        if let Some(p) = self.party.get_mut(battle.player_idx) {
                            p.apply_status_damage();
                            p.tick_status();
                            player_fainted_from_status = p.is_fainted() && !fainted;
                        } else {
                            player_fainted_from_status = false;
                        }
                        let enemy_status_dmg = battle.enemy.apply_status_damage();
                        battle.enemy.tick_status();

                        if player_fainted_from_status {
                            BattlePhase::PlayerFainted
                        } else if enemy_status_dmg > 0 && battle.enemy.is_fainted() {
                            let exp = get_species(battle.enemy.species_id)
                                .map(|sp| exp_gained(sp, battle.enemy.level, battle.is_wild))
                                .unwrap_or(10);
                            BattlePhase::EnemyFainted { exp_gained: exp }
                        } else {
                            BattlePhase::ActionSelect { cursor: 0 }
                        }
                    };

                    if !has_pending { battle.turn_count += 1; }
                    battle.phase = BattlePhase::Text {
                        message: msg, timer: 0.0, next_phase: wrap_estat(next, &e_stage_msg),
                    };
                } else {
                    battle.phase = BattlePhase::EnemyAttack { timer: t, move_id, damage, effectiveness, is_crit };
                }
            }

            BattlePhase::Text { ref message, timer, ref next_phase } => {
                let t = timer + dt;
                if (is_confirm(engine) && t > 0.3) || t > 3.0 {
                    battle.phase = *next_phase.clone();
                } else {
                    battle.phase = BattlePhase::Text {
                        message: message.clone(), timer: t, next_phase: next_phase.clone(),
                    };
                }
            }

            BattlePhase::EnemyFainted { exp_gained: exp } => {
                sfx_faint(engine);
                if let Some(p) = self.party.get_mut(battle.player_idx) {
                    p.exp += exp;
                    if let Some(sp) = get_species(p.species_id) {
                        let next_exp = exp_for_level(p.level + 1, sp.growth_rate);
                        if p.exp >= next_exp && p.level < 100 {
                            p.level += 1;
                            p.recalc_stats();
                            // Check for new moves
                            let new_moves = p.check_new_moves();
                            for new_move in new_moves {
                                for i in 0..4 {
                                    if p.moves[i].is_none() {
                                        p.moves[i] = Some(new_move);
                                        if let Some(md) = get_move(new_move) {
                                            p.move_pp[i] = md.pp;
                                            p.move_max_pp[i] = md.pp;
                                        }
                                        break;
                                    }
                                }
                            }
                            // Check for evolution
                            let evo_species = get_species(p.species_id)
                                .and_then(|s| {
                                    if let (Some(evo_lvl), Some(evo_into)) = (s.evolution_level, s.evolution_into) {
                                        if p.level >= evo_lvl { Some(evo_into) } else { None }
                                    } else { None }
                                });
                            if let Some(evo) = evo_species {
                                // Set up evolution after level up display
                                battle.phase = BattlePhase::Text {
                                    message: format!("{} grew to LV{}!", p.name(), p.level),
                                    timer: 0.0,
                                    next_phase: Box::new(BattlePhase::Won { timer: 0.0 }),
                                };
                                self.battle = Some(battle);
                                // Schedule evolution after battle
                                self.phase = GamePhase::Battle;
                                // We'll handle evolution after Won phase
                                // Store pending evolution
                                engine.global_state.set_f64("pending_evolution", evo as f64);
                                return;
                            }
                            sfx_level_up(engine);
                            battle.phase = BattlePhase::LevelUp { timer: 0.0 };
                            self.battle = Some(battle);
                            return;
                        }
                    }
                }
                // Check if trainer has more Pokemon
                if !battle.is_wild && !battle.trainer_team.is_empty() {
                    let next_enemy = battle.trainer_team.remove(0);
                    battle.trainer_team_idx += 1;
                    let next_name = next_enemy.name().to_string();
                    battle.enemy = next_enemy;
                    battle.enemy_hp_display = battle.enemy.hp as f64;
                    battle.enemy_stages = [0; 7]; // Reset enemy stages on new Pokemon
                    battle.phase = BattlePhase::Text {
                        message: format!("Trainer sent out {}!", next_name),
                        timer: 0.0,
                        next_phase: Box::new(BattlePhase::ActionSelect { cursor: 0 }),
                    };
                } else {
                    battle.phase = BattlePhase::Won { timer: 0.0 };
                }
            }

            BattlePhase::LevelUp { timer } => {
                let t = timer + dt;
                if t > 2.0 || is_confirm(engine) {
                    // Check if trainer has more Pokemon
                    if !battle.is_wild && !battle.trainer_team.is_empty() {
                        let next_enemy = battle.trainer_team.remove(0);
                        battle.trainer_team_idx += 1;
                        let next_name = next_enemy.name().to_string();
                        battle.enemy = next_enemy;
                        battle.enemy_hp_display = battle.enemy.hp as f64;
                        battle.enemy_stages = [0; 7]; // Reset enemy stages on new Pokemon
                        battle.phase = BattlePhase::Text {
                            message: format!("Trainer sent out {}!", next_name),
                            timer: 0.0,
                            next_phase: Box::new(BattlePhase::ActionSelect { cursor: 0 }),
                        };
                    } else {
                        battle.phase = BattlePhase::Won { timer: 0.0 };
                    }
                } else {
                    battle.phase = BattlePhase::LevelUp { timer: t };
                }
            }

            BattlePhase::Won { timer } => {
                let t = timer + dt;
                if t > 1.0 {
                    // Trainer battle rewards
                    if !battle.is_wild {
                        if let Some((map_id, npc_idx)) = self.trainer_battle_npc.take() {
                            let reward = 100 + (battle.enemy.level as u32) * 10;
                            self.money += reward;
                            self.defeated_trainers.push((map_id, npc_idx));

                            engine.global_state.set_f64("in_battle", 0.0);
                            self.battle = None;

                            // Check for pending evolution first
                            let pending_evo = engine.global_state.get_f64("pending_evolution").unwrap_or(0.0) as u16;
                            if pending_evo > 0 {
                                engine.global_state.set_f64("pending_evolution", 0.0);
                                self.phase = GamePhase::Evolution { timer: 0.0, new_species: pending_evo };
                            } else {
                                // Check if this was a gym leader battle
                                let badge_action = match (map_id, npc_idx) {
                                    (MapId::VioletGym, 0) => Some(DialogueAction::GiveBadge { badge_num: 0 }),
                                    (MapId::AzaleaGym, 0) => Some(DialogueAction::GiveBadge { badge_num: 1 }),
                                    (MapId::GoldenrodGym, 0) => Some(DialogueAction::GiveBadge { badge_num: 2 }),
                                    (MapId::EcruteakGym, 0) => Some(DialogueAction::GiveBadge { badge_num: 3 }),
                                    (MapId::OlivineGym, 0) => Some(DialogueAction::GiveBadge { badge_num: 4 }),
                                    _ => None,
                                };

                                let reward_text = format!("Got ${} for winning!", reward);
                                self.dialogue = Some(DialogueState {
                                    lines: vec![reward_text],
                                    current_line: 0, char_index: 0, timer: 0.0,
                                    on_complete: badge_action.unwrap_or(DialogueAction::None),
                                });
                                self.phase = GamePhase::Dialogue;
                            }
                            return;
                        }
                    }

                    engine.global_state.set_f64("in_battle", 0.0);
                    self.battle = None;

                    // Check for pending evolution
                    let pending_evo = engine.global_state.get_f64("pending_evolution").unwrap_or(0.0) as u16;
                    if pending_evo > 0 {
                        engine.global_state.set_f64("pending_evolution", 0.0);
                        self.phase = GamePhase::Evolution { timer: 0.0, new_species: pending_evo };
                    } else {
                        self.phase = GamePhase::Overworld;
                    }
                    return;
                }
                battle.phase = BattlePhase::Won { timer: t };
            }

            BattlePhase::Run => {
                engine.global_state.set_f64("in_battle", 0.0);
                self.phase = GamePhase::Overworld;
                self.battle = None;
                return;
            }

            BattlePhase::RunFailed { timer } => {
                let t = timer + dt;
                if t > 1.0 {
                    let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
                    battle.phase = BattlePhase::EnemyAttack {
                        timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                    };
                } else {
                    battle.phase = BattlePhase::RunFailed { timer: t };
                }
            }

            BattlePhase::PlayerFainted => {
                sfx_faint(engine);
                let any_alive = self.party.iter().any(|p| !p.is_fainted());
                if any_alive {
                    // Auto-switch to next alive Pokemon — reset player stages
                    battle.player_stages = [0; 7];
                    for (i, p) in self.party.iter().enumerate() {
                        if !p.is_fainted() {
                            let pname = p.name().to_string();
                            battle.player_idx = i;
                            battle.player_hp_display = p.hp as f64;
                            battle.phase = BattlePhase::Text {
                                message: format!("Go! {}!", pname),
                                timer: 0.0,
                                next_phase: Box::new(BattlePhase::ActionSelect { cursor: 0 }),
                            };
                            break;
                        }
                    }
                } else {
                    // Whiteout - lose half money, heal, warp to Pokemon Center
                    let lost = self.money / 2;
                    self.money -= lost;
                    for p in &mut self.party { p.heal(); }
                    engine.global_state.set_f64("in_battle", 0.0);
                    self.battle = None;
                    self.dialogue = Some(DialogueState {
                        lines: vec![
                            "You are out of usable".to_string(),
                            "POKEMON!".to_string(),
                            "You blacked out!".to_string(),
                            format!("You lost ${}...", lost),
                        ],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.change_map(MapId::PokemonCenter, 5, 6);
                    self.phase = GamePhase::Dialogue;
                    return;
                }
            }
        }

        self.battle = Some(battle);
    }

    fn calc_enemy_move(&self, engine: &mut Engine, enemy: &Pokemon, player_idx: usize, enemy_stages: &[i8; 7], player_stages: &[i8; 7]) -> (MoveId, u16, f64, bool) {
        let available: Vec<MoveId> = enemy.moves.iter().filter_map(|m| *m).collect();
        if available.is_empty() { return (MOVE_TACKLE, 5, 1.0, false); }

        // Smart AI: 50% chance to pick best move by effectiveness, 50% random
        let mid = if let Some(pp) = self.party.get(player_idx) {
            let sp = get_species(pp.species_id);
            let dt1 = sp.map(|s| s.type1).unwrap_or(PokemonType::Normal);
            let dt2 = sp.and_then(|s| s.type2);
            let use_smart = engine.rng.next_u64() % 2 == 0;
            if use_smart {
                // Pick the move with highest effectiveness (ties broken by power)
                let mut best = available[0];
                let mut best_score = 0.0_f64;
                for &m in &available {
                    if let Some(md) = get_move(m) {
                        let eff = combined_effectiveness(md.move_type, dt1, dt2);
                        let score = eff * md.power as f64;
                        if score > best_score {
                            best_score = score;
                            best = m;
                        }
                    }
                }
                best
            } else {
                available[(engine.rng.next_u64() as usize) % available.len()]
            }
        } else {
            available[(engine.rng.next_u64() as usize) % available.len()]
        };

        // Accuracy check for enemy move (apply accuracy/evasion stages)
        // Gen 2: all moves use accuracy + stage modifiers, including status
        let accuracy_ok = if let Some(md) = get_move(mid) {
            if md.accuracy >= 255 {
                true // Never-miss moves (Faint Attack, Swift)
            } else {
                let acc_mult = accuracy_stage_multiplier(enemy_stages[STAGE_ACC]);
                let eva_mult = accuracy_stage_multiplier(player_stages[STAGE_EVA]);
                let effective_acc = (md.accuracy as f64 * acc_mult / eva_mult).min(100.0);
                if effective_acc >= 100.0 {
                    true
                } else {
                    (engine.rng.next_u64() % 100) < effective_acc as u64
                }
            }
        } else { true };

        let is_crit = accuracy_ok && (engine.rng.next_u64() % CRIT_CHANCE) == 0;
        if !accuracy_ok {
            (mid, 0, 1.0, false) // miss — zero damage
        } else if let (Some(md), Some(pp)) = (get_move(mid), self.party.get(player_idx)) {
            let sp = get_species(pp.species_id);
            let dt1 = sp.map(|s| s.type1).unwrap_or(PokemonType::Normal);
            let dt2 = sp.and_then(|s| s.type2);
            let rng = DAMAGE_ROLL_MIN + engine.rng.next_f64() * DAMAGE_ROLL_RANGE;
            // Use Defense for Physical moves, Sp.Defense for Special moves
            let def_stat = match md.category {
                MoveCategory::Physical => pp.defense,
                _ => pp.sp_defense,
            };
            // Stat stage multipliers (enemy attacking, player defending)
            let atk_stage = match md.category {
                MoveCategory::Physical => enemy_stages[STAGE_ATK],
                _ => enemy_stages[STAGE_SPA],
            };
            let def_stage = match md.category {
                MoveCategory::Physical => player_stages[STAGE_DEF],
                _ => player_stages[STAGE_SPD],
            };
            let atk_mult = if is_crit { stage_multiplier(atk_stage.max(0)) } else { stage_multiplier(atk_stage) };
            let def_mult = if is_crit { stage_multiplier(def_stage.min(0)) } else { stage_multiplier(def_stage) };
            let (dmg, eff) = calc_damage(enemy, def_stat, dt1, dt2, md, rng, is_crit, atk_mult, def_mult);
            (mid, dmg, eff, is_crit)
        } else {
            (mid, 5, 1.0, false)
        }
    }

    fn export_battle_state_from(&self, battle: &BattleState, engine: &mut Engine) {
        engine.global_state.set_f64("in_battle", 1.0);
        if let Some(sp) = get_species(battle.enemy.species_id) {
            engine.global_state.set_str("enemy_pokemon", &sp.name.to_lowercase());
            engine.global_state.set_f64("enemy_level", battle.enemy.level as f64);
            engine.global_state.set_f64("enemy_hp", battle.enemy.hp as f64);
            engine.global_state.set_f64("enemy_max_hp", battle.enemy.max_hp as f64);
        }
        if let Some(pp) = self.party.get(battle.player_idx) {
            if let Some(sp) = get_species(pp.species_id) {
                engine.global_state.set_str("player_pokemon", &sp.name.to_lowercase());
                engine.global_state.set_f64("player_level", pp.level as f64);
                engine.global_state.set_f64("player_hp", pp.hp as f64);
                engine.global_state.set_f64("player_max_hp", pp.max_hp as f64);
            }
        }
    }

    // ─── Dialogue Logic ────────────────────────────────

    fn step_dialogue(&mut self, engine: &mut Engine) {
        let dt = 1.0 / 60.0;
        let mut done = false;
        let mut action = DialogueAction::None;

        if let Some(dialogue) = &mut self.dialogue {
            dialogue.timer += dt;
            let chars_per_sec = 30.0;
            let target = (dialogue.timer * chars_per_sec) as usize;
            let line_len = dialogue.lines.get(dialogue.current_line).map(|l| l.len()).unwrap_or(0);
            dialogue.char_index = target.min(line_len);

            let confirm = is_confirm(engine);

            if confirm {
                if dialogue.char_index < line_len {
                    dialogue.char_index = line_len;
                    dialogue.timer = line_len as f64 / chars_per_sec;
                } else if dialogue.current_line + 1 < dialogue.lines.len() {
                    dialogue.current_line += 1;
                    dialogue.char_index = 0;
                    dialogue.timer = 0.0;
                } else {
                    action = dialogue.on_complete.clone();
                    done = true;
                }
            }
        }

        if done {
            self.dialogue = None;
            match action {
                DialogueAction::None => {
                    if self.battle.is_some() {
                        // Return to battle (e.g. after using potion/item in battle)
                        self.phase = GamePhase::Battle;
                    } else {
                        self.phase = GamePhase::Overworld;
                    }
                }
                DialogueAction::Heal => {
                    for p in &mut self.party { p.heal(); }
                    sfx_heal(engine);
                    self.phase = GamePhase::Healing { timer: 0.0 };
                }
                DialogueAction::GiveStarter => {
                    if !self.has_starter {
                        self.phase = GamePhase::StarterSelect { cursor: 0 };
                    } else {
                        self.phase = GamePhase::Overworld;
                    }
                }
                DialogueAction::OpenMart => {
                    self.phase = GamePhase::PokeMart { cursor: 0 };
                }
                DialogueAction::GiveBadge { badge_num } => {
                    self.badges |= 1 << badge_num;
                    let badge_name = match badge_num {
                        0 => "ZEPHYR BADGE",
                        1 => "HIVE BADGE",
                        2 => "PLAIN BADGE",
                        3 => "FOG BADGE",
                        4 => "MINERAL BADGE",
                        _ => "BADGE",
                    };
                    self.dialogue = Some(DialogueState {
                        lines: vec![
                            format!("Received the {}!", badge_name),
                            "Pokemon up to LV 20 will obey!".to_string(),
                        ],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                }
                DialogueAction::StartTrainerBattle { team } => {
                    if let Some(&(species, level)) = team.first() {
                        self.register_seen(species);
                        let enemy = Pokemon::new(species, level);
                        let player_idx = self.party.iter().position(|p| !p.is_fainted()).unwrap_or(0);
                        let player_hp = self.party.get(player_idx).map(|p| p.hp as f64).unwrap_or(0.0);
                        // Build remaining team (all except the lead)
                        let remaining: Vec<Pokemon> = team.iter().skip(1)
                            .map(|&(s, l)| {
                                self.register_seen(s);
                                Pokemon::new(s, l)
                            })
                            .collect();
                        self.battle = Some(BattleState {
                            phase: BattlePhase::Intro { timer: 0.0 },
                            enemy,
                            player_idx,
                            is_wild: false,
                            player_hp_display: player_hp,
                            enemy_hp_display: 0.0,
                            turn_count: 0,
                            trainer_team: remaining,
                            trainer_team_idx: 0,
                            pending_player_move: None,
                            player_stages: [0; 7],
                            enemy_stages: [0; 7],
                        });
                        self.encounter_flash_count = 0;
                        self.phase = GamePhase::EncounterTransition { timer: 0.0 };
                    } else {
                        // Safety fallback: empty team, return to overworld
                        self.phase = GamePhase::Overworld;
                    }
                }
            }
        }
    }

    // ─── Menu Logic ────────────────────────────────────

    fn step_menu(&mut self, engine: &mut Engine) {
        let items = 5u8;
        if is_down(engine) {
            self.menu_cursor = (self.menu_cursor + 1) % items;
        } else if is_up(engine) {
            self.menu_cursor = if self.menu_cursor == 0 { items - 1 } else { self.menu_cursor - 1 };
        }

        let confirm = is_confirm(engine);
        let cancel = is_cancel(engine);

        if cancel { self.phase = GamePhase::Overworld; return; }

        if confirm {
            sfx_select(engine);
            match self.menu_cursor {
                0 => self.phase = GamePhase::PokemonMenu { cursor: 0 },
                1 => self.phase = GamePhase::BagMenu { cursor: 0 },
                2 => self.phase = GamePhase::Pokedex { cursor: 0, scroll: 0 },
                3 => {
                    // Save: export state to global_state for JS to persist
                    engine.global_state.set_f64("save_money", self.money as f64);
                    engine.global_state.set_f64("save_party_size", self.party.len() as f64);
                    engine.global_state.set_f64("savebadges", self.badges as f64);
                    engine.global_state.set_f64("save_pokedex_seen", self.pokedex_seen.len() as f64);
                    engine.global_state.set_f64("save_pokedex_caught", self.pokedex_caught.len() as f64);
                    self.dialogue = Some(DialogueState {
                        lines: vec!["Game saved!".to_string()],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                }
                4 => self.phase = GamePhase::Overworld,
                _ => {}
            }
        }
    }

    fn step_pokemon_menu(&mut self, engine: &mut Engine) {
        let cursor = if let GamePhase::PokemonMenu { cursor } = &self.phase { *cursor } else { 0 };
        let party_size = self.party.len() as u8;
        if party_size == 0 { self.phase = GamePhase::Overworld; return; }

        if is_down(engine) {
            self.phase = GamePhase::PokemonMenu { cursor: (cursor + 1) % party_size };
        } else if is_up(engine) {
            self.phase = GamePhase::PokemonMenu { cursor: if cursor == 0 { party_size - 1 } else { cursor - 1 } };
        }

        let confirm = is_confirm(engine);

        if is_cancel(engine) {
            self.phase = if self.battle.is_some() { GamePhase::Battle } else { GamePhase::Menu };
            return;
        }

        if confirm {
            if self.battle.is_some() {
                // Switch Pokemon in battle
                let selected = cursor as usize;
                if let Some(battle) = &self.battle {
                    if selected == battle.player_idx {
                        // Already out
                        return;
                    }
                }
                if let Some(pkmn) = self.party.get(selected) {
                    if pkmn.is_fainted() {
                        return;
                    }
                    if let Some(battle) = &mut self.battle {
                        battle.player_idx = selected;
                        battle.player_hp_display = self.party[selected].hp as f64;
                        battle.player_stages = [0; 7]; // Reset player stages on switch
                        let pname = self.party[selected].name().to_string();
                        battle.phase = BattlePhase::Text {
                            message: format!("Go! {}!", pname),
                            timer: 0.0,
                            next_phase: Box::new(BattlePhase::ActionSelect { cursor: 0 }),
                        };
                    }
                    self.phase = GamePhase::Battle;
                }
            } else {
                // Show summary screen
                self.phase = GamePhase::PokemonSummary { index: cursor };
            }
        }
    }

    fn step_pokemon_summary(&mut self, engine: &mut Engine) {
        let cancel = is_cancel(engine) || is_confirm(engine);
        if cancel {
            self.phase = GamePhase::PokemonMenu { cursor: 0 };
        }
    }

    fn step_pc_menu(&mut self, engine: &mut Engine) {
        let (mode, cursor) = if let GamePhase::PCMenu { mode, cursor } = &self.phase {
            (*mode, *cursor)
        } else { return; };

        if mode == 0 {
            // Mode select: WITHDRAW / DEPOSIT / CLOSE
            if is_down(engine) {
                self.phase = GamePhase::PCMenu { mode: 0, cursor: (cursor + 1) % 3 };
            } else if is_up(engine) {
                self.phase = GamePhase::PCMenu { mode: 0, cursor: if cursor == 0 { 2 } else { cursor - 1 } };
            }
            if is_cancel(engine) { self.phase = GamePhase::Overworld; return; }
            if is_confirm(engine) {
                match cursor {
                    0 => self.phase = GamePhase::PCMenu { mode: 1, cursor: 0 }, // withdraw
                    1 => self.phase = GamePhase::PCMenu { mode: 2, cursor: 0 }, // deposit
                    _ => self.phase = GamePhase::Overworld,
                }
            }
        } else if mode == 1 {
            // Withdraw mode
            let pc_count = self.pc_boxes.len() as u8;
            let total = pc_count + 1; // +1 for BACK
            if is_down(engine) {
                self.phase = GamePhase::PCMenu { mode: 1, cursor: (cursor + 1) % total };
            } else if is_up(engine) {
                self.phase = GamePhase::PCMenu { mode: 1, cursor: if cursor == 0 { total - 1 } else { cursor - 1 } };
            }
            if is_cancel(engine) { self.phase = GamePhase::PCMenu { mode: 0, cursor: 0 }; return; }
            if is_confirm(engine) {
                if cursor == pc_count {
                    self.phase = GamePhase::PCMenu { mode: 0, cursor: 0 };
                    return;
                }
                if self.party.len() >= 6 {
                    self.dialogue = Some(DialogueState {
                        lines: vec!["Your party is full!".to_string()],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                    return;
                }
                if (cursor as usize) < self.pc_boxes.len() {
                    let pkmn = self.pc_boxes.remove(cursor as usize);
                    let name = pkmn.name().to_string();
                    self.party.push(pkmn);
                    self.dialogue = Some(DialogueState {
                        lines: vec![format!("Withdrew {}!", name)],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                }
            }
        } else {
            // Deposit mode
            let party_count = self.party.len() as u8;
            let total = party_count + 1; // +1 for BACK
            if is_down(engine) {
                self.phase = GamePhase::PCMenu { mode: 2, cursor: (cursor + 1) % total };
            } else if is_up(engine) {
                self.phase = GamePhase::PCMenu { mode: 2, cursor: if cursor == 0 { total - 1 } else { cursor - 1 } };
            }
            if is_cancel(engine) { self.phase = GamePhase::PCMenu { mode: 0, cursor: 0 }; return; }
            if is_confirm(engine) {
                if cursor == party_count {
                    self.phase = GamePhase::PCMenu { mode: 0, cursor: 0 };
                    return;
                }
                if self.party.len() <= 1 {
                    self.dialogue = Some(DialogueState {
                        lines: vec!["Can't deposit your last Pokemon!".to_string()],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                    return;
                }
                if (cursor as usize) < self.party.len() {
                    let pkmn = self.party.remove(cursor as usize);
                    let name = pkmn.name().to_string();
                    self.pc_boxes.push(pkmn);
                    self.dialogue = Some(DialogueState {
                        lines: vec![format!("Deposited {}!", name)],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                }
            }
        }
    }

    fn register_seen(&mut self, species: SpeciesId) {
        if !self.pokedex_seen.contains(&species) {
            self.pokedex_seen.push(species);
        }
    }

    fn register_caught(&mut self, species: SpeciesId) {
        self.register_seen(species);
        if !self.pokedex_caught.contains(&species) {
            self.pokedex_caught.push(species);
        }
    }

    fn step_pokedex(&mut self, engine: &mut Engine) {
        let (cursor, scroll) = if let GamePhase::Pokedex { cursor, scroll } = &self.phase {
            (*cursor, *scroll)
        } else { return; };

        let total = self.pokedex_seen.len() as u8;
        if total == 0 {
            if is_cancel(engine) {
                self.phase = GamePhase::Menu;
            }
            return;
        }

        if is_down(engine) {
            let new_cursor = if cursor + 1 < total { cursor + 1 } else { 0 };
            let new_scroll = if new_cursor >= scroll + 6 { new_cursor.saturating_sub(5) } else if new_cursor < scroll { new_cursor } else { scroll };
            self.phase = GamePhase::Pokedex { cursor: new_cursor, scroll: new_scroll };
        } else if is_up(engine) {
            let new_cursor = if cursor > 0 { cursor - 1 } else { total.saturating_sub(1) };
            let new_scroll = if new_cursor < scroll { new_cursor } else if new_cursor >= scroll + 6 { new_cursor.saturating_sub(5) } else { scroll };
            self.phase = GamePhase::Pokedex { cursor: new_cursor, scroll: new_scroll };
        }

        if is_cancel(engine) {
            self.phase = GamePhase::Menu;
        }
    }

    // ─── Poke Mart Logic ──────────────────────────────────

    fn step_pokemart(&mut self, engine: &mut Engine) {
        let cursor = if let GamePhase::PokeMart { cursor } = &self.phase { *cursor } else { 0 };

        let item_count = MART_INVENTORY.len() as u8;

        if is_down(engine) {
            self.phase = GamePhase::PokeMart { cursor: (cursor + 1) % item_count };
        } else if is_up(engine) {
            self.phase = GamePhase::PokeMart { cursor: if cursor == 0 { item_count - 1 } else { cursor - 1 } };
        }

        let confirm = is_confirm(engine);
        let cancel = is_cancel(engine);

        if cancel {
            self.dialogue = Some(DialogueState {
                lines: vec!["Come again!".to_string()],
                current_line: 0, char_index: 0, timer: 0.0,
                on_complete: DialogueAction::None,
            });
            self.phase = GamePhase::Dialogue;
            return;
        }

        if confirm {
            if let Some(&(item_id, price)) = MART_INVENTORY.get(cursor as usize) {
                if self.money >= price as u32 {
                    self.money -= price as u32;
                    self.bag.add_item(item_id, 1);
                    let name = get_item(item_id).map(|i| i.name).unwrap_or("???");
                    self.dialogue = Some(DialogueState {
                        lines: vec![format!("Bought {} for ${}!", name, price)],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                } else {
                    self.dialogue = Some(DialogueState {
                        lines: vec!["Not enough money!".to_string()],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════
    // ─── RENDERING ─────────────────────────────────────
    // ═══════════════════════════════════════════════════

    fn render_title(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        fill_virtual_screen(fb, ctx, Color::from_rgba(8, 8, 24, 255));

        let gold = Color::from_rgba(248, 208, 48, 255);
        let shadow = Color::from_rgba(128, 80, 0, 255);

        draw_text_shadowed(fb, ctx, "POKEMON", 40, 25, gold, shadow);
        draw_text_shadowed(fb, ctx, "GOLD VERSION", 25, 45,
            Color::from_rgba(248, 176, 48, 255), Color::from_rgba(96, 64, 0, 255));

        fill_rect_v(fb, ctx, 20, 58, 120, 1, Color::from_rgba(248, 208, 48, 128));

        if (self.title_blink_timer * 2.0) as u32 % 2 == 0 {
            draw_text_pkmn(fb, ctx, "PRESS START", 32, 100, Color::from_rgba(248, 248, 248, 255));
        }

        draw_text_pkmn(fb, ctx, "CRUSTY ENGINE", 28, 125, Color::from_rgba(120, 120, 140, 255));
        draw_text_pkmn(fb, ctx, "V0.1", 65, 135, Color::from_rgba(80, 80, 100, 255));
    }

    fn render_starter_select(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, cursor: u8) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        fill_virtual_screen(fb, ctx, Color::from_rgba(248, 248, 248, 255));

        draw_text_pkmn(fb, ctx, "CHOOSE YOUR POKEMON!", 5, 5, Color::from_rgba(40, 40, 48, 255));

        let starters = ["CHIKORITA", "CYNDAQUIL", "TOTODILE"];
        let types = ["GRASS", "FIRE", "WATER"];
        let colors = [
            Color::from_rgba(104, 200, 80, 255),
            Color::from_rgba(240, 128, 48, 255),
            Color::from_rgba(80, 144, 240, 255),
        ];

        for i in 0..3 {
            let y = 25 + i as i32 * 35;
            if i as u8 == cursor {
                fill_rect_v(fb, ctx, 2, y - 2, 156, 30, Color::from_rgba(232, 240, 248, 255));
                draw_cursor(fb, ctx, 4, y + 8, Color::from_rgba(40, 40, 48, 255));
            }
            fill_rect_v(fb, ctx, 16, y + 4, 16, 16, colors[i]);
            draw_text_pkmn(fb, ctx, starters[i], 38, y + 4, Color::from_rgba(40, 40, 48, 255));
            draw_text_pkmn(fb, ctx, types[i], 38, y + 16, colors[i]);
        }

        draw_text_pkmn(fb, ctx, "Z/ENTER TO SELECT", 10, 132, Color::from_rgba(120, 120, 140, 255));
    }

    fn update_camera(&mut self) {
        let ppx = self.player.x as f64 * TILE_PX as f64;
        let ppy = self.player.y as f64 * TILE_PX as f64;

        let (wdx, wdy) = if self.player.is_walking {
            match self.player.facing {
                Direction::Up => (0.0, -self.player.walk_offset * TILE_PX as f64),
                Direction::Down => (0.0, self.player.walk_offset * TILE_PX as f64),
                Direction::Left => (-self.player.walk_offset * TILE_PX as f64, 0.0),
                Direction::Right => (self.player.walk_offset * TILE_PX as f64, 0.0),
            }
        } else { (0.0, 0.0) };

        let target_x = ppx + wdx + TILE_PX as f64 / 2.0 - (VIEW_TILES_X * TILE_PX / 2) as f64;
        let target_y = ppy + wdy + TILE_PX as f64 / 2.0 - (VIEW_TILES_Y * TILE_PX / 2) as f64;

        let map_pw = (self.current_map.width as i32 * TILE_PX) as f64;
        let map_ph = (self.current_map.height as i32 * TILE_PX) as f64;
        let vw = (VIEW_TILES_X * TILE_PX) as f64;
        let vh = (VIEW_TILES_Y * TILE_PX) as f64;
        let clamped_x = target_x.max(0.0).min((map_pw - vw).max(0.0));
        let clamped_y = target_y.max(0.0).min((map_ph - vh).max(0.0));

        let lerp = CAMERA_LERP;
        self.camera_x += (clamped_x - self.camera_x) * lerp;
        self.camera_y += (clamped_y - self.camera_y) * lerp;

        // Snap when very close to avoid sub-pixel jitter
        if (clamped_x - self.camera_x).abs() < 0.1 { self.camera_x = clamped_x; }
        if (clamped_y - self.camera_y).abs() < 0.1 { self.camera_y = clamped_y; }
    }

    fn render_overworld(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        let scale = ctx.scale;

        fb.clear(Color::from_rgba(8, 8, 16, 255));

        let cam_x = self.camera_x;
        let cam_y = self.camera_y;

        let stx = (cam_x / TILE_PX as f64).floor() as i32;
        let sty = (cam_y / TILE_PX as f64).floor() as i32;
        let etx = stx + VIEW_TILES_X + 2;
        let ety = sty + VIEW_TILES_Y + 2;

        for ty in sty..ety {
            for tx in stx..etx {
                if tx < 0 || ty < 0 || tx as usize >= self.current_map.width || ty as usize >= self.current_map.height { continue; }
                let tile_id = self.current_map.tiles[ty as usize * self.current_map.width + tx as usize];
                let actual_id = if tile_id == 5 && self.water_frame == 1 { 6 } else { tile_id };

                let sx = (tx as f64 * TILE_PX as f64 - cam_x) as i32;
                let sy = (ty as f64 * TILE_PX as f64 - cam_y) as i32;
                let (fbx, fby) = ctx.to_fb(sx, sy);

                if let Some(sd) = self.tile_cache.get(actual_id as usize) {
                    draw_sprite(fb, sd, TILE_W, TILE_H, fbx, fby, scale, tile_palette(tile_id));
                }
            }
        }

        // NPCs
        for npc in &self.current_map.npcs {
            let sx = (npc.x as f64 * TILE_PX as f64 - cam_x) as i32;
            let sy = (npc.y as f64 * TILE_PX as f64 - cam_y) as i32;
            let (fx, fy) = ctx.to_fb(sx, sy);
            if let Some(sd) = self.npc_sprite_cache.get(npc.sprite_id as usize) {
                draw_sprite(fb, sd, NPC_W, NPC_H, fx, fy, scale, npc_palette(npc.sprite_id));
            }
        }

        // Player
        let ppx = self.player.x as f64 * TILE_PX as f64;
        let ppy = self.player.y as f64 * TILE_PX as f64;
        let (wdx, wdy) = if self.player.is_walking {
            match self.player.facing {
                Direction::Up => (0.0, -self.player.walk_offset * TILE_PX as f64),
                Direction::Down => (0.0, self.player.walk_offset * TILE_PX as f64),
                Direction::Left => (-self.player.walk_offset * TILE_PX as f64, 0.0),
                Direction::Right => (self.player.walk_offset * TILE_PX as f64, 0.0),
            }
        } else { (0.0, 0.0) };
        let psx = (ppx + wdx - cam_x) as i32;
        let psy = (ppy + wdy - cam_y) as i32;
        let (pfx, pfy) = ctx.to_fb(psx, psy);
        let dir_off = match self.player.facing {
            Direction::Down => 0, Direction::Up => 3, Direction::Left => 6, Direction::Right => 9,
        };
        let si = dir_off + self.player.walk_frame as usize;
        if let Some(sd) = self.player_sprite_cache.get(si) {
            draw_sprite(fb, sd, PLAYER_W, PLAYER_H, pfx, pfy, scale, &PAL_PLAYER);
        }

        // Day/night tint overlay
        if self.day_night_tint > 0.01 {
            let a = (self.day_night_tint * 180.0).min(180.0) as u8;
            fill_virtual_screen(fb, ctx, Color::from_rgba(16, 16, 64, a));
        }

        // Map name
        draw_text_pkmn(fb, ctx, self.current_map.name, 4, 2, Color::from_rgba(248, 248, 248, 200));

        // Time of day indicator
        let time_str = if self.time_of_day < 5.0 || self.time_of_day >= 19.0 {
            "NIGHT"
        } else if self.time_of_day < 7.0 {
            "DAWN"
        } else if self.time_of_day < 17.0 {
            "DAY"
        } else {
            "DUSK"
        };
        draw_text_pkmn(fb, ctx, time_str, 126, 2, Color::from_rgba(200, 200, 220, 150));
    }

    fn render_battle(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        let battle = match &self.battle { Some(b) => b, None => return };
        let dark = Color::from_rgba(40, 40, 48, 255);

        fill_virtual_screen(fb, ctx, Color::from_rgba(248, 248, 248, 255));

        // Platforms
        fill_rect_v(fb, ctx, 80, 48, 78, 3, Color::from_rgba(168, 168, 176, 255));
        fill_rect_v(fb, ctx, 2, 88, 78, 3, Color::from_rgba(168, 168, 176, 255));

        // Enemy info
        draw_text_box(fb, ctx, 2, 2, 76, 28);
        draw_text_pkmn(fb, ctx, battle.enemy.name(), 8, 6, dark);
        let elvl = format!("LV{}", battle.enemy.level);
        draw_text_pkmn(fb, ctx, &elvl, 52, 6, dark);
        draw_hp_bar(fb, ctx, 22, 20, 48, battle.enemy_hp_display as u16, battle.enemy.max_hp);

        // Enemy status condition
        let enemy_status_text = status_text(&battle.enemy.status);
        if !enemy_status_text.is_empty() {
            draw_text_pkmn(fb, ctx, enemy_status_text, 8, 26, status_color(&battle.enemy.status));
        }

        // Player info
        draw_text_box(fb, ctx, 82, 56, 76, 40);
        if let Some(pp) = self.party.get(battle.player_idx) {
            draw_text_pkmn(fb, ctx, pp.name(), 88, 60, dark);
            let plvl = format!("LV{}", pp.level);
            draw_text_pkmn(fb, ctx, &plvl, 132, 60, dark);
            let player_status = status_text(&pp.status);
            if !player_status.is_empty() {
                draw_text_pkmn(fb, ctx, player_status, 88, 68, status_color(&pp.status));
            }
            draw_hp_bar(fb, ctx, 102, 74, 48, battle.player_hp_display as u16, pp.max_hp);
            let hp_str = format!("{}/{}", pp.hp, pp.max_hp);
            draw_text_pkmn(fb, ctx, &hp_str, 104, 80, dark);
            // EXP bar
            let current_exp = pp.exp;
            let species = get_species(pp.species_id);
            let level_exp = species.map(|s| exp_for_level(pp.level, s.growth_rate)).unwrap_or(0);
            let next_exp = species.map(|s| exp_for_level(pp.level + 1, s.growth_rate)).unwrap_or(1);
            let exp_in_level = current_exp.saturating_sub(level_exp);
            let exp_needed = next_exp.saturating_sub(level_exp).max(1);
            draw_exp_bar(fb, ctx, 102, 88, 48, exp_in_level, exp_needed);
        }

        match &battle.phase {
            BattlePhase::Intro { timer } => {
                if *timer < 0.5 {
                    let a = (255.0 * (1.0 - timer * 2.0)) as u8;
                    fill_virtual_screen(fb, ctx, Color::from_rgba(248, 248, 248, a));
                }
                draw_text_box(fb, ctx, 2, 98, 156, 42);
                let msg = if battle.is_wild {
                    format!("Wild {} appeared!", battle.enemy.name())
                } else {
                    format!("Trainer sent out {}!", battle.enemy.name())
                };
                draw_text_pkmn(fb, ctx, &msg, 10, 106, dark);
            }

            BattlePhase::ActionSelect { cursor } => {
                draw_text_box(fb, ctx, 2, 98, 76, 42);
                draw_text_pkmn(fb, ctx, "What will", 10, 102, dark);
                if let Some(p) = self.party.get(battle.player_idx) {
                    let msg = format!("{} do?", p.name());
                    draw_text_pkmn(fb, ctx, &msg, 10, 114, dark);
                }

                draw_text_box(fb, ctx, 80, 98, 78, 42);
                let items = ["FIGHT", "BAG", "PKMN", "RUN"];
                let pos = [(86, 104), (126, 104), (86, 124), (126, 124)];
                for (i, (item, (x, y))) in items.iter().zip(pos.iter()).enumerate() {
                    draw_text_pkmn(fb, ctx, item, *x, *y, dark);
                    if i as u8 == *cursor { draw_cursor(fb, ctx, x - 8, *y, dark); }
                }
            }

            BattlePhase::MoveSelect { cursor } => {
                draw_text_box(fb, ctx, 2, 98, 156, 42);
                // Get enemy types for effectiveness display
                let enemy_sp = get_species(battle.enemy.species_id);
                let edt1 = enemy_sp.map(|s| s.type1).unwrap_or(PokemonType::Normal);
                let edt2 = enemy_sp.and_then(|s| s.type2);

                if let Some(pp) = self.party.get(battle.player_idx) {
                    for (i, ms) in pp.moves.iter().enumerate() {
                        if let Some(mid) = ms {
                            if let Some(md) = get_move(*mid) {
                                let x = if i % 2 == 0 { 14 } else { 86 };
                                let y = 104 + (i / 2) as i32 * 14;
                                let has_pp = pp.move_pp[i] > 0;
                                let name_color = if has_pp { type_color(md.move_type) } else { Color::from_rgba(160, 160, 168, 255) };
                                draw_text_pkmn(fb, ctx, md.name, x, y, name_color);
                                let pp_color = if has_pp { Color::from_rgba(120, 120, 140, 255) } else { Color::from_rgba(200, 80, 80, 255) };
                                let pp_s = format!("{}/{}", pp.move_pp[i], pp.move_max_pp[i]);
                                draw_text_pkmn(fb, ctx, &pp_s, x + 42, y + 8, pp_color);
                                if i as u8 == *cursor {
                                    draw_cursor(fb, ctx, x - 8, y, dark);
                                    // Show effectiveness of selected move
                                    if md.power > 0 {
                                        let eff = combined_effectiveness(md.move_type, edt1, edt2);
                                        let (eff_label, eff_color) = if eff > 1.5 {
                                            ("SE!", Color::from_rgba(80, 200, 80, 255))
                                        } else if eff < 0.5 && eff > 0.01 {
                                            ("NVE", Color::from_rgba(200, 120, 80, 255))
                                        } else if eff < 0.01 {
                                            ("X", Color::from_rgba(160, 80, 80, 255))
                                        } else {
                                            ("", Color::from_rgba(0, 0, 0, 0))
                                        };
                                        if !eff_label.is_empty() {
                                            draw_text_pkmn(fb, ctx, eff_label, 130, 136, eff_color);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            BattlePhase::Text { message, .. } => {
                draw_text_box(fb, ctx, 2, 98, 156, 42);
                for (i, line) in message.split('\n').enumerate() {
                    draw_text_pkmn(fb, ctx, line, 10, 106 + i as i32 * 12, dark);
                }
            }

            BattlePhase::PlayerAttack { move_id, .. } => {
                draw_text_box(fb, ctx, 2, 98, 156, 42);
                let name = self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default();
                let mn = get_move(*move_id).map(|m| m.name).unwrap_or("???");
                let msg = format!("{} used {}!", name, mn);
                draw_text_pkmn(fb, ctx, &msg, 10, 106, dark);
            }

            BattlePhase::EnemyAttack { move_id, .. } => {
                draw_text_box(fb, ctx, 2, 98, 156, 42);
                let mn = get_move(*move_id).map(|m| m.name).unwrap_or("???");
                let prefix = if battle.is_wild { "Wild " } else { "Foe " };
                let msg = format!("{}{} used {}!", prefix, battle.enemy.name(), mn);
                draw_text_pkmn(fb, ctx, &msg, 10, 106, dark);
            }

            BattlePhase::EnemyFainted { exp_gained: exp } => {
                draw_text_box(fb, ctx, 2, 98, 156, 42);
                let msg = format!("{} fainted!", battle.enemy.name());
                draw_text_pkmn(fb, ctx, &msg, 10, 106, dark);
                let exp_msg = format!("Gained {} EXP!", exp);
                draw_text_pkmn(fb, ctx, &exp_msg, 10, 118, dark);
            }

            BattlePhase::LevelUp { .. } => {
                draw_text_box(fb, ctx, 2, 98, 156, 42);
                if let Some(p) = self.party.get(battle.player_idx) {
                    let msg = format!("{} grew to LV{}!", p.name(), p.level);
                    draw_text_pkmn(fb, ctx, &msg, 10, 106, dark);
                }
            }

            BattlePhase::Won { .. } => {
                draw_text_box(fb, ctx, 2, 98, 156, 42);
                draw_text_pkmn(fb, ctx, "You won!", 10, 106, dark);
            }

            BattlePhase::RunFailed { .. } => {
                draw_text_box(fb, ctx, 2, 98, 156, 42);
                draw_text_pkmn(fb, ctx, "Can't escape!", 10, 106, dark);
            }

            BattlePhase::PlayerFainted => {
                draw_text_box(fb, ctx, 2, 98, 156, 42);
                if let Some(p) = self.party.get(battle.player_idx) {
                    let msg = format!("{} fainted!", p.name());
                    draw_text_pkmn(fb, ctx, &msg, 10, 106, dark);
                }
            }

            BattlePhase::Run => {} // handled in step
        }

    }

    fn render_dialogue(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let ctx = match &self.ctx { Some(c) => c, None => return };

        // Draw overworld behind
        self.render_overworld(fb);

        if let Some(dialogue) = &self.dialogue {
            draw_text_box(fb, ctx, 2, 98, 156, 42);
            if let Some(line) = dialogue.lines.get(dialogue.current_line) {
                let visible: String = line.chars().take(dialogue.char_index).collect();
                draw_text_pkmn(fb, ctx, &visible, 10, 106, Color::from_rgba(40, 40, 48, 255));
            }
            if let Some(line) = dialogue.lines.get(dialogue.current_line) {
                if dialogue.char_index >= line.len() && (self.frame_count / 20) % 2 == 0 {
                    draw_text_pkmn(fb, ctx, "V", 146, 132, Color::from_rgba(40, 40, 48, 255));
                }
            }
        }
    }

    fn render_menu(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        self.render_overworld(fb);

        draw_text_box(fb, ctx, 96, 2, 60, 82);
        let items = ["POKEMON", "BAG", "POKEDEX", "SAVE", "EXIT"];
        for (i, item) in items.iter().enumerate() {
            let y = 8 + i as i32 * 14;
            draw_text_pkmn(fb, ctx, item, 114, y, Color::from_rgba(40, 40, 48, 255));
            if i as u8 == self.menu_cursor {
                draw_cursor(fb, ctx, 104, y, Color::from_rgba(40, 40, 48, 255));
            }
        }

        draw_text_box(fb, ctx, 96, 86, 60, 18);
        draw_text_pkmn(fb, ctx, "GOLD", 108, 90, Color::from_rgba(40, 40, 48, 255));

        let money_str = format!("${}", self.money);
        draw_text_box(fb, ctx, 2, 120, 70, 18);
        draw_text_pkmn(fb, ctx, &money_str, 8, 124, Color::from_rgba(40, 40, 48, 255));
    }

    fn render_pokemon_menu(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, cursor: u8) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        let dark = Color::from_rgba(40, 40, 48, 255);

        fill_virtual_screen(fb, ctx, Color::from_rgba(248, 248, 248, 255));
        draw_text_pkmn(fb, ctx, "POKEMON", 55, 3, dark);
        fill_rect_v(fb, ctx, 4, 12, 152, 1, Color::from_rgba(168, 168, 176, 255));

        for (i, pkmn) in self.party.iter().enumerate() {
            let y = 16 + i as i32 * 20;
            if i as u8 == cursor {
                fill_rect_v(fb, ctx, 2, y - 1, 156, 18, Color::from_rgba(232, 240, 248, 255));
                draw_cursor(fb, ctx, 4, y + 4, dark);
            }
            let name_color = if pkmn.is_fainted() { Color::from_rgba(160, 80, 80, 255) } else { dark };
            draw_text_pkmn(fb, ctx, pkmn.name(), 14, y + 2, name_color);
            let lvl = format!("LV{}", pkmn.level);
            draw_text_pkmn(fb, ctx, &lvl, 80, y + 2, dark);
            draw_hp_bar(fb, ctx, 44, y + 12, 40, pkmn.hp, pkmn.max_hp);
            let hps = format!("{}/{}", pkmn.hp, pkmn.max_hp);
            draw_text_pkmn(fb, ctx, &hps, 100, y + 10, Color::from_rgba(80, 80, 96, 255));
            // Show fainted or status
            if pkmn.is_fainted() {
                draw_text_pkmn(fb, ctx, "FNT", 140, y + 2, Color::from_rgba(160, 80, 80, 255));
            } else {
                let st = status_text(&pkmn.status);
                if !st.is_empty() {
                    draw_text_pkmn(fb, ctx, st, 140, y + 2, status_color(&pkmn.status));
                }
            }
        }

        draw_text_pkmn(fb, ctx, "X/ESC TO CLOSE", 20, 133, Color::from_rgba(120, 120, 140, 255));
    }

    // ─── Bag Menu Logic ─────────────────────────────────

    fn step_bag_menu(&mut self, engine: &mut Engine) {
        let cursor = if let GamePhase::BagMenu { cursor } = &self.phase { *cursor } else { 0 };
        let item_count = self.bag.items.len() as u8;
        if item_count == 0 {
            self.dialogue = Some(DialogueState {
                lines: vec!["Bag is empty!".to_string()],
                current_line: 0, char_index: 0, timer: 0.0,
                on_complete: DialogueAction::None,
            });
            self.phase = GamePhase::Dialogue;
            return;
        }

        if is_down(engine) {
            self.phase = GamePhase::BagMenu { cursor: (cursor + 1) % item_count };
        } else if is_up(engine) {
            self.phase = GamePhase::BagMenu { cursor: if cursor == 0 { item_count - 1 } else { cursor - 1 } };
        }

        let confirm = is_confirm(engine);
        let cancel = is_cancel(engine);

        if cancel {
            if self.battle.is_some() {
                self.phase = GamePhase::Battle;
            } else {
                self.phase = GamePhase::Menu;
            }
            return;
        }

        if confirm {
            if let Some(&(item_id, _qty)) = self.bag.items.get(cursor as usize) {
                if let Some(item_data) = get_item(item_id) {
                    if item_data.is_ball {
                        // Poke Ball: use in battle
                        if self.battle.is_some() {
                            self.try_catch_pokemon(engine);
                        } else {
                            self.dialogue = Some(DialogueState {
                                lines: vec!["Can't use that here!".to_string()],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                        }
                    } else if item_data.heal_amount > 0 || item_data.is_revive || item_data.is_status_heal {
                        // Healing/revive/status heal: select target Pokemon
                        self.phase = GamePhase::BagUseItem { item_id, target_cursor: 0 };
                    } else if item_id == ITEM_REPEL {
                        // Repel: prevent wild encounters for 100 steps
                        if self.battle.is_some() {
                            self.dialogue = Some(DialogueState {
                                lines: vec!["Can't use that here!".to_string()],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                        } else {
                            self.bag.use_item(item_id);
                            self.repel_steps = 100;
                            self.dialogue = Some(DialogueState {
                                lines: vec!["Used a REPEL!".to_string(), "Wild Pokemon won't appear for a while.".to_string()],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                        }
                    } else if item_id == ITEM_ESCAPE_ROPE {
                        // Escape Rope: warp to last Pokemon Center
                        if self.battle.is_some() {
                            self.dialogue = Some(DialogueState {
                                lines: vec!["Can't use that in battle!".to_string()],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                        } else {
                            self.bag.use_item(item_id);
                            self.change_map(MapId::PokemonCenter, 5, 6);
                            self.dialogue = Some(DialogueState {
                                lines: vec!["Used an ESCAPE ROPE!".to_string()],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                        }
                    } else {
                        self.dialogue = Some(DialogueState {
                            lines: vec!["Can't use that now!".to_string()],
                            current_line: 0, char_index: 0, timer: 0.0,
                            on_complete: DialogueAction::None,
                        });
                        self.phase = GamePhase::Dialogue;
                    }
                }
            }
        }
    }

    fn step_bag_use_item(&mut self, engine: &mut Engine) {
        let (item_id, cursor) = if let GamePhase::BagUseItem { item_id, target_cursor } = &self.phase {
            (*item_id, *target_cursor)
        } else { return; };

        let party_size = self.party.len() as u8;
        if party_size == 0 { self.phase = GamePhase::BagMenu { cursor: 0 }; return; }

        if is_down(engine) {
            self.phase = GamePhase::BagUseItem { item_id, target_cursor: (cursor + 1) % party_size };
        } else if is_up(engine) {
            self.phase = GamePhase::BagUseItem { item_id, target_cursor: if cursor == 0 { party_size - 1 } else { cursor - 1 } };
        }

        let cancel = is_cancel(engine);
        let confirm = is_confirm(engine);

        if cancel {
            self.phase = GamePhase::BagMenu { cursor: 0 };
            return;
        }

        if confirm {
            if let Some(item_data) = get_item(item_id) {
                if let Some(pkmn) = self.party.get_mut(cursor as usize) {
                    let name = pkmn.name().to_string();
                    let item_name = item_data.name.to_string();

                    // Revive: only works on fainted Pokemon
                    if item_data.is_revive {
                        if !pkmn.is_fainted() {
                            self.dialogue = Some(DialogueState {
                                lines: vec![format!("{} isn't fainted!", name)],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                            return;
                        }
                        pkmn.hp = pkmn.max_hp / 2;
                        pkmn.clear_status();
                        self.bag.use_item(item_id);
                        let msg1 = format!("Used {} on {}!", item_name, name);
                        let msg2 = format!("{} was revived!", name);
                        if self.battle.is_some() {
                            let mut b = self.battle.take().unwrap();
                            let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(
                                engine, &b.enemy, b.player_idx, &b.enemy_stages, &b.player_stages,
                            );
                            b.phase = BattlePhase::Text {
                                message: msg1, timer: 0.0,
                                next_phase: Box::new(BattlePhase::Text {
                                    message: msg2, timer: 0.0,
                                    next_phase: Box::new(BattlePhase::EnemyAttack {
                                        timer: 0.0, move_id: e_move, damage: e_dmg,
                                        effectiveness: e_eff, is_crit: e_crit,
                                    }),
                                }),
                            };
                            self.battle = Some(b);
                            self.phase = GamePhase::Battle;
                        } else {
                            self.dialogue = Some(DialogueState {
                                lines: vec![msg1, msg2],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                        }
                        return;
                    }

                    // Status heal: cure status conditions
                    if item_data.is_status_heal {
                        if matches!(pkmn.status, StatusCondition::None) {
                            self.dialogue = Some(DialogueState {
                                lines: vec![format!("{} has no status problem!", name)],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                            return;
                        }
                        pkmn.clear_status();
                        self.bag.use_item(item_id);
                        let msg = format!("{} was cured!", name);
                        if self.battle.is_some() {
                            let mut b = self.battle.take().unwrap();
                            let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(
                                engine, &b.enemy, b.player_idx, &b.enemy_stages, &b.player_stages,
                            );
                            b.phase = BattlePhase::Text {
                                message: msg, timer: 0.0,
                                next_phase: Box::new(BattlePhase::EnemyAttack {
                                    timer: 0.0, move_id: e_move, damage: e_dmg,
                                    effectiveness: e_eff, is_crit: e_crit,
                                }),
                            };
                            self.battle = Some(b);
                            self.phase = GamePhase::Battle;
                        } else {
                            self.dialogue = Some(DialogueState {
                                lines: vec![msg],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                        }
                        return;
                    }

                    // HP healing item
                    if pkmn.hp >= pkmn.max_hp {
                        self.dialogue = Some(DialogueState {
                            lines: vec![format!("{} is already at full HP!", name)],
                            current_line: 0, char_index: 0, timer: 0.0,
                            on_complete: DialogueAction::None,
                        });
                        self.phase = GamePhase::Dialogue;
                        return;
                    }
                    if pkmn.is_fainted() {
                        self.dialogue = Some(DialogueState {
                            lines: vec![format!("{} has fainted!", name)],
                            current_line: 0, char_index: 0, timer: 0.0,
                            on_complete: DialogueAction::None,
                        });
                        self.phase = GamePhase::Dialogue;
                        return;
                    }
                    let old_hp = pkmn.hp;
                    pkmn.hp = (pkmn.hp + item_data.heal_amount).min(pkmn.max_hp);
                    let healed = pkmn.hp - old_hp;
                    self.bag.use_item(item_id);
                    let msg1 = format!("Used {} on {}!", item_name, name);
                    let msg2 = format!("Restored {} HP!", healed);
                    if self.battle.is_some() {
                        // In battle: use battle text system, enemy gets a turn
                        let mut b = self.battle.take().unwrap();
                        let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(
                            engine, &b.enemy, b.player_idx, &b.enemy_stages, &b.player_stages,
                        );
                        b.phase = BattlePhase::Text {
                            message: msg1, timer: 0.0,
                            next_phase: Box::new(BattlePhase::Text {
                                message: msg2, timer: 0.0,
                                next_phase: Box::new(BattlePhase::EnemyAttack {
                                    timer: 0.0, move_id: e_move, damage: e_dmg,
                                    effectiveness: e_eff, is_crit: e_crit,
                                }),
                            }),
                        };
                        self.battle = Some(b);
                        self.phase = GamePhase::Battle;
                    } else {
                        self.dialogue = Some(DialogueState {
                            lines: vec![msg1, msg2],
                            current_line: 0, char_index: 0, timer: 0.0,
                            on_complete: DialogueAction::None,
                        });
                        self.phase = GamePhase::Dialogue;
                    }
                }
            }
        }
    }

    fn try_catch_pokemon(&mut self, engine: &mut Engine) {
        let mut battle = match self.battle.take() {
            Some(b) => b,
            None => { self.phase = GamePhase::Overworld; return; }
        };

        if !battle.is_wild {
            self.dialogue = Some(DialogueState {
                lines: vec!["Can't catch a trainer's Pokemon!".to_string()],
                current_line: 0, char_index: 0, timer: 0.0,
                on_complete: DialogueAction::None,
            });
            self.battle = Some(battle);
            self.phase = GamePhase::Dialogue;
            return;
        }

        // Use the selected ball from bag
        let (used_ball, ball_mult) = if let Some(&(item_id, _)) = self.bag.items.iter().find(|(id, _)| get_item(*id).map(|i| i.is_ball).unwrap_or(false)) {
            let mult = if item_id == ITEM_GREAT_BALL { 1.5 } else { 1.0 };
            if !self.bag.use_item(item_id) {
                self.battle = Some(battle);
                self.phase = GamePhase::Battle;
                return;
            }
            (item_id, mult)
        } else {
            self.battle = Some(battle);
            self.phase = GamePhase::Battle;
            return;
        };

        let ball_name = if used_ball == ITEM_GREAT_BALL { "GREAT BALL" } else { "POKE BALL" };

        // Gen 2 catch formula (simplified)
        let max_hp = battle.enemy.max_hp as f64;
        let cur_hp = battle.enemy.hp as f64;
        let catch_rate = get_species(battle.enemy.species_id)
            .map(|s| s.catch_rate as f64)
            .unwrap_or(128.0);
        let rate = ((3.0 * max_hp - 2.0 * cur_hp) * catch_rate * ball_mult) / (3.0 * max_hp);
        let shake_prob = rate / 255.0;

        let r = engine.rng.next_f64();
        let caught = r < shake_prob;

        if caught {
            sfx_catch(engine);
            self.register_caught(battle.enemy.species_id);
            let enemy_name = battle.enemy.name().to_string();
            if self.party.len() < 6 {
                self.party.push(battle.enemy.clone());
                self.dialogue = Some(DialogueState {
                    lines: vec![
                        format!("You threw a {}!", ball_name),
                        format!("Gotcha! {} was caught!", enemy_name),
                    ],
                    current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
            } else {
                self.pc_boxes.push(battle.enemy.clone());
                self.dialogue = Some(DialogueState {
                    lines: vec![
                        format!("Gotcha! {} was caught!", enemy_name),
                        "Your party is full!".to_string(),
                        format!("{} was sent to the PC!", enemy_name),
                    ],
                    current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
            }
            engine.global_state.set_f64("in_battle", 0.0);
            self.battle = None;
            self.phase = GamePhase::Dialogue;
        } else {
            // Failed catch - enemy gets a turn
            let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
            battle.phase = BattlePhase::Text {
                message: format!("You threw a {}!\nOh no! It broke free!", ball_name),
                timer: 0.0,
                next_phase: Box::new(BattlePhase::EnemyAttack {
                    timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                }),
            };
            self.battle = Some(battle);
            self.phase = GamePhase::Battle;
        }
    }

    // ─── Bag Rendering ────────────────────────────────

    fn render_bag_menu(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, cursor: u8) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        let dark = Color::from_rgba(40, 40, 48, 255);

        fill_virtual_screen(fb, ctx, Color::from_rgba(248, 248, 248, 255));
        draw_text_pkmn(fb, ctx, "BAG", 65, 3, dark);
        fill_rect_v(fb, ctx, 4, 12, 152, 1, Color::from_rgba(168, 168, 176, 255));

        if self.bag.is_empty() {
            draw_text_pkmn(fb, ctx, "Empty!", 55, 60, Color::from_rgba(120, 120, 140, 255));
        } else {
            for (i, &(item_id, qty)) in self.bag.items.iter().enumerate() {
                let y = 16 + i as i32 * 18;
                if y > 128 { break; }
                if i as u8 == cursor {
                    fill_rect_v(fb, ctx, 2, y - 1, 156, 16, Color::from_rgba(232, 240, 248, 255));
                    draw_cursor(fb, ctx, 4, y + 2, dark);
                }
                let name = get_item(item_id).map(|i| i.name).unwrap_or("???");
                draw_text_pkmn(fb, ctx, name, 14, y + 2, dark);
                let qty_str = format!("x{}", qty);
                draw_text_pkmn(fb, ctx, &qty_str, 120, y + 2, Color::from_rgba(80, 80, 96, 255));
            }
        }

        draw_text_pkmn(fb, ctx, "X/ESC TO CLOSE", 20, 133, Color::from_rgba(120, 120, 140, 255));
    }

    fn render_bag_use_item(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, _item_id: u8, target_cursor: u8) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        let dark = Color::from_rgba(40, 40, 48, 255);

        fill_virtual_screen(fb, ctx, Color::from_rgba(248, 248, 248, 255));
        draw_text_pkmn(fb, ctx, "USE ON WHO?", 30, 3, dark);
        fill_rect_v(fb, ctx, 4, 12, 152, 1, Color::from_rgba(168, 168, 176, 255));

        for (i, pkmn) in self.party.iter().enumerate() {
            let y = 16 + i as i32 * 20;
            if i as u8 == target_cursor {
                fill_rect_v(fb, ctx, 2, y - 1, 156, 18, Color::from_rgba(232, 240, 248, 255));
                draw_cursor(fb, ctx, 4, y + 4, dark);
            }
            draw_text_pkmn(fb, ctx, pkmn.name(), 14, y + 2, dark);
            draw_hp_bar(fb, ctx, 44, y + 12, 40, pkmn.hp, pkmn.max_hp);
            let hps = format!("{}/{}", pkmn.hp, pkmn.max_hp);
            draw_text_pkmn(fb, ctx, &hps, 100, y + 10, Color::from_rgba(80, 80, 96, 255));
        }

        draw_text_pkmn(fb, ctx, "X/ESC TO CANCEL", 15, 133, Color::from_rgba(120, 120, 140, 255));
    }

    fn render_pc_menu(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, mode: u8, cursor: u8) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        let dark = Color::from_rgba(40, 40, 48, 255);
        let dim = Color::from_rgba(80, 80, 96, 255);

        fill_virtual_screen(fb, ctx, Color::from_rgba(200, 220, 248, 255));
        draw_text_pkmn(fb, ctx, "BILL'S PC", 42, 3, dark);
        fill_rect_v(fb, ctx, 4, 12, 152, 1, Color::from_rgba(168, 168, 176, 255));

        if mode == 0 {
            // Mode select
            let options = ["WITHDRAW", "DEPOSIT", "CLOSE"];
            for (i, opt) in options.iter().enumerate() {
                let y = 30 + i as i32 * 20;
                if i as u8 == cursor {
                    fill_rect_v(fb, ctx, 20, y - 2, 120, 18, Color::from_rgba(248, 248, 255, 255));
                    draw_cursor(fb, ctx, 22, y + 2, dark);
                }
                draw_text_pkmn(fb, ctx, opt, 34, y + 2, dark);
            }
            let pc_str = format!("PC: {} stored", self.pc_boxes.len());
            draw_text_pkmn(fb, ctx, &pc_str, 30, 100, dim);
            let party_str = format!("PARTY: {}/6", self.party.len());
            draw_text_pkmn(fb, ctx, &party_str, 30, 112, dim);
        } else if mode == 1 {
            // Withdraw
            draw_text_pkmn(fb, ctx, "WITHDRAW", 50, 14, dark);
            if self.pc_boxes.is_empty() {
                draw_text_pkmn(fb, ctx, "No Pokemon stored!", 10, 40, dim);
            } else {
                for (i, pkmn) in self.pc_boxes.iter().enumerate() {
                    let y = 26 + i as i32 * 18;
                    if y > 110 { break; }
                    if i as u8 == cursor {
                        fill_rect_v(fb, ctx, 2, y - 1, 156, 16, Color::from_rgba(248, 248, 255, 255));
                        draw_cursor(fb, ctx, 4, y + 2, dark);
                    }
                    draw_text_pkmn(fb, ctx, pkmn.name(), 14, y + 2, dark);
                    let lvl = format!("LV{}", pkmn.level);
                    draw_text_pkmn(fb, ctx, &lvl, 100, y + 2, dim);
                    draw_hp_bar(fb, ctx, 120, y + 8, 30, pkmn.hp, pkmn.max_hp);
                }
            }
            let back_y = 26 + self.pc_boxes.len().min(5) as i32 * 18;
            if cursor == self.pc_boxes.len() as u8 {
                draw_cursor(fb, ctx, 4, back_y + 2, dark);
            }
            draw_text_pkmn(fb, ctx, "BACK", 14, back_y + 2, dark);
            let party_str = format!("PARTY: {}/6", self.party.len());
            draw_text_pkmn(fb, ctx, &party_str, 10, 133, dim);
        } else {
            // Deposit
            draw_text_pkmn(fb, ctx, "DEPOSIT", 52, 14, dark);
            for (i, pkmn) in self.party.iter().enumerate() {
                let y = 26 + i as i32 * 18;
                if i as u8 == cursor {
                    fill_rect_v(fb, ctx, 2, y - 1, 156, 16, Color::from_rgba(248, 248, 255, 255));
                    draw_cursor(fb, ctx, 4, y + 2, dark);
                }
                let name_color = if pkmn.is_fainted() { Color::from_rgba(160, 80, 80, 255) } else { dark };
                draw_text_pkmn(fb, ctx, pkmn.name(), 14, y + 2, name_color);
                let lvl = format!("LV{}", pkmn.level);
                draw_text_pkmn(fb, ctx, &lvl, 100, y + 2, dim);
                draw_hp_bar(fb, ctx, 120, y + 8, 30, pkmn.hp, pkmn.max_hp);
            }
            let back_y = 26 + self.party.len().min(6) as i32 * 18;
            if cursor == self.party.len() as u8 {
                draw_cursor(fb, ctx, 4, back_y + 2, dark);
            }
            draw_text_pkmn(fb, ctx, "BACK", 14, back_y + 2, dark);
            let pc_str = format!("PC: {} stored", self.pc_boxes.len());
            draw_text_pkmn(fb, ctx, &pc_str, 10, 133, dim);
        }
    }

    fn render_pokedex(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, cursor: u8, scroll: u8) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        let dark = Color::from_rgba(40, 40, 48, 255);
        let dim = Color::from_rgba(80, 80, 96, 255);

        fill_virtual_screen(fb, ctx, Color::from_rgba(248, 248, 248, 255));
        let header = format!("POKEDEX  {}/{}", self.pokedex_caught.len(), self.pokedex_seen.len());
        draw_text_pkmn(fb, ctx, &header, 20, 3, dark);
        fill_rect_v(fb, ctx, 4, 12, 152, 1, Color::from_rgba(168, 168, 176, 255));

        if self.pokedex_seen.is_empty() {
            draw_text_pkmn(fb, ctx, "No Pokemon seen!", 20, 60, dim);
        } else {
            let visible_count = 6u8.min(self.pokedex_seen.len() as u8);
            for i in 0..visible_count {
                let idx = (scroll + i) as usize;
                if idx >= self.pokedex_seen.len() { break; }
                let species_id = self.pokedex_seen[idx];
                let y = 16 + i as i32 * 18;

                if scroll + i == cursor {
                    fill_rect_v(fb, ctx, 2, y - 1, 156, 16, Color::from_rgba(232, 240, 248, 255));
                    draw_cursor(fb, ctx, 4, y + 2, dark);
                }

                let caught = self.pokedex_caught.contains(&species_id);
                let marker = if caught { "o" } else { "-" };
                draw_text_pkmn(fb, ctx, marker, 14, y + 2, dim);

                if let Some(sp) = get_species(species_id) {
                    let num = format!("#{:03}", sp.id);
                    draw_text_pkmn(fb, ctx, &num, 22, y + 2, dim);
                    draw_text_pkmn(fb, ctx, sp.name, 58, y + 2, dark);
                    let type_str = format!("{:?}", sp.type1);
                    draw_text_pkmn(fb, ctx, &type_str.to_uppercase(), 120, y + 2, type_color(sp.type1));
                }
            }
        }

        draw_text_pkmn(fb, ctx, "X/ESC TO CLOSE", 20, 133, Color::from_rgba(120, 120, 140, 255));
    }

    fn render_pokemart(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, cursor: u8) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        let dark = Color::from_rgba(40, 40, 48, 255);

        fill_virtual_screen(fb, ctx, Color::from_rgba(248, 248, 248, 255));
        draw_text_pkmn(fb, ctx, "POKE MART", 42, 3, dark);
        fill_rect_v(fb, ctx, 4, 12, 152, 1, Color::from_rgba(168, 168, 176, 255));

        for (i, &(item_id, price)) in MART_INVENTORY.iter().enumerate() {
            let y = 16 + i as i32 * 18;
            if i as u8 == cursor {
                fill_rect_v(fb, ctx, 2, y - 1, 156, 16, Color::from_rgba(232, 240, 248, 255));
                draw_cursor(fb, ctx, 4, y + 2, dark);
            }
            let name = get_item(item_id).map(|i| i.name).unwrap_or("???");
            draw_text_pkmn(fb, ctx, name, 14, y + 2, dark);
            let price_str = format!("${}", price);
            draw_text_pkmn(fb, ctx, &price_str, 110, y + 2, Color::from_rgba(80, 80, 96, 255));
        }

        // Money display
        let money_str = format!("MONEY: ${}", self.money);
        draw_text_box(fb, ctx, 2, 110, 90, 18);
        draw_text_pkmn(fb, ctx, &money_str, 8, 114, dark);

        draw_text_pkmn(fb, ctx, "X/ESC TO LEAVE", 20, 133, Color::from_rgba(120, 120, 140, 255));
    }

    fn render_pokemon_summary(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, index: u8) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        let dark = Color::from_rgba(40, 40, 48, 255);
        let dim = Color::from_rgba(80, 80, 96, 255);

        fill_virtual_screen(fb, ctx, Color::from_rgba(248, 248, 248, 255));

        if let Some(pkmn) = self.party.get(index as usize) {
            // Header
            draw_text_pkmn(fb, ctx, pkmn.name(), 8, 4, dark);
            let lvl = format!("LV{}", pkmn.level);
            draw_text_pkmn(fb, ctx, &lvl, 120, 4, dark);
            fill_rect_v(fb, ctx, 4, 14, 152, 1, Color::from_rgba(168, 168, 176, 255));

            // Type
            if let Some(sp) = get_species(pkmn.species_id) {
                let type_str = format!("{:?}", sp.type1);
                draw_text_pkmn(fb, ctx, &type_str.to_uppercase(), 8, 18, type_color(sp.type1));
                if let Some(t2) = sp.type2 {
                    let t2_str = format!("{:?}", t2);
                    draw_text_pkmn(fb, ctx, &t2_str.to_uppercase(), 75, 18, type_color(t2));
                }
            }

            // HP
            let hp_str = format!("HP: {}/{}", pkmn.hp, pkmn.max_hp);
            draw_text_pkmn(fb, ctx, &hp_str, 8, 30, dark);
            draw_hp_bar(fb, ctx, 55, 38, 80, pkmn.hp, pkmn.max_hp);

            // Stats
            let stats = [
                format!("ATK: {}", pkmn.attack),
                format!("DEF: {}", pkmn.defense),
                format!("SATK:{}", pkmn.sp_attack),
                format!("SDEF:{}", pkmn.sp_defense),
                format!("SPD: {}", pkmn.speed),
            ];
            for (i, stat) in stats.iter().enumerate() {
                let x = if i < 3 { 8 } else { 80 };
                let y = 44 + (i % 3) as i32 * 12;
                draw_text_pkmn(fb, ctx, stat, x, y, dim);
            }

            // EXP
            let species = get_species(pkmn.species_id);
            let next_exp = species.map(|s| exp_for_level(pkmn.level + 1, s.growth_rate)).unwrap_or(1);
            let to_next = next_exp.saturating_sub(pkmn.exp);
            let exp_str = format!("TO NEXT LV: {}", to_next);
            draw_text_pkmn(fb, ctx, &exp_str, 8, 80, dim);

            // Moves
            fill_rect_v(fb, ctx, 4, 90, 152, 1, Color::from_rgba(168, 168, 176, 255));
            draw_text_pkmn(fb, ctx, "MOVES", 55, 93, dark);
            for (i, ms) in pkmn.moves.iter().enumerate() {
                if let Some(mid) = ms {
                    if let Some(md) = get_move(*mid) {
                        let y = 105 + i as i32 * 10;
                        draw_text_pkmn(fb, ctx, md.name, 8, y, dark);
                        let pp_str = format!("{}/{}", pkmn.move_pp[i], pkmn.move_max_pp[i]);
                        draw_text_pkmn(fb, ctx, &pp_str, 110, y, dim);
                    }
                }
            }
        }

        draw_text_pkmn(fb, ctx, "PRESS ANY KEY", 22, 140, Color::from_rgba(120, 120, 140, 255));
    }

    fn render_healing(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        self.render_overworld(fb);

        draw_text_box(fb, ctx, 10, 50, 140, 40);
        draw_text_pkmn(fb, ctx, "Your POKEMON were", 18, 58, Color::from_rgba(40, 40, 48, 255));
        draw_text_pkmn(fb, ctx, "healed to full health!", 18, 70, Color::from_rgba(40, 40, 48, 255));
    }

    fn render_encounter_transition(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, timer: f64) {
        let ctx = match &self.ctx { Some(c) => c, None => return };

        // Draw overworld as base
        self.render_overworld(fb);

        // Classic Pokemon encounter flash: alternating black/white bands
        let flash_phase = (timer * 12.0) as u32;
        if flash_phase % 2 == 0 {
            // Black flash
            fill_virtual_screen(fb, ctx, Color::from_rgba(8, 8, 16, 255));
        } else {
            // White flash
            fill_virtual_screen(fb, ctx, Color::from_rgba(248, 248, 248, 255));
        }

        // During last portion, slide to black
        if timer > 0.6 {
            let fade = ((timer - 0.6) / 0.2).min(1.0);
            let a = (fade * 255.0) as u8;
            fill_virtual_screen(fb, ctx, Color::from_rgba(8, 8, 16, a));
        }
    }

    fn render_evolution(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, timer: f64, new_species: SpeciesId) {
        let ctx = match &self.ctx { Some(c) => c, None => return };

        fill_virtual_screen(fb, ctx, Color::from_rgba(8, 8, 24, 255));

        // Pulsing light effect
        let pulse = ((timer * 3.0).sin() * 0.5 + 0.5) as f64;
        let glow_a = (pulse * 80.0) as u8;
        fill_rect_v(fb, ctx, 40, 20, 80, 80, Color::from_rgba(248, 248, 200, glow_a));

        // Text
        if let Some(p) = self.party.first() {
            let old_name = p.name().to_string();
            let new_name = get_species(new_species).map(|s| s.name).unwrap_or("???");

            if timer < 1.5 {
                draw_text_pkmn(fb, ctx, "What?", 60, 30, Color::from_rgba(248, 248, 248, 255));
                let msg = format!("{} is", old_name);
                draw_text_pkmn(fb, ctx, &msg, 30, 50, Color::from_rgba(248, 248, 248, 255));
                draw_text_pkmn(fb, ctx, "evolving!", 45, 62, Color::from_rgba(248, 248, 248, 255));
            } else {
                let msg = format!("{} evolved", old_name);
                draw_text_pkmn(fb, ctx, &msg, 15, 40, Color::from_rgba(248, 248, 248, 255));
                let msg2 = format!("into {}!", new_name);
                draw_text_pkmn(fb, ctx, &msg2, 25, 56, Color::from_rgba(248, 208, 48, 255));
            }
        }

        draw_text_pkmn(fb, ctx, "PRESS Z", 50, 130, Color::from_rgba(120, 120, 140, 255));
    }
}

fn status_text(s: &StatusCondition) -> &'static str {
    match s {
        StatusCondition::None => "",
        StatusCondition::Poison => "PSN",
        StatusCondition::Burn => "BRN",
        StatusCondition::Paralysis => "PAR",
        StatusCondition::Sleep { .. } => "SLP",
        StatusCondition::Freeze => "FRZ",
    }
}

fn status_color(s: &StatusCondition) -> Color {
    match s {
        StatusCondition::Poison => Color::from_rgba(160, 64, 160, 255),
        StatusCondition::Burn => Color::from_rgba(240, 128, 48, 255),
        StatusCondition::Paralysis => Color::from_rgba(248, 208, 48, 255),
        StatusCondition::Sleep { .. } => Color::from_rgba(120, 120, 160, 255),
        StatusCondition::Freeze => Color::from_rgba(152, 216, 216, 255),
        StatusCondition::None => Color::from_rgba(0, 0, 0, 0),
    }
}

fn type_color(t: PokemonType) -> Color {
    match t {
        PokemonType::Normal => Color::from_rgba(168, 168, 120, 255),
        PokemonType::Fire => Color::from_rgba(240, 128, 48, 255),
        PokemonType::Water => Color::from_rgba(104, 144, 240, 255),
        PokemonType::Grass => Color::from_rgba(120, 200, 80, 255),
        PokemonType::Electric => Color::from_rgba(248, 208, 48, 255),
        PokemonType::Ice => Color::from_rgba(152, 216, 216, 255),
        PokemonType::Fighting => Color::from_rgba(192, 48, 40, 255),
        PokemonType::Poison => Color::from_rgba(160, 64, 160, 255),
        PokemonType::Ground => Color::from_rgba(224, 192, 104, 255),
        PokemonType::Flying => Color::from_rgba(168, 144, 240, 255),
        PokemonType::Psychic => Color::from_rgba(248, 88, 136, 255),
        PokemonType::Bug => Color::from_rgba(168, 184, 32, 255),
        PokemonType::Rock => Color::from_rgba(184, 160, 56, 255),
        PokemonType::Ghost => Color::from_rgba(112, 88, 152, 255),
        PokemonType::Dragon => Color::from_rgba(112, 56, 248, 255),
        PokemonType::Dark => Color::from_rgba(112, 88, 72, 255),
        PokemonType::Steel => Color::from_rgba(184, 184, 208, 255),
    }
}

// ─── SFX helpers ────────────────────────────────────────

fn sfx_hit(engine: &mut Engine, super_effective: bool) {
    let freq = if super_effective { 800.0 } else { 500.0 };
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: freq, duration: 0.08, volume: 0.3,
        waveform: Waveform::Square, attack: 0.005, decay: 0.06,
    });
}

fn sfx_faint(engine: &mut Engine) {
    for (i, f) in [600.0, 400.0, 200.0].iter().enumerate() {
        engine.sound_queue.push(SoundCommand::PlayTone {
            frequency: *f, duration: 0.15, volume: 0.25,
            waveform: Waveform::Square, attack: 0.01, decay: 0.1 + i as f64 * 0.05,
        });
    }
}

fn sfx_level_up(engine: &mut Engine) {
    for (i, f) in [523.0, 659.0, 784.0, 1047.0].iter().enumerate() {
        engine.sound_queue.push(SoundCommand::PlayTone {
            frequency: *f, duration: 0.12, volume: 0.2,
            waveform: Waveform::Square, attack: 0.01, decay: 0.08 + i as f64 * 0.02,
        });
    }
}

fn sfx_catch(engine: &mut Engine) {
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 880.0, duration: 0.15, volume: 0.25,
        waveform: Waveform::Square, attack: 0.01, decay: 0.12,
    });
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 1100.0, duration: 0.2, volume: 0.3,
        waveform: Waveform::Square, attack: 0.05, decay: 0.15,
    });
}

fn sfx_heal(engine: &mut Engine) {
    for (i, f) in [440.0, 554.0, 659.0, 880.0].iter().enumerate() {
        engine.sound_queue.push(SoundCommand::PlayTone {
            frequency: *f, duration: 0.1, volume: 0.15,
            waveform: Waveform::Sine, attack: 0.02, decay: 0.08 + i as f64 * 0.02,
        });
    }
}

fn sfx_select(engine: &mut Engine) {
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 700.0, duration: 0.05, volume: 0.15,
        waveform: Waveform::Square, attack: 0.005, decay: 0.04,
    });
}

fn try_inflict_status(target: &mut Pokemon, move_id: MoveId, rng_roll: f64) {
    if !matches!(target.status, StatusCondition::None) { return; }
    match move_id {
        MOVE_POISON_STING => { if rng_roll < 0.3 { target.status = StatusCondition::Poison; } }
        MOVE_EMBER => { if rng_roll < 0.1 { target.status = StatusCondition::Burn; } }
        MOVE_THUNDER_SHOCK => { if rng_roll < 0.1 { target.status = StatusCondition::Paralysis; } }
        MOVE_LICK => { if rng_roll < 0.3 { target.status = StatusCondition::Paralysis; } }
        MOVE_HYPNOSIS => { target.status = StatusCondition::Sleep { turns: 3 }; }
        _ => {}
    }
}

fn eff_text(e: f64) -> &'static str {
    if e > 1.5 { "Super effective!" }
    else if e < 0.5 && e > 0.01 { "Not very effective..." }
    else if e < 0.01 { "No effect!" }
    else { "" }
}

// ─── Simulation Trait ───────────────────────────────────

impl Simulation for PokemonSim {
    fn setup(&mut self, engine: &mut Engine) {
        engine.config.background = Color::from_rgba(8, 8, 16, 255);
        self.ctx = Some(RenderContext::new(engine.framebuffer.width, engine.framebuffer.height));
        self.init_sprite_caches();
        engine.global_state.set_f64("in_battle", 0.0);
        engine.global_state.set_str("game_phase", "title");
        engine.global_state.set_f64("pending_evolution", 0.0);
    }

    fn step(&mut self, engine: &mut Engine) {
        self.frame_count += 1;

        match self.phase.clone() {
            GamePhase::TitleScreen => {
                self.title_blink_timer += 1.0 / 60.0;
                let start = is_confirm(engine);
                if start {
                    engine.global_state.set_str("game_phase", "overworld");
                    if !self.has_starter {
                        self.change_map(MapId::ElmLab, 5, 8);
                    }
                    self.phase = GamePhase::Overworld;
                }
            }

            GamePhase::StarterSelect { cursor } => {
                if is_down(engine) {
                    self.phase = GamePhase::StarterSelect { cursor: (cursor + 1) % 3 };
                } else if is_up(engine) {
                    self.phase = GamePhase::StarterSelect { cursor: if cursor == 0 { 2 } else { cursor - 1 } };
                }

                if is_confirm(engine) {
                    let species = match cursor { 0 => CHIKORITA, 1 => CYNDAQUIL, 2 => TOTODILE, _ => CHIKORITA };
                    // Rival picks type-advantaged starter (GSC logic)
                    self.rival_starter = match species {
                        CHIKORITA => CYNDAQUIL,   // Fire beats Grass
                        CYNDAQUIL => TOTODILE,    // Water beats Fire
                        TOTODILE => CHIKORITA,    // Grass beats Water
                        _ => CYNDAQUIL,
                    };
                    let starter = Pokemon::new(species, 5);
                    let name = get_species(species).map(|s| s.name).unwrap_or("???").to_string();
                    self.party.push(starter);
                    self.has_starter = true;
                    self.register_caught(species);
                    // Give starter items
                    self.bag.add_item(ITEM_POTION, 5);
                    self.bag.add_item(ITEM_POKE_BALL, 5);
                    self.dialogue = Some(DialogueState {
                        lines: vec![
                            format!("You received {}!", name),
                            "Take good care of it!".to_string(),
                            "You received 5 POTIONs!".to_string(),
                            "You received 5 POKE BALLs!".to_string(),
                        ],
                        current_line: 0, char_index: 0, timer: 0.0, on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                }
            }

            GamePhase::Overworld => {
                engine.global_state.set_str("game_phase", "overworld");
                self.step_overworld(engine);
            }

            GamePhase::Battle => {
                engine.global_state.set_str("game_phase", "battle");
                self.step_battle(engine);
            }

            GamePhase::Dialogue => self.step_dialogue(engine),
            GamePhase::Menu => self.step_menu(engine),
            GamePhase::PokemonMenu { .. } => self.step_pokemon_menu(engine),
            GamePhase::BagMenu { .. } => self.step_bag_menu(engine),
            GamePhase::BagUseItem { .. } => self.step_bag_use_item(engine),
            GamePhase::PokeMart { .. } => self.step_pokemart(engine),
            GamePhase::PokemonSummary { .. } => self.step_pokemon_summary(engine),
            GamePhase::Pokedex { .. } => self.step_pokedex(engine),
            GamePhase::PCMenu { .. } => self.step_pc_menu(engine),

            GamePhase::Healing { timer } => {
                let t = timer + 1.0 / 60.0;
                if t > 2.0 || is_confirm(engine) {
                    self.phase = GamePhase::Overworld;
                } else {
                    self.phase = GamePhase::Healing { timer: t };
                }
            }

            GamePhase::EncounterTransition { timer } => {
                let dt = 1.0 / 60.0;
                let t = timer + dt;
                // Flash effect: rapidly alternate black and white
                let flash_cycle = (t * 12.0) as u32; // 12 Hz flashing
                if flash_cycle % 2 == 0 {
                    self.screen_flash = 1.0;
                } else {
                    self.screen_flash = 0.0;
                }
                self.encounter_flash_count = flash_cycle as u8;

                if t > 0.8 {
                    self.screen_flash = 0.0;
                    engine.global_state.set_str("game_phase", "battle");
                    self.phase = GamePhase::Battle;
                } else {
                    self.phase = GamePhase::EncounterTransition { timer: t };
                }
            }

            GamePhase::Evolution { timer, new_species } => {
                let dt = 1.0 / 60.0;
                let t = timer + dt;
                // Flash during evolution
                if t < 2.0 {
                    let flash_cycle = (t * 6.0) as u32;
                    self.screen_flash = if flash_cycle % 2 == 0 { 0.8 } else { 0.0 };
                } else {
                    self.screen_flash = 0.0;
                }

                if t > 3.0 || (t > 1.5 && is_confirm(engine)) {
                    // Apply evolution to the Pokemon that triggered it
                    // Find the party member with the pre-evolution species
                    let evo_idx = self.party.iter().position(|p| {
                        get_species(p.species_id).and_then(|s| s.evolution_into).map(|e| e == new_species).unwrap_or(false)
                    }).unwrap_or(0);
                    if let Some(p) = self.party.get_mut(evo_idx) {
                        p.species_id = new_species;
                        p.recalc_stats();
                        self.register_caught(new_species);
                    }
                    self.screen_flash = 0.0;
                    self.phase = GamePhase::Overworld;
                } else {
                    self.phase = GamePhase::Evolution { timer: t, new_species };
                }
            }
        }

        // Decay screen effects
        let dt = 1.0 / 60.0;
        let in_transition = matches!(self.phase, GamePhase::EncounterTransition { .. } | GamePhase::Evolution { .. });
        if self.screen_flash > 0.0 && !in_transition {
            self.screen_flash = (self.screen_flash - dt * 4.0).max(0.0);
        }
        if self.screen_shake > 0.0 {
            self.screen_shake = (self.screen_shake - dt * 12.0).max(0.0);
            // Simple shake offset using frame count
            let shake_t = self.frame_count as f64 * 0.7;
            self.screen_shake_x = shake_t.sin() * self.screen_shake;
            self.screen_shake_y = shake_t.cos() * self.screen_shake * 0.5;
        } else {
            self.screen_shake_x = 0.0;
            self.screen_shake_y = 0.0;
        }
    }

    fn render(&self, engine: &mut Engine) {
        if self.ctx.is_none() { return; }

        let fb = &mut engine.framebuffer;
        match &self.phase {
            GamePhase::TitleScreen => self.render_title(fb),
            GamePhase::StarterSelect { cursor } => self.render_starter_select(fb, *cursor),
            GamePhase::Overworld => self.render_overworld(fb),
            GamePhase::EncounterTransition { timer } => self.render_encounter_transition(fb, *timer),
            GamePhase::Battle => self.render_battle(fb),
            GamePhase::Dialogue => self.render_dialogue(fb),
            GamePhase::Menu => self.render_menu(fb),
            GamePhase::PokemonMenu { cursor } => self.render_pokemon_menu(fb, *cursor),
            GamePhase::BagMenu { cursor } => self.render_bag_menu(fb, *cursor),
            GamePhase::BagUseItem { item_id, target_cursor } => self.render_bag_use_item(fb, *item_id, *target_cursor),
            GamePhase::Healing { .. } => self.render_healing(fb),
            GamePhase::Evolution { timer, new_species } => self.render_evolution(fb, *timer, *new_species),
            GamePhase::PokeMart { cursor } => self.render_pokemart(fb, *cursor),
            GamePhase::PokemonSummary { index } => self.render_pokemon_summary(fb, *index),
            GamePhase::Pokedex { cursor, scroll } => self.render_pokedex(fb, *cursor, *scroll),
            GamePhase::PCMenu { mode, cursor } => self.render_pc_menu(fb, *mode, *cursor),
        }

        // Screen flash overlay (white flash for attacks)
        if self.screen_flash > 0.01 {
            if let Some(ctx) = &self.ctx {
                let a = (self.screen_flash * 255.0).min(255.0) as u8;
                fill_virtual_screen(fb, ctx, Color::from_rgba(255, 255, 255, a));
            }
        }

        // Export sprite position data for JS battle overlay
        if matches!(self.phase, GamePhase::Battle) {
            if let (Some(battle), Some(ctx)) = (&self.battle, &self.ctx) {
                let s = ctx.scale as f64;
                // Apply screen shake to sprite positions
                let shake_x = self.screen_shake_x as i32;
                let shake_y = self.screen_shake_y as i32;

                let (ex, ey) = ctx.to_fb(96, 6);
                engine.global_state.set_f64("enemy_sprite_x", (ex + shake_x) as f64);
                engine.global_state.set_f64("enemy_sprite_y", (ey + shake_y) as f64);
                engine.global_state.set_f64("enemy_sprite_size", 40.0 * s);

                let (px, py) = ctx.to_fb(10, 48);
                engine.global_state.set_f64("player_sprite_x", (px + shake_x) as f64);
                engine.global_state.set_f64("player_sprite_y", (py + shake_y) as f64);
                engine.global_state.set_f64("player_sprite_size", 40.0 * s);
                engine.global_state.set_f64("render_scale", s);

                if let Some(sp) = get_species(battle.enemy.species_id) {
                    engine.global_state.set_str("enemy_pokemon", &sp.name.to_lowercase());
                }
                if let Some(pp) = self.party.get(battle.player_idx) {
                    if let Some(sp) = get_species(pp.species_id) {
                        engine.global_state.set_str("player_pokemon", &sp.name.to_lowercase());
                    }
                }
            }
        }
    }
}
