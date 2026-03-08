# Pokemon Crystal Vector Database — Embedding Preparation Guide

> Purpose: Step-by-step instructions for processing raw data files into embedding-ready chunks
> Audience: Engineers building the chunking and embedding pipeline

---

## 1. File-by-File Processing Instructions

### Source Files and Their Chunk Strategy

| File | Category | Chunk Strategy | Expected Chunks |
|------|----------|----------------|-----------------|
| `data/species/all_species.md` | species | 1 chunk per Pokemon (200-400 tokens) | ~251 |
| `data/moves/all_moves.md` | moves | 1 chunk per move (150-300 tokens) | ~251 |
| `data/types/type_chart.md` | types | 1 chunk per type + 1 overview chunk | ~19 |
| `data/types/type_interactions.md` | types | 500-token chunks with 100-token overlap | ~10-15 |
| `data/trainers/gym_leaders.md` | trainers | 1 chunk per gym leader (300-500 tokens) | ~16 |
| `data/trainers/elite_four.md` | trainers | 1 chunk per E4 member + 1 Champion | ~6 |
| `data/trainers/rival_battles.md` | trainers | 1 chunk per rival encounter | ~7 |
| `data/encounters/johto_wild.md` | encounters | 1 chunk per route/area | ~30 |
| `data/encounters/kanto_wild.md` | encounters | 1 chunk per route/area | ~25 |
| `data/items/all_items.md` | items | Group by category (balls, potions, etc.) 300 tokens each | ~20 |
| `data/mechanics/damage_formula.md` | mechanics | 500-1000 token chunks, 100 overlap | ~10 |
| `data/mechanics/status_effects.md` | mechanics | 1 chunk per status condition | ~8 |
| `data/mechanics/battle_flow.md` | mechanics | 500-token chunks | ~8 |
| `data/maps/johto_locations.md` | locations | 1 chunk per city + 1 per route + 1 per dungeon | ~50 |
| `data/maps/kanto_locations.md` | locations | 1 chunk per city + 1 per route | ~40 |
| `data/maps/game_progression.md` | locations | 1 chunk per game section/part | ~15 |
| `data/strategy/team_building.md` | strategy | 300-700 token chunks | ~15 |
| `data/strategy/gym_strategies.md` | strategy | 1 chunk per gym strategy | ~16 |
| `data/strategy/elite_four_strategies.md` | strategy | 1 chunk per E4 member + prep | ~8 |
| `data/strategy/speedrun_notes.md` | strategy | 500-token chunks | ~12 |
| `data/strategy/competitive_gen2.md` | competitive | 1 chunk per key Pokemon + team archetypes | ~30 |
| `data/meta/qa_pairs.md` | qa | 1 chunk per Q&A pair | ~205 |

**Total estimated chunks: ~800-1000**

---

## 2. Chunk ID Generation

### Format
```
{category}_{subcategory}_{identifier}_{sequence_number}
```

### Examples
```
species_fire_typhlosion_001
moves_ice_ice_beam_001
trainers_gym_falkner_001
trainers_gym_falkner_002          # if gym strategy needs 2 chunks
locations_johto_violet_city_001
mechanics_damage_formula_001
mechanics_damage_formula_002      # continued chunk
strategy_team_building_early_001
competitive_ou_snorlax_001
qa_factual_q001
```

### Rules
1. All lowercase, underscores for spaces
2. Category matches the metadata `category` field
3. Subcategory provides semantic grouping
4. Identifier is the primary entity name
5. Sequence number (3 digits, zero-padded) handles multi-chunk entities

---

## 3. Metadata Extraction

### Automatic Metadata from File Structure

```python
def extract_metadata(file_path: str, chunk_text: str, chunk_index: int) -> dict:
    """Extract metadata from file path and chunk content."""

    # Category from directory
    parts = file_path.split('/')
    category = parts[-2]  # e.g., 'species', 'moves', 'maps'

    # Source file
    source_file = file_path

    # Entities: extract capitalized proper nouns (Pokemon names, move names)
    entities = extract_game_entities(chunk_text)

    # Difficulty: based on content complexity
    difficulty = classify_difficulty(chunk_text)

    # Game phase: infer from level ranges or location
    game_phase = infer_game_phase(chunk_text)

    # Region
    region = infer_region(file_path, chunk_text)

    # Content type
    content_type = classify_content_type(chunk_text, category)

    return {
        "chunk_id": generate_chunk_id(category, chunk_text, chunk_index),
        "content_type": content_type,
        "category": category,
        "subcategory": extract_subcategory(chunk_text, category),
        "entities": entities,
        "difficulty": difficulty,
        "source_file": source_file,
        "source_section": extract_section_header(chunk_text),
        "game_phase": game_phase,
        "region": region,
        "tags": extract_tags(chunk_text),
        "related_chunks": [],  # Populated in post-processing
        "token_count": count_tokens(chunk_text),
    }
```

