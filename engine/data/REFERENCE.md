# Pokemon Gold/Silver — Master Game Reference

> **Purpose**: Single source of truth for building and designing the Pokemon Gold/Silver
> recreation. All game rules, progression gates, map connectivity, and mechanics are
> documented here. Consult before implementing any feature. Detailed data lives in
> companion files referenced below.
>
> **Sources**: Bulbapedia, Serebii, pret/pokecrystal disassembly
>
> **Last updated**: Sprint 86

---

## Table of Contents

1. [Data File Index](#1-data-file-index)
2. [Complete Game Progression](#2-complete-game-progression)
3. [Map Connectivity Graph](#3-map-connectivity-graph)
4. [Progression Gates](#4-progression-gates)
5. [Story Flags](#5-story-flags)
6. [City & Town Reference](#6-city--town-reference)
7. [Interior Maps](#7-interior-maps)
8. [Key Mechanics Rules](#8-key-mechanics-rules)
9. [Physical/Special Split](#9-physicalspecial-split)
10. [Implementation Checklist](#10-implementation-checklist)

---

## 1. Data File Index

| File | Contents |
|------|----------|
| `engine/data/REFERENCE.md` | This file — master index and game design rules |
| `engine/data/gym_e4_rival_data.txt` | All 8 gym leaders, E4, Champion, Rival (7 encounters) with full movesets |
| `engine/docs/gen2_battle_mechanics.txt` | Damage formula, crit rates, stat calc, EXP, catch rate, status effects, type chart |
| `engine/JOHTO_DATA_ROUTES_29_33.txt` | Routes 29-33: wild encounters, trainers, connections |
| `engine/data/johto_routes_34_46.txt` | Routes 34-46 + dungeon data (when compiled) |
| `engine/data/gen2_moves_pokemon.txt` | Move data, base stats, learnsets, evolutions (when compiled) |
| `engine/ENGINE_POKEMON.md` | Engine architecture notes from 45+ sprints |

---

## 2. Complete Game Progression

The player must complete these in order. Each section lists required actions and gates.

### Part 1: New Bark Town → Cherrygrove City
1. Wake up, go downstairs
2. Visit Elm's Lab, receive starter Pokemon (Chikorita/Cyndaquil/Totodile)
3. Exit east to Route 29
4. Route 29 → west to Cherrygrove City (no trainers)
5. Guide Gent shows you around Cherrygrove (optional)

### Part 2: Mr. Pokemon's Errand
6. North to Route 30
7. Route 30 → Mr. Pokemon's House (receive Mystery Egg + Pokedex from Oak)
8. Elm calls — return to New Bark Town
9. **RIVAL BATTLE 1**: Silver outside Cherrygrove (1 Pokemon, Lv.5)
10. Return egg to Elm, name rival, receive Pokeballs

### Part 3: Violet City
11. Route 30 → Route 31 → Violet City
12. (Optional) Sprout Tower — 3 floors, Sage battles, TM70 Flash at top
13. **GYM 1**: Falkner — Pidgey Lv.7, Pidgeotto Lv.9 → **Zephyr Badge** (Flash HM)

### Part 4: Union Cave
14. South to Route 32 (8 trainers)
15. (Optional) Ruins of Alph — puzzle rooms, Unown
16. Route 32 south → Union Cave
17. Union Cave → Route 33

### Part 5: Azalea Town
18. Route 33 → Azalea Town
19. Slowpoke Well — defeat Team Rocket (3 Rocket Grunts + Executive)
20. **GYM 2**: Bugsy — Metapod Lv.14, Kakuna Lv.14, Scyther Lv.16 → **Hive Badge** (Cut HM)
21. **RIVAL BATTLE 2**: Silver at Azalea Town entrance (3 Pokemon)
22. Ilex Forest — Cut required, Farfetch'd quest, receive HM01 Cut
23. Ilex Forest → Route 34

### Part 6: Goldenrod City
24. Route 34 → Goldenrod City (largest city)
25. (Optional) Goldenrod Dept Store, Game Corner, Name Rater, Bike Shop
26. **GYM 3**: Whitney — Clefairy Lv.18, Miltank Lv.20 → **Plain Badge** (Strength HM)
27. Whitney cries, talk to Lass in gym to get badge + TM45 Attract

### Part 7: National Park → Ecruteak
28. North to Route 35
29. Route 35 → National Park (Bug-Catching Contest on Tue/Thu/Sat)
30. National Park → Route 36
31. **GATE**: Sudowoodo blocks Route 36 — need SquirtBottle from Goldenrod flower shop
    (flower shop only gives it after you have Plain Badge)
32. Route 36 → Route 37 → Ecruteak City

### Part 8: Ecruteak City
33. Ecruteak City — Kimono Girls, Bill, burned tower
34. Burned Tower — encounter legendary beasts (Raikou, Entei, Suicune flee)
35. **RIVAL BATTLE 3**: Silver in Burned Tower (4 Pokemon)
36. **GYM 4**: Morty — Gastly Lv.21, 2x Haunter Lv.21/23, Gengar Lv.25 → **Fog Badge** (Surf HM)
37. Route 38 → Route 39

### Part 9: Olivine City
38. Route 39 → Olivine City
39. Olivine Lighthouse — Jasmine's Ampharos is sick, needs medicine
40. **GATE**: Cannot fight Olivine Gym until medicine is delivered

### Part 10: Cianwood City
41. Route 40 (Surf required) → Route 41 → Cianwood City
42. **GYM 5**: Chuck — Primeape Lv.27, Poliwrath Lv.30 → **Storm Badge** (Fly HM)
43. Get SecretPotion from Cianwood pharmacy
44. Fly back to Olivine, deliver medicine to Jasmine

### Part 11: Olivine Gym + Mahogany
45. **GYM 6**: Jasmine — 2x Magnemite Lv.30, Steelix Lv.35 → **Mineral Badge**
46. Route 42 → Mahogany Town (or Surf east from Ecruteak)

### Part 12: Lake of Rage + Rocket Hideout
47. Route 43 → Lake of Rage (Red Gyarados event)
48. Meet Lance at Lake of Rage
49. Rocket Hideout in Mahogany Town (multi-floor dungeon with Lance)
50. Defeat Team Rocket Executive
51. **GYM 7**: Pryce — Seel Lv.27, Dewgong Lv.29, Piloswine Lv.31 → **Glacier Badge** (Whirlpool HM)

### Part 13: Radio Tower Takeover
52. Return to Goldenrod City — Radio Tower taken over by Team Rocket
53. Clear Radio Tower (5 floors of Rocket battles)
54. Goldenrod Underground — get Basement Key
55. Defeat Rocket Executive, free Director
56. Receive Clear Bell (Gold) / Silver Wing (Silver)

### Part 14: Route 44 → Blackthorn City
57. Route 44 → Ice Path (Strength + Waterfall puzzle)
58. Ice Path → Blackthorn City
59. **GYM 8**: Clair — 3x Dragonair Lv.37, Kingdra Lv.40 → **Rising Badge** (Waterfall HM)
60. **GATE**: Clair refuses badge — must pass Dragon's Den quiz first
61. Dragon's Den — answer Master's questions correctly
62. Clair gives Rising Badge + TM24 DragonBreath

### Part 15: Victory Road → Pokemon League
63. Fly to New Bark Town, Surf east to Route 27
64. Route 27 → Route 26 → Victory Road
65. **RIVAL BATTLE 5**: Silver in Victory Road (6 Pokemon)
66. Indigo Plateau:
    - **E4 #1**: Will (Psychic) — 5 Pokemon, Lv.40-42
    - **E4 #2**: Koga (Poison) — 5 Pokemon, Lv.40-44
    - **E4 #3**: Bruno (Fighting) — 5 Pokemon, Lv.42-46
    - **E4 #4**: Karen (Dark) — 5 Pokemon, Lv.42-47
    - **CHAMPION**: Lance — 6 Pokemon, Lv.44-50

---

## 3. Map Connectivity Graph

### Legend
- `→` = one-way (ledge/scripted)
- `↔` = bidirectional walk
- `~` = requires Surf
- `[GATE]` = progression gate (see Section 4)

### Main Path
```
NewBarkTown ↔ Route29 ↔ CherrygroveCity ↔ Route30 ↔ Route31 ↔ VioletCity
                |                                         |
                ↔ Route46 (ledge down only from R46)      ↔ DarkCave(west)

VioletCity ↔ Route32 ↔ UnionCave ↔ Route33 ↔ AzaleaTown
               |                                  |
               ↔ RuinsOfAlph                      ↔ SlowpokeWell
                                                  |
                                                  ↔ IlexForest ↔ Route34 ↔ GoldenrodCity

GoldenrodCity ↔ Route35 ↔ NationalPark ↔ Route36 ↔ Route37 ↔ EcruteakCity
                                            |
                                            ↔ VioletCity (via Route36 south fork)

EcruteakCity ↔ Route38 ↔ Route39 ↔ OlivineCity
     |                                  |
     ↔ BurnedTower                      ↔ OlivineLighthouse
     ↔ Route42 ↔ MtMortar ↔ MahoganyTown

OlivineCity ~ Route40 ~ Route41 ~ CianwoodCity

MahoganyTown ↔ Route43 ↔ LakeOfRage
     |
     ↔ RocketHideout
     ↔ Route44 ↔ IcePath ↔ BlackthornCity
                               |
                               ↔ DragonsDen
                               ↔ Route45 → Route46 (one-way ledges down)

NewBarkTown ~ Route27 ~ Route26 ~ VictoryRoad ~ IndigoPlateauGate
```

### Interior Connections (per city)

| City | Interior Maps |
|------|--------------|
| New Bark Town | PlayerHouse1F/2F, ElmLab, GenericHouse |
| Cherrygrove City | PokemonCenter, GenericHouse(x2) |
| Violet City | PokemonCenter, VioletGym, SproutTower, GenericHouse |
| Azalea Town | PokemonCenter, AzaleaGym, GenericHouse, SlowpokeWell |
| Goldenrod City | PokemonCenter, GoldenrodGym, DeptStore, GenericHouse(x3+) |
| Ecruteak City | PokemonCenter, EcruteakGym, BurnedTower, GenericHouse |
| Olivine City | PokemonCenter, OlivineGym, OlivineLighthouse, GenericHouse |
| Cianwood City | PokemonCenter, CianwoodGym, GenericHouse |
| Mahogany Town | PokemonCenter, MahoganyGym, RocketHideout, GenericHouse |
| Blackthorn City | PokemonCenter, BlackthornGym, DragonsDen, GenericHouse |

---

## 4. Progression Gates

These MUST be enforced to prevent sequence breaking. Listed in order encountered.

| # | Location | Gate Type | Requirement | What Happens |
|---|----------|-----------|-------------|-------------|
| G1 | Route 29 → Route 46 | One-way | Ledge | Can see Route 46 entrance but can only jump DOWN from 46→29, not climb up |
| G2 | Route 32 → Union Cave | Badge check | Zephyr Badge | NPC blocks path south without badge |
| G3 | Ilex Forest → Route 34 | Badge check | Hive Badge | Cannot use Cut without Hive Badge; trees block path |
| G4 | Route 36 (Sudowoodo) | Item check | SquirtBottle | Sudowoodo blocks path; SquirtBottle only available after Plain Badge |
| G5 | Route 40/41 | HM check | Surf (Fog Badge) | Cannot cross water to Cianwood without Surf |
| G6 | Olivine Gym | Story flag | Medicine delivered | Jasmine won't battle until Ampharos is healed |
| G7 | Route 43 → Lake of Rage | Story check | After Pryce? | Accessible but Red Gyarados event triggers Rocket subplot |
| G8 | Radio Tower | Story flag | Rocket subplot | Rockets take over after Mahogany events |
| G9 | Ice Path | Item/HM check | Strength (Plain Badge) + Waterfall | Boulders require Strength; waterfall at end |
| G10 | Dragon's Den | Story flag | Beat Clair | Must pass quiz to get Rising Badge |
| G11 | Route 27 | Badge check | All 8 badges | Gate guard checks all badges before allowing passage |
| G12 | Victory Road | Badge check | All 8 badges | Gate at entrance |

### Our Implementation Status
- G1: Implemented (Route 46 not yet built as full map, one-way ledge from 46→29)
- G2: Implemented (warp intercept checks Zephyr Badge)
- G3: Implemented (warp intercept checks Hive Badge)
- G4: NOT YET — Sudowoodo/SquirtBottle system needed
- G5: NOT YET — Surf HM overworld usage needed
- G6: NOT YET — Lighthouse subplot needed
- G7: NOT YET — Lake of Rage not built
- G8: NOT YET — Radio Tower subplot needed
- G9: NOT YET — Ice Path not built
- G10: NOT YET — Dragon's Den not built
- G11: Implemented (warp intercept checks 8 badges)
- G12: NOT YET — Victory Road not built

---

## 5. Story Flags

Flags that should be tracked in the save system to control progression.

| Flag Name | Set When | Used For |
|-----------|----------|----------|
| `FLAG_GOT_STARTER` | Received starter from Elm | Allow leaving New Bark Town |
| `FLAG_GOT_EGG` | Visited Mr. Pokemon | Trigger rival battle, Elm call |
| `FLAG_DELIVERED_EGG` | Returned egg to Elm | Receive Pokeballs, unlock Pokegear |
| `FLAG_BEAT_FALKNER` | Beat Gym 1 | G2 gate, Sprout Tower complete |
| `FLAG_BEAT_BUGSY` | Beat Gym 2 | G3 gate, Cut usable |
| `FLAG_BEAT_WHITNEY` | Beat Gym 3 | SquirtBottle available, G4 gate |
| `FLAG_CLEARED_SUDOWOODO` | Used SquirtBottle | Route 36 passable |
| `FLAG_BEAT_MORTY` | Beat Gym 4 | Surf usable, G5 gate |
| `FLAG_BEAT_CHUCK` | Beat Gym 5 | Fly usable, SecretPotion |
| `FLAG_DELIVERED_MEDICINE` | Gave medicine to Jasmine | G6 gate, can fight Jasmine |
| `FLAG_BEAT_JASMINE` | Beat Gym 6 | Mineral Badge |
| `FLAG_RED_GYARADOS` | Encountered Red Gyarados | Rocket subplot triggers |
| `FLAG_ROCKET_MAHOGANY` | Cleared Rocket Hideout | Mahogany Gym accessible |
| `FLAG_BEAT_PRYCE` | Beat Gym 7 | Whirlpool usable |
| `FLAG_ROCKET_RADIO` | Cleared Radio Tower | Route 44 / Ice Path accessible |
| `FLAG_BEAT_CLAIR` | Beat Gym 8 | Dragon's Den quiz |
| `FLAG_DRAGON_DEN` | Passed Dragon's Den quiz | Rising Badge, Waterfall usable |
| `FLAG_BEAT_E4` | Beat Champion Lance | Kanto post-game unlocked |
| `badges` | Bitmask (8 bits) | Badge-based gates, stat boosts |

---

## 6. City & Town Reference

### Buildings Per City (Gold/Silver)

**New Bark Town** (pop. 11)
- Player's House (2 floors)
- Elm's Pokemon Lab
- 1 NPC house (neighbor)
- No Poke Mart, no Pokemon Center

**Cherrygrove City** (pop. 22)
- Pokemon Center
- 2 NPC houses
- Guide Gent's house
- No Gym

**Violet City** (pop. 52)
- Pokemon Center
- Poke Mart
- Violet Gym (Falkner)
- Sprout Tower (3F)
- Earl's Pokemon Academy
- 2 NPC houses

**Azalea Town** (pop. 31)
- Pokemon Center
- Azalea Gym (Bugsy)
- Kurt's house (Apricorn ball maker)
- Slowpoke Well entrance
- Charcoal Kiln
- 1 NPC house

**Goldenrod City** (pop. 126)
- Pokemon Center
- Goldenrod Gym (Whitney)
- Department Store (6F)
- Goldenrod Game Corner
- Radio Tower (5F)
- Underground (Basement)
- Name Rater's house
- Bike Shop
- Flower Shop
- Bill's house (after Ecruteak)
- Multiple NPC houses

**Ecruteak City** (pop. 46)
- Pokemon Center
- Ecruteak Gym (Morty)
- Burned Tower
- Tin Tower entrance
- Dance Theater (5 Kimono Girls)
- Bill's family home
- 2 NPC houses

**Olivine City** (pop. 47)
- Pokemon Center
- Olivine Gym (Jasmine)
- Olivine Lighthouse (6F)
- S.S. Aqua port (post-game)
- Poke Mart
- 2 NPC houses

**Cianwood City** (pop. 27)
- Pokemon Center
- Cianwood Gym (Chuck)
- Cianwood Pharmacy
- Photo Studio
- 1 NPC house

**Mahogany Town** (pop. 26)
- Pokemon Center
- Mahogany Gym (Pryce)
- Rocket Hideout (under shop)
- Souvenir Shop
- 1 NPC house

**Blackthorn City** (pop. 40)
- Pokemon Center
- Blackthorn Gym (Clair)
- Dragon's Den entrance
- Move Deleter's house
- Move Tutor's house
- 1 NPC house

---

## 7. Interior Maps

### Shared Interiors (our implementation)
We use shared `MapId` for common interiors to save map data:

| Shared Map | Used By | Dynamic Exit |
|-----------|---------|-------------|
| `PokemonCenter` | All cities | `last_pokecenter_map` tracks which city to exit to |
| `GenericHouse` | All NPC houses | `last_house_map` + `last_house_x/y` tracks exact door position |

### PokemonCenter Layout (universal)
- Healing counter (top center) — Nurse Joy
- PC (top-left corner) — Bill's PC for box storage
- Exit door (bottom center)
- Size: 10x8 tiles

### GenericHouse Layout (universal)
- NPC inside (dialogue varies by which door entered — use last_house_map to determine)
- Bookshelf, table
- Exit door (bottom center)
- Size: 8x6 tiles

### Unique Interior Maps
- PlayerHouse1F/2F — New Bark, player starts upstairs
- ElmLab — starter selection, egg delivery
- VioletGym — platform-style with elevated walkways
- AzaleaGym — spider web floor
- GoldenrodGym — Clefairy maze
- EcruteakGym — invisible floor puzzle
- OlivineGym — straightforward
- SproutTower — 3 floors, shaking pillar
- BurnedTower — falling floor, legendary beast encounter
- OlivineLighthouse — 6 floors
- SlowpokeWell — cave with Rockets
- IlexForest — outdoor maze, Farfetch'd chase

---

## 8. Key Mechanics Rules

### Battle System
- **Turn order**: Determined by Speed stat (higher goes first). Priority moves override.
- **Accuracy check**: `hit = (move_accuracy * accuracy_stage / evasion_stage) > random(0-99)`
- **Damage**: See `gen2_battle_mechanics.txt` for full formula
- **Critical hits**: ~6.6% base rate, 2x damage (see mechanics file for stage table)
- **STAB**: 1.5x if move type matches Pokemon type
- **Type effectiveness**: 0x (immune), 0.5x, 1x, 2x, 4x (dual type)

### Wild Encounters
- **Encounter rate**: Each step in tall grass has ~8.5% chance (varies by area)
- **Species selection**: Weighted random from encounter table
- **Time-of-day**: Morning (4-10), Day (10-18), Night (18-4) — different encounter tables
- **Repel**: Blocks encounters with Pokemon whose level < lead Pokemon's level

### Trainer Battles
- **Line of sight**: Trainers see 1-5 tiles ahead in their facing direction
- **Exclamation mark**: Trainer walks to player, forced battle
- **Defeated trainers**: Never re-battle (except Pokegear rematch trainers)
- **Prize money**: Base rate * highest-level Pokemon's level
- **Forced switch**: When active Pokemon faints, must switch before opponent attacks
- **All fainted**: Black out → return to last Pokemon Center, lose half money

### Experience & Leveling
- **EXP formula**: `base_exp * level / 7` (simplified; full formula in mechanics file)
- **Level up**: Check after every EXP gain. Can level up multiple times.
- **Move learning**: At each level-up, check learnset. If learning a move and party has 4 moves, prompt to forget one.
- **Evolution**: Check after level-up. Some Pokemon evolve at specific levels.
- **Stat recalculation**: Recalculate ALL stats on every level-up.

### Catching Pokemon
- **Formula**: See `gen2_battle_mechanics.txt` for full formula
- **Key rule**: Lower HP = higher catch rate. Status conditions help (Sleep/Freeze best).
- **Ball order**: Poke Ball < Great Ball < Ultra Ball < Master Ball
- **Shake animation**: 0-3 shakes before catch/escape result
- **Known Gen2 bug**: Paralysis/Burn/Poison status bonus is skipped (coded but bugged)

### Items
- **Pokeball types**: Poke Ball, Great Ball, Ultra Ball, Master Ball (+ Apricorn balls)
- **Healing**: Potion (20), Super Potion (50), Hyper Potion (200), Max Potion (full), Full Restore (full+status)
- **Status healing**: Antidote, Parlyz Heal, Awakening, Burn Heal, Ice Heal, Full Heal
- **Revive**: Revive (half HP), Max Revive (full HP)
- **Repel**: Repel (100 steps), Super Repel (200), Max Repel (250)

### Saving
- **Save location**: Overworld only (not in battles or menus)
- **Save data**: Party, PC boxes, bag, badges, flags, money, position, Pokedex
- **One save slot**: New game overwrites existing save

---

## 9. Physical/Special Split

**CRITICAL**: In Gen 2, move category is determined by TYPE, not per-move.

| Category | Types |
|----------|-------|
| **Physical** | Normal, Fighting, Poison, Ground, Flying, Bug, Rock, Ghost, Steel |
| **Special** | Fire, Water, Grass, Electric, Ice, Psychic, Dragon, Dark |

### Common Gotchas
- **Pursuit** (Dark) → Special (not Physical!)
- **Shadow Ball** (Ghost) → Physical
- **Fire Punch, Ice Punch, ThunderPunch** → Special (Fire/Ice/Electric types)
- **Crabhammer** (Water) → Special
- **Acid** (Poison) → Physical
- **Hidden Power** → Special (always, regardless of calculated type)
- **Hyper Beam** (Normal) → Physical
- **Gust** (Flying) → Physical
- **Bite** (Dark) → Special

---

## 10. Implementation Checklist

### Currently Implemented (through Sprint 85)
- [x] Overworld movement, tile collision, NPC interaction
- [x] Wild encounters (grass only, no time-of-day yet)
- [x] Trainer battles with LOS system
- [x] Turn-based battle with damage calc, type effectiveness
- [x] Stat stages (all 7)
- [x] Critical hits
- [x] Status conditions (poison, burn, paralysis, sleep, freeze)
- [x] Pokemon catching with ball types
- [x] Party management (switch, view stats)
- [x] PC box storage (deposit/withdraw)
- [x] Poke Mart (buy/sell)
- [x] Pokemon Center healing
- [x] Bag system with item usage
- [x] Save/Load system
- [x] Map transitions with fade
- [x] 33 maps through Olivine City
- [x] 8 gym battles
- [x] 63+ species
- [x] Progression gates (G1-G3, G11)
- [x] Sound effects (SoundCommand queue)
- [x] Shared interiors (PokemonCenter, GenericHouse)

### Not Yet Implemented
- [ ] Time-of-day system (morning/day/night encounter tables)
- [ ] Surf/Fly/Cut/Strength overworld HM usage
- [ ] Sudowoodo/SquirtBottle (G4)
- [ ] Evolution system
- [ ] Move learning on level-up (prompt to forget)
- [ ] Held items
- [ ] Weather effects (Rain Dance, Sunny Day in battle)
- [ ] Multi-hit moves display
- [ ] Confusion self-hit
- [ ] Toxic escalating damage
- [ ] Badge stat boosts in battle
- [ ] Repel system
- [ ] Fishing/Surfing encounters
- [ ] Headbutt tree encounters
- [ ] Bug-Catching Contest
- [ ] Pokegear (phone rematches)
- [ ] Apricorn ball crafting (Kurt)
- [ ] Kanto post-game
- [ ] Breeding (Day Care)
- [ ] Happiness system
- [ ] Shiny Pokemon

### Priority Order for Remaining Work
1. **Move learning on level-up** — Critical for player experience
2. **Evolution system** — Core gameplay loop
3. **Battle text improvements** — Critical hit, status damage, multi-hit display
4. **Time-of-day** — Affects encounter tables significantly
5. **Remaining maps** (Cianwood → Blackthorn → Victory Road)
6. **HM overworld usage** (Surf, Fly, Cut, Strength)
7. **Story events** (Rocket subplot, legendary beasts)
8. **Held items** — Affects battle balance
9. **Weather** — Required for some gym/E4 battles
10. **Post-game** — Kanto region

---

## Appendix A: Badge Effects

| Badge | Stat Boost | HM Unlock |
|-------|-----------|-----------|
| Zephyr (Falkner) | Attack ×1.125 | Flash |
| Hive (Bugsy) | — | Cut |
| Plain (Whitney) | Speed ×1.125 | Strength |
| Fog (Morty) | — | Surf |
| Storm (Chuck) | — | Fly |
| Mineral (Jasmine) | Defense ×1.125 | — |
| Glacier (Pryce) | SpAtk & SpDef ×1.125 | Whirlpool |
| Rising (Clair) | — | Waterfall |

## Appendix B: Rival Team Variants (Quick Ref)

Silver's team depends on player's starter choice:
- Player Chikorita → Silver has Cyndaquil line
- Player Cyndaquil → Silver has Totodile line
- Player Totodile → Silver has Chikorita line

His constant team members across all variants:
- Gastly → Haunter → Gengar
- Zubat → Golbat → Crobat
- Magnemite → Magneton
- Sneasel
- Kadabra → Alakazam

See `gym_e4_rival_data.txt` for full movesets at each encounter.

## Appendix C: Dungeon Maps

| Dungeon | Floors | Notable |
|---------|--------|---------|
| Sprout Tower | 3 | Sage trainers, TM Flash at top, Bellsprout theme |
| Slowpoke Well | 2 | Rocket Grunts, Slowpoke rescue |
| Union Cave | 2 (+ B2F Friday) | Lapras appears on B2F every Friday |
| Ilex Forest | 1 (maze) | Farfetch'd puzzle for Cut, shrine |
| Burned Tower | 2 | Fall through floor, legendary beasts flee |
| Olivine Lighthouse | 6 | Sick Ampharos at top |
| Mt. Mortar | 3 | Optional dungeon, Karate King |
| Rocket Hideout | B1-B3 | Under Mahogany souvenir shop, with Lance |
| Ice Path | B1-B3 | Sliding ice puzzles, Strength boulders |
| Dragon's Den | 1 + shrine | Whirlpool entrance, Master's quiz |
| Dark Cave | 2 (split) | West entrance from Route 31, east from Route 46 |
| Victory Road | Multiple | Final gauntlet before E4 |
| Mt. Silver | Multiple | Post-game, Red battle at summit |
