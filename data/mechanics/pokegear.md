# Pokemon Crystal - Pokegear System

Complete documentation of the Pokegear, its cards, and all subsystems. Source: pokecrystal engine/pokegear/, engine/phone/, data/phone/, constants/phone_constants.asm.

---

## Overview

The Pokegear is a multi-function device given to the player by Mom at the start of the game. It has four cards (screens), unlocked progressively through the adventure. Access it from the menu or with the dedicated key.

---

## Cards

### 1. Clock Card (POKEGEARCARD_CLOCK)
**Available:** From game start
**Function:** Displays the current time (hours:minutes) and day of the week. The internal clock runs in real time using the Game Boy's RTC (Real-Time Clock). Time affects wild encounters, berry growth, daily events, phone calls, and radio programming.

**Time periods:**
- Morning: 4:00 AM - 9:59 AM
- Day: 10:00 AM - 5:59 PM
- Night: 6:00 PM - 3:59 AM

### 2. Map Card (POKEGEARCARD_MAP)
**Available:** Given by the Guide Gent in Cherrygrove City (first visit)
**Function:** Displays the Town Map with the player's current position. Toggles between Johto and Kanto maps.

**Features:**
- Cursor can be moved to any landmark to see its name
- Player icon shows current position (animated walking sprite)
- When on the S.S. Aqua, a special Fast Ship icon replaces the player sprite
- Shows Fly destinations (landmarks only, not routes)
- Two regions: JOHTO_REGION and KANTO_REGION, switchable with Select

### 3. Phone Card (POKEGEARCARD_PHONE)
**Available:** From game start
**Function:** View registered phone numbers and make calls. See Phone System section below.

**UI:** Scrollable list of up to 10 contacts. Cursor navigates with D-pad, A to call, B to close.

### 4. Radio Card (POKEGEARCARD_RADIO)
**Available:** Obtained from the Goldenrod Radio Tower after answering 5 quiz questions correctly
**Function:** Tune in to radio stations. See Radio System (data/mechanics/radio_system.md) for full details.

**Upgrades:**
- EXPN Card: Obtained from the Lavender Radio Tower manager after restoring power. Unlocks Kanto radio stations and enhanced Pokemon Music channel features.

---

## Phone System

### Basics
- Maximum contacts: 10 (CONTACT_LIST_SIZE)
- Some contacts are permanent (Mom, Prof. Elm) and cannot be deleted
- Trainers offer their number after being defeated on certain routes
- Phone calls can be received (incoming) or placed (outgoing)
- Calls are blocked in areas with the phone service flag set to TRUE (gyms, dungeons, etc.)

### Permanent Contacts

| Contact | When Registered |
|---------|----------------|
| Mom | Always registered from start |
| Prof. Elm | Registered from start |
| Prof. Oak | After getting the Radio Card |
| Bill | After meeting Bill |

### Registerable Trainer Contacts

| Contact | Class | Route | Notable Feature |
|---------|-------|-------|-----------------|
| Youngster Joey | Youngster | Route 30 | Famous for calling about his top percentage Rattata |
| Bug Catcher Wade | Bug Catcher | Route 31 | Gives berries |
| Fisher Ralph | Fisher | Route 32 | Reports Qwilfish swarms |
| Picnicker Liz | Picnicker | Route 32 | Reports item finds (Thunderstone) |
| Hiker Anthony | Hiker | Route 33 | Reports Dunsparce swarms |
| Camper Todd | Camper | Route 34 | Reports item finds |
| Picnicker Gina | Picnicker | Route 34 | Gives Leaf Stone |
| Juggler Irwin | Juggler | Route 35 | Talks about player obsessively |
| Bug Catcher Arnie | Bug Catcher | Route 35 | Reports Yanma swarms |
| Schoolboy Alan | Schoolboy | Route 36 | Reports item finds (Fire Stone) |
| Lass Dana | Lass | Route 38 | Gives Thunderstone |
| Schoolboy Chad | Schoolboy | Route 38 | Reports item finds |
| Pokefan Derek | Pokefan (M) | Route 39 | Reports item finds |
| Fisher Tully | Fisher | Route 42 | Gives Water Stone |
| Pokemaniac Brent | Pokemaniac | Route 43 | Reports item finds |
| Picnicker Tiffany | Picnicker | Route 43 | Gives Pink Bow |
| Bird Keeper Vance | Bird Keeper | Route 44 | Reports item finds |
| Fisher Wilton | Fisher | Route 44 | Reports Remoraid swarms |
| Blackbelt Kenji | Blackbelt | Route 45 | Reports item finds |
| Hiker Parry | Hiker | Route 45 | Reports Marill swarms |
| Picnicker Erin | Picnicker | Route 46 | Reports item finds |
| Schoolboy Jack | Schoolboy | National Park | Reports item finds |
| Pokefan Beverly | Pokefan (F) | National Park | Gives Nuggets |
| Sailor Huey | Sailor | Lighthouse | Reports item finds |
| Cooltrainer Gaven | Cooltrainer (M) | Route 26 | Post-game contact |
| Cooltrainer Beth | Cooltrainer (F) | Route 26 | Post-game contact |
| Bird Keeper Jose | Bird Keeper | Route 27 | Post-game contact |
| Cooltrainer Reena | Cooltrainer (F) | Route 27 | Post-game contact |
| Buena | (Special) | Dept Store Roof | Registered after Buena's Password event |

### Incoming Call Mechanics

Incoming calls trigger randomly in the overworld with these conditions:
1. Player must not be standing on a door/entrance tile
2. A time-based cooldown timer must have elapsed
3. 50% random chance check
4. Current map must not have phone service disabled (phone_flag = FALSE in maps.asm)
5. At least one registered contact must be available to call

