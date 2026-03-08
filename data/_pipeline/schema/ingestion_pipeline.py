#!/usr/bin/env python3
"""
Pokemon Crystal Vector DB Ingestion Pipeline

Processes raw .md data files from data/ subdirectories into JSONL chunks
following the schema defined in schema.md.

Usage:
    python ingestion_pipeline.py                    # Process all files
    python ingestion_pipeline.py --source species   # Process only species
    python ingestion_pipeline.py --validate         # Validate existing JSONL
    python ingestion_pipeline.py --stats            # Print statistics
"""

import json
import os
import re
import sys
import argparse
from pathlib import Path
from typing import Optional

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------

BASE_DIR = Path(__file__).resolve().parent.parent  # data/
SCHEMA_DIR = BASE_DIR / "schema"
OUTPUT_DIR = BASE_DIR / "chunks"

# Raw data sources — original tabular files (v1 parsers)
RAW_SOURCES = {
    "species": BASE_DIR / "species" / "all_pokemon.md",
    "moves": BASE_DIR / "moves" / "all_moves.md",
    "types": BASE_DIR / "types" / "type_chart.md",
    "items": BASE_DIR / "items" / "all_items.md",
    "encounters": BASE_DIR / "encounters" / "wild_encounters.md",
    "trainers": BASE_DIR / "trainers" / "all_trainers.md",
}

# New prose/supplementary data files (v2 parsers)
# Format: (file_path, doc_type, category, subcategory, tags, header_level)
PROSE_SOURCES: list[tuple[Path, str, str, str, list[str], int]] = [
    # Mechanics (prose, ## or ### sections)
    (BASE_DIR / "mechanics" / "damage_formula.md", "mechanic", "battle_system", "damage_calc", ["damage", "formula", "verified_asm"], 2),
    (BASE_DIR / "mechanics" / "status_effects.md", "mechanic", "battle_system", "status_effects", ["status_condition", "verified_asm"], 2),
    (BASE_DIR / "mechanics" / "battle_mechanics.md", "mechanic", "battle_system", "damage_calc", ["damage", "verified_asm"], 2),
    (BASE_DIR / "mechanics" / "move_edge_cases.md", "battle_rule", "battle_system", "move_interactions", ["edge_case", "verified_asm"], 2),
    (BASE_DIR / "mechanics" / "ai_behavior.md", "mechanic", "battle_system", "ai_behavior", ["verified_asm"], 2),
    (BASE_DIR / "mechanics" / "stat_calculation.md", "mechanic", "battle_system", "stat_stages", ["formula", "verified_asm"], 2),
    (BASE_DIR / "mechanics" / "breeding_mechanics.md", "mechanic", "pokemon_data", "breeding", ["derived"], 2),
    (BASE_DIR / "mechanics" / "evolution_mechanics.md", "mechanic", "pokemon_data", "evolution_methods", ["derived"], 2),
    (BASE_DIR / "mechanics" / "formulas_reference.md", "mechanic", "battle_system", "damage_calc", ["formula", "verified_asm"], 2),
    (BASE_DIR / "mechanics" / "glitches_quirks.md", "battle_rule", "battle_system", "glitches", ["edge_case", "verified_asm"], 2),
    (BASE_DIR / "mechanics" / "battle_scenarios.md", "battle_rule", "battle_system", "move_interactions", ["edge_case"], 2),
    (BASE_DIR / "mechanics" / "link_battle.md", "mechanic", "battle_system", "link_battle", ["derived"], 2),
    # Maps / Locations (### per location)
    (BASE_DIR / "maps" / "johto_locations.md", "map_location", "overworld", "cities_towns", ["johto"], 3),
    (BASE_DIR / "maps" / "kanto_locations.md", "map_location", "overworld", "cities_towns", ["kanto", "post_game"], 3),
    (BASE_DIR / "maps" / "game_progression.md", "story_event", "story_progression", "main_quest", ["step_by_step"], 2),
    (BASE_DIR / "maps" / "npc_dialogue.md", "game_corner", "side_features", "npc_services", ["derived"], 2),
    (BASE_DIR / "maps" / "ruins_of_alph.md", "map_location", "overworld", "dungeons", ["johto"], 2),
    # Strategy
    (BASE_DIR / "strategy" / "gym_strategies.md", "strategy", "meta_game", "boss_strategies", ["gym_leader", "strategy", "beginner"], 3),
    (BASE_DIR / "strategy" / "elite_four_strategies.md", "strategy", "meta_game", "boss_strategies", ["elite_four", "strategy"], 3),
    (BASE_DIR / "strategy" / "team_building.md", "strategy", "meta_game", "team_building", ["team_building", "beginner"], 2),
    (BASE_DIR / "strategy" / "speedrun_notes.md", "strategy", "meta_game", "speedrun", ["advanced"], 2),
    (BASE_DIR / "strategy" / "competitive_gen2.md", "strategy", "meta_game", "competitive", ["advanced"], 2),
    # Species supplements
    (BASE_DIR / "species" / "competitive_pokemon.md", "strategy", "meta_game", "competitive", ["advanced"], 2),
    (BASE_DIR / "species" / "pokemon_trivia.md", "game_corner", "side_features", "trivia", ["derived"], 2),
    # Encounters supplements
    (BASE_DIR / "encounters" / "special_encounters.md", "wild_encounter", "overworld", "special_encounters", ["legendary"], 2),
    (BASE_DIR / "encounters" / "battle_tower.md", "trainer", "trainer_data", "battle_tower", ["post_game"], 2),
    (BASE_DIR / "encounters" / "item_locations.md", "item", "items_data", "item_locations", ["list"], 2),
    # Trainer supplements
    (BASE_DIR / "trainers" / "gym_leader_rematches.md", "trainer", "trainer_data", "rematch_trainers", ["derived"], 2),
    # Additional mechanics (added post-v2)
    (BASE_DIR / "mechanics" / "rng_mechanics.md", "mechanic", "battle_system", "rng", ["verified_asm"], 2),
    (BASE_DIR / "mechanics" / "state_machine.md", "mechanic", "battle_system", "state_machine", ["verified_asm"], 2),
    (BASE_DIR / "mechanics" / "time_system.md", "mechanic", "overworld", "time_system", ["derived"], 2),
    (BASE_DIR / "mechanics" / "pokegear.md", "mechanic", "side_features", "pokegear", ["derived"], 2),
    (BASE_DIR / "mechanics" / "radio_system.md", "mechanic", "side_features", "radio", ["derived"], 2),
    (BASE_DIR / "mechanics" / "mystery_gift_decorations.md", "game_corner", "side_features", "mystery_gift", ["derived"], 2),
    # Maps supplements
    (BASE_DIR / "maps" / "music_by_location.md", "game_corner", "music_art", "music_tracks", ["derived"], 2),
]

# QA pairs file (special parser)
QA_FILE = BASE_DIR / "meta" / "qa_pairs.md"
# Learnset-by-move reverse index (special parser)
LEARNSET_BY_MOVE_FILE = BASE_DIR / "species" / "learnset_by_move.md"
# Evolution chains (special parser)
EVOLUTION_CHAINS_FILE = BASE_DIR / "species" / "evolution_chains.md"
# Meta files (skip — they are pipeline instructions, not game data)
META_SKIP_FILES = {
    BASE_DIR / "meta" / "embedding_prep.md",
    BASE_DIR / "meta" / "optimization_guide.md",
}

# Gen 2 physical/special split by type
PHYSICAL_TYPES = {
    "normal", "fighting", "flying", "poison", "ground",
    "rock", "bug", "ghost", "steel",
}
SPECIAL_TYPES = {
    "fire", "water", "electric", "grass", "ice",
    "psychic", "dragon", "dark",
}

