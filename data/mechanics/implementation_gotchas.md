# Pokemon Crystal — Top 50 Implementation Gotchas

Source: pokecrystal disassembly (docs/bugs_and_glitches.md, engine/battle/core.asm, engine/battle/effect_commands.asm, engine/items/item_effects.asm, constants/type_constants.asm)

This is the "lessons learned" document for anyone building a Gen 2 battle engine. Every entry below is something that trips up recreations — either because the original game has a bug you must replicate for accuracy, the wiki describes it wrong, or the order-of-operations is non-obvious.

---

## Category 1: Integer Overflow and Truncation

### Gotcha #1: TruncateHL_BC causes stat wrap-around above 1024

The damage formula uses 16-bit Attack and Defense values but truncates them to 8 bits via `TruncateHL_BC`. This function right-shifts both values by 2 repeatedly until the high bytes are zero, then uses the low bytes. If Attack or Defense exceeds 1024 (after boosts like Thick Club, Light Ball, Reflect, Light Screen), the truncation creates a ratio that wraps, making the boosted stat appear SMALLER.

**The trap:** In non-link battles, Crystal runs the truncation check TWICE (a loop re-check after `.finish`). In link battles, it only runs ONCE (skips the re-check via `cp LINK_COLOSSEUM`). This means the Reflect/Light Screen overflow bug only manifests in link battles.

Source: `engine/battle/effect_commands.asm` lines 2614-2658

### Gotcha #2: Thick Club and Light Ball double BEFORE truncation

`SpeciesItemBoost` doubles the stat value (`sla l; rl h`) with no cap. If the result exceeds 1024, TruncateHL_BC wraps it. Example: A Marowak with 512+ Attack holding Thick Club gets doubled to 1024+, which wraps to a tiny value, dealing LESS damage than without the item.

Source: `engine/battle/effect_commands.asm`, SpeciesItemBoost

### Gotcha #3: Metal Powder is applied AFTER truncation

`DittoMetalPowder` multiplies Defense by 1.5 AFTER `TruncateHL_BC` has already reduced it to 8 bits. Because it operates on the 8-bit value `c` (not the original 16-bit value), if the 1.5x multiplication causes c to overflow 255, it wraps around to a small value, potentially REDUCING defense.

Source: `engine/battle/effect_commands.asm`, DittoMetalPowder (called after TruncateHL_BC)

### Gotcha #4: Experience underflow for level 1 Medium-Slow Pokemon

The Medium-Slow growth rate formula `(6/5)*n^3 - 15*n^2 + 100*n - 140` yields -54 at level 1. Since experience is unsigned 24-bit, this underflows to 16,777,162 — a massive value. This means a level 1 Medium-Slow Pokemon thinks it has max experience and cannot gain more.

Source: `engine/pokemon/experience.asm`, CalcExpAtLevel

### Gotcha #5: Stat stage multipliers are integer division

Stat stage multipliers use integer division: multiplied by numerator, then divided by denominator. At stage +1, the multiplier is 3/2 — but because of integer truncation, a base stat of 1 at +1 yields `1 * 3 / 2 = 1` (not 1.5). Always multiply first, then divide.

### Gotcha #6: Damage formula minimum is 1, not 0

After all modifiers, if damage is 0, it is set to 1. The only exception is type immunity (Normal vs Ghost, etc.), which explicitly sets damage to 0. If you calculate damage as 0 from rounding, you must floor it at 1.

### Gotcha #7: Catch rate formula breaks for max HP > 341

The catch rate calculation uses `3 * MaxHP` in a single byte. For Pokemon with max HP > 341 (only possible at very high levels — Chansey, Blissey), `3 * MaxHP` overflows a 16-bit intermediate, corrupting the calculation. The game tries to prevent this with right-shifts when the high byte is non-zero, but the prevention is imperfect.

Source: `engine/items/item_effects.asm` line 311

### Gotcha #8: HP percentage calculation divides by zero for max HP < 4

The AI's switch-out text calculates HP as a percentage using `HP * 25 / (MaxHP / 4)`. If MaxHP < 4, the division by 4 truncates to 0, causing an infinite loop in `_Divide`. This freezes the game.

Source: `engine/battle/core.asm`, SendOutMonText

---

## Category 2: Order-of-Operations Traps

