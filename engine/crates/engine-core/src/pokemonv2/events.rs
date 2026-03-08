// AI-INSTRUCTIONS: pokemonv2/events.rs — Event system. Imports data.rs and maps.rs (MapId only).
// Sprint 2: ScriptResult enum, loaded_wild_species on ScriptState, new ScriptStep variants,
//           15+ new event flags, ~20 new scripts, SceneState expanded to 32 entries.
// Import graph: events.rs <- data.rs, maps.rs(MapId only)

use super::data::{
    BattleType, Direction, Emote, NpcState, PlayerState, Pokemon, SpeciesId,
    ITEM_BERRY, ITEM_MYSTIC_WATER, ITEM_POTION,
    CYNDAQUIL, TOTODILE, CHIKORITA,
    MUSIC_SHOW_ME_AROUND, MUSIC_RIVAL_ENCOUNTER, MUSIC_RIVAL_AFTER,
    HOPPIP,
};
use super::maps::MapId;

// ── Event Flag Constants ──────────────────────────────────────────────────────

pub const EVENT_ENGINE_POKEGEAR: u16 = 0;
pub const EVENT_ENGINE_PHONE_CARD: u16 = 1;
pub const EVENT_PHONE_MOM: u16 = 2;
pub const EVENT_PLAYERS_HOUSE_MOM_1: u16 = 3;
pub const EVENT_PLAYERS_HOUSE_MOM_2: u16 = 4;
pub const EVENT_GOT_A_POKEMON_FROM_ELM: u16 = 5;
pub const EVENT_GOT_CYNDAQUIL_FROM_ELM: u16 = 6;
pub const EVENT_GOT_TOTODILE_FROM_ELM: u16 = 7;
pub const EVENT_GOT_CHIKORITA_FROM_ELM: u16 = 8;
pub const EVENT_RIVAL_NEW_BARK_TOWN: u16 = 9;
pub const EVENT_ENGINE_FLYPOINT_NEW_BARK: u16 = 10;
pub const EVENT_COP_IN_ELMS_LAB: u16 = 11;
pub const EVENT_ELMS_AIDE_IN_LAB: u16 = 12;
pub const EVENT_CYNDAQUIL_POKEBALL_IN_ELMS_LAB: u16 = 13;
pub const EVENT_TOTODILE_POKEBALL_IN_ELMS_LAB: u16 = 14;
pub const EVENT_CHIKORITA_POKEBALL_IN_ELMS_LAB: u16 = 15;
pub const EVENT_PLAYERS_HOUSE_1F_NEIGHBOR: u16 = 16;
// Sprint 2 additions
pub const EVENT_GUIDE_GENT_VISIBLE_IN_CHERRYGROVE: u16 = 17;
pub const EVENT_GUIDE_GENT_IN_HIS_HOUSE: u16 = 18;
pub const EVENT_RIVAL_CHERRYGROVE_CITY: u16 = 19;
pub const EVENT_GOT_MYSTIC_WATER_IN_CHERRYGROVE: u16 = 20;
pub const EVENT_ENGINE_MAP_CARD: u16 = 21;
pub const EVENT_ENGINE_FLYPOINT_CHERRYGROVE: u16 = 22;
pub const EVENT_DUDE_TALKED_TO_YOU: u16 = 23;
pub const EVENT_LEARNED_TO_CATCH_POKEMON: u16 = 24;
pub const EVENT_ROUTE_29_TUSCANY_OF_TUESDAY: u16 = 25;
pub const EVENT_ROUTE_29_POTION: u16 = 26;
pub const EVENT_MET_TUSCANY_OF_TUESDAY: u16 = 27;
pub const EVENT_GOT_PINK_BOW_FROM_TUSCANY: u16 = 28;
pub const EVENT_GAVE_MYSTERY_EGG_TO_ELM: u16 = 29;
pub const EVENT_ENGINE_POKEDEX: u16 = 30;
pub const EVENT_ENGINE_ZEPHYRBADGE: u16 = 31;

// ── Scene Constants ───────────────────────────────────────────────────────────

pub const SCENE_ELMSLAB_MEET_ELM: u8 = 0;
pub const SCENE_ELMSLAB_CANT_LEAVE: u8 = 1;
pub const SCENE_ELMSLAB_NOOP: u8 = 2;
pub const SCENE_ELMSLAB_MEET_OFFICER: u8 = 3;
pub const SCENE_ELMSLAB_UNUSED: u8 = 4;
pub const SCENE_ELMSLAB_AIDE_GIVES_POTION: u8 = 5;
pub const SCENE_ELMSLAB_AIDE_GIVES_POKE_BALLS: u8 = 6;

pub const SCENE_NEWBARKTOWN_TEACHER_STOPS_YOU: u8 = 0;
pub const SCENE_NEWBARKTOWN_NOOP: u8 = 1;

pub const SCENE_PLAYERSHOUSE1F_MEET_MOM: u8 = 0;
pub const SCENE_PLAYERSHOUSE1F_NOOP: u8 = 1;

// Sprint 2 scenes
pub const SCENE_ROUTE29_NOOP: u8 = 0;
pub const SCENE_ROUTE29_CATCH_TUTORIAL: u8 = 1;

pub const SCENE_CHERRYGROVECITY_NOOP: u8 = 0;
pub const SCENE_CHERRYGROVECITY_MEET_RIVAL: u8 = 1;

// ── Script ID Constants ───────────────────────────────────────────────────────

// Sprint 1 scripts
pub const SCRIPT_MEET_MOM: u16 = 1;
pub const SCRIPT_TEACHER_STOPS_1: u16 = 2;
pub const SCRIPT_TEACHER_STOPS_2: u16 = 3;
pub const SCRIPT_RIVAL_INTERACTION: u16 = 4;
pub const SCRIPT_ELM_INTRO: u16 = 5;
pub const SCRIPT_STARTER_CYNDAQUIL: u16 = 6;
pub const SCRIPT_STARTER_TOTODILE: u16 = 7;
pub const SCRIPT_STARTER_CHIKORITA: u16 = 8;
pub const SCRIPT_LAB_TRY_TO_LEAVE: u16 = 9;
pub const SCRIPT_NBT_SIGN: u16 = 10;
pub const SCRIPT_PLAYER_HOUSE_SIGN: u16 = 11;
pub const SCRIPT_ELM_LAB_SIGN: u16 = 12;
pub const SCRIPT_ELM_HOUSE_SIGN: u16 = 13;
pub const SCRIPT_HOUSE1F_STOVE: u16 = 14;
pub const SCRIPT_HOUSE1F_SINK: u16 = 15;
pub const SCRIPT_HOUSE1F_FRIDGE: u16 = 16;
pub const SCRIPT_HOUSE1F_TV: u16 = 17;
pub const SCRIPT_HOUSE2F_PC: u16 = 18;
pub const SCRIPT_HOUSE2F_RADIO: u16 = 19;
pub const SCRIPT_HOUSE2F_BOOKSHELF: u16 = 20;
pub const SCRIPT_LAB_HEALING_MACHINE: u16 = 21;
pub const SCRIPT_LAB_BOOKSHELF: u16 = 22;
pub const SCRIPT_LAB_TRASHCAN: u16 = 23;
pub const SCRIPT_LAB_WINDOW: u16 = 24;
pub const SCRIPT_LAB_PC: u16 = 25;
pub const SCRIPT_MEET_OFFICER: u16 = 100;
pub const SCRIPT_AIDE_GIVES_POTION: u16 = 101;
pub const SCRIPT_AIDE_GIVES_BALLS: u16 = 102;

