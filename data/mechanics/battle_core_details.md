# Pokemon Crystal -- Battle Core Details

Source: pokecrystal disassembly (`engine/battle/core.asm`, 9147 lines)

This document covers exact implementation details from `core.asm` that are NOT already in `battle_mechanics.md`. Cross-reference that file for turn structure, between-turn effects, switching basics, experience formula, flee formula, multi-hit distribution, Protect mechanics, and weather system.

---

## Battle Initialization

### StartBattle (line 8020)
- Aborts silently if `wPartyCount == 0` (walk-through-walls edge case)
- Saves `wTimeOfDayPal`, calls `BattleIntro` -> `DoBattle` -> `ExitBattle`, restores palette

### BattleIntro (line 8042)
- Sequence: LoadTrainerOrWildMonPic -> PlayBattleMusic -> ShowLinkBattleParticipants -> FindFirstAliveMonAndStartBattle -> ClearBattleRAM -> InitEnemy -> sliding pic animation -> BattleStartMessage
- For wild battles, `UpdateEnemyHUD` is called during intro; for trainers it is not

### InitEnemyTrainer (line 8122)
- Sets `wBattleMode = TRAINER_BATTLE`
- Calls `GetTrainerAttributes` then `ReadTrainerParty` to build OT party
- Special case: RIVAL1's first mon always has its item cleared (`wOTPartyMon1Item = 0`)
- If the trainer is a gym leader (`IsGymLeader`), every non-fainted party mon gets `HAPPINESS_GYMBATTLE` applied before the battle starts

### InitEnemyWildmon (line 8177)
- Sets `wBattleMode = WILD_BATTLE`
- Calls `LoadEnemyMon` to generate the wild Pokemon
- Copies moves/PP to `wWildMonMoves`/`wWildMonPP` (backup for catching)

---

## LoadEnemyMon — Wild Pokemon Generation (line 5957)

### Item Assignment
- Trainer battles: item comes from OT party struct
- Wild battles with `BATTLETYPE_FORCEITEM` (Ho-Oh, Lugia, Snorlax): always get `wBaseItem1`
- Normal wild: 75% no item, 23% Item1, 2% Item2
  - Roll 1: random < 75% => no item
  - Roll 2 (if item): random < 8% => Item2, else Item1

### DV Generation
- **Trainer battles**: DVs from `GetTrainerDVs` (class-based preset)
- **Roaming Pokemon** (Entei, Raikou): DVs stored in `wRoamMonDVs`; initialized randomly on first encounter, preserved across encounters
- **Forced shiny** (`BATTLETYPE_FORCESHINY`, Red Gyarados): DVs = `$EA, $AA` (ATKDEFDV_SHINY, SPDSPCDV_SHINY)
- **Normal wild**: two random bytes

### Unown Letter Filtering
- DVs determine Unown letter; if the letter hasn't been unlocked yet via `wUnlockedUnowns`, DVs are re-rolled
- BUG: Combined with forced shiny battletype, this causes an infinite loop (shiny DVs always produce the same letter)

### Magikarp Length Filtering
- BUG: Length limits use untranslated mm values but the length is stored in feet/inches, making the size filters ineffective
- BUG: Lake of Rage Magikarp are supposed to be longer but the comparison logic is inverted, making them shorter

### Status/HP
- Wild: tree encounters may start asleep (`TREEMON_SLEEP_TURNS`) based on time-of-day lists
- Roaming: HP stored as single byte (lo byte only, since L40 Raikou/Entei have <256 HP); preserved between encounters
- Trainer: HP and status copied from OT party struct

### BUG: PRZ and BRN stat reductions (line 6413)
- `LoadEnemyMon` returns without calling `ApplyStatusEffectOnEnemyStats` for switched-in trainer Pokemon
- Result: a paralyzed or burned Pokemon that switches in does NOT have its Speed or Attack halved until the next stat recalculation

---

## Status Effect Stat Modifications

### ApplyPrzEffectOnSpeed (line 6579)
- If paralyzed: Speed = Speed / 4 (two right shifts)
- Minimum Speed = 1

