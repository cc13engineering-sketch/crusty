# Pokemon Crystal -- Complete Stat System Reference

Source: pokecrystal disassembly, Bulbapedia stat mechanics pages.

---

## Stat Formulas

Gen 2 uses **DVs** (0-15) and **Stat Experience** (0-65535). There are NO natures.

### HP Formula

```
HP = floor(((Base + DV) * 2 + floor(ceil(sqrt(StatExp)) / 4)) * Level / 100) + Level + 10
```

### Other Stats (Atk, Def, SpAtk, SpDef, Speed)

```
Stat = floor(((Base + DV) * 2 + floor(ceil(sqrt(StatExp)) / 4)) * Level / 100) + 5
```

Where:
- **Base** = species base stat value (0-255)
- **DV** = Determinant Value (0-15)
- **StatExp** = Stat Experience (0-65535)
- **Level** = Pokemon's current level (1-100)

### Stat Experience Contribution

The Stat Experience contribution to the formula is:
```
floor(ceil(sqrt(StatExp)) / 4)
```

Maximum: floor(ceil(sqrt(65535)) / 4) = floor(256 / 4) = 64

So at max Stat Experience, you get +64 to the inner sum.

### Example Calculation

Level 50 Mewtwo (base SpAtk = 154), DV = 15, max StatExp:
```
Inner = (154 + 15) * 2 + 64 = 338 + 64 = 402
Stat = floor(402 * 50 / 100) + 5 = 201 + 5 = 206
```

---

## Determinant Values (DVs)

DVs are Gen 2's equivalent of IVs. Range: 0-15 for each stat.

### DV Storage

DVs are stored as two bytes (16 bits total):
- Byte 1: `AAAA DDDD` (high nibble = Attack DV, low nibble = Defense DV)
- Byte 2: `SSSS PPPP` (high nibble = Speed DV, low nibble = Special DV)

### HP DV

HP DV is NOT stored directly. It is derived from the lowest bit of each other DV:

```
HP_DV = (Attack_DV & 1) * 8 + (Defense_DV & 1) * 4 + (Speed_DV & 1) * 2 + (Special_DV & 1)
```

This means HP DV range is also 0-15, but it's determined by the other 4 DVs.

### Special DV

The Special DV is **shared** between Special Attack and Special Defense. Both use the same DV value.

---

## Gender Determination

Gender is determined by the **Attack DV**:

- Each species has a gender threshold (0-15, or 255 for genderless)
- If Attack DV >= threshold: Male
- If Attack DV < threshold: Female

Common thresholds:
- 0: Always male (genderless species use 255 instead)
- 2: 87.5% male / 12.5% female (starters, Eevee)
- 4: 75% male / 25% female
- 8: 50% male / 50% female (most Pokemon)
- 12: 25% male / 75% female
- 14: 12.5% male / 87.5% female
- 255: Genderless

**Consequence**: Female Pokemon can NEVER have Attack DV >= their gender threshold. For species with 87.5% male ratio (threshold = 2), females can only have Attack DV 0 or 1, severely limiting their max Attack stat.

---

## Shininess

A Pokemon is shiny if ALL of the following DV conditions are met:
- Defense DV = 10
- Speed DV = 10
- Special DV = 10
- Attack DV is one of: 2, 3, 6, 7, 10, 11, 14, 15

This gives a 5/65536 probability per encounter (Attack has 4 valid values out of 16; others must be exactly 10). In practice this is approximately **1/8192**.

The Attack DV constraint means:
- HP DV for shinies is always: (Atk&1)*8 + 0*4 + 0*2 + 0*1 -- since Def=10 (even), Spd=10 (even), Spc=10 (even). So HP DV = 0 or 8.
- Female shinies are impossible for species with gender ratio requiring Attack DV < 2 (since minimum shiny Attack DV is 2).

---

## Stat Experience