// Sprint 2: Route 29 scripts
pub const SCRIPT_ROUTE29_SIGN1: u16 = 200;
pub const SCRIPT_ROUTE29_SIGN2: u16 = 201;
pub const SCRIPT_CATCHING_TUTORIAL_DUDE: u16 = 202;
pub const SCRIPT_CATCHING_TUTORIAL_1: u16 = 203;
pub const SCRIPT_CATCHING_TUTORIAL_2: u16 = 204;
pub const SCRIPT_ROUTE29_YOUNGSTER: u16 = 205;
pub const SCRIPT_ROUTE29_TEACHER: u16 = 206;
pub const SCRIPT_ROUTE29_FISHER: u16 = 207;
pub const SCRIPT_ROUTE29_COOLTRAINER_M: u16 = 208;
pub const SCRIPT_ROUTE29_FRUIT_TREE: u16 = 209;
pub const SCRIPT_ROUTE29_POTION: u16 = 210;
pub const SCRIPT_TUSCANY: u16 = 211;

// Sprint 2: Route29Route46Gate scripts
pub const SCRIPT_GATE_OFFICER: u16 = 220;
pub const SCRIPT_GATE_YOUNGSTER: u16 = 221;

// Sprint 2: CherrygroveCity scripts
pub const SCRIPT_GUIDE_GENT_CITY: u16 = 230;
pub const SCRIPT_CHERRYGROVE_RIVAL: u16 = 231;
pub const SCRIPT_CHERRYGROVE_TEACHER: u16 = 232;
pub const SCRIPT_CHERRYGROVE_YOUNGSTER: u16 = 233;
pub const SCRIPT_MYSTIC_WATER_GUY: u16 = 234;
pub const SCRIPT_CHERRYGROVE_SIGN: u16 = 240;
pub const SCRIPT_GUIDE_GENT_HOUSE_SIGN: u16 = 241;
pub const SCRIPT_CHERRYGROVE_MART_SIGN: u16 = 242;
pub const SCRIPT_CHERRYGROVE_POKECENTER_SIGN: u16 = 243;

// Sprint 2: CherrygrovePokecenter scripts
pub const SCRIPT_CHERRYGROVE_NURSE: u16 = 250;
pub const SCRIPT_POKECENTER_FISHER: u16 = 251;
pub const SCRIPT_POKECENTER_GENTLEMAN: u16 = 252;
pub const SCRIPT_POKECENTER_TEACHER: u16 = 253;

// Sprint 2: CherrygroveMart scripts
pub const SCRIPT_CHERRYGROVE_CLERK: u16 = 260;
pub const SCRIPT_MART_COOLTRAINER: u16 = 261;
pub const SCRIPT_MART_YOUNGSTER: u16 = 262;

// Sprint 2: GuideGentsHouse scripts
pub const SCRIPT_GUIDE_GENT_HOME: u16 = 270;
pub const SCRIPT_GUIDE_GENT_BOOKSHELF: u16 = 271;

// Sprint 2: GymSpeechHouse scripts
pub const SCRIPT_GYM_SPEECH_POKEFAN: u16 = 280;
pub const SCRIPT_GYM_SPEECH_BUG_CATCHER: u16 = 281;
pub const SCRIPT_GYM_SPEECH_BOOKSHELF: u16 = 282;

// Sprint 2: EvoSpeechHouse scripts
pub const SCRIPT_EVO_SPEECH_LASS: u16 = 290;
pub const SCRIPT_EVO_SPEECH_YOUNGSTER: u16 = 291;
pub const SCRIPT_EVO_SPEECH_BOOKSHELF: u16 = 292;

// ── EventFlags ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct EventFlags {
    flags: [u64; 32],
}

impl EventFlags {
    pub fn new() -> Self {
        Self { flags: [0u64; 32] }
    }

    pub fn has(&self, id: u16) -> bool {
        let word = id as usize / 64;
        let bit  = id as usize % 64;
        word < self.flags.len() && self.flags[word] & (1u64 << bit) != 0
    }

    pub fn set(&mut self, id: u16) {
        let word = id as usize / 64;
        let bit  = id as usize % 64;
        if word < self.flags.len() {
            self.flags[word] |= 1u64 << bit;
        }
    }

    pub fn clear(&mut self, id: u16) {
        let word = id as usize / 64;
        let bit  = id as usize % 64;
        if word < self.flags.len() {
            self.flags[word] &= !(1u64 << bit);
        }
    }
}

// ── SceneState ────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct SceneState {
    scenes: Vec<u8>,
}

impl SceneState {
    pub fn new() -> Self {
        Self { scenes: vec![0u8; 32] }  // Review #9: expanded from 16 to 32
    }

    pub fn get(&self, map: MapId) -> u8 {
        let idx = map as usize;
        if idx < self.scenes.len() { self.scenes[idx] } else { 0 }
    }

    pub fn set(&mut self, map: MapId, scene: u8) {
        let idx = map as usize;
        if idx >= self.scenes.len() {
            self.scenes.resize(idx + 1, 0);
        }
        self.scenes[idx] = scene;
    }
}

// ── ScriptResult ──────────────────────────────────────────────────────────────

/// Result of a script step execution. Replaces the old bool return type.
#[derive(Clone, Debug)]
pub enum ScriptResult {
    /// Script is still executing.
    Running,
    /// Script hit End step.
    Ended,
    /// Script wants to start a battle. mod.rs must create BattleState and switch phase.
    StartBattle {
        battle_type: BattleType,
        species: Option<(SpeciesId, u8)>,
    },
}

// ── ScriptStep Enum ───────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum SpecialFn {
    HealParty,
    RestartMapMusic,
    FadeOutMusic,
}

#[derive(Clone, Debug)]
pub enum ScriptStep {
    // Text
    ShowText(String),
    WaitButton,
    CloseText,

    // Movement
    MoveNpc { npc_idx: u8, steps: Vec<(Direction, u8)> },
    MovePlayer { steps: Vec<(Direction, u8)> },
    TurnNpc { npc_idx: u8, direction: Direction },
    TurnPlayer(Direction),

    // Emotes & effects
    ShowEmote { npc_idx: u8, emote: Emote, frames: u8 },
    PlaySound(u8),
    PlayMusic(u8),
    Pause(f64),

    // Game state
    SetEvent(u16),
    ClearEvent(u16),
    SetScene { map: MapId, scene_id: u8 },
    GiveItem { item_id: u8, count: u8 },
    GivePokemon { species: SpeciesId, level: u8, held_item: u8 },
    HideNpc(u8),
    ShowNpc(u8),

    // Control flow
    CheckEvent { flag: u16, jump_if_true: usize },
    YesNo { yes_jump: usize, no_jump: usize },
    Jump(usize),
    End,

    // NPC facing
    FacingPlayer { npc_idx: u8 },
    Heal,

    // Sprint 2 additions
    LoadWildMon { species: SpeciesId, level: u8 },
    StartBattle { battle_type: BattleType },
    Follow { npc_idx: u8 },
    StopFollow,
    MoveObject { npc_idx: u8, x: i32, y: i32 },
    PlayMapMusic,
    Special(SpecialFn),
}

// ── ScriptState ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ScriptState {
    pub steps: Vec<ScriptStep>,
    pub pc: usize,
    pub timer: f64,
    pub waiting_for_input: bool,
    pub text_buffer: Option<String>,
    pub showing_yesno: bool,
    pub yesno_cursor: u8,
    pub move_queue: Vec<(Direction, u8)>,
    pub move_progress: f64,
    pub moving_npc: Option<u8>,
    pub loaded_wild_species: Option<(SpeciesId, u8)>,  // Review #2: for LoadWildMon->StartBattle handoff
}

