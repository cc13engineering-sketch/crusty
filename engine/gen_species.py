#!/usr/bin/env python3
"""Parse all 251 pokecrystal ASM base stat files and generate species_data.rs"""

import re
import os

BASE = "/Users/colinlaptop/DIY/AI/crusty/engine/crates/engine-core/src/pokemon/pokecrystal-master"
BASE_STATS_ASM = os.path.join(BASE, "data/pokemon/base_stats.asm")
BASE_STATS_DIR = os.path.join(BASE, "data/pokemon/base_stats")
NAMES_ASM = os.path.join(BASE, "data/pokemon/names.asm")

TYPE_MAP = {
    "NORMAL": "Normal", "FIRE": "Fire", "WATER": "Water",
    "ELECTRIC": "Electric", "GRASS": "Grass", "ICE": "Ice",
    "FIGHTING": "Fighting", "POISON": "Poison", "GROUND": "Ground",
    "FLYING": "Flying", "PSYCHIC_TYPE": "Psychic", "BUG": "Bug",
    "ROCK": "Rock", "GHOST": "Ghost", "DRAGON": "Dragon",
    "DARK": "Dark", "STEEL": "Steel",
}

GROWTH_MAP = {
    "GROWTH_MEDIUM_FAST": "MediumFast",
    "GROWTH_MEDIUM_SLOW": "MediumSlow",
    "GROWTH_FAST": "Fast",
    "GROWTH_SLOW": "Slow",
}

def parse_names():
    names = []
    with open(NAMES_ASM) as f:
        for line in f:
            m = re.match(r'\s*dname\s+"([^"]+)"', line)
            if m:
                names.append(m.group(1))
            if len(names) == 251:
                break
    return names

def parse_order():
    """Return ordered list of (index_1based, filename_stem) from base_stats.asm"""
    order = []
    with open(BASE_STATS_ASM) as f:
        for line in f:
            m = re.match(r'\s*INCLUDE\s+"data/pokemon/base_stats/([^"]+)\.asm"', line)
            if m:
                order.append(m.group(1))
    return order

def parse_species_file(filepath):
    with open(filepath) as f:
        content = f.read()

    # Stats line: db hp, atk, def, spd, sat, sdf
    stats_m = re.search(r'db\s+(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+)', content)
    if not stats_m:
        raise ValueError(f"No stats in {filepath}")
    hp, atk, def_, spd, sat, sdf = [int(x) for x in stats_m.groups()]

    # Types: db TYPE1, TYPE2
    types_m = re.search(r'db\s+(\w+),\s*(\w+)\s*;\s*type', content)
    if not types_m:
        raise ValueError(f"No types in {filepath}")
    t1_raw, t2_raw = types_m.group(1), types_m.group(2)
    t1 = TYPE_MAP.get(t1_raw)
    t2 = TYPE_MAP.get(t2_raw)
    if t1 is None:
        raise ValueError(f"Unknown type {t1_raw} in {filepath}")
    if t2 is None:
        raise ValueError(f"Unknown type {t2_raw} in {filepath}")

    # Catch rate
    catch_m = re.search(r'db\s+(\d+)\s*;\s*catch rate', content)
    if not catch_m:
        raise ValueError(f"No catch rate in {filepath}")
    catch_rate = int(catch_m.group(1))

    # Base exp
    exp_m = re.search(r'db\s+(\d+)\s*;\s*base exp', content)
    if not exp_m:
        raise ValueError(f"No base exp in {filepath}")
    base_exp = int(exp_m.group(1))

    # Growth rate
    growth_m = re.search(r'db\s+(GROWTH_\w+)\s*;\s*growth rate', content)
    if not growth_m:
        raise ValueError(f"No growth rate in {filepath}")
    growth_raw = growth_m.group(1)
    growth = GROWTH_MAP.get(growth_raw)
    if growth is None:
        raise ValueError(f"Unknown growth rate {growth_raw} in {filepath}")

    return {
        "hp": hp, "atk": atk, "def": def_, "spd": spd,
        "sat": sat, "sdf": sdf,
        "type1": t1, "type2": t2,
        "catch_rate": catch_rate,
        "base_exp": base_exp,
        "growth": growth,
    }

def rust_ident(name):
    """Convert species display name to valid Rust const identifier"""
    # Replace special chars
    s = name.replace("♀", "_F").replace("♂", "_M")
    s = s.replace("'", "").replace(".", "").replace("-", "_")
    s = s.replace(" ", "_").upper()
    return s

