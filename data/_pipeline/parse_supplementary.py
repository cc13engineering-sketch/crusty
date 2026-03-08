#!/usr/bin/env python3
"""Generate supplementary deep-dive data files from pokecrystal source."""

import re
import glob

BASE = "/Users/colinlaptop/DIY/AI/crusty/engine/crates/engine-core/src/pokemon/pokecrystal-master"
OUT = "/Users/colinlaptop/DIY/AI/crusty/data"

# ─── Shared helpers ───

TYPE_MAP = {
    "NORMAL": "Normal", "FIGHTING": "Fighting", "FLYING": "Flying",
    "POISON": "Poison", "GROUND": "Ground", "ROCK": "Rock",
    "BUG": "Bug", "GHOST": "Ghost", "STEEL": "Steel",
    "FIRE": "Fire", "WATER": "Water", "GRASS": "Grass",
    "ELECTRIC": "Electric", "PSYCHIC_TYPE": "Psychic", "ICE": "Ice",
    "DRAGON": "Dragon", "DARK": "Dark", "CURSE_TYPE": "???",
}

def fmt_move(name):
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
    return " ".join(p.capitalize() for p in name.split("_"))

def fmt_item(name):
    if name == "NO_ITEM": return "None"
    return " ".join(p.capitalize() for p in name.split("_"))


POKEMON_NAMES = []
def load_names():
    global POKEMON_NAMES
    if POKEMON_NAMES:
        return
    with open(f"{BASE}/data/pokemon/names.asm") as f:
        for line in f:
            m = re.search(r'dname "([^"]+)"', line)
            if m:
                n = m.group(1)
                if n not in ("?????", "EGG"):
                    POKEMON_NAMES.append(n)
                if len(POKEMON_NAMES) >= 251:
                    break


def name_to_dex(pname):
    """Convert a PascalCase asm label name to a dex number."""
    special = {"NidoranF": 29, "NidoranM": 32, "MrMime": 122, "FarfetchD": 83, "HoOh": 250}
    if pname in special:
        return special[pname]
    for idx, n in enumerate(POKEMON_NAMES):
        clean = n.replace("\u2640", "F").replace("\u2642", "M").replace(".", "").replace("'", "").replace("-", "").replace(" ", "")
        if clean.upper() == pname.upper():
            return idx + 1
    return None


# ═══════════════════════════════════════════
# FILE 1: Evolution Chains
# ═══════════════════════════════════════════

def gen_evolution_chains():
    load_names()

    # Parse all evolutions
    evo_from = {}  # dex -> [(method, target_dex)]
    evo_to = {}    # dex -> parent_dex

    with open(f"{BASE}/data/pokemon/evos_attacks.asm") as f:
        content = f.read()

    sections = re.split(r'\n(\w+)EvosAttacks:', content)
    for i in range(1, len(sections), 2):
        pname = sections[i]
        sdata = sections[i+1] if i+1 < len(sections) else ""
        dex = name_to_dex(pname)
        if not dex:
            continue

        evolutions = []
        for line in sdata.strip().split("\n"):
            line = line.strip()
            if line == "db 0 ; no more evolutions":
                break
            em = re.match(r'db EVOLVE_(\w+),\s*(.*)', line)
            if em:
                etype = em.group(1)
                params = [p.strip() for p in em.group(2).split(",")]
                evolutions.append((etype, params))

        for etype, params in evolutions:
            target_const = params[-1]
            target_dex = name_to_dex(target_const.replace("_", ""))
            if not target_dex:
                # try as-is in POKEMON_NAMES
                for idx, n in enumerate(POKEMON_NAMES):
                    if n == target_const.replace("_", " ").upper() or n == target_const:
                        target_dex = idx + 1
                        break
            if not target_dex:
                clean = target_const.replace("_", "")
                for idx, n in enumerate(POKEMON_NAMES):
                    c = n.replace("\u2640", "F").replace("\u2642", "M").replace(".", "").replace("'", "").replace("-", "").replace(" ", "")
                    if c.upper() == clean.upper():
                        target_dex = idx + 1
                        break

            if target_dex:
                method_str = _format_evo_method(etype, params[:-1])
                if dex not in evo_from:
                    evo_from[dex] = []
                evo_from[dex].append((method_str, target_dex))
                evo_to[target_dex] = dex

    # Build chains - find all base forms (no parent)
    base_forms = set()
    for d in range(1, 252):
        if d not in evo_to:
            base_forms.add(d)

    # Group into families
    families = []
    visited = set()

    for base in sorted(base_forms):
        if base in visited:
            continue
        family = _build_family(base, evo_from, visited)
        if family:
            families.append(family)

    # Generate output
    lines = []
    lines.append("# Pokemon Crystal - Complete Evolution Chains\n")
    lines.append("All evolution families organized by base form. Sourced from pokecrystal evos_attacks.asm.\n")
    lines.append("---\n")

    for family in families:
        base_dex = family[0]
        base_name = POKEMON_NAMES[base_dex - 1]

        # Determine family type
        evos = evo_from.get(base_dex, [])
        if not evos:
            # Single-stage, skip unless notable
            lines.append(f"## {base_name} (#{base_dex:03d}) — No Evolution\n")
            lines.append("---\n")
            continue

        is_branching = len(evos) > 1 or any(len(evo_from.get(t, [])) > 1 for _, t in evos)

        lines.append(f"## {base_name} Family\n")

        if is_branching:
            lines.append("**Branching evolution chain**\n")

        # Draw the chain
        _draw_chain(base_dex, evo_from, lines, 0)
        lines.append("")
        lines.append("---\n")

    with open(f"{OUT}/species/evolution_chains.md", "w") as f:
        f.write("\n".join(lines))
    print(f"Wrote species/evolution_chains.md ({len(lines)} lines)")


def _format_evo_method(etype, params):
    if etype == "LEVEL":
        return f"Level {params[0]}"
    elif etype == "ITEM":
        return fmt_item(params[0])
    elif etype == "TRADE":
        if params[0] == "-1":
            return "Trade"
        return f"Trade holding {fmt_item(params[0])}"
    elif etype == "HAPPINESS":
        t = {"TR_ANYTIME": "any time", "TR_MORNDAY": "morning/day", "TR_NITE": "night"}.get(params[0], params[0])
        return f"Happiness ({t})"
    elif etype == "STAT":
        cond = {"ATK_LT_DEF": "Atk < Def", "ATK_GT_DEF": "Atk > Def", "ATK_EQ_DEF": "Atk = Def"}.get(params[1], params[1])
        return f"Level {params[0]} ({cond})"
    return f"{etype}: {', '.join(params)}"


def _build_family(base_dex, evo_from, visited):
    family = []
    queue = [base_dex]
    while queue:
        d = queue.pop(0)
        if d in visited:
            continue
        visited.add(d)
        family.append(d)
        for _, target in evo_from.get(d, []):
            queue.append(target)
    return family


def _draw_chain(dex, evo_from, lines, depth):
    name = POKEMON_NAMES[dex - 1]
    prefix = "  " * depth
    if depth == 0:
        lines.append(f"{prefix}**{name}** (#{dex:03d})")
    evos = evo_from.get(dex, [])
    for method, target_dex in evos:
        target_name = POKEMON_NAMES[target_dex - 1]
        lines.append(f"{prefix}  -> [{method}] -> **{target_name}** (#{target_dex:03d})")
        _draw_chain(target_dex, evo_from, lines, depth + 1)


# ═══════════════════════════════════════════
# FILE 2: Learnset by Move (reverse index)
# ═══════════════════════════════════════════

