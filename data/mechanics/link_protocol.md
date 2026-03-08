# Pokemon Crystal -- Link Protocol & Communication Details

Source: pokecrystal disassembly (engine/link/*.asm)

---

## Serial Communication Protocol

### Connection Modes (wLinkMode)

| Mode | Constant | Description |
|------|----------|-------------|
| Time Capsule | LINK_TIMECAPSULE | Gen 1 <-> Gen 2 trading |
| Trade Center | LINK_TRADECENTER | Gen 2 <-> Gen 2 trading |
| Colosseum | LINK_COLOSSEUM | Gen 2 <-> Gen 2 battle |

### Serial Data Constants

| Constant | Purpose |
|----------|---------|
| SERIAL_PREAMBLE_BYTE | Marks start of data blocks |
| SERIAL_NO_DATA_BYTE | Placeholder / padding byte |
| SERIAL_PATCH_LIST_PART_TERMINATOR | Ends a patch list section |
| SERIAL_MAIL_PREAMBLE_BYTE | Marks start of mail data |
| SERIAL_MAIL_REPLACEMENT_BYTE | Replaced with SERIAL_NO_DATA_BYTE after transfer |
| SERIAL_LINK_BYTE_TIMEOUT | Timeout for individual byte transfers |

### Clock Roles

- **Internal clock (USING_INTERNAL_CLOCK):** Initiator -- sends clock signal
- **External clock:** Receiver -- syncs to incoming clock
- Player using internal clock gets extra delay frames before starting exchange

---

## Data Exchange Flow

### Gen 2 to Gen 2 (Gen2ToGen2LinkComms)

1. **ClearLinkData** -- zero out link buffers
2. **Link_PrepPartyData_Gen2** -- stage player party data for transfer
3. **FixDataForLinkTransfer** -- encode data for serial safety
4. **CheckLinkTimeout_Gen2** -- verify connection is alive (Gen 2 only; Gen 1 path skips this)
5. **Exchange RNG state** -- swap random number preamble + seeds (SERIAL_RN_PREAMBLE_LENGTH + SERIAL_RNS_LENGTH bytes) via Serial_ExchangeBytes
6. **Exchange party data** -- swap full party structs: player name + party count + species list + 2 bytes + (PARTYMON_STRUCT_LENGTH + NAME_LENGTH * 2) per party mon + 3 trailer bytes
7. **Exchange patch lists** -- swap SERIAL_PATCH_LIST_LENGTH bytes (fix bytes that collide with control characters)
8. **Exchange mail** (trade center only) -- swap wLinkPlayerMail via ExchangeBytes
9. **Re-enable interrupts** (JOYPAD + SERIAL + TIMER + VBLANK)
10. **Copy random numbers** -- Link_CopyRandomNumbers for battle RNG sync
11. **Parse OT data** -- copy from link buffer into wOT* WRAM fields
12. **Apply patch lists** -- restore bytes that were replaced during transfer
13. **Process mail** (trade center) -- find mail preamble, copy messages, handle language conversion (English to French/German/Spanish/Italian)
14. **Copy OT party** -- populate wOTPlayerName, wOTPartyCount, wOTPlayerID, wOTPartyMons
15. **Branch:**
    - LINK_COLOSSEUM: set wOtherTrainerClass = CAL, start link battle via StartBattle predef
    - LINK_TRADECENTER: play MUSIC_ROUTE_30, initialize trade menu

### Gen 2 to Gen 1 (Gen2ToGen1LinkComms / Time Capsule)

1. **ClearLinkData** -- zero out link buffers
2. **Link_PrepPartyData_Gen1** -- stage party data in Gen 1 format (REDMON_STRUCT_LENGTH per Pokemon)
3. **FixDataForLinkTransfer** -- encode
4. **Exchange RNG state** -- same as Gen 2 path
5. **Exchange party data** -- uses REDMON_STRUCT_LENGTH instead of PARTYMON_STRUCT_LENGTH
6. **Exchange patch lists** -- same as Gen 2
7. **Re-enable interrupts**
8. **Validate party count** -- reject if 0 or >= 7 Pokemon
9. **Copy OT data** with Gen 1 struct sizes
10. **Apply patch lists** -- restore patched bytes using SERIAL_PATCH_DATA_SIZE offset
11. **Convert species** -- ConvertMon_1to2 via Pokered_MonIndices lookup table for each OT party species
12. **Convert party structs** -- Link_ConvertPartyStruct1to2 adapts Gen 1 mon structs to Gen 2 format
13. **Initialize trade menu** -- play MUSIC_ROUTE_30, display trade screen

---

## Patch List System

Serial transfer uses control characters (preamble, terminator, no-data) that can collide with actual Pokemon data bytes. The patch list system handles this:

1. **Before sending:** scan party data for bytes matching control characters; replace them with safe values and record their positions in wPlayerPatchLists
2. **After receiving:** read wOTPatchLists; for each position entry, replace the byte at that offset with SERIAL_NO_DATA_BYTE (the actual value)
3. **Two sections** per patch list, separated by SERIAL_PATCH_LIST_PART_TERMINATOR
4. **Data size:** SERIAL_PATCH_DATA_SIZE bytes per patch section

---

## Species Index Conversion (time_capsule_2.asm)

### ConvertMon_2to1
- Input: Gen 2 species index in wTempSpecies
- Scans Pokered_MonIndices table sequentially
- Returns the position (1-indexed) where the Gen 2 index appears
- This maps Gen 2 internal order to Gen 1 internal order

### ConvertMon_1to2
- Input: Gen 1 species index in wTempSpecies
- Uses index as direct offset into Pokered_MonIndices table
- Returns the Gen 2 species number stored at that position
- The table is included from data/pokemon/gen1_order.asm

---

## Time Capsule Validation (time_capsule.asm)

### ValidateOTTrademon
Checks that the incoming trade Pokemon is legitimate:

1. **Species match:** the species byte in the party list must match the species byte in the mon struct (or be an EGG)
2. **Level cap:** level must be <= MAX_LEVEL (100)
3. **Type consistency (Time Capsule only):**
   - Loads the species' base data and compares Type1/Type2
   - **Exception:** Magnemite and Magneton skip the type check (their type changed from pure Electric in Gen 1 to Electric/Steel in Gen 2)
   - If types don't match, the trade is flagged as abnormal (carry flag set)

### CheckAnyOtherAliveMonsForTrade
- Ensures the player has at least one other alive (non-zero HP) Pokemon if they trade away the selected one
- Also checks: if all player's other mons are fainted, the incoming OT mon must have non-zero HP (prevents both players having all-fainted parties)

---

## Mystery Gift Protocol (mystery_gift.asm, mystery_gift_2.asm)

### Communication Method
- Uses Game Boy Color **infrared port** (rRP register), NOT the serial link cable
- IR_SENDER and IR_RECEIVER roles determined by InitializeIRCommunicationRoles

### IR Communication Constants

| Constant | Value | Description |
|----------|-------|-------------|
| REGION_PREFIX | $96 | Region check prefix byte |
| REGION_CODE | $90 | USA region code |
| MESSAGE_PREFIX | $5a | Message data prefix |
| NAME_CARD_PREFIX | $3c | Name card data prefix |

### Error Flags (hMGStatusFlags)

| Flag | Bit | Meaning |
|------|-----|---------|
| MG_WRONG_CHECKSUM | 0 | Checksum validation failed |
| MG_TIMED_OUT | 1 | Communication timed out |
| MG_CANCELED | 4 | User pressed B to cancel |
| MG_WRONG_PREFIX | 7 | Region/message prefix mismatch |
| MG_OKAY | ~all above | All checks passed |

### Exchange Flow (ExchangeMysteryGiftData)
1. Disable interrupts, clear audio channels
2. Initialize IR communication
3. Negotiate sender/receiver roles
4. Check status flags -- retry on error, abort on cancel
5. **Sender path:** send payload, switch to receiver, receive payload, switch to sender, send empty block
6. **Receiver path:** receive payload, switch to sender, send payload, switch to receiver, receive empty block
7. Timeout: 4 seconds (60 * 4 = 240 VBlank frames) before restarting

### Data Staged for Exchange (StageDataForMysteryGift)

The following data is packed into wMysteryGiftPlayerData:
1. Game version (GS_VERSION + 1)
2. Player ID (2 bytes from SRAM)
3. Player name (NAME_LENGTH bytes)
4. Pokedex caught count (1 byte)
5. Random decoration/item flag (1 bit, 50/50)
6. Random item sample 1 (tiered probability)
7. Random item sample 2 (tiered probability)
8. Backup mystery gift item (from SRAM)
9. Number of daily mystery gift partners

### Item/Decoration Probability Tiers

The RandomSample function uses cascading probability:

| Tier | Probability | Item range |
|------|-------------|------------|
| Common | ~90% | Items 0-15 (from MysteryGiftItems table) |
| Uncommon | ~8% | Items 16-23 |
| Rare | ~1.6% | Items 24-31 |
| Very Rare | ~0.4% | Items 32-33 |

### Daily Limits
- **5 gifts per day maximum** (MAX_MYSTERY_GIFT_PARTNERS) -- tracked via sNumDailyMysteryGiftPartnerIDs
- **1 gift per person per day** -- partner IDs stored in sDailyMysteryGiftPartnerIDs, checked against wMysteryGiftPartnerID
- Compatible with Pokemon Pikachu 2 (POKEMON_PIKACHU_2_VERSION) -- skips daily limit checks

### Gift Types
- **Decoration:** sent to player's room, checked against already-received decorations
- **Item:** stored in sBackupMysteryGiftItem, retrieved at Poke Mart counter
- **Fallback:** if index out of range, defaults to DECOFLAG_RED_CARPET / GREAT_BALL

### Mystery Gift Trainer House
- Partner's name saved to sMysteryGiftPartnerName
- Trainer data saved to sMysteryGiftTrainer
- sMysteryGiftTrainerHouseFlag set to TRUE
- Enables battling the mystery gift partner's team in Viridian City Trainer House

---

## Trade Screen UI (link_trade.asm)

### Graphics
- LinkCommsBorderGFX: 2bpp tile graphics for trade screen borders
- CableTradeBorderTopTilemap / CableTradeBorderBottomTilemap: cable trade layout
- MobileTradeBorderTilemap: mobile adapter layout (Japanese)

### InitTradeSpeciesList
1. Load border graphics
2. Load cable trade tilemap
3. Initialize palette map
4. Place both players' names and party species lists
5. Add "CANCEL" option at bottom

### PlaceTradePartnerNamesAndParty
- Player name at coords (4,0), OT name at coords (4,8)
- Player party species listed at (7,1), OT party at (7,9)
- Species names looked up via GetPokemonName

### LinkTradeMenu
- 2D menu system for selecting trade Pokemon
- Cursor navigates between player/OT party lists
- Handles joypad filtering for simultaneous input

---

## Link Battle Setup

### Colosseum Entry (from link.asm)
1. OT trainer class set to CAL (generic link opponent)
2. OT player name copied to wOTClassName
3. Options temporarily set to TEXT_DELAY_MED, stereo preserved
4. Text acceleration disabled during link battle
5. STAT interrupt enabled (rIE bit B_IE_STAT) for LY compare during battle
6. StartBattle predef called
7. After battle: restore options, reload Pokemon data, exit link communications

### Link Timeout
- If Gen 2 connection check fails: LinkTimeout displayed
- "Link has timed out" message shown for 10 frames
- Screen cleared, SGB layout reset

---

## Mail Transfer (Gen 2 to Gen 2)

### Mail Exchange
- Mail data exchanged separately after party data (trade center only)
- Uses ExchangeBytes (no preamble check, unlike Serial_ExchangeBytes)

### Mail Processing After Receipt
1. Find SERIAL_MAIL_PREAMBLE_BYTE in received data
2. Skip padding (SERIAL_NO_DATA_BYTE and duplicate preambles)
3. Copy mail messages (MAIL_MSG_LENGTH + 1 per party member)
4. Copy mail metadata (author, item, etc.)
5. Apply mail patch list
6. **Language conversion:**
   - ParseMailLanguage determines source language
   - ConvertEnglishMailToFrenchGerman for language codes < 3
   - ConvertEnglishMailToSpanishItalian for language codes 3-4
   - No conversion for same-language or Japanese

---

## Virtual Console Patches

The link code contains multiple vc_patch and vc_hook points for the 3DS Virtual Console wireless adapter:

| Hook/Patch | Purpose |
|------------|---------|
| Wireless_net_delay_5 / _8 | Increase delay from 3 to 26 frames for wireless latency |
| Wireless_ExchangeBytes_* | Hook exchange functions for wireless transport |
| Infrared_ExchangeMysteryGiftData_* | Replace IR communication with wireless for VC |
| Infrared_stage_party_data | VC stages extra party data for Mystery Gift |

In the VC build (_CRYSTAL11_VC), the IR mystery gift exchange is replaced with a dummy delay loop that waits for the wireless adapter to handle the actual data transfer externally.

---

## Init List (init_list.asm)

### InitList (noted as "useless" in source)
- Sets up wListPointer and wNamedObjectType based on wInitListType
- Handles: INIT_ENEMYOT_LIST, INIT_PLAYEROT_LIST, INIT_MON_LIST, INIT_BAG_ITEM_LIST
- Sets wItemAttributesPointer to ItemAttributes
- The function appears to be legacy code that is no longer called by the link system
