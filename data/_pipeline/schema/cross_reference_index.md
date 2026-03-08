# Pokemon Crystal Vector DB — Cross-Reference Index

> Demonstrates how chunks link to each other via `related_entities`.
> Built from the 62 sample chunks in `sample_chunks.jsonl`.

---

## 1. Link Statistics

| Metric | Value |
|---|---|
| Total chunks | 62 |
| Total links (directional) | 228 |
| Average links per chunk | 3.7 |
| Max links on a single chunk | 7 |
| Orphan chunks (0 outgoing links) | 0 |
| Most-referenced chunk | `mechanic_damage_formula_001` (8 inbound) |

---

## 2. Hub Chunks (Most Connected)

These chunks are referenced by many others and serve as central nodes in the knowledge graph.

| Chunk ID | Inbound Links | Role |
|---|---|---|
| `mechanic_damage_formula_001` | 8 | Core damage formula — referenced by modifiers, STAB, weather, crits |
| `trainer_whitney_001` | 5 | Whitney battle — linked by Miltank, Rollout, Attract, strategy, Goldenrod |
| `species_miltank_001` | 4 | Miltank data — linked by Whitney, Rollout, Attract, strategy |
| `type_fire_offensive_001` | 3 | Fire matchups — linked by Charcoal, defensive pair, damage formula |
| `move_rollout_001` | 3 | Rollout — linked by Whitney, Miltank, strategy |
| `move_ice_beam_001` | 3 | Ice Beam — linked by Ice type, Lance strategy, freeze mechanic |
| `species_dragonite_001` | 3 | Dragonite — linked by Lance, Dragon type, evolution chain |

---

## 3. Cross-Reference Graph (Selected Clusters)

### Cluster: Whitney's Gym Battle

```
strategy_whitney_001
  ├── trainer_whitney_001
  │     ├── species_clefairy_001
  │     ├── species_miltank_001
  │     │     ├── move_rollout_001
  │     │     │     └── move_defense_curl_001
  │     │     ├── move_attract_001
  │     │     └── move_milk_drink_001
  │     ├── map_goldenrod_city_001
  │     └── story_violet_gym_001
  ├── species_machop_001 (counter)
  └── species_geodude_001 (counter)
        └── wild_dark_cave_grass_001
```

This cluster shows how a strategy question ("How do I beat Whitney?") can traverse from the strategy chunk → trainer data → individual Pokemon → individual moves, and also link to counter Pokemon and where to find them.

### Cluster: Damage Calculation

```
mechanic_damage_formula_001
  ├── mechanic_damage_modifiers_001
  │     ├── mechanic_critical_hit_001
  │     │     └── item_scope_lens_001
  │     └── mechanic_weather_rain_001
  │           └── move_rain_dance_001
  ├── mechanic_stab_001
  ├── type_fire_offensive_001
  │     └── item_charcoal_001
  └── mechanic_stat_stages_001
        └── move_swords_dance_001
```

A complex question like "How much damage does Typhlosion's Flamethrower do in rain?" requires traversing: damage formula → weather modifier → fire type effectiveness → STAB rules.

### Cluster: Champion Lance

```
trainer_lance_001
  ├── species_dragonite_001
  │     ├── evolution_dratini_001
  │     ├── type_dragon_offensive_001
  │     └── learnset_dragonite_level_001
  ├── species_gyarados_001
  ├── species_charizard_001
  ├── species_aerodactyl_001
  └── type_ice_offensive_001 (counter)
        ├── move_ice_beam_001
        └── move_blizzard_001
```

### Cluster: Early Game Route Progression

```
story_elm_starter_001
  ├── species_chikorita_001
  ├── species_cyndaquil_001
  ├── species_totodile_001
  └── map_new_bark_town_001
        └── wild_route29_grass_001
              ├── species_pidgey_001
              └── species_sentret_001

story_rival_first_001
  ├── trainer_rival_cherrygrove_001
  └── story_elm_starter_001

map_violet_city_001
  ├── trainer_falkner_001
  │     ├── species_pidgey_001
  │     └── species_pidgeotto_001
  ├── wild_sprout_tower_001
  └── map_route_31_001
```

### Cluster: Evolution Mechanics

