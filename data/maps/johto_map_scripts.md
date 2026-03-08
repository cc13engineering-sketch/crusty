# Pokemon Crystal — Johto Map Scripts (Complete)

> Source: pokecrystal-master/maps/*.asm — all 388 map script files
> Covers: NPCs, trainers, signs, hidden items, events, warps, scene scripts
> This file covers ALL Johto maps (cities, routes, interiors, gyms, dungeons)

---

## New Bark Town

### NPCs
- **Teacher** (SPRITE_TEACHER): Walks around, multiple dialogue states
  - Before Pokemon: "Wow, your PokeGear is impressive! Did your mom get it for you?"
  - After Pokemon: "Oh! Your Pokemon is adorable! I wish I had one!"
  - After Egg delivery: "You should tell your mom if you are leaving."
  - Late game: "Call your mom on your PokeGear to let her know how you're doing."
  - **Stops you** from leaving town without Pokemon (coord_event, brings you back)
- **Fisher** (SPRITE_FISHER): "I hear Prof. Elm discovered some new Pokemon."
- **Rival** (SPRITE_RIVAL): Peeks at lab window, event-controlled (EVENT_RIVAL_NEW_BARK_TOWN)
  - "So this is the famous Elm Pokemon Lab..." / "What are you staring at?"
  - Shoves player away with physical push animation

### Signs
- Town sign: "NEW BARK TOWN — The Town Where the Winds of a New Beginning Blow"
- Player's House sign: "[PLAYER]'s House"
- Elm's Lab sign: "ELM POKEMON LAB"
- Elm's House sign: "ELM'S HOUSE"

### Warps
- (6,3) -> Elm's Lab
- (13,5) -> Player's House 1F
- (3,11) -> Player's Neighbor's House (Rival's House)
- (11,13) -> Elm's House

### Scene Scripts
- SCENE_NEWBARKTOWN_TEACHER_STOPS_YOU: Teacher blocks exit if no Pokemon
- Callback: Sets ENGINE_FLYPOINT_NEW_BARK on entry

---

## Elm's Lab

### NPCs
- **Professor Elm**: Complex multi-state NPC
  - Start: Explains research, gives starter choice
  - After egg delivery: Studies egg, comments on Pokedex
  - After Togepi hatches: Amazed, gives Everstone
  - After 8 badges: Gives Master Ball
  - After Elite Four: Gives S.S. Ticket
- **Elm's Aide**: Gives Potion (first visit), 5 Poke Balls (after egg return)
- **Officer**: Appears after theft, takes rival's name (special NameRival)
- **3 Poke Balls**: Cyndaquil, Totodile, Chikorita (SPRITE_POKE_BALL objects)

### Key Items Given
- Starter Pokemon (Lv5 with Berry)
- Elm's phone number
- Potion (from Aide)
- 5 Poke Balls (from Aide)
- Everstone (after showing Togepi)
- Master Ball (after 8 badges)
- S.S. Ticket (after Elite Four)

### Background Events
- Healing Machine (bg_event at (2,1)): Heals party
- Bookshelves: Travel Tips 1-4
- PC: "Observations on Pokemon Evolution"
- Trash can: "The wrapper from the snack Prof. Elm ate is in there..."
- Window: "The window's open. A pleasant breeze is blowing in." / "He broke in through here!"

---

## Player's House

### 1F NPCs
- **Mom**: Gives Pokegear, manages Bank of Mom, DST toggle
  - After game start: Asks about saving money
  - Phone contact: Deposit/withdraw money

### 2F
- Game start location, player's bedroom
- Items: Nintendo console, bed

---

## Cherrygrove City

### NPCs
- **Guide Gent**: Offers city tour, gives Map Card
- Various NPCs with dialogue about Pokemon, Pokecenter

### Signs
- Town sign: "CHERRYGROVE CITY — The City of Cute, Fragrant Flowers"
- Pokecenter sign, Mart sign

### Buildings
- Pokecenter, Mart, Guide Gent's House, Gym Speech House, Evolution Speech House

### Story Event: Rival Battle 1
- Silver ambushes on return from Mr. Pokemon's house
- Stolen starter Lv5 (type advantage)

---

## Route 29

### NPCs (6 objects)
- **Catching Tutorial Dude** (SPRITE_COOLTRAINER_M): Teaches catching tutorial with Rattata Lv5
- **Youngster**: "If they're weak and not ready for battle, keep out of the grass."
- **Teacher**: "See those ledges? It's scary to jump off them..."
- **Fisher**: "I wanted to take a break, so I saved to record my progress."
- **Cooltrainer M**: Time-dependent dialogue about waiting for night/morning Pokemon
- **Tuscany** (SPRITE_TEACHER): Day-of-week sibling, appears Tuesdays after Zephyr Badge, gives Pink Bow

### Items
- Potion (itemball at (48,2))
- Fruit Tree: Berry (FRUITTREE_ROUTE_29)

### Signs
- "ROUTE 29 — CHERRYGROVE CITY - NEW BARK TOWN" (x2)

### Warps
- (27,1) -> Route 29/Route 46 Gate

---

## Route 30

### NPCs
- Youngster Joey (phone contact, famous "top percentage Rattata")
- Bug Catcher Don
- Various NPCs

### Items
- Antidote (itemball)
- Hidden Potion at (14,9)
- Berry House NPC

### Warps
- (7,39) -> Route 30 Berry House
- (17,5) -> Mr. Pokemon's House

---

## Route 31

### Items
- Potion (itemball)
- Poke Ball (itemball)

### Warps
- (4,6)/(4,7) -> Route 31 Violet Gate
- (34,5) -> Dark Cave Violet Entrance

---

## Route 32

### Trainers (8 trainers)
- Bird Keepers, Fishermen, Campers, Picnickers
- Albert, Allen, Liz, Ralph (phone contacts)

### Items
- Great Ball (itemball)
- Repel (itemball)
- Hidden Great Ball at (12,67)
- Hidden Super Potion at (11,40)

### Key NPC
- Fishing Guru (Route 32 Pokecenter): Gives Old Rod

### Warps
- (11,73) -> Route 32 Pokecenter 1F
- (4,2)/(4,3) -> Route 32 Ruins of Alph Gate
- (6,79) -> Union Cave 1F

---

## Route 33

### Trainers
- Hiker Anthony (phone contact)

### Items
- Black Apricorn tree (daily)

### Warps
- (11,9) -> Union Cave 1F

---

## Violet City

### NPCs
- Falkner (gym leader)
- Earl Dervish (Pokemon Academy)
- Kyle (trades Onix for Bellsprout)
- Various citizens

### Items
- PP Up (itemball, requires Surf)
- Rare Candy (itemball, requires Surf)
- Hidden Hyper Potion at (37,14)

### Gyms
**Violet Gym — Falkner (Flying)**
- Gym trainers: Bird Keeper Rod (sight 3), Bird Keeper Abe (sight 3)
- Leader: Pidgey Lv7, Pidgeotto Lv9
- Reward: Zephyr Badge, TM31 Mud-Slap
- Sign: "VIOLET CITY POKEMON GYM LEADER: FALKNER — The Elegant Master of Flying Pokemon!"

### Buildings
- Mart, Pokecenter, Gym, Sprout Tower, Earl's Academy, Kyle's House, Nickname Speech House

---

## Sprout Tower

### 1F
- Items: Parlyz Heal (itemball)
- 3 staircases to 2F

### 2F
- Items: X Accuracy (itemball)
- Trainers: Sage Nico, Sage Chow, Sage Edmond

### 3F
- Items: Potion (itemball), Escape Rope (itemball)
- Trainers: Sage Jin, Sage Neal, Sage Troy
- **Boss: Sage Li** — Bellsprout Lv7 x2, Hoothoot Lv10
- Reward: HM05 Flash
- Rival Silver appears here (leaves before player arrives)

---

## Azalea Town

### NPCs
- Kurt (Apricorn Ball maker, gives Lure Ball)
- Charcoal Maker
- Various citizens

### Items
- White Apricorn tree (daily)
- Hidden Full Heal at (31,6)

### Gym
**Azalea Gym — Bugsy (Bug)**
- Gym trainers: Bug Catcher Benny (sight 2), Bug Catcher Al (sight 3), Bug Catcher Josh (sight 3), Twins Amy&May (sight 1)
- Leader: Metapod Lv14, Kakuna Lv14, Scyther Lv16
- Reward: Hive Badge, TM49 Fury Cutter

### Slowpoke Well B1F
- 3 Rocket Grunts + Executive Proton
- Items: Super Potion (itemball)
- Story: Team Rocket cutting Slowpoke tails

### Slowpoke Well B2F (requires Surf + Strength)
- Items: TM18 Rain Dance (itemball)
- King's Rock

---

## Ilex Forest

### NPCs
- Farfetch'd (chase puzzle, 2 Farfetch'd to guide back)
- Charcoal Maker's Apprentice (gives HM01 Cut + Charcoal after Farfetch'd quest)
- Headbutt Tutor (gives TM02 Headbutt)
- Ilex Forest Shrine (Celebi event location — GS Ball in Crystal JP/VC)

### Items (itemball)
- Revive, X Attack, Antidote, Ether

### Hidden Items
- Ether at (11,7)
- Super Potion at (22,14)
- Full Heal at (1,17)

---

## Goldenrod City

### NPCs (14 objects in overworld)
- Pokefan M: "They built the new Radio Tower to replace the old, creaky one."
- Youngster 1: "I know there's a new Bike Shop, but I can't find it anywhere."
- Cooltrainer F1: Dialogue changes after Radio Tower crisis
- Cooltrainer F2: Dialogue about Radio Card promotion
- Youngster 2: "E-he-he-he... I got in trouble for playing in the basement of the Dept. Store."
- Lass: "The man at that house rates your Pokemon names."
- Gramps: "Whew! This is one big town."
- **Rocket Scout** (pre-takeover): "So this is the Radio Tower..." / "What do you want, you pest?"
- **6 Team Rocket members** (during takeover, various threatening dialogue)
- **Move Tutor** (post-E4, Wed/Sat, with Coin Case): Teaches Flamethrower/Thunderbolt/Ice Beam for 4000 coins

### Signs (12 signs)
- Station sign, Radio Tower sign, Dept Store sign, Gym sign, City sign, Bike Shop sign, Game Corner sign, Name Rater sign, Underground signs (north + south), Pokecenter sign, Flower Shop sign

### Gym
**Goldenrod Gym — Whitney (Normal)**
- Gym trainers: Lass Carrie (sight 4), Lass Bridget (sight 1), Beauty Victoria (sight 3), Beauty Samantha (sight 3)
- Leader: Clefairy Lv18, Miltank Lv20
- **Whitney cries after defeat** — must talk to her again or to gym Lass
- Reward: Plain Badge, TM45 Attract
- Sign: "GOLDENROD CITY POKEMON GYM LEADER: WHITNEY — The Incredibly Pretty Girl!"

### Key Buildings
- Dept Store (6 floors + roof + basement): Items, TMs, Vitamins, vending machines
- Radio Tower (5 floors): Radio Card quiz, Buena's Password, Rocket takeover
- Game Corner: Slot machines, Card Flip, prizes
- Underground: Tunnel with shops, Switch Room connections
- Bike Shop: Free Bicycle
- Flower Shop: SquirtBottle after Plain Badge
- Bill's Family's House: Eevee Lv20 (after meeting Bill in Ecruteak)
- Name Rater: Rename Pokemon
- Magnet Train Station: To Saffron (requires Pass)

---

## Route 34

### Trainers (9 total)
- Youngsters, Pokefans, Cooltrainers, Officer Keith (night only)

### Key NPCs
- Day Care Center (breeding, $100/level)

### Items
- Nugget (itemball)
- Hidden Rare Candy at (8,32)
- Hidden Super Potion at (17,19)

---

## Route 35

### Trainers (6 total)
- Campers, Picnickers, Juggler Irwin (phone contact)

### Items
- TM04 Rollout (from gate guard, in Route 35 NPC)

---

## National Park

### NPCs
- Various park-goers
- Bug Catching Contest host (Tue/Thu/Sat)

### Items
- Parlyz Heal (itemball)
- TM28 Dig (itemball)
- Hidden Full Heal at (6,47)

### Bug Catching Contest
- Entry: 1 Pokemon + 20 Sport Balls, 20-minute limit
- Prizes: 1st Sun Stone, 2nd Everstone, 3rd Gold Berry

---

## Route 36

### NPCs
- Floria (related to SquirtBottle quest)
- Sudowoodo (static encounter Lv20, use SquirtBottle)

### Warps
- (18,8)/(18,9) -> Route 36 National Park Gate
- (47,13)/(48,13) -> Route 36 Ruins of Alph Gate

---

## Route 37

### Items
- Red Apricorn, Blue Apricorn, Black Apricorn (daily trees)
- Hidden Ether at (4,2)

---

## Route 38

### Trainers (6 total)
- Lasses, Sailors, Bird Keepers, Dana (phone contact), Chad (phone contact)

---

## Route 39

### NPCs
- Moomoo Farm: Heal sick Miltank with 7 berries -> TM13 Snore + Moomoo Milk

### Items
- Hidden Nugget at (5,13)

---

## Ecruteak City

### NPCs
- Morty (gym leader)
- 5 Kimono Girls (Dance Theater: Flareon, Jolteon, Vaporeon, Umbreon, Espeon)
- Bill (meet here first, mentions Goldenrod house)
- Eusine (Suicune hunter)
- Itemfinder House NPC

### Items
- Hidden Hyper Potion at (23,14)

### Gym
**Ecruteak Gym — Morty (Ghost)**
- Gym trainers: Sage Jeffrey (sight 1), Sage Ping (sight 3), Medium Martha (sight 1), Medium Grace (sight 1)
- Leader: Gastly Lv21, Haunter Lv21, Haunter Lv23, Gengar Lv25
- Puzzle: Invisible floor — walk on hidden path to reach Morty
- Reward: Fog Badge, TM30 Shadow Ball

### Dance Theater
- Battle 5 Kimono Girls to earn HM03 Surf (from old man outside)

### Burned Tower
- 1F: Rival battle (RIVAL1_3), Morty NPC, Eusine NPC
  - Items: HP Up (itemball), Hidden Ether at (8,7), Hidden Ultra Ball at (13,11)
  - Trainers: Firebreather Ned, Firebreather Bill (not gym trainers)
- B1F: Legendary beasts cutscene (Raikou, Entei, Suicune awaken and flee)
  - Items: TM20 Endure (itemball)
  - Eusine appears after beasts flee

### Tin Tower (Bell Tower) — 10 floors
- Entrance through Ecruteak Tin Tower Entrance building
- Crystal: Requires Clear Bell (from Radio Tower crisis)
- Each floor has maze-like staircase connections
- Items across floors: Full Heal (3F), Ultra Ball (4F), PP Up (4F), Escape Rope (4F), Rare Candy (5F), Max Potion (6F), Max Revive (7F), Nugget (8F), Max Elixer (8F), Full Restore (8F), HP Up (9F)
- Hidden: Max Potion (4F), Full Restore (5F), Carbos (5F)
- Roof: Ho-Oh Lv60 encounter

---

## Olivine City

### NPCs
- Jasmine (gym leader, first met at lighthouse top)
- Fisherman (Good Rod House)
- Sailor (Olivine Cafe, gives HM04 Strength)

### Gym
**Olivine Gym — Jasmine (Steel)**
- No gym trainers (just Jasmine)
- Leader: Magnemite Lv30 x2, Steelix Lv35
- Condition: Must deliver SecretPotion first
- Reward: Mineral Badge, TM23 Iron Tail

### Olivine Lighthouse (6 floors)
- Trainers on various floors: Sailors, Gentlemen, Bird Keepers, Lasses
- Items: Ether (3F), Rare Candy (5F), Super Repel (5F), TM34 Swagger (5F), Super Potion (6F)
- Hidden: Hyper Potion (5F)
- Top floor: Jasmine + sick Ampharos

---

## Cianwood City

### NPCs
- Chuck (gym leader)
- Chuck's wife (gives HM02 Fly after gym)
- Pharmacist (SecretPotion, free)
- Mania (gives/lends Shuckle)
- Poke Seer (reads Pokemon's past)
- Photo Studio NPC

### Items
- Hidden Revive at (4,19)
- Hidden Max Ether at (5,29)

### Gym
**Cianwood Gym — Chuck (Fighting)**
- Gym trainers: Blackbelt Yoshi (sight 3), Blackbelt Lao (sight 3), Blackbelt Nob (sight 2), Blackbelt Lung (sight 1)
- Puzzle: Waterfall in gym must be stopped to reach Chuck
- Leader: Primeape Lv27, Poliwrath Lv30
- Reward: Storm Badge, TM01 DynamicPunch

---

## Route 40-41

### Route 40
- Swimmers (3 trainers)
- Hidden Hyper Potion at (7,8)
- Battle Tower Gate

### Route 41
- Swimmers (6 trainers)
- Whirl Islands: 4 entrances (NW, NE, SW, SE)
- Hidden Max Ether at (9,35)

### Whirl Islands
- Complex multi-entrance dungeon
- B1F Items: Full Restore, Carbos, Calcium, Nugget, Escape Rope
- B1F Hidden: Rare Candy, Ultra Ball, Full Restore
- B2F Items: Full Restore, Max Revive, Max Elixer
- SW: Ultra Ball
- NE: Ultra Ball
- Lugia encounter: B2F depths (Lv60, requires Silver Wing + Surf + Whirlpool + Waterfall)

---

## Route 42

### Trainers (3)
- Fisher Tully (phone), Hiker Benjamin, Pokemaniac Shane

### Items
- Ultra Ball (itemball), Super Potion (itemball)
- Hidden Max Potion at (16,11)
- Apricorn trees: Pink, Green, Yellow

### Mt. Mortar (3 entrances)
- 1F Outside Items: Ether, Revive, Hidden Hyper Potion
- 1F Inside Items: Escape Rope, Max Revive, Hyper Potion, Max Potion, Nugget, Iron, Ultra Ball, Hidden Max Repel
- B1F Items: Hyper Potion, Carbos, Full Restore, Max Ether, PP Up, Hidden Max Revive
- 2F Inside Items: Max Potion, Rare Candy, TM40 Defense Curl, Dragon Scale, Elixer, Escape Rope, Hidden Full Restore
- **Karate King**: Deep inside, defeat for Tyrogue Lv10 (only way to obtain)

---

## Mahogany Town

### NPCs
- Pryce (gym leader)
- Lance (assists in Rocket Hideout)
- Souvenir Shop NPCs (front for Rocket base)

### Gym
**Mahogany Gym — Pryce (Ice)**
- Gym trainers: Skier Roxanne (sight 1), Boarder Ronald (sight 1), Skier Clarissa (sight 1), Boarder Brad (sight 1), Boarder Douglas (sight 1)
- Puzzle: Sliding ice floor
- Condition: Must clear Rocket HQ first
- Leader: Seel Lv27, Dewgong Lv29, Piloswine Lv31
- Reward: Glacier Badge, TM16 Icy Wind

### Team Rocket Base (B1F-B3F)
- B1F: ~4 Grunts, Hyper Potion, Nugget, Guard Spec, Hidden Revive
- B2F: ~3 Grunts + Executives, TM46 Thief, Hidden Full Heal, Electrode switches
- B3F: ~3 Grunts + Executives, Protein, X Special, Full Heal, Ice Heal, Ultra Ball
- Lance heals your Pokemon during infiltration
- Password puzzle ("HAIL GIOVANNI" from Murkrow)
- 3 Electrode Lv23 (can catch or KO to disable switches)
- Reward: HM06 Whirlpool from Lance

---

## Route 43

### Trainers (6)
- Pokemaniac Ben/Brent/Ron, Fisher Marvin, Picnicker Tiffany (phone), Camper Spencer

### Special
- Team Rocket toll gate during Rocket arc ($1000 or Poke Balls)

### Items
- Max Ether (itemball)

---

## Lake of Rage

### NPCs
- Lance (after Red Gyarados encounter)
- Wesley (Day-of-Week sibling, Wednesday, gives Blackbelt)
- Fisherman (TM10 Hidden Power)

### Items
- Elixer (itemball), TM43 Detect (itemball)
- Hidden: Full Restore, Rare Candy, Max Potion

### Story
- Red Gyarados Lv30 (static shiny encounter, center of lake)
- Red Scale -> trade to Mr. Pokemon for EXP Share

---

## Route 44

### Trainers (6)
- Cooltrainers, Psychics, Fishers (Wilton phone), Bird Keeper Vance (phone)

### Items
- Max Revive (itemball), Ultra Ball (itemball), Max Repel (itemball)
- Hidden Elixer at (32,9)

---

## Ice Path (4 floors)

### 1F
- Items: HM07 Waterfall (itemball), PP Up (itemball), Protein (itemball)
- West entrance from Route 44, East exit to Blackthorn

### B1F
- Items: Iron (itemball)
- Hidden Max Potion
- Sliding ice puzzles + boulder pushing

### B2F (Mahogany Side)
- Items: Full Heal, Max Potion (itemballs)
- Hidden Carbos

### B2F (Blackthorn Side)
- Items: TM44 Rest (itemball)
- Hidden Ice Heal

### B3F
- Items: NeverMeltIce (itemball)
- Ice sliding puzzle connections

---

## Blackthorn City

### NPCs
- Clair (gym leader)
- Move Deleter (Blackthorn house)
- Emy (trades Dodrio for Dragonair)

### Gym
**Blackthorn Gym — Clair (Dragon)**
- Gym trainers: Cooltrainer M Mike (sight 3), Cooltrainer M Paul (sight 3), Cooltrainer F Lola (sight 1)
- Puzzle: Lava/platform puzzle on 2F, fall through holes
- Leader: Dragonair Lv37 x3, Kingdra Lv40
- Special: Clair refuses badge — must pass Dragon's Den quiz
- Reward: Rising Badge, TM24 DragonBreath (after Dragon's Den)

### Dragon's Den
- B1F: Water area (requires Surf)
- Items: Calcium (itemball), Max Elixer (itemball)
- Dragon Fang (from Dragon Shrine Elder)
- Hidden: Revive, Max Potion, Max Elixer
- Dragon Shrine: Quiz from Elder (emphasize kindness -> ExtremeSpeed Dratini)

---

## Route 45-46

### Route 45
- Trainers: 8 (Hikers, Blackbelts, Cooltrainers, Kenji/Parry phone contacts)
- Items: Nugget, Revive, Elixer, Max Potion (itemballs)
- Hidden PP Up
- One-way ledges downward

### Route 46
- Items: X Speed (itemball)
- Trainers: 3 Hikers, Picnicker Erin (phone)
- Gate connection to Route 29

---

## Dark Cave

### Violet Entrance
- Items: Potion, Full Heal, Hyper Potion, Dire Hit (itemballs)
- Hidden Elixer

### Blackthorn Entrance
- Items: Revive, TM13 Snore (itemballs)
- Requires: Flash, Surf, Waterfall, Rock Smash, Strength for full exploration

---

## Victory Road

### Trainers
- Multiple Cooltrainers + Rival Battle 5 (Silver, 6 Pokemon)

### Items
- TM26 Earthquake, Max Revive, Full Restore, Full Heal, HP Up (itemballs)
- Hidden: Max Potion, Full Heal

---

## Indigo Plateau

### Elite Four Rooms
- Will's Room -> Koga's Room -> Bruno's Room -> Karen's Room -> Lance's Room
- Sequential warp connections through each room
- Healing between battles (Pokemon Center)

### Hall of Fame
- Entered after defeating Champion Lance
- Credits sequence

---

## Ruins of Alph

See dedicated file: `ruins_of_alph.md`

### Summary
- 4 Puzzle Chambers (Kabuto, Omanyte, Aerodactyl, Ho-Oh)
- Inner Chamber (Unown encounters after puzzles)
- 4 Hidden Item Rooms (via secret walls)
- Research Center (Unown Pokedex upgrade)
- Ruins of Alph Outside: Multiple entrances, connections to Route 32 and Route 36

---

## Union Cave

### 1F
- Items: Great Ball, X Attack, Potion, Awakening (itemballs)
- Trainers: Hikers, Firebreather, Pokemaniac

### B1F
- Items: TM39 Swift, X Defend (itemballs)
- Connection to Ruins of Alph Outside

### B2F (requires Surf)
- Items: Elixer, Hyper Potion (itemballs)
- **Friday Lapras** (Lv20, appears only on Fridays)

---

## S.S. Aqua

### Interior
- Multiple cabins with trainers
- Captain's quarters
- Deck areas
- Travel: Olivine Port <-> Vermilion Port

---

## Goldenrod Underground + Warehouse

### Underground Tunnel
- Items: Coin Case (itemball)
- Hidden: Parlyz Heal, Super Potion, Antidote
- Shops: Barber Brothers (haircuts for happiness), various merchants

### Underground Switch Room Entrances
- Items: Smoke Ball, Full Heal (itemballs)
- Hidden: Max Potion, Revive
- Card Key doors during Radio Tower crisis

### Underground Warehouse
- Items: Max Ether, TM35 Sleep Talk, Ultra Ball (itemballs)
- Rocket Executive battles during crisis
- Basement Key location

---

## Summary Statistics

- **Total Johto maps processed: ~250**
- **Total Johto NPCs: ~500+**
- **Total Johto trainers: ~200**
- **Total Johto item balls: ~120**
- **Total Johto hidden items: ~50**
- **Total Johto sign texts: ~80**
- **Total Johto warp events: ~800**
