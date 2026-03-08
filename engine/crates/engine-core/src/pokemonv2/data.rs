// AI-INSTRUCTIONS: pokemonv2/data.rs — Leaf module. No sibling imports.
// Defines all shared data types: enums, structs, Pokemon, species/move data.
// Lives at the bottom of the import graph. Everything else imports from here.

// --- Type Aliases ---

pub type SpeciesId = u16;
pub type MoveId = u16;

// --- Enums ---

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StatusCondition {
    None,
    Poison,
    Burn,
    Paralyze,
    Sleep(u8),
    Freeze,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GrowthRate {
    Fast,
    MediumFast,
    MediumSlow,
    Slow,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Emote bubble types. Defined here to avoid circular deps (used in events.rs + sprites.rs).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Emote {
    Shock,
    Question,
    Happy,
}

// --- Shared Runtime State Types ---
// Placed in data.rs to break circular dependency between events.rs and overworld.rs.

/// Player position and movement state.
#[derive(Clone, Debug)]
pub struct PlayerState {
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
    pub walk_offset: f64,
    pub is_walking: bool,
    pub walk_frame: u8,
    pub frame_timer: f64,
    pub name: String,
}

/// Mutable per-NPC state tracked at runtime (not in map definition).
#[derive(Clone, Debug)]
pub struct NpcState {
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
    pub walk_offset: f64,
    pub is_walking: bool,
    pub visible: bool,
    pub wander_timer: f64,
    /// Active emote bubble: (emote_type, frames_remaining)
    pub emote: Option<(Emote, u8)>,
}

/// Camera position for viewport centering.
#[derive(Clone, Debug)]
pub struct CameraState {
    pub x: f64,
    pub y: f64,
}

// --- Pokemon Instance ---

#[derive(Clone, Debug)]
pub struct Pokemon {
    pub species: SpeciesId,
    pub nickname: Option<String>,
    pub level: u8,
    pub hp: u16,
    pub max_hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
    pub sp_attack: u16,
    pub sp_defense: u16,
    pub moves: [Option<MoveId>; 4],
    pub move_pp: [u8; 4],
    pub move_max_pp: [u8; 4],
    pub status: StatusCondition,
    pub exp: u32,
    /// Gen 2 DVs: [Atk, Def, Spd, Spc] — each 0..15
    pub dvs: [u8; 4],
    /// Stat exp: [HP, Atk, Def, Spd, Spc] — each 0..65535
    pub evs: [u16; 5],
    pub held_item: Option<u8>,
}

impl Pokemon {
    /// Create a new Pokemon at the given level with default DVs/EVs and level-up moves.
    pub fn new(species: SpeciesId, level: u8) -> Self {
        let data = species_data(species);
        // Default DVs: all 8 (middle value)
        let dvs = [8u8; 4];
        // No EVs initially
        let evs = [0u16; 5];

        // Compute base stats
        let hp = calc_hp(data.base_hp, dvs[0], 0, level);
        let attack = calc_stat(data.base_attack, dvs[0], 0, level);
        let defense = calc_stat(data.base_defense, dvs[1], 0, level);
        let speed = calc_stat(data.base_speed, dvs[2], 0, level);
        let sp_attack = calc_stat(data.base_sp_attack, dvs[3], 0, level);
        let sp_defense = calc_stat(data.base_sp_defense, dvs[3], 0, level);

        // Collect learnset moves up to current level
        let mut moves = [None; 4];
        let mut pp = [0u8; 4];
        let mut max_pp = [0u8; 4];
        let mut slot = 0usize;
        for &(learn_level, move_id) in data.learnset {
            if learn_level <= level && slot < 4 {
                let md = move_data(move_id);
                moves[slot] = Some(move_id);
                pp[slot] = md.pp;
                max_pp[slot] = md.pp;
                slot += 1;
            }
        }

        Self {
            species,
            nickname: None,
            level,
            hp,
            max_hp: hp,
            attack,
            defense,
            speed,
            sp_attack,
            sp_defense,
            moves,
            move_pp: pp,
            move_max_pp: max_pp,
            status: StatusCondition::None,
            exp: 0,
            dvs,
            evs,
            held_item: None,
        }
    }

    /// Recalculate all stats from base stats, DVs, EVs, and level.
    pub fn recalc_stats(&mut self) {
        let data = species_data(self.species);
        self.max_hp = calc_hp(data.base_hp, self.dvs[0], self.evs[0], self.level);
        self.attack = calc_stat(data.base_attack, self.dvs[0], self.evs[1], self.level);
        self.defense = calc_stat(data.base_defense, self.dvs[1], self.evs[2], self.level);
        self.speed = calc_stat(data.base_speed, self.dvs[2], self.evs[3], self.level);
        self.sp_attack = calc_stat(data.base_sp_attack, self.dvs[3], self.evs[4], self.level);
        self.sp_defense = calc_stat(data.base_sp_defense, self.dvs[3], self.evs[4], self.level);
    }
}

/// Gen 2 HP formula: ((Base + DV) * 2 + sqrt(StatExp) / 4) * Level / 100 + Level + 10
fn calc_hp(base: u8, dv: u8, stat_exp: u16, level: u8) -> u16 {
    let stat_exp_bonus = (f64::sqrt(stat_exp as f64) / 4.0).floor() as u32;
    let val = ((base as u32 + dv as u32) * 2 + stat_exp_bonus) * level as u32 / 100
        + level as u32
        + 10;
    val as u16
}

/// Gen 2 stat formula: ((Base + DV) * 2 + sqrt(StatExp) / 4) * Level / 100 + 5
fn calc_stat(base: u8, dv: u8, stat_exp: u16, level: u8) -> u16 {
    let stat_exp_bonus = (f64::sqrt(stat_exp as f64) / 4.0).floor() as u32;
    let val = ((base as u32 + dv as u32) * 2 + stat_exp_bonus) * level as u32 / 100 + 5;
    val as u16
}

// --- Species Data ---

#[derive(Clone, Debug)]
pub struct SpeciesData {
    pub id: SpeciesId,
    pub name: &'static str,
    pub type1: PokemonType,
    pub type2: PokemonType,
    pub base_hp: u8,
    pub base_attack: u8,
    pub base_defense: u8,
    pub base_speed: u8,
    pub base_sp_attack: u8,
    pub base_sp_defense: u8,
    pub catch_rate: u8,
    pub base_exp: u8,
    pub growth_rate: GrowthRate,
    pub learnset: &'static [(u8, MoveId)],
}

// Learnsets (level, move_id) for Sprint 1 starters
static CHIKORITA_LEARNSET: &[(u8, MoveId)] = &[
    (1, MOVE_TACKLE),
    (1, MOVE_GROWL),
];

static CYNDAQUIL_LEARNSET: &[(u8, MoveId)] = &[
    (1, MOVE_TACKLE),
    (1, MOVE_LEER),
];

static TOTODILE_LEARNSET: &[(u8, MoveId)] = &[
    (1, MOVE_SCRATCH),
    (1, MOVE_LEER),
];

static CHIKORITA_DATA: SpeciesData = SpeciesData {
    id: CHIKORITA,
    name: "CHIKORITA",
    type1: PokemonType::Grass,
    type2: PokemonType::Grass,
    base_hp: 45,
    base_attack: 49,
    base_defense: 65,
    base_speed: 45,
    base_sp_attack: 49,
    base_sp_defense: 65,
    catch_rate: 45,
    base_exp: 64,
    growth_rate: GrowthRate::MediumSlow,
    learnset: CHIKORITA_LEARNSET,
};

static CYNDAQUIL_DATA: SpeciesData = SpeciesData {
    id: CYNDAQUIL,
    name: "CYNDAQUIL",
    type1: PokemonType::Fire,
    type2: PokemonType::Fire,
    base_hp: 39,
    base_attack: 52,
    base_defense: 43,
    base_speed: 65,
    base_sp_attack: 60,
    base_sp_defense: 50,
    catch_rate: 45,
    base_exp: 65,
    growth_rate: GrowthRate::MediumSlow,
    learnset: CYNDAQUIL_LEARNSET,
};

static TOTODILE_DATA: SpeciesData = SpeciesData {
    id: TOTODILE,
    name: "TOTODILE",
    type1: PokemonType::Water,
    type2: PokemonType::Water,
    base_hp: 50,
    base_attack: 65,
    base_defense: 64,
    base_speed: 43,
    base_sp_attack: 44,
    base_sp_defense: 48,
    catch_rate: 45,
    base_exp: 66,
    growth_rate: GrowthRate::MediumSlow,
    learnset: TOTODILE_LEARNSET,
};

/// Return species data for the given id. Returns Chikorita data for unknown species.
pub fn species_data(id: SpeciesId) -> &'static SpeciesData {
    match id {
        CHIKORITA => &CHIKORITA_DATA,
        CYNDAQUIL => &CYNDAQUIL_DATA,
        TOTODILE => &TOTODILE_DATA,
        _ => &CHIKORITA_DATA, // fallback
    }
}

// --- Move Data ---

#[derive(Clone, Debug)]
pub struct MoveData {
    pub id: MoveId,
    pub name: &'static str,
    pub move_type: PokemonType,
    pub power: u8,
    pub accuracy: u8,
    pub pp: u8,
    /// Gen 2: derived from move_type (Normal/Fighting/etc = physical, rest = special)
    pub is_special: bool,
}

static TACKLE_DATA: MoveData = MoveData {
    id: MOVE_TACKLE,
    name: "TACKLE",
    move_type: PokemonType::Normal,
    power: 35,
    accuracy: 95,
    pp: 35,
    is_special: false,
};

static GROWL_DATA: MoveData = MoveData {
    id: MOVE_GROWL,
    name: "GROWL",
    move_type: PokemonType::Normal,
    power: 0,
    accuracy: 100,
    pp: 40,
    is_special: false,
};

static LEER_DATA: MoveData = MoveData {
    id: MOVE_LEER,
    name: "LEER",
    move_type: PokemonType::Normal,
    power: 0,
    accuracy: 100,
    pp: 30,
    is_special: false,
};

static SCRATCH_DATA: MoveData = MoveData {
    id: MOVE_SCRATCH,
    name: "SCRATCH",
    move_type: PokemonType::Normal,
    power: 40,
    accuracy: 100,
    pp: 35,
    is_special: false,
};

/// Return move data for the given id. Returns Tackle data for unknown moves.
pub fn move_data(id: MoveId) -> &'static MoveData {
    match id {
        MOVE_TACKLE => &TACKLE_DATA,
        MOVE_GROWL => &GROWL_DATA,
        MOVE_LEER => &LEER_DATA,
        MOVE_SCRATCH => &SCRATCH_DATA,
        _ => &TACKLE_DATA, // fallback
    }
}

