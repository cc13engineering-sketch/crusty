# Sprint 1 Implementation Plan: New Bark Town Starting Area

> Written by Frodo (Technical Writer)
> Source: ARCHITECTURE.md (Gandalf + Aragorn), SPRINT1_POKEMON_REFERENCE.md (Gimli)
> Target: Rust engineers implementing pokemonv2/
> Rev 2: Updated for Aragorn's architectural recommendations (static slices, SceneState Vec, temp_flags, script_id indirection, u8 coords)
> Rev 3: Bilbo's review modifications (circular dep resolution, SceneState determinism fix, Emote location, syntax fixes)

---

## Module File Plan

After Sprint 1, the directory looks like:

```
pokemonv2/
  mod.rs          -- PokemonV2Sim, GamePhase, Simulation impl, input helpers
  data.rs         -- PokemonType, SpeciesId, MoveId, Pokemon, StatusCondition, items,
                     Direction, Emote, PlayerState, NpcState, CameraState (shared types)
  maps.rs         -- MapId, MapData, WarpDef, NpcDef, CoordEvent, BgEvent, load_map()
  overworld.rs    -- step_overworld(), collision, warps, camera helpers, NPC wandering
  events.rs       -- ScriptStep, ScriptState, step_script(), EventFlags, SceneState, script registry
  render.rs       -- render_game() dispatch, tile rendering, sprites, text box
  dialogue.rs     -- DialogueState (standalone NPC talk, not scripted sequences)
  sprites.rs      -- Tile/sprite generation (procedural, no assets)
```

### CRITICAL: Circular Dependency Resolution (Bilbo Rev 3)

The import graph MUST be acyclic. To achieve this, all shared data types live in `data.rs`:
- `Direction`, `Emote`, `PlayerState`, `NpcState`, `CameraState` live in **data.rs** (not overworld.rs)
- `ScriptStep`, `EventFlags`, `SceneState` live in **events.rs**
- `MapId`, `MapData`, `NpcDef`, etc. live in **maps.rs**

**Acyclic import graph:**
```
data.rs       <- LEAF (no sibling imports)
events.rs     <- imports data.rs only (Direction, SpeciesId, Emote, PlayerState, NpcState)
maps.rs       <- imports data.rs only (Direction, SpeciesId); uses script_id: u16 (no ScriptStep import needed)
overworld.rs  <- imports data.rs (Direction, PlayerState, NpcState, CameraState), maps.rs (*), events.rs (EventFlags, SceneState)
render.rs     <- imports data.rs, maps.rs, overworld.rs (constants), events.rs
dialogue.rs   <- LEAF
sprites.rs    <- imports data.rs (Direction, Emote)
mod.rs        <- imports everything
```

No module imports from a module that imports back from it. This compiles.

`battle.rs` is NOT created in Sprint 1 (no battles in New Bark Town scope).

---

## Phase 1: Core Data Types (`data.rs`)

**File**: `pokemonv2/data.rs`
**Goal**: All Pokemon data structures compile. No game logic yet.

### Type Aliases

```rust
pub type SpeciesId = u16;
pub type MoveId = u16;
```

### Enums

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PokemonType {
    Normal, Fire, Water, Electric, Grass, Ice,
    Fighting, Poison, Ground, Flying, Psychic, Bug,
    Rock, Ghost, Dragon, Dark, Steel,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StatusCondition {
    None, Poison, Burn, Paralyze, Sleep(u8), Freeze,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GrowthRate {
    Fast, MediumFast, MediumSlow, Slow,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up, Down, Left, Right,
}

/// Emote bubble types (lives in data.rs to avoid circular deps)
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Emote {
    Shock,
    Question,
    Happy,
}
```

### Shared Runtime State Types (Bilbo Rev 3: moved here from overworld.rs to break circular deps)

```rust
/// Player position and movement state.
#[derive(Clone, Debug)]
pub struct PlayerState {
    pub x: i32,           // tile x
    pub y: i32,           // tile y
    pub facing: Direction,
    pub walk_offset: f64, // 0.0 = centered on tile, moves toward TILE_PX
    pub is_walking: bool,
    pub walk_frame: u8,   // animation frame (0-3)
    pub frame_timer: f64, // accumulator for walk animation
    pub name: String,
}

/// Mutable per-NPC state tracked at runtime (not in map definition).
#[derive(Clone, Debug)]
pub struct NpcState {
    pub x: i32,           // i32 for interpolation math; initialized from NpcDef.x as i32
    pub y: i32,
    pub facing: Direction,
    pub walk_offset: f64,
    pub is_walking: bool,
    pub visible: bool,
    pub wander_timer: f64,
}

/// Camera position for viewport centering.
#[derive(Clone, Debug)]
pub struct CameraState {
    pub x: f64,
    pub y: f64,
}
```

### Structs

```rust
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
    pub dvs: [u8; 4],    // Gen 2 DVs (Atk, Def, Spd, Spc) — 0..15
    pub evs: [u16; 5],   // HP, Atk, Def, Spd, Spc — stat exp (0..65535)
    pub held_item: Option<u8>,
}
```

### Species Data

```rust
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
    pub learnset: &'static [(u8, MoveId)],  // (level, move_id)
}
```

### Move Data

```rust
#[derive(Clone, Debug)]
pub struct MoveData {
    pub id: MoveId,
    pub name: &'static str,
    pub move_type: PokemonType,
    pub power: u8,
    pub accuracy: u8,
    pub pp: u8,
    pub is_special: bool,
}
```

### Functions

```rust
/// Return species data for the given id. Panics on unknown species.
pub fn species_data(id: SpeciesId) -> &'static SpeciesData { ... }

/// Return move data for the given id. Panics on unknown move.
pub fn move_data(id: MoveId) -> &'static MoveData { ... }

/// Gen 2 type effectiveness. Returns 1.0, 2.0, 0.5, or 0.0.
pub fn type_effectiveness(atk: PokemonType, def: PokemonType) -> f64 { ... }

/// Create a new Pokemon at the given level with default DVs/EVs and level-up moves.
impl Pokemon {
    pub fn new(species: SpeciesId, level: u8) -> Self { ... }
    /// Recalculate stats from base stats, DVs, EVs, and level.
    pub fn recalc_stats(&mut self) { ... }
}
```

### Constants — Sprint 1 Species IDs

```rust
pub const CHIKORITA: SpeciesId = 152;
pub const CYNDAQUIL: SpeciesId = 155;
pub const TOTODILE: SpeciesId = 158;
```

### Constants — Sprint 1 Move IDs

```rust
pub const MOVE_TACKLE: MoveId = 33;
pub const MOVE_GROWL: MoveId = 45;
pub const MOVE_LEER: MoveId = 43;
pub const MOVE_SCRATCH: MoveId = 10;
```

### Constants — Item IDs

```rust
pub const ITEM_BERRY: u8 = 3;
pub const ITEM_POTION: u8 = 17;
pub const ITEM_POKEGEAR: u8 = 59;
```

### Sprint 1 Scope vs. Stubs

- **Sprint 1**: Chikorita, Cyndaquil, Totodile species data + their Lv1-5 moves. `type_effectiveness()` full 17x17 table. `Pokemon::new()` with stat calc.
- **Stub**: `species_data()` only needs 3 entries. `move_data()` only needs Tackle/Growl/Leer/Scratch. More species/moves added in future sprints.

### Dependencies

None. This is a leaf module.

### Compile Check

After Phase 1: `cargo check` succeeds with data.rs fully compiling. No other modules import it yet.

---

## Phase 2: Map System (`maps.rs`)

**File**: `pokemonv2/maps.rs`
**Goal**: MapId enum, MapData struct, all warp/NPC/event types, `load_map()` returns data for 4 maps.

### Key Architectural Decision: Static Slices for Immutable Map Data

Per Aragorn's recommendation, tiles, collision, and warps use `&'static` slices for zero-allocation immutable data. NPCs, coord_events, and bg_events remain `Vec` because they are filtered by event flags at load time.

### Enums

```rust
#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub enum MapId {
    NewBarkTown,
    PlayersHouse1F,
    PlayersHouse2F,
    ElmsLab,
    // Stub entries for warp targets that exist but aren't fully built:
    ElmsHouse,
    PlayersNeighborsHouse,
    Route29,
    Route27,
}

/// NPC movement patterns. Matches pokecrystal's SPRITEMOVEDATA_ constants.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NpcMoveType {
    Still,
    Standing(Direction),        // faces one direction, never moves
    SpinRandom,                 // rotates randomly (slow)
    WalkUpDown,                 // walks up and down within range
    WalkLeftRight,              // walks left and right within range
    // Future: Wander, Follow, etc.
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BgEventKind {
    Read,                       // BGEVENT_READ — interact facing any direction
    FaceUp,                     // BGEVENT_UP — must face up to interact
    FaceDown,                   // BGEVENT_DOWN — must face down
    IfSet(u16),                 // BGEVENT_IFSET — only active when flag is set
}

// NOTE: Emote enum is defined in data.rs (Bilbo Rev 3 — shared type)
```

### Collision Tile Constants

```rust
pub const C_FLOOR: u8 = 0;    // walkable
pub const C_WALL: u8 = 1;     // solid
pub const C_WATER: u8 = 2;    // impassable without Surf
pub const C_WARP: u8 = 3;     // triggers warp check (mats, doors)
pub const C_COUNTER: u8 = 4;  // can interact across (like Elm's desk)
```

### Structs

