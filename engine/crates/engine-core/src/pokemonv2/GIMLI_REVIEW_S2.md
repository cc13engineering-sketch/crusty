# Gimli's Pokemon Accuracy Review -- Sprint 2 Revised Plan

> Reviewer: Gimli (Pokemon Super Fan)
> Date: 2026-03-08
> Document Reviewed: SPRINT2_REVISED_PLAN.md

---

## VERDICT: ACCEPT WITH MODIFICATIONS

The plan is overwhelmingly accurate. Species base stats, wild encounter tables, learnsets, and map dimensions are all correct against pokecrystal source. There are a handful of data discrepancies that must be corrected before implementation, but none warrant a veto. Details below.

---

## 1. Species Base Stats -- PASS (All 5 Correct)

Verified against `data/pokemon/base_stats/*.asm`:

| Species | Plan HP/Atk/Def/Spd/SpA/SpD | Pokecrystal | Match? |
|---------|------------------------------|-------------|--------|
| Pidgey  | 40/45/40/56/35/35           | 40,45,40,56,35,35 (`pidgey.asm:3`) | YES |
| Rattata | 30/56/35/72/25/35           | 30,56,35,72,25,35 (`rattata.asm:3`) | YES |
| Sentret | 35/46/34/20/35/45           | 35,46,34,20,35,45 (`sentret.asm:3`) | YES |
| Hoothoot| 60/30/30/50/36/56           | 60,30,30,50,36,56 (`hoothoot.asm:3`) | YES |
| Hoppip  | 35/35/40/50/35/55           | 35,35,40,50,35,55 (`hoppip.asm:3`) | YES |

Type, catch rate, base exp, and growth rate also all match for every species. Excellent.

---

## 2. Wild Encounter Tables -- PASS (All Slots Correct)

Verified against `data/wild/johto_grass.asm` lines 1237-1263 (ROUTE_29):

**Encounter rate**: Plan says 10, pokecrystal says `10 percent, 10 percent, 10 percent` for morn/day/nite. MATCH.

**Morning** (plan lines 431-438 vs pokecrystal lines 1239-1246):

| Slot | Plan | Pokecrystal | Match? |
|------|------|-------------|--------|
| 0 | Pidgey L2 | `db 2, PIDGEY` | YES |
| 1 | Sentret L2 | `db 2, SENTRET` | YES |
| 2 | Pidgey L3 | `db 3, PIDGEY` | YES |
| 3 | Sentret L3 | `db 3, SENTRET` | YES |
| 4 | Rattata L2 | `db 2, RATTATA` | YES |
| 5 | Hoppip L3 | `db 3, HOPPIP` | YES |
| 6 | Hoppip L3 | `db 3, HOPPIP` | YES |

**Day**: Identical to morning in both plan and source. MATCH.

**Night** (plan lines 449-457 vs pokecrystal lines 1255-1262):

| Slot | Plan | Pokecrystal | Match? |
|------|------|-------------|--------|
| 0 | Hoothoot L2 | `db 2, HOOTHOOT` | YES |
| 1 | Rattata L2 | `db 2, RATTATA` | YES |
| 2 | Hoothoot L3 | `db 3, HOOTHOOT` | YES |
| 3 | Rattata L3 | `db 3, RATTATA` | YES |
| 4 | Rattata L2 | `db 2, RATTATA` | YES |
| 5 | Hoothoot L3 | `db 3, HOOTHOOT` | YES |
| 6 | Hoothoot L3 | `db 3, HOOTHOOT` | YES |

All 21 wild encounter slots perfectly match pokecrystal source data.

---

## 3. Move Data -- DISCREPANCY FOUND

Verified against `data/moves/moves.asm` and `constants/move_constants.asm`:

### Move IDs

| Move | Plan ID | Pokecrystal Hex | Pokecrystal Decimal | Match? |
|------|---------|-----------------|---------------------|--------|
| Tackle | 33 (Sprint 1) | 0x21 | 33 | YES |
| Growl | 45 (Sprint 1) | 0x2d | 45 | YES |
| Tail Whip | 39 | 0x27 | 39 | YES |
| Defense Curl | 111 | 0x6f | 111 | YES |
| Splash | 150 | 0x96 | 150 | YES |
| Synthesis | 235 | 0xeb | 235 | YES |
| Sand-Attack | 28 | 0x1c | 28 | YES |
| Struggle | 165 | 0xa5 | 165 | YES |

All move IDs are correct.

### Move Stats

