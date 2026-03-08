// AI-INSTRUCTIONS: pokemonv2/maps.rs — Map system. Imports data.rs only.
// MapId enum, MapData struct, all warp/NPC/event types, load_map() returning data for Sprint 1 maps.
// Coordinates verified against pokecrystal-master .asm files via SPRINT1_POKEMON_REFERENCE.md.
// Import graph: maps.rs <- data.rs ONLY (no events.rs import — uses script_id: u16 indirection)

use super::data::{Direction, NpcState, SpeciesId};

// ── Collision Tile Constants ──────────────────────────────────────────────────
pub const C_FLOOR: u8 = 0;   // walkable
pub const C_WALL: u8 = 1;    // solid
pub const C_WATER: u8 = 2;   // impassable without Surf
pub const C_WARP: u8 = 3;    // triggers warp check (mats, doors)
pub const C_COUNTER: u8 = 4; // interact across (Elm's desk)

// ── Enums ────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub enum MapId {
    NewBarkTown,
    PlayersHouse1F,
    PlayersHouse2F,
    ElmsLab,
    ElmsHouse,
    PlayersNeighborsHouse,
    Route29,
    Route27,
}

/// NPC movement patterns. Matches pokecrystal's SPRITEMOVEDATA_ constants.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NpcMoveType {
    Still,
    Standing(Direction),  // faces one direction, never moves
    SpinRandom,           // rotates randomly
    WalkUpDown,           // walks up and down within range
    WalkLeftRight,        // walks left and right within range
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BgEventKind {
    Read,            // BGEVENT_READ — interact facing any direction
    FaceUp,          // BGEVENT_UP — must face up
    FaceDown,        // BGEVENT_DOWN — must face down
    IfSet(u16),      // BGEVENT_IFSET — only active when flag is set
}

// ── Structs ───────────────────────────────────────────────────────────────────

pub struct MapData {
    pub id: MapId,
    pub name: &'static str,
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<u8>,
    pub collision: Vec<u8>,
    pub warps: Vec<WarpDef>,
    pub npcs: Vec<NpcDef>,
    pub coord_events: Vec<CoordEvent>,
    pub bg_events: Vec<BgEvent>,
    pub wild_encounters: Vec<WildEncounter>,
    pub connections: MapConnections,
    pub music_id: u8,
}

pub struct WarpDef {
    pub x: i32,
    pub y: i32,
    pub dest_map: MapId,
    pub dest_warp_id: u8,
}

pub struct MapConnection {
    pub direction: Direction,
    pub dest_map: MapId,
    pub offset: i8,
}

pub struct MapConnections {
    pub north: Option<MapConnection>,
    pub south: Option<MapConnection>,
    pub east: Option<MapConnection>,
    pub west: Option<MapConnection>,
}

impl MapConnections {
    pub fn none() -> Self {
        Self { north: None, south: None, east: None, west: None }
    }
}

pub struct NpcDef {
    pub x: i32,
    pub y: i32,
    pub sprite_id: u8,
    pub move_type: NpcMoveType,
    pub script_id: u16,          // index into script registry in events.rs
    pub event_flag: Option<u16>, // flag index; controls NPC visibility
    pub event_flag_show: bool,   // false=hide when set, true=show when set
    pub palette: u8,
    pub facing: Direction,
    pub name: &'static str,
}

pub struct CoordEvent {
    pub x: i32,
    pub y: i32,
    pub scene_id: u8,
    pub script_id: u16,
}

pub struct BgEvent {
    pub x: i32,
    pub y: i32,
    pub kind: BgEventKind,
    pub script_id: u16,
}

pub struct WildEncounter {
    pub species: SpeciesId,
    pub min_level: u8,
    pub max_level: u8,
    pub rate: u8,
}

// ── NPC State Initialization ──────────────────────────────────────────────────

