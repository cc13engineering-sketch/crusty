# Sprint 4 Revised Plan: Route 30 + Mr. Pokemon's House

> Bilbo's review + revised implementation plan. Incorporates Gandalf's architecture
> with corrections, compilation-safe code, and phase-by-phase implementation order.

---

## Review Summary

Gandalf's architecture is **solid**. The pokecrystal data has been verified against source
files. I'm making the following modifications:

### Modifications

1. **M1 (Critical/Compilation)**: Architecture proposes `Vec<(SpeciesId, u8)>` for trainer
   party in TrainerDef. Our system uses `Vec<Pokemon>` elsewhere. Use `Vec<(SpeciesId, u8)>`
   as the definition and construct `Pokemon::new()` at battle start — keeps data lightweight.

2. **M2 (Critical/Compilation)**: Architecture proposes adding `enemy_party: Vec<Pokemon>`
   and `enemy_index: usize` to BattleState. This requires changing the existing `enemy: Pokemon`
   field to work with the party system. Must keep backward-compatible: single-mon battles still
   work by having a 1-element party.

3. **M3 (Critical/Compilation)**: Architecture recommends Option A (sight-range in overworld.rs)
   for trainer detection. For Sprint 4, use a **hybrid approach**: add `trainer_range: Option<u8>`
   to NpcDef, and check sight in `step_overworld()` AFTER player finishes walking. Return a new
   `OverworldResult::TrainerBattle` variant. This avoids touching coord_events and is reusable.

4. **M4 (Data Accuracy)**: Growth rates verified against pokecrystal:
   - Caterpie: MediumFast (not MediumSlow)
   - Metapod: MediumFast
   - Weedle: MediumFast
   - Zubat: MediumFast
   - Poliwag: MediumSlow
   - Ledyba: Fast
   - Spinarak: Fast

5. **M5 (Data Accuracy)**: Catch rates verified: Caterpie=255, Metapod=120, Weedle=255,
   Zubat=255, Poliwag=255, Ledyba=255, Spinarak=255.

6. **M6 (Data Accuracy)**: Architecture says Caterpie learnset is "Tackle(1), StringShot(1)"
   — confirmed correct. But at level 3-4, the moves they know are important for battle testing.
   Poliwag at level 4 only knows Bubble(1). Zubat at level 3 only knows LeechLife(1).
   Spinarak at level 3 knows PoisonSting(1)+StringShot(1).

7. **M7 (Structural)**: The `OverworldResult::TrainerBattle` must carry enough data for mod.rs
   to create the BattleState. Include `trainer_party: Vec<(SpeciesId, u8)>`, `battle_type: BattleType`,
   and `beaten_flag: u16` so mod.rs can set the flag after victory.

8. **M8 (Structural)**: Mr. Pokemon cutscene must set the rival's stolen pokeball flag correctly.
   From pokecrystal: if player got Totodile -> rival stole Chikorita (set CHIKORITA ball flag);
   if player got Chikorita -> rival stole Cyndaquil (set CYNDAQUIL ball flag); otherwise ->
   set TOTODILE ball flag. This is ALREADY handled by our existing pokeball flags — the architecture
   just needs to set the REMAINING ball flag (the one the player didn't pick and the rival didn't steal).

9. **M9 (Structural)**: New `MapId` variants need to be added BEFORE Route30BerryHouse and
   MrPokemonsHouse builders. Route31 stub also needed as connection target.
   The MapId enum must maintain stable ordering for SceneState indexing.

10. **M10 (Structural)**: Architecture says Berry House NPC gives Berry on first visit.
    We already have ITEM_BERRY constant. The script needs CheckEvent + GiveItem pattern
    (same as Mystic Water guy in Sprint 2).

11. **M11 (Move Data)**: Need to add `MOVE_BUBBLE` for Poliwag. Architecture lists Bubble
    as a move Poliwag knows at level 1 but doesn't include it in the new moves list.
    Bubble: Water type, power 20, accuracy 100, pp 30, is_special=true.

12. **M12 (Script)**: The Mr. Pokemon cutscene is complex with Oak's walk + dialogue + flags.
    Split into two script builders: `build_mr_pokemon_meet_script()` for the full entry scene
    including Oak, and `build_mr_pokemon_talk_script()` for post-scene conversations.

