# Pokemon Crystal — Story Events by Map (Chronological)

> Source: pokecrystal-master map scripts — scene_scripts, coord_events, event flags
> Every story event, cutscene, and trigger in chronological game progression order

---

## 1. New Bark Town — Game Start

### Player's House 2F — Wake Up
- **Trigger**: Game start
- **What happens**: Player in bedroom, set clock/day, go downstairs
- **Flags set**: Time/day initialized

### Player's House 1F — Mom gives Pokegear
- **Trigger**: Going downstairs for first time
- **What happens**: Mom gives POKEGEAR, explains phone
- **Flags set**: EVENT_GOT_POKEGEAR_FROM_MOM

### Elm's Lab — Meet Professor Elm
- **Trigger**: Enter lab (SCENE_ELMSLAB_MEET_ELM)
- **What happens**: Walk up to Elm, he explains research, asks for help
- **Cutscene**: Player auto-walks to Elm, Elm gets email from Mr. Pokemon
- **Player must say "yes" (can't refuse)**
- **Dialogue**: "I needed to ask you a favor... I'm writing a paper..."

### Elm's Lab — Choose Starter
- **Trigger**: After Elm's intro speech
- **What happens**: Choose Cyndaquil (fire), Totodile (water), or Chikorita (grass), all Lv5
- **Flags set**: EVENT_GOT_CYNDAQUIL/TOTODILE/CHIKORITA_FROM_ELM, EVENT_GOT_A_POKEMON_FROM_ELM
- **Items**: Starter holds Berry; Elm's phone number registered (PHONE_ELM)
- **After**: SCENE changes to SCENE_ELMSLAB_AIDE_GIVES_POTION

### Elm's Lab — Aide Gives Potion
- **Trigger**: Walking past Aide position (coord_event at (4,8)/(5,8))
- **What happens**: Aide walks to player, gives Potion
- **Flags set**: Scene advances to SCENE_ELMSLAB_NOOP

### New Bark Town — Teacher Stops You
- **Trigger**: coord_event at (1,8)/(1,9), SCENE_NEWBARKTOWN_TEACHER_STOPS_YOU
- **Condition**: Before getting a Pokemon from Elm
- **What happens**: Teacher runs to player, warns about dangerous grass, brings you back
- **Music**: MUSIC_MOM plays during cutscene
- **Dialogue**: "It's dangerous to go out without a Pokemon!"

### New Bark Town — Rival Peeks at Lab
- **Trigger**: SPRITE_RIVAL at position (3,2), EVENT_RIVAL_NEW_BARK_TOWN controls visibility
- **What happens**: Rival stands outside lab window, says "So this is the famous Elm Pokemon Lab..."
- **Interaction**: If talked to, he shoves player away ("...What are you staring at?")

---

## 2. Route 29 — Catching Tutorial

### Route 29 — Dude's Catching Tutorial
- **Trigger**: coord_event at (53,8)/(53,9), SCENE_ROUTE29_CATCH_TUTORIAL
- **Condition**: After delivering Mystery Egg to Elm (EVENT_GAVE_MYSTERY_EGG_TO_ELM set)
- **What happens**: Cooltrainer M runs up, offers to show how to catch Pokemon
- **Demo**: Loads wild Rattata Lv5, runs BATTLETYPE_TUTORIAL
- **Flags set**: EVENT_LEARNED_TO_CATCH_POKEMON, EVENT_DUDE_TALKED_TO_YOU

### Route 29 — Tuscany (Day-of-Week Sibling)
- **Trigger**: Appears on Tuesdays after Zephyr Badge (ENGINE_ZEPHYRBADGE)
- **What happens**: Tuscany introduces herself, gives Pink Bow
- **Flags set**: EVENT_MET_TUSCANY_OF_TUESDAY, EVENT_GOT_PINK_BOW_FROM_TUSCANY

---

## 3. Cherrygrove City — Guide Gent

### Cherrygrove City — Guide Gent Tour
- **Trigger**: First visit to Cherrygrove
- **What happens**: Guide Gent offers city tour, shows Pokemon Center, Mart, route to Route 30
- **Item**: Map Card for Pokegear
- **Flags set**: EVENT_GOT_MAP_CARD_FROM_GUIDE_GENT

---

## 4. Route 30 / Mr. Pokemon's House

### Mr. Pokemon's House — Mystery Egg + Pokedex
- **Trigger**: Enter Mr. Pokemon's house
- **What happens**: Mr. Pokemon gives Mystery Egg; Professor Oak appears, evaluates player, gives Pokedex
- **Items**: MYSTERY_EGG, POKEDEX
- **Flags set**: EVENT_GOT_MYSTERY_EGG_FROM_MR_POKEMON
- **After**: Elm calls frantically — Pokemon stolen, return immediately

---

## 5. Cherrygrove City — Rival Battle 1

### Cherrygrove City — Silver Ambush
- **Trigger**: Returning south through Cherrygrove after Mr. Pokemon's house
- **Condition**: EVENT_RIVAL_CHERRYGROVE_CITY
- **What happens**: Rival blocks path, battle initiates
- **Battle**: RIVAL1 with stolen starter Lv5 (type advantage over player's choice)
- **After battle**: Rival drops trainer card, player reports name to Elm later

---

## 6. Elm's Lab — Return with Egg

### Elm's Lab — Deliver Mystery Egg
- **Trigger**: Talk to Elm with MYSTERY_EGG in bag
- **What happens**: Hand over egg, Elm excited about Pokedex from Oak
- **Flags set**: EVENT_GAVE_MYSTERY_EGG_TO_ELM, ENGINE_MOBILE_SYSTEM
- **Enables**: Route 29 catching tutorial (SCENE_ROUTE29_CATCH_TUTORIAL)
- **Enables**: Route 30 trainers (EVENT_ROUTE_30_YOUNGSTER_JOEY cleared)
- **Scene advances**: SCENE_ELMSLAB_AIDE_GIVES_POKE_BALLS

### Elm's Lab — Meet Officer
- **Trigger**: coord_event at (4,5)/(5,5), SCENE_ELMSLAB_MEET_OFFICER
- **What happens**: Police officer investigating theft, asks about red-haired boy
- **Player names the Rival** (special NameRival)
- **Flags set**: Scene advances to NOOP

### Elm's Lab — Aide Gives 5 Poke Balls
- **Trigger**: coord_event at (4,8)/(5,8), SCENE_ELMSLAB_AIDE_GIVES_POKE_BALLS
- **What happens**: Aide gives 5 Poke Balls for Pokedex quest
- **Items**: POKE_BALL x5

---

## 7. Sprout Tower — HM05 Flash

### Sprout Tower 3F — Sage Li
- **Trigger**: Defeat Sage Li at top
- **What happens**: Gives HM05 Flash
- **Note**: Rival (Silver) is seen here leaving, having already defeated Li
- **Flags set**: EVENT_SPROUT_TOWER_SAGE_LI

---

## 8. Violet City — Badge 1

### Violet Gym — Falkner
- **Trigger**: Enter gym, defeat trainers, challenge Falkner
- **Battle**: Pidgey Lv7, Pidgeotto Lv9
- **Reward**: ZEPHYR BADGE (ENGINE_ZEPHYRBADGE), TM31 Mud-Slap
- **Enables**: Flash outside battle, Pokemon up to Lv20 obey

### Violet Pokecenter — Elm's Aide Gives Togepi Egg
- **Trigger**: Visit Pokecenter after beating Falkner
- **Condition**: ENGINE_ZEPHYRBADGE set
- **What happens**: Elm's Aide waiting, gives Togepi Egg
- **Flags set**: EVENT_GOT_TOGEPI_EGG_FROM_ELMS_AIDE

---

## 9. Azalea Town — Badge 2

### Slowpoke Well B1F — Team Rocket
- **Trigger**: Enter well (Kurt rushes ahead)
- **Battles**: 3 Rocket Grunts + Executive Proton (Zubat Lv8, Koffing Lv12)
- **Story**: Team Rocket cutting Slowpoke tails for profit
- **After**: Kurt gives Lure Ball, Team Rocket cleared from Azalea
- **Flags set**: EVENT_CLEARED_SLOWPOKE_WELL

### Azalea Gym — Bugsy
- **Battle**: Metapod Lv14, Kakuna Lv14, Scyther Lv16
- **Reward**: HIVE BADGE (ENGINE_HIVEBADGE), TM49 Fury Cutter
- **Enables**: Cut outside battle

### Azalea Town Exit — Rival Battle 2
- **Trigger**: Walking toward Ilex Forest exit
- **Battle**: RIVAL1_2 — Gastly Lv12, Zubat Lv14, [starter evo] Lv16
- **Flags set**: EVENT_RIVAL_AZALEA_TOWN

---

## 10. Ilex Forest — Farfetch'd Quest

### Ilex Forest — Farfetch'd Chase
- **Trigger**: Enter forest, charcoal maker's apprentice has lost Farfetch'd
- **Puzzle**: Step behind Farfetch'd from correct direction to guide it back
- **Reward**: HM01 Cut, Charcoal
- **Also**: NPC gives TM02 Headbutt deeper in forest
- **Flags set**: EVENT_ILEX_FOREST_FARFETCHD

---

## 11. Goldenrod City — Badge 3

### Goldenrod Gym — Whitney
- **Battle**: Clefairy Lv18 (Encore, Metronome), Miltank Lv20 (Rollout, Milk Drink, Attract, Stomp)
- **Special**: Whitney cries after losing; must talk to her or gym trainers to receive badge
- **Reward**: PLAIN BADGE (ENGINE_PLAINBADGE), TM45 Attract
- **Enables**: Strength outside battle, SquirtBottle from Flower Shop

### Goldenrod Flower Shop — SquirtBottle
- **Trigger**: Talk to shopkeeper with Plain Badge
- **Item**: SQUIRTBOTTLE
- **Purpose**: Remove Sudowoodo on Route 36

### Goldenrod Radio Tower — Radio Card
- **Trigger**: Pass quiz (answers: Yes, Yes, No, Yes, No)
- **Item**: RADIO_CARD for Pokegear
- **Flag**: ENGINE_RADIO_CARD

### Goldenrod Move Tutor (Crystal-exclusive)
- **Trigger**: After Elite Four, on Wednesday or Saturday, with Coin Case
- **What happens**: NPC outside Game Corner teaches Flamethrower/Thunderbolt/Ice Beam for 4000 coins
- **Flag**: ENGINE_DAILY_MOVE_TUTOR (daily reset)

---

## 12. Route 36 — Sudowoodo

### Route 36 — Sudowoodo Battle
- **Trigger**: Use SquirtBottle on fake tree
- **Battle**: Wild Sudowoodo Lv20 (Rock type)
- **After**: Route 37 to Ecruteak opens

---

## 13. Ecruteak City — Badge 4

### Burned Tower 1F — Rival Battle 3
- **Trigger**: Rival appears in Burned Tower
- **Battle**: RIVAL1_3 — 4 Pokemon (~Lv20s, includes Haunter, Magnemite, starter evo)
- **Flags set**: EVENT_RIVAL_BURNED_TOWER

### Burned Tower B1F — Release the Beasts
- **Trigger**: Fall through floor to B1F (coord_event), SCENE_BURNEDTOWERB1F_RELEASE_THE_BEASTS
- **Cutscene**: Raikou, Entei, Suicune awaken and flee one by one
  - Raikou runs left, Entei runs right, Suicune pauses to look at player then runs
- **Flags set**: EVENT_RELEASED_THE_BEASTS
- **Enables**: Raikou and Entei begin roaming Johto (special InitRoamMons)
- **Enables**: Ecruteak Gym (SCENE_ECRUTEAKGYM_NOOP)
- **Enables**: Suicune appears at Cianwood City (Crystal)
- **After**: Eusine appears in B1F, talks about Suicune

### Dance Theater — Kimono Girls + HM03 Surf
- **Trigger**: Battle 5 Kimono Girls (each has Eeveelution: Flareon, Jolteon, Vaporeon, Umbreon, Espeon)
- **After winning**: Old man outside gives HM03 Surf
- **Flags set**: EVENT_GOT_HM03_SURF

### Ecruteak Gym — Morty
- **Battle**: Gastly Lv21, Haunter Lv21, Haunter Lv23, Gengar Lv25
- **Puzzle**: Invisible floor (walk on hidden path)
- **Reward**: FOG BADGE (ENGINE_FOGBADGE), TM30 Shadow Ball
- **Enables**: Surf outside battle

---

## 14. Olivine + Cianwood — Badges 5-6

### Olivine Lighthouse 6F — Sick Ampharos
- **Trigger**: Climb to top of lighthouse
- **What happens**: Jasmine tends to sick Ampharos (Amphy), needs SecretPotion from Cianwood
- **Gym blocked**: Olivine Gym cannot be challenged until medicine delivered

### Cianwood City — Suicune Sighting (Crystal)
- **Trigger**: Enter Cianwood, SCENE_CIANWOODCITY_SUICUNE_AND_EUSINE
- **What happens**: Suicune appears on northern rocks, Eusine arrives, Suicune flees
- **Flags set**: EVENT_SAW_SUICUNE_AT_CIANWOOD_CITY

### Cianwood Gym — Chuck
- **Battle**: Primeape Lv27, Poliwrath Lv30
- **Reward**: STORM BADGE (ENGINE_STORMBADGE), TM01 DynamicPunch
- **After**: Chuck's wife outside gives HM02 Fly

### Cianwood Pharmacy — SecretPotion
- **Item**: SECRETPOTION (free)

### Olivine Lighthouse — Deliver Medicine
- **Trigger**: Talk to Jasmine with SecretPotion
- **What happens**: Jasmine heals Ampharos, returns to gym
- **Flags set**: EVENT_JASMINE_RETURNED_TO_GYM

### Olivine Gym — Jasmine
- **Battle**: Magnemite Lv30 x2, Steelix Lv35
- **Reward**: MINERAL BADGE (ENGINE_MINERALBADGE), TM23 Iron Tail

---

## 15. Lake of Rage + Mahogany — Badge 7

### Lake of Rage — Red Gyarados
- **Trigger**: Surf to center of lake
- **Battle**: Shiny Red Gyarados Lv30 (static encounter, guaranteed shiny)
- **After**: Meet Lance on shore, he's investigating Team Rocket signal
- **Item**: Red Scale (from Gyarados; trade to Mr. Pokemon for EXP Share)

### Route 42 — Suicune Sighting (Crystal)
- **Trigger**: After Cianwood sighting
- **What happens**: Suicune appears on Route 42, flees
- **Flags set**: EVENT_SAW_SUICUNE_ON_ROUTE_42

### Team Rocket Base (Mahogany) B1F-B3F
- **Trigger**: Lance leads you to hidden entrance under Souvenir Shop
- **Battles**: ~10 Rocket Grunts, 3 Electrode Lv23 (switches), Executives
- **Puzzle**: Password doors ("HAIL GIOVANNI", spoken by Murkrow), Electrode power switches
- **Story**: Shut down signal forcing Magikarp evolution
- **Lance assists**: Heals your Pokemon during infiltration
- **Reward**: HM06 Whirlpool (from Lance)
- **Items**: TM46 Thief, Hyper Potion, Nugget, Guard Spec, Protein, X Special, Full Heal, Ice Heal, Ultra Ball
- **Flags set**: EVENT_CLEARED_ROCKET_HIDEOUT

### Mahogany Gym — Pryce
- **Condition**: Must clear Rocket HQ first
- **Battle**: Seel Lv27, Dewgong Lv29, Piloswine Lv31
- **Puzzle**: Sliding ice floor
- **Reward**: GLACIER BADGE (ENGINE_GLACIERBADGE), TM16 Icy Wind
- **Enables**: Whirlpool outside battle

---

## 16. Goldenrod Radio Tower Crisis

### Goldenrod City — Rocket Takeover
- **Trigger**: After Mahogany Gym, return to Goldenrod
- **What happens**: Team Rocket has taken over Radio Tower; Rockets patrol the city
- **Flags**: EVENT_GOLDENROD_CITY_ROCKET_TAKEOVER, EVENT_RADIO_TOWER_ROCKET_TAKEOVER

### Radio Tower 1F-5F — Clear Rocket Forces
- **Battles**: Multiple Rocket Grunts + Executives on each floor
- **Story**: Need Basement Key from Underground to access Director

### Goldenrod Underground — Rival Battle 4
- **Trigger**: Encounter Silver in Underground during Radio Tower crisis
- **Battle**: RIVAL1_4 — 5 Pokemon (~Lv30s)

### Goldenrod Underground Warehouse — Executives
- **Battles**: Executive Proton, Executive Ariana, Executive Archer
- **Story**: Archer is the final boss, claims to carry on Giovanni's legacy
- **After defeating Archer**: Director rescued, Radio Tower liberated

### Radio Tower 5F — Reward
- **Trigger**: After clearing all Rockets
- **Reward**: CLEAR_BELL (Crystal) / RAINBOW_WING (Gold) / SILVER_WING (Silver)
- **Flags set**: EVENT_CLEARED_RADIO_TOWER
- **Enables**: Tin Tower access for legendary encounter (Crystal)

---

## 17. Ice Path + Blackthorn — Badge 8

### Ice Path B1F-B3F
- **Puzzle**: Sliding ice floors + boulder pushing (requires Strength)
- **Item**: HM07 Waterfall (in 1F)
- **No trainers**: Pure puzzle dungeon

### Blackthorn Gym — Clair
- **Battle**: Dragonair Lv37 x3, Kingdra Lv40
- **Puzzle**: Lava/platform puzzle (2F connections)
- **Special**: Clair REFUSES badge after defeat

### Dragon's Den — Badge Quest
- **Trigger**: Must enter Dragon's Den behind gym (requires Surf + Whirlpool)
- **Reach**: Dragon Shrine in center of lake
- **Quiz**: Dragon Master asks 5 questions (correct = emphasize kindness/love)
- **After quiz**: Clair reluctantly gives RISING BADGE
- **Reward**: RISING BADGE (ENGINE_RISINGBADGE), TM24 DragonBreath
- **Bonus**: Elder gives Dratini Lv15 (ExtremeSpeed moveset if all answers correct)
- **Enables**: Waterfall outside battle, all Pokemon obey
- **Flags set**: EVENT_BEAT_CLAIR

---

## 18. Tin Tower — Legendary Encounter (Crystal)

### Tin Tower Entrance — Kimono Girls (Crystal)
- **Trigger**: Clear Bell + all 8 badges
- **What happens**: Must battle 5 Kimono Girls again at Tin Tower entrance
- **After winning**: Suicune appears at Tin Tower base (Lv40, static encounter)

### Tin Tower Roof — Ho-Oh (Crystal)
- **Trigger**: Rainbow Wing (from Pewter City old man after all 16 badges)
- **Battle**: Ho-Oh Lv60
- **Moves**: Sacred Fire, Sunny Day, Fire Blast, Swift

---

## 19. Suicune Chase Sequence (Crystal-exclusive)

Suicune appears at fixed locations in sequence:
1. **Burned Tower B1F** (Ecruteak) — flees with beasts
2. **Cianwood City** (north rocks) — Eusine present, flees
3. **Route 42** — flees
4. **Vermilion City** — flees
5. **Route 14** — flees
6. **Tin Tower** (with Clear Bell) — final battle Lv40

Each sighting sets a flag: EVENT_SAW_SUICUNE_AT_[LOCATION]

---

## 20. Victory Road — Route to Champion

### Route 26/27 — Badge Check Gates
- **Trigger**: All 8 Johto badges required to enter Victory Road
- **Guards check each badge individually

### Victory Road — Rival Battle 5
- **Trigger**: Encounter Silver in Victory Road
- **Battle**: RIVAL1_5 — Full team of 6 Pokemon (~Lv34-38)
- **Dialogue**: Silver beginning to question his approach to Pokemon

---

## 21. Indigo Plateau — Elite Four + Champion

### Elite Four (Sequential)
1. **Will** (Psychic): Xatu Lv40, Jynx Lv41, Exeggutor Lv41, Slowbro Lv41, Xatu Lv42
2. **Koga** (Poison): Ariados Lv40, Venomoth Lv41, Forretress Lv43, Muk Lv42, Crobat Lv44
3. **Bruno** (Fighting): Hitmontop Lv42, Hitmonlee Lv42, Hitmonchan Lv42, Onix Lv43, Machamp Lv46
4. **Karen** (Dark): Umbreon Lv42, Vileplume Lv42, Gengar Lv45, Murkrow Lv44, Houndoom Lv47

### Champion Lance
- **Battle**: Gyarados Lv44, Dragonite Lv47 x2, Aerodactyl Lv46, Dragonite Lv50, Charizard Lv46
- **After**: Hall of Fame, credits
- **Flags set**: EVENT_BEAT_ELITE_FOUR
- **Enables**: Kanto post-game, S.S. Ticket from Elm

---

## 22. Post-Game: Kanto

### Elm's Lab — S.S. Ticket
- **Trigger**: Visit Elm after Elite Four
- **Item**: S.S. TICKET
- **Enables**: S.S. Aqua travel (Olivine -> Vermilion)

### S.S. Aqua — Travel to Kanto
- **Route**: Olivine Port -> Vermilion Port
- **Events on ship**: Various trainer battles, meet rival's Pokeball (in a cabin)

### Vermilion City — Suicune Sighting (Crystal)
- **Trigger**: Part of Suicune chase sequence
- **What happens**: Suicune on dock area, flees

### Cerulean City — Rocket Grunt + Machine Part
- **Trigger**: Visit Cerulean, find Rocket Grunt near gym
- **Battle**: Defeat Grunt, he reveals hiding spot
- **Quest item**: Machine Part (hidden in Cerulean Gym pool at (3,8))
- **Purpose**: Return to Power Plant to restore electricity
- **Flags set**: EVENT_FOUND_MACHINE_PART_IN_CERULEAN_GYM

### Power Plant — Restore Power
- **Trigger**: Return Machine Part to Power Plant Manager
- **What happens**: Kanto power restored
- **Enables**: Lavender Radio Tower operational, EXPN Card available
- **Flags set**: EVENT_RESTORED_POWER_TO_KANTO

### Lavender Radio Tower — EXPN Card
- **Item**: EXPN CARD (enables Kanto radio stations)
- **Enables**: Poke Flute channel (wake Snorlax on Routes 12 and 16)

### Route 25 — Find Misty
- **Trigger**: Visit Bill's House / Cape on Route 25
- **What happens**: Misty is on a date with a trainer; interrupting scares the date away
- **After**: Misty returns to Cerulean Gym, can be challenged

### Copycat's House (Saffron) — Lost Item Quest
1. Copycat lost her Poke Doll
2. Go to Pokemon Fan Club in Vermilion — Chairman has Lost Item
3. Return Lost Item to Copycat
4. **Reward**: PASS (Magnet Train ticket, Goldenrod <-> Saffron)

### Mt. Moon Square — Monday Night Clefairy Dance
- **Trigger**: Visit Mt. Moon Square on Monday night
- **What happens**: Clefairy dance around stone, player interrupts, they flee
- **Item**: Moon Stone (hidden, regenerates weekly)

---

## 23. Kanto Gym Battles

Order is flexible; common sequence:
- Lt. Surge (Vermilion) — Thunder Badge
- Sabrina (Saffron) — Marsh Badge
- Erika (Celadon) — Rainbow Badge
- Janine (Fuchsia) — Soul Badge
- Brock (Pewter) — Boulder Badge
- Misty (Cerulean) — Cascade Badge (requires Power Plant quest)
- Blaine (Seafoam Islands) — Volcano Badge
- Blue (Viridian) — Earth Badge

---

## 24. Mt. Silver — Final Battle

### Mt. Silver Entrance
- **Trigger**: All 16 badges required
- **Route 28 gate opens

### Silver Cave Summit — Trainer Red
- **Trigger**: Climb to summit
- **Cutscene**: Red says nothing ("...")
- **Battle**: Pikachu Lv81, Espeon Lv73, Snorlax Lv75, Venusaur Lv77, Charizard Lv77, Blastoise Lv77
- **After**: Credits roll again — true ending
- **Red's team is the highest level in the game**

---

## Appendix: Key Event Flags

| Flag | Meaning |
|------|---------|
| EVENT_GOT_A_POKEMON_FROM_ELM | Chose starter |
| EVENT_GOT_MYSTERY_EGG_FROM_MR_POKEMON | Visited Mr. Pokemon |
| EVENT_GAVE_MYSTERY_EGG_TO_ELM | Returned egg to Elm |
| EVENT_RIVAL_CHERRYGROVE_CITY | Rival Battle 1 done |
| EVENT_RIVAL_AZALEA_TOWN | Rival Battle 2 done |
| EVENT_RIVAL_BURNED_TOWER | Rival Battle 3 done |
| EVENT_RELEASED_THE_BEASTS | Legendary beasts flee |
| EVENT_CLEARED_SLOWPOKE_WELL | Rocket cleared from well |
| EVENT_CLEARED_ROCKET_HIDEOUT | Mahogany HQ cleared |
| EVENT_CLEARED_RADIO_TOWER | Radio Tower crisis resolved |
| EVENT_BEAT_CLAIR | Defeated Clair |
| EVENT_BEAT_ELITE_FOUR | Became Champion |
| EVENT_RESTORED_POWER_TO_KANTO | Power Plant quest done |
| EVENT_FOUND_MACHINE_PART_IN_CERULEAN_GYM | Found quest item |
| ENGINE_ZEPHYRBADGE through ENGINE_EARTHBADGE | All 16 badge flags |

### Engine Flags (Persistent)
- ENGINE_FLYPOINT_[CITY] — Fly destination unlocked
- ENGINE_RADIO_CARD — Has radio
- ENGINE_[BADGE]BADGE — Badge obtained
- ENGINE_DAILY_* — Daily event cooldowns

### Scene Flags (Map-specific)
- SCENE_[MAP]_[EVENT] — Controls which cutscene plays on map entry
- setscene advances the scene counter for the current map
- setmapscene changes another map's scene remotely