```rust
pub struct MapData {
    pub id: MapId,
    pub name: &'static str,           // display name (e.g., "NEW BARK TOWN")
    pub width: u8,                     // u8 sufficient (max ~40 tiles wide)
    pub height: u8,
    pub tiles: &'static [u8],         // visual tile IDs, row-major (STATIC, zero alloc)
    pub collision: &'static [u8],     // collision type per tile (STATIC, zero alloc)
    pub warps: &'static [WarpDef],    // warp points (STATIC, zero alloc)
    pub npcs: Vec<NpcDef>,            // Vec: NPCs filtered by event flags at load
    pub coord_events: Vec<CoordEvent>,
    pub bg_events: Vec<BgEvent>,
    pub wild_encounters: Vec<WildEncounter>,
    pub connections: MapConnections,   // N/S/E/W scrolling edge connections
    pub music_id: u8,
}

/// Door-to-door warp (interior transitions, stairs).
/// Coordinates are u8 (tile positions within the map).
pub struct WarpDef {
    pub x: u8,
    pub y: u8,
    pub dest_map: MapId,
    pub dest_warp_id: u8,  // index into destination map's warp array
}

/// Scrolling edge connections (outdoor map-to-map seamless transitions).
pub struct MapConnections {
    pub north: Option<MapConnection>,
    pub south: Option<MapConnection>,
    pub east: Option<MapConnection>,
    pub west: Option<MapConnection>,
}

pub struct MapConnection {
    pub direction: Direction,
    pub dest_map: MapId,
    pub offset: i8,  // tile offset for alignment
}

pub struct NpcDef {
    pub x: u8,
    pub y: u8,
    pub sprite_id: u8,
    pub move_type: NpcMoveType,       // Standing(Dir), SpinRandom, WalkUpDown, etc.
    pub script_id: u16,               // index into script registry (events.rs)
    pub event_flag: Option<u16>,      // flag index; controls NPC visibility
    pub event_flag_show: bool,        // false = hide when set, true = show when set
    pub palette: u8,
    pub facing: Direction,
    pub name: &'static str,           // debug/display name
}

pub struct CoordEvent {
    pub x: u8,
    pub y: u8,
    pub scene_id: u8,
    pub script_id: u16,               // index into script registry (events.rs)
}

pub struct BgEvent {
    pub x: u8,
    pub y: u8,
    pub kind: BgEventKind,
    pub script_id: u16,               // index into script registry (events.rs)
}

pub struct WildEncounter {
    pub species: SpeciesId,
    pub min_level: u8,
    pub max_level: u8,
    pub rate: u8,
}
```

### NPC Runtime State (lives in `data.rs`, not here -- Bilbo Rev 3)

Per Bilbo's circular dependency resolution, `NpcState` is defined in `data.rs` alongside `PlayerState` and `CameraState`. See Phase 1 "Shared Runtime State Types" section above.

### Functions

```rust
/// Load map data for the given MapId. Returns a fully populated MapData.
/// Tiles, collision, and warps point to static data. NPCs/events are built as Vecs.
pub fn load_map(id: MapId) -> MapData { ... }

/// Check if tile at (x, y) is walkable. Returns false for out-of-bounds.
/// Coordinates are i32 to support negative values from boundary checks.
pub fn is_walkable(map: &MapData, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 || x >= map.width as i32 || y >= map.height as i32 { return false; }
    let idx = (y as usize) * (map.width as usize) + (x as usize);
    map.collision[idx] == C_FLOOR || map.collision[idx] == C_WARP
}

/// Find warp at position (x, y), if any.
pub fn find_warp(map: &MapData, x: i32, y: i32) -> Option<&WarpDef> {
    map.warps.iter().find(|w| w.x as i32 == x && w.y as i32 == y)
}

/// Find coord_event at position (x, y) matching the given scene_id.
pub fn find_coord_event(map: &MapData, x: i32, y: i32, scene_id: u8) -> Option<&CoordEvent> {
    map.coord_events.iter().find(|e| e.x as i32 == x && e.y as i32 == y && e.scene_id == scene_id)
}

/// Find bg_event at position (x, y) matching the interaction direction.
pub fn find_bg_event(map: &MapData, x: i32, y: i32, facing: Direction) -> Option<&BgEvent> {
    map.bg_events.iter().find(|e| {
        e.x as i32 == x && e.y as i32 == y && match e.kind {
            BgEventKind::Read => true,
            BgEventKind::FaceUp => facing == Direction::Up,
            BgEventKind::FaceDown => facing == Direction::Down,
            BgEventKind::IfSet(_) => true, // flag check done by caller
        }
    })
}
```

### Static Data Pattern for Map Tiles/Collision/Warps

Each map's immutable data is declared as `static` arrays:

```rust
// Example: PlayersHouse2F
static PLAYERS_HOUSE_2F_TILES: [u8; 48] = [ /* 8*6 tile IDs */ ];
static PLAYERS_HOUSE_2F_COLLISION: [u8; 48] = [ /* 8*6 collision */ ];
static PLAYERS_HOUSE_2F_WARPS: [WarpDef; 1] = [
    WarpDef { x: 7, y: 0, dest_map: MapId::PlayersHouse1F, dest_warp_id: 2 },
];

// In load_map():
MapId::PlayersHouse2F => MapData {
    id: MapId::PlayersHouse2F,
    name: "PLAYER'S HOUSE 2F",
    width: 8,
    height: 6,
    tiles: &PLAYERS_HOUSE_2F_TILES,
    collision: &PLAYERS_HOUSE_2F_COLLISION,
    warps: &PLAYERS_HOUSE_2F_WARPS,
    npcs: vec![/* ... */],
    // ...
}
```

This eliminates heap allocation for all immutable map geometry.

### Map Data Encoding — Sprint 1 Maps

All coordinates from SPRINT1_POKEMON_REFERENCE.md (Gimli's file), sourced from pokecrystal `.asm` files.

#### PlayersHouse2F (8 x 6 tiles)

- **Warps** (1):
  - (7, 0) -> PlayersHouse1F, dest_warp_id=2 (stairs)
- **NPCs** (4 decoration objects): Console (4,2), Doll_1 (4,4), Doll_2 (5,4), BigDoll (0,1)
  - All decorations are STILL, no interaction scripts in Sprint 1 scope
- **BG Events** (4):
  - PC at (2,1) kind=Up: "Accessed own PC."
  - Radio at (3,1) kind=Read: Pokemon Talk intro text
  - Bookshelf at (5,1) kind=Read: "Picture book."
  - Poster at (6,0) kind=IfSet(flag): conditional decoration poster
- **Coord Events**: none
- **Collision**: walls around perimeter, furniture tiles are C_WALL, floor is C_FLOOR, stair tile is C_WARP
- **Initial scene**: 0 (no scenes)
- Player spawn point: (3, 3)

#### PlayersHouse1F (10 x 8 tiles)

- **Warps** (3):
  - (6, 7) -> NewBarkTown, dest_warp_id=1 (exit door)
  - (7, 7) -> NewBarkTown, dest_warp_id=1 (exit door)
  - (9, 0) -> PlayersHouse2F, dest_warp_id=0 (stairs up)
- **NPCs** (5):
  - Mom1 at (7,4) facing=Left, event_flag=EVENT_PLAYERS_HOUSE_MOM_1, event_flag_show=false (visible when flag NOT set — first visit)
  - Mom2-morning at (2,2) facing=Up, event_flag=EVENT_PLAYERS_HOUSE_MOM_2, event_flag_show=true (visible when set, morning only) -- **Sprint 1 stub**: only Mom1 variant active initially
  - Mom3-day at (7,4) facing=Left, event_flag=EVENT_PLAYERS_HOUSE_MOM_2, event_flag_show=true -- stub
  - Mom4-night at (0,2) facing=Up, event_flag=EVENT_PLAYERS_HOUSE_MOM_2, event_flag_show=true -- stub
  - Neighbor (Pokefan_F) at (4,4) facing=Right, event_flag=EVENT_PLAYERS_HOUSE_1F_NEIGHBOR, event_flag_show=true
- **BG Events** (4):
  - Stove at (0,1) kind=Read: "CINNABAR VOLCANO BURGER! Make your Pokemon do a double take!"
  - Sink at (1,1) kind=Read: "The sink is spotless."
  - Fridge at (2,1) kind=Read: "FRESH WATER and LEMONADE."
  - TV at (4,1) kind=Read: "A movie about two boys on a train."
- **Coord Events** (2):
  - (8, 4) scene_id=SCENE_PLAYERSHOUSE1F_MEET_MOM (0): MeetMom script
  - (9, 4) scene_id=SCENE_PLAYERSHOUSE1F_MEET_MOM (0): MeetMom script
- **Initial scene**: 0 (SCENE_PLAYERSHOUSE1F_MEET_MOM)

**MeetMom Script** (script_id=SCRIPT_MEET_MOM, triggered at coord_event):
```
FacingPlayer { npc_idx: 0 }         // Mom turns to face player
ShowText("MOM: Oh, <PLAYER>! ...")
ShowText("MOM: ...PROF.ELM was looking for you.")
ShowText("MOM: Here, take this. POKEGEAR.")
GiveItem { item_id: ITEM_POKEGEAR, count: 1 }
SetEvent(EVENT_ENGINE_POKEGEAR)
ShowText("MOM: PROF.ELM is in his lab next door.")
SetScene { map: MapId::PlayersHouse1F, scene_id: 1 }  // advance to NOOP
SetEvent(EVENT_PLAYERS_HOUSE_MOM_1)   // clear Mom1, activate Mom2 variants
SetEvent(EVENT_PLAYERS_HOUSE_MOM_2)   // activate time-of-day Mom positions
End
```

#### NewBarkTown (18 x 20 tiles)

Note: Gimli's reference says 9 blocks wide x 10 blocks tall. Each block = 2 tiles. So **18 tiles wide x 20 tiles tall** (not 20x18 as in the architecture doc — Gimli's pokecrystal-derived numbers are authoritative).

- **Warps** (4):
  - (6, 3) -> ElmsLab, dest_warp_id=0
  - (13, 5) -> PlayersHouse1F, dest_warp_id=0
  - (3, 11) -> PlayersNeighborsHouse, dest_warp_id=0
  - (11, 13) -> ElmsHouse, dest_warp_id=0
- **NPCs** (3):
  - Teacher at (6, 8), SpinRandom, no event flag (always visible)
  - Fisher at (12, 9), WalkUpDown, no event flag
  - Rival (Silver) at (3, 2), StandingRight, event_flag=EVENT_RIVAL_NEW_BARK_TOWN, event_flag_show=true