---

## Implementation Phases

### Phase 1: data.rs — New Species + Moves + Items (~200 lines)

**Species ID Constants:**
```rust
pub const CATERPIE: SpeciesId = 10;
pub const METAPOD: SpeciesId = 11;
pub const WEEDLE: SpeciesId = 13;
pub const ZUBAT: SpeciesId = 41;
pub const POLIWAG: SpeciesId = 60;
pub const LEDYBA: SpeciesId = 165;
pub const SPINARAK: SpeciesId = 167;
```

**Move ID Constants:**
```rust
pub const MOVE_STRING_SHOT: MoveId = 81;
pub const MOVE_POISON_STING: MoveId = 40;
pub const MOVE_HARDEN: MoveId = 106;
pub const MOVE_LEECH_LIFE: MoveId = 141;
pub const MOVE_CONSTRICT: MoveId = 132;
pub const MOVE_BUBBLE: MoveId = 145;
pub const MOVE_SUPERSONIC: MoveId = 48;
```

**Item Constants:**
```rust
pub const ITEM_MYSTERY_EGG: u8 = 130;  // key item, not consumed
```

**Music Constants:**
```rust
pub const MUSIC_PROF_OAK: u8 = 13;
pub const MUSIC_JOHTO_TRAINER_BATTLE: u8 = 14;
```

**Move Data** — add statics + match arms in `move_data()`:

| Move | Type | Power | Acc | PP | Special? |
|------|------|-------|-----|----|----------|
| STRING_SHOT | Bug | 0 | 95 | 40 | false |
| POISON_STING | Poison | 15 | 100 | 35 | false |
| HARDEN | Normal | 0 | 100 | 30 | false |
| LEECH_LIFE | Bug | 20 | 100 | 15 | false |
| CONSTRICT | Normal | 10 | 100 | 35 | false |
| BUBBLE | Water | 20 | 100 | 30 | true |
| SUPERSONIC | Normal | 0 | 55 | 20 | false |

**Species Data** — add learnsets + statics + match arms in `species_data()`:

