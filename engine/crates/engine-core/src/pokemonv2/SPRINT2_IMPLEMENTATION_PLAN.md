# Sprint 2 Implementation Plan: Route 29, Route 29/46 Gate, and Cherrygrove City

> Written by Frodo (Technical Writer)
> Source: SPRINT2_ARCHITECTURE.md (Gandalf), ARCHITECTURE.md (Gandalf + Aragorn), pokecrystal-master .asm files
> Target: Rust engineers implementing pokemonv2/
> Depends on: Sprint 1 fully implemented (New Bark Town, Player's House, Elm's Lab)

---

## Module File Plan

After Sprint 2, the directory looks like:

```
pokemonv2/
  mod.rs          -- PokemonV2Sim, GamePhase, Simulation impl, input helpers, battle/map-connection dispatch
  data.rs         -- (expanded) 5 new species, 4 new moves, TimeOfDay, new item constants
  maps.rs         -- (expanded) 8 new/replaced maps, C_GRASS/C_LEDGE_D collision, WildEncounterTable
  overworld.rs    -- (expanded) ledge movement, wild encounter checks, map edge -> MapConnection
  events.rs       -- (expanded) 15+ new event flags, 6 new ScriptStep variants, ~20 new scripts
  battle.rs       -- NEW MODULE: BattleState, BattlePhase, auto-battle loop, Gen 2 damage calc
  render.rs       -- (expanded) grass tiles, ledge tiles, battle screen stub
  dialogue.rs     -- (unchanged)
  sprites.rs      -- (expanded) 11 new sprite IDs
```

### Acyclic Import Graph (Updated for Sprint 2)

```
data.rs          <- LEAF (no sibling imports)
  |
  +-- events.rs  <- imports data.rs, maps::MapId
  +-- maps.rs    <- imports data::{Direction, SpeciesId, NpcState, TimeOfDay}
  +-- battle.rs  <- imports data::{Pokemon, SpeciesId, MoveId, PokemonType, species_data, move_data, type_effectiveness}
  +-- dialogue.rs <- LEAF
  +-- sprites.rs <- imports data::{Direction, Emote}
  |
  +-- overworld.rs <- imports data, maps, events::{EventFlags, SceneState}
  |
  +-- render.rs  <- imports data, maps, overworld::constants, events, sprites, battle
  |
  +-- mod.rs     <- imports everything
```

No module imports from a module that imports back from it. `battle.rs` depends ONLY on `data.rs` -- it does not import maps, events, or overworld.

---

## Phase 1: Core Data Additions (`data.rs`)

**File**: `pokemonv2/data.rs`
**Goal**: Add 5 new species, 4 new moves, TimeOfDay enum, new item constants. All compile with no game logic changes.

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

### New Move Data Entries

Add to `move_data()` match:

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
    move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 15, is_special: false,
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

**Note on Hoppip**: Hoppip's level-1 move is Splash (does nothing). Its first damaging move is Tackle at level 10. At levels 2-3 (Route 29 encounter levels), Hoppip knows only Splash. The battle system must handle this edge case -- if a Pokemon has no damaging move, it uses Struggle (see Phase 4).

### New Item Constants

```rust
pub const ITEM_POKE_BALL: u8 = 4;
pub const ITEM_ANTIDOTE: u8 = 18;
pub const ITEM_PARLYZ_HEAL: u8 = 19;
pub const ITEM_AWAKENING: u8 = 20;
pub const ITEM_MYSTIC_WATER: u8 = 41;
pub const ITEM_PINK_BOW: u8 = 42;
pub const ITEM_MAP_CARD: u8 = 43;
```

### Dependencies

None. data.rs remains a leaf module.

### Compile Check

After Phase 1: `cargo check` succeeds. `species_data(PIDGEY)`, `move_data(MOVE_TAIL_WHIP)`, `get_time_of_day(500.0)` all return valid data.

---

## Phase 2: Map System Expansion (`maps.rs`)

**File**: `pokemonv2/maps.rs`
**Goal**: Add 8 new/replaced maps, new collision types, WildEncounterTable struct, map connection data.

### New MapId Variants

