# Sprint 6 Architecture: QA Audit + Species Database Expansion

## Situation

**Plan mismatch:** The sprint queue assumed sprints 4-5 defined all 251 species. In reality, sprints 4-5 were world-building sprints (Route 30, Route 31, Violet City). Only **17 species** exist in `data.rs`:

Chikorita (152), Cyndaquil (155), Totodile (158), Pidgey (16), Rattata (19), Sentret (161), Hoothoot (163), Hoppip (187), Caterpie (10), Metapod (11), Weedle (13), Zubat (41), Poliwag (60), Ledyba (165), Spinarak (167), Bellsprout (69), Gastly (92)

**QA result on existing 17:** All 17 species have been verified against `pokecrystal-master/data/pokemon/base_stats/*.asm`. Every field (HP, Atk, Def, Spd, SpAtk, SpDef, type1, type2, catch_rate, base_exp, growth_rate) matches the canonical ASM data exactly. No corrections needed.

Sprint 6 must therefore: (a) confirm the 17 are correct (done -- they are), and (b) expand to all 251 species.

## Data Source

**Canonical source:** `crates/engine-core/src/pokemon/pokecrystal-master/data/pokemon/`

Each species has a base stats file at `base_stats/<name>.asm` with this structure:
```asm
db SPECIES_NAME ; dex_number
db  hp, atk, def, spd, sat, sdf    ; base stats
db TYPE1, TYPE2                      ; types
db catch_rate                        ; catch rate
db base_exp                          ; base experience yield
db ITEM1, ITEM2                      ; wild held items
db GENDER_RATIO                      ; gender ratio constant
db 100                               ; unknown
db hatch_cycles                      ; egg step cycles
db 5                                 ; unknown
; ... sprite dimensions, unused pointers ...
db GROWTH_RATE                       ; growth rate
dn EGG_GROUP1, EGG_GROUP2           ; egg groups
```

Learnsets (level-up moves) are in `evos_attacks.asm`:
```asm
SpeciesEvosAttacks:
    db EVOLVE_LEVEL, level, EVOLVED_SPECIES  ; evolution entry (0+ of these)
    db 0                                      ; end evolutions
    db level, MOVE_NAME                       ; level-up move (1+ of these)
    db 0                                      ; end moves
```

## Proposed Approach

### Phase 1: SpeciesData struct -- no changes needed

The existing `SpeciesData` struct covers all fields needed for gameplay:
```rust
pub struct SpeciesData {
    pub id: SpeciesId,
    pub name: &'static str,
    pub type1: PokemonType,
    pub type2: PokemonType,
    pub base_hp: u8,
    pub base_attack: u8,
    pub base_defense: u8,
    pub base_speed: u8,
    pub base_sp_attack: u8,
    pub base_sp_defense: u8,
    pub catch_rate: u8,
    pub base_exp: u8,
    pub growth_rate: GrowthRate,
    pub learnset: &'static [(u8, MoveId)],
}
```

Fields like `gender_ratio` and `egg_groups` are NOT needed yet -- breeding/gender mechanics are far-future sprints. Adding them now would bloat every entry for no benefit. They can be added when breeding is implemented.

### Phase 2: File organization -- split into `species_data.rs`

The current `data.rs` is already ~1050 lines with just 17 species. Adding 234 more species (each needing ~10 lines for static data + learnset) would make `data.rs` ~3500+ lines. This is unwieldy.

**Proposed split:**
- `data.rs` -- keeps all non-species definitions: enums, Pokemon struct, stat formulas, move data, type effectiveness, item/move/music constants, battle/time-of-day enums. Unchanged.
- `species_data.rs` -- new file. Contains: all 251 `SpeciesData` statics, all learnset arrays, all `SpeciesId` constants, and the `species_data()` dispatch function.

`data.rs` will `pub use species_data::*;` so all downstream code is unaffected.

### Phase 3: Data pattern -- const array with index lookup

Replace the current `match id { ... }` dispatch with a **const array** indexed by national dex number (1-251). This is simpler, faster, and eliminates the risk of missing a match arm:

```rust
// species_data.rs

pub const NUM_SPECIES: usize = 251;

// All learnsets defined as static slices
static BULBASAUR_LEARNSET: &[(u8, MoveId)] = &[
    (1, MOVE_TACKLE), (4, MOVE_GROWL), (7, MOVE_LEECH_SEED), (10, MOVE_VINE_WHIP),
    // ...
];
// ... 250 more learnsets ...

// Master array indexed by (dex_number - 1)
static SPECIES_TABLE: [SpeciesData; NUM_SPECIES] = [
    // #001 Bulbasaur
    SpeciesData {
        id: 1, name: "BULBASAUR",
        type1: PokemonType::Grass, type2: PokemonType::Poison,
        base_hp: 45, base_attack: 49, base_defense: 49,
        base_speed: 45, base_sp_attack: 65, base_sp_defense: 65,
        catch_rate: 45, base_exp: 64,
        growth_rate: GrowthRate::MediumSlow,
        learnset: BULBASAUR_LEARNSET,
    },
    // ... #002 through #251 ...
];

pub fn species_data(id: SpeciesId) -> &'static SpeciesData {
    let idx = id as usize;
    if idx >= 1 && idx <= NUM_SPECIES {
        &SPECIES_TABLE[idx - 1]
    } else {
        &SPECIES_TABLE[0] // fallback to Bulbasaur
    }
}
```