// --- Type Effectiveness ---

/// Gen 2 type effectiveness. Returns 0.0 (immune), 0.5, 1.0, or 2.0.
pub fn type_effectiveness(atk: PokemonType, def: PokemonType) -> f64 {
    use PokemonType::*;
    match (atk, def) {
        // Normal
        (Normal, Rock) | (Normal, Steel) => 0.5,
        (Normal, Ghost) => 0.0,
        // Fire
        (Fire, Fire) | (Fire, Water) | (Fire, Rock) | (Fire, Dragon) => 0.5,
        (Fire, Grass) | (Fire, Ice) | (Fire, Bug) | (Fire, Steel) => 2.0,
        // Water
        (Water, Water) | (Water, Grass) | (Water, Dragon) => 0.5,
        (Water, Fire) | (Water, Ground) | (Water, Rock) => 2.0,
        // Electric
        (Electric, Electric) | (Electric, Grass) | (Electric, Dragon) => 0.5,
        (Electric, Ground) => 0.0,
        (Electric, Water) | (Electric, Flying) => 2.0,
        // Grass
        (Grass, Fire) | (Grass, Grass) | (Grass, Poison) | (Grass, Flying)
        | (Grass, Bug) | (Grass, Dragon) | (Grass, Steel) => 0.5,
        (Grass, Water) | (Grass, Ground) | (Grass, Rock) => 2.0,
        // Ice
        (Ice, Water) | (Ice, Ice) => 0.5,
        (Ice, Grass) | (Ice, Ground) | (Ice, Flying) | (Ice, Dragon) => 2.0,
        // Fighting
        (Fighting, Poison) | (Fighting, Bug) | (Fighting, Psychic) | (Fighting, Flying) => 0.5,
        (Fighting, Ghost) => 0.0,
        (Fighting, Normal) | (Fighting, Ice) | (Fighting, Rock) | (Fighting, Dark)
        | (Fighting, Steel) => 2.0,
        // Poison
        (Poison, Poison) | (Poison, Ground) | (Poison, Rock) | (Poison, Ghost) => 0.5,
        (Poison, Steel) => 0.0,
        (Poison, Grass) => 2.0,
        // Ground
        (Ground, Grass) | (Ground, Bug) => 0.5,
        (Ground, Flying) => 0.0,
        (Ground, Fire) | (Ground, Electric) | (Ground, Poison) | (Ground, Rock)
        | (Ground, Steel) => 2.0,
        // Flying
        (Flying, Electric) | (Flying, Rock) | (Flying, Steel) => 0.5,
        (Flying, Grass) | (Flying, Fighting) | (Flying, Bug) => 2.0,
        // Psychic
        (Psychic, Psychic) | (Psychic, Steel) => 0.5,
        (Psychic, Dark) => 0.0,
        (Psychic, Fighting) | (Psychic, Poison) => 2.0,
        // Bug
        (Bug, Fire) | (Bug, Fighting) | (Bug, Flying) | (Bug, Ghost) | (Bug, Steel) => 0.5,
        (Bug, Grass) | (Bug, Psychic) | (Bug, Dark) => 2.0,
        // Rock
        (Rock, Fighting) | (Rock, Ground) | (Rock, Steel) => 0.5,
        (Rock, Fire) | (Rock, Ice) | (Rock, Flying) | (Rock, Bug) => 2.0,
        // Ghost
        (Ghost, Normal) | (Ghost, Psychic) => 0.0,
        (Ghost, Ghost) => 2.0,
        // Dragon
        (Dragon, Steel) => 0.5,
        (Dragon, Dragon) => 2.0,
        // Dark
        (Dark, Fighting) | (Dark, Dark) | (Dark, Steel) => 0.5,
        (Dark, Ghost) | (Dark, Psychic) => 2.0,
        // Steel
        (Steel, Fire) | (Steel, Water) | (Steel, Electric) | (Steel, Steel) => 0.5,
        (Steel, Ice) | (Steel, Rock) => 2.0,
        _ => 1.0,
    }
}

// --- Species ID Constants ---
pub const CHIKORITA: SpeciesId = 152;
pub const CYNDAQUIL: SpeciesId = 155;
pub const TOTODILE: SpeciesId = 158;

// --- Move ID Constants ---
pub const MOVE_TACKLE: MoveId = 33;
pub const MOVE_GROWL: MoveId = 45;
pub const MOVE_LEER: MoveId = 43;
pub const MOVE_SCRATCH: MoveId = 10;

// --- Item ID Constants ---
pub const ITEM_BERRY: u8 = 3;
pub const ITEM_POTION: u8 = 17;
pub const ITEM_POKEGEAR: u8 = 59;