/// Build runtime NPC states from map definition. All NPCs start visible;
/// event-flag visibility is applied by mod.rs after loading.
pub fn init_npc_states(map: &MapData) -> Vec<NpcState> {
    map.npcs.iter().map(|def| NpcState {
        x: def.x,
        y: def.y,
        facing: def.facing,
        walk_offset: 0.0,
        is_walking: false,
        visible: true,  // refined by refresh_npc_visibility in mod.rs
        wander_timer: 0.0,
        emote: None,
    }).collect()
}

// ── Map Functions ─────────────────────────────────────────────────────────────

/// Check if tile at (x, y) is walkable (floor or warp tile).
pub fn is_walkable(map: &MapData, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 || x >= map.width || y >= map.height {
        return false;
    }
    let idx = (y * map.width + x) as usize;
    let c = map.collision[idx];
    c == C_FLOOR || c == C_WARP
}

/// Find warp at (x, y) if any.
pub fn find_warp(map: &MapData, x: i32, y: i32) -> Option<&WarpDef> {
    map.warps.iter().find(|w| w.x == x && w.y == y)
}

/// Find coord_event at (x, y) matching the given scene_id.
pub fn find_coord_event(map: &MapData, x: i32, y: i32, scene_id: u8) -> Option<&CoordEvent> {
    map.coord_events.iter().find(|e| e.x == x && e.y == y && e.scene_id == scene_id)
}

/// Find bg_event at (x, y) matching the player's facing direction.
pub fn find_bg_event(map: &MapData, x: i32, y: i32, facing: Direction) -> Option<&BgEvent> {
    map.bg_events.iter().find(|e| {
        e.x == x && e.y == y && match e.kind {
            BgEventKind::Read => true,
            BgEventKind::FaceUp => facing == Direction::Up,
            BgEventKind::FaceDown => facing == Direction::Down,
            BgEventKind::IfSet(_) => true, // flag check done by caller
        }
    })
}

/// Resolve player spawn position from a dest_warp_id index.
/// Player spawns 1 tile south of (below) the warp tile (stepped out of a door).
pub fn resolve_warp_position(map: &MapData, dest_warp_id: u8) -> (i32, i32) {
    let idx = dest_warp_id as usize;
    if idx < map.warps.len() {
        let w = &map.warps[idx];
        (w.x, (w.y + 1).min(map.height - 1))
    } else if !map.warps.is_empty() {
        let w = &map.warps[0];
        (w.x, (w.y + 1).min(map.height - 1))
    } else {
        (map.width / 2, map.height / 2)
    }
}

// ── Map Load ──────────────────────────────────────────────────────────────────

/// Load map data for the given MapId.
pub fn load_map(id: MapId) -> MapData {
    match id {
        MapId::PlayersHouse2F       => build_players_house_2f(),
        MapId::PlayersHouse1F       => build_players_house_1f(),
        MapId::NewBarkTown          => build_new_bark_town(),
        MapId::ElmsLab              => build_elms_lab(),
        MapId::ElmsHouse            => build_stub_house(MapId::NewBarkTown, 3, "ELM'S HOUSE", 3),
        MapId::PlayersNeighborsHouse => build_stub_house(MapId::NewBarkTown, 2, "NEIGHBOR'S HOUSE", 2),
        MapId::Route29              => build_stub_route(MapId::NewBarkTown, "ROUTE 29"),
        MapId::Route27              => build_stub_route(MapId::NewBarkTown, "ROUTE 27"),
    }
}

// ── Map Builder Helpers ───────────────────────────────────────────────────────

fn fill_room(w: i32, h: i32, floor_tile: u8) -> (Vec<u8>, Vec<u8>) {
    let total = (w * h) as usize;
    let mut tiles = vec![floor_tile; total];
    let mut col = vec![C_FLOOR; total];

    // Perimeter walls
    for x in 0..w {
        let top = x as usize;
        let bot = ((h - 1) * w + x) as usize;
        tiles[top] = 2; tiles[bot] = 2;
        col[top] = C_WALL; col[bot] = C_WALL;
    }
    for y in 0..h {
        let left = (y * w) as usize;
        let right = (y * w + w - 1) as usize;
        tiles[left] = 2; tiles[right] = 2;
        col[left] = C_WALL; col[right] = C_WALL;
    }
    (tiles, col)
}

