# Sprint 5 Architecture: Route 31 + Route 31/Violet Gate + Violet City Exterior

Sprint 5 adds Route 31 (full outdoor route with Bug Catcher Wade trainer, Kenya mail sidequest,
Dark Cave entrance), Route 31/Violet Gate building, and Violet City exterior (first gym city)
with its full building warp network.

---

## Source of Truth

All data comes from `pokecrystal-master/`:
- `maps/Route31.asm` -- NPCs, warps, bg_events, object_events, scripts
- `maps/Route31VioletGate.asm` -- gate NPCs + warps
- `maps/VioletCity.asm` -- city NPCs, warps, bg_events, Earl script, flypoint callback
- `data/wild/johto_grass.asm` lines 1293-1318 -- Route 31 encounter tables
- `data/trainers/parties.asm` lines 1516-1522 -- Wade1 party (Caterpie/2 x3 + Weedle/3)
- `constants/map_constants.asm` -- ROUTE_31 = 20x9 metatiles (40x18 tiles), ROUTE_31_VIOLET_GATE = 5x4 metatiles (10x8 tiles), VIOLET_CITY = 20x18 metatiles (40x36 tiles)
- `data/maps/attributes.asm` -- Route31 connections: south->Route30(offset 10), west->VioletCity(offset -9); VioletCity connections: south->Route32(offset 0), west->Route36(offset 0), east->Route31(offset 9)
- `data/pokemon/base_stats/bellsprout.asm` -- new species for Route 31 encounters
- `data/pokemon/base_stats/gastly.asm` -- new species for Route 31 night encounters
- `data/pokemon/evos_attacks.asm` -- Bellsprout learnset (VineWhip@1, Growth@6), Gastly learnset (Hypnosis@1, Lick@1)

---

## New Maps

### Route 31 (40 x 18 tiles, 20x9 metatiles)

**pokecrystal dimensions**: `map_const ROUTE_31, 20, 9` = 40 tiles wide x 18 tiles tall.

**Connections** (from `attributes.asm`):
- South -> Route30 (offset: 10)
- West -> VioletCity (offset: -9) -- NOTE: connection goes THROUGH Route31VioletGate warps

**Warps** (from `Route31_MapEvents`):
```
warp_event  4,  6, ROUTE_31_VIOLET_GATE, 3   -- gate upper (warp idx 0)
warp_event  4,  7, ROUTE_31_VIOLET_GATE, 4   -- gate lower (warp idx 1)
warp_event 34,  5, DARK_CAVE_VIOLET_ENTRANCE, 1  -- Dark Cave (warp idx 2, stub)
```

**NPCs** (7 object_events from Route31.asm):

| Idx | Name | Sprite | Position | MoveType | Type | Notes |
|-----|------|--------|----------|----------|------|-------|
| 0 | FISHER (Kenya mail) | FISHER | (17,7) | StandingDown | Script | Kenya sidequest NPC |
| 1 | YOUNGSTER | YOUNGSTER | (9,5) | Wander(1,1) | Script | Dialogue NPC |
| 2 | BUG_CATCHER (Wade) | BUG_CATCHER | (21,13) | StandingLeft | Trainer(range=5) | EVENT_BEAT_BUG_CATCHER_WADE |
| 3 | COOLTRAINER_M | COOLTRAINER_M | (33,8) | Wander(1,1) | Script | Dialogue about Dark Cave |
| 4 | FRUIT_TREE | FRUIT_TREE | (16,7) | Still | Script | FRUITTREE_ROUTE_31 |
| 5 | POKE_BALL1 (Potion) | POKE_BALL | (29,5) | Still | ItemBall | EVENT_ROUTE_31_POTION |
| 6 | POKE_BALL2 (Poke Ball) | POKE_BALL | (19,15) | Still | ItemBall | EVENT_ROUTE_31_POKE_BALL |

**BG Events** (from Route31_MapEvents):
- `(7, 5)` BGEVENT_READ -- Route 31 sign
- `(31, 5)` BGEVENT_READ -- Dark Cave sign

**Callback**: MAPCALLBACK_NEWMAP -> Route31CheckMomCallCallback (checks EVENT_TALKED_TO_MOM_AFTER_MYSTERY_EGG_QUEST, if false triggers specialphonecall SPECIALCALL_WORRIED). For Sprint 5, this is a stub -- just log the callback.