- **BG Events** (4 signs):
  - (8, 8) kind=Read: "NEW BARK TOWN\nThe Town Where the Winds of a New Beginning Blow."
  - (11, 5) kind=Read: "<PLAYER>'s House"
  - (3, 3) kind=Read: "ELM POKeMON LAB"
  - (9, 13) kind=Read: "ELM'S HOUSE"
- **Coord Events** (2):
  - (1, 8) scene_id=0 (SCENE_NEWBARKTOWN_TEACHER_STOPS_YOU): TeacherStopsYouScene1
  - (1, 9) scene_id=0 (SCENE_NEWBARKTOWN_TEACHER_STOPS_YOU): TeacherStopsYouScene2
- **Connections** (2):
  - West -> Route29, offset=0
  - East -> Route27, offset=0
- **Initial scene**: 0 (SCENE_NEWBARKTOWN_TEACHER_STOPS_YOU)

**Teacher Stops You Scene 1** (coord at x=1, y=8):
```
ShowText("TEACHER: Hey, wait! It's dangerous to go out without a POKeMON!")
ShowText("TEACHER: Wild POKeMON jump out of the grass on the way to the next town.")
MovePlayer { steps: [(Right, 4)] }
TurnPlayer(Left)
End
```

**Teacher Stops You Scene 2** (coord at x=1, y=9):
```
ShowText("TEACHER: Hey, wait! It's dangerous to go out without a POKeMON!")
ShowText("TEACHER: Wild POKeMON jump out of the grass on the way to the next town.")
MovePlayer { steps: [(Right, 5)] }
TurnPlayer(Left)
End
```

**Rival Interaction Script**:
```
FacingPlayer { npc_idx: 2 }
ShowText("...So this is the famous ELM POKeMON LAB...")
TurnNpc { npc_idx: 2, direction: Left }
Pause(0.3)
TurnNpc { npc_idx: 2, direction: Right }
ShowText("...What are you staring at?")
End
```

#### ElmsLab (10 x 12 tiles)

Note: Gimli's .blk analysis says 30 bytes / 10-wide x 3-tall blocks. With standard lab interior tileset, this is **10 tiles wide x 12 tiles tall** (each block-row = 4 tile-rows for lab tileset).

- **Warps** (2):
  - (4, 11) -> NewBarkTown, dest_warp_id=0
  - (5, 11) -> NewBarkTown, dest_warp_id=0
- **NPCs** (6):
  - Elm at (5, 2) facing=Down, no event_flag (always visible). Name: "PROF.ELM"
    - **Important**: During SCENE_ELMSLAB_MEET_ELM (scene 0), Elm is repositioned to (3, 4) by map callback before the walk-in cutscene. After cutscene, Elm walks back to (5, 2).
  - Aide at (2, 9) SpinRandom, event_flag=EVENT_ELMS_AIDE_IN_LAB, event_flag_show=true
    - Initially hidden (flag not set). Visible after theft event. **Sprint 1 scope**: not visible.
  - PokeBall1 (Cyndaquil) at (6, 3) Still, event_flag=EVENT_CYNDAQUIL_POKEBALL_IN_ELMS_LAB, event_flag_show=false (visible when flag NOT set)
  - PokeBall2 (Totodile) at (7, 3) Still, event_flag=EVENT_TOTODILE_POKEBALL_IN_ELMS_LAB, event_flag_show=false
  - PokeBall3 (Chikorita) at (8, 3) Still, event_flag=EVENT_CHIKORITA_POKEBALL_IN_ELMS_LAB, event_flag_show=false
  - Officer at (5, 3) StandingUp, event_flag=EVENT_COP_IN_ELMS_LAB, event_flag_show=true
    - **Sprint 1 stub**: Officer not visible initially. Appears after egg-return event (future sprint).
- **BG Events** (12+):
  - Healing machine at (2, 1) kind=Read
  - Bookshelves at (6,1), (7,1), (8,1), (9,1) kind=Read
  - Travel tip books at (0,7), (1,7), (2,7), (3,7) kind=Read
  - Trashcan at (9, 3) kind=Read: "The wrapper from the snack ELM ate."
  - Window at (5, 0) kind=Read: "The window is open." / "The window is broken!" (post-theft)
  - PC at (3, 5) kind=Down: "ELM's research notes."
- **Coord Events** (8):
  - (4, 6) scene_id=1 (SCENE_ELMSLAB_CANT_LEAVE): LabTryToLeaveScript
  - (5, 6) scene_id=1 (SCENE_ELMSLAB_CANT_LEAVE): LabTryToLeaveScript
  - (4, 5) scene_id=3 (SCENE_ELMSLAB_MEET_OFFICER): MeetCopScript -- **future sprint stub**
  - (5, 5) scene_id=3 (SCENE_ELMSLAB_MEET_OFFICER): MeetCopScript -- **future sprint stub**
  - (4, 8) scene_id=5 (SCENE_ELMSLAB_AIDE_GIVES_POTION): AideGivesPotion -- **future sprint stub**
  - (5, 8) scene_id=5 (SCENE_ELMSLAB_AIDE_GIVES_POTION): AideGivesPotion -- **future sprint stub**
  - (4, 8) scene_id=6 (SCENE_ELMSLAB_AIDE_GIVES_POKE_BALLS): AideGivesBalls -- **future sprint stub**
  - (5, 8) scene_id=6 (SCENE_ELMSLAB_AIDE_GIVES_POKE_BALLS): AideGivesBalls -- **future sprint stub**
- **Initial scene**: 0 (SCENE_ELMSLAB_MEET_ELM)