### Entity Extraction

```python
# Known entity lists for matching
POKEMON_NAMES = [
    "Bulbasaur", "Ivysaur", "Venusaur", ..., "Celebi"  # All 251
]

MOVE_NAMES = [
    "Pound", "Karate Chop", ..., "Beat Up"  # All 251 moves
]

ITEM_NAMES = [
    "Poke Ball", "Great Ball", ..., "GS Ball"  # All items
]

TRAINER_NAMES = [
    "Falkner", "Bugsy", "Whitney", "Morty", "Chuck",
    "Jasmine", "Pryce", "Clair", "Will", "Koga",
    "Bruno", "Karen", "Lance", "Red", "Blue", "Silver",
    "Lt. Surge", "Sabrina", "Erika", "Janine", "Brock",
    "Misty", "Blaine"
]

LOCATION_NAMES = [
    "New Bark Town", "Cherrygrove City", "Violet City",
    "Azalea Town", "Goldenrod City", "Ecruteak City",
    "Olivine City", "Cianwood City", "Mahogany Town",
    "Blackthorn City", "Route 29", "Route 30", ...
]

def extract_game_entities(text: str) -> list[str]:
    """Find all game entities mentioned in text."""
    found = []
    for entity_list in [POKEMON_NAMES, MOVE_NAMES, ITEM_NAMES,
                         TRAINER_NAMES, LOCATION_NAMES]:
        for entity in entity_list:
            if entity.lower() in text.lower():
                found.append(entity)
    return list(set(found))
```

### Difficulty Classification

```python
def classify_difficulty(text: str) -> str:
    """Classify chunk difficulty based on content."""
    advanced_signals = [
        "formula", "DV", "competitive", "Smogon", "OU", "Ubers",
        "metagame", "Spikes", "phazing", "RestTalk", "EV",
        "stat experience", "Baton Pass chain", "damage calc"
    ]
    intermediate_signals = [
        "evolution", "type effectiveness", "STAB", "critical hit",
        "status condition", "held item", "breeding", "friendship"
    ]

    text_lower = text.lower()
    advanced_count = sum(1 for s in advanced_signals if s.lower() in text_lower)
    intermediate_count = sum(1 for s in intermediate_signals if s.lower() in text_lower)

    if advanced_count >= 2:
        return "advanced"
    elif intermediate_count >= 2 or advanced_count >= 1:
        return "intermediate"
    else:
        return "beginner"
```

### Game Phase Inference

```python
def infer_game_phase(text: str) -> str:
    """Infer which game phase this content is relevant to."""
    # Check for level ranges
    levels = re.findall(r'Lv\.?\s*(\d+)', text)
    if levels:
        max_level = max(int(l) for l in levels)
        if max_level <= 20:
            return "early"
        elif max_level <= 35:
            return "mid"
        elif max_level <= 50:
            return "late"
        else:
            return "post_game"

    # Check for location-based phase
    early_locations = ["New Bark", "Cherrygrove", "Violet", "Azalea",
                       "Route 29", "Route 30", "Route 31", "Route 32", "Route 33"]
    late_locations = ["Blackthorn", "Victory Road", "Indigo Plateau",
                      "Ice Path", "Dragon's Den"]
    post_game = ["Kanto", "Mt. Silver", "Trainer Red", "Vermilion",
                 "Cerulean", "Pewter", "Saffron"]

    text_lower = text.lower()
    if any(loc.lower() in text_lower for loc in post_game):
        return "post_game"
    if any(loc.lower() in text_lower for loc in late_locations):
        return "late"
    if any(loc.lower() in text_lower for loc in early_locations):
        return "early"

    return "all"
```

---

## 4. Cross-Reference Index

### Building the Index

After all chunks are created, build a cross-reference graph:

```python
def build_cross_references(chunks: list[dict]) -> list[dict]:
    """Add related_chunks to each chunk based on shared entities."""

    # Build entity → chunk_id index
    entity_index: dict[str, list[str]] = {}
    for chunk in chunks:
        for entity in chunk["entities"]:
            if entity not in entity_index:
                entity_index[entity] = []
            entity_index[entity].append(chunk["chunk_id"])

    # For each chunk, find chunks that share entities
    for chunk in chunks:
        related = set()
        for entity in chunk["entities"]:
            for related_id in entity_index[entity]:
                if related_id != chunk["chunk_id"]:
                    related.add(related_id)

        # Limit to top 10 most related (by shared entity count)
        related_scored = {}
        for rid in related:
            # Find the related chunk
            related_chunk = next(c for c in chunks if c["chunk_id"] == rid)
            shared = len(set(chunk["entities"]) & set(related_chunk["entities"]))
            related_scored[rid] = shared

        top_related = sorted(related_scored, key=related_scored.get, reverse=True)[:10]
        chunk["related_chunks"] = top_related

    return chunks
```

### Manual Cross-References to Add

Some relationships are semantic rather than entity-based:

```python
MANUAL_CROSS_REFS = {
    # Move → mechanic it uses
    "moves_*_rollout_*": ["mechanics_damage_rollout_*"],
    "moves_*_fury_cutter_*": ["mechanics_damage_doubling_*"],

    # Species → strategy content
    "species_*_snorlax_*": ["competitive_ou_snorlax_*", "strategy_counter_snorlax_*"],

    # Gym leader → recommended team
    "trainers_gym_whitney_*": ["strategy_gym_whitney_*", "species_*_machop_*"],

    # Location → wild encounters
    "locations_johto_route_29_*": ["encounters_johto_route_29_*"],
}
```

---

## 5. Sample Chunking Pipeline (Python)

