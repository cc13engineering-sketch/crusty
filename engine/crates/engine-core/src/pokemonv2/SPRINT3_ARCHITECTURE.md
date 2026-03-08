# Sprint 3 QA Architecture

QA audit for Groups 1-2: verify the complete starting area from player bedroom through Cherrygrove City.

---

## Testability Assessment

### What We CAN Test Without `Engine`

Most of the game logic is testable directly. The key observation: `PokemonV2Sim::with_state()` already provides a test-ready constructor that bypasses the title screen. The subsystems are well-factored:

| System | Testable Without Engine? | How |
|--------|--------------------------|-----|
| Map loading (dimensions, tiles, collision) | Yes | `load_map()` is pure |
| Warp definitions & bidirectionality | Yes | `load_map()` + iterate warps |
| NPC counts, positions, event flags | Yes | `load_map().npcs` |
| Coord events (position, scene_id, script_id) | Yes | `load_map().coord_events` |
| BG events (position, kind, script_id) | Yes | `load_map().bg_events` |
| Wild encounter tables | Yes | `load_map().wild_encounters` |
| Event flag operations | Yes | `EventFlags::new/set/has/clear` |
| Scene state operations | Yes | `SceneState::new/set/get` |
| Script step sequences | Yes | `get_script()` + inspect steps |
| Script execution engine | Yes | `step_script()` takes refs, no Engine needed |
| Pokemon stats at level | Yes | `Pokemon::new()` is pure |
| Battle damage calculation | Yes | `calc_damage()` is pure |
| Battle auto-progression | Yes | `step_battle()` takes refs |
| Map connections | Yes | `load_map().connections` |
| Collision/walkability | Yes | `is_walkable()` / `is_walkable_with_direction()` pure |
| Wild encounter resolution | Yes | `check_wild_encounter()` is pure |
| NPC visibility refresh | Yes | `PokemonV2Sim::refresh_npc_visibility()` |
| Rival species counter | Yes | `PokemonV2Sim::get_rival_species()` |

### What Requires `Engine`

| System | Needs Engine? | Why |
|--------|---------------|-----|
| `step_overworld()` | Yes | reads `engine.input`, `engine.rng` |
| `step()` (full sim) | Yes | calls `step_overworld`, input helpers |
| Walking/movement | Yes | input-driven via `engine.input.keys_held` |
| NPC wandering | Yes | reads `engine.rng` |

However: `Engine::new(160, 144)` is available in tests (used in existing Sprint 1 tests), so full-sim tests are also feasible. We just inject keys into `engine.input.keys_pressed` / `keys_held`.

### Full Headless Playthrough

A complete bedroom-to-Cherrygrove playthrough IS feasible via `PokemonV2Sim::with_state()` + `Engine::new()` + injecting inputs. The existing test `test_warp_bedroom_to_1f` already demonstrates this pattern. However, a full playthrough test would be 100+ frames with careful input sequencing. We should implement it as a dedicated integration-level test.

---

## Test Plan: 10 Verification Groups

All tests go in inline `#[cfg(test)] mod tests` blocks within the file that owns the tested functionality. This keeps tests co-located with their code and is the pattern already established in all modules.

### Group 1: Player Spawn & Bedroom (mod.rs tests)

```
test_player_spawn_position_and_map
  - PokemonV2Sim::new() spawns at PlayersHouse2F (3,3)
  - Phase is TitleScreen
  - Party is empty, money is 3000, badges is 0

test_bedroom_map_structure
  - PlayersHouse2F dimensions: 8x6
  - Exactly 1 warp (staircase at 7,0 -> PlayersHouse1F)
  - 4 NPCs (CONSOLE, DOLL_1, DOLL_2, BIG_DOLL) -- all decorations
  - 4 bg_events (PC, radio, bookshelf, stair area)
  - No wild encounters
  - No coord_events
```

### Group 2: Stair Warp & Mom Cutscene (mod.rs + maps.rs tests)