**Wild Encounters** (from johto_grass.asm lines 1293-1318):
```
encounter_rate: 10 (all periods) -- actually "10 percent" in asm

morning: [Ledyba/4, Caterpie/4, Bellsprout/5, Pidgey/5, Weedle/4, Hoppip/5, Hoppip/5]
day:     [Pidgey/4, Caterpie/4, Bellsprout/5, Pidgey/5, Weedle/4, Hoppip/5, Hoppip/5]
night:   [Spinarak/4, Poliwag/4, Bellsprout/5, Hoothoot/5, Zubat/4, Gastly/5, Gastly/5]
```

**Terrain features**:
- Grass patches in multiple areas (encounter zones)
- Dark Cave entrance at east side (34,5) -- warp target is a stub map
- Trees/walls forming the route corridor
- No ledges on Route 31

### Route 31/Violet Gate (10 x 8 tiles, 5x4 metatiles)

Standard gate building connecting Route 31 to Violet City.

**Warps** (from Route31VioletGate_MapEvents):
```
warp_event  0,  4, VIOLET_CITY, 8    -- west exit upper (warp idx 0)
warp_event  0,  5, VIOLET_CITY, 9    -- west exit lower (warp idx 1)
warp_event  9,  4, ROUTE_31, 1       -- east exit upper (warp idx 2)
warp_event  9,  5, ROUTE_31, 2       -- east exit lower (warp idx 3)
```

**NPCs** (2):
- OFFICER at (5,2), StandingDown, PAL_NPC_RED -- "Did you visit SPROUT TOWER?"
- COOLTRAINER_F at (1,2), SpinRandom(slow), PAL_NPC_BLUE -- "I came too far out."

**BG Events**: None.

### Violet City Exterior (40 x 36 tiles, 20x18 metatiles)

First gym city. Large outdoor map with 9 warps to buildings (most are stubs for Sprint 5).

**Connections** (from `attributes.asm`):
- South -> Route32 (offset: 0) -- stub
- West -> Route36 (offset: 0) -- stub
- East -> Route31 (offset: 9)

**Warps** (from VioletCity_MapEvents):
```
warp_event  9, 17, VIOLET_MART, 2                     -- warp idx 0
warp_event 18, 17, VIOLET_GYM, 1                      -- warp idx 1
warp_event 30, 17, EARLS_POKEMON_ACADEMY, 1            -- warp idx 2
warp_event  3, 15, VIOLET_NICKNAME_SPEECH_HOUSE, 1     -- warp idx 3
warp_event 31, 25, VIOLET_POKECENTER_1F, 1             -- warp idx 4
warp_event 21, 29, VIOLET_KYLES_HOUSE, 1               -- warp idx 5
warp_event 23,  5, SPROUT_TOWER_1F, 1                  -- warp idx 6
warp_event 39, 24, ROUTE_31_VIOLET_GATE, 1             -- warp idx 7
warp_event 39, 25, ROUTE_31_VIOLET_GATE, 2             -- warp idx 8
```

**NPCs** (8 object_events from VioletCity.asm):

| Idx | Name | Sprite | Position | MoveType | Script | Event Flag | Notes |
|-----|------|--------|----------|----------|--------|------------|-------|
| 0 | EARL | FISHER | (13,16) | SpinRandom(slow) | VioletCityEarlScript | EVENT_VIOLET_CITY_EARL | Conditional, walks to academy |
| 1 | LASS | LASS | (28,28) | Wander(2,2) | Dialogue | -1 | Ghosts in Sprout Tower |
| 2 | SUPER_NERD | SUPER_NERD | (24,14) | Wander(1,2) | Dialogue | -1 | Beat gym leader |
| 3 | GRAMPS | GRAMPS | (17,20) | WalkLeftRight(1) | Dialogue | -1 | Falkner's father |
| 4 | YOUNGSTER | YOUNGSTER | (5,18) | SpinRandom(slow) | Dialogue | -1 | Wiggly tree |
| 5 | FRUIT_TREE | FRUIT_TREE | (14,29) | Still | FruitTree | -1 | FRUITTREE_VIOLET_CITY (PRZCureBerry) |
| 6 | POKE_BALL1 (PP Up) | POKE_BALL | (4,1) | Still | ItemBall | EVENT_VIOLET_CITY_PP_UP | |
| 7 | POKE_BALL2 (Rare Candy) | POKE_BALL | (35,5) | Still | ItemBall | EVENT_VIOLET_CITY_RARE_CANDY | |

