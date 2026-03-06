// AI-INSTRUCTIONS: Pokemon data module. Contains all species data, move data, type effectiveness
// chart, and level-up/growth rate calculations for Pokemon Gold/Silver/Crystal recreation.
// Types: SpeciesId (u16), MoveId (u16). All stats follow Gen 2 formulas.

/// Pokemon elemental types (Gen 2)
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    Grass,
    Electric,
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

/// Move damage category
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MoveCategory {
    Physical,
    Special,
    Status,
}

/// Experience growth rate
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GrowthRate {
    Fast,
    MediumFast,
    MediumSlow,
    Slow,
}

/// Status conditions (Gen 2)
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StatusCondition {
    None,
    Poison,
    Burn,
    Paralysis,
    Sleep { turns: u8 },
    Freeze,
}

/// Species identification
pub type SpeciesId = u16;
/// Move identification
pub type MoveId = u16;

// ─── Species IDs ────────────────────────────────────────
pub const CHIKORITA: SpeciesId = 152;
pub const BAYLEEF: SpeciesId = 153;
pub const CYNDAQUIL: SpeciesId = 155;
pub const QUILAVA: SpeciesId = 156;
pub const TOTODILE: SpeciesId = 158;
pub const CROCONAW: SpeciesId = 159;
pub const PIDGEY: SpeciesId = 16;
pub const RATTATA: SpeciesId = 19;
pub const SENTRET: SpeciesId = 161;
pub const HOOTHOOT: SpeciesId = 163;
pub const CATERPIE: SpeciesId = 10;
pub const WEEDLE: SpeciesId = 13;
pub const GEODUDE: SpeciesId = 74;
pub const ZUBAT: SpeciesId = 41;
pub const BELLSPROUT: SpeciesId = 69;
pub const GASTLY: SpeciesId = 92;
pub const ONIX: SpeciesId = 95;
pub const MAGIKARP: SpeciesId = 129;
pub const LEDYBA: SpeciesId = 165;
pub const SPINARAK: SpeciesId = 167;
pub const MAREEP: SpeciesId = 179;
pub const WOOPER: SpeciesId = 194;
pub const HOPPIP: SpeciesId = 187;
pub const ODDISH: SpeciesId = 43;
pub const DROWZEE: SpeciesId = 96;
pub const ABRA: SpeciesId = 63;
pub const DITTO: SpeciesId = 132;
pub const PARAS: SpeciesId = 46;
pub const CLEFAIRY: SpeciesId = 35;
pub const MILTANK: SpeciesId = 241;
pub const SNUBBULL: SpeciesId = 209;
pub const JIGGLYPUFF: SpeciesId = 39;
pub const MEOWTH: SpeciesId = 52;
pub const NIDORAN_F: SpeciesId = 29;
pub const NIDORAN_M: SpeciesId = 32;
pub const GROWLITHE: SpeciesId = 58;
pub const VULPIX: SpeciesId = 37;
pub const STANTLER: SpeciesId = 234;
pub const YANMA: SpeciesId = 193;
pub const VENONAT: SpeciesId = 48;
pub const HAUNTER: SpeciesId = 93;
pub const GENGAR: SpeciesId = 94;
pub const SUDOWOODO: SpeciesId = 185;
pub const KOFFING: SpeciesId = 109;
pub const RATICATE: SpeciesId = 20;
pub const MAGMAR: SpeciesId = 126;
pub const EEVEE: SpeciesId = 133;
pub const VAPOREON: SpeciesId = 134;
pub const JOLTEON: SpeciesId = 135;
pub const FLAREON: SpeciesId = 136;
pub const ESPEON: SpeciesId = 196;
pub const UMBREON: SpeciesId = 197;
// ─── Route 38-39 / Olivine species ──────────────────────
pub const PIDGEOTTO: SpeciesId = 17;
pub const NOCTOWL: SpeciesId = 164;
pub const FARFETCHD: SpeciesId = 83;
pub const TAUROS: SpeciesId = 128;
pub const MAGNEMITE: SpeciesId = 81;
pub const DODUO: SpeciesId = 84;
pub const FLAAFFY: SpeciesId = 180;
pub const PSYDUCK: SpeciesId = 54;
pub const MR_MIME: SpeciesId = 122;
pub const SKIPLOOM: SpeciesId = 188;
pub const CORSOLA: SpeciesId = 222;
pub const SLOWPOKE: SpeciesId = 79;
pub const PIKACHU: SpeciesId = 25;
pub const POLIWHIRL: SpeciesId = 61;
pub const KRABBY: SpeciesId = 98;
pub const STEELIX: SpeciesId = 208;
// ─── Lighthouse / Jasmine species ───────────────────────
pub const POLIWAG: SpeciesId = 60;
pub const MARILL: SpeciesId = 183;
pub const SPEAROW: SpeciesId = 21;
pub const FEAROW: SpeciesId = 22;
pub const MACHOP: SpeciesId = 66;
pub const AMPHAROS: SpeciesId = 181;
// ─── Route 40 / Cianwood species ─────────────────────────
pub const MANKEY: SpeciesId = 56;
pub const PRIMEAPE: SpeciesId = 57;
pub const POLIWRATH: SpeciesId = 62;
pub const TENTACOOL: SpeciesId = 72;
pub const TENTACRUEL: SpeciesId = 73;
pub const MACHOKE: SpeciesId = 67;
pub const MACHAMP: SpeciesId = 68;

pub const SEEL: SpeciesId = 86;
pub const DEWGONG: SpeciesId = 87;
pub const SWINUB: SpeciesId = 220;
pub const PILOSWINE: SpeciesId = 221;
pub const GIRAFARIG: SpeciesId = 203;
pub const GOLBAT: SpeciesId = 42;
pub const GYARADOS: SpeciesId = 130;
pub const GOLDEEN: SpeciesId = 118;
pub const SEAKING: SpeciesId = 119;
// ─── Sprint 50: Blackthorn / Phase 3 species ────────────
pub const JYNX: SpeciesId = 124;
pub const SNEASEL: SpeciesId = 215;
pub const DELIBIRD: SpeciesId = 225;
pub const DRATINI: SpeciesId = 147;
pub const DRAGONAIR: SpeciesId = 148;
pub const DRAGONITE: SpeciesId = 149;
pub const KINGDRA: SpeciesId = 230;
pub const HORSEA: SpeciesId = 116;
pub const SEADRA: SpeciesId = 117;

// ─── Move IDs ───────────────────────────────────────────
pub const MOVE_SMOG: MoveId = 123;
pub const MOVE_TACKLE: MoveId = 33;
pub const MOVE_GROWL: MoveId = 45;
pub const MOVE_RAZOR_LEAF: MoveId = 75;
pub const MOVE_LEER: MoveId = 43;
pub const MOVE_EMBER: MoveId = 52;
pub const MOVE_SMOKESCREEN: MoveId = 108;
pub const MOVE_SCRATCH: MoveId = 10;
pub const MOVE_WATER_GUN: MoveId = 55;
pub const MOVE_RAGE: MoveId = 99;
pub const MOVE_QUICK_ATTACK: MoveId = 98;
pub const MOVE_GUST: MoveId = 16;
pub const MOVE_TAIL_WHIP: MoveId = 39;
pub const MOVE_BITE: MoveId = 44;
pub const MOVE_PECK: MoveId = 64;
pub const MOVE_FORESIGHT: MoveId = 193;
pub const MOVE_STRING_SHOT: MoveId = 81;
pub const MOVE_POISON_STING: MoveId = 40;
pub const MOVE_VINE_WHIP: MoveId = 22;
pub const MOVE_DEFENSE_CURL: MoveId = 111;
pub const MOVE_SAND_ATTACK: MoveId = 28;
pub const MOVE_BIND: MoveId = 20;
pub const MOVE_THUNDER_SHOCK: MoveId = 84;
pub const MOVE_ROCK_THROW: MoveId = 88;
pub const MOVE_HYPNOSIS: MoveId = 95;
pub const MOVE_NIGHT_SHADE: MoveId = 101;
pub const MOVE_LICK: MoveId = 122;
pub const MOVE_SPLASH: MoveId = 150;
pub const MOVE_SCARY_FACE: MoveId = 184;
pub const MOVE_LEECH_LIFE: MoveId = 141;
pub const MOVE_MUD_SLAP: MoveId = 189;
pub const MOVE_ABSORB: MoveId = 71;
pub const MOVE_CONFUSION: MoveId = 93;
pub const MOVE_POUND: MoveId = 1;
pub const MOVE_TELEPORT: MoveId = 100;
pub const MOVE_TRANSFORM: MoveId = 144;
pub const MOVE_POISON_POWDER: MoveId = 77;
pub const MOVE_STUN_SPORE: MoveId = 78;
pub const MOVE_SLEEP_POWDER: MoveId = 79;
pub const MOVE_ACID: MoveId = 51;
pub const MOVE_FURY_SWIPES: MoveId = 154;
pub const MOVE_SLAM: MoveId = 21;
pub const MOVE_CONFUSE_RAY: MoveId = 109;
pub const MOVE_MAGNITUDE: MoveId = 222;
pub const MOVE_HYPER_FANG: MoveId = 162;
pub const MOVE_DOUBLE_KICK: MoveId = 24;
pub const MOVE_ROLLOUT: MoveId = 205;
pub const MOVE_ATTRACT: MoveId = 213;
pub const MOVE_STOMP: MoveId = 23;
pub const MOVE_MILK_DRINK: MoveId = 208;
pub const MOVE_DOUBLESLAP: MoveId = 3;
pub const MOVE_METRONOME: MoveId = 118;
pub const MOVE_SING: MoveId = 47;
pub const MOVE_DISABLE: MoveId = 50;
pub const MOVE_ENCORE: MoveId = 227;
// ─── Morty / Ecruteak + Route 35-37 moves ──────────────
pub const MOVE_SHADOW_BALL: MoveId = 247;
pub const MOVE_DREAM_EATER: MoveId = 138;
pub const MOVE_SPITE: MoveId = 180;
pub const MOVE_MEAN_LOOK: MoveId = 212;
pub const MOVE_CURSE: MoveId = 174;
pub const MOVE_MIMIC: MoveId = 102;
pub const MOVE_HORN_ATTACK: MoveId = 30;
pub const MOVE_FOCUS_ENERGY: MoveId = 116;
pub const MOVE_TAKE_DOWN: MoveId = 36;
pub const MOVE_ROAR: MoveId = 46;
pub const MOVE_FLAMETHROWER: MoveId = 53;
pub const MOVE_FIRE_SPIN: MoveId = 83;
pub const MOVE_SUPERSONIC: MoveId = 48;
pub const MOVE_SONIC_BOOM: MoveId = 49;
pub const MOVE_PSYBEAM: MoveId = 60;
pub const MOVE_LOW_KICK: MoveId = 67;
pub const MOVE_FLAIL: MoveId = 175;
pub const MOVE_ROCK_SLIDE: MoveId = 157;
pub const MOVE_FURY_ATTACK: MoveId = 31;
// ─── Ecruteak / Burned Tower / Eeveelution moves ────────
pub const MOVE_SLUDGE: MoveId = 124;
pub const MOVE_SELF_DESTRUCT: MoveId = 120;
pub const MOVE_HAZE: MoveId = 114;
pub const MOVE_PURSUIT: MoveId = 228;
pub const MOVE_FIRE_PUNCH: MoveId = 7;
// ─── Route 38-39 / Olivine moves ────────────────────────
pub const MOVE_THUNDER_WAVE: MoveId = 86;
pub const MOVE_BUBBLE: MoveId = 145;
pub const MOVE_HARDEN: MoveId = 106;
pub const MOVE_BARRIER: MoveId = 112;
pub const MOVE_MEDITATE: MoveId = 96;
pub const MOVE_VICEGRIP: MoveId = 11;
pub const MOVE_DOUBLE_TEAM: MoveId = 104;
pub const MOVE_RECOVER: MoveId = 105;
pub const MOVE_SWORDS_DANCE: MoveId = 14;
pub const MOVE_WHIRLWIND: MoveId = 18;
pub const MOVE_WING_ATTACK: MoveId = 17;
pub const MOVE_TRI_ATTACK: MoveId = 161;
pub const MOVE_DRILL_PECK: MoveId = 65;
pub const MOVE_BUBBLEBEAM: MoveId = 61;
pub const MOVE_THUNDERBOLT: MoveId = 85;
pub const MOVE_FAINT_ATTACK: MoveId = 185;
pub const MOVE_PAY_DAY: MoveId = 6;
pub const MOVE_IRON_TAIL: MoveId = 231;
pub const MOVE_SCREECH: MoveId = 103;
pub const MOVE_SUNNY_DAY: MoveId = 241;
pub const MOVE_KARATE_CHOP: MoveId = 2;
pub const MOVE_BODY_SLAM: MoveId = 34;
pub const MOVE_SEISMIC_TOSS: MoveId = 69;
// ─── Route 40 / Cianwood moves ──────────────────────────
pub const MOVE_CROSS_CHOP: MoveId = 238;
pub const MOVE_SUBMISSION: MoveId = 66;
pub const MOVE_DYNAMIC_PUNCH: MoveId = 223;
pub const MOVE_SURF: MoveId = 57;
pub const MOVE_CONSTRICT: MoveId = 132;
pub const MOVE_WRAP: MoveId = 35;
// ─── Mahogany / Ice moves ──────────────────────────────
pub const MOVE_HEADBUTT: MoveId = 29;
pub const MOVE_ICY_WIND: MoveId = 196;
pub const MOVE_AURORA_BEAM: MoveId = 62;
pub const MOVE_ICE_BEAM: MoveId = 58;
pub const MOVE_REST: MoveId = 156;
pub const MOVE_POWDER_SNOW: MoveId = 181;
pub const MOVE_EARTHQUAKE: MoveId = 89;
pub const MOVE_BLIZZARD: MoveId = 59;
pub const MOVE_HYDRO_PUMP: MoveId = 56;
pub const MOVE_DRAGON_RAGE: MoveId = 82;
pub const MOVE_TWISTER: MoveId = 239;
pub const MOVE_ENDURE: MoveId = 203;
pub const MOVE_AMNESIA: MoveId = 133;
pub const MOVE_THRASH: MoveId = 37;
// ─── Sprint 50: Blackthorn / Phase 3 moves ──────────────
pub const MOVE_AGILITY: MoveId = 97;
pub const MOVE_OUTRAGE: MoveId = 200;
pub const MOVE_HYPER_BEAM: MoveId = 63;
pub const MOVE_PRESENT: MoveId = 217;
pub const MOVE_ICE_PUNCH: MoveId = 8;
pub const MOVE_LOVELY_KISS: MoveId = 142;
pub const MOVE_SLASH: MoveId = 163;
pub const MOVE_SAFEGUARD: MoveId = 219;
// ─── Route 45/46 species + moves ────────────────────────
pub const GRAVELER: SpeciesId = 75;
pub const GLIGAR: SpeciesId = 207;
pub const TEDDIURSA: SpeciesId = 216;
pub const URSARING: SpeciesId = 217;
pub const SKARMORY: SpeciesId = 227;
pub const GOLEM: SpeciesId = 76;
pub const MOVE_SWIFT: MoveId = 129;
pub const MOVE_STEEL_WING: MoveId = 211;
// ─── Sprint 55: Route 27/26 species + moves ────────────
pub const PONYTA: SpeciesId = 77;
pub const RAPIDASH: SpeciesId = 78;
pub const SANDSHREW: SpeciesId = 27;
pub const SANDSLASH: SpeciesId = 28;
pub const DODRIO: SpeciesId = 85;
pub const ARCANINE: SpeciesId = 59;
pub const QUAGSIRE: SpeciesId = 195;
pub const MOVE_FIRE_BLAST: MoveId = 126;
pub const MOVE_EXTREME_SPEED: MoveId = 245;
pub const MOVE_FLAME_WHEEL: MoveId = 172;
// ─── Sprint 56: E4 / Victory Road species + moves ──────
pub const XATU: SpeciesId = 178;
pub const SLOWBRO: SpeciesId = 80;
pub const EXEGGUTOR: SpeciesId = 103;
pub const ARIADOS: SpeciesId = 168;
pub const FORRETRESS: SpeciesId = 205;
pub const MUK: SpeciesId = 89;
pub const VENOMOTH: SpeciesId = 49;
pub const CROBAT: SpeciesId = 169;
pub const HITMONTOP: SpeciesId = 237;
pub const HITMONLEE: SpeciesId = 106;
pub const HITMONCHAN: SpeciesId = 107;
pub const VILEPLUME: SpeciesId = 45;
pub const MURKROW: SpeciesId = 198;
pub const HOUNDOOM: SpeciesId = 229;
pub const AERODACTYL: SpeciesId = 142;
pub const CHARIZARD: SpeciesId = 6;
pub const MOVE_PSYCHIC: MoveId = 94;
pub const MOVE_CRUNCH: MoveId = 242;

