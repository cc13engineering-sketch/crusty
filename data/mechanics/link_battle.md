# Pokemon Crystal - Link Battle & Trading Mechanics

Multiplayer mechanics for Pokemon Crystal, including link battles, trading, Time Capsule, and Stadium 2.

---

## Link Cable Battles

### Standard Link Battle
- **Players:** 2 (via Game Boy link cable or wireless adapter)
- **Format:** 6v6 singles (use your full team)
- **Level:** All Pokemon fight at their actual levels (no scaling)
- **Rules:** No items from bag during battle. Held items work normally.
- **Mechanics:** Standard Gen 2 battle mechanics apply

### Colosseum (Pokemon Stadium 2)
- **Format:** Various rulesets selectable in Stadium 2
- **Level cap options:** Lv50 flat, Lv100 flat, or no restriction
- **Special rules:** Can ban specific Pokemon or items via rental/selection screens

---

## Flat Battle Rules

When playing competitive link battles, common community rules include:

### Standard Rules (Unofficial Community Standard)
- **Level Cap:** Level 50 or Level 100
- **Team Size:** 6v6 or 3v3
- **Species Clause:** No duplicate Pokemon species
- **Item Clause:** No duplicate held items (mirrors Battle Tower)
- **Sleep Clause:** Cannot put more than one opposing Pokemon to sleep at a time
- **Freeze Clause:** Sometimes enforced — no more than one frozen opponent
- **Evasion Clause:** Double Team and Minimize are banned
- **OHKO Clause:** Fissure, Horn Drill, Guillotine are banned
- **Self-KO Clause:** If both players are on their last Pokemon, Explosion/Self-Destruct/Destiny Bond results in a loss for the user

### Banned Pokemon (Ubers)
The following are typically banned from standard play (Smogon GSC OU rules):
- Mewtwo
- Mew
- Lugia
- Ho-Oh
- Celebi

---

## Trading

### Standard Trading
- Trade any Pokemon between Gold, Silver, and Crystal
- Traded Pokemon gain 1.5x EXP
- Pokemon traded retain their OT (original trainer) name and ID
- Pokemon with a different OT may disobey if over-leveled (badge-dependent)

### Obedience by Badge Count
| Badges | Max Obedient Level |
|--------|-------------------|
| 0 | 10 |
| 1 | 10 |
| 2 | 30 |
| 3 | 30 |
| 4 | 50 |
| 5 | 50 |
| 6 | 70 |
| 7 | 70 |
| 8+ | Any level (always obeys) |

### Trade Evolutions
Some Pokemon evolve when traded:
| Pokemon | Condition | Evolves Into |
|---------|-----------|-------------|
| Kadabra | Trade | Alakazam |
| Machoke | Trade | Machamp |
| Graveler | Trade | Golem |
| Haunter | Trade | Gengar |
| Onix | Trade holding Metal Coat | Steelix |
| Scyther | Trade holding Metal Coat | Scizor |
| Poliwhirl | Trade holding King's Rock | Politoed |
| Slowpoke | Trade holding King's Rock | Slowking |
| Seadra | Trade holding Dragon Scale | Kingdra |
| Porygon | Trade holding Up-Grade | Porygon2 |

---

## Time Capsule (Gen 1 Compatibility)

### Overview
The Time Capsule (located on Pokemon Center 2F) allows trading between Gen 2 (Gold/Silver/Crystal) and Gen 1 (Red/Blue/Yellow).

### Restrictions
Trading TO Gen 1 requires:
- The Pokemon must exist in Gen 1 (original 151 only)
- The Pokemon must not know any Gen 2 moves (moves #166+ are Gen 2 exclusive)
- The Pokemon must not be holding a Gen 2 item
- No eggs can be traded
- Pokemon must not be Shiny (DVs that produce Shiny in Gen 2 are valid in Gen 1 but the concept doesn't exist)

### Valid Gen 1 Pokemon (151)
All original 151 Pokemon can be traded via Time Capsule, as long as they meet the move/item restrictions.

### What Happens During Transfer
**Gen 2 -> Gen 1:**
- Stats are recalculated using Gen 1 formulas (Special replaces SpAtk/SpDef)
- Held items are removed
- Happiness is discarded
- Gender is discarded (Gen 1 has no gender)
- Shiny status is preserved via DVs (but invisible in Gen 1)

**Gen 1 -> Gen 2:**
- Special stat splits into SpAtk and SpDef (same value for both)
- No held item assigned
- Happiness set to base value
- Gender calculated from DVs
- Shiny status calculated from DVs

### Move Compatibility
**Gen 2 moves that CANNOT go to Gen 1 (examples):**
- All moves with index > 165 (Rollout, Baton Pass, Encore, Swagger, etc.)
- This means Pokemon with only Gen 2 moves cannot be traded back

**Strategy:** Teach Gen 1 TMs before trading to Gen 1. Some moves are shared between gens.

---

## Pokemon Stadium 2 Compatibility

### Features
- Fully compatible with Gold, Silver, and Crystal via Transfer Pak
- Can organize PC boxes and manage team from N64
- Provides Lv50 and Lv100 battle modes
- Includes rental Pokemon for battles
- Includes mini-games
- Mystery Gift compatibility
- Speed up Gen 2 gameplay (2x speed on N64)

### Stadium 2 Exclusive Features
- **Academy:** Learn about Pokemon, types, and battle mechanics
- **Earl's Pokemon Academy:** Detailed type chart and move information
- **Pokemon Lab:** IV/stat checker, move deleter/relearner functions
- **Gym Leader Castle:** Re-fight all 16 gym leaders and Elite Four with enhanced teams
- **R2 (Round 2):** Harder versions of all challenges after completing R1

### Battle Modes
| Mode | Rules |
|------|-------|
| Little Cup | Only basic-stage Pokemon, Level 5 |
| Poke Cup | Level 50-55, sum of levels <= 155 |
| Prime Cup | Level 100 flat, no restrictions |
| Challenge Cup | Random teams assigned |
| Free Battle | Custom rules |

---

## Mystery Gift

### Overview
- One Mystery Gift per day (based on real-time clock)
- Trade decorations for your room in New Bark Town
- Mystery Gift with Pokemon Stadium 2 grants special items
- Mystery Gift between two Gen 2 cartridges trades decorations

### Available Decorations
Carpets, dolls, posters, beds, plants, game consoles, and more. Over 30 unique decoration items.

### Stadium 2 Mystery Gift
Connecting to Stadium 2 via Mystery Gift can yield:
- Rare Berries
- Evolution Stones
- Other useful items

---

## Infrared Trading (Crystal Only)

Pokemon Crystal supports infrared communication via the Game Boy Color IR port for:
- Mystery Gift (decoration exchange)
- Quick trading between Crystal cartridges

This does NOT replace link cable — full battles and standard trading still require a link cable.