**BG Events** (from VioletCity_MapEvents):
- `(24, 20)` BGEVENT_READ -- VioletCitySign
- `(15, 17)` BGEVENT_READ -- VioletGymSign
- `(24, 8)`  BGEVENT_READ -- SproutTowerSign
- `(27, 17)` BGEVENT_READ -- EarlsPokemonAcademySign
- `(32, 25)` BGEVENT_READ -- VioletCityPokecenterSign
- `(10, 17)` BGEVENT_READ -- VioletCityMartSign
- `(37, 14)` BGEVENT_ITEM -- VioletCityHiddenHyperPotion (EVENT_VIOLET_CITY_HIDDEN_HYPER_POTION)

**Callback**: MAPCALLBACK_NEWMAP -> `setflag ENGINE_FLYPOINT_VIOLET`

**Stub Building Maps**: Violet Mart, Violet Gym, Earl's Academy, Nickname Speech House,
Violet Pokecenter 1F, Kyle's House, Sprout Tower 1F -- all 7 are stubs for Sprint 5.
Each gets a minimal `build_stub_house()` or similar that has return warps to VioletCity.

---

## Module Changes

### `data.rs` -- New Species + Moves

**New species** needed for Route 31 encounters:

| Species | ID | Type1 | Type2 | HP | Atk | Def | Spd | SpA | SpD | Growth | Learnset (Lv 1-5) |
|---------|-----|-------|-------|-----|-----|-----|-----|-----|-----|--------|-------------------|
| Bellsprout | 69 | Grass | Poison | 50 | 75 | 35 | 40 | 70 | 30 | MediumSlow | VineWhip(1) |
| Gastly | 92 | Ghost | Poison | 30 | 35 | 30 | 80 | 100 | 35 | MediumSlow | Hypnosis(1), Lick(1) |

Source: `pokecrystal-master/data/pokemon/base_stats/bellsprout.asm` and `gastly.asm`.

**New species ID constants**:
```rust
pub const BELLSPROUT: SpeciesId = 69;
pub const GASTLY: SpeciesId = 92;
```

**New moves** needed for Bellsprout and Gastly learnsets:

| Move | ID | Type | Power | Accuracy | PP | Physical/Special |
|------|-----|------|-------|----------|-----|-----------------|
| Vine Whip | 22 | Grass | 35 | 100 | 10 | Special |
| Hypnosis | 95 | Psychic | 0 | 60 | 20 | N/A (status) |
| Lick | 122 | Ghost | 20 | 100 | 30 | Physical |

Source: pokecrystal move constants (0x16=22, 0x5f=95, 0x7a=122).

**New move ID constants**:
```rust
pub const MOVE_VINE_WHIP: MoveId = 22;
pub const MOVE_HYPNOSIS: MoveId = 95;
pub const MOVE_LICK: MoveId = 122;
```

**New item constants** (for Violet City item balls):
```rust
pub const ITEM_PP_UP: u8 = 48;
pub const ITEM_RARE_CANDY: u8 = 43;
pub const ITEM_PRZ_CURE_BERRY: u8 = 54;
pub const ITEM_TM50_NIGHTMARE: u8 = 200;  // TM item, stub ID
```

**New music constant**:
```rust
pub const MUSIC_VIOLET_CITY: u8 = 15;
pub const MUSIC_ROUTE_31: u8 = 16;
```

### `maps.rs` -- New MapId Variants + Builders

**New MapId variants** (add to MapId enum):
```rust
// Sprint 5 (new)
Route31VioletGate,
VioletCity,
// Violet City building stubs
VioletMart,
VioletGym,
EarlsPokemonAcademy,
VioletNicknameSpeechHouse,
VioletPokecenter1F,
VioletKylesHouse,
SproutTower1F,
// Connection target stubs
DarkCaveVioletEntrance,
Route32,
Route36,
```

**Existing Route31 stub -> full implementation**: Replace `build_route31_stub()` with `build_route31()`.

**New builder functions**:
- `build_route31()` -- 40x18, grass patches, 3 warps (gate x2 + Dark Cave), 7 NPCs, 2 bg_events, wild encounters, connections south+west
- `build_route31_violet_gate()` -- 10x8, standard gate template, 4 warps, 2 NPCs
- `build_violet_city()` -- 40x36, 9 warps, 8 NPCs, 7 bg_events, connections south+west+east
- `build_route31_encounters()` -- wild encounter table helper
- 7 stub building builders for Violet City interiors
- `build_dark_cave_violet_entrance_stub()` -- minimal stub (connection target only)
- `build_route32_stub()` -- minimal stub (connection target)
- `build_route36_stub()` -- minimal stub (connection target)

