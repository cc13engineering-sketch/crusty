# Bilbo Review: Sprint 5 Implementation Plan

**VERDICT: ACCEPT WITH MODIFICATIONS**

---

## Summary

Frodo's plan is thorough, well-structured, and faithfully derived from Gandalf's architecture. The warp topology is correct, all pokecrystal data has been accurately transcribed, and the phased approach is sound. I found **4 modifications needed** -- all are minor corrections that don't affect the plan's structure.

---

## Modifications

### 1. Remove or Update Existing `test_route31_stub` Test

**What's wrong**: The existing test `test_route31_stub` (maps.rs line 1797) asserts `width == 20, height == 20` and `wild_encounters.is_none()`. The plan replaces the stub with a full 40x18 map that has encounters, which will cause this test to fail. The plan adds a new `test_route31_dimensions` but does not mention removing/updating the old test.

**Fix**: In Phase 2, add explicit instruction: "Delete or replace the existing `test_route31_stub` test (maps.rs ~line 1797). The new `test_route31_dimensions` and `test_route31_has_encounters` tests supersede it."

### 2. Update `test_all_map_entry_scripts_dont_panic` in mod.rs

**What's wrong**: The existing test `test_all_map_entry_scripts_dont_panic` (mod.rs ~line 941) iterates over all Sprint 1-2 maps to verify no panics on entry. It does not include Sprint 4 or Sprint 5 maps. If a new map's `check_map_entry_scripts` or `check_map_callbacks` panics, this test won't catch it.

**Fix**: In Phase 4, add instruction to update `test_all_map_entry_scripts_dont_panic` to include all Sprint 5 map IDs:
```rust
MapId::Route31, MapId::Route31VioletGate, MapId::VioletCity,
MapId::VioletMart, MapId::VioletGym, MapId::EarlsPokemonAcademy,
MapId::VioletNicknameSpeechHouse, MapId::VioletPokecenter1F,
MapId::VioletKylesHouse, MapId::SproutTower1F,
MapId::DarkCaveVioletEntrance, MapId::Route32, MapId::Route36,
```
Also add the Sprint 4 maps that were missed: `MapId::Route30, MapId::Route30BerryHouse, MapId::MrPokemonsHouse, MapId::Route31`.

### 3. Missing `METAPOD` Import in maps.rs

**What's wrong**: The plan's Phase 1I import line for maps.rs does not include `METAPOD`. While `METAPOD` is not used in Route 31 encounters, the existing maps.rs import line (line 9-11) already excludes it -- so this is fine as-is. However, the plan replaces the *entire* import line. Check that the replacement doesn't accidentally drop `METAPOD` if it's needed elsewhere.

**Fix**: No action needed. Verified that `METAPOD` is not referenced anywhere in maps.rs, only in data.rs. The current import line at maps.rs:11 does not import METAPOD, so the plan's replacement is consistent.

**(Modification withdrawn -- no issue found.)**

### 3. (Revised) `build_violet_city_stub` Return Warp Index Consistency Note

**What's wrong**: The `build_violet_city_stub()` function creates two return warps (warp 0 and warp 1) at `warp_x` and `warp_x + 1`, both pointing to the same `return_warp_id`. In pokecrystal, building exit warps typically have 2 tiles pointing to the *same* city warp index (e.g., VioletMart warps 1 and 2 both go to `VIOLET_CITY, 1`). The plan correctly implements this pattern. However, the city-side warps (VioletCity warp 0-6) each have a single tile that points to the building's warp 0 (`dest_warp_id: 0`). This is correct because `resolve_warp_position` uses warp 0 to determine the arrival position.

**Fix**: No modification needed. Verified correct.

### 3. (Actual) Missing `ITEM_POKE_BALL` Import in events.rs

**What's wrong**: The plan's events.rs import (3C) includes `ITEM_POKE_BALL` correctly. The `build_route31_poke_ball_script()` uses it. This is fine.

**Fix**: No modification needed. Verified correct.

### 3. Add `WEEDLE` to events.rs Imports (Already Covered)

The plan's 3C section does add `WEEDLE` to the events.rs imports. Verified this is present. No issue.

---

## Final Modifications List (Numbered)

1. **Delete existing `test_route31_stub` test** in maps.rs (~line 1797) during Phase 2. The plan's new tests supersede it.

2. **Update `test_all_map_entry_scripts_dont_panic`** in mod.rs during Phase 4 to include all Sprint 4 and Sprint 5 map IDs.

3. **`ITEM_PRZ_CURE_BERRY` is declared but unused**. This is acceptable (future fruit tree refinement) but note that the Violet City fruit tree script currently uses the generic `build_fruit_tree_script()` which doesn't give any item. If Gimli flags this as an accuracy issue, the fruit tree for Violet City should give a PRZ Cure Berry via `GiveItem`. This is a Gimli-level decision, not a plan structural issue -- leaving it as-is for now.

4. **Vine Whip `is_special: true` is correct** for Gen 2 (Grass type = special). No change needed. Just confirming for the record since this is a common accuracy pitfall.

---

## Verification Checklist

| Check | Result |
|-------|--------|
| Warp topology bidirectional | PASS -- all Route31 <-> Gate <-> VioletCity warps verified against pokecrystal 1-based -> 0-based conversion |
| Building return warps match pokecrystal | PASS -- all 7 stubs return to correct VioletCity warp index |
| Event flag numbering (44-57) | PASS -- no conflicts with existing flags (0-43) |
| Script ID numbering (400-446) | PASS -- no conflicts with existing IDs (1-334) |
| Wade party matches pokecrystal | PASS -- Caterpie/2, Caterpie/2, Weedle/3, Caterpie/2 |
| Bellsprout base stats | PASS -- 50/75/35/40/70/30, Grass/Poison, catch 255, base_exp 84, MediumSlow |
| Gastly base stats | PASS -- 30/35/30/80/100/35, Ghost/Poison, catch 190, base_exp 95, MediumSlow |
| Route 31 encounters vs pokecrystal | PASS -- all 21 slots match exactly |
| Bellsprout learnset | PASS -- VineWhip@1, Growth@6 |
| Gastly learnset | PASS -- Hypnosis@1, Lick@1 |
| Move data accuracy | PASS -- VineWhip(35/Grass/100/10/special), Hypnosis(0/Psychic/60/20), Lick(20/Ghost/100/30/physical), Growth(0/Normal/100/40) |
| MapId enum ordering | PASS -- Sprint 5 variants appended after Sprint 4 |
| Route30 north connection compatibility | PASS -- Route30 has north->Route31(offset -10), Route31 has south->Route30(offset 10) |
| Rust patterns (f64, no unwrap) | PASS -- plan uses i32 for coords, no unwrap in any builder |
| Test coverage | PASS with modifications 1-2 above |

---

## Overall Assessment

The plan is well-executed. Frodo correctly translated all pokecrystal data, built bidirectional warps with proper 0-based indexing, and provided comprehensive tests. The two required modifications are minor (stale test removal and existing test expansion). The plan is ready for Gimli's accuracy review and then implementation.
