# Pokemon Crystal — RNG Mechanics

Source: pokecrystal disassembly (home/random.asm, home/vblank.asm, engine/battle/core.asm)

---

## Two Separate RNG Systems

Pokemon Crystal has TWO distinct random number generators with fundamentally different designs:

### 1. Hardware RNG (`Random`)

Used for all non-battle randomness: wild encounters, DVs, overworld events, item drops, etc.

**Implementation (home/random.asm):**
```
Random::
    ldh a, [rDIV]      ; read hardware divider register
    ld b, a
    ldh a, [hRandomAdd]
    adc b               ; hRandomAdd += DIV + carry
    ldh [hRandomAdd], a

    ldh a, [rDIV]       ; read DIV again (different value now)
    ld b, a
    ldh a, [hRandomSub]
    sbc b               ; hRandomSub -= DIV - carry
    ldh [hRandomSub], a
    ret                  ; returns hRandomSub in a
```

**Key properties:**
- Based on the Game Boy's hardware DIV register (increments at 16,384 Hz)
- Produces two values: `hRandomAdd` and `hRandomSub`
- NOT deterministic from a seed — depends on exact CPU timing and DIV state
- Also advanced during VBlank interrupt (same add/sub logic runs every frame in VBlank_Normal)
- The VBlank advancement means the RNG state changes every frame regardless of whether `Random` is called
- Returns `hRandomSub` in register A

**Why this matters for recreation:** You cannot replicate this RNG with a simple PRNG. The DIV register depends on exact cycle-level CPU timing. Frame-by-frame emulation is required. For a game engine recreation, use a deterministic PRNG instead and accept that wild encounter sequences won't match the original hardware.

### 2. Battle PRNG (`BattleRandom` / `_BattleRandom`)

Used for ALL randomness during battle: damage rolls, accuracy checks, critical hit checks, move effect chances, AI decisions, confusion self-hit, paralysis check, etc.

**Implementation (engine/battle/core.asm lines 6880-6946):**

In non-link battles: `_BattleRandom` simply calls `Random` (hardware RNG).

In link battles: Uses a synchronized PRNG to keep both Game Boys in sync:

```
; PRNG formula: a[n+1] = (a[n] * 5 + 1) % 256
; Operates in streams of 10 values (SERIAL_RNS_LENGTH)
```

**Link battle PRNG algorithm:**
1. 10 seed values are stored in `wLinkBattleRNs`
2. Values are consumed sequentially via `wLinkBattleRNCount`
3. When all 10 are consumed (actually 9 — the last value triggers regeneration), each seed is advanced:
   - `new_value = (old_value * 5 + 1) mod 256`
4. This is a simple Linear Congruential Generator (LCG) with a=5, c=1, m=256
5. The counter resets to 0 and consumption starts again

**Properties of the link PRNG:**
- Period: 256 per seed (full period LCG with odd multiplier and odd increment mod power of 2)
- Deterministic from initial seeds
- Seeds are exchanged between players at battle start
- The "last value triggers regeneration" quirk means only 9 of 10 values are used before the set is regenerated — the 10th value from the previous generation is returned alongside triggering new generation

Source: `home/random.asm`, `engine/battle/core.asm` lines 6880-6946

---

## RandomRange Function

```
RandomRange::
; Return a random number between 0 and a (non-inclusive).
```

Uses rejection sampling to avoid modulo bias:
1. Calculate `b = 256 % c` (where c = input range)
2. Call `Random` repeatedly until result < (256 - b)
3. Divide result by c to get remainder in range [0, c-1]

This is correct and unbiased, unlike naive `Random() % n`.

Source: `home/random.asm` lines 50-80

---

## What Consumes RNG Calls

### Battle RNG Consumers (BattleRandom)

| Consumer | When | Notes |
|----------|------|-------|
| **Accuracy check** | Every attack | Random byte compared against accuracy threshold |
| **Critical hit check** | Every damaging attack | Random byte compared against crit chance |
| **Damage variation** | Every damaging attack | Random byte in range 217-255 for 85-100% damage |
| **Move effect chance** | Secondary effects | e.g., 30% chance to burn for Flamethrower |
| **Confusion self-hit** | Each turn while confused | 50% chance: `cp 50 percent + 1` |
| **Infatuation immobility** | Each turn while attracted | 50% chance to skip turn |
| **Paralysis immobility** | Each turn if paralyzed | 25% chance to be fully paralyzed |
| **Sleep counter** | When put to sleep | 1-7 turns randomly assigned |
| **Disable duration** | When Disable is used | Random duration |
| **Encore duration** | When Encore is used | Random duration |
| **Confusion duration** | When confused | 2-5 turns |
| **Multi-hit count** | Multi-hit moves | 2-5 hits with weighted probabilities |
| **Metronome move selection** | Metronome used | Random move from valid pool |
| **Present power** | Present used | 40/30/10/20% for power tiers |
| **Magnitude power** | Magnitude used | Weighted random magnitude 4-10 |
| **AI move selection** | Enemy turn | AI scoring adjustments use RNG |
| **AI switching** | Enemy turn | Random factors in switch decisions |
| **Quick Claw activation** | Move order determination | Random check against item parameter |
| **Speed tie breaking** | Equal speed, no priority | 50% coin flip |
| **Move order for both switching** | Both sides switch in link | 50% coin flip |
| **Protect/Detect** | Consecutive uses | Success threshold checked against random |
| **Sleep Talk move selection** | Sleep Talk used | Random from valid moves |
| **Conversion2 type selection** | Conversion2 used | Random from resistant types |

