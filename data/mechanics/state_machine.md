# Pokemon Crystal — Internal State Machines

Source: pokecrystal disassembly (engine/battle/core.asm, engine/battle/effect_commands.asm, engine/overworld/, home/)

---

## Battle State Machine

The battle engine is a hierarchical state machine with clearly defined phases, transitions, and reset points.

### Top-Level Battle Flow

```
InitBattle
  -> DoBattle
    -> BattleTurn (main loop)
      -> [per turn]:
        1. HandleBerserkGene
        2. UpdateBattleMonInParty
        3. AIChooseMove
        4. CheckPlayerLockedIn
        5. BattleMenu (player input)
        6. ParsePlayerAction
        7. EnemyTriesToFlee (wild only)
        8. DetermineMoveOrder
        9. Execute turns (Battle_PlayerFirst or Battle_EnemyFirst)
        10. HandleBetweenTurnEffects
      -> [loop or exit]
    -> CleanUpBattleRAM
  -> GiveExperiencePoints (if won)
  -> Return to overworld
```

Source: `engine/battle/core.asm` lines 1-230

### Per-Turn Reset (BattleTurn.loop)

At the START of each turn, these are reset to 0:
- `wPlayerIsSwitching`
- `wEnemyIsSwitching`
- `wBattleHasJustStarted`
- `wPlayerJustGotFrozen`
- `wEnemyJustGotFrozen`
- `wCurDamage` (2 bytes)

Source: `engine/battle/core.asm` lines 166-173

### Move Order Determination (DetermineMoveOrder)

Priority system (checked in order):
1. **Switching vs. moves:** If one side switches and the other uses a move, the switcher goes first (but see link battle coin flip exception)
2. **Move priority:** Protect/Endure (3) > Quick Attack/Mach Punch (2) > Normal moves (1) > Counter/Mirror Coat/Roar/Whirlwind (0) > Vital Throw (0, hardcoded last)
3. **Quick Claw:** If priorities are equal, Quick Claw holder gets a random chance to go first
4. **Speed comparison:** Higher speed goes first
5. **Speed tie:** 50% coin flip (BattleRandom)

**Vital Throw special case:** Hardcoded to priority 0 BEFORE the effect priority table is checked. `cp VITAL_THROW` returns `a = 0` immediately.

**Counter/Mirror Coat/Roar/Whirlwind:** Priority 0 means they go AFTER normal moves but still participate in the priority system.

Source: `engine/battle/core.asm` lines 442-561, `data/moves/effects_priorities.asm`

### Move Execution Phase (per side)

Each turn executes in this order:
1. `EndUserDestinyBond` — clear user's Destiny Bond flag
2. `DoPlayerTurn` / `DoEnemyTurn` — execute the move:
   a. `CheckTurn` (pre-move checks)
   b. Move effect script (command-by-command)
3. `EndOpponentProtectEndureDestinyBond` — clear opponent's Protect, Endure, Destiny Bond
4. Faint checks (both sides)
5. `ResidualDamage` for the side that moved (PSN/BRN/Toxic -> Leech Seed -> Nightmare -> Curse)
6. Faint check again

Source: `engine/battle/core.asm` lines 869-960

### Pre-Move Checks (CheckTurn) — Exact Order

The CheckTurn function checks these conditions in this precise order. If any condition prevents action, execution jumps to EndTurn:

1. **CANNOT_MOVE check:** If move is $FF, skip turn
2. **Recharge:** If SUBSTATUS_RECHARGE is set, clear it and skip
3. **Sleep:** Decrement counter. If still asleep, skip (unless Snore/Sleep Talk)
4. **Freeze:** If frozen, skip (unless Flame Wheel/Sacred Fire which thaw)
5. **Flinch:** If SUBSTATUS_FLINCHED, clear it and skip
6. **Disable:** Decrement counter. If disabled move expires, announce
7. **Confusion:** Decrement counter. 50% chance to hit self in confusion
8. **Attract:** 50% chance to be immobilized by infatuation
9. **Disabled move check:** If using the disabled move, skip
10. **Paralysis:** 25% chance to be fully paralyzed

