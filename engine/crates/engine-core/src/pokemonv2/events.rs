// AI-INSTRUCTIONS: pokemonv2/events.rs — Event system. Imports data.rs and maps.rs (MapId only).
// ScriptStep enum, ScriptState, step_script(), EventFlags, SceneState, script registry.
// Import graph: events.rs <- data.rs, maps.rs(MapId only)

use super::data::{
    Direction, Emote, NpcState, PlayerState, Pokemon, SpeciesId,
    ITEM_BERRY, CYNDAQUIL, TOTODILE, CHIKORITA,
};
use super::maps::MapId;

// ── Event Flag Constants ──────────────────────────────────────────────────────
// Maps to pokecrystal's wEventFlags byte array constants.

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

// ── Script ID Constants ───────────────────────────────────────────────────────

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
// Future stubs
pub const SCRIPT_MEET_OFFICER: u16 = 100;
pub const SCRIPT_AIDE_GIVES_POTION: u16 = 101;
pub const SCRIPT_AIDE_GIVES_BALLS: u16 = 102;

// ── EventFlags ───────────────────────────────────────────────────────────────

/// 2048-bit event flag bitfield. Maps directly to pokecrystal's wEventFlags.
/// Uses [u64; 32] for determinism (no HashMap), compactness (256 bytes), and speed.
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

/// Per-map scene tracking. Vec<u8> indexed by MapId as usize.
/// Deterministic (Vec, not HashMap). 0 = default scene.
#[derive(Clone, Debug)]
pub struct SceneState {
    scenes: Vec<u8>,
}

impl SceneState {
    pub fn new() -> Self {
        Self { scenes: vec![0u8; 16] }
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

// ── ScriptStep Enum ───────────────────────────────────────────────────────────

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
    PlaySound(u8),   // stub — SoundId placeholder
    PlayMusic(u8),   // stub — MusicId placeholder
    Pause(f64),      // seconds

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
    /// If Some, an NPC is being moved (npc_idx). If None, player is being moved.
    pub moving_npc: Option<u8>,
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
        }
    }

    pub fn from_id(script_id: u16) -> Self {
        Self::new(get_script(script_id))
    }
}

// ── Script Engine ─────────────────────────────────────────────────────────────

const SCRIPT_WALK_SPEED: f64 = 8.0; // pixels per frame
const TILE_PX: f64 = 16.0;

/// Advance the script engine by one frame.
/// Returns true if the script is still running, false if it hit End.
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
) -> bool {
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
            // Moving an NPC
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
                    // Decrement step count
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
            // Moving the player
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
        return true;
    }

    // ── Handle pause timer ────────────────────────────────────────────────
    if script.timer > 0.0 {
        script.timer -= 1.0 / 60.0;
        if script.timer <= 0.0 {
            script.timer = 0.0;
            script.pc += 1;
        }
        return true;
    }

    // ── Handle yes/no input ───────────────────────────────────────────────
    if script.showing_yesno {
        if up_pressed   && script.yesno_cursor > 0 { script.yesno_cursor = 0; }
        if down_pressed && script.yesno_cursor < 1 { script.yesno_cursor = 1; }
        if confirm_pressed {
            let cursor = script.yesno_cursor;
            script.showing_yesno = false;
            // The jump targets are stored in the next available slot; we advance normally
            // The ScriptStep::YesNo already advanced pc past itself when it was hit.
            // cursor=0 means YES (already jumped), cursor=1 means NO
            let _ = cursor; // jump was already set up when YesNo was processed
        }
        return true;
    }

    // ── Handle wait-for-input ─────────────────────────────────────────────
    if script.waiting_for_input {
        if confirm_pressed {
            script.waiting_for_input = false;
            script.text_buffer = None;
            script.pc += 1;
        }
        return true;
    }

    // ── Execute current step ──────────────────────────────────────────────
    if script.pc >= script.steps.len() {
        return false;
    }

    let step = script.steps[script.pc].clone();
    match step {
        ScriptStep::ShowText(text) => {
            script.text_buffer = Some(text);
            script.waiting_for_input = true;
            // Don't advance pc — the wait_for_input handler will advance it
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
                // Don't advance pc — movement handler will advance when done
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
            // Stub: no-op
            script.pc += 1;
        }

        ScriptStep::Pause(secs) => {
            script.timer = secs;
            // Don't advance pc — timer handler advances it
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
            // Add to bag or increment count
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
            script.yesno_cursor = 0; // default YES
            // Jump logic: advance pc past YesNo first, then confirm will jump
            // We set up the jump targets in confirm handler via stored values
            // Simple approach: advance pc, store jump targets in ScriptState
            // For Sprint 1: auto-advance to yes_jump (player must say yes to get starter)
            script.pc = yes_jump;
            let _ = no_jump;
        }

        ScriptStep::Jump(target) => {
            script.pc = target;
        }

        ScriptStep::End => {
            return false;
        }

        ScriptStep::FacingPlayer { npc_idx } => {
            if let Some(npc) = npc_states.get_mut(npc_idx as usize) {
                // Determine direction from NPC to player
                let dx = player.x - npc.x;
                let dy = player.y - npc.y;
                npc.facing = if dx.abs() >= dy.abs() {
                    if dx > 0 { Direction::Right } else { Direction::Left }
                } else {
                    if dy > 0 { Direction::Down } else { Direction::Up }
                };
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
    }

    true
}

// ── Script Registry ───────────────────────────────────────────────────────────

/// Look up a script by ID and return its steps.
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

        SCRIPT_HOUSE1F_STOVE    => simple_text("CINNABAR VOLCANO BURGER!\nMake your Pokemon do a double take!"),
        SCRIPT_HOUSE1F_SINK     => simple_text("The sink is spotless."),
        SCRIPT_HOUSE1F_FRIDGE   => simple_text("FRESH WATER and LEMONADE."),
        SCRIPT_HOUSE1F_TV       => simple_text("A movie about two boys on a train."),
        SCRIPT_HOUSE2F_PC       => simple_text("Accessed own PC."),
        SCRIPT_HOUSE2F_RADIO    => simple_text("Welcome to POKEMON TALK."),
        SCRIPT_HOUSE2F_BOOKSHELF => simple_text("Picture book."),

        SCRIPT_LAB_HEALING_MACHINE => simple_text("A Pokemon healing machine. All HP restored!"),
        SCRIPT_LAB_BOOKSHELF    => simple_text("Research papers on Pokemon ecology."),
        SCRIPT_LAB_TRASHCAN     => simple_text("The wrapper from the snack ELM ate."),
        SCRIPT_LAB_WINDOW       => simple_text("The window is open."),
        SCRIPT_LAB_PC           => simple_text("ELM's research notes."),

        // Future sprint stubs — no-op
        SCRIPT_MEET_OFFICER | SCRIPT_AIDE_GIVES_POTION | SCRIPT_AIDE_GIVES_BALLS => {
            vec![ScriptStep::End]
        }

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
        ScriptStep::PlaySound(1), // SFX_GLASS_TING (email notification)
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
    let npc_idx = (choice + 2) as u8; // pokeball npcs are idx 2,3,4
    let left_steps = (choice + 1) as u8;

    vec![
        ScriptStep::ShowText(format!("It's {}! Will you take it?", name)),
        ScriptStep::YesNo { yes_jump: 2, no_jump: 12 }, // auto-jumps to yes
        // Yes path (idx 2):
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
        // GIMLI FIX: set to AIDE_GIVES_POTION (5), NOT NOOP (2)
        ScriptStep::SetScene { map: MapId::ElmsLab, scene_id: SCENE_ELMSLAB_AIDE_GIVES_POTION },
        ScriptStep::SetScene { map: MapId::NewBarkTown, scene_id: SCENE_NEWBARKTOWN_NOOP },
        ScriptStep::End,
        // No path (idx 12 — unreachable for Sprint 1, auto-yes):
        ScriptStep::End,
    ]
}