**Scene constants** (from pokecrystal, Gimli's reference):
```rust
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
```

**Elm Walk-In Cutscene** (fires on first entry when scene == SCENE_ELMSLAB_MEET_ELM):

This is a special coord_event triggered at map entry. Implementation: when map loads with scene == 0, immediately begin this script rather than using a tile-based coord_event.

```
// Elm is pre-positioned at (3, 4) by map init callback
MovePlayer { steps: [(Up, 7)] }       // Player walks up 7 tiles from door
TurnPlayer(Left)                       // Face Elm
ShowEmote { npc_idx: 0, emote: Shock, frames: 15 }
TurnNpc { npc_idx: 0, direction: Right }  // Elm faces player
ShowText("ELM: <PLAYER>! There you are!")
ShowText("ELM: I needed to ask you a favor.")
ShowText("ELM: I'm writing a research paper...")
ShowText("ELM: You see, a Pokemon acquaintance of mine, MR.POKEMON, says he has a discovery.")
ShowText("ELM: Will you go see what it's about?")
// YesNo loop — must say yes (loop on no)
PlaySound(SFX_GLASS_TING)             // email notification
Pause(0.5)
ShowEmote { npc_idx: 0, emote: Shock, frames: 10 }
TurnNpc { npc_idx: 0, direction: Down }
ShowText("ELM: Hm? I have an email!")
ShowText("ELM: ...It's from MR.POKEMON!")
ShowText("ELM: Go ahead — pick a Pokemon for your journey!")
// Move Elm back to default position
MoveNpc { npc_idx: 0, steps: [(Up, 1)] }
MoveNpc { npc_idx: 0, steps: [(Right, 2), (Up, 1)] }
TurnNpc { npc_idx: 0, direction: Down }
TurnPlayer(Up)
TurnPlayer(Right)
SetScene { map: MapId::ElmsLab, scene_id: 1 }   // SCENE_ELMSLAB_CANT_LEAVE
End
```

**Starter Selection** — Triggered by interacting with Pokeball NPCs (idx 2/3/4):

```
// PokeBall1 (Cyndaquil) interaction script:
ShowText("It's CYNDAQUIL, the fire Pokemon. Will you take it?")
YesNo { yes_jump: 3, no_jump: LAST }
// yes:
GivePokemon { species: CYNDAQUIL, level: 5, held_item: ITEM_BERRY }
SetEvent(EVENT_GOT_CYNDAQUIL_FROM_ELM)
SetEvent(EVENT_GOT_A_POKEMON_FROM_ELM)
SetEvent(EVENT_CYNDAQUIL_POKEBALL_IN_ELMS_LAB)
HideNpc(2)                           // hide this pokeball
MovePlayer { steps: [(Left, 1), (Up, 1)] }
TurnPlayer(Up)
// Elm speaks about the starter
ShowText("ELM: I knew you'd pick that one!")
ShowText("ELM: You can use that healing machine any time.")
ShowText("ELM: Now, head to MR.POKEMON's place on Route 30!")
// Update scenes: Elm lab free, NBT teacher stops blocking
SetScene { map: MapId::ElmsLab, scene_id: 2 }        // SCENE_ELMSLAB_NOOP
SetScene { map: MapId::NewBarkTown, scene_id: 1 }    // SCENE_NEWBARKTOWN_NOOP
End
```

Similar scripts for Totodile (idx 3) and Chikorita (idx 4) with species-specific text and movement offsets:
- Cyndaquil: player moves Left 1
- Totodile: player moves Left 2
- Chikorita: player moves Left 3

**LabTryToLeave Script** (scene=1, coord at y=6):
```
ShowText("ELM: Wait! You need to try out that Pokemon first!")
ShowText("ELM: Go ahead and use the healing machine!")
MovePlayer { steps: [(Up, 1)] }      // Push player back
End
```

### Stub Maps (warp targets only)

`ElmsHouse` and `PlayersNeighborsHouse` — these are generic house interiors. Sprint 1 only needs them as warp targets. Implement as a minimal 10x8 room with one warp back to NewBarkTown at the correct warp_id.

```rust
MapId::ElmsHouse => {
    // Minimal room: 10x8, one exit warp
    // Warp 0: (6, 7) -> NewBarkTown, dest_warp_id=3
}
MapId::PlayersNeighborsHouse => {
    // Minimal room: 10x8, one exit warp
    // Warp 0: (6, 7) -> NewBarkTown, dest_warp_id=2
}
```

### Dependencies (Bilbo Rev 3: maps.rs imports data.rs ONLY -- no events.rs import needed since we use script_id: u16)

- `use super::data::{Direction, SpeciesId, NpcState};`

### Compile Check

After Phase 2: data.rs + maps.rs compile (NO dependency on events.rs). `load_map()` returns valid MapData for all 6+ MapIds. Tests can call `load_map(MapId::NewBarkTown)` and inspect warps/NPCs.

**NOTE (Bilbo Rev 3):** `load_map()` must also handle `MapId::Route29` and `MapId::Route27` as minimal stub maps (e.g. 4x4 empty room) to avoid panics, since they are in the MapId enum.

---

## Phase 3: Event System (`events.rs`)

**File**: `pokemonv2/events.rs`
**Goal**: ScriptStep enum, ScriptState, step_script() function, event flag system, scene state, temp flags, script registry.

### Event Flag System

```rust
/// Event flags stored as a bitfield array.
/// Pokecrystal has 1,332 event flags. We use [u64; 32] = 2048 bits.
/// Why bitfield over HashMap:
/// - Deterministic (engine requirement), compact (256 bytes), fast (bitshift vs hash),
///   serializable (raw array for saves), pokecrystal-compatible (maps to wEventFlags)
pub struct EventFlags {
    flags: [u64; 32],
}

impl EventFlags {
    pub fn new() -> Self { Self { flags: [0u64; 32] } }
    pub fn has(&self, id: u16) -> bool {
        let (word, bit) = (id as usize / 64, id as usize % 64);
        word < self.flags.len() && self.flags[word] & (1u64 << bit) != 0
    }
    pub fn set(&mut self, id: u16) {
        let (word, bit) = (id as usize / 64, id as usize % 64);
        if word < self.flags.len() { self.flags[word] |= 1u64 << bit; }
    }
    pub fn clear(&mut self, id: u16) {
        let (word, bit) = (id as usize / 64, id as usize % 64);
        if word < self.flags.len() { self.flags[word] &= !(1u64 << bit); }
    }
}
```

### Scene State

Per Aragorn's recommendation: `Vec<u8>` indexed by `MapId as usize`, not HashMap. Deterministic and zero-overhead lookup.

```rust
/// Per-map scene tracking. Indexed by MapId as usize, 0 = default scene.
pub struct SceneState {
    scenes: Vec<u8>,  // pre-sized to MapId variant count
}

impl SceneState {
    pub fn new() -> Self {
        // Pre-allocate for all MapId variants (8 in Sprint 1, grows with future sprints)
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
```

### Temporary Flags

Pokecrystal has 8 temporary flags (`EVENT_TEMPORARY_UNTIL_MAP_RELOAD_1-8`) that reset when the player transitions to a new map. Stored as a separate `u8` in `PokemonV2Sim`.

```rust
// In mod.rs PokemonV2Sim:
pub temp_flags: u8,    // 8 temporary flags, zeroed on map transition

// In change_map():
self.temp_flags = 0;   // reset temp flags on every map transition
```

### ScriptStep Enum

Updated names per architecture: `SetEvent`/`ClearEvent`/`CheckEvent` (not SetFlag/ClearFlag/CheckFlag). `SetScene` takes explicit `map: MapId` parameter. `GivePokemon` includes `held_item`.

```rust
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
    PlaySound(u8),     // SoundId placeholder
    PlayMusic(u8),     // MusicId placeholder
    Pause(f64),        // seconds

    // Game state
    SetEvent(u16),     // set event flag by ID
    ClearEvent(u16),   // clear event flag by ID
    SetScene { map: MapId, scene_id: u8 },  // set a specific map's scene counter
    GiveItem { item_id: u8, count: u8 },
    GivePokemon { species: SpeciesId, level: u8, held_item: u8 },
    HideNpc(u8),
    ShowNpc(u8),

    // Control flow
    CheckEvent { flag: u16, jump_if_true: usize },
    YesNo { yes_jump: usize, no_jump: usize },
    Jump(usize),       // unconditional jump to step index
    End,

    // NPC facing
    FacingPlayer { npc_idx: u8 },  // NPC turns to face the player
    Heal,                           // heal party
}
```

### Script Registry

NpcDef, CoordEvent, and BgEvent reference scripts by `script_id: u16` (not inline `Vec<ScriptStep>`). The registry resolves IDs to scripts:

```rust
/// Script ID constants for Sprint 1.
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
// Future sprint stubs:
pub const SCRIPT_MEET_OFFICER: u16 = 100;
pub const SCRIPT_AIDE_GIVES_POTION: u16 = 101;
pub const SCRIPT_AIDE_GIVES_BALLS: u16 = 102;

/// Look up a script by ID. Returns the ScriptStep sequence.
/// Scripts are built by builder functions (build_meet_mom_script(), etc.)
pub fn get_script(id: u16) -> Vec<ScriptStep> {
    match id {
        SCRIPT_MEET_MOM => build_meet_mom_script(),
        SCRIPT_TEACHER_STOPS_1 => build_teacher_stops_script(1),
        SCRIPT_TEACHER_STOPS_2 => build_teacher_stops_script(2),
        SCRIPT_RIVAL_INTERACTION => build_rival_interaction_script(),
        SCRIPT_ELM_INTRO => build_elm_intro_script(),
        SCRIPT_STARTER_CYNDAQUIL => build_starter_script(0),
        SCRIPT_STARTER_TOTODILE => build_starter_script(1),
        SCRIPT_STARTER_CHIKORITA => build_starter_script(2),
        SCRIPT_LAB_TRY_TO_LEAVE => build_lab_try_to_leave_script(),
        // Signs and BG events return simple ShowText scripts:
        SCRIPT_NBT_SIGN => vec![
            ScriptStep::ShowText("NEW BARK TOWN\nThe Town Where the Winds of a New Beginning Blow.".into()),
            ScriptStep::End,
        ],
        // ... other sign/bg scripts follow same pattern
        _ => vec![ScriptStep::End],  // unknown script = no-op
    }
}
```

### ScriptState

```rust
#[derive(Clone, Debug)]
pub struct ScriptState {
    pub steps: Vec<ScriptStep>,
    pub pc: usize,
    pub timer: f64,
    pub waiting_for_input: bool,
    pub text_buffer: Option<String>,
    pub showing_yesno: bool,
    pub yesno_cursor: u8,
    pub move_queue: Vec<(Direction, u8)>,  // remaining movement steps
    pub move_progress: f64,                // pixel progress on current move step
}
```

### Functions

```rust
/// Create a new ScriptState from a list of steps.
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
        }
    }

    /// Convenience: create ScriptState from a script_id via the registry.
    pub fn from_id(script_id: u16) -> Self {
        Self::new(get_script(script_id))
    }
}

/// Advance the script engine by one frame.
/// Returns true if the script is still running, false if it hit End.
///
/// This is the core script processing loop. Each frame:
/// 1. If waiting_for_input and player presses confirm, advance pc
/// 2. If move_queue is non-empty, animate movement (WALK_SPEED px/frame)
/// 3. Otherwise, execute the current ScriptStep:
///    - ShowText: set text_buffer, set waiting_for_input = true
///    - WaitButton: set waiting_for_input = true
///    - CloseText: clear text_buffer, advance pc
///    - MoveNpc/MovePlayer: populate move_queue, advance pc after last step
///    - TurnNpc/TurnPlayer: instant, advance pc
///    - SetEvent/ClearEvent: modify EventFlags, advance pc
///    - SetScene: modify SceneState for specified map, advance pc
///    - GiveItem: add to bag, advance pc
///    - GivePokemon: create Pokemon::new(species, level) with held_item, add to party, advance pc
///    - HideNpc/ShowNpc: toggle NpcState.visible, advance pc
///    - Pause: decrement timer, advance pc when <= 0
///    - CheckEvent: check EventFlags, jump or advance
///    - YesNo: show yes/no UI, wait for input, jump to yes_jump or no_jump
///    - Jump: set pc to target
///    - End: return false
///    - FacingPlayer: compute direction from NPC to player, set NPC facing
///    - Heal: restore all party HP to max
///    - ShowEmote: set emote timer, advance when done
pub fn step_script(
    script: &mut ScriptState,
    player: &mut PlayerState,
    npc_states: &mut Vec<NpcState>,
    flags: &mut EventFlags,
    scenes: &mut SceneState,
    current_map_id: MapId,
    party: &mut Vec<Pokemon>,
    bag: &mut Vec<(u8, u8)>,
    confirm_pressed: bool,
    cancel_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,
) -> bool { ... }
```

### Event Flag Constants (Sprint 1)

```rust
// Flag indices (u16). These map to pokecrystal's event constants.
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
```

### Dependencies (Bilbo Rev 3: events.rs imports data.rs ONLY -- no overworld.rs import)

- `use super::data::{Direction, SpeciesId, Pokemon, Emote, PlayerState, NpcState};`
- `use super::maps::MapId;`

### Compile Check

After Phase 3: events.rs compiles. ScriptState can be created from script_id. step_script can be called (overworld.rs PlayerState/NpcState needed — implement minimal stubs first).

---

## Phase 4: Overworld (`overworld.rs`)

**File**: `pokemonv2/overworld.rs`
**Goal**: Player movement, collision, warp transitions, camera, NPC wandering.

### Constants

```rust
pub const TILE_PX: i32 = 16;
pub const VIEW_TILES_X: i32 = 10;
pub const VIEW_TILES_Y: i32 = 9;
pub const WALK_SPEED: f64 = 8.0;   // pixels per frame at 60fps
pub const CAMERA_LERP: f64 = 0.2;
pub const NPC_WANDER_INTERVAL: f64 = 2.0; // seconds between random NPC steps
```

### Structs (Bilbo Rev 3: PlayerState, NpcState, CameraState are in data.rs now)

`PlayerState`, `NpcState`, and `CameraState` are imported from `data.rs`. No struct definitions needed here.

### Functions

```rust
/// Main overworld update. Called each frame when GamePhase == Overworld.
///
/// Logic flow:
/// 1. If player is_walking:
///    a. Advance walk_offset by WALK_SPEED
///    b. Tick frame_timer, cycle walk_frame every 4 frames
///    c. When walk_offset >= TILE_PX:
///       - Snap player to destination tile (update x/y based on facing)
///       - Set is_walking = false, walk_offset = 0.0
///       - Check coord_events at new (x, y) with current scene_id
///       - Check warps at new (x, y)
///       - (Wild encounters not in Sprint 1 scope)
///       Return early — no new input processed this frame.
///
/// 2. If player is NOT walking:
///    a. Read direction input (held_up/down/left/right)
///    b. If a direction is held:
///       - Update player.facing to that direction
///       - Compute target tile (x + dx, y + dy)
///       - Check collision: is_walkable(map, tx, ty) AND no NPC at (tx, ty)
///       - If walkable: set is_walking = true, walk_offset = 0.0
///    c. If confirm pressed (and not walking):
///       - Compute tile player is facing
///       - Check for NPC at that tile -> trigger NPC interaction script
///       - Check for BgEvent at that tile matching facing direction
///
/// 3. Update camera:
///    - Target = (player.x * TILE_PX + walk_dx, player.y * TILE_PX + walk_dy)
///    - camera.x += (target_x - camera.x) * CAMERA_LERP
///    - camera.y += (target_y - camera.y) * CAMERA_LERP
///
/// 4. Tick NPC wander timers:
///    - For each NPC with SpinRandom or WalkUpDown movement:
///      increment wander_timer. When >= NPC_WANDER_INTERVAL:
///      pick random direction, check collision, start walk if clear.
pub fn step_overworld(
    player: &mut PlayerState,
    camera: &mut CameraState,
    map: &MapData,
    npc_states: &mut Vec<NpcState>,
    flags: &EventFlags,
    scenes: &SceneState,
    engine: &Engine,
) -> OverworldResult { ... }

/// Result of one overworld step. Tells the caller what happened.
pub enum OverworldResult {
    Nothing,
    WarpTo { dest_map: MapId, dest_warp_id: u8 },
    TriggerScript { script_id: u16, npc_idx: Option<u8> },  // Bilbo Rev 3: u16 script_id, not Vec<ScriptStep>
    TriggerCoordEvent { script_id: u16 },                    // Bilbo Rev 3: u16 script_id
}

/// Check if an NPC occupies tile (x, y). Returns NPC index if found.
pub fn npc_at(npc_states: &[NpcState], x: i32, y: i32) -> Option<usize> { ... }

/// Process a map warp: look up destination map, find warp by dest_warp_id,
/// return the spawn position.
pub fn resolve_warp(dest_map: MapId, dest_warp_id: u8) -> (i32, i32) {
    let map = load_map(dest_map);
    // Find the warp in dest_map whose index == dest_warp_id
    // Player spawns 1 tile below the warp tile (facing down into the room)
    // or 1 tile above for upstairs warps
    (map.warps[dest_warp_id as usize].x, map.warps[dest_warp_id as usize].y + 1)
}

/// Snap camera to player position (used on map transitions).
pub fn snap_camera(camera: &mut CameraState, player: &PlayerState) {
    camera.x = (player.x * TILE_PX) as f64;
    camera.y = (player.y * TILE_PX) as f64;
}
```

### Dependencies

- `use super::data::{Direction, PlayerState, NpcState, CameraState};`
- `use super::maps::*;`
- `use super::events::{EventFlags, SceneState};`
- `use crate::engine::Engine;`

Bilbo Rev 3: `OverworldResult::TriggerScript` and `TriggerCoordEvent` return `u16` script_ids (not `Vec<ScriptStep>`) to keep overworld.rs free of ScriptStep imports. The caller (mod.rs) resolves script_id to steps via events.rs script registry.

### Compile Check

After Phase 4: All game logic modules compile. The overworld step function can be called from mod.rs.

---

## Phase 5: Render Pipeline (`render.rs`)

**File**: `pokemonv2/render.rs`
**Goal**: Tile rendering, sprites, text box, camera viewport. All rendering reads state, never mutates it.

### Constants

```rust
const TILE_PX: i32 = 16;
const VIEW_TILES_X: i32 = 10;
const VIEW_TILES_Y: i32 = 9;
const SCREEN_W: i32 = VIEW_TILES_X * TILE_PX;  // 160
const SCREEN_H: i32 = VIEW_TILES_Y * TILE_PX;  // 144
const CHAR_W: i32 = 6;
const CHAR_H: i32 = 8;
const TEXT_BOX_Y: i32 = SCREEN_H - 40;  // text box at bottom
const TEXT_MAX_CHARS: usize = 24;
```

### Functions

```rust
/// Top-level render dispatch.
pub fn render_game(sim: &PokemonV2Sim, engine: &mut Engine) {
    match sim.phase {
        GamePhase::TitleScreen => render_title(sim, engine),
        GamePhase::Overworld | GamePhase::Script => render_overworld(sim, engine),
        GamePhase::Dialogue => render_dialogue(sim, engine),
        GamePhase::StarterSelect { cursor } => render_starter_select(cursor, engine),
        GamePhase::MapTransition { timer } => render_transition(timer, engine),
        _ => {}
    }
}

/// Render the overworld view.
/// 1. Clear framebuffer to black
/// 2. Compute camera offset: cam_ox = camera.x - (SCREEN_W/2), cam_oy = camera.y - (SCREEN_H/2)
/// 3. For each tile in the viewport (visible tile range):
///    a. Compute screen position: sx = tile_x * TILE_PX - cam_ox, sy = tile_y * TILE_PX - cam_oy
///    b. Look up tile visual ID from map.tiles[y * width + x]
///    c. Draw 16x16 tile using procedural color palette (tile_color(tile_id))
/// 4. Draw NPCs:
///    a. For each visible NpcState, draw sprite at (npc.x * TILE_PX - cam_ox + walk_dx, ...)
///    b. Use sprite_id to select color/shape
/// 5. Draw player sprite at camera center + walk_offset
/// 6. If script text_buffer is Some, draw text box overlay
/// 7. If showing_yesno, draw yes/no choice box
fn render_overworld(sim: &PokemonV2Sim, engine: &mut Engine) { ... }

/// Draw a text box at the bottom of the screen with the given text.
/// Box: semi-transparent dark background, white border, white text.
/// Text is word-wrapped to TEXT_MAX_CHARS per line, max 2 visible lines.
fn draw_text_box(engine: &mut Engine, text: &str) { ... }

/// Draw a yes/no selection box (right side, above text box).
fn draw_yesno_box(engine: &mut Engine, cursor: u8) { ... }

/// Draw the title screen.
/// Centered "POKEMON CRYSTAL" text, pulsing indicator, "Press Start" text.
fn render_title(sim: &PokemonV2Sim, engine: &mut Engine) { ... }

/// Draw starter selection screen.
/// Three pokeball icons with names, cursor highlighting selection.
fn render_starter_select(cursor: u8, engine: &mut Engine) { ... }

/// Draw fade transition between maps.
/// timer 0.0..0.5: fade to black. 0.5..1.0: fade from black.
fn render_transition(timer: f64, engine: &mut Engine) { ... }

/// Render standalone dialogue (non-scripted NPC talk).
fn render_dialogue(sim: &PokemonV2Sim, engine: &mut Engine) { ... }
```

### Tile Color Palette (procedural, no assets)

```rust
/// Map tile visual ID to a color. Same approach as v1.
/// 0 = grass green, 1 = path tan, 2 = wall dark, 3 = water blue,
/// 4 = floor beige, 5 = wood brown, 6 = roof red, 7 = door, etc.
fn tile_color(tile_id: u8) -> Color { ... }

/// NPC sprite color by sprite_id.
fn sprite_color(sprite_id: u8) -> Color { ... }
```

### Text Rendering

```rust
/// Render a character at pixel position (px, py) using a built-in 6x8 bitmap font.
/// Same pattern as v1: each character is a column of 6 bits, 8 rows.
fn draw_char(engine: &mut Engine, ch: char, px: i32, py: i32, color: Color) { ... }

/// Render a string starting at (px, py). Newlines advance by CHAR_H.
fn draw_text(engine: &mut Engine, text: &str, px: i32, py: i32, color: Color) { ... }

/// Word-wrap text to max_chars per line.
fn wrap_text(text: &str, max_chars: usize) -> String { ... }
```

### Dependencies

- `use super::{PokemonV2Sim, GamePhase};`
- `use super::overworld::{TILE_PX, VIEW_TILES_X, VIEW_TILES_Y};`
- `use crate::engine::Engine;`
- `use crate::rendering::color::Color;`

### Compile Check

After Phase 5: Full render pipeline compiles. `render_game()` can be called from `PokemonV2Sim::render()`.

---

## Phase 6: Dialogue System (`dialogue.rs`)

**File**: `pokemonv2/dialogue.rs`
**Goal**: Standalone dialogue for simple NPC interactions and sign reading.

### Structs

```rust
#[derive(Clone, Debug)]
pub struct DialogueState {
    pub lines: Vec<String>,
    pub current_line: usize,
    pub char_index: usize,
    pub timer: f64,
    pub on_complete: DialogueAction,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DialogueAction {
    Resume,        // return to overworld
    StartScript(usize),  // chain into a script
}
```

### Functions

```rust
impl DialogueState {
    pub fn new(text: &str) -> Self { ... }
    /// Advance dialogue by one frame. Returns true while still active.
    pub fn step(&mut self, confirm_pressed: bool) -> bool { ... }
}
```

### Dependencies

None beyond std.

### Compile Check

Trivial module, compiles independently.

---

## Phase 7: Sprites System (`sprites.rs`)

**File**: `pokemonv2/sprites.rs`
**Goal**: Procedural sprite generation for player, NPCs, pokeballs, and tiles.

### Sprite ID Constants

```rust
pub const SPRITE_PLAYER: u8 = 0;
pub const SPRITE_MOM: u8 = 1;
pub const SPRITE_ELM: u8 = 2;
pub const SPRITE_TEACHER: u8 = 3;
pub const SPRITE_FISHER: u8 = 4;
pub const SPRITE_RIVAL: u8 = 5;
pub const SPRITE_POKE_BALL: u8 = 6;
pub const SPRITE_SCIENTIST: u8 = 7;
pub const SPRITE_OFFICER: u8 = 8;
pub const SPRITE_POKEFAN_F: u8 = 9;
pub const SPRITE_CONSOLE: u8 = 10;
pub const SPRITE_DOLL_1: u8 = 11;
pub const SPRITE_DOLL_2: u8 = 12;
pub const SPRITE_BIG_DOLL: u8 = 13;
```

### Functions

```rust
/// Draw a 16x16 sprite at screen position (sx, sy).
/// direction and walk_frame control which animation frame to show.
/// Sprites are procedurally generated (colored rectangles with simple details).
pub fn draw_sprite(
    engine: &mut Engine,
    sprite_id: u8,
    sx: i32,
    sy: i32,
    direction: Direction,
    walk_frame: u8,
) { ... }

/// Draw emote bubble above an NPC.
pub fn draw_emote(engine: &mut Engine, emote: Emote, sx: i32, sy: i32) { ... }
```

### Dependencies

- `use super::data::{Direction, Emote};`
- `use crate::engine::Engine;`
- `use crate::rendering::color::Color;`

---

## Phase 8: Main Sim Update (`mod.rs`)

**File**: `pokemonv2/mod.rs`
**Goal**: Wire everything together. PokemonV2Sim struct, GamePhase state machine, Simulation trait impl.

### Module Declarations

```rust
pub mod data;
pub mod maps;
pub mod events;
pub mod overworld;
pub mod render;
pub mod dialogue;
pub mod sprites;
```

### Imports

```rust
use crate::engine::Engine;
use crate::simulation::Simulation;
use crate::rendering::color::Color;
use data::*;
use maps::*;
use events::*;
use overworld::*;
use render::*;
use dialogue::*;
```

### GamePhase Enum

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GamePhase {
    TitleScreen,
    Overworld,
    Dialogue,
    Script,
    StarterSelect { cursor: u8 },
    MapTransition { timer: f64 },
    // Stubs for future sprints:
    Battle,
    Menu,
}
```

### PokemonV2Sim Struct

```rust
pub struct PokemonV2Sim {
    pub phase: GamePhase,
    pub player: PlayerState,
    pub party: Vec<Pokemon>,
    pub current_map_id: MapId,
    pub current_map: MapData,
    pub npc_states: Vec<NpcState>,
    pub bag: Vec<(u8, u8)>,          // (item_id, count)
    pub event_flags: EventFlags,         // [u64; 32] bitfield (2048 flags)
    pub scene_state: SceneState,         // per-map scene IDs (Vec<u8>)
    pub temp_flags: u8,                  // 8 temporary flags, reset on map transition
    pub dialogue: Option<DialogueState>,
    pub script: Option<ScriptState>,
    pub camera: CameraState,
    pub frame_count: u64,
    pub total_time: f64,
    pub step_count: u32,
    pub money: u32,
    pub badges: u8,
    pub title_timer: f64,
    pub player_name: String,             // 7 chars max, default "GOLD"
    pub day_night_tint: f64,             // stub for future day/night cycle
    pub time_of_day: f64,               // stub
    // Map transition state
    pub pending_warp: Option<(MapId, u8)>,
}
```

### Constructor

```rust
impl PokemonV2Sim {
    pub fn new() -> Self {
        let start_map_id = MapId::PlayersHouse2F;
        let start_map = load_map(start_map_id);
        let npc_states = init_npc_states(&start_map);
        Self {
            phase: GamePhase::TitleScreen,
            player: PlayerState {
                x: 3, y: 3,   // Player's room spawn point (pokecrystal)
                facing: Direction::Down,
                walk_offset: 0.0,
                is_walking: false,
                walk_frame: 0,
                frame_timer: 0.0,
                name: "GOLD".to_string(),
            },
            party: Vec::new(),
            current_map_id: start_map_id,
            current_map: start_map,
            npc_states,
            bag: Vec::new(),
            event_flags: EventFlags::new(),
            scene_state: SceneState::new(),
            temp_flags: 0,
            dialogue: None,
            script: None,
            camera: CameraState { x: 3.0 * 16.0, y: 3.0 * 16.0 },
            frame_count: 0,
            total_time: 0.0,
            step_count: 0,
            money: 3000,
            badges: 0,
            title_timer: 0.0,
            player_name: "GOLD".to_string(),
            day_night_tint: 0.0,
            time_of_day: 0.0,
            pending_warp: None,
        }
    }
}
```

### Simulation Trait Implementation

```rust
impl Simulation for PokemonV2Sim {
    fn setup(&mut self, _engine: &mut Engine) {
        // Reset to initial state
        *self = Self::new();
    }

    fn step(&mut self, engine: &mut Engine) {
        self.frame_count += 1;
        self.total_time += 1.0 / 60.0;

        match self.phase {
            GamePhase::TitleScreen => self.step_title(engine),
            GamePhase::Overworld => self.step_overworld_phase(engine),
            GamePhase::Dialogue => self.step_dialogue(engine),
            GamePhase::Script => self.step_script_phase(engine),
            GamePhase::StarterSelect { cursor } => self.step_starter_select(cursor, engine),
            GamePhase::MapTransition { timer } => self.step_map_transition(timer, engine),
            GamePhase::Battle | GamePhase::Menu => {} // stubs
        }
    }

    fn render(&self, engine: &mut Engine) {
        render_game(self, engine);
    }
}
```

### Phase Step Methods

```rust
impl PokemonV2Sim {
    /// Title screen: pulse animation, confirm to start.
    fn step_title(&mut self, engine: &Engine) {
        self.title_timer += 1.0 / 60.0;
        if is_confirm(engine) {
            // Transition to overworld — player starts in bedroom
            self.phase = GamePhase::Overworld;
        }
    }

    /// Overworld phase: delegate to step_overworld(), handle results.
    fn step_overworld_phase(&mut self, engine: &Engine) {
        let result = step_overworld(
            &mut self.player,
            &mut self.camera,
            &self.current_map,
            &mut self.npc_states,
            &self.event_flags,
            &self.scene_state,
            engine,
        );
        match result {
            OverworldResult::Nothing => {}
            OverworldResult::WarpTo { dest_map, dest_warp_id } => {
                self.pending_warp = Some((dest_map, dest_warp_id));
                self.phase = GamePhase::MapTransition { timer: 0.0 };
            }
            // Bilbo Rev 3: resolve script_id to steps via registry
            OverworldResult::TriggerScript { script_id, .. }
            | OverworldResult::TriggerCoordEvent { script_id } => {
                let steps = get_script(script_id); // from events.rs script registry
                self.script = Some(ScriptState::new(steps));
                self.phase = GamePhase::Script;
            }
        }
    }

    /// Script phase: drive the script engine one frame.
    fn step_script_phase(&mut self, engine: &Engine) {
        if let Some(ref mut script) = self.script {
            let still_running = step_script(
                script,
                &mut self.player,
                &mut self.npc_states,
                &mut self.event_flags,
                &mut self.scene_state,
                self.current_map_id,
                &mut self.party,
                &mut self.bag,
                is_confirm(engine),
                is_cancel(engine),
                is_up(engine),
                is_down(engine),
            );
            if !still_running {
                self.script = None;
                self.phase = GamePhase::Overworld;
            }
        }
    }

    /// Dialogue phase: drive standalone dialogue.
    fn step_dialogue(&mut self, engine: &Engine) {
        if let Some(ref mut dlg) = self.dialogue {
            let still_active = dlg.step(is_confirm(engine));
            if !still_active {
                self.dialogue = None;
                self.phase = GamePhase::Overworld;
            }
        }
    }

    /// Map transition: fade out, change map, fade in.
    fn step_map_transition(&mut self, timer: f64, _engine: &Engine) {
        let new_timer = timer + 1.0 / 60.0;
        if new_timer >= 0.5 && timer < 0.5 {
            // Midpoint: actually change the map
            if let Some((dest_map, dest_warp_id)) = self.pending_warp.take() {
                self.change_map(dest_map, dest_warp_id);
            }
        }
        if new_timer >= 1.0 {
            self.phase = GamePhase::Overworld;
            // Check for map-entry scripts (like Elm walk-in cutscene)
            self.check_map_entry_scripts();
        } else {
            self.phase = GamePhase::MapTransition { timer: new_timer };
        }
    }

    /// Starter selection screen.
    fn step_starter_select(&mut self, cursor: u8, engine: &Engine) {
        let mut c = cursor;
        if is_left(engine) && c > 0 { c -= 1; }
        if is_right(engine) && c < 2 { c += 1; }
        if is_confirm(engine) {
            // Give starter Pokemon
            let species = match c {
                0 => CYNDAQUIL,
                1 => TOTODILE,
                _ => CHIKORITA,
            };
            let mut starter = Pokemon::new(species, 5);
            starter.held_item = Some(ITEM_BERRY);
            self.party.push(starter);

            // Set appropriate flags
            self.event_flags.set(EVENT_GOT_A_POKEMON_FROM_ELM);
            match c {
                0 => {
                    self.event_flags.set(EVENT_GOT_CYNDAQUIL_FROM_ELM);
                    self.event_flags.set(EVENT_CYNDAQUIL_POKEBALL_IN_ELMS_LAB);
                }
                1 => {
                    self.event_flags.set(EVENT_GOT_TOTODILE_FROM_ELM);
                    self.event_flags.set(EVENT_TOTODILE_POKEBALL_IN_ELMS_LAB);
                }
                _ => {
                    self.event_flags.set(EVENT_GOT_CHIKORITA_FROM_ELM);
                    self.event_flags.set(EVENT_CHIKORITA_POKEBALL_IN_ELMS_LAB);
                }
            }

            // Update NPC visibility (hide chosen pokeball)
            self.refresh_npc_visibility();

            // Run post-starter script (Elm directions)
            let post_script = build_post_starter_script(c);
            self.script = Some(ScriptState::new(post_script));
            self.phase = GamePhase::Script;
        }
        if is_cancel(engine) {
            // Return to overworld (cancel starter selection)
            self.phase = GamePhase::Overworld;
        }
        if matches!(self.phase, GamePhase::StarterSelect { .. }) { // Bilbo Rev 3: fix syntax
            self.phase = GamePhase::StarterSelect { cursor: c };
        }
    }

    /// Change to a new map, positioning player at the target warp.
    fn change_map(&mut self, dest_map: MapId, dest_warp_id: u8) {
        self.current_map_id = dest_map;
        self.current_map = load_map(dest_map);
        self.npc_states = init_npc_states(&self.current_map);
        self.temp_flags = 0;  // reset temporary flags on every map transition

        // Position player at destination warp
        let (wx, wy) = resolve_warp_position(&self.current_map, dest_warp_id);
        self.player.x = wx;
        self.player.y = wy;
        self.player.is_walking = false;
        self.player.walk_offset = 0.0;

        // Snap camera
        snap_camera(&mut self.camera, &self.player);

        // Apply NPC visibility based on current flags
        self.refresh_npc_visibility();
    }

    /// Check for map-entry scripts (like Elm's lab intro cutscene).
    fn check_map_entry_scripts(&mut self) {
        match self.current_map_id {
            MapId::ElmsLab => {
                let scene = self.scene_state.get(self.current_map_id);
                if scene == SCENE_ELMSLAB_MEET_ELM {
                    // First entry: Elm walk-in cutscene
                    // Reposition Elm to (3, 4) before cutscene starts
                    if let Some(elm) = self.npc_states.get_mut(0) {
                        elm.x = 3;
                        elm.y = 4;
                    }
                    let steps = build_elm_intro_script();
                    self.script = Some(ScriptState::new(steps));
                    self.phase = GamePhase::Script;
                }
            }
            _ => {}
        }
    }

    /// Refresh NPC visibility based on current event flags.
    fn refresh_npc_visibility(&mut self) {
        for (i, npc_def) in self.current_map.npcs.iter().enumerate() {
            if let Some(flag) = npc_def.event_flag {
                let flag_set = self.event_flags.has(flag);
                if let Some(state) = self.npc_states.get_mut(i) {
                    state.visible = if npc_def.event_flag_show { flag_set } else { !flag_set };
                }
            }
        }
    }
}
```

### Input Helpers (in mod.rs)

```rust
fn is_confirm(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("KeyZ")
        || engine.input.keys_pressed.contains("Space")
        || engine.input.keys_pressed.contains("Enter")
}
fn is_cancel(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("KeyX")
        || engine.input.keys_pressed.contains("Escape")
}
fn is_up(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("ArrowUp")
        || engine.input.keys_pressed.contains("KeyW")
}
fn is_down(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("ArrowDown")
        || engine.input.keys_pressed.contains("KeyS")
}
fn is_left(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("ArrowLeft")
        || engine.input.keys_pressed.contains("KeyA")
}
fn is_right(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("ArrowRight")
        || engine.input.keys_pressed.contains("KeyD")
}
fn held_up(engine: &Engine) -> bool {
    engine.input.keys_held.contains("ArrowUp")
        || engine.input.keys_held.contains("KeyW")
}
fn held_down(engine: &Engine) -> bool {
    engine.input.keys_held.contains("ArrowDown")
        || engine.input.keys_held.contains("KeyS")
}
fn held_left(engine: &Engine) -> bool {
    engine.input.keys_held.contains("ArrowLeft")
        || engine.input.keys_held.contains("KeyA")
}
fn held_right(engine: &Engine) -> bool {
    engine.input.keys_held.contains("ArrowRight")
        || engine.input.keys_held.contains("KeyD")
}
```

### Script Builder Functions

```rust
/// Build the Elm's Lab intro cutscene script.
fn build_elm_intro_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::MovePlayer { steps: vec![(Direction::Up, 7)] },
        ScriptStep::TurnPlayer(Direction::Left),
        ScriptStep::ShowEmote { npc_idx: 0, emote: Emote::Shock, frames: 15 },
        ScriptStep::TurnNpc { npc_idx: 0, direction: Direction::Right },
        ScriptStep::ShowText("ELM: <PLAYER>! There you are!".to_string()),
        ScriptStep::ShowText("ELM: I needed to ask you a favor.".to_string()),
        ScriptStep::ShowText("ELM: You see, a POKeMON acquaintance, MR.POKeMON, has a discovery.".to_string()),
        ScriptStep::ShowText("ELM: Will you go see what it's about?".to_string()),
        // Email notification
        ScriptStep::PlaySound(1), // SFX_GLASS_TING
        ScriptStep::Pause(0.5),
        ScriptStep::ShowEmote { npc_idx: 0, emote: Emote::Shock, frames: 10 },
        ScriptStep::TurnNpc { npc_idx: 0, direction: Direction::Down },
        ScriptStep::ShowText("ELM: Hm? I have an email!".to_string()),
        ScriptStep::ShowText("ELM: ...It's from MR.POKeMON!".to_string()),
        ScriptStep::ShowText("ELM: Go ahead--pick a POKeMON!".to_string()),
        // Elm returns to position
        ScriptStep::MoveNpc { npc_idx: 0, steps: vec![(Direction::Up, 1)] },
        ScriptStep::MoveNpc { npc_idx: 0, steps: vec![(Direction::Right, 2), (Direction::Up, 1)] },
        ScriptStep::TurnNpc { npc_idx: 0, direction: Direction::Down },
        ScriptStep::TurnPlayer(Direction::Up),
        ScriptStep::TurnPlayer(Direction::Right),
        ScriptStep::SetScene { map: MapId::ElmsLab, scene_id: SCENE_ELMSLAB_CANT_LEAVE },
        ScriptStep::End,
    ]
}