**load_map() dispatcher**: Add all new MapId variants.

### `events.rs` -- New Event Flags + Scripts

**New event flags**:
```rust
pub const EVENT_BEAT_BUG_CATCHER_WADE: u16 = 44;
pub const EVENT_WADE_ASKED_FOR_PHONE_NUMBER: u16 = 45;
pub const EVENT_ROUTE_31_POTION: u16 = 46;
pub const EVENT_ROUTE_31_POKE_BALL: u16 = 47;
pub const EVENT_VIOLET_CITY_EARL: u16 = 48;
pub const EVENT_VIOLET_CITY_PP_UP: u16 = 49;
pub const EVENT_VIOLET_CITY_RARE_CANDY: u16 = 50;
pub const EVENT_VIOLET_CITY_HIDDEN_HYPER_POTION: u16 = 51;
pub const EVENT_ENGINE_FLYPOINT_VIOLET: u16 = 52;
pub const EVENT_GOT_TM50_NIGHTMARE: u16 = 53;
pub const EVENT_GOT_KENYA: u16 = 54;
pub const EVENT_GAVE_KENYA: u16 = 55;
pub const EVENT_TALKED_TO_MOM_AFTER_MYSTERY_EGG_QUEST: u16 = 56;
pub const EVENT_EARLS_ACADEMY_EARL: u16 = 57;
```

**New script ID constants** (~20 new):
```rust
// Sprint 5: Route 31 scripts
pub const SCRIPT_ROUTE31_SIGN: u16 = 400;
pub const SCRIPT_DARK_CAVE_SIGN: u16 = 401;
pub const SCRIPT_TRAINER_WADE: u16 = 402;
pub const SCRIPT_ROUTE31_MAIL_RECIPIENT: u16 = 403;
pub const SCRIPT_ROUTE31_YOUNGSTER: u16 = 404;
pub const SCRIPT_ROUTE31_COOLTRAINER_M: u16 = 405;
pub const SCRIPT_ROUTE31_FRUIT_TREE: u16 = 406;
pub const SCRIPT_ROUTE31_POTION: u16 = 407;
pub const SCRIPT_ROUTE31_POKE_BALL: u16 = 408;

// Sprint 5: Route 31 Violet Gate scripts
pub const SCRIPT_GATE_OFFICER_VIOLET: u16 = 420;
pub const SCRIPT_GATE_COOLTRAINER_F_VIOLET: u16 = 421;

// Sprint 5: Violet City scripts
pub const SCRIPT_VIOLET_CITY_EARL: u16 = 430;
pub const SCRIPT_VIOLET_CITY_LASS: u16 = 431;
pub const SCRIPT_VIOLET_CITY_SUPER_NERD: u16 = 432;
pub const SCRIPT_VIOLET_CITY_GRAMPS: u16 = 433;
pub const SCRIPT_VIOLET_CITY_YOUNGSTER: u16 = 434;
pub const SCRIPT_VIOLET_CITY_FRUIT_TREE: u16 = 435;
pub const SCRIPT_VIOLET_CITY_PP_UP: u16 = 436;
pub const SCRIPT_VIOLET_CITY_RARE_CANDY: u16 = 437;
pub const SCRIPT_VIOLET_CITY_SIGN: u16 = 440;
pub const SCRIPT_VIOLET_GYM_SIGN: u16 = 441;
pub const SCRIPT_SPROUT_TOWER_SIGN: u16 = 442;
pub const SCRIPT_EARLS_ACADEMY_SIGN: u16 = 443;
pub const SCRIPT_VIOLET_POKECENTER_SIGN: u16 = 444;
pub const SCRIPT_VIOLET_MART_SIGN: u16 = 445;
pub const SCRIPT_VIOLET_CITY_HIDDEN_HYPER_POTION: u16 = 446;
```

**New scripts to build**:

1. **Wade trainer script** (`build_trainer_wade_script()`):
   - Standard trainer pattern: emote, walk to player, "seen text", LoadTrainerParty, StartBattle
   - Wade1 party: Caterpie/2 x3 + Weedle/3 (from `parties.asm` line 1516-1522)
   - After battle: post-battle text about catching Pokemon, phone number ask (stub), set EVENT_WADE_ASKED_FOR_PHONE_NUMBER

