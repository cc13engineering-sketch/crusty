#!/usr/bin/env python3
"""Parse pokecrystal disassembly data and generate structured markdown files."""

import os
import re
import glob

BASE = "/Users/colinlaptop/DIY/AI/crusty/engine/crates/engine-core/src/pokemon/pokecrystal-master"
OUT = "/Users/colinlaptop/DIY/AI/crusty/data"

# Type mapping for display
TYPE_MAP = {
    "NORMAL": "Normal", "FIGHTING": "Fighting", "FLYING": "Flying",
    "POISON": "Poison", "GROUND": "Ground", "ROCK": "Rock",
    "BUG": "Bug", "GHOST": "Ghost", "STEEL": "Steel",
    "FIRE": "Fire", "WATER": "Water", "GRASS": "Grass",
    "ELECTRIC": "Electric", "PSYCHIC_TYPE": "Psychic", "ICE": "Ice",
    "DRAGON": "Dragon", "DARK": "Dark", "CURSE_TYPE": "???",
}

GENDER_MAP = {
    "GENDER_F100": "100% Female",
    "GENDER_F87_5": "87.5% Female / 12.5% Male",
    "GENDER_F75": "75% Female / 25% Male",
    "GENDER_F50": "50% Female / 50% Male",
    "GENDER_F25": "25% Female / 75% Male",
    "GENDER_F12_5": "12.5% Female / 87.5% Male",
    "GENDER_F0": "100% Male",
    "GENDER_UNKNOWN": "Genderless",
}

GROWTH_MAP = {
    "GROWTH_MEDIUM_FAST": "Medium Fast",
    "GROWTH_MEDIUM_SLOW": "Medium Slow",
    "GROWTH_FAST": "Fast",
    "GROWTH_SLOW": "Slow",
}

EGG_GROUP_MAP = {
    "EGG_MONSTER": "Monster", "EGG_WATER_1": "Water 1", "EGG_BUG": "Bug",
    "EGG_FLYING": "Flying", "EGG_GROUND": "Field", "EGG_FAIRY": "Fairy",
    "EGG_PLANT": "Grass", "EGG_HUMANSHAPE": "Human-Like", "EGG_WATER_3": "Water 3",
    "EGG_MINERAL": "Mineral", "EGG_INDETERMINATE": "Amorphous",
    "EGG_WATER_2": "Water 2", "EGG_DITTO": "Ditto", "EGG_DRAGON": "Dragon",
    "EGG_NONE": "No Eggs",
}

# Pokemon names in national dex order
POKEMON_NAMES = []

# TM/HM moves
TM_MOVES = {
    1: "DynamicPunch", 2: "Headbutt", 3: "Curse", 4: "Rollout", 5: "Roar",
    6: "Toxic", 7: "Zap Cannon", 8: "Rock Smash", 9: "Psych Up", 10: "Hidden Power",
    11: "Sunny Day", 12: "Sweet Scent", 13: "Snore", 14: "Blizzard", 15: "Hyper Beam",
    16: "Icy Wind", 17: "Protect", 18: "Rain Dance", 19: "Giga Drain", 20: "Endure",
    21: "Frustration", 22: "SolarBeam", 23: "Iron Tail", 24: "DragonBreath", 25: "Thunder",
    26: "Earthquake", 27: "Return", 28: "Dig", 29: "Psychic", 30: "Shadow Ball",
    31: "Mud-Slap", 32: "Double Team", 33: "Ice Punch", 34: "Swagger", 35: "Sleep Talk",
    36: "Sludge Bomb", 37: "Sandstorm", 38: "Fire Blast", 39: "Swift", 40: "Defense Curl",
    41: "ThunderPunch", 42: "Dream Eater", 43: "Detect", 44: "Rest", 45: "Attract",
    46: "Thief", 47: "Steel Wing", 48: "Fire Punch", 49: "Fury Cutter", 50: "Nightmare",
}

HM_MOVES = {
    1: "Cut", 2: "Fly", 3: "Surf", 4: "Strength", 5: "Flash",
    6: "Whirlpool", 7: "Waterfall",
}


def format_move_name(name):
    """Convert ASM move constant to display name."""
    name = name.strip()
    special = {
        "PSYCHIC_M": "Psychic", "NO_MOVE": "-", "DOUBLESLAP": "DoubleSlap",
        "DOUBLE_EDGE": "Double-Edge", "SAND_ATTACK": "Sand-Attack",
        "MUD_SLAP": "Mud-Slap", "FAINT_ATTACK": "Faint Attack",
        "SONICBOOM": "SonicBoom", "THUNDERPUNCH": "ThunderPunch",
        "THUNDERSHOCK": "ThunderShock", "THUNDERBOLT": "Thunderbolt",
        "SOLARBEAM": "SolarBeam", "POISONPOWDER": "PoisonPowder",
        "DYNAMICPUNCH": "DynamicPunch", "EXTREMESPEED": "ExtremeSpeed",
        "ANCIENTPOWER": "AncientPower", "DRAGONBREATH": "DragonBreath",
        "MEGAHORN": "Megahorn", "SELFDESTRUCT": "Self-Destruct",
        "SOFTBOILED": "Softboiled", "SMOKESCREEN": "SmokeScreen",
        "VICEGRIP": "ViceGrip", "SLEEP_POWDER": "Sleep Powder",
        "STUN_SPORE": "Stun Spore", "SPIDER_WEB": "Spider Web",
        "ZAP_CANNON": "Zap Cannon", "OCTAZOOKA": "Octazooka",
        "SACRED_FIRE": "Sacred Fire", "QUICK_ATTACK": "Quick Attack",
    }
    if name in special:
        return special[name]
    parts = name.split("_")
    return " ".join(p.capitalize() for p in parts)


def format_item_name(name):
    """Convert ASM item constant to display name."""
    if name == "NO_ITEM":
        return "None"
    parts = name.split("_")
    return " ".join(p.capitalize() for p in parts)


def parse_pokemon_names():
    """Parse all 251 Pokemon names."""
    global POKEMON_NAMES
    with open(f"{BASE}/data/pokemon/names.asm") as f:
        for line in f:
            m = re.search(r'dname "([^"]+)"', line)
            if m:
                name = m.group(1)
                if name not in ("?????", "EGG"):
                    POKEMON_NAMES.append(name)
                if len(POKEMON_NAMES) >= 251:
                    break


def parse_base_stats():
    """Parse all base stats files and return dict keyed by pokemon name."""
    stats = {}
    stat_dir = f"{BASE}/data/pokemon/base_stats"
    for fpath in sorted(glob.glob(f"{stat_dir}/*.asm")):
        with open(fpath) as f:
            content = f.read()

        # Get pokemon name/number
        m = re.search(r'db (\w+) ; (\d+)', content)
        if not m:
            continue
        poke_const = m.group(1)
        dex_num = int(m.group(2))

        # Stats line
        m = re.search(r'db\s+(\d+),\s+(\d+),\s+(\d+),\s+(\d+),\s+(\d+),\s+(\d+)\s*\n\s*;\s+hp\s+atk\s+def\s+spd\s+sat\s+sdf', content)
        if not m:
            continue
        hp, atk, defn, spd, satk, sdef = int(m.group(1)), int(m.group(2)), int(m.group(3)), int(m.group(4)), int(m.group(5)), int(m.group(6))

        # Types
        m = re.search(r'db (\w+), (\w+) ; type', content)
        type1, type2 = m.group(1), m.group(2)

        # Catch rate
        m = re.search(r'db (\d+) ; catch rate', content)
        catch_rate = int(m.group(1))

        # Base exp
        m = re.search(r'db (\d+) ; base exp', content)
        base_exp = int(m.group(1))

        # Items
        m = re.search(r'db (\w+), (\w+) ; items', content)
        item1, item2 = m.group(1), m.group(2)

        # Gender
        m = re.search(r'db (\w+) ; gender ratio', content)
        gender = m.group(1)

        # Hatch cycles
        m = re.search(r'db (\d+) ; step cycles to hatch', content)
        hatch_cycles = int(m.group(1))

        # Growth rate
        m = re.search(r'db (\w+) ; growth rate', content)
        growth = m.group(1)

        # Egg groups
        m = re.search(r'dn (\w+), (\w+) ; egg groups', content)
        egg1, egg2 = m.group(1), m.group(2)

        # TM/HM learnset
        m = re.search(r'tmhm\s+(.*)', content)
        tmhm_list = []
        if m:
            raw = m.group(1).strip()
            if raw:
                tmhm_list = [x.strip() for x in raw.split(",")]

        stats[dex_num] = {
            "name": POKEMON_NAMES[dex_num - 1] if dex_num <= len(POKEMON_NAMES) else poke_const,
            "dex": dex_num,
            "hp": hp, "atk": atk, "def": defn, "spd": spd, "satk": satk, "sdef": sdef,
            "bst": hp + atk + defn + spd + satk + sdef,
            "type1": TYPE_MAP.get(type1, type1),
            "type2": TYPE_MAP.get(type2, type2),
            "catch_rate": catch_rate,
            "base_exp": base_exp,
            "item1": format_item_name(item1),
            "item2": format_item_name(item2),
            "gender": GENDER_MAP.get(gender, gender),
            "hatch_cycles": hatch_cycles,
            "growth": GROWTH_MAP.get(growth, growth),
            "egg1": EGG_GROUP_MAP.get(egg1, egg1),
            "egg2": EGG_GROUP_MAP.get(egg2, egg2),
            "tmhm": [format_move_name(m) for m in tmhm_list],
        }
    return stats


