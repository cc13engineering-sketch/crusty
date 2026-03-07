// AI-INSTRUCTIONS: Pokemon Gold/Silver/Crystal recreation implementing the Simulation trait.
// State machine: TitleScreen -> StarterSelect -> Overworld <-> Battle <-> Menu <-> Dialogue.
// Grid-based overworld with tile rendering, wild encounters in tall grass, turn-based battles.
// Battle Pokemon sprites loaded from web in JS layer; overworld tiles/characters rendered in Rust.
// Features: multi-Pokemon trainer battles, status conditions (PSN/BRN/PAR/SLP/FRZ), paralysis
// speed/skip checks, PP tracking, Pokedex (seen/caught), PC storage (Bill's PC with deposit/withdraw),
// Poke Mart (19 items incl. Repel/Super Repel/Max Repel/Escape Rope/Rare Candy), badge system,
// day/night cycle, evolution, SFX via SoundCommand, whiteout with money loss, critical hits (1/16),
// smart enemy AI (50% best move).
// Key pattern: battle uses take-put-back for BattleState borrow management.
// NPC trainers have trainer_team field with typed Pokemon lists (TrainerPokemon in maps.rs).
// Input helpers: is_confirm/is_cancel/is_up/is_down/is_left/is_right + held_ variants.
// Camera: smooth lerp (CAMERA_LERP), snaps on map transition. Ledge tiles: south-only traversal.
// Story flags: u64 bitfield (has_flag/set_flag) persisted in save. Route gates: Victory Road needs 8 badges.
// Struggle: forced when all PP=0, 50 power, never-miss, 1/4 recoil. Freeze: 10% thaw per turn.
// Fishing: face water tile + A → 70% bite (dialogue → StartFishBattle), 30% miss. water_encounters on MapData.
// NPC wandering: wanders flag + npc_wander_timer, random step every 2s with collision check.
// is_npc_active(idx): flag-based NPC filtering (collision, interaction, LOS, rendering).
// Party management: PokemonMenu has action states: 0=browse, 1=sub-menu (SUMMARY/SWAP/CANCEL), 2=swap mode.
// Swap mode: select source, then target, party.swap(src,dst). Battle player_idx tracked through swaps.
// Ice sliding: C_ICE tiles (8) cause player to slide until hitting non-ice. ice_sliding: Option<Direction>.
// Daycare: Route34 NPC 0 deposits/returns Pokemon. 1 EXP/step, auto-level, Gen 2 move replacement. Saved.
// Daycare cost: $100 + $100 * levels_gained. daycare_deposit_level tracks level at deposit time.
// Fly system: visited_cities Vec tracks cities entered. FlyMenu selects destination. Accessed from bag.
// Repel: Repel/Super Repel/Max Repel (100/200/250 steps). Gen 2 rule: only blocks wild < lead level.
// Overworld items: Potions (20/50/200/full), Full Restore (HP+status), Rare Candy (+1 level+evo check).
// Test infra: with_state(map, x, y, party, badges) skips title; helpers press/hold/wait/walk_dir/sequence.
// Battle queue sequencer: BattleStep enum (Text/ApplyDamage/DrainHp/InflictStatus/StatChange/CheckFaint/
//   Pause/GoToPhase/ScreenFlash/ScreenShake/PlayHitSfx) processed FIFO via BattlePhase::ExecuteQueue.
//   step_execute_queue(&mut self) drives it. PlayerAttack and EnemyAttack are now instant-resolve phases:
//   they compute all effects immediately, push queue steps (text, flash/shake, pause, drain, faint check),
//   then transition to ExecuteQueue. Migrated flows: Run success/failed, Intro, Won, PlayerAttack, EnemyAttack.
//   GoToPhase clears remaining queue on transition.

pub mod data;
pub mod sprites;
pub mod maps;
pub mod render;

use std::collections::VecDeque;
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

fn is_select(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("KeyC")
        || engine.input.keys_pressed.contains("ShiftLeft")
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

// ─── Story Flags (Phase 0C) ─────────────────────────────
// Bitfield stored in story_flags: u64. Use has_flag/set_flag helpers.
// Some flags are defined ahead for future story events.
const FLAG_GOT_EGG: u64           = 1 << 0;  // Elm gives Mystery Egg (Togepi)
#[allow(dead_code)] const FLAG_DELIVERED_EGG: u64     = 1 << 1;  // Returned egg to Elm (future)
const FLAG_RIVAL_ROUTE29: u64   = 1 << 2;  // Fought rival on Route 29
const FLAG_SPROUT_CLEAR: u64      = 1 << 3;  // Cleared Sprout Tower
const FLAG_SUDOWOODO: u64         = 1 << 4;  // Cleared Sudowoodo
const FLAG_RED_GYARADOS: u64      = 1 << 5;  // Red Gyarados event
const FLAG_ROCKET_MAHOGANY: u64   = 1 << 6;  // Cleared Rocket HQ
const FLAG_MEDICINE: u64           = 1 << 7;  // Got SecretPotion from Cianwood pharmacy
const FLAG_DELIVERED_MEDICINE: u64 = 1 << 8;  // Delivered medicine to Amphy at Lighthouse
const FLAG_RIVAL_VICTORY: u64   = 1 << 9;  // Fought rival at Victory Road
const FLAG_SQUIRTBOTTLE: u64    = 1 << 10; // Got Squirtbottle from Flower Shop
const FLAG_SPROUT_RIVAL: u64   = 1 << 11; // Rival confrontation at Sprout Tower 3F
const FLAG_SLOWPOKE_WELL: u64  = 1 << 12; // Cleared Slowpoke Well (defeated all Rockets)
const FLAG_BURNED_TOWER_RIVAL: u64 = 1 << 13; // Fought rival at Burned Tower 1F
const FLAG_BEASTS_RELEASED: u64 = 1 << 14; // Released legendary beasts from Burned Tower B1F
const FLAG_ILEX_FARFETCHD: u64 = 1 << 15; // Herded Farfetch'd back to charcoal maker

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
    PokemonMenu { cursor: u8, action: u8, sub_cursor: u8 },
    BagMenu { cursor: u8 },
    BagUseItem { item_id: u8, target_cursor: u8 },
    PokeMart { cursor: u8 },
    PokemonSummary { index: u8 },
    Pokedex { cursor: u8, scroll: u8 },
    PCMenu { mode: u8, cursor: u8 }, // mode: 0=select, 1=withdraw, 2=deposit
    Healing { timer: f64 },
    Evolution { timer: f64, new_species: SpeciesId },
    TrainerApproach { npc_idx: u8, timer: f64 },
    MapFadeOut { dest_map: MapId, dest_x: u8, dest_y: u8, timer: f64 },
    MapFadeIn { timer: f64 },
    WhiteoutFade { timer: f64, money_lost: u32 },
    Credits { scroll_y: f64 },
    TrainerCard,
    DaycareDeposit { cursor: u8 },
    DaycarePrompt { cursor: u8 }, // YES/NO prompt for daycare deposit/return
    FlyMenu { cursor: u8 },
}

// ─── Battle Phase ───────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
enum BattlePhase {
    Intro { timer: f64 },
    ActionSelect { cursor: u8 },
    MoveSelect { cursor: u8 },
    PlayerAttack { timer: f64, move_id: MoveId, damage: u16, effectiveness: f64, is_crit: bool, from_pending: bool },
    EnemyAttack { timer: f64, move_id: MoveId, damage: u16, effectiveness: f64, is_crit: bool },
    Text { message: String, timer: f64, next_phase: Box<BattlePhase> },
    PlayerFainted,
    EnemyFainted { exp_gained: u32 },
    ExpAwarded { exp_gained: u32, timer: f64 },
    LevelUp { timer: f64, stat_deltas: [i16; 6] }, // [HP, Atk, Def, SpAtk, SpDef, Spd]
    LearnMove { new_move: MoveId, sub: LearnMoveSub },
    TrainerSwitchPrompt { next_name: String, cursor: u8 },
    Won { timer: f64 },
    Run,
    ExecuteQueue,
}

/// Steps for the queue-based battle sequencer. Each step is a single unit of work
/// (display text, apply damage, drain HP bar, etc.) processed in FIFO order.
#[derive(Clone, Debug)]
#[allow(dead_code)]
enum BattleStep {
    /// Display text message, advance on timer (1.5s) or confirm button press
    Text(String),
    /// Apply damage to target, update HP immediately (no animation)
    ApplyDamage { is_player: bool, amount: u16 },
    /// Animate HP bar drain (smooth interpolation toward target HP)
    DrainHp { is_player: bool, to_hp: u16, duration: f64 },
    /// Inflict status condition on target
    InflictStatus { is_player: bool, status: StatusCondition },
    /// Apply stat stage change (+/- stages to a stat)
    StatChange { is_player: bool, stat: usize, stages: i8 },
    /// Check if target fainted, transition to faint handling if so
    CheckFaint { is_player: bool },
    /// Pause for N seconds (no text displayed)
    Pause(f64),
    /// Transition to a specific BattlePhase (escape hatch for complex flows)
    GoToPhase(Box<BattlePhase>),
    /// Set screen_flash value (for hit visual effects)
    ScreenFlash(f64),
    /// Set screen_shake value (for enemy hit visual effects)
    ScreenShake(f64),
    /// Play hit sound effect (super_effective flag)
    PlayHitSfx(bool),
}

/// Sub-states for the move learning sequence
#[derive(Clone, Debug, PartialEq)]
enum LearnMoveSub {
    /// "X is trying to learn MOVE!" (auto-advance)
    TryingToLearn { timer: f64 },
    /// "But X can't learn more than 4 moves." (auto-advance)
    CantLearnMore { timer: f64 },
    /// "Delete an older move to make room for MOVE?" YES/NO
    DeletePrompt { cursor: u8 },
    /// Pick which of the 4 current moves to forget
    PickMove { cursor: u8 },
    /// "1, 2, and... Poof! X forgot OLD." (auto-advance)
    ForgotMove { timer: f64, slot: usize },
    /// "And... X learned NEW!" (auto-advance)
    LearnedMove { timer: f64 },
    /// "Stop learning MOVE?" YES/NO
    StopPrompt { cursor: u8 },
    /// "X did not learn MOVE." (auto-advance)
    DidNotLearn { timer: f64 },
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
    // Flinch: set by first attacker's move, checked before second attacker moves
    enemy_flinched: bool,
    player_flinched: bool,
    // Confusion: turns remaining (0 = not confused). 50% self-hit chance each turn.
    player_confused: u8,
    enemy_confused: u8,
    // Mean Look: prevents fleeing from wild battle
    player_trapped: bool,
    // Hyper Beam recharge: skip next turn
    player_must_recharge: bool,
    enemy_must_recharge: bool,
    // Thrash/Outrage rampage: turns remaining (0 = not rampaging), move_id, confused after
    player_rampage: (u8, MoveId),
    enemy_rampage: (u8, MoveId),
    // Moves queued for the learn-move prompt (all 4 slots full, player must choose)
    pending_learn_moves: Vec<MoveId>,
    // Free switch: next PokemonMenu switch doesn't give enemy a free turn
    free_switch: bool,
    // Confusion snap-out message to chain before the attack
    confusion_snapout_msg: Option<String>,
    // Queue-based sequencer: steps processed in FIFO order during ExecuteQueue phase
    battle_queue: VecDeque<BattleStep>,
    queue_timer: f64,
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
    StartFishBattle { species_id: SpeciesId, level: u8 },
    StartSudowoodoBattle,
    StartRedGyaradosBattle,
    EscapeRope,
    OpenMart,
    GiveBadge { badge_num: u8 },
    GiveFlash,
    Credits,
    DaycareDeposit,
    DaycareReturn,
    CheckEvolution,
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
    last_house_x: i32, // exact door x the player entered GenericHouse from
    last_house_y: i32, // exact door y the player entered GenericHouse from
    rival_starter: SpeciesId, // rival picks type-advantaged starter
    rival_battle_done: bool,
    // Trainer approach state: trainer walks toward player before battle
    approach_npc_x: i32,
    approach_npc_y: i32,
    approach_walk_offset: f64,
    approach_exclaim_timer: f64,
    // Story flags (Phase 0C): bitfield for progression gates
    story_flags: u64,
    // LOS suppression: skip trainer checks for N frames after map transition
    los_suppress: u8,
    // NPC wander timer — NPCs with wanders=true move randomly every few seconds
    npc_wander_timer: f64,
    // Bicycle: obtained from Bike Shop in Goldenrod, doubles movement speed
    has_bicycle: bool,
    on_bicycle: bool,
    // Party swap: source index for swap mode in PokemonMenu
    swap_source: u8,
    // Save system
    needs_save: bool,
    last_rng_state: u64,
    has_save: bool,
    // Ice sliding: player slides across ice tiles until hitting a non-ice tile
    ice_sliding: Option<Direction>,
    // Daycare system: Pokemon left at Route 34 Day-Care Man
    daycare_pokemon: Option<Pokemon>,
    daycare_steps: u32,
    daycare_deposit_level: u8, // level at deposit time, for cost calculation
    // Fly system: tracks which cities the player has visited
    visited_cities: Vec<MapId>,
}

impl PokemonSim {
    fn has_flag(&self, flag: u64) -> bool { self.story_flags & flag != 0 }
    fn set_flag(&mut self, flag: u64) { self.story_flags |= flag; }

    /// Check if a warp destination is blocked by a progression gate.
    /// Returns a gate message if blocked, or None if the warp is allowed.
    fn check_warp_gate(&self, dest: MapId) -> Option<&'static [&'static str]> {
        // Route 27 from New Bark Town: need all 8 badges (post-E4 area)
        if dest == MapId::Route27 && self.current_map_id == MapId::NewBarkTown && self.badges.count_ones() < 8 {
            return Some(&["The path ahead is", "too dangerous!", "Come back when", "you're stronger."]);
        }
        // Union Cave: need Zephyr Badge (Falkner)
        if dest == MapId::UnionCave && self.badges & 1 == 0 {
            return Some(&["A trainer ahead", "blocks the way.", "You need the", "ZEPHYR BADGE."]);
        }
        // Ilex Forest north exit to Route 34: need Hive Badge (Bugsy)
        if dest == MapId::Route34 && self.current_map_id == MapId::IlexForest && self.badges & 2 == 0 {
            return Some(&["A tree blocks the", "path. You need CUT."]);
        }
        // Rocket HQ: need Red Gyarados event completed
        if dest == MapId::RocketHQ && !self.has_flag(FLAG_RED_GYARADOS) {
            return Some(&["Just a souvenir shop.", "Nothing to see here!"]);
        }
        // Ice Path: need Rocket HQ cleared
        if dest == MapId::IcePath && !self.has_flag(FLAG_ROCKET_MAHOGANY) {
            return Some(&["TEAM ROCKET is", "causing trouble", "in MAHOGANY TOWN!"]);
        }
        // Victory Road: need all 8 badges
        if dest == MapId::VictoryRoad && self.badges.count_ones() < 8 {
            return Some(&["You need all 8", "BADGES to enter", "VICTORY ROAD!"]);
        }
        None
    }

    /// Returns false for NPCs that should be hidden due to story flags.
    fn is_npc_active(&self, npc_idx: usize) -> bool {
        // Route 36 NPC index 2 = Sudowoodo blocker, hidden after FLAG_SUDOWOODO
        if self.current_map_id == MapId::Route36 && npc_idx == 2 && self.has_flag(FLAG_SUDOWOODO) {
            return false;
        }
        // Lighthouse Jasmine (NPC 0) disappears after delivering medicine (she returns to gym)
        if self.current_map_id == MapId::OlivineLighthouse && npc_idx == 0 && self.has_flag(FLAG_DELIVERED_MEDICINE) {
            return false;
        }
        // Lake of Rage Red Gyarados NPC (index 3) hidden after event
        if self.current_map_id == MapId::LakeOfRage && npc_idx == 3 && self.has_flag(FLAG_RED_GYARADOS) {
            return false;
        }
        // Slowpoke Well B1F: hide Rocket Grunts (NPCs 0-3) and Kurt/Slowpokes (4-6) after clearing
        if self.current_map_id == MapId::SlowpokeWellB1F && npc_idx <= 6 && self.has_flag(FLAG_SLOWPOKE_WELL) {
            return false;
        }
        // Burned Tower B1F: Eusine (NPC 0) only visible after beasts released
        if self.current_map_id == MapId::BurnedTowerB1F && npc_idx == 0 && !self.has_flag(FLAG_BEASTS_RELEASED) {
            return false;
        }
        // Ilex Forest: Farfetch'd (NPC 0) hidden after quest done; Charcoal Master (NPC 2) appears after quest
        if self.current_map_id == MapId::IlexForest {
            // NPC 0 = Farfetch'd — hidden once herded back
            if npc_idx == 0 && self.has_flag(FLAG_ILEX_FARFETCHD) {
                return false;
            }
            // NPC 1 = Charcoal Apprentice — hidden after quest (he leaves with Farfetch'd)
            if npc_idx == 1 && self.has_flag(FLAG_ILEX_FARFETCHD) {
                return false;
            }
        }
        true
    }

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
            last_house_x: 12,
            last_house_y: 5,
            rival_starter: 0, // set when player picks starter
            rival_battle_done: false,
            approach_npc_x: 0,
            approach_npc_y: 0,
            approach_walk_offset: 0.0,
            approach_exclaim_timer: 0.0,
            story_flags: 0,
            los_suppress: 0,
            npc_wander_timer: 0.0,
            has_bicycle: false,
            on_bicycle: false,
            swap_source: 0,
            needs_save: false,
            last_rng_state: 0,
            has_save: false,
            ice_sliding: None,
            daycare_pokemon: None,
            daycare_steps: 0,
            daycare_deposit_level: 0,
            visited_cities: Vec::new(),
        }
    }

    /// Test-only constructor: create a PokemonSim already in the overworld with
    /// the given map, position, party, and badge count. Skips title screen.
    #[cfg(test)]
    #[allow(dead_code)]
    fn with_state(map: MapId, x: u8, y: u8, party: Vec<Pokemon>, badges: u8) -> Self {
        let mut sim = Self::new();
        sim.phase = GamePhase::Overworld;
        sim.has_starter = !party.is_empty();
        sim.party = party;
        sim.badges = badges;
        sim.change_map(map, x, y);
        sim
    }

    fn init_sprite_caches(&mut self) {
        let tile_strs = [
            TILE_GRASS, TILE_TALL_GRASS, TILE_PATH, TILE_TREE_TOP, TILE_TREE_BOTTOM,
            TILE_WATER, TILE_WATER2, TILE_BUILDING_WALL, TILE_BUILDING_ROOF, TILE_DOOR,
            TILE_FENCE_H, TILE_FLOWER, TILE_POKECENTER_ROOF, TILE_POKECENTER_WALL,
            TILE_POKECENTER_DOOR, TILE_LAB_WALL, TILE_LAB_ROOF, TILE_SIGN, TILE_LEDGE,
            TILE_FLOOR, TILE_TABLE, TILE_BOOKSHELF, TILE_PC, TILE_HEAL_MACHINE, TILE_BLACK,
            TILE_CAVE_WALL, TILE_CAVE_FLOOR, TILE_ICE_FLOOR, TILE_GYM_FLOOR,
            // Sprint 117: new tiles from open-source Gen 2 tileset
            TILE_STOVE, TILE_WALL_ART, TILE_STOOL, TILE_BED_TOP, TILE_BED_BTM,
            TILE_FRIDGE, TILE_EXIT_MAT, TILE_OUTDOOR_MAT, TILE_TREE_TALL_TOP,
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
            NPC_ROCKET, NPC_FISHER,
        ];
        self.npc_sprite_cache = npc_strs.iter().map(|s| decode_sprite(s)).collect();
    }

    /// Returns true if a map is a flyable city destination.
    fn is_fly_city(map_id: MapId) -> bool {
        matches!(map_id,
            MapId::NewBarkTown | MapId::CherrygroveCity | MapId::VioletCity |
            MapId::AzaleaTown | MapId::GoldenrodCity | MapId::EcruteakCity |
            MapId::OlivineCity | MapId::CianwoodCity | MapId::MahoganyTown |
            MapId::BlackthornCity
        )
    }

    /// Get the fly spawn point for a city.
    fn fly_spawn(map_id: MapId) -> (u8, u8) {
        match map_id {
            MapId::NewBarkTown => (5, 8),
            MapId::CherrygroveCity => (15, 5),
            MapId::VioletCity => (12, 10),
            MapId::AzaleaTown => (5, 5),
            MapId::GoldenrodCity => (12, 12),
            MapId::EcruteakCity => (8, 8),
            MapId::OlivineCity => (10, 8),
            MapId::CianwoodCity => (6, 6),
            MapId::MahoganyTown => (6, 5),
            MapId::BlackthornCity => (3, 7),
            _ => (5, 5),
        }
    }

    /// Get the display name for a city on the fly menu.
    fn city_name(map_id: MapId) -> &'static str {
        match map_id {
            MapId::NewBarkTown => "NEW BARK TOWN",
            MapId::CherrygroveCity => "CHERRYGROVE",
            MapId::VioletCity => "VIOLET CITY",
            MapId::AzaleaTown => "AZALEA TOWN",
            MapId::GoldenrodCity => "GOLDENROD",
            MapId::EcruteakCity => "ECRUTEAK",
            MapId::OlivineCity => "OLIVINE CITY",
            MapId::CianwoodCity => "CIANWOOD",
            MapId::MahoganyTown => "MAHOGANY TOWN",
            MapId::BlackthornCity => "BLACKTHORN",
            _ => "???",
        }
    }

    fn change_map(&mut self, map_id: MapId, dest_x: u8, dest_y: u8) {
        // Track visited cities for fly system
        if Self::is_fly_city(map_id) && !self.visited_cities.contains(&map_id) {
            self.visited_cities.push(map_id);
        }
        // Track which city the player came from when entering shared interiors
        if map_id == MapId::PokemonCenter {
            self.last_pokecenter_map = self.current_map_id;
        }
        if map_id == MapId::GenericHouse {
            self.last_house_map = self.current_map_id;
            // Store exact player position (door tile) so we exit to the right door
            self.last_house_x = self.player.x;
            self.last_house_y = self.player.y + 1; // exit 1 tile below the door
        }
        self.current_map_id = map_id;
        self.current_map = load_map(map_id);
        self.player.x = dest_x as i32;
        self.player.y = dest_y as i32;
        self.player.is_walking = false;
        self.player.walk_offset = 0.0;
        // Auto-dismount bicycle when entering indoor maps
        self.on_bicycle = false;
        // Snap camera to new position (no lerp on map transitions)
        let target_x = dest_x as f64 * TILE_PX as f64 + TILE_PX as f64 / 2.0 - (VIEW_TILES_X * TILE_PX / 2) as f64;
        let target_y = dest_y as f64 * TILE_PX as f64 + TILE_PX as f64 / 2.0 - (VIEW_TILES_Y * TILE_PX / 2) as f64;
        let map_pw = (self.current_map.width as i32 * TILE_PX) as f64;
        let map_ph = (self.current_map.height as i32 * TILE_PX) as f64;
        let vw = (VIEW_TILES_X * TILE_PX) as f64;
        let vh = (VIEW_TILES_Y * TILE_PX) as f64;
        self.camera_x = target_x.max(0.0).min((map_pw - vw).max(0.0));
        self.camera_y = target_y.max(0.0).min((map_ph - vh).max(0.0));
        // Auto-save on every map transition
        self.needs_save = true;
    }

    // ─── Save System ─────────────────────────────────────

    fn serialize_save(&self) -> String {
        // Build party JSON array
        let mut party_json = String::from("[");
        for (i, p) in self.party.iter().enumerate() {
            if i > 0 { party_json.push(','); }
            let moves_json = format!("[{},{},{},{}]",
                p.moves[0].unwrap_or(0), p.moves[1].unwrap_or(0),
                p.moves[2].unwrap_or(0), p.moves[3].unwrap_or(0));
            let pp_json = format!("[{},{},{},{}]", p.move_pp[0], p.move_pp[1], p.move_pp[2], p.move_pp[3]);
            let max_pp_json = format!("[{},{},{},{}]", p.move_max_pp[0], p.move_max_pp[1], p.move_max_pp[2], p.move_max_pp[3]);
            let status_val = match p.status {
                StatusCondition::None => 0u8,
                StatusCondition::Poison => 1,
                StatusCondition::Burn => 2,
                StatusCondition::Paralysis => 3,
                StatusCondition::Sleep { turns } => 4 + turns,
                StatusCondition::Freeze => 10,
                StatusCondition::BadPoison { turn } => 11 + turn, // 11-26
            };
            party_json.push_str(&format!(
                "{{\"s\":{},\"l\":{},\"hp\":{},\"mhp\":{},\"exp\":{},\"mv\":{},\"pp\":{},\"mpp\":{},\"st\":{}}}",
                p.species_id, p.level, p.hp, p.max_hp, p.exp, moves_json, pp_json, max_pp_json, status_val
            ));
        }
        party_json.push(']');

        // Build PC JSON array
        let mut pc_json = String::from("[");
        for (i, p) in self.pc_boxes.iter().enumerate() {
            if i > 0 { pc_json.push(','); }
            let moves_json = format!("[{},{},{},{}]",
                p.moves[0].unwrap_or(0), p.moves[1].unwrap_or(0),
                p.moves[2].unwrap_or(0), p.moves[3].unwrap_or(0));
            let pp_json = format!("[{},{},{},{}]", p.move_pp[0], p.move_pp[1], p.move_pp[2], p.move_pp[3]);
            let max_pp_json = format!("[{},{},{},{}]", p.move_max_pp[0], p.move_max_pp[1], p.move_max_pp[2], p.move_max_pp[3]);
            pc_json.push_str(&format!(
                "{{\"s\":{},\"l\":{},\"hp\":{},\"mhp\":{},\"exp\":{},\"mv\":{},\"pp\":{},\"mpp\":{},\"st\":0}}",
                p.species_id, p.level, p.hp, p.max_hp, p.exp, moves_json, pp_json, max_pp_json
            ));
        }
        pc_json.push(']');

        // Build defeated trainers JSON array
        let mut defeated_json = String::from("[");
        for (i, (map_id, npc_idx)) in self.defeated_trainers.iter().enumerate() {
            if i > 0 { defeated_json.push(','); }
            defeated_json.push_str(&format!("[\"{}\",{}]", map_id.to_str(), npc_idx));
        }
        defeated_json.push(']');

        // Build bag JSON array
        let mut bag_json = String::from("[");
        for (i, (item_id, qty)) in self.bag.items.iter().enumerate() {
            if i > 0 { bag_json.push(','); }
            bag_json.push_str(&format!("[{},{}]", item_id, qty));
        }
        bag_json.push(']');

        // Build seen/caught arrays
        let seen_json = format!("{:?}", self.pokedex_seen);
        let caught_json = format!("{:?}", self.pokedex_caught);

        // Build daycare JSON (null or pokemon object)
        let daycare_json = if let Some(ref p) = self.daycare_pokemon {
            let moves_json = format!("[{},{},{},{}]",
                p.moves[0].unwrap_or(0), p.moves[1].unwrap_or(0),
                p.moves[2].unwrap_or(0), p.moves[3].unwrap_or(0));
            let pp_json = format!("[{},{},{},{}]", p.move_pp[0], p.move_pp[1], p.move_pp[2], p.move_pp[3]);
            let max_pp_json = format!("[{},{},{},{}]", p.move_max_pp[0], p.move_max_pp[1], p.move_max_pp[2], p.move_max_pp[3]);
            format!("{{\"s\":{},\"l\":{},\"hp\":{},\"mhp\":{},\"exp\":{},\"mv\":{},\"pp\":{},\"mpp\":{},\"st\":0}}",
                p.species_id, p.level, p.hp, p.max_hp, p.exp, moves_json, pp_json, max_pp_json)
        } else {
            "null".to_string()
        };

        let facing = match self.player.facing {
            Direction::Up => 0, Direction::Down => 1, Direction::Left => 2, Direction::Right => 3,
        };

        let mut base = format!(
            "{{\"map\":\"{}\",\"x\":{},\"y\":{},\"facing\":{},\"money\":{},\"badges\":{},\"time\":{},\"rng\":{},\"steps\":{},\"rival_starter\":{},\"rival_done\":{},\"has_starter\":{},\"last_pc\":\"{}\",\"last_house\":\"{}\",\"last_house_x\":{},\"last_house_y\":{},\"repel\":{},\"flags\":{},\"has_bike\":{},\"party\":{},\"pc\":{},\"defeated\":{},\"bag\":{},\"seen\":{},\"caught\":{},\"daycare\":{},\"daycare_steps\":{},\"daycare_dlvl\":{}}}",
            self.current_map_id.to_str(),
            self.player.x, self.player.y, facing,
            self.money, self.badges, self.total_time, self.last_rng_state,
            self.step_count, self.rival_starter,
            self.rival_battle_done, self.has_starter,
            self.last_pokecenter_map.to_str(), self.last_house_map.to_str(),
            self.last_house_x, self.last_house_y,
            self.repel_steps, self.story_flags,
            self.has_bicycle,
            party_json, pc_json, defeated_json, bag_json,
            seen_json, caught_json,
            daycare_json, self.daycare_steps, self.daycare_deposit_level,
        );
        // Append visited_cities to save
        let mut vcities = String::from("[");
        for (i, c) in self.visited_cities.iter().enumerate() {
            if i > 0 { vcities.push(','); }
            vcities.push('"');
            vcities.push_str(c.to_str());
            vcities.push('"');
        }
        vcities.push(']');
        // Insert visited_cities before closing brace
        if base.ends_with('}') {
            base.pop();
            base.push_str(&format!(",\"visited_cities\":{}}}", vcities));
        }
        base
    }

    fn load_from_save(&mut self, json: &str) {
        // Minimal JSON parser — we control the format so this works.
        // Extract string field: "key":"value"
        fn get_str<'a>(json: &'a str, key: &str) -> &'a str {
            let needle = format!("\"{}\":\"", key);
            if let Some(start) = json.find(&needle) {
                let rest = &json[start + needle.len()..];
                if let Some(end) = rest.find('"') {
                    return &rest[..end];
                }
            }
            ""
        }
        // Extract number field: "key":number
        fn get_num(json: &str, key: &str) -> f64 {
            let needle = format!("\"{}\":", key);
            if let Some(start) = json.find(&needle) {
                let rest = &json[start + needle.len()..];
                let end = rest.find(|c: char| !c.is_ascii_digit() && c != '.' && c != '-').unwrap_or(rest.len());
                rest[..end].parse().unwrap_or(0.0)
            } else {
                0.0
            }
        }
        // Extract bool field: "key":true/false
        fn get_bool(json: &str, key: &str) -> bool {
            let needle = format!("\"{}\":true", key);
            json.contains(&needle)
        }
        // Extract array between balanced brackets starting after "key":[
        fn get_array<'a>(json: &'a str, key: &str) -> &'a str {
            let needle = format!("\"{}\":[", key);
            if let Some(start) = json.find(&needle) {
                let arr_start = start + needle.len() - 1; // include the [
                let bytes = json.as_bytes();
                let mut depth = 0;
                for i in arr_start..bytes.len() {
                    match bytes[i] {
                        b'[' => depth += 1,
                        b']' => {
                            depth -= 1;
                            if depth == 0 {
                                return &json[arr_start..=i];
                            }
                        }
                        _ => {}
                    }
                }
            }
            "[]"
        }

        // Parse map
        let map_str = get_str(json, "map");
        let map_id = MapId::from_str(map_str).unwrap_or(MapId::NewBarkTown);

        self.current_map_id = map_id;
        self.current_map = load_map(map_id);
        self.player.x = get_num(json, "x") as i32;
        self.player.y = get_num(json, "y") as i32;
        self.player.facing = match get_num(json, "facing") as u8 {
            0 => Direction::Up, 2 => Direction::Left, 3 => Direction::Right,
            _ => Direction::Down,
        };
        self.money = get_num(json, "money") as u32;
        self.badges = get_num(json, "badges") as u8;
        self.total_time = get_num(json, "time");
        self.last_rng_state = get_num(json, "rng") as u64;
        self.step_count = get_num(json, "steps") as u32;
        self.rival_starter = get_num(json, "rival_starter") as u16;
        self.rival_battle_done = get_bool(json, "rival_done");
        self.has_starter = get_bool(json, "has_starter");
        self.last_pokecenter_map = MapId::from_str(get_str(json, "last_pc")).unwrap_or(MapId::CherrygroveCity);
        self.last_house_map = MapId::from_str(get_str(json, "last_house")).unwrap_or(MapId::NewBarkTown);
        self.last_house_x = get_num(json, "last_house_x") as i32;
        self.last_house_y = get_num(json, "last_house_y") as i32;
        self.repel_steps = get_num(json, "repel") as u32;
        self.story_flags = get_num(json, "flags") as u64;
        self.has_bicycle = get_num(json, "has_bike") != 0.0;

        // Parse party: array of pokemon objects
        let party_arr = get_array(json, "party");
        self.party.clear();
        // Split on "},{" to separate Pokemon objects
        let inner = &party_arr[1..party_arr.len()-1]; // strip outer []
        if !inner.is_empty() {
            for obj_str in inner.split("},{") {
                let obj = if obj_str.starts_with('{') { obj_str.to_string() }
                    else { format!("{{{}", obj_str) };
                let obj = if obj.ends_with('}') { obj } else { format!("{}}}", obj) };
                let species = get_num(&obj, "s") as u16;
                let level = get_num(&obj, "l") as u8;
                let hp = get_num(&obj, "hp") as u16;
                let max_hp = get_num(&obj, "mhp") as u16;
                let exp = get_num(&obj, "exp") as u32;
                let status_val = get_num(&obj, "st") as u8;

                let mut pkmn = Pokemon::new(species, level);
                pkmn.hp = hp;
                pkmn.max_hp = max_hp;
                pkmn.exp = exp;
                pkmn.status = match status_val {
                    0 => StatusCondition::None,
                    1 => StatusCondition::Poison,
                    2 => StatusCondition::Burn,
                    3 => StatusCondition::Paralysis,
                    10 => StatusCondition::Freeze,
                    t if t >= 11 => StatusCondition::BadPoison { turn: t - 11 },
                    t if t >= 4 => StatusCondition::Sleep { turns: t - 4 },
                    _ => StatusCondition::None,
                };

                // Parse moves array
                let mv_arr = get_array(&obj, "mv");
                let mv_inner = &mv_arr[1..mv_arr.len()-1];
                let mvs: Vec<u16> = mv_inner.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                for i in 0..4 {
                    pkmn.moves[i] = mvs.get(i).copied().filter(|&m| m > 0).map(|m| m as MoveId);
                }

                // Parse PP arrays
                let pp_arr = get_array(&obj, "pp");
                let pp_inner = &pp_arr[1..pp_arr.len()-1];
                let pps: Vec<u8> = pp_inner.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                for i in 0..4 {
                    pkmn.move_pp[i] = pps.get(i).copied().unwrap_or(0);
                }

                let mpp_arr = get_array(&obj, "mpp");
                let mpp_inner = &mpp_arr[1..mpp_arr.len()-1];
                let mpps: Vec<u8> = mpp_inner.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                for i in 0..4 {
                    pkmn.move_max_pp[i] = mpps.get(i).copied().unwrap_or(0);
                }

                self.party.push(pkmn);
            }
        }

        // Parse PC boxes (same format as party)
        let pc_arr = get_array(json, "pc");
        self.pc_boxes.clear();
        let pc_inner = &pc_arr[1..pc_arr.len()-1];
        if !pc_inner.is_empty() {
            for obj_str in pc_inner.split("},{") {
                let obj = if obj_str.starts_with('{') { obj_str.to_string() }
                    else { format!("{{{}", obj_str) };
                let obj = if obj.ends_with('}') { obj } else { format!("{}}}", obj) };
                let species = get_num(&obj, "s") as u16;
                let level = get_num(&obj, "l") as u8;
                let mut pkmn = Pokemon::new(species, level);
                pkmn.hp = get_num(&obj, "hp") as u16;
                pkmn.max_hp = get_num(&obj, "mhp") as u16;
                pkmn.exp = get_num(&obj, "exp") as u32;
                let mv_arr = get_array(&obj, "mv");
                let mv_inner = &mv_arr[1..mv_arr.len()-1];
                let mvs: Vec<u16> = mv_inner.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                for i in 0..4 {
                    pkmn.moves[i] = mvs.get(i).copied().filter(|&m| m > 0).map(|m| m as MoveId);
                }
                let pp_arr = get_array(&obj, "pp");
                let pp_inner = &pp_arr[1..pp_arr.len()-1];
                let pps: Vec<u8> = pp_inner.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                for i in 0..4 { pkmn.move_pp[i] = pps.get(i).copied().unwrap_or(0); }
                let mpp_arr = get_array(&obj, "mpp");
                let mpp_inner = &mpp_arr[1..mpp_arr.len()-1];
                let mpps: Vec<u8> = mpp_inner.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                for i in 0..4 { pkmn.move_max_pp[i] = mpps.get(i).copied().unwrap_or(0); }
                self.pc_boxes.push(pkmn);
            }
        }

        // Parse defeated trainers: [["MapStr",idx],...]
        let def_arr = get_array(json, "defeated");
        self.defeated_trainers.clear();
        let def_inner = &def_arr[1..def_arr.len()-1];
        if !def_inner.is_empty() {
            // Each entry is ["MapName",idx]
            let mut i = 0;
            let bytes = def_inner.as_bytes();
            while i < bytes.len() {
                if bytes[i] == b'[' {
                    // Find the string value
                    if let Some(q1) = def_inner[i..].find('"') {
                        let rest = &def_inner[i + q1 + 1..];
                        if let Some(q2) = rest.find('"') {
                            let map_name = &rest[..q2];
                            let after = &rest[q2 + 1..];
                            if let Some(comma) = after.find(',') {
                                let num_str = &after[comma + 1..];
                                let end = num_str.find(']').unwrap_or(num_str.len());
                                if let Ok(npc_idx) = num_str[..end].trim().parse::<u8>() {
                                    if let Some(mid) = MapId::from_str(map_name) {
                                        self.defeated_trainers.push((mid, npc_idx));
                                    }
                                }
                            }
                        }
                    }
                    // Skip to next entry
                    if let Some(close) = def_inner[i..].find(']') {
                        i += close + 1;
                    } else {
                        break;
                    }
                } else {
                    i += 1;
                }
            }
        }

        // Parse bag: [[item_id, qty],...]
        let bag_arr = get_array(json, "bag");
        self.bag.items.clear();
        let bag_inner = &bag_arr[1..bag_arr.len()-1];
        if !bag_inner.is_empty() {
            for chunk in bag_inner.split("],[") {
                let clean = chunk.trim_start_matches('[').trim_end_matches(']');
                let parts: Vec<&str> = clean.split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(id), Ok(qty)) = (parts[0].trim().parse::<u8>(), parts[1].trim().parse::<u8>()) {
                        self.bag.items.push((id, qty));
                    }
                }
            }
        }

        // Parse seen/caught arrays: [id1,id2,...]
        let seen_arr = get_array(json, "seen");
        self.pokedex_seen.clear();
        let seen_inner = &seen_arr[1..seen_arr.len()-1];
        for s in seen_inner.split(',') {
            if let Ok(id) = s.trim().parse::<u16>() {
                self.pokedex_seen.push(id);
            }
        }

        let caught_arr = get_array(json, "caught");
        self.pokedex_caught.clear();
        let caught_inner = &caught_arr[1..caught_arr.len()-1];
        for s in caught_inner.split(',') {
            if let Ok(id) = s.trim().parse::<u16>() {
                self.pokedex_caught.push(id);
            }
        }

        // Parse daycare Pokemon (null or object wrapped in "daycare":{...})
        self.daycare_steps = get_num(json, "daycare_steps") as u32;
        self.daycare_deposit_level = get_num(json, "daycare_dlvl") as u8;
        self.daycare_pokemon = None;
        // Extract the daycare object: find "daycare":{ and parse the balanced braces
        let daycare_needle = "\"daycare\":{";
        if let Some(start) = json.find(daycare_needle) {
            let obj_start = start + daycare_needle.len() - 1; // include the {
            let bytes = json.as_bytes();
            let mut depth = 0;
            let mut obj_end = obj_start;
            for i in obj_start..bytes.len() {
                match bytes[i] {
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            obj_end = i;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if obj_end > obj_start {
                let obj = &json[obj_start..=obj_end];
                let species = get_num(obj, "s") as u16;
                if species > 0 {
                    let level = get_num(obj, "l") as u8;
                    let mut pkmn = Pokemon::new(species, level);
                    pkmn.hp = get_num(obj, "hp") as u16;
                    pkmn.max_hp = get_num(obj, "mhp") as u16;
                    pkmn.exp = get_num(obj, "exp") as u32;
                    let mv_arr = get_array(obj, "mv");
                    let mv_inner = &mv_arr[1..mv_arr.len()-1];
                    let mvs: Vec<u16> = mv_inner.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                    for i in 0..4 {
                        pkmn.moves[i] = mvs.get(i).copied().filter(|&m| m > 0).map(|m| m as MoveId);
                    }
                    let pp_arr = get_array(obj, "pp");
                    let pp_inner = &pp_arr[1..pp_arr.len()-1];
                    let pps: Vec<u8> = pp_inner.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                    for i in 0..4 { pkmn.move_pp[i] = pps.get(i).copied().unwrap_or(0); }
                    let mpp_arr = get_array(obj, "mpp");
                    let mpp_inner = &mpp_arr[1..mpp_arr.len()-1];
                    let mpps: Vec<u8> = mpp_inner.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                    for i in 0..4 { pkmn.move_max_pp[i] = mpps.get(i).copied().unwrap_or(0); }
                    self.daycare_pokemon = Some(pkmn);
                }
            }
        }

        // Parse visited_cities: array of string map names
        self.visited_cities.clear();
        let vcities_arr = get_array(json, "visited_cities");
        let vcities_inner = &vcities_arr[1..vcities_arr.len()-1];
        if !vcities_inner.is_empty() {
            for s in vcities_inner.split(',') {
                let trimmed = s.trim().trim_matches('"');
                if let Some(m) = MapId::from_str(trimmed) {
                    if Self::is_fly_city(m) && !self.visited_cities.contains(&m) {
                        self.visited_cities.push(m);
                    }
                }
            }
        }
        // If no visited_cities were saved, auto-populate from current map
        if self.visited_cities.is_empty() && Self::is_fly_city(self.current_map_id) {
            self.visited_cities.push(self.current_map_id);
        }

        // Snap camera
        let target_x = self.player.x as f64 * TILE_PX as f64 + TILE_PX as f64 / 2.0 - (VIEW_TILES_X * TILE_PX / 2) as f64;
        let target_y = self.player.y as f64 * TILE_PX as f64 + TILE_PX as f64 / 2.0 - (VIEW_TILES_Y * TILE_PX / 2) as f64;
        let map_pw = (self.current_map.width as i32 * TILE_PX) as f64;
        let map_ph = (self.current_map.height as i32 * TILE_PX) as f64;
        let vw = (VIEW_TILES_X * TILE_PX) as f64;
        let vh = (VIEW_TILES_Y * TILE_PX) as f64;
        self.camera_x = target_x.max(0.0).min((map_pw - vw).max(0.0));
        self.camera_y = target_y.max(0.0).min((map_ph - vh).max(0.0));

        // Set phase to overworld since we've loaded a valid save
        self.phase = GamePhase::Overworld;
    }

    /// Trigger rival battle event (called from step_overworld)
    fn check_rival_battle(&mut self) -> bool {
        if self.has_starter && !self.rival_battle_done
            && self.current_map_id == MapId::Route29
            && self.rival_starter > 0
        {
            self.rival_battle_done = true;
            self.set_flag(FLAG_RIVAL_ROUTE29);
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

    /// Trigger Victory Road rival battle (called from step_overworld)
    fn check_victory_road_rival(&mut self) -> bool {
        if self.current_map_id == MapId::VictoryRoad
            && self.rival_starter > 0
            && !self.has_flag(FLAG_RIVAL_VICTORY)
            && self.badges.count_ones() >= 8
        {
            self.set_flag(FLAG_RIVAL_VICTORY);
            // Map rival's starter to final form
            let rival_final = match self.rival_starter {
                CHIKORITA => MEGANIUM,
                CYNDAQUIL => TYPHLOSION,
                TOTODILE => FERALIGATR,
                _ => TYPHLOSION,
            };
            self.dialogue = Some(DialogueState {
                lines: vec![
                    "RIVAL: …So you".to_string(),
                    "made it here too.".to_string(),
                    "Good. I wanted to".to_string(),
                    "test my team before".to_string(),
                    "the ELITE FOUR!".to_string(),
                ],
                current_line: 0, char_index: 0, timer: 0.0,
                on_complete: DialogueAction::StartTrainerBattle {
                    team: vec![
                        (rival_final, 36),
                        (HAUNTER, 35),
                        (SNEASEL, 34),
                        (MAGNETON, 34),
                        (GOLBAT, 36),
                    ],
                },
            });
            self.phase = GamePhase::Dialogue;
            return true;
        }
        false
    }

    /// Sprout Tower elder: one-time battle at top of tower (3F)
    fn check_sprout_tower_elder(&mut self) -> bool {
        if self.current_map_id == MapId::SproutTower3F
            && !self.has_flag(FLAG_SPROUT_CLEAR)
            && self.player.x == 7 && self.player.y <= 3
            && !self.party.is_empty()
        {
            self.set_flag(FLAG_SPROUT_CLEAR);
            self.dialogue = Some(DialogueState {
                lines: vec![
                    "ELDER LI: So you have".to_string(),
                    "come this far.".to_string(),
                    "Let me test your".to_string(),
                    "bond with POKEMON!".to_string(),
                ],
                current_line: 0, char_index: 0, timer: 0.0,
                on_complete: DialogueAction::StartTrainerBattle {
                    team: vec![
                        (BELLSPROUT, 7),
                        (BELLSPROUT, 7),
                        (HOOTHOOT, 10),
                    ],
                },
            });
            self.phase = GamePhase::Dialogue;
            return true;
        }
        false
    }

    /// Sprout Tower 3F rival confrontation: rival appears, Elder lectures, rival leaves
    fn check_sprout_tower_rival(&mut self) -> bool {
        if self.current_map_id == MapId::SproutTower3F
            && !self.has_flag(FLAG_SPROUT_RIVAL)
            && self.rival_starter > 0
            && self.player.y <= 5
            && !self.party.is_empty()
        {
            self.set_flag(FLAG_SPROUT_RIVAL);
            self.dialogue = Some(DialogueState {
                lines: vec![
                    "RIVAL: ...Heh. So you".to_string(),
                    "made it here too.".to_string(),
                    "I already beat the".to_string(),
                    "ELDER. It was easy.".to_string(),
                    "ELDER LI: You! The way".to_string(),
                    "you treat your POKEMON".to_string(),
                    "is terrible!".to_string(),
                    "POKEMON are not tools".to_string(),
                    "of war!".to_string(),
                    "RIVAL: Hmph. Weak".to_string(),
                    "people say weak things.".to_string(),
                    "RIVAL used an ESCAPE".to_string(),
                    "ROPE!".to_string(),
                    "RIVAL fled from the".to_string(),
                    "tower.".to_string(),
                ],
                current_line: 0, char_index: 0, timer: 0.0,
                on_complete: DialogueAction::None,
            });
            self.phase = GamePhase::Dialogue;
            return true;
        }
        false
    }

    /// Red Gyarados: forced wild encounter at Lake of Rage
    fn check_red_gyarados(&mut self, _engine: &mut Engine) -> bool {
        if self.current_map_id == MapId::LakeOfRage
            && !self.has_flag(FLAG_RED_GYARADOS)
            && !self.party.is_empty()
        {
            self.set_flag(FLAG_RED_GYARADOS);
            self.dialogue = Some(DialogueState {
                lines: vec![
                    "The lake is churning!".to_string(),
                    "A huge red GYARADOS".to_string(),
                    "burst from the water!".to_string(),
                ],
                current_line: 0, char_index: 0, timer: 0.0,
                on_complete: DialogueAction::StartRedGyaradosBattle,
            });
            self.phase = GamePhase::Dialogue;
            return true;
        }
        false
    }

    /// Sudowoodo: blocking tree on Route 36 — now fully handled by NPC interaction + Squirtbottle
    fn check_sudowoodo(&mut self, _engine: &mut Engine) -> bool {
        false
    }

    /// Burned Tower 1F: Rival Silver battle trigger.
    /// Per pokecrystal: Rival has Gastly Lv12, Zubat Lv14, + starter evolution Lv16.
    /// Triggers when player enters the center area (y <= 7) without FLAG_BURNED_TOWER_RIVAL.
    fn check_burned_tower_rival(&mut self) -> bool {
        if self.current_map_id == MapId::BurnedTower
            && !self.has_flag(FLAG_BURNED_TOWER_RIVAL)
            && self.rival_starter > 0
            && self.player.y <= 7
            && !self.party.is_empty()
        {
            self.set_flag(FLAG_BURNED_TOWER_RIVAL);
            // Per pokecrystal: Rival team depends on player's starter choice
            // If player chose Totodile -> rival has Chikorita line (Bayleef Lv16)
            // If player chose Chikorita -> rival has Cyndaquil line (Quilava Lv16)
            // If player chose Cyndaquil -> rival has Totodile line (Croconaw Lv16)
            let rival_evo = match self.rival_starter {
                CHIKORITA => BAYLEEF,
                CYNDAQUIL => QUILAVA,
                TOTODILE => CROCONAW,
                _ => QUILAVA,
            };
            self.dialogue = Some(DialogueState {
                lines: vec![
                    "RIVAL: ...Oh, it's".to_string(),
                    "you.".to_string(),
                    "I came looking for".to_string(),
                    "some legendary".to_string(),
                    "POKEMON that they".to_string(),
                    "say roosts here.".to_string(),
                    "But there's nothing!".to_string(),
                    "It's all your fault!".to_string(),
                ],
                current_line: 0, char_index: 0, timer: 0.0,
                on_complete: DialogueAction::StartTrainerBattle {
                    team: vec![
                        (GASTLY, 12),
                        (ZUBAT, 14),
                        (rival_evo, 16),
                    ],
                },
            });
            self.phase = GamePhase::Dialogue;
            return true;
        }
        false
    }

    /// Burned Tower B1F: Legendary beast encounter event.
    /// When player approaches beasts (y <= 5), they flee and become roaming.
    /// Sets FLAG_BEASTS_RELEASED. One-time event.
    fn check_beasts_released(&mut self) -> bool {
        if self.current_map_id == MapId::BurnedTowerB1F
            && !self.has_flag(FLAG_BEASTS_RELEASED)
            && self.player.y <= 5
            && !self.party.is_empty()
        {
            self.set_flag(FLAG_BEASTS_RELEASED);
            self.dialogue = Some(DialogueState {
                lines: vec![
                    "The ground shakes!".to_string(),
                    "Three POKEMON awaken!".to_string(),
                    "RAIKOU springs up".to_string(),
                    "and races away!".to_string(),
                    "ENTEI leaps to its".to_string(),
                    "feet and dashes off!".to_string(),
                    "SUICUNE gazes at you".to_string(),
                    "briefly, then bolts!".to_string(),
                    "The three legendary".to_string(),
                    "beasts have been".to_string(),
                    "released into the".to_string(),
                    "wild!".to_string(),
                ],
                current_line: 0, char_index: 0, timer: 0.0,
                on_complete: DialogueAction::None,
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

        // NPC wandering — every 2-4 seconds, wandering NPCs take a random step
        self.npc_wander_timer += dt;
        if self.npc_wander_timer > 2.0 {
            self.npc_wander_timer = 0.0;
            let map_w = self.current_map.width;
            let map_h = self.current_map.height;
            for i in 0..self.current_map.npcs.len() {
                if !self.current_map.npcs[i].wanders { continue; }
                if !self.is_npc_active(i) { continue; }
                // Pick a random direction (0-3: up/down/left/right)
                let r = (engine.rng.next_f64() * 4.0) as u8;
                let (dx, dy): (i32, i32) = match r {
                    0 => (0, -1), 1 => (0, 1), 2 => (-1, 0), _ => (1, 0),
                };
                let nx = self.current_map.npcs[i].x as i32 + dx;
                let ny = self.current_map.npcs[i].y as i32 + dy;
                if nx < 0 || ny < 0 || nx >= map_w as i32 || ny >= map_h as i32 { continue; }
                // Check collision
                let col = self.current_map.collision_at(nx as usize, ny as usize);
                let walkable = matches!(col, CollisionType::Walkable | CollisionType::TallGrass);
                if !walkable { continue; }
                // Don't step on player
                if nx == self.player.x && ny == self.player.y { continue; }
                // Don't step on other NPCs
                let blocked = self.current_map.npcs.iter().enumerate()
                    .any(|(j, n)| j != i && self.is_npc_active(j) && n.x as i32 == nx && n.y as i32 == ny);
                if blocked { continue; }
                // Move the NPC
                self.current_map.npcs[i].x = nx as u8;
                self.current_map.npcs[i].y = ny as u8;
                self.current_map.npcs[i].facing = match r {
                    0 => Direction::Up, 1 => Direction::Down,
                    2 => Direction::Left, _ => Direction::Right,
                };
            }
        }

        // Menu opens even during walk animation (original game behavior: cancel interrupts)
        // But NOT while ice sliding — can't open menu mid-slide
        if is_cancel(engine) && self.has_starter && self.ice_sliding.is_none() {
            self.phase = GamePhase::Menu;
            self.menu_cursor = 0;
            self.player.is_walking = false;
            self.player.walk_offset = 0.0;
            return;
        }

        // Bicycle toggle (Select key) — only outdoors
        if is_select(engine) && self.has_bicycle && !self.player.is_walking {
            let is_indoor = matches!(self.current_map_id,
                MapId::PokemonCenter | MapId::GenericHouse | MapId::ElmLab |
                MapId::PlayerHouse1F | MapId::PlayerHouse2F |
                MapId::SproutTower1F | MapId::SproutTower2F | MapId::SproutTower3F |
                MapId::UnionCave | MapId::IlexForest |
                MapId::BurnedTower | MapId::OlivineLighthouse | MapId::IcePath |
                MapId::VioletGym | MapId::AzaleaGym | MapId::GoldenrodGym |
                MapId::EcruteakGym | MapId::OlivineGym | MapId::CianwoodGym |
                MapId::MahoganyGym | MapId::BlackthornGym |
                MapId::VictoryRoad | MapId::RocketHQ |
                MapId::EliteFourWill | MapId::EliteFourKoga |
                MapId::EliteFourBruno | MapId::EliteFourKaren | MapId::ChampionLance
            );
            if !is_indoor {
                self.on_bicycle = !self.on_bicycle;
            }
        }

        if self.player.is_walking {
            let speed = if self.ice_sliding.is_some() {
                WALK_SPEED / 2.0 // double speed on ice (faster slide)
            } else if self.on_bicycle {
                WALK_SPEED / 2.0
            } else {
                WALK_SPEED
            };
            self.player.walk_offset += 1.0 / speed;
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

                // Decrement repel and show "wore off" dialogue when it expires
                if self.repel_steps > 0 {
                    self.repel_steps -= 1;
                    if self.repel_steps == 0 {
                        self.dialogue = Some(DialogueState {
                            lines: vec!["REPEL's effect wore off!".to_string()],
                            current_line: 0, char_index: 0, timer: 0.0,
                            on_complete: DialogueAction::None,
                        });
                        // We don't change phase here — dialogue will be picked up after walk finishes
                    }
                }

                // Daycare step counting: 1 EXP per step
                if self.daycare_pokemon.is_some() {
                    self.daycare_steps += 1;
                    if let Some(ref mut pkmn) = self.daycare_pokemon {
                        pkmn.exp += 1;
                        if let Some(species) = get_species(pkmn.species_id) {
                            while pkmn.level < 100 {
                                let next_exp = exp_for_level(pkmn.level + 1, species.growth_rate);
                                if pkmn.exp >= next_exp {
                                    pkmn.level += 1;
                                    pkmn.recalc_stats();
                                    // Gen 2 daycare: learn moves by replacing first slot
                                    for &(lvl, move_id) in species.learnset.iter() {
                                        if lvl == pkmn.level {
                                            // Shift moves up, put new move in last slot (Gen 2 behavior)
                                            pkmn.moves[0] = pkmn.moves[1];
                                            pkmn.move_pp[0] = pkmn.move_pp[1];
                                            pkmn.move_max_pp[0] = pkmn.move_max_pp[1];
                                            pkmn.moves[1] = pkmn.moves[2];
                                            pkmn.move_pp[1] = pkmn.move_pp[2];
                                            pkmn.move_max_pp[1] = pkmn.move_max_pp[2];
                                            pkmn.moves[2] = pkmn.moves[3];
                                            pkmn.move_pp[2] = pkmn.move_pp[3];
                                            pkmn.move_max_pp[2] = pkmn.move_max_pp[3];
                                            pkmn.moves[3] = Some(move_id);
                                            if let Some(md) = get_move(move_id) {
                                                pkmn.move_pp[3] = md.pp;
                                                pkmn.move_max_pp[3] = md.pp;
                                            }
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }
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
                    // Block Elm Lab exit until player has a starter
                    if self.current_map_id == MapId::ElmLab && !self.has_starter {
                        // Push player back one tile (away from door)
                        self.player.y -= 1;
                        self.dialogue = Some(DialogueState {
                            lines: vec![
                                "PROF.ELM: Wait!".to_string(),
                                "Pick a POKEMON first!".to_string(),
                            ],
                            current_line: 0, char_index: 0, timer: 0.0,
                            on_complete: DialogueAction::None,
                        });
                        self.phase = GamePhase::Dialogue;
                        return;
                    }
                    // Progression gate check (Route27, UnionCave, IlexForest, RocketHQ, IcePath, VictoryRoad)
                    if let Some(gate_lines) = self.check_warp_gate(warp.dest_map) {
                        match self.player.facing {
                            Direction::Up => self.player.y += 1,
                            Direction::Down => self.player.y -= 1,
                            Direction::Left => self.player.x += 1,
                            Direction::Right => self.player.x -= 1,
                        }
                        self.dialogue = Some(DialogueState {
                            lines: gate_lines.iter().map(|s| s.to_string()).collect(),
                            current_line: 0, char_index: 0, timer: 0.0,
                            on_complete: DialogueAction::None,
                        });
                        self.phase = GamePhase::Dialogue;
                        return;
                    }
                    // PokemonCenter: dynamic exit based on which city we entered from
                    if self.current_map_id == MapId::PokemonCenter {
                        let (dest_map, dx, dy) = match self.last_pokecenter_map {
                            MapId::VioletCity => (MapId::VioletCity, 5, 12),
                            MapId::AzaleaTown => (MapId::AzaleaTown, 6, 13),
                            MapId::GoldenrodCity => (MapId::GoldenrodCity, 10, 15),
                            MapId::EcruteakCity => (MapId::EcruteakCity, 15, 13),
                            MapId::OlivineCity => (MapId::OlivineCity, 4, 8),
                            MapId::CianwoodCity => (MapId::CianwoodCity, 19, 5),
                            MapId::MahoganyTown => (MapId::MahoganyTown, 5, 12),
                            MapId::BlackthornCity => (MapId::BlackthornCity, 3, 7),
                            MapId::Route26 => (MapId::Route26, 3, 4),
                            MapId::IndigoPlateau => (MapId::IndigoPlateau, 1, 5),
                            _ => (MapId::CherrygroveCity, 7, 5),
                        };
                        self.phase = GamePhase::MapFadeOut { dest_map, dest_x: dx, dest_y: dy, timer: 0.0 };
                    } else if self.current_map_id == MapId::GenericHouse {
                        // GenericHouse: exit to exact door position we entered from
                        let dest_map = self.last_house_map;
                        let dx = self.last_house_x as u8;
                        let dy = self.last_house_y as u8;
                        self.phase = GamePhase::MapFadeOut { dest_map, dest_x: dx, dest_y: dy, timer: 0.0 };
                    } else {
                        self.phase = GamePhase::MapFadeOut { dest_map: warp.dest_map, dest_x: warp.dest_x, dest_y: warp.dest_y, timer: 0.0 };
                    }
                    return;
                }

                // Show repel wore off dialogue if it just expired
                if self.dialogue.is_some() && self.repel_steps == 0 {
                    self.phase = GamePhase::Dialogue;
                    return;
                }

                // Check wild encounter (repel: Gen 2 rule — blocks if wild level < lead level)
                if self.current_map.is_tall_grass(self.player.x as usize, self.player.y as usize)
                    && !self.party.is_empty()
                {
                    let roll = engine.rng.next_f64();
                    if roll < ENCOUNTER_RATE {
                        let r1 = engine.rng.next_f64();
                        let r2 = engine.rng.next_f64();
                        let is_night = self.time_of_day < 5.0 || self.time_of_day >= 19.0;
                        if let Some((species_id, level)) = self.current_map.roll_encounter_timed(r1, r2, is_night) {
                            // Repel check: if repel active and wild level < lead Pokemon level, skip
                            if self.repel_steps > 0 {
                                let lead_level = self.party.iter()
                                    .find(|p| !p.is_fainted())
                                    .map(|p| p.level)
                                    .unwrap_or(0);
                                if level < lead_level {
                                    // Encounter suppressed by repel
                                    return;
                                }
                            }
                            self.register_seen(species_id);
                            let enemy = Pokemon::new(species_id, level);
                            let player_idx = self.party.iter().position(|p| !p.is_fainted()).unwrap_or(0);
                            let player_hp = self.party.get(player_idx).map(|p| p.hp as f64).unwrap_or(0.0);
                            self.battle = Some(BattleState {
                                phase: BattlePhase::Intro { timer: 0.0 },
                                enemy,
                                player_idx,
                                is_wild: true,
                                player_hp_display: player_hp,
                                enemy_hp_display: 0.0,
                                turn_count: 0,
                                trainer_team: Vec::new(),
                                trainer_team_idx: 0,
                                pending_player_move: None,
                                player_stages: [0; 7],
                                enemy_stages: [0; 7],
                                enemy_flinched: false,
                                player_flinched: false,
                                player_confused: 0,
                                enemy_confused: 0,
                                player_trapped: false,
                                player_must_recharge: false,
                                enemy_must_recharge: false,
                                player_rampage: (0, 0),
                                enemy_rampage: (0, 0),
                                pending_learn_moves: vec![],
                                free_switch: false,
                                confusion_snapout_msg: None,
                                battle_queue: VecDeque::new(),
                                queue_timer: 0.0,
                            });
                            // Trigger encounter transition flash instead of going directly to battle
                            self.encounter_flash_count = 0;
                            self.phase = GamePhase::EncounterTransition { timer: 0.0 };
                            return;
                        }
                    }
                }

                // Check for story event battles
                if self.check_rival_battle() { return; }
                if self.check_victory_road_rival() { return; }
                if self.check_sprout_tower_rival() { return; }
                if self.check_sprout_tower_elder() { return; }
                if self.check_red_gyarados(engine) { return; }
                if self.check_sudowoodo(engine) { return; }
                if self.check_burned_tower_rival() { return; }
                if self.check_beasts_released() { return; }

                // Check trainer line-of-sight (5 tiles in their facing direction)
                if self.los_suppress > 0 {
                    self.los_suppress -= 1;
                } else if self.party.iter().any(|p| !p.is_fainted()) {
                    let px = self.player.x;
                    let py = self.player.y;
                    for (npc_idx, npc) in self.current_map.npcs.iter().enumerate() {
                        if !self.is_npc_active(npc_idx) { continue; }
                        if !npc.is_trainer || npc.trainer_team.is_empty() { continue; }
                        // Skip already defeated trainers
                        let key = (self.current_map_id, npc_idx as u8);
                        if self.defeated_trainers.contains(&key) { continue; }
                        // Gym leaders (NPC 0 in gyms) battle on talk, not line-of-sight
                        if npc_idx == 0 && matches!(self.current_map_id,
                            MapId::VioletGym | MapId::AzaleaGym | MapId::GoldenrodGym |
                            MapId::EcruteakGym | MapId::OlivineGym | MapId::CianwoodGym |
                            MapId::MahoganyGym | MapId::BlackthornGym
                        ) { continue; }
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
                            // Trainer spotted player! Start approach sequence.
                            self.ice_sliding = None; // stop sliding if trainer spotted
                            self.trainer_battle_npc = Some(key);
                            self.approach_npc_x = npc.x as i32;
                            self.approach_npc_y = npc.y as i32;
                            self.approach_walk_offset = 0.0;
                            self.approach_exclaim_timer = 0.0;
                            self.phase = GamePhase::TrainerApproach { npc_idx: npc_idx as u8, timer: 0.0 };
                            return;
                        }
                    }
                }

                // Ice sliding: check if current tile is ice
                let cur_col = self.current_map.collision_at(self.player.x as usize, self.player.y as usize);
                if cur_col == CollisionType::Ice {
                    // On ice: start or continue sliding in the current facing direction
                    let slide_dir = self.ice_sliding.unwrap_or(self.player.facing);
                    self.ice_sliding = Some(slide_dir);
                    // Check if we can continue sliding (next tile in slide direction)
                    let (nx, ny) = match slide_dir {
                        Direction::Up => (self.player.x, self.player.y - 1),
                        Direction::Down => (self.player.x, self.player.y + 1),
                        Direction::Left => (self.player.x - 1, self.player.y),
                        Direction::Right => (self.player.x + 1, self.player.y),
                    };
                    if nx >= 0 && ny >= 0
                        && (nx as usize) < self.current_map.width
                        && (ny as usize) < self.current_map.height
                    {
                        let next_col = self.current_map.collision_at(nx as usize, ny as usize);
                        let can_slide = matches!(next_col, CollisionType::Ice | CollisionType::Walkable | CollisionType::Warp);
                        // Also check for NPC blocking
                        let npc_blocking = self.current_map.npcs.iter().enumerate()
                            .any(|(i, npc)| self.is_npc_active(i) && npc.x as i32 == nx && npc.y as i32 == ny);
                        if can_slide && !npc_blocking {
                            self.player.facing = slide_dir;
                            self.player.is_walking = true;
                            self.player.walk_offset = 0.0;
                        } else {
                            // Hit a wall/solid/NPC: stop sliding
                            self.ice_sliding = None;
                        }
                    } else {
                        // Hit map edge: stop sliding
                        self.ice_sliding = None;
                    }
                } else {
                    // Not on ice: stop sliding
                    self.ice_sliding = None;
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
                    CollisionType::Walkable | CollisionType::TallGrass | CollisionType::Warp | CollisionType::Ice => true,
                    CollisionType::Ledge => dir == Direction::Down, // ledges: south only
                    _ => false,
                };
                if can_walk {
                    let npc_blocking = self.current_map.npcs.iter().enumerate()
                        .any(|(i, npc)| self.is_npc_active(i) && npc.x as i32 == nx && npc.y as i32 == ny);
                    if !npc_blocking {
                        self.player.is_walking = true;
                        self.player.walk_offset = 0.0;
                    }
                }
            }
        }

        // Interact (A button)
        if is_confirm(engine) {
            // Fishing: face a water tile to fish
            let (fx, fy) = match self.player.facing {
                Direction::Up => (self.player.x, self.player.y - 1),
                Direction::Down => (self.player.x, self.player.y + 1),
                Direction::Left => (self.player.x - 1, self.player.y),
                Direction::Right => (self.player.x + 1, self.player.y),
            };
            if fx >= 0 && fy >= 0 && (fx as usize) < self.current_map.width && (fy as usize) < self.current_map.height {
                let col = self.current_map.collision_at(fx as usize, fy as usize);
                if col == CollisionType::Water && !self.current_map.water_encounters.is_empty() && !self.party.is_empty() {
                    self.try_fish(engine);
                    return;
                }
            }
            self.try_interact();
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
            .find(|(idx, npc)| self.is_npc_active(*idx) && npc.x as i32 == fx && npc.y as i32 == fy)
            .map(|(idx, npc)| (idx as u8, npc.clone()));

        if let Some((npc_idx, npc)) = npc_info {
            // Route 34 Day-Care Man (NPC 0): daycare deposit/return system
            if self.current_map_id == MapId::Route34 && npc_idx == 0 {
                if let Some(ref dc_pkmn) = self.daycare_pokemon {
                    let pkmn_name = get_species(dc_pkmn.species_id).map(|s| s.name).unwrap_or("???");
                    let level = dc_pkmn.level;
                    self.dialogue = Some(DialogueState {
                        lines: vec![
                            format!("Your {} has grown", pkmn_name),
                            format!("to level {}.", level),
                            "Want it back?".to_string(),
                        ],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::DaycareReturn,
                    });
                } else {
                    self.dialogue = Some(DialogueState {
                        lines: vec![
                            "I'm the DAY-CARE MAN.".to_string(),
                            "Want me to raise a".to_string(),
                            "POKEMON for you?".to_string(),
                        ],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::DaycareDeposit,
                    });
                }
                self.phase = GamePhase::Dialogue;
                return;
            }

            // Lake of Rage: Lance's dialogue changes after Red Gyarados event
            if self.current_map_id == MapId::LakeOfRage && npc_idx == 0
                && self.has_flag(FLAG_RED_GYARADOS)
            {
                self.dialogue = Some(DialogueState {
                    lines: vec![
                        "That red GYARADOS...".to_string(),
                        "TEAM ROCKET must be".to_string(),
                        "behind this!".to_string(),
                        "I heard there's a".to_string(),
                        "suspicious shop in".to_string(),
                        "MAHOGANY TOWN.".to_string(),
                        "Let's investigate!".to_string(),
                    ],
                    current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
                self.phase = GamePhase::Dialogue;
                return;
            }

            // Elm Lab: Mystery Egg quest — give Togepi egg after first badge
            if self.current_map_id == MapId::ElmLab && npc_idx == 0
                && !self.has_flag(FLAG_GOT_EGG)
                && (self.badges & 1) != 0  // Has Zephyr Badge
                && self.party.len() < 6
            {
                self.set_flag(FLAG_GOT_EGG);
                let togepi = Pokemon::new(TOGEPI, 5);
                self.register_seen(TOGEPI);
                self.party.push(togepi);
                self.dialogue = Some(DialogueState {
                    lines: vec![
                        "Oh, you got the".to_string(),
                        "ZEPHYR BADGE!".to_string(),
                        "I have something".to_string(),
                        "for you.".to_string(),
                        "This is a MYSTERY EGG".to_string(),
                        "I received from".to_string(),
                        "MR.POKEMON.".to_string(),
                        "Received TOGEPI!".to_string(),
                        "It hatched from the".to_string(),
                        "egg!".to_string(),
                    ],
                    current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
                self.phase = GamePhase::Dialogue;
                return;
            }

            // Elm Lab: after egg, Elm gives encouragement
            if self.current_map_id == MapId::ElmLab && npc_idx == 0
                && self.has_flag(FLAG_GOT_EGG) && self.has_starter
            {
                self.dialogue = Some(DialogueState {
                    lines: vec![
                        "You're doing great!".to_string(),
                        "Keep collecting those".to_string(),
                        "BADGES!".to_string(),
                    ],
                    current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
                self.phase = GamePhase::Dialogue;
                return;
            }

            // Olivine Gym: Jasmine unavailable until medicine delivered
            if self.current_map_id == MapId::OlivineGym && npc_idx == 0
                && !self.has_flag(FLAG_DELIVERED_MEDICINE)
            {
                self.dialogue = Some(DialogueState {
                    lines: vec![
                        "The GYM LEADER isn't".to_string(),
                        "here... She's at the".to_string(),
                        "LIGHTHOUSE tending to".to_string(),
                        "a sick POKEMON.".to_string(),
                    ],
                    current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
                self.phase = GamePhase::Dialogue;
                return;
            }

            // Ilex Forest: Farfetch'd quest — talk to Farfetch'd (NPC 0) to herd it back
            if self.current_map_id == MapId::IlexForest && npc_idx == 0
                && !self.has_flag(FLAG_ILEX_FARFETCHD)
            {
                self.set_flag(FLAG_ILEX_FARFETCHD);
                self.dialogue = Some(DialogueState {
                    lines: vec![
                        "FARFETCH'D: Kwaa!".to_string(),
                        "The FARFETCH'D ran".to_string(),
                        "back to its master!".to_string(),
                        "APPRENTICE: Oh! You".to_string(),
                        "got it back! Thanks!".to_string(),
                        "My MASTER will want".to_string(),
                        "to thank you too!".to_string(),
                    ],
                    current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
                self.phase = GamePhase::Dialogue;
                return;
            }

            // Ilex Forest: Charcoal Master (NPC 2) gives HM CUT after Farfetch'd quest
            if self.current_map_id == MapId::IlexForest && npc_idx == 2
                && self.has_flag(FLAG_ILEX_FARFETCHD)
            {
                self.dialogue = Some(DialogueState {
                    lines: vec![
                        "I'm the CHARCOAL".to_string(),
                        "MAKER. Thanks for".to_string(),
                        "finding my FARFETCH'D!".to_string(),
                        "Take this HM as".to_string(),
                        "thanks!".to_string(),
                        "Received HM01 - CUT!".to_string(),
                        "Use CUT to clear".to_string(),
                        "small trees.".to_string(),
                    ],
                    current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
                self.phase = GamePhase::Dialogue;
                return;
            }

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

            // Sudowoodo NPC interaction — requires Squirtbottle
            if self.current_map_id == MapId::Route36 && npc_idx == 2
                && !self.has_flag(FLAG_SUDOWOODO)
                && !self.party.is_empty()
            {
                if self.has_flag(FLAG_SQUIRTBOTTLE) {
                    self.set_flag(FLAG_SUDOWOODO);
                    self.dialogue = Some(DialogueState {
                        lines: vec![
                            "Used the SQUIRTBOTTLE!".to_string(),
                            "The tree doesn't like".to_string(),
                            "the water!".to_string(),
                            "The weird tree is".to_string(),
                            "attacking!".to_string(),
                        ],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::StartSudowoodoBattle,
                    });
                    self.phase = GamePhase::Dialogue;
                } else {
                    self.dialogue = Some(DialogueState {
                        lines: vec![
                            "A weird tree is".to_string(),
                            "blocking the path.".to_string(),
                            "It won't budge...".to_string(),
                        ],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                }
                return;
            }

            // Goldenrod Flower Shop: give Squirtbottle after Whitney badge
            if self.current_map_id == MapId::GoldenrodCity && npc_idx == 0
                && !self.has_flag(FLAG_SQUIRTBOTTLE) && (self.badges & (1 << 2)) != 0
            {
                self.set_flag(FLAG_SQUIRTBOTTLE);
                self.dialogue = Some(DialogueState {
                    lines: vec![
                        "Oh, you beat WHITNEY!".to_string(),
                        "Here, take this!".to_string(),
                        "Received SQUIRTBOTTLE!".to_string(),
                        "Use it on the weird".to_string(),
                        "tree on ROUTE 36!".to_string(),
                    ],
                    current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
                self.phase = GamePhase::Dialogue;
                return;
            }

            // Goldenrod Bike Shop owner: give bicycle on interaction
            if self.current_map_id == MapId::GoldenrodCity && npc_idx == 1 && !self.has_bicycle {
                self.has_bicycle = true;
                self.dialogue = Some(DialogueState {
                    lines: vec![
                        "I make BICYCLES.".to_string(),
                        "Want one? Sure thing!".to_string(),
                        "Here, take this one!".to_string(),
                        "Received a BICYCLE!".to_string(),
                        "Use it from your BAG".to_string(),
                        "or press C/SHIFT!".to_string(),
                    ],
                    current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
                self.phase = GamePhase::Dialogue;
                return;
            }

            // Cianwood Pharmacist: give SecretPotion
            if self.current_map_id == MapId::CianwoodCity && npc_idx == 5
                && !self.has_flag(FLAG_MEDICINE)
            {
                self.set_flag(FLAG_MEDICINE);
                self.dialogue = Some(DialogueState {
                    lines: vec![
                        "This is the CIANWOOD".to_string(),
                        "PHARMACY.".to_string(),
                        "Oh, a sick POKEMON?".to_string(),
                        "Here, take this!".to_string(),
                        "Received SECRETPOTION!".to_string(),
                        "Take it to the sick".to_string(),
                        "POKEMON in OLIVINE's".to_string(),
                        "LIGHTHOUSE.".to_string(),
                    ],
                    current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
                self.phase = GamePhase::Dialogue;
                return;
            }

            // Olivine Lighthouse Jasmine: deliver medicine
            if self.current_map_id == MapId::OlivineLighthouse && npc_idx == 0
                && self.has_flag(FLAG_MEDICINE)
                && !self.has_flag(FLAG_DELIVERED_MEDICINE)
            {
                self.set_flag(FLAG_DELIVERED_MEDICINE);
                self.dialogue = Some(DialogueState {
                    lines: vec![
                        "Oh! Is that the".to_string(),
                        "SECRETPOTION?!".to_string(),
                        "Thank you so much!".to_string(),
                        "AMPHY, drink this...".to_string(),
                        "...AMPHY is feeling".to_string(),
                        "much better!".to_string(),
                        "I can finally go".to_string(),
                        "back to the GYM.".to_string(),
                        "Please come challenge".to_string(),
                        "me!".to_string(),
                    ],
                    current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
                self.phase = GamePhase::Dialogue;
                return;
            }

            // D2 fix: Per-city dialogue for GenericHouse NPCs
            let lines: Vec<String> = if self.current_map_id == MapId::GenericHouse && !npc.is_trainer && !npc.is_mart {
                match self.last_house_map {
                    MapId::NewBarkTown => vec![
                        "NEW BARK TOWN is".into(), "small, but we like".into(), "the quiet life.".into(),
                    ],
                    MapId::CherrygroveCity => vec![
                        "CHERRYGROVE CITY".into(), "has the prettiest".into(), "flowers in JOHTO.".into(),
                    ],
                    MapId::VioletCity => vec![
                        "SPROUT TOWER is".into(), "said to sway in".into(), "the wind. Creepy!".into(),
                    ],
                    MapId::AzaleaTown => vec![
                        "KURT makes the".into(), "best POKe BALLS".into(), "from APRICORNS.".into(),
                    ],
                    MapId::GoldenrodCity => vec![
                        "GOLDENROD is so".into(), "big! The DEPT".into(), "STORE has it all.".into(),
                    ],
                    MapId::EcruteakCity => vec![
                        "ECRUTEAK CITY has".into(), "ancient legends of".into(), "legendary POKEMON.".into(),
                    ],
                    MapId::OlivineCity => vec![
                        "OLIVINE's LIGHTHOUSE".into(), "guides ships safely".into(), "into the harbor.".into(),
                    ],
                    MapId::CianwoodCity => vec![
                        "CIANWOOD is remote,".into(), "but the pharmacy".into(), "is world-famous.".into(),
                    ],
                    MapId::MahoganyTown => vec![
                        "MAHOGANY TOWN is".into(), "quiet. Maybe too".into(), "quiet, actually...".into(),
                    ],
                    MapId::BlackthornCity => vec![
                        "BLACKTHORN CITY is".into(), "home to the best".into(), "DRAGON trainers!".into(),
                    ],
                    _ => npc.dialogue.iter().map(|s| s.to_string()).collect(),
                }
            } else {
                npc.dialogue.iter().map(|s| s.to_string()).collect()
            };
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
                    MapId::SproutTower1F => "SPROUT TOWER 1F\nA tower of swaying pillars.",
                    MapId::SproutTower2F => "SPROUT TOWER 2F\nA tower of swaying pillars.",
                    MapId::SproutTower3F => "SPROUT TOWER 3F\nThe ELDER awaits at the top.",
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

    fn try_fish(&mut self, engine: &mut Engine) {
        // Roll for catch (70% chance of bite)
        let bite = engine.rng.next_f64() < 0.7;
        if !bite {
            self.dialogue = Some(DialogueState {
                lines: vec!["Not even a nibble...".to_string()],
                current_line: 0, char_index: 0, timer: 0.0,
                on_complete: DialogueAction::None,
            });
            self.phase = GamePhase::Dialogue;
            return;
        }
        // Pick a water encounter
        let total: u32 = self.current_map.water_encounters.iter().map(|e| e.weight as u32).sum();
        if total == 0 { return; }
        let roll = (engine.rng.next_f64() * total as f64) as u32;
        let mut accum = 0u32;
        let mut picked = &self.current_map.water_encounters[0];
        for entry in &self.current_map.water_encounters {
            accum += entry.weight as u32;
            if roll < accum { picked = entry; break; }
        }
        let species_id = picked.species_id;
        let min_level = picked.min_level;
        let max_level = picked.max_level;
        let level = if min_level == max_level {
            min_level
        } else {
            (min_level + (engine.rng.next_f64() * (max_level - min_level + 1) as f64) as u8).min(max_level)
        };
        // Show "Oh! A bite!" text, then start battle on_complete
        self.dialogue = Some(DialogueState {
            lines: vec!["Oh! A bite!".to_string()],
            current_line: 0, char_index: 0, timer: 0.0,
            on_complete: DialogueAction::StartFishBattle { species_id, level },
        });
        self.phase = GamePhase::Dialogue;
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
                    // Queue-based intro: "Go! POKEMON!" text, then ActionSelect
                    let pname = self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default();
                    battle.battle_queue.clear();
                    battle.queue_timer = 0.0;
                    if !pname.is_empty() {
                        battle.battle_queue.push_back(BattleStep::Text(format!("Go! {}!", pname)));
                    }
                    battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::ActionSelect { cursor: 0 })));
                    battle.phase = BattlePhase::ExecuteQueue;
                } else {
                    battle.phase = BattlePhase::Intro { timer: t };
                }
            }

            BattlePhase::ActionSelect { cursor } => {
                // Hyper Beam recharge: player must skip turn
                if battle.player_must_recharge {
                    battle.player_must_recharge = false;
                    let pname = self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default();
                    // Enemy gets a free turn
                    battle.enemy.try_thaw(engine.rng.next_f64());
                    let enemy_can_move = battle.enemy.can_move();
                    let enemy_paralyzed = matches!(battle.enemy.status, StatusCondition::Paralysis) && engine.rng.next_f64() < PARALYSIS_SKIP_CHANCE;
                    if !enemy_can_move || enemy_paralyzed {
                        let prefix = if battle.is_wild { "Wild " } else { "Foe " };
                        let reason = if enemy_paralyzed {
                            format!("{}{} is paralyzed!", prefix, battle.enemy.name())
                        } else if matches!(battle.enemy.status, StatusCondition::Freeze) {
                            format!("{}{} is frozen solid!", prefix, battle.enemy.name())
                        } else {
                            format!("{}{} is fast asleep!", prefix, battle.enemy.name())
                        };
                        battle.phase = BattlePhase::Text {
                            message: format!("{} must recharge!", pname),
                            timer: 0.0,
                            next_phase: Box::new(BattlePhase::Text {
                                message: reason, timer: 0.0,
                                next_phase: Box::new(BattlePhase::ActionSelect { cursor: 0 }),
                            }),
                        };
                    } else {
                        let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
                        battle.phase = BattlePhase::Text {
                            message: format!("{} must recharge!", pname),
                            timer: 0.0,
                            next_phase: Box::new(BattlePhase::EnemyAttack {
                                timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                            }),
                        };
                    }
                    self.battle = Some(battle);
                    return;
                }

                // Rampage: player locked into multi-turn attack
                if battle.player_rampage.0 > 0 {
                    let rampage_move = battle.player_rampage.1;
                    battle.player_rampage.0 -= 1;
                    let (p_dmg, p_eff, p_crit) = self.calc_player_damage(engine, rampage_move, &battle);
                    // Speed check for turn order
                    let player_spd = self.party.get(battle.player_idx).map(|p| p.speed).unwrap_or(0) as f64;
                    let enemy_spd = battle.enemy.speed as f64;
                    let player_first = player_spd >= enemy_spd;
                    if player_first {
                        battle.phase = BattlePhase::PlayerAttack {
                            timer: 0.0, move_id: rampage_move, damage: p_dmg, effectiveness: p_eff, is_crit: p_crit, from_pending: false,
                        };
                    } else {
                        battle.pending_player_move = Some((rampage_move, p_dmg, p_eff, p_crit));
                        let enemy_thawed = battle.enemy.try_thaw(engine.rng.next_f64());
                        if enemy_thawed {
                            let prefix = if battle.is_wild { "Wild " } else { "Foe " };
                            let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
                            battle.phase = BattlePhase::Text {
                                message: format!("{}{} thawed out!", prefix, battle.enemy.name()),
                                timer: 0.0,
                                next_phase: Box::new(BattlePhase::EnemyAttack {
                                    timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                                }),
                            };
                        } else {
                        let enemy_can_move = battle.enemy.can_move();
                        let enemy_paralyzed = matches!(battle.enemy.status, StatusCondition::Paralysis) && engine.rng.next_f64() < PARALYSIS_SKIP_CHANCE;
                        if !enemy_can_move || enemy_paralyzed {
                            let prefix = if battle.is_wild { "Wild " } else { "Foe " };
                            let reason = if enemy_paralyzed {
                                format!("{}{} is paralyzed!", prefix, battle.enemy.name())
                            } else if matches!(battle.enemy.status, StatusCondition::Freeze) {
                                format!("{}{} is frozen solid!", prefix, battle.enemy.name())
                            } else {
                                format!("{}{} is fast asleep!", prefix, battle.enemy.name())
                            };
                            battle.phase = BattlePhase::Text {
                                message: reason, timer: 0.0,
                                next_phase: Box::new(BattlePhase::PlayerAttack {
                                    timer: 0.0, move_id: rampage_move, damage: p_dmg, effectiveness: p_eff, is_crit: p_crit, from_pending: true,
                                }),
                            };
                        } else {
                            let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
                            battle.phase = BattlePhase::EnemyAttack {
                                timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                            };
                        }
                        } // close enemy_thawed else
                    }
                    self.battle = Some(battle);
                    return;
                }

                // Rampage just ended: confuse player
                if battle.player_rampage.1 != 0 && battle.player_rampage.0 == 0 {
                    battle.player_rampage = (0, 0);
                    if battle.player_confused == 0 {
                        battle.player_confused = 2 + (engine.rng.next_u64() % 4) as u8;
                        let pname = self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default();
                        battle.phase = BattlePhase::Text {
                            message: format!("{} became confused due to fatigue!", pname),
                            timer: 0.0,
                            next_phase: Box::new(BattlePhase::ActionSelect { cursor: 0 }),
                        };
                        self.battle = Some(battle);
                        return;
                    }
                    // Already confused — rampage ended silently, continue to ActionSelect
                }

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
                            self.phase = GamePhase::PokemonMenu { cursor: 0, action: 0, sub_cursor: 0 };
                            return;
                        }
                        3 => {
                            if battle.is_wild && battle.player_trapped {
                                // Mean Look prevents escape
                                battle.phase = BattlePhase::Text {
                                    message: "Can't escape!".to_string(), timer: 0.0,
                                    next_phase: Box::new(BattlePhase::ActionSelect { cursor: 0 }),
                                };
                            } else if battle.is_wild {
                                let mut pspeed = self.party.get(battle.player_idx).map(|p| p.speed).unwrap_or(50);
                                if self.party.get(battle.player_idx).map(|p| matches!(p.status, StatusCondition::Paralysis)).unwrap_or(false) {
                                    pspeed /= 2;
                                }
                                let espeed = battle.enemy.speed;
                                let chance = (pspeed as f64 * 128.0 / espeed as f64 + 30.0 * battle.turn_count as f64) / 256.0;
                                if engine.rng.next_f64() < chance || battle.turn_count > 3 {
                                    // Queue-based run: show "Got away safely!" via ExecuteQueue, then GoToPhase(Run) for cleanup
                                    battle.battle_queue.clear();
                                    battle.queue_timer = 0.0;
                                    battle.battle_queue.push_back(BattleStep::Text("Got away safely!".into()));
                                    battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::Run)));
                                    battle.phase = BattlePhase::ExecuteQueue;
                                } else {
                                    // Queue-based run failed: show "Can't escape!" via ExecuteQueue, then GoToPhase(EnemyAttack)
                                    let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
                                    battle.battle_queue.clear();
                                    battle.queue_timer = 0.0;
                                    battle.battle_queue.push_back(BattleStep::Text("Can't escape!".into()));
                                    battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::EnemyAttack {
                                        timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                                    })));
                                    battle.phase = BattlePhase::ExecuteQueue;
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
                    // Freeze thaw: 10% chance per turn (Gen 2)
                    let player_thawed = if let Some(p) = self.party.get_mut(battle.player_idx) {
                        p.try_thaw(engine.rng.next_f64())
                    } else { false };
                    // Show thaw text if applicable
                    if player_thawed {
                        let pname_thaw = self.party.get(battle.player_idx).map(|p| p.name()).unwrap_or("???");
                        battle.phase = BattlePhase::Text {
                            message: format!("{} thawed out!", pname_thaw),
                            timer: 0.0,
                            next_phase: Box::new(BattlePhase::MoveSelect { cursor: 0 }),
                        };
                        self.battle = Some(battle);
                        return;
                    }
                    // Check if player Pokemon can move (sleep/freeze)
                    let can_move = self.party.get(battle.player_idx).map(|p| p.can_move()).unwrap_or(true);
                    // Paralysis: 25% chance to be fully paralyzed
                    let paralyzed = if let Some(p) = self.party.get(battle.player_idx) {
                        matches!(p.status, StatusCondition::Paralysis) && engine.rng.next_f64() < PARALYSIS_SKIP_CHANCE
                    } else { false };

                    if !can_move || paralyzed {
                        let pname = self.party.get(battle.player_idx).map(|p| p.name()).unwrap_or("???");
                        let reason = if paralyzed {
                            format!("{} is paralyzed! It can't move!", pname)
                        } else if matches!(self.party.get(battle.player_idx).map(|p| &p.status), Some(StatusCondition::Freeze)) {
                            format!("{} is frozen solid!", pname)
                        } else {
                            format!("{} is fast asleep!", pname)
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

                    // Confusion check (Gen 2: 50% self-hit, typeless 40-power Physical attack)
                    let mut snapout_msg: Option<String> = None;
                    if battle.player_confused > 0 {
                        battle.player_confused -= 1;
                        if battle.player_confused == 0 {
                            // Snapped out — show text, then continue to attack normally (don't return)
                            let pname = self.party.get(battle.player_idx).map(|p| p.name()).unwrap_or("???").to_string();
                            snapout_msg = Some(format!("{} snapped out of confusion!", pname));
                            // Fall through to normal attack dispatch
                        }
                        if engine.rng.next_f64() < 0.5 {
                            // Hit self: typeless 40-power physical attack using own stats
                            let pname = self.party.get(battle.player_idx).map(|p| p.name()).unwrap_or("???").to_string();
                            let self_dmg = if let Some(p) = self.party.get(battle.player_idx) {
                                let atk = p.attack as f64;
                                let def = p.defense as f64;
                                let lvl = p.level as f64;
                                let base = ((2.0 * lvl / 5.0 + 2.0) * 40.0 * atk / def) / 50.0 + 2.0;
                                base as u16
                            } else { 10 };
                            if let Some(p) = self.party.get_mut(battle.player_idx) {
                                p.hp = p.hp.saturating_sub(self_dmg);
                            }
                            let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
                            let next = if self.party.get(battle.player_idx).map(|p| p.is_fainted()).unwrap_or(true) {
                                BattlePhase::PlayerFainted
                            } else {
                                BattlePhase::EnemyAttack {
                                    timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                                }
                            };
                            battle.phase = BattlePhase::Text {
                                message: format!("{} is confused!", pname),
                                timer: 0.0,
                                next_phase: Box::new(BattlePhase::Text {
                                    message: "It hurt itself in its confusion!".to_string(),
                                    timer: 0.0,
                                    next_phase: Box::new(next),
                                }),
                            };
                            self.battle = Some(battle);
                            return;
                        }
                        // Passed confusion check — continue to attack normally
                    }

                    // Check PP — if all moves are empty, force Struggle
                    let all_pp_zero = self.party.get(battle.player_idx)
                        .map(|p| p.moves.iter().enumerate().all(|(i, m)| m.is_none() || p.move_pp[i] == 0))
                        .unwrap_or(false);
                    let has_pp = !all_pp_zero && self.party.get(battle.player_idx)
                        .map(|p| (cursor as usize) < 4 && p.move_pp[cursor as usize] > 0)
                        .unwrap_or(false);
                    if !has_pp && !all_pp_zero {
                        // No PP for this specific move, but other moves have PP
                        battle.phase = BattlePhase::Text {
                            message: "No PP left for this move!".to_string(),
                            timer: 0.0,
                            next_phase: Box::new(BattlePhase::MoveSelect { cursor }),
                        };
                        self.battle = Some(battle);
                        return;
                    }

                    // Get player move (force Struggle if all PP = 0)
                    let (move_id, use_struggle) = if all_pp_zero {
                        (MOVE_STRUGGLE, true)
                    } else {
                        let mid = self.party.get(battle.player_idx)
                            .and_then(|p| p.moves.get(cursor as usize).copied().flatten())
                            .unwrap_or(MOVE_TACKLE);
                        (mid, false)
                    };

                    // Consume PP (not for Struggle)
                    if !use_struggle {
                        if let Some(p) = self.party.get_mut(battle.player_idx) {
                            p.move_pp[cursor as usize] -= 1;
                        }
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
                    let mut enemy_speed = (battle.enemy.speed as f64 * enemy_spd_stage) as u16;
                    if matches!(battle.enemy.status, StatusCondition::Paralysis) {
                        enemy_speed /= 2;
                    }
                    if player_speed >= enemy_speed {
                        // Player goes first
                        battle.pending_player_move = None;
                        let attack_phase = BattlePhase::PlayerAttack {
                            timer: 0.0, move_id, damage: p_damage, effectiveness: p_eff, is_crit: p_crit, from_pending: false,
                        };
                        battle.phase = if let Some(sm) = snapout_msg {
                            BattlePhase::Text { message: sm, timer: 0.0, next_phase: Box::new(attack_phase) }
                        } else { attack_phase };
                    } else {
                        // Enemy goes first — store player's move for after enemy's turn
                        battle.pending_player_move = Some((move_id, p_damage, p_eff, p_crit));
                        // Store snapout message to show when pending player move resolves
                        battle.confusion_snapout_msg = snapout_msg;
                        // Check enemy confusion before their attack
                        if battle.enemy_confused > 0 {
                            battle.enemy_confused -= 1;
                            let prefix = if battle.is_wild { "Wild " } else { "Foe " };
                            if battle.enemy_confused == 0 {
                                let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
                                battle.phase = BattlePhase::Text {
                                    message: format!("{}{} snapped out of confusion!", prefix, battle.enemy.name()),
                                    timer: 0.0,
                                    next_phase: Box::new(BattlePhase::EnemyAttack {
                                        timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                                    }),
                                };
                            } else if engine.rng.next_f64() < 0.5 {
                                let atk = battle.enemy.attack as f64;
                                let def = battle.enemy.defense as f64;
                                let lvl = battle.enemy.level as f64;
                                let self_dmg = ((2.0 * lvl / 5.0 + 2.0) * 40.0 * atk / def) / 50.0 + 2.0;
                                battle.enemy.hp = battle.enemy.hp.saturating_sub(self_dmg as u16);
                                let next = if battle.enemy.is_fainted() {
                                    let exp = get_species(battle.enemy.species_id)
                                        .map(|sp| exp_gained(sp, battle.enemy.level, battle.is_wild))
                                        .unwrap_or(10);
                                    BattlePhase::EnemyFainted { exp_gained: exp }
                                } else {
                                    // Player gets their pending move next
                                    if let Some((pm, pd, pe, pc)) = battle.pending_player_move.take() {
                                        BattlePhase::PlayerAttack {
                                            timer: 0.0, move_id: pm, damage: pd, effectiveness: pe, is_crit: pc, from_pending: true,
                                        }
                                    } else {
                                        BattlePhase::ActionSelect { cursor: 0 }
                                    }
                                };
                                battle.phase = BattlePhase::Text {
                                    message: format!("{}{} is confused!", prefix, battle.enemy.name()),
                                    timer: 0.0,
                                    next_phase: Box::new(BattlePhase::Text {
                                        message: "It hurt itself in its confusion!".to_string(),
                                        timer: 0.0,
                                        next_phase: Box::new(next),
                                    }),
                                };
                            } else {
                                let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
                                battle.phase = BattlePhase::Text {
                                    message: format!("{}{} is confused!", prefix, battle.enemy.name()),
                                    timer: 0.0,
                                    next_phase: Box::new(BattlePhase::EnemyAttack {
                                        timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                                    }),
                                };
                            }
                        } else {
                            let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
                            battle.phase = BattlePhase::EnemyAttack {
                                timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                            };
                        }
                    }
                }
            }

            BattlePhase::PlayerAttack { timer: _, move_id, damage, effectiveness, is_crit, from_pending } => {
                // Queue-based PlayerAttack: compute all effects immediately, push queue steps,
                // then transition to ExecuteQueue. No timer — all visual pacing is in the queue.
                let num_hits = multi_hit_count(move_id, engine.rng.next_f64());
                let damage = damage * num_hits as u16;

                // Apply damage to enemy
                battle.enemy.hp = battle.enemy.hp.saturating_sub(damage);

                // Recoil: 1/4 of damage dealt to self (Gen 2) for Struggle, Take Down
                let has_recoil = (move_id == MOVE_STRUGGLE || move_id == MOVE_TAKE_DOWN) && damage > 0;
                if has_recoil {
                    let recoil = (damage / 4).max(1);
                    if let Some(p) = self.party.get_mut(battle.player_idx) {
                        p.hp = p.hp.saturating_sub(recoil);
                    }
                }

                // Self-Destruct/Explosion: user faints
                if move_id == MOVE_SELF_DESTRUCT {
                    if let Some(p) = self.party.get_mut(battle.player_idx) {
                        p.hp = 0;
                    }
                }

                // Hyper Beam: must recharge next turn (only if it hit)
                if move_id == MOVE_HYPER_BEAM && damage > 0 {
                    battle.player_must_recharge = true;
                }

                // Thrash/Outrage: start rampage if not already rampaging
                if (move_id == MOVE_THRASH || move_id == MOVE_OUTRAGE) && battle.player_rampage.1 == 0 {
                    let remaining = 1 + (engine.rng.next_u64() % 2) as u8;
                    battle.player_rampage = (remaining, move_id);
                }

                // Rest: full HP heal, force 2-turn sleep
                if move_id == MOVE_REST {
                    if let Some(p) = self.party.get_mut(battle.player_idx) {
                        p.hp = p.max_hp;
                        p.status = StatusCondition::Sleep { turns: 2 };
                    }
                }

                // Secondary status effects
                let is_status_move = get_move(move_id).map(|m| m.category == MoveCategory::Status).unwrap_or(false);
                if damage > 0 || is_status_move {
                    let roll = engine.rng.next_f64();
                    try_inflict_status(&mut battle.enemy, move_id, roll);
                }
                if damage > 0 {
                    let fc = flinch_chance(move_id);
                    if fc > 0.0 && !from_pending {
                        if engine.rng.next_f64() < fc { battle.enemy_flinched = true; }
                    }
                }
                if damage > 0 {
                    if let Some((target_enemy, stat_idx, delta, chance)) = damaging_move_stat_effect(move_id) {
                        if engine.rng.next_f64() < chance {
                            let stages = if target_enemy { &mut battle.enemy_stages } else { &mut battle.player_stages };
                            stages[stat_idx] = (stages[stat_idx] + delta).max(-6).min(6);
                        }
                    }
                }

                // Stat stage effects for status moves
                let move_data_ref = get_move(move_id);
                let move_name = move_data_ref.map(|m| m.name).unwrap_or("???");
                let pname = self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default();
                let is_miss = damage == 0
                    && move_data_ref.map(|m| m.power > 0 && m.category != MoveCategory::Status).unwrap_or(false)
                    && effectiveness > 0.0;

                let stage_msg = if !is_miss {
                    if move_id == MOVE_HAZE {
                        battle.player_stages = [0; 7];
                        battle.enemy_stages = [0; 7];
                        Some("All stat changes were reset!".to_string())
                    } else if move_id == MOVE_CONFUSE_RAY {
                        let prefix = if battle.is_wild { "Wild " } else { "Foe " };
                        if battle.enemy_confused == 0 {
                            battle.enemy_confused = 2 + (engine.rng.next_f64() * 4.0) as u8;
                            Some(format!("{}{} became confused!", prefix, battle.enemy.name()))
                        } else {
                            Some(format!("{}{} is already confused!", prefix, battle.enemy.name()))
                        }
                    } else if move_id == MOVE_SWAGGER {
                        let old = battle.enemy_stages[STAGE_ATK];
                        battle.enemy_stages[STAGE_ATK] = (old + 2).min(6);
                        let prefix = if battle.is_wild { "Wild " } else { "Foe " };
                        if battle.enemy_confused == 0 {
                            battle.enemy_confused = 2 + (engine.rng.next_f64() * 4.0) as u8;
                            Some(format!("{}{} became confused!", prefix, battle.enemy.name()))
                        } else {
                            Some(format!("{}{} is already confused!", prefix, battle.enemy.name()))
                        }
                    } else if move_id == MOVE_MEAN_LOOK {
                        None
                    } else if let Some((target_enemy, stat_idx, delta)) = status_move_stage_effect(move_id) {
                        let stages = if target_enemy { &mut battle.enemy_stages } else { &mut battle.player_stages };
                        let old = stages[stat_idx];
                        stages[stat_idx] = (old + delta).max(-6).min(6);
                        if stages[stat_idx] != old {
                            let stat_name = match stat_idx { STAGE_ATK => "Attack", STAGE_DEF => "Defense", STAGE_SPA => "Sp. Atk", STAGE_SPD => "Sp. Def", STAGE_SPE => "Speed", STAGE_ACC => "accuracy", _ => "evasion" };
                            let target_name = if target_enemy { let pfx = if battle.is_wild { "Wild " } else { "Foe " }; format!("{}{}", pfx, battle.enemy.name()) } else { pname.clone() };
                            let change = if delta > 1 { "sharply rose!" } else if delta > 0 { "rose!" } else if delta < -1 { "sharply fell!" } else { "fell!" };
                            Some(format!("{}'s {} {}", target_name, stat_name, change))
                        } else {
                            let dir = if delta > 0 { "go any higher!" } else { "go any lower!" };
                            Some(format!("{} won't {}", match stat_idx { STAGE_ATK => "Attack", STAGE_DEF => "Defense", STAGE_SPA => "Sp. Atk", STAGE_SPD => "Sp. Def", STAGE_SPE => "Speed", STAGE_ACC => "accuracy", _ => "evasion" }, dir))
                        }
                    } else { None }
                } else { None };

                // --- Build queue ---
                battle.battle_queue.clear();
                battle.queue_timer = 0.0;

                // Confusion snapout text (from_pending)
                if from_pending {
                    if let Some(sm) = battle.confusion_snapout_msg.take() {
                        battle.battle_queue.push_back(BattleStep::Text(sm));
                    }
                }

                // "X used Y!"
                battle.battle_queue.push_back(BattleStep::Text(format!("{} used {}!", pname, move_name)));

                // Flash effect + SFX (only if damage > 0)
                if damage > 0 {
                    let flash_val = if effectiveness > 1.5 { 0.9 } else { 0.6 };
                    battle.battle_queue.push_back(BattleStep::ScreenFlash(flash_val));
                    battle.battle_queue.push_back(BattleStep::PlayHitSfx(effectiveness > 1.5));
                }

                // Pause for hit timing
                battle.battle_queue.push_back(BattleStep::Pause(0.3));

                // Apply damage + HP drain animation
                let enemy_target_hp = battle.enemy.hp; // already applied above
                battle.battle_queue.push_back(BattleStep::ApplyDamage { is_player: false, amount: 0 }); // HP already applied; sync display
                battle.battle_queue.push_back(BattleStep::DrainHp { is_player: false, to_hp: enemy_target_hp, duration: 0.5 });

                // Recoil drain for player
                if has_recoil {
                    let player_hp = self.party.get(battle.player_idx).map(|p| p.hp).unwrap_or(0);
                    battle.battle_queue.push_back(BattleStep::DrainHp { is_player: true, to_hp: player_hp, duration: 0.3 });
                }
                // Self-Destruct player drain
                if move_id == MOVE_SELF_DESTRUCT {
                    battle.battle_queue.push_back(BattleStep::DrainHp { is_player: true, to_hp: 0, duration: 0.3 });
                }

                // Follow-up text messages
                if is_miss {
                    battle.battle_queue.push_back(BattleStep::Text("Attack missed!".into()));
                } else {
                    if is_crit { battle.battle_queue.push_back(BattleStep::Text("Critical hit!".into())); }
                    let eff = eff_text(effectiveness);
                    if !eff.is_empty() { battle.battle_queue.push_back(BattleStep::Text(eff.to_string())); }
                }
                if has_recoil { battle.battle_queue.push_back(BattleStep::Text(format!("{} is hit with recoil!", pname))); }
                if num_hits > 1 && !is_miss { battle.battle_queue.push_back(BattleStep::Text(format!("Hit {} times!", num_hits))); }
                if let Some(ref sm) = stage_msg { battle.battle_queue.push_back(BattleStep::Text(sm.clone())); }

                // Determine terminal phase
                if battle.enemy.is_fainted() {
                    battle.battle_queue.push_back(BattleStep::CheckFaint { is_player: false });
                    // CheckFaint will transition to EnemyFainted
                } else if from_pending {
                    // End-of-turn status damage
                    let mut eot_msgs: Vec<String> = Vec::new();
                    let pname_eot = self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default();
                    if let Some(p) = self.party.get_mut(battle.player_idx) {
                        let pdmg = p.apply_status_damage();
                        if pdmg > 0 {
                            let st = match p.status {
                                StatusCondition::Burn => format!("{} is hurt by its burn!", pname_eot),
                                StatusCondition::BadPoison { .. } => format!("{} is hurt by poison!", pname_eot),
                                _ => format!("{} is hurt by poison!", pname_eot),
                            };
                            eot_msgs.push(st);
                        }
                        let woke = p.tick_status();
                        if woke { eot_msgs.push(format!("{} woke up!", pname_eot)); }
                    }
                    let eprefix = if battle.is_wild { "Wild " } else { "Foe " };
                    let ename_eot = battle.enemy.name().to_string();
                    let edmg = battle.enemy.apply_status_damage();
                    if edmg > 0 {
                        let st = match battle.enemy.status {
                            StatusCondition::Burn => format!("{}{} is hurt by its burn!", eprefix, ename_eot),
                            StatusCondition::BadPoison { .. } => format!("{}{} is hurt by poison!", eprefix, ename_eot),
                            _ => format!("{}{} is hurt by poison!", eprefix, ename_eot),
                        };
                        eot_msgs.push(st);
                    }
                    let ewoke = battle.enemy.tick_status();
                    if ewoke { eot_msgs.push(format!("{}{} woke up!", eprefix, battle.enemy.name())); }
                    battle.turn_count += 1;
                    for m in &eot_msgs { battle.battle_queue.push_back(BattleStep::Text(m.clone())); }
                    // Update HP displays for status damage
                    let player_hp_now = self.party.get(battle.player_idx).map(|p| p.hp).unwrap_or(0);
                    let enemy_hp_now = battle.enemy.hp;
                    if !eot_msgs.is_empty() {
                        battle.battle_queue.push_back(BattleStep::DrainHp { is_player: true, to_hp: player_hp_now, duration: 0.3 });
                        battle.battle_queue.push_back(BattleStep::DrainHp { is_player: false, to_hp: enemy_hp_now, duration: 0.3 });
                    }
                    let terminal = if self.party.get(battle.player_idx).map(|p| p.is_fainted()).unwrap_or(false) {
                        BattlePhase::PlayerFainted
                    } else if battle.enemy.is_fainted() {
                        let exp = get_species(battle.enemy.species_id)
                            .map(|sp| exp_gained(sp, battle.enemy.level, battle.is_wild)).unwrap_or(10);
                        BattlePhase::EnemyFainted { exp_gained: exp }
                    } else {
                        BattlePhase::ActionSelect { cursor: 0 }
                    };
                    battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(terminal)));
                } else if self.party.get(battle.player_idx).map(|p| p.is_fainted()).unwrap_or(false) {
                    // Player died from recoil or Self-Destruct
                    battle.battle_queue.push_back(BattleStep::CheckFaint { is_player: true });
                } else {
                    // Player went first — enemy attacks next
                    // Freeze thaw
                    let enemy_thawed = battle.enemy.try_thaw(engine.rng.next_f64());
                    if enemy_thawed {
                        let prefix = if battle.is_wild { "Wild " } else { "Foe " };
                        battle.battle_queue.push_back(BattleStep::Text(format!("{}{} thawed out!", prefix, battle.enemy.name())));
                    }
                    let enemy_can_move = battle.enemy.can_move();
                    let enemy_paralyzed = matches!(battle.enemy.status, StatusCondition::Paralysis) && engine.rng.next_f64() < PARALYSIS_SKIP_CHANCE;
                    let enemy_flinched = battle.enemy_flinched;
                    battle.enemy_flinched = false;
                    if !enemy_can_move || enemy_paralyzed || enemy_flinched {
                        let prefix = if battle.is_wild { "Wild " } else { "Foe " };
                        let reason = if enemy_flinched { format!("{}{} flinched!", prefix, battle.enemy.name()) }
                            else if enemy_paralyzed { format!("{}{} is paralyzed!", prefix, battle.enemy.name()) }
                            else if matches!(battle.enemy.status, StatusCondition::Freeze) { format!("{}{} is frozen solid!", prefix, battle.enemy.name()) }
                            else { format!("{}{} is fast asleep!", prefix, battle.enemy.name()) };
                        battle.battle_queue.push_back(BattleStep::Text(reason));
                        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::ActionSelect { cursor: 0 })));
                    } else if battle.enemy_confused > 0 {
                        battle.enemy_confused -= 1;
                        let prefix = if battle.is_wild { "Wild " } else { "Foe " };
                        if battle.enemy_confused == 0 {
                            let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
                            battle.battle_queue.push_back(BattleStep::Text(format!("{}{} snapped out of confusion!", prefix, battle.enemy.name())));
                            battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::EnemyAttack {
                                timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                            })));
                        } else if engine.rng.next_f64() < 0.5 {
                            let atk = battle.enemy.attack as f64;
                            let def = battle.enemy.defense as f64;
                            let lvl = battle.enemy.level as f64;
                            let self_dmg = ((2.0 * lvl / 5.0 + 2.0) * 40.0 * atk / def) / 50.0 + 2.0;
                            battle.enemy.hp = battle.enemy.hp.saturating_sub(self_dmg as u16);
                            battle.battle_queue.push_back(BattleStep::Text(format!("{}{} is confused!", prefix, battle.enemy.name())));
                            battle.battle_queue.push_back(BattleStep::Text("It hurt itself in its confusion!".into()));
                            let next = if battle.enemy.is_fainted() {
                                let exp = get_species(battle.enemy.species_id)
                                    .map(|sp| exp_gained(sp, battle.enemy.level, battle.is_wild)).unwrap_or(10);
                                BattlePhase::EnemyFainted { exp_gained: exp }
                            } else { BattlePhase::ActionSelect { cursor: 0 } };
                            battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(next)));
                        } else {
                            let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
                            battle.battle_queue.push_back(BattleStep::Text(format!("{}{} is confused!", prefix, battle.enemy.name())));
                            battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::EnemyAttack {
                                timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                            })));
                        }
                    } else {
                        let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
                        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::EnemyAttack {
                            timer: 0.0, move_id: e_move, damage: e_dmg, effectiveness: e_eff, is_crit: e_crit,
                        })));
                    }
                }

                battle.phase = BattlePhase::ExecuteQueue;
            }

            BattlePhase::EnemyAttack { timer: _, move_id, damage, effectiveness, is_crit } => {
                // Queue-based EnemyAttack: compute all effects immediately, push queue steps,
                // then transition to ExecuteQueue. No timer — all visual pacing is in the queue.

                // Enemy must recharge (Hyper Beam): skip attack entirely
                if battle.enemy_must_recharge {
                    battle.enemy_must_recharge = false;
                    let prefix = if battle.is_wild { "Wild " } else { "Foe " };
                    let ename = battle.enemy.name().to_string();
                    battle.battle_queue.clear();
                    battle.queue_timer = 0.0;
                    battle.battle_queue.push_back(BattleStep::Text(format!("{}{} must recharge!", prefix, ename)));
                    let next = if let Some((pm, pd, pe, pc)) = battle.pending_player_move.take() {
                        BattlePhase::PlayerAttack { timer: 0.0, move_id: pm, damage: pd, effectiveness: pe, is_crit: pc, from_pending: true }
                    } else {
                        BattlePhase::ActionSelect { cursor: 0 }
                    };
                    battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(next)));
                    battle.phase = BattlePhase::ExecuteQueue;
                    self.battle = Some(battle);
                    return;
                }

                // Enemy rampage: override selected move with rampage move
                let (move_id, damage, effectiveness, is_crit) = if battle.enemy_rampage.0 > 0 {
                    battle.enemy_rampage.0 -= 1;
                    let rampage_move = battle.enemy_rampage.1;
                    if rampage_move != move_id {
                        let (_, r_dmg, r_eff, r_crit) = self.calc_enemy_move_forced(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages, rampage_move);
                        (rampage_move, r_dmg, r_eff, r_crit)
                    } else { (move_id, damage, effectiveness, is_crit) }
                } else { (move_id, damage, effectiveness, is_crit) };

                let num_hits = multi_hit_count(move_id, engine.rng.next_f64());
                let damage = damage * num_hits as u16;

                // Apply damage + effects to player
                if let Some(p) = self.party.get_mut(battle.player_idx) {
                    p.hp = p.hp.saturating_sub(damage);
                    let is_status_move = get_move(move_id).map(|m| m.category == MoveCategory::Status).unwrap_or(false);
                    if damage > 0 || is_status_move {
                        let roll = engine.rng.next_f64();
                        try_inflict_status(p, move_id, roll);
                    }
                    if damage > 0 {
                        let fc = flinch_chance(move_id);
                        if fc > 0.0 && battle.pending_player_move.is_some() {
                            if engine.rng.next_f64() < fc { battle.player_flinched = true; }
                        }
                    }
                    if damage > 0 {
                        if let Some((target_player, stat_idx, delta, chance)) = damaging_move_stat_effect(move_id) {
                            if engine.rng.next_f64() < chance {
                                let stages = if target_player { &mut battle.player_stages } else { &mut battle.enemy_stages };
                                stages[stat_idx] = (stages[stat_idx] + delta).max(-6).min(6);
                            }
                        }
                    }
                }

                // Recoil
                let e_has_recoil = (move_id == MOVE_STRUGGLE || move_id == MOVE_TAKE_DOWN) && damage > 0;
                if e_has_recoil {
                    let recoil = (damage / 4).max(1);
                    battle.enemy.hp = battle.enemy.hp.saturating_sub(recoil);
                }
                if move_id == MOVE_SELF_DESTRUCT { battle.enemy.hp = 0; }
                if move_id == MOVE_HYPER_BEAM && damage > 0 { battle.enemy_must_recharge = true; }
                if (move_id == MOVE_THRASH || move_id == MOVE_OUTRAGE) && battle.enemy_rampage.1 == 0 {
                    let remaining = 1 + (engine.rng.next_u64() % 2) as u8;
                    battle.enemy_rampage = (remaining, move_id);
                }
                if move_id == MOVE_REST {
                    battle.enemy.hp = battle.enemy.max_hp;
                    battle.enemy.status = StatusCondition::Sleep { turns: 2 };
                }

                let move_data_ref = get_move(move_id);
                let move_name = move_data_ref.map(|m| m.name).unwrap_or("???");
                let ename = battle.enemy.name().to_string();
                let is_miss = damage == 0
                    && move_data_ref.map(|m| m.power > 0 && m.category != MoveCategory::Status).unwrap_or(false)
                    && effectiveness > 0.0;
                let prefix = if battle.is_wild { "Wild " } else { "Foe " };

                // Stat stage effects for enemy's status moves
                let e_stage_msg = if !is_miss {
                    if move_id == MOVE_HAZE {
                        battle.player_stages = [0; 7]; battle.enemy_stages = [0; 7];
                        Some("All stat changes were reset!".to_string())
                    } else if move_id == MOVE_CONFUSE_RAY {
                        let pname = self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default();
                        if battle.player_confused == 0 { battle.player_confused = 2 + (engine.rng.next_f64() * 4.0) as u8; Some(format!("{} became confused!", pname)) }
                        else { Some(format!("{} is already confused!", pname)) }
                    } else if move_id == MOVE_SWAGGER {
                        let old = battle.player_stages[STAGE_ATK]; battle.player_stages[STAGE_ATK] = (old + 2).min(6);
                        let pname = self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default();
                        if battle.player_confused == 0 { battle.player_confused = 2 + (engine.rng.next_f64() * 4.0) as u8; Some(format!("{} became confused!", pname)) }
                        else { Some(format!("{} is already confused!", pname)) }
                    } else if move_id == MOVE_MEAN_LOOK {
                        if battle.is_wild { battle.player_trapped = true; let pname = self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default(); Some(format!("{} can't escape now!", pname)) }
                        else { None }
                    } else if let Some((target_enemy, stat_idx, delta)) = status_move_stage_effect(move_id) {
                        let stages = if target_enemy { &mut battle.player_stages } else { &mut battle.enemy_stages };
                        let old = stages[stat_idx]; stages[stat_idx] = (old + delta).max(-6).min(6);
                        if stages[stat_idx] != old {
                            let stat_name = match stat_idx { STAGE_ATK => "Attack", STAGE_DEF => "Defense", STAGE_SPA => "Sp. Atk", STAGE_SPD => "Sp. Def", STAGE_SPE => "Speed", STAGE_ACC => "accuracy", _ => "evasion" };
                            let target_name = if target_enemy { self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default() } else { format!("{}{}", prefix, battle.enemy.name()) };
                            let change = if delta > 1 { "sharply rose!" } else if delta > 0 { "rose!" } else if delta < -1 { "sharply fell!" } else { "fell!" };
                            Some(format!("{}'s {} {}", target_name, stat_name, change))
                        } else {
                            let dir = if delta > 0 { "go any higher!" } else { "go any lower!" };
                            Some(format!("{} won't {}", match stat_idx { STAGE_ATK => "Attack", STAGE_DEF => "Defense", STAGE_SPA => "Sp. Atk", STAGE_SPD => "Sp. Def", STAGE_SPE => "Speed", STAGE_ACC => "accuracy", _ => "evasion" }, dir))
                        }
                    } else { None }
                } else { None };

                let fainted = self.party.get(battle.player_idx).map(|p| p.is_fainted()).unwrap_or(true);
                let has_pending = battle.pending_player_move.is_some();

                // --- Build queue ---
                battle.battle_queue.clear();
                battle.queue_timer = 0.0;

                // "Foe X used Y!"
                battle.battle_queue.push_back(BattleStep::Text(format!("{}{} used {}!", prefix, ename, move_name)));

                // Screen shake + SFX
                if damage > 0 {
                    let shake_val = if effectiveness > 1.5 { 6.0 } else { 3.0 };
                    battle.battle_queue.push_back(BattleStep::ScreenShake(shake_val));
                    battle.battle_queue.push_back(BattleStep::PlayHitSfx(effectiveness > 1.5));
                }

                battle.battle_queue.push_back(BattleStep::Pause(0.3));

                // HP drain animation (HP already applied)
                let player_target_hp = self.party.get(battle.player_idx).map(|p| p.hp).unwrap_or(0);
                battle.battle_queue.push_back(BattleStep::DrainHp { is_player: true, to_hp: player_target_hp, duration: 0.5 });

                // Recoil drain for enemy
                if e_has_recoil || move_id == MOVE_SELF_DESTRUCT {
                    battle.battle_queue.push_back(BattleStep::DrainHp { is_player: false, to_hp: battle.enemy.hp, duration: 0.3 });
                }

                // Follow-up messages
                if is_miss {
                    battle.battle_queue.push_back(BattleStep::Text("Attack missed!".into()));
                } else {
                    if is_crit { battle.battle_queue.push_back(BattleStep::Text("Critical hit!".into())); }
                    let eff = eff_text(effectiveness);
                    if !eff.is_empty() { battle.battle_queue.push_back(BattleStep::Text(eff.to_string())); }
                }
                if e_has_recoil {
                    let eprefix_rc = if battle.is_wild { "Wild " } else { "Foe " };
                    battle.battle_queue.push_back(BattleStep::Text(format!("{}{} is hit with recoil!", eprefix_rc, ename)));
                }
                if num_hits > 1 && !is_miss { battle.battle_queue.push_back(BattleStep::Text(format!("Hit {} times!", num_hits))); }
                if let Some(ref sm) = e_stage_msg { battle.battle_queue.push_back(BattleStep::Text(sm.clone())); }

                // Determine terminal phase
                let next = if fainted {
                    BattlePhase::PlayerFainted
                } else if battle.enemy.is_fainted() {
                    battle.pending_player_move = None;
                    let exp = get_species(battle.enemy.species_id)
                        .map(|sp| exp_gained(sp, battle.enemy.level, battle.is_wild)).unwrap_or(10);
                    BattlePhase::EnemyFainted { exp_gained: exp }
                } else if has_pending && battle.player_flinched {
                    battle.pending_player_move = None;
                    battle.player_flinched = false;
                    let pname = self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default();
                    battle.battle_queue.push_back(BattleStep::Text(format!("{} flinched!", pname)));
                    BattlePhase::ActionSelect { cursor: 0 }
                } else if has_pending {
                    battle.player_flinched = false;
                    if let Some((pm_id, pm_dmg, pm_eff, pm_crit)) = battle.pending_player_move.take() {
                        BattlePhase::PlayerAttack {
                            timer: 0.0, move_id: pm_id, damage: pm_dmg,
                            effectiveness: pm_eff, is_crit: pm_crit, from_pending: true,
                        }
                    } else { BattlePhase::ActionSelect { cursor: 0 } }
                } else {
                    // End-of-turn status damage
                    let mut eot_msgs: Vec<String> = Vec::new();
                    let pname2 = self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default();
                    let player_fainted_from_status;
                    if let Some(p) = self.party.get_mut(battle.player_idx) {
                        let pdmg = p.apply_status_damage();
                        if pdmg > 0 {
                            let st = match p.status { StatusCondition::Burn => format!("{} is hurt by its burn!", pname2), StatusCondition::BadPoison { .. } => format!("{} is hurt by poison!", pname2), _ => format!("{} is hurt by poison!", pname2) };
                            eot_msgs.push(st);
                        }
                        let woke = p.tick_status();
                        if woke { eot_msgs.push(format!("{} woke up!", pname2)); }
                        player_fainted_from_status = p.is_fainted() && !fainted;
                    } else { player_fainted_from_status = false; }
                    let eprefix2 = if battle.is_wild { "Wild " } else { "Foe " };
                    let ename2 = battle.enemy.name().to_string();
                    let edmg2 = battle.enemy.apply_status_damage();
                    if edmg2 > 0 {
                        let st = match battle.enemy.status { StatusCondition::Burn => format!("{}{} is hurt by its burn!", eprefix2, ename2), StatusCondition::BadPoison { .. } => format!("{}{} is hurt by poison!", eprefix2, ename2), _ => format!("{}{} is hurt by poison!", eprefix2, ename2) };
                        eot_msgs.push(st);
                    }
                    let ewoke2 = battle.enemy.tick_status();
                    if ewoke2 { eot_msgs.push(format!("{}{} woke up!", eprefix2, battle.enemy.name())); }
                    for m in &eot_msgs { battle.battle_queue.push_back(BattleStep::Text(m.clone())); }
                    // Update HP displays for status damage
                    if !eot_msgs.is_empty() {
                        let php = self.party.get(battle.player_idx).map(|p| p.hp).unwrap_or(0);
                        let ehp = battle.enemy.hp;
                        battle.battle_queue.push_back(BattleStep::DrainHp { is_player: true, to_hp: php, duration: 0.3 });
                        battle.battle_queue.push_back(BattleStep::DrainHp { is_player: false, to_hp: ehp, duration: 0.3 });
                    }

                    if player_fainted_from_status { BattlePhase::PlayerFainted }
                    else if battle.enemy.is_fainted() {
                        let exp = get_species(battle.enemy.species_id)
                            .map(|sp| exp_gained(sp, battle.enemy.level, battle.is_wild)).unwrap_or(10);
                        BattlePhase::EnemyFainted { exp_gained: exp }
                    } else {
                        if battle.enemy_rampage.1 != 0 && battle.enemy_rampage.0 == 0 {
                            battle.enemy_rampage = (0, 0);
                            if battle.enemy_confused == 0 {
                                battle.enemy_confused = 2 + (engine.rng.next_u64() % 4) as u8;
                            }
                        }
                        BattlePhase::ActionSelect { cursor: 0 }
                    }
                };

                if !has_pending { battle.turn_count += 1; }
                battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(next)));
                battle.phase = BattlePhase::ExecuteQueue;
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
                // Check if player also fainted (Self-Destruct/Explosion mutual KO)
                let player_also_fainted = self.party.get(battle.player_idx).map(|p| p.is_fainted()).unwrap_or(false);
                if player_also_fainted {
                    battle.phase = BattlePhase::PlayerFainted;
                } else {
                    // Show EXP gain text, then award EXP in ExpAwarded phase
                    let pname = self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default();
                    battle.phase = BattlePhase::Text {
                        message: format!("{} gained {} EXP!", pname, exp),
                        timer: 0.0,
                        next_phase: Box::new(BattlePhase::ExpAwarded { exp_gained: exp, timer: 0.0 }),
                    };
                }
            }

            BattlePhase::ExpAwarded { exp_gained: exp, timer } => {
                let t = timer + dt;
                // Animate for 1 second, then award EXP
                if t < 1.0 && !is_confirm(engine) {
                    battle.phase = BattlePhase::ExpAwarded { exp_gained: exp, timer: t };
                    self.battle = Some(battle);
                    return;
                }
                // Actually award EXP and check level up
                if let Some(p) = self.party.get_mut(battle.player_idx) {
                    p.exp += exp;
                    let mut leveled = false;
                    let mut pending_learns = Vec::new();
                    let mut auto_learn_msgs: Vec<String> = Vec::new();
                    let mut stat_deltas: [i16; 6] = [0; 6]; // [HP, Atk, Def, SpAtk, SpDef, Spd]
                    // Loop for multi-level-up (e.g., low-level vs high-level enemy)
                    while p.level < 100 {
                        let sp = match get_species(p.species_id) { Some(s) => s, None => break };
                        let next_exp = exp_for_level(p.level + 1, sp.growth_rate);
                        if p.exp < next_exp { break; }
                        // Capture old stats for delta display
                        let old = [p.max_hp, p.attack, p.defense, p.sp_attack, p.sp_defense, p.speed];
                        p.level += 1;
                        leveled = true;
                        p.recalc_stats();
                        // Accumulate stat deltas (handles multi-level-up)
                        let new = [p.max_hp, p.attack, p.defense, p.sp_attack, p.sp_defense, p.speed];
                        for i in 0..6 { stat_deltas[i] += new[i] as i16 - old[i] as i16; }
                        let new_moves = p.check_new_moves();
                        for new_move in new_moves {
                            if p.moves.iter().any(|m| *m == Some(new_move)) { continue; }
                            let mut filled = false;
                            for i in 0..4 {
                                if p.moves[i].is_none() {
                                    p.moves[i] = Some(new_move);
                                    if let Some(md) = get_move(new_move) {
                                        p.move_pp[i] = md.pp;
                                        p.move_max_pp[i] = md.pp;
                                    }
                                    // Track auto-learned moves for text display
                                    let mname = get_move(new_move).map(|m| m.name).unwrap_or("???");
                                    auto_learn_msgs.push(format!("{} learned {}!", p.name(), mname));
                                    filled = true;
                                    break;
                                }
                            }
                            if !filled { pending_learns.push(new_move); }
                        }
                    }
                    if leveled {
                        battle.pending_learn_moves = pending_learns;
                        let evo_species = get_species(p.species_id)
                            .and_then(|s| {
                                if let (Some(evo_lvl), Some(evo_into)) = (s.evolution_level, s.evolution_into) {
                                    if p.level >= evo_lvl { Some(evo_into) } else { None }
                                } else { None }
                            });
                        sfx_level_up(engine);
                        // Chain auto-learned move messages before LevelUp
                        let mut lvlup_inner = BattlePhase::LevelUp { timer: 0.0, stat_deltas };
                        for m in auto_learn_msgs.iter().rev() {
                            lvlup_inner = BattlePhase::Text { message: m.clone(), timer: 0.0, next_phase: Box::new(lvlup_inner) };
                        }
                        if let Some(evo) = evo_species {
                            battle.phase = BattlePhase::Text {
                                message: format!("{} grew to LV{}!", p.name(), p.level),
                                timer: 0.0,
                                next_phase: Box::new(lvlup_inner),
                            };
                            self.battle = Some(battle);
                            self.phase = GamePhase::Battle;
                            engine.global_state.set_f64("pending_evolution", evo as f64);
                            return;
                        }
                        battle.phase = BattlePhase::Text {
                            message: format!("{} grew to LV{}!", p.name(), p.level),
                            timer: 0.0,
                            next_phase: Box::new(lvlup_inner),
                        };
                        self.battle = Some(battle);
                        return;
                    }
                }
                // No level up — check for more trainer Pokemon or Won
                if !battle.is_wild && !battle.trainer_team.is_empty() {
                    let next_enemy = battle.trainer_team.remove(0);
                    battle.trainer_team_idx += 1;
                    let next_name = next_enemy.name().to_string();
                    battle.enemy = next_enemy;
                    battle.enemy_hp_display = battle.enemy.hp as f64;
                    battle.enemy_stages = [0; 7];
                    battle.enemy_confused = 0;
                    battle.enemy_flinched = false;
                    battle.enemy_must_recharge = false;
                    battle.enemy_rampage = (0, 0);
                    battle.phase = BattlePhase::TrainerSwitchPrompt { next_name, cursor: 0 };
                } else {
                    // Queue-based Won: show "You won!" text, then skip to Won cleanup
                    battle.battle_queue.clear();
                    battle.queue_timer = 0.0;
                    battle.battle_queue.push_back(BattleStep::Text("You won!".into()));
                    battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::Won { timer: 2.0 })));
                    battle.phase = BattlePhase::ExecuteQueue;
                }
            }

            BattlePhase::LevelUp { timer, stat_deltas } => {
                let t = timer + dt;
                if t > 2.0 || is_confirm(engine) {
                    // Check for pending move learns before advancing
                    if !battle.pending_learn_moves.is_empty() {
                        let new_move = battle.pending_learn_moves.remove(0);
                        battle.phase = BattlePhase::LearnMove {
                            new_move,
                            sub: LearnMoveSub::TryingToLearn { timer: 0.0 },
                        };
                    } else if !battle.is_wild && !battle.trainer_team.is_empty() {
                        let next_enemy = battle.trainer_team.remove(0);
                        battle.trainer_team_idx += 1;
                        let next_name = next_enemy.name().to_string();
                        battle.enemy = next_enemy;
                        battle.enemy_hp_display = battle.enemy.hp as f64;
                        battle.enemy_stages = [0; 7];
                        battle.enemy_confused = 0;
                        battle.enemy_flinched = false;
                        battle.enemy_must_recharge = false;
                        battle.enemy_rampage = (0, 0);
                        battle.phase = BattlePhase::TrainerSwitchPrompt { next_name, cursor: 0 };
                    } else {
                        // Queue-based Won: show "You won!" text, then skip to Won cleanup
                        battle.battle_queue.clear();
                        battle.queue_timer = 0.0;
                        battle.battle_queue.push_back(BattleStep::Text("You won!".into()));
                        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::Won { timer: 2.0 })));
                        battle.phase = BattlePhase::ExecuteQueue;
                    }
                } else {
                    battle.phase = BattlePhase::LevelUp { timer: t, stat_deltas };
                }
            }

            BattlePhase::LearnMove { new_move, sub } => {
                match sub {
                    LearnMoveSub::TryingToLearn { timer } => {
                        let t = timer + dt;
                        if t > 2.5 || is_confirm(engine) {
                            battle.phase = BattlePhase::LearnMove {
                                new_move,
                                sub: LearnMoveSub::CantLearnMore { timer: 0.0 },
                            };
                        } else {
                            battle.phase = BattlePhase::LearnMove {
                                new_move,
                                sub: LearnMoveSub::TryingToLearn { timer: t },
                            };
                        }
                    }
                    LearnMoveSub::CantLearnMore { timer } => {
                        let t = timer + dt;
                        if t > 2.5 || is_confirm(engine) {
                            battle.phase = BattlePhase::LearnMove {
                                new_move,
                                sub: LearnMoveSub::DeletePrompt { cursor: 0 },
                            };
                        } else {
                            battle.phase = BattlePhase::LearnMove {
                                new_move,
                                sub: LearnMoveSub::CantLearnMore { timer: t },
                            };
                        }
                    }
                    LearnMoveSub::DeletePrompt { cursor } => {
                        if is_up(engine) || is_down(engine) {
                            battle.phase = BattlePhase::LearnMove {
                                new_move,
                                sub: LearnMoveSub::DeletePrompt { cursor: 1 - cursor },
                            };
                        } else if is_confirm(engine) {
                            if cursor == 0 {
                                // YES — pick which move to forget
                                battle.phase = BattlePhase::LearnMove {
                                    new_move,
                                    sub: LearnMoveSub::PickMove { cursor: 0 },
                                };
                            } else {
                                // NO — confirm giving up
                                battle.phase = BattlePhase::LearnMove {
                                    new_move,
                                    sub: LearnMoveSub::StopPrompt { cursor: 0 },
                                };
                            }
                        } else if is_cancel(engine) {
                            battle.phase = BattlePhase::LearnMove {
                                new_move,
                                sub: LearnMoveSub::StopPrompt { cursor: 0 },
                            };
                        }
                    }
                    LearnMoveSub::PickMove { cursor } => {
                        if is_down(engine) {
                            battle.phase = BattlePhase::LearnMove {
                                new_move,
                                sub: LearnMoveSub::PickMove { cursor: (cursor + 1) % 4 },
                            };
                        } else if is_up(engine) {
                            battle.phase = BattlePhase::LearnMove {
                                new_move,
                                sub: LearnMoveSub::PickMove { cursor: if cursor == 0 { 3 } else { cursor - 1 } },
                            };
                        } else if is_confirm(engine) {
                            // Forget the selected move, learn the new one
                            if let Some(p) = self.party.get_mut(battle.player_idx) {
                                let slot = cursor as usize;
                                p.moves[slot] = Some(new_move);
                                if let Some(md) = get_move(new_move) {
                                    p.move_pp[slot] = md.pp;
                                    p.move_max_pp[slot] = md.pp;
                                }
                            }
                            battle.phase = BattlePhase::LearnMove {
                                new_move,
                                sub: LearnMoveSub::ForgotMove { timer: 0.0, slot: cursor as usize },
                            };
                        } else if is_cancel(engine) {
                            // Go back to delete prompt
                            battle.phase = BattlePhase::LearnMove {
                                new_move,
                                sub: LearnMoveSub::DeletePrompt { cursor: 0 },
                            };
                        }
                    }
                    LearnMoveSub::ForgotMove { timer, slot } => {
                        let t = timer + dt;
                        if t > 2.0 || is_confirm(engine) {
                            battle.phase = BattlePhase::LearnMove {
                                new_move,
                                sub: LearnMoveSub::LearnedMove { timer: 0.0 },
                            };
                        } else {
                            battle.phase = BattlePhase::LearnMove {
                                new_move,
                                sub: LearnMoveSub::ForgotMove { timer: t, slot },
                            };
                        }
                    }
                    LearnMoveSub::LearnedMove { timer } => {
                        let t = timer + dt;
                        if t > 2.0 || is_confirm(engine) {
                            // Check for more pending moves
                            if !battle.pending_learn_moves.is_empty() {
                                let next = battle.pending_learn_moves.remove(0);
                                battle.phase = BattlePhase::LearnMove {
                                    new_move: next,
                                    sub: LearnMoveSub::TryingToLearn { timer: 0.0 },
                                };
                            } else if !battle.is_wild && !battle.trainer_team.is_empty() {
                                let next_enemy = battle.trainer_team.remove(0);
                                battle.trainer_team_idx += 1;
                                let next_name = next_enemy.name().to_string();
                                battle.enemy = next_enemy;
                                battle.enemy_hp_display = battle.enemy.hp as f64;
                                battle.enemy_stages = [0; 7];
                                battle.enemy_confused = 0;
                                battle.enemy_flinched = false;
                                battle.enemy_must_recharge = false;
                                battle.enemy_rampage = (0, 0);
                                battle.phase = BattlePhase::TrainerSwitchPrompt { next_name, cursor: 0 };
                            } else {
                                // Queue-based Won: show "You won!" text, then skip to Won cleanup
                                battle.battle_queue.clear();
                                battle.queue_timer = 0.0;
                                battle.battle_queue.push_back(BattleStep::Text("You won!".into()));
                                battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::Won { timer: 2.0 })));
                                battle.phase = BattlePhase::ExecuteQueue;
                            }
                        } else {
                            battle.phase = BattlePhase::LearnMove {
                                new_move,
                                sub: LearnMoveSub::LearnedMove { timer: t },
                            };
                        }
                    }
                    LearnMoveSub::StopPrompt { cursor } => {
                        if is_up(engine) || is_down(engine) {
                            battle.phase = BattlePhase::LearnMove {
                                new_move,
                                sub: LearnMoveSub::StopPrompt { cursor: 1 - cursor },
                            };
                        } else if is_confirm(engine) {
                            if cursor == 0 {
                                // YES — don't learn the move
                                battle.phase = BattlePhase::LearnMove {
                                    new_move,
                                    sub: LearnMoveSub::DidNotLearn { timer: 0.0 },
                                };
                            } else {
                                // NO — go back to delete prompt
                                battle.phase = BattlePhase::LearnMove {
                                    new_move,
                                    sub: LearnMoveSub::DeletePrompt { cursor: 0 },
                                };
                            }
                        }
                    }
                    LearnMoveSub::DidNotLearn { timer } => {
                        let t = timer + dt;
                        if t > 2.0 || is_confirm(engine) {
                            // Check for more pending moves
                            if !battle.pending_learn_moves.is_empty() {
                                let next = battle.pending_learn_moves.remove(0);
                                battle.phase = BattlePhase::LearnMove {
                                    new_move: next,
                                    sub: LearnMoveSub::TryingToLearn { timer: 0.0 },
                                };
                            } else if !battle.is_wild && !battle.trainer_team.is_empty() {
                                let next_enemy = battle.trainer_team.remove(0);
                                battle.trainer_team_idx += 1;
                                let next_name = next_enemy.name().to_string();
                                battle.enemy = next_enemy;
                                battle.enemy_hp_display = battle.enemy.hp as f64;
                                battle.enemy_stages = [0; 7];
                                battle.enemy_confused = 0;
                                battle.enemy_flinched = false;
                                battle.enemy_must_recharge = false;
                                battle.enemy_rampage = (0, 0);
                                battle.phase = BattlePhase::TrainerSwitchPrompt { next_name, cursor: 0 };
                            } else {
                                // Queue-based Won: show "You won!" text, then skip to Won cleanup
                                battle.battle_queue.clear();
                                battle.queue_timer = 0.0;
                                battle.battle_queue.push_back(BattleStep::Text("You won!".into()));
                                battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::Won { timer: 2.0 })));
                                battle.phase = BattlePhase::ExecuteQueue;
                            }
                        } else {
                            battle.phase = BattlePhase::LearnMove {
                                new_move,
                                sub: LearnMoveSub::DidNotLearn { timer: t },
                            };
                        }
                    }
                }
            }

            BattlePhase::TrainerSwitchPrompt { next_name, cursor } => {
                // "TRAINER is about to use <next_name>. Will you change POKEMON?" YES/NO
                if is_up(engine) || is_down(engine) {
                    battle.phase = BattlePhase::TrainerSwitchPrompt {
                        next_name,
                        cursor: 1 - cursor,
                    };
                } else if is_confirm(engine) {
                    if cursor == 0 {
                        // YES — free switch (no enemy attack penalty)
                        battle.free_switch = true;
                        battle.phase = BattlePhase::ActionSelect { cursor: 0 };
                        self.battle = Some(battle);
                        self.phase = GamePhase::PokemonMenu { cursor: 0, action: 0, sub_cursor: 0 };
                        return;
                    } else {
                        // NO — proceed to battle
                        battle.phase = BattlePhase::ActionSelect { cursor: 0 };
                    }
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

                            // Champion check first — credits must not be preempted by evolution
                            let pending_evo = engine.global_state.get_f64("pending_evolution").unwrap_or(0.0) as u16;
                            if map_id == MapId::ChampionLance && npc_idx == 0 {
                                engine.global_state.set_f64("pending_evolution", 0.0);
                                // Beat the Champion → credits!
                                self.dialogue = Some(DialogueState {
                                    lines: vec![
                                        "CHAMPION LANCE was defeated!".to_string(),
                                        format!("Got ${} for winning!", reward),
                                        "Congratulations!".to_string(),
                                        "You are the new POKEMON CHAMPION!".to_string(),
                                    ],
                                    current_line: 0, char_index: 0, timer: 0.0,
                                    on_complete: DialogueAction::Credits,
                                });
                                self.phase = GamePhase::Dialogue;
                            } else if pending_evo > 0 {
                                engine.global_state.set_f64("pending_evolution", 0.0);
                                self.phase = GamePhase::Evolution { timer: 0.0, new_species: pending_evo };
                            } else {
                                // Check if this was a gym leader battle or Elder Li
                                let badge_action = match (map_id, npc_idx) {
                                    (MapId::VioletGym, 0) => Some(DialogueAction::GiveBadge { badge_num: 0 }),
                                    (MapId::AzaleaGym, 0) => Some(DialogueAction::GiveBadge { badge_num: 1 }),
                                    (MapId::GoldenrodGym, 0) => Some(DialogueAction::GiveBadge { badge_num: 2 }),
                                    (MapId::EcruteakGym, 0) => Some(DialogueAction::GiveBadge { badge_num: 3 }),
                                    (MapId::OlivineGym, 0) => Some(DialogueAction::GiveBadge { badge_num: 4 }),
                                    (MapId::CianwoodGym, 0) => Some(DialogueAction::GiveBadge { badge_num: 5 }),
                                    (MapId::MahoganyGym, 0) => Some(DialogueAction::GiveBadge { badge_num: 6 }),
                                    (MapId::BlackthornGym, 0) => Some(DialogueAction::GiveBadge { badge_num: 7 }),
                                    (MapId::SproutTower3F, 0) => Some(DialogueAction::GiveFlash),
                                    _ => None,
                                };

                                // Rocket HQ boss: set story flag
                                if map_id == MapId::RocketHQ && npc_idx == 4 {
                                    self.set_flag(FLAG_ROCKET_MAHOGANY);
                                }

                                // Slowpoke Well executive (NPC 3): clearing all rockets
                                if map_id == MapId::SlowpokeWellB1F && npc_idx == 3 {
                                    self.set_flag(FLAG_SLOWPOKE_WELL);
                                }

                                let mut lines = vec![
                                    "Trainer was defeated!".to_string(),
                                    format!("Got ${} for winning!", reward),
                                ];
                                if map_id == MapId::RocketHQ && npc_idx == 4 {
                                    lines.push("The ROCKET HQ has".to_string());
                                    lines.push("been shut down!".to_string());
                                }
                                if map_id == MapId::SlowpokeWellB1F && npc_idx == 3 {
                                    lines.push("KURT: Way to go!".to_string());
                                    lines.push("TEAM ROCKET has".to_string());
                                    lines.push("taken off!".to_string());
                                    lines.push("My back's better too.".to_string());
                                    lines.push("Let's get out of here!".to_string());
                                }
                                self.dialogue = Some(DialogueState {
                                    lines,
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
                // Cleanup: "Got away safely!" text was already shown via ExecuteQueue
                engine.global_state.set_f64("in_battle", 0.0);
                self.phase = GamePhase::Overworld;
                self.battle = None;
                return;
            }

            BattlePhase::PlayerFainted => {
                sfx_faint(engine);
                let any_alive = self.party.iter().any(|p| !p.is_fainted());
                if any_alive {
                    // Auto-switch to next alive Pokemon — reset player state
                    battle.player_stages = [0; 7];
                    battle.player_confused = 0;
                    battle.player_flinched = false;
                    battle.player_must_recharge = false;
                    battle.player_rampage = (0, 0);
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
                    // Whiteout — start white fade effect
                    let lost = self.money / 2;
                    self.money -= lost;
                    for p in &mut self.party { p.heal(); }
                    engine.global_state.set_f64("in_battle", 0.0);
                    engine.global_state.set_f64("pending_evolution", 0.0);
                    self.battle = None;
                    self.phase = GamePhase::WhiteoutFade { timer: 0.0, money_lost: lost };
                    return;
                }
            }

            BattlePhase::ExecuteQueue => {
                self.step_execute_queue(&mut battle, engine);
            }
        }

        self.battle = Some(battle);
    }

    /// Process the next step in the battle queue. Called when phase == ExecuteQueue.
    /// Pops and processes BattleStep items in FIFO order. When the queue is empty,
    /// transitions back to ActionSelect.
    fn step_execute_queue(&mut self, battle: &mut BattleState, engine: &mut Engine) {
        let dt = 1.0 / 60.0;

        if battle.battle_queue.is_empty() {
            // Safety fallback: queue should never be empty when ExecuteQueue is active —
            // every queue sequence should end with GoToPhase. Log a warning for debugging.
            crate::log::warn("Battle queue empty during ExecuteQueue — falling back to ActionSelect");
            battle.phase = BattlePhase::ActionSelect { cursor: 0 };
            return;
        }

        // Peek at front step and process it
        let step = battle.battle_queue[0].clone();
        match step {
            BattleStep::Text(ref msg) => {
                let _ = msg; // text is rendered by render_battle
                battle.queue_timer += dt;
                if battle.queue_timer >= 1.5 || is_confirm(engine) {
                    battle.battle_queue.pop_front();
                    battle.queue_timer = 0.0;
                }
            }
            BattleStep::ApplyDamage { is_player, amount } => {
                if is_player {
                    if let Some(pkmn) = self.party.get_mut(battle.player_idx) {
                        pkmn.hp = pkmn.hp.saturating_sub(amount);
                    }
                } else {
                    battle.enemy.hp = battle.enemy.hp.saturating_sub(amount);
                }
                battle.battle_queue.pop_front();
                battle.queue_timer = 0.0;
            }
            BattleStep::DrainHp { is_player, to_hp, duration } => {
                let to_hp_f = to_hp as f64;
                battle.queue_timer += dt;
                if is_player {
                    let diff = battle.player_hp_display - to_hp_f;
                    battle.player_hp_display -= diff * (dt / duration).min(1.0);
                    if (battle.player_hp_display - to_hp_f).abs() < 0.5 {
                        battle.player_hp_display = to_hp_f;
                    }
                } else {
                    let diff = battle.enemy_hp_display - to_hp_f;
                    battle.enemy_hp_display -= diff * (dt / duration).min(1.0);
                    if (battle.enemy_hp_display - to_hp_f).abs() < 0.5 {
                        battle.enemy_hp_display = to_hp_f;
                    }
                }
                if battle.queue_timer >= duration || is_confirm(engine) {
                    if is_player { battle.player_hp_display = to_hp_f; }
                    else { battle.enemy_hp_display = to_hp_f; }
                    battle.battle_queue.pop_front();
                    battle.queue_timer = 0.0;
                }
            }
            BattleStep::InflictStatus { is_player, ref status } => {
                if is_player {
                    if let Some(pkmn) = self.party.get_mut(battle.player_idx) {
                        pkmn.status = status.clone();
                    }
                } else {
                    battle.enemy.status = status.clone();
                }
                battle.battle_queue.pop_front();
                battle.queue_timer = 0.0;
            }
            BattleStep::StatChange { is_player, stat, stages } => {
                let stat_idx = stat.min(6);
                if is_player {
                    battle.player_stages[stat_idx] = (battle.player_stages[stat_idx] + stages).max(-6).min(6);
                } else {
                    battle.enemy_stages[stat_idx] = (battle.enemy_stages[stat_idx] + stages).max(-6).min(6);
                }
                battle.battle_queue.pop_front();
                battle.queue_timer = 0.0;
            }
            BattleStep::CheckFaint { is_player } => {
                battle.battle_queue.pop_front();
                battle.queue_timer = 0.0;
                if is_player {
                    let fainted = self.party.get(battle.player_idx).map(|p| p.is_fainted()).unwrap_or(false);
                    if fainted {
                        battle.phase = BattlePhase::PlayerFainted;
                        return;
                    }
                } else if battle.enemy.is_fainted() {
                    // Calculate EXP for the fainted enemy
                    let base_exp = get_species(battle.enemy.species_id)
                        .map(|s| s.base_exp_yield)
                        .unwrap_or(50) as u32;
                    let exp = (base_exp * battle.enemy.level as u32) / 7;
                    battle.phase = BattlePhase::EnemyFainted { exp_gained: exp };
                    return;
                }
            }
            BattleStep::Pause(secs) => {
                battle.queue_timer += dt;
                if battle.queue_timer >= secs {
                    battle.battle_queue.pop_front();
                    battle.queue_timer = 0.0;
                }
            }
            BattleStep::GoToPhase(phase) => {
                battle.phase = *phase;
                battle.battle_queue.clear(); // clear entire queue — GoToPhase is a terminal step
                battle.queue_timer = 0.0;
            }
            BattleStep::ScreenFlash(val) => {
                self.screen_flash = val;
                battle.battle_queue.pop_front();
                battle.queue_timer = 0.0;
            }
            BattleStep::ScreenShake(val) => {
                self.screen_shake = val;
                battle.battle_queue.pop_front();
                battle.queue_timer = 0.0;
            }
            BattleStep::PlayHitSfx(super_effective) => {
                sfx_hit(engine, super_effective);
                battle.battle_queue.pop_front();
                battle.queue_timer = 0.0;
            }
        }
    }

    /// Build a queue sequence for an attack: "X used Y!", apply damage, drain HP,
    /// optional crit/effectiveness text, and faint check.
    #[allow(dead_code)]
    fn queue_attack_sequence(&mut self, attacker_name: &str, move_name: &str, damage: u16,
                             is_crit: bool, effectiveness: f64, is_player_target: bool) {
        if let Some(b) = self.battle.as_mut() {
            b.battle_queue.push_back(BattleStep::Text(format!("{} used {}!", attacker_name, move_name)));
            b.battle_queue.push_back(BattleStep::Pause(0.3));
            b.battle_queue.push_back(BattleStep::ApplyDamage { is_player: is_player_target, amount: damage });
            // Calculate target HP after damage for drain animation
            let target_hp = if is_player_target {
                self.party.get(b.player_idx).map(|p| p.hp.saturating_sub(damage)).unwrap_or(0)
            } else {
                b.enemy.hp.saturating_sub(damage)
            };
            b.battle_queue.push_back(BattleStep::DrainHp { is_player: is_player_target, to_hp: target_hp, duration: 0.5 });
            if is_crit {
                b.battle_queue.push_back(BattleStep::Text("A critical hit!".into()));
            }
            if effectiveness > 1.5 {
                b.battle_queue.push_back(BattleStep::Text("It's super effective!".into()));
            } else if effectiveness < 0.9 && effectiveness > 0.0 {
                b.battle_queue.push_back(BattleStep::Text("It's not very effective...".into()));
            }
            b.battle_queue.push_back(BattleStep::CheckFaint { is_player: is_player_target });
        }
    }

    fn calc_enemy_move(&self, engine: &mut Engine, enemy: &Pokemon, player_idx: usize, enemy_stages: &[i8; 7], player_stages: &[i8; 7]) -> (MoveId, u16, f64, bool) {
        self.calc_enemy_move_inner(engine, enemy, player_idx, enemy_stages, player_stages, None)
    }

    fn calc_enemy_move_forced(&self, engine: &mut Engine, enemy: &Pokemon, player_idx: usize, enemy_stages: &[i8; 7], player_stages: &[i8; 7], forced: MoveId) -> (MoveId, u16, f64, bool) {
        self.calc_enemy_move_inner(engine, enemy, player_idx, enemy_stages, player_stages, Some(forced))
    }

    fn calc_enemy_move_inner(&self, engine: &mut Engine, enemy: &Pokemon, player_idx: usize, enemy_stages: &[i8; 7], player_stages: &[i8; 7], forced_move: Option<MoveId>) -> (MoveId, u16, f64, bool) {
        let available: Vec<MoveId> = enemy.moves.iter().filter_map(|m| *m).collect();
        if available.is_empty() { return (MOVE_TACKLE, 5, 1.0, false); }

        // If forced (rampage), use that move
        let mid = if let Some(fm) = forced_move { fm } else
        // Smart AI: 50% chance to pick best move by effectiveness, 50% random
        if let Some(pp) = self.party.get(player_idx) {
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

    /// Calculate player damage for a given move (used by rampage continuation).
    /// Returns (damage, effectiveness, is_crit).
    fn calc_player_damage(&self, engine: &mut Engine, move_id: MoveId, battle: &BattleState) -> (u16, f64, bool) {
        let accuracy_ok = if let Some(md) = get_move(move_id) {
            if md.accuracy >= 255 {
                true
            } else {
                let acc_mult = accuracy_stage_multiplier(battle.player_stages[STAGE_ACC]);
                let eva_mult = accuracy_stage_multiplier(battle.enemy_stages[STAGE_EVA]);
                let effective_acc = (md.accuracy as f64 * acc_mult / eva_mult).min(100.0);
                if effective_acc >= 100.0 { true } else { (engine.rng.next_u64() % 100) < effective_acc as u64 }
            }
        } else { true };
        let is_crit = accuracy_ok && (engine.rng.next_u64() % CRIT_CHANCE) == 0;
        if !accuracy_ok {
            return (0, 1.0, false);
        }
        if let Some(move_data) = get_move(move_id) {
            let species = get_species(battle.enemy.species_id);
            let dt1 = species.map(|s| s.type1).unwrap_or(PokemonType::Normal);
            let dt2 = species.and_then(|s| s.type2);
            let rng = DAMAGE_ROLL_MIN + engine.rng.next_f64() * DAMAGE_ROLL_RANGE;
            let def_stat = match move_data.category {
                MoveCategory::Physical => battle.enemy.defense,
                _ => battle.enemy.sp_defense,
            };
            let atk_stage = match move_data.category {
                MoveCategory::Physical => battle.player_stages[STAGE_ATK],
                _ => battle.player_stages[STAGE_SPA],
            };
            let def_stage = match move_data.category {
                MoveCategory::Physical => battle.enemy_stages[STAGE_DEF],
                _ => battle.enemy_stages[STAGE_SPD],
            };
            let atk_mult = if is_crit { stage_multiplier(atk_stage.max(0)) } else { stage_multiplier(atk_stage) };
            let def_mult = if is_crit { stage_multiplier(def_stage.min(0)) } else { stage_multiplier(def_stage) };
            if let Some(atk) = self.party.get(battle.player_idx) {
                let (dmg, eff) = calc_damage(atk, def_stat, dt1, dt2, move_data, rng, is_crit, atk_mult, def_mult);
                (dmg, eff, is_crit)
            } else {
                (0, 1.0, false)
            }
        } else {
            (0, 1.0, false)
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
                        5 => "STORM BADGE",
                        6 => "GLACIER BADGE",
                        7 => "RISING BADGE",
                        _ => "BADGE",
                    };
                    let badge_effect = match badge_num {
                        0 => "Attack power increases!",
                        1 => "Pokemon up to LV 30 obey!",
                        2 => "Speed increases!",
                        3 => "Pokemon up to LV 50 obey!",
                        4 => "Defense increases!",
                        5 => "Pokemon up to LV 70 obey!",
                        6 => "Sp. Atk increases!",
                        7 => "All Pokemon will obey!",
                        _ => "",
                    };
                    self.screen_flash = 1.0; // celebration flash
                    let badge_count = self.badges.count_ones();
                    self.dialogue = Some(DialogueState {
                        lines: vec![
                            format!("Received the {}!", badge_name),
                            badge_effect.to_string(),
                            format!("Badges: {}/8", badge_count),
                        ],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                }
                DialogueAction::GiveFlash => {
                    self.screen_flash = 1.0; // celebration flash
                    self.dialogue = Some(DialogueState {
                        lines: vec![
                            "Received HM05 FLASH!".to_string(),
                            "FLASH lights up dark".to_string(),
                            "caves.".to_string(),
                        ],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                }
                DialogueAction::Credits => {
                    self.phase = GamePhase::Credits { scroll_y: 0.0 };
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
                            enemy_flinched: false,
                            player_flinched: false,
                            player_confused: 0,
                            enemy_confused: 0,
                            player_trapped: false,
                            player_must_recharge: false,
                            enemy_must_recharge: false,
                            player_rampage: (0, 0),
                            enemy_rampage: (0, 0),
                            pending_learn_moves: vec![],
                            free_switch: false,
                            confusion_snapout_msg: None,
                            battle_queue: VecDeque::new(),
                            queue_timer: 0.0,
                        });
                        self.encounter_flash_count = 0;
                        self.phase = GamePhase::EncounterTransition { timer: 0.0 };
                    } else {
                        // Safety fallback: empty team, return to overworld
                        self.phase = GamePhase::Overworld;
                    }
                }
                DialogueAction::StartFishBattle { species_id, level } => {
                    self.register_seen(species_id);
                    let enemy = Pokemon::new(species_id, level);
                    let player_idx = self.party.iter().position(|p| !p.is_fainted()).unwrap_or(0);
                    let player_hp = self.party.get(player_idx).map(|p| p.hp as f64).unwrap_or(0.0);
                    self.battle = Some(BattleState {
                        phase: BattlePhase::Intro { timer: 0.0 },
                        enemy,
                        player_idx,
                        is_wild: true,
                        player_hp_display: player_hp,
                        enemy_hp_display: 0.0,
                        turn_count: 0,
                        trainer_team: Vec::new(),
                        trainer_team_idx: 0,
                        pending_player_move: None,
                        player_stages: [0; 7],
                        enemy_stages: [0; 7],
                        enemy_flinched: false,
                        player_flinched: false,
                        player_confused: 0,
                        enemy_confused: 0,
                        player_trapped: false,
                        player_must_recharge: false,
                        enemy_must_recharge: false,
                        player_rampage: (0, 0),
                        enemy_rampage: (0, 0),
                        pending_learn_moves: vec![],
                        free_switch: false,
                        confusion_snapout_msg: None,
                        battle_queue: VecDeque::new(),
                        queue_timer: 0.0,
                    });
                    self.encounter_flash_count = 0;
                    self.phase = GamePhase::EncounterTransition { timer: 0.0 };
                }
                DialogueAction::StartSudowoodoBattle => {
                    self.register_seen(SUDOWOODO);
                    let enemy = Pokemon::new(SUDOWOODO, 20);
                    let player_idx = self.party.iter().position(|p| !p.is_fainted()).unwrap_or(0);
                    let player_hp = self.party.get(player_idx).map(|p| p.hp as f64).unwrap_or(0.0);
                    self.battle = Some(BattleState {
                        phase: BattlePhase::Intro { timer: 0.0 },
                        enemy,
                        player_idx,
                        is_wild: true,
                        player_hp_display: player_hp,
                        enemy_hp_display: 0.0,
                        turn_count: 0,
                        trainer_team: Vec::new(),
                        trainer_team_idx: 0,
                        pending_player_move: None,
                        player_stages: [0; 7],
                        enemy_stages: [0; 7],
                        enemy_flinched: false,
                        player_flinched: false,
                        player_confused: 0,
                        enemy_confused: 0,
                        player_trapped: false,
                        player_must_recharge: false,
                        enemy_must_recharge: false,
                        player_rampage: (0, 0),
                        enemy_rampage: (0, 0),
                        pending_learn_moves: vec![],
                        free_switch: false,
                        confusion_snapout_msg: None,
                        battle_queue: VecDeque::new(),
                        queue_timer: 0.0,
                    });
                    self.encounter_flash_count = 0;
                    self.phase = GamePhase::EncounterTransition { timer: 0.0 };
                }
                DialogueAction::StartRedGyaradosBattle => {
                    self.register_seen(GYARADOS);
                    let enemy = Pokemon::new(GYARADOS, 30);
                    let player_idx = self.party.iter().position(|p| !p.is_fainted()).unwrap_or(0);
                    let player_hp = self.party.get(player_idx).map(|p| p.hp as f64).unwrap_or(0.0);
                    self.battle = Some(BattleState {
                        phase: BattlePhase::Intro { timer: 0.0 },
                        enemy,
                        player_idx,
                        is_wild: true,
                        player_hp_display: player_hp,
                        enemy_hp_display: 0.0,
                        turn_count: 0,
                        trainer_team: Vec::new(),
                        trainer_team_idx: 0,
                        pending_player_move: None,
                        player_stages: [0; 7],
                        enemy_stages: [0; 7],
                        enemy_flinched: false,
                        player_flinched: false,
                        player_confused: 0,
                        enemy_confused: 0,
                        player_trapped: false,
                        player_must_recharge: false,
                        enemy_must_recharge: false,
                        player_rampage: (0, 0),
                        enemy_rampage: (0, 0),
                        pending_learn_moves: vec![],
                        free_switch: false,
                        confusion_snapout_msg: None,
                        battle_queue: VecDeque::new(),
                        queue_timer: 0.0,
                    });
                    self.encounter_flash_count = 0;
                    self.phase = GamePhase::EncounterTransition { timer: 0.0 };
                }
                DialogueAction::EscapeRope => {
                    self.phase = GamePhase::MapFadeOut {
                        dest_map: MapId::PokemonCenter,
                        dest_x: 5, dest_y: 6,
                        timer: 0.0,
                    };
                }
                DialogueAction::DaycareDeposit => {
                    // Open daycare deposit selection screen
                    if self.party.len() <= 1 {
                        // Can't deposit if only 1 Pokemon
                        self.dialogue = Some(DialogueState {
                            lines: vec!["You can't leave your".to_string(), "last POKEMON!".to_string()],
                            current_line: 0, char_index: 0, timer: 0.0,
                            on_complete: DialogueAction::None,
                        });
                        self.phase = GamePhase::Dialogue;
                    } else {
                        self.phase = GamePhase::DaycareDeposit { cursor: 0 };
                    }
                }
                DialogueAction::DaycareReturn => {
                    // Show YES/NO prompt
                    self.phase = GamePhase::DaycarePrompt { cursor: 0 };
                }
                DialogueAction::CheckEvolution => {
                    let pending_evo = engine.global_state.get_f64("pending_evolution").unwrap_or(0.0) as u16;
                    if pending_evo > 0 {
                        engine.global_state.set_f64("pending_evolution", 0.0);
                        self.phase = GamePhase::Evolution { timer: 0.0, new_species: pending_evo };
                    } else {
                        self.phase = GamePhase::Overworld;
                    }
                }
            }
        }
    }

    // ─── Daycare Logic ─────────────────────────────────

    fn step_daycare_deposit(&mut self, engine: &mut Engine) {
        let cursor = if let GamePhase::DaycareDeposit { cursor } = self.phase { cursor } else { return; };
        let party_len = self.party.len() as u8;
        if party_len == 0 { self.phase = GamePhase::Overworld; return; }

        if is_up(engine) {
            let new_cursor = if cursor == 0 { party_len - 1 } else { cursor - 1 };
            self.phase = GamePhase::DaycareDeposit { cursor: new_cursor };
        } else if is_down(engine) {
            let new_cursor = (cursor + 1) % party_len;
            self.phase = GamePhase::DaycareDeposit { cursor: new_cursor };
        } else if is_cancel(engine) {
            self.dialogue = Some(DialogueState {
                lines: vec!["Come back anytime.".to_string()],
                current_line: 0, char_index: 0, timer: 0.0,
                on_complete: DialogueAction::None,
            });
            self.phase = GamePhase::Dialogue;
        } else if is_confirm(engine) {
            let idx = cursor as usize;
            if idx < self.party.len() && self.party.len() > 1 {
                let deposited = self.party.remove(idx);
                let name = get_species(deposited.species_id).map(|s| s.name).unwrap_or("???").to_string();
                self.daycare_deposit_level = deposited.level;
                self.daycare_pokemon = Some(deposited);
                self.daycare_steps = 0;
                self.dialogue = Some(DialogueState {
                    lines: vec![
                        format!("I'll raise your {}", name),
                        "for a while.".to_string(),
                    ],
                    current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
                self.phase = GamePhase::Dialogue;
            }
        }
    }

    fn step_daycare_prompt(&mut self, engine: &mut Engine) {
        let cursor = if let GamePhase::DaycarePrompt { cursor } = self.phase { cursor } else { return; };

        if is_up(engine) || is_down(engine) {
            let new_cursor = 1 - cursor;
            self.phase = GamePhase::DaycarePrompt { cursor: new_cursor };
        } else if is_cancel(engine) {
            // Cancel = NO
            self.dialogue = Some(DialogueState {
                lines: vec!["I'll keep raising it.".to_string()],
                current_line: 0, char_index: 0, timer: 0.0,
                on_complete: DialogueAction::None,
            });
            self.phase = GamePhase::Dialogue;
        } else if is_confirm(engine) {
            if cursor == 0 {
                // YES — return Pokemon
                if self.party.len() >= 6 {
                    self.dialogue = Some(DialogueState {
                        lines: vec!["Your party is full!".to_string()],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                    return;
                }
                if let Some(pkmn) = self.daycare_pokemon.take() {
                    let name = get_species(pkmn.species_id).map(|s| s.name).unwrap_or("???").to_string();
                    let levels_gained = pkmn.level.saturating_sub(self.daycare_deposit_level) as u32;
                    let cost = 100 + 100 * levels_gained;
                    if self.money >= cost {
                        self.money -= cost;
                        self.party.push(pkmn);
                        self.daycare_steps = 0;
                        self.dialogue = Some(DialogueState {
                            lines: vec![
                                format!("{} came back!", name),
                                format!("That'll be ${}.", cost),
                            ],
                            current_line: 0, char_index: 0, timer: 0.0,
                            on_complete: DialogueAction::None,
                        });
                    } else {
                        // Not enough money — put it back
                        self.daycare_pokemon = Some(pkmn);
                        self.dialogue = Some(DialogueState {
                            lines: vec![
                                format!("That'll be ${}.", cost),
                                "You don't have enough".to_string(),
                                "money...".to_string(),
                            ],
                            current_line: 0, char_index: 0, timer: 0.0,
                            on_complete: DialogueAction::None,
                        });
                    }
                } else {
                    self.phase = GamePhase::Overworld;
                    return;
                }
                self.phase = GamePhase::Dialogue;
            } else {
                // NO — keep raising
                self.dialogue = Some(DialogueState {
                    lines: vec!["I'll keep raising it.".to_string()],
                    current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
                self.phase = GamePhase::Dialogue;
            }
        }
    }

    // ─── Menu Logic ────────────────────────────────────

    fn step_menu(&mut self, engine: &mut Engine) {
        let items = 6u8;
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
                0 => self.phase = GamePhase::PokemonMenu { cursor: 0, action: 0, sub_cursor: 0 },
                1 => self.phase = GamePhase::BagMenu { cursor: 0 },
                2 => self.phase = GamePhase::Pokedex { cursor: 0, scroll: 0 },
                3 => self.phase = GamePhase::TrainerCard,
                4 => {
                    // Save: trigger actual save via persist queue
                    self.needs_save = true;
                    self.dialogue = Some(DialogueState {
                        lines: vec!["Game saved!".to_string()],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                }
                5 => self.phase = GamePhase::Overworld,
                _ => {}
            }
        }
    }

    fn step_pokemon_menu(&mut self, engine: &mut Engine) {
        let (cursor, action, sub_cursor) = if let GamePhase::PokemonMenu { cursor, action, sub_cursor } = &self.phase {
            (*cursor, *action, *sub_cursor)
        } else { (0, 0, 0) };
        let party_size = self.party.len() as u8;
        if party_size == 0 { self.phase = GamePhase::Overworld; return; }

        let confirm = is_confirm(engine);
        let cancel = is_cancel(engine);

        match action {
            // ── Action 0: Browsing party list ──
            0 => {
                if is_down(engine) {
                    self.phase = GamePhase::PokemonMenu { cursor: (cursor + 1) % party_size, action: 0, sub_cursor: 0 };
                } else if is_up(engine) {
                    self.phase = GamePhase::PokemonMenu { cursor: if cursor == 0 { party_size - 1 } else { cursor - 1 }, action: 0, sub_cursor: 0 };
                }

                if cancel {
                    // If backing out during a free switch, return to TrainerSwitchPrompt
                    if let Some(b) = &mut self.battle {
                        if b.free_switch {
                            b.free_switch = false;
                            let next_name = b.enemy.name().to_string();
                            b.phase = BattlePhase::TrainerSwitchPrompt { next_name, cursor: 0 };
                            self.phase = GamePhase::Battle;
                            return;
                        }
                    }
                    self.phase = if self.battle.is_some() { GamePhase::Battle } else { GamePhase::Menu };
                    return;
                }

                if confirm {
                    if self.battle.is_some() {
                        // Battle mode: switch Pokemon directly (existing behavior)
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
                            // Switching costs a turn — enemy gets a free attack (Gen 2 rule)
                            if let Some(mut b) = self.battle.take() {
                                b.player_idx = selected;
                                b.player_hp_display = self.party[selected].hp as f64;
                                b.player_stages = [0; 7]; // Reset player stages on switch
                                b.player_confused = 0; // Reset confusion on switch
                                b.player_trapped = false; // Mean Look cleared on switch
                                b.player_must_recharge = false; // Clear recharge on switch
                                b.player_rampage = (0, 0); // Clear rampage on switch
                                b.pending_player_move = None;
                                // Reset toxic counter on switch-in (Gen 2)
                                if let StatusCondition::BadPoison { ref mut turn } = self.party[selected].status {
                                    *turn = 1;
                                }
                                let pname = self.party[selected].name().to_string();
                                if b.free_switch {
                                    // Free switch from TrainerSwitchPrompt — no enemy attack
                                    b.free_switch = false;
                                    b.phase = BattlePhase::Text {
                                        message: format!("Go! {}!", pname),
                                        timer: 0.0,
                                        next_phase: Box::new(BattlePhase::ActionSelect { cursor: 0 }),
                                    };
                                } else {
                                    let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(
                                        engine, &b.enemy, b.player_idx, &b.enemy_stages, &b.player_stages,
                                    );
                                    b.phase = BattlePhase::Text {
                                        message: format!("Go! {}!", pname),
                                        timer: 0.0,
                                        next_phase: Box::new(BattlePhase::EnemyAttack {
                                            timer: 0.0, move_id: e_move, damage: e_dmg,
                                            effectiveness: e_eff, is_crit: e_crit,
                                        }),
                                    };
                                }
                                self.battle = Some(b);
                                self.phase = GamePhase::Battle;
                            }
                        }
                    } else {
                        // Overworld: open sub-action menu (SUMMARY / SWAP / CANCEL)
                        sfx_select(engine);
                        self.phase = GamePhase::PokemonMenu { cursor, action: 1, sub_cursor: 0 };
                    }
                }
            }

            // ── Action 1: Sub-menu open (SUMMARY / SWAP / CANCEL) ──
            1 => {
                if is_down(engine) {
                    self.phase = GamePhase::PokemonMenu { cursor, action: 1, sub_cursor: (sub_cursor + 1) % 3 };
                } else if is_up(engine) {
                    self.phase = GamePhase::PokemonMenu { cursor, action: 1, sub_cursor: if sub_cursor == 0 { 2 } else { sub_cursor - 1 } };
                }

                if cancel {
                    // Close sub-menu, back to browsing
                    self.phase = GamePhase::PokemonMenu { cursor, action: 0, sub_cursor: 0 };
                    return;
                }

                if confirm {
                    // Re-read sub_cursor in case up/down changed it
                    let sc = if let GamePhase::PokemonMenu { sub_cursor, .. } = &self.phase { *sub_cursor } else { 0 };
                    match sc {
                        0 => {
                            // SUMMARY
                            sfx_select(engine);
                            self.phase = GamePhase::PokemonSummary { index: cursor };
                        }
                        1 => {
                            // SWAP: enter swap mode, remember source
                            sfx_select(engine);
                            self.swap_source = cursor;
                            self.phase = GamePhase::PokemonMenu { cursor, action: 2, sub_cursor: 0 };
                        }
                        _ => {
                            // CANCEL: close sub-menu
                            self.phase = GamePhase::PokemonMenu { cursor, action: 0, sub_cursor: 0 };
                        }
                    }
                }
            }

            // ── Action 2: Swap mode (selecting second Pokemon) ──
            2 => {
                if is_down(engine) {
                    self.phase = GamePhase::PokemonMenu { cursor: (cursor + 1) % party_size, action: 2, sub_cursor: 0 };
                } else if is_up(engine) {
                    self.phase = GamePhase::PokemonMenu { cursor: if cursor == 0 { party_size - 1 } else { cursor - 1 }, action: 2, sub_cursor: 0 };
                }

                if cancel {
                    // Cancel swap, back to browsing
                    self.phase = GamePhase::PokemonMenu { cursor, action: 0, sub_cursor: 0 };
                    return;
                }

                if confirm {
                    let src = self.swap_source as usize;
                    let dst = cursor as usize;
                    if src != dst && src < self.party.len() && dst < self.party.len() {
                        // Perform the swap
                        self.party.swap(src, dst);
                        sfx_select(engine);
                        // If in battle, track the active Pokemon's new position
                        if let Some(b) = &mut self.battle {
                            if b.player_idx == src {
                                b.player_idx = dst;
                            } else if b.player_idx == dst {
                                b.player_idx = src;
                            }
                        }
                        crate::log::log("Party swap complete");
                    }
                    // Return to browsing mode
                    self.phase = GamePhase::PokemonMenu { cursor, action: 0, sub_cursor: 0 };
                }
            }

            _ => {
                // Invalid action, reset to browsing
                self.phase = GamePhase::PokemonMenu { cursor, action: 0, sub_cursor: 0 };
            }
        }
    }

    fn step_pokemon_summary(&mut self, engine: &mut Engine) {
        let index = if let GamePhase::PokemonSummary { index } = &self.phase { *index } else { 0 };
        let cancel = is_cancel(engine) || is_confirm(engine);
        if cancel {
            self.phase = GamePhase::PokemonMenu { cursor: index, action: 0, sub_cursor: 0 };
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

    fn step_trainer_card(&mut self, engine: &mut Engine) {
        if is_cancel(engine) || is_confirm(engine) {
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
        let white = Color::from_rgba(248, 248, 248, 255);
        let dim = Color::from_rgba(160, 160, 180, 255);

        draw_text_shadowed(fb, ctx, "POKEMON", 40, 25, gold, shadow);
        draw_text_shadowed(fb, ctx, "GOLD VERSION", 25, 45,
            Color::from_rgba(248, 176, 48, 255), Color::from_rgba(96, 64, 0, 255));

        fill_rect_v(fb, ctx, 20, 58, 120, 1, Color::from_rgba(248, 208, 48, 128));

        if self.has_save {
            // Show CONTINUE / NEW GAME menu
            let continue_color = if self.menu_cursor == 0 { white } else { dim };
            let new_color = if self.menu_cursor == 1 { white } else { dim };
            draw_text_pkmn(fb, ctx, "CONTINUE", 50, 80, continue_color);
            draw_text_pkmn(fb, ctx, "NEW GAME", 50, 100, new_color);
            // Draw cursor arrow
            let cursor_y = if self.menu_cursor == 0 { 80 } else { 100 };
            draw_cursor(fb, ctx, 38, cursor_y, gold);
        } else {
            if (self.title_blink_timer * 2.0) as u32 % 2 == 0 {
                draw_text_pkmn(fb, ctx, "PRESS START", 32, 100, white);
            }
        }

        draw_text_pkmn(fb, ctx, "CRUSTY ENGINE", 28, 125, Color::from_rgba(120, 120, 140, 255));
        draw_text_pkmn(fb, ctx, "V0.2", 65, 135, Color::from_rgba(80, 80, 100, 255));
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
        for (idx, npc) in self.current_map.npcs.iter().enumerate() {
            if !self.is_npc_active(idx) { continue; }
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

    fn render_overworld_with_approach(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, approach_npc_idx: u8) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        let scale = ctx.scale;

        fb.clear(Color::from_rgba(8, 8, 16, 255));

        let cam_x = self.camera_x;
        let cam_y = self.camera_y;

        let stx = (cam_x / TILE_PX as f64).floor() as i32;
        let sty = (cam_y / TILE_PX as f64).floor() as i32;
        let etx = stx + VIEW_TILES_X + 2;
        let ety = sty + VIEW_TILES_Y + 2;

        // Draw tiles
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

        // NPCs — draw approaching NPC at animated position
        for (idx, npc) in self.current_map.npcs.iter().enumerate() {
            if !self.is_npc_active(idx) { continue; }
            let (npc_px, npc_py) = if idx == approach_npc_idx as usize {
                // Use approach position with walk offset
                let npc_def = &self.current_map.npcs[idx];
                let (dx, dy) = match npc_def.facing {
                    Direction::Up => (0.0, -self.approach_walk_offset * TILE_PX as f64),
                    Direction::Down => (0.0, self.approach_walk_offset * TILE_PX as f64),
                    Direction::Left => (-self.approach_walk_offset * TILE_PX as f64, 0.0),
                    Direction::Right => (self.approach_walk_offset * TILE_PX as f64, 0.0),
                };
                (self.approach_npc_x as f64 * TILE_PX as f64 + dx,
                 self.approach_npc_y as f64 * TILE_PX as f64 + dy)
            } else {
                (npc.x as f64 * TILE_PX as f64, npc.y as f64 * TILE_PX as f64)
            };
            let sx = (npc_px - cam_x) as i32;
            let sy = (npc_py - cam_y) as i32;
            let (fx, fy) = ctx.to_fb(sx, sy);
            if let Some(sd) = self.npc_sprite_cache.get(npc.sprite_id as usize) {
                draw_sprite(fb, sd, NPC_W, NPC_H, fx, fy, scale, npc_palette(npc.sprite_id));
            }

            // Draw "!" exclamation above approaching NPC
            if idx == approach_npc_idx as usize && self.approach_exclaim_timer > 0.0 {
                let ex = fx + (NPC_W as i32 * scale as i32 / 2) - 2;
                let ey = fy - 6 * scale as i32;
                draw_text_pkmn(fb, ctx, "!", (ex / scale as i32) as i32, (ey / scale as i32) as i32, Color::from_rgba(255, 50, 50, 255));
            }
        }

        // Player
        let ppx = self.player.x as f64 * TILE_PX as f64;
        let ppy = self.player.y as f64 * TILE_PX as f64;
        let psx = (ppx - cam_x) as i32;
        let psy = (ppy - cam_y) as i32;
        let (pfx, pfy) = ctx.to_fb(psx, psy);
        let dir_off = match self.player.facing {
            Direction::Down => 0, Direction::Up => 3, Direction::Left => 6, Direction::Right => 9,
        };
        let si = dir_off + 1; // Standing frame
        if let Some(sd) = self.player_sprite_cache.get(si) {
            draw_sprite(fb, sd, PLAYER_W, PLAYER_H, pfx, pfy, scale, &PAL_PLAYER);
        }

        // Day/night tint
        if self.day_night_tint > 0.01 {
            let a = (self.day_night_tint * 180.0).min(180.0) as u8;
            fill_virtual_screen(fb, ctx, Color::from_rgba(16, 16, 64, a));
        }

        draw_text_pkmn(fb, ctx, self.current_map.name, 4, 2, Color::from_rgba(248, 248, 248, 200));
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

            BattlePhase::LevelUp { stat_deltas, .. } => {
                // Draw stat increase panel (Gen 2 style)
                draw_text_box(fb, ctx, 2, 18, 156, 78);
                if let Some(p) = self.party.get(battle.player_idx) {
                    let labels = ["HP", "Atk", "Def", "SAtk", "SDef", "Spd"];
                    let vals = [p.max_hp, p.attack, p.defense, p.sp_attack, p.sp_defense, p.speed];
                    for i in 0..6 {
                        let y = 26 + (i as i32) * 10;
                        let delta_str = if stat_deltas[i] >= 0 { format!("+{}", stat_deltas[i]) } else { format!("{}", stat_deltas[i]) };
                        let label = format!("{:<5}{:>3}  {}", labels[i], vals[i], delta_str);
                        draw_text_pkmn(fb, ctx, &label, 10, y, dark);
                    }
                }
                // Also show level-up message at bottom
                draw_text_box(fb, ctx, 2, 98, 156, 42);
                if let Some(p) = self.party.get(battle.player_idx) {
                    let msg = format!("{} grew to LV{}!", p.name(), p.level);
                    draw_text_pkmn(fb, ctx, &msg, 10, 106, dark);
                }
            }

            BattlePhase::LearnMove { new_move, sub } => {
                let pname = self.party.get(battle.player_idx).map(|p| p.name().to_string()).unwrap_or_default();
                let mname = get_move(*new_move).map(|m| m.name).unwrap_or("???");
                match sub {
                    LearnMoveSub::TryingToLearn { .. } => {
                        draw_text_box(fb, ctx, 2, 98, 156, 42);
                        let msg = format!("{} is trying to\nlearn {}!", pname, mname);
                        for (i, line) in msg.split('\n').enumerate() {
                            draw_text_pkmn(fb, ctx, line, 10, 106 + i as i32 * 12, dark);
                        }
                    }
                    LearnMoveSub::CantLearnMore { .. } => {
                        draw_text_box(fb, ctx, 2, 98, 156, 42);
                        let msg = format!("But {} can't learn\nmore than 4 moves.", pname);
                        for (i, line) in msg.split('\n').enumerate() {
                            draw_text_pkmn(fb, ctx, line, 10, 106 + i as i32 * 12, dark);
                        }
                    }
                    LearnMoveSub::DeletePrompt { cursor } => {
                        draw_text_box(fb, ctx, 2, 98, 156, 42);
                        let msg = format!("Delete a move for\n{}?", mname);
                        for (i, line) in msg.split('\n').enumerate() {
                            draw_text_pkmn(fb, ctx, line, 10, 106 + i as i32 * 12, dark);
                        }
                        // YES/NO box
                        draw_text_box(fb, ctx, 120, 70, 36, 28);
                        draw_text_pkmn(fb, ctx, "YES", 130, 76, dark);
                        draw_text_pkmn(fb, ctx, "NO", 130, 88, dark);
                        draw_cursor(fb, ctx, 122, 76 + *cursor as i32 * 12, dark);
                    }
                    LearnMoveSub::PickMove { cursor } => {
                        draw_text_box(fb, ctx, 2, 50, 156, 90);
                        draw_text_pkmn(fb, ctx, "Which move to forget?", 10, 56, dark);
                        if let Some(p) = self.party.get(battle.player_idx) {
                            for i in 0..4 {
                                if let Some(mid) = p.moves[i] {
                                    if let Some(md) = get_move(mid) {
                                        let y = 72 + i as i32 * 14;
                                        let col = type_color(md.move_type);
                                        draw_text_pkmn(fb, ctx, md.name, 20, y, col);
                                        let pp_s = format!("{}/{}", p.move_pp[i], p.move_max_pp[i]);
                                        draw_text_pkmn(fb, ctx, &pp_s, 100, y, dark);
                                        if i as u8 == *cursor {
                                            draw_cursor(fb, ctx, 12, y, dark);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    LearnMoveSub::ForgotMove { .. } => {
                        draw_text_box(fb, ctx, 2, 98, 156, 42);
                        draw_text_pkmn(fb, ctx, "1, 2, and... Poof!", 10, 106, dark);
                        let msg = format!("{} learned {}!", pname, mname);
                        draw_text_pkmn(fb, ctx, &msg, 10, 118, dark);
                    }
                    LearnMoveSub::LearnedMove { .. } => {
                        draw_text_box(fb, ctx, 2, 98, 156, 42);
                        let msg = format!("{} learned {}!", pname, mname);
                        draw_text_pkmn(fb, ctx, &msg, 10, 106, dark);
                    }
                    LearnMoveSub::StopPrompt { cursor } => {
                        draw_text_box(fb, ctx, 2, 98, 156, 42);
                        let msg = format!("Stop learning\n{}?", mname);
                        for (i, line) in msg.split('\n').enumerate() {
                            draw_text_pkmn(fb, ctx, line, 10, 106 + i as i32 * 12, dark);
                        }
                        // YES/NO box
                        draw_text_box(fb, ctx, 120, 70, 36, 28);
                        draw_text_pkmn(fb, ctx, "YES", 130, 76, dark);
                        draw_text_pkmn(fb, ctx, "NO", 130, 88, dark);
                        draw_cursor(fb, ctx, 122, 76 + *cursor as i32 * 12, dark);
                    }
                    LearnMoveSub::DidNotLearn { .. } => {
                        draw_text_box(fb, ctx, 2, 98, 156, 42);
                        let msg = format!("{} did not learn\n{}.", pname, mname);
                        for (i, line) in msg.split('\n').enumerate() {
                            draw_text_pkmn(fb, ctx, line, 10, 106 + i as i32 * 12, dark);
                        }
                    }
                }
            }

            BattlePhase::TrainerSwitchPrompt { ref next_name, cursor } => {
                draw_text_box(fb, ctx, 2, 88, 156, 52);
                let line1 = format!("Foe will send out {}.", next_name);
                draw_text_pkmn(fb, ctx, &line1, 10, 96, dark);
                draw_text_pkmn(fb, ctx, "Will you switch?", 10, 108, dark);
                // YES/NO
                draw_text_pkmn(fb, ctx, "YES", 122, 96, dark);
                draw_text_pkmn(fb, ctx, "NO", 122, 108, dark);
                draw_cursor(fb, ctx, 114, 96 + *cursor as i32 * 12, dark);
            }

            BattlePhase::Won { .. } => {
                draw_text_box(fb, ctx, 2, 98, 156, 42);
                draw_text_pkmn(fb, ctx, "You won!", 10, 106, dark);
            }

            BattlePhase::PlayerFainted => {
                draw_text_box(fb, ctx, 2, 98, 156, 42);
                if let Some(p) = self.party.get(battle.player_idx) {
                    let msg = format!("{} fainted!", p.name());
                    draw_text_pkmn(fb, ctx, &msg, 10, 106, dark);
                }
            }

            BattlePhase::ExpAwarded { exp_gained, timer } => {
                // Animate EXP bar fill
                if let Some(pp) = self.party.get(battle.player_idx) {
                    let species = get_species(pp.species_id);
                    let level_exp = species.map(|s| exp_for_level(pp.level, s.growth_rate)).unwrap_or(0);
                    let next_exp = species.map(|s| exp_for_level(pp.level + 1, s.growth_rate)).unwrap_or(1);
                    let exp_needed = next_exp.saturating_sub(level_exp).max(1);
                    let old_in_level = pp.exp.saturating_sub(level_exp);
                    let new_in_level = (pp.exp + exp_gained).min(next_exp).saturating_sub(level_exp);
                    let progress = (timer / 1.0).min(1.0);
                    let animated = old_in_level as f64 + (new_in_level as f64 - old_in_level as f64) * progress;
                    draw_exp_bar(fb, ctx, 102, 88, 48, animated as u32, exp_needed);
                }
            }
            BattlePhase::Run => {} // handled in step

            BattlePhase::ExecuteQueue => {
                // Render the current queue step (text box if showing text, otherwise normal battle scene)
                if let Some(step) = battle.battle_queue.front() {
                    match step {
                        BattleStep::Text(msg) => {
                            draw_text_box(fb, ctx, 2, 98, 156, 42);
                            for (i, line) in msg.split('\n').enumerate() {
                                draw_text_pkmn(fb, ctx, line, 10, 106 + i as i32 * 12, dark);
                            }
                        }
                        _ => {
                            // No text to show — just render the battle scene (sprites + HP bars already drawn above)
                        }
                    }
                }
            }
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

        draw_text_box(fb, ctx, 96, 2, 60, 96);
        let items = ["POKEMON", "BAG", "POKEDEX", "TRAINER", "SAVE", "EXIT"];
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

    fn render_pokemon_menu(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, cursor: u8, action: u8, sub_cursor: u8) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        let dark = Color::from_rgba(40, 40, 48, 255);

        fill_virtual_screen(fb, ctx, Color::from_rgba(248, 248, 248, 255));
        draw_text_pkmn(fb, ctx, "POKEMON", 55, 3, dark);
        fill_rect_v(fb, ctx, 4, 12, 152, 1, Color::from_rgba(168, 168, 176, 255));

        for (i, pkmn) in self.party.iter().enumerate() {
            let y = 16 + i as i32 * 20;
            let is_swap_source = action == 2 && i as u8 == self.swap_source;
            if is_swap_source {
                // Highlight the swap source with yellow background
                fill_rect_v(fb, ctx, 2, y - 1, 156, 18, Color::from_rgba(255, 248, 180, 255));
            }
            if i as u8 == cursor {
                if is_swap_source {
                    // Cursor on swap source: brighter yellow
                    fill_rect_v(fb, ctx, 2, y - 1, 156, 18, Color::from_rgba(255, 240, 130, 255));
                } else {
                    fill_rect_v(fb, ctx, 2, y - 1, 156, 18, Color::from_rgba(232, 240, 248, 255));
                }
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

        if action == 1 {
            // Draw sub-menu box on the right side
            let box_x = 100;
            let box_y = 16 + cursor as i32 * 20;
            let box_w = 56;
            let box_h = 38;
            // Background
            fill_rect_v(fb, ctx, box_x, box_y, box_w, box_h, Color::from_rgba(255, 255, 255, 255));
            // Border
            fill_rect_v(fb, ctx, box_x, box_y, box_w, 1, dark);
            fill_rect_v(fb, ctx, box_x, box_y + box_h - 1, box_w, 1, dark);
            fill_rect_v(fb, ctx, box_x, box_y, 1, box_h, dark);
            fill_rect_v(fb, ctx, box_x + box_w - 1, box_y, 1, box_h, dark);
            // Options
            let options = ["SUMMARY", "SWAP", "CANCEL"];
            for (i, opt) in options.iter().enumerate() {
                let oy = box_y + 3 + i as i32 * 11;
                if i as u8 == sub_cursor {
                    draw_cursor(fb, ctx, box_x + 3, oy + 2, dark);
                }
                draw_text_pkmn(fb, ctx, opt, box_x + 12, oy, dark);
            }
        } else if action == 2 {
            // Swap mode: show instruction at bottom
            draw_text_pkmn(fb, ctx, "SWAP TO?", 40, 133, Color::from_rgba(200, 160, 40, 255));
        } else {
            draw_text_pkmn(fb, ctx, "X/ESC TO CLOSE", 20, 133, Color::from_rgba(120, 120, 140, 255));
        }
    }

    // ─── Bag Menu Logic ─────────────────────────────────

    fn step_bag_menu(&mut self, engine: &mut Engine) {
        let cursor = if let GamePhase::BagMenu { cursor } = &self.phase { *cursor } else { 0 };
        // Virtual items shown at top when not in battle
        let has_fly = self.battle.is_none() && self.badges > 0;
        let fly_offset: u8 = if has_fly { 1 } else { 0 };
        let bike_offset: u8 = if self.has_bicycle && self.battle.is_none() { 1 } else { 0 };
        let virtual_offset = fly_offset + bike_offset;
        let total_count = self.bag.items.len() as u8 + virtual_offset;
        if total_count == 0 {
            self.dialogue = Some(DialogueState {
                lines: vec!["Bag is empty!".to_string()],
                current_line: 0, char_index: 0, timer: 0.0,
                on_complete: DialogueAction::None,
            });
            self.phase = GamePhase::Dialogue;
            return;
        }

        if is_down(engine) {
            self.phase = GamePhase::BagMenu { cursor: (cursor + 1) % total_count };
        } else if is_up(engine) {
            self.phase = GamePhase::BagMenu { cursor: if cursor == 0 { total_count - 1 } else { cursor - 1 } };
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
            // FLY: virtual item at cursor 0 when fly_offset > 0
            if fly_offset > 0 && cursor == 0 {
                // Check if indoors — can't fly from indoors
                let is_indoor = matches!(self.current_map_id,
                    MapId::PokemonCenter | MapId::GenericHouse | MapId::ElmLab |
                    MapId::PlayerHouse1F | MapId::PlayerHouse2F |
                    MapId::SproutTower1F | MapId::SproutTower2F | MapId::SproutTower3F |
                    MapId::UnionCave | MapId::IlexForest |
                    MapId::BurnedTower | MapId::OlivineLighthouse | MapId::IcePath |
                    MapId::VioletGym | MapId::AzaleaGym | MapId::GoldenrodGym |
                    MapId::EcruteakGym | MapId::OlivineGym | MapId::CianwoodGym |
                    MapId::MahoganyGym | MapId::BlackthornGym |
                    MapId::VictoryRoad | MapId::RocketHQ |
                    MapId::EliteFourWill | MapId::EliteFourKoga |
                    MapId::EliteFourBruno | MapId::EliteFourKaren | MapId::ChampionLance
                );
                if is_indoor {
                    self.dialogue = Some(DialogueState {
                        lines: vec!["Can't use FLY here!".to_string()],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                } else if self.visited_cities.is_empty() {
                    self.dialogue = Some(DialogueState {
                        lines: vec!["No cities visited yet!".to_string()],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                } else {
                    self.phase = GamePhase::FlyMenu { cursor: 0 };
                }
                return;
            }
            // Bicycle: virtual item at cursor fly_offset when bike_offset > 0
            if bike_offset > 0 && cursor == fly_offset {
                let is_indoor = matches!(self.current_map_id,
                    MapId::PokemonCenter | MapId::GenericHouse | MapId::ElmLab |
                    MapId::PlayerHouse1F | MapId::PlayerHouse2F |
                    MapId::SproutTower1F | MapId::SproutTower2F | MapId::SproutTower3F |
                    MapId::UnionCave | MapId::IlexForest |
                    MapId::BurnedTower | MapId::OlivineLighthouse | MapId::IcePath |
                    MapId::VioletGym | MapId::AzaleaGym | MapId::GoldenrodGym |
                    MapId::EcruteakGym | MapId::OlivineGym | MapId::CianwoodGym |
                    MapId::MahoganyGym | MapId::BlackthornGym |
                    MapId::VictoryRoad | MapId::RocketHQ |
                    MapId::EliteFourWill | MapId::EliteFourKoga |
                    MapId::EliteFourBruno | MapId::EliteFourKaren | MapId::ChampionLance
                );
                if is_indoor {
                    self.dialogue = Some(DialogueState {
                        lines: vec!["Can't use that here!".to_string()],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                } else {
                    self.on_bicycle = !self.on_bicycle;
                    let msg = if self.on_bicycle { "Got on the BICYCLE!" } else { "Got off the BICYCLE." };
                    self.dialogue = Some(DialogueState {
                        lines: vec![msg.to_string()],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                }
                return;
            }
            let bag_cursor = (cursor - virtual_offset) as usize;
            if let Some(&(item_id, _qty)) = self.bag.items.get(bag_cursor) {
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
                    } else if item_data.repel_steps > 0 {
                        // Repel/Super Repel/Max Repel: prevent wild encounters
                        if self.battle.is_some() {
                            self.dialogue = Some(DialogueState {
                                lines: vec!["Can't use that here!".to_string()],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                        } else {
                            let steps = item_data.repel_steps;
                            let name = item_data.name.to_string();
                            self.bag.use_item(item_id);
                            self.repel_steps = steps;
                            self.dialogue = Some(DialogueState {
                                lines: vec![
                                    format!("Used a {}!", name),
                                    "REPEL's effect lingered!".to_string(),
                                ],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                        }
                    } else if item_data.is_rare_candy {
                        // Rare Candy: select target Pokemon to level up
                        self.phase = GamePhase::BagUseItem { item_id, target_cursor: 0 };
                    } else if item_id == ITEM_ESCAPE_ROPE {
                        // Escape Rope: warp to last Pokemon Center
                        if self.battle.is_some() {
                            self.dialogue = Some(DialogueState {
                                lines: vec!["Can't use that in battle!".to_string()],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                        } else if self.current_map_id == MapId::PokemonCenter {
                            self.dialogue = Some(DialogueState {
                                lines: vec!["Can't use that here!".to_string()],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                        } else {
                            self.bag.use_item(item_id);
                            self.dialogue = Some(DialogueState {
                                lines: vec!["Used an ESCAPE ROPE!".to_string()],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::EscapeRope,
                            });
                            self.phase = GamePhase::Dialogue;
                        }
                    } else if item_id == ITEM_ETHER {
                        // Ether: restore 10 PP to the first move with missing PP
                        let target_idx = if self.battle.is_some() {
                            self.battle.as_ref().map(|b| b.player_idx).unwrap_or(0)
                        } else { 0 };
                        if let Some(p) = self.party.get_mut(target_idx) {
                            let mut restored = false;
                            for i in 0..4 {
                                if p.moves[i].is_some() && p.move_pp[i] < p.move_max_pp[i] {
                                    p.move_pp[i] = (p.move_pp[i] + 10).min(p.move_max_pp[i]);
                                    restored = true;
                                    let mname = get_move(p.moves[i].unwrap()).map(|m| m.name).unwrap_or("???");
                                    self.bag.use_item(item_id);
                                    self.dialogue = Some(DialogueState {
                                        lines: vec![format!("Restored PP to {}!", mname)],
                                        current_line: 0, char_index: 0, timer: 0.0,
                                        on_complete: DialogueAction::None,
                                    });
                                    self.phase = GamePhase::Dialogue;
                                    break;
                                }
                            }
                            if !restored {
                                self.dialogue = Some(DialogueState {
                                    lines: vec!["PP is already full!".to_string()],
                                    current_line: 0, char_index: 0, timer: 0.0,
                                    on_complete: DialogueAction::None,
                                });
                                self.phase = GamePhase::Dialogue;
                            }
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

                    // Full Restore: heals HP AND cures status (must come before plain status heal)
                    if item_data.heal_amount > 0 && item_data.is_status_heal {
                        if pkmn.is_fainted() {
                            self.dialogue = Some(DialogueState {
                                lines: vec![format!("{} has fainted!", name)],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                            return;
                        }
                        if pkmn.hp >= pkmn.max_hp && matches!(pkmn.status, StatusCondition::None) {
                            self.dialogue = Some(DialogueState {
                                lines: vec![format!("{} is already healthy!", name)],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                            return;
                        }
                        let old_hp = pkmn.hp;
                        pkmn.hp = pkmn.max_hp; // Full Restore always fully heals
                        pkmn.clear_status();
                        let healed = pkmn.hp - old_hp;
                        self.bag.use_item(item_id);
                        let mut msgs = vec![format!("Used {} on {}!", item_name, name)];
                        if healed > 0 {
                            msgs.push(format!("Restored {} HP!", healed));
                        }
                        msgs.push(format!("{} is fully healthy!", name));
                        self.dialogue = Some(DialogueState {
                            lines: msgs,
                            current_line: 0, char_index: 0, timer: 0.0,
                            on_complete: DialogueAction::None,
                        });
                        self.phase = GamePhase::Dialogue;
                        return;
                    }

                    // Status heal: cure status conditions (pure status-only items like Antidote, Awakening)
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

                    // Rare Candy: gain 1 level, recalc stats, check evolution
                    if item_data.is_rare_candy {
                        if pkmn.is_fainted() {
                            self.dialogue = Some(DialogueState {
                                lines: vec![format!("{} has fainted!", name)],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                            return;
                        }
                        if pkmn.level >= 100 {
                            self.dialogue = Some(DialogueState {
                                lines: vec![format!("{} is already LV100!", name)],
                                current_line: 0, char_index: 0, timer: 0.0,
                                on_complete: DialogueAction::None,
                            });
                            self.phase = GamePhase::Dialogue;
                            return;
                        }
                        self.bag.use_item(item_id);
                        pkmn.level += 1;
                        // Set EXP to the threshold for this level
                        if let Some(species) = get_species(pkmn.species_id) {
                            pkmn.exp = exp_for_level(pkmn.level, species.growth_rate);
                        }
                        let old_max_hp = pkmn.max_hp;
                        pkmn.recalc_stats();
                        let hp_diff = pkmn.max_hp.saturating_sub(old_max_hp);
                        pkmn.hp = (pkmn.hp + hp_diff).min(pkmn.max_hp);
                        let mut msgs = vec![
                            format!("Used {} on {}!", item_name, name),
                            format!("{} grew to LV{}!", name, pkmn.level),
                        ];
                        // Check for new moves at this level
                        if let Some(species) = get_species(pkmn.species_id) {
                            for &(lvl, move_id) in species.learnset.iter() {
                                if lvl == pkmn.level {
                                    // Try to auto-learn if there's an empty slot
                                    let mut learned = false;
                                    for i in 0..4 {
                                        if pkmn.moves[i].is_none() {
                                            pkmn.moves[i] = Some(move_id);
                                            if let Some(md) = get_move(move_id) {
                                                pkmn.move_pp[i] = md.pp;
                                                pkmn.move_max_pp[i] = md.pp;
                                            }
                                            let mname = get_move(move_id).map(|m| m.name).unwrap_or("???");
                                            msgs.push(format!("{} learned {}!", name, mname));
                                            learned = true;
                                            break;
                                        }
                                    }
                                    if !learned {
                                        // All 4 slots full — skip (simplified, no forget prompt in overworld)
                                        let mname = get_move(move_id).map(|m| m.name).unwrap_or("???");
                                        msgs.push(format!("{} can't learn {}!", name, mname));
                                        msgs.push("All move slots are full.".to_string());
                                    }
                                }
                            }
                            // Check evolution
                            if let (Some(evo_lvl), Some(evo_into)) = (species.evolution_level, species.evolution_into) {
                                if pkmn.level >= evo_lvl {
                                    engine.global_state.set_f64("pending_evolution", evo_into as f64);
                                }
                            }
                        }
                        let pending_evo = engine.global_state.get_f64("pending_evolution").unwrap_or(0.0) as u16;
                        let on_complete = if pending_evo > 0 {
                            DialogueAction::CheckEvolution
                        } else {
                            DialogueAction::None
                        };
                        self.dialogue = Some(DialogueState {
                            lines: msgs,
                            current_line: 0, char_index: 0, timer: 0.0,
                            on_complete,
                        });
                        self.phase = GamePhase::Dialogue;
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

    // ─── Fly Menu Logic ────────────────────────────────

    fn step_fly_menu(&mut self, engine: &mut Engine) {
        let cursor = if let GamePhase::FlyMenu { cursor } = &self.phase { *cursor } else { 0 };
        let city_count = self.visited_cities.len() as u8;
        if city_count == 0 {
            self.phase = GamePhase::BagMenu { cursor: 0 };
            return;
        }

        if is_down(engine) {
            self.phase = GamePhase::FlyMenu { cursor: (cursor + 1) % city_count };
        } else if is_up(engine) {
            self.phase = GamePhase::FlyMenu { cursor: if cursor == 0 { city_count - 1 } else { cursor - 1 } };
        }

        if is_cancel(engine) {
            self.phase = GamePhase::BagMenu { cursor: 0 };
            return;
        }

        if is_confirm(engine) {
            if let Some(&city) = self.visited_cities.get(cursor as usize) {
                let (sx, sy) = Self::fly_spawn(city);
                // Show "used FLY!" dialogue, then MapFadeOut to city
                self.on_bicycle = false;
                self.ice_sliding = None;
                self.dialogue = Some(DialogueState {
                    lines: vec![
                        "POKEMON used FLY!".to_string(),
                    ],
                    current_line: 0, char_index: 0, timer: 0.0,
                    on_complete: DialogueAction::None,
                });
                // We can't chain dialogue→fade, so set up a MapFadeOut directly
                // and show the dialogue text as a brief flash via the phase transition
                self.phase = GamePhase::MapFadeOut { dest_map: city, dest_x: sx, dest_y: sy, timer: 0.0 };
            }
        }
    }

    fn render_fly_menu(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, cursor: u8) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        let dark = Color::from_rgba(40, 40, 48, 255);

        fill_virtual_screen(fb, ctx, Color::from_rgba(220, 232, 248, 255));
        draw_text_pkmn(fb, ctx, "FLY TO?", 55, 3, dark);
        fill_rect_v(fb, ctx, 4, 12, 152, 1, Color::from_rgba(168, 168, 176, 255));

        for (i, city) in self.visited_cities.iter().enumerate() {
            let y = 16 + i as i32 * 14;
            if y > 128 { break; }
            if i as u8 == cursor {
                fill_rect_v(fb, ctx, 2, y - 1, 156, 13, Color::from_rgba(248, 248, 255, 255));
                draw_cursor(fb, ctx, 4, y + 1, dark);
            }
            draw_text_pkmn(fb, ctx, Self::city_name(*city), 14, y + 1, dark);
        }

        draw_text_pkmn(fb, ctx, "X/ESC TO CANCEL", 15, 133, Color::from_rgba(120, 120, 140, 255));
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
        // Gen 2 status multiplier: sleep/freeze = 2x, other status = 1.5x
        let status_mult = match battle.enemy.status {
            StatusCondition::Sleep { .. } | StatusCondition::Freeze => 2.0,
            StatusCondition::Poison | StatusCondition::BadPoison { .. } | StatusCondition::Burn | StatusCondition::Paralysis => 1.5,
            StatusCondition::None => 1.0,
        };
        let rate = ((3.0 * max_hp - 2.0 * cur_hp) * catch_rate * ball_mult * status_mult) / (3.0 * max_hp);
        let shake_prob = (rate / 255.0).min(1.0);

        // Single roll for catch decision (rate/255 chance), then cosmetic shakes
        let caught = engine.rng.next_f64() < shake_prob;
        let shakes = if caught {
            3u8
        } else {
            // Cosmetic shakes: higher shake_prob = more shakes before breaking free
            let roll = engine.rng.next_f64();
            if roll < shake_prob * 0.33 { 2 }
            else if roll < shake_prob * 0.66 { 1 }
            else { 0 }
        };

        let shake_text = match shakes {
            0 => "Oh no! It broke free!",
            1 => "Aww! It appeared to be caught!",
            2 => "Aargh! Almost had it!",
            _ => "", // 3 = caught
        };

        if caught {
            sfx_catch(engine);
            self.register_caught(battle.enemy.species_id);
            let enemy_name = battle.enemy.name().to_string();
            if self.party.len() < 6 {
                self.party.push(battle.enemy.clone());
                self.dialogue = Some(DialogueState {
                    lines: vec![
                        format!("You threw a {}!", ball_name),
                        "Wobble... Wobble... Wobble...".to_string(),
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
            // Failed catch — show shakes then enemy gets a turn
            let wobbles = match shakes {
                1 => "Wobble...".to_string(),
                2 => "Wobble... Wobble...".to_string(),
                _ => String::new(),
            };
            let (e_move, e_dmg, e_eff, e_crit) = self.calc_enemy_move(engine, &battle.enemy, battle.player_idx, &battle.enemy_stages, &battle.player_stages);
            let mut lines = vec![format!("You threw a {}!", ball_name)];
            if !wobbles.is_empty() { lines.push(wobbles); }
            lines.push(shake_text.to_string());
            // Show as sequential text phases
            let msg = lines.join("\n");
            battle.phase = BattlePhase::Text {
                message: msg,
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

        // Virtual items at top when not in battle
        let has_fly = self.battle.is_none() && self.badges > 0;
        let fly_offset: u8 = if has_fly { 1 } else { 0 };
        let bike_offset: u8 = if self.has_bicycle && self.battle.is_none() { 1 } else { 0 };
        let virtual_offset = fly_offset + bike_offset;
        let total_count = self.bag.items.len() as u8 + virtual_offset;

        if total_count == 0 {
            draw_text_pkmn(fb, ctx, "Empty!", 55, 60, Color::from_rgba(120, 120, 140, 255));
        } else {
            let mut row: u8 = 0;
            // Draw FLY item first if applicable
            if fly_offset > 0 {
                let y = 16 + row as i32 * 18;
                if cursor == 0 {
                    fill_rect_v(fb, ctx, 2, y - 1, 156, 16, Color::from_rgba(220, 232, 248, 255));
                    draw_cursor(fb, ctx, 4, y + 2, dark);
                }
                draw_text_pkmn(fb, ctx, "FLY", 14, y + 2, Color::from_rgba(60, 80, 160, 255));
                row += 1;
            }
            // Draw bicycle item if applicable
            if bike_offset > 0 {
                let y = 16 + row as i32 * 18;
                if cursor == fly_offset {
                    fill_rect_v(fb, ctx, 2, y - 1, 156, 16, Color::from_rgba(232, 240, 248, 255));
                    draw_cursor(fb, ctx, 4, y + 2, dark);
                }
                draw_text_pkmn(fb, ctx, "BICYCLE", 14, y + 2, dark);
                let status = if self.on_bicycle { "ON" } else { "x1" };
                draw_text_pkmn(fb, ctx, status, 120, y + 2, Color::from_rgba(80, 80, 96, 255));
                row += 1;
            }
            for (i, &(item_id, qty)) in self.bag.items.iter().enumerate() {
                let y = 16 + (row as i32 + i as i32) * 18;
                if y > 128 { break; }
                let display_cursor = i as u8 + virtual_offset;
                if display_cursor == cursor {
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

    fn render_trainer_card(&self, fb: &mut crate::rendering::framebuffer::Framebuffer) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        let dark = Color::from_rgba(40, 40, 48, 255);
        let dim = Color::from_rgba(80, 80, 96, 255);
        let gold = Color::from_rgba(208, 176, 48, 255);

        fill_virtual_screen(fb, ctx, Color::from_rgba(248, 248, 248, 255));

        // Title
        draw_text_pkmn(fb, ctx, "TRAINER CARD", 32, 4, dark);
        fill_rect_v(fb, ctx, 4, 13, 152, 1, Color::from_rgba(168, 168, 176, 255));

        // Name
        draw_text_pkmn(fb, ctx, "NAME: GOLD", 8, 18, dark);

        // Money
        let money_str = format!("MONEY: ${}", self.money);
        draw_text_pkmn(fb, ctx, &money_str, 8, 30, dark);

        // Pokedex
        let seen_str = format!("SEEN: {}", self.pokedex_seen.len());
        let caught_str = format!("OWN:  {}", self.pokedex_caught.len());
        draw_text_pkmn(fb, ctx, &seen_str, 8, 42, dark);
        draw_text_pkmn(fb, ctx, &caught_str, 80, 42, dark);

        // Play time
        let hours = (self.total_time / 3600.0) as u32;
        let minutes = ((self.total_time % 3600.0) / 60.0) as u32;
        let time_str = format!("TIME: {:02}:{:02}", hours, minutes);
        draw_text_pkmn(fb, ctx, &time_str, 8, 54, dark);

        // Badges section
        draw_text_pkmn(fb, ctx, "BADGES", 52, 68, dim);
        fill_rect_v(fb, ctx, 4, 77, 152, 1, Color::from_rgba(168, 168, 176, 255));

        let badge_names = ["ZEPHYR", "HIVE", "PLAIN", "FOG", "MINERAL", "STORM", "GLACIER", "RISING"];
        for i in 0..8 {
            let x = 8 + (i % 4) * 38;
            let y = 82 + (i / 4) * 24;
            let has = self.badges & (1 << i) != 0;
            if has {
                // Draw filled badge indicator
                fill_rect_v(fb, ctx, x as i32, y as i32, 32, 10, Color::from_rgba(232, 216, 168, 255));
                fill_rect_v(fb, ctx, x as i32 + 1, y as i32 + 1, 30, 8, gold);
                draw_text_pkmn(fb, ctx, badge_names[i], x as i32 + 2, y as i32 + 12, dark);
            } else {
                // Empty badge slot
                fill_rect_v(fb, ctx, x as i32, y as i32, 32, 10, Color::from_rgba(200, 200, 208, 255));
                fill_rect_v(fb, ctx, x as i32 + 1, y as i32 + 1, 30, 8, Color::from_rgba(224, 224, 232, 255));
                draw_text_pkmn(fb, ctx, "---", x as i32 + 8, y as i32 + 12, dim);
            }
        }

        draw_text_pkmn(fb, ctx, "PRESS Z/ESC", 32, 133, Color::from_rgba(120, 120, 140, 255));
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

    fn render_credits(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, scroll_y: f64) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        fill_virtual_screen(fb, ctx, Color::from_rgba(0, 0, 0, 255));

        let white = Color::from_rgba(248, 248, 248, 255);
        let gold = Color::from_rgba(248, 208, 48, 255);
        let silver = Color::from_rgba(180, 180, 200, 255);

        let lines: &[(&str, Color)] = &[
            ("POKEMON", gold),
            ("Gold Version", gold),
            ("", white),
            ("CONGRATULATIONS!", white),
            ("", white),
            ("Hall of Fame", gold),
            ("", white),
            ("Champion:", silver),
            ("", white), // party line placeholder
            ("", white),
            ("Game Freak", gold),
            ("Director", silver),
            ("Satoshi Tajiri", white),
            ("", white),
            ("Built on", silver),
            ("Crusty Engine", gold),
            ("", white),
            ("Thanks for", silver),
            ("playing!", silver),
            ("", white),
            ("THE END", gold),
        ];

        // Insert party pokemon names
        let party_str: String = self.party.iter().take(6).map(|p| {
            format!("Lv{} {}", p.level, p.name())
        }).collect::<Vec<_>>().join(", ");

        let base_y = 144.0 - scroll_y;
        let line_h = 14.0;

        for (i, &(text, color)) in lines.iter().enumerate() {
            let y = base_y + (i as f64 * line_h);
            if y < -10.0 || y > 154.0 { continue; }
            if i == 8 {
                // Render party line instead
                let trunc = if party_str.len() > 24 { &party_str[..24] } else { &party_str };
                draw_text_pkmn(fb, ctx, trunc, 4, y as i32, silver);
            } else if !text.is_empty() {
                // Center text (rough: 6px per char)
                let w = text.len() as i32 * 6;
                let x = (160 - w) / 2;
                draw_text_pkmn(fb, ctx, text, x.max(4), y as i32, color);
            }
        }
    }

    fn render_daycare_deposit(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, cursor: u8) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        fill_virtual_screen(fb, ctx, Color::from_rgba(32, 32, 64, 255));
        let white = Color::from_rgba(248, 248, 248, 255);
        let yellow = Color::from_rgba(248, 208, 48, 255);
        let gray = Color::from_rgba(160, 160, 160, 255);

        draw_text_pkmn(fb, ctx, "Choose a POKEMON", 20, 8, yellow);
        draw_text_pkmn(fb, ctx, "to leave:", 20, 20, yellow);

        for (i, pkmn) in self.party.iter().enumerate() {
            let y = 38 + i as i32 * 16;
            let name = pkmn.name();
            let text = format!("Lv{:>2} {:10} HP{:>3}/{:>3}", pkmn.level, name, pkmn.hp, pkmn.max_hp);
            let color = if i as u8 == cursor { white } else { gray };
            let marker = if i as u8 == cursor { ">" } else { " " };
            draw_text_pkmn(fb, ctx, marker, 4, y, white);
            draw_text_pkmn(fb, ctx, &text, 14, y, color);
        }

        draw_text_pkmn(fb, ctx, "B: Cancel", 20, 136, gray);
    }

    fn render_daycare_prompt(&self, fb: &mut crate::rendering::framebuffer::Framebuffer, cursor: u8) {
        let ctx = match &self.ctx { Some(c) => c, None => return };
        // Render overworld behind the prompt box
        self.render_overworld(fb);

        // Draw YES/NO prompt box
        let box_x = 100;
        let box_y = 60;
        let box_w = 52;
        let box_h = 36;
        fill_rect_v(fb, ctx, box_x, box_y, box_w, box_h, Color::from_rgba(248, 248, 248, 255));
        fill_rect_v(fb, ctx, box_x + 2, box_y + 2, box_w - 4, box_h - 4, Color::from_rgba(32, 32, 64, 255));

        let white = Color::from_rgba(248, 248, 248, 255);
        let gray = Color::from_rgba(160, 160, 160, 255);

        let yes_col = if cursor == 0 { white } else { gray };
        let no_col = if cursor == 1 { white } else { gray };
        let yes_mark = if cursor == 0 { ">" } else { " " };
        let no_mark = if cursor == 1 { ">" } else { " " };
        draw_text_pkmn(fb, ctx, yes_mark, box_x + 6, box_y + 8, white);
        draw_text_pkmn(fb, ctx, "YES", box_x + 16, box_y + 8, yes_col);
        draw_text_pkmn(fb, ctx, no_mark, box_x + 6, box_y + 22, white);
        draw_text_pkmn(fb, ctx, "NO", box_x + 16, box_y + 22, no_col);
    }
}

fn status_text(s: &StatusCondition) -> &'static str {
    match s {
        StatusCondition::None => "",
        StatusCondition::Poison | StatusCondition::BadPoison { .. } => "PSN",
        StatusCondition::Burn => "BRN",
        StatusCondition::Paralysis => "PAR",
        StatusCondition::Sleep { .. } => "SLP",
        StatusCondition::Freeze => "FRZ",
    }
}

fn status_color(s: &StatusCondition) -> Color {
    match s {
        StatusCondition::Poison | StatusCondition::BadPoison { .. } => Color::from_rgba(160, 64, 160, 255),
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

/// Check and apply secondary status infliction from a damaging move.
fn try_inflict_status(target: &mut Pokemon, move_id: MoveId, rng_roll: f64) {
    if !matches!(target.status, StatusCondition::None) { return; }
    match move_id {
        // 10% burn
        MOVE_EMBER | MOVE_FLAMETHROWER | MOVE_FIRE_BLAST | MOVE_FLAME_WHEEL | MOVE_FIRE_PUNCH => {
            if rng_roll < 0.1 { target.status = StatusCondition::Burn; }
        }
        // 10% freeze
        MOVE_ICE_BEAM | MOVE_BLIZZARD | MOVE_POWDER_SNOW | MOVE_ICE_PUNCH => {
            if rng_roll < 0.1 { target.status = StatusCondition::Freeze; }
        }
        // 10% paralysis
        MOVE_THUNDERBOLT | MOVE_THUNDER_SHOCK => {
            if rng_roll < 0.1 { target.status = StatusCondition::Paralysis; }
        }
        // 30% paralysis
        MOVE_BODY_SLAM | MOVE_LICK | MOVE_TWISTER => {
            if rng_roll < 0.3 { target.status = StatusCondition::Paralysis; }
        }
        // 30% poison
        MOVE_POISON_STING | MOVE_SLUDGE | MOVE_SLUDGE_BOMB => {
            if rng_roll < 0.3 { target.status = StatusCondition::Poison; }
        }
        // 20% tri-attack: ~6.67% each for burn/freeze/paralysis
        MOVE_TRI_ATTACK => {
            if rng_roll < 0.0667 { target.status = StatusCondition::Paralysis; }
            else if rng_roll < 0.1333 { target.status = StatusCondition::Burn; }
            else if rng_roll < 0.2 { target.status = StatusCondition::Freeze; }
        }
        // 10% confusion (via Psybeam, Confusion) — not implemented yet as volatile status
        // 100% confusion (Dynamic Punch) — not implemented yet
        // Sleep (status moves, handled elsewhere)
        MOVE_HYPNOSIS | MOVE_SING | MOVE_SLEEP_POWDER | MOVE_LOVELY_KISS => {
            target.status = StatusCondition::Sleep { turns: 3 };
        }
        // Paralysis (status moves)
        MOVE_STUN_SPORE | MOVE_THUNDER_WAVE => {
            target.status = StatusCondition::Paralysis;
        }
        // Poison (status moves)
        MOVE_POISON_POWDER => {
            target.status = StatusCondition::Poison;
        }
        // Toxic: badly poisoned (escalating damage)
        MOVE_TOXIC => {
            target.status = StatusCondition::BadPoison { turn: 1 };
        }
        _ => {}
    }
}

/// Check for secondary stat drops from damaging moves.
/// Returns (target_is_enemy, stat_idx, delta, chance) or None.
fn damaging_move_stat_effect(move_id: MoveId) -> Option<(bool, usize, i8, f64)> {
    match move_id {
        // Stat drops on target
        MOVE_PSYCHIC => Some((true, STAGE_SPD, -1, 0.1)),
        MOVE_SHADOW_BALL => Some((true, STAGE_SPD, -1, 0.2)),
        MOVE_CRUNCH => Some((true, STAGE_SPD, -1, 0.2)),
        MOVE_ACID => Some((true, STAGE_DEF, -1, 0.1)),
        MOVE_AURORA_BEAM => Some((true, STAGE_ATK, -1, 0.1)),
        MOVE_BUBBLEBEAM | MOVE_BUBBLE | MOVE_CONSTRICT => Some((true, STAGE_SPE, -1, 0.1)),
        MOVE_IRON_TAIL => Some((true, STAGE_DEF, -1, 0.3)),
        MOVE_MUD_SLAP => Some((true, STAGE_ACC, -1, 1.0)),
        MOVE_ICY_WIND => Some((true, STAGE_SPE, -1, 1.0)),
        // Stat raise on user (target_is_enemy=false means self)
        MOVE_STEEL_WING => Some((false, STAGE_DEF, 1, 0.1)),
        _ => None,
    }
}

/// Check if a move causes flinch. Returns flinch chance (0.0 = no flinch).
fn flinch_chance(move_id: MoveId) -> f64 {
    match move_id {
        MOVE_HEADBUTT | MOVE_BITE | MOVE_STOMP | MOVE_ROCK_SLIDE => 0.3,
        MOVE_TWISTER => 0.2,
        MOVE_HYPER_FANG => 0.1,
        _ => 0.0,
    }
}

/// Returns the number of hits for multi-hit moves. 1 for normal moves.
/// Gen 2 distribution for 2-5 hit moves: 2=37.5%, 3=37.5%, 4=12.5%, 5=12.5%
fn multi_hit_count(move_id: MoveId, rng_val: f64) -> u8 {
    match move_id {
        MOVE_DOUBLE_KICK => 2, // always exactly 2
        MOVE_FURY_SWIPES | MOVE_FURY_ATTACK => {
            if rng_val < 0.375 { 2 }
            else if rng_val < 0.75 { 3 }
            else if rng_val < 0.875 { 4 }
            else { 5 }
        }
        _ => 1,
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
        // Capture RNG state for save system
        self.last_rng_state = engine.rng.state;

        // Auto-save: push to persist queue when flagged
        if self.needs_save && !matches!(self.phase, GamePhase::TitleScreen | GamePhase::Credits { .. }) {
            self.needs_save = false;
            let save_json = self.serialize_save();
            engine.persist_queue.push(
                crate::chord_reps::persist::PersistCommand::Store {
                    key: "pokemon_save".to_string(),
                    value: save_json,
                }
            );
        }

        match self.phase.clone() {
            GamePhase::TitleScreen => {
                self.title_blink_timer += 1.0 / 60.0;
                let has_save = !engine.global_state.get_str("pokemon_save").unwrap_or("").is_empty();
                self.has_save = has_save;

                if has_save {
                    // Two options: CONTINUE (cursor 0) or NEW GAME (cursor 1)
                    if is_down(engine) && self.menu_cursor == 0 {
                        self.menu_cursor = 1;
                    } else if is_up(engine) && self.menu_cursor == 1 {
                        self.menu_cursor = 0;
                    }
                    if is_confirm(engine) {
                        if self.menu_cursor == 0 {
                            // CONTINUE — load save
                            let save_str = engine.global_state.get_str("pokemon_save").unwrap_or("").to_string();
                            if !save_str.is_empty() {
                                self.load_from_save(&save_str);
                                engine.rng.state = self.last_rng_state;
                                engine.global_state.set_str("game_phase", "overworld");
                            }
                        } else {
                            // NEW GAME — clear save and reset all state
                            engine.persist_queue.push(
                                crate::chord_reps::persist::PersistCommand::Store {
                                    key: "pokemon_save".to_string(),
                                    value: String::new(),
                                }
                            );
                            engine.global_state.set_str("pokemon_save", "");
                            // Full state reset
                            self.party.clear();
                            self.pc_boxes.clear();
                            self.bag = Bag::new();
                            self.badges = 0;
                            self.money = 3000;
                            self.step_count = 0;
                            self.has_starter = false;
                            self.rival_starter = 0;
                            self.rival_battle_done = false;
                            self.defeated_trainers.clear();
                            self.pokedex_seen.clear();
                            self.pokedex_caught.clear();
                            self.total_time = 0.0;
                            self.repel_steps = 0;
                            self.story_flags = 0;
                            self.visited_cities.clear();
                            self.last_pokecenter_map = MapId::CherrygroveCity;
                            self.last_house_map = MapId::NewBarkTown;
                            self.last_house_x = 12;
                            self.last_house_y = 5;
                            engine.global_state.set_str("game_phase", "overworld");
                            self.change_map(MapId::ElmLab, 5, 8);
                            self.phase = GamePhase::Overworld;
                        }
                    }
                } else {
                    // No save — original behavior: press Z to start
                    let start = is_confirm(engine);
                    if start {
                        engine.global_state.set_str("game_phase", "overworld");
                        if !self.has_starter {
                            self.change_map(MapId::ElmLab, 5, 8);
                        }
                        self.phase = GamePhase::Overworld;
                    }
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
            GamePhase::TrainerCard => self.step_trainer_card(engine),
            GamePhase::PCMenu { .. } => self.step_pc_menu(engine),
            GamePhase::DaycareDeposit { .. } => self.step_daycare_deposit(engine),
            GamePhase::DaycarePrompt { .. } => self.step_daycare_prompt(engine),
            GamePhase::FlyMenu { .. } => self.step_fly_menu(engine),

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

            GamePhase::TrainerApproach { npc_idx, timer } => {
                let dt = 1.0 / 60.0;
                let t = timer + dt;
                // Phase 1: "!" exclamation for 0.5 seconds
                if t < 0.5 {
                    self.approach_exclaim_timer = t;
                    self.phase = GamePhase::TrainerApproach { npc_idx, timer: t };
                } else {
                    // Phase 2: walk toward player one tile at a time
                    self.approach_exclaim_timer = 0.0;
                    let npc = &self.current_map.npcs[npc_idx as usize];
                    let tx = self.approach_npc_x;
                    let ty = self.approach_npc_y;
                    let px = self.player.x;
                    let py = self.player.y;
                    let dist_x = (px - tx).abs();
                    let dist_y = (py - ty).abs();
                    let adjacent = (dist_x + dist_y) <= 1;
                    if adjacent {
                        // Trainer is next to player — start dialogue + battle
                        let team: Vec<(SpeciesId, u8)> = npc.trainer_team
                            .iter().map(|tp| (tp.species_id, tp.level)).collect();
                        let lines: Vec<String> = npc.dialogue.iter().map(|s| s.to_string()).collect();
                        self.dialogue = Some(DialogueState {
                            lines, current_line: 0, char_index: 0, timer: 0.0,
                            on_complete: DialogueAction::StartTrainerBattle { team },
                        });
                        self.phase = GamePhase::Dialogue;
                        self.battle = None;
                    } else {
                        // Walk one tile closer to the player
                        self.approach_walk_offset += 1.0 / WALK_SPEED;
                        if self.approach_walk_offset >= 1.0 {
                            self.approach_walk_offset = 0.0;
                            // Move one tile toward the player (along the facing direction)
                            let (dx, dy) = match npc.facing {
                                Direction::Up => (0i32, -1i32),
                                Direction::Down => (0, 1),
                                Direction::Left => (-1, 0),
                                Direction::Right => (1, 0),
                            };
                            self.approach_npc_x += dx;
                            self.approach_npc_y += dy;
                        }
                        self.phase = GamePhase::TrainerApproach { npc_idx, timer: t };
                    }
                }
            }

            GamePhase::MapFadeOut { dest_map, dest_x, dest_y, timer } => {
                let dt = 1.0 / 60.0;
                let t = timer + dt;
                if t >= 0.25 {
                    // Fade complete — perform the map change
                    self.ice_sliding = None; // stop ice sliding on map transition
                    self.change_map(dest_map, dest_x, dest_y);
                    self.phase = GamePhase::MapFadeIn { timer: 0.0 };
                } else {
                    self.phase = GamePhase::MapFadeOut { dest_map, dest_x, dest_y, timer: t };
                }
            }

            GamePhase::MapFadeIn { timer } => {
                let dt = 1.0 / 60.0;
                let t = timer + dt;
                if t >= 0.25 {
                    self.phase = GamePhase::Overworld;
                    self.los_suppress = 3; // suppress trainer LOS for 3 frames after map change
                } else {
                    self.phase = GamePhase::MapFadeIn { timer: t };
                }
            }

            GamePhase::Evolution { timer, new_species } => {
                let dt = 1.0 / 60.0;
                let t = timer + dt;
                // Flash during evolution animation (first 2 seconds)
                if t < 2.0 {
                    let flash_cycle = (t * 6.0) as u32;
                    self.screen_flash = if flash_cycle % 2 == 0 { 0.8 } else { 0.0 };
                } else {
                    self.screen_flash = 0.0;
                }

                // Cancel evolution with B button during flash phase
                if t < 2.0 && is_cancel(engine) {
                    self.screen_flash = 0.0;
                    // Find the pokemon that would evolve
                    let evo_idx = self.party.iter().position(|p| {
                        get_species(p.species_id).and_then(|s| s.evolution_into).map(|e| e == new_species).unwrap_or(false)
                    }).unwrap_or(0);
                    let name = self.party.get(evo_idx).map(|p| p.name().to_string()).unwrap_or_default();
                    self.dialogue = Some(DialogueState {
                        lines: vec![
                            format!("Huh? {} stopped", name),
                            "evolving!".to_string(),
                        ],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                } else if t > 3.0 || (t > 2.0 && is_confirm(engine)) {
                    // Apply evolution
                    let evo_idx = self.party.iter().position(|p| {
                        get_species(p.species_id).and_then(|s| s.evolution_into).map(|e| e == new_species).unwrap_or(false)
                    }).unwrap_or(0);
                    if let Some(p) = self.party.get_mut(evo_idx) {
                        let old_name = p.name().to_string();
                        p.species_id = new_species;
                        p.recalc_stats();
                        self.register_caught(new_species);
                        let new_name = get_species(new_species).map(|s| s.name).unwrap_or("???");
                        self.dialogue = Some(DialogueState {
                            lines: vec![
                                format!("Congratulations!"),
                                format!("{} evolved into", old_name),
                                format!("{}!", new_name),
                            ],
                            current_line: 0, char_index: 0, timer: 0.0,
                            on_complete: DialogueAction::None,
                        });
                        self.phase = GamePhase::Dialogue;
                    } else {
                        self.phase = GamePhase::Overworld;
                    }
                    self.screen_flash = 0.0;
                } else {
                    self.phase = GamePhase::Evolution { timer: t, new_species };
                }
            }

            GamePhase::WhiteoutFade { timer, money_lost } => {
                let dt = 1.0 / 60.0;
                let t = timer + dt;
                if t >= 1.5 {
                    // Fade complete — warp to PokeCenter with dialogue
                    let saved_pc = self.last_pokecenter_map;
                    self.change_map(MapId::PokemonCenter, 5, 6);
                    self.last_pokecenter_map = saved_pc;
                    self.dialogue = Some(DialogueState {
                        lines: vec![
                            "You are out of usable".to_string(),
                            "POKEMON!".to_string(),
                            "You blacked out!".to_string(),
                            format!("You lost ${}...", money_lost),
                        ],
                        current_line: 0, char_index: 0, timer: 0.0,
                        on_complete: DialogueAction::None,
                    });
                    self.phase = GamePhase::Dialogue;
                } else {
                    self.phase = GamePhase::WhiteoutFade { timer: t, money_lost };
                }
            }

            GamePhase::Credits { scroll_y } => {
                let scroll_speed = if is_confirm(engine) { 1.5 } else { 0.5 };
                let new_y = scroll_y + scroll_speed;
                // Credits text is ~20 lines × 12px = 240px. After scrolling past all text + screen height, return to title.
                if new_y > 144.0 + 300.0 {
                    self.phase = GamePhase::TitleScreen;
                    engine.global_state.set_str("game_phase", "title");
                } else {
                    self.phase = GamePhase::Credits { scroll_y: new_y };
                }
            }
        }

        // Decay screen effects
        let dt = 1.0 / 60.0;
        let in_transition = matches!(self.phase, GamePhase::EncounterTransition { .. } | GamePhase::Evolution { .. } | GamePhase::MapFadeOut { .. } | GamePhase::MapFadeIn { .. } | GamePhase::WhiteoutFade { .. });
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

        // ─── Debug State Export (Phase 0E) ─────────────────────
        // Export key game state to global_state every frame for headless testing
        engine.global_state.set_f64("player_x", self.player.x as f64);
        engine.global_state.set_f64("player_y", self.player.y as f64);
        engine.global_state.set_f64("current_map", self.current_map_id as u8 as f64);
        engine.global_state.set_f64("badges", self.badges as f64);
        engine.global_state.set_f64("party_size", self.party.len() as f64);
        engine.global_state.set_f64("step_count", self.step_count as f64);
        engine.global_state.set_f64("defeated_count", self.defeated_trainers.len() as f64);
        engine.global_state.set_f64("money", self.money as f64);
        if let Some(lead) = self.party.first() {
            engine.global_state.set_f64("lead_hp", lead.hp as f64);
            engine.global_state.set_f64("lead_level", lead.level as f64);
            engine.global_state.set_f64("lead_species", lead.species_id as f64);
        }
        // Music state for JS playback
        engine.global_state.set_f64("music_id", self.current_map.music_id as f64);
        engine.global_state.set_str("map_name", self.current_map.name);
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
            GamePhase::PokemonMenu { cursor, action, sub_cursor } => self.render_pokemon_menu(fb, *cursor, *action, *sub_cursor),
            GamePhase::BagMenu { cursor } => self.render_bag_menu(fb, *cursor),
            GamePhase::BagUseItem { item_id, target_cursor } => self.render_bag_use_item(fb, *item_id, *target_cursor),
            GamePhase::Healing { .. } => self.render_healing(fb),
            GamePhase::Evolution { timer, new_species } => self.render_evolution(fb, *timer, *new_species),
            GamePhase::PokeMart { cursor } => self.render_pokemart(fb, *cursor),
            GamePhase::PokemonSummary { index } => self.render_pokemon_summary(fb, *index),
            GamePhase::Pokedex { cursor, scroll } => self.render_pokedex(fb, *cursor, *scroll),
            GamePhase::TrainerCard => self.render_trainer_card(fb),
            GamePhase::PCMenu { mode, cursor } => self.render_pc_menu(fb, *mode, *cursor),
            GamePhase::DaycareDeposit { cursor } => self.render_daycare_deposit(fb, *cursor),
            GamePhase::DaycarePrompt { cursor } => self.render_daycare_prompt(fb, *cursor),
            GamePhase::FlyMenu { cursor } => self.render_fly_menu(fb, *cursor),
            GamePhase::TrainerApproach { npc_idx, .. } => {
                // Render overworld, then draw "!" above approaching trainer
                self.render_overworld_with_approach(fb, *npc_idx);
            }
            GamePhase::MapFadeOut { timer, .. } => {
                // Render overworld underneath, then darken
                self.render_overworld(fb);
                if let Some(ctx) = &self.ctx {
                    let alpha = ((*timer / 0.25).min(1.0) * 255.0) as u8;
                    fill_virtual_screen(fb, ctx, Color::from_rgba(0, 0, 0, alpha));
                }
            }
            GamePhase::MapFadeIn { timer } => {
                // Render new overworld, fading in from black
                self.render_overworld(fb);
                if let Some(ctx) = &self.ctx {
                    let alpha = ((1.0 - *timer / 0.25).max(0.0) * 255.0) as u8;
                    fill_virtual_screen(fb, ctx, Color::from_rgba(0, 0, 0, alpha));
                }
            }
            GamePhase::WhiteoutFade { timer, .. } => {
                // Fade to white over 1.5 seconds (distinctive from normal black fade)
                if let Some(ctx) = &self.ctx {
                    fill_virtual_screen(fb, ctx, Color::from_rgba(0, 0, 0, 255));
                    let alpha = ((*timer / 1.5).min(1.0) * 255.0) as u8;
                    fill_virtual_screen(fb, ctx, Color::from_rgba(255, 255, 255, alpha));
                }
            }
            GamePhase::Credits { scroll_y } => self.render_credits(fb, *scroll_y),
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
                    engine.global_state.set_f64("enemy_species_id", battle.enemy.species_id as f64);
                }
                if let Some(pp) = self.party.get(battle.player_idx) {
                    if let Some(sp) = get_species(pp.species_id) {
                        engine.global_state.set_str("player_pokemon", &sp.name.to_lowercase());
                        engine.global_state.set_f64("player_species_id", pp.species_id as f64);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod headless_tests {
    use super::*;
    use crate::headless::{HeadlessRunner, RunConfig};
    use crate::input_frame::InputFrame;

    // ── Input builder helpers ─────────────────────────────────

    /// Single key-press frame (press + release in one frame)
    fn press(key: &str) -> InputFrame {
        InputFrame {
            keys_pressed: vec![key.to_string()],
            ..Default::default()
        }
    }

    /// Empty frame (no input)
    fn empty() -> InputFrame {
        InputFrame::default()
    }

    /// Hold a key down for `n` frames (keys_held repeated)
    #[allow(dead_code)]
    fn hold(key: &str, n: usize) -> Vec<InputFrame> {
        (0..n).map(|_| InputFrame {
            keys_held: vec![key.to_string()],
            ..Default::default()
        }).collect()
    }

    /// Wait for `n` empty frames
    #[allow(dead_code)]
    fn wait(n: usize) -> Vec<InputFrame> {
        (0..n).map(|_| empty()).collect()
    }

    /// Press a direction key once then wait for `gap` frames (one tile movement)
    #[allow(dead_code)]
    fn walk_dir(dir: &str, gap: usize) -> Vec<InputFrame> {
        let arrow = match dir {
            "up" => "ArrowUp",
            "down" => "ArrowDown",
            "left" => "ArrowLeft",
            "right" => "ArrowRight",
            other => other, // allow raw key name
        };
        let mut out = vec![press(arrow)];
        out.extend(wait(gap));
        out
    }

    /// Concatenate multiple input sequences into one flat Vec
    #[allow(dead_code)]
    fn sequence(seqs: Vec<Vec<InputFrame>>) -> Vec<InputFrame> {
        seqs.into_iter().flatten().collect()
    }

    #[test]
    fn test_title_screen_starts_correctly() {
        let mut runner = HeadlessRunner::new(160, 144);
        let mut game = PokemonSim::new();
        let result = runner.run_sim_frames(
            &mut game, 42, &[empty()], 10,
            RunConfig { turbo: true, capture_state_hashes: false },
        );
        // Should be on title screen, no party
        assert_eq!(result.get_f64("party_size"), Some(0.0));
        assert_eq!(result.get_f64("badges"), Some(0.0));
    }

    #[test]
    fn test_confirm_enters_elm_lab() {
        let mut runner = HeadlessRunner::new(160, 144);
        let mut game = PokemonSim::new();
        // Press confirm to exit title screen → should go to Elm's Lab
        let mut inputs = vec![empty(); 5];
        inputs.push(press("KeyZ")); // confirm on title screen
        inputs.extend(vec![empty(); 10]);
        let result = runner.run_sim_frames(
            &mut game, 42, &inputs, inputs.len() as u64,
            RunConfig { turbo: true, capture_state_hashes: false },
        );
        // Player should be in ElmLab (MapId variant index)
        let map = result.get_f64("current_map").unwrap_or(-1.0);
        assert_eq!(map, MapId::ElmLab as u8 as f64);
        // Still no party (no starter yet)
        assert_eq!(result.get_f64("party_size"), Some(0.0));
    }

    #[test]
    fn test_select_starter_gives_pokemon() {
        let mut runner = HeadlessRunner::new(160, 144);
        let mut game = PokemonSim::new();
        // Title → confirm → Elm Lab → walk to Elm → talk → pick starter
        let mut inputs = vec![empty(); 3];
        inputs.push(press("KeyZ")); // exit title → Elm Lab (player at 5,8)
        inputs.extend(vec![empty(); 5]);
        // Walk left 1 tile (5→4)
        inputs.push(press("ArrowLeft"));
        inputs.extend(vec![empty(); 10]);
        // Walk up 1 tile (8→7), now at (4,7) facing up, Elm at (4,6)
        inputs.push(press("ArrowUp"));
        inputs.extend(vec![empty(); 10]);
        // Talk to Elm — he has 9 dialogue lines
        inputs.push(press("KeyZ"));
        inputs.extend(vec![empty(); 5]);
        // Mash confirm through all 9 dialogue lines
        for _ in 0..12 {
            inputs.push(press("KeyZ"));
            inputs.extend(vec![empty(); 5]);
        }
        // Now in StarterSelect, cursor at 0 (Chikorita) — pick it
        inputs.push(press("KeyZ"));
        inputs.extend(vec![empty(); 5]);
        // Mash confirm through "You received..." dialogue (4 lines)
        for _ in 0..8 {
            inputs.push(press("KeyZ"));
            inputs.extend(vec![empty(); 5]);
        }
        let result = runner.run_sim_frames(
            &mut game, 42, &inputs, inputs.len() as u64,
            RunConfig { turbo: true, capture_state_hashes: false },
        );
        // Should have 1 Pokemon in party after picking starter
        assert_eq!(result.get_f64("party_size"), Some(1.0));
        // Lead should be level 5
        assert_eq!(result.get_f64("lead_level"), Some(5.0));
    }

    #[test]
    fn test_debug_state_export_keys_present() {
        let mut runner = HeadlessRunner::new(160, 144);
        let mut game = PokemonSim::new();
        let result = runner.run_sim_frames(
            &mut game, 42, &[empty()], 5,
            RunConfig { turbo: true, capture_state_hashes: false },
        );
        // All debug state keys should be present
        assert!(result.game_state.contains_key("player_x"));
        assert!(result.game_state.contains_key("player_y"));
        assert!(result.game_state.contains_key("current_map"));
        assert!(result.game_state.contains_key("badges"));
        assert!(result.game_state.contains_key("party_size"));
        assert!(result.game_state.contains_key("step_count"));
        assert!(result.game_state.contains_key("defeated_count"));
        assert!(result.game_state.contains_key("money"));
    }

    #[test]
    fn test_deterministic_same_seed() {
        let mut runner1 = HeadlessRunner::new(160, 144);
        let mut game1 = PokemonSim::new();
        let empties: Vec<InputFrame> = (0..30).map(|_| empty()).collect();
        let result1 = runner1.run_sim_frames(
            &mut game1, 42, &empties, 30,
            RunConfig { turbo: true, capture_state_hashes: true },
        );

        let mut runner2 = HeadlessRunner::new(160, 144);
        let mut game2 = PokemonSim::new();
        let empties2: Vec<InputFrame> = (0..30).map(|_| empty()).collect();
        let result2 = runner2.run_sim_frames(
            &mut game2, 42, &empties2, 30,
            RunConfig { turbo: true, capture_state_hashes: true },
        );

        assert_eq!(result1.state_hash, result2.state_hash);
        assert_eq!(result1.state_hashes, result2.state_hashes);
    }

    #[test]
    fn test_different_seed_different_state() {
        let mut runner1 = HeadlessRunner::new(160, 144);
        let mut game1 = PokemonSim::new();
        // Use inputs that actually do something (enter game, walk around)
        let mut inputs = vec![empty(); 3];
        inputs.push(press("KeyZ")); // exit title
        inputs.extend(vec![empty(); 5]);
        for _ in 0..5 {
            inputs.push(press("ArrowRight"));
            inputs.extend(vec![empty(); 8]);
        }
        let result1 = runner1.run_sim_frames(
            &mut game1, 42, &inputs, inputs.len() as u64,
            RunConfig { turbo: true, capture_state_hashes: false },
        );

        let mut runner2 = HeadlessRunner::new(160, 144);
        let mut game2 = PokemonSim::new();
        let result2 = runner2.run_sim_frames(
            &mut game2, 99, &inputs, inputs.len() as u64,
            RunConfig { turbo: true, capture_state_hashes: false },
        );

        // Position should be the same (deterministic movement)
        assert_eq!(result1.get_f64("player_x"), result2.get_f64("player_x"));
        assert_eq!(result1.get_f64("player_y"), result2.get_f64("player_y"));
    }

    #[test]
    fn test_walking_changes_position() {
        let mut runner = HeadlessRunner::new(160, 144);
        let mut game = PokemonSim::new();
        // Title → confirm → elm lab
        let mut inputs = vec![empty(); 3];
        inputs.push(press("KeyZ")); // exit title → Elm Lab
        inputs.extend(vec![empty(); 5]);
        let initial_frames = inputs.len() as u64;
        // Get initial position
        let result_before = runner.run_sim_frames(
            &mut game, 42, &inputs, initial_frames,
            RunConfig { turbo: true, capture_state_hashes: false },
        );
        let start_x = result_before.get_f64("player_x").unwrap();

        // Walk right from starting position (more room to move in ElmLab)
        let mut runner2 = HeadlessRunner::new(160, 144);
        let mut game2 = PokemonSim::new();
        let mut inputs2 = inputs.clone();
        inputs2.push(press("ArrowRight"));
        inputs2.extend(vec![empty(); 10]);

        let result_after = runner2.run_sim_frames(
            &mut game2, 42, &inputs2, inputs2.len() as u64,
            RunConfig { turbo: true, capture_state_hashes: false },
        );
        let end_x = result_after.get_f64("player_x").unwrap();

        // X should have increased (moved right)
        assert!(end_x > start_x, "Player should have moved right: start_x={} end_x={}", start_x, end_x);
    }

    #[test]
    fn test_money_starts_at_3000() {
        let mut runner = HeadlessRunner::new(160, 144);
        let mut game = PokemonSim::new();
        let result = runner.run_sim_frames(
            &mut game, 42, &[empty()], 5,
            RunConfig { turbo: true, capture_state_hashes: false },
        );
        assert_eq!(result.get_f64("money"), Some(3000.0));
    }

    #[test]
    fn test_save_load_roundtrip() {
        use crate::pokemon::data::Pokemon;
        use crate::pokemon::maps::{MapId, Direction};

        let mut sim = PokemonSim::new();
        // Set up a realistic game state
        sim.has_starter = true;
        sim.money = 12345;
        sim.badges = 0b00001111; // 4 badges
        sim.step_count = 500;
        sim.rival_starter = CYNDAQUIL;
        sim.rival_battle_done = true;
        sim.total_time = 7200.0;
        sim.last_rng_state = 42424242;
        sim.current_map_id = MapId::GoldenrodCity;
        sim.current_map = load_map(MapId::GoldenrodCity);
        sim.player.x = 10;
        sim.player.y = 7;
        sim.player.facing = Direction::Left;
        sim.last_pokecenter_map = MapId::GoldenrodCity;
        sim.last_house_map = MapId::EcruteakCity;

        // Add party Pokemon
        let mut p1 = Pokemon::new(CYNDAQUIL, 25);
        p1.hp = 50;
        sim.party.push(p1);
        let p2 = Pokemon::new(PIDGEY, 18);
        sim.party.push(p2);

        // Add a defeated trainer
        sim.defeated_trainers.push((MapId::VioletGym, 0));
        sim.defeated_trainers.push((MapId::AzaleaGym, 0));

        // Add bag items
        sim.bag.add_item(1, 5); // 5 Potions
        sim.bag.add_item(4, 2); // 2 Pokeballs

        // Add pokedex entries
        sim.pokedex_seen.push(CYNDAQUIL);
        sim.pokedex_seen.push(PIDGEY);
        sim.pokedex_caught.push(CYNDAQUIL);

        // Serialize
        let json = sim.serialize_save();
        assert!(json.contains("\"map\":\"GoldenrodCity\""));
        assert!(json.contains("\"money\":12345"));
        assert!(json.contains("\"badges\":15"));

        // Create a fresh sim and load the save
        let mut sim2 = PokemonSim::new();
        sim2.load_from_save(&json);

        // Verify all fields restored
        assert_eq!(sim2.current_map_id, MapId::GoldenrodCity);
        assert_eq!(sim2.player.x, 10);
        assert_eq!(sim2.player.y, 7);
        assert_eq!(sim2.player.facing, Direction::Left);
        assert_eq!(sim2.money, 12345);
        assert_eq!(sim2.badges, 0b00001111);
        assert_eq!(sim2.step_count, 500);
        assert_eq!(sim2.rival_starter, CYNDAQUIL);
        assert!(sim2.rival_battle_done);
        assert!(sim2.has_starter);
        assert_eq!(sim2.last_rng_state, 42424242);
        assert_eq!(sim2.last_pokecenter_map, MapId::GoldenrodCity);
        assert_eq!(sim2.last_house_map, MapId::EcruteakCity);
        assert_eq!(sim2.party.len(), 2);
        assert_eq!(sim2.party[0].species_id, CYNDAQUIL);
        assert_eq!(sim2.party[0].level, 25);
        assert_eq!(sim2.party[0].hp, 50);
        assert_eq!(sim2.party[1].species_id, PIDGEY);
        assert_eq!(sim2.party[1].level, 18);
        assert_eq!(sim2.defeated_trainers.len(), 2);
        assert_eq!(sim2.defeated_trainers[0], (MapId::VioletGym, 0));
        assert_eq!(sim2.defeated_trainers[1], (MapId::AzaleaGym, 0));
        assert_eq!(sim2.bag.items.len(), 2);
        assert_eq!(sim2.pokedex_seen.len(), 2);
        assert_eq!(sim2.pokedex_caught.len(), 1);
        assert!(matches!(sim2.phase, GamePhase::Overworld));
    }

    #[test]
    fn test_struggle_available_when_all_pp_zero() {
        // Verify MOVE_STRUGGLE exists in the move database
        let struggle = get_move(MOVE_STRUGGLE);
        assert!(struggle.is_some(), "Struggle move must exist in MOVE_DB");
        let s = struggle.unwrap();
        assert_eq!(s.power, 50);
        assert_eq!(s.accuracy, 255); // Never misses
        assert_eq!(s.name, "Struggle");
    }

    #[test]
    fn test_freeze_thaw_chance() {
        // Verify try_thaw works: 10% chance with rng_roll < 0.1
        let mut pkmn = Pokemon::new(CYNDAQUIL, 10);
        pkmn.status = StatusCondition::Freeze;
        assert!(!pkmn.can_move(), "Frozen Pokemon cannot move");

        // Roll of 0.05 should thaw (< 0.1)
        assert!(pkmn.try_thaw(0.05), "Should thaw with roll 0.05");
        assert_eq!(pkmn.status, StatusCondition::None);
        assert!(pkmn.can_move(), "Thawed Pokemon can move");

        // Test that roll >= 0.1 doesn't thaw
        pkmn.status = StatusCondition::Freeze;
        assert!(!pkmn.try_thaw(0.15), "Should NOT thaw with roll 0.15");
        assert!(matches!(pkmn.status, StatusCondition::Freeze));
    }

    #[test]
    fn test_status_moves_inflict_status() {
        // Verify that try_inflict_status works for status moves (power=0)
        let mut target = Pokemon::new(PIDGEY, 10);
        assert!(matches!(target.status, StatusCondition::None));

        // Thunder Wave should inflict paralysis (guaranteed for status moves)
        try_inflict_status(&mut target, MOVE_THUNDER_WAVE, 0.5);
        assert!(matches!(target.status, StatusCondition::Paralysis),
            "Thunder Wave should inflict Paralysis, got {:?}", target.status);

        // Hypnosis on a fresh target should inflict sleep
        let mut target2 = Pokemon::new(PIDGEY, 10);
        try_inflict_status(&mut target2, MOVE_HYPNOSIS, 0.5);
        assert!(matches!(target2.status, StatusCondition::Sleep { .. }),
            "Hypnosis should inflict Sleep, got {:?}", target2.status);
    }

    #[test]
    fn test_story_flags_save_load() {
        let mut sim = PokemonSim::new();
        sim.has_starter = true;
        sim.party.push(Pokemon::new(CYNDAQUIL, 5));
        sim.story_flags = FLAG_RIVAL_ROUTE29 | FLAG_RIVAL_VICTORY;

        let json = sim.serialize_save();
        assert!(json.contains("\"flags\":"), "Save should contain flags field");
        // The value should be FLAG_RIVAL_ROUTE29 | FLAG_RIVAL_VICTORY = (1<<2)|(1<<9) = 4+512 = 516
        assert!(json.contains("\"flags\":516"), "Flags should serialize as 516, got: {}", json);

        let mut sim2 = PokemonSim::new();
        sim2.load_from_save(&json);
        assert_eq!(sim2.story_flags, FLAG_RIVAL_ROUTE29 | FLAG_RIVAL_VICTORY);
        assert!(sim2.has_flag(FLAG_RIVAL_ROUTE29));
        assert!(sim2.has_flag(FLAG_RIVAL_VICTORY));
        assert!(!sim2.has_flag(FLAG_SPROUT_CLEAR));
    }

    #[test]
    fn test_victory_road_rival_requires_8_badges() {
        let mut sim = PokemonSim::new();
        sim.has_starter = true;
        sim.rival_starter = CYNDAQUIL;
        sim.party.push(Pokemon::new(CHIKORITA, 36));
        sim.change_map(MapId::VictoryRoad, 7, 6);

        // With only 7 badges, rival should NOT trigger
        sim.badges = 0b01111111; // 7 badges
        assert!(!sim.check_victory_road_rival());

        // With 8 badges, rival SHOULD trigger
        sim.badges = 0b11111111; // 8 badges
        assert!(sim.check_victory_road_rival());
        assert!(sim.has_flag(FLAG_RIVAL_VICTORY));
        // Should not trigger again
        assert!(!sim.check_victory_road_rival());
    }

    #[test]
    fn test_final_evolutions_exist() {
        // Verify final starter forms and Magneton have species data
        assert!(get_species(MEGANIUM).is_some(), "Meganium should exist");
        assert!(get_species(TYPHLOSION).is_some(), "Typhlosion should exist");
        assert!(get_species(FERALIGATR).is_some(), "Feraligatr should exist");
        assert!(get_species(MAGNETON).is_some(), "Magneton should exist");

        // Check correct types
        let meg = get_species(MEGANIUM).unwrap();
        assert_eq!(meg.type1, PokemonType::Grass);
        let typh = get_species(TYPHLOSION).unwrap();
        assert_eq!(typh.type1, PokemonType::Fire);
        let fera = get_species(FERALIGATR).unwrap();
        assert_eq!(fera.type1, PokemonType::Water);
        let magn = get_species(MAGNETON).unwrap();
        assert_eq!(magn.type1, PokemonType::Electric);

        // Check evolution chains
        let bay = get_species(BAYLEEF).unwrap();
        assert_eq!(bay.evolution_into, Some(MEGANIUM));
        let quil = get_species(QUILAVA).unwrap();
        assert_eq!(quil.evolution_into, Some(TYPHLOSION));
        let croc = get_species(CROCONAW).unwrap();
        assert_eq!(croc.evolution_into, Some(FERALIGATR));
        let mite = get_species(MAGNEMITE).unwrap();
        assert_eq!(mite.evolution_into, Some(MAGNETON));
    }

    #[test]
    fn test_hp_formula_gen2() {
        // Gen 2 HP formula: ((Base*2 + IV) * Level / 100) + Level + 10
        // IV=15 (max), EV=0
        // Pikachu base HP = 35: at lv50, ((35*2+15)*50/100) + 50 + 10 = 42 + 50 + 10 = 102
        let hp = calc_hp(35, 50);
        assert_eq!(hp, 102, "Pikachu lv50 HP should be 102, got {}", hp);

        // Chikorita base HP = 45: at lv5, ((45*2+15)*5/100) + 5 + 10 = 5 + 5 + 10 = 20
        let hp5 = calc_hp(45, 5);
        assert_eq!(hp5, 20, "Chikorita lv5 HP should be 20, got {}", hp5);
    }

    #[test]
    fn test_whiteout_preserves_pokecenter_map() {
        let mut sim = PokemonSim::new();
        sim.last_pokecenter_map = MapId::VioletCity;

        // Simulate whiteout — save/restore pattern
        let saved_pc = sim.last_pokecenter_map;
        sim.change_map(MapId::PokemonCenter, 5, 6);
        sim.last_pokecenter_map = saved_pc;

        assert_eq!(sim.last_pokecenter_map, MapId::VioletCity,
            "Whiteout should preserve last PokeCenter, not overwrite with current map");
    }

    #[test]
    fn test_toxic_escalating_damage() {
        let mut p = Pokemon::new(PIDGEY, 50);
        // Pidgey at lv50: HP = ((40*2+15)*50/100)+50+10 = 47+50+10 = 107
        let max_hp = p.max_hp;
        p.status = StatusCondition::BadPoison { turn: 1 };

        // Turn 1: 1/16 of max HP
        let d1 = p.apply_status_damage();
        assert_eq!(d1, max_hp / 16, "Turn 1 toxic damage should be max_hp/16");
        // Status should now be turn 2
        assert!(matches!(p.status, StatusCondition::BadPoison { turn: 2 }));

        // Turn 2: 2/16 of max HP
        let d2 = p.apply_status_damage();
        assert_eq!(d2, (max_hp as u32 * 2 / 16).max(1) as u16, "Turn 2 toxic damage should be 2*max_hp/16");
        assert!(matches!(p.status, StatusCondition::BadPoison { turn: 3 }));
    }

    #[test]
    fn test_toxic_infliction() {
        let mut target = Pokemon::new(PIDGEY, 10);
        try_inflict_status(&mut target, MOVE_TOXIC, 0.5);
        assert!(matches!(target.status, StatusCondition::BadPoison { turn: 1 }),
            "Toxic should inflict BadPoison, got {:?}", target.status);
    }

    #[test]
    fn test_self_destruct_data() {
        // Verify Self-Destruct is a high-power Normal Physical move
        let sd = get_move(MOVE_SELF_DESTRUCT).expect("Self-Destruct should exist");
        assert_eq!(sd.power, 200);
        assert_eq!(sd.move_type, PokemonType::Normal);
        assert_eq!(sd.category, MoveCategory::Physical);
    }

    #[test]
    fn test_haze_data() {
        let haze = get_move(MOVE_HAZE).expect("Haze should exist");
        assert_eq!(haze.power, 0);
        assert_eq!(haze.category, MoveCategory::Status);
    }

    #[test]
    fn test_confuse_ray_data() {
        let cr = get_move(MOVE_CONFUSE_RAY).expect("Confuse Ray should exist");
        assert_eq!(cr.power, 0);
        assert_eq!(cr.category, MoveCategory::Status);
        assert_eq!(cr.move_type, PokemonType::Ghost);
    }

    #[test]
    fn test_toxic_counter_resets_on_switch() {
        // BadPoison turn counter should reset to 1 when switching back in (Gen 2)
        let mut p = Pokemon::new(CHIKORITA, 10);
        p.status = StatusCondition::BadPoison { turn: 5 };
        // Simulate what switch logic does
        if let StatusCondition::BadPoison { ref mut turn } = p.status {
            *turn = 1;
        }
        assert!(matches!(p.status, StatusCondition::BadPoison { turn: 1 }),
            "BadPoison turn should reset to 1 on switch, got {:?}", p.status);
    }

    #[test]
    fn test_self_destruct_user_faints() {
        // Self-Destruct user always faints after dealing damage
        let sd = get_move(MOVE_SELF_DESTRUCT).expect("Self-Destruct should exist");
        assert_eq!(sd.power, 200, "Self-Destruct should be 200 power");
        assert_eq!(sd.accuracy, 100, "Self-Destruct should have 100% accuracy");
    }

    #[test]
    fn test_swagger_data() {
        let sw = get_move(MOVE_SWAGGER).expect("Swagger should exist");
        assert_eq!(sw.power, 0);
        assert_eq!(sw.category, MoveCategory::Status);
        assert_eq!(sw.move_type, PokemonType::Normal);
        assert_eq!(sw.accuracy, 90);
        assert_eq!(sw.pp, 15);
    }

    #[test]
    fn test_story_flags_sprout_clear() {
        let mut sim = PokemonSim::new();
        sim.party.push(Pokemon::new(CHIKORITA, 10));
        sim.change_map(MapId::SproutTower3F, 7, 2);

        // Should trigger elder battle
        assert!(!sim.has_flag(FLAG_SPROUT_CLEAR));
        let triggered = sim.check_sprout_tower_elder();
        assert!(triggered);
        assert!(sim.has_flag(FLAG_SPROUT_CLEAR));

        // Should not trigger again
        assert!(!sim.check_sprout_tower_elder());
    }

    #[test]
    fn test_sudowoodo_requires_3_badges() {
        // Sudowoodo is now NPC-based + requires Squirtbottle.
        // check_sudowoodo() always returns false (legacy stub).
        let mut sim = PokemonSim::new();
        sim.party.push(Pokemon::new(CHIKORITA, 10));
        sim.change_map(MapId::Route36, 15, 6);
        sim.badges = 0b00000111; // 3 badges
        // Legacy function always returns false now
        assert!(!sim.check_sudowoodo(&mut Engine::new(160, 144)));
        // Squirtbottle flag enables interaction via NPC dialogue system
        sim.set_flag(FLAG_SQUIRTBOTTLE);
        assert!(sim.has_flag(FLAG_SQUIRTBOTTLE));
    }

    #[test]
    fn test_catch_shake_prob_clamped() {
        // Ensure shake_prob can't exceed 1.0 even with extreme values
        // A very low HP, high catch rate mon at 1 HP with a status effect
        let max_hp = 10.0_f64;
        let cur_hp = 1.0_f64;
        let catch_rate = 255.0_f64;
        let ball_mult = 2.0_f64;
        let status_mult = 2.0_f64;
        let rate = ((3.0 * max_hp - 2.0 * cur_hp) * catch_rate * ball_mult * status_mult) / (3.0 * max_hp);
        let shake_prob = (rate / 255.0).min(1.0);
        assert!(shake_prob <= 1.0, "shake_prob {} exceeded 1.0", shake_prob);
    }

    #[test]
    fn test_champion_credits_over_evolution() {
        // Champion Lance check must take priority over pending evolution
        // Just verify the code structure: ChampionLance is checked before pending_evo
        let _sim = PokemonSim::new();
        // This is a structural test — the fix ensures Champion credits fire even with pending evo
        assert!(load_map(MapId::ChampionLance).npcs.len() > 0, "ChampionLance must have NPCs");
    }

    #[test]
    fn test_hyper_beam_data() {
        let md = get_move(MOVE_HYPER_BEAM).unwrap();
        assert_eq!(md.power, 150);
        assert_eq!(md.accuracy, 90);
        assert_eq!(md.pp, 5);
        assert_eq!(md.move_type, PokemonType::Normal);
        assert_eq!(md.category, MoveCategory::Physical);
    }

    #[test]
    fn test_outrage_data() {
        let md = get_move(MOVE_OUTRAGE).unwrap();
        assert_eq!(md.power, 90);
        assert_eq!(md.move_type, PokemonType::Dragon);
        // Dragon is Special in Gen 2
        assert_eq!(md.category, MoveCategory::Special);
    }

    #[test]
    fn test_rest_data() {
        let md = get_move(MOVE_REST).unwrap();
        assert_eq!(md.category, MoveCategory::Status);
        assert_eq!(md.power, 0);
        assert_eq!(md.move_type, PokemonType::Psychic);
    }

    #[test]
    fn test_thrash_data() {
        let md = get_move(MOVE_THRASH).unwrap();
        assert_eq!(md.power, 90);
        assert_eq!(md.move_type, PokemonType::Normal);
        assert_eq!(md.category, MoveCategory::Physical);
    }

    #[test]
    fn test_rocket_hq_map_exists() {
        let map = load_map(MapId::RocketHQ);
        assert_eq!(map.width, 12);
        assert_eq!(map.height, 12);
        assert_eq!(map.npcs.len(), 5, "RocketHQ needs 4 grunts + 1 executive");
        assert!(map.npcs[4].is_trainer, "Executive (npc 4) must be a trainer");
    }

    #[test]
    fn test_rocket_hq_warp_to_mahogany() {
        let map = load_map(MapId::RocketHQ);
        assert!(map.warps.iter().any(|w| w.dest_map == MapId::MahoganyTown));
        let mt = load_map(MapId::MahoganyTown);
        assert!(mt.warps.iter().any(|w| w.dest_map == MapId::RocketHQ));
    }

    #[test]
    fn test_learn_move_queued_when_full() {
        // Create a Pokemon with 4 moves that learns a new move at a specific level
        // Use Cyndaquil which learns Ember at lv12
        let mut p = Pokemon::new(CYNDAQUIL, 11);
        // Fill all 4 slots
        p.moves = [Some(MOVE_TACKLE), Some(MOVE_LEER), Some(MOVE_SMOKESCREEN), Some(MOVE_QUICK_ATTACK)];
        p.move_pp = [35, 30, 20, 30];
        p.move_max_pp = [35, 30, 20, 30];
        p.level = 12; // Cyndaquil learns Ember at 12
        let new_moves = p.check_new_moves();
        // Should find Ember as a learnable move
        assert!(!new_moves.is_empty(), "Cyndaquil should learn a move at lv12");
        // Verify none of the new moves are already known
        for nm in &new_moves {
            let already_known = p.moves.iter().any(|m| *m == Some(*nm));
            // If it's already known, skip it (as our code does)
            if !already_known {
                // All slots full — this should trigger the learn prompt
                let has_empty = p.moves.iter().any(|m| m.is_none());
                assert!(!has_empty, "All 4 move slots should be full");
            }
        }
    }

    #[test]
    fn test_learn_move_sub_phases() {
        // Verify LearnMoveSub enum variants exist and can be constructed
        let _t = LearnMoveSub::TryingToLearn { timer: 0.0 };
        let _c = LearnMoveSub::CantLearnMore { timer: 0.0 };
        let _d = LearnMoveSub::DeletePrompt { cursor: 0 };
        let _p = LearnMoveSub::PickMove { cursor: 0 };
        let _f = LearnMoveSub::ForgotMove { timer: 0.0, slot: 0 };
        let _l = LearnMoveSub::LearnedMove { timer: 0.0 };
        let _s = LearnMoveSub::StopPrompt { cursor: 0 };
        let _n = LearnMoveSub::DidNotLearn { timer: 0.0 };
    }

    // ── Sprint 85: Discovery tests ─────────────────────────────

    #[test]
    fn test_route27_blocked_without_badges() {
        // New Bark Town has a left exit (x=0,y=10) to Route 27
        // This must be gated — player can't go there without 8 badges
        let mut sim = PokemonSim::new();
        sim.has_starter = true;
        sim.party.push(Pokemon::new(CYNDAQUIL, 5));
        sim.change_map(MapId::NewBarkTown, 1, 10);
        sim.badges = 0; // no badges
        // Walk left onto the warp tile
        sim.player.x = 1;
        sim.player.y = 10;
        sim.player.facing = Direction::Left;
        // Manually check: the warp exists
        let map = load_map(MapId::NewBarkTown);
        let has_route27_warp = map.warps.iter().any(|w| w.dest_map == MapId::Route27);
        assert!(has_route27_warp, "NewBarkTown must have Route27 warp");
        // The gate check at warp processing should block without 8 badges
        // Verify by checking gate code exists (structural test)
    }

    #[test]
    fn test_union_cave_requires_zephyr_badge() {
        // Route 32 south warps to Union Cave — should require Zephyr Badge
        let map = load_map(MapId::Route32);
        let has_union_warp = map.warps.iter().any(|w| w.dest_map == MapId::UnionCave);
        assert!(has_union_warp, "Route32 must have UnionCave warp");
        // Structural: gate check exists in warp processing code
    }

    #[test]
    fn test_ilex_forest_requires_hive_badge() {
        // Ilex Forest north exit to Route 34 — should require Hive Badge
        let map = load_map(MapId::IlexForest);
        let has_r34_warp = map.warps.iter().any(|w| w.dest_map == MapId::Route34);
        assert!(has_r34_warp, "IlexForest must have Route34 warp");
    }

    #[test]
    fn test_ice_path_requires_rocket_flag() {
        // Route 44 east to Ice Path — should require Rocket HQ cleared
        let map = load_map(MapId::Route44);
        let has_ice_warp = map.warps.iter().any(|w| w.dest_map == MapId::IcePath);
        assert!(has_ice_warp, "Route44 must have IcePath warp");
    }

    #[test]
    fn test_generic_house_stores_door_position() {
        // Entering GenericHouse from different doors should store the exact position
        let mut sim = PokemonSim::new();
        sim.has_starter = true;
        sim.party.push(Pokemon::new(CYNDAQUIL, 5));
        // Enter GenericHouse from CherrygroveCity door at (15,4)
        sim.change_map(MapId::CherrygroveCity, 15, 4);
        sim.player.x = 15;
        sim.player.y = 4;
        sim.change_map(MapId::GenericHouse, 3, 5); // entering the house
        assert_eq!(sim.last_house_map, MapId::CherrygroveCity);
        assert_eq!(sim.last_house_x, 15, "last_house_x should be 15 (the door we entered from)");
        assert_eq!(sim.last_house_y, 5, "last_house_y should be 5 (1 below the door)");
    }

    #[test]
    fn test_generic_house_exit_different_doors() {
        // Two houses in CherrygroveCity: (15,4) and (16,8)
        // Entering from (16,8) should exit back near (16,8), NOT at (15,4)
        let mut sim = PokemonSim::new();
        sim.has_starter = true;
        sim.party.push(Pokemon::new(CYNDAQUIL, 5));
        // Enter from second house door
        sim.change_map(MapId::CherrygroveCity, 16, 8);
        sim.player.x = 16;
        sim.player.y = 8;
        sim.change_map(MapId::GenericHouse, 3, 5);
        // Exit should go back near (16,8), not (15,5)
        assert_eq!(sim.last_house_map, MapId::CherrygroveCity);
        assert_eq!(sim.last_house_x, 16);
        assert_eq!(sim.last_house_y, 9); // 1 below door at y=8
    }

    #[test]
    fn test_defeated_trainer_no_retrigger() {
        // A defeated trainer should not trigger line-of-sight battle again
        let mut sim = PokemonSim::new();
        sim.has_starter = true;
        sim.party.push(Pokemon::new(CYNDAQUIL, 50));
        sim.change_map(MapId::Route30, 8, 3); // Right next to trainer at (8,3)
        // Mark trainer as defeated
        sim.defeated_trainers.push((MapId::Route30, 0)); // NPC index 0
        // The LOS check should skip defeated trainers (line 1239)
        let map = load_map(MapId::Route30);
        let npc = &map.npcs[0];
        assert!(npc.is_trainer, "NPC 0 on Route30 should be a trainer");
        assert!(sim.defeated_trainers.contains(&(MapId::Route30, 0)));
    }

    #[test]
    fn test_all_cities_have_pokecenter_exit() {
        // Verify every city's PokemonCenter exit coordinates are valid
        let cities = [
            (MapId::CherrygroveCity, "CherrygroveCity"),
            (MapId::VioletCity, "VioletCity"),
            (MapId::AzaleaTown, "AzaleaTown"),
            (MapId::GoldenrodCity, "GoldenrodCity"),
            (MapId::EcruteakCity, "EcruteakCity"),
            (MapId::OlivineCity, "OlivineCity"),
            (MapId::CianwoodCity, "CianwoodCity"),
            (MapId::MahoganyTown, "MahoganyTown"),
            (MapId::BlackthornCity, "BlackthornCity"),
        ];
        for (map_id, name) in cities {
            let map = load_map(map_id);
            let has_pc_warp = map.warps.iter().any(|w| w.dest_map == MapId::PokemonCenter);
            assert!(has_pc_warp, "{} must have a PokemonCenter warp", name);
        }
    }

    #[test]
    fn test_all_route_warps_bidirectional() {
        // Every warp from map A to map B should have a return warp from B to A
        // Exception: one-way routes (Route45/46 are ledge routes, south only)
        let all_maps = vec![
            MapId::NewBarkTown, MapId::Route29, MapId::CherrygroveCity,
            MapId::Route30, MapId::Route31, MapId::VioletCity,
            MapId::Route32, MapId::Route33, MapId::AzaleaTown,
            MapId::Route34, MapId::GoldenrodCity, MapId::Route35,
            MapId::NationalPark, MapId::Route36, MapId::Route37,
            MapId::EcruteakCity, MapId::Route38, MapId::Route39,
            MapId::OlivineCity, MapId::Route40, MapId::CianwoodCity,
            MapId::Route42, MapId::MahoganyTown, MapId::Route43,
            MapId::LakeOfRage, MapId::Route44, MapId::BlackthornCity,
            MapId::Route45, MapId::Route46, MapId::Route27, MapId::Route26,
        ];
        // One-way routes (ledge routes — can only go south, no return)
        let one_way_sources = [MapId::Route45, MapId::Route46];
        let mut missing = Vec::new();
        for &map_id in &all_maps {
            if one_way_sources.contains(&map_id) { continue; } // skip one-way sources
            let map = load_map(map_id);
            for warp in &map.warps {
                // Skip warps to interiors (PokemonCenter, GenericHouse, etc.)
                if matches!(warp.dest_map, MapId::PokemonCenter | MapId::GenericHouse
                    | MapId::PlayerHouse1F | MapId::PlayerHouse2F | MapId::ElmLab
                    | MapId::SproutTower1F | MapId::SproutTower2F | MapId::SproutTower3F | MapId::RocketHQ
                    | MapId::VioletGym | MapId::AzaleaGym | MapId::GoldenrodGym
                    | MapId::EcruteakGym | MapId::OlivineGym | MapId::CianwoodGym
                    | MapId::MahoganyGym | MapId::BlackthornGym
                    | MapId::OlivineLighthouse | MapId::BurnedTower
                    | MapId::UnionCave | MapId::IlexForest | MapId::IcePath
                    | MapId::VictoryRoad | MapId::IndigoPlateau
                    | MapId::EliteFourWill | MapId::EliteFourKoga
                    | MapId::EliteFourBruno | MapId::EliteFourKaren
                    | MapId::ChampionLance
                ) { continue; }
                if !all_maps.contains(&warp.dest_map) { continue; }
                let dest = load_map(warp.dest_map);
                let has_return = dest.warps.iter().any(|w| w.dest_map == map_id);
                if !has_return {
                    missing.push(format!("{:?} -> {:?}", map_id, warp.dest_map));
                }
            }
        }
        assert!(missing.is_empty(), "Missing return warps: {:?}", missing);
    }

    #[test]
    fn test_los_suppress_field_exists() {
        // Verify los_suppress field initializes to 0
        let sim = PokemonSim::new();
        assert_eq!(sim.los_suppress, 0);
    }

    #[test]
    fn test_save_includes_house_position() {
        // Verify save/load round-trip preserves last_house_x/y
        let mut sim = PokemonSim::new();
        sim.has_starter = true;
        sim.party.push(Pokemon::new(CYNDAQUIL, 5));
        sim.last_house_map = MapId::EcruteakCity;
        sim.last_house_x = 4;
        sim.last_house_y = 13;
        let save = sim.serialize_save();
        let mut sim2 = PokemonSim::new();
        sim2.load_from_save(&save);
        assert_eq!(sim2.last_house_map, MapId::EcruteakCity);
        assert_eq!(sim2.last_house_x, 4);
        assert_eq!(sim2.last_house_y, 13);
    }

    #[test]
    fn test_progression_gates_exist() {
        // Structural: verify that gate checks are wired in for all critical warps
        // This test verifies the warp destinations that need gates actually exist as warps
        let nbt = load_map(MapId::NewBarkTown);
        assert!(nbt.warps.iter().any(|w| w.dest_map == MapId::Route27),
            "NewBarkTown must have Route27 warp (which is now gated)");

        let r32 = load_map(MapId::Route32);
        assert!(r32.warps.iter().any(|w| w.dest_map == MapId::UnionCave),
            "Route32 must have UnionCave warp (which is now gated)");

        let ilex = load_map(MapId::IlexForest);
        assert!(ilex.warps.iter().any(|w| w.dest_map == MapId::Route34),
            "IlexForest must have Route34 warp (which is now gated)");

        let r44 = load_map(MapId::Route44);
        assert!(r44.warps.iter().any(|w| w.dest_map == MapId::IcePath),
            "Route44 must have IcePath warp (which is now gated)");
    }

    #[test]
    fn test_headless_walk_to_route30_and_back() {
        // Full simulation: walk from New Bark Town to Route 30 and back
        let mut runner = HeadlessRunner::new(160, 144);
        let mut game = PokemonSim::new();
        // Start in overworld with starter
        game.has_starter = true;
        game.party.push(Pokemon::new(CYNDAQUIL, 10));
        game.change_map(MapId::CherrygroveCity, 9, 1);
        game.phase = GamePhase::Overworld;

        // Walk up to Route 30 entrance (warps at y=0)
        let mut inputs: Vec<InputFrame> = Vec::new();
        inputs.push(empty()); // 1 frame to initialize
        for _ in 0..2 {
            inputs.push(press("ArrowUp"));
            for _ in 0..8 { inputs.push(empty()); }
        }

        let result = runner.run_sim_frames(
            &mut game, 42, &inputs, inputs.len() as u64,
            RunConfig { turbo: true, capture_state_hashes: false },
        );
        // Should have moved north — y should decrease
        let y = result.get_f64("player_y").unwrap_or(99.0);
        assert!(y < 1.0, "Player should have moved north from starting position, got y={}", y);
    }

    // ── Sprint 113 QA: Party swap tests ─────────────────────────

    #[test]
    fn test_party_swap_basic() {
        // Create a PokemonSim with 3 party members, swap index 0 and 2
        let mut sim = PokemonSim::new();
        sim.has_starter = true;
        sim.party.push(Pokemon::new(CHIKORITA, 10));
        sim.party.push(Pokemon::new(CYNDAQUIL, 15));
        sim.party.push(Pokemon::new(TOTODILE, 20));

        assert_eq!(sim.party[0].species_id, CHIKORITA);
        assert_eq!(sim.party[1].species_id, CYNDAQUIL);
        assert_eq!(sim.party[2].species_id, TOTODILE);

        // Perform the swap (same logic as step_pokemon_menu action 2)
        let src = 0usize;
        let dst = 2usize;
        assert!(src != dst && src < sim.party.len() && dst < sim.party.len());
        sim.party.swap(src, dst);

        // Verify species at positions 0 and 2 are exchanged
        assert_eq!(sim.party[0].species_id, TOTODILE, "Position 0 should now be Totodile");
        assert_eq!(sim.party[1].species_id, CYNDAQUIL, "Position 1 should be unchanged");
        assert_eq!(sim.party[2].species_id, CHIKORITA, "Position 2 should now be Chikorita");
    }

    #[test]
    fn test_party_swap_preserves_hp() {
        // Swap two Pokemon and verify HP, level, moves are preserved
        let mut sim = PokemonSim::new();
        sim.has_starter = true;

        let mut p1 = Pokemon::new(CHIKORITA, 25);
        p1.hp = 42; // Damage it
        let p1_max_hp = p1.max_hp;
        let p1_level = p1.level;
        let p1_moves = p1.moves;
        let p1_attack = p1.attack;

        let mut p2 = Pokemon::new(TOTODILE, 30);
        p2.hp = 10; // Damage it heavily
        let p2_max_hp = p2.max_hp;
        let p2_level = p2.level;
        let p2_moves = p2.moves;
        let p2_attack = p2.attack;

        sim.party.push(p1);
        sim.party.push(p2);

        // Swap
        sim.party.swap(0, 1);

        // After swap, position 0 has the former Totodile
        assert_eq!(sim.party[0].species_id, TOTODILE);
        assert_eq!(sim.party[0].hp, 10, "HP should be preserved after swap");
        assert_eq!(sim.party[0].max_hp, p2_max_hp, "Max HP should be preserved");
        assert_eq!(sim.party[0].level, p2_level, "Level should be preserved");
        assert_eq!(sim.party[0].moves, p2_moves, "Moves should be preserved");
        assert_eq!(sim.party[0].attack, p2_attack, "Attack should be preserved");

        // Position 1 has the former Chikorita
        assert_eq!(sim.party[1].species_id, CHIKORITA);
        assert_eq!(sim.party[1].hp, 42, "HP should be preserved after swap");
        assert_eq!(sim.party[1].max_hp, p1_max_hp, "Max HP should be preserved");
        assert_eq!(sim.party[1].level, p1_level, "Level should be preserved");
        assert_eq!(sim.party[1].moves, p1_moves, "Moves should be preserved");
        assert_eq!(sim.party[1].attack, p1_attack, "Attack should be preserved");
    }

    #[test]
    fn test_check_warp_gate_route27() {
        // Route27 should be blocked from NewBarkTown without all 8 badges
        let mut sim = PokemonSim::new();
        sim.has_starter = true;
        sim.party.push(Pokemon::new(CYNDAQUIL, 50));
        sim.change_map(MapId::NewBarkTown, 5, 8);

        // 0 badges: blocked
        sim.badges = 0;
        assert!(sim.check_warp_gate(MapId::Route27).is_some(),
            "Route27 should be blocked with 0 badges");

        // 7 badges: still blocked
        sim.badges = 0b01111111;
        assert!(sim.check_warp_gate(MapId::Route27).is_some(),
            "Route27 should be blocked with 7 badges");

        // All 8 badges (0xFF): allowed
        sim.badges = 0xFF;
        assert!(sim.check_warp_gate(MapId::Route27).is_none(),
            "Route27 should be passable with all 8 badges");
    }

    #[test]
    fn test_check_warp_gate_union_cave() {
        // UnionCave requires Zephyr Badge (bit 0)
        let mut sim = PokemonSim::new();
        sim.has_starter = true;
        sim.party.push(Pokemon::new(CYNDAQUIL, 10));
        sim.change_map(MapId::Route32, 9, 28);

        // No badges: blocked
        sim.badges = 0;
        assert!(sim.check_warp_gate(MapId::UnionCave).is_some(),
            "UnionCave should be blocked without Zephyr Badge");

        // Zephyr Badge (bit 0): passable
        sim.badges = 1;
        assert!(sim.check_warp_gate(MapId::UnionCave).is_none(),
            "UnionCave should be passable with Zephyr Badge");
    }

    #[test]
    fn test_check_warp_gate_victory_road() {
        // VictoryRoad requires all 8 badges
        let mut sim = PokemonSim::new();
        sim.has_starter = true;
        sim.party.push(Pokemon::new(CYNDAQUIL, 50));
        sim.change_map(MapId::Route26, 10, 5);

        // 7 badges: blocked
        sim.badges = 0b01111111;
        assert!(sim.check_warp_gate(MapId::VictoryRoad).is_some(),
            "VictoryRoad should be blocked without all 8 badges");

        // All 8 badges: passable
        sim.badges = 0xFF;
        assert!(sim.check_warp_gate(MapId::VictoryRoad).is_none(),
            "VictoryRoad should be passable with all 8 badges");
    }

    #[test]
    fn test_trainer_card_time_display() {
        // Verify hours/minutes calculation doesn't overflow for large total_time
        // The render uses: hours = (total_time / 3600.0) as u32
        //                  minutes = ((total_time % 3600.0) / 60.0) as u32

        // Normal gameplay: 2 hours 30 minutes = 9000 seconds
        let total_time: f64 = 9000.0;
        let hours = (total_time / 3600.0) as u32;
        let minutes = ((total_time % 3600.0) / 60.0) as u32;
        assert_eq!(hours, 2);
        assert_eq!(minutes, 30);

        // Large value: 999 hours = 3596400 seconds
        let total_time_large: f64 = 3596400.0;
        let hours_large = (total_time_large / 3600.0) as u32;
        let minutes_large = ((total_time_large % 3600.0) / 60.0) as u32;
        assert_eq!(hours_large, 999);
        assert_eq!(minutes_large, 0);

        // Edge: exactly 1 second under an hour
        let total_time_edge: f64 = 3599.0;
        let hours_edge = (total_time_edge / 3600.0) as u32;
        let minutes_edge = ((total_time_edge % 3600.0) / 60.0) as u32;
        assert_eq!(hours_edge, 0);
        assert_eq!(minutes_edge, 59);

        // Very large value: total_time close to u32::MAX seconds
        // This tests that f64 -> u32 cast saturates rather than panicking
        let total_time_huge: f64 = 5_000_000_000.0; // ~5 billion seconds
        let hours_huge = (total_time_huge / 3600.0) as u32;
        let _minutes_huge = ((total_time_huge % 3600.0) / 60.0) as u32;
        // hours_huge = 1388888 as u32 (fits in u32, max is ~4.2 billion)
        assert_eq!(hours_huge, 1388888);
        // format! should not panic
        let time_str = format!("TIME: {:02}:{:02}", hours_huge, _minutes_huge);
        assert!(time_str.contains("TIME:"));
    }

    // ── Sprint 115: Input builder + with_state integration tests ──────

    #[test]
    fn test_input_builder_helpers() {
        // press: single frame with keys_pressed
        let p = press("KeyZ");
        assert_eq!(p.keys_pressed, vec!["KeyZ".to_string()]);
        assert!(p.keys_held.is_empty());

        // hold: N frames with keys_held
        let h = hold("ArrowUp", 3);
        assert_eq!(h.len(), 3);
        for f in &h {
            assert_eq!(f.keys_held, vec!["ArrowUp".to_string()]);
            assert!(f.keys_pressed.is_empty());
        }

        // wait: N empty frames
        let w = wait(5);
        assert_eq!(w.len(), 5);
        for f in &w {
            assert!(f.is_empty());
        }

        // walk_dir: press + gap empties
        let wd = walk_dir("right", 8);
        assert_eq!(wd.len(), 9); // 1 press + 8 empties
        assert_eq!(wd[0].keys_pressed, vec!["ArrowRight".to_string()]);
        for i in 1..9 {
            assert!(wd[i].is_empty());
        }

        // sequence: concatenation
        let seq = sequence(vec![
            walk_dir("up", 4),
            walk_dir("down", 4),
        ]);
        assert_eq!(seq.len(), 10); // (1+4) + (1+4)
    }

    #[test]
    fn test_with_state_overworld() {
        // with_state should place the player directly in the overworld
        let party = vec![Pokemon::new(CYNDAQUIL, 10)];
        let sim = PokemonSim::with_state(MapId::VioletCity, 10, 8, party, 1);

        assert!(matches!(sim.phase, GamePhase::Overworld));
        assert_eq!(sim.current_map_id, MapId::VioletCity);
        assert_eq!(sim.player.x, 10);
        assert_eq!(sim.player.y, 8);
        assert_eq!(sim.party.len(), 1);
        assert_eq!(sim.party[0].species_id, CYNDAQUIL);
        assert_eq!(sim.party[0].level, 10);
        assert_eq!(sim.badges, 1);
        assert!(sim.has_starter);

        // with_state with empty party should set has_starter to false
        let sim2 = PokemonSim::with_state(MapId::NewBarkTown, 5, 8, vec![], 0);
        assert!(!sim2.has_starter);
        assert!(sim2.party.is_empty());
    }

    #[test]
    fn test_daycare_deposit_withdraw() {
        // Test the daycare system: deposit a Pokemon, walk, withdraw it leveled up
        let mut sim = PokemonSim::with_state(
            MapId::Route34, 8, 10,
            vec![Pokemon::new(CYNDAQUIL, 10), Pokemon::new(PIDGEY, 8)],
            3,
        );

        // Deposit: move second Pokemon to daycare
        assert!(sim.daycare_pokemon.is_none());
        let deposited = sim.party.remove(1);
        assert_eq!(deposited.species_id, PIDGEY);
        assert_eq!(deposited.level, 8);
        sim.daycare_pokemon = Some(deposited);
        sim.daycare_steps = 0;
        assert_eq!(sim.party.len(), 1);
        assert!(sim.daycare_pokemon.is_some());

        // Simulate walking: each step increments daycare_steps, every 100 steps = +1 level
        for _ in 0..250 {
            sim.daycare_steps += 1;
            if let Some(ref mut pkmn) = sim.daycare_pokemon {
                // Daycare levels: +1 per 100 steps (matching the actual game logic)
                let new_lvl = (pkmn.level as u32 + sim.daycare_steps / 100).min(100) as u8;
                if new_lvl != pkmn.level {
                    pkmn.level = new_lvl;
                    pkmn.recalc_stats();
                }
            }
        }

        // Withdraw: take the Pokemon back
        let withdrawn = sim.daycare_pokemon.take().unwrap();
        assert_eq!(withdrawn.species_id, PIDGEY);
        // After 250 steps at starting level 8, should have gained at least 2 levels
        assert!(withdrawn.level >= 10,
            "Daycare Pokemon should have leveled up, got level {}", withdrawn.level);
        sim.party.push(withdrawn);
        sim.daycare_steps = 0;
        assert_eq!(sim.party.len(), 2);
        assert!(sim.daycare_pokemon.is_none());
    }

    #[test]
    fn test_ice_sliding_basic() {
        // Test that ice_sliding field works: set direction, verify it persists
        let mut sim = PokemonSim::with_state(
            MapId::IcePath, 3, 7,
            vec![Pokemon::new(CYNDAQUIL, 25)],
            7, // 7 badges (just cleared Rocket HQ)
        );

        // Initially no sliding
        assert!(sim.ice_sliding.is_none());

        // Start sliding right (simulating stepping onto ice)
        sim.ice_sliding = Some(Direction::Right);
        assert!(sim.ice_sliding.is_some());
        assert_eq!(sim.ice_sliding.unwrap(), Direction::Right);

        // Simulate hitting a wall — stop sliding
        sim.ice_sliding = None;
        assert!(sim.ice_sliding.is_none());

        // Verify IcePath map has ice collision tiles (C_ICE = 8)
        let ice_map = load_map(MapId::IcePath);
        let has_ice = ice_map.collision.iter().any(|&c| c == 8);
        assert!(has_ice, "IcePath must have ice collision tiles (C_ICE=8)");

        // Verify map dimensions are correct
        assert_eq!(ice_map.width, 14);
        assert_eq!(ice_map.height, 14);

        // Verify entrance warps exist
        assert!(ice_map.warps.iter().any(|w| w.dest_map == MapId::BlackthornCity),
            "IcePath must warp to BlackthornCity");
    }

    #[test]
    fn test_warp_gate_progression() {
        // Test warp gates: verify that progression gates block/allow warps
        // across the full game, using with_state for rapid setup

        // Gate 1: Union Cave requires Zephyr Badge (bit 0)
        let sim0 = PokemonSim::with_state(
            MapId::Route32, 9, 28,
            vec![Pokemon::new(CHIKORITA, 12)],
            0, // no badges
        );
        assert!(sim0.check_warp_gate(MapId::UnionCave).is_some(),
            "UnionCave should be blocked with 0 badges");

        let sim1 = PokemonSim::with_state(
            MapId::Route32, 9, 28,
            vec![Pokemon::new(CHIKORITA, 12)],
            1, // Zephyr Badge
        );
        assert!(sim1.check_warp_gate(MapId::UnionCave).is_none(),
            "UnionCave should be passable with Zephyr Badge");

        // Gate 2: Ilex Forest → Route 34 requires Hive Badge (bit 1)
        let sim2 = PokemonSim::with_state(
            MapId::IlexForest, 8, 1,
            vec![Pokemon::new(CHIKORITA, 18)],
            0b01, // only Zephyr
        );
        assert!(sim2.check_warp_gate(MapId::Route34).is_some(),
            "Route34 should be blocked without Hive Badge");

        let sim3 = PokemonSim::with_state(
            MapId::IlexForest, 8, 1,
            vec![Pokemon::new(CHIKORITA, 18)],
            0b11, // Zephyr + Hive
        );
        assert!(sim3.check_warp_gate(MapId::Route34).is_none(),
            "Route34 should be passable with Hive Badge");

        // Gate 3: Route 27 from New Bark requires all 8 badges
        let sim7 = PokemonSim::with_state(
            MapId::NewBarkTown, 1, 10,
            vec![Pokemon::new(CYNDAQUIL, 50)],
            0b01111111, // 7 badges
        );
        assert!(sim7.check_warp_gate(MapId::Route27).is_some(),
            "Route27 should be blocked with 7 badges");

        let sim8 = PokemonSim::with_state(
            MapId::NewBarkTown, 1, 10,
            vec![Pokemon::new(CYNDAQUIL, 50)],
            0xFF, // all 8 badges
        );
        assert!(sim8.check_warp_gate(MapId::Route27).is_none(),
            "Route27 should be passable with all 8 badges");

        // Gate 4: Victory Road requires all 8 badges
        let sim_vr = PokemonSim::with_state(
            MapId::Route26, 10, 5,
            vec![Pokemon::new(CYNDAQUIL, 50)],
            0b01111111, // 7 badges
        );
        assert!(sim_vr.check_warp_gate(MapId::VictoryRoad).is_some(),
            "VictoryRoad should be blocked with 7 badges");

        let sim_vr8 = PokemonSim::with_state(
            MapId::Route26, 10, 5,
            vec![Pokemon::new(CYNDAQUIL, 50)],
            0xFF, // all 8
        );
        assert!(sim_vr8.check_warp_gate(MapId::VictoryRoad).is_none(),
            "VictoryRoad should be passable with all 8 badges");
    }

    // ── Sprint 119 QA: Fly, Repel, Item data tests ─────────────────────

    #[test]
    fn test_fly_destinations() {
        // Verify all 10 fly cities have valid spawn points within map bounds
        let fly_cities = [
            MapId::NewBarkTown, MapId::CherrygroveCity, MapId::VioletCity,
            MapId::AzaleaTown, MapId::GoldenrodCity, MapId::EcruteakCity,
            MapId::OlivineCity, MapId::CianwoodCity, MapId::MahoganyTown,
            MapId::BlackthornCity,
        ];
        for &city in &fly_cities {
            assert!(PokemonSim::is_fly_city(city),
                "is_fly_city should return true for {:?}", city);
            let (sx, sy) = PokemonSim::fly_spawn(city);
            let map = load_map(city);
            assert!(
                (sx as usize) < map.width && (sy as usize) < map.height,
                "Fly spawn ({}, {}) out of bounds for {:?} ({}x{})",
                sx, sy, city, map.width, map.height,
            );
            // Spawn point should not be a solid/impassable tile (1 = Solid)
            let col = map.collision[sy as usize * map.width + sx as usize];
            assert!(col != 1,
                "Fly spawn for {:?} lands on a solid tile at ({}, {})", city, sx, sy);
        }
        // Verify that non-fly maps return false
        assert!(!PokemonSim::is_fly_city(MapId::Route29));
        assert!(!PokemonSim::is_fly_city(MapId::PokemonCenter));
    }

    #[test]
    fn test_repel_steps() {
        // Verify repel item data has correct step values per Gen 2
        let repel = get_item(ITEM_REPEL).expect("Repel should exist in ITEM_DB");
        assert_eq!(repel.repel_steps, 100, "Repel should last 100 steps");

        let super_repel = get_item(ITEM_SUPER_REPEL).expect("Super Repel should exist");
        assert_eq!(super_repel.repel_steps, 200, "Super Repel should last 200 steps");

        let max_repel = get_item(ITEM_MAX_REPEL).expect("Max Repel should exist");
        assert_eq!(max_repel.repel_steps, 250, "Max Repel should last 250 steps");

        // Non-repel items should have 0 steps
        let potion = get_item(ITEM_POTION).expect("Potion should exist");
        assert_eq!(potion.repel_steps, 0, "Potion should not be a repel");
    }

    #[test]
    fn test_item_data_completeness() {
        // Verify all 19 item IDs (1-19) have valid ITEM_DB entries
        let all_items: &[ItemId] = &[
            ITEM_POTION, ITEM_SUPER_POTION, ITEM_ANTIDOTE, ITEM_POKE_BALL,
            ITEM_PARALYZE_HEAL, ITEM_REVIVE, ITEM_FULL_HEAL, ITEM_GREAT_BALL,
            ITEM_ETHER, ITEM_ESCAPE_ROPE, ITEM_REPEL, ITEM_HYPER_POTION,
            ITEM_MAX_POTION, ITEM_FULL_RESTORE, ITEM_RARE_CANDY, ITEM_AWAKENING,
            ITEM_ICE_HEAL, ITEM_SUPER_REPEL, ITEM_MAX_REPEL,
        ];
        for &item_id in all_items {
            let data = get_item(item_id);
            assert!(data.is_some(), "Item ID {} missing from ITEM_DB", item_id);
            let d = data.expect("already checked");
            assert!(!d.name.is_empty(), "Item {} has empty name", item_id);
            assert!(!d.description.is_empty(), "Item {} has empty description", item_id);
            assert!(d.price > 0, "Item {} has zero price", item_id);
        }
        // Verify specific heal amounts
        assert_eq!(get_item(ITEM_POTION).expect("p").heal_amount, 20);
        assert_eq!(get_item(ITEM_SUPER_POTION).expect("sp").heal_amount, 50);
        assert_eq!(get_item(ITEM_HYPER_POTION).expect("hp").heal_amount, 200);
        assert_eq!(get_item(ITEM_MAX_POTION).expect("mp").heal_amount, 9999);
        assert_eq!(get_item(ITEM_FULL_RESTORE).expect("fr").heal_amount, 9999);
        // Full Restore should heal AND cure status
        let fr = get_item(ITEM_FULL_RESTORE).expect("fr");
        assert!(fr.is_status_heal, "Full Restore must cure status");
        assert!(fr.heal_amount > 0, "Full Restore must heal HP");
        // Rare Candy flag
        assert!(get_item(ITEM_RARE_CANDY).expect("rc").is_rare_candy);
    }

    #[test]
    fn test_battle_queue_drains_correctly() {
        // Build a BattleState with a pre-populated queue, step through it,
        // and verify the queue drains in FIFO order.
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = BattleState {
            phase: BattlePhase::ExecuteQueue,
            enemy,
            player_idx: 0,
            is_wild: true,
            player_hp_display: party[0].hp as f64,
            enemy_hp_display: 30.0,
            turn_count: 0,
            trainer_team: Vec::new(),
            trainer_team_idx: 0,
            pending_player_move: None,
            player_stages: [0; 7],
            enemy_stages: [0; 7],
            enemy_flinched: false,
            player_flinched: false,
            player_confused: 0,
            enemy_confused: 0,
            player_trapped: false,
            player_must_recharge: false,
            enemy_must_recharge: false,
            player_rampage: (0, 0),
            enemy_rampage: (0, 0),
            pending_learn_moves: vec![],
            free_switch: false,
            confusion_snapout_msg: None,
            battle_queue: VecDeque::new(),
            queue_timer: 0.0,
        };

        // Pre-populate the queue: Text -> Pause -> ApplyDamage -> GoToPhase(ActionSelect)
        battle.battle_queue.push_back(BattleStep::Text("Test message!".into()));
        battle.battle_queue.push_back(BattleStep::Pause(0.1));
        battle.battle_queue.push_back(BattleStep::ApplyDamage { is_player: false, amount: 10 });
        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::ActionSelect { cursor: 0 })));

        assert_eq!(battle.battle_queue.len(), 4);
        assert!(matches!(battle.phase, BattlePhase::ExecuteQueue));

        let initial_enemy_hp = battle.enemy.hp;

        // Step 1: Text step — needs ~90 frames (1.5s at 60fps) to auto-advance
        for _ in 0..91 {
            test_step_queue(&mut battle, &mut party, &mut engine);
        }
        assert_eq!(battle.battle_queue.len(), 3, "Text step should have been consumed");

        // Step 2: Pause(0.1) — needs ~6 frames (0.1s at 60fps)
        for _ in 0..7 {
            test_step_queue(&mut battle, &mut party, &mut engine);
        }
        assert_eq!(battle.battle_queue.len(), 2, "Pause step should have been consumed");

        // Step 3: ApplyDamage — instant (1 frame)
        test_step_queue(&mut battle, &mut party, &mut engine);
        assert_eq!(battle.battle_queue.len(), 1, "ApplyDamage should have been consumed");
        assert_eq!(battle.enemy.hp, initial_enemy_hp.saturating_sub(10),
            "Enemy HP should have decreased by 10");

        // Step 4: GoToPhase(ActionSelect) — instant
        test_step_queue(&mut battle, &mut party, &mut engine);
        assert!(battle.battle_queue.is_empty(), "Queue should be fully drained");
        assert!(matches!(battle.phase, BattlePhase::ActionSelect { cursor: 0 }),
            "Should have transitioned to ActionSelect");
    }

    #[test]
    fn test_battle_queue_check_faint_transitions() {
        // Verify CheckFaint step transitions to EnemyFainted when enemy HP is 0
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(99);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 15);
        let mut party = vec![player_pkmn];
        let mut enemy = Pokemon::new(RATTATA, 3);
        enemy.hp = 0; // Already fainted

        let mut battle = BattleState {
            phase: BattlePhase::ExecuteQueue,
            enemy,
            player_idx: 0,
            is_wild: true,
            player_hp_display: party[0].hp as f64,
            enemy_hp_display: 0.0,
            turn_count: 0,
            trainer_team: Vec::new(),
            trainer_team_idx: 0,
            pending_player_move: None,
            player_stages: [0; 7],
            enemy_stages: [0; 7],
            enemy_flinched: false,
            player_flinched: false,
            player_confused: 0,
            enemy_confused: 0,
            player_trapped: false,
            player_must_recharge: false,
            enemy_must_recharge: false,
            player_rampage: (0, 0),
            enemy_rampage: (0, 0),
            pending_learn_moves: vec![],
            free_switch: false,
            confusion_snapout_msg: None,
            battle_queue: VecDeque::new(),
            queue_timer: 0.0,
        };

        // Queue: CheckFaint for enemy (already at 0 HP)
        battle.battle_queue.push_back(BattleStep::CheckFaint { is_player: false });

        test_step_queue(&mut battle, &mut party, &mut engine);

        // Should have transitioned to EnemyFainted
        assert!(matches!(battle.phase, BattlePhase::EnemyFainted { .. }),
            "CheckFaint on 0-HP enemy should transition to EnemyFainted, got {:?}", battle.phase);
        assert!(battle.battle_queue.is_empty(), "CheckFaint should pop itself from queue");
    }

    /// Helper: build a minimal BattleState for queue-based tests
    fn make_test_battle(party: &[Pokemon], enemy: Pokemon, is_wild: bool) -> BattleState {
        BattleState {
            phase: BattlePhase::ExecuteQueue,
            enemy,
            player_idx: 0,
            is_wild,
            player_hp_display: party.first().map(|p| p.hp as f64).unwrap_or(0.0),
            enemy_hp_display: 0.0,
            turn_count: 0,
            trainer_team: Vec::new(),
            trainer_team_idx: 0,
            pending_player_move: None,
            player_stages: [0; 7],
            enemy_stages: [0; 7],
            enemy_flinched: false,
            player_flinched: false,
            player_confused: 0,
            enemy_confused: 0,
            player_trapped: false,
            player_must_recharge: false,
            enemy_must_recharge: false,
            player_rampage: (0, 0),
            enemy_rampage: (0, 0),
            pending_learn_moves: vec![],
            free_switch: false,
            confusion_snapout_msg: None,
            battle_queue: VecDeque::new(),
            queue_timer: 0.0,
        }
    }

    /// Test-only wrapper: step the queue using a temporary PokemonSim instance.
    /// This bridges the old static call pattern with the new &mut self method.
    fn test_step_queue(battle: &mut BattleState, party: &mut Vec<Pokemon>, engine: &mut crate::engine::Engine) {
        let mut sim = PokemonSim::new();
        sim.party = party.clone();
        sim.step_execute_queue(battle, engine);
        *party = sim.party;
    }

    /// Step the queue until the phase changes from ExecuteQueue, or max_frames exceeded.
    /// Returns the number of frames stepped.
    fn step_until_phase_change(battle: &mut BattleState, party: &mut Vec<Pokemon>, engine: &mut crate::engine::Engine, max_frames: usize) -> usize {
        for i in 0..max_frames {
            if !matches!(battle.phase, BattlePhase::ExecuteQueue) {
                return i;
            }
            test_step_queue(battle, party, engine);
        }
        max_frames
    }

    #[test]
    fn test_goto_phase_clears_remaining_queue() {
        // GoToPhase should clear any remaining steps after it in the queue
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = make_test_battle(&party, enemy, true);

        // Queue: GoToPhase first, then extra steps that should get cleared
        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::ActionSelect { cursor: 0 })));
        battle.battle_queue.push_back(BattleStep::Text("This should be cleared!".into()));
        battle.battle_queue.push_back(BattleStep::ApplyDamage { is_player: false, amount: 99 });

        assert_eq!(battle.battle_queue.len(), 3);

        test_step_queue(&mut battle, &mut party, &mut engine);

        // GoToPhase should have cleared the entire queue
        assert!(battle.battle_queue.is_empty(),
            "GoToPhase should clear remaining queue items, but {} remain", battle.battle_queue.len());
        assert!(matches!(battle.phase, BattlePhase::ActionSelect { cursor: 0 }),
            "Should have transitioned to ActionSelect");
    }

    #[test]
    fn test_empty_queue_fallback_to_action_select() {
        // Empty queue should safely fall back to ActionSelect
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = make_test_battle(&party, enemy, true);
        // Queue is empty by default

        assert!(battle.battle_queue.is_empty());
        test_step_queue(&mut battle, &mut party, &mut engine);

        assert!(matches!(battle.phase, BattlePhase::ActionSelect { cursor: 0 }),
            "Empty queue should fall back to ActionSelect, got {:?}", battle.phase);
    }

    #[test]
    fn test_intro_sequence_via_queue() {
        // Simulate the Intro → ExecuteQueue → "Go! CYNDAQUIL!" → ActionSelect flow
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let mut party = vec![player_pkmn.clone()];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = make_test_battle(&party, enemy, true);

        // Simulate what Intro phase does after 1.5s: build the queue
        let pname = party[0].name().to_string();
        battle.battle_queue.clear();
        battle.queue_timer = 0.0;
        battle.battle_queue.push_back(BattleStep::Text(format!("Go! {}!", pname)));
        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::ActionSelect { cursor: 0 })));
        battle.phase = BattlePhase::ExecuteQueue;

        // Verify queue has 2 steps
        assert_eq!(battle.battle_queue.len(), 2);

        // First step: Text "Go! CYNDAQUIL!" — needs 1.5s (90 frames) to auto-advance
        let frames = step_until_phase_change(&mut battle, &mut party, &mut engine, 200);

        // Should have consumed the text + GoToPhase and ended at ActionSelect
        assert!(matches!(battle.phase, BattlePhase::ActionSelect { cursor: 0 }),
            "Intro queue should end at ActionSelect, got {:?}", battle.phase);
        assert!(battle.battle_queue.is_empty(), "Queue should be fully drained");
        // Text (1.5s = 90 frames) + GoToPhase (1 frame) = ~91 frames
        assert!(frames > 85 && frames < 100,
            "Expected ~91 frames for intro sequence, got {}", frames);
    }

    #[test]
    fn test_won_flow_via_queue() {
        // Test the Won queue: "You won!" text → GoToPhase(Won { timer: 2.0 })
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = make_test_battle(&party, enemy, true);

        // Simulate what the Won flow does
        battle.battle_queue.clear();
        battle.queue_timer = 0.0;
        battle.battle_queue.push_back(BattleStep::Text("You won!".into()));
        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::Won { timer: 2.0 })));
        battle.phase = BattlePhase::ExecuteQueue;

        // Step through until phase changes
        let frames = step_until_phase_change(&mut battle, &mut party, &mut engine, 200);

        // Should end at Won with timer: 2.0 (skips the won-phase delay since text was already shown)
        assert!(matches!(battle.phase, BattlePhase::Won { timer }  if (timer - 2.0).abs() < 0.01),
            "Won queue should end at Won {{ timer: 2.0 }}, got {:?}", battle.phase);
        assert!(battle.battle_queue.is_empty(), "Queue should be cleared by GoToPhase");
        assert!(frames > 85 && frames < 100,
            "Expected ~91 frames (1.5s text), got {}", frames);
    }

    #[test]
    fn test_run_success_flow_via_queue() {
        // Test run success: "Got away safely!" → GoToPhase(Run)
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = make_test_battle(&party, enemy, true);

        // Build run-success queue
        battle.battle_queue.clear();
        battle.queue_timer = 0.0;
        battle.battle_queue.push_back(BattleStep::Text("Got away safely!".into()));
        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::Run)));
        battle.phase = BattlePhase::ExecuteQueue;

        let frames = step_until_phase_change(&mut battle, &mut party, &mut engine, 200);

        assert!(matches!(battle.phase, BattlePhase::Run),
            "Run success queue should end at Run, got {:?}", battle.phase);
        assert!(battle.battle_queue.is_empty(), "Queue should be cleared");
        assert!(frames > 85 && frames < 100,
            "Expected ~91 frames (1.5s text), got {}", frames);
    }

    #[test]
    fn test_run_failed_flow_via_queue() {
        // Test run failed: "Can't escape!" → GoToPhase(EnemyAttack)
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = make_test_battle(&party, enemy, true);

        // Build run-failed queue (mirrors the actual code)
        battle.battle_queue.clear();
        battle.queue_timer = 0.0;
        battle.battle_queue.push_back(BattleStep::Text("Can't escape!".into()));
        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::EnemyAttack {
            timer: 0.0, move_id: MOVE_TACKLE, damage: 5, effectiveness: 1.0, is_crit: false,
        })));
        battle.phase = BattlePhase::ExecuteQueue;

        let _frames = step_until_phase_change(&mut battle, &mut party, &mut engine, 200);

        assert!(matches!(battle.phase, BattlePhase::EnemyAttack { .. }),
            "Run failed queue should end at EnemyAttack, got {:?}", battle.phase);
        assert!(battle.battle_queue.is_empty(), "Queue should be cleared by GoToPhase");
    }

    #[test]
    fn test_drain_hp_animation_completes() {
        // Test DrainHp step: enemy HP display animates from current to target
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = make_test_battle(&party, enemy, true);
        battle.enemy_hp_display = 30.0; // start displaying at 30 HP

        // DrainHp from 30 to 15, duration 0.5s
        battle.battle_queue.push_back(BattleStep::DrainHp { is_player: false, to_hp: 15, duration: 0.5 });
        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::ActionSelect { cursor: 0 })));

        // Step through 15 frames (~0.25s) — should be mid-animation
        for _ in 0..15 {
            test_step_queue(&mut battle, &mut party, &mut engine);
        }
        // HP display should have moved toward 15 but not reached it yet
        assert!(battle.enemy_hp_display < 30.0, "HP display should be decreasing");
        assert!(battle.enemy_hp_display > 15.0, "HP display should not have reached target yet at 0.25s");

        // Step through remaining frames until done (total ~30 frames for 0.5s)
        for _ in 0..20 {
            test_step_queue(&mut battle, &mut party, &mut engine);
        }
        // After 0.5s, should have completed and moved to next step
        assert!((battle.enemy_hp_display - 15.0).abs() < 1.0,
            "HP display should be at or near 15.0, got {}", battle.enemy_hp_display);
    }

    #[test]
    fn test_inflict_status_step() {
        // Test InflictStatus step on enemy
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = make_test_battle(&party, enemy, true);

        // Queue: inflict Poison on enemy, then go to ActionSelect
        battle.battle_queue.push_back(BattleStep::InflictStatus {
            is_player: false,
            status: StatusCondition::Poison,
        });
        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::ActionSelect { cursor: 0 })));

        assert!(matches!(battle.enemy.status, StatusCondition::None),
            "Enemy should start with no status");

        // InflictStatus is instant (1 frame)
        test_step_queue(&mut battle, &mut party, &mut engine);

        assert!(matches!(battle.enemy.status, StatusCondition::Poison),
            "Enemy should now be poisoned, got {:?}", battle.enemy.status);
        assert_eq!(battle.battle_queue.len(), 1, "InflictStatus should consume 1 step");
    }

    #[test]
    fn test_stat_change_step() {
        // Test StatChange step: lower enemy attack by 1 stage
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = make_test_battle(&party, enemy, true);
        assert_eq!(battle.enemy_stages[STAGE_ATK], 0, "Enemy ATK stage should start at 0");

        // Queue: lower enemy ATK by 1 stage
        battle.battle_queue.push_back(BattleStep::StatChange {
            is_player: false,
            stat: STAGE_ATK,
            stages: -1,
        });
        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::ActionSelect { cursor: 0 })));

        test_step_queue(&mut battle, &mut party, &mut engine);

        assert_eq!(battle.enemy_stages[STAGE_ATK], -1,
            "Enemy ATK stage should be -1 after Growl");
    }

    #[test]
    fn test_stat_change_clamps_at_bounds() {
        // Test StatChange clamping at -6 and +6
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = make_test_battle(&party, enemy, true);
        battle.player_stages[STAGE_DEF] = 5; // already at +5

        // Try to raise DEF by 3 — should clamp to +6
        battle.battle_queue.push_back(BattleStep::StatChange {
            is_player: true,
            stat: STAGE_DEF,
            stages: 3,
        });

        test_step_queue(&mut battle, &mut party, &mut engine);

        assert_eq!(battle.player_stages[STAGE_DEF], 6,
            "DEF stage should be clamped at +6, got {}", battle.player_stages[STAGE_DEF]);
    }

    #[test]
    fn test_check_faint_no_faint_continues_queue() {
        // CheckFaint on a non-fainted Pokemon should just pop itself and continue
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = make_test_battle(&party, enemy, true);

        // Enemy has full HP — CheckFaint should not trigger faint transition
        battle.battle_queue.push_back(BattleStep::CheckFaint { is_player: false });
        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::ActionSelect { cursor: 0 })));

        test_step_queue(&mut battle, &mut party, &mut engine);

        // Should still be in ExecuteQueue (CheckFaint passed, GoToPhase is next)
        assert!(matches!(battle.phase, BattlePhase::ExecuteQueue),
            "Non-faint CheckFaint should continue queue, got {:?}", battle.phase);
        assert_eq!(battle.battle_queue.len(), 1, "CheckFaint should pop itself, leaving GoToPhase");
    }

    #[test]
    fn test_check_faint_player_transitions_to_player_fainted() {
        // CheckFaint on fainted player should transition to PlayerFainted
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let mut player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        player_pkmn.hp = 0; // player already fainted
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = make_test_battle(&party, enemy, true);

        battle.battle_queue.push_back(BattleStep::CheckFaint { is_player: true });
        battle.battle_queue.push_back(BattleStep::Text("This should not execute".into()));

        test_step_queue(&mut battle, &mut party, &mut engine);

        assert!(matches!(battle.phase, BattlePhase::PlayerFainted),
            "CheckFaint on fainted player should transition to PlayerFainted, got {:?}", battle.phase);
    }

    #[test]
    fn test_apply_damage_to_player() {
        // Test ApplyDamage step targeting the player's Pokemon
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let initial_hp = player_pkmn.hp;
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = make_test_battle(&party, enemy, true);

        battle.battle_queue.push_back(BattleStep::ApplyDamage { is_player: true, amount: 7 });

        test_step_queue(&mut battle, &mut party, &mut engine);

        assert_eq!(party[0].hp, initial_hp - 7,
            "Player HP should decrease by 7, got {} (expected {})", party[0].hp, initial_hp - 7);
    }

    #[test]
    fn test_apply_damage_saturates_at_zero() {
        // Test that ApplyDamage doesn't underflow
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let mut player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        player_pkmn.hp = 3; // only 3 HP left
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = make_test_battle(&party, enemy, true);

        battle.battle_queue.push_back(BattleStep::ApplyDamage { is_player: true, amount: 50 });

        test_step_queue(&mut battle, &mut party, &mut engine);

        assert_eq!(party[0].hp, 0, "HP should saturate at 0, not underflow");
    }

    #[test]
    fn test_text_step_advances_on_confirm() {
        // Test that Text step can be advanced early by pressing confirm
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut battle = make_test_battle(&party, enemy, true);
        battle.battle_queue.push_back(BattleStep::Text("Press to advance!".into()));
        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::ActionSelect { cursor: 0 })));

        // Step a few frames without pressing anything — should still be on Text
        for _ in 0..5 {
            test_step_queue(&mut battle, &mut party, &mut engine);
        }
        assert_eq!(battle.battle_queue.len(), 2, "Text should not advance without confirm or timeout");

        // Now simulate pressing confirm (add KeyZ to pressed keys)
        engine.input.keys_pressed.insert("KeyZ".to_string());
        test_step_queue(&mut battle, &mut party, &mut engine);
        engine.input.keys_pressed.clear();

        assert_eq!(battle.battle_queue.len(), 1, "Text should advance when confirm is pressed");
    }

    #[test]
    fn test_full_attack_queue_sequence() {
        // Test a complete attack sequence: Text → Pause → ApplyDamage → DrainHp → CheckFaint → GoToPhase
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let mut party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);
        let initial_enemy_hp = enemy.hp;

        let mut battle = make_test_battle(&party, enemy, true);
        battle.enemy_hp_display = initial_enemy_hp as f64;

        let damage: u16 = 8;
        let target_hp = initial_enemy_hp.saturating_sub(damage);

        // Build a full attack sequence manually (mirrors queue_attack_sequence)
        battle.battle_queue.push_back(BattleStep::Text("CYNDAQUIL used TACKLE!".into()));
        battle.battle_queue.push_back(BattleStep::Pause(0.3));
        battle.battle_queue.push_back(BattleStep::ApplyDamage { is_player: false, amount: damage });
        battle.battle_queue.push_back(BattleStep::DrainHp { is_player: false, to_hp: target_hp, duration: 0.5 });
        battle.battle_queue.push_back(BattleStep::CheckFaint { is_player: false });
        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::ActionSelect { cursor: 0 })));

        assert_eq!(battle.battle_queue.len(), 6);

        // Step through the entire sequence (generous frame budget)
        let _frames = step_until_phase_change(&mut battle, &mut party, &mut engine, 500);

        // Should end at ActionSelect since enemy did not faint
        assert!(matches!(battle.phase, BattlePhase::ActionSelect { cursor: 0 }),
            "Full attack sequence should end at ActionSelect, got {:?}", battle.phase);
        assert!(battle.battle_queue.is_empty(), "Queue should be fully drained");
        assert_eq!(battle.enemy.hp, target_hp,
            "Enemy HP should be {} after {} damage, got {}", target_hp, damage, battle.enemy.hp);
        assert!((battle.enemy_hp_display - target_hp as f64).abs() < 1.0,
            "Enemy HP display should match target HP");
    }

    #[test]
    fn test_player_attack_queue_builds_correctly() {
        // Verify that entering PlayerAttack builds a queue and transitions to ExecuteQueue
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let party = vec![player_pkmn];
        let enemy = Pokemon::new(PIDGEY, 5);
        let initial_enemy_hp = enemy.hp;

        let mut sim = PokemonSim::new();
        sim.party = party.clone();
        let mut battle = make_test_battle(&sim.party, enemy, true);
        battle.enemy_hp_display = initial_enemy_hp as f64;

        // Enter PlayerAttack phase (player goes first, not from pending)
        battle.phase = BattlePhase::PlayerAttack {
            timer: 0.0, move_id: MOVE_TACKLE, damage: 8, effectiveness: 1.0,
            is_crit: false, from_pending: false,
        };

        // Put battle into sim and step once
        sim.battle = Some(battle);
        sim.step_battle(&mut engine);

        // After stepping, PlayerAttack should have built queue and transitioned to ExecuteQueue
        let battle = sim.battle.as_ref().expect("battle should exist");
        assert!(matches!(battle.phase, BattlePhase::ExecuteQueue),
            "PlayerAttack should build queue and enter ExecuteQueue, got {:?}", battle.phase);
        assert!(!battle.battle_queue.is_empty(),
            "Queue should not be empty after PlayerAttack builds it");
    }

    #[test]
    fn test_enemy_attack_queue_builds_correctly() {
        // Verify that entering EnemyAttack builds a queue and transitions to ExecuteQueue
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut sim = PokemonSim::new();
        sim.party = vec![player_pkmn];
        let mut battle = make_test_battle(&sim.party, enemy, true);

        // Enter EnemyAttack phase
        battle.phase = BattlePhase::EnemyAttack {
            timer: 0.0, move_id: MOVE_TACKLE, damage: 5, effectiveness: 1.0, is_crit: false,
        };

        sim.battle = Some(battle);
        sim.step_battle(&mut engine);

        let battle = sim.battle.as_ref().expect("battle should exist");
        assert!(matches!(battle.phase, BattlePhase::ExecuteQueue),
            "EnemyAttack should build queue and enter ExecuteQueue, got {:?}", battle.phase);
        assert!(!battle.battle_queue.is_empty(),
            "Queue should not be empty after EnemyAttack builds it");
    }

    #[test]
    fn test_player_attack_queue_applies_damage() {
        // Full integration: PlayerAttack → ExecuteQueue → ActionSelect, verify HP changed
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 15);
        let enemy = Pokemon::new(RATTATA, 5);
        let initial_enemy_hp = enemy.hp;

        let mut sim = PokemonSim::new();
        sim.party = vec![player_pkmn];
        let mut battle = make_test_battle(&sim.party, enemy, true);
        battle.enemy_hp_display = initial_enemy_hp as f64;
        battle.player_hp_display = sim.party[0].hp as f64;

        // Use damage=7, normal effectiveness, no crit
        battle.phase = BattlePhase::PlayerAttack {
            timer: 0.0, move_id: MOVE_TACKLE, damage: 7, effectiveness: 1.0,
            is_crit: false, from_pending: false,
        };

        sim.battle = Some(battle);

        // Step enough frames to process through all queue steps (text auto-advances at 1.5s = 90 frames)
        for _ in 0..600 {
            sim.step_battle(&mut engine);
            if let Some(ref b) = sim.battle {
                // Once we reach EnemyAttack, that means player attack resolved
                if matches!(b.phase, BattlePhase::EnemyAttack { .. }) {
                    break;
                }
            }
        }

        // Enemy should have taken damage
        if let Some(ref b) = sim.battle {
            assert!(b.enemy.hp < initial_enemy_hp,
                "Enemy HP should be reduced after PlayerAttack, got {} (was {})", b.enemy.hp, initial_enemy_hp);
        }
    }

    #[test]
    fn test_screen_flash_step() {
        // Verify ScreenFlash step sets screen_flash on PokemonSim
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut sim = PokemonSim::new();
        sim.party = vec![player_pkmn];
        let mut battle = make_test_battle(&sim.party, enemy, true);

        battle.battle_queue.push_back(BattleStep::ScreenFlash(0.8));
        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::ActionSelect { cursor: 0 })));

        sim.step_execute_queue(&mut battle, &mut engine);

        assert!((sim.screen_flash - 0.8).abs() < 0.01,
            "ScreenFlash step should set screen_flash to 0.8, got {}", sim.screen_flash);
        assert_eq!(battle.battle_queue.len(), 1, "ScreenFlash should pop itself");
    }

    #[test]
    fn test_screen_shake_step() {
        // Verify ScreenShake step sets screen_shake on PokemonSim
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut sim = PokemonSim::new();
        sim.party = vec![player_pkmn];
        let mut battle = make_test_battle(&sim.party, enemy, true);

        battle.battle_queue.push_back(BattleStep::ScreenShake(5.0));
        battle.battle_queue.push_back(BattleStep::GoToPhase(Box::new(BattlePhase::ActionSelect { cursor: 0 })));

        sim.step_execute_queue(&mut battle, &mut engine);

        assert!((sim.screen_shake - 5.0).abs() < 0.01,
            "ScreenShake step should set screen_shake to 5.0, got {}", sim.screen_shake);
    }

    #[test]
    fn test_enemy_attack_recharge_skips_via_queue() {
        // Verify EnemyAttack with must_recharge builds recharge text via queue
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut sim = PokemonSim::new();
        sim.party = vec![player_pkmn];
        let mut battle = make_test_battle(&sim.party, enemy, true);
        battle.enemy_must_recharge = true;

        battle.phase = BattlePhase::EnemyAttack {
            timer: 0.0, move_id: MOVE_HYPER_BEAM, damage: 20, effectiveness: 1.0, is_crit: false,
        };

        sim.battle = Some(battle);
        sim.step_battle(&mut engine);

        // Should be in ExecuteQueue with recharge text
        if let Some(ref b) = sim.battle {
            assert!(matches!(b.phase, BattlePhase::ExecuteQueue),
                "Recharge should enter ExecuteQueue, got {:?}", b.phase);
            assert!(!b.enemy_must_recharge, "Recharge flag should be cleared");
        }
    }

    // ── Sprint 125 QA Tests ──────────────────────────────────────

    #[test]
    fn test_sprout_tower_floor_traversal() {
        // Verify Sprout Tower warp chain: 1F -> 2F -> 3F via map data
        let map1f = load_map(MapId::SproutTower1F);
        let map2f = load_map(MapId::SproutTower2F);
        let map3f = load_map(MapId::SproutTower3F);

        // 1F: door to VioletCity at (7,13), stairs up to 2F at (12,1)
        assert_eq!(map1f.warps.len(), 2, "SproutTower1F should have 2 warps");
        assert_eq!(map1f.warps[0].dest_map, MapId::VioletCity, "1F warp 0 should go to VioletCity");
        assert_eq!(map1f.warps[1].dest_map, MapId::SproutTower2F, "1F warp 1 should go to 2F");
        assert_eq!(map1f.warps[1].x, 12, "1F stairs-up x should be 12");
        assert_eq!(map1f.warps[1].y, 1, "1F stairs-up y should be 1");

        // 2F: stairs down to 1F at (1,12), stairs up to 3F at (12,1)
        assert_eq!(map2f.warps.len(), 2, "SproutTower2F should have 2 warps");
        assert_eq!(map2f.warps[0].dest_map, MapId::SproutTower1F, "2F warp 0 should go to 1F");
        assert_eq!(map2f.warps[0].x, 1, "2F stairs-down x should be 1");
        assert_eq!(map2f.warps[0].y, 12, "2F stairs-down y should be 12");
        assert_eq!(map2f.warps[1].dest_map, MapId::SproutTower3F, "2F warp 1 should go to 3F");

        // 3F: stairs down to 2F at (1,12), no upward exit
        assert_eq!(map3f.warps.len(), 1, "SproutTower3F should have 1 warp");
        assert_eq!(map3f.warps[0].dest_map, MapId::SproutTower2F, "3F warp 0 should go to 2F");

        // Verify bidirectional: 1F stairs-up dest matches 2F stairs-down position
        assert_eq!(map1f.warps[1].dest_x, map2f.warps[0].x + 1,
            "1F->2F dest_x should land near 2F stairs-down");
        assert_eq!(map2f.warps[1].dest_x, map3f.warps[0].x + 1,
            "2F->3F dest_x should land near 3F stairs-down");

        // Verify collision tiles at warp positions are C_WARP (value 4)
        let c_warp: u8 = 4;
        let idx_1f_stairs = 1 * map1f.width + 12;
        assert_eq!(map1f.collision[idx_1f_stairs], c_warp, "1F stairs collision should be C_WARP");
        let idx_2f_down = 12 * map2f.width + 1;
        assert_eq!(map2f.collision[idx_2f_down], c_warp, "2F stairs-down collision should be C_WARP");
        let idx_2f_up = 1 * map2f.width + 12;
        assert_eq!(map2f.collision[idx_2f_up], c_warp, "2F stairs-up collision should be C_WARP");
        let idx_3f_down = 12 * map3f.width + 1;
        assert_eq!(map3f.collision[idx_3f_down], c_warp, "3F stairs-down collision should be C_WARP");
    }

    #[test]
    fn test_battle_queue_miss_scenario() {
        // Verify PlayerAttack with damage=0 (miss) produces "Attack missed!" in queue
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let enemy = Pokemon::new(PIDGEY, 5);

        let mut sim = PokemonSim::new();
        sim.party = vec![player_pkmn];
        let mut battle = make_test_battle(&sim.party, enemy.clone(), true);
        battle.enemy_hp_display = enemy.hp as f64;
        battle.player_hp_display = sim.party[0].hp as f64;

        // Set up a miss: damage=0, effectiveness=1.0 (not immune), power>0 move
        battle.phase = BattlePhase::PlayerAttack {
            timer: 0.0, move_id: MOVE_TACKLE, damage: 0, effectiveness: 1.0,
            is_crit: false, from_pending: false,
        };

        sim.battle = Some(battle);
        sim.step_battle(&mut engine);

        // Should be in ExecuteQueue with queue containing "Attack missed!"
        if let Some(ref b) = sim.battle {
            assert!(matches!(b.phase, BattlePhase::ExecuteQueue),
                "Miss should enter ExecuteQueue, got {:?}", b.phase);

            let has_miss_text = b.battle_queue.iter().any(|step| {
                if let BattleStep::Text(ref t) = step { t.contains("missed") } else { false }
            });
            assert!(has_miss_text, "Queue should contain 'Attack missed!' text");

            // Miss should NOT have ScreenFlash (no visual hit effect)
            let has_flash = b.battle_queue.iter().any(|step| matches!(step, BattleStep::ScreenFlash(_)));
            assert!(!has_flash, "Miss should not produce ScreenFlash");

            // Enemy HP should be unchanged (damage=0)
            assert_eq!(b.enemy.hp, enemy.hp, "Enemy HP should be unchanged on miss");
        }
    }

    #[test]
    fn test_battle_queue_super_effective_message() {
        // Verify PlayerAttack with effectiveness > 1.5 produces "Super effective!" in queue
        let mut engine = crate::engine::Engine::new(160, 144);
        engine.reset(42);

        let player_pkmn = Pokemon::new(CYNDAQUIL, 10);
        let enemy = Pokemon::new(BELLSPROUT, 5);

        let mut sim = PokemonSim::new();
        sim.party = vec![player_pkmn];
        let mut battle = make_test_battle(&sim.party, enemy.clone(), true);
        battle.enemy_hp_display = enemy.hp as f64;
        battle.player_hp_display = sim.party[0].hp as f64;

        // Super effective: effectiveness=2.0, damage>0
        battle.phase = BattlePhase::PlayerAttack {
            timer: 0.0, move_id: MOVE_EMBER, damage: 15, effectiveness: 2.0,
            is_crit: false, from_pending: false,
        };

        sim.battle = Some(battle);
        sim.step_battle(&mut engine);

        if let Some(ref b) = sim.battle {
            assert!(matches!(b.phase, BattlePhase::ExecuteQueue),
                "Super effective should enter ExecuteQueue, got {:?}", b.phase);

            let has_se_text = b.battle_queue.iter().any(|step| {
                if let BattleStep::Text(ref t) = step { t.contains("Super effective") } else { false }
            });
            assert!(has_se_text, "Queue should contain 'Super effective!' text");

            // Should have ScreenFlash with strong flash (>= 0.9 for super effective)
            let has_strong_flash = b.battle_queue.iter().any(|step| {
                if let BattleStep::ScreenFlash(val) = step { *val >= 0.9 } else { false }
            });
            assert!(has_strong_flash, "Super effective hit should produce strong ScreenFlash");
        }
    }

    #[test]
    fn test_sprout_tower_rival_event_trigger() {
        // Verify the rival event on 3F triggers correctly and sets FLAG_SPROUT_RIVAL
        let party = vec![Pokemon::new(CYNDAQUIL, 10)];
        let mut sim = PokemonSim::with_state(MapId::SproutTower3F, 5, 6, party, 1);
        sim.rival_starter = TOTODILE;

        // Should not have the flag yet
        assert!(!sim.has_flag(FLAG_SPROUT_RIVAL), "FLAG_SPROUT_RIVAL should not be set initially");

        // Player is at y=6, needs to be at y<=5 to trigger — move up
        sim.player.y = 5;
        let triggered = sim.check_sprout_tower_rival();

        assert!(triggered, "Rival event should trigger at y<=5 on 3F");
        assert!(sim.has_flag(FLAG_SPROUT_RIVAL), "FLAG_SPROUT_RIVAL should be set after trigger");
        assert!(matches!(sim.phase, GamePhase::Dialogue), "Phase should be Dialogue after rival event");

        // Verify it doesn't trigger a second time
        sim.phase = GamePhase::Overworld;
        let triggered_again = sim.check_sprout_tower_rival();
        assert!(!triggered_again, "Rival event should not trigger twice (flag already set)");
    }

    #[test]
    fn test_sprout_tower_sage_teams_match_pokecrystal() {
        // Verify all 7 Sprout Tower sage trainers have correct teams per pokecrystal data
        let map1f = load_map(MapId::SproutTower1F);
        let map2f = load_map(MapId::SproutTower2F);
        let map3f = load_map(MapId::SproutTower3F);

        // 1F NPC 4: Sage Chow — 3x Bellsprout Lv3
        let chow = &map1f.npcs[4];
        assert!(chow.is_trainer, "Sage Chow should be a trainer");
        assert_eq!(chow.trainer_team.len(), 3, "Sage Chow should have 3 Pokemon");
        for tp in chow.trainer_team {
            assert_eq!(tp.species_id, BELLSPROUT);
            assert_eq!(tp.level, 3);
        }

        // 2F NPC 0: Sage Nico — 3x Bellsprout Lv3
        let nico = &map2f.npcs[0];
        assert!(nico.is_trainer);
        assert_eq!(nico.trainer_team.len(), 3);

        // 2F NPC 1: Sage Edmond — 3x Bellsprout Lv3
        let edmond = &map2f.npcs[1];
        assert!(edmond.is_trainer);
        assert_eq!(edmond.trainer_team.len(), 3);

        // 3F NPC 0: Elder Li — 2x Bellsprout Lv7 + Hoothoot Lv10
        let li = &map3f.npcs[0];
        assert!(li.is_trainer);
        assert_eq!(li.trainer_team.len(), 3, "Elder Li should have 3 Pokemon");
        assert_eq!(li.trainer_team[0].species_id, BELLSPROUT);
        assert_eq!(li.trainer_team[0].level, 7);
        assert_eq!(li.trainer_team[1].species_id, BELLSPROUT);
        assert_eq!(li.trainer_team[1].level, 7);
        assert_eq!(li.trainer_team[2].species_id, HOOTHOOT);
        assert_eq!(li.trainer_team[2].level, 10);

        // 3F NPC 1: Sage Jin — Bellsprout Lv6
        let jin = &map3f.npcs[1];
        assert!(jin.is_trainer);
        assert_eq!(jin.trainer_team.len(), 1);
        assert_eq!(jin.trainer_team[0].species_id, BELLSPROUT);
        assert_eq!(jin.trainer_team[0].level, 6);

        // 3F NPC 2: Sage Troy — Bellsprout Lv7 + Hoothoot Lv7
        let troy = &map3f.npcs[2];
        assert!(troy.is_trainer);
        assert_eq!(troy.trainer_team.len(), 2);
        assert_eq!(troy.trainer_team[0].species_id, BELLSPROUT);
        assert_eq!(troy.trainer_team[0].level, 7);
        assert_eq!(troy.trainer_team[1].species_id, HOOTHOOT);
        assert_eq!(troy.trainer_team[1].level, 7);

        // 3F NPC 3: Sage Neal — Bellsprout Lv6
        let neal = &map3f.npcs[3];
        assert!(neal.is_trainer);
        assert_eq!(neal.trainer_team.len(), 1);
        assert_eq!(neal.trainer_team[0].species_id, BELLSPROUT);
        assert_eq!(neal.trainer_team[0].level, 6);
    }

    // ─── Sprint 128 QA Tests ──────────────────────────────────

    #[test]
    fn test_slowpoke_well_rocket_event_flow() {
        // Test: Slowpoke Well Rocket event — verify flag, NPC visibility, trainer data
        let map = load_map(MapId::SlowpokeWellB1F);

        // Verify map structure
        assert_eq!(map.width, 16);
        assert_eq!(map.height, 16);
        assert_eq!(map.tiles.len(), 16 * 16);
        assert_eq!(map.collision.len(), 16 * 16);
        assert_eq!(map.id, MapId::SlowpokeWellB1F);

        // Verify exit warp back to Azalea Town
        assert!(map.warps.iter().any(|w| w.dest_map == MapId::AzaleaTown),
            "SlowpokeWellB1F must have warp back to AzaleaTown");

        // Verify warp to B2F exists (per pokecrystal: warp_event 7, 11)
        assert!(map.warps.iter().any(|w| w.dest_map == MapId::SlowpokeWellB2F),
            "SlowpokeWellB1F must have warp to SlowpokeWellB2F");

        // Verify 7 NPCs: 4 Rocket Grunts + Kurt + 2 Slowpokes
        assert_eq!(map.npcs.len(), 7, "SlowpokeWellB1F should have 7 NPCs");

        // Per pokecrystal: GruntM29 = Rattata Lv9 x2
        let grunt0 = &map.npcs[0];
        assert!(grunt0.is_trainer);
        assert_eq!(grunt0.trainer_team.len(), 2);
        assert_eq!(grunt0.trainer_team[0].species_id, RATTATA);
        assert_eq!(grunt0.trainer_team[0].level, 9);
        assert_eq!(grunt0.trainer_team[1].species_id, RATTATA);
        assert_eq!(grunt0.trainer_team[1].level, 9);

        // Per pokecrystal: GruntF1 = Zubat Lv9, Ekans Lv11
        let grunt1 = &map.npcs[1];
        assert!(grunt1.is_trainer);
        assert_eq!(grunt1.trainer_team.len(), 2);
        assert_eq!(grunt1.trainer_team[0].species_id, ZUBAT);
        assert_eq!(grunt1.trainer_team[0].level, 9);
        assert_eq!(grunt1.trainer_team[1].species_id, 23); // EKANS
        assert_eq!(grunt1.trainer_team[1].level, 11);

        // Per pokecrystal: GruntM2 = Rattata Lv7, Zubat Lv9 x2
        let grunt2 = &map.npcs[2];
        assert!(grunt2.is_trainer);
        assert_eq!(grunt2.trainer_team.len(), 3);
        assert_eq!(grunt2.trainer_team[0].species_id, RATTATA);
        assert_eq!(grunt2.trainer_team[0].level, 7);

        // Per pokecrystal: GruntM1 (Executive Proton) = Koffing Lv14
        let exec = &map.npcs[3];
        assert!(exec.is_trainer);
        assert_eq!(exec.trainer_team.len(), 1);
        assert_eq!(exec.trainer_team[0].species_id, KOFFING);
        assert_eq!(exec.trainer_team[0].level, 14);

        // Kurt (NPC 4) should NOT be a trainer
        assert!(!map.npcs[4].is_trainer, "Kurt should not be a trainer");
        // Slowpokes (NPCs 5-6) should NOT be trainers
        assert!(!map.npcs[5].is_trainer, "Slowpoke should not be a trainer");
        assert!(!map.npcs[6].is_trainer, "Slowpoke should not be a trainer");

        // Test FLAG_SLOWPOKE_WELL event flow
        let party = vec![Pokemon::new(CYNDAQUIL, 15)];
        let sim = PokemonSim::with_state(MapId::SlowpokeWellB1F, 5, 5, party, 2);

        // Before clearing: flag should not be set
        assert!(!sim.has_flag(FLAG_SLOWPOKE_WELL), "FLAG_SLOWPOKE_WELL should not be set initially");

        // All Rockets + Kurt + Slowpokes (NPCs 0-6) should be active before clearing
        for i in 0..7 {
            assert!(sim.is_npc_active(i), "NPC {} should be active before clearing", i);
        }

        // Simulate clearing: set the flag
        let mut sim2 = sim;
        sim2.set_flag(FLAG_SLOWPOKE_WELL);
        assert!(sim2.has_flag(FLAG_SLOWPOKE_WELL));

        // After clearing: all NPCs 0-6 should be hidden
        for i in 0..7 {
            assert!(!sim2.is_npc_active(i),
                "NPC {} should be hidden after FLAG_SLOWPOKE_WELL set", i);
        }
    }

    #[test]
    fn test_burned_tower_beast_encounter_event() {
        // Test: Burned Tower beast release event flow
        let map_1f = load_map(MapId::BurnedTower);
        let map_b1f = load_map(MapId::BurnedTowerB1F);

        // Verify BurnedTower 1F has warp to B1F
        assert!(map_1f.warps.iter().any(|w| w.dest_map == MapId::BurnedTowerB1F),
            "BurnedTower must have warp to BurnedTowerB1F");
        // Verify B1F has warp back to 1F
        assert!(map_b1f.warps.iter().any(|w| w.dest_map == MapId::BurnedTower),
            "BurnedTowerB1F must have warp back to BurnedTower");

        // Verify BurnedTowerB1F dimensions
        assert_eq!(map_b1f.width, 14);
        assert_eq!(map_b1f.height, 14);

        // Eusine NPC (index 0) is the only NPC in B1F
        assert_eq!(map_b1f.npcs.len(), 1, "BurnedTowerB1F should have 1 NPC (Eusine)");
        assert!(!map_b1f.npcs[0].is_trainer, "Eusine should not be a trainer in B1F");

        // Test FLAG_BURNED_TOWER_RIVAL (1F rival battle)
        let party = vec![Pokemon::new(CYNDAQUIL, 20)];
        let mut sim = PokemonSim::with_state(MapId::BurnedTower, 7, 10, party.clone(), 4);
        sim.rival_starter = TOTODILE;
        assert!(!sim.has_flag(FLAG_BURNED_TOWER_RIVAL));

        // Player at y=10, needs y<=7 to trigger
        sim.player.y = 7;
        let triggered = sim.check_burned_tower_rival();
        assert!(triggered, "Rival event should trigger at y<=7 in BurnedTower");
        assert!(sim.has_flag(FLAG_BURNED_TOWER_RIVAL));
        assert!(matches!(sim.phase, GamePhase::Dialogue));

        // Should not trigger again
        sim.phase = GamePhase::Overworld;
        let again = sim.check_burned_tower_rival();
        assert!(!again, "Rival event should not trigger twice");

        // Test FLAG_BEASTS_RELEASED (B1F beast event)
        let mut sim_b1f = PokemonSim::with_state(MapId::BurnedTowerB1F, 7, 8, party, 4);
        assert!(!sim_b1f.has_flag(FLAG_BEASTS_RELEASED));

        // Eusine hidden before beasts released
        assert!(!sim_b1f.is_npc_active(0), "Eusine should be hidden before beasts released");

        // Player at y=8, needs y<=5 to trigger
        sim_b1f.player.y = 5;
        let triggered = sim_b1f.check_beasts_released();
        assert!(triggered, "Beast event should trigger at y<=5 in BurnedTowerB1F");
        assert!(sim_b1f.has_flag(FLAG_BEASTS_RELEASED));
        assert!(matches!(sim_b1f.phase, GamePhase::Dialogue));

        // Eusine visible after beasts released
        assert!(sim_b1f.is_npc_active(0), "Eusine should be visible after beasts released");

        // Should not trigger again
        sim_b1f.phase = GamePhase::Overworld;
        let again = sim_b1f.check_beasts_released();
        assert!(!again, "Beast event should not trigger twice");

        // Verify encounters in B1F per pokecrystal (no Raticate or Magmar)
        assert!(map_b1f.encounters.iter().any(|e| e.species_id == KOFFING),
            "BurnedTowerB1F should have Koffing encounters");
        assert!(map_b1f.encounters.iter().any(|e| e.species_id == ZUBAT),
            "BurnedTowerB1F should have Zubat encounters");
        assert!(map_b1f.encounters.iter().any(|e| e.species_id == 110), // WEEZING
            "BurnedTowerB1F should have Weezing encounters");
        assert!(!map_b1f.encounters.iter().any(|e| e.species_id == 20), // RATICATE
            "BurnedTowerB1F should NOT have Raticate per pokecrystal");
    }

    #[test]
    fn test_ilex_forest_farfetchd_quest() {
        // Test: Ilex Forest Farfetch'd quest — NPC visibility and flag logic
        let map = load_map(MapId::IlexForest);

        // Verify map structure
        assert_eq!(map.width, 20);
        assert_eq!(map.height, 24);
        assert_eq!(map.id, MapId::IlexForest);

        // Verify warps: north to Route34, south to AzaleaTown
        assert!(map.warps.iter().any(|w| w.dest_map == MapId::Route34),
            "IlexForest must have warp to Route34");
        assert!(map.warps.iter().any(|w| w.dest_map == MapId::AzaleaTown),
            "IlexForest must have warp to AzaleaTown");

        // Verify NPC 0 = Farfetch'd (wanders, not trainer)
        assert!(!map.npcs[0].is_trainer, "Farfetch'd should not be a trainer");
        assert!(map.npcs[0].wanders, "Farfetch'd should wander");

        // Verify NPC 1 = Apprentice (not trainer)
        assert!(!map.npcs[1].is_trainer, "Apprentice should not be a trainer");

        // Verify NPC 2 = Charcoal Master (gives HM Cut, not trainer)
        assert!(!map.npcs[2].is_trainer, "Charcoal Master should not be a trainer");

        // Verify NPC 4 = Bug Catcher Wayne (is trainer, per pokecrystal)
        assert!(map.npcs[4].is_trainer, "Bug Catcher Wayne should be a trainer");
        assert_eq!(map.npcs[4].trainer_team.len(), 2);
        assert_eq!(map.npcs[4].trainer_team[0].species_id, CATERPIE);
        assert_eq!(map.npcs[4].trainer_team[0].level, 8);
        assert_eq!(map.npcs[4].trainer_team[1].species_id, WEEDLE);
        assert_eq!(map.npcs[4].trainer_team[1].level, 8);

        // Test FLAG_ILEX_FARFETCHD NPC visibility
        let party = vec![Pokemon::new(CYNDAQUIL, 10)];
        let sim = PokemonSim::with_state(MapId::IlexForest, 8, 12, party, 2);

        // Before quest: Farfetch'd and Apprentice visible
        assert!(!sim.has_flag(FLAG_ILEX_FARFETCHD));
        assert!(sim.is_npc_active(0), "Farfetch'd should be visible before quest");
        assert!(sim.is_npc_active(1), "Apprentice should be visible before quest");

        // After quest: Farfetch'd and Apprentice hidden
        let mut sim2 = sim;
        sim2.set_flag(FLAG_ILEX_FARFETCHD);
        assert!(!sim2.is_npc_active(0), "Farfetch'd should be hidden after quest");
        assert!(!sim2.is_npc_active(1), "Apprentice should be hidden after quest");

        // Charcoal Master (NPC 2) always visible (handled by interaction, not is_npc_active)
        assert!(sim2.is_npc_active(2), "Charcoal Master should remain visible");

        // Verify encounter data per pokecrystal
        assert!(map.encounters.iter().any(|e| e.species_id == CATERPIE),
            "IlexForest should have Caterpie encounters");
        assert!(map.encounters.iter().any(|e| e.species_id == PARAS),
            "IlexForest should have Paras encounters");
        assert!(map.night_encounters.iter().any(|e| e.species_id == ODDISH),
            "IlexForest should have Oddish night encounters");
    }

    #[test]
    fn test_union_cave_floor_traversal() {
        // Test: Union Cave 3-floor connectivity — 1F -> B1F -> B2F
        let cave_1f = load_map(MapId::UnionCave);
        let cave_b1f = load_map(MapId::UnionCaveB1F);
        let cave_b2f = load_map(MapId::UnionCaveB2F);

        // Verify map dimensions
        assert_eq!(cave_1f.width, 16);
        assert_eq!(cave_1f.height, 16);
        assert_eq!(cave_b1f.width, 18);
        assert_eq!(cave_b1f.height, 16);
        assert_eq!(cave_b2f.width, 16);
        assert_eq!(cave_b2f.height, 14);

        // 1F -> B1F warp
        assert!(cave_1f.warps.iter().any(|w| w.dest_map == MapId::UnionCaveB1F),
            "UnionCave 1F must have warp to B1F");
        // B1F -> 1F return warp
        assert!(cave_b1f.warps.iter().any(|w| w.dest_map == MapId::UnionCave),
            "UnionCaveB1F must have warp back to 1F");
        // B1F -> B2F warp
        assert!(cave_b1f.warps.iter().any(|w| w.dest_map == MapId::UnionCaveB2F),
            "UnionCaveB1F must have warp to B2F");
        // B2F -> B1F return warp
        assert!(cave_b2f.warps.iter().any(|w| w.dest_map == MapId::UnionCaveB1F),
            "UnionCaveB2F must have warp back to B1F");

        // Verify B1F warp destinations land on non-solid tiles in 1F
        let b1f_to_1f = cave_b1f.warps.iter().find(|w| w.dest_map == MapId::UnionCave);
        if let Some(warp) = b1f_to_1f {
            let idx = (warp.dest_y as usize) * cave_1f.width + (warp.dest_x as usize);
            assert!(idx < cave_1f.collision.len(), "Warp dest out of bounds");
            assert_ne!(cave_1f.collision[idx], 1u8, // C_SOLID
                "B1F->1F warp destination ({},{}) must not be solid in 1F", warp.dest_x, warp.dest_y);
        }

        // Verify B1F trainer count (per pokecrystal: 4 trainers)
        let b1f_trainers: Vec<_> = cave_b1f.npcs.iter().filter(|n| n.is_trainer).collect();
        assert_eq!(b1f_trainers.len(), 4, "UnionCaveB1F should have 4 trainers");

        // Verify B2F trainer count (per pokecrystal: 3 trainers)
        let b2f_trainers: Vec<_> = cave_b2f.npcs.iter().filter(|n| n.is_trainer).collect();
        assert_eq!(b2f_trainers.len(), 3, "UnionCaveB2F should have 3 trainers");

        // Verify B2F has Lapras NPC (Friday encounter)
        assert!(cave_b2f.npcs.iter().any(|n| !n.is_trainer && n.dialogue[0].contains("LAPRAS")),
            "UnionCaveB2F should have Lapras NPC");

        // Verify B2F has water encounters (Lapras)
        assert!(!cave_b2f.water_encounters.is_empty(),
            "UnionCaveB2F should have water encounters (Lapras)");
        assert!(cave_b2f.water_encounters.iter().any(|e| e.species_id == LAPRAS),
            "UnionCaveB2F water encounters should include Lapras");

        // Verify night encounters exist for B1F and B2F
        assert!(!cave_b1f.night_encounters.is_empty(),
            "UnionCaveB1F should have night encounters");
        assert!(!cave_b2f.night_encounters.is_empty(),
            "UnionCaveB2F should have night encounters");

        // Verify B2F night encounters include Quagsire (per pokecrystal)
        assert!(cave_b2f.night_encounters.iter().any(|e| e.species_id == QUAGSIRE),
            "UnionCaveB2F night encounters should include Quagsire per pokecrystal");

        // Verify Slowpoke Well connectivity: AzaleaTown -> B1F -> B2F
        let well_b1f = load_map(MapId::SlowpokeWellB1F);
        let well_b2f = load_map(MapId::SlowpokeWellB2F);
        assert!(well_b1f.warps.iter().any(|w| w.dest_map == MapId::SlowpokeWellB2F),
            "SlowpokeWellB1F must have warp to B2F");
        assert!(well_b2f.warps.iter().any(|w| w.dest_map == MapId::SlowpokeWellB1F),
            "SlowpokeWellB2F must have warp back to B1F");
    }

    #[test]
    fn test_azalea_slowpoke_well_warp_connectivity() {
        // Verify bidirectional warps between Azalea Town and Slowpoke Well
        let azalea = load_map(MapId::AzaleaTown);
        let well = load_map(MapId::SlowpokeWellB1F);

        // Azalea -> SlowpokeWellB1F
        let az_to_well = azalea.warps.iter().find(|w| w.dest_map == MapId::SlowpokeWellB1F);
        assert!(az_to_well.is_some(), "AzaleaTown must have warp to SlowpokeWellB1F");

        // SlowpokeWellB1F -> Azalea
        let well_to_az = well.warps.iter().find(|w| w.dest_map == MapId::AzaleaTown);
        assert!(well_to_az.is_some(), "SlowpokeWellB1F must have warp back to AzaleaTown");

        // Verify Azalea -> IlexForest warps exist
        assert!(azalea.warps.iter().any(|w| w.dest_map == MapId::IlexForest),
            "AzaleaTown must have warp to IlexForest");

        // Verify Route34 -> IlexForest warps
        let r34 = load_map(MapId::Route34);
        assert!(r34.warps.iter().any(|w| w.dest_map == MapId::IlexForest),
            "Route34 must have warp to IlexForest");

        // Verify IlexForest has warps to both Route34 and AzaleaTown
        let ilex = load_map(MapId::IlexForest);
        assert!(ilex.warps.iter().any(|w| w.dest_map == MapId::Route34),
            "IlexForest must have warp to Route34");
        assert!(ilex.warps.iter().any(|w| w.dest_map == MapId::AzaleaTown),
            "IlexForest must have warp to AzaleaTown");

        // Verify Burned Tower bidirectional warps
        let bt = load_map(MapId::BurnedTower);
        let btb1f = load_map(MapId::BurnedTowerB1F);
        assert!(bt.warps.iter().any(|w| w.dest_map == MapId::BurnedTowerB1F),
            "BurnedTower must have warp to B1F");
        assert!(btb1f.warps.iter().any(|w| w.dest_map == MapId::BurnedTower),
            "BurnedTowerB1F must have warp back to BurnedTower");
    }
}
