# Bilbo's Senior Sprint Engineer Review: Sprint 2 Implementation Plan

**Verdict: ACCEPT WITH MODIFICATIONS**

The plan is comprehensive, well-structured, and demonstrates thorough cross-referencing with pokecrystal source data. It covers all 10 required systems from Gandalf's architecture. The 8-phase implementation order is sound. However, I've identified several compilation blockers, a borrow checker trap, and some design inconsistencies that must be fixed before Mary/Pippin/Sam start coding.

---

## Critical Issues (Must Fix Before Implementation)

### 1. `wild_encounters` Field Type Mismatch -- Breaking Change to MapData

**Phase**: 2 (maps.rs)
**Problem**: The plan says to replace `wild_encounters: Vec<WildEncounter>` with `wild_encounters: Option<WildEncounterTable>`. This is correct conceptually, but ALL existing Sprint 1 maps set `wild_encounters: vec![]`. The plan must provide a migration path:

- All existing `load_map` builders (`build_players_house_2f`, `build_players_house_1f`, `build_new_bark_town`, `build_elms_lab`, `build_stub_house`, `build_stub_route`) must change `wild_encounters: vec![]` to `wild_encounters: None`.
- The `WildEncounter` struct (lines 118-123 of maps.rs) must be removed entirely since it's replaced by `WildSlot` + `WildEncounterTable`.

**Fix**: Explicitly state that `wild_encounters: vec![]` becomes `wild_encounters: None` in all Sprint 1 map builders. Remove the old `WildEncounter` struct.

### 2. `step_script()` Return Type Needs Refactoring for Battle Integration

**Phase**: 6 (mod.rs) / 3 (events.rs)
**Problem**: The current `step_script()` returns `bool` (true = still running, false = ended). But Sprint 2 adds `LoadWildMon` and `StartBattle` script steps that need to signal mod.rs to create a `BattleState` and switch to `GamePhase::Battle`. The plan's Phase 6 code shows:

```rust
return ScriptResult::StartBattle {
    battle_type: *battle_type,
    species: script.loaded_wild_species,
};
```

But `step_script()` currently returns `bool`, not `ScriptResult`. This is a significant refactor.

**Fix**: Change `step_script()` to return an enum:

```rust
pub enum ScriptResult {
    Running,       // still executing
    Ended,         // hit End step
    StartBattle {  // transition to battle
        battle_type: BattleType,
        species: Option<(SpeciesId, u8)>,
    },
}
```

Update all callers in `mod.rs` -- specifically `step_script_phase()`:

```rust
fn step_script_phase(&mut self, engine: &Engine) {
    if let Some(ref mut script) = self.script {
        let result = step_script(script, ...);
        match result {
            ScriptResult::Running => {}
            ScriptResult::Ended => {
                self.script = None;
                self.phase = GamePhase::Overworld;
                self.refresh_npc_visibility();
            }
            ScriptResult::StartBattle { battle_type, species } => {
                // Create battle, switch phase
            }
        }
    }
}
```

Also need a `loaded_wild_species: Option<(SpeciesId, u8)>` field on `ScriptState` for the `LoadWildMon` -> `StartBattle` handoff. The plan mentions this but doesn't add it to the ScriptState struct definition.

### 3. `step_overworld()` Signature Needs RNG + TimeOfDay Parameters

**Phase**: 5 (overworld.rs)
**Problem**: The current `step_overworld()` signature does not include parameters for RNG bytes or TimeOfDay, but the wild encounter check needs both. The plan's pseudocode says `let rng_enc = /* get byte from engine rng */` but doesn't specify how.

**Fix**: Either:
- (A) Pass `rng_byte_enc: u8` and `rng_byte_slot: u8` and `time_of_day: TimeOfDay` as parameters to `step_overworld()`. This means `step_overworld_phase()` in mod.rs must read from `engine.rng` before calling.
- (B) Pass the `Engine` reference (already present) and read `engine.rng` inside `step_overworld()`.

Option (B) is simpler since `engine: &Engine` is already a parameter. But `engine.rng` might need to be mutable to generate random bytes. If `SeededRng` has a `next()` method that requires `&mut self`, we need `&mut Engine` or must extract bytes beforehand.

**Recommended**: Option (A). The caller extracts RNG bytes and TimeOfDay from `engine` and `self.total_time` before calling `step_overworld()`. This keeps `step_overworld()` pure and testable:

```rust
fn step_overworld_phase(&mut self, engine: &Engine) {
    let time_of_day = get_time_of_day(self.total_time);
    let rng_enc = (engine.rng.state & 0xFF) as u8;
    let rng_slot = ((engine.rng.state >> 8) & 0xFF) as u8;
    let result = step_overworld(
        &mut self.player, &mut self.camera, &self.current_map,
        &mut self.npc_states, &self.event_flags, &self.scene_state,
        engine, time_of_day, rng_enc, rng_slot,
    );
    // ... handle result
}
```

### 4. `battle.rs` Imports `BattleType` from `events.rs` -- Import Graph Violation

**Phase**: 4 (battle.rs)
**Problem**: The plan's import graph says `battle.rs` depends ONLY on `data.rs`. But the Phase 4 code shows:

```rust
use super::events::BattleType;
```

This creates: `battle.rs -> events.rs` AND `events.rs` defines `StartBattle { battle_type: BattleType }` which means events.rs needs BattleType too. If BattleType is in events.rs, that's fine (battle imports from events, not vice versa). But the import graph in the plan says battle depends only on data.

**Fix**: Either:
- (A) Move `BattleType` and `BattleResult` to `data.rs` (it's a leaf-level enum with no dependencies). Then both `battle.rs` and `events.rs` import from data. Clean graph.
- (B) Accept that `battle.rs -> events.rs` is a one-way dependency. Update the import graph accordingly.

**Recommended**: (A). Move `BattleType` to `data.rs`. It's a pure data enum with no sibling imports. This keeps battle.rs truly dependent only on data.rs.

### 5. `check_wild_encounter` Uses `map.wild_encounters` as `Option<WildEncounterTable>` But Is Called in `overworld.rs`

**Phase**: 5 (overworld.rs)
**Problem**: `check_wild_encounter()` is defined in overworld.rs but references `map.wild_encounters` as `Option<WildEncounterTable>`. The `WildEncounterTable` struct is defined in maps.rs. This is fine -- overworld already imports from maps. But `TimeOfDay` is imported from data.rs, not maps.rs. The plan's dependency section says `use super::data::TimeOfDay;` which is correct only if TimeOfDay is in data.rs (it is, per Phase 1). Good.

However, `check_wild_encounter` also needs `C_GRASS` from maps.rs. The plan's dependency section says `use super::maps::{C_GRASS, C_LEDGE_D, C_LEDGE_L, C_LEDGE_R};` -- correct.

**No fix needed**, but the plan should explicitly list the full import set for overworld.rs at the top of Phase 5.

### 6. Route 29/46 Gate Dimensions Discrepancy

**Phase**: 2 (maps.rs)
**Problem**: The architecture doc says "4x4 blocks" for Route29Route46Gate. But the implementation plan says "5x4 blocks = 10x8 tiles". Pokecrystal's `map_const ROUTE_29_ROUTE_46_GATE, 5, 4` means 5 blocks wide, 4 blocks tall = 10x8 tiles. The implementation plan's dimensions (10x8 tiles) are correct. But the architecture table says "4x4 blocks = 8x8 tiles" which is wrong.

**Fix**: Use the implementation plan's 10x8 tiles (5x4 blocks). The architecture doc table has a typo.

### 7. `is_walkable_with_direction` Placement

**Phase**: 5 (overworld.rs)
**Problem**: The plan puts `is_walkable_with_direction` in overworld.rs, but it references `C_FLOOR`, `C_WARP`, `C_GRASS`, `C_LEDGE_D`, `C_LEDGE_L`, `C_LEDGE_R` from maps.rs. The existing `is_walkable` is in maps.rs. Having two walkability functions in different modules is confusing.

**Fix**: Put `is_walkable_with_direction` in maps.rs alongside `is_walkable`. Overworld.rs already imports `is_walkable` from maps. Add the new function there and update overworld.rs to import it. This keeps collision logic centralized in maps.rs.

---

## Medium Issues (Should Fix)

### 8. `map.width as i32` Casts Are Redundant

**Phase**: 5 (overworld.rs)
**Problem**: The plan's `is_walkable_with_direction` casts `map.width as i32`. But `map.width` is already `i32` (established in Sprint 1, maps.rs line 52: `pub width: i32`). These casts compile but are misleading. Similarly `map.height as i32` is redundant.

**Fix**: Remove `as i32` casts from map width/height comparisons throughout the plan.

### 9. SceneState Vec Size Needs Expansion

**Phase**: 3 (events.rs)
**Problem**: SceneState currently initializes with `vec![0u8; 16]`, enough for Sprint 1's 8 MapId variants. Sprint 2 adds 8+ new MapId variants (16+ total). The `set()` method does `resize()` when needed, so it won't panic, but it's cleaner to initialize with a larger size.

**Fix**: Change `SceneState::new()` to `vec![0u8; 32]` to accommodate Sprint 2 maps without mid-game resizes.

### 10. `Battle` GamePhase Variant Already Exists as Stub

**Phase**: 6 (mod.rs)
**Problem**: The plan says to add `Battle` to GamePhase. It already exists (mod.rs line 38: `Battle, // stub`). The plan should say "update the existing Battle variant" and wire it up, not "add new variant". The current `step()` method has `GamePhase::Battle | GamePhase::Menu => {} // stubs` which must be replaced.

**Fix**: Replace the `GamePhase::Battle | GamePhase::Menu => {}` line with the battle step logic. Keep `Menu` as stub.

### 11. Missing `battle` Field on PokemonV2Sim

**Phase**: 6 (mod.rs)
**Problem**: The plan says to add `pub battle: Option<BattleState>` to PokemonV2Sim. This must also be initialized in `PokemonV2Sim::new()` as `battle: None` and in `with_state()`. The plan doesn't explicitly list the constructor update.

**Fix**: Add `battle: None` to both `new()` and `with_state()` constructors.

### 12. `render_battle` Function Signature Mismatch

**Phase**: 7 (render.rs)
**Problem**: The plan's `draw_text` calls in `render_battle` use a different signature than the existing `draw_text` function. The existing function is `fn draw_text(engine, text, x, y, color)` but the plan shows `draw_text(engine, x, y, text, color)` with x,y before text.

**Fix**: Use the existing signature: `draw_text(engine, text, x, y, color)`.

### 13. Missing `Route30` MapId Variant

**Phase**: 2 (maps.rs)
**Problem**: CherrygroveCity has a north connection to Route30, but Route30 is not in the Sprint 2 MapId enum. The plan mentions adding it as a note but doesn't include it in the MapId enum definition.

**Fix**: Add `Route30` to the MapId enum and provide a stub map (same pattern as Route27).

### 14. Music Constants Referenced But Not Defined

**Phase**: 3 (events.rs)
**Problem**: The Guide Gent script uses `PlayMusic(MUSIC_SHOW_ME_AROUND)`, rival script uses `PlayMusic(MUSIC_RIVAL_ENCOUNTER)` and `PlayMusic(MUSIC_RIVAL_AFTER)`. These constants are never defined.

**Fix**: Add music ID constants to events.rs (or data.rs):

```rust
pub const MUSIC_SHOW_ME_AROUND: u8 = 10;
pub const MUSIC_RIVAL_ENCOUNTER: u8 = 11;
pub const MUSIC_RIVAL_AFTER: u8 = 12;
```

These are stubs (no audio system), but the constants must exist for the code to compile.

### 15. Borrow Checker: `self.party.first_mut()` in Battle Phase

**Phase**: 6 (mod.rs)
**Problem**: The plan's battle step code has:

```rust
if let Some(ref mut battle) = self.battle {
    if let Some(ref mut pokemon) = self.party.first_mut() {
        let still_running = step_battle(battle, pokemon, FIXED_DT, rng_byte);
```

This borrows `self.battle` mutably via `ref mut battle`, then borrows `self.party` mutably via `first_mut()`. Both are disjoint fields of `self`, so Rust's borrow checker handles this correctly with disjoint field borrows. **This is fine.**

However, the next block accesses `self.current_battle_type()` and `self.battle` again after the inner block. If `self.battle` is still borrowed, this would fail. But since `battle` goes out of scope after the `if let`, it should be okay.

**No fix needed**, but implementers should be careful to keep the battle borrow scoped tightly.

---

## Minor Issues (Nice to Fix)

### 16. Script Index Arithmetic in Catching Tutorial

**Phase**: 3 (events.rs)
**Problem**: The catching tutorial script has `YesNo { yes_jump: 9, no_jump: 14 }` with a comment saying the "no" path is at step 14, but later says "adjust to step 21". The actual vec indices are correct ONLY if each script step takes exactly one vec element. The plan warns about this but doesn't provide verified indices.

**Fix**: Implementers must count vec elements carefully. The plan should provide a commented version with explicit indices. But this is an implementation detail, not a plan defect.

### 17. `MOVE_STRUGGLE` Constant Defined in battle.rs, Not data.rs

**Phase**: 4 (battle.rs)
**Problem**: `MOVE_STRUGGLE` is defined as a constant in battle.rs, but `move_data()` in data.rs needs to handle it. The plan says "Add to `data.rs` move_data" for the Struggle MoveData entry, but defines the constant in battle.rs.

**Fix**: Move `pub const MOVE_STRUGGLE: MoveId = 165;` to data.rs alongside other move constants. Then battle.rs imports it from data.rs.

### 18. Existing `is_walkable` Used by NPC Wander in overworld.rs

**Phase**: 5 (overworld.rs)
**Problem**: The plan says to update `is_walkable` so C_GRASS is walkable (adding `c == C_GRASS` to the check). But the existing `is_walkable` is also used by `tick_npc_wander` for NPC movement bounds checking. NPCs should also be able to walk on grass tiles, so this is correct.

**No fix needed** -- just noting that this is consistent.

### 19. `find_npc_by_event_flag` Method Doesn't Exist

**Phase**: 3 (events.rs) / 6 (mod.rs)
**Problem**: The `check_map_callbacks` code uses `self.find_npc_by_event_flag(EVENT_ROUTE_29_TUSCANY_OF_TUESDAY)` but this method doesn't exist on PokemonV2Sim.

**Fix**: Either implement it or simplify to:

```rust
for (i, npc_def) in self.current_map.npcs.iter().enumerate() {
    if npc_def.event_flag == Some(EVENT_ROUTE_29_TUSCANY_OF_TUESDAY) {
        if let Some(state) = self.npc_states.get_mut(i) {
            state.visible = false;
        }
    }
}
```

---

## Architecture Praise (What's Good)

1. **Phased implementation order** -- Data -> Maps -> Events -> Battle -> Overworld -> Mod -> Render -> Tests. Each phase builds on the previous without forward references. This is exactly right for compilation-order development.

2. **WildEncounterTable with time-of-day slots** -- Direct translation of pokecrystal's 7-slot-per-period format. The probability distribution (30/30/20/10/5/2.5/2.5) is correct.

3. **Battle system scoping** -- Auto-battle-only for Sprint 2 is the right decision. No move selection UI, no bag, no switching. This delivers the minimum needed for catching tutorial, rival fight, and wild encounters without over-engineering.

4. **Map connection fade transition** -- Correct to defer seamless scrolling. The existing MapTransition infrastructure handles fade-to-black. Reuse it.

5. **Mart stub as dialogue** -- Good pragmatic decision. Full mart UI is a significant feature; stub it for now.

6. **Comprehensive warp bidirectionality table** -- The cross-reference table in Phase 2 is excellent. Every warp has a verified return path.

7. **Script text fidelity** -- All NPC dialogue is directly sourced from pokecrystal .asm files. This is exactly what the project requires.

8. **Edge case handling** -- Hoppip Splash/Struggle handling, Tuscany always-hidden stub, tutorial auto-catch, CanLose no-gameover. Good coverage.

9. **Test specifications** -- 17+ test specs covering maps, warps, connections, encounters, ledges, battles, and scripts. Good Sprint 2 coverage.

10. **Import graph maintenance** -- The updated acyclic graph is clearly documented and correct (pending the BattleType fix in issue #4).

---

## Summary of Required Modifications

| # | Phase | Change | Priority |
|---|-------|--------|----------|
| 1 | 2 | Migrate `wild_encounters: vec![]` to `None` in all Sprint 1 map builders; remove old `WildEncounter` struct | CRITICAL |
| 2 | 3,6 | Change `step_script()` return type to `ScriptResult` enum; add `loaded_wild_species` to ScriptState | CRITICAL |
| 3 | 5,6 | Pass RNG bytes + TimeOfDay to `step_overworld()` or extract before call | CRITICAL |
| 4 | 4 | Move `BattleType` (and `BattleResult`) to `data.rs` to maintain clean import graph | HIGH |
| 7 | 5 | Put `is_walkable_with_direction` in `maps.rs`, not `overworld.rs` | HIGH |
| 8 | 5 | Remove redundant `as i32` casts on map width/height | MEDIUM |
| 9 | 3 | Expand SceneState initial vec to 32 entries | MEDIUM |
| 10 | 6 | Use existing `Battle` GamePhase variant; don't redeclare | MEDIUM |
| 11 | 6 | Add `battle: None` to `new()` and `with_state()` constructors | MEDIUM |
| 12 | 7 | Fix `draw_text` argument order to match existing signature | MEDIUM |
| 13 | 2 | Add `Route30` to MapId enum with stub map | MEDIUM |
| 14 | 3 | Define music constants (MUSIC_SHOW_ME_AROUND, etc.) | MEDIUM |
| 17 | 4 | Move MOVE_STRUGGLE constant to data.rs | LOW |
| 19 | 6 | Implement `find_npc_by_event_flag` or inline the loop | LOW |

**Bottom line**: 4 critical issues that would prevent compilation, 3 high-priority structural fixes, and 7 medium/low cleanup items. All are straightforward to fix. The plan's design decisions are sound and the scope is appropriate for Sprint 2.