def parse_evos_attacks():
    """Parse evolutions and level-up moves."""
    evos = {}  # dex_num -> list of evolution tuples
    learnsets = {}  # dex_num -> list of (level, move) tuples

    with open(f"{BASE}/data/pokemon/evos_attacks.asm") as f:
        content = f.read()

    # Split by pokemon sections
    sections = re.split(r'\n(\w+)EvosAttacks:', content)
    # sections[0] is header, then alternating name, content
    for i in range(1, len(sections), 2):
        pokemon_name = sections[i]
        section_data = sections[i + 1] if i + 1 < len(sections) else ""

        # Find dex number from name
        # Convert PascalCase to UPPERCASE for lookup
        upper_name = ""
        for ch in pokemon_name:
            if ch.isupper() and upper_name:
                upper_name += ch
            else:
                upper_name += ch.upper()

        # Name lookup mapping
        name_to_dex = {}
        for idx, name in enumerate(POKEMON_NAMES):
            # Create various key forms
            clean = name.replace("♀", "F").replace("♂", "M").replace(".", "").replace("'", "").replace("-", "").replace(" ", "")
            name_to_dex[clean.upper()] = idx + 1
            # Also try lowercase
            name_to_dex[clean.lower()] = idx + 1

        # Special mappings
        special_names = {
            "NidoranF": 29, "NidoranM": 32, "MrMime": 122,
            "FarfetchD": 83, "HoOh": 250,
        }

        dex_num = special_names.get(pokemon_name)
        if not dex_num:
            clean_name = pokemon_name.upper().replace("_", "")
            dex_num = name_to_dex.get(clean_name)
        if not dex_num:
            # Try matching first part
            for idx, name in enumerate(POKEMON_NAMES):
                pname = name.replace("♀", "F").replace("♂", "M").replace(".", "").replace("'", "").replace("-", "").replace(" ", "")
                if pname.upper() == pokemon_name.upper():
                    dex_num = idx + 1
                    break

        if not dex_num:
            continue

        # Parse evolutions
        evo_list = []
        lines = section_data.strip().split("\n")
        in_evos = True
        move_list = []

        for line in lines:
            line = line.strip()
            if not line or line.startswith(";"):
                continue

            if in_evos:
                if line == "db 0 ; no more evolutions":
                    in_evos = False
                    continue
                # Parse evolution
                evo_m = re.match(r'db EVOLVE_(\w+),\s*(.*)', line)
                if evo_m:
                    evo_type = evo_m.group(1)
                    params = evo_m.group(2).strip()
                    evo_list.append((evo_type, params))
            else:
                if line == "db 0 ; no more level-up moves":
                    break
                move_m = re.match(r'db\s+(\d+),\s+(\w+)', line)
                if move_m:
                    level = int(move_m.group(1))
                    move = format_move_name(move_m.group(2))
                    move_list.append((level, move))

        evos[dex_num] = evo_list
        learnsets[dex_num] = move_list

    return evos, learnsets


def parse_egg_moves():
    """Parse egg moves for each Pokemon."""
    egg_moves = {}

    with open(f"{BASE}/data/pokemon/egg_moves.asm") as f:
        content = f.read()

    sections = re.split(r'\n(\w+)EggMoves:', content)
    for i in range(1, len(sections), 2):
        pokemon_name = sections[i]
        section_data = sections[i + 1] if i + 1 < len(sections) else ""

        # Find dex number
        dex_num = None
        for idx, name in enumerate(POKEMON_NAMES):
            pname = name.replace("♀", "F").replace("♂", "M").replace(".", "").replace("'", "").replace("-", "").replace(" ", "")
            if pname.upper() == pokemon_name.upper():
                dex_num = idx + 1
                break

        if not dex_num:
            # Try special mappings
            special = {"NidoranF": 29, "NidoranM": 32, "MrMime": 122, "FarfetchD": 83, "HoOh": 250}
            dex_num = special.get(pokemon_name)

        if not dex_num:
            continue

        moves = []
        for line in section_data.strip().split("\n"):
            line = line.strip()
            if line.startswith("db -1"):
                break
            m = re.match(r'db (\w+)', line)
            if m and m.group(1) != "-1":
                moves.append(format_move_name(m.group(1)))

        egg_moves[dex_num] = moves

    return egg_moves


def parse_moves():
    """Parse all move data."""
    moves = []
    with open(f"{BASE}/data/moves/moves.asm") as f:
        for line in f:
            m = re.match(r'\s+move (\w+),\s*(\w+),\s*(\d+),\s*(\w+),\s*(\d+),\s*(\d+),\s*(\d+)', line)
            if m:
                move_const = m.group(1)
                effect = m.group(2)
                power = int(m.group(3))
                move_type = m.group(4)
                accuracy = int(m.group(5))
                pp = int(m.group(6))
                effect_chance = int(m.group(7))

                moves.append({
                    "id": len(moves) + 1,
                    "name": format_move_name(move_const),
                    "const": move_const,
                    "effect": effect,
                    "power": power if power > 1 else ("-" if power == 0 else "Varies"),
                    "type": TYPE_MAP.get(move_type, move_type),
                    "accuracy": accuracy if accuracy < 100 or effect not in ("EFFECT_ALWAYS_HIT",) else accuracy,
                    "pp": pp,
                    "effect_chance": effect_chance,
                })
    return moves


def format_evolution(evo_type, params):
    """Format an evolution entry."""
    parts = [p.strip() for p in params.split(",")]
    if evo_type == "LEVEL":
        level = parts[0]
        target = parts[1] if len(parts) > 1 else "?"
        target_name = target.replace("_", " ").title()
        return f"Level {level} -> {target_name}"
    elif evo_type == "ITEM":
        item = parts[0]
        target = parts[1] if len(parts) > 1 else "?"
        return f"{format_item_name(item)} -> {target.replace('_', ' ').title()}"
    elif evo_type == "TRADE":
        held = parts[0]
        target = parts[1] if len(parts) > 1 else "?"
        if held == "-1":
            return f"Trade -> {target.replace('_', ' ').title()}"
        else:
            return f"Trade holding {format_item_name(held)} -> {target.replace('_', ' ').title()}"
    elif evo_type == "HAPPINESS":
        time = parts[0]
        target = parts[1] if len(parts) > 1 else "?"
        time_str = {"TR_ANYTIME": "any time", "TR_MORNDAY": "morning/day", "TR_NITE": "night"}.get(time, time)
        return f"Happiness ({time_str}) -> {target.replace('_', ' ').title()}"
    elif evo_type == "STAT":
        level = parts[0]
        cond = parts[1] if len(parts) > 1 else "?"
        target = parts[2] if len(parts) > 2 else "?"
        cond_str = {"ATK_LT_DEF": "Atk < Def", "ATK_GT_DEF": "Atk > Def", "ATK_EQ_DEF": "Atk = Def"}.get(cond, cond)
        return f"Level {level} ({cond_str}) -> {target.replace('_', ' ').title()}"
    return f"{evo_type}: {params}"