fn set_tile(tiles: &mut Vec<u8>, col: &mut Vec<u8>, w: i32, x: i32, y: i32, tile: u8, collision: u8) {
    let idx = (y * w + x) as usize;
    if idx < tiles.len() {
        tiles[idx] = tile;
        col[idx] = collision;
    }
}

// ── PlayersHouse2F (8 x 6) ───────────────────────────────────────────────────
// Source: pokecrystal-master/maps/PlayersHouse2F.asm

fn build_players_house_2f() -> MapData {
    let (w, h) = (8i32, 6i32);
    let (mut tiles, mut col) = fill_room(w, h, 4); // tile 4 = floor beige

    // Stair warp tile at (7, 0) — top-right corner
    // Also make (7, 1) walkable so player can approach the stairs from below
    set_tile(&mut tiles, &mut col, w, 7, 0, 6, C_WARP);
    set_tile(&mut tiles, &mut col, w, 7, 1, 4, C_FLOOR);

    // Furniture tiles (C_WALL = impassable objects)
    // Console at (4,2), Doll_1 (4,4), Doll_2 (5,4), BigDoll (0,1)
    set_tile(&mut tiles, &mut col, w, 4, 2, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 4, 4, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 5, 4, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 0, 1, 5, C_WALL);
    // PC at (2,1), Radio at (3,1), Bookshelf at (5,1), Poster at (6,0)
    set_tile(&mut tiles, &mut col, w, 2, 1, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 3, 1, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 5, 1, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 6, 0, 5, C_WALL);

    MapData {
        id: MapId::PlayersHouse2F,
        name: "PLAYER'S HOUSE 2F",
        width: w,
        height: h,
        tiles,
        collision: col,
        warps: vec![
            WarpDef { x: 7, y: 0, dest_map: MapId::PlayersHouse1F, dest_warp_id: 2 },
        ],
        npcs: vec![
            NpcDef { x: 4, y: 2, sprite_id: 10, move_type: NpcMoveType::Still, script_id: 0, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "CONSOLE" },
            NpcDef { x: 4, y: 4, sprite_id: 11, move_type: NpcMoveType::Still, script_id: 0, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "DOLL_1" },
            NpcDef { x: 5, y: 4, sprite_id: 12, move_type: NpcMoveType::Still, script_id: 0, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "DOLL_2" },
            NpcDef { x: 0, y: 1, sprite_id: 13, move_type: NpcMoveType::Still, script_id: 0, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "BIG_DOLL" },
        ],
        coord_events: vec![],
        bg_events: vec![
            BgEvent { x: 2, y: 1, kind: BgEventKind::FaceUp, script_id: 18 },     // PC
            BgEvent { x: 3, y: 1, kind: BgEventKind::Read,   script_id: 19 },     // Radio
            BgEvent { x: 5, y: 1, kind: BgEventKind::Read,   script_id: 20 },     // Bookshelf
            BgEvent { x: 6, y: 0, kind: BgEventKind::Read,   script_id: 0  },     // Poster (stub)
        ],
        wild_encounters: vec![],
        connections: MapConnections::none(),
        music_id: 1,
    }
}

// ── PlayersHouse1F (10 x 8) ──────────────────────────────────────────────────
// Source: pokecrystal-master/maps/PlayersHouse1F.asm