/// Static species data
#[derive(Debug)]
pub struct SpeciesData {
    pub id: SpeciesId,
    pub name: &'static str,
    pub type1: PokemonType,
    pub type2: Option<PokemonType>,
    pub base_hp: u16,
    pub base_attack: u16,
    pub base_defense: u16,
    pub base_sp_attack: u16,
    pub base_sp_defense: u16,
    pub base_speed: u16,
    pub catch_rate: u8,
    pub base_exp_yield: u16,
    pub growth_rate: GrowthRate,
    pub learnset: &'static [(u8, MoveId)],
    pub evolution_level: Option<u8>,
    pub evolution_into: Option<SpeciesId>,
}

/// Static move data
#[derive(Debug)]
pub struct MoveData {
    pub id: MoveId,
    pub name: &'static str,
    pub move_type: PokemonType,
    pub category: MoveCategory,
    pub power: u8,
    pub accuracy: u8,
    pub pp: u8,
    pub description: &'static str,
}

/// A Pokemon instance (owned by player or wild)
#[derive(Clone, Debug)]
pub struct Pokemon {
    pub species_id: SpeciesId,
    pub nickname: Option<String>,
    pub level: u8,
    pub hp: u16,
    pub max_hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub sp_attack: u16,
    pub sp_defense: u16,
    pub speed: u16,
    pub exp: u32,
    pub moves: [Option<MoveId>; 4],
    pub move_pp: [u8; 4],
    pub move_max_pp: [u8; 4],
    pub status: StatusCondition,
}

// ─── Species Database ───────────────────────────────────