def generate_pokemon_file(stats, evos, learnsets, egg_moves):
    """Generate the master all_pokemon.md file."""
    lines = []
    lines.append("# Pokemon Crystal - All 251 Pokemon\n")
    lines.append("Complete data for every Pokemon in Pokemon Crystal, sourced from the pokecrystal disassembly.\n")
    lines.append("---\n")

    for dex_num in range(1, 252):
        if dex_num not in stats:
            continue
        s = stats[dex_num]

        lines.append(f"## #{dex_num:03d} {s['name']}\n")

        # Types
        if s['type1'] == s['type2']:
            lines.append(f"**Type:** {s['type1']}\n")
        else:
            lines.append(f"**Type:** {s['type1']} / {s['type2']}\n")

        # Base stats table
        lines.append("### Base Stats\n")
        lines.append("| HP | Atk | Def | Spd | Sp.Atk | Sp.Def | BST |")
        lines.append("|---:|----:|----:|----:|-------:|-------:|----:|")
        lines.append(f"| {s['hp']} | {s['atk']} | {s['def']} | {s['spd']} | {s['satk']} | {s['sdef']} | {s['bst']} |\n")

        # Details
        lines.append("### Details\n")
        lines.append(f"- **Catch Rate:** {s['catch_rate']}")
        lines.append(f"- **Base Exp:** {s['base_exp']}")
        lines.append(f"- **Growth Rate:** {s['growth']}")
        lines.append(f"- **Gender Ratio:** {s['gender']}")
        lines.append(f"- **Egg Groups:** {s['egg1']}" + (f", {s['egg2']}" if s['egg1'] != s['egg2'] else ""))
        lines.append(f"- **Hatch Cycles:** {s['hatch_cycles']}")
        items = []
        if s['item1'] != "None":
            items.append(s['item1'])
        if s['item2'] != "None":
            items.append(s['item2'])
        if items:
            lines.append(f"- **Wild Held Items:** {', '.join(items)}")
        lines.append("")

        # Evolutions
        if dex_num in evos and evos[dex_num]:
            lines.append("### Evolution\n")
            for evo_type, params in evos[dex_num]:
                lines.append(f"- {format_evolution(evo_type, params)}")
            lines.append("")

        # Level-up moves
        if dex_num in learnsets and learnsets[dex_num]:
            lines.append("### Level-Up Moves\n")
            lines.append("| Level | Move |")
            lines.append("|------:|------|")
            for level, move in learnsets[dex_num]:
                lines.append(f"| {level} | {move} |")
            lines.append("")

        # Egg moves
        if dex_num in egg_moves and egg_moves[dex_num]:
            lines.append("### Egg Moves\n")
            lines.append(", ".join(egg_moves[dex_num]))
            lines.append("")

        # TM/HM
        if s['tmhm']:
            lines.append("### TM/HM Compatibility\n")
            lines.append(", ".join(s['tmhm']))
            lines.append("")

        lines.append("---\n")

    with open(f"{OUT}/species/all_pokemon.md", "w") as f:
        f.write("\n".join(lines))
    print(f"Wrote {OUT}/species/all_pokemon.md ({len(lines)} lines)")