### ApplyBrnEffectOnAttack (line 6624)
- If burned: Attack = Attack / 2 (one right shift)
- Minimum Attack = 1

These are applied during `InitBattleMon`/`InitEnemyMon` setup, NOT dynamically during battle.

---

## Badge Stat Boosts (line 6762)

Applied to player's battle mon only (not in link or Battle Tower):

| Badge | Stat Boosted |
|-------|-------------|
| ZephyrBadge | Attack |
| PlainBadge | Speed (swapped with MineralBadge in code) |
| MineralBadge | Defense (swapped with PlainBadge in code) |
| GlacierBadge | Sp. Attack + Sp. Defense |

- Each boost: stat += stat / 8 (12.5% increase)
- Cap at 999
- BUG: GlacierBadge Sp. Defense boost may not apply — the code checks every-other badge using `srl b` + `call c, BoostStat`, but the Sp. Defense boost depends on the carry flag from the Sp. Attack boost's cap check rather than the badge bit

---

## Stat Level Multiplier System (line 6665)

### ApplyStatLevelMultiplier
- Uses `StatLevelMultipliers_Applied` table (included from `data/battle/stat_multipliers.asm`)
- For each of the 5 battle stats: `stat = baseStat * multiplier_num / multiplier_den`
- Cap at 999, minimum 1
- Called during Baton Pass (`PassedBattleMonEntrance`) to apply inherited stat stages to new mon

---

## Fainting Logic

### HandleEnemyMonFaint (line 2003)
1. Call `FaintEnemyPokemon` (animation, "foe fainted" text)
2. If player mon also fainted, call `FaintYourPokemon`
3. Set `wWhichMonFaintedFirst = 0` (enemy fainted first)
4. Call `UpdateBattleStateAndExperienceAfterEnemyFaint`
5. Check if player has any alive mons; if not, `LostBattle`
6. Wild battle: set `wBattleEnded = 1`, end
7. Trainer: check all OT mons fainted -> `WinTrainerBattle`
8. If player mon also fainted: ask to switch, handle double switch

### HandlePlayerMonFaint (line 2603)
1. Call `FaintYourPokemon` (cry, animation, "fainted" text)
2. If enemy also fainted, call `FaintEnemyPokemon`
3. Set `wWhichMonFaintedFirst = 1` (player fainted first)
4. Call `UpdateFaintedPlayerMon` (happiness penalty, battle result)
5. Check alive mons; if none, `LostBattle`
6. If enemy also fainted: award exp, check trainer defeated
7. Force player to choose next mon

### UpdateFaintedPlayerMon (line 2652)
- Clears participant bit from `wBattleParticipantsNotFainted`
- Clears enemy IN_LOOP substatus
- Clears battle mon status, updates party struct
- Happiness reduction: `HAPPINESS_FAINTED` normally, `HAPPINESS_BEATENBYSTRONGFOE` if enemy level > (player level + 30)
- Sets `wBattleResult` to LOSE

### DoubleSwitch (line 2071)
- Occurs when both sides need to send out new Pokemon
- Order depends on link clock (`hSerialConnectionStatus`)
- Both sides get Spikes damage

---

## Experience Award Implementation (line 6977)

### GiveExperiencePoints
- Skipped in link battles and Battle Tower
- Calls `.EvenlyDivideExpAmongParticipants` to divide base stats by participant count
- For each alive party mon that participated:
  1. Award stat experience (base stats added to stat exp, doubled if Pokerus)
  2. Calculate base EXP: `baseExp * enemyLevel / 7`
  3. Apply 1.5x multiplier for: traded mon, trainer battle, Lucky Egg (each independent, can stack)
  4. Cap at max exp for level 100
  5. Check level up: if level increased, recalculate stats, learn moves, set evolution flag

### Level Up During Battle
- New max HP calculated; difference added to current HP (heals the amount gained)
- If the leveling mon is the active battle mon:
  - Battle stats updated from party struct
  - Stat level multipliers re-applied
  - Status stat effects (PRZ/BRN) re-applied
  - Badge boosts re-applied
  - HUD updated