Stat Experience is Gen 2's EV-like system. Range: 0-65535 per stat.

### Accumulation

When you defeat a Pokemon, each of your participating Pokemon gains Stat Experience equal to the **base stats** of the defeated species:

- HP StatExp += defeated Pokemon's base HP
- Attack StatExp += defeated Pokemon's base Attack
- Defense StatExp += defeated Pokemon's base Defense
- Speed StatExp += defeated Pokemon's base Speed
- Special StatExp += defeated Pokemon's base Special (used for both SpAtk and SpDef)

There is NO cap on total Stat Experience across stats (unlike EVs in Gen 3+). You can max all five.

### Vitamins

Vitamins (HP Up, Protein, Iron, Calcium, Carbos) each add 2560 to the respective stat's Stat Experience. They cannot raise StatExp above 25600.

### Stat Recalculation

Stats are recalculated:
- On level up
- On evolution
- On depositing/withdrawing from PC
- On using vitamins (Rare Candy triggers level up recalc)
- NOT continuously in battle -- stat changes in battle use stat stages

---

## Stat Stages (In-Battle Modifiers)

Stat stages range from -6 to +6 (default 0). Applied as multipliers:

### Attack, Defense, Special Attack, Special Defense, Speed

| Stage | Multiplier | Fraction |
|-------|-----------|----------|
| -6 | 0.25 | 2/8 |
| -5 | 0.286 | 2/7 |
| -4 | 0.333 | 2/6 |
| -3 | 0.4 | 2/5 |
| -2 | 0.5 | 2/4 |
| -1 | 0.667 | 2/3 |
| 0 | 1.0 | 2/2 |
| +1 | 1.5 | 3/2 |
| +2 | 2.0 | 4/2 |
| +3 | 2.5 | 5/2 |
| +4 | 3.0 | 6/2 |
| +5 | 3.5 | 7/2 |
| +6 | 4.0 | 8/2 |

### Accuracy and Evasion

Use different multipliers:

| Stage | Multiplier | Fraction |
|-------|-----------|----------|
| -6 | 0.333 | 3/9 |
| -5 | 0.375 | 3/8 |
| -4 | 0.429 | 3/7 |
| -3 | 0.5 | 3/6 |
| -2 | 0.6 | 3/5 |
| -1 | 0.75 | 3/4 |
| 0 | 1.0 | 3/3 |
| +1 | 1.333 | 4/3 |
| +2 | 1.667 | 5/3 |
| +3 | 2.0 | 6/3 |
| +4 | 2.333 | 7/3 |
| +5 | 2.667 | 8/3 |
| +6 | 3.0 | 9/3 |

### Accuracy Check Formula

```
EffectiveAccuracy = MoveAccuracy * AccuracyStageMultiplier * EvasionStageMultiplier
```

Where evasion stage is inverted (subtracted from MAX_STAT_LEVEL + 1 = 14).

If Foresight is active on the target AND the target's evasion stage > attacker's accuracy stage, the evasion/accuracy modifiers are bypassed entirely (the move always hits at its base accuracy).

### Stat Stage Interactions

- **Baton Pass**: All stat stages are passed to the incoming Pokemon
- **Psych Up**: Copies all stat stages from the target
- **Haze**: Resets all stat stages to 0 for both sides
- Switching out normally resets all stat stages to 0
- **Critical hits**: See damage_formula.md Phase 3

---

## Paralysis and Burn Stat Effects

### Paralysis Speed Reduction

When paralyzed, the Speed stat is multiplied by 25% (quartered). This is applied to the battle stat directly and stacks with stat stage modifications.

### Burn Attack Reduction

When burned, the Attack stat is multiplied by 50% (halved). This affects physical move damage.

On a critical hit: If the attacker's Attack stage >= target's Defense stage, the burn halving is NOT ignored. If the target's Defense stage > attacker's Attack stage, the burn halving IS ignored (along with all stat stages).
