# Pokemon Crystal -- Mini-Games

Source: pokecrystal disassembly (engine/games/*.asm)

---

## Slot Machine (Game Corner, Goldenrod City)

### Overview
- Cost: 1, 2, or 3 coins per spin
- Coin case maximum: 9999 coins
- 3 reels, each with 15 symbols
- Player presses A to stop each reel sequentially

### Reel Compositions

**Reel 1 (15 symbols):**
| Position | Symbol |
|----------|--------|
| 0 | Seven |
| 1 | Cherry |
| 2 | Staryu |
| 3 | Pikachu |
| 4 | Squirtle |
| 5 | Seven |
| 6 | Cherry |
| 7 | Staryu |
| 8 | Pikachu |
| 9 | Squirtle |
| 10 | Pokeball |
| 11 | Cherry |
| 12 | Staryu |
| 13 | Pikachu |
| 14 | Squirtle |

**Reel 2 (15 symbols):**
| Position | Symbol |
|----------|--------|
| 0 | Seven |
| 1 | Pikachu |
| 2 | Cherry |
| 3 | Squirtle |
| 4 | Staryu |
| 5 | Pokeball |
| 6 | Pikachu |
| 7 | Cherry |
| 8 | Squirtle |
| 9 | Staryu |
| 10 | Pokeball |
| 11 | Pikachu |
| 12 | Cherry |
| 13 | Squirtle |
| 14 | Staryu |

**Reel 3 (15 symbols):**
| Position | Symbol |
|----------|--------|
| 0 | Seven |
| 1 | Pikachu |
| 2 | Cherry |
| 3 | Squirtle |
| 4 | Staryu |
| 5 | Pikachu |
| 6 | Cherry |
| 7 | Squirtle |
| 8 | Staryu |
| 9 | Pikachu |
| 10 | Pokeball |
| 11 | Cherry |
| 12 | Squirtle |
| 13 | Staryu |
| 14 | Pikachu |

Symbol counts across all reels:
- Seven: 2/2/1 = 5
- Pokeball: 1/2/1 = 4
- Cherry: 3/3/3 = 9
- Pikachu: 3/3/4 = 10
- Squirtle: 3/3/3 = 9
- Staryu: 3/3/3 = 9

### Payout Table

| Match | Payout (coins) |
|-------|---------------|
| Seven-Seven-Seven | 300 |
| Pokeball-Pokeball-Pokeball | 50 |
| Cherry-Cherry-Cherry | 6 |
| Pikachu-Pikachu-Pikachu | 8 |
| Squirtle-Squirtle-Squirtle | 10 |
| Staryu-Staryu-Staryu | 15 |

### Lines Checked (by bet amount)

| Bet | Lines Checked |
|-----|---------------|
| 1 coin | Middle row only |
| 2 coins | Middle + Top + Bottom rows |
| 3 coins | Middle + Top + Bottom + Both diagonals |

### Bias System (Internal Manipulation)

On each spin, the game determines a "bias" -- a preferred outcome symbol. The bias system is **not** random:

**Normal Machine Bias Probabilities:**
| Threshold | Symbol | Cumulative % |
|-----------|--------|-------------|
| ~1% | Seven | ~1% |
| ~1% | Pokeball | ~1% |
| ~4% | Staryu | ~3% |
| ~8% | Squirtle | ~4% |
| ~16% | Pikachu | ~8% |
| ~19% | Cherry | ~3% |
| 100% | No bias | ~81% |

**Lucky Machine Bias Probabilities:**
| Threshold | Symbol | Cumulative % |
|-----------|--------|-------------|
| ~1% | Seven | ~1% |
| ~1% | Pokeball | <1% |
| ~3% | Staryu | ~2% |
| ~6% | Squirtle | ~3% |
| ~12% | Pikachu | ~6% |
| ~31% | Cherry | ~19% |
| 100% | No bias | ~69% |

**Lucky vs Normal determination:** On game init, there's a 12.5% chance the machine is flagged as "lucky" (wKeepSevenBiasChance = TRUE). The remaining 87.5% is normal mode.

### Reel Manipulation Mechanics

When bias is active:
1. **Reel 1:** Manipulates up to 4 extra positions to find the biased symbol anywhere on the visible reel
2. **Reel 2:** Manipulates up to 4 positions to line up the biased symbol with reel 1
3. **Reel 3:** Manipulates up to 4 positions to complete the match (if biased) or avoid a match (if no bias)

**Special Reel 2 "Skip to 7" mode:** When bet >= 2 coins and reel 1 shows a Seven, there's a 31.25% chance reel 2 will fast-spin to line up its Seven. This is mostly cosmetic tension -- reel 3 then has special behavior:

**Reel 3 special modes when both reels show Seven:**
- If biased to Seven: 29.7% normal stop, 23.4% slow advance, 23.4% Golem, 23.4% Chansey
- If NOT biased to Seven: 37.5% normal stop, 31.3% slow advance, 31.3% Golem, 0% Chansey

Golem mode: Golem drops down and shakes the machine, advancing the reel a variable number of slots
Chansey mode: Only available with Seven bias; Chansey walks across and drops an egg, advancing reel 3 by 17 positions repeatedly until Sevens line up

### Seven Streak System

After lining up Sevens, the game decides whether to keep the Seven bias for the next spin:
- Normal mode (87.5% of machines): 25% chance to keep Seven bias
- "Lucky" mode (12.5% of machines): 12.5% chance to keep Seven bias

NOTE: The comments in the code say this seems inverted -- the rarer mode has worse streak odds, suggesting a possible developer bug.

### Known Bug
- Payout sound effects cut each other off during coin dispensing

---

## Card Flip (Game Corner, Goldenrod City)

### Overview
- Cost: 3 coins per round
- Deck: 24 cards (4 Pokemon x 6 levels), shuffled randomly
- Up to 12 rounds per deck before reshuffling

### Card Types
- 4 Pokemon: Pikachu, Jigglypuff, Poliwag, Oddish
- 6 Levels: 1 through 6
- Total: 24 unique cards

### Gameplay
1. Two cards dealt face-down (top and bottom)
2. Player chooses one card to flip
3. Unchosen card is hidden
4. Player places a bet on the grid (species, level, or combination)
5. Chosen card is revealed
6. Win/loss determined by bet type

### Bet Types and Payouts

| Bet Type | Description | Payout | True Odds |
|----------|-------------|--------|-----------|
| Pokemon pair (Pika/Jiggly or Poli/Oddish) | 2 of 4 possible species | 6 coins | 1 in 2 |
| Level pair (1-2, 3-4, or 5-6) | 2 of 6 possible levels | 9 coins | 1 in 3 |
| Single Pokemon | 1 of 4 possible species | 12 coins | 1 in 4 |
| Single Level | 1 of 6 possible levels | 18 coins | 1 in 6 |
| Exact card (Pokemon + Level) | 1 of 24 possible cards | 72 coins | 1 in 24 |

### Expected Value Analysis
- Cost per play: 3 coins
- Pokemon pair: EV = 6 * (1/2) - 3 = 0 (break even)
- Level pair: EV = 9 * (1/3) - 3 = 0 (break even)
- Single Pokemon: EV = 12 * (1/4) - 3 = 0 (break even)
- Single Level: EV = 18 * (1/6) - 3 = 0 (break even)
- Exact card: EV = 72 * (1/24) - 3 = 0 (break even)

All bets are theoretically fair. However, discarded cards are tracked and visible on the board, so card counting gives the player an edge over time.

### Discard Tracking
- Previously revealed cards are marked on the right-side grid
- Players can track which cards have already been seen
- This reduces the effective pool, shifting odds in the player's favor for later rounds

---

## Unown Puzzle (Ruins of Alph)

### Overview
- Slide puzzle with 16 pieces in a 6x6 grid (4x4 inner playfield)
- Pieces can be picked up and placed in any empty border/edge slot
- 4 different puzzles available

### Puzzles
1. **Kabuto** (fossil Pokemon)
2. **Omanyte** (fossil Pokemon)
3. **Aerodactyl** (fossil Pokemon)
4. **Ho-Oh** (legendary)

### Mechanics
- 6x6 grid: outer ring is border, inner 4x4 is the puzzle area
- 16 pieces numbered 1-16
- Pieces start in random positions around the border
- Player moves cursor with D-pad, picks up/places pieces with A
- Only one piece can be held at a time
- Pieces can only be placed on empty slots
- START button quits

### Solved Configuration
```
00 00 00 00 00 00
00 01 02 03 04 00
00 05 06 07 08 00
00 09 10 11 12 00
00 13 14 15 16 00
00 00 00 00 00 00
```
(00 = border/empty, pieces 1-16 in reading order)

### Completion
- Solving the puzzle triggers SFX_1ST_PLACE sound
- Sets wSolvedUnownPuzzle flag to TRUE
- Unlocks Unown encounters in the adjacent chamber

---

## Memory Game (Unused / Japanese-only)

### Overview
- 9x5 grid of 45 face-down cards
- Match pairs of identical cards
- 5 attempts to find matching pairs
- Cards use Pokemon sprite tiles as their faces

### Card Types
8 different card types, distributed across 45 positions:
- Distribution varies by difficulty (3 difficulty levels, selected from menu)

**Level 1 distribution:** 2, 3, 6, 6, 6, 8, 8, 6
**Level 2 distribution:** 2, 2, 4, 6, 6, 8, 8, 9
**Level 3 distribution:** 2, 2, 2, 4, 7, 8, 8, 12

### Gameplay
1. Cards are shuffled randomly into the 45-position grid
2. Player gets 5 tries
3. Each try: select 2 cards to flip
4. If they match: both removed, match recorded, displayed at top
5. If no match: cards flip back face-down after 64 frames
6. After 5 tries (or all matched): all remaining cards revealed
7. Player can play again or quit

### Implementation Notes
- The memory game uses Japanese text strings ("とったもの" = "Taken items", "あと　かい" = "Remaining times")
- MemoryGameGFX placeholder exists in card_flip.asm (the graphics were planned but the game was cut from the international release)
- The cursor is a sprite animation object that moves in tile-sized increments
