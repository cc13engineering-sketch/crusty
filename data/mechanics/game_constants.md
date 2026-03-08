# Pokemon Crystal -- Comprehensive Game Constants Reference

Source: pokecrystal constants/ directory (46 files, ~9,835 lines)

---

## Battle Constants (battle_constants.asm)

### Level Limits
- MAX_LEVEL = 100
- MIN_LEVEL = 2
- EGG_LEVEL = 5

### Stats
- NUM_MOVES = 4 (moves per Pokemon)
- BASE_STAT_LEVEL = 7 (default stat stage, +0)
- MAX_STAT_LEVEL = 13 (maximum stat stage, +6)
- MAX_STAT_VALUE = 999
- NUM_STATS = 5 (HP, Atk, Def, Spd, SAtk, SDef -- HP counted separately)
- STAT_MIN_NORMAL = 5 (minimum stat value)
- STAT_MIN_HP = 10 (minimum HP value)

### Sleep Turns
- REST_SLEEP_TURNS = 2 (Rest always sleeps for 2 turns)
- TREEMON_SLEEP_TURNS = 7 (headbutt tree encounter sleep)

### Move Priority
- BASE_PRIORITY = 1 (default priority; higher = faster)

### Type Effectiveness (scaled by 10)
- SUPER_EFFECTIVE = 20 (2x)
- MORE_EFFECTIVE = 15 (1.5x -- not used in Gen 2 type chart)
- EFFECTIVE = 10 (1x, neutral)
- NOT_VERY_EFFECTIVE = 5 (0.5x)
- NO_EFFECT = 0 (immune)

### Shiny DVs
- ATKDEFDV_SHINY = $EA (Atk=14, Def=10)
- SPDSPCDV_SHINY = $AA (Spd=10, Spc=10)
- Shiny requires: Atk DV in {2,3,6,7,10,11,14,15}, others = 10

### Battle Types (wBattleType)
| Value | Constant | Description |
|-------|----------|-------------|
| 0 | BATTLETYPE_NORMAL | Standard battle |
| 1 | BATTLETYPE_CANLOSE | Can lose without whiteout (e.g. Rival in Burned Tower) |
| 2 | BATTLETYPE_DEBUG | Debug battle |
| 3 | BATTLETYPE_TUTORIAL | Dude's catching tutorial |
| 4 | BATTLETYPE_FISH | Fishing encounter |
| 5 | BATTLETYPE_ROAMING | Roaming beast (Raikou/Entei) |
| 6 | BATTLETYPE_CONTEST | Bug Catching Contest |
| 7 | BATTLETYPE_FORCESHINY | Force shiny (unused) |
| 8 | BATTLETYPE_TREE | Headbutt tree encounter |
| 9 | BATTLETYPE_TRAP | Trapping encounter |
| 10 | BATTLETYPE_FORCEITEM | Force held item |
| 11 | BATTLETYPE_CELEBI | Celebi encounter |
| 12 | BATTLETYPE_SUICUNE | Suicune encounter |

### Status Conditions (wBattleMonStatus)
- Bits 0-2: SLP_MASK (sleep turns, 0-7)
- Bit 3: PSN (poison)
- Bit 4: BRN (burn)
- Bit 5: FRZ (freeze)
- Bit 6: PAR (paralysis)

### Sub-Status 1 (wPlayerSubStatus1/wEnemySubStatus1)
| Bit | Flag | Description |
|-----|------|-------------|
| 0 | SUBSTATUS_NIGHTMARE | Taking Nightmare damage |
| 1 | SUBSTATUS_CURSE | Cursed (Ghost Curse) |
| 2 | SUBSTATUS_PROTECT | Protected this turn |
| 3 | SUBSTATUS_IDENTIFIED | Foresight active |
| 4 | SUBSTATUS_PERISH | Perish Song countdown |
| 5 | SUBSTATUS_ENDURE | Endure active this turn |
| 6 | SUBSTATUS_ROLLOUT | In Rollout chain |
| 7 | SUBSTATUS_IN_LOVE | Attracted/Infatuated |

### Sub-Status 2 (wPlayerSubStatus2/wEnemySubStatus2)
| Bit | Flag | Description |
|-----|------|-------------|
| 0 | SUBSTATUS_CURLED | Defense Curl used (doubles Rollout) |