def gen_learnset_by_move():
    load_names()

    # move_name -> [(pokemon_name, method, details)]
    move_learners = {}

    # 1) Level-up moves from evos_attacks.asm
    with open(f"{BASE}/data/pokemon/evos_attacks.asm") as f:
        content = f.read()

    sections = re.split(r'\n(\w+)EvosAttacks:', content)
    for i in range(1, len(sections), 2):
        pname = sections[i]
        sdata = sections[i+1] if i+1 < len(sections) else ""
        dex = name_to_dex(pname)
        if not dex:
            continue
        pokemon_name = POKEMON_NAMES[dex - 1]

        in_moves = False
        for line in sdata.strip().split("\n"):
            line = line.strip()
            if line == "db 0 ; no more evolutions":
                in_moves = True
                continue
            if not in_moves:
                continue
            if line == "db 0 ; no more level-up moves":
                break
            mm = re.match(r'db\s+(\d+),\s+(\w+)', line)
            if mm:
                level = int(mm.group(1))
                move = fmt_move(mm.group(2))
                if move not in move_learners:
                    move_learners[move] = []
                move_learners[move].append((pokemon_name, f"Level {level}"))

    # 2) Egg moves
    with open(f"{BASE}/data/pokemon/egg_moves.asm") as f:
        content = f.read()

    sections = re.split(r'\n(\w+)EggMoves:', content)
    for i in range(1, len(sections), 2):
        pname = sections[i]
        dex = name_to_dex(pname)
        if not dex:
            continue
        pokemon_name = POKEMON_NAMES[dex - 1]
        sdata = sections[i+1] if i+1 < len(sections) else ""
        for line in sdata.strip().split("\n"):
            line = line.strip()
            if line.startswith("db -1"):
                break
            mm = re.match(r'db (\w+)', line)
            if mm and mm.group(1) != "-1":
                move = fmt_move(mm.group(1))
                if move not in move_learners:
                    move_learners[move] = []
                move_learners[move].append((pokemon_name, "Egg Move"))

    # 3) TM/HM compatibility from base_stats
    tm_map = {
        "DYNAMICPUNCH": "TM01", "HEADBUTT": "TM02", "CURSE": "TM03", "ROLLOUT": "TM04",
        "ROAR": "TM05", "TOXIC": "TM06", "ZAP_CANNON": "TM07", "ROCK_SMASH": "TM08",
        "PSYCH_UP": "TM09", "HIDDEN_POWER": "TM10", "SUNNY_DAY": "TM11",
        "SWEET_SCENT": "TM12", "SNORE": "TM13", "BLIZZARD": "TM14",
        "HYPER_BEAM": "TM15", "ICY_WIND": "TM16", "PROTECT": "TM17",
        "RAIN_DANCE": "TM18", "GIGA_DRAIN": "TM19", "ENDURE": "TM20",
        "FRUSTRATION": "TM21", "SOLARBEAM": "TM22", "IRON_TAIL": "TM23",
        "DRAGONBREATH": "TM24", "THUNDER": "TM25", "EARTHQUAKE": "TM26",
        "RETURN": "TM27", "DIG": "TM28", "PSYCHIC_M": "TM29",
        "SHADOW_BALL": "TM30", "MUD_SLAP": "TM31", "DOUBLE_TEAM": "TM32",
        "ICE_PUNCH": "TM33", "SWAGGER": "TM34", "SLEEP_TALK": "TM35",
        "SLUDGE_BOMB": "TM36", "SANDSTORM": "TM37", "FIRE_BLAST": "TM38",
        "SWIFT": "TM39", "DEFENSE_CURL": "TM40", "THUNDERPUNCH": "TM41",
        "DREAM_EATER": "TM42", "DETECT": "TM43", "REST": "TM44",
        "ATTRACT": "TM45", "THIEF": "TM46", "STEEL_WING": "TM47",
        "FIRE_PUNCH": "TM48", "FURY_CUTTER": "TM49", "NIGHTMARE": "TM50",
        "CUT": "HM01", "FLY": "HM02", "SURF": "HM03", "STRENGTH": "HM04",
        "FLASH": "HM05", "WHIRLPOOL": "HM06", "WATERFALL": "HM07",
    }

    for fpath in sorted(glob.glob(f"{BASE}/data/pokemon/base_stats/*.asm")):
        with open(fpath) as f:
            content = f.read()
        dm = re.search(r'db (\w+) ; (\d+)', content)
        if not dm:
            continue
        dex = int(dm.group(2))
        if dex > 251:
            continue
        pokemon_name = POKEMON_NAMES[dex - 1]

        tm = re.search(r'tmhm\s+(.*)', content)
        if tm:
            raw = tm.group(1).strip()
            if raw:
                for mv_const in [x.strip() for x in raw.split(",")]:
                    # Skip comments and empty entries
                    mv_const = mv_const.split(";")[0].strip()
                    if not mv_const or mv_const.startswith(";"):
                        continue
                    move = fmt_move(mv_const)
                    tm_label = tm_map.get(mv_const, "TM/HM")
                    if move not in move_learners:
                        move_learners[move] = []
                    move_learners[move].append((pokemon_name, tm_label))

    # Generate output
    lines = []
    lines.append("# Pokemon Crystal - Learnset by Move (Reverse Index)\n")
    lines.append("For every move, which Pokemon can learn it and how.\n")
    lines.append("---\n")

    for move_name in sorted(move_learners.keys()):
        learners = move_learners[move_name]
        lines.append(f"## {move_name}\n")

        # Group by method
        level_up = [(p, m) for p, m in learners if m.startswith("Level")]
        egg = [(p, m) for p, m in learners if m == "Egg Move"]
        tm_hm = [(p, m) for p, m in learners if m.startswith("TM") or m.startswith("HM")]

        if level_up:
            lines.append("**Level-Up:**")
            # Deduplicate and sort
            seen = {}
            for p, m in level_up:
                if p not in seen:
                    seen[p] = m
                else:
                    # Keep lowest level
                    existing_lvl = int(seen[p].split()[1])
                    new_lvl = int(m.split()[1])
                    if new_lvl < existing_lvl:
                        seen[p] = m
            for p in sorted(seen.keys()):
                lines.append(f"- {p} ({seen[p]})")
            lines.append("")

        if tm_hm:
            # Get TM number
            tm_nums = set(m for _, m in tm_hm)
            tm_label = ", ".join(sorted(tm_nums))
            lines.append(f"**{tm_label}:**")
            pokes = sorted(set(p for p, _ in tm_hm))
            lines.append(", ".join(pokes))
            lines.append("")

        if egg:
            lines.append("**Egg Move:**")
            pokes = sorted(set(p for p, _ in egg))
            lines.append(", ".join(pokes))
            lines.append("")

        lines.append("---\n")

    with open(f"{OUT}/species/learnset_by_move.md", "w") as f:
        f.write("\n".join(lines))
    print(f"Wrote species/learnset_by_move.md ({len(lines)} lines)")


# ═══════════════════════════════════════════
# FILE 3: Item Locations
# ═══════════════════════════════════════════

