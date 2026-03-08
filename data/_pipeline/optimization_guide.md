# Pokemon Crystal Vector Database — LLM Optimization Guide

> Purpose: Technical guide for embedding, chunking, retrieving, and serving Pokemon Crystal data to LLMs
> Audience: Engineers building the RAG pipeline

---

## 1. Embedding Model Recommendations

### Primary Recommendation
**text-embedding-3-large** (OpenAI)
- 3072 dimensions (can truncate to 1536 or 768 for cost savings)
- Best balance of quality, speed, and cost
- Excellent at semantic similarity for game terminology
- ~$0.13 per 1M tokens

### Alternatives
| Model | Dimensions | Strengths | Weaknesses |
|-------|-----------|-----------|------------|
| voyage-3-large (Voyage AI) | 1024 | Excellent retrieval quality; code-aware | Higher latency |
| text-embedding-3-small (OpenAI) | 1536 | Cheapest; fast | Lower quality for nuanced queries |
| nomic-embed-text-v1.5 (open source) | 768 | Free; self-hostable | Requires GPU; lower quality |
| bge-large-en-v1.5 (BAAI, open source) | 1024 | Good quality; free | Requires GPU |
| mxbai-embed-large-v1 (Mixedbread) | 1024 | Strong retrieval; open source | Less tested on gaming domains |

### Recommendation: Start with text-embedding-3-large. If cost is a concern, text-embedding-3-small with matryoshka dimensionality reduction to 512 dimensions provides 90% of the quality at 1/3 the cost.

---

## 2. Chunk Sizing Strategy

Different content types require different chunking approaches for optimal retrieval.

### Factual Lookups (Species stats, move data, item data)
```
Chunk size: 200-400 tokens
Overlap: 0 (each chunk is self-contained)
Rationale: Factual data is atomic — a species' stats don't need context from other species.
```

**Example chunk:**
```
Typhlosion (#157) — Fire type
Base Stats: HP 78 / Atk 84 / Def 78 / SpA 109 / SpD 85 / Spe 100
Total: 534
Abilities: N/A (Gen 2)
Evolution: Cyndaquil → Quilava (Lv14) → Typhlosion (Lv36)
Key moves: Flamethrower, ThunderPunch, Earthquake, Fire Blast, Swift
Growth Rate: Medium Slow
Catch Rate: 45
Gender Ratio: 87.5% Male / 12.5% Female
```

### Mechanical Explanations (Damage formula, status effects, type interactions)
```
Chunk size: 500-1000 tokens
Overlap: 100 tokens
Rationale: Mechanics need surrounding context (e.g., the damage formula references STAB, type effectiveness, crits — these should be in the same chunk or overlap into adjacent chunks).
```

### Strategy Content (Team building, gym strategies)
```
Chunk size: 300-700 tokens
Overlap: 50 tokens
Rationale: Strategy advice is semi-modular. A gym strategy is self-contained but references Pokemon available at that point.
```

### Q&A Pairs (Evaluation and training data)
```
Chunk size: As-is (question + answer together)
Overlap: 0
Rationale: Q&A pairs are pre-formed retrieval units. Never split a question from its answer.
```

### Location Data (Maps, routes, connections)
```
Chunk size: 300-600 tokens
Overlap: 50 tokens
Rationale: Each location is somewhat self-contained but references connections to other locations.
```

---

## 3. Metadata Schema

Every chunk should carry structured metadata for filtering and re-ranking.

```json
{
  "chunk_id": "species_typhlosion_001",
  "content_type": "factual",
  "category": "species",
  "subcategory": "fire_type",
  "entities": ["Typhlosion", "Cyndaquil", "Quilava"],
  "difficulty": "beginner",
  "source_file": "data/species/all_species.md",
  "source_section": "Typhlosion",
  "game_phase": "all",
  "region": "johto",
  "tags": ["starter", "fire", "evolution", "base_stats"],
  "related_chunks": ["species_cyndaquil_001", "species_quilava_001", "move_flamethrower_001"],
  "token_count": 287,
  "last_updated": "2024-03-07"
}
```

### Metadata Field Definitions

