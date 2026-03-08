# Pokemon Crystal - Move Details

Supplementary move data not in all_moves.md. Priority values, grammar flags, special move tables, power curves, and in-game descriptions.

Source: `data/moves/*.asm`

---

## Move Priority

From `data/moves/effects_priorities.asm`. Most moves have priority 1 (default). Listed are the exceptions:

| Effect | Priority | Moves |
|--------|----------|-------|
| Protect / Detect | 3 | Protect, Detect |
| Endure | 3 | Endure |
| Priority Hit (Quick Attack, etc.) | 2 | Quick Attack, Mach Punch, ExtremeSpeed |
| Force Switch (Roar/Whirlwind) | 0 | Roar, Whirlwind |
| Counter | 0 | Counter |
| Mirror Coat | 0 | Mirror Coat |

Priority 0 means the move always goes last (after all priority 1 moves, regardless of Speed).
Priority 2 means the move goes before normal moves.
Priority 3 means the move goes before priority 2 moves.

---

## Critical Hit Moves (High Crit Ratio)

From `data/moves/critical_hit_moves.asm`. These moves have +1 to their critical hit stage:

- Karate Chop
- Razor Wind
- Razor Leaf
- Crabhammer
- Slash
- Aeroblast
- Cross Chop

---

## Flail / Reversal Power Table

From `data/moves/flail_reversal_power.asm`. Power scales inversely with remaining HP:

| HP Remaining | Power |
|-------------|-------|
| < 2% (1 pixel) | 200 |
| < 8% (4 pixels) | 150 |
| < 20% (10 pixels) | 100 |
| < 33% (16 pixels) | 80 |
| < 67% (32 pixels) | 40 |
| >= 67% (full bar) | 20 |

HP bar is 48 pixels wide. Thresholds are in pixels.

---

## Magnitude Power Table

From `data/moves/magnitude_power.asm`:

| Magnitude | Power | Chance |
|-----------|-------|--------|
| 4 | 10 | 5% |
| 5 | 30 | 10% |
| 6 | 50 | 20% |
| 7 | 70 | 30% |
| 8 | 90 | 20% |
| 9 | 110 | 10% |
| 10 | 150 | 5% |

Average power: 71

---

## Present Power Table

From `data/moves/present_power.asm`:

| Outcome | Chance | Effect |
|---------|--------|--------|
| 40 power | 40% | Damage |
| 80 power | 30% | Damage |
| 120 power | 10% | Damage |
| Heal 1/4 HP | 20% | Heals opponent! |

---

## Metronome Exceptions

From `data/moves/metronome_exception_moves.asm`. Metronome cannot randomly select these moves:

- (No Move)
- Metronome (itself)
- Struggle
- Sketch
- Mimic
- Counter
- Mirror Coat
- Protect
- Detect
- Endure
- Destiny Bond
- Sleep Talk
- Thief

---

## Move Grammar (Japanese Localization Artifacts)

From `data/moves/grammar.asm`. In English, all moves use "[Pokemon] used [Move]!" but the original Japanese had 5 grammar patterns. The data still exists:

### Group 0 — "used [move]"
Swords Dance, Growth, Strength, Harden, Minimize, Smokescreen, Withdraw, Defense Curl, Egg Bomb, Smog, Bone Club, Flash, Splash, Acid Armor, Bonemerang, Rest, Sharpen, Substitute, Mind Reader, Snore, Protect, Spikes, Endure, Rollout, Swagger, Sleep Talk, Hidden Power, Psych Up, ExtremeSpeed

### Group 1 — "did [move]"
Recover, Teleport, Bide, Self-Destruct, Amnesia, Flail

### Group 2 — "did [move]"
Meditate, Agility, Mimic, Double Team, Barrage, Transform, Struggle, Scary Face

### Group 3 — "[move] attack"
Pound, Scratch, Vicegrip, Wing Attack, Fly, Bind, Slam, Horn Attack, Wrap, Thrash, Tail Whip, Leer, Bite, Growl, Roar, Sing, Peck, Absorb, String Shot, Earthquake, Fissure, Dig, Toxic, Screech, Metronome, Lick, Clamp, Constrict, Poison Gas, Bubble, Slash, Spider Web, Nightmare, Curse, Foresight, Charm, Attract, Rock Smash

### Group 4 — "[Pokemon]'s [move]!" (default for unlisted moves)
All other moves.

---

## TM/HM/Tutor Move Mapping

The move constants for TMs/HMs are defined in item_constants.asm and loaded via `data/moves/tmhm_moves.asm`. The table stores move IDs in order: TM01-TM50, HM01-HM07, then Move Tutors.