### Gotcha #9: Pre-move check order is strict and counter-intuitive

CheckTurn evaluates conditions in this exact order. Getting it wrong changes game behavior:
1. CANNOT_MOVE ($FF) check
2. Recharge (Hyper Beam)
3. Sleep (decrement, check Snore/Sleep Talk)
4. Freeze (check Flame Wheel/Sacred Fire thaw)
5. Flinch
6. Disable (decrement counter)
7. Confusion (decrement counter, 50% self-hit)
8. Attract (50% immobility)
9. Disabled move check
10. Paralysis (25% full paralysis)

**The trap:** Paralysis is checked LAST. A confused Pokemon rolls confusion first; if it passes, it still has a 25% chance of being fully paralyzed. A flinched Pokemon never gets to the confusion or paralysis check. Sleep is checked before freeze — a frozen+sleeping Pokemon (impossible normally, but possible via code) would process sleep, not freeze.

Source: `engine/battle/effect_commands.asm` lines 110-350

### Gotcha #10: Residual damage order within a turn

After each side moves, their residual damage is applied in this order:
1. Poison/Burn (1/8 max HP; Toxic: 1/16 * counter, incrementing)
2. Leech Seed (1/8 max HP drain to opponent)
3. Nightmare (1/4 max HP, only while asleep)
4. Curse (1/4 max HP)

Each has a faint check. If a Pokemon faints from poison, Leech Seed does NOT heal the opponent.

Source: `engine/battle/core.asm` lines 1005-1104

### Gotcha #11: Between-turn effects order

After BOTH sides have moved, HandleBetweenTurnEffects fires in this order (with faint checks between 1-4):
1. Future Sight countdown/damage
2. Sandstorm damage
3. Wrap/Bind/Fire Spin/Clamp damage
4. Perish Song countdown
5. Leftovers healing (NO faint check from here on)
6. Mystery Berry PP restoration
7. Defrost (random thaw — the ONLY non-move way to thaw)
8. Safeguard countdown
9. Light Screen/Reflect countdown
10. Stat-boosting held items
11. Healing held items
12. UpdateBattleMonInParty
13. Encore countdown

**The trap:** Leftovers healing happens AFTER Sandstorm damage, not before. A Pokemon at 1 HP takes Sandstorm damage and faints before Leftovers can save it.

Source: `engine/battle/core.asm` lines 250-296

### Gotcha #12: Destiny Bond clears at the START of the user's next turn

`EndUserDestinyBond` runs before DoPlayerTurn/DoEnemyTurn. If you set Destiny Bond on turn 1 and your opponent doesn't KO you, it's cleared at the start of turn 2 BEFORE you act. You don't get to "keep" Destiny Bond across turns.

Source: `engine/battle/core.asm` lines 869-875

### Gotcha #13: Protect/Endure clear after the OPPONENT'S move, not your own

`EndOpponentProtectEndureDestinyBond` runs after each side executes their move, clearing the opponent's Protect/Endure/Destiny Bond flags. This means Protect lasts exactly one attack from the opponent, as expected. But if the Protect user moves second, their Protect flag is still set during the opponent's attack AND only clears after their own move.

Source: `engine/battle/core.asm` lines 880-900

### Gotcha #14: Faint check order determines simultaneous KO winner

When both Pokemon faint on the same turn (Perish Song, Destiny Bond, recoil), the faint check order determines who "loses":
- If player moved first: `CheckFaint_PlayerThenEnemy` — player is checked first
- If enemy moved first: `CheckFaint_EnemyThenPlayer` — enemy is checked first

Whoever is checked first and found fainted is the "loser." This is unintuitive — the one who moved first gets checked first, meaning they LOSE simultaneous KO situations.

Source: `engine/battle/core.asm` lines 298-340

### Gotcha #15: Move priority special cases

Priority is NOT just a lookup table. Vital Throw is hardcoded to priority 0 BEFORE the priority table is checked (`cp VITAL_THROW` returns `a = 0` immediately). The table itself:
- Protect/Endure: 3
- Quick Attack/Mach Punch: 2
- Normal moves: 1 (implicit default)
- Counter/Mirror Coat/Roar/Whirlwind: 0

**Wiki misconception:** Many sources list Counter/Mirror Coat as priority -1. In Crystal, they're priority 0 — numerically below the default 1, so they go after normal moves, but this is via comparison, not a negative value.

