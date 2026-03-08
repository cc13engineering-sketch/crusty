# Pokemon Crystal -- Core Battle Flow Reference

Source: pokecrystal disassembly (engine/battle/core.asm, engine/battle/effect_commands.asm)

---

## Turn Structure

Each battle turn follows this sequence:

### 1. Player Input Phase
- Player selects: Fight / Item / Switch / Run
- If Encored: forced to use the last move used
- If locked in (Rollout, Outrage, Bide, recharging): skip selection

### 2. Enemy AI Phase
- AI selects move or switches (via AI_SwitchOrTryItem)
- AI can use items (potions, status heals) for trainer battles

### 3. Move Order Determination (DetermineMoveOrder)

Priority is determined in this strict order:

**a) Switching priority**
- If either side switches, the switcher goes second (the other side attacks the switch-in)
- If both sides switch (link battles), random order (50/50)

**b) Move priority check (CompareMovePriority)**
- Priority levels in Gen 2:
  - +2: (none in Gen 2)
  - +1: Mach Punch, Quick Attack, ExtremeSpeed
  - 0: All regular moves
  - -1: Counter, Mirror Coat, Vital Throw
  - -6: (none in Gen 2 -- Roar/Whirlwind are priority 0)
- Higher priority ALWAYS goes first

**c) Quick Claw check (for equal priority)**
- If one side has Quick Claw: random chance to go first (based on item's activation rate)
- If both sides have Quick Claw: each side rolls independently, speed check if neither activates

**d) Speed comparison (for equal priority, no Quick Claw)**
- Compare 16-bit Speed stats directly (wBattleMonSpeed vs wEnemyMonSpeed)
- Includes stat stage modifications, paralysis quartering
- Higher speed goes first

**e) Speed tie**
- Random 50/50 coin flip

### 4. Execute Moves

For each battler (in the determined order):

#### Pre-Move Checks (CheckTurn / CheckEnemyTurn)

These are checked in this EXACT order, and each can end the turn:

1. **Recharge**: If recharging (Hyper Beam), skip turn and clear recharge flag
2. **Sleep**: Decrement sleep counter. If counter > 0 and not using Snore/Sleep Talk, skip turn. If counter reaches 0, wake up (Pokemon CAN act this turn after waking)
3. **Freeze**: If frozen and not using Flame Wheel/Sacred Fire, skip turn entirely. (NO random thaw check in CheckTurn -- thaw is handled separately via HandleDefrost between turns at 10% per end-of-turn)
4. **Flinch**: If flinched, skip turn. Flinch flag is cleared
5. **Disable**: Decrement disable counter. If expires, clear disabled move
6. **Confusion**: Decrement confusion counter. If > 0, 50% chance of self-hit. Self-hit uses 40 power typeless physical move with user's own Attack/Defense
7. **Attract**: If infatuated, 50% chance of being immobilized
8. **Disabled move check**: If trying to use a disabled move, fail
9. **Paralysis**: 25% chance of full paralysis (can't move)

#### Move Execution (DoMove)

The move effect script is read and executed command by command. Common command sequence for a basic damaging move:

```
checkobedience -> usedmovetext -> doturn -> critical -> damagestats ->
damagecalc -> stab -> damagevariation -> checkhit -> hittarget ->
failuretext -> checkfaint -> criticaltext -> supereffectivetext ->
checkfaint -> buildopponentrage -> endmove
```

#### Obedience Check (BattleCommand_CheckObedience)
- Only applies to the player's Pokemon
- Only for Pokemon whose OT ID doesn't match the player's
- Badge obedience levels: No badges = 10, Hive = 30, Fog = 50, Storm = 70, Rising = all
- Disobedient Pokemon may: use a random move, nap (fall asleep), hit itself in confusion, or do nothing

### 5. Between-Turn Effects (HandleBetweenTurnEffects)

Processed in this order after both sides have acted:

1. **Check faints** (player first or enemy first depending on link clock)
2. **Future Sight** damage delivery (3 turns after setup)
3. **Check faints** again
4. **Weather** damage (Sandstorm: 1/8 HP to non-Rock/Ground/Steel types)
5. **Check faints** again
6. **Wrap** damage (1/16 HP per turn for trapped Pokemon)
7. **Check faints** again
8. **Perish Song** countdown (decrement; faint at 0)
9. **Check faints** again
10. **Leftovers** healing (1/16 max HP per turn)
11. **Mystery Berry** PP restoration
12. **Defrost** check (20% chance to thaw if frozen -- note: this is the ONLY random thaw, not in CheckTurn)
13. **Safeguard** countdown
14. **Reflect/Light Screen** countdown (expire after 5 turns)
15. **Stat-boosting held items** (not applicable in Gen 2 base game)
16. **Healing items** (Berry, Gold Berry auto-use)
17. **Encore** countdown

### 6. Residual Damage (ResidualDamage)

Called after each individual turn (not just between-turn). Processes in order:

1. **Poison/Burn** damage: 1/8 max HP
   - Toxic: starts at 1/16 max HP, increments by 1/16 each turn (N/16 on turn N)
   - Toxic counter resets on switch
2. **Leech Seed** drain: 1/8 max HP transferred to opponent
3. **Nightmare** damage: 1/4 max HP (only while asleep)
4. **Curse** damage (Ghost-type Curse): 1/4 max HP

---

## Switching Mechanics

### Normal Switching
- Uses the player's turn (no attack that turn)
- Enemy attacks the new Pokemon (calculated against the new Pokemon's stats)
- Stat stages reset to 0
- Volatile statuses cleared: confusion, attraction, flinch, focus energy, etc.
- Toxic counter resets to regular poison
- Substitute is lost
- Non-volatile statuses persist: sleep, paralysis, burn, poison, freeze

### Pursuit Interaction
- If the opponent uses Pursuit and the user switches, Pursuit executes BEFORE the switch with doubled damage
- The switching Pokemon takes the hit at its current stats
- If Pursuit KOs, the Pokemon faints before switching
- BUG: A Pokemon fainted by Pursuit retains its old status when revived

### Baton Pass
- Passes to the replacement: stat stage changes, confusion, Focus Energy, Substitute (with its remaining HP), Mean Look/Spider Web trap, Perish Song counter, Ingrain (not in Gen 2), Lock-On/Mind Reader
- Does NOT pass: Nightmare, Disable, Attraction, Transform, Encore, wrap

### Spikes
- Deals damage when a Pokemon switches in (grounded Pokemon only)
- 1/8 max HP damage
- Only one layer exists in Gen 2

### Forced Switches
- Roar / Whirlwind: force opponent to switch to random party member
- In wild battles: end the battle (flee effect)
- The incoming Pokemon takes Spikes damage

---

## Experience Award Formula

```
EXP = b * L / 7 * (1/s) * a * t * e
```

Where:
- **b** = base experience yield of the defeated species
- **L** = level of the defeated Pokemon
- **s** = number of participants (or divided for Exp Share)
- **a** = 1.5 if trainer-owned, 1.0 if wild
- **t** = 1.5 if Pokemon is traded (different OT), 1.0 if original trainer
- **e** = 1.5 if holding Lucky Egg, 1.0 otherwise

### Exp Share Distribution
- If Exp Share is in party: experience is split
- Half goes to all participants (divided by number)
- Half goes to all Exp Share holders (divided by number of holders)
- A Pokemon can receive both shares if it participated AND holds Exp Share

### Stat Experience Gain
- Each stat gains StatExp equal to the defeated Pokemon's base stat
- Applies to all Pokemon that participated or hold Exp Share
- Stat recalculation happens on level up

---

## Wild Battle Flee Formula

```
F = (Pokemon_Speed * 32) / Enemy_Speed + 30 * Escape_Attempts
```

If F > 255 or random(0-255) < F: escape succeeds.

Each failed attempt increments the escape counter, making subsequent attempts easier.

---

## Multi-Hit Move Distribution

For moves that hit 2-5 times (Fury Attack, Pin Missile, Spike Cannon, Barrage, Comet Punch, DoubleSlap, Fury Swipes, Bone Rush):

| Hits | Probability |
|------|-------------|
| 2 | 3/8 (37.5%) |
| 3 | 3/8 (37.5%) |
| 4 | 1/8 (12.5%) |
| 5 | 1/8 (12.5%) |

Average: 3.17 hits

Double Hit moves (Bonemerang, Double Kick): always hit exactly 2 times.

---

## Protect/Detect Mechanics

Success rate decreases with consecutive uses:
- 1st use: 100% (255/256)
- 2nd consecutive: ~50% (127/256)
- 3rd consecutive: ~25% (63/256)
- Pattern: success chance halves each consecutive turn
- Using any other move resets the counter

Protect blocks ALL moves except:
- Moves that target the user (stat-up moves, Substitute, etc.)
- Transform (but it fails against Protect anyway)
- Perish Song passes through Protect

---

## Weather System

Three weather conditions exist in Gen 2:

### Rain Dance (5 turns)
- Water moves: 1.5x damage
- Fire moves: 0.5x damage
- Thunder: never misses (bypasses accuracy check entirely)
- SolarBeam: 0.5x damage AND requires charge turn
- Moonlight/Synthesis/Morning Sun: heal 1/4 max HP instead of 1/2

### Sunny Day (5 turns)
- Fire moves: 1.5x damage
- Water moves: 0.5x damage
- SolarBeam: no charge turn needed
- Moonlight/Synthesis/Morning Sun: heal 2/3 max HP instead of 1/2

### Sandstorm (5 turns)
- Deals 1/8 max HP damage per turn to non-Rock, non-Ground, non-Steel types
- No damage boost to any move type
- Moonlight/Synthesis/Morning Sun: heal 1/4 max HP instead of 1/2
