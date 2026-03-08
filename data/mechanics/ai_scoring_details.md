# Pokemon Crystal -- AI Scoring Details

Source: pokecrystal disassembly (engine/battle/ai/scoring.asm, ai/move.asm, ai/switch.asm, ai/items.asm, ai/redundant.asm)

---

## AI System Overview

### Score System
- Each of the enemy's 4 moves starts with a base score of **20**
- Disabled or 0-PP moves get score **80** (effectively never chosen)
- Up to **16 AI scoring layers** are applied based on trainer class attributes
- Each layer can increment (discourage) or decrement (encourage) a move's score
- After all layers, scores are decremented round-robin until one reaches 0
- Ties are broken by re-incrementing partially decremented scores
- A random move among the lowest-scoring moves is chosen
- **Lower score = better move**

### Score Modification Conventions
- `dec [hl]` = encourage (-1)
- `inc [hl]` = discourage (+1)
- `dec [hl]` x2 = greatly encourage (-2)
- `inc [hl]` x2 = greatly discourage (+2)
- `AIDiscourageMove` = add 10 to score (essentially dismiss)
- `dec [hl]` x5 = maximally encourage (used by AI_Risky for KO moves)

### Probability Helpers
- `AI_50_50`: 50% chance (returns carry)
- `AI_80_20`: 80% chance (returns carry ~20% of the time, so 80% proceeds)
- `BattleRandom`: generates random byte 0-255
- Percentages like `cp 10 percent` compare against proportional byte values

---

## AI Scoring Layers (AIScoringPointers)

### Layer 1: AI_Basic
**Purpose:** Filter obviously bad moves.
- Dismiss status-only moves if the player already has a status condition
- Dismiss status moves if the player has Safeguard active
- Uses `AI_Redundant` to check if a move's effect is already in play (e.g. Light Screen already up, already seeded, already confused)

### Layer 2: AI_Setup
**Purpose:** Encourage stat moves on first turns.
- 50% chance to greatly encourage stat-up moves on enemy's first turn
- 50% chance to greatly encourage stat-down moves on player's first turn
- ~88% chance to greatly discourage stat-modifying moves on later turns

### Layer 3: AI_Types
**Purpose:** Type effectiveness awareness.
- Dismiss immune moves (add 10)
- Encourage super-effective damaging moves (-1)
- Discourage not-very-effective damaging moves (+1) ONLY if the enemy has at least one damaging move of a different type

### Layer 4: AI_Offensive
**Purpose:** Prefer damaging moves.
- Greatly discourage (+2) any move with 0 base power

### Layer 5: AI_Smart
**Purpose:** Context-specific per-effect scoring. The most complex layer with 80+ effect handlers.

### Layer 6: AI_Opportunist
**Purpose:** Discourage stalling when low on HP.
- If HP < 25%: discourage stall moves (+1)
- If HP 25-50%: 50% chance to discourage stall moves (+1)
- Stall moves defined in `data/battle/ai/stall_moves.asm`

### Layer 7: AI_Aggressive
**Purpose:** Favor highest-damage move.
- Calculates actual damage for each damaging move (using `AIDamageCalc`)
- Discourages (+1) all damaging moves except the one that does the most damage
- Ignores moves with power 0-1 (Seismic Toss, Counter, etc.)
- Ignores reckless moves (Selfdestruct, etc.) from the comparison

### Layer 8: AI_Cautious
**Purpose:** Discourage residual effects after turn 1.
- 90% chance to discourage (+1) residual-effect moves after the first turn
- Residual moves defined in `data/battle/ai/residual_moves.asm`
- **BUG:** The `ret nc` after the random check should be `jr nc, .loop`, causing the function to sometimes return early without checking all moves

### Layer 9: AI_Status
**Purpose:** Don't use status moves the target is immune to.
- Dismiss Toxic/Poison against Poison-type targets
- Dismiss any status or damaging move with 0 type matchup (immune)

### Layer 10: AI_Risky
**Purpose:** Greatly encourage KO moves.
- Calculates damage for each damaging move
- If damage >= player's remaining HP: maximally encourage (-5)
- Risky moves (Self-Destruct effects) at max HP: skip or 80% chance to skip
- Risky effects defined in `data/battle/ai/risky_effects.asm`

### Layer 11-16: AI_None
- Placeholder layers that do nothing

---

## AI_Smart Effect Handlers (Complete Reference)