def gen_item_locations():
    lines = []
    lines.append("# Pokemon Crystal - Item Locations\n")
    lines.append("Every obtainable item and where/how to get it.\n")
    lines.append("Sources: pokecrystal marts.asm, fruit_trees.asm, map scripts, and Bulbapedia.\n")
    lines.append("---\n")

    # === Mart data ===
    lines.append("## Poke Mart Inventories\n")

    with open(f"{BASE}/data/items/marts.asm") as f:
        content = f.read()

    # Parse each mart
    marts = re.split(r'\n(Mart\w+):', content)
    for i in range(1, len(marts), 2):
        mart_name = marts[i]
        if mart_name == "Marts" or mart_name == "DefaultMart":
            continue
        mart_data = marts[i+1] if i+1 < len(marts) else ""

        display = mart_name.replace("Mart", "").replace("Dex", " (after Pokedex)")
        # Add spaces before capitals
        display = re.sub(r'([A-Z])', r' \1', display).strip()
        display = display.replace("  ", " ").replace("2 F", "2F").replace("3 F", "3F").replace("4 F", "4F").replace("5 F", "5F")

        items = []
        for line in mart_data.strip().split("\n"):
            line = line.strip()
            mm = re.match(r'db (\w+)', line)
            if mm:
                item_const = mm.group(1)
                if item_const.isdigit() or item_const == "-1":
                    continue
                items.append(fmt_item(item_const).replace("Tm ", "TM ").replace("Hm ", "HM "))

        if items:
            lines.append(f"### {display}")
            for item in items:
                lines.append(f"- {item}")
            lines.append("")

    # === Fruit Trees ===
    lines.append("## Berry Trees (Daily)\n")
    lines.append("Berries regrow daily after being picked.\n")

    fruit_trees = [
        ("Route 29", "Berry"), ("Route 30", "Berry"), ("Route 38", "Berry"),
        ("Route 46", "Berry"), ("Route 30", "PsnCureBerry"), ("Route 33", "PsnCureBerry"),
        ("Route 31", "Bitter Berry"), ("Route 43", "Bitter Berry"),
        ("Violet City", "PrzCureBerry"), ("Route 46", "PrzCureBerry"),
        ("Route 35", "MysteryBerry"), ("Route 45", "MysteryBerry"),
        ("Route 36", "Ice Berry"), ("Route 26", "Ice Berry"),
        ("Route 39", "Mint Berry"), ("Route 44", "Burnt Berry"),
        ("Route 37", "Red Apricorn"), ("Route 37", "Blu Apricorn"),
        ("Route 37", "Blk Apricorn"), ("Azalea Town", "Wht Apricorn"),
        ("Route 42", "Pnk Apricorn"), ("Route 42", "Grn Apricorn"),
        ("Route 42", "Ylw Apricorn"),
        ("Route 11 (Kanto)", "Berry"), ("Route 2 (Kanto)", "PsnCureBerry"),
        ("Route 1 (Kanto)", "Bitter Berry"), ("Route 8 (Kanto)", "PrzCureBerry"),
        ("Pewter City", "Ice Berry"), ("Pewter City", "Mint Berry"),
        ("Fuchsia City", "Burnt Berry"),
    ]
    for loc, item in fruit_trees:
        lines.append(f"- **{loc}:** {item}")
    lines.append("")

    # === Key Items ===
    lines.append("## Key Item Locations\n")
    key_items = [
        ("Town Map", "Given by rival's mom (New Bark Town) after getting your first Pokemon"),
        ("Bicycle", "Bicycle Shop in Goldenrod City (free)"),
        ("Coin Case", "Underground in Goldenrod City (trade Coin Case from the guy in the house)"),
        ("Itemfinder", "Ecruteak City (house next to Gym)"),
        ("Old Rod", "Route 32 (Fisherman on the bridge south of Violet City)"),
        ("Good Rod", "Olivine City (Fisherman in house south of Pokemon Center)"),
        ("Super Rod", "Route 12 (house on fishing peninsula, Kanto)"),
        ("SquirtBottle", "Goldenrod City flower shop (after Whitney badge)"),
        ("Red Scale", "Lake of Rage (after Red Gyarados event)"),
        ("SecretPotion", "Cianwood City pharmacy"),
        ("S.S. Ticket", "Professor Elm (after beating Elite Four)"),
        ("Mystery Egg", "Mr. Pokemon's house (Route 30) — trade Red Scale"),
        ("Clear Bell", "Radio Tower (after Team Rocket event, Crystal only)"),
        ("Silver Wing", "Radio Tower Director (after Team Rocket event)"),
        ("Rainbow Wing", "Radio Tower Director (after beating Team Rocket, Crystal only)"),
        ("GS Ball", "Kurt in Azalea Town (Crystal Japan event, Crystal Virtual Console)"),
        ("Basement Key", "Team Rocket HQ in Mahogany Town"),
        ("Card Key", "Underground Warehouse (Goldenrod Radio Tower event)"),
        ("Machine Part", "Cerulean Gym (from Team Rocket Grunt)"),
        ("Lost Item", "Vermilion City Pokemon Fan Club"),
        ("Pass", "Copycat in Saffron City (trade Lost Item)"),
        ("Exp. Share", "Mr. Pokemon's house (Route 30, trade Red Scale)"),
        ("Blue Card", "Radio Tower (after answering quiz correctly)"),
    ]
    for item, loc in key_items:
        lines.append(f"- **{item}:** {loc}")
    lines.append("")

    # === Evolution Items ===
    lines.append("## Evolution Stone / Item Locations\n")
    evo_items = [
        ("Fire Stone", "Bill's Grandfather (show Vulpix), School Boy Alan (phone gift), Pokeathalon Dome"),
        ("Water Stone", "Bill's Grandfather (show Staryu), Fisher Tully (phone gift), Pokeathalon Dome"),
        ("Thunderstone", "Bill's Grandfather (show Pichu), Dana (phone gift), Pokeathalon Dome"),
        ("Leaf Stone", "Bill's Grandfather (show Oddish), Picnicker Gina (phone gift)"),
        ("Moon Stone", "Ruins of Alph, Mt. Moon (Monday night), Mom shopping"),
        ("Sun Stone", "Bug Catching Contest (1st prize), Pokeathalon Dome"),
        ("Metal Coat", "S.S. Aqua (find on ship), Wild Magnemite (5% hold)"),
        ("Dragon Scale", "Mt. Mortar (Dragon's Den area), Wild Horsea/Seadra (rare hold)"),
        ("King's Rock", "Slowpoke Well (after clearing Team Rocket), Wild Poliwhirl (rare hold)"),
        ("Up-Grade", "Silph Co. (talk to scientist after getting National Dex in GSC)"),
    ]
    for item, loc in evo_items:
        lines.append(f"- **{item}:** {loc}")
    lines.append("")

    # === Held Items (notable) ===
    lines.append("## Notable Held Item Locations\n")
    held_items = [
        ("Leftovers", "Celadon City Restaurant (trash can), S.S. Aqua"),
        ("Quick Claw", "National Park (given by woman)"),
        ("Focus Band", "Cianwood City (Shuckle man)"),
        ("Scope Lens", "Goldenrod City (after Radio Tower event)"),
        ("BrightPowder", "Defeat Red at Mt. Silver"),
        ("King's Rock", "Slowpoke Well B2F"),
        ("Amulet Coin", "Goldenrod Underground (Basement)"),
        ("Cleanse Tag", "Tin Tower 5F"),
        ("Smoke Ball", "Burned Tower B1F"),
        ("Charcoal", "Azalea Town Charcoal Kiln (gift)"),
        ("Mystic Water", "Cherrygrove City (Mr. Pokemon's house, Crystal)"),
        ("Miracle Seed", "Route 32 (from the guy on the bridge)"),
        ("Sharp Beak", "Route 40 (trainers, hidden)"),
        ("Soft Sand", "Route 34 (hidden, beach area)"),
        ("Magnet", "Olivine City Lighthouse 5F (hidden)"),
        ("NeverMeltIce", "Ice Path B2F"),
        ("TwistedSpoon", "Route 28 (Kanto side, near Mt. Silver)"),
        ("Poison Barb", "Wild Beedrill (hold), Goldenrod Underground"),
        ("Spell Tag", "Blackthorn City (hidden in house)"),
        ("Hard Stone", "Route 36 (hidden)"),
        ("BlackBelt", "Lake of Rage (hidden)"),
        ("BlackGlasses", "Dark Cave (Blackthorn entrance)"),
        ("SilverPowder", "Wild Butterfree (hold), Bug Catching Contest"),
        ("Dragon Fang", "Dragon's Den (from Elder after Clair's badge)"),
        ("Lucky Egg", "Wild Chansey (rare hold, 5%)"),
        ("Metal Powder", "Wild Ditto (rare hold, 5%)"),
        ("Thick Club", "Wild Cubone/Marowak (rare hold, 5%)"),
        ("Light Ball", "Wild Pikachu (rare hold, 5%)"),
        ("Stick", "Wild Farfetch'd (rare hold, 5%)"),
        ("Lucky Punch", "Wild Chansey (common hold, 8%)"),
        ("Berserk Gene", "Cerulean City (hidden in destroyed area)"),
    ]
    for item, loc in held_items:
        lines.append(f"- **{item}:** {loc}")
    lines.append("")

    # === TM Locations ===
    lines.append("## TM Locations\n")
    lines.append("| TM | Move | Location |")
    lines.append("|---:|------|----------|")
    tm_locs = [
        (1, "DynamicPunch", "Cianwood Gym (Chuck)"),
        (2, "Headbutt", "Ilex Forest (find Farfetch'd), Goldenrod Dept Store 5F"),
        (3, "Curse", "Celadon Mansion (at night)"),
        (4, "Rollout", "Route 35 (from girl in gate)"),
        (5, "Roar", "Route 32 (from the man on bridge)"),
        (6, "Toxic", "Fuchsia City Gym (Janine)"),
        (7, "Zap Cannon", "Power Plant (after fixing Machine Part)"),
        (8, "Rock Smash", "Route 36 (from hiker)"),
        (9, "Psych Up", "Saffron City Gym (Sabrina, Kanto)"),
        (10, "Hidden Power", "Lake of Rage Gift Shop, Celadon Dept Store 3F"),
        (11, "Sunny Day", "Radio Tower 3F (Buena prizes), Celadon Dept Store 3F"),
        (12, "Sweet Scent", "National Park (Lass)"),
        (13, "Snore", "Route 39 (from man in house)"),
        (14, "Blizzard", "Goldenrod Game Corner Prize (5500 coins)"),
        (15, "Hyper Beam", "Goldenrod Game Corner Prize (7500 coins)"),
        (16, "Icy Wind", "Mahogany Gym (Pryce)"),
        (17, "Protect", "Celadon Dept Store 3F"),
        (18, "Rain Dance", "Slowpoke Well, Celadon Dept Store 3F"),
        (19, "Giga Drain", "Celadon Gym (Erika, Kanto)"),
        (20, "Endure", "Burned Tower B1F"),
        (21, "Frustration", "Goldenrod Dept Store 5F (Sunday)"),
        (22, "SolarBeam", "Route 27 (hidden in rocks near border)"),
        (23, "Iron Tail", "Olivine Gym (Jasmine)"),
        (24, "DragonBreath", "Blackthorn Gym (Clair)"),
        (25, "Thunder", "Goldenrod Game Corner Prize (5500 coins)"),
        (26, "Earthquake", "Victory Road"),
        (27, "Return", "Goldenrod Dept Store 5F (Sunday)"),
        (28, "Dig", "National Park (from Trainer Tips NPC)"),
        (29, "Psychic", "Saffron City (Mr. Psychic's house)"),
        (30, "Shadow Ball", "Ecruteak Gym (Morty)"),
        (31, "Mud-Slap", "Violet Gym (Falkner)"),
        (32, "Double Team", "Goldenrod Game Corner Prize (1500 coins)"),
        (33, "Ice Punch", "Goldenrod Dept Store 5F"),
        (34, "Swagger", "Olivine Lighthouse 5F"),
        (35, "Sleep Talk", "Goldenrod Underground (Basement)"),
        (36, "Sludge Bomb", "Route 43 (from shady NPC)"),
        (37, "Sandstorm", "Route 27, Celadon Dept Store 3F"),
        (38, "Fire Blast", "Goldenrod Game Corner Prize (5500 coins)"),
        (39, "Swift", "Union Cave B2F"),
        (40, "Defense Curl", "Mt. Mortar"),
        (41, "ThunderPunch", "Goldenrod Dept Store 5F"),
        (42, "Dream Eater", "Viridian City (old man's house)"),
        (43, "Detect", "Lake of Rage"),
        (44, "Rest", "Ice Path B1F"),
        (45, "Attract", "Goldenrod Gym (Whitney)"),
        (46, "Thief", "Team Rocket HQ (Mahogany Town)"),
        (47, "Steel Wing", "Route 28 (from NPC near Mt. Silver)"),
        (48, "Fire Punch", "Goldenrod Dept Store 5F"),
        (49, "Fury Cutter", "Azalea Gym (Bugsy)"),
        (50, "Nightmare", "Route 31 (NPC in house)"),
    ]
    for num, move, loc in tm_locs:
        lines.append(f"| TM{num:02d} | {move} | {loc} |")
    lines.append("")

    lines.append("## HM Locations\n")
    lines.append("| HM | Move | Location |")
    lines.append("|---:|------|----------|")
    hm_locs = [
        (1, "Cut", "Ilex Forest (from Charcoal Man after Farfetch'd event)"),
        (2, "Fly", "Cianwood City (from Chuck's wife after defeating Chuck)"),
        (3, "Surf", "Ecruteak City Dance Theater (after defeating Kimono Girls)"),
        (4, "Strength", "Olivine City Cafe"),
        (5, "Flash", "Sprout Tower (defeat Elder)"),
        (6, "Whirlpool", "Team Rocket HQ B2F (Lance gives it to you)"),
        (7, "Waterfall", "Ice Path 1F"),
    ]
    for num, move, loc in hm_locs:
        lines.append(f"| HM{num:02d} | {move} | {loc} |")
    lines.append("")

    with open(f"{OUT}/encounters/item_locations.md", "w") as f:
        f.write("\n".join(lines))
    print(f"Wrote encounters/item_locations.md ({len(lines)} lines)")