```rust
static CATERPIE_LEARNSET: &[(u8, MoveId)] = &[
    (1, MOVE_TACKLE), (1, MOVE_STRING_SHOT),
];
static CATERPIE_DATA: SpeciesData = SpeciesData {
    id: CATERPIE, name: "CATERPIE", type1: PokemonType::Bug, type2: PokemonType::Bug,
    base_hp: 45, base_attack: 30, base_defense: 35, base_speed: 45,
    base_sp_attack: 20, base_sp_defense: 20, catch_rate: 255, base_exp: 53,
    growth_rate: GrowthRate::MediumFast, learnset: CATERPIE_LEARNSET,
};

static METAPOD_LEARNSET: &[(u8, MoveId)] = &[(1, MOVE_HARDEN)];
static METAPOD_DATA: SpeciesData = SpeciesData {
    id: METAPOD, name: "METAPOD", type1: PokemonType::Bug, type2: PokemonType::Bug,
    base_hp: 50, base_attack: 20, base_defense: 55, base_speed: 30,
    base_sp_attack: 25, base_sp_defense: 25, catch_rate: 120, base_exp: 72,
    growth_rate: GrowthRate::MediumFast, learnset: METAPOD_LEARNSET,
};

static WEEDLE_LEARNSET: &[(u8, MoveId)] = &[
    (1, MOVE_POISON_STING), (1, MOVE_STRING_SHOT),
];
static WEEDLE_DATA: SpeciesData = SpeciesData {
    id: WEEDLE, name: "WEEDLE", type1: PokemonType::Bug, type2: PokemonType::Poison,
    base_hp: 40, base_attack: 35, base_defense: 30, base_speed: 50,
    base_sp_attack: 20, base_sp_defense: 20, catch_rate: 255, base_exp: 52,
    growth_rate: GrowthRate::MediumFast, learnset: WEEDLE_LEARNSET,
};

static ZUBAT_LEARNSET: &[(u8, MoveId)] = &[
    (1, MOVE_LEECH_LIFE), (6, MOVE_SUPERSONIC),
];
static ZUBAT_DATA: SpeciesData = SpeciesData {
    id: ZUBAT, name: "ZUBAT", type1: PokemonType::Poison, type2: PokemonType::Flying,
    base_hp: 40, base_attack: 45, base_defense: 35, base_speed: 55,
    base_sp_attack: 30, base_sp_defense: 40, catch_rate: 255, base_exp: 54,
    growth_rate: GrowthRate::MediumFast, learnset: ZUBAT_LEARNSET,
};

static POLIWAG_LEARNSET: &[(u8, MoveId)] = &[
    (1, MOVE_BUBBLE), (7, MOVE_HYPNOSIS),
];
// Note: at level 4, Poliwag only knows Bubble
static POLIWAG_DATA: SpeciesData = SpeciesData {
    id: POLIWAG, name: "POLIWAG", type1: PokemonType::Water, type2: PokemonType::Water,
    base_hp: 40, base_attack: 50, base_defense: 40, base_speed: 90,
    base_sp_attack: 40, base_sp_defense: 40, catch_rate: 255, base_exp: 77,
    growth_rate: GrowthRate::MediumSlow, learnset: POLIWAG_LEARNSET,
};

static LEDYBA_LEARNSET: &[(u8, MoveId)] = &[
    (1, MOVE_TACKLE), (8, MOVE_SUPERSONIC),
];
static LEDYBA_DATA: SpeciesData = SpeciesData {
    id: LEDYBA, name: "LEDYBA", type1: PokemonType::Bug, type2: PokemonType::Flying,
    base_hp: 40, base_attack: 20, base_defense: 30, base_speed: 55,
    base_sp_attack: 40, base_sp_defense: 80, catch_rate: 255, base_exp: 54,
    growth_rate: GrowthRate::Fast, learnset: LEDYBA_LEARNSET,
};

static SPINARAK_LEARNSET: &[(u8, MoveId)] = &[
    (1, MOVE_POISON_STING), (1, MOVE_STRING_SHOT),
    (6, MOVE_SCARY_FACE), (11, MOVE_CONSTRICT),
];
// Note: ScaryFace not implemented as a move — Spinarak at level 3 only has PoisonSting+StringShot
static SPINARAK_DATA: SpeciesData = SpeciesData {
    id: SPINARAK, name: "SPINARAK", type1: PokemonType::Bug, type2: PokemonType::Poison,
    base_hp: 40, base_attack: 60, base_defense: 40, base_speed: 30,
    base_sp_attack: 40, base_sp_defense: 40, catch_rate: 255, base_exp: 54,
    growth_rate: GrowthRate::Fast, learnset: SPINARAK_LEARNSET,
};
```

Note: Spinarak's ScaryFace (at level 6) is a status move we don't need to implement as
move data — it has 0 power and won't be used in auto-battle. We can add it as a stub or
omit it. Since Spinarak at encounter level 3 won't have it, it's safe to omit for now.

Update `species_data()` match to include all 7 new species.

### Phase 2: maps.rs — New Maps + Route 30 Full (~250 lines)

**MapId additions** (add BEFORE Route30 in the enum to maintain ordering):
```rust
Route30BerryHouse,
MrPokemonsHouse,
Route31,   // stub
```

**NpcDef addition** — add trainer_range field:
```rust
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
    pub trainer_range: Option<u8>,  // NEW: sight range for trainer battles
}
```

**CRITICAL**: Every existing NpcDef construction must add `trainer_range: None`.
This affects ALL map builders. Use find-and-replace to add `trainer_range: None` to
every NpcDef literal.

**build_route30()** — replace stub with full 20x54 map:
- Terrain: grass patches, walls forming corridor, water on east (C_WATER)
- 2 warps: (7,39)->Route30BerryHouse warp 0, (17,5)->MrPokemonsHouse warp 0
- 11 NPCs per pokecrystal Route30.asm (verified)
- 5 bg_events per pokecrystal
- Wild encounters (same encounter_rate=10 for all periods)
- Connections: north->Route31(offset -10), south->CherrygroveCity(offset -5)
- Trainer NPCs get trainer_range: Joey(3), Mikey(1), Don(3)

