# Sprint 4 Architecture: Route 30 + Mr. Pokemon's House

Sprint 4 adds Route 30 (full outdoor route with 3 trainers), Route 30 Berry House,
Mr. Pokemon's House (Mystery Egg + Pokedex from Oak), and the trainer battle system.

---

## Source of Truth

All data comes from `pokecrystal-master/`:
- `maps/Route30.asm` -- NPCs, warps, bg_events, object_events
- `maps/Route30BerryHouse.asm` -- Berry House NPC + events
- `maps/MrPokemonsHouse.asm` -- Mr. Pokemon + Oak scene chain
- `data/wild/johto_grass.asm` lines 1265-1291 -- Route 30 encounter tables
- `data/trainers/parties.asm` -- Joey (Rattata/4), Mikey (Pidgey/2+Rattata/4), Don (Caterpie/3 x2)
- `constants/map_constants.asm` line 491 -- Route 30 is 10x27 metatiles = 20x54 tiles
- `data/maps/attributes.asm` lines 163-165 -- connections: north->Route31(-10), south->CherrygroveCity(-5)

---

## New Maps

### Route 30 (20 x 54)

**pokecrystal dimensions**: 10 metatiles wide x 27 metatiles tall = 20 tiles x 54 tiles.

**Connections** (from `attributes.asm`):
- North -> Route31 (offset: -10) [stub for now]
- South -> CherrygroveCity (offset: -5)

**Warps** (from `Route30_MapEvents`):
- `warp_event 7, 39, ROUTE_30_BERRY_HOUSE, 1` -- Berry House door
- `warp_event 17, 5, MR_POKEMONS_HOUSE, 1` -- Mr. Pokemon's House door

**NPCs** (11 object_events from Route30.asm):