# ═══════════════════════════════════════════
# FILE 4: Phone Trainer Rematches
# ═══════════════════════════════════════════

def gen_trainer_rematches():
    lines = []
    lines.append("# Pokemon Crystal - Trainer Rematches & Phone System\n")
    lines.append("In Pokemon Crystal, gym leaders do NOT have rematches.")
    lines.append("Rematches are available from registered Phone Trainers.\n")
    lines.append("After you beat them once, exchange numbers, and they'll call when ready for a rematch.\n")
    lines.append("---\n")

    # Phone trainers with rematches (from phone_contacts.asm + engine_flags.asm)
    phone_trainers = [
        {
            "name": "Schoolboy Jack", "route": "National Park", "class": "Schoolboy",
            "initial": "Lv12 Oddish, Lv15 Voltorb",
            "rematch": "Lv30 Gloom, Lv28 Voltorb, Lv31 Electrode, Lv33 Vileplume",
            "gift": None, "time": "Anytime",
        },
        {
            "name": "Pokefan Beverly", "route": "National Park", "class": "Pokefan F",
            "initial": "Lv14 Snubbull",
            "rematch": "Lv30 Granbull",
            "gift": "Nugget", "time": "Anytime",
        },
        {
            "name": "Sailor Huey", "route": "Olivine Lighthouse", "class": "Sailor",
            "initial": "Lv13 Poliwag, Lv13 Poliwhirl",
            "rematch": "Lv28 Poliwhirl, Lv28 Poliwhirl, Lv32 Poliwrath",
            "gift": None, "time": "Anytime",
        },
        {
            "name": "Cooltrainer Gaven", "route": "Route 26", "class": "Cooltrainer M",
            "initial": "Lv39 Victreebel, Lv39 Kingler, Lv39 Flareon",
            "rematch": "Lv42 Victreebel, Lv42 Kingler, Lv45 Flareon",
            "gift": None, "time": "Anytime",
        },
        {
            "name": "Cooltrainer Beth", "route": "Route 26", "class": "Cooltrainer F",
            "initial": "Lv40 Rapidash",
            "rematch": "Lv44 Rapidash",
            "gift": None, "time": "Anytime",
        },
        {
            "name": "Bird Keeper Jose", "route": "Route 27", "class": "Bird Keeper",
            "initial": "Lv40 Fearow",
            "rematch": "Lv44 Fearow, Lv40 Dodrio",
            "gift": "Star Piece", "time": "Anytime",
        },
        {
            "name": "Youngster Joey", "route": "Route 30", "class": "Youngster",
            "initial": "Lv4 Rattata",
            "rematch": "Lv30 Raticate",
            "gift": None, "time": "Anytime",
        },
        {
            "name": "Bug Catcher Wade", "route": "Route 31", "class": "Bug Catcher",
            "initial": "Lv2 Caterpie, Lv2 Caterpie, Lv3 Caterpie, Lv4 Caterpie",
            "rematch": "Lv24 Butterfree, Lv24 Butterfree, Lv26 Butterfree, Lv28 Beedrill",
            "gift": "Berry, PsnCureBerry, Bitter Berry", "time": "Anytime",
        },
        {
            "name": "Fisher Ralph", "route": "Route 32", "class": "Fisher",
            "initial": "Lv10 Goldeen",
            "rematch": "Lv30 Seaking, Lv28 Goldeen",
            "gift": None, "time": "Anytime",
            "special": "Calls about Qwilfish swarm on Route 32",
        },
        {
            "name": "Picnicker Liz", "route": "Route 32", "class": "Picnicker",
            "initial": "Lv9 Nidoran F",
            "rematch": "Lv28 Nidorina, Lv30 Nidoqueen",
            "gift": "Thunderstone", "time": "Anytime",
        },
        {
            "name": "Hiker Anthony", "route": "Route 33", "class": "Hiker",
            "initial": "Lv11 Geodude, Lv11 Machop",
            "rematch": "Lv30 Graveler, Lv30 Machoke, Lv28 Graveler",
            "gift": None, "time": "Anytime",
            "special": "Calls about Dunsparce swarm in Dark Cave",
        },
        {
            "name": "Camper Todd", "route": "Route 34", "class": "Camper",
            "initial": "Lv14 Psyduck",
            "rematch": "Lv32 Golduck, Lv30 Psyduck",
            "gift": None, "time": "Anytime",
        },
        {
            "name": "Picnicker Gina", "route": "Route 34", "class": "Picnicker",
            "initial": "Lv14 Hoppip, Lv14 Hoppip, Lv16 Bulbasaur",
            "rematch": "Lv30 Skiploom, Lv30 Skiploom, Lv32 Ivysaur",
            "gift": "Leaf Stone", "time": "Anytime",
        },
        {
            "name": "Bug Catcher Arnie", "route": "Route 35", "class": "Bug Catcher",
            "initial": "Lv15 Venonat",
            "rematch": "Lv32 Venomoth",
            "gift": None, "time": "Anytime",
            "special": "Calls about Yanma swarm on Route 35",
        },
        {
            "name": "Schoolboy Alan", "route": "Route 36", "class": "Schoolboy",
            "initial": "Lv16 Tangela",
            "rematch": "Lv33 Tangela, Lv35 Yanma",
            "gift": "Fire Stone", "time": "Anytime",
        },
        {
            "name": "Lass Dana", "route": "Route 38", "class": "Lass",
            "initial": "Lv18 Flaaffy, Lv18 Psyduck",
            "rematch": "Lv32 Ampharos, Lv32 Golduck",
            "gift": "Thunderstone", "time": "Anytime",
        },
        {
            "name": "Schoolboy Chad", "route": "Route 38", "class": "Schoolboy",
            "initial": "Lv19 MrMime",
            "rematch": "Lv34 MrMime",
            "gift": None, "time": "Anytime",
        },
        {
            "name": "Pokefan Derek", "route": "Route 39", "class": "Pokefan M",
            "initial": "Lv17 Pikachu",
            "rematch": "Lv36 Pikachu",
            "gift": "Nugget", "time": "Anytime",
        },
        {
            "name": "Fisher Tully", "route": "Route 42", "class": "Fisher",
            "initial": "Lv19 Qwilfish",
            "rematch": "Lv34 Qwilfish",
            "gift": "Water Stone", "time": "Anytime",
        },
        {
            "name": "Pokemaniac Brent", "route": "Route 43", "class": "Pokemaniac",
            "initial": "Lv24 Lickitung",
            "rematch": "Lv36 Lickitung",
            "gift": None, "time": "Anytime",
        },
        {
            "name": "Picnicker Tiffany", "route": "Route 43", "class": "Picnicker",
            "initial": "Lv24 Clefairy",
            "rematch": "Lv38 Clefable",
            "gift": "Pink Bow", "time": "Anytime",
        },
        {
            "name": "Bird Keeper Vance", "route": "Route 44", "class": "Bird Keeper",
            "initial": "Lv25 Pidgeotto, Lv25 Pidgeotto",
            "rematch": "Lv36 Pidgeot, Lv32 Pidgeotto, Lv32 Pidgeotto",
            "gift": None, "time": "Anytime",
        },
        {
            "name": "Fisher Wilton", "route": "Route 44", "class": "Fisher",
            "initial": "Lv23 Goldeen, Lv24 Goldeen, Lv25 Seaking",
            "rematch": "Lv36 Seaking, Lv34 Goldeen, Lv34 Goldeen",
            "gift": None, "time": "Anytime",
            "special": "Calls about Remoraid swarm on Route 44",
        },
        {
            "name": "Blackbelt Kenji", "route": "Route 45", "class": "Blackbelt",
            "initial": "Lv28 Machoke",
            "rematch": "Lv36 Machoke, Lv38 Machamp",
            "gift": None, "time": "Anytime",
        },
        {
            "name": "Hiker Parry", "route": "Route 45", "class": "Hiker",
            "initial": "Lv28 Onix",
            "rematch": "Lv35 Steelix, Lv33 Onix, Lv33 Onix",
            "gift": None, "time": "Anytime",
            "special": "Calls about Marill swarm on Mt. Mortar",
        },
        {
            "name": "Picnicker Erin", "route": "Route 46", "class": "Picnicker",
            "initial": "Lv26 Ponyta, Lv26 Ponyta",
            "rematch": "Lv34 Rapidash, Lv32 Ponyta, Lv32 Ponyta",
            "gift": None, "time": "Anytime",
        },
    ]

    lines.append("## Phone Trainers (Rematchable)\n")
    lines.append("After defeating each trainer and exchanging phone numbers, they may call for rematches.\n")

    for t in phone_trainers:
        lines.append(f"### {t['name']}")
        lines.append(f"**Location:** {t['route']}  ")
        lines.append(f"**Available:** {t['time']}\n")
        lines.append(f"**Initial Team:** {t['initial']}  ")
        lines.append(f"**Rematch Team:** {t['rematch']}")
        if t.get("gift"):
            lines.append(f"**Gift Items:** {t['gift']}")
        if t.get("special"):
            lines.append(f"**Special:** {t['special']}")
        lines.append("")

    # Swarm info
    lines.append("## Swarm Notifications\n")
    lines.append("Certain phone trainers will call to report swarms of rare Pokemon:\n")
    swarms = [
        ("Hiker Anthony", "Route 33", "Dunsparce in Dark Cave"),
        ("Bug Catcher Arnie", "Route 35", "Yanma on Route 35"),
        ("Fisher Ralph", "Route 32", "Qwilfish on Route 32 (fishing)"),
        ("Fisher Wilton", "Route 44", "Remoraid on Route 44 (fishing)"),
        ("Hiker Parry", "Route 45", "Marill on Mt. Mortar (surfing)"),
    ]
    for trainer, route, swarm in swarms:
        lines.append(f"- **{trainer}** ({route}): {swarm}")
    lines.append("")

    # Gift items from phone
    lines.append("## Phone Gift Items\n")
    lines.append("Some trainers will call to give you items:\n")
    gifts = [
        ("Pokefan Beverly", "Nugget"),
        ("Pokefan Derek", "Nugget"),
        ("Bug Catcher Wade", "Berry, PsnCureBerry, Bitter Berry"),
        ("Lass Dana", "Thunderstone"),
        ("Picnicker Liz", "Thunderstone"),
        ("Schoolboy Alan", "Fire Stone"),
        ("Picnicker Gina", "Leaf Stone"),
        ("Fisher Tully", "Water Stone"),
        ("Bird Keeper Jose", "Star Piece"),
        ("Picnicker Tiffany", "Pink Bow"),
    ]
    for trainer, item in gifts:
        lines.append(f"- **{trainer}:** {item}")
    lines.append("")

    # Gym leader schedules (Crystal only: they give you their phone number for rematches... no, actually they DON'T)
    lines.append("## Gym Leader Phone Numbers\n")
    lines.append("In Pokemon Crystal, you CAN get gym leader phone numbers but they do NOT offer rematches.")
    lines.append("They only provide general tips and occasionally mention where rare Pokemon appear.\n")
    gym_phones = [
        ("Falkner", "After getting all 8 badges, talk to him in Violet Gym (Saturday morning)"),
        ("Bugsy", "After getting all 8 badges, talk to him in Azalea Gym (Thursday)"),
        ("Whitney", "After getting all 8 badges, talk to her in Goldenrod Dept Store 6F (Saturday afternoon)"),
        ("Morty", "After getting all 8 badges, talk to him in Burned Tower 1F (Tuesday night)"),
        ("Chuck", "After getting all 8 badges, talk to his wife outside Cianwood Gym (Wednesday night)"),
        ("Jasmine", "After getting all 8 badges, talk to her in Olivine Cafe (every day)"),
        ("Pryce", "After getting all 8 badges, talk to him near Lake of Rage (every morning)"),
        ("Clair", "After getting all 8 badges, talk to her in Dragon's Den (every day)"),
    ]
    for leader, when in gym_phones:
        lines.append(f"- **{leader}:** {when}")
    lines.append("")

    with open(f"{OUT}/trainers/gym_leader_rematches.md", "w") as f:
        f.write("\n".join(lines))
    print(f"Wrote trainers/gym_leader_rematches.md ({len(lines)} lines)")