The system then:
1. Gets all available callers (contacts registered in phone list)
2. Checks time-of-day compatibility for each caller
3. Randomly selects one caller from the available pool
4. Loads and executes their caller script

### Call Content Types

**Trainer calls can include:**
- **Item gifts:** Trainer found a rare item and holds it for you. Pick up at their route.
- **Swarm reports:** Rare Pokemon appearing on a specific route. Swarms last until midnight.
- **Rematch offers:** Trainer wants to battle again with stronger team.
- **Random chatter:** Flavor dialogue about their training, the weather, etc.
- **Gossip:** Some trainers share rumors about other trainers or Pokemon.

### Special Calls (Elm)

Prof. Elm calls for story events:
| Special Call | Trigger |
|-------------|---------|
| SPECIALCALL_POKERUS | Random; reports Pokerus discovery |
| SPECIALCALL_ROBBED | After Team Rocket event at Lake of Rage |
| SPECIALCALL_ASSISTANT | His aide has something for you |
| SPECIALCALL_WEIRDBROADCAST | Team Rocket Radio Tower takeover |
| SPECIALCALL_SSTICKET | S.S. Ticket is ready |
| SPECIALCALL_BIKESHOP | Bike shop in Goldenrod has a bike for you |
| SPECIALCALL_WORRIED | Story concern about your journey |
| SPECIALCALL_MASTERBALL | Master Ball reward after defeating Team Rocket |

### Mom Calls

Mom manages your money savings. She can:
- Report purchases she made with your savings (evolution stones, berries, decorations for your room)
- Ask if you want to keep saving or stop
- Her item pool includes: Super Potion, Repel, Antidote, and occasionally rare items like Moon Stone

---

## Pokegear Technical Details

### State Machine
The Pokegear uses a jumptable-based state machine with 13 states:
- CLOCKINIT/CLOCKJOYPAD: Clock display
- MAPCHECKREGION/JOHTOMAPINIT/JOHTOMAPJOYPAD: Johto map
- KANTOMAPINIT/KANTOMAPJOYPAD: Kanto map
- PHONEINIT/PHONEJOYPAD/MAKEPHONECALL/FINISHPHONECALL: Phone
- RADIOINIT/RADIOJOYPAD: Radio

### Card Switching
Left/Right on the D-pad cycles between available cards. Not all cards are always available -- Map requires the Map Card, Radio requires the Radio Card. The indicator arrow at the top shows current card position.

### Music Handling
When opening Pokegear, the current map music is preserved. Radio stations play their own music. When exiting Pokegear, if a radio station was playing, the game checks whether to restart map music or use enter-map music transitions (RESTART_MAP_MUSIC = $FE, ENTER_MAP_MUSIC = $FF).

---

## Fly System (Map Card Related)

While not directly a Pokegear feature, the Map Card shows Fly destinations. Fly is available after obtaining HM02 (from Chuck's wife in Cianwood) and the Storm Badge.

### Johto Fly Points
New Bark Town, Cherrygrove City, Violet City, Azalea Town, Goldenrod City, Ecruteak City, Olivine City, Cianwood City, Mahogany Town, Lake of Rage, Blackthorn City, Silver Cave

### Kanto Fly Points
Pallet Town, Viridian City, Pewter City, Cerulean City, Vermilion City, Celadon City, Lavender Town, Saffron City, Fuchsia City, Cinnabar Island, Indigo Plateau

You can only Fly to cities/landmarks you have visited. Routes are not valid Fly destinations.

---

## Trainer Swarm System (Phone-Triggered)

Certain phone contacts report Pokemon swarms -- temporary appearances of rare Pokemon on their route. Active swarms override normal encounter tables until midnight.

| Reporter | Route | Swarm Pokemon |
|----------|-------|---------------|
| Fisher Ralph | Route 32 | Qwilfish |
| Hiker Anthony | Route 33 | Dunsparce |
| Bug Catcher Arnie | Route 35 | Yanma |
| Fisher Wilton | Route 44 | Remoraid |
| Hiker Parry | Route 45 | Marill |

---

## Trainer Rematch System (Phone-Triggered)

After receiving their phone number, trainers can call to request rematches. Their teams get progressively stronger with each rematch. Rematch availability:
- Trainers call randomly (subject to cooldown timers)
- Meet them at their original route location
- Levels scale up each time (up to 3 rematch tiers)
- Some trainers give items as rewards for rematches

---

## Buena's Password (Phone + Radio Integration)

After participating in Buena's Password on the radio:
1. Listen to the password on the radio (6 PM - midnight)
2. Go to the Goldenrod Radio Tower and tell Buena the password
3. Earn Blue Card points (varying by password category: 6-16 points)
4. Redeem points for prizes

After participating, Buena's number is registered. She can call about upcoming broadcasts.

### Password Categories (from buenas_passwords.asm)
| Category | Examples | Points |
|----------|----------|--------|
| Johto Starters | Cyndaquil, Totodile, Chikorita | 10 |
| Beverages | Fresh Water, Soda Pop, Lemonade | 12 |
| Healing Items | Potion, Antidote, Parlyz Heal | 12 |
| Balls | Poke Ball, Great Ball, Ultra Ball | 12 |
| Pokemon 1 | Pikachu, Rattata, Geodude | 10 |
| Pokemon 2 | Hoothoot, Spinarak, Drowzee | 10 |
| Johto Towns | New Bark Town, Cherrygrove City, Azalea Town | 16 |
| Types | Flying, Bug, Grass | 6 |
| Moves | Tackle, Growl, Mud-Slap | 12 |
| X Items | X Attack, X Defend, X Speed | 12 |
| Radio Stations | Pokemon Talk, Pokemon Music, Lucky Channel | 13 |