/// Build the post-starter-selection script (Elm gives directions).
fn build_post_starter_script(choice: u8) -> Vec<ScriptStep> {
    // Movement varies by choice:
    // Cyndaquil (0): Left 1, Up 1
    // Totodile (1): Left 2, Up 1
    // Chikorita (2): Left 3, Up 1
    let left_steps = (choice + 1) as u8;
    vec![
        ScriptStep::MovePlayer { steps: vec![(Direction::Left, left_steps), (Direction::Up, 1)] },
        ScriptStep::TurnPlayer(Direction::Up),
        ScriptStep::ShowText("ELM: I knew you'd pick that one!".to_string()),
        ScriptStep::ShowText("ELM: You can use that healing machine any time.".to_string()),
        ScriptStep::ShowText("ELM: Now, head to MR.POKeMON's place on Route 30!".to_string()),
        ScriptStep::SetScene { map: MapId::ElmsLab, scene_id: SCENE_ELMSLAB_NOOP },
        ScriptStep::End,
    ]
}
```

### Test Infrastructure Helper

```rust
impl PokemonV2Sim {
    /// Create a sim in a specific state for testing (skips title screen).
    #[cfg(test)]
    pub fn with_state(map: MapId, x: i32, y: i32, party: Vec<Pokemon>) -> Self {
        let mut sim = Self::new();
        sim.phase = GamePhase::Overworld;
        sim.party = party;
        sim.current_map_id = map;
        sim.current_map = load_map(map);
        sim.npc_states = init_npc_states(&sim.current_map);
        sim.player.x = x;
        sim.player.y = y;
        snap_camera(&mut sim.camera, &sim.player);
        sim.refresh_npc_visibility();
        sim
    }
}
```

### Dependencies

All submodules.

### Compile Check

After Phase 8: `cargo check` and `cargo test` both pass. The full Simulation trait is wired up.

---

## Phase 9: Tests

**File**: `pokemonv2/mod.rs` (test module at bottom)
**Goal**: Headless simulation tests verifying spawn, walk, warp, events, scripts.

### Test Helper Functions

```rust
#[cfg(test)]
mod tests {
    use super::*;
    // Bilbo Rev 3: removed unused HeadlessRunner import
    use crate::input_frame::InputFrame;