# Type weakness/resistance chart (for species chunks)
TYPE_CHART = {
    ("normal", "rock"): 0.5, ("normal", "steel"): 0.5, ("normal", "ghost"): 0,
    ("fire", "fire"): 0.5, ("fire", "water"): 0.5, ("fire", "grass"): 2,
    ("fire", "ice"): 2, ("fire", "bug"): 2, ("fire", "rock"): 0.5,
    ("fire", "dragon"): 0.5, ("fire", "steel"): 2,
    ("water", "fire"): 2, ("water", "water"): 0.5, ("water", "grass"): 0.5,
    ("water", "ground"): 2, ("water", "rock"): 2, ("water", "dragon"): 0.5,
    ("electric", "water"): 2, ("electric", "electric"): 0.5,
    ("electric", "grass"): 0.5, ("electric", "ground"): 0,
    ("electric", "flying"): 2, ("electric", "dragon"): 0.5,
    ("grass", "fire"): 0.5, ("grass", "water"): 2, ("grass", "grass"): 0.5,
    ("grass", "poison"): 0.5, ("grass", "ground"): 2, ("grass", "flying"): 0.5,
    ("grass", "bug"): 0.5, ("grass", "rock"): 2, ("grass", "dragon"): 0.5,
    ("grass", "steel"): 0.5,
    ("ice", "fire"): 0.5, ("ice", "water"): 0.5, ("ice", "grass"): 2,
    ("ice", "ice"): 0.5, ("ice", "ground"): 2, ("ice", "flying"): 2,
    ("ice", "dragon"): 2, ("ice", "steel"): 0.5,
    ("fighting", "normal"): 2, ("fighting", "ice"): 2, ("fighting", "poison"): 0.5,
    ("fighting", "flying"): 0.5, ("fighting", "psychic"): 0.5,
    ("fighting", "bug"): 0.5, ("fighting", "rock"): 2, ("fighting", "ghost"): 0,
    ("fighting", "dark"): 2, ("fighting", "steel"): 2,
    ("poison", "grass"): 2, ("poison", "poison"): 0.5, ("poison", "ground"): 0.5,
    ("poison", "rock"): 0.5, ("poison", "ghost"): 0.5, ("poison", "steel"): 0,
    ("ground", "fire"): 2, ("ground", "electric"): 2, ("ground", "grass"): 0.5,
    ("ground", "poison"): 2, ("ground", "flying"): 0, ("ground", "bug"): 0.5,
    ("ground", "rock"): 2, ("ground", "steel"): 2,
    ("flying", "electric"): 0.5, ("flying", "grass"): 2, ("flying", "fighting"): 2,
    ("flying", "bug"): 2, ("flying", "rock"): 0.5, ("flying", "steel"): 0.5,
    ("psychic", "fighting"): 2, ("psychic", "poison"): 2,
    ("psychic", "psychic"): 0.5, ("psychic", "dark"): 0, ("psychic", "steel"): 0.5,
    ("bug", "fire"): 0.5, ("bug", "grass"): 2, ("bug", "fighting"): 0.5,
    ("bug", "poison"): 0.5, ("bug", "flying"): 0.5, ("bug", "psychic"): 2,
    ("bug", "ghost"): 0.5, ("bug", "dark"): 2, ("bug", "steel"): 0.5,
    ("rock", "fire"): 2, ("rock", "ice"): 2, ("rock", "fighting"): 0.5,
    ("rock", "ground"): 0.5, ("rock", "flying"): 2, ("rock", "bug"): 2,
    ("rock", "steel"): 0.5,
    ("ghost", "normal"): 0, ("ghost", "psychic"): 2, ("ghost", "dark"): 0.5,
    ("ghost", "steel"): 0.5, ("ghost", "ghost"): 2,
    ("dragon", "dragon"): 2, ("dragon", "steel"): 0.5,
    ("dark", "fighting"): 0.5, ("dark", "psychic"): 2, ("dark", "ghost"): 2,
    ("dark", "dark"): 0.5, ("dark", "steel"): 0.5,
    ("steel", "fire"): 0.5, ("steel", "water"): 0.5, ("steel", "electric"): 0.5,
    ("steel", "ice"): 2, ("steel", "rock"): 2, ("steel", "steel"): 0.5,
}

ALL_TYPES = [
    "normal", "fire", "water", "electric", "grass", "ice", "fighting",
    "poison", "ground", "flying", "psychic", "bug", "rock", "ghost",
    "dragon", "dark", "steel",
]

# Standardized tags from taxonomy
VALID_TAGS = {
    "starter", "legendary", "mythical", "pseudo_legendary", "baby_pokemon",
    "trade_evolution", "johto_native", "kanto_native", "gen1", "gen2_new", "unown",
    "physical", "special", "status", "contact", "sound_based", "punch_move",
    "high_priority", "negative_priority", "never_miss", "multi_hit", "two_turn",
    "recoil", "drain", "self_destruct", "weather_boosted", "field_effect",
    "normal", "fire", "water", "electric", "grass", "ice", "fighting", "poison",
    "ground", "flying", "psychic", "bug", "rock", "ghost", "dragon", "dark", "steel",
    "gym_leader", "elite_four", "champion", "rival", "rocket_grunt", "rocket_admin",
    "johto", "kanto", "indoor", "outdoor", "cave", "tower", "forest", "ocean",
    "early_game", "mid_game", "late_game", "post_game",
    "damage", "accuracy", "evasion", "critical", "weather", "priority", "switching",
    "status_condition", "stat_boost", "stat_drop", "turn_order", "faint",
    "experience", "capture", "formula", "table", "list", "step_by_step",
    "edge_case", "exception", "beginner", "intermediate", "advanced",
    "verified_asm", "derived", "approximate",
    "damaging_moves", "status_moves", "hm_moves", "type_items",
    "type_chart", "stab", "burn", "freeze", "rain",
    "held_items_battle", "pokeballs", "team_building", "strategy",
    "team_rocket", "grass",
}


def slugify(name: str) -> str:
    """Convert a name to a chunk-ID-safe slug."""
    s = name.lower().strip()
    s = s.replace("'", "").replace(".", "").replace("'", "")
    s = re.sub(r"[^a-z0-9]+", "_", s)
    s = s.strip("_")
    return s


