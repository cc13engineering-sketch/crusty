# Pokemon Crystal -- Effect Commands Reference

Source: pokecrystal disassembly (engine/battle/effect_commands.asm, 6780 lines)

---

## Overview

Each move has an **effect script** -- a sequence of **command bytes** stored in MoveEffectsPointers. When a move executes, DoMove copies the script to wBattleScriptBuffer and processes commands sequentially via the BattleCommandPointers dispatch table.

Each command byte maps to a BattleCommand_* function. The script ends with `endmove` commands.

---

## Standard Damaging Move Script

A basic damaging move (e.g., Tackle, Surf) uses this command sequence:

```
checkobedience    ; obedience check (traded Pokemon)
usedmovetext      ; print "[Pokemon] used [Move]!"
doturn            ; consume PP, check continuous moves
critical          ; determine critical hit
damagestats       ; get attack/defense stats for damage calc
damagecalc        ; compute damage value
stab              ; apply STAB + type effectiveness + weather + badges
damagevariation   ; random 85-100% multiplier
checkhit          ; accuracy/evasion check
hittarget         ; apply damage, animate, show HP bar
failuretext       ; print miss/fail text if applicable
checkfaint        ; check if target fainted (+ Destiny Bond)
criticaltext      ; print "Critical hit!" or "One-hit KO!"
supereffectivetext ; print effectiveness message
checkfaint        ; second faint check
buildopponentrage ; increment opponent's Rage counter if active
endmove           ; end the effect script
```

---

## Complete Command Reference

### Flow Control Commands

| Command | Function | Description |
|---------|----------|-------------|
| `endmove` | EndMoveEffect | Terminates the effect script by writing endmove bytes |
| `endturn` | (end of turn marker) | Marks end of a turn for two-turn moves |
| `checkturn` | DoTurn | Consumes PP, handles continuous move logic |
| `checkobedience` | CheckObedience | Obedience check for traded Pokemon |
| `charge` | BattleCommand_Charge | Charge turn (Fly, Dig, SolarBeam, etc.) |
| `checkcharge` | BattleCommand_CheckCharge | Check if charge turn is complete |
| `rampage` | BattleCommand_Rampage | Start rampage (Outrage, Thrash) |
| `checkrampage` | BattleCommand_CheckRampage | Check if rampage continues |
| `startloop` | BattleCommand_StartLoop | Initialize multi-hit loop counter |
| `endloop` | BattleCommand_EndLoop | Loop back to `critical` for multi-hit |

### Damage Calculation Commands

| Command | Function | Description |
|---------|----------|-------------|
| `critical` | BattleCommand_Critical | Determine if this hit is critical |
| `damagestats` | BattleCommand_DamageStats | Load attack/defense/level for damage calc |
| `damagecalc` | BattleCommand_DamageCalc | Compute damage formula |
| `stab` | BattleCommand_Stab | Apply STAB, type effectiveness, weather, badges |
| `damagevariation` | BattleCommand_DamageVariation | Random 85-100% damage multiplier |
| `constantdamage` | BattleCommand_ConstantDamage | Fixed damage (Seismic Toss, Psywave, Super Fang, Reversal) |
| `clearmissdamage` | BattleCommand_ClearMissDamage | Reset damage to 0 if missed |
| `ragedamage` | BattleCommand_RageDamage | Multiply damage by Rage counter |

### Hit/Miss Commands