### Sub-Status 3 (wPlayerSubStatus3/wEnemySubStatus3)
| Bit | Flag | Description |
|-----|------|-------------|
| 0 | SUBSTATUS_BIDE | Storing energy for Bide |
| 1 | SUBSTATUS_RAMPAGE | In Thrash/Outrage/Petal Dance |
| 2 | SUBSTATUS_IN_LOOP | Multi-hit move in progress |
| 3 | SUBSTATUS_FLINCHED | Flinched this turn |
| 4 | SUBSTATUS_CHARGED | Charging (SolarBeam/Fly/Dig turn 1) |
| 5 | SUBSTATUS_UNDERGROUND | Underground (Dig) |
| 6 | SUBSTATUS_FLYING | In the air (Fly) |
| 7 | SUBSTATUS_CONFUSED | Confused |

### Sub-Status 4 (wPlayerSubStatus4/wEnemySubStatus4)
| Bit | Flag | Description |
|-----|------|-------------|
| 0 | SUBSTATUS_X_ACCURACY | X Accuracy active |
| 1 | SUBSTATUS_MIST | Mist active |
| 2 | SUBSTATUS_FOCUS_ENERGY | Focus Energy/Dire Hit active |
| 4 | SUBSTATUS_SUBSTITUTE | Has Substitute |
| 5 | SUBSTATUS_RECHARGE | Must recharge (Hyper Beam) |
| 6 | SUBSTATUS_RAGE | Rage active |
| 7 | SUBSTATUS_LEECH_SEED | Leech Seeded |

### Sub-Status 5 (wPlayerSubStatus5/wEnemySubStatus5)
| Bit | Flag | Description |
|-----|------|-------------|
| 0 | SUBSTATUS_TOXIC | Badly poisoned (toxic) |
| 3 | SUBSTATUS_TRANSFORMED | Currently Transformed |
| 4 | SUBSTATUS_ENCORED | Locked by Encore |
| 5 | SUBSTATUS_LOCK_ON | Lock-On/Mind Reader active |
| 6 | SUBSTATUS_DESTINY_BOND | Destiny Bond active |
| 7 | SUBSTATUS_CANT_RUN | Trapped (Mean Look/Spider Web) |

### Screen Flags (wPlayerScreens/wEnemyScreens)
| Bit | Flag | Description |
|-----|------|-------------|
| 0 | SCREENS_SPIKES | Spikes on field |
| 2 | SCREENS_SAFEGUARD | Safeguard active |
| 3 | SCREENS_LIGHT_SCREEN | Light Screen active |
| 4 | SCREENS_REFLECT | Reflect active |

### Weather (wBattleWeather)
| Value | Constant | Description |
|-------|----------|-------------|
| 0 | WEATHER_NONE | No weather |
| 1 | WEATHER_RAIN | Rain Dance |
| 2 | WEATHER_SUN | Sunny Day |
| 3 | WEATHER_SANDSTORM | Sandstorm |
| 4 | WEATHER_RAIN_END | Rain ending |
| 5 | WEATHER_SUN_END | Sun ending |
| 6 | WEATHER_SANDSTORM_END | Sandstorm ending |

### Move Struct (7 bytes per move)
| Offset | Field | Description |
|--------|-------|-------------|
| 0 | MOVE_ANIM | Animation ID |
| 1 | MOVE_EFFECT | Effect command script |
| 2 | MOVE_POWER | Base power |
| 3 | MOVE_TYPE | Type (high nybble) + physical/special |
| 4 | MOVE_ACC | Accuracy (0-255, where 255 = ~100%) |
| 5 | MOVE_PP | Base PP |
| 6 | MOVE_CHANCE | Secondary effect chance (0-255) |

---

## Engine Flags (engine_flags.asm)

### Pokegear Flags
- ENGINE_RADIO_CARD, ENGINE_MAP_CARD, ENGINE_PHONE_CARD, ENGINE_EXPN_CARD, ENGINE_POKEGEAR

### Day-Care
- ENGINE_DAY_CARE_MAN_HAS_EGG, ENGINE_DAY_CARE_MAN_HAS_MON, ENGINE_DAY_CARE_LADY_HAS_MON

### Mom
- ENGINE_MOM_SAVING_MONEY, ENGINE_MOM_ACTIVE

### Status Flags
- ENGINE_POKEDEX, ENGINE_UNOWN_DEX, ENGINE_CAUGHT_POKERUS
- ENGINE_ROCKET_SIGNAL_ON_CH20, ENGINE_CREDITS_SKIP

