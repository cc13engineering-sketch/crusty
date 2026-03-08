# World Area Build Order

> **Purpose**: Defines the optimal implementation sequence for all 152 area data files,
> organized into sprint-sized groups that follow game progression, respect dependencies,
> and produce playable increments at each stage.
>
> **Total areas**: 152 (from JSON files in this directory)
> **Estimated sprint groups**: 45 groups (content, content, QA cadence)
> **Existing engine state**: 33 maps partially built through Olivine City (migration needed)

---

## Table of Contents

1. [Ordering Rationale](#1-ordering-rationale)
2. [Dependency Notes](#2-dependency-notes)
3. [Missing Warp Targets (Shared Templates)](#3-missing-warp-targets-shared-templates)
4. [Complete Build Order](#4-complete-build-order)
5. [Sprint Group Breakdown](#5-sprint-group-breakdown)
6. [Technical Migration Notes](#6-technical-migration-notes)

---

## 1. Ordering Rationale

The build order follows these principles, listed in priority order:

1. **Game progression order**: Areas are sequenced exactly as a player encounters them
   on a first playthrough, from New Bark Town through the Elite Four.

2. **Exterior before interior**: A town/city exterior map must exist before its interior
   buildings (gyms, shops, houses) can be warped into.

3. **Routes between towns**: The route connecting Town A to Town B is built after Town A
   but before Town B, so the player can travel the path.

4. **Dungeons at encounter point**: Sprout Tower is built with Violet City content,
   Slowpoke Well with Azalea Town, etc. -- the point where the player first enters them.

5. **HM gate awareness**: Areas requiring Surf (Routes 40-41, Cianwood), Strength (Ice Path),
   Waterfall (Dragon's Den), or Whirlpool (Whirl Islands) are placed after the player
   obtains those HMs through badge progression.

6. **Complexity progression**: Simpler maps (small buildings, short routes) come before
   complex multi-floor dungeons. The engine's map systems get battle-tested on easy
   maps first.

7. **Template reuse**: PokeCenters, Marts, and generic speech houses use shared templates.
   The first sprint that needs a PokeCenter establishes the template; all subsequent ones
   reuse it.

8. **Playable increments**: Each sprint group produces a self-contained playable segment.
   After completing a group, the player can walk from the start to the newest area.

---

## 2. Dependency Notes

### Badge-Gated HM Requirements

| Badge | HM Unlocked | Areas That Require It |
|-------|-------------|----------------------|
| Zephyr (Gym 1, Violet) | Flash | Dark Cave (optional, visibility) |
| Hive (Gym 2, Azalea) | Cut | Ilex Forest tree barriers |
| Plain (Gym 3, Goldenrod) | Strength | Ice Path boulders, Blackthorn Gym 2F |
| Fog (Gym 4, Ecruteak) | Surf | Routes 40-41, Cianwood, Route 42 water, Whirl Islands |
| Storm (Gym 5, Cianwood) | Fly | No area gate (convenience HM) |
| Mineral (Gym 6, Olivine) | -- | No HM gate |
| Glacier (Gym 7, Mahogany) | Whirlpool | Whirl Islands entrance |
| Rising (Gym 8, Blackthorn) | Waterfall | Dragon's Den B1F waterfall, Tohjo Falls |

### Story Flag Dependencies

| Story Event | Must Complete Before |
|-------------|---------------------|
| Get Starter (Elm's Lab) | Leave New Bark Town |
| Deliver Egg to Elm | Get Pokeballs, Route 30+ trainers |
| Beat Falkner | Route 32 south passage |
| Beat Bugsy | Cut in Ilex Forest |
| Beat Whitney | SquirtBottle (Sudowoodo on Route 36) |
| Beat Morty | Surf overworld usage |
| Lighthouse/Medicine delivery | Fight Jasmine in Olivine Gym |
| Red Gyarados event | Rocket Hideout opens |
| Clear Rocket Hideout | Mahogany Gym accessible |
| Clear Radio Tower | Route 44 / Ice Path fully accessible |
| Beat Clair | Dragon's Den quiz |
| All 8 badges | Route 27 gate, Victory Road |

---

## 3. Missing Warp Targets (Shared Templates)

67 warp targets referenced in area data don't have dedicated JSON files. These fall into
categories that should use shared templates or be created as simple interiors:

### Use Shared PokemonCenter Template (10x8 tiles)
- `cherrygrove_pokecenter_1f`, `violet_pokecenter_1f`, `azalea_pokecenter_1f`
- `goldenrod_pokecenter_1f`, `ecruteak_pokecenter_1f`, `olivine_pokecenter_1f`
- `cianwood_pokecenter_1f`, `mahogany_pokecenter_1f`, `blackthorn_pokecenter_1f`
- `route_32_pokecenter_1f`, `pokecenter_2f` (universal 2F)

### Use Shared Mart Template
- `cherrygrove_mart`, `violet_mart`, `azalea_mart`, `olivine_mart`
- `ecruteak_mart`, `mahogany_mart_1f`, `blackthorn_mart`

### Use Shared GenericHouse Template (8x6 tiles)
- `players_neighbors_house`, `elms_house`, `guide_gents_house`
- `cherrygrove_evolution_speech_house`, `cherrygrove_gym_speech_house`
- `violet_nickname_speech_house`, `violet_kyles_house`
- `route_30_berry_house`
- `goldenrod_bike_shop`, `goldenrod_flower_shop`, `goldenrod_happiness_rater`
- `goldenrod_name_rater`, `goldenrod_pp_speech_house`, `bills_familys_house`
- `ecruteak_lugia_speech_house`, `ecruteak_itemfinder_house`
- `olivine_tims_house`, `olivine_house_beta`, `olivine_punishment_speech_house`
- `olivine_good_rod_house`
- `manias_house`, `cianwood_lugia_speech_house`, `cianwood_photo_studio`
- `poke_seers_house`
- `mahogany_red_gyarados_speech_house`
- `blackthorn_dragon_speech_house`, `blackthorn_emys_house`, `move_deleters_house`
- `lake_of_rage_hidden_power_house`, `lake_of_rage_magikarp_house`
- `day_of_week_siblings_house`
- `route_40_battle_tower_gate`

### Special Interiors (need minimal unique data)
- `olivine_port_passage` -- corridor to Olivine Port
- `ecruteak_tin_tower_entrance` -- gate to Tin Tower (note: `tin_tower_entrance` JSON exists)
- `goldenrod_underground_switch_room_entrances` -- switch room sub-area
- `goldenrod_dept_store_b1f` -- department store basement
- `fast_ship_1f` -- S.S. Aqua (post-game)
- `saffron_magnet_train_station` -- Kanto (post-game)
- `victory_road_gate` -- gate building before Victory Road
- `route_23` -- short route between Victory Road gate and Indigo Plateau
- `ruins_of_alph_*_item_room` (4 rooms) -- small reward rooms behind puzzles

### Build Strategy for Missing Targets
Each sprint that introduces a town should also register that town's shared-template
interiors (PokeCenters, Marts, speech houses). These require minimal effort since the
templates already exist -- just warp destination data and NPC dialogue assignment.

---

## 4. Complete Build Order

Every one of the 152 area_ids, numbered in build sequence. Areas within the same sprint
group share a group number prefix.

### Phase 1: Starting Area (New Bark Town)
```
  1. new_bark_town
  2. players_house_2f
  3. players_house_1f
  4. elms_lab
```

### Phase 2: Route 29 + Cherrygrove
```
  5. route_29
  6. route_29_route_46_gate
  7. cherrygrove_city
```

### Phase 3: Route 30 + Mr. Pokemon's House
```
  8. route_30
  9. mr_pokemons_house
```

### Phase 4: Route 31 + Violet City Exterior
```
 10. route_31
 11. route_31_violet_gate
 12. violet_city
```

### Phase 5: Violet City Interiors + Sprout Tower
```
 13. violet_gym
 14. earls_pokemon_academy
 15. sprout_tower_1f
 16. sprout_tower_2f
 17. sprout_tower_3f
```

### Phase 6: Route 32 + Ruins of Alph Hub
```
 18. route_32
 19. route_32_pokecenter
 20. route_32_ruins_of_alph_gate
 21. ruins_of_alph_outside
 22. ruins_of_alph_research_center
```

### Phase 7: Ruins of Alph Chambers
```
 23. ruins_of_alph_kabuto_chamber
 24. ruins_of_alph_ho_oh_chamber
 25. ruins_of_alph_omanyte_chamber
 26. ruins_of_alph_aerodactyl_chamber
 27. ruins_of_alph_inner_chamber
```

### Phase 8: Union Cave + Route 33
```
 28. union_cave_1f
 29. union_cave_b1f
 30. union_cave_b2f
 31. route_33
```

### Phase 9: Azalea Town + Slowpoke Well
```
 32. azalea_town
 33. azalea_gym
 34. kurts_house
 35. charcoal_kiln
 36. slowpoke_well_b1f
 37. slowpoke_well_b2f
```

### Phase 10: Ilex Forest + Route 34
```
 38. ilex_forest_azalea_gate
 39. ilex_forest
 40. route_34_ilex_forest_gate
 41. route_34
 42. day_care
```

### Phase 11: Goldenrod City Exterior + Core Buildings
```
 43. goldenrod_city
 44. goldenrod_gym
 45. goldenrod_dept_store
 46. goldenrod_game_corner
```

### Phase 12: Goldenrod City Secondary Buildings
```
 47. goldenrod_underground
 48. goldenrod_underground_warehouse
 49. goldenrod_magnet_train_station
 50. radio_tower_1f
 51. radio_tower_2f
 52. radio_tower_3f
 53. radio_tower_4f
 54. radio_tower_5f
```

### Phase 13: Route 35 + National Park
```
 55. route_35_goldenrod_gate
 56. route_35
 57. route_35_national_park_gate
 58. national_park
 59. national_park_bug_contest
```

### Phase 14: Route 36 + Route 37
```
 60. route_36_national_park_gate
 61. route_36
 62. route_36_ruins_of_alph_gate
 63. route_37
```

### Phase 15: Ecruteak City + Key Interiors
```
 64. ecruteak_city
 65. ecruteak_gym
 66. dance_theater
 67. wise_trios_room
 68. tin_tower_entrance
```

### Phase 16: Burned Tower
```
 69. burned_tower_1f
 70. burned_tower_b1f
```

### Phase 17: Tin Tower (Floors 1-5)
```
 71. tin_tower_1f
 72. tin_tower_2f
 73. tin_tower_3f
 74. tin_tower_4f
 75. tin_tower_5f
```

### Phase 18: Tin Tower (Floors 6-Roof)
```
 76. tin_tower_6f
 77. tin_tower_7f
 78. tin_tower_8f
 79. tin_tower_9f
 80. tin_tower_roof
```

### Phase 19: Route 38 + Route 39
```
 81. route_38_ecruteak_gate
 82. route_38
 83. route_39
 84. route_39_barn
 85. route_39_farmhouse
```

### Phase 20: Olivine City + Lighthouse (Lower)
```
 86. olivine_city
 87. olivine_gym
 88. olivine_cafe
 89. olivine_port
 90. olivine_lighthouse_1f
 91. olivine_lighthouse_2f
 92. olivine_lighthouse_3f
```

### Phase 21: Olivine Lighthouse (Upper)
```
 93. olivine_lighthouse_4f
 94. olivine_lighthouse_5f
 95. olivine_lighthouse_6f
```

### Phase 22: Sea Routes + Cianwood
```
 96. route_40
 97. route_41
 98. cianwood_city
 99. cianwood_gym
100. cianwood_pharmacy
```

### Phase 23: Route 42 + Mahogany Town
```
101. route_42_ecruteak_gate
102. route_42
103. mahogany_town
104. mahogany_gym
```

### Phase 24: Route 43 + Lake of Rage
```
105. route_43_mahogany_gate
106. route_43
107. route_43_gate
108. lake_of_rage
```

### Phase 25: Rocket Hideout
```
109. rocket_hideout_b1f
110. rocket_hideout_b2f
111. rocket_hideout_b3f
```

### Phase 26: Mt. Mortar (Optional Dungeon)
```
112. mt_mortar_1f_outside
113. mt_mortar_1f_inside
114. mt_mortar_2f
115. mt_mortar_b1f
```

### Phase 27: Route 44 + Ice Path (Upper)
```
116. route_44
117. ice_path_1f
118. ice_path_b1f
```

### Phase 28: Ice Path (Lower) + Blackthorn Arrival
```
119. ice_path_b2f_mahogany
120. ice_path_b2f_blackthorn
121. ice_path_b3f
```

### Phase 29: Blackthorn City + Gym
```
122. blackthorn_city
123. blackthorn_gym_1f
124. blackthorn_gym_2f
```

### Phase 30: Dragon's Den
```
125. dragons_den_1f
126. dragons_den_b1f
127. dragon_shrine
```

### Phase 31: Route 45 + Route 46 + Dark Cave
```
128. route_45
129. route_46
130. dark_cave_violet_entrance
131. dark_cave_blackthorn_entrance
```

### Phase 32: Whirl Islands (Entrances)
```
132. whirl_islands_nw
133. whirl_islands_ne
134. whirl_islands_sw
135. whirl_islands_se
```

### Phase 33: Whirl Islands (Interior + Lugia)
```
136. whirl_islands_cave
137. whirl_islands_b1f
138. whirl_islands_b2f
139. whirl_islands_lugia_chamber
```

### Phase 34: Victory Road Approach
```
140. route_27
141. route_27_sandstorm_house
142. tohjo_falls
143. route_26
144. route_26_heal_house
```

### Phase 35: Victory Road
```
145. victory_road
```

### Phase 36: Indigo Plateau + Elite Four
```
146. indigo_plateau_pokecenter
147. wills_room
148. kogas_room
149. brunos_room
150. karens_room
151. lances_room
152. hall_of_fame
```

---

## 5. Sprint Group Breakdown

Sprint groups follow a **content, content, QA** cadence. Every 3rd group is a QA audit
sprint that tests and fixes the previous two content groups.

---

### Group 1: New Bark Town -- The Beginning
**Areas (4):**
- `new_bark_town`
- `players_house_2f`
- `players_house_1f`
- `elms_lab`

**Why together**: These form the complete starting experience. The player wakes up in
their bedroom (2F), goes downstairs (1F), exits to town, visits Elm's Lab, receives
starter Pokemon. All four areas are tightly coupled through warps and the opening
story sequence.

**Prerequisites**: None (this is the starting point)
**Complexity**: Simple -- small maps, no trainers, scripted events only
**Migration**: These maps exist in the old engine. Migrate tile layouts and warp data
from old format to new area JSON format.

---

### Group 2: Route 29 + Cherrygrove City
**Areas (3):**
- `route_29`
- `route_29_route_46_gate`
- `cherrygrove_city`

**Why together**: Route 29 is the first route the player walks. The gate to Route 46
is on this route (one-way down from 46, but the gate building is accessible). Cherrygrove
is the destination. Together they form the first "travel to a new location" experience.

**Prerequisites**: Group 1 (New Bark Town)
**Complexity**: Simple -- Route 29 has no trainers, Cherrygrove is small
**Migration**: These maps exist in the old engine.

---

### Group 3: QA Audit -- Starting Area
**QA Focus**: Test Groups 1-2
- Verify player spawn in bedroom, walk through house, exit to town
- Verify Elm's Lab starter selection event
- Verify Route 29 traversal and wild encounters
- Verify Cherrygrove arrival and shared PokemonCenter template
- Test Route 29/46 gate (one-way ledge enforcement)
- Verify save/load at Cherrygrove PokemonCenter

---

### Group 4: Route 30 + Mr. Pokemon's Errand
**Areas (2):**
- `route_30`
- `mr_pokemons_house`

**Why together**: Route 30 leads to Mr. Pokemon's House, which is the destination of
the first story errand. The player must walk this route and visit the house to receive
the Mystery Egg and meet Professor Oak.

**Prerequisites**: Group 2 (Cherrygrove City)
**Complexity**: Simple -- short route, small house interior
**Migration**: These maps exist in the old engine.

---

### Group 5: Violet City Approach
**Areas (3):**
- `route_31`
- `route_31_violet_gate`
- `violet_city`

**Why together**: Route 31 connects Route 30 to Violet City via the gate. This is the
player's path to the first gym city. The Dark Cave entrance on Route 31 exists as a
warp target but Dark Cave itself is built later (optional, requires Flash).

**Prerequisites**: Group 4 (Route 30)
**Complexity**: Medium -- Violet City is the first full city with gym, mart, PokemonCenter
**Migration**: These maps exist in the old engine.

---

### Group 6: QA Audit -- Early Routes
**QA Focus**: Test Groups 4-5
- Verify Mr. Pokemon's House event (receive egg, meet Oak)
- Verify Rival Battle 1 trigger on return through Cherrygrove
- Verify egg delivery to Elm and receiving Pokeballs
- Verify Route 31 traversal and trainer battles
- Verify Violet City warp destinations (gym, academy, PokemonCenter)
- Test gate building transitions

---

### Group 7: Violet City Interiors + Sprout Tower
**Areas (5):**
- `violet_gym`
- `earls_pokemon_academy`
- `sprout_tower_1f`
- `sprout_tower_2f`
- `sprout_tower_3f`

**Why together**: All interiors accessed from Violet City. Sprout Tower is the first
optional dungeon, typically attempted before the gym. The gym battle (Falkner) is the
first badge gate.

**Prerequisites**: Group 5 (Violet City exterior)
**Complexity**: Medium -- Sprout Tower is 3 floors with Sage trainers; gym has elevated walkways
**Migration**: These maps exist in the old engine.

---

### Group 8: Route 32 + Ruins of Alph Hub
**Areas (5):**
- `route_32`
- `route_32_pokecenter`
- `route_32_ruins_of_alph_gate`
- `ruins_of_alph_outside`
- `ruins_of_alph_research_center`

**Why together**: Route 32 is the long southbound route from Violet City. It contains
the mid-route PokemonCenter and the gate to Ruins of Alph. The Ruins Outside area is
the hub for all puzzle chambers. The Research Center is a simple interior on the hub.

**Prerequisites**: Group 7 (Zephyr Badge from Violet Gym needed to pass Route 32 gate)
**Complexity**: Medium -- Route 32 is the longest route with 8 trainers; Ruins hub is moderate
**Migration**: These maps exist in the old engine.

---

### Group 9: QA Audit -- Violet City Block
**QA Focus**: Test Groups 7-8
- Verify Violet Gym battle (Falkner) and badge award
- Verify Sprout Tower sage battles and TM Flash reward
- Verify Route 32 badge gate (requires Zephyr Badge)
- Verify Route 32 PokemonCenter (mid-route healing)
- Verify Ruins of Alph Outside hub warps
- Test Earl's Academy NPC dialogues

---

### Group 10: Ruins of Alph Chambers
**Areas (5):**
- `ruins_of_alph_kabuto_chamber`
- `ruins_of_alph_ho_oh_chamber`
- `ruins_of_alph_omanyte_chamber`
- `ruins_of_alph_aerodactyl_chamber`
- `ruins_of_alph_inner_chamber`

**Why together**: All five sub-areas of the Ruins of Alph puzzle complex. The four
puzzle chambers each lead to the Inner Chamber where Unown are encountered. Best built
as a unit since they share the same tileset and mechanics.

**Prerequisites**: Group 8 (Ruins of Alph Outside hub)
**Complexity**: Medium -- puzzle mechanics, Unown encounter system
**Migration**: Not in old engine (new content).

---

### Group 11: Union Cave + Route 33
**Areas (4):**
- `union_cave_1f`
- `union_cave_b1f`
- `union_cave_b2f`
- `route_33`

**Why together**: Union Cave is the mandatory transit dungeon between Route 32 and Route 33.
B1F and B2F are optional side areas (B2F requires Surf, available later). Route 33 is the
short connector to Azalea Town.

**Prerequisites**: Group 8 (Route 32)
**Complexity**: Medium -- 3-floor cave, 5 trainers on 1F, water areas on lower floors
**Migration**: These maps exist in the old engine.

---

### Group 12: QA Audit -- Ruins + Union Cave
**QA Focus**: Test Groups 10-11
- Verify all four Ruins puzzle chambers and puzzle completion
- Verify Inner Chamber Unown encounters
- Verify Union Cave 1F traversal (north-south passage)
- Verify Union Cave B1F/B2F exploration and Surf requirement on B2F
- Verify Route 33 exit to Azalea Town direction
- Test cave wild encounter tables

---

### Group 13: Azalea Town + Slowpoke Well
**Areas (5):**
- `azalea_town`
- `azalea_gym`
- `kurts_house`
- `slowpoke_well_b1f`
- `slowpoke_well_b2f`

**Why together**: Azalea Town is the second gym city. Slowpoke Well must be cleared
(defeat Rockets) before the gym is accessible in the story. Kurt's house is key for
Apricorn balls. The Charcoal Kiln is deferred to keep this group at 5 areas.

**Prerequisites**: Group 11 (Route 33)
**Complexity**: Medium -- Slowpoke Well has Rocket battles; Azalea Gym has spider web floor
**Migration**: These maps exist in the old engine.

---

### Group 14: Azalea Extras + Ilex Forest
**Areas (4):**
- `charcoal_kiln`
- `ilex_forest_azalea_gate`
- `ilex_forest`
- `route_34_ilex_forest_gate`

**Why together**: Charcoal Kiln completes Azalea Town interiors. The Ilex Forest is
the next mandatory area after Azalea Gym (requires Cut from Hive Badge). Both gates
bracket the forest.

**Prerequisites**: Group 13 (Azalea Gym / Hive Badge for Cut)
**Complexity**: Medium-Complex -- Ilex Forest is large (30x54) with Farfetch'd puzzle
**Migration**: Ilex Forest exists in old engine; gates may be new.

---

### Group 15: QA Audit -- Azalea Block
**QA Focus**: Test Groups 13-14
- Verify Slowpoke Well Rocket battles and story progression
- Verify Azalea Gym (Bugsy) battle and Hive Badge
- Verify Rival Battle 2 trigger at Azalea entrance
- Verify Cut usage in Ilex Forest
- Verify Farfetch'd puzzle in Ilex Forest
- Test Kurt's Apricorn ball crafting system
- Verify gate transitions (Azalea-Ilex, Ilex-Route 34)

---

### Group 16: Route 34 + Day Care + Goldenrod Arrival
**Areas (3):**
- `route_34`
- `day_care`
- `goldenrod_city`

**Why together**: Route 34 connects Ilex Forest to Goldenrod City. The Day Care is
on Route 34 and is a significant gameplay feature (breeding). Goldenrod City exterior
completes the travel path.

**Prerequisites**: Group 14 (Route 34 Ilex Forest Gate)
**Complexity**: Medium -- Route 34 has 9 trainers; Goldenrod is the largest city
**Migration**: These maps exist in the old engine.

---

### Group 17: Goldenrod Core Buildings
**Areas (4):**
- `goldenrod_gym`
- `goldenrod_dept_store`
- `goldenrod_game_corner`
- `goldenrod_underground`

**Why together**: The four most important Goldenrod interiors. The gym gives Plain Badge
(needed for Strength and SquirtBottle). The Dept Store is a major shopping location.
The Game Corner and Underground provide additional content.

**Prerequisites**: Group 16 (Goldenrod City exterior)
**Complexity**: Complex -- Whitney's gym has flower maze; Dept Store is multi-floor;
Underground is large
**Migration**: These maps exist in the old engine.

---

### Group 18: QA Audit -- Goldenrod Block
**QA Focus**: Test Groups 16-17
- Verify Route 34 trainer gauntlet and Day Care functionality
- Verify Goldenrod City warp destinations (all buildings accessible)
- Verify Goldenrod Gym (Whitney) battle, crying mechanic, badge
- Verify Dept Store floor navigation and shopping
- Verify Game Corner slot machine/card flip
- Verify Underground passage (north-south Goldenrod connector)
- Test SquirtBottle availability after Plain Badge

---

### Group 19: Goldenrod Secondary + Radio Tower
**Areas (5):**
- `goldenrod_underground_warehouse`
- `goldenrod_magnet_train_station`
- `radio_tower_1f`
- `radio_tower_2f`
- `radio_tower_3f`

**Why together**: The remaining Goldenrod interiors. Radio Tower floors 1-3 are built
now (the Rocket takeover event happens later, but the normal Radio Tower content is
available from this point). The Underground Warehouse connects to Radio Tower events.
Magnet Train Station is post-game but the building exists.

**Prerequisites**: Group 17 (Goldenrod core)
**Complexity**: Medium -- Radio Tower floors are similar layouts; Warehouse has Rocket trainers
**Migration**: Radio Tower may be partially in old engine.

---

### Group 20: Radio Tower Upper + National Park
**Areas (5):**
- `radio_tower_4f`
- `radio_tower_5f`
- `route_35_goldenrod_gate`
- `route_35`
- `route_35_national_park_gate`

**Why together**: Completes Radio Tower (all 5 floors ready for both normal and Rocket
takeover states). Route 35 and its gates connect Goldenrod northward to National Park.

**Prerequisites**: Group 19 (Radio Tower lower); Group 16 (Goldenrod exterior for Route 35)
**Complexity**: Medium -- Radio Tower upper floors follow same pattern; Route 35 has 9 trainers
**Migration**: Route 35 may exist in old engine.

---

### Group 21: QA Audit -- Goldenrod Extended
**QA Focus**: Test Groups 19-20
- Verify Radio Tower all 5 floors (normal state)
- Verify Underground Warehouse access
- Verify Magnet Train Station (blocked until post-game Pass)
- Verify Route 35 traversal and trainer battles
- Verify gate transitions (Goldenrod-Route 35, Route 35-National Park)
- Test Radio Card quiz on 1F

---

### Group 22: National Park + Route 36
**Areas (5):**
- `national_park`
- `national_park_bug_contest`
- `route_36_national_park_gate`
- `route_36`
- `route_36_ruins_of_alph_gate`

**Why together**: National Park (both normal and Bug Contest variants) plus Route 36
which connects the park to Violet City and Route 37. The Ruins of Alph gate on Route 36
provides alternate access to the Ruins.

**Prerequisites**: Group 20 (Route 35 National Park Gate)
**Complexity**: Medium-Complex -- Bug Contest requires special encounter/capture rules;
Route 36 has Sudowoodo gate
**Migration**: National Park may exist in old engine.

---

### Group 23: Route 37 + Ecruteak City
**Areas (3):**
- `route_37`
- `ecruteak_city`
- `ecruteak_gym`

**Why together**: Route 37 is the short connector from Route 36 to Ecruteak City.
Ecruteak is the fourth gym city. The gym (Morty's invisible floor puzzle) is the primary
gameplay target.

**Prerequisites**: Group 22 (Route 36 -- requires SquirtBottle to pass Sudowoodo)
**Complexity**: Complex -- Ecruteak Gym has invisible floor puzzle with many self-warps
**Migration**: Ecruteak City exists in old engine.

---

### Group 24: QA Audit -- National Park through Ecruteak
**QA Focus**: Test Groups 22-23
- Verify National Park exploration and NPC interactions
- Verify Bug-Catching Contest (Tue/Thu/Sat activation, special encounters, scoring)
- Verify Sudowoodo blocking Route 36 without SquirtBottle
- Verify Sudowoodo battle/removal with SquirtBottle
- Verify Route 37 traversal
- Verify Ecruteak City arrival and building access
- Verify Ecruteak Gym invisible floor puzzle and Morty battle

---

### Group 25: Ecruteak Interiors + Burned Tower
**Areas (5):**
- `dance_theater`
- `wise_trios_room`
- `tin_tower_entrance`
- `burned_tower_1f`
- `burned_tower_b1f`

**Why together**: Ecruteak's key story buildings. The Dance Theater (Kimono Girls) is a
significant side quest. The Burned Tower is mandatory -- falling through the floor
triggers the legendary beasts' release. Wise Trio's Room and the Tin Tower entrance
set up the legendary quest.

**Prerequisites**: Group 23 (Ecruteak City exterior)
**Complexity**: Complex -- Burned Tower has floor-collapse mechanic and legendary beast event;
Dance Theater has 5 Kimono Girl battles
**Migration**: Burned Tower exists in old engine.

---

### Group 26: Tin Tower (Lower Half)
**Areas (5):**
- `tin_tower_1f`
- `tin_tower_2f`
- `tin_tower_3f`
- `tin_tower_4f`
- `tin_tower_5f`

**Why together**: First five floors of Tin Tower (Bell Tower). This is a 9-floor dungeon
best built in two halves. Requires Clear Bell item (from Radio Tower event) to access
in the story, but the physical maps can be built now.

**Prerequisites**: Group 25 (Tin Tower entrance)
**Complexity**: Medium -- repetitive floor layouts with increasing puzzle complexity
**Migration**: Not in old engine (new content).

---

### Group 27: QA Audit -- Ecruteak Block
**QA Focus**: Test Groups 25-26
- Verify Burned Tower floor-collapse and legendary beast release event
- Verify Rival Battle 3 in Burned Tower
- Verify Kimono Girl battles in Dance Theater
- Verify Wise Trio's Room dialogue and Tin Tower access control
- Verify Tin Tower floors 1-5 traversal, items, warp connections
- Test Fog Badge (Morty) enables Surf HM usage

---

### Group 28: Tin Tower (Upper Half + Roof)
**Areas (5):**
- `tin_tower_6f`
- `tin_tower_7f`
- `tin_tower_8f`
- `tin_tower_9f`
- `tin_tower_roof`

**Why together**: Upper five floors plus roof of Tin Tower. Floor 7 has the most complex
puzzle. The roof is the Ho-Oh encounter location.

**Prerequisites**: Group 26 (Tin Tower lower half)
**Complexity**: Complex -- Floor 7 warp puzzle, Ho-Oh legendary battle on roof
**Migration**: Not in old engine (new content).

---

### Group 29: Route 38 + Route 39 + Moo Moo Farm
**Areas (5):**
- `route_38_ecruteak_gate`
- `route_38`
- `route_39`
- `route_39_barn`
- `route_39_farmhouse`

**Why together**: The westbound path from Ecruteak to Olivine. Route 38 and 39 connect
the two cities. The Moo Moo Farm (barn + farmhouse) is on Route 39 and has the MooMoo
Milk quest.

**Prerequisites**: Group 23 (Ecruteak City)
**Complexity**: Medium -- two routes with trainers, farm sidequest
**Migration**: These maps exist in the old engine.

---

### Group 30: QA Audit -- Tin Tower + Routes to Olivine
**QA Focus**: Test Groups 28-29
- Verify Tin Tower floors 6-9 puzzle mechanics and warp connections
- Verify Ho-Oh encounter on Tin Tower Roof (Clear Bell required)
- Verify Route 38/39 traversal and trainer battles
- Verify Moo Moo Farm sidequest (sick Miltank, MooMoo Milk reward)
- Verify gate transitions (Ecruteak-Route 38)
- Test route connectivity end-to-end (Ecruteak to Olivine direction)

---

### Group 31: Olivine City + Lighthouse (Lower)
**Areas (5):**
- `olivine_city`
- `olivine_gym`
- `olivine_cafe`
- `olivine_lighthouse_1f`
- `olivine_lighthouse_2f`

**Why together**: Olivine City exterior and its most important interiors. The gym cannot
be fought until medicine is delivered (story gate), but the building is accessible. The
lighthouse is the key story location -- lower floors start the climb.

**Prerequisites**: Group 29 (Route 39)
**Complexity**: Medium -- Olivine Gym is simple; lighthouse floors have trainer battles
**Migration**: These maps exist in the old engine (this is the migration boundary).

---

### Group 32: Olivine Lighthouse (Upper) + Port
**Areas (5):**
- `olivine_lighthouse_3f`
- `olivine_lighthouse_4f`
- `olivine_lighthouse_5f`
- `olivine_lighthouse_6f`
- `olivine_port`

**Why together**: Upper lighthouse floors lead to Jasmine and sick Ampharos on 6F. The
port provides access to the S.S. Aqua (post-game) and is part of Olivine's identity.

**Prerequisites**: Group 31 (Olivine City + Lighthouse 1F-2F)
**Complexity**: Medium -- 4 lighthouse floors follow similar patterns; port is moderate
**Migration**: Lighthouse upper floors may exist in old engine.

---

### Group 33: QA Audit -- Olivine Block
**QA Focus**: Test Groups 31-32
- Verify Olivine City building access (all warps)
- Verify lighthouse full climb (6 floors) and Jasmine/Ampharos event
- Verify Olivine Gym blocked until medicine delivery
- Verify Olivine Cafe and Port
- Verify connection from Route 39 into Olivine
- Test Surf availability from Olivine (south to Routes 40-41)

---

### Group 34: Sea Routes + Cianwood City
**Areas (5):**
- `route_40`
- `route_41`
- `cianwood_city`
- `cianwood_gym`
- `cianwood_pharmacy`

**Why together**: The water routes from Olivine to Cianwood (requiring Surf) and Cianwood
itself. Chuck's gym gives Storm Badge (Fly). The pharmacy provides SecretPotion to heal
Jasmine's Ampharos.

**Prerequisites**: Group 31 (Olivine City); Fog Badge (Morty) for Surf
**Complexity**: Complex -- large water routes with Swimmer trainers; Cianwood Gym has
waterfall-and-strength puzzle
**Migration**: Not in old engine (new content).

---

### Group 35: Route 42 + Mahogany Town
**Areas (4):**
- `route_42_ecruteak_gate`
- `route_42`
- `mahogany_town`
- `mahogany_gym`

**Why together**: The eastbound path from Ecruteak to Mahogany via Route 42. Mahogany Town
is the seventh gym city. The gym cannot be fought until the Rocket Hideout is cleared,
but the town exterior and gym building are built here.

**Prerequisites**: Group 23 (Ecruteak City); Fog Badge for Surf (Route 42 has water sections)
**Complexity**: Medium -- Route 42 has water segments; Mahogany Gym has ice floor puzzle
**Migration**: Not in old engine (new content).

---

### Group 36: QA Audit -- Cianwood + Mahogany
**QA Focus**: Test Groups 34-35
- Verify Surf requirement on Routes 40-41
- Verify Cianwood Gym (Chuck) battle and Storm Badge
- Verify Cianwood Pharmacy SecretPotion acquisition
- Verify medicine delivery to Jasmine and Olivine Gym unlock
- Verify Olivine Gym (Jasmine) battle and Mineral Badge
- Verify Route 42 traversal (Ecruteak to Mahogany)
- Verify Mahogany Town arrival and building access
- Test Fly HM after Storm Badge

---

### Group 37: Route 43 + Lake of Rage + Rocket Hideout
**Areas (5):**
- `route_43_mahogany_gate`
- `route_43`
- `route_43_gate`
- `lake_of_rage`
- `rocket_hideout_b1f`

**Why together**: The northbound path from Mahogany to Lake of Rage (Red Gyarados event)
and the first floor of the Rocket Hideout, which opens after the lake event. The Route 43
Rocket toll gate is a unique scripted event.

**Prerequisites**: Group 35 (Mahogany Town)
**Complexity**: Complex -- Lake of Rage has Red Gyarados event; Rocket Hideout has traps
and Lance NPC ally
**Migration**: Not in old engine (new content).

---

### Group 38: Rocket Hideout (Remaining) + Mt. Mortar
**Areas (5):**
- `rocket_hideout_b2f`
- `rocket_hideout_b3f`
- `mt_mortar_1f_outside`
- `mt_mortar_1f_inside`
- `mt_mortar_2f`

**Why together**: Completing the Rocket Hideout dungeon (B2F and B3F have password puzzles
and the Executive boss). Mt. Mortar is the optional dungeon on Route 42 -- built here
because it becomes relevant after Mahogany is reached.

**Prerequisites**: Group 37 (Rocket Hideout B1F); Group 35 (Route 42 for Mt. Mortar access)
**Complexity**: Complex -- Rocket Hideout has switch/password puzzles; Mt. Mortar is
large multi-floor cave
**Migration**: Not in old engine (new content).

---

### Group 39: QA Audit -- Lake of Rage + Rocket Subplot
**QA Focus**: Test Groups 37-38
- Verify Route 43 Rocket toll gate event
- Verify Lake of Rage Red Gyarados encounter and event trigger
- Verify Lance meeting at Lake of Rage
- Verify Rocket Hideout all 3 floors (traps, passwords, Lance ally)
- Verify Rocket Executive battle and hideout clearance
- Verify Mahogany Gym unlocks after Rocket Hideout
- Verify Mahogany Gym (Pryce) battle and Glacier Badge
- Verify Mt. Mortar exploration (Karate King, Tyrogue)

---

### Group 40: Mt. Mortar B1F + Route 44 + Ice Path Start
**Areas (4):**
- `mt_mortar_b1f`
- `route_44`
- `ice_path_1f`
- `ice_path_b1f`

**Why together**: Completes Mt. Mortar (the deepest floor with Karate King). Route 44
is the penultimate Johto route leading to Ice Path. Ice Path 1F and B1F are the entry
floors of the Strength/ice puzzle dungeon.

**Prerequisites**: Group 38 (Mt. Mortar upper floors); Group 35 (Mahogany for Route 44);
Radio Tower clearance story flag for full Route 44 access
**Complexity**: Complex -- Ice Path has Strength boulder puzzles and ice sliding
**Migration**: Not in old engine (new content).

---

### Group 41: Ice Path (Lower) + Blackthorn City
**Areas (5):**
- `ice_path_b2f_mahogany`
- `ice_path_b2f_blackthorn`
- `ice_path_b3f`
- `blackthorn_city`
- `blackthorn_gym_1f`

**Why together**: The deeper Ice Path floors and emergence into Blackthorn City. The gym
1F is included because players will go straight there.

**Prerequisites**: Group 40 (Ice Path upper floors)
**Complexity**: Complex -- ice sliding puzzles on B2F/B3F; Blackthorn Gym has lava/dragon theme
**Migration**: Not in old engine (new content).

---

### Group 42: QA Audit -- Ice Path + Blackthorn
**QA Focus**: Test Groups 40-41
- Verify Ice Path all 5 floors (Strength boulders, ice sliding, waterfall)
- Verify Route 44 traversal and trainers
- Verify Blackthorn City arrival and building access
- Verify Blackthorn Gym 1F trainer battles
- Verify Mt. Mortar B1F Karate King battle and Tyrogue reward
- Test full Ice Path traversal end-to-end

---

### Group 43: Blackthorn Gym 2F + Dragon's Den
**Areas (4):**
- `blackthorn_gym_2f`
- `dragons_den_1f`
- `dragons_den_b1f`
- `dragon_shrine`

**Why together**: Completes the Blackthorn story arc. Gym 2F has the Strength puzzle
that leads to Clair. Dragon's Den is the mandatory post-battle area where Clair
refuses the badge until the player passes the Dragon Shrine quiz.

**Prerequisites**: Group 41 (Blackthorn City + Gym 1F)
**Complexity**: Complex -- Gym 2F Strength puzzle; Dragon's Den has Surf + Whirlpool;
Dragon Shrine has quiz mechanic
**Migration**: Not in old engine (new content).

---

### Group 44: Route 45 + Route 46 + Dark Cave
**Areas (4):**
- `route_45`
- `route_46`
- `dark_cave_violet_entrance`
- `dark_cave_blackthorn_entrance`

**Why together**: The south-bound mountain routes from Blackthorn. Route 45 is one-way
(ledges down), connecting to Route 46 and eventually back to Route 29. Dark Cave has
entrances from both Route 31 (built earlier) and Route 45 (Blackthorn side), plus an
internal connection between the two halves.

**Prerequisites**: Group 41 (Blackthorn City for Route 45); Group 5 (Route 31 for Dark Cave
Violet entrance -- already built)
**Complexity**: Medium -- routes are linear (one-way ledges); Dark Cave is moderate
**Migration**: Not in old engine (new content).

---

### Group 45: QA Audit -- Blackthorn Complete + Southern Routes
**QA Focus**: Test Groups 43-44
- Verify Blackthorn Gym 2F Strength puzzle
- Verify Clair battle and badge refusal
- Verify Dragon's Den Surf/Whirlpool navigation
- Verify Dragon Shrine quiz and Rising Badge award
- Verify Route 45 one-way ledge descent
- Verify Route 46 connects back to Route 29 area
- Verify Dark Cave both entrances and internal connection
- Test all 8 badges collected

---

### Group 46: Whirl Islands
**Areas (4):**
- `whirl_islands_nw`
- `whirl_islands_ne`
- `whirl_islands_sw`
- `whirl_islands_se`

**Why together**: The four entrance caves of the Whirl Islands complex, accessed from
Route 41 (already built). Requires Whirlpool (Glacier Badge from Pryce). These are
the surface-level entrances that lead to the shared underground.

**Prerequisites**: Group 34 (Route 41); Glacier Badge for Whirlpool
**Complexity**: Medium -- four small cave entrances with whirlpool gates
**Migration**: Not in old engine (new content).

---

### Group 47: Whirl Islands Interior + Lugia
**Areas (4):**
- `whirl_islands_cave`
- `whirl_islands_b1f`
- `whirl_islands_b2f`
- `whirl_islands_lugia_chamber`

**Why together**: The underground floors of the Whirl Islands leading to the Lugia
encounter. B1F is the main connector floor. The Lugia Chamber is the deepest point
and the encounter location.

**Prerequisites**: Group 46 (Whirl Islands entrances)
**Complexity**: Complex -- multi-entrance dungeon navigation; Lugia legendary battle
**Migration**: Not in old engine (new content).

---

### Group 48: QA Audit -- Whirl Islands
**QA Focus**: Test Groups 46-47
- Verify all four Whirl Islands entrances accessible from Route 41
- Verify Whirlpool requirement at entrances
- Verify B1F connects all four entrances
- Verify navigation to Lugia Chamber via NW entrance path
- Verify Lugia legendary encounter
- Verify Silver Wing / Tidal Bell requirement
- Test full Whirl Islands traversal

---

### Group 49: Victory Road Approach (Routes 27 + 26)
**Areas (5):**
- `route_27`
- `route_27_sandstorm_house`
- `tohjo_falls`
- `route_26`
- `route_26_heal_house`

**Why together**: The final routes to the Pokemon League. Route 27 (Surf east from
New Bark Town) through Tohjo Falls to Route 26. Requires all 8 badges to pass the gate
guards. Both heal houses provide rest stops on the long journey.

**Prerequisites**: All 8 badges; Group 1 (New Bark Town for Route 27 west connection)
**Complexity**: Medium -- long routes with trainers; Tohjo Falls has Waterfall requirement
**Migration**: Not in old engine (new content).

---

### Group 50: Victory Road
**Areas (1):**
- `victory_road`

**Why together**: Victory Road is a single large cave map (20x72 tiles) but is dense
enough to warrant its own sprint. It has multi-level navigation with Strength puzzles
and the Rival Battle 5.

**Prerequisites**: Group 49 (Route 26)
**Complexity**: Complex -- large multi-section cave with Strength/Waterfall puzzles
**Migration**: Not in old engine (new content).

---

### Group 51: QA Audit -- Victory Road Approach
**QA Focus**: Test Groups 49-50
- Verify all-8-badges gate check on Route 27
- Verify Route 27 Surf requirements and trainer battles
- Verify Tohjo Falls Waterfall usage
- Verify Route 26 traversal and heal house
- Verify Victory Road full traversal (all sections, puzzles)
- Verify Rival Battle 5 trigger in Victory Road
- Test end-to-end journey from New Bark Town to Indigo Plateau

---

### Group 52: Indigo Plateau + Elite Four
**Areas (4):**
- `indigo_plateau_pokecenter`
- `wills_room`
- `kogas_room`
- `brunos_room`

**Why together**: The Indigo Plateau PokemonCenter (last healing point) and the first
three Elite Four rooms. Will, Koga, and Bruno are fought in sequence.

**Prerequisites**: Group 50 (Victory Road)
**Complexity**: Complex -- three boss battles in sequence; E4 room transitions
**Migration**: Not in old engine (new content).

---

### Group 53: Karen + Lance + Hall of Fame
**Areas (3):**
- `karens_room`
- `lances_room`
- `hall_of_fame`

**Why together**: The final two Elite Four members and the Hall of Fame. Karen is E4 #4,
Lance is the Champion. The Hall of Fame is the game's victory sequence.

**Prerequisites**: Group 52 (E4 rooms 1-3)
**Complexity**: Complex -- Champion battle is the most challenging; Hall of Fame is a
special victory screen/sequence
**Migration**: Not in old engine (new content).

---

### Group 54: QA Audit -- Elite Four + Game Completion
**QA Focus**: Test Groups 52-53
- Verify Indigo Plateau PokemonCenter (last save point)
- Verify all 5 E4/Champion battles in sequence (Will, Koga, Bruno, Karen, Lance)
- Verify E4 room-to-room transitions (no skipping)
- Verify team restoration between E4 members is NOT allowed
- Verify Hall of Fame sequence triggers after Champion defeat
- Verify credits/post-game flag is set
- Test game completion end-to-end from Indigo Plateau PokemonCenter

---

## 6. Technical Migration Notes

### Existing Maps (Groups 1-31)

The first 33 maps through Olivine City were built in a previous agentic experiment.
Each sprint that touches these areas must handle migration:

1. **Tile layout migration**: The old maps used a different tile layout format. Each
   area's `tile_layout.rows` in the new JSON must be reconciled with whatever the
   engine currently stores. The new JSON data is canonical -- old layouts should be
   updated to match.

2. **Warp coordinate verification**: Old warp coordinates may not match the new JSON
   data (which was derived from pokecrystal-master). Each warp must be verified against
   the JSON `warps` array.

3. **NPC and trainer data**: The old maps had placeholder NPC data. The new JSON files
   have accurate NPC positions, dialogue summaries, and trainer teams derived from
   pokecrystal. This data should replace any old placeholders.

4. **Connection offsets**: Map connections (north/south/east/west border stitching)
   have specific offsets in the new JSON. These must match the engine's connection
   system.

5. **Wild encounter tables**: The new JSON has time-of-day encounter tables. The old
   engine may not have time-of-day support yet -- migrate encounter data but use the
   "day" table as default until time-of-day is implemented.

### Migration-Affected Area IDs

The following areas likely exist in the old engine and need migration rather than
fresh creation:

```
new_bark_town, players_house_1f, players_house_2f, elms_lab
route_29, cherrygrove_city
route_30, mr_pokemons_house
route_31, route_31_violet_gate, violet_city
violet_gym, earls_pokemon_academy, sprout_tower_1f, sprout_tower_2f, sprout_tower_3f
route_32, route_32_pokecenter
union_cave_1f, union_cave_b1f
route_33
azalea_town, azalea_gym, kurts_house, slowpoke_well_b1f
ilex_forest
route_34, day_care
goldenrod_city, goldenrod_gym, goldenrod_dept_store, goldenrod_game_corner
goldenrod_underground
route_35
national_park
route_36, route_37
ecruteak_city, ecruteak_gym
burned_tower_1f, burned_tower_b1f
route_38, route_39
olivine_city, olivine_gym, olivine_lighthouse_1f through olivine_lighthouse_6f
```

### New Content (Groups 32+)

Areas from Cianwood City onward are entirely new content with no migration concerns:

```
route_40, route_41, cianwood_city, cianwood_gym, cianwood_pharmacy
route_42, mahogany_town, mahogany_gym
route_43, lake_of_rage
rocket_hideout_b1f through b3f
mt_mortar (all floors)
route_44, ice_path (all floors)
blackthorn_city, blackthorn_gym_1f, blackthorn_gym_2f
dragons_den_1f, dragons_den_b1f, dragon_shrine
route_45, route_46, dark_cave (both entrances)
whirl_islands (all 8 areas)
route_27, tohjo_falls, route_26
victory_road
indigo_plateau_pokecenter, E4 rooms, hall_of_fame
```

### Shared Template Interiors

The 67 warp targets without JSON files (listed in Section 3) should be created using
shared templates as each parent town/route is built. These do not count toward the
152-area total but must be registered in the engine's map system so warps resolve
correctly. Estimated effort: 1-2 areas per sprint as incidental work.

---

## Appendix: Area Count by Type

| Area Type | Count | Examples |
|-----------|-------|---------|
| town | 10 | new_bark_town, cherrygrove_city, violet_city, etc. |
| route | 22 | route_29 through route_46 |
| building | 38 | gyms, houses, gates, shops |
| cave | 21 | union_cave, ice_path, dark_cave, mt_mortar, etc. |
| dungeon | 18 | sprout_tower, burned_tower, rocket_hideout, tin_tower |
| special | 13 | ruins_of_alph chambers, national_park, E4 rooms |
| forest | 1 | ilex_forest |
| lake | 1 | lake_of_rage |
| **Total** | **152** | |

## Appendix: Full Area ID Sequence (Quick Reference)

```
  1. new_bark_town                     77. tin_tower_7f
  2. players_house_2f                  78. tin_tower_8f
  3. players_house_1f                  79. tin_tower_9f
  4. elms_lab                          80. tin_tower_roof
  5. route_29                          81. route_38_ecruteak_gate
  6. route_29_route_46_gate            82. route_38
  7. cherrygrove_city                  83. route_39
  8. route_30                          84. route_39_barn
  9. mr_pokemons_house                 85. route_39_farmhouse
 10. route_31                          86. olivine_city
 11. route_31_violet_gate              87. olivine_gym
 12. violet_city                       88. olivine_cafe
 13. violet_gym                        89. olivine_port
 14. earls_pokemon_academy             90. olivine_lighthouse_1f
 15. sprout_tower_1f                   91. olivine_lighthouse_2f
 16. sprout_tower_2f                   92. olivine_lighthouse_3f
 17. sprout_tower_3f                   93. olivine_lighthouse_4f
 18. route_32                          94. olivine_lighthouse_5f
 19. route_32_pokecenter               95. olivine_lighthouse_6f
 20. route_32_ruins_of_alph_gate       96. route_40
 21. ruins_of_alph_outside             97. route_41
 22. ruins_of_alph_research_center     98. cianwood_city
 23. ruins_of_alph_kabuto_chamber      99. cianwood_gym
 24. ruins_of_alph_ho_oh_chamber      100. cianwood_pharmacy
 25. ruins_of_alph_omanyte_chamber    101. route_42_ecruteak_gate
 26. ruins_of_alph_aerodactyl_chamber 102. route_42
 27. ruins_of_alph_inner_chamber      103. mahogany_town
 28. union_cave_1f                    104. mahogany_gym
 29. union_cave_b1f                   105. route_43_mahogany_gate
 30. union_cave_b2f                   106. route_43
 31. route_33                         107. route_43_gate
 32. azalea_town                      108. lake_of_rage
 33. azalea_gym                       109. rocket_hideout_b1f
 34. kurts_house                      110. rocket_hideout_b2f
 35. charcoal_kiln                    111. rocket_hideout_b3f
 36. slowpoke_well_b1f                112. mt_mortar_1f_outside
 37. slowpoke_well_b2f                113. mt_mortar_1f_inside
 38. ilex_forest_azalea_gate          114. mt_mortar_2f
 39. ilex_forest                      115. mt_mortar_b1f
 40. route_34_ilex_forest_gate        116. route_44
 41. route_34                         117. ice_path_1f
 42. day_care                         118. ice_path_b1f
 43. goldenrod_city                   119. ice_path_b2f_mahogany
 44. goldenrod_gym                    120. ice_path_b2f_blackthorn
 45. goldenrod_dept_store             121. ice_path_b3f
 46. goldenrod_game_corner            122. blackthorn_city
 47. goldenrod_underground            123. blackthorn_gym_1f
 48. goldenrod_underground_warehouse  124. blackthorn_gym_2f
 49. goldenrod_magnet_train_station   125. dragons_den_1f
 50. radio_tower_1f                   126. dragons_den_b1f
 51. radio_tower_2f                   127. dragon_shrine
 52. radio_tower_3f                   128. route_45
 53. radio_tower_4f                   129. route_46
 54. radio_tower_5f                   130. dark_cave_violet_entrance
 55. route_35_goldenrod_gate          131. dark_cave_blackthorn_entrance
 56. route_35                         132. whirl_islands_nw
 57. route_35_national_park_gate      133. whirl_islands_ne
 58. national_park                    134. whirl_islands_sw
 59. national_park_bug_contest        135. whirl_islands_se
 60. route_36_national_park_gate      136. whirl_islands_cave
 61. route_36                         137. whirl_islands_b1f
 62. route_36_ruins_of_alph_gate      138. whirl_islands_b2f
 63. route_37                         139. whirl_islands_lugia_chamber
 64. ecruteak_city                    140. route_27
 65. ecruteak_gym                     141. route_27_sandstorm_house
 66. dance_theater                    142. tohjo_falls
 67. wise_trios_room                  143. route_26
 68. tin_tower_entrance               144. route_26_heal_house
 69. burned_tower_1f                  145. victory_road
 70. burned_tower_b1f                 146. indigo_plateau_pokecenter
 71. tin_tower_1f                     147. wills_room
 72. tin_tower_2f                     148. kogas_room
 73. tin_tower_3f                     149. brunos_room
 74. tin_tower_4f                     150. karens_room
 75. tin_tower_5f                     151. lances_room
 76. tin_tower_6f                     152. hall_of_fame
```

All 152 unique area_ids from JSON files are accounted for in this build sequence.
No duplicates, no omissions -- verified programmatically against the JSON file set.