| Command | Function | Description |
|---------|----------|-------------|
| `checkhit` | BattleCommand_CheckHit | Full accuracy check (Dream Eater, Protect, Lock-On, Fly/Dig, Thunder/Rain, X Accuracy, stat modifiers, BrightPowder) |
| `effectchance` | BattleCommand_EffectChance | Secondary effect chance check (vs Substitute, move's CHANCE field) |
| `checksafeguard` | BattleCommand_CheckSafeguard | Fail if target has Safeguard |
| `resettypematchup` | BattleCommand_ResetTypeMatchup | Reset type multiplier to 1.0 (for moves that ignore type) |

### Damage Application Commands

| Command | Function | Description |
|---------|----------|-------------|
| `hittarget` | BattleCommand_ApplyDamage | Apply damage, handle Endure/Focus Band, update HP bars |
| `kingsrock` | BattleCommand_HeldFlinch | King's Rock flinch check |
| `recoil` | BattleCommand_Recoil | Apply 1/4 recoil damage to user |
| `pursuit` | BattleCommand_Pursuit | Double damage if opponent switching |
| `falseswipe` | BattleCommand_FalseSwipe | Cap damage to leave 1 HP |
| `selfdestruct` | BattleCommand_Selfdestruct | Set user HP to 0, animate |
| `doubleflydamage` | BattleCommand_DoubleFlyingDamage | Double damage vs flying target |
| `doubleundergrounddamage` | BattleCommand_DoubleUndergroundDamage | Double damage vs underground target |
| `doubleminimizedamage` | BattleCommand_DoubleMinimizeDamage | Double damage vs Minimized target |

### Status Infliction Commands

| Command | Function | Description |
|---------|----------|-------------|
| `sleeptarget` | BattleCommand_SleepTarget | Inflict sleep (checks items, Safeguard, AI handicap) |
| `poisontarget` | BattleCommand_PoisonTarget | Inflict poison (secondary effect, checks type/Safeguard) |
| `poison` | BattleCommand_Poison | Inflict poison/toxic (primary move effect) |
| `burntarget` | BattleCommand_BurnTarget | Inflict burn (checks Fire type, defrosts) |
| `freezetarget` | BattleCommand_FreezeTarget | Inflict freeze (checks Ice type, sun weather) |
| `paralyzetarget` | BattleCommand_ParalyzeTarget | Inflict paralysis (secondary effect) |
| `paralyze` | BattleCommand_Paralyze | Inflict paralysis (primary move -- Thunder Wave, Stun Spore) |
| `tristatuseffect` | BattleCommand_TriStatusChance | 1/3 each: paralyze/freeze/burn (Tri Attack) |
| `confusetarget` | BattleCommand_ConfuseTarget | Inflict confusion (secondary effect) |
| `confuse` | BattleCommand_Confuse | Inflict confusion (primary move) |
| `flinchtarget` | BattleCommand_FlinchTarget | Inflict flinch |
| `fakeout` | BattleCommand_FakeOut | Flinch only if user moved first (Fake Out) |
| `traptarget` | BattleCommand_TrapTarget | Trap for 3-6 turns (Bind, Wrap, Fire Spin, Whirlpool, Clamp) |

### Stat Modification Commands

| Command | Function | Description |
|---------|----------|-------------|
| `attackup` | BattleCommand_AttackUp | +1 Attack |
| `defenseup` | BattleCommand_DefenseUp | +1 Defense |
| `speedup` | BattleCommand_SpeedUp | +1 Speed |
| `specialattackup` | BattleCommand_SpecialAttackUp | +1 SpAtk |
| `specialdefenseup` | BattleCommand_SpecialDefenseUp | +1 SpDef |
| `accuracyup` | BattleCommand_AccuracyUp | +1 Accuracy |
| `evasionup` | BattleCommand_EvasionUp | +1 Evasion |
| `attackup2` | BattleCommand_AttackUp2 | +2 Attack |
| `defenseup2` | BattleCommand_DefenseUp2 | +2 Defense |
| `speedup2` | BattleCommand_SpeedUp2 | +2 Speed |
| `specialattackup2` | BattleCommand_SpecialAttackUp2 | +2 SpAtk |
| `specialdefenseup2` | BattleCommand_SpecialDefenseUp2 | +2 SpDef |
| `accuracyup2` | BattleCommand_AccuracyUp2 | +2 Accuracy |
| `evasionup2` | BattleCommand_EvasionUp2 | +2 Evasion |
| `attackdown` | BattleCommand_AttackDown | -1 Attack (opponent) |
| `defensedown` | BattleCommand_DefenseDown | -1 Defense |
| `speeddown` | BattleCommand_SpeedDown | -1 Speed |
| `specialattackdown` | BattleCommand_SpecialAttackDown | -1 SpAtk |
| `specialdefensedown` | BattleCommand_SpecialDefenseDown | -1 SpDef |
| `accuracydown` | BattleCommand_AccuracyDown | -1 Accuracy |
| `evasiondown` | BattleCommand_EvasionDown | -1 Evasion |
| `attackdown2` | BattleCommand_AttackDown2 | -2 Attack |
| `defensedown2` | BattleCommand_DefenseDown2 | -2 Defense |
| `speeddown2` | BattleCommand_SpeedDown2 | -2 Speed |
| `specialattackdown2` | BattleCommand_SpecialAttackDown2 | -2 SpAtk |
| `specialdefensedown2` | BattleCommand_SpecialDefenseDown2 | -2 SpDef |
| `accuracydown2` | BattleCommand_AccuracyDown2 | -2 Accuracy |
| `evasiondown2` | BattleCommand_EvasionDown2 | -2 Evasion |
| `allstatsup` | BattleCommand_AllStatsUp | +1 all 5 stats (Ancientpower) |
| `resetstats` | BattleCommand_ResetStats | Reset all stats to base (Haze) |
| `statmessageup` | BattleCommand_StatUpMessage | Print "[stat] rose!" |
| `statmessagedown` | BattleCommand_StatDownMessage | Print "[stat] fell!" |
| `statupfailtext` | BattleCommand_StatUpFailText | Print fail message for stat up |
| `statdownfailtext` | BattleCommand_StatDownFailText | Print fail message for stat down |

### Text/Animation Commands

| Command | Function | Description |
|---------|----------|-------------|
| `usedmovetext` | (print used text) | Print "[Pokemon] used [Move]!" |
| `failuretext` | BattleCommand_FailureText | Print miss/failure text, end multi-hit |
| `criticaltext` | BattleCommand_CriticalText | Print "Critical hit!" or wait 20 frames |
| `supereffectivetext` | BattleCommand_SuperEffectiveText | Print effectiveness message |
| `supereffectivelooptext` | BattleCommand_SuperEffectiveLoopText | Same but skips if in multi-hit loop |
| `cleartext` | BattleCommand_ClearText | Clear textbox (multi-hit moves) |
| `bidefailtext` | BattleCommand_BideFailText | Print Bide failure text |
| `moveanim` | BattleCommand_MoveAnim | Play move animation (with sub handling) |
| `moveanimnonsub` | BattleCommand_MoveAnimNoSub | Play animation without sub handling |
| `statupanim` | BattleCommand_StatUpAnim | Play stat up animation |
| `statdownanim` | BattleCommand_StatDownAnim | Play stat down animation |
| `movedelay` | BattleCommand_MoveDelay | Wait 40 frames |
| `lowersub` | BattleCommand_LowerSub | Lower substitute sprite |
| `raisesub` | BattleCommand_RaiseSub | Raise substitute sprite |

### Healing Commands

| Command | Function | Description |
|---------|----------|-------------|
| `heal` | BattleCommand_Heal | Heal HP (Recover/Rest) |
| `draintarget` | BattleCommand_DrainTarget | Drain HP (Absorb, Mega Drain) |
| `eatdream` | BattleCommand_EatDream | Dream Eater drain |
| `healmorn` | BattleCommand_HealMorn | Morning Sun |
| `healday` | BattleCommand_HealDay | Synthesis |
| `healnite` | BattleCommand_HealNite | Moonlight |

### Special Move Commands

| Command | Function | Description |
|---------|----------|-------------|
| `ohko` | BattleCommand_OHKO | One-hit KO (Fissure, Horn Drill, Guillotine) |
| `switchturn` | BattleCommand_SwitchTurn | Swap hBattleTurn (user<->opponent) |
| `forceswitch` | BattleCommand_ForceSwitch | Roar/Whirlwind forced switch |
| `curl` | BattleCommand_Curl | Set Defense Curl flag |
| `screen` | BattleCommand_Screen | Reflect or Light Screen |
| `rechargenextturn` | BattleCommand_RechargeNextTurn | Set recharge flag (Hyper Beam) |
| `arenatrap` | BattleCommand_ArenaTrap | Mean Look / Spider Web |
| `defrost` | BattleCommand_Defrost | Thaw user (Flame Wheel, Sacred Fire) |
| `defrostopponent` | BattleCommand_DefrostOpponent | Thaw opponent + Attack up |
| `skipsuncharge` | BattleCommand_SkipSunCharge | Skip charge in sun (SolarBeam) |

### Move Effect Commands (dedicated files)

| Command | Function | File |
|---------|----------|------|
| `attract` | BattleCommand_Attract | attract.asm |
| `batonpass` | BattleCommand_BatonPass | baton_pass.asm |
| `beatup` | BattleCommand_BeatUp | beat_up.asm |
| `bellydrum` | BattleCommand_BellyDrum | belly_drum.asm |
| `bide` | BattleCommand_Bide | bide.asm |
| `clearhazards` | BattleCommand_ClearHazards | rapid_spin.asm |
| `conversion` | BattleCommand_Conversion | conversion.asm |
| `conversion2` | BattleCommand_Conversion2 | conversion2.asm |
| `counter` | BattleCommand_Counter | counter.asm |
| `curse` | BattleCommand_Curse | curse.asm |
| `destinybond` | BattleCommand_DestinyBond | destiny_bond.asm |
| `disable` | BattleCommand_Disable | disable.asm |
| `encore` | BattleCommand_Encore | encore.asm |
| `endure` | BattleCommand_Endure | endure.asm |
| `focusenergy` | BattleCommand_FocusEnergy | focus_energy.asm |
| `foresight` | BattleCommand_Foresight | foresight.asm |
| `frustrationpower` | BattleCommand_FrustrationPower | frustration.asm |
| `furycutter` | BattleCommand_FuryCutter | fury_cutter.asm |
| `futuresight` | BattleCommand_FutureSight | future_sight.asm |
| `happinesspower` | BattleCommand_HappinessPower | return.asm |
| `healbell` | BattleCommand_HealBell | heal_bell.asm |
| `hiddenpower` | BattleCommand_HiddenPower | hidden_power.asm |
| `leechseed` | BattleCommand_LeechSeed | leech_seed.asm |
| `lockon` | BattleCommand_LockOn | lock_on.asm |
| `magnitude` | BattleCommand_GetMagnitude | magnitude.asm |
| `metronome` | BattleCommand_Metronome | metronome.asm |
| `mimic` | BattleCommand_Mimic | mimic.asm |
| `mirrorcoat` | BattleCommand_MirrorCoat | mirror_coat.asm |
| `mirrormove` | BattleCommand_MirrorMove | mirror_move.asm |
| `mist` | BattleCommand_Mist | mist.asm |
| `nightmare` | BattleCommand_Nightmare | nightmare.asm |
| `painsplit` | BattleCommand_PainSplit | pain_split.asm |
| `payday` | BattleCommand_PayDay | pay_day.asm |
| `perishsong` | BattleCommand_PerishSong | perish_song.asm |
| `present` | BattleCommand_Present | present.asm |
| `protect` | BattleCommand_Protect | protect.asm |
| `psychup` | BattleCommand_PsychUp | psych_up.asm |
| `pursuit` | BattleCommand_Pursuit | pursuit.asm |
| `rage` | BattleCommand_Rage | rage.asm |
| `rollout` | BattleCommand_Rollout | rollout.asm |
| `safeguard` | BattleCommand_Safeguard | safeguard.asm |
| `sandstorm` | BattleCommand_StartSandstorm | sandstorm.asm |
| `sketch` | BattleCommand_Sketch | sketch.asm |
| `sleeptalk` | BattleCommand_SleepTalk | sleep_talk.asm |
| `snore` | BattleCommand_Snore | snore.asm |
| `spikes` | BattleCommand_Spikes | spikes.asm |
| `splash` | BattleCommand_Splash | splash.asm |
| `startrain` | BattleCommand_StartRain | rain_dance.asm |
| `startsun` | BattleCommand_StartSun | sunny_day.asm |
| `substitute` | BattleCommand_Substitute | substitute.asm |
| `teleport` | BattleCommand_Teleport | teleport.asm |
| `thief` | BattleCommand_Thief | thief.asm |
| `thunderaccuracy` | BattleCommand_ThunderAccuracy | thunder.asm |
| `transform` | BattleCommand_Transform | transform.asm |
| `triplekick` | BattleCommand_TripleKick | triple_kick.asm |

---

## Execution Flow Details

### DoMove (how scripts are executed)

1. Look up move's effect in MoveEffectsPointers
2. Copy the effect script bytes to wBattleScriptBuffer
3. Set wBattleScriptBufferAddress to start of buffer
4. Loop: read next command byte, dispatch via BattleCommandPointers
5. Continue until `endmove` command is reached

### Key Dispatch Functions

- **DoPlayerTurn**: sets player turn, calls DoMove
- **DoEnemyTurn**: sets enemy turn, calls DoMove
- **ResetTurn**: used by Mirror Move/Metronome/Sleep Talk -- reruns DoMove with a new move loaded
- **EndMoveEffect**: writes endmove bytes to current position, terminating the script early
- **SkipToBattleCommand**: scans forward in script to find command b

### Critical Hit Calculation

Critical hit level is determined by summing bonuses:
- Base: 0
- Chansey + Lucky Punch: +2
- Farfetch'd + Stick: +2
- Focus Energy: +1
- High-crit move (Slash, Karate Chop, etc.): +2
- Scope Lens: +1

CriticalHitChances table (by level):
- Level 0: 17/256 (~6.6%)
- Level 1: 32/256 (12.5%)
- Level 2: 64/256 (25%)
- Level 3: 85/256 (33.2%)
- Level 4: 128/256 (50%)

### CheckHit Order of Operations

The accuracy check follows this exact sequence:
1. **DreamEater**: fails if target not asleep
2. **Protect**: fails if target is protected
3. **DrainSub**: fails if draining a substitute
4. **LockOn**: guaranteed hit if locked on (unless flying + ground move)
5. **FlyDigMoves**: fails if target underground/flying (unless move can hit them)
6. **ThunderRain**: Thunder always hits in rain
7. **XAccuracy**: always hits with X Accuracy
8. **EFFECT_ALWAYS_HIT**: Swift, etc. always hit
9. **StatModifiers**: apply accuracy/evasion stage modifiers
10. **BrightPowder**: reduce accuracy by holder's % miss
11. **Random roll**: compare random(0-255) vs modified accuracy

### Multi-Hit Distribution (BattleCommand_EndLoop)

| Effect | Hits |
|--------|------|
| EFFECT_DOUBLE_HIT | always 2 |
| EFFECT_POISON_MULTI_HIT | always 2 (Twineedle) |
| EFFECT_TRIPLE_KICK | 1-3 (random, weighted toward fewer) |
| EFFECT_BEAT_UP | 1 per party member |
| EFFECT_MULTI_HIT | 2-5 with probability: 2=3/8, 3=3/8, 4=1/8, 5=1/8 |

Multi-hit moves loop back to the `critical` command in the script, re-rolling critical hit and damage each iteration.

### Stat Stage System

Stat levels range from 1-13 (base = 7). Multipliers:

| Level | Multiplier |
|-------|-----------|
| 1 | 25/100 |
| 2 | 28/100 |
| 3 | 33/100 |
| 4 | 40/100 |
| 5 | 50/100 |
| 6 | 66/100 |
| 7 | 100/100 |
| 8 | 150/100 |
| 9 | 200/100 |
| 10 | 250/100 |
| 11 | 300/100 |
| 12 | 350/100 |
| 13 | 400/100 |

Max stat value: 999.

Accuracy/Evasion use a separate multiplier table (AccuracyLevelMultipliers) with numerator/denominator pairs.

### AI Handicap (25% Failure)

For status moves used by the AI (sleep, poison, paralyze, stat downs), there's a 25% random failure chance. This does NOT apply:
- In link battles
- In Battle Tower
- If Lock-On is active
- For accuracy-lowering secondary effects (EFFECT_ACCURACY_DOWN_HIT)

---

## Helper Functions

| Function | Description |
|----------|-------------|
| GetBattleVar(a) | Read battle variable by ID |
| GetBattleVarAddr(a) | Get address of battle variable |
| BattleRandom() | Generate random byte from battle RNG |
| CheckSubstituteOpp() | Return nz if opponent has substitute |
| CheckHiddenOpponent() | Return nz if opponent is flying/underground |
| CheckOpponentWentFirst() | Return z if user went first this turn |
| GetUserItem() | Return user's held item effect in bc |
| GetOpponentItem() | Return opponent's held item effect in bc |
| SafeCheckSafeguard() | Return nz if opponent has Safeguard |
| AnimateCurrentMove() | Play current move's animation |
| AnimateFailedMove() | Play failed move animation (sub handling + delay) |
| ResetDamage() | Set wCurDamage to 0 |
| StdBattleTextbox(hl) | Display battle text from hl |
| UpdateOpponentInParty() | Sync battle mon data back to party struct |
| RefreshBattleHuds() | Redraw both HP bars |
| CallBattleCore(hl) | Far call to function in Battle Core bank |