impl ScriptState {
    pub fn new(steps: Vec<ScriptStep>) -> Self {
        Self {
            steps,
            pc: 0,
            timer: 0.0,
            waiting_for_input: false,
            text_buffer: None,
            showing_yesno: false,
            yesno_cursor: 0,
            move_queue: Vec::new(),
            move_progress: 0.0,
            moving_npc: None,
            loaded_wild_species: None,
        }
    }

    pub fn from_id(script_id: u16) -> Self {
        Self::new(get_script(script_id))
    }
}

// ── Script Engine ─────────────────────────────────────────────────────────────

const SCRIPT_WALK_SPEED: f64 = 8.0;
const TILE_PX: f64 = 16.0;

/// Advance the script engine by one frame.
/// Returns ScriptResult indicating running/ended/battle-start.
#[allow(clippy::too_many_arguments)]
pub fn step_script(
    script: &mut ScriptState,
    player: &mut PlayerState,
    npc_states: &mut Vec<NpcState>,
    flags: &mut EventFlags,
    scenes: &mut SceneState,
    _current_map_id: MapId,
    party: &mut Vec<Pokemon>,
    bag: &mut Vec<(u8, u8)>,
    confirm_pressed: bool,
    _cancel_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,
) -> ScriptResult {
    // ── Handle active emotes ───────────────────────────────────────────────
    for npc in npc_states.iter_mut() {
        if let Some((_, ref mut frames)) = npc.emote {
            if *frames > 0 { *frames -= 1; }
            if *frames == 0 { npc.emote = None; }
        }
    }

    // ── Handle movement queue ─────────────────────────────────────────────
    if !script.move_queue.is_empty() {
        let (dir, _count) = script.move_queue[0];
        script.move_progress += SCRIPT_WALK_SPEED;

        if let Some(npc_idx) = script.moving_npc {
            if let Some(npc) = npc_states.get_mut(npc_idx as usize) {
                npc.facing = dir;
                npc.is_walking = true;
                if script.move_progress >= TILE_PX {
                    script.move_progress = 0.0;
                    match dir {
                        Direction::Up    => npc.y -= 1,
                        Direction::Down  => npc.y += 1,
                        Direction::Left  => npc.x -= 1,
                        Direction::Right => npc.x += 1,
                    }
                    npc.is_walking = false;
                    if script.move_queue[0].1 > 1 {
                        script.move_queue[0].1 -= 1;
                    } else {
                        script.move_queue.remove(0);
                        if script.move_queue.is_empty() {
                            script.moving_npc = None;
                            script.pc += 1;
                        }
                    }
                }
            } else {
                script.move_queue.clear();
                script.moving_npc = None;
                script.pc += 1;
            }
        } else {
            player.facing = dir;
            player.is_walking = true;
            player.walk_offset = script.move_progress;
            if script.move_progress >= TILE_PX {
                script.move_progress = 0.0;
                player.walk_offset = 0.0;
                player.is_walking = false;
                match dir {
                    Direction::Up    => player.y -= 1,
                    Direction::Down  => player.y += 1,
                    Direction::Left  => player.x -= 1,
                    Direction::Right => player.x += 1,
                }
                if script.move_queue[0].1 > 1 {
                    script.move_queue[0].1 -= 1;
                } else {
                    script.move_queue.remove(0);
                    if script.move_queue.is_empty() {
                        script.pc += 1;
                    }
                }
            }
        }
        return ScriptResult::Running;
    }

    // ── Handle pause timer ────────────────────────────────────────────────
    if script.timer > 0.0 {
        script.timer -= 1.0 / 60.0;
        if script.timer <= 0.0 {
            script.timer = 0.0;
            script.pc += 1;
        }
        return ScriptResult::Running;
    }

    // ── Handle yes/no input ───────────────────────────────────────────────
    if script.showing_yesno {
        if up_pressed   && script.yesno_cursor > 0 { script.yesno_cursor = 0; }
        if down_pressed && script.yesno_cursor < 1 { script.yesno_cursor = 1; }
        if confirm_pressed {
            script.showing_yesno = false;
            let _ = script.yesno_cursor;
        }
        return ScriptResult::Running;
    }

    // ── Handle wait-for-input ─────────────────────────────────────────────
    if script.waiting_for_input {
        if confirm_pressed {
            script.waiting_for_input = false;
            script.text_buffer = None;
            script.pc += 1;
        }
        return ScriptResult::Running;
    }

    // ── Execute current step ──────────────────────────────────────────────
    if script.pc >= script.steps.len() {
        return ScriptResult::Ended;
    }

    let step = script.steps[script.pc].clone();
    match step {
        ScriptStep::ShowText(text) => {
            script.text_buffer = Some(text);
            script.waiting_for_input = true;
        }

        ScriptStep::WaitButton => {
            script.waiting_for_input = true;
        }

        ScriptStep::CloseText => {
            script.text_buffer = None;
            script.pc += 1;
        }

        ScriptStep::MovePlayer { steps } => {
            if steps.is_empty() {
                script.pc += 1;
            } else {
                script.move_queue = steps;
                script.moving_npc = None;
                script.move_progress = 0.0;
            }
        }

        ScriptStep::MoveNpc { npc_idx, steps } => {
            if steps.is_empty() {
                script.pc += 1;
            } else {
                script.move_queue = steps;
                script.moving_npc = Some(npc_idx);
                script.move_progress = 0.0;
            }
        }

        ScriptStep::TurnNpc { npc_idx, direction } => {
            if let Some(npc) = npc_states.get_mut(npc_idx as usize) {
                npc.facing = direction;
            }
            script.pc += 1;
        }

        ScriptStep::TurnPlayer(direction) => {
            player.facing = direction;
            script.pc += 1;
        }

        ScriptStep::ShowEmote { npc_idx, emote, frames } => {
            if let Some(npc) = npc_states.get_mut(npc_idx as usize) {
                npc.emote = Some((emote, frames));
            }
            script.pc += 1;
        }

        ScriptStep::PlaySound(_) | ScriptStep::PlayMusic(_) => {
            script.pc += 1;
        }

        ScriptStep::Pause(secs) => {
            script.timer = secs;
        }

        ScriptStep::SetEvent(flag) => {
            flags.set(flag);
            script.pc += 1;
        }

        ScriptStep::ClearEvent(flag) => {
            flags.clear(flag);
            script.pc += 1;
        }

        ScriptStep::SetScene { map, scene_id } => {
            scenes.set(map, scene_id);
            script.pc += 1;
        }

        ScriptStep::GiveItem { item_id, count } => {
            if let Some(slot) = bag.iter_mut().find(|(id, _)| *id == item_id) {
                slot.1 = slot.1.saturating_add(count);
            } else {
                bag.push((item_id, count));
            }
            script.pc += 1;
        }

        ScriptStep::GivePokemon { species, level, held_item } => {
            if party.len() < 6 {
                let mut poke = Pokemon::new(species, level);
                poke.held_item = Some(held_item);
                party.push(poke);
            }
            script.pc += 1;
        }

        ScriptStep::HideNpc(npc_idx) => {
            if let Some(npc) = npc_states.get_mut(npc_idx as usize) {
                npc.visible = false;
            }
            script.pc += 1;
        }

        ScriptStep::ShowNpc(npc_idx) => {
            if let Some(npc) = npc_states.get_mut(npc_idx as usize) {
                npc.visible = true;
            }
            script.pc += 1;
        }

        ScriptStep::CheckEvent { flag, jump_if_true } => {
            if flags.has(flag) {
                script.pc = jump_if_true;
            } else {
                script.pc += 1;
            }
        }

        ScriptStep::YesNo { yes_jump, no_jump } => {
            script.showing_yesno = true;
            script.yesno_cursor = 0;
            script.pc = yes_jump;
            let _ = no_jump;
        }

        ScriptStep::Jump(target) => {
            script.pc = target;
        }

        ScriptStep::End => {
            return ScriptResult::Ended;
        }

        ScriptStep::FacingPlayer { npc_idx } => {
            if let Some(npc) = npc_states.get_mut(npc_idx as usize) {
                let dx = player.x - npc.x;
                let dy = player.y - npc.y;
                npc.facing = if dx.abs() >= dy.abs() {
                    if dx > 0 { Direction::Right } else { Direction::Left }
                } else if dy > 0 { Direction::Down } else { Direction::Up };
            }
            script.pc += 1;
        }

        ScriptStep::Heal => {
            for poke in party.iter_mut() {
                poke.hp = poke.max_hp;
                poke.status = super::data::StatusCondition::None;
            }
            script.pc += 1;
        }

        // Sprint 2 new steps
        ScriptStep::LoadWildMon { species, level } => {
            script.loaded_wild_species = Some((species, level));
            script.pc += 1;
        }

        ScriptStep::StartBattle { battle_type } => {
            let species = script.loaded_wild_species.take();
            script.pc += 1;
            return ScriptResult::StartBattle { battle_type, species };
        }

        ScriptStep::Follow { .. } => {
            // Sprint 2 no-op: movement handled by MoveNpc/MovePlayer pairs
            script.pc += 1;
        }

        ScriptStep::StopFollow => {
            script.pc += 1;
        }

        ScriptStep::MoveObject { npc_idx, x, y } => {
            if let Some(npc) = npc_states.get_mut(npc_idx as usize) {
                npc.x = x;
                npc.y = y;
            }
            script.pc += 1;
        }

        ScriptStep::PlayMapMusic => {
            // No-op for Sprint 2 (no audio system)
            script.pc += 1;
        }

        ScriptStep::Special(special_fn) => {
            match special_fn {
                SpecialFn::HealParty => {
                    for p in party.iter_mut() {
                        p.hp = p.max_hp;
                    }
                }
                SpecialFn::RestartMapMusic => { /* no-op Sprint 2 */ }
                SpecialFn::FadeOutMusic => { /* no-op Sprint 2 */ }
            }
            script.pc += 1;
        }
    }

    ScriptResult::Running
}

