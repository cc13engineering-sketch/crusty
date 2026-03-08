# Pokemon Crystal - Ruins of Alph Complete Guide

Source: pokecrystal disassembly (maps/RuinsOfAlph*.asm, data/pokemon/unown_words.asm, data/wild/unlocked_unowns.asm).

---

## Overview

The Ruins of Alph is a mysterious archaeological site south of Violet City and west of Route 32 in Johto. It contains:
- 4 Puzzle Chambers (each with a sliding puzzle)
- 1 Inner Chamber (main hall with statues)
- 4 Hidden Item Rooms (accessed via secret passages)
- 4 Word Rooms (Unown text chambers)
- 1 Research Center
- Unown wild encounters (only available after solving puzzles)

---

## Puzzle Chambers

Each chamber has a sliding tile puzzle depicting a prehistoric/legendary Pokemon. Solving a puzzle unlocks a set of Unown letters and drops you into the Inner Chamber.

### 1. Kabuto Chamber
**Location:** Northeast corner of ruins (accessible from Inner Chamber north entrance)
**Puzzle:** Rearrange tiles to form Kabuto
**Description:** "A Pokemon that hid on the sea floor. Eyes on its back scanned the area."
**Wall Hint:** ESCAPE (written in Unown text)
**Secret:** Use an Escape Rope while in the chamber to open the hidden wall
**Unlocks:** Unown A through K (ENGINE_UNLOCKED_UNOWNS_A_TO_K)
**Hidden Items (Item Room):** Berry, PsnCureBerry, Heal Powder, EnergyPowder

### 2. Omanyte Chamber
**Location:** Southwest corner (accessible from Inner Chamber west entrance)
**Puzzle:** Rearrange tiles to form Omanyte
**Wall Hint:** WATER (written in Unown text)
**Secret:** Have a Water Stone in your bag while in the chamber
**Unlocks:** Unown L through R (ENGINE_UNLOCKED_UNOWNS_L_TO_R)
**Hidden Items (Item Room):** MysteryBerry, Mystic Water, Stardust, Star Piece

### 3. Aerodactyl Chamber
**Location:** Southeast corner (accessible from Inner Chamber east entrance)
**Puzzle:** Rearrange tiles to form Aerodactyl
**Wall Hint:** LIGHT (written in Unown text)
**Secret:** Use Flash while in the chamber
**Unlocks:** Unown S through W (ENGINE_UNLOCKED_UNOWNS_S_TO_W)
**Hidden Items (Item Room):** Gold Berry, Moon Stone, Heal Powder, Energy Root

### 4. Ho-Oh Chamber
**Location:** Northwest corner (accessible from Inner Chamber south entrance)
**Puzzle:** Rearrange tiles to form Ho-Oh
**Wall Hint:** HO OH (written in Unown text)
**Secret:** Have Ho-Oh in your party while in the chamber
**Unlocks:** Unown X through Z (ENGINE_UNLOCKED_UNOWNS_X_TO_Z)
**Hidden Items (Item Room):** Gold Berry, MysteryBerry, Revival Herb, Charcoal

---

## Unlockable Unown Letters

Unown forms are unlocked in 4 sets, one per puzzle:

| Puzzle Solved | Unown Letters Unlocked |
|--------------|----------------------|
| Kabuto | A, B, C, D, E, F, G, H, I, J, K (11 letters) |
| Omanyte | L, M, N, O, P, Q, R (7 letters) |
| Aerodactyl | S, T, U, V, W (5 letters) |
| Ho-Oh | X, Y, Z (3 letters) |

**Total:** 26 letters (A-Z)

After solving a puzzle, wild Unown matching those letters appear in the Inner Chamber (main hall with ruins). Walk around in the main chamber to encounter them.

---

## Unown Pokedex Words

Each Unown letter has an associated word, visible in the Unown Pokedex mode (accessed from the Pokedex after catching at least 3 different Unown forms):

| Letter | Word |
|--------|------|
| A | ANGRY |
| B | BEAR |
| C | CHASE |
| D | DIRECT |
| E | ENGAGE |
| F | FIND |
| G | GIVE |
| H | HELP |
| I | INCREASE |
| J | JOIN |
| K | KEEP |
| L | LAUGH |
| M | MAKE |
| N | NUZZLE |
| O | OBSERVE |
| P | PERFORM |
| Q | QUICKEN |
| R | REASSURE |
| S | SEARCH |
| T | TELL |
| U | UNDO |
| V | VANISH |
| W | WANT |
| X | XXXXX |
| Y | YIELD |
| Z | ZOOM |

