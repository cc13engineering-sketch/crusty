# Sprint 2 Architecture Proposal

Route 29, Route 29/46 Gate, Cherrygrove City, and all Cherrygrove interior buildings.

---

## Summary of New Systems Required

Sprint 2 introduces 7 new systems and expands 3 existing ones. Below is the priority order, from "must be fully implemented" to "can be stubbed."

### Must Implement Fully
1. **Wild encounters** — Route 29 is unplayable without them
2. **Map connections** — New Bark Town <-> Route 29 <-> Cherrygrove must be seamless outdoor transitions
3. **Time-of-day system** — Route 29 encounter tables vary by morning/day/night
4. **Ledge tiles** — Route 29 has one-way jump ledges
5. **New species/moves data** — Pidgey, Sentret, Rattata, Hoothoot, Hoppip + their movesets
6. **Item system expansion** — Mystic Water gift, Potion item ball, fruit tree berry, Map Card
7. **PokéCenter healing template** — Nurse Joy healer script
8. **Mart template** — Clerk shopkeeper with phase-based inventory
9. **Rival battle** — BATTLETYPE_CANLOSE in Cherrygrove (battle foundation)
10. **Catching tutorial** — Scripted wild encounter (loadwildmon + catchtutorial)

### Can Be Stubbed
11. **Fishing/water encounters** — Cherrygrove coastal water. Stub: water tiles impassable, no Surf
12. **Headbutt trees** — TREEMON_SET_ROUTE. Stub: no-op interaction
13. **Day-of-week system** — Tuscany Tuesday-only. Stub: never visible (requires ENGINE_ZEPHYRBADGE too)
14. **Flypoint system** — ENGINE_FLYPOINT_CHERRYGROVE. Stub: set flag but no fly menu yet

---

## Module Changes

### `data.rs` — New Species, Moves, Items

**New species constants and data** (7 species needed for Route 29 encounters + rival battle):

```rust
// Route 29 wild encounters
pub const PIDGEY: SpeciesId = 16;
pub const RATTATA: SpeciesId = 19;
pub const SENTRET: SpeciesId = 161;
pub const HOOTHOOT: SpeciesId = 163;
pub const HOPPIP: SpeciesId = 187;

// Rival's Pokemon (counter-starter, Level 5)
// Chikorita (152), Cyndaquil (155), Totodile (158) already defined

// Catching tutorial
// Rattata (19) already covered above
```

**New species data structs** — each with base stats and learnset from pokecrystal `base_stats/`. At Level 2-3 they only know their level-1 moves:

| Species | Types | Base HP/Atk/Def/Spd/SpA/SpD | Level 1 Moves |
|---------|-------|------------------------------|---------------|
| Pidgey | Normal/Flying | 40/45/40/56/35/35 | Tackle |
| Rattata | Normal | 30/56/35/72/25/35 | Tackle, Tail Whip |
| Sentret | Normal | 35/46/34/20/35/45 | Tackle, Defense Curl |
| Hoothoot | Normal/Flying | 60/30/30/50/36/56 | Tackle, Growl |
| Hoppip | Grass/Flying | 35/35/40/50/35/55 | Splash, Synthesis |

**New move data** — Tail Whip, Defense Curl, Splash, Synthesis (status moves, power=0).

**New item constants**:
```rust
pub const ITEM_MYSTIC_WATER: u8 = 41;  // +20% Water power
pub const ITEM_PINK_BOW: u8 = 42;      // +10% Normal power
pub const ITEM_MAP_CARD: u8 = 43;      // Key item
pub const ITEM_POKE_BALL: u8 = 4;      // For mart inventory
pub const ITEM_ANTIDOTE: u8 = 18;
pub const ITEM_PARLYZ_HEAL: u8 = 19;
```

**New enum variant for TimeOfDay**:
```rust
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TimeOfDay {
    Morning,  // 04:00 - 09:59
    Day,      // 10:00 - 17:59
    Night,    // 18:00 - 03:59
}
```

This goes in `data.rs` since it's referenced by both maps (encounter tables) and overworld (tint).

### `maps.rs` — New Maps, Collision Types, Encounter System

**New MapId variants**:
```rust
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
    Route46,  // stub — gate target only
}
```

**New collision type**:
```rust
pub const C_GRASS: u8 = 5;  // walkable, triggers wild encounter check
pub const C_LEDGE_D: u8 = 6;  // one-way: can jump south, can't climb north
pub const C_LEDGE_L: u8 = 7;  // one-way: can jump left (future)
pub const C_LEDGE_R: u8 = 8;  // one-way: can jump right (future)
```