// ── Script Registry ───────────────────────────────────────────────────────────

pub fn get_script(id: u16) -> Vec<ScriptStep> {
    match id {
        SCRIPT_MEET_MOM          => build_meet_mom_script(),
        SCRIPT_TEACHER_STOPS_1   => build_teacher_stops_script(1),
        SCRIPT_TEACHER_STOPS_2   => build_teacher_stops_script(2),
        SCRIPT_RIVAL_INTERACTION => build_rival_interaction_script(),
        SCRIPT_ELM_INTRO         => build_elm_intro_script(),
        SCRIPT_STARTER_CYNDAQUIL => build_starter_script(0),
        SCRIPT_STARTER_TOTODILE  => build_starter_script(1),
        SCRIPT_STARTER_CHIKORITA => build_starter_script(2),
        SCRIPT_LAB_TRY_TO_LEAVE  => build_lab_try_to_leave_script(),

        SCRIPT_NBT_SIGN => simple_text("NEW BARK TOWN\nThe Town Where the Winds of a New Beginning Blow."),
        SCRIPT_PLAYER_HOUSE_SIGN => simple_text("PLAYER's House"),
        SCRIPT_ELM_LAB_SIGN     => simple_text("ELM POKeMON LAB"),
        SCRIPT_ELM_HOUSE_SIGN   => simple_text("ELM'S HOUSE"),

        SCRIPT_HOUSE1F_STOVE    => simple_text("CINNABAR VOLCANO BURGER!"),
        SCRIPT_HOUSE1F_SINK     => simple_text("The sink is spotless."),
        SCRIPT_HOUSE1F_FRIDGE   => simple_text("FRESH WATER and LEMONADE."),
        SCRIPT_HOUSE1F_TV       => simple_text("A movie about two boys on a train."),
        SCRIPT_HOUSE2F_PC       => simple_text("Accessed own PC."),
        SCRIPT_HOUSE2F_RADIO    => simple_text("Welcome to POKEMON TALK."),
        SCRIPT_HOUSE2F_BOOKSHELF => simple_text("Picture book."),

        SCRIPT_LAB_HEALING_MACHINE => simple_text("A Pokemon healing machine."),
        SCRIPT_LAB_BOOKSHELF    => simple_text("Research papers on Pokemon ecology."),
        SCRIPT_LAB_TRASHCAN     => simple_text("The wrapper from ELM's snack."),
        SCRIPT_LAB_WINDOW       => simple_text("The window is open."),
        SCRIPT_LAB_PC           => simple_text("ELM's research notes."),

        SCRIPT_MEET_OFFICER | SCRIPT_AIDE_GIVES_POTION | SCRIPT_AIDE_GIVES_BALLS => {
            vec![ScriptStep::End]
        }

        // Sprint 2: Route 29 scripts
        SCRIPT_ROUTE29_SIGN1 => simple_text("ROUTE 29\n\nCHERRYGROVE CITY -\nNEW BARK TOWN"),
        SCRIPT_ROUTE29_SIGN2 => simple_text("ROUTE 29\n\nCHERRYGROVE CITY -\nNEW BARK TOWN"),
        SCRIPT_CATCHING_TUTORIAL_DUDE => build_catching_tutorial_dude_script(),
        SCRIPT_CATCHING_TUTORIAL_1 | SCRIPT_CATCHING_TUTORIAL_2 => build_catching_tutorial_encounter(),
        SCRIPT_ROUTE29_YOUNGSTER => simple_text("YOUNGSTER: I'm training to be the best!"),
        SCRIPT_ROUTE29_TEACHER  => simple_text("If you catch all the POKeMON, you'll be famous!"),
        SCRIPT_ROUTE29_FISHER   => simple_text("You can fish for POKeMON in ponds and rivers."),
        SCRIPT_ROUTE29_COOLTRAINER_M => simple_text("I raised my POKeMON carefully!"),
        SCRIPT_ROUTE29_FRUIT_TREE => build_fruit_tree_script(),
        SCRIPT_ROUTE29_POTION   => build_route29_potion_script(),
        SCRIPT_TUSCANY          => build_tuscany_script(),

        // Sprint 2: Gate scripts
        SCRIPT_GATE_OFFICER     => simple_text("This gate connects ROUTE 29 and ROUTE 46."),
        SCRIPT_GATE_YOUNGSTER   => simple_text("I can't wait to explore ROUTE 46!"),

        // Sprint 2: Cherrygrove scripts
        SCRIPT_GUIDE_GENT_CITY  => build_guide_gent_tour_script(),
        SCRIPT_CHERRYGROVE_RIVAL => build_cherrygrove_rival_script(),
        SCRIPT_CHERRYGROVE_TEACHER => simple_text("This is CHERRYGROVE CITY.\nThe flower city!"),
        SCRIPT_CHERRYGROVE_YOUNGSTER => simple_text("I love exploring the city."),
        SCRIPT_MYSTIC_WATER_GUY => build_mystic_water_guy_script(),
        SCRIPT_CHERRYGROVE_SIGN => simple_text("CHERRYGROVE CITY\n\nThe City of Cute,\nFragrant Flowers"),
        SCRIPT_GUIDE_GENT_HOUSE_SIGN => simple_text("GUIDE GENT'S HOUSE"),
        SCRIPT_CHERRYGROVE_MART_SIGN => simple_text("CHERRYGROVE MART"),
        SCRIPT_CHERRYGROVE_POKECENTER_SIGN => simple_text("POKEMON CENTER"),

        // Sprint 2: Pokecenter scripts
        SCRIPT_CHERRYGROVE_NURSE => build_pokecenter_nurse_script(),
        SCRIPT_POKECENTER_FISHER => simple_text("It's great. I can store any number of POKeMON, and it's all free."),
        SCRIPT_POKECENTER_GENTLEMAN => simple_text("That PC is free for any trainer to use."),
        SCRIPT_POKECENTER_TEACHER => simple_text("Our POKeMON CENTER heals your POKeMON!"),

        // Sprint 2: Mart scripts
        SCRIPT_CHERRYGROVE_CLERK => build_mart_clerk_script(),
        SCRIPT_MART_COOLTRAINER => simple_text("They're fresh out of POKe BALLS!"),
        SCRIPT_MART_YOUNGSTER   => simple_text("When I was in the grass, a bug POKeMON poisoned mine!"),

        // Sprint 2: Houses
        SCRIPT_GUIDE_GENT_HOME  => simple_text("When I was a wee lad, I was a hot-shot trainer!"),
        SCRIPT_GUIDE_GENT_BOOKSHELF => simple_text("A book about famous trainers."),
        SCRIPT_GYM_SPEECH_POKEFAN => simple_text("You're trying to see how good you are as a POKeMON trainer?"),
        SCRIPT_GYM_SPEECH_BUG_CATCHER => simple_text("When I get older, I'm going to be a GYM LEADER!"),
        SCRIPT_GYM_SPEECH_BOOKSHELF => simple_text("A book about GYM BADGES."),
        SCRIPT_EVO_SPEECH_LASS  => simple_text("POKeMON change? I would be shocked if one did that!"),
        SCRIPT_EVO_SPEECH_YOUNGSTER => simple_text("POKeMON gain experience in battle and change their form."),
        SCRIPT_EVO_SPEECH_BOOKSHELF => simple_text("A book about POKeMON evolution."),

        _ => vec![ScriptStep::End],
    }
}