fn build_meet_mom_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::FacingPlayer { npc_idx: 0 },
        ScriptStep::ShowText("MOM: Oh, <PLAYER>! ...PROF.ELM was looking for you.".to_string()),
        ScriptStep::ShowText("MOM: Here, take this. It's the POKEGEAR!".to_string()),
        ScriptStep::GiveItem { item_id: 59, count: 1 }, // ITEM_POKEGEAR
        ScriptStep::SetEvent(EVENT_ENGINE_POKEGEAR),
        ScriptStep::ShowText("MOM: PROF.ELM is in his lab next door.".to_string()),
        ScriptStep::SetScene { map: MapId::PlayersHouse1F, scene_id: SCENE_PLAYERSHOUSE1F_NOOP },
        ScriptStep::SetEvent(EVENT_PLAYERS_HOUSE_MOM_1),
        // GIMLI FIX: ClearEvent MOM_2 (not SetEvent) so time-of-day Moms become visible
        ScriptStep::ClearEvent(EVENT_PLAYERS_HOUSE_MOM_2),
        ScriptStep::End,
    ]
}

fn build_teacher_stops_script(variant: u8) -> Vec<ScriptStep> {
    let right_steps = if variant == 1 { 4u8 } else { 5u8 };
    vec![
        ScriptStep::ShowText("Wait, <PLAYER>!".to_string()),
        ScriptStep::ShowText("What do you think you're doing?".to_string()),
        ScriptStep::ShowText("It's dangerous to go out without a #MON!\nWild #MON jump out of the grass on the way to the next town.".to_string()),
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
    fn test_script_show_text_then_end() {
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

        // Frame 1: ShowText sets text_buffer
        let running = step_script(&mut script, &mut player, &mut npc_states,
            &mut flags, &mut scenes, MapId::NewBarkTown,
            &mut party, &mut bag, false, false, false, false);
        assert!(running);
        assert!(script.text_buffer.is_some());

        // Frame 2: confirm advances past text
        let running = step_script(&mut script, &mut player, &mut npc_states,
            &mut flags, &mut scenes, MapId::NewBarkTown,
            &mut party, &mut bag, true, false, false, false);
        assert!(running); // text cleared, now at End

        // Frame 3: End step
        let running = step_script(&mut script, &mut player, &mut npc_states,
            &mut flags, &mut scenes, MapId::NewBarkTown,
            &mut party, &mut bag, false, false, false, false);
        assert!(!running, "Script should end after End step");
    }

    #[test]
    fn test_all_scripts_compile() {
        // Verify all script IDs produce non-empty step lists
        let ids = [1u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        for id in ids {
            let steps = get_script(id);
            assert!(!steps.is_empty(), "Script {} returned empty", id);
        }
    }
}
