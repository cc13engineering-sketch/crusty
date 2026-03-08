# Pokemon Crystal Vector DB — Retrieval Strategy Configuration

> How to query the vector DB for maximum accuracy on any Pokemon Crystal question.
> Covers hybrid search, re-ranking, query expansion, metadata filtering, and multi-hop retrieval.

---

## 1. Hybrid Search: Dense + Sparse

Use **hybrid search** combining dense vector similarity with sparse keyword matching. Pokemon content has many proper nouns (Bulbasaur, Thunderbolt, Falkner, Route 29) that benefit from exact keyword matching, while conceptual queries ("how does weather affect damage?") need semantic similarity.

### Configuration

```yaml
search:
  strategy: hybrid
  dense:
    weight: 0.7           # Semantic similarity
    model: text-embedding-3-large
    dimensions: 1536
    top_k: 20             # Retrieve 20 candidates from dense search
  sparse:
    weight: 0.3           # Keyword/BM25 matching
    method: bm25          # Or SPLADE if available
    top_k: 20             # Retrieve 20 candidates from sparse search
  fusion:
    method: reciprocal_rank_fusion  # RRF to merge dense + sparse results
    k: 60                 # RRF constant
    final_top_k: 10       # Return top 10 after fusion
```

### Why 70/30 Dense/Sparse

- Most questions are conceptual ("How does X work?") — dense excels
- But many questions name specific entities ("What moves does Typhlosion learn?") — sparse catches exact matches that dense might rank lower
- Pokemon names are unique tokens that BM25 handles well
- 70/30 tested as optimal for knowledge-base domains with proper nouns

### Sparse Search Notes

If using **Pinecone**: Enable hybrid with `sparse_values` on upsert (BM25 or learned sparse).
If using **Qdrant**: Use payload indexing + text match for keyword component.
If using **Weaviate**: Built-in BM25 + vector hybrid via `hybrid` query type.
If using **Chroma**: No native sparse — use pre-filtering on metadata instead.

---

## 2. Re-Ranking

After hybrid retrieval returns ~10 candidates, re-rank for precision.

### Re-Ranking Model

**Recommended**: `cohere-rerank-v3` or `bge-reranker-v2-m3`

```yaml
reranker:
  enabled: true
  model: cohere-rerank-v3    # Or bge-reranker-v2-m3 (local/free)
  top_n: 5                    # Keep top 5 after re-ranking
  input: query + candidate_text
```

### Re-Ranking Strategy

1. Hybrid search returns 10 candidates
2. For each candidate, construct input: `query: "{user_question}" | document: "{chunk_text}"`
3. Re-ranker scores each pair
4. Return top 5 by re-ranker score
5. If top score < 0.3, flag as "low confidence" — may need query reformulation

### When to Skip Re-Ranking

- Simple entity lookups ("What are Pikachu's base stats?") — first dense result is usually correct
- If latency budget is < 200ms, skip re-ranking to save ~100ms
- For batch ingestion validation queries, re-ranking adds unnecessary cost

---

## 3. Query Expansion Templates

Expand user queries to improve retrieval coverage. Different question types need different expansions.

### Expansion Strategy

```yaml
query_expansion:
  enabled: true
  method: llm_expansion   # Use a small LLM to generate expansions
  model: claude-haiku-4-5  # Fast, cheap
  max_expansions: 3        # Generate up to 3 query variants
  merge_results: true      # Combine results from all variants via RRF
```

### Template Expansions by Query Type

#### Entity Lookup
```
User: "Tell me about Scizor"
Expansions:
  1. "Scizor base stats type abilities"
  2. "Scizor evolution method moves learnset"
  3. "Scizor Pokemon Crystal species data"
```

#### Mechanic Question
```
User: "How do critical hits work?"
Expansions:
  1. "critical hit calculation formula rate stages"
  2. "critical hit damage modifier stat stages ignored"
  3. "Pokemon Crystal Gen 2 critical hit mechanics"
```

#### Comparison / Relational
```
User: "What's super effective against Steelix?"
Expansions:
  1. "Steelix type weaknesses Steel Ground"
  2. "Fire Fighting Ground Water super effective Steel Ground"
  3. "type matchup Steel Ground defensive"
```

