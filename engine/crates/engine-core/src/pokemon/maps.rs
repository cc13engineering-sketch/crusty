// AI-INSTRUCTIONS: Pokemon map data module. Contains all map definitions for a
// Pokemon Gold/Silver/Crystal recreation. Maps are tile-based grids stored row-by-row.
// Each map has visual tile IDs, collision data, warp points, NPC definitions, and
// wild encounter tables. Use load_map(MapId) to get a complete MapData struct.
// Tile IDs correspond to sprite indices in sprites.rs. Collision values map to
// CollisionType variants. Maps: NewBarkTown (20x18), Route29 (30x14),
// CherrygroveCity (20x18), Route30 (30x18), Route31 (30x14),
// VioletCity (24x18), VioletGym (10x10), SproutTower1F/2F/3F (14x14 each),
// PlayerHouse1F (10x8), PlayerHouse2F (10x8), ElmLab (10x10), PokemonCenter (10x8),
// Route32 (20x30), UnionCave (16x16), GenericHouse (8x6), Route33 (20x12),
// AzaleaTown (20x18), AzaleaGym (10x10), IlexForest (16x20), Route34 (16x20).

// Species IDs used in encounter tables (matching data.rs constants)
const CATERPIE: u16 = 10;
const WEEDLE: u16 = 13;
const PIDGEY: u16 = 16;
const RATTATA: u16 = 19;
const BELLSPROUT: u16 = 69;
const GASTLY: u16 = 92;
const SENTRET: u16 = 161;
const HOOTHOOT: u16 = 163;
const LEDYBA: u16 = 165;
const HOPPIP: u16 = 187;
const ZUBAT: u16 = 41;
const GEODUDE: u16 = 74;
const ONIX: u16 = 95;
const SPINARAK: u16 = 167;
const MAREEP: u16 = 179;
const WOOPER: u16 = 194;
const MAGIKARP: u16 = 129;
const ODDISH: u16 = 43;
const DROWZEE: u16 = 96;
const PARAS: u16 = 46;
const CLEFAIRY: u16 = 35;
const MILTANK: u16 = 241;
const SNUBBULL: u16 = 209;
const JIGGLYPUFF: u16 = 39;
const MEOWTH: u16 = 52;
const NIDORAN_F: u16 = 29;
const NIDORAN_M: u16 = 32;
const GROWLITHE: u16 = 58;
const VULPIX: u16 = 37;
const STANTLER: u16 = 234;
const YANMA: u16 = 193;
const VENONAT: u16 = 48;
const HAUNTER: u16 = 93;
const GENGAR: u16 = 94;
const RATICATE: u16 = 20;
const MAGMAR: u16 = 126;
const KOFFING: u16 = 109;
const PIDGEOTTO: u16 = 17;
const TAUROS: u16 = 128;
const MAGNEMITE: u16 = 81;
const FARFETCHD: u16 = 83;
const NOCTOWL: u16 = 164;
const DODUO: u16 = 84;
const FLAAFFY: u16 = 180;
const PSYDUCK: u16 = 54;
const SKIPLOOM: u16 = 188;
const SLOWPOKE: u16 = 79;
const PIKACHU: u16 = 25;
const POLIWHIRL: u16 = 61;
const KRABBY: u16 = 98;
const STEELIX: u16 = 208;
const SPEAROW: u16 = 21;
const FEAROW: u16 = 22;
const POLIWAG: u16 = 60;
const MARILL: u16 = 183;
const TENTACOOL: u16 = 72;
const TENTACRUEL: u16 = 73;
const PRIMEAPE: u16 = 57;
const POLIWRATH: u16 = 62;
const MANKEY: u16 = 56;
const MACHOP: u16 = 66;
const MACHOKE: u16 = 67;
const GOLBAT: u16 = 42;
const SWINUB: u16 = 220;
const SEEL: u16 = 86;
const DEWGONG: u16 = 87;
const PILOSWINE: u16 = 221;
const GIRAFARIG: u16 = 203;
const GYARADOS: u16 = 130;
const GOLDEEN: u16 = 118;
const JYNX: u16 = 124;
const SNEASEL: u16 = 215;
const DELIBIRD: u16 = 225;
const DRATINI: u16 = 147;
const DRAGONAIR: u16 = 148;
const DRAGONITE: u16 = 149;
const KINGDRA: u16 = 230;
const HORSEA: u16 = 116;
const SEADRA: u16 = 117;
const GRAVELER: u16 = 75;
const GLIGAR: u16 = 207;
const TEDDIURSA: u16 = 216;
const SKARMORY: u16 = 227;
const PONYTA: u16 = 77;
const SANDSHREW: u16 = 27;
const SANDSLASH: u16 = 28;
const DODRIO: u16 = 85;
const ARCANINE: u16 = 59;
const QUAGSIRE: u16 = 195;
const XATU: u16 = 178;
const SLOWBRO: u16 = 80;
const EXEGGUTOR: u16 = 103;
const ARIADOS: u16 = 168;
const FORRETRESS: u16 = 205;
const MUK: u16 = 89;
const VENOMOTH: u16 = 49;
const CROBAT: u16 = 169;
const HITMONTOP: u16 = 237;
const HITMONLEE: u16 = 106;
const HITMONCHAN: u16 = 107;
const VILEPLUME: u16 = 45;
const MURKROW: u16 = 198;
const HOUNDOOM: u16 = 229;
const AERODACTYL: u16 = 142;
const CHARIZARD: u16 = 6;
const ABRA: u16 = 63;
const URSARING: u16 = 217;
const MACHAMP: u16 = 68;
const UMBREON: u16 = 197;
const SHELLDER: u16 = 90;

// ─── Tile IDs (matching sprites.rs) ─────────────────────
const GRASS: u8 = 0;
const TALL_GRASS: u8 = 1;
const PATH: u8 = 2;
const TREE_TOP: u8 = 3;
const TREE_BOTTOM: u8 = 4;
const WATER: u8 = 5;
const _WATER2: u8 = 6;
const BUILDING_WALL: u8 = 7;
const BUILDING_ROOF: u8 = 8;
const DOOR: u8 = 9;
const FENCE_H: u8 = 10;
const FLOWER: u8 = 11;
const POKECENTER_ROOF: u8 = 12;
const POKECENTER_WALL: u8 = 13;
const POKECENTER_DOOR: u8 = 14;
const LAB_WALL: u8 = 15;
const LAB_ROOF: u8 = 16;
const SIGN: u8 = 17;
const LEDGE: u8 = 18;
const FLOOR: u8 = 19;
const TABLE: u8 = 20;
const BOOKSHELF: u8 = 21;
const PC: u8 = 22;
const HEAL_MACHINE: u8 = 23;
const BLACK: u8 = 24;
const CAVE_WALL: u8 = 25;
const CAVE_FLOOR: u8 = 26;
const ICE_FLOOR: u8 = 27;
const GYM_FLOOR: u8 = 28;

// ─── Collision Constants ────────────────────────────────
const C_WALK: u8 = 0;  // Walkable
const C_SOLID: u8 = 1; // Solid
const C_TALL: u8 = 2;  // TallGrass
const C_WATER: u8 = 3; // Water
const C_WARP: u8 = 4;  // Warp
const C_LEDGE: u8 = 5; // Ledge
const C_COUNTER: u8 = 6; // Counter
const C_SIGN: u8 = 7;  // Sign
const C_ICE: u8 = 8;   // Ice (sliding puzzle)

// ─── Types ──────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MapId {
    NewBarkTown,
    Route29,
    CherrygroveCity,
    Route30,
    Route31,
    VioletCity,
    VioletGym,
    SproutTower1F,
    SproutTower2F,
    SproutTower3F,
    PlayerHouse1F,
    PlayerHouse2F,
    ElmLab,
    PokemonCenter,
    Route32,
    UnionCave,
    GenericHouse,
    Route33,
    AzaleaTown,
    AzaleaGym,
    IlexForest,
    Route34,
    GoldenrodCity,
    GoldenrodGym,
    Route35,
    NationalPark,
    Route36,
    Route37,
    EcruteakCity,
    BurnedTower,
    EcruteakGym,
    Route38,
    Route39,
    OlivineCity,
    OlivineGym,
    OlivineLighthouse,
    Route40,
    CianwoodCity,
    CianwoodGym,
    Route42,
    MahoganyTown,
    MahoganyGym,
    Route43,
    LakeOfRage,
    Route44,
    IcePath,
    BlackthornCity,
    BlackthornGym,
    Route45,
    Route46,
    Route27,
    Route26,
    VictoryRoad,
    IndigoPlateau,
    EliteFourWill,
    EliteFourKoga,
    EliteFourBruno,
    EliteFourKaren,
    ChampionLance,
    RocketHQ,
}