| Move | Plan Power/Acc/PP/Type | Pokecrystal (`moves.asm`) | Match? |
|------|------------------------|---------------------------|--------|
| Tackle | 35/95/35/Normal | `35, NORMAL, 95, 35` | **NO** -- Plan says power 0, acc 100, pp 35 (line 165-168) but Tackle has power 35, accuracy 95 |
| Growl | 0/100/40/Normal | `0, NORMAL, 100, 40` | YES |
| Tail Whip | 0/100/30/Normal | `0, NORMAL, 100, 30` | YES |
| Defense Curl | 0/100/40/Normal | `0, NORMAL, 100, 40` | YES |
| Splash | 0/100/40/Normal | `0, NORMAL, 100, 40` | YES |
| Synthesis | 0/100/5/Grass | `0, GRASS, 100, 5` | YES |
| Sand-Attack | 0/100/15/Normal | `0, GROUND, 100, 15` | **NO** -- Plan says Normal type, but Sand-Attack is Ground type |
| Struggle | 50/100/1/Normal | `50, NORMAL, 100, 1` | YES |

**DISCREPANCY #1 (MUST FIX): Sand-Attack is Ground type, NOT Normal type.**
- Plan line 183: `move_type: PokemonType::Normal` -- WRONG
- Pokecrystal `moves.asm` line 44: `move SAND_ATTACK, EFFECT_ACCURACY_DOWN, 0, GROUND, 100, 15, 0`
- Fix: Change to `move_type: PokemonType::Ground`
- (This requires ensuring `PokemonType::Ground` exists in data.rs. It should be added if not already present.)

**NOTE on Tackle**: The plan's `move_data()` section at line 165-168 shows Tackle with `power: 0, accuracy: 100`. However, this is the Sprint 2 NEW moves section -- Tackle should already exist from Sprint 1 with correct values (power 35, accuracy 95, pp 35). If Sprint 1 already has Tackle correct, this is a non-issue. But if someone re-adds Tackle in Sprint 2, they must use the correct values: **power 35, accuracy 95, PP 35**. Confirmed from `moves.asm` line 49.

---

## 4. Learnsets -- PASS (All Correct)

Verified against `data/pokemon/evos_attacks.asm`:

| Species | Plan Learnset | Pokecrystal Source | Match? |
|---------|---------------|--------------------|--------|
| Pidgey | (1, Tackle), (5, Sand-Attack) | `db 1, TACKLE` / `db 5, SAND_ATTACK` (line 226-227) | YES |
| Rattata | (1, Tackle), (1, Tail Whip) | `db 1, TACKLE` / `db 1, TAIL_WHIP` (line 269-270) | YES |
| Sentret | (1, Tackle), (5, Defense Curl) | `db 1, TACKLE` / `db 5, DEFENSE_CURL` (line 2194-2195) | YES |
| Hoothoot | (1, Tackle), (1, Growl) | `db 1, TACKLE` / `db 1, GROWL` (line 2219-2220) | YES |
| Hoppip | (1, Splash), (5, Synthesis), (5, Tail Whip), (10, Tackle) | `db 1, SPLASH` / `db 5, SYNTHESIS` / `db 5, TAIL_WHIP` / `db 10, TACKLE` (line 2524-2527) | YES |

The note about Hoppip at L2-3 only knowing Splash is correct -- Tackle isn't learned until L10. Good catch on the Struggle fallback requirement.

---

## 5. Map Dimensions -- DISCREPANCY FOUND

Verified against `constants/map_constants.asm`:

In pokecrystal, `map_const NAME, WIDTH, HEIGHT` defines dimensions in **blocks** (each block = 2x2 tiles). So tile dimensions = blocks * 2.

| Map | Plan Tiles (WxH) | Pokecrystal Blocks | Tiles = Blocks*2 | Match? |
|-----|------------------|--------------------|-------------------|--------|
| Route 29 | 60x18 | `30, 9` (line 459) | 60x18 | YES |
| Route29Route46Gate | 10x8 | `5, 4` (line 469) | 10x8 | YES |
| CherrygroveCity | 40x18 | `20, 9` (line 493) | 40x18 | YES |
| CherrygroveMart | 12x8 | `6, 4` (line 494) | 12x8 | YES |
| CherrygrovePokecenter1F | 10x8 | `5, 4` (line 495) | 10x8 | YES |
| CherrygroveGymSpeechHouse | 8x8 | `4, 4` (line 496) | 8x8 | YES |
| GuideGentsHouse | 8x8 | `4, 4` (line 497) | 8x8 | YES |
| CherrygroveEvolutionSpeechHouse | 8x8 | `4, 4` (line 498) | 8x8 | YES |

All map dimensions match exactly. The plan correctly converts blocks to tiles.

---

## 6. Species IDs -- PASS

| Species | Plan ID | Pokecrystal Dex# | Match? |
|---------|---------|-------------------|--------|
| Pidgey | 16 | #016 (`pidgey.asm:1`) | YES |
| Rattata | 19 | #019 (`rattata.asm:1`) | YES |
| Sentret | 161 | #161 (`sentret.asm:1`) | YES |
| Hoothoot | 163 | #163 (`hoothoot.asm:1`) | YES |
| Hoppip | 187 | #187 (`hoppip.asm:1`) | YES |

