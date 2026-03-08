// AI-INSTRUCTIONS: pokemonv2/maps.rs — Map system. Imports data.rs only.
// MapId enum, MapData struct, all warp/NPC/event types, load_map() returning data for all maps.
// Sprint 2: New maps (Route29 full, CherrygroveCity, Route29Route46Gate, buildings), WildEncounterTable,
//           C_GRASS/C_LEDGE collision, is_walkable_with_direction, MapConnection handling.
// Sprint 4: Route30 full (20x54), Route30BerryHouse, MrPokemonsHouse, Route31 stub.
//           NpcDef.trainer_range for sight-range trainer battles. Route30 encounter data.
// Import graph: maps.rs <- data.rs ONLY

use super::data::{Direction, NpcState, SpeciesId,
    PIDGEY, RATTATA, SENTRET, HOOTHOOT, HOPPIP,
    CATERPIE, WEEDLE, ZUBAT, POLIWAG, LEDYBA, SPINARAK};

// ── Collision Tile Constants ──────────────────────────────────────────────────
pub const C_FLOOR: u8 = 0;    // walkable
pub const C_WALL: u8 = 1;     // solid
pub const C_WATER: u8 = 2;    // impassable without Surf
pub const C_WARP: u8 = 3;     // triggers warp check (mats, doors)
pub const C_COUNTER: u8 = 4;  // interact across (Elm's desk)
pub const C_GRASS: u8 = 5;    // walkable, triggers wild encounter on step complete
pub const C_LEDGE_D: u8 = 6;  // one-way south: can jump south only
pub const C_LEDGE_L: u8 = 7;  // one-way left (future use)
pub const C_LEDGE_R: u8 = 8;  // one-way right (future use)

// ── Enums ────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub enum MapId {
    // Sprint 1 (existing)
    NewBarkTown,
    PlayersHouse1F,
    PlayersHouse2F,
    ElmsLab,
    ElmsHouse,
    PlayersNeighborsHouse,
    Route29,
    Route27,
    // Sprint 2 (new)
    Route29Route46Gate,
    CherrygroveCity,
    CherrygrovePokecenter1F,
    CherrygroveMart,
    GuideGentsHouse,
    CherrygroveGymSpeechHouse,
    CherrygroveEvolutionSpeechHouse,
    Route46,
    Route30,
    // Sprint 4 (new)
    Route30BerryHouse,
    MrPokemonsHouse,
    Route31,
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
    pub wild_encounters: Option<WildEncounterTable>,  // None if no wild encounters
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
    pub script_id: u16,
    pub event_flag: Option<u16>,
    pub event_flag_show: bool,
    pub palette: u8,
    pub facing: Direction,
    pub name: &'static str,
    pub trainer_range: Option<u8>,  // sight range for trainer battles
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

/// A single encounter slot: species at a fixed level.
#[derive(Clone, Debug)]
pub struct WildSlot {
    pub species: SpeciesId,
    pub level: u8,
}

/// Time-of-day encounter table for grass encounters.
/// Each period has exactly 7 slots matching pokecrystal's format.
/// Slot probabilities: [0]=30%, [1]=30%, [2]=20%, [3]=10%, [4]=5%, [5]=2.5%, [6]=2.5%
#[derive(Clone, Debug)]
pub struct WildEncounterTable {
    pub morning: Vec<WildSlot>,
    pub day: Vec<WildSlot>,
    pub night: Vec<WildSlot>,
    pub encounter_rate: u8,
}

// ── NPC State Initialization ──────────────────────────────────────────────────

/// Build runtime NPC states from map definition.
pub fn init_npc_states(map: &MapData) -> Vec<NpcState> {
    map.npcs.iter().map(|def| NpcState {
        x: def.x,
        y: def.y,
        facing: def.facing,
        walk_offset: 0.0,
        is_walking: false,
        visible: true,
        wander_timer: 0.0,
        emote: None,
    }).collect()
}

// ── Map Functions ─────────────────────────────────────────────────────────────

/// Check if tile at (x, y) is walkable (floor, warp, or grass tile).
pub fn is_walkable(map: &MapData, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 || x >= map.width || y >= map.height {
        return false;
    }
    let idx = (y * map.width + x) as usize;
    let c = map.collision[idx];
    c == C_FLOOR || c == C_WARP || c == C_GRASS
}

/// Check if tile at (x, y) is walkable when approached from the given direction.
/// Ledge tiles are only walkable if the player is facing the ledge's direction.
pub fn is_walkable_with_direction(map: &MapData, x: i32, y: i32, facing: Direction) -> bool {
    if x < 0 || y < 0 || x >= map.width || y >= map.height {
        return false;
    }
    let idx = (y * map.width + x) as usize;
    let c = map.collision[idx];
    match c {
        C_FLOOR | C_WARP | C_GRASS => true,
        C_LEDGE_D => facing == Direction::Down,
        C_LEDGE_L => facing == Direction::Left,
        C_LEDGE_R => facing == Direction::Right,
        _ => false,
    }
}

pub fn find_warp(map: &MapData, x: i32, y: i32) -> Option<&WarpDef> {
    map.warps.iter().find(|w| w.x == x && w.y == y)
}

pub fn find_coord_event(map: &MapData, x: i32, y: i32, scene_id: u8) -> Option<&CoordEvent> {
    map.coord_events.iter().find(|e| e.x == x && e.y == y && e.scene_id == scene_id)
}

pub fn find_bg_event(map: &MapData, x: i32, y: i32, facing: Direction) -> Option<&BgEvent> {
    map.bg_events.iter().find(|e| {
        e.x == x && e.y == y && match e.kind {
            BgEventKind::Read => true,
            BgEventKind::FaceUp => facing == Direction::Up,
            BgEventKind::FaceDown => facing == Direction::Down,
            BgEventKind::IfSet(_) => true,
        }
    })
}

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

pub fn load_map(id: MapId) -> MapData {
    match id {
        MapId::PlayersHouse2F       => build_players_house_2f(),
        MapId::PlayersHouse1F       => build_players_house_1f(),
        MapId::NewBarkTown          => build_new_bark_town(),
        MapId::ElmsLab              => build_elms_lab(),
        MapId::ElmsHouse            => build_stub_house(MapId::NewBarkTown, 3, "ELM'S HOUSE", 3),
        MapId::PlayersNeighborsHouse => build_stub_house(MapId::NewBarkTown, 2, "NEIGHBOR'S HOUSE", 2),
        MapId::Route29              => build_route29(),
        MapId::Route27              => build_stub_route(MapId::NewBarkTown, "ROUTE 27"),
        MapId::Route29Route46Gate   => build_route29_route46_gate(),
        MapId::CherrygroveCity      => build_cherrygrove_city(),
        MapId::CherrygrovePokecenter1F => build_cherrygrove_pokecenter_1f(),
        MapId::CherrygroveMart      => build_cherrygrove_mart(),
        MapId::GuideGentsHouse      => build_guide_gents_house(),
        MapId::CherrygroveGymSpeechHouse => build_cherrygrove_gym_speech_house(),
        MapId::CherrygroveEvolutionSpeechHouse => build_cherrygrove_evo_speech_house(),
        MapId::Route46              => build_route46_stub(),
        MapId::Route30              => build_route30(),
        MapId::Route30BerryHouse    => build_route30_berry_house(),
        MapId::MrPokemonsHouse      => build_mr_pokemons_house(),
        MapId::Route31              => build_route31_stub(),
    }
}

// ── Map Builder Helpers ───────────────────────────────────────────────────────

fn fill_room(w: i32, h: i32, floor_tile: u8) -> (Vec<u8>, Vec<u8>) {
    let total = (w * h) as usize;
    let mut tiles = vec![floor_tile; total];
    let mut col = vec![C_FLOOR; total];

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

fn build_players_house_2f() -> MapData {
    let (w, h) = (8i32, 6i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);

    set_tile(&mut tiles, &mut col, w, 7, 0, 6, C_WARP);
    set_tile(&mut tiles, &mut col, w, 7, 1, 4, C_FLOOR);
    set_tile(&mut tiles, &mut col, w, 4, 2, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 4, 4, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 5, 4, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 0, 1, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 2, 1, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 3, 1, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 5, 1, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 6, 0, 5, C_WALL);

    MapData {
        id: MapId::PlayersHouse2F,
        name: "PLAYER'S HOUSE 2F",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 7, y: 0, dest_map: MapId::PlayersHouse1F, dest_warp_id: 2 },
        ],
        npcs: vec![
            NpcDef { x: 4, y: 2, sprite_id: 10, move_type: NpcMoveType::Still, script_id: 0, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "CONSOLE", trainer_range: None },
            NpcDef { x: 4, y: 4, sprite_id: 11, move_type: NpcMoveType::Still, script_id: 0, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "DOLL_1", trainer_range: None },
            NpcDef { x: 5, y: 4, sprite_id: 12, move_type: NpcMoveType::Still, script_id: 0, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "DOLL_2", trainer_range: None },
            NpcDef { x: 0, y: 1, sprite_id: 13, move_type: NpcMoveType::Still, script_id: 0, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "BIG_DOLL", trainer_range: None },
        ],
        coord_events: vec![],
        bg_events: vec![
            BgEvent { x: 2, y: 1, kind: BgEventKind::FaceUp, script_id: 18 },
            BgEvent { x: 3, y: 1, kind: BgEventKind::Read,   script_id: 19 },
            BgEvent { x: 5, y: 1, kind: BgEventKind::Read,   script_id: 20 },
            BgEvent { x: 6, y: 0, kind: BgEventKind::Read,   script_id: 0  },
        ],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: 1,
    }
}

// ── PlayersHouse1F (10 x 8) ──────────────────────────────────────────────────