fn simple_text(text: &str) -> Vec<ScriptStep> {
    vec![
        ScriptStep::ShowText(text.to_string()),
        ScriptStep::End,
    ]
}

// ── Script Builders ───────────────────────────────────────────────────────────

pub fn build_elm_intro_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::MovePlayer { steps: vec![(Direction::Up, 7)] },
        ScriptStep::TurnPlayer(Direction::Left),
        ScriptStep::ShowEmote { npc_idx: 0, emote: Emote::Shock, frames: 15 },
        ScriptStep::TurnNpc { npc_idx: 0, direction: Direction::Right },
        ScriptStep::ShowText("ELM: <PLAYER>! There you are!".to_string()),
        ScriptStep::ShowText("ELM: I needed to ask you a favor.".to_string()),
        ScriptStep::ShowText("ELM: You see, a POKeMON acquaintance, MR.POKeMON, has a discovery.".to_string()),
        ScriptStep::ShowText("ELM: Will you go see what it's about?".to_string()),
        ScriptStep::PlaySound(1),
        ScriptStep::Pause(0.5),
        ScriptStep::ShowEmote { npc_idx: 0, emote: Emote::Shock, frames: 10 },
        ScriptStep::TurnNpc { npc_idx: 0, direction: Direction::Down },
        ScriptStep::ShowText("ELM: Hm? I have an email!".to_string()),
        ScriptStep::ShowText("ELM: ...It's from MR.POKeMON!".to_string()),
        ScriptStep::ShowText("ELM: Go ahead--pick a POKeMON!".to_string()),
        ScriptStep::MoveNpc { npc_idx: 0, steps: vec![(Direction::Up, 1)] },
        ScriptStep::MoveNpc { npc_idx: 0, steps: vec![(Direction::Right, 2), (Direction::Up, 1)] },
        ScriptStep::TurnNpc { npc_idx: 0, direction: Direction::Down },
        ScriptStep::TurnPlayer(Direction::Up),
        ScriptStep::TurnPlayer(Direction::Right),
        ScriptStep::SetScene { map: MapId::ElmsLab, scene_id: SCENE_ELMSLAB_CANT_LEAVE },
        ScriptStep::End,
    ]
}

fn build_starter_script(choice: u8) -> Vec<ScriptStep> {
    let (species, name, event_got, event_ball) = match choice {
        0 => (CYNDAQUIL, "CYNDAQUIL", EVENT_GOT_CYNDAQUIL_FROM_ELM, EVENT_CYNDAQUIL_POKEBALL_IN_ELMS_LAB),
        1 => (TOTODILE,  "TOTODILE",  EVENT_GOT_TOTODILE_FROM_ELM,  EVENT_TOTODILE_POKEBALL_IN_ELMS_LAB),
        _ => (CHIKORITA, "CHIKORITA", EVENT_GOT_CHIKORITA_FROM_ELM, EVENT_CHIKORITA_POKEBALL_IN_ELMS_LAB),
    };
    let npc_idx = (choice + 2) as u8;
    let left_steps = (choice + 1) as u8;

    vec![
        ScriptStep::ShowText(format!("It's {}! Will you take it?", name)),
        ScriptStep::YesNo { yes_jump: 2, no_jump: 12 },
        // Yes path:
        ScriptStep::GivePokemon { species, level: 5, held_item: ITEM_BERRY },
        ScriptStep::SetEvent(event_got),
        ScriptStep::SetEvent(EVENT_GOT_A_POKEMON_FROM_ELM),
        ScriptStep::SetEvent(event_ball),
        ScriptStep::HideNpc(npc_idx),
        ScriptStep::MovePlayer { steps: vec![(Direction::Left, left_steps), (Direction::Up, 1)] },
        ScriptStep::TurnPlayer(Direction::Up),
        ScriptStep::ShowText("ELM: I knew you'd pick that one!".to_string()),
        ScriptStep::ShowText("ELM: You can use that healing machine any time.".to_string()),
        ScriptStep::ShowText("ELM: Now, head to MR.POKeMON's place on Route 30!".to_string()),
        ScriptStep::SetScene { map: MapId::ElmsLab, scene_id: SCENE_ELMSLAB_AIDE_GIVES_POTION },
        ScriptStep::SetScene { map: MapId::NewBarkTown, scene_id: SCENE_NEWBARKTOWN_NOOP },
        ScriptStep::End,
        // No path (unreachable):
        ScriptStep::End,
    ]
}

fn build_meet_mom_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::FacingPlayer { npc_idx: 0 },
        ScriptStep::ShowText("MOM: Oh, <PLAYER>! ...PROF.ELM was looking for you.".to_string()),
        ScriptStep::ShowText("MOM: Here, take this. It's the POKEGEAR!".to_string()),
        ScriptStep::GiveItem { item_id: 59, count: 1 },
        ScriptStep::SetEvent(EVENT_ENGINE_POKEGEAR),
        ScriptStep::ShowText("MOM: PROF.ELM is in his lab next door.".to_string()),
        ScriptStep::SetScene { map: MapId::PlayersHouse1F, scene_id: SCENE_PLAYERSHOUSE1F_NOOP },
        ScriptStep::SetEvent(EVENT_PLAYERS_HOUSE_MOM_1),
        ScriptStep::ClearEvent(EVENT_PLAYERS_HOUSE_MOM_2),
        ScriptStep::End,
    ]
}