const SPECIES_DB: &[SpeciesData] = &[
    // Starters
    SpeciesData {
        id: CHIKORITA, name: "Chikorita",
        type1: PokemonType::Grass, type2: None,
        base_hp: 45, base_attack: 49, base_defense: 65,
        base_sp_attack: 49, base_sp_defense: 65, base_speed: 45,
        catch_rate: 45, base_exp_yield: 64, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_GROWL), (8, MOVE_RAZOR_LEAF), (12, MOVE_VINE_WHIP), (15, MOVE_POISON_POWDER)],
        evolution_level: Some(16), evolution_into: Some(BAYLEEF),
    },
    SpeciesData {
        id: BAYLEEF, name: "Bayleef",
        type1: PokemonType::Grass, type2: None,
        base_hp: 60, base_attack: 62, base_defense: 80,
        base_sp_attack: 63, base_sp_defense: 80, base_speed: 60,
        catch_rate: 45, base_exp_yield: 141, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_GROWL), (8, MOVE_RAZOR_LEAF), (12, MOVE_VINE_WHIP), (15, MOVE_POISON_POWDER)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: CYNDAQUIL, name: "Cyndaquil",
        type1: PokemonType::Fire, type2: None,
        base_hp: 39, base_attack: 52, base_defense: 43,
        base_sp_attack: 60, base_sp_defense: 50, base_speed: 65,
        catch_rate: 45, base_exp_yield: 65, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_LEER), (6, MOVE_SMOKESCREEN), (12, MOVE_EMBER), (19, MOVE_QUICK_ATTACK)],
        evolution_level: Some(14), evolution_into: Some(QUILAVA),
    },
    SpeciesData {
        id: QUILAVA, name: "Quilava",
        type1: PokemonType::Fire, type2: None,
        base_hp: 58, base_attack: 64, base_defense: 58,
        base_sp_attack: 80, base_sp_defense: 65, base_speed: 80,
        catch_rate: 45, base_exp_yield: 142, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_LEER), (6, MOVE_SMOKESCREEN), (12, MOVE_EMBER), (21, MOVE_QUICK_ATTACK)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: TOTODILE, name: "Totodile",
        type1: PokemonType::Water, type2: None,
        base_hp: 50, base_attack: 65, base_defense: 64,
        base_sp_attack: 44, base_sp_defense: 48, base_speed: 43,
        catch_rate: 45, base_exp_yield: 66, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_SCRATCH), (1, MOVE_LEER), (7, MOVE_RAGE), (13, MOVE_WATER_GUN), (20, MOVE_BITE)],
        evolution_level: Some(18), evolution_into: Some(CROCONAW),
    },
    SpeciesData {
        id: CROCONAW, name: "Croconaw",
        type1: PokemonType::Water, type2: None,
        base_hp: 65, base_attack: 80, base_defense: 80,
        base_sp_attack: 59, base_sp_defense: 63, base_speed: 58,
        catch_rate: 45, base_exp_yield: 143, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_SCRATCH), (1, MOVE_LEER), (7, MOVE_RAGE), (13, MOVE_WATER_GUN), (21, MOVE_BITE)],
        evolution_level: None, evolution_into: None,
    },
    // Wild Pokemon
    SpeciesData {
        id: PIDGEY, name: "Pidgey",
        type1: PokemonType::Normal, type2: Some(PokemonType::Flying),
        base_hp: 40, base_attack: 45, base_defense: 40,
        base_sp_attack: 35, base_sp_defense: 35, base_speed: 56,
        catch_rate: 255, base_exp_yield: 55, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_TACKLE), (5, MOVE_SAND_ATTACK), (9, MOVE_GUST), (15, MOVE_QUICK_ATTACK)],
        evolution_level: Some(18), evolution_into: Some(17), // Pidgeotto
    },
    SpeciesData {
        id: RATTATA, name: "Rattata",
        type1: PokemonType::Normal, type2: None,
        base_hp: 30, base_attack: 56, base_defense: 35,
        base_sp_attack: 25, base_sp_defense: 35, base_speed: 72,
        catch_rate: 255, base_exp_yield: 57, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_TAIL_WHIP), (7, MOVE_QUICK_ATTACK), (13, MOVE_HYPER_FANG)],
        evolution_level: Some(20), evolution_into: Some(20), // Raticate
    },
    SpeciesData {
        id: SENTRET, name: "Sentret",
        type1: PokemonType::Normal, type2: None,
        base_hp: 35, base_attack: 46, base_defense: 34,
        base_sp_attack: 35, base_sp_defense: 45, base_speed: 20,
        catch_rate: 255, base_exp_yield: 57, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_DEFENSE_CURL), (7, MOVE_QUICK_ATTACK), (12, MOVE_SCRATCH), (17, MOVE_FURY_SWIPES), (25, MOVE_SLAM)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: HOOTHOOT, name: "Hoothoot",
        type1: PokemonType::Normal, type2: Some(PokemonType::Flying),
        base_hp: 60, base_attack: 30, base_defense: 30,
        base_sp_attack: 36, base_sp_defense: 56, base_speed: 50,
        catch_rate: 255, base_exp_yield: 58, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_GROWL), (6, MOVE_FORESIGHT), (11, MOVE_PECK)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: CATERPIE, name: "Caterpie",
        type1: PokemonType::Bug, type2: None,
        base_hp: 45, base_attack: 30, base_defense: 35,
        base_sp_attack: 20, base_sp_defense: 20, base_speed: 45,
        catch_rate: 255, base_exp_yield: 53, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_STRING_SHOT)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: WEEDLE, name: "Weedle",
        type1: PokemonType::Bug, type2: Some(PokemonType::Poison),
        base_hp: 40, base_attack: 35, base_defense: 30,
        base_sp_attack: 20, base_sp_defense: 20, base_speed: 50,
        catch_rate: 255, base_exp_yield: 52, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_POISON_STING), (1, MOVE_STRING_SHOT)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: GEODUDE, name: "Geodude",
        type1: PokemonType::Rock, type2: Some(PokemonType::Ground),
        base_hp: 40, base_attack: 80, base_defense: 100,
        base_sp_attack: 30, base_sp_defense: 30, base_speed: 20,
        catch_rate: 255, base_exp_yield: 73, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_DEFENSE_CURL), (11, MOVE_ROCK_THROW), (16, MOVE_MAGNITUDE)],
        evolution_level: Some(25), evolution_into: Some(75), // Graveler
    },
    SpeciesData {
        id: ZUBAT, name: "Zubat",
        type1: PokemonType::Poison, type2: Some(PokemonType::Flying),
        base_hp: 40, base_attack: 45, base_defense: 35,
        base_sp_attack: 30, base_sp_defense: 40, base_speed: 55,
        catch_rate: 255, base_exp_yield: 54, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_LEECH_LIFE), (1, MOVE_LEER), (6, MOVE_BITE), (19, MOVE_CONFUSE_RAY)],
        evolution_level: Some(22), evolution_into: Some(42), // Golbat
    },
    SpeciesData {
        id: BELLSPROUT, name: "Bellsprout",
        type1: PokemonType::Grass, type2: Some(PokemonType::Poison),
        base_hp: 50, base_attack: 75, base_defense: 35,
        base_sp_attack: 70, base_sp_defense: 30, base_speed: 40,
        catch_rate: 255, base_exp_yield: 84, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_VINE_WHIP), (7, MOVE_GROWL), (15, MOVE_SLEEP_POWDER), (17, MOVE_POISON_POWDER), (19, MOVE_STUN_SPORE), (23, MOVE_ACID)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: GASTLY, name: "Gastly",
        type1: PokemonType::Ghost, type2: Some(PokemonType::Poison),
        base_hp: 30, base_attack: 35, base_defense: 30,
        base_sp_attack: 100, base_sp_defense: 35, base_speed: 80,
        catch_rate: 255, base_exp_yield: 95, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_LICK), (1, MOVE_SPITE), (1, MOVE_HYPNOSIS), (8, MOVE_MEAN_LOOK), (13, MOVE_CURSE), (16, MOVE_NIGHT_SHADE), (21, MOVE_CONFUSE_RAY), (28, MOVE_DREAM_EATER)],
        evolution_level: Some(25), evolution_into: Some(93), // Haunter
    },
    SpeciesData {
        id: ONIX, name: "Onix",
        type1: PokemonType::Rock, type2: Some(PokemonType::Ground),
        base_hp: 35, base_attack: 45, base_defense: 160,
        base_sp_attack: 30, base_sp_defense: 45, base_speed: 70,
        catch_rate: 255, base_exp_yield: 108, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_BIND), (9, MOVE_ROCK_THROW)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: MAGIKARP, name: "Magikarp",
        type1: PokemonType::Water, type2: None,
        base_hp: 20, base_attack: 10, base_defense: 55,
        base_sp_attack: 15, base_sp_defense: 20, base_speed: 80,
        catch_rate: 255, base_exp_yield: 20, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_SPLASH), (15, MOVE_TACKLE)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: LEDYBA, name: "Ledyba",
        type1: PokemonType::Bug, type2: Some(PokemonType::Flying),
        base_hp: 40, base_attack: 20, base_defense: 30,
        base_sp_attack: 40, base_sp_defense: 80, base_speed: 55,
        catch_rate: 255, base_exp_yield: 54, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (8, MOVE_PECK)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: SPINARAK, name: "Spinarak",
        type1: PokemonType::Bug, type2: Some(PokemonType::Poison),
        base_hp: 40, base_attack: 60, base_defense: 40,
        base_sp_attack: 40, base_sp_defense: 40, base_speed: 30,
        catch_rate: 255, base_exp_yield: 54, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_POISON_STING), (1, MOVE_STRING_SHOT), (11, MOVE_SCARY_FACE)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: MAREEP, name: "Mareep",
        type1: PokemonType::Electric, type2: None,
        base_hp: 55, base_attack: 40, base_defense: 40,
        base_sp_attack: 65, base_sp_defense: 45, base_speed: 35,
        catch_rate: 255, base_exp_yield: 59, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_GROWL), (9, MOVE_THUNDER_SHOCK)],
        evolution_level: Some(15), evolution_into: Some(180), // Flaaffy
    },
    SpeciesData {
        id: WOOPER, name: "Wooper",
        type1: PokemonType::Water, type2: Some(PokemonType::Ground),
        base_hp: 55, base_attack: 45, base_defense: 45,
        base_sp_attack: 25, base_sp_defense: 25, base_speed: 15,
        catch_rate: 255, base_exp_yield: 52, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_WATER_GUN), (1, MOVE_TAIL_WHIP), (11, MOVE_SLAM)],
        evolution_level: Some(20), evolution_into: Some(QUAGSIRE),
    },
    SpeciesData {
        id: HOPPIP, name: "Hoppip",
        type1: PokemonType::Grass, type2: Some(PokemonType::Flying),
        base_hp: 35, base_attack: 35, base_defense: 40,
        base_sp_attack: 35, base_sp_defense: 55, base_speed: 50,
        catch_rate: 255, base_exp_yield: 50, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_TAIL_WHIP)],
        evolution_level: None, evolution_into: None,
    },
    // ─── New species: Ilex Forest / Route 34 ─────────────
    SpeciesData {
        id: ODDISH, name: "Oddish",
        type1: PokemonType::Grass, type2: Some(PokemonType::Poison),
        base_hp: 45, base_attack: 50, base_defense: 55,
        base_sp_attack: 75, base_sp_defense: 65, base_speed: 30,
        catch_rate: 255, base_exp_yield: 78, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_ABSORB), (14, MOVE_POISON_POWDER), (16, MOVE_STUN_SPORE), (18, MOVE_SLEEP_POWDER), (23, MOVE_ACID)],
        evolution_level: Some(21), evolution_into: Some(44), // Gloom
    },
    SpeciesData {
        id: DROWZEE, name: "Drowzee",
        type1: PokemonType::Psychic, type2: None,
        base_hp: 60, base_attack: 48, base_defense: 45,
        base_sp_attack: 43, base_sp_defense: 90, base_speed: 42,
        catch_rate: 190, base_exp_yield: 102, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_POUND), (1, MOVE_HYPNOSIS), (18, MOVE_CONFUSION)],
        evolution_level: Some(26), evolution_into: Some(97), // Hypno
    },
    SpeciesData {
        id: ABRA, name: "Abra",
        type1: PokemonType::Psychic, type2: None,
        base_hp: 25, base_attack: 20, base_defense: 15,
        base_sp_attack: 105, base_sp_defense: 55, base_speed: 90,
        catch_rate: 200, base_exp_yield: 73, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_TELEPORT)],
        evolution_level: Some(16), evolution_into: Some(64), // Kadabra
    },
    SpeciesData {
        id: DITTO, name: "Ditto",
        type1: PokemonType::Normal, type2: None,
        base_hp: 48, base_attack: 48, base_defense: 48,
        base_sp_attack: 48, base_sp_defense: 48, base_speed: 48,
        catch_rate: 35, base_exp_yield: 61, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TRANSFORM)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: PARAS, name: "Paras",
        type1: PokemonType::Bug, type2: Some(PokemonType::Grass),
        base_hp: 35, base_attack: 70, base_defense: 55,
        base_sp_attack: 45, base_sp_defense: 55, base_speed: 25,
        catch_rate: 190, base_exp_yield: 70, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_SCRATCH), (7, MOVE_STUN_SPORE), (13, MOVE_POISON_POWDER), (19, MOVE_LEECH_LIFE)],
        evolution_level: Some(24), evolution_into: Some(47), // Parasect
    },
    // ─── Goldenrod / Whitney Gym species ─────────────────
    SpeciesData {
        id: CLEFAIRY, name: "Clefairy",
        type1: PokemonType::Normal, type2: None,
        base_hp: 70, base_attack: 45, base_defense: 48,
        base_sp_attack: 60, base_sp_defense: 65, base_speed: 35,
        catch_rate: 150, base_exp_yield: 68, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_POUND), (1, MOVE_GROWL), (8, MOVE_ENCORE), (13, MOVE_SING), (18, MOVE_DOUBLESLAP), (25, MOVE_METRONOME)],
        evolution_level: None, evolution_into: None, // Evolves with Moon Stone
    },
    SpeciesData {
        id: MILTANK, name: "Miltank",
        type1: PokemonType::Normal, type2: None,
        base_hp: 95, base_attack: 80, base_defense: 105,
        base_sp_attack: 40, base_sp_defense: 70, base_speed: 100,
        catch_rate: 45, base_exp_yield: 200, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_TACKLE), (5, MOVE_GROWL), (8, MOVE_DEFENSE_CURL), (11, MOVE_STOMP), (15, MOVE_MILK_DRINK), (18, MOVE_ATTRACT), (19, MOVE_ROLLOUT)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: SNUBBULL, name: "Snubbull",
        type1: PokemonType::Normal, type2: None, // Gen 2: Normal type (became Fairy in Gen 6)
        base_hp: 60, base_attack: 80, base_defense: 50,
        base_sp_attack: 40, base_sp_defense: 40, base_speed: 30,
        catch_rate: 190, base_exp_yield: 63, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_SCARY_FACE), (7, MOVE_BITE), (13, MOVE_LICK)],
        evolution_level: Some(23), evolution_into: Some(210), // Granbull
    },
    SpeciesData {
        id: JIGGLYPUFF, name: "Jigglypuff",
        type1: PokemonType::Normal, type2: None,
        base_hp: 115, base_attack: 45, base_defense: 20,
        base_sp_attack: 45, base_sp_defense: 25, base_speed: 20,
        catch_rate: 170, base_exp_yield: 76, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_SING), (4, MOVE_DEFENSE_CURL), (9, MOVE_POUND), (14, MOVE_DISABLE), (19, MOVE_ROLLOUT)],
        evolution_level: None, evolution_into: None, // Evolves with Moon Stone
    },
    SpeciesData {
        id: MEOWTH, name: "Meowth",
        type1: PokemonType::Normal, type2: None,
        base_hp: 40, base_attack: 45, base_defense: 35,
        base_sp_attack: 40, base_sp_defense: 40, base_speed: 90,
        catch_rate: 255, base_exp_yield: 69, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_SCRATCH), (1, MOVE_GROWL), (11, MOVE_BITE), (20, MOVE_FURY_SWIPES)],
        evolution_level: Some(28), evolution_into: Some(53), // Persian
    },
    // ─── Route 35-37, Ecruteak, Morty species ───────────────
    SpeciesData {
        id: NIDORAN_F, name: "Nidoran F",
        type1: PokemonType::Poison, type2: None,
        base_hp: 55, base_attack: 47, base_defense: 52,
        base_sp_attack: 40, base_sp_defense: 40, base_speed: 41,
        catch_rate: 235, base_exp_yield: 59, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_GROWL), (1, MOVE_TACKLE), (8, MOVE_SCRATCH), (12, MOVE_DOUBLE_KICK), (17, MOVE_POISON_STING), (23, MOVE_BITE)],
        evolution_level: Some(16), evolution_into: Some(30), // Nidorina
    },
    SpeciesData {
        id: NIDORAN_M, name: "Nidoran M",
        type1: PokemonType::Poison, type2: None,
        base_hp: 46, base_attack: 57, base_defense: 40,
        base_sp_attack: 40, base_sp_defense: 40, base_speed: 50,
        catch_rate: 235, base_exp_yield: 60, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_LEER), (1, MOVE_TACKLE), (8, MOVE_HORN_ATTACK), (12, MOVE_DOUBLE_KICK), (17, MOVE_POISON_STING), (23, MOVE_FOCUS_ENERGY)],
        evolution_level: Some(16), evolution_into: Some(33), // Nidorino
    },
    SpeciesData {
        id: GROWLITHE, name: "Growlithe",
        type1: PokemonType::Fire, type2: None,
        base_hp: 55, base_attack: 70, base_defense: 45,
        base_sp_attack: 70, base_sp_defense: 50, base_speed: 60,
        catch_rate: 190, base_exp_yield: 91, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_BITE), (1, MOVE_ROAR), (7, MOVE_EMBER), (13, MOVE_LEER), (19, MOVE_TAKE_DOWN), (25, MOVE_FLAMETHROWER), (31, MOVE_FLAME_WHEEL), (37, MOVE_AGILITY)],
        evolution_level: Some(36), evolution_into: Some(ARCANINE),
    },
    SpeciesData {
        id: VULPIX, name: "Vulpix",
        type1: PokemonType::Fire, type2: None,
        base_hp: 38, base_attack: 41, base_defense: 40,
        base_sp_attack: 50, base_sp_defense: 65, base_speed: 65,
        catch_rate: 190, base_exp_yield: 63, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_EMBER), (1, MOVE_TAIL_WHIP), (7, MOVE_QUICK_ATTACK), (13, MOVE_CONFUSE_RAY), (19, MOVE_FIRE_SPIN), (25, MOVE_FLAMETHROWER)],
        evolution_level: None, evolution_into: None, // Fire Stone evolution
    },
    SpeciesData {
        id: STANTLER, name: "Stantler",
        type1: PokemonType::Normal, type2: None,
        base_hp: 73, base_attack: 95, base_defense: 62,
        base_sp_attack: 85, base_sp_defense: 65, base_speed: 85,
        catch_rate: 45, base_exp_yield: 165, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_TACKLE), (7, MOVE_LEER), (11, MOVE_HYPNOSIS), (16, MOVE_STOMP), (23, MOVE_SAND_ATTACK), (28, MOVE_TAKE_DOWN), (33, MOVE_CONFUSE_RAY)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: YANMA, name: "Yanma",
        type1: PokemonType::Bug, type2: Some(PokemonType::Flying),
        base_hp: 65, base_attack: 65, base_defense: 45,
        base_sp_attack: 75, base_sp_defense: 45, base_speed: 95,
        catch_rate: 75, base_exp_yield: 147, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_FORESIGHT), (7, MOVE_QUICK_ATTACK), (13, MOVE_DOUBLE_KICK), (19, MOVE_SONIC_BOOM), (25, MOVE_SUPERSONIC)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: VENONAT, name: "Venonat",
        type1: PokemonType::Bug, type2: Some(PokemonType::Poison),
        base_hp: 60, base_attack: 55, base_defense: 50,
        base_sp_attack: 40, base_sp_defense: 55, base_speed: 45,
        catch_rate: 190, base_exp_yield: 75, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_DISABLE), (9, MOVE_FORESIGHT), (13, MOVE_SUPERSONIC), (17, MOVE_CONFUSION), (20, MOVE_POISON_POWDER), (25, MOVE_LEECH_LIFE), (28, MOVE_STUN_SPORE), (33, MOVE_PSYBEAM)],
        evolution_level: Some(31), evolution_into: Some(49), // Venomoth
    },
    SpeciesData {
        id: HAUNTER, name: "Haunter",
        type1: PokemonType::Ghost, type2: Some(PokemonType::Poison),
        base_hp: 45, base_attack: 50, base_defense: 45,
        base_sp_attack: 115, base_sp_defense: 55, base_speed: 95,
        catch_rate: 90, base_exp_yield: 126, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_LICK), (1, MOVE_HYPNOSIS), (1, MOVE_SPITE), (8, MOVE_MEAN_LOOK), (13, MOVE_CURSE), (16, MOVE_NIGHT_SHADE), (21, MOVE_CONFUSE_RAY), (25, MOVE_SHADOW_BALL), (31, MOVE_DREAM_EATER)],
        evolution_level: None, evolution_into: None, // Trade evolution
    },
    SpeciesData {
        id: GENGAR, name: "Gengar",
        type1: PokemonType::Ghost, type2: Some(PokemonType::Poison),
        base_hp: 60, base_attack: 65, base_defense: 60,
        base_sp_attack: 130, base_sp_defense: 75, base_speed: 110,
        catch_rate: 45, base_exp_yield: 190, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_LICK), (1, MOVE_HYPNOSIS), (1, MOVE_SPITE), (8, MOVE_MEAN_LOOK), (13, MOVE_CURSE), (16, MOVE_NIGHT_SHADE), (21, MOVE_CONFUSE_RAY), (25, MOVE_SHADOW_BALL), (31, MOVE_DREAM_EATER)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: SUDOWOODO, name: "Sudowoodo",
        type1: PokemonType::Rock, type2: None,
        base_hp: 70, base_attack: 100, base_defense: 115,
        base_sp_attack: 30, base_sp_defense: 65, base_speed: 30,
        catch_rate: 65, base_exp_yield: 135, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_ROCK_THROW), (1, MOVE_MIMIC), (10, MOVE_FLAIL), (15, MOVE_LOW_KICK), (20, MOVE_ROCK_SLIDE), (25, MOVE_SLAM)],
        evolution_level: None, evolution_into: None,
    },
    // ─── Ecruteak / Burned Tower species ─────────────────
    SpeciesData {
        id: KOFFING, name: "Koffing",
        type1: PokemonType::Poison, type2: None,
        base_hp: 40, base_attack: 65, base_defense: 95,
        base_sp_attack: 60, base_sp_defense: 45, base_speed: 35,
        catch_rate: 190, base_exp_yield: 114, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_POISON_STING), (1, MOVE_TACKLE), (9, MOVE_SMOG), (17, MOVE_SLUDGE), (21, MOVE_SMOKESCREEN), (25, MOVE_SELF_DESTRUCT), (33, MOVE_HAZE)],
        evolution_level: Some(35), evolution_into: None, // Weezing not yet added
    },
    SpeciesData {
        id: RATICATE, name: "Raticate",
        type1: PokemonType::Normal, type2: None,
        base_hp: 55, base_attack: 81, base_defense: 60,
        base_sp_attack: 50, base_sp_defense: 70, base_speed: 97,
        catch_rate: 127, base_exp_yield: 116, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_TAIL_WHIP), (7, MOVE_QUICK_ATTACK), (13, MOVE_HYPER_FANG), (20, MOVE_SCARY_FACE), (30, MOVE_PURSUIT)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: MAGMAR, name: "Magmar",
        type1: PokemonType::Fire, type2: None,
        base_hp: 65, base_attack: 95, base_defense: 57,
        base_sp_attack: 100, base_sp_defense: 85, base_speed: 93,
        catch_rate: 45, base_exp_yield: 167, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_EMBER), (7, MOVE_LEER), (13, MOVE_SMOG), (19, MOVE_FIRE_PUNCH), (25, MOVE_SMOKESCREEN), (33, MOVE_FLAMETHROWER)],
        evolution_level: None, evolution_into: None,
    },
    // ─── Eevee + Eeveelutions (Kimono Girls) ─────────────
    SpeciesData {
        id: EEVEE, name: "Eevee",
        type1: PokemonType::Normal, type2: None,
        base_hp: 55, base_attack: 55, base_defense: 50,
        base_sp_attack: 45, base_sp_defense: 65, base_speed: 55,
        catch_rate: 45, base_exp_yield: 92, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_TAIL_WHIP), (8, MOVE_SAND_ATTACK), (16, MOVE_GROWL), (23, MOVE_QUICK_ATTACK), (30, MOVE_BITE)],
        evolution_level: None, evolution_into: None, // Stone/friendship evolution
    },
    SpeciesData {
        id: VAPOREON, name: "Vaporeon",
        type1: PokemonType::Water, type2: None,
        base_hp: 130, base_attack: 65, base_defense: 60,
        base_sp_attack: 110, base_sp_defense: 95, base_speed: 65,
        catch_rate: 45, base_exp_yield: 196, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_TAIL_WHIP), (8, MOVE_SAND_ATTACK), (16, MOVE_WATER_GUN), (23, MOVE_QUICK_ATTACK), (30, MOVE_BITE)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: JOLTEON, name: "Jolteon",
        type1: PokemonType::Electric, type2: None,
        base_hp: 65, base_attack: 65, base_defense: 60,
        base_sp_attack: 110, base_sp_defense: 95, base_speed: 130,
        catch_rate: 45, base_exp_yield: 197, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_TAIL_WHIP), (8, MOVE_SAND_ATTACK), (16, MOVE_THUNDER_SHOCK), (23, MOVE_QUICK_ATTACK), (30, MOVE_BITE)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: FLAREON, name: "Flareon",
        type1: PokemonType::Fire, type2: None,
        base_hp: 65, base_attack: 130, base_defense: 60,
        base_sp_attack: 95, base_sp_defense: 110, base_speed: 65,
        catch_rate: 45, base_exp_yield: 198, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_TAIL_WHIP), (8, MOVE_SAND_ATTACK), (16, MOVE_EMBER), (23, MOVE_QUICK_ATTACK), (30, MOVE_BITE)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: ESPEON, name: "Espeon",
        type1: PokemonType::Psychic, type2: None,
        base_hp: 65, base_attack: 65, base_defense: 60,
        base_sp_attack: 130, base_sp_defense: 95, base_speed: 110,
        catch_rate: 45, base_exp_yield: 197, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_TAIL_WHIP), (8, MOVE_SAND_ATTACK), (16, MOVE_CONFUSION), (23, MOVE_QUICK_ATTACK), (30, MOVE_PSYBEAM)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: UMBREON, name: "Umbreon",
        type1: PokemonType::Dark, type2: None,
        base_hp: 95, base_attack: 65, base_defense: 110,
        base_sp_attack: 60, base_sp_defense: 130, base_speed: 65,
        catch_rate: 45, base_exp_yield: 197, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_TAIL_WHIP), (8, MOVE_SAND_ATTACK), (16, MOVE_PURSUIT), (23, MOVE_QUICK_ATTACK), (30, MOVE_CONFUSE_RAY)],
        evolution_level: None, evolution_into: None,
    },
    // ─── Route 38-39 / Olivine species ──────────────────────
    SpeciesData {
        id: PIDGEOTTO, name: "Pidgeotto",
        type1: PokemonType::Normal, type2: Some(PokemonType::Flying),
        base_hp: 63, base_attack: 60, base_defense: 55,
        base_sp_attack: 50, base_sp_defense: 50, base_speed: 71,
        catch_rate: 120, base_exp_yield: 113, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_SAND_ATTACK), (1, MOVE_GUST), (5, MOVE_SAND_ATTACK), (9, MOVE_GUST), (15, MOVE_QUICK_ATTACK), (23, MOVE_WHIRLWIND), (33, MOVE_WING_ATTACK)],
        evolution_level: Some(36), evolution_into: None,
    },
    SpeciesData {
        id: NOCTOWL, name: "Noctowl",
        type1: PokemonType::Normal, type2: Some(PokemonType::Flying),
        base_hp: 100, base_attack: 50, base_defense: 50,
        base_sp_attack: 86, base_sp_defense: 96, base_speed: 70,
        catch_rate: 90, base_exp_yield: 162, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_GROWL), (1, MOVE_FORESIGHT), (1, MOVE_PECK), (6, MOVE_FORESIGHT), (11, MOVE_PECK), (16, MOVE_HYPNOSIS), (25, MOVE_CONFUSION)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: FARFETCHD, name: "Farfetch'd",
        type1: PokemonType::Normal, type2: Some(PokemonType::Flying),
        base_hp: 52, base_attack: 65, base_defense: 55,
        base_sp_attack: 58, base_sp_defense: 62, base_speed: 60,
        catch_rate: 45, base_exp_yield: 94, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_PECK), (7, MOVE_SAND_ATTACK), (13, MOVE_LEER), (19, MOVE_FURY_ATTACK), (25, MOVE_SWORDS_DANCE)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: TAUROS, name: "Tauros",
        type1: PokemonType::Normal, type2: None,
        base_hp: 75, base_attack: 100, base_defense: 95,
        base_sp_attack: 40, base_sp_defense: 70, base_speed: 110,
        catch_rate: 45, base_exp_yield: 211, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_TACKLE), (4, MOVE_TAIL_WHIP), (8, MOVE_RAGE), (13, MOVE_HORN_ATTACK), (19, MOVE_SCARY_FACE), (26, MOVE_PURSUIT)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: MAGNEMITE, name: "Magnemite",
        type1: PokemonType::Electric, type2: Some(PokemonType::Steel),
        base_hp: 25, base_attack: 35, base_defense: 70,
        base_sp_attack: 95, base_sp_defense: 55, base_speed: 45,
        catch_rate: 190, base_exp_yield: 89, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (6, MOVE_THUNDER_SHOCK), (11, MOVE_SUPERSONIC), (16, MOVE_SONIC_BOOM), (21, MOVE_THUNDER_WAVE), (27, MOVE_THUNDERBOLT)],
        evolution_level: Some(30), evolution_into: None,
    },
    SpeciesData {
        id: DODUO, name: "Doduo",
        type1: PokemonType::Normal, type2: Some(PokemonType::Flying),
        base_hp: 35, base_attack: 85, base_defense: 45,
        base_sp_attack: 35, base_sp_defense: 35, base_speed: 75,
        catch_rate: 190, base_exp_yield: 96, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_PECK), (1, MOVE_GROWL), (9, MOVE_PURSUIT), (13, MOVE_FURY_ATTACK), (21, MOVE_TRI_ATTACK), (25, MOVE_RAGE)],
        evolution_level: Some(31), evolution_into: Some(DODRIO),
    },
    SpeciesData {
        id: FLAAFFY, name: "Flaaffy",
        type1: PokemonType::Electric, type2: None,
        base_hp: 70, base_attack: 55, base_defense: 55,
        base_sp_attack: 80, base_sp_defense: 60, base_speed: 45,
        catch_rate: 120, base_exp_yield: 117, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_GROWL), (1, MOVE_THUNDER_SHOCK), (9, MOVE_THUNDER_SHOCK), (18, MOVE_THUNDER_WAVE), (27, MOVE_THUNDERBOLT)],
        evolution_level: Some(30), evolution_into: None,
    },
    SpeciesData {
        id: PSYDUCK, name: "Psyduck",
        type1: PokemonType::Water, type2: None,
        base_hp: 50, base_attack: 52, base_defense: 48,
        base_sp_attack: 65, base_sp_defense: 50, base_speed: 55,
        catch_rate: 190, base_exp_yield: 80, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_SCRATCH), (5, MOVE_TAIL_WHIP), (10, MOVE_DISABLE), (16, MOVE_CONFUSION), (23, MOVE_FURY_SWIPES)],
        evolution_level: Some(33), evolution_into: None,
    },
    SpeciesData {
        id: MR_MIME, name: "Mr. Mime",
        type1: PokemonType::Psychic, type2: None,
        base_hp: 40, base_attack: 45, base_defense: 65,
        base_sp_attack: 100, base_sp_defense: 120, base_speed: 90,
        catch_rate: 45, base_exp_yield: 136, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_BARRIER), (6, MOVE_CONFUSION), (11, MOVE_MEDITATE), (16, MOVE_DOUBLESLAP), (21, MOVE_PSYBEAM)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: SKIPLOOM, name: "Skiploom",
        type1: PokemonType::Grass, type2: Some(PokemonType::Flying),
        base_hp: 55, base_attack: 45, base_defense: 50,
        base_sp_attack: 45, base_sp_defense: 65, base_speed: 80,
        catch_rate: 120, base_exp_yield: 136, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_SPLASH), (1, MOVE_TACKLE), (1, MOVE_TAIL_WHIP), (10, MOVE_TACKLE), (13, MOVE_POISON_POWDER), (15, MOVE_STUN_SPORE), (17, MOVE_SLEEP_POWDER)],
        evolution_level: Some(27), evolution_into: None,
    },
    SpeciesData {
        id: CORSOLA, name: "Corsola",
        type1: PokemonType::Water, type2: Some(PokemonType::Rock),
        base_hp: 65, base_attack: 55, base_defense: 95,
        base_sp_attack: 65, base_sp_defense: 95, base_speed: 35,
        catch_rate: 60, base_exp_yield: 113, growth_rate: GrowthRate::Fast,
        learnset: &[(1, MOVE_TACKLE), (7, MOVE_HARDEN), (13, MOVE_BUBBLE), (19, MOVE_RECOVER), (25, MOVE_BUBBLEBEAM)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: SLOWPOKE, name: "Slowpoke",
        type1: PokemonType::Water, type2: Some(PokemonType::Psychic),
        base_hp: 90, base_attack: 65, base_defense: 65,
        base_sp_attack: 40, base_sp_defense: 40, base_speed: 15,
        catch_rate: 190, base_exp_yield: 99, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_CURSE), (1, MOVE_TACKLE), (6, MOVE_GROWL), (15, MOVE_WATER_GUN), (20, MOVE_CONFUSION)],
        evolution_level: Some(37), evolution_into: None,
    },
    SpeciesData {
        id: PIKACHU, name: "Pikachu",
        type1: PokemonType::Electric, type2: None,
        base_hp: 35, base_attack: 55, base_defense: 40,
        base_sp_attack: 50, base_sp_defense: 40, base_speed: 90,
        catch_rate: 190, base_exp_yield: 82, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_THUNDER_SHOCK), (1, MOVE_GROWL), (6, MOVE_TAIL_WHIP), (8, MOVE_THUNDER_WAVE), (11, MOVE_QUICK_ATTACK), (15, MOVE_DOUBLE_TEAM), (20, MOVE_SLAM), (26, MOVE_THUNDERBOLT)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: POLIWHIRL, name: "Poliwhirl",
        type1: PokemonType::Water, type2: None,
        base_hp: 65, base_attack: 65, base_defense: 65,
        base_sp_attack: 50, base_sp_defense: 50, base_speed: 90,
        catch_rate: 120, base_exp_yield: 131, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_BUBBLE), (1, MOVE_HYPNOSIS), (1, MOVE_WATER_GUN), (7, MOVE_HYPNOSIS), (13, MOVE_WATER_GUN), (19, MOVE_DOUBLESLAP)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: KRABBY, name: "Krabby",
        type1: PokemonType::Water, type2: None,
        base_hp: 30, base_attack: 105, base_defense: 90,
        base_sp_attack: 25, base_sp_defense: 25, base_speed: 50,
        catch_rate: 225, base_exp_yield: 115, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_BUBBLE), (5, MOVE_LEER), (12, MOVE_VICEGRIP), (16, MOVE_HARDEN), (23, MOVE_STOMP)],
        evolution_level: Some(28), evolution_into: None,
    },
    SpeciesData {
        id: STEELIX, name: "Steelix",
        type1: PokemonType::Steel, type2: Some(PokemonType::Ground),
        base_hp: 75, base_attack: 85, base_defense: 200,
        base_sp_attack: 55, base_sp_defense: 65, base_speed: 30,
        catch_rate: 25, base_exp_yield: 196, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_SCREECH), (9, MOVE_ROCK_THROW), (13, MOVE_BIND), (21, MOVE_ROCK_SLIDE), (25, MOVE_SLAM), (33, MOVE_IRON_TAIL)],
        evolution_level: None, evolution_into: None,
    },
    // ─── Lighthouse / Jasmine species ───────────────────────
    SpeciesData {
        id: POLIWAG, name: "Poliwag",
        type1: PokemonType::Water, type2: None,
        base_hp: 40, base_attack: 50, base_defense: 40,
        base_sp_attack: 40, base_sp_defense: 40, base_speed: 90,
        catch_rate: 255, base_exp_yield: 77, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_BUBBLE), (7, MOVE_HYPNOSIS), (13, MOVE_WATER_GUN), (19, MOVE_DOUBLESLAP), (31, MOVE_BODY_SLAM)],
        evolution_level: Some(25), evolution_into: Some(POLIWHIRL),
    },
    SpeciesData {
        id: MARILL, name: "Marill",
        type1: PokemonType::Water, type2: None,
        base_hp: 70, base_attack: 20, base_defense: 50,
        base_sp_attack: 20, base_sp_defense: 50, base_speed: 40,
        catch_rate: 190, base_exp_yield: 58, growth_rate: GrowthRate::Fast,
        learnset: &[(1, MOVE_TACKLE), (3, MOVE_DEFENSE_CURL), (6, MOVE_TAIL_WHIP), (10, MOVE_WATER_GUN), (15, MOVE_ROLLOUT), (21, MOVE_BUBBLEBEAM)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: SPEAROW, name: "Spearow",
        type1: PokemonType::Normal, type2: Some(PokemonType::Flying),
        base_hp: 40, base_attack: 60, base_defense: 30,
        base_sp_attack: 31, base_sp_defense: 31, base_speed: 70,
        catch_rate: 255, base_exp_yield: 58, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_PECK), (1, MOVE_GROWL), (7, MOVE_LEER), (13, MOVE_FURY_ATTACK), (25, MOVE_PURSUIT)],
        evolution_level: Some(20), evolution_into: Some(FEAROW),
    },
    SpeciesData {
        id: FEAROW, name: "Fearow",
        type1: PokemonType::Normal, type2: Some(PokemonType::Flying),
        base_hp: 65, base_attack: 90, base_defense: 65,
        base_sp_attack: 61, base_sp_defense: 61, base_speed: 100,
        catch_rate: 90, base_exp_yield: 162, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_PECK), (1, MOVE_GROWL), (1, MOVE_LEER), (1, MOVE_FURY_ATTACK), (26, MOVE_PURSUIT), (32, MOVE_DRILL_PECK)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: MACHOP, name: "Machop",
        type1: PokemonType::Fighting, type2: None,
        base_hp: 70, base_attack: 80, base_defense: 50,
        base_sp_attack: 35, base_sp_defense: 35, base_speed: 35,
        catch_rate: 180, base_exp_yield: 70, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_LOW_KICK), (1, MOVE_LEER), (7, MOVE_FOCUS_ENERGY), (13, MOVE_KARATE_CHOP), (19, MOVE_SEISMIC_TOSS), (25, MOVE_FORESIGHT)],
        evolution_level: Some(28), evolution_into: None,
    },
    SpeciesData {
        id: AMPHAROS, name: "Ampharos",
        type1: PokemonType::Electric, type2: None,
        base_hp: 90, base_attack: 75, base_defense: 85,
        base_sp_attack: 115, base_sp_defense: 90, base_speed: 55,
        catch_rate: 45, base_exp_yield: 194, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_GROWL), (1, MOVE_THUNDER_SHOCK), (9, MOVE_THUNDER_SHOCK), (18, MOVE_THUNDER_WAVE), (27, MOVE_THUNDERBOLT)],
        evolution_level: None, evolution_into: None,
    },
    // ─── Route 40 / Cianwood species ─────────────────────
    SpeciesData {
        id: MANKEY, name: "Mankey",
        type1: PokemonType::Fighting, type2: None,
        base_hp: 40, base_attack: 80, base_defense: 35,
        base_sp_attack: 35, base_sp_defense: 45, base_speed: 70,
        catch_rate: 190, base_exp_yield: 74, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_SCRATCH), (1, MOVE_LEER), (9, MOVE_LOW_KICK), (15, MOVE_KARATE_CHOP), (21, MOVE_FURY_SWIPES), (27, MOVE_SEISMIC_TOSS)],
        evolution_level: Some(28), evolution_into: None,
    },
    SpeciesData {
        id: PRIMEAPE, name: "Primeape",
        type1: PokemonType::Fighting, type2: None,
        base_hp: 65, base_attack: 105, base_defense: 60,
        base_sp_attack: 60, base_sp_defense: 70, base_speed: 95,
        catch_rate: 75, base_exp_yield: 149, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_SCRATCH), (1, MOVE_LEER), (1, MOVE_LOW_KICK), (15, MOVE_KARATE_CHOP), (21, MOVE_FURY_SWIPES), (28, MOVE_CROSS_CHOP), (36, MOVE_SEISMIC_TOSS)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: POLIWRATH, name: "Poliwrath",
        type1: PokemonType::Water, type2: Some(PokemonType::Fighting),
        base_hp: 90, base_attack: 85, base_defense: 95,
        base_sp_attack: 70, base_sp_defense: 90, base_speed: 70,
        catch_rate: 45, base_exp_yield: 185, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_WATER_GUN), (1, MOVE_HYPNOSIS), (1, MOVE_DOUBLESLAP), (35, MOVE_SUBMISSION), (43, MOVE_DYNAMIC_PUNCH), (51, MOVE_SURF)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: TENTACOOL, name: "Tentacool",
        type1: PokemonType::Water, type2: Some(PokemonType::Poison),
        base_hp: 40, base_attack: 40, base_defense: 35,
        base_sp_attack: 50, base_sp_defense: 100, base_speed: 70,
        catch_rate: 190, base_exp_yield: 105, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_POISON_STING), (6, MOVE_SUPERSONIC), (12, MOVE_CONSTRICT), (19, MOVE_ACID), (25, MOVE_BUBBLEBEAM), (30, MOVE_WRAP)],
        evolution_level: Some(30), evolution_into: None,
    },
    SpeciesData {
        id: TENTACRUEL, name: "Tentacruel",
        type1: PokemonType::Water, type2: Some(PokemonType::Poison),
        base_hp: 80, base_attack: 70, base_defense: 65,
        base_sp_attack: 80, base_sp_defense: 120, base_speed: 100,
        catch_rate: 60, base_exp_yield: 205, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_POISON_STING), (1, MOVE_SUPERSONIC), (1, MOVE_CONSTRICT), (19, MOVE_ACID), (25, MOVE_BUBBLEBEAM), (30, MOVE_WRAP), (38, MOVE_SCREECH)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: MACHOKE, name: "Machoke",
        type1: PokemonType::Fighting, type2: None,
        base_hp: 80, base_attack: 100, base_defense: 70,
        base_sp_attack: 50, base_sp_defense: 60, base_speed: 45,
        catch_rate: 90, base_exp_yield: 146, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_LOW_KICK), (1, MOVE_LEER), (1, MOVE_FOCUS_ENERGY), (13, MOVE_KARATE_CHOP), (19, MOVE_SEISMIC_TOSS), (25, MOVE_FORESIGHT), (34, MOVE_SUBMISSION), (43, MOVE_CROSS_CHOP)],
        evolution_level: Some(38), evolution_into: None,
    },
    SpeciesData {
        id: MACHAMP, name: "Machamp",
        type1: PokemonType::Fighting, type2: None,
        base_hp: 90, base_attack: 130, base_defense: 80,
        base_sp_attack: 65, base_sp_defense: 85, base_speed: 55,
        catch_rate: 45, base_exp_yield: 193, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_LOW_KICK), (1, MOVE_LEER), (1, MOVE_FOCUS_ENERGY), (13, MOVE_KARATE_CHOP), (19, MOVE_SEISMIC_TOSS), (25, MOVE_FORESIGHT), (34, MOVE_SUBMISSION), (43, MOVE_CROSS_CHOP), (52, MOVE_DYNAMIC_PUNCH)],
        evolution_level: None, evolution_into: None,
    },
    // ─── Sprint 47: Mahogany / Ice species ──────────────────
    SpeciesData {
        id: SEEL, name: "Seel",
        type1: PokemonType::Water, type2: None,
        base_hp: 65, base_attack: 45, base_defense: 55,
        base_sp_attack: 45, base_sp_defense: 70, base_speed: 45,
        catch_rate: 190, base_exp_yield: 100, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_HEADBUTT), (9, MOVE_GROWL), (13, MOVE_ICY_WIND), (21, MOVE_AURORA_BEAM), (29, MOVE_REST), (37, MOVE_ICE_BEAM)],
        evolution_level: Some(34), evolution_into: Some(DEWGONG),
    },
    SpeciesData {
        id: DEWGONG, name: "Dewgong",
        type1: PokemonType::Water, type2: Some(PokemonType::Ice),
        base_hp: 90, base_attack: 70, base_defense: 80,
        base_sp_attack: 70, base_sp_defense: 95, base_speed: 70,
        catch_rate: 75, base_exp_yield: 176, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_HEADBUTT), (1, MOVE_GROWL), (1, MOVE_ICY_WIND), (1, MOVE_AURORA_BEAM), (29, MOVE_REST), (34, MOVE_TAKE_DOWN), (42, MOVE_ICE_BEAM)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: SWINUB, name: "Swinub",
        type1: PokemonType::Ice, type2: Some(PokemonType::Ground),
        base_hp: 50, base_attack: 50, base_defense: 40,
        base_sp_attack: 30, base_sp_defense: 30, base_speed: 50,
        catch_rate: 225, base_exp_yield: 78, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_POWDER_SNOW), (10, MOVE_MUD_SLAP), (19, MOVE_ENDURE), (28, MOVE_ICY_WIND), (37, MOVE_AMNESIA), (46, MOVE_BLIZZARD)],
        evolution_level: Some(33), evolution_into: Some(PILOSWINE),
    },
    SpeciesData {
        id: PILOSWINE, name: "Piloswine",
        type1: PokemonType::Ice, type2: Some(PokemonType::Ground),
        base_hp: 100, base_attack: 100, base_defense: 80,
        base_sp_attack: 60, base_sp_defense: 60, base_speed: 50,
        catch_rate: 75, base_exp_yield: 160, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_POWDER_SNOW), (1, MOVE_MUD_SLAP), (1, MOVE_ENDURE), (10, MOVE_MUD_SLAP), (19, MOVE_ENDURE), (28, MOVE_ICY_WIND), (33, MOVE_FURY_ATTACK), (42, MOVE_BLIZZARD), (56, MOVE_EARTHQUAKE)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: GIRAFARIG, name: "Girafarig",
        type1: PokemonType::Normal, type2: Some(PokemonType::Psychic),
        base_hp: 70, base_attack: 80, base_defense: 65,
        base_sp_attack: 90, base_sp_defense: 65, base_speed: 85,
        catch_rate: 60, base_exp_yield: 149, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_GROWL), (1, MOVE_CONFUSION), (7, MOVE_STOMP), (13, MOVE_PSYBEAM), (20, MOVE_FAINT_ATTACK), (30, MOVE_TAKE_DOWN)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: GOLBAT, name: "Golbat",
        type1: PokemonType::Poison, type2: Some(PokemonType::Flying),
        base_hp: 75, base_attack: 80, base_defense: 70,
        base_sp_attack: 65, base_sp_defense: 75, base_speed: 90,
        catch_rate: 90, base_exp_yield: 171, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_LEECH_LIFE), (1, MOVE_SUPERSONIC), (1, MOVE_BITE), (12, MOVE_CONFUSE_RAY), (19, MOVE_WING_ATTACK), (30, MOVE_MEAN_LOOK)],
        evolution_level: Some(36), evolution_into: Some(CROBAT),
    },
    SpeciesData {
        id: GYARADOS, name: "Gyarados",
        type1: PokemonType::Water, type2: Some(PokemonType::Flying),
        base_hp: 95, base_attack: 125, base_defense: 79,
        base_sp_attack: 60, base_sp_defense: 100, base_speed: 81,
        catch_rate: 45, base_exp_yield: 214, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_THRASH), (1, MOVE_BITE), (1, MOVE_DRAGON_RAGE), (1, MOVE_LEER), (20, MOVE_TWISTER), (25, MOVE_HYDRO_PUMP)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: GOLDEEN, name: "Goldeen",
        type1: PokemonType::Water, type2: None,
        base_hp: 45, base_attack: 67, base_defense: 60,
        base_sp_attack: 35, base_sp_defense: 50, base_speed: 63,
        catch_rate: 225, base_exp_yield: 111, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_PECK), (1, MOVE_TAIL_WHIP), (10, MOVE_SUPERSONIC), (15, MOVE_HORN_ATTACK), (24, MOVE_FURY_ATTACK), (33, MOVE_WATER_GUN)],
        evolution_level: Some(33), evolution_into: Some(SEAKING),
    },
    SpeciesData {
        id: SEAKING, name: "Seaking",
        type1: PokemonType::Water, type2: None,
        base_hp: 80, base_attack: 92, base_defense: 65,
        base_sp_attack: 65, base_sp_defense: 80, base_speed: 68,
        catch_rate: 60, base_exp_yield: 170, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_PECK), (1, MOVE_TAIL_WHIP), (1, MOVE_SUPERSONIC), (1, MOVE_HORN_ATTACK), (24, MOVE_FURY_ATTACK), (33, MOVE_WATER_GUN), (41, MOVE_HYDRO_PUMP)],
        evolution_level: None, evolution_into: None,
    },
    // ─── Sprint 50: Blackthorn / Phase 3 species ────────────
    SpeciesData {
        id: JYNX, name: "Jynx",
        type1: PokemonType::Ice, type2: Some(PokemonType::Psychic),
        base_hp: 65, base_attack: 50, base_defense: 35,
        base_sp_attack: 115, base_sp_defense: 95, base_speed: 95,
        catch_rate: 45, base_exp_yield: 137, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_POUND), (1, MOVE_LICK), (1, MOVE_LOVELY_KISS), (1, MOVE_POWDER_SNOW), (21, MOVE_DOUBLESLAP), (25, MOVE_ICE_PUNCH), (35, MOVE_MEAN_LOOK), (41, MOVE_BODY_SLAM), (51, MOVE_THRASH), (57, MOVE_BLIZZARD)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: SNEASEL, name: "Sneasel",
        type1: PokemonType::Dark, type2: Some(PokemonType::Ice),
        base_hp: 55, base_attack: 95, base_defense: 55,
        base_sp_attack: 35, base_sp_defense: 75, base_speed: 115,
        catch_rate: 60, base_exp_yield: 132, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_SCRATCH), (1, MOVE_LEER), (9, MOVE_QUICK_ATTACK), (17, MOVE_SCREECH), (25, MOVE_FAINT_ATTACK), (33, MOVE_FURY_SWIPES), (41, MOVE_AGILITY), (49, MOVE_SLASH)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: DELIBIRD, name: "Delibird",
        type1: PokemonType::Ice, type2: Some(PokemonType::Flying),
        base_hp: 45, base_attack: 55, base_defense: 45,
        base_sp_attack: 65, base_sp_defense: 45, base_speed: 75,
        catch_rate: 45, base_exp_yield: 183, growth_rate: GrowthRate::Fast,
        learnset: &[(1, MOVE_PRESENT)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: DRATINI, name: "Dratini",
        type1: PokemonType::Dragon, type2: None,
        base_hp: 41, base_attack: 64, base_defense: 45,
        base_sp_attack: 50, base_sp_defense: 50, base_speed: 50,
        catch_rate: 45, base_exp_yield: 67, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_WRAP), (1, MOVE_LEER), (8, MOVE_THUNDER_WAVE), (15, MOVE_TWISTER), (22, MOVE_DRAGON_RAGE), (29, MOVE_SLAM), (36, MOVE_AGILITY), (43, MOVE_SAFEGUARD), (50, MOVE_OUTRAGE), (57, MOVE_HYPER_BEAM)],
        evolution_level: Some(30), evolution_into: Some(DRAGONAIR),
    },
    SpeciesData {
        id: DRAGONAIR, name: "Dragonair",
        type1: PokemonType::Dragon, type2: None,
        base_hp: 61, base_attack: 84, base_defense: 65,
        base_sp_attack: 70, base_sp_defense: 70, base_speed: 70,
        catch_rate: 45, base_exp_yield: 144, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_WRAP), (1, MOVE_LEER), (1, MOVE_THUNDER_WAVE), (1, MOVE_TWISTER), (22, MOVE_DRAGON_RAGE), (29, MOVE_SLAM), (38, MOVE_AGILITY), (47, MOVE_SAFEGUARD), (56, MOVE_OUTRAGE), (65, MOVE_HYPER_BEAM)],
        evolution_level: Some(55), evolution_into: Some(DRAGONITE),
    },
    SpeciesData {
        id: DRAGONITE, name: "Dragonite",
        type1: PokemonType::Dragon, type2: Some(PokemonType::Flying),
        base_hp: 91, base_attack: 134, base_defense: 95,
        base_sp_attack: 100, base_sp_defense: 100, base_speed: 80,
        catch_rate: 45, base_exp_yield: 218, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_WRAP), (1, MOVE_LEER), (1, MOVE_THUNDER_WAVE), (1, MOVE_TWISTER), (22, MOVE_DRAGON_RAGE), (29, MOVE_SLAM), (38, MOVE_AGILITY), (47, MOVE_SAFEGUARD), (55, MOVE_WING_ATTACK), (61, MOVE_OUTRAGE), (75, MOVE_HYPER_BEAM)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: HORSEA, name: "Horsea",
        type1: PokemonType::Water, type2: None,
        base_hp: 30, base_attack: 40, base_defense: 70,
        base_sp_attack: 70, base_sp_defense: 25, base_speed: 60,
        catch_rate: 225, base_exp_yield: 83, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_BUBBLE), (8, MOVE_SMOKESCREEN), (15, MOVE_LEER), (22, MOVE_WATER_GUN), (29, MOVE_TWISTER), (36, MOVE_AGILITY), (43, MOVE_HYDRO_PUMP)],
        evolution_level: Some(32), evolution_into: Some(SEADRA),
    },
    SpeciesData {
        id: SEADRA, name: "Seadra",
        type1: PokemonType::Water, type2: None,
        base_hp: 55, base_attack: 65, base_defense: 95,
        base_sp_attack: 95, base_sp_defense: 45, base_speed: 85,
        catch_rate: 75, base_exp_yield: 155, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_BUBBLE), (1, MOVE_SMOKESCREEN), (1, MOVE_LEER), (1, MOVE_WATER_GUN), (29, MOVE_TWISTER), (40, MOVE_AGILITY), (51, MOVE_HYDRO_PUMP)],
        evolution_level: Some(38), evolution_into: Some(KINGDRA),
    },
    SpeciesData {
        id: KINGDRA, name: "Kingdra",
        type1: PokemonType::Water, type2: Some(PokemonType::Dragon),
        base_hp: 75, base_attack: 95, base_defense: 95,
        base_sp_attack: 95, base_sp_defense: 95, base_speed: 85,
        catch_rate: 45, base_exp_yield: 207, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_BUBBLE), (1, MOVE_SMOKESCREEN), (1, MOVE_LEER), (1, MOVE_WATER_GUN), (29, MOVE_TWISTER), (40, MOVE_AGILITY), (51, MOVE_HYDRO_PUMP)],
        evolution_level: None, evolution_into: None,
    },
    // ─── Sprint 52: Route 45/46 species ────────────────────
    SpeciesData {
        id: GRAVELER, name: "Graveler",
        type1: PokemonType::Rock, type2: Some(PokemonType::Ground),
        base_hp: 55, base_attack: 95, base_defense: 115,
        base_sp_attack: 45, base_sp_defense: 45, base_speed: 35,
        catch_rate: 120, base_exp_yield: 134, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_DEFENSE_CURL), (11, MOVE_ROCK_THROW), (16, MOVE_MAGNITUDE), (21, MOVE_SELF_DESTRUCT), (29, MOVE_ROCK_SLIDE), (36, MOVE_EARTHQUAKE)],
        evolution_level: Some(38), evolution_into: Some(GOLEM),
    },
    SpeciesData {
        id: GOLEM, name: "Golem",
        type1: PokemonType::Rock, type2: Some(PokemonType::Ground),
        base_hp: 80, base_attack: 110, base_defense: 130,
        base_sp_attack: 55, base_sp_defense: 65, base_speed: 45,
        catch_rate: 45, base_exp_yield: 177, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_DEFENSE_CURL), (1, MOVE_ROCK_THROW), (1, MOVE_MAGNITUDE), (21, MOVE_SELF_DESTRUCT), (29, MOVE_ROCK_SLIDE), (36, MOVE_EARTHQUAKE)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: GLIGAR, name: "Gligar",
        type1: PokemonType::Ground, type2: Some(PokemonType::Flying),
        base_hp: 65, base_attack: 75, base_defense: 105,
        base_sp_attack: 35, base_sp_defense: 65, base_speed: 85,
        catch_rate: 60, base_exp_yield: 108, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_POISON_STING), (6, MOVE_SAND_ATTACK), (13, MOVE_HARDEN), (20, MOVE_QUICK_ATTACK), (28, MOVE_FAINT_ATTACK), (36, MOVE_SLASH), (44, MOVE_SCREECH)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: TEDDIURSA, name: "Teddiursa",
        type1: PokemonType::Normal, type2: None,
        base_hp: 60, base_attack: 80, base_defense: 50,
        base_sp_attack: 50, base_sp_defense: 50, base_speed: 40,
        catch_rate: 120, base_exp_yield: 124, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_SCRATCH), (1, MOVE_LEER), (8, MOVE_LICK), (15, MOVE_FURY_SWIPES), (22, MOVE_FAINT_ATTACK), (29, MOVE_REST), (36, MOVE_SLASH), (43, MOVE_THRASH)],
        evolution_level: Some(30), evolution_into: Some(URSARING),
    },
    SpeciesData {
        id: URSARING, name: "Ursaring",
        type1: PokemonType::Normal, type2: None,
        base_hp: 90, base_attack: 130, base_defense: 75,
        base_sp_attack: 75, base_sp_defense: 75, base_speed: 55,
        catch_rate: 60, base_exp_yield: 189, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_SCRATCH), (1, MOVE_LEER), (1, MOVE_LICK), (1, MOVE_FURY_SWIPES), (22, MOVE_FAINT_ATTACK), (29, MOVE_REST), (36, MOVE_SLASH), (43, MOVE_THRASH)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: SKARMORY, name: "Skarmory",
        type1: PokemonType::Steel, type2: Some(PokemonType::Flying),
        base_hp: 65, base_attack: 80, base_defense: 140,
        base_sp_attack: 40, base_sp_defense: 70, base_speed: 70,
        catch_rate: 25, base_exp_yield: 168, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_LEER), (1, MOVE_PECK), (13, MOVE_SAND_ATTACK), (19, MOVE_SWIFT), (25, MOVE_AGILITY), (31, MOVE_FURY_ATTACK), (37, MOVE_SLASH), (43, MOVE_STEEL_WING)],
        evolution_level: None, evolution_into: None,
    },
    // ─── Sprint 55: Route 27/26 species ──────────────────
    SpeciesData {
        id: PONYTA, name: "Ponyta",
        type1: PokemonType::Fire, type2: None,
        base_hp: 50, base_attack: 85, base_defense: 55,
        base_sp_attack: 65, base_sp_defense: 65, base_speed: 90,
        catch_rate: 190, base_exp_yield: 152, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (4, MOVE_GROWL), (8, MOVE_TAIL_WHIP), (13, MOVE_EMBER), (19, MOVE_STOMP), (26, MOVE_FIRE_SPIN), (34, MOVE_TAKE_DOWN), (40, MOVE_FURY_ATTACK), (47, MOVE_AGILITY), (61, MOVE_FIRE_BLAST)],
        evolution_level: Some(40), evolution_into: Some(RAPIDASH),
    },
    SpeciesData {
        id: RAPIDASH, name: "Rapidash",
        type1: PokemonType::Fire, type2: None,
        base_hp: 65, base_attack: 100, base_defense: 70,
        base_sp_attack: 80, base_sp_defense: 80, base_speed: 105,
        catch_rate: 60, base_exp_yield: 192, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_GROWL), (1, MOVE_TAIL_WHIP), (1, MOVE_EMBER), (19, MOVE_STOMP), (26, MOVE_FIRE_SPIN), (34, MOVE_TAKE_DOWN), (40, MOVE_FURY_ATTACK), (47, MOVE_AGILITY), (61, MOVE_FIRE_BLAST)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: SANDSHREW, name: "Sandshrew",
        type1: PokemonType::Ground, type2: None,
        base_hp: 50, base_attack: 75, base_defense: 85,
        base_sp_attack: 20, base_sp_defense: 30, base_speed: 40,
        catch_rate: 255, base_exp_yield: 93, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_SCRATCH), (6, MOVE_DEFENSE_CURL), (11, MOVE_SAND_ATTACK), (17, MOVE_POISON_STING), (23, MOVE_SLASH), (30, MOVE_SWIFT), (37, MOVE_FURY_SWIPES), (45, MOVE_EARTHQUAKE)],
        evolution_level: Some(22), evolution_into: Some(SANDSLASH),
    },
    SpeciesData {
        id: SANDSLASH, name: "Sandslash",
        type1: PokemonType::Ground, type2: None,
        base_hp: 75, base_attack: 100, base_defense: 110,
        base_sp_attack: 45, base_sp_defense: 55, base_speed: 65,
        catch_rate: 90, base_exp_yield: 163, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_SCRATCH), (1, MOVE_DEFENSE_CURL), (1, MOVE_SAND_ATTACK), (17, MOVE_POISON_STING), (24, MOVE_SLASH), (33, MOVE_SWIFT), (42, MOVE_FURY_SWIPES), (52, MOVE_EARTHQUAKE)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: DODRIO, name: "Dodrio",
        type1: PokemonType::Normal, type2: Some(PokemonType::Flying),
        base_hp: 60, base_attack: 110, base_defense: 70,
        base_sp_attack: 60, base_sp_defense: 60, base_speed: 100,
        catch_rate: 45, base_exp_yield: 158, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_PECK), (1, MOVE_GROWL), (1, MOVE_PURSUIT), (1, MOVE_FURY_ATTACK), (21, MOVE_TRI_ATTACK), (25, MOVE_RAGE), (38, MOVE_DRILL_PECK), (47, MOVE_AGILITY)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: ARCANINE, name: "Arcanine",
        type1: PokemonType::Fire, type2: None,
        base_hp: 90, base_attack: 110, base_defense: 80,
        base_sp_attack: 100, base_sp_defense: 80, base_speed: 95,
        catch_rate: 75, base_exp_yield: 213, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_ROAR), (1, MOVE_LEER), (1, MOVE_TAKE_DOWN), (1, MOVE_FLAME_WHEEL), (50, MOVE_EXTREME_SPEED)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: QUAGSIRE, name: "Quagsire",
        type1: PokemonType::Water, type2: Some(PokemonType::Ground),
        base_hp: 95, base_attack: 85, base_defense: 85,
        base_sp_attack: 65, base_sp_defense: 65, base_speed: 35,
        catch_rate: 90, base_exp_yield: 137, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_WATER_GUN), (1, MOVE_TAIL_WHIP), (11, MOVE_SLAM), (23, MOVE_AMNESIA), (35, MOVE_EARTHQUAKE), (47, MOVE_HAZE)],
        evolution_level: None, evolution_into: None,
    },
    // ─── Sprint 56: E4 / Victory Road species ────────────────
    SpeciesData {
        id: XATU, name: "Xatu",
        type1: PokemonType::Psychic, type2: Some(PokemonType::Flying),
        base_hp: 65, base_attack: 75, base_defense: 70,
        base_sp_attack: 95, base_sp_defense: 70, base_speed: 95,
        catch_rate: 75, base_exp_yield: 147, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_PECK), (1, MOVE_LEER), (10, MOVE_NIGHT_SHADE), (20, MOVE_TELEPORT), (35, MOVE_CONFUSION), (50, MOVE_CONFUSE_RAY), (65, MOVE_PSYCHIC)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: SLOWBRO, name: "Slowbro",
        type1: PokemonType::Water, type2: Some(PokemonType::Psychic),
        base_hp: 95, base_attack: 75, base_defense: 110,
        base_sp_attack: 100, base_sp_defense: 80, base_speed: 30,
        catch_rate: 75, base_exp_yield: 164, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_GROWL), (1, MOVE_WATER_GUN), (20, MOVE_CONFUSION), (29, MOVE_DISABLE), (34, MOVE_HEADBUTT), (46, MOVE_AMNESIA), (54, MOVE_PSYCHIC)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: EXEGGUTOR, name: "Exeggutor",
        type1: PokemonType::Grass, type2: Some(PokemonType::Psychic),
        base_hp: 95, base_attack: 95, base_defense: 85,
        base_sp_attack: 125, base_sp_defense: 65, base_speed: 55,
        catch_rate: 45, base_exp_yield: 212, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_HYPNOSIS), (1, MOVE_CONFUSION), (19, MOVE_STOMP), (31, MOVE_PSYCHIC)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: ARIADOS, name: "Ariados",
        type1: PokemonType::Bug, type2: Some(PokemonType::Poison),
        base_hp: 70, base_attack: 90, base_defense: 70,
        base_sp_attack: 60, base_sp_defense: 60, base_speed: 40,
        catch_rate: 90, base_exp_yield: 140, growth_rate: GrowthRate::Fast,
        learnset: &[(1, MOVE_POISON_STING), (1, MOVE_STRING_SHOT), (1, MOVE_SCARY_FACE), (1, MOVE_CONSTRICT), (17, MOVE_NIGHT_SHADE), (25, MOVE_LEECH_LIFE), (34, MOVE_FURY_SWIPES), (53, MOVE_SCREECH), (53, MOVE_AGILITY)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: FORRETRESS, name: "Forretress",
        type1: PokemonType::Bug, type2: Some(PokemonType::Steel),
        base_hp: 75, base_attack: 90, base_defense: 140,
        base_sp_attack: 60, base_sp_defense: 60, base_speed: 40,
        catch_rate: 75, base_exp_yield: 163, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_SELF_DESTRUCT), (22, MOVE_TAKE_DOWN), (36, MOVE_SWIFT)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: MUK, name: "Muk",
        type1: PokemonType::Poison, type2: None,
        base_hp: 105, base_attack: 105, base_defense: 75,
        base_sp_attack: 65, base_sp_defense: 100, base_speed: 50,
        catch_rate: 75, base_exp_yield: 157, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_POUND), (1, MOVE_HARDEN), (23, MOVE_SLUDGE), (31, MOVE_SCREECH), (45, MOVE_ACID)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: VENOMOTH, name: "Venomoth",
        type1: PokemonType::Bug, type2: Some(PokemonType::Poison),
        base_hp: 70, base_attack: 65, base_defense: 60,
        base_sp_attack: 90, base_sp_defense: 75, base_speed: 90,
        catch_rate: 75, base_exp_yield: 138, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (1, MOVE_DISABLE), (1, MOVE_FORESIGHT), (1, MOVE_SUPERSONIC), (13, MOVE_CONFUSION), (17, MOVE_POISON_POWDER), (20, MOVE_LEECH_LIFE), (28, MOVE_STUN_SPORE), (38, MOVE_PSYBEAM), (48, MOVE_SLEEP_POWDER), (55, MOVE_PSYCHIC)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: CROBAT, name: "Crobat",
        type1: PokemonType::Poison, type2: Some(PokemonType::Flying),
        base_hp: 85, base_attack: 90, base_defense: 80,
        base_sp_attack: 70, base_sp_defense: 80, base_speed: 130,
        catch_rate: 90, base_exp_yield: 204, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_SCREECH), (1, MOVE_LEECH_LIFE), (1, MOVE_SUPERSONIC), (1, MOVE_BITE), (12, MOVE_BITE), (19, MOVE_CONFUSE_RAY), (30, MOVE_WING_ATTACK), (42, MOVE_MEAN_LOOK)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: HITMONTOP, name: "Hitmontop",
        type1: PokemonType::Fighting, type2: None,
        base_hp: 50, base_attack: 95, base_defense: 95,
        base_sp_attack: 35, base_sp_defense: 110, base_speed: 70,
        catch_rate: 45, base_exp_yield: 138, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_TACKLE), (7, MOVE_FOCUS_ENERGY), (13, MOVE_PURSUIT), (19, MOVE_QUICK_ATTACK), (25, MOVE_DOUBLE_KICK), (31, MOVE_TAKE_DOWN)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: HITMONLEE, name: "Hitmonlee",
        type1: PokemonType::Fighting, type2: None,
        base_hp: 50, base_attack: 120, base_defense: 53,
        base_sp_attack: 35, base_sp_defense: 110, base_speed: 87,
        catch_rate: 45, base_exp_yield: 139, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_DOUBLE_KICK), (6, MOVE_MEDITATE), (16, MOVE_LOW_KICK), (21, MOVE_FOCUS_ENERGY), (31, MOVE_FORESIGHT), (41, MOVE_ENDURE)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: HITMONCHAN, name: "Hitmonchan",
        type1: PokemonType::Fighting, type2: None,
        base_hp: 50, base_attack: 105, base_defense: 79,
        base_sp_attack: 35, base_sp_defense: 110, base_speed: 76,
        catch_rate: 45, base_exp_yield: 140, growth_rate: GrowthRate::MediumFast,
        learnset: &[(1, MOVE_POUND), (7, MOVE_AGILITY), (13, MOVE_PURSUIT), (26, MOVE_FIRE_PUNCH), (26, MOVE_ICE_PUNCH), (38, MOVE_TAKE_DOWN)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: VILEPLUME, name: "Vileplume",
        type1: PokemonType::Grass, type2: Some(PokemonType::Poison),
        base_hp: 75, base_attack: 80, base_defense: 85,
        base_sp_attack: 100, base_sp_defense: 90, base_speed: 50,
        catch_rate: 45, base_exp_yield: 184, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_ABSORB), (1, MOVE_STUN_SPORE), (1, MOVE_ACID), (1, MOVE_SLEEP_POWDER)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: MURKROW, name: "Murkrow",
        type1: PokemonType::Dark, type2: Some(PokemonType::Flying),
        base_hp: 60, base_attack: 85, base_defense: 42,
        base_sp_attack: 85, base_sp_defense: 42, base_speed: 91,
        catch_rate: 30, base_exp_yield: 107, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_PECK), (11, MOVE_PURSUIT), (16, MOVE_HAZE), (25, MOVE_NIGHT_SHADE), (35, MOVE_FAINT_ATTACK), (45, MOVE_MEAN_LOOK)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: HOUNDOOM, name: "Houndoom",
        type1: PokemonType::Dark, type2: Some(PokemonType::Fire),
        base_hp: 75, base_attack: 90, base_defense: 50,
        base_sp_attack: 110, base_sp_defense: 80, base_speed: 95,
        catch_rate: 45, base_exp_yield: 204, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_LEER), (1, MOVE_EMBER), (7, MOVE_ROAR), (13, MOVE_SMOG), (19, MOVE_BITE), (26, MOVE_FAINT_ATTACK), (35, MOVE_FLAMETHROWER), (44, MOVE_CRUNCH)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: AERODACTYL, name: "Aerodactyl",
        type1: PokemonType::Rock, type2: Some(PokemonType::Flying),
        base_hp: 80, base_attack: 105, base_defense: 65,
        base_sp_attack: 60, base_sp_defense: 75, base_speed: 130,
        catch_rate: 45, base_exp_yield: 202, growth_rate: GrowthRate::Slow,
        learnset: &[(1, MOVE_WING_ATTACK), (8, MOVE_AGILITY), (15, MOVE_BITE), (22, MOVE_SUPERSONIC), (29, MOVE_SCARY_FACE), (36, MOVE_TAKE_DOWN), (43, MOVE_HYPER_BEAM)],
        evolution_level: None, evolution_into: None,
    },
    SpeciesData {
        id: CHARIZARD, name: "Charizard",
        type1: PokemonType::Fire, type2: Some(PokemonType::Flying),
        base_hp: 78, base_attack: 84, base_defense: 78,
        base_sp_attack: 109, base_sp_defense: 85, base_speed: 100,
        catch_rate: 45, base_exp_yield: 209, growth_rate: GrowthRate::MediumSlow,
        learnset: &[(1, MOVE_SCRATCH), (1, MOVE_GROWL), (1, MOVE_EMBER), (1, MOVE_RAGE), (20, MOVE_SCARY_FACE), (27, MOVE_FLAMETHROWER), (34, MOVE_WING_ATTACK), (36, MOVE_SLASH), (44, MOVE_DRAGON_RAGE), (54, MOVE_FIRE_SPIN)],
        evolution_level: None, evolution_into: None,
    },
];