**Note:** X's word is "XXXXX" — a placeholder since there's no good X word that matches the theme of verbs/actions.

---

## Unown Stats and Mechanics

**Type:** Psychic
**Base Stats:** 48 HP / 72 Atk / 48 Def / 48 Spd / 72 SpAtk / 48 SpDef (BST: 336)
**Only Move:** Hidden Power (type and power vary by DVs)
**Catch Rate:** 225 (fairly easy)
**Wild Levels:** 5 (inner chamber encounters)

**Determining Unown's Letter:**
The Unown letter is determined by the Pokemon's DVs (IVs in Gen 2):
- Take the middle 2 bits of each DV: Atk, Def, Speed, Special
- Combine into an 8-bit value
- Divide by 10 to get the letter (0=A, 1=B, ..., 25=Z)
- This means different Unown letters have different Hidden Power types/powers

---

## Inner Chamber

The main hall of the ruins. Contains statues of ancient Pokemon and serves as the hub connecting all 4 puzzle chambers.

**Warps to:**
- Ho-Oh Chamber (southwest)
- Kabuto Chamber (northeast)
- Omanyte Chamber (southwest lower)
- Aerodactyl Chamber (southeast lower)
- Ruins of Alph Outside (south exit)

After solving any puzzle, you fall through the floor into the Inner Chamber and a "strange presence" message appears. Wild Unown begin appearing.

---

## Research Center

**Location:** South entrance of Ruins of Alph
**NPCs:**
- Scientists studying the ruins
- After catching 3+ Unown forms, a scientist gives you the Unown Pokedex upgrade
- After catching all 26 forms, the lead scientist congratulates you and gives you a print of the Unown alphabet

---

## Radio Signal

When inside the Ruins of Alph, tuning the radio to a specific frequency reveals a mysterious signal — the "???" channel plays eerie sounds that are unique to the ruins. This has no gameplay effect but adds atmosphere.

---

## Hidden Messages and Lore

The wall patterns in each chamber spell out messages in Unown text:

- **Kabuto Chamber:** "ESCAPE" — hints at using an Escape Rope to open the secret wall
- **Omanyte Chamber:** "WATER" — hints at needing a Water Stone
- **Aerodactyl Chamber:** "LIGHT" — hints at using Flash (HM05)
- **Ho-Oh Chamber:** "HO OH" — hints at bringing Ho-Oh in your party

The Ruins of Alph represent an ancient civilization that worshipped Unown as symbols of language and communication. The puzzle chambers each depict a Pokemon believed to be revered by the ancients — Kabuto, Omanyte, and Aerodactyl (fossil Pokemon) and Ho-Oh (legendary bird).

---

## Item Summary

All items obtainable in the Ruins of Alph:

**Kabuto Item Room:** Berry, PsnCureBerry, Heal Powder, EnergyPowder
**Ho-Oh Item Room:** Gold Berry, MysteryBerry, Revival Herb, Charcoal
**Omanyte Item Room:** MysteryBerry, Mystic Water, Stardust, Star Piece
**Aerodactyl Item Room:** Gold Berry, Moon Stone, Heal Powder, Energy Root

**Total unique items:** Berry, PsnCureBerry, Heal Powder (x2), EnergyPowder, Gold Berry (x2), MysteryBerry (x2), Revival Herb, Charcoal, Mystic Water, Stardust, Star Piece, Moon Stone, Energy Root

---

## Walkthrough Tips

1. Visit the Kabuto Chamber first (accessible earliest, northeast entrance from Route 32 or Violet City)
2. Solve the sliding puzzle to unlock Unown A-K
3. Catch 3 different Unown to get the Pokedex upgrade from the Research Center
4. Return later with Escape Rope, Water Stone, Flash, and Ho-Oh to unlock the hidden walls
5. Each hidden wall leads to an Item Room, then to a Word Room
6. Complete all 4 puzzles to unlock all 26 Unown forms
7. The Unown Pokedex is a collector's side quest — no major reward beyond completion