Source: `data/moves/effects_priorities.asm`, `engine/battle/core.asm` lines 442-470

---

## Category 3: Type System Gotchas

### Gotcha #16: Physical/Special split is by TYPE, not by move

All moves of a given type are either physical or special. There is NO per-move split.

Physical types (const 0-9): Normal, Fighting, Flying, Poison, Ground, Rock, Bird (unused), Bug, Ghost, Steel
Special types (const 20-27): Fire, Water, Grass, Electric, Psychic, Ice, Dragon, Dark

The boundary: `DEF SPECIAL EQU const_value` at const 20. If `moveType >= SPECIAL`, the move is special. Otherwise physical.

**The trap:** Ghost and Dark do NOT share physical/special categorization despite both being "dark" thematically. Ghost is physical (const 8). Dark is special (const 27).

Source: `constants/type_constants.asm`

### Gotcha #17: CURSOR_TYPE (const 19) is between Physical and Special

The type constant `CURSE_TYPE` at index 19 sits between Steel (9) and Fire (20). Curse checks for Ghost type specifically — if the user is Ghost, it applies the curse effect; otherwise, it's the stat-modification version. Curse is typeless for damage purposes.

### Gotcha #18: Type chart uses a terminator-separated format

The type matchup table in `data/types/type_matchups.asm` is a flat list of `(attacker, defender, effectiveness)` triplets terminated by `$FE` (separating normal matchups from Foresight-enabled matchups) and `$FF` (end of table). The Foresight section re-enables Normal/Fighting vs Ghost matchups.

**The trap:** If you use a hashmap for type matchups, you miss the Foresight mechanic. The game literally has TWO sections of the same table — normal and Foresight-modified.

### Gotcha #19: Dragon Scale boosts Dragon moves, not Dragon Fang

Due to a data entry bug, `DRAGON_SCALE` has `HELD_DRAGON_BOOST` effect while `DRAGON_FANG` has `HELD_NONE`. The wrong item boosts Dragon-type damage.

Source: `data/items/attributes.asm`

---

## Category 4: 1/256 and Off-by-One Errors

### Gotcha #20: The 1/256 miss — 100% accuracy moves can miss

Accuracy check: `BattleRandom` returns 0-255, compared against the accuracy threshold. Since `cp threshold` sets carry if random < threshold, a random value of 255 always fails the check when the threshold is 255 (100%). This creates a 1/256 miss rate for all moves, including those with 100% accuracy.

Only Swift and moves used during Lock-On/Mind Reader bypass the accuracy check entirely.

### Gotcha #21: 100% secondary effect chance fails 1/256 of the time

Same principle: `BattleCommand_EffectChance` calls `BattleRandom` and compares against the effect chance. A 100% chance ($FF = 255) still fails when the random byte is 255, because `cp 255` with value 255 results in equal, not less-than.

Source: `engine/battle/effect_commands.asm`, BattleCommand_EffectChance

### Gotcha #22: Sleep counter is 1-7, but the Pokemon wakes WHEN the counter reaches 0

Sleep duration: `BattleRandom AND %111` gives 0-7. If the result is 0, re-roll (so effective range is 1-7). Each turn, the counter decrements BEFORE the sleep check. So a counter of 1 means the Pokemon wakes THIS turn (counter goes 1->0, wake up). A counter of 7 means 6 full turns of sleep plus waking on the 7th turn's check.

### Gotcha #23: Confusion counter is 2-5 turns, same decrement-then-check pattern

`BattleRandom AND %11` + 2 gives 2-5. Counter decrements, then if 0, confusion ends. So minimum 1 full turn confused, maximum 4 full turns.

### Gotcha #24: Protect success rate halves on consecutive use, but uses UNSIGNED comparison

The Protect/Detect success rate starts at 255/256 and halves with each consecutive use: 255, 127, 63, 31, 15, 7, 3, 1, 0. Once it reaches 0, Protect always fails. The counter is stored and persists until a non-Protect move is used, at which point it resets.

### Gotcha #25: PP Up bits are stored in the same byte as PP

The top 2 bits of each PP byte store how many PP Ups have been applied (0-3). The bottom 6 bits store current PP. When checking if a move has PP, you must mask with `PP_MASK` ($3F = 63). The bug: Disable's PP check uses `and a` instead of `and PP_MASK`, so a move with 0 PP but 1+ PP Ups (bits 6-7 set) passes the check, preventing Struggle.