def estimate_tokens(text: str) -> int:
    """Rough token estimate: ~4 characters per token for English."""
    return max(1, len(text) // 4)


def get_defensive_weaknesses(type1: str, type2: Optional[str]) -> dict:
    """Calculate defensive type matchups for a species."""
    multipliers = {}
    for atk_type in ALL_TYPES:
        m = 1.0
        m *= TYPE_CHART.get((atk_type, type1), 1.0)
        if type2 and type2 != "none":
            m *= TYPE_CHART.get((atk_type, type2), 1.0)
        if m != 1.0:
            multipliers[atk_type] = m
    return multipliers


# ---------------------------------------------------------------------------
# Parsers — one per raw data source
# ---------------------------------------------------------------------------

def parse_species(filepath: Path) -> list[dict]:
    """Parse all_pokemon.md into species + learnset + evolution chunks."""
    chunks = []
    text = filepath.read_text(encoding="utf-8")

    # Split into individual Pokemon sections
    pokemon_sections = re.split(r"^## #(\d+) ([A-Z][A-Z0-9 .'♀♂-]+)$", text, flags=re.MULTILINE)
    # Result: [preamble, dex_num, name, section_body, dex_num, name, section_body, ...]

    i = 1
    while i < len(pokemon_sections) - 2:
        dex_num = int(pokemon_sections[i])
        raw_name = pokemon_sections[i + 1].strip()
        body = pokemon_sections[i + 2]
        i += 3

        name = raw_name.title()
        slug = slugify(raw_name)

        # Parse type
        type_match = re.search(r"\*\*Type:\*\*\s*(.+)", body)
        types_str = type_match.group(1).strip() if type_match else "Normal"
        type_parts = [t.strip().lower() for t in types_str.split("/")]
        type1 = type_parts[0]
        type2 = type_parts[1] if len(type_parts) > 1 else "none"

        # Parse base stats
        stats_match = re.search(
            r"\|\s*(\d+)\s*\|\s*(\d+)\s*\|\s*(\d+)\s*\|\s*(\d+)\s*\|\s*(\d+)\s*\|\s*(\d+)\s*\|\s*(\d+)\s*\|",
            body
        )
        if stats_match:
            hp, atk, dfn, spd, spa, spdef, bst = [int(x) for x in stats_match.groups()]
        else:
            hp = atk = dfn = spd = spa = spdef = bst = 0

        # Parse details
        catch_rate = _extract_int(body, r"\*\*Catch Rate:\*\*\s*(\d+)")
        growth_match = re.search(r"\*\*Growth Rate:\*\*\s*(.+)", body)
        growth_rate = slugify(growth_match.group(1).strip()) if growth_match else "medium_fast"
        gender_match = re.search(r"\*\*Gender Ratio:\*\*\s*(.+)", body)
        gender_ratio = gender_match.group(1).strip() if gender_match else "50% male"
        egg_match = re.search(r"\*\*Egg Groups:\*\*\s*(.+)", body)
        egg_groups = [slugify(g.strip()) for g in egg_match.group(1).split(",")] if egg_match else []

        # Parse evolution
        evo_lines = re.findall(r"[-*]\s*(.+?)\s*->\s*(.+)", body)
        evo_text = ""
        evo_family = [name]
        for evo_cond, evo_target in evo_lines:
            evo_family.append(evo_target.strip())
            evo_text += f"\n{name} evolves: {evo_cond.strip()} → {evo_target.strip()}"

        # Parse level-up moves
        level_moves = re.findall(r"\|\s*(\d+)\s*\|\s*(.+?)\s*\|", body)
        # Filter out header row
        level_moves = [(lv, mv.strip()) for lv, mv in level_moves if mv.strip() and not mv.strip().startswith("-")]

        # Parse TM/HM
        tmhm_match = re.search(r"### TM/HM Compatibility\s*\n\s*\n(.+?)(?:\n\n|\n---|\Z)", body, re.DOTALL)
        tmhm_list = []
        if tmhm_match:
            tmhm_text_raw = tmhm_match.group(1).strip()
            tmhm_list = [m.strip() for m in tmhm_text_raw.split(",") if m.strip()]

        # Parse egg moves
        egg_move_match = re.search(r"### Egg Moves\s*\n\s*\n(.+?)(?:\n\n|\n---|\n###|\Z)", body, re.DOTALL)
        egg_move_list = []
        if egg_move_match:
            egg_move_list = [m.strip() for m in egg_move_match.group(1).strip().split(",") if m.strip()]

        # Build tags
        tags = [type1]
        if type2 != "none":
            tags.append(type2)
        if dex_num <= 151:
            tags.append("gen1")
        else:
            tags.append("gen2_new")
            tags.append("johto_native")
        if bst >= 600 and dex_num not in (150, 151, 249, 250, 251):
            tags.append("pseudo_legendary")
        if dex_num in (243, 244, 245, 249, 250):
            tags.append("legendary")
        if dex_num == 251:
            tags.append("mythical")
        if dex_num in (1, 4, 7, 152, 155, 158):
            tags.append("starter")

        # Defensive weaknesses text
        matchups = get_defensive_weaknesses(type1, type2 if type2 != "none" else None)
        weak_to = [f"{t.title()} ({m}x)" for t, m in matchups.items() if m > 1]
        resists = [f"{t.title()} ({m}x)" for t, m in matchups.items() if 0 < m < 1]
        immune_to = [t.title() for t, m in matchups.items() if m == 0]

        type_display = f"{type1.title()}/{type2.title()}" if type2 != "none" else type1.title()

        # --- Species chunk ---
        species_id = f"species_{slug}_001"
        learnset_level_id = f"learnset_{slug}_level_001"
        learnset_tmhm_id = f"learnset_{slug}_tmhm_001"
        evolution_id = f"evolution_{slug}_001"

        related = [learnset_level_id]
        if tmhm_list:
            related.append(learnset_tmhm_id)
        if evo_lines:
            related.append(evolution_id)

        defense_text = ""
        if weak_to:
            defense_text += f"\nWeak to: {', '.join(weak_to)}."
        if resists:
            defense_text += f"\nResists: {', '.join(resists)}."
        if immune_to:
            defense_text += f"\nImmune to: {', '.join(immune_to)}."

        species_text = (
            f"[Pokemon Crystal] {name} (#{dex_num:03d}) — {type_display} Pokemon\n"
            f"Base Stats: HP {hp} / Atk {atk} / Def {dfn} / SpA {spa} / SpD {spdef} / Spe {spd} (BST {bst})\n\n"
            f"{name} is a {type_display}-type Pokemon. "
            f"Catch rate: {catch_rate}. Growth rate: {growth_rate.replace('_', ' ').title()}. "
            f"Gender: {gender_ratio}. Egg groups: {', '.join(g.replace('_', ' ').title() for g in egg_groups)}."
            f"{evo_text}"
            f"{defense_text}"
        )

        chunks.append({
            "id": species_id,
            "doc_type": "species",
            "name": name,
            "category": "pokemon_data",
            "subcategory": "base_stats",
            "tags": tags,
            "related_entities": related,
            "source": "pokecrystal",
            "source_file": f"data/pokemon/base_stats/{slug}.asm",
            "generation": 2,
            "game": "crystal",
            "dex_number": dex_num,
            "type1": type1,
            "type2": type2,
            "base_stats": {"hp": hp, "atk": atk, "def": dfn, "spa": spa, "spd": spdef, "spe": spd},
            "catch_rate": catch_rate,
            "growth_rate": growth_rate,
            "egg_groups": egg_groups,
            "gender_ratio": gender_ratio,
            "token_estimate": estimate_tokens(species_text),
            "text": species_text,
        })

        # --- Level-up learnset chunk ---
        if level_moves:
            moves_text_parts = [f"Lv{lv} {mv}" for lv, mv in level_moves]
            learnset_text = (
                f"[Pokemon Crystal] {name} — Level-Up Moves\n\n"
                f"{', '.join(moves_text_parts)}.\n\n"
                f"{name} learns {len(level_moves)} moves by level-up."
            )
            chunks.append({
                "id": learnset_level_id,
                "doc_type": "learnset",
                "name": name,
                "category": "pokemon_data",
                "subcategory": "level_up_learnset",
                "tags": tags[:3],  # type tags + gen tag
                "related_entities": [species_id],
                "source": "pokecrystal",
                "source_file": "data/pokemon/evos_attacks.asm",
                "generation": 2,
                "game": "crystal",
                "species_id": species_id,
                "learnset_type": "level_up",
                "move_list": [mv for _, mv in level_moves],
                "token_estimate": estimate_tokens(learnset_text),
                "text": learnset_text,
            })

        # --- TM/HM learnset chunk ---
        if tmhm_list:
            tmhm_text = (
                f"[Pokemon Crystal] {name} — TM/HM Compatibility\n\n"
                f"{', '.join(tmhm_list)}.\n\n"
                f"{name} is compatible with {len(tmhm_list)} TMs/HMs."
            )
            chunks.append({
                "id": learnset_tmhm_id,
                "doc_type": "learnset",
                "name": name,
                "category": "pokemon_data",
                "subcategory": "tm_hm_learnset",
                "tags": tags[:3],
                "related_entities": [species_id],
                "source": "pokecrystal",
                "source_file": f"data/pokemon/base_stats/{slug}.asm",
                "generation": 2,
                "game": "crystal",
                "species_id": species_id,
                "learnset_type": "tm_hm",
                "move_list": tmhm_list,
                "token_estimate": estimate_tokens(tmhm_text),
                "text": tmhm_text,
            })

        # --- Egg move learnset chunk ---
        if egg_move_list:
            egg_text = (
                f"[Pokemon Crystal] {name} — Egg Moves\n\n"
                f"{', '.join(egg_move_list)}."
            )
            egg_id = f"learnset_{slug}_egg_001"
            chunks.append({
                "id": egg_id,
                "doc_type": "learnset",
                "name": name,
                "category": "pokemon_data",
                "subcategory": "egg_moves",
                "tags": tags[:3],
                "related_entities": [species_id],
                "source": "pokecrystal",
                "source_file": "data/pokemon/egg_moves.asm",
                "generation": 2,
                "game": "crystal",
                "species_id": species_id,
                "learnset_type": "egg_move",
                "move_list": egg_move_list,
                "token_estimate": estimate_tokens(egg_text),
                "text": egg_text,
            })

        # --- Evolution chunk (one per base species only) ---
        if evo_lines:
            evo_desc_parts = []
            for evo_cond, evo_target in evo_lines:
                evo_desc_parts.append(f"{evo_cond.strip()} → {evo_target.strip()}")
            evo_text_chunk = (
                f"[Pokemon Crystal] Evolution: {' → '.join(evo_family)}\n\n"
                + "\n".join(evo_desc_parts)
            )
            chunks.append({
                "id": evolution_id,
                "doc_type": "evolution",
                "name": name,
                "category": "pokemon_data",
                "subcategory": "evolution_chains",
                "tags": tags[:3],
                "related_entities": [species_id],
                "source": "pokecrystal",
                "source_file": "data/pokemon/evos_attacks.asm",
                "generation": 2,
                "game": "crystal",
                "species_id": species_id,
                "evo_family": evo_family,
                "token_estimate": estimate_tokens(evo_text_chunk),
                "text": evo_text_chunk,
            })

    return chunks


def parse_moves(filepath: Path) -> list[dict]:
    """Parse all_moves.md into move chunks."""
    chunks = []
    text = filepath.read_text(encoding="utf-8")

    sections = re.split(r"^## #(\d+) (.+)$", text, flags=re.MULTILINE)

    i = 1
    while i < len(sections) - 2:
        move_num = int(sections[i])
        move_name = sections[i + 1].strip()
        body = sections[i + 2]
        i += 3

        slug = slugify(move_name)

        type_match = re.search(r"\*\*Type:\*\*\s*(\w+)", body)
        move_type = type_match.group(1).strip().lower() if type_match else "normal"

        power_match = re.search(r"\*\*Power:\*\*\s*(\d+|-)", body)
        power = int(power_match.group(1)) if power_match and power_match.group(1) != "-" else 0

        acc_match = re.search(r"\*\*Accuracy:\*\*\s*(\d+)%", body)
        accuracy = int(acc_match.group(1)) if acc_match else 0

        pp_match = re.search(r"\*\*PP:\*\*\s*(\d+)", body)
        pp = int(pp_match.group(1)) if pp_match else 0

        effect_match = re.search(r"\*\*Effect Chance:\*\*\s*(\d+)%", body)
        effect_chance = int(effect_match.group(1)) if effect_match else 0

        # Description is everything after the metadata block
        desc_lines = []
        for line in body.strip().split("\n"):
            line = line.strip()
            if line and not line.startswith("**") and not line.startswith("---"):
                desc_lines.append(line)
        description = " ".join(desc_lines).strip()

        category = "physical" if move_type in PHYSICAL_TYPES else "special"
        if power == 0 and not description.lower().startswith("deal"):
            category = "status"

        tags = [move_type]
        if category == "status":
            tags.append("status")
            tags.append("status_moves")
        else:
            tags.append(category)
            tags.append("damaging_moves")

        move_display_cat = category.title()
        power_str = str(power) if power > 0 else "-"

        chunk_text = (
            f"[Pokemon Crystal] Move: {move_name} — {move_type.title()} / {move_display_cat} / "
            f"Power {power_str} / Acc {accuracy}% / PP {pp}\n\n"
            f"{description}"
        )
        if effect_chance:
            chunk_text += f"\n\nSecondary effect chance: {effect_chance}%."

        chunks.append({
            "id": f"move_{slug}_001",
            "doc_type": "move",
            "name": move_name,
            "category": "moves_data",
            "subcategory": "move_stats",
            "tags": tags,
            "related_entities": [f"type_{move_type}_offensive_001"],
            "source": "pokecrystal",
            "source_file": "data/moves/moves.asm",
            "generation": 2,
            "game": "crystal",
            "move_type": move_type,
            "move_category": category,
            "power": power,
            "accuracy": accuracy,
            "pp": pp,
            "effect_chance": effect_chance,
            "priority": 0,
            "token_estimate": estimate_tokens(chunk_text),
            "text": chunk_text,
        })

    return chunks


def parse_types(filepath: Path) -> list[dict]:
    """Parse type_chart.md into per-type offensive + defensive chunks."""
    chunks = []

    for atk_type in ALL_TYPES:
        # Offensive summary
        super_eff = []
        not_very = []
        immune = []
        for def_type in ALL_TYPES:
            m = TYPE_CHART.get((atk_type, def_type), 1.0)
            if m > 1:
                super_eff.append(def_type.title())
            elif 0 < m < 1:
                not_very.append(def_type.title())
            elif m == 0:
                immune.append(def_type.title())

        phys_spec = "Physical" if atk_type in PHYSICAL_TYPES else "Special"

        off_text = (
            f"[Pokemon Crystal] Type Matchup: {atk_type.title()} — Offensive Summary\n\n"
            f"Super effective (2x) against: {', '.join(super_eff) if super_eff else 'none'}\n"
            f"Not very effective (0.5x) against: {', '.join(not_very) if not_very else 'none'}\n"
            f"No effect (0x) against: {', '.join(immune) if immune else 'none'}\n\n"
            f"In Gen 2, all {atk_type.title()} moves are {phys_spec}."
        )

        chunks.append({
            "id": f"type_{atk_type}_offensive_001",
            "doc_type": "type_interaction",
            "name": atk_type.title(),
            "category": "type_system",
            "subcategory": "offensive_matchups",
            "tags": [atk_type, "type_chart"],
            "related_entities": [f"type_{atk_type}_defensive_001"],
            "source": "pokecrystal",
            "source_file": "data/types/type_matchups.asm",
            "generation": 2,
            "game": "crystal",
            "attacking_type": atk_type,
            "token_estimate": estimate_tokens(off_text),
            "text": off_text,
        })

        # Defensive summary
        weak_to = []
        resists_list = []
        immune_from = []
        for other_type in ALL_TYPES:
            m = TYPE_CHART.get((other_type, atk_type), 1.0)
            if m > 1:
                weak_to.append(other_type.title())
            elif 0 < m < 1:
                resists_list.append(other_type.title())
            elif m == 0:
                immune_from.append(other_type.title())

        def_text = (
            f"[Pokemon Crystal] Type Matchup: {atk_type.title()} — Defensive Summary\n\n"
            f"Weak to (2x from): {', '.join(weak_to) if weak_to else 'none'}\n"
            f"Resists (0.5x from): {', '.join(resists_list) if resists_list else 'none'}\n"
            f"Immune to (0x from): {', '.join(immune_from) if immune_from else 'none'}"
        )

        chunks.append({
            "id": f"type_{atk_type}_defensive_001",
            "doc_type": "type_interaction",
            "name": atk_type.title(),
            "category": "type_system",
            "subcategory": "defensive_matchups",
            "tags": [atk_type, "type_chart"],
            "related_entities": [f"type_{atk_type}_offensive_001"],
            "source": "pokecrystal",
            "source_file": "data/types/type_matchups.asm",
            "generation": 2,
            "game": "crystal",
            "defending_type": atk_type,
            "token_estimate": estimate_tokens(def_text),
            "text": def_text,
        })

    return chunks


def parse_items(filepath: Path) -> list[dict]:
    """Parse all_items.md into item chunks."""
    chunks = []
    text = filepath.read_text(encoding="utf-8")

    # Parse table rows: | NAME | PRICE | EFFECT |
    current_section = "other"
    for line in text.split("\n"):
        section_match = re.match(r"^## (.+)", line)
        if section_match:
            current_section = slugify(section_match.group(1))
            continue

        row_match = re.match(r"\|\s*(.+?)\s*\|\s*(.+?)\s*\|\s*(.+?)\s*\|", line)
        if not row_match:
            continue
        item_name = row_match.group(1).strip()
        price_str = row_match.group(2).strip()
        effect = row_match.group(3).strip()

        # Skip header rows
        if item_name in ("Item", "---", "TM/HM", "Move"):
            continue
        if item_name.startswith("-"):
            continue

        slug = slugify(item_name)
        price = int(price_str) if price_str.replace(",", "").isdigit() else 0

        # Determine pocket/subcategory
        if current_section in ("poke_balls",):
            subcategory = "pokeballs"
        elif current_section in ("held_items",):
            subcategory = "held_items"
        elif current_section in ("berries",):
            subcategory = "berries"
        elif current_section in ("key_items",):
            subcategory = "key_items"
        elif current_section in ("tms_hms",):
            subcategory = "tm_hm_items"
        elif current_section in ("medicine", "status_healing"):
            subcategory = "medicine"
        elif current_section in ("vitamins",):
            subcategory = "vitamins"
        elif current_section in ("evolution_stones",):
            subcategory = "evolution_stones"
        elif current_section in ("battle_items",):
            subcategory = "battle_items"
        elif current_section in ("mail",):
            subcategory = "mail"
        else:
            subcategory = "other"

        tags = [subcategory]

        chunk_text = (
            f"[Pokemon Crystal] Item: {item_name} — {subcategory.replace('_', ' ').title()}\n\n"
            f"Price: {price if price else 'N/A'}. "
            f"Effect: {effect if effect != '-' else 'No listed effect.'}"
        )

        chunks.append({
            "id": f"item_{slug}_001",
            "doc_type": "item",
            "name": item_name,
            "category": "items_data",
            "subcategory": subcategory,
            "tags": tags,
            "related_entities": [],
            "source": "pokecrystal",
            "source_file": "data/items/names.asm",
            "generation": 2,
            "game": "crystal",
            "item_pocket": subcategory,
            "item_effect": effect if effect != "-" else "",
            "item_price": price,
            "holdable": subcategory in ("held_items", "berries"),
            "token_estimate": estimate_tokens(chunk_text),
            "text": chunk_text,
        })

    return chunks


def parse_encounters(filepath: Path) -> list[dict]:
    """Parse wild_encounters.md into per-location encounter chunks."""
    chunks = []
    text = filepath.read_text(encoding="utf-8")

    # Split by ### Location headers
    location_sections = re.split(r"^### (.+)$", text, flags=re.MULTILINE)

    # Determine overall encounter section (grass, water, fishing)
    current_method = "grass"

    i = 1
    while i < len(location_sections) - 1:
        location_name = location_sections[i].strip()
        body = location_sections[i + 1]
        i += 2

        # Check if the text before this was a section header changing method
        if "Water Encounters" in location_name or "Water" in location_name:
            current_method = "water"
            continue
        if "Fishing" in location_name:
            current_method = "fishing"
            continue

        slug = slugify(location_name)

        # Parse time-of-day encounters
        encounters_by_time = {}
        current_time = "any"
        species_seen = set()

        for line in body.strip().split("\n"):
            line = line.strip()
            time_match = re.match(r"\*\*(\w+):\*\*", line)
            if time_match:
                current_time = time_match.group(1).lower()
                continue
            mon_match = re.match(r"[-*]\s*(\w[\w\s.'♀♂-]*?)\s+Lv(\d+)", line)
            if mon_match:
                species = mon_match.group(1).strip()
                level = mon_match.group(2)
                if current_time not in encounters_by_time:
                    encounters_by_time[current_time] = []
                encounters_by_time[current_time].append(f"{species} Lv{level}")
                species_seen.add(slugify(species))

        if not encounters_by_time:
            continue

        # Build text
        enc_text_parts = [f"[Pokemon Crystal] Wild Encounters: {location_name} — {current_method.title()}\n"]
        for time_key, mons in encounters_by_time.items():
            # Deduplicate while preserving order
            seen = set()
            unique_mons = []
            for m in mons:
                if m not in seen:
                    seen.add(m)
                    unique_mons.append(m)
            enc_text_parts.append(f"\n{time_key.title()}: {', '.join(unique_mons)}")

        chunk_text = "\n".join(enc_text_parts)

        related = [f"species_{s}_001" for s in list(species_seen)[:5]]

        chunks.append({
            "id": f"wild_{slug}_{current_method}_001",
            "doc_type": "wild_encounter",
            "name": location_name,
            "category": "overworld",
            "subcategory": "routes",
            "tags": ["johto", current_method],
            "related_entities": related,
            "source": "pokecrystal",
            "source_file": f"data/wild/johto_{current_method}.asm",
            "generation": 2,
            "game": "crystal",
            "location_id": f"map_{slug}_001",
            "encounter_method": current_method,
            "species_list": list(species_seen),
            "token_estimate": estimate_tokens(chunk_text),
            "text": chunk_text,
        })

    return chunks


def parse_trainers(filepath: Path) -> list[dict]:
    """Parse all_trainers.md into trainer chunks."""
    chunks = []
    text = filepath.read_text(encoding="utf-8")

    # Split by ## TrainerClass headers
    class_sections = re.split(r"^## (\w[\w\s]+)$", text, flags=re.MULTILINE)

    i = 1
    trainer_count = {}  # Track duplicate names
    while i < len(class_sections) - 1:
        trainer_class = class_sections[i].strip()
        body = class_sections[i + 1]
        i += 2

        # Split by ### individual trainer entries
        trainer_entries = re.split(r"^### (.+)$", body, flags=re.MULTILINE)

        j = 1
        while j < len(trainer_entries) - 1:
            trainer_name = trainer_entries[j].strip()
            team_body = trainer_entries[j + 1]
            j += 2

            # Parse team members
            team = []
            for line in team_body.strip().split("\n"):
                mon_match = re.match(r"[-*]\s*Lv(\d+)\s+(\w[\w\s.'♀♂-]*?)(?::\s*(.+))?$", line)
                if mon_match:
                    level = int(mon_match.group(1))
                    species = mon_match.group(2).strip()
                    moves_str = mon_match.group(3)
                    moves = [m.strip() for m in moves_str.split(",")] if moves_str else []
                    team.append({"level": level, "species": species, "moves": moves})

            if not team:
                continue

            # Generate unique slug
            base_slug = slugify(trainer_name)
            trainer_count[base_slug] = trainer_count.get(base_slug, 0) + 1
            count = trainer_count[base_slug]
            slug = f"{base_slug}_{count:03d}" if count > 1 else base_slug

            # Determine tags
            tags = []
            class_lower = trainer_class.lower()
            if class_lower in ("falkner", "whitney", "bugsy", "morty", "pryce",
                               "jasmine", "chuck", "clair"):
                tags.append("gym_leader")
                tags.append("johto")
            elif class_lower in ("will", "koga", "bruno", "karen"):
                tags.append("elite_four")
            elif class_lower == "lance" or (class_lower == "champion" and "lance" in trainer_name.lower()):
                tags.append("champion")
            elif "rival" in class_lower.lower():
                tags.append("rival")

            team_display = ", ".join(f"{m['species']} Lv{m['level']}" for m in team)
            team_detail = ""
            for m in team:
                moves_str = ", ".join(m["moves"]) if m["moves"] else "(no moves listed)"
                team_detail += f"\n- Lv{m['level']} {m['species']}: {moves_str}"

            chunk_text = (
                f"[Pokemon Crystal] Trainer Battle: {trainer_class} {trainer_name}\n"
                f"Team: {team_display}\n"
                f"{team_detail}"
            )

            team_species = [slugify(m["species"]) for m in team]
            related = [f"species_{s}_001" for s in team_species[:5]]

            chunks.append({
                "id": f"trainer_{slug}_001",
                "doc_type": "trainer",
                "name": trainer_name,
                "category": "trainer_data",
                "subcategory": "gym_leaders" if "gym_leader" in tags else "route_trainers",
                "tags": tags,
                "related_entities": related,
                "source": "pokecrystal",
                "source_file": "data/trainers/parties.asm",
                "generation": 2,
                "game": "crystal",
                "trainer_class": trainer_class,
                "team_size": len(team),
                "team_levels": [m["level"] for m in team],
                "token_estimate": estimate_tokens(chunk_text),
                "text": chunk_text,
            })

    return chunks


# ---------------------------------------------------------------------------
# Parsers — v2: prose/supplementary data sources
# ---------------------------------------------------------------------------

def parse_prose_sections(filepath: Path, doc_type: str, category: str,
                         subcategory: str, tags: list[str],
                         header_level: int) -> list[dict]:
    """Parse a prose markdown file into chunks by splitting on ## or ### headers.

    Each section (header + content until next same-level header) becomes one chunk.
    Handles nested headers by treating sub-headers as part of the parent chunk
    when header_level=2, or splitting on ### when header_level=3.
    """
    chunks = []
    text = filepath.read_text(encoding="utf-8")
    source_name = filepath.stem
    slug_base = slugify(source_name)

    # Build split pattern based on header level
    if header_level == 3:
        # Split on ### but keep content under ## as context
        pattern = r"^### (.+)$"
    else:
        # Split on ## (will capture ### subsections within each chunk)
        pattern = r"^## (.+)$"

    parts = re.split(pattern, text, flags=re.MULTILINE)
    # Result: [preamble, header1, body1, header2, body2, ...]

    # If there's meaningful preamble (file-level intro), include it
    preamble = parts[0].strip()
    preamble_lines = [l for l in preamble.split("\n") if l.strip() and not l.startswith("#") and not l.startswith(">") and l.strip() != "---"]

    seq = 0

    # If preamble is substantial (more than just title/source lines), chunk it
    if len(preamble_lines) > 3:
        seq += 1
        title_match = re.search(r"^# (.+)$", preamble, re.MULTILINE)
        title = title_match.group(1).strip() if title_match else source_name.replace("_", " ").title()
        preamble_text = (
            f"[Pokemon Crystal] {title} — Overview\n\n"
            + "\n".join(preamble_lines)
        )
        chunk_id = f"{doc_type}_{slug_base}_intro_{seq:03d}"
        chunks.append({
            "id": chunk_id,
            "doc_type": doc_type,
            "name": title,
            "category": category,
            "subcategory": subcategory,
            "tags": tags[:],
            "related_entities": [],
            "source": "derived",
            "source_file": str(filepath.relative_to(BASE_DIR.parent)),
            "generation": 2,
            "game": "crystal",
            "token_estimate": estimate_tokens(preamble_text),
            "text": preamble_text,
        })

    # Process each section
    i = 1
    while i < len(parts) - 1:
        header = parts[i].strip()
        body = parts[i + 1].strip()
        i += 2

        # Skip empty or trivial sections
        content_lines = [l for l in body.split("\n") if l.strip() and l.strip() != "---"]
        if len(content_lines) < 2:
            continue

        seq += 1
        header_slug = slugify(header)
        chunk_id = f"{doc_type}_{slug_base}_{header_slug}_{seq:03d}"

        # Truncate chunk_id if too long
        if len(chunk_id) > 80:
            chunk_id = chunk_id[:76] + f"_{seq:03d}"

        section_text = (
            f"[Pokemon Crystal] {header}\n\n"
            + body
        )

        # Trim overly long chunks (split at ~1500 tokens worth = ~6000 chars)
        if len(section_text) > 6000:
            # Keep first ~5500 chars + truncation note
            section_text = section_text[:5500] + "\n\n[...continued in next chunk]"

        chunks.append({
            "id": chunk_id,
            "doc_type": doc_type,
            "name": header,
            "category": category,
            "subcategory": subcategory,
            "tags": tags[:],
            "related_entities": [],
            "source": "derived",
            "source_file": str(filepath.relative_to(BASE_DIR.parent)),
            "generation": 2,
            "game": "crystal",
            "token_estimate": estimate_tokens(section_text),
            "text": section_text,
        })

    return chunks


def parse_qa_pairs(filepath: Path) -> list[dict]:
    """Parse qa_pairs.md into individual Q&A chunks for evaluation."""
    chunks = []
    text = filepath.read_text(encoding="utf-8")

    # Split on ### Q\d+
    qa_sections = re.split(r"^### (Q\d+)\s*$", text, flags=re.MULTILINE)

    i = 1
    while i < len(qa_sections) - 1:
        q_label = qa_sections[i].strip()  # e.g., "Q1"
        body = qa_sections[i + 1].strip()
        i += 2

        # Extract question
        q_match = re.search(r"\*\*Q:\*\*\s*(.+)", body)
        a_match = re.search(r"\*\*A:\*\*\s*(.+)", body)
        cat_match = re.search(r"\*\*Category:\*\*\s*(\w+)", body)
        diff_match = re.search(r"\*\*Difficulty:\*\*\s*(\w+)", body)

        if not q_match or not a_match:
            continue

        question = q_match.group(1).strip()
        answer = a_match.group(1).strip()
        qa_category = cat_match.group(1).strip().lower() if cat_match else "general"
        difficulty = diff_match.group(1).strip().lower() if diff_match else "beginner"

        q_num = int(re.search(r"\d+", q_label).group())
        chunk_id = f"qa_{slugify(qa_category)}_{q_num:03d}"

        chunk_text = (
            f"[Pokemon Crystal] Evaluation Q&A — {q_label}\n\n"
            f"Question: {question}\n"
            f"Answer: {answer}\n"
            f"Category: {qa_category}, Difficulty: {difficulty}"
        )

        tags = [qa_category, difficulty]

        chunks.append({
            "id": chunk_id,
            "doc_type": "game_corner",
            "name": f"QA {q_label}: {question[:50]}",
            "category": "evaluation",
            "subcategory": "qa_pairs",
            "tags": tags,
            "related_entities": [],
            "source": "derived",
            "source_file": "data/meta/qa_pairs.md",
            "generation": 2,
            "game": "crystal",
            "qa_question": question,
            "qa_answer": answer,
            "qa_category": qa_category,
            "qa_difficulty": difficulty,
            "token_estimate": estimate_tokens(chunk_text),
            "text": chunk_text,
        })

    return chunks


def parse_learnset_by_move(filepath: Path) -> list[dict]:
    """Parse learnset_by_move.md — reverse index: which Pokemon learn each move."""
    chunks = []
    text = filepath.read_text(encoding="utf-8")

    # Split on ## MoveName
    sections = re.split(r"^## (.+)$", text, flags=re.MULTILINE)

    i = 1
    seq = 0
    while i < len(sections) - 1:
        move_name = sections[i].strip()
        body = sections[i + 1].strip()
        i += 2

        if not body or body == "---":
            continue

        seq += 1
        slug = slugify(move_name)
        chunk_id = f"learnset_by_move_{slug}_{seq:03d}"

        # Parse learners
        level_up = []
        egg_moves = []
        tm_hm = []
        current_section = "level_up"

        for line in body.split("\n"):
            line = line.strip()
            if line.startswith("**Level-Up"):
                current_section = "level_up"
                continue
            elif line.startswith("**Egg Move"):
                current_section = "egg"
                continue
            elif line.startswith("**TM") or line.startswith("**HM"):
                current_section = "tm_hm"
                continue
            elif line.startswith("---"):
                continue

            mon_match = re.match(r"[-*]\s*(\w[\w\s.'♀♂-]*?)(?:\s*\((.+?)\))?$", line)
            if mon_match:
                mon = mon_match.group(1).strip()
                detail = mon_match.group(2).strip() if mon_match.group(2) else ""
                entry = f"{mon} ({detail})" if detail else mon
                if current_section == "level_up":
                    level_up.append(entry)
                elif current_section == "egg":
                    egg_moves.append(mon)
                elif current_section == "tm_hm":
                    tm_hm.append(mon)
            elif line and not line.startswith("#") and not line.startswith("*"):
                # Bare Pokemon name (some egg moves listed without bullets)
                if current_section == "egg":
                    egg_moves.append(line)
                elif current_section == "tm_hm":
                    tm_hm.append(line)

        parts = [f"[Pokemon Crystal] Move Learners: {move_name}\n"]
        if level_up:
            parts.append(f"\nLevel-Up: {', '.join(level_up)}")
        if egg_moves:
            parts.append(f"\nEgg Move: {', '.join(egg_moves)}")
        if tm_hm:
            parts.append(f"\nTM/HM: {', '.join(tm_hm)}")

        chunk_text = "\n".join(parts)

        all_pokemon = level_up + egg_moves + tm_hm
        related = [f"move_{slug}_001"]

        chunks.append({
            "id": chunk_id,
            "doc_type": "learnset",
            "name": f"Learners of {move_name}",
            "category": "pokemon_data",
            "subcategory": "move_learners",
            "tags": ["list"],
            "related_entities": related,
            "source": "pokecrystal",
            "source_file": "data/pokemon/evos_attacks.asm",
            "generation": 2,
            "game": "crystal",
            "move_name": move_name,
            "learner_count": len(all_pokemon),
            "token_estimate": estimate_tokens(chunk_text),
            "text": chunk_text,
        })

    return chunks


def parse_evolution_chains(filepath: Path) -> list[dict]:
    """Parse evolution_chains.md into per-family evolution chunks."""
    chunks = []
    text = filepath.read_text(encoding="utf-8")

    # Split on ## FAMILY_NAME Family
    sections = re.split(r"^## (.+)$", text, flags=re.MULTILINE)

    i = 1
    seq = 0
    while i < len(sections) - 1:
        family_header = sections[i].strip()
        body = sections[i + 1].strip()
        i += 2

        if not body or body == "---":
            continue

        seq += 1
        # Extract base Pokemon name from header (e.g., "BULBASAUR Family")
        family_match = re.match(r"(\w[\w\s'-]*?)\s*Family", family_header, re.IGNORECASE)
        if family_match:
            family_name = family_match.group(1).strip()
        else:
            family_name = family_header.replace("Family", "").strip()

        slug = slugify(family_name)
        chunk_id = f"evo_chain_{slug}_{seq:03d}"

        # Extract all Pokemon in the chain
        members = re.findall(r"\*\*(\w[\w\s.'♀♂-]*?)\*\*\s*\(#(\d+)\)", body)
        member_names = [m[0] for m in members]
        member_nums = [int(m[1]) for m in members]

        # Extract evolution methods
        methods = re.findall(r"\[(.+?)\]", body)

        chunk_text = (
            f"[Pokemon Crystal] Evolution Chain: {family_name} Family\n\n"
            + body.replace("---", "").strip()
        )

        related = [f"species_{slugify(m)}_001" for m in member_names[:5]]

        chunks.append({
            "id": chunk_id,
            "doc_type": "evolution",
            "name": f"{family_name} Evolution Chain",
            "category": "pokemon_data",
            "subcategory": "evolution_chains",
            "tags": ["list"],
            "related_entities": related,
            "source": "pokecrystal",
            "source_file": "data/pokemon/evos_attacks.asm",
            "generation": 2,
            "game": "crystal",
            "family_base": family_name,
            "family_members": member_names,
            "evo_methods": methods,
            "token_estimate": estimate_tokens(chunk_text),
            "text": chunk_text,
        })

    return chunks


# ---------------------------------------------------------------------------
# Validation
# ---------------------------------------------------------------------------

def validate_chunk(chunk: dict) -> list[str]:
    """Validate a single chunk against schema. Returns list of errors."""
    errors = []
    required_fields = [
        "id", "doc_type", "name", "category", "subcategory",
        "tags", "related_entities", "source", "generation", "game", "text",
    ]
    for field in required_fields:
        if field not in chunk:
            errors.append(f"Missing required field: {field}")

    # ID format
    if "id" in chunk:
        if not re.match(r"^[a-z][a-z0-9_]+_\d{3}$", chunk["id"]):
            errors.append(f"ID format invalid: {chunk['id']}")

    # Text not empty
    if "text" in chunk and len(chunk["text"].strip()) < 20:
        errors.append(f"Text too short ({len(chunk['text'])} chars): {chunk.get('id', '?')}")

    # Token estimate reasonable
    if "token_estimate" in chunk:
        te = chunk["token_estimate"]
        if te < 10:
            errors.append(f"Token estimate suspiciously low ({te}): {chunk.get('id', '?')}")
        if te > 2000:
            errors.append(f"Token estimate too high ({te}): {chunk.get('id', '?')}")

    return errors


def validate_jsonl(filepath: Path) -> tuple[int, int, list[str]]:
    """Validate all chunks in a JSONL file. Returns (total, valid, errors)."""
    total = 0
    valid = 0
    all_errors = []

    with open(filepath) as f:
        for line_num, line in enumerate(f, 1):
            total += 1
            try:
                chunk = json.loads(line)
            except json.JSONDecodeError as e:
                all_errors.append(f"Line {line_num}: Invalid JSON — {e}")
                continue

            errors = validate_chunk(chunk)
            if errors:
                for err in errors:
                    all_errors.append(f"Line {line_num} ({chunk.get('id', '?')}): {err}")
            else:
                valid += 1

    return total, valid, all_errors


# ---------------------------------------------------------------------------
# Utilities
# ---------------------------------------------------------------------------

def _extract_int(text: str, pattern: str) -> int:
    m = re.search(pattern, text)
    return int(m.group(1)) if m else 0


def write_chunks(chunks: list[dict], filepath: Path):
    """Write chunks to JSONL file."""
    filepath.parent.mkdir(parents=True, exist_ok=True)
    with open(filepath, "w", encoding="utf-8") as f:
        for chunk in chunks:
            f.write(json.dumps(chunk, ensure_ascii=False) + "\n")
    print(f"  Wrote {len(chunks)} chunks to {filepath}")


def print_stats(chunks: list[dict]):
    """Print statistics about a set of chunks."""
    by_type = {}
    total_tokens = 0
    for c in chunks:
        dt = c["doc_type"]
        by_type[dt] = by_type.get(dt, 0) + 1
        total_tokens += c.get("token_estimate", 0)

    print(f"\n{'='*50}")
    print(f"CHUNK STATISTICS")
    print(f"{'='*50}")
    print(f"{'Doc Type':<25} {'Count':>8}")
    print(f"{'-'*25} {'-'*8}")
    for dt in sorted(by_type.keys()):
        print(f"{dt:<25} {by_type[dt]:>8}")
    print(f"{'-'*25} {'-'*8}")
    print(f"{'TOTAL':<25} {len(chunks):>8}")
    print(f"{'Est. tokens':<25} {total_tokens:>8}")
    print(f"{'Avg tokens/chunk':<25} {total_tokens // max(len(chunks), 1):>8}")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description="Pokemon Crystal Vector DB Ingestion Pipeline")
    parser.add_argument("--source", choices=list(RAW_SOURCES.keys()), help="Process only this source")
    parser.add_argument("--validate", action="store_true", help="Validate existing JSONL files")
    parser.add_argument("--stats", action="store_true", help="Print statistics only")
    parser.add_argument("--output-dir", type=str, default=str(OUTPUT_DIR), help="Output directory")
    args = parser.parse_args()

    output_dir = Path(args.output_dir)

    if args.validate:
        # Validate existing JSONL files
        jsonl_files = list(output_dir.glob("*.jsonl"))
        if not jsonl_files:
            # Also check sample chunks
            sample = SCHEMA_DIR / "sample_chunks.jsonl"
            if sample.exists():
                jsonl_files = [sample]

        for jf in jsonl_files:
            print(f"\nValidating {jf.name}...")
            total, valid, errors = validate_jsonl(jf)
            print(f"  {valid}/{total} chunks valid")
            if errors:
                for err in errors[:20]:
                    print(f"  ERROR: {err}")
                if len(errors) > 20:
                    print(f"  ... and {len(errors) - 20} more errors")
        return

    # Process raw sources (v1 tabular parsers)
    parsers = {
        "species": parse_species,
        "moves": parse_moves,
        "types": parse_types,
        "items": parse_items,
        "encounters": parse_encounters,
        "trainers": parse_trainers,
    }

    all_chunks = []
    sources_to_process = [args.source] if args.source else list(RAW_SOURCES.keys())

    for source_name in sources_to_process:
        filepath = RAW_SOURCES[source_name]
        if not filepath.exists():
            print(f"WARNING: {filepath} not found, skipping")
            continue

        print(f"\nProcessing {source_name} from {filepath.name}...")
        parse_fn = parsers[source_name]
        chunks = parse_fn(filepath)
        print(f"  Parsed {len(chunks)} chunks")

        # Validate
        error_count = 0
        for chunk in chunks:
            errors = validate_chunk(chunk)
            if errors:
                error_count += 1
                if error_count <= 3:
                    for err in errors:
                        print(f"  WARN: {err}")

        if error_count:
            print(f"  {error_count} chunks have validation warnings")

        # Write output
        out_file = output_dir / f"{source_name}_chunks.jsonl"
        write_chunks(chunks, out_file)
        all_chunks.extend(chunks)

    # Process prose/supplementary sources (v2 parsers)
    if not args.source:
        prose_chunks_all = []
        for filepath, doc_type, category, subcat, tags, hlevel in PROSE_SOURCES:
            if not filepath.exists():
                print(f"WARNING: {filepath} not found, skipping")
                continue
            print(f"\nProcessing prose: {filepath.name}...")
            chunks = parse_prose_sections(filepath, doc_type, category, subcat, tags, hlevel)
            print(f"  Parsed {len(chunks)} chunks")

            error_count = 0
            for chunk in chunks:
                errs = validate_chunk(chunk)
                if errs:
                    error_count += 1
                    if error_count <= 3:
                        for err in errs:
                            print(f"  WARN: {err}")
            if error_count:
                print(f"  {error_count} chunks have validation warnings")

            prose_chunks_all.extend(chunks)

        # Write all prose chunks to one file
        if prose_chunks_all:
            write_chunks(prose_chunks_all, output_dir / "prose_chunks.jsonl")
        all_chunks.extend(prose_chunks_all)

        # Process QA pairs
        if QA_FILE.exists():
            print(f"\nProcessing QA pairs: {QA_FILE.name}...")
            qa_chunks = parse_qa_pairs(QA_FILE)
            print(f"  Parsed {len(qa_chunks)} chunks")
            error_count = 0
            for chunk in qa_chunks:
                errs = validate_chunk(chunk)
                if errs:
                    error_count += 1
                    if error_count <= 3:
                        for err in errs:
                            print(f"  WARN: {err}")
            if error_count:
                print(f"  {error_count} chunks have validation warnings")
            if qa_chunks:
                write_chunks(qa_chunks, output_dir / "qa_chunks.jsonl")
            all_chunks.extend(qa_chunks)
        else:
            print(f"WARNING: {QA_FILE} not found, skipping")

        # Process learnset-by-move reverse index
        if LEARNSET_BY_MOVE_FILE.exists():
            print(f"\nProcessing learnset-by-move: {LEARNSET_BY_MOVE_FILE.name}...")
            lbm_chunks = parse_learnset_by_move(LEARNSET_BY_MOVE_FILE)
            print(f"  Parsed {len(lbm_chunks)} chunks")
            error_count = 0
            for chunk in lbm_chunks:
                errs = validate_chunk(chunk)
                if errs:
                    error_count += 1
                    if error_count <= 3:
                        for err in errs:
                            print(f"  WARN: {err}")
            if error_count:
                print(f"  {error_count} chunks have validation warnings")
            if lbm_chunks:
                write_chunks(lbm_chunks, output_dir / "learnset_by_move_chunks.jsonl")
            all_chunks.extend(lbm_chunks)
        else:
            print(f"WARNING: {LEARNSET_BY_MOVE_FILE} not found, skipping")

        # Process evolution chains
        if EVOLUTION_CHAINS_FILE.exists():
            print(f"\nProcessing evolution chains: {EVOLUTION_CHAINS_FILE.name}...")
            evo_chunks = parse_evolution_chains(EVOLUTION_CHAINS_FILE)
            print(f"  Parsed {len(evo_chunks)} chunks")
            error_count = 0
            for chunk in evo_chunks:
                errs = validate_chunk(chunk)
                if errs:
                    error_count += 1
                    if error_count <= 3:
                        for err in errs:
                            print(f"  WARN: {err}")
            if error_count:
                print(f"  {error_count} chunks have validation warnings")
            if evo_chunks:
                write_chunks(evo_chunks, output_dir / "evolution_chains_chunks.jsonl")
            all_chunks.extend(evo_chunks)
        else:
            print(f"WARNING: {EVOLUTION_CHAINS_FILE} not found, skipping")

    if args.stats or not args.source:
        print_stats(all_chunks)

    # Write combined file
    if not args.source and all_chunks:
        combined = output_dir / "all_chunks.jsonl"
        write_chunks(all_chunks, combined)

    print(f"\nDone. Output in {output_dir}/")


if __name__ == "__main__":
    main()