Ledge mechanic: player can move onto a `C_LEDGE_D` tile from the north (jumping down) but NOT from the south, east, or west. The overworld movement handler checks: if destination tile is a ledge tile, only allow if the player's facing matches the ledge direction. After stepping onto a ledge tile, the player "hops" — add a small vertical offset bounce animation in the walk interpolation.

**Revamped WildEncounter for time-of-day**:
```rust
pub struct WildEncounterTable {
    pub morning: Vec<WildSlot>,
    pub day: Vec<WildSlot>,
    pub night: Vec<WildSlot>,
    pub encounter_rate: u8,  // pokecrystal "X percent" — higher = more frequent
}

pub struct WildSlot {
    pub species: SpeciesId,
    pub level: u8,
}
```

Route 29 from `johto_grass.asm`:
- Morning/Day: Pidgey L2, Sentret L2, Pidgey L3, Sentret L3, Rattata L2, Hoppip L3, Hoppip L3
- Night: Hoothoot L2, Rattata L2, Hoothoot L3, Rattata L3, Rattata L2, Hoothoot L3, Hoothoot L3
- encounter_rate: 10 percent (all three periods)

The 7-slot structure maps directly to pokecrystal's format. Slot selection follows the original probability distribution: slots 0-1 are 30% each, slot 2 is 20%, slot 3 is 10%, slot 4 is 5%, slots 5-6 are 2.5% each. We can simplify to a flat random pick for Sprint 2 and refine probabilities later.

**Map data for new maps** — dimensions from pokecrystal map headers:

| Map | Block Size | Tile Size | Warps | NPCs | Notes |
|-----|-----------|-----------|-------|------|-------|
| Route29 | 30x9 | 60x18 | 1 (gate at 27,1) | 8 (Dude, Youngster, Teacher, FruitTree, Fisher, CooltrainerM, Tuscany, Potion ball) | Grass tiles, ledges, connections E->NBT, W->Cherrygrove |
| Route29Route46Gate | 4x4 | 8x8 | 4 (2 north to Route46, 2 south to Route29) | 2 (Officer, Youngster) | NorthSouthGate tileset |
| CherrygroveCity | 20x9 | 40x18 | 5 (Mart, Pokecenter, GymSpeech, GuideGent, EvoSpeech) | 5 (Guide Gent, Rival, Teacher, Youngster, Fisher) | Connections N->Route30, E->Route29 |
| CherrygrovePokecenter1F | 5x4 | 10x8 | 3 (2 exit, 1 stairs) | 4 (Nurse, Fisher, Gentleman, Teacher) | Shared template |
| CherrygroveMart | 6x4 | 12x8 | 2 (exit) | 3 (Clerk, CooltrainerM, Youngster) | Shared template |
| GuideGentsHouse | 4x4 | 8x8 | 2 (exit) | 1 (Gramps NPC if Guide Gent returned home) | |
| CherrygroveGymSpeechHouse | 4x4 | 8x8 | 2 (exit) | 2 (Pokefan_M, Bug Catcher) | |
| CherrygroveEvolutionSpeechHouse | 4x4 | 8x8 | 2 (exit) | 2 (Lass, Youngster) | |

**Map connections** — the key Sprint 2 architecture addition. New Bark Town's existing west connection to Route29 becomes real (not a stub). Route 29 connects east to New Bark Town, west to Cherrygrove. Cherrygrove connects east to Route 29, north to Route 30 (stub).

**Critical architectural decision**: Map connections are NOT warps. They're seamless outdoor-to-outdoor transitions. When the player walks off the left edge of New Bark Town, the game smoothly scrolls into Route 29 without a fade-to-black. Sprint 1 already has `MapConnections` in `MapData` but the `step_overworld` function doesn't handle edge-walking. Sprint 2 must implement this.

Connection handling in `overworld.rs`:
1. When player walks toward a map edge and `is_walkable` returns false (out of bounds):
2. Check if the current map has a connection in that direction
3. If yes: initiate a map connection transition — swap `current_map`, adjust player coordinates by the offset, load new NPC states
4. Unlike warp transitions, there is NO fade-to-black. The camera just keeps scrolling.

For Sprint 2, we can use a simplified approach: trigger a `MapTransition` like warps do (with a brief fade), then refine to seamless scrolling in a later sprint. This avoids the complexity of rendering two maps simultaneously.

