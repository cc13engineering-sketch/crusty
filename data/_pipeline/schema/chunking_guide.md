# Pokemon Crystal Vector DB — Chunking Strategy Guide

> How to split Pokemon Crystal knowledge into vector-DB-ready chunks.
> Goal: each chunk is self-contained, retrievable, and contextually complete.

---

## 1. Core Principles

1. **One entity, one chunk** — A chunk should answer questions about ONE thing (one Pokemon, one move, one mechanic). Never mix unrelated entities.
2. **Self-contained comprehension** — A reader seeing ONLY this chunk should understand it without needing another chunk. Prepend context headers.
3. **Consistent granularity** — Similar entities should produce similarly-sized chunks. Don't have a 50-token Rattata chunk and a 2000-token Pikachu chunk.
4. **Overlap for mechanics** — Mechanical/procedural content benefits from overlap between chunks. Factual/tabular content does not.
5. **Split on semantic boundaries** — Never split mid-sentence, mid-formula, or mid-table-row.

---

## 2. Chunk Size Guidelines by doc_type

| doc_type | Target tokens | Min | Max | Overlap |
|---|---|---|---|---|
| `species` | 300 | 150 | 500 | 0 |
| `learnset` | 400 | 200 | 700 | 0 |
| `evolution` | 200 | 100 | 400 | 0 |
| `move` | 250 | 100 | 400 | 0 |
| `item` | 200 | 100 | 350 | 0 |
| `trainer` | 400 | 200 | 700 | 0 |
| `map_location` | 500 | 300 | 900 | 50 tokens |
| `wild_encounter` | 300 | 150 | 600 | 0 |
| `mechanic` | 700 | 400 | 1200 | 100 tokens |
| `battle_rule` | 500 | 250 | 800 | 75 tokens |
| `type_interaction` | 300 | 150 | 500 | 0 |
| `story_event` | 350 | 200 | 600 | 50 tokens |
| `strategy` | 500 | 300 | 800 | 75 tokens |
| `game_corner` | 400 | 200 | 700 | 50 tokens |
| `music_art` | 200 | 100 | 350 | 0 |

### When to Split a Chunk

Split into multiple chunks (with sequential `_001`, `_002` suffixes) when:
- Content exceeds the max token limit for its doc_type
- The content covers distinct sub-topics (e.g., a mechanic chunk covering both the formula AND all edge cases)
- A large table would be more retrievable as separate rows/groups

### When NOT to Split

Keep as one chunk even if slightly over max when:
- A formula + its variables + one worked example form a logical unit
- An evolution chain with 3 stages and all methods fits together
- A trainer's full team (up to 6 Pokemon with movesets) is one battle

---

## 3. Context Headers

Every chunk MUST start with a **context header** — a 1-3 line prefix that establishes what this chunk is about, even when read in isolation.

### Header Templates by doc_type

#### species
```
[Pokemon Crystal] {Name} (#{dex_number}) — {Type1}/{Type2} Pokemon
Base Stats: HP {hp} / Atk {atk} / Def {def} / SpA {spa} / SpD {spd} / Spe {spe}
```

#### learnset
```
[Pokemon Crystal] {Species Name} — {Learnset Type} Moves
```

#### evolution
```
[Pokemon Crystal] Evolution Chain: {Base} → {Stage2} → {Stage3}
```

#### move
```
[Pokemon Crystal] Move: {Name} — {Type} / {Category} / Power {power} / Acc {accuracy}% / PP {pp}
```

#### item
```
[Pokemon Crystal] Item: {Name} — {Pocket Category}
```

#### trainer
```
[Pokemon Crystal] Trainer Battle: {Class} {Name} at {Location}
Team: {Pokemon1} Lv{level}, {Pokemon2} Lv{level}, ...
```

#### map_location
```
[Pokemon Crystal] Location: {Name} — {Region} {Type}
Connections: {North}, {South}, {East}, {West}
```

#### wild_encounter
```
[Pokemon Crystal] Wild Encounters: {Location} — {Method} ({Time of Day})
```

#### mechanic
```
[Pokemon Crystal] Battle Mechanic: {Topic}
```

#### battle_rule
```
[Pokemon Crystal] Battle Rule: {Description}
Applies to: {Move(s) or condition(s)}
```

#### type_interaction
```
[Pokemon Crystal] Type Matchup: {Type} — {Offensive/Defensive} Summary
```

#### story_event
```
[Pokemon Crystal] Story: {Event Name}
Location: {Where} | Prerequisites: {What must happen first}
```

#### strategy
```
[Pokemon Crystal] Strategy Guide: {Topic}
```

---

## 4. Special Content Handling

### Tables

Tables are common in Pokemon data (stat tables, encounter tables, type charts).

**Small tables (< 10 rows)**: Keep inline in the chunk as markdown table.
```
| Level | Move |
|-------|------|
| 1     | Tackle |
| 4     | Growl |
| 7     | Leech Seed |
```

**Medium tables (10-30 rows)**: Keep as one chunk if under max tokens. Format as markdown table.

**Large tables (30+ rows)**: Split into logical groups.
- Type chart: split by attacking type (one chunk = "Fire-type offensive matchups")
- Full TM list: split into groups of 10-15 TMs
- Encounter tables: split by location or by method (grass/water/fish)