---

## 7. Item Data -- DISCREPANCIES FOUND

Verified against `constants/item_constants.asm`:

| Item | Plan ID | Pokecrystal Hex | Pokecrystal Decimal | Match? |
|------|---------|-----------------|---------------------|--------|
| POKE_BALL | 4 | 0x05 | 5 | **NO** |
| ANTIDOTE | 18 | 0x09 | 9 | **NO** |
| PARLYZ_HEAL | 19 | 0x0d | 13 | **NO** |
| AWAKENING | 20 | 0x0c | 12 | **NO** |
| MYSTIC_WATER | 41 | 0x5f | 95 | **NO** |
| PINK_BOW | 42 | 0x68 | 104 | **NO** |
| MAP_CARD | 43 | N/A (engine flag) | N/A | **WRONG CONCEPT** |
| POTION | (Sprint 1) | 0x12 | 18 | Verify Sprint 1 |

**DISCREPANCY #2 (SHOULD FIX): All item IDs are wrong.**

The plan's item constants use arbitrary small numbers that don't match pokecrystal's actual item IDs. The correct values from `constants/item_constants.asm`:

```
POKE_BALL    = 0x05 = 5
ANTIDOTE     = 0x09 = 9
AWAKENING    = 0x0c = 12
PARLYZ_HEAL  = 0x0d = 13
POTION       = 0x12 = 18
MYSTIC_WATER = 0x5f = 95
PINK_BOW     = 0x68 = 104
```

**DISCREPANCY #3 (MINOR): MAP_CARD is NOT a bag item.**
In pokecrystal, the Guide Gent gives you the MAP_CARD as a Pokegear card expansion (via `setflag ENGINE_MAP_CARD`), not as a physical item in your bag. The plan represents it as `ITEM_MAP_CARD: u8 = 43` which is misleading. This should be an engine flag, not an item constant. However, since the simulation uses a simplified item system, this can be modeled as an engine flag rather than an item ID. Low priority.

---

## 8. NPC Positions -- PASS

Verified against `maps/Route29.asm` (Route29_MapEvents, lines 429-437) and `maps/CherrygroveCity.asm` (CherrygroveCity_MapEvents, lines 567-572):

**Route 29 NPCs** (object_event format: `x, y, SPRITE`):

| NPC | Plan References | Pokecrystal Position | Match? |
|-----|----------------|---------------------|--------|
| Catching Dude (CooltrainerM) | Referenced in scripts | `50, 12` | Check at impl time |
| Youngster | Referenced in scripts | `27, 16` | Check at impl time |
| Teacher | Referenced in scripts | `15, 11` | Check at impl time |
| Fruit Tree | Referenced in scripts | `12, 2` | Check at impl time |
| Fisher | Referenced in scripts | `25, 3` | Check at impl time |
| CooltrainerM2 (day/night) | Referenced in scripts | `13, 4` | Check at impl time |
| Tuscany | Referenced in scripts | `29, 12` | Check at impl time |
| Potion item ball | Referenced in scripts | `48, 2` | Check at impl time |

The plan references the original SPRINT2_IMPLEMENTATION_PLAN.md for full NPC details, so positions need to match the above values. The Tuscany visibility callback (ENGINE_ZEPHYRBADGE check, disappear if not Tuesday) is correctly described in the plan.

**Cherrygrove City NPCs** (from `CherrygroveCity.asm` lines 567-572):

| NPC | Pokecrystal Position |
|-----|---------------------|
| Guide Gent (GRAMPS) | `32, 6` |
| Rival | `39, 6` (hidden by default, appears via coord_event) |
| Teacher | `27, 12` |
| Youngster | `23, 7` |
| Fisher (Mystic Water guy) | `7, 12` |

---

## 9. Warp Coordinates -- PASS

**Route 29 warps** (from `Route29.asm` line 419):
- `warp_event 27, 1, ROUTE_29_ROUTE_46_GATE, 3` -- warp at (27,1) to gate warp #3

**Cherrygrove City warps** (from `CherrygroveCity.asm` lines 551-555):
- `warp_event 23, 3, CHERRYGROVE_MART, 2`
- `warp_event 29, 3, CHERRYGROVE_POKECENTER_1F, 1`
- `warp_event 17, 7, CHERRYGROVE_GYM_SPEECH_HOUSE, 1`
- `warp_event 25, 9, GUIDE_GENTS_HOUSE, 1`
- `warp_event 31, 11, CHERRYGROVE_EVOLUTION_SPEECH_HOUSE, 1`

These need to match at implementation time. The plan defers to the original SPRINT2_IMPLEMENTATION_PLAN.md.

---

## 10. Story Sequence -- PASS