Add to existing `MapId` enum:

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
}
```

### New Collision Constants

Add below existing constants:

```rust
pub const C_GRASS: u8 = 5;    // walkable, triggers wild encounter check on step complete
pub const C_LEDGE_D: u8 = 6;  // one-way south: can jump south, blocked from south/east/west
pub const C_LEDGE_L: u8 = 7;  // one-way left (future use)
pub const C_LEDGE_R: u8 = 8;  // one-way right (future use)
```

### WildEncounterTable Struct

Replace the existing `WildEncounter` struct with a time-of-day-aware table.

**Important**: The existing `WildEncounter` struct (`{ species, min_level, max_level, rate }`) and the `wild_encounters: Vec<WildEncounter>` field on `MapData` must be replaced. The old struct is not used by any Sprint 1 map (all have empty wild_encounters vecs).

```rust
/// A single encounter slot: species at a fixed level.
/// Pokecrystal uses 7 slots per time-of-day period with fixed probability distribution.
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
    pub encounter_rate: u8,  // pokecrystal "X percent" -- higher = more frequent
}
```

Update `MapData` to use the new type:

```rust
pub struct MapData {
    // ... existing fields ...
    pub wild_encounters: Option<WildEncounterTable>,  // was: Vec<WildEncounter>
    // ... rest unchanged ...
}
```

### Route 29 Wild Encounter Data

From pokecrystal `data/wild/johto_grass.asm` lines 1237-1262:

```rust
fn build_route29_encounters() -> WildEncounterTable {
    WildEncounterTable {
        encounter_rate: 10,
        morning: vec![
            WildSlot { species: PIDGEY, level: 2 },   // 30%
            WildSlot { species: SENTRET, level: 2 },   // 30%
            WildSlot { species: PIDGEY, level: 3 },    // 20%
            WildSlot { species: SENTRET, level: 3 },   // 10%
            WildSlot { species: RATTATA, level: 2 },   //  5%
            WildSlot { species: HOPPIP, level: 3 },    //  2.5%
            WildSlot { species: HOPPIP, level: 3 },    //  2.5%
        ],
        day: vec![
            WildSlot { species: PIDGEY, level: 2 },    // 30%
            WildSlot { species: SENTRET, level: 2 },    // 30%
            WildSlot { species: PIDGEY, level: 3 },     // 20%
            WildSlot { species: SENTRET, level: 3 },    // 10%
            WildSlot { species: RATTATA, level: 2 },    //  5%
            WildSlot { species: HOPPIP, level: 3 },     //  2.5%
            WildSlot { species: HOPPIP, level: 3 },     //  2.5%
        ],
        night: vec![
            WildSlot { species: HOOTHOOT, level: 2 },  // 30%
            WildSlot { species: RATTATA, level: 2 },    // 30%
            WildSlot { species: HOOTHOOT, level: 3 },   // 20%
            WildSlot { species: RATTATA, level: 3 },    // 10%
            WildSlot { species: RATTATA, level: 2 },    //  5%
            WildSlot { species: HOOTHOOT, level: 3 },   //  2.5%
            WildSlot { species: HOOTHOOT, level: 3 },   //  2.5%
        ],
    }
}
```

### Map Data for All Sprint 2 Maps

All coordinates verified against pokecrystal `.asm` map event data. Block dimensions from pokecrystal headers; tile dimensions = block dimensions * 2.

#### Route 29 (30x9 blocks = 60x18 tiles)

**Replaces the existing Route29 stub map entirely.**

From pokecrystal `maps/Route29.asm`:

- **Warps** (1):
  - (27, 1) -> Route29Route46Gate, dest_warp_id=3
- **NPCs** (8, from `def_object_events`):
  - idx 0: CooltrainerM "Dude" at (50, 12), SPRITEMOVEDATA_SPINRANDOM_SLOW, script=CatchingTutorialDudeScript
  - idx 1: Youngster at (27, 16), WalkUpDown range=1, script=Route29YoungsterScript
  - idx 2: Teacher at (15, 11), WalkLeftRight range=1, script=Route29TeacherScript
  - idx 3: Fruit Tree at (12, 2), Still, script=Route29FruitTree
  - idx 4: Fisher at (25, 3), StandingUp, script=Route29FisherScript
  - idx 5: CooltrainerM at (13, 4), StandingDown, script=Route29CooltrainerMScript
  - idx 6: Teacher "Tuscany" at (29, 12), SpinRandom, event_flag=EVENT_ROUTE_29_TUSCANY_OF_TUESDAY, event_flag_show=true (visible when set)
  - idx 7: PokeBall "Potion" at (48, 2), Still, event_flag=EVENT_ROUTE_29_POTION, event_flag_show=false (visible when NOT set)
- **Coord Events** (2):
  - (53, 8) scene_id=SCENE_ROUTE29_CATCH_TUTORIAL: Route29Tutorial1
  - (53, 9) scene_id=SCENE_ROUTE29_CATCH_TUTORIAL: Route29Tutorial2
- **BG Events** (2 signs):
  - (51, 7) kind=Read: "ROUTE 29\n\nCHERRYGROVE CITY -\nNEW BARK TOWN"
  - (3, 5) kind=Read: "ROUTE 29\n\nCHERRYGROVE CITY -\nNEW BARK TOWN"
- **Connections** (3 from pokecrystal attributes.asm):
  - North -> Route46, offset=10
  - West -> CherrygroveCity, offset=0
  - East -> NewBarkTown, offset=0
- **Wild encounters**: build_route29_encounters() (see above)
- **Initial scene**: SCENE_ROUTE29_CATCH_TUTORIAL (1) -- catching tutorial pending
- **Map callbacks**: MAPCALLBACK_OBJECTS Route29TuscanyCallback

**Route 29 collision layout**: The 60x18 grid contains:
- Floor (C_FLOOR): paths, open areas
- Wall (C_WALL): trees, cliff faces, building walls (gate)
- Grass (C_GRASS): tall grass patches scattered through the route
- Ledge (C_LEDGE_D): south-facing ledge strips creating shortcuts
- Warp (C_WARP): gate entrance at (27, 1)
- Water (C_WATER): not present on Route 29

The exact tile/collision arrays should be derived from the pokecrystal `.blk` file structure, approximated as a playable layout with grass patches in the central areas and ledge strips creating one-way shortcuts southward.

#### Route29Route46Gate (5x4 blocks = 10x8 tiles)

From pokecrystal `maps/Route29Route46Gate.asm`:

- **Warps** (4):
  - (4, 0) -> Route46, dest_warp_id=1
  - (5, 0) -> Route46, dest_warp_id=2
  - (4, 7) -> Route29, dest_warp_id=1 (note: Route29's warp_event uses dest_warp_id=3 which is this warp's INDEX; use matching logic)
  - (5, 7) -> Route29, dest_warp_id=1
- **NPCs** (2):
  - idx 0: Officer at (0, 4), StandingRight, script=Route29Route46GateOfficerScript
  - idx 1: Youngster at (6, 4), WalkUpDown range=1, script=Route29Route46GateYoungsterScript
- **No coord/bg events, no wild encounters, no connections**

#### CherrygroveCity (20x9 blocks = 40x18 tiles)

From pokecrystal `maps/CherrygroveCity.asm`:

- **Warps** (5):
  - (23, 3) -> CherrygroveMart, dest_warp_id=2
  - (29, 3) -> CherrygrovePokecenter1F, dest_warp_id=1
  - (17, 7) -> CherrygroveGymSpeechHouse, dest_warp_id=1
  - (25, 9) -> GuideGentsHouse, dest_warp_id=1
  - (31, 11) -> CherrygroveEvolutionSpeechHouse, dest_warp_id=1
- **NPCs** (5):
  - idx 0: Gramps "Guide Gent" at (32, 6), StandingDown, event_flag=EVENT_GUIDE_GENT_IN_HIS_HOUSE, event_flag_show=false (visible when flag NOT set)
  - idx 1: Rival at (39, 6), SpinRandom, event_flag=EVENT_RIVAL_CHERRYGROVE_CITY, event_flag_show=true (visible when set)
  - idx 2: Teacher at (27, 12), WalkLeftRight range=1, script=CherrygroveTeacherScript
  - idx 3: Youngster at (23, 7), WalkLeftRight range=1, script=CherrygroveYoungsterScript
  - idx 4: Fisher "Mystic Water Guy" at (7, 12), StandingRight, script=MysticWaterGuy
- **Coord Events** (2):
  - (33, 6) scene_id=SCENE_CHERRYGROVECITY_MEET_RIVAL: CherrygroveRivalSceneNorth
  - (33, 7) scene_id=SCENE_CHERRYGROVECITY_MEET_RIVAL: CherrygroveRivalSceneSouth
- **BG Events** (4):
  - (30, 8) kind=Read: "CHERRYGROVE CITY\n\nThe City of Cute,\nFragrant Flowers"
  - (23, 9) kind=Read: "GUIDE GENT'S HOUSE"
  - (24, 3) kind=Read: "CHERRYGROVE MART" (standard mart sign)
  - (30, 3) kind=Read: "POKEMON CENTER" (standard pokecenter sign)
- **Connections** (2 from pokecrystal attributes.asm):
  - North -> Route30, offset=5 (stub -- Route30 not built yet)
  - East -> Route29, offset=0
- **Map callback**: MAPCALLBACK_NEWMAP sets ENGINE_FLYPOINT_CHERRYGROVE
- **No wild encounters** (city map)
- **Initial scene**: SCENE_CHERRYGROVECITY_MEET_RIVAL (1) -- rival appears on return from Mr. Pokemon

#### CherrygrovePokecenter1F (5x4 blocks = 10x8 tiles)

From pokecrystal `maps/CherrygrovePokecenter1F.asm`:

- **Warps** (3):
  - (3, 7) -> CherrygroveCity, dest_warp_id=2 (exit south)
  - (4, 7) -> CherrygroveCity, dest_warp_id=2 (exit south)
  - (0, 7) -> Pokecenter2F (stub -- stairs, locked for Sprint 2)
- **NPCs** (4):
  - idx 0: Nurse at (3, 1), StandingDown, script=CherrygrovePokecenter1FNurseScript (PokecenterNurseScript standard)
  - idx 1: Fisher at (2, 3), StandingUp, script: "It's great. I can store any number of POKeMON, and it's all free."
  - idx 2: Gentleman at (8, 6), StandingUp, script: "That PC is free for any trainer to use."
  - idx 3: Teacher at (1, 6), StandingRight, script: conditional (checks EVENT_GAVE_MYSTERY_EGG_TO_ELM)
- **No coord/bg events, no wild encounters, no connections**

#### CherrygroveMart (6x4 blocks = 12x8 tiles)

From pokecrystal `maps/CherrygroveMart.asm`:

- **Warps** (2):
  - (2, 7) -> CherrygroveCity, dest_warp_id=1
  - (3, 7) -> CherrygroveCity, dest_warp_id=1
- **NPCs** (3):
  - idx 0: Clerk at (1, 3), StandingRight, script=CherrygroveMartClerkScript (checks EVENT_GAVE_MYSTERY_EGG_TO_ELM for inventory phase)
  - idx 1: CooltrainerM at (7, 6), WalkLeftRight range=2, script: conditional ("They're fresh out of POKe BALLS!" / "POKe BALLS are in stock!")
  - idx 2: Youngster at (2, 5), StandingDown, script: "When I was walking in the grass, a bug POKeMON poisoned my POKeMON!..."
- **Mart inventory** (from pokecrystal `data/items/marts.asm`):
  - Pre-egg: Potion, Antidote, Parlyz Heal, Awakening
  - Post-egg (after EVENT_GAVE_MYSTERY_EGG_TO_ELM): Poke Ball, Potion, Antidote, Parlyz Heal, Awakening
- **Sprint 2 mart behavior**: Stub as dialogue. Clerk says "POTION - 300" and gives a free Potion (full mart UI deferred). See Critical Decisions section.
- **No coord/bg events, no wild encounters, no connections**

#### GuideGentsHouse (4x4 blocks = 8x8 tiles)

From pokecrystal `maps/GuideGentsHouse.asm`:

- **Warps** (2):
  - (2, 7) -> CherrygroveCity, dest_warp_id=4
  - (3, 7) -> CherrygroveCity, dest_warp_id=4
- **NPCs** (1):
  - idx 0: Gramps at (2, 3), StandingRight, event_flag=EVENT_GUIDE_GENT_VISIBLE_IN_CHERRYGROVE, event_flag_show=true (visible when Guide Gent flag IS set -- meaning he has returned home after the tour)
  - Script: "When I was a wee lad, I was a hot-shot trainer!..."
- **BG Events** (2):
  - (0, 1) kind=Read: standard bookshelf text
  - (1, 1) kind=Read: standard bookshelf text
- **No coord events, no wild encounters, no connections**

#### CherrygroveGymSpeechHouse (4x4 blocks = 8x8 tiles)

From pokecrystal `maps/CherrygroveGymSpeechHouse.asm`:

- **Warps** (2):
  - (2, 7) -> CherrygroveCity, dest_warp_id=3
  - (3, 7) -> CherrygroveCity, dest_warp_id=3
- **NPCs** (2):
  - idx 0: PokefanM at (2, 3), StandingDown, script: "You're trying to see how good you are as a POKeMON trainer?..."
  - idx 1: BugCatcher at (5, 5), WalkLeftRight range=1, script: "When I get older, I'm going to be a GYM LEADER!..."
- **BG Events** (2):
  - (0, 1) kind=Read: standard bookshelf text
  - (1, 1) kind=Read: standard bookshelf text
- **No coord events, no wild encounters, no connections**

#### CherrygroveEvolutionSpeechHouse (4x4 blocks = 8x8 tiles)

From pokecrystal `maps/CherrygroveEvolutionSpeechHouse.asm`:

- **Warps** (2):
  - (2, 7) -> CherrygroveCity, dest_warp_id=5
  - (3, 7) -> CherrygroveCity, dest_warp_id=5
- **NPCs** (2):
  - idx 0: Lass at (3, 5), StandingLeft, script: "POKeMON change? I would be shocked if one did that!"
  - idx 1: Youngster at (2, 5), StandingRight, script: "POKeMON gain experience in battle and change their form."
- **BG Events** (2):
  - (0, 1) kind=Read: standard bookshelf text
  - (1, 1) kind=Read: standard bookshelf text
- **No coord events, no wild encounters, no connections**

### Stub Maps

**Route46** -- needed as a warp target for Route29Route46Gate's north warps. Minimal 10x8 room with one exit warp back to the gate.

```rust
MapId::Route46 => {
    // Stub: 10x8, one warp
    // Warp 0: (4, 7) -> Route29Route46Gate, dest_warp_id=0
    // Warp 1: (4, 0) -> Route29Route46Gate, dest_warp_id=0 (north exit target)
    // Warp 2: (5, 0) -> Route29Route46Gate, dest_warp_id=1 (north exit target)
}
```

**Route30** -- needed as CherrygroveCity's north connection target. Minimal stub similar to Sprint 1's Route29 stub.

```rust
// If Route30 is not already in MapId, add it as a stub:
MapId::Route30 => {
    // Stub: 10x10, connections south to CherrygroveCity
}
```

**Note**: Update `MapId` enum with `Route30` if not already present. Sprint 1 had Route27 as a stub; this follows the same pattern.

### Warp Bidirectionality Verification

All warps must be bidirectional. Cross-reference table:

| From Map | Warp Pos | Dest Map | Dest Warp ID | Return: Dest Warp Pos | Return Dest | Return Warp ID |
|----------|----------|----------|-------------|----------------------|-------------|---------------|
| Route29 | (27,1) | R29R46Gate | 3 | R29R46Gate (4,7)/(5,7) | Route29 | 1 |
| R29R46Gate | (4,0)/(5,0) | Route46 | 1/2 | Route46 stub warps | R29R46Gate | 0/1 |
| CherrygroveCity | (23,3) | CherrygroveMart | 2 | CherrygroveMart (2,7)/(3,7) | CherrygroveCity | 1 |
| CherrygroveCity | (29,3) | CherrygrovePokecenter1F | 1 | Pokecenter (3,7)/(4,7) | CherrygroveCity | 2 |
| CherrygroveCity | (17,7) | GymSpeechHouse | 1 | GymSpeech (2,7)/(3,7) | CherrygroveCity | 3 |
| CherrygroveCity | (25,9) | GuideGentsHouse | 1 | GuideGent (2,7)/(3,7) | CherrygroveCity | 4 |
| CherrygroveCity | (31,11) | EvoSpeechHouse | 1 | EvoSpeech (2,7)/(3,7) | CherrygroveCity | 5 |

**Critical note on warp indexing**: Pokecrystal warp indices are 1-based in the .asm files. Our Rust arrays are 0-based. The pokecrystal `warp_event 27, 1, ROUTE_29_ROUTE_46_GATE, 3` means "warp to the 3rd warp entry (1-based) in Route29Route46Gate". In 0-based arrays, that's index 2. **Implementers must adjust**: either use 1-based IDs throughout (matching pokecrystal) and subtract 1 at lookup time, or convert all references now. Sprint 1 already established a convention -- follow it.

### Map Connection Data

```rust
// In Route29 MapData:
connections: MapConnections {
    north: Some(MapConnection { direction: Direction::Up, dest_map: MapId::Route46, offset: 10 }),
    south: None,
    east: Some(MapConnection { direction: Direction::Right, dest_map: MapId::NewBarkTown, offset: 0 }),
    west: Some(MapConnection { direction: Direction::Left, dest_map: MapId::CherrygroveCity, offset: 0 }),
},

// In CherrygroveCity MapData:
connections: MapConnections {
    north: Some(MapConnection { direction: Direction::Up, dest_map: MapId::Route30, offset: 5 }),
    south: None,
    east: Some(MapConnection { direction: Direction::Right, dest_map: MapId::Route29, offset: 0 }),
    west: None,
},

