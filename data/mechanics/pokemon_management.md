# Pokemon Crystal -- Pokemon Management

Source: pokecrystal disassembly (engine/pokemon/*.asm)

---

## Bill's PC (Box System)

### Overview (bills_pc.asm, bills_pc_top.asm)
- Accessed from any Pokemon Center PC
- Functions: Deposit, Withdraw, Move (between boxes), Change Box

### Box Structure
- 14 boxes total (NUM_BOXES)
- Each box holds up to 20 Pokemon (MONS_PER_BOX)
- Current box stored in wCurBox
- Box data stored in SRAM

### Deposit
1. Party list displayed with Pokemon names, levels, gender
2. Player selects a Pokemon to deposit
3. Cannot deposit if only 1 Pokemon in party
4. Cannot deposit eggs into PC in certain situations
5. Pokemon moved from party to current box
6. If mail attached, prompt to remove first

### Withdraw
1. Box contents displayed
2. Player selects a Pokemon to withdraw
3. Cannot withdraw if party is full (6 Pokemon)
4. Pokemon moved from current box to party

### Move Pokemon Without Mail (move_mon.asm, move_mon_wo_mail.asm)
- Move Pokemon between boxes without manual deposit/withdraw
- Handles SRAM save during box switch
- Saves game during box change to prevent data loss

### Box Change
- Switching boxes triggers a save (ChangeBoxSaveGame in save.asm)
- Current box is saved, then new box is loaded
- "SAVING... DON'T TURN OFF THE POWER" message displayed

### Implementation Details
- BillsPC_GetSelectedPokemonSpecies: reads species from box data
- CopyBoxmonSpecies: copies species list for display
- PCMonInfo: displays selected Pokemon's stats, level, type
- PC operations use wBillsPC_LoadedBox to track which box is displayed

---

## Move Learning (learn.asm)

### LearnMove Process
1. Check if any of the 4 move slots is empty
2. If empty slot found: learn move directly into that slot
3. If all 4 slots occupied: enter ForgetMove routine

### ForgetMove Routine
1. "Which move should be forgotten?" prompt
2. Display all 4 current moves
3. Player selects a move to replace
4. **HM Move Check:** IsHMMove is called -- HM moves CANNOT be forgotten
5. If player tries to forget an HM move: "HM moves can't be forgotten" message
6. If player cancels: "Stop learning [move]?" confirmation
7. If confirmed cancel: "Did not learn [move]" message

### Move Replacement
1. "1, 2, and... Poof!" text with SFX_SWITCH_POKEMON sound
2. Old move replaced with new move
3. PP set to the new move's base PP
4. If in battle and it's the current battle mon (not Transformed): update wBattleMonMoves and wBattleMonPP arrays

### Disabled Move Interaction
- If the forgotten move was the currently disabled move, disable is cleared
- wDisabledMove and wPlayerDisableCount both reset to 0

---

## Party Management

### Switch Party Mons (switchpartymons.asm)
- Reorder Pokemon in party via menu
- Swaps all data: species, struct, nickname, OT name, mail

### Party Menu (party_menu.asm)
- Displays party with: species icon, nickname, level, HP bar, status
- Options per Pokemon: Stats, Switch, Item, Cancel
- In battle: additional Fight option

### Mon Menu (mon_menu.asm, mon_submenu.asm)
- Context menu when selecting a party Pokemon
- Options vary by context (field vs battle vs PC)
- Field options: Stats, Switch, Item (Give/Take), Move (HM use), Cancel
- Includes mail read/remove options for mail-holding Pokemon

---

## Health System (health.asm)

### HealParty
- Iterates through all party Pokemon
- Skips eggs (cp EGG)
- For each non-egg Pokemon: calls HealPartyMon

### HealPartyMon
1. Clears status byte (MON_STATUS = 0)
2. Clears unused byte after status
3. Copies MaxHP to CurrentHP (full heal)
4. Calls RestoreAllPP (restores all move PP to max)

### HP Bar Calculation (ComputeHPBarPixels)
```
pixels = (currentHP * HP_BAR_LENGTH_PX) / maxHP
```
- If maxHP > 255: both values divided by 4 to fit in single byte math
- Minimum 1 pixel if HP > 0

---

## Stats Screen (stats_screen.asm)

### Display Pages
The stats screen cycles through multiple pages:

**Page 1: Main Stats**
- Pokemon name, level, gender
- HP (current/max)
- Attack, Defense, Speed, Sp.Atk, Sp.Def
- Status condition
- Item held
- Type(s)

**Page 2: Move Details**
- All 4 moves with current PP / max PP
- Move types

**Page 3: OT / DV Info**
- OT Name and ID
- Caught location and time (from caught data)

### Stat Calculation Display
- Stats shown are the fully calculated values including DVs, Stat Experience, and level
- Base stats are NOT shown to the player

---

## Experience System (experience.asm)

### Experience Groups
Pokemon use different growth rate curves:
1. **Fast** (800,000 EXP to level 100)
2. **Medium Fast** (1,000,000 EXP)
3. **Medium Slow** (1,059,860 EXP)
4. **Slow** (1,250,000 EXP)

### Level Up
- When EXP threshold crossed: level incremented
- Stats recalculated with new level
- Check for moves learned at new level
- Check for evolution conditions

---

## Breeding System (breeding.asm, breedmon_level_growth.asm)

### Egg Cycle
- breedmon_level_growth.asm handles experience tracking for Day Care Pokemon
- Each step = 1 EXP
- Pokemon in Day Care auto-learn level-up moves (oldest move deleted if full)

### Compatibility Check
- Same egg group required
- Opposite genders or Ditto
- Day Care Man reports compatibility level

---

## Caught Data (caught_data.asm)

### What's Stored
Each caught Pokemon records:
- **Time of catch:** Morning/Day/Night
- **Level caught:** The level when originally obtained
- **OT Gender:** Male or Female trainer
- **Location:** Map ID where caught

### SetCaughtData
- Called when Pokemon is caught, hatched, or received via in-game trade
- Writes to the Pokemon's caught data bytes in its struct

---

## Nickname System (correct_nick_errors.asm)

### Validation
- correct_nick_errors.asm scans for invalid characters in nicknames
- Replaces invalid bytes with valid characters
- Ensures nickname is properly terminated with '@' delimiter

### Correct Party Errors (correct_party_errors.asm)
- Validates party data integrity
- Fixes species bytes that don't match
- Ensures party count is accurate

---

## Mail System (mail.asm, mail_2.asm, european_mail.asm)

### Overview
- Pokemon can hold mail items
- Mail contains player-written messages
- Mail prevents Pokemon from being released or deposited normally

### Mail Types
9 mail types: Flower Mail, Surf Mail, LiteBlue Mail, Portrait Mail, Lovely Mail, Eon Mail, Morph Mail, BlueSky Mail, Music Mail, Mirage Mail

### Mail Mechanics
- Attaching mail: requires a mail item in bag
- Reading mail: displays the message
- Removing mail: returns mail item to bag, clears Pokemon's held item
- Sending to PC: mail must be removed first
- Trading: mail transfers with the Pokemon

### European Mail (european_mail.asm)
- Handles character encoding differences between Japanese and international versions
- Converts mail text for cross-region compatibility

---

## Type System (types.asm)

### Type Matching
- GetMoveDamageType: returns whether a move is Physical or Special based on its type
- Types 0-8 (Normal through Rock) are Physical
- Types 20+ (Fire through Dark) are Special
- This determines whether Attack/Defense or SpAtk/SpDef are used

### Type Display
- PrintMonTypes: displays a Pokemon's type(s) on the stats screen
- GetTypeName: retrieves the string name for a type

---

## Search Functions

### Search Owned (search_owned.asm)
- Searches through PC boxes for specific Pokemon
- Used by various game functions that need to find Pokemon

### Search Party (search_party.asm)
- Searches party for specific conditions
- Used for checking if player has required Pokemon/moves for field events

### Knows Move (knows_move.asm)
- Checks if a specific Pokemon knows a specific move
- Used for HM field move checks and move tutor validation

---

## Temp Mon (tempmon.asm)
- Temporary Pokemon data buffer used during operations
- CopyTempMon: copies between party/box/temp structures
- Used during trades, PC operations, and battle setup
