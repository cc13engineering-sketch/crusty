# Sprint 6 Implementation Plan: Species Database Expansion (1-251)

## Overview

Expand `data.rs` from 17 species to all 251. Split species data into a new dedicated `species_data.rs` file using a const-array dispatch pattern. All downstream code is unaffected because `data.rs` re-exports everything.

**Canonical data source:** `crates/engine-core/src/pokemon/pokecrystal-master/data/pokemon/`

---

## Current State

- `data.rs` is ~910 lines, currently has 17 `SpeciesData` statics and a `match`-based `species_data()` dispatch
- All SpeciesId constants live at the bottom of `data.rs` (lines 794-863)
- All MoveId constants live at the bottom of `data.rs` (lines 799-843)
- `species_data()` at line 507 returns `&'static SpeciesData` by matching on SpeciesId

---

## Phase 1: Create `species_data.rs` Skeleton

**File to create:** `crates/engine-core/src/pokemonv2/species_data.rs`

**File to edit:** `crates/engine-core/src/pokemonv2/data.rs`
**File to edit:** `crates/engine-core/src/pokemonv2/mod.rs` (or wherever the module tree lives)

### 1a. Wire the module

Find where `mod.rs` declares submodules for pokemonv2. Add `pub mod species_data;` **before** the `data` module so it compiles standalone (species_data imports from data, not vice versa).

Check the module tree file:
```
crates/engine-core/src/pokemonv2/mod.rs
```

Add to mod.rs (in the correct position, before any module that imports data):
```rust
pub mod species_data;
```

### 1b. Add re-export to data.rs

At the **top** of the species-related section in `data.rs` (around line 219, just before the existing `SpeciesData` struct), add:
```rust
pub use crate::pokemonv2::species_data::*;
```

Wait -- `data.rs` cannot `use` `species_data` if `species_data` imports from `data`. This would be circular. The correct approach:

- `species_data.rs` imports types FROM `data.rs` (PokemonType, GrowthRate, SpeciesData, SpeciesId, MoveId, move constants)
- `data.rs` does NOT import from `species_data.rs` -- instead, the re-export happens in `mod.rs`

So in `mod.rs`:
```rust
pub mod data;
pub mod species_data;
pub use species_data::{species_data, SpeciesId, NUM_SPECIES};
// ... or just let callers use `pokemonv2::species_data::species_data()`
```

Check whether existing callers use `data::species_data()` or `pokemonv2::species_data()`. The existing function is `pub fn species_data(id: SpeciesId) -> &'static SpeciesData` in `data.rs`. To avoid breaking callers, keep the function signature identical but move the body to `species_data.rs`, then re-export from `data.rs` via `pub use`.

**Cleanest approach (no circular imports):**

In `data.rs`:
```rust
// Remove the old species_data() function body and all SpeciesId/learnset/SpeciesData statics
// Add at the end of data.rs, below SpeciesData struct:
// pub use crate::pokemonv2::species_data::{species_data, NUM_SPECIES};
// (and all SpeciesId consts)
```

In `mod.rs`:
```rust
pub mod data;
pub mod species_data;  // declared here, not in data.rs
```

In `species_data.rs`:
```rust
use super::data::{SpeciesData, SpeciesId, MoveId, PokemonType, GrowthRate};
use super::data::{MOVE_TACKLE, MOVE_GROWL, /* etc */};
```

### 1c. Skeleton structure of species_data.rs

