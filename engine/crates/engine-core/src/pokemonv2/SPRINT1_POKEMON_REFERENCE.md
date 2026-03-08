# Sprint 1 Pokemon Accuracy Reference: New Bark Town Cluster

> Compiled by Gimli (Pokemon accuracy guardian)
> Primary source: pokecrystal-master .asm files
> Secondary source: data/maps/ compiled docs
> Game: Pokemon Crystal (Gen II)
> Sprint scope: Player House 2F, Player House 1F, New Bark Town exterior, Elm's Lab

---

## 1. Map Dimensions

Derived from .blk file sizes. Each block = 2x2 tiles. Each tile = 8x8 pixels.
Coordinate system: (col, row) — 0-indexed, as used in warp_event / object_event / coord_event.

| Map | .blk bytes | Likely dimensions (blocks) | Tiles (16px each) |
|-----|------------|----------------------------|--------------------|
| NewBarkTown.blk | 90 bytes | 9 wide x 10 tall | 18×20 tiles |
| ElmsLab.blk | 30 bytes | 10 wide x 3 tall (interior rooms are wider than tall) | ~10×10 tiles |
| PlayersHouse1F.blk | 20 bytes | 10 wide x 2 tall | ~10×7 tiles |
| PlayersHouse2F.blk | 12 bytes | ~6-8 wide x ~2 tall | ~8×6 tiles |

Note: Interior .blk dimensions must be interpreted with tileset constraints. Width is typically
10 blocks for standard interiors. PlayersHouse1F is 10w × 2h = 20 blocks, PlayersHouse2F is
likely 6w × 2h = 12 blocks.

**Tileset for NewBarkTown exterior: $05** (from `map_attributes NewBarkTown, NEW_BARK_TOWN, $05`)
Interior maps use tileset $00 (standard interior tileset).

---

## 2. Warp Connections (from pokecrystal .asm files — AUTHORITATIVE)

### NewBarkTown.asm warp_events
```
warp_event  6,  3, ELMS_LAB, 1           ; Elm's Lab entrance
warp_event 13,  5, PLAYERS_HOUSE_1F, 1   ; Player's house entrance
warp_event  3, 11, PLAYERS_NEIGHBORS_HOUSE, 1  ; Rival's house
warp_event 11, 13, ELMS_HOUSE, 1         ; Elm's house
```

### PlayersHouse1F.asm warp_events
```
warp_event  6,  7, NEW_BARK_TOWN, 2      ; exit (south door) — warp #2
warp_event  7,  7, NEW_BARK_TOWN, 2      ; exit (south door) — warp #2
warp_event  9,  0, PLAYERS_HOUSE_2F, 1  ; stairs to 2F
```

### PlayersHouse2F.asm warp_events
```
warp_event  7,  0, PLAYERS_HOUSE_1F, 3  ; stairs down to 1F
```

### ElmsLab.asm warp_events
```
warp_event  4, 11, NEW_BARK_TOWN, 1     ; exit — warp #1 in NBT
warp_event  5, 11, NEW_BARK_TOWN, 1     ; exit — warp #1 in NBT
```

---

## 3. NPCs and Object Events

### NewBarkTown — 3 objects
| Const | Sprite | Position (x,y) | Movement | Event Flag |
|-------|--------|----------------|----------|------------|
| NEWBARKTOWN_TEACHER | SPRITE_TEACHER | (6, 8) | SPRITEMOVEDATA_SPINRANDOM_SLOW | none (always visible) |
| NEWBARKTOWN_FISHER | SPRITE_FISHER | (12, 9) | SPRITEMOVEDATA_WALK_UP_DOWN | PAL_NPC_GREEN |
| NEWBARKTOWN_RIVAL | SPRITE_RIVAL | (3, 2) | SPRITEMOVEDATA_STANDING_RIGHT | EVENT_RIVAL_NEW_BARK_TOWN |

- **Rival** is conditionally visible via EVENT_RIVAL_NEW_BARK_TOWN (visible before player gets starter and returns)
- **Teacher** moves randomly (slow spin), blocks player from leaving toward Route 29 without a Pokemon

