# Pokemon Crystal -- Event Engine

Source: pokecrystal disassembly (engine/events/*.asm)

---

## Bug Catching Contest

### Overview
- Location: National Park (Tuesdays, Thursdays, Saturdays)
- Duration: 20 minutes (real-time timer via StartBugContestTimer)
- Player receives 20 Park Balls (BUG_CONTEST_BALLS)
- Can keep only one caught Pokemon (the best one)

### Available Pokemon
Wild Pokemon in the Bug Catching Contest are drawn from the National Park's contest encounter table (data/wild/bug_contest_mons.asm). The available species and levels are contest-specific.

### Contest Scoring Formula

Player score is calculated from the caught Pokemon's stats:

```
Score = (MaxHP * 4)
      + Attack
      + Defense
      + Speed
      + SpAtk
      + SpDef
      + DV_bonus
      + (RemainingHP / 8)
      + (1 if holding item, 0 otherwise)
```

**DV Bonus Calculation:**
```
dv_bonus = ((DVs_byte1 & 0x02) << 2)     ; Attack DV bit 1
         + ((DVs_byte1 >> 4 & 0x02) * 2) ; Defense DV bit 1
         + (DVs_byte2 & 0x02)             ; Speed DV bit 1
         + ((DVs_byte2 >> 4 & 0x02) >> 1) ; Special DV bit 1
```
This adds up to 0-12 bonus based on specific DV bits.

### AI Contestant Scoring

Each NPC contestant:
1. Randomly selects one of 3 pre-defined possible catches (index 0, 1, or 2 with equal probability, rejecting 3)
2. Each possible catch has a species and a base score
3. The base score is randomly perturbed by adding 0-7 (random AND 0b111)
4. Results are ranked against the player's score

### Judging

Winners determined by DetermineContestWinners:
1. Each contestant's score is compared against current 1st/2nd/3rd
2. If better than current Nth place, bump lower entries down
3. Player's score is compared last (after all AI contestants)

### Prizes
- 1st place: Sun Stone
- 2nd place: Everstone
- 3rd place: Gold Berry

---

## Battle Tower

### Overview
- Location: Battle Tower (after obtaining all 8 Johto badges)
- Format: 3v3 singles, Level 50 or open level (depending on mode)
- 7 consecutive battles per challenge
- Rewards for completing streaks

### Entry Rules (from rules.asm)

**Standard Battle Tower rules:**
1. Exactly 3 Pokemon required (CheckBTRule_PartyCountEq3)
2. All 3 species must be different (CheckBTRule_PartySpeciesAreUnique)
3. All 3 held items must be different (CheckBTRule_PartyItemsAreUnique)
4. No eggs allowed (CheckBTRule_HasPartyAnEgg)

**Mobile Battle rules (Japanese mobile adapter):**
1. Party must have exactly 3 Pokemon
2. Party must have 3 non-egg mons

### Battle Flow
1. InitBattleTowerChallengeRAM clears all state
2. Player battles 7 trainers sequentially
3. wNrOfBeatenBattleTowerTrainers tracks progress
4. wBattleTowerBattleEnded set to TRUE when streak complete or lost
5. Save state preserved in SRAM (sBattleTowerChallengeState)

### Streak Tracking
- Challenge state stored in SRAM
- BATTLETOWER_RECEIVED_REWARD state cleared on next save
- Streak data includes trainer IDs, scores, player data

### Trainer Loading (load_trainer.asm)
- Trainers loaded from pre-defined Battle Tower data sets
- Each trainer has 3 Pokemon with specific movesets, items, DVs
- Trainer class determined by get_trainer_class.asm

---

## Pokerus

### Contraction (de novo infection)

```
Probability: 3/65536 per step cycle (~0.0046%)
```

Conditions for contraction:
1. Must have visited Goldenrod City at least once (STATUSFLAGS2_REACHED_GOLDENROD_F)
2. No party member currently has active Pokerus (low nibble != 0)
3. Random check: hRandomAdd must be 0 AND hRandomSub must be < 3
4. Random party member selected (0 to partyCount-1)
5. If selected mon already had Pokerus (high nibble != 0), abort

### Strain and Duration

The Pokerus byte format: `SSSS DDDD`
- High nibble (S): Strain (1-15)
- Low nibble (D): Days remaining (1-4)

Generation:
```
1. Generate random byte, reject 0
2. If high nibble is 0: use byte as-is for strain/duration
3. Otherwise:
   - Strain = (random & 0x07) + 1 (values 1-8)
   - Duration = (strain_value & 0x03) + 1 (values 1-4)
   - Final byte = (strain << 4) | duration
```

### Spreading

Conditions for spread:
1. 1/3 chance per step cycle (33% when an infected mon exists)
2. Party must have > 1 Pokemon
3. Direction: 50% chance to check forward, 50% backward in party
4. Stops at: cured Pokemon (low nibble = 0 with high nibble set), end of party

When infecting adjacent mon:
```
new_strain = infected_mon_strain (high nibble preserved)
new_duration = (strain & 0x03) + 1
```

### Curing
- Duration decremented daily at midnight
- When duration reaches 0: strain preserved, duration stays 0
- Cured Pokemon (high nibble set, low nibble 0) cannot be re-infected
- Cured Pokemon retains the 2x Stat Experience bonus permanently

### Shuckle Berry Conversion (same function)
- 6.25% chance per step cycle (1/16)
- If a Shuckle in party holds a Berry, converts it to Berry Juice
- Also requires Goldenrod visit flag

---

## Day Care (engine/events/daycare.asm)

### Egg Production
- See breeding_mechanics.md for full details
- Step counter at 256 steps checks compatibility
- Compatibility determines egg chance per cycle

### Level Gain
- Pokemon gain 1 EXP per step
- Day Care man gives different messages based on egg availability

---

## Daily Events

### Fruit Trees (fruit_trees.asm)
- Berry-producing trees throughout Johto/Kanto
- One berry per tree per day
- Berry type varies by tree location

### Haircut Brothers (haircut.asm)
- Goldenrod Tunnel, available once per day
- Older brother: available any day, lower happiness gain
- Younger brother: available specific days, higher happiness gain
- Random happiness increase amount

### Lucky Number (lucky_number.asm)
- Radio Tower lottery, once per week (reset on specific day)
- Compares drawn number against OT IDs of all party/box Pokemon
- Matching last digits = prizes (1 digit = Berry, 5 digits = Master Ball)

### Buena's Password (buena.asm, buena_menu.asm)
- Radio program gives daily password
- Tell Buena the password in the Radio Tower at night
- Earn Blue Card points, exchangeable for prizes

### Mom's Savings (mom.asm, mom_phone.asm)
- Mom saves a fraction of prize money
- She occasionally buys items with saved money
- Items delivered to player via phone call or upon visiting home

### Kurt's Apricorn Balls (kurt.asm)
- Give Kurt an Apricorn, wait one day
- Each Apricorn color produces a specific ball type
- kurt_selectquantity_interpretjoypad.asm handles quantity selection

---

## Special Events

### Celebi Event (celebi.asm)
- GS Ball event triggers Celebi encounter
- wBattleResult has BATTLERESULT_CAUGHT_CELEBI flag

### Magikarp Size Check (magikarp.asm)
- Measures Magikarp length based on DVs and level
- Lake of Rage records tracked

### Odd Egg (odd_egg.asm)
- Special egg from Daycare with enhanced shiny odds
- Contains one of several baby Pokemon

### NPC Trades (npc_trade.asm)
- In-game trades with specific NPCs
- Each trade has defined species, DVs, OT, nickname

### Move Deleter (move_deleter.asm)
- Can delete any move, including HMs
- Located in Blackthorn City

### Move Tutor (move_tutor.asm)
- Teaches specific moves
- Crystal-exclusive feature

### Name Rater (name_rater.asm)
- Renames Pokemon
- Cannot rename traded Pokemon (different OT)

### Poke Seer (poke_seer.asm)
- Tells where and when a Pokemon was caught
- Reads caught data fields

---

## Field Move Effects

### Implemented Field Moves (field_moves.asm)
- Cut, Surf, Fly, Strength, Flash, Whirlpool, Waterfall, Rock Smash
- Headbutt, Dig, Teleport, Softboiled/Milk Drink, Sweet Scent

### Sweet Scent (sweet_scent.asm)
- Forces a wild encounter when used in grass/caves
- No effect indoors or on water

### Fishing (fish.asm, fishing_gfx.asm)
- Old Rod, Good Rod, Super Rod
- Each rod has different encounter tables and levels
- Fishing requires facing water
- "Oh! A bite!" -> timing minigame

### Repel (repel.asm)
- Counter decremented per step
- Prompt to use another when expired

### Poison Step (poisonstep.asm)
- Poison damage applied every 4 steps outside battle
- Pokemon faints at 1 HP in Gen 2 (does not faint from field poison in Crystal; reduced to 1 HP)

---

## Whiteout/Blackout (whiteout.asm)
- Triggered when all party Pokemon faint
- Player returned to last Pokemon Center
- Half of money lost (money / 2)

---

## Hall of Fame (halloffame.asm)
- Records party when defeating the Champion
- Up to NUM_HOF_TEAMS entries stored in SRAM
- Entries shifted down when new one added (oldest dropped)
- Virtual Console hook: enables GS Ball event after first Hall of Fame entry

---

## Special Overworld Scripts (specials.asm, std_scripts.asm)
- Standard collision scripts for map objects
- Trainer encounter scripts (trainer_scripts.asm)
- Magnet Train animation (magnet_train.asm)
- Elevator control (elevator.asm)
- Heal machine animation (heal_machine_anim.asm)
- Map name popup (map_name_sign.asm)