    fn press(key: &str) -> InputFrame {
        InputFrame {
            keys_pressed: vec![key.to_string()],
            ..Default::default()
        }
    }

    fn empty() -> InputFrame {
        InputFrame::default()
    }

    fn hold(key: &str, n: usize) -> Vec<InputFrame> {
        (0..n).map(|_| InputFrame {
            keys_held: vec![key.to_string()],
            ..Default::default()
        }).collect()
    }

    fn walk_dir(dir: &str, gap: usize) -> Vec<InputFrame> {
        let arrow = match dir {
            "up" => "ArrowUp", "down" => "ArrowDown",
            "left" => "ArrowLeft", _ => "ArrowRight",
        };
        let mut frames = vec![press(arrow)];
        frames.extend(vec![empty(); gap]);
        frames
    }
}
```

### Test Cases

```rust
#[test]
fn test_pokemonv2_creates() {
    let sim = PokemonV2Sim::new();
    assert_eq!(sim.phase, GamePhase::TitleScreen);
}

#[test]
fn test_title_to_overworld() {
    // Press confirm on title screen -> should transition to Overworld
    let mut sim = PokemonV2Sim::new();
    let mut engine = Engine::new(160, 144);
    sim.setup(&mut engine);
    engine.input.keys_pressed.insert("KeyZ".to_string());
    sim.step(&mut engine);
    assert_eq!(sim.phase, GamePhase::Overworld);
}

