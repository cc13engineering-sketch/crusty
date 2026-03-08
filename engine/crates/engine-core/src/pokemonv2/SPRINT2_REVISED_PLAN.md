# Sprint 2 Revised Implementation Plan

> Revised by Bilbo (Senior Sprint Engineer)
> Incorporates all modifications from BILBO_REVIEW_S2.md + Gimli's Pokemon accuracy fixes
> Base plan by Frodo (Technical Writer) from SPRINT2_IMPLEMENTATION_PLAN.md
> APPROVED -- ready for parallel implementation by Mary/Pippin/Sam

---

## Summary of All Modifications from Review

This section lists every change from the original plan. Each is tagged with its review issue number and priority.

| # | Phase | Change | Priority | Status |
|---|-------|--------|----------|--------|
| 1 | 2 | Migrate `wild_encounters: vec![]` to `None` in all Sprint 1 map builders; remove old `WildEncounter` struct | CRITICAL | Incorporated |
| 2 | 3,6 | Change `step_script()` return type from `bool` to `ScriptResult` enum; add `loaded_wild_species` to ScriptState | CRITICAL | Incorporated |
| 3 | 5,6 | Pass RNG bytes + TimeOfDay as parameters to `step_overworld()` (Option A: caller extracts) | CRITICAL | Incorporated |
| 4 | 1,4 | Move `BattleType` and `BattleResult` to `data.rs` (not events.rs) for clean import graph | HIGH | Incorporated |
| 7 | 2,5 | Put `is_walkable_with_direction` in `maps.rs`, not `overworld.rs` | HIGH | Incorporated |
| 8 | 5 | Remove redundant `as i32` casts on map width/height (already i32) | MEDIUM | Incorporated |
| 9 | 3 | Expand SceneState initial vec from 16 to 32 entries | MEDIUM | Incorporated |
| 10 | 6 | Use existing `Battle` GamePhase variant (already a stub); don't redeclare | MEDIUM | Incorporated |
| 11 | 6 | Add `battle: None` to both `new()` and `with_state()` constructors | MEDIUM | Incorporated |
| 12 | 7 | Fix `draw_text` argument order to match existing signature: `(engine, text, x, y, color)` | MEDIUM | Incorporated |
| 13 | 2 | Add `Route30` to MapId enum with stub map | MEDIUM | Incorporated |
| 14 | 3 | Define music constants (MUSIC_SHOW_ME_AROUND, etc.) in data.rs | MEDIUM | Incorporated |
| 17 | 1,4 | Move MOVE_STRUGGLE constant to data.rs (not battle.rs) | LOW | Incorporated |
| 19 | 6 | Inline `find_npc_by_event_flag` as a loop (don't reference undefined method) | LOW | Incorporated |
| G1 | 1 | Sand-Attack type: PokemonType::Ground, not Normal | HIGH | Incorporated |
| G2 | 1 | Fix all item IDs to match pokecrystal (POKE_BALL=5, ANTIDOTE=9, AWAKENING=12, PARLYZ_HEAL=13, MYSTIC_WATER=95, PINK_BOW=104) | HIGH | Incorporated |
| G3 | 1,3 | MAP_CARD is an engine flag only, not a bag item. Remove ITEM_MAP_CARD constant and GiveItem from guide gent script | HIGH | Incorporated |

---

## Module File Plan (Unchanged)

After Sprint 2, the directory looks like:

```
pokemonv2/
  mod.rs          -- PokemonV2Sim, GamePhase, Simulation impl, input helpers, battle/map-connection dispatch
  data.rs         -- (expanded) 5 new species, 4+ new moves, TimeOfDay, BattleType, BattleResult, new items, music stubs
  maps.rs         -- (expanded) 8+ new maps, C_GRASS/C_LEDGE collision, WildEncounterTable, is_walkable_with_direction
  overworld.rs    -- (expanded) ledge movement, wild encounter checks, map edge -> MapConnection
  events.rs       -- (expanded) 15+ new event flags, 6 new ScriptStep variants, ~20 new scripts, ScriptResult enum
  battle.rs       -- NEW MODULE: BattleState, BattlePhase, auto-battle loop, Gen 2 damage calc
  render.rs       -- (expanded) grass tiles, ledge tiles, battle screen stub
  dialogue.rs     -- (unchanged)
  sprites.rs      -- (expanded) 11 new sprite IDs
```

### Acyclic Import Graph (REVISED -- Review Issue #4)

```
data.rs          <- LEAF (no sibling imports)
                    NOW CONTAINS: BattleType, BattleResult, MOVE_STRUGGLE, music constants
  |
  +-- events.rs  <- imports data.rs, maps::MapId
  +-- maps.rs    <- imports data::{Direction, SpeciesId, NpcState, TimeOfDay}
  +-- battle.rs  <- imports data::{Pokemon, SpeciesId, MoveId, PokemonType, BattleType, BattleResult, species_data, move_data, type_effectiveness, MOVE_STRUGGLE}
  +-- dialogue.rs <- LEAF
  +-- sprites.rs <- imports data::{Direction, Emote}
  |
  +-- overworld.rs <- imports data, maps (including is_walkable_with_direction), events::{EventFlags, SceneState}
  |
  +-- render.rs  <- imports data, maps, overworld::constants, events, sprites, battle
  |
  +-- mod.rs     <- imports everything
```

**Key change from original**: `battle.rs` depends ONLY on `data.rs`. `BattleType` and `BattleResult` live in `data.rs`, not `events.rs`. This eliminates the `battle.rs -> events.rs` dependency that violated the original graph.

---

## Phase 1: Core Data Additions (`data.rs`)

**File**: `pokemonv2/data.rs`
**Goal**: Add 5 new species, 4+ new moves, TimeOfDay enum, BattleType/BattleResult enums, MOVE_STRUGGLE, music constants, new item constants.

### REVIEW CHANGE #4: BattleType and BattleResult in data.rs

Add these enums to data.rs (NOT events.rs). They are pure data with no sibling imports.

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BattleType {
    Wild,          // standard wild encounter
    Tutorial,      // BATTLETYPE_TUTORIAL -- catching demo, auto-catch
    CanLose,       // BATTLETYPE_CANLOSE -- rival, no game-over on loss
    Normal,        // standard trainer battle
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BattleResult {
    Won,
    Lost,
    Fled,
    Caught,
}
```

### TimeOfDay Enum

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TimeOfDay {
    Morning,  // 04:00 - 09:59
    Day,      // 10:00 - 17:59
    Night,    // 18:00 - 03:59
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
```

### New Species Constants

```rust
// Route 29 wild encounters
pub const PIDGEY: SpeciesId = 16;
pub const RATTATA: SpeciesId = 19;
pub const SENTRET: SpeciesId = 161;
pub const HOOTHOOT: SpeciesId = 163;
pub const HOPPIP: SpeciesId = 187;
```

### New Move Constants

```rust
pub const MOVE_TAIL_WHIP: MoveId = 39;
pub const MOVE_DEFENSE_CURL: MoveId = 111;
pub const MOVE_SPLASH: MoveId = 150;
pub const MOVE_SYNTHESIS: MoveId = 235;
pub const MOVE_SAND_ATTACK: MoveId = 28;
```

### REVIEW CHANGE #17: MOVE_STRUGGLE in data.rs

```rust
/// Struggle -- used when a Pokemon has no damaging moves (e.g., Hoppip with only Splash).
pub const MOVE_STRUGGLE: MoveId = 165;
```

### REVIEW CHANGE #14: Music Constants in data.rs

```rust
// Music ID stub constants (no audio system -- values are placeholders)
pub const MUSIC_SHOW_ME_AROUND: u8 = 10;
pub const MUSIC_RIVAL_ENCOUNTER: u8 = 11;
pub const MUSIC_RIVAL_AFTER: u8 = 12;
```

### New Move Data Entries

Add to `move_data()` match arms (use static pattern matching existing Sprint 1 statics):

```rust
MOVE_TAIL_WHIP => &MoveData {
    id: MOVE_TAIL_WHIP, name: "Tail Whip",
    move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 30, is_special: false,
},
MOVE_DEFENSE_CURL => &MoveData {
    id: MOVE_DEFENSE_CURL, name: "Defense Curl",
    move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 40, is_special: false,
},
MOVE_SPLASH => &MoveData {
    id: MOVE_SPLASH, name: "Splash",
    move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 40, is_special: false,
},
MOVE_SYNTHESIS => &MoveData {
    id: MOVE_SYNTHESIS, name: "Synthesis",
    move_type: PokemonType::Grass, power: 0, accuracy: 100, pp: 5, is_special: true,
},
MOVE_SAND_ATTACK => &MoveData {
    id: MOVE_SAND_ATTACK, name: "Sand-Attack",
    move_type: PokemonType::Ground, power: 0, accuracy: 100, pp: 15, is_special: false,
    // GIMLI FIX: Sand-Attack is Ground type, not Normal
},
MOVE_STRUGGLE => &MoveData {
    id: MOVE_STRUGGLE, name: "Struggle",
    move_type: PokemonType::Normal, power: 50, accuracy: 100, pp: 1, is_special: false,
},
```

### New Species Data Entries

All data sourced from pokecrystal `data/pokemon/base_stats/*.asm` and `data/pokemon/evos_attacks.asm`.

Add to `species_data()` match:

```rust
PIDGEY => &SpeciesData {
    id: PIDGEY, name: "Pidgey",
    type1: PokemonType::Normal, type2: PokemonType::Flying,
    base_hp: 40, base_attack: 45, base_defense: 40,
    base_speed: 56, base_sp_attack: 35, base_sp_defense: 35,
    catch_rate: 255, base_exp: 55,
    growth_rate: GrowthRate::MediumSlow,
    learnset: &[
        (1, MOVE_TACKLE),
        (5, MOVE_SAND_ATTACK),
    ],
},
RATTATA => &SpeciesData {
    id: RATTATA, name: "Rattata",
    type1: PokemonType::Normal, type2: PokemonType::Normal,
    base_hp: 30, base_attack: 56, base_defense: 35,
    base_speed: 72, base_sp_attack: 25, base_sp_defense: 35,
    catch_rate: 255, base_exp: 57,
    growth_rate: GrowthRate::MediumFast,
    learnset: &[
        (1, MOVE_TACKLE),
        (1, MOVE_TAIL_WHIP),
    ],
},
SENTRET => &SpeciesData {
    id: SENTRET, name: "Sentret",
    type1: PokemonType::Normal, type2: PokemonType::Normal,
    base_hp: 35, base_attack: 46, base_defense: 34,
    base_speed: 20, base_sp_attack: 35, base_sp_defense: 45,
    catch_rate: 255, base_exp: 57,
    growth_rate: GrowthRate::MediumFast,
    learnset: &[
        (1, MOVE_TACKLE),
        (5, MOVE_DEFENSE_CURL),
    ],
},
HOOTHOOT => &SpeciesData {
    id: HOOTHOOT, name: "Hoothoot",
    type1: PokemonType::Normal, type2: PokemonType::Flying,
    base_hp: 60, base_attack: 30, base_defense: 30,
    base_speed: 50, base_sp_attack: 36, base_sp_defense: 56,
    catch_rate: 255, base_exp: 58,
    growth_rate: GrowthRate::MediumFast,
    learnset: &[
        (1, MOVE_TACKLE),
        (1, MOVE_GROWL),
    ],
},
HOPPIP => &SpeciesData {
    id: HOPPIP, name: "Hoppip",
    type1: PokemonType::Grass, type2: PokemonType::Flying,
    base_hp: 35, base_attack: 35, base_defense: 40,
    base_speed: 50, base_sp_attack: 35, base_sp_defense: 55,
    catch_rate: 255, base_exp: 74,
    growth_rate: GrowthRate::MediumSlow,
    learnset: &[
        (1, MOVE_SPLASH),
        (5, MOVE_SYNTHESIS),
        (5, MOVE_TAIL_WHIP),
        (10, MOVE_TACKLE),
    ],
},
```

**Note on Hoppip**: At levels 2-3 (Route 29 encounter levels), Hoppip knows only Splash (power 0). Battle system must use Struggle. See Phase 4.

### New Item Constants

```rust
// GIMLI FIX: All item IDs corrected to match pokecrystal constants
pub const ITEM_POKE_BALL: u8 = 5;
pub const ITEM_ANTIDOTE: u8 = 9;
pub const ITEM_AWAKENING: u8 = 12;
pub const ITEM_PARLYZ_HEAL: u8 = 13;
pub const ITEM_MYSTIC_WATER: u8 = 95;
pub const ITEM_PINK_BOW: u8 = 104;
// GIMLI FIX: MAP_CARD is NOT a bag item. It is an engine flag (EVENT_ENGINE_MAP_CARD).
// Remove ITEM_MAP_CARD constant entirely. The guide gent script uses SetEvent only.
```

### Dependencies

None. data.rs remains a leaf module.

### Compile Check

After Phase 1: `cargo check` succeeds. `species_data(PIDGEY)`, `move_data(MOVE_TAIL_WHIP)`, `move_data(MOVE_STRUGGLE)`, `get_time_of_day(500.0)` all return valid data. `BattleType` and `BattleResult` are accessible from data.rs.

---

## Phase 2: Map System Expansion (`maps.rs`)

**File**: `pokemonv2/maps.rs`
**Goal**: Add 8+ new maps, new collision types, WildEncounterTable struct, map connection data.

### REVIEW CHANGE #13: MapId Enum with Route30

```rust
#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub enum MapId {
    // Sprint 1 (existing)
    NewBarkTown, PlayersHouse1F, PlayersHouse2F, ElmsLab,
    ElmsHouse, PlayersNeighborsHouse, Route29, Route27,
    // Sprint 2 (new)
    Route29Route46Gate,
    CherrygroveCity,
    CherrygrovePokecenter1F,
    CherrygroveMart,
    GuideGentsHouse,
    CherrygroveGymSpeechHouse,
    CherrygroveEvolutionSpeechHouse,
    Route46,  // stub -- gate warp target only
    Route30,  // REVIEW #13: stub -- CherrygroveCity north connection target
}
```

### New Collision Constants

```rust
pub const C_GRASS: u8 = 5;    // walkable, triggers wild encounter check on step complete
pub const C_LEDGE_D: u8 = 6;  // one-way south: can jump south only
pub const C_LEDGE_L: u8 = 7;  // one-way left (future use)
pub const C_LEDGE_R: u8 = 8;  // one-way right (future use)
```

### WildEncounterTable Struct

**Replace** the existing `WildEncounter` struct entirely.

```rust
/// A single encounter slot: species at a fixed level.
#[derive(Clone, Debug)]
pub struct WildSlot {
    pub species: SpeciesId,
    pub level: u8,
}

/// Time-of-day encounter table for grass/cave encounters.
/// Each period has exactly 7 slots matching pokecrystal's format.
/// Slot probabilities: [0]=30%, [1]=30%, [2]=20%, [3]=10%, [4]=5%, [5]=2.5%, [6]=2.5%
#[derive(Clone, Debug)]
pub struct WildEncounterTable {
    pub morning: Vec<WildSlot>,
    pub day: Vec<WildSlot>,
    pub night: Vec<WildSlot>,
    pub encounter_rate: u8,
}
```

### REVIEW CHANGE #1 (CRITICAL): MapData field change + Sprint 1 migration

Update `MapData`:

```rust
pub struct MapData {
    // ... existing fields ...
    pub wild_encounters: Option<WildEncounterTable>,  // CHANGED from Vec<WildEncounter>
    // ... rest unchanged ...
}
```

**Remove** the old `WildEncounter` struct entirely (lines 118-123 of current maps.rs):

```rust
// DELETE THIS:
// pub struct WildEncounter {
//     pub species: SpeciesId,
//     pub min_level: u8,
//     pub max_level: u8,
//     pub rate: u8,
// }
```

**Migrate ALL Sprint 1 map builders**: Change `wild_encounters: vec![]` to `wild_encounters: None` in:

- `build_players_house_2f()` -- line 285 of current maps.rs
- `build_players_house_1f()` -- line 344
- `build_new_bark_town()` -- line 433
- `build_elms_lab()` -- line 525
- `build_stub_house()` -- line 553
- `build_stub_route()` -- line 576

**Exact code change in each builder:**
```rust
// BEFORE (in every Sprint 1 map builder):
wild_encounters: vec![],

// AFTER:
wild_encounters: None,
```

### REVIEW CHANGE #7: is_walkable_with_direction in maps.rs

Add this function to `maps.rs` alongside the existing `is_walkable`:

```rust
/// Check if tile at (x, y) is walkable when approached from the given direction.
/// Ledge tiles are only walkable if the player is facing the ledge's direction.
pub fn is_walkable_with_direction(map: &MapData, x: i32, y: i32, facing: Direction) -> bool {
    if x < 0 || y < 0 || x >= map.width || y >= map.height {  // REVIEW #8: no as i32 cast needed
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
```

**Also update existing `is_walkable`** to include C_GRASS:

```rust
pub fn is_walkable(map: &MapData, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 || x >= map.width || y >= map.height {
        return false;
    }
    let idx = (y * map.width + x) as usize;
    let c = map.collision[idx];
    c == C_FLOOR || c == C_WARP || c == C_GRASS  // ADDED C_GRASS
}
```

### Route 29 Wild Encounter Data

From pokecrystal `data/wild/johto_grass.asm`:

```rust
fn build_route29_encounters() -> WildEncounterTable {
    WildEncounterTable {
        encounter_rate: 10,
        morning: vec![
            WildSlot { species: PIDGEY, level: 2 },
            WildSlot { species: SENTRET, level: 2 },
            WildSlot { species: PIDGEY, level: 3 },
            WildSlot { species: SENTRET, level: 3 },
            WildSlot { species: RATTATA, level: 2 },
            WildSlot { species: HOPPIP, level: 3 },
            WildSlot { species: HOPPIP, level: 3 },
        ],
        day: vec![
            WildSlot { species: PIDGEY, level: 2 },
            WildSlot { species: SENTRET, level: 2 },
            WildSlot { species: PIDGEY, level: 3 },
            WildSlot { species: SENTRET, level: 3 },
            WildSlot { species: RATTATA, level: 2 },
            WildSlot { species: HOPPIP, level: 3 },
            WildSlot { species: HOPPIP, level: 3 },
        ],
        night: vec![
            WildSlot { species: HOOTHOOT, level: 2 },
            WildSlot { species: RATTATA, level: 2 },
            WildSlot { species: HOOTHOOT, level: 3 },
            WildSlot { species: RATTATA, level: 3 },
            WildSlot { species: RATTATA, level: 2 },
            WildSlot { species: HOOTHOOT, level: 3 },
            WildSlot { species: HOOTHOOT, level: 3 },
        ],
    }
}
```

### Map Data for All Sprint 2 Maps

All map data specifications are identical to the original plan. See SPRINT2_IMPLEMENTATION_PLAN.md Phase 2 for full details on:

- Route 29 (60x18 tiles) -- replaces Sprint 1 stub entirely
- Route29Route46Gate (10x8 tiles) -- REVIEW #6: 5x4 blocks, NOT 4x4 as architecture doc says
- CherrygroveCity (40x18 tiles)
- CherrygrovePokecenter1F (10x8 tiles)
- CherrygroveMart (12x8 tiles)
- GuideGentsHouse (8x8 tiles)
- CherrygroveGymSpeechHouse (8x8 tiles)
- CherrygroveEvolutionSpeechHouse (8x8 tiles)
- Route46 stub
- Route30 stub (REVIEW #13: new)

### Route30 Stub Map

```rust
MapId::Route30 => {
    let (w, h) = (20i32, 20i32);
    let tiles = vec![0u8; (w * h) as usize];
    let col = vec![C_FLOOR; (w * h) as usize];
    MapData {
        id: MapId::Route30,
        name: "ROUTE 30",
        width: w,
        height: h,
        tiles,
        collision: col,
        warps: vec![],
        npcs: vec![],
        coord_events: vec![],
        bg_events: vec![],
        wild_encounters: None,  // REVIEW #1: None, not vec![]
        connections: MapConnections {
            north: None,
            south: Some(MapConnection { direction: Direction::Down, dest_map: MapId::CherrygroveCity, offset: -5 }),
            east: None,
            west: None,
        },
        music_id: 0,
    }
}
```

### Warp Bidirectionality & Map Connection Data

Identical to original plan. See SPRINT2_IMPLEMENTATION_PLAN.md Phase 2.

### imports update

```rust
use super::data::{Direction, SpeciesId, NpcState, TimeOfDay};
```

### Compile Check

After Phase 2: All maps load. `wild_encounters: None` compiles for Sprint 1 maps. `wild_encounters: Some(build_route29_encounters())` compiles for Route 29. `is_walkable_with_direction` compiles in maps.rs.

---

## Phase 3: Event System Expansion (`events.rs`)

**File**: `pokemonv2/events.rs`
**Goal**: New event flags, scene constants, ScriptStep variants, ScriptResult enum, ~20 new scripts.

### REVIEW CHANGE #9: SceneState Initial Size

In `SceneState::new()`:

```rust
// BEFORE:
Self { scenes: vec![0u8; 16] }

// AFTER:
Self { scenes: vec![0u8; 32] }
```

### REVIEW CHANGE #2 (CRITICAL): ScriptResult Enum

Add this enum to events.rs. This replaces the `bool` return type of `step_script()`.

```rust
/// Result of a script step execution.
/// Replaces the old bool return (true=running, false=ended).
#[derive(Clone, Debug)]
pub enum ScriptResult {
    /// Script is still executing
    Running,
    /// Script hit End step
    Ended,
    /// Script wants to start a battle -- mod.rs must create BattleState and switch phase
    StartBattle {
        battle_type: BattleType,
        species: Option<(SpeciesId, u8)>,  // from LoadWildMon, or None for rival (mod.rs determines)
    },
}
```

**Import required**: `use super::data::BattleType;` (BattleType lives in data.rs per Review #4)

### REVIEW CHANGE #2 (CRITICAL): loaded_wild_species on ScriptState

Add field to `ScriptState`:

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
    pub move_queue: Vec<(Direction, u8)>,
    pub move_progress: f64,
    pub moving_npc: Option<u8>,
    pub loaded_wild_species: Option<(SpeciesId, u8)>,  // NEW: for LoadWildMon -> StartBattle handoff
}
```

Update `ScriptState::new()`:
```rust
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
            moving_npc: None,
            loaded_wild_species: None,  // NEW
        }
    }
}
```

### REVIEW CHANGE #2 (CRITICAL): step_script() Returns ScriptResult

Change the function signature:

```rust
// BEFORE:
pub fn step_script(...) -> bool {

// AFTER:
pub fn step_script(...) -> ScriptResult {
```

Change all `return true` to `return ScriptResult::Running`.
Change `return false` and the `ScriptStep::End` branch to `return ScriptResult::Ended`.

The `StartBattle` step returns:
```rust
ScriptStep::StartBattle { battle_type } => {
    let species = script.loaded_wild_species.take();
    script.pc += 1;
    return ScriptResult::StartBattle {
        battle_type: *battle_type,
        species,
    };
}
```

### New ScriptStep Variants

Add to `ScriptStep` enum. Note: `BattleType` is imported from `data.rs` (Review #4).

```rust
pub enum ScriptStep {
    // ... existing Sprint 1 variants ...

    // Sprint 2 additions:
    LoadWildMon { species: SpeciesId, level: u8 },
    StartBattle { battle_type: BattleType },
    Follow { npc_idx: u8 },
    StopFollow,
    MoveObject { npc_idx: u8, x: i32, y: i32 },
    PlayMapMusic,
    Special(SpecialFn),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SpecialFn {
    HealParty,
    RestartMapMusic,
    FadeOutMusic,
}
```

### New ScriptStep Handler Code in step_script()

Add these match arms:

```rust
ScriptStep::LoadWildMon { species, level } => {
    script.loaded_wild_species = Some((*species, *level));
    script.pc += 1;
}
ScriptStep::StartBattle { battle_type } => {
    let species = script.loaded_wild_species.take();
    script.pc += 1;
    return ScriptResult::StartBattle {
        battle_type: *battle_type,
        species,
    };
}
ScriptStep::Follow { .. } => {
    // Sprint 2 no-op: movement handled by MoveNpc/MovePlayer pairs
    script.pc += 1;
}
ScriptStep::StopFollow => {
    script.pc += 1;
}
ScriptStep::MoveObject { npc_idx, x, y } => {
    if let Some(npc) = npc_states.get_mut(*npc_idx as usize) {
        npc.x = *x;
        npc.y = *y;
    }
    script.pc += 1;
}
ScriptStep::PlayMapMusic => {
    // No-op for Sprint 2 (no audio system)
    script.pc += 1;
}
ScriptStep::Special(special_fn) => {
    match special_fn {
        SpecialFn::HealParty => {
            for p in party.iter_mut() {
                p.hp = p.max_hp;
            }
        }
        SpecialFn::RestartMapMusic => { /* no-op Sprint 2 */ }
        SpecialFn::FadeOutMusic => { /* no-op Sprint 2 */ }
    }
    script.pc += 1;
}
```

### New Event Flag Constants, Scene Constants, Script ID Constants, and All Script Implementations

**Identical to the original plan** with the Gimli fix below. See SPRINT2_IMPLEMENTATION_PLAN.md Phase 3 for the full listing of:
- Event flag constants (EVENT_GUIDE_GENT_VISIBLE_IN_CHERRYGROVE through EVENT_ENGINE_ZEPHYRBADGE)
- Scene constants (SCENE_ROUTE29_*, SCENE_CHERRYGROVECITY_*)
- Script ID constants (SCRIPT_ROUTE29_SIGN1 through SCRIPT_EVO_SPEECH_BOOKSHELF)
- All script builder functions

**GIMLI FIX: Guide Gent Tour Script Change**

In `build_guide_gent_tour_script()`, remove the `GiveItem` for MAP_CARD. MAP_CARD is an engine flag, not a bag item. The original plan has:

```rust
// ORIGINAL (WRONG):
GiveItem { item_id: ITEM_MAP_CARD, count: 1 },
SetEvent(EVENT_ENGINE_MAP_CARD),
ShowText("<PLAYER>'s POKeGEAR now has a MAP!".into()),

// CORRECTED:
SetEvent(EVENT_ENGINE_MAP_CARD),
ShowText("<PLAYER>'s POKeGEAR now has a MAP!".into()),
// GiveItem removed -- MAP_CARD is tracked via EVENT_ENGINE_MAP_CARD flag only
```

Also update the test spec: `test_guide_gent_gives_map_card` should verify `EVENT_ENGINE_MAP_CARD` is set, but should NOT check for ITEM_MAP_CARD in the bag.

**Important note on music constants in scripts**: The scripts reference `MUSIC_SHOW_ME_AROUND`, `MUSIC_RIVAL_ENCOUNTER`, and `MUSIC_RIVAL_AFTER`. These are now defined in `data.rs` (Review #14), so events.rs must import them:

```rust
use super::data::{
    // ... existing imports ...
    BattleType, MUSIC_SHOW_ME_AROUND, MUSIC_RIVAL_ENCOUNTER, MUSIC_RIVAL_AFTER,
    ITEM_MYSTIC_WATER, ITEM_POTION,
    // GIMLI FIX: ITEM_MAP_CARD removed -- MAP_CARD is an engine flag, not a bag item
};
```

### Compile Check

After Phase 3: All script_ids resolve. `ScriptResult` enum compiles. `step_script()` returns `ScriptResult` instead of `bool`. `loaded_wild_species` field exists on `ScriptState`.

---

## Phase 4: Battle System Foundation (`battle.rs`) -- NEW MODULE

**File**: `pokemonv2/battle.rs`

### REVIEW CHANGE #4: Imports from data.rs Only

```rust
use super::data::{
    Pokemon, SpeciesId, MoveId, PokemonType,
    BattleType, BattleResult,  // REVIEW #4: from data.rs, NOT events.rs
    species_data, move_data, type_effectiveness,
    MOVE_STRUGGLE,  // REVIEW #17: from data.rs
};
```

### Core Types

```rust
#[derive(Clone, Debug)]
pub struct BattleState {
    pub enemy: Pokemon,
    pub battle_type: BattleType,
    pub turn_count: u8,
    pub phase: BattlePhase,
    pub message: Option<String>,
    pub message_timer: f64,
    pub result: Option<BattleResult>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BattlePhase {
    Intro,
    PlayerTurn,
    EnemyTurn,
    Message,
    Victory,
    Defeat,
    Caught,
    Flee,
}
```

**Note**: `BattleType` and `BattleResult` are NOT defined here. They're imported from `data.rs` (Review #4).

### Constants

```rust
const MESSAGE_TIME: f64 = 1.5;
const AUTO_FLEE_TURNS: u8 = 10;
const TUTORIAL_CATCH_TURN: u8 = 3;
```

**Note**: `MOVE_STRUGGLE` is NOT defined here. It's imported from `data.rs` (Review #17).

### Battle Creation, Damage Calc, Move Selection, Auto-Battle Step

**Identical to original plan.** See SPRINT2_IMPLEMENTATION_PLAN.md Phase 4 for full code of:
- `BattleState::new_wild()` and `BattleState::new_trainer()`
- `calc_damage()` -- Gen 2 damage formula with STAB, type effectiveness, random factor
- `pick_damaging_move()` -- returns MOVE_STRUGGLE if no damaging move
- `step_battle()` -- auto-battle loop with Tutorial/Wild/CanLose handling

### Compile Check

After Phase 4: `BattleState::new_wild(PIDGEY, 2, BattleType::Wild)` compiles. battle.rs imports ONLY from data.rs.

---

## Phase 5: Overworld Systems (`overworld.rs`)

**File**: `pokemonv2/overworld.rs`
**Goal**: Ledge movement, wild encounter checks, map connection transitions.

### REVIEW CHANGE #7: Import is_walkable_with_direction from maps.rs

```rust
use super::maps::{
    find_bg_event, find_coord_event, find_warp,
    is_walkable, is_walkable_with_direction,  // REVIEW #7: both from maps.rs
    MapData, MapId, NpcMoveType,
    C_GRASS, C_LEDGE_D, C_LEDGE_L, C_LEDGE_R,  // REVIEW #5: import collision constants
};
use super::data::{CameraState, Direction, NpcState, PlayerState, TimeOfDay, SpeciesId};
use super::events::{EventFlags, SceneState};
use crate::engine::Engine;
```

### New OverworldResult Variants

```rust
pub enum OverworldResult {
    Nothing,
    WarpTo { dest_map: MapId, dest_warp_id: u8 },
    TriggerScript { script_id: u16, npc_idx: Option<u8> },
    TriggerCoordEvent { script_id: u16 },
    // Sprint 2 additions:
    WildEncounter { species: SpeciesId, level: u8 },
    MapConnection { direction: Direction, dest_map: MapId, offset: i8 },
}
```

### REVIEW CHANGE #3 (CRITICAL): step_overworld() Signature

The original plan didn't specify how RNG bytes and TimeOfDay reach `step_overworld()`. Using Option A from the review: the caller (mod.rs) extracts RNG bytes and TimeOfDay before calling.

```rust
/// Main overworld update -- called each frame when GamePhase == Overworld.
pub fn step_overworld(
    player: &mut PlayerState,
    camera: &mut CameraState,
    map: &MapData,
    npc_states: &mut Vec<NpcState>,
    _flags: &EventFlags,
    scenes: &SceneState,
    engine: &Engine,
    time_of_day: TimeOfDay,    // NEW: caller computes from total_time
    rng_enc: u8,               // NEW: random byte for encounter check
    rng_slot: u8,              // NEW: random byte for slot selection
) -> OverworldResult {
```

### Movement Input: Use is_walkable_with_direction

In the directional input handling section, replace `is_walkable` with `is_walkable_with_direction`:

```rust
if let Some(dir) = maybe_dir {
    player.facing = dir;
    let (tx, ty) = target_tile(player.x, player.y, dir);

    // REVIEW #7: use direction-aware walkability for ledge support
    if is_walkable_with_direction(map, tx, ty, dir) && npc_at(npc_states, tx, ty).is_none() {
        player.is_walking = true;
        player.walk_offset = 0.0;
    } else if tx < 0 || ty < 0 || tx >= map.width || ty >= map.height {
        // Check for map connection
        let conn = match dir {
            Direction::Left => &map.connections.west,
            Direction::Right => &map.connections.east,
            Direction::Up => &map.connections.north,
            Direction::Down => &map.connections.south,
        };
        if let Some(connection) = conn {
            return OverworldResult::MapConnection {
                direction: dir,
                dest_map: connection.dest_map,
                offset: connection.offset,
            };
        }
    }
}
```

### Walk-Complete: Wild Encounter Check

After walk completes and player position is updated, BEFORE checking warps:

```rust
// After walk completion:
// ...snap to destination...

// Check grass encounter (BEFORE warps)
if let Some((species, level)) = check_wild_encounter(map, player.x, player.y, time_of_day, rng_enc, rng_slot) {
    update_camera(camera, player);
    return OverworldResult::WildEncounter { species, level };
}

// Then check coord events
// Then check warps
// ...existing code...
```

### Wild Encounter Check Function

```rust
pub fn check_wild_encounter(
    map: &MapData,
    x: i32,
    y: i32,
    time_of_day: TimeOfDay,
    rng_encounter: u8,
    rng_slot: u8,
) -> Option<(SpeciesId, u8)> {
    let idx = (y * map.width + x) as usize;  // REVIEW #8: no as i32 cast needed
    if map.collision[idx] != C_GRASS {
        return None;
    }

    if let Some(ref table) = map.wild_encounters {
        if rng_encounter >= table.encounter_rate {
            return None;
        }

        let slots = match time_of_day {
            TimeOfDay::Morning => &table.morning,
            TimeOfDay::Day => &table.day,
            TimeOfDay::Night => &table.night,
        };

        if slots.is_empty() {
            return None;
        }

        let slot_idx = match rng_slot {
            0..=76 => 0,
            77..=153 => 1,
            154..=204 => 2,
            205..=229 => 3,
            230..=242 => 4,
            243..=248 => 5,
            _ => 6,
        };

        let slot_idx = slot_idx.min(slots.len() - 1);
        let slot = &slots[slot_idx];
        Some((slot.species, slot.level))
    } else {
        None
    }
}
```

### Compile Check

After Phase 5: `step_overworld` compiles with new parameters. `is_walkable_with_direction` is imported from maps.rs, not defined here.

---

## Phase 6: Main Module Updates (`mod.rs`)

**File**: `pokemonv2/mod.rs`

### Add battle module declaration

```rust
pub mod data;
pub mod maps;
pub mod events;
pub mod overworld;
pub mod render;
pub mod dialogue;
pub mod sprites;
pub mod battle;  // NEW
```

### REVIEW CHANGE #10: Use Existing Battle GamePhase Variant

The `Battle` variant already exists at mod.rs line 38: `Battle, // stub`. Do NOT add a new variant. Just wire up the existing stub.

### REVIEW CHANGE #11: Add battle Field to PokemonV2Sim

```rust
pub struct PokemonV2Sim {
    // ... existing fields ...
    pub battle: Option<battle::BattleState>,  // NEW
}
```

**Update `new()`** -- add `battle: None`:
```rust
impl PokemonV2Sim {
    pub fn new() -> Self {
        // ... existing code ...
        let mut sim = Self {
            // ... existing fields ...
            battle: None,  // REVIEW #11: initialize in constructor
        };
        // ...
    }
}
```

**Update `with_state()`** -- add `battle: None`:
```rust
#[cfg(test)]
pub fn with_state(map: MapId, x: i32, y: i32, party: Vec<Pokemon>) -> Self {
    let mut sim = Self::new();
    // ... existing setup ...
    sim.battle = None;  // REVIEW #11: also in test helper
    sim
}
```

### REVIEW CHANGE #10: Replace Battle Stub in step()

```rust
// BEFORE:
GamePhase::Battle | GamePhase::Menu => {} // stubs

// AFTER:
GamePhase::Battle => self.step_battle_phase(engine),
GamePhase::Menu => {} // stub
```

### REVIEW CHANGE #3 (CRITICAL): step_overworld_phase() Extracts RNG + TimeOfDay

```rust
fn step_overworld_phase(&mut self, engine: &Engine) {
    let time_of_day = data::get_time_of_day(self.total_time);
    let rng_enc = (engine.rng.state & 0xFF) as u8;
    let rng_slot = ((engine.rng.state >> 8) & 0xFF) as u8;

    let result = step_overworld(
        &mut self.player,
        &mut self.camera,
        &self.current_map,
        &mut self.npc_states,
        &self.event_flags,
        &self.scene_state,
        engine,
        time_of_day,   // NEW
        rng_enc,       // NEW
        rng_slot,      // NEW
    );
    match result {
        OverworldResult::Nothing => {}
        OverworldResult::WarpTo { dest_map, dest_warp_id } => {
            self.pending_warp = Some((dest_map, dest_warp_id));
            self.phase = GamePhase::MapTransition { timer: 0.0 };
        }
        OverworldResult::TriggerScript { script_id, .. }
        | OverworldResult::TriggerCoordEvent { script_id } => {
            let steps = get_script(script_id);
            self.script = Some(ScriptState::new(steps));
            self.phase = GamePhase::Script;
        }
        OverworldResult::WildEncounter { species, level } => {
            self.start_wild_battle(species, level);
        }
        OverworldResult::MapConnection { direction, dest_map, offset } => {
            self.handle_map_connection(direction, dest_map, offset);
        }
    }
}
```

### REVIEW CHANGE #2 (CRITICAL): step_script_phase() Uses ScriptResult

```rust
fn step_script_phase(&mut self, engine: &Engine) {
    if let Some(ref mut script) = self.script {
        let result = step_script(
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
        match result {
            ScriptResult::Running => {}
            ScriptResult::Ended => {
                self.script = None;
                self.phase = GamePhase::Overworld;
                self.refresh_npc_visibility();
            }
            ScriptResult::StartBattle { battle_type, species } => {
                // Create battle from script
                match battle_type {
                    BattleType::Tutorial | BattleType::Wild => {
                        if let Some((sp, lv)) = species {
                            let battle_state = battle::BattleState::new_wild(sp, lv, battle_type);
                            self.battle = Some(battle_state);
                            self.phase = GamePhase::Battle;
                        }
                    }
                    BattleType::CanLose | BattleType::Normal => {
                        // Rival battle: determine counter-starter
                        let rival_species = self.get_rival_species();
                        let battle_state = battle::BattleState::new_trainer(rival_species, 5, battle_type);
                        self.battle = Some(battle_state);
                        self.phase = GamePhase::Battle;
                    }
                }
            }
        }
    } else {
        self.phase = GamePhase::Overworld;
    }
}
```

### Battle Phase Step

```rust
fn step_battle_phase(&mut self, engine: &Engine) {
    let rng_byte = (engine.rng.state & 0xFF) as u8;

    if let Some(ref mut battle_state) = self.battle {
        if let Some(ref mut pokemon) = self.party.first_mut() {
            let still_running = battle::step_battle(battle_state, pokemon, 1.0 / 60.0, rng_byte);
            if !still_running {
                let result = battle_state.result;
                let battle_type = battle_state.battle_type;
                self.battle = None;

                match result {
                    Some(BattleResult::Lost) if battle_type != BattleType::CanLose => {
                        // Whiteout: heal party, warp to last center
                        self.heal_party();
                        self.warp_to_last_pokecenter();
                    }
                    _ => {
                        // Resume: if we have a pending script, go back to Script phase
                        self.phase = if self.script.is_some() {
                            GamePhase::Script
                        } else {
                            GamePhase::Overworld
                        };
                    }
                }
            }
        }
    }
}
```

### Helper Methods

```rust
fn start_wild_battle(&mut self, species: SpeciesId, level: u8) {
    let battle_state = battle::BattleState::new_wild(species, level, BattleType::Wild);
    self.battle = Some(battle_state);
    self.phase = GamePhase::Battle;
}

fn handle_map_connection(&mut self, direction: Direction, dest_map: MapId, offset: i8) {
    self.current_map_id = dest_map;
    self.current_map = load_map(dest_map);
    self.npc_states = init_npc_states(&self.current_map);
    self.temp_flags = 0;

    match direction {
        Direction::Left => {
            self.player.x = self.current_map.width - 1;
            self.player.y = self.player.y + offset as i32;
        }
        Direction::Right => {
            self.player.x = 0;
            self.player.y = self.player.y + offset as i32;
        }
        Direction::Up => {
            self.player.y = self.current_map.height - 1;
            self.player.x = self.player.x + offset as i32;
        }
        Direction::Down => {
            self.player.y = 0;
            self.player.x = self.player.x + offset as i32;
        }
    }
    snap_camera(&mut self.camera, &self.player);
    self.refresh_npc_visibility();
    self.check_map_callbacks();
}

fn get_rival_species(&self) -> SpeciesId {
    if self.event_flags.has(EVENT_GOT_CYNDAQUIL_FROM_ELM) { TOTODILE }
    else if self.event_flags.has(EVENT_GOT_TOTODILE_FROM_ELM) { CHIKORITA }
    else { CYNDAQUIL }
}

fn heal_party(&mut self) {
    for p in self.party.iter_mut() {
        p.hp = p.max_hp;
        p.status = data::StatusCondition::None;
    }
}

fn warp_to_last_pokecenter(&mut self) {
    // Sprint 2: warp to Cherrygrove Pokecenter if flypoint set, else Elm's Lab
    let dest = if self.event_flags.has(EVENT_ENGINE_FLYPOINT_CHERRYGROVE) {
        MapId::CherrygrovePokecenter1F
    } else {
        MapId::ElmsLab
    };
    self.change_map(dest, 0);
    self.phase = GamePhase::Overworld;
}
```

### REVIEW CHANGE #19: check_map_callbacks Inlines NPC Search

```rust
fn check_map_callbacks(&mut self) {
    match self.current_map_id {
        MapId::CherrygroveCity => {
            self.event_flags.set(EVENT_ENGINE_FLYPOINT_CHERRYGROVE);
        }
        MapId::Route29 => {
            // Tuscany: always hidden for Sprint 2 (no day-of-week system)
            // REVIEW #19: inline loop instead of undefined find_npc_by_event_flag method
            for (i, npc_def) in self.current_map.npcs.iter().enumerate() {
                if npc_def.event_flag == Some(EVENT_ROUTE_29_TUSCANY_OF_TUESDAY) {
                    if let Some(state) = self.npc_states.get_mut(i) {
                        state.visible = false;
                    }
                }
            }
        }
        _ => {}
    }
}
```

### Compile Check

After Phase 6: Full game loop compiles. Battle phase is wired. Script returns ScriptResult. Map connections trigger map changes.

---

## Phase 7: Render Pipeline Updates (`render.rs`)

**File**: `pokemonv2/render.rs`

### REVIEW CHANGE #10: Update Battle Render Dispatch

```rust
// BEFORE:
GamePhase::Battle | GamePhase::Menu => render_overworld(sim, engine), // stubs

// AFTER:
GamePhase::Battle => render_battle(sim, engine),
GamePhase::Menu => render_overworld(sim, engine), // stub
```

### REVIEW CHANGE #12: Fix draw_text Argument Order

The existing `draw_text` signature is `fn draw_text(engine, text, x, y, color)`. The original plan's battle render had the arguments in wrong order. Use the correct order:

```rust
fn render_battle(sim: &PokemonV2Sim, engine: &mut Engine) {
    fill_rect(engine, 0, 0, SCREEN_W, SCREEN_H, Color::from_rgba(248, 248, 248, 255));

    if let Some(ref battle) = sim.battle {
        if let Some(ref player_mon) = sim.party.first() {
            let player_data = data::species_data(player_mon.species);
            let enemy_data = data::species_data(battle.enemy.species);

            // Enemy info (top-left)
            // REVIEW #12: correct argument order: (engine, text, x, y, color)
            draw_text(engine, enemy_data.name, 8, 8, Color::from_rgba(0, 0, 0, 255));
            draw_text(engine, &format!("Lv{}", battle.enemy.level), 8, 18, Color::from_rgba(0, 0, 0, 255));
            let enemy_hp_pct = battle.enemy.hp as f64 / battle.enemy.max_hp as f64;
            draw_hp_bar(engine, 8, 28, enemy_hp_pct);

            // Player info (bottom-right)
            let py = SCREEN_H - 50;
            draw_text(engine, player_data.name, 80, py, Color::from_rgba(0, 0, 0, 255));
            draw_text(engine, &format!("Lv{}", player_mon.level), 80, py + 10, Color::from_rgba(0, 0, 0, 255));
            draw_text(engine, &format!("HP {}/{}", player_mon.hp, player_mon.max_hp), 80, py + 20, Color::from_rgba(0, 0, 0, 255));
            let player_hp_pct = player_mon.hp as f64 / player_mon.max_hp as f64;
            draw_hp_bar(engine, 80, py + 30, player_hp_pct);
        }

        if let Some(ref msg) = battle.message {
            draw_text_box(engine, msg);
        }
    }
}

fn draw_hp_bar(engine: &mut Engine, x: i32, y: i32, pct: f64) {
    let bar_w = 60;
    let bar_h = 4;
    fill_rect(engine, x, y, bar_w, bar_h, Color::from_rgba(64, 64, 64, 255));
    let fill_w = (pct * bar_w as f64) as i32;
    let color = if pct > 0.5 {
        Color::from_rgba(0, 200, 0, 255)
    } else if pct > 0.2 {
        Color::from_rgba(200, 200, 0, 255)
    } else {
        Color::from_rgba(200, 0, 0, 255)
    };
    fill_rect(engine, x, y, fill_w, bar_h, color);
}
```

### Grass and Ledge Tile Rendering, New Sprites

**Identical to original plan.** See SPRINT2_IMPLEMENTATION_PLAN.md Phase 7.

### Compile Check

After Phase 7: Full render pipeline compiles. Battle screen draws with correct `draw_text` argument order.

---

## Phase 8: Tests

**Identical to original plan** with the following adjustments for review changes:

### Test Adjustments

1. **Wild encounters use `Option`**: Tests should check `map.wild_encounters.is_some()` / `.is_none()` instead of `.is_empty()`.

2. **ScriptResult tests**: Any test that calls `step_script()` must match against `ScriptResult::Running` / `ScriptResult::Ended` instead of `true` / `false`.

3. **step_overworld() tests**: Must pass `time_of_day`, `rng_enc`, `rng_slot` parameters.

4. **Ledge tests use maps::is_walkable_with_direction**: Import from maps, not overworld.

5. **MOVE_STRUGGLE imported from data**: `use super::data::MOVE_STRUGGLE;`

See SPRINT2_IMPLEMENTATION_PLAN.md Phase 8 for full test specifications. All test code is structurally identical but with the parameter/return-type adjustments noted above.

---

## Implementation Order Summary

| Phase | Module | Key Review Changes |
|-------|--------|--------------------|
| 1 | data.rs | #4: BattleType/BattleResult here. #17: MOVE_STRUGGLE here. #14: Music constants here. |
| 2 | maps.rs | #1: wild_encounters vec![] -> None. Remove WildEncounter. #7: is_walkable_with_direction here. #13: Route30 stub. #6: Gate 5x4 blocks. |
| 3 | events.rs | #2: ScriptResult enum, loaded_wild_species on ScriptState, step_script returns ScriptResult. #9: SceneState vec size 32. |
| 4 | battle.rs | #4: Import BattleType from data, not events. #17: Import MOVE_STRUGGLE from data. |
| 5 | overworld.rs | #3: step_overworld takes time_of_day, rng_enc, rng_slot. #7: Import is_walkable_with_direction from maps. #8: No redundant as i32 casts. |
| 6 | mod.rs | #10: Use existing Battle variant. #11: battle: None in constructors. #2: Handle ScriptResult. #3: Extract RNG/TimeOfDay before calling step_overworld. #19: Inline Tuscany loop. |
| 7 | render.rs | #12: Correct draw_text arg order. #10: Wire Battle render. |
| 8 | Tests | Adjust for Option<WildEncounterTable>, ScriptResult, new step_overworld params. |