fn build_players_house_1f() -> MapData {
    let (w, h) = (10i32, 8i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);

    // Door warps at (6,7) and (7,7) — bottom row
    set_tile(&mut tiles, &mut col, w, 6, 7, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 7, 7, 3, C_WARP);
    // Stair up at (9,0) — top-right
    set_tile(&mut tiles, &mut col, w, 9, 0, 6, C_WARP);

    // Kitchen counter tiles
    set_tile(&mut tiles, &mut col, w, 0, 1, 5, C_WALL); // Stove
    set_tile(&mut tiles, &mut col, w, 1, 1, 5, C_WALL); // Sink
    set_tile(&mut tiles, &mut col, w, 2, 1, 5, C_WALL); // Fridge
    set_tile(&mut tiles, &mut col, w, 4, 1, 5, C_WALL); // TV

    MapData {
        id: MapId::PlayersHouse1F,
        name: "PLAYER'S HOUSE 1F",
        width: w,
        height: h,
        tiles,
        collision: col,
        warps: vec![
            WarpDef { x: 6, y: 7, dest_map: MapId::NewBarkTown,    dest_warp_id: 1 }, // exit left door
            WarpDef { x: 7, y: 7, dest_map: MapId::NewBarkTown,    dest_warp_id: 1 }, // exit right door
            WarpDef { x: 9, y: 0, dest_map: MapId::PlayersHouse2F, dest_warp_id: 0 }, // stairs up
        ],
        npcs: vec![
            // Mom1: visible when EVENT_PLAYERS_HOUSE_MOM_1 (3) is NOT set
            NpcDef { x: 7, y: 4, sprite_id: 1, move_type: NpcMoveType::Standing(Direction::Left), script_id: 1, event_flag: Some(3), event_flag_show: false, palette: 0, facing: Direction::Left, name: "MOM1" },
            // Mom2 morning: visible when MOM_2 (4) IS set
            NpcDef { x: 2, y: 2, sprite_id: 1, move_type: NpcMoveType::Standing(Direction::Up), script_id: 1, event_flag: Some(4), event_flag_show: true, palette: 0, facing: Direction::Up, name: "MOM2" },
            // Mom3 day: visible when MOM_2 (4) IS set
            NpcDef { x: 7, y: 4, sprite_id: 1, move_type: NpcMoveType::Standing(Direction::Left), script_id: 1, event_flag: Some(4), event_flag_show: true, palette: 0, facing: Direction::Left, name: "MOM3" },
            // Mom4 night: visible when MOM_2 (4) IS set
            NpcDef { x: 0, y: 2, sprite_id: 1, move_type: NpcMoveType::Standing(Direction::Up), script_id: 1, event_flag: Some(4), event_flag_show: true, palette: 0, facing: Direction::Up, name: "MOM4" },
            // Pokefan_F neighbor: visible when 1F_NEIGHBOR (16) IS set
            NpcDef { x: 4, y: 4, sprite_id: 9, move_type: NpcMoveType::Standing(Direction::Right), script_id: 0, event_flag: Some(16), event_flag_show: true, palette: 0, facing: Direction::Right, name: "NEIGHBOR" },
        ],
        coord_events: vec![
            CoordEvent { x: 8, y: 4, scene_id: 0, script_id: 1 }, // SCENE_PLAYERSHOUSE1F_MEET_MOM
            CoordEvent { x: 9, y: 4, scene_id: 0, script_id: 1 },
        ],
        bg_events: vec![
            BgEvent { x: 0, y: 1, kind: BgEventKind::Read, script_id: 14 }, // Stove
            BgEvent { x: 1, y: 1, kind: BgEventKind::Read, script_id: 15 }, // Sink
            BgEvent { x: 2, y: 1, kind: BgEventKind::Read, script_id: 16 }, // Fridge
            BgEvent { x: 4, y: 1, kind: BgEventKind::Read, script_id: 17 }, // TV
        ],
        wild_encounters: vec![],
        connections: MapConnections::none(),
        music_id: 1,
    }
}

