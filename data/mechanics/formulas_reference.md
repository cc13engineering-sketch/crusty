# Pokemon Crystal — Complete Formulas Reference

Source: pokecrystal disassembly (engine/battle/, engine/pokemon/, engine/items/, engine/events/)

---

## Damage Formula

```
Damage = (((2 * Level / 5 + 2) * Power * A / D) / 50 + 2) * Modifier
```

Where:
- Level = attacker's level
- Power = move's base power
- A = attacker's Attack (physical) or Special Attack (special)
- D = defender's Defense (physical) or Special Defense (special)
- Modifier = STAB * TypeEffectiveness * CritMultiplier * WeatherMod * BadgeBoost * ItemBoost * DamageVariation

Phases (exact order from effect_commands.asm):
1. Base: ((2*Level/5 + 2) * Power * A / D) / 50 + 2
2. Critical hit: 2x damage (bypasses stat stage modifiers for relevant stat)
3. Weather: Rain+Water 1.5x, Rain+Fire 0.5x, Sun+Fire 1.5x, Sun+Water 0.5x, Rain+SolarBeam 0.5x
4. Badge type boost: 1.125x per matching badge (player only, not in link/BattleTower)
5. STAB: 1.5x if move type matches attacker's type
6. Type effectiveness: 2x (super effective), 0.5x (not very effective), 0x (immune) — applied per type for dual-type Pokemon
7. Damage variation: random integer 217-255, divided by 255 (effectively 85-100% of calculated damage)
8. Item boosts: Thick Club (Marowak 2x Atk), Light Ball (Pikachu 2x SpAtk), Metal Powder (Ditto 1.5x Def)

Cap: Damage capped at 997 before variation, final result capped at 997.

Source: `engine/battle/effect_commands.asm` — BattleCommand_DamageCalc, PlayerAttackDamage, EnemyAttackDamage

---

## Stat Calculation

### HP Formula
```
HP = ((Base + DV) * 2 + floor(sqrt(StatExp)) / 4) * Level / 100 + Level + 10
```

### Other Stats (Attack, Defense, Speed, Special Attack, Special Defense)
```
Stat = ((Base + DV) * 2 + floor(sqrt(StatExp)) / 4) * Level / 100 + 5
```

- Base: Species base stat (0-255)
- DV: Determinant Value (0-15) — Gen 2 uses DVs, not IVs
- StatExp: Stat Experience from defeated Pokemon (0-65535)
- Level: 1-100

Source: `engine/pokemon/mon_stats.asm` — CalcMonStats

### DV System
- Attack DV: 0-15 (4 bits)
- Defense DV: 0-15 (4 bits)
- Speed DV: 0-15 (4 bits)
- Special DV: 0-15 (4 bits, used for both SpAtk and SpDef)
- HP DV: Derived from other DVs. HP_DV = (Atk_DV & 1) << 3 | (Def_DV & 1) << 2 | (Spd_DV & 1) << 1 | (Spc_DV & 1)

### Gender from DVs
Gender is determined by Attack DV compared to the species' gender ratio threshold. If Atk DV >= threshold, male; otherwise, female. Genderless species have ratio 255.

### Shininess from DVs
Shiny if: Def DV = 10, Spd DV = 10, Spc DV = 10, and Atk DV is 2, 3, 6, 7, 10, 11, 14, or 15. Probability: 1/8192.

---

## Stat Stages

| Stage | Multiplier |
|-------|-----------|
| -6 | 25/100 |
| -5 | 28/100 |
| -4 | 33/100 |
| -3 | 40/100 |
| -2 | 50/100 |
| -1 | 66/100 |
| 0 | 100/100 |
| +1 | 150/100 |
| +2 | 200/100 |
| +3 | 250/100 |
| +4 | 300/100 |
| +5 | 350/100 |
| +6 | 400/100 |

Accuracy/Evasion stages use a different table:

| Stage | Multiplier |
|-------|-----------|
| -6 | 33/100 |
| -5 | 36/100 |
| -4 | 43/100 |
| -3 | 50/100 |
| -2 | 60/100 |
| -1 | 75/100 |
| 0 | 100/100 |
| +1 | 133/100 |
| +2 | 166/100 |
| +3 | 200/100 |
| +4 | 250/100 |
| +5 | 266/100 |
| +6 | 300/100 |

Source: `data/battle/stat_multipliers.asm`, `data/battle/accuracy_multipliers.asm`

---

## Critical Hit

### Critical Hit Stages
| Stage | Chance |
|-------|--------|
| 0 | 17/256 (~6.6%, ~1/15) |
| 1 | 32/256 (~12.5%, ~1/8) |
| 2 | 64/256 (25%, 1/4) |
| 3 | 85/256 (~33%, ~1/3) |
| 4+ | 128/256 (50%, 1/2) |

Source: `data/battle/critical_hit_chances.asm`

### Stage Calculation
- Base stage: 0
- High-crit move (Karate Chop, Razor Wind, Razor Leaf, Crabhammer, Slash, Aeroblast, Cross Chop): +2
- Focus Energy: +1
- Scope Lens held: +1
- Maximum effective stage: 4 (all above cap)