2. **Kenya mail recipient** (`build_route31_mail_recipient_script()`):
   - Full Kenya sidequest:
     - If EVENT_GOT_TM50_NIGHTMARE set -> describe Nightmare text
     - If EVENT_GOT_KENYA set -> try to give Kenya (stub: simplified to just dialogue for now since the full mail/Pokemon swap system isn't implemented)
     - Else -> sleepy man dialogue
   - Sprint 5 simplification: Just the default "sleepy man" dialogue. The Kenya sidequest requires a Pokemon mail system not yet implemented.

3. **Route 31 dialogue NPCs** -- simple jumptextfaceplayer scripts:
   - Youngster: talks about Dark Cave Pokemon + Falkner
   - CooltrainerM: talks about Dark Cave needing Flash

4. **Violet City Earl** (`build_violet_city_earl_script()`):
   - Sprint 5 simplification: Earl asks "Did you beat Falkner?", YesNo.
   - If yes: "Very nice indeed!" dialogue
   - If no: "Follow me!" -- Earl walks to academy, disappears, sets EVENT_EARLS_ACADEMY_EARL
   - Full follow/spin movement sequence is complex; Sprint 5 can simplify to dialogue + flag set

5. **Violet City dialogue NPCs** -- simple scripts:
   - Lass: ghosts in Sprout Tower, Normal moves don't work on ghosts
   - Super Nerd: beat gym leader for prime time
   - Gramps: Falkner inherited gym from father
   - Youngster: wiggly tree up ahead

6. **Sign scripts** -- ShowText + End for all signs:
   - Route 31 sign: "ROUTE 31 / VIOLET CITY - CHERRYGROVE CITY"
   - Dark Cave sign: "DARK CAVE"
   - Violet City sign: "VIOLET CITY / The City of Nostalgic Scents"
   - Violet Gym sign: "VIOLET CITY POKEMON GYM / LEADER: FALKNER"
   - Sprout Tower sign: "SPROUT TOWER"
   - Earl's Academy sign: "EARL'S POKEMON ACADEMY"
   - Pokecenter/Mart signs: standard

7. **Item ball scripts** -- standard CheckEvent + GiveItem + SetEvent pattern

### `overworld.rs` -- No structural changes

The trainer sight detection system already works from Sprint 4. Wade uses the same
`trainer_range: Some(5)` field on his NpcDef and the same `is_in_sight()` check.

### `battle.rs` -- No structural changes

Wade's 4-Pokemon party uses the same `BattleState::new_trainer_party()` introduced in Sprint 4.
No new battle mechanics needed.

### `mod.rs` -- Map callbacks + new stubs

**check_map_callbacks()** additions:
```rust
MapId::VioletCity => {
    self.event_flags.set(EVENT_ENGINE_FLYPOINT_VIOLET);
}
```

**warp_to_last_pokecenter()** addition:
```rust
// After Violet Pokecenter is visited, blackout warps there
if self.event_flags.has(EVENT_ENGINE_FLYPOINT_VIOLET) {
    MapId::VioletPokecenter1F
} else if self.event_flags.has(EVENT_ENGINE_FLYPOINT_CHERRYGROVE) {
    MapId::CherrygrovePokecenter1F
} else {
    MapId::ElmsLab
}
```

---

## Wade Trainer Battle (Detailed Specification)

**Trainer**: Bug Catcher Wade at Route 31 (21,13), facing Left, range 5.

**Party** (from `parties.asm` BUG_CATCHER(4) WADE):
```
Caterpie Lv2
Caterpie Lv2
Weedle   Lv3
Caterpie Lv2
```

**Battle flow**:
1. Player enters Wade's sight range (5 tiles left of him, same row y=13)
2. OverworldResult::TrainerBattle triggers
3. Script: emote Shock, walk toward player, seen text "I caught a bunch of POKEMON. Let me battle with you!"
4. LoadTrainerParty with 4 mons, beaten_flag = EVENT_BEAT_BUG_CATCHER_WADE
5. StartBattle(BattleType::Normal)
6. On victory: beaten_flag set, Wade's NPC hidden (event_flag_show=false + flag set)
7. Post-battle dialogue: phone number request (stub, sets EVENT_WADE_ASKED_FOR_PHONE_NUMBER)

**Implementation**: Same pattern as Sprint 4 trainer scripts (Joey/Mikey/Don).

---

## Kenya Mail Sidequest (Stub Specification)

The full Kenya sidequest requires:
- Player has Kenya the Spearow (from Route 35 guard, not yet implemented)
- Kenya holds Mail with specific text
- Player trades Kenya+mail to the Fisher NPC on Route 31
- Fisher gives TM50 Nightmare in return

**Sprint 5 implementation**: The Fisher NPC at (17,7) shows the default "sleepy man" dialogue.
The EVENT_GOT_KENYA flag won't be set (the Kenya giver is on Route 35), so the NPC just shows
the sleepy text. Full sidequest completion deferred to when Route 35 is implemented.

---

## Warp Topology (Bidirectional Consistency)

### Route 31 <-> Route 31 Violet Gate
```
Route31 warp 0: (4,6) -> Route31VioletGate warp 2 (idx 2 = east upper at 9,4)
Route31 warp 1: (4,7) -> Route31VioletGate warp 3 (idx 3 = east lower at 9,5)
Route31VioletGate warp 2: (9,4) -> Route31 warp 0 (idx 0 = gate upper at 4,6)
Route31VioletGate warp 3: (9,5) -> Route31 warp 1 (idx 1 = gate lower at 4,7)
```

### Route 31 Violet Gate <-> Violet City
```
Route31VioletGate warp 0: (0,4) -> VioletCity warp 7 (idx 7 = east gate upper at 39,24)
Route31VioletGate warp 1: (0,5) -> VioletCity warp 8 (idx 8 = east gate lower at 39,25)
VioletCity warp 7: (39,24) -> Route31VioletGate warp 0 (idx 0 = west upper at 0,4)
VioletCity warp 8: (39,25) -> Route31VioletGate warp 1 (idx 1 = west lower at 0,5)
```

### Route 31 -> Dark Cave (one-way stub)
```
Route31 warp 2: (34,5) -> DarkCaveVioletEntrance warp 0
DarkCaveVioletEntrance warp 0: -> Route31 warp 2 (return)
```

### Violet City -> Interior Buildings (7 stubs)
```
VioletCity warp 0: (9,17)  -> VioletMart warp 0
VioletCity warp 1: (18,17) -> VioletGym warp 0
VioletCity warp 2: (30,17) -> EarlsPokemonAcademy warp 0
VioletCity warp 3: (3,15)  -> VioletNicknameSpeechHouse warp 0
VioletCity warp 4: (31,25) -> VioletPokecenter1F warp 0
VioletCity warp 5: (21,29) -> VioletKylesHouse warp 0
VioletCity warp 6: (23,5)  -> SproutTower1F warp 0
```
Each stub interior has a return warp back to its corresponding VioletCity warp index.

---

## Connection Topology

### Route 31 Connections
```
South -> Route30 (offset: 10)
  - Player walks south off Route31 -> arrives at Route30 top edge, x += 10
  - Already implemented in Sprint 4: Route30 has north->Route31(offset -10)
  - Bidirectional: Route30 north offset=-10, Route31 south offset=10 (symmetric)

West -> VioletCity (offset: -9)
  - NOTE: In pokecrystal, Route31 has "connection west, VioletCity, VIOLET_CITY, -9"
  - However, the actual traversal uses the gate building warps (Route31 -> Gate -> VioletCity)
  - The map connection is a fallback. We implement it but the gate warps handle normal traffic.
```

### Violet City Connections
```
South -> Route32 (offset: 0) -- stub target
East  -> Route31 (offset: 9)
West  -> Route36 (offset: 0) -- stub target
```

---

## New Species Data Required

From `pokecrystal-master/data/pokemon/base_stats/` and `evos_attacks.asm`:

| Species | ID | Type1 | Type2 | HP | Atk | Def | Spd | SpA | SpD | CatchRate | BaseExp | Growth | Learnset (Lv 1-6) |
|---------|-----|-------|-------|-----|-----|-----|-----|-----|-----|-----------|---------|--------|-------------------|
| Bellsprout | 69 | Grass | Poison | 50 | 75 | 35 | 40 | 70 | 30 | 255 | 84 | MediumSlow | VineWhip(1), Growth(6) |
| Gastly | 92 | Ghost | Poison | 30 | 35 | 30 | 80 | 100 | 35 | 190 | 95 | MediumSlow | Hypnosis(1), Lick(1) |

Note: Kakuna appears in Route 30 encounters but was NOT added in Sprint 4 (only Metapod was).
Kakuna is NOT in Route 31 encounters so it's not needed for Sprint 5 either.

---

## Violet City Stub Buildings (7 total)

All Violet City interior buildings are stubs for Sprint 5. Each needs:
- A MapId variant
- A minimal builder function (8x8 or 10x8 room with return warp)
- A load_map() entry

**Building dimensions** (from pokecrystal map_constants):
- VioletMart: standard mart (8x8)
- VioletGym: gym layout (10x10) -- NOTE: will be fleshed out in a later sprint
- EarlsPokemonAcademy: 8x8
- VioletNicknameSpeechHouse: 8x8
- VioletPokecenter1F: standard pokecenter (10x8)
- VioletKylesHouse: 8x8
- SproutTower1F: larger, 10x10 -- stub only

For Sprint 5, all use `build_stub_house()` or similar minimal template with:
- Correct MapId
- Return warp to VioletCity at the correct warp index
- Empty NPCs/events (will be populated in future sprints)

---

## Phased Implementation Plan

### Phase 1: Data Layer (~80 lines in data.rs)

**Goal**: New species, moves, and item constants compile.

1. Add species ID constants: BELLSPROUT(69), GASTLY(92)
2. Add move ID constants: MOVE_VINE_WHIP(22), MOVE_HYPNOSIS(95), MOVE_LICK(122)
3. Add SpeciesData statics for Bellsprout and Gastly (base stats from pokecrystal)
4. Add MoveData entries for new moves
5. Add item constants: ITEM_PP_UP, ITEM_RARE_CANDY, ITEM_PRZ_CURE_BERRY
6. Add music constants: MUSIC_VIOLET_CITY, MUSIC_ROUTE_31
7. Update species_data() and move_data() match arms

**Tests**: species_data lookup for Bellsprout/Gastly, move_data for new moves, Pokemon::new() for Bellsprout at level 5 knows VineWhip.

### Phase 2: Maps Layer (~500 lines in maps.rs)

**Goal**: All new maps load correctly with proper dimensions, warps, NPCs, encounters.

1. Add MapId variants: Route31VioletGate, VioletCity, VioletMart, VioletGym, EarlsPokemonAcademy, VioletNicknameSpeechHouse, VioletPokecenter1F, VioletKylesHouse, SproutTower1F, DarkCaveVioletEntrance, Route32, Route36
2. Replace `build_route31_stub()` with full `build_route31()`:
   - 40x18 tiles, grass patches, 3 warps, 7 NPCs, 2 bg_events
   - Wild encounter table (morning/day/night, 7 slots each, encounter_rate 10)
   - Connections: south->Route30(10), west->VioletCity(-9)
   - import BELLSPROUT, GASTLY at top of maps.rs
3. Implement `build_route31_violet_gate()`:
   - 10x8, standard gate layout, 4 warps (bidirectional), 2 NPCs
4. Implement `build_violet_city()`:
   - 40x36, 9 warps, 8 NPCs, 7 bg_events, connections south+west+east
   - Building blocks for visual structure (approximate)
5. Implement `build_route31_encounters()` encounter table helper
6. Implement 7 Violet City stub buildings + 3 stub routes (DarkCave, Route32, Route36)
7. Update `load_map()` dispatcher with all new maps

**Tests**: map dimensions (Route31=40x18, Gate=10x8, VioletCity=40x36), warp bidirectional consistency for Route31<->Gate<->VioletCity, NPC count per map, encounter table slot counts, connections.

### Phase 3: Events Layer (~200 lines in events.rs)

**Goal**: All scripts compile and flag/scene state transitions work.

1. Add event flag constants (EVENT_BEAT_BUG_CATCHER_WADE through EVENT_EARLS_ACADEMY_EARL)
2. Add script ID constants (~20 IDs, 400-series for Route 31, 420-series for gate, 430-440-series for Violet City)
3. Build script functions:
   - `build_trainer_wade_script()` -- Wade trainer battle (4-mon party: Caterpie/2 x3 + Weedle/3)
   - `build_route31_mail_recipient_script()` -- sleepy man dialogue (stub)
   - `build_route31_youngster_script()` -- dialogue about Falkner
   - `build_route31_cooltrainer_m_script()` -- Dark Cave dialogue
   - `build_violet_city_earl_script()` -- Earl YesNo interaction (simplified)
   - Simple dialogue scripts for Violet City NPCs (5 scripts)
   - Sign scripts (8 scripts)
   - Item ball scripts (4 scripts: Route31 Potion, Route31 PokeBall, Violet PP Up, Violet Rare Candy)
   - Fruit tree scripts (2: Route31, Violet City)
4. Register all scripts in `get_script()`

**Tests**: Wade script loads trainer party correctly, item ball scripts check/set flags, sign scripts produce text.

### Phase 4: Integration in mod.rs (~30 lines)

**Goal**: Map transitions, callbacks, and blackout point work.

1. Add VioletCity to `check_map_callbacks()` (set ENGINE_FLYPOINT_VIOLET)
2. Update `warp_to_last_pokecenter()` to include Violet Pokecenter
3. Verify all map transitions work:
   - Route30 -> Route31 (north connection, offset 10)
   - Route31 -> Route31VioletGate (warp at 4,6/4,7)
   - Route31VioletGate -> VioletCity (warp at 0,4/0,5)
   - VioletCity -> stub buildings (all 7 warps)
   - VioletCity -> Route31 (east connection, offset 9)

**Tests**: full path test (Route30 -> Route31 -> Gate -> VioletCity), flypoint flag set on VioletCity entry, blackout routing.

### Phase 5: Polish + QA (~20 lines)

1. Ensure all terrain is correct (grass tiles, walls, water)
2. Verify Wade trainer sight triggers at range 5
3. Verify all item balls check their event flags
4. Run full `cargo test` pass
5. Check no compilation warnings

---

## Estimated Change Summary

| File | Lines Added (est.) | Key Changes |
|------|-------------------|-------------|
| `data.rs` | ~80 | 2 species data + 3 move data + item/music constants |
| `maps.rs` | ~500 | build_route31(), build_route31_violet_gate(), build_violet_city(), 10 stub builders, encounter table |
| `events.rs` | ~200 | ~20 scripts, ~14 event flags, ~20 script IDs |
| `overworld.rs` | ~0 | No changes (Sprint 4 trainer system handles Wade) |
| `battle.rs` | ~0 | No changes (Sprint 4 multi-party system handles Wade) |
| `mod.rs` | ~30 | Violet flypoint callback, blackout routing, new map dispatching |
| **Total** | **~810** | |

---

## Architectural Decisions

### Violet City Stubs vs Full Implementation

Sprint 5 implements Violet City as an **exterior only**. All 7 interior buildings are stubs with
return warps but no NPCs or scripts. This is the right scope because:
- The gym (Falkner) is a major feature requiring its own sprint
- Sprout Tower is a multi-floor dungeon
- Pokecenter/Mart need heal/shop systems already working from Sprint 2
- The exterior + connections give the player a full path from Route 30 to Violet City

### Wade as the Only Trainer

Route 31 has only 1 trainer (Bug Catcher Wade), unlike Route 30's 3 trainers. This keeps
Sprint 5 focused on map topology rather than battle content. Wade uses the same trainer
system from Sprint 4.

### Bellsprout + Gastly: Two New Species Only

Route 31's encounter tables introduce exactly 2 new species not in any prior route:
- Bellsprout (Grass/Poison) -- morning/day/night grass encounters
- Gastly (Ghost/Poison) -- night-only encounters

All other species in Route 31 encounters (Caterpie, Pidgey, Weedle, Hoppip, Spinarak, Poliwag,
Hoothoot, Zubat, Ledyba) are already defined from Sprints 2 and 4.

### NpcMoveType.SpinSlow

Violet City uses `SPRITEMOVEDATA_SPINRANDOM_SLOW` for several NPCs (Earl, Youngster).
The existing `SpinRandom` variant covers this -- the "slow" part just means a longer wander
timer. We can use `SpinRandom` with a longer timer or add a new variant. Recommended:
just use `SpinRandom` and accept the minor timing difference.

### Explicitly Deferred
- Violet Gym (Falkner) -- separate sprint
- Sprout Tower -- separate sprint (multi-floor dungeon)
- Violet Pokecenter heal functionality (already works from Sprint 2 heal system)
- Violet Mart shop items
- Earl's Academy interior NPCs
- Kenya mail sidequest completion (requires Route 35)
- Dark Cave interior (requires Flash HM)
- Route 32 + Route 36 full implementation
- Mom phone call callback on Route 31
