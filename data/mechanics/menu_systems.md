# Pokemon Crystal -- Menu Systems

Source: pokecrystal disassembly (engine/menus/*.asm)

---

## Start Menu (start_menu.asm)

### Menu Options
The start menu options vary based on game progress:

**Standard options:**
1. **Pokedex** (if received)
2. **Pokemon** (party menu)
3. **Pack** (bag/items)
4. **[Player Name]** (trainer card)
5. **Save**
6. **Option**
7. **Exit**

Additional options may appear based on flags (e.g., Pokegear after receiving it).

---

## Options Menu (options_menu.asm)

### Available Options (8 total)

| Option | Values | Default |
|--------|--------|---------|
| TEXT SPEED | Fast, Med, Slow | Med |
| BATTLE SCENE | On, Off | On |
| BATTLE STYLE | Shift, Set | Shift |
| SOUND | Mono, Stereo | Mono |
| PRINT | Lightest, Lighter, Normal, Darker, Darkest | Normal |
| MENU ACCOUNT | On, Off | Off |
| FRAME | Type 1-8 | Type 1 |
| CANCEL | (exits options) | -- |

### Text Speed
- Fast: TEXT_DELAY_FAST (1 frame per character)
- Medium: TEXT_DELAY_MED (3 frames per character)
- Slow: TEXT_DELAY_SLOW (5 frames per character)
- Stored in low nibble of wOptions

### Battle Scene
- On: battle animations play
- Off: battle animations skip (faster battles)
- Stored as bit in wOptions

### Battle Style
- Shift: prompted to switch Pokemon when opponent's mon faints
- Set: no switch prompt (competitive style)
- Stored as bit in wOptions

### Sound
- Mono: all sound through both speakers
- Stereo: pseudo-stereo panning
- Stored as bit in wOptions

### Print
- Controls Game Boy Printer darkness level
- 5 levels from Lightest to Darkest

### Menu Account
- On: shows money in the start menu
- Off: hides money display

### Frame
- 8 different text box border styles (Type 1 through Type 8)
- Changes the visual border around all text boxes

### Known Bug
- Options menu fails to clear joypad state on initialization, meaning a button press from the previous screen can inadvertently change a setting

---

## Save System (save.asm)

### Save Process (_SaveGameData)
1. Set wSaveFileExists = TRUE
2. Stage RTC time for save (StageRTCTimeForSave)
3. Backup Mystery Gift data (BackupMysteryGift)
4. Write primary save:
   - ValidateSave (write validation marker)
   - SaveOptions (wOptions to SRAM)
   - SavePlayerData (player name, ID, position, badges, money, etc.)
   - SavePokemonData (party, Pokedex, PC boxes metadata)
   - SaveBox (current box contents)
   - SaveChecksum (integrity check)
5. Write backup save (identical structure):
   - ValidateBackupSave
   - SaveBackupOptions
   - SaveBackupPlayerData
   - SaveBackupPokemonData
   - SaveBackupChecksum
6. UpdateStackTop (debug: track max stack depth)
7. BackupPartyMonMail
8. BackupGSBallFlag
9. SaveRTC (real-time clock state)
10. Clear Battle Tower reward state if applicable

### Save Corruption Protection
- **Dual save slots:** Primary and backup saves are both written
- **Checksums:** Each save has a checksum to detect corruption
- **Validation markers:** ValidateSave/ValidateBackupSave write magic bytes
- **Player ID comparison:** CompareLoadedAndSavedPlayerID checks if save belongs to current player

### Box Change Save
- Switching PC boxes triggers a forced save (ChangeBoxSaveGame)
- "SAVING... DON'T TURN OFF THE POWER" message shown
- Current box saved to SRAM, new box loaded from SRAM
- This prevents box data loss if power is cut during box operations

### What Gets Saved
- Player data: name, ID, gender, position, direction, money, coins, badges
- Pokemon data: party, PC box metadata, Pokedex flags
- Current box contents (only one box loaded in WRAM at a time)
- Options (text speed, sound, battle style, etc.)
- Event flags and variables
- RTC state
- Mystery Gift data
- Hall of Fame entries
- Link battle statistics
- Battle Tower challenge state

### Save File Detection
- wSaveFileExists checked on boot
- If another player's save exists (different player ID): warning message
- Player can choose to overwrite or keep existing save

### Overwrite Protection
- AskOverwriteSaveFile checks:
  1. If no save exists: proceed (after erasing any remnants)
  2. If save exists with same player ID: "Already a save file. Is it OK to overwrite?"
  3. If save exists with different player ID: "Another save file..." warning

### Erase Previous Save (ErasePreviousSave)
When overwriting:
1. EraseBoxes (all 14 boxes cleared)
2. EraseHallOfFame
3. EraseLinkBattleStats
4. EraseMysteryGift
5. SaveData
6. EraseBattleTowerStatus
7. Clear stack top tracker

### Hall of Fame Save (AddHallOfFameEntry)
- Up to NUM_HOF_TEAMS entries stored
- New entry shifts all existing entries down by one
- Oldest entry (if at capacity) is lost
- Virtual Console hook: enables GS Ball event after first Hall of Fame

---

## Main Menu (main_menu.asm)

### Menu Options
1. **NEW GAME** -- Start a new game
2. **CONTINUE** -- Load saved game (only if save exists)
3. **OPTION** -- Access options menu
4. **MYSTERY GIFT** -- Mystery Gift (if unlocked)
5. **MOBILE** -- Mobile Adapter (Japanese version, unused in international)

### Continue Screen
- Displays save file info: player name, badges, Pokedex count, play time

---

## Intro Menu (intro_menu.asm)
- New game setup sequence
- Calls init_gender.asm for player gender selection (Crystal exclusive)

### Gender Selection (init_gender.asm)
- Crystal-exclusive feature
- Player chooses "Are you a boy? Or are you a girl?"
- Sets wPlayerGender

---

## Naming Screen (naming_screen.asm)

### Character Limits
| Context | Max Length |
|---------|-----------|
| Player name | 7 characters |
| Pokemon nickname | 10 characters |
| Box name | 8 characters |

### Available Characters
- Upper case: A-Z
- Lower case: a-z
- Special characters: various punctuation marks
- Character sets vary by context and language version

### Input
- D-pad navigates character grid
- A selects character
- B deletes last character
- START confirms name
- DEL option on screen to delete characters

---

## Scrolling Menu (scrolling_menu.asm)

### General Purpose Menu Engine
- Used by item lists, move lists, PC Pokemon lists
- Supports scrolling through lists longer than screen
- Handles cursor movement, scrollbar display
- Up/Down scrolls list
- A selects, B cancels

---

## Delete Save (delete_save.asm)

### Process
- Accessible from main menu with button combination (Up + Select + B on title screen)
- Confirmation prompt before deletion
- Erases all SRAM save data

---

## Trainer Card (trainer_card.asm)

### Displayed Information
- Player name and ID number
- Money
- Pokedex: Seen / Caught counts
- Badges: visual display of collected badges
- Play time

---

## Menu Engine (menu.asm, menu_2.asm)

### Core Menu Functions
- LoadMenuHeader: sets up menu dimensions, options, and callbacks
- VerticalMenu: standard vertical selection menu
- PlaceYesNoBox: simple Yes/No prompt
- Textbox: draws a bordered text box
- ScrollingMenu: handles long scrollable lists

### Menu Data Format
Each menu has:
- Flags (backup tiles, cursor style)
- Coordinates (start/end positions)
- Data pointer (items, strings)
- Default selection

---

## Empty SRAM (empty_sram.asm)

### Initialization
- Called on first boot or after save deletion
- Fills all SRAM banks with 0
- Sets up default box names
- Initializes save validation bytes to empty state