fn build_players_house_1f() -> MapData {
    let (w, h) = (10i32, 8i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);

    set_tile(&mut tiles, &mut col, w, 6, 7, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 7, 7, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 9, 0, 6, C_WARP);
    set_tile(&mut tiles, &mut col, w, 0, 1, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 1, 1, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 2, 1, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 4, 1, 5, C_WALL);

    MapData {
        id: MapId::PlayersHouse1F,
        name: "PLAYER'S HOUSE 1F",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 6, y: 7, dest_map: MapId::NewBarkTown,    dest_warp_id: 1 },
            WarpDef { x: 7, y: 7, dest_map: MapId::NewBarkTown,    dest_warp_id: 1 },
            WarpDef { x: 9, y: 0, dest_map: MapId::PlayersHouse2F, dest_warp_id: 0 },
        ],
        npcs: vec![
            NpcDef { x: 7, y: 4, sprite_id: 1, move_type: NpcMoveType::Standing(Direction::Left), script_id: 1, event_flag: Some(3), event_flag_show: false, palette: 0, facing: Direction::Left, name: "MOM1", trainer_range: None },
            NpcDef { x: 2, y: 2, sprite_id: 1, move_type: NpcMoveType::Standing(Direction::Up), script_id: 1, event_flag: Some(4), event_flag_show: true, palette: 0, facing: Direction::Up, name: "MOM2", trainer_range: None },
            NpcDef { x: 7, y: 4, sprite_id: 1, move_type: NpcMoveType::Standing(Direction::Left), script_id: 1, event_flag: Some(4), event_flag_show: true, palette: 0, facing: Direction::Left, name: "MOM3", trainer_range: None },
            NpcDef { x: 0, y: 2, sprite_id: 1, move_type: NpcMoveType::Standing(Direction::Up), script_id: 1, event_flag: Some(4), event_flag_show: true, palette: 0, facing: Direction::Up, name: "MOM4", trainer_range: None },
            NpcDef { x: 4, y: 4, sprite_id: 9, move_type: NpcMoveType::Standing(Direction::Right), script_id: 0, event_flag: Some(16), event_flag_show: true, palette: 0, facing: Direction::Right, name: "NEIGHBOR", trainer_range: None },
        ],
        coord_events: vec![
            CoordEvent { x: 8, y: 4, scene_id: 0, script_id: 1 },
            CoordEvent { x: 9, y: 4, scene_id: 0, script_id: 1 },
        ],
        bg_events: vec![
            BgEvent { x: 0, y: 1, kind: BgEventKind::Read, script_id: 14 },
            BgEvent { x: 1, y: 1, kind: BgEventKind::Read, script_id: 15 },
            BgEvent { x: 2, y: 1, kind: BgEventKind::Read, script_id: 16 },
            BgEvent { x: 4, y: 1, kind: BgEventKind::Read, script_id: 17 },
        ],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: 1,
    }
}

// ── NewBarkTown (18 x 20) ────────────────────────────────────────────────────

fn build_new_bark_town() -> MapData {
    let (w, h) = (18i32, 20i32);
    let total = (w * h) as usize;
    let mut tiles = vec![0u8; total];
    let mut col = vec![C_FLOOR; total];

    for x in 0..w {
        for y in 6..14 {
            let idx = (y * w + x) as usize;
            tiles[idx] = 1;
            col[idx] = C_FLOOR;
        }
    }
    for x in 4..9  { for y in 0..5  { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    for x in 12..16 { for y in 2..6 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    for x in 10..14 { for y in 11..16 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    for x in 1..6  { for y in 9..14 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }

    set_tile(&mut tiles, &mut col, w, 6,  3, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 13, 5, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 3, 11, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 11,13, 3, C_WARP);

    MapData {
        id: MapId::NewBarkTown,
        name: "NEW BARK TOWN",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 6,  y: 3,  dest_map: MapId::ElmsLab,              dest_warp_id: 0 },
            WarpDef { x: 13, y: 5,  dest_map: MapId::PlayersHouse1F,       dest_warp_id: 0 },
            WarpDef { x: 3,  y: 11, dest_map: MapId::PlayersNeighborsHouse, dest_warp_id: 0 },
            WarpDef { x: 11, y: 13, dest_map: MapId::ElmsHouse,            dest_warp_id: 0 },
        ],
        npcs: vec![
            NpcDef { x: 6,  y: 8, sprite_id: 3, move_type: NpcMoveType::SpinRandom, script_id: 2, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "TEACHER", trainer_range: None },
            NpcDef { x: 12, y: 9, sprite_id: 4, move_type: NpcMoveType::WalkUpDown,  script_id: 0, event_flag: None, event_flag_show: false, palette: 1, facing: Direction::Down, name: "FISHER", trainer_range: None },
            NpcDef { x: 3,  y: 2, sprite_id: 5, move_type: NpcMoveType::Standing(Direction::Right), script_id: 4, event_flag: Some(9), event_flag_show: true, palette: 0, facing: Direction::Right, name: "RIVAL", trainer_range: None },
        ],
        coord_events: vec![
            CoordEvent { x: 1, y: 8, scene_id: 0, script_id: 2 },
            CoordEvent { x: 1, y: 9, scene_id: 0, script_id: 3 },
        ],
        bg_events: vec![
            BgEvent { x: 8,  y: 8,  kind: BgEventKind::Read, script_id: 10 },
            BgEvent { x: 11, y: 5,  kind: BgEventKind::Read, script_id: 11 },
            BgEvent { x: 3,  y: 3,  kind: BgEventKind::Read, script_id: 12 },
            BgEvent { x: 9,  y: 13, kind: BgEventKind::Read, script_id: 13 },
        ],
        wild_encounters: None,
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

fn build_elms_lab() -> MapData {
    let (w, h) = (10i32, 12i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);

    set_tile(&mut tiles, &mut col, w, 4, 11, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 5, 11, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 2, 1, 5, C_WALL);
    for x in 6..10 { set_tile(&mut tiles, &mut col, w, x, 1, 5, C_WALL); }
    for x in 0..4  { set_tile(&mut tiles, &mut col, w, x, 7, 5, C_WALL); }
    for x in 2..5  { set_tile(&mut tiles, &mut col, w, x, 3, 4, C_COUNTER); }
    set_tile(&mut tiles, &mut col, w, 9, 3, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 3, 5, 5, C_WALL);
    set_tile(&mut tiles, &mut col, w, 5, 0, 5, C_WALL);

    MapData {
        id: MapId::ElmsLab,
        name: "ELM'S LAB",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 4, y: 11, dest_map: MapId::NewBarkTown, dest_warp_id: 0 },
            WarpDef { x: 5, y: 11, dest_map: MapId::NewBarkTown, dest_warp_id: 0 },
        ],
        npcs: vec![
            NpcDef { x: 5, y: 2, sprite_id: 2, move_type: NpcMoveType::Standing(Direction::Down), script_id: 0, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "ELM", trainer_range: None },
            NpcDef { x: 2, y: 9, sprite_id: 7, move_type: NpcMoveType::SpinRandom, script_id: 0, event_flag: Some(12), event_flag_show: true, palette: 0, facing: Direction::Down, name: "AIDE", trainer_range: None },
            NpcDef { x: 6, y: 3, sprite_id: 6, move_type: NpcMoveType::Still, script_id: 6, event_flag: Some(13), event_flag_show: false, palette: 0, facing: Direction::Down, name: "BALL_CYNDAQUIL", trainer_range: None },
            NpcDef { x: 7, y: 3, sprite_id: 6, move_type: NpcMoveType::Still, script_id: 7, event_flag: Some(14), event_flag_show: false, palette: 0, facing: Direction::Down, name: "BALL_TOTODILE", trainer_range: None },
            NpcDef { x: 8, y: 3, sprite_id: 6, move_type: NpcMoveType::Still, script_id: 8, event_flag: Some(15), event_flag_show: false, palette: 0, facing: Direction::Down, name: "BALL_CHIKORITA", trainer_range: None },
            NpcDef { x: 5, y: 3, sprite_id: 8, move_type: NpcMoveType::Standing(Direction::Up), script_id: 0, event_flag: Some(11), event_flag_show: true, palette: 0, facing: Direction::Up, name: "OFFICER", trainer_range: None },
        ],
        coord_events: vec![
            CoordEvent { x: 4, y: 6, scene_id: 1, script_id: 9 },
            CoordEvent { x: 5, y: 6, scene_id: 1, script_id: 9 },
            CoordEvent { x: 4, y: 5, scene_id: 3, script_id: 100 },
            CoordEvent { x: 5, y: 5, scene_id: 3, script_id: 100 },
            CoordEvent { x: 4, y: 8, scene_id: 5, script_id: 101 },
            CoordEvent { x: 5, y: 8, scene_id: 5, script_id: 101 },
            CoordEvent { x: 4, y: 8, scene_id: 6, script_id: 102 },
            CoordEvent { x: 5, y: 8, scene_id: 6, script_id: 102 },
        ],
        bg_events: vec![
            BgEvent { x: 2, y: 1, kind: BgEventKind::Read,    script_id: 21 },
            BgEvent { x: 6, y: 1, kind: BgEventKind::Read,    script_id: 22 },
            BgEvent { x: 7, y: 1, kind: BgEventKind::Read,    script_id: 22 },
            BgEvent { x: 8, y: 1, kind: BgEventKind::Read,    script_id: 22 },
            BgEvent { x: 9, y: 1, kind: BgEventKind::Read,    script_id: 22 },
            BgEvent { x: 0, y: 7, kind: BgEventKind::Read,    script_id: 22 },
            BgEvent { x: 1, y: 7, kind: BgEventKind::Read,    script_id: 22 },
            BgEvent { x: 2, y: 7, kind: BgEventKind::Read,    script_id: 22 },
            BgEvent { x: 3, y: 7, kind: BgEventKind::Read,    script_id: 22 },
            BgEvent { x: 9, y: 3, kind: BgEventKind::Read,    script_id: 23 },
            BgEvent { x: 5, y: 0, kind: BgEventKind::Read,    script_id: 24 },
            BgEvent { x: 3, y: 5, kind: BgEventKind::FaceDown, script_id: 25 },
        ],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: 3,
    }
}

// ── Stub Maps ────────────────────────────────────────────────────────────────

fn build_stub_house(return_map: MapId, return_warp_id: u8, name: &'static str, _my_warp_id_in_nbt: u8) -> MapData {
    let (w, h) = (10i32, 8i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);
    set_tile(&mut tiles, &mut col, w, 6, 7, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 7, 7, 3, C_WARP);
    MapData {
        id: if name.contains("NEIGHBOR") { MapId::PlayersNeighborsHouse } else { MapId::ElmsHouse },
        name, width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 6, y: 7, dest_map: return_map, dest_warp_id: return_warp_id },
            WarpDef { x: 7, y: 7, dest_map: return_map, dest_warp_id: return_warp_id },
        ],
        npcs: vec![],
        coord_events: vec![],
        bg_events: vec![],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: 1,
    }
}

