# Pokemon Crystal - Event Data Tables

All event-related data tables from pokecrystal data/events/ (12 files).

Source: `data/events/*.asm`

---

## Happiness Changes

From `data/events/happiness_changes.asm`. Three columns: change when happiness < 100, < 200, >= 200.

| Event | <100 | <200 | >=200 |
|-------|------|------|-------|
| Gained a level | +5 | +3 | +2 |
| Vitamin used | +5 | +3 | +2 |
| X Item used | +1 | +1 | +0 |
| Battled a Gym Leader | +3 | +2 | +1 |
| Learned a move | +1 | +1 | +0 |
| Lost to an enemy | -1 | -1 | -1 |
| Fainted due to poison | -5 | -5 | -10 |
| Lost to a much stronger enemy | -5 | -5 | -10 |
| Haircut (older brother) - low roll | +1 | +1 | +1 |
| Haircut (older brother) - mid roll | +3 | +3 | +1 |
| Haircut (older brother) - high roll | +5 | +5 | +2 |
| Haircut (younger brother) - low roll | +1 | +1 | +1 |
| Haircut (younger brother) - mid roll | +3 | +3 | +1 |
| Haircut (younger brother) - high roll | +10 | +10 | +4 |
| Heal Powder / EnergyPowder (bitter) | -5 | -5 | -10 |
| Energy Root (bitter) | -10 | -10 | -15 |
| Revival Herb (bitter) | -15 | -15 | -20 |
| Daisy's Grooming | +3 | +3 | +1 |
| Level up in caught location | +10 | +6 | +4 |

### Haircut Probabilities

From `data/events/happiness_probabilities.asm`:

**Older Haircut Brother (Goldenrod Underground):**
| Roll | Chance | Happiness Change Text |
|------|--------|----------------------|
| Low | 30% | "It looks a little Pokemon took a Pokemon." |
| Mid | 50% | "Looks Pokemon became Pokemon." |
| High | 20% | "It looks Pokemon loves your Pokemon!" |

**Younger Haircut Brother (Goldenrod Underground):**
| Roll | Chance | Happiness Change Text |
|------|--------|----------------------|
| Low | 60% | Minor increase |
| Mid | 30% | Moderate increase |
| High | 10% | Large increase (+10/+10/+4) |

