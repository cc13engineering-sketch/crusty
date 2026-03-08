# Pokemon Crystal — Key NPCs, Gift Pokemon, and Services

Source: pokecrystal disassembly (engine/events/, data/events/, data/phone/)

---

## Move Tutors

Pokemon Crystal has three move tutors, all in Goldenrod City Game Corner area:

| Move | Tutor Location | Conditions | Cost |
|------|---------------|------------|------|
| Flamethrower | Goldenrod Game Corner (MOVETUTOR_FLAMETHROWER) | Can learn via TM/HM compatibility check | 4000 coins |
| Thunderbolt | Goldenrod Game Corner (MOVETUTOR_THUNDERBOLT) | Can learn via TM/HM compatibility check | 4000 coins |
| Ice Beam | Goldenrod Game Corner (MOVETUTOR_ICE_BEAM) | Can learn via TM/HM compatibility check | 4000 coins |

Move tutor grants +1 happiness (HAPPINESS_LEARNMOVE) on successful teaching.

Source: `engine/events/move_tutor.asm`

---

## Name Rater

**Location:** Goldenrod City

**Function:** Rename any Pokemon you own (matching OT name and ID). Cannot rename:
- Eggs
- Pokemon received in trades (different OT name or ID)

**Restrictions:**
- Max nickname length: 10 characters (MON_NAME_LENGTH - 1)
- Cannot set to empty name or same as current name
- Uses NamingScreen UI

Source: `engine/events/name_rater.asm`

---

## Move Deleter

**Location:** Blackthorn City

**Function:** Delete any single move from a Pokemon's moveset, including HM moves. Cannot delete:
- The last remaining move (Pokemon must keep at least 1 move)
- Moves from Eggs (will not operate on Eggs)

Source: `engine/events/move_deleter.asm`

---

## Gift Pokemon

### Story/Event Gift Pokemon