### Story Progress
- ENGINE_BUG_CONTEST_TIMER, ENGINE_ROCKETS_IN_RADIO_TOWER
- ENGINE_REACHED_GOLDENROD, ENGINE_ROCKETS_IN_MAHOGANY

### Badges (Johto)
ENGINE_ZEPHYRBADGE through ENGINE_RISINGBADGE (8 badges)

### Badges (Kanto)
ENGINE_BOULDERBADGE through ENGINE_EARTHBADGE (8 badges)

### Unown Unlocks
- ENGINE_UNLOCKED_UNOWNS_A_TO_K, L_TO_R, S_TO_W, X_TO_Z

### Fly Points
26 fly destinations: Players House, Pallet through Cinnabar, New Bark through Silver Cave

### Daily Events
- ENGINE_KURT_MAKING_BALLS, ENGINE_DAILY_BUG_CONTEST
- ENGINE_QWILFISH_SWARM, ENGINE_UNION_CAVE_LAPRAS
- ENGINE_MT_MOON_SQUARE_CLEFAIRY, ENGINE_BUENAS_PASSWORD
- ENGINE_DAILY_MOVE_TUTOR, ENGINE_DAISYS_GROOMING

### Rematch Flags
24 trainers with daily rematch flags (Joey, Wade, Ralph, etc.)