fn build_stub_route(return_map: MapId, name: &'static str) -> MapData {
    let (w, h) = (20i32, 20i32);
    let tiles = vec![0u8; (w * h) as usize];
    let col   = vec![C_FLOOR; (w * h) as usize];
    MapData {
        id: MapId::Route27,
        name, width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 19, y: 10, dest_map: return_map, dest_warp_id: 0 },
        ],
        npcs: vec![],
        coord_events: vec![],
        bg_events: vec![],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: 0,
    }
}

// ── Route 29 Wild Encounter Data ──────────────────────────────────────────────

fn build_route29_encounters() -> WildEncounterTable {
    WildEncounterTable {
        encounter_rate: 10,
        morning: vec![
            WildSlot { species: PIDGEY,   level: 2 },
            WildSlot { species: SENTRET,  level: 2 },
            WildSlot { species: PIDGEY,   level: 3 },
            WildSlot { species: SENTRET,  level: 3 },
            WildSlot { species: RATTATA,  level: 2 },
            WildSlot { species: HOPPIP,   level: 3 },
            WildSlot { species: HOPPIP,   level: 3 },
        ],
        day: vec![
            WildSlot { species: PIDGEY,   level: 2 },
            WildSlot { species: SENTRET,  level: 2 },
            WildSlot { species: PIDGEY,   level: 3 },
            WildSlot { species: SENTRET,  level: 3 },
            WildSlot { species: RATTATA,  level: 2 },
            WildSlot { species: HOPPIP,   level: 3 },
            WildSlot { species: HOPPIP,   level: 3 },
        ],
        night: vec![
            WildSlot { species: HOOTHOOT, level: 2 },
            WildSlot { species: RATTATA,  level: 2 },
            WildSlot { species: HOOTHOOT, level: 3 },
            WildSlot { species: RATTATA,  level: 3 },
            WildSlot { species: RATTATA,  level: 2 },
            WildSlot { species: HOOTHOOT, level: 3 },
            WildSlot { species: HOOTHOOT, level: 3 },
        ],
    }
}

// ── Route 29 (60 x 18) ───────────────────────────────────────────────────────
// Source: pokecrystal-master/maps/Route29.asm

fn build_route29() -> MapData {
    let (w, h) = (60i32, 18i32);
    let total = (w * h) as usize;
    let mut tiles = vec![0u8; total];
    let mut col = vec![C_FLOOR; total];

    // Perimeter walls (top/bottom borders, cliff faces)
    for x in 0..w {
        set_tile(&mut tiles, &mut col, w, x, 0, 2, C_WALL);
        set_tile(&mut tiles, &mut col, w, x, 17, 2, C_WALL);
    }

    // Grass patches (central areas of the route)
    for x in 5..20 {
        for y in 8..13 {
            set_tile(&mut tiles, &mut col, w, x, y, 7, C_GRASS);
        }
    }
    for x in 30..50 {
        for y in 3..8 {
            set_tile(&mut tiles, &mut col, w, x, y, 7, C_GRASS);
        }
    }
    for x in 35..55 {
        for y in 10..16 {
            set_tile(&mut tiles, &mut col, w, x, y, 7, C_GRASS);
        }
    }

    // Ledge strips (south-facing, one-way shortcuts)
    for x in 5..25 {
        set_tile(&mut tiles, &mut col, w, x, 6, 8, C_LEDGE_D);
    }
    for x in 30..55 {
        set_tile(&mut tiles, &mut col, w, x, 9, 8, C_LEDGE_D);
    }

    // Gate entrance warp at (27, 1)
    set_tile(&mut tiles, &mut col, w, 27, 1, 3, C_WARP);

    // Tree/wall obstacles (rough approximation)
    for x in 0..5 {
        for y in 1..17 {
            set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL);
        }
    }

    MapData {
        id: MapId::Route29,
        name: "ROUTE 29",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 27, y: 1, dest_map: MapId::Route29Route46Gate, dest_warp_id: 2 },
        ],
        npcs: vec![
            // idx 0: Dude (catching tutorial) at (50,12)
            NpcDef { x: 50, y: 12, sprite_id: 14, move_type: NpcMoveType::SpinRandom, script_id: 202, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "DUDE", trainer_range: None },
            // idx 1: Youngster at (27,16)
            NpcDef { x: 27, y: 16, sprite_id: 15, move_type: NpcMoveType::WalkUpDown, script_id: 205, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "YOUNGSTER", trainer_range: None },
            // idx 2: Teacher at (15,11)
            NpcDef { x: 15, y: 11, sprite_id: 3, move_type: NpcMoveType::WalkLeftRight, script_id: 206, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "TEACHER", trainer_range: None },
            // idx 3: Fruit Tree at (12,2)
            NpcDef { x: 12, y: 2, sprite_id: 16, move_type: NpcMoveType::Still, script_id: 209, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "FRUIT_TREE", trainer_range: None },
            // idx 4: Fisher at (25,3)
            NpcDef { x: 25, y: 3, sprite_id: 4, move_type: NpcMoveType::Standing(Direction::Up), script_id: 207, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Up, name: "FISHER", trainer_range: None },
            // idx 5: CooltrainerM at (13,4)
            NpcDef { x: 13, y: 4, sprite_id: 14, move_type: NpcMoveType::Standing(Direction::Down), script_id: 208, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "COOLTRAINER_M", trainer_range: None },
            // idx 6: Tuscany at (29,12) -- hidden unless EVENT_ROUTE_29_TUSCANY_OF_TUESDAY is set
            NpcDef { x: 29, y: 12, sprite_id: 3, move_type: NpcMoveType::SpinRandom, script_id: 211, event_flag: Some(25), event_flag_show: true, palette: 0, facing: Direction::Down, name: "TUSCANY", trainer_range: None },
            // idx 7: Potion PokeBall at (48,2) -- visible when EVENT_ROUTE_29_POTION NOT set
            NpcDef { x: 48, y: 2, sprite_id: 6, move_type: NpcMoveType::Still, script_id: 210, event_flag: Some(26), event_flag_show: false, palette: 0, facing: Direction::Down, name: "POTION_BALL", trainer_range: None },
        ],
        coord_events: vec![
            CoordEvent { x: 53, y: 8, scene_id: 1, script_id: 203 },
            CoordEvent { x: 53, y: 9, scene_id: 1, script_id: 204 },
        ],
        bg_events: vec![
            BgEvent { x: 51, y: 7, kind: BgEventKind::Read, script_id: 200 },
            BgEvent { x: 3,  y: 5, kind: BgEventKind::Read, script_id: 201 },
        ],
        wild_encounters: Some(build_route29_encounters()),
        connections: MapConnections {
            north: Some(MapConnection { direction: Direction::Up,   dest_map: MapId::Route46, offset: 10 }),
            south: None,
            east:  Some(MapConnection { direction: Direction::Right, dest_map: MapId::NewBarkTown, offset: 0 }),
            west:  Some(MapConnection { direction: Direction::Left,  dest_map: MapId::CherrygroveCity, offset: 0 }),
        },
        music_id: 4,
    }
}

// ── Route29Route46Gate (10 x 8) ──────────────────────────────────────────────

fn build_route29_route46_gate() -> MapData {
    let (w, h) = (10i32, 8i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);

    set_tile(&mut tiles, &mut col, w, 4, 0, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 5, 0, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 4, 7, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 5, 7, 3, C_WARP);

    MapData {
        id: MapId::Route29Route46Gate,
        name: "ROUTE 29 GATE",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 4, y: 0, dest_map: MapId::Route46, dest_warp_id: 1 },
            WarpDef { x: 5, y: 0, dest_map: MapId::Route46, dest_warp_id: 2 },
            WarpDef { x: 4, y: 7, dest_map: MapId::Route29, dest_warp_id: 0 },
            WarpDef { x: 5, y: 7, dest_map: MapId::Route29, dest_warp_id: 0 },
        ],
        npcs: vec![
            NpcDef { x: 0, y: 4, sprite_id: 8, move_type: NpcMoveType::Standing(Direction::Right), script_id: 220, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Right, name: "OFFICER", trainer_range: None },
            NpcDef { x: 6, y: 4, sprite_id: 15, move_type: NpcMoveType::WalkUpDown, script_id: 221, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "YOUNGSTER", trainer_range: None },
        ],
        coord_events: vec![],
        bg_events: vec![],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: 5,
    }
}

// ── CherrygroveCity (40 x 18) ────────────────────────────────────────────────

