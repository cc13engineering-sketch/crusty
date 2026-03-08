# pokemonv2 Architecture Proposal

Sprint 1 foundational architecture for the Pokemon Crystal v2 rewrite.

---

## Design Principles

1. **Modular files over monolith.** v1 put everything in `mod.rs` (15K+ lines). v2 splits into focused modules, each under ~1000 lines.
2. **Data-driven maps.** Map definitions live in declarative structs, not imperative code. Every map is a `MapData` with tiles, warps, NPCs, coord_events, and bg_events.
3. **Scene/script system.** Crystal's event scripting (coord_events, scene triggers, NPC interactions) gets a first-class `ScriptStep` enum that drives cutscenes declaratively.
4. **State machine with sub-states.** The top-level `GamePhase` enum handles major modes. Sub-state (e.g. battle phase, dialogue state) is stored alongside, not nested inside the enum.
5. **Separation of concerns.** Rendering reads state but never mutates it. Step logic never touches the framebuffer. Input helpers are pure functions.
6. **Event flags as scalable bitfield array.** Pokecrystal has 1,332 event flags and 87 scene variables. Use `[u64; 32]` (2048 bits) instead of v1's single `u64`. Separate scene state per map, with temporary flags that reset on map transition.
7. **Pokecrystal is truth.** All map layouts, NPC placements, warp coordinates, dialogue text, and event logic reference the `.asm` files in `pokecrystal-master/`.

---

## Module Structure

```
pokemonv2/
  mod.rs          -- PokemonV2Sim, GamePhase, Simulation impl, input helpers
  data.rs         -- Types, species data, move data, items, type chart
  maps.rs         -- MapId, MapData, Tile, Warp, NpcDef, load_map()
  overworld.rs    -- Player movement, camera, NPC wandering, collision, warp logic
  events.rs       -- Event flags, ScriptEngine, ScriptStep, coord_events, scene system
  battle.rs       -- BattleState, BattlePhase, damage calc, turn resolution
  render.rs       -- All rendering: tiles, sprites, UI, text boxes, menus
  dialogue.rs     -- DialogueState, text wrapping, typewriter effect
  sprites.rs      -- Tile/sprite generation, cached pixel data
```

---

## Core Data Structures

### `mod.rs` -- Top-level simulation

```rust
pub struct PokemonV2Sim {
    phase: GamePhase,
    player: PlayerState,
    party: Vec<Pokemon>,
    current_map_id: MapId,
    current_map: MapData,
    bag: Bag,
    event_flags: EventFlags,         // from events.rs -- [u64; 32] bitfield (2048 flags)
    scene_state: SceneState,         // from events.rs -- per-map scene IDs
    temp_flags: u8,                  // 8 temporary flags, reset on map transition
    battle: Option<BattleState>,     // from battle.rs
    dialogue: Option<DialogueState>, // from dialogue.rs
    script: Option<ScriptState>,     // from events.rs -- active cutscene/coord_event
    camera: CameraState,
    frame_count: u64,
    total_time: f64,
    step_count: u32,
    money: u32,
    badges: u8,
    day_night_tint: f64,
    time_of_day: f64,
    player_name: String,             // 7 chars max
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GamePhase {
    TitleScreen,
    Overworld,
    Dialogue,
    Battle,
    Menu,
    StarterSelect { cursor: u8 },
    MapTransition { timer: f64 },
    Script,  // executing a cutscene/coord_event
}
```

Key difference from v1: `Script` is a first-class game phase. When a coord_event or NPC interaction triggers a scripted sequence, the game enters `GamePhase::Script` and the `ScriptEngine` drives execution frame by frame.

### `data.rs` -- Pokemon data

Same proven patterns from v1:
- `PokemonType` enum (17 Gen 2 types)
- `SpeciesId` / `MoveId` as `u16` type aliases
- `StatusCondition` enum
- `GrowthRate` enum
- Species database: `fn species_data(id: SpeciesId) -> SpeciesData`
- Move database: `fn move_data(id: MoveId) -> MoveData`
- Type effectiveness: `fn type_effectiveness(atk: PokemonType, def: PokemonType) -> f64`
- Pokemon instance struct with HP, stats, moves, level, EVs, DVs (Gen 2 term)
- Item constants

### `maps.rs` -- Map system

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MapId {
    NewBarkTown,
    PlayersHouse1F,
    PlayersHouse2F,
    ElmsLab,
    ElmsHouse,
    PlayersNeighborsHouse,
    Route29,
    // ... future maps
}