| Field | Type | Values | Purpose |
|-------|------|--------|---------|
| `chunk_id` | string | `{category}_{name}_{seq}` | Unique identifier |
| `content_type` | enum | `factual`, `mechanical`, `strategic`, `location`, `qa`, `lore` | Chunk classification |
| `category` | enum | `species`, `moves`, `items`, `types`, `trainers`, `locations`, `mechanics`, `strategy`, `competitive`, `meta` | Domain category |
| `subcategory` | string | Free-form refinement | Narrower topic |
| `entities` | string[] | Named entities mentioned | Entity-based filtering |
| `difficulty` | enum | `beginner`, `intermediate`, `advanced`, `expert` | User expertise filtering |
| `source_file` | string | File path | Provenance tracking |
| `game_phase` | enum | `early`, `mid`, `late`, `post_game`, `all` | When in game this is relevant |
| `region` | enum | `johto`, `kanto`, `both`, `meta` | Geographic relevance |
| `tags` | string[] | Free-form keywords | Broad search support |
| `related_chunks` | string[] | chunk_id references | Cross-reference graph |
| `token_count` | int | Actual token count | Cost estimation |

---

## 4. Query Augmentation

User queries about Pokemon games often use informal language. The retrieval pipeline should expand queries with game-specific terminology.

### Synonym Expansion Table
| User Term | Expand To |
|-----------|-----------|
| "stats" | "base stats", "stat stages", "DVs", "stat experience" |
| "moves" | "attacks", "TM", "HM", "level-up moves", "egg moves" |
| "type chart" | "type effectiveness", "super effective", "not very effective", "immune" |
| "shiny" | "shiny Pokemon", "alternate color", "DVs for shiny" |
| "EV" / "effort" | "stat experience" (Gen 2 uses stat exp, not EVs) |
| "ability" | "abilities do not exist in Gen 2" (important disambiguation) |
| "nature" | "natures do not exist in Gen 2" |
| "held item" | "held items", "Leftovers", "berries" |
| "gym" | "gym leader", "gym badge", "gym puzzle" |
| "E4" / "elite four" | "Elite Four", "Pokemon League", "Will", "Koga", "Bruno", "Karen", "Lance" |
| "dragon" | could be: Dragon type, Dragon's Den, Dragonite, DragonBreath, Dragon Rage |
| "thunder" | could be: Thunder (move), Thunderbolt, ThunderPunch, Thunder Badge, Thunder Stone |
| "fire blast" | Fire Blast (move), but also check Fire type, Blaine, Typhlosion |

### Ambiguity Resolution
Some terms map to multiple concepts. The system should:
1. Embed the query as-is
2. Retrieve top-K results across all possible meanings
3. Use the metadata `category` field to diversify results
4. Let the LLM disambiguate based on conversation context

**Example**: "Tell me about Thunder"
- Retrieve: Thunder (move, 120 power Electric), Thunder Badge (Lt. Surge), Thunder Stone (evolution item), ThunderPunch (move, 75 power), Thunderbolt (move, 95 power)
- The LLM picks the relevant one based on context

---

## 5. Retrieval Pipeline

### Architecture
```
User Query
    ↓
[Query Preprocessing]
  - Lowercase, normalize
  - Synonym expansion
  - Extract entities (Pokemon names, move names)
    ↓
[Dense Retrieval]
  - Embed query with same model as corpus
  - ANN search (cosine similarity)
  - Retrieve top 50 candidates
    ↓
[Metadata Filter]
  - Filter by content_type if query specifies (e.g., "how does X work" → mechanical)
  - Filter by game_phase if determinable
  - Filter by region if specified
    ↓
[Re-Ranking]
  - Cross-encoder re-rank (e.g., bge-reranker-v2-m3)
  - Or: Cohere rerank API
  - Score based on relevance + diversity
    ↓
[Context Assembly]
  - Select top 5-10 chunks (stay under LLM context budget)
  - Order by: exact match first, then category diversity
  - Include related_chunks if space allows
  - Total target: 2000-4000 tokens of context
    ↓
[LLM Generation]
  - System prompt with domain context
  - Retrieved chunks as context
  - User query
  - Generate answer with citations
```