Source: `engine/battle/core.asm`, CheckPlayerHasUsableMoves

---

## Category 5: Catch Rate Bugs

### Gotcha #26: BRN/PSN/PAR give +0 to catch rate, not +5

The catch rate status bonus code checks `and 1 << FRZ | SLP_MASK` for +10, then immediately does `and a` (which tests the SAME register A, which was already masked). Since A was already ANDed with `FRZ | SLP_MASK`, the second check only catches frozen/sleeping Pokemon that survived the first check — it never reaches the `ld c, 5` for BRN/PSN/PAR. All non-frozen, non-sleeping status conditions give +0 instead of the intended +5.

Source: `engine/items/item_effects.asm` lines 340-349

### Gotcha #27: Moon Ball checks EVOLVE_ITEM instead of MOON_STONE

The Moon Ball should boost catch rate for Pokemon that evolve with a Moon Stone. Instead, it checks whether the evolution method byte equals `EVOLVE_ITEM` (which is true for all stone evolutions), then reads the WRONG byte for the item check (reads the evolved species byte instead of the item byte due to wrong number of `inc hl` operations). Result: Moon Ball NEVER boosts catch rate.

Source: `engine/items/item_effects.asm`, MoonBallMultiplier (lines 897-936)

### Gotcha #28: Love Ball boosts for SAME gender instead of opposite

The Love Ball should boost catch rate when the player's Pokemon and the wild Pokemon are opposite genders. The comparison logic is inverted — it boosts when genders MATCH instead of when they differ.

Source: `engine/items/item_effects.asm`, LoveBallMultiplier

### Gotcha #29: Fast Ball only works for Magnemite, Grimer, and Tangela

The Fast Ball should boost catch rate for Pokemon with a flee flag. However, it reads from the `FleeMons` table using `GetFarByte` (reading one byte at a time) but the table stores 2-byte entries (species + flee rate). The `inc hl` after each read only advances by 1, so it reads: byte 0, byte 2, byte 4 — catching only the first 3 species' IDs while skipping the flee rate bytes. The first 3 species in FleeMons happen to be Magnemite, Grimer, and Tangela.

Source: `engine/items/item_effects.asm`, FastBallMultiplier (lines 1001-1026)

### Gotcha #30: HELD_CATCH_CHANCE has no effect

The catch rate code checks if the PLAYER's Pokemon holds an item with `HELD_CATCH_CHANCE` effect, but then immediately `pop de; ld a, d` which loads the catch rate back from the stack, overwriting whatever the held item check was supposed to modify. The held catch bonus is calculated but never applied.

Source: `engine/items/item_effects.asm` lines 357-370

---

## Category 6: Damage Calculation Pitfalls

### Gotcha #31: STAB and type effectiveness are MULTIPLICATION, not addition

The damage formula applies STAB (1.5x) and type effectiveness as multipliers, not additive bonuses. Type effectiveness is multiplicative across both defender types: super effective against both types = 4x, not 2x+2x.

### Gotcha #32: Critical hits IGNORE negative stat stages for the attacker AND positive stages for the defender

On a critical hit, if the attacker's stat stage is negative, it's treated as neutral. If the defender's stat stage is positive, it's treated as neutral. Neutral and positive attacker stages still apply. Neutral and negative defender stages still apply. This is NOT simply "ignore all stat changes."

### Gotcha #33: Damage variation range is 217-255, not 85%-100%

The random damage multiplier picks a byte from 217-255 (39 possible values), then multiplies and divides by 255. This gives a range of approximately 85.1%-100%, but the discrete steps are NOT evenly spaced percentages.

### Gotcha #34: Confusion self-hit damage uses the user's own Attack and Defense

Confusion damage is calculated as a 40-power typeless physical hit of the confused Pokemon against itself. It uses the user's Attack and Defense, applies Thick Club/Light Ball and type-boosting items (bug), and even doubles for Explosion/Self-Destruct (bug). A proper recreation should replicate these bugs.

Source: `engine/battle/effect_commands.asm`, HitSelfInConfusion

### Gotcha #35: Explosion and Self-Destruct halve the TARGET's Defense, not double the damage