```
evolution_bulbasaur_001
  ├── species_bulbasaur_001
  │     ├── learnset_bulbasaur_level_001
  │     ├── learnset_bulbasaur_tmhm_001
  │     └── learnset_bulbasaur_egg_001
  ├── species_ivysaur_001
  └── species_venusaur_001

item_kings_rock_001
  ├── evolution_poliwhirl_001 (trade evo)
  └── evolution_slowpoke_001 (trade evo)

item_thunderstone_001
  └── species_pikachu_001 → species_raichu_001
```

---

## 4. Multi-Hop Query Examples

### Query: "Can Feraligatr learn Ice Beam?"

```
Hop 1: Search "Feraligatr Ice Beam learnset" → learnset_feraligatr_tmhm_001
Hop 2: Follow related_entities → move_ice_beam_001 (confirms move details)
Answer: Yes, via move tutor in Goldenrod Game Corner (Crystal only)
```

### Query: "What's super effective against Lance's strongest Pokemon?"

```
Hop 1: Search "Lance team" → trainer_lance_001 (identifies Lv50 Dragonite as ace)
Hop 2: Follow related_entities → species_dragonite_001 (Dragon/Flying typing)
Hop 3: Follow related_entities → type_dragon_offensive_001 or search "Dragon Flying weaknesses"
        → type_ice_offensive_001 (Ice is 4x effective)
Answer: Ice (4x), Dragon (2x), Rock (2x)
```

### Query: "Where can I get a Pokemon to counter Whitney before Goldenrod?"

```
Hop 1: Search "Whitney counter" → strategy_whitney_001 (recommends female Geodude, Machop trade)
Hop 2: Follow related_entities → wild_dark_cave_grass_001 (Geodude location)
Hop 3: Follow related_entities → map_goldenrod_city_001 (Machop trade in Dept Store)
Answer: Geodude from Dark Cave (Route 31), or trade Drowzee for Machop in Goldenrod Dept Store
```

### Query: "How does weather affect the damage formula?"

```
Hop 1: Search "weather damage formula" → mechanic_damage_formula_001 (lists weather as modifier)
Hop 2: Follow related_entities → mechanic_damage_modifiers_001 (weather detail)
Hop 3: Follow related_entities → mechanic_weather_rain_001 (specific rain effects)
Answer: Rain = Water 1.5x, Fire 0.5x. Sun = Fire 1.5x, Water 0.5x.
```

---

## 5. Relationship Type Distribution

| Relationship | Count in Sample | Example |
|---|---|---|
| species → learnset | 8 | species_bulbasaur → learnset_bulbasaur_level |
| species → evolution | 6 | species_bulbasaur → evolution_bulbasaur |
| trainer → species | 18 | trainer_lance → species_dragonite |
| trainer → map | 7 | trainer_falkner → map_violet_city |
| move → type_interaction | 9 | move_thunderbolt → type_electric_offensive |
| mechanic → mechanic | 8 | mechanic_damage_formula → mechanic_critical_hit |
| strategy → trainer | 2 | strategy_whitney → trainer_whitney |
| strategy → species | 4 | strategy_whitney → species_machop |
| wild_encounter → species | 10 | wild_route29_grass → species_pidgey |
| wild_encounter → map | 4 | wild_route29_grass → map_route_29 |
| item → species | 3 | item_light_ball → species_pikachu |
| item → evolution | 2 | item_kings_rock → evolution_poliwhirl |
| story → map | 3 | story_radio_tower → map_goldenrod_city |
| story → trainer | 2 | story_rival_first → trainer_rival_cherrygrove |
| type → move | 8 | type_fire_offensive → move_flamethrower |
| type → type (pair) | 6 | type_fire_offensive → type_fire_defensive |

---

## 6. Bidirectional Link Verification

For production, every forward link should have an inverse. Sample verification:

| Forward Link | Expected Inverse | Present? |
|---|---|---|
| `trainer_whitney_001` → `species_miltank_001` | `species_miltank_001` → `trainer_whitney_001` | Yes |
| `species_dragonite_001` → `trainer_lance_001` | `trainer_lance_001` → `species_dragonite_001` | Yes |
| `mechanic_damage_formula_001` → `mechanic_stab_001` | `mechanic_stab_001` → `mechanic_damage_formula_001` | Yes |
| `move_rollout_001` → `trainer_whitney_001` | `trainer_whitney_001` → `move_rollout_001` | Yes |
| `item_charcoal_001` → `type_fire_offensive_001` | `type_fire_offensive_001` → `item_charcoal_001` | Yes |

All sampled bidirectional links verified. The ingestion pipeline (below) includes automated bidirectional link validation.