fn build_teacher_stops_script(variant: u8) -> Vec<ScriptStep> {
    let right_steps = if variant == 1 { 4u8 } else { 5u8 };
    vec![
        ScriptStep::ShowText("Wait, <PLAYER>!".to_string()),
        ScriptStep::ShowText("What do you think you're doing?".to_string()),
        ScriptStep::ShowText("It's dangerous to go out without a #MON!".to_string()),
        ScriptStep::MovePlayer { steps: vec![(Direction::Right, right_steps)] },
        ScriptStep::TurnPlayer(Direction::Left),
        ScriptStep::End,
    ]
}

fn build_rival_interaction_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::FacingPlayer { npc_idx: 2 },
        ScriptStep::ShowText("...So this is the famous ELM POKeMON LAB...".to_string()),
        ScriptStep::TurnNpc { npc_idx: 2, direction: Direction::Left },
        ScriptStep::Pause(0.3),
        ScriptStep::TurnNpc { npc_idx: 2, direction: Direction::Right },
        ScriptStep::ShowText("...What are you staring at?".to_string()),
        ScriptStep::End,
    ]
}

fn build_lab_try_to_leave_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::ShowText("ELM: Wait! You need to try out that Pokemon first!".to_string()),
        ScriptStep::ShowText("ELM: Go ahead and use the healing machine!".to_string()),
        ScriptStep::MovePlayer { steps: vec![(Direction::Up, 1)] },
        ScriptStep::End,
    ]
}

pub fn build_post_starter_script(choice: u8) -> Vec<ScriptStep> {
    build_starter_script(choice)
}

// Sprint 2 script builders

fn build_catching_tutorial_dude_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::FacingPlayer { npc_idx: 0 },
        ScriptStep::ShowText("DUDE: Oh, you're a new trainer!".to_string()),
        ScriptStep::ShowText("DUDE: Want me to show you how to catch a POKeMON?".to_string()),
        ScriptStep::SetEvent(EVENT_DUDE_TALKED_TO_YOU),
        ScriptStep::ShowText("DUDE: Watch closely!".to_string()),
        ScriptStep::LoadWildMon { species: HOPPIP, level: 3 },
        ScriptStep::StartBattle { battle_type: BattleType::Tutorial },
        ScriptStep::ShowText("DUDE: See? That's how you catch one!".to_string()),
        ScriptStep::SetEvent(EVENT_LEARNED_TO_CATCH_POKEMON),
        ScriptStep::SetScene { map: MapId::Route29, scene_id: SCENE_ROUTE29_NOOP },
        ScriptStep::End,
    ]
}

fn build_catching_tutorial_encounter() -> Vec<ScriptStep> {
    vec![
        ScriptStep::LoadWildMon { species: HOPPIP, level: 3 },
        ScriptStep::StartBattle { battle_type: BattleType::Tutorial },
        ScriptStep::SetScene { map: MapId::Route29, scene_id: SCENE_ROUTE29_NOOP },
        ScriptStep::End,
    ]
}

fn build_fruit_tree_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::ShowText("A tree with unusual berries!".to_string()),
        ScriptStep::End,
    ]
}

fn build_route29_potion_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::GiveItem { item_id: ITEM_POTION, count: 1 },
        ScriptStep::SetEvent(EVENT_ROUTE_29_POTION),
        ScriptStep::ShowText("Found a POTION!".to_string()),
        ScriptStep::End,
    ]
}

fn build_tuscany_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::FacingPlayer { npc_idx: 6 },
        ScriptStep::ShowText("TUSCANY: It's Tuesday! Here's a gift!".to_string()),
        ScriptStep::SetEvent(EVENT_MET_TUSCANY_OF_TUESDAY),
        ScriptStep::SetEvent(EVENT_GOT_PINK_BOW_FROM_TUSCANY),
        ScriptStep::ShowText("Received PINK BOW!".to_string()),
        ScriptStep::End,
    ]
}

fn build_guide_gent_tour_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::FacingPlayer { npc_idx: 0 },
        ScriptStep::ShowText("OLD MAN: Are you new to this town?".to_string()),
        ScriptStep::ShowText("OLD MAN: Allow me to show you around!".to_string()),
        ScriptStep::PlayMusic(MUSIC_SHOW_ME_AROUND),
        ScriptStep::Follow { npc_idx: 0 },
        // Walk tour: guide moves, player follows
        ScriptStep::MoveNpc { npc_idx: 0, steps: vec![(Direction::Left, 3)] },
        ScriptStep::MovePlayer { steps: vec![(Direction::Left, 3)] },
        ScriptStep::ShowText("OLD MAN: That's the POKEMON CENTER.\nYour POKeMON get healed for free!".to_string()),
        ScriptStep::MoveNpc { npc_idx: 0, steps: vec![(Direction::Left, 4)] },
        ScriptStep::MovePlayer { steps: vec![(Direction::Left, 4)] },
        ScriptStep::ShowText("OLD MAN: That building over there is the MART.\nBuy useful items there.".to_string()),
        ScriptStep::MoveNpc { npc_idx: 0, steps: vec![(Direction::Down, 3), (Direction::Left, 2)] },
        ScriptStep::MovePlayer { steps: vec![(Direction::Down, 3), (Direction::Left, 2)] },
        ScriptStep::ShowText("OLD MAN: And this is my house!".to_string()),
        ScriptStep::StopFollow,
        // GIMLI FIX: MAP_CARD is NOT a bag item -- use SetEvent only
        ScriptStep::SetEvent(EVENT_ENGINE_MAP_CARD),
        ScriptStep::ShowText("<PLAYER>'s POKeGEAR now has a MAP!".to_string()),
        ScriptStep::SetEvent(EVENT_GUIDE_GENT_IN_HIS_HOUSE),
        ScriptStep::SetEvent(EVENT_GUIDE_GENT_VISIBLE_IN_CHERRYGROVE),
        ScriptStep::PlayMapMusic,
        ScriptStep::End,
    ]
}

fn build_cherrygrove_rival_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::FacingPlayer { npc_idx: 1 },
        ScriptStep::PlayMusic(MUSIC_RIVAL_ENCOUNTER),
        ScriptStep::ShowText("RIVAL: Heh! Look who made it to CHERRYGROVE!".to_string()),
        ScriptStep::ShowText("RIVAL: Let's see whose POKeMON is better!".to_string()),
        ScriptStep::StartBattle { battle_type: BattleType::CanLose },
        ScriptStep::PlayMusic(MUSIC_RIVAL_AFTER),
        ScriptStep::ShowText("RIVAL: Hmph... Not bad. But I'll beat you next time!".to_string()),
        ScriptStep::SetScene { map: MapId::CherrygroveCity, scene_id: SCENE_CHERRYGROVECITY_NOOP },
        ScriptStep::ClearEvent(EVENT_RIVAL_CHERRYGROVE_CITY),
        ScriptStep::PlayMapMusic,
        ScriptStep::End,
    ]
}

fn build_mystic_water_guy_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::FacingPlayer { npc_idx: 4 },
        ScriptStep::CheckEvent { flag: EVENT_GOT_MYSTIC_WATER_IN_CHERRYGROVE, jump_if_true: 5 },
        ScriptStep::ShowText("FISHER: Here, take this MYSTIC WATER!\nIt powers up WATER-type moves!".to_string()),
        ScriptStep::GiveItem { item_id: ITEM_MYSTIC_WATER, count: 1 },
        ScriptStep::SetEvent(EVENT_GOT_MYSTIC_WATER_IN_CHERRYGROVE),
        // jump target 5:
        ScriptStep::ShowText("FISHER: MYSTIC WATER is great for WATER-type moves!".to_string()),
        ScriptStep::End,
    ]
}