### `events.rs` — New Event Flags, Scene Constants, Scripts

**New event flag constants**:
```rust
// Sprint 2 event flags (starting after Sprint 1's highest: 16)
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
pub const EVENT_GAVE_MYSTERY_EGG_TO_ELM: u16 = 29; // future, but needed for mart phase check
pub const EVENT_ENGINE_POKEDEX: u16 = 30;
pub const EVENT_ENGINE_ZEPHYRBADGE: u16 = 31;
```

**New scene constants**:
```rust
// Route 29 scenes
pub const SCENE_ROUTE29_NOOP: u8 = 0;
pub const SCENE_ROUTE29_CATCH_TUTORIAL: u8 = 1;

// Cherrygrove scenes
pub const SCENE_CHERRYGROVECITY_NOOP: u8 = 0;
pub const SCENE_CHERRYGROVECITY_MEET_RIVAL: u8 = 1;
```

**New ScriptStep variants needed**:
```rust
pub enum ScriptStep {
    // ... existing variants ...

    // Sprint 2 additions:
    /// Start a wild battle (scripted encounter, e.g., catching tutorial)
    LoadWildMon { species: SpeciesId, level: u8 },
    /// Start a trainer/rival battle
    StartBattle { battle_type: BattleType },
    /// Follow another NPC (player walks behind them)
    Follow { npc_idx: u8 },
    StopFollow,
    /// Change player position without animation (moveobject equivalent)
    MoveObject { npc_idx: u8, x: i32, y: i32 },
    /// Play map music (resume from special music)
    PlayMapMusic,
    /// Special game functions
    Special(SpecialFn),
    /// Check if a flag is set, conditional text branching
    CheckFlag { flag: u16, jump_if_true: usize },
    /// Warp to another map from script
    ScriptWarp { dest_map: MapId, dest_warp_id: u8 },
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BattleType {
    Wild,
    Tutorial,      // BATTLETYPE_TUTORIAL — catching demo
    CanLose,       // BATTLETYPE_CANLOSE — rival, no game-over
    Normal,        // Standard trainer battle
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SpecialFn {
    HealParty,
    RestartMapMusic,
    FadeOutMusic,
}
```

Note: `CheckEvent` already exists as `CheckEvent { flag, jump_if_true }`. The new `CheckFlag` is identical — we'll just use the existing `CheckEvent`.

**Major new scripts**:
1. **Guide Gent city tour** (~50 steps): scripted follow movement across Cherrygrove, 5 stops with dialogue, gives Map Card, clears EVENT_GUIDE_GENT_VISIBLE_IN_CHERRYGROVE
2. **Rival Silver ambush** (~30 steps): appear, walk to player, dialogue, BATTLETYPE_CANLOSE battle, post-battle exit movement, HealParty
3. **Catching tutorial** (~20 steps): Dude approaches, dialogue, yes/no, follow movement, loadwildmon Rattata L5, catchtutorial
4. **Nurse Joy healer** (~5 steps): heal party, dialogue
5. **Mart Clerk** (~3 steps): check EVENT_GAVE_MYSTERY_EGG_TO_ELM flag, open appropriate mart inventory
6. **Mystic Water Fisher** (~5 steps): check EVENT_GOT_MYSTIC_WATER, give item, set flag
7. **Item ball Potion** (~3 steps): give item, set EVENT_ROUTE_29_POTION, hide NPC
8. **Fruit tree** (~3 steps): give berry, set daily cooldown (simplify to event flag for Sprint 2)

### `overworld.rs` — Encounter Checks, Ledge Movement, Map Connections

**Wild encounter check** — added to the walk-complete handler:

```rust
// After walk complete, BEFORE checking warps:
if map.collision[idx] == C_GRASS {
    if should_trigger_encounter(encounter_rate, rng) {
        let slot = pick_encounter_slot(wild_table, time_of_day, rng);
        return OverworldResult::WildEncounter { species: slot.species, level: slot.level };
    }
}
```

New `OverworldResult` variant:
```rust
pub enum OverworldResult {
    Nothing,
    WarpTo { dest_map: MapId, dest_warp_id: u8 },
    TriggerScript { script_id: u16, npc_idx: Option<u8> },
    TriggerCoordEvent { script_id: u16 },
    // Sprint 2:
    WildEncounter { species: SpeciesId, level: u8 },
    MapConnection { direction: Direction, dest_map: MapId, offset: i8 },
}
```

**Ledge movement** — modify `is_walkable` to handle ledge tiles:

```rust
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
```

**Map edge walking** — When the player's target tile is out of bounds, check connections:

```rust
if !in_bounds(map, tx, ty) {
    if let Some(conn) = get_connection_for_direction(map, player.facing) {
        return OverworldResult::MapConnection {
            direction: player.facing,
            dest_map: conn.dest_map,
            offset: conn.offset,
        };
    }
    // No connection — player can't move
}
```

### `mod.rs` — Battle Phase, Map Connection Handler, Encounter Handler

**New handler in `step_overworld_phase`**:
```rust
OverworldResult::WildEncounter { species, level } => {
    // Create wild Pokemon, enter battle phase
    self.start_wild_battle(species, level);
}
OverworldResult::MapConnection { direction, dest_map, offset } => {
    self.handle_map_connection(direction, dest_map, offset);
}
```

**`handle_map_connection`** — Sprint 2 simplified version (fade transition):
```rust
fn handle_map_connection(&mut self, direction: Direction, dest_map: MapId, offset: i8) {
    self.current_map_id = dest_map;
    self.current_map = load_map(dest_map);
    self.npc_states = init_npc_states(&self.current_map);
    self.temp_flags = 0;

    // Place player at the connection edge of the new map
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
```

**Map callbacks** — new concept from pokecrystal. When entering a map, certain callbacks fire:
- `MAPCALLBACK_NEWMAP`: fires on every map entry (e.g., set flypoint)
- `MAPCALLBACK_OBJECTS`: fires to conditionally show/hide objects (e.g., Tuscany)

Add a simple `check_map_callbacks()` method to `PokemonV2Sim` that handles per-map entry logic:
```rust
fn check_map_callbacks(&mut self) {
    match self.current_map_id {
        MapId::CherrygroveCity => {
            self.event_flags.set(EVENT_ENGINE_FLYPOINT_CHERRYGROVE);
        }
        MapId::Route29 => {
            // Tuscany: visible only on Tuesday AND after Zephyr Badge
            // For Sprint 2: always hidden (no day-of-week system yet)
        }
        _ => {}
    }
}
```

### `render.rs` — Grass Tile Animation, Battle Screen Stub

**Grass tile rendering** — C_GRASS tiles get a darker green tint and a subtle pattern overlay so the player can distinguish walkable grass from decorative grass:

```rust
// In tile rendering:
if map.collision[idx] == C_GRASS {
    // Draw base grass
    fill_rect(engine, sx, sy, TILE_PX, TILE_PX, Color::from_rgba(56, 120, 48, 255));
    // Draw grass tufts (lighter green dashes)
    fill_rect(engine, sx + 2, sy + 6, 4, 2, Color::from_rgba(80, 160, 64, 255));
    fill_rect(engine, sx + 10, sy + 10, 4, 2, Color::from_rgba(80, 160, 64, 255));
}
```

**Ledge tile rendering** — ledge tiles get a shadow line on their south edge to indicate the drop.

**Battle screen** — minimal stub for Sprint 2:
- Show player's Pokemon name/HP bar on bottom-left
- Show enemy Pokemon name/HP bar on top-right
- Auto-battle: alternate Tackle exchanges until one faints
- For BATTLETYPE_CANLOSE: return to overworld regardless of outcome
- For BATTLETYPE_TUTORIAL: auto-catch after a few turns

### `sprites.rs` — New Sprite IDs

```rust
pub const SPRITE_GRAMPS: u8 = 14;        // Guide Gent
pub const SPRITE_NURSE: u8 = 15;          // Nurse Joy
pub const SPRITE_CLERK: u8 = 16;          // Mart clerk
pub const SPRITE_GENTLEMAN: u8 = 17;      // Pokecenter Gentleman
pub const SPRITE_YOUNGSTER: u8 = 18;      // Various Youngsters
pub const SPRITE_COOLTRAINER_M: u8 = 19;  // CooltrainerM / Dude
pub const SPRITE_POKEFAN_M: u8 = 20;      // Pokefan M
pub const SPRITE_BUG_CATCHER: u8 = 21;    // Bug Catcher
pub const SPRITE_LASS: u8 = 22;           // Lass
pub const SPRITE_FRUIT_TREE: u8 = 23;     // Fruit tree object
pub const SPRITE_ITEM_BALL: u8 = 24;      // Ground item (Potion)
```

---

## Battle System Foundation