# ═══════════════════════════════════════════
# FILE 5: Competitive Pokemon (GSC Metagame)
# ═══════════════════════════════════════════

def gen_competitive_pokemon():
    lines = []
    lines.append("# Pokemon Crystal - Competitive Pokemon Guide (GSC Metagame)\n")
    lines.append("Top competitive Pokemon for Gen 2 OU/UU, sourced from Smogon GSC analysis.\n")
    lines.append("Tiers: Uber > OU > BL > UU > NU. Pokemon Crystal uses Gen 2 mechanics.\n")
    lines.append("---\n")

    pokemon_data = [
        {
            "name": "Snorlax", "dex": 143, "tier": "OU",
            "role": "Special Wall / Tank / CurseLax",
            "stats": "160/110/65/30/65/110",
            "sets": [
                "CurseLax: Curse / Rest / Sleep Talk / Body Slam or Double-Edge",
                "MixLax: Body Slam / Earthquake / Fire Blast or Flamethrower / Rest or Self-Destruct",
                "DrumLax: Belly Drum / Body Slam / Earthquake / Rest",
            ],
            "why": "The king of GSC OU. Snorlax's enormous HP and Special Defense make it the premier special tank. CurseLax is the most dangerous late-game sweeper. It fits on virtually every team.",
            "counters": "Skarmory walls physical sets. Misdreavus blocks Body Slam. Machamp with Cross Chop. Tyranitar can trade blows. Roar/Whirlwind users to phaze CurseLax.",
        },
        {
            "name": "Tyranitar", "dex": 248, "tier": "OU",
            "role": "Mixed Attacker / Pursuit Trapper",
            "stats": "100/134/110/61/95/100",
            "sets": [
                "MixTar: Rock Slide / Fire Blast / DynamicPunch or Earthquake / Pursuit",
                "CurseTar: Curse / Rock Slide / Earthquake / Rest",
            ],
            "why": "Massive 600 BST with excellent mixed attacking stats. Pursuit traps Gengar and Starmie. DynamicPunch always confuses. Sets Sandstorm with its mere presence (not literally - no Sand Stream yet).",
            "counters": "Machamp, Heracross, Marowak, Steelix. 4x weak to Fighting. Water-types like Suicune and Vaporeon.",
        },
        {
            "name": "Zapdos", "dex": 145, "tier": "OU",
            "role": "Special Attacker / RestTalker",
            "stats": "90/90/85/100/125/90",
            "sets": [
                "RestTalk: Thunder / Hidden Power Ice / Rest / Sleep Talk",
                "Offensive: Thunderbolt / Hidden Power Ice or Water / Thunder Wave / Rest",
            ],
            "why": "Best Electric-type in GSC. Excellent typing with only two weaknesses. Great bulk and power. Threatens Waters and Skarmory.",
            "counters": "Snorlax (with high SpDef), Raikou, Blissey, Ground-types like Rhydon/Golem (immune to Thunder).",
        },
        {
            "name": "Skarmory", "dex": 227, "tier": "OU",
            "role": "Physical Wall / Spiker / Phazer",
            "stats": "65/80/140/70/40/70",
            "sets": [
                "Spiker: Spikes / Whirlwind / Drill Peck / Rest or Toxic",
                "CursKarm: Curse / Drill Peck / Rest / Sleep Talk",
            ],
            "why": "The premier physical wall. Immune to Ground, resists Normal. Spikes + Whirlwind is incredibly effective for passive damage. Walls Snorlax, Marowak, and physical attackers.",
            "counters": "Zapdos, Raikou (4x weak to Electric). Fire-types like Charizard. Special attackers bypass its pitiful SpDef.",
        },
        {
            "name": "Starmie", "dex": 121, "tier": "OU",
            "role": "Rapid Spinner / Special Attacker",
            "stats": "60/75/85/115/100/85",
            "sets": [
                "Spinner: Rapid Spin / Surf / Psychic or Thunderbolt / Recover",
                "Offensive: Surf / Thunderbolt / Psychic / Recover",
            ],
            "why": "The best Rapid Spinner in GSC. Blazing speed. Good coverage with Surf/Thunderbolt/Psychic. Recover for sustainability. Vital for removing Spikes.",
            "counters": "Pursuit Tyranitar traps it. Snorlax tanks hits. Blissey. Dark-types immune to Psychic.",
        },
        {
            "name": "Suicune", "dex": 245, "tier": "OU",
            "role": "Bulky Water / CroSuicune",
            "stats": "100/75/115/85/90/115",
            "sets": [
                "CroSuicune: Surf / Ice Beam / Rest / Sleep Talk",
                "Roar: Surf / Roar / Toxic / Rest",
            ],
            "why": "Extreme physical and special bulk. RestTalk makes it very hard to kill. Roar variant is a great phazer that racks up Spikes damage.",
            "counters": "Zapdos, Raikou (Electric attacks). Vaporeon with Water Absorb. Snorlax can outlast. Grass-types.",
        },
        {
            "name": "Exeggutor", "dex": 103, "tier": "OU",
            "role": "Sleep Lead / Special Attacker / Exploder",
            "stats": "95/95/85/55/125/65",
            "sets": [
                "Standard: Sleep Powder / Psychic / Giga Drain or Hidden Power Fire / Explosion",
                "Stun Spore: Stun Spore / Sleep Powder / Psychic / Explosion",
            ],
            "why": "One of the best sleep leads. Sleep Powder + Explosion makes it a potent team disruptor. Strong Psychic STAB. Can catch Steel-types with HP Fire.",
            "counters": "Snorlax (huge SpDef), Blissey. Heracross resists Psychic and threatens. Skarmory walls non-HP Fire sets.",
        },
        {
            "name": "Gengar", "dex": 94, "tier": "OU",
            "role": "Special Attacker / Spinblocker / Exploder",
            "stats": "60/65/60/110/130/75",
            "sets": [
                "Standard: Thunderbolt / Ice Punch / Hypnosis / Explosion",
                "Perish Trapper: Mean Look / Perish Song / Protect / Destiny Bond",
            ],
            "why": "Ghost-typing blocks Rapid Spin, preserving Spikes. Levitate-like immunity to Ground (Levitate) and Normal immunity. 130 SpAtk hits hard. Hypnosis and Explosion provide disruption.",
            "counters": "Pursuit Tyranitar traps and KOs. Snorlax tanks special hits. Blissey. Dark-types immune to STAB.",
        },
        {
            "name": "Machamp", "dex": 68, "tier": "OU",
            "role": "Physical Attacker / Snorlax Counter",
            "stats": "90/130/80/55/65/85",
            "sets": [
                "Standard: Cross Chop / Rock Slide / Earthquake / Hidden Power Bug or Fire Blast",
                "RestTalk: Cross Chop / Rock Slide / Rest / Sleep Talk",
                "CurseChamp: Curse / Cross Chop / Rock Slide / Rest",
            ],
            "why": "The premier Snorlax counter. Cross Chop 2HKOs most Snorlax. Excellent coverage. Curse sets are potent late-game.",
            "counters": "Skarmory walls physical attacks. Starmie outspeeds and hits with Psychic/Surf. Gengar immune to Cross Chop.",
        },
        {
            "name": "Raikou", "dex": 243, "tier": "OU",
            "role": "Special Attacker / RestTalker",
            "stats": "90/85/75/115/115/100",
            "sets": [
                "RestTalk: Thunder / Crunch / Rest / Sleep Talk",
                "Offensive: Thunderbolt / Crunch / Hidden Power Ice or Water / Rest",
            ],
            "why": "Fastest relevant Electric-type. 115 SpAtk + 115 Speed is devastating. Crunch covers Psychics and Ghosts. RestTalk keeps it healthy.",
            "counters": "Snorlax (great SpDef), Blissey, Ground-types (immune to Electric), Rhydon, Steelix.",
        },
        {
            "name": "Marowak", "dex": 105, "tier": "OU",
            "role": "Physical Wallbreaker",
            "stats": "60/80/110/45/50/80",
            "sets": [
                "Swords Dance: Swords Dance / Earthquake / Rock Slide / Hidden Power Bug",
                "Standard: Earthquake / Rock Slide / Hidden Power Bug / Rest or Swords Dance",
            ],
            "why": "With Thick Club, Marowak has the highest effective Attack in GSC. Earthquake + Rock Slide has perfect neutral coverage. Can 2HKO almost anything after Swords Dance.",
            "counters": "Skarmory (immune to EQ, resists Rock Slide). Exeggutor. Suicune. Starmie outspeeds. Must be careful of its poor Speed and Special bulk.",
        },
        {
            "name": "Blissey", "dex": 242, "tier": "OU",
            "role": "Special Wall",
            "stats": "255/10/10/55/75/135",
            "sets": [
                "Support: Softboiled / Heal Bell / Toxic / Light Screen or Seismic Toss",
                "Counter: Softboiled / Heal Bell / Counter / Toxic",
            ],
            "why": "Literally cannot be KOed by special attacks thanks to 255 HP and 135 SpDef. Heal Bell cures team status. Counter punishes physical attackers who try to exploit 10 Defense.",
            "counters": "Any physical attacker: Machamp, Marowak, Snorlax with Curse. Toxic stalling.",
        },
        {
            "name": "Cloyster", "dex": 91, "tier": "OU",
            "role": "Spiker / Physical Wall",
            "stats": "50/95/180/70/85/45",
            "sets": [
                "Spiker: Spikes / Surf / Explosion / Rapid Spin or Toxic",
            ],
            "why": "180 Defense is insane. Reliable Spikes setter. Explosion is devastating. Surf for STAB. Alternative Spiker to Forretress/Skarmory.",
            "counters": "Special attacks (only 45 SpDef). Zapdos, Raikou. Starmie spins away Spikes.",
        },
        {
            "name": "Forretress", "dex": 205, "tier": "OU",
            "role": "Spiker / Rapid Spinner / Exploder",
            "stats": "75/90/140/40/60/60",
            "sets": [
                "Utility: Spikes / Rapid Spin / Explosion / Hidden Power Bug or Toxic",
            ],
            "why": "The only Pokemon that both sets and removes Spikes. Steel/Bug typing gives many resistances. Explosion for high damage on exit. Critical team support.",
            "counters": "4x weak to Fire: Charizard, Fire Blast from anything. Gengar blocks Rapid Spin. Magneton.",
        },
        {
            "name": "Heracross", "dex": 214, "tier": "OU",
            "role": "Physical Attacker / Sleep Absorber",
            "stats": "80/125/75/85/40/95",
            "sets": [
                "RestTalk: Megahorn / Earthquake or Rock Slide / Rest / Sleep Talk",
                "Swords Dance: Megahorn / Earthquake / Swords Dance / Hidden Power Rock",
            ],
            "why": "125 Attack with Megahorn is devastating. Bug/Fighting hits many types super effectively. RestTalk variant absorbs Sleep Powder from Exeggutor.",
            "counters": "Skarmory walls completely. Gengar immune to Fighting. Flying-types. Starmie outspeeds with Psychic.",
        },
        {
            "name": "Vaporeon", "dex": 134, "tier": "OU",
            "role": "Bulky Water / Growth Sweeper",
            "stats": "130/65/60/65/110/95",
            "sets": [
                "Growth: Growth / Surf / Ice Beam / Rest",
                "Support: Surf / Ice Beam / Toxic / Rest or Haze",
            ],
            "why": "Massive HP and good SpAtk. Growth boosts make it dangerous. Ice Beam hits Grass and Dragon types. Can absorb Water attacks.",
            "counters": "Zapdos, Raikou (Electric attacks). Snorlax. Blissey. Grass-types like Exeggutor.",
        },
        {
            "name": "Alakazam", "dex": 65, "tier": "OU",
            "role": "Special Sweeper",
            "stats": "55/50/45/120/135/85",
            "sets": [
                "Standard: Psychic / Thunder Punch / Fire Punch / Recover",
                "Encore: Psychic / Thunder Punch / Encore / Recover",
            ],
            "why": "120 Speed and 135 SpAtk make it one of the fastest and hardest-hitting special attackers. Elemental punches give excellent coverage.",
            "counters": "Snorlax, Blissey (huge SpDef). Tyranitar (Dark immune to Psychic). Pursuit users.",
        },
        {
            "name": "Misdreavus", "dex": 200, "tier": "OU",
            "role": "Spinblocker / Perish Trapper",
            "stats": "60/60/60/85/85/85",
            "sets": [
                "Perish Trapper: Mean Look / Perish Song / Protect / Thunderbolt or Thunder",
                "Support: Thunderbolt / Shadow Ball / Hypnosis / Pain Split",
            ],
            "why": "Key Spinblocker. Normal immunity blocks Snorlax's Body Slam. Perish Song + Mean Look traps and KOs. Can spread Hypnosis.",
            "counters": "Tyranitar (Pursuit), Houndoom (Dark STAB). Snorlax can switch on Perish Song.",
        },
        {
            "name": "Nidoking", "dex": 34, "tier": "UU",
            "role": "Mixed Attacker / Coverage Monster",
            "stats": "81/102/77/85/85/75",
            "sets": [
                "MixKing: Earthquake / Ice Beam / Thunderbolt or Thunder / Lovely Kiss",
            ],
            "why": "Incredible coverage with Earthquake/Ice Beam/Thunderbolt. Lovely Kiss puts threats to sleep. Decent mixed offenses and Speed. Poison typing gives Fighting resistance.",
            "counters": "Suicune, Starmie (outspeed, resist). Snorlax walls special attacks. Skarmory walls physical.",
        },
        {
            "name": "Espeon", "dex": 196, "tier": "UU",
            "role": "Baton Passer / Special Attacker",
            "stats": "65/65/60/110/130/95",
            "sets": [
                "Baton Pass: Growth / Baton Pass / Psychic / Morning Sun",
                "Offensive: Psychic / Hidden Power Fire / Morning Sun / Reflect",
            ],
            "why": "130 SpAtk and 110 Speed. Best Baton Passer in GSC with Growth. Morning Sun for recovery. Can threaten sweeps on its own.",
            "counters": "Tyranitar, Houndoom (Dark type). Snorlax, Blissey. Pursuit trappers. Roar/Whirlwind stops Baton Pass.",
        },
        {
            "name": "Umbreon", "dex": 197, "tier": "UU",
            "role": "Cleric / Special Wall / Mean Look Trapper",
            "stats": "95/65/110/65/60/130",
            "sets": [
                "Mean Look: Mean Look / Baton Pass / Moonlight / Charm or Toxic",
                "Cleric: Heal Bell / Toxic / Moonlight / Pursuit or Faint Attack",
            ],
            "why": "Incredible special bulk (110 Def/130 SpDef). Heal Bell support. Mean Look + Baton Pass combos well. Dark typing gives Psychic immunity.",
            "counters": "Machamp (Fighting STAB). Heracross. Can't deal damage. Taunt (not in Gen 2).",
        },
        {
            "name": "Steelix", "dex": 208, "tier": "UU",
            "role": "Physical Wall / Phazer",
            "stats": "75/85/200/30/55/65",
            "sets": [
                "Standard: Earthquake / Roar / Toxic / Rest or Explosion",
                "CurseSteelix: Curse / Earthquake / Explosion / Rest",
            ],
            "why": "200 Defense is the highest in GSC. Excellent Steel/Ground typing resists many common attacks. Roar + Spikes racks up damage.",
            "counters": "Water attacks (Surf, Hydro Pump). Ground-types. Special attackers. 4x weak to nothing but 30 Speed is crippling.",
        },
        {
            "name": "Charizard", "dex": 6, "tier": "UU",
            "role": "Mixed Attacker / Belly Drummer",
            "stats": "78/84/78/100/109/85",
            "sets": [
                "Belly Drum: Belly Drum / Fire Blast / Earthquake / Rock Slide",
                "Mixed: Fire Blast / Earthquake / Hidden Power Grass / Sunny Day",
            ],
            "why": "Belly Drum is high-risk high-reward. Fire/Flying coverage. 100 Speed and decent mixed stats. Sunny Day powers up Fire Blast. Can sweep entire teams with Belly Drum.",
            "counters": "Blissey, Snorlax tank special hits. Starmie outspeeds. Stealth Rock (not in Gen 2). Rock Slide from faster threats.",
        },
        {
            "name": "Miltank", "dex": 241, "tier": "UU",
            "role": "Cleric / Physical Tank",
            "stats": "95/80/105/100/40/70",
            "sets": [
                "CurseTank: Curse / Body Slam / Earthquake / Milk Drink or Heal Bell",
            ],
            "why": "100 Speed before Curse is excellent. Heal Bell cures team status. Milk Drink for reliable recovery. Body Slam + Earthquake covers most threats.",
            "counters": "Skarmory. Ghost-types immune to Body Slam + Earthquake. Machamp, Heracross.",
        },
        {
            "name": "Jynx", "dex": 124, "tier": "UU",
            "role": "Sleep Lead / Special Attacker",
            "stats": "65/50/35/95/115/95",
            "sets": [
                "Lead: Lovely Kiss / Ice Beam / Psychic / Dream Eater or Hidden Power Fire",
            ],
            "why": "Lovely Kiss is the best sleep move (75% accuracy). Ice Beam + Psychic is great dual STAB. 95 Speed is decent. Always opens with sleep.",
            "counters": "Extremely frail physically (35 Def). Snorlax tanks hits. Steel-types resist both STABs. Pursuit.",
        },
        {
            "name": "Scizor", "dex": 212, "tier": "UU",
            "role": "Swords Dancer / Baton Passer",
            "stats": "70/130/100/65/55/80",
            "sets": [
                "Swords Dance: Swords Dance / Steel Wing / Hidden Power Bug / Baton Pass",
            ],
            "why": "130 Attack with Swords Dance. Steel/Bug typing has many resistances. Baton Pass passes SD boosts. Only weak to Fire.",
            "counters": "4x weak to Fire. Skarmory. Zapdos. Any Fire move KOs.",
        },
        {
            "name": "Mewtwo", "dex": 150, "tier": "Uber",
            "role": "Special Sweeper / Stallbreaker",
            "stats": "106/110/90/130/154/90",
            "sets": [
                "Standard: Psychic / Ice Beam or Shadow Ball / Flamethrower or Thunder / Recover",
                "CurseTwo: Curse / Psychic / Shadow Ball / Recover",
            ],
            "why": "The most powerful Pokemon in Gen 2. 154 SpAtk and 130 Speed are unmatched. Huge movepool covers everything. Can go physical with Curse. Banned from standard play.",
            "counters": "Blissey, Snorlax (huge SpDef). Dark-types (immune to Psychic). Mewtwo mirror.",
        },
        {
            "name": "Lugia", "dex": 249, "tier": "Uber",
            "role": "Bulky Support / CurseLugia",
            "stats": "106/90/130/110/90/154",
            "sets": [
                "CurseLugia: Curse / Aeroblast / Earthquake / Recover",
                "Support: Aeroblast / Psychic / Rest / Whirlwind",
            ],
            "why": "Insane 154 SpDef and 130 Def make it nearly unkillable. Curse + Aeroblast + Earthquake has excellent coverage. Recover for longevity. The ultimate tank.",
            "counters": "Tyranitar (Rock + Dark), Raikou (Electric), Zapdos. Roar/Whirlwind to phaze Curse boosts.",
        },
        {
            "name": "Ho-Oh", "dex": 250, "tier": "Uber",
            "role": "Mixed Attacker / Sacred Fire",
            "stats": "106/130/90/90/110/154",
            "sets": [
                "Standard: Sacred Fire / Earthquake / Recover / Hidden Power Rock or Curse",
            ],
            "why": "Sacred Fire has 50% burn rate, crippling physical attackers. 130 Attack and 110 SpAtk. 154 SpDef tanks special hits. Incredible power and bulk.",
            "counters": "Tyranitar (Rock resists Fire, STAB Rock Slide). Blissey. Water-types. Rock Slide from anything fast.",
        },
    ]

    for p in pokemon_data:
        lines.append(f"## #{p['dex']:03d} {p['name']} [{p['tier']}]\n")
        lines.append(f"**Role:** {p['role']}  ")
        lines.append(f"**Stats:** {p['stats']} (HP/Atk/Def/Spd/SpAtk/SpDef)\n")

        lines.append("### Recommended Sets\n")
        for s in p["sets"]:
            lines.append(f"- **{s}**")
        lines.append("")

        lines.append(f"### Why It's Good\n{p['why']}\n")
        lines.append(f"### Counters\n{p['counters']}\n")
        lines.append("---\n")

    # Tier summary
    lines.append("## GSC Tier Summary\n")
    lines.append("### Ubers (Banned)")
    lines.append("Mewtwo, Lugia, Ho-Oh, Celebi, Mew\n")
    lines.append("### OU (Standard)")
    lines.append("Snorlax, Tyranitar, Zapdos, Skarmory, Starmie, Suicune, Exeggutor, Gengar, Machamp, Raikou, Marowak, Blissey, Cloyster, Forretress, Heracross, Vaporeon, Alakazam, Misdreavus\n")
    lines.append("### BL (Borderline)")
    lines.append("Dragonite, Jolteon, Tentacruel, Porygon2, Smeargle\n")
    lines.append("### UU")
    lines.append("Nidoking, Espeon, Umbreon, Steelix, Charizard, Miltank, Jynx, Scizor, Kangaskhan, Electabuzz, Mr. Mime, Qwilfish, Quagsire, Hypno, Granbull, Piloswine\n")

    with open(f"{OUT}/species/competitive_pokemon.md", "w") as f:
        f.write("\n".join(lines))
    print(f"Wrote species/competitive_pokemon.md ({len(lines)} lines)")


# ═══════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════

if __name__ == "__main__":
    print("Generating supplementary data files...\n")
    gen_evolution_chains()
    gen_learnset_by_move()
    gen_item_locations()
    gen_trainer_rematches()
    gen_competitive_pokemon()
    print("\nAll supplementary files generated!")