### Phone Item Flags
10 trainers who call with held items (Beverly's Nugget, etc.)

### Swarm Flags
- ENGINE_DUNSPARCE_SWARM, ENGINE_YANMA_SWARM

---

## Collision Constants (collision_constants.asm)

### Tile Permission Types
| Value | Constant | Description |
|-------|----------|-------------|
| $00 | LAND_TILE | Walkable land |
| $01 | WATER_TILE | Swimmable water |
| $0F | WALL_TILE | Impassable wall |
| $10 | TALK | Can talk across |

### Key Collision Tiles
- $07: COLL_WALL
- $12: COLL_CUT_TREE
- $14: COLL_LONG_GRASS
- $15: COLL_HEADBUTT_TREE
- $18: COLL_TALL_GRASS (wild encounters)
- $23: COLL_ICE (sliding)
- $24: COLL_WHIRLPOOL
- $29: COLL_WATER
- $33: COLL_WATERFALL
- $60: COLL_PIT (holes)
- $70: COLL_WARP_CARPET_DOWN
- $71: COLL_DOOR
- $72: COLL_LADDER
- $7A: COLL_STAIRCASE
- $7B: COLL_CAVE
- $7C: COLL_WARP_PANEL
- $90: COLL_COUNTER (shop counter)
- $91: COLL_BOOKSHELF
- $93: COLL_PC
- $94: COLL_RADIO
- $95: COLL_TOWN_MAP
- $96: COLL_MART_SHELF
- $97: COLL_TV
- $9D: COLL_WINDOW
- $A0-$A7: HOP tiles (ledges)
- $B0-$B7: Directional walls

---

## Map Constants (map_constants.asm)

### Map Groups
26 map groups total, organized geographically:
1. Olivine area (13 maps)
2. Mahogany area (11 maps)
3. General Johto landmarks
4. Ecruteak area
5. Blackthorn area
6. Cinnabar/Cerulean/Azalea areas
7. Lake of Rage area
8. Silver Cave area
9. Goldenrod area
10-26: Various indoor/dungeon groups

### Total Maps
~254 maps across all groups.

---

## Event Flags (event_flags.asm, ~1465 lines)

### Categories (1400+ flags)
- **Story events**: Got starter, gave Mystery Egg, cleared Radio Tower, beat Elite Four, etc.
- **TM/HM gifts**: 8 Johto gym TMs, 7 HMs, plus field TMs
- **Item gifts**: Key items, weekly sibling gifts (Frieda, Tuscany, etc.)
- **Hidden items**: ~80 hidden items across all maps
- **Trainer beaten flags**: One per beatable trainer
- **Map-specific flags**: Door unlocked, event completed, NPC moved, etc.

### Key Story Flags
- EVENT_GOT_A_POKEMON_FROM_ELM (game start)
- EVENT_CLEARED_SLOWPOKE_WELL (Badge 2 area)
- EVENT_HERDED_FARFETCHD (Ilex Forest)
- EVENT_FOUGHT_SUDOWOODO (Route 36)
- EVENT_JASMINE_RETURNED_TO_GYM (Badge 6)
- EVENT_CLEARED_ROCKET_HIDEOUT (Mahogany)
- EVENT_CLEARED_RADIO_TOWER (Goldenrod)
- EVENT_RELEASED_THE_BEASTS (Tin Tower)
- EVENT_BEAT_ELITE_FOUR (endgame)

---

## Music Constants (music_constants.asm)

103 music tracks (MUSIC_NONE through MUSIC_MOBILE_CENTER).
See maps/music_and_sound.md for full listing with locations.

### Special Music Values
- MUSIC_MAHOGANY_MART = MUSIC_SUICUNE_BATTLE ($64) -- overloaded
- RADIO_TOWER_MUSIC = $80 -- bit flag for Radio Tower
- RESTART_MAP_MUSIC = $FE
- ENTER_MAP_MUSIC = $FF

---

## SFX Constants (sfx_constants.asm)

~190 sound effects organized by category:
- UI sounds (menu, read text, save, etc.)
- Battle SFX (per-move sounds)
- Field SFX (jump, grass rustle, door, fly, etc.)
- Pokemon cries (separate system)

---

## Type Constants (type_constants.asm)

18 types in Crystal's type system:
- Physical: NORMAL, FIGHTING, FLYING, POISON, GROUND, ROCK, BUG, GHOST, STEEL
- Special: FIRE, WATER, GRASS, ELECTRIC, PSYCHIC, ICE, DRAGON, DARK
- CURSE_T (???) type for Curse move

---

## Sprite/Object Constants

### Movement Data Types
19 movement types for NPCs (see scripting_commands.md)

### Sprite Palette Constants
- PAL_NPC_RED, PAL_NPC_BLUE, PAL_NPC_GREEN, PAL_NPC_BROWN
- PAL_NPC_PINK, PAL_NPC_SILVER

### Object Limits
- NUM_OBJECT_STRUCTS = 16 (max simultaneous map objects including player)
- NUM_SPRITE_ANIM_STRUCTS = 10

---

## Pokemon Data Constants (pokemon_data_constants.asm)

### Party/Box Struct Sizes
- PARTYMON_STRUCT_LENGTH = 48 bytes
- BOXMON_STRUCT_LENGTH = 32 bytes
- PARTY_LENGTH = 6 (max party size)
- MONS_PER_BOX = 20 (max Pokemon per box)

### Caught Data Packing
- CAUGHT_TIME_MASK = %11000000 (2 bits for time of day)
- CAUGHT_LEVEL_MASK = %00111111 (6 bits for level, max 63)
- CAUGHT_GENDER_MASK = %10000000 (1 bit for OT gender)
- CAUGHT_LOCATION_MASK = %01111111 (7 bits for location, max 127)

### Growth Rates
6 experience growth rates: FAST, MEDIUM_FAST, MEDIUM_SLOW, SLOW, plus SLIGHTLY_FAST and SLIGHTLY_SLOW (unused in official data)

---

## Item Constants (item_constants.asm)

### Item Categories
- Regular items: MASTER_BALL through BERRY
- Key items: BICYCLE, CARD_KEY, BASEMENT_KEY, etc.
- Medicine: POTION, SUPER_POTION, HYPER_POTION, etc.
- Balls: POKE_BALL, GREAT_BALL, ULTRA_BALL, etc. (including Apricorn balls)
- TMs: TM01-TM50
- HMs: HM01-HM07
- Berries: BERRY, BITTER_BERRY, BURNT_BERRY, etc.

### Held Item Effects
Type-boost items, stat-boost items, status cure berries, etc.
Key: HELD_DRAGON_BOOST is on Dragon Scale (bug -- should be Dragon Fang)

---

## Trainer Constants (trainer_constants.asm)

67 trainer classes from FALKNER (#1) through MYSTICALMAN (#67).
Includes gym leaders, Elite Four, rival variants (RIVAL1, RIVAL2), and generic classes.

---

## Serial/Link Constants (serial_constants.asm)

### Link Modes
- LINK_TIMECAPSULE = 1
- LINK_COLOSSEUM = 2
- LINK_TRADECENTER = 3
- LINK_MOBILE = 4

---

## Hardware Constants (hardware.inc)

Game Boy hardware register addresses and bit definitions.
Key registers: rLCDC, rSTAT, rSCY, rSCX, rLY, rBGP, rOBP0, rOBP1, rWY, rWX, rVBK, rKEY1, rHDMA1-5, rRP, rBCPS, rOCPS, rWBK, rIE, rIF.