### Exp Share Implementation (line 2154)
- `IsAnyMonHoldingExpShare`: scans party, returns bitmask in `d` and count in `e`
- If any mon holds Exp Share: base stats halved before first distribution
- First pass: exp to participants using halved base stats
- Second pass: exp to Exp Share holders using original (backed up) base stats
- A mon that both participated AND holds Exp Share gets BOTH distributions

---

## Reward Money (WinTrainerBattle, line 2347)

### Amulet Coin
- Checked via `CheckAmuletCoin` when sending out player mon
- If active: `wBattleReward` is doubled

### Money Distribution (line 2415)
- Base reward doubled by Amulet Coin if applicable
- Mom's saving mode splits quarters of reward:
  - `MOM_SAVING_SOME_MONEY`: 1/4 to mom, 3/4 to player
  - `MOM_SAVING_HALF_MONEY`: 2/4 to mom, 2/4 to player
  - `MOM_SAVING_ALL_MONEY` (plus conditions): 3/4 to mom, 1/4 to player
- All amounts capped at MAX_MONEY (999999)
- Pay Day money: doubled by Amulet Coin, added to wallet at battle end

---

## AI Switching Logic

### FindMonInOTPartyToSwitchIntoBattle (line 3236)
For each alive OT party mon (excluding current):
1. **LookUpTheEffectivenessOfEveryMove**: Check if ANY of the OT mon's moves is super effective against the player's current types. If so, set bit 0 of `wEnemyEffectivenessVsPlayerMons`.
2. **IsThePlayerMonTypesEffectiveAgainstOTMon**: Check if the player mon's types are super effective against this OT mon. If the OT mon has a SE move AND player is SE against it, these cancel out (reset bit).

### ScoreMonTypeMatchups (line 3357)
- Priority: choose a mon that has super effective moves against player AND isn't weak to player's types
- Fallback: choose a mon the player isn't super effective against
- Last resort: random alive mon (random & 7, retry if invalid)

### CheckWhetherSwitchmonIsPredetermined (line 3170)
- Link battle: switchmon from `wBattleAction`
- `wEnemySwitchMonIndex` set externally (e.g. by AI): use that
- Battle just started: use mon 0
- Otherwise: AI selects via `FindMonInOTPartyToSwitchIntoBattle`

### Shift Mode (EnemySwitch, line 3128)
- Only offered if: not battle start, player has >1 mon, not linked, battle style is SHIFT
- Player's current mon must not be fainted
- If player accepts: player switches first, then enemy sends out

---

## Player Switching Implementation

### TryPlayerSwitch (line 5150)
- Can't switch if same mon already out
- Can't switch if trapped (`wPlayerWrapCount > 0` or `SUBSTATUS_CANT_RUN`)
- Selected mon must be alive

### BattleMonEntrance (line 5243)
1. Display withdraw text (varies based on enemy HP % change since switch-in)
2. 50-frame delay
3. Clear RAGE substatus
4. Check for Pursuit (enemy turn, `PursuitSwitch`)
5. If not KO'd by Pursuit: play recall animation
6. Initialize new battle mon (stats, stat levels, statuses)
7. Spikes damage on switch-in

### PursuitSwitch (line 4153)
- Checks if opponent's current move is EFFECT_PURSUIT
- If so: executes the opponent's turn (DoPlayerTurn or DoEnemyTurn) against the switching mon
- Sets move to CANNOT_MOVE after execution (prevents double execution)
- If switch-in is KO'd: play faint animation, return carry (skip recall animation)

---

## Withdraw/Send-out Text Selection

### SendOutMonText (line 7616)
- Based on enemy HP percentage:
  - >= 70%: "Go! <MON>!"
  - 40-69%: "Do it! <MON>!"
  - 10-39%: "Go for it, <MON>!"
  - < 10%: "Your foe's weak! Get'm, <MON>!"
- Link battles: always "Go! <MON>!" if battle just started
- BUG: If enemy max HP < 4, the division by (maxHP/4) divides by zero, freezing the game