### Sleep Moves (EFFECT_SLEEP)
- If enemy has Dream Eater or Nightmare: 50% chance to greatly encourage (-2)
- Otherwise: 50% chance to greatly encourage (-2)

### Leech Hit (EFFECT_LEECH_HIT)
- Not very effective: 60% chance to discourage (+1)
- Super effective AND enemy HP not full: 80% chance to encourage (-1)

### Selfdestruct/Explosion (EFFECT_SELFDESTRUCT)
- If enemy's last mon AND player's last mon: don't discourage
- If enemy's last mon AND player has others: greatly discourage (+3)
- If enemy HP > 50%: greatly discourage (+3)
- If enemy HP < 25%: do nothing
- If enemy HP 25-50%: ~92% chance to greatly discourage (+3)

### Dream Eater (EFFECT_DREAM_EATER)
- 90% chance to greatly encourage (-3)
- (AI_Basic already filters this to sleeping targets only)

### Evasion Up (EFFECT_EVASION_UP)
- At max evasion: dismiss (+10)
- Full HP + player toxic: greatly encourage (-2)
- Full HP: 70% chance to greatly encourage (-2)
- HP < 25%: greatly discourage (+2)
- HP 25-50%: 50% chance to greatly discourage (+2)
- HP 50-100%: 20% chance to greatly encourage (-2)
- Player toxic: 70% chance to greatly encourage (-2)
- Player seeded: 50% chance to encourage (-1)
- Enemy evasion > player accuracy: discourage (+1)
- Player in Fury Cutter/Rollout: greatly encourage (-2)

### Always Hit (EFFECT_ALWAYS_HIT)
- 80% chance to greatly encourage (-2) if enemy accuracy < -2 or player evasion > +2

### Mirror Move (EFFECT_MIRROR_MOVE)
- Player didn't move last turn: dismiss if enemy faster, nothing if slower
- Player used a useful move: 50% chance to encourage (-1), then 90% chance to encourage again (-1) if enemy is faster

### Accuracy Down (EFFECT_ACCURACY_DOWN)
- Same complex HP-based logic as Evasion Up (mirrors it)

### Reset Stats / Haze (EFFECT_RESET_STATS)
- 85% chance to encourage (-1) if any enemy stat < -2 or any player stat > +2
- Discourage (+1) otherwise

### Bide (EFFECT_BIDE)
- 90% chance to discourage (+1) unless enemy HP is full

### Force Switch / Roar/Whirlwind (EFFECT_FORCE_SWITCH)
- Discourage (+1) if the player hasn't shown a super-effective move

### Heal / Morning Sun / Synthesis / Moonlight
- Enemy HP < 25%: 90% chance to greatly encourage (-2)
- Enemy HP > 50%: discourage (+1)

### Toxic / Leech Seed (EFFECT_TOXIC / EFFECT_LEECH_SEED)
- Discourage (+1) if player HP < 50%

### Light Screen / Reflect
- ~92% chance to discourage (+1) unless enemy HP is full

### OHKO Moves (EFFECT_OHKO)
- Dismiss (+10) if player level > enemy level
- Discourage (+1) if player HP < 50%

### Trap Target / Bind/Wrap/Fire Spin (EFFECT_TRAP_TARGET)
- Player already trapped: 50% chance to discourage (+1)
- Player toxic/love/identified/Rollout/Nightmare: 50% chance to greatly encourage (-2) if enemy HP > 25%
- Player's first turn: 50% chance to greatly encourage (-2) if enemy HP > 25%

### Confuse (EFFECT_CONFUSE)
- Player HP < 50%: 90% chance to discourage (+1)
- Player HP < 25%: discourage again (+1)

### Sp.Def Up 2 (EFFECT_SP_DEF_UP_2)
- Enemy HP < 50%: discourage (+1)
- Enemy SpDef > +3: discourage (+1)
- Enemy SpDef < +2 AND player is special type: 80% chance to greatly encourage (-2)

### Fly/Dig (EFFECT_FLY)
- Player flying/underground AND enemy faster: greatly encourage (-3)

### Super Fang (EFFECT_SUPER_FANG)
- Discourage (+1) if player HP < 25%

### Paralyze (EFFECT_PARALYZE)
- Player HP < 25%: 50% chance to discourage (+1)
- Enemy slower AND enemy HP > 25%: 80% chance to greatly encourage (-2)

