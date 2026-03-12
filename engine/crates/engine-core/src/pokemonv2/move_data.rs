// AI-INSTRUCTIONS: pokemonv2/move_data.rs — All 251 move data from pokecrystal-master.
// Source of truth: pokecrystal-master/data/moves/moves.asm
// Auto-generated from ASM files. Sprint 7: Complete move database.

use super::data::{MoveData, MoveId, PokemonType};

pub const NUM_MOVES: usize = 251;

// --- MoveId Constants (1-251, matching ASM order) ---

pub const MOVE_POUND: MoveId = 1;
pub const MOVE_KARATE_CHOP: MoveId = 2;
pub const MOVE_DOUBLESLAP: MoveId = 3;
pub const MOVE_COMET_PUNCH: MoveId = 4;
pub const MOVE_MEGA_PUNCH: MoveId = 5;
pub const MOVE_PAY_DAY: MoveId = 6;
pub const MOVE_FIRE_PUNCH: MoveId = 7;
pub const MOVE_ICE_PUNCH: MoveId = 8;
pub const MOVE_THUNDERPUNCH: MoveId = 9;
pub const MOVE_SCRATCH: MoveId = 10;
pub const MOVE_VICEGRIP: MoveId = 11;
pub const MOVE_GUILLOTINE: MoveId = 12;
pub const MOVE_RAZOR_WIND: MoveId = 13;
pub const MOVE_SWORDS_DANCE: MoveId = 14;
pub const MOVE_CUT: MoveId = 15;
pub const MOVE_GUST: MoveId = 16;
pub const MOVE_WING_ATTACK: MoveId = 17;
pub const MOVE_WHIRLWIND: MoveId = 18;
pub const MOVE_FLY: MoveId = 19;
pub const MOVE_BIND: MoveId = 20;
pub const MOVE_SLAM: MoveId = 21;
pub const MOVE_VINE_WHIP: MoveId = 22;
pub const MOVE_STOMP: MoveId = 23;
pub const MOVE_DOUBLE_KICK: MoveId = 24;
pub const MOVE_MEGA_KICK: MoveId = 25;
pub const MOVE_JUMP_KICK: MoveId = 26;
pub const MOVE_ROLLING_KICK: MoveId = 27;
pub const MOVE_SAND_ATTACK: MoveId = 28;
pub const MOVE_HEADBUTT: MoveId = 29;
pub const MOVE_HORN_ATTACK: MoveId = 30;
pub const MOVE_FURY_ATTACK: MoveId = 31;
pub const MOVE_HORN_DRILL: MoveId = 32;
pub const MOVE_TACKLE: MoveId = 33;
pub const MOVE_BODY_SLAM: MoveId = 34;
pub const MOVE_WRAP: MoveId = 35;
pub const MOVE_TAKE_DOWN: MoveId = 36;
pub const MOVE_THRASH: MoveId = 37;
pub const MOVE_DOUBLE_EDGE: MoveId = 38;
pub const MOVE_TAIL_WHIP: MoveId = 39;
pub const MOVE_POISON_STING: MoveId = 40;
pub const MOVE_TWINEEDLE: MoveId = 41;
pub const MOVE_PIN_MISSILE: MoveId = 42;
pub const MOVE_LEER: MoveId = 43;
pub const MOVE_BITE: MoveId = 44;
pub const MOVE_GROWL: MoveId = 45;
pub const MOVE_ROAR: MoveId = 46;
pub const MOVE_SING: MoveId = 47;
pub const MOVE_SUPERSONIC: MoveId = 48;
pub const MOVE_SONICBOOM: MoveId = 49;
pub const MOVE_DISABLE: MoveId = 50;
pub const MOVE_ACID: MoveId = 51;
pub const MOVE_EMBER: MoveId = 52;
pub const MOVE_FLAMETHROWER: MoveId = 53;
pub const MOVE_MIST: MoveId = 54;
pub const MOVE_WATER_GUN: MoveId = 55;
pub const MOVE_HYDRO_PUMP: MoveId = 56;
pub const MOVE_SURF: MoveId = 57;
pub const MOVE_ICE_BEAM: MoveId = 58;
pub const MOVE_BLIZZARD: MoveId = 59;
pub const MOVE_PSYBEAM: MoveId = 60;
pub const MOVE_BUBBLEBEAM: MoveId = 61;
pub const MOVE_AURORA_BEAM: MoveId = 62;
pub const MOVE_HYPER_BEAM: MoveId = 63;
pub const MOVE_PECK: MoveId = 64;
pub const MOVE_DRILL_PECK: MoveId = 65;
pub const MOVE_SUBMISSION: MoveId = 66;
pub const MOVE_LOW_KICK: MoveId = 67;
pub const MOVE_COUNTER: MoveId = 68;
pub const MOVE_SEISMIC_TOSS: MoveId = 69;
pub const MOVE_STRENGTH: MoveId = 70;
pub const MOVE_ABSORB: MoveId = 71;
pub const MOVE_MEGA_DRAIN: MoveId = 72;
pub const MOVE_LEECH_SEED: MoveId = 73;
pub const MOVE_GROWTH: MoveId = 74;
pub const MOVE_RAZOR_LEAF: MoveId = 75;
pub const MOVE_SOLARBEAM: MoveId = 76;
pub const MOVE_POISONPOWDER: MoveId = 77;
pub const MOVE_STUN_SPORE: MoveId = 78;
pub const MOVE_SLEEP_POWDER: MoveId = 79;
pub const MOVE_PETAL_DANCE: MoveId = 80;
pub const MOVE_STRING_SHOT: MoveId = 81;
pub const MOVE_DRAGON_RAGE: MoveId = 82;
pub const MOVE_FIRE_SPIN: MoveId = 83;
pub const MOVE_THUNDERSHOCK: MoveId = 84;
pub const MOVE_THUNDERBOLT: MoveId = 85;
pub const MOVE_THUNDER_WAVE: MoveId = 86;
pub const MOVE_THUNDER: MoveId = 87;
pub const MOVE_ROCK_THROW: MoveId = 88;
pub const MOVE_EARTHQUAKE: MoveId = 89;
pub const MOVE_FISSURE: MoveId = 90;
pub const MOVE_DIG: MoveId = 91;
pub const MOVE_TOXIC: MoveId = 92;
pub const MOVE_CONFUSION: MoveId = 93;
pub const MOVE_PSYCHIC_M: MoveId = 94;
pub const MOVE_HYPNOSIS: MoveId = 95;
pub const MOVE_MEDITATE: MoveId = 96;
pub const MOVE_AGILITY: MoveId = 97;
pub const MOVE_QUICK_ATTACK: MoveId = 98;
pub const MOVE_RAGE: MoveId = 99;
pub const MOVE_TELEPORT: MoveId = 100;
pub const MOVE_NIGHT_SHADE: MoveId = 101;
pub const MOVE_MIMIC: MoveId = 102;
pub const MOVE_SCREECH: MoveId = 103;
pub const MOVE_DOUBLE_TEAM: MoveId = 104;
pub const MOVE_RECOVER: MoveId = 105;
pub const MOVE_HARDEN: MoveId = 106;
pub const MOVE_MINIMIZE: MoveId = 107;
pub const MOVE_SMOKESCREEN: MoveId = 108;
pub const MOVE_CONFUSE_RAY: MoveId = 109;
pub const MOVE_WITHDRAW: MoveId = 110;
pub const MOVE_DEFENSE_CURL: MoveId = 111;
pub const MOVE_BARRIER: MoveId = 112;
pub const MOVE_LIGHT_SCREEN: MoveId = 113;
pub const MOVE_HAZE: MoveId = 114;
pub const MOVE_REFLECT: MoveId = 115;
pub const MOVE_FOCUS_ENERGY: MoveId = 116;
pub const MOVE_BIDE: MoveId = 117;
pub const MOVE_METRONOME: MoveId = 118;
pub const MOVE_MIRROR_MOVE: MoveId = 119;
pub const MOVE_SELFDESTRUCT: MoveId = 120;
pub const MOVE_EGG_BOMB: MoveId = 121;
pub const MOVE_LICK: MoveId = 122;
pub const MOVE_SMOG: MoveId = 123;
pub const MOVE_SLUDGE: MoveId = 124;
pub const MOVE_BONE_CLUB: MoveId = 125;
pub const MOVE_FIRE_BLAST: MoveId = 126;
pub const MOVE_WATERFALL: MoveId = 127;
pub const MOVE_CLAMP: MoveId = 128;
pub const MOVE_SWIFT: MoveId = 129;
pub const MOVE_SKULL_BASH: MoveId = 130;
pub const MOVE_SPIKE_CANNON: MoveId = 131;
pub const MOVE_CONSTRICT: MoveId = 132;
pub const MOVE_AMNESIA: MoveId = 133;
pub const MOVE_KINESIS: MoveId = 134;
pub const MOVE_SOFTBOILED: MoveId = 135;
pub const MOVE_HI_JUMP_KICK: MoveId = 136;
pub const MOVE_GLARE: MoveId = 137;
pub const MOVE_DREAM_EATER: MoveId = 138;
pub const MOVE_POISON_GAS: MoveId = 139;
pub const MOVE_BARRAGE: MoveId = 140;
pub const MOVE_LEECH_LIFE: MoveId = 141;
pub const MOVE_LOVELY_KISS: MoveId = 142;
pub const MOVE_SKY_ATTACK: MoveId = 143;
pub const MOVE_TRANSFORM: MoveId = 144;
pub const MOVE_BUBBLE: MoveId = 145;
pub const MOVE_DIZZY_PUNCH: MoveId = 146;
pub const MOVE_SPORE: MoveId = 147;
pub const MOVE_FLASH: MoveId = 148;
pub const MOVE_PSYWAVE: MoveId = 149;
pub const MOVE_SPLASH: MoveId = 150;
pub const MOVE_ACID_ARMOR: MoveId = 151;
pub const MOVE_CRABHAMMER: MoveId = 152;
pub const MOVE_EXPLOSION: MoveId = 153;
pub const MOVE_FURY_SWIPES: MoveId = 154;
pub const MOVE_BONEMERANG: MoveId = 155;
pub const MOVE_REST: MoveId = 156;
pub const MOVE_ROCK_SLIDE: MoveId = 157;
pub const MOVE_HYPER_FANG: MoveId = 158;
pub const MOVE_SHARPEN: MoveId = 159;
pub const MOVE_CONVERSION: MoveId = 160;
pub const MOVE_TRI_ATTACK: MoveId = 161;
pub const MOVE_SUPER_FANG: MoveId = 162;
pub const MOVE_SLASH: MoveId = 163;
pub const MOVE_SUBSTITUTE: MoveId = 164;
pub const MOVE_STRUGGLE: MoveId = 165;
pub const MOVE_SKETCH: MoveId = 166;
pub const MOVE_TRIPLE_KICK: MoveId = 167;
pub const MOVE_THIEF: MoveId = 168;
pub const MOVE_SPIDER_WEB: MoveId = 169;
pub const MOVE_MIND_READER: MoveId = 170;
pub const MOVE_NIGHTMARE: MoveId = 171;
pub const MOVE_FLAME_WHEEL: MoveId = 172;
pub const MOVE_SNORE: MoveId = 173;
pub const MOVE_CURSE: MoveId = 174;
pub const MOVE_FLAIL: MoveId = 175;
pub const MOVE_CONVERSION2: MoveId = 176;
pub const MOVE_AEROBLAST: MoveId = 177;
pub const MOVE_COTTON_SPORE: MoveId = 178;
pub const MOVE_REVERSAL: MoveId = 179;
pub const MOVE_SPITE: MoveId = 180;
pub const MOVE_POWDER_SNOW: MoveId = 181;
pub const MOVE_PROTECT: MoveId = 182;
pub const MOVE_MACH_PUNCH: MoveId = 183;
pub const MOVE_SCARY_FACE: MoveId = 184;
pub const MOVE_FAINT_ATTACK: MoveId = 185;
pub const MOVE_SWEET_KISS: MoveId = 186;
pub const MOVE_BELLY_DRUM: MoveId = 187;
pub const MOVE_SLUDGE_BOMB: MoveId = 188;
pub const MOVE_MUD_SLAP: MoveId = 189;
pub const MOVE_OCTAZOOKA: MoveId = 190;
pub const MOVE_SPIKES: MoveId = 191;
pub const MOVE_ZAP_CANNON: MoveId = 192;
pub const MOVE_FORESIGHT: MoveId = 193;
pub const MOVE_DESTINY_BOND: MoveId = 194;
pub const MOVE_PERISH_SONG: MoveId = 195;
pub const MOVE_ICY_WIND: MoveId = 196;
pub const MOVE_DETECT: MoveId = 197;
pub const MOVE_BONE_RUSH: MoveId = 198;
pub const MOVE_LOCK_ON: MoveId = 199;
pub const MOVE_OUTRAGE: MoveId = 200;
pub const MOVE_SANDSTORM: MoveId = 201;
pub const MOVE_GIGA_DRAIN: MoveId = 202;
pub const MOVE_ENDURE: MoveId = 203;
pub const MOVE_CHARM: MoveId = 204;
pub const MOVE_ROLLOUT: MoveId = 205;
pub const MOVE_FALSE_SWIPE: MoveId = 206;
pub const MOVE_SWAGGER: MoveId = 207;
pub const MOVE_MILK_DRINK: MoveId = 208;
pub const MOVE_SPARK: MoveId = 209;
pub const MOVE_FURY_CUTTER: MoveId = 210;
pub const MOVE_STEEL_WING: MoveId = 211;
pub const MOVE_MEAN_LOOK: MoveId = 212;
pub const MOVE_ATTRACT: MoveId = 213;
pub const MOVE_SLEEP_TALK: MoveId = 214;
pub const MOVE_HEAL_BELL: MoveId = 215;
pub const MOVE_RETURN: MoveId = 216;
pub const MOVE_PRESENT: MoveId = 217;
pub const MOVE_FRUSTRATION: MoveId = 218;
pub const MOVE_SAFEGUARD: MoveId = 219;
pub const MOVE_PAIN_SPLIT: MoveId = 220;
pub const MOVE_SACRED_FIRE: MoveId = 221;
pub const MOVE_MAGNITUDE: MoveId = 222;
pub const MOVE_DYNAMICPUNCH: MoveId = 223;
pub const MOVE_MEGAHORN: MoveId = 224;
pub const MOVE_DRAGONBREATH: MoveId = 225;
pub const MOVE_BATON_PASS: MoveId = 226;
pub const MOVE_ENCORE: MoveId = 227;
pub const MOVE_PURSUIT: MoveId = 228;
pub const MOVE_RAPID_SPIN: MoveId = 229;
pub const MOVE_SWEET_SCENT: MoveId = 230;
pub const MOVE_IRON_TAIL: MoveId = 231;
pub const MOVE_METAL_CLAW: MoveId = 232;
pub const MOVE_VITAL_THROW: MoveId = 233;
pub const MOVE_MORNING_SUN: MoveId = 234;
pub const MOVE_SYNTHESIS: MoveId = 235;
pub const MOVE_MOONLIGHT: MoveId = 236;
pub const MOVE_HIDDEN_POWER: MoveId = 237;
pub const MOVE_CROSS_CHOP: MoveId = 238;
pub const MOVE_TWISTER: MoveId = 239;
pub const MOVE_RAIN_DANCE: MoveId = 240;
pub const MOVE_SUNNY_DAY: MoveId = 241;
pub const MOVE_CRUNCH: MoveId = 242;
pub const MOVE_MIRROR_COAT: MoveId = 243;
pub const MOVE_PSYCH_UP: MoveId = 244;
pub const MOVE_EXTREMESPEED: MoveId = 245;
pub const MOVE_ANCIENTPOWER: MoveId = 246;
pub const MOVE_SHADOW_BALL: MoveId = 247;
pub const MOVE_FUTURE_SIGHT: MoveId = 248;
pub const MOVE_ROCK_SMASH: MoveId = 249;
pub const MOVE_WHIRLPOOL: MoveId = 250;
pub const MOVE_BEAT_UP: MoveId = 251;