Source: `engine/battle/effect_commands.asm` lines 110-350

### Between-Turn Effects (HandleBetweenTurnEffects) — Exact Order

After BOTH sides have moved (or attempted to move):

**With faint checks between each:**
1. Future Sight countdown/damage
2. Weather (Sandstorm damage to non-Rock/Ground/Steel)
3. Wrap/Bind/Fire Spin/Clamp damage
4. Perish Song countdown (both sides)

**Without faint checks (NoMoreFaintingConditions):**
5. Leftovers healing
6. Mystery Berry PP restoration
7. Defrost (random thaw chance — note: this is the ONLY place freeze can end besides moves)
8. Safeguard countdown
9. Light Screen / Reflect countdown
10. Stat-boosting held items check
11. Healing held items check
12. UpdateBattleMonInParty (sync battle data to party)
13. Encore countdown

Source: `engine/battle/core.asm` lines 250-296

### Residual Damage Order (ResidualDamage)

Applied to the side that just moved, BEFORE the other side's ResidualDamage:

1. Poison/Burn damage (1/8 max HP; Toxic scales by 1/16 * toxic_counter)
2. Leech Seed drain (1/8 max HP, heals opponent)
3. Nightmare damage (1/4 max HP, only while sleeping)
4. Curse damage (1/4 max HP)
5. Faint check after each

Source: `engine/battle/core.asm` lines 1005-1104

### Faint Check Order

The faint check order depends on who moved first:
- If player moved first (hSerialConnectionStatus != USING_EXTERNAL_CLOCK): `CheckFaint_PlayerThenEnemy`
- If enemy moved first: `CheckFaint_EnemyThenPlayer`

This matters for simultaneous KOs (Perish Song, Destiny Bond, etc.) — whoever is checked first and found fainted "loses."

Source: `engine/battle/core.asm` lines 298-340

---

## What State Persists vs. Resets

### Persists Between Turns (within a battle)
- Stat stages (Attack, Defense, Speed, SpAtk, SpDef, Accuracy, Evasion)
- Volatile status (confusion counter, Encore counter, Disable counter, Perish Song counter)
- Substatus flags (Leech Seed, Curse, Focus Energy, Substitute, etc.)
- Wrap/trap counter
- Toxic counter
- Future Sight counter and stored damage
- Last move used (for Encore, Disable targeting)
- Fury Cutter/Rollout consecutive hit counter