### Formulas

ALWAYS keep a formula with its variable definitions in the same chunk. Never split:
```
GOOD (one chunk):
  Damage = (((2 * Level / 5 + 2) * Power * A / D / 50) * Modifier + 2)
  Where:
    Level = attacker's level
    Power = move's base power
    A = relevant attack stat
    D = relevant defense stat
    Modifier = critical * STAB * type * random

BAD (split across chunks):
  Chunk 1: Damage = (((2 * Level / 5 + 2) * Power * A / D / 50) * Modifier + 2)
  Chunk 2: Where Level = attacker's level, Power = move's base power ...
```

If formula + all modifiers + examples exceed max tokens, split as:
- Chunk 1: Base formula + core variables + one example
- Chunk 2: Context header repeating the formula + all modifier details + edge cases

### Lists (Learnsets, Encounter Lists)

**Short lists (< 20 items)**: Keep as one chunk.

**Long lists (20+ items)**: Include ALL items but use compact format:
```
Level-up moves: Tackle(1), Growl(4), Leech Seed(7), Vine Whip(10), ...
```
rather than one-per-line.

If still too long, split with overlap of the last 3 items from the previous chunk.

### ASM Source Snippets

When including raw pokecrystal ASM as evidence:
- Always include the file path as source attribution
- Keep ASM snippets short (5-15 lines)
- Always pair with a plain-English explanation
- Use the ASM to support claims, not as the primary content

```
GOOD:
  Whitney's Miltank knows Rollout, Attract, Stomp, and Milk Drink (all at level 20).
  Source: data/trainers/parties.asm
  > db 20, MILTANK, ROLLOUT, ATTRACT, STOMP, MILK_DRINK

BAD:
  db 20, MILTANK, ROLLOUT, ATTRACT, STOMP, MILK_DRINK
  (no explanation, no context)
```

---

## 5. Parent-Child Chunk Relationships

Some entities naturally decompose into a parent overview + child detail chunks.

### Parent-Child Patterns

| Parent | Children |
|---|---|
| `species_bulbasaur_001` (overview) | `learnset_bulbasaur_level_001`, `learnset_bulbasaur_tmhm_001`, `learnset_bulbasaur_egg_001` |
| `map_violet_city_001` (overview) | `trainer_falkner_001`, `wild_sprout_tower_2f_001`, `story_elm_egg_delivery_001` |
| `mechanic_damage_formula_001` (core) | `mechanic_damage_modifiers_001`, `battle_rule_critical_hit_stages_001` |
| `evolution_eevee_001` (chain overview) | `item_water_stone_001`, `item_fire_stone_001`, ... |

### Metadata for Parent-Child

```yaml
# On child chunk:
parent_chunk_id: "species_bulbasaur_001"

# On parent chunk:
child_chunk_ids: ["learnset_bulbasaur_level_001", "learnset_bulbasaur_tmhm_001"]
```

This enables:
- **Drill-down retrieval**: Retrieve parent, then optionally fetch children for detail
- **Roll-up retrieval**: Retrieve a child, then fetch parent for broader context
- **Scoped retrieval**: Filter to only children of a specific parent

---

## 6. Deduplication Rules

Avoid storing the same fact in multiple chunks. When overlap is necessary:

1. **Canonical chunk owns the fact** — The chunk where the fact is most contextually relevant owns it in full detail.
2. **Other chunks reference, don't repeat** — Use a brief mention + cross-reference:
   ```
   Bulbasaur evolves into Ivysaur at level 16 (see: evolution_bulbasaur_001 for full chain).
   ```
3. **Type chart is the exception** — Type effectiveness info is duplicated across `type_interaction` chunks (per attacking type) AND mentioned in `species` chunks. This is intentional — type questions are high-frequency and benefit from multiple retrieval paths.

### Dedup Priority (who owns the canonical version)

| Fact | Canonical Owner |
|---|---|
| Base stats for a Pokemon | `species` chunk |
| Moves a Pokemon learns | `learnset` chunk |
| Move power/accuracy/effect | `move` chunk |
| Type effectiveness | `type_interaction` chunk |
| Where a Pokemon is found | `wild_encounter` chunk |
| Trainer's team | `trainer` chunk |
| How damage is calculated | `mechanic` chunk |
| Story event sequence | `story_event` chunk |
| Item location and effect | `item` chunk |

---

## 7. Quality Checklist

Before finalizing any chunk, verify:

- [ ] **Has context header** — First 1-3 lines establish topic without external context
- [ ] **Under max tokens** — Within the size limit for its doc_type
- [ ] **Factually correct** — Cross-checked against pokecrystal ASM source
- [ ] **Self-contained** — Makes sense if read in isolation
- [ ] **Properly tagged** — Uses only standardized tags from taxonomy.md
- [ ] **Has related_entities** — At least 3 cross-references to other chunks
- [ ] **No orphan splits** — If split into parts, all parts reference each other
- [ ] **No mid-break splits** — Not split in the middle of a sentence, formula, or table row
- [ ] **Source attributed** — `source` and `source_file` fields populated
- [ ] **ID follows convention** — Matches `{doc_type}_{entity_slug}_{seq:03d}` format