def main():
    names_list = parse_names()
    order = parse_order()

    assert len(order) == 251, f"Expected 251 species in order, got {len(order)}"
    assert len(names_list) == 251, f"Expected 251 names, got {len(names_list)}"

    species_data = []
    for idx, stem in enumerate(order):
        filepath = os.path.join(BASE_STATS_DIR, stem + ".asm")
        data = parse_species_file(filepath)
        data["id"] = idx + 1
        data["name"] = names_list[idx]
        data["ident"] = rust_ident(names_list[idx])
        species_data.append(data)

    # Generate Rust
    lines = []
    lines.append("// AI-INSTRUCTIONS: pokemonv2/species_data.rs — All 251 Pokemon species data.")
    lines.append("// Sprint 6: Generated from pokecrystal-master ASM files.")
    lines.append("// Source: data/pokemon/base_stats/*.asm + data/pokemon/names.asm")
    lines.append("")
    lines.append("use super::data::{SpeciesData, SpeciesId, MoveId, PokemonType, GrowthRate};")
    lines.append("use super::data::MOVE_TACKLE;")
    lines.append("")
    lines.append("pub const NUM_SPECIES: usize = 251;")
    lines.append("")

    # SpeciesId constants
    lines.append("// --- SpeciesId Constants (1-251) ---")
    for d in species_data:
        lines.append(f"pub const {d['ident']}: SpeciesId = {d['id']};")
    lines.append("")

    # Learnset stubs
    lines.append("// --- Learnset stubs (full learnsets in Sprint 7+) ---")
    for d in species_data:
        lines.append(f"static {d['ident']}_LEARNSET: &[(u8, MoveId)] = &[(1, MOVE_TACKLE)];")
    lines.append("")

    # SPECIES_TABLE
    lines.append("// --- Master Species Table ---")
    lines.append(f"static SPECIES_TABLE: [SpeciesData; NUM_SPECIES] = [")
    for d in species_data:
        ident = d['ident']
        lines.append(f"    SpeciesData {{")
        lines.append(f"        id: {ident},")
        lines.append(f'        name: "{d["name"]}",')
        lines.append(f"        type1: PokemonType::{d['type1']},")
        lines.append(f"        type2: PokemonType::{d['type2']},")
        lines.append(f"        base_hp: {d['hp']},")
        lines.append(f"        base_attack: {d['atk']},")
        lines.append(f"        base_defense: {d['def']},")
        lines.append(f"        base_speed: {d['spd']},")
        lines.append(f"        base_sp_attack: {d['sat']},")
        lines.append(f"        base_sp_defense: {d['sdf']},")
        lines.append(f"        catch_rate: {d['catch_rate']},")
        lines.append(f"        base_exp: {d['base_exp']},")
        lines.append(f"        growth_rate: GrowthRate::{d['growth']},")
        lines.append(f"        learnset: {ident}_LEARNSET,")
        lines.append(f"    }},")
    lines.append("];")
    lines.append("")

    # species_data fn
    lines.append("/// Return species data for the given SpeciesId (1-based).")
    lines.append("/// Returns Bulbasaur data for unknown/out-of-range ids.")
    lines.append("pub fn species_data(id: SpeciesId) -> &'static SpeciesData {")
    lines.append("    let idx = id as usize;")
    lines.append("    if idx >= 1 && idx <= NUM_SPECIES {")
    lines.append("        &SPECIES_TABLE[idx - 1]")
    lines.append("    } else {")
    lines.append("        &SPECIES_TABLE[0]")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    output = "\n".join(lines)
    out_path = "/Users/colinlaptop/DIY/AI/crusty/engine/crates/engine-core/src/pokemonv2/species_data.rs"
    with open(out_path, "w") as f:
        f.write(output)
    print(f"Written to {out_path}")
    print(f"Total species: {len(species_data)}")
    # Print some spot checks
    for check_id in [1, 25, 129, 150, 151, 152, 155, 158, 242, 249, 250, 251]:
        d = species_data[check_id-1]
        print(f"  #{check_id:3d} {d['name']:12s} {d['ident']:20s} hp={d['hp']:3d} types={d['type1']}/{d['type2']}")

if __name__ == "__main__":
    main()
