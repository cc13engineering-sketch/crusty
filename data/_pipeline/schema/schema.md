# Pokemon Crystal Vector DB — Master Schema

> Complete schema for a RAG vector database covering every aspect of Pokemon Crystal.
> Designed for maximum retrieval accuracy on arbitrary natural-language questions.

---

## 1. Embedding Model & Distance Metric

### Recommended Model
**Primary**: `text-embedding-3-large` (OpenAI, 3072 dimensions)
- Best balance of quality, cost, and ecosystem support
- Supports dimension reduction via `dimensions` parameter (1536 for cost savings)
- Strong performance on domain-specific factual recall

**Alternative**: `voyage-3-large` (Voyage AI, 1024 dimensions)
- Superior on code/technical content; useful if embedding raw ASM snippets

### Distance Metric
**Cosine similarity** — the standard for normalized text embeddings.
- All major embedding models produce L2-normalized vectors; cosine = dot product
- Works well for heterogeneous content (short facts vs. long mechanical explanations)
- Configure index as `cosine` in Pinecone/Qdrant/Weaviate, or `ip` (inner product) if vectors are pre-normalized

### Dimensionality
- Use **1536 dimensions** (text-embedding-3-large with reduction) for production
- Use full **3072** only if retrieval precision on edge-case mechanics questions is insufficient at 1536

---

## 2. Document Types

Each chunk in the vector DB has a `doc_type` field. There are **15 primary document types**:

| doc_type | Description | Typical chunk size |
|---|---|---|
| `species` | Pokemon base stats, types, abilities, catch rate, growth rate, egg groups | 200-400 tokens |
| `learnset` | Level-up moves, TM/HM compatibility, egg moves for one species | 300-600 tokens |
| `evolution` | Evolution chain for one family (methods, levels, items, conditions) | 150-300 tokens |
| `move` | Single move: type, power, accuracy, PP, effect description, priority | 150-350 tokens |
| `item` | Single item: effect, location(s), price, category | 150-300 tokens |
| `trainer` | One trainer battle: name, class, location, team (species/levels/moves/items) | 300-600 tokens |
| `map_location` | One map area: connections, landmarks, wild encounters, items, NPCs | 400-800 tokens |
| `wild_encounter` | Encounter table for one area: species, levels, rates by time-of-day | 200-500 tokens |
| `mechanic` | Battle mechanic rule: damage formula, stat stages, accuracy check, weather, etc. | 500-1000 tokens |
| `battle_rule` | Specific interaction rule: move effect resolution, priority bracket, switching | 300-700 tokens |
| `type_interaction` | Type chart entry or set of entries (offensive/defensive for one type) | 200-400 tokens |
| `story_event` | Story progression step: what happens, prerequisites, flags set | 200-500 tokens |
| `strategy` | Competitive/in-game strategy: team building, matchup advice, boss fight tips | 300-700 tokens |
| `game_corner` | Mini-games, side features: Bug Catching Contest, Pokegear, radio, phone | 300-600 tokens |
| `music_art` | Music tracks, visual design notes, sprite info | 150-300 tokens |

---

## 3. Required Metadata Fields

Every chunk MUST have these fields:

### Universal Fields (all doc_types)

```yaml
id:               string    # Unique chunk ID: "{doc_type}_{entity_id}_{chunk_seq}"
                            # e.g. "species_bulbasaur_001", "move_thunderbolt_001"
doc_type:         string    # One of the 15 types above
name:             string    # Human-readable name: "Bulbasaur", "Thunderbolt", "Route 29"
category:         string    # Top-level domain (see taxonomy.md)
subcategory:      string    # Second-level domain (see taxonomy.md)
tags:             string[]  # Standardized tag vocabulary (see taxonomy.md)
related_entities: string[]  # IDs of related chunks for cross-referencing
source:           string    # Origin: "pokecrystal", "bulbapedia", "serebii", "derived"
source_file:      string    # Path within pokecrystal-master if applicable
                            # e.g. "data/pokemon/base_stats/bulbasaur.asm"
generation:       int       # Always 2 for Crystal
game:             string    # "crystal" (or "gold_silver" for shared content)
text:             string    # The chunk text to be embedded
```

### Type-Specific Metadata

#### `species`
```yaml
dex_number:       int       # National dex number (1-251)
type1:            string    # Primary type
type2:            string    # Secondary type (or "none")
base_stats:       object    # {hp, atk, def, spd, spa, spd}
catch_rate:       int       # 0-255
growth_rate:      string    # "fast", "medium_fast", "medium_slow", "slow"
egg_groups:       string[]  # e.g. ["monster", "plant"]
gender_ratio:     string    # e.g. "87.5% male", "genderless"
```