### Elm's Lab — 6 objects
| Const | Sprite | Position (x,y) | Movement | Event Flag |
|-------|--------|----------------|----------|------------|
| ELMSLAB_ELM | SPRITE_ELM | (5, 2) | SPRITEMOVEDATA_STANDING_DOWN | none |
| ELMSLAB_ELMS_AIDE | SPRITE_SCIENTIST | (2, 9) | SPRITEMOVEDATA_SPINRANDOM_SLOW | EVENT_ELMS_AIDE_IN_LAB |
| ELMSLAB_POKE_BALL1 | SPRITE_POKE_BALL | (6, 3) | SPRITEMOVEDATA_STILL | EVENT_CYNDAQUIL_POKEBALL_IN_ELMS_LAB |
| ELMSLAB_POKE_BALL2 | SPRITE_POKE_BALL | (7, 3) | SPRITEMOVEDATA_STILL | EVENT_TOTODILE_POKEBALL_IN_ELMS_LAB |
| ELMSLAB_POKE_BALL3 | SPRITE_POKE_BALL | (8, 3) | SPRITEMOVEDATA_STILL | EVENT_CHIKORITA_POKEBALL_IN_ELMS_LAB |
| ELMSLAB_OFFICER | SPRITE_OFFICER | (5, 3) | SPRITEMOVEDATA_STANDING_UP | EVENT_COP_IN_ELMS_LAB |

- Pokeball order on desk: Cyndaquil (left/6,3), Totodile (center/7,3), Chikorita (right/8,3)
- Elm starts at (3,4) during intro walk-in scene (moved by ElmsLabMoveElmCallback), then returns to (5,2)
- Officer appears ONLY after player returns with Mystery Egg (EVENT_COP_IN_ELMS_LAB)
- Aide is hidden until after theft (EVENT_ELMS_AIDE_IN_LAB controls visibility)

### Player's House 1F — 5 objects (Mom has 4 entries for time-of-day variants)
| Const | Sprite | Position (x,y) | Movement | Condition |
|-------|--------|----------------|----------|-----------|
| PLAYERSHOUSE1F_MOM1 | SPRITE_MOM | (7, 4) | STANDING_LEFT | EVENT_PLAYERS_HOUSE_MOM_1 (first visit) |
| PLAYERSHOUSE1F_MOM2 | SPRITE_MOM | (2, 2) | STANDING_UP | EVENT_PLAYERS_HOUSE_MOM_2, MORN only |
| PLAYERSHOUSE1F_MOM3 | SPRITE_MOM | (7, 4) | STANDING_LEFT | EVENT_PLAYERS_HOUSE_MOM_2, DAY only |
| PLAYERSHOUSE1F_MOM4 | SPRITE_MOM | (0, 2) | STANDING_UP | EVENT_PLAYERS_HOUSE_MOM_2, NITE only |
| PLAYERSHOUSE1F_POKEFAN_F | SPRITE_POKEFAN_F | (4, 4) | STANDING_RIGHT | EVENT_PLAYERS_HOUSE_1F_NEIGHBOR, PAL_NPC_RED |

- Mom position changes by time of day AFTER first visit (MOM_1 clears, MOM_2 activates)
- Neighbor (Pokefan F) is the neighbor's mother who visits; she mentions her daughter wants to work for Elm

### Player's House 2F — 4 decoration objects (all tied to decoration system)
| Const | Sprite | Position (x,y) | Notes |
|-------|--------|----------------|-------|
| PLAYERSHOUSE2F_CONSOLE | SPRITE_CONSOLE | (4, 2) | Describable decoration |
| PLAYERSHOUSE2F_DOLL_1 | SPRITE_DOLL_1 | (4, 4) | Left doll (DECODESC_LEFT_DOLL) |
| PLAYERSHOUSE2F_DOLL_2 | SPRITE_DOLL_2 | (5, 4) | Right doll (DECODESC_RIGHT_DOLL) |
| PLAYERSHOUSE2F_BIG_DOLL | SPRITE_BIG_DOLL | (0, 1) | SPRITEMOVEDATA_BIGDOLL |

---

## 4. Coord Events (Trigger Zones)