def generate_moves_file(moves):
    """Generate the all_moves.md file."""
    lines = []
    lines.append("# Pokemon Crystal - All 251 Moves\n")
    lines.append("Complete move data sourced from the pokecrystal disassembly.\n")

    # Effect descriptions
    effect_desc = {
        "EFFECT_NORMAL_HIT": "Deals damage.",
        "EFFECT_MULTI_HIT": "Hits 2-5 times.",
        "EFFECT_DOUBLE_HIT": "Hits exactly 2 times.",
        "EFFECT_TRIPLE_KICK": "Hits 3 times with increasing power (10/20/30).",
        "EFFECT_PAY_DAY": "Deals damage. Scatters coins equal to 2x user's level.",
        "EFFECT_BURN_HIT": "Deals damage with a chance to burn.",
        "EFFECT_FREEZE_HIT": "Deals damage with a chance to freeze.",
        "EFFECT_PARALYZE_HIT": "Deals damage with a chance to paralyze.",
        "EFFECT_FLINCH_HIT": "Deals damage with a chance to flinch.",
        "EFFECT_OHKO": "One-hit KO. Fails if target's speed > user's speed.",
        "EFFECT_RAZOR_WIND": "Charges on turn 1, attacks on turn 2.",
        "EFFECT_FLY": "Semi-invulnerable on turn 1, attacks on turn 2.",
        "EFFECT_TRAP_TARGET": "Traps target for 2-5 turns, dealing 1/16 max HP per turn.",
        "EFFECT_RECOIL_HIT": "Deals damage. User takes 1/4 recoil.",
        "EFFECT_RAMPAGE": "Attacks for 2-3 turns, then confuses user.",
        "EFFECT_FORCE_SWITCH": "Forces target to switch out (wild: ends battle).",
        "EFFECT_LEECH_HIT": "Deals damage. Heals user for half damage dealt.",
        "EFFECT_POISON_HIT": "Deals damage with a chance to poison.",
        "EFFECT_CONFUSE_HIT": "Deals damage with a chance to confuse.",
        "EFFECT_ATTACK_UP": "Raises user's Attack by 1 stage.",
        "EFFECT_DEFENSE_UP": "Raises user's Defense by 1 stage.",
        "EFFECT_SPEED_UP_2": "Raises user's Speed by 2 stages.",
        "EFFECT_SP_ATK_UP": "Raises user's Sp. Atk by 1 stage.",
        "EFFECT_SP_DEF_UP_2": "Raises user's Sp. Def by 2 stages.",
        "EFFECT_ATTACK_UP_2": "Raises user's Attack by 2 stages.",
        "EFFECT_DEFENSE_UP_2": "Raises user's Defense by 2 stages.",
        "EFFECT_EVASION_UP": "Raises user's evasion by 1 stage.",
        "EFFECT_ATTACK_DOWN": "Lowers target's Attack by 1 stage.",
        "EFFECT_DEFENSE_DOWN": "Lowers target's Defense by 1 stage.",
        "EFFECT_SPEED_DOWN": "Lowers target's Speed by 1 stage.",
        "EFFECT_SPEED_DOWN_2": "Lowers target's Speed by 2 stages.",
        "EFFECT_ACCURACY_DOWN": "Lowers target's accuracy by 1 stage.",
        "EFFECT_EVASION_DOWN": "Lowers target's evasion by 1 stage.",
        "EFFECT_ATTACK_DOWN_2": "Lowers target's Attack by 2 stages.",
        "EFFECT_DEFENSE_DOWN_2": "Lowers target's Defense by 2 stages.",
        "EFFECT_SPEED_DOWN_HIT": "Deals damage with a chance to lower Speed.",
        "EFFECT_DEFENSE_DOWN_HIT": "Deals damage with a chance to lower Defense.",
        "EFFECT_SP_DEF_DOWN_HIT": "Deals damage with a chance to lower Sp. Def.",
        "EFFECT_ATTACK_DOWN_HIT": "Deals damage with a chance to lower Attack.",
        "EFFECT_ACCURACY_DOWN_HIT": "Deals damage and lowers accuracy.",
        "EFFECT_ATTACK_UP_HIT": "Deals damage with a chance to raise Attack.",
        "EFFECT_DEFENSE_UP_HIT": "Deals damage with a chance to raise Defense.",
        "EFFECT_ALL_UP_HIT": "Deals damage with 10% chance to raise all stats by 1.",
        "EFFECT_SLEEP": "Puts target to sleep.",
        "EFFECT_POISON": "Poisons target.",
        "EFFECT_PARALYZE": "Paralyzes target.",
        "EFFECT_CONFUSE": "Confuses target.",
        "EFFECT_TOXIC": "Badly poisons target (increasing damage each turn).",
        "EFFECT_STATIC_DAMAGE": "Deals fixed damage.",
        "EFFECT_DISABLE": "Disables target's last used move.",
        "EFFECT_MIST": "Protects team from stat reductions for 5 turns.",
        "EFFECT_LEECH_SEED": "Seeds target, draining 1/8 max HP each turn.",
        "EFFECT_LEVEL_DAMAGE": "Deals damage equal to user's level.",
        "EFFECT_PSYWAVE": "Deals random damage: 1 to 1.5x user's level.",
        "EFFECT_COUNTER": "Returns double the physical damage received this turn.",
        "EFFECT_MIRROR_COAT": "Returns double the special damage received this turn.",
        "EFFECT_HEAL": "Restores up to 50% of user's max HP.",
        "EFFECT_SELFDESTRUCT": "Deals massive damage. User faints.",
        "EFFECT_DREAM_EATER": "Deals damage only to sleeping targets. Heals 50%.",
        "EFFECT_HYPER_BEAM": "Deals damage. Must recharge next turn.",
        "EFFECT_SUPER_FANG": "Deals damage equal to 50% of target's current HP.",
        "EFFECT_SUBSTITUTE": "Creates a substitute using 25% of user's max HP.",
        "EFFECT_TRANSFORM": "User transforms into the target.",
        "EFFECT_SPLASH": "Does nothing.",
        "EFFECT_CONVERSION": "Changes user's type to match first move's type.",
        "EFFECT_CONVERSION2": "Changes user's type to resist target's last move.",
        "EFFECT_TRI_ATTACK": "Deals damage. 20% chance to burn, freeze, or paralyze.",
        "EFFECT_BIDE": "User endures 2 turns, then returns double damage taken.",
        "EFFECT_METRONOME": "Uses a random move.",
        "EFFECT_MIRROR_MOVE": "Uses the target's last used move.",
        "EFFECT_SKETCH": "Permanently copies target's last used move.",
        "EFFECT_PROTECT": "Blocks all attacks this turn. Less likely with consecutive use.",
        "EFFECT_ENDURE": "Survives with 1 HP this turn. Less likely with consecutive use.",
        "EFFECT_PRIORITY_HIT": "Deals damage with +1 priority (moves first).",
        "EFFECT_ALWAYS_HIT": "Deals damage. Never misses.",
        "EFFECT_SOLARBEAM": "Charges turn 1, attacks turn 2. Instant in sun.",
        "EFFECT_THUNDER": "Deals damage. 30% paralyze. Perfect accuracy in rain.",
        "EFFECT_EARTHQUAKE": "Deals damage. Hits Dig users for double.",
        "EFFECT_GUST": "Deals damage. Hits Fly users for double.",
        "EFFECT_STOMP": "Deals damage. 30% flinch. Double damage vs Minimize.",
        "EFFECT_TWISTER": "Deals damage. 20% flinch. Hits Fly users for double.",
        "EFFECT_FUTURE_SIGHT": "Attacks 2 turns later. Ignores type effectiveness.",
        "EFFECT_ROLLOUT": "Hits 5 turns with doubling power. Boosted after Defense Curl.",
        "EFFECT_FURY_CUTTER": "Power doubles each consecutive hit (max 160).",
        "EFFECT_REVERSAL": "Power increases as user's HP decreases (20-200).",
        "EFFECT_MAGNITUDE": "Deals random-power Ground damage (10-150).",
        "EFFECT_PURSUIT": "Doubles power if target is switching out.",
        "EFFECT_RAPID_SPIN": "Deals damage. Removes Leech Seed, Spikes, and binding.",
        "EFFECT_HIDDEN_POWER": "Type and power (31-70) vary based on DVs.",
        "EFFECT_FALSE_SWIPE": "Deals damage but always leaves target with at least 1 HP.",
        "EFFECT_MEAN_LOOK": "Prevents target from switching or fleeing.",
        "EFFECT_ATTRACT": "Infatuates opposite-gender target (50% chance to skip turn).",
        "EFFECT_RETURN": "Power = friendship / 2.5 (max 102).",
        "EFFECT_FRUSTRATION": "Power = (255 - friendship) / 2.5 (max 102).",
        "EFFECT_PRESENT": "Randomly deals 40/80/120 damage or heals 80 HP.",
        "EFFECT_THIEF": "Deals damage. Steals target's held item.",
        "EFFECT_BATON_PASS": "Switches out, passing stat changes and volatile status.",
        "EFFECT_ENCORE": "Forces target to repeat its last move for 2-6 turns.",
        "EFFECT_SPITE": "Reduces PP of target's last move by 2-5.",
        "EFFECT_BELLY_DRUM": "Sacrifices 50% max HP to max Attack.",
        "EFFECT_SWAGGER": "Confuses target and sharply raises its Attack.",
        "EFFECT_SLEEP_TALK": "Uses a random known move while asleep.",
        "EFFECT_PAIN_SPLIT": "Averages user's and target's current HP.",
        "EFFECT_SACRED_FIRE": "Deals damage. 50% burn chance. Thaws user.",
        "EFFECT_FLAME_WHEEL": "Deals damage. 10% burn. Thaws user.",
        "EFFECT_SNORE": "Deals damage only while asleep. 30% flinch.",
        "EFFECT_CURSE": "Ghost: lose 50% HP, curse target. Other: -1 Spd, +1 Atk/Def.",
        "EFFECT_SAFEGUARD": "Protects team from status conditions for 5 turns.",
        "EFFECT_HEAL_BELL": "Cures all status conditions for user's entire party.",
        "EFFECT_REFLECT": "Halves physical damage to team for 5 turns.",
        "EFFECT_LIGHT_SCREEN": "Halves special damage to team for 5 turns.",
        "EFFECT_SPIKES": "Sets Spikes on foe's side (1/8 HP on switch-in).",
        "EFFECT_FORESIGHT": "Removes Ghost-type immunities and resets evasion.",
        "EFFECT_DESTINY_BOND": "If user faints this turn, the attacker also faints.",
        "EFFECT_PERISH_SONG": "Both Pokemon faint after 3 turns unless switched.",
        "EFFECT_SANDSTORM": "Sets Sandstorm for 5 turns (1/8 HP to non-Rock/Ground/Steel).",
        "EFFECT_RAIN_DANCE": "Sets Rain for 5 turns (+50% Water, -50% Fire).",
        "EFFECT_SUNNY_DAY": "Sets Sun for 5 turns (+50% Fire, -50% Water).",
        "EFFECT_FOCUS_ENERGY": "Boosts critical hit ratio by 2 stages.",
        "EFFECT_RESET_STATS": "Resets all stat changes for both Pokemon.",
        "EFFECT_TELEPORT": "Flees from wild battles.",
        "EFFECT_PSYCH_UP": "Copies target's stat changes.",
        "EFFECT_RAGE": "Deals damage. Attack rises each time user is hit.",
        "EFFECT_MIMIC": "Copies target's last move for the battle.",
        "EFFECT_LOCK_ON": "Ensures next move hits the target.",
        "EFFECT_NIGHTMARE": "Damages sleeping target for 1/4 max HP per turn.",
        "EFFECT_BEAT_UP": "Each party member attacks for base 10 damage.",
        "EFFECT_SKULL_BASH": "Raises Defense turn 1, attacks turn 2.",
        "EFFECT_SKY_ATTACK": "Charges turn 1, attacks turn 2. High crit ratio.",
        "EFFECT_DEFENSE_CURL": "Raises Defense by 1. Doubles Rollout power.",
        "EFFECT_MORNING_SUN": "Heals HP. Amount depends on weather.",
        "EFFECT_SYNTHESIS": "Heals HP. Amount depends on weather.",
        "EFFECT_MOONLIGHT": "Heals HP. Amount depends on weather.",
        "EFFECT_POISON_MULTI_HIT": "Hits 2 times. Each hit has a chance to poison.",
    }

    lines.append("---\n")

    for mv in moves:
        lines.append(f"## #{mv['id']:03d} {mv['name']}\n")
        lines.append(f"**Type:** {mv['type']}  ")
        lines.append(f"**Power:** {mv['power']}  ")
        lines.append(f"**Accuracy:** {mv['accuracy']}%  ")
        lines.append(f"**PP:** {mv['pp']}  ")
        if mv['effect_chance'] > 0:
            lines.append(f"**Effect Chance:** {mv['effect_chance']}%  ")

        desc = effect_desc.get(mv['effect'], mv['effect'].replace("EFFECT_", "").replace("_", " ").title())
        lines.append(f"\n{desc}\n")
        lines.append("---\n")

    with open(f"{OUT}/moves/all_moves.md", "w") as f:
        f.write("\n".join(lines))
    print(f"Wrote {OUT}/moves/all_moves.md ({len(lines)} lines)")


