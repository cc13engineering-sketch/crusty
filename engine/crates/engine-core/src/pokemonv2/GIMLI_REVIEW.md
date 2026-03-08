# Gimli's Pokemon Crystal Accuracy Review — Sprint 1

> Reviewer: Gimli (Pokemon Super Fan, accuracy guardian)
> Sources checked: pokecrystal-master .asm files (authoritative), SPRINT1_IMPLEMENTATION_PLAN.md, SPRINT1_POKEMON_REFERENCE.md
> Verdict: **ACCEPT WITH NOTES**

---

## Executive Summary

The Sprint 1 plan is substantially accurate. The reference document (SPRINT1_POKEMON_REFERENCE.md) is excellent and faithfully transcribes pokecrystal coordinates, NPC data, event flags, and scene sequences. The implementation plan correctly uses this reference data. I found **one meaningful accuracy error**, **three minor inaccuracies**, and several observations for the implementers.

---

## Verdict: ACCEPT WITH NOTES

The plan can proceed as written. The one meaningful error (scene after starter choice) must be fixed to avoid a broken game flow, but it is a single-line correction. All other data is solid.

---

## Findings

### CRITICAL FIX REQUIRED

#### 1. Wrong scene set after starter choice in ElmDirectionsScript

**Plan says** (starter selection script, bottom of ElmsLab section):
```
SetScene { map: MapId::ElmsLab, scene_id: 2 }   // SCENE_ELMSLAB_NOOP
```

**Actual pokecrystal** (`ElmsLab.asm`, line 276):
```
setscene SCENE_ELMSLAB_AIDE_GIVES_POTION      ; scene 5
setmapscene NEW_BARK_TOWN, SCENE_NEWBARKTOWN_NOOP
```

After the player picks a starter, the lab scene advances to `SCENE_ELMSLAB_AIDE_GIVES_POTION` (5), NOT `SCENE_ELMSLAB_NOOP` (2). This matters: the coord events at (4,8) and (5,8) only fire when scene == 5. If you skip directly to scene 2, the Aide-gives-Potion sequence can never trigger. The NBT scene does correctly advance to NOOP.

**Fix**: In the starter selection script, change:
```rust
SetScene { map: MapId::ElmsLab, scene_id: 2 }   // WRONG
```
to:
```rust
SetScene { map: MapId::ElmsLab, scene_id: 5 }   // SCENE_ELMSLAB_AIDE_GIVES_POTION
```

---

### MINOR INACCURACIES

#### 2. Teacher scene — dialog accuracy

The plan reconstructs teacher dialog as:
> "TEACHER: Hey, wait! It's dangerous to go out without a POKeMON!"
> "TEACHER: Wild POKeMON jump out of the grass on the way to the next town."

Actual pokecrystal dialog uses **three separate text nodes** (not two), and the teacher speaks them in this order:
1. `Text_WaitPlayer`: "Wait, <PLAY_G>!" (first, before teacher moves)
2. `Text_WhatDoYouThinkYoureDoing`: "What do you think you're doing?" (after teacher runs to player)
3. `Text_ItsDangerousToGoAlone`: "It's dangerous to go out without a #MON! Wild #MON jump out of the grass on the way to the next town." (after teacher escorts player back)

The plan collapses this into two dialog nodes and omits the "Wait!" interjection. For Sprint 1 these are just stub ShowText calls so this is not blocking, but implementers should know the actual flow.

Also: the teacher says "Wait, <PLAY_G>!" not "Hey, wait!" and does NOT prefix with "TEACHER:" in the game text (the text is unattributed).

#### 3. Fisher NPC palette

The reference says Fisher has `PAL_NPC_GREEN`. The actual pokecrystal object_event for Fisher (NewBarkTown.asm line 303):
```
object_event 12,  9, SPRITE_FISHER, SPRITEMOVEDATA_WALK_UP_DOWN, 0, 1, -1, -1, PAL_NPC_GREEN, ...
```
This **matches** — confirmed correct.

The Teacher NPC has palette `0` (no special palette), which also matches the plan.

#### 4. PlayersHouse1F — Mom event flag polarity