// ── NewBarkTown (18 x 20) ────────────────────────────────────────────────────
// Source: pokecrystal-master/maps/NewBarkTown.asm
// 9 blocks wide x 10 blocks tall, each block = 2 tiles = 18 x 20 tiles

fn build_new_bark_town() -> MapData {
    let (w, h) = (18i32, 20i32);
    let total = (w * h) as usize;

    // Exterior map: grass (0), path (1), walls (2), water (3), warp mats (3/C_WARP)
    let mut tiles = vec![0u8; total]; // grass default
    let mut col = vec![C_FLOOR; total];

    // Path tiles through town center
    for x in 0..w {
        for y in 6..14 {
            let idx = (y * w + x) as usize;
            tiles[idx] = 1; // path/road tile
            col[idx] = C_FLOOR;
        }
    }

    // Building walls (impassable) — rough approximation
    // Elm's Lab building: columns 4-8, rows 0-4
    for x in 4..9 {
        for y in 0..5 {
            set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL);
        }
    }
    // Player's house: columns 12-15, rows 2-5
    for x in 12..16 {
        for y in 2..6 {
            set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL);
        }
    }
    // Elm's house: columns 10-13, rows 11-15
    for x in 10..14 {
        for y in 11..16 {
            set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL);
        }
    }
    // Neighbor's house: columns 1-5, rows 9-13
    for x in 1..6 {
        for y in 9..14 {
            set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL);
        }
    }

    // Map edge — western route opening at x=0 (walkable exit to Route 29)
    // Eastern route opening at x=17 (Route 27)
    // Warp door tiles (entry points)
    set_tile(&mut tiles, &mut col, w, 6,  3, 3, C_WARP);  // ElmsLab door
    set_tile(&mut tiles, &mut col, w, 13, 5, 3, C_WARP);  // PlayersHouse1F door
    set_tile(&mut tiles, &mut col, w, 3, 11, 3, C_WARP);  // NeighborsHouse door
    set_tile(&mut tiles, &mut col, w, 11,13, 3, C_WARP);  // ElmsHouse door

    MapData {
        id: MapId::NewBarkTown,
        name: "NEW BARK TOWN",
        width: w,
        height: h,
        tiles,
        collision: col,
        warps: vec![
            WarpDef { x: 6,  y: 3,  dest_map: MapId::ElmsLab,              dest_warp_id: 0 },
            WarpDef { x: 13, y: 5,  dest_map: MapId::PlayersHouse1F,       dest_warp_id: 0 },
            WarpDef { x: 3,  y: 11, dest_map: MapId::PlayersNeighborsHouse, dest_warp_id: 0 },
            WarpDef { x: 11, y: 13, dest_map: MapId::ElmsHouse,            dest_warp_id: 0 },
        ],
        npcs: vec![
            NpcDef { x: 6,  y: 8, sprite_id: 3, move_type: NpcMoveType::SpinRandom, script_id: 2, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "TEACHER" },
            NpcDef { x: 12, y: 9, sprite_id: 4, move_type: NpcMoveType::WalkUpDown,  script_id: 0, event_flag: None, event_flag_show: false, palette: 1, facing: Direction::Down, name: "FISHER" },
            NpcDef { x: 3,  y: 2, sprite_id: 5, move_type: NpcMoveType::Standing(Direction::Right), script_id: 4, event_flag: Some(9), event_flag_show: true, palette: 0, facing: Direction::Right, name: "RIVAL" },
        ],
        coord_events: vec![
            CoordEvent { x: 1, y: 8, scene_id: 0, script_id: 2 }, // SCENE_NEWBARKTOWN_TEACHER_STOPS_YOU
            CoordEvent { x: 1, y: 9, scene_id: 0, script_id: 3 },
        ],
        bg_events: vec![
            BgEvent { x: 8,  y: 8,  kind: BgEventKind::Read, script_id: 10 }, // NBT sign
            BgEvent { x: 11, y: 5,  kind: BgEventKind::Read, script_id: 11 }, // Player's house sign
            BgEvent { x: 3,  y: 3,  kind: BgEventKind::Read, script_id: 12 }, // Elm Lab sign
            BgEvent { x: 9,  y: 13, kind: BgEventKind::Read, script_id: 13 }, // Elm's house sign
        ],
        wild_encounters: vec![],
        connections: MapConnections {
            north: None,
            south: None,
            east: Some(MapConnection { direction: Direction::Right, dest_map: MapId::Route27, offset: 0 }),
            west: Some(MapConnection { direction: Direction::Left,  dest_map: MapId::Route29, offset: 0 }),
        },
        music_id: 2,
    }
}

