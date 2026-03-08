# Pokemon Crystal - AI Data Tables

Battle AI scoring data and stat modifier tables from pokecrystal data/battle/ and data/battle/ai/.

Source: `data/battle/*.asm`, `data/battle/ai/*.asm`

---

## Stat Modifier Multipliers

From `data/battle/stat_multipliers.asm`. Applied to Attack, Defense, Speed, Sp.Atk, Sp.Def:

| Stage | Multiplier | Percentage |
|-------|-----------|------------|
| -6 | 25/100 | 25% |
| -5 | 28/100 | 28% |
| -4 | 33/100 | 33% |
| -3 | 40/100 | 40% |
| -2 | 50/100 | 50% |
| -1 | 66/100 | 66% |
| 0 | 1/1 | 100% |
| +1 | 15/10 | 150% |
| +2 | 2/1 | 200% |
| +3 | 25/10 | 250% |
| +4 | 3/1 | 300% |
| +5 | 35/10 | 350% |
| +6 | 4/1 | 400% |

## Accuracy Modifier Multipliers

From `data/battle/accuracy_multipliers.asm`. Different from stat multipliers:

| Stage | Multiplier | Percentage |
|-------|-----------|------------|
| -6 | 33/100 | 33% |
| -5 | 36/100 | 36% |
| -4 | 43/100 | 43% |
| -3 | 50/100 | 50% |
| -2 | 60/100 | 60% |
| -1 | 75/100 | 75% |
| 0 | 1/1 | 100% |
| +1 | 133/100 | 133% |
| +2 | 166/100 | 166% |
| +3 | 2/1 | 200% |
| +4 | 233/100 | 233% |
| +5 | 133/50 | 266% |
| +6 | 3/1 | 300% |

---

## Critical Hit Chances

From `data/battle/critical_hit_chances.asm`:

| Crit Stage | Chance | Probability |
|-----------|--------|-------------|
| +0 (base) | 1/16 | 6.25% |
| +1 | 1/8 | 12.5% |
| +2 | 1/4 | 25% |
| +3 | 1/3 | 33.3% |
| +4 | 1/2 | 50% |
| +5 | 1/2 | 50% (capped) |
| +6 | 1/2 | 50% (capped) |