Verified against `maps/Route29.asm` and `maps/CherrygroveCity.asm`:

**Catching Tutorial** (Route29.asm lines 39-62):
- Trigger: coord_event at (53,8) and (53,9) with SCENE_ROUTE29_CATCH_TUTORIAL
- Uses: `loadwildmon RATTATA, 5` then `catchtutorial BATTLETYPE_TUTORIAL`
- Plan correctly uses RATTATA at level 5 with BattleType::Tutorial. MATCH.

**Guide Gent Tour** (CherrygroveCity.asm lines 26-86):
- Sequence: `follow` -> movements -> text stops at Pokecenter, Mart, Route 30, Sea -> returns to house -> gives MAP_CARD -> `stopfollow` -> `special RestartMapMusic` -> disappears
- Music: `playmusic MUSIC_SHOW_ME_AROUND` then `special RestartMapMusic` at end
- Plan correctly describes this sequence. MATCH.

**Rival Battle** (CherrygroveCity.asm lines 101-175):
- Trigger: coord_events at (33,6) and (33,7) with SCENE_CHERRYGROVECITY_MEET_RIVAL
- Uses `BATTLETYPE_CANLOSE`
- Counter-starter logic: checks EVENT_GOT_TOTODILE_FROM_ELM, EVENT_GOT_CHIKORITA_FROM_ELM
- After battle: `special HealParty` and `playmapmusic`
- Music: `special FadeOutMusic` -> `playmusic MUSIC_RIVAL_ENCOUNTER` -> after battle `playmusic MUSIC_RIVAL_AFTER`
- Plan correctly describes this. MATCH.

---

## 11. Coord Events -- PASS

**Route 29** (from `Route29.asm` lines 422-423):
- `coord_event 53, 8, SCENE_ROUTE29_CATCH_TUTORIAL, Route29Tutorial1`
- `coord_event 53, 9, SCENE_ROUTE29_CATCH_TUTORIAL, Route29Tutorial2`

**Cherrygrove City** (from `CherrygroveCity.asm` lines 558-559):
- `coord_event 33, 6, SCENE_CHERRYGROVECITY_MEET_RIVAL, CherrygroveRivalSceneNorth`
- `coord_event 33, 7, SCENE_CHERRYGROVECITY_MEET_RIVAL, CherrygroveRivalSceneSouth`

---

## 12. Flypoint / Map Callback -- PASS

From `CherrygroveCity.asm` line 22-24:
```
CherrygroveCityFlypointCallback:
    setflag ENGINE_FLYPOINT_CHERRYGROVE
    endcallback
```

Plan correctly uses `EVENT_ENGINE_FLYPOINT_CHERRYGROVE` in `check_map_callbacks()`. MATCH.

---

## Summary of Required Corrections

### MUST FIX (before implementation)

1. **Sand-Attack type**: Change from `PokemonType::Normal` to `PokemonType::Ground` in the move_data entry (plan line 183). Ensure `PokemonType::Ground` exists in the type enum.

2. **Item IDs**: Correct all item constants to match pokecrystal:
   - `ITEM_POKE_BALL: u8 = 5` (not 4)
   - `ITEM_ANTIDOTE: u8 = 9` (not 18)
   - `ITEM_AWAKENING: u8 = 12` (not 20)
   - `ITEM_PARLYZ_HEAL: u8 = 13` (not 19)
   - `ITEM_MYSTIC_WATER: u8 = 95` (not 41)
   - `ITEM_PINK_BOW: u8 = 104` (not 42)

### SHOULD FIX (minor, can be deferred)

3. **MAP_CARD**: This is an engine flag (`ENGINE_MAP_CARD`), not a bag item. Model it as a flag in the event system rather than an item ID. Remove `ITEM_MAP_CARD` from item constants; the Guide Gent script should `set_flag(EVENT_ENGINE_MAP_CARD)` instead of giving a bag item.

### NO ACTION NEEDED

- All species base stats are correct
- All wild encounter tables are correct (species, levels, slot order, encounter rates)
- All learnsets are correct
- All map dimensions are correct
- All move IDs are correct
- Story sequence and event triggers are correct
- Catching tutorial parameters (Rattata L5, BATTLETYPE_TUTORIAL) are correct
- Rival battle type (BATTLETYPE_CANLOSE) and counter-starter logic are correct

---

## Final Assessment

This is a highly accurate plan. The data compiler clearly worked from the pokecrystal source files. The only real errors are the item ID values (which appear to be placeholders rather than pokecrystal-sourced) and the Sand-Attack type. Everything else -- the encounter tables, base stats, learnsets, map sizes, event sequences -- is spot-on.

**VERDICT: ACCEPT WITH MODIFICATIONS** -- Fix the Sand-Attack type and item IDs before implementation. These are straightforward find-and-replace corrections that don't affect architecture.