Sprint 2 needs a minimal battle system. It must handle:
1. **Wild encounters** on Route 29 grass
2. **Catching tutorial** — scripted Rattata L5 encounter
3. **Rival battle** — BATTLETYPE_CANLOSE Silver with counter-starter L5

### Minimal Battle Architecture

Do NOT build the full battle system. Build the minimum:

```rust
// In mod.rs (or a new battle.rs module)
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
    PlayerTurn,     // auto-select Tackle/Scratch
    EnemyTurn,      // enemy attacks
    Message,        // show damage text
    Victory,        // player wins
    Defeat,         // player loses (only matters for non-CANLOSE)
    Caught,         // tutorial catch
    Flee,           // player ran away
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BattleResult {
    Won,
    Lost,
    Fled,
    Caught,
}
```

**Auto-battle for Sprint 2**: The battle runs automatically. Each turn, both sides use their first damaging move. Simple damage calc:
```
damage = ((2 * level / 5 + 2) * power * atk / def) / 50 + 2
```
Apply STAB (x1.5 if move type matches attacker type) and type effectiveness. Random factor: multiply by `[217..255]/255` (Gen 2 formula).

**Battle flow in `step()`**:
```rust
GamePhase::Battle => {
    if let Some(ref mut battle) = self.battle {
        battle.step(engine, &mut self.party);
        if let Some(result) = battle.result {
            self.battle = None;
            self.phase = GamePhase::Overworld; // or Script for scripted battles
            match result {
                BattleResult::Lost if battle.battle_type != BattleType::CanLose => {
                    // Whiteout — teleport to last PokéCenter
                }
                _ => {} // resume overworld
            }
        }
    }
}
```

For rival battle specifically: after battle, the script continues (rival exits, party healed via HealParty special).

### Decision: battle.rs as new module

Create `battle.rs` as a new submodule. Place it in the import graph:
```
data <- events <- maps <- overworld <- battle <- render
```
Wait — that's wrong. Battle needs Pokemon from data but doesn't need maps or overworld. Better:
```
data <- battle (battle reads species/move data from data.rs)
data <- events (events can trigger battles via ScriptStep)
```

