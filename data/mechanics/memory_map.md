# Pokemon Crystal -- Memory Map (RAM Layout)

Source: pokecrystal ram/ directory (wram.asm, hram.asm, sram.asm, vram.asm)

---

## Overview

The Game Boy Color has:
- **WRAM** (Work RAM): 8 banks x 4KB = 32KB. Bank 0 ($C000-$CFFF) is always mapped. Banks 1-7 ($D000-$DFFF) are switchable.
- **HRAM** (High RAM): 127 bytes ($FF80-$FFFE). Fast access via `ldh` instruction.
- **SRAM** (Save RAM): Battery-backed, stores save data.
- **VRAM** (Video RAM): 2 banks for tiles and tilemaps.

---

## WRAM Bank 0 ($C000-$CFFF) -- Always Accessible

### Stack ($C000-$C0FF)
- wStackBottom..wStackTop: 256-byte CPU stack

### Audio Engine ($C100-$C2B0 approx)
- wMusicPlaying: Nonzero if music active
- wChannel1..wChannel8: Per-channel audio state (channel_struct)
- wCurTrackDuty, wCurTrackVolumeEnvelope, wCurTrackFrequency
- wCurMusicByte, wCurChannel
- wVolume: Master volume (maps to rAUDVOL)
- wSoundOutput: Channel output routing (maps to rAUDTERM)
- wMusicID: Current music track ID
- wMusicBank: Bank of current music
- wLowHealthAlarm: Low HP beep control
- wMusicFade: Fade in/out control
- wCryPitch, wCryLength: Pokemon cry parameters
- wSFXPriority: If nonzero, SFX mutes music
- wMapMusic: Current map's music ID

### General State ($C2B0+)
- wLZAddress/wLZBank: LZ decompression pointers
- wInputType, wAutoInputAddress/Bank/Length: Auto-input (demo) system
- wDebugFlags, wGameLogicPaused, wSpriteUpdatesEnabled
- wMapTimeOfDay: Current time of day
- wLinkMode: Link cable mode (0=none, 1=TimeCapsule, 2=Colosseum, 3=Trade)
- wScriptVar: General-purpose script variable (used by all overworld scripts)
- wPlayerNextMovement, wPlayerMovement: Player movement state
- wMovementObject, wMovementDataBank/Address: NPC movement data

### Sprite Animations ($C300-$C3B0)
- wSpriteAnimDict: Maps sprite anim IDs to VRAM offsets
- wSpriteAnim1..wSpriteAnim10: Animation state structs

### Shadow OAM ($C400-$C49F)
- wShadowOAM: 40 sprites x 4 bytes each. Copied to OAM during VBlank.
  - Per sprite: Y position, X position, Tile number, Attributes

### Tilemap ($C4A0-$C5D7)
- wTilemap: 20x18 grid of 8x8 tile indices (360 bytes)

### Battle Data (UNION at $C600+)
This is a large union -- different battle-related data overlaps in memory.

#### Active Battle State
- wEnemyMoveStruct, wPlayerMoveStruct: Current move data (7 bytes each)
- wEnemyMonNickname, wBattleMonNickname: 11 bytes each
- wBattleMon: Player's active Pokemon (battle_struct)
- wWildMon: Nonzero if wild battle
- wEnemyTrainerItem1/Item2: AI items
- wEnemyTrainerBaseReward: Prize money base
- wEnemyTrainerAIFlags: 3 bytes of AI behavior flags
- wOTClassName: Trainer class name string

#### Battle Variables
- wCurOTMon: Enemy's current party index
- wBattleParticipantsNotFainted: Bit array of who battled
- wTypeModifier: Current effectiveness (>10 super, <10 not very)
- wCriticalHit: 0=no, 1=crit, 2=OHKO
- wAttackMissed: Nonzero = miss

#### Sub-Statuses (per side)
- wPlayerSubStatus1..5, wEnemySubStatus1..5: See game_constants.md

#### Turn Counters (per side)
- wPlayer/EnemyRolloutCount, ConfuseCount, ToxicCount
- wPlayer/EnemyDisableCount, EncoreCount, PerishCount
- wPlayer/EnemyFuryCutterCount, ProtectCount

#### Damage Tracking
- wPlayerDamageTaken, wEnemyDamageTaken: 2 bytes each (last damage received)
- wBattleReward: 3-byte prize money accumulator

#### Stats (per side)
- wPlayerStats/wEnemyStats: 5 stats x 2 bytes = 10 bytes each
- wPlayerStatLevels/wEnemyStatLevels: 7 stages (Atk,Def,Spd,SAtk,SDef,Acc,Eva) + 1 unused

#### Screen Effects
- wPlayerScreens/wEnemyScreens: Spikes, Safeguard, Light Screen, Reflect
- wPlayer/EnemySafeguardCount, LightScreenCount, ReflectCount

#### Weather
- wBattleWeather: Current weather (0-6)
- wWeatherCount: Turns remaining