### WithdrawMonText (line 7705)
- Based on enemy HP % lost since switch-in:
  - 0% lost: "That's enough! Come back!"
  - 1-29%: "Come back!"
  - 30-69%: "OK! Come back!"
  - >= 70%: "Good! Come back!"

---

## Battle Menu System (line 4875)

### Menu Options
- Position 1: Fight -> `MoveSelectionScreen`
- Position 2: PKMN -> party menu with Switch/Stats/Cancel
- Position 3: Pack -> items (blocked in link/Battle Tower)
- Position 4: Run -> `TryToRunAwayFromBattle`
- Contest battles use a special menu (`ContestBattleMenu`)

### MoveSelectionScreen (line 5326)
- Checks if player has usable moves; forces Struggle if none
- Displays move list with PP, type, and current/max PP
- SELECT key: swap moves (updates both battle struct and party struct, unless Transformed)
- Disabled move: shows "Disabled!" instead of type info
- No PP: "There's no PP left for this move!" and re-shows menu

### CheckPlayerHasUsableMoves (line 5734)
- If no disable: OR all 4 PP values; if zero, Struggle
- If disabled: check all PP except the disabled move slot
- BUG: PP_MASK applied incorrectly means a disabled move with PP Ups but 0 base PP left may not trigger Struggle correctly

---

## ParseEnemyAction (line 5781)

### Wild Pokemon Move Selection
- Random move from moveset (random & 3)
- Skip if: no PP, disabled, or empty slot
- Retry until valid move found; if none, Struggle

### Trainer Move Selection
- Handled externally by `AI_SwitchOrTryItem` (not in core.asm)
- Move index passed via `wCurEnemyMoveNum`

### State Resets After Move Selection
- Fury Cutter count: reset unless current move is EFFECT_FURY_CUTTER
- Rage: clear SUBSTATUS_RAGE and counter unless current move is EFFECT_RAGE
- Protect count: reset unless current move is EFFECT_PROTECT or EFFECT_ENDURE

---

## NewBattleMonStatus / NewEnemyMonStatus

### Cleared on switch (line 4078 / 3596)
- All 5 substatus bytes (SubStatus1-5)
- Counter/tracking variables: DisableCount, FuryCutterCount, ProtectCount, RageCounter, DisabledMove, Minimized, WrapCount (both sides), TurnsTaken
- Used moves list (player only)
- LastPlayerCounterMove, LastEnemyCounterMove, LastPlayerMove/LastEnemyMove
- Opponent's SUBSTATUS_CANT_RUN cleared (Mean Look/Spider Web trap)

---

## Flee Mechanics Detail (line 3676)

### Blocked escapes
- `BATTLETYPE_TRAP`, `BATTLETYPE_CELEBI`, `BATTLETYPE_FORCESHINY`, `BATTLETYPE_SUICUNE`: can't flee
- Trainer battles: "There's no escape from a trainer battle!"
- `SUBSTATUS_CANT_RUN` (Mean Look): can't escape
- `wPlayerWrapCount > 0` (trapped by Wrap/Bind): can't escape

### Guaranteed escapes
- `BATTLETYPE_DEBUG`, `BATTLETYPE_CONTEST`: always escape
- Link battles: always escape (forfeit)
- Holding `HELD_ESCAPE` item (Smoke Ball): always escape, prints item message

### Speed-based escape
- If player speed >= enemy speed: always escape
- Otherwise: `F = (playerSpeed * 32 / (enemySpeed / 4)) + 30 * (attempts - 1)`
- If quotient > 255: escape
- Random roll: if F >= random(0-255), escape
- Each failed attempt adds 30 to the formula

---

## Link Battle RNG (line 6880)

### _BattleRandom
- Non-link battles: use normal `Random` function
- Link battles: use shared PRNG with 10 seeds
- PRNG formula: `a[n+1] = (a[n] * 5 + 1) % 256`
- Seeds stored in `wLinkBattleRNs` (10 bytes)
- After consuming 9 values, all 10 seeds are advanced and counter resets
- Ensures both Game Boys produce identical random sequences