The plan correctly notes Mom1 is `event_flag_show=false` (visible when flag NOT set — i.e., first visit). However, note that the actual pokecrystal MeetMom script does something the plan omits:

```asm
setevent EVENT_PLAYERS_HOUSE_MOM_1
clearevent EVENT_PLAYERS_HOUSE_MOM_2  ; NOTE: clears MOM_2, not sets it
```

The plan says:
```
SetEvent(EVENT_PLAYERS_HOUSE_MOM_1)   // clear Mom1, activate Mom2 variants
SetEvent(EVENT_PLAYERS_HOUSE_MOM_2)   // activate time-of-day Mom positions
```

Actual code **clears** `EVENT_PLAYERS_HOUSE_MOM_2`, it does not set it. The pokecrystal object_events for Mom2/3/4 use `EVENT_PLAYERS_HOUSE_MOM_2` as show-when-set, so if it's cleared they're hidden — which is wrong. Looking at the source more carefully:

```asm
object_event  2,  2, SPRITE_MOM, SPRITEMOVEDATA_STANDING_UP, 0, 0, -1, MORN, ...  EVENT_PLAYERS_HOUSE_MOM_2
object_event  7,  4, SPRITE_MOM, SPRITEMOVEDATA_STANDING_LEFT, 0, 0, -1, DAY, ...  EVENT_PLAYERS_HOUSE_MOM_2
object_event  0,  2, SPRITE_MOM, SPRITEMOVEDATA_STANDING_UP, 0, 0, -1, NITE, ...  EVENT_PLAYERS_HOUSE_MOM_2
```

The object_events for time-of-day Mom use `EVENT_PLAYERS_HOUSE_MOM_2`. Looking at the actual polarity: in pokecrystal, `object_event` with an event flag at the end uses `OBJECTTYPE_SCRIPT` with show-when-flag-is-NOT-set (bit 0 of flags param). The default is hide-when-set. So clearing `EVENT_PLAYERS_HOUSE_MOM_2` makes Mom2/3/4 visible (they show when flag is not set).

The reference document says "EVENT_PLAYERS_HOUSE_MOM_2: Cleared alongside MOM_1 — Time-of-day Mom positions activate" which accurately describes the intended behavior. But the plan's script stub incorrectly calls `SetEvent(EVENT_PLAYERS_HOUSE_MOM_2)` — it should call `ClearEvent(EVENT_PLAYERS_HOUSE_MOM_2)` (same as `clearevent`).

**Fix in MeetMom script**: Replace `SetEvent(EVENT_PLAYERS_HOUSE_MOM_2)` with `ClearEvent(EVENT_PLAYERS_HOUSE_MOM_2)`.

---

## CONFIRMED ACCURATE — Cross-Referenced Against Pokecrystal

All of the following were verified against the canonical .asm files:

### NewBarkTown
- Warps: (6,3)->ELMS_LAB warp 1, (13,5)->PLAYERS_HOUSE_1F warp 1, (3,11)->RIVALS_HOUSE warp 1, (11,13)->ELMS_HOUSE warp 1 — all correct
- NPCs: Teacher (6,8) SpinRandom, Fisher (12,9) WalkUpDown PAL_GREEN, Rival (3,2) StandingRight EVENT_RIVAL_NEW_BARK_TOWN — all correct
- BG events: (8,8), (11,5), (3,3), (9,13) — all correct
- Coord events: (1,8) and (1,9) both on SCENE_NEWBARKTOWN_TEACHER_STOPS_YOU — correct
- Scene numbering: TEACHER_STOPS_YOU=0, NOOP=1 — correct

### Elm's Lab
- Warps: (4,11) and (5,11) both -> NEW_BARK_TOWN warp 1 — correct
- NPCs: all 6 NPCs at correct positions with correct event flags — correct
- Pokeball order: Cyndaquil (6,3), Totodile (7,3), Chikorita (8,3) left-to-right — confirmed
- Elm starts at (3,4) during MEET_ELM scene via callback — confirmed
- Officer at (5,3) not (5,2) — plan correctly says (5,3)
- Coord events: all 8 events at correct coordinates and scene IDs — correct
- BG events: all 15 events confirmed against actual .asm
- Scene constants: MEET_ELM=0, CANT_LEAVE=1, NOOP=2, MEET_OFFICER=3, UNUSED=4, AIDE_GIVES_POTION=5, AIDE_GIVES_POKE_BALLS=6 — correct

