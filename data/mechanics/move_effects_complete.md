# Pokemon Crystal -- Move Effects Complete Reference

Source: pokecrystal disassembly (engine/battle/move_effects/*.asm, engine/battle/effect_commands.asm)

---

## Overview

Each move has an **effect** (EFFECT_*) that determines which effect script (sequence of command bytes) is executed. Many effects are handled inline in `effect_commands.asm`; complex ones have dedicated files in `move_effects/`.

There are 58 dedicated move effect files. This document covers every one with exact implementation details.

---

## Status-Inflicting Moves

### Sleep (BattleCommand_SleepTarget)
- **File**: effect_commands.asm (inline)
- Fails if: target has status, target already asleep, substitute up
- Blocked by: HELD_PREVENT_SLEEP item
- Sleep duration: random 1-6 turns (`SLP_MASK & random`, reroll 0 and 7)
- **Battle Tower**: sleep capped at 3 turns max (`%011` mask)
- **AI handicap**: 25% chance of failure for AI (not in link/Battle Tower/Lock-On)
- After inflicting: calls UseHeldStatusHealingItem (Mint Berry etc.)

### Poison (BattleCommand_Poison / BattleCommand_PoisonTarget)
- **File**: effect_commands.asm (inline)
- Cannot poison Poison-types (CheckIfTargetIsPoisonType checks both type slots)
- Fails if: target has any status, substitute up
- Blocked by: HELD_PREVENT_POISON, Safeguard
- **Toxic variant** (EFFECT_TOXIC): sets SUBSTATUS_TOXIC and resets toxic counter to 0
- **AI handicap**: 25% chance of failure for AI (same conditions as sleep)

### Burn (BattleCommand_BurnTarget)
- **File**: effect_commands.asm (inline)
- Cannot burn Fire-types (CheckMoveTypeMatchesTarget)
- Fails if: target has any status, substitute up
- Blocked by: HELD_PREVENT_BURN, Safeguard
- If target is frozen: thaws them instead of burning (Defrost)
- After burning: applies burn attack reduction (ApplyBrnEffectOnAttack)

### Freeze (BattleCommand_FreezeTarget)
- **File**: effect_commands.asm (inline)
- Cannot freeze Ice-types
- Cannot freeze in sun weather (WEATHER_SUN)
- Fails if: target has any status, substitute up
- Blocked by: HELD_PREVENT_FREEZE, Safeguard
- After freezing: opponent can't move this turn, ends recharge

### Paralyze (BattleCommand_ParalyzeTarget)
- **File**: effect_commands.asm (inline)
- Fails if: target has any status, substitute up
- Blocked by: HELD_PREVENT_PARALYZE, Safeguard
- After paralyzing: applies speed reduction (ApplyPrzEffectOnSpeed)

### Tri-Attack (BattleCommand_TriStatusChance)
- **File**: effect_commands.asm (inline)
- 1/3 chance each of: paralyze, freeze, burn
- Uses EffectChance first (secondary effect chance)

### Confuse (BattleCommand_Confuse -- via effect script)
- Confusion counter: random 2-5 turns
- 50% chance of self-hit per turn while confused
- Self-hit: 40 power, typeless, physical, uses own Attack/Defense

---

## Stat Modification Moves

### Stat Up (BattleCommand_StatUp / RaiseStat)
- **File**: effect_commands.asm (inline)
- 7 stats: Attack, Defense, Speed, SpAtk, SpDef, Accuracy, Evasion
- Each has single (+1) and double (+2) variants
- Stat levels range from 1 to 13 (base = 7)
- Multipliers: level 1 = 25/100, level 7 = 100/100, level 13 = 400/100
- Max stat value: 999
- Fails if: already at max level or stat already at 999
- **Minimize**: special handling -- sets wPlayerMinimized/wEnemyMinimized flag

### Stat Down (BattleCommand_StatDown / LowerStat)
- **File**: effect_commands.asm (inline)
- Blocked by: Mist (SUBSTATUS_MIST) -- but only for pure stat-down effects
- CheckMist: only triggers for EFFECT_ATTACK_DOWN through EFFECT_EVASION_DOWN, _DOWN_2 variants, and _DOWN_HIT variants
- **AI handicap**: 25% chance of failure for AI (not in link/Battle Tower/Lock-On). Exception: EFFECT_ACCURACY_DOWN_HIT bypasses this
- Fails if: already at minimum, substitute up, target hidden (Fly/Dig)

### All Stats Up (BattleCommand_AllStatsUp -- Ancientpower)
- Raises Attack, Defense, Speed, SpAtk, SpDef each by 1 stage
- Each stat raised independently (one failing doesn't stop others)

### Swagger
- **File**: effect_commands.asm (via Confuse + AttackUp2)
- Raises target's Attack by 2, then confuses them
- AI_Redundant: discouraged if target already confused

### Psych Up (BattleCommand_PsychUp)
- **File**: move_effects/psych_up.asm
- Copies all 7 stat levels from opponent to user
- Fails if opponent has no modified stats (all at base level 7)
- Recalculates stats after copying

---

## Damage Calculation Moves

### Return (BattleCommand_HappinessPower)
- **File**: move_effects/return.asm
- Power = happiness * 10 / 25
- **BUG**: deals 0 damage at 0 happiness (since power becomes 0)

### Frustration (BattleCommand_FrustrationPower)
- **File**: move_effects/frustration.asm
- Power = (255 - happiness) * 10 / 25
- **BUG**: deals 0 damage at 255 happiness

### Hidden Power (BattleCommand_HiddenPower)
- **File**: move_effects/hidden_power.asm (109 lines)
- Type = ((DefDV & 3) + ((AtkDV & 3) << 2)) + 1, skipping BIRD and unused types
- Power = (((AtkDV>>3)<<3 | (DefDV>>3)<<2 | (SpdDV>>3)<<1 | (SpcDV>>3)) * 5 + (SpcDV & 3)) / 2 + 31
- Range: 31-70 power

### Reversal / Flail (EFFECT_REVERSAL)
- **File**: effect_commands.asm (BattleCommand_ConstantDamage)
- Power based on current HP / max HP ratio (uses 48x multiplier):
  - <=1: 200 power
  - <=4: 150
  - <=9: 100
  - <=16: 80
  - <=32: 40
  - else: 20
- Uses FlailReversalPower lookup table

### Magnitude (BattleCommand_GetMagnitude)
- **File**: move_effects/magnitude.asm
- Random power from MagnitudePower table:
  - Magnitude 4: power 10 (5% chance, random < 13)
  - Magnitude 5: power 30 (10%, random < 38)
  - Magnitude 6: power 50 (20%, random < 89)
  - Magnitude 7: power 70 (30%, random < 166)
  - Magnitude 8: power 90 (20%, random < 217)
  - Magnitude 9: power 110 (10%, random < 242)
  - Magnitude 10: power 150 (5%, random < 256)

### Present (BattleCommand_Present)
- **File**: move_effects/present.asm
- Random outcome from PresentPower table:
  - 40 power (40% chance)
  - 80 power (30%)
  - 120 power (10%)
  - Heal opponent 1/4 max HP (20%)
- **BUG**: damage incorrect in link battles (uses wrong damage variable)

### Psywave (EFFECT_PSYWAVE)
- **File**: effect_commands.asm (inline in BattleCommand_ConstantDamage)
- Damage = random(1 to level*1.5)

### Super Fang (EFFECT_SUPER_FANG)
- **File**: effect_commands.asm (inline)
- Damage = target's current HP / 2 (minimum 1)

### Level Damage (EFFECT_LEVEL_DAMAGE -- Night Shade / Seismic Toss)
- **File**: effect_commands.asm (inline)
- Damage = user's level (exactly)

---

## Multi-Hit and Multi-Turn Moves

### Rollout (BattleCommand_Rollout)
- **File**: move_effects/rollout.asm (91 lines)
- 5-turn sequence, damage doubles each hit
- Defense Curl (SUBSTATUS_CURLED) doubles base damage further
- Resets on miss

### Fury Cutter (BattleCommand_FuryCutter)
- **File**: move_effects/fury_cutter.asm (54 lines)
- Consecutive hit counter increments each successful hit
- Damage doubles each hit, capped at 5 turns (16x)
- Resets on miss (ResetFuryCutterCount)

### Triple Kick (BattleCommand_TripleKick / KickCounter)
- **File**: move_effects/triple_kick.asm (31 lines)
- 3 hits, each adds base damage again (1x, 2x, 3x cumulative)
- wBattleAnimParam tracks kick count
- Overflow caps at 65535

### Rampage (BattleCommand_Rampage -- Outrage/Thrash/Petal Dance)
- **File**: effect_commands.asm (inline)
- Duration: 2-3 turns (random 1-2 more turns after first)
- Cannot rampage during Sleep Talk
- After rampage ends: confused for 2-3 turns (unless Safeguard active)
- Sets wSomeoneIsRampaging flag

### Bide (BattleCommand_Bide)
- **File**: move_effects/bide.asm (96 lines)
- Stores damage taken over 2-3 turns
- Unleashes at double the accumulated damage
- Uses SUBSTATUS_BIDE flag

---

## Healing Moves

### Heal / Recovery (BattleCommand_Heal -- Recover/Milk Drink/Softboiled)
- **File**: effect_commands.asm (inline, via SapHealth/RestoreHP)
- Heals 1/2 max HP
- Fails at full HP (AI_Redundant checks this)

### Moonlight / Synthesis / Morning Sun
- **File**: effect_commands.asm (inline)
- Weather-dependent healing:
  - Sun: 2/3 max HP
  - Clear: 1/2 max HP
  - Rain/Sandstorm: 1/4 max HP
- AI_Redundant: discouraged at full HP

### Rest (EFFECT_HEAL -- special case)
- Fully heals HP and cures status
- Puts user to sleep for exactly 2 turns

### Pain Split (BattleCommand_PainSplit)
- **File**: move_effects/pain_split.asm (91 lines)
- Averages both Pokemon's current HP
- Each Pokemon's HP capped at its own max HP
- Does not fail against Ghost types (it's not a Normal move targeting)

### Drain Moves (BattleCommand_DrainTarget / BattleCommand_EatDream)
- **File**: effect_commands.asm (inline, SapHealth)
- Absorb/Mega Drain/Giga Drain: heal 1/2 of damage dealt
- Dream Eater: heal 1/2 of damage dealt (only works on sleeping targets)
- Minimum drain: 1 HP
- **Note**: cannot drain through Substitute (CheckHit blocks it)

---

## Protection Moves

### Protect / Detect (BattleCommand_Protect)
- **File**: move_effects/protect.asm (75 lines)
- Shared ProtectChance function (also used by Endure)
- Success rate halves with consecutive use: 255/256, 127/256, 63/256...
- Fails if: user went first (must go second), user has Substitute
- Counter resets on any non-Protect/Detect/Endure move
- Random check: generate random 1-255, must be < success threshold

### Endure (BattleCommand_Endure)
- **File**: move_effects/endure.asm (15 lines)
- Uses same ProtectChance as Protect
- Sets SUBSTATUS_ENDURE -- survives with 1 HP
- ApplyDamage checks Endure before Focus Band

### Safeguard (BattleCommand_Safeguard)
- **File**: move_effects/safeguard.asm (22 lines)
- Sets SCREENS_SAFEGUARD flag, lasts 5 turns
- Prevents status conditions on user's side
- Cannot stack (fails if already active)

---

## Field Effect Moves

### Spikes (BattleCommand_Spikes)
- **File**: move_effects/spikes.asm (25 lines)
- Sets SCREENS_SPIKES on opponent's side
- Only 1 layer in Gen 2
- Fails if spikes already down
- Deals 1/8 max HP on switch-in

### Rapid Spin (BattleCommand_ClearHazards)
- **File**: move_effects/rapid_spin.asm (35 lines)
- Clears from user: Leech Seed, Spikes, Wrap/Bind
- Each removal has its own text message

### Leech Seed (BattleCommand_LeechSeed)
- **File**: move_effects/leech_seed.asm (40 lines)
- Cannot seed Grass-types (checks both type slots)
- Fails if: target already seeded, substitute up, missed
- Drains 1/8 max HP per turn, heals opponent

### Weather Moves
- **Rain Dance** (move_effects/rain_dance.asm, 9 lines): sets WEATHER_RAIN, 5 turns
- **Sunny Day** (move_effects/sunny_day.asm, 9 lines): sets WEATHER_SUN, 5 turns
- **Sandstorm** (move_effects/sandstorm.asm, 17 lines): sets WEATHER_SANDSTORM, 5 turns. Fails if sandstorm already active
- Rain/Sun don't check for existing weather (always succeed)

---

## Screen Moves

### Reflect
- Sets SCREENS_REFLECT, doubles physical defense in damage calc
- Lasts 5 turns (HandleScreens decrements)
- Cannot stack

### Light Screen
- Sets SCREENS_LIGHT_SCREEN, doubles special defense in damage calc
- Lasts 5 turns
- Cannot stack

---

## Status Utility Moves

### Attract (BattleCommand_Attract)
- **File**: move_effects/attract.asm (76 lines)
- Requires opposite genders (CheckOppositeGender)
- Gender determined from DVs via GetGender
- Transformed Pokemon use backup DVs
- Fails if: missed, same gender, genderless, target already in love, target hidden
- 50% chance of immobilization per turn

### Nightmare (BattleCommand_Nightmare)
- **File**: move_effects/nightmare.asm (36 lines)
- Target must be sleeping
- Fails if: target not asleep, already nightmared, substitute up, target hidden
- Deals 1/4 max HP per turn in ResidualDamage

### Disable (BattleCommand_Disable)
- **File**: move_effects/disable.asm (71 lines)
- Disables opponent's last used move
- Duration: random 1-7 turns (random & 7, reroll 0, then +1)
- Stored as: (move_index << 4) | duration
- Fails if: missed, no last move, last move is Struggle, move has 0 PP, already disabled

### Encore (BattleCommand_Encore)
- **File**: move_effects/encore.asm (119 lines)
- Forces opponent to repeat their last move
- Duration: random 3-6 turns (random & 3, then +3)
- Fails if: no last move, last move is Struggle/Encore/Mirror Move, move has 0 PP, already encored
- If user went first: immediately forces opponent's current move selection

### Spite (BattleCommand_Spite)
- **File**: move_effects/spite.asm (85 lines)
- Reduces PP of opponent's last used move by 2-5 (random & 3, +2)
- Respects PP_MASK (doesn't affect PP-Up bits)
- Updates party struct (unless transformed or wild)
- Fails if: missed, no last move, Struggle, 0 PP

### Mean Look / Spider Web (EFFECT_MEAN_LOOK)
- Sets SUBSTATUS_CANT_RUN on opponent
- Prevents switching and fleeing

### Foresight / Odor Sleuth (BattleCommand_Foresight)
- **File**: move_effects/foresight.asm (21 lines)
- Sets SUBSTATUS_IDENTIFIED on target
- Removes Normal/Fighting immunity from Ghost types
- Also resets evasion advantage (if target evasion > user accuracy, Foresight bypasses)

### Lock-On / Mind Reader (BattleCommand_LockOn)
- **File**: move_effects/lock_on.asm (20 lines)
- Sets SUBSTATUS_LOCK_ON on target
- Next move guaranteed to hit (unless flying + ground move)
- Consumed after one turn

### Perish Song (BattleCommand_PerishSong)
- **File**: move_effects/perish_song.asm (37 lines)
- Affects BOTH sides (sets SUBSTATUS_PERISH on each)
- Counter starts at 4, decremented each turn in HandlePerishSong
- Faints at 0
- Skips a side if already under Perish Song
- Fails only if both sides already have Perish Song active

### Destiny Bond (BattleCommand_DestinyBond)
- **File**: move_effects/destiny_bond.asm (8 lines)
- Sets SUBSTATUS_DESTINY_BOND on user
- If user faints from damage, attacker also faints
- Cleared at end of turn (EndOpponentProtectEndureDestinyBond)

---

## Move Copying / Transformation Moves

### Transform (BattleCommand_Transform)
- **File**: move_effects/transform.asm (151 lines)
- Copies: species, types, moves (all PP set to 5, Sketch gets 1), DVs, stats, stat levels
- Sets SUBSTATUS_TRANSFORMED
- Fails if: user already transformed, target has substitute

### Mimic (BattleCommand_Mimic)
- **File**: move_effects/mimic.asm (49 lines)
- Replaces Mimic with opponent's last used move (temporary, battle only)
- Sets PP to 5
- Fails if: missed, no last move, Struggle, already knows the move, target hidden

### Sketch (BattleCommand_Sketch)
- **File**: move_effects/sketch.asm (117 lines)
- **Permanently** replaces Sketch with opponent's last used move
- Copies base PP from move data
- Updates party struct (permanent change)
- Fails in link battles (always fails)
- Fails if: target has substitute, target transformed, no last move, Struggle, already knows move
- **BUG**: Transformed Pokemon can Sketch and learn unobtainable moves

### Mirror Move (BattleCommand_MirrorMove)
- **File**: move_effects/mirror_move.asm (50 lines)
- Uses opponent's last counter move
- Fails if no last move or user already knows it
- Calls ResetTurn to execute the copied move

### Metronome (BattleCommand_Metronome)
- **File**: move_effects/metronome.asm (42 lines)
- Picks random move (1 to NUM_ATTACKS)
- Excludes: MetronomeExcepts list (Metronome, Struggle, Sketch, Mimic, Sleep Talk, Counter, Mirror Coat, Protect, Detect, Endure, Destiny Bond, Thief)
- Also excludes moves the user already knows
- Calls ResetTurn to execute

### Sleep Talk (BattleCommand_SleepTalk)
- **File**: move_effects/sleep_talk.asm (142 lines)
- Must be asleep to use
- Randomly selects one of user's other moves (not Sleep Talk itself, not disabled move)
- Excludes two-turn moves: Skull Bash, Razor Wind, Sky Attack, SolarBeam, Fly, Bide
- Fails if: not asleep, only has Sleep Talk, all other moves are two-turn or disabled

### Snore (BattleCommand_Snore)
- **File**: move_effects/snore.asm (11 lines)
- Must be asleep to use -- if not asleep, resets damage and fails
- If asleep: executes normally as a damaging move with 30% flinch chance

---

## Item/HP Interaction Moves

### Thief (BattleCommand_Thief)
- **File**: move_effects/thief.asm (111 lines)
- Steals opponent's held item after dealing damage
- Conditions: user has no item, opponent has an item, item is not Mail
- In link battles: cannot steal from wild Pokemon
- Clears item from both battle struct and party struct
- Enemy stealing: item is permanently lost

### Pay Day (BattleCommand_PayDay)
- **File**: move_effects/pay_day.asm (25 lines)
- Scatters coins equal to 2x user's level
- Added to wPayDayMoney (3-byte counter, max 16M)

### Belly Drum (BattleCommand_BellyDrum)
- **File**: move_effects/belly_drum.asm (31 lines)
- Costs 50% max HP, maxes Attack to +6
- **BUG**: raises attack even if under 50% HP (calls AttackUp2 before HP check succeeds, so the stat boost applies even when HP is insufficient and the move "fails")

### Substitute (BattleCommand_Substitute)
- **File**: move_effects/substitute.asm (86 lines)
- Costs 1/4 max HP (fails if HP too low or at exactly 1/4)
- Creates substitute with HP = 1/4 user's max HP
- Clears wrap/trap effects
- Sets SUBSTATUS_SUBSTITUTE

---

## Switching/Fleeing Moves

### Baton Pass (BattleCommand_BatonPass)
- **File**: move_effects/baton_pass.asm (215 lines)
- **Passes**: stat levels, confusion, Focus Energy, Substitute (with HP), Mean Look trap, Perish Song counter
- **Clears**: Nightmare (if not asleep), Disable, Attraction, Transform, Encore, wrap counts, last move
- ResetBatonPassStatus handles the clearing

### Teleport / Whirlwind / Roar
- **Teleport** (move_effects/teleport.asm, 95 lines): flee from wild battle. Level-based success check. Cannot flee from: shiny/trap/Celebi/Suicune battles, Mean Look. **BUG**: wild Pokemon always succeed regardless of level
- **ForceSwitch** (effect_commands.asm): Roar/Whirlwind force random switch. Same flee restrictions. Level-based success in wild battles

### Pursuit (BattleCommand_Pursuit)
- **File**: move_effects/pursuit.asm (24 lines)
- Doubles damage if opponent is switching (wEnemyIsSwitching/wPlayerIsSwitching)
- Simple left-shift of wCurDamage, overflow caps at 65535

---

## Self-Sacrifice Moves

### Selfdestruct / Explosion (BattleCommand_Selfdestruct)
- **File**: move_effects/selfdestruct.asm (30 lines)
- Sets user's HP to 0, clears status
- Clears Leech Seed from user
- Clears Destiny Bond from opponent
- Defense halved in damage calc (EFFECT_SELFDESTRUCT check in DamageCalc)

### Counter (BattleCommand_Counter)
- **File**: move_effects/counter.asm (57 lines)
- Returns double the physical damage received
- Checks last counter move type < SPECIAL (physical)
- **BUG**: triggers even if opponent used an item (not a move)

### Mirror Coat (BattleCommand_MirrorCoat)
- **File**: move_effects/mirror_coat.asm (58 lines)
- Returns double the special damage received
- Checks last counter move type >= SPECIAL
- **BUG**: triggers even if opponent used an item

### Future Sight (BattleCommand_FutureSight)
- **File**: move_effects/future_sight.asm (77 lines)
- Sets 4-turn countdown (wPlayerFutureSightCount/wEnemyFutureSightCount)
- Stores calculated damage
- Delivered in HandleFutureSight when counter reaches 1
- Ignores type effectiveness on delivery (EFFECTIVE forced)
- **BUG**: AI checks SCREENS_UNUSED instead of Future Sight count for redundancy

---

## Miscellaneous Moves

### Conversion (BattleCommand_Conversion)
- **File**: move_effects/conversion.asm (95 lines)
- Changes user's type to match one of its moves' types
- Randomly selects from available move types
- Excludes CURSE_TYPE and types user already has
- Fails if no valid type available

### Conversion2 (BattleCommand_Conversion2)
- **File**: move_effects/conversion2.asm (63 lines)
- Changes user's type to resist opponent's last used move
- Randomly generates types until finding one that takes < 1x damage
- Excludes unused types
- Fails if: missed, no last move, last move was CURSE_TYPE

### Focus Energy (BattleCommand_FocusEnergy)
- **File**: move_effects/focus_energy.asm (14 lines)
- Sets SUBSTATUS_FOCUS_ENERGY (+1 critical hit level)
- Fails if already active

### Mist (BattleCommand_Mist)
- **File**: move_effects/mist.asm (14 lines)
- Sets SUBSTATUS_MIST
- Prevents opponent from lowering user's stats
- Fails if already active

### Rage (BattleCommand_Rage)
- **File**: move_effects/rage.asm (6 lines)
- Sets SUBSTATUS_RAGE
- wRageCounter increments each time user is hit (BattleCommand_BuildOpponentRage)
- Damage multiplied by rage counter (BattleCommand_RageDamage)

### Splash (BattleCommand_Splash)
- **File**: move_effects/splash.asm (5 lines)
- Does nothing. Prints "But nothing happened!"

### False Swipe (BattleCommand_FalseSwipe)
- **File**: move_effects/false_swipe.asm (47 lines)
- Caps damage so target always survives with at least 1 HP
- Cancels critical hit if it would KO

### Heal Bell (BattleCommand_HealBell)
- **File**: move_effects/heal_bell.asm (33 lines)
- Cures status of ALL party members (not just active)
- Clears Nightmare from active Pokemon
- Recalculates stats (removes burn/paralysis penalties)

### Curse (BattleCommand_Curse)
- **File**: move_effects/curse.asm (91 lines)
- **Ghost-type user**: costs 50% max HP, sets SUBSTATUS_CURSE on target (1/4 HP damage per turn)
- **Non-Ghost user**: +1 Attack, +1 Defense, -1 Speed

### Defrost (BattleCommand_DefrostOpponent -- Flame Wheel/Sacred Fire)
- **File**: effect_commands.asm (inline)
- Thaws frozen opponent as secondary effect
- Also raises user's Attack by 1 stage

### Thunder Accuracy (BattleCommand_ThunderAccuracy)
- **File**: move_effects/thunder.asm (17 lines)
- Rain: 100% accuracy (also guaranteed hit via ThunderRain in CheckHit)
- Sun: 50% accuracy
- Clear: normal 70% accuracy

### Curl (BattleCommand_Curl -- Defense Curl)
- **File**: effect_commands.asm (inline, 4 lines)
- Sets SUBSTATUS_CURLED (doubles Rollout/Ice Ball damage)

---

## Damage Calculation Details

### Standard Damage Formula (BattleCommand_DamageCalc)
```
damage = ((2 * level / 5 + 2) * power * attack / defense / 50) + 2
```
- Critical hit: doubles the quotient (before +2)
- Item boost: type-matching held items multiply by (100 + item_effect) / 100
- Selfdestruct/Explosion: defense halved
- Max damage: 997 (before +2 = 999)
- Min damage: 2

### Damage Stats (BattleCommand_DamageStats)
- Physical (type < SPECIAL): uses Attack vs Defense
- Special (type >= SPECIAL): uses SpAtk vs SpDef
- Reflect: doubles physical defense
- Light Screen: doubles special defense
- Critical hits: ignore stat stages if they disadvantage the attacker
- **Thick Club**: doubles Attack for Cubone/Marowak
- **Light Ball**: doubles SpAtk for Pikachu
- **Metal Powder**: 1.5x defense for Ditto opponent
- **BUG**: Thick Club/Light Ball can wrap above 1024
- **BUG**: Reflect/Light Screen can wrap defense above 1024
- **BUG**: Metal Powder can increase damage (adds to wrong stat in some cases)

### Damage Variation (BattleCommand_DamageVariation)
- Multiplies by random 85-100%
- Only applies if damage >= 2
- Distribution is slightly non-uniform due to integer division

### STAB (BattleCommand_Stab)
- 1.5x if move type matches either of user's types
- Type effectiveness: uses TypeMatchups table
  - 0x: immune (sets AttackMissed)
  - 5/10: not very effective (0.5x)
  - 10/10: neutral (1x)
  - 20/10: super effective (2x)
- Foresight: skips the "after -2 delimiter" section of type matchups (removes Ghost immunities)
- Weather modifiers applied via DoWeatherModifiers
- Badge type boosts applied via DoBadgeTypeBoosts (player only, not in link/Battle Tower)

---

## Known Bugs Summary

1. **Belly Drum**: boosts Attack even under 50% HP
2. **Return/Frustration**: 0 damage at extreme happiness values
3. **Counter/Mirror Coat**: triggers when opponent used item instead of move
4. **Present**: incorrect damage in link battles
5. **Sketch**: Transformed Pokemon can learn unobtainable moves
6. **Beat Up**: may fail to raise Substitute, can desync link battles, may trigger King's Rock on failure
7. **Teleport**: wild Pokemon always succeed regardless of level difference
8. **Future Sight AI**: checks wrong flag (SCREENS_UNUSED) for redundancy
9. **Thick Club/Light Ball**: attack can wrap above 1024
10. **Reflect/Light Screen**: defense can wrap above 1024
11. **Metal Powder**: can increase damage taken in some cases
12. **Confusion damage**: affected by type-boosting items and Explosion defense halving
13. **EffectChance**: 100% secondary effects fail in 1/256 uses
14. **Berserk Gene**: confusion lasts 256 turns or inherits previous count