// ─── Move Database ──────────────────────────────────────

const MOVE_DB: &[MoveData] = &[
    MoveData { id: MOVE_TACKLE, name: "Tackle", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 35, accuracy: 95, pp: 35, description: "A full-body charge attack." },
    MoveData { id: MOVE_SCRATCH, name: "Scratch", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 40, accuracy: 100, pp: 35, description: "Scratches with sharp claws." },
    MoveData { id: MOVE_GROWL, name: "Growl", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 40, description: "Reduces the foe's Attack." },
    MoveData { id: MOVE_LEER, name: "Leer", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 30, description: "Reduces the foe's Defense." },
    MoveData { id: MOVE_TAIL_WHIP, name: "Tail Whip", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 30, description: "Lowers the foe's Defense." },
    MoveData { id: MOVE_RAZOR_LEAF, name: "Razor Leaf", move_type: PokemonType::Grass, category: MoveCategory::Special, power: 55, accuracy: 95, pp: 25, description: "Leaves are launched to slash." },
    MoveData { id: MOVE_VINE_WHIP, name: "Vine Whip", move_type: PokemonType::Grass, category: MoveCategory::Special, power: 35, accuracy: 100, pp: 10, description: "Strikes with slender vines." },
    MoveData { id: MOVE_EMBER, name: "Ember", move_type: PokemonType::Fire, category: MoveCategory::Special, power: 40, accuracy: 100, pp: 25, description: "A weak fire attack." },
    MoveData { id: MOVE_SMOKESCREEN, name: "Smokescreen", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 20, description: "Lowers the foe's accuracy." },
    MoveData { id: MOVE_WATER_GUN, name: "Water Gun", move_type: PokemonType::Water, category: MoveCategory::Special, power: 40, accuracy: 100, pp: 25, description: "Squirts water to attack." },
    MoveData { id: MOVE_RAGE, name: "Rage", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 20, accuracy: 100, pp: 20, description: "Raises Attack if hit." },
    MoveData { id: MOVE_BITE, name: "Bite", move_type: PokemonType::Dark, category: MoveCategory::Special, power: 60, accuracy: 100, pp: 25, description: "May cause flinching." },
    MoveData { id: MOVE_QUICK_ATTACK, name: "Quick Attack", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 40, accuracy: 100, pp: 30, description: "Always strikes first." },
    MoveData { id: MOVE_GUST, name: "Gust", move_type: PokemonType::Flying, category: MoveCategory::Physical, power: 40, accuracy: 100, pp: 35, description: "Whips up a small gust." },
    MoveData { id: MOVE_PECK, name: "Peck", move_type: PokemonType::Flying, category: MoveCategory::Physical, power: 35, accuracy: 100, pp: 35, description: "Jabs the foe with a beak." },
    MoveData { id: MOVE_FORESIGHT, name: "Foresight", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 40, description: "Negates accuracy reduction." },
    MoveData { id: MOVE_STRING_SHOT, name: "String Shot", move_type: PokemonType::Bug, category: MoveCategory::Status, power: 0, accuracy: 95, pp: 40, description: "Reduces the foe's Speed." },
    MoveData { id: MOVE_POISON_STING, name: "Poison Sting", move_type: PokemonType::Poison, category: MoveCategory::Physical, power: 15, accuracy: 100, pp: 35, description: "May poison the foe." },
    MoveData { id: MOVE_DEFENSE_CURL, name: "Defense Curl", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 40, description: "Raises the user's Defense." },
    MoveData { id: MOVE_SAND_ATTACK, name: "Sand Attack", move_type: PokemonType::Ground, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 15, description: "Reduces the foe's accuracy." },
    MoveData { id: MOVE_BIND, name: "Bind", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 15, accuracy: 75, pp: 20, description: "Binds the foe for 2-5 turns." },
    MoveData { id: MOVE_THUNDER_SHOCK, name: "ThunderShock", move_type: PokemonType::Electric, category: MoveCategory::Special, power: 40, accuracy: 100, pp: 30, description: "May paralyze the foe." },
    MoveData { id: MOVE_ROCK_THROW, name: "Rock Throw", move_type: PokemonType::Rock, category: MoveCategory::Physical, power: 50, accuracy: 90, pp: 15, description: "Drops rocks on the foe." },
    MoveData { id: MOVE_HYPNOSIS, name: "Hypnosis", move_type: PokemonType::Psychic, category: MoveCategory::Status, power: 0, accuracy: 60, pp: 20, description: "May put the foe to sleep." },
    MoveData { id: MOVE_NIGHT_SHADE, name: "Night Shade", move_type: PokemonType::Ghost, category: MoveCategory::Physical, power: 1, accuracy: 100, pp: 15, description: "Inflicts damage equal to level." },
    MoveData { id: MOVE_LICK, name: "Lick", move_type: PokemonType::Ghost, category: MoveCategory::Physical, power: 20, accuracy: 100, pp: 30, description: "May paralyze the foe." },
    MoveData { id: MOVE_SPLASH, name: "Splash", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 40, description: "Has no effect whatsoever." },
    MoveData { id: MOVE_SCARY_FACE, name: "Scary Face", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 90, pp: 10, description: "Sharply reduces Speed." },
    MoveData { id: MOVE_LEECH_LIFE, name: "Leech Life", move_type: PokemonType::Bug, category: MoveCategory::Physical, power: 20, accuracy: 100, pp: 15, description: "Drains HP from the foe." },
    MoveData { id: MOVE_MUD_SLAP, name: "Mud-Slap", move_type: PokemonType::Ground, category: MoveCategory::Physical, power: 20, accuracy: 100, pp: 10, description: "Reduces the foe's accuracy." },
    // ─── New moves for levels 15-25 and new species ──────
    MoveData { id: MOVE_ABSORB, name: "Absorb", move_type: PokemonType::Grass, category: MoveCategory::Special, power: 20, accuracy: 100, pp: 25, description: "Drains HP from the foe." },
    MoveData { id: MOVE_CONFUSION, name: "Confusion", move_type: PokemonType::Psychic, category: MoveCategory::Special, power: 50, accuracy: 100, pp: 25, description: "May confuse the foe." },
    MoveData { id: MOVE_POUND, name: "Pound", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 40, accuracy: 100, pp: 35, description: "Pounds with forelegs or tail." },
    MoveData { id: MOVE_TELEPORT, name: "Teleport", move_type: PokemonType::Psychic, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 20, description: "Flee from wild battles." },
    MoveData { id: MOVE_TRANSFORM, name: "Transform", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 10, description: "Copies the foe's stats." },
    MoveData { id: MOVE_POISON_POWDER, name: "PoisonPowder", move_type: PokemonType::Poison, category: MoveCategory::Status, power: 0, accuracy: 75, pp: 35, description: "May poison the foe." },
    MoveData { id: MOVE_STUN_SPORE, name: "Stun Spore", move_type: PokemonType::Grass, category: MoveCategory::Status, power: 0, accuracy: 75, pp: 30, description: "May paralyze the foe." },
    MoveData { id: MOVE_SLEEP_POWDER, name: "Sleep Powder", move_type: PokemonType::Grass, category: MoveCategory::Status, power: 0, accuracy: 75, pp: 15, description: "May put the foe to sleep." },
    MoveData { id: MOVE_ACID, name: "Acid", move_type: PokemonType::Poison, category: MoveCategory::Physical, power: 40, accuracy: 100, pp: 30, description: "May lower Defense." },
    MoveData { id: MOVE_FURY_SWIPES, name: "Fury Swipes", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 18, accuracy: 80, pp: 15, description: "Rakes with claws 2-5 times." },
    MoveData { id: MOVE_SLAM, name: "Slam", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 80, accuracy: 75, pp: 20, description: "Slams the foe with a tail." },
    MoveData { id: MOVE_CONFUSE_RAY, name: "Confuse Ray", move_type: PokemonType::Ghost, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 10, description: "Confuses the foe." },
    MoveData { id: MOVE_MAGNITUDE, name: "Magnitude", move_type: PokemonType::Ground, category: MoveCategory::Physical, power: 70, accuracy: 100, pp: 30, description: "Ground quake of random power." },
    MoveData { id: MOVE_HYPER_FANG, name: "Hyper Fang", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 80, accuracy: 90, pp: 15, description: "May cause flinching." },
    MoveData { id: MOVE_DOUBLE_KICK, name: "Double Kick", move_type: PokemonType::Fighting, category: MoveCategory::Physical, power: 30, accuracy: 100, pp: 30, description: "Kicks the foe twice." },
    // ─── Whitney / Goldenrod Gym moves ───────────────────
    MoveData { id: MOVE_ROLLOUT, name: "Rollout", move_type: PokemonType::Rock, category: MoveCategory::Physical, power: 30, accuracy: 90, pp: 20, description: "Power doubles per hit." },
    MoveData { id: MOVE_ATTRACT, name: "Attract", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 15, description: "Infatuates the foe." },
    MoveData { id: MOVE_STOMP, name: "Stomp", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 65, accuracy: 100, pp: 20, description: "May cause flinching." },
    MoveData { id: MOVE_MILK_DRINK, name: "Milk Drink", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 10, description: "Restores half max HP." },
    MoveData { id: MOVE_DOUBLESLAP, name: "DoubleSlap", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 15, accuracy: 85, pp: 10, description: "Hits 2-5 times." },
    MoveData { id: MOVE_METRONOME, name: "Metronome", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 10, description: "Uses a random move." },
    MoveData { id: MOVE_SING, name: "Sing", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 55, pp: 15, description: "May put the foe to sleep." },
    MoveData { id: MOVE_DISABLE, name: "Disable", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 55, pp: 20, description: "Disables the foe's move." },
    MoveData { id: MOVE_ENCORE, name: "Encore", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 5, description: "Forces move repetition." },
    // ─── Morty / Ecruteak + Route 35-37 moves ──────────────
    MoveData { id: MOVE_SHADOW_BALL, name: "Shadow Ball", move_type: PokemonType::Ghost, category: MoveCategory::Physical, power: 80, accuracy: 100, pp: 15, description: "May lower Sp.Def." },
    MoveData { id: MOVE_DREAM_EATER, name: "Dream Eater", move_type: PokemonType::Psychic, category: MoveCategory::Special, power: 100, accuracy: 100, pp: 15, description: "Drains a sleeping foe." },
    MoveData { id: MOVE_SPITE, name: "Spite", move_type: PokemonType::Ghost, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 10, description: "Cuts the foe's PP." },
    MoveData { id: MOVE_MEAN_LOOK, name: "Mean Look", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 5, description: "Prevents the foe from fleeing." },
    MoveData { id: MOVE_CURSE, name: "Curse", move_type: PokemonType::Ghost, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 10, description: "Works differently for Ghosts." },
    MoveData { id: MOVE_MIMIC, name: "Mimic", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 10, description: "Copies a foe's move." },
    MoveData { id: MOVE_HORN_ATTACK, name: "Horn Attack", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 65, accuracy: 100, pp: 25, description: "Jabs with a sharp horn." },
    MoveData { id: MOVE_FOCUS_ENERGY, name: "Focus Energy", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 30, description: "Raises critical hit ratio." },
    MoveData { id: MOVE_TAKE_DOWN, name: "Take Down", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 90, accuracy: 85, pp: 20, description: "A tackle that also hurts." },
    MoveData { id: MOVE_ROAR, name: "Roar", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 20, description: "Scares wild foe away." },
    MoveData { id: MOVE_FLAMETHROWER, name: "Flamethrower", move_type: PokemonType::Fire, category: MoveCategory::Special, power: 95, accuracy: 100, pp: 15, description: "May burn the foe." },
    MoveData { id: MOVE_FIRE_SPIN, name: "Fire Spin", move_type: PokemonType::Fire, category: MoveCategory::Special, power: 15, accuracy: 70, pp: 15, description: "Traps foe in fire 2-5 turns." },
    MoveData { id: MOVE_SUPERSONIC, name: "Supersonic", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 55, pp: 20, description: "May confuse the foe." },
    MoveData { id: MOVE_SONIC_BOOM, name: "Sonic Boom", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 1, accuracy: 90, pp: 20, description: "Always inflicts 20 HP damage." },
    MoveData { id: MOVE_PSYBEAM, name: "Psybeam", move_type: PokemonType::Psychic, category: MoveCategory::Special, power: 65, accuracy: 100, pp: 20, description: "May confuse the foe." },
    MoveData { id: MOVE_LOW_KICK, name: "Low Kick", move_type: PokemonType::Fighting, category: MoveCategory::Physical, power: 50, accuracy: 90, pp: 20, description: "A low, tripping kick." },
    MoveData { id: MOVE_FLAIL, name: "Flail", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 1, accuracy: 100, pp: 15, description: "Stronger when HP is low." },
    MoveData { id: MOVE_ROCK_SLIDE, name: "Rock Slide", move_type: PokemonType::Rock, category: MoveCategory::Physical, power: 75, accuracy: 90, pp: 10, description: "May cause flinching." },
    MoveData { id: MOVE_FURY_ATTACK, name: "Fury Attack", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 15, accuracy: 85, pp: 20, description: "Jabs the foe 2-5 times." },
    // ─── Ecruteak / Burned Tower / Eeveelution moves ────
    MoveData { id: MOVE_SMOG, name: "Smog", move_type: PokemonType::Poison, category: MoveCategory::Physical, power: 20, accuracy: 70, pp: 20, description: "May poison the foe." },
    MoveData { id: MOVE_SLUDGE, name: "Sludge", move_type: PokemonType::Poison, category: MoveCategory::Physical, power: 65, accuracy: 100, pp: 20, description: "May poison the foe." },
    MoveData { id: MOVE_SELF_DESTRUCT, name: "Selfdestruct", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 200, accuracy: 100, pp: 5, description: "Faints user; hurts foe hard." },
    MoveData { id: MOVE_HAZE, name: "Haze", move_type: PokemonType::Ice, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 30, description: "Resets all stat changes." },
    MoveData { id: MOVE_PURSUIT, name: "Pursuit", move_type: PokemonType::Dark, category: MoveCategory::Special, power: 40, accuracy: 100, pp: 20, description: "Hits hard on switch-out." },
    MoveData { id: MOVE_FIRE_PUNCH, name: "Fire Punch", move_type: PokemonType::Fire, category: MoveCategory::Special, power: 75, accuracy: 100, pp: 15, description: "May burn the foe." },
    // ─── Route 38-39 / Olivine moves ────────────────────────
    MoveData { id: MOVE_THUNDER_WAVE, name: "Thunder Wave", move_type: PokemonType::Electric, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 20, description: "May paralyze the foe." },
    MoveData { id: MOVE_BUBBLE, name: "Bubble", move_type: PokemonType::Water, category: MoveCategory::Special, power: 20, accuracy: 100, pp: 30, description: "May lower the foe's Speed." },
    MoveData { id: MOVE_HARDEN, name: "Harden", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 30, description: "Raises the user's Defense." },
    MoveData { id: MOVE_BARRIER, name: "Barrier", move_type: PokemonType::Psychic, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 30, description: "Sharply raises Defense." },
    MoveData { id: MOVE_MEDITATE, name: "Meditate", move_type: PokemonType::Psychic, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 40, description: "Raises the user's Attack." },
    MoveData { id: MOVE_VICEGRIP, name: "ViceGrip", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 55, accuracy: 100, pp: 30, description: "Grips with large pincers." },
    MoveData { id: MOVE_DOUBLE_TEAM, name: "Double Team", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 15, description: "Raises the user's evasion." },
    MoveData { id: MOVE_RECOVER, name: "Recover", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 20, description: "Restores half max HP." },
    MoveData { id: MOVE_SWORDS_DANCE, name: "Swords Dance", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 30, description: "Sharply raises Attack." },
    MoveData { id: MOVE_WHIRLWIND, name: "Whirlwind", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 20, description: "Blows away the foe in wild." },
    MoveData { id: MOVE_WING_ATTACK, name: "Wing Attack", move_type: PokemonType::Flying, category: MoveCategory::Physical, power: 60, accuracy: 100, pp: 35, description: "Strikes with spread wings." },
    MoveData { id: MOVE_TRI_ATTACK, name: "Tri Attack", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 80, accuracy: 100, pp: 10, description: "May paralyze, burn, or freeze." },
    MoveData { id: MOVE_DRILL_PECK, name: "Drill Peck", move_type: PokemonType::Flying, category: MoveCategory::Physical, power: 80, accuracy: 100, pp: 20, description: "A spiraling, drilling peck." },
    MoveData { id: MOVE_BUBBLEBEAM, name: "BubbleBeam", move_type: PokemonType::Water, category: MoveCategory::Special, power: 65, accuracy: 100, pp: 20, description: "May lower the foe's Speed." },
    MoveData { id: MOVE_THUNDERBOLT, name: "Thunderbolt", move_type: PokemonType::Electric, category: MoveCategory::Special, power: 95, accuracy: 100, pp: 15, description: "May paralyze the foe." },
    MoveData { id: MOVE_FAINT_ATTACK, name: "Faint Attack", move_type: PokemonType::Dark, category: MoveCategory::Special, power: 60, accuracy: 255, pp: 20, description: "Never misses the foe." },
    MoveData { id: MOVE_PAY_DAY, name: "Pay Day", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 40, accuracy: 100, pp: 20, description: "Scatters coins after battle." },
    MoveData { id: MOVE_IRON_TAIL, name: "Iron Tail", move_type: PokemonType::Steel, category: MoveCategory::Physical, power: 100, accuracy: 75, pp: 15, description: "Attacks with a steel tail." },
    MoveData { id: MOVE_SCREECH, name: "Screech", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 85, pp: 40, description: "Sharply lowers Defense." },
    MoveData { id: MOVE_SUNNY_DAY, name: "Sunny Day", move_type: PokemonType::Fire, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 5, description: "Boosts Fire moves for 5 turns." },
    // ─── Lighthouse / Jasmine moves ─────────────────────────
    MoveData { id: MOVE_KARATE_CHOP, name: "Karate Chop", move_type: PokemonType::Fighting, category: MoveCategory::Physical, power: 50, accuracy: 100, pp: 25, description: "High critical-hit ratio." },
    MoveData { id: MOVE_BODY_SLAM, name: "Body Slam", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 85, accuracy: 100, pp: 15, description: "May paralyze the foe." },
    MoveData { id: MOVE_SEISMIC_TOSS, name: "Seismic Toss", move_type: PokemonType::Fighting, category: MoveCategory::Physical, power: 1, accuracy: 100, pp: 20, description: "Damage equals user's level." },
    // ─── Route 40 / Cianwood moves ──────────────────────────
    MoveData { id: MOVE_CROSS_CHOP, name: "Cross Chop", move_type: PokemonType::Fighting, category: MoveCategory::Physical, power: 100, accuracy: 80, pp: 5, description: "High critical-hit ratio." },
    MoveData { id: MOVE_SUBMISSION, name: "Submission", move_type: PokemonType::Fighting, category: MoveCategory::Physical, power: 80, accuracy: 80, pp: 25, description: "Also hurts the user a little." },
    MoveData { id: MOVE_DYNAMIC_PUNCH, name: "DynamicPunch", move_type: PokemonType::Fighting, category: MoveCategory::Physical, power: 100, accuracy: 50, pp: 5, description: "Always confuses if it hits." },
    MoveData { id: MOVE_SURF, name: "Surf", move_type: PokemonType::Water, category: MoveCategory::Special, power: 95, accuracy: 100, pp: 15, description: "A big wave hits all." },
    MoveData { id: MOVE_CONSTRICT, name: "Constrict", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 10, accuracy: 100, pp: 35, description: "May lower Speed." },
    MoveData { id: MOVE_WRAP, name: "Wrap", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 15, accuracy: 85, pp: 20, description: "Traps the foe for 2-5 turns." },
    // ─── Mahogany / Ice moves ──────────────────────────────
    MoveData { id: MOVE_HEADBUTT, name: "Headbutt", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 70, accuracy: 100, pp: 15, description: "May cause flinching." },
    MoveData { id: MOVE_ICY_WIND, name: "Icy Wind", move_type: PokemonType::Ice, category: MoveCategory::Special, power: 55, accuracy: 95, pp: 15, description: "Lowers the foe's Speed." },
    MoveData { id: MOVE_AURORA_BEAM, name: "Aurora Beam", move_type: PokemonType::Ice, category: MoveCategory::Special, power: 65, accuracy: 100, pp: 20, description: "May lower Attack." },
    MoveData { id: MOVE_ICE_BEAM, name: "Ice Beam", move_type: PokemonType::Ice, category: MoveCategory::Special, power: 95, accuracy: 100, pp: 10, description: "May freeze the foe." },
    MoveData { id: MOVE_REST, name: "Rest", move_type: PokemonType::Psychic, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 10, description: "Sleep 2 turns, full HP restore." },
    MoveData { id: MOVE_POWDER_SNOW, name: "Powder Snow", move_type: PokemonType::Ice, category: MoveCategory::Special, power: 40, accuracy: 100, pp: 25, description: "May freeze the foe." },
    MoveData { id: MOVE_EARTHQUAKE, name: "Earthquake", move_type: PokemonType::Ground, category: MoveCategory::Physical, power: 100, accuracy: 100, pp: 10, description: "Tough but can't hit airborne." },
    MoveData { id: MOVE_BLIZZARD, name: "Blizzard", move_type: PokemonType::Ice, category: MoveCategory::Special, power: 120, accuracy: 70, pp: 5, description: "May freeze the foe." },
    MoveData { id: MOVE_HYDRO_PUMP, name: "Hydro Pump", move_type: PokemonType::Water, category: MoveCategory::Special, power: 120, accuracy: 80, pp: 5, description: "Blasts water at high pressure." },
    MoveData { id: MOVE_DRAGON_RAGE, name: "Dragon Rage", move_type: PokemonType::Dragon, category: MoveCategory::Special, power: 1, accuracy: 100, pp: 10, description: "Always does 40 damage." },
    MoveData { id: MOVE_TWISTER, name: "Twister", move_type: PokemonType::Dragon, category: MoveCategory::Special, power: 40, accuracy: 100, pp: 20, description: "May cause flinching." },
    MoveData { id: MOVE_ENDURE, name: "Endure", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 10, description: "Always leaves at least 1 HP." },
    MoveData { id: MOVE_AMNESIA, name: "Amnesia", move_type: PokemonType::Psychic, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 20, description: "Sharply raises Sp. Def." },
    MoveData { id: MOVE_THRASH, name: "Thrash", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 90, accuracy: 100, pp: 20, description: "Rampages 2-3 turns, confuses." },
    // ─── Sprint 50: Blackthorn / Phase 3 moves ──────────────
    MoveData { id: MOVE_AGILITY, name: "Agility", move_type: PokemonType::Psychic, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 30, description: "Sharply raises Speed." },
    MoveData { id: MOVE_OUTRAGE, name: "Outrage", move_type: PokemonType::Dragon, category: MoveCategory::Special, power: 90, accuracy: 100, pp: 15, description: "Rampages 2-3 turns, confuses." },
    MoveData { id: MOVE_HYPER_BEAM, name: "Hyper Beam", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 150, accuracy: 90, pp: 5, description: "Must recharge next turn." },
    MoveData { id: MOVE_PRESENT, name: "Present", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 40, accuracy: 90, pp: 15, description: "Random damage or heal." },
    MoveData { id: MOVE_ICE_PUNCH, name: "Ice Punch", move_type: PokemonType::Ice, category: MoveCategory::Special, power: 75, accuracy: 100, pp: 15, description: "May freeze the foe." },
    MoveData { id: MOVE_LOVELY_KISS, name: "Lovely Kiss", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 75, pp: 10, description: "Puts the foe to sleep." },
    MoveData { id: MOVE_SLASH, name: "Slash", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 70, accuracy: 100, pp: 20, description: "High critical-hit ratio." },
    MoveData { id: MOVE_SAFEGUARD, name: "Safeguard", move_type: PokemonType::Normal, category: MoveCategory::Status, power: 0, accuracy: 100, pp: 25, description: "Prevents status for 5 turns." },
    // ─── Sprint 52: Route 45/46 moves ──────────────────────
    MoveData { id: MOVE_SWIFT, name: "Swift", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 60, accuracy: 255, pp: 20, description: "Never misses." },
    MoveData { id: MOVE_STEEL_WING, name: "Steel Wing", move_type: PokemonType::Steel, category: MoveCategory::Physical, power: 70, accuracy: 90, pp: 25, description: "May raise Defense." },
    // ─── Sprint 55: Route 27/26 moves ──────────────────────
    MoveData { id: MOVE_FIRE_BLAST, name: "Fire Blast", move_type: PokemonType::Fire, category: MoveCategory::Special, power: 120, accuracy: 85, pp: 5, description: "May burn the foe." },
    MoveData { id: MOVE_EXTREME_SPEED, name: "ExtremeSpeed", move_type: PokemonType::Normal, category: MoveCategory::Physical, power: 80, accuracy: 100, pp: 5, description: "Always strikes first." },
    MoveData { id: MOVE_FLAME_WHEEL, name: "Flame Wheel", move_type: PokemonType::Fire, category: MoveCategory::Special, power: 60, accuracy: 100, pp: 25, description: "May burn the foe." },
    // ─── Sprint 56: E4 moves ───────────────────────────────
    MoveData { id: MOVE_PSYCHIC, name: "Psychic", move_type: PokemonType::Psychic, category: MoveCategory::Special, power: 90, accuracy: 100, pp: 10, description: "May lower Sp.Def." },
    MoveData { id: MOVE_CRUNCH, name: "Crunch", move_type: PokemonType::Dark, category: MoveCategory::Special, power: 80, accuracy: 100, pp: 15, description: "May lower Defense." },
];

