# Pokemon Crystal -- Move Edge Cases and Special Mechanics

Source: pokecrystal disassembly (engine/battle/move_effects/*.asm)

---

## Baton Pass

### What IS Passed
- All stat stage changes (Attack, Defense, SpAtk, SpDef, Speed, Accuracy, Evasion)
- Confusion (with remaining turn count)
- Focus Energy / Dire Hit
- Substitute (with remaining HP)
- Mean Look / Spider Web trap -- **WAIT, NO**: Actually NOT passed (cleared in ResetBatonPassStatus via wPlayerWrapCount/wEnemyWrapCount reset)
- Perish Song counter (continues counting down on the new Pokemon)
- Leech Seed
- Lock-On / Mind Reader
- Ingrain (not in Gen 2)
- Curse (non-Ghost, the stat changes are passed; Ghost-type Curse damage IS passed as it's a volatile status)

### What is NOT Passed
- Nightmare (explicitly cleared if the new Pokemon is not asleep)
- Disable (explicitly cleared via ResetActorDisable)
- Attraction / Infatuation (explicitly cleared for both sides)
- Transform status (cleared)
- Encore (cleared, along with its counter)
- Wrap/Bind/Fire Spin (wPlayerWrapCount and wEnemyWrapCount set to 0)
- Last move used (set to 0, preventing Encore/Mirror Move/Counter from working)
- Type changes from Conversion/Conversion2

### Edge Cases
- If the user has no other Pokemon to switch to, Baton Pass fails
- Wild Pokemon cannot use Baton Pass (no team to switch to)
- Baton Pass does trigger Spikes damage on the incoming Pokemon
- Baton Pass does NOT trigger Pursuit (it's a switch by effect, not by switch command)

---

## Perish Song + Mean Look Interactions

- Perish Song affects both Pokemon
- Mean Look prevents switching
- Perish Song counter IS passed by Baton Pass
- Mean Look trap is NOT passed by Baton Pass
- Classic combo: Use Mean Look, then Perish Song. The opponent is trapped and will faint in 3 turns. You can Baton Pass out (keeping the Perish counter but losing the trap) or switch normally (losing the Perish counter).
- If BOTH Pokemon have Perish Song active and neither switches: the one that reaches 0 first faints. They're checked in order (player first or enemy first depending on link clock).

---

## Sleep Talk

### Core Mechanics
- Can only be used while asleep
- Randomly selects one of the user's other 3 moves and executes it
- The selected move's PP is NOT consumed (Sleep Talk's PP is consumed instead)

### Exclusions (moves Sleep Talk will NOT call)
- Sleep Talk itself (explicitly filtered out)
- The disabled move (if any)
- Two-turn moves: Fly, Dig, Razor Wind, Sky Attack, Skull Bash, SolarBeam, Bide
- Empty move slots

### Special Interactions
- Sleep Talk CAN call Rest, but Rest fails (the Pokemon is already asleep)
- Sleep Talk CAN call Focus Punch (not in Gen 2)
- If Sleep Talk calls a two-turn move, it would be excluded (filtered out during selection)
- If ALL other moves are two-turn moves or disabled, Sleep Talk fails

---

## Counter and Mirror Coat

### Counter
- **Type check**: Only reflects damage from **physical** moves (type < SPECIAL constant)
- **Damage**: Returns 2x the damage received from the last physical move
- **Priority**: -1 (always goes last in its priority bracket)
- **Conditions for success**:
  1. Opponent used a move last turn (wLastCounterMove is not 0)
  2. The last move was not Counter itself
  3. Type matchup is not immune (checked via BattleCommand_ResetTypeMatchup)
  4. The opponent went first this turn (CheckOpponentWentFirst)
  5. The last move had non-zero base power
  6. The last move was physical type
  7. Damage taken was non-zero (wCurDamage > 0)
- **BUG**: Counter still works if the opponent used an item instead of a move (the last move data isn't cleared when using items)

### Mirror Coat
- **Type check**: Only reflects damage from **special** moves (type >= SPECIAL constant)
- **Damage**: Returns 2x the damage received from the last special move
- **Priority**: -1
- **Same conditions as Counter**, but checks for special type instead of physical
- **BUG**: Same item bug as Counter

### Key Edge Cases
- Counter/Mirror Coat fail if the user moves first (they need to take damage first)
- They use the damage value stored in wCurDamage, which persists from the last hit taken
- If a Substitute absorbed the damage, wCurDamage is still set, so Counter/Mirror Coat still work against Substitutes
- Counter cannot reflect special moves; Mirror Coat cannot reflect physical moves
- Neither works against fixed-damage moves that have no type (but this doesn't apply in Gen 2 -- Seismic Toss is Normal/physical, Night Shade is Ghost/special)

---

## Future Sight

- **Delay**: Stored damage is delivered 3 turns later (counter starts at 4, triggers at 1)
- **Damage calculation**: Uses the user's stats at the time of USING Future Sight (damage is pre-calculated and stored in wPlayerFutureSightDamage/wEnemyFutureSightDamage)
- **Type effectiveness**: Ignores type on delivery (the stored damage is applied directly)
- **Cannot stack**: Using Future Sight while a previous one is pending fails
- **Cannot be reflected**: Counter/Mirror Coat don't work against Future Sight damage
- **Timing**: Delivered during HandleFutureSight in HandleBetweenTurnEffects (before weather, wrap, perish song)

---

## Hidden Power

### Type Calculation
```
TypeIndex = ((AtkDV & 3) << 2) + (DefDV & 3)
TypeIndex++ (skip Normal)
if TypeIndex >= BIRD: TypeIndex++ (skip Bird type)
if TypeIndex >= UNUSED_TYPES: TypeIndex += (UNUSED_TYPES_END - UNUSED_TYPES) (skip unused type slots)
```

Possible types: All types EXCEPT Normal and (the internal Bird type). In practice: Fighting, Flying, Poison, Ground, Rock, Bug, Ghost, Steel, Fire, Water, Grass, Electric, Psychic, Ice, Dragon, Dark.

### Power Calculation
```
Power = ((AtkDV & 8) | ((DefDV & 8) >> 1) | ((SpdDV & 8) >> 2) | ((SpcDV & 8) >> 3)) * 5
       + (SpcDV & 3)
Power = Power / 2 + 31
```

Range: 31 to 70.

The top bit of each DV contributes to a 4-bit value (0-15), multiplied by 5. The lowest 2 bits of Special DV add 0-3. The result is halved and 31 is added.

---

## Present

### Power Distribution
From PresentPower table:
| Random value range | Power | Probability |
|-------------------|-------|-------------|
| 0-101 (40%) | 40 | 40% |
| 102-179 (30%) | 80 | 30% |
| 180-203 (10%) | 120 | 10% |
| 204-255 (20%) | Heal | 20% |

Healing effect: Restores 1/4 of the TARGET's max HP (heals the opponent!).

**BUG**: In link battles, Present damage is calculated incorrectly because the STAB/type effectiveness calculation pushes bc/de in single-player mode but not in link mode, causing register corruption.

---

## Return and Frustration

### Return
```
Power = Happiness * 10 / 25
```
At max happiness (255): Power = 102
At min happiness (0): Power = 0

**BUG**: When happiness is 0-2, the formula rounds down to 0 power, dealing no damage at all.

### Frustration
```
Power = (255 - Happiness) * 10 / 25
```
At min happiness (0): Power = 102
At max happiness (255): Power = 0

**BUG**: When happiness is 253-255, the formula rounds down to 0 power.

---

## Rollout and Defense Curl

### Rollout Mechanics
- 5 consecutive hits, damage doubling each hit
- Hit 1: base damage, Hit 2: 2x, Hit 3: 4x, Hit 4: 8x, Hit 5: 16x
- If a hit misses, the sequence ends
- After 5 hits, the move ends naturally
- Rollout locks the user in (can't switch or choose another move)

### Defense Curl Interaction
- If the user has used Defense Curl (SUBSTATUS_CURLED is set), Rollout damage is further doubled
- Effectively: Hit 1: 2x, Hit 2: 4x, ... Hit 5: 32x
- Defense Curl's Rollout boost persists even after stat stages are reset (it's a separate flag)

### Ice Ball
- Not in Gen 2 (introduced Gen 3)

---

## Fury Cutter

- Starts at base power (10 in Gen 2)
- Doubles each consecutive successful hit: 10, 20, 40, 80, 160
- **Caps at 5 turns' worth** (16x multiplier, so max effective power = 160)
- Missing resets the counter
- Switching resets the counter
- Using any other move resets the counter

---

## Magnitude

Random power selection from MagnitudePower table:

| Magnitude | Power | Probability |
|-----------|-------|-------------|
| 4 | 10 | 5% |
| 5 | 30 | 10% |
| 6 | 50 | 20% |
| 7 | 70 | 30% |
| 8 | 90 | 20% |
| 9 | 110 | 10% |
| 10 | 150 | 5% |

Average power: 71

Magnitude can hit Dig users for double damage (same as Earthquake).

---

## Transform, Mimic, and Sketch

### Transform
- Copies the target's: species (appearance), types, stats (except HP), stat stages, moves (all 4), and current PP (set to 5 for each move)
- Does NOT copy: HP, level, DVs, status conditions
- The transformed Pokemon's "real" species is preserved internally
- BUG: Catching a Transformed Pokemon always catches a Ditto (it checks the original species)
- A Transformed Pokemon cannot use Sketch successfully
- BUG: A Transformed Pokemon CAN use Sketch to learn moves, which can result in otherwise unobtainable movesets

### Mimic
- Copies the target's last-used move, replacing Mimic in the moveset
- Lasts until the Pokemon switches out
- PP for the mimicked move is set to the move's normal max PP
- Does not consume PP from the original move

### Sketch
- Permanently copies the target's last-used move, replacing Sketch
- This is a permanent change (persists after battle)
- Cannot copy Struggle or Sketch itself
- Fails if the target hasn't used a move

---

## Metronome

- Randomly selects any move in the game and executes it
- Cannot call: Metronome, Struggle, Sketch, Mimic, Sleep Talk, Counter, Mirror Coat, Protect, Detect, Endure, Destiny Bond, Thief
- The selected move uses Metronome's PP (not the called move's PP)
- Can call two-turn moves (executes the charge turn, requiring another Metronome use to complete -- but in practice the user would need to be locked in)

---

## Encore

- Forces the target to repeat their last-used move for 2-6 turns
- Fails if: target hasn't used a move, target used Encore/Mirror Move/Struggle/Transform
- The Encored move's PP is still consumed normally
- If the Encored move runs out of PP, the Pokemon uses Struggle
- Cleared on switch
- Not passed by Baton Pass

---

## Disable

- Disables the target's last-used move for 1-7 turns
- The disabled move cannot be selected
- If the disabled move is the only move with PP, the Pokemon uses Struggle
- Duration counter stored as upper nibble of wPlayerDisableCount/wEnemyDisableCount
- Cleared on switch
- BUG: A Disabled but PP Up-enhanced move may not trigger Struggle (checks `and a` instead of `and PP_MASK`)

---

## Pain Split

- Averages both Pokemon's current HP: each gets (user HP + target HP) / 2
- If the total is odd, the user gets the smaller half
- Can raise HP above current (but not above max)
- If the result exceeds max HP, it's capped at max HP
- Ignores type immunity and Substitute

---

## Belly Drum

- Maxes the user's Attack stat stage to +6
- Costs 1/2 of max HP
- **BUG**: The game checks Attack boost success BEFORE checking HP. So if Attack is already at +6 (boost fails), the move fails without losing HP. But if Attack can be boosted at all, it first does a +2 boost, then checks HP. If HP < 50%, the +2 boost STILL APPLIES but the move "fails" (the HP isn't subtracted, but Attack still went up by 2 stages).

---

## Thief

- Steals the target's held item
- Only works if: the user has no held item AND the target has a held item
- The user takes the target's item; the target loses it
- In trainer battles: the item is returned after battle
- In wild battles: the item is permanently stolen
- Mail cannot be stolen

---

## Rapid Spin

- Removes: Leech Seed, Spikes (from user's side), Bind/Wrap/Clamp/Fire Spin/Whirlpool
- Also deals damage (20 base power, Normal type)
- The removal effects happen even if the damage portion misses (as long as the move is used)

---

## Psych Up

- Copies ALL of the target's stat stage changes (Attack, Defense, SpAtk, SpDef, Speed, Accuracy, Evasion)
- Does NOT copy: Focus Energy, type changes, or other volatile conditions
- Overwrites the user's current stat stages entirely

---

## Beat Up

- Hits once for each non-fainted, non-statused Pokemon in the user's party
- Each hit uses the base Attack of the party member and a fixed base power
- Type: Dark (all hits)
- **BUG**: With only one Pokemon in the party, Beat Up fails to raise Substitute and King's Rock doesn't proc
- **BUG**: Can desynchronize link battles due to a register comparison error
- **BUG**: May trigger King's Rock even if all hits failed