**build_route30_berry_house()** — 8x8 standard house:
- 2 warps: (2,7) and (3,7) -> Route30, warp 0
- 1 NPC: POKEFAN_M at (2,3), StandingDown
- 2 bg_events: bookshelves at (0,1) and (1,1)

**build_mr_pokemons_house()** — 8x8 facility:
- 2 warps: (2,7) and (3,7) -> Route30, warp 1
- 2 NPCs: MR_POKEMON (GENTLEMAN) at (3,5) StandingRight, OAK at (6,5) StandingUp
- Oak: event_flag=EVENT_MR_POKEMONS_HOUSE_OAK, event_flag_show=false (disappears when flag SET)
- 5 bg_events per pokecrystal

**build_route31_stub()** — minimal connection target:
- 20x20, all floor, no encounters, connection south->Route30

**load_map()** match — add Route30BerryHouse, MrPokemonsHouse, Route31, and replace
build_route30_stub() with build_route30().

**Route 30 encounter data:**
```rust
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
```

Import the new species constants from data.rs.

### Phase 3: events.rs — New Flags + Scripts (~200 lines)

**New event flags** (32-42):
```rust
pub const EVENT_BEAT_YOUNGSTER_JOEY: u16 = 32;
pub const EVENT_BEAT_YOUNGSTER_MIKEY: u16 = 33;
pub const EVENT_BEAT_BUG_CATCHER_DON: u16 = 34;
pub const EVENT_ROUTE_30_BATTLE: u16 = 35;
pub const EVENT_ROUTE_30_YOUNGSTER_JOEY: u16 = 36;
pub const EVENT_ROUTE_30_ANTIDOTE: u16 = 37;
pub const EVENT_ROUTE_30_HIDDEN_POTION: u16 = 38;
pub const EVENT_GOT_BERRY_FROM_ROUTE_30_HOUSE: u16 = 39;
pub const EVENT_GOT_MYSTERY_EGG_FROM_MR_POKEMON: u16 = 40;
pub const EVENT_MR_POKEMONS_HOUSE_OAK: u16 = 41;
pub const EVENT_JOEY_ASKED_FOR_PHONE_NUMBER: u16 = 42;
```

**New scene constants:**
```rust
pub const SCENE_MRPOKEMONSHOUSE_MEET_MR_POKEMON: u8 = 0;
pub const SCENE_MRPOKEMONSHOUSE_NOOP: u8 = 1;
```

**New script IDs** (300-334):
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
pub const SCRIPT_ROUTE30_HIDDEN_POTION: u16 = 313;

// Berry House scripts
pub const SCRIPT_BERRY_HOUSE_POKEFAN: u16 = 320;
pub const SCRIPT_BERRY_HOUSE_BOOKSHELF: u16 = 321;