Battle is a mostly-independent module that receives a `Pokemon` (player's) and creates an enemy `Pokemon`, then runs a turn-based loop. The `mod.rs` orchestrates: when overworld/script signals a battle, `mod.rs` creates the `BattleState` and switches to `GamePhase::Battle`.

---

## Import Graph (Updated)

```
data.rs          (leaf — no sibling imports)
  |
  +-- events.rs  (imports data, maps::MapId)
  +-- maps.rs    (imports data::{Direction, NpcState, SpeciesId})
  +-- battle.rs  (imports data::{Pokemon, SpeciesId, MoveId, species_data, move_data, type_effectiveness})
  +-- dialogue.rs (leaf — no imports)
  +-- sprites.rs (imports data::{Direction, Emote})
  |
  +-- overworld.rs (imports data, maps, events::{EventFlags, SceneState})
  |
  +-- render.rs  (imports data, maps, overworld::constants, events, sprites, battle)
  |
  +-- mod.rs     (imports everything)
```

---

## Implementation Order

### Phase 1: Data Layer (~25% of sprint)
1. Add new species data (Pidgey, Rattata, Sentret, Hoothoot, Hoppip) + their moves to `data.rs`
2. Add TimeOfDay enum, new item constants
3. Add new collision constants (C_GRASS, C_LEDGE_D) to `maps.rs`
4. Add WildEncounterTable struct to `maps.rs`
5. Add new MapId variants
6. Add new event flag and scene constants to `events.rs`
7. Add new sprite IDs to `sprites.rs`

### Phase 2: Map Data (~25% of sprint)
1. Replace Route29 stub with full 60x18 map (grass tiles, ledge tiles, building wall tiles, path tiles)
2. Build Route29Route46Gate (8x8 gate interior)
3. Build CherrygroveCity (40x18 outdoor)
4. Build CherrygrovePokecenter1F (10x8)
5. Build CherrygroveMart (12x8)
6. Build 3 generic houses (8x8 each)
7. Wire all warps bidirectionally
8. Wire all map connections (NBT<->R29<->Cherrygrove)
9. Add all NPCs with correct positions, sprites, movement types, event flags

### Phase 3: Overworld Systems (~20% of sprint)
1. Implement `is_walkable_with_direction` for ledge tile support
2. Implement map edge detection -> MapConnection handling in `step_overworld`
3. Implement `handle_map_connection` in `mod.rs`
4. Implement grass encounter check in walk-complete handler
5. Implement `should_trigger_encounter` (simple RNG probability)
6. Implement `pick_encounter_slot` (random slot from time-of-day table)
7. Implement `check_map_callbacks` for NEWMAP callbacks

### Phase 4: Battle Foundation (~15% of sprint)
1. Create `battle.rs` with BattleState, BattlePhase
2. Implement minimal auto-battle loop (Tackle exchanges)
3. Implement damage calculation (Gen 2 formula)
4. Implement BattleResult -> return to overworld
5. Wire GamePhase::Battle in mod.rs step() and render()
6. Add minimal battle render (HP bars, messages)

### Phase 5: Scripts & Events (~10% of sprint)
1. Build Guide Gent tour script (follow movement, 5 dialogue stops, Map Card)
2. Build rival ambush script (appear, battle, exit, HealParty)
3. Build catching tutorial script (approach, follow, loadwildmon, catchtutorial)
4. Build Nurse Joy healer script
5. Build Mart Clerk script (with flag-based inventory phase)
6. Build Mystic Water gift script
7. Build item ball Potion script
8. Build all NPC dialogue scripts (signs, NPCs)

### Phase 6: Tests (~5% of sprint)
1. Test all Sprint 2 maps load with correct dimensions
2. Test warp bidirectional consistency for all new maps
3. Test map connections (Route29 <-> NBT, Route29 <-> Cherrygrove)
4. Test wild encounter triggers on grass tiles
5. Test ledge movement (can jump south, blocked north)
6. Test battle auto-resolution
7. Test Guide Gent tour gives Map Card and sets flags
8. Test rival battle CANLOSE doesn't game-over
9. Test Mystic Water gift sets flag

---

## Critical Design Decisions

### 1. Map connections: fade vs seamless
**Decision: Use fade transition for Sprint 2.** Seamless scrolling requires rendering two maps simultaneously with camera offset math — significant render.rs complexity. Fade transitions use the existing MapTransition infrastructure. Refine to seamless later.

### 2. Battle system scope
**Decision: Auto-battle only for Sprint 2.** No move selection UI, no bag in battle, no switching. Player's Pokemon auto-uses first damaging move. Enemy does the same. This is sufficient for the catching tutorial (auto-catch), rival fight (auto-battle to conclusion), and wild encounters (auto-fight or auto-flee after 1 turn).

### 3. Mart system
**Decision: Stub mart as dialogue for Sprint 2.** The full mart UI (cursor, money deduction, inventory management) is a menu system. For Sprint 2: interacting with the Clerk shows "POTION - 300" text, gives a free Potion, and says "Thank you!" This is wrong vs the real game but gets the script working. Full mart UI comes in a later sprint.

### 4. Follow mechanic
**Decision: Simplified follow for Sprint 2.** In pokecrystal, `follow` makes the player trail behind an NPC step-for-step. For Sprint 2: use `MovePlayer` with matching steps after `MoveNpc`. The visual result is similar enough. True follow requires pathfinding logic.

### 5. Time-of-day source
**Decision: Use `total_time` modulo 24 hours as simulated game clock.** Map `total_time` to a time-of-day:
```rust
fn get_time_of_day(total_time: f64) -> TimeOfDay {
    // 1 real second = 1 game minute (sped up)
    let game_minutes = (total_time * 60.0) as u32 % (24 * 60);
    let hour = game_minutes / 60;
    match hour {
        4..=9 => TimeOfDay::Morning,
        10..=17 => TimeOfDay::Day,
        _ => TimeOfDay::Night,
    }
}
```
Or allow the test harness to set time_of_day directly. For simulation determinism, time_of_day should be derivable from total_time and a configurable speed multiplier.

### 6. Route29 stub replacement
**Decision: Completely replace `build_stub_route` for Route29.** The current Route29 is a featureless 20x20 flat map. Replace with the full 60x18 map from pokecrystal data, including: grass patches, ledge strips, path tiles, building walls (for gate), NPC placements, warp to gate, connection points.

---

## What NOT to Build in Sprint 2

- Full battle UI (move selection, bag, switch, run menu)
- Pokeball catching mechanics (except tutorial auto-catch)
- PC storage system
- Phone/Pokegear functionality
- Day-of-week system (Tuscany stays hidden)
- Surf/water traversal
- Headbutt tree encounters
- Fly menu
- Full mart purchase/sell UI
- Experience/leveling from battles (battles are auto and don't grant XP yet)
- PokéCenter 2F (stairs locked, as per pokecrystal)
