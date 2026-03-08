# Pokemon Crystal -- Design Flaws

Source: pokecrystal docs/design_flaws.md (official documentation)

These are NOT bugs -- the code works correctly, but the design is unnecessarily complicated or fragile.

---

## 1. Pic Banks Offset by PICS_FIX

Pokemon/trainer pic pointers use `dba_pic` instead of standard `dba`. A macro offsets banks by PICS_FIX ($36), then `FixPicBank` translates back via a lookup table. This exists purely as a historical artifact -- the pics were originally in different banks and the indirection was never cleaned up.

**Impact**: Makes adding new Pokemon pics more complex than necessary.

---

## 2. PokemonPicPointers and UnownPicPointers Same Address Assumption

`GetFrontpicPointer` and `GetMonBackpic` assume both pointer tables start at the same ROM address but in different banks. This is enforced by `layout.link` with `org $4000`. Moving either table would break all sprite loading.

**Impact**: Data layout is constrained by code assumptions.

---

## 3. Footprints Split Into Top/Bottom Halves

Footprints are stored as 16x64-tile image with tops and bottoms interleaved in groups of 8. Loading requires two separate copy operations. Should be stored contiguously.

**Impact**: Unnecessary complexity in footprint loading code.

---

## 4. Music IDs $64 and $80+ Have Special Behavior

Music ID $64 (MUSIC_SUICUNE_BATTLE) is reused as MUSIC_MAHOGANY_MART, causing GetMapMusic to check story flags and play either MUSIC_ROCKET_HIDEOUT or MUSIC_CHERRYGROVE_CITY. IDs $80+ use RADIO_TOWER_MUSIC bit flag. This overloading means certain music IDs can't be used for normal map music.

**Impact**: Limited music assignment flexibility. Adding new songs near these IDs is dangerous.

---

## 5. ITEM_C3 and ITEM_DC Break TM Sequence

The 50 TMs should be a continuous sequence, but ITEM_C3 and ITEM_DC are gap items inserted to align catch rates with held items for Gen 1 trade compatibility. `GetTMHMNumber` and `GetNumberedTMHM` must compensate with skip logic.

**Historical reason**: Originally the TMs started at $C4, and these gaps corresponded to catch rates 200 (Abra) and 225 (Krabby/Horsea/Goldeen/Staryu). When TMs shifted down by 5, the gaps remained at wrong positions.

---

## 6. Pokedex Entry Banks Derived from Species IDs

Pokedex entry pointers use `dw` (2 bytes) instead of `dba` (3 bytes). The bank is derived from species ID using `rlca; rlca; maskbits NUM_DEX_ENTRY_BANKS`. This calculation is duplicated in THREE separate routines:
- `GetDexEntryPointer` (pokedex)
- `HeavyBall_GetDexEntryBank` (item effects)
- `PokedexShow_GetDexEntryBank` (radio)

**Impact**: Adding/reordering Pokemon requires careful attention to bank boundaries.

---

## 7. 6-Bit Caught Level (Max 63)

Caught data packs time-of-day (2 bits), level (6 bits), gender (1 bit), and location (7 bits) into 2 bytes. The 6-bit level field can only store 0-63. Pokemon caught at level 64+ overflow into the time-of-day bits.

**Impact**: This is why Lugia/Ho-Oh are level 60 in Crystal (down from 70 in Gold/Silver). Level 70 would show as level 6 caught "during the day" instead of "in the morning."

---

## 8. Sine Wave Code Duplicated 5 Times

Identical sine wave calculation (via `calc_sine_wave` macro) appears in:
1. `_Sine` (engine/math/sine.asm)
2. `Sprites_Sine` (engine/sprite_anims/core.asm)
3. `BattleAnim_Sine` (engine/battle_anims/functions.asm)
4. `StartTrainerBattle_DrawSineWave` (engine/battle/battle_transition.asm)
5. `CelebiEvent_Cosine` (engine/events/celebi.asm)

Each instance includes its own copy of the sine lookup table (32 entries * 2 bytes = 64 bytes per copy).

**Impact**: Wastes ~256 bytes of ROM. Should use single home bank routine.

---

## 9. GetForestTreeFrame Inefficiency

Returns 0 if input is even, 2 if odd. Uses a long chain of comparisons instead of a simple `and 1; add a`.

**Impact**: Wasteful but functionally correct.

---

## 10. Scripting Engine 127 Bank Limit

The overworld scripting engine uses 7-bit bank values, limiting scripts to banks 0-127. This constrains ROM layout for map scripts.

**Impact**: If the ROM grows beyond 2 MB of script data, bank management becomes problematic.
