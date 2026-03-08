# Pokemon Crystal -- Phone System Reference

Source: pokecrystal engine/phone/ (47 files, ~5,700 lines)

---

## Phone System Architecture (phone.asm)

### Contact List
- wPhoneList: Array of CONTACT_LIST_SIZE contact IDs
- Each contact is an index into PhoneContacts table
- Permanent numbers (Elm, Mom, etc.) don't count toward player limit

### Phone Contact Struct (PHONE_CONTACT_SIZE bytes)
| Field | Description |
|-------|-------------|
| PHONE_CONTACT_TRAINER_CLASS | Trainer class (0 for non-trainer) |
| PHONE_CONTACT_TRAINER_ID | Trainer ID within class |
| PHONE_CONTACT_SCRIPT1_BANK | Bank for player-initiated call script |
| PHONE_CONTACT_SCRIPT1_ADDR | Address for player-initiated call script |
| PHONE_CONTACT_SCRIPT1_TIME | Time-of-day mask for player calls |
| PHONE_CONTACT_SCRIPT2_BANK | Bank for NPC-initiated call script |
| PHONE_CONTACT_SCRIPT2_ADDR | Address for NPC-initiated call script |
| PHONE_CONTACT_SCRIPT2_TIME | Time-of-day mask for incoming calls |
| PHONE_CONTACT_MAP_GROUP | Map group where contact resides |
| PHONE_CONTACT_MAP_NUMBER | Map number where contact resides |

### Core Operations

**AddPhoneNumber**: Check if already exists, find open slot, add contact ID.

**DelCellNum**: Find contact in list, zero it out.

**_CheckCellNum**: Linear scan of wPhoneList for contact ID.

**GetRemainingSpaceInPhoneList**: Count permanent numbers already registered, subtract from CONTACT_LIST_SIZE.

---

## Incoming Calls (CheckPhoneCall)

### Call Trigger Conditions
All must be true:
1. Player not standing on entrance tile (CheckStandingOnEntrance)
2. Receive call timer has elapsed (CheckReceiveCallTimer)
3. 50% random chance (Random AND $7F check)
4. Map has phone service (GetMapPhoneService == 0)
5. At least one available caller (GetAvailableCallers > 0)

### Available Caller Selection
GetAvailableCallers filters registered contacts:
- Contact must be in wPhoneList (non-zero)
- Contact's SCRIPT2_TIME must match current time of day
- Contact must be on a different map than the player

ChooseRandomCaller: Random selection from available callers (Random swap+mask mod count).

### Call Timing
- wTimeCyclesSinceLastCall tracks how many call cycles have passed
- Delay between calls decreases with each cycle:
  - Cycle 0: 20 minutes
  - Cycle 1: 10 minutes
  - Cycle 2: 5 minutes
  - Cycle 3+: 3 minutes
- Timer is reset after each call (InitCallReceiveDelay)

### Call Presentation
RingTwice_StartCall:
1. Play SFX_CALL
2. Show phone textbox with caller name (2 ring cycles)
3. Run caller's Script2 (NPC-initiated script)
4. waitbutton
5. HangUp (SFX_HANG_UP + boop animation)
6. Reset call delay

---

## Outgoing Calls (MakePhoneCallFromPokegear)

### Call Validation
1. Not in link mode (wLinkMode == 0)
2. Map has phone service
3. Contact's SCRIPT1_TIME matches current time of day
4. If on same map as contact: "Just go talk to them!" script

### Call Execution
If valid: Load contact's Script1 (player-initiated script), execute via callback.

If out of area: Run PhoneOutOfAreaScript ("You're out of the service area").

---

## Special Phone Calls

### Special Call System
SpecialPhoneCallList indexed by wSpecialPhoneCallID:
- Each entry has a condition function (outside-only vs anywhere)
- Custom script bank/address for the call content

### Known Special Calls
| ID | Constant | Description |
|----|----------|-------------|
| 1 | SPECIALCALL_BIKESHOP | Bike shop owner after 1024 bike steps |
| - | PHONE_BILL | Bill's PC box full notification |
| - | PHONE_ELM | Prof. Elm story calls |

