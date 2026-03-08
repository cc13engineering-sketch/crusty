# Pokemon Crystal -- Overworld Engine Reference

Source: pokecrystal engine/overworld/ (24 files, ~15,300 lines)

---

## Main Loop (events.asm)

### OverworldLoop
Three-state machine driven by wMapStatus:
1. **MAPSTATUS_START** (StartMap) -- Clear state, init phone delay, run map setup
2. **MAPSTATUS_ENTER** (EnterMap) -- Run map setup script, set 5-step wild encounter cooldown
3. **MAPSTATUS_HANDLE** (HandleMap) -- Main per-frame processing:
   - ResetOverworldDelay (2 frames max)
   - HandleMapTimeAndJoypad (UpdateTime, GetJoypad, TimeOfDayPals)
   - HandleCmdQueue (queued commands)
   - MapEvents (PlayerEvents + ScriptEvents)
   - HandleMapObjects (NPC step + player step + object visibility)
   - NextOverworldFrame (delay remaining)
   - HandleMapBackground (sprites, scrolling, map name sign)
   - CheckPlayerState (enable/disable events based on step flags)

### Player Event Priority (PlayerEvents)
Checked in order; first match wins:
1. **CheckTrainerEvent** -- Trainer line of sight
2. **CheckTileEvent** -- Warps, connections, coord events, step counting, wild encounters
3. **RunMemScript** -- Queued reentry scripts
4. **RunSceneScript** -- Scene-based scripts (wCurMapSceneScriptCount)
5. **CheckTimeEvents** -- Bug Contest timer, daily reset, Pokerus tick, phone calls
6. **OWPlayerInput** -- A-button (talk/interact), Start (menu), Select (registered item)

### Player Event Types (PLAYEREVENT_*)
| Value | Constant | Description |
|-------|----------|-------------|
| 0 | PLAYEREVENT_NONE | No event |
| 1 | PLAYEREVENT_SEENBYTRAINER | Spotted by trainer |
| 2 | PLAYEREVENT_TALKTOTRAINER | Talked to trainer |
| 3 | PLAYEREVENT_ITEMBALL | Found item ball |
| 4 | PLAYEREVENT_CONNECTION | Map connection |
| 5 | PLAYEREVENT_WARP | Warp tile |
| 6 | PLAYEREVENT_FALL | Pit/fall hole |
| 7 | PLAYEREVENT_WHITEOUT | White out |
| 8 | PLAYEREVENT_HATCH | Egg hatching |
| 9 | PLAYEREVENT_JOYCHANGEFACING | Direction change |

### Enabled Player Events (wEnabledPlayerEvents)
Bit flags controlling which events are active:
- Bit 0: PLAYEREVENTS_WARPS_AND_CONNECTIONS
- Bit 1: PLAYEREVENTS_COORD_EVENTS
- Bit 2: PLAYEREVENTS_COUNT_STEPS
- Bit 3: PLAYEREVENTS_WILD_ENCOUNTERS
- Bit 5: PLAYEREVENTS_UNUSED

---

## Player Movement (player_movement.asm)

### DoPlayerMovement
Main entry point. Flow:
1. GetDPad -- Read joypad, apply downhill forcing
2. TranslateIntoMovement -- Based on player state:

**Player States:**
| State | Behavior |
|-------|----------|
| PLAYER_NORMAL | Standard walking |
| PLAYER_BIKE | Fast movement (STEP_BIKE) |
| PLAYER_SURF | Water movement |
| PLAYER_SURF_PIKA | Surfing Pikachu variant |
| PLAYER_SKATE | Ice sliding |

**Movement Pipeline (Normal/Bike):**
1. CheckForced -- On ice, forces same direction
2. GetAction -- Maps D-pad to walking direction, facing, delta X/Y, collision tile
3. CheckTile -- Forced movement tiles (waterfalls, walk tiles, warps override input)
4. CheckTurning -- If facing != walking direction, turn first
5. TryStep -- Check land permissions, NPC collision, ice, bike speed
6. TryJump -- Ledge hopping (HI_NYBBLE_LEDGES tiles)
7. CheckWarp -- Edge warp detection

**Step Types:**
| Constant | Speed |
|----------|-------|
| STEP_WALK | Normal (slow_step) |
| STEP_BIKE | Fast (big_step) |
| STEP_ICE | Slide (fast_slide_step) |
| STEP_LEDGE | Jump (jump_step) |
| STEP_TURN | Turn in place |