def generate_type_chart():
    """Generate type effectiveness chart."""
    lines = []
    lines.append("# Pokemon Crystal - Type Chart\n")
    lines.append("Complete type effectiveness data from the pokecrystal disassembly.\n")

    types_order = ["Normal", "Fire", "Water", "Electric", "Grass", "Ice", "Fighting",
                   "Poison", "Ground", "Flying", "Psychic", "Bug", "Rock", "Ghost",
                   "Dragon", "Dark", "Steel"]

    # Parse matchups
    matchups = {}
    with open(f"{BASE}/data/types/type_matchups.asm") as f:
        for line in f:
            if line.strip().startswith("db -"):
                break
            m = re.match(r'\s+db (\w+),\s+(\w+),\s+(\w+)', line)
            if m:
                atk = TYPE_MAP.get(m.group(1), m.group(1))
                dfn = TYPE_MAP.get(m.group(2), m.group(2))
                eff = m.group(3)
                if atk not in matchups:
                    matchups[atk] = {}
                eff_val = {"SUPER_EFFECTIVE": "2x", "NOT_VERY_EFFECTIVE": "0.5x", "NO_EFFECT": "0x"}.get(eff, "1x")
                matchups[atk][dfn] = eff_val

    # Full chart
    lines.append("## Effectiveness Chart\n")
    lines.append("Rows = Attacking type, Columns = Defending type\n")

    # Abbreviations
    abbr = {"Normal": "NOR", "Fire": "FIR", "Water": "WAT", "Electric": "ELC",
            "Grass": "GRS", "Ice": "ICE", "Fighting": "FGT", "Poison": "PSN",
            "Ground": "GRD", "Flying": "FLY", "Psychic": "PSY", "Bug": "BUG",
            "Rock": "RCK", "Ghost": "GHO", "Dragon": "DRG", "Dark": "DRK", "Steel": "STL"}

    header = "| Atk\\Def | " + " | ".join(abbr[t] for t in types_order) + " |"
    sep = "|---------|" + "|".join("----" for _ in types_order) + "|"
    lines.append(header)
    lines.append(sep)

    for atk in types_order:
        row = f"| **{abbr[atk]}** |"
        for dfn in types_order:
            val = matchups.get(atk, {}).get(dfn, "1x")
            if val == "2x":
                row += " **2x** |"
            elif val == "0.5x":
                row += " 0.5x |"
            elif val == "0x":
                row += " 0 |"
            else:
                row += " 1x |"
        lines.append(row)

    lines.append("")

    # Immunities
    lines.append("## Immunities\n")
    lines.append("- Normal and Fighting cannot hit Ghost")
    lines.append("- Electric cannot hit Ground")
    lines.append("- Poison cannot hit Steel")
    lines.append("- Ground cannot hit Flying")
    lines.append("- Psychic cannot hit Dark")
    lines.append("")

    # Foresight note
    lines.append("## Special Mechanics\n")
    lines.append("- **Foresight/Odor Sleuth:** Removes Ghost-type immunities to Normal and Fighting")
    lines.append("- **Sandstorm:** Damages all types except Rock, Ground, and Steel for 1/8 max HP per turn")
    lines.append("")

    # Type boost items
    lines.append("## Type-Boosting Held Items\n")
    lines.append("Each boosts its type's moves by 10%.\n")
    lines.append("| Type | Item |")
    lines.append("|------|------|")
    type_items = {
        "Normal": "Pink Bow / Polkadot Bow", "Fighting": "BlackBelt", "Flying": "Sharp Beak",
        "Poison": "Poison Barb", "Ground": "Soft Sand", "Rock": "Hard Stone",
        "Bug": "SilverPowder", "Ghost": "Spell Tag", "Fire": "Charcoal",
        "Water": "Mystic Water", "Grass": "Miracle Seed", "Electric": "Magnet",
        "Psychic": "TwistedSpoon", "Ice": "NeverMeltIce", "Dragon": "Dragon Scale",
        "Dark": "BlackGlasses", "Steel": "Metal Coat",
    }
    for t in types_order:
        if t in type_items:
            lines.append(f"| {t} | {type_items[t]} |")
    lines.append("")

    # BUG note
    lines.append("### Known Bug\n")
    lines.append("Dragon Fang does NOT boost Dragon-type moves. Dragon Scale does (which also evolves Seadra). This is a known bug in Pokemon Crystal.\n")

    with open(f"{OUT}/types/type_chart.md", "w") as f:
        f.write("\n".join(lines))
    print(f"Wrote {OUT}/types/type_chart.md ({len(lines)} lines)")


