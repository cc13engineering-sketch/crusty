# Pokemon Crystal -- Trainer AI Deep Dive

Source: pokecrystal disassembly (engine/battle/ai/scoring.asm, engine/battle/ai/move.asm, engine/battle/ai/items.asm, engine/battle/ai/switch.asm)

---

## AI Architecture Overview

The AI system works through **scoring layers**. Each AI layer adjusts a score for each of the enemy's 4 moves. Lower scores = more likely to be chosen. Each move starts with a base score, and layers add/subtract from it.

After all applicable layers run, the AI selects the move with the lowest score (with random tiebreaking).

### AI Layer Assignment

Each trainer class can have up to 3 AI layers assigned (stored in trainer data). The layers are:

| Layer | Constant | Function | Description |
|-------|----------|----------|-------------|
| 1 | AI_BASIC | AI_Basic | Dismiss redundant moves |
| 2 | AI_SETUP | AI_Setup | Encourage setup on turn 1 |
| 3 | AI_TYPES | AI_Types | Consider type effectiveness |
| 4 | AI_OFFENSIVE | AI_Offensive | Discourage status moves |
| 5 | AI_SMART | AI_Smart | Context-specific scoring (the big one) |
| 6 | AI_CAUTIOUS | AI_Cautious | Risk-averse behavior |
| 7 | AI_STATUS | AI_Status | Encourage status moves |
| 8 | AI_RISKY | AI_Risky | High-risk high-reward behavior |

---

## AI Layer Details

### AI_Basic (Layer 1)

Applied to almost all trainers. Prevents obviously stupid moves:

- **Redundant moves**: Calls AI_Redundant to check if a move would be useless (e.g., using Rain Dance when it's already raining, using a healing move at full HP)
- **Status on statused target**: If the target already has a non-volatile status condition, dismiss pure status moves
- **Safeguard blocking**: If the target is behind Safeguard, dismiss status moves

Effect: Adds a large penalty (via AIDiscourageMove) to make the move very unlikely.

### AI_Setup (Layer 2)

Encourages stat-modifying moves on turn 1:

- **Stat-up moves** (Attack Up, Defense Up, etc.): 50% chance to strongly encourage (-2 to score) on the FIRST turn of the enemy Pokemon. After turn 1, ~88% chance to discourage (+2 to score).
- **Stat-down moves** (Attack Down, Defense Down, etc.): 50% chance to strongly encourage on the first turn of the PLAYER's Pokemon. Same discouragement after turn 1.

### AI_Types (Layer 3)

Considers type matchups:

- **Immune moves**: Heavily penalized (AIDiscourageMove)
- **Super effective moves**: Score decreased by 1 (slightly encouraged)
- **Not very effective moves**: Score increased by 1 IF the enemy has another move of a different type that deals damage. If all damaging moves are the same type, no penalty (since there's no better option).
- **Neutral moves**: No change

### AI_Offensive (Layer 4)

Simple layer that discourages all non-damaging moves by +2 to score.

### AI_Smart (Layer 5)

The most complex layer. Has specific handlers for 60+ move effects. Key behaviors:

#### Sleep Moves
- Greatly encouraged if the enemy also has Dream Eater or Nightmare
- 50% chance to encourage otherwise

#### Healing Moves (Recover, Softboiled, Milk Drink)
- Discouraged if the enemy has high HP
- Encouraged if the enemy has low HP and is faster than the player

#### Toxic
- Encouraged if the player has high HP (more value from gradual damage)
- Discouraged if the player has low HP or already has a status

#### Explosion / Self-Destruct
- Discouraged if the enemy has high HP
- Encouraged if the enemy has low HP
- BUG: Sometimes misjudges the situation

#### Protect / Detect
- Encouraged if the enemy is Toxic'd/seeded (stall)
- Discouraged otherwise (AI recognizes it's often useless)

#### Perish Song
- Encouraged if the enemy has Mean Look/Spider Web
- 50% chance to encourage otherwise

#### Priority Moves (Quick Attack, Mach Punch, ExtremeSpeed)
- Strongly encouraged if the player has low HP (can pick off)
- Discouraged if the player has high HP

#### Counter / Mirror Coat
- Encouraged when the player has used physical/special moves
- Discouraged if the player hasn't attacked yet

#### Attract
- Encouraged if genders are opposite
- Dismissed if genders are same or either is genderless

#### Weather Moves (Rain Dance, Sunny Day)
- Encouraged if the enemy has moves that benefit from the weather
- Discouraged if already the same weather

#### Baton Pass
- Encouraged if the enemy has stat boosts
- Discouraged if the enemy has no boosts or low HP

### AI_Cautious (Layer 6)

Risk-averse behavior:
- Discourages moves with low accuracy
- Discourages recoil moves
- Discourages moves that are not very effective
- BUG: May fail to discourage residual/setup moves properly

### AI_Status (Layer 7)

Encourages status-inflicting moves:
- Encourages sleep, paralysis, and confusion moves
- Only if the player doesn't already have a status

### AI_Risky (Layer 8)

Encourages high-risk moves:
- Encourages OHKO moves (Fissure, Horn Drill, Guillotine)
- Encourages Explosion/Self-Destruct
- Encourages high-power inaccurate moves

---

## AI Move Selection (engine/battle/ai/move.asm)

After scoring is complete:

1. Find the minimum score among all moves
2. Eliminate moves that are more than 2 points above the minimum
3. From remaining moves, select randomly with equal probability

This means the AI has some randomness -- even sub-optimal moves can be chosen if they're within 2 points of the best option.

---

## AI Switching (engine/battle/ai/switch.asm)

The AI considers switching when:
- Its Pokemon is at a significant type disadvantage
- Its Pokemon has been badly statused
- A party member has a much better type matchup

Switch decision factors:
- Type effectiveness of the player's last move against current Pokemon
- HP remaining on the current Pokemon
- Whether a party member resists/is immune to the player's moves
- Whether the current Pokemon is trapped (Mean Look, etc.)

In practice, most regular trainers rarely switch. Gym Leaders and E4 members with AI_SMART are more likely to switch strategically.

---

## AI Item Usage (engine/battle/ai/items.asm)

Trainers can be configured to use items. The AI checks conditions before using each item type:

### Healing Items
- **Potion / Super Potion / Hyper Potion / Max Potion / Full Restore**: Used when HP drops below a threshold (varies by item and trainer)
- Full Restore also heals status conditions

### Status Items
- **Full Heal**: Used when the Pokemon has a non-volatile status condition
- BUG: Doesn't cure Nightmare or confusion
- BUG: Doesn't restore Attack/Speed drops from burn/paralysis

### Stat Items
- **X Attack, X Defend, X Speed, X Special, X Accuracy, Dire Hit**: Used strategically based on the AI layer
- Guard Spec: Prevents stat drops

### Usage Limits
- Each trainer has a fixed set of items they can use
- Items are consumed when used (can't be used again)
- The AI uses items INSTEAD of attacking (costs the turn)
- BUG: The AI might try to use its base reward money as an item index

---

## Gym Leader and E4 AI Configurations

### Gym Leaders (Johto)

Most Gym Leaders use AI_BASIC + AI_TYPES + AI_SMART, giving them intelligent move selection with type awareness.

- **Falkner**: Basic AI, straightforward attacking
- **Bugsy**: Uses U-turn-like strategies (not in Gen 2, but positions for type advantage)
- **Whitney**: Known for Miltank's Rollout + Milk Drink combo
- **Morty**: Uses Mean Look + Curse/Hypnosis combo
- **Chuck**: Aggressive AI, favors Fighting moves
- **Jasmine**: Defensive, uses Iron Tail and Steelix's bulk
- **Pryce**: Ice-focused, uses Blizzard in Hail-like conditions
- **Clair**: Dragon-focused, aggressive with DragonBreath and Dragonite

### Elite Four

The E4 members have among the most sophisticated AI:
- They use Full Restores at low HP
- They employ type-coverage strategies
- They switch more frequently than regular trainers
- AI_SMART layer handles most of their decision making

### Red

The ultimate trainer. Uses high-level AI with:
- Level 81 Pikachu, level 73-77 team
- Full Restore usage
- Smart type-based switching
- Aggressive offensive play

---

## AI Known Bugs Summary

1. **Mean Look on self-toxic**: Encourages Mean Look when the AI's own Pokemon is badly poisoned
2. **Conversion2 after turn 1**: Discourages Conversion2 after the first turn for no good reason
3. **Sun-synergy blindness**: Doesn't recognize SolarBeam/Flame Wheel/Moonlight benefits from Sunny Day
4. **Future Sight stacking**: Doesn't check if Future Sight is already pending, wastes turns
5. **CheckTypeMatchup assumption**: AI's type checking can return wrong results due to register corruption
6. **Item as reward bug**: Can attempt to use the base reward money value as an item
7. **Full Heal incompleteness**: Doesn't cure Nightmare, confusion, or stat drops from burn/paralysis