#[test]
fn test_player_spawns_in_bedroom() {
    let sim = PokemonV2Sim::with_state(MapId::PlayersHouse2F, 3, 3, vec![]);
    assert_eq!(sim.player.x, 3);
    assert_eq!(sim.player.y, 3);
    assert_eq!(sim.current_map_id, MapId::PlayersHouse2F);
}

#[test]
fn test_all_sprint1_maps_load() {
    // Verify every Sprint 1 map loads without panic
    let maps = [
        MapId::PlayersHouse2F, MapId::PlayersHouse1F,
        MapId::NewBarkTown, MapId::ElmsLab,
        MapId::ElmsHouse, MapId::PlayersNeighborsHouse,
    ];
    for &id in &maps {
        let map = load_map(id);
        assert!(map.width > 0);
        assert!(map.height > 0);
        assert_eq!(map.tiles.len(), (map.width * map.height) as usize);
        assert_eq!(map.collision.len(), (map.width * map.height) as usize);
    }
}

#[test]
fn test_warp_bidirectional_consistency() {
    // Every warp A->B should have a return warp B->A
    let maps = [
        MapId::PlayersHouse2F, MapId::PlayersHouse1F,
        MapId::NewBarkTown, MapId::ElmsLab,
    ];
    for &map_id in &maps {
        let map = load_map(map_id);
        for warp in &map.warps {
            let dest = load_map(warp.dest_map);
            let has_return = dest.warps.iter().any(|w| w.dest_map == map_id);
            assert!(has_return,
                "Warp from {:?} to {:?} has no return warp", map_id, warp.dest_map);
        }
    }
}