// --- Effect ID Constants ---

pub const EFFECT_NORMAL_HIT: u8 = 0;
pub const EFFECT_MULTI_HIT: u8 = 1;
pub const EFFECT_PAY_DAY: u8 = 2;
pub const EFFECT_BURN_HIT: u8 = 3;
pub const EFFECT_FREEZE_HIT: u8 = 4;
pub const EFFECT_PARALYZE_HIT: u8 = 5;
pub const EFFECT_OHKO: u8 = 6;
pub const EFFECT_RAZOR_WIND: u8 = 7;
pub const EFFECT_ATTACK_UP_2: u8 = 8;
pub const EFFECT_GUST: u8 = 9;
pub const EFFECT_FORCE_SWITCH: u8 = 10;
pub const EFFECT_FLY: u8 = 11;
pub const EFFECT_TRAP_TARGET: u8 = 12;
pub const EFFECT_STOMP: u8 = 13;
pub const EFFECT_DOUBLE_HIT: u8 = 14;
pub const EFFECT_JUMP_KICK: u8 = 15;
pub const EFFECT_FLINCH_HIT: u8 = 16;
pub const EFFECT_ACCURACY_DOWN: u8 = 17;
pub const EFFECT_RECOIL_HIT: u8 = 18;
pub const EFFECT_RAMPAGE: u8 = 19;
pub const EFFECT_DEFENSE_DOWN: u8 = 20;
pub const EFFECT_POISON_HIT: u8 = 21;
pub const EFFECT_POISON_MULTI_HIT: u8 = 22;
pub const EFFECT_ATTACK_DOWN: u8 = 23;
pub const EFFECT_SLEEP: u8 = 24;
pub const EFFECT_CONFUSE: u8 = 25;
pub const EFFECT_STATIC_DAMAGE: u8 = 26;
pub const EFFECT_DISABLE: u8 = 27;
pub const EFFECT_DEFENSE_DOWN_HIT: u8 = 28;
pub const EFFECT_MIST: u8 = 29;
pub const EFFECT_CONFUSE_HIT: u8 = 30;
pub const EFFECT_SPEED_DOWN_HIT: u8 = 31;
pub const EFFECT_ATTACK_DOWN_HIT: u8 = 32;
pub const EFFECT_HYPER_BEAM: u8 = 33;
pub const EFFECT_COUNTER: u8 = 34;
pub const EFFECT_LEVEL_DAMAGE: u8 = 35;
pub const EFFECT_LEECH_HIT: u8 = 36;
pub const EFFECT_LEECH_SEED: u8 = 37;
pub const EFFECT_SP_ATK_UP: u8 = 38;
pub const EFFECT_SOLARBEAM: u8 = 39;
pub const EFFECT_POISON: u8 = 40;
pub const EFFECT_PARALYZE: u8 = 41;
pub const EFFECT_SPEED_DOWN: u8 = 42;
pub const EFFECT_THUNDER: u8 = 43;
pub const EFFECT_EARTHQUAKE: u8 = 44;
pub const EFFECT_TOXIC: u8 = 45;
pub const EFFECT_SP_DEF_DOWN_HIT: u8 = 46;
pub const EFFECT_ATTACK_UP: u8 = 47;
pub const EFFECT_SPEED_UP_2: u8 = 48;
pub const EFFECT_PRIORITY_HIT: u8 = 49;
pub const EFFECT_RAGE: u8 = 50;
pub const EFFECT_TELEPORT: u8 = 51;
pub const EFFECT_MIMIC: u8 = 52;
pub const EFFECT_DEFENSE_DOWN_2: u8 = 53;
pub const EFFECT_EVASION_UP: u8 = 54;
pub const EFFECT_HEAL: u8 = 55;
pub const EFFECT_DEFENSE_UP: u8 = 56;
pub const EFFECT_DEFENSE_CURL: u8 = 57;
pub const EFFECT_DEFENSE_UP_2: u8 = 58;
pub const EFFECT_LIGHT_SCREEN: u8 = 59;
pub const EFFECT_RESET_STATS: u8 = 60;
pub const EFFECT_REFLECT: u8 = 61;
pub const EFFECT_FOCUS_ENERGY: u8 = 62;
pub const EFFECT_BIDE: u8 = 63;
pub const EFFECT_METRONOME: u8 = 64;
pub const EFFECT_MIRROR_MOVE: u8 = 65;
pub const EFFECT_SELFDESTRUCT: u8 = 66;
pub const EFFECT_ALWAYS_HIT: u8 = 67;
pub const EFFECT_SKULL_BASH: u8 = 68;
pub const EFFECT_SP_DEF_UP_2: u8 = 69;
pub const EFFECT_DREAM_EATER: u8 = 70;
pub const EFFECT_SKY_ATTACK: u8 = 71;
pub const EFFECT_TRANSFORM: u8 = 72;
pub const EFFECT_PSYWAVE: u8 = 73;
pub const EFFECT_SPLASH: u8 = 74;
pub const EFFECT_CONVERSION: u8 = 75;
pub const EFFECT_TRI_ATTACK: u8 = 76;
pub const EFFECT_SUPER_FANG: u8 = 77;
pub const EFFECT_SUBSTITUTE: u8 = 78;
pub const EFFECT_SKETCH: u8 = 79;
pub const EFFECT_TRIPLE_KICK: u8 = 80;
pub const EFFECT_THIEF: u8 = 81;
pub const EFFECT_MEAN_LOOK: u8 = 82;
pub const EFFECT_LOCK_ON: u8 = 83;
pub const EFFECT_NIGHTMARE: u8 = 84;
pub const EFFECT_FLAME_WHEEL: u8 = 85;
pub const EFFECT_SNORE: u8 = 86;
pub const EFFECT_CURSE: u8 = 87;
pub const EFFECT_REVERSAL: u8 = 88;
pub const EFFECT_CONVERSION2: u8 = 89;
pub const EFFECT_SPEED_DOWN_2: u8 = 90;
pub const EFFECT_SPITE: u8 = 91;
pub const EFFECT_PROTECT: u8 = 92;
pub const EFFECT_BELLY_DRUM: u8 = 93;
pub const EFFECT_ACCURACY_DOWN_HIT: u8 = 94;
pub const EFFECT_SPIKES: u8 = 95;
pub const EFFECT_FORESIGHT: u8 = 96;
pub const EFFECT_DESTINY_BOND: u8 = 97;
pub const EFFECT_PERISH_SONG: u8 = 98;
pub const EFFECT_SANDSTORM: u8 = 99;
pub const EFFECT_ENDURE: u8 = 100;
pub const EFFECT_ATTACK_DOWN_2: u8 = 101;
pub const EFFECT_ROLLOUT: u8 = 102;
pub const EFFECT_FALSE_SWIPE: u8 = 103;
pub const EFFECT_SWAGGER: u8 = 104;
pub const EFFECT_FURY_CUTTER: u8 = 105;
pub const EFFECT_DEFENSE_UP_HIT: u8 = 106;
pub const EFFECT_ATTRACT: u8 = 107;
pub const EFFECT_SLEEP_TALK: u8 = 108;
pub const EFFECT_HEAL_BELL: u8 = 109;
pub const EFFECT_RETURN: u8 = 110;
pub const EFFECT_PRESENT: u8 = 111;
pub const EFFECT_FRUSTRATION: u8 = 112;
pub const EFFECT_SAFEGUARD: u8 = 113;
pub const EFFECT_PAIN_SPLIT: u8 = 114;
pub const EFFECT_SACRED_FIRE: u8 = 115;
pub const EFFECT_MAGNITUDE: u8 = 116;
pub const EFFECT_BATON_PASS: u8 = 117;
pub const EFFECT_ENCORE: u8 = 118;
pub const EFFECT_PURSUIT: u8 = 119;
pub const EFFECT_RAPID_SPIN: u8 = 120;
pub const EFFECT_EVASION_DOWN: u8 = 121;
pub const EFFECT_ATTACK_UP_HIT: u8 = 122;
pub const EFFECT_MORNING_SUN: u8 = 123;
pub const EFFECT_SYNTHESIS: u8 = 124;
pub const EFFECT_MOONLIGHT: u8 = 125;
pub const EFFECT_HIDDEN_POWER: u8 = 126;
pub const EFFECT_TWISTER: u8 = 127;
pub const EFFECT_RAIN_DANCE: u8 = 128;
pub const EFFECT_SUNNY_DAY: u8 = 129;
pub const EFFECT_MIRROR_COAT: u8 = 130;
pub const EFFECT_PSYCH_UP: u8 = 131;
pub const EFFECT_ALL_UP_HIT: u8 = 132;
pub const EFFECT_FUTURE_SIGHT: u8 = 133;
pub const EFFECT_BEAT_UP: u8 = 134;