```
test_house1f_map_structure
  - PlayersHouse1F dimensions: 10x8
  - 3 warps: 2 exit doors (6,7 and 7,7 -> NewBarkTown) + staircase (9,0 -> 2F)
  - 5 NPCs with event flags (MOM1 flag=3, MOM2/3/4 flag=4, NEIGHBOR flag=16)
  - 2 coord_events at (8,4) and (9,4) with scene_id=0

test_meet_mom_script_gives_pokegear
  - get_script(SCRIPT_MEET_MOM) contains SetEvent(EVENT_ENGINE_POKEGEAR)
  - Contains GiveItem { item_id: 59, count: 1 }
  - Contains SetScene { map: PlayersHouse1F, scene_id: SCENE_PLAYERSHOUSE1F_NOOP }
  - Sets EVENT_PLAYERS_HOUSE_MOM_1 and clears EVENT_PLAYERS_HOUSE_MOM_2

test_meet_mom_script_execution
  - Execute SCRIPT_MEET_MOM through step_script()
  - After completion: EVENT_ENGINE_POKEGEAR is set
  - Bag contains item 59
  - Scene for PlayersHouse1F is NOOP (1)
```

### Group 3: New Bark Town & Teacher Block (maps.rs + events.rs tests)

```
test_new_bark_town_full_structure
  - Dimensions: 18x20
  - 4 warps: ElmsLab, PlayersHouse1F, PlayersNeighborsHouse, ElmsHouse
  - 3 NPCs: TEACHER at (6,8), FISHER at (12,9), RIVAL at (3,2) with event_flag=9
  - 2 coord_events at (1,8) and (1,9) with scene_id=0
  - 4 bg_events (signs)
  - Map connections: east->Route27, west->Route29

test_teacher_stops_script_moves_player_right
  - get_script(SCRIPT_TEACHER_STOPS_1) contains MovePlayer { steps: [(Right, 4)] }
  - get_script(SCRIPT_TEACHER_STOPS_2) contains MovePlayer { steps: [(Right, 5)] }

test_teacher_block_coord_events_at_correct_positions
  - Coord events at (1,8) and (1,9) both have scene_id=0 (SCENE_NEWBARKTOWN_TEACHER_STOPS_YOU)
  - script_ids are 2 and 3 (SCRIPT_TEACHER_STOPS_1/2)
```

### Group 4: Elm's Lab & Starter Selection (maps.rs + events.rs + mod.rs tests)

```
test_elms_lab_full_structure
  - Dimensions: 10x12
  - 2 warps to NewBarkTown (at 4,11 and 5,11)
  - 6 NPCs: ELM, AIDE, BALL_CYNDAQUIL(6,3), BALL_TOTODILE(7,3), BALL_CHIKORITA(8,3), OFFICER
  - Pokeball NPCs have event_flags 13/14/15 with event_flag_show=false (hidden when flag set)
  - 8 coord_events (cant-leave + meet-officer + aide-gives + balls)
  - 12 bg_events

test_elms_lab_scene_progression
  - Default scene is 0 (SCENE_ELMSLAB_MEET_ELM)
  - After elm intro: scene becomes 1 (SCENE_ELMSLAB_CANT_LEAVE)
  - Coord events at (4,6)/(5,6) with scene_id=1 -> SCRIPT_LAB_TRY_TO_LEAVE
  - After starter: scene becomes 5 (SCENE_ELMSLAB_AIDE_GIVES_POTION)

test_starter_selection_all_three_choices
  - For each choice (0=Cyndaquil, 1=Totodile, 2=Chikorita):
    - with_state at ElmsLab, set phase to StarterSelect{cursor}
    - Inject confirm (KeyZ)
    - After step: party has 1 Pokemon of correct species at level 5
    - Correct event flags set (EVENT_GOT_<SPECIES>_FROM_ELM + EVENT_GOT_A_POKEMON_FROM_ELM)
    - Correct pokeball flag set (EVENT_<SPECIES>_POKEBALL_IN_ELMS_LAB)
    - ElmsLab scene is SCENE_ELMSLAB_AIDE_GIVES_POTION (5)
    - NewBarkTown scene is SCENE_NEWBARKTOWN_NOOP (1)

test_starter_held_item_berry
  - After starter selection, party[0].held_item == Some(ITEM_BERRY)

test_cant_leave_lab_without_starter
  - Coord events at (4,6)/(5,6) have scene_id=SCENE_ELMSLAB_CANT_LEAVE
  - get_script(SCRIPT_LAB_TRY_TO_LEAVE) contains MovePlayer steps pushing player back
```