// ─── Type Effectiveness Chart ───────────────────────────

/// Returns damage multiplier for attack type vs defense type.
/// 2.0 = super effective, 0.5 = not very effective, 0.0 = no effect
pub fn type_effectiveness(atk: PokemonType, def: PokemonType) -> f64 {
    use PokemonType::*;
    match (atk, def) {
        // Normal
        (Normal, Rock) => 0.5, (Normal, Ghost) => 0.0, (Normal, Steel) => 0.5,
        // Fire
        (Fire, Fire) => 0.5, (Fire, Water) => 0.5, (Fire, Grass) => 2.0,
        (Fire, Ice) => 2.0, (Fire, Bug) => 2.0, (Fire, Rock) => 0.5,
        (Fire, Dragon) => 0.5, (Fire, Steel) => 2.0,
        // Water
        (Water, Fire) => 2.0, (Water, Water) => 0.5, (Water, Grass) => 0.5,
        (Water, Ground) => 2.0, (Water, Rock) => 2.0, (Water, Dragon) => 0.5,
        // Grass
        (Grass, Fire) => 0.5, (Grass, Water) => 2.0, (Grass, Grass) => 0.5,
        (Grass, Poison) => 0.5, (Grass, Ground) => 2.0, (Grass, Flying) => 0.5,
        (Grass, Bug) => 0.5, (Grass, Rock) => 2.0, (Grass, Dragon) => 0.5,
        (Grass, Steel) => 0.5,
        // Electric
        (Electric, Water) => 2.0, (Electric, Electric) => 0.5, (Electric, Grass) => 0.5,
        (Electric, Ground) => 0.0, (Electric, Flying) => 2.0, (Electric, Dragon) => 0.5,
        // Flying
        (Flying, Electric) => 0.5, (Flying, Grass) => 2.0, (Flying, Fighting) => 2.0,
        (Flying, Bug) => 2.0, (Flying, Rock) => 0.5, (Flying, Steel) => 0.5,
        // Fighting
        (Fighting, Normal) => 2.0, (Fighting, Ice) => 2.0, (Fighting, Poison) => 0.5,
        (Fighting, Flying) => 0.5, (Fighting, Psychic) => 0.5, (Fighting, Bug) => 0.5,
        (Fighting, Rock) => 2.0, (Fighting, Ghost) => 0.0, (Fighting, Dark) => 2.0,
        (Fighting, Steel) => 2.0,
        // Poison
        (Poison, Grass) => 2.0, (Poison, Poison) => 0.5, (Poison, Ground) => 0.5,
        (Poison, Rock) => 0.5, (Poison, Ghost) => 0.5, (Poison, Steel) => 0.0,
        // Ground
        (Ground, Fire) => 2.0, (Ground, Electric) => 2.0, (Ground, Grass) => 0.5,
        (Ground, Poison) => 2.0, (Ground, Flying) => 0.0, (Ground, Bug) => 0.5,
        (Ground, Rock) => 2.0, (Ground, Steel) => 2.0,
        // Psychic
        (Psychic, Fighting) => 2.0, (Psychic, Poison) => 2.0, (Psychic, Psychic) => 0.5,
        (Psychic, Dark) => 0.0, (Psychic, Steel) => 0.5,
        // Bug
        (Bug, Fire) => 0.5, (Bug, Grass) => 2.0, (Bug, Fighting) => 0.5,
        (Bug, Poison) => 0.5, (Bug, Flying) => 0.5, (Bug, Psychic) => 2.0,
        (Bug, Ghost) => 0.5, (Bug, Dark) => 2.0, (Bug, Steel) => 0.5,
        // Rock
        (Rock, Fire) => 2.0, (Rock, Ice) => 2.0, (Rock, Fighting) => 0.5,
        (Rock, Ground) => 0.5, (Rock, Flying) => 2.0, (Rock, Bug) => 2.0,
        (Rock, Steel) => 0.5,
        // Ghost
        (Ghost, Normal) => 0.0, (Ghost, Psychic) => 2.0, (Ghost, Ghost) => 2.0,
        (Ghost, Dark) => 0.5, (Ghost, Steel) => 0.5,
        // Dragon
        (Dragon, Dragon) => 2.0, (Dragon, Steel) => 0.5,
        // Dark
        (Dark, Fighting) => 0.5, (Dark, Psychic) => 2.0, (Dark, Ghost) => 2.0,
        (Dark, Dark) => 0.5, (Dark, Steel) => 0.5,
        // Steel
        (Steel, Fire) => 0.5, (Steel, Water) => 0.5, (Steel, Electric) => 0.5,
        (Steel, Ice) => 2.0, (Steel, Rock) => 2.0, (Steel, Steel) => 0.5,
        // Ice
        (Ice, Fire) => 0.5, (Ice, Water) => 0.5, (Ice, Grass) => 2.0,
        (Ice, Ice) => 0.5, (Ice, Ground) => 2.0, (Ice, Flying) => 2.0,
        (Ice, Dragon) => 2.0, (Ice, Steel) => 0.5,
        // Default: neutral
        _ => 1.0,
    }
}