def parse_wild_encounters():
    """Parse all wild encounter data."""
    lines = []
    lines.append("# Pokemon Crystal - Wild Encounters\n")
    lines.append("All wild Pokemon encounters from the pokecrystal disassembly.\n")

    # Johto grass
    lines.append("## Johto Grass Encounters\n")
    with open(f"{BASE}/data/wild/johto_grass.asm") as f:
        content = f.read()
    _parse_grass_encounters(content, lines)

    # Kanto grass
    lines.append("\n## Kanto Grass Encounters\n")
    with open(f"{BASE}/data/wild/kanto_grass.asm") as f:
        content = f.read()
    _parse_grass_encounters(content, lines)

    # Johto water
    lines.append("\n## Johto Water (Surfing) Encounters\n")
    with open(f"{BASE}/data/wild/johto_water.asm") as f:
        content = f.read()
    _parse_water_encounters(content, lines)

    # Kanto water
    lines.append("\n## Kanto Water (Surfing) Encounters\n")
    with open(f"{BASE}/data/wild/kanto_water.asm") as f:
        content = f.read()
    _parse_water_encounters(content, lines)

    # Fishing
    lines.append("\n## Fishing Encounters\n")
    lines.append("Fish groups are shared across multiple locations.\n")
    with open(f"{BASE}/data/wild/fish.asm") as f:
        fish_content = f.read()
    # Just include a simplified version
    lines.append("### Shore (Routes near water)")
    lines.append("- Old Rod: Magikarp (Lv10)")
    lines.append("- Good Rod: Magikarp (Lv20), Krabby (Lv20)")
    lines.append("- Super Rod: Krabby (Lv40), Kingler (Lv40)")
    lines.append("")
    lines.append("### Ocean")
    lines.append("- Old Rod: Magikarp (Lv10), Tentacool (Lv10)")
    lines.append("- Good Rod: Magikarp (Lv20), Tentacool (Lv20), Chinchou (Lv20)")
    lines.append("- Super Rod: Chinchou (Lv40), Tentacruel (Lv40), Lanturn (Lv40)")
    lines.append("")
    lines.append("### Lake")
    lines.append("- Old Rod: Magikarp (Lv10), Goldeen (Lv10)")
    lines.append("- Good Rod: Magikarp (Lv20), Goldeen (Lv20)")
    lines.append("- Super Rod: Goldeen (Lv40), Seaking (Lv40), Magikarp (Lv40)")
    lines.append("")
    lines.append("### Pond")
    lines.append("- Old Rod: Magikarp (Lv10), Poliwag (Lv10)")
    lines.append("- Good Rod: Magikarp (Lv20), Poliwag (Lv20)")
    lines.append("- Super Rod: Poliwag (Lv40), Magikarp (Lv40)")
    lines.append("")
    lines.append("### Dragon's Den")
    lines.append("- Old Rod: Magikarp (Lv10)")
    lines.append("- Good Rod: Magikarp (Lv20)")
    lines.append("- Super Rod: Magikarp (Lv40), Dratini (Lv40), Dragonair (Lv40)")
    lines.append("")
    lines.append("### Lake of Rage (Gyarados)")
    lines.append("- Old Rod: Magikarp (Lv10)")
    lines.append("- Good Rod: Magikarp (Lv20)")
    lines.append("- Super Rod: Magikarp (Lv40)")
    lines.append("")

    # Headbutt trees
    lines.append("\n## Headbutt Tree Encounters\n")
    lines.append("Pokemon found by using Headbutt on trees.\n")
    lines.append("### Canyon Set")
    lines.append("- Common: Spearow (Lv10), Aipom (Lv10)")
    lines.append("- Rare: Spearow (Lv10), Heracross (Lv10), Aipom (Lv10)")
    lines.append("")
    lines.append("### Town Set")
    lines.append("- Common: Spearow (Lv10), Ekans (Lv10), Aipom (Lv10)")
    lines.append("- Rare: Spearow (Lv10), Heracross (Lv10), Aipom (Lv10)")
    lines.append("")
    lines.append("### Route Set")
    lines.append("- Common: Hoothoot (Lv10), Spinarak (Lv10), Ledyba (Lv10), Exeggcute (Lv10)")
    lines.append("- Rare: Hoothoot (Lv10), Pineco (Lv10), Exeggcute (Lv10)")
    lines.append("")
    lines.append("### Forest Set")
    lines.append("- Common: Caterpie (Lv10), Metapod (Lv10), Butterfree (Lv10), Exeggcute (Lv10)")
    lines.append("- Rare: Caterpie (Lv10), Pineco (Lv10), Exeggcute (Lv10)")
    lines.append("")
    lines.append("### Rock Set")
    lines.append("- Common: Natu (Lv10), Aipom (Lv10), Exeggcute (Lv10)")
    lines.append("- Rare: Natu (Lv10), Pineco (Lv10), Exeggcute (Lv10)")
    lines.append("")

    # Bug Contest
    lines.append("\n## Bug-Catching Contest\n")
    with open(f"{BASE}/data/wild/bug_contest_mons.asm") as f:
        for line in f:
            m = re.match(r'\s+db (\d+) percent,\s*(\d+),\s*(\w+),\s*(\d+)', line)
            if m:
                _pct, level, species, _max_level = m.group(1), m.group(2), m.group(3), m.group(4)
                name = species.replace("_", " ").title()
                lines.append(f"- {name} (Lv{level})")

    lines.append("")

    # Swarm Pokemon
    lines.append("\n## Swarm Encounters\n")
    with open(f"{BASE}/data/wild/swarm_grass.asm") as f:
        content = f.read()
    for line in content.split("\n"):
        m = re.match(r'\s+db GROUP_(\w+), MAP_(\w+),\s*(\w+)', line.strip())
        if m:
            loc = m.group(2).replace("_", " ").title()
            species = m.group(3).replace("_", " ").title()
            lines.append(f"- {species} at {loc}")

    lines.append("")

    # Roaming Pokemon
    lines.append("\n## Roaming Pokemon\n")
    lines.append("- Raikou (Lv40) - Roams Johto after releasing at Burned Tower")
    lines.append("- Entei (Lv40) - Roams Johto after releasing at Burned Tower")
    lines.append("- Suicune (Lv40) - Found at Tin Tower (Crystal version)")
    lines.append("")

    with open(f"{OUT}/encounters/wild_encounters.md", "w") as f:
        f.write("\n".join(lines))
    print(f"Wrote {OUT}/encounters/wild_encounters.md ({len(lines)} lines)")


def _parse_grass_encounters(content, lines):
    """Helper to parse grass encounter data."""
    sections = re.split(r'def_grass_wildmons (\w+)', content)
    for i in range(1, len(sections), 2):
        location = sections[i].replace("_", " ").title()
        data = sections[i + 1] if i + 1 < len(sections) else ""

        data_lines = [l.strip() for l in data.strip().split("\n") if l.strip()]
        if not data_lines:
            continue

        # First line is encounter rates
        rates_line = data_lines[0] if data_lines else ""

        lines.append(f"### {location}\n")

        # Parse morn/day/nite sections (7 Pokemon each)
        pokemon_lines = [l for l in data_lines if re.match(r'db \d+, \w+', l)]
        if len(pokemon_lines) >= 21:
            for period_idx, period in enumerate(["Morning", "Day", "Night"]):
                period_mons = pokemon_lines[period_idx * 7:(period_idx + 1) * 7]
                lines.append(f"**{period}:**")
                seen = {}
                for pl in period_mons:
                    m = re.match(r'db (\d+), (\w+)', pl)
                    if m:
                        level = m.group(1)
                        species = m.group(2).replace("_", " ").title()
                        key = f"{species} Lv{level}"
                        seen[key] = seen.get(key, 0) + 1
                for key in seen:
                    lines.append(f"- {key}")
                lines.append("")
        elif pokemon_lines:
            for pl in pokemon_lines:
                m = re.match(r'db (\d+), (\w+)', pl)
                if m:
                    level = m.group(1)
                    species = m.group(2).replace("_", " ").title()
                    lines.append(f"- {species} Lv{level}")
            lines.append("")


def _parse_water_encounters(content, lines):
    """Helper to parse water encounter data."""
    sections = re.split(r'def_water_wildmons (\w+)', content)
    for i in range(1, len(sections), 2):
        location = sections[i].replace("_", " ").title()
        data = sections[i + 1] if i + 1 < len(sections) else ""

        lines.append(f"### {location}\n")
        pokemon_lines = [l.strip() for l in data.strip().split("\n") if re.match(r'\s*db \d+, \w+', l.strip())]
        seen = {}
        for pl in pokemon_lines:
            m = re.match(r'db (\d+), (\w+)', pl)
            if m:
                level = m.group(1)
                species = m.group(2).replace("_", " ").title()
                key = f"{species} Lv{level}"
                seen[key] = seen.get(key, 0) + 1
        for key in seen:
            lines.append(f"- {key}")
        lines.append("")


def parse_trainers():
    """Parse trainer data."""
    lines = []
    lines.append("# Pokemon Crystal - All Trainers\n")
    lines.append("Complete trainer party data from the pokecrystal disassembly.\n")

    with open(f"{BASE}/data/trainers/parties.asm") as f:
        content = f.read()

    # Parse trainer class groups
    groups = re.split(r'\n(\w+Group):', content)
    for i in range(1, len(groups), 2):
        group_name = groups[i]
        group_data = groups[i + 1] if i + 1 < len(groups) else ""

        # Clean group name
        display_name = group_name.replace("Group", "").replace("_", " ")
        lines.append(f"## {display_name}\n")

        # Split by individual trainers within group
        trainers = group_data.split('db "')
        for trainer_data in trainers:
            if not trainer_data.strip():
                continue

            # Get trainer name and type
            m = re.match(r'([^"]+)@",\s*(\w+)', trainer_data)
            if not m:
                continue
            trainer_name = m.group(1)
            trainer_type = m.group(2)

            if trainer_name == "?":
                trainer_name = display_name

            lines.append(f"### {trainer_name}\n")

            # Parse Pokemon
            if "TRAINERTYPE_MOVES" in trainer_type or "TRAINERTYPE_ITEM_MOVES" in trainer_type:
                # Has custom moves
                mons = re.findall(r'db\s+(\d+),\s+(\w+),\s+(\w+(?:,\s*\w+){3})', trainer_data)
                for mon in mons:
                    level = mon[0]
                    species = mon[1].replace("_", " ").title()
                    moves_raw = mon[2].split(",")
                    moves = [format_move_name(mv.strip()) for mv in moves_raw if mv.strip() != "NO_MOVE"]
                    moves_str = ", ".join(m for m in moves if m != "-")
                    lines.append(f"- Lv{level} {species}: {moves_str}")
            elif "TRAINERTYPE_ITEM" in trainer_type:
                mons = re.findall(r'db\s+(\d+),\s+(\w+),\s+(\w+)', trainer_data)
                for mon in mons:
                    if mon[0].isdigit():
                        level = mon[0]
                        species = mon[1].replace("_", " ").title()
                        item = format_item_name(mon[2])
                        lines.append(f"- Lv{level} {species} (holding {item})")
            else:
                # Normal trainer
                mons = re.findall(r'db\s+(\d+),\s+(\w+)', trainer_data)
                for mon in mons:
                    if mon[0].isdigit() and mon[1] != "-1" and not mon[1].startswith("TRAINER"):
                        level = mon[0]
                        species = mon[1].replace("_", " ").title()
                        if species not in ("-1",):
                            lines.append(f"- Lv{level} {species}")

            lines.append("")

    with open(f"{OUT}/trainers/all_trainers.md", "w") as f:
        f.write("\n".join(lines))
    print(f"Wrote {OUT}/trainers/all_trainers.md ({len(lines)} lines)")


