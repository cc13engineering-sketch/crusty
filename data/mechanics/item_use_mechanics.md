# Pokemon Crystal -- Item Use Mechanics

Source: pokecrystal disassembly (engine/items/*.asm)

---

## Item Effect Dispatch

Each item has an entry in the ItemEffects jump table (item_effects.asm). Items are categorized by their effect function:

### Effect Categories

| Effect Function | Items |
|----------------|-------|
| PokeBallEffect | Master Ball, Ultra Ball, Great Ball, Poke Ball, Heavy Ball, Level Ball, Lure Ball, Fast Ball, Friend Ball, Moon Ball, Love Ball, Park Ball |
| RestoreHPEffect | Max Potion, Hyper Potion, Super Potion, Potion, Fresh Water, Soda Pop, Lemonade, Moomoo Milk, RageCandyBar, Berry, Gold Berry, Berry Juice |
| FullRestoreEffect | Full Restore |
| StatusHealingEffect | Antidote, Burn Heal, Ice Heal, Awakening, Parlyz Heal, Full Heal, PsnCureBerry, PrzCureBerry, Burnt Berry, Ice Berry, Mint Berry, MiracleBerry |
| ReviveEffect | Revive, Max Revive |
| VitaminEffect | HP Up, Protein, Iron, Carbos, Calcium |
| RareCandyEffect | Rare Candy |
| XItemEffect | X Attack, X Defend, X Speed, X Special |
| XAccuracyEffect | X Accuracy |
| GuardSpecEffect | Guard Spec |
| DireHitEffect | Dire Hit |
| EvoStoneEffect | Moon Stone, Fire Stone, Thunderstone, Water Stone, Leaf Stone, Sun Stone |
| RepelEffect | Repel |
| SuperRepelEffect | Super Repel |
| MaxRepelEffect | Max Repel |
| EscapeRopeEffect | Escape Rope |
| RestorePPEffect | PP Up, Ether, Max Ether, Elixer, Max Elixer, Mystery Berry |
| BitterBerryEffect | Bitter Berry |
| SacredAshEffect | Sacred Ash |
| NoEffect | All held items, key items without use effects, mail |

---

## Poke Ball Catch Formula

### Catch Rate Calculation

```
Step 1: Get base catch rate of species (wEnemyMonCatchRate = 0-255)

Step 2: Apply ball-specific multiplier
  - Each special ball calls its own function from BallMultiplierFunctionTable
  - Result stored in register b (modified catch rate)

Step 3: HP factor calculation (skipped for Level Ball)
  MaxHP_x2 = EnemyMonMaxHP * 2
  CurHP_x3 = EnemyMonHP * 3

  If MaxHP_x2 > 255:
    MaxHP_x2 >>= 2
    CurHP_x3 >>= 2

  If CurHP_x3 == 0: CurHP_x3 = 1

  adjusted_rate = (catch_rate * (MaxHP_x2 - CurHP_x3)) / MaxHP_x2

Step 4: Status bonus
  If Frozen or Asleep: add 10
  Else if BRN/PSN/PAR: add 0 (BUG: code checks non-zero after AND with FRZ|SLP mask, so BRN/PSN/PAR add nothing)

Step 5: Random check
  Generate random byte (0-255)
  If random < adjusted_rate: caught!
  If random >= adjusted_rate: failed

Step 6: Master Ball always catches, Tutorial always catches
```

### Known Catch Formula Bugs
1. **BRN/PSN/PAR do not affect catch rate** -- The status check ANDs with FRZ|SLP mask first, then checks non-zero for the +5 bonus. BRN/PSN/PAR are never tested.
2. **HELD_CATCH_CHANCE has no effect** -- The battle mon's held item effect is read but the result is never properly applied (push/pop clobbers the comparison).
3. **Catch rate breaks for Pokemon with max HP > 341** -- Integer overflow in the HP calculation.
4. **Catching a Transformed Pokemon always catches Ditto** -- The SUBSTATUS_TRANSFORMED check defaults to DITTO.

### Ball-Specific Multipliers

Each special ball has a function in BallMultiplierFunctionTable that modifies the catch rate (register b):

| Ball | Modifier Logic |
|------|---------------|
| Master Ball | Always catches (bypasses formula) |
| Ultra Ball | Catch rate * 2 |
| Great Ball | Catch rate * 1.5 |
| Poke Ball | No modification |
| Level Ball | Compare levels: if player >= 4x enemy: rate*4; >= 2x: rate*2; else: rate*1 (BUG: off-by-one in comparison) |
| Lure Ball | If fishing: rate * 3; else: no modification |
| Moon Ball | If species evolves with Moon Stone: rate * 4 (BUG: checks wrong item constant) |
| Friend Ball | No catch rate bonus; sets initial happiness to 200 |
| Fast Ball | If species has flee flag: rate * 4; else: no modification |
| Heavy Ball | Based on species weight: heavy Pokemon get +20/+30/+40 bonus, light ones get -20 |
| Love Ball | If same species + opposite gender: rate * 8; else: no modification |
| Park Ball | Used in Bug Catching Contest; standard catch rate |

### Post-Catch Processing

After successful catch:
1. Pokemon added to party (if room) or sent to current PC box
2. Friend Ball sets happiness to 200 (FRIEND_BALL_HAPPINESS)
3. Caught data set (OT name, ID, location, time)
4. Pokedex updated (caught flag set)
5. If new species: Pokedex entry displayed
6. Player asked to give nickname

---

## HP Restoration Items

### Restore Amounts

| Item | HP Restored |
|------|-------------|
| Potion | 20 |
| Super Potion | 50 |
| Hyper Potion | 200 |
| Max Potion | Full HP |
| Full Restore | Full HP + cure all status |
| Fresh Water | 50 |
| Soda Pop | 60 |
| Lemonade | 80 |
| Moomoo Milk | 100 |
| RageCandyBar | 20 |
| Berry | 10 |
| Gold Berry | 30 |
| Berry Juice | 20 |
| Energy Root | 200 (happiness penalty) |
| Energypowder | 50 (happiness penalty) |

### Bitter Medicine Happiness Penalties
- Energypowder: -5 happiness (friendship < 200), -10 (friendship >= 200)
- Energy Root: -10 happiness (friendship < 200), -15 (friendship >= 200)
- Heal Powder: -5 happiness
- Revival Herb: -10 or -15 happiness

---

## Status Healing Items

| Item | Cures |
|------|-------|
| Antidote | Poison |
| Burn Heal | Burn |
| Ice Heal | Freeze |
| Awakening | Sleep |
| Parlyz Heal | Paralysis |
| Full Heal | All status conditions |
| Full Restore | All status + full HP |
| PsnCureBerry | Poison (auto-use in battle as held item) |
| PrzCureBerry | Paralysis (auto-use) |
| Burnt Berry | Freeze (auto-use) |
| Ice Berry | Burn (auto-use) -- NOTE: counterintuitive name |
| Mint Berry | Sleep (auto-use) |
| MiracleBerry | Any status (auto-use) |
| Bitter Berry | Confusion |

---

## PP Restoration Items

| Item | Effect |
|------|--------|
| Ether | Restores 10 PP to one move |
| Max Ether | Fully restores PP to one move |
| Elixer | Restores 10 PP to all moves |
| Max Elixer | Fully restores PP to all moves |
| PP Up | Permanently increases max PP of one move by 20% of base (up to 3 times = +60%) |
| Mystery Berry | Restores 5 PP to one move (auto-use as held item when PP hits 0) |

---

## Battle Items (X Items)

Used in battle to temporarily boost stats:

| Item | Effect |
|------|--------|
| X Attack | +1 Attack stage |
| X Defend | +1 Defense stage |
| X Speed | +1 Speed stage |
| X Special | +1 Special Attack stage |
| X Accuracy | +1 Accuracy stage |
| Guard Spec | Prevents stat reduction for 5 turns |
| Dire Hit | Increases critical hit rate by +1 stage |

These items call the same stat modification routines as stat-boosting moves.

---

## Evolution Stones

Moon Stone, Fire Stone, Thunderstone, Water Stone, Leaf Stone, Sun Stone

- Triggered by using the stone from the bag on a compatible Pokemon
- Uses the EVOLVE_ITEM evolution method
- wForceEvolution flag is set to indicate stone usage
- The stone is consumed on successful evolution
- Will not work if the Pokemon holds an Everstone (but this is a separate check for level-up only -- stones bypass Everstone)

---

## Revive Items

| Item | Effect |
|------|--------|
| Revive | Restores a fainted Pokemon to 50% max HP |
| Max Revive | Restores a fainted Pokemon to 100% max HP |
| Revival Herb | Restores a fainted Pokemon to full HP (happiness penalty) |
| Sacred Ash | Revives ALL fainted party Pokemon to full HP |

---

## Vitamins

HP Up, Protein, Iron, Carbos, Calcium

- Each adds 2560 Stat Experience to the corresponding stat
- Maximum Stat Experience per stat: 65535
- Cannot be used if stat experience is already at 65535
- Stat recalculation happens immediately

---

## Repel Items

| Item | Steps |
|------|-------|
| Repel | 100 |
| Super Repel | 200 |
| Max Repel | 250 |

Repels prevent wild encounters with Pokemon whose level is strictly less than the lead party Pokemon's level.

---

## Item Use Validation

Items check various conditions before use:
- HP healing items: Pokemon must not be at full HP and must not be fainted
- Status heals: Pokemon must have the relevant status condition
- Revives: Pokemon must be fainted (HP = 0)
- Battle items (X Items): Must be in battle
- Evolution stones: Pokemon must be compatible
- Vitamins: Stat experience must not already be maxed
- Poke Balls: Must be in wild battle (not trainer battle), party or box must have room

---

## Pack/Bag System

### Pockets
The bag has multiple pockets (engine/items/pack.asm):
1. **Items** -- General items
2. **Key Items** -- Quest items
3. **Balls** -- All Poke Ball variants
4. **TMs/HMs** -- Technical/Hidden Machines

### Item Limits
- Maximum 20 different items per pocket (except TMs/HMs which hold all)
- Maximum 99 of any single item
- Items can be reordered within pockets via SELECT

### TM/HM System (engine/items/tmhm.asm)
- TMs are consumed on use
- HMs are not consumed (infinite use)
- HM moves cannot be forgotten through normal means (ForgetMove checks IsHMMove)
- Compatibility check: each species has a TM/HM learnability bitfield

---

## Mart System (engine/items/mart.asm, buy_sell_toss.asm)

### Buy
- Items purchased at marked-up prices
- Price displayed before purchase
- Quantity selection available
- Cannot exceed bag pocket capacity

### Sell
- Items sold at half purchase price
- Key items and HMs cannot be sold
- Mail cannot be sold while attached to Pokemon

### Toss
- Items can be discarded from bag
- Key items cannot be tossed
- Confirmation prompt before tossing