#### Strategy Question
```
User: "How do I beat Whitney's Miltank?"
Expansions:
  1. "Whitney Miltank Rollout strategy counter"
  2. "Whitney gym leader Goldenrod team moves"
  3. "Rollout counter Fighting type Machop Geodude"
```

#### Location Question
```
User: "Where can I find Larvitar?"
Expansions:
  1. "Larvitar wild encounter location route"
  2. "Larvitar catch found Mt Silver"
  3. "wild encounter Larvitar level rate"
```

### Query Classification

Before expansion, classify the query intent:

```
entity_lookup    — asking about a specific Pokemon/move/item/trainer
mechanic_query   — asking how a game system works
comparison       — comparing entities or asking about matchups
strategy         — asking for advice or how to beat something
location         — asking where to find something or about a place
story            — asking about plot events or progression
meta             — asking about glitches, speedruns, competitive play
```

Use this classification to select the right expansion template and to apply metadata pre-filters.

---

## 4. Metadata Filtering Strategies

Pre-filter chunks before vector search to narrow the search space and improve precision.

### Filter Application by Query Type

#### Entity Lookup
```yaml
# "What moves does Feraligatr learn?"
filters:
  - doc_type: ["learnset"]
  - name: "Feraligatr"   # Or species_id contains "feraligatr"
```

#### Type Question
```yaml
# "What is super effective against Ghost?"
filters:
  - doc_type: ["type_interaction"]
  - tags: {contains: "ghost"}
```

#### Location Question
```yaml
# "What Pokemon are on Route 34?"
filters:
  - doc_type: ["wild_encounter", "map_location"]
  - name: {contains: "Route 34"}
```

#### Trainer Question
```yaml
# "What is Lance's team?"
filters:
  - doc_type: ["trainer"]
  - name: {contains: "Lance"}
```

#### Mechanic Question
```yaml
# "How does sandstorm work?"
filters:
  - doc_type: ["mechanic", "battle_rule"]
  - category: "battle_system"
  - tags: {contains: "weather"}
```

### Filter Decision Tree

```
Does the query mention a specific Pokemon name?
  → Filter: species name or species_id match
Does the query mention a specific move name?
  → Filter: move name match
Does the query mention a specific location?
  → Filter: location name match
Does the query mention a trainer/gym leader?
  → Filter: trainer name or class match
Does the query ask "how" or "why" about a mechanic?
  → Filter: doc_type in [mechanic, battle_rule]
Does the query ask "where" or "find"?
  → Filter: doc_type in [wild_encounter, map_location, item]
Does the query ask about story/plot?
  → Filter: doc_type in [story_event]
  → Filter: category = "story_progression"
No clear entity or type?
  → No pre-filter, rely on dense+sparse search
```

### Important: Never Over-Filter

- If a filter returns 0 results, **fall back to unfiltered search**
- For ambiguous queries, use broader filters (multiple doc_types)
- Entity name matching should be fuzzy (Miltank, miltank, MILTANK all match)

---

## 5. Multi-Hop Retrieval

Some questions require information from multiple chunks to answer completely.

### When Multi-Hop is Needed

| Question Pattern | Hops Required |
|---|---|
| "What are Pikachu's base stats?" | 1 (single entity lookup) |
| "Can Typhlosion learn Thunderpunch?" | 1-2 (learnset, maybe move details) |
| "What's the best team for Whitney?" | 2-3 (trainer data + type matchups + available Pokemon) |
| "How much damage does Steelix's Iron Tail do to Dragonite?" | 3-4 (move data + attacker stats + defender stats + damage formula) |
| "What's the optimal route from Goldenrod to Mahogany?" | 2-3 (map connections + required HMs + encounter data) |

### Multi-Hop Pipeline

```
Step 1: Initial Retrieval
  - Run hybrid search on user query → top 5 chunks

Step 2: Context Analysis
  - Does the retrieved context fully answer the question?
  - If YES → return answer
  - If NO → identify what's missing

Step 3: Follow-Up Retrieval
  - Generate follow-up queries for missing information
  - Use related_entities from Step 1 chunks as retrieval hints
  - Run targeted search with metadata filters

Step 4: Synthesis
  - Combine all retrieved chunks
  - Present to LLM with full context for answer generation
```

### Implementation