```python
#!/usr/bin/env python3
"""
Pokemon Crystal Vector DB — Chunking Pipeline
Processes raw markdown files into embedding-ready chunks with metadata.
"""

import re
import json
import hashlib
from pathlib import Path
from dataclasses import dataclass, asdict
from typing import Optional

import tiktoken  # pip install tiktoken

# Initialize tokenizer
enc = tiktoken.encoding_for_model("text-embedding-3-large")


@dataclass
class Chunk:
    chunk_id: str
    text: str
    content_type: str
    category: str
    subcategory: str
    entities: list[str]
    difficulty: str
    source_file: str
    source_section: str
    game_phase: str
    region: str
    tags: list[str]
    related_chunks: list[str]
    token_count: int


def count_tokens(text: str) -> int:
    """Count tokens using the embedding model's tokenizer."""
    return len(enc.encode(text))


def chunk_by_headers(
    text: str,
    max_tokens: int = 500,
    overlap_tokens: int = 50,
    header_pattern: str = r'^#{1,4}\s+'
) -> list[tuple[str, str]]:
    """Split text by markdown headers, respecting token limits.

    Returns list of (section_header, chunk_text) tuples.
    """
    lines = text.split('\n')
    chunks = []
    current_header = ""
    current_text = []

    for line in lines:
        if re.match(header_pattern, line):
            # Save previous chunk if it exists
            if current_text:
                chunk_text = '\n'.join(current_text).strip()
                if chunk_text:
                    # Split further if too large
                    sub_chunks = split_if_too_large(
                        chunk_text, max_tokens, overlap_tokens
                    )
                    for sc in sub_chunks:
                        chunks.append((current_header, sc))

            current_header = line.strip('#').strip()
            current_text = [line]
        else:
            current_text.append(line)

    # Don't forget the last section
    if current_text:
        chunk_text = '\n'.join(current_text).strip()
        if chunk_text:
            sub_chunks = split_if_too_large(
                chunk_text, max_tokens, overlap_tokens
            )
            for sc in sub_chunks:
                chunks.append((current_header, sc))

    return chunks


def split_if_too_large(
    text: str,
    max_tokens: int,
    overlap_tokens: int
) -> list[str]:
    """Split text into chunks if it exceeds max_tokens."""
    tokens = count_tokens(text)
    if tokens <= max_tokens:
        return [text]

    # Split by paragraphs first
    paragraphs = text.split('\n\n')
    chunks = []
    current_chunk = []
    current_tokens = 0

    for para in paragraphs:
        para_tokens = count_tokens(para)

        if current_tokens + para_tokens > max_tokens and current_chunk:
            # Save current chunk
            chunks.append('\n\n'.join(current_chunk))

            # Overlap: keep last paragraph
            if overlap_tokens > 0 and current_chunk:
                last = current_chunk[-1]
                if count_tokens(last) <= overlap_tokens:
                    current_chunk = [last]
                    current_tokens = count_tokens(last)
                else:
                    current_chunk = []
                    current_tokens = 0
            else:
                current_chunk = []
                current_tokens = 0

        current_chunk.append(para)
        current_tokens += para_tokens

    if current_chunk:
        chunks.append('\n\n'.join(current_chunk))

    return chunks


def generate_chunk_id(
    category: str,
    subcategory: str,
    identifier: str,
    seq: int
) -> str:
    """Generate a deterministic chunk ID."""
    # Normalize
    identifier = re.sub(r'[^a-z0-9]', '_', identifier.lower())
    identifier = re.sub(r'_+', '_', identifier).strip('_')
    return f"{category}_{subcategory}_{identifier}_{seq:03d}"


def process_species_file(file_path: str) -> list[Chunk]:
    """Process the all_species.md file into per-Pokemon chunks."""
    text = Path(file_path).read_text()
    chunks = []

    # Split by Pokemon entries (### headers)
    sections = chunk_by_headers(text, max_tokens=400, overlap_tokens=0,
                                 header_pattern=r'^###\s+')

    for i, (header, content) in enumerate(sections):
        # Extract Pokemon name from header
        pokemon_name = header.split('(')[0].strip().split('#')[-1].strip()
        if not pokemon_name:
            pokemon_name = f"section_{i}"

        chunk = Chunk(
            chunk_id=generate_chunk_id("species", "data", pokemon_name, 1),
            text=content,
            content_type="factual",
            category="species",
            subcategory=extract_type_subcategory(content),
            entities=extract_game_entities(content),
            difficulty="beginner",
            source_file=file_path,
            source_section=header,
            game_phase="all",
            region=infer_region_from_text(content),
            tags=extract_tags_from_text(content),
            related_chunks=[],
            token_count=count_tokens(content),
        )
        chunks.append(chunk)

    return chunks


def process_qa_file(file_path: str) -> list[Chunk]:
    """Process qa_pairs.md into individual Q&A chunks."""
    text = Path(file_path).read_text()
    chunks = []

    # Parse Q&A pairs
    qa_pattern = re.compile(
        r'\*\*Q:\*\*\s*(.*?)\n\*\*A:\*\*\s*(.*?)\n\*\*Category:\*\*\s*(\w+)\s*\*\*Difficulty:\*\*\s*(\w+)',
        re.DOTALL
    )

    # Simpler pattern for our format
    blocks = re.split(r'###\s+Q\d+', text)

    for i, block in enumerate(blocks):
        if not block.strip():
            continue

        q_match = re.search(r'\*\*Q:\*\*\s*(.*?)$', block, re.MULTILINE)
        a_match = re.search(r'\*\*A:\*\*\s*(.*?)$', block, re.MULTILINE)
        cat_match = re.search(r'\*\*Category:\*\*\s*(\w+)', block)
        diff_match = re.search(r'\*\*Difficulty:\*\*\s*(\w+)', block)

        if q_match and a_match:
            question = q_match.group(1).strip()
            answer = a_match.group(1).strip()
            category = cat_match.group(1) if cat_match else "general"
            difficulty = diff_match.group(1) if diff_match else "beginner"

            qa_text = f"Question: {question}\nAnswer: {answer}"

            chunk = Chunk(
                chunk_id=generate_chunk_id("qa", category, f"q{i+1:03d}", 1),
                text=qa_text,
                content_type="qa",
                category="qa",
                subcategory=category,
                entities=extract_game_entities(qa_text),
                difficulty=difficulty,
                source_file=file_path,
                source_section=f"Q{i+1}",
                game_phase="all",
                region="both",
                tags=["evaluation", "qa"],
                related_chunks=[],
                token_count=count_tokens(qa_text),
            )
            chunks.append(chunk)

    return chunks


def process_generic_file(
    file_path: str,
    category: str,
    max_tokens: int = 500,
    overlap: int = 50
) -> list[Chunk]:
    """Process any markdown file into chunks."""
    text = Path(file_path).read_text()
    sections = chunk_by_headers(text, max_tokens=max_tokens,
                                 overlap_tokens=overlap)
    chunks = []

    for i, (header, content) in enumerate(sections):
        identifier = re.sub(r'[^a-z0-9]', '_', header.lower())[:30]

        chunk = Chunk(
            chunk_id=generate_chunk_id(category, "general", identifier, i+1),
            text=content,
            content_type=classify_content_type_from_text(content, category),
            category=category,
            subcategory=extract_subcategory_from_header(header),
            entities=extract_game_entities(content),
            difficulty=classify_difficulty_from_text(content),
            source_file=file_path,
            source_section=header,
            game_phase=infer_game_phase_from_text(content),
            region=infer_region_from_text(content),
            tags=extract_tags_from_text(content),
            related_chunks=[],
            token_count=count_tokens(content),
        )
        chunks.append(chunk)

    return chunks


# =============================================================================
# MAIN PIPELINE
# =============================================================================

def run_pipeline(data_dir: str = "data/") -> list[dict]:
    """Run the full chunking pipeline on all data files."""

    all_chunks: list[Chunk] = []
    data_path = Path(data_dir)

    # Process each file type with appropriate settings
    file_configs = [
        # (glob_pattern, category, max_tokens, overlap)
        ("species/*.md", "species", 400, 0),
        ("moves/*.md", "moves", 300, 0),
        ("types/*.md", "types", 500, 100),
        ("trainers/*.md", "trainers", 500, 50),
        ("encounters/*.md", "encounters", 400, 0),
        ("items/*.md", "items", 300, 0),
        ("mechanics/*.md", "mechanics", 800, 100),
        ("maps/*.md", "locations", 500, 50),
        ("strategy/*.md", "strategy", 600, 50),
    ]

    for pattern, category, max_tok, overlap in file_configs:
        for file_path in data_path.glob(pattern):
            print(f"Processing {file_path}...")
            chunks = process_generic_file(
                str(file_path), category, max_tok, overlap
            )
            all_chunks.extend(chunks)

    # Process Q&A pairs specially
    qa_path = data_path / "meta" / "qa_pairs.md"
    if qa_path.exists():
        print(f"Processing {qa_path}...")
        qa_chunks = process_qa_file(str(qa_path))
        all_chunks.extend(qa_chunks)

    # Build cross-references
    print("Building cross-references...")
    all_chunks = build_cross_references_for_chunks(all_chunks)

    # Convert to dicts for serialization
    chunk_dicts = [asdict(c) for c in all_chunks]

    # Write output
    output_path = data_path / "meta" / "chunks.jsonl"
    with open(output_path, 'w') as f:
        for chunk in chunk_dicts:
            f.write(json.dumps(chunk) + '\n')

    print(f"Generated {len(chunk_dicts)} chunks → {output_path}")

    # Write stats
    stats = {
        "total_chunks": len(chunk_dicts),
        "total_tokens": sum(c["token_count"] for c in chunk_dicts),
        "by_category": {},
        "by_difficulty": {},
        "by_content_type": {},
    }
    for c in chunk_dicts:
        for field in ["category", "difficulty", "content_type"]:
            key = f"by_{field}"
            val = c[field]
            stats[key][val] = stats[key].get(val, 0) + 1

    stats_path = data_path / "meta" / "chunk_stats.json"
    with open(stats_path, 'w') as f:
        json.dump(stats, f, indent=2)

    print(f"Stats → {stats_path}")
    return chunk_dicts


if __name__ == "__main__":
    run_pipeline()
```