### NewBarkTown coord_events
```
coord_event  1,  8, SCENE_NEWBARKTOWN_TEACHER_STOPS_YOU, NewBarkTown_TeacherStopsYouScene1
coord_event  1,  9, SCENE_NEWBARKTOWN_TEACHER_STOPS_YOU, NewBarkTown_TeacherStopsYouScene2
```
- **Triggers** when player steps at x=1, y=8 or y=9 (western edge, toward Route 29)
- **Condition**: Only fires if scene = SCENE_NEWBARKTOWN_TEACHER_STOPS_YOU (i.e., before getting Pokemon from Elm)
- After getting starter, scene changes to SCENE_NEWBARKTOWN_NOOP (set in ElmDirectionsScript via `setmapscene NEW_BARK_TOWN, SCENE_NEWBARKTOWN_NOOP`)

### Player's House 1F coord_events
```
coord_event  8,  4, SCENE_PLAYERSHOUSE1F_MEET_MOM, MeetMomLeftScript
coord_event  9,  4, SCENE_PLAYERSHOUSE1F_MEET_MOM, MeetMomRightScript
```
- **Triggers** Pokegear handoff scene when player steps at (8,4) or (9,4)
- Scene = SCENE_PLAYERSHOUSE1F_MEET_MOM on first entry

### Elm's Lab coord_events
```
coord_event  4,  6, SCENE_ELMSLAB_CANT_LEAVE, LabTryToLeaveScript    ; blocks exit before choosing starter
coord_event  5,  6, SCENE_ELMSLAB_CANT_LEAVE, LabTryToLeaveScript
coord_event  4,  5, SCENE_ELMSLAB_MEET_OFFICER, MeetCopScript        ; officer encounter after egg return
coord_event  5,  5, SCENE_ELMSLAB_MEET_OFFICER, MeetCopScript2
coord_event  4,  8, SCENE_ELMSLAB_AIDE_GIVES_POTION, AideScript_WalkPotion1  ; aide gives Potion
coord_event  5,  8, SCENE_ELMSLAB_AIDE_GIVES_POTION, AideScript_WalkPotion2
coord_event  4,  8, SCENE_ELMSLAB_AIDE_GIVES_POKE_BALLS, AideScript_WalkBalls1  ; aide gives 5 Pokeballs
coord_event  5,  8, SCENE_ELMSLAB_AIDE_GIVES_POKE_BALLS, AideScript_WalkBalls2
```

---

## 5. Background Events (Interactive Objects/Signs)

### NewBarkTown bg_events (signs)
```
bg_event  8,  8, BGEVENT_READ, NewBarkTownSign          ; "NEW BARK TOWN - The Town Where..."
bg_event 11,  5, BGEVENT_READ, NewBarkTownPlayersHouseSign  ; "<PLAYER>'s House"
bg_event  3,  3, BGEVENT_READ, NewBarkTownElmsLabSign   ; "ELM #MON LAB"
bg_event  9, 13, BGEVENT_READ, NewBarkTownElmsHouseSign ; "ELM'S HOUSE"
```

### Elm's Lab bg_events (bookshelves, PC, healing machine, window)
```
bg_event  2,  1, BGEVENT_READ, ElmsLabHealingMachine    ; healing machine
bg_event  6-9, 1, BGEVENT_READ, ElmsLabBookshelf (x4)   ; bookshelves
bg_event  0-3, 7, BGEVENT_READ, ElmsLabTravelTip1-4     ; travel tip books
bg_event  6-9, 7, BGEVENT_READ, ElmsLabBookshelf (x4)   ; more bookshelves
bg_event  9,  3, BGEVENT_READ, ElmsLabTrashcan          ; "The wrapper from the snack Elm ate"
bg_event  5,  0, BGEVENT_READ, ElmsLabWindow            ; window (break-in text post-theft)
bg_event  3,  5, BGEVENT_DOWN, ElmsLabPC                ; "OBSERVATIONS ON #MON EVOLUTION"
```

### Player's House 1F bg_events
```
bg_event  0,  1, BGEVENT_READ, PlayersHouse1FStoveScript  ; "CINNABAR VOLCANO BURGER!"
bg_event  1,  1, BGEVENT_READ, PlayersHouse1FSinkScript   ; spotless sink
bg_event  2,  1, BGEVENT_READ, PlayersHouse1FFridgeScript ; FRESH WATER and LEMONADE
bg_event  4,  1, BGEVENT_READ, PlayersHouse1FTVScript     ; movie about two boys on a train
```