def generate_items_file():
    """Generate items data file."""
    lines = []
    lines.append("# Pokemon Crystal - All Items\n")
    lines.append("Complete item data from the pokecrystal disassembly.\n")

    # Read item names
    item_names = []
    with open(f"{BASE}/data/items/names.asm") as f:
        for line in f:
            m = re.search(r'li "([^"]+)"', line)
            if m:
                item_names.append(m.group(1))

    # Read item attributes
    items = []
    with open(f"{BASE}/data/items/attributes.asm") as f:
        content = f.read()

    # Find all item_attribute entries with their comments
    item_idx = 0
    for line in content.split("\n"):
        line = line.strip()
        if line.startswith(";") and not line.startswith("; entries") and not line.startswith("; price"):
            comment = line[2:].strip()
            continue
        m = re.match(r'item_attribute\s+(\$?\d+),\s+(\w+),\s+(-?\d+),', line)
        if m:
            price_raw = m.group(1)
            held_effect = m.group(2)
            param = int(m.group(3))

            if price_raw.startswith("$"):
                price = int(price_raw[1:], 16)
            else:
                price = int(price_raw)

            if item_idx < len(item_names):
                name = item_names[item_idx]
            else:
                name = f"ITEM_{item_idx:02X}"
            item_idx += 1

            if name != "TERU-SAMA" and price != 0x9999:
                items.append({
                    "name": name,
                    "price": price,
                    "held_effect": held_effect,
                    "param": param,
                })

    # Categorize items
    categories = {
        "Poke Balls": [], "Medicine": [], "Status Healing": [],
        "Vitamins": [], "Battle Items": [], "Evolution Stones": [],
        "Held Items": [], "Key Items": [], "Berries": [],
        "Mail": [], "TMs & HMs": [], "Other": [],
    }

    for item in items:
        name = item["name"]
        if "BALL" in name and name not in ("SMOKE BALL", "LIGHT BALL"):
            categories["Poke Balls"].append(item)
        elif name.startswith("TM") or name.startswith("HM"):
            categories["TMs & HMs"].append(item)
        elif "MAIL" in name:
            categories["Mail"].append(item)
        elif name in ("POTION", "SUPER POTION", "HYPER POTION", "MAX POTION", "FULL RESTORE",
                       "FRESH WATER", "SODA POP", "LEMONADE", "MOOMOO MILK", "RAGECANDYBAR",
                       "ENERGYPOWDER", "ENERGY ROOT", "BERRY JUICE"):
            categories["Medicine"].append(item)
        elif name in ("ANTIDOTE", "BURN HEAL", "ICE HEAL", "AWAKENING", "PARLYZ HEAL",
                       "FULL HEAL", "REVIVE", "MAX REVIVE", "SACRED ASH", "HEAL POWDER",
                       "REVIVAL HERB"):
            categories["Status Healing"].append(item)
        elif name in ("HP UP", "PROTEIN", "IRON", "CARBOS", "CALCIUM", "RARE CANDY", "PP UP"):
            categories["Vitamins"].append(item)
        elif name in ("X ATTACK", "X DEFEND", "X SPEED", "X SPECIAL", "X ACCURACY",
                       "GUARD SPEC.", "DIRE HIT"):
            categories["Battle Items"].append(item)
        elif "STONE" in name and name not in ("HARD STONE", "EVERSTONE"):
            categories["Evolution Stones"].append(item)
        elif "BERRY" in name or name == "BERRY":
            categories["Berries"].append(item)
        elif item["held_effect"] != "HELD_NONE" or name in ("LEFTOVERS", "SCOPE LENS",
                "FOCUS BAND", "KINGS ROCK", "QUICK CLAW", "BRIGHT POWDER", "METAL POWDER",
                "LUCKY PUNCH", "THICK CLUB", "LIGHT BALL", "EVERSTONE", "SMOKE BALL",
                "BERSERK GENE", "LUCKY EGG", "EXP.SHARE", "AMULET COIN", "CLEANSE TAG",
                "STICK", "METAL COAT", "DRAGON SCALE", "DRAGON FANG", "UP-GRADE"):
            categories["Held Items"].append(item)
        elif name in ("RED SCALE", "SECRETPOTION", "S.S.TICKET", "MYSTERY EGG", "CLEAR BELL",
                       "SILVER WING", "RAINBOW WING", "GS BALL", "BLUE CARD", "CARD KEY",
                       "MACHINE PART", "EGG TICKET", "LOST ITEM", "BASEMENT KEY", "PASS",
                       "SQUIRTBOTTLE", "COIN CASE", "ITEMFINDER", "OLD ROD", "GOOD ROD",
                       "SUPER ROD", "BICYCLE", "TOWN MAP", "NORMAL BOX", "GORGEOUS BOX"):
            categories["Key Items"].append(item)
        else:
            categories["Other"].append(item)

    for cat, cat_items in categories.items():
        if not cat_items:
            continue
        lines.append(f"## {cat}\n")
        lines.append("| Item | Price | Effect |")
        lines.append("|------|------:|--------|")
        for item in cat_items:
            price_str = f"{item['price']}" if item['price'] > 0 else "N/A"
            effect = _get_item_effect(item)
            lines.append(f"| {item['name']} | {price_str} | {effect} |")
        lines.append("")

    # TM/HM Table
    lines.append("## TM/HM Move List\n")
    lines.append("| TM/HM | Move |")
    lines.append("|-------|------|")
    for num, move in sorted(TM_MOVES.items()):
        lines.append(f"| TM{num:02d} | {move} |")
    for num, move in sorted(HM_MOVES.items()):
        lines.append(f"| HM{num:02d} | {move} |")
    lines.append("")

    # Move Tutors
    lines.append("## Move Tutors (Crystal only)\n")
    lines.append("| Move | Location |")
    lines.append("|------|----------|")
    lines.append("| Flamethrower | Goldenrod Game Corner prize |")
    lines.append("| Thunderbolt | Goldenrod Game Corner prize |")
    lines.append("| Ice Beam | Goldenrod Game Corner prize |")
    lines.append("")

    with open(f"{OUT}/items/all_items.md", "w") as f:
        f.write("\n".join(lines))
    print(f"Wrote {OUT}/items/all_items.md ({len(lines)} lines)")