| Pokemon | Level | Location | Giver | Special Notes |
|---------|-------|----------|-------|---------------|
| Cyndaquil/Totodile/Chikorita | 5 | New Bark Town (Elm's Lab) | Prof. Elm | Starter choice |
| Togepi Egg | - | Elm's Lab (after beating Falkner) | Elm's Aide | Hatches into Togepi |
| Eevee | 20 | Goldenrod City (Bill's house) | Bill | After meeting Bill in Ecruteak |
| Shuckle (SHUCKIE) | 15 | Cianwood City | Mania | OT: MANIA (ID: 00518), holds Berry. Can be returned if happiness < 150; if >= 150, Mania lets you keep it |
| Dratini | 15 | Dragon's Den | Elder | After answering quiz correctly. Special moveset: Wrap, Thunder Wave, Twister, ExtremeSpeed (if all answers "correct") or normal moveset |
| Tyrogue | 10 | Mt. Mortar | Karate King | After defeating him in battle |
| Spearow (KENYA) | ~10 | Route 35 gate | Guard | Carrying mail to deliver to Route 31 |
| Odd Egg | - | Goldenrod Pokemon Center | Daycare Man | Crystal-exclusive. Random species from baby Pokemon pool |
| Sudowoodo | 20 | Route 36 | Wild encounter | Use SquirtBottle (static encounter) |
| Red Gyarados | 30 | Lake of Rage | Wild encounter | Static shiny encounter |
| Suicune | 40 | Tin Tower | Wild encounter | Crystal-exclusive location |
| Ho-Oh | 60 | Tin Tower | Wild encounter | Need Rainbow Wing |
| Lugia | 60 | Whirl Islands | Wild encounter | Need Silver Wing |
| Snorlax | 50 | Route 11 | Wild encounter | Wake with Poke Flute (radio) |
| Lapras | 20 | Union Cave B2F | Wild encounter | Only appears on Fridays |

### Shuckle (SHUCKIE) Special Mechanics
- OT: "MANIA", OT ID: 00518
- Holds Berry
- Daily flag: DAILYFLAGS1_GOT_SHUCKIE_TODAY_F
- Return check: Must be Shuckle species with matching OT name and OT ID
- If happiness >= 150 when returning, Mania says it's too happy and lets you keep it
- If fainted, Mania won't accept it back (different dialog)

Source: `engine/events/shuckle.asm`

### Dratini Special Movesets
- ExtremeSpeed moveset (correct Dragon's Den answers): Wrap, Thunder Wave, Twister, ExtremeSpeed
- Normal moveset (wrong answers): Wrap, Leer, Thunder Wave, Twister

Source: `engine/events/dratini.asm`

### Odd Egg (Crystal-exclusive)
Random species with weighted probabilities. Species pool: Pichu, Cleffa, Igglybuff, Tyrogue, Smoochum, Elekid, Magby. Each has predetermined DVs and moves. The Odd Egg has a higher shiny chance than normal (14% in Japanese, different in localized).

Source: `engine/events/odd_egg.asm`, `data/events/odd_eggs.asm`

---

## In-Game Trades (NPC Trades)

| Give | Receive | Nickname | OT | Location | Held Item | Notes |
|------|---------|----------|----|----------|-----------|-------|
| Abra | Machop | MUSCLE | MIKE | Goldenrod Dept Store | Gold Berry | - |
| Bellsprout | Onix | ROCKY | KYLE | Violet City | Bitter Berry | - |
| Krabby | Voltorb | VOLTY | TIM | Olivine City | PRZCureBerry | - |
| Dragonair | Dodrio | DORIS | EMY | Blackthorn City | Smoke Ball | Female only |
| Haunter | Xatu | PAUL | CHRIS | Cianwood City | MysteryBerry | - |
| Chansey | Aerodactyl | AEROY | KIM | Route 14 | Gold Berry | - |
| Dugtrio | Magneton | MAGGIE | FOREST | Power Plant | Metal Coat | - |

Each NPC trade:
- Has predetermined DVs (set in the trade data, not random)
- Can only be completed once per save file (uses wTradeFlags)
- Has specific dialog sets (Collector, Happy, Girl, Newbie)
- Some require specific gender of the given Pokemon

Source: `data/events/npc_trades.asm`, `engine/events/npc_trade.asm`

---

## Phone Contacts

### Key Contacts (always available)

| Contact | Location | Features |
|---------|----------|----------|
| Mom | Player's House | Bank of Mom (save/withdraw money), DST toggle, buys decorations |
| Prof. Elm | New Bark Town | Evolution advice, story progression calls |
| Bill | N/A | PC Box notifications, full box warnings |
| Buena | Goldenrod Dept Store Roof | Buena's Password (radio show) |

### Trainer Phone Contacts (rematch + tips)

| Contact | Class | Route/Location |
|---------|-------|----------------|
| Joey | Youngster | Route 30 |
| Wade | Bug Catcher | Route 31 |
| Ralph | Fisher | Route 32 |
| Liz | Picnicker | Route 32 |
| Anthony | Hiker | Route 33 |
| Todd | Camper | Route 34 |
| Gina | Picnicker | Route 34 |
| Irwin | Juggler | Route 35 |
| Arnie | Bug Catcher | Route 35 |
| Alan | Schoolboy | Route 36 |
| Dana | Lass | Route 38 |
| Chad | Schoolboy | Route 38 |
| Derek | PokefanM | Route 39 |
| Tully | Fisher | Route 42 |
| Brent | Pokemaniac | Route 43 |
| Tiffany | Picnicker | Route 43 |
| Vance | Bird Keeper | Route 44 |
| Wilton | Fisher | Route 44 |
| Kenji | Blackbelt | Route 45 |
| Parry | Hiker | Route 45 |
| Erin | Picnicker | Route 46 |
| Jack | Schoolboy | National Park |
| Beverly | PokefanF | National Park |
| Huey | Sailor | Olivine Lighthouse |
| Gaven | CooltrainerM | Route 26 |
| Beth | CooltrainerF | Route 26 |
| Jose | Bird Keeper | Route 27 |
| Reena | CooltrainerF | Route 27 |

### Phone Call System
- Trainers can call you with: rematch requests, item notifications (found rare items), swarm reports, general chat
- You can call trainers to arrange rematches
- Call receive delay system: starts at 20-minute timer, decreases with consecutive calls (20 -> 10 -> 5 -> 3 minutes)
- Call timing controlled by wTimeCyclesSinceLastCall

Source: `data/phone/phone_contacts.asm`, `engine/events/specials.asm`, `engine/overworld/time.asm`

---

## Mom's Banking System

**Location:** Accessible via phone or in person

**Features:**
- Deposit money (wallet -> Mom's savings)
- Withdraw money (Mom's savings -> wallet)
- Auto-save option: Mom saves a portion of prize money automatically when toggled on
- DST toggle: Mom asks about Daylight Saving Time adjustment

**Auto-save behavior:** When MOM_SAVING_SOME_MONEY_F is set, a percentage of trainer battle winnings is automatically deposited.

**Max money:** Wallet max 999,999. Mom's savings max 999,999 (separate 3-byte value).

**Mom buys items:** Mom will occasionally call to say she bought something for you with your savings. Items she can buy include decorations (dolls, posters) from the Goldenrod Dept Store.

Source: `engine/events/mom.asm`

---

## Key Story NPCs

### Professor Elm
- Gives starter Pokemon
- Gives Togepi Egg
- Phone contact for evolution advice
- Located in New Bark Town

### Professor Oak
- Gives Pokedex
- Located in his lab (Pallet Town, accessible later)
- Prof Oak's PC accessible from Pokemon Centers

### Bill
- Storage system operator
- Gives Eevee in Goldenrod City
- Phone calls about full PC boxes

### Kurt
- Converts Apricorns into special Poke Balls
- Located in Azalea Town
- Takes one Apricorn per day, ball ready next day
- Ball types: Level Ball (RED), Lure Ball (BLU), Moon Ball (YLW), Friend Ball (GRN), Love Ball (PNK), Heavy Ball (BLK), Fast Ball (WHT)

### Eusine
- Suicune researcher
- Appears in Cianwood, Mahogany, Tin Tower
- Battles player once before Suicune encounter

### Kimono Girls
- Five sisters in Ecruteak City
- Each uses a different Eeveelution
- Must defeat all for Clear Bell / Suicune access (Crystal)

---

## Buena's Password

**Location:** Goldenrod Radio Tower (Buena's show on radio), prize counter at Goldenrod Dept Store Roof

**Mechanics:**
- Listen to Buena's show on the radio to hear the daily password
- Tell Buena the password at the Dept Store Roof to earn a Blue Card point
- Blue Card points exchanged for prizes

Source: `engine/events/buena.asm`, `engine/events/buena_menu.asm`

---

## Game Corner (Goldenrod City)

**Games:** Card Flip, Slot Machines

**Prize Pokemon:**
- Abra (100 coins)
- Cubone (800 coins)
- Wobbuffet (1500 coins, Crystal only)

**Prize TMs:**
- TM25 Thunder (5500 coins)
- TM14 Blizzard (5500 coins)
- TM38 Fire Blast (5500 coins)

Source: `engine/events/specials.asm` — GameCornerPrizeMonCheckDex

---

## Lucky Number Show

**Day:** Every Friday
**Location:** Goldenrod Radio Tower

**Mechanics:**
- Weekly lucky number drawn (5-digit ID)
- Checks all party Pokemon and PC box Pokemon IDs
- Match last 2 digits: PP Up
- Match last 3+ digits: Exp. Share
- Match all 5 digits: Master Ball

Source: `engine/events/lucky_number.asm`

---

## Fruit Trees

Berry trees throughout Johto and Kanto that regenerate daily. Each tree produces a specific berry type. Trees are checked against daily flags (DAILYFLAGS1_ALL_FRUIT_TREES_F) and reset once per day.

### Johto Fruit Trees
- Route 29: Berry
- Route 30: Berry, PSNCureBerry
- Route 31: Bitter Berry
- Route 33: PSNCureBerry
- Route 35: MysteryBerry
- Route 36: Ice Berry
- Route 37: Red Apricorn, Blue Apricorn, Black Apricorn
- Route 38: Berry
- Route 39: Mint Berry
- Route 42: Pink Apricorn, Green Apricorn, Yellow Apricorn
- Route 43: Bitter Berry
- Route 44: Burnt Berry
- Route 45: MysteryBerry
- Route 46: Berry, PRZCureBerry
- Violet City: PRZCureBerry
- Azalea Town: White Apricorn

### Kanto Fruit Trees
- Route 1: Bitter Berry
- Route 2: PSNCureBerry
- Route 8: PRZCureBerry
- Route 11: Berry
- Route 26: Ice Berry
- Pewter City: Ice Berry, Mint Berry
- Fuchsia City: Burnt Berry

Source: `data/items/fruit_trees.asm`