/// Combined type effectiveness for dual-type Pokemon
pub fn combined_effectiveness(atk_type: PokemonType, def_type1: PokemonType, def_type2: Option<PokemonType>) -> f64 {
    let mut mult = type_effectiveness(atk_type, def_type1);
    if let Some(t2) = def_type2 {
        mult *= type_effectiveness(atk_type, t2);
    }
    mult
}

// ─── Lookup Functions ───────────────────────────────────

pub fn get_species(id: SpeciesId) -> Option<&'static SpeciesData> {
    SPECIES_DB.iter().find(|s| s.id == id)
}

pub fn get_move(id: MoveId) -> Option<&'static MoveData> {
    MOVE_DB.iter().find(|m| m.id == id)
}

// ─── Stat Calculation (Gen 2 formulas) ──────────────────

/// Calculate HP stat for a given level
pub fn calc_hp(base: u16, level: u8) -> u16 {
    let l = level as u16;
    // Simplified: HP = ((Base * 2 + IV + EV/4) * Level / 100) + Level + 10
    // Using fixed IV=15, EV=0 for simplicity
    ((base * 2 + 15) * l / 100) + l + 10
}

/// Calculate non-HP stat for a given level
pub fn calc_stat(base: u16, level: u8) -> u16 {
    let l = level as u16;
    // Stat = ((Base * 2 + IV + EV/4) * Level / 100) + 5
    ((base * 2 + 15) * l / 100) + 5
}