impl MapId {
    pub fn from_str(s: &str) -> Option<MapId> {
        match s {
            "NewBarkTown" => Some(MapId::NewBarkTown),
            "Route29" => Some(MapId::Route29),
            "CherrygroveCity" => Some(MapId::CherrygroveCity),
            "Route30" => Some(MapId::Route30),
            "Route31" => Some(MapId::Route31),
            "VioletCity" => Some(MapId::VioletCity),
            "VioletGym" => Some(MapId::VioletGym),
            "SproutTower" | "SproutTower1F" => Some(MapId::SproutTower1F),
            "SproutTower2F" => Some(MapId::SproutTower2F),
            "SproutTower3F" => Some(MapId::SproutTower3F),
            "PlayerHouse1F" => Some(MapId::PlayerHouse1F),
            "PlayerHouse2F" => Some(MapId::PlayerHouse2F),
            "ElmLab" => Some(MapId::ElmLab),
            "PokemonCenter" => Some(MapId::PokemonCenter),
            "Route32" => Some(MapId::Route32),
            "UnionCave" => Some(MapId::UnionCave),
            "GenericHouse" => Some(MapId::GenericHouse),
            "Route33" => Some(MapId::Route33),
            "AzaleaTown" => Some(MapId::AzaleaTown),
            "AzaleaGym" => Some(MapId::AzaleaGym),
            "IlexForest" => Some(MapId::IlexForest),
            "Route34" => Some(MapId::Route34),
            "GoldenrodCity" => Some(MapId::GoldenrodCity),
            "GoldenrodGym" => Some(MapId::GoldenrodGym),
            "Route35" => Some(MapId::Route35),
            "NationalPark" => Some(MapId::NationalPark),
            "Route36" => Some(MapId::Route36),
            "Route37" => Some(MapId::Route37),
            "EcruteakCity" => Some(MapId::EcruteakCity),
            "BurnedTower" => Some(MapId::BurnedTower),
            "EcruteakGym" => Some(MapId::EcruteakGym),
            "Route38" => Some(MapId::Route38),
            "Route39" => Some(MapId::Route39),
            "OlivineCity" => Some(MapId::OlivineCity),
            "OlivineGym" => Some(MapId::OlivineGym),
            "OlivineLighthouse" => Some(MapId::OlivineLighthouse),
            "Route40" => Some(MapId::Route40),
            "CianwoodCity" => Some(MapId::CianwoodCity),
            "CianwoodGym" => Some(MapId::CianwoodGym),
            "Route42" => Some(MapId::Route42),
            "MahoganyTown" => Some(MapId::MahoganyTown),
            "MahoganyGym" => Some(MapId::MahoganyGym),
            "Route43" => Some(MapId::Route43),
            "LakeOfRage" => Some(MapId::LakeOfRage),
            "Route44" => Some(MapId::Route44),
            "IcePath" => Some(MapId::IcePath),
            "BlackthornCity" => Some(MapId::BlackthornCity),
            "BlackthornGym" => Some(MapId::BlackthornGym),
            "Route45" => Some(MapId::Route45),
            "Route46" => Some(MapId::Route46),
            "Route27" => Some(MapId::Route27),
            "Route26" => Some(MapId::Route26),
            "VictoryRoad" => Some(MapId::VictoryRoad),
            "IndigoPlateau" => Some(MapId::IndigoPlateau),
            "EliteFourWill" => Some(MapId::EliteFourWill),
            "EliteFourKoga" => Some(MapId::EliteFourKoga),
            "EliteFourBruno" => Some(MapId::EliteFourBruno),
            "EliteFourKaren" => Some(MapId::EliteFourKaren),
            "ChampionLance" => Some(MapId::ChampionLance),
            "RocketHQ" => Some(MapId::RocketHQ),
            _ => None,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            MapId::NewBarkTown => "NewBarkTown",
            MapId::Route29 => "Route29",
            MapId::CherrygroveCity => "CherrygroveCity",
            MapId::Route30 => "Route30",
            MapId::Route31 => "Route31",
            MapId::VioletCity => "VioletCity",
            MapId::VioletGym => "VioletGym",
            MapId::SproutTower1F => "SproutTower1F",
            MapId::SproutTower2F => "SproutTower2F",
            MapId::SproutTower3F => "SproutTower3F",
            MapId::PlayerHouse1F => "PlayerHouse1F",
            MapId::PlayerHouse2F => "PlayerHouse2F",
            MapId::ElmLab => "ElmLab",
            MapId::PokemonCenter => "PokemonCenter",
            MapId::Route32 => "Route32",
            MapId::UnionCave => "UnionCave",
            MapId::GenericHouse => "GenericHouse",
            MapId::Route33 => "Route33",
            MapId::AzaleaTown => "AzaleaTown",
            MapId::AzaleaGym => "AzaleaGym",
            MapId::IlexForest => "IlexForest",
            MapId::Route34 => "Route34",
            MapId::GoldenrodCity => "GoldenrodCity",
            MapId::GoldenrodGym => "GoldenrodGym",
            MapId::Route35 => "Route35",
            MapId::NationalPark => "NationalPark",
            MapId::Route36 => "Route36",
            MapId::Route37 => "Route37",
            MapId::EcruteakCity => "EcruteakCity",
            MapId::BurnedTower => "BurnedTower",
            MapId::EcruteakGym => "EcruteakGym",
            MapId::Route38 => "Route38",
            MapId::Route39 => "Route39",
            MapId::OlivineCity => "OlivineCity",
            MapId::OlivineGym => "OlivineGym",
            MapId::OlivineLighthouse => "OlivineLighthouse",
            MapId::Route40 => "Route40",
            MapId::CianwoodCity => "CianwoodCity",
            MapId::CianwoodGym => "CianwoodGym",
            MapId::Route42 => "Route42",
            MapId::MahoganyTown => "MahoganyTown",
            MapId::MahoganyGym => "MahoganyGym",
            MapId::Route43 => "Route43",
            MapId::LakeOfRage => "LakeOfRage",
            MapId::Route44 => "Route44",
            MapId::IcePath => "IcePath",
            MapId::BlackthornCity => "BlackthornCity",
            MapId::BlackthornGym => "BlackthornGym",
            MapId::Route45 => "Route45",
            MapId::Route46 => "Route46",
            MapId::Route27 => "Route27",
            MapId::Route26 => "Route26",
            MapId::VictoryRoad => "VictoryRoad",
            MapId::IndigoPlateau => "IndigoPlateau",
            MapId::EliteFourWill => "EliteFourWill",
            MapId::EliteFourKoga => "EliteFourKoga",
            MapId::EliteFourBruno => "EliteFourBruno",
            MapId::EliteFourKaren => "EliteFourKaren",
            MapId::ChampionLance => "ChampionLance",
            MapId::RocketHQ => "RocketHQ",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CollisionType {
    Walkable,      // 0 - can walk on
    Solid,         // 1 - can't walk through (walls, trees, etc.)
    TallGrass,     // 2 - can walk, triggers wild encounters
    Water,         // 3 - can't walk (would need Surf)
    Warp,          // 4 - walking here triggers a map transition
    Ledge,         // 5 - can jump down south only
    Counter,       // 6 - can interact across (Pokemon Center counter)
    Sign,          // 7 - can interact with (face it and press A)
    Ice,           // 8 - walkable, triggers sliding (player slides until hitting non-ice)
}

impl CollisionType {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => CollisionType::Walkable,
            1 => CollisionType::Solid,
            2 => CollisionType::TallGrass,
            3 => CollisionType::Water,
            4 => CollisionType::Warp,
            5 => CollisionType::Ledge,
            6 => CollisionType::Counter,
            7 => CollisionType::Sign,
            8 => CollisionType::Ice,
            _ => CollisionType::Solid,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WarpData {
    pub x: u8,
    pub y: u8,
    pub dest_map: MapId,
    pub dest_x: u8,
    pub dest_y: u8,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct TrainerPokemon {
    pub species_id: u16,
    pub level: u8,
}

#[derive(Clone, Debug)]
pub struct NpcDef {
    pub x: u8,
    pub y: u8,
    pub sprite_id: u8, // 0=Elm, 1=Mom, 2=Youngster, 3=Lass, 4=Nurse, 5=OldMan
    pub facing: Direction,
    pub dialogue: &'static [&'static str],
    pub is_trainer: bool,
    pub is_mart: bool,
    pub wanders: bool,
    pub trainer_team: &'static [TrainerPokemon],
}

#[derive(Clone, Debug)]
pub struct EncounterEntry {
    pub species_id: u16,
    pub min_level: u8,
    pub max_level: u8,
    pub weight: u8, // relative probability weight
}

#[derive(Clone, Debug)]
pub struct MapData {
    pub id: MapId,
    pub name: &'static str,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<u8>,     // tile visual IDs (indexes into tile sprites)
    pub collision: Vec<u8>, // CollisionType values
    pub warps: Vec<WarpData>,
    pub npcs: Vec<NpcDef>,
    pub encounters: Vec<EncounterEntry>,
    pub night_encounters: Vec<EncounterEntry>, // night-only encounters (empty = use day table)
    pub water_encounters: Vec<EncounterEntry>, // fishing/surf encounters on water tiles
    pub music_id: u8,
}

// ─── Map Loader ─────────────────────────────────────────

pub fn load_map(id: MapId) -> MapData {
    match id {
        MapId::NewBarkTown => build_new_bark_town(),
        MapId::Route29 => build_route_29(),
        MapId::CherrygroveCity => build_cherrygrove_city(),
        MapId::Route30 => build_route_30(),
        MapId::Route31 => build_route_31(),
        MapId::VioletCity => build_violet_city(),
        MapId::VioletGym => build_violet_gym(),
        MapId::SproutTower1F => build_sprout_tower_1f(),
        MapId::SproutTower2F => build_sprout_tower_2f(),
        MapId::SproutTower3F => build_sprout_tower_3f(),
        MapId::PlayerHouse1F => build_player_house_1f(),
        MapId::PlayerHouse2F => build_player_house_2f(),
        MapId::ElmLab => build_elm_lab(),
        MapId::PokemonCenter => build_pokemon_center(),
        MapId::Route32 => build_route_32(),
        MapId::UnionCave => build_union_cave(),
        MapId::GenericHouse => build_generic_house(),
        MapId::Route33 => build_route_33(),
        MapId::AzaleaTown => build_azalea_town(),
        MapId::AzaleaGym => build_azalea_gym(),
        MapId::IlexForest => build_ilex_forest(),
        MapId::Route34 => build_route_34(),
        MapId::GoldenrodCity => build_goldenrod_city(),
        MapId::GoldenrodGym => build_goldenrod_gym(),
        MapId::Route35 => build_route_35(),
        MapId::NationalPark => build_national_park(),
        MapId::Route36 => build_route_36(),
        MapId::Route37 => build_route_37(),
        MapId::EcruteakCity => build_ecruteak_city(),
        MapId::BurnedTower => build_burned_tower(),
        MapId::EcruteakGym => build_ecruteak_gym(),
        MapId::Route38 => build_route_38(),
        MapId::Route39 => build_route_39(),
        MapId::OlivineCity => build_olivine_city(),
        MapId::OlivineGym => build_olivine_gym(),
        MapId::OlivineLighthouse => build_olivine_lighthouse(),
        MapId::Route40 => build_route_40(),
        MapId::CianwoodCity => build_cianwood_city(),
        MapId::CianwoodGym => build_cianwood_gym(),
        MapId::Route42 => build_route_42(),
        MapId::MahoganyTown => build_mahogany_town(),
        MapId::MahoganyGym => build_mahogany_gym(),
        MapId::Route43 => build_route_43(),
        MapId::LakeOfRage => build_lake_of_rage(),
        MapId::Route44 => build_route_44(),
        MapId::IcePath => build_ice_path(),
        MapId::BlackthornCity => build_blackthorn_city(),
        MapId::BlackthornGym => build_blackthorn_gym(),
        MapId::Route45 => build_route_45(),
        MapId::Route46 => build_route_46(),
        MapId::Route27 => build_route_27(),
        MapId::Route26 => build_route_26(),
        MapId::VictoryRoad => build_victory_road(),
        MapId::IndigoPlateau => build_indigo_plateau(),
        MapId::EliteFourWill => build_elite_four_will(),
        MapId::EliteFourKoga => build_elite_four_koga(),
        MapId::EliteFourBruno => build_elite_four_bruno(),
        MapId::EliteFourKaren => build_elite_four_karen(),
        MapId::ChampionLance => build_champion_lance(),
        MapId::RocketHQ => build_rocket_hq(),
    }
}

// ─── New Bark Town (20x18) ──────────────────────────────
// Layout reference (approximate):
//   Row  0: trees across the top
//   Row  1: trees across the top (bottom halves)
//   Row  2: trees left | grass | roof areas
//   Row  3: trees left | grass | wall areas
//   Row  4: trees left | grass | door areas + path
//   Rows 5-10: grass, paths, buildings
//   Rows 11-14: grass, paths, flowers
//   Row 15: path / grass leading to water
//   Rows 16-17: water (ocean) along bottom

fn build_new_bark_town() -> MapData {
    let w: usize = 20;
    let h: usize = 18;

    // T=TREE_TOP, B=TREE_BOTTOM, G=GRASS, P=PATH, W=WATER
    // R=BUILDING_ROOF, A=BUILDING_WALL, D=DOOR
    // LR=LAB_ROOF, LW=LAB_WALL, S=SIGN, F=FLOWER, FN=FENCE_H
    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: tree tops across top edge
        TREE_TOP, TREE_TOP, TREE_TOP, TREE_TOP, TREE_TOP, TREE_TOP, TREE_TOP, TREE_TOP, TREE_TOP, TREE_TOP,
        TREE_TOP, TREE_TOP, TREE_TOP, TREE_TOP, TREE_TOP, TREE_TOP, TREE_TOP, TREE_TOP, TREE_TOP, TREE_TOP,
        // Row 1: tree bottoms across top edge
        TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM,
        TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM, TREE_BOTTOM,
        // Row 2: left trees | grass | player house roof | gap | NPC house roof
        TREE_TOP, TREE_TOP, GRASS, GRASS, BUILDING_ROOF, BUILDING_ROOF, GRASS, GRASS, GRASS, GRASS,
        GRASS, BUILDING_ROOF, BUILDING_ROOF, GRASS, GRASS, LAB_ROOF, LAB_ROOF, LAB_ROOF, GRASS, TREE_TOP,
        // Row 3: left trees | grass | player house wall | gap | NPC house wall
        TREE_BOTTOM, TREE_BOTTOM, GRASS, GRASS, BUILDING_WALL, BUILDING_WALL, GRASS, GRASS, GRASS, GRASS,
        GRASS, BUILDING_WALL, BUILDING_WALL, GRASS, GRASS, LAB_WALL, LAB_WALL, LAB_WALL, GRASS, TREE_BOTTOM,
        // Row 4: left trees | grass | player house door | path | NPC house door | lab
        TREE_TOP, TREE_TOP, GRASS, GRASS, BUILDING_WALL, DOOR, GRASS, GRASS, GRASS, GRASS,
        GRASS, BUILDING_WALL, DOOR, GRASS, GRASS, LAB_WALL, LAB_WALL, DOOR, GRASS, GRASS,
        // Row 5: left tree bottom | grass path connecting
        TREE_BOTTOM, TREE_BOTTOM, GRASS, GRASS, PATH, PATH, PATH, PATH, PATH, PATH,
        PATH, PATH, PATH, PATH, PATH, PATH, PATH, PATH, GRASS, GRASS,
        // Row 6: left trees | grass | path
        TREE_TOP, TREE_TOP, GRASS, GRASS, GRASS, PATH, GRASS, GRASS, GRASS, GRASS,
        GRASS, GRASS, PATH, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS,
        // Row 7: left tree bottoms | grass | path
        TREE_BOTTOM, TREE_BOTTOM, GRASS, GRASS, GRASS, PATH, GRASS, GRASS, FLOWER, FLOWER,
        GRASS, GRASS, PATH, GRASS, GRASS, GRASS, FLOWER, FLOWER, GRASS, GRASS,
        // Row 8: trees | grass | path | fence
        TREE_TOP, TREE_TOP, GRASS, GRASS, GRASS, PATH, GRASS, GRASS, GRASS, GRASS,
        GRASS, GRASS, PATH, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS,
        // Row 9: tree bottom | grass | path leads to sign
        TREE_BOTTOM, TREE_BOTTOM, GRASS, GRASS, GRASS, PATH, GRASS, GRASS, GRASS, GRASS,
        GRASS, GRASS, PATH, GRASS, GRASS, GRASS, GRASS, GRASS, SIGN, GRASS,
        // Row 10: path going left to Route 27 and right to Route 29
        PATH, PATH, PATH, PATH, PATH, PATH, PATH, PATH, PATH, PATH,
        PATH, PATH, PATH, PATH, PATH, PATH, PATH, PATH, PATH, PATH,
        // Row 11: grass area
        GRASS, GRASS, FLOWER, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS,
        GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS,
        // Row 12: grass with fence along bottom
        GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS,
        GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS,
        // Row 13: grass near water edge
        GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS,
        GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS, GRASS,
        // Row 14: fence before water
        FENCE_H, FENCE_H, FENCE_H, FENCE_H, FENCE_H, FENCE_H, FENCE_H, FENCE_H, FENCE_H, FENCE_H,
        FENCE_H, FENCE_H, FENCE_H, FENCE_H, FENCE_H, FENCE_H, FENCE_H, FENCE_H, FENCE_H, FENCE_H,
        // Row 15: water edge
        WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER,
        WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER,
        // Row 16: water
        WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER,
        WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER,
        // Row 17: water
        WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER,
        WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER, WATER,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid tree tops
        C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID,
        C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID,
        // Row 1: solid tree bottoms
        C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID,
        C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID,
        // Row 2: trees | grass | roofs (solid)
        C_SOLID, C_SOLID, C_WALK, C_WALK, C_SOLID, C_SOLID, C_WALK, C_WALK, C_WALK, C_WALK,
        C_WALK, C_SOLID, C_SOLID, C_WALK, C_WALK, C_SOLID, C_SOLID, C_SOLID, C_WALK, C_SOLID,
        // Row 3: trees | grass | walls (solid)
        C_SOLID, C_SOLID, C_WALK, C_WALK, C_SOLID, C_SOLID, C_WALK, C_WALK, C_WALK, C_WALK,
        C_WALK, C_SOLID, C_SOLID, C_WALK, C_WALK, C_SOLID, C_SOLID, C_SOLID, C_WALK, C_SOLID,
        // Row 4: trees | grass | walls + doors (warp)
        C_SOLID, C_SOLID, C_WALK, C_WALK, C_SOLID, C_WARP, C_WALK, C_WALK, C_WALK, C_WALK,
        C_WALK, C_SOLID, C_WARP, C_WALK, C_WALK, C_SOLID, C_SOLID, C_WARP, C_WALK, C_WALK,
        // Row 5: trees | path (walkable)
        C_SOLID, C_SOLID, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        // Row 6: trees | grass | path
        C_SOLID, C_SOLID, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        // Row 7: trees | grass | path | flowers
        C_SOLID, C_SOLID, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        // Row 8: trees | grass | path
        C_SOLID, C_SOLID, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        // Row 9: trees | grass | path | sign
        C_SOLID, C_SOLID, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_SIGN, C_WALK,
        // Row 10: path left→Route27 (x=0) and right→Route29 (x=19)
        C_WARP, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WARP,
        // Row 11: grass
        C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        // Row 12: grass
        C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        // Row 13: grass
        C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
        // Row 14: fence (solid)
        C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID,
        C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID, C_SOLID,
        // Row 15: water
        C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER,
        C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER,
        // Row 16: water
        C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER,
        C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER,
        // Row 17: water
        C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER,
        C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER, C_WATER,
    ];

    debug_assert_eq!(tiles.len(), w * h, "NewBarkTown tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "NewBarkTown collision count mismatch");

    let warps = vec![
        // Player house door -> PlayerHouse1F
        WarpData { x: 5, y: 4, dest_map: MapId::PlayerHouse1F, dest_x: 4, dest_y: 6 },
        // NPC house door -> GenericHouse
        WarpData { x: 12, y: 4, dest_map: MapId::GenericHouse, dest_x: 3, dest_y: 5 },
        // Elm's Lab door -> ElmLab
        WarpData { x: 17, y: 4, dest_map: MapId::ElmLab, dest_x: 4, dest_y: 8 },
        // Right edge exit -> Route 29
        WarpData { x: 19, y: 10, dest_map: MapId::Route29, dest_x: 1, dest_y: 7 },
        // Left edge exit -> Route 27
        WarpData { x: 0, y: 10, dest_map: MapId::Route27, dest_x: 22, dest_y: 6 },
    ];

    let npcs = vec![
        // Old man near the south part of town
        NpcDef {
            x: 8, y: 8, sprite_id: 5, facing: Direction::Down,
            dialogue: &[
                "NEW BARK TOWN",
                "The town where winds of a",
                "new beginning blow.",
            ],
            is_trainer: false, is_mart: false, wanders: true, trainer_team: &[],
        },
        // Youngster near the path
        NpcDef {
            x: 10, y: 6, sprite_id: 2, facing: Direction::Left,
            dialogue: &[
                "PROF. ELM's lab is",
                "right here! He studies",
                "POKEMON evolution.",
                "You should visit him!",
            ],
            is_trainer: false, is_mart: false, wanders: true, trainer_team: &[],
        },
        // Lass near flowers
        NpcDef {
            x: 3, y: 11, sprite_id: 3, facing: Direction::Right,
            dialogue: &[
                "CHERRYGROVE CITY is",
                "just east on ROUTE 29.",
                "They have a POKEMON",
                "CENTER there!",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];

    MapData {
        id: MapId::NewBarkTown,
        name: "NEW BARK TOWN",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters: vec![],
        night_encounters: vec![], water_encounters: vec![],
        music_id: 1,
    }
}

// ─── Route 29 (30x14) ──────────────────────────────────
// Connects New Bark Town (left) to Cherrygrove City (right).
// Trees along top and bottom, tall grass patches, winding path, ledges.

fn build_route_29() -> MapData {
    let w: usize = 30;
    let h: usize = 14;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: tree tops all across
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: tree bottoms
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: trees left | grass with some tall grass | trees right
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,
        GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,
        TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 3: trees | grass corridor with tall grass patches
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,
        GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,
        TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4: open area with path fragments
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,PATH,
        PATH,PATH,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,
        GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,
        // Row 5: grass + path + ledge
        GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,PATH,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 6: ledges below some areas
        GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,LEDGE,LEDGE,LEDGE,
        GRASS,PATH,GRASS,GRASS,LEDGE,LEDGE,PATH,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 7: main east-west path (entry from New Bark left, exit to Cherrygrove right)
        PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        // Row 8: grass below path + sign
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,SIGN,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 9: more grass + tall grass patches
        GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 10: grass + tall grass
        GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 11: grass
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 12: tree tops along bottom
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 13: tree bottoms along bottom
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: trees solid | grass walk | tall grass | tree clusters
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,
        C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 3:
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,
        C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 4: path
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5: grass + path
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 6: ledges
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_LEDGE,C_LEDGE,C_LEDGE,
        C_WALK,C_WALK,C_WALK,C_WALK,C_LEDGE,C_LEDGE,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 7: main path (left warp, right warp)
        C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,
        // Row 8: grass + sign
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SIGN,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 9: tall grass patches
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 10: tall grass patches
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 11: grass
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 12: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 13: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "Route29 tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "Route29 collision count mismatch");

    let warps = vec![
        // Left edge -> New Bark Town (right exit)
        WarpData { x: 0, y: 7, dest_map: MapId::NewBarkTown, dest_x: 18, dest_y: 10 },
        // Right edge -> Cherrygrove City (left entry)
        WarpData { x: 29, y: 7, dest_map: MapId::CherrygroveCity, dest_x: 1, dest_y: 10 },
    ];

    let npcs = vec![
        // Guide Gent NPC
        NpcDef {
            x: 5, y: 8, sprite_id: 5, facing: Direction::Up,
            dialogue: &[
                "CHERRYGROVE CITY is",
                "just ahead! They have",
                "a POKEMON CENTER where",
                "you can heal up.",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // Youngster NPC
        NpcDef {
            x: 15, y: 5, sprite_id: 2, facing: Direction::Down,
            dialogue: &[
                "POKEMON hide in the grass.",
                "You need your own POKEMON",
                "to go through safely!",
            ],
            is_trainer: false, is_mart: false, wanders: true, trainer_team: &[],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: PIDGEY, min_level: 2, max_level: 4, weight: 30 },
        EncounterEntry { species_id: RATTATA, min_level: 2, max_level: 4, weight: 30 },
        EncounterEntry { species_id: SENTRET, min_level: 2, max_level: 4, weight: 25 },
        EncounterEntry { species_id: HOOTHOOT, min_level: 2, max_level: 3, weight: 15 },
    ];

    MapData {
        id: MapId::Route29,
        name: "ROUTE 29",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![
            EncounterEntry { species_id: HOOTHOOT, min_level: 2, max_level: 4, weight: 50 },
            EncounterEntry { species_id: RATTATA, min_level: 2, max_level: 4, weight: 35 },
            EncounterEntry { species_id: SPINARAK, min_level: 2, max_level: 3, weight: 15 },
        ],
        water_encounters: vec![],
        music_id: 2,
    }
}

// ─── Cherrygrove City (20x18) ──────────────────────────
// Second town. Has a Pokemon Center, a couple houses, water at bottom.

fn build_cherrygrove_city() -> MapData {
    let w: usize = 20;
    let h: usize = 18;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: tree tops with north exit gap at columns 9-10
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,PATH,
        PATH,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: tree bottoms with north exit gap at columns 9-10
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,PATH,
        PATH,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: trees left | grass | pokecenter roof | path north | house roof
        TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,POKECENTER_ROOF,POKECENTER_ROOF,POKECENTER_ROOF,PATH,
        PATH,GRASS,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,GRASS,TREE_TOP,
        // Row 3: trees | pokecenter wall | path north | house wall
        TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,POKECENTER_WALL,POKECENTER_WALL,POKECENTER_WALL,PATH,
        PATH,GRASS,GRASS,GRASS,BUILDING_WALL,BUILDING_WALL,GRASS,GRASS,GRASS,TREE_BOTTOM,
        // Row 4: pokecenter door | house door
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,POKECENTER_WALL,POKECENTER_DOOR,POKECENTER_WALL,GRASS,
        GRASS,GRASS,GRASS,GRASS,BUILDING_WALL,DOOR,GRASS,GRASS,GRASS,GRASS,
        // Row 5: path in front of buildings
        GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,
        // Row 6: grass below path | another house roof
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,GRASS,
        // Row 7: grass | house wall + sign
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,FLOWER,FLOWER,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,BUILDING_WALL,BUILDING_WALL,GRASS,GRASS,GRASS,
        // Row 8: more grass | house door
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,BUILDING_WALL,DOOR,GRASS,GRASS,GRASS,
        // Row 9: sign near pokecenter
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,SIGN,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,GRASS,GRASS,
        // Row 10: east-west path (entry from Route 29 on left)
        PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 11: grass
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 12: grass with flowers
        GRASS,GRASS,FLOWER,FLOWER,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,FLOWER,FLOWER,GRASS,GRASS,GRASS,GRASS,
        // Row 13: grass
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 14: fence before water
        FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,
        FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,
        // Row 15: water
        WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
        WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
        // Row 16: water
        WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
        WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
        // Row 17: water
        WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
        WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid trees with north exit warp at columns 9-10
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,
        C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: solid trees with path at columns 9-10
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,
        C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: roofs solid, path north at columns 9-10
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 3: walls solid, path north at columns 9-10
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 4: doors are warps
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_SOLID,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5: path walkable
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 6: grass | house roof solid
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,
        // Row 7: grass | house wall solid
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,
        // Row 8: house door warp
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,
        // Row 9: sign
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SIGN,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 10: entry/exit path (left = warp from Route 29)
        C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 11: grass walkable
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 12: grass walkable
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 13: grass walkable
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 14: fence solid
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 15: water
        C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
        // Row 16: water
        C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
        // Row 17: water
        C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
    ];

    debug_assert_eq!(tiles.len(), w * h, "CherrygroveCity tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "CherrygroveCity collision count mismatch");

    let warps = vec![
        // Left edge entry from Route 29
        WarpData { x: 0, y: 10, dest_map: MapId::Route29, dest_x: 28, dest_y: 7 },
        // Pokemon Center door
        WarpData { x: 7, y: 4, dest_map: MapId::PokemonCenter, dest_x: 4, dest_y: 6 },
        // NPC house door (upper right) -> GenericHouse
        WarpData { x: 15, y: 4, dest_map: MapId::GenericHouse, dest_x: 3, dest_y: 5 },
        // NPC house door (lower right) -> GenericHouse
        WarpData { x: 16, y: 8, dest_map: MapId::GenericHouse, dest_x: 3, dest_y: 5 },
        // North exit to Route 30 (column 9)
        WarpData { x: 9, y: 0, dest_map: MapId::Route30, dest_x: 14, dest_y: 16 },
        // North exit to Route 30 (column 10)
        WarpData { x: 10, y: 0, dest_map: MapId::Route30, dest_x: 15, dest_y: 16 },
    ];

    let npcs = vec![
        // Guide Gent (old man near pokecenter sign)
        NpcDef {
            x: 5, y: 9, sprite_id: 5, facing: Direction::Right,
            dialogue: &[
                "The sea breeze is",
                "wonderful here!",
                "MR. POKEMON lives",
                "north on ROUTE 30.",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // Nurse Joy standing outside pokecenter
        NpcDef {
            x: 8, y: 5, sprite_id: 4, facing: Direction::Down,
            dialogue: &[
                "Welcome to our POKEMON",
                "CENTER! We can heal your",
                "POKEMON to full health.",
                "Shall I heal them?",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // Youngster
        NpcDef {
            x: 12, y: 11, sprite_id: 2, facing: Direction::Up,
            dialogue: &[
                "ROUTE 30 leads north",
                "to MR. POKEMON's house.",
                "He collects rare items!",
            ],
            is_trainer: false, is_mart: false, wanders: true, trainer_team: &[],
        },
        // Mart clerk
        NpcDef {
            x: 14, y: 5, sprite_id: 5, facing: Direction::Down,
            dialogue: &[
                "Welcome to the POKE MART!",
                "What would you like?",
            ],
            is_trainer: false, is_mart: true, wanders: false, trainer_team: &[],
        },
    ];

    MapData {
        id: MapId::CherrygroveCity,
        name: "CHERRYGROVE CITY",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters: vec![],
        night_encounters: vec![], water_encounters: vec![],
        music_id: 3,
    }
}

// ─── Route 30 (30x18) ──────────────────────────────────
// Connects Cherrygrove City (south) northward. Trees along edges, tall grass
// patches, a small pond, ledges, trainers, and a sign at the entrance.

fn build_route_30() -> MapData {
    let w: usize = 30;
    let h: usize = 18;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: tree tops all across
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: tree bottoms
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: trees | grass | tall grass left patch | path | grass | pond
        TREE_TOP,TREE_TOP,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,WATER,WATER,GRASS,TREE_TOP,TREE_TOP,
        // Row 3: trees | grass | tall grass left patch | path | grass | pond
        TREE_BOTTOM,TREE_BOTTOM,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        PATH,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,WATER,WATER,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4: trees | grass | path going north-south | grass | pond edge
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        PATH,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,WATER,WATER,WATER,TREE_TOP,TREE_TOP,
        // Row 5: trees | grass | ledge | path | tall grass right | grass
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,LEDGE,LEDGE,LEDGE,GRASS,GRASS,
        PATH,GRASS,GRASS,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 6: trees | grass | path widens | tall grass right
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,PATH,PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 7: trees | tall grass patch | path | grass | flower area
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        FLOWER,FLOWER,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 8: trees | grass | ledge | path | sign
        TREE_TOP,TREE_TOP,GRASS,GRASS,LEDGE,LEDGE,LEDGE,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 9: trees | tall grass | path | grass
        TREE_BOTTOM,TREE_BOTTOM,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 10: trees | tall grass | path east-west to east exit → Route 31
        TREE_TOP,TREE_TOP,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,PATH,PATH,
        // Row 11: trees | grass | path goes south | east exit continues
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,PATH,PATH,PATH,
        PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,PATH,PATH,
        // Row 12: trees | grass | path | grass | flowers
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,
        FLOWER,FLOWER,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 13: trees | grass | path curves east | grass
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 14: trees | grass with sign | path going south
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,PATH,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 15: trees | grass | sign | path | grass
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,SIGN,PATH,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 16: trees | grass | path widens to south exit
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 17: bottom row - trees with south exit gap (connects to CherrygroveCity north)
        // 2-tile-wide exit at x=14,15 to match Cherrygrove's 2-tile north border
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,PATH,PATH,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: trees | grass | tall grass | path | grass | pond
        C_SOLID,C_SOLID,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WALK,C_SOLID,C_SOLID,
        // Row 3: trees | grass | tall grass | path | grass | pond
        C_SOLID,C_SOLID,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WALK,C_SOLID,C_SOLID,
        // Row 4: trees | grass | path | grass | pond
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,C_SOLID,C_SOLID,
        // Row 5: trees | grass | ledge | path | tall grass | grass
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_LEDGE,C_LEDGE,C_LEDGE,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 6: trees | grass | path | tall grass
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 7: trees | grass | path | flowers
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 8: trees | grass | ledge | path
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_LEDGE,C_LEDGE,C_LEDGE,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 9: trees | tall grass | path | tall grass
        C_SOLID,C_SOLID,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 10: trees | tall grass | path east → Route 31 exit
        C_SOLID,C_SOLID,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,
        // Row 11: trees | grass | path east → Route 31 exit
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,
        // Row 12: trees | grass | path | flowers
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 13: trees | grass | path east-west
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 14: trees | grass | path south
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 15: trees | sign | path south
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_SIGN,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 16: trees | grass | path widens south
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 17: trees with south exit warp to CherrygroveCity (2-tile-wide at x=14,15)
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "Route30 tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "Route30 collision count mismatch");

    let warps = vec![
        // South exit -> CherrygroveCity north (2-tile-wide at x=14,15)
        WarpData { x: 14, y: 17, dest_map: MapId::CherrygroveCity, dest_x: 9, dest_y: 1 },
        WarpData { x: 15, y: 17, dest_map: MapId::CherrygroveCity, dest_x: 10, dest_y: 1 },
        // East exit -> Route 31 (row 10, at midpoint east-west path)
        WarpData { x: 29, y: 10, dest_map: MapId::Route31, dest_x: 1, dest_y: 7 },
        // East exit -> Route 31 (row 11)
        WarpData { x: 29, y: 11, dest_map: MapId::Route31, dest_x: 1, dest_y: 8 },
    ];

    let npcs = vec![
        // Youngster trainer near the first tall grass patch
        NpcDef {
            x: 8, y: 3, sprite_id: 2, facing: Direction::Left,
            dialogue: &[
                "I'm training my POKEMON",
                "on ROUTE 30! Let's battle!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: RATTATA, level: 4 }],
        },
        // Lass trainer on the east side near tall grass
        NpcDef {
            x: 19, y: 6, sprite_id: 3, facing: Direction::Left,
            dialogue: &[
                "My POKEMON are getting",
                "stronger every day!",
                "Want to see?",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: PIDGEY, level: 5 }],
        },
        // Youngster trainer in the middle area
        NpcDef {
            x: 16, y: 11, sprite_id: 2, facing: Direction::Up,
            dialogue: &[
                "I've been waiting for a",
                "trainer to come by!",
                "Let's have a battle!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: RATTATA, level: 4 },
                TrainerPokemon { species_id: PIDGEY, level: 4 },
            ],
        },
        // Hint NPC — old man near MR. POKEMON's area
        NpcDef {
            x: 24, y: 4, sprite_id: 5, facing: Direction::Down,
            dialogue: &[
                "MR. POKEMON lives north",
                "of here. He's always",
                "finding rare things!",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: CATERPIE, min_level: 3, max_level: 5, weight: 15 },
        EncounterEntry { species_id: WEEDLE, min_level: 3, max_level: 5, weight: 15 },
        EncounterEntry { species_id: PIDGEY, min_level: 3, max_level: 6, weight: 25 },
        EncounterEntry { species_id: RATTATA, min_level: 3, max_level: 5, weight: 15 },
        EncounterEntry { species_id: BELLSPROUT, min_level: 4, max_level: 6, weight: 15 },
        EncounterEntry { species_id: SPINARAK, min_level: 3, max_level: 5, weight: 15 },
    ];

    MapData {
        id: MapId::Route30,
        name: "ROUTE 30",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![
            EncounterEntry { species_id: HOOTHOOT, min_level: 3, max_level: 6, weight: 40 },
            EncounterEntry { species_id: RATTATA, min_level: 3, max_level: 5, weight: 25 },
            EncounterEntry { species_id: ZUBAT, min_level: 3, max_level: 5, weight: 20 },
            EncounterEntry { species_id: SPINARAK, min_level: 3, max_level: 5, weight: 15 },
        ],
        water_encounters: vec![],
        music_id: 2,
    }
}

// ─── Route 31 (30x14) ───────────────────────────────────
// Connects Route30/CherrygroveCity (west) to VioletCity (east).
// Trees along top and bottom, tall grass patches, winding path.

fn build_route_31() -> MapData {
    let w: usize = 30;
    let h: usize = 14;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: tree tops all across
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: tree bottoms
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: trees left | grass | tall grass patches | trees right
        TREE_TOP,TREE_TOP,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 3: trees | grass corridor with tall grass
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4: grass with path fragments
        GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,PATH,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,GRASS,GRASS,
        // Row 5: grass + path + sign
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,PATH,GRASS,GRASS,GRASS,SIGN,PATH,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 6: grass + path
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,
        GRASS,PATH,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 7: main east-west path (entry from Route30 left, exit to VioletCity right)
        PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        // Row 8: main path secondary row
        PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,
        // Row 9: grass with tall grass patches
        GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 10: grass + tall grass
        GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 11: grass
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 12: tree tops along bottom
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 13: tree bottoms along bottom
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: trees | grass | tall grass | tree clusters
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 3: trees | grass | tall grass
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 4: path
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5: grass + path + sign
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SIGN,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 6: grass + path + tall grass
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 7: main path (left warp, right warp)
        C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,
        // Row 8: grass + path + tall grass
        C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,
        // Row 9: tall grass patches
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_TALL,C_TALL,C_WALK,
        C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 10: tall grass patches
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 11: grass
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 12: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 13: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "Route31 tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "Route31 collision count mismatch");

    let warps = vec![
        // Left edge -> Route 30 (east exit)
        WarpData { x: 0, y: 7, dest_map: MapId::Route30, dest_x: 28, dest_y: 10 },
        WarpData { x: 0, y: 8, dest_map: MapId::Route30, dest_x: 28, dest_y: 11 },
        // Right edge -> Violet City (west entry)
        WarpData { x: 29, y: 7, dest_map: MapId::VioletCity, dest_x: 1, dest_y: 10 },
        WarpData { x: 29, y: 8, dest_map: MapId::VioletCity, dest_x: 1, dest_y: 11 },
    ];

    let npcs = vec![
        // Bug Catcher trainer
        NpcDef {
            x: 8, y: 6, sprite_id: 2, facing: Direction::Down,
            dialogue: &[
                "I'm a BUG CATCHER!",
                "My CATERPIE is the best!",
                "Let's battle!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: CATERPIE, level: 7 },
                TrainerPokemon { species_id: WEEDLE, level: 7 },
            ],
        },
        // Youngster trainer
        NpcDef {
            x: 20, y: 5, sprite_id: 2, facing: Direction::Left,
            dialogue: &[
                "Hey! Are you heading to",
                "VIOLET CITY too?",
                "Let me test your skills!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: RATTATA, level: 6 }],
        },
        // Hint NPC about Violet City gym
        NpcDef {
            x: 15, y: 8, sprite_id: 5, facing: Direction::Up,
            dialogue: &[
                "VIOLET CITY has a GYM!",
                "The leader uses BIRD",
                "POKEMON. Train up!",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: BELLSPROUT, min_level: 3, max_level: 5, weight: 25 },
        EncounterEntry { species_id: CATERPIE, min_level: 3, max_level: 5, weight: 15 },
        EncounterEntry { species_id: WEEDLE, min_level: 3, max_level: 4, weight: 15 },
        EncounterEntry { species_id: PIDGEY, min_level: 5, max_level: 7, weight: 20 },
        EncounterEntry { species_id: LEDYBA, min_level: 4, max_level: 5, weight: 10 },
        EncounterEntry { species_id: SPINARAK, min_level: 4, max_level: 5, weight: 15 },
    ];

    MapData {
        id: MapId::Route31,
        name: "ROUTE 31",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![
            EncounterEntry { species_id: HOOTHOOT, min_level: 5, max_level: 7, weight: 35 },
            EncounterEntry { species_id: GASTLY, min_level: 5, max_level: 7, weight: 20 },
            EncounterEntry { species_id: RATTATA, min_level: 4, max_level: 6, weight: 20 },
            EncounterEntry { species_id: ZUBAT, min_level: 4, max_level: 6, weight: 15 },
            EncounterEntry { species_id: SPINARAK, min_level: 4, max_level: 5, weight: 10 },
        ],
        water_encounters: vec![],
        music_id: 2,
    }
}

// ─── Violet City (24x18) ────────────────────────────────
// City with Pokemon Center, Poke Mart, Violet Gym, and Sprout Tower entrance.
// "The city of nostalgic scents."

fn build_violet_city() -> MapData {
    let w: usize = 24;
    let h: usize = 18;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: tree tops across with Sprout Tower area
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: tree bottoms
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: grass | Sprout Tower building roof area (right side)
        TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,
        GRASS,GRASS,GRASS,TREE_TOP,
        // Row 3: grass | Sprout Tower wall
        TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,BUILDING_WALL,BUILDING_WALL,BUILDING_WALL,
        GRASS,GRASS,GRASS,TREE_BOTTOM,
        // Row 4: grass | Sprout Tower door + Gym roof
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,BUILDING_ROOF,
        BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,GRASS,GRASS,GRASS,BUILDING_WALL,DOOR,BUILDING_WALL,
        GRASS,GRASS,GRASS,GRASS,
        // Row 5: grass | Gym wall + path to tower
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,BUILDING_WALL,
        BUILDING_WALL,BUILDING_WALL,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,
        GRASS,GRASS,GRASS,GRASS,
        // Row 6: grass | Gym door + path
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,BUILDING_WALL,
        DOOR,BUILDING_WALL,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 7: grass + path in front of gym
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,
        PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 8: grass + path leading south
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 9: PokemonCenter roof | Mart roof | path
        GRASS,GRASS,GRASS,GRASS,POKECENTER_ROOF,POKECENTER_ROOF,POKECENTER_ROOF,PATH,GRASS,GRASS,
        GRASS,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,GRASS,GRASS,
        SIGN,GRASS,GRASS,GRASS,
        // Row 10: PokemonCenter wall | path | Mart wall
        PATH,PATH,PATH,PATH,POKECENTER_WALL,POKECENTER_WALL,POKECENTER_WALL,PATH,GRASS,GRASS,
        GRASS,GRASS,GRASS,BUILDING_WALL,BUILDING_WALL,BUILDING_WALL,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 11: PokemonCenter door | path | Mart door
        PATH,PATH,GRASS,GRASS,POKECENTER_WALL,POKECENTER_DOOR,POKECENTER_WALL,PATH,PATH,PATH,
        PATH,PATH,PATH,BUILDING_WALL,BUILDING_WALL,DOOR,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 12: path + grass
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 13: grass + flowers
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,FLOWER,FLOWER,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 14: grass
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 15: fence before water (gap at x=11,12 for south exit to Route 32)
        FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,
        FENCE_H,PATH,PATH,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,FENCE_H,
        FENCE_H,FENCE_H,FENCE_H,FENCE_H,
        // Row 16: water (path gap at x=11,12 for south exit)
        WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
        WATER,PATH,PATH,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
        WATER,WATER,WATER,WATER,
        // Row 17: water (warp tiles at x=11,12 for south exit)
        WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
        WATER,PATH,PATH,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
        WATER,WATER,WATER,WATER,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: trees | grass | Sprout Tower roof solid
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 3: Sprout Tower wall solid
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 4: Gym roof solid | Sprout Tower door warp
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_SOLID,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5: Gym wall solid | path
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 6: Gym door warp | path
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_WARP,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 7: path walkable
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 8: path
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 9: PokemonCenter roof solid | Mart roof solid | sign
        C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,
        C_SIGN,C_WALK,C_WALK,C_WALK,
        // Row 10: PokemonCenter wall solid | Mart wall solid (west warp entry)
        C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 11: PokemonCenter door warp | Mart door warp (west warp entry row 2)
        C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_SOLID,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 12: path
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 13: grass + flowers
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 14: grass
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 15: fence solid (gap at x=11,12 for south exit)
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 16: water (walkable path at x=11,12)
        C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WATER,
        // Row 17: water (warp at x=11,12 to Route 32)
        C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WARP,C_WARP,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WATER,
    ];

    debug_assert_eq!(tiles.len(), w * h, "VioletCity tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "VioletCity collision count mismatch");

    let warps = vec![
        // West edge entry from Route 31 (row 10)
        WarpData { x: 0, y: 10, dest_map: MapId::Route31, dest_x: 28, dest_y: 7 },
        // West edge entry from Route 31 (row 11)
        WarpData { x: 0, y: 11, dest_map: MapId::Route31, dest_x: 28, dest_y: 8 },
        // Pokemon Center door -> PokemonCenter interior
        WarpData { x: 5, y: 11, dest_map: MapId::PokemonCenter, dest_x: 4, dest_y: 6 },
        // Poke Mart door -> GenericHouse (used as mart interior)
        WarpData { x: 15, y: 11, dest_map: MapId::GenericHouse, dest_x: 3, dest_y: 5 },
        // Violet Gym door -> VioletGym interior
        WarpData { x: 10, y: 6, dest_map: MapId::VioletGym, dest_x: 5, dest_y: 8 },
        // Sprout Tower door -> SproutTower1F interior
        WarpData { x: 18, y: 4, dest_map: MapId::SproutTower1F, dest_x: 7, dest_y: 12 },
        // South exit to Route 32 (x=11,12 at bottom edge)
        WarpData { x: 11, y: 17, dest_map: MapId::Route32, dest_x: 9, dest_y: 1 },
        WarpData { x: 12, y: 17, dest_map: MapId::Route32, dest_x: 10, dest_y: 1 },
    ];

    let npcs = vec![
        // Nurse standing near Pokemon Center
        NpcDef {
            x: 6, y: 12, sprite_id: 4, facing: Direction::Up,
            dialogue: &[
                "Welcome to VIOLET CITY's",
                "POKEMON CENTER!",
                "We can heal your POKEMON.",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // Old man near the sign
        NpcDef {
            x: 19, y: 8, sprite_id: 5, facing: Direction::Down,
            dialogue: &[
                "SPROUT TOWER is a",
                "training ground for",
                "monks. FALKNER is our",
                "GYM LEADER.",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // Youngster near gym
        NpcDef {
            x: 7, y: 7, sprite_id: 2, facing: Direction::Right,
            dialogue: &[
                "FALKNER's PIDGEOTTO is",
                "tough! Train at SPROUT",
                "TOWER first. The monks",
                "will test your skills!",
            ],
            is_trainer: false, is_mart: false, wanders: true, trainer_team: &[],
        },
        // Lass near flowers
        NpcDef {
            x: 10, y: 13, sprite_id: 3, facing: Direction::Up,
            dialogue: &[
                "I love the scent of",
                "flowers in this city!",
                "It's so nostalgic...",
            ],
            is_trainer: false, is_mart: false, wanders: true, trainer_team: &[],
        },
        // Mart clerk
        NpcDef {
            x: 15, y: 12, sprite_id: 5, facing: Direction::Left,
            dialogue: &[
                "Welcome! We have a great",
                "selection of items today!",
            ],
            is_trainer: false, is_mart: true, wanders: false, trainer_team: &[],
        },
    ];

    MapData {
        id: MapId::VioletCity,
        name: "VIOLET CITY",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters: vec![],
        night_encounters: vec![], water_encounters: vec![],
        music_id: 7,
    }
}

// ─── Violet Gym (10x10) ─────────────────────────────────
// Indoor gym with Gym Leader Falkner and one trainee.

fn build_violet_gym() -> MapData {
    let w: usize = 10;
    let h: usize = 10;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: top wall
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: back wall
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 2: Falkner stands here (center)
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 3: floor
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 4: floor
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 5: trainee stands here
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 6: floor
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 7: floor
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 8: floor near entrance
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 9: bottom wall with door
        BLACK,BLACK,BLACK,BLACK,BLACK,DOOR,BLACK,BLACK,BLACK,BLACK,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid wall
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 2: floor (Falkner)
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 3: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 4: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 5: floor (trainee)
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 7: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 8: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 9: wall + door warp
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "VioletGym tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "VioletGym collision count mismatch");

    let warps = vec![
        // Door exits to Violet City (in front of gym)
        WarpData { x: 5, y: 9, dest_map: MapId::VioletCity, dest_x: 10, dest_y: 7 },
    ];

    let npcs = vec![
        // Gym Leader Falkner - Gen 2 team: Pidgey L7, Pidgeotto L9
        NpcDef {
            x: 5, y: 2, sprite_id: 2, facing: Direction::Down,
            dialogue: &[
                "I'm FALKNER, the VIOLET",
                "CITY GYM LEADER!",
                "People say you can clip",
                "flying POKEMON's wings",
                "with a jolt of electricity.",
                "I won't allow such a Pokemon",
                "battle to Pokemon!",
                "Let me show you the real",
                "power of the magnificent",
                "bird POKEMON!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: PIDGEY, level: 7 },
                TrainerPokemon { species_id: PIDGEY, level: 9 }, // Pidgeotto L9 in real game, but we use Pidgey
            ],
        },
        // Trainee
        NpcDef {
            x: 3, y: 5, sprite_id: 2, facing: Direction::Right,
            dialogue: &[
                "FALKNER is amazing!",
                "His bird POKEMON are",
                "so graceful in battle!",
                "But first, you'll have",
                "to get past me!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: PIDGEY, level: 7 }],
        },
    ];

    MapData {
        id: MapId::VioletGym,
        name: "VIOLET GYM",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters: vec![],
        night_encounters: vec![], water_encounters: vec![],
        music_id: 8,
    }
}

// ─── Sprout Tower (14x14) ───────────────────────────────
// Indoor tower with monks/NPCs and wild encounters (Rattata, Gastly).

// ─── Sprout Tower 1F (14x14) ─────────────────────────────
// Entry floor: NPCs (Granny, Teacher, 2 non-trainer Sages), Sage Chow (3x Bellsprout Lv3),
// Parlyz Heal item, stairs up to 2F, entrance from Violet City.
fn build_sprout_tower_1f() -> MapData {
    let w: usize = 14;
    let h: usize = 14;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: top wall
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: back wall with bookshelves + stairs up (right side)
        BLACK,FLOOR,BOOKSHELF,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BOOKSHELF,BOOKSHELF,BLACK,
        // Row 2: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 3: floor with center pillar (table tiles)
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,TABLE,TABLE,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 4: floor with center pillar
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,TABLE,TABLE,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 5: floor — Granny NPC area
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 6: floor — Teacher NPC area
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 7: bookshelves on sides (pillar decorations)
        BLACK,BOOKSHELF,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BOOKSHELF,BLACK,
        // Row 8: floor — Sage NPCs area
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 9: floor — Sage Chow trainer area
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 10: center pillar lower section
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,TABLE,TABLE,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 11: floor — item (Parlyz Heal) area
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 12: floor near entrance
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 13: bottom wall with entrance door
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,DOOR,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid wall
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: bookshelves solid, stairs warp at (12,1)
        C_SOLID,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_SOLID,
        // Row 2: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 3: center pillar solid
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 4: center pillar solid
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 5: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 7: bookshelves solid on sides
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 8: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 9: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 10: center pillar solid
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 11: floor (item location)
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 12: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 13: wall + door warp
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "SproutTower1F tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "SproutTower1F collision count mismatch");

    let warps = vec![
        // Door exits to Violet City (in front of Sprout Tower)
        WarpData { x: 7, y: 13, dest_map: MapId::VioletCity, dest_x: 18, dest_y: 5 },
        // Stairs up to 2F (top-right corner) — arrives next to stairs-down on 2F
        WarpData { x: 12, y: 1, dest_map: MapId::SproutTower2F, dest_x: 2, dest_y: 12 },
    ];

    let npcs = vec![
        // NPC 0: Granny — non-trainer, flavor dialogue
        NpcDef {
            x: 3, y: 5, sprite_id: 5, facing: Direction::Right,
            dialogue: &[
                "The pillar in the center",
                "is a 100-foot BELLSPROUT.",
                "It sways to protect the",
                "tower from earthquakes.",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // NPC 1: Teacher — non-trainer, explains tower purpose
        NpcDef {
            x: 10, y: 6, sprite_id: 3, facing: Direction::Left,
            dialogue: &[
                "Trainers come here to",
                "test their skills.",
                "The SAGES are strict",
                "but fair!",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // NPC 2: Sage (non-trainer) — pillar lore
        NpcDef {
            x: 4, y: 8, sprite_id: 5, facing: Direction::Down,
            dialogue: &[
                "BELLSPROUT is revered in",
                "this tower.",
                "We meditate on the",
                "swaying of the pillar.",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // NPC 3: Sage (non-trainer) — direction hint
        NpcDef {
            x: 9, y: 3, sprite_id: 5, facing: Direction::Down,
            dialogue: &[
                "The ELDER awaits on the",
                "top floor.",
                "Prove yourself worthy!",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // NPC 4: Sage Chow — trainer (3x Bellsprout Lv3)
        NpcDef {
            x: 3, y: 9, sprite_id: 5, facing: Direction::Right,
            dialogue: &[
                "SAGE CHOW: I will test",
                "your resolve!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: BELLSPROUT, level: 3 },
                TrainerPokemon { species_id: BELLSPROUT, level: 3 },
                TrainerPokemon { species_id: BELLSPROUT, level: 3 },
            ],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: RATTATA, min_level: 3, max_level: 5, weight: 60 },
        EncounterEntry { species_id: GASTLY, min_level: 3, max_level: 5, weight: 40 },
    ];

    MapData {
        id: MapId::SproutTower1F,
        name: "SPROUT TOWER 1F",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![], water_encounters: vec![],
        music_id: 9,
    }
}

// ─── Sprout Tower 2F (14x14) ─────────────────────────────
// Middle floor: Sage Nico (3x Bellsprout Lv3), Sage Edmond (3x Bellsprout Lv3),
// X Accuracy item, stairs down to 1F + stairs up to 3F.
fn build_sprout_tower_2f() -> MapData {
    let w: usize = 14;
    let h: usize = 14;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: top wall
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: back wall with stairs up (right side)
        BLACK,FLOOR,BOOKSHELF,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BOOKSHELF,BOOKSHELF,BLACK,
        // Row 2: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 3: floor with center pillar
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,TABLE,TABLE,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 4: floor with center pillar
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,TABLE,TABLE,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 5: floor — Sage Nico area
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 6: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 7: bookshelves on sides
        BLACK,BOOKSHELF,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BOOKSHELF,BLACK,
        // Row 8: floor — Sage Edmond area + X Accuracy item
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 9: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 10: center pillar lower
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,TABLE,TABLE,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 11: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 12: floor — stairs down (left side)
        BLACK,BOOKSHELF,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 13: bottom wall
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid wall
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: stairs up warp at (12,1)
        C_SOLID,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_SOLID,
        // Row 2: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 3: center pillar solid
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 4: center pillar solid
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 5: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 7: bookshelves solid on sides
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 8: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 9: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 10: center pillar solid
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 11: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 12: stairs down warp at (1,12)
        C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 13: solid wall
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "SproutTower2F tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "SproutTower2F collision count mismatch");

    let warps = vec![
        // Stairs down to 1F (left side)
        WarpData { x: 1, y: 12, dest_map: MapId::SproutTower1F, dest_x: 12, dest_y: 2 },
        // Stairs up to 3F (right side) — arrives next to stairs-down on 3F
        WarpData { x: 12, y: 1, dest_map: MapId::SproutTower3F, dest_x: 2, dest_y: 12 },
    ];

    let npcs = vec![
        // NPC 0: Sage Nico — trainer (3x Bellsprout Lv3)
        NpcDef {
            x: 3, y: 5, sprite_id: 5, facing: Direction::Right,
            dialogue: &[
                "SAGE NICO: Do you know",
                "why BELLSPROUT is",
                "revered? Let me show you!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: BELLSPROUT, level: 3 },
                TrainerPokemon { species_id: BELLSPROUT, level: 3 },
                TrainerPokemon { species_id: BELLSPROUT, level: 3 },
            ],
        },
        // NPC 1: Sage Edmond — trainer (3x Bellsprout Lv3)
        NpcDef {
            x: 10, y: 8, sprite_id: 5, facing: Direction::Left,
            dialogue: &[
                "SAGE EDMOND: The way of",
                "the tower is patience!",
                "I'll test yours!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: BELLSPROUT, level: 3 },
                TrainerPokemon { species_id: BELLSPROUT, level: 3 },
                TrainerPokemon { species_id: BELLSPROUT, level: 3 },
            ],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: RATTATA, min_level: 3, max_level: 5, weight: 60 },
        EncounterEntry { species_id: GASTLY, min_level: 3, max_level: 5, weight: 40 },
    ];

    MapData {
        id: MapId::SproutTower2F,
        name: "SPROUT TOWER 2F",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![], water_encounters: vec![],
        music_id: 9,
    }
}

// ─── Sprout Tower 3F (14x14) ─────────────────────────────
// Top floor: Rival event, Sage Jin (Bellsprout Lv6), Sage Troy (Bellsprout Lv7+Hoothoot Lv7),
// Sage Neal (Bellsprout Lv6), Elder Li (2x Bellsprout Lv7+Hoothoot Lv10).
// Elder Li gives HM05 Flash on defeat. Items: Potion, Escape Rope.
fn build_sprout_tower_3f() -> MapData {
    let w: usize = 14;
    let h: usize = 14;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: top wall
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: back wall — Elder Li area
        BLACK,FLOOR,BOOKSHELF,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BOOKSHELF,FLOOR,BLACK,
        // Row 2: floor — Elder Li stands here
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 3: floor with center pillar
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,TABLE,TABLE,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 4: floor with center pillar — rival event trigger zone
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,TABLE,TABLE,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 5: floor — Sage Jin area
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 6: floor — Sage Troy area
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 7: bookshelves on sides
        BLACK,BOOKSHELF,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BOOKSHELF,BLACK,
        // Row 8: floor — Sage Neal area
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 9: floor — items (Potion, Escape Rope)
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 10: center pillar lower
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,TABLE,TABLE,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 11: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 12: floor — stairs down (left side)
        BLACK,BOOKSHELF,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 13: bottom wall
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid wall
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: bookshelves solid
        C_SOLID,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_SOLID,
        // Row 2: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 3: center pillar solid
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 4: center pillar solid
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 5: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 7: bookshelves solid on sides
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 8: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 9: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 10: center pillar solid
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 11: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 12: stairs down warp at (1,12)
        C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 13: solid wall
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "SproutTower3F tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "SproutTower3F collision count mismatch");

    let warps = vec![
        // Stairs down to 2F (left side)
        WarpData { x: 1, y: 12, dest_map: MapId::SproutTower2F, dest_x: 12, dest_y: 2 },
    ];

    let npcs = vec![
        // NPC 0: Elder Li — top-floor boss trainer
        NpcDef {
            x: 7, y: 2, sprite_id: 5, facing: Direction::Down,
            dialogue: &[
                "ELDER LI: So you have",
                "come this far.",
                "Let me test your",
                "bond with POKEMON!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: BELLSPROUT, level: 7 },
                TrainerPokemon { species_id: BELLSPROUT, level: 7 },
                TrainerPokemon { species_id: HOOTHOOT, level: 10 },
            ],
        },
        // NPC 1: Sage Jin — trainer (Bellsprout Lv6)
        NpcDef {
            x: 3, y: 5, sprite_id: 5, facing: Direction::Right,
            dialogue: &[
                "SAGE JIN: The swaying",
                "pillar teaches us",
                "balance!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: BELLSPROUT, level: 6 },
            ],
        },
        // NPC 2: Sage Troy — trainer (Bellsprout Lv7 + Hoothoot Lv7)
        NpcDef {
            x: 10, y: 6, sprite_id: 5, facing: Direction::Left,
            dialogue: &[
                "SAGE TROY: A true",
                "trainer respects all",
                "POKEMON! Prepare!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: BELLSPROUT, level: 7 },
                TrainerPokemon { species_id: HOOTHOOT, level: 7 },
            ],
        },
        // NPC 3: Sage Neal — trainer (Bellsprout Lv6)
        NpcDef {
            x: 3, y: 8, sprite_id: 5, facing: Direction::Right,
            dialogue: &[
                "SAGE NEAL: The path to",
                "the ELDER is not easy!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: BELLSPROUT, level: 6 },
            ],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: RATTATA, min_level: 3, max_level: 5, weight: 60 },
        EncounterEntry { species_id: GASTLY, min_level: 3, max_level: 5, weight: 40 },
    ];

    MapData {
        id: MapId::SproutTower3F,
        name: "SPROUT TOWER 3F",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![], water_encounters: vec![],
        music_id: 9,
    }
}

// ─── Player's House 1F (10x8) ──────────────────────────
// Simple interior with floor, table, TV/bookshelf, stairs, door.

fn build_player_house_1f() -> MapData {
    let w: usize = 10;
    let h: usize = 8;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: top wall (black border)
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: bookshelves and TV along wall
        BLACK,BOOKSHELF,BOOKSHELF,FLOOR,FLOOR,FLOOR,FLOOR,BOOKSHELF,PC,BLACK,
        // Row 2: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 3: table area
        BLACK,FLOOR,FLOOR,TABLE,TABLE,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 4: chairs around table
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 5: open floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 6: stairs area (left) and floor
        BLACK,BOOKSHELF,BOOKSHELF,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 7: bottom wall with 2-tile-wide door
        BLACK,BLACK,BLACK,BLACK,DOOR,DOOR,BLACK,BLACK,BLACK,BLACK,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid wall
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: furniture solid, floor walk
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 3: table solid
        C_SOLID,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 4: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 5: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6: stairs solid, floor
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 7: wall solid, 2-tile door warp
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "PlayerHouse1F tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "PlayerHouse1F collision count mismatch");

    let warps = vec![
        // Door exits to New Bark Town (in front of player house) - 2 tiles wide
        WarpData { x: 4, y: 7, dest_map: MapId::NewBarkTown, dest_x: 5, dest_y: 5 },
        WarpData { x: 5, y: 7, dest_map: MapId::NewBarkTown, dest_x: 5, dest_y: 5 },
    ];

    let npcs = vec![
        // Mom NPC
        NpcDef {
            x: 5, y: 4, sprite_id: 1, facing: Direction::Down,
            dialogue: &[
                "...... ...... ......",
                "Right. All boys leave",
                "home some day. It said",
                "so on TV.",
                "Oh, did PROF.ELM call?",
                "Better go see him then!",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];

    MapData {
        id: MapId::PlayerHouse1F,
        name: "PLAYER'S HOUSE",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters: vec![],
        night_encounters: vec![], water_encounters: vec![],
        music_id: 4,
    }
}

// ─── Player's House 2F (10x8) ──────────────────────────
// Player's bedroom. Mostly decorative.

fn build_player_house_2f() -> MapData {
    let w: usize = 10;
    let h: usize = 8;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: top wall
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: PC and bed area
        BLACK,PC,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BOOKSHELF,BOOKSHELF,BLACK,
        // Row 2: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 3: floor with table
        BLACK,FLOOR,FLOOR,FLOOR,TABLE,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 4: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 5: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 6: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 7: bottom wall (stairs down)
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid wall
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: PC solid, bed solid
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 3: table solid
        C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 4: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 5: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 7: solid wall
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "PlayerHouse2F tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "PlayerHouse2F collision count mismatch");

    MapData {
        id: MapId::PlayerHouse2F,
        name: "PLAYER'S ROOM",
        width: w,
        height: h,
        tiles,
        collision,
        warps: vec![],
        npcs: vec![],
        encounters: vec![],
        night_encounters: vec![], water_encounters: vec![],
        music_id: 4,
    }
}

// ─── Elm's Lab (10x10) ─────────────────────────────────
// Professor Elm's laboratory. Tables with pokeballs, bookshelves, PC.

fn build_elm_lab() -> MapData {
    let w: usize = 10;
    let h: usize = 10;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: top wall
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: bookshelves along top wall, PC in corner
        BLACK,BOOKSHELF,BOOKSHELF,BOOKSHELF,FLOOR,FLOOR,BOOKSHELF,BOOKSHELF,PC,BLACK,
        // Row 2: open floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 3: floor with machines/bookshelves on sides
        BLACK,BOOKSHELF,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BOOKSHELF,BLACK,
        // Row 4: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 5: pokeball table (the starter selection table)
        BLACK,FLOOR,FLOOR,TABLE,TABLE,TABLE,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 6: floor in front of table (Elm stands here)
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 7: open floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 8: floor near door
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 9: bottom wall with 2-tile-wide door
        BLACK,BLACK,BLACK,BLACK,DOOR,DOOR,BLACK,BLACK,BLACK,BLACK,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid wall
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: bookshelves & PC solid
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 3: bookshelves on sides solid
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 4: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 5: pokeball table (counter - can interact across)
        C_SOLID,C_WALK,C_WALK,C_COUNTER,C_COUNTER,C_COUNTER,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6: floor (Elm stands here)
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 7: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 8: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 9: wall + 2-tile door warp
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "ElmLab tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "ElmLab collision count mismatch");

    let warps = vec![
        // Door exits to New Bark Town (in front of lab) - 2 tiles wide
        WarpData { x: 4, y: 9, dest_map: MapId::NewBarkTown, dest_x: 17, dest_y: 5 },
        WarpData { x: 5, y: 9, dest_map: MapId::NewBarkTown, dest_x: 17, dest_y: 5 },
    ];

    let npcs = vec![
        // Prof. Elm standing near the pokeball table
        NpcDef {
            x: 4, y: 6, sprite_id: 0, facing: Direction::Down,
            dialogue: &[
                "Ah, I've been waiting",
                "for you!",
                "I'm PROF.ELM. I study",
                "rare POKEMON.",
                "I have a favor to ask",
                "of you. Could you go to",
                "MR.POKEMON's house for me?",
                "Here, take one of these",
                "POKEMON for protection!",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];

    MapData {
        id: MapId::ElmLab,
        name: "ELM'S LAB",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters: vec![],
        night_encounters: vec![], water_encounters: vec![],
        music_id: 5,
    }
}

// ─── Pokemon Center Interior (10x8) ────────────────────
// Healing machine with Nurse Joy behind counter, PC, decorations.

fn build_pokemon_center() -> MapData {
    let w: usize = 10;
    let h: usize = 8;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: top wall
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: back wall with healing machine and bookshelves
        BLACK,BOOKSHELF,FLOOR,FLOOR,HEAL_MACHINE,HEAL_MACHINE,FLOOR,FLOOR,PC,BLACK,
        // Row 2: Nurse Joy stands behind counter here (visually)
        BLACK,FLOOR,FLOOR,FLOOR,HEAL_MACHINE,HEAL_MACHINE,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 3: counter top (the line patients face)
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 4: open floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 5: open floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 6: floor near entrance
        BLACK,BOOKSHELF,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BOOKSHELF,BLACK,
        // Row 7: bottom wall with 2-tile-wide door
        BLACK,BLACK,BLACK,BLACK,DOOR,DOOR,BLACK,BLACK,BLACK,BLACK,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid wall
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: furniture solid
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 2: healing machine area (counter - can interact across)
        C_SOLID,C_WALK,C_WALK,C_WALK,C_COUNTER,C_COUNTER,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 3: floor in front of counter
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 4: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 5: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6: bookshelves solid on sides
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 7: wall + 2-tile door warp
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "PokemonCenter tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "PokemonCenter collision count mismatch");

    let warps = vec![
        // Door exits to Cherrygrove City (in front of pokecenter) - 2 tiles wide
        WarpData { x: 4, y: 7, dest_map: MapId::CherrygroveCity, dest_x: 7, dest_y: 5 },
        WarpData { x: 5, y: 7, dest_map: MapId::CherrygroveCity, dest_x: 7, dest_y: 5 },
    ];

    let npcs = vec![
        // Nurse Joy behind counter
        NpcDef {
            x: 4, y: 2, sprite_id: 4, facing: Direction::Down,
            dialogue: &[
                "Welcome to our POKEMON",
                "CENTER!",
                "We restore your tired",
                "POKEMON to full health.",
                "Would you like me to heal",
                "your POKEMON?",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];

    MapData {
        id: MapId::PokemonCenter,
        name: "POKEMON CENTER",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters: vec![],
        night_encounters: vec![], water_encounters: vec![],
        music_id: 6,
    }
}

// ─── Route 32 (20x30) ──────────────────────────────────
// Long vertical route connecting Violet City (north) to Union Cave (south).
// Path runs down the middle, tall grass on sides, water on the right,
// sign near entrance, two trainer NPCs.

fn build_route_32() -> MapData {
    let w: usize = 20;
    let h: usize = 30;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: north entrance from Violet City — trees with gap at x=9,10
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,PATH,
        PATH,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: tree bottoms with gap
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,PATH,
        PATH,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: grass with path and sign
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,SIGN,GRASS,GRASS,GRASS,GRASS,WATER,WATER,WATER,
        // Row 3: grass + tall grass patches left, path center, water right
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,WATER,WATER,WATER,
        // Row 4: tall grass left, path center, water right
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,WATER,WATER,WATER,
        // Row 5: grass, path, water
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,WATER,WATER,WATER,
        // Row 6: grass, path, fence segment on right
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,FENCE_H,FENCE_H,WATER,WATER,WATER,
        // Row 7: tall grass left, path center, water right
        GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,WATER,WATER,WATER,
        // Row 8: tall grass, path, trainer area
        GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,WATER,WATER,WATER,
        // Row 9: grass, path, grass
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,WATER,WATER,WATER,
        // Row 10: trees left, path, grass, water
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,WATER,WATER,WATER,
        // Row 11: tree bottoms, path, grass, water
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,WATER,WATER,WATER,
        // Row 12: grass, tall grass patch, path
        GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 13: grass, tall grass patch, path
        GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 14: grass, path
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 15: grass, path, tall grass right
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 16: grass, path, tall grass right
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 17: trees left, path
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 18: tree bottoms, path
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 19: grass, path, trainer area
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 20: grass, tall grass left, path
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 21: grass, tall grass left, path
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 22: grass, path
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 23: grass, path, fence
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,FENCE_H,FENCE_H,FENCE_H,FENCE_H,
        // Row 24: grass, path, trees right
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 25: grass, path, trees right
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 26: grass, path widens to cave entrance
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,
        PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 27: tree bottoms, cave entrance area
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,
        PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 28: trees across bottom with cave entrance gap at x=9,10
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,PATH,
        PATH,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 29: tree bottoms with cave entrance gap (warp to Union Cave)
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,PATH,
        PATH,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid trees with warp gap at x=9,10 (from Violet City)
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,
        C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: solid trees with walkable gap
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,
        C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: grass, path, sign, water
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_SIGN,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        // Row 3: grass, tall grass, path, water
        C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        // Row 4: tall grass, path, water
        C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        // Row 5: grass, path, water
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        // Row 6: grass, path, fence, water
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WATER,C_WATER,C_WATER,
        // Row 7: tall grass, path, water
        C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        // Row 8: tall grass, path, water
        C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        // Row 9: grass, path, water
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        // Row 10: trees, path, water
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        // Row 11: trees, path, water
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        // Row 12: grass, tall grass, path
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 13: grass, tall grass, path
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 14: grass, path
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 15: grass, path, tall grass right
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 16: grass, path, tall grass right
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 17: trees, path, trees
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 18: trees, path, trees
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 19: grass, path
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 20: grass, tall grass, path
        C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 21: grass, tall grass, path
        C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 22: grass, path
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 23: grass, path, fence
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 24: grass, path, trees
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 25: grass, path, trees
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 26: trees, path widens, trees
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 27: trees, cave entrance area, trees
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 28: solid trees with gap at x=9,10
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,
        C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 29: solid trees with warp gap at x=9,10 (to Union Cave)
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,
        C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "Route32 tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "Route32 collision count mismatch");

    let warps = vec![
        // North exit to Violet City (x=9,10 at row 0)
        WarpData { x: 9, y: 0, dest_map: MapId::VioletCity, dest_x: 11, dest_y: 16 },
        WarpData { x: 10, y: 0, dest_map: MapId::VioletCity, dest_x: 12, dest_y: 16 },
        // South exit to Union Cave (x=9,10 at row 29)
        WarpData { x: 9, y: 29, dest_map: MapId::UnionCave, dest_x: 7, dest_y: 2 },
        WarpData { x: 10, y: 29, dest_map: MapId::UnionCave, dest_x: 8, dest_y: 2 },
    ];

    let npcs = vec![
        // Youngster trainer near the top of the route
        NpcDef {
            x: 6, y: 8, sprite_id: 2, facing: Direction::Right,
            dialogue: &[
                "I've been training hard",
                "on ROUTE 32!",
                "My POKEMON are getting",
                "stronger every day!",
                "Let's see how yours",
                "measure up!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: RATTATA, level: 6 },
                TrainerPokemon { species_id: BELLSPROUT, level: 7 },
            ],
        },
        // Lass trainer near the middle-bottom of the route
        NpcDef {
            x: 13, y: 19, sprite_id: 3, facing: Direction::Left,
            dialogue: &[
                "Did you know UNION CAVE",
                "is just ahead?",
                "You'd better be ready!",
                "Let's battle first!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: PIDGEY, level: 7 },
                TrainerPokemon { species_id: HOPPIP, level: 6 },
            ],
        },
        // Hint NPC about Union Cave
        NpcDef {
            x: 8, y: 24, sprite_id: 5, facing: Direction::Down,
            dialogue: &[
                "UNION CAVE connects",
                "to AZALEA TOWN.",
                "Watch out for the",
                "wild POKEMON inside!",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: BELLSPROUT, min_level: 6, max_level: 8, weight: 20 },
        EncounterEntry { species_id: RATTATA, min_level: 5, max_level: 7, weight: 15 },
        EncounterEntry { species_id: PIDGEY, min_level: 6, max_level: 8, weight: 15 },
        EncounterEntry { species_id: WEEDLE, min_level: 5, max_level: 7, weight: 10 },
        EncounterEntry { species_id: HOPPIP, min_level: 6, max_level: 7, weight: 15 },
        EncounterEntry { species_id: MAREEP, min_level: 6, max_level: 8, weight: 25 },
    ];

    MapData {
        id: MapId::Route32,
        name: "ROUTE 32",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![],
        water_encounters: vec![
            EncounterEntry { species_id: TENTACOOL, min_level: 15, max_level: 20, weight: 40 },
            EncounterEntry { species_id: QUAGSIRE, min_level: 15, max_level: 20, weight: 15 },
            EncounterEntry { species_id: MAGIKARP, min_level: 10, max_level: 15, weight: 30 },
            EncounterEntry { species_id: GOLDEEN, min_level: 15, max_level: 20, weight: 15 },
        ],
        music_id: 3,
    }
}

// ─── Union Cave (16x16) ────────────────────────────────
// Dark cave interior connecting from Route 32.
// BLACK walls, FLOOR walkable area, PATH main route.
// Wild encounters: Geodude, Zubat, Onix, Rattata.

fn build_union_cave() -> MapData {
    let w: usize = 16;
    let h: usize = 16;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: top wall
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,
        // Row 1: back wall with entrance gap at x=7,8
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,PATH,PATH,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,
        // Row 2: cave interior opening up
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,PATH,PATH,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,
        // Row 3: wider cave area
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,PATH,PATH,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,CAVE_WALL,
        // Row 4: main cave area
        CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,PATH,PATH,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,
        // Row 5: wide area with side alcove left
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,PATH,PATH,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 6: wide area
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,PATH,PATH,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 7: path bends, open area
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,PATH,PATH,PATH,PATH,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 8: wide chamber
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,PATH,CAVE_FLOOR,CAVE_FLOOR,PATH,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 9: path continues south
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,PATH,PATH,PATH,PATH,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 10: wide area
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,PATH,PATH,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 11: narrowing cave
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,PATH,PATH,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 12: cave narrows
        CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,PATH,PATH,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,
        // Row 13: narrower
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,PATH,PATH,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,CAVE_WALL,
        // Row 14: near south exit
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,PATH,PATH,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,
        // Row 15: south wall with exit gap at x=7,8
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,PATH,PATH,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid wall
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: wall with warp entrance at x=7,8 (from Route 32)
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: cave opening
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 3: wider
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 4: main area
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 5: wide area
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6: wide area
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 7: path area
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 8: chamber
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 9: path
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 10: wide area
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 11: cave
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 12: narrowing
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 13: narrower
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 14: near exit
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 15: wall with warp exit at x=7,8 (back to Route 32)
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "UnionCave tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "UnionCave collision count mismatch");

    let warps = vec![
        // North entrance warps back to Route 32 (near south end of Route 32)
        WarpData { x: 7, y: 1, dest_map: MapId::Route32, dest_x: 9, dest_y: 28 },
        WarpData { x: 8, y: 1, dest_map: MapId::Route32, dest_x: 10, dest_y: 28 },
        // South exit to Route 33
        WarpData { x: 7, y: 15, dest_map: MapId::Route33, dest_x: 1, dest_y: 5 },
        WarpData { x: 8, y: 15, dest_map: MapId::Route33, dest_x: 1, dest_y: 6 },
    ];

    let npcs = vec![
        // Hiker trainer in the cave
        NpcDef {
            x: 5, y: 7, sprite_id: 5, facing: Direction::Right,
            dialogue: &[
                "I'm a HIKER! I love",
                "exploring caves!",
                "UNION CAVE goes deep",
                "underground. They say",
                "LAPRAS appears in the",
                "basement on Fridays!",
                "But first, let's battle!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: GEODUDE, level: 8 },
                TrainerPokemon { species_id: GEODUDE, level: 6 },
            ],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: ZUBAT, min_level: 5, max_level: 7, weight: 30 },
        EncounterEntry { species_id: GEODUDE, min_level: 6, max_level: 8, weight: 25 },
        EncounterEntry { species_id: ONIX, min_level: 6, max_level: 8, weight: 10 },
        EncounterEntry { species_id: RATTATA, min_level: 5, max_level: 7, weight: 10 },
        EncounterEntry { species_id: WOOPER, min_level: 5, max_level: 8, weight: 25 },
    ];

    MapData {
        id: MapId::UnionCave,
        name: "UNION CAVE",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![], water_encounters: vec![],
        music_id: 9,
    }
}

// ─── Utility Functions ──────────────────────────────────

impl MapData {
    /// Get the tile at (x, y). Returns None if out of bounds.
    pub fn tile_at(&self, x: usize, y: usize) -> Option<u8> {
        if x < self.width && y < self.height {
            Some(self.tiles[y * self.width + x])
        } else {
            None
        }
    }

    /// Get the collision type at (x, y). Returns Solid if out of bounds.
    pub fn collision_at(&self, x: usize, y: usize) -> CollisionType {
        if x < self.width && y < self.height {
            CollisionType::from_u8(self.collision[y * self.width + x])
        } else {
            CollisionType::Solid
        }
    }

    /// Check if a position is walkable.
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        matches!(
            self.collision_at(x, y),
            CollisionType::Walkable | CollisionType::TallGrass | CollisionType::Warp | CollisionType::Ledge | CollisionType::Ice
        )
    }

    /// Find the warp at (x, y), if any.
    pub fn warp_at(&self, x: u8, y: u8) -> Option<&WarpData> {
        self.warps.iter().find(|w| w.x == x && w.y == y)
    }

    /// Check if the position has tall grass (triggers encounter check).
    pub fn is_tall_grass(&self, x: usize, y: usize) -> bool {
        self.collision_at(x, y) == CollisionType::TallGrass
    }

    /// Pick a random encounter based on weights. Returns (species_id, level).
    /// `roll` should be in [0.0, 1.0) for species selection.
    /// `level_roll` should be in [0.0, 1.0) for level within range.
    pub fn roll_encounter(&self, roll: f64, level_roll: f64) -> Option<(u16, u8)> {
        Self::roll_from_table(&self.encounters, roll, level_roll)
    }

    /// Roll encounter with time of day. Uses night_encounters if night and non-empty.
    pub fn roll_encounter_timed(&self, roll: f64, level_roll: f64, is_night: bool) -> Option<(u16, u8)> {
        let table = if is_night && !self.night_encounters.is_empty() {
            &self.night_encounters
        } else {
            &self.encounters
        };
        Self::roll_from_table(table, roll, level_roll)
    }

    fn roll_from_table(table: &[EncounterEntry], roll: f64, level_roll: f64) -> Option<(u16, u8)> {
        if table.is_empty() {
            return None;
        }

        let total_weight: u32 = table.iter().map(|e| e.weight as u32).sum();
        if total_weight == 0 {
            return None;
        }

        let target = (roll * total_weight as f64) as u32;
        let mut cumulative: u32 = 0;

        for entry in table {
            cumulative += entry.weight as u32;
            if target < cumulative {
                let level_range = (entry.max_level - entry.min_level) as f64;
                let level = entry.min_level + (level_roll * (level_range + 1.0)).min(level_range) as u8;
                return Some((entry.species_id, level));
            }
        }

        // Fallback: last encounter
        if let Some(last) = table.last() {
            Some((last.species_id, last.min_level))
        } else {
            None
        }
    }
}

fn build_generic_house() -> MapData {
    let w: usize = 8;
    let h: usize = 7;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        BLACK,BOOKSHELF,FLOOR,FLOOR,FLOOR,FLOOR,PC,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,TABLE,TABLE,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,BLACK,BLACK,DOOR,DOOR,BLACK,BLACK,BLACK,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "GenericHouse tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "GenericHouse collision count mismatch");

    // Exit warp — destination will be overridden dynamically (like PokemonCenter)
    let warps = vec![
        WarpData { x: 3, y: 6, dest_map: MapId::NewBarkTown, dest_x: 12, dest_y: 5 },
        WarpData { x: 4, y: 6, dest_map: MapId::NewBarkTown, dest_x: 12, dest_y: 5 },
    ];

    let npcs = vec![
        NpcDef {
            x: 4, y: 3, sprite_id: 5, facing: Direction::Down,
            dialogue: &[
                "I love this town.",
                "It's so peaceful here.",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];

    MapData {
        id: MapId::GenericHouse,
        name: "HOUSE",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters: vec![],
        night_encounters: vec![], water_encounters: vec![],
        music_id: 0,
    }
}

// ─── Route 33 (20x12) ──────────────────────────────────
// Short route connecting Union Cave to Azalea Town. Rainy in the game.

fn build_route_33() -> MapData {
    let w: usize = 20;
    let h: usize = 12;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: trees
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: trees
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: trees + grass
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 3: trees + grass + path
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4: grass + path
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,GRASS,
        GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,
        // Row 5: main east-west path (entry from Union Cave left)
        PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,PATH,PATH,
        PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,PATH,PATH,
        // Row 6: main path continued (exit to Azalea right)
        PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,PATH,PATH,PATH,
        // Row 7: grass + tall grass
        GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,PATH,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,PATH,GRASS,GRASS,
        // Row 8: grass
        GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,
        // Row 9: grass
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 10: tree tops
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 11: tree bottoms
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0-1: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 3
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 4
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5: left warp from Union Cave, right continues
        C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,
        // Row 6: right warp to Azalea Town
        C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,
        // Row 7
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,
        // Row 8
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,
        // Row 9
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 10-11: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "Route33 tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "Route33 collision count mismatch");

    let warps = vec![
        // Left edge -> Union Cave south exit
        WarpData { x: 0, y: 5, dest_map: MapId::UnionCave, dest_x: 7, dest_y: 14 },
        WarpData { x: 0, y: 6, dest_map: MapId::UnionCave, dest_x: 8, dest_y: 14 },
        // Right edge -> Azalea Town
        WarpData { x: 19, y: 5, dest_map: MapId::AzaleaTown, dest_x: 1, dest_y: 9 },
        WarpData { x: 19, y: 6, dest_map: MapId::AzaleaTown, dest_x: 1, dest_y: 10 },
    ];

    let npcs = vec![
        NpcDef {
            x: 10, y: 4, sprite_id: 2, facing: Direction::Down,
            dialogue: &[
                "It always rains on",
                "ROUTE 33. So annoying!",
            ],
            is_trainer: false, is_mart: false, wanders: true, trainer_team: &[],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: RATTATA, min_level: 6, max_level: 8, weight: 30 },
        EncounterEntry { species_id: GEODUDE, min_level: 6, max_level: 8, weight: 20 },
        EncounterEntry { species_id: ZUBAT, min_level: 6, max_level: 8, weight: 15 },
        EncounterEntry { species_id: HOPPIP, min_level: 6, max_level: 8, weight: 20 },
        EncounterEntry { species_id: SPINARAK, min_level: 6, max_level: 8, weight: 15 },
    ];

    MapData {
        id: MapId::Route33,
        name: "ROUTE 33",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![], water_encounters: vec![],
        music_id: 2,
    }
}

// ─── Azalea Town (20x18) ────────────────────────────────
// Home of Kurt the Pokeball maker and Bugsy's Gym.

fn build_azalea_town() -> MapData {
    let w: usize = 20;
    let h: usize = 18;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: tree tops with NE exit to Ilex Forest
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,PATH,PATH,TREE_TOP,
        // Row 1: tree bottoms with path to Ilex
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,PATH,PATH,TREE_BOTTOM,
        // Row 2: grass + building roofs
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 3: grass + Kurt's house wall
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,BUILDING_WALL,BUILDING_WALL,BUILDING_WALL,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 4: grass + Kurt's house door
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,BUILDING_WALL,DOOR,BUILDING_WALL,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 5: grass + path in front
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,
        PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 6: Gym roof + path
        GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,
        GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 7: Gym wall + path
        GRASS,BUILDING_WALL,BUILDING_WALL,BUILDING_WALL,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,
        GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 8: Gym door + path
        GRASS,BUILDING_WALL,DOOR,BUILDING_WALL,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,
        GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 9: path (entry from Route 33)
        PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 10: path + PokemonCenter
        PATH,PATH,GRASS,GRASS,GRASS,POKECENTER_ROOF,POKECENTER_ROOF,POKECENTER_ROOF,GRASS,GRASS,
        GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 11: PokemonCenter + mart
        PATH,GRASS,GRASS,GRASS,GRASS,POKECENTER_WALL,POKECENTER_WALL,POKECENTER_WALL,GRASS,BUILDING_ROOF,
        BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 12: PokemonCenter door + mart wall
        GRASS,GRASS,GRASS,GRASS,GRASS,POKECENTER_WALL,POKECENTER_DOOR,POKECENTER_WALL,GRASS,BUILDING_WALL,
        BUILDING_WALL,BUILDING_WALL,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 13: path + mart door
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,BUILDING_WALL,
        DOOR,BUILDING_WALL,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 14: grass + sign
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,SIGN,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 15: flowers + grass
        GRASS,GRASS,FLOWER,FLOWER,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 16: tree tops
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 17: tree bottoms
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid trees with NE exit warp to Ilex Forest
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,
        // Row 1: path leading to Ilex exit
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 2: building roof solid
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 3: building wall solid
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 4: Kurt's door
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_SOLID,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5: path
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 6: Gym roof solid
        C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 7: Gym wall solid
        C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 8: Gym door
        C_WALK,C_SOLID,C_WARP,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 9: path (left warp from Route 33)
        C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 10: PokemonCenter roof solid
        C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 11: walls
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 12: PokemonCenter door, mart wall
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_SOLID,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 13: mart door
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_WARP,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 14: sign
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_SIGN,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 15
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 16-17: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "AzaleaTown tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "AzaleaTown collision count mismatch");

    let warps = vec![
        // West edge -> Route 33
        WarpData { x: 0, y: 9, dest_map: MapId::Route33, dest_x: 18, dest_y: 5 },
        WarpData { x: 0, y: 10, dest_map: MapId::Route33, dest_x: 18, dest_y: 6 },
        // Kurt's house door -> GenericHouse
        WarpData { x: 8, y: 4, dest_map: MapId::GenericHouse, dest_x: 3, dest_y: 5 },
        // Azalea Gym door
        WarpData { x: 2, y: 8, dest_map: MapId::AzaleaGym, dest_x: 5, dest_y: 8 },
        // Pokemon Center door
        WarpData { x: 6, y: 12, dest_map: MapId::PokemonCenter, dest_x: 4, dest_y: 6 },
        // Mart door -> GenericHouse (mart interior)
        WarpData { x: 10, y: 13, dest_map: MapId::GenericHouse, dest_x: 3, dest_y: 5 },
        // Northeast exit -> Ilex Forest (south)
        WarpData { x: 17, y: 0, dest_map: MapId::IlexForest, dest_x: 3, dest_y: 18 },
        WarpData { x: 18, y: 0, dest_map: MapId::IlexForest, dest_x: 4, dest_y: 18 },
    ];

    let npcs = vec![
        NpcDef {
            x: 5, y: 5, sprite_id: 2, facing: Direction::Down,
            dialogue: &[
                "KURT makes special POKE",
                "BALLS from APRICORNS.",
                "BUGSY runs the GYM.",
                "He's a bug expert!",
            ],
            is_trainer: false, is_mart: false, wanders: true, trainer_team: &[],
        },
        NpcDef {
            x: 12, y: 12, sprite_id: 5, facing: Direction::Left,
            dialogue: &[
                "Welcome! We have a great",
                "selection of items today!",
            ],
            is_trainer: false, is_mart: true, wanders: false, trainer_team: &[],
        },
    ];

    MapData {
        id: MapId::AzaleaTown,
        name: "AZALEA TOWN",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters: vec![],
        night_encounters: vec![], water_encounters: vec![],
        music_id: 10,
    }
}

// ─── Azalea Gym (10x10) ────────────────────────────────
// Bug-type gym. Leader: Bugsy.

fn build_azalea_gym() -> MapData {
    let w: usize = 10;
    let h: usize = 10;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: black walls
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: gym interior
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 2: gym leader area
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 3
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 4
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 5: trainee area
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 6
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 7
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 8
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 9: exit door
        BLACK,BLACK,BLACK,BLACK,DOOR,DOOR,BLACK,BLACK,BLACK,BLACK,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "AzaleaGym tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "AzaleaGym collision count mismatch");

    let warps = vec![
        WarpData { x: 4, y: 9, dest_map: MapId::AzaleaTown, dest_x: 2, dest_y: 9 },
        WarpData { x: 5, y: 9, dest_map: MapId::AzaleaTown, dest_x: 2, dest_y: 9 },
    ];

    let npcs = vec![
        // Gym Leader Bugsy
        NpcDef {
            x: 5, y: 2, sprite_id: 2, facing: Direction::Down,
            dialogue: &[
                "I'm BUGSY!",
                "I never lose when it",
                "comes to BUG POKEMON!",
                "My research is going",
                "to make me the best!",
                "Let me demonstrate!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: CATERPIE, level: 14 },
                TrainerPokemon { species_id: SPINARAK, level: 14 },
                TrainerPokemon { species_id: CATERPIE, level: 16 },
            ],
        },
        // Gym trainee
        NpcDef {
            x: 3, y: 5, sprite_id: 2, facing: Direction::Right,
            dialogue: &[
                "BUG POKEMON are so",
                "cool! They evolve",
                "really quickly!",
                "Battle me!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: CATERPIE, level: 10 },
                TrainerPokemon { species_id: WEEDLE, level: 10 },
            ],
        },
    ];

    MapData {
        id: MapId::AzaleaGym,
        name: "AZALEA GYM",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters: vec![],
        night_encounters: vec![], water_encounters: vec![],
        music_id: 8,
    }
}

// ─── Ilex Forest (16x20) ─────────────────────────────────
// Dense forest connecting Azalea Town (south) to Route 34 (north).
// Contains Farfetch'd chase event, Charcoal Maker, Cut tree, and Headbutt tutor.
fn build_ilex_forest() -> MapData {
    let w: usize = 16;
    let h: usize = 20;

    // T=TREE_TOP, B=TREE_BOTTOM, G=GRASS, P=PATH, TG=TALL_GRASS, F=FLOWER
    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: dense tree canopy top
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: tree bottoms with north exit gap (blocked by Cut tree)
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,PATH,PATH,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: forest path winds east
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,GRASS,PATH,PATH,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 3
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,PATH,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4: headbutt tutor area
        TREE_TOP,TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 5
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,TALL_GRASS,GRASS,PATH,GRASS,GRASS,TALL_GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 6: winding path area
        TREE_TOP,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,PATH,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 7
        TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 8: Farfetch'd area
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,GRASS,GRASS,TREE_TOP,
        // Row 9
        TREE_BOTTOM,GRASS,GRASS,TREE_TOP,GRASS,PATH,GRASS,TREE_TOP,GRASS,GRASS,GRASS,TREE_BOTTOM,GRASS,GRASS,GRASS,TREE_BOTTOM,
        // Row 10: mid-forest clearing
        TREE_TOP,GRASS,GRASS,TREE_BOTTOM,GRASS,PATH,GRASS,TREE_BOTTOM,GRASS,TREE_TOP,GRASS,GRASS,GRASS,TREE_TOP,GRASS,TREE_TOP,
        // Row 11
        TREE_BOTTOM,GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,GRASS,TREE_BOTTOM,GRASS,GRASS,GRASS,TREE_BOTTOM,GRASS,TREE_BOTTOM,
        // Row 12: second Farfetch'd area
        TREE_TOP,TREE_TOP,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,
        // Row 13
        TREE_BOTTOM,GRASS,GRASS,PATH,PATH,GRASS,TREE_TOP,GRASS,GRASS,TREE_TOP,GRASS,GRASS,TREE_TOP,GRASS,GRASS,TREE_BOTTOM,
        // Row 14: charcoal kiln area
        TREE_TOP,GRASS,GRASS,PATH,GRASS,GRASS,TREE_BOTTOM,GRASS,GRASS,TREE_BOTTOM,GRASS,GRASS,TREE_BOTTOM,GRASS,GRASS,TREE_TOP,
        // Row 15
        TREE_BOTTOM,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,
        // Row 16
        TREE_TOP,GRASS,PATH,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,
        // Row 17
        TREE_BOTTOM,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,
        // Row 18: approaching south exit
        TREE_TOP,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TREE_TOP,TREE_TOP,GRASS,GRASS,TREE_TOP,TREE_TOP,GRASS,GRASS,TREE_TOP,
        // Row 19: south exit (to Azalea Town)
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,PATH,PATH,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid tree tops
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: tree bottoms with north exit (Cut tree blocks)
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 3
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 4
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 5
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 6
        C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 7
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 8
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_SOLID,
        // Row 9
        C_SOLID,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 10
        C_SOLID,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_SOLID,
        // Row 11
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_SOLID,
        // Row 12
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 13
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_SOLID,
        // Row 14
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_SOLID,
        // Row 15
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 16
        C_SOLID,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 17
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 18
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_SOLID,
        // Row 19: south exit
        C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    let warps = vec![
        // North exit → Route 34 (south end)
        WarpData { x: 7, y: 1, dest_map: MapId::Route34, dest_x: 7, dest_y: 17 },
        WarpData { x: 8, y: 1, dest_map: MapId::Route34, dest_x: 8, dest_y: 17 },
        // South exit → Azalea Town (north end)
        WarpData { x: 3, y: 19, dest_map: MapId::AzaleaTown, dest_x: 17, dest_y: 1 },
        WarpData { x: 4, y: 19, dest_map: MapId::AzaleaTown, dest_x: 18, dest_y: 1 },
    ];

    let npcs = vec![
        // Charcoal Maker's apprentice (near south entrance)
        NpcDef {
            x: 5, y: 15, sprite_id: 2, facing: Direction::Down,
            dialogue: &[
                "Oh no! My master's",
                "FARFETCH'D has run off",
                "into the forest!",
                "I can't control it",
                "because I don't have",
                "any GYM BADGES...",
                "Could you find it?",
            ],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
        // Headbutt tutor
        NpcDef {
            x: 7, y: 4, sprite_id: 3, facing: Direction::Down,
            dialogue: &[
                "Did you know you can",
                "HEADBUTT trees to find",
                "hidden POKEMON?",
                "Here, take this TM.",
            ],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: CATERPIE, min_level: 5, max_level: 6, weight: 30 },
        EncounterEntry { species_id: WEEDLE, min_level: 5, max_level: 6, weight: 15 },
        EncounterEntry { species_id: ODDISH, min_level: 5, max_level: 6, weight: 20 },
        EncounterEntry { species_id: PARAS, min_level: 5, max_level: 6, weight: 15 },
        EncounterEntry { species_id: ZUBAT, min_level: 5, max_level: 6, weight: 10 },
        EncounterEntry { species_id: PIDGEY, min_level: 7, max_level: 7, weight: 10 },
    ];

    MapData {
        id: MapId::IlexForest,
        name: "ILEX FOREST",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![], water_encounters: vec![],
        music_id: 10,
    }
}

// ─── Route 34 (16x20) ───────────────────────────────────
// Connects Ilex Forest (south) to Goldenrod City (north).
// Day Care Center near the north end. Tall grass with Drowzee, Rattata, Abra, Ditto.
fn build_route_34() -> MapData {
    let w: usize = 16;
    let h: usize = 20;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: north exit to Goldenrod
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,PATH,PATH,PATH,PATH,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,PATH,PATH,PATH,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: Day Care building
        TREE_TOP,TREE_TOP,TREE_TOP,BUILDING_ROOF,BUILDING_ROOF,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 3
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,BUILDING_WALL,BUILDING_WALL,GRASS,PATH,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4
        TREE_TOP,TREE_TOP,TREE_TOP,DOOR,FENCE_H,GRASS,PATH,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 5
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 6: trainer area
        TREE_TOP,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,PATH,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 7
        TREE_BOTTOM,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 8
        TREE_TOP,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 9
        TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 10
        TREE_TOP,GRASS,TALL_GRASS,GRASS,GRASS,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 11
        TREE_BOTTOM,GRASS,TALL_GRASS,GRASS,PATH,PATH,PATH,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 12
        TREE_TOP,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 13
        TREE_BOTTOM,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 14
        TREE_TOP,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 15
        TREE_BOTTOM,GRASS,PATH,PATH,GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 16
        TREE_TOP,GRASS,PATH,GRASS,GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 17
        TREE_BOTTOM,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 18
        TREE_TOP,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 19: south exit to Ilex Forest
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,PATH,PATH,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: north exit
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: Day Care roof
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 3
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 4: Day Care door
        C_SOLID,C_SOLID,C_SOLID,C_SIGN,C_SOLID,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 5
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 6
        C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 7
        C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 8
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 9
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 10
        C_SOLID,C_WALK,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 11
        C_SOLID,C_WALK,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 12
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 13
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 14
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 15
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 16
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 17
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 18
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 19: south exit
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    let warps = vec![
        // North exit → Goldenrod City (south end)
        WarpData { x: 6, y: 0, dest_map: MapId::GoldenrodCity, dest_x: 11, dest_y: 18 },
        WarpData { x: 7, y: 0, dest_map: MapId::GoldenrodCity, dest_x: 12, dest_y: 18 },
        WarpData { x: 8, y: 0, dest_map: MapId::GoldenrodCity, dest_x: 13, dest_y: 18 },
        WarpData { x: 9, y: 0, dest_map: MapId::GoldenrodCity, dest_x: 14, dest_y: 18 },
        // South exit → Ilex Forest (north exit)
        WarpData { x: 7, y: 19, dest_map: MapId::IlexForest, dest_x: 7, dest_y: 2 },
        WarpData { x: 8, y: 19, dest_map: MapId::IlexForest, dest_x: 8, dest_y: 2 },
    ];

    let npcs = vec![
        // Day Care Man (outside)
        NpcDef {
            x: 5, y: 4, sprite_id: 3, facing: Direction::Down,
            dialogue: &[
                "I'm the DAY-CARE MAN.",
                "We look after your",
                "POKEMON for you.",
                "Would you like to use",
                "the DAY-CARE?",
            ],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
        // Youngster trainer
        NpcDef {
            x: 6, y: 7, sprite_id: 2, facing: Direction::Left,
            dialogue: &[
                "I'm raising my team",
                "by battling trainers",
                "on this route!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: RATTATA, level: 12 },
                TrainerPokemon { species_id: PIDGEY, level: 12 },
            ],
        },
        // Picnicker
        NpcDef {
            x: 4, y: 11, sprite_id: 4, facing: Direction::Right,
            dialogue: &[
                "Route 34 is perfect",
                "for training POKEMON!",
                "Let's battle!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: HOPPIP, level: 11 },
                TrainerPokemon { species_id: HOPPIP, level: 11 },
            ],
        },
        // Hint NPC about daycare
        NpcDef {
            x: 5, y: 14, sprite_id: 5, facing: Direction::Up,
            dialogue: &[
                "The DAY-CARE MAN",
                "raises POKEMON for",
                "a fee. They gain",
                "EXP as you walk!",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: DROWZEE, min_level: 10, max_level: 12, weight: 35 },
        EncounterEntry { species_id: RATTATA, min_level: 11, max_level: 13, weight: 25 },
        EncounterEntry { species_id: PIDGEY, min_level: 12, max_level: 12, weight: 15 },
        EncounterEntry { species_id: ABRA,   min_level: 10, max_level: 12, weight: 10 },
        EncounterEntry { species_id: DROWZEE, min_level: 10, max_level: 10, weight: 10 },
        EncounterEntry { species_id: DROWZEE, min_level: 12, max_level: 12, weight: 5 },
    ];

    MapData {
        id: MapId::Route34,
        name: "ROUTE 34",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![], water_encounters: vec![],
        music_id: 2,
    }
}

// ─── Goldenrod City (24x20) ─────────────────────────────
// Largest city in Johto. Pokemon Center, Mart, Gym, Bike Shop, Radio Tower.
// South exit to Route 34, north exit to Route 35.
fn build_goldenrod_city() -> MapData {
    let w: usize = 24;
    let h: usize = 20;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: north border (trees + Route 35 exit)
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,PATH,PATH,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        GRASS,PATH,PATH,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: Radio Tower roof + buildings
        GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,GRASS,GRASS,GRASS,
        PATH,PATH,PATH,GRASS,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 3: Radio Tower wall
        GRASS,GRASS,BUILDING_WALL,BUILDING_WALL,BUILDING_WALL,GRASS,GRASS,GRASS,GRASS,GRASS,
        PATH,PATH,PATH,GRASS,GRASS,GRASS,BUILDING_WALL,BUILDING_WALL,BUILDING_WALL,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 4: Radio Tower door + path
        GRASS,GRASS,BUILDING_WALL,DOOR,BUILDING_WALL,GRASS,PATH,PATH,PATH,PATH,
        PATH,PATH,PATH,PATH,PATH,GRASS,BUILDING_WALL,DOOR,BUILDING_WALL,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 5: Main east-west road
        GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        PATH,PATH,PATH,GRASS,
        // Row 6: Dept Store roof
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,BUILDING_ROOF,
        BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 7: Dept Store wall
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,BUILDING_WALL,
        BUILDING_WALL,BUILDING_WALL,BUILDING_WALL,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 8: Dept Store door + path
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,BUILDING_WALL,
        BUILDING_WALL,DOOR,BUILDING_WALL,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 9: Gym roof area
        GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,PATH,PATH,PATH,PATH,
        PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 10: Gym wall
        GRASS,BUILDING_WALL,BUILDING_WALL,BUILDING_WALL,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 11: Gym door
        GRASS,BUILDING_WALL,DOOR,BUILDING_WALL,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 12: path + PokeCenter roof
        GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,BUILDING_ROOF,
        BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 13: PokeCenter wall + Pokecenter door area
        GRASS,GRASS,PATH,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,POKECENTER_WALL,
        POKECENTER_WALL,POKECENTER_WALL,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,
        BUILDING_ROOF,GRASS,GRASS,GRASS,
        // Row 14: PokeCenter door
        GRASS,GRASS,PATH,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,POKECENTER_WALL,
        POKECENTER_DOOR,POKECENTER_WALL,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,BUILDING_WALL,DOOR,
        BUILDING_WALL,GRASS,GRASS,GRASS,
        // Row 15: Bike Shop area + south path
        GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,FENCE_H,FENCE_H,
        FENCE_H,GRASS,GRASS,GRASS,
        // Row 16: Flower area
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,
        GRASS,FLOWER,FLOWER,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 17
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,
        GRASS,FLOWER,FLOWER,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,
        // Row 18
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,GRASS,PATH,PATH,PATH,PATH,
        PATH,PATH,PATH,PATH,PATH,PATH,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 19: south exit to Route 34
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,PATH,PATH,PATH,PATH,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: north border
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: roofs solid
        C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 3: walls solid
        C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 4: doors
        C_WALK,C_WALK,C_SOLID,C_SIGN,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SIGN,C_SOLID,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5: main road
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 6
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 7
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 8
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WARP,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 9: gym roof
        C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 10: gym wall
        C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 11: gym door
        C_WALK,C_SOLID,C_WARP,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 12: PokeCenter roof
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 13: PokeCenter wall
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,
        // Row 14: PokeCenter door + Bike Shop door
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_WARP,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SIGN,
        C_SOLID,C_WALK,C_WALK,C_WALK,
        // Row 15: path
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,
        // Row 16: flower area
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 17
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 18
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 19: south exit
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_WARP,C_WARP,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    let warps = vec![
        // North exit → Route 35
        WarpData { x: 11, y: 0, dest_map: MapId::Route35, dest_x: 6, dest_y: 18 },
        WarpData { x: 12, y: 0, dest_map: MapId::Route35, dest_x: 7, dest_y: 18 },
        // South exit → Route 34
        WarpData { x: 11, y: 19, dest_map: MapId::Route34, dest_x: 6, dest_y: 1 },
        WarpData { x: 12, y: 19, dest_map: MapId::Route34, dest_x: 7, dest_y: 1 },
        WarpData { x: 13, y: 19, dest_map: MapId::Route34, dest_x: 8, dest_y: 1 },
        WarpData { x: 14, y: 19, dest_map: MapId::Route34, dest_x: 9, dest_y: 1 },
        // Gym door
        WarpData { x: 2, y: 11, dest_map: MapId::GoldenrodGym, dest_x: 5, dest_y: 8 },
        // Pokemon Center door
        WarpData { x: 10, y: 14, dest_map: MapId::PokemonCenter, dest_x: 4, dest_y: 6 },
        // Dept Store door → GenericHouse (mart)
        WarpData { x: 11, y: 8, dest_map: MapId::GenericHouse, dest_x: 3, dest_y: 5 },
    ];

    let npcs = vec![
        // Flower Shop lady
        NpcDef {
            x: 11, y: 16, sprite_id: 4, facing: Direction::Down,
            dialogue: &[
                "Welcome to the",
                "FLOWER SHOP!",
                "After you beat WHITNEY",
                "come back for a",
                "SQUIRTBOTTLE.",
            ],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
        // Bike Shop owner (sign interaction)
        NpcDef {
            x: 21, y: 15, sprite_id: 3, facing: Direction::Left,
            dialogue: &[
                "I make BICYCLES.",
                "Want one? Just take it!",
                "Ride it everywhere!",
            ],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
        // City guide
        NpcDef {
            x: 8, y: 5, sprite_id: 2, facing: Direction::Right,
            dialogue: &[
                "The DEPT STORE has",
                "everything! WHITNEY's",
                "GYM is famous...",
                "and frustrating.",
                "The GYM is south.",
                "WHITNEY uses NORMAL",
                "type POKEMON.",
            ],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
    ];

    MapData {
        id: MapId::GoldenrodCity,
        name: "GOLDENROD CITY",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters: vec![],
        night_encounters: vec![], water_encounters: vec![],
        music_id: 11,
    }
}

// ─── Goldenrod Gym (10x10) ──────────────────────────────
// Whitney's Normal-type Gym. Clefairy maze pattern.
fn build_goldenrod_gym() -> MapData {
    let w: usize = 10;
    let h: usize = 10;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 2: Whitney's area
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 3
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 4: flower maze
        BLACK,GYM_FLOOR,FLOWER,FLOWER,GYM_FLOOR,GYM_FLOOR,FLOWER,FLOWER,GYM_FLOOR,BLACK,
        // Row 5
        BLACK,GYM_FLOOR,FLOWER,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,FLOWER,GYM_FLOOR,BLACK,
        // Row 6
        BLACK,GYM_FLOOR,FLOWER,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,FLOWER,GYM_FLOOR,BLACK,
        // Row 7
        BLACK,GYM_FLOOR,FLOWER,FLOWER,GYM_FLOOR,GYM_FLOOR,FLOWER,FLOWER,GYM_FLOOR,BLACK,
        // Row 8
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 9: exit
        BLACK,BLACK,BLACK,BLACK,DOOR,DOOR,BLACK,BLACK,BLACK,BLACK,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 2
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 3
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 4: flower walls
        C_SOLID,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_SOLID,
        // Row 5
        C_SOLID,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_SOLID,
        // Row 6
        C_SOLID,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_SOLID,
        // Row 7
        C_SOLID,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_SOLID,
        // Row 8
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 9: exit door
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    let warps = vec![
        WarpData { x: 4, y: 9, dest_map: MapId::GoldenrodCity, dest_x: 2, dest_y: 12 },
        WarpData { x: 5, y: 9, dest_map: MapId::GoldenrodCity, dest_x: 2, dest_y: 12 },
    ];

    let npcs = vec![
        // Whitney - Gym Leader
        NpcDef {
            x: 5, y: 2, sprite_id: 5, facing: Direction::Down,
            dialogue: &[
                "I'm WHITNEY!",
                "Everyone says I'm",
                "pretty NORMAL...",
                "But I'm also pretty",
                "tough! Come on!",
                "Let me show you!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: CLEFAIRY, level: 18 },
                TrainerPokemon { species_id: MILTANK, level: 20 },
            ],
        },
        // Lass Carrie
        NpcDef {
            x: 3, y: 5, sprite_id: 4, facing: Direction::Right,
            dialogue: &[
                "NORMAL type POKEMON",
                "are so cute!",
                "Don't you think so?",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: SNUBBULL, level: 18 },
            ],
        },
        // Lass Bridget
        NpcDef {
            x: 5, y: 6, sprite_id: 4, facing: Direction::Down,
            dialogue: &[
                "JIGGLYPUFF's SING",
                "is a lullaby! Watch!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: JIGGLYPUFF, level: 15 },
                TrainerPokemon { species_id: JIGGLYPUFF, level: 15 },
                TrainerPokemon { species_id: JIGGLYPUFF, level: 15 },
            ],
        },
        // Beauty Samantha
        NpcDef {
            x: 8, y: 3, sprite_id: 4, facing: Direction::Left,
            dialogue: &[
                "My cute MEOWTH will",
                "scratch you to bits!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: MEOWTH, level: 16 },
                TrainerPokemon { species_id: MEOWTH, level: 16 },
            ],
        },
    ];

    MapData {
        id: MapId::GoldenrodGym,
        name: "GOLDENROD GYM",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters: vec![],
        night_encounters: vec![], water_encounters: vec![],
        music_id: 8,
    }
}

// ─── Route 35 (16x20) ───────────────────────────────────
// Connects Goldenrod City (south) to National Park (north).
// S-curve path, lake on west side, grass patches east.
fn build_route_35() -> MapData {
    let w: usize = 16;
    let h: usize = 20;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: north border / warp to National Park
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,PATH,PATH,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,PATH,PATH,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,GRASS,GRASS,PATH,PATH,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 3
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,PATH,PATH,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4
        WATER,WATER,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 5
        WATER,WATER,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 6
        WATER,WATER,WATER,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 7
        WATER,WATER,WATER,GRASS,GRASS,PATH,PATH,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 8
        WATER,WATER,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 9
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 10
        GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 11
        GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 12
        GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 13
        GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 14
        TREE_TOP,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 15
        TREE_BOTTOM,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 16
        TREE_TOP,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 17
        TREE_BOTTOM,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 18
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,GRASS,PATH,PATH,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 19: south border / warp to Goldenrod
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,PATH,PATH,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: north warps
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 3
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 4
        C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 5
        C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 6
        C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 7
        C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 8
        C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 9
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 10
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 11
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 12
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 13
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 14
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 15
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 16
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 17
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 18
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 19: south warps
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "Route35 tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "Route35 collision count mismatch");

    let warps = vec![
        // North exits → National Park (land one tile inside)
        WarpData { x: 6, y: 0, dest_map: MapId::NationalPark, dest_x: 9, dest_y: 15 },
        WarpData { x: 7, y: 0, dest_map: MapId::NationalPark, dest_x: 10, dest_y: 15 },
        // South exits → Goldenrod City (land one tile inside)
        WarpData { x: 6, y: 19, dest_map: MapId::GoldenrodCity, dest_x: 11, dest_y: 1 },
        WarpData { x: 7, y: 19, dest_map: MapId::GoldenrodCity, dest_x: 12, dest_y: 1 },
    ];

    let npcs = vec![
        // Bird Keeper
        NpcDef {
            x: 8, y: 3, sprite_id: 2, facing: Direction::Down,
            dialogue: &["Bird Keeper wants", "to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: PIDGEY, level: 12 },
                TrainerPokemon { species_id: PIDGEY, level: 14 },
            ],
        },
        // Bug Catcher
        NpcDef {
            x: 11, y: 7, sprite_id: 2, facing: Direction::Left,
            dialogue: &["Bug Catcher wants", "to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: CATERPIE, level: 15 },
                TrainerPokemon { species_id: WEEDLE, level: 15 },
            ],
        },
        // Officer
        NpcDef {
            x: 4, y: 13, sprite_id: 3, facing: Direction::Right,
            dialogue: &["Officer wants", "to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: GROWLITHE, level: 14 },
                TrainerPokemon { species_id: GROWLITHE, level: 14 },
            ],
        },
        // Hint NPC about National Park
        NpcDef {
            x: 3, y: 4, sprite_id: 5, facing: Direction::Down,
            dialogue: &[
                "NATIONAL PARK is popular",
                "for the BUG-CATCHING",
                "CONTEST. It's just",
                "north of here!",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: NIDORAN_F, min_level: 12, max_level: 14, weight: 25 },
        EncounterEntry { species_id: NIDORAN_M, min_level: 12, max_level: 14, weight: 25 },
        EncounterEntry { species_id: DROWZEE,   min_level: 13, max_level: 15, weight: 20 },
        EncounterEntry { species_id: PIDGEY,    min_level: 13, max_level: 15, weight: 15 },
        EncounterEntry { species_id: YANMA,     min_level: 12, max_level: 13, weight: 5  },
    ];

    MapData {
        id: MapId::Route35,
        name: "ROUTE 35",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![],
        water_encounters: vec![
            EncounterEntry { species_id: POLIWAG, min_level: 15, max_level: 20, weight: 40 },
            EncounterEntry { species_id: POLIWHIRL, min_level: 18, max_level: 20, weight: 15 },
            EncounterEntry { species_id: MAGIKARP, min_level: 10, max_level: 20, weight: 45 },
        ],
        music_id: 1,
    }
}

// ─── National Park (20x18) ──────────────────────────────
// Large open park. Connects Route 35 (south) to Route 36 (east).
fn build_national_park() -> MapData {
    let w: usize = 20;
    let h: usize = 18;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: north border
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2
        TREE_TOP,TREE_TOP,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 3
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,FLOWER,FLOWER,GRASS,PATH,PATH,GRASS,FLOWER,FLOWER,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 5
        GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,FLOWER,FLOWER,GRASS,GRASS,PATH,PATH,GRASS,GRASS,FLOWER,FLOWER,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,
        // Row 6
        GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,
        // Row 7
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 8: east exit to Route 36
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        // Row 9: east exit to Route 36
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        // Row 10
        GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 11
        GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 12
        GRASS,GRASS,TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,
        // Row 13
        GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,
        // Row 14
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 15
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 16: south border (warp to Route 35)
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,PATH,PATH,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 17: south warp row
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,PATH,PATH,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 3
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 4
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5
        C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,
        // Row 6
        C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,
        // Row 7
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 8: east warp
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,
        // Row 9: east warp
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,
        // Row 10
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 11
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 12
        C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,
        // Row 13
        C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,
        // Row 14
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 15
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 16: south warp
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 17
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "NationalPark tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "NationalPark collision count mismatch");

    let warps = vec![
        // South exits → Route 35 (land one tile inside)
        WarpData { x: 9,  y: 16, dest_map: MapId::Route35, dest_x: 6, dest_y: 1 },
        WarpData { x: 10, y: 16, dest_map: MapId::Route35, dest_x: 7, dest_y: 1 },
        // East exits → Route 36 (land one tile inside)
        WarpData { x: 19, y: 8, dest_map: MapId::Route36, dest_x: 1, dest_y: 6 },
        WarpData { x: 19, y: 9, dest_map: MapId::Route36, dest_x: 1, dest_y: 7 },
    ];

    let npcs = vec![
        // Park info NPC
        NpcDef {
            x: 9, y: 7, sprite_id: 5, facing: Direction::Down,
            dialogue: &[
                "Welcome to",
                "NATIONAL PARK!",
                "Many rare Bug POKeMON",
                "live here.",
            ],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
        // Pokefan trainer
        NpcDef {
            x: 14, y: 11, sprite_id: 4, facing: Direction::Left,
            dialogue: &["Pokefan wants", "to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: SENTRET, level: 14 },
            ],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: CATERPIE, min_level: 10, max_level: 13, weight: 30 },
        EncounterEntry { species_id: WEEDLE,   min_level: 10, max_level: 13, weight: 30 },
        EncounterEntry { species_id: PIDGEY,   min_level: 12, max_level: 14, weight: 25 },
        EncounterEntry { species_id: HOPPIP,   min_level: 12, max_level: 14, weight: 15 },
    ];

    MapData {
        id: MapId::NationalPark,
        name: "NATIONAL PARK",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![], water_encounters: vec![],
        music_id: 5,
    }
}

// ─── Route 36 (20x14) ──────────────────────────────────
// Connects National Park (west) to Route 37 (east). Sudowoodo roadblock.
fn build_route_36() -> MapData {
    let w: usize = 20;
    let h: usize = 14;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: north border
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2
        TREE_TOP,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TREE_TOP,
        // Row 3
        TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,TREE_BOTTOM,
        // Row 4
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,
        // Row 5
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 6: west warp row
        PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        // Row 7: west warp row
        PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        // Row 8
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 9
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,PATH,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 10
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 11
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 12
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 13: south border
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 3
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_SOLID,
        // Row 4
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,
        // Row 5
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 6: west warp + east warp
        C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,
        // Row 7: west warp + east warp
        C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,
        // Row 8
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 9
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 10
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 11
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 12
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 13
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "Route36 tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "Route36 collision count mismatch");

    let warps = vec![
        // West exits → National Park (land one tile inside)
        WarpData { x: 0, y: 6, dest_map: MapId::NationalPark, dest_x: 18, dest_y: 8 },
        WarpData { x: 0, y: 7, dest_map: MapId::NationalPark, dest_x: 18, dest_y: 9 },
        // East exits → Route 37 (land one tile inside)
        WarpData { x: 19, y: 6, dest_map: MapId::Route37, dest_x: 1, dest_y: 5 },
        WarpData { x: 19, y: 7, dest_map: MapId::Route37, dest_x: 1, dest_y: 6 },
    ];

    let npcs = vec![
        // NPC mentioning weird tree
        NpcDef {
            x: 12, y: 6, sprite_id: 5, facing: Direction::Left,
            dialogue: &[
                "There's a weird tree",
                "blocking the path.",
                "It wiggles when you",
                "get close... Maybe",
                "water would help?",
            ],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
        // Psychic trainer
        NpcDef {
            x: 4, y: 5, sprite_id: 3, facing: Direction::Right,
            dialogue: &["Psychic wants", "to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: DROWZEE, level: 13 },
                TrainerPokemon { species_id: DROWZEE, level: 15 },
            ],
        },
        // Sudowoodo blocker — NPC index 2, hidden when FLAG_SUDOWOODO set
        NpcDef {
            x: 14, y: 6, sprite_id: 5, facing: Direction::Down,
            dialogue: &[
                "It's a Pokemon!",
                "The weird tree",
                "doesn't like water!",
            ],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: PIDGEY,    min_level: 12, max_level: 14, weight: 30 },
        EncounterEntry { species_id: BELLSPROUT, min_level: 12, max_level: 14, weight: 25 },
        EncounterEntry { species_id: HOPPIP,    min_level: 12, max_level: 14, weight: 20 },
        EncounterEntry { species_id: RATTATA,   min_level: 12, max_level: 14, weight: 15 },
        EncounterEntry { species_id: STANTLER,  min_level: 13, max_level: 14, weight: 10 },
    ];

    MapData {
        id: MapId::Route36,
        name: "ROUTE 36",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![], water_encounters: vec![],
        music_id: 1,
    }
}

// ─── Route 37 (16x14) ──────────────────────────────────
// Short route connecting Route 36 (west) to Ecruteak City (east, placeholder).
fn build_route_37() -> MapData {
    let w: usize = 16;
    let h: usize = 14;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: north border
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1
        TREE_BOTTOM,TREE_BOTTOM,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2
        TREE_TOP,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,TREE_TOP,TREE_TOP,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TREE_TOP,
        // Row 3
        TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,
        // Row 4
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 5: west warp row
        PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,PATH,PATH,
        // Row 6: west warp row
        PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,PATH,PATH,
        // Row 7
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 8
        GRASS,GRASS,TREE_TOP,TREE_TOP,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,
        // Row 9
        GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,
        // Row 10
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 11
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 12
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 13: south border
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_SOLID,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 3
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 4
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5: west warp + east warp
        C_WARP,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WARP,
        // Row 6: west warp + east warp
        C_WARP,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WARP,
        // Row 7
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 8
        C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 9
        C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 10
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 11
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 12
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 13
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "Route37 tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "Route37 collision count mismatch");

    let warps = vec![
        // West exits → Route 36 (land one tile inside)
        WarpData { x: 0, y: 5, dest_map: MapId::Route36, dest_x: 18, dest_y: 6 },
        WarpData { x: 0, y: 6, dest_map: MapId::Route36, dest_x: 18, dest_y: 7 },
        // East exits → Ecruteak City (land at x=1, one tile inside the single warp column)
        WarpData { x: 15, y: 5, dest_map: MapId::EcruteakCity, dest_x: 1, dest_y: 8 },
        WarpData { x: 15, y: 6, dest_map: MapId::EcruteakCity, dest_x: 1, dest_y: 9 },
    ];

    let npcs = vec![
        // Psychic trainer
        NpcDef {
            x: 7, y: 6, sprite_id: 3, facing: Direction::Down,
            dialogue: &["Psychic wants", "to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: DROWZEE, level: 17 },
            ],
        },
        // Hint NPC about Ecruteak
        NpcDef {
            x: 12, y: 5, sprite_id: 5, facing: Direction::Left,
            dialogue: &[
                "ECRUTEAK CITY has",
                "ancient legends about",
                "legendary POKEMON.",
                "The BURNED TOWER and",
                "TIN TOWER are famous!",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: PIDGEY,   min_level: 13, max_level: 15, weight: 25 },
        EncounterEntry { species_id: STANTLER,  min_level: 14, max_level: 16, weight: 20 },
        EncounterEntry { species_id: GROWLITHE, min_level: 14, max_level: 15, weight: 20 },
        EncounterEntry { species_id: VULPIX,    min_level: 14, max_level: 16, weight: 10 },
        EncounterEntry { species_id: HOPPIP,   min_level: 13, max_level: 15, weight: 15 },
        EncounterEntry { species_id: HOOTHOOT,  min_level: 14, max_level: 15, weight: 10 },
    ];

    MapData {
        id: MapId::Route37,
        name: "ROUTE 37",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![], water_encounters: vec![],
        music_id: 1,
    }
}

// ─── Ecruteak City (20x18) ─────────────────────────────
// Historic city known for its Burned Tower and Dance Theater.
// Connections: west to Route 37, east to Route 38 (future), north to Route 42 (future).
// Buildings: Burned Tower, Ecruteak Gym, Pokemon Center, Dance Theater.
fn build_ecruteak_city() -> MapData {
    let w: usize = 20;
    let h: usize = 18;

    let tiles: Vec<u8> = vec![
        // Row 0
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: Burned Tower roof
        TREE_TOP,TREE_TOP,GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 3: Burned Tower wall / Gym roof
        TREE_BOTTOM,TREE_BOTTOM,GRASS,BUILDING_WALL,BUILDING_WALL,BUILDING_WALL,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,BUILDING_WALL,BUILDING_WALL,BUILDING_WALL,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4: Burned Tower door / Gym wall
        GRASS,GRASS,GRASS,BUILDING_WALL,DOOR,BUILDING_WALL,GRASS,GRASS,PATH,PATH,PATH,GRASS,BUILDING_WALL,DOOR,BUILDING_WALL,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 5
        GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,PATH,GRASS,PATH,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 6: paths
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 7
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 8: west exit
        PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,
        // Row 9: west/east exits
        PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,
        // Row 10: Dance Theater roof / PokemonCenter roof
        GRASS,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,PATH,GRASS,POKECENTER_ROOF,POKECENTER_ROOF,POKECENTER_ROOF,GRASS,GRASS,GRASS,
        // Row 11: Dance Theater wall / PokemonCenter wall
        GRASS,GRASS,GRASS,BUILDING_WALL,BUILDING_WALL,BUILDING_WALL,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,PATH,GRASS,POKECENTER_WALL,POKECENTER_WALL,POKECENTER_WALL,GRASS,GRASS,GRASS,
        // Row 12: Dance Theater door / PokemonCenter door
        GRASS,GRASS,GRASS,BUILDING_WALL,DOOR,BUILDING_WALL,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,GRASS,POKECENTER_WALL,POKECENTER_DOOR,POKECENTER_WALL,GRASS,GRASS,GRASS,
        // Row 13
        GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        // Row 14
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,
        // Row 15
        TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,SIGN,GRASS,GRASS,GRASS,GRASS,SIGN,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,
        // Row 16
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 17
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
    ];

    let collision: Vec<u8> = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_SOLID,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 3
        C_SOLID,C_SOLID,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 4: Burned Tower door at (4,4), Gym door at (13,4)
        C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 6
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 7
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 8: west exit warp at x=0 only, east exit warps at x=18,19
        C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WARP,
        // Row 9
        C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WARP,
        // Row 10
        C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,
        // Row 11
        C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,
        // Row 12: Dance Theater door at (4,12), PokemonCenter door at (15,12)
        C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_SOLID,C_WALK,C_WALK,C_WALK,
        // Row 13
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 14
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 15
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SIGN,C_WALK,C_WALK,C_WALK,C_WALK,C_SIGN,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 16: south exit to Route 42 at x=9,10
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 17
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "EcruteakCity tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "EcruteakCity collision count mismatch");

    let warps = vec![
        // West exit → Route 37 (single column of warps at x=0)
        WarpData { x: 0, y: 8, dest_map: MapId::Route37, dest_x: 14, dest_y: 5 },
        WarpData { x: 0, y: 9, dest_map: MapId::Route37, dest_x: 14, dest_y: 6 },
        // East exit → Route 38
        WarpData { x: 18, y: 8, dest_map: MapId::Route38, dest_x: 1, dest_y: 4 },
        WarpData { x: 18, y: 9, dest_map: MapId::Route38, dest_x: 1, dest_y: 5 },
        WarpData { x: 19, y: 8, dest_map: MapId::Route38, dest_x: 1, dest_y: 4 },
        WarpData { x: 19, y: 9, dest_map: MapId::Route38, dest_x: 1, dest_y: 5 },
        // South exit → Route 42
        WarpData { x: 9, y: 16, dest_map: MapId::Route42, dest_x: 1, dest_y: 2 },
        WarpData { x: 10, y: 16, dest_map: MapId::Route42, dest_x: 2, dest_y: 2 },
        // Burned Tower entrance (4,4) — land one tile above exit warp
        WarpData { x: 4, y: 4, dest_map: MapId::BurnedTower, dest_x: 7, dest_y: 11 },
        // Ecruteak Gym entrance (13,4) — land one tile above exit warp
        WarpData { x: 13, y: 4, dest_map: MapId::EcruteakGym, dest_x: 5, dest_y: 7 },
        // Dance Theater → GenericHouse (4,12)
        WarpData { x: 4, y: 12, dest_map: MapId::GenericHouse, dest_x: 4, dest_y: 4 },
        // Pokemon Center (15,12)
        WarpData { x: 15, y: 12, dest_map: MapId::PokemonCenter, dest_x: 4, dest_y: 6 },
    ];

    let npcs = vec![
        // Old man near Burned Tower
        NpcDef {
            x: 6, y: 3, sprite_id: 5, facing: Direction::Down,
            dialogue: &[
                "The BURNED TOWER and",
                "TIN TOWER hold ancient",
                "secrets. Legends say",
                "HO-OH once perched",
                "atop TIN TOWER!",
            ],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
        // Kimono Girl NPC
        NpcDef {
            x: 4, y: 13, sprite_id: 4, facing: Direction::Up,
            dialogue: &[
                "The DANCE THEATER",
                "features the",
                "KIMONO GIRLS!",
            ],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
        // Sign text guide
        NpcDef {
            x: 10, y: 14, sprite_id: 3, facing: Direction::Down,
            dialogue: &[
                "ECRUTEAK CITY",
                "A Historical and",
                "Mystical City",
            ],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
    ];

    MapData {
        id: MapId::EcruteakCity,
        name: "ECRUTEAK CITY",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters: vec![],
        night_encounters: vec![], water_encounters: vec![],
        music_id: 5,
    }
}

// ─── Burned Tower (14x14) ──────────────────────────────
// Ruined interior. Wild Rattata, Koffing, Zubat. Legendary beast encounter (visual only).
// Eusine and rival appear here. Floor has holes.
fn build_burned_tower() -> MapData {
    let w: usize = 14;
    let h: usize = 14;

    let tiles: Vec<u8> = vec![
        // Row 0
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 2
        BLACK,FLOOR,BLACK,BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,BLACK,FLOOR,BLACK,
        // Row 3
        BLACK,FLOOR,BLACK,BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,BLACK,FLOOR,BLACK,
        // Row 4
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 5
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,FLOOR,FLOOR,BLACK,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 6: center area with legendary beast spots
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 7
        BLACK,FLOOR,FLOOR,FLOOR,BLACK,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 8
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 9
        BLACK,FLOOR,BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,FLOOR,BLACK,
        // Row 10
        BLACK,FLOOR,BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,FLOOR,BLACK,
        // Row 11
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 12
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 13: exit
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];

    let collision: Vec<u8> = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 2
        C_SOLID,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_SOLID,
        // Row 3
        C_SOLID,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_SOLID,
        // Row 4
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 5: holes (solid)
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 7: more holes
        C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 8
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 9
        C_SOLID,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_SOLID,
        // Row 10
        C_SOLID,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_SOLID,
        // Row 11
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 12: exit at bottom
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 13
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "BurnedTower tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "BurnedTower collision count mismatch");

    let warps = vec![
        // Exit → Ecruteak City
        WarpData { x: 7, y: 12, dest_map: MapId::EcruteakCity, dest_x: 4, dest_y: 5 },
    ];

    let npcs = vec![
        // Eusine
        NpcDef {
            x: 7, y: 4, sprite_id: 3, facing: Direction::Down,
            dialogue: &[
                "I'm EUSINE.",
                "I've been chasing",
                "SUICUNE for ten",
                "years...",
                "It's said the three",
                "legendary beasts",
                "rest in this tower.",
            ],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
        // Rival (sprite_id 2 = Youngster placeholder until rival sprite is added)
        NpcDef {
            x: 5, y: 6, sprite_id: 2, facing: Direction::Right,
            dialogue: &[
                "...What are YOU",
                "doing here?",
                "The legendary POKe-",
                "MON are mine!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: GASTLY, level: 20 },
                TrainerPokemon { species_id: ZUBAT, level: 20 },
            ],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: KOFFING, min_level: 14, max_level: 16, weight: 35 },
        EncounterEntry { species_id: RATTATA, min_level: 13, max_level: 15, weight: 30 },
        EncounterEntry { species_id: ZUBAT, min_level: 14, max_level: 14, weight: 15 },
        EncounterEntry { species_id: RATICATE, min_level: 15, max_level: 15, weight: 10 },
        EncounterEntry { species_id: MAGMAR, min_level: 14, max_level: 16, weight: 10 },
    ];

    MapData {
        id: MapId::BurnedTower,
        name: "BURNED TOWER",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
        night_encounters: vec![], water_encounters: vec![],
        music_id: 9,
    }
}

// ─── Ecruteak Gym (10x10) ──────────────────────────────
// Morty's Ghost-type Gym. Dark room with invisible path.
// Morty: Gastly lv21, Haunter lv21, Gengar lv25, Haunter lv23.
// Trainers: Medium (Gastly), Sage (Haunter), Medium (Gastly x2).
fn build_ecruteak_gym() -> MapData {
    let w: usize = 10;
    let h: usize = 10;

    let tiles: Vec<u8> = vec![
        // Row 0
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: Morty at top
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 2
        BLACK,GYM_FLOOR,BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,GYM_FLOOR,BLACK,
        // Row 3
        BLACK,GYM_FLOOR,BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,GYM_FLOOR,BLACK,
        // Row 4
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 5
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 6
        BLACK,GYM_FLOOR,BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,GYM_FLOOR,BLACK,
        // Row 7
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 8
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 9: exit
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];

    let collision: Vec<u8> = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 2: invisible walls
        C_SOLID,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_SOLID,
        // Row 3
        C_SOLID,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_SOLID,
        // Row 4
        C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 5
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6
        C_SOLID,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_SOLID,
        // Row 7
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 8: exit
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 9
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "EcruteakGym tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "EcruteakGym collision count mismatch");

    let warps = vec![
        // Exit → Ecruteak City
        WarpData { x: 5, y: 8, dest_map: MapId::EcruteakCity, dest_x: 13, dest_y: 5 },
    ];

    let npcs = vec![
        // Morty (gym leader, NPC index 0)
        NpcDef {
            x: 5, y: 1, sprite_id: 0, facing: Direction::Down,
            dialogue: &[
                "Gym Leader MORTY",
                "wants to battle!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: GASTLY, level: 21 },
                TrainerPokemon { species_id: HAUNTER, level: 21 },
                TrainerPokemon { species_id: HAUNTER, level: 23 },
                TrainerPokemon { species_id: GENGAR, level: 25 },
            ],
        },
        // Medium Martha
        NpcDef {
            x: 3, y: 3, sprite_id: 4, facing: Direction::Right,
            dialogue: &[
                "Medium MARTHA",
                "wants to battle!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: GASTLY, level: 20 },
                TrainerPokemon { species_id: GASTLY, level: 20 },
            ],
        },
        // Sage Ping
        NpcDef {
            x: 7, y: 5, sprite_id: 5, facing: Direction::Left,
            dialogue: &[
                "Sage PING",
                "wants to battle!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: HAUNTER, level: 22 },
            ],
        },
    ];

    MapData {
        id: MapId::EcruteakGym,
        name: "ECRUTEAK GYM",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters: vec![],
        night_encounters: vec![], water_encounters: vec![],
        music_id: 8,
    }
}

// ─── Route 38 (20×10) ──────────────────────────────────
fn build_route_38() -> MapData {
    let width = 20;
    let height = 10;
    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_TOP,TREE_TOP,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        TREE_BOTTOM,TREE_BOTTOM,TALL_GRASS,TALL_GRASS,TALL_GRASS,PATH,PATH,PATH,TALL_GRASS,TALL_GRASS,TALL_GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,TREE_BOTTOM,TREE_BOTTOM,
        GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,
        TREE_TOP,TREE_TOP,TALL_GRASS,TALL_GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,PATH,PATH,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        TREE_BOTTOM,TREE_BOTTOM,TALL_GRASS,TALL_GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,PATH,PATH,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];
    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,
        C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,
        C_SOLID,C_SOLID,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];
    let warps = vec![
        WarpData { x: 19, y: 4, dest_map: MapId::EcruteakCity, dest_x: 2, dest_y: 8 },
        WarpData { x: 19, y: 5, dest_map: MapId::EcruteakCity, dest_x: 2, dest_y: 9 },
        WarpData { x: 0, y: 4, dest_map: MapId::Route39, dest_x: 8, dest_y: 2 },
        WarpData { x: 0, y: 5, dest_map: MapId::Route39, dest_x: 9, dest_y: 2 },
        WarpData { x: 5, y: 9, dest_map: MapId::Route39, dest_x: 3, dest_y: 2 },
        WarpData { x: 6, y: 9, dest_map: MapId::Route39, dest_x: 4, dest_y: 2 },
        WarpData { x: 11, y: 9, dest_map: MapId::Route39, dest_x: 6, dest_y: 2 },
        WarpData { x: 12, y: 9, dest_map: MapId::Route39, dest_x: 7, dest_y: 2 },
    ];
    let npcs = vec![
        NpcDef { x: 4, y: 3, sprite_id: 2, facing: Direction::Right,
            dialogue: &["I'm raising my DODUO", "to be the best!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: DODUO, level: 15 }, TrainerPokemon { species_id: DODUO, level: 16 }, TrainerPokemon { species_id: DODUO, level: 17 }],
        },
        NpcDef { x: 14, y: 6, sprite_id: 1, facing: Direction::Left,
            dialogue: &["The sea breeze from", "Olivine is great!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: WOOPER, level: 19 }],
        },
        NpcDef { x: 9, y: 3, sprite_id: 3, facing: Direction::Down,
            dialogue: &["My FLAAFFY evolved", "from MAREEP!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: FLAAFFY, level: 18 }, TrainerPokemon { species_id: PSYDUCK, level: 18 }],
        },
        NpcDef { x: 15, y: 7, sprite_id: 3, facing: Direction::Up,
            dialogue: &["Aren't flower Pokemon", "just beautiful?"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: HOPPIP, level: 17 }, TrainerPokemon { species_id: SKIPLOOM, level: 17 }],
        },
    ];
    let encounters = vec![
        EncounterEntry { species_id: RATICATE, min_level: 16, max_level: 16, weight: 30 },
        EncounterEntry { species_id: RATTATA, min_level: 16, max_level: 16, weight: 20 },
        EncounterEntry { species_id: MAGNEMITE, min_level: 16, max_level: 16, weight: 20 },
        EncounterEntry { species_id: PIDGEOTTO, min_level: 16, max_level: 16, weight: 10 },
        EncounterEntry { species_id: MILTANK, min_level: 13, max_level: 13, weight: 10 },
        EncounterEntry { species_id: TAUROS, min_level: 13, max_level: 13, weight: 5 },
        EncounterEntry { species_id: FARFETCHD, min_level: 16, max_level: 16, weight: 5 },
    ];
    MapData { id: MapId::Route38, name: "ROUTE 38", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![
        EncounterEntry { species_id: NOCTOWL, min_level: 16, max_level: 18, weight: 30 },
        EncounterEntry { species_id: RATTATA, min_level: 14, max_level: 16, weight: 25 },
        EncounterEntry { species_id: GASTLY, min_level: 14, max_level: 16, weight: 20 },
        EncounterEntry { species_id: SPINARAK, min_level: 14, max_level: 16, weight: 15 },
        EncounterEntry { species_id: MEOWTH, min_level: 15, max_level: 16, weight: 10 },
    ], water_encounters: vec![], music_id: 2 }
}

// ─── Route 39 (12×18) ──────────────────────────────────
fn build_route_39() -> MapData {
    let width = 12;
    let height = 18;
    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        TREE_TOP,TREE_TOP,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,TREE_TOP,TREE_TOP,
        TREE_BOTTOM,TREE_BOTTOM,GRASS,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        FENCE_H,FENCE_H,GRASS,BUILDING_WALL,DOOR,GRASS,GRASS,BUILDING_WALL,DOOR,GRASS,FENCE_H,FENCE_H,
        FENCE_H,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,FENCE_H,
        FENCE_H,TALL_GRASS,TALL_GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,FENCE_H,
        FENCE_H,FENCE_H,FENCE_H,GRASS,GRASS,PATH,PATH,GRASS,GRASS,FENCE_H,FENCE_H,FENCE_H,
        TREE_TOP,TREE_TOP,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,TREE_TOP,TREE_TOP,
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        TREE_TOP,GRASS,GRASS,TALL_GRASS,TALL_GRASS,PATH,PATH,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TREE_TOP,
        TREE_BOTTOM,GRASS,GRASS,TALL_GRASS,TALL_GRASS,PATH,PATH,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TREE_BOTTOM,
        TREE_TOP,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,TREE_TOP,
        TREE_BOTTOM,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,TREE_BOTTOM,
        TREE_TOP,TREE_TOP,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,TREE_TOP,TREE_TOP,
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];
    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_WALK,C_WALK,C_WALK,C_WARP,C_WARP,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_SOLID,C_WARP,C_WALK,C_WALK,C_SOLID,C_WARP,C_WALK,C_SOLID,C_SOLID,
        C_SOLID,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_SOLID,
        C_SOLID,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_WARP,C_WARP,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,
    ];
    let warps = vec![
        WarpData { x: 3, y: 1, dest_map: MapId::Route38, dest_x: 5, dest_y: 8 },
        WarpData { x: 4, y: 1, dest_map: MapId::Route38, dest_x: 6, dest_y: 8 },
        WarpData { x: 8, y: 1, dest_map: MapId::Route38, dest_x: 11, dest_y: 8 },
        WarpData { x: 9, y: 1, dest_map: MapId::Route38, dest_x: 12, dest_y: 8 },
        WarpData { x: 4, y: 4, dest_map: MapId::GenericHouse, dest_x: 4, dest_y: 4 },
        WarpData { x: 8, y: 4, dest_map: MapId::GenericHouse, dest_x: 4, dest_y: 4 },
        WarpData { x: 3, y: 17, dest_map: MapId::OlivineCity, dest_x: 9, dest_y: 2 },
        WarpData { x: 4, y: 17, dest_map: MapId::OlivineCity, dest_x: 10, dest_y: 2 },
        WarpData { x: 5, y: 17, dest_map: MapId::OlivineCity, dest_x: 11, dest_y: 2 },
        WarpData { x: 6, y: 17, dest_map: MapId::OlivineCity, dest_x: 12, dest_y: 2 },
        WarpData { x: 7, y: 17, dest_map: MapId::OlivineCity, dest_x: 13, dest_y: 2 },
        WarpData { x: 8, y: 17, dest_map: MapId::OlivineCity, dest_x: 14, dest_y: 2 },
    ];
    let npcs = vec![
        NpcDef { x: 3, y: 9, sprite_id: 2, facing: Direction::Right,
            dialogue: &["My SLOWPOKE are", "slow but powerful!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: SLOWPOKE, level: 17 }, TrainerPokemon { species_id: SLOWPOKE, level: 20 }],
        },
        NpcDef { x: 7, y: 12, sprite_id: 1, facing: Direction::Left,
            dialogue: &["My PIKACHU is", "my best friend!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: PIKACHU, level: 17 }],
        },
        NpcDef { x: 5, y: 15, sprite_id: 1, facing: Direction::Up,
            dialogue: &["I'm heading to", "Olivine Port!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: POLIWHIRL, level: 17 }, TrainerPokemon { species_id: RATICATE, level: 17 }, TrainerPokemon { species_id: KRABBY, level: 19 }],
        },
        NpcDef { x: 5, y: 5, sprite_id: 5, facing: Direction::Down,
            dialogue: &["OLIVINE CITY's", "LIGHTHOUSE guides", "ships safely to port.", "It's just south!"],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];
    let encounters = vec![
        EncounterEntry { species_id: RATICATE, min_level: 16, max_level: 17, weight: 30 },
        EncounterEntry { species_id: RATTATA, min_level: 16, max_level: 16, weight: 20 },
        EncounterEntry { species_id: MAGNEMITE, min_level: 16, max_level: 16, weight: 20 },
        EncounterEntry { species_id: PIDGEOTTO, min_level: 16, max_level: 16, weight: 10 },
        EncounterEntry { species_id: MILTANK, min_level: 15, max_level: 15, weight: 10 },
        EncounterEntry { species_id: TAUROS, min_level: 15, max_level: 15, weight: 5 },
        EncounterEntry { species_id: FARFETCHD, min_level: 16, max_level: 16, weight: 5 },
    ];
    MapData { id: MapId::Route39, name: "ROUTE 39", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![
        EncounterEntry { species_id: NOCTOWL, min_level: 16, max_level: 18, weight: 30 },
        EncounterEntry { species_id: RATTATA, min_level: 14, max_level: 16, weight: 25 },
        EncounterEntry { species_id: GASTLY, min_level: 14, max_level: 16, weight: 20 },
        EncounterEntry { species_id: MEOWTH, min_level: 15, max_level: 16, weight: 15 },
        EncounterEntry { species_id: SPINARAK, min_level: 14, max_level: 16, weight: 10 },
    ], water_encounters: vec![], music_id: 2 }
}

// ─── Olivine City (20×18) ──────────────────────────────
fn build_olivine_city() -> MapData {
    let width = 20;
    let height = 18;
    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        TREE_BOTTOM,TREE_BOTTOM,GRASS,BUILDING_ROOF,BUILDING_ROOF,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,BUILDING_ROOF,BUILDING_ROOF,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        GRASS,GRASS,GRASS,BUILDING_WALL,DOOR,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,BUILDING_WALL,DOOR,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,POKECENTER_ROOF,POKECENTER_ROOF,POKECENTER_ROOF,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,POKECENTER_WALL,POKECENTER_WALL,DOOR,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,
        GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,
        GRASS,BUILDING_WALL,BUILDING_WALL,DOOR,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,BUILDING_WALL,BUILDING_WALL,BUILDING_WALL,DOOR,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,BUILDING_WALL,BUILDING_WALL,BUILDING_WALL,DOOR,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
        WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
    ];
    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_WARP,C_WARP,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_SOLID,C_SOLID,
        C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_SOLID,C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,
        C_WALK,C_SOLID,C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
    ];
    let warps = vec![
        WarpData { x: 9, y: 1, dest_map: MapId::Route39, dest_x: 3, dest_y: 16 },
        WarpData { x: 10, y: 1, dest_map: MapId::Route39, dest_x: 4, dest_y: 16 },
        WarpData { x: 11, y: 1, dest_map: MapId::Route39, dest_x: 5, dest_y: 16 },
        WarpData { x: 12, y: 1, dest_map: MapId::Route39, dest_x: 6, dest_y: 16 },
        WarpData { x: 13, y: 1, dest_map: MapId::Route39, dest_x: 7, dest_y: 16 },
        WarpData { x: 14, y: 1, dest_map: MapId::Route39, dest_x: 8, dest_y: 16 },
        // Gym door (4,4) → OlivineGym (land one tile above exit warp)
        WarpData { x: 4, y: 4, dest_map: MapId::OlivineGym, dest_x: 5, dest_y: 7 },
        // House door (16,4) → GenericHouse
        WarpData { x: 16, y: 4, dest_map: MapId::GenericHouse, dest_x: 3, dest_y: 5 },
        // Pokemon Center door (4,7)
        WarpData { x: 4, y: 7, dest_map: MapId::PokemonCenter, dest_x: 4, dest_y: 6 },
        // Cafe door (3,11) → GenericHouse
        WarpData { x: 3, y: 11, dest_map: MapId::GenericHouse, dest_x: 3, dest_y: 5 },
        // Lighthouse door (17,11) → OlivineLighthouse
        WarpData { x: 17, y: 11, dest_map: MapId::OlivineLighthouse, dest_x: 5, dest_y: 9 },
        // South dock exit → Route 40 (both land on walkable tiles x=3,4 in Route40)
        WarpData { x: 5, y: 15, dest_map: MapId::Route40, dest_x: 3, dest_y: 1 },
        WarpData { x: 6, y: 15, dest_map: MapId::Route40, dest_x: 4, dest_y: 1 },
    ];
    let npcs = vec![
        NpcDef { x: 8, y: 2, sprite_id: 5, facing: Direction::Down,
            dialogue: &["OLIVINE CITY", "The Port of Crashing", "Waves. JASMINE tends", "to the sick AMPHAROS", "in the LIGHTHOUSE."],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        NpcDef { x: 9, y: 13, sprite_id: 1, facing: Direction::Down,
            dialogue: &["The S.S. AQUA docks", "here. It sails to", "VERMILION CITY."],
            is_trainer: false, is_mart: false, wanders: true, trainer_team: &[],
        },
        NpcDef { x: 4, y: 10, sprite_id: 3, facing: Direction::Right,
            dialogue: &["JASMINE is the GYM", "LEADER but she's at", "the LIGHTHOUSE now."],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        NpcDef { x: 14, y: 13, sprite_id: 5, facing: Direction::Left,
            dialogue: &["I love fishing off", "the Olivine coast!"],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        NpcDef { x: 17, y: 4, sprite_id: 5, facing: Direction::Down,
            dialogue: &["Welcome to the", "OLIVINE MART!"],
            is_trainer: false, is_mart: true, wanders: false, trainer_team: &[],
        },
    ];
    MapData { id: MapId::OlivineCity, name: "OLIVINE CITY", width, height, tiles, collision, warps, npcs, encounters: vec![], night_encounters: vec![], water_encounters: vec![
        EncounterEntry { species_id: TENTACOOL, min_level: 20, max_level: 24, weight: 40 },
        EncounterEntry { species_id: KRABBY, min_level: 20, max_level: 24, weight: 25 },
        EncounterEntry { species_id: MAGIKARP, min_level: 10, max_level: 20, weight: 35 },
    ], music_id: 3 }
}
// ─── Olivine Lighthouse (10x12) ────────────────────────
// Simplified to single floor. Trainers from floors 3-5, Jasmine+Amphy at top.
fn build_olivine_lighthouse() -> MapData {
    let w: usize = 10;
    let h: usize = 12;
    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];
    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];
    debug_assert_eq!(tiles.len(), w * h);
    debug_assert_eq!(collision.len(), w * h);
    let warps = vec![
        WarpData { x: 5, y: 10, dest_map: MapId::OlivineCity, dest_x: 17, dest_y: 12 },
    ];
    let npcs = vec![
        // Jasmine (non-trainer here, she's at the top with sick Amphy)
        NpcDef {
            x: 4, y: 1, sprite_id: 0, facing: Direction::Down,
            dialogue: &["AMPHY is sick...", "I can't leave its", "side... Please, I", "need MEDICINE from", "CIANWOOD CITY."],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // Amphy (sick Ampharos, represented as NPC)
        NpcDef {
            x: 5, y: 1, sprite_id: 5, facing: Direction::Down,
            dialogue: &["... ... ...", "(AMPHY looks weak)"],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // Sailor Huey (floor 3)
        NpcDef {
            x: 2, y: 4, sprite_id: 2, facing: Direction::Right,
            dialogue: &["Sailor HUEY", "wants to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: POLIWAG, level: 18 },
                TrainerPokemon { species_id: POLIWHIRL, level: 20 },
            ],
        },
        // Gentleman Preston (floor 3)
        NpcDef {
            x: 7, y: 3, sprite_id: 4, facing: Direction::Left,
            dialogue: &["Gentleman PRESTON", "wants to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: GROWLITHE, level: 18 },
                TrainerPokemon { species_id: GROWLITHE, level: 18 },
            ],
        },
        // Lass Connie (floor 4)
        NpcDef {
            x: 3, y: 6, sprite_id: 3, facing: Direction::Down,
            dialogue: &["Lass CONNIE", "wants to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: MARILL, level: 21 },
            ],
        },
        // Sailor Kent (floor 5)
        NpcDef {
            x: 7, y: 7, sprite_id: 2, facing: Direction::Left,
            dialogue: &["Sailor KENT", "wants to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: KRABBY, level: 20 },
                TrainerPokemon { species_id: KRABBY, level: 20 },
            ],
        },
        // Bird Keeper Denis (floor 5)
        NpcDef {
            x: 2, y: 8, sprite_id: 1, facing: Direction::Right,
            dialogue: &["Bird Keeper DENIS", "wants to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: SPEAROW, level: 18 },
                TrainerPokemon { species_id: FEAROW, level: 20 },
                TrainerPokemon { species_id: SPEAROW, level: 18 },
            ],
        },
    ];
    MapData { id: MapId::OlivineLighthouse, name: "LIGHTHOUSE", width: w, height: h, tiles, collision, warps, npcs, encounters: vec![], night_encounters: vec![], water_encounters: vec![], music_id: 5 }
}
// ─── Olivine Gym (10x10) ───────────────────────────────
// Jasmine - Steel type. Available immediately (skip Lighthouse quest).
fn build_olivine_gym() -> MapData {
    let w: usize = 10;
    let h: usize = 10;
    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];
    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];
    debug_assert_eq!(tiles.len(), w * h);
    debug_assert_eq!(collision.len(), w * h);
    let warps = vec![
        WarpData { x: 5, y: 8, dest_map: MapId::OlivineCity, dest_x: 4, dest_y: 5 },
    ];
    let npcs = vec![
        NpcDef {
            x: 5, y: 1, sprite_id: 0, facing: Direction::Down,
            dialogue: &["Gym Leader JASMINE","wants to battle!","...I'm not very","good at this..."],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: MAGNEMITE, level: 30 },
                TrainerPokemon { species_id: MAGNEMITE, level: 30 },
                TrainerPokemon { species_id: STEELIX, level: 35 },
            ],
        },
    ];
    MapData { id: MapId::OlivineGym, name: "OLIVINE GYM", width: w, height: h, tiles, collision, warps, npcs, encounters: vec![], night_encounters: vec![], water_encounters: vec![], music_id: 8 }
}

// ─── Route 40 (8×20) ─── Pier walkway over ocean ──────────
// In the real game this is a surf route. We use a pier/dock path instead.
fn build_route_40() -> MapData {
    let width = 8;
    let height = 20;
    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        WATER,WATER,WATER,PATH,PATH,WATER,WATER,WATER,
        WATER,WATER,WATER,PATH,PATH,WATER,WATER,WATER,
        WATER,WATER,WATER,PATH,PATH,WATER,WATER,WATER,
        WATER,WATER,PATH,PATH,PATH,PATH,WATER,WATER,
        WATER,WATER,PATH,PATH,PATH,PATH,WATER,WATER,
        WATER,WATER,WATER,PATH,PATH,WATER,WATER,WATER,
        WATER,WATER,WATER,PATH,PATH,WATER,WATER,WATER,
        WATER,WATER,WATER,PATH,PATH,WATER,WATER,WATER,
        WATER,WATER,PATH,PATH,PATH,PATH,WATER,WATER,
        WATER,WATER,PATH,PATH,PATH,PATH,WATER,WATER,
        WATER,WATER,WATER,PATH,PATH,WATER,WATER,WATER,
        WATER,WATER,WATER,PATH,PATH,WATER,WATER,WATER,
        WATER,WATER,WATER,PATH,PATH,WATER,WATER,WATER,
        WATER,WATER,PATH,PATH,PATH,PATH,WATER,WATER,
        WATER,WATER,PATH,PATH,PATH,PATH,WATER,WATER,
        WATER,WATER,WATER,PATH,PATH,WATER,WATER,WATER,
        WATER,WATER,WATER,PATH,PATH,WATER,WATER,WATER,
        WATER,WATER,WATER,PATH,PATH,WATER,WATER,WATER,
        WATER,WATER,WATER,PATH,PATH,WATER,WATER,WATER,
        WATER,WATER,WATER,PATH,PATH,WATER,WATER,WATER,
    ];
    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        C_WATER,C_WATER,C_WATER,C_WARP,C_WARP,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WARP,C_WARP,C_WATER,C_WATER,C_WATER,
    ];
    let warps = vec![
        WarpData { x: 3, y: 0, dest_map: MapId::OlivineCity, dest_x: 5, dest_y: 14 },
        WarpData { x: 4, y: 0, dest_map: MapId::OlivineCity, dest_x: 6, dest_y: 14 },
        WarpData { x: 3, y: 19, dest_map: MapId::CianwoodCity, dest_x: 9, dest_y: 1 },
        WarpData { x: 4, y: 19, dest_map: MapId::CianwoodCity, dest_x: 10, dest_y: 1 },
    ];
    let npcs = vec![
        NpcDef { x: 3, y: 4, sprite_id: 2, facing: Direction::Down,
            dialogue: &["Swimmer SIMON", "wants to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: TENTACOOL, level: 20 }, TrainerPokemon { species_id: TENTACOOL, level: 20 }],
        },
        NpcDef { x: 4, y: 9, sprite_id: 1, facing: Direction::Up,
            dialogue: &["Swimmer ELAINE", "wants to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: KRABBY, level: 21 }],
        },
        NpcDef { x: 3, y: 14, sprite_id: 2, facing: Direction::Down,
            dialogue: &["Swimmer RANDALL", "wants to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: TENTACOOL, level: 18 }, TrainerPokemon { species_id: TENTACRUEL, level: 22 }],
        },
    ];
    let encounters = vec![
        EncounterEntry { species_id: TENTACOOL, min_level: 20, max_level: 24, weight: 60 },
        EncounterEntry { species_id: TENTACRUEL, min_level: 24, max_level: 24, weight: 20 },
        EncounterEntry { species_id: KRABBY, min_level: 20, max_level: 22, weight: 20 },
    ];
    MapData { id: MapId::Route40, name: "ROUTE 40", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![], water_encounters: vec![
        EncounterEntry { species_id: TENTACOOL, min_level: 20, max_level: 25, weight: 40 },
        EncounterEntry { species_id: TENTACRUEL, min_level: 23, max_level: 25, weight: 15 },
        EncounterEntry { species_id: MAGIKARP, min_level: 10, max_level: 20, weight: 30 },
        EncounterEntry { species_id: KRABBY, min_level: 20, max_level: 24, weight: 15 },
    ], music_id: 2 }
}

// ─── Cianwood City (20×14) ───────────────────────────────
fn build_cianwood_city() -> MapData {
    let width = 20;
    let height = 14;
    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,GRASS,GRASS,GRASS,GRASS,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
        WATER,WATER,WATER,WATER,WATER,WATER,WATER,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,PATH,PATH,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,POKECENTER_ROOF,POKECENTER_ROOF,POKECENTER_ROOF,
        GRASS,BUILDING_WALL,DOOR,GRASS,GRASS,BUILDING_WALL,DOOR,GRASS,GRASS,PATH,PATH,GRASS,GRASS,BUILDING_WALL,DOOR,GRASS,GRASS,POKECENTER_WALL,POKECENTER_WALL,DOOR,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,PATH,PATH,GRASS,GRASS,PATH,PATH,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,GRASS,GRASS,
        GRASS,GRASS,GRASS,BUILDING_WALL,BUILDING_WALL,DOOR,GRASS,PATH,PATH,GRASS,GRASS,PATH,PATH,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
        WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,
    ];
    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WALK,C_WARP,C_WARP,C_WALK,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        C_WALK,C_SOLID,C_WARP,C_WALK,C_WALK,C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WARP,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
        C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,
    ];
    let warps = vec![
        WarpData { x: 9, y: 0, dest_map: MapId::Route40, dest_x: 3, dest_y: 18 },
        WarpData { x: 10, y: 0, dest_map: MapId::Route40, dest_x: 4, dest_y: 18 },
        WarpData { x: 2, y: 4, dest_map: MapId::CianwoodGym, dest_x: 5, dest_y: 7 },
        WarpData { x: 6, y: 4, dest_map: MapId::GenericHouse, dest_x: 3, dest_y: 5 },
        WarpData { x: 14, y: 4, dest_map: MapId::GenericHouse, dest_x: 3, dest_y: 5 },
        WarpData { x: 19, y: 4, dest_map: MapId::PokemonCenter, dest_x: 4, dest_y: 6 },
        WarpData { x: 5, y: 8, dest_map: MapId::GenericHouse, dest_x: 3, dest_y: 5 },
    ];
    let npcs = vec![
        NpcDef { x: 9, y: 2, sprite_id: 5, facing: Direction::Down,
            dialogue: &["CIANWOOD CITY", "A Port of Winds"],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        NpcDef { x: 8, y: 5, sprite_id: 1, facing: Direction::Right,
            dialogue: &["The PHARMACY has", "rare medicines!", "CHUCK is in the GYM."],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        NpcDef { x: 12, y: 6, sprite_id: 3, facing: Direction::Left,
            dialogue: &["CHUCK trains under", "the waterfall. His", "POLIWRATH is scary!"],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        NpcDef { x: 7, y: 10, sprite_id: 2, facing: Direction::Up,
            dialogue: &["I took the ferry", "from Olivine to get", "here!"],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        NpcDef { x: 15, y: 5, sprite_id: 5, facing: Direction::Down,
            dialogue: &["Welcome to the", "CIANWOOD MART!"],
            is_trainer: false, is_mart: true, wanders: false, trainer_team: &[],
        },
        // Pharmacist (NPC index 5) — gives SecretPotion
        NpcDef { x: 11, y: 6, sprite_id: 5, facing: Direction::Down,
            dialogue: &["This is the CIANWOOD", "PHARMACY. We have", "rare medicines."],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];
    MapData { id: MapId::CianwoodCity, name: "CIANWOOD CITY", width, height, tiles, collision, warps, npcs, encounters: vec![], night_encounters: vec![], water_encounters: vec![
        EncounterEntry { species_id: TENTACOOL, min_level: 20, max_level: 24, weight: 40 },
        EncounterEntry { species_id: KRABBY, min_level: 20, max_level: 24, weight: 25 },
        EncounterEntry { species_id: MAGIKARP, min_level: 10, max_level: 20, weight: 35 },
    ], music_id: 3 }
}

// ─── Cianwood Gym (10x10) ───────────────────────────────
// Chuck — Fighting type. Storm Badge.
fn build_cianwood_gym() -> MapData {
    let w: usize = 10;
    let h: usize = 10;
    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];
    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];
    debug_assert_eq!(tiles.len(), w * h);
    debug_assert_eq!(collision.len(), w * h);
    let warps = vec![
        WarpData { x: 5, y: 8, dest_map: MapId::CianwoodCity, dest_x: 2, dest_y: 5 },
    ];
    let npcs = vec![
        NpcDef {
            x: 5, y: 1, sprite_id: 0, facing: Direction::Down,
            dialogue: &["Gym Leader CHUCK","wants to battle!","WAHAHAHA! Let me","show you my power!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: PRIMEAPE, level: 27 },
                TrainerPokemon { species_id: POLIWRATH, level: 30 },
            ],
        },
        NpcDef {
            x: 2, y: 4, sprite_id: 2, facing: Direction::Right,
            dialogue: &["Blackbelt YOSHI", "wants to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: MACHOKE, level: 25 }],
        },
        NpcDef {
            x: 7, y: 5, sprite_id: 2, facing: Direction::Left,
            dialogue: &["Blackbelt LAO", "wants to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: MACHOKE, level: 23 }, TrainerPokemon { species_id: MACHOKE, level: 23 }],
        },
    ];
    MapData { id: MapId::CianwoodGym, name: "CIANWOOD GYM", width: w, height: h, tiles, collision, warps, npcs, encounters: vec![], night_encounters: vec![], water_encounters: vec![], music_id: 8 }
}

// ─── Route 42 (20×14) ──────────────────────────────────
// Connects Ecruteak City (north) to Mahogany Town (east). Mountain pass with cave (skipped).
fn build_route_42() -> MapData {
    let width: usize = 20;
    let height: usize = 14;
    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: tree border
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: west entrance from Ecruteak
        TREE_BOTTOM,PATH,PATH,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2
        GRASS,PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 3
        GRASS,GRASS,PATH,PATH,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 4
        GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 5: mountain/cave entrance area
        TREE_TOP,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 6: cave wall (solid) / path continues south
        TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 7: path winds east
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 8
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 9
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,
        // Row 10: continuing east
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        // Row 11
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,
        // Row 12
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 13: tree border
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
    ];
    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: west entrance
        C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 2
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 3
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 4
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 6
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 7
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 8
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 9: east exit at (18,9) (19,9)
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WARP,
        // Row 10: east exit at (18,10) (19,10)
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WARP,
        // Row 11
        C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 12
        C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 13
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];
    let warps = vec![
        // West entrance → Ecruteak City south exit
        WarpData { x: 1, y: 1, dest_map: MapId::EcruteakCity, dest_x: 9, dest_y: 15 },
        WarpData { x: 2, y: 1, dest_map: MapId::EcruteakCity, dest_x: 10, dest_y: 15 },
        // East exit → Mahogany Town west entrance
        WarpData { x: 18, y: 9, dest_map: MapId::MahoganyTown, dest_x: 2, dest_y: 8 },
        WarpData { x: 19, y: 9, dest_map: MapId::MahoganyTown, dest_x: 2, dest_y: 8 },
        WarpData { x: 18, y: 10, dest_map: MapId::MahoganyTown, dest_x: 2, dest_y: 9 },
        WarpData { x: 19, y: 10, dest_map: MapId::MahoganyTown, dest_x: 2, dest_y: 9 },
    ];
    let npcs = vec![
        NpcDef {
            x: 10, y: 4, sprite_id: 5, facing: Direction::Left,
            dialogue: &["MAHOGANY TOWN is", "east of here. I hear", "strange things are", "happening at the", "LAKE OF RAGE up north."],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // Fisher trainer
        NpcDef {
            x: 14, y: 7, sprite_id: 3, facing: Direction::Down,
            dialogue: &["I love fishing in", "the mountain streams!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: GOLDEEN, level: 22 }, TrainerPokemon { species_id: POLIWHIRL, level: 22 }],
        },
        // Hiker trainer
        NpcDef {
            x: 4, y: 8, sprite_id: 4, facing: Direction::Right,
            dialogue: &["The mountains are", "tough terrain!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: GEODUDE, level: 23 }, TrainerPokemon { species_id: MACHOP, level: 23 }],
        },
    ];
    let encounters = vec![
        EncounterEntry { species_id: MANKEY, min_level: 13, max_level: 15, weight: 30 },
        EncounterEntry { species_id: SPEAROW, min_level: 13, max_level: 15, weight: 20 },
        EncounterEntry { species_id: RATTATA, min_level: 13, max_level: 14, weight: 20 },
        EncounterEntry { species_id: GOLBAT, min_level: 16, max_level: 17, weight: 10 },
        EncounterEntry { species_id: ZUBAT, min_level: 13, max_level: 15, weight: 10 },
        EncounterEntry { species_id: MAREEP, min_level: 13, max_level: 15, weight: 10 },
    ];
    MapData { id: MapId::Route42, name: "ROUTE 42", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![], water_encounters: vec![], music_id: 2 }
}

// ─── Mahogany Town (16×14) ──────────────────────────────
// Small town between Route 42 (west) and Route 43/44. Gym, PokemonCenter, Mart.
fn build_mahogany_town() -> MapData {
    let w: usize = 16;
    let h: usize = 14;
    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: tree border
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: gym roof
        TREE_TOP,TREE_TOP,GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 3: gym wall
        TREE_BOTTOM,TREE_BOTTOM,GRASS,BUILDING_WALL,BUILDING_WALL,DOOR,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 5: mart roof / house roof
        GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,GRASS,
        // Row 6: mart wall / house wall
        GRASS,GRASS,BUILDING_WALL,DOOR,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,BUILDING_WALL,DOOR,GRASS,GRASS,GRASS,
        // Row 7: main path
        GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,
        // Row 8: west/east exits
        PATH,PATH,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,PATH,PATH,
        // Row 9
        PATH,PATH,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,PATH,PATH,
        // Row 10: pokecenter
        GRASS,GRASS,GRASS,POKECENTER_ROOF,POKECENTER_ROOF,POKECENTER_ROOF,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        // Row 11
        GRASS,GRASS,GRASS,POKECENTER_WALL,POKECENTER_WALL,POKECENTER_DOOR,GRASS,GRASS,SIGN,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,
        // Row 12
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,
        // Row 13: tree border
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
    ];
    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_WALK,C_WALK,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_SOLID,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 3: gym door at (5,3)
        C_SOLID,C_SOLID,C_WALK,C_SOLID,C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 4
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5
        C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,
        // Row 6: mart door (3,6), house door (12,6)
        C_WALK,C_WALK,C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,
        // Row 7
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 8: west exit (0,8)(1,8) east exit (14,8)(15,8)
        C_WARP,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WARP,
        // Row 9: west exit (0,9)(1,9) east exit (14,9)(15,9)
        C_WARP,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WARP,
        // Row 10
        C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 11: pokecenter door (5,11)
        C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WARP,C_WALK,C_WALK,C_SIGN,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 12
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 13
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];
    let warps = vec![
        // West exit → Route 42
        WarpData { x: 0, y: 8, dest_map: MapId::Route42, dest_x: 17, dest_y: 9 },
        WarpData { x: 1, y: 8, dest_map: MapId::Route42, dest_x: 17, dest_y: 9 },
        WarpData { x: 0, y: 9, dest_map: MapId::Route42, dest_x: 17, dest_y: 10 },
        WarpData { x: 1, y: 9, dest_map: MapId::Route42, dest_x: 17, dest_y: 10 },
        // North exit → Route 43
        WarpData { x: 5, y: 1, dest_map: MapId::Route43, dest_x: 5, dest_y: 17 },
        WarpData { x: 6, y: 1, dest_map: MapId::Route43, dest_x: 6, dest_y: 17 },
        WarpData { x: 9, y: 1, dest_map: MapId::Route43, dest_x: 5, dest_y: 17 },
        WarpData { x: 10, y: 1, dest_map: MapId::Route43, dest_x: 6, dest_y: 17 },
        // East exit → Route 44
        WarpData { x: 14, y: 8, dest_map: MapId::Route44, dest_x: 2, dest_y: 5 },
        WarpData { x: 15, y: 8, dest_map: MapId::Route44, dest_x: 2, dest_y: 5 },
        WarpData { x: 14, y: 9, dest_map: MapId::Route44, dest_x: 2, dest_y: 6 },
        WarpData { x: 15, y: 9, dest_map: MapId::Route44, dest_x: 2, dest_y: 6 },
        // Gym door (5,3)
        WarpData { x: 5, y: 3, dest_map: MapId::MahoganyGym, dest_x: 5, dest_y: 7 },
        // "Mart" (3,6) → Rocket HQ (souvenir shop is the front for Team Rocket)
        WarpData { x: 3, y: 6, dest_map: MapId::RocketHQ, dest_x: 5, dest_y: 9 },
        // House (12,6) → GenericHouse
        WarpData { x: 12, y: 6, dest_map: MapId::GenericHouse, dest_x: 4, dest_y: 4 },
        // PokemonCenter (5,11)
        WarpData { x: 5, y: 11, dest_map: MapId::PokemonCenter, dest_x: 4, dest_y: 6 },
    ];
    let npcs = vec![
        NpcDef {
            x: 7, y: 4, sprite_id: 5, facing: Direction::Down,
            dialogue: &["MAHOGANY TOWN", "is known for its", "old-fashioned feel.", "Home of the ICE gym!"],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        NpcDef {
            x: 3, y: 7, sprite_id: 2, facing: Direction::Right,
            dialogue: &["Something suspicious", "about that shop...", "I wouldn't go in", "if I were you."],
            is_trainer: false, is_mart: true, wanders: false, trainer_team: &[],
        },
        NpcDef {
            x: 10, y: 12, sprite_id: 5, facing: Direction::Up,
            dialogue: &["LAKE OF RAGE is", "north of here.", "Strange things have", "been happening there."],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];
    MapData { id: MapId::MahoganyTown, name: "MAHOGANY TOWN", width: w, height: h, tiles, collision, warps, npcs, encounters: vec![], night_encounters: vec![], water_encounters: vec![], music_id: 1 }
}

// ─── Mahogany Gym (10×10) ───────────────────────────────
// Pryce — Ice-type. Glacier Badge (#7).
fn build_mahogany_gym() -> MapData {
    let w: usize = 10;
    let h: usize = 10;
    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: Pryce
        BLACK,BLACK,BLACK,BLACK,ICE_FLOOR,ICE_FLOOR,BLACK,BLACK,BLACK,BLACK,
        // Row 2
        BLACK,BLACK,BLACK,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,BLACK,BLACK,BLACK,
        // Row 3: ice floor
        BLACK,BLACK,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,BLACK,BLACK,
        // Row 4
        BLACK,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,BLACK,
        // Row 5: trainers
        BLACK,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,BLACK,
        // Row 6
        BLACK,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,BLACK,
        // Row 7
        BLACK,BLACK,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,BLACK,BLACK,
        // Row 8: entrance
        BLACK,BLACK,BLACK,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,BLACK,BLACK,BLACK,
        // Row 9
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];
    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: Pryce at (5,1)
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 3
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 4
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 5
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 7
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 8: exit warp at (5,8)
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WARP,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 9
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];
    let warps = vec![
        // Exit → Mahogany Town (in front of gym door)
        WarpData { x: 5, y: 8, dest_map: MapId::MahoganyTown, dest_x: 5, dest_y: 4 },
    ];
    let npcs = vec![
        // Pryce — gym leader (NPC #0)
        NpcDef {
            x: 5, y: 1, sprite_id: 5, facing: Direction::Down,
            dialogue: &["I am PRYCE, the", "master of ICE.", "My Pokemon and I", "have trained for", "decades. Prepare", "for a chilling", "battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: SEEL, level: 27 },
                TrainerPokemon { species_id: DEWGONG, level: 29 },
                TrainerPokemon { species_id: PILOSWINE, level: 31 },
            ],
        },
        // Skier trainer
        NpcDef {
            x: 2, y: 5, sprite_id: 3, facing: Direction::Right,
            dialogue: &["Ice is beautiful", "and deadly!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: SWINUB, level: 24 },
                TrainerPokemon { species_id: DEWGONG, level: 24 },
            ],
        },
        // Boarder trainer
        NpcDef {
            x: 7, y: 4, sprite_id: 4, facing: Direction::Left,
            dialogue: &["The cold doesn't", "bother us at all!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: SEEL, level: 25 },
                TrainerPokemon { species_id: SWINUB, level: 25 },
            ],
        },
    ];
    MapData { id: MapId::MahoganyGym, name: "MAHOGANY GYM", width: w, height: h, tiles, collision, warps, npcs, encounters: vec![], night_encounters: vec![], water_encounters: vec![], music_id: 8 }
}

// ─── Route 43 (12×20) ──────────────────────────────────
// Connects Mahogany Town (south) to Lake of Rage (north). Grass route with trainers.
fn build_route_43() -> MapData {
    let width: usize = 12;
    let height: usize = 20;
    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: tree border
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: north exit to Lake of Rage
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,PATH,PATH,PATH,PATH,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2
        TREE_TOP,TREE_TOP,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 3
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,PATH,GRASS,GRASS,PATH,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,PATH,GRASS,GRASS,PATH,TALL_GRASS,TALL_GRASS,GRASS,GRASS,
        // Row 5
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,PATH,GRASS,GRASS,PATH,TALL_GRASS,TALL_GRASS,GRASS,GRASS,
        // Row 6: gatehouse area
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,
        // Row 7
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 8
        GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,
        // Row 9
        GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,
        // Row 10
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,
        // Row 11
        GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        // Row 12: trees narrow the path
        TREE_TOP,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,TREE_TOP,
        // Row 13
        TREE_BOTTOM,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,TREE_BOTTOM,
        // Row 14
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,PATH,GRASS,GRASS,PATH,TALL_GRASS,TALL_GRASS,GRASS,GRASS,
        // Row 15
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,PATH,GRASS,GRASS,PATH,TALL_GRASS,TALL_GRASS,GRASS,GRASS,
        // Row 16
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,
        // Row 17
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 18: south exit to Mahogany Town
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,PATH,PATH,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 19: tree border
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
    ];
    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: north exit
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 3
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 4
        C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,
        // Row 5
        C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,
        // Row 6
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 7
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 8
        C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,
        // Row 9
        C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,
        // Row 10
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 11
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 12
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 13
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 14
        C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,
        // Row 15
        C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,
        // Row 16
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 17
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 18: south exit
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WARP,C_WARP,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 19
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];
    let warps = vec![
        // North exit → Lake of Rage
        WarpData { x: 4, y: 1, dest_map: MapId::LakeOfRage, dest_x: 7, dest_y: 11 },
        WarpData { x: 5, y: 1, dest_map: MapId::LakeOfRage, dest_x: 8, dest_y: 11 },
        WarpData { x: 6, y: 1, dest_map: MapId::LakeOfRage, dest_x: 7, dest_y: 11 },
        WarpData { x: 7, y: 1, dest_map: MapId::LakeOfRage, dest_x: 8, dest_y: 11 },
        // South exit → Mahogany Town north
        WarpData { x: 5, y: 18, dest_map: MapId::MahoganyTown, dest_x: 7, dest_y: 4 },
        WarpData { x: 6, y: 18, dest_map: MapId::MahoganyTown, dest_x: 8, dest_y: 4 },
    ];
    let npcs = vec![
        // Camper trainer
        NpcDef {
            x: 3, y: 6, sprite_id: 3, facing: Direction::Right,
            dialogue: &["I came all the way", "from MAHOGANY for", "some training!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: PIDGEOTTO, level: 24 }, TrainerPokemon { species_id: RATICATE, level: 24 }],
        },
        // Picnicker trainer
        NpcDef {
            x: 9, y: 10, sprite_id: 2, facing: Direction::Left,
            dialogue: &["The wild Pokemon", "here are unusual!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: FLAAFFY, level: 24 }, TrainerPokemon { species_id: GIRAFARIG, level: 26 }],
        },
        // Psychic trainer
        NpcDef {
            x: 5, y: 14, sprite_id: 4, facing: Direction::Down,
            dialogue: &["I sense a great", "disturbance at the", "LAKE OF RAGE..."],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: GIRAFARIG, level: 26 }],
        },
    ];
    let encounters = vec![
        EncounterEntry { species_id: GIRAFARIG, min_level: 15, max_level: 17, weight: 25 },
        EncounterEntry { species_id: PIDGEOTTO, min_level: 16, max_level: 17, weight: 20 },
        EncounterEntry { species_id: VENONAT, min_level: 15, max_level: 16, weight: 20 },
        EncounterEntry { species_id: NOCTOWL, min_level: 17, max_level: 17, weight: 10 },
        EncounterEntry { species_id: FLAAFFY, min_level: 15, max_level: 17, weight: 15 },
        EncounterEntry { species_id: RATICATE, min_level: 16, max_level: 17, weight: 10 },
    ];
    MapData { id: MapId::Route43, name: "ROUTE 43", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![], water_encounters: vec![], music_id: 2 }
}

// ─── Lake of Rage (16×14) ───────────────────────────────
// Red Gyarados event area. Large lake with shore, trees.
fn build_lake_of_rage() -> MapData {
    let width: usize = 16;
    let height: usize = 14;
    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,WATER,WATER,WATER,WATER,WATER,WATER,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 3
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4
        GRASS,GRASS,GRASS,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,GRASS,GRASS,GRASS,
        // Row 5: Red Gyarados in water
        GRASS,GRASS,GRASS,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,GRASS,GRASS,GRASS,
        // Row 6
        GRASS,GRASS,GRASS,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,GRASS,GRASS,GRASS,
        // Row 7
        GRASS,GRASS,GRASS,GRASS,WATER,WATER,WATER,WATER,WATER,WATER,WATER,WATER,GRASS,GRASS,GRASS,GRASS,
        // Row 8
        GRASS,GRASS,GRASS,GRASS,GRASS,WATER,WATER,WATER,WATER,WATER,WATER,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 9: shore
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,
        // Row 10
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,
        // Row 11: sign
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,SIGN,PATH,PATH,SIGN,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 12: south exit to Route 43
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 13
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
    ];
    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 3
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 4
        C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,
        // Row 5
        C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,
        // Row 6
        C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,
        // Row 7
        C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 8
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 9
        C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,
        // Row 10
        C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,
        // Row 11
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SIGN,C_WALK,C_WALK,C_SIGN,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 12: south exit
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WARP,C_WARP,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 13
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];
    let warps = vec![
        // South exit → Route 43
        WarpData { x: 7, y: 12, dest_map: MapId::Route43, dest_x: 5, dest_y: 2 },
        WarpData { x: 8, y: 12, dest_map: MapId::Route43, dest_x: 6, dest_y: 2 },
    ];
    let npcs = vec![
        // Lance NPC (appears during Red Gyarados event)
        NpcDef {
            x: 4, y: 8, sprite_id: 2, facing: Direction::Right,
            dialogue: &["I'm LANCE of the", "ELITE FOUR.", "I've been investigating", "the lake's disturbance.", "A red GYARADOS was", "spotted here. Be careful!"],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // Fisherman
        NpcDef {
            x: 12, y: 8, sprite_id: 7, facing: Direction::Left,
            dialogue: &["I saw a huge red", "GYARADOS! It was", "terrifying!"],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // Cooltrainer (trainer)
        NpcDef {
            x: 3, y: 10, sprite_id: 4, facing: Direction::Down,
            dialogue: &["Something evil is", "happening in MAHOGANY."],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: GYARADOS, level: 27 }, TrainerPokemon { species_id: GOLBAT, level: 27 }],
        },
        // Red Gyarados (NPC 3) — visible at water's edge, hidden after event
        NpcDef {
            x: 4, y: 2, sprite_id: 3, facing: Direction::Down,
            dialogue: &["A huge red GYARADOS", "is thrashing in the", "lake!"],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];
    let encounters = vec![
        EncounterEntry { species_id: MAGIKARP, min_level: 10, max_level: 20, weight: 90 },
        EncounterEntry { species_id: GYARADOS, min_level: 15, max_level: 15, weight: 10 },
    ];
    MapData { id: MapId::LakeOfRage, name: "LAKE OF RAGE", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![], water_encounters: vec![
        EncounterEntry { species_id: MAGIKARP, min_level: 15, max_level: 25, weight: 70 },
        EncounterEntry { species_id: GYARADOS, min_level: 25, max_level: 30, weight: 30 },
    ], music_id: 4 }
}

fn build_route_44() -> MapData {
    let width = 20;
    let height = 12;
    let tiles = vec![
        // Row 0: trees
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: trees
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: trees | grass with tall grass patches
        TREE_TOP,TREE_TOP,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 3: trees | grass and path
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,PATH,PATH,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4: grass | path | pond
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,WATER,WATER,GRASS,GRASS,
        // Row 5: west entrance | path through middle | east side
        PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,WATER,WATER,GRASS,PATH,
        // Row 6: west entrance | path | east side
        PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,
        // Row 7: grass | path continues
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 8: tall grass patches | path
        TREE_TOP,TREE_TOP,GRASS,TALL_GRASS,TALL_GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 9: trees | grass
        TREE_BOTTOM,TREE_BOTTOM,GRASS,TALL_GRASS,TALL_GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 10: trees
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 11: trees
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];
    let collision = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 3
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 4
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WALK,C_WALK,
        // Row 5: west entrance at (0,5)(1,5)
        C_WARP,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WALK,C_WARP,
        // Row 6: west entrance at (0,6)(1,6), east entrance at (19,6)
        C_WARP,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,
        // Row 7
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 8
        C_SOLID,C_SOLID,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 9
        C_SOLID,C_SOLID,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 10
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 11
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];
    let warps = vec![
        // West exit → Mahogany Town east
        WarpData { x: 0, y: 5, dest_map: MapId::MahoganyTown, dest_x: 13, dest_y: 8 },
        WarpData { x: 1, y: 5, dest_map: MapId::MahoganyTown, dest_x: 13, dest_y: 8 },
        WarpData { x: 0, y: 6, dest_map: MapId::MahoganyTown, dest_x: 13, dest_y: 9 },
        WarpData { x: 1, y: 6, dest_map: MapId::MahoganyTown, dest_x: 13, dest_y: 9 },
        // East exit → Ice Path
        WarpData { x: 19, y: 5, dest_map: MapId::IcePath, dest_x: 1, dest_y: 6 },
        WarpData { x: 19, y: 6, dest_map: MapId::IcePath, dest_x: 1, dest_y: 7 },
    ];
    let npcs = vec![
        // Trainer 1: Psychic (middle of route)
        NpcDef {
            x: 9, y: 4, sprite_id: 2, facing: Direction::Down,
            dialogue: &["My PSYCHIC powers", "tell me you'll lose!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: DROWZEE, level: 24 }, TrainerPokemon { species_id: HAUNTER, level: 26 }],
        },
        // Trainer 2: Fisher (near pond)
        NpcDef {
            x: 15, y: 4, sprite_id: 7, facing: Direction::Down,
            dialogue: &["I fish here every", "day, rain or shine!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: POLIWAG, level: 24 }, TrainerPokemon { species_id: POLIWHIRL, level: 26 }],
        },
        // Trainer 3: Bird Keeper (south path)
        NpcDef {
            x: 10, y: 8, sprite_id: 2, facing: Direction::Up,
            dialogue: &["My birds are the", "fastest around!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: PIDGEOTTO, level: 26 }, TrainerPokemon { species_id: FEAROW, level: 26 }],
        },
        // NPC 4: Hint NPC about Ice Path
        NpcDef {
            x: 5, y: 7, sprite_id: 5, facing: Direction::Right,
            dialogue: &["ICE PATH is to the", "east. Watch your", "step on the ice!", "It's treacherous."],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];
    let encounters = vec![
        EncounterEntry { species_id: BELLSPROUT, min_level: 22, max_level: 24, weight: 30 },
        EncounterEntry { species_id: ODDISH, min_level: 22, max_level: 24, weight: 25 },
        EncounterEntry { species_id: RATICATE, min_level: 23, max_level: 25, weight: 15 },
        EncounterEntry { species_id: PIDGEOTTO, min_level: 23, max_level: 25, weight: 15 },
        EncounterEntry { species_id: POLIWAG, min_level: 22, max_level: 24, weight: 10 },
        EncounterEntry { species_id: GEODUDE, min_level: 22, max_level: 24, weight: 5 },
    ];
    MapData { id: MapId::Route44, name: "ROUTE 44", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![], water_encounters: vec![
        EncounterEntry { species_id: POLIWAG, min_level: 20, max_level: 25, weight: 40 },
        EncounterEntry { species_id: POLIWHIRL, min_level: 22, max_level: 25, weight: 20 },
        EncounterEntry { species_id: MAGIKARP, min_level: 10, max_level: 20, weight: 40 },
    ], music_id: 2 }
}

fn build_ice_path() -> MapData {
    let width = 14;
    let height = 14;
    // Ice Path sliding puzzle layout:
    // Player enters from west (0,6)/(0,7), needs to reach east exit (13,7).
    // Ice patches (ICE_FLOOR) cause sliding — player slides until hitting a rock (CAVE_WALL) or
    // normal floor (CAVE_FLOOR). Strategic rocks placed to create a solvable puzzle requiring
    // 3+ direction changes.
    //
    // Solution path: Enter at (1,7) → walk right to (3,7) → step right onto ice at (4,7) →
    // slide right, hit rock at (7,6)/(7,7) stop at (6,7) → step down to (6,8) →
    // step right onto ice at (7,8) → slide right, hit wall stop at (11,8) → step up to (11,7) →
    // step right onto ice at (12,7) → slide right to exit warp at (13,7).
    let tiles = vec![
        // Row 0: cave walls
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,
        // Row 1: cave walls with upper alcove
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,
        // Row 2: opening up — ice starts
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,CAVE_WALL,
        // Row 3: wider cave with ice
        CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,CAVE_WALL,ICE_FLOOR,ICE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,
        // Row 4: ice puzzle area — rock at (7,4) forces direction change
        CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,CAVE_WALL,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,
        // Row 5: main passage with ice
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,CAVE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 6: west entrance — rock at (7,6) blocks straight slide
        CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,CAVE_WALL,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 7: east exit at (13,7) — walk floor then ice
        CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,CAVE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,CAVE_FLOOR,
        // Row 8: lower passage with ice
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,ICE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 9: ice patches — rock at (12,9) stops sliding
        CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,ICE_FLOOR,ICE_FLOOR,CAVE_WALL,CAVE_FLOOR,ICE_FLOOR,ICE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,
        // Row 10: narrowing
        CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,
        // Row 11: narrow
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,CAVE_WALL,
        // Row 12: cave walls
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,
        // Row 13: cave walls
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,
    ];
    let collision = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: ice starts
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_ICE,C_ICE,C_ICE,C_ICE,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 3: rock at (7,3)
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_ICE,C_ICE,C_ICE,C_SOLID,C_ICE,C_ICE,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 4: rock at (7,4)
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_ICE,C_ICE,C_ICE,C_SOLID,C_ICE,C_ICE,C_ICE,C_WALK,C_SOLID,C_SOLID,
        // Row 5
        C_SOLID,C_WALK,C_WALK,C_WALK,C_ICE,C_ICE,C_ICE,C_WALK,C_ICE,C_ICE,C_ICE,C_WALK,C_WALK,C_SOLID,
        // Row 6: west entrance at (0,6), rock at (7,6)
        C_WARP,C_WALK,C_WALK,C_WALK,C_ICE,C_ICE,C_ICE,C_SOLID,C_ICE,C_ICE,C_ICE,C_ICE,C_WALK,C_SOLID,
        // Row 7: east exit at (13,7)
        C_WARP,C_WALK,C_WALK,C_WALK,C_ICE,C_ICE,C_ICE,C_WALK,C_ICE,C_ICE,C_ICE,C_ICE,C_ICE,C_WARP,
        // Row 8: ice puzzle lower area
        C_SOLID,C_WALK,C_WALK,C_WALK,C_ICE,C_ICE,C_ICE,C_ICE,C_ICE,C_ICE,C_ICE,C_ICE,C_WALK,C_SOLID,
        // Row 9: rock at (6,9)
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_ICE,C_ICE,C_SOLID,C_WALK,C_ICE,C_ICE,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 10
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 11
        C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 12
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 13
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];
    let warps = vec![
        // West entrance → Route 44 east
        WarpData { x: 0, y: 6, dest_map: MapId::Route44, dest_x: 18, dest_y: 5 },
        WarpData { x: 0, y: 7, dest_map: MapId::Route44, dest_x: 18, dest_y: 6 },
        // East exit → Blackthorn City west
        WarpData { x: 13, y: 7, dest_map: MapId::BlackthornCity, dest_x: 2, dest_y: 8 },
    ];
    let npcs = vec![
        // Trainer 1: Boarder (upper area, standing on walkable floor near ice)
        NpcDef {
            x: 3, y: 3, sprite_id: 2, facing: Direction::Down,
            dialogue: &["The ice makes it", "hard to battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: SWINUB, level: 28 }, TrainerPokemon { species_id: SNEASEL, level: 28 }],
        },
        // Trainer 2: Skier (lower area, standing on walkable floor)
        NpcDef {
            x: 7, y: 10, sprite_id: 2, facing: Direction::Up,
            dialogue: &["I trained on", "mountains for this!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: JYNX, level: 28 }, TrainerPokemon { species_id: DELIBIRD, level: 30 }],
        },
    ];
    let encounters = vec![
        EncounterEntry { species_id: ZUBAT, min_level: 22, max_level: 24, weight: 20 },
        EncounterEntry { species_id: GOLBAT, min_level: 24, max_level: 26, weight: 15 },
        EncounterEntry { species_id: SWINUB, min_level: 22, max_level: 24, weight: 30 },
        EncounterEntry { species_id: GEODUDE, min_level: 22, max_level: 24, weight: 15 },
        EncounterEntry { species_id: JYNX, min_level: 24, max_level: 26, weight: 10 },
        EncounterEntry { species_id: SNEASEL, min_level: 24, max_level: 26, weight: 10 },
    ];
    MapData { id: MapId::IcePath, name: "ICE PATH", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![], water_encounters: vec![
        EncounterEntry { species_id: MAGIKARP, min_level: 15, max_level: 20, weight: 40 },
        EncounterEntry { species_id: SEEL, min_level: 20, max_level: 25, weight: 30 },
        EncounterEntry { species_id: SHELLDER, min_level: 20, max_level: 25, weight: 30 },
    ], music_id: 3 }
}

fn build_blackthorn_city() -> MapData {
    let width = 20;
    let height = 14;
    let tiles = vec![
        // Row 0: trees
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: trees
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: gym roof area
        GRASS,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,GRASS,
        // Row 3: gym wall + mart roof
        GRASS,GRASS,GRASS,BUILDING_WALL,BUILDING_WALL,DOOR,BUILDING_WALL,GRASS,GRASS,PATH,PATH,GRASS,GRASS,BUILDING_WALL,BUILDING_WALL,DOOR,BUILDING_WALL,GRASS,GRASS,GRASS,
        // Row 4: path
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        // Row 5: Pokemon Center roof + path
        GRASS,POKECENTER_ROOF,POKECENTER_ROOF,POKECENTER_ROOF,GRASS,PATH,GRASS,PATH,PATH,GRASS,GRASS,PATH,PATH,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,
        // Row 6: Pokemon Center wall + path
        GRASS,POKECENTER_WALL,POKECENTER_WALL,POKECENTER_DOOR,GRASS,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,
        // Row 7: path connections
        GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,
        // Row 8: west entrance + house + path
        PATH,PATH,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 9: house wall
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,BUILDING_WALL,BUILDING_WALL,DOOR,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 10: path south
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 11: flowers and path to south exit
        GRASS,GRASS,FLOWER,FLOWER,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,FLOWER,FLOWER,GRASS,GRASS,
        // Row 12: trees
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,PATH,PATH,PATH,PATH,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 13: trees
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];
    let collision = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: gym roof + mart roof (solid)
        C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,
        // Row 3: gym wall/door + mart wall/door
        C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WARP,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WARP,C_SOLID,C_WALK,C_WALK,C_WALK,
        // Row 4
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5: pokecenter roof
        C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 6: pokecenter door
        C_WALK,C_SOLID,C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 7
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 8: west entrance at (0,8)(1,8), house roof
        C_WARP,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 9: house wall/door
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 10
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 11
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 12: south exit at (8,12)(9,12)(10,12)(11,12)
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 13
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];
    let warps = vec![
        // West entrance → Ice Path east exit
        WarpData { x: 0, y: 8, dest_map: MapId::IcePath, dest_x: 12, dest_y: 7 },
        WarpData { x: 1, y: 8, dest_map: MapId::IcePath, dest_x: 12, dest_y: 7 },
        // Gym door (5,3)
        WarpData { x: 5, y: 3, dest_map: MapId::BlackthornGym, dest_x: 5, dest_y: 7 },
        // PokemonCenter door (3,6)
        WarpData { x: 3, y: 6, dest_map: MapId::PokemonCenter, dest_x: 4, dest_y: 6 },
        // Mart door (15,3) → GenericHouse
        WarpData { x: 15, y: 3, dest_map: MapId::GenericHouse, dest_x: 3, dest_y: 5 },
        // House door (10,9) → GenericHouse
        WarpData { x: 10, y: 9, dest_map: MapId::GenericHouse, dest_x: 3, dest_y: 5 },
        // South exit → Route 45
        WarpData { x: 8, y: 12, dest_map: MapId::Route45, dest_x: 4, dest_y: 2 },
        WarpData { x: 9, y: 12, dest_map: MapId::Route45, dest_x: 5, dest_y: 2 },
        WarpData { x: 10, y: 12, dest_map: MapId::Route45, dest_x: 6, dest_y: 2 },
        WarpData { x: 11, y: 12, dest_map: MapId::Route45, dest_x: 7, dest_y: 2 },
    ];
    let npcs = vec![
        // NPC 0: Old man near gym
        NpcDef {
            x: 7, y: 3, sprite_id: 0, facing: Direction::Down,
            dialogue: &["CLAIR is the toughest", "GYM LEADER in JOHTO!", "Her DRAGON types are", "nearly unbeatable."],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
        // NPC 1: Girl near center
        NpcDef {
            x: 6, y: 6, sprite_id: 1, facing: Direction::Down,
            dialogue: &["BLACKTHORN CITY is", "home to the DRAGON", "masters. Beat CLAIR", "and earn the last", "JOHTO badge!"],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
        // NPC 2: Move Tutor NPC
        NpcDef {
            x: 12, y: 7, sprite_id: 0, facing: Direction::Left,
            dialogue: &["I can teach your", "POKEMON special moves.", "Come back anytime!"],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
    ];
    let encounters = vec![];
    MapData { id: MapId::BlackthornCity, name: "BLACKTHORN CITY", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![], water_encounters: vec![], music_id: 1 }
}

fn build_blackthorn_gym() -> MapData {
    let width = 10;
    let height = 10;
    let tiles = vec![
        // Row 0: walls
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: Clair's platform
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 2
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 3: trainer area
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 4
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 5: trainer area
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 6
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 7
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 8: entrance
        BLACK,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,DOOR,GYM_FLOOR,GYM_FLOOR,GYM_FLOOR,BLACK,
        // Row 9: walls
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];
    let collision = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 2
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 3
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 4
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 5
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 7
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 8: entrance warp at (5,8)
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 9
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];
    let warps = vec![
        // Exit → Blackthorn City gym door
        WarpData { x: 5, y: 8, dest_map: MapId::BlackthornCity, dest_x: 5, dest_y: 4 },
    ];
    let npcs = vec![
        // NPC 0: Clair (gym leader)
        NpcDef {
            x: 5, y: 2, sprite_id: 1, facing: Direction::Down,
            dialogue: &["I am CLAIR!", "The world's best", "dragon master!", "You dare challenge", "me? Fine!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: DRAGONAIR, level: 37 },
                TrainerPokemon { species_id: DRAGONAIR, level: 37 },
                TrainerPokemon { species_id: DRAGONAIR, level: 37 },
                TrainerPokemon { species_id: KINGDRA, level: 40 },
            ],
        },
        // NPC 1: Cooltrainer left
        NpcDef {
            x: 2, y: 5, sprite_id: 2, facing: Direction::Right,
            dialogue: &["Dragon-types are", "the strongest!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: HORSEA, level: 33 }, TrainerPokemon { species_id: SEADRA, level: 35 }],
        },
        // NPC 2: Cooltrainer right
        NpcDef {
            x: 7, y: 5, sprite_id: 2, facing: Direction::Left,
            dialogue: &["Our dragons will", "burn you up!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: DRATINI, level: 33 }, TrainerPokemon { species_id: DRAGONAIR, level: 35 }],
        },
    ];
    let encounters = vec![];
    MapData { id: MapId::BlackthornGym, name: "BLACKTHORN GYM", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![], water_encounters: vec![], music_id: 5 }
}

fn build_route_45() -> MapData {
    let width = 12;
    let height = 24;
    let tiles = vec![
        // Row 0: trees top border
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: trees with north entrance
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,PATH,PATH,PATH,PATH,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: mountain terrain
        TREE_TOP,TREE_TOP,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 3
        TREE_BOTTOM,TREE_BOTTOM,GRASS,TALL_GRASS,TALL_GRASS,PATH,PATH,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4
        GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 5: ledge area
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,
        // Row 6
        TREE_TOP,TREE_TOP,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,
        // Row 7
        TREE_BOTTOM,TREE_BOTTOM,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 8
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 9
        GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,
        // Row 10
        TREE_TOP,TREE_TOP,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 11
        TREE_BOTTOM,TREE_BOTTOM,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 12: middle section
        GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TALL_GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 13
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 14
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 15
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 16
        GRASS,GRASS,GRASS,TALL_GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 17
        GRASS,GRASS,GRASS,TALL_GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,
        // Row 18
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        // Row 19
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,TALL_GRASS,TALL_GRASS,GRASS,
        // Row 20
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,
        // Row 21: approaching south exit
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 22: south exit
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,PATH,PATH,PATH,PATH,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 23: trees
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];
    let collision = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: north entrance at (4-7,1)
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 3
        C_SOLID,C_SOLID,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 4
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,
        // Row 6
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,
        // Row 7
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 8
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_SOLID,C_SOLID,
        // Row 9
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,
        // Row 10
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 11
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 12
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_WALK,C_SOLID,C_SOLID,C_SOLID,
        // Row 13
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 14
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 15
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 16
        C_WALK,C_WALK,C_WALK,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 17
        C_WALK,C_WALK,C_WALK,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 18
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 19
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,
        // Row 20
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,
        // Row 21
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 22: south exit at (4-7,22)
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 23
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];
    let warps = vec![
        // North entrance → Blackthorn City south
        WarpData { x: 4, y: 1, dest_map: MapId::BlackthornCity, dest_x: 8, dest_y: 11 },
        WarpData { x: 5, y: 1, dest_map: MapId::BlackthornCity, dest_x: 9, dest_y: 11 },
        WarpData { x: 6, y: 1, dest_map: MapId::BlackthornCity, dest_x: 10, dest_y: 11 },
        WarpData { x: 7, y: 1, dest_map: MapId::BlackthornCity, dest_x: 11, dest_y: 11 },
        // South exit → Route 46 north
        WarpData { x: 4, y: 22, dest_map: MapId::Route46, dest_x: 4, dest_y: 2 },
        WarpData { x: 5, y: 22, dest_map: MapId::Route46, dest_x: 5, dest_y: 2 },
        WarpData { x: 6, y: 22, dest_map: MapId::Route46, dest_x: 6, dest_y: 2 },
        WarpData { x: 7, y: 22, dest_map: MapId::Route46, dest_x: 7, dest_y: 2 },
    ];
    let npcs = vec![
        // Trainer 1: Hiker (upper area)
        NpcDef {
            x: 5, y: 5, sprite_id: 2, facing: Direction::Down,
            dialogue: &["These mountains are", "my training ground!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: GEODUDE, level: 27 }, TrainerPokemon { species_id: GRAVELER, level: 29 }],
        },
        // Trainer 2: Blackbelt (middle)
        NpcDef {
            x: 4, y: 12, sprite_id: 2, facing: Direction::Down,
            dialogue: &["My fighting spirit", "burns bright!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: MACHOKE, level: 29 }, TrainerPokemon { species_id: PRIMEAPE, level: 29 }],
        },
        // Trainer 3: Cooltrainer (lower area)
        NpcDef {
            x: 6, y: 18, sprite_id: 2, facing: Direction::Up,
            dialogue: &["I've trained hard", "on this mountain!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: SKARMORY, level: 30 }, TrainerPokemon { species_id: GLIGAR, level: 30 }],
        },
    ];
    let encounters = vec![
        EncounterEntry { species_id: GEODUDE, min_level: 23, max_level: 27, weight: 25 },
        EncounterEntry { species_id: GRAVELER, min_level: 26, max_level: 28, weight: 15 },
        EncounterEntry { species_id: GLIGAR, min_level: 24, max_level: 26, weight: 15 },
        EncounterEntry { species_id: TEDDIURSA, min_level: 23, max_level: 25, weight: 20 },
        EncounterEntry { species_id: SKARMORY, min_level: 24, max_level: 26, weight: 5 },
        EncounterEntry { species_id: RATICATE, min_level: 25, max_level: 27, weight: 10 },
        EncounterEntry { species_id: SPEAROW, min_level: 23, max_level: 25, weight: 10 },
    ];
    MapData { id: MapId::Route45, name: "ROUTE 45", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![], water_encounters: vec![], music_id: 2 }
}

fn build_route_46() -> MapData {
    let width = 12;
    let height = 16;
    let tiles = vec![
        // Row 0: trees
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: north entrance from Route 45
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,PATH,PATH,PATH,PATH,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2
        TREE_TOP,TREE_TOP,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 3
        TREE_BOTTOM,TREE_BOTTOM,GRASS,TALL_GRASS,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4
        GRASS,GRASS,GRASS,TALL_GRASS,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,
        // Row 5
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 6
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 7
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 8
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 9
        GRASS,GRASS,GRASS,TALL_GRASS,PATH,PATH,GRASS,GRASS,TALL_GRASS,GRASS,GRASS,GRASS,
        // Row 10
        TREE_TOP,TREE_TOP,GRASS,TALL_GRASS,PATH,GRASS,GRASS,GRASS,TALL_GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 11
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 12
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 13: approaching south exit
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 14: south exit → Route 29
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,PATH,PATH,PATH,PATH,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 15: trees
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];
    let collision = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: north entrance at (4-7,1)
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 3
        C_SOLID,C_SOLID,C_WALK,C_TALL,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_SOLID,C_SOLID,
        // Row 4
        C_WALK,C_WALK,C_WALK,C_TALL,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,
        // Row 5
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 6
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 7
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 8
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 9
        C_WALK,C_WALK,C_WALK,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_WALK,C_WALK,C_WALK,
        // Row 10
        C_SOLID,C_SOLID,C_WALK,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_WALK,C_SOLID,C_SOLID,
        // Row 11
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 12
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 13
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 14: south exit at (4-7,14)
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 15
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];
    let warps = vec![
        // North entrance → Route 45 south
        WarpData { x: 4, y: 1, dest_map: MapId::Route45, dest_x: 4, dest_y: 21 },
        WarpData { x: 5, y: 1, dest_map: MapId::Route45, dest_x: 5, dest_y: 21},
        WarpData { x: 6, y: 1, dest_map: MapId::Route45, dest_x: 6, dest_y: 21},
        WarpData { x: 7, y: 1, dest_map: MapId::Route45, dest_x: 7, dest_y: 21},
        // South exit → Route 29 (near east end, on walkable path tiles)
        WarpData { x: 4, y: 14, dest_map: MapId::Route29, dest_x: 26, dest_y: 5 },
        WarpData { x: 5, y: 14, dest_map: MapId::Route29, dest_x: 27, dest_y: 5 },
        WarpData { x: 6, y: 14, dest_map: MapId::Route29, dest_x: 26, dest_y: 4 },
        WarpData { x: 7, y: 14, dest_map: MapId::Route29, dest_x: 27, dest_y: 4 },
    ];
    let npcs = vec![
        // Trainer 1: Hiker
        NpcDef {
            x: 5, y: 5, sprite_id: 2, facing: Direction::Down,
            dialogue: &["I climbed all the", "way from ROUTE 29!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: GEODUDE, level: 25 }, TrainerPokemon { species_id: GEODUDE, level: 25 }, TrainerPokemon { species_id: GRAVELER, level: 27 }],
        },
        // Trainer 2: Picnicker
        NpcDef {
            x: 4, y: 10, sprite_id: 1, facing: Direction::Right,
            dialogue: &["This route is a", "great shortcut!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[TrainerPokemon { species_id: TEDDIURSA, level: 27 }, TrainerPokemon { species_id: RATICATE, level: 27 }],
        },
        // NPC 3: Hint about connecting to Route 29
        NpcDef {
            x: 6, y: 8, sprite_id: 5, facing: Direction::Down,
            dialogue: &["This path connects", "all the way back to", "ROUTE 29 and NEW", "BARK TOWN!"],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
    ];
    let encounters = vec![
        EncounterEntry { species_id: GEODUDE, min_level: 20, max_level: 24, weight: 30 },
        EncounterEntry { species_id: RATTATA, min_level: 18, max_level: 22, weight: 25 },
        EncounterEntry { species_id: SPEAROW, min_level: 18, max_level: 22, weight: 25 },
        EncounterEntry { species_id: GEODUDE, min_level: 22, max_level: 26, weight: 10 },
        EncounterEntry { species_id: GRAVELER, min_level: 24, max_level: 26, weight: 10 },
    ];
    MapData { id: MapId::Route46, name: "ROUTE 46", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![], water_encounters: vec![], music_id: 2 }
}

// ─── Route 27 (24x12) ──────────────────────────────────
// Connects New Bark Town (east/right) to Route 26 (west/left).
// Long east-west route with tall grass, trainers, and strong wild Pokemon.
// In GSC this is east of New Bark but we have it west for map layout.

fn build_route_27() -> MapData {
    let width: usize = 24;
    let height: usize = 12;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: trees across top
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: tree bottoms
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: trees | grass | tall grass patches
        TREE_TOP,TREE_TOP,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,
        GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 3: trees | grass corridor
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,
        GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4: open area with path
        GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,PATH,PATH,PATH,GRASS,
        GRASS,PATH,PATH,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,
        // Row 5: water edge | path
        WATER,WATER,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,
        GRASS,PATH,GRASS,GRASS,SIGN,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,
        // Row 6: water | main east-west path
        WATER,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,PATH,
        // Row 7: water edge | grass
        WATER,WATER,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 8: grass | tall grass
        GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,
        TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 9: grass | tall grass
        GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,
        TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 10: tree tops along bottom
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 11: tree bottoms along bottom
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: trees | walk | tall grass
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 3: trees | walk | tall grass
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 4: open path area
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5: water | path | sign
        C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_SIGN,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 6: water at x=0 | main path | right warp to NewBarkTown
        C_WATER,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,
        // Row 7: water | grass
        C_WATER,C_WATER,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 8: grass | tall grass
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,
        C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 9: grass | tall grass
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,
        C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 10: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 11: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), width * height, "Route27 tiles count mismatch");
    debug_assert_eq!(collision.len(), width * height, "Route27 collision count mismatch");

    let warps = vec![
        // Right edge → New Bark Town (left exit)
        WarpData { x: 23, y: 6, dest_map: MapId::NewBarkTown, dest_x: 2, dest_y: 10 },
        // Left edge → Route 26
        WarpData { x: 1, y: 6, dest_map: MapId::Route26, dest_x: 5, dest_y: 17 },
    ];

    let npcs = vec![
        // Cooltrainer (female)
        NpcDef {
            x: 8, y: 4, sprite_id: 3, facing: Direction::Down,
            dialogue: &["The road ahead is long.", "Make sure your POKEMON", "are ready for battle!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: DODRIO, level: 30 },
                TrainerPokemon { species_id: RATICATE, level: 30 },
            ],
        },
        // Psychic (male)
        NpcDef {
            x: 15, y: 7, sprite_id: 2, facing: Direction::Up,
            dialogue: &["I can see your future...", "You will face the ELITE", "FOUR soon!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: QUAGSIRE, level: 32 },
                TrainerPokemon { species_id: NOCTOWL, level: 32 },
            ],
        },
        // Bird Keeper
        NpcDef {
            x: 20, y: 4, sprite_id: 2, facing: Direction::Left,
            dialogue: &["My birds are the", "fastest around!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: FEAROW, level: 29 },
                TrainerPokemon { species_id: DODRIO, level: 31 },
            ],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: DODUO, min_level: 28, max_level: 30, weight: 30 },
        EncounterEntry { species_id: RATICATE, min_level: 28, max_level: 30, weight: 25 },
        EncounterEntry { species_id: PONYTA, min_level: 28, max_level: 30, weight: 15 },
        EncounterEntry { species_id: SANDSLASH, min_level: 28, max_level: 30, weight: 10 },
        EncounterEntry { species_id: DODRIO, min_level: 30, max_level: 32, weight: 5 },
        EncounterEntry { species_id: ARCANINE, min_level: 30, max_level: 30, weight: 5 },
        EncounterEntry { species_id: QUAGSIRE, min_level: 28, max_level: 30, weight: 10 },
    ];

    MapData { id: MapId::Route27, name: "ROUTE 27", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![], water_encounters: vec![
        EncounterEntry { species_id: TENTACOOL, min_level: 25, max_level: 30, weight: 35 },
        EncounterEntry { species_id: TENTACRUEL, min_level: 28, max_level: 30, weight: 15 },
        EncounterEntry { species_id: MAGIKARP, min_level: 15, max_level: 25, weight: 30 },
        EncounterEntry { species_id: SHELLDER, min_level: 25, max_level: 30, weight: 20 },
    ], music_id: 2 }
}

// ─── Route 26 (12x20) ──────────────────────────────────
// North-south route connecting Route 27 (south) to Victory Road (north).
// Trainers, tall grass, path going north toward Indigo Plateau.

fn build_route_26() -> MapData {
    let width: usize = 12;
    let height: usize = 20;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: trees across top
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
        // Row 1: tree bottoms + gap for north exit
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,PATH,PATH,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        // Row 2: trees | pokecenter | path
        TREE_TOP,TREE_TOP,POKECENTER_ROOF,POKECENTER_ROOF,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 3: trees | pokecenter | path
        TREE_BOTTOM,TREE_BOTTOM,POKECENTER_WALL,POKECENTER_DOOR,GRASS,PATH,GRASS,GRASS,SIGN,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 4: grass | path
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 5: tall grass | path | tall grass
        TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,
        // Row 6: tall grass | path | grass
        TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,
        // Row 7: grass | path winds east
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,
        // Row 8: trees | path continues | grass
        TREE_TOP,TREE_TOP,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,
        // Row 9: trees | path
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,
        // Row 10: grass | path winds west
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,
        // Row 11: tall grass | path | tall grass
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,PATH,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,
        // Row 12: tall grass | path
        GRASS,GRASS,TALL_GRASS,TALL_GRASS,PATH,GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,
        // Row 13: grass | path continues south
        GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 14: trees | path | trees
        TREE_TOP,TREE_TOP,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 15: trees | path | trees
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,TREE_BOTTOM,TREE_BOTTOM,
        // Row 16: grass | path going south
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 17: grass | path
        GRASS,GRASS,GRASS,TALL_GRASS,TALL_GRASS,PATH,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 18: grass | path south exit area
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        // Row 19: trees along bottom
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: solid + north exit warps (y=1)
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2: trees | pokecenter roof (solid) | walk | path
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 3: trees | pokecenter wall + door | walk | sign
        C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_SIGN,C_WALK,C_SOLID,C_SOLID,
        // Row 4: walkable
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 5: tall grass | path
        C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,
        // Row 6: tall grass | path
        C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,
        // Row 7: walk | path east
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 8: trees | walk | path
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 9: trees | walk | path
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 10: walk | path west
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 11: walk | tall grass | path
        C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,
        // Row 12: walk | tall grass | path
        C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,
        // Row 13: walk | path
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 14: trees | walk | path
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 15: trees | walk | path
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 16: walk | path
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 17: walk | tall grass | path
        C_WALK,C_WALK,C_WALK,C_TALL,C_TALL,C_WALK,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 18: south exit warps
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        // Row 19: solid trees
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), width * height, "Route26 tiles count mismatch");
    debug_assert_eq!(collision.len(), width * height, "Route26 collision count mismatch");

    let warps = vec![
        // North exit → Victory Road
        WarpData { x: 5, y: 1, dest_map: MapId::VictoryRoad, dest_x: 7, dest_y: 11 },
        WarpData { x: 6, y: 1, dest_map: MapId::VictoryRoad, dest_x: 8, dest_y: 11 },
        // PokemonCenter door
        WarpData { x: 3, y: 3, dest_map: MapId::PokemonCenter, dest_x: 4, dest_y: 6 },
        // South exit → Route 27
        WarpData { x: 5, y: 18, dest_map: MapId::Route27, dest_x: 2, dest_y: 6 },
    ];

    let npcs = vec![
        // Cooltrainer (male) — strong trainer near top
        NpcDef {
            x: 8, y: 5, sprite_id: 2, facing: Direction::Left,
            dialogue: &["You're headed for the", "POKEMON LEAGUE?", "I'll test your strength!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: ARCANINE, level: 33 },
                TrainerPokemon { species_id: SANDSLASH, level: 33 },
            ],
        },
        // Cooltrainer (female) — middle area
        NpcDef {
            x: 3, y: 11, sprite_id: 3, facing: Direction::Right,
            dialogue: &["My POKEMON and I have", "trained together for", "years!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: PONYTA, level: 32 },
                TrainerPokemon { species_id: DODRIO, level: 34 },
            ],
        },
        // Psychic — south section
        NpcDef {
            x: 7, y: 16, sprite_id: 2, facing: Direction::Up,
            dialogue: &["The ELITE FOUR awaits", "at the end of this road."],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: QUAGSIRE, level: 34 },
                TrainerPokemon { species_id: NOCTOWL, level: 34 },
            ],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: DODUO, min_level: 28, max_level: 30, weight: 30 },
        EncounterEntry { species_id: RATICATE, min_level: 28, max_level: 30, weight: 20 },
        EncounterEntry { species_id: PONYTA, min_level: 28, max_level: 30, weight: 20 },
        EncounterEntry { species_id: SANDSLASH, min_level: 28, max_level: 30, weight: 10 },
        EncounterEntry { species_id: DODRIO, min_level: 30, max_level: 32, weight: 5 },
        EncounterEntry { species_id: ARCANINE, min_level: 30, max_level: 30, weight: 5 },
        EncounterEntry { species_id: SANDSHREW, min_level: 26, max_level: 28, weight: 10 },
    ];

    MapData { id: MapId::Route26, name: "ROUTE 26", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![], water_encounters: vec![], music_id: 2 }
}

// ─── Victory Road (14x14) ──────────────────────────────
// Cave dungeon connecting Route 26 to Indigo Plateau.
// Strong trainers and wild encounters (lv30-38).

fn build_victory_road() -> MapData {
    let width: usize = 14;
    let height: usize = 14;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: solid walls
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,
        // Row 1: north exit gap
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,PATH,PATH,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,
        // Row 2: cave interior
        CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,
        // Row 3
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 4
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 5
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 6
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 7
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 8
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 9
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 10
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 11
        CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,
        // Row 12: south exit
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,
        // Row 13: solid
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: north exit warps
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 3
        C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 4
        C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 5
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 7
        C_SOLID,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_SOLID,
        // Row 8
        C_SOLID,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_SOLID,
        // Row 9
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 10
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 11
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 12: south exit warps
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 13
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), width * height, "VictoryRoad tiles count mismatch");
    debug_assert_eq!(collision.len(), width * height, "VictoryRoad collision count mismatch");

    let warps = vec![
        // North exit → Indigo Plateau
        WarpData { x: 6, y: 1, dest_map: MapId::IndigoPlateau, dest_x: 6, dest_y: 7 },
        WarpData { x: 7, y: 1, dest_map: MapId::IndigoPlateau, dest_x: 7, dest_y: 7 },
        // South exit → Route 26
        WarpData { x: 7, y: 12, dest_map: MapId::Route26, dest_x: 5, dest_y: 2 },
        WarpData { x: 8, y: 12, dest_map: MapId::Route26, dest_x: 6, dest_y: 2 },
    ];

    let npcs = vec![
        // Cooltrainer — top left area
        NpcDef {
            x: 2, y: 3, sprite_id: 2, facing: Direction::Right,
            dialogue: &["Only the strongest", "trainers make it", "through here!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: GRAVELER, level: 34 },
                TrainerPokemon { species_id: GOLBAT, level: 34 },
                TrainerPokemon { species_id: URSARING, level: 36 },
            ],
        },
        // Cooltrainer — middle area
        NpcDef {
            x: 8, y: 6, sprite_id: 3, facing: Direction::Left,
            dialogue: &["You'll need all your", "strength to face the", "ELITE FOUR!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: ONIX, level: 35 },
                TrainerPokemon { species_id: STEELIX, level: 37 },
            ],
        },
        // Cooltrainer — bottom area
        NpcDef {
            x: 4, y: 10, sprite_id: 2, facing: Direction::Up,
            dialogue: &["I've trained for years", "to get this far!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: GOLBAT, level: 35 },
                TrainerPokemon { species_id: GRAVELER, level: 35 },
                TrainerPokemon { species_id: MACHOKE, level: 36 },
            ],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: GOLBAT, min_level: 32, max_level: 34, weight: 25 },
        EncounterEntry { species_id: GRAVELER, min_level: 32, max_level: 34, weight: 20 },
        EncounterEntry { species_id: ONIX, min_level: 32, max_level: 34, weight: 15 },
        EncounterEntry { species_id: MACHOKE, min_level: 32, max_level: 34, weight: 15 },
        EncounterEntry { species_id: URSARING, min_level: 34, max_level: 36, weight: 10 },
        EncounterEntry { species_id: GEODUDE, min_level: 30, max_level: 32, weight: 15 },
    ];

    MapData { id: MapId::VictoryRoad, name: "VICTORY ROAD", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![], water_encounters: vec![], music_id: 5 }
}

// ─── Indigo Plateau (14x10) ──────────────────────────────
// Lobby area with PokemonCenter. Entry to Elite Four.

fn build_indigo_plateau() -> MapData {
    let width: usize = 14;
    let height: usize = 10;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: roof
        BLACK,BLACK,BLACK,BLACK,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,BUILDING_ROOF,BLACK,BLACK,BLACK,BLACK,
        // Row 1: wall + E4 entrance
        BLACK,BLACK,BLACK,BLACK,BUILDING_WALL,BUILDING_WALL,BUILDING_WALL,DOOR,BUILDING_WALL,BUILDING_WALL,BLACK,BLACK,BLACK,BLACK,
        // Row 2: path leading to building
        BLACK,BLACK,BLACK,BLACK,GRASS,GRASS,PATH,PATH,PATH,GRASS,BLACK,BLACK,BLACK,BLACK,
        // Row 3: pokecenter left side
        POKECENTER_ROOF,POKECENTER_ROOF,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,SIGN,BLACK,
        // Row 4: pokecenter door
        POKECENTER_WALL,POKECENTER_DOOR,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,BLACK,
        // Row 5: open area
        GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,BLACK,
        // Row 6: path
        GRASS,GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,BLACK,
        // Row 7: more grass
        GRASS,GRASS,FLOWER,GRASS,GRASS,PATH,PATH,PATH,PATH,GRASS,GRASS,FLOWER,GRASS,BLACK,
        // Row 8: south exit area
        GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,BLACK,
        // Row 9: trees along bottom
        TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,TREE_TOP,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: E4 entrance door
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 2
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 3
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SIGN,C_SOLID,
        // Row 4: pokecenter door
        C_SOLID,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 5
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 7
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 8: south exit warps
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 9
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), width * height, "IndigoPlateau tiles count mismatch");
    debug_assert_eq!(collision.len(), width * height, "IndigoPlateau collision count mismatch");

    let warps = vec![
        // E4 entrance → Will's room
        WarpData { x: 7, y: 1, dest_map: MapId::EliteFourWill, dest_x: 5, dest_y: 7 },
        // PokemonCenter door
        WarpData { x: 1, y: 4, dest_map: MapId::PokemonCenter, dest_x: 4, dest_y: 6 },
        // South exit → Victory Road
        WarpData { x: 6, y: 8, dest_map: MapId::VictoryRoad, dest_x: 6, dest_y: 2 },
        WarpData { x: 7, y: 8, dest_map: MapId::VictoryRoad, dest_x: 7, dest_y: 2 },
    ];

    let npcs = vec![
        // Guard NPC
        NpcDef {
            x: 6, y: 2, sprite_id: 2, facing: Direction::Down,
            dialogue: &["Welcome to the", "INDIGO PLATEAU!", "The ELITE FOUR awaits."],
            is_trainer: false, is_mart: false, wanders: false,
            trainer_team: &[],
        },
    ];

    let encounters = vec![];

    MapData { id: MapId::IndigoPlateau, name: "INDIGO PLATEAU", width, height, tiles, collision, warps, npcs, encounters, night_encounters: vec![], water_encounters: vec![], music_id: 1 }
}

// ─── Elite Four Will (10x10) ──────────────────────────────
// Psychic-type specialist. First E4 member.

fn build_elite_four_will() -> MapData {
    let w: usize = 10;
    let h: usize = 10;

    let tiles: Vec<u8> = vec![
        // Row 0
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: door to next room
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,DOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 2: Will's position
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 3
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 4
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 5
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 6
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 7
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 8: entrance
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 9
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];

    let collision: Vec<u8> = vec![
        // Row 0
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        // Row 1: door to Koga's room (only accessible after beating Will)
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 2
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 3
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 4
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 5
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 6
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 7
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 8: entrance from Indigo Plateau
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 9
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "EliteFourWill tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "EliteFourWill collision count mismatch");

    let warps = vec![
        // Forward → Koga's room
        WarpData { x: 5, y: 1, dest_map: MapId::EliteFourKoga, dest_x: 5, dest_y: 7 },
        // Back → Indigo Plateau
        WarpData { x: 5, y: 8, dest_map: MapId::IndigoPlateau, dest_x: 7, dest_y: 2 },
    ];

    let npcs = vec![
        // Will (E4 member, NPC index 0)
        NpcDef {
            x: 5, y: 3, sprite_id: 0, facing: Direction::Down,
            dialogue: &[
                "ELITE FOUR WILL",
                "wants to battle!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: XATU, level: 40 },
                TrainerPokemon { species_id: JYNX, level: 41 },
                TrainerPokemon { species_id: EXEGGUTOR, level: 41 },
                TrainerPokemon { species_id: SLOWBRO, level: 41 },
                TrainerPokemon { species_id: XATU, level: 42 },
            ],
        },
    ];

    MapData { id: MapId::EliteFourWill, name: "ELITE FOUR WILL", width: w, height: h, tiles, collision, warps, npcs, encounters: vec![], night_encounters: vec![], water_encounters: vec![], music_id: 6 }
}

// ─── Elite Four Koga (10x10) ──────────────────────────────
// Poison-type specialist. Second E4 member.

fn build_elite_four_koga() -> MapData {
    let w: usize = 10;
    let h: usize = 10;

    let tiles: Vec<u8> = vec![
        // Row 0
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: door to next room
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,DOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 2
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 3: Koga's position
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 4
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 5
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 6
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 7
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 8: entrance
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 9
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];

    let collision: Vec<u8> = vec![
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "EliteFourKoga tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "EliteFourKoga collision count mismatch");

    let warps = vec![
        // Forward → Bruno's room
        WarpData { x: 5, y: 1, dest_map: MapId::EliteFourBruno, dest_x: 5, dest_y: 7 },
        // Back → Will's room
        WarpData { x: 5, y: 8, dest_map: MapId::EliteFourWill, dest_x: 5, dest_y: 2 },
    ];

    let npcs = vec![
        // Koga (E4 member, NPC index 0)
        NpcDef {
            x: 5, y: 3, sprite_id: 0, facing: Direction::Down,
            dialogue: &[
                "ELITE FOUR KOGA",
                "wants to battle!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: ARIADOS, level: 40 },
                TrainerPokemon { species_id: VENOMOTH, level: 41 },
                TrainerPokemon { species_id: FORRETRESS, level: 43 },
                TrainerPokemon { species_id: MUK, level: 42 },
                TrainerPokemon { species_id: CROBAT, level: 44 },
            ],
        },
    ];

    MapData { id: MapId::EliteFourKoga, name: "ELITE FOUR KOGA", width: w, height: h, tiles, collision, warps, npcs, encounters: vec![], night_encounters: vec![], water_encounters: vec![], music_id: 6 }
}

// ─── Elite Four Bruno (10x10) ──────────────────────────────
// Fighting-type specialist. Third E4 member.

fn build_elite_four_bruno() -> MapData {
    let w: usize = 10;
    let h: usize = 10;

    let tiles: Vec<u8> = vec![
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,DOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];

    let collision: Vec<u8> = vec![
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "EliteFourBruno tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "EliteFourBruno collision count mismatch");

    let warps = vec![
        // Forward → Karen's room
        WarpData { x: 5, y: 1, dest_map: MapId::EliteFourKaren, dest_x: 5, dest_y: 7 },
        // Back → Koga's room
        WarpData { x: 5, y: 8, dest_map: MapId::EliteFourKoga, dest_x: 5, dest_y: 2 },
    ];

    let npcs = vec![
        // Bruno (E4 member, NPC index 0)
        NpcDef {
            x: 5, y: 3, sprite_id: 0, facing: Direction::Down,
            dialogue: &[
                "ELITE FOUR BRUNO",
                "wants to battle!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: HITMONTOP, level: 42 },
                TrainerPokemon { species_id: HITMONLEE, level: 42 },
                TrainerPokemon { species_id: HITMONCHAN, level: 42 },
                TrainerPokemon { species_id: ONIX, level: 43 },
                TrainerPokemon { species_id: MACHAMP, level: 46 },
            ],
        },
    ];

    MapData { id: MapId::EliteFourBruno, name: "ELITE FOUR BRUNO", width: w, height: h, tiles, collision, warps, npcs, encounters: vec![], night_encounters: vec![], water_encounters: vec![], music_id: 6 }
}

// ─── Elite Four Karen (10x10) ──────────────────────────────
// Dark-type specialist. Fourth E4 member.

fn build_elite_four_karen() -> MapData {
    let w: usize = 10;
    let h: usize = 10;

    let tiles: Vec<u8> = vec![
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,DOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];

    let collision: Vec<u8> = vec![
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "EliteFourKaren tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "EliteFourKaren collision count mismatch");

    let warps = vec![
        // Forward → Champion Lance's room
        WarpData { x: 5, y: 1, dest_map: MapId::ChampionLance, dest_x: 5, dest_y: 7 },
        // Back → Bruno's room
        WarpData { x: 5, y: 8, dest_map: MapId::EliteFourBruno, dest_x: 5, dest_y: 2 },
    ];

    let npcs = vec![
        // Karen (E4 member, NPC index 0)
        NpcDef {
            x: 5, y: 3, sprite_id: 0, facing: Direction::Down,
            dialogue: &[
                "ELITE FOUR KAREN",
                "wants to battle!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: UMBREON, level: 42 },
                TrainerPokemon { species_id: VILEPLUME, level: 42 },
                TrainerPokemon { species_id: GENGAR, level: 45 },
                TrainerPokemon { species_id: MURKROW, level: 44 },
                TrainerPokemon { species_id: HOUNDOOM, level: 47 },
            ],
        },
    ];

    MapData { id: MapId::EliteFourKaren, name: "ELITE FOUR KAREN", width: w, height: h, tiles, collision, warps, npcs, encounters: vec![], night_encounters: vec![], water_encounters: vec![], music_id: 6 }
}

// ─── Champion Lance (10x10) ──────────────────────────────
// Dragon-type Champion. Final battle of the Elite Four.

fn build_champion_lance() -> MapData {
    let w: usize = 10;
    let h: usize = 10;

    let tiles: Vec<u8> = vec![
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
    ];

    let collision: Vec<u8> = vec![
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "ChampionLance tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "ChampionLance collision count mismatch");

    let warps = vec![
        // Exit → back to Indigo Plateau (after winning, player is returned here)
        WarpData { x: 5, y: 8, dest_map: MapId::IndigoPlateau, dest_x: 7, dest_y: 2 },
    ];

    let npcs = vec![
        // Lance (Champion, NPC index 0)
        NpcDef {
            x: 5, y: 2, sprite_id: 0, facing: Direction::Down,
            dialogue: &[
                "CHAMPION LANCE",
                "wants to battle!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: GYARADOS, level: 44 },
                TrainerPokemon { species_id: DRAGONITE, level: 47 },
                TrainerPokemon { species_id: DRAGONITE, level: 47 },
                TrainerPokemon { species_id: AERODACTYL, level: 46 },
                TrainerPokemon { species_id: CHARIZARD, level: 46 },
                TrainerPokemon { species_id: DRAGONITE, level: 50 },
            ],
        },
    ];

    MapData { id: MapId::ChampionLance, name: "CHAMPION LANCE", width: w, height: h, tiles, collision, warps, npcs, encounters: vec![], night_encounters: vec![], water_encounters: vec![], music_id: 6 }
}

// ─── Rocket HQ (12x12) ──────────────────────────────────
// Hidden beneath Mahogany Town. 4 Rocket Grunts + Executive boss.
// Clearing sets FLAG_ROCKET_MAHOGANY.
fn build_rocket_hq() -> MapData {
    let w: usize = 12;
    let h: usize = 12;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: walls
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,
        // Row 1: corridor
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 2: corridor with tables
        CAVE_WALL,CAVE_FLOOR,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 3: corridor
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 4: cross corridor
        CAVE_WALL,CAVE_FLOOR,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_FLOOR,CAVE_WALL,
        // Row 5: main hall
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 6: main hall
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 7: corridor
        CAVE_WALL,CAVE_FLOOR,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 8: corridor
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 9: boss room
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 10: exit
        CAVE_WALL,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_FLOOR,CAVE_WALL,
        // Row 11: walls
        CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,CAVE_WALL,
    ];

    #[rustfmt::skip]
    let collision: Vec<u8> = vec![
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WARP,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "RocketHQ tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "RocketHQ collision count mismatch");

    let warps = vec![
        // Exit → back to Mahogany Town (mart door position)
        WarpData { x: 5, y: 10, dest_map: MapId::MahoganyTown, dest_x: 3, dest_y: 7 },
    ];

    let npcs = vec![
        // Rocket Grunt 1
        NpcDef {
            x: 3, y: 3, sprite_id: 6, facing: Direction::Down,
            dialogue: &["ROCKET GRUNT:", "Heh! You won't stop", "us this time!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: RATTATA, level: 24 },
                TrainerPokemon { species_id: KOFFING, level: 24 },
            ],
        },
        // Rocket Grunt 2
        NpcDef {
            x: 8, y: 3, sprite_id: 6, facing: Direction::Left,
            dialogue: &["ROCKET GRUNT:", "Team Rocket will", "never disband!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: ZUBAT, level: 22 },
                TrainerPokemon { species_id: RATICATE, level: 24 },
                TrainerPokemon { species_id: ZUBAT, level: 22 },
            ],
        },
        // Rocket Grunt 3
        NpcDef {
            x: 3, y: 7, sprite_id: 6, facing: Direction::Right,
            dialogue: &["ROCKET GRUNT:", "Our experiments at", "the Lake will succeed!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: KOFFING, level: 25 },
                TrainerPokemon { species_id: MUK, level: 25 },
            ],
        },
        // Rocket Grunt 4
        NpcDef {
            x: 9, y: 7, sprite_id: 6, facing: Direction::Down,
            dialogue: &["ROCKET GRUNT:", "You can't reach the", "boss! Give up!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: GOLBAT, level: 26 },
                TrainerPokemon { species_id: RATICATE, level: 24 },
            ],
        },
        // Executive (Boss, NPC index 4)
        NpcDef {
            x: 6, y: 1, sprite_id: 0, facing: Direction::Down,
            dialogue: &["EXECUTIVE:", "So you defeated all", "our grunts...", "No matter! I'll crush", "you myself!"],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: GOLBAT, level: 28 },
                TrainerPokemon { species_id: KOFFING, level: 28 },
                TrainerPokemon { species_id: MUK, level: 30 },
            ],
        },
    ];

    MapData { id: MapId::RocketHQ, name: "ROCKET HQ", width: w, height: h, tiles, collision, warps, npcs, encounters: vec![], night_encounters: vec![], water_encounters: vec![], music_id: 5 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bark_town_dimensions() {
        let map = load_map(MapId::NewBarkTown);
        assert_eq!(map.width, 20);
        assert_eq!(map.height, 18);
        assert_eq!(map.tiles.len(), 20 * 18);
        assert_eq!(map.collision.len(), 20 * 18);
    }

    #[test]
    fn test_route_29_dimensions() {
        let map = load_map(MapId::Route29);
        assert_eq!(map.width, 30);
        assert_eq!(map.height, 14);
        assert_eq!(map.tiles.len(), 30 * 14);
        assert_eq!(map.collision.len(), 30 * 14);
    }

    #[test]
    fn test_cherrygrove_dimensions() {
        let map = load_map(MapId::CherrygroveCity);
        assert_eq!(map.width, 20);
        assert_eq!(map.height, 18);
        assert_eq!(map.tiles.len(), 20 * 18);
        assert_eq!(map.collision.len(), 20 * 18);
    }

    #[test]
    fn test_route_30_dimensions() {
        let map = load_map(MapId::Route30);
        assert_eq!(map.width, 30);
        assert_eq!(map.height, 18);
        assert_eq!(map.tiles.len(), 30 * 18);
        assert_eq!(map.collision.len(), 30 * 18);
    }

    #[test]
    fn test_player_house_1f_dimensions() {
        let map = load_map(MapId::PlayerHouse1F);
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 8);
        assert_eq!(map.tiles.len(), 10 * 8);
        assert_eq!(map.collision.len(), 10 * 8);
    }

    #[test]
    fn test_player_house_2f_dimensions() {
        let map = load_map(MapId::PlayerHouse2F);
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 8);
        assert_eq!(map.tiles.len(), 10 * 8);
        assert_eq!(map.collision.len(), 10 * 8);
    }

    #[test]
    fn test_elm_lab_dimensions() {
        let map = load_map(MapId::ElmLab);
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);
        assert_eq!(map.tiles.len(), 10 * 10);
        assert_eq!(map.collision.len(), 10 * 10);
    }

    #[test]
    fn test_pokemon_center_dimensions() {
        let map = load_map(MapId::PokemonCenter);
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 8);
        assert_eq!(map.tiles.len(), 10 * 8);
        assert_eq!(map.collision.len(), 10 * 8);
    }

    #[test]
    fn test_all_maps_have_matching_tile_collision_sizes() {
        let maps = [
            MapId::NewBarkTown,
            MapId::Route29,
            MapId::CherrygroveCity,
            MapId::Route30,
            MapId::Route31,
            MapId::VioletCity,
            MapId::VioletGym,
            MapId::SproutTower1F, MapId::SproutTower2F, MapId::SproutTower3F,
            MapId::PlayerHouse1F,
            MapId::PlayerHouse2F,
            MapId::ElmLab,
            MapId::PokemonCenter,
            MapId::Route32,
            MapId::UnionCave,
            MapId::GenericHouse,
            MapId::Route33,
            MapId::AzaleaTown,
            MapId::AzaleaGym,
            MapId::IlexForest,
            MapId::Route34,
            MapId::GoldenrodCity,
            MapId::GoldenrodGym,
            MapId::Route35,
            MapId::NationalPark,
            MapId::Route36,
            MapId::Route37,
            MapId::EcruteakCity,
            MapId::BurnedTower,
            MapId::EcruteakGym,
            MapId::Route38,
            MapId::Route39,
            MapId::OlivineCity,
            MapId::OlivineGym,
            MapId::OlivineLighthouse,
            MapId::Route40,
            MapId::CianwoodCity,
            MapId::CianwoodGym,
            MapId::Route42,
            MapId::MahoganyTown,
            MapId::MahoganyGym,
            MapId::Route43,
            MapId::LakeOfRage,
            MapId::Route44,
            MapId::IcePath,
            MapId::BlackthornCity,
            MapId::BlackthornGym,
            MapId::Route45,
            MapId::Route46,
            MapId::Route27,
            MapId::Route26,
            MapId::VictoryRoad,
            MapId::IndigoPlateau,
            MapId::EliteFourWill,
            MapId::EliteFourKoga,
            MapId::EliteFourBruno,
            MapId::EliteFourKaren,
            MapId::ChampionLance,
            MapId::RocketHQ,
        ];
        for id in &maps {
            let map = load_map(*id);
            assert_eq!(
                map.tiles.len(),
                map.width * map.height,
                "tiles size mismatch for {:?}",
                id
            );
            assert_eq!(
                map.collision.len(),
                map.width * map.height,
                "collision size mismatch for {:?}",
                id
            );
        }
    }

    #[test]
    fn test_all_npcs_on_walkable() {
        let all_maps = [
            MapId::NewBarkTown, MapId::Route29, MapId::CherrygroveCity,
            MapId::Route30, MapId::Route31, MapId::VioletCity, MapId::VioletGym,
            MapId::SproutTower1F, MapId::SproutTower2F, MapId::SproutTower3F, MapId::PlayerHouse1F, MapId::PlayerHouse2F,
            MapId::ElmLab, MapId::PokemonCenter, MapId::Route32, MapId::UnionCave,
            MapId::GenericHouse, MapId::Route33, MapId::AzaleaTown, MapId::AzaleaGym,
            MapId::IlexForest, MapId::Route34, MapId::GoldenrodCity, MapId::GoldenrodGym,
            MapId::Route35, MapId::NationalPark, MapId::Route36, MapId::Route37,
            MapId::EcruteakCity, MapId::BurnedTower, MapId::EcruteakGym,
            MapId::Route38, MapId::Route39, MapId::OlivineCity, MapId::OlivineGym,
            MapId::OlivineLighthouse,
            MapId::Route40, MapId::CianwoodCity, MapId::CianwoodGym,
            MapId::Route42, MapId::MahoganyTown, MapId::MahoganyGym,
            MapId::Route43, MapId::LakeOfRage,
            MapId::Route44, MapId::IcePath, MapId::BlackthornCity, MapId::BlackthornGym,
            MapId::Route45, MapId::Route46,
            MapId::Route27, MapId::Route26,
            MapId::VictoryRoad, MapId::IndigoPlateau,
            MapId::EliteFourWill, MapId::EliteFourKoga,
            MapId::EliteFourBruno, MapId::EliteFourKaren,
            MapId::ChampionLance,
        ];
        for map_id in &all_maps {
            let map = load_map(*map_id);
            for (ni, npc) in map.npcs.iter().enumerate() {
                let nx = npc.x as usize;
                let ny = npc.y as usize;
                assert!(
                    nx < map.width && ny < map.height,
                    "NPC #{} in {:?} at ({},{}) is out of bounds ({}x{})",
                    ni, map_id, nx, ny, map.width, map.height
                );
                let idx = ny * map.width + nx;
                let coll = map.collision[idx];
                assert!(
                    coll == C_WALK || coll == C_COUNTER || coll == C_TALL,
                    "NPC #{} in {:?} at ({},{}) on collision {} — expected C_WALK(0), C_TALL(2), or C_COUNTER(6).",
                    ni, map_id, nx, ny, coll
                );
            }
        }
    }

    #[test]
    fn test_tile_at_in_bounds() {
        let map = load_map(MapId::NewBarkTown);
        assert!(map.tile_at(0, 0).is_some());
        assert!(map.tile_at(19, 17).is_some());
    }

    #[test]
    fn test_tile_at_out_of_bounds() {
        let map = load_map(MapId::NewBarkTown);
        assert!(map.tile_at(20, 0).is_none());
        assert!(map.tile_at(0, 18).is_none());
    }

    #[test]
    fn test_collision_at_out_of_bounds_returns_solid() {
        let map = load_map(MapId::NewBarkTown);
        assert_eq!(map.collision_at(100, 100), CollisionType::Solid);
    }

    #[test]
    fn test_new_bark_town_has_warps() {
        let map = load_map(MapId::NewBarkTown);
        assert!(!map.warps.is_empty());
        // Should have warp to PlayerHouse1F
        assert!(map.warps.iter().any(|w| w.dest_map == MapId::PlayerHouse1F));
        // Should have warp to ElmLab
        assert!(map.warps.iter().any(|w| w.dest_map == MapId::ElmLab));
        // Should have warp to Route29
        assert!(map.warps.iter().any(|w| w.dest_map == MapId::Route29));
    }

    #[test]
    fn test_route_29_has_encounters() {
        let map = load_map(MapId::Route29);
        assert!(!map.encounters.is_empty());
        // Should have Pidgey
        assert!(map.encounters.iter().any(|e| e.species_id == PIDGEY));
        // Should have Rattata
        assert!(map.encounters.iter().any(|e| e.species_id == RATTATA));
    }

    #[test]
    fn test_route_29_has_tall_grass() {
        let map = load_map(MapId::Route29);
        // There should be at least some tall grass tiles
        let tall_grass_count = map.collision.iter().filter(|&&c| c == C_TALL).count();
        assert!(tall_grass_count > 0, "Route 29 should have tall grass");
    }

    #[test]
    fn test_towns_have_no_encounters() {
        let nbt = load_map(MapId::NewBarkTown);
        let cgc = load_map(MapId::CherrygroveCity);
        assert!(nbt.encounters.is_empty());
        assert!(cgc.encounters.is_empty());
    }

    #[test]
    fn test_roll_encounter_route29() {
        let map = load_map(MapId::Route29);
        // Roll = 0.0 should return something
        let result = map.roll_encounter(0.0, 0.5);
        assert!(result.is_some());
        // Roll = 0.99 should return something
        let result = map.roll_encounter(0.99, 0.5);
        assert!(result.is_some());
    }

    #[test]
    fn test_roll_encounter_empty_map() {
        let map = load_map(MapId::NewBarkTown);
        let result = map.roll_encounter(0.5, 0.5);
        assert!(result.is_none());
    }

    #[test]
    fn test_warp_connectivity() {
        // NewBarkTown -> Route29 -> CherrygroveCity -> Route30 should all connect
        let nbt = load_map(MapId::NewBarkTown);
        let r29 = load_map(MapId::Route29);
        let cgc = load_map(MapId::CherrygroveCity);
        let r30 = load_map(MapId::Route30);

        // NBT has exit to Route29
        assert!(nbt.warps.iter().any(|w| w.dest_map == MapId::Route29));
        // Route29 has exit back to NBT and forward to Cherrygrove
        assert!(r29.warps.iter().any(|w| w.dest_map == MapId::NewBarkTown));
        assert!(r29.warps.iter().any(|w| w.dest_map == MapId::CherrygroveCity));
        // Cherrygrove has exit back to Route29 and north to Route30
        assert!(cgc.warps.iter().any(|w| w.dest_map == MapId::Route29));
        assert!(cgc.warps.iter().any(|w| w.dest_map == MapId::Route30));
        // Route30 has exit south to CherrygroveCity
        assert!(r30.warps.iter().any(|w| w.dest_map == MapId::CherrygroveCity));
    }

    #[test]
    fn test_interior_maps_exit_correctly() {
        // Player house exits to New Bark Town
        let ph = load_map(MapId::PlayerHouse1F);
        assert!(ph.warps.iter().any(|w| w.dest_map == MapId::NewBarkTown));

        // Elm lab exits to New Bark Town
        let lab = load_map(MapId::ElmLab);
        assert!(lab.warps.iter().any(|w| w.dest_map == MapId::NewBarkTown));

        // Pokemon Center exits to Cherrygrove
        let pc = load_map(MapId::PokemonCenter);
        assert!(pc.warps.iter().any(|w| w.dest_map == MapId::CherrygroveCity));
    }

    #[test]
    fn test_collision_type_from_u8() {
        assert_eq!(CollisionType::from_u8(0), CollisionType::Walkable);
        assert_eq!(CollisionType::from_u8(1), CollisionType::Solid);
        assert_eq!(CollisionType::from_u8(2), CollisionType::TallGrass);
        assert_eq!(CollisionType::from_u8(3), CollisionType::Water);
        assert_eq!(CollisionType::from_u8(4), CollisionType::Warp);
        assert_eq!(CollisionType::from_u8(5), CollisionType::Ledge);
        assert_eq!(CollisionType::from_u8(6), CollisionType::Counter);
        assert_eq!(CollisionType::from_u8(7), CollisionType::Sign);
        assert_eq!(CollisionType::from_u8(8), CollisionType::Ice);
        assert_eq!(CollisionType::from_u8(255), CollisionType::Solid); // unknown -> Solid
    }

    #[test]
    fn test_elm_lab_has_elm_npc() {
        let map = load_map(MapId::ElmLab);
        assert!(map.npcs.iter().any(|n| n.sprite_id == 0)); // Elm = sprite 0
    }

    #[test]
    fn test_player_house_has_mom() {
        let map = load_map(MapId::PlayerHouse1F);
        assert!(map.npcs.iter().any(|n| n.sprite_id == 1)); // Mom = sprite 1
    }

    #[test]
    fn test_pokemon_center_has_nurse() {
        let map = load_map(MapId::PokemonCenter);
        assert!(map.npcs.iter().any(|n| n.sprite_id == 4)); // Nurse = sprite 4
    }

    #[test]
    fn test_route_30_has_encounters() {
        let map = load_map(MapId::Route30);
        assert!(!map.encounters.is_empty());
        // Should have Caterpie, Weedle, Pidgey, Rattata, Bellsprout
        assert!(map.encounters.iter().any(|e| e.species_id == CATERPIE));
        assert!(map.encounters.iter().any(|e| e.species_id == WEEDLE));
        assert!(map.encounters.iter().any(|e| e.species_id == PIDGEY));
        assert!(map.encounters.iter().any(|e| e.species_id == RATTATA));
        assert!(map.encounters.iter().any(|e| e.species_id == BELLSPROUT));
    }

    #[test]
    fn test_route_30_has_tall_grass() {
        let map = load_map(MapId::Route30);
        let tall_grass_count = map.collision.iter().filter(|&&c| c == C_TALL).count();
        assert!(tall_grass_count > 0, "Route 30 should have tall grass");
    }

    #[test]
    fn test_route_30_has_trainers() {
        let map = load_map(MapId::Route30);
        let trainer_count = map.npcs.iter().filter(|n| n.is_trainer).count();
        assert!(trainer_count >= 2, "Route 30 should have at least 2 trainers");
    }

    #[test]
    fn test_route_30_has_sign() {
        let map = load_map(MapId::Route30);
        let sign_count = map.collision.iter().filter(|&&c| c == C_SIGN).count();
        assert!(sign_count > 0, "Route 30 should have a sign");
    }

    #[test]
    fn test_route_30_has_water() {
        let map = load_map(MapId::Route30);
        let water_count = map.collision.iter().filter(|&&c| c == C_WATER).count();
        assert!(water_count > 0, "Route 30 should have water (pond)");
    }

    #[test]
    fn test_route_30_has_ledges() {
        let map = load_map(MapId::Route30);
        let ledge_count = map.collision.iter().filter(|&&c| c == C_LEDGE).count();
        assert!(ledge_count > 0, "Route 30 should have ledges");
    }

    #[test]
    fn test_roll_encounter_route30() {
        let map = load_map(MapId::Route30);
        let result = map.roll_encounter(0.0, 0.5);
        assert!(result.is_some());
        let result = map.roll_encounter(0.99, 0.5);
        assert!(result.is_some());
    }

    #[test]
    fn test_cherrygrove_has_route30_warp() {
        let map = load_map(MapId::CherrygroveCity);
        assert!(map.warps.iter().any(|w| w.dest_map == MapId::Route30));
    }

    /// Phase 0B (GUIDE.md): every warp destination must land on C_WALK.
    /// Destinations on C_WARP cause immediate re-warp loops.
    /// Destinations on C_SOLID/C_WATER trap the player.
    #[test]
    fn test_all_warps_valid() {
        let all_maps = vec![
            MapId::NewBarkTown, MapId::Route29, MapId::CherrygroveCity,
            MapId::Route30, MapId::Route31, MapId::VioletCity, MapId::VioletGym,
            MapId::SproutTower1F, MapId::SproutTower2F, MapId::SproutTower3F, MapId::PlayerHouse1F, MapId::PlayerHouse2F,
            MapId::ElmLab, MapId::PokemonCenter, MapId::Route32, MapId::UnionCave,
            MapId::GenericHouse, MapId::Route33, MapId::AzaleaTown, MapId::AzaleaGym,
            MapId::IlexForest, MapId::Route34, MapId::GoldenrodCity, MapId::GoldenrodGym,
            MapId::Route35, MapId::NationalPark, MapId::Route36, MapId::Route37,
            MapId::EcruteakCity, MapId::BurnedTower, MapId::EcruteakGym,
            MapId::Route38, MapId::Route39, MapId::OlivineCity, MapId::OlivineGym,
            MapId::OlivineLighthouse,
            MapId::Route40, MapId::CianwoodCity, MapId::CianwoodGym,
            MapId::Route42, MapId::MahoganyTown, MapId::MahoganyGym,
            MapId::Route43, MapId::LakeOfRage,
            MapId::Route44, MapId::IcePath, MapId::BlackthornCity, MapId::BlackthornGym,
            MapId::Route45, MapId::Route46,
            MapId::Route27, MapId::Route26,
            MapId::VictoryRoad, MapId::IndigoPlateau,
            MapId::EliteFourWill, MapId::EliteFourKoga,
            MapId::EliteFourBruno, MapId::EliteFourKaren,
            MapId::ChampionLance,
        ];
        let mut errors = Vec::new();
        for &src_id in &all_maps {
            let src = load_map(src_id);
            for (wi, warp) in src.warps.iter().enumerate() {
                let dest = load_map(warp.dest_map);
                let dx = warp.dest_x as usize;
                let dy = warp.dest_y as usize;
                if dx >= dest.width || dy >= dest.height {
                    errors.push(format!(
                        "{:?} warp #{} → {:?} ({},{}) — OUT OF BOUNDS (map {}x{})",
                        src_id, wi, warp.dest_map, dx, dy, dest.width, dest.height
                    ));
                    continue;
                }
                let coll = dest.collision[dy * dest.width + dx];
                if coll != C_WALK && coll != C_TALL && coll != C_ICE {
                    let name = match coll {
                        C_SOLID => "C_SOLID", C_TALL => "C_TALL",
                        C_WATER => "C_WATER", C_WARP => "C_WARP",
                        C_LEDGE => "C_LEDGE", C_COUNTER => "C_COUNTER",
                        C_SIGN => "C_SIGN", C_ICE => "C_ICE", _ => "UNKNOWN",
                    };
                    errors.push(format!(
                        "{:?} warp #{} → {:?} ({},{}) — lands on {} (expected C_WALK)",
                        src_id, wi, warp.dest_map, dx, dy, name
                    ));
                }
            }
        }
        assert!(errors.is_empty(), "Warp validation failures:\n{}", errors.join("\n"));
    }
}