// ── ElmsLab (10 x 12) ────────────────────────────────────────────────────────
// Source: pokecrystal-master/maps/ElmsLab.asm
// 5 blocks wide x 6 blocks tall = 10 tiles x 12 tiles

fn build_elms_lab() -> MapData {
    let (w, h) = (10i32, 12i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);

    // Door/warp tiles at bottom
    set_tile(&mut tiles, &mut col, w, 4, 11, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 5, 11, 3, C_WARP);

    // Lab equipment (counters, machines — impassable)
    // Healing machine area at (2,1)
    set_tile(&mut tiles, &mut col, w, 2, 1, 5, C_WALL);
    // Bookshelves at (6,1),(7,1),(8,1),(9,1)
    for x in 6..10 { set_tile(&mut tiles, &mut col, w, x, 1, 5, C_WALL); }
    // Travel books at (0,7),(1,7),(2,7),(3,7)
    for x in 0..4  { set_tile(&mut tiles, &mut col, w, x, 7, 5, C_WALL); }
    // Elm's desk counter at (2,3),(3,3),(4,3)
    for x in 2..5  { set_tile(&mut tiles, &mut col, w, x, 3, 4, C_COUNTER); }
    // Trashcan
    set_tile(&mut tiles, &mut col, w, 9, 3, 5, C_WALL);
    // PC at (3,5)
    set_tile(&mut tiles, &mut col, w, 3, 5, 5, C_WALL);
    // Window
    set_tile(&mut tiles, &mut col, w, 5, 0, 5, C_WALL);

    MapData {
        id: MapId::ElmsLab,
        name: "ELM'S LAB",
        width: w,
        height: h,
        tiles,
        collision: col,
        warps: vec![
            WarpDef { x: 4, y: 11, dest_map: MapId::NewBarkTown, dest_warp_id: 0 },
            WarpDef { x: 5, y: 11, dest_map: MapId::NewBarkTown, dest_warp_id: 0 },
        ],
        npcs: vec![
            // idx 0: Elm at (5,2) facing Down (repositioned to (3,4) during MEET_ELM scene by mod.rs)
            NpcDef { x: 5, y: 2, sprite_id: 2, move_type: NpcMoveType::Standing(Direction::Down), script_id: 0, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "ELM" },
            // idx 1: Aide — hidden initially (EVENT_ELMS_AIDE_IN_LAB = 12, show when set)
            NpcDef { x: 2, y: 9, sprite_id: 7, move_type: NpcMoveType::SpinRandom, script_id: 0, event_flag: Some(12), event_flag_show: true, palette: 0, facing: Direction::Down, name: "AIDE" },
            // idx 2: PokeBall Cyndaquil (6,3) — visible when EVENT_CYNDAQUIL_POKEBALL=13 NOT set
            NpcDef { x: 6, y: 3, sprite_id: 6, move_type: NpcMoveType::Still, script_id: 6, event_flag: Some(13), event_flag_show: false, palette: 0, facing: Direction::Down, name: "BALL_CYNDAQUIL" },
            // idx 3: PokeBall Totodile (7,3) — visible when EVENT_TOTODILE_POKEBALL=14 NOT set
            NpcDef { x: 7, y: 3, sprite_id: 6, move_type: NpcMoveType::Still, script_id: 7, event_flag: Some(14), event_flag_show: false, palette: 0, facing: Direction::Down, name: "BALL_TOTODILE" },
            // idx 4: PokeBall Chikorita (8,3) — visible when EVENT_CHIKORITA_POKEBALL=15 NOT set
            NpcDef { x: 8, y: 3, sprite_id: 6, move_type: NpcMoveType::Still, script_id: 8, event_flag: Some(15), event_flag_show: false, palette: 0, facing: Direction::Down, name: "BALL_CHIKORITA" },
            // idx 5: Officer — hidden initially (EVENT_COP_IN_ELMS_LAB=11, show when set)
            NpcDef { x: 5, y: 3, sprite_id: 8, move_type: NpcMoveType::Standing(Direction::Up), script_id: 0, event_flag: Some(11), event_flag_show: true, palette: 0, facing: Direction::Up, name: "OFFICER" },
        ],
        coord_events: vec![
            // Scene 1 = SCENE_ELMSLAB_CANT_LEAVE: block player from leaving
            CoordEvent { x: 4, y: 6, scene_id: 1, script_id: 9 },
            CoordEvent { x: 5, y: 6, scene_id: 1, script_id: 9 },
            // Scene 3 = SCENE_ELMSLAB_MEET_OFFICER (future sprint stubs)
            CoordEvent { x: 4, y: 5, scene_id: 3, script_id: 100 },
            CoordEvent { x: 5, y: 5, scene_id: 3, script_id: 100 },
            // Scene 5 = SCENE_ELMSLAB_AIDE_GIVES_POTION (future sprint stubs)
            CoordEvent { x: 4, y: 8, scene_id: 5, script_id: 101 },
            CoordEvent { x: 5, y: 8, scene_id: 5, script_id: 101 },
            // Scene 6 = SCENE_ELMSLAB_AIDE_GIVES_POKE_BALLS (future sprint stubs)
            CoordEvent { x: 4, y: 8, scene_id: 6, script_id: 102 },
            CoordEvent { x: 5, y: 8, scene_id: 6, script_id: 102 },
        ],
        bg_events: vec![
            BgEvent { x: 2, y: 1, kind: BgEventKind::Read,    script_id: 21 }, // Healing machine
            BgEvent { x: 6, y: 1, kind: BgEventKind::Read,    script_id: 22 }, // Bookshelf
            BgEvent { x: 7, y: 1, kind: BgEventKind::Read,    script_id: 22 },
            BgEvent { x: 8, y: 1, kind: BgEventKind::Read,    script_id: 22 },
            BgEvent { x: 9, y: 1, kind: BgEventKind::Read,    script_id: 22 },
            BgEvent { x: 0, y: 7, kind: BgEventKind::Read,    script_id: 22 }, // Travel books
            BgEvent { x: 1, y: 7, kind: BgEventKind::Read,    script_id: 22 },
            BgEvent { x: 2, y: 7, kind: BgEventKind::Read,    script_id: 22 },
            BgEvent { x: 3, y: 7, kind: BgEventKind::Read,    script_id: 22 },
            BgEvent { x: 9, y: 3, kind: BgEventKind::Read,    script_id: 23 }, // Trashcan
            BgEvent { x: 5, y: 0, kind: BgEventKind::Read,    script_id: 24 }, // Window
            BgEvent { x: 3, y: 5, kind: BgEventKind::FaceDown, script_id: 25 }, // PC
        ],
        wild_encounters: vec![],
        connections: MapConnections::none(),
        music_id: 3,
    }
}