### Retrieval Parameters
```python
RETRIEVAL_CONFIG = {
    "top_k_initial": 50,        # Dense retrieval candidates
    "top_k_reranked": 10,       # After re-ranking
    "top_k_final": 5,           # Included in LLM context
    "max_context_tokens": 4000, # Total context budget
    "similarity_threshold": 0.65,  # Minimum cosine similarity
    "diversity_weight": 0.2,    # Encourage category diversity
}
```

### Handling Edge Cases

**Multi-hop queries**: "What level should my Typhlosion be to beat Clair?"
- Requires: Typhlosion stats/moves + Clair's team + level scaling mechanics
- Strategy: Retrieve from multiple categories (species, trainers, mechanics)

**Comparison queries**: "Is Feraligatr or Typhlosion better for the Elite Four?"
- Requires: Both species data + E4 team data + type matchup analysis
- Strategy: Use entity extraction to find both Pokemon, then retrieve E4 data

**Negation queries**: "What Pokemon can't learn Surf?"
- Dense retrieval struggles with negation
- Strategy: Retrieve all Surf-related data, let LLM filter

---

## 6. Evaluation Strategy

### Metrics
- **Hit Rate@5**: Does the correct answer appear in top 5 retrieved chunks?
- **MRR (Mean Reciprocal Rank)**: How high is the correct chunk ranked?
- **Answer Accuracy**: Does the LLM produce the correct answer given retrieved context?
- **Hallucination Rate**: Does the LLM add information not in the context?

### Evaluation Process
1. Use the 200+ Q&A pairs from `qa_pairs.md`
2. For each question:
   a. Run through retrieval pipeline
   b. Check if answer-containing chunk is retrieved (Hit Rate)
   c. Check ranking position (MRR)
   d. Generate answer with LLM
   e. Compare to gold answer (exact match or semantic similarity)
3. Calculate aggregate metrics
4. Identify failure categories:
   - Wrong entity retrieved (e.g., wrong Pokemon)
   - Missing context (relevant chunk not retrieved)
   - Hallucination (LLM invents information)

### Target Metrics
| Metric | Target | Acceptable |
|--------|--------|------------|
| Hit Rate@5 | >90% | >80% |
| MRR | >0.75 | >0.60 |
| Answer Accuracy | >85% | >75% |
| Hallucination Rate | <5% | <10% |

---

## 7. System Prompt Template

```
You are a Pokemon Crystal expert assistant. You answer questions about Pokemon Gold, Silver, and Crystal (Generation 2) using the provided context.

Rules:
1. Only use information from the provided context. If the context doesn't contain enough information, say so.
2. Gen 2 does NOT have abilities, natures, EVs (it has stat experience), physical/special split per move (split is by TYPE), or Fairy type.
3. When discussing competitive play, specify if you mean in-game or Smogon competitive.
4. Cite specific data points (levels, damage values, percentages) when available.
5. If a question is ambiguous (e.g., "Thunder" could be a move or badge), clarify which meaning is relevant.

Context:
{retrieved_chunks}

User question: {query}
```

---

## 8. Data Quality Checklist

Before embedding, verify each data file:
- [ ] No contradictions between files (e.g., Typhlosion stats match across species.md and team_building.md)
- [ ] All numeric values verified against pokecrystal source
- [ ] No Gen 3+ mechanics accidentally included (abilities, natures, physical/special per-move split)
- [ ] Consistent naming (e.g., "Special Attack" not "Sp. Atk" in some places and "SpA" in others — pick one and be consistent)
- [ ] Cross-references are valid (related_chunks point to existing chunks)
- [ ] No duplicate chunks with conflicting information

### Common Gen 2 Mistakes to Avoid
1. Listing abilities (Gen 2 has no abilities)
2. Using per-move physical/special split (it's by TYPE in Gen 2)
3. Listing Fairy type moves/matchups (Fairy doesn't exist)
4. Using modern stat names (Gen 2: "Special Attack" and "Special Defense" exist but were recently split from "Special")
5. Confusing stat experience (Gen 2) with EVs (Gen 3+)
6. Listing Pokemon that don't exist yet (no Pokemon past #251)