fn build_pokecenter_nurse_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::FacingPlayer { npc_idx: 0 },
        ScriptStep::ShowText("NURSE: Welcome to the POKeMON CENTER!".to_string()),
        ScriptStep::ShowText("NURSE: We heal your POKeMON back to full health!".to_string()),
        ScriptStep::Special(SpecialFn::HealParty),
        ScriptStep::ShowText("NURSE: Your POKeMON are fighting fit!".to_string()),
        ScriptStep::End,
    ]
}

fn build_mart_clerk_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::FacingPlayer { npc_idx: 0 },
        ScriptStep::ShowText("CLERK: Welcome! May I help you?".to_string()),
        ScriptStep::ShowText("CLERK: We have POTION - 300\nANTIDOTE - 100".to_string()),
        ScriptStep::ShowText("CLERK: Here's a free POTION for your first visit!".to_string()),
        ScriptStep::GiveItem { item_id: ITEM_POTION, count: 1 },
        ScriptStep::End,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_flags_basic() {
        let mut flags = EventFlags::new();
        assert!(!flags.has(EVENT_GOT_A_POKEMON_FROM_ELM));
        flags.set(EVENT_GOT_A_POKEMON_FROM_ELM);
        assert!(flags.has(EVENT_GOT_A_POKEMON_FROM_ELM));
        flags.clear(EVENT_GOT_A_POKEMON_FROM_ELM);
        assert!(!flags.has(EVENT_GOT_A_POKEMON_FROM_ELM));
    }

    #[test]
    fn test_scene_state_basic() {
        let mut s = SceneState::new();
        assert_eq!(s.get(MapId::ElmsLab), 0);
        s.set(MapId::ElmsLab, SCENE_ELMSLAB_CANT_LEAVE);
        assert_eq!(s.get(MapId::ElmsLab), SCENE_ELMSLAB_CANT_LEAVE);
    }

    #[test]
    fn test_scene_state_size() {
        // Review #9: expanded from 16 to 32
        let s = SceneState::new();
        // MapId::CherrygroveCity = 9 (well within 32)
        assert_eq!(s.get(MapId::CherrygroveCity), 0);
    }

    #[test]
    fn test_script_result_running() {
        let steps = vec![
            ScriptStep::ShowText("Hello!".to_string()),
            ScriptStep::End,
        ];
        let mut script = ScriptState::new(steps);
        let mut player = PlayerState {
            x: 0, y: 0, facing: Direction::Down,
            walk_offset: 0.0, is_walking: false,
            walk_frame: 0, frame_timer: 0.0,
            name: "TEST".to_string(),
        };
        let mut npc_states = Vec::new();
        let mut flags = EventFlags::new();
        let mut scenes = SceneState::new();
        let mut party = Vec::new();
        let mut bag = Vec::new();

        let result = step_script(&mut script, &mut player, &mut npc_states,
            &mut flags, &mut scenes, MapId::NewBarkTown,
            &mut party, &mut bag, false, false, false, false);
        assert!(matches!(result, ScriptResult::Running));
        assert!(script.text_buffer.is_some());

        // confirm to advance past text
        let _ = step_script(&mut script, &mut player, &mut npc_states,
            &mut flags, &mut scenes, MapId::NewBarkTown,
            &mut party, &mut bag, true, false, false, false);

        let result = step_script(&mut script, &mut player, &mut npc_states,
            &mut flags, &mut scenes, MapId::NewBarkTown,
            &mut party, &mut bag, false, false, false, false);
        assert!(matches!(result, ScriptResult::Ended), "Script should end after End step");
    }

    #[test]
    fn test_load_wild_mon_then_start_battle() {
        let steps = vec![
            ScriptStep::LoadWildMon { species: 16, level: 3 },
            ScriptStep::StartBattle { battle_type: BattleType::Tutorial },
            ScriptStep::End,
        ];
        let mut script = ScriptState::new(steps);
        let mut player = PlayerState {
            x: 0, y: 0, facing: Direction::Down,
            walk_offset: 0.0, is_walking: false,
            walk_frame: 0, frame_timer: 0.0,
            name: "TEST".to_string(),
        };
        let mut npc_states = Vec::new();
        let mut flags = EventFlags::new();
        let mut scenes = SceneState::new();
        let mut party = Vec::new();
        let mut bag = Vec::new();

        // Frame 1: LoadWildMon
        let result = step_script(&mut script, &mut player, &mut npc_states,
            &mut flags, &mut scenes, MapId::Route29,
            &mut party, &mut bag, false, false, false, false);
        assert!(matches!(result, ScriptResult::Running));
        assert_eq!(script.loaded_wild_species, Some((16, 3)));

        // Frame 2: StartBattle
        let result = step_script(&mut script, &mut player, &mut npc_states,
            &mut flags, &mut scenes, MapId::Route29,
            &mut party, &mut bag, false, false, false, false);
        assert!(matches!(result, ScriptResult::StartBattle { battle_type: BattleType::Tutorial, species: Some((16, 3)) }));
    }

    #[test]
    fn test_guide_gent_gives_map_card() {
        let steps = build_guide_gent_tour_script();
        let has_set_map_card = steps.iter().any(|s| matches!(s, ScriptStep::SetEvent(EVENT_ENGINE_MAP_CARD)));
        let has_give_item_map_card = steps.iter().any(|s| matches!(s, ScriptStep::GiveItem { item_id: 43, .. }));
        assert!(has_set_map_card, "Guide Gent script should set EVENT_ENGINE_MAP_CARD");
        assert!(!has_give_item_map_card, "GIMLI FIX: Guide Gent should NOT give MAP_CARD as a bag item");
    }

    #[test]
    fn test_all_scripts_compile() {
        let ids = [1u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
                   200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
                   220, 221, 230, 231, 232, 233, 234, 250, 260, 270, 280, 290];
        for id in ids {
            let steps = get_script(id);
            assert!(!steps.is_empty(), "Script {} returned empty", id);
        }
    }

    // ── Sprint 3 QA: Group 2 — Meet Mom Script ─────────────────────────

    #[test]
    fn test_meet_mom_script_gives_pokegear() {
        let steps = get_script(SCRIPT_MEET_MOM);
        let has_pokegear_event = steps.iter().any(|s| matches!(s, ScriptStep::SetEvent(EVENT_ENGINE_POKEGEAR)));
        let has_pokegear_item = steps.iter().any(|s| matches!(s, ScriptStep::GiveItem { item_id: 59, count: 1 }));
        let has_scene_noop = steps.iter().any(|s| matches!(s,
            ScriptStep::SetScene { map: MapId::PlayersHouse1F, scene_id: 1 }));
        let has_set_mom1 = steps.iter().any(|s| matches!(s, ScriptStep::SetEvent(EVENT_PLAYERS_HOUSE_MOM_1)));
        let has_clear_mom2 = steps.iter().any(|s| matches!(s, ScriptStep::ClearEvent(EVENT_PLAYERS_HOUSE_MOM_2)));
        assert!(has_pokegear_event, "Should set EVENT_ENGINE_POKEGEAR");
        assert!(has_pokegear_item, "Should give PokeGear item 59");
        assert!(has_scene_noop, "Should set PlayersHouse1F scene to NOOP");
        assert!(has_set_mom1, "Should set EVENT_PLAYERS_HOUSE_MOM_1");
        assert!(has_clear_mom2, "Should clear EVENT_PLAYERS_HOUSE_MOM_2");
    }

    #[test]
    fn test_meet_mom_script_execution() {
        let mut script = ScriptState::from_id(SCRIPT_MEET_MOM);
        let mut player = PlayerState {
            x: 8, y: 4, facing: Direction::Down,
            walk_offset: 0.0, is_walking: false,
            walk_frame: 0, frame_timer: 0.0,
            name: "GOLD".to_string(),
        };
        let mut npc_states = vec![NpcState {
            x: 7, y: 4, facing: Direction::Left, walk_offset: 0.0,
            is_walking: false, visible: true, wander_timer: 0.0, emote: None,
        }];
        let mut flags = EventFlags::new();
        let mut scenes = SceneState::new();
        let mut party = Vec::new();
        let mut bag = Vec::new();

        // Run script to completion (press confirm at each text prompt)
        for _ in 0..200 {
            let result = step_script(&mut script, &mut player, &mut npc_states,
                &mut flags, &mut scenes, MapId::PlayersHouse1F,
                &mut party, &mut bag, true, false, false, false);
            if matches!(result, ScriptResult::Ended) { break; }
        }
        assert!(flags.has(EVENT_ENGINE_POKEGEAR), "After script: pokegear flag set");
        assert!(bag.iter().any(|(id, _)| *id == 59), "After script: bag has pokegear");
        assert_eq!(scenes.get(MapId::PlayersHouse1F), SCENE_PLAYERSHOUSE1F_NOOP);
    }

    // ── Sprint 3 QA: Group 3 — Teacher Stops Script ────────────────────

    #[test]
    fn test_teacher_stops_scripts_move_player_right() {
        let steps1 = get_script(SCRIPT_TEACHER_STOPS_1);
        let has_move_right = steps1.iter().any(|s| match s {
            ScriptStep::MovePlayer { steps } => steps.iter().any(|(d, _)| *d == Direction::Right),
            _ => false,
        });
        assert!(has_move_right, "Teacher stops script 1 should move player right");

        let steps2 = get_script(SCRIPT_TEACHER_STOPS_2);
        let has_move_right2 = steps2.iter().any(|s| match s {
            ScriptStep::MovePlayer { steps } => steps.iter().any(|(d, _)| *d == Direction::Right),
            _ => false,
        });
        assert!(has_move_right2, "Teacher stops script 2 should move player right");
    }

    // ── Sprint 3 QA: Group 4 — Starter Selection ───────────────────────

    #[test]
    fn test_cant_leave_lab_without_starter() {
        let steps = get_script(SCRIPT_LAB_TRY_TO_LEAVE);
        let has_push_back = steps.iter().any(|s| match s {
            ScriptStep::MovePlayer { steps } => steps.iter().any(|(d, _)| *d == Direction::Up),
            _ => false,
        });
        assert!(has_push_back, "Lab try-to-leave script should push player back (Up)");
    }

    // ── Sprint 3 QA: Group 5 — Route 29 Scripts ────────────────────────

    #[test]
    fn test_route29_potion_script_gives_item() {
        let steps = get_script(SCRIPT_ROUTE29_POTION);
        let has_give_potion = steps.iter().any(|s| matches!(s, ScriptStep::GiveItem { item_id: ITEM_POTION, count: 1 }));
        let has_set_flag = steps.iter().any(|s| matches!(s, ScriptStep::SetEvent(EVENT_ROUTE_29_POTION)));
        assert!(has_give_potion, "Should give ITEM_POTION");
        assert!(has_set_flag, "Should set EVENT_ROUTE_29_POTION");
    }

    #[test]
    fn test_catching_tutorial_dude_starts_tutorial_battle() {
        let steps = get_script(SCRIPT_CATCHING_TUTORIAL_DUDE);
        let has_load_wild = steps.iter().any(|s| matches!(s, ScriptStep::LoadWildMon { species: HOPPIP, level: 3 }));
        let has_tutorial_battle = steps.iter().any(|s| matches!(s, ScriptStep::StartBattle { battle_type: BattleType::Tutorial }));
        let has_learned_flag = steps.iter().any(|s| matches!(s, ScriptStep::SetEvent(EVENT_LEARNED_TO_CATCH_POKEMON)));
        assert!(has_load_wild, "Should load Hoppip level 3");
        assert!(has_tutorial_battle, "Should start Tutorial battle");
        assert!(has_learned_flag, "Should set EVENT_LEARNED_TO_CATCH_POKEMON");
    }

    // ── Sprint 3 QA: Group 7 — Cherrygrove Scripts ─────────────────────

    #[test]
    fn test_guide_gent_tour_script_correctness() {
        let steps = build_guide_gent_tour_script();
        let has_map_card_flag = steps.iter().any(|s| matches!(s, ScriptStep::SetEvent(EVENT_ENGINE_MAP_CARD)));
        let has_gent_in_house = steps.iter().any(|s| matches!(s, ScriptStep::SetEvent(EVENT_GUIDE_GENT_IN_HIS_HOUSE)));
        let has_gent_visible = steps.iter().any(|s| matches!(s, ScriptStep::SetEvent(EVENT_GUIDE_GENT_VISIBLE_IN_CHERRYGROVE)));
        let has_follow = steps.iter().any(|s| matches!(s, ScriptStep::Follow { .. }));
        let has_stop_follow = steps.iter().any(|s| matches!(s, ScriptStep::StopFollow));
        let has_play_music = steps.iter().any(|s| matches!(s, ScriptStep::PlayMusic(MUSIC_SHOW_ME_AROUND)));
        let has_play_map_music = steps.iter().any(|s| matches!(s, ScriptStep::PlayMapMusic));
        assert!(has_map_card_flag, "Should set EVENT_ENGINE_MAP_CARD flag");
        assert!(has_gent_in_house);
        assert!(has_gent_visible);
        assert!(has_follow);
        assert!(has_stop_follow);
        assert!(has_play_music);
        assert!(has_play_map_music);
    }

    #[test]
    fn test_rival_ambush_starts_canlose_battle() {
        let steps = get_script(SCRIPT_CHERRYGROVE_RIVAL);
        let has_canlose = steps.iter().any(|s| matches!(s, ScriptStep::StartBattle { battle_type: BattleType::CanLose }));
        let has_rival_music = steps.iter().any(|s| matches!(s, ScriptStep::PlayMusic(MUSIC_RIVAL_ENCOUNTER)));
        assert!(has_canlose, "Cherrygrove rival should use BattleType::CanLose");
        assert!(has_rival_music, "Should play rival encounter music");
    }

    #[test]
    fn test_mystic_water_guy_gives_item_once() {
        let steps = get_script(SCRIPT_MYSTIC_WATER_GUY);
        let has_check = steps.iter().any(|s| matches!(s, ScriptStep::CheckEvent { flag: EVENT_GOT_MYSTIC_WATER_IN_CHERRYGROVE, .. }));
        let has_give = steps.iter().any(|s| matches!(s, ScriptStep::GiveItem { item_id: ITEM_MYSTIC_WATER, count: 1 }));
        let has_set = steps.iter().any(|s| matches!(s, ScriptStep::SetEvent(EVENT_GOT_MYSTIC_WATER_IN_CHERRYGROVE)));
        assert!(has_check, "Should check if already received");
        assert!(has_give, "Should give ITEM_MYSTIC_WATER");
        assert!(has_set, "Should set the received flag");
    }

    #[test]
    fn test_pokecenter_nurse_heals_party() {
        let steps = get_script(SCRIPT_CHERRYGROVE_NURSE);
        let has_heal = steps.iter().any(|s| matches!(s, ScriptStep::Special(SpecialFn::HealParty)));
        assert!(has_heal, "Nurse script should heal party via Special::HealParty");
    }

    #[test]
    fn test_mart_clerk_gives_free_potion() {
        let steps = get_script(SCRIPT_CHERRYGROVE_CLERK);
        let has_give_potion = steps.iter().any(|s| matches!(s, ScriptStep::GiveItem { item_id: ITEM_POTION, count: 1 }));
        assert!(has_give_potion, "Mart clerk should give free POTION");
    }
}