pub struct MapData {
    pub id: MapId,
    pub name: &'static str,
    pub width: u8,               // u8 sufficient (max ~40 tiles wide)
    pub height: u8,
    pub tiles: &'static [u8],   // visual tile IDs, row-major (static, never changes)
    pub collision: &'static [u8], // collision type per tile (static)
    pub warps: &'static [WarpDef], // warp points (static)
    pub npcs: Vec<NpcDef>,       // Vec because NPCs are filtered by event flags at load
    pub coord_events: Vec<CoordEvent>,
    pub bg_events: Vec<BgEvent>,
    pub wild_encounters: Vec<WildEncounter>,
    pub connections: MapConnections,  // scrolling edge connections (N/S/E/W)
    pub music_id: u8,
}

/// Door-to-door warp (interior transitions, stairs)
pub struct WarpDef {
    pub x: u8,
    pub y: u8,
    pub dest_map: MapId,
    pub dest_warp_id: u8,  // index into destination map's warp array
}

/// Scrolling edge connection (outdoor map-to-map seamless transition)
pub struct MapConnection {
    pub direction: Direction,
    pub dest_map: MapId,
    pub offset: i8,  // tile offset for alignment
}

pub struct MapConnections {
    pub north: Option<MapConnection>,
    pub south: Option<MapConnection>,
    pub east: Option<MapConnection>,
    pub west: Option<MapConnection>,
}

pub struct NpcDef {
    pub x: u8,
    pub y: u8,
    pub sprite_id: u8,
    pub move_type: NpcMoveType,  // Standing(Dir), SpinRandom, WalkAxis(Dir, range)
    pub script_id: u16,          // which script to run on interaction
    pub event_flag: Option<u16>, // hide/show based on event flag ID
    pub palette: u8,
    pub facing: Direction,
}

pub struct CoordEvent {
    pub x: u8,
    pub y: u8,
    pub scene_id: u8,           // only triggers when map scene == this value
    pub script_id: u16,         // which script to run
}

pub struct BgEvent {
    pub x: u8,
    pub y: u8,
    pub kind: BgEventKind,       // Read, FaceUp, FaceDown, IfSet
    pub script_id: u16,
}
```

### Sprint 1 Maps (from pokecrystal)

| Map | Dimensions | Warps | NPCs | Coord Events |
|-----|-----------|-------|------|--------------|
| NewBarkTown | 20x18 | 4 (ElmsLab, PlayersHouse1F, NeighborsHouse, ElmsHouse) | 3 (Teacher, Fisher, Rival) | 2 (Teacher stops you) |
| PlayersHouse2F | 8x6 | 1 (stairs to 1F) | 0 (decorations are objects) | 0 |
| PlayersHouse1F | 10x8 | 3 (door to town x2, stairs to 2F) | 5 (Mom x4 variants, Neighbor) | 2 (MeetMom left/right) |
| ElmsLab | 10x12 | 2 (door to town x2) | 6 (Elm, Aide, 3 Poke Balls, Officer) | 8 (cant leave, meet officer, aide gives potion/balls) |

---

## Event Flag System (`events.rs`)

### EventFlags -- Scalable bitfield array

Pokecrystal has **1,332 event flags** and **87 scene variables**. v1's single `u64` (64 flags max) is fundamentally insufficient. The bitfield array approach maps directly to pokecrystal's `wEventFlags` byte array:

```rust
/// 1,332 flags need ceil(1332/64) = 21 u64s. Use 32 for headroom (2048 bits).
pub struct EventFlags {
    flags: [u64; 32],
}

impl EventFlags {
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

Why bitfield over HashMap:
- **Deterministic**: bitwise ops are perfectly reproducible (engine determinism requirement)
- **Compact**: 256 bytes vs kilobytes for HashMap
- **Fast**: single array index + bitshift vs hash computation
- **Serializable**: raw `[u64; 32]` for save games
- **pokecrystal-compatible**: maps directly to the original's byte-array-with-bit-indexing

Event flag IDs are `u16` constants, added incrementally per sprint:
```rust
const EVENT_GOT_A_POKEMON_FROM_ELM: u16 = 35;
const EVENT_RIVAL_NEW_BARK_TOWN: u16 = 36;
// ... Sprint 1 defines ~20 flags for the New Bark Town area
```

### SceneState -- Per-map scene tracking

Each map with scripted scenes has a `u8` scene ID that controls which coord_events are active:

```rust
pub struct SceneState {
    scenes: Vec<u8>,  // indexed by MapId as usize, 0 = default scene
}

impl SceneState {
    pub fn get(&self, map: MapId) -> u8 { ... }
    pub fn set(&mut self, map: MapId, scene: u8) { ... }
}
```

### Temporary Flags

Pokecrystal has 8 temporary flags (`EVENT_TEMPORARY_UNTIL_MAP_RELOAD_1-8`) that reset when the player transitions to a new map. Stored as a separate `u8` that gets zeroed in the map transition logic.

---

## Script/Cutscene System (`events.rs`)

This is the biggest architectural improvement over v1. Instead of ad-hoc `if` chains in the step function, scripted events are declarative sequences:

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
    PlaySound(SoundId),
    PlayMusic(MusicId),
    Pause(f64),