fn build_cherrygrove_city() -> MapData {
    let (w, h) = (40i32, 18i32);
    let total = (w * h) as usize;
    let mut tiles = vec![1u8; total]; // path default
    let mut col = vec![C_FLOOR; total];

    // Perimeter walls
    for x in 0..w {
        set_tile(&mut tiles, &mut col, w, x, 0, 2, C_WALL);
        set_tile(&mut tiles, &mut col, w, x, 17, 2, C_WALL);
    }

    // Building blocks (approximate)
    // Mart area: columns 22-26, rows 0-4
    for x in 22..27 { for y in 0..4 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    // Pokecenter area: columns 28-32, rows 0-4
    for x in 28..33 { for y in 0..4 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    // Gym speech house: columns 16-20, rows 6-9
    for x in 16..21 { for y in 6..9 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    // Guide Gent's house: columns 24-28, rows 8-12
    for x in 24..29 { for y in 8..12 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    // Evo speech house: columns 30-34, rows 10-14
    for x in 30..35 { for y in 10..14 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }

    // Warp tiles
    set_tile(&mut tiles, &mut col, w, 23, 3, 3, C_WARP); // Mart
    set_tile(&mut tiles, &mut col, w, 29, 3, 3, C_WARP); // Pokecenter
    set_tile(&mut tiles, &mut col, w, 17, 7, 3, C_WARP); // Gym Speech House
    set_tile(&mut tiles, &mut col, w, 25, 9, 3, C_WARP); // Guide Gent's House
    set_tile(&mut tiles, &mut col, w, 31, 11, 3, C_WARP); // Evo Speech House

    MapData {
        id: MapId::CherrygroveCity,
        name: "CHERRYGROVE CITY",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 23, y: 3,  dest_map: MapId::CherrygroveMart,                dest_warp_id: 2 },
            WarpDef { x: 29, y: 3,  dest_map: MapId::CherrygrovePokecenter1F,         dest_warp_id: 1 },
            WarpDef { x: 17, y: 7,  dest_map: MapId::CherrygroveGymSpeechHouse,       dest_warp_id: 1 },
            WarpDef { x: 25, y: 9,  dest_map: MapId::GuideGentsHouse,                 dest_warp_id: 1 },
            WarpDef { x: 31, y: 11, dest_map: MapId::CherrygroveEvolutionSpeechHouse, dest_warp_id: 1 },
        ],
        npcs: vec![
            // idx 0: Guide Gent -- hidden when EVENT_GUIDE_GENT_IN_HIS_HOUSE (18) is set
            NpcDef { x: 32, y: 6, sprite_id: 17, move_type: NpcMoveType::Standing(Direction::Down), script_id: 230, event_flag: Some(18), event_flag_show: false, palette: 0, facing: Direction::Down, name: "GUIDE_GENT", trainer_range: None },
            // idx 1: Rival -- visible when EVENT_RIVAL_CHERRYGROVE_CITY (19) is set
            NpcDef { x: 39, y: 6, sprite_id: 5, move_type: NpcMoveType::SpinRandom, script_id: 231, event_flag: Some(19), event_flag_show: true, palette: 0, facing: Direction::Down, name: "RIVAL", trainer_range: None },
            // idx 2: Teacher
            NpcDef { x: 27, y: 12, sprite_id: 3, move_type: NpcMoveType::WalkLeftRight, script_id: 232, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "TEACHER", trainer_range: None },
            // idx 3: Youngster
            NpcDef { x: 23, y: 7, sprite_id: 15, move_type: NpcMoveType::WalkLeftRight, script_id: 233, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "YOUNGSTER", trainer_range: None },
            // idx 4: Fisher (Mystic Water Guy)
            NpcDef { x: 7, y: 12, sprite_id: 4, move_type: NpcMoveType::Standing(Direction::Right), script_id: 234, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Right, name: "MYSTIC_WATER_GUY", trainer_range: None },
        ],
        coord_events: vec![
            CoordEvent { x: 33, y: 6, scene_id: 1, script_id: 231 },
            CoordEvent { x: 33, y: 7, scene_id: 1, script_id: 231 },
        ],
        bg_events: vec![
            BgEvent { x: 30, y: 8, kind: BgEventKind::Read, script_id: 240 },
            BgEvent { x: 23, y: 9, kind: BgEventKind::Read, script_id: 241 },
            BgEvent { x: 24, y: 3, kind: BgEventKind::Read, script_id: 242 },
            BgEvent { x: 30, y: 3, kind: BgEventKind::Read, script_id: 243 },
        ],
        wild_encounters: None,
        connections: MapConnections {
            north: Some(MapConnection { direction: Direction::Up,   dest_map: MapId::Route30, offset: 5 }),
            south: None,
            east:  Some(MapConnection { direction: Direction::Right, dest_map: MapId::Route29, offset: 0 }),
            west:  None,
        },
        music_id: 6,
    }
}

// ── CherrygrovePokecenter1F (10 x 8) ─────────────────────────────────────────

fn build_cherrygrove_pokecenter_1f() -> MapData {
    let (w, h) = (10i32, 8i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);

    set_tile(&mut tiles, &mut col, w, 3, 7, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 4, 7, 3, C_WARP);

    MapData {
        id: MapId::CherrygrovePokecenter1F,
        name: "CHERRYGROVE POKEMON CENTER",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 3, y: 7, dest_map: MapId::CherrygroveCity, dest_warp_id: 2 },
            WarpDef { x: 4, y: 7, dest_map: MapId::CherrygroveCity, dest_warp_id: 2 },
        ],
        npcs: vec![
            NpcDef { x: 3, y: 1, sprite_id: 18, move_type: NpcMoveType::Standing(Direction::Down), script_id: 250, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "NURSE", trainer_range: None },
            NpcDef { x: 2, y: 3, sprite_id: 4,  move_type: NpcMoveType::Standing(Direction::Up),   script_id: 251, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Up,   name: "FISHER", trainer_range: None },
            NpcDef { x: 8, y: 6, sprite_id: 19, move_type: NpcMoveType::Standing(Direction::Up),   script_id: 252, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Up,   name: "GENTLEMAN", trainer_range: None },
            NpcDef { x: 1, y: 6, sprite_id: 3,  move_type: NpcMoveType::Standing(Direction::Right), script_id: 253, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Right, name: "TEACHER", trainer_range: None },
        ],
        coord_events: vec![],
        bg_events: vec![],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: 7,
    }
}

// ── CherrygroveMart (12 x 8) ─────────────────────────────────────────────────

fn build_cherrygrove_mart() -> MapData {
    let (w, h) = (12i32, 8i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);

    set_tile(&mut tiles, &mut col, w, 2, 7, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 3, 7, 3, C_WARP);

    MapData {
        id: MapId::CherrygroveMart,
        name: "CHERRYGROVE MART",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 2, y: 7, dest_map: MapId::CherrygroveCity, dest_warp_id: 1 },
            WarpDef { x: 3, y: 7, dest_map: MapId::CherrygroveCity, dest_warp_id: 1 },
        ],
        npcs: vec![
            NpcDef { x: 1, y: 3, sprite_id: 20, move_type: NpcMoveType::Standing(Direction::Right), script_id: 260, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Right, name: "CLERK", trainer_range: None },
            NpcDef { x: 7, y: 6, sprite_id: 14, move_type: NpcMoveType::WalkLeftRight, script_id: 261, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "COOLTRAINER", trainer_range: None },
            NpcDef { x: 2, y: 5, sprite_id: 15, move_type: NpcMoveType::Standing(Direction::Down), script_id: 262, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "YOUNGSTER", trainer_range: None },
        ],
        coord_events: vec![],
        bg_events: vec![],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: 8,
    }
}

// ── GuideGentsHouse (8 x 8) ──────────────────────────────────────────────────

fn build_guide_gents_house() -> MapData {
    let (w, h) = (8i32, 8i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);

    set_tile(&mut tiles, &mut col, w, 2, 7, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 3, 7, 3, C_WARP);

    MapData {
        id: MapId::GuideGentsHouse,
        name: "GUIDE GENT'S HOUSE",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 2, y: 7, dest_map: MapId::CherrygroveCity, dest_warp_id: 4 },
            WarpDef { x: 3, y: 7, dest_map: MapId::CherrygroveCity, dest_warp_id: 4 },
        ],
        npcs: vec![
            // Gramps: visible when EVENT_GUIDE_GENT_VISIBLE_IN_CHERRYGROVE (17) IS set
            NpcDef { x: 2, y: 3, sprite_id: 17, move_type: NpcMoveType::Standing(Direction::Right), script_id: 270, event_flag: Some(17), event_flag_show: true, palette: 0, facing: Direction::Right, name: "GUIDE_GENT_HOME", trainer_range: None },
        ],
        coord_events: vec![],
        bg_events: vec![
            BgEvent { x: 0, y: 1, kind: BgEventKind::Read, script_id: 271 },
            BgEvent { x: 1, y: 1, kind: BgEventKind::Read, script_id: 271 },
        ],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: 1,
    }
}

// ── CherrygroveGymSpeechHouse (8 x 8) ────────────────────────────────────────

fn build_cherrygrove_gym_speech_house() -> MapData {
    let (w, h) = (8i32, 8i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);

    set_tile(&mut tiles, &mut col, w, 2, 7, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 3, 7, 3, C_WARP);

    MapData {
        id: MapId::CherrygroveGymSpeechHouse,
        name: "CHERRYGROVE GYM SPEECH HOUSE",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 2, y: 7, dest_map: MapId::CherrygroveCity, dest_warp_id: 3 },
            WarpDef { x: 3, y: 7, dest_map: MapId::CherrygroveCity, dest_warp_id: 3 },
        ],
        npcs: vec![
            NpcDef { x: 2, y: 3, sprite_id: 21, move_type: NpcMoveType::Standing(Direction::Down), script_id: 280, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "POKEFAN_M", trainer_range: None },
            NpcDef { x: 5, y: 5, sprite_id: 22, move_type: NpcMoveType::WalkLeftRight, script_id: 281, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "BUG_CATCHER", trainer_range: None },
        ],
        coord_events: vec![],
        bg_events: vec![
            BgEvent { x: 0, y: 1, kind: BgEventKind::Read, script_id: 282 },
            BgEvent { x: 1, y: 1, kind: BgEventKind::Read, script_id: 282 },
        ],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: 1,
    }
}

// ── CherrygroveEvolutionSpeechHouse (8 x 8) ──────────────────────────────────

fn build_cherrygrove_evo_speech_house() -> MapData {
    let (w, h) = (8i32, 8i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);

    set_tile(&mut tiles, &mut col, w, 2, 7, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 3, 7, 3, C_WARP);

    MapData {
        id: MapId::CherrygroveEvolutionSpeechHouse,
        name: "EVOLUTION SPEECH HOUSE",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 2, y: 7, dest_map: MapId::CherrygroveCity, dest_warp_id: 5 },
            WarpDef { x: 3, y: 7, dest_map: MapId::CherrygroveCity, dest_warp_id: 5 },
        ],
        npcs: vec![
            NpcDef { x: 3, y: 5, sprite_id: 23, move_type: NpcMoveType::Standing(Direction::Left), script_id: 290, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Left, name: "LASS", trainer_range: None },
            NpcDef { x: 2, y: 5, sprite_id: 15, move_type: NpcMoveType::Standing(Direction::Right), script_id: 291, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Right, name: "YOUNGSTER", trainer_range: None },
        ],
        coord_events: vec![],
        bg_events: vec![
            BgEvent { x: 0, y: 1, kind: BgEventKind::Read, script_id: 292 },
            BgEvent { x: 1, y: 1, kind: BgEventKind::Read, script_id: 292 },
        ],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: 1,
    }
}

