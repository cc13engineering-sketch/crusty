# Gimli Pokemon Accuracy Review — Sprint 4 (Route 30 + Mr. Pokemon's House)

**Verdict: ACCEPT**

Every data point in the Sprint 4 Revised Plan has been verified against the pokecrystal-master source files. There are zero data errors. The plan is accurate and ready for implementation.

---

## Verification Summary

### Species Base Stats (7 species) — ALL CORRECT

| Species | Plan HP/Atk/Def/Spd/SpA/SpD | Pokecrystal Source | Match |
|---------|------|------|:---:|
| Caterpie (10) | 45/30/35/45/20/20 | 45/30/35/45/20/20 (base_stats/caterpie.asm) | YES |
| Metapod (11) | 50/20/55/30/25/25 | 50/20/55/30/25/25 (base_stats/metapod.asm) | YES |
| Weedle (13) | 40/35/30/50/20/20 | 40/35/30/50/20/20 (base_stats/weedle.asm) | YES |
| Zubat (41) | 40/45/35/55/30/40 | 40/45/35/55/30/40 (base_stats/zubat.asm) | YES |
| Poliwag (60) | 40/50/40/90/40/40 | 40/50/40/90/40/40 (base_stats/poliwag.asm) | YES |
| Ledyba (165) | 40/20/30/55/40/80 | 40/20/30/55/40/80 (base_stats/ledyba.asm) | YES |
| Spinarak (167) | 40/60/40/30/40/40 | 40/60/40/30/40/40 (base_stats/spinarak.asm) | YES |

### Species Types — ALL CORRECT

| Species | Plan Types | Pokecrystal | Match |
|---------|------|------|:---:|
| Caterpie | Bug/Bug | BUG/BUG | YES |
| Metapod | Bug/Bug | BUG/BUG | YES |
| Weedle | Bug/Poison | BUG/POISON | YES |
| Zubat | Poison/Flying | POISON/FLYING | YES |
| Poliwag | Water/Water | WATER/WATER | YES |
| Ledyba | Bug/Flying | BUG/FLYING | YES |
| Spinarak | Bug/Poison | BUG/POISON | YES |

### Growth Rates — ALL CORRECT

Caterpie=MediumFast, Metapod=MediumFast, Weedle=MediumFast, Zubat=MediumFast, Poliwag=MediumSlow, Ledyba=Fast, Spinarak=Fast. All match pokecrystal.

### Catch Rates — ALL CORRECT

Caterpie=255, Metapod=120, Weedle=255, Zubat=255, Poliwag=255, Ledyba=255, Spinarak=255.

### Base Exp — ALL CORRECT

Caterpie=53, Metapod=72, Weedle=52, Zubat=54, Poliwag=77, Ledyba=54, Spinarak=54.

### Move IDs (7 moves) — ALL CORRECT

| Move | Plan ID | Pokecrystal (hex -> dec) | Match |
|------|---------|------|:---:|
| STRING_SHOT | 81 | 0x51 = 81 | YES |
| POISON_STING | 40 | 0x28 = 40 | YES |
| HARDEN | 106 | 0x6a = 106 | YES |
| LEECH_LIFE | 141 | 0x8d = 141 | YES |
| CONSTRICT | 132 | 0x84 = 132 | YES |
| BUBBLE | 145 | 0x91 = 145 | YES |
| SUPERSONIC | 48 | 0x30 = 48 | YES |

### Move Data (7 moves) — ALL CORRECT

| Move | Plan Type/Pow/Acc/PP | Pokecrystal (moves.asm) | Match |
|------|------|------|:---:|
| STRING_SHOT | Bug/0/95/40 | BUG/0/95%/40 | YES |
| POISON_STING | Poison/15/100/35 | POISON/15/100%/35 | YES |
| HARDEN | Normal/0/100/30 | NORMAL/0/100%/30 | YES |
| LEECH_LIFE | Bug/20/100/15 | BUG/20/100%/15 | YES |
| CONSTRICT | Normal/10/100/35 | NORMAL/10/100%/35 | YES |
| BUBBLE | Water/20/100/30 | WATER/20/100%/30 | YES |
| SUPERSONIC | Normal/0/55/20 | NORMAL/0/55%/20 | YES |

### Learnsets — ALL CORRECT

| Species | Plan | Pokecrystal (evos_attacks.asm) | Match |
|---------|------|------|:---:|
| Caterpie | Tackle(1), StringShot(1) | TACKLE(1), STRING_SHOT(1) | YES |
| Metapod | Harden(1) | HARDEN(1) | YES |
| Weedle | PoisonSting(1), StringShot(1) | POISON_STING(1), STRING_SHOT(1) | YES |
| Zubat | LeechLife(1), Supersonic(6) | LEECH_LIFE(1), SUPERSONIC(6) | YES |
| Poliwag | Bubble(1), Hypnosis(7) | BUBBLE(1), HYPNOSIS(7) | YES |
| Ledyba | Tackle(1), Supersonic(8) | TACKLE(1), SUPERSONIC(8) | YES |
| Spinarak | PoisonSting(1), StringShot(1), ScaryFace(6), Constrict(11) | POISON_STING(1), STRING_SHOT(1), SCARY_FACE(6), CONSTRICT(11) | YES |