---

## 6. Post-Processing Validation

After generating chunks, run these validation checks:

```python
def validate_chunks(chunks: list[dict]) -> list[str]:
    """Validate chunk quality and consistency."""
    errors = []
    chunk_ids = set()

    for chunk in chunks:
        # Check for duplicate IDs
        if chunk["chunk_id"] in chunk_ids:
            errors.append(f"Duplicate chunk_id: {chunk['chunk_id']}")
        chunk_ids.add(chunk["chunk_id"])

        # Check token count
        if chunk["token_count"] > 1000:
            errors.append(
                f"Chunk {chunk['chunk_id']} exceeds 1000 tokens "
                f"({chunk['token_count']})"
            )

        if chunk["token_count"] < 20:
            errors.append(
                f"Chunk {chunk['chunk_id']} too small "
                f"({chunk['token_count']} tokens)"
            )

        # Check for empty fields
        if not chunk["text"].strip():
            errors.append(f"Empty text in {chunk['chunk_id']}")

        if not chunk["entities"]:
            errors.append(
                f"No entities extracted for {chunk['chunk_id']} "
                f"(might be OK for meta content)"
            )

        # Check related_chunks reference valid IDs
        for ref in chunk["related_chunks"]:
            if ref not in chunk_ids:
                errors.append(
                    f"Invalid cross-reference {ref} in {chunk['chunk_id']}"
                )

        # Check for Gen 3+ concepts that shouldn't be in Gen 2 data
        gen3_terms = ["ability", "nature ", "EVs ", "Fairy type",
                      "Mega Evolution", "Z-Move"]
        for term in gen3_terms:
            if term.lower() in chunk["text"].lower():
                # Allow if explicitly saying "doesn't exist"
                if "not" in chunk["text"].lower() or "no " in chunk["text"].lower():
                    continue
                errors.append(
                    f"Possible Gen 3+ content in {chunk['chunk_id']}: "
                    f"found '{term}'"
                )

    return errors
```