Crit stage modifiers:
- High crit moves (Slash, Cross Chop, etc.): +1
- Focus Energy: +1
- Scope Lens (held): +1
- Lucky Punch (Chansey held): +2
- Stick (Farfetch'd held): +2
- Dire Hit: +1

---

## Weather Modifiers

From `data/battle/weather_modifiers.asm`:

### Type Modifiers in Weather
| Weather | Type | Effect |
|---------|------|--------|
| Rain | Water | 1.5x damage (MORE_EFFECTIVE) |
| Rain | Fire | 0.5x damage (NOT_VERY_EFFECTIVE) |
| Sun | Fire | 1.5x damage (MORE_EFFECTIVE) |
| Sun | Water | 0.5x damage (NOT_VERY_EFFECTIVE) |

### Move-Specific Weather Modifiers
| Weather | Move | Effect |
|---------|------|--------|
| Rain | SolarBeam | 0.5x damage (charges in 1 turn in sun, 2 turns in rain) |

---

## Ball Wobble Probabilities

From `data/battle/wobble_probabilities.asm`. After throwing a ball, each wobble has a probability of success based on the modified catch rate:

| Catch Rate | Wobble Chance (per shake) |
|-----------|--------------------------|
| 1 | 63/255 (24.7%) |
| 2 | 75/255 (29.4%) |
| 3 | 84/255 (32.9%) |
| 5 | 95/255 (37.3%) |
| 10 | 113/255 (44.3%) |
| 20 | 134/255 (52.5%) |
| 40 | 160/255 (62.7%) |
| 80 | 191/255 (74.9%) |
| 120 | 211/255 (82.7%) |
| 160 | 227/255 (89.0%) |
| 200 | 240/255 (94.1%) |
| 240 | 251/255 (98.4%) |
| 255 | 255/255 (100%) |

A successful catch requires 3 consecutive wobbles passing this check. So for catch rate 120: 0.827^3 = 56.4% catch chance.

---

## Held Item Battle Effects

### Consumable Effects
From `data/battle/held_consumables.asm`. These held items are consumed when activated:

- HELD_BERRY (Berry, Gold Berry) — Auto-heal HP
- HELD_HEAL_POISON (PSNCureBerry) — Auto-cure poison
- HELD_HEAL_FREEZE (Burnt Berry) — Auto-cure freeze
- HELD_HEAL_BURN (Ice Berry) — Auto-cure burn
- HELD_HEAL_SLEEP (Mint Berry) — Auto-cure sleep
- HELD_HEAL_PARALYZE (PRZCureBerry) — Auto-cure paralysis
- HELD_HEAL_STATUS (MiracleBerry) — Auto-cure any status
- HELD_ATTACK_UP through HELD_EVASION_UP — Stat berries (unused in vanilla)
- HELD_ESCAPE (Smoke Ball) — Guaranteed escape
- HELD_CRITICAL_UP (unused stat berry)

### Status Healing Effects
From `data/battle/held_heal_status.asm`:

| Held Effect | Status Cured |
|------------|-------------|
| HELD_HEAL_POISON | PSN |
| HELD_HEAL_FREEZE | FRZ |
| HELD_HEAL_BURN | BRN |
| HELD_HEAL_SLEEP | SLP (any sleep counter) |
| HELD_HEAL_PARALYZE | PAR |
| HELD_HEAL_STATUS | All status conditions |

### Stat-Up Items
From `data/battle/held_stat_up.asm`. When triggered, these items raise a stat by 1 stage:

| Held Effect | Stat | Battle Command |
|------------|------|---------------|
| HELD_ATTACK_UP | Attack | AttackUp |
| HELD_DEFENSE_UP | Defense | DefenseUp |
| HELD_SPEED_UP | Speed | SpeedUp |
| HELD_SP_ATTACK_UP | Sp. Attack | SpecialAttackUp |
| HELD_SP_DEFENSE_UP | Sp. Defense | SpecialDefenseUp |
| HELD_ACCURACY_UP | Accuracy | AccuracyUp |
| HELD_EVASION_UP | Evasion | EvasionUp |

---

## AI Scoring System

The AI uses layered scoring. Each AI layer modifies a score value for each of the 4 moves. Lower score = more likely to be chosen.

### AI_BASIC (Layer 1)
- **Status-only effects discouraged** if foe already has status: Sleep, Toxic, Poison, Paralyze effects get +5 penalty

### AI_CAUTIOUS (Layer 2)
- **Residual moves discouraged** after first turn. These moves get penalized on later turns:
  - Mist, Leech Seed, PoisonPowder, Stun Spore, Thunder Wave
  - Focus Energy, Bide, Poison Gas, Transform, Conversion, Substitute, Spikes

### AI_AGGRESSIVE (Layer 3)
- Encourages the strongest move available
- **Reckless moves exempt** from "strongest move" check: Self-Destruct/Explosion, Rampage moves (Thrash/Petal Dance), Multi-Hit, Double Hit

### AI_OPPORTUNIST (Layer 4)
- **Stall moves discouraged** when HP is low. At low HP, the following get penalized:
  - Swords Dance, Tail Whip, Leer, Growl, Disable, Mist, Counter, Leech Seed
  - Growth, String Shot, Meditate, Agility, Rage, Mimic, Screech, Harden
  - Withdraw, Defense Curl, Barrier, Light Screen, Haze, Reflect
  - Focus Energy, Bide, Amnesia, Transform, Splash, Acid Armor
  - Sharpen, Conversion, Substitute, Flame Wheel

### AI_RISKY (Layer 5)
- Will not use risky moves at max HP even if they KO:
  - Self-Destruct/Explosion effects
  - OHKO moves (Fissure, Guillotine, Horn Drill)

### AI_SMART (Layer 6)
The most complex layer. Key behaviors:

#### Encourage Rain Dance with Water/Thunder moves:
Water Gun, Hydro Pump, Surf, BubbleBeam, Thunder, Waterfall, Clamp, Bubble, Crabhammer, Octazooka, Whirlpool

#### Encourage Sunny Day with Fire/recovery moves:
Fire Punch, Ember, Flamethrower, Fire Spin, Fire Blast, Sacred Fire, Morning Sun, Synthesis
**BUG:** Does NOT encourage Sunny Day with SolarBeam, Flame Wheel, or Moonlight

#### Encourage Encore when foe uses setup moves:
Swords Dance, Whirlwind, Leer, Roar, Disable, Mist, Leech Seed, Growth, PoisonPowder, String Shot, Meditate, Agility, Teleport, Screech, Haze, Focus Energy, Dream Eater, Poison Gas, Splash, Sharpen, Conversion, Super Fang, Substitute, Triple Kick, Spider Web, Mind Reader, Flame Wheel, Aeroblast, Cotton Spore, Powder Snow

#### Use Mirror Move/Mimic/Disable after good enemy moves:
Double-Edge, Sing, Flamethrower, Hydro Pump, Surf, Ice Beam, Blizzard, Hyper Beam, Sleep Powder, Thunderbolt, Thunder, Earthquake, Toxic, Psychic, Hypnosis, Recover, Fire Blast, Softboiled, Super Fang

---

## Constant Damage Effects (AI special handling)

From `data/battle/ai/constant_damage_effects.asm`. AI uses special damage calculation for:
- Super Fang (halves HP)
- Static Damage (SonicBoom = 20, Dragon Rage = 40)
- Level Damage (Seismic Toss, Night Shade = user's level)
- Psywave (random 1x to 1.5x level)

---

## Stat Names

From `data/battle/stat_names.asm`:
- ATTACK, DEFENSE, SPEED, SP_ATTACK, SP_DEFENSE, ACCURACY, EVASION
