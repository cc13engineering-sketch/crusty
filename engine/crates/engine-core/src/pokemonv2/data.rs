// AI-INSTRUCTIONS: pokemonv2/data.rs — Leaf module. No sibling imports.
// Defines all shared data types: enums, structs, Pokemon, species/move data.
// Lives at the bottom of the import graph. Everything else imports from here.
// Sprint 6: All 251 species moved to species_data.rs. Re-exported via pub use.
// Sprint 7: All 251 moves moved to move_data.rs. Re-exported via pub use.
//           MoveData struct extended with effect_id, effect_chance, priority, high_crit.

// Re-export all 251 species data from species_data.rs
pub use super::species_data::{species_data, NUM_SPECIES};
pub use super::species_data::{
    BULBASAUR, IVYSAUR, VENUSAUR, CHARMANDER, CHARMELEON, CHARIZARD,
    SQUIRTLE, WARTORTLE, BLASTOISE, CATERPIE, METAPOD, BUTTERFREE,
    WEEDLE, KAKUNA, BEEDRILL, PIDGEY, PIDGEOTTO, PIDGEOT,
    RATTATA, RATICATE, SPEAROW, FEAROW, EKANS, ARBOK,
    PIKACHU, RAICHU, SANDSHREW, SANDSLASH, NIDORAN_F, NIDORINA,
    NIDOQUEEN, NIDORAN_M, NIDORINO, NIDOKING, CLEFAIRY, CLEFABLE,
    VULPIX, NINETALES, JIGGLYPUFF, WIGGLYTUFF, ZUBAT, GOLBAT,
    ODDISH, GLOOM, VILEPLUME, PARAS, PARASECT, VENONAT, VENOMOTH,
    DIGLETT, DUGTRIO, MEOWTH, PERSIAN, PSYDUCK, GOLDUCK,
    MANKEY, PRIMEAPE, GROWLITHE, ARCANINE, POLIWAG, POLIWHIRL,
    POLIWRATH, ABRA, KADABRA, ALAKAZAM, MACHOP, MACHOKE, MACHAMP,
    BELLSPROUT, WEEPINBELL, VICTREEBEL, TENTACOOL, TENTACRUEL,
    GEODUDE, GRAVELER, GOLEM, PONYTA, RAPIDASH, SLOWPOKE, SLOWBRO,
    MAGNEMITE, MAGNETON, FARFETCH_D, DODUO, DODRIO, SEEL, DEWGONG,
    GRIMER, MUK, SHELLDER, CLOYSTER, GASTLY, HAUNTER, GENGAR,
    ONIX, DROWZEE, HYPNO, KRABBY, KINGLER, VOLTORB, ELECTRODE,
    EXEGGCUTE, EXEGGUTOR, CUBONE, MAROWAK, HITMONLEE, HITMONCHAN,
    LICKITUNG, KOFFING, WEEZING, RHYHORN, RHYDON, CHANSEY, TANGELA,
    KANGASKHAN, HORSEA, SEADRA, GOLDEEN, SEAKING, STARYU, STARMIE,
    MR_MIME, SCYTHER, JYNX, ELECTABUZZ, MAGMAR, PINSIR, TAUROS,
    MAGIKARP, GYARADOS, LAPRAS, DITTO, EEVEE, VAPOREON, JOLTEON,
    FLAREON, PORYGON, OMANYTE, OMASTAR, KABUTO, KABUTOPS, AERODACTYL,
    SNORLAX, ARTICUNO, ZAPDOS, MOLTRES, DRATINI, DRAGONAIR, DRAGONITE,
    MEWTWO, MEW, CHIKORITA, BAYLEEF, MEGANIUM, CYNDAQUIL, QUILAVA,
    TYPHLOSION, TOTODILE, CROCONAW, FERALIGATR, SENTRET, FURRET,
    HOOTHOOT, NOCTOWL, LEDYBA, LEDIAN, SPINARAK, ARIADOS, CROBAT,
    CHINCHOU, LANTURN, PICHU, CLEFFA, IGGLYBUFF, TOGEPI, TOGETIC,
    NATU, XATU, MAREEP, FLAAFFY, AMPHAROS, BELLOSSOM, MARILL, AZUMARILL,
    SUDOWOODO, POLITOED, HOPPIP, SKIPLOOM, JUMPLUFF, AIPOM, SUNKERN,
    SUNFLORA, YANMA, WOOPER, QUAGSIRE, ESPEON, UMBREON, MURKROW,
    SLOWKING, MISDREAVUS, UNOWN, WOBBUFFET, GIRAFARIG, PINECO, FORRETRESS,
    DUNSPARCE, GLIGAR, STEELIX, SNUBBULL, GRANBULL, QWILFISH, SCIZOR,
    SHUCKLE, HERACROSS, SNEASEL, TEDDIURSA, URSARING, SLUGMA, MAGCARGO,
    SWINUB, PILOSWINE, CORSOLA, REMORAID, OCTILLERY, DELIBIRD, MANTINE,
    SKARMORY, HOUNDOUR, HOUNDOOM, KINGDRA, PHANPY, DONPHAN, PORYGON2,
    STANTLER, SMEARGLE, TYROGUE, HITMONTOP, SMOOCHUM, ELEKID, MAGBY,
    MILTANK, BLISSEY, RAIKOU, ENTEI, SUICUNE, LARVITAR, PUPITAR,
    TYRANITAR, LUGIA, HO_OH, CELEBI,
};

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
    pub step_frame: u32,
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
    pub walk_frame: u8,
    pub step_frame: u32,
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
    /// Effect ID (maps to EFFECT_* constants in move_data.rs)
    pub effect_id: u8,
    /// Secondary effect chance (0-100 percentage)
    pub effect_chance: u8,
    /// Move priority (-1 to +3, default 1)
    pub priority: i8,
    /// True for the 7 high-crit-ratio moves
    pub high_crit: bool,
}