---

## 7. Embedding and Upload

### Embedding with OpenAI

```python
import openai

client = openai.OpenAI()

def embed_chunks(chunks: list[dict], batch_size: int = 100) -> list[dict]:
    """Embed all chunks using text-embedding-3-large."""
    for i in range(0, len(chunks), batch_size):
        batch = chunks[i:i+batch_size]
        texts = [c["text"] for c in batch]

        response = client.embeddings.create(
            model="text-embedding-3-large",
            input=texts,
            dimensions=1536  # Truncate for cost savings
        )

        for j, embedding_data in enumerate(response.data):
            batch[j]["embedding"] = embedding_data.embedding

        print(f"Embedded {i+len(batch)}/{len(chunks)} chunks")

    return chunks
```

### Upload to Vector Database

```python
# Example: Pinecone
import pinecone

def upload_to_pinecone(chunks: list[dict], index_name: str = "pokemon-crystal"):
    pc = pinecone.Pinecone()
    index = pc.Index(index_name)

    vectors = []
    for chunk in chunks:
        vectors.append({
            "id": chunk["chunk_id"],
            "values": chunk["embedding"],
            "metadata": {
                k: v for k, v in chunk.items()
                if k not in ("text", "embedding", "chunk_id")
                and not isinstance(v, list)  # Pinecone metadata limitations
            }
        })

    # Upsert in batches of 100
    for i in range(0, len(vectors), 100):
        batch = vectors[i:i+100]
        index.upsert(vectors=batch)

    print(f"Uploaded {len(vectors)} vectors to {index_name}")
```

### Alternative: Qdrant (Open Source)

```python
from qdrant_client import QdrantClient
from qdrant_client.models import VectorParams, Distance, PointStruct

def upload_to_qdrant(chunks: list[dict], collection_name: str = "pokemon-crystal"):
    client = QdrantClient(host="localhost", port=6333)

    # Create collection
    client.create_collection(
        collection_name=collection_name,
        vectors_config=VectorParams(size=1536, distance=Distance.COSINE),
    )

    # Upload
    points = [
        PointStruct(
            id=i,
            vector=chunk["embedding"],
            payload={
                "chunk_id": chunk["chunk_id"],
                "text": chunk["text"],
                "category": chunk["category"],
                "entities": chunk["entities"],
                "difficulty": chunk["difficulty"],
                "game_phase": chunk["game_phase"],
                "region": chunk["region"],
                "tags": chunk["tags"],
            }
        )
        for i, chunk in enumerate(chunks)
    ]

    client.upsert(collection_name=collection_name, points=points)
    print(f"Uploaded {len(points)} points to {collection_name}")
```

---

## 8. Estimated Costs

### Embedding Costs (text-embedding-3-large at 1536 dims)
- ~1000 chunks * ~400 avg tokens = ~400K tokens
- Cost: ~$0.05 (very cheap)
- One-time cost; re-embed only when data changes

### Storage Costs
- 1000 vectors * 1536 dims * 4 bytes = ~6MB of vector data
- Plus metadata: ~2MB
- Total: ~8MB (trivial)

### Query Costs (per query)
- Embedding the query: ~50 tokens = $0.000006
- Vector search: depends on provider (Pinecone: ~$0.0004/query)
- LLM generation: ~4000 context tokens + ~500 output = ~$0.01 per query
- **Total per query: ~$0.01**