| Idx | Name | Sprite | Position | MoveType | Type | Event Flag | Notes |
|-----|------|--------|----------|----------|------|------------|-------|
| 0 | YOUNGSTER1 (pre-battle cutscene Joey) | YOUNGSTER | (5,26) | StandingUp | Script | EVENT_ROUTE_30_BATTLE (hide after) | ImportantBattleScript |
| 1 | YOUNGSTER2 (Joey, trainer) | YOUNGSTER | (2,28) | StandingRight | Trainer(range=3) | EVENT_ROUTE_30_YOUNGSTER_JOEY | TrainerYoungsterJoey |
| 2 | YOUNGSTER3 (Mikey, trainer) | YOUNGSTER | (5,23) | StandingDown | Trainer(range=1) | -1 (always visible) | TrainerYoungsterMikey |
| 3 | BUG_CATCHER (Don, trainer) | BUG_CATCHER | (1,7) | StandingDown | Trainer(range=3) | -1 (always visible) | TrainerBugCatcherDon |
| 4 | YOUNGSTER4 (directions NPC) | YOUNGSTER | (7,30) | WalkLeftRight | Script | -1 | Route30YoungsterScript |
| 5 | MONSTER1 (Joey's Rattata) | MONSTER | (5,24) | StandingDown | Script | EVENT_ROUTE_30_BATTLE | cutscene sprite |
| 6 | MONSTER2 (Mikey's Rattata) | MONSTER | (5,25) | StandingUp | Script | EVENT_ROUTE_30_BATTLE | cutscene sprite |
| 7 | FRUIT_TREE1 | FRUIT_TREE | (5,39) | Still | Script | -1 | FruitTree |
| 8 | FRUIT_TREE2 | FRUIT_TREE | (11,5) | Still | Script | -1 | FruitTree |
| 9 | COOLTRAINER_F | COOLTRAINER_F | (2,13) | StandingDown | Script | -1 | generic dialogue |
| 10 | POKE_BALL (Antidote) | POKE_BALL | (8,35) | Still | ItemBall | EVENT_ROUTE_30_ANTIDOTE | Antidote item |

**BG Events** (from Route30_MapEvents):
- `(9, 43)` -- Route 30 sign
- `(13, 29)` -- MrPokemonsHouseDirectionsSign
- `(15, 5)` -- MrPokemonsHouseSign
- `(3, 21)` -- TrainerTips sign
- `(14, 9)` -- hidden Potion (BGEVENT_ITEM, EVENT_ROUTE_30_HIDDEN_POTION)

**Wild Encounters** (from johto_grass.asm):
```
encounter_rate: 10 (all periods)

morning: [Ledyba/3, Caterpie/3, Caterpie/4, Pidgey/4, Weedle/3, Hoppip/4, Hoppip/4]
day:     [Pidgey/3, Caterpie/3, Caterpie/4, Pidgey/4, Weedle/3, Hoppip/4, Hoppip/4]
night:   [Spinarak/3, Hoothoot/3, Poliwag/4, Hoothoot/4, Zubat/3, Hoothoot/4, Hoothoot/4]
```

**Terrain features**:
- Grass patches throughout (encounter zones)
- Trees/walls forming the route corridor
- No ledges on Route 30 (unlike Route 29)
- Water on east side (impassable, no Surf yet)

### Route 30 Berry House (8 x 8)

Standard house template (same as GuideGentsHouse pattern).

**Warps**:
- `(2,7)` and `(3,7)` -> Route30, warp 1

**NPCs** (1):
- POKEFAN_M at (2,3), StandingDown -- gives Berry on first visit (EVENT_GOT_BERRY_FROM_ROUTE_30_HOUSE)

**BG Events** (2):
- `(0,1)` and `(1,1)` -- bookshelf

### Mr. Pokemon's House (8 x 8)

Uses TILESET_FACILITY, not TILESET_HOUSE. Same physical dimensions as a house though.

**Warps**:
- `(2,7)` and `(3,7)` -> Route30, warp 2

**NPCs** (2):
- MR_POKEMON (GENTLEMAN) at (3,5), StandingRight -- always visible
- OAK at (6,5), StandingUp -- visible only when EVENT_MR_POKEMONS_HOUSE_OAK is NOT set (disappears after giving Pokedex)

**Scene Scripts** (2 scenes):
- SCENE_MRPOKEMONSHOUSE_MEET_MR_POKEMON (0) -- triggers full cutscene on entry
- SCENE_MRPOKEMONSHOUSE_NOOP (1) -- no auto-script

**BG Events** (5):
- `(0,1)` and `(1,1)` -- foreign magazines
- `(6,1)` and `(7,1)` -- broken computer
- `(6,4)` -- strange coins

**Mr. Pokemon Cutscene** (SCENE 0, triggered on map entry):
1. Mr. Pokemon emote SHOCK, turns DOWN
2. Text: intro + egg explanation
3. Player walks RIGHT, UP to Mr. Pokemon
4. `giveitem MYSTERY_EGG` -> set EVENT_GOT_MYSTERY_EGG_FROM_MR_POKEMON
5. `blackoutmod CHERRYGROVE_CITY` (sets respawn point)
6. Mr. Pokemon introduces Oak
7. Oak script chain begins

**Oak Script Chain**:
1. Play MUSIC_PROF_OAK
2. Oak walks DOWN, LEFT, LEFT to player
3. Player turns RIGHT
4. Long dialogue about Pokedex
5. `setflag ENGINE_POKEDEX`
6. Oak exits (walks DOWN, LEFT, disappears)
7. `special RestartMapMusic`
8. Mr. Pokemon offers to heal party
9. `special HealParty`
10. Set scene flags:
    - `setevent EVENT_RIVAL_NEW_BARK_TOWN` (show rival in New Bark on return)
    - `setevent EVENT_PLAYERS_HOUSE_1F_NEIGHBOR`
    - `clearevent EVENT_PLAYERS_NEIGHBORS_HOUSE_NEIGHBOR`
    - `setscene SCENE_MRPOKEMONSHOUSE_NOOP`
    - `setmapscene CHERRYGROVE_CITY, SCENE_CHERRYGROVECITY_MEET_RIVAL`
    - `setmapscene ELMS_LAB, SCENE_ELMSLAB_MEET_OFFICER`
    - `specialphonecall SPECIALCALL_ROBBED` (Elm calls about robbery)
    - `clearevent EVENT_COP_IN_ELMS_LAB`
    - Set rival's stolen starter pokeball flag (counter-pick logic)

---

## Module Changes

### `data.rs` -- New Species + Items

**New species** needed for Route 30 encounters:
```rust
pub const CATERPIE: SpeciesId = 10;
pub const METAPOD: SpeciesId = 11;
pub const WEEDLE: SpeciesId = 13;
pub const ZUBAT: SpeciesId = 41;
pub const POLIWAG: SpeciesId = 60;
pub const LEDYBA: SpeciesId = 165;
pub const SPINARAK: SpeciesId = 167;
```

Each needs a `SpeciesData` static with base stats and learnset. Source: `pokecrystal-master/data/pokemon/base_stats/`.

**New items**:
```rust
pub const ITEM_ANTIDOTE: u8 = 9;    // already defined
pub const ITEM_MYSTERY_EGG: u8 = 130; // key item, not consumed
```

**New move** (for Caterpie's String Shot, Weedle's Poison Sting, etc.):
```rust
pub const MOVE_STRING_SHOT: MoveId = 81;
pub const MOVE_POISON_STING: MoveId = 40;
pub const MOVE_HARDEN: MoveId = 106;
pub const MOVE_CONFUSION: MoveId = 93;   // Ledyba at higher levels
pub const MOVE_SUPERSONIC: MoveId = 48;
pub const MOVE_LEECH_LIFE: MoveId = 141;
pub const MOVE_CONSTRICT: MoveId = 132;  // Spinarak
```

Note: At levels 3-4, most of these Pokemon only know 1-2 moves. Check base_stats for exact learnsets.

### `maps.rs` -- New MapId Variants + Builders

**New MapId variants**:
```rust
// Add to MapId enum:
Route30BerryHouse,
MrPokemonsHouse,
Route31,   // stub (connection target)
```

Note: Route30 and Route31 stubs already exist in MapId.

**New builder functions**:
- `build_route30()` -- 20x54, grass patches, 2 warps, 11 NPCs, 5 bg_events, wild encounters, connections
- `build_route30_berry_house()` -- 8x8, standard house template
- `build_mr_pokemons_house()` -- 8x8, 2 NPCs (Oak conditional), 5 bg_events
- Update `build_route30_stub()` -> full `build_route30()` (replace the existing stub)

### `events.rs` -- New Event Flags + Scripts + Trainer System

**New event flags**:
```rust
pub const EVENT_BEAT_YOUNGSTER_JOEY: u16 = 32;
pub const EVENT_BEAT_YOUNGSTER_MIKEY: u16 = 33;
pub const EVENT_BEAT_BUG_CATCHER_DON: u16 = 34;
pub const EVENT_ROUTE_30_BATTLE: u16 = 35;      // pre-battle cutscene
pub const EVENT_ROUTE_30_YOUNGSTER_JOEY: u16 = 36;
pub const EVENT_ROUTE_30_ANTIDOTE: u16 = 37;
pub const EVENT_ROUTE_30_HIDDEN_POTION: u16 = 38;
pub const EVENT_GOT_BERRY_FROM_ROUTE_30_HOUSE: u16 = 39;
pub const EVENT_GOT_MYSTERY_EGG_FROM_MR_POKEMON: u16 = 40;
pub const EVENT_MR_POKEMONS_HOUSE_OAK: u16 = 41;
pub const EVENT_JOEY_ASKED_FOR_PHONE_NUMBER: u16 = 42;
```

**New scene constants**:
```rust
pub const SCENE_MRPOKEMONSHOUSE_MEET_MR_POKEMON: u8 = 0;
pub const SCENE_MRPOKEMONSHOUSE_NOOP: u8 = 1;
```

**New scripts** (~15 new script IDs):
```rust
// Route 30 scripts
pub const SCRIPT_JOEY_PREBATTLE_CUTSCENE: u16 = 300;
pub const SCRIPT_TRAINER_JOEY: u16 = 301;
pub const SCRIPT_TRAINER_MIKEY: u16 = 302;
pub const SCRIPT_TRAINER_DON: u16 = 303;
pub const SCRIPT_ROUTE30_YOUNGSTER_DIRECTIONS: u16 = 304;
pub const SCRIPT_ROUTE30_COOLTRAINER_F: u16 = 305;
pub const SCRIPT_ROUTE30_SIGN: u16 = 306;
pub const SCRIPT_MR_POKEMON_HOUSE_DIRECTIONS_SIGN: u16 = 307;
pub const SCRIPT_MR_POKEMON_HOUSE_SIGN: u16 = 308;
pub const SCRIPT_ROUTE30_TRAINER_TIPS: u16 = 309;
pub const SCRIPT_ROUTE30_ANTIDOTE: u16 = 310;
pub const SCRIPT_ROUTE30_FRUIT_TREE_1: u16 = 311;
pub const SCRIPT_ROUTE30_FRUIT_TREE_2: u16 = 312;

// Berry House scripts
pub const SCRIPT_BERRY_HOUSE_POKEFAN: u16 = 320;
pub const SCRIPT_BERRY_HOUSE_BOOKSHELF: u16 = 321;

// Mr. Pokemon's House scripts
pub const SCRIPT_MR_POKEMON: u16 = 330;
pub const SCRIPT_MR_POKEMON_MEET: u16 = 331;  // scene 0 entry script
pub const SCRIPT_MR_POKEMON_MAGAZINES: u16 = 332;
pub const SCRIPT_MR_POKEMON_COMPUTER: u16 = 333;
pub const SCRIPT_MR_POKEMON_COINS: u16 = 334;
```

### Trainer Battle System (New Concept)

Sprint 2 only had the rival CanLose battle. Sprint 4 introduces proper trainer battles with:

1. **Trainer sight range** -- trainers detect the player within `range` tiles in their facing direction
2. **Pre-battle sequence** -- trainer walks to player, "!" emote, dialogue, then battle
3. **EVENT_BEAT_<TRAINER>** flag -- set after victory, prevents re-battle
4. **Multi-Pokemon parties** -- Mikey has 2 Pokemon (Pidgey/2 + Rattata/4), Don has 2 (Caterpie/3 x2)

**Architecture decision**: Add trainer detection to `step_overworld()`.

```rust
// New struct in events.rs or data.rs:
pub struct TrainerDef {
    pub npc_idx: u8,
    pub trainer_class: u8,
    pub party: Vec<(SpeciesId, u8)>,  // (species, level) pairs
    pub range: u8,
    pub beaten_flag: u16,
    pub seen_text: &'static str,
    pub beaten_text: &'static str,
}
```

**New overworld result**:
```rust
OverworldResult::TrainerBattle {
    npc_idx: u8,
    trainer_def_idx: u8,  // index into map's trainer list
}
```

**Battle system changes** (`battle.rs`):
- `BattleState` needs to support multi-Pokemon enemy parties
- After defeating one enemy mon, the next one comes out
- Victory only when all enemy Pokemon fainted
- Prize money on win (not critical for Sprint 4)

Alternatively, for Sprint 4 simplicity: since the trainers have 1-2 low-level Pokemon, we can implement sequential enemy Pokemon without a full party system. The `BattleState` gets:
```rust
pub enemy_party: Vec<Pokemon>,  // full enemy team
pub enemy_index: usize,         // current enemy mon index
```

When current enemy faints, advance `enemy_index`. If no more, battle won.

### Trainer Sight Detection in `overworld.rs`

Add a check in `step_overworld()` after player stops walking:

```rust
// After processing movement, check trainer line-of-sight
for (i, npc_def) in map.npcs.iter().enumerate() {
    if let Some(trainer) = &npc_def.trainer {
        if flags.has(trainer.beaten_flag) { continue; }
        if !npc_states[i].visible { continue; }
        if is_in_sight(npc_states[i], player, trainer.range) {
            return OverworldResult::TrainerBattle { npc_idx: i as u8, ... };
        }
    }
}
```

**Simplification option**: Instead of adding a `trainer` field to NpcDef, we can use the existing `script_id` to look up trainer data. When a trainer sees the player, trigger their script which includes a `StartBattle` step. This keeps the existing architecture clean.

### `mod.rs` -- Mr. Pokemon Scene Entry

Add to `check_map_entry_scripts()`:
```rust
MapId::MrPokemonsHouse => {
    let scene = self.scene_state.get(self.current_map_id);
    if scene == SCENE_MRPOKEMONSHOUSE_MEET_MR_POKEMON {
        let steps = build_mr_pokemon_meet_script();
        self.script = Some(ScriptState::new(steps));
        self.phase = GamePhase::Script;
    }
}
```

---

## Mr. Pokemon Scene Flag Chain (Critical Path)

The Mr. Pokemon house visit is the major story progression gate. After the cutscene completes, these flags must be set to unlock the return journey:

1. `EVENT_GOT_MYSTERY_EGG_FROM_MR_POKEMON` -- player received the egg
2. `EVENT_ENGINE_POKEDEX` -- Oak gave the Pokedex
3. `EVENT_RIVAL_NEW_BARK_TOWN` -- rival appears in New Bark Town (for return trip)
4. `EVENT_RIVAL_CHERRYGROVE_CITY` -> `SCENE_CHERRYGROVECITY_MEET_RIVAL` -- rival battle triggers on return through Cherrygrove (this was already implemented in Sprint 2!)
5. `SCENE_ELMSLAB_MEET_OFFICER` -- Elm's Lab scene for when player returns with egg
6. `EVENT_COP_IN_ELMS_LAB` cleared -- officer NPC appears in lab
7. Stolen starter pokeball flag set (rival takes the type-advantage starter)

The Sprint 2 rival battle in Cherrygrove is already wired to `SCENE_CHERRYGROVECITY_MEET_RIVAL`. The Mr. Pokemon cutscene just needs to SET that scene, which means the existing rival battle code activates automatically on the return trip. No new battle code needed for that.

---

## Joey Pre-Battle Cutscene (Full Specification)

In pokecrystal, Route 30 has a scripted cutscene visible as you approach Joey. Two MONSTER
sprite objects (Rattata at (5,24) and (5,25)) animate a tackle sequence before the actual
trainer battle triggers. This is NOT a wild encounter -- they are environment object_events
with sprite SPRITE_RATTATA, controlled by EVENT_ROUTE_30_BATTLE.

**Objects involved**:
- NPC idx 0: YOUNGSTER at (5,26) -- the battle trigger object (hidden after EVENT_ROUTE_30_BATTLE)
- NPC idx 5: MONSTER at (5,24) -- Joey's Rattata (facing down, hidden after EVENT_ROUTE_30_BATTLE)
- NPC idx 6: MONSTER at (5,25) -- Mikey's Rattata (facing up, hidden after EVENT_ROUTE_30_BATTLE)

**Script sequence** (SCRIPT_JOEY_PREBATTLE_CUTSCENE):
1. Lock player movement
2. MONSTER at (5,24) moves DOWN 1 tile (tackle animation)
3. MONSTER at (5,25) moves UP 1 tile (tackle animation)
4. Both MONSTERS return to original positions
5. Repeat once more (2 tackle exchanges)
6. Joey (NPC idx 1 at (2,28)) emote SHOCK
7. Both MONSTERs disappear (set EVENT_ROUTE_30_BATTLE -> hides idx 0, 5, 6)
8. Joey walks to player
9. Joey dialogue: trainer battle intro text
10. Start trainer battle vs Joey (Rattata Lv4)
11. After battle: Joey asks for phone number (stub -- just dialogue, no phone registration)

**How to trigger**: When the player walks into the coord_event zone near (5,26) AND
EVENT_ROUTE_30_BATTLE is NOT set, trigger SCRIPT_JOEY_PREBATTLE_CUTSCENE.
After the script runs and the battle completes, EVENT_ROUTE_30_BATTLE is set, so the
three objects (trigger + 2 monsters) disappear permanently.

**Implementation in events.rs**: Build the script as a ScriptStep vector:
```rust
fn build_joey_prebattle_cutscene() -> Vec<ScriptStep> {
    vec![
        ScriptStep::LockPlayer,
        ScriptStep::MoveNpc { npc_idx: 5, dir: Direction::Down, tiles: 1 },
        ScriptStep::MoveNpc { npc_idx: 6, dir: Direction::Up, tiles: 1 },
        ScriptStep::MoveNpc { npc_idx: 5, dir: Direction::Up, tiles: 1 },
        ScriptStep::MoveNpc { npc_idx: 6, dir: Direction::Down, tiles: 1 },
        // second exchange
        ScriptStep::MoveNpc { npc_idx: 5, dir: Direction::Down, tiles: 1 },
        ScriptStep::MoveNpc { npc_idx: 6, dir: Direction::Up, tiles: 1 },
        ScriptStep::MoveNpc { npc_idx: 5, dir: Direction::Up, tiles: 1 },
        ScriptStep::MoveNpc { npc_idx: 6, dir: Direction::Down, tiles: 1 },
        ScriptStep::Emote { npc_idx: 1, emote: Emote::Shock },
        ScriptStep::SetFlag(EVENT_ROUTE_30_BATTLE),  // hides trigger + monsters
        ScriptStep::MoveNpcToPlayer { npc_idx: 1 },
        ScriptStep::Dialogue("Wow! My RATTATA is so strong! Let me battle you!"),
        ScriptStep::StartBattle {
            battle_type: BattleType::Normal,
            trainer_party: vec![(RATTATA, 4)],
        },
        ScriptStep::SetFlag(EVENT_BEAT_YOUNGSTER_JOEY),
        ScriptStep::Dialogue("Your POKeMON are really tough! ..."),
        ScriptStep::UnlockPlayer,
    ]
}
```

Note: `MoveNpc` and `MoveNpcToPlayer` are new ScriptStep variants needed for Sprint 4.
`MoveNpc` moves an NPC a fixed number of tiles in a direction. `MoveNpcToPlayer` pathfinds
the NPC to the player's adjacent tile (simple Manhattan walk -- no obstacle avoidance needed
for these short scripted walks).

---

## GenericHouse Template (Route 30 Berry House)

Route 30 Berry House is the first "generic house" -- a reusable 8x8 interior template
used by many Johto houses. Rather than hand-building each one, define a template function.

**GenericHouse spec** (from pokecrystal TILESET_HOUSE maps):
- Dimensions: 8 wide x 8 tall (same as PlayersHouse1F, GuideGentsHouse)
- Tileset: TILESET_HOUSE
- Floor: C_FLOOR(0) everywhere except walls
- Walls: row 0 is all C_WALL, column 0 and 7 are C_WALL
- Door warps: (2,7) and (3,7) -- bottom-center, always exit to the outdoor map
- Standard furniture tiles: bookshelf at row 1 (C_WALL), table/chairs mid-room

**Template function in maps.rs**:
```rust
fn build_generic_house(
    map_id: MapId,
    exit_map: MapId,
    exit_warp_id: u8,
    npcs: Vec<NpcDef>,
    bg_events: Vec<BgEvent>,
) -> MapData {
    let width = 8;
    let height = 8;
    let mut tiles = vec![0u8; width * height];
    let mut collision = vec![C_FLOOR; width * height];
    // ... standard house layout ...
    // row 0: all wall
    // col 0, col 7: wall
    // row 1 cols 0-1: bookshelf (wall)
    let warps = vec![
        WarpDef { x: 2, y: 7, target_map: exit_map, target_warp: exit_warp_id },
        WarpDef { x: 3, y: 7, target_map: exit_map, target_warp: exit_warp_id },
    ];
    // ... build collision with C_WARP at door tiles ...
    MapData { map_id, width, height, tiles, collision, warps, npcs, bg_events, ..Default::default() }
}
```

**Route 30 Berry House usage**:
```rust
fn build_route30_berry_house() -> MapData {
    build_generic_house(
        MapId::Route30BerryHouse,
        MapId::Route30,
        0, // warp 0 on Route 30 = (7,39) Berry House door
        vec![NpcDef {
            x: 2, y: 3,
            sprite: SpriteId::PokefanM,
            movement: MoveType::StandingDown,
            script_id: SCRIPT_BERRY_HOUSE_POKEFAN,
            event_flag: None,
        }],
        vec![
            BgEvent { x: 0, y: 1, script_id: SCRIPT_BERRY_HOUSE_BOOKSHELF },
            BgEvent { x: 1, y: 1, script_id: SCRIPT_BERRY_HOUSE_BOOKSHELF },
        ],
    )
}
```

This template will be reused for future houses (Route 31 house, etc.).

---

## Joey Rematch / Phone System (Stub Specification)

Joey's rematch system is the most complex trainer rematch in the early game. For Sprint 4,
we implement the **first encounter only** and stub the infrastructure for future tiers.

**Full rematch system** (for reference, from pokecrystal):
- JOEY1: Rattata Lv4 (initial encounter, Sprint 4)
- JOEY2: Rattata Lv15 (unlocked at ENGINE_FLYPOINT_GOLDENROD)
- JOEY3: Rattata Lv21 (unlocked at ENGINE_FLYPOINT_OLIVINE)
- JOEY4: Rattata Lv30 (unlocked at EVENT_CLEARED_RADIO_TOWER)
- JOEY5: Raticate Lv37 (unlocked at EVENT_BEAT_ELITE_FOUR, can give HP_UP)

**Sprint 4 implementation**:
- After beating Joey, his talk script shows post-battle dialogue
- Joey asks for phone number -- player "accepts" (no actual Pokegear system yet)
- Set EVENT_JOEY_ASKED_FOR_PHONE_NUMBER flag
- Future sprints can check this flag + progression flags to offer rematches

**Data structure for future use** (define now, use later):
```rust
pub struct RematchTier {
    pub party: Vec<(SpeciesId, u8)>,
    pub required_flag: u16,  // game-progress flag that unlocks this tier
}
```

No need to implement RematchTier logic in Sprint 4. Just define the struct so the
architecture is forward-compatible. Joey's initial battle uses the standard trainer system.

---

## New Species Data Required

All from `pokecrystal-master/data/pokemon/base_stats/`:

| Species | ID | Type1 | Type2 | HP | Atk | Def | Spd | SpA | SpD | Learnset (Lv 1-5) |
|---------|-----|-------|-------|-----|-----|-----|-----|-----|-----|-------------------|
| Caterpie | 10 | Bug | Bug | 45 | 30 | 35 | 45 | 20 | 20 | Tackle(1), StringShot(1) |
| Metapod | 11 | Bug | Bug | 50 | 20 | 55 | 30 | 25 | 25 | Harden(1) |
| Weedle | 13 | Bug | Poison | 40 | 35 | 30 | 50 | 20 | 20 | PoisonSting(1), StringShot(1) |
| Zubat | 41 | Poison | Flying | 40 | 45 | 35 | 55 | 30 | 40 | LeechLife(1) |
| Poliwag | 60 | Water | Water | 40 | 50 | 40 | 90 | 40 | 40 | Bubble(1) |
| Ledyba | 165 | Bug | Flying | 40 | 20 | 30 | 55 | 40 | 80 | Tackle(1) |
| Spinarak | 167 | Bug | Poison | 40 | 60 | 40 | 30 | 40 | 40 | PoisonSting(1), StringShot(1) |

---

## Phased Implementation Plan

This document serves as both architecture and implementation plan. Implementation proceeds
in 5 phases, each building on the prior. Every phase ends with `cargo check` + `cargo test`.

### Phase 1: Data Layer (~200 lines in data.rs)

**Goal**: All new species, moves, and items exist and compile.

1. Add species ID constants: CATERPIE(10), METAPOD(11), WEEDLE(13), ZUBAT(41), POLIWAG(60), LEDYBA(165), SPINARAK(167)
2. Add move ID constants: MOVE_STRING_SHOT(81), MOVE_POISON_STING(40), MOVE_HARDEN(106), MOVE_LEECH_LIFE(141), MOVE_CONSTRICT(132), MOVE_SUPERSONIC(48), MOVE_BUBBLE(12)
3. Add SpeciesData statics for all 7 new species (base stats from pokecrystal base_stats/*.asm)
4. Add MoveData entries for new moves (power/accuracy/type/pp from pokecrystal)
5. Add ITEM_MYSTERY_EGG(130), ITEM_BERRY(31) constants
6. Add `TrainerDef` struct and `RematchTier` struct (data definitions only)

**Tests**: species_data lookup for each new species, move_data lookup for each new move.

### Phase 2: Maps Layer (~250 lines in maps.rs)

**Goal**: All 3 new maps load correctly with proper dimensions, warps, NPCs, and encounters.

1. Add MapId variants: `Route30BerryHouse`, `MrPokemonsHouse` (Route30 and Route31 already exist as stubs)
2. Implement `build_generic_house()` template function
3. Implement `build_route30_berry_house()` using the template (8x8, 1 NPC, 2 bg_events)
4. Implement `build_mr_pokemons_house()` (8x8, TILESET_FACILITY, 2 NPCs -- Oak conditional on flag, 5 bg_events)
5. Replace `build_route30_stub()` with full `build_route30()`:
   - 20x54 tiles, grass patches marked C_GRASS, walls, water on east side (C_WATER)
   - 11 NPCs as specified in the NPC table above
   - 2 warps (Berry House at (7,39), Mr. Pokemon's House at (17,5))
   - 5 bg_events (4 signs + 1 hidden Potion)
   - Wild encounter tables (morning/day/night, 7 slots each, encounter_rate 10)
   - Connections: north->Route31(offset -10), south->CherrygroveCity(offset -5)
6. Add `trainer_range: Option<u8>` field to `NpcDef` (None for non-trainers)
7. Update `load_map()` dispatcher with new maps

**Tests**: map dimensions, warp validity (both directions), NPC count, encounter table slot count, connection offsets.

### Phase 3: Events Layer (~200 lines in events.rs)

**Goal**: All scripts compile and produce correct flag/scene state transitions.

1. Add event flag constants (EVENT_BEAT_YOUNGSTER_JOEY through EVENT_JOEY_ASKED_FOR_PHONE_NUMBER)
2. Add scene constants (SCENE_MRPOKEMONSHOUSE_MEET_MR_POKEMON, SCENE_MRPOKEMONSHOUSE_NOOP)
3. Add script ID constants (~15 IDs, 300-series for Route 30, 320-series for Berry House, 330-series for Mr. Pokemon's House)
4. Add new ScriptStep variants:
   - `MoveNpc { npc_idx: u8, dir: Direction, tiles: u8 }` -- move NPC fixed tiles
   - `MoveNpcToPlayer { npc_idx: u8 }` -- NPC walks to player's adjacent tile
   - `HideNpc { npc_idx: u8 }` -- visually remove NPC (for Oak exit)
   - `StartTrainerBattle { trainer_party: Vec<(SpeciesId, u8)> }` -- initiate trainer battle
   - `HealParty` -- heal all Pokemon in player's party
   - `SetMapScene { map_id: MapId, scene: u8 }` -- set scene for a different map
5. Implement `step_script()` handlers for new ScriptStep variants
6. Build script functions:
   - `build_joey_prebattle_cutscene()` -- full cutscene as specified above
   - `build_trainer_joey_script()`, `build_trainer_mikey_script()`, `build_trainer_don_script()` -- standard trainer battle scripts (emote, walk, dialogue, StartTrainerBattle)
   - `build_mr_pokemon_meet_script()` -- full Mr. Pokemon + Oak scene chain
   - `build_berry_house_pokefan_script()` -- give Berry on first visit
   - Simple dialogue scripts for signs, cooltrainer, bookshelf, etc.
7. Register all scripts in `get_script()`

**Tests**: script flag chain verification (run Mr. Pokemon script, verify all flags/scenes set), trainer scripts set beaten flags, Berry NPC checks/sets its flag.

### Phase 4: Battle + Overworld (~90 lines in battle.rs + overworld.rs)

**Goal**: Multi-Pokemon trainer battles work, trainer sight detection triggers battles.

1. **battle.rs**: Add `enemy_party: Vec<Pokemon>` and `enemy_index: usize` to BattleState
2. **battle.rs**: Modify `step_battle()` -- when current enemy faints, advance `enemy_index`. Victory only when `enemy_index >= enemy_party.len()`
3. **battle.rs**: Add `BattleType::Trainer` variant (no flee option, no catch option)
4. **overworld.rs**: Add trainer sight-range check in `step_overworld()`:
   - After player stops moving, iterate NPCs with `trainer_range.is_some()`
   - Skip if beaten_flag is set
   - Check if player is within `range` tiles in NPC's facing direction
   - If detected: return `OverworldResult::TrainerBattle { npc_idx }`
5. **mod.rs**: Handle `OverworldResult::TrainerBattle` -- trigger the trainer's script (which includes the battle start step)

**Tests**: multi-Pokemon battle (2 enemy mons, verify both faint before victory), trainer sight detection (player in range = detected, out of range = not detected, beaten trainer = ignored).

### Phase 5: Integration + Scene Wiring (~30 lines in mod.rs)

**Goal**: Full Route 30 -> Mr. Pokemon's House -> return trip flows correctly.

1. Add MrPokemonsHouse to `check_map_entry_scripts()` (trigger scene 0 on first entry)
2. Wire ScriptResult::StartTrainerBattle to create BattleState with trainer party
3. Wire post-battle: when trainer battle ends with victory, resume script (for post-battle dialogue + flag setting)
4. Verify the full story chain:
   - Enter Route 30 from Cherrygrove (south connection)
   - Walk north, trigger Joey pre-battle cutscene + battle
   - Enter Berry House, get Berry
   - Continue north to Mr. Pokemon's House
   - Scene 0 auto-triggers: get Mystery Egg, get Pokedex from Oak
   - Scene sets CHERRYGROVE rival scene + ELMS_LAB officer scene
   - Exit south, return through Route 30 to Cherrygrove -> rival battle triggers (Sprint 2 code)

**Tests**: full integration test (start at CherrygroveCity, walk to Route30, enter MrPokemonsHouse, verify flags set for return journey).

---

## Estimated Change Summary

| File | Lines Added (est.) | Key Changes |
|------|-------------------|-------------|
| `data.rs` | ~200 | 7 species data + ~7 move data + items + TrainerDef struct |
| `maps.rs` | ~250 | build_route30(), build_generic_house(), build_route30_berry_house(), build_mr_pokemons_house(), trainer_range field |
| `events.rs` | ~200 | ~15 scripts, ~12 event flags, 6 new ScriptStep variants, scene constants |
| `overworld.rs` | ~40 | Trainer sight-range detection loop |
| `battle.rs` | ~50 | Multi-Pokemon enemy party support, BattleType::Trainer |
| `mod.rs` | ~30 | MrPokemonsHouse entry script, TrainerBattle handling, post-battle script resume |
| **Total** | **~770** | |

---

## Architectural Decisions

### Trainer Detection: Option A (Sight-Range)

Add `trainer_range: Option<u8>` to `NpcDef`. Check line-of-sight in `step_overworld()`
after player stops moving. Trainer walks to player, then triggers script.

This is a reusable system needed for all future routes. The NpcDef already has fields for
script_id and event_flag, so adding trainer_range is minimal.

### GenericHouse Template

Define `build_generic_house()` as a shared template function in `maps.rs`. All standard
Johto house interiors (8x8, TILESET_HOUSE, bottom-center door) use this template.
Per-house customization is passed via NPC and BG event parameters.

### Multi-Pokemon Battles

Extend `BattleState` with `enemy_party: Vec<Pokemon>` and `enemy_index: usize` rather
than creating a separate TrainerBattleState. This keeps the battle system unified -- wild
battles are just a party of 1.

### Joey Cutscene: Script-Based

The MONSTER sprite objects are regular NPCs with a special sprite. The tackle animation
is implemented as MoveNpc script steps, not a custom animation system. This keeps the
architecture simple and reusable for future scripted NPC movements.

### Explicitly Deferred
- Joey rematch tiers JOEY2-JOEY5 (define RematchTier struct now, implement logic later)
- Phone number registration / Pokegear system
- Water tiles / Surf encounters on Route 30 east side
- Headbutt trees
- Route 31 full implementation (connection target stub only)
- Elm phone call SPECIALCALL_ROBBED (stub as dialogue text overlay)
