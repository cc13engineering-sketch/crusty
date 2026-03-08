# Pokemon Crystal Vector DB — Master Index

> Registry of all data files produced by each agent for ingestion into the vector DB.
> Agents register their output files here as they complete work.
> The ingestion pipeline reads this index to know what to process.

---

## Status Key

| Status | Meaning |
|---|---|
| `pending` | File planned but not yet created |
| `draft` | File created, not yet reviewed/validated |
| `validated` | Cross-checked against pokecrystal source, ready for ingestion |
| `ingested` | Loaded into vector DB |
| `stale` | Needs update (source data changed or errors found) |

---

## Schema Files (data-scientist)

| File | Status | Description |
|---|---|---|
| `data/schema/schema.md` | validated | Master schema: doc types, metadata fields, index config |
| `data/schema/taxonomy.md` | validated | Classification hierarchy, tags, entity relationships |
| `data/schema/chunking_guide.md` | validated | Chunking strategy, size limits, context headers |
| `data/schema/index.md` | validated | This file — master registry |
| `data/schema/retrieval_config.md` | validated | Retrieval strategy, hybrid search, re-ranking |

---

## Pokemon & Moves Data (pokemon-geek — Task #1)

| File | Status | Description | Est. Chunks |
|---|---|---|---|
| `data/pokemon/species_chunks.jsonl` | pending | All 251 species base data | 251 |
| `data/pokemon/learnsets_chunks.jsonl` | pending | Level-up, TM/HM, egg move learnsets | ~750 |
| `data/pokemon/evolutions_chunks.jsonl` | pending | All evolution chains and methods | ~120 |
| `data/moves/move_chunks.jsonl` | pending | All 251 moves with stats and effects | 251 |
| `data/items/item_chunks.jsonl` | pending | All items with effects and locations | ~180 |
| `data/types/type_chunks.jsonl` | pending | Type chart, matchups per type | ~36 |

---

## Battle Mechanics & Game Design (game-designer — Task #3)

| File | Status | Description | Est. Chunks |
|---|---|---|---|
| `data/mechanics/damage_chunks.jsonl` | pending | Damage formula, modifiers, edge cases | ~15 |
| `data/mechanics/accuracy_chunks.jsonl` | pending | Accuracy/evasion system, OHKO formula | ~8 |
| `data/mechanics/status_chunks.jsonl` | pending | All status conditions and their effects | ~12 |
| `data/mechanics/weather_chunks.jsonl` | pending | Weather system (rain, sun, sandstorm) | ~6 |
| `data/mechanics/stat_stages_chunks.jsonl` | pending | Stat stage system and modifiers | ~5 |
| `data/mechanics/priority_chunks.jsonl` | pending | Priority brackets and speed resolution | ~5 |
| `data/mechanics/battle_rules_chunks.jsonl` | pending | Move effects, interactions, edge cases | ~100 |
| `data/mechanics/ai_chunks.jsonl` | pending | Trainer AI behavior and scoring | ~10 |
| `data/mechanics/held_items_chunks.jsonl` | pending | In-battle held item effects | ~20 |
| `data/mechanics/capture_chunks.jsonl` | pending | Catch rate formula and ball modifiers | ~5 |

---

## World Data, Trainers & Strategy (formatter — Task #4)

| File | Status | Description | Est. Chunks |
|---|---|---|---|
| `data/world/locations_chunks.jsonl` | pending | All map locations with connections | ~200 |
| `data/world/encounters_chunks.jsonl` | pending | Wild encounter tables per area | ~300 |
| `data/trainers/trainer_chunks.jsonl` | pending | All trainer battles with teams | ~400 |
| `data/story/events_chunks.jsonl` | pending | Story progression events | ~100 |
| `data/strategy/boss_guides_chunks.jsonl` | pending | Gym leader / E4 / Red strategies | ~30 |
| `data/strategy/team_building_chunks.jsonl` | pending | Team composition guides | ~15 |
| `data/strategy/routing_chunks.jsonl` | pending | Route planning / grinding guides | ~15 |
| `data/side/features_chunks.jsonl` | pending | Bug Contest, Game Corner, Pokegear, etc. | ~40 |
| `data/audio/music_chunks.jsonl` | pending | Music and visual info | ~30 |

---

## JSONL Format Specification

Each line in a `.jsonl` file is one chunk, formatted as a JSON object:

```json
{
  "id": "species_bulbasaur_001",
  "doc_type": "species",
  "name": "Bulbasaur",
  "category": "pokemon_data",
  "subcategory": "base_stats",
  "tags": ["grass", "poison", "johto_native", "starter"],
  "related_entities": ["learnset_bulbasaur_level_001", "evolution_bulbasaur_001", "move_vine_whip_001"],
  "source": "pokecrystal",
  "source_file": "data/pokemon/base_stats/bulbasaur.asm",
  "generation": 2,
  "game": "crystal",
  "dex_number": 1,
  "type1": "grass",
  "type2": "poison",
  "text": "[Pokemon Crystal] Bulbasaur (#001) — Grass/Poison Pokemon\nBase Stats: HP 45 / Atk 49 / Def 49 / SpA 65 / SpD 65 / Spe 45\n\nBulbasaur is a Grass/Poison-type Pokemon with National Dex number 001. It has a catch rate of 45 (very low — hard to catch), a medium-slow growth rate, and belongs to the Monster and Plant egg groups. It is 87.5% male. It does not hold any items in the wild.\n\nBulbasaur evolves into Ivysaur at level 16. Its base stat total is 318, with balanced defenses and slightly higher special stats than physical. As a Grass/Poison type, it is weak to Fire, Ice, Flying, and Psychic, and resists Water, Electric, Grass, and Fighting."
}
```

### Field Requirements for JSONL

**Required in every chunk**: `id`, `doc_type`, `name`, `category`, `subcategory`, `tags`, `related_entities`, `source`, `generation`, `game`, `text`

**Optional (type-specific)**: All other metadata fields from schema.md

### Naming Convention for Files

```
data/{domain}/{content_type}_chunks.jsonl
```

Examples:
- `data/pokemon/species_chunks.jsonl`
- `data/mechanics/damage_chunks.jsonl`
- `data/world/locations_chunks.jsonl`

---

## Ingestion Pipeline (future)

The ingestion pipeline will:

1. Read this `index.md` to discover all registered JSONL files
2. For each file with status `validated`:
   a. Parse each JSON line
   b. Validate against schema.md (required fields, tag vocabulary, ID format)
   c. Generate embedding for the `text` field
   d. Upsert to vector DB with all metadata fields
   e. Update status to `ingested`
3. Build cross-reference index from `related_entities` fields
4. Run validation queries to spot-check retrieval quality

---

## Chunk Count Summary

| Domain | Est. Chunks |
|---|---|
| Pokemon Data (species, learnsets, evolutions) | ~1,121 |
| Moves Data | ~251 |
| Items Data | ~180 |
| Type System | ~36 |
| Battle Mechanics | ~186 |
| World / Locations | ~500 |
| Trainers | ~400 |
| Story Events | ~100 |
| Strategy | ~60 |
| Side Features | ~40 |
| Audio / Visual | ~30 |
| **TOTAL** | **~2,904** |