```rust
// AI-INSTRUCTIONS: pokemonv2/species_data.rs — All 251 species static data.
// Imports types from data.rs. Exported back via data.rs pub use.
// Source: pokecrystal-master/data/pokemon/base_stats/*.asm (canonical)
//         pokecrystal-master/data/pokemon/evos_attacks.asm (learnsets)

use super::data::{GrowthRate, MoveId, PokemonType, SpeciesData, SpeciesId};
use super::data::{
    MOVE_BUBBLE, MOVE_CONSTRICT, MOVE_DEFENSE_CURL, MOVE_GROWL,
    MOVE_GROWTH, MOVE_HARDEN, MOVE_HYPNOSIS, MOVE_LEECH_LIFE,
    MOVE_LEER, MOVE_LICK, MOVE_POISON_STING, MOVE_SAND_ATTACK,
    MOVE_SCRATCH, MOVE_SPLASH, MOVE_STRING_SHOT, MOVE_STRUGGLE,
    MOVE_SUPERSONIC, MOVE_SYNTHESIS, MOVE_TACKLE, MOVE_TAIL_WHIP,
    MOVE_VINE_WHIP,
};

pub const NUM_SPECIES: usize = 251;

// ============================================================
// SpeciesId constants — ALL 251 national dex numbers
// ============================================================
pub const BULBASAUR: SpeciesId = 1;
pub const IVYSAUR: SpeciesId = 2;
// ... through CELEBI: SpeciesId = 251

// ============================================================
// Learnsets — static slices of (level, MoveId)
// ============================================================
static BULBASAUR_LEARNSET: &[(u8, MoveId)] = &[
    (1, MOVE_TACKLE), (4, MOVE_GROWL), /* etc from evos_attacks.asm */
];
// ... 250 more

// ============================================================
// SPECIES_TABLE — master array indexed by (dex_number - 1)
// ============================================================
static SPECIES_TABLE: [SpeciesData; NUM_SPECIES] = [
    // #001 BULBASAUR
    SpeciesData {
        id: 1, name: "BULBASAUR",
        type1: PokemonType::Grass, type2: PokemonType::Poison,
        base_hp: 45, base_attack: 49, base_defense: 49,
        base_speed: 45, base_sp_attack: 65, base_sp_defense: 65,
        catch_rate: 45, base_exp: 64,
        growth_rate: GrowthRate::MediumSlow,
        learnset: BULBASAUR_LEARNSET,
    },
    // ... #002 through #251
];

pub fn species_data(id: SpeciesId) -> &'static SpeciesData {
    let idx = id as usize;
    if idx >= 1 && idx <= NUM_SPECIES {
        &SPECIES_TABLE[idx - 1]
    } else {
        &SPECIES_TABLE[0] // fallback: Bulbasaur
    }
}
```

---

## Phase 2: Populate All 251 Species

**Source files to read for each species:**
- Base stats: `crates/engine-core/src/pokemon/pokecrystal-master/data/pokemon/base_stats/<name>.asm`
- Species order: `crates/engine-core/src/pokemon/pokecrystal-master/data/pokemon/base_stats.asm` (line 23-273)

### Species order (from base_stats.asm includes, position = dex number)