// Sprint 7: All move data moved to move_data.rs. Re-exported below.

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

// --- Move data re-exports from move_data.rs (Sprint 7) ---
pub use super::move_data::{move_data, NUM_MOVES};
pub use super::move_data::{
    MOVE_POUND, MOVE_KARATE_CHOP, MOVE_DOUBLESLAP, MOVE_COMET_PUNCH, MOVE_MEGA_PUNCH,
    MOVE_PAY_DAY, MOVE_FIRE_PUNCH, MOVE_ICE_PUNCH, MOVE_THUNDERPUNCH, MOVE_SCRATCH,
    MOVE_VICEGRIP, MOVE_GUILLOTINE, MOVE_RAZOR_WIND, MOVE_SWORDS_DANCE, MOVE_CUT,
    MOVE_GUST, MOVE_WING_ATTACK, MOVE_WHIRLWIND, MOVE_FLY, MOVE_BIND,
    MOVE_SLAM, MOVE_VINE_WHIP, MOVE_STOMP, MOVE_DOUBLE_KICK, MOVE_MEGA_KICK,
    MOVE_JUMP_KICK, MOVE_ROLLING_KICK, MOVE_SAND_ATTACK, MOVE_HEADBUTT, MOVE_HORN_ATTACK,
    MOVE_FURY_ATTACK, MOVE_HORN_DRILL, MOVE_TACKLE, MOVE_BODY_SLAM, MOVE_WRAP,
    MOVE_TAKE_DOWN, MOVE_THRASH, MOVE_DOUBLE_EDGE, MOVE_TAIL_WHIP, MOVE_POISON_STING,
    MOVE_TWINEEDLE, MOVE_PIN_MISSILE, MOVE_LEER, MOVE_BITE, MOVE_GROWL,
    MOVE_ROAR, MOVE_SING, MOVE_SUPERSONIC, MOVE_SONICBOOM, MOVE_DISABLE,
    MOVE_ACID, MOVE_EMBER, MOVE_FLAMETHROWER, MOVE_MIST, MOVE_WATER_GUN,
    MOVE_HYDRO_PUMP, MOVE_SURF, MOVE_ICE_BEAM, MOVE_BLIZZARD, MOVE_PSYBEAM,
    MOVE_BUBBLEBEAM, MOVE_AURORA_BEAM, MOVE_HYPER_BEAM, MOVE_PECK, MOVE_DRILL_PECK,
    MOVE_SUBMISSION, MOVE_LOW_KICK, MOVE_COUNTER, MOVE_SEISMIC_TOSS, MOVE_STRENGTH,
    MOVE_ABSORB, MOVE_MEGA_DRAIN, MOVE_LEECH_SEED, MOVE_GROWTH, MOVE_RAZOR_LEAF,
    MOVE_SOLARBEAM, MOVE_POISONPOWDER, MOVE_STUN_SPORE, MOVE_SLEEP_POWDER, MOVE_PETAL_DANCE,
    MOVE_STRING_SHOT, MOVE_DRAGON_RAGE, MOVE_FIRE_SPIN, MOVE_THUNDERSHOCK, MOVE_THUNDERBOLT,
    MOVE_THUNDER_WAVE, MOVE_THUNDER, MOVE_ROCK_THROW, MOVE_EARTHQUAKE, MOVE_FISSURE,
    MOVE_DIG, MOVE_TOXIC, MOVE_CONFUSION, MOVE_PSYCHIC_M, MOVE_HYPNOSIS,
    MOVE_MEDITATE, MOVE_AGILITY, MOVE_QUICK_ATTACK, MOVE_RAGE, MOVE_TELEPORT,
    MOVE_NIGHT_SHADE, MOVE_MIMIC, MOVE_SCREECH, MOVE_DOUBLE_TEAM, MOVE_RECOVER,
    MOVE_HARDEN, MOVE_MINIMIZE, MOVE_SMOKESCREEN, MOVE_CONFUSE_RAY, MOVE_WITHDRAW,
    MOVE_DEFENSE_CURL, MOVE_BARRIER, MOVE_LIGHT_SCREEN, MOVE_HAZE, MOVE_REFLECT,
    MOVE_FOCUS_ENERGY, MOVE_BIDE, MOVE_METRONOME, MOVE_MIRROR_MOVE, MOVE_SELFDESTRUCT,
    MOVE_EGG_BOMB, MOVE_LICK, MOVE_SMOG, MOVE_SLUDGE, MOVE_BONE_CLUB,
    MOVE_FIRE_BLAST, MOVE_WATERFALL, MOVE_CLAMP, MOVE_SWIFT, MOVE_SKULL_BASH,
    MOVE_SPIKE_CANNON, MOVE_CONSTRICT, MOVE_AMNESIA, MOVE_KINESIS, MOVE_SOFTBOILED,
    MOVE_HI_JUMP_KICK, MOVE_GLARE, MOVE_DREAM_EATER, MOVE_POISON_GAS, MOVE_BARRAGE,
    MOVE_LEECH_LIFE, MOVE_LOVELY_KISS, MOVE_SKY_ATTACK, MOVE_TRANSFORM, MOVE_BUBBLE,
    MOVE_DIZZY_PUNCH, MOVE_SPORE, MOVE_FLASH, MOVE_PSYWAVE, MOVE_SPLASH,
    MOVE_ACID_ARMOR, MOVE_CRABHAMMER, MOVE_EXPLOSION, MOVE_FURY_SWIPES, MOVE_BONEMERANG,
    MOVE_REST, MOVE_ROCK_SLIDE, MOVE_HYPER_FANG, MOVE_SHARPEN, MOVE_CONVERSION,
    MOVE_TRI_ATTACK, MOVE_SUPER_FANG, MOVE_SLASH, MOVE_SUBSTITUTE, MOVE_STRUGGLE,
    MOVE_SKETCH, MOVE_TRIPLE_KICK, MOVE_THIEF, MOVE_SPIDER_WEB, MOVE_MIND_READER,
    MOVE_NIGHTMARE, MOVE_FLAME_WHEEL, MOVE_SNORE, MOVE_CURSE, MOVE_FLAIL,
    MOVE_CONVERSION2, MOVE_AEROBLAST, MOVE_COTTON_SPORE, MOVE_REVERSAL, MOVE_SPITE,
    MOVE_POWDER_SNOW, MOVE_PROTECT, MOVE_MACH_PUNCH, MOVE_SCARY_FACE, MOVE_FAINT_ATTACK,
    MOVE_SWEET_KISS, MOVE_BELLY_DRUM, MOVE_SLUDGE_BOMB, MOVE_MUD_SLAP, MOVE_OCTAZOOKA,
    MOVE_SPIKES, MOVE_ZAP_CANNON, MOVE_FORESIGHT, MOVE_DESTINY_BOND, MOVE_PERISH_SONG,
    MOVE_ICY_WIND, MOVE_DETECT, MOVE_BONE_RUSH, MOVE_LOCK_ON, MOVE_OUTRAGE,
    MOVE_SANDSTORM, MOVE_GIGA_DRAIN, MOVE_ENDURE, MOVE_CHARM, MOVE_ROLLOUT,
    MOVE_FALSE_SWIPE, MOVE_SWAGGER, MOVE_MILK_DRINK, MOVE_SPARK, MOVE_FURY_CUTTER,
    MOVE_STEEL_WING, MOVE_MEAN_LOOK, MOVE_ATTRACT, MOVE_SLEEP_TALK, MOVE_HEAL_BELL,
    MOVE_RETURN, MOVE_PRESENT, MOVE_FRUSTRATION, MOVE_SAFEGUARD, MOVE_PAIN_SPLIT,
    MOVE_SACRED_FIRE, MOVE_MAGNITUDE, MOVE_DYNAMICPUNCH, MOVE_MEGAHORN, MOVE_DRAGONBREATH,
    MOVE_BATON_PASS, MOVE_ENCORE, MOVE_PURSUIT, MOVE_RAPID_SPIN, MOVE_SWEET_SCENT,
    MOVE_IRON_TAIL, MOVE_METAL_CLAW, MOVE_VITAL_THROW, MOVE_MORNING_SUN, MOVE_SYNTHESIS,
    MOVE_MOONLIGHT, MOVE_HIDDEN_POWER, MOVE_CROSS_CHOP, MOVE_TWISTER, MOVE_RAIN_DANCE,
    MOVE_SUNNY_DAY, MOVE_CRUNCH, MOVE_MIRROR_COAT, MOVE_PSYCH_UP, MOVE_EXTREMESPEED,
    MOVE_ANCIENTPOWER, MOVE_SHADOW_BALL, MOVE_FUTURE_SIGHT, MOVE_ROCK_SMASH,
    MOVE_WHIRLPOOL, MOVE_BEAT_UP,
};