### Group 5: Route 29 Traversal (maps.rs + overworld.rs tests)

```
test_route29_full_structure
  - Dimensions: 60x18
  - 1 warp at (27,1) to Route29Route46Gate
  - 8 NPCs: DUDE, YOUNGSTER, TEACHER, FRUIT_TREE, FISHER, COOLTRAINER_M, TUSCANY(event_flag=25), POTION_BALL(event_flag=26)
  - 2 coord_events at (53,8)/(53,9) with scene_id=1 (catching tutorial)
  - 2 bg_events (signs)
  - Connections: north->Route46, east->NewBarkTown, west->CherrygroveCity

test_route29_wild_encounters_by_time_of_day
  - Morning: slots[0]=Pidgey/2, slots[1]=Sentret/2 (7 total)
  - Day: slots[0]=Pidgey/2, slots[1]=Sentret/2 (7 total)
  - Night: slots[0]=Hoothoot/2, slots[1]=Rattata/2 (7 total)
  - Encounter rate = 10
  - All slots have correct species from pokecrystal data

test_route29_grass_tiles_present
  - Multiple grass patches exist in collision map (C_GRASS = 5)
  - Grass tiles are walkable

test_route29_ledge_tiles_one_way
  - Ledge strips (C_LEDGE_D) exist
  - Walkable only when facing Down
  - Not walkable when facing Up/Left/Right

test_wild_encounter_slot_distribution
  - rng_slot=0 -> slot 0 (30%)
  - rng_slot=77 -> slot 1 (30%)
  - rng_slot=154 -> slot 2 (20%)
  - rng_slot=205 -> slot 3 (10%)
  - rng_slot=230 -> slot 4 (5%)
  - rng_slot=243 -> slot 5 (2.5%)
  - rng_slot=249 -> slot 6 (2.5%)

test_catching_tutorial_coord_events
  - Coord events at (53,8) and (53,9) trigger when scene is SCENE_ROUTE29_CATCH_TUTORIAL
  - Script 203/204 loads Hoppip/3 then starts Tutorial battle

test_route29_potion_item_ball
  - NPC "POTION_BALL" at (48,2) has event_flag=26, event_flag_show=false
  - get_script(SCRIPT_ROUTE29_POTION) gives ITEM_POTION and sets EVENT_ROUTE_29_POTION
```

### Group 6: Route 29/46 Gate (maps.rs tests)

```
test_route29_route46_gate_structure
  - Dimensions: 10x8
  - 4 warps: 2 north to Route46, 2 south to Route29
  - 2 NPCs: OFFICER, YOUNGSTER
  - No wild encounters, no coord_events

test_gate_warp_resolution
  - Gate south warps (2,3) point to Route29 warp 0
  - Gate north warps (0,1) point to Route46 warps 1,2
  - Route29 warp at (27,1) points to gate warp 2
```

### Group 7: Cherrygrove City (maps.rs + events.rs tests)

```
test_cherrygrove_city_full_structure
  - Dimensions: 40x18
  - 5 warps: Mart, Pokecenter, GymSpeechHouse, GuideGentsHouse, EvoSpeechHouse
  - 5 NPCs: GUIDE_GENT(flag=18,show=false), RIVAL(flag=19,show=true), TEACHER, YOUNGSTER, MYSTIC_WATER_GUY
  - 2 coord_events at (33,6)/(33,7) with scene_id=1 (rival ambush)
  - 4 bg_events (signs)
  - Connections: north->Route30, east->Route29

test_cherrygrove_flypoint_set_on_entry
  - check_map_callbacks for CherrygroveCity sets EVENT_ENGINE_FLYPOINT_CHERRYGROVE

test_guide_gent_tour_script_correctness
  - Script sets EVENT_ENGINE_MAP_CARD (NOT a bag item -- verified GIMLI fix)
  - Script sets EVENT_GUIDE_GENT_IN_HIS_HOUSE and EVENT_GUIDE_GENT_VISIBLE_IN_CHERRYGROVE
  - Contains Follow/StopFollow and MoveNpc/MovePlayer pairs
  - Contains PlayMusic(MUSIC_SHOW_ME_AROUND) and PlayMapMusic

test_rival_ambush_coord_events
  - Coord events at (33,6) and (33,7) with scene_id=SCENE_CHERRYGROVECITY_MEET_RIVAL
  - Script 231 starts CanLose battle

test_mystic_water_guy_gives_item_once
  - Script 234 checks EVENT_GOT_MYSTIC_WATER_IN_CHERRYGROVE
  - First visit: gives ITEM_MYSTIC_WATER, sets flag
  - Second visit: jumps to alternate text
```

