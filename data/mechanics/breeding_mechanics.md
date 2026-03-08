# Pokemon Crystal -- Breeding Mechanics

Source: pokecrystal disassembly, Bulbapedia breeding mechanics, community research.

---

## Breeding Compatibility

### Egg Groups

Pokemon are assigned to one or two Egg Groups. Two Pokemon can breed if they share at least one Egg Group AND are of opposite genders (or one is Ditto).

**Gen 2 Egg Groups** (15 total):
1. Monster
2. Water 1
3. Water 2
4. Water 3
5. Bug
6. Flying
7. Field
8. Fairy
9. Grass
10. Human-Like
11. Mineral
12. Amorphous (Indeterminate)
13. Ditto
14. Dragon
15. No Eggs Discovered (Undiscoverable) -- cannot breed

### Special Cases
- **Ditto**: Can breed with any Pokemon that can breed (any Egg Group except Undiscoverable), regardless of gender. Always produces eggs of the NON-Ditto partner's species.
- **Genderless Pokemon**: Can ONLY breed with Ditto (e.g., Magnemite, Voltorb, Staryu)
- **Baby Pokemon**: Cannot breed (they're in the Undiscoverable group until evolved)
- **Legendary Pokemon**: Most are in Undiscoverable (cannot breed). Exception: none in Gen 2 can breed.

### Compatibility Check at Day Care

The Day Care Man checks compatibility and gives different messages:

| Compatibility Level | Message | Egg chance per 256 steps |
|----|---------|--------------------------|
| Different species, different OT | "The two seem to get along" | 50.2% |
| Same species, different OT | "The two seem to get along very well" | 70.3% |
| Different species, same OT | "The two don't seem to like each other" | 19.9% |
| Same species, same OT | "The two seem to get along" | 50.2% |
| Incompatible | "They prefer to play with other Pokemon" | 0% |

---

## Egg Production

### Step Counter
- Every 256 steps, the game checks if the Day Care pair can produce an egg
- If compatible, a random check determines if an egg is produced (based on compatibility level above)
- Only ONE egg can exist at a time (must pick up the current egg before another can be generated)

### Species Inheritance
- The egg always hatches into the **lowest evolution** of the mother's species
- Exception: If breeding with Ditto, the egg is the lowest evolution of the non-Ditto parent
- If the baby is a baby Pokemon (Pichu, Cleffa, etc.), the baby form is produced
- Exception: Certain incense-requiring babies (not in Gen 2) -- in Gen 2, breeding Snorlax always gives Snorlax, not Munchlax (Munchlax doesn't exist)

---

## DV Inheritance

### How DVs are Determined for Eggs

Gen 2 has a specific DV inheritance system:

#### Defense DV
- **Inherited**: The Defense DV is directly copied from the opposite-gender parent (or from Ditto if breeding with Ditto)

#### Special DV
- **Semi-inherited**: The Special DV starts as the opposite-gender parent's Special DV, then:
  - 50% chance: kept as-is
  - 50% chance: modified by +8 or -8 (wraps: if >= 8, subtract 8; if < 8, add 8)
  - This ensures variety while keeping some inheritance

#### Attack DV
- **Random**: Generated completely randomly (0-15)

#### Speed DV
- **Random**: Generated completely randomly (0-15)

#### HP DV
- **Derived**: Calculated from the other 4 DVs as always:
  ```
  HP_DV = (Attack & 1) * 8 + (Defense & 1) * 4 + (Speed & 1) * 2 + (Special & 1)
  ```

### Gender Implications
Since gender is determined by Attack DV, and Attack DV is random for eggs:
- The gender of offspring is essentially random based on the species' gender ratio
- Species with skewed ratios (like starters at 87.5% male) will still produce mostly male offspring

---

## Egg Moves

### Inheritance Rules
The father can pass moves to the offspring that:
1. The offspring species can learn the move as an Egg Move (defined in egg_moves.asm)
2. The father currently knows the move

### TM/HM Move Inheritance
If the father knows a TM or HM move that the baby species can also learn via TM/HM, the baby inherits that move.

### Move Priority (when egg has more moves to learn than 4 slots)
1. Default level-up moves for level 5 (or level 1) come first
2. TM/HM moves from father
3. Egg Moves from father
4. Earlier moves in the list have priority (later ones may be lost)

### Chain Breeding
Some Pokemon can only learn certain moves through multi-generation breeding chains. Example:
- Skarmory can learn Drill Peck as an Egg Move
- A male Dodrio (which learns Drill Peck by level-up) breeds with a female Skarmory
- The Skarmory offspring knows Drill Peck

More complex chains involve intermediate species to pass moves between otherwise incompatible groups.

---

## Egg Hatching

### Step Counter
Each egg species has a base number of egg cycles needed to hatch. Each cycle = 256 steps.

Common hatch cycle counts:
| Cycles | Steps | Pokemon |
|--------|-------|---------|
| 5 | 1280 | Magikarp, Gyarados |
| 10 | 2560 | Pidgey, Rattata, most common Pokemon |
| 15 | 3840 | Many mid-tier Pokemon |
| 20 | 5120 | Starters, pseudo-legendaries |
| 25 | 6400 | Chansey, Eevee |
| 30 | 7680 | Lapras, Snorlax |
| 35 | 8960 | Rare Pokemon |
| 40 | 10240 | Very rare (Aerodactyl, etc.) |

### Hatching Level
Eggs hatch at **level 5** (not level 1, due to the Medium Slow experience underflow bug).

### Stat Experience
Newly hatched Pokemon have 0 Stat Experience in all stats.

---

## Shiny Breeding

### The Shiny DV Pattern
A Pokemon is shiny when:
- Defense DV = 10
- Speed DV = 10
- Special DV = 10
- Attack DV = 2, 3, 6, 7, 10, 11, 14, or 15

### Shiny Ditto Exploit

Since Defense DV and Special DV are inherited (or semi-inherited) from the opposite-gender parent:

1. Start with a **Shiny Ditto** (Defense = 10, Speed = 10, Special = 10, Attack = 2/3/6/7/10/11/14/15)
2. Breed it with any compatible Pokemon
3. The offspring:
   - Defense DV = 10 (inherited from Ditto) -- MATCHES shiny requirement
   - Special DV = 10 (50% chance to keep) or 2 (50% chance, +/- 8) -- 10 MATCHES, 2 does NOT
   - Attack DV = random (4 out of 16 values match: 2,3,6,7,10,11,14,15 = 8/16 = 50%) -- wait, that's 8 values, 50%
   - Speed DV = random (must be 10, only 1/16 chance)

Actually, let me recalculate more carefully:
- Defense: 10 (guaranteed from Ditto) -- check
- Special: 50% chance of 10 (inherited as-is) -- need this to be 10
- Attack: random, 8/16 = 50% chance of a shiny-compatible value
- Speed: random, 1/16 chance of being exactly 10

Combined probability: 1 * 0.5 * 0.5 * (1/16) = 1/64

So breeding with a Shiny Ditto gives approximately **1/64 chance** of a shiny offspring, compared to the normal 1/8192.

### Breeding Two Shinies

If both parents are shiny (and compatible), the odds are even higher, since both parents contribute favorable DVs. In practice, obtaining two compatible shiny parents is very difficult.

### Gender Limitation

Since Attack DV determines gender AND shininess:
- For species requiring low Attack DV for female (like starters with gender threshold 2): Attack DV must be 0 or 1 for female, but shiny requires Attack DV 2+
- Therefore: **Female shinies are impossible** for species with high male ratios (87.5% male) since the minimum shiny Attack DV (2) exceeds the female threshold (< 2)

---

## Day Care Level and Experience

### How Level-Up Works in Day Care
- Pokemon gain 1 experience point per step while in the Day Care
- This is flat experience, not affected by growth rate formulas
- Pokemon in Day Care CAN level up (and learn level-up moves)
- If a Pokemon in Day Care would learn a move but has 4 moves already, the oldest move is deleted and the new one is added (no player choice)

### BUG: Day Care Experience Loss
If a Pokemon is deposited in the Day Care and retrieved without gaining enough experience to maintain its level, it may lose experience due to how the game recalculates experience on retrieval. This primarily affects Pokemon at low levels with the Medium Slow growth rate.

---

## Egg Group Compatibility Table

Key breeding pairs that people commonly look up:

| Father | Mother | Egg Moves of Interest |
|--------|--------|----------------------|
| Dodrio | Skarmory | Drill Peck |
| Smeargle | Any | Any move via Sketch |
| Mr. Mime | Abra/Kadabra | Barrier |
| Beedrill | Scyther | Baton Pass (not available) |
| Golduck | Psyduck | Cross Chop |
| Houndoom | Vulpix | Fire Spin |

Note: Smeargle is the universal egg move father -- since Sketch can permanently learn any move, Smeargle can pass nearly any move as an Egg Move to compatible species in the Field egg group.