// ── Route46 Stub (10 x 8) ────────────────────────────────────────────────────

fn build_route46_stub() -> MapData {
    let (w, h) = (10i32, 8i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);

    set_tile(&mut tiles, &mut col, w, 4, 7, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 4, 0, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 5, 0, 3, C_WARP);

    MapData {
        id: MapId::Route46,
        name: "ROUTE 46",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 4, y: 7, dest_map: MapId::Route29Route46Gate, dest_warp_id: 0 },
            WarpDef { x: 4, y: 0, dest_map: MapId::Route29Route46Gate, dest_warp_id: 0 },
            WarpDef { x: 5, y: 0, dest_map: MapId::Route29Route46Gate, dest_warp_id: 1 },
        ],
        npcs: vec![],
        coord_events: vec![],
        bg_events: vec![],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: 0,
    }
}

// ── Route 30 Full (20 x 54) ──────────────────────────────────────────────────
// Sprint 4: Full outdoor route with grass, water, 11 NPCs, 2 warps to houses,
// connections north to Route31 and south to CherrygroveCity.
// Trainers: Youngster Joey (Rattata/4), Youngster Mikey (Pidgey/2 + Rattata/4),
//           Bug Catcher Don (Caterpie/3 + Caterpie/3)

fn build_route30() -> MapData {
    let (w, h) = (20i32, 54i32);
    let mut tiles = vec![1u8; (w * h) as usize];
    let mut col   = vec![C_WALL; (w * h) as usize];

    // Helper to set tile + collision
    let set = |tiles: &mut Vec<u8>, col: &mut Vec<u8>, x: i32, y: i32, t: u8, c: u8| {
        if x >= 0 && x < w && y >= 0 && y < h {
            let idx = (y * w + x) as usize;
            tiles[idx] = t;
            col[idx] = c;
        }
    };

    // Fill walkable corridor: columns 3-16, full height
    for y in 0..h {
        for x in 3..17 {
            set(&mut tiles, &mut col, x, y, 0, C_FLOOR);
        }
    }

    // Grass patches: Route 30 has grass areas per pokecrystal
    // Southern grass patches (y=42..50, x=5..12)
    for y in 42..50 {
        for x in 5..12 {
            set(&mut tiles, &mut col, x, y, 2, C_GRASS);
        }
    }
    // Middle grass patches (y=25..32, x=8..15)
    for y in 25..32 {
        for x in 8..15 {
            set(&mut tiles, &mut col, x, y, 2, C_GRASS);
        }
    }
    // Northern grass patches (y=8..15, x=4..11)
    for y in 8..15 {
        for x in 4..11 {
            set(&mut tiles, &mut col, x, y, 2, C_GRASS);
        }
    }

    // Water on east side (y=18..24, x=14..16)
    for y in 18..24 {
        for x in 14..17 {
            set(&mut tiles, &mut col, x, y, 3, C_WATER);
        }
    }

    // Warp mats for house doors
    set(&mut tiles, &mut col, 7, 39, 4, C_WARP);  // Berry House door
    set(&mut tiles, &mut col, 17, 5, 4, C_WARP);   // Mr. Pokemon's House door

    MapData {
        id: MapId::Route30,
        name: "ROUTE 30",
        width: w,
        height: h,
        tiles,
        collision: col,
        warps: vec![
            WarpDef { x: 7, y: 39, dest_map: MapId::Route30BerryHouse, dest_warp_id: 0 },
            WarpDef { x: 17, y: 5, dest_map: MapId::MrPokemonsHouse, dest_warp_id: 0 },
        ],
        npcs: vec![
            // 0: Youngster Joey — trainer, script 301, beaten flag 32, range 3
            NpcDef { x: 10, y: 35, sprite_id: 15, move_type: NpcMoveType::Standing(Direction::Left), script_id: 301, event_flag: Some(32), event_flag_show: false, palette: 0, facing: Direction::Left, name: "JOEY", trainer_range: Some(3) },
            // 1: Youngster Mikey — trainer, script 302, beaten flag 33, range 1
            NpcDef { x: 8, y: 22, sprite_id: 15, move_type: NpcMoveType::Standing(Direction::Down), script_id: 302, event_flag: Some(33), event_flag_show: false, palette: 0, facing: Direction::Down, name: "MIKEY", trainer_range: Some(1) },
            // 2: Bug Catcher Don — trainer, script 303, beaten flag 34, range 3
            NpcDef { x: 6, y: 16, sprite_id: 22, move_type: NpcMoveType::Standing(Direction::Right), script_id: 303, event_flag: Some(34), event_flag_show: false, palette: 0, facing: Direction::Right, name: "DON", trainer_range: Some(3) },
            // 3: Youngster giving directions
            NpcDef { x: 12, y: 44, sprite_id: 15, move_type: NpcMoveType::WalkUpDown, script_id: 304, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "YOUNGSTER_DIRECTIONS", trainer_range: None },
            // 4: CooltrainerF
            NpcDef { x: 14, y: 33, sprite_id: 24, move_type: NpcMoveType::Standing(Direction::Down), script_id: 305, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "COOLTRAINER_F", trainer_range: None },
            // 5: Fruit Tree 1
            NpcDef { x: 5, y: 37, sprite_id: 16, move_type: NpcMoveType::Still, script_id: 311, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "FRUIT_TREE_1", trainer_range: None },
            // 6: Fruit Tree 2
            NpcDef { x: 15, y: 20, sprite_id: 16, move_type: NpcMoveType::Still, script_id: 312, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "FRUIT_TREE_2", trainer_range: None },
            // 7: Antidote item ball
            NpcDef { x: 4, y: 18, sprite_id: 6, move_type: NpcMoveType::Still, script_id: 310, event_flag: Some(37), event_flag_show: false, palette: 0, facing: Direction::Down, name: "ANTIDOTE_BALL", trainer_range: None },
            // 8: Battle guy
            NpcDef { x: 13, y: 38, sprite_id: 14, move_type: NpcMoveType::Standing(Direction::Left), script_id: 309, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Left, name: "BATTLE_GUY", trainer_range: None },
            // 9: Hidden Potion
            NpcDef { x: 16, y: 10, sprite_id: 6, move_type: NpcMoveType::Still, script_id: 313, event_flag: Some(38), event_flag_show: false, palette: 0, facing: Direction::Down, name: "HIDDEN_POTION", trainer_range: None },
            // 10: Joey pre-battle cutscene NPC (phone number request after battle)
            NpcDef { x: 10, y: 36, sprite_id: 15, move_type: NpcMoveType::Standing(Direction::Up), script_id: 300, event_flag: Some(42), event_flag_show: true, palette: 0, facing: Direction::Up, name: "JOEY_PHONE", trainer_range: None },
        ],
        coord_events: vec![],
        bg_events: vec![
            BgEvent { x: 10, y: 53, kind: BgEventKind::Read, script_id: 306 },  // Route 30 sign (south end)
            BgEvent { x: 17, y: 6,  kind: BgEventKind::Read, script_id: 308 },  // Mr. Pokemon's House sign
            BgEvent { x: 7, y: 40,  kind: BgEventKind::Read, script_id: 307 },  // Berry House directions sign
            BgEvent { x: 12, y: 34, kind: BgEventKind::Read, script_id: 309 },  // Trainer Tips sign
            BgEvent { x: 10, y: 0,  kind: BgEventKind::Read, script_id: 306 },  // Route 30 sign (north end)
        ],
        wild_encounters: Some(build_route30_encounters()),
        connections: MapConnections {
            north: Some(MapConnection { direction: Direction::Up, dest_map: MapId::Route31, offset: -10 }),
            south: Some(MapConnection { direction: Direction::Down, dest_map: MapId::CherrygroveCity, offset: -5 }),
            east: None,
            west: None,
        },
        music_id: 0,
    }
}

fn build_route30_encounters() -> WildEncounterTable {
    WildEncounterTable {
        encounter_rate: 10,
        morning: vec![
            WildSlot { species: LEDYBA,   level: 3 },
            WildSlot { species: CATERPIE, level: 3 },
            WildSlot { species: CATERPIE, level: 4 },
            WildSlot { species: PIDGEY,   level: 4 },
            WildSlot { species: WEEDLE,   level: 3 },
            WildSlot { species: HOPPIP,   level: 4 },
            WildSlot { species: HOPPIP,   level: 4 },
        ],
        day: vec![
            WildSlot { species: PIDGEY,   level: 3 },
            WildSlot { species: CATERPIE, level: 3 },
            WildSlot { species: CATERPIE, level: 4 },
            WildSlot { species: PIDGEY,   level: 4 },
            WildSlot { species: WEEDLE,   level: 3 },
            WildSlot { species: HOPPIP,   level: 4 },
            WildSlot { species: HOPPIP,   level: 4 },
        ],
        night: vec![
            WildSlot { species: SPINARAK, level: 3 },
            WildSlot { species: HOOTHOOT, level: 3 },
            WildSlot { species: POLIWAG,  level: 4 },
            WildSlot { species: HOOTHOOT, level: 4 },
            WildSlot { species: ZUBAT,    level: 3 },
            WildSlot { species: HOOTHOOT, level: 4 },
            WildSlot { species: HOOTHOOT, level: 4 },
        ],
    }
}

// ── Route 30 Berry House (8 x 8) ────────────────────────────────────────────

fn build_route30_berry_house() -> MapData {
    let (w, h) = (8i32, 8i32);
    let (tiles, col) = fill_room(w, h, 0);
    MapData {
        id: MapId::Route30BerryHouse,
        name: "ROUTE 30 BERRY HOUSE",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 2, y: 7, dest_map: MapId::Route30, dest_warp_id: 0 },
            WarpDef { x: 3, y: 7, dest_map: MapId::Route30, dest_warp_id: 0 },
        ],
        npcs: vec![
            NpcDef { x: 2, y: 3, sprite_id: 21, move_type: NpcMoveType::Standing(Direction::Down), script_id: 320, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Down, name: "POKEFAN_BERRY", trainer_range: None },
        ],
        coord_events: vec![],
        bg_events: vec![
            BgEvent { x: 0, y: 1, kind: BgEventKind::Read, script_id: 321 },
            BgEvent { x: 1, y: 1, kind: BgEventKind::Read, script_id: 321 },
        ],
        wild_encounters: None,
        connections: MapConnections { north: None, south: None, east: None, west: None },
        music_id: 0,
    }
}