```
001 bulbasaur     002 ivysaur       003 venusaur      004 charmander
005 charmeleon    006 charizard     007 squirtle       008 wartortle
009 blastoise     010 caterpie      011 metapod        012 butterfree
013 weedle        014 kakuna        015 beedrill       016 pidgey
017 pidgeotto     018 pidgeot       019 rattata        020 raticate
021 spearow       022 fearow        023 ekans          024 arbok
025 pikachu       026 raichu        027 sandshrew      028 sandslash
029 nidoran_f     030 nidorina      031 nidoqueen      032 nidoran_m
033 nidorino      034 nidoking      035 clefairy       036 clefable
037 vulpix        038 ninetales     039 jigglypuff     040 wigglytuff
041 zubat         042 golbat        043 oddish         044 gloom
045 vileplume     046 paras         047 parasect       048 venonat
049 venomoth      050 diglett       051 dugtrio        052 meowth
053 persian       054 psyduck       055 golduck        056 mankey
057 primeape      058 growlithe     059 arcanine       060 poliwag
061 poliwhirl     062 poliwrath     063 abra           064 kadabra
065 alakazam      066 machop        067 machoke        068 machamp
069 bellsprout    070 weepinbell    071 victreebel     072 tentacool
073 tentacruel    074 geodude       075 graveler       076 golem
077 ponyta        078 rapidash      079 slowpoke       080 slowbro
081 magnemite     082 magneton      083 farfetch_d     084 doduo
085 dodrio        086 seel          087 dewgong        088 grimer
089 muk           090 shellder      091 cloyster       092 gastly
093 haunter       094 gengar        095 onix           096 drowzee
097 hypno         098 krabby        099 kingler        100 voltorb
101 electrode     102 exeggcute     103 exeggutor      104 cubone
105 marowak       106 hitmonlee     107 hitmonchan     108 lickitung
109 koffing       110 weezing       111 rhyhorn        112 rhydon
113 chansey       114 tangela       115 kangaskhan     116 horsea
117 seadra        118 goldeen       119 seaking        120 staryu
121 starmie       122 mr__mime      123 scyther        124 jynx
125 electabuzz    126 magmar        127 pinsir         128 tauros
129 magikarp      130 gyarados      131 lapras         132 ditto
133 eevee         134 vaporeon      135 jolteon        136 flareon
137 porygon       138 omanyte       139 omastar        140 kabuto
141 kabutops      142 aerodactyl   143 snorlax        144 articuno
145 zapdos        146 moltres       147 dratini        148 dragonair
149 dragonite     150 mewtwo        151 mew            152 chikorita
153 bayleef       154 meganium      155 cyndaquil      156 quilava
157 typhlosion    158 totodile      159 croconaw       160 feraligatr
161 sentret       162 furret        163 hoothoot       164 noctowl
165 ledyba        166 ledian        167 spinarak       168 ariados
169 crobat        170 chinchou      171 lanturn        172 pichu
173 cleffa        174 igglybuff     175 togepi         176 togetic
177 natu          178 xatu          179 mareep         180 flaaffy
181 ampharos      182 bellossom     183 marill         184 azumarill
185 sudowoodo     186 politoed      187 hoppip         188 skiploom
189 jumpluff      190 aipom         191 sunkern        192 sunflora
193 yanma         194 wooper        195 quagsire       196 espeon
197 umbreon       198 murkrow       199 slowking       200 misdreavus
201 unown         202 wobbuffet     203 girafarig      204 pineco
205 forretress    206 dunsparce     207 gligar         208 steelix
209 snubbull      210 granbull      211 qwilfish       212 scizor
213 shuckle       214 heracross     215 sneasel        216 teddiursa
217 ursaring      218 slugma        219 magcargo       220 swinub
221 piloswine     222 corsola       223 remoraid       224 octillery
225 delibird      226 mantine       227 skarmory       228 houndour
229 houndoom      230 kingdra       231 phanpy         232 donphan
233 porygon2      234 stantler      235 smeargle       236 tyrogue
237 hitmontop     238 smoochum      239 elekid         240 magby
241 miltank       242 blissey       243 raikou         244 entei
245 suicune       246 larvitar      247 pupitar        248 tyranitar
249 lugia         250 ho_oh         251 celebi
```

### ASM field mapping

Each `base_stats/<name>.asm` file has this layout:
```asm
db SPECIES_NAME  ; first line (name constant, ignore — use filename)
db hp, atk, def, spd, sat, sdf  ; 6 base stats
db TYPE1, TYPE2   ; types
db catch_rate
db base_exp
db ITEM1, ITEM2   ; held items (ignore for our struct)
db GENDER_RATIO   ; (ignore)
db 100            ; unknown (ignore)
db hatch_cycles   ; (ignore)
db 5              ; unknown (ignore)
INCBIN ...        ; sprite dimensions (ignore)
dw NULL, NULL     ; unused pointers (ignore)
db GROWTH_RATE    ; growth rate
dn EGG_GROUP1, EGG_GROUP2  ; (ignore)
; tmhm ...        ; (ignore)
```

Fields to extract: `hp, atk, def, spd, sat, sdf, TYPE1, TYPE2, catch_rate, base_exp, GROWTH_RATE`

### Type constant mapping