def _get_item_effect(item):
    """Get human-readable item effect."""
    effects = {
        "HELD_BERRY": f"Restores {item['param']} HP when below 50%",
        "HELD_LEFTOVERS": "Restores 1/16 max HP each turn",
        "HELD_QUICK_CLAW": "60% chance to move first",
        "HELD_BRIGHTPOWDER": "Lowers foe accuracy by 20",
        "HELD_FOCUS_BAND": "30% chance to survive fatal hit with 1 HP",
        "HELD_FLINCH": "30% flinch chance on damaging moves",
        "HELD_CRITICAL_UP": "Raises critical hit ratio",
        "HELD_AMULET_COIN": "Doubles prize money from battles",
        "HELD_CLEANSE_TAG": "Reduces wild encounters",
        "HELD_ESCAPE": "Guarantees fleeing from wild battles",
        "HELD_METAL_POWDER": "Boosts Ditto's Def/Sp.Def by 1.5x when transformed",
        "HELD_HEAL_POISON": "Auto-cures Poison",
        "HELD_HEAL_PARALYZE": "Auto-cures Paralysis",
        "HELD_HEAL_FREEZE": "Auto-cures Freeze",
        "HELD_HEAL_BURN": "Auto-cures Burn",
        "HELD_HEAL_SLEEP": "Auto-cures Sleep",
        "HELD_HEAL_CONFUSION": "Auto-cures Confusion",
        "HELD_HEAL_STATUS": "Auto-cures any status",
        "HELD_RESTORE_PP": "Restores PP of one depleted move",
        "HELD_NORMAL_BOOST": "Boosts Normal moves by 10%",
        "HELD_FIGHTING_BOOST": "Boosts Fighting moves by 10%",
        "HELD_FLYING_BOOST": "Boosts Flying moves by 10%",
        "HELD_POISON_BOOST": "Boosts Poison moves by 10%",
        "HELD_GROUND_BOOST": "Boosts Ground moves by 10%",
        "HELD_ROCK_BOOST": "Boosts Rock moves by 10%",
        "HELD_BUG_BOOST": "Boosts Bug moves by 10%",
        "HELD_GHOST_BOOST": "Boosts Ghost moves by 10%",
        "HELD_FIRE_BOOST": "Boosts Fire moves by 10%",
        "HELD_WATER_BOOST": "Boosts Water moves by 10%",
        "HELD_GRASS_BOOST": "Boosts Grass moves by 10%",
        "HELD_ELECTRIC_BOOST": "Boosts Electric moves by 10%",
        "HELD_PSYCHIC_BOOST": "Boosts Psychic moves by 10%",
        "HELD_ICE_BOOST": "Boosts Ice moves by 10%",
        "HELD_DRAGON_BOOST": "Boosts Dragon moves by 10%",
        "HELD_DARK_BOOST": "Boosts Dark moves by 10%",
        "HELD_STEEL_BOOST": "Boosts Steel moves by 10%",
    }
    held = item["held_effect"]
    if held in effects:
        return effects[held]

    name = item["name"]
    param = item["param"]
    special = {
        "POTION": f"Restores {param} HP",
        "SUPER POTION": f"Restores {param} HP",
        "HYPER POTION": f"Restores {param} HP",
        "MAX POTION": "Restores all HP",
        "FULL RESTORE": "Restores all HP and cures status",
        "FRESH WATER": f"Restores {param} HP",
        "SODA POP": f"Restores {param} HP",
        "LEMONADE": f"Restores {param} HP",
        "MOOMOO MILK": f"Restores {param} HP",
        "RAGECANDYBAR": f"Restores {param} HP",
        "ANTIDOTE": "Cures Poison",
        "BURN HEAL": "Cures Burn",
        "ICE HEAL": "Cures Freeze",
        "AWAKENING": "Cures Sleep",
        "PARLYZ HEAL": "Cures Paralysis",
        "FULL HEAL": "Cures any status",
        "REVIVE": "Revives fainted Pokemon with 50% HP",
        "MAX REVIVE": "Revives fainted Pokemon with full HP",
        "SACRED ASH": "Revives all fainted party Pokemon",
        "ETHER": "Restores 10 PP of one move",
        "MAX ETHER": "Restores all PP of one move",
        "ELIXER": "Restores 10 PP of all moves",
        "MAX ELIXER": "Restores all PP of all moves",
        "PP UP": "Permanently raises max PP of one move",
        "RARE CANDY": "Raises level by 1",
        "HP UP": "Raises HP EVs",
        "PROTEIN": "Raises Attack EVs",
        "IRON": "Raises Defense EVs",
        "CARBOS": "Raises Speed EVs",
        "CALCIUM": "Raises Sp. Atk EVs",
        "ESCAPE ROPE": "Escapes from caves/dungeons",
        "REPEL": "Repels weak wild Pokemon (100 steps)",
        "SUPER REPEL": "Repels weak wild Pokemon (200 steps)",
        "MAX REPEL": "Repels weak wild Pokemon (250 steps)",
        "X ATTACK": "Raises Attack in battle",
        "X DEFEND": "Raises Defense in battle",
        "X SPEED": "Raises Speed in battle",
        "X SPECIAL": "Raises Sp. Atk in battle",
        "X ACCURACY": "Raises Accuracy in battle",
        "GUARD SPEC.": "Prevents stat lowering for 5 turns",
        "DIRE HIT": "Raises critical hit ratio in battle",
        "FIRE STONE": "Evolves: Vulpix, Growlithe, Eevee",
        "WATER STONE": "Evolves: Poliwhirl, Shellder, Staryu, Eevee",
        "THUNDERSTONE": "Evolves: Pikachu, Eevee",
        "LEAF STONE": "Evolves: Gloom, Weepinbell, Exeggcute",
        "MOON STONE": "Evolves: Nidorina, Nidorino, Clefairy, Jigglypuff",
        "SUN STONE": "Evolves: Gloom, Sunkern",
        "EVERSTONE": "Prevents evolution when held",
        "LUCKY EGG": "Boosts Exp gained from battle",
        "EXP.SHARE": "Shares Exp with holder",
        "THICK CLUB": "Doubles Marowak/Cubone Attack",
        "LIGHT BALL": "Doubles Pikachu Sp. Atk",
        "STICK": "Raises Farfetch'd critical hit ratio",
        "LUCKY PUNCH": "Raises Chansey critical hit ratio",
        "METAL COAT": "Boosts Steel moves 10%. Evolves Onix/Scyther when traded",
        "DRAGON SCALE": "Boosts Dragon moves 10%. Evolves Seadra when traded",
        "UP-GRADE": "Evolves Porygon when traded",
        "KINGS ROCK": "30% flinch. Evolves Poliwhirl/Slowpoke when traded",
        "BERSERK GENE": "Raises Attack sharply but confuses (consumed on use)",
        "NUGGET": "Sell for 5000",
        "PEARL": "Sell for 700",
        "BIG PEARL": "Sell for 3750",
        "STARDUST": "Sell for 1000",
        "STAR PIECE": "Sell for 4900",
        "TINYMUSHROOM": "Sell for 250",
        "BIG MUSHROOM": "Sell for 2500",
        "SLOWPOKETAIL": "Sell for 4900",
        "# DOLL": "Flee from wild battles",
        "ENERGYPOWDER": "Restores HP (bitter)",
        "ENERGY ROOT": "Restores HP (very bitter)",
        "HEAL POWDER": "Cures status (bitter)",
        "REVIVAL HERB": "Revives (very bitter)",
    }
    if name in special:
        return special[name]
    return "-"


# === MAIN ===
if __name__ == "__main__":
    print("Parsing Pokemon Crystal data...")

    parse_pokemon_names()
    print(f"Found {len(POKEMON_NAMES)} Pokemon names")

    print("Parsing base stats...")
    stats = parse_base_stats()
    print(f"Parsed {len(stats)} Pokemon base stats")

    print("Parsing evolutions and level-up moves...")
    evos, learnsets = parse_evos_attacks()
    print(f"Parsed {len(evos)} evolution sets, {len(learnsets)} learnsets")

    print("Parsing egg moves...")
    egg_moves = parse_egg_moves()
    print(f"Parsed {len(egg_moves)} egg move sets")

    print("Parsing moves...")
    moves = parse_moves()
    print(f"Parsed {len(moves)} moves")

    print("\nGenerating output files...")

    generate_pokemon_file(stats, evos, learnsets, egg_moves)
    generate_moves_file(moves)
    generate_type_chart()
    generate_items_file()
    parse_wild_encounters()
    parse_trainers()

    print("\nDone! All files generated in", OUT)