// Mr. Pokemon's House scripts
pub const SCRIPT_MR_POKEMON: u16 = 330;
pub const SCRIPT_MR_POKEMON_MEET: u16 = 331;
pub const SCRIPT_MR_POKEMON_MAGAZINES: u16 = 332;
pub const SCRIPT_MR_POKEMON_COMPUTER: u16 = 333;
pub const SCRIPT_MR_POKEMON_COINS: u16 = 334;
```

**New ScriptStep variant for trainer battles:**
```rust
ScriptStep::StartTrainerBattle {
    party: Vec<(SpeciesId, u8)>,
    beaten_flag: u16,
    seen_text: String,
},
```

Actually — simpler approach. Trainer scripts use existing StartBattle with BattleType::Normal.
The trainer party info is passed through the script state. Add a new field to ScriptState:
```rust
pub trainer_party: Option<Vec<(SpeciesId, u8)>>,
pub trainer_beaten_flag: Option<u16>,
```

And a new ScriptStep:
```rust
ScriptStep::LoadTrainerParty {
    party: Vec<(SpeciesId, u8)>,
    beaten_flag: u16,
},
```

This mirrors the LoadWildMon -> StartBattle pattern already in place.

**Script builders:**

Trainer scripts follow the pattern:
1. FacingPlayer
2. ShowText (seen text)
3. LoadTrainerParty (species/level pairs + beaten flag)
4. StartBattle { battle_type: BattleType::Normal }
5. ShowText (beaten text)
6. End

Berry House: CheckEvent -> GiveItem + SetEvent, or after-text.

Mr. Pokemon meet scene (SCRIPT_MR_POKEMON_MEET / scene 0 entry):
1. ShowEmote { npc_idx: 0, emote: Shock, frames: 15 }
2. TurnNpc { npc_idx: 0, direction: Down }
3. ShowText (intro texts)
4. MovePlayer { steps: [(Right, 1), (Up, 1)] }
5. ShowText (egg explanation)
6. GiveItem { ITEM_MYSTERY_EGG, 1 }
7. SetEvent(EVENT_GOT_MYSTERY_EGG_FROM_MR_POKEMON)
8. ShowText (day-care couple)
9. TurnNpc { npc_idx: 0, direction: Right }
10. ShowText (introduces Oak)
11. TurnNpc { npc_idx: 0, direction: Down }
12. TurnNpc { npc_idx: 1, direction: Left } // Oak turns to player
13. ShowText (Oak's speech intro)
14. PlayMusic(MUSIC_PROF_OAK)
15. MoveNpc { npc_idx: 1, steps: [(Down, 1), (Left, 2)] }
16. TurnPlayer(Right)
17. ShowText (Oak long dialogue + Pokedex)
18. SetEvent(EVENT_ENGINE_POKEDEX) // using setflag ENGINE_POKEDEX
19. ShowText (Oak goodbye)
20. TurnPlayer(Down)
21. MoveNpc { npc_idx: 1, steps: [(Down, 1), (Left, 1)] }
22. HideNpc(1) // Oak disappears
23. SetEvent(EVENT_MR_POKEMONS_HOUSE_OAK)
24. Special(RestartMapMusic)
25. Pause(0.25)
26. TurnPlayer(Up)
27. ShowText (Mr. Pokemon offers heal)
28. Heal
29. ShowText ("I'm depending on you!")
30. SetEvent(EVENT_RIVAL_NEW_BARK_TOWN)
31. SetEvent(EVENT_PLAYERS_HOUSE_1F_NEIGHBOR)
32. ClearEvent(EVENT_PLAYERS_NEIGHBORS_HOUSE_NEIGHBOR) // need new flag constant if missing
33. SetScene { map: MrPokemonsHouse, scene_id: SCENE_MRPOKEMONSHOUSE_NOOP }
34. SetScene { map: CherrygroveCity, scene_id: SCENE_CHERRYGROVECITY_MEET_RIVAL }
35. SetScene { map: ElmsLab, scene_id: SCENE_ELMSLAB_MEET_OFFICER }
36. ClearEvent(EVENT_COP_IN_ELMS_LAB) // officer appears
37. End

Note: EVENT_PLAYERS_NEIGHBORS_HOUSE_NEIGHBOR may need to be added as a new event flag if
not already defined. Check — we have EVENT_PLAYERS_HOUSE_1F_NEIGHBOR=16 but not the neighbor's
house one. Add it.

**get_script() additions:** Add match arms for all new script IDs.

### Phase 4: battle.rs — Multi-Pokemon Enemy Parties (~50 lines)

Replace the single `enemy: Pokemon` with a party system:

```rust
pub struct BattleState {
    pub enemy_party: Vec<Pokemon>,   // full enemy team
    pub enemy_index: usize,          // current enemy mon
    pub battle_type: BattleType,
    pub turn_count: u8,
    pub phase: BattlePhase,
    pub message: Option<String>,
    pub message_timer: f64,
    pub result: Option<BattleResult>,
    pub beaten_flag: Option<u16>,    // flag to set on victory
}
```

Add a helper `pub fn current_enemy(&self) -> &Pokemon` and `current_enemy_mut(&mut self) -> &mut Pokemon`.

**new_wild()** creates a 1-element party. **new_trainer()** becomes:
```rust
pub fn new_trainer(party: Vec<(SpeciesId, u8)>, battle_type: BattleType, beaten_flag: Option<u16>) -> Self {
    let enemy_party: Vec<Pokemon> = party.iter().map(|&(sp, lv)| Pokemon::new(sp, lv)).collect();
    Self {
        enemy_party,
        enemy_index: 0,
        battle_type,
        turn_count: 0,
        phase: BattlePhase::Intro,
        message: None,
        message_timer: 0.0,
        result: None,
        beaten_flag,
    }
}
```

**step_battle() changes:**
- Replace `battle.enemy` with `battle.current_enemy()` / `battle.current_enemy_mut()`
- When current enemy HP reaches 0, check if `enemy_index + 1 < enemy_party.len()`:
  - If yes: advance `enemy_index`, show "Trainer sent out X!" message
  - If no: battle won

**Backward compatibility:** All existing callers that used `new_wild(species, level, type)`
now construct `enemy_party: vec![Pokemon::new(species, level)]`. Update `new_wild()`:
```rust
pub fn new_wild(species: SpeciesId, level: u8, battle_type: BattleType) -> Self {
    Self {
        enemy_party: vec![Pokemon::new(species, level)],
        enemy_index: 0,
        battle_type, turn_count: 0, phase: BattlePhase::Intro,
        message: None, message_timer: 0.0, result: None, beaten_flag: None,
    }
}
```

**Existing code references `battle.enemy`** — all must change to `battle.current_enemy()`
or `battle.enemy_party[battle.enemy_index]`. This is the biggest refactor risk. Search for
all `battle.enemy` references in battle.rs and mod.rs.

### Phase 5: overworld.rs — Trainer Sight Detection (~40 lines)

Add a new OverworldResult variant:
```rust
OverworldResult::TrainerBattle {
    npc_idx: u8,
    script_id: u16,
},
```

Add trainer sight check in `step_overworld()`, after player finishes walking (when
`player.walk_offset >= TILE_PX` resolves):

```rust
// After coord event check, before returning Nothing:
// Check trainer line-of-sight
for (i, npc_def) in map.npcs.iter().enumerate() {
    if let Some(range) = npc_def.trainer_range {
        // Skip if beaten flag is set
        if let Some(flag) = npc_def.event_flag {
            if _flags.has(flag) { continue; }
        }
        if let Some(state) = npc_states.get(i) {
            if !state.visible { continue; }
            if is_in_sight(state, player, range) {
                return OverworldResult::TrainerBattle {
                    npc_idx: i as u8,
                    script_id: npc_def.script_id,
                };
            }
        }
    }
}
```

Add `is_in_sight()` helper:
```rust
fn is_in_sight(npc: &NpcState, player: &PlayerState, range: u8) -> bool {
    let range = range as i32;
    match npc.facing {
        Direction::Up    => npc.x == player.x && player.y >= npc.y - range && player.y < npc.y,
        Direction::Down  => npc.x == player.x && player.y > npc.y && player.y <= npc.y + range,
        Direction::Left  => npc.y == player.y && player.x >= npc.x - range && player.x < npc.x,
        Direction::Right => npc.y == player.y && player.x > npc.x && player.x <= npc.x + range,
    }
}
```

Also: rename `_flags` parameter to `flags` (remove underscore) since we now use it.

### Phase 6: mod.rs — Wiring + Mr. Pokemon Entry Script (~40 lines)

**Handle new OverworldResult::TrainerBattle:**
```rust
OverworldResult::TrainerBattle { npc_idx, script_id } => {
    // Trigger trainer script (which contains LoadTrainerParty + StartBattle)
    let steps = get_script(script_id);
    self.script = Some(ScriptState::new(steps));
    self.phase = GamePhase::Script;
}
```

**Handle ScriptResult::StartBattle changes for trainer parties:**
In step_script_phase, when StartBattle with BattleType::Normal:
```rust
BattleType::Normal => {
    // Use trainer_party from script state
    if let Some(ref script) = self.script {
        if let Some(ref party) = script.trainer_party {
            let battle_state = battle::BattleState::new_trainer(
                party.clone(), battle_type,
                script.trainer_beaten_flag,
            );
            self.battle = Some(battle_state);
            self.phase = GamePhase::Battle;
        }
    }
}
```

**Mr. Pokemon house entry script:**
In `check_map_entry_scripts()`:
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

**Handle trainer beaten flag after battle:**
In `step_battle_phase()`, after victory:
```rust
Some(BattleResult::Won) => {
    if let Some(ref battle_state) = self.battle {
        if let Some(flag) = battle_state.beaten_flag {
            self.event_flags.set(flag);
        }
    }
    // ... existing logic
}
```

**Update check_map_callbacks():**
```rust
MapId::Route30 => {
    // No special callbacks needed for Sprint 4
}
```

**Update battle.enemy references:**
All references to `battle.enemy` in mod.rs (in step_battle_phase) must use
`battle.enemy_party[battle.enemy_index]` or the helper method.

### Phase 7: Tests (~25 tests)

**data.rs tests:**
- test_new_species_data: verify all 7 species load with correct types, stats
- test_new_moves_data: verify all 7 new moves
- test_caterpie_learnset: level 3 Caterpie has Tackle + StringShot
- test_poliwag_learnset: level 4 Poliwag has only Bubble

**maps.rs tests:**
- test_route30_dimensions: 20x54
- test_route30_has_wild_encounters: 7 slots per period
- test_route30_connections: north=Route31, south=CherrygroveCity
- test_route30_warps: 2 warps to houses
- test_route30_npcs: 11 NPCs, trainers have trainer_range
- test_route30_berry_house: 8x8, 1 NPC, 2 warps
- test_mr_pokemons_house: 8x8, 2 NPCs, 2 warps, 5 bg_events
- test_route30_bidirectional_warps: houses warp back to Route30
- test_route31_stub: exists as connection target

**events.rs tests:**
- test_trainer_scripts_compile: all new script IDs return non-empty steps
- test_berry_house_gives_berry_once: CheckEvent + GiveItem pattern
- test_mr_pokemon_meet_script_sets_flags: verify critical flags set
- test_mr_pokemon_scene_sets_rival_cherrygrove: scene chain

**battle.rs tests:**
- test_multi_pokemon_party: trainer with 2 mons, both must faint for victory
- test_single_pokemon_still_works: backward compat with Vec<Pokemon> of length 1
- test_trainer_joey_party: Rattata/4, ends in victory

**mod.rs tests:**
- test_mr_pokemons_house_entry_triggers_scene
- test_route30_map_loads_with_all_variants: ensure MapId enum additions don't break

---

## Compilation Risk Areas

1. **battle.rs `enemy` -> `enemy_party[enemy_index]`**: Must find/replace ALL references.
   The `battle.enemy.hp`, `battle.enemy.species`, etc. all need updating.

2. **NpcDef `trainer_range` field**: Every NpcDef literal in maps.rs needs the new field.
   ~60+ NpcDef constructions across all map builders.

3. **MapId enum ordering**: Adding Route30BerryHouse/MrPokemonsHouse/Route31 changes the
   integer values of subsequent MapId variants. SceneState uses `map as usize`, so if Route30
   was previously the last variant (index 16), adding 3 more variants AFTER it is fine.
   But Route30BerryHouse and MrPokemonsHouse must be added AFTER Route30 in the enum.

4. **ScriptState new fields**: `trainer_party` and `trainer_beaten_flag` must be initialized
   in `ScriptState::new()` as None.

5. **Import statements**: data.rs new constants need importing in maps.rs, events.rs, etc.

---

## Implementation Order

Execute phases in this order to minimize broken intermediate states:

1. **Phase 1 (data.rs)** — pure additions, no existing code touched
2. **Phase 4 (battle.rs)** — refactor enemy->enemy_party BEFORE adding new maps/scripts
3. **Phase 2 (maps.rs)** — add NpcDef field + new maps (requires Phase 1 for species constants)
4. **Phase 3 (events.rs)** — add flags + scripts (requires Phase 1 for species, Phase 4 for battle)
5. **Phase 5 (overworld.rs)** — trainer sight detection
6. **Phase 6 (mod.rs)** — wire everything together
7. **Phase 7 (tests)** — verify

Run `cargo check` after each phase. Run `cargo test` after Phase 7.