| ASM constant | Rust variant |
|---|---|
| `NORMAL` | `PokemonType::Normal` |
| `FIRE` | `PokemonType::Fire` |
| `WATER` | `PokemonType::Water` |
| `ELECTRIC` | `PokemonType::Electric` |
| `GRASS` | `PokemonType::Grass` |
| `ICE` | `PokemonType::Ice` |
| `FIGHTING` | `PokemonType::Fighting` |
| `POISON` | `PokemonType::Poison` |
| `GROUND` | `PokemonType::Ground` |
| `FLYING` | `PokemonType::Flying` |
| `PSYCHIC_TYPE` | `PokemonType::Psychic` |
| `BUG` | `PokemonType::Bug` |
| `ROCK` | `PokemonType::Rock` |
| `GHOST` | `PokemonType::Ghost` |
| `DRAGON` | `PokemonType::Dragon` |
| `DARK` | `PokemonType::Dark` |
| `STEEL` | `PokemonType::Steel` |

Note: In pokecrystal ASM, `PSYCHIC_TYPE` is used (not `PSYCHIC`) to avoid conflict with the Psychic move name.

### Growth rate constant mapping

| ASM constant | Rust variant |
|---|---|
| `GROWTH_MEDIUM_FAST` | `GrowthRate::MediumFast` |
| `GROWTH_MEDIUM_SLOW` | `GrowthRate::MediumSlow` |
| `GROWTH_FAST` | `GrowthRate::Fast` |
| `GROWTH_SLOW` | `GrowthRate::Slow` |

### Mono-type rule

When `TYPE1 == TYPE2` in the ASM (e.g., `db NORMAL, NORMAL`), set both `type1` and `type2` to the same variant. This is correct Gen 2 behavior.

### Name convention

Use the uppercase ASM constant name as the `name` field string. Examples:
- bulbasaur.asm → `"BULBASAUR"`
- nidoran_f.asm → `"NIDORAN_F"` (with underscore, matches pokecrystal)
- farfetch_d.asm → `"FARFETCH_D"`
- mr__mime.asm → `"MR._MIME"` (two underscores in filename = period+space in display name — use `"MR._MIME"`)
- ho_oh.asm → `"HO-OH"` (underscore = hyphen in display)

---

## Phase 3: Learnsets

**Source file:** `crates/engine-core/src/pokemon/pokecrystal-master/data/pokemon/evos_attacks.asm`

### Reading the learnset format

For each species, find the `<Species>EvosAttacks:` label. Skip all `db EVOLVE_*` lines and the `db 0` terminator. Read lines of the form `db level, MOVE_NAME` until `db 0`.

Example from `BulbasaurEvosAttacks:`:
```asm
db EVOLVE_LEVEL, 16, IVYSAUR
db 0                          ; <- end evos
db 1, TACKLE                  ; <- learnset starts
db 4, GROWL
db 7, LEECH_SEED
db 10, VINE_WHIP
db 15, POISONPOWDER
db 15, SLEEP_POWDER
db 20, RAZOR_LEAF
db 25, SWEET_SCENT
db 32, GROWTH
db 39, SYNTHESIS
db 46, SOLARBEAM
db 0                          ; <- learnset ends
```

Maps to:
```rust
static BULBASAUR_LEARNSET: &[(u8, MoveId)] = &[
    (1, MOVE_TACKLE), (4, MOVE_GROWL), (7, MOVE_LEECH_SEED),
    (10, MOVE_VINE_WHIP), (15, MOVE_POISONPOWDER), (15, MOVE_SLEEP_POWDER),
    (20, MOVE_RAZOR_LEAF), (25, MOVE_SWEET_SCENT), (32, MOVE_GROWTH),
    (39, MOVE_SYNTHESIS), (46, MOVE_SOLARBEAM),
];
```

### Move constants

For any move that does NOT yet exist as a `MOVE_*` constant in `data.rs`, add the constant there. Do NOT put new move constants in `species_data.rs` — keep all `MoveId` constants in `data.rs`.

Sprint 6 only needs learnset moves that exist in the level-up data, not all 250 moves. For moves not yet defined, create a stub constant:

```rust
// In data.rs — new MoveId constants needed for learnsets
pub const MOVE_LEECH_SEED: MoveId = 73;
pub const MOVE_POISONPOWDER: MoveId = 77;
pub const MOVE_SLEEP_POWDER: MoveId = 79;
pub const MOVE_RAZOR_LEAF: MoveId = 75;
pub const MOVE_SWEET_SCENT: MoveId = 230;
pub const MOVE_SOLARBEAM: MoveId = 76;
// ... etc
```

For any move constant that is needed in a learnset but has no MoveData yet defined, add a stub to `move_data()` returning `TACKLE_DATA` as fallback (same pattern as the existing `_ => &TACKLE_DATA`). The match already has a wildcard fallback so new constants don't break anything — just add the constant value.

### Learnset stub policy

If a species has NO level-up moves in `evos_attacks.asm` (rare — check Ditto, Unown), use:
```rust
static DITTO_LEARNSET: &[(u8, MoveId)] = &[(1, MOVE_TRANSFORM)];
```
Every learnset must be non-empty. The test will fail on empty learnsets.

---

## Phase 4: Move SpeciesId Constants and Wire Up

### 4a. Move SpeciesId constants

In `data.rs`, the SpeciesId constants are at lines 794-863. Move ALL of them into `species_data.rs` (they are part of the species database). This consolidates all 251 constants into one place.

After moving, add the full list 1-251 in order:
```rust
// In species_data.rs
pub const BULBASAUR: SpeciesId = 1;
pub const IVYSAUR: SpeciesId = 2;
pub const VENUSAUR: SpeciesId = 3;
// ... all existing ones + new ones ...
pub const CHIKORITA: SpeciesId = 152;  // already existed in data.rs
pub const CELEBI: SpeciesId = 251;
```

### 4b. Remove from data.rs

From `data.rs`, remove:
- All `static *_LEARNSET` arrays (lines 240-489)
- All `static *_DATA: SpeciesData` statics (lines 255-504)
- The `species_data()` function (lines 507-530)
- All `SpeciesId` constants at the bottom (lines 794-863)

Do NOT remove:
- The `SpeciesData` struct definition
- The `SpeciesId` type alias (`pub type SpeciesId = u16;`)
- Any `MoveId` constants
- Any `MoveData` statics or `move_data()` function

### 4c. Re-export from data.rs

Add near the bottom of `data.rs` (after existing content, before tests):
```rust
// Re-export species data from species_data module
pub use crate::pokemonv2::species_data::{
    species_data, NUM_SPECIES,
    BULBASAUR, IVYSAUR, // ... or just wildcard:
};
// OR just let callers import from species_data directly via mod.rs
```

The cleanest approach: do NOT re-export in `data.rs`. Instead, ensure `mod.rs` makes both modules public so callers can do `use pokemonv2::species_data::CHIKORITA` or simply `use pokemonv2::data::CHIKORITA` if re-exported. Check existing call sites to decide.

**Recommended:** grep for all uses of `CHIKORITA`, `species_data(`, etc. to find all call sites, then decide whether a `pub use` re-export in `data.rs` is needed.

### 4d. Update mod.rs

```rust
pub mod data;
pub mod species_data;
```

Ensure `species_data` is declared in the module tree. If it is a submodule of `data`, it goes inside `data.rs` as `pub mod species_data;` — but that creates the circular import problem. Keep them as siblings in `mod.rs`.

---

## Phase 5: Tests

Add a `#[cfg(test)]` block at the bottom of `species_data.rs`:

### Test 1: All 251 species have valid data
```rust
#[test]
fn test_all_251_species_have_valid_data() {
    for id in 1..=251u16 {
        let data = species_data(id);
        assert_eq!(data.id, id, "species {} id mismatch", id);
        assert!(data.base_hp > 0, "species {} hp=0", id);
        assert!(data.base_attack > 0, "species {} atk=0", id);
        assert!(data.base_defense > 0, "species {} def=0", id);
        assert!(data.base_speed > 0, "species {} spd=0", id);
        assert!(data.base_sp_attack > 0, "species {} sat=0", id);
        assert!(data.base_sp_defense > 0, "species {} sdf=0", id);
        assert!(data.catch_rate > 0, "species {} catch_rate=0", id);
        assert!(!data.name.is_empty(), "species {} has empty name", id);
        assert!(!data.learnset.is_empty(), "species {} has empty learnset", id);
    }
}
```