#[test]
fn test_walking_changes_position() {
    let mut sim = PokemonV2Sim::with_state(MapId::PlayersHouse2F, 3, 3, vec![]);
    let mut engine = Engine::new(160, 144);
    let start_x = sim.player.x;
    // Walk right
    engine.input.keys_held.insert("ArrowRight".to_string());
    sim.step(&mut engine);
    // Player should start walking
    assert!(sim.player.is_walking || sim.player.x != start_x);
}

#[test]
fn test_collision_blocks_wall() {
    let mut sim = PokemonV2Sim::with_state(MapId::PlayersHouse2F, 3, 3, vec![]);
    let mut engine = Engine::new(160, 144);
    // Try walking up into wall (y=0 perimeter)
    for _ in 0..20 {
        engine.input.keys_held.insert("ArrowUp".to_string());
        sim.step(&mut engine);
        engine.input.keys_held.clear();
        engine.input.keys_pressed.clear();
    }
    // Player should not be at y < 1 (wall)
    assert!(sim.player.y >= 1, "Player walked through wall to y={}", sim.player.y);
}

#[test]
fn test_warp_bedroom_to_1f() {
    // Walk to stairs at (7, 0) in PlayersHouse2F
    let mut sim = PokemonV2Sim::with_state(MapId::PlayersHouse2F, 6, 1, vec![]);
    let mut engine = Engine::new(160, 144);
    // Walk right then up to reach stair warp at (7, 0)
    // After walking onto warp tile, map should change
    engine.input.keys_held.insert("ArrowRight".to_string());
    for _ in 0..4 { sim.step(&mut engine); engine.input.keys_pressed.clear(); }
    engine.input.keys_held.clear();
    engine.input.keys_held.insert("ArrowUp".to_string());
    for _ in 0..4 { sim.step(&mut engine); engine.input.keys_pressed.clear(); }
    // Should have triggered warp or be in transition
    let warped = sim.current_map_id == MapId::PlayersHouse1F
        || matches!(sim.phase, GamePhase::MapTransition { .. });
    assert!(warped, "Player should warp from 2F stairs to 1F");
}

#[test]
fn test_coord_event_teacher_blocks() {
    // In New Bark Town, walking to x=1 with no Pokemon should trigger teacher script
    let mut sim = PokemonV2Sim::with_state(MapId::NewBarkTown, 3, 8, vec![]);
    // Scene should be 0 (teacher blocks)
    assert_eq!(sim.scenes.get(MapId::NewBarkTown as u16), 0);
    // Walk left toward x=1
    let mut engine = Engine::new(160, 144);
    for _ in 0..30 {
        engine.input.keys_held.insert("ArrowLeft".to_string());
        sim.step(&mut engine);
        engine.input.keys_held.clear();
        engine.input.keys_pressed.clear();
        sim.step(&mut engine); // gap frame
    }
    // Should have triggered script phase (teacher stops you)
    let triggered = matches!(sim.phase, GamePhase::Script)
        || sim.player.x > 1; // pushed back by script
    assert!(triggered, "Teacher should block player from leaving without Pokemon");
}

#[test]
fn test_script_engine_basic() {
    // Test that ScriptState processes ShowText -> WaitButton -> End
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

    // Frame 3: should hit End
    let running = step_script(&mut script, &mut player, &mut npc_states,
        &mut flags, &mut scenes, MapId::NewBarkTown,
        &mut party, &mut bag, false, false, false, false);
    assert!(!running, "Script should end after End step");
}

#[test]
fn test_npc_collision() {
    // NPCs should block player movement
    let map = load_map(MapId::NewBarkTown);
    let npc_states = init_npc_states(&map);
    // Teacher is at (6, 8) — verify we find NPC at that position
    let found = npc_at(&npc_states, 6, 8);
    assert!(found.is_some(), "Teacher NPC should be at (6, 8)");
}

#[test]
fn test_elm_lab_has_correct_npcs() {
    let map = load_map(MapId::ElmsLab);
    assert_eq!(map.npcs.len(), 6, "Elm's lab should have 6 NPCs");
    // Pokeballs at (6,3), (7,3), (8,3)
    assert_eq!(map.npcs[2].x, 6); assert_eq!(map.npcs[2].y, 3);
    assert_eq!(map.npcs[3].x, 7); assert_eq!(map.npcs[3].y, 3);
    assert_eq!(map.npcs[4].x, 8); assert_eq!(map.npcs[4].y, 3);
}

#[test]
fn test_starter_pokemon_stats() {
    // Verify Cyndaquil at Lv5 has correct base stats
    let cyndaquil = Pokemon::new(CYNDAQUIL, 5);
    assert_eq!(cyndaquil.level, 5);
    assert_eq!(cyndaquil.species, CYNDAQUIL);
    assert!(cyndaquil.hp > 0);
    // Should know Tackle and Leer at Lv5
    assert!(cyndaquil.moves.iter().any(|m| *m == Some(MOVE_TACKLE)));
    assert!(cyndaquil.moves.iter().any(|m| *m == Some(MOVE_LEER)));
}

#[test]
fn test_event_flags() {
    let mut flags = EventFlags::new();
    assert!(!flags.has(EVENT_GOT_A_POKEMON_FROM_ELM));
    flags.set(EVENT_GOT_A_POKEMON_FROM_ELM);
    assert!(flags.has(EVENT_GOT_A_POKEMON_FROM_ELM));
    flags.clear(EVENT_GOT_A_POKEMON_FROM_ELM);
    assert!(!flags.has(EVENT_GOT_A_POKEMON_FROM_ELM));
}

#[test]
fn test_new_bark_town_dimensions() {
    let map = load_map(MapId::NewBarkTown);
    // 9 blocks x 10 blocks = 18 tiles x 20 tiles (from Gimli's reference)
    assert_eq!(map.width, 18);
    assert_eq!(map.height, 20);
}

#[test]
fn test_elms_lab_coord_events() {
    let map = load_map(MapId::ElmsLab);
    // Should have 8 coord_events total
    assert_eq!(map.coord_events.len(), 8);
    // Cant-leave events at y=6
    let cant_leave: Vec<_> = map.coord_events.iter()
        .filter(|e| e.scene_id == SCENE_ELMSLAB_CANT_LEAVE)
        .collect();
    assert_eq!(cant_leave.len(), 2);
    assert!(cant_leave.iter().all(|e| e.y == 6));
}
```

---

## Implementation Order Summary

| Phase | Files Created/Modified | What Compiles After |
|-------|----------------------|---------------------|
| 1 | `data.rs` | data.rs standalone |
| 2 | `maps.rs` | data.rs + maps.rs (maps imports data + events types) |
| 3 | `events.rs` | data.rs + maps.rs + events.rs |
| 4 | `overworld.rs` | All logic modules |
| 5 | `render.rs` | All modules incl. rendering |
| 6 | `dialogue.rs` | All modules |
| 7 | `sprites.rs` | All modules |
| 8 | `mod.rs` (full rewrite) | Full `cargo check` passes |
| 9 | `mod.rs` (tests section) | `cargo test` passes |

### Circular Dependency Resolution (Bilbo Rev 3 -- FINAL)

**Problem:** maps.rs needs ScriptStep from events.rs; events.rs needs PlayerState/NpcState; overworld.rs needs both.

**Solution adopted:** Move all shared data types (PlayerState, NpcState, CameraState, Emote, Direction) into `data.rs`. Use `script_id: u16` indirection in maps.rs (Aragorn's suggestion) so maps.rs never imports events.rs. Use script_id in OverworldResult too so overworld.rs never imports ScriptStep.

**Final import graph (acyclic, verified):**
```
data.rs       <- LEAF
events.rs     <- data.rs, maps.rs(MapId only)
maps.rs       <- data.rs only
overworld.rs  <- data.rs, maps.rs, events.rs(EventFlags, SceneState)
render.rs     <- data.rs, maps.rs, overworld.rs, events.rs
sprites.rs    <- data.rs
dialogue.rs   <- LEAF
mod.rs        <- all modules
```

### Parallel Implementation Opportunities

These phases can be worked on in parallel by different agents:
- **Group A (Mary)**: Phase 1 (data.rs) + Phase 6 (dialogue.rs) + Phase 7 (sprites.rs) -- no inter-dependencies, all leaf modules
- **Group B (Pippin)**: Phase 2 (maps.rs) + Phase 3 (events.rs) -- maps depends on data only; events depends on data + maps
- **Group C (Sam)**: Phase 4 (overworld.rs) + Phase 5 (render.rs) -- depend on Group A + B
- **Sequential**: Phase 8 (mod.rs wiring) must come after Groups A+B+C. Phase 9 (tests) after Phase 8.

---

## Sprint 1 Scope Boundaries

### In Scope
- Player spawn in bedroom (3,3), walk around, descend stairs
- Mom's house 1F with MeetMom Pokegear cutscene
- New Bark Town exterior with Teacher-blocks-you coord_event
- Elm's Lab with walk-in cutscene, starter selection (3 Pokemon)
- Post-starter: teacher no longer blocks, can walk freely
- Map transitions with fade effect
- Text box rendering and script-driven dialogue
- All event flags from Sprint 1 reference
- Headless tests for all above

### NOT In Scope (stubs only)
- Battle system (no wild encounters, no trainer battles)
- Menu system (bag, Pokemon, save, etc.)
- PC storage
- Day/night cycle (stub the field but don't render tint)
- Route 29 and beyond (only warp targets as MapId entries)
- Elm's aide giving Potion/Pokeballs (future sprint scenes)
- Officer Jenny (future sprint)
- Time-of-day Mom variants (only Mom1 active)
- Decoration toggle system in Player's House 2F
- Radio interaction (stub BG event text only)
- Sound/music (stub PlaySound/PlayMusic as no-ops)