### Resets Between Turns
- `wCurDamage` (zeroed at turn start)
- `wPlayerIsSwitching` / `wEnemyIsSwitching`
- `wPlayerJustGotFrozen` / `wEnemyJustGotFrozen`
- `wAttackMissed` (zeroed in CheckTurn)
- `wEffectFailed` (zeroed in CheckTurn)
- Protect/Endure (cleared after opponent's move via EndOpponentProtectEndureDestinyBond)
- Destiny Bond (cleared at start of user's turn AND after opponent's move)
- Flinch (cleared when processed in CheckTurn)

### Persists Between Battles (party data)
- HP, PP, Status conditions (sleep counter, poison, burn, freeze, paralysis)
- Experience, Level, Stats
- Stat Experience (EVs)
- Happiness
- Held items
- Pokerus status and countdown
- Caught data (OT, ID, location, time)

### Resets Between Battles (CleanUpBattleRAM)
- ALL stat stages (reset to 0/neutral)
- ALL substatus flags (confusion, curse, leech seed, encore, disable, etc.)
- Toxic counter
- wBattleMode, wBattleType
- wOtherTrainerClass
- Flee attempt counter
- Menu cursor positions
- All wPlayerSubStatus1 through wEnemyFuryCutterCount (large block zeroed)

Source: `engine/battle/core.asm` lines 8288-8318

### Persists When Switching Pokemon (within battle)
- **Carries over:** Spikes (field effect), Weather (field effect), entry hazards
- **Lost on switch-out:** ALL stat stages, confusion, Encore, Disable, Leech Seed, trapping, Curse marker, Nightmare, Focus Energy, Substitute, Perish Song counter, toxic counter (reverts to regular poison), attraction, flinch

### Baton Pass Persistence
Baton Pass specifically preserves:
- All stat stages (+/- Attack, Defense, etc.)
- Confusion
- Focus Energy
- Substitute (and its remaining HP)
- Perish Song counter
- Mean Look/Spider Web trapping
- Leech Seed
- Curse damage flag

Baton Pass clears (via ResetBatonPassStatus):
- Nightmare, Disable, Attraction, Transform, Encore
- Wrap/trap counts
- Last move used

---

## Overworld State Machine

### Map Loading Sequence

When transitioning between maps:
1. Save current map state (NPC positions, event flags)
2. Load new map data (tilemap, collision, NPCs)
3. Execute map setup scripts (setup_script_pointers.asm)
4. Initialize NPC sprites and positions
5. Check scene scripts
6. Process map triggers
7. Resume player control

### Event Processing Order (per step)

Each overworld step:
1. Joypad input processed
2. Player movement applied
3. Step counter events checked:
   - Repel counter decrement
   - Poison step damage
   - Egg step counter
   - Happiness step counter
   - Day Care experience
   - Phone call timer
4. Wild encounter check (if applicable tile)
5. Map connection/warp check
6. Trainer line-of-sight check
7. Tile-based event trigger check

### Map State Persistence

**Persists across map transitions:**
- Event flags (story progress, item pickups, trainer defeats)
- Phone contacts registered
- Day/time state
- Player money, items, Pokemon party
- Daily flags (fruit trees, haircuts, etc.)

**Resets on map load:**
- NPC positions return to defaults
- Trainer line-of-sight resets
- Music may change based on map/time

---

## Menu State Machine

### Menu Nesting

Pokemon Crystal supports nested menus with a stack-based system:
- Main menu can open submenus (Party, Pack, PokeGear, etc.)
- Each submenu can open further submenus
- `LoadMenuHeader` pushes menu state
- `ExitMenu` / `CloseWindow` pops menu state
- `ExitAllMenus` clears the entire stack

### Battle Menu Flow

```
BattleMenu
  -> FIGHT -> Move Selection -> Execute
  -> PKMN -> Party Menu -> Switch/Use Move
  -> PACK -> Item Selection -> Use Item
  -> RUN -> TryToRunAwayFromBattle
```

### Key Menu Constraints
- Battle items can only be used on the player's turn
- Cannot use certain items in certain battle types (Battle Tower, link)
- Menu cursor position persists within a battle (wBattleMenuCursorPosition)
- Last used pocket remembered (wLastPocket)

---

## VBlank Interrupt Architecture

The VBlank interrupt serves as the effective main loop:

```
VBlank (60 Hz)
  -> RNG advancement
  -> Screen register updates (SCX, SCY, WY, WX)
  -> Graphics operations (one per frame, prioritized):
    1. BG Map Buffer update
    2. Palette update (CGB)
    3. DMA Transfer
    4. BG Map update
  -> Tile serving (2bpp, 1bpp)
  -> Tileset animation
  -> OAM DMA
  -> Joypad reading
  -> Sound engine update
  -> Game Timer
```

Multiple VBlank handlers for different game states:
- VBLANK_NORMAL: Standard gameplay
- VBLANK_CUTSCENE: Cutscene mode
- VBLANK_SOUNDONLY: Only processes sound
- VBLANK_SERIAL: Serial communication mode
- VBLANK_CREDITS: Credits sequence
- VBLANK_DMATRANSFER: Large DMA operations

Source: `home/vblank.asm`

---

## Timer and Frame Counting

- `hVBlankCounter`: Incremented every VBlank (frame counter)
- Game runs at ~59.7 FPS (Game Boy hardware refresh rate)
- `DelayFrames`: Waits c frames by halting until VBlank
- Text display speed affects game pacing (options: FAST, MID, SLOW)
- Battle animations can be toggled off for speed