    // Game state
    SetEvent(u16),    // set event flag by ID
    ClearEvent(u16),  // clear event flag by ID
    SetScene { map: MapId, scene_id: u8 },  // set a map's scene counter
    GiveItem { item_id: u8, count: u8 },
    GivePokemon { species: SpeciesId, level: u8, held_item: u8 },
    HideNpc(u8),
    ShowNpc(u8),

    // Control flow
    CheckEvent { flag: u16, jump_if_true: usize },
    YesNo { yes_jump: usize, no_jump: usize },
    Jump(usize),  // unconditional jump to step index
    End,

    // Transitions
    FacingPlayer { npc_idx: u8 },  // NPC turns to face the player
    Heal,  // heal party
}

pub struct ScriptState {
    pub steps: Vec<ScriptStep>,
    pub pc: usize,          // program counter (current step index)
    pub timer: f64,         // for timed steps (Pause, text advancement)
    pub waiting: bool,      // waiting for player input
    pub text_buffer: Option<String>, // currently displayed text
}
```

The `ScriptEngine` processes one step per frame (or waits when blocking). This naturally handles Crystal's scripting model: `writetext` -> `waitbutton` -> `closetext` -> `applymovement` -> etc.

### Scene System

Each map has a `scene_id: u8` that tracks progression state. Coord_events only fire when the map's current scene matches their `scene_id`. This is exactly how pokecrystal works:

- ElmsLab has scenes: `MEET_ELM (0)`, `CANT_LEAVE (1)`, `NOOP (2)`, `MEET_OFFICER (3)`, `UNUSED (4)`, `AIDE_GIVES_POTION (5)`, `AIDE_GIVES_POKE_BALLS (6)`
- NewBarkTown has scenes: `TEACHER_STOPS_YOU (0)`, `NOOP (1)`

When a script calls `SetScene(n)`, the map's active scene changes, enabling/disabling different coord_events.

---

## Overworld System (`overworld.rs`)

Handles per-frame player movement, collision, warping, NPC wandering, and encounter checks.

```rust
pub struct PlayerState {
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
    pub walk_offset: f64,
    pub is_walking: bool,
    pub walk_frame: u8,
    pub frame_timer: f64,
}

pub struct CameraState {
    pub x: f64,
    pub y: f64,
}

pub fn step_overworld(sim: &mut PokemonV2Sim, engine: &Engine) {
    // 1. Process held direction input -> start walking
    // 2. Animate walk (interpolate walk_offset)
    // 3. On walk complete: check coord_events, check warps, check encounters
    // 4. Update camera (smooth lerp toward player)
    // 5. Tick NPC wander timers
}
```

Key design: `step_overworld` is a standalone function, not a method buried in a 2000-line `match` block. It takes `&mut PokemonV2Sim` and `&Engine` and handles all overworld logic.

---

## Battle System (`battle.rs`)

Preserves v1's proven queue-based sequencer pattern:

```rust
pub struct BattleState {
    pub phase: BattlePhase,
    pub enemy: Pokemon,
    pub player_idx: usize,
    pub is_wild: bool,
    pub trainer_name: String,
    pub queue: VecDeque<BattleStep>,
    pub queue_timer: f64,
    pub player_stages: [i8; 7],
    pub enemy_stages: [i8; 7],
    // ... additional battle fields
}
```

Not needed for Sprint 1, but the skeleton is here so the architecture is complete. Battle implementation comes in later sprints.

---

## Rendering (`render.rs`)

All rendering is read-only on game state. Separated into clear sub-functions:

```rust
pub fn render_game(sim: &PokemonV2Sim, engine: &mut Engine) {
    match sim.phase {
        GamePhase::TitleScreen => render_title(sim, engine),
        GamePhase::Overworld | GamePhase::Script => render_overworld(sim, engine),
        GamePhase::Dialogue => render_dialogue(sim, engine),
        GamePhase::Battle => render_battle(sim, engine),
        GamePhase::Menu => render_menu(sim, engine),
        GamePhase::StarterSelect { cursor } => render_starter_select(cursor, engine),
        GamePhase::MapTransition { timer } => render_transition(timer, engine),
    }
}