The game implements the 2x damage of Explosion/Self-Destruct by halving the defender's Defense stat in the damage formula, not by doubling the power or final damage. This interacts with TruncateHL_BC — halving Defense means the Defense value entering truncation is smaller, potentially changing the truncation ratio.

### Gotcha #36: Badge boosts apply to base stats, stacking multiplicatively

The Johto badges boost stats by multiplying by 9/8 (1.125x). These are applied to the Pokemon's calculated stat, stacking with stat stages. Glacier Badge has a bug: it boosts Special Defense only if Special Attack's high byte is zero, because the boost routine for SpDef incorrectly checks the SpAtk register value.

Source: `engine/battle/core.asm`, BadgeStatBoosts

---

## Category 7: Move-Specific Gotchas

### Gotcha #37: Belly Drum boosts Attack BEFORE checking if HP is sufficient

Belly Drum calls `BattleCommand_AttackUp2` first, THEN checks if HP >= 50%. If HP < 50%, it fails and displays failure text — but the Attack boost already happened. The move costs no HP but still raises Attack by 2 stages (not max).

Source: `engine/battle/move_effects/belly_drum.asm`

### Gotcha #38: Berserk Gene confusion lasts 256 turns

`HandleBerserkGene` sets the confusion substatus flag but never initializes the confusion counter. The counter retains whatever value was in memory — either 0 (which wraps to 256 turns of confusion since the decrement happens before the zero-check) or the previous Pokemon's leftover confusion count.

### Gotcha #39: Counter and Mirror Coat work even when the opponent used an item

Counter/Mirror Coat check `wCurDamage` for the last damage dealt. If the opponent used an item instead of attacking, `wCurDamage` retains the value from the PREVIOUS turn's attack. Counter/Mirror Coat will reflect that old damage.

Source: `engine/battle/move_effects/counter.asm`

### Gotcha #40: Present damage is wrong in link battles

Crystal tried to fix Present's damage calculation but only applied the fix for non-link battles. In link battles, `push bc/push de` before `BattleCommand_Stab` and `pop de/pop bc` after are skipped, causing the bc/de registers (which hold the damage value) to be corrupted by the STAB calculation.

### Gotcha #41: Return/Frustration deal 0 damage at extreme happiness

Return's power = `happiness * 10 / 25`. At happiness 0-2, this rounds to 0. A 0-power move deals 1 damage minimum (Gotcha #6), but the game sets d=0 and the damage formula with power 0 produces 0 before the minimum check, so it actually deals 0. Same for Frustration at happiness 253-255.

### Gotcha #42: Moves that raise stats after damage don't trigger on KO

Move effect scripts run commands sequentially. For moves like AncientPower (AllUpHit), the `checkfaint` command runs BEFORE `allstatsup`. If the opponent faints, `checkfaint` ends the move effect, so the stat boost never happens.

Source: `data/moves/effects.asm`

### Gotcha #43: Defense-lowering moves can lower Defense after breaking a Substitute

Acid, Iron Tail, and Rock Smash call `effectchance` TWICE in their effect scripts — once before `hittarget` and once before the defense-lowering command. Even if the first hit breaks the Substitute, the second `effectchance` + defense-down still executes against the now-unprotected Pokemon.

---

## Category 8: Status and Volatile Condition Gotchas

### Gotcha #44: Toxic counter resets to regular poison on switch

When a Pokemon switches out, its toxic counter resets. If it switches back in, it has regular poison (1/8 max HP per turn), NOT toxic (scaling). Toxic counter only persists if the Pokemon stays in battle. Baton Pass does NOT pass toxic counter.

### Gotcha #45: Freeze can only be cured by specific means

Freeze ends ONLY by:
1. Being hit by a Fire-type move (any Fire move, even if it does 0 damage)
2. Using Flame Wheel or Sacred Fire (user thaws itself during CheckTurn)
3. Random 10% thaw chance during HandleBetweenTurnEffects step 7

There is NO duration counter. Freeze is permanent until one of these three events occurs. Many recreations incorrectly add a turn counter.

### Gotcha #46: Pursuit doubles damage specifically during switch-out

Pursuit's double-damage is not just "if opponent switches." The engine specifically calls `PursuitSwitch` during the switch sequence, which doubles Pursuit's damage and executes it BEFORE the switch completes. A Pokemon that faints from Pursuit during switching has its old status condition when revived (bug: the game doesn't properly update the fainted mon's status in the party).

