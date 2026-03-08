# Pokemon Crystal -- Definitive Damage Formula Reference

Source: pokecrystal disassembly (engine/battle/effect_commands.asm), verified against Bulbapedia and Smogon analyses.

---

## Step-by-Step Damage Calculation

The damage formula is computed in a strict order. All arithmetic uses integer math (truncation/floor at each step).

### Phase 1: Base Damage (BattleCommand_DamageCalc)

```
BaseDamage = ((2 * Level / 5 + 2) * Power * Attack / Defense) / 50
```

Where:
- **Level** = attacker's level
- **Power** = move's base power (0 = no damage)
- **Attack** = attacker's effective Attack (physical) or Special Attack (special)
- **Defense** = target's effective Defense (physical) or Special Defense (special)

Each division truncates to integer. The computation is done in order:
1. `2 * Level` (can overflow; if it does, high byte is set to 1)
2. Divide by 5
3. Add 2
4. Multiply by Power
5. Multiply by Attack
6. Divide by Defense (minimum Defense = 1)
7. Divide by 50

### Phase 2: Stat Selection and Screens

**Physical moves** (type < SPECIAL constant, i.e., Normal through Rock):
- Uses attacker's Attack vs. target's Defense
- Reflect doubles the target's Defense value before stat stage application

**Special moves** (type >= SPECIAL constant, i.e., Fire through Dark):
- Uses attacker's Special Attack vs. target's Special Defense
- Light Screen doubles the target's Special Defense value before stat stage application

Screens double the raw stat value (sla c; rl b in ASM), which can cause **overflow past 1024** -- see Glitches section.

### Phase 3: Critical Hit Stat Handling (CheckDamageStatsCritical)

On a critical hit:
- If the target's defensive stat stage >= attacker's offensive stat stage: ALL stat stage modifiers are ignored, burn's Attack halving is ignored, and Reflect/Light Screen are ignored. Raw base stats are used.
- If the attacker's offensive stat stage > target's defensive stat stage: stat stages are applied normally (attacker keeps its boosts, defender keeps its drops).

### Phase 4: Species-Specific Item Boosts

Applied to the Attack/SpAtk value BEFORE TruncateHL_BC:

- **Thick Club**: Doubles Attack if held by Cubone or Marowak (physical moves only)
- **Light Ball**: Doubles Special Attack if held by Pikachu (special moves only)
- **Metal Powder**: Multiplies Defense by 1.5 if held by Ditto (applied after TruncateHL_BC)

BUG: These doublings can wrap around above 1024 (16-bit overflow) -- see Glitches.

### Phase 5: TruncateHL_BC

The 16-bit Attack and Defense values are iteratively right-shifted by 2 bits each until both fit in 8 bits. This preserves the ratio while fitting into the 8-bit registers used by the damage formula.

BUG: In link battles (Colosseum), truncation only runs once. In single-player battles, it runs twice (checking `h or b` again), which can cause values > 1024 to wrap around.

### Phase 6: Self-Destruct/Explosion

If the move has EFFECT_SELFDESTRUCT, the target's Defense is halved (srl c). Minimum Defense is 1.

### Phase 7: Type-Boosting Items

After the base damage calculation, type-boosting held items apply:
```
Damage = Damage * (100 + ItemBonus) / 100
```

Item bonuses are typically 10% (adding 10 to 100):
- Charcoal (Fire), Mystic Water (Water), Magnet (Electric), Miracle Seed (Grass)
- NeverMeltIce (Ice), Black Belt (Fighting), Poison Barb (Poison)
- Soft Sand (Ground), Sharp Beak (Flying), Twisted Spoon (Psychic)
- Silver Powder (Bug), Hard Stone (Rock), Spell Tag (Ghost)
- Dragon Scale (Dragon -- BUG: should be Dragon Fang), Black Glasses (Dark)
- Metal Coat (Steel), Pink Bow/Polkadot Bow (Normal)
- Silk Scarf is NOT in Gen 2

BUG: Dragon Scale, not Dragon Fang, boosts Dragon-type moves due to a data error.