```yaml
multi_hop:
  enabled: true
  max_hops: 3              # Maximum retrieval rounds
  chunks_per_hop: 5        # Retrieve 5 chunks per round
  max_total_chunks: 12     # Cap total context at 12 chunks
  use_related_entities: true  # Follow cross-references from retrieved chunks

  # Decision model for whether to hop
  hop_decision:
    model: claude-haiku-4-5
    prompt: |
      Given this question and the retrieved context, is the context
      sufficient to fully answer the question?
      If not, what specific information is still needed?
      Respond with: SUFFICIENT or NEED: <what's missing>
```

### Related-Entity Chasing

When a retrieved chunk has `related_entities`, use them for targeted follow-up:

```
Retrieved: species_dragonite_001
  related_entities: [
    "learnset_dragonite_level_001",
    "evolution_dratini_001",
    "type_dragon_defensive_001"
  ]

If the question needs learnset info → fetch learnset_dragonite_level_001 directly by ID
If the question needs type info → fetch type_dragon_defensive_001 directly by ID
```

This is faster and more accurate than running another vector search.

---

## 6. Answer Generation Prompt Template

After retrieval, present context to the LLM for answer generation:

```
You are a Pokemon Crystal expert. Answer the user's question using ONLY the
provided context. If the context doesn't contain enough information, say so.

CONTEXT:
{chunk_1_text}

{chunk_2_text}

{chunk_3_text}
...

USER QUESTION: {query}

INSTRUCTIONS:
- Be precise and cite specific numbers (stats, damage, levels)
- If multiple sources give conflicting info, prefer the one marked source: "pokecrystal"
- For damage calculations, show your work step by step
- For team building advice, explain type matchup reasoning
- If the question cannot be fully answered from context, state what information is missing
```

---

## 7. Performance Benchmarks

Target latency for each retrieval path:

| Path | Target Latency | Notes |
|---|---|---|
| Single-hop (entity lookup) | < 300ms | Dense search + metadata filter |
| Single-hop with re-rank | < 500ms | Add re-ranker pass |
| Multi-hop (2 rounds) | < 1000ms | Two retrieval rounds |
| Multi-hop (3 rounds) | < 1500ms | Three retrieval rounds (rare) |
| Full pipeline (retrieval + LLM) | < 3000ms | Including answer generation |

### Cache Strategy

```yaml
cache:
  enabled: true
  ttl: 3600                # 1 hour (data doesn't change)
  strategy: query_hash     # Cache by normalized query hash
  store: redis             # Or in-memory for small deployments

  # Pre-warm cache with common queries
  prewarm_queries:
    - "What are the starter Pokemon?"
    - "How does the damage formula work?"
    - "What is the type chart?"
    - "How to beat Whitney?"
    - "Where to find Eevee?"
    - "What are Tyranitar's base stats?"
```

---

## 8. Evaluation Queries

Use these queries to validate retrieval quality after ingestion:

### Factual (should return exact answer in top-1)
1. "What are Meganium's base stats?" → species_meganium_001
2. "What type is Thunderbolt?" → move_thunderbolt_001
3. "Who is the 4th gym leader?" → trainer_morty_001
4. "What Pokemon can you find on Route 29?" → wild_route_29_grass_001

### Mechanical (should return relevant mechanic in top-3)
5. "How is critical hit rate calculated?" → mechanic_critical_hit_001
6. "Does STAB apply to status moves?" → mechanic_stab_001 (answer: no)
7. "What happens when Rollout hits 5 times in a row?" → battle_rule_rollout_001

### Multi-Hop (needs 2+ chunks)
8. "Can Alakazam learn Shadow Ball?" → learnset_alakazam_tmhm_001 + move_shadow_ball_001
9. "What's super effective against Lance's Dragonite?" → trainer_lance_001 + type_dragon_defensive_001
10. "Where is the best place to grind before Whitney?" → strategy_whitney_001 + map_route_34_001

### Edge Case (tests specificity)
11. "Does Earthquake hit Pokemon using Fly?" → battle_rule_fly_001 (answer: yes in Gen 2)
12. "What happens if both Pokemon faint from Destiny Bond?" → battle_rule_destiny_bond_001
13. "Can you catch Celebi in Crystal?" → story_event_celebi_001 (Japan-only GS Ball event)