#### Move Tracking
- wPlayerUsedMoves: Up to 4 moves used (in order)
- wLastPlayerMove, wLastEnemyMove: Last move used
- wLastPlayerCounterMove, wLastEnemyCounterMove: For Counter/Mirror Coat
- wPlayerFutureSightCount/Damage, wEnemyFutureSightCount/Damage
- wPlayerRageCounter, wEnemyRageCounter

#### Trapping
- wPlayerTrappingMove, wEnemyTrappingMove: Active trapping move
- wPlayerWrapCount, wEnemyWrapCount: Turns remaining

---

## WRAM Bank 1 ($D000-$DFFF) -- Persistent Game State

### Player Data
- wPlayerData: Player name, ID, money, badges, items
- wPlayerName: 8 bytes (7 chars + terminator)
- wPlayerID: 2 bytes
- wMoney: 3 bytes (BCD)
- wJohtoBadges, wKantoBadges: 1 byte each (8 bits = 8 badges)

### Party Pokemon
- wPartyCount: Number of Pokemon in party (0-6)
- wPartySpecies: 6 species bytes + terminator
- wPartyMon1..wPartyMon6: PARTYMON_STRUCT_LENGTH (48) bytes each
  - Species, Item, Moves (4), Move PP (4), Happiness
  - DVs (2 bytes), StatExp (5 x 2 bytes)
  - Level, Status, HP (current/max), Stats (Atk/Def/Spd/SAtk/SDef)
  - Caught data (time/level, gender/location)
- wPartyMonOTs: 6 x NAME_LENGTH
- wPartyMonNicknames: 6 x MON_NAME_LENGTH

### Pokedex
- wPokedexOwned: 32 bytes (256 bits, 1 per species)
- wPokedexSeen: 32 bytes

### Items
- wItems: Bag items (pocket)
- wKeyItems: Key items pocket
- wBalls: Poke Balls pocket
- wTMsHMs: TM/HM pocket
- wPCItems: PC storage items

### Map State
- wMapGroup, wMapNumber: Current map
- wPlayerDir: Player facing direction
- wXCoord, wYCoord: Player position on map

### Time
- wCurDay: Day of week (0-6)
- wCurHour, wCurMinute, wCurSecond: Current time
- wTimeOfDay: 0=morning, 1=day, 2=night

### Event Flags
- wEventFlags: Large bit array (~1400 flags) tracking all story events, items obtained, trainers beaten, etc.

### Phone
- wPhoneList: Phone contacts array

---

## HRAM ($FF80-$FFFE) -- High RAM

Fast-access variables for critical game engine state:

### RNG State
- hRandomAdd: Hardware RNG accumulator (add)
- hRandomSub: Hardware RNG accumulator (subtract)

### Math Registers
- hMultiplicand: 3-byte multiplicand
- hMultiplier: 1-byte multiplier
- hProduct: 4-byte product (overlaps with hDividend)
- hDivisor: 1-byte divisor
- hQuotient: 4-byte quotient

### Battle Turn
- hBattleTurn: 0=player, 1=enemy

### System
- hROMBank: Current ROM bank
- hVBlank: VBlank handler mode
- hJoyPressed, hJoyDown: Joypad state
- hInMenu: Menu mode flag

---

## SRAM -- Save Data

### Save Structure
- sOptions: Game options (text speed, battle style, etc.)
- sPlayerData: Complete player state snapshot
- sPartyMon: Party Pokemon data
- sPokemonBoxes: PC box storage (up to 14 boxes x 20 Pokemon)
- sBoxCount, sBoxSpecies, sBoxMons: Current box data
- sMysteryGiftData: Mystery Gift state
- sBattleTowerData: Battle Tower records

---

## VRAM -- Video RAM

### Bank 0
- vTiles0: Sprite tiles ($8000-$87FF)
- vTiles1: Shared tiles ($8800-$8FFF)
- vTiles2: BG tiles ($9000-$97FF)
- vBGMap0: BG tilemap ($9800-$9BFF, 32x32)
- vBGMap1: Window tilemap ($9C00-$9FFF, 32x32)

### Bank 1
- vTiles3-5: Additional tile data (CGB only)
- Attribute maps for color/priority

---

## Key RAM Relationships for Battle Simulation

To simulate a battle turn, the key variables are:
1. **wBattleMon / wEnemyMon**: Active Pokemon stats
2. **wPlayerSubStatus1-5 / wEnemySubStatus1-5**: All volatile effects
3. **wPlayerStatLevels / wEnemyStatLevels**: Stat stages (-6 to +6, stored as 1-13)
4. **wPlayerScreens / wEnemyScreens**: Field effects
5. **wBattleWeather / wWeatherCount**: Weather
6. **wPlayerDamageTaken / wEnemyDamageTaken**: For Counter/Mirror Coat
7. **wCurPlayerMove / wCurEnemyMove**: Selected moves
8. **Turn counters**: Confusion, toxic, disable, encore, perish, wrap, etc.
