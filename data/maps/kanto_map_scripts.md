# Pokemon Crystal — Kanto Map Scripts Reference

> Source: pokecrystal-master/maps/*.asm — Complete Kanto map data
> Every NPC, trainer, item, warp, sign, hidden item, event trigger, and story scene

---

## Pallet Town

**Motto:** "A Tranquil Setting of Peace & Purity"

**Warps:** Red's House 1F | Blue's House | Oak's Lab

**NPCs:**
- Teacher — "I'm raising Pokemon too. They serve as my private guards."
- Fisher — "Technology is incredible! You can now trade Pokemon across time like e-mail."

**Signs:** Pallet Town | Red's House | Oak's Lab | Blue's House

**Flypoint:** ENGINE_FLYPOINT_PALLET

---

### Oak's Lab

**NPCs:**
- **Prof. Oak** — Checks badges, evaluates Pokedex. If player has all 16 badges → opens Mt. Silver (EVENT_OPENED_MT_SILVER). If only 8 Johto badges → encourages collecting Kanto badges. Runs special ProfOaksPCBoot for Pokedex evaluation.
- Scientist 1 — Prof's Pokemon Talk radio isn't aired in Kanto
- Scientist 2 — Thanks player for Pokedex work
- Scientist 3 — Pokemon Talk isn't live broadcast (secret)

**Key Events:**
- `EVENT_TALKED_TO_OAK_IN_KANTO` — Set on first visit
- `EVENT_OPENED_MT_SILVER` — Set when player has all 16 badges; unlocks Route 28 → Mt. Silver

**Interactables:** Bookshelves (DifficultBookshelfScript), posters ("Press START to open the MENU" / "The SAVE option is on the MENU"), PC (email from Prof. Elm in New Bark Town)

---

### Red's House 1F / Blue's House

Standard NPC houses with dialogue about Red/Blue's adventures. No quest-critical content.

---

## Viridian City

**Motto:** "The Eternally Green Paradise"

**Warps:** Viridian Gym | Nickname Speech House | Trainer House 1F | Viridian Mart | Viridian Pokecenter

**NPCs:**
- **Coffee Gramps** — Yes/No dialogue about being expert at catching Pokemon
- **Gramps near Gym** — Before Blue returns: "This GYM didn't have a LEADER until recently. A young man from PALLET became the LEADER, but he's often away." After EVENT_BLUE_IN_CINNABAR: "Are you going to battle the LEADER? Good luck."
- **Fisher** — Gives TM42 Dream Eater (EVENT_GOT_TM42_DREAM_EATER). Free, no prereqs.
- Youngster — Mentions items on the ground in Viridian Forest

**Signs:** Viridian City | Viridian Gym | Welcome to Viridian City | Trainer House | Pokecenter | Mart

**Flypoint:** ENGINE_FLYPOINT_VIRIDIAN

---

### Viridian Gym

**Leader:** Blue (EARTHBADGE)
- Badge flag: ENGINE_EARTHBADGE
- Trainer: loadtrainer BLUE, BLUE1
- Event: EVENT_BEAT_BLUE
- No gym trainers (just Blue and Gym Guide)
- Blue only appears after EVENT_BLUE_IN_CINNABAR is cleared (player talks to Blue at Cinnabar first)
- Both Blue and Gym Guide controlled by EVENT_VIRIDIAN_GYM_BLUE flag

**Gym Guide:** "The GYM LEADER is a guy who battled the CHAMPION three years ago."

**Blue's Dialogue:** "Yo! Finally got here, huh? ...You're telling me you conquered all the GYMS in JOHTO? Heh! JOHTO's GYMS must be pretty pathetic then."

**Blue's Win Text:** "What? How the heck did I lose to you? ...Tch, all right... Here, take this--it's EARTHBADGE."

---

### Trainer House 1F

Battle facility where player can fight NPC trainer or Mystery Gift opponent. Not a standard gym.

---

## Pewter City

**Motto:** "A Stone Gray City"

**Warps:** Nidoran Speech House | Pewter Gym | Pewter Mart | Pewter Pokecenter | Snooze Speech House

**NPCs:**
- CooltrainerF — "Have you visited PEWTER GYM? The LEADER uses rock-type Pokemon."
- Bug Catcher — "At night, CLEFAIRY come out to play at MT.MOON. But not every night."
- **Gramps** — Gives SILVER_WING (EVENT_GOT_SILVER_WING). "Ah, you came all the way out here from JOHTO? ...Here. I want you to have this item I found in JOHTO." (Enables Lugia encounter at Whirl Islands)
- 2 Fruit Trees (FRUITTREE_PEWTER_CITY_1, FRUITTREE_PEWTER_CITY_2)

**Signs:** Pewter City | Pewter Gym | Pewter Museum (closed for renovations) | Mt. Moon Gift Shop | Welcome to Pewter City | Pokecenter | Mart

**Flypoint:** ENGINE_FLYPOINT_PEWTER

---

### Pewter Gym

**Leader:** Brock (BOULDERBADGE)
- Badge flag: ENGINE_BOULDERBADGE
- Trainer: loadtrainer BROCK, BROCK1
- Event: EVENT_BEAT_BROCK
- Auto-defeats gym trainer when leader beaten: EVENT_BEAT_CAMPER_JERRY

**Gym Trainers:**
| Trainer | Class | Sight |
|---------|-------|-------|
| Jerry | Camper | 3 |

**Brock's Intro:** "Wow, it's not often that we get a challenger from JOHTO. I'm BROCK, the PEWTER GYM LEADER. I'm an expert on rock-type Pokemon."

---

## Cerulean City

**Motto:** "A Mysterious Blue Aura Surrounds It"

**Warps:** Gym Badge Speech House | Police Station | Trade Speech House | Cerulean Pokecenter | Cerulean Gym | Cerulean Mart

**NPCs:**
- CooltrainerM — Before Machine Part returned: mentions Power Plant accident. After: talks about Pokemon collecting.
- Super Nerd — "The CAPE in the north is a good place for dates."
- CooltrainerF + Slowbro — Shows off Slowbro's Confusion
- **Fisher** — Before Machine Part: "I'm a huge fan of CERULEAN GYM's MISTY." After meeting Rocket at gym: "I saw this shady guy go off toward CERULEAN's CAPE."
- **Youngster** — Hints at Berserk Gene with Itemfinder sounds if not yet found (EVENT_FOUND_BERSERK_GENE_IN_CERULEAN_CITY). Mentions old cave (Cerulean Cave, blocked in Crystal).

**Hidden Items:**
- Berserk Gene at (2, 12) — unique item

**Signs:** Cerulean City | Cerulean Gym | Bike Shop (moved to Goldenrod) | Police Station | Cerulean Cape | Locked Door | Pokecenter | Mart

**Flypoint:** ENGINE_FLYPOINT_CERULEAN

---

### Cerulean Gym

**Leader:** Misty (CASCADEBADGE)
- Badge flag: ENGINE_CASCADEBADGE
- Trainer: loadtrainer MISTY, MISTY1
- Event: EVENT_BEAT_MISTY
- Auto-defeats gym trainers: EVENT_BEAT_SWIMMERF_DIANA, EVENT_BEAT_SWIMMERF_BRIANA, EVENT_BEAT_SWIMMERM_PARKER

**Gym Trainers:**
| Trainer | Class | Sight |
|---------|-------|-------|
| Diana | SwimmerF | 3 |
| Briana | SwimmerF | 1 |
| Parker | SwimmerM | 3 |

**Scene Scripts:**
- SCENE_CERULEANGYM_GRUNT_RUNS_OUT — Rocket Grunt cutscene:
  - Grunt runs down, bumps into player, says broken English ("Oops! I so sorry!"), panics, flees
  - Sets EVENT_MET_ROCKET_GRUNT_AT_CERULEAN_GYM
  - Clears EVENT_ROUTE_24_ROCKET, EVENT_ROUTE_25_MISTY_BOYFRIEND (enables Misty/Rocket on Route 25)
  - Sets SCENE_ROUTE25 to SCENE_ROUTE25_MISTYS_DATE
  - Sets SCENE_POWERPLANT to SCENE_POWERPLANT_NOOP

**Hidden Items:**
- Machine Part at (3, 8) — quest-critical item for Power Plant restoration

**Misty's Intro:** "I was expecting you, you pest! You may have a lot of JOHTO GYM BADGES, but you'd better not take me too lightly."

---

## Vermilion City

**Motto:** "The Port of Exquisite Sunsets"

**Warps:** Fishing Speech House | Vermilion Pokecenter | Pokemon Fan Club | Magnet Train Speech House | Vermilion Mart | Diglett's Cave Speech House | Vermilion Gym | Vermilion Port Passage (x2) | Diglett's Cave

**NPCs:**
- Teacher — Describes Vermilion Port as Kanto's seaside gateway
- **Gramps + Machop** — Machop stomps ground flat (earthquake effect). Owner has no money for construction.
- Super Nerd — Points out Vermilion Gym
- **Snorlax** — Level 50 wild battle (BATTLETYPE_FORCEITEM). Requires PokeFlute channel on radio (special SnorlaxAwake check). EVENT_FOUGHT_SNORLAX / EVENT_VERMILION_CITY_SNORLAX. Disappears after battle.
- **Badge Guy (Pokefan M)** — Gives HP_UP when player has all 16 badges (EVENT_GOT_HP_UP_FROM_VERMILION_GUY). Progressive dialogue based on badge count.

**Hidden Items:**
- Full Heal at (12, 19)

**Signs:** Vermilion City | Vermilion Gym | Pokemon Fan Club | Diglett's Cave | Vermilion Port | Pokecenter | Mart

**Flypoint:** ENGINE_FLYPOINT_VERMILION

---

### Vermilion Gym

**Leader:** Lt. Surge (THUNDERBADGE)
- Badge flag: ENGINE_THUNDERBADGE
- Trainer: loadtrainer LT_SURGE, LT_SURGE1
- Event: EVENT_BEAT_LTSURGE
- Auto-defeats gym trainers: EVENT_BEAT_GENTLEMAN_GREGORY, EVENT_BEAT_GUITARIST_VINCENT, EVENT_BEAT_JUGGLER_HORTON

**Gym Trainers:**
| Trainer | Class | Sight |
|---------|-------|-------|
| Gregory | Gentleman | 4 |
| Vincent | Guitarist | 3 |
| Horton | Juggler | 4 |

**Gym Puzzle:** 15 trash can bg_events (3x5 grid). In Gen 2 Crystal, traps are not active (Gym Guide confirms: "the traps aren't active right now").

**Surge's Intro:** "Hey, you little tyke! ...When it comes to electric Pokemon, I'm number one! I've never lost on the battlefield. I'll zap you just like I did my enemies in war!"

---

### Vermilion Port

Entrance to S.S. Aqua. Hidden item: Iron at (16, 13).

---

## Lavender Town

**Motto:** "The Noble Purple Town"

**Warps:** Lavender Pokecenter | Mr. Fuji's House | Lavender Speech House | Lavender Name Rater | Lavender Mart | Soul House | Lav Radio Tower 1F

**NPCs:**
- Pokefan M — Points out Kanto Radio Tower
- Teacher — "KANTO has many good radio shows."
- Gramps — People pay respects to departed Pokemon souls
- Youngster — "You need a POKE FLUTE to wake sleeping Pokemon."

**Signs:** Lavender Town | Kanto Radio Station | Volunteer Pokemon House | Soul House | Pokecenter | Mart

**Key Building:** Kanto Radio Tower (Lav Radio Tower 1F) — Player can receive EXPN Card here to upgrade PokeGear radio (enables PokeFlute channel needed for Snorlax).

**Flypoint:** ENGINE_FLYPOINT_LAVENDER

---

### Lavender Name Rater

Renames player's Pokemon. Same as Goldenrod's Name Rater.

---

## Celadon City

**Motto:** "The City of Rainbow Dreams"

**Warps:** Celadon Dept. Store 1F | Celadon Mansion 1F (front door) | Celadon Mansion 1F (back door, x2) | Celadon Pokecenter | Celadon Game Corner | Game Corner Prize Room | Celadon Gym | Celadon Cafe

**NPCs:**
- Fisher + Poliwrath — "This POLIWRATH is my partner."
- Teacher 1 — Lost at slot machines (AU version: shorter dialogue)
- Gramps 1 — Grimer appearing in pond
- Gramps 2 — "This GYM is great! Only girls are allowed here!"
- Youngster 1 — "CELADON MANSION has a hidden back door."
- Youngster 2 — Eating contest at restaurant
- Teacher 2 — Celadon Dept. Store has biggest selection
- Lass — Generic walking dialogue

**Hidden Items:**
- PP Up at (37, 21)

**Signs:** Celadon City | Celadon Gym | Dept. Store | Celadon Mansion | Game Corner | Trainer Tips (Guard Spec.) | Pokecenter

**Flypoint:** ENGINE_FLYPOINT_CELADON

---

### Celadon Gym

**Leader:** Erika (RAINBOWBADGE)
- Badge flag: ENGINE_RAINBOWBADGE
- Trainer: loadtrainer ERIKA, ERIKA1
- Event: EVENT_BEAT_ERIKA
- **TM reward:** TM19 Giga Drain (EVENT_GOT_TM19_GIGA_DRAIN)
- Auto-defeats gym trainers: EVENT_BEAT_LASS_MICHELLE, EVENT_BEAT_PICNICKER_TANYA, EVENT_BEAT_BEAUTY_JULIA, EVENT_BEAT_TWINS_JO_AND_ZOE

**Gym Trainers:**
| Trainer | Class | Sight |
|---------|-------|-------|
| Michelle | Lass | 2 |
| Tanya | Picnicker | 2 |
| Julia | Beauty | 2 |
| Jo & Zoe (1) | Twins | 1 |
| Jo & Zoe (2) | Twins | 1 |

**Erika's Intro:** "Hello... Lovely weather, isn't it? ...I'm afraid I may doze off... ...Oh? All the way from JOHTO, you say?"

---

### Celadon Dept. Store (Floors 1-6 + Elevator)

Major shopping center. Sells TMs, evolution stones, vitamins, etc. Rooftop vending machines.

### Celadon Game Corner + Prize Room

Slot machines. Prize Room exchanges coins for Pokemon/TMs. AU version has modified dialogue.

### Celadon Mansion (Floors 1-3 + Roof + Roof House)

Back entrance accessible. Contains NPCs with lore.

---

## Saffron City

**Motto:** "Shining, Golden Land of Commerce"

**Warps:** Fighting Dojo | Saffron Gym | Saffron Mart | Saffron Pokecenter | Mr. Psychic's House | Saffron Magnet Train Station | Silph Co. 1F | Copycat's House 1F | Route 5/6/7/8 gates (6 gate warps total)

**NPCs:**
- Lass 1 — Before Machine Part: Copycat girl mimics people. After: Copycat lost her Poke Doll (Clefairy).
- Pokefan M — Before: Magnet Train might not be running. After: Can zip home on Magnet Train.
- CooltrainerM — Accidentally went to Fighting Dojo instead of Gym
- CooltrainerF — Silph Co. history with Team Rocket
- Fisher — Before: Trouble at Power Plant. After: Trouble resolved.
- Youngster 1 — Anxious about alleys
- Youngster 2 — Trainer House in Viridian
- Lass 2 — City featured on radio

**Signs:** Saffron City | Saffron Gym | Fighting Dojo | Silph Co. | Mr. Psychic's House | Magnet Train Station | Pokecenter | Mart

**Flypoint:** ENGINE_FLYPOINT_SAFFRON

---

### Saffron Gym

**Leader:** Sabrina (MARSHBADGE)
- Badge flag: ENGINE_MARSHBADGE
- Trainer: loadtrainer SABRINA, SABRINA1
- Event: EVENT_BEAT_SABRINA
- Auto-defeats gym trainers: EVENT_BEAT_MEDIUM_REBECCA, EVENT_BEAT_MEDIUM_DORIS, EVENT_BEAT_PSYCHIC_FRANKLIN, EVENT_BEAT_PSYCHIC_JARED
- **Teleport pad puzzle:** 32 warp_events (internal gym teleporters)

**Gym Trainers:**
| Trainer | Class | Sight |
|---------|-------|-------|
| Rebecca | Medium | 3 |
| Franklin | Psychic | 3 |
| Doris | Medium | 2 |
| Jared | Psychic | 2 |

**Sabrina's Intro:** "I knew you were coming... Three years ago I had a vision of your arrival."

---

### Fighting Dojo

- Black Belt NPC — Karate King is training in a cave in Johto (Mt. Mortar)
- **Focus Band** — Itemball pickup (EVENT_PICKED_UP_FOCUS_BAND)
- Signs: "What goes around comes around!" / "Enemies on every side!"

---

### Copycat's House (1F + 2F)

**Copycat Sidequest:**
- After EVENT_RETURNED_MACHINE_PART, Copycat lost her Poke Doll (Clefairy)
- Player gets Lost Item from Vermilion Pokemon Fan Club
- Return to Copycat → receive Pass (Magnet Train Pass)
- Parents: Pokefan M ("My daughter likes to mimic people") and Pokefan F
- Blissey pet on 1F

---

### Silph Co. 1F

Limited access in Crystal — just lobby with receptionist. Not the full dungeon from Gen 1.

---

### Saffron Magnet Train Station

Connects Saffron ↔ Goldenrod. Requires Pass (from Copycat quest) and EVENT_RESTORED_POWER_TO_KANTO.

---

## Fuchsia City

**Motto:** "Behold! It's Passion Pink!"

**Warps:** Fuchsia Mart | Safari Zone Main Office | Fuchsia Gym | Bill's Older Sister's House | Fuchsia Pokecenter | Safari Zone Warden's Home | Safari Zone Gate (beta/inaccessible) | Route 15 Gate | Route 19 Gate

**NPCs:**
- Youngster — "One of the ELITE FOUR used to be the LEADER of FUCHSIA's GYM." (Koga)
- Pokefan M — "KOGA's daughter succeeded him as the GYM LEADER."
- Teacher — Safari Zone is closed
- Fruit Tree (FRUITTREE_FUCHSIA_CITY)

**Signs:** Fuchsia City | Fuchsia Gym | Safari Zone Office (closed) | Warden's Home | Safari Zone Closed (Warden traveling abroad) | No Littering | Pokecenter | Mart

**Flypoint:** ENGINE_FLYPOINT_FUCHSIA

---

### Fuchsia Gym

**Leader:** Janine (SOULBADGE)
- Badge flag: ENGINE_SOULBADGE
- Trainer: loadtrainer JANINE, JANINE1
- Event: EVENT_BEAT_JANINE
- **TM reward:** TM06 Toxic (EVENT_GOT_TM06_TOXIC)
- Auto-defeats gym trainers: EVENT_BEAT_LASS_ALICE, EVENT_BEAT_LASS_LINDA, EVENT_BEAT_PICNICKER_CINDY, EVENT_BEAT_CAMPER_BARRY

**Gym Gimmick:** All 4 gym trainers disguised as Janine (using SPRITE_JANINE / variable sprites). Each spins when interacted with, then reveals true identity:
- SPRITE_FUCHSIA_GYM_1 → Lass Alice
- SPRITE_FUCHSIA_GYM_2 → Lass Linda
- SPRITE_FUCHSIA_GYM_3 → Picnicker Cindy
- SPRITE_FUCHSIA_GYM_4 → Camper Barry

After each battle, sprite changes via `variablesprite` to show real identity. Movement: SPRITEMOVEDATA_SPINRANDOM_FAST for all disguised trainers.

**Janine's Intro:** "Fufufufu... I'm sorry to disappoint you... I'm only joking! I'm the real deal! JANINE of FUCHSIA GYM, that's me!"

---

### Bill's Older Sister's House

Teaches moves to player's Pokemon (move deletion/relearning service in some versions).

### Safari Zone Warden's Home

Warden is away. Contains lore items.

---

## Cinnabar Island

**Motto:** "The Fiery Town of Burning Desire"

**Warps:** Cinnabar Pokecenter (only building — rest destroyed by volcano)

**NPCs:**
- **Blue** — Appears only when EVENT_BLUE_IN_CINNABAR is set. Long dialogue about volcano destroying the town, then tells player to come to Viridian Gym. Uses teleport animation to leave. Clears EVENT_VIRIDIAN_GYM_BLUE (makes Blue + Gym Guide appear in Viridian Gym).

**Blue's Dialogue:** "My name's BLUE. I was once the CHAMPION, although it was for only a short time... That meddling RED did me in... A volcano erupts, and just like that, a whole town disappears... If you want to battle me, come to the VIRIDIAN GYM."

**Hidden Items:**
- Rare Candy at (9, 1)

**Signs:** Cinnabar Pokecenter | Cinnabar Gym (notice: relocated to Seafoam Islands — BLAINE) | Cinnabar Island

**Flypoint:** ENGINE_FLYPOINT_CINNABAR

---

### Seafoam Gym (Blaine's relocated gym)

**Leader:** Blaine (VOLCANOBADGE)
- Badge flag: ENGINE_VOLCANOBADGE
- Trainer: loadtrainer BLAINE, BLAINE1
- Event: EVENT_BEAT_BLAINE
- No gym trainers — just Blaine in a cave
- Gym Guide appears after battle (EVENT_SEAFOAM_GYM_GYM_GUIDE)
- Location: Inside Seafoam Islands cave, accessed from Route 20

**Blaine's Intro:** "Waaah! My GYM in CINNABAR burned down. My fire-breathing Pokemon and I are homeless because of the volcano."

---

## Power Plant

**Location:** End of Route 9/10

**NPCs:**
- Officer 1 — Reports theft, later gets phone call about Cerulean suspect
- Gym Guide 1 — Stolen part essential for generator
- Gym Guide 2 — Power Plant history (was abandoned, rebuilt for Magnet Train)
- Officer 2 — Manager's emotional state
- Gym Guide 3 — Magnet Train needs electricity
- **Manager** — Quest-critical NPC:
  - First visit: Sets EVENT_MET_MANAGER_AT_POWER_PLANT, triggers Cerulean Gym Rocket scene
  - With Machine Part: Takes MACHINE_PART, sets EVENT_RETURNED_MACHINE_PART, enables Magnet Train (EVENT_RESTORED_POWER_TO_KANTO, clears EVENT_SAFFRON_TRAIN_STATION_POPULATION, clears EVENT_GOLDENROD_TRAIN_STATION_GENTLEMAN)
  - **TM reward:** TM07 Zap Cannon (EVENT_GOT_TM07_ZAP_CANNON)
- **Forest** — In-game trade NPC (NPC_TRADE_FOREST)

**Scene Scripts:**
- SCENE_POWERPLANT_GUARD_GETS_PHONE_CALL — Officer gets phone call about Cerulean suspect. coord_event at (5, 12). Triggers cutscene where officer runs to colleagues, reports, then asks player for cooperation.

**Key Event Chain:**
1. Visit Power Plant → meet Manager → EVENT_MET_MANAGER_AT_POWER_PLANT
2. Manager clears EVENT_CERULEAN_GYM_ROCKET + EVENT_FOUND_MACHINE_PART_IN_CERULEAN_GYM
3. Sets Cerulean Gym scene to SCENE_CERULEANGYM_GRUNT_RUNS_OUT
4. Player goes to Cerulean Gym → Rocket Grunt scene
5. Grunt flees → sets EVENT_MET_ROCKET_GRUNT_AT_CERULEAN_GYM
6. Player finds Machine Part hidden in Cerulean Gym (3, 8)
7. Returns Machine Part to Manager → EVENT_RETURNED_MACHINE_PART
8. Magnet Train enabled, Route 5/6 underground path changes

---

## Mt. Moon / Mt. Moon Square

### Mt. Moon

**Warps:** Route 3 ↔ Route 4 (through cave), internal floor connections, Mt. Moon Square (x2)

**Story Event:** Rival battle (SCENE_MOUNTMOON_RIVAL_BATTLE)
- Rival appears, walks toward player
- Music: MUSIC_RIVAL_ENCOUNTER
- Battle: loadtrainer RIVAL2, RIVAL2_1_TOTODILE/CHIKORITA/CYNDAQUIL (based on starter)
- After: EVENT_BEAT_RIVAL_IN_MT_MOON
- Rival leaves with MUSIC_RIVAL_AFTER

### Mt. Moon Square

**Clefairy Dance Event:** (Monday night only)
- `checkflag ENGINE_MT_MOON_SQUARE_CLEFAIRY` — only once per week
- `readvar VAR_WEEKDAY` must be MONDAY
- `checktime NITE` must be true
- Two Clefairy appear, dance around (elaborate choreography with follow/stopfollow)
- Notice player → flee
- Leaves behind a Moon Stone (hidden item at 7,7)
- Sets ENGINE_MT_MOON_SQUARE_CLEFAIRY (resets weekly)

**Hidden Items:**
- Moon Stone at (7, 7) — regenerates weekly after Clefairy dance

**Other:** Smashable rock, Mt. Moon Gift Shop entrance

---

## Rock Tunnel (1F + B1F)

**Warps:** Route 9 ↔ Route 10 South, internal connections (6 warps on 1F)

**Items (1F):**
- Elixer (itemball)
- TM Steel Wing (itemball)

**Hidden Items (1F):**
- X Accuracy at (24, 4)
- X Defend at (21, 15)

**Hidden Items (B1F):**
- Max Potion at (4, 14)

No trainers in the cave itself (trainers are on Route 9 and Route 10).

---

## Diglett's Cave

**Warps:** Vermilion City ↔ Route 2 (with internal connections)

**NPCs:**
- Pokefan M — "A bunch of DIGLETT popped out of the ground! That was shocking."

**Hidden Items:**
- Max Revive at (6, 11)

---

## Mt. Silver / Silver Cave

### Silver Cave Outside

**Warps:** Silver Cave Pokecenter | Silver Cave Room 1

**Hidden Items:**
- Full Restore at (9, 25)

**Signs:** Pokecenter | MT.SILVER

**Flypoint:** ENGINE_FLYPOINT_SILVER_CAVE

### Silver Cave Room 1

**Warps:** Silver Cave Outside ↔ Silver Cave Room 2

**Items:**
- Max Elixer (itemball)
- Protein (itemball)
- Escape Rope (itemball)
- Ultra Ball (itemball)

**Hidden Items:**
- Dire Hit at (16, 23)
- Ultra Ball at (17, 12)

### Silver Cave Room 2

**Warps:** Silver Cave Room 1 ↔ Silver Cave Room 3, Silver Cave Item Rooms (x2)

**Items:**
- Calcium (itemball)
- Ultra Ball (itemball)
- PP Up (itemball)

**Hidden Items:**
- Max Potion at (14, 31)

### Silver Cave Room 3 (Summit)

**Warps:** Silver Cave Room 2

**The Final Battle: Trainer Red**
- `object_event 9, 10, SPRITE_RED` — controlled by EVENT_RED_IN_MT_SILVER
- Music fades out (special FadeOutMusic)
- Red's dialogue: "......" (silent — just ellipses)
- Battle: loadtrainer RED, RED1
- Win/loss text: "..."
- After battle: Red disappears, screen fades to black, party healed (special HealParty), credits roll

**Red's Team (hardcoded in trainer data):**
- Pikachu Lv81, Espeon Lv73, Snorlax Lv75, Venusaur Lv77, Charizard Lv77, Blastoise Lv77

---

## Kanto Routes

### Route 1 (Pallet → Viridian)

**Trainers:** Schoolboy Danny, CooltrainerF Quinn (2 trainers)
**Items:** Fruit Tree (FRUITTREE_ROUTE_1)

### Route 2 (Viridian → Pewter, Viridian Forest area)

**Trainers:** Bug Catcher Rob, Bug Catcher Ed, Bug Catcher Doug (3 trainers)
**Items:** Dire Hit, Max Potion, Carbos, Elixer (all itemballs), Fruit Tree (FRUITTREE_ROUTE_2)
**Hidden Items:** Max Ether at (7, 23), Full Heal at (4, 14), Full Restore at (4, 27), Revive at (11, 30)
**Connection:** Diglett's Cave entrance

### Route 3 (Pewter → Mt. Moon)

**Trainers:** Firebreather Otis, Youngster Warren, Youngster Jimmy, Firebreather Burt (4 trainers)
**Connection:** Mt. Moon entrance

### Route 4 (Mt. Moon → Cerulean)

**Trainers:** Bird Keeper Hank, Picnicker Hope, Picnicker Sharon (3 trainers)
**Items:** HP Up (itemball)
**Hidden Items:** Ultra Ball at (10, 3)

### Route 5 (Cerulean → Saffron, north gate)

No trainers. Gate to Saffron (may be blocked early).
**Underground Path entrance** — blocked by Pokefan M until Power Plant quest complete (EVENT_ROUTE_5_6_POKEFAN_M_BLOCKS_UNDERGROUND_PATH)

### Route 6 (Saffron → Vermilion, south gate)

**Trainers:** Pokefan M Rex, Pokefan M Allan (2 trainers)
**Underground Path entrance** — same blocking flag as Route 5

### Route 7 (Saffron → Celadon, west gate)

No trainers. Gate passage.

### Route 8 (Saffron → Lavender, east gate)

**Trainers:** Biker Dwayne, Biker Harris, Biker Zeke, Super Nerd Sam, Super Nerd Tom (5 trainers)
**Items:** Fruit Tree (FRUITTREE_ROUTE_8)

### Route 9 (Cerulean → Rock Tunnel / Power Plant)

**Trainers:** Camper Dean, Picnicker Heidi, Camper Sid, Picnicker Edna, Hiker Tim, Hiker Sidney (6 trainers)
**Hidden Items:** Ether at (41, 15)

### Route 10 (Rock Tunnel → Lavender area)

**Route 10 South — Trainers:** Hiker Jim, Pokefan M Robert (2 trainers)
**Route 10 North** — Power Plant entrance

### Route 11 (Vermilion → east)

**Trainers:** Youngster Owen, Youngster Jason, Psychic Herman, Psychic Fidel (4 trainers)
**Items:** Fruit Tree (FRUITTREE_ROUTE_11)
**Hidden Items:** Revive at (32, 5)

### Route 12 (South of Lavender, Fishing Guru area)

**Trainers:** Fisher Kyle, Fisher Martin, Fisher Stephen, Fisher Barney (4 trainers)
**Items:** Calcium (itemball), Nugget (itemball)
**Hidden Items:** Elixer at (14, 13)

### Route 13 (South of Route 12)

**Trainers:** Pokefan M Alex, Pokefan M Joshua, Bird Keeper Perry, Bird Keeper Bret, Hiker Kenny (5 trainers)
**Hidden Items:** Calcium at (30, 13)

### Route 14 (South of Route 13)

**Trainers:** Pokefan M Carter, Bird Keeper Roy, Pokefan M Trevor (3 trainers)
**Special:** In-game trade NPC — Kim (NPC_TRADE_KIM)

### Route 15 (West toward Fuchsia)

**Trainers:** Teacher Colette, Teacher Hillary, Schoolboy Kipp, Schoolboy Tommy, Schoolboy Johnny, Schoolboy Billy (6 trainers)
**Items:** PP Up (itemball)

### Route 16 (West of Celadon, Cycling Road north entrance)

No trainers on this section.

### Route 17 (Cycling Road — Celadon to Fuchsia)

**Trainers:** Biker Charles, Biker Riley, Biker Joel, Biker Glenn (4 trainers)
**Hidden Items:** Max Ether at (9, 54), Max Elixer at (8, 77)

### Route 18 (Cycling Road south exit → Fuchsia)

**Trainers:** Bird Keeper Boris, Bird Keeper Bob (2 trainers)

### Route 19 (Fuchsia → south, water route)

**Trainers:** SwimmerF Dawn, SwimmerM Harold, SwimmerM Jerome, SwimmerM Tucker (4 trainers)

### Route 20 (Water route — Seafoam Islands)

**Trainers:** SwimmerF Nicole, SwimmerF Lori, SwimmerM Cameron (3 trainers)
**Connection:** Seafoam Gym (Blaine's relocated gym)

### Route 21 (Pallet → Cinnabar, water route)

**Trainers:** SwimmerM Seth, SwimmerF Nikki, Fisher Arnold (3 trainers)

### Route 24 (Cerulean → north, Nugget Bridge)

No trainers in Crystal (bridge trainers from Gen 1 removed).
**Rocket Grunt** — appears during Machine Part quest (EVENT_ROUTE_24_ROCKET)

### Route 25 (East of Route 24, Bill's Sea Cottage area)

**Trainers:** Schoolboy Dudley, Lass Ellen, Schoolboy Joe, Lass Laura, Camper Lloyd, Lass Shannon, Super Nerd Pat (7 trainers)
**Items:** Protein (itemball)
**Special:** Nugget given by NPC (verbosegiveitem NUGGET)
**Story:** Misty's date scene — Misty and boyfriend appear here (EVENT_ROUTE_25_MISTY_BOYFRIEND). After player interrupts, Misty returns to gym.

### Route 26 (Tohjo Falls → Indigo Plateau)

**Trainers:** CooltrainerM Jake, CooltrainerM Gaven, CooltrainerF Joyce, CooltrainerF Beth, Psychic Richard, Fisher Scott (6 trainers)
**Items:** Max Elixer (itemball), Fruit Tree (FRUITTREE_ROUTE_26)

### Route 27 (New Bark → Tohjo Falls)

**Trainers:** Psychic Gilbert, Bird Keeper Jose, CooltrainerM Blake, CooltrainerM Brian, CooltrainerF Reena, CooltrainerF Megan (6 trainers)
**Items:** TM Solarbeam (itemball), Rare Candy (itemball)
**Special:** Star Piece given by NPC (verbosegiveitem STAR_PIECE)

### Route 28 (Indigo Plateau → Mt. Silver)

No trainers. Short route to Mt. Silver entrance.
**Hidden Items:** Rare Candy at (25, 2)
**Access:** Requires EVENT_OPENED_MT_SILVER (all 16 badges, talk to Oak)

---

## Underground Path (Kanto)

Connects Route 5 ↔ Route 6 (under Saffron City). Blocked until Power Plant quest complete.

**Hidden Items:**
- Full Restore at (3, 9)
- X Special at (1, 19)

---

## Kanto Gym Summary

| Gym | Leader | Badge | Badge Flag | TM Reward | Trainers |
|-----|--------|-------|------------|-----------|----------|
| Pewter | Brock | Boulder | ENGINE_BOULDERBADGE | *(none)* | 1 (Camper Jerry) |
| Cerulean | Misty | Cascade | ENGINE_CASCADEBADGE | *(none)* | 3 (2 SwimmerF + 1 SwimmerM) |
| Vermilion | Lt. Surge | Thunder | ENGINE_THUNDERBADGE | *(none)* | 3 (Gentleman + Guitarist + Juggler) |
| Celadon | Erika | Rainbow | ENGINE_RAINBOWBADGE | TM19 Giga Drain | 5 (Lass + Picnicker + Beauty + 2 Twins) |
| Saffron | Sabrina | Marsh | ENGINE_MARSHBADGE | *(none)* | 4 (2 Medium + 2 Psychic) |
| Fuchsia | Janine | Soul | ENGINE_SOULBADGE | TM06 Toxic | 4 (2 Lass + Picnicker + Camper) |
| Seafoam | Blaine | Volcano | ENGINE_VOLCANOBADGE | *(none)* | 0 |
| Viridian | Blue | Earth | ENGINE_EARTHBADGE | *(none)* | 0 |

---

## Kanto Route Trainer Totals

| Route | Trainers | Notable |
|-------|----------|---------|
| Route 1 | 2 | Schoolboy + CooltrainerF |
| Route 2 | 3 | 3 Bug Catchers |
| Route 3 | 4 | 2 Firebreathers + 2 Youngsters |
| Route 4 | 3 | Bird Keeper + 2 Picnickers |
| Route 5 | 0 | — |
| Route 6 | 2 | 2 Pokefan M |
| Route 7 | 0 | — |
| Route 8 | 5 | 3 Bikers + 2 Super Nerds |
| Route 9 | 6 | 2 Campers + 2 Picnickers + 2 Hikers |
| Route 10S | 2 | Hiker + Pokefan M |
| Route 11 | 4 | 2 Youngsters + 2 Psychics |
| Route 12 | 4 | 4 Fishers |
| Route 13 | 5 | 2 Pokefan M + 2 Bird Keepers + Hiker |
| Route 14 | 3 | 2 Pokefan M + Bird Keeper |
| Route 15 | 6 | 2 Teachers + 4 Schoolboys |
| Route 16 | 0 | — |
| Route 17 | 4 | 4 Bikers |
| Route 18 | 2 | 2 Bird Keepers |
| Route 19 | 4 | 1 SwimmerF + 3 SwimmerM |
| Route 20 | 3 | 2 SwimmerF + 1 SwimmerM |
| Route 21 | 3 | 1 SwimmerM + 1 SwimmerF + 1 Fisher |
| Route 24 | 0 | Rocket Grunt (event) |
| Route 25 | 7 | 3 Schoolboys + 3 Lasses + Super Nerd + Camper |
| Route 26 | 6 | 2 CooltrainerM + 2 CooltrainerF + Psychic + Fisher |
| Route 27 | 6 | 2 CooltrainerM + 2 CooltrainerF + Psychic + Bird Keeper |
| Route 28 | 0 | — |
| **Total** | **84** | |

---

## Key Kanto Event Flags

| Flag | Purpose |
|------|---------|
| EVENT_BLUE_IN_CINNABAR | Blue appears at Cinnabar; cleared when he leaves → enables Viridian Gym |
| EVENT_VIRIDIAN_GYM_BLUE | Controls Blue + Gym Guide visibility in Viridian Gym |
| EVENT_MET_MANAGER_AT_POWER_PLANT | Triggers Cerulean Gym Rocket scene |
| EVENT_CERULEAN_GYM_ROCKET | Rocket Grunt appears in Cerulean Gym |
| EVENT_MET_ROCKET_GRUNT_AT_CERULEAN_GYM | Grunt scene completed; enables Route 25 Misty |
| EVENT_FOUND_MACHINE_PART_IN_CERULEAN_GYM | Machine Part hidden item available |
| EVENT_RETURNED_MACHINE_PART | Machine Part returned to Manager; enables Magnet Train |
| EVENT_RESTORED_POWER_TO_KANTO | Power fully restored; Magnet Train operational |
| EVENT_SAFFRON_TRAIN_STATION_POPULATION | Controls Magnet Train station NPCs |
| EVENT_ROUTE_5_6_POKEFAN_M_BLOCKS_UNDERGROUND_PATH | Underground Path blocked |
| EVENT_GOT_SILVER_WING | Silver Wing from Pewter gramps (Lugia access) |
| EVENT_OPENED_MT_SILVER | Oak opens Mt. Silver when all 16 badges collected |
| EVENT_RED_IN_MT_SILVER | Red appears at summit |
| EVENT_FOUGHT_SNORLAX | Vermilion Snorlax battle completed |
| EVENT_VERMILION_CITY_SNORLAX | Controls Snorlax sprite visibility |
| ENGINE_MT_MOON_SQUARE_CLEFAIRY | Weekly Clefairy dance flag |
| EVENT_BEAT_RIVAL_IN_MT_MOON | Rival battle in Mt. Moon completed |
| EVENT_TALKED_TO_OAK_IN_KANTO | First visit to Oak's Lab in Kanto |
| EVENT_GOT_TM42_DREAM_EATER | TM Dream Eater from Viridian Fisher |
| EVENT_GOT_TM07_ZAP_CANNON | TM Zap Cannon from Power Plant Manager |
| EVENT_GOT_TM19_GIGA_DRAIN | TM Giga Drain from Erika |
| EVENT_GOT_TM06_TOXIC | TM Toxic from Janine |
| EVENT_GOT_HP_UP_FROM_VERMILION_GUY | HP Up reward for all 16 badges |
| EVENT_FOUND_BERSERK_GENE_IN_CERULEAN_CITY | Unique hidden item |

---

## Summary Statistics

- **Kanto cities/towns:** 10 (Pallet, Viridian, Pewter, Cerulean, Vermilion, Lavender, Celadon, Saffron, Fuchsia, Cinnabar)
- **Kanto gyms:** 8 (all with unique badge flags and leader trainers)
- **Kanto routes documented:** 22 (Routes 1-21, 24-28)
- **Total route trainers:** ~84
- **Total gym trainers:** ~20 (across all 8 gyms)
- **Key dungeons:** Mt. Moon, Rock Tunnel, Diglett's Cave, Silver Cave (4 rooms)
- **Major sidequests:** Power Plant restoration, Copycat's Lost Item, Snorlax wake-up, Mt. Silver access
- **Special battles:** Snorlax (Lv50), Rival (Mt. Moon), Red (Silver Cave summit)
- **TMs obtainable:** Dream Eater (Viridian), Zap Cannon (Power Plant), Giga Drain (Celadon Gym), Toxic (Fuchsia Gym), Steel Wing (Rock Tunnel)
- **Gift items:** Silver Wing (Pewter), HP Up (Vermilion, 16 badges), Focus Band (Fighting Dojo), Nugget (Route 25), Star Piece (Route 27)