**Movement Results (PLAYERMOVEMENT_*):**
- PLAYERMOVEMENT_NORMAL (0) -- No movement
- PLAYERMOVEMENT_WARP (1) -- Entered warp
- PLAYERMOVEMENT_TURN (2) -- Changed facing
- PLAYERMOVEMENT_FORCE_TURN (3) -- Forced by tile
- PLAYERMOVEMENT_FINISH (4) -- Step started
- PLAYERMOVEMENT_CONTINUE (5) -- Continue current
- PLAYERMOVEMENT_EXIT_WATER (6) -- Leaving water
- PLAYERMOVEMENT_JUMP (7) -- Ledge hop

### Ledge System
Collision tiles $A0-$A7 (HI_NYBBLE_LEDGES). Each tile encodes which facing directions allow jumping. Plays SFX_JUMP_OVER_LEDGE. The low 3 bits index into a mask table checking if the player's facing matches.

### Collision Checks
- **CheckLandPerms**: Tile permissions (wTilePermissions) AND facing direction, plus walkability check
- **CheckSurfPerms**: Returns 0 for water, 1 for land (exit water), carry for blocked
- **CheckNPC**: IsNPCAtCoord checks all 16 object structs for collision at target coordinates
- **CheckStrengthBoulder**: If Strength active and object has STRENGTH_BOULDER_F, set BOULDER_MOVING_F

### Ice Sliding
On PLAYER_SKATE state or on ice tiles (CheckIceTile):
- Input is locked to current direction (CheckForced)
- Movement continues until hitting wall or non-ice tile
- If hitting wall on ice, no bump sound (HitWall)
- CheckStandingOnIce checks wPlayerTileCollision and wPlayerState

---

## NPC Movement (npc_movement.asm)

### CanObjectMoveInDirection
Per-NPC movement validation:
1. Swimming NPCs: Check WillObjectBumpIntoLand (BUG: no radius check)
2. Normal NPCs: Check WillObjectBumpIntoWater
3. Unless NOCLIP_TILES_F set
4. Check WillObjectBumpIntoSomeoneElse (unless NOCLIP_OBJS_F)
5. Check HasObjectReachedMovementLimit (unless MOVE_ANYWHERE_F)
6. Check IsObjectMovingOffEdgeOfScreen

### Movement Radius
OBJECT_RADIUS stores X radius in low nybble, Y radius in high nybble. Object cannot move beyond OBJECT_INIT_X/Y +/- radius.

### NPC Flags (OBJECT_FLAGS1)
- NOCLIP_TILES_F: Ignore tile collision
- NOCLIP_OBJS_F: Ignore NPC collision
- MOVE_ANYWHERE_F: Ignore radius and screen bounds
- EMOTE_OBJECT_F: Is an emote bubble, not a real NPC

### Counter Tiles
CheckFacingObject doubles the interaction distance for COLL_COUNTER tiles, allowing talking across shop counters.

---

## Wild Encounters (wildmons.asm)

### Encounter Flow
1. **TryWildEncounter** -- Get encounter rate, apply modifiers, random check
2. **ChooseWildEncounter** -- Pick species from probability table
3. **CheckRepelEffect** -- Compare wild level to lead Pokemon level

### Encounter Rate Calculation
```
Base rate = wMornEncounterRate / wDayEncounterRate / wNiteEncounterRate (time-dependent)
           or wWaterEncounterRate (if surfing)
Then apply:
  - Pokemon March radio: double rate
  - Ruins of Alph radio: double rate
  - Pokemon Lullaby radio: halve rate
  - Cleanse Tag on any party member: halve rate
Finally: Random() < modified_rate triggers encounter
```

### Wild Mon Probability Tables
**Grass (7 slots per time of day):**
| Slot | Cumulative % | Individual % |
|------|-------------|--------------|
| 0 | 30% | 30% |
| 1 | 60% | 30% |
| 2 | 80% | 20% |
| 3 | 90% | 10% |
| 4 | 95% | 5% |
| 5 | 99% | 4% |
| 6 | 100% | 1% |

**Water (3 slots):**
| Slot | Cumulative % | Individual % |
|------|-------------|--------------|
| 0 | 60% | 60% |
| 1 | 90% | 30% |
| 2 | 100% | 10% |

### Surf Level Variation
When surfing, wild Pokemon levels get random bonuses:
- 35% chance: +0
- 30% chance: +1
- 20% chance: +2
- 10% chance: +3
- 5% chance: +4