### Test 2: Spot-check against pokecrystal ASM values

Verify a representative sample. Read the `.asm` file while implementing each entry:

```rust
#[test]
fn test_bulbasaur_matches_pokecrystal() {
    let d = species_data(BULBASAUR);
    assert_eq!(d.id, 1);
    assert_eq!(d.name, "BULBASAUR");
    assert_eq!((d.base_hp, d.base_attack, d.base_defense), (45, 49, 49));
    assert_eq!((d.base_speed, d.base_sp_attack, d.base_sp_defense), (45, 65, 65));
    assert_eq!(d.type1, PokemonType::Grass);
    assert_eq!(d.type2, PokemonType::Poison);
    assert_eq!(d.catch_rate, 45);
    assert_eq!(d.base_exp, 64);
    assert!(matches!(d.growth_rate, GrowthRate::MediumSlow));
}

#[test]
fn test_chikorita_matches_pokecrystal() {
    let d = species_data(CHIKORITA);  // id=152
    assert_eq!(d.id, 152);
    assert_eq!(d.name, "CHIKORITA");
    // chikorita.asm: db 45,49,65,45,49,65
    assert_eq!((d.base_hp, d.base_attack, d.base_defense), (45, 49, 65));
    assert_eq!((d.base_speed, d.base_sp_attack, d.base_sp_defense), (45, 49, 65));
    assert_eq!(d.type1, PokemonType::Grass);
    assert_eq!(d.type2, PokemonType::Grass);
    assert_eq!(d.catch_rate, 45);
    assert_eq!(d.base_exp, 64);
    assert!(matches!(d.growth_rate, GrowthRate::MediumSlow));
}

#[test]
fn test_magikarp_highest_catch_rate() {
    let d = species_data(MAGIKARP);  // id=129
    assert_eq!(d.catch_rate, 255);
    assert_eq!(d.name, "MAGIKARP");
}

#[test]
fn test_lugia_matches_pokecrystal() {
    let d = species_data(LUGIA);  // id=249
    assert_eq!(d.type1, PokemonType::Psychic);
    assert_eq!(d.type2, PokemonType::Flying);
    // lugia.asm: db 106,90,130,110,90,154
    assert_eq!((d.base_hp, d.base_attack, d.base_defense), (106, 90, 130));
    assert_eq!((d.base_speed, d.base_sp_attack, d.base_sp_defense), (110, 90, 154));
    assert_eq!(d.catch_rate, 3);
    assert!(matches!(d.growth_rate, GrowthRate::Slow));
}

#[test]
fn test_celebi_matches_pokecrystal() {
    let d = species_data(CELEBI);  // id=251
    assert_eq!(d.id, 251);
    assert_eq!(d.name, "CELEBI");
    assert_eq!(d.type1, PokemonType::Psychic);
    assert_eq!(d.type2, PokemonType::Grass);
    assert!(matches!(d.growth_rate, GrowthRate::MediumSlow));
}

#[test]
fn test_gastly_dual_type() {
    let d = species_data(GASTLY);  // id=92
    assert_eq!(d.type1, PokemonType::Ghost);
    assert_eq!(d.type2, PokemonType::Poison);
}
```

### Test 3: Regression — existing 17 species

The existing tests in `data.rs` (`test_caterpie_data_accuracy`, `test_bellsprout_data`, etc.) must all continue to pass. Do not modify them. After the refactor, run `cargo test` to confirm zero regressions.

### Test 4: Pokemon::new() integration