Source: `data/moves/critical_hit_moves.asm`, effect_commands.asm BattleCommand_Critical

---

## Catch Rate Formula

```
CatchValue = (CatchRate * BallModifier * (MaxHP * 3 - CurrentHP * 2)) / (MaxHP * 3)
```

Then: StatusBonus = +10 if Frozen/Asleep, +0 otherwise (BUG: BRN/PSN/PAR should add +5 but don't due to a bug in item_effects.asm)

```
FinalCatchValue = CatchValue + StatusBonus
```

Random number 0-255 is generated. If random < FinalCatchValue, the Pokemon is caught.

### Ball Multipliers (from BallMultiplierFunctionTable)
| Ball | Multiplier |
|------|-----------|
| Master Ball | Always catches |
| Ultra Ball | 2x catch rate |
| Great Ball | 1.5x catch rate |
| Poke Ball | 1x (no multiplier) |
| Safari Ball | 1.5x (leftover from Gen 1) |
| Park Ball | 1.5x |
| Level Ball | 8x if player level/4 > enemy; 4x if /2 >; 2x if > |
| Lure Ball | 3x if fishing encounter |
| Moon Ball | BUG: Should be catch rate *4 for Moon Stone evolvers, but checks HELD_BURN_HEAL instead of HELD_MOON_STONE, so it never works |
| Love Ball | BUG: Boosts catch rate 8x when SAME gender instead of opposite gender |
| Fast Ball | BUG: Should boost for Pokemon with flee rate >= 150, but compares wrong data — only works for Grimer, Tangela, and Mr. Mime |
| Heavy Ball | -20 if weight < 102.4kg, +0 if < 204.8kg, +20 if < 307.2kg, +30 if < 409.6kg, +40 if >= 409.6kg |
| Friend Ball | No catch rate bonus (sets caught Pokemon's happiness to 200) |

Source: `engine/items/item_effects.asm` lines 230-1060

---

## Experience Gain Formula

```
BaseExp = (EnemyBaseExp * EnemyLevel) / 7
```

Multipliers (each is 1.5x, applied sequentially):
- Traded Pokemon (different OT ID): 1.5x
- Trainer battle (not wild): 1.5x
- Lucky Egg held: 1.5x

Maximum: All three can stack. Traded Pokemon in a trainer battle with Lucky Egg gets 1.5^3 = 3.375x experience.

Source: `engine/battle/core.asm` lines 7067-7110, BoostExp at line 7391

### Experience Groups (Growth Rates)
```
Formula: (a/b)*n^3 + c*n^2 + d*n - e
```

| Group | a/b | c | d | e | Total at L100 |
|-------|-----|---|---|---|---------------|
| Medium Fast | 1/1 | 0 | 0 | 0 | 1,000,000 |
| Slightly Fast | 3/4 | 10 | 0 | 30 | 849,970 |
| Slightly Slow | 3/4 | 20 | 0 | 70 | 949,930 |
| Medium Slow | 6/5 | -15 | 100 | 140 | 1,059,860 |
| Fast | 4/5 | 0 | 0 | 0 | 800,000 |
| Slow | 5/4 | 0 | 0 | 0 | 1,250,000 |

BUG: Medium Slow gives negative experience at level 1 due to the formula producing a negative value, causing an underflow to a very large number.

Source: `data/growth_rates.asm`

---

## Happiness System

### Happiness Thresholds
- Range: 0-255
- Evolution threshold (Espeon, Umbreon, Blissey): >= 220
- Frustration: power = (255 - happiness) * 10 / 25
- Return: power = happiness * 10 / 25

### Happiness Changes (per event)
Format: [happiness < 100, happiness 100-199, happiness >= 200]

| Event | Low | Mid | High |
|-------|-----|-----|------|
| Level up | +5 | +3 | +2 |
| Level up at caught location | +10 | +6 | +4 |
| Vitamin | +5 | +3 | +2 |
| X Item in battle | +1 | +1 | +0 |
| Beat Gym Leader | +3 | +2 | +1 |
| Learn a move (TM/tutor) | +1 | +1 | +0 |
| Lose to enemy | -1 | -1 | -1 |
| Faint to poison in overworld | -5 | -5 | -10 |
| Lose to much stronger enemy | -5 | -5 | -10 |
| Haircut (older brother, bad) | +1 | +1 | +1 |
| Haircut (older brother, good) | +3 | +3 | +1 |
| Haircut (older brother, great) | +5 | +5 | +2 |
| Haircut (younger brother, bad) | +1 | +1 | +1 |
| Haircut (younger brother, good) | +3 | +3 | +1 |
| Haircut (younger brother, great) | +10 | +10 | +4 |
| Energypowder/Heal Powder | -5 | -5 | -10 |
| Energy Root | -10 | -10 | -15 |
| Revival Herb | -15 | -15 | -20 |
| Daisy's Grooming | +3 | +3 | +1 |
| Walking (every other 256-step cycle) | +1 | +1 | +1* |

*Walking happiness: every other step cycle, each party member gains +1 happiness (capped at 255).

### Haircut Probabilities
- Older brother: 30% bad, 50% good, 20% great
- Younger brother: 60% bad, 30% good, 10% great

Source: `data/events/happiness_changes.asm`, `data/events/happiness_probabilities.asm`, `engine/events/happiness_egg.asm`

---

## Flee Formula (Wild Battles)

```
F = (PlayerSpeed * 32) / (EnemySpeed / 4)
```

If PlayerSpeed >= EnemySpeed: always escape.

Otherwise: Each failed attempt adds +30 to F. Generate random byte; if random < F, escape succeeds.

Items: Smoke Ball (HELD_ESCAPE) guarantees escape. Poke Doll/Fluffy Tail also guarantee escape.

Cannot flee: Trainer battles, Mean Look/Spider Web (SUBSTATUS_CANT_RUN), Wrap/Bind/Fire Spin/Clamp (wPlayerWrapCount > 0), specific battle types (BATTLETYPE_TRAP, BATTLETYPE_CELEBI, BATTLETYPE_SUICUNE, BATTLETYPE_FORCESHINY).

Source: `engine/battle/core.asm` TryToRunAwayFromBattle (line 3676)

---

## Prize Money Formula

```
PrizeMoney = BaseReward * LastPokemonLevel * 2
```

If Amulet Coin held: PrizeMoney *= 2

Base reward comes from the trainer class's base money value. The level used is the last Pokemon the trainer sent out.

### Whiteout Money Loss
Player loses exactly half their money upon whiteout (all Pokemon fainted).
```
NewMoney = Money >> 1  (arithmetic right shift of 3-byte value)
```

Source: `engine/events/whiteout.asm` — HalveMoney

---

## Pay Day

```
CoinsPerUse = UserLevel * 2
```

Amulet Coin doubles the total at end of battle.

Source: `engine/battle/move_effects/pay_day.asm`, `engine/battle/core.asm` CheckPayDay (line 8320)

---

## Step Counter Mechanics

### Egg Hatching
- Every step cycle, Day Care Pokemon gain +1 experience point
- Egg step counter: each egg species has an "egg cycles" value (in base stats). Each cycle = 256 steps. Counter decrements by 1 each cycle. Egg hatches when counter reaches 0.

### Repel
- Repel: 100 steps
- Super Repel: 200 steps
- Max Repel: 250 steps
- Repels prevent wild encounters from Pokemon with level < lead party member's level

### Poison Step Damage
- Each step in the overworld, each poisoned party member loses 1 HP
- If HP reaches 0, the Pokemon faints (happiness penalty: -5/-5/-10 based on happiness bracket)
- If all Pokemon faint to poison, whiteout occurs
- In Gen 2 (unlike Gen 1), Pokemon CAN faint from poison in the overworld

Source: `engine/events/poisonstep.asm`, `engine/events/happiness_egg.asm`

### Happiness Walking
- Every 256 steps, a counter toggles
- Every other toggle (512 steps effectively), each non-egg party member gains +1 happiness
- Capped at 255

Source: `engine/events/happiness_egg.asm` — StepHappiness

---

## Day Care

### Day Care Experience
- Each step cycle, Day Care Pokemon gain +1 to their 3-byte experience value
- Experience capped at MAX_DAY_CARE_EXP
- Level is recalculated from experience when withdrawn
- Cost to retrieve: 100 + (100 * levels_gained)

### Egg Generation
- Compatibility checked via wBreedingCompatibility
- Egg probability per step cycle based on compatibility:
  - >= 230: ~31% chance per random check
  - >= 170: ~16% chance
  - >= 110: ~12% chance
  - < 110: ~4% chance
- Steps between checks: random (wStepsToEgg starts at a random value, decrements to 0)

Source: `engine/events/happiness_egg.asm` — DayCareStep

---

## Hidden Power

```
Type = ((AtkDV & 3) << 2) | (DefDV & 3)
```
Maps to types, skipping Normal and ??? (Bird):
0=Fighting, 1=Flying, 2=Poison, 3=Ground, 4=Rock, 5=Bug, 6=Ghost, 7=Steel, 8=Fire, 9=Water, 10=Grass, 11=Electric, 12=Psychic, 13=Ice, 14=Dragon, 15=Dark

```
Power = (5 * ((AtkDV >> 3) | (DefDV >> 3) << 1 | (SpdDV >> 3) << 2 | (SpcDV >> 3) << 3) + (SpcDV & 3)) / 2 + 31
```
Range: 31-70

Source: `engine/battle/hidden_power.asm`

---

## AI Scoring

The AI uses a scoring system with 8 layers, each adjusting move scores:
- Score range: 0-255 (lower = better, starts at $80 = 128)
- Each layer can add/subtract from scores
- After all layers, the move with the lowest score is selected
- Ties are broken randomly

Layers: AI_Basic, AI_Setup, AI_Types, AI_Offensive, AI_Smart, AI_Cautious, AI_Status, AI_Risky

Source: `engine/battle/ai/scoring.asm`