**Why const array over match:**
- No chance of forgetting a species in a match arm
- O(1) lookup vs linear match scan
- Enforces contiguous coverage (compile error if array length wrong)
- More natural for 251 entries

### Phase 4: GrowthRate enum -- add missing variant

The ASM data uses `GROWTH_ERRATIC` for no Gen 2 species, but **does** use these four (which we already have):
- `GROWTH_FAST`
- `GROWTH_MEDIUM_FAST`
- `GROWTH_MEDIUM_SLOW`
- `GROWTH_SLOW`

No changes needed to the enum.

### Phase 5: Learnsets -- level-up moves only (minimal set)

For species whose moves haven't been defined yet as `MoveId` constants, use **stub learnsets** with `MOVE_TACKLE` as a placeholder. This is safe because:
- The game only instantiates Pokemon via `Pokemon::new()` which pulls from the learnset
- Wild encounters and trainer battles just need *some* valid move
- Full move data (all 251 moves) will come in Sprint 7+

Each learnset includes only level-up moves (from `evos_attacks.asm`), **not** TM/HM moves. TM/HM compatibility is a future sprint concern.

### Phase 6: SpeciesId constants

Define all 251 as sequential constants:
```rust
pub const BULBASAUR: SpeciesId = 1;
pub const IVYSAUR: SpeciesId = 2;
// ... through ...
pub const CELEBI: SpeciesId = 251;
```

Remove the scattered per-sprint constant blocks. Consolidate into one sorted block.

## Test Strategy

### 1. Parametric QA tests (all 251 species)
```rust
#[test]
fn test_all_251_species_have_valid_data() {
    for id in 1..=251u16 {
        let data = species_data(id);
        assert_eq!(data.id, id);
        assert!(data.base_hp > 0);
        assert!(data.base_attack > 0);
        assert!(data.base_defense > 0);
        assert!(data.base_speed > 0);
        assert!(data.base_sp_attack > 0);
        assert!(data.base_sp_defense > 0);
        assert!(data.catch_rate > 0);
        assert!(!data.name.is_empty());
        assert!(!data.learnset.is_empty());
    }
}
```

### 2. Spot-check tests against pokecrystal ASM (10-15 species)
Verify exact values for a representative sample covering:
- Starters: Chikorita, Cyndaquil, Totodile
- Common: Pidgey, Rattata, Geodude
- Dual-type: Bulbasaur (Grass/Poison), Gastly (Ghost/Poison)
- Legendaries: Lugia, Ho-Oh, Celebi
- Edge cases: Magikarp (highest catch rate 255), Chansey (low stats + high HP)

```rust
#[test]
fn test_bulbasaur_matches_pokecrystal() {
    let d = species_data(1);
    assert_eq!(d.name, "BULBASAUR");
    assert_eq!((d.base_hp, d.base_attack, d.base_defense), (45, 49, 49));
    assert_eq!((d.base_speed, d.base_sp_attack, d.base_sp_defense), (45, 65, 65));
    assert_eq!(d.type1, PokemonType::Grass);
    assert_eq!(d.type2, PokemonType::Poison);
    assert_eq!(d.catch_rate, 45);
    assert_eq!(d.base_exp, 64);
    assert!(matches!(d.growth_rate, GrowthRate::MediumSlow));
}
```

### 3. Regression tests
Ensure existing 17 species retain identical behavior. The existing tests in `data.rs` (test_caterpie_data_accuracy, test_bellsprout_data, test_gastly_data, etc.) should pass unchanged.

### 4. Pokemon::new() integration test
Verify that creating Pokemon from new species IDs works correctly:
```rust
#[test]
fn test_pokemon_new_works_for_all_251() {
    for id in 1..=251u16 {
        let p = Pokemon::new(id, 5);
        assert_eq!(p.species, id);
        assert!(p.hp > 0);
        assert!(p.moves.iter().any(|m| m.is_some()));
    }
}
```

## Implementation Order

1. Create `species_data.rs` with the `SPECIES_TABLE` array and all 251 entries
2. Move `SpeciesId` constants from `data.rs` to `species_data.rs`, add missing 1-251
3. Move `species_data()` function from `data.rs` to `species_data.rs`
4. Add `pub mod species_data;` to module tree, `pub use species_data::*;` in `data.rs`
5. Add parametric + spot-check tests
6. Run `cargo test` -- all existing tests must pass unchanged
7. Verify `cargo build --target wasm32-unknown-unknown` compiles

## Risks & Mitigations

**Risk:** Learnset stubs using MOVE_TACKLE for undefined moves may cause weird battle behavior.
**Mitigation:** Only affects species not yet encountered in-game. Sprint 7 will define all move data.

**Risk:** 251-entry const array is large (~3000 lines).
**Mitigation:** It's in its own file (`species_data.rs`). The data is static, rarely edited, and trivially searchable by species name.

**Risk:** Some species have base_exp = 0 would fail assertions (e.g., none in Gen 2 actually do, but edge case).
**Mitigation:** Cross-referenced against pokecrystal ASM -- all 251 species have base_exp > 0.

## Summary

- Existing 17 species: **verified correct** against pokecrystal ASM
- Struct changes: **none**
- New file: `species_data.rs` (all 251 species, const array pattern)
- Data source: pokecrystal `base_stats/*.asm` + `evos_attacks.asm`
- Tests: parametric all-251 + spot-checks + regression
- Estimated new lines: ~3000 (data) + ~100 (tests)