#### `learnset`
```yaml
species_id:       string    # Links to parent species chunk
learnset_type:    string    # "level_up", "tm_hm", "egg_move", "tutor"
move_list:        string[]  # Move names in this learnset
```

#### `evolution`
```yaml
species_id:       string    # Base species in chain
evo_method:       string    # "level", "item", "trade", "happiness", "stat"
evo_family:       string[]  # All species in chain: ["Bulbasaur", "Ivysaur", "Venusaur"]
```

#### `move`
```yaml
move_type:        string    # Element type: "fire", "water", etc.
move_category:    string    # "physical" or "special" (determined by type in Gen 2)
power:            int       # Base power (0 for status moves)
accuracy:         int       # 1-100 percent (0 = always hits)
pp:               int       # Base PP
effect:           string    # Effect constant: "EFFECT_NORMAL_HIT", "EFFECT_BURN_HIT", etc.
effect_chance:    int       # Secondary effect chance percent (0 = N/A)
priority:         int       # Move priority bracket (-6 to +5)
```

#### `item`
```yaml
item_pocket:      string    # "balls", "items", "key_items", "tm_hm"
item_effect:      string    # Brief effect: "restores 20 HP", "boosts fire moves 10%"
item_price:       int       # Buy price in Pokedollars
item_locations:   string[]  # Where to obtain: ["Goldenrod Dept Store", "Route 34"]
holdable:         bool      # Can be held by a Pokemon
```

#### `trainer`
```yaml
trainer_class:    string    # "gym_leader", "elite_four", "champion", "rival", "route_trainer"
trainer_location: string    # Map where encountered
badge:            string    # Badge awarded (gym leaders only)
team_size:        int       # Number of Pokemon
team_levels:      int[]     # Levels of each Pokemon
rematch:          bool      # Is this a rematch/later encounter
```

#### `map_location`
```yaml
region:           string    # "johto" or "kanto"
map_type:         string    # "city", "town", "route", "dungeon", "building", "cave"
connections:      string[]  # Adjacent map names
has_pokecenter:   bool
has_pokemart:     bool
has_gym:          bool
gym_leader:       string    # Name if applicable
required_hms:     string[]  # HMs needed to fully explore
```

#### `wild_encounter`
```yaml
location_id:      string    # Links to parent map
encounter_method: string    # "grass", "water", "fishing_old", "fishing_good", "fishing_super", "headbutt", "rock_smash"
time_of_day:      string    # "morning", "day", "night", "any"
species_list:     string[]  # Pokemon available
level_range:      string    # "3-7"
```

#### `mechanic`
```yaml
mechanic_scope:   string    # "damage", "accuracy", "critical", "weather", "status", "stat_stage", "switching", "priority", "held_item"
has_formula:      bool      # Contains a mathematical formula
formula_text:     string    # The formula itself if applicable
```

#### `battle_rule`
```yaml
rule_type:        string    # "move_effect", "interaction", "edge_case", "ai_behavior"
moves_involved:   string[]  # Which moves this rule applies to
conditions:       string[]  # When this rule triggers
```

#### `type_interaction`
```yaml
attacking_type:   string
defending_type:   string
effectiveness:    string    # "super_effective", "not_very_effective", "no_effect", "neutral"
multiplier:       float     # 2.0, 0.5, 0.0, 1.0
```

#### `story_event`
```yaml
progression_order: int      # Sequential order in game progression
prerequisites:    string[]  # What must be done first
flags_set:        string[]  # Game flags this event sets
location:         string    # Where this event occurs
```

#### `strategy`
```yaml
strategy_type:    string    # "boss_fight", "team_building", "route_planning", "competitive"
difficulty:       string    # "beginner", "intermediate", "advanced"
target_opponent:  string    # If boss strategy, which boss
```

---

## 4. Chunk ID Convention

Format: `{doc_type}_{entity_slug}_{seq:03d}`

Examples:
- `species_bulbasaur_001` — Bulbasaur base stats
- `learnset_bulbasaur_level_001` — Bulbasaur level-up moves
- `learnset_bulbasaur_tmhm_001` — Bulbasaur TM/HM compatibility
- `move_thunderbolt_001` — Thunderbolt move data
- `trainer_falkner_001` — Gym Leader Falkner battle
- `map_violet_city_001` — Violet City overview
- `wild_route29_grass_001` — Route 29 grass encounters
- `mechanic_damage_formula_001` — Core damage formula
- `mechanic_damage_formula_002` — Damage formula modifiers (overflow chunk)
- `battle_rule_counter_mirror_coat_001` — Counter/Mirror Coat interaction
- `type_fire_offensive_001` — Fire-type offensive matchups
- `story_elm_lab_start_001` — Getting starter from Elm