### Map Dimensions — ALL CORRECT

| Map | Plan (tiles) | Pokecrystal (blocks -> tiles) | Match |
|-----|------|------|:---:|
| Route 30 | 20x54 | 10x27 = 20x54 (map_constants.asm:491) | YES |
| Route30BerryHouse | 8x8 | 4x4 = 8x8 (map_constants.asm:499) | YES |
| MrPokemonsHouse | 8x8 | 4x4 = 8x8 (map_constants.asm:500) | YES |

### Route 30 Wild Encounters (21 slots) — ALL CORRECT

Verified all 7 slots x 3 time periods against johto_grass.asm lines 1265-1291. Every species/level combination matches exactly. Encounter rate = 10 percent matches.

### Route 30 Warps — CORRECT

| Plan | Pokecrystal (Route30.asm) | Match |
|------|------|:---:|
| (7,39) -> Route30BerryHouse | warp_event 7, 39, ROUTE_30_BERRY_HOUSE, 1 | YES |
| (17,5) -> MrPokemonsHouse | warp_event 17, 5, MR_POKEMONS_HOUSE, 1 | YES |

Berry House warps back: (2,7) and (3,7) -> ROUTE_30 warp 1. Correct.
Mr Pokemon's House warps back: (2,7) and (3,7) -> ROUTE_30 warp 2. Correct.

### Route 30 NPC Count — CORRECT

Plan: 11 NPCs. Pokecrystal: 11 object_events. Correct.

### Route 30 bg_events Count — CORRECT

Plan: 5 bg_events. Pokecrystal: 5 (at positions 9/43, 13/29, 15/5, 3/21, 14/9). Correct.

### Trainer Parties — ALL CORRECT

| Trainer | Plan | Pokecrystal (parties.asm) | Match |
|---------|------|------|:---:|
| Youngster Joey (1) | Rattata Lv4 | RATTATA/4, TRAINERTYPE_NORMAL | YES |
| Youngster Mikey (2) | Pidgey Lv2 + Rattata Lv4 | PIDGEY/2 + RATTATA/4, TRAINERTYPE_NORMAL | YES |
| Bug Catcher Don (1) | Caterpie Lv3 x2 | CATERPIE/3 + CATERPIE/3, TRAINERTYPE_NORMAL | YES |

### Trainer Sight Ranges — ALL CORRECT

| Trainer | Plan Range | Pokecrystal (Route30.asm OBJECTTYPE_TRAINER field) | Match |
|---------|------|------|:---:|
| Joey | 3 | 3 | YES |
| Mikey | 1 | 1 | YES |
| Don | 3 | 3 | YES |

### Mr. Pokemon's House NPCs — CORRECT

| NPC | Plan | Pokecrystal (MrPokemonsHouse.asm) | Match |
|-----|------|------|:---:|
| Mr. Pokemon | GENTLEMAN at (3,5) StandingRight | SPRITE_GENTLEMAN, 3, 5, STANDING_RIGHT | YES |
| Oak | OAK at (6,5) StandingUp | SPRITE_OAK, 6, 5, STANDING_UP | YES |

Oak has event_flag EVENT_MR_POKEMONS_HOUSE_OAK and disappears when flag is set. Correct.

### Berry House NPC — CORRECT

POKEFAN_M at (2,3) StandingDown. Matches pokecrystal Route30BerryHouse.asm.

### Mr. Pokemon Scene Script — CORRECT

Key elements verified against MrPokemonsHouse.asm:
- Scene 0 is SCENE_MRPOKEMONSHOUSE_MEET_MR_POKEMON (entry triggers cutscene)
- Scene 1 is SCENE_MRPOKEMONSHOUSE_NOOP (post-cutscene, no auto-trigger)
- Mr. Pokemon gives MYSTERY_EGG
- Oak walks in, gives Pokedex (ENGINE_POKEDEX flag)
- Party healed
- Rival pokeball flag logic matches pokecrystal (Totodile->Chikorita, Chikorita->Cyndaquil, else->Totodile)
- Oak disappears (EVENT_MR_POKEMONS_HOUSE_OAK set)
- Scene chains set for CherrygroveCity (rival ambush) and ElmsLab (meet officer)

---

## Minor Notes (Not Errors)

1. **M8 description wording**: The plan's M8 says it sets "the REMAINING ball flag (the one the player didn't pick and the rival didn't steal)". The actual logic sets the flag for the Pokemon the RIVAL stole (making that pokeball empty on Elm's table). The flag-setting code is correct; only the English description is slightly misleading. Not a code issue.

2. **Metapod learnset**: Pokecrystal lists HARDEN at both level 1 and level 7. The plan correctly deduplicates to just (1, MOVE_HARDEN) since level 7 Harden is redundant. This is fine.

---

## Final Verdict

**ACCEPT** — Zero data errors found. All species stats, types, growth rates, catch rates, base exp, move IDs, move data, learnsets, map dimensions, encounter tables, warp coordinates, NPC positions, trainer parties, trainer sight ranges, and scene script logic have been verified against the pokecrystal-master assembly source files.

The plan is ready for parallel implementation by Mary/Pippin/Sam.