// In NewBarkTown MapData (MODIFY existing):
// The west connection to Route29 already exists from Sprint 1 but was a stub target.
// No changes needed if the connection already points to MapId::Route29.
connections: MapConnections {
    north: None,
    south: None,
    east: Some(MapConnection { direction: Direction::Right, dest_map: MapId::Route27, offset: 0 }),
    west: Some(MapConnection { direction: Direction::Left, dest_map: MapId::Route29, offset: 0 }),
},
```

### Dependencies

- `use super::data::{Direction, SpeciesId, NpcState, TimeOfDay};`

### Compile Check

After Phase 2: `load_map(MapId::Route29)` returns a full 60x18 map with grass tiles, ledge tiles, 8 NPCs, encounter table. `load_map(MapId::CherrygroveCity)` returns 40x18 map with 5 NPCs and connections. All 8+ new maps load without panic.

---

## Phase 3: Event System Expansion (`events.rs`)

**File**: `pokemonv2/events.rs`
**Goal**: New event flags, scene constants, ScriptStep variants, and ~20 new scripts.

### New Event Flag Constants

Starting after Sprint 1's highest (EVENT_PLAYERS_HOUSE_1F_NEIGHBOR = 16):

```rust
pub const EVENT_GUIDE_GENT_VISIBLE_IN_CHERRYGROVE: u16 = 17;
pub const EVENT_GUIDE_GENT_IN_HIS_HOUSE: u16 = 18;
pub const EVENT_RIVAL_CHERRYGROVE_CITY: u16 = 19;
pub const EVENT_GOT_MYSTIC_WATER_IN_CHERRYGROVE: u16 = 20;
pub const EVENT_ENGINE_MAP_CARD: u16 = 21;
pub const EVENT_ENGINE_FLYPOINT_CHERRYGROVE: u16 = 22;
pub const EVENT_DUDE_TALKED_TO_YOU: u16 = 23;
pub const EVENT_LEARNED_TO_CATCH_POKEMON: u16 = 24;
pub const EVENT_ROUTE_29_TUSCANY_OF_TUESDAY: u16 = 25;
pub const EVENT_ROUTE_29_POTION: u16 = 26;
pub const EVENT_MET_TUSCANY_OF_TUESDAY: u16 = 27;
pub const EVENT_GOT_PINK_BOW_FROM_TUSCANY: u16 = 28;
pub const EVENT_GAVE_MYSTERY_EGG_TO_ELM: u16 = 29;
pub const EVENT_ENGINE_POKEDEX: u16 = 30;
pub const EVENT_ENGINE_ZEPHYRBADGE: u16 = 31;
pub const EVENT_GOT_TOTODILE_FROM_ELM: u16 = 32;
pub const EVENT_GOT_CHIKORITA_FROM_ELM: u16 = 33;
```

**Note**: EVENT_GOT_TOTODILE_FROM_ELM and EVENT_GOT_CHIKORITA_FROM_ELM may already exist from Sprint 1 (EVENT_GOT_CYNDAQUIL_FROM_ELM = 6, EVENT_GOT_TOTODILE_FROM_ELM = 7, EVENT_GOT_CHIKORITA_FROM_ELM = 8). If so, do not re-add them -- just reference the existing constants. The rival battle uses these to determine the counter-starter.

### New Scene Constants

```rust
// Route 29 scenes
pub const SCENE_ROUTE29_NOOP: u8 = 0;
pub const SCENE_ROUTE29_CATCH_TUTORIAL: u8 = 1;

// Cherrygrove scenes
pub const SCENE_CHERRYGROVECITY_NOOP: u8 = 0;
pub const SCENE_CHERRYGROVECITY_MEET_RIVAL: u8 = 1;
```

### New ScriptStep Variants

Add to existing `ScriptStep` enum:

```rust
pub enum ScriptStep {
    // ... existing Sprint 1 variants ...

    // Sprint 2 additions:

    /// Start a scripted wild battle (e.g., catching tutorial).
    /// The battle system creates a wild Pokemon from species+level.
    LoadWildMon { species: SpeciesId, level: u8 },

    /// Start a battle. battle_type determines behavior.
    StartBattle { battle_type: BattleType },

    /// Follow an NPC (player trails behind them step-by-step).
    /// Sprint 2 simplified: use matching MovePlayer steps after MoveNpc instead.
    /// This variant is reserved for future true follow implementation.
    Follow { npc_idx: u8 },

    /// Stop following.
    StopFollow,

    /// Teleport an NPC to a position without animation (pokecrystal moveobject).
    MoveObject { npc_idx: u8, x: i32, y: i32 },

    /// Resume map's default music (after special music like rival encounter).
    PlayMapMusic,