### Group 8: Cherrygrove Buildings (maps.rs tests)

```
test_cherrygrove_pokecenter_structure
  - Dimensions: 10x8
  - 2 warps back to CherrygroveCity
  - 4 NPCs: NURSE, FISHER, GENTLEMAN, TEACHER
  - Nurse script heals party (Special::HealParty)

test_cherrygrove_mart_structure
  - Dimensions: 12x8
  - 2 warps back to CherrygroveCity
  - 3 NPCs: CLERK, COOLTRAINER, YOUNGSTER
  - Clerk script gives free POTION

test_guide_gents_house_structure
  - Dimensions: 8x8
  - 2 warps back to CherrygroveCity
  - 1 NPC: GUIDE_GENT_HOME with event_flag=17, event_flag_show=true
  - 2 bg_events (bookshelves)

test_gym_speech_house_structure
  - Dimensions: 8x8, 2 warps, 2 NPCs, 2 bg_events

test_evo_speech_house_structure
  - Dimensions: 8x8, 2 warps, 2 NPCs, 2 bg_events

test_all_cherrygrove_building_warps_bidirectional
  - Every building warp to CherrygroveCity has a return warp from CherrygroveCity
```

### Group 9: Rival Battle (battle.rs + mod.rs tests)

```
test_rival_species_counters_starter
  - Cyndaquil chosen -> rival gets Totodile
  - Totodile chosen -> rival gets Chikorita
  - Chikorita chosen -> rival gets Cyndaquil

test_rival_battle_is_canlose
  - Cherrygrove rival script uses StartBattle { battle_type: BattleType::CanLose }
  - On loss: player does NOT warp to pokecenter (CanLose special case)

test_rival_battle_completes
  - Create BattleState::new_trainer(rival_species, 5, BattleType::CanLose)
  - Run step_battle() until completion
  - Result is Won, Lost, or Fled (never Caught)

test_canlose_battle_does_not_blackout
  - After BattleResult::Lost with BattleType::CanLose:
    - Phase returns to Script/Overworld (not warp to pokecenter)

test_starter_vs_rival_damage_nonzero
  - For each starter, calc_damage against the counter-rival species deals >0 damage
```

### Group 10: Integration Playthrough (mod.rs tests)

```
test_full_bedroom_to_new_bark_flow
  - Start at PlayersHouse2F (3,3)
  - Walk to stair warp -> transition to PlayersHouse1F
  - Walk to MeetMom coord_event -> script triggers, pokegear given
  - Walk to exit door -> warp to NewBarkTown
  - Verify: on NewBarkTown map at correct warp position

test_new_bark_to_elms_lab_flow
  - Start at NewBarkTown near ElmsLab warp
  - Walk to lab warp -> transition to ElmsLab
  - Map entry script (SCENE_ELMSLAB_MEET_ELM) triggers Elm intro
  - After intro: scene is SCENE_ELMSLAB_CANT_LEAVE

test_starter_to_route29_flow
  - Start at ElmsLab with scene=CANT_LEAVE
  - Select starter (Cyndaquil, cursor=0)
  - Verify party has starter
  - Walk to exit -> should now be allowed (scene is AIDE_GIVES_POTION, not CANT_LEAVE)
  - Map connection or warp to NewBarkTown

test_all_map_entry_scripts_dont_panic
  - For every MapId: load map, init NPC states, call check_map_entry_scripts
  - No panics or out-of-bounds
```

---

## Test File Organization