### Icy Wind / Speed Down Hit (EFFECT_SPEED_DOWN_HIT)
- Only for Icy Wind specifically
- ~88% chance to greatly encourage (-2) if: enemy HP > 25%, player's first turn, player faster

### Substitute (EFFECT_SUBSTITUTE)
- Dismiss (+10) if enemy HP < 50%

### Hyper Beam (EFFECT_HYPER_BEAM)
- Enemy HP > 50%: random discourage (+1 or +2)
- Enemy HP < 25%: 50% chance to encourage (-1)

### Rage (EFFECT_RAGE)
- Rage building: 50% chance to encourage (-1), plus counter-based bonuses
- Not building + HP > 50%: 20% chance to encourage (-1)
- Not building + HP < 50%: discourage (+1)

### Mimic (EFFECT_MIMIC)
- Player didn't move: dismiss if faster, discourage if slower
- Player used super-effective move: 50% chance to double encourage
- Player used useful move: 50% chance to encourage (-1)

### Counter (EFFECT_COUNTER)
- Count player's known physical damaging moves
- 3+ physical moves: ~60% chance to encourage (-1)
- Last player move was physical damaging: ~60% chance to encourage (-1)
- No physical moves: discourage (+1)

### Mirror Coat (EFFECT_MIRROR_COAT)
- Same as Counter but checks for special (type >= SPECIAL) moves instead of physical

### Encore (EFFECT_ENCORE)
- Enemy slower: greatly discourage (+3)
- Player's last move has no power AND is not very effective against enemy: ~72% chance to greatly encourage (-2)
- Player's last move is in EncoreMoves list: ~72% chance to greatly encourage (-2)
- Otherwise: greatly discourage (+3)

### Pain Split (EFFECT_PAIN_SPLIT)
- Discourage (+1) if enemy HP * 2 > player HP

### Snore / Sleep Talk (EFFECT_SNORE / EFFECT_SLEEP_TALK)
- If asleep with >1 turn remaining: greatly encourage (-3)
- Otherwise: greatly discourage (+3)

### Spite (EFFECT_SPITE)
- Player didn't use a move: if faster, dismiss; if slower, 50% chance to discourage
- Player's last move PP < 6: ~60% chance to greatly encourage (-2)
- Player's last move PP >= 15: discourage (+1)

### Destiny Bond / Reversal / Skull Bash
- Discourage (+1) if enemy HP > 25%

### Heal Bell (EFFECT_HEAL_BELL)
- No team status: dismiss (+10) unless current mon is statused
- Current mon statused: encourage (-1)
- Current mon frozen or asleep: 50% chance to greatly encourage (-2)

### Priority Hit / Quick Attack/Mach Punch (EFFECT_PRIORITY_HIT)
- If enemy already faster: return (no bonus)
- If player flying/underground: dismiss (+10)
- If move will KO (calculates actual damage): greatly encourage (-3)

### Thief (EFFECT_THIEF)
- Always add $1e (30) to score -- essentially never use unless it's the only option

### Conversion2 (EFFECT_CONVERSION2)
- **BUG:** Checks `wLastPlayerMove` but the condition `jr nz, .discourage` means it discourages WHEN the player HAS used a move, which is backwards
- If player's move is super effective: 50% chance to encourage (-1)
- Otherwise: 90% chance to discourage (+1)

### Disable (EFFECT_DISABLE)
- Enemy faster + player's last move is useful: ~60% chance to encourage (-1)
- Otherwise: ~92% chance to discourage (+1)