### Wild Data Sources
Checked in order:
1. SwarmGrassWildMons / SwarmWaterWildMons (if swarm active)
2. JohtoGrassWildMons / KantoGrassWildMons (based on IsInJohto)
3. JohtoWaterWildMons / KantoWaterWildMons

### Roaming Pokemon (Raikou/Entei)
- Initialized at level 40 (Raikou on Route 42, Entei on Route 37)
- ~29.3% chance per grass step to check: 25/64 base * 3/4 not-zero = 75/256
- 50/50 Raikou vs Entei
- Must be on same map as player
- UpdateRoamMons: Each map change, beasts move to adjacent route
  - 1/8n chance per move to jump to random map
  - Won't return to player's last map (backtracking avoidance)
- RoamMaps table: 16 entries with adjacency lists
- HP preserved between encounters (wRoamMon1HP)

### Wild Encounter Cooldown
- 5-step cooldown on map entry (SetUpFiveStepWildEncounterCooldown)
- No encounters in first 5 steps after entering a map
- Caves and dungeons don't need grass -- any non-ice tile works
- CheckGrassCollision required for outdoor encounters

### Repel
- wRepelEffect counts down each step
- If active: wild encounter only triggers if wild level >= lead Pokemon level
- Checks first non-fainted Pokemon's level

### Bug Catching Contest
- 40% encounter rate in super tall grass, 20% in normal grass
- Uses ContestMons table with species, min level, max level, probability
- Random level between min and max for each encounter

---

## Warp System (warp_connection.asm)

### Map Connections
EnterMapConnection handles the 4 cardinal connections:
- Updates wMapGroup/wMapNumber to connected map
- Calculates new wXCoord/wYCoord from connection strip offsets
- Computes wOverworldMapAnchor for the block map

### Warp Processing
EnterMapWarp:
1. SaveDigWarp -- If going from outdoor to indoor, save for Dig/Escape Rope
   - Exception: MOUNT_MOON_SQUARE and TIN_TOWER_ROOF (outdoor maps inside indoor groups)
2. SetSpawn -- If entering Pokemon Center (TILESET_POKECENTER or TILESET_POKECOM_CENTER), set respawn point
3. Copy wNextWarp/wNextMapGroup/wNextMapNumber to active map state

### Edge Warps
Directional warp carpets ($70-$73) trigger when walking in the matching direction:
- COLL_WARP_CARPET_DOWN: Walking right
- COLL_WARP_CARPET_UP: Walking left
- COLL_WARP_CARPET_LEFT: Walking up
- COLL_WARP_CARPET_RIGHT: Walking down

### Warp SFX
- Doors: SFX_ENTER_DOOR
- Warp panels: SFX_WARP_TO
- Everything else: SFX_EXIT_BUILDING

---

## Tile Events (tile_events.asm)

### Warp Collision Detection
CheckWarpCollision: Pit tiles ($60, $68) and HI_NYBBLE_WARPS ($70+) are warps.

### Grass Collision (wild encounter eligibility)
Tiles that count as grass: COLL_CUT_08, COLL_TALL_GRASS, COLL_LONG_GRASS, COLL_CUT_28, COLL_WATER, COLL_GRASS_48 through COLL_GRASS_4C.

### A-Button Tile Events (TryTileCollisionEvent)
Checked in order when pressing A facing a tile:
1. CheckFacingTileForStdScript (standard tile scripts)
2. Cut tree
3. Whirlpool
4. Waterfall
5. Headbutt tree
6. Surf

---

## Map Setup (map_setup.asm)

### RunMapSetupScript
Uses hMapEntryMethod to select a setup script from MapSetupScripts table. Each setup script is a list of command indices into MapSetupCommands.

### Map Entry Methods (MAPSETUP_*)
- MAPSETUP_CONNECTION: Map connection crossing
- MAPSETUP_DOOR: Standard warp
- MAPSETUP_FALL: Falling through pit
- MAPSETUP_RELOADMAP: Reload (resets poison counter)
- Others for fly, warp, etc.

### Key Setup Operations
- LoadMapGraphics: Tileset, tile GFX, sprites, fonts
- LoadMapPalettes: SGB/CGB palette setup
- RefreshMapSprites: Clear sprites, init map name sign, get movement permissions
- HandleNewMap: Clear buffers, reset bike flags, run MAPCALLBACK_NEWMAP
- HandleContinueMap: Clear command queue, run MAPCALLBACK_CMDQUEUE, set time of day