```rust
#[test]
fn test_pokemon_new_works_for_all_251() {
    use super::data::Pokemon;
    for id in 1..=251u16 {
        let p = Pokemon::new(id, 5);
        assert_eq!(p.species, id);
        assert!(p.hp > 0, "species {} hp=0 at level 5", id);
        assert!(p.moves.iter().any(|m| m.is_some()), "species {} has no moves", id);
    }
}
```

---

## Implementation Steps (in order)

1. **Read** the full `evos_attacks.asm` to build the learnset for each species. Do this while writing the learnset statics in `species_data.rs`.

2. **Create `species_data.rs`** with:
   - File header comment (AI-INSTRUCTIONS)
   - `use` imports from `super::data`
   - All 251 `SpeciesId` constants in dex order
   - All 251 learnset statics (read from `evos_attacks.asm`)
   - `NUM_SPECIES` constant
   - `SPECIES_TABLE` array with all 251 `SpeciesData` entries (read from `base_stats/*.asm`)
   - `species_data()` function
   - Test block

3. **Add new `MoveId` constants to `data.rs`** for any move referenced in learnsets that isn't already defined. Add them in a new grouped block:
   ```rust
   // --- Move ID Constants (Sprint 6: learnset moves) ---
   pub const MOVE_EMBER: MoveId = 52;
   pub const MOVE_SMOKESCREEN: MoveId = 108;
   // etc
   ```
   These constants just need the correct numerical value from pokecrystal. MoveData entries for these are NOT required in Sprint 6 (the `move_data()` fallback handles them).

4. **Remove from `data.rs`:**
   - All `static *_LEARNSET` arrays
   - All `static *_DATA: SpeciesData` statics
   - The `species_data()` function body
   - All `SpeciesId` constants (they now live in `species_data.rs`)

5. **Update `mod.rs`** to add `pub mod species_data;`

6. **Fix any import errors** in files that previously imported SpeciesId constants from `data.rs`. Since `mod.rs` has `pub mod species_data`, callers may need to update their `use` path from `pokemonv2::data::CHIKORITA` to `pokemonv2::species_data::CHIKORITA`. OR add re-exports in `mod.rs`:
   ```rust
   pub use species_data::{
       species_data, BULBASAUR, IVYSAUR, /* ... all 251 ... */ CELEBI,
       NUM_SPECIES,
   };
   ```

7. **Run `cargo test`** — all tests must pass.

8. **Run `cargo build --target wasm32-unknown-unknown`** — must compile.

---

## Key Invariants

- `SPECIES_TABLE` must have exactly `NUM_SPECIES` (251) entries. If the count is wrong, the compiler catches it with `[T; N]` array length mismatch.
- Each entry at index `i` must have `id == i + 1`. The parametric test catches this.
- No learnset may be empty. The parametric test catches this.
- All `SpeciesId` constants must equal the species' 1-based national dex number.
- The existing `SpeciesData` struct, `Pokemon` struct, and public API signatures do NOT change.

---

## File Summary

| File | Action | Expected size |
|---|---|---|
| `species_data.rs` | CREATE | ~3000+ lines |
| `data.rs` | REMOVE species statics, REMOVE old SpeciesId consts, ADD new MoveId consts | shrinks to ~700 lines |
| `mod.rs` | ADD `pub mod species_data;` | +1 line |

---

## Risk Notes

- **Learnsets for stubs:** For move constants with no `MoveData`, `Pokemon::new()` calls `move_data(id)` which falls back to `TACKLE_DATA` (pp=35). This is safe — wild Pokemon and trainers just get Tackle stats for undefined moves.
- **Ditto (id=132):** Has no level-up moves. Learnset must have at least `(1, MOVE_TRANSFORM)`. Check `evos_attacks.asm` — Ditto's entry is `db 1, TRANSFORM` (only move).
- **Unown (id=201):** Only learns Hidden Power. Check evos_attacks.asm for exact level.
- **Farfetch'd filename:** `farfetch_d.asm` — apostrophe becomes underscore in filename.
- **Mr. Mime filename:** `mr__mime.asm` — double underscore, no period.
- **Ho-Oh filename:** `ho_oh.asm` — hyphen becomes underscore.