BUG: Type-boosting items also affect confusion self-hit damage (they shouldn't).

### Phase 8: Critical Hit Multiplier

If critical hit: Damage = Damage * 2 (left shift by 1). Capped at $FFFF (65535).

### Phase 9: Damage Cap

Raw damage is capped at 997, then 2 is added back, for a maximum of **999** and minimum of **2** (for any move that deals damage at all).

### Phase 10: STAB (BattleCommand_Stab)

Same Type Attack Bonus: If the move's type matches either of the user's types:
```
Damage = Damage + Damage / 2  (i.e., 1.5x, truncated)
```

### Phase 11: Weather Modifiers (DoWeatherModifiers)

Applied as a multiplier/divisor of 10:
- Rain + Water move: Damage * 15 / 10 (1.5x)
- Rain + Fire move: Damage * 5 / 10 (0.5x)
- Sun + Fire move: Damage * 15 / 10 (1.5x)
- Sun + Water move: Damage * 5 / 10 (0.5x)
- Rain + SolarBeam: Damage * 5 / 10 (0.5x) -- special move effect modifier
- Minimum 1 damage after weather

### Phase 12: Badge Boosts (DoBadgeTypeBoosts)

Only applies to the **player's** Pokemon (not enemy/link). Only in single-player (not link, not Battle Tower).

```
Damage = Damage + Damage / 8  (i.e., 1.125x, truncated)
```

Minimum boost is 1 (if Damage/8 rounds to 0, boost is 1 instead).

Badge-to-type mapping:
| Badge | Type |
|-------|------|
| Zephyr Badge | Flying |
| Hive Badge | Bug |
| Plain Badge | Normal |
| Fog Badge | Ghost |
| Mineral Badge | Steel |
| Storm Badge | Fighting |
| Glacier Badge | Ice |
| Rising Badge | Dragon |
| Boulder Badge (Kanto) | Rock |
| Cascade Badge (Kanto) | Water |
| Thunder Badge (Kanto) | Electric |
| Rainbow Badge (Kanto) | Grass |
| Soul Badge (Kanto) | Poison |
| Marsh Badge (Kanto) | Psychic |
| Volcano Badge (Kanto) | Fire |
| Earth Badge (Kanto) | Ground |

### Phase 13: Type Effectiveness (in BattleCommand_Stab)

Multiplied per defending type. Stored as 10-based (10 = neutral):
- Super effective: multiply by 20, divide by 10 (2x)
- Not very effective: multiply by 5, divide by 10 (0.5x)
- Immune: multiply by 0 (0x, also sets AttackMissed)
- Minimum 1 damage if non-zero effectiveness

For dual-typed defenders, both types are checked independently. Possible net multipliers: 0x, 0.25x, 0.5x, 1x, 2x, 4x.

### Phase 14: Damage Variation (BattleCommand_DamageVariation)

Only applies if damage >= 2 before variation.

Random factor: a random byte is generated, right-rotated, and must be >= 217 (85% of 256). The result is multiplied by the damage and divided by 256.

Effective range: 85% to 100% of calculated damage. Due to the generation method, higher rolls are slightly rarer.

### Phase 15: Move-Specific Modifiers

Applied at various points depending on the move effect:

- **Pursuit**: Damage doubled if opponent is switching (sla; rl; cap at $FFFF)
- **Triple Kick**: Damage multiplied by hit number (1, 2, 3)
- **Rollout**: Damage doubles each consecutive hit (up to 5 hits = 16x). Defense Curl doubles Rollout's initial power.
- **Fury Cutter**: Damage doubles each consecutive hit (capped at 5 doublings = 16x base 10 = 160 power)
- **Future Sight**: Damage is pre-calculated and stored, delivered 3 turns later. Ignores type effectiveness on delivery.

---

## Critical Hit Calculation

### Critical Hit Rate

Base critical level is 0. Modifiers stack additively:

| Condition | Bonus |
|-----------|-------|
| Focus Energy or Dire Hit | +1 |
| Scope Lens held | +1 |
| High crit-ratio move | +2 |
| Chansey + Lucky Punch | +2 (replaces all above) |
| Farfetch'd + Stick | +2 (replaces all above) |

High crit-ratio moves: Karate Chop, Razor Wind, Razor Leaf, Crabhammer, Slash, Aeroblast, Cross Chop.

### Critical Hit Chance Table

| Stage | Chance | Probability |
|-------|--------|-------------|
| 0 | 17/256 | ~6.64% |
| 1 | 32/256 | 12.5% |
| 2 | 64/256 | 25.0% |
| 3 | 85/256 | ~33.2% |
| 4+ | 128/256 | 50.0% |

Note: Chansey with Lucky Punch or Farfetch'd with Stick: the +2 replaces all other modifiers, giving 25% crit rate. With Focus Energy on top, that becomes stage 3 (33.2%).

### Focus Energy in Gen 2

Unlike Gen 1 (where Focus Energy divided the crit rate by 4 due to a bug), Focus Energy correctly adds +1 to the critical hit stage in Gen 2.

---

## Fixed/Constant Damage Moves

These bypass the normal damage formula entirely:

- **Seismic Toss / Night Shade** (EFFECT_LEVEL_DAMAGE): Damage = user's level
- **Psywave** (EFFECT_PSYWAVE): Damage = random 1 to (user's level * 1.5), exclusive
- **Super Fang** (EFFECT_SUPER_FANG): Damage = target's current HP / 2 (minimum 1)
- **Dragon Rage**: Always 40 damage
- **Sonic Boom**: Always 20 damage
- **Flail / Reversal** (EFFECT_REVERSAL): Power based on current HP percentage (lookup table)
- **Present**: Random power selection (see move_edge_cases.md)

### Flail/Reversal Power Table

| HP remaining (x/48) | Power |
|---------------------|-------|
| 0-1 | 200 |
| 2-4 | 150 |
| 5-9 | 100 |
| 10-16 | 80 |
| 17-32 | 40 |
| 33-48 | 20 |

---

## Integer Overflow Edge Cases

1. **Thick Club + stat boosts**: Cubone/Marowak Attack doubled by Thick Club, then further boosted by Swords Dance. If the 16-bit Attack value exceeds 1024 before TruncateHL_BC, the truncation loop wraps around, potentially producing very low values.

2. **Light Ball + stat boosts**: Same issue for Pikachu's Special Attack.

3. **Reflect/Light Screen + high Defense/SpDef**: Doubling a 16-bit defense stat can overflow above 1024, causing the same truncation bug.

4. **Metal Powder**: Adding 50% of a high defense value can overflow, potentially making Ditto take MORE damage.

5. **Critical hit damage**: The 2x multiplier can overflow the 16-bit damage value, capped at $FFFF.

6. **Damage cap**: The game caps at 997 + 2 = 999, so overflow in the formula itself is caught, but stat overflows happen before the cap is applied.