### Mean Look (EFFECT_MEAN_LOOK)
- Enemy HP < 50%: discourage (+1)
- Player is last mon: dismiss (+10)
- **BUG:** Checks enemy's own toxic status instead of player's
- Player in love/identified/Rollout/Nightmare: 80% chance to greatly encourage (-3)
- Player has only NVE moves: return (don't encourage)

### Nightmare (EFFECT_NIGHTMARE)
- 50% chance to encourage (-1)

### Flame Wheel (EFFECT_FLAME_WHEEL)
- If enemy is frozen: maximally encourage (-5)

### Curse (EFFECT_CURSE)
- **Non-Ghost:** HP < 50%: discourage. Attack > +3: discourage. Player is Ghost: greatly discourage. Player is physical type: 80% chance to greatly encourage.
- **Ghost:** Player already cursed: dismiss. Enemy's last mon + not player's last: highly discourage (+4). Enemy HP < 25%: highly discourage. Enemy HP full + first turn: 50% greatly encourage (-2).

### Protect (EFFECT_PROTECT)
- Already used Protect: greatly discourage (+2)
- Player locked on: discourage (+1)
- Player Fury Cutter >= 3, charged, toxic, seeded, cursed, or Rollout >= 3: 80% chance to encourage (-1)

### Foresight (EFFECT_FORESIGHT)
- Enemy accuracy < -2 or player evasion > +2 or player is Ghost: 60% chance to greatly encourage (-2)
- Otherwise: 92% chance to discourage (+1)

### Perish Song (EFFECT_PERISH_SONG)
- Enemy is last mon: add 5 to score (heavily dismiss)
- Player trapped (Mean Look): 50% chance to encourage (-1)
- Player has super-effective moves: 50% chance to discourage (+1)

### Sandstorm (EFFECT_SANDSTORM)
- Player is Rock/Ground/Steel: greatly discourage (+2)
- Player HP < 50%: discourage (+1)
- Otherwise: 50% chance to encourage (-1)

### Endure (EFFECT_ENDURE)
- Already used Protect: greatly discourage (+2)
- Enemy HP full: greatly discourage (+2)
- Enemy HP > 25%: discourage (+1)
- Enemy has Reversal: 80% chance to greatly encourage (-3)
- Enemy locked on: 50% chance to greatly encourage (-2)

### Fury Cutter (EFFECT_FURY_CUTTER)
- Bonus based on current counter: count 1 = -1, count 2 = -3, count >= 3 = -6
- Then falls through to Rollout scoring

### Rollout (EFFECT_ROLLOUT)
- In love, confused, or paralyzed: 80% chance to discourage (+1)
- HP < 25%, low accuracy, or high player evasion: 80% chance to discourage (+1)
- Otherwise: ~79% chance to greatly encourage (-2)

### Swagger / Attract
- Player's first turn: ~79% chance to encourage (-1)
- Later turns: 80% chance to discourage (+1)

### Safeguard (EFFECT_SAFEGUARD)
- 80% chance to discourage (+1) if player HP < 50%

### Magnitude / Earthquake
- Player used Dig last turn AND is underground AND enemy faster: greatly encourage (-2)
- Player used Dig but not underground AND enemy slower: 50% chance to encourage (-1)

### Baton Pass (EFFECT_BATON_PASS)
- Discourage (+1) if player hasn't shown super-effective moves

### Pursuit (EFFECT_PURSUIT)
- Player HP < 25%: 50% chance to greatly encourage (-2)
- Otherwise: 80% chance to discourage (+1)

### Rapid Spin (EFFECT_RAPID_SPIN)
- If enemy is trapped, seeded, or Spikes are up: 80% chance to greatly encourage (-2)

### Hidden Power (EFFECT_HIDDEN_POWER)
- Calculates actual type/power from DVs
- Not very effective OR power < 50: discourage (+1)
- Super effective: encourage (-1)
- Power >= 70: encourage (-1)

### Rain Dance (EFFECT_RAIN_DANCE)
- Player is Water: greatly discourage (+3)
- Player is Fire: greatly encourage (-2) if player HP > 50% and first turn
- Enemy has a rain-boosted move AND player HP > 50%: 50% chance to encourage (-1)

### Sunny Day (EFFECT_SUNNY_DAY)
- Player is Fire: greatly discourage (+3)
- Player is Water: greatly encourage (-2) if player HP > 50% and first turn
- Enemy has a sun-boosted move AND player HP > 50%: 50% chance to encourage (-1)

### Belly Drum (EFFECT_BELLY_DRUM)
- Attack > +2 or HP < 50%: add 5 (heavily dismiss)
- HP not full: discourage (+1)

### Psych Up (EFFECT_PSYCH_UP)
- Sums all stat levels for both sides
- Enemy stats >= player stats: greatly discourage (+2)
- Player accuracy < -1 or enemy evasion > 0: return
- Otherwise: 80% chance to encourage (-1)

### Solarbeam (EFFECT_SOLARBEAM)
- Sunny: 80% chance to greatly encourage (-2)
- Rain: 90% chance to greatly discourage (+2)

### Thunder (EFFECT_THUNDER)
- Sunny: 90% chance to discourage (+1)

### Twister / Gust
- Player used Fly last turn AND flying AND enemy faster: greatly encourage (-2)
- Player used Fly AND enemy slower: 50% chance to encourage (-1)

### Future Sight (EFFECT_FUTURE_SIGHT)
- Player flying/underground AND enemy faster: greatly encourage (-2)

### Stomp (EFFECT_STOMP)
- 80% chance to encourage (-1) if player used Minimize

---

## AI Switch Logic (switch.asm)

### CheckAbleToSwitch
Called to determine if the AI should switch. Evaluates:
1. Whether the enemy has alive mons to switch to
2. Perish Song at count 1: always try to switch
3. Player has super-effective moves: try to switch
4. Search for a mon that: has > 25% HP, resists the player, has a super-effective move

### Switch Frequency (from trainer attributes)

| Attribute | Low Priority (score $10) | Med Priority ($20) | High Priority ($30) |
|-----------|--------------------------|---------------------|---------------------|
| SwitchOften | 50% switch | 79% switch | 96% switch |
| SwitchSometimes | 20% switch | 50% switch | 80% switch |
| SwitchRarely | 8% switch | 12% switch | 79% switch |

### Switch Candidate Selection
1. `FindAliveEnemyMons` -- find all non-fainted mons
2. `FindEnemyMonsWithAtLeastQuarterMaxHP` -- filter to > 25% HP
3. `FindEnemyMonsThatResistPlayer` -- filter to those resisting player's shown moves
4. `FindAliveEnemyMonsWithASuperEffectiveMove` -- further filter to those with SE moves

---

## AI Item Usage (items.asm)

### AI_TryItem
- Not used in Battle Tower (items banned)
- Checks `wEnemyTrainerItem1` and `wEnemyTrainerItem2`
- Only uses items on the highest-level party member
- Looks up item in `AI_Items` table
- Each item has a condition check and use routine
- **BUG:** Base reward value can accidentally be read as an item due to pointer math

### AI Item Categories
Trainers carry up to 2 items, used based on HP thresholds and status conditions:
- Full Heal: used when current mon is statused
- Full Restore: used when HP is critical (varies by trainer)
- Potions: used at various HP thresholds
- X Items: used to boost stats

---

## AI Redundancy Checks (redundant.asm)

### AI_Redundant
Checks if a move effect is already active/useless:

| Effect | Redundant When |
|--------|---------------|
| Light Screen | Already active on enemy side |
| Mist | Already has Mist substatus |
| Focus Energy | Already has Focus Energy |
| Confuse | Player already confused OR has Safeguard |
| Transform | Already transformed |
| Reflect | Already active on enemy side |
| Substitute | Already has Substitute |
| Leech Seed | Player already seeded |
| Disable | Player already disabled |
| Encore | Player already encored |
| Snore/Sleep Talk | Enemy NOT asleep (redundant) |
| Mean Look | Player already trapped |
| Nightmare | Player not asleep OR already has Nightmare |
| Spikes | Already have Spikes on player side |
| Foresight | Player already identified |
| Perish Song | Player already has Perish count |
| Sandstorm | Weather already Sandstorm |
| Attract | Can't attract (same gender/genderless) OR player already attracted |
| Safeguard | Already active on enemy side |
| Rain Dance | Weather already Rain |
| Sunny Day | Weather already Sun |
| Dream Eater | Player NOT asleep |
| Swagger | Player already confused |
| Future Sight | **BUG:** checks SCREENS_UNUSED bit instead of future sight count |
| Heal/Morning Sun/Synthesis/Moonlight | Enemy HP already full |
| Teleport | Always redundant (NPC trainers can't flee) |

---

## Move Selection (move.asm)

### AIChooseMove
1. Wild Pokemon: attack completely at random (return immediately)
2. Link battles: return (link opponent handles their own moves)
3. If locked in (Rollout, Outrage, etc.): return
4. Initialize all 4 scores to 20; disabled/no-PP moves get 80
5. Apply each enabled AI layer from TrainerClassAttributes
6. Decrement all scores round-robin until one reaches 0
7. Re-increment to resolve ties (minimum viable score = 1)
8. Zero out all non-minimum scores
9. Randomly pick among tied minimum-score moves

### Layer Application
- Trainer class attributes are stored as a 16-bit flag field
- Each bit enables one AI layer (layers 1-16)
- Battle Tower always uses the first trainer class's attributes (Falkner's)