Entity slug rules:
- Lowercase, underscores for spaces
- Pokemon names: `bulbasaur`, `mr_mime`, `farfetchd`, `nidoran_f`, `nidoran_m`
- Moves: `thunder_wave`, `hi_jump_kick`, `double_edge`
- Locations: `route_29`, `violet_city`, `sprout_tower_2f`

---

## 5. Cross-Reference Linking Strategy

Every chunk includes `related_entities` — an array of chunk IDs that are semantically connected. This enables:

1. **Graph traversal** — follow links to answer multi-hop questions
2. **Context augmentation** — pull related chunks into LLM context alongside the retrieved chunk
3. **Cluster-based retrieval** — retrieve a chunk + its N closest linked chunks

### Linking Rules

| From doc_type | Links to | Relationship |
|---|---|---|
| `species` | `learnset`, `evolution`, `wild_encounter` | has_learnset, evolves_to, found_at |
| `learnset` | `species`, `move` | belongs_to, contains_move |
| `evolution` | `species`, `item` | evolves_from, requires_item |
| `move` | `species` (via learnset), `mechanic`, `battle_rule` | learned_by, governed_by |
| `item` | `species`, `map_location`, `evolution` | held_by, found_at, enables_evolution |
| `trainer` | `species`, `map_location`, `story_event` | uses_pokemon, located_at, part_of |
| `map_location` | `wild_encounter`, `trainer`, `item`, `story_event` | has_encounters, has_trainers, contains_item |
| `wild_encounter` | `species`, `map_location` | contains_species, located_at |
| `mechanic` | `move`, `battle_rule` | applies_to, has_subrule |
| `battle_rule` | `move`, `mechanic` | involves_move, part_of_mechanic |
| `type_interaction` | `species`, `move` | affects_species, affects_move |
| `story_event` | `map_location`, `trainer`, `item` | occurs_at, involves_battle, grants_item |
| `strategy` | `trainer`, `species`, `move`, `item` | targets_fight, recommends_pokemon |

### Link Density Target
- Each chunk should have **3-10 related entities**
- High-value hub chunks (damage formula, popular species) may have 20+
- Orphan chunks (0 links) indicate incomplete ingestion — flag for review

---

## 6. Namespace / Collection Strategy

Use **one collection** with metadata filtering rather than multiple collections. This allows cross-type queries (e.g., "What Water-type moves can Feraligatr learn that are boosted by Rain?").

### Recommended Namespaces (if using Pinecone)
- `crystal_v1` — production namespace
- `crystal_draft` — staging for new ingestion runs

### Index Configuration
```yaml
index:
  name: pokemon-crystal
  dimension: 1536   # or 3072 for full-resolution
  metric: cosine
  pod_type: p2.x1   # or serverless
  metadata_config:
    indexed:         # Fields to enable filtering on
      - doc_type
      - category
      - subcategory
      - name
      - tags
      - region
      - move_type
      - type1
      - type2
      - trainer_class
      - map_type
      - mechanic_scope
```

---

## 7. Estimated Collection Size

| doc_type | Estimated chunks | Notes |
|---|---|---|
| `species` | 251 | One per Pokemon |
| `learnset` | 750 | ~3 per species (level-up, TM/HM, egg) |
| `evolution` | 120 | One per evolution family |
| `move` | 251 | One per move |
| `item` | 180 | Key items + held items + consumables + TMs/HMs |
| `trainer` | 400 | Gym leaders + E4 + rival + route trainers |
| `map_location` | 200 | Cities + routes + dungeons + buildings |
| `wild_encounter` | 300 | Multiple per route (grass/water/fish/headbutt x time) |
| `mechanic` | 80 | Core rules, formulas, systems |
| `battle_rule` | 150 | Move effects, edge cases, interactions |
| `type_interaction` | 36 | One per type (offensive + defensive summary) |
| `story_event` | 100 | Major progression beats |
| `strategy` | 60 | Boss guides, team building |
| `game_corner` | 40 | Side features, mini-games |
| `music_art` | 30 | Tracks, visual info |
| **TOTAL** | **~2,950** | Comfortable for any vector DB |
