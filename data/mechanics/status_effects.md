# Pokemon Crystal -- Status Effects Complete Reference

Source: pokecrystal disassembly (engine/battle/effect_commands.asm, engine/battle/core.asm)

---

## Non-Volatile Status Conditions

These persist after switching out. Only one can be active at a time.

### Sleep

- **Duration**: 1-7 turns (randomly set when inflicted)
- **Rest**: Sets sleep to exactly 2 turns (wakes on the 3rd turn and can act)
- **Counter behavior**: Decremented at the START of the Pokemon's turn (CheckTurn). When the counter reaches 0, the Pokemon wakes up AND CAN ACT that same turn.
- **Switch behavior**: Sleep counter is preserved on switch (the counter continues from where it was)
- **Bypass moves**: Snore and Sleep Talk can be used while asleep
- **Interaction with Nightmare**: Nightmare causes 1/4 max HP damage per turn while asleep. Nightmare is cleared when the Pokemon wakes up.
- **Sleep clause**: Not enforced by the game engine (only a player/tournament rule)

### Freeze

- **Duration**: Indefinite (no automatic thaw during turn checks!)
- **Thaw check**: 20% chance at the END of each turn cycle (HandleDefrost, called in HandleBetweenTurnEffects). This is NOT checked during CheckTurn.
- **Self-thaw moves**: Flame Wheel and Sacred Fire thaw the user before attacking (checked in CheckTurn). The user can use these moves even while frozen.
- **Hit by Fire move**: Being hit by ANY Fire-type move thaws the target
- **Cannot act**: While frozen, the Pokemon cannot move at all (turn is skipped)
- **Immunity**: Fire-type Pokemon cannot be frozen (checked when attempting to inflict)
- **Key difference from Gen 1**: In Gen 1, freeze was permanent (no random thaw). Gen 2 added the 20% thaw.

### Paralysis

- **Full paralysis**: 25% chance each turn of being unable to act
- **Speed reduction**: Speed stat is multiplied by 25% (quartered). This is a persistent battle stat modification.
- **Does not wear off**: Must be healed by item, move, or switching to a cleric
- **Immunity**: Electric-type Pokemon cannot be paralyzed
- **Interaction with paralysis check order**: Paralysis check happens LAST in the pre-move checks (after recharge, sleep, freeze, flinch, disable, confusion, attract). This means if a Pokemon would be immobilized by confusion AND paralysis, only the first check (confusion) applies.

### Burn