// ── Mr. Pokemon's House (8 x 8) ─────────────────────────────────────────────

fn build_mr_pokemons_house() -> MapData {
    let (w, h) = (8i32, 8i32);
    let (tiles, col) = fill_room(w, h, 0);
    MapData {
        id: MapId::MrPokemonsHouse,
        name: "MR. POKEMON'S HOUSE",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: 2, y: 7, dest_map: MapId::Route30, dest_warp_id: 1 },
            WarpDef { x: 3, y: 7, dest_map: MapId::Route30, dest_warp_id: 1 },
        ],
        npcs: vec![
            // 0: MR_POKEMON (Gentleman sprite) — always present
            NpcDef { x: 3, y: 5, sprite_id: 19, move_type: NpcMoveType::Standing(Direction::Right), script_id: 330, event_flag: None, event_flag_show: false, palette: 0, facing: Direction::Right, name: "MR_POKEMON", trainer_range: None },
            // 1: OAK — disappears when EVENT_MR_POKEMONS_HOUSE_OAK (41) is set
            NpcDef { x: 6, y: 5, sprite_id: 25, move_type: NpcMoveType::Standing(Direction::Up), script_id: 0, event_flag: Some(41), event_flag_show: false, palette: 0, facing: Direction::Up, name: "OAK", trainer_range: None },
        ],
        coord_events: vec![],
        bg_events: vec![
            BgEvent { x: 0, y: 1, kind: BgEventKind::Read, script_id: 332 },   // magazines
            BgEvent { x: 1, y: 1, kind: BgEventKind::Read, script_id: 332 },   // magazines
            BgEvent { x: 6, y: 1, kind: BgEventKind::Read, script_id: 333 },   // computer
            BgEvent { x: 7, y: 1, kind: BgEventKind::Read, script_id: 333 },   // computer
            BgEvent { x: 4, y: 3, kind: BgEventKind::Read, script_id: 334 },   // coins on table
        ],
        wild_encounters: None,
        connections: MapConnections { north: None, south: None, east: None, west: None },
        music_id: 0,
    }
}

// ── Route 31 Stub (20 x 20) ─────────────────────────────────────────────────