Move Tutors (Crystal only):
1. Flamethrower
2. Thunderbolt
3. Ice Beam

---

## In-Game Move Descriptions (All 251 Moves)

From `data/moves/descriptions.asm`. Each move has a 2-line description viewable in the move summary screen.

### Physical Attacks
| Move | Description |
|------|------------|
| Pound | "Pounds with forelegs or tail." |
| Karate Chop | "Has a high critical hit ratio." |
| Double Slap | "Repeatedly slaps 2-5 times." |
| Comet Punch | "Repeatedly punches 2-5 times." |
| Mega Punch | "A powerful punch thrown very hard." |
| Pay Day | "Throws coins. Gets them back later." |
| Scratch | "Scratches with sharp claws." |
| Vicegrip | "Grips with powerful pincers." |
| Slam | "Slams the foe with a tail, vine, etc." |
| Stomp | "An attack that may cause flinching." |
| Double Kick | "A double kicking attack." |
| Mega Kick | "A powerful kicking attack." |
| Jump Kick | "May miss, damaging the user." |
| Rolling Kick | "A fast, spinning kick." |
| Headbutt | "An attack that may make foe flinch." |
| Horn Attack | "An attack using a horn to jab." |
| Fury Attack | "Jabs the target 2-5 times." |
| Tackle | "A full-body charge attack." |
| Body Slam | "An attack that may cause paralysis." |
| Take Down | "A tackle that also hurts the user." |
| Thrash | "Works 2-3 turns and confuses user." |
| Double-Edge | "A tackle that also hurts the user." |
| Bite | "An attack that may cause flinching." |
| Peck | "Jabs the foe with a beak, etc." |
| Drill Peck | "A strong, spinning-peck attack." |
| Submission | "An attack that also hurts the user." |
| Low Kick | "An attack that may cause flinching." |
| Strength | "A powerful physical attack." |
| Slash | "Has a high critical hit ratio." |
| Bone Club | "An attack that may cause flinching." |
| Hyper Fang | "An attack that may cause flinching." |
| Quick Attack | "Lets the user get in the first hit." |

### Special Attacks
| Move | Description |
|------|------------|
| Fire Punch | "A fiery punch. May cause a burn." |
| Ice Punch | "An icy punch. May cause freezing." |
| ThunderPunch | "An electric punch. It may paralyze." |
| Ember | "An attack that may inflict a burn." |
| Flamethrower | "An attack that may inflict a burn." |
| Water Gun | "Squirts water to attack." |
| Hydro Pump | "A powerful water-type attack." |
| Surf | "A strong water-type attack." |
| Ice Beam | "An attack that may freeze the foe." |
| Blizzard | "An attack that may freeze the foe." |
| Psybeam | "An attack that may confuse the foe." |
| Hyper Beam | "1st turn: Attack. 2nd turn: Rest." |
| Thunderbolt | "A strong electrical attack." |
| Thunder | "An attack that may cause paralysis." |
| Earthquake | "Tough but can't hit flying foes." |
| Psychic | "An attack that may lower SP.DEF." |
| Dragon Rage | "Always inflicts 40HP damage." |
| SonicBoom | "Always inflicts 20HP damage." |

### OHKO Moves
| Move | Description |
|------|------------|
| Guillotine | "A one-hit KO, pincer attack." |
| Horn Drill | "A one-hit KO, drill attack." |
| Fissure | "A one-hit KO, ground attack." |

### Status Moves
| Move | Description |
|------|------------|
| Swords Dance | "A dance that increases ATTACK." |
| Tail Whip | "Lowers the foe's DEFENSE." |
| Leer | "Reduces the foe's DEFENSE." |
| Growl | "Reduces the foe's ATTACK." |
| Sing | "May cause the foe to fall asleep." |
| Supersonic | "Sound waves that cause confusion." |
| Disable | "Disables the foe's most recent move." |
| Counter | "Returns a physical blow double." |
| Seismic Toss | "The user's level equals damage HP." |
| Toxic | "Poisons the foe with an intensifying toxin." |
| Recover | "Recovers up to half the user's max HP." |
| Rest | "Sleep for 2 turns to fully recover." |
| Substitute | "Creates a decoy using 1/4 of the user's max HP." |
| Destiny Bond | "The foe faints if the user does." |
| Perish Song | "Both faint in 3 turns unless switched." |
| Protect | "Foils attack that turn. May fail." |
| Detect | "Evades attack that turn. May fail." |
| Endure | "Always leaves at least 1HP." |
| Baton Pass | "Switches while keeping stat changes." |
| Mean Look | "Prevents the foe from escaping." |
| Encore | "Makes the foe repeat 2-6 times." |