### Bike State Management
- CheckForcedBiking: If BIKEFLAGS_ALWAYS_ON_BIKE_F, force PLAYER_BIKE
- CheckSurfing: If on water tile, force PLAYER_SURF
- ResetSurfingOrBikingState: Reset to PLAYER_NORMAL when entering indoor/dungeon

---

## Step Counting (events.asm CountStep)

Every step triggers:
1. **CheckSpecialPhoneCall** -- Priority phone calls
2. **DoRepelStep** -- Decrement and check Repel counter
3. **wPoisonStepCount++** -- Poison step tracking
4. **wStepCount++** -- Total step counter
5. **Every 256 steps**: StepHappiness (increase party happiness)
6. **At step 128**: DoEggStep (decrement egg hatch counter)
7. **DayCareStep**: +1 EXP to DayCare Pokemon
8. **Every 4 poison steps**: DoPoisonStep (damage poisoned Pokemon)
9. **DoBikeStep**: Track bike steps for bike shop call (1024 steps triggers)

---

## Sprite/Object System (overworld.asm)

### Sprite Loading
- AddMapSprites: Outdoor maps use OutdoorSprites table (per map group); indoor maps enumerate objects
- SPRITE_GFX_LIST_CAPACITY entries in wUsedSprites
- Sprites sorted by type (bubble sort), arranged in VRAM tiles
- Two tile tables: tiles $00-$7F in VRAM bank 1, tiles $80+ in bank 0

### Sprite Types
- WALKING_SPRITE: 12 tiles (4 directions x 3 frames)
- STANDING_SPRITE: 12 tiles
- STILL_SPRITE: 4 tiles (single direction)

### Player Sprite
GetPlayerSprite checks wPlayerGender and wPlayerState to select from ChrisStateSprites or KrisStateSprites table.

---

## Object Event Handling (events.asm)

### A-Button Object Types
| Type | Handler |
|------|---------|
| OBJECTTYPE_SCRIPT | Run script at MAPOBJECT_SCRIPT_POINTER |
| OBJECTTYPE_ITEMBALL | Load item data, trigger PLAYEREVENT_ITEMBALL |
| OBJECTTYPE_TRAINER | TalkToTrainer, trigger PLAYEREVENT_TALKTOTRAINER |
| OBJECTTYPE_3-6 | Dummy (do nothing) |

### BG Event Types
| Type | Trigger |
|------|---------|
| BGEVENT_READ | Any facing |
| BGEVENT_UP/DOWN/LEFT/RIGHT | Must face that direction |
| BGEVENT_IFSET | Run if flag is set |
| BGEVENT_IFNOTSET | Run if flag is not set |
| BGEVENT_ITEM | Hidden item (checks flag) |
| BGEVENT_COPY | Copy data (checks flag) |

---

## Movement Commands (movement.asm)

90 movement commands ($00-$59) including:
- $00-$03: turn_head (4 directions)
- $04-$07: turn_step (4 directions)
- $08-$0B: slow_step (4 directions)
- $0C-$0F: step (normal speed, 4 directions)
- $10-$13: big_step (bike speed, 4 directions)
- $14-$1F: slide steps (3 speeds x 4 directions)
- $20-$27: turn_away/turn_in
- $28-$2B: waterfall turns
- $2C-$37: jump steps (3 speeds x 4 directions)
- $38-$39: remove/set sliding
- $3A-$3B: remove/fix facing
- $3C-$3D: show/hide object
- $3E-$46: sleep (various durations)
- $47: step_end
- $48: step_wait_end
- $49: remove_object
- $4A: step_loop
- $4B: stop
- $4C-$4D: teleport_from/to
- $4E: skyfall
- $4F: step_dig
- $50: step_bump
- $51-$52: fish animations
- $53-$54: hide/show emote
- $55: step_shake
- $56: tree_shake
- $57: rock_smash
- $58: return_dig
- $59: skyfall_top

---

## Known Bugs

1. **No bump noise on tile $3E**: Standing direction indexes into edge warp table incorrectly
2. **Swimming NPCs ignore radius**: CanObjectMoveInDirection skips HasObjectReachedMovementLimit for swimming NPCs
3. **ChooseWildEncounter validates level as species**: ValidateTempWildMonSpecies is called with level in register a instead of species
4. **RandomUnseenWildMon always picks morning species**: Adds time-of-day offset after selecting slot 5+, but base pointer already includes the offset
5. **LoadSpriteGFX overflow**: Doesn't limit UsedSprites capacity
6. **TryObjectEvent ACE**: Object type used as index into jump table can go out of bounds
