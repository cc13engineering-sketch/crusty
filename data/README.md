# Pokemon Crystal Knowledge Base

Complete knowledge base for Pokemon Crystal (Gen II), extracted from the [pokecrystal disassembly](https://github.com/pret/pokecrystal) and cross-referenced against the original ASM source code. Designed for semantic search via MCP local RAG.

---

## MCP RAG Setup

This folder is the `DOCS_PATH` for the `mcp-local-rag` server. All `.md` files in subdirectories (except `_pipeline/`) are game knowledge ready for ingestion.

### Register the MCP server

Add to `.mcp.json` in your project root:
```json
{
  "mcpServers": {
    "pokemon-rag": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "mcp-local-rag"],
      "env": {
        "DOCS_PATH": "./data"
      }
    }
  }
}
```

Or via CLI:
```bash
claude mcp add --transport stdio pokemon-rag -- npx -y mcp-local-rag
```

### First use: ingest all files
```
Use the ingest_file tool to ingest all .md files in data/ subdirectories.
Skip files in data/_pipeline/ — those are build artifacts, not game knowledge.
```

### Querying
Use `query_documents` for any Pokemon Crystal question. The data is semantic — keyword search won't work well. Examples:
- "What level does Typhlosion learn Flamethrower?"
- "How does the Gen 2 damage formula work?"
- "What's the best team for Whitney's Miltank?"
- "What happens when Perish Song count hits 0 for both Pokemon?"

---

## Directory Structure

```
data/
├── README.md                          <- This file
│
├── species/                           <- Pokemon data (6 files)
│   ├── all_pokemon.md                 <- All 251 species: stats, types, learnsets, evolutions, egg moves
│   ├── evolution_chains.md            <- All evolution families with methods
│   ├── learnset_by_move.md            <- Reverse index: move -> which Pokemon learn it
│   ├── competitive_pokemon.md         <- Top 50 competitive Pokemon (Smogon GSC tiers)
│   ├── pokemon_trivia.md              <- Design origins, quirks, version exclusives
│   └── faq.md                         <- 117 Q&A: "How do I get Eevee?", etc.
│
├── moves/                             <- Move data (2 files)
│   ├── all_moves.md                   <- All 251 moves: power, type, accuracy, PP, effects
│   └── move_details.md               <- Priority values, crit moves, power tables, exceptions
│
├── items/                             <- Item data (2 files)
│   ├── all_items.md                   <- All items: prices, effects, pockets, TM list
│   └── item_details.md               <- Descriptions, healing values, special shops, catch modifiers
│
├── types/                             <- Type data (1 file)
│   └── type_chart.md                  <- Full 17x17 type effectiveness chart
│
├── trainers/                          <- Trainer data (2 files)
│   ├── all_trainers.md                <- Every trainer party (gym leaders, E4, rival, route trainers)
│   └── gym_leader_rematches.md        <- Phone rematch system and schedules
│
├── encounters/                        <- Encounter data (4 files)
│   ├── wild_encounters.md             <- All wild Pokemon by route/area/time-of-day
│   ├── special_encounters.md          <- Legendaries, gift Pokemon, in-game trades
│   ├── item_locations.md              <- Every obtainable item and where to find it
│   └── battle_tower.md               <- All 70 BT trainers, 210 Pokemon builds
│
├── mechanics/                         <- Game mechanics (44 files)
│   ├── damage_formula.md              <- Step-by-step damage calc with every modifier
│   ├── stat_calculation.md            <- Stats, DVs, stat exp, stat stages
│   ├── battle_mechanics.md            <- Turn order, priority, switching, exp formula
│   ├── battle_core_details.md         <- Full battle engine from core.asm
│   ├── battle_scenarios.md            <- 36 specific scenarios with exact outcomes
│   ├── effect_commands_reference.md   <- Every BattleCommand in execution order
│   ├── status_effects.md              <- Sleep, para, burn, freeze, toxic, confusion
│   ├── move_edge_cases.md             <- Baton Pass, Counter, Future Sight, Hidden Power
│   ├── move_effects_complete.md       <- All 58 move effect ASM files documented
│   ├── formulas_reference.md          <- Every formula: catch rate, exp, happiness, flee
│   ├── ai_behavior.md                 <- Trainer AI scoring layers
│   ├── ai_scoring_details.md          <- Exact AI scoring numbers from scoring.asm
│   ├── ai_data_tables.md              <- Stat multipliers, crit chances, weather modifiers
│   ├── breeding_mechanics.md          <- Egg groups, DV inheritance, shiny breeding
│   ├── evolution_mechanics.md         <- All evolution methods and happiness system
│   ├── bugs_and_glitches.md           <- All 92 known bugs (from pokecrystal docs)
│   ├── design_flaws.md                <- 10 documented design flaws
│   ├── implementation_gotchas.md      <- 55 recreation pitfalls with ASM citations
│   ├── rng_mechanics.md               <- Hardware RNG vs battle PRNG
│   ├── state_machine.md               <- Battle/overworld/menu state machines
│   ├── overworld_engine.md            <- Player movement, collision, wild encounters
│   ├── event_engine.md                <- Bug Contest, Battle Tower, Pokerus
│   ├── event_data_tables.md           <- Happiness, trades, odd eggs, Magikarp lengths
│   ├── time_system.md                 <- Time-of-day, day-of-week events
│   ├── rtc_engine.md                  <- Real-time clock hardware, DST
│   ├── item_use_mechanics.md          <- How each item works when used
│   ├── menu_systems.md                <- Start menu, options, save system
│   ├── mini_games.md                  <- Slot Machine odds, Card Flip, Unown Puzzle
│   ├── pokegear.md                    <- Map/Radio/Phone cards
│   ├── radio_system.md                <- 11 radio stations
│   ├── phone_engine.md                <- Call scheduling, rematches
│   ├── link_battle.md                 <- Link rules, Time Capsule, Stadium 2
│   ├── link_protocol.md               <- Serial protocol, trade engine, Mystery Gift
│   ├── pokemon_management.md          <- Bill's PC, move learning, party management
│   ├── pokedex_system.md              <- Pokedex modes, search, Oak's rating
│   ├── collision_and_tilesets.md       <- Collision tiles, tileset properties
│   ├── sprites_and_animations.md      <- 102 overworld sprites, animation system
│   ├── graphics_and_cutscenes.md      <- Intro, credits, battle anims
│   ├── game_constants.md              <- All game constants by category
│   ├── game_text_strings.md           <- All battle messages and common text
│   ├── memory_map.md                  <- Complete WRAM/HRAM/SRAM/VRAM layout
│   ├── core_routines.md               <- Key home/ routines (RNG, math, text)
│   ├── scripting_commands.md          <- Event, movement, text, music commands
│   ├── mystery_gift_decorations.md    <- Mystery Gift and decoration system
│   └── printer_system.md             <- Game Boy Printer protocol
│
├── maps/                              <- World/location data (11 files)
│   ├── johto_locations.md             <- All Johto cities, routes, dungeons
│   ├── johto_map_scripts.md           <- Johto map scripts: NPCs, warps, events
│   ├── kanto_locations.md             <- All Kanto locations
│   ├── kanto_map_scripts.md           <- Kanto map scripts
│   ├── game_progression.md            <- Complete walkthrough
│   ├── story_events_by_map.md         <- 24 story events with triggers and flags
│   ├── hidden_items_complete.md       <- All 84 hidden items with coordinates
│   ├── warp_connections.md            <- All 1,312 warp events
│   ├── npc_dialogue.md                <- Move tutors, gift Pokemon, key NPCs
│   ├── phone_call_content.md          <- Phone call text by trainer
│   ├── music_and_sound.md             <- All 103 music tracks, SFX, audio engine
│   └── ruins_of_alph.md              <- Puzzles, Unown forms, hidden messages
│
├── strategy/                          <- Strategy guides (5 files)
│   ├── gym_strategies.md              <- All 16 gym leaders: teams, weaknesses, counters
│   ├── elite_four_strategies.md       <- E4 + Champion Lance
│   ├── team_building.md               <- Team composition, availability by gym
│   ├── speedrun_notes.md              <- Any% Glitchless routing
│   └── competitive_gen2.md            <- Smogon tiers, team archetypes
│
├── meta/                              <- Evaluation data
│   └── qa_pairs.md                    <- 205 Q&A pairs for retrieval testing
│
└── _pipeline/                         <- Build artifacts (NOT for RAG ingestion)
    ├── chunks/                        <- Pre-computed JSONL chunks (3,130 chunks)
    ├── schema/                        <- Vector DB schema, taxonomy, ingestion pipeline
    ├── parse_pokecrystal.py           <- ASM data extraction script
    ├── parse_supplementary.py         <- Supplementary data extraction
    ├── embedding_prep.md              <- Embedding instructions
    ├── optimization_guide.md          <- LLM optimization notes
    └── quality_audit.md              <- Dataset accuracy audit
```

---

## Coverage

| Domain | Files | What's Covered |
|--------|-------|----------------|
| Species | 6 | All 251 Pokemon: stats, learnsets, evolutions, egg moves, competitive, trivia, FAQ |
| Moves | 2 | All 251 moves + priority, crit, power tables, descriptions |
| Items | 2 | All items + healing values, catch modifiers, special shops |
| Types | 1 | Full 17x17 type chart |
| Trainers | 2 | Every trainer party + phone rematch system |
| Encounters | 4 | Wild, legendaries, gifts, trades, Battle Tower (210 builds) |
| Mechanics | 44 | Complete engine: damage, battle, AI, breeding, RNG, overworld, events, menus, link, graphics, memory map, all 92 bugs |
| Maps | 11 | All Johto + Kanto: locations, scripts, 1,312 warps, 84 hidden items, story events, music |
| Strategy | 5 | Gym strategies, E4, team building, speedrun, competitive |
| Evaluation | 1 | 205 Q&A pairs |

**81 markdown files | ~52,000 lines | All sourced from pokecrystal ASM**

---

## Data Source

All data extracted from [pret/pokecrystal](https://github.com/pret/pokecrystal) — the fully decompiled Pokemon Crystal ROM. Local copy at `engine/crates/engine-core/src/pokemon/pokecrystal-master/`. Strategy content supplemented from Smogon.

---

## Alternative: Custom JSONL Pipeline

The `_pipeline/` directory contains a custom chunking pipeline if you prefer manual vector DB management:
```bash
python data/_pipeline/schema/ingestion_pipeline.py           # Generate JSONL
python data/_pipeline/schema/ingestion_pipeline.py --validate # Validate
```
See `_pipeline/schema/retrieval_config.md` for search strategy and `_pipeline/optimization_guide.md` for embedding model recommendations.