**Daisy's Grooming (Pallet Town):**
- 99.6% chance of success (BUG: should always work, but doesn't due to off-by-one)

---

## NPC Trades

From `data/events/npc_trades.asm`:

| Location | You Give | You Get | Nickname | OT Name | OT ID | Held Item | Gender Req |
|----------|----------|---------|----------|---------|-------|-----------|------------|
| Goldenrod Dept Store | Abra | Machop | MUSCLE | MIKE | 37460 | Gold Berry | Either |
| Violet City | Bellsprout | Onix | ROCKY | KYLE | 48926 | Bitter Berry | Either |
| Olivine City | Krabby | Voltorb | VOLTY | TIM | 29189 | PRZCureBerry | Either |
| Blackthorn City | Dragonair | Dodrio | DORIS | EMY | 00283 | Smoke Ball | Female only |
| Route 39 (Ecruteak side) | Haunter | Xatu | PAUL | CHRIS | 15616 | MysteryBerry | Either |
| Route 14 | Chansey | Aerodactyl | AEROY | KIM | 26491 | Gold Berry | Either |
| Power Plant | Dugtrio | Magneton | MAGGIE | FOREST | 50082 | Metal Coat | Either |

---

## Odd Egg (Crystal Only)

From `data/events/odd_eggs.asm`. The Odd Egg from the Day-Care Man can hatch into one of 7 species. Each species has a normal version (all 0 DVs) and a shiny version (DVs 2/10/10/10). All hatch at level 5 with Dizzy Punch as an egg move.

| Species | Normal Chance | Shiny Chance | Total | Moves |
|---------|-------------|-------------|-------|-------|
| Pichu | 8% | 1% | 9% | ThunderShock, Charm, Dizzy Punch |
| Cleffa | 16% | 3% | 19% | Pound, Charm, Dizzy Punch |
| Igglybuff | 16% | 3% | 19% | Sing, Charm, Dizzy Punch |
| Smoochum | 14% | 2% | 16% | Pound, Lick, Dizzy Punch |
| Magby | 10% | 2% | 12% | Ember, Dizzy Punch |
| Elekid | 12% | 2% | 14% | Quick Attack, Leer, Dizzy Punch |
| Tyrogue | 10% | 1% | 11% | Tackle, Dizzy Punch |

**Overall shiny chance from Odd Egg: 14%** (far higher than the normal 1/8192)

---

## Bug Catching Contest

### Contestants and Scores

From `data/events/bug_contest_winners.asm`. Each NPC contestant has 3 pre-set possible catches with scores:

| Contestant | Class | 1st Place Mon/Score | 2nd Place Mon/Score | 3rd Place Mon/Score |
|-----------|-------|--------------------|--------------------|---------------------|
| Don | Bug Catcher | Kakuna / 300 | Metapod / 285 | Caterpie / 226 |
| Ed | Bug Catcher | Butterfree / 286 | Butterfree / 251 | Caterpie / 237 |
| Nick | Cooltrainer M | Scyther / 357 | Butterfree / 349 | Pinsir / 368 |
| William | Pokefan M | Pinsir / 332 | Butterfree / 324 | Venonat / 321 |
| Benny | Bug Catcher | Butterfree / 318 | Weedle / 295 | Caterpie / 285 |
| Barry | Camper | Pinsir / 366 | Venonat / 329 | Kakuna / 314 |
| Cindy | Picnicker | Butterfree / 341 | Metapod / 301 | Caterpie / 264 |
| Josh | Bug Catcher | Scyther / 326 | Butterfree / 292 | Metapod / 282 |
| Samuel | Youngster | Weedle / 270 | Pinsir / 282 | Caterpie / 251 |
| Kipp | Schoolboy | Venonat / 267 | Paras / 254 | Kakuna / 259 |

The game selects 10 random contestants per contest (3 appear physically in National Park).

---

## Magikarp Length Calculation

From `data/events/magikarp_lengths.asm`. Used for the Magikarp size contest at the Lake of Rage.

The length is calculated from DVs using a threshold table. Due to a bug in the BC comparison, only register B is used for the threshold check.

| BC Threshold | Divisor | Resulting Length Range |
|-------------|---------|----------------------|
| 110 | 1 | Shortest |
| 310 | 2 | Very small |
| 710 | 4 | Small |
| 2,710 | 20 | Below average |
| 7,710 | 50 | Average |
| 17,710 | 100 | Above average |
| 32,710 | 150 | Large |
| 47,710 | 150 | Very large |
| 57,710 | 100 | Huge |
| 62,710 | 50 | Enormous |
| 64,710 | 20 | Near maximum |
| 65,210 | 5 | Exceptional |
| 65,410 | 2 | Near record |

---

## Pokedex Ratings (Prof. Oak's PC)

From `data/events/pokedex_ratings.asm`:

| Pokemon Caught | Rating |
|---------------|--------|
| 0-9 | Rating 1 (lowest fanfare) |
| 10-19 | Rating 2 |
| 20-34 | Rating 3 |
| 35-49 | Rating 4 |
| 50-64 | Rating 5 |
| 65-79 | Rating 6 |
| 80-94 | Rating 7 |
| 95-109 | Rating 8 |
| 110-124 | Rating 9 |
| 125-139 | Rating 10 |
| 140-154 | Rating 11 |
| 155-169 | Rating 12 |
| 170-184 | Rating 13 |
| 185-199 | Rating 14 |
| 200-214 | Rating 15 |
| 215-229 | Rating 16 |
| 230-239 | Rating 17 |
| 240-248 | Rating 18 |
| 249-255 | Rating 19 (highest, completion) |

---

## Unown Wall Words

From `data/events/unown_walls.asm`. The four Unown puzzle words in the Ruins of Alph:

1. **ESCAPE** - Kabuto chamber
2. **LIGHT** - Omanyte chamber
3. **WATER** - Ho-Oh chamber (requires Surf)
4. **HO-OH** - Aerodactyl chamber

---

## Elevator Floors

From `data/events/elevator_floors.asm`:

B4F, B3F, B2F, B1F, 1F, 2F, 3F, 4F, 5F, 6F, 7F, 8F, 9F, 10F, 11F, ROOF

Used in Goldenrod Dept Store, Radio Tower, and other buildings with elevators.

---

## Engine Flags (Game State Tracking)

From `data/events/engine_flags.asm`. Key engine flags tracked:

### Pokegear Flags
- Radio Card, Map Card, Phone Card, EXPN Card, Pokegear obtained

### Day-Care Flags
- Day-Care Man has egg, has mon; Day-Care Lady has mon

### Mom's Savings
- Saving money active, Mom active

### Status Flags
- Pokedex obtained, Unown Dex, Caught Pokerus, Rocket signal on
- Hall of Fame entered, Bug Contest timer, Rockets in Radio Tower
- Reached Goldenrod, Rockets in Mahogany

### Bike Flags
- Strength active, Always on bike, Downhill

### Badges (all individually tracked)
- Johto: Zephyr, Hive, Plain, Fog, Mineral, Storm, Glacier, Rising
- Kanto: Boulder, Cascade, Thunder, Rainbow, Soul, Marsh, Volcano, Earth

### Fly/Spawn Points (26 total)
- Home, Pallet, Viridian, Pewter, Cerulean, Rock Tunnel, Vermilion
- Lavender, Saffron, Celadon, Fuchsia, Cinnabar, Indigo Plateau
- New Bark, Cherrygrove, Violet, Azalea, Cianwood, Goldenrod
- Olivine, Ecruteak, Mahogany, Lake of Rage, Blackthorn, Mt. Silver

### Daily Flags
- Kurt making balls, Bug Contest, Fish swarm, Time Capsule
- All fruit trees, Got Shuckie, Underground bargain, Trainer House
- Mt. Moon Square Clefairy, Union Cave Lapras, Haircut received
- Dept Store TM27 Return, Daisy's grooming, Indigo rival fight
- Move Tutor, Buena's Password

### Phone Rematch Trainers (24)
Jack, Huey, Gaven, Beth, Jose, Reena, Joey, Wade, Ralph, Liz, Anthony, Todd, Gina, Arnie, Alan, Dana, Chad, Tully, Brent, Tiffany, Vance, Wilton, Parry, Erin

### Phone Item Gifts (10 trainers give items)
| Trainer | Item |
|---------|------|
| Beverly | Nugget |
| Jose | Star Piece |
| Wade | Various (event-based) |
| Gina | Leaf Stone |
| Alan | Fire Stone |
| Liz | Thunderstone |
| Derek | Nugget |
| Tully | Water Stone |
| Tiffany | Pink Bow |
| Wilton | Various (event-based) |

### Swarm Flags
- Dunsparce swarm (Dark Cave), Yanma swarm (Route 35)
- Buena's Password active, Dept Store sale active

### Other Flags
- Player gender (female), Celebi event active, Lucky Number Show