fn build_route31_stub() -> MapData {
    let (w, h) = (20i32, 20i32);
    let tiles = vec![0u8; (w * h) as usize];
    let col = vec![C_FLOOR; (w * h) as usize];

    MapData {
        id: MapId::Route31,
        name: "ROUTE 31",
        width: w, height: h, tiles, collision: col,
        warps: vec![],
        npcs: vec![],
        coord_events: vec![],
        bg_events: vec![],
        wild_encounters: None,
        connections: MapConnections {
            north: None,
            south: Some(MapConnection { direction: Direction::Down, dest_map: MapId::Route30, offset: 10 }),
            east: None,
            west: None,
        },
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
            assert!(m.width > 0);
            assert!(m.height > 0);
            assert_eq!(m.tiles.len(), (m.width * m.height) as usize);
            assert_eq!(m.collision.len(), (m.width * m.height) as usize);
            assert!(m.wild_encounters.is_none(), "Sprint 1 map {:?} should have None wild_encounters", id);
        }
    }

    #[test]
    fn test_sprint2_maps_load() {
        let ids = [
            MapId::Route29, MapId::Route29Route46Gate, MapId::CherrygroveCity,
            MapId::CherrygrovePokecenter1F, MapId::CherrygroveMart, MapId::GuideGentsHouse,
            MapId::CherrygroveGymSpeechHouse, MapId::CherrygroveEvolutionSpeechHouse,
            MapId::Route46, MapId::Route30,
        ];
        for &id in &ids {
            let m = load_map(id);
            assert!(m.width > 0, "Map {:?} has zero width", id);
            assert!(m.height > 0, "Map {:?} has zero height", id);
            assert_eq!(m.tiles.len(), (m.width * m.height) as usize, "tiles mismatch {:?}", id);
            assert_eq!(m.collision.len(), (m.width * m.height) as usize, "collision mismatch {:?}", id);
        }
    }

    #[test]
    fn test_route29_has_encounters() {
        let m = load_map(MapId::Route29);
        assert!(m.wild_encounters.is_some(), "Route29 should have wild encounters");
        let table = m.wild_encounters.unwrap();
        assert_eq!(table.morning.len(), 7);
        assert_eq!(table.day.len(), 7);
        assert_eq!(table.night.len(), 7);
        assert_eq!(table.encounter_rate, 10);
    }

    #[test]
    fn test_is_walkable_with_direction_ledge() {
        let m = load_map(MapId::Route29);
        // Find a C_LEDGE_D tile to test directional walkability
        let has_ledge = m.collision.iter().any(|&c| c == C_LEDGE_D);
        assert!(has_ledge, "Route29 should have ledge tiles");
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
        assert_eq!(m.npcs.len(), 6);
    }

    #[test]
    fn test_elm_lab_pokeball_positions() {
        let m = load_map(MapId::ElmsLab);
        assert_eq!(m.npcs[2].x, 6); assert_eq!(m.npcs[2].y, 3);
        assert_eq!(m.npcs[3].x, 7); assert_eq!(m.npcs[3].y, 3);
        assert_eq!(m.npcs[4].x, 8); assert_eq!(m.npcs[4].y, 3);
    }

    #[test]
    fn test_warp_bidirectional_sprint1() {
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

    #[test]
    fn test_cherrygrove_has_5_warps() {
        let m = load_map(MapId::CherrygroveCity);
        assert_eq!(m.warps.len(), 5);
    }

    #[test]
    fn test_is_walkable_includes_grass() {
        let m = load_map(MapId::Route29);
        // Find a grass tile
        let grass_pos = m.collision.iter().enumerate().find(|(_, &c)| c == C_GRASS);
        if let Some((idx, _)) = grass_pos {
            let x = (idx as i32) % m.width;
            let y = (idx as i32) / m.width;
            assert!(is_walkable(&m, x, y), "Grass tile should be walkable");
        }
    }

    // ── Sprint 3 QA: Group 1 — Player Spawn & Bedroom ──────────────────

    #[test]
    fn test_bedroom_map_structure() {
        let m = load_map(MapId::PlayersHouse2F);
        assert_eq!(m.width, 8);
        assert_eq!(m.height, 6);
        assert_eq!(m.warps.len(), 1, "Bedroom has 1 staircase warp");
        assert_eq!(m.warps[0].x, 7);
        assert_eq!(m.warps[0].y, 0);
        assert_eq!(m.warps[0].dest_map, MapId::PlayersHouse1F);
        assert_eq!(m.npcs.len(), 4, "4 decoration NPCs");
        assert_eq!(m.bg_events.len(), 4);
        assert!(m.wild_encounters.is_none());
        assert!(m.coord_events.is_empty());
    }

    // ── Sprint 3 QA: Group 2 — Stair Warp & Mom ────────────────────────

    #[test]
    fn test_house1f_map_structure() {
        let m = load_map(MapId::PlayersHouse1F);
        assert_eq!(m.width, 10);
        assert_eq!(m.height, 8);
        assert_eq!(m.warps.len(), 3, "2 exit doors + 1 staircase");
        // Exit doors
        assert_eq!(m.warps[0].dest_map, MapId::NewBarkTown);
        assert_eq!(m.warps[1].dest_map, MapId::NewBarkTown);
        // Staircase
        assert_eq!(m.warps[2].dest_map, MapId::PlayersHouse2F);
        assert_eq!(m.npcs.len(), 5, "MOM1, MOM2, MOM3, MOM4, NEIGHBOR");
        // Mom variants with event flags
        assert_eq!(m.npcs[0].event_flag, Some(3)); // MOM1
        assert_eq!(m.npcs[1].event_flag, Some(4)); // MOM2
        assert_eq!(m.npcs[4].event_flag, Some(16)); // NEIGHBOR
        // Coord events for meet-mom cutscene
        assert_eq!(m.coord_events.len(), 2);
        assert_eq!(m.coord_events[0].x, 8);
        assert_eq!(m.coord_events[0].y, 4);
        assert_eq!(m.coord_events[0].scene_id, 0);
    }

    // ── Sprint 3 QA: Group 3 — New Bark Town ───────────────────────────

    #[test]
    fn test_new_bark_town_full_structure() {
        let m = load_map(MapId::NewBarkTown);
        assert_eq!(m.width, 18);
        assert_eq!(m.height, 20);
        assert_eq!(m.warps.len(), 4, "ElmsLab, PlayersHouse1F, NeighborsHouse, ElmsHouse");
        assert_eq!(m.warps[0].dest_map, MapId::ElmsLab);
        assert_eq!(m.warps[1].dest_map, MapId::PlayersHouse1F);
        assert_eq!(m.warps[2].dest_map, MapId::PlayersNeighborsHouse);
        assert_eq!(m.warps[3].dest_map, MapId::ElmsHouse);
        assert_eq!(m.npcs.len(), 3, "TEACHER, FISHER, RIVAL");
        assert_eq!(m.npcs[0].name, "TEACHER");
        assert_eq!(m.npcs[0].x, 6);
        assert_eq!(m.npcs[0].y, 8);
        assert_eq!(m.npcs[2].name, "RIVAL");
        assert_eq!(m.npcs[2].event_flag, Some(9));
        assert_eq!(m.coord_events.len(), 2);
        assert_eq!(m.bg_events.len(), 4);
        // Map connections
        assert!(m.connections.east.is_some());
        assert_eq!(m.connections.east.as_ref().unwrap().dest_map, MapId::Route27);
        assert!(m.connections.west.is_some());
        assert_eq!(m.connections.west.as_ref().unwrap().dest_map, MapId::Route29);
        assert!(m.connections.north.is_none());
        assert!(m.connections.south.is_none());
        assert!(m.wild_encounters.is_none());
    }

    #[test]
    fn test_teacher_block_coord_events() {
        let m = load_map(MapId::NewBarkTown);
        let teacher_events: Vec<_> = m.coord_events.iter()
            .filter(|e| e.scene_id == 0)
            .collect();
        assert_eq!(teacher_events.len(), 2);
        assert!(teacher_events.iter().any(|e| e.x == 1 && e.y == 8));
        assert!(teacher_events.iter().any(|e| e.x == 1 && e.y == 9));
    }

    // ── Sprint 3 QA: Group 4 — Elm's Lab ───────────────────────────────

    #[test]
    fn test_elms_lab_full_structure() {
        let m = load_map(MapId::ElmsLab);
        assert_eq!(m.width, 10);
        assert_eq!(m.height, 12);
        assert_eq!(m.warps.len(), 2);
        assert_eq!(m.warps[0].dest_map, MapId::NewBarkTown);
        assert_eq!(m.npcs.len(), 6, "ELM, AIDE, 3 pokeballs, OFFICER");
        // Pokeball event flags: hidden when set
        assert_eq!(m.npcs[2].event_flag, Some(13));
        assert!(!m.npcs[2].event_flag_show);
        assert_eq!(m.npcs[3].event_flag, Some(14));
        assert!(!m.npcs[3].event_flag_show);
        assert_eq!(m.npcs[4].event_flag, Some(15));
        assert!(!m.npcs[4].event_flag_show);
        assert_eq!(m.coord_events.len(), 8);
        assert_eq!(m.bg_events.len(), 12);
        assert!(m.wild_encounters.is_none());
    }

    #[test]
    fn test_elms_lab_scene_coord_events() {
        let m = load_map(MapId::ElmsLab);
        // Cant-leave events at scene_id=1, y=6
        let cant_leave: Vec<_> = m.coord_events.iter()
            .filter(|e| e.scene_id == 1)
            .collect();
        assert_eq!(cant_leave.len(), 2);
        assert!(cant_leave.iter().all(|e| e.y == 6));
        // Meet-officer events at scene_id=3
        let meet_officer: Vec<_> = m.coord_events.iter()
            .filter(|e| e.scene_id == 3)
            .collect();
        assert_eq!(meet_officer.len(), 2);
    }

    // ── Sprint 3 QA: Group 5 — Route 29 ────────────────────────────────

    #[test]
    fn test_route29_full_structure() {
        let m = load_map(MapId::Route29);
        assert_eq!(m.width, 60);
        assert_eq!(m.height, 18);
        assert_eq!(m.warps.len(), 1, "1 warp to Route29Route46Gate");
        assert_eq!(m.warps[0].x, 27);
        assert_eq!(m.warps[0].y, 1);
        assert_eq!(m.warps[0].dest_map, MapId::Route29Route46Gate);
        assert_eq!(m.npcs.len(), 8, "DUDE, YOUNGSTER, TEACHER, FRUIT_TREE, FISHER, COOLTRAINER_M, TUSCANY, POTION_BALL");
        assert_eq!(m.coord_events.len(), 2);
        assert_eq!(m.bg_events.len(), 2);
        // Connections
        assert!(m.connections.north.is_some());
        assert_eq!(m.connections.north.as_ref().unwrap().dest_map, MapId::Route46);
        assert!(m.connections.east.is_some());
        assert_eq!(m.connections.east.as_ref().unwrap().dest_map, MapId::NewBarkTown);
        assert!(m.connections.west.is_some());
        assert_eq!(m.connections.west.as_ref().unwrap().dest_map, MapId::CherrygroveCity);
    }

    #[test]
    fn test_route29_wild_encounters_by_time_of_day() {
        let m = load_map(MapId::Route29);
        let table = m.wild_encounters.as_ref().unwrap();
        assert_eq!(table.encounter_rate, 10);
        // Morning
        assert_eq!(table.morning[0].species, PIDGEY);
        assert_eq!(table.morning[0].level, 2);
        assert_eq!(table.morning[1].species, SENTRET);
        assert_eq!(table.morning[1].level, 2);
        // Day
        assert_eq!(table.day[0].species, PIDGEY);
        assert_eq!(table.day[0].level, 2);
        assert_eq!(table.day[1].species, SENTRET);
        assert_eq!(table.day[1].level, 2);
        // Night
        assert_eq!(table.night[0].species, HOOTHOOT);
        assert_eq!(table.night[0].level, 2);
        assert_eq!(table.night[1].species, RATTATA);
        assert_eq!(table.night[1].level, 2);
    }

    #[test]
    fn test_route29_grass_tiles_present() {
        let m = load_map(MapId::Route29);
        let grass_count = m.collision.iter().filter(|&&c| c == C_GRASS).count();
        assert!(grass_count > 10, "Route29 should have many grass tiles, found {}", grass_count);
    }

    #[test]
    fn test_route29_ledge_tiles_one_way() {
        let m = load_map(MapId::Route29);
        let ledge_pos = m.collision.iter().enumerate().find(|(_, &c)| c == C_LEDGE_D);
        assert!(ledge_pos.is_some(), "Route29 should have south-facing ledge tiles");
        let (idx, _) = ledge_pos.unwrap();
        let x = (idx as i32) % m.width;
        let y = (idx as i32) / m.width;
        assert!(is_walkable_with_direction(&m, x, y, Direction::Down));
        assert!(!is_walkable_with_direction(&m, x, y, Direction::Up));
        assert!(!is_walkable_with_direction(&m, x, y, Direction::Left));
        assert!(!is_walkable_with_direction(&m, x, y, Direction::Right));
    }

    #[test]
    fn test_route29_potion_item_ball() {
        let m = load_map(MapId::Route29);
        let potion_npc = m.npcs.iter().find(|n| n.name == "POTION_BALL");
        assert!(potion_npc.is_some());
        let npc = potion_npc.unwrap();
        assert_eq!(npc.x, 48);
        assert_eq!(npc.y, 2);
        assert_eq!(npc.event_flag, Some(26));
        assert!(!npc.event_flag_show, "Hidden when flag IS set (item already picked up)");
    }

    #[test]
    fn test_route29_tuscany_event_flag() {
        let m = load_map(MapId::Route29);
        let tuscany = m.npcs.iter().find(|n| n.name == "TUSCANY");
        assert!(tuscany.is_some());
        let npc = tuscany.unwrap();
        assert_eq!(npc.event_flag, Some(25));
        assert!(npc.event_flag_show, "Visible only when flag is set");
    }

    #[test]
    fn test_catching_tutorial_coord_events() {
        let m = load_map(MapId::Route29);
        let tutorial_events: Vec<_> = m.coord_events.iter()
            .filter(|e| e.scene_id == 1)
            .collect();
        assert_eq!(tutorial_events.len(), 2);
        assert!(tutorial_events.iter().any(|e| e.x == 53 && e.y == 8));
        assert!(tutorial_events.iter().any(|e| e.x == 53 && e.y == 9));
    }

    // ── Sprint 3 QA: Group 6 — Route 29/46 Gate ────────────────────────

    #[test]
    fn test_route29_route46_gate_structure() {
        let m = load_map(MapId::Route29Route46Gate);
        assert_eq!(m.width, 10);
        assert_eq!(m.height, 8);
        assert_eq!(m.warps.len(), 4, "2 north to Route46, 2 south to Route29");
        assert_eq!(m.npcs.len(), 2, "OFFICER, YOUNGSTER");
        assert!(m.wild_encounters.is_none());
        assert!(m.coord_events.is_empty());
    }

    #[test]
    fn test_gate_warp_connectivity() {
        let gate = load_map(MapId::Route29Route46Gate);
        // South warps -> Route29
        assert_eq!(gate.warps[2].dest_map, MapId::Route29);
        assert_eq!(gate.warps[3].dest_map, MapId::Route29);
        // North warps -> Route46
        assert_eq!(gate.warps[0].dest_map, MapId::Route46);
        assert_eq!(gate.warps[1].dest_map, MapId::Route46);
        // Route29 warp -> gate
        let r29 = load_map(MapId::Route29);
        assert_eq!(r29.warps[0].dest_map, MapId::Route29Route46Gate);
    }

    // ── Sprint 3 QA: Group 7 — Cherrygrove City ────────────────────────

    #[test]
    fn test_cherrygrove_city_full_structure() {
        let m = load_map(MapId::CherrygroveCity);
        assert_eq!(m.width, 40);
        assert_eq!(m.height, 18);
        assert_eq!(m.warps.len(), 5);
        assert_eq!(m.npcs.len(), 5, "GUIDE_GENT, RIVAL, TEACHER, YOUNGSTER, MYSTIC_WATER_GUY");
        // Guide Gent: hidden when EVENT_GUIDE_GENT_IN_HIS_HOUSE (18) set
        assert_eq!(m.npcs[0].name, "GUIDE_GENT");
        assert_eq!(m.npcs[0].event_flag, Some(18));
        assert!(!m.npcs[0].event_flag_show);
        // Rival: visible when EVENT_RIVAL_CHERRYGROVE_CITY (19) set
        assert_eq!(m.npcs[1].name, "RIVAL");
        assert_eq!(m.npcs[1].event_flag, Some(19));
        assert!(m.npcs[1].event_flag_show);
        assert_eq!(m.coord_events.len(), 2);
        assert_eq!(m.bg_events.len(), 4);
        // Connections
        assert!(m.connections.north.is_some());
        assert_eq!(m.connections.north.as_ref().unwrap().dest_map, MapId::Route30);
        assert!(m.connections.east.is_some());
        assert_eq!(m.connections.east.as_ref().unwrap().dest_map, MapId::Route29);
        assert!(m.connections.west.is_none());
        assert!(m.wild_encounters.is_none());
    }

    #[test]
    fn test_rival_ambush_coord_events() {
        let m = load_map(MapId::CherrygroveCity);
        let rival_events: Vec<_> = m.coord_events.iter()
            .filter(|e| e.scene_id == 1)
            .collect();
        assert_eq!(rival_events.len(), 2);
        assert!(rival_events.iter().any(|e| e.x == 33 && e.y == 6));
        assert!(rival_events.iter().any(|e| e.x == 33 && e.y == 7));
    }

    // ── Sprint 3 QA: Group 8 — Cherrygrove Buildings ────────────────────

    #[test]
    fn test_cherrygrove_pokecenter_structure() {
        let m = load_map(MapId::CherrygrovePokecenter1F);
        assert_eq!(m.width, 10);
        assert_eq!(m.height, 8);
        assert_eq!(m.warps.len(), 2);
        assert_eq!(m.warps[0].dest_map, MapId::CherrygroveCity);
        assert_eq!(m.npcs.len(), 4, "NURSE, FISHER, GENTLEMAN, TEACHER");
        assert_eq!(m.npcs[0].name, "NURSE");
    }

    #[test]
    fn test_cherrygrove_mart_structure() {
        let m = load_map(MapId::CherrygroveMart);
        assert_eq!(m.width, 12);
        assert_eq!(m.height, 8);
        assert_eq!(m.warps.len(), 2);
        assert_eq!(m.warps[0].dest_map, MapId::CherrygroveCity);
        assert_eq!(m.npcs.len(), 3, "CLERK, COOLTRAINER, YOUNGSTER");
        assert_eq!(m.npcs[0].name, "CLERK");
    }

    #[test]
    fn test_guide_gents_house_structure() {
        let m = load_map(MapId::GuideGentsHouse);
        assert_eq!(m.width, 8);
        assert_eq!(m.height, 8);
        assert_eq!(m.warps.len(), 2);
        assert_eq!(m.warps[0].dest_map, MapId::CherrygroveCity);
        assert_eq!(m.npcs.len(), 1);
        assert_eq!(m.npcs[0].event_flag, Some(17));
        assert!(m.npcs[0].event_flag_show);
        assert_eq!(m.bg_events.len(), 2);
    }

    #[test]
    fn test_gym_speech_house_structure() {
        let m = load_map(MapId::CherrygroveGymSpeechHouse);
        assert_eq!(m.width, 8);
        assert_eq!(m.height, 8);
        assert_eq!(m.warps.len(), 2);
        assert_eq!(m.npcs.len(), 2);
        assert_eq!(m.bg_events.len(), 2);
    }

    #[test]
    fn test_evo_speech_house_structure() {
        let m = load_map(MapId::CherrygroveEvolutionSpeechHouse);
        assert_eq!(m.width, 8);
        assert_eq!(m.height, 8);
        assert_eq!(m.warps.len(), 2);
        assert_eq!(m.npcs.len(), 2);
        assert_eq!(m.bg_events.len(), 2);
    }

    #[test]
    fn test_all_cherrygrove_building_warps_bidirectional() {
        let buildings = [
            MapId::CherrygrovePokecenter1F,
            MapId::CherrygroveMart,
            MapId::GuideGentsHouse,
            MapId::CherrygroveGymSpeechHouse,
            MapId::CherrygroveEvolutionSpeechHouse,
        ];
        let city = load_map(MapId::CherrygroveCity);
        for &bldg_id in &buildings {
            let bldg = load_map(bldg_id);
            // Building has warp back to CherrygroveCity
            let has_exit = bldg.warps.iter().any(|w| w.dest_map == MapId::CherrygroveCity);
            assert!(has_exit, "{:?} should have warp back to CherrygroveCity", bldg_id);
            // CherrygroveCity has warp into building
            let has_entry = city.warps.iter().any(|w| w.dest_map == bldg_id);
            assert!(has_entry, "CherrygroveCity should have warp into {:?}", bldg_id);
        }
    }

    #[test]
    fn test_route46_stub() {
        let m = load_map(MapId::Route46);
        assert_eq!(m.width, 10);
        assert_eq!(m.height, 8);
        assert!(m.warps.len() >= 1);
        assert!(m.wild_encounters.is_none());
    }

    #[test]
    fn test_route30_stub_has_south_connection() {
        let m = load_map(MapId::Route30);
        assert!(m.connections.south.is_some());
        assert_eq!(m.connections.south.as_ref().unwrap().dest_map, MapId::CherrygroveCity);
    }

    // ── Sprint 4: Route 30 + Mr. Pokemon's House Tests ──────────────────

    #[test]
    fn test_route30_dimensions() {
        let m = load_map(MapId::Route30);
        assert_eq!(m.width, 20);
        assert_eq!(m.height, 54);
    }

    #[test]
    fn test_route30_has_wild_encounters() {
        let m = load_map(MapId::Route30);
        assert!(m.wild_encounters.is_some());
        let table = m.wild_encounters.as_ref().unwrap();
        assert_eq!(table.morning.len(), 7);
        assert_eq!(table.day.len(), 7);
        assert_eq!(table.night.len(), 7);
        assert_eq!(table.encounter_rate, 10);
    }

    #[test]
    fn test_route30_connections() {
        let m = load_map(MapId::Route30);
        assert!(m.connections.north.is_some());
        assert_eq!(m.connections.north.as_ref().unwrap().dest_map, MapId::Route31);
        assert!(m.connections.south.is_some());
        assert_eq!(m.connections.south.as_ref().unwrap().dest_map, MapId::CherrygroveCity);
    }

    #[test]
    fn test_route30_warps() {
        let m = load_map(MapId::Route30);
        assert_eq!(m.warps.len(), 2);
        assert_eq!(m.warps[0].dest_map, MapId::Route30BerryHouse);
        assert_eq!(m.warps[1].dest_map, MapId::MrPokemonsHouse);
    }

    #[test]
    fn test_route30_npcs() {
        let m = load_map(MapId::Route30);
        assert_eq!(m.npcs.len(), 11, "Route 30 should have 11 NPCs");
        // Trainers have trainer_range
        let trainers: Vec<_> = m.npcs.iter().filter(|n| n.trainer_range.is_some()).collect();
        assert_eq!(trainers.len(), 3, "Route 30 should have 3 trainers: Joey, Mikey, Don");
        assert_eq!(trainers[0].name, "JOEY");
        assert_eq!(trainers[0].trainer_range, Some(3));
        assert_eq!(trainers[1].name, "MIKEY");
        assert_eq!(trainers[1].trainer_range, Some(1));
        assert_eq!(trainers[2].name, "DON");
        assert_eq!(trainers[2].trainer_range, Some(3));
    }

    #[test]
    fn test_route30_berry_house() {
        let m = load_map(MapId::Route30BerryHouse);
        assert_eq!(m.width, 8);
        assert_eq!(m.height, 8);
        assert_eq!(m.npcs.len(), 1);
        assert_eq!(m.warps.len(), 2);
        assert_eq!(m.warps[0].dest_map, MapId::Route30);
        assert_eq!(m.bg_events.len(), 2);
        assert!(m.wild_encounters.is_none());
    }

    #[test]
    fn test_mr_pokemons_house() {
        let m = load_map(MapId::MrPokemonsHouse);
        assert_eq!(m.width, 8);
        assert_eq!(m.height, 8);
        assert_eq!(m.npcs.len(), 2, "MR_POKEMON + OAK");
        assert_eq!(m.npcs[0].name, "MR_POKEMON");
        assert_eq!(m.npcs[1].name, "OAK");
        assert_eq!(m.npcs[1].event_flag, Some(41)); // EVENT_MR_POKEMONS_HOUSE_OAK
        assert!(!m.npcs[1].event_flag_show, "Oak disappears when flag is set");
        assert_eq!(m.warps.len(), 2);
        assert_eq!(m.warps[0].dest_map, MapId::Route30);
        assert_eq!(m.bg_events.len(), 5);
        assert!(m.wild_encounters.is_none());
    }

    #[test]
    fn test_route30_bidirectional_warps() {
        // Route30 -> BerryHouse warps back to Route30
        let berry = load_map(MapId::Route30BerryHouse);
        let has_exit = berry.warps.iter().any(|w| w.dest_map == MapId::Route30);
        assert!(has_exit, "Berry House should warp back to Route30");

        // Route30 -> MrPokemonsHouse warps back to Route30
        let mr = load_map(MapId::MrPokemonsHouse);
        let has_exit = mr.warps.iter().any(|w| w.dest_map == MapId::Route30);
        assert!(has_exit, "Mr. Pokemon's House should warp back to Route30");

        // Route30 has warps into both houses
        let route30 = load_map(MapId::Route30);
        let has_berry = route30.warps.iter().any(|w| w.dest_map == MapId::Route30BerryHouse);
        let has_mr = route30.warps.iter().any(|w| w.dest_map == MapId::MrPokemonsHouse);
        assert!(has_berry, "Route30 should have warp to Berry House");
        assert!(has_mr, "Route30 should have warp to Mr. Pokemon's House");
    }

    #[test]
    fn test_route31_stub() {
        let m = load_map(MapId::Route31);
        assert_eq!(m.width, 20);
        assert_eq!(m.height, 20);
        assert!(m.connections.south.is_some());
        assert_eq!(m.connections.south.as_ref().unwrap().dest_map, MapId::Route30);
        assert!(m.wild_encounters.is_none());
    }

    #[test]
    fn test_sprint4_all_maps_load() {
        let ids = [
            MapId::Route30, MapId::Route30BerryHouse,
            MapId::MrPokemonsHouse, MapId::Route31,
        ];
        for &id in &ids {
            let m = load_map(id);
            assert!(m.width > 0, "Map {:?} has zero width", id);
            assert!(m.height > 0, "Map {:?} has zero height", id);
            assert_eq!(m.tiles.len(), (m.width * m.height) as usize, "tiles mismatch {:?}", id);
            assert_eq!(m.collision.len(), (m.width * m.height) as usize, "collision mismatch {:?}", id);
        }
    }

    #[test]
    fn test_route30_grass_tiles_present() {
        let m = load_map(MapId::Route30);
        let grass_count = m.collision.iter().filter(|&&c| c == C_GRASS).count();
        assert!(grass_count > 20, "Route 30 should have many grass tiles, found {}", grass_count);
    }

    #[test]
    fn test_route30_trainer_event_flags() {
        let m = load_map(MapId::Route30);
        // Joey has beaten flag 32
        assert_eq!(m.npcs[0].event_flag, Some(32));
        assert!(!m.npcs[0].event_flag_show, "Trainer NPCs should hide when beaten flag set");
        // Mikey has beaten flag 33
        assert_eq!(m.npcs[1].event_flag, Some(33));
        // Don has beaten flag 34
        assert_eq!(m.npcs[2].event_flag, Some(34));
    }
}