/// Experience needed to reach a given level
pub fn exp_for_level(level: u8, growth: GrowthRate) -> u32 {
    let n = level as u32;
    if n <= 1 { return 0; }
    match growth {
        GrowthRate::Fast => (4 * n * n * n) / 5,
        GrowthRate::MediumFast => n * n * n,
        GrowthRate::MediumSlow => {
            let n3 = n * n * n;
            let n2 = n * n;
            // 6n^3/5 - 15n^2 + 100n - 140
            (6 * n3 / 5).saturating_sub(15 * n2) + 100 * n - 140
        }
        GrowthRate::Slow => (5 * n * n * n) / 4,
    }
}

/// Experience gained from defeating a Pokemon (Gen 2 formula)
/// EXP = (a * b * L) / 7 where a=1 for wild, 1.5 for trainer
pub fn exp_gained(defeated_species: &SpeciesData, defeated_level: u8, is_wild: bool) -> u32 {
    let b = defeated_species.base_exp_yield as u32;
    let l = defeated_level as u32;
    if is_wild {
        (b * l) / 7
    } else {
        (3 * b * l) / 14 // 1.5x wild rate for trainers
    }
}

// ─── Pokemon Creation ───────────────────────────────────

impl Pokemon {
    /// Create a new Pokemon at the given level with appropriate moves
    pub fn new(species_id: SpeciesId, level: u8) -> Self {
        let species = get_species(species_id).expect("Invalid species ID");

        // Calculate stats
        let max_hp = calc_hp(species.base_hp, level);
        let attack = calc_stat(species.base_attack, level);
        let defense = calc_stat(species.base_defense, level);
        let sp_attack = calc_stat(species.base_sp_attack, level);
        let sp_defense = calc_stat(species.base_sp_defense, level);
        let speed = calc_stat(species.base_speed, level);

        // Determine moves: take the last 4 moves the Pokemon would know at this level
        let mut available_moves: Vec<MoveId> = species.learnset.iter()
            .filter(|(lvl, _)| *lvl <= level)
            .map(|(_, mid)| *mid)
            .collect();

        // Take last 4 (most recently learned)
        while available_moves.len() > 4 {
            available_moves.remove(0);
        }

        let mut moves = [None; 4];
        let mut move_pp = [0u8; 4];
        let mut move_max_pp = [0u8; 4];

        for (i, &mid) in available_moves.iter().enumerate() {
            moves[i] = Some(mid);
            if let Some(md) = get_move(mid) {
                move_pp[i] = md.pp;
                move_max_pp[i] = md.pp;
            }
        }

        let exp = exp_for_level(level, species.growth_rate);

        Pokemon {
            species_id,
            nickname: None,
            level,
            hp: max_hp,
            max_hp,
            attack,
            defense,
            sp_attack,
            sp_defense,
            speed,
            exp,
            moves,
            move_pp,
            move_max_pp,
            status: StatusCondition::None,
        }
    }