### Gotcha #47: Lock-On and Mind Reader don't always bypass Fly/Dig

The `CheckHiddenOpponent` function checks `SUBSTATUS_FLYING | SUBSTATUS_UNDERGROUND` but doesn't first check if Lock-On/Mind Reader (`SUBSTATUS_LOCK_ON`) is active. Moves that call `CheckHiddenOpponent` (like Attract, Curse, Mean Look, stat-lowering moves) will miss against Fly/Dig even with Lock-On active. Only direct-damage accuracy checks properly handle Lock-On.

### Gotcha #48: Perish Song and Spikes can leave a Pokemon at 0 HP alive

When Perish Song KOs a Pokemon and the replacement takes Spikes damage that would faint it, the faint check after Spikes doesn't trigger properly. The replacement survives at 0 HP — alive but with no HP. This is because the faint-handling code only processes one faint per check cycle, and the Spikes damage occurs in a different code path that doesn't recheck.

---

## Category 9: AI and Trainer Gotchas

### Gotcha #49: AI scoring adds randomness, doesn't pick the "best" move

The AI doesn't calculate optimal moves. It starts each move with a score of 0, runs scoring layers that add/subtract from the score, then picks the LOWEST score (lower = better). Each AI layer (basic, setup, smart, cautious, risky, aggressive) modifies scores based on heuristics. Random variation is added so the AI isn't perfectly predictable.

**Common misconception:** The AI does NOT evaluate damage and pick the highest-damage move. It uses pattern-matching heuristics per move effect.

### Gotcha #50: RIVAL2 has lower DVs than RIVAL1

The rival's trainer class changes from `RIVAL1` to `RIVAL2` after a story event. `RIVAL2` has different (lower) DV values than `RIVAL1`, meaning the rival's Pokemon actually get WEAKER after the story progression. This is a data entry error, not intentional difficulty scaling.

Source: `data/trainers/dvs.asm`

---

## Category 10: Miscellaneous Critical Gotchas

### Bonus Gotcha #51: Sketch on a Transformed Pokemon checks the WRONG side

Sketch should fail if the USER is transformed (so you can't permanently learn a Transformed move). Instead, it checks if the OPPONENT is transformed (`BATTLE_VARS_SUBSTATUS5_OPP` instead of `BATTLE_VARS_SUBSTATUS5`). A Ditto that Transforms and then uses Sketch via Sleep Talk can permanently learn the Sketched move.

### Bonus Gotcha #52: Catching a Transformed Pokemon always catches Ditto

The catch code checks `SUBSTATUS_TRANSFORMED` and, if set, forces the species to DITTO regardless of what the Pokemon actually is. If a non-Ditto Pokemon was Transformed (via Transform move), catching it gives you a Ditto.

### Bonus Gotcha #53: wCurDamage is zeroed at turn start, not move start

`wCurDamage` is zeroed once at the beginning of BattleTurn, not before each move execution. This means the second mover's Counter/Mirror Coat can reflect the first mover's damage (intended), but if the first mover used a non-damaging move, wCurDamage is 0 from the turn-start reset, so Counter/Mirror Coat correctly fails.

### Bonus Gotcha #54: Experience gain has three independently stacking 1.5x multipliers

Experience formula: `(BaseExp * EnemyLevel) / 7`, then:
1. Traded Pokemon: x1.5 (different OT ID)
2. Trainer battle: x1.5 (wBattleMode != wild)
3. Lucky Egg: x1.5 (held item check)

All three stack multiplicatively: a traded Pokemon with Lucky Egg in a trainer battle gets `base * 1.5 * 1.5 * 1.5 = base * 3.375`. Each multiplication is done via `BoostExp` which multiplies the 24-bit `hQuotient` by 3/2 via shift-and-add.

Source: `engine/battle/core.asm` lines 7078-7107

### Bonus Gotcha #55: The game uses 8-bit math on a system with no hardware multiply

All multiplications go through a software `Multiply` routine using `hMultiplicand` (24-bit) and `hMultiplier` (8-bit). Divisions use `Divide` with `hDividend` (24-bit) and `hDivisor` (8-bit). Any formula that needs larger operands must be restructured to fit these constraints, which is why many calculations have surprising truncation points.