// ── Stub Maps ────────────────────────────────────────────────────────────────

fn build_stub_house(return_map: MapId, return_warp_id: u8, name: &'static str, my_warp_id_in_nbt: u8) -> MapData {
    let (w, h) = (10i32, 8i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);
    set_tile(&mut tiles, &mut col, w, 6, 7, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 7, 7, 3, C_WARP);
    let _ = my_warp_id_in_nbt; // used to document intent
    MapData {
        id: if name.contains("NEIGHBOR") { MapId::PlayersNeighborsHouse } else { MapId::ElmsHouse },
        name,
        width: w,
        height: h,
        tiles,
        collision: col,
        warps: vec![
            WarpDef { x: 6, y: 7, dest_map: return_map, dest_warp_id: return_warp_id },
            WarpDef { x: 7, y: 7, dest_map: return_map, dest_warp_id: return_warp_id },
        ],
        npcs: vec![],
        coord_events: vec![],
        bg_events: vec![],
        wild_encounters: vec![],
        connections: MapConnections::none(),
        music_id: 1,
    }
}

fn build_stub_route(return_map: MapId, name: &'static str) -> MapData {
    let (w, h) = (20i32, 20i32);
    let tiles = vec![0u8; (w * h) as usize];
    let col   = vec![C_FLOOR; (w * h) as usize];
    MapData {
        id: if name.contains("29") { MapId::Route29 } else { MapId::Route27 },
        name,
        width: w,
        height: h,
        tiles,
        collision: col,
        warps: vec![
            WarpDef { x: 19, y: 10, dest_map: return_map, dest_warp_id: 0 },
        ],
        npcs: vec![],
        coord_events: vec![],
        bg_events: vec![],
        wild_encounters: vec![],
        connections: MapConnections::none(),
        music_id: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_sprint1_maps_load() {
        let ids = [
            MapId::PlayersHouse2F, MapId::PlayersHouse1F,
            MapId::NewBarkTown, MapId::ElmsLab,
            MapId::ElmsHouse, MapId::PlayersNeighborsHouse,
        ];
        for &id in &ids {
            let m = load_map(id);
            assert!(m.width > 0, "Map {:?} has zero width", id);
            assert!(m.height > 0, "Map {:?} has zero height", id);
            assert_eq!(m.tiles.len(), (m.width * m.height) as usize,
                "Map {:?} tiles length mismatch", id);
            assert_eq!(m.collision.len(), (m.width * m.height) as usize,
                "Map {:?} collision length mismatch", id);
        }
    }

    #[test]
    fn test_new_bark_town_dimensions() {
        let m = load_map(MapId::NewBarkTown);
        assert_eq!(m.width, 18);
        assert_eq!(m.height, 20);
    }

    #[test]
    fn test_elm_lab_has_6_npcs() {
        let m = load_map(MapId::ElmsLab);
        assert_eq!(m.npcs.len(), 6, "Elm's lab should have 6 NPCs");
    }

    #[test]
    fn test_elm_lab_pokeball_positions() {
        let m = load_map(MapId::ElmsLab);
        assert_eq!(m.npcs[2].x, 6); assert_eq!(m.npcs[2].y, 3); // Cyndaquil
        assert_eq!(m.npcs[3].x, 7); assert_eq!(m.npcs[3].y, 3); // Totodile
        assert_eq!(m.npcs[4].x, 8); assert_eq!(m.npcs[4].y, 3); // Chikorita
    }

    #[test]
    fn test_elm_lab_coord_events_count() {
        let m = load_map(MapId::ElmsLab);
        assert_eq!(m.coord_events.len(), 8);
    }

    #[test]
    fn test_warp_bidirectional() {
        let checked = [
            MapId::PlayersHouse2F, MapId::PlayersHouse1F,
            MapId::NewBarkTown, MapId::ElmsLab,
        ];
        for &map_id in &checked {
            let m = load_map(map_id);
            for warp in &m.warps {
                let dest = load_map(warp.dest_map);
                let has_return = dest.warps.iter().any(|w| w.dest_map == map_id);
                assert!(has_return, "Warp from {:?} to {:?} has no return", map_id, warp.dest_map);
            }
        }
    }
}