    /// Recalculate stats (after leveling up)
    pub fn recalc_stats(&mut self) {
        if let Some(species) = get_species(self.species_id) {
            let old_max_hp = self.max_hp;
            self.max_hp = calc_hp(species.base_hp, self.level);
            self.attack = calc_stat(species.base_attack, self.level);
            self.defense = calc_stat(species.base_defense, self.level);
            self.sp_attack = calc_stat(species.base_sp_attack, self.level);
            self.sp_defense = calc_stat(species.base_sp_defense, self.level);
            self.speed = calc_stat(species.base_speed, self.level);
            // Increase current HP by the same amount max increased
            let hp_increase = self.max_hp.saturating_sub(old_max_hp);
            self.hp = self.hp.saturating_add(hp_increase).min(self.max_hp);
        }
    }

    /// Check if this Pokemon should learn a new move at its current level
    pub fn check_new_moves(&self) -> Vec<MoveId> {
        if let Some(species) = get_species(self.species_id) {
            species.learnset.iter()
                .filter(|(lvl, _)| *lvl == self.level)
                .map(|(_, mid)| *mid)
                .collect()
        } else {
            vec![]
        }
    }

    /// Heal to full HP, restore all PP, and clear status
    pub fn heal(&mut self) {
        self.hp = self.max_hp;
        for i in 0..4 {
            self.move_pp[i] = self.move_max_pp[i];
        }
        self.status = StatusCondition::None;
    }

    /// Is this Pokemon fainted?
    pub fn is_fainted(&self) -> bool {
        self.hp == 0
    }

    /// Get species name
    pub fn name(&self) -> &str {
        if let Some(nick) = &self.nickname {
            nick.as_str()
        } else if let Some(species) = get_species(self.species_id) {
            species.name
        } else {
            "???"
        }
    }

    /// Clear any status condition
    pub fn clear_status(&mut self) {
        self.status = StatusCondition::None;
    }

    /// Apply end-of-turn status damage. Returns damage dealt.
    pub fn apply_status_damage(&mut self) -> u16 {
        let damage = match self.status {
            StatusCondition::Poison | StatusCondition::Burn => self.max_hp / 8,
            _ => 0,
        };
        if damage > 0 {
            self.hp = self.hp.saturating_sub(damage);
        }
        damage
    }

    /// Check if this Pokemon can move this turn.
    /// Paralysis RNG check happens externally in mod.rs; here we always return true for it.
    pub fn can_move(&self) -> bool {
        match self.status {
            StatusCondition::Sleep { turns } => turns == 0,
            StatusCondition::Freeze => false,
            _ => true,
        }
    }

    /// Tick status at end of turn (decrements sleep counter, etc.)
    pub fn tick_status(&mut self) {
        match self.status {
            StatusCondition::Sleep { turns } => {
                if turns > 0 {
                    let new_turns = turns - 1;
                    if new_turns == 0 {
                        self.status = StatusCondition::None;
                    } else {
                        self.status = StatusCondition::Sleep { turns: new_turns };
                    }
                }
            }
            _ => {}
        }
    }
}

// ─── Items ─────────────────────────────────────────────

pub type ItemId = u8;

pub const ITEM_POTION: ItemId = 1;
pub const ITEM_SUPER_POTION: ItemId = 2;
pub const ITEM_ANTIDOTE: ItemId = 3;
pub const ITEM_POKE_BALL: ItemId = 4;
pub const ITEM_PARALYZE_HEAL: ItemId = 5;
pub const ITEM_REVIVE: ItemId = 6;
pub const ITEM_FULL_HEAL: ItemId = 7;
pub const ITEM_GREAT_BALL: ItemId = 8;
pub const ITEM_ETHER: ItemId = 9;
pub const ITEM_ESCAPE_ROPE: ItemId = 10;
pub const ITEM_REPEL: ItemId = 11;

#[derive(Clone, Debug)]
pub struct ItemData {
    pub id: ItemId,
    pub name: &'static str,
    pub description: &'static str,
    pub heal_amount: u16,
    pub is_ball: bool,
    pub price: u16,
    pub is_revive: bool,
    pub is_status_heal: bool,
}

const ITEM_DB: &[ItemData] = &[
    ItemData { id: ITEM_POTION, name: "Potion", description: "Restores 20 HP.", heal_amount: 20, is_ball: false, price: 300, is_revive: false, is_status_heal: false },
    ItemData { id: ITEM_SUPER_POTION, name: "Super Potion", description: "Restores 50 HP.", heal_amount: 50, is_ball: false, price: 700, is_revive: false, is_status_heal: false },
    ItemData { id: ITEM_ANTIDOTE, name: "Antidote", description: "Cures poison.", heal_amount: 0, is_ball: false, price: 100, is_revive: false, is_status_heal: true },
    ItemData { id: ITEM_POKE_BALL, name: "Poke Ball", description: "Catches wild Pokemon.", heal_amount: 0, is_ball: true, price: 200, is_revive: false, is_status_heal: false },
    ItemData { id: ITEM_PARALYZE_HEAL, name: "Paralyze Heal", description: "Cures paralysis.", heal_amount: 0, is_ball: false, price: 200, is_revive: false, is_status_heal: true },
    ItemData { id: ITEM_REVIVE, name: "Revive", description: "Revives a fainted Pokemon to 50% HP.", heal_amount: 0, is_ball: false, price: 1500, is_revive: true, is_status_heal: false },
    ItemData { id: ITEM_FULL_HEAL, name: "Full Heal", description: "Cures all status conditions.", heal_amount: 0, is_ball: false, price: 600, is_revive: false, is_status_heal: true },
    ItemData { id: ITEM_GREAT_BALL, name: "Great Ball", description: "A good Ball with a higher catch rate.", heal_amount: 0, is_ball: true, price: 600, is_revive: false, is_status_heal: false },
    ItemData { id: ITEM_ETHER, name: "Ether", description: "Restores 10 PP to one move.", heal_amount: 0, is_ball: false, price: 1200, is_revive: false, is_status_heal: false },
    ItemData { id: ITEM_ESCAPE_ROPE, name: "Escape Rope", description: "Escapes from dungeons instantly.", heal_amount: 0, is_ball: false, price: 550, is_revive: false, is_status_heal: false },
    ItemData { id: ITEM_REPEL, name: "Repel", description: "Repels wild Pokemon for 100 steps.", heal_amount: 0, is_ball: false, price: 350, is_revive: false, is_status_heal: false },
];

pub fn get_item(id: ItemId) -> Option<&'static ItemData> {
    ITEM_DB.iter().find(|i| i.id == id)
}

/// Player's bag: (item_id, quantity) pairs
#[derive(Clone, Debug)]
pub struct Bag {
    pub items: Vec<(ItemId, u8)>,
}

impl Bag {
    pub fn new() -> Self {
        Bag { items: Vec::new() }
    }

    pub fn add_item(&mut self, id: ItemId, qty: u8) {
        if let Some(entry) = self.items.iter_mut().find(|(i, _)| *i == id) {
            entry.1 = entry.1.saturating_add(qty);
        } else {
            self.items.push((id, qty));
        }
    }

    pub fn use_item(&mut self, id: ItemId) -> bool {
        if let Some(entry) = self.items.iter_mut().find(|(i, _)| *i == id) {
            if entry.1 > 0 {
                entry.1 -= 1;
                if entry.1 == 0 {
                    self.items.retain(|(i, _)| *i != id);
                }
                return true;
            }
        }
        false
    }

    pub fn count(&self, id: ItemId) -> u8 {
        self.items.iter().find(|(i, _)| *i == id).map(|(_, q)| *q).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Damage calculation (Gen 2 formula, simplified)
/// atk_stage_mult and def_stage_mult should be pre-computed from stage_multiplier()
pub fn calc_damage(
    attacker: &Pokemon,
    defender_defense: u16,
    defender_type1: PokemonType,
    defender_type2: Option<PokemonType>,
    move_data: &MoveData,
    rng_roll: f64, // 0.85 to 1.0
    is_crit: bool,
    atk_stage_mult: f64,
    def_stage_mult: f64,
) -> (u16, f64) {
    // Status moves do no damage
    if move_data.category == MoveCategory::Status || move_data.power == 0 {
        return (0, 1.0);
    }

    let level = attacker.level as f64;
    let attack_stat = match move_data.category {
        MoveCategory::Physical => attacker.attack as f64 * atk_stage_mult,
        MoveCategory::Special => attacker.sp_attack as f64 * atk_stage_mult,
        MoveCategory::Status => 0.0,
    };
    let defense_stat = defender_defense as f64 * def_stage_mult;
    let power = move_data.power as f64;

    // STAB (Same Type Attack Bonus)
    let attacker_species = get_species(attacker.species_id);
    let stab = if let Some(sp) = attacker_species {
        if move_data.move_type == sp.type1 || sp.type2 == Some(move_data.move_type) {
            1.5
        } else {
            1.0
        }
    } else {
        1.0
    };

    // Type effectiveness
    let effectiveness = combined_effectiveness(move_data.move_type, defender_type1, defender_type2);

    // Critical hit multiplier (Gen 2: 2x)
    let crit_mult = if is_crit { 2.0 } else { 1.0 };

    // Burn halves Physical damage (Gen 2)
    let burn_mult = if matches!(attacker.status, StatusCondition::Burn) && move_data.category == MoveCategory::Physical {
        0.5
    } else {
        1.0
    };

    // Damage formula: ((2*Level/5 + 2) * Power * A/D) / 50 + 2) * STAB * Type * Random * Crit * Burn
    let base = ((2.0 * level / 5.0 + 2.0) * power * attack_stat / defense_stat) / 50.0 + 2.0;
    let damage = (base * stab * effectiveness * rng_roll * crit_mult * burn_mult).max(1.0) as u16;

    (damage, effectiveness)
}