- **End-of-turn damage**: 1/8 max HP per turn (rounded down, minimum 1)
- **Attack reduction**: Attack stat halved while burned (affects physical damage)
- **Timing**: Burn damage is applied during ResidualDamage, which runs after each individual turn
- **Immunity**: Fire-type Pokemon cannot be burned
- **On critical hits**: Burn's Attack halving is ignored only if the target's Defense stage > attacker's Attack stage (see critical hit rules in damage_formula.md)
- **Confusion self-hit**: Burn DOES halve confusion self-hit damage (since it's treated as physical)
- **BUG**: When switching in a burned Pokemon, the Attack reduction may not properly apply until the next stat recalculation

### Poison

- **End-of-turn damage**: 1/8 max HP per turn (rounded down, minimum 1)
- **Overworld**: Poison also damages in the overworld (1 HP per 4 steps). In Crystal, Pokemon faint at 1 HP from overworld poison; in Gold/Silver they can faint.
- **Immunity**: Poison-type and Steel-type Pokemon cannot be poisoned
- **Timing**: Same as burn (ResidualDamage)

### Badly Poisoned (Toxic)

- **Escalating damage**: N/16 of max HP, where N starts at 1 and increments each turn
  - Turn 1: 1/16, Turn 2: 2/16, Turn 3: 3/16, etc.
  - Maximum: 15/16 per turn (counter caps at 15)
- **Switch behavior**: In Gen 2, switching out **converts** badly poisoned to regular poison. The toxic counter is completely lost.
- **Implementation**: The game stores a toxic counter (wPlayerToxicCount / wEnemyToxicCount). Each turn, it increments by 1. Damage = (1/16 max HP) * counter.
- **Internal storage**: Badly poisoned is stored as SUBSTATUS_TOXIC in substatus5, plus regular poison in the status byte. When the toxic substatus is cleared (on switch), regular poison remains.

---

## Volatile Status Conditions

These are cleared when the Pokemon switches out.

### Confusion

- **Duration**: 2-5 turns (randomly set when inflicted)
- **Self-hit chance**: 50% each turn of hitting self instead of executing the chosen move
- **Self-hit damage**: Calculated as a typeless physical move with **40 base power**, using the confused Pokemon's own Attack and Defense
  - No STAB (typeless)
  - No type effectiveness (typeless)
  - Burn DOES halve the self-hit damage (it's physical)
  - Critical hits are disabled for confusion self-hits (wCriticalHit set to 0)
  - BUG: Type-boosting items and Explosion defense-halving DO affect confusion damage
- **Counter behavior**: Decremented at the start of the turn (before the 50% check). If it reaches 0, confusion ends and the Pokemon can act normally.
- **Cleared by**: Switching out, or duration expiring
- **BUG**: Focus Band cannot prevent KO from confusion self-hit in Gen 2

### Attraction (Infatuation)

- **Condition**: Only works between opposite genders
- **Effect**: 50% chance each turn of being immobilized ("immobilized by love")
- **Cleared by**: Switching out either the infatuated Pokemon or the Pokemon that caused it
- **Cleared by**: Heal Bell removes it
- **Not passed by**: Baton Pass
- **Special**: Both sides can be infatuated simultaneously

### Flinch

- **Effect**: Pokemon cannot move this turn (100% skip)
- **Application**: Only works if the flinching Pokemon moves SECOND (the flinch-causing move must go first)
- **Cleared**: Automatically at the start of each turn (reset in CheckPlayerLockedIn/CheckEnemyLockedIn before turn starts)
- **Source**: King's Rock (10% chance on any damaging move), certain moves with built-in flinch chance

### Trapped (Mean Look / Spider Web / Wrap)

**Mean Look / Spider Web:**
- Prevents switching (but not Baton Pass)
- Cleared when the trapping Pokemon faints or switches out
- Baton Pass does NOT pass the trap (the incoming Pokemon is free to switch)

**Wrap / Bind / Fire Spin / Whirlpool / Clamp:**
- Prevents switching for 2-5 turns
- Deals 1/16 max HP per turn (at end of turn in HandleWrap)
- If the trapping Pokemon switches or faints, the wrap ends
- Creating a Substitute clears wrap

### Leech Seed

- **Effect**: Drains 1/8 max HP per turn, heals the opponent by the same amount
- **Cleared by**: Switching out
- **Immunity**: Grass-type Pokemon cannot be Leech Seeded
- **Interaction with Substitute**: Cannot Leech Seed a Pokemon behind a Substitute. Draining does NOT work through a Substitute (the seeded Pokemon still loses HP, but healing is blocked if the opponent has a Sub).
- **Baton Pass**: Leech Seed IS passed by Baton Pass

### Curse (Ghost-type)

- **Effect**: Cursed Pokemon loses 1/4 max HP per turn
- **Application**: Ghost-type user sacrifices 1/2 its max HP to curse the target
- **Cleared by**: Switching out
- **Non-Ghost Curse**: For non-Ghost users, Curse raises Attack and Defense by 1 stage and lowers Speed by 1 stage (no residual damage, no HP sacrifice)

### Nightmare

- **Effect**: 1/4 max HP damage per turn, but ONLY while asleep
- **Cleared by**: Waking up (automatically), switching out
- **Application**: Can only be used on sleeping Pokemon
- **Baton Pass**: Nightmare is NOT passed (explicitly cleared in ResetBatonPassStatus)

### Focus Energy

- **Effect**: +1 critical hit stage
- **Duration**: Until switch or battle end
- **Baton Pass**: IS passed by Baton Pass

### Substitute

- **HP cost**: 1/4 of max HP (rounded down)
- **Fails if**: HP is not above 1/4 max (must have enough HP remaining after the cost)
- **Substitute HP**: Equal to 1/4 max HP (the same amount sacrificed)
- **Blocks**: Most damaging moves, status moves (except those that bypass)
- **Does NOT block**: Sound-based moves are not special in Gen 2 (Perish Song does bypass)
- **Draining moves**: Absorb/Mega Drain/Giga Drain/Dream Eater/Leech Life fail against Substitute (checked in CheckHit .DrainSub)
- **Baton Pass**: Substitute IS passed (with remaining HP)

### Perish Song

- **Counter**: Starts at 3 (displayed as "perish count fell to 3"), decrements each turn
- **Implementation**: Actually stored as 4, decremented in HandlePerishSong. Faints when reaching 0.
- **Both sides**: Affects both the user and the target (unless already affected)
- **Switching**: Switching out removes the Perish Song counter
- **Baton Pass**: Perish Song counter IS passed by Baton Pass
- **BUG**: Perish Song + Spikes can leave a Pokemon at 0 HP without fainting (see glitches)

### Encore

- **Duration**: 2-6 turns (locks the target into their last-used move)
- **Cleared by**: Switching out, or duration expiring
- **Fails if**: Target hasn't used a move yet, or target used Struggle, Encore, Mirror Move, or Transform
- **Not passed by**: Baton Pass

### Disable

- **Duration**: 1-7 turns (random, stored as upper nibble of DisableCount)
- **Effect**: Prevents use of one specific move (the last move used by the target)
- **Cleared by**: Switching out, or duration expiring
- **Not passed by**: Baton Pass

### Lock-On / Mind Reader

- **Effect**: Next move guaranteed to hit (bypasses accuracy check)
- **Duration**: 1 turn only (cleared after the guaranteed hit)
- **Exception**: Does not guarantee hits against Pokemon using Fly or Dig if the move is Earthquake, Fissure, or Magnitude (these check Fly status even with Lock-On)
- **BUG**: Lock-On doesn't always bypass Fly/Dig for status moves (Attract, Curse, etc.)

### Destiny Bond

- **Effect**: If the user faints from damage on the turn Destiny Bond was used, the attacker also faints
- **Duration**: Until the user moves again (cleared at start of each turn in EndUserDestinyBond)
- **Timing**: Must be active when the user faints from a direct attack

### Rage

- **Effect**: While Raging, the user's Attack stat stage increases by 1 each time it is hit
- **Duration**: Until the user selects a different move or faints

### Safeguard

- **Duration**: 5 turns
- **Effect**: Prevents non-volatile status conditions from being inflicted on the protected side
- **Does NOT prevent**: Confusion, attraction, stat drops, or volatile conditions
- **Does NOT heal**: Existing status conditions remain