### Player's House 2F bg_events
```
bg_event  2,  1, BGEVENT_UP, PlayersHousePCScript       ; PC (up interaction)
bg_event  3,  1, BGEVENT_READ, PlayersHouseRadioScript  ; Radio (plays Pokemon Talk theme initially)
bg_event  5,  1, BGEVENT_READ, PlayersHouseBookshelfScript ; picture bookshelf
bg_event  6,  0, BGEVENT_IFSET, PlayersHousePosterScript  ; poster (conditional)
```

---

## 6. Scene / Event Flag Sequence for Sprint 1

### Scene progression in Elm's Lab (ELMS_LAB scene IDs, in order):
1. `SCENE_ELMSLAB_MEET_ELM` — Initial entry, walk-up cutscene, Elm emails Mr. Pokemon, choose starter
2. `SCENE_ELMSLAB_CANT_LEAVE` — After choosing starter (can't exit until scene advances); Elm tells player to use healing machine
3. `SCENE_ELMSLAB_NOOP` — After Aide gives Potion (scene goes to NOOP, lab is free to explore)
4. `SCENE_ELMSLAB_MEET_OFFICER` — After returning with Mystery Egg, before naming Rival
5. `SCENE_ELMSLAB_UNUSED` — Unused
6. `SCENE_ELMSLAB_AIDE_GIVES_POTION` — Coord-triggered; Aide walks over and gives Potion
7. `SCENE_ELMSLAB_AIDE_GIVES_POKE_BALLS` — After returning egg; Aide gives 5 Pokeballs

### Scene progression in Player's House 1F:
1. `SCENE_PLAYERSHOUSE1F_MEET_MOM` — First visit: Mom gives Pokegear, sets clock
2. `SCENE_PLAYERSHOUSE1F_NOOP` — Normal state after Pokegear received

### Scene progression in New Bark Town exterior:
1. `SCENE_NEWBARKTOWN_TEACHER_STOPS_YOU` — Before getting Pokemon; Teacher blocks western exit
2. `SCENE_NEWBARKTOWN_NOOP` — After getting Pokemon from Elm

### Key event flags for Sprint 1 area:
| Flag | Set When | Effect |
|------|----------|--------|
| ENGINE_POKEGEAR | Mom gives Pokegear | Pokegear usable |
| ENGINE_PHONE_CARD | Mom gives Pokegear | Phone accessible |
| PHONE_MOM | Mom gives Pokegear | Mom in phone contacts |
| EVENT_PLAYERS_HOUSE_MOM_1 | Mom gives Pokegear | Clears first-entry trigger |
| EVENT_PLAYERS_HOUSE_MOM_2 | Cleared alongside MOM_1 | Time-of-day Mom positions activate |
| EVENT_GOT_A_POKEMON_FROM_ELM | Player chooses any starter | Activates Rival in Cherrygrove, enables leaving lab |
| EVENT_GOT_CYNDAQUIL_FROM_ELM | Chose Cyndaquil | Pokeball 1 disappears |
| EVENT_GOT_TOTODILE_FROM_ELM | Chose Totodile | Pokeball 2 disappears |
| EVENT_GOT_CHIKORITA_FROM_ELM | Chose Chikorita | Pokeball 3 disappears |
| EVENT_RIVAL_NEW_BARK_TOWN | Rival visible outside lab | Rival NPC appears at (3,2) |
| ENGINE_FLYPOINT_NEW_BARK | Enter New Bark Town (callback) | Fly destination registered |
| EVENT_FIRST_TIME_BANKING_WITH_MOM | After returning with egg | Bank of Mom system unlocks |

---

## 7. Starter Pokemon Data (Lv5 from Elm)

All starters: given via `givepoke <SPECIES>, 5, BERRY` — they hold a BERRY at Lv5.

### Chikorita (Dex #152)
- **Type**: Grass / Grass
- **Base Stats**: HP 45 / Atk 49 / Def 65 / Spd 45 / SpAtk 49 / SpDef 65
- **Catch rate**: 45 | **Base Exp**: 64
- **Gender ratio**: GENDER_F12_5 (12.5% female, 87.5% male)
- **Growth rate**: MEDIUM_SLOW
- **Egg groups**: Monster + Plant
- **Evolves**: Lv16 → Bayleef, Lv32 → Meganium
- **Moves at Lv5**: Tackle (Lv1), Growl (Lv1)
- **Next moves**: Razor Leaf (Lv8), Reflect (Lv12), PoisonPowder (Lv15)
- **Held item given**: BERRY

### Cyndaquil (Dex #155)
- **Type**: Fire / Fire
- **Base Stats**: HP 39 / Atk 52 / Def 43 / Spd 65 / SpAtk 60 / SpDef 50
- **Catch rate**: 45 | **Base Exp**: 65
- **Gender ratio**: GENDER_F12_5
- **Growth rate**: MEDIUM_SLOW
- **Egg groups**: Ground + Ground
- **Evolves**: Lv14 → Quilava, Lv36 → Typhlosion
- **Moves at Lv5**: Tackle (Lv1), Leer (Lv1)
- **Next moves**: Smokescreen (Lv6), Ember (Lv12)
- **Held item given**: BERRY

### Totodile (Dex #158)
- **Type**: Water / Water
- **Base Stats**: HP 50 / Atk 65 / Def 64 / Spd 43 / SpAtk 44 / SpDef 48
- **Catch rate**: 45 | **Base Exp**: 66
- **Gender ratio**: GENDER_F12_5
- **Growth rate**: MEDIUM_SLOW
- **Egg groups**: Monster + Water1
- **Evolves**: Lv18 → Croconaw, Lv30 → Feraligatr
- **Moves at Lv5**: Scratch (Lv1), Leer (Lv1)
- **Next moves**: Rage (Lv7), Water Gun (Lv13)
- **Held item given**: BERRY

---

## 8. Items Given During Sprint 1 Scope

| Item | Where/How | Event Flag Set |
|------|-----------|----------------|
| POKEGEAR | Mom gives on 1F of Player's House (first visit) | ENGINE_POKEGEAR, ENGINE_PHONE_CARD |
| BERRY | Held by chosen starter (given via givepoke) | EVENT_GOT_*_FROM_ELM |
| POTION | Elm's Aide gives via coord_event (4,8)/(5,8) in lab | scene advances to SCENE_ELMSLAB_NOOP |
| Elm's phone # | Added automatically after choosing starter (addcellnum PHONE_ELM) | none (phone list) |
| Mom's phone # | Added automatically when Pokegear given (addcellnum PHONE_MOM) | none (phone list) |

Items given OUTSIDE Sprint 1 scope (for architect reference — do not implement in Sprint 1):
- 5 Poke Balls (Aide, after returning Mystery Egg — SCENE_ELMSLAB_AIDE_GIVES_POKE_BALLS)
- Pokedex (from Oak at Mr. Pokemon's House)
- Mystery Egg (from Mr. Pokemon)

---

## 9. Music

- **New Bark Town exterior**: MUSIC_NEW_BARK_TOWN
- **Player's House 1F**: MUSIC_MOM (played during intro cutscene)
- **Elm's Lab**: MUSIC_ELMSLAB (referenced via `musicfadeout MUSIC_NEW_BARK_TOWN` in 2F radio)
- **Teacher cutscene**: `playmusic MUSIC_MOM` (overrides map music during teacher-blocks-you scene)

---

## 10. Elm's Lab Scene Walk-In Cutscene (Movement Sequence)

This is the first cutscene the player experiences. Exact movements from ElmsLab.asm:

1. `ElmsLab_WalkUpToElmMovement`: Player auto-walks UP 7 steps, then `turn_head LEFT`
2. `showemote EMOTE_SHOCK, ELMSLAB_ELM, 15` — Elm shows shock emote for 15 frames
3. `turnobject ELMSLAB_ELM, RIGHT` — Elm turns right to face player
4. Elm speaks intro text (multi-page, can't refuse — yesorno loop)
5. `playsound SFX_GLASS_TING` + `pause 30` — email notification sound
6. `showemote EMOTE_SHOCK, ELMSLAB_ELM, 10`
7. `turnobject ELMSLAB_ELM, DOWN` — Elm turns to read email
8. Multiple dialog pages about Mr. Pokemon
9. `ElmsLab_ElmToDefaultPositionMovement1`: Elm steps UP 1
10. `ElmsLab_ElmToDefaultPositionMovement2`: Elm steps RIGHT 2, UP 1, `turn_head DOWN`
11. Player `turnobject PLAYER, UP` then `RIGHT`
12. Elm speaks "Choose a Pokemon!" text
13. `setscene SCENE_ELMSLAB_CANT_LEAVE`

After choosing starter, movements vary by which Pokemon:
- Cyndaquil: player moves LEFT 1, UP 1, `turn_head UP`
- Totodile: player moves LEFT 2, UP 1, `turn_head UP`
- Chikorita: player moves LEFT 3, UP 1, `turn_head UP`

Then ElmDirectionsScript runs (adds phone number, Elm points to healing machine, ends scene).

---

## 11. Teacher-Blocks-You Scenes (NewBarkTown)

Two variants depending on which coord tile player hits:

**Scene 1 (y=8)**: Teacher runs LEFT 4 steps, then escorts player back RIGHT 4 steps + `turn_head LEFT`
**Scene 2 (y=9)**: Teacher runs LEFT 5 steps + `turn_head DOWN`, escorts player back RIGHT 5 steps + `turn_head LEFT`

Key text: "It's dangerous to go out without a #MON! Wild #MON jump out of the grass on the way to the next town."

---

## 12. Rival Outside Lab

The Rival (Silver) appears at position (3, 2) in New Bark Town, standing RIGHT, controlled by EVENT_RIVAL_NEW_BARK_TOWN.

Interaction script:
1. Rival text 1: "<……> So this is the famous ELM #MON LAB…"
2. Player shoves: `turn_head LEFT` on Rival, `follow PLAYER, RIVAL`, player jumps DOWN
3. Rival returns RIGHT 1 step (back to window position)
4. Text 2: "…What are you staring at?"

The rival never actually battles the player here — this is a pre-starter sighting only.

---

## 13. Map Connections (Exterior)

NewBarkTown connections (from attributes.asm):
- **West**: Route 29 (connection offset 0)
- **East**: Route 27 (connection offset 0) — late game, requires Surf

Fly point: ENGINE_FLYPOINT_NEW_BARK (set via MAPCALLBACK_NEWMAP callback on every entry)

---

## 14. Notes for Architects / Implementers

1. **Elm's starting position during intro**: Elm is moved to (3, 4) by ElmsLabMoveElmCallback BEFORE the scene script fires. After the walk-in, he returns to (5, 2) via ElmToDefaultPositionMovement.

2. **Scene IDs are ordinal**: SCENE_ELMSLAB_MEET_ELM = 0, SCENE_ELMSLAB_CANT_LEAVE = 1, SCENE_ELMSLAB_NOOP = 2, SCENE_ELMSLAB_MEET_OFFICER = 3, SCENE_ELMSLAB_UNUSED = 4, SCENE_ELMSLAB_AIDE_GIVES_POTION = 5, SCENE_ELMSLAB_AIDE_GIVES_POKE_BALLS = 6.

3. **`checkscene` in pokecrystal**: Returns true if scene > 0 (i.e., NOT the 0th scene). Used in `ElmsLabMoveElmCallback` to skip moving Elm if we're past the intro.

4. **Healing machine in lab**: Functional before player has any Pokemon (shows "I wonder what this does?"), becomes usable after EVENT_GOT_A_POKEMON_FROM_ELM.

5. **Player's House 2F**: The room uses a decoration system (ToggleDecorationsVisibility, ToggleMaptileDecorations). Objects like CONSOLE, DOLL_1, DOLL_2, BIG_DOLL are decorations that can be removed/placed. This is the in-game "room decoration" feature.

6. **Radio in 2F**: On first listen (before getting starter), plays a special 4-phase Pokemon Talk intro with music fades. After getting starter, uses the standard Radio1Script.

7. **Pokeballs on desk**: Left-to-right order is Cyndaquil (6,3), Totodile (7,3), Chikorita (8,3). Pokecrystal places them this way consistently — maintain this exact order.

8. **Map music transition**: When Teacher stops you in New Bark Town, music switches to MUSIC_MOM temporarily, then `RestartMapMusic` restores map music. This is an important detail for the audio system.
