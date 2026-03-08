# Pokemon Crystal -- Core Engine Routines (home/)

Source: pokecrystal home/ directory (57 files, ~12,654 lines)

These routines live in ROM bank 0 and are callable from any bank.

---

## Random Number Generation (home/random.asm)

### Random
Hardware-based RNG using the DIV register (increments at 16384 Hz).
- Adds DIV to hRandomAdd, subtracts DIV from hRandomSub
- Called from VBlank as well (two entropy sources)
- Returns value in `a` (the hRandomSub value)
- NOT suitable for link battles (would desync)

### BattleRandom
Deterministic PRNG for battle engine.
- Lives in a separate bank, called via farcall
- Uses a 10-byte PRNG stream (wLinkBattleRNCount tracks position)
- Shared seed ensures link battles stay in sync
- Returns random byte in `a`

### RandomRange
Returns random number in range [0, a).
- Uses rejection sampling to avoid modulo bias
- Calls Random internally (not BattleRandom)

---

## Math Routines (home/math.asm)

### SimpleMultiply
`a = a * c` (8-bit multiplication via repeated addition)

### SimpleDivide
`b = a / c` (quotient), `a = a % c` (remainder)

### Multiply
`hProduct = hMultiplicand * hMultiplier` (3-byte x 1-byte = 4-byte result)
- All values big-endian
- Actual implementation in engine/math/math.asm (_Multiply)

### Divide
`hQuotient = hDividend / hDivisor` (up to 4-byte dividend, 1-byte divisor)
- BUG: Dividing by 0 enters infinite loop (see HP<4 switch bug)

---

## Text Engine (home/text.asm)

### PrintText
Main text rendering entry point.
- Processes text command bytes ($00-$16, $50)
- Handles inline control characters ($4E, $4F, $50, $51, $55, $57, $58)
- Supports RAM references, BCD numbers, far bank text

### TextCommands Table
Jump table of 23 text command handlers:
- $00: PlaceText (raw string until "@")
- $01: TextFromRAM (print from RAM address)
- $09: PrintNumber (decimal number display)
- $14: TextBuffer (indexed string buffer 0-6)
- $16: FarText (cross-bank text loading)

---

## Map Engine (home/map.asm)

### GetMapField
Retrieve a field from the map data structure.
- Fields: MAP_MAPGROUP, MAP_MAPNUMBER, MAP_BORDER_BLOCK, MAP_HEIGHT, MAP_WIDTH, MAP_BLOCKDATA_BANK, MAP_BLOCKDATA_PTR, MAP_SCRIPT_BANK, MAP_SCRIPT_PTR, MAP_EVENT_BANK, MAP_EVENT_PTR, MAP_CONNECTIONS, MAP_MUSIC, MAP_PALETTE, MAP_PHONE, MAP_FISHING, MAP_TILESET, MAP_ENVIRONMENT, MAP_LOCATION

### GetMapMusic
Gets music for current map. Special handling:
- MUSIC_MAHOGANY_MART ($64): Plays ROCKET_HIDEOUT or CHERRYGROVE based on story flags
- RADIO_TOWER_MUSIC ($80+): Plays ROCKET_OVERTURE or GOLDENROD based on story flags

### LoadMetatiles
Loads 4x4 tile metatiles from tileset data.
- BUG: Wraps around past 128 blocks (add a uses signed arithmetic)

### ReadObjectEvents
Loads NPC/object data from map events.
- BUG: Overflows into wObjectMasks for maps with many objects

---

## Joypad (home/joypad.asm)

### GetJoypad
Reads hardware joypad register and updates:
- hJoyPressed: Buttons newly pressed this frame
- hJoyDown: Buttons currently held
- hJoyReleased: Buttons released this frame

### Button Constants
- PAD_RIGHT=$10, PAD_LEFT=$20, PAD_UP=$40, PAD_DOWN=$80
- A_BUTTON=$01, B_BUTTON=$02, SELECT=$04, START=$08

---

## Copy Routines (home/copy.asm)

### CopyBytes
`memcpy(de, hl, bc)` -- Copy bc bytes from hl to de

### ByteFill
`memset(hl, a, bc)` -- Fill bc bytes at hl with value a

### FarCopyBytes
Copy bytes from another ROM bank (bank in `a`)

---

## Pokemon Utilities (home/pokemon.asm)

### GetBaseData
Load base stats for species in wCurSpecies.
- Reads from BaseData table in bank $10
- Fills wBaseStats, wBaseType1/2, wBaseCatchRate, wBaseExp, etc.

### GetMoveName
Get name of move at wNamedObjectIndex.

### GetItemName
Get name of item at wNamedObjectIndex.

---

## Battle Variables (home/battle_vars.asm)

### GetBattleVar / GetBattleVarAddr
Access battle variables by index (BATTLE_VARS_SUBSTATUS1 through BATTLE_VARS_LAST_MOVE_OPP).
- Automatically selects player or enemy based on hBattleTurn
- GetBattleVar returns value in `a`
- GetBattleVarAddr returns address in `hl`

---

## Flag Operations (home/flag.asm)

### CheckFlag / SetFlag / ResetFlag
Operate on bit arrays (wEventFlags, engine flags, etc.)
- Register `b` = SET_FLAG or RESET_FLAG or CHECK_FLAG
- `de` = flag index
- Uses SmallFarFlagAction for flags in other banks

---

## Time System (home/time.asm)

### UpdateTime
Reads RTC (Real Time Clock) hardware and updates:
- wCurDay, wCurHour, wCurMinute, wCurSecond
- wTimeOfDay: 0=morning (4-9), 1=day (10-17), 2=night (18-3)

---

## Item System (home/item.asm)

### CheckItem
Check if player has item in bag.

### ReceiveItem / TossItem
Add/remove items from appropriate pocket.

---

## Trainer System (home/trainers.asm)

### CheckTrainerFlag / SetTrainerFlag
Check/set the "already beaten" flag for trainers.
- Each trainer has a unique flag in wEventFlags

---

## Serial Communication (home/serial.asm)

### Serial_ExchangeBytes
Exchange data over link cable.
- Handles clock master/slave synchronization
- Used for trading, battling, Mystery Gift

---

## Map Object System (home/map_objects.asm)

### GetMapObject
Get pointer to map object struct by index.

### CheckObjectVisibility
Check if object should be visible (respects event flags).

### SpawnObject / DespawnObject
Create/remove objects from the active object list.

---

## Sine/Cosine (home/sine.asm)

### Sine
`a = d * sin(e * pi/32)` -- Used for screen effects.
Lookup table with 32 entries covering 0 to pi.

---

## Key Routine Relationships

### Damage Calculation Flow
1. `BattleCommand_DamageStats` -- Get Atk/Def values
2. `BattleCommand_DamageCalc` -- Core formula: ((2*level/5+2) * power * atk/def) / 50 + 2
3. `BattleCommand_Stab` -- Apply STAB (1.5x) and type effectiveness
4. `BattleCommand_DamageVariation` -- Random 85-100%
5. Item boosts, weather, critical hits applied within these steps

### Map Loading Flow
1. `LoadMapData` -- Load map header, tileset, blockdata
2. `LoadMapAttributes` -- Load map scripts, events
3. `ReadObjectEvents` -- Spawn NPCs
4. Run MAPCALLBACK_NEWMAP callbacks
5. `GetMapMusic` + play music
6. Run scene scripts if applicable