// --- Item ID Constants ---
pub const ITEM_BERRY: u8 = 3;
pub const ITEM_POKE_BALL: u8 = 5;
pub const ITEM_ANTIDOTE: u8 = 9;
pub const ITEM_AWAKENING: u8 = 12;
pub const ITEM_PARLYZ_HEAL: u8 = 13;
pub const ITEM_POTION: u8 = 17;
pub const ITEM_POKEGEAR: u8 = 59;
pub const ITEM_MYSTIC_WATER: u8 = 95;
pub const ITEM_PINK_BOW: u8 = 104;
pub const ITEM_MYSTERY_EGG: u8 = 130;  // key item, not consumed
// --- Item ID Constants (Sprint 5 additions) ---
pub const ITEM_PP_UP: u8 = 48;
pub const ITEM_RARE_CANDY: u8 = 43;
pub const ITEM_PRZ_CURE_BERRY: u8 = 54;
pub const ITEM_HYPER_POTION: u8 = 26;
// NOTE: MAP_CARD is NOT a bag item. It is tracked via EVENT_ENGINE_MAP_CARD flag only.

// Move ID constants now re-exported from move_data.rs above.

// --- Battle Enums ---

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BattleType {
    Wild,      // standard wild encounter
    Tutorial,  // BATTLETYPE_TUTORIAL -- catching demo, auto-catch
    CanLose,   // BATTLETYPE_CANLOSE -- rival, no game-over on loss
    Normal,    // standard trainer battle
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BattleResult {
    Won,
    Lost,
    Fled,
    Caught,
}

// --- Time Of Day ---

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TimeOfDay {
    Morning, // 04:00 - 09:59
    Day,     // 10:00 - 17:59
    Night,   // 18:00 - 03:59
}

/// Derive time of day from total elapsed game time.
/// 1 real second = 1 game minute (accelerated clock).
pub fn get_time_of_day(total_time: f64) -> TimeOfDay {
    let game_minutes = (total_time * 60.0) as u32 % (24 * 60);
    let hour = game_minutes / 60;
    match hour {
        4..=9 => TimeOfDay::Morning,
        10..=17 => TimeOfDay::Day,
        _ => TimeOfDay::Night,
    }
}

// --- Music Constants ---
pub const MUSIC_SHOW_ME_AROUND: u8 = 10;
pub const MUSIC_RIVAL_ENCOUNTER: u8 = 11;
pub const MUSIC_RIVAL_AFTER: u8 = 12;
pub const MUSIC_PROF_OAK: u8 = 13;
pub const MUSIC_JOHTO_TRAINER_BATTLE: u8 = 14;
pub const MUSIC_VIOLET_CITY: u8 = 15;
pub const MUSIC_ROUTE_31: u8 = 16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_species_data() {
        let species = [CATERPIE, METAPOD, WEEDLE, ZUBAT, POLIWAG, LEDYBA, SPINARAK];
        for &sp in &species {
            let data = species_data(sp);
            assert!(data.base_hp > 0, "Species {} should have non-zero HP", sp);
            assert!(data.base_attack > 0, "Species {} should have non-zero Attack", sp);
            assert!(!data.learnset.is_empty(), "Species {} should have a learnset", sp);
        }
    }

    #[test]
    fn test_caterpie_data_accuracy() {
        let data = species_data(CATERPIE);
        assert_eq!(data.name, "CATERPIE");
        assert_eq!(data.type1, PokemonType::Bug);
        assert_eq!(data.base_hp, 45);
        assert_eq!(data.base_attack, 30);
        assert_eq!(data.base_defense, 35);
        assert_eq!(data.base_speed, 45);
        assert_eq!(data.catch_rate, 255);
        assert!(matches!(data.growth_rate, GrowthRate::MediumFast));
    }

    #[test]
    fn test_new_moves_data() {
        let moves = [MOVE_STRING_SHOT, MOVE_POISON_STING, MOVE_HARDEN,
                     MOVE_LEECH_LIFE, MOVE_CONSTRICT, MOVE_BUBBLE, MOVE_SUPERSONIC];
        for &mv in &moves {
            let data = move_data(mv);
            assert!(!data.name.is_empty(), "Move {} should have a name", mv);
            assert!(data.pp > 0, "Move {} should have non-zero PP", mv);
        }
    }

    #[test]
    fn test_bellsprout_data() {
        let data = species_data(BELLSPROUT);
        assert_eq!(data.name, "BELLSPROUT");
        assert_eq!(data.type1, PokemonType::Grass);
        assert_eq!(data.type2, PokemonType::Poison);
        assert_eq!(data.base_hp, 50);
        assert_eq!(data.base_attack, 75);
        assert_eq!(data.base_sp_attack, 70);
        assert_eq!(data.catch_rate, 255);
        assert!(matches!(data.growth_rate, GrowthRate::MediumSlow));
    }

    #[test]
    fn test_gastly_data() {
        let data = species_data(GASTLY);
        assert_eq!(data.name, "GASTLY");
        assert_eq!(data.type1, PokemonType::Ghost);
        assert_eq!(data.type2, PokemonType::Poison);
        assert_eq!(data.base_hp, 30);
        assert_eq!(data.base_sp_attack, 100);
        assert_eq!(data.base_speed, 80);
        assert_eq!(data.catch_rate, 190);
    }

    #[test]
    fn test_bellsprout_learnset_at_level5() {
        let poke = Pokemon::new(BELLSPROUT, 5);
        let known: Vec<_> = poke.moves.iter().filter_map(|&m| m).collect();
        assert_eq!(known.len(), 1, "Level 5 Bellsprout should know only VineWhip");
        assert!(known.contains(&MOVE_VINE_WHIP));
    }

    #[test]
    fn test_gastly_learnset_at_level5() {
        let poke = Pokemon::new(GASTLY, 5);
        let known: Vec<_> = poke.moves.iter().filter_map(|&m| m).collect();
        assert_eq!(known.len(), 2, "Level 5 Gastly should know Hypnosis + Lick");
        assert!(known.contains(&MOVE_HYPNOSIS));
        assert!(known.contains(&MOVE_LICK));
    }

    #[test]
    fn test_vine_whip_is_special() {
        let data = move_data(MOVE_VINE_WHIP);
        assert!(data.is_special, "Vine Whip should be a special move (Grass type in Gen 2)");
        assert_eq!(data.power, 35);
        assert_eq!(data.move_type, PokemonType::Grass);
    }

    #[test]
    fn test_sprint5_moves_data() {
        let moves = [MOVE_VINE_WHIP, MOVE_HYPNOSIS, MOVE_LICK, MOVE_GROWTH];
        for &mv in &moves {
            let data = move_data(mv);
            assert!(!data.name.is_empty(), "Move {} should have a name", mv);
            assert!(data.pp > 0, "Move {} should have non-zero PP", mv);
        }
    }

    #[test]
    fn test_caterpie_learnset_at_level3() {
        let poke = Pokemon::new(CATERPIE, 3);
        // Caterpie at level 3 should know Tackle (lv1) + StringShot (lv1)
        let known: Vec<_> = poke.moves.iter().filter_map(|&m| m).collect();
        assert_eq!(known.len(), 2, "Level 3 Caterpie should know 2 moves");
        assert!(known.contains(&MOVE_TACKLE));
        assert!(known.contains(&MOVE_STRING_SHOT));
    }

    #[test]
    fn test_poliwag_learnset_at_level4() {
        let poke = Pokemon::new(POLIWAG, 4);
        // Poliwag at level 4 only knows Bubble (lv1)
        let known: Vec<_> = poke.moves.iter().filter_map(|&m| m).collect();
        assert_eq!(known.len(), 1, "Level 4 Poliwag should know only Bubble");
        assert!(known.contains(&MOVE_BUBBLE));
    }

    #[test]
    fn test_zubat_learnset_at_level3() {
        let poke = Pokemon::new(ZUBAT, 3);
        // Zubat at level 3 only knows LeechLife (lv1)
        let known: Vec<_> = poke.moves.iter().filter_map(|&m| m).collect();
        assert_eq!(known.len(), 1, "Level 3 Zubat should know only LeechLife");
        assert!(known.contains(&MOVE_LEECH_LIFE));
    }

    #[test]
    fn test_spinarak_learnset_at_level3() {
        let poke = Pokemon::new(SPINARAK, 3);
        // Spinarak at level 3 knows PoisonSting (lv1) + StringShot (lv1)
        let known: Vec<_> = poke.moves.iter().filter_map(|&m| m).collect();
        assert_eq!(known.len(), 2, "Level 3 Spinarak should know 2 moves");
        assert!(known.contains(&MOVE_POISON_STING));
        assert!(known.contains(&MOVE_STRING_SHOT));
    }

    #[test]
    fn test_bubble_is_special() {
        let data = move_data(MOVE_BUBBLE);
        assert!(data.is_special, "Bubble should be a special move");
        assert_eq!(data.power, 20);
        assert_eq!(data.move_type, PokemonType::Water);
    }

    // --- Sprint 7 Tests: All 251 moves ---

    #[test]
    fn test_all_251_moves_have_valid_data() {
        for id in 1..=251u16 {
            let d = move_data(id);
            assert_eq!(d.id, id, "Move {} has wrong id field", id);
            assert!(!d.name.is_empty(), "Move {} has empty name", id);
            assert!(d.pp > 0 && d.pp <= 40, "Move {} has invalid PP: {}", id, d.pp);
            assert!(d.accuracy > 0 && d.accuracy <= 100, "Move {} has invalid accuracy: {}", id, d.accuracy);
            assert!(d.priority >= 0 && d.priority <= 3, "Move {} has invalid priority: {}", id, d.priority);
        }
    }

    #[test]
    fn test_tackle_matches_pokecrystal() {
        let d = move_data(MOVE_TACKLE);
        assert_eq!(d.id, 33);
        assert_eq!(d.name, "TACKLE");
        assert_eq!(d.move_type, PokemonType::Normal);
        assert_eq!(d.power, 35);
        assert_eq!(d.accuracy, 95);
        assert_eq!(d.pp, 35);
        assert!(!d.is_special);
        assert_eq!(d.effect_chance, 0);
        assert_eq!(d.priority, 1);
        assert!(!d.high_crit);
    }

    #[test]
    fn test_fire_punch_matches_pokecrystal() {
        let d = move_data(MOVE_FIRE_PUNCH);
        assert_eq!(d.name, "FIRE PUNCH");
        assert_eq!(d.move_type, PokemonType::Fire);
        assert_eq!(d.power, 75);
        assert!(d.is_special); // Fire is special in Gen 2
        assert_eq!(d.effect_chance, 10);
        assert_eq!(d.priority, 1);
    }

    #[test]
    fn test_surf_matches_pokecrystal() {
        let d = move_data(MOVE_SURF);
        assert_eq!(d.name, "SURF");
        assert_eq!(d.move_type, PokemonType::Water);
        assert_eq!(d.power, 95);
        assert_eq!(d.accuracy, 100);
        assert_eq!(d.pp, 15);
        assert!(d.is_special);
    }

    #[test]
    fn test_blizzard_matches_pokecrystal() {
        let d = move_data(MOVE_BLIZZARD);
        assert_eq!(d.name, "BLIZZARD");
        assert_eq!(d.move_type, PokemonType::Ice);
        assert_eq!(d.power, 120);
        assert_eq!(d.accuracy, 70);
        assert_eq!(d.pp, 5);
        assert_eq!(d.effect_chance, 10); // 10% freeze
    }

    #[test]
    fn test_earthquake_matches_pokecrystal() {
        let d = move_data(MOVE_EARTHQUAKE);
        assert_eq!(d.name, "EARTHQUAKE");
        assert_eq!(d.move_type, PokemonType::Ground);
        assert_eq!(d.power, 100);
        assert_eq!(d.accuracy, 100);
        assert_eq!(d.pp, 10);
        assert!(!d.is_special); // Ground is physical
    }

    #[test]
    fn test_psychic_matches_pokecrystal() {
        let d = move_data(MOVE_PSYCHIC_M);
        assert_eq!(d.name, "PSYCHIC");
        assert_eq!(d.move_type, PokemonType::Psychic);
        assert_eq!(d.power, 90);
        assert_eq!(d.accuracy, 100);
        assert_eq!(d.pp, 10);
        assert_eq!(d.effect_chance, 10); // 10% SpDef drop
        assert!(d.is_special);
    }

    #[test]
    fn test_quick_attack_has_priority_2() {
        let d = move_data(MOVE_QUICK_ATTACK);
        assert_eq!(d.name, "QUICK ATTACK");
        assert_eq!(d.priority, 2);
    }

    #[test]
    fn test_protect_has_priority_3() {
        let d = move_data(MOVE_PROTECT);
        assert_eq!(d.name, "PROTECT");
        assert_eq!(d.priority, 3);
    }

    #[test]
    fn test_counter_has_priority_0() {
        let d = move_data(MOVE_COUNTER);
        assert_eq!(d.name, "COUNTER");
        assert_eq!(d.priority, 0);
    }

    #[test]
    fn test_extremespeed_has_priority_2() {
        let d = move_data(MOVE_EXTREMESPEED);
        assert_eq!(d.name, "EXTREMESPEED");
        assert_eq!(d.priority, 2);
        assert_eq!(d.power, 80);
    }

    #[test]
    fn test_slash_has_high_crit() {
        let d = move_data(MOVE_SLASH);
        assert_eq!(d.name, "SLASH");
        assert!(d.high_crit);
    }

    #[test]
    fn test_exactly_7_high_crit_moves() {
        let count = (1..=251u16).filter(|&id| move_data(id).high_crit).count();
        assert_eq!(count, 7, "Should have exactly 7 high-crit moves");
    }

    #[test]
    fn test_curse_uses_ghost_type() {
        let d = move_data(MOVE_CURSE);
        assert_eq!(d.name, "CURSE");
        assert_eq!(d.move_type, PokemonType::Ghost);
        assert_eq!(d.power, 0);
    }

    #[test]
    fn test_move_data_fallback_for_invalid_id() {
        let d0 = move_data(0);
        let d999 = move_data(999);
        assert_eq!(d0.id, 33, "Invalid id 0 should return Tackle");
        assert_eq!(d999.id, 33, "Invalid id 999 should return Tackle");
    }

    #[test]
    fn test_hyper_beam_matches_pokecrystal() {
        let d = move_data(MOVE_HYPER_BEAM);
        assert_eq!(d.name, "HYPER BEAM");
        assert_eq!(d.power, 150);
        assert_eq!(d.accuracy, 90);
        assert_eq!(d.pp, 5);
    }

    #[test]
    fn test_swords_dance_is_status() {
        let d = move_data(MOVE_SWORDS_DANCE);
        assert_eq!(d.name, "SWORDS DANCE");
        assert_eq!(d.power, 0);
        assert_eq!(d.pp, 30);
    }

    #[test]
    fn test_thunderbolt_matches_pokecrystal() {
        let d = move_data(MOVE_THUNDERBOLT);
        assert_eq!(d.name, "THUNDERBOLT");
        assert_eq!(d.move_type, PokemonType::Electric);
        assert_eq!(d.power, 95);
        assert_eq!(d.accuracy, 100);
        assert_eq!(d.pp, 15);
        assert_eq!(d.effect_chance, 10); // 10% para
        assert!(d.is_special);
    }

    #[test]
    fn test_shadow_ball_matches_pokecrystal() {
        let d = move_data(MOVE_SHADOW_BALL);
        assert_eq!(d.name, "SHADOW BALL");
        assert_eq!(d.move_type, PokemonType::Ghost);
        assert_eq!(d.power, 80);
        assert_eq!(d.accuracy, 100);
        assert_eq!(d.pp, 15);
        assert_eq!(d.effect_chance, 20);
    }

    #[test]
    fn test_priority_distribution() {
        let p0 = (1..=251u16).filter(|&id| move_data(id).priority == 0).count();
        let p2 = (1..=251u16).filter(|&id| move_data(id).priority == 2).count();
        let p3 = (1..=251u16).filter(|&id| move_data(id).priority == 3).count();
        // FORCE_SWITCH: Whirlwind(18), Roar(46) = 2
        // COUNTER: Counter(68) = 1
        // MIRROR_COAT: Mirror Coat(243) = 1
        assert_eq!(p0, 4, "4 moves with priority 0");
        // PRIORITY_HIT: Quick Attack(98), Mach Punch(183), ExtremeSpeed(245) = 3
        assert_eq!(p2, 3, "3 moves with priority 2");
        // PROTECT: Protect(182), Detect(197) = 2
        // ENDURE: Endure(203) = 1
        assert_eq!(p3, 3, "3 moves with priority 3");
    }
}