### Overworld RNG Consumers (Random)

| Consumer | When | Notes |
|----------|------|-------|
| **Wild encounter check** | Every step in grass/cave/water | Encounter rate compared against random |
| **Wild species selection** | Encounter triggered | Weighted random from encounter table |
| **Wild level selection** | Encounter triggered | Random from level range |
| **DV generation** | Wild/bred Pokemon | 2 random bytes for 4 DVs |
| **Egg step counter** | Day Care step | Random value for wStepsToEgg |
| **Egg species determination** | Odd Egg | Weighted random from baby pool |
| **Breeding compatibility check** | Day Care step | Random vs. compatibility threshold |
| **Shiny determination** | Generation | Via DV check (derived from random DVs) |
| **Haircut result** | Haircut used | Random against probability table |
| **Phone call timing** | Overworld steps | Random delay between calls |
| **Fruit tree reset** | Daily | Via daily flag system |
| **Happiness walking** | Every 256 steps | Step counter, not truly random |
| **Roaming Pokemon movement** | Map transition | Random route selection |
| **Swarm determination** | Daily events | Random swarm activation |

---

## RNG Manipulation

### Speedrun RNG Manipulation

In non-link battles, `BattleRandom` calls `Random` which reads the hardware DIV register. This means:

1. **Frame-perfect inputs** change DIV state, shifting all subsequent random values
2. **Menuing** (opening/closing menus) advances the VBlank RNG counter
3. **Text speed** affects when RNG calls occur relative to VBlank
4. **Walking patterns** before encounters affect the RNG state

Speedrunners exploit this by:
- Using specific menu sequences before critical battles to manipulate damage rolls
- Frame-perfect encounter manipulation for desired wild Pokemon/DVs
- Specific movement patterns to control encounter timing

### Link Battle Determinism

In link battles, both players share the same 10 PRNG seeds exchanged at connection. This means:
- Both consoles produce identical random sequences
- The sequence is fully deterministic from the initial seeds
- If you know the seeds, you can predict every random outcome
- Seeds are exchanged in the clear during the link protocol

### DV Manipulation

DVs are generated by calling `Random` twice:
- First call: Attack DV (high nibble) and Defense DV (low nibble)
- Second call: Speed DV (high nibble) and Special DV (low nibble)
- HP DV is derived from the low bits of the other four

Since wild encounter DVs use the hardware RNG, frame-precise manipulation of the encounter timing can select specific DV combinations. The shiny check (specific DV pattern) means shiny Pokemon can be manipulated with precise timing.

---

## VBlank RNG Advancement

Every frame during VBlank_Normal (home/vblank.asm lines 68-79):
```
; advance random variables
ldh a, [rDIV]
ld b, a
ldh a, [hRandomAdd]
adc b
ldh [hRandomAdd], a

ldh a, [rDIV]
ld b, a
ldh a, [hRandomSub]
sbc b
ldh [hRandomSub], a
```

This runs ~59.7 times per second (Game Boy frame rate). It means the RNG state is constantly changing even when the game is idle, making the hardware RNG truly unpredictable without cycle-accurate emulation.

---

## Implementation Notes for Recreation

1. **Do NOT use the hardware RNG for a recreation.** Use a seeded PRNG (xorshift, Mersenne Twister, or even the link battle LCG).

2. **The link battle PRNG is the only deterministic option.** Formula: `next = (current * 5 + 1) & 0xFF`. Period 256.

3. **BattleRandom vs Random distinction matters.** In the original, non-link battles use hardware RNG for battle randomness. If you want deterministic replays, always use the seeded PRNG path.

4. **RNG consumption order is critical for determinism.** If you consume RNG in a different order than the original (e.g., checking crit before accuracy instead of after), your battle outcomes will diverge even with identical seeds.

5. **The 10-seed stream system** in link battles means you get 9 values per regeneration cycle. The implementation has a subtle off-by-one: `cp SERIAL_RNS_LENGTH - 1` means the 9th value (index 8) triggers regeneration, and the caller receives the 10th value (index 9) from the OLD set while regeneration produces a NEW set.