### Bike Shop Call
Triggered in DoBikeStep when:
- STATUSFLAGS2_BIKE_SHOP_CALL_F is set
- Player is on bike
- Map has phone service
- wBikeStep reaches 1024
- No other special call queued

---

## Phone Script System (scripts/)

### Script Structure
Each contact has two scripts:
1. **CallerScript** (incoming call / NPC-initiated)
2. **CalleeScript** (outgoing call / player-initiated)

### Content Generation
Calls use randomized content from shared templates:

**Generic Call Flow (Phone_GenericCall_Male/Female):**
1. RandomPhoneMon (get random Pokemon from caller's party)
2. 50% branch: Bragging call vs Generic call
3. RandomPhoneWildMon (get random wild Pokemon from caller's route)
4. 50% branch: Defeated mon vs Got away
5. Hang up text

**Call Content Types:**
- **Bragging**: Caller talks about their Pokemon (per-caller unique text)
- **Defeated mon**: Caller beat a wild Pokemon (per-caller text)
- **Lost a mon**: Wild Pokemon got away (per-caller text)
- **Rematch request**: Caller wants to battle again + landmark name
- **Found item**: Caller found an item for the player
- **Bug contest**: Caller discusses upcoming contest
- **Rare mon sighting**: RandomUnseenWildMon -- picks rare Pokemon from caller's route
- **Swarm alert**: Dunsparce/Yanma swarm notification

### Rematch System
Per-caller ENGINE flags control rematch availability:
- ENGINE_{NAME}_READY_FOR_REMATCH: Set when caller wants to battle
- ENGINE_{NAME}_{DAY}_{TIME}: Time-specific event flags
- Daily rematch flags reset in CheckDailyResetTimer

Example (Joey):
```
JoeyPhoneCallerScript:
  - Greet
  - If ready for rematch: generic call
  - If Monday afternoon flag set: generic call
  - 2/3 chance: JoeyWantsBattle (set rematch flag, give landmark)
  - 1/3 chance: Phone_GenericCall_Male
```

### Phone Contacts (45 scripts)
**Trainers (25 contacts):**
- Male: Joey, Wade, Ralph, Anthony, Todd, Irwin, Arnie, Alan, Chad, Derek, Tully, Brent, Vance, Wilton, Parry, Jack, Gaven, Jose, Huey, Kenji
- Female: Beverly, Beth, Reena, Liz, Gina, Dana, Tiffany, Erin

**Non-Trainer Contacts:**
- Mom, Prof. Elm, Prof. Oak, Bill, Buena, Bike Shop

### Item Gift System
Some callers find items for the player:
- Male: Jose, Wade, Alan, Derek, Tully, Wilton
- Female: Beverly, Gina, Dana, Tiffany
- Daily phone item flags (wDailyPhoneItemFlags) track per-caller gifts

### Gossip System
Some callers have gossip scripts (separate files):
- Brent, Chad, Irwin, Jack, Liz

---

## Non-Trainer Caller Names
Special indexed name table for Mom, Elm, Oak, Bill, Buena, Bike Shop displayed in phone UI instead of trainer class/name format.

---

## Known Bugs

1. **BrokenPlaceFarString**: The PhoneCall routine (unused `phonecall` script command) calls BrokenPlaceFarString which is not in bank 0 -- will crash if called
2. **Wrong number on empty slot**: If wPhoneList has a zero entry selected, displays "Wrong number" WrongNumber script

---

## Key RAM Variables

| Variable | Description |
|----------|-------------|
| wPhoneList | Contact ID array (CONTACT_LIST_SIZE entries) |
| wCurCaller | Current caller contact ID |
| wCallerContact | Loaded PHONE_CONTACT_SIZE struct for active call |
| wSpecialPhoneCallID | Queued special call (0 = none) |
| wTimeCyclesSinceLastCall | Call delay cycle counter (0-3) |
| wReceiveCallDelay_MinsRemaining | Minutes until next call eligible |
| wNumAvailableCallers | Count of eligible callers |
| wAvailableCallers | Array of eligible caller IDs |
| wPhoneScriptBank | Bank for current phone script |
| wPhoneCaller | Address for current phone script |