// --- Move Table (indexed by move_id - 1) ---

static MOVE_TABLE: [MoveData; NUM_MOVES] = [
    // #001 POUND
    MoveData {
        id: 1, name: "POUND",
        move_type: PokemonType::Normal, power: 40, accuracy: 100, pp: 35,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #002 KARATE CHOP
    MoveData {
        id: 2, name: "KARATE CHOP",
        move_type: PokemonType::Fighting, power: 50, accuracy: 100, pp: 25,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: true,
    },
    // #003 DOUBLESLAP
    MoveData {
        id: 3, name: "DOUBLESLAP",
        move_type: PokemonType::Normal, power: 15, accuracy: 85, pp: 10,
        is_special: false,
        effect_id: EFFECT_MULTI_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #004 COMET PUNCH
    MoveData {
        id: 4, name: "COMET PUNCH",
        move_type: PokemonType::Normal, power: 18, accuracy: 85, pp: 15,
        is_special: false,
        effect_id: EFFECT_MULTI_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #005 MEGA PUNCH
    MoveData {
        id: 5, name: "MEGA PUNCH",
        move_type: PokemonType::Normal, power: 80, accuracy: 85, pp: 20,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #006 PAY DAY
    MoveData {
        id: 6, name: "PAY DAY",
        move_type: PokemonType::Normal, power: 40, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_PAY_DAY, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #007 FIRE PUNCH
    MoveData {
        id: 7, name: "FIRE PUNCH",
        move_type: PokemonType::Fire, power: 75, accuracy: 100, pp: 15,
        is_special: true,
        effect_id: EFFECT_BURN_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #008 ICE PUNCH
    MoveData {
        id: 8, name: "ICE PUNCH",
        move_type: PokemonType::Ice, power: 75, accuracy: 100, pp: 15,
        is_special: true,
        effect_id: EFFECT_FREEZE_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #009 THUNDERPUNCH
    MoveData {
        id: 9, name: "THUNDERPUNCH",
        move_type: PokemonType::Electric, power: 75, accuracy: 100, pp: 15,
        is_special: true,
        effect_id: EFFECT_PARALYZE_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #010 SCRATCH
    MoveData {
        id: 10, name: "SCRATCH",
        move_type: PokemonType::Normal, power: 40, accuracy: 100, pp: 35,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #011 VICEGRIP
    MoveData {
        id: 11, name: "VICEGRIP",
        move_type: PokemonType::Normal, power: 55, accuracy: 100, pp: 30,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #012 GUILLOTINE
    MoveData {
        id: 12, name: "GUILLOTINE",
        move_type: PokemonType::Normal, power: 0, accuracy: 30, pp: 5,
        is_special: false,
        effect_id: EFFECT_OHKO, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #013 RAZOR WIND
    MoveData {
        id: 13, name: "RAZOR WIND",
        move_type: PokemonType::Normal, power: 80, accuracy: 75, pp: 10,
        is_special: false,
        effect_id: EFFECT_RAZOR_WIND, effect_chance: 0,
        priority: 1, high_crit: true,
    },
    // #014 SWORDS DANCE
    MoveData {
        id: 14, name: "SWORDS DANCE",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 30,
        is_special: false,
        effect_id: EFFECT_ATTACK_UP_2, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #015 CUT
    MoveData {
        id: 15, name: "CUT",
        move_type: PokemonType::Normal, power: 50, accuracy: 95, pp: 30,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #016 GUST
    MoveData {
        id: 16, name: "GUST",
        move_type: PokemonType::Flying, power: 40, accuracy: 100, pp: 35,
        is_special: false,
        effect_id: EFFECT_GUST, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #017 WING ATTACK
    MoveData {
        id: 17, name: "WING ATTACK",
        move_type: PokemonType::Flying, power: 60, accuracy: 100, pp: 35,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #018 WHIRLWIND
    MoveData {
        id: 18, name: "WHIRLWIND",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_FORCE_SWITCH, effect_chance: 0,
        priority: 0, high_crit: false,
    },
    // #019 FLY
    MoveData {
        id: 19, name: "FLY",
        move_type: PokemonType::Flying, power: 70, accuracy: 95, pp: 15,
        is_special: false,
        effect_id: EFFECT_FLY, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #020 BIND
    MoveData {
        id: 20, name: "BIND",
        move_type: PokemonType::Normal, power: 15, accuracy: 75, pp: 20,
        is_special: false,
        effect_id: EFFECT_TRAP_TARGET, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #021 SLAM
    MoveData {
        id: 21, name: "SLAM",
        move_type: PokemonType::Normal, power: 80, accuracy: 75, pp: 20,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #022 VINE WHIP
    MoveData {
        id: 22, name: "VINE WHIP",
        move_type: PokemonType::Grass, power: 35, accuracy: 100, pp: 10,
        is_special: true,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #023 STOMP
    MoveData {
        id: 23, name: "STOMP",
        move_type: PokemonType::Normal, power: 65, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_STOMP, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #024 DOUBLE KICK
    MoveData {
        id: 24, name: "DOUBLE KICK",
        move_type: PokemonType::Fighting, power: 30, accuracy: 100, pp: 30,
        is_special: false,
        effect_id: EFFECT_DOUBLE_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #025 MEGA KICK
    MoveData {
        id: 25, name: "MEGA KICK",
        move_type: PokemonType::Normal, power: 120, accuracy: 75, pp: 5,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #026 JUMP KICK
    MoveData {
        id: 26, name: "JUMP KICK",
        move_type: PokemonType::Fighting, power: 70, accuracy: 95, pp: 25,
        is_special: false,
        effect_id: EFFECT_JUMP_KICK, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #027 ROLLING KICK
    MoveData {
        id: 27, name: "ROLLING KICK",
        move_type: PokemonType::Fighting, power: 60, accuracy: 85, pp: 15,
        is_special: false,
        effect_id: EFFECT_FLINCH_HIT, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #028 SAND-ATTACK
    MoveData {
        id: 28, name: "SAND-ATTACK",
        move_type: PokemonType::Ground, power: 0, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_ACCURACY_DOWN, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #029 HEADBUTT
    MoveData {
        id: 29, name: "HEADBUTT",
        move_type: PokemonType::Normal, power: 70, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_FLINCH_HIT, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #030 HORN ATTACK
    MoveData {
        id: 30, name: "HORN ATTACK",
        move_type: PokemonType::Normal, power: 65, accuracy: 100, pp: 25,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #031 FURY ATTACK
    MoveData {
        id: 31, name: "FURY ATTACK",
        move_type: PokemonType::Normal, power: 15, accuracy: 85, pp: 20,
        is_special: false,
        effect_id: EFFECT_MULTI_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #032 HORN DRILL
    MoveData {
        id: 32, name: "HORN DRILL",
        move_type: PokemonType::Normal, power: 1, accuracy: 30, pp: 5,
        is_special: false,
        effect_id: EFFECT_OHKO, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #033 TACKLE
    MoveData {
        id: 33, name: "TACKLE",
        move_type: PokemonType::Normal, power: 35, accuracy: 95, pp: 35,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #034 BODY SLAM
    MoveData {
        id: 34, name: "BODY SLAM",
        move_type: PokemonType::Normal, power: 85, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_PARALYZE_HIT, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #035 WRAP
    MoveData {
        id: 35, name: "WRAP",
        move_type: PokemonType::Normal, power: 15, accuracy: 85, pp: 20,
        is_special: false,
        effect_id: EFFECT_TRAP_TARGET, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #036 TAKE DOWN
    MoveData {
        id: 36, name: "TAKE DOWN",
        move_type: PokemonType::Normal, power: 90, accuracy: 85, pp: 20,
        is_special: false,
        effect_id: EFFECT_RECOIL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #037 THRASH
    MoveData {
        id: 37, name: "THRASH",
        move_type: PokemonType::Normal, power: 90, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_RAMPAGE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #038 DOUBLE-EDGE
    MoveData {
        id: 38, name: "DOUBLE-EDGE",
        move_type: PokemonType::Normal, power: 120, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_RECOIL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #039 TAIL WHIP
    MoveData {
        id: 39, name: "TAIL WHIP",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 30,
        is_special: false,
        effect_id: EFFECT_DEFENSE_DOWN, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #040 POISON STING
    MoveData {
        id: 40, name: "POISON STING",
        move_type: PokemonType::Poison, power: 15, accuracy: 100, pp: 35,
        is_special: false,
        effect_id: EFFECT_POISON_HIT, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #041 TWINEEDLE
    MoveData {
        id: 41, name: "TWINEEDLE",
        move_type: PokemonType::Bug, power: 25, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_POISON_MULTI_HIT, effect_chance: 20,
        priority: 1, high_crit: false,
    },
    // #042 PIN MISSILE
    MoveData {
        id: 42, name: "PIN MISSILE",
        move_type: PokemonType::Bug, power: 14, accuracy: 85, pp: 20,
        is_special: false,
        effect_id: EFFECT_MULTI_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #043 LEER
    MoveData {
        id: 43, name: "LEER",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 30,
        is_special: false,
        effect_id: EFFECT_DEFENSE_DOWN, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #044 BITE
    MoveData {
        id: 44, name: "BITE",
        move_type: PokemonType::Dark, power: 60, accuracy: 100, pp: 25,
        is_special: true,
        effect_id: EFFECT_FLINCH_HIT, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #045 GROWL
    MoveData {
        id: 45, name: "GROWL",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 40,
        is_special: false,
        effect_id: EFFECT_ATTACK_DOWN, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #046 ROAR
    MoveData {
        id: 46, name: "ROAR",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_FORCE_SWITCH, effect_chance: 0,
        priority: 0, high_crit: false,
    },
    // #047 SING
    MoveData {
        id: 47, name: "SING",
        move_type: PokemonType::Normal, power: 0, accuracy: 55, pp: 15,
        is_special: false,
        effect_id: EFFECT_SLEEP, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #048 SUPERSONIC
    MoveData {
        id: 48, name: "SUPERSONIC",
        move_type: PokemonType::Normal, power: 0, accuracy: 55, pp: 20,
        is_special: false,
        effect_id: EFFECT_CONFUSE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #049 SONICBOOM
    MoveData {
        id: 49, name: "SONICBOOM",
        move_type: PokemonType::Normal, power: 20, accuracy: 90, pp: 20,
        is_special: false,
        effect_id: EFFECT_STATIC_DAMAGE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #050 DISABLE
    MoveData {
        id: 50, name: "DISABLE",
        move_type: PokemonType::Normal, power: 0, accuracy: 55, pp: 20,
        is_special: false,
        effect_id: EFFECT_DISABLE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #051 ACID
    MoveData {
        id: 51, name: "ACID",
        move_type: PokemonType::Poison, power: 40, accuracy: 100, pp: 30,
        is_special: false,
        effect_id: EFFECT_DEFENSE_DOWN_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #052 EMBER
    MoveData {
        id: 52, name: "EMBER",
        move_type: PokemonType::Fire, power: 40, accuracy: 100, pp: 25,
        is_special: true,
        effect_id: EFFECT_BURN_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #053 FLAMETHROWER
    MoveData {
        id: 53, name: "FLAMETHROWER",
        move_type: PokemonType::Fire, power: 95, accuracy: 100, pp: 15,
        is_special: true,
        effect_id: EFFECT_BURN_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #054 MIST
    MoveData {
        id: 54, name: "MIST",
        move_type: PokemonType::Ice, power: 0, accuracy: 100, pp: 30,
        is_special: true,
        effect_id: EFFECT_MIST, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #055 WATER GUN
    MoveData {
        id: 55, name: "WATER GUN",
        move_type: PokemonType::Water, power: 40, accuracy: 100, pp: 25,
        is_special: true,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #056 HYDRO PUMP
    MoveData {
        id: 56, name: "HYDRO PUMP",
        move_type: PokemonType::Water, power: 120, accuracy: 80, pp: 5,
        is_special: true,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #057 SURF
    MoveData {
        id: 57, name: "SURF",
        move_type: PokemonType::Water, power: 95, accuracy: 100, pp: 15,
        is_special: true,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #058 ICE BEAM
    MoveData {
        id: 58, name: "ICE BEAM",
        move_type: PokemonType::Ice, power: 95, accuracy: 100, pp: 10,
        is_special: true,
        effect_id: EFFECT_FREEZE_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #059 BLIZZARD
    MoveData {
        id: 59, name: "BLIZZARD",
        move_type: PokemonType::Ice, power: 120, accuracy: 70, pp: 5,
        is_special: true,
        effect_id: EFFECT_FREEZE_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #060 PSYBEAM
    MoveData {
        id: 60, name: "PSYBEAM",
        move_type: PokemonType::Psychic, power: 65, accuracy: 100, pp: 20,
        is_special: true,
        effect_id: EFFECT_CONFUSE_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #061 BUBBLEBEAM
    MoveData {
        id: 61, name: "BUBBLEBEAM",
        move_type: PokemonType::Water, power: 65, accuracy: 100, pp: 20,
        is_special: true,
        effect_id: EFFECT_SPEED_DOWN_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #062 AURORA BEAM
    MoveData {
        id: 62, name: "AURORA BEAM",
        move_type: PokemonType::Ice, power: 65, accuracy: 100, pp: 20,
        is_special: true,
        effect_id: EFFECT_ATTACK_DOWN_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #063 HYPER BEAM
    MoveData {
        id: 63, name: "HYPER BEAM",
        move_type: PokemonType::Normal, power: 150, accuracy: 90, pp: 5,
        is_special: false,
        effect_id: EFFECT_HYPER_BEAM, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #064 PECK
    MoveData {
        id: 64, name: "PECK",
        move_type: PokemonType::Flying, power: 35, accuracy: 100, pp: 35,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #065 DRILL PECK
    MoveData {
        id: 65, name: "DRILL PECK",
        move_type: PokemonType::Flying, power: 80, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #066 SUBMISSION
    MoveData {
        id: 66, name: "SUBMISSION",
        move_type: PokemonType::Fighting, power: 80, accuracy: 80, pp: 25,
        is_special: false,
        effect_id: EFFECT_RECOIL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #067 LOW KICK
    MoveData {
        id: 67, name: "LOW KICK",
        move_type: PokemonType::Fighting, power: 50, accuracy: 90, pp: 20,
        is_special: false,
        effect_id: EFFECT_FLINCH_HIT, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #068 COUNTER
    MoveData {
        id: 68, name: "COUNTER",
        move_type: PokemonType::Fighting, power: 1, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_COUNTER, effect_chance: 0,
        priority: 0, high_crit: false,
    },
    // #069 SEISMIC TOSS
    MoveData {
        id: 69, name: "SEISMIC TOSS",
        move_type: PokemonType::Fighting, power: 1, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_LEVEL_DAMAGE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #070 STRENGTH
    MoveData {
        id: 70, name: "STRENGTH",
        move_type: PokemonType::Normal, power: 80, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #071 ABSORB
    MoveData {
        id: 71, name: "ABSORB",
        move_type: PokemonType::Grass, power: 20, accuracy: 100, pp: 20,
        is_special: true,
        effect_id: EFFECT_LEECH_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #072 MEGA DRAIN
    MoveData {
        id: 72, name: "MEGA DRAIN",
        move_type: PokemonType::Grass, power: 40, accuracy: 100, pp: 10,
        is_special: true,
        effect_id: EFFECT_LEECH_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #073 LEECH SEED
    MoveData {
        id: 73, name: "LEECH SEED",
        move_type: PokemonType::Grass, power: 0, accuracy: 90, pp: 10,
        is_special: true,
        effect_id: EFFECT_LEECH_SEED, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #074 GROWTH
    MoveData {
        id: 74, name: "GROWTH",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 40,
        is_special: false,
        effect_id: EFFECT_SP_ATK_UP, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #075 RAZOR LEAF
    MoveData {
        id: 75, name: "RAZOR LEAF",
        move_type: PokemonType::Grass, power: 55, accuracy: 95, pp: 25,
        is_special: true,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: true,
    },
    // #076 SOLARBEAM
    MoveData {
        id: 76, name: "SOLARBEAM",
        move_type: PokemonType::Grass, power: 120, accuracy: 100, pp: 10,
        is_special: true,
        effect_id: EFFECT_SOLARBEAM, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #077 POISONPOWDER
    MoveData {
        id: 77, name: "POISONPOWDER",
        move_type: PokemonType::Poison, power: 0, accuracy: 75, pp: 35,
        is_special: false,
        effect_id: EFFECT_POISON, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #078 STUN SPORE
    MoveData {
        id: 78, name: "STUN SPORE",
        move_type: PokemonType::Grass, power: 0, accuracy: 75, pp: 30,
        is_special: true,
        effect_id: EFFECT_PARALYZE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #079 SLEEP POWDER
    MoveData {
        id: 79, name: "SLEEP POWDER",
        move_type: PokemonType::Grass, power: 0, accuracy: 75, pp: 15,
        is_special: true,
        effect_id: EFFECT_SLEEP, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #080 PETAL DANCE
    MoveData {
        id: 80, name: "PETAL DANCE",
        move_type: PokemonType::Grass, power: 70, accuracy: 100, pp: 20,
        is_special: true,
        effect_id: EFFECT_RAMPAGE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #081 STRING SHOT
    MoveData {
        id: 81, name: "STRING SHOT",
        move_type: PokemonType::Bug, power: 0, accuracy: 95, pp: 40,
        is_special: false,
        effect_id: EFFECT_SPEED_DOWN, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #082 DRAGON RAGE
    MoveData {
        id: 82, name: "DRAGON RAGE",
        move_type: PokemonType::Dragon, power: 40, accuracy: 100, pp: 10,
        is_special: true,
        effect_id: EFFECT_STATIC_DAMAGE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #083 FIRE SPIN
    MoveData {
        id: 83, name: "FIRE SPIN",
        move_type: PokemonType::Fire, power: 15, accuracy: 70, pp: 15,
        is_special: true,
        effect_id: EFFECT_TRAP_TARGET, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #084 THUNDERSHOCK
    MoveData {
        id: 84, name: "THUNDERSHOCK",
        move_type: PokemonType::Electric, power: 40, accuracy: 100, pp: 30,
        is_special: true,
        effect_id: EFFECT_PARALYZE_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #085 THUNDERBOLT
    MoveData {
        id: 85, name: "THUNDERBOLT",
        move_type: PokemonType::Electric, power: 95, accuracy: 100, pp: 15,
        is_special: true,
        effect_id: EFFECT_PARALYZE_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #086 THUNDER WAVE
    MoveData {
        id: 86, name: "THUNDER WAVE",
        move_type: PokemonType::Electric, power: 0, accuracy: 100, pp: 20,
        is_special: true,
        effect_id: EFFECT_PARALYZE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #087 THUNDER
    MoveData {
        id: 87, name: "THUNDER",
        move_type: PokemonType::Electric, power: 120, accuracy: 70, pp: 10,
        is_special: true,
        effect_id: EFFECT_THUNDER, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #088 ROCK THROW
    MoveData {
        id: 88, name: "ROCK THROW",
        move_type: PokemonType::Rock, power: 50, accuracy: 90, pp: 15,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #089 EARTHQUAKE
    MoveData {
        id: 89, name: "EARTHQUAKE",
        move_type: PokemonType::Ground, power: 100, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_EARTHQUAKE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #090 FISSURE
    MoveData {
        id: 90, name: "FISSURE",
        move_type: PokemonType::Ground, power: 1, accuracy: 30, pp: 5,
        is_special: false,
        effect_id: EFFECT_OHKO, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #091 DIG
    MoveData {
        id: 91, name: "DIG",
        move_type: PokemonType::Ground, power: 60, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_FLY, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #092 TOXIC
    MoveData {
        id: 92, name: "TOXIC",
        move_type: PokemonType::Poison, power: 0, accuracy: 85, pp: 10,
        is_special: false,
        effect_id: EFFECT_TOXIC, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #093 CONFUSION
    MoveData {
        id: 93, name: "CONFUSION",
        move_type: PokemonType::Psychic, power: 50, accuracy: 100, pp: 25,
        is_special: true,
        effect_id: EFFECT_CONFUSE_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #094 PSYCHIC
    MoveData {
        id: 94, name: "PSYCHIC",
        move_type: PokemonType::Psychic, power: 90, accuracy: 100, pp: 10,
        is_special: true,
        effect_id: EFFECT_SP_DEF_DOWN_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #095 HYPNOSIS
    MoveData {
        id: 95, name: "HYPNOSIS",
        move_type: PokemonType::Psychic, power: 0, accuracy: 60, pp: 20,
        is_special: true,
        effect_id: EFFECT_SLEEP, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #096 MEDITATE
    MoveData {
        id: 96, name: "MEDITATE",
        move_type: PokemonType::Psychic, power: 0, accuracy: 100, pp: 40,
        is_special: true,
        effect_id: EFFECT_ATTACK_UP, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #097 AGILITY
    MoveData {
        id: 97, name: "AGILITY",
        move_type: PokemonType::Psychic, power: 0, accuracy: 100, pp: 30,
        is_special: true,
        effect_id: EFFECT_SPEED_UP_2, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #098 QUICK ATTACK
    MoveData {
        id: 98, name: "QUICK ATTACK",
        move_type: PokemonType::Normal, power: 40, accuracy: 100, pp: 30,
        is_special: false,
        effect_id: EFFECT_PRIORITY_HIT, effect_chance: 0,
        priority: 2, high_crit: false,
    },
    // #099 RAGE
    MoveData {
        id: 99, name: "RAGE",
        move_type: PokemonType::Normal, power: 20, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_RAGE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #100 TELEPORT
    MoveData {
        id: 100, name: "TELEPORT",
        move_type: PokemonType::Psychic, power: 0, accuracy: 100, pp: 20,
        is_special: true,
        effect_id: EFFECT_TELEPORT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #101 NIGHT SHADE
    MoveData {
        id: 101, name: "NIGHT SHADE",
        move_type: PokemonType::Ghost, power: 1, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_LEVEL_DAMAGE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #102 MIMIC
    MoveData {
        id: 102, name: "MIMIC",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_MIMIC, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #103 SCREECH
    MoveData {
        id: 103, name: "SCREECH",
        move_type: PokemonType::Normal, power: 0, accuracy: 85, pp: 40,
        is_special: false,
        effect_id: EFFECT_DEFENSE_DOWN_2, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #104 DOUBLE TEAM
    MoveData {
        id: 104, name: "DOUBLE TEAM",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_EVASION_UP, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #105 RECOVER
    MoveData {
        id: 105, name: "RECOVER",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_HEAL, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #106 HARDEN
    MoveData {
        id: 106, name: "HARDEN",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 30,
        is_special: false,
        effect_id: EFFECT_DEFENSE_UP, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #107 MINIMIZE
    MoveData {
        id: 107, name: "MINIMIZE",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_EVASION_UP, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #108 SMOKESCREEN
    MoveData {
        id: 108, name: "SMOKESCREEN",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_ACCURACY_DOWN, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #109 CONFUSE RAY
    MoveData {
        id: 109, name: "CONFUSE RAY",
        move_type: PokemonType::Ghost, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_CONFUSE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #110 WITHDRAW
    MoveData {
        id: 110, name: "WITHDRAW",
        move_type: PokemonType::Water, power: 0, accuracy: 100, pp: 40,
        is_special: true,
        effect_id: EFFECT_DEFENSE_UP, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #111 DEFENSE CURL
    MoveData {
        id: 111, name: "DEFENSE CURL",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 40,
        is_special: false,
        effect_id: EFFECT_DEFENSE_CURL, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #112 BARRIER
    MoveData {
        id: 112, name: "BARRIER",
        move_type: PokemonType::Psychic, power: 0, accuracy: 100, pp: 30,
        is_special: true,
        effect_id: EFFECT_DEFENSE_UP_2, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #113 LIGHT SCREEN
    MoveData {
        id: 113, name: "LIGHT SCREEN",
        move_type: PokemonType::Psychic, power: 0, accuracy: 100, pp: 30,
        is_special: true,
        effect_id: EFFECT_LIGHT_SCREEN, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #114 HAZE
    MoveData {
        id: 114, name: "HAZE",
        move_type: PokemonType::Ice, power: 0, accuracy: 100, pp: 30,
        is_special: true,
        effect_id: EFFECT_RESET_STATS, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #115 REFLECT
    MoveData {
        id: 115, name: "REFLECT",
        move_type: PokemonType::Psychic, power: 0, accuracy: 100, pp: 20,
        is_special: true,
        effect_id: EFFECT_REFLECT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #116 FOCUS ENERGY
    MoveData {
        id: 116, name: "FOCUS ENERGY",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 30,
        is_special: false,
        effect_id: EFFECT_FOCUS_ENERGY, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #117 BIDE
    MoveData {
        id: 117, name: "BIDE",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_BIDE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #118 METRONOME
    MoveData {
        id: 118, name: "METRONOME",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_METRONOME, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #119 MIRROR MOVE
    MoveData {
        id: 119, name: "MIRROR MOVE",
        move_type: PokemonType::Flying, power: 0, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_MIRROR_MOVE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #120 SELFDESTRUCT
    MoveData {
        id: 120, name: "SELFDESTRUCT",
        move_type: PokemonType::Normal, power: 200, accuracy: 100, pp: 5,
        is_special: false,
        effect_id: EFFECT_SELFDESTRUCT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #121 EGG BOMB
    MoveData {
        id: 121, name: "EGG BOMB",
        move_type: PokemonType::Normal, power: 100, accuracy: 75, pp: 10,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #122 LICK
    MoveData {
        id: 122, name: "LICK",
        move_type: PokemonType::Ghost, power: 20, accuracy: 100, pp: 30,
        is_special: false,
        effect_id: EFFECT_PARALYZE_HIT, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #123 SMOG
    MoveData {
        id: 123, name: "SMOG",
        move_type: PokemonType::Poison, power: 20, accuracy: 70, pp: 20,
        is_special: false,
        effect_id: EFFECT_POISON_HIT, effect_chance: 40,
        priority: 1, high_crit: false,
    },
    // #124 SLUDGE
    MoveData {
        id: 124, name: "SLUDGE",
        move_type: PokemonType::Poison, power: 65, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_POISON_HIT, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #125 BONE CLUB
    MoveData {
        id: 125, name: "BONE CLUB",
        move_type: PokemonType::Ground, power: 65, accuracy: 85, pp: 20,
        is_special: false,
        effect_id: EFFECT_FLINCH_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #126 FIRE BLAST
    MoveData {
        id: 126, name: "FIRE BLAST",
        move_type: PokemonType::Fire, power: 120, accuracy: 85, pp: 5,
        is_special: true,
        effect_id: EFFECT_BURN_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #127 WATERFALL
    MoveData {
        id: 127, name: "WATERFALL",
        move_type: PokemonType::Water, power: 80, accuracy: 100, pp: 15,
        is_special: true,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #128 CLAMP
    MoveData {
        id: 128, name: "CLAMP",
        move_type: PokemonType::Water, power: 35, accuracy: 75, pp: 10,
        is_special: true,
        effect_id: EFFECT_TRAP_TARGET, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #129 SWIFT
    MoveData {
        id: 129, name: "SWIFT",
        move_type: PokemonType::Normal, power: 60, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_ALWAYS_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #130 SKULL BASH
    MoveData {
        id: 130, name: "SKULL BASH",
        move_type: PokemonType::Normal, power: 100, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_SKULL_BASH, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #131 SPIKE CANNON
    MoveData {
        id: 131, name: "SPIKE CANNON",
        move_type: PokemonType::Normal, power: 20, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_MULTI_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #132 CONSTRICT
    MoveData {
        id: 132, name: "CONSTRICT",
        move_type: PokemonType::Normal, power: 10, accuracy: 100, pp: 35,
        is_special: false,
        effect_id: EFFECT_SPEED_DOWN_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #133 AMNESIA
    MoveData {
        id: 133, name: "AMNESIA",
        move_type: PokemonType::Psychic, power: 0, accuracy: 100, pp: 20,
        is_special: true,
        effect_id: EFFECT_SP_DEF_UP_2, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #134 KINESIS
    MoveData {
        id: 134, name: "KINESIS",
        move_type: PokemonType::Psychic, power: 0, accuracy: 80, pp: 15,
        is_special: true,
        effect_id: EFFECT_ACCURACY_DOWN, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #135 SOFTBOILED
    MoveData {
        id: 135, name: "SOFTBOILED",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_HEAL, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #136 HI JUMP KICK
    MoveData {
        id: 136, name: "HI JUMP KICK",
        move_type: PokemonType::Fighting, power: 85, accuracy: 90, pp: 20,
        is_special: false,
        effect_id: EFFECT_JUMP_KICK, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #137 GLARE
    MoveData {
        id: 137, name: "GLARE",
        move_type: PokemonType::Normal, power: 0, accuracy: 75, pp: 30,
        is_special: false,
        effect_id: EFFECT_PARALYZE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #138 DREAM EATER
    MoveData {
        id: 138, name: "DREAM EATER",
        move_type: PokemonType::Psychic, power: 100, accuracy: 100, pp: 15,
        is_special: true,
        effect_id: EFFECT_DREAM_EATER, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #139 POISON GAS
    MoveData {
        id: 139, name: "POISON GAS",
        move_type: PokemonType::Poison, power: 0, accuracy: 55, pp: 40,
        is_special: false,
        effect_id: EFFECT_POISON, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #140 BARRAGE
    MoveData {
        id: 140, name: "BARRAGE",
        move_type: PokemonType::Normal, power: 15, accuracy: 85, pp: 20,
        is_special: false,
        effect_id: EFFECT_MULTI_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #141 LEECH LIFE
    MoveData {
        id: 141, name: "LEECH LIFE",
        move_type: PokemonType::Bug, power: 20, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_LEECH_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #142 LOVELY KISS
    MoveData {
        id: 142, name: "LOVELY KISS",
        move_type: PokemonType::Normal, power: 0, accuracy: 75, pp: 10,
        is_special: false,
        effect_id: EFFECT_SLEEP, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #143 SKY ATTACK
    MoveData {
        id: 143, name: "SKY ATTACK",
        move_type: PokemonType::Flying, power: 140, accuracy: 90, pp: 5,
        is_special: false,
        effect_id: EFFECT_SKY_ATTACK, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #144 TRANSFORM
    MoveData {
        id: 144, name: "TRANSFORM",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_TRANSFORM, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #145 BUBBLE
    MoveData {
        id: 145, name: "BUBBLE",
        move_type: PokemonType::Water, power: 20, accuracy: 100, pp: 30,
        is_special: true,
        effect_id: EFFECT_SPEED_DOWN_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #146 DIZZY PUNCH
    MoveData {
        id: 146, name: "DIZZY PUNCH",
        move_type: PokemonType::Normal, power: 70, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_CONFUSE_HIT, effect_chance: 20,
        priority: 1, high_crit: false,
    },
    // #147 SPORE
    MoveData {
        id: 147, name: "SPORE",
        move_type: PokemonType::Grass, power: 0, accuracy: 100, pp: 15,
        is_special: true,
        effect_id: EFFECT_SLEEP, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #148 FLASH
    MoveData {
        id: 148, name: "FLASH",
        move_type: PokemonType::Normal, power: 0, accuracy: 70, pp: 20,
        is_special: false,
        effect_id: EFFECT_ACCURACY_DOWN, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #149 PSYWAVE
    MoveData {
        id: 149, name: "PSYWAVE",
        move_type: PokemonType::Psychic, power: 1, accuracy: 80, pp: 15,
        is_special: true,
        effect_id: EFFECT_PSYWAVE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #150 SPLASH
    MoveData {
        id: 150, name: "SPLASH",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 40,
        is_special: false,
        effect_id: EFFECT_SPLASH, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #151 ACID ARMOR
    MoveData {
        id: 151, name: "ACID ARMOR",
        move_type: PokemonType::Poison, power: 0, accuracy: 100, pp: 40,
        is_special: false,
        effect_id: EFFECT_DEFENSE_UP_2, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #152 CRABHAMMER
    MoveData {
        id: 152, name: "CRABHAMMER",
        move_type: PokemonType::Water, power: 90, accuracy: 85, pp: 10,
        is_special: true,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: true,
    },
    // #153 EXPLOSION
    MoveData {
        id: 153, name: "EXPLOSION",
        move_type: PokemonType::Normal, power: 250, accuracy: 100, pp: 5,
        is_special: false,
        effect_id: EFFECT_SELFDESTRUCT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #154 FURY SWIPES
    MoveData {
        id: 154, name: "FURY SWIPES",
        move_type: PokemonType::Normal, power: 18, accuracy: 80, pp: 15,
        is_special: false,
        effect_id: EFFECT_MULTI_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #155 BONEMERANG
    MoveData {
        id: 155, name: "BONEMERANG",
        move_type: PokemonType::Ground, power: 50, accuracy: 90, pp: 10,
        is_special: false,
        effect_id: EFFECT_DOUBLE_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #156 REST
    MoveData {
        id: 156, name: "REST",
        move_type: PokemonType::Psychic, power: 0, accuracy: 100, pp: 10,
        is_special: true,
        effect_id: EFFECT_HEAL, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #157 ROCK SLIDE
    MoveData {
        id: 157, name: "ROCK SLIDE",
        move_type: PokemonType::Rock, power: 75, accuracy: 90, pp: 10,
        is_special: false,
        effect_id: EFFECT_FLINCH_HIT, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #158 HYPER FANG
    MoveData {
        id: 158, name: "HYPER FANG",
        move_type: PokemonType::Normal, power: 80, accuracy: 90, pp: 15,
        is_special: false,
        effect_id: EFFECT_FLINCH_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #159 SHARPEN
    MoveData {
        id: 159, name: "SHARPEN",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 30,
        is_special: false,
        effect_id: EFFECT_ATTACK_UP, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #160 CONVERSION
    MoveData {
        id: 160, name: "CONVERSION",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 30,
        is_special: false,
        effect_id: EFFECT_CONVERSION, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #161 TRI ATTACK
    MoveData {
        id: 161, name: "TRI ATTACK",
        move_type: PokemonType::Normal, power: 80, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_TRI_ATTACK, effect_chance: 20,
        priority: 1, high_crit: false,
    },
    // #162 SUPER FANG
    MoveData {
        id: 162, name: "SUPER FANG",
        move_type: PokemonType::Normal, power: 1, accuracy: 90, pp: 10,
        is_special: false,
        effect_id: EFFECT_SUPER_FANG, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #163 SLASH
    MoveData {
        id: 163, name: "SLASH",
        move_type: PokemonType::Normal, power: 70, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: true,
    },
    // #164 SUBSTITUTE
    MoveData {
        id: 164, name: "SUBSTITUTE",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_SUBSTITUTE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #165 STRUGGLE
    MoveData {
        id: 165, name: "STRUGGLE",
        move_type: PokemonType::Normal, power: 50, accuracy: 100, pp: 1,
        is_special: false,
        effect_id: EFFECT_RECOIL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #166 SKETCH
    MoveData {
        id: 166, name: "SKETCH",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 1,
        is_special: false,
        effect_id: EFFECT_SKETCH, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #167 TRIPLE KICK
    MoveData {
        id: 167, name: "TRIPLE KICK",
        move_type: PokemonType::Fighting, power: 10, accuracy: 90, pp: 10,
        is_special: false,
        effect_id: EFFECT_TRIPLE_KICK, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #168 THIEF
    MoveData {
        id: 168, name: "THIEF",
        move_type: PokemonType::Dark, power: 40, accuracy: 100, pp: 10,
        is_special: true,
        effect_id: EFFECT_THIEF, effect_chance: 100,
        priority: 1, high_crit: false,
    },
    // #169 SPIDER WEB
    MoveData {
        id: 169, name: "SPIDER WEB",
        move_type: PokemonType::Bug, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_MEAN_LOOK, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #170 MIND READER
    MoveData {
        id: 170, name: "MIND READER",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 5,
        is_special: false,
        effect_id: EFFECT_LOCK_ON, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #171 NIGHTMARE
    MoveData {
        id: 171, name: "NIGHTMARE",
        move_type: PokemonType::Ghost, power: 0, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_NIGHTMARE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #172 FLAME WHEEL
    MoveData {
        id: 172, name: "FLAME WHEEL",
        move_type: PokemonType::Fire, power: 60, accuracy: 100, pp: 25,
        is_special: true,
        effect_id: EFFECT_FLAME_WHEEL, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #173 SNORE
    MoveData {
        id: 173, name: "SNORE",
        move_type: PokemonType::Normal, power: 40, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_SNORE, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #174 CURSE
    MoveData {
        id: 174, name: "CURSE",
        move_type: PokemonType::Ghost, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_CURSE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #175 FLAIL
    MoveData {
        id: 175, name: "FLAIL",
        move_type: PokemonType::Normal, power: 1, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_REVERSAL, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #176 CONVERSION2
    MoveData {
        id: 176, name: "CONVERSION2",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 30,
        is_special: false,
        effect_id: EFFECT_CONVERSION2, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #177 AEROBLAST
    MoveData {
        id: 177, name: "AEROBLAST",
        move_type: PokemonType::Flying, power: 100, accuracy: 95, pp: 5,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: true,
    },
    // #178 COTTON SPORE
    MoveData {
        id: 178, name: "COTTON SPORE",
        move_type: PokemonType::Grass, power: 0, accuracy: 85, pp: 40,
        is_special: true,
        effect_id: EFFECT_SPEED_DOWN_2, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #179 REVERSAL
    MoveData {
        id: 179, name: "REVERSAL",
        move_type: PokemonType::Fighting, power: 1, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_REVERSAL, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #180 SPITE
    MoveData {
        id: 180, name: "SPITE",
        move_type: PokemonType::Ghost, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_SPITE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #181 POWDER SNOW
    MoveData {
        id: 181, name: "POWDER SNOW",
        move_type: PokemonType::Ice, power: 40, accuracy: 100, pp: 25,
        is_special: true,
        effect_id: EFFECT_FREEZE_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #182 PROTECT
    MoveData {
        id: 182, name: "PROTECT",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_PROTECT, effect_chance: 0,
        priority: 3, high_crit: false,
    },
    // #183 MACH PUNCH
    MoveData {
        id: 183, name: "MACH PUNCH",
        move_type: PokemonType::Fighting, power: 40, accuracy: 100, pp: 30,
        is_special: false,
        effect_id: EFFECT_PRIORITY_HIT, effect_chance: 0,
        priority: 2, high_crit: false,
    },
    // #184 SCARY FACE
    MoveData {
        id: 184, name: "SCARY FACE",
        move_type: PokemonType::Normal, power: 0, accuracy: 90, pp: 10,
        is_special: false,
        effect_id: EFFECT_SPEED_DOWN_2, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #185 FAINT ATTACK
    MoveData {
        id: 185, name: "FAINT ATTACK",
        move_type: PokemonType::Dark, power: 60, accuracy: 100, pp: 20,
        is_special: true,
        effect_id: EFFECT_ALWAYS_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #186 SWEET KISS
    MoveData {
        id: 186, name: "SWEET KISS",
        move_type: PokemonType::Normal, power: 0, accuracy: 75, pp: 10,
        is_special: false,
        effect_id: EFFECT_CONFUSE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #187 BELLY DRUM
    MoveData {
        id: 187, name: "BELLY DRUM",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_BELLY_DRUM, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #188 SLUDGE BOMB
    MoveData {
        id: 188, name: "SLUDGE BOMB",
        move_type: PokemonType::Poison, power: 90, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_POISON_HIT, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #189 MUD-SLAP
    MoveData {
        id: 189, name: "MUD-SLAP",
        move_type: PokemonType::Ground, power: 20, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_ACCURACY_DOWN_HIT, effect_chance: 100,
        priority: 1, high_crit: false,
    },
    // #190 OCTAZOOKA
    MoveData {
        id: 190, name: "OCTAZOOKA",
        move_type: PokemonType::Water, power: 65, accuracy: 85, pp: 10,
        is_special: true,
        effect_id: EFFECT_ACCURACY_DOWN_HIT, effect_chance: 50,
        priority: 1, high_crit: false,
    },
    // #191 SPIKES
    MoveData {
        id: 191, name: "SPIKES",
        move_type: PokemonType::Ground, power: 0, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_SPIKES, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #192 ZAP CANNON
    MoveData {
        id: 192, name: "ZAP CANNON",
        move_type: PokemonType::Electric, power: 100, accuracy: 50, pp: 5,
        is_special: true,
        effect_id: EFFECT_PARALYZE_HIT, effect_chance: 100,
        priority: 1, high_crit: false,
    },
    // #193 FORESIGHT
    MoveData {
        id: 193, name: "FORESIGHT",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 40,
        is_special: false,
        effect_id: EFFECT_FORESIGHT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #194 DESTINY BOND
    MoveData {
        id: 194, name: "DESTINY BOND",
        move_type: PokemonType::Ghost, power: 0, accuracy: 100, pp: 5,
        is_special: false,
        effect_id: EFFECT_DESTINY_BOND, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #195 PERISH SONG
    MoveData {
        id: 195, name: "PERISH SONG",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 5,
        is_special: false,
        effect_id: EFFECT_PERISH_SONG, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #196 ICY WIND
    MoveData {
        id: 196, name: "ICY WIND",
        move_type: PokemonType::Ice, power: 55, accuracy: 95, pp: 15,
        is_special: true,
        effect_id: EFFECT_SPEED_DOWN_HIT, effect_chance: 100,
        priority: 1, high_crit: false,
    },
    // #197 DETECT
    MoveData {
        id: 197, name: "DETECT",
        move_type: PokemonType::Fighting, power: 0, accuracy: 100, pp: 5,
        is_special: false,
        effect_id: EFFECT_PROTECT, effect_chance: 0,
        priority: 3, high_crit: false,
    },
    // #198 BONE RUSH
    MoveData {
        id: 198, name: "BONE RUSH",
        move_type: PokemonType::Ground, power: 25, accuracy: 80, pp: 10,
        is_special: false,
        effect_id: EFFECT_MULTI_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #199 LOCK-ON
    MoveData {
        id: 199, name: "LOCK-ON",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 5,
        is_special: false,
        effect_id: EFFECT_LOCK_ON, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #200 OUTRAGE
    MoveData {
        id: 200, name: "OUTRAGE",
        move_type: PokemonType::Dragon, power: 90, accuracy: 100, pp: 15,
        is_special: true,
        effect_id: EFFECT_RAMPAGE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #201 SANDSTORM
    MoveData {
        id: 201, name: "SANDSTORM",
        move_type: PokemonType::Rock, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_SANDSTORM, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #202 GIGA DRAIN
    MoveData {
        id: 202, name: "GIGA DRAIN",
        move_type: PokemonType::Grass, power: 60, accuracy: 100, pp: 5,
        is_special: true,
        effect_id: EFFECT_LEECH_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #203 ENDURE
    MoveData {
        id: 203, name: "ENDURE",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_ENDURE, effect_chance: 0,
        priority: 3, high_crit: false,
    },
    // #204 CHARM
    MoveData {
        id: 204, name: "CHARM",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_ATTACK_DOWN_2, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #205 ROLLOUT
    MoveData {
        id: 205, name: "ROLLOUT",
        move_type: PokemonType::Rock, power: 30, accuracy: 90, pp: 20,
        is_special: false,
        effect_id: EFFECT_ROLLOUT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #206 FALSE SWIPE
    MoveData {
        id: 206, name: "FALSE SWIPE",
        move_type: PokemonType::Normal, power: 40, accuracy: 100, pp: 40,
        is_special: false,
        effect_id: EFFECT_FALSE_SWIPE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #207 SWAGGER
    MoveData {
        id: 207, name: "SWAGGER",
        move_type: PokemonType::Normal, power: 0, accuracy: 90, pp: 15,
        is_special: false,
        effect_id: EFFECT_SWAGGER, effect_chance: 100,
        priority: 1, high_crit: false,
    },
    // #208 MILK DRINK
    MoveData {
        id: 208, name: "MILK DRINK",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_HEAL, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #209 SPARK
    MoveData {
        id: 209, name: "SPARK",
        move_type: PokemonType::Electric, power: 65, accuracy: 100, pp: 20,
        is_special: true,
        effect_id: EFFECT_PARALYZE_HIT, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #210 FURY CUTTER
    MoveData {
        id: 210, name: "FURY CUTTER",
        move_type: PokemonType::Bug, power: 10, accuracy: 95, pp: 20,
        is_special: false,
        effect_id: EFFECT_FURY_CUTTER, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #211 STEEL WING
    MoveData {
        id: 211, name: "STEEL WING",
        move_type: PokemonType::Steel, power: 70, accuracy: 90, pp: 25,
        is_special: false,
        effect_id: EFFECT_DEFENSE_UP_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #212 MEAN LOOK
    MoveData {
        id: 212, name: "MEAN LOOK",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 5,
        is_special: false,
        effect_id: EFFECT_MEAN_LOOK, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #213 ATTRACT
    MoveData {
        id: 213, name: "ATTRACT",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_ATTRACT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #214 SLEEP TALK
    MoveData {
        id: 214, name: "SLEEP TALK",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_SLEEP_TALK, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #215 HEAL BELL
    MoveData {
        id: 215, name: "HEAL BELL",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 5,
        is_special: false,
        effect_id: EFFECT_HEAL_BELL, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #216 RETURN
    MoveData {
        id: 216, name: "RETURN",
        move_type: PokemonType::Normal, power: 1, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_RETURN, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #217 PRESENT
    MoveData {
        id: 217, name: "PRESENT",
        move_type: PokemonType::Normal, power: 1, accuracy: 90, pp: 15,
        is_special: false,
        effect_id: EFFECT_PRESENT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #218 FRUSTRATION
    MoveData {
        id: 218, name: "FRUSTRATION",
        move_type: PokemonType::Normal, power: 1, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_FRUSTRATION, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #219 SAFEGUARD
    MoveData {
        id: 219, name: "SAFEGUARD",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 25,
        is_special: false,
        effect_id: EFFECT_SAFEGUARD, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #220 PAIN SPLIT
    MoveData {
        id: 220, name: "PAIN SPLIT",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_PAIN_SPLIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #221 SACRED FIRE
    MoveData {
        id: 221, name: "SACRED FIRE",
        move_type: PokemonType::Fire, power: 100, accuracy: 95, pp: 5,
        is_special: true,
        effect_id: EFFECT_SACRED_FIRE, effect_chance: 50,
        priority: 1, high_crit: false,
    },
    // #222 MAGNITUDE
    MoveData {
        id: 222, name: "MAGNITUDE",
        move_type: PokemonType::Ground, power: 1, accuracy: 100, pp: 30,
        is_special: false,
        effect_id: EFFECT_MAGNITUDE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #223 DYNAMICPUNCH
    MoveData {
        id: 223, name: "DYNAMICPUNCH",
        move_type: PokemonType::Fighting, power: 100, accuracy: 50, pp: 5,
        is_special: false,
        effect_id: EFFECT_CONFUSE_HIT, effect_chance: 100,
        priority: 1, high_crit: false,
    },
    // #224 MEGAHORN
    MoveData {
        id: 224, name: "MEGAHORN",
        move_type: PokemonType::Bug, power: 120, accuracy: 85, pp: 10,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #225 DRAGONBREATH
    MoveData {
        id: 225, name: "DRAGONBREATH",
        move_type: PokemonType::Dragon, power: 60, accuracy: 100, pp: 20,
        is_special: true,
        effect_id: EFFECT_PARALYZE_HIT, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #226 BATON PASS
    MoveData {
        id: 226, name: "BATON PASS",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 40,
        is_special: false,
        effect_id: EFFECT_BATON_PASS, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #227 ENCORE
    MoveData {
        id: 227, name: "ENCORE",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 5,
        is_special: false,
        effect_id: EFFECT_ENCORE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #228 PURSUIT
    MoveData {
        id: 228, name: "PURSUIT",
        move_type: PokemonType::Dark, power: 40, accuracy: 100, pp: 20,
        is_special: true,
        effect_id: EFFECT_PURSUIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #229 RAPID SPIN
    MoveData {
        id: 229, name: "RAPID SPIN",
        move_type: PokemonType::Normal, power: 20, accuracy: 100, pp: 40,
        is_special: false,
        effect_id: EFFECT_RAPID_SPIN, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #230 SWEET SCENT
    MoveData {
        id: 230, name: "SWEET SCENT",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 20,
        is_special: false,
        effect_id: EFFECT_EVASION_DOWN, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #231 IRON TAIL
    MoveData {
        id: 231, name: "IRON TAIL",
        move_type: PokemonType::Steel, power: 100, accuracy: 75, pp: 15,
        is_special: false,
        effect_id: EFFECT_DEFENSE_DOWN_HIT, effect_chance: 30,
        priority: 1, high_crit: false,
    },
    // #232 METAL CLAW
    MoveData {
        id: 232, name: "METAL CLAW",
        move_type: PokemonType::Steel, power: 50, accuracy: 95, pp: 35,
        is_special: false,
        effect_id: EFFECT_ATTACK_UP_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #233 VITAL THROW
    MoveData {
        id: 233, name: "VITAL THROW",
        move_type: PokemonType::Fighting, power: 70, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_ALWAYS_HIT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #234 MORNING SUN
    MoveData {
        id: 234, name: "MORNING SUN",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 5,
        is_special: false,
        effect_id: EFFECT_MORNING_SUN, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #235 SYNTHESIS
    MoveData {
        id: 235, name: "SYNTHESIS",
        move_type: PokemonType::Grass, power: 0, accuracy: 100, pp: 5,
        is_special: true,
        effect_id: EFFECT_SYNTHESIS, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #236 MOONLIGHT
    MoveData {
        id: 236, name: "MOONLIGHT",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 5,
        is_special: false,
        effect_id: EFFECT_MOONLIGHT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #237 HIDDEN POWER
    MoveData {
        id: 237, name: "HIDDEN POWER",
        move_type: PokemonType::Normal, power: 1, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_HIDDEN_POWER, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #238 CROSS CHOP
    MoveData {
        id: 238, name: "CROSS CHOP",
        move_type: PokemonType::Fighting, power: 100, accuracy: 80, pp: 5,
        is_special: false,
        effect_id: EFFECT_NORMAL_HIT, effect_chance: 0,
        priority: 1, high_crit: true,
    },
    // #239 TWISTER
    MoveData {
        id: 239, name: "TWISTER",
        move_type: PokemonType::Dragon, power: 40, accuracy: 100, pp: 20,
        is_special: true,
        effect_id: EFFECT_TWISTER, effect_chance: 20,
        priority: 1, high_crit: false,
    },
    // #240 RAIN DANCE
    MoveData {
        id: 240, name: "RAIN DANCE",
        move_type: PokemonType::Water, power: 0, accuracy: 90, pp: 5,
        is_special: true,
        effect_id: EFFECT_RAIN_DANCE, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #241 SUNNY DAY
    MoveData {
        id: 241, name: "SUNNY DAY",
        move_type: PokemonType::Fire, power: 0, accuracy: 90, pp: 5,
        is_special: true,
        effect_id: EFFECT_SUNNY_DAY, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #242 CRUNCH
    MoveData {
        id: 242, name: "CRUNCH",
        move_type: PokemonType::Dark, power: 80, accuracy: 100, pp: 15,
        is_special: true,
        effect_id: EFFECT_SP_DEF_DOWN_HIT, effect_chance: 20,
        priority: 1, high_crit: false,
    },
    // #243 MIRROR COAT
    MoveData {
        id: 243, name: "MIRROR COAT",
        move_type: PokemonType::Psychic, power: 1, accuracy: 100, pp: 20,
        is_special: true,
        effect_id: EFFECT_MIRROR_COAT, effect_chance: 0,
        priority: 0, high_crit: false,
    },
    // #244 PSYCH UP
    MoveData {
        id: 244, name: "PSYCH UP",
        move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 10,
        is_special: false,
        effect_id: EFFECT_PSYCH_UP, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #245 EXTREMESPEED
    MoveData {
        id: 245, name: "EXTREMESPEED",
        move_type: PokemonType::Normal, power: 80, accuracy: 100, pp: 5,
        is_special: false,
        effect_id: EFFECT_PRIORITY_HIT, effect_chance: 0,
        priority: 2, high_crit: false,
    },
    // #246 ANCIENTPOWER
    MoveData {
        id: 246, name: "ANCIENTPOWER",
        move_type: PokemonType::Rock, power: 60, accuracy: 100, pp: 5,
        is_special: false,
        effect_id: EFFECT_ALL_UP_HIT, effect_chance: 10,
        priority: 1, high_crit: false,
    },
    // #247 SHADOW BALL
    MoveData {
        id: 247, name: "SHADOW BALL",
        move_type: PokemonType::Ghost, power: 80, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_SP_DEF_DOWN_HIT, effect_chance: 20,
        priority: 1, high_crit: false,
    },
    // #248 FUTURE SIGHT
    MoveData {
        id: 248, name: "FUTURE SIGHT",
        move_type: PokemonType::Psychic, power: 80, accuracy: 90, pp: 15,
        is_special: true,
        effect_id: EFFECT_FUTURE_SIGHT, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #249 ROCK SMASH
    MoveData {
        id: 249, name: "ROCK SMASH",
        move_type: PokemonType::Fighting, power: 20, accuracy: 100, pp: 15,
        is_special: false,
        effect_id: EFFECT_DEFENSE_DOWN_HIT, effect_chance: 50,
        priority: 1, high_crit: false,
    },
    // #250 WHIRLPOOL
    MoveData {
        id: 250, name: "WHIRLPOOL",
        move_type: PokemonType::Water, power: 15, accuracy: 70, pp: 15,
        is_special: true,
        effect_id: EFFECT_TRAP_TARGET, effect_chance: 0,
        priority: 1, high_crit: false,
    },
    // #251 BEAT UP
    MoveData {
        id: 251, name: "BEAT UP",
        move_type: PokemonType::Dark, power: 10, accuracy: 100, pp: 10,
        is_special: true,
        effect_id: EFFECT_BEAT_UP, effect_chance: 0,
        priority: 1, high_crit: false,
    },
];

/// Return move data for the given id. Returns Tackle data for unknown/invalid moves.
pub fn move_data(id: MoveId) -> &'static MoveData {
    let idx = id as usize;
    if idx >= 1 && idx <= NUM_MOVES {
        &MOVE_TABLE[idx - 1]
    } else {
        &MOVE_TABLE[32] // Tackle is move #33, index 32
    }
}