---

## Roaming Pokemon Handling (line 8602)

### BattleEnd_HandleRoamMons
- If roaming battle won: clear HP, map group/number, species (permanently removed)
- If roaming battle fled/lost: save current HP (lo byte only) back to roam struct
- Non-roaming battles: 1/16 chance to call `UpdateRoamMons` (move them to new locations)

---

## HP Fraction Helpers

All functions clamp minimum to 1:

| Function | Fraction | Implementation |
|----------|----------|----------------|
| GetSixteenthMaxHP | 1/16 | GetQuarterMaxHP >> 2 |
| GetEighthMaxHP | 1/8 | GetQuarterMaxHP >> 1 |
| GetQuarterMaxHP | 1/4 | GetMaxHP >> 2 |
| GetHalfMaxHP | 1/2 | GetMaxHP >> 1 |

`GetMaxHP` reads from `wBattleMonMaxHP` or `wEnemyMonMaxHP` based on `hBattleTurn`.

---

## Held Item Between-Turn Effects

### HandleHealingItems (line 4245)
Order: Player then Enemy (or reversed for link clock)
For each side:
1. **HP healing** (`HandleHPHealingItem`): Berry/Gold Berry — triggers only if HP < 50% max. Heals `c` HP (item's parameter), caps at max HP, consumes item.
2. **Status healing** (`UseHeldStatusHealingItem`): Checks `HeldStatusHealingEffects` table. If held item matches a status, clears status, toxic substatus, nightmare substatus. If `HELD_HEAL_STATUS` (MiracleBerry): also clears confusion. Recalculates stats (PRZ/BRN effects). Consumes item.
3. **Confusion healing** (`UseConfusionHealingItem`): Bitter Berry or MiracleBerry. Clears confusion substatus, consumes item.

### HandleStatBoostingHeldItems (line 4476)
- Checks `HeldStatUpItems` table (unused in base game — no items have these effects)
- If stat boost succeeds: consumes item, prints activation message

---

## Weather Between-Turn Implementation (line 1685)

### HandleWeather
- Decrement `wWeatherCount`; if reaches 0, clear weather and print end message
- If still active: print continuation message
- Sandstorm damage: applied to each side independently
  - Skip if Underground (Dig)
  - Skip if either type is Rock, Ground, or Steel
  - Damage: `GetEighthMaxHP` -> `SubtractHPFromUser`

---

## HandleScreens (line 1620)

### Light Screen / Reflect Countdown
- Each screen tracked by separate counter (`wPlayerLightScreenCount`/`wPlayerReflectCount` and enemy equivalents)
- Each tick: decrement counter; if reaches 0, clear the screen bit and print message
- "Your/Enemy Light Screen fell!" / "Your/Enemy Reflect faded!"

---

## Known Bugs from core.asm

1. **Pursuit faint status bug** (line 4195): Pokemon KO'd by Pursuit retains its old status when revived (status not cleared before faint)
2. **Berserk Gene 256-turn confusion** (line 348): `dec [hl]` on a confusion counter of 0 wraps to 255, giving 256 turns of confusion
3. **PRZ/BRN stat reduction not applied on switch** (line 6413): `LoadEnemyMon` returns before applying status stat effects to switched trainer mons
4. **GlacierBadge Sp.Def boost unreliable** (line 6806): Sp.Defense boost depends on carry from Sp.Attack cap check, not the badge bit
5. **Magikarp length unit conversion** (line 6169): Mm-based size filters applied to feet/inch values
6. **Lake of Rage Magikarp size inversion** (line 6198): Filter logic makes them shorter instead of longer
7. **Low-HP switch text freeze** (line 7636): Division by zero if enemy max HP < 4
8. **Disabled PP Up Struggle bug** (line 5769): PP check uses AND with PP_MASK incorrectly for disabled moves with PP Ups
9. **Unown forced-shiny infinite loop** (line 6152): Shiny DVs always produce the same Unown letter; if that letter is locked, re-roll loops forever