    /// Execute a special game function.
    Special(SpecialFn),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BattleType {
    Wild,          // standard wild encounter
    Tutorial,      // BATTLETYPE_TUTORIAL -- catching demo, auto-catch
    CanLose,       // BATTLETYPE_CANLOSE -- rival, no game-over on loss
    Normal,        // standard trainer battle
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SpecialFn {
    HealParty,          // restore all party HP to max
    RestartMapMusic,    // resume map music after special track
    FadeOutMusic,       // fade out current music
}
```

**Note on Follow/StopFollow**: In pokecrystal, `follow` makes the player walk behind an NPC step-for-step. For Sprint 2, we simplify: the Guide Gent tour uses `MoveNpc` + matching `MovePlayer` steps to achieve a similar visual result. The `Follow`/`StopFollow` variants exist in the enum for script accuracy but the implementation can be a no-op that just sets/clears a flag. The `MovePlayer` calls handle actual movement.

### Script ID Constants (Sprint 2)

Continue from Sprint 1's highest script_id:

```rust
// Route 29 scripts
pub const SCRIPT_ROUTE29_SIGN1: u16 = 200;
pub const SCRIPT_ROUTE29_SIGN2: u16 = 201;
pub const SCRIPT_CATCHING_TUTORIAL_DUDE: u16 = 202;
pub const SCRIPT_CATCHING_TUTORIAL_1: u16 = 203;
pub const SCRIPT_CATCHING_TUTORIAL_2: u16 = 204;
pub const SCRIPT_ROUTE29_YOUNGSTER: u16 = 205;
pub const SCRIPT_ROUTE29_TEACHER: u16 = 206;
pub const SCRIPT_ROUTE29_FISHER: u16 = 207;
pub const SCRIPT_ROUTE29_COOLTRAINER_M: u16 = 208;
pub const SCRIPT_ROUTE29_FRUIT_TREE: u16 = 209;
pub const SCRIPT_ROUTE29_POTION: u16 = 210;
pub const SCRIPT_TUSCANY: u16 = 211;

// Route29Route46Gate scripts
pub const SCRIPT_GATE_OFFICER: u16 = 220;
pub const SCRIPT_GATE_YOUNGSTER: u16 = 221;

// Cherrygrove City scripts
pub const SCRIPT_GUIDE_GENT_TOUR: u16 = 230;
pub const SCRIPT_GUIDE_GENT_NO: u16 = 231;
pub const SCRIPT_RIVAL_SCENE_NORTH: u16 = 232;
pub const SCRIPT_RIVAL_SCENE_SOUTH: u16 = 233;
pub const SCRIPT_CHERRYGROVE_TEACHER: u16 = 234;
pub const SCRIPT_CHERRYGROVE_YOUNGSTER: u16 = 235;
pub const SCRIPT_MYSTIC_WATER_GUY: u16 = 236;
pub const SCRIPT_CHERRYGROVE_SIGN: u16 = 237;
pub const SCRIPT_GUIDE_GENT_HOUSE_SIGN: u16 = 238;
pub const SCRIPT_CHERRYGROVE_MART_SIGN: u16 = 239;
pub const SCRIPT_CHERRYGROVE_POKECENTER_SIGN: u16 = 240;

// Pokecenter scripts
pub const SCRIPT_NURSE_JOY: u16 = 250;
pub const SCRIPT_POKECENTER_FISHER: u16 = 251;
pub const SCRIPT_POKECENTER_GENTLEMAN: u16 = 252;
pub const SCRIPT_POKECENTER_TEACHER: u16 = 253;

// Mart scripts
pub const SCRIPT_MART_CLERK: u16 = 260;
pub const SCRIPT_MART_COOLTRAINER_M: u16 = 261;
pub const SCRIPT_MART_YOUNGSTER: u16 = 262;

// House scripts
pub const SCRIPT_GUIDE_GENT_HOUSE_GRAMPS: u16 = 270;
pub const SCRIPT_GUIDE_GENT_HOUSE_BOOKSHELF: u16 = 271;
pub const SCRIPT_GYM_SPEECH_POKEFAN: u16 = 280;
pub const SCRIPT_GYM_SPEECH_BUG_CATCHER: u16 = 281;
pub const SCRIPT_GYM_SPEECH_BOOKSHELF: u16 = 282;
pub const SCRIPT_EVO_SPEECH_LASS: u16 = 290;
pub const SCRIPT_EVO_SPEECH_YOUNGSTER: u16 = 291;
pub const SCRIPT_EVO_SPEECH_BOOKSHELF: u16 = 292;
```

### Major Script Implementations

All scripts below are direct translations from pokecrystal `.asm` files with exact dialogue text.

#### 1. Guide Gent City Tour (SCRIPT_GUIDE_GENT_TOUR)

From pokecrystal `maps/CherrygroveCity.asm` `CherrygroveCityGuideGent`:

```rust
fn build_guide_gent_tour_script() -> Vec<ScriptStep> {
    vec![
        // Intro dialogue (Guide Gent intro + yes/no handled by NPC talk wrapper)
        ShowText("OK, then!\nFollow me!".into()),
        WaitButton,
        CloseText,
        PlayMusic(MUSIC_SHOW_ME_AROUND),
        Follow { npc_idx: 0 },
        // Movement 1: Guide Gent walks LEFT LEFT UP LEFT -> stops at Pokecenter
        MoveNpc { npc_idx: 0, steps: vec![(Left, 2), (Up, 1), (Left, 1)] },
        MovePlayer { steps: vec![(Left, 2), (Up, 1), (Left, 1)] },
        ShowText("This is a POKeMON CENTER. They heal your POKeMON in no time at all.\n\nYou'll be relying on them a lot, so you better learn about them.".into()),
        WaitButton,
        CloseText,
        // Movement 2: LEFT x6, face UP -> stops at Mart
        MoveNpc { npc_idx: 0, steps: vec![(Left, 6)] },
        MovePlayer { steps: vec![(Left, 6)] },
        TurnPlayer(Direction::Up),
        ShowText("This is a POKeMON MART.\n\nThey sell BALLS for catching wild POKeMON and other useful items.".into()),
        WaitButton,
        CloseText,
        // Movement 3: LEFT x7, face UP -> Route 30 exit
        MoveNpc { npc_idx: 0, steps: vec![(Left, 7)] },
        MovePlayer { steps: vec![(Left, 7)] },
        TurnPlayer(Direction::Up),
        ShowText("ROUTE 30 is out this way.\n\nTrainers will be battling their prized POKeMON there.".into()),
        WaitButton,
        CloseText,
        // Movement 4: LEFT x3 DOWN LEFT x3 DOWN, face LEFT -> sea overlook
        MoveNpc { npc_idx: 0, steps: vec![(Left, 3), (Down, 1), (Left, 3), (Down, 1)] },
        MovePlayer { steps: vec![(Left, 3), (Down, 1), (Left, 3), (Down, 1)] },
        TurnPlayer(Direction::Left),
        ShowText("This is the sea, as you can see.\n\nSome POKeMON are found only in water.".into()),
        WaitButton,
        CloseText,
        // Movement 5: DOWN x2 RIGHT x10 DOWN x2 RIGHT x5, face UP -> back at Guide Gent's house
        MoveNpc { npc_idx: 0, steps: vec![(Down, 2), (Right, 10), (Down, 2), (Right, 5)] },
        MovePlayer { steps: vec![(Down, 2), (Right, 10), (Down, 2), (Right, 5)] },
        TurnPlayer(Direction::Up),
        Pause(1.0),
        TurnNpc { npc_idx: 0, direction: Direction::Left },
        TurnPlayer(Direction::Right),
        ShowText("Here...\n\nIt's my house! Thanks for your company.\n\nLet me give you a small gift.".into()),
        WaitButton,
        // Give MAP CARD
        GiveItem { item_id: ITEM_MAP_CARD, count: 1 },
        SetEvent(EVENT_ENGINE_MAP_CARD),
        ShowText("<PLAYER>'s POKeGEAR now has a MAP!".into()),
        WaitButton,
        ShowText("POKeGEAR becomes more useful as you add CARDS.\n\nI wish you luck on your journey!".into()),
        WaitButton,
        CloseText,
        StopFollow,
        Special(SpecialFn::RestartMapMusic),
        TurnPlayer(Direction::Up),
        // Guide Gent walks into his house and disappears
        MoveNpc { npc_idx: 0, steps: vec![(Up, 2)] },
        HideNpc(0),
        ClearEvent(EVENT_GUIDE_GENT_VISIBLE_IN_CHERRYGROVE),
        End,
    ]
}
```

**Note**: The `Follow`/`StopFollow` are bookkeeping. Actual movement is via `MoveNpc`/`MovePlayer` pairs.

#### 2. Rival Silver Ambush (SCRIPT_RIVAL_SCENE_NORTH / SOUTH)

From pokecrystal `maps/CherrygroveCity.asm` `CherrygroveRivalSceneNorth` / `CherrygroveRivalSceneSouth`:

```rust
fn build_rival_scene_south() -> Vec<ScriptStep> {
    vec![
        // South variant: first move rival to (39, 7)
        MoveObject { npc_idx: 1, x: 39, y: 7 },
        // Then continue with same logic as north...
        TurnPlayer(Direction::Right),
        ShowEmote { npc_idx: 255, emote: Emote::Shock, frames: 15 }, // 255 = player emote
        Special(SpecialFn::FadeOutMusic),
        Pause(0.25),
        ShowNpc(1),  // make rival visible
        // Rival walks to player (5 steps left)
        MoveNpc { npc_idx: 1, steps: vec![(Left, 5)] },
        TurnPlayer(Direction::Right),
        PlayMusic(MUSIC_RIVAL_ENCOUNTER),
        ShowText("...... ......\n\nYou got a POKeMON at the LAB.\n\nWhat a waste. A wimp like you.\n\n...... ......\n\nDon't you get what I'm saying?\n\nWell, I too, have a good POKeMON.\n\nI'll show you what I mean!".into()),
        WaitButton,
        CloseText,
        // Determine counter-starter and battle
        // Implementation note: the script engine checks EVENT_GOT_TOTODILE_FROM_ELM
        // and EVENT_GOT_CHIKORITA_FROM_ELM to determine rival's Pokemon.
        // For Sprint 2, use a simplified approach: LoadWildMon with the counter-starter.
        // The mod.rs battle setup reads event flags to pick the right species.
        StartBattle { battle_type: BattleType::CanLose },
        // Post-battle (runs regardless of win/loss)
        PlayMusic(MUSIC_RIVAL_AFTER),
        // Check result -- both branches have similar text
        ShowText("...... ......\n\nMy name's ???.\n\nI'm going to be the world's greatest POKeMON trainer.".into()),
        WaitButton,
        CloseText,
        // Rival pushes player down and exits
        MovePlayer { steps: vec![(Down, 1)] },
        TurnPlayer(Direction::Left),
        MoveNpc { npc_idx: 1, steps: vec![(Left, 4), (Up, 2), (Left, 2)] },
        HideNpc(1),
        SetScene { map: MapId::CherrygroveCity, scene_id: 0 }, // SCENE_CHERRYGROVECITY_NOOP
        Special(SpecialFn::HealParty),
        PlayMapMusic,
        End,
    ]
}

fn build_rival_scene_north() -> Vec<ScriptStep> {
    // Same as south but without the initial MoveObject
    // (rival is already at correct position for north trigger)
    vec![
        TurnPlayer(Direction::Right),
        ShowEmote { npc_idx: 255, emote: Emote::Shock, frames: 15 },
        Special(SpecialFn::FadeOutMusic),
        Pause(0.25),
        ShowNpc(1),
        MoveNpc { npc_idx: 1, steps: vec![(Left, 5)] },
        TurnPlayer(Direction::Right),
        PlayMusic(MUSIC_RIVAL_ENCOUNTER),
        ShowText("...... ......\n\nYou got a POKeMON at the LAB.\n\nWhat a waste. A wimp like you.\n\n...... ......\n\nDon't you get what I'm saying?\n\nWell, I too, have a good POKeMON.\n\nI'll show you what I mean!".into()),
        WaitButton,
        CloseText,
        StartBattle { battle_type: BattleType::CanLose },
        PlayMusic(MUSIC_RIVAL_AFTER),
        ShowText("...... ......\n\nMy name's ???.\n\nI'm going to be the world's greatest POKeMON trainer.".into()),
        WaitButton,
        CloseText,
        MovePlayer { steps: vec![(Down, 1)] },
        TurnPlayer(Direction::Left),
        MoveNpc { npc_idx: 1, steps: vec![(Left, 4), (Up, 2), (Left, 2)] },
        HideNpc(1),
        SetScene { map: MapId::CherrygroveCity, scene_id: 0 },
        Special(SpecialFn::HealParty),
        PlayMapMusic,
        End,
    ]
}
```

**Rival's Pokemon determination** (in `mod.rs`, not in the script):
- If player chose Cyndaquil -> rival has Totodile L5
- If player chose Totodile -> rival has Chikorita L5
- If player chose Chikorita -> rival has Cyndaquil L5

This is checked via event flags (EVENT_GOT_CYNDAQUIL_FROM_ELM, EVENT_GOT_TOTODILE_FROM_ELM, EVENT_GOT_CHIKORITA_FROM_ELM) when `StartBattle` executes. The mod.rs handler calls a helper:

```rust
fn get_rival_species(&self) -> SpeciesId {
    if self.event_flags.has(EVENT_GOT_CYNDAQUIL_FROM_ELM) { TOTODILE }
    else if self.event_flags.has(EVENT_GOT_TOTODILE_FROM_ELM) { CHIKORITA }
    else { CYNDAQUIL }
}
```

#### 3. Catching Tutorial (SCRIPT_CATCHING_TUTORIAL_1)

From pokecrystal `maps/Route29.asm` `Route29Tutorial1`:

```rust
fn build_catching_tutorial_1() -> Vec<ScriptStep> {
    vec![
        // Dude (NPC idx 0) reacts
        TurnNpc { npc_idx: 0, direction: Direction::Up },
        ShowEmote { npc_idx: 0, emote: Emote::Shock, frames: 15 },
        // Dude walks to player: UP x4, RIGHT x2
        MoveNpc { npc_idx: 0, steps: vec![(Up, 4), (Right, 2)] },
        TurnPlayer(Direction::Left),
        SetEvent(EVENT_DUDE_TALKED_TO_YOU),
        ShowText("I've seen you a couple times. How many POKeMON have you caught?\n\nWould you like me to show you how to catch POKeMON?".into()),
        // YesNo: yes -> continue, no -> refuse
        YesNo { yes_jump: 9, no_jump: 14 },
        // (step 8 - unreachable, placeholder)
        Jump(9),
        // yes path (step 9):
        CloseText,
        Follow { npc_idx: 0 },
        // Dude walks to grass: LEFT x2, DOWN x4
        MoveNpc { npc_idx: 0, steps: vec![(Left, 2), (Down, 4)] },
        StopFollow,
        LoadWildMon { species: RATTATA, level: 5 },
        StartBattle { battle_type: BattleType::Tutorial },
        // After tutorial battle
        TurnNpc { npc_idx: 0, direction: Direction::Up },
        ShowText("That's how you do it.\n\nIf you weaken them first, POKeMON are easier to catch.".into()),
        WaitButton,
        CloseText,
        SetScene { map: MapId::Route29, scene_id: 0 }, // SCENE_ROUTE29_NOOP
        SetEvent(EVENT_LEARNED_TO_CATCH_POKEMON),
        End,
        // no path (step 14 from above -- adjust indices at build time):
        ShowText("Oh. Fine, then.\n\nAnyway, if you want to catch POKeMON, you have to walk a lot.".into()),
        WaitButton,
        CloseText,
        // Dude walks back: LEFT x2, DOWN x4
        MoveNpc { npc_idx: 0, steps: vec![(Left, 2), (Down, 4)] },
        SetScene { map: MapId::Route29, scene_id: 0 },
        End,
    ]
}
```

**Important**: The step indices for `YesNo { yes_jump, no_jump }` and `Jump` must be correct absolute indices into the Vec. The implementer must count carefully. The "no" path at step 14 means: `YesNo { yes_jump: 9, no_jump: 21 }` (approximately -- adjust based on actual vec length).

Tutorial 2 is nearly identical but with different movement data (DudeMovementData2a/2b in pokecrystal: UP x3, RIGHT x2 for approach; LEFT x2, DOWN x3 for retreat).

#### 4. Catching Tutorial Dude Regular Script (SCRIPT_CATCHING_TUTORIAL_DUDE)

Post-tutorial interaction script, from pokecrystal `CatchingTutorialDudeScript`:

```rust
fn build_catching_tutorial_dude_script() -> Vec<ScriptStep> {
    vec![
        FacingPlayer { npc_idx: 0 },
        // Check if already learned
        CheckEvent { flag: EVENT_LEARNED_TO_CATCH_POKEMON, jump_if_true: 5 },
        // Check if eligible (needs EVENT_GAVE_MYSTERY_EGG_TO_ELM)
        CheckEvent { flag: EVENT_GAVE_MYSTERY_EGG_TO_ELM, jump_if_true: 3 },
        // Not eligible yet
        ShowText("POKeMON hide in the grass. Who knows when they'll pop out...".into()),
        WaitButton, CloseText, End,
        // Eligible: offer repeat tutorial (step 3)
        ShowText("Huh? You want me to show you how to catch POKeMON?".into()),
        YesNo { yes_jump: 7, no_jump: 13 },
        Jump(7),
        // yes (step 7):
        CloseText,
        LoadWildMon { species: RATTATA, level: 5 },
        StartBattle { battle_type: BattleType::Tutorial },
        ShowText("That's how you do it.\n\nIf you weaken them first, POKeMON are easier to catch.".into()),
        WaitButton, CloseText,
        SetEvent(EVENT_LEARNED_TO_CATCH_POKEMON),
        End,
        // no (step 13):
        ShowText("Oh. Fine, then.\n\nAnyway, if you want to catch POKeMON, you have to walk a lot.".into()),
        WaitButton, CloseText, End,
        // already learned (step 5 from first check -- redirect)
        ShowText("POKeMON hide in the grass. Who knows when they'll pop out...".into()),
        WaitButton, CloseText, End,
    ]
}
```

#### 5. Nurse Joy Healer (SCRIPT_NURSE_JOY)

Standard PokecenterNurseScript:

```rust
fn build_nurse_joy_script() -> Vec<ScriptStep> {
    vec![
        FacingPlayer { npc_idx: 0 },
        ShowText("Welcome to our POKeMON CENTER.\n\nWe can heal your POKeMON to full health. Shall I heal your POKeMON?".into()),
        YesNo { yes_jump: 4, no_jump: 8 },
        Jump(4),
        // yes:
        CloseText,
        Heal,
        ShowText("Thank you for waiting. Your POKeMON are fully healed.\n\nWe hope to see you again!".into()),
        WaitButton, CloseText, End,
        // no (step 8):
        ShowText("We hope to see you again!".into()),
        WaitButton, CloseText, End,
    ]
}
```

#### 6. Mart Clerk (SCRIPT_MART_CLERK)

Sprint 2 stub -- no full mart UI:

```rust
fn build_mart_clerk_script() -> Vec<ScriptStep> {
    vec![
        FacingPlayer { npc_idx: 0 },
        // Check inventory phase
        CheckEvent { flag: EVENT_GAVE_MYSTERY_EGG_TO_ELM, jump_if_true: 4 },
        // Pre-egg inventory: Potion, Antidote, Parlyz Heal, Awakening
        ShowText("Welcome! How may I help you?\n\nPOTION - 300\nANTIDOTE - 100\nPARLYZ HEAL - 200\nAWAKENING - 250\n\n(Mart purchase not available yet)".into()),
        WaitButton, CloseText, End,
        // Post-egg inventory (step 4): adds Poke Ball
        ShowText("Welcome! How may I help you?\n\nPOKe BALL - 200\nPOTION - 300\nANTIDOTE - 100\nPARLYZ HEAL - 200\nAWAKENING - 250\n\n(Mart purchase not available yet)".into()),
        WaitButton, CloseText, End,
    ]
}
```

#### 7. Mystic Water Guy (SCRIPT_MYSTIC_WATER_GUY)

From pokecrystal `MysticWaterGuy`:

```rust
fn build_mystic_water_guy_script() -> Vec<ScriptStep> {
    vec![
        FacingPlayer { npc_idx: 4 },
        CheckEvent { flag: EVENT_GOT_MYSTIC_WATER_IN_CHERRYGROVE, jump_if_true: 7 },
        ShowText("A POKeMON I caught had an item.\n\nI think it's MYSTIC WATER.\n\nI don't need it, so do you want it?".into()),
        WaitButton,
        GiveItem { item_id: ITEM_MYSTIC_WATER, count: 1 },
        SetEvent(EVENT_GOT_MYSTIC_WATER_IN_CHERRYGROVE),
        // After giving
        ShowText("Back to fishing for me, then.".into()),
        WaitButton, CloseText, End,
        // Already got it (step 7):
        ShowText("Back to fishing for me, then.".into()),
        WaitButton, CloseText, End,
    ]
}
```

#### 8. Item Ball Potion (SCRIPT_ROUTE29_POTION)

Standard itemball script:

```rust
fn build_route29_potion_script() -> Vec<ScriptStep> {
    vec![
        GiveItem { item_id: ITEM_POTION, count: 1 },
        ShowText("<PLAYER> found POTION!".into()),
        WaitButton, CloseText,
        SetEvent(EVENT_ROUTE_29_POTION),
        HideNpc(7),  // NPC idx 7 is the item ball
        End,
    ]
}
```

#### 9. Fruit Tree (SCRIPT_ROUTE29_FRUIT_TREE)

Simplified for Sprint 2 (no daily cooldown system):

```rust
fn build_route29_fruit_tree_script() -> Vec<ScriptStep> {
    vec![
        GiveItem { item_id: ITEM_BERRY, count: 1 },
        ShowText("<PLAYER> found a BERRY!".into()),
        WaitButton, CloseText, End,
    ]
}
```

#### 10. Simple Dialogue NPCs

All simple NPC scripts follow the `jumptextfaceplayer` pattern:

```rust
// Route 29
SCRIPT_ROUTE29_YOUNGSTER => vec![
    FacingPlayer { npc_idx: 1 },
    ShowText("Yo. How are your POKeMON?\n\nIf they're weak and not ready for battle, keep out of the grass.".into()),
    WaitButton, CloseText, End,
],
SCRIPT_ROUTE29_TEACHER => vec![
    FacingPlayer { npc_idx: 2 },
    ShowText("See those ledges? It's scary to jump off them.\n\nBut you can go to NEW BARK without walking through the grass.".into()),
    WaitButton, CloseText, End,
],
SCRIPT_ROUTE29_FISHER => vec![
    FacingPlayer { npc_idx: 4 },
    ShowText("I wanted to take a break, so I saved to record my progress.".into()),
    WaitButton, CloseText, End,
],
SCRIPT_ROUTE29_COOLTRAINER_M => vec![
    FacingPlayer { npc_idx: 5 },
    // Time-of-day conditional text (simplified: always show day text for Sprint 2)
    ShowText("I'm waiting for POKeMON that appear only at night.".into()),
    WaitButton, CloseText, End,
],

// Gate
SCRIPT_GATE_OFFICER => vec![
    FacingPlayer { npc_idx: 0 },
    ShowText("You can't climb ledges.\n\nBut you can jump down from them to take a shortcut.".into()),
    WaitButton, CloseText, End,
],
SCRIPT_GATE_YOUNGSTER => vec![
    FacingPlayer { npc_idx: 1 },
    ShowText("Different kinds of POKeMON appear past here.\n\nIf you want to catch them all, you have to look everywhere.".into()),
    WaitButton, CloseText, End,
],

// Cherrygrove Teacher (conditional)
SCRIPT_CHERRYGROVE_TEACHER => vec![
    FacingPlayer { npc_idx: 2 },
    CheckEvent { flag: EVENT_ENGINE_MAP_CARD, jump_if_true: 4 },
    ShowText("Did you talk to the old man by the POKeMON CENTER?\n\nHe'll put a MAP of JOHTO on your POKeGEAR.".into()),
    WaitButton, CloseText, End,
    // Has Map Card (step 4):
    ShowText("When you're with POKeMON, going anywhere is fun.".into()),
    WaitButton, CloseText, End,
],

// Cherrygrove Youngster (conditional)
SCRIPT_CHERRYGROVE_YOUNGSTER => vec![
    FacingPlayer { npc_idx: 3 },
    CheckEvent { flag: EVENT_ENGINE_POKEDEX, jump_if_true: 4 },
    ShowText("MR.POKeMON's house is still farther up ahead.".into()),
    WaitButton, CloseText, End,
    // Has Pokedex (step 4):
    ShowText("I battled the trainers on the road.\n\nMy POKeMON lost. They're a mess! I must take them to a POKeMON CENTER.".into()),
    WaitButton, CloseText, End,
],

// Pokecenter NPCs
SCRIPT_POKECENTER_FISHER => vec![
    FacingPlayer { npc_idx: 1 },
    ShowText("It's great. I can store any number of POKeMON, and it's all free.".into()),
    WaitButton, CloseText, End,
],
SCRIPT_POKECENTER_GENTLEMAN => vec![
    FacingPlayer { npc_idx: 2 },
    ShowText("That PC is free for any trainer to use.".into()),
    WaitButton, CloseText, End,
],
SCRIPT_POKECENTER_TEACHER => vec![
    FacingPlayer { npc_idx: 3 },
    CheckEvent { flag: EVENT_GAVE_MYSTERY_EGG_TO_ELM, jump_if_true: 4 },
    ShowText("The COMMUNICATION CENTER upstairs was just built.\n\nBut they're still finishing it up.".into()),
    WaitButton, CloseText, End,
    ShowText("The COMMUNICATION CENTER upstairs was just built.\n\nI traded POKeMON there already!".into()),
    WaitButton, CloseText, End,
],

// Mart NPCs
SCRIPT_MART_COOLTRAINER_M => vec![
    FacingPlayer { npc_idx: 1 },
    CheckEvent { flag: EVENT_GAVE_MYSTERY_EGG_TO_ELM, jump_if_true: 4 },
    ShowText("They're fresh out of POKe BALLS!\n\nWhen will they get more of them?".into()),
    WaitButton, CloseText, End,
    ShowText("POKe BALLS are in stock! Now I can catch POKeMON!".into()),
    WaitButton, CloseText, End,
],
SCRIPT_MART_YOUNGSTER => vec![
    FacingPlayer { npc_idx: 2 },
    ShowText("When I was walking in the grass, a bug POKeMON poisoned my POKeMON!\n\nI just kept going, but then my POKeMON fainted.\n\nYou should keep an ANTIDOTE with you.".into()),
    WaitButton, CloseText, End,
],

// House NPCs
SCRIPT_GUIDE_GENT_HOUSE_GRAMPS => vec![
    FacingPlayer { npc_idx: 0 },
    ShowText("When I was a wee lad, I was a hot-shot trainer!\n\nHere's a word of advice: Catch lots of POKeMON!\n\nTreat them all with kindness!".into()),
    WaitButton, CloseText, End,
],
SCRIPT_GYM_SPEECH_POKEFAN => vec![
    FacingPlayer { npc_idx: 0 },
    ShowText("You're trying to see how good you are as a POKeMON trainer?\n\nYou better visit the POKeMON GYMS all over JOHTO and collect BADGES.".into()),
    WaitButton, CloseText, End,
],
SCRIPT_GYM_SPEECH_BUG_CATCHER => vec![
    FacingPlayer { npc_idx: 1 },
    ShowText("When I get older, I'm going to be a GYM LEADER!\n\nI make my POKeMON battle with my friend's to make them tougher!".into()),
    WaitButton, CloseText, End,
],
SCRIPT_EVO_SPEECH_LASS => vec![
    FacingPlayer { npc_idx: 0 },
    ShowText("POKeMON change?\n\nI would be shocked if one did that!".into()),
    WaitButton, CloseText, End,
],
SCRIPT_EVO_SPEECH_YOUNGSTER => vec![
    FacingPlayer { npc_idx: 1 },
    ShowText("POKeMON gain experience in battle and change their form.".into()),
    WaitButton, CloseText, End,
],
```

### Map Callback System

Add to `events.rs` or `mod.rs` (wherever `check_map_callbacks` lives):

```rust
fn check_map_callbacks(&mut self) {
    match self.current_map_id {
        MapId::CherrygroveCity => {
            // MAPCALLBACK_NEWMAP: always set flypoint
            self.event_flags.set(EVENT_ENGINE_FLYPOINT_CHERRYGROVE);
        }
        MapId::Route29 => {
            // MAPCALLBACK_OBJECTS: Tuscany visibility
            // Requires ENGINE_ZEPHYRBADGE AND it's Tuesday
            // Sprint 2: always hide Tuscany (no day-of-week system)
            if let Some(tuscany_idx) = self.find_npc_by_event_flag(EVENT_ROUTE_29_TUSCANY_OF_TUESDAY) {
                self.npc_states[tuscany_idx].visible = false;
            }
        }
        _ => {}
    }
}
```

### Dependencies

- `use super::data::{Direction, SpeciesId, Pokemon, Emote, PlayerState, NpcState};`
- `use super::maps::MapId;`

### Compile Check

After Phase 3: All new script_ids resolve via `get_script()`. ScriptState can be created for any Sprint 2 script. New ScriptStep variants compile.

---

## Phase 4: Battle System Foundation (`battle.rs`) -- NEW MODULE

**File**: `pokemonv2/battle.rs`
**Goal**: Minimal auto-battle system supporting wild encounters, catching tutorial, and rival CANLOSE battle.

### Imports

```rust
use super::data::{
    Pokemon, SpeciesId, MoveId, PokemonType,
    species_data, move_data, type_effectiveness,
};
```

### Core Types

```rust
use super::events::BattleType;

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
    Intro,          // "Wild PIDGEY appeared!"
    PlayerTurn,     // auto-select first damaging move
    EnemyTurn,      // enemy attacks
    Message,        // show damage text, wait for timer
    Victory,        // player wins
    Defeat,         // player loses
    Caught,         // tutorial auto-catch
    Flee,           // player ran (wild only)
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BattleResult {
    Won,
    Lost,
    Fled,
    Caught,
}
```

### Constants

```rust
/// Struggle -- used when a Pokemon has no damaging moves (e.g., Hoppip with only Splash).
pub const MOVE_STRUGGLE: MoveId = 165;

/// Message display time in seconds.
const MESSAGE_TIME: f64 = 1.5;

/// Max turns before auto-flee for wild battles.
const AUTO_FLEE_TURNS: u8 = 10;

/// Auto-catch turn for tutorial battles.
const TUTORIAL_CATCH_TURN: u8 = 3;
```

### Battle Creation

```rust
impl BattleState {
    /// Create a new battle against a wild Pokemon.
    pub fn new_wild(species: SpeciesId, level: u8, battle_type: BattleType) -> Self {
        let enemy = Pokemon::new(species, level);
        let name = species_data(species).name;
        Self {
            enemy,
            battle_type,
            turn_count: 0,
            phase: BattlePhase::Intro,
            message: Some(format!("Wild {} appeared!", name)),
            message_timer: MESSAGE_TIME,
            result: None,
        }
    }

    /// Create a new battle against a rival/trainer Pokemon.
    pub fn new_trainer(species: SpeciesId, level: u8, battle_type: BattleType) -> Self {
        let enemy = Pokemon::new(species, level);
        let name = species_data(species).name;
        Self {
            enemy,
            battle_type,
            turn_count: 0,
            phase: BattlePhase::Intro,
            message: Some(format!("RIVAL sent out {}!", name)),
            message_timer: MESSAGE_TIME,
            result: None,
        }
    }
}
```

### Damage Calculation

Gen 2 damage formula from pokecrystal:

```rust
/// Gen 2 damage formula.
/// damage = ((2 * level / 5 + 2) * power * atk / def) / 50 + 2
/// Apply STAB (x1.5), type effectiveness, random factor [217..255]/255.
pub fn calc_damage(
    attacker: &Pokemon,
    defender: &Pokemon,
    move_id: MoveId,
    rng_byte: u8,  // 217..255 range
) -> u16 {
    let mv = move_data(move_id);
    if mv.power == 0 {
        return 0; // status move does no damage
    }

    let level = attacker.level as f64;
    let power = mv.power as f64;

    // Physical vs Special split (Gen 2: type-based)
    let (atk, def) = if mv.is_special {
        (attacker.sp_attack as f64, defender.sp_defense as f64)
    } else {
        (attacker.attack as f64, defender.defense as f64)
    };

    // Base damage
    let mut damage = ((2.0 * level / 5.0 + 2.0) * power * atk / def) / 50.0 + 2.0;

    // STAB
    let atk_data = species_data(attacker.species);
    if mv.move_type == atk_data.type1 || mv.move_type == atk_data.type2 {
        damage *= 1.5;
    }

    // Type effectiveness (check both defender types)
    let def_data = species_data(defender.species);
    let eff1 = type_effectiveness(mv.move_type, def_data.type1);
    let eff2 = if def_data.type1 != def_data.type2 {
        type_effectiveness(mv.move_type, def_data.type2)
    } else {
        1.0
    };
    damage *= eff1 * eff2;

    // Random factor: [217..255]/255
    let random_factor = rng_byte.max(217) as f64 / 255.0;
    damage *= random_factor;

    damage.max(1.0) as u16
}
```

### Move Selection

```rust
/// Pick the first damaging move the Pokemon knows.
/// If no damaging move exists, return MOVE_STRUGGLE.
pub fn pick_damaging_move(pokemon: &Pokemon) -> MoveId {
    for slot in &pokemon.moves {
        if let Some(move_id) = slot {
            let mv = move_data(*move_id);
            if mv.power > 0 {
                return *move_id;
            }
        }
    }
    MOVE_STRUGGLE
}
```

### Auto-Battle Step Function

```rust
/// Advance the battle by one frame.
/// Returns true if battle is still running.
pub fn step_battle(
    battle: &mut BattleState,
    player_pokemon: &mut Pokemon,
    dt: f64,
    rng_byte: u8,
) -> bool {
    // Handle message timer
    if battle.message_timer > 0.0 {
        battle.message_timer -= dt;
        if battle.message_timer > 0.0 {
            return true; // still showing message
        }
    }

    match battle.phase {
        BattlePhase::Intro => {
            battle.turn_count += 1;
            battle.phase = BattlePhase::PlayerTurn;
            true
        }
        BattlePhase::PlayerTurn => {
            // Tutorial auto-catch
            if battle.battle_type == BattleType::Tutorial && battle.turn_count >= TUTORIAL_CATCH_TURN {
                battle.phase = BattlePhase::Caught;
                battle.message = Some("Gotcha! The POKeMON was caught!".into());
                battle.message_timer = MESSAGE_TIME;
                return true;
            }

            // Wild: auto-flee after too many turns
            if battle.battle_type == BattleType::Wild && battle.turn_count >= AUTO_FLEE_TURNS {
                battle.phase = BattlePhase::Flee;
                battle.message = Some("Got away safely!".into());
                battle.message_timer = MESSAGE_TIME;
                return true;
            }

            // Pick move and attack
            let move_id = pick_damaging_move(player_pokemon);
            let damage = calc_damage(player_pokemon, &battle.enemy, move_id, rng_byte);
            let move_name = move_data(move_id).name;
            let player_name = species_data(player_pokemon.species).name;

            battle.enemy.hp = battle.enemy.hp.saturating_sub(damage);
            battle.message = Some(format!("{} used {}! ({} damage)", player_name, move_name, damage));
            battle.message_timer = MESSAGE_TIME;

            if battle.enemy.hp == 0 {
                battle.phase = BattlePhase::Victory;
            } else {
                battle.phase = BattlePhase::EnemyTurn;
            }
            true
        }
        BattlePhase::EnemyTurn => {
            let move_id = pick_damaging_move(&battle.enemy);
            let damage = calc_damage(&battle.enemy, player_pokemon, move_id, rng_byte);
            let move_name = move_data(move_id).name;
            let enemy_name = species_data(battle.enemy.species).name;

            player_pokemon.hp = player_pokemon.hp.saturating_sub(damage);
            battle.message = Some(format!("{} used {}! ({} damage)", enemy_name, move_name, damage));
            battle.message_timer = MESSAGE_TIME;

            if player_pokemon.hp == 0 {
                battle.phase = BattlePhase::Defeat;
            } else {
                battle.turn_count += 1;
                battle.phase = BattlePhase::PlayerTurn;
            }
            true
        }
        BattlePhase::Victory => {
            let enemy_name = species_data(battle.enemy.species).name;
            battle.message = Some(format!("{} fainted!", enemy_name));
            battle.message_timer = MESSAGE_TIME;
            battle.result = Some(BattleResult::Won);
            false
        }
        BattlePhase::Defeat => {
            let player_name = species_data(player_pokemon.species).name;
            battle.message = Some(format!("{} fainted!", player_name));
            battle.message_timer = MESSAGE_TIME;
            battle.result = Some(BattleResult::Lost);
            false
        }
        BattlePhase::Caught => {
            battle.result = Some(BattleResult::Caught);
            false
        }
        BattlePhase::Flee => {
            battle.result = Some(BattleResult::Fled);
            false
        }
        BattlePhase::Message => {
            // Shouldn't reach here -- message_timer handles delays
            true
        }
    }
}
```

### Struggle Move Data

Add to `data.rs` move_data:

```rust
MOVE_STRUGGLE => &MoveData {
    id: 165, name: "Struggle",
    move_type: PokemonType::Normal, power: 50, accuracy: 100, pp: 1, is_special: false,
},
```

### Dependencies

- `use super::data::*;` (species/move data, Pokemon struct, type effectiveness)
- `use super::events::BattleType;`

### Compile Check

After Phase 4: `BattleState::new_wild(PIDGEY, 2, BattleType::Wild)` compiles. `step_battle()` can run a battle to completion. `calc_damage()` produces correct values for Tackle (power 35, Normal type) vs Pidgey.

---

## Phase 5: Overworld Systems (`overworld.rs`)

**File**: `pokemonv2/overworld.rs`
**Goal**: Ledge movement, wild encounter checks, map connection transitions.

### New OverworldResult Variants

Add to existing enum:

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

### Direction-Aware Walkability

Replace `is_walkable` with a direction-aware version:

```rust
/// Check if tile at (x, y) is walkable when approached from the given direction.
/// Ledge tiles are only walkable if the player is facing the ledge's direction.
pub fn is_walkable_with_direction(map: &MapData, x: i32, y: i32, facing: Direction) -> bool {
    if x < 0 || y < 0 || x >= map.width as i32 || y >= map.height as i32 {
        return false;
    }
    let idx = (y as usize) * (map.width as usize) + (x as usize);
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

Keep the old `is_walkable` function as a non-direction wrapper for backward compatibility:

```rust
pub fn is_walkable(map: &MapData, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 || x >= map.width as i32 || y >= map.height as i32 { return false; }
    let idx = (y as usize) * (map.width as usize) + (x as usize);
    let c = map.collision[idx];
    c == C_FLOOR || c == C_WARP || c == C_GRASS
}
```

### Wild Encounter Check

Add after walk-complete, BEFORE checking warps:

```rust
/// Check if the player stepped on grass and should trigger a wild encounter.
/// Called after walk completion when the player's new tile is C_GRASS.
pub fn check_wild_encounter(
    map: &MapData,
    x: i32,
    y: i32,
    time_of_day: TimeOfDay,
    rng_encounter: u8,  // 0..255 random byte for encounter check
    rng_slot: u8,        // 0..255 random byte for slot selection
) -> Option<(SpeciesId, u8)> {
    let idx = (y as usize) * (map.width as usize) + (x as usize);
    if map.collision[idx] != C_GRASS {
        return None;
    }

    if let Some(ref table) = map.wild_encounters {
        // Encounter check: encounter_rate out of 255
        // pokecrystal "10 percent" roughly means 10/255 chance per step
        if rng_encounter >= table.encounter_rate {
            return None;
        }

        // Select slot based on time of day
        let slots = match time_of_day {
            TimeOfDay::Morning => &table.morning,
            TimeOfDay::Day => &table.day,
            TimeOfDay::Night => &table.night,
        };

        if slots.is_empty() {
            return None;
        }

        // Slot selection using pokecrystal probability distribution:
        // Slot 0: 30% (0..76)
        // Slot 1: 30% (76..153)
        // Slot 2: 20% (153..204)
        // Slot 3: 10% (204..229)
        // Slot 4:  5% (229..242)
        // Slot 5:  2.5% (242..248)
        // Slot 6:  2.5% (248..255)
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

### Map Edge Detection

Modify `step_overworld` to check map connections when the target tile is out of bounds:

```rust
// In step_overworld, where movement is blocked by out-of-bounds:
if tx < 0 || ty < 0 || tx >= map.width as i32 || ty >= map.height as i32 {
    // Check for map connection in the player's facing direction
    let conn = match player.facing {
        Direction::Left => &map.connections.west,
        Direction::Right => &map.connections.east,
        Direction::Up => &map.connections.north,
        Direction::Down => &map.connections.south,
    };
    if let Some(connection) = conn {
        return OverworldResult::MapConnection {
            direction: player.facing,
            dest_map: connection.dest_map,
            offset: connection.offset,
        };
    }
    // No connection -- player can't move
}
```

### Walk-Complete Handler Update

Update the walk-complete section in `step_overworld`:

```rust
// After walk completes (walk_offset >= TILE_PX):
// 1. Snap to destination tile
player.x += dx;
player.y += dy;
player.walk_offset = 0.0;
player.is_walking = false;

// 2. Check grass encounter (BEFORE warps)
// Requires two random bytes from engine RNG
let rng_enc = /* get byte from engine rng */;
let rng_slot = /* get byte from engine rng */;
if let Some((species, level)) = check_wild_encounter(map, player.x, player.y, time_of_day, rng_enc, rng_slot) {
    return OverworldResult::WildEncounter { species, level };
}

// 3. Check coord events
if let Some(ce) = find_coord_event(map, player.x, player.y, current_scene_id) {
    return OverworldResult::TriggerCoordEvent { script_id: ce.script_id };
}

// 4. Check warps
if let Some(warp) = find_warp(map, player.x, player.y) {
    return OverworldResult::WarpTo { dest_map: warp.dest_map, dest_warp_id: warp.dest_warp_id };
}
```

### Dependencies

- Existing imports plus: `use super::data::TimeOfDay;`
- `use super::maps::{C_GRASS, C_LEDGE_D, C_LEDGE_L, C_LEDGE_R};`

### Compile Check

After Phase 5: `step_overworld` returns `WildEncounter` when player steps on C_GRASS and RNG triggers. `MapConnection` returned when player walks off map edge toward a connected map. `is_walkable_with_direction` correctly blocks/allows ledge tiles.

---

## Phase 6: Main Module Updates (`mod.rs`)

**File**: `pokemonv2/mod.rs`
**Goal**: Wire battle phase, map connection handler, wild encounter handler. Update GamePhase.

### New GamePhase Variant

```rust
pub enum GamePhase {
    // ... existing variants ...
    Battle,  // NEW: auto-battle in progress
}
```

### New Fields on PokemonV2Sim

```rust
pub struct PokemonV2Sim {
    // ... existing fields ...
    pub battle: Option<BattleState>,  // NEW: active battle state
}
```

### Handle New OverworldResult Variants

In the `step()` method where overworld results are processed:

```rust
OverworldResult::WildEncounter { species, level } => {
    self.start_wild_battle(species, level);
}
OverworldResult::MapConnection { direction, dest_map, offset } => {
    self.handle_map_connection(direction, dest_map, offset);
}
```

### start_wild_battle

```rust
fn start_wild_battle(&mut self, species: SpeciesId, level: u8) {
    let battle = BattleState::new_wild(species, level, BattleType::Wild);
    self.battle = Some(battle);
    self.phase = GamePhase::Battle;
}
```

### handle_map_connection

Sprint 2 uses fade transition (simplified, not seamless scrolling):

```rust
fn handle_map_connection(&mut self, direction: Direction, dest_map: MapId, offset: i8) {
    self.current_map_id = dest_map;
    self.current_map = load_map(dest_map);
    self.npc_states = init_npc_states(&self.current_map);
    self.temp_flags = 0;

    // Place player at the connection edge of the new map
    match direction {
        Direction::Left => {
            self.player.x = self.current_map.width as i32 - 1;
            self.player.y = self.player.y + offset as i32;
        }
        Direction::Right => {
            self.player.x = 0;
            self.player.y = self.player.y + offset as i32;
        }
        Direction::Up => {
            self.player.y = self.current_map.height as i32 - 1;
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
```

### Battle Phase in step()

```rust
GamePhase::Battle => {
    if let Some(ref mut battle) = self.battle {
        let rng_byte = /* get from engine rng */;
        if let Some(ref mut pokemon) = self.party.first_mut() {
            let still_running = step_battle(battle, pokemon, FIXED_DT, rng_byte);
            if !still_running {
                if let Some(result) = battle.result {
                    self.battle = None;
                    match result {
                        BattleResult::Lost if self.current_battle_type() != Some(BattleType::CanLose) => {
                            // Whiteout: heal party, teleport to last Pokecenter
                            self.heal_party();
                            // For Sprint 2: warp to CherrygrovePokecenter1F if visited, else ElmsLab
                            self.warp_to_last_pokecenter();
                        }
                        _ => {
                            // Resume overworld (or script for scripted battles)
                            self.phase = if self.active_script.is_some() {
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
}
```

### ScriptStep Execution for New Variants

In `step_script` (events.rs), handle new ScriptStep variants:

```rust
ScriptStep::LoadWildMon { species, level } => {
    // Store species/level for next StartBattle
    // The mod.rs handler creates the BattleState
    script.loaded_wild_species = Some((*species, *level));
    script.pc += 1;
}
ScriptStep::StartBattle { battle_type } => {
    // Signal mod.rs to create battle
    // Return a special result from step_script
    script.pc += 1;
    return ScriptResult::StartBattle {
        battle_type: *battle_type,
        species: script.loaded_wild_species,
    };
}
ScriptStep::Follow { npc_idx } => {
    // Sprint 2 no-op: just advance pc
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
    // Resume map music (no-op for Sprint 2 -- no audio system)
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

### mod.rs Module Declaration

Add battle.rs to the module:

```rust
mod battle;
mod data;
mod dialogue;
mod events;
mod maps;
mod overworld;
mod render;
mod sprites;
```

### Dependencies

All existing plus: `use battle::*;`

### Compile Check

After Phase 6: Full game loop compiles. Walking on grass can trigger wild battles. Walking off map edges transitions to connected maps. Rival battle script creates a CANLOSE battle. Tutorial creates a TUTORIAL battle.

---

## Phase 7: Render Pipeline Updates (`render.rs`)

**File**: `pokemonv2/render.rs`
**Goal**: Grass tile rendering, ledge tile rendering, battle screen, new sprite colors.

### Grass Tile Rendering

In the tile rendering loop, add C_GRASS handling:

```rust
if map.collision[idx] == C_GRASS {
    // Base grass -- slightly darker green
    fill_rect(engine, sx, sy, TILE_PX, TILE_PX, Color::from_rgba(56, 120, 48, 255));
    // Grass tufts -- lighter green dashes for texture
    fill_rect(engine, sx + 2, sy + 6, 4, 2, Color::from_rgba(80, 160, 64, 255));
    fill_rect(engine, sx + 10, sy + 10, 4, 2, Color::from_rgba(80, 160, 64, 255));
}
```

### Ledge Tile Rendering

```rust
if map.collision[idx] == C_LEDGE_D {
    // Same as floor but with shadow line on south edge
    fill_rect(engine, sx, sy, TILE_PX, TILE_PX, Color::from_rgba(72, 144, 72, 255));
    // Shadow line (2px tall, dark)
    fill_rect(engine, sx, sy + TILE_PX - 2, TILE_PX, 2, Color::from_rgba(40, 80, 40, 255));
}
```

### Battle Screen

Add `GamePhase::Battle` to the render dispatch:

```rust
GamePhase::Battle => render_battle(sim, engine),
```

Minimal battle render:

```rust
fn render_battle(sim: &PokemonV2Sim, engine: &mut Engine) {
    // Clear to white (battle background)
    fill_rect(engine, 0, 0, SCREEN_W, SCREEN_H, Color::from_rgba(248, 248, 248, 255));

    if let Some(ref battle) = sim.battle {
        if let Some(ref player_mon) = sim.party.first() {
            let player_data = species_data(player_mon.species);
            let enemy_data = species_data(battle.enemy.species);

            // Enemy info (top-right area)
            draw_text(engine, 8, 8, enemy_data.name, Color::from_rgba(0, 0, 0, 255));
            draw_text(engine, 8, 18, &format!("Lv{}", battle.enemy.level), Color::from_rgba(0, 0, 0, 255));
            // Enemy HP bar
            let enemy_hp_pct = battle.enemy.hp as f64 / battle.enemy.max_hp as f64;
            draw_hp_bar(engine, 8, 28, enemy_hp_pct);

            // Player info (bottom-left area)
            let py = SCREEN_H - 50;
            draw_text(engine, 80, py, player_data.name, Color::from_rgba(0, 0, 0, 255));
            draw_text(engine, 80, py + 10, &format!("Lv{}", player_mon.level), Color::from_rgba(0, 0, 0, 255));
            draw_text(engine, 80, py + 20, &format!("HP {}/{}", player_mon.hp, player_mon.max_hp), Color::from_rgba(0, 0, 0, 255));
            let player_hp_pct = player_mon.hp as f64 / player_mon.max_hp as f64;
            draw_hp_bar(engine, 80, py + 30, player_hp_pct);
        }

        // Battle message at bottom
        if let Some(ref msg) = battle.message {
            draw_text_box(engine, msg);
        }
    }
}

fn draw_hp_bar(engine: &mut Engine, x: i32, y: i32, pct: f64) {
    let bar_w = 60;
    let bar_h = 4;
    // Background
    fill_rect(engine, x, y, bar_w, bar_h, Color::from_rgba(64, 64, 64, 255));
    // Fill (green > yellow > red based on %)
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

### New Sprite IDs

Add to `sprites.rs`:

```rust
pub const SPRITE_GRAMPS: u8 = 14;
pub const SPRITE_NURSE: u8 = 15;
pub const SPRITE_CLERK: u8 = 16;
pub const SPRITE_GENTLEMAN: u8 = 17;
pub const SPRITE_YOUNGSTER: u8 = 18;
pub const SPRITE_COOLTRAINER_M: u8 = 19;
pub const SPRITE_POKEFAN_M: u8 = 20;
pub const SPRITE_BUG_CATCHER: u8 = 21;
pub const SPRITE_LASS: u8 = 22;
pub const SPRITE_FRUIT_TREE: u8 = 23;
pub const SPRITE_ITEM_BALL: u8 = 24;
```

Add corresponding color palettes in `draw_sprite` for each new sprite ID.

### Dependencies

Existing plus: `use super::battle::BattleState;`

### Compile Check

After Phase 7: Full render pipeline compiles. Battle screen draws HP bars and messages. Grass and ledge tiles render with visual distinction.

---

## Phase 8: Tests

**File**: Test module in `mod.rs` or a separate test file.
**Goal**: Verify all Sprint 2 systems work correctly.

### Test Specifications

#### Map Tests

```rust
#[test]
fn test_route29_dimensions() {
    let map = load_map(MapId::Route29);
    assert_eq!(map.width, 60);
    assert_eq!(map.height, 18);
    assert_eq!(map.tiles.len(), 60 * 18);
    assert_eq!(map.collision.len(), 60 * 18);
}

#[test]
fn test_cherrygrove_city_dimensions() {
    let map = load_map(MapId::CherrygroveCity);
    assert_eq!(map.width, 40);
    assert_eq!(map.height, 18);
}

#[test]
fn test_all_sprint2_maps_load() {
    let maps = [
        MapId::Route29, MapId::Route29Route46Gate,
        MapId::CherrygroveCity, MapId::CherrygrovePokecenter1F,
        MapId::CherrygroveMart, MapId::GuideGentsHouse,
        MapId::CherrygroveGymSpeechHouse, MapId::CherrygroveEvolutionSpeechHouse,
        MapId::Route46,
    ];
    for id in &maps {
        let map = load_map(*id);
        assert!(map.width > 0 && map.height > 0);
        assert_eq!(map.tiles.len(), map.width as usize * map.height as usize);
    }
}
```

#### Warp Bidirectionality Tests

```rust
#[test]
fn test_warp_bidirectional_cherrygrove() {
    // For each warp in CherrygroveCity, verify the destination map has a warp back
    let city = load_map(MapId::CherrygroveCity);
    for (i, warp) in city.warps.iter().enumerate() {
        let dest = load_map(warp.dest_map);
        let return_warp = &dest.warps[warp.dest_warp_id as usize];
        assert_eq!(return_warp.dest_map, MapId::CherrygroveCity,
            "Warp {} in CherrygroveCity does not return correctly", i);
    }
}
```

#### Map Connection Tests

```rust
#[test]
fn test_map_connections_route29() {
    let map = load_map(MapId::Route29);
    assert!(map.connections.east.is_some());
    assert_eq!(map.connections.east.as_ref().unwrap().dest_map, MapId::NewBarkTown);
    assert!(map.connections.west.is_some());
    assert_eq!(map.connections.west.as_ref().unwrap().dest_map, MapId::CherrygroveCity);
    assert!(map.connections.north.is_some());
    assert_eq!(map.connections.north.as_ref().unwrap().dest_map, MapId::Route46);
}

#[test]
fn test_map_connections_bidirectional() {
    // Route29 connects east to NBT; NBT connects west to Route29
    let r29 = load_map(MapId::Route29);
    let nbt = load_map(MapId::NewBarkTown);
    assert_eq!(r29.connections.east.as_ref().unwrap().dest_map, MapId::NewBarkTown);
    assert_eq!(nbt.connections.west.as_ref().unwrap().dest_map, MapId::Route29);
}
```

#### Wild Encounter Tests

```rust
#[test]
fn test_wild_encounter_route29_has_table() {
    let map = load_map(MapId::Route29);
    assert!(map.wild_encounters.is_some());
    let table = map.wild_encounters.as_ref().unwrap();
    assert_eq!(table.morning.len(), 7);
    assert_eq!(table.day.len(), 7);
    assert_eq!(table.night.len(), 7);
    assert_eq!(table.encounter_rate, 10);
}

#[test]
fn test_wild_encounter_morning_species() {
    let map = load_map(MapId::Route29);
    let table = map.wild_encounters.as_ref().unwrap();
    assert_eq!(table.morning[0].species, PIDGEY);
    assert_eq!(table.morning[0].level, 2);
    assert_eq!(table.morning[1].species, SENTRET);
}

#[test]
fn test_wild_encounter_night_species() {
    let map = load_map(MapId::Route29);
    let table = map.wild_encounters.as_ref().unwrap();
    assert_eq!(table.night[0].species, HOOTHOOT);
    assert_eq!(table.night[1].species, RATTATA);
}

#[test]
fn test_no_wild_encounters_in_city() {
    let map = load_map(MapId::CherrygroveCity);
    assert!(map.wild_encounters.is_none());
}
```

#### Ledge Movement Tests

```rust
#[test]
fn test_ledge_walkable_correct_direction() {
    // Create a small test map with a ledge tile
    let mut collision = vec![C_FLOOR; 9]; // 3x3
    collision[4] = C_LEDGE_D; // center tile is south-facing ledge
    let map = /* construct test MapData with this collision, width=3, height=3 */;
    // Can walk onto ledge from the north (facing down)
    assert!(is_walkable_with_direction(&map, 1, 1, Direction::Down));
    // Cannot walk onto ledge from the south (facing up)
    assert!(!is_walkable_with_direction(&map, 1, 1, Direction::Up));
    // Cannot walk onto ledge from the side
    assert!(!is_walkable_with_direction(&map, 1, 1, Direction::Left));
    assert!(!is_walkable_with_direction(&map, 1, 1, Direction::Right));
}
```

#### Battle Tests

```rust
#[test]
fn test_battle_creation() {
    let battle = BattleState::new_wild(PIDGEY, 2, BattleType::Wild);
    assert_eq!(battle.phase, BattlePhase::Intro);
    assert_eq!(battle.enemy.species, PIDGEY);
    assert_eq!(battle.enemy.level, 2);
    assert!(battle.result.is_none());
}

#[test]
fn test_battle_damage_calc() {
    let attacker = Pokemon::new(CYNDAQUIL, 5);
    let defender = Pokemon::new(PIDGEY, 2);
    let damage = calc_damage(&attacker, &defender, MOVE_TACKLE, 230);
    assert!(damage > 0);
    assert!(damage < 100); // sanity check
}

#[test]
fn test_battle_auto_resolves() {
    let mut battle = BattleState::new_wild(PIDGEY, 2, BattleType::Wild);
    let mut player_mon = Pokemon::new(CYNDAQUIL, 5);
    // Run battle to completion
    for _ in 0..100 {
        if !step_battle(&mut battle, &mut player_mon, 1.0 / 60.0, 230) {
            break;
        }
    }
    assert!(battle.result.is_some());
}

#[test]
fn test_battle_canlose_no_gameover() {
    let mut battle = BattleState::new_trainer(TOTODILE, 5, BattleType::CanLose);
    // Set player HP to 1 so they lose quickly
    let mut player_mon = Pokemon::new(CYNDAQUIL, 5);
    player_mon.hp = 1;
    for _ in 0..100 {
        if !step_battle(&mut battle, &mut player_mon, 1.0 / 60.0, 230) {
            break;
        }
    }
    assert_eq!(battle.result, Some(BattleResult::Lost));
    // CANLOSE means no game-over -- the caller (mod.rs) handles this
}

#[test]
fn test_tutorial_auto_catches() {
    let mut battle = BattleState::new_wild(RATTATA, 5, BattleType::Tutorial);
    let mut player_mon = Pokemon::new(CYNDAQUIL, 5);
    for _ in 0..200 {
        if !step_battle(&mut battle, &mut player_mon, 1.0 / 60.0, 230) {
            break;
        }
    }
    assert_eq!(battle.result, Some(BattleResult::Caught));
}

#[test]
fn test_hoppip_uses_struggle() {
    // Hoppip at level 2 only knows Splash (power 0)
    let hoppip = Pokemon::new(HOPPIP, 2);
    let move_id = pick_damaging_move(&hoppip);
    assert_eq!(move_id, MOVE_STRUGGLE);
}
```

#### Script Tests

```rust
#[test]
fn test_guide_gent_gives_map_card() {
    // Create a sim, place player in CherrygroveCity near Guide Gent
    // Run the guide gent tour script to completion
    // Verify EVENT_ENGINE_MAP_CARD is set
    // Verify ITEM_MAP_CARD is in bag
}

#[test]
fn test_mystic_water_sets_flag() {
    // Trigger Mystic Water Guy script
    // Verify EVENT_GOT_MYSTIC_WATER_IN_CHERRYGROVE is set
    // Verify ITEM_MYSTIC_WATER is in bag
}

#[test]
fn test_rival_battle_sets_scene_noop() {
    // After rival battle, scene should be SCENE_CHERRYGROVECITY_NOOP (0)
}

#[test]
fn test_potion_item_ball_hides_npc() {
    // After picking up Route 29 Potion, EVENT_ROUTE_29_POTION is set
    // NPC idx 7 should be hidden
}
```

---

## Critical Design Decisions (from Gandalf's Architecture)

### 1. Map Connections: Fade vs Seamless
**Decision: Use fade transition for Sprint 2.** Seamless scrolling requires rendering two maps simultaneously with camera offset math. Fade transitions reuse existing `MapTransition` infrastructure. Refine to seamless in a later sprint.

### 2. Battle System Scope
**Decision: Auto-battle only for Sprint 2.** No move selection UI, no bag, no switching, no run menu. Player's Pokemon auto-uses first damaging move. Enemy does the same. Sufficient for catching tutorial (auto-catch after 3 turns), rival fight (auto-battle), and wild encounters (auto-fight or auto-flee after 10 turns).

### 3. Mart System
**Decision: Stub mart as dialogue for Sprint 2.** The full mart UI (cursor, money, inventory management) is a menu system. For Sprint 2: interacting with the Clerk shows the price list as text. Full mart UI deferred to a later sprint.

### 4. Follow Mechanic
**Decision: Simplified follow for Sprint 2.** The `Follow`/`StopFollow` script steps exist but are no-ops. Actual follow behavior is simulated by matching `MoveNpc` and `MovePlayer` movement steps.

### 5. Time-of-Day Source
**Decision: Use `total_time` modulo 24 hours.** 1 real second = 1 game minute. `get_time_of_day()` in data.rs computes Morning/Day/Night from total elapsed time. Test harness can override by controlling total_time directly.

### 6. Route 29 Stub Replacement
**Decision: Completely replace the Sprint 1 Route29 stub.** The old 20x20 featureless map is removed. The new 60x18 map from pokecrystal data replaces it entirely.

---

## What NOT to Build in Sprint 2

- Full battle UI (move selection, bag, switch, run menu)
- Pokeball catching mechanics (except tutorial auto-catch)
- PC storage system
- Phone/Pokegear functionality beyond Map Card
- Day-of-week system (Tuscany stays hidden)
- Surf/water traversal
- Headbutt tree encounters
- Fly menu
- Full mart purchase/sell UI
- Experience/leveling from battles
- PokéCenter 2F (stairs locked)
- Seamless map scrolling (use fade transitions)

---

## Implementation Order Summary

| Phase | Module | % of Sprint | Key Deliverables |
|-------|--------|------------|-----------------|
| 1 | data.rs | 10% | 5 species, 4+ moves, TimeOfDay, items |
| 2 | maps.rs | 25% | 8 maps with tiles/collision/warps/NPCs/encounters |
| 3 | events.rs | 20% | 15+ flags, 6 script variants, ~20 scripts |
| 4 | battle.rs | 15% | BattleState, damage calc, auto-battle loop |
| 5 | overworld.rs | 10% | Ledges, encounters, map connections |
| 6 | mod.rs | 10% | Battle phase, connection handler, script handlers |
| 7 | render.rs + sprites.rs | 5% | Grass/ledge tiles, battle screen, new sprites |
| 8 | Tests | 5% | 15+ tests covering all new systems |