### PlayersHouse1F
- Warps: (6,7), (7,7) -> NEW_BARK_TOWN warp 2; (9,0) -> PLAYERS_HOUSE_2F warp 1 — correct
- NPCs: Mom1 (7,4), Mom2 (2,2) MORN, Mom3 (7,4) DAY, Mom4 (0,2) NITE, Pokefan_F (4,4) — all correct
- BG events: Stove (0,1), Sink (1,1), Fridge (2,1), TV (4,1) — correct
- Coord events: (8,4) and (9,4) on SCENE_PLAYERSHOUSE1F_MEET_MOM — correct

### PlayersHouse2F
- Warp: (7,0) -> PLAYERS_HOUSE_1F warp 3 — correct
- NPCs: Console (4,2), Doll_1 (4,4), Doll_2 (5,4), BigDoll (0,1) — all correct
- BG events: PC (2,1) BGEVENT_UP, Radio (3,1) READ, Bookshelf (5,1) READ, Poster (6,0) BGEVENT_IFSET — all correct

### Starter Pokemon Data
All three starters verified against `data/pokemon/base_stats/`:

| Pokemon | HP | Atk | Def | Spd | SpAtk | SpDef | Type | Catch | Exp | Growth |
|---------|----|----|-----|-----|-------|-------|------|-------|-----|--------|
| Chikorita (#152) | 45 | 49 | 65 | 45 | 49 | 65 | Grass/Grass | 45 | 64 | Medium Slow |
| Cyndaquil (#155) | 39 | 52 | 43 | 65 | 60 | 50 | Fire/Fire | 45 | 65 | Medium Slow |
| Totodile (#158) | 50 | 65 | 64 | 43 | 44 | 48 | Water/Water | 45 | 66 | Medium Slow |

All base stats in the reference and plan match pokecrystal exactly.

Starters given at Lv5 with BERRY held item — confirmed (`givepoke CYNDAQUIL, 5, BERRY` etc).

### Elm Walk-In Cutscene
- Player walks UP 7 steps, turn_head LEFT — confirmed
- Elm shows EMOTE_SHOCK for 15 frames — confirmed
- Elm turns RIGHT — confirmed
- SFX_GLASS_TING plays, pause 30 frames — confirmed
- Second EMOTE_SHOCK for 10 frames — confirmed
- Elm returns via: UP 1 step, then RIGHT 2 + UP 1 + turn_head DOWN — confirmed
- Scene set to SCENE_ELMSLAB_CANT_LEAVE after intro — confirmed

### Move IDs (Sprint 1)
- TACKLE: 33 — confirmed standard
- GROWL: 45 — confirmed standard
- LEER: 43 — confirmed standard
- SCRATCH: 10 — confirmed standard

---

## Summary of Required Changes

| Priority | Location | Issue | Fix |
|----------|----------|-------|-----|
| CRITICAL | ElmsLab starter selection script | Sets scene to NOOP (2) instead of AIDE_GIVES_POTION (5) | Change scene_id: 2 to scene_id: 5 |
| MINOR | MeetMom script | SetEvent(MOM_2) should be ClearEvent(MOM_2) | Replace SetEvent with ClearEvent |
| NOTE | Teacher scene dialog | Plan shows 2 dialog nodes; actual game uses 3 separate text nodes with different structure | Update dialog reconstruction when polishing |

---

## Final Verdict

**ACCEPT WITH NOTES**

Fix the SCENE_ELMSLAB_CANT_LEAVE → SCENE_ELMSLAB_AIDE_GIVES_POTION bug and the MOM_2 set/clear inversion before code review. Everything else is solid. The reference document quality is excellent — it clearly came from reading the actual .asm files, not from memory or secondary sources.