fn render_overworld(sim: &PokemonV2Sim, engine: &mut Engine) {
    // 1. Clear framebuffer
    // 2. Draw tile grid (camera-relative)
    // 3. Draw NPCs
    // 4. Draw player
    // 5. Draw text box if script has active text
    // 6. Apply day/night tint
}
```

Tile rendering uses the same 16px tile size as v1 (`TILE_PX = 16`), with a 10x9 viewport.

---

## Input Helpers

Same proven pattern from v1 -- pure functions reading `engine.input`:

```rust
fn is_confirm(engine: &Engine) -> bool { ... }
fn is_cancel(engine: &Engine) -> bool { ... }
fn is_up(engine: &Engine) -> bool { ... }
// etc.
```

These live in `mod.rs` since they're used everywhere.

---

## Dialogue System (`dialogue.rs`)

```rust
pub struct DialogueState {
    pub lines: Vec<String>,
    pub current_line: usize,
    pub char_index: usize,
    pub timer: f64,
    pub on_complete: DialogueAction,
}
```

Handles standalone dialogues (NPC talk, sign reading). For scripted sequences, the `ScriptEngine` manages text display directly via `ScriptStep::ShowText`.

---

## Sprint 1 Implementation Flow

1. **Phase 1: Core infrastructure** -- Set up module files, data types, MapData structs, input helpers
2. **Phase 2: Map data** -- Encode all 4 maps from pokecrystal (NewBarkTown, PlayersHouse1F/2F, ElmsLab) with tiles, warps, NPCs, events
3. **Phase 3: Overworld** -- Player movement, collision, camera, warp transitions, NPC rendering
4. **Phase 4: Script engine** -- ScriptStep processing, text display, NPC movement commands
5. **Phase 5: New Bark Town events** -- Teacher-stops-you coord_event, MeetMom scene, Elm intro + starter selection
6. **Phase 6: Tests** -- Headless simulation tests for spawning, walking, warping, and event triggers

---

## Key Differences from v1

| Aspect | v1 | v2 |
|--------|----|----|
| File structure | 1 monolithic mod.rs (15K lines) | 8 focused modules (~500-1500 lines each) |
| Event flags | Single u64 (64 flags max) | `[u64; 32]` bitfield array (2048 flags, matches pokecrystal) |
| Scene state | Implicit via flags | Explicit `SceneState` per map + temporary flags that reset on transition |
| Event scripting | Ad-hoc if/match chains in step() | Declarative ScriptStep sequences with a program counter |
| Map scenes | Implicit via flags | Explicit scene_id per map, matching pokecrystal's scene system |
| Map data | All heap-allocated Vec | Static slices (`&'static [u8]`) for tiles/collision/warps |
| Map connections | Not supported (warp-only) | Warps (doors) + MapConnections (scrolling edges) as separate systems |
| Coord events | Not supported | First-class CoordEvent matching pokecrystal exactly |
| BG events | Hardcoded sign checks | Declarative BgEvent list per map |
| NPC visibility | Hardcoded is_npc_active() | Event flag-based filtering via `NpcDef.event_flag: Option<u16>` |
| Game phase | 20+ variants with data | Clean 8-variant enum; sub-state in separate structs |

---

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    // 1. Verify all maps load without panic
    // 2. Verify warp consistency (A->B and B->A)
    // 3. Verify player spawns at correct position
    // 4. Verify walking into warp triggers map transition
    // 5. Verify coord_event fires when stepping on trigger tile
    // 6. Verify script engine processes ShowText -> WaitButton -> End
    // 7. Verify starter selection gives Pokemon and sets flags
    // 8. Verify NPC collision blocks player movement
}
```

Tests use a headless `Engine` (no rendering) to drive the simulation via `step()` calls with synthetic input.

---

## Constants

```rust
const TILE_PX: i32 = 16;
const VIEW_TILES_X: i32 = 10;
const VIEW_TILES_Y: i32 = 9;
const WALK_SPEED: f64 = 8.0;  // pixels per frame at 60fps
const CAMERA_LERP: f64 = 0.2;
const TEXT_MAX_CHARS: usize = 24;
```

Same as v1 -- these values produce correct Game Boy-style rendering.
