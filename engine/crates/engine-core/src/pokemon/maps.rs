// AI-INSTRUCTIONS: Pokemon map data module. Contains all map definitions for a
// Pokemon Gold/Silver/Crystal recreation. Maps are tile-based grids stored row-by-row.
// Each map has visual tile IDs, collision data, warp points, NPC definitions, and
// wild encounter tables. Use load_map(MapId) to get a complete MapData struct.
// Tile IDs correspond to sprite indices in sprites.rs. Collision values map to
// CollisionType variants. Maps: NewBarkTown (20x18), Route29 (30x14),
// CherrygroveCity (20x18), Route30 (30x18), Route31 (30x14),
// VioletCity (24x18), VioletGym (10x10), SproutTower (14x14),
// PlayerHouse1F (10x8), PlayerHouse2F (10x8), ElmLab (10x10), PokemonCenter (10x8),
// Route32 (20x30), UnionCave (16x16).

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

// ─── Collision Constants ────────────────────────────────
const C_WALK: u8 = 0;  // Walkable
const C_SOLID: u8 = 1; // Solid
const C_TALL: u8 = 2;  // TallGrass
const C_WATER: u8 = 3; // Water
const C_WARP: u8 = 4;  // Warp
const C_LEDGE: u8 = 5; // Ledge
const C_COUNTER: u8 = 6; // Counter
const C_SIGN: u8 = 7;  // Sign

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
    SproutTower,
    PlayerHouse1F,
    PlayerHouse2F,
    ElmLab,
    PokemonCenter,
    Route32,
    UnionCave,
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
        MapId::SproutTower => build_sprout_tower(),
        MapId::PlayerHouse1F => build_player_house_1f(),
        MapId::PlayerHouse2F => build_player_house_2f(),
        MapId::ElmLab => build_elm_lab(),
        MapId::PokemonCenter => build_pokemon_center(),
        MapId::Route32 => build_route_32(),
        MapId::UnionCave => build_union_cave(),
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
        // Row 10: grass with path going right to exit
        GRASS, GRASS, GRASS, GRASS, GRASS, PATH, PATH, PATH, PATH, PATH,
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
        // Row 10: path going right to exit (rightmost = warp to Route 29)
        C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK, C_WALK,
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
        WarpData { x: 5, y: 4, dest_map: MapId::PlayerHouse1F, dest_x: 4, dest_y: 7 },
        // NPC house door (not implemented interior, warp to self for now)
        WarpData { x: 12, y: 4, dest_map: MapId::NewBarkTown, dest_x: 12, dest_y: 5 },
        // Elm's Lab door -> ElmLab
        WarpData { x: 17, y: 4, dest_map: MapId::ElmLab, dest_x: 4, dest_y: 9 },
        // Right edge exit -> Route 29
        WarpData { x: 19, y: 10, dest_map: MapId::Route29, dest_x: 0, dest_y: 7 },
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
                "PROF.ELM is always in",
                "his lab researching.",
                "He's a Pokemon Professor!",
            ],
            is_trainer: false, is_mart: false, wanders: true, trainer_team: &[],
        },
        // Lass near flowers
        NpcDef {
            x: 3, y: 11, sprite_id: 3, facing: Direction::Right,
            dialogue: &[
                "Have you been to",
                "CHERRYGROVE CITY?",
                "It's just past ROUTE 29.",
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
        encounters: vec![], // no wild encounters in town
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
        WarpData { x: 0, y: 7, dest_map: MapId::NewBarkTown, dest_x: 19, dest_y: 10 },
        // Right edge -> Cherrygrove City (left entry)
        WarpData { x: 29, y: 7, dest_map: MapId::CherrygroveCity, dest_x: 0, dest_y: 10 },
    ];

    let npcs = vec![
        // Guide Gent NPC
        NpcDef {
            x: 5, y: 8, sprite_id: 5, facing: Direction::Up,
            dialogue: &[
                "If your POKEMON are weak",
                "and about to faint, you",
                "should heal them at a",
                "POKEMON CENTER.",
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
        WarpData { x: 0, y: 10, dest_map: MapId::Route29, dest_x: 29, dest_y: 7 },
        // Pokemon Center door
        WarpData { x: 7, y: 4, dest_map: MapId::PokemonCenter, dest_x: 4, dest_y: 7 },
        // NPC house door (upper right)
        WarpData { x: 15, y: 4, dest_map: MapId::CherrygroveCity, dest_x: 15, dest_y: 5 },
        // NPC house door (lower right)
        WarpData { x: 16, y: 8, dest_map: MapId::CherrygroveCity, dest_x: 16, dest_y: 9 },
        // North exit to Route 30 (column 9)
        WarpData { x: 9, y: 0, dest_map: MapId::Route30, dest_x: 14, dest_y: 17 },
        // North exit to Route 30 (column 10)
        WarpData { x: 10, y: 0, dest_map: MapId::Route30, dest_x: 15, dest_y: 17 },
    ];

    let npcs = vec![
        // Guide Gent (old man near pokecenter sign)
        NpcDef {
            x: 5, y: 9, sprite_id: 5, facing: Direction::Right,
            dialogue: &[
                "CHERRYGROVE CITY",
                "The city of cute, fragrant",
                "flowers.",
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
                "Did you just come from",
                "NEW BARK TOWN?",
                "That's a long walk!",
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
        encounters: vec![], // no wild encounters in town
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
        // Row 2: trees | grass | tall grass left patch | path | grass | pond | east exit
        TREE_TOP,TREE_TOP,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        PATH,PATH,PATH,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,WATER,WATER,GRASS,PATH,PATH,
        // Row 3: trees | grass | tall grass left patch | path | grass | pond | east exit
        TREE_BOTTOM,TREE_BOTTOM,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        PATH,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,WATER,WATER,GRASS,PATH,PATH,
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
        // Row 10: trees | tall grass | path east-west stretch | tall grass
        TREE_TOP,TREE_TOP,GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,PATH,PATH,PATH,PATH,PATH,GRASS,GRASS,
        GRASS,TALL_GRASS,TALL_GRASS,TALL_GRASS,GRASS,GRASS,GRASS,GRASS,TREE_TOP,TREE_TOP,
        // Row 11: trees | grass | path goes south | grass | ledge
        TREE_BOTTOM,TREE_BOTTOM,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,GRASS,
        GRASS,GRASS,GRASS,PATH,GRASS,GRASS,GRASS,PATH,GRASS,GRASS,
        GRASS,GRASS,GRASS,GRASS,GRASS,LEDGE,LEDGE,LEDGE,TREE_BOTTOM,TREE_BOTTOM,
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
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
        TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,PATH,PATH,PATH,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,TREE_BOTTOM,
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
        // Row 2: trees | grass | tall grass | path | grass | pond | east exit
        C_SOLID,C_SOLID,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WALK,C_WALK,C_WARP,
        // Row 3: trees | grass | tall grass | path | grass | pond | east exit
        C_SOLID,C_SOLID,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WATER,C_WATER,C_WALK,C_WALK,C_WARP,
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
        // Row 10: trees | tall grass | path east-west | tall grass
        C_SOLID,C_SOLID,C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_TALL,C_TALL,C_TALL,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,C_SOLID,
        // Row 11: trees | grass | path | ledge
        C_SOLID,C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,
        C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_LEDGE,C_LEDGE,C_LEDGE,C_SOLID,C_SOLID,
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
        // Row 17: trees with south exit warp to CherrygroveCity
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_WARP,C_WARP,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "Route30 tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "Route30 collision count mismatch");

    let warps = vec![
        // South exit -> CherrygroveCity north (column 13 -> Cherrygrove col 9)
        WarpData { x: 13, y: 17, dest_map: MapId::CherrygroveCity, dest_x: 9, dest_y: 1 },
        // South exit -> CherrygroveCity north (column 14 -> Cherrygrove col 9)
        WarpData { x: 14, y: 17, dest_map: MapId::CherrygroveCity, dest_x: 9, dest_y: 1 },
        // South exit -> CherrygroveCity north (column 15 -> Cherrygrove col 10)
        WarpData { x: 15, y: 17, dest_map: MapId::CherrygroveCity, dest_x: 10, dest_y: 1 },
        // East exit -> Route 31 (row 2)
        WarpData { x: 29, y: 2, dest_map: MapId::Route31, dest_x: 0, dest_y: 7 },
        // East exit -> Route 31 (row 3)
        WarpData { x: 29, y: 3, dest_map: MapId::Route31, dest_x: 0, dest_y: 8 },
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
        WarpData { x: 0, y: 7, dest_map: MapId::Route30, dest_x: 28, dest_y: 2 },
        WarpData { x: 0, y: 8, dest_map: MapId::Route30, dest_x: 28, dest_y: 3 },
        // Right edge -> Violet City (west entry)
        WarpData { x: 29, y: 7, dest_map: MapId::VioletCity, dest_x: 0, dest_y: 10 },
        WarpData { x: 29, y: 8, dest_map: MapId::VioletCity, dest_x: 0, dest_y: 11 },
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
        WarpData { x: 0, y: 10, dest_map: MapId::Route31, dest_x: 29, dest_y: 7 },
        // West edge entry from Route 31 (row 11)
        WarpData { x: 0, y: 11, dest_map: MapId::Route31, dest_x: 29, dest_y: 8 },
        // Pokemon Center door -> PokemonCenter interior
        WarpData { x: 5, y: 11, dest_map: MapId::PokemonCenter, dest_x: 4, dest_y: 7 },
        // Poke Mart door (not implemented interior, warp to self)
        WarpData { x: 15, y: 11, dest_map: MapId::VioletCity, dest_x: 15, dest_y: 12 },
        // Violet Gym door -> VioletGym interior
        WarpData { x: 10, y: 6, dest_map: MapId::VioletGym, dest_x: 5, dest_y: 9 },
        // Sprout Tower door -> SproutTower interior
        WarpData { x: 18, y: 4, dest_map: MapId::SproutTower, dest_x: 7, dest_y: 13 },
        // South exit to Route 32 (x=11,12 at bottom edge)
        WarpData { x: 11, y: 17, dest_map: MapId::Route32, dest_x: 9, dest_y: 0 },
        WarpData { x: 12, y: 17, dest_map: MapId::Route32, dest_x: 10, dest_y: 0 },
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
                "VIOLET CITY is known for",
                "SPROUT TOWER to the north.",
                "Many trainers go there",
                "to train.",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // Youngster near gym
        NpcDef {
            x: 7, y: 7, sprite_id: 2, facing: Direction::Right,
            dialogue: &[
                "The GYM LEADER FALKNER",
                "uses BIRD POKEMON!",
                "Make sure you're ready!",
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
        encounters: vec![], // no wild encounters in town
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
        // Row 0: top wall (black border)
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: back wall
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 2: Falkner stands here (center)
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 3: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 4: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 5: trainee stands here
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 6: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 7: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 8: floor near entrance
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
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
        music_id: 8,
    }
}

// ─── Sprout Tower (14x14) ───────────────────────────────
// Indoor tower with monks/NPCs and wild encounters (Rattata, Gastly).

fn build_sprout_tower() -> MapData {
    let w: usize = 14;
    let h: usize = 14;

    #[rustfmt::skip]
    let tiles: Vec<u8> = vec![
        // Row 0: top wall (black border)
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: back wall with bookshelves
        BLACK,FLOOR,BOOKSHELF,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BOOKSHELF,FLOOR,BLACK,
        // Row 2: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 3: floor with center pillar
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,TABLE,TABLE,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 4: floor with center pillar
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,TABLE,TABLE,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 5: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 6: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 7: floor with bookshelves on sides
        BLACK,BOOKSHELF,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BOOKSHELF,BLACK,
        // Row 8: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 9: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 10: floor with center pillar
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,TABLE,TABLE,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 11: floor
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 12: floor near entrance
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 13: bottom wall with door
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,DOOR,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
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
        // Row 12: floor
        C_SOLID,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_WALK,C_SOLID,
        // Row 13: wall + door warp
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "SproutTower tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "SproutTower collision count mismatch");

    let warps = vec![
        // Door exits to Violet City (in front of Sprout Tower)
        WarpData { x: 7, y: 13, dest_map: MapId::VioletCity, dest_x: 18, dest_y: 5 },
    ];

    let npcs = vec![
        // Monk near the top
        NpcDef {
            x: 4, y: 2, sprite_id: 5, facing: Direction::Down,
            dialogue: &[
                "SPROUT TOWER is a place",
                "of training.",
                "The shaking pillar is a",
                "100-foot-tall BELLSPROUT.",
                "It sways to ward off",
                "earthquakes.",
            ],
            is_trainer: false, is_mart: false, wanders: false, trainer_team: &[],
        },
        // Monk near the middle
        NpcDef {
            x: 10, y: 8, sprite_id: 5, facing: Direction::Left,
            dialogue: &[
                "BELLSPROUT is revered in",
                "this tower.",
                "We train ourselves and",
                "our POKEMON here daily.",
                "Show me your strength!",
            ],
            is_trainer: true, is_mart: false, wanders: false,
            trainer_team: &[
                TrainerPokemon { species_id: BELLSPROUT, level: 7 },
                TrainerPokemon { species_id: BELLSPROUT, level: 7 },
            ],
        },
    ];

    let encounters = vec![
        EncounterEntry { species_id: RATTATA, min_level: 3, max_level: 5, weight: 60 },
        EncounterEntry { species_id: GASTLY, min_level: 3, max_level: 5, weight: 40 },
    ];

    MapData {
        id: MapId::SproutTower,
        name: "SPROUT TOWER",
        width: w,
        height: h,
        tiles,
        collision,
        warps,
        npcs,
        encounters,
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
        // Row 7: bottom wall with door
        BLACK,BLACK,BLACK,BLACK,DOOR,BLACK,BLACK,BLACK,BLACK,BLACK,
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
        // Row 7: wall solid, door warp
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "PlayerHouse1F tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "PlayerHouse1F collision count mismatch");

    let warps = vec![
        // Door exits to New Bark Town (in front of player house)
        WarpData { x: 4, y: 7, dest_map: MapId::NewBarkTown, dest_x: 5, dest_y: 5 },
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
        // Row 9: bottom wall with door
        BLACK,BLACK,BLACK,BLACK,DOOR,BLACK,BLACK,BLACK,BLACK,BLACK,
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
        // Row 9: wall + door warp
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "ElmLab tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "ElmLab collision count mismatch");

    let warps = vec![
        // Door exits to New Bark Town (in front of lab)
        WarpData { x: 4, y: 9, dest_map: MapId::NewBarkTown, dest_x: 17, dest_y: 5 },
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
        // Row 7: bottom wall with door
        BLACK,BLACK,BLACK,BLACK,DOOR,BLACK,BLACK,BLACK,BLACK,BLACK,
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
        // Row 7: wall + door warp
        C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_WARP,C_SOLID,C_SOLID,C_SOLID,C_SOLID,C_SOLID,
    ];

    debug_assert_eq!(tiles.len(), w * h, "PokemonCenter tiles count mismatch");
    debug_assert_eq!(collision.len(), w * h, "PokemonCenter collision count mismatch");

    let warps = vec![
        // Door exits to Cherrygrove City (in front of pokecenter)
        WarpData { x: 4, y: 7, dest_map: MapId::CherrygroveCity, dest_x: 7, dest_y: 5 },
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
        WarpData { x: 9, y: 29, dest_map: MapId::UnionCave, dest_x: 7, dest_y: 1 },
        WarpData { x: 10, y: 29, dest_map: MapId::UnionCave, dest_x: 8, dest_y: 1 },
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
        // Row 0: top wall (all black)
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 1: back wall with entrance gap at x=7,8
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,PATH,PATH,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
        // Row 2: cave interior opening up
        BLACK,BLACK,BLACK,BLACK,FLOOR,FLOOR,FLOOR,PATH,PATH,FLOOR,FLOOR,FLOOR,BLACK,BLACK,BLACK,BLACK,
        // Row 3: wider cave area
        BLACK,BLACK,BLACK,FLOOR,FLOOR,FLOOR,FLOOR,PATH,PATH,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,BLACK,BLACK,
        // Row 4: main cave area
        BLACK,BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,PATH,PATH,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,BLACK,
        // Row 5: wide area with side alcove left
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,PATH,PATH,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 6: wide area
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,PATH,PATH,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 7: path bends, open area
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,PATH,PATH,PATH,PATH,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 8: wide chamber
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,PATH,FLOOR,FLOOR,PATH,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 9: path continues south
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,PATH,PATH,PATH,PATH,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 10: wide area
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,PATH,PATH,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 11: narrowing cave
        BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,PATH,PATH,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,
        // Row 12: cave narrows
        BLACK,BLACK,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,PATH,PATH,FLOOR,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,BLACK,
        // Row 13: narrower
        BLACK,BLACK,BLACK,FLOOR,FLOOR,FLOOR,FLOOR,PATH,PATH,FLOOR,FLOOR,FLOOR,FLOOR,BLACK,BLACK,BLACK,
        // Row 14: near south exit
        BLACK,BLACK,BLACK,BLACK,FLOOR,FLOOR,FLOOR,PATH,PATH,FLOOR,FLOOR,FLOOR,BLACK,BLACK,BLACK,BLACK,
        // Row 15: south wall with exit gap at x=7,8 (warp back to Route 32)
        BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,PATH,PATH,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,BLACK,
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
        // South exit warps back to Route 32 (dead end loop for now)
        WarpData { x: 7, y: 15, dest_map: MapId::Route32, dest_x: 9, dest_y: 28 },
        WarpData { x: 8, y: 15, dest_map: MapId::Route32, dest_x: 10, dest_y: 28 },
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
            CollisionType::Walkable | CollisionType::TallGrass | CollisionType::Warp | CollisionType::Ledge
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
        if self.encounters.is_empty() {
            return None;
        }

        let total_weight: u32 = self.encounters.iter().map(|e| e.weight as u32).sum();
        if total_weight == 0 {
            return None;
        }

        let target = (roll * total_weight as f64) as u32;
        let mut cumulative: u32 = 0;

        for entry in &self.encounters {
            cumulative += entry.weight as u32;
            if target < cumulative {
                let level_range = (entry.max_level - entry.min_level) as f64;
                let level = entry.min_level + (level_roll * (level_range + 1.0)).min(level_range) as u8;
                return Some((entry.species_id, level));
            }
        }

        // Fallback: last encounter
        if let Some(last) = self.encounters.last() {
            Some((last.species_id, last.min_level))
        } else {
            None
        }
    }
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
            MapId::SproutTower,
            MapId::PlayerHouse1F,
            MapId::PlayerHouse2F,
            MapId::ElmLab,
            MapId::PokemonCenter,
            MapId::Route32,
            MapId::UnionCave,
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
}