All tests stay in inline `#[cfg(test)] mod tests` within their respective files:

| File | New Tests Added | Focus |
|------|-----------------|-------|
| `mod.rs` | Groups 1, 2 (partial), 4 (partial), 9 (partial), 10 | Sim-level integration, starter selection, rival |
| `maps.rs` | Groups 1-8 structural tests | Map dimensions, warps, NPCs, coord_events |
| `events.rs` | Groups 2-5, 7 script inspection | Script content verification, flag operations |
| `overworld.rs` | Group 5 encounter/walkability tests | Wild encounters, ledge mechanics |
| `battle.rs` | Group 9 battle tests | Damage calc, canlose behavior, auto-battle |

---

## Estimated Test Count

| Group | Tests | Priority |
|-------|-------|----------|
| 1. Player Spawn & Bedroom | 2 | P0 |
| 2. Stair Warp & Mom | 3 | P0 |
| 3. New Bark Town & Teacher | 3 | P0 |
| 4. Elm's Lab & Starters | 5 | P0 |
| 5. Route 29 Traversal | 7 | P0 |
| 6. Route 29/46 Gate | 2 | P1 |
| 7. Cherrygrove City | 5 | P0 |
| 8. Cherrygrove Buildings | 6 | P1 |
| 9. Rival Battle | 5 | P0 |
| 10. Integration Playthrough | 4 | P0 |
| **Total** | **42** | |

All P0 tests (34) must pass. P1 tests (8) verify secondary structures.

---

## Existing Tests to Keep (Not Duplicate)

The following tests already exist across the modules and cover Sprint 1/2 basics. We will NOT duplicate these:

- `test_pokemonv2_creates` (mod.rs)
- `test_title_to_overworld` (mod.rs)
- `test_player_spawns_in_bedroom` (mod.rs)
- `test_all_sprint1_maps_load` (mod.rs, maps.rs)
- `test_warp_bidirectional_consistency` (mod.rs)
- `test_walking_changes_position` (mod.rs)
- `test_collision_blocks_wall` (mod.rs)
- `test_warp_bedroom_to_1f` (mod.rs)
- `test_coord_event_teacher_blocks` (mod.rs)
- `test_script_engine_basic` (mod.rs)
- `test_npc_collision` (mod.rs)
- `test_elm_lab_has_correct_npcs` (mod.rs)
- `test_starter_pokemon_stats` (mod.rs)
- `test_event_flags` (mod.rs, events.rs)
- `test_starter_scene_sets_aide_gives_potion` (mod.rs)
- `test_sprint2_maps_load` (maps.rs)
- `test_route29_has_encounters` (maps.rs)
- `test_is_walkable_with_direction_ledge` (maps.rs)
- `test_all_scripts_compile` (events.rs)
- `test_load_wild_mon_then_start_battle` (events.rs)
- `test_guide_gent_gives_map_card` (events.rs)
- `test_ledge_only_walkable_facing_down` (overworld.rs)
- `test_wild_encounter_on_grass` (overworld.rs)
- `test_tutorial_auto_catches` (battle.rs)
- `test_battle_wild_runs_auto` (battle.rs)

Sprint 3 adds ~42 NEW tests on top of these existing ~25.

---

## Implementation Notes

1. **No new files needed.** All tests go in existing `#[cfg(test)]` modules.

2. **Test helper `with_state()`** already exists on `PokemonV2Sim`. We may want one additional helper for script execution tests that doesn't require Engine:
   ```rust
   #[cfg(test)]
   fn run_script_to_completion(script_id: u16, flags: &mut EventFlags, bag: &mut Vec<(u8, u8)>, party: &mut Vec<Pokemon>) -> ScriptResult
   ```

3. **Script content inspection** (checking that a script contains specific steps) is a valid test strategy. We don't need to fully execute every script -- verifying the step sequence catches structural bugs.

4. **Full playthrough tests** (Group 10) use `Engine::new(160, 144)` + input injection. Each test covers one segment (bedroom->1F, 1F->NewBark, etc.) rather than one massive test.

5. **Encounter table verification** should compare against pokecrystal data for accuracy. The tables are hardcoded in maps.rs so we just verify the Vec contents.
