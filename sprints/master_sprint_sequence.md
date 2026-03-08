# Master Sprint Sequence — Crusty Pokemon Crystal Clone
# Total: 264 sprints (176 work + 88 QA)
# QA sprints: every 3rd (3, 6, 9, ...)
# Data source: /data/ folder (81 .md files, ~52K lines)
# Design principle: foundations -> core systems -> content -> polish -> integration

# ============================================================
# PHASE 1: DATA FOUNDATIONS (Sprints 1-15)
# ============================================================

## Sprint 1
goal: Define all 17 types and the full 17x17 type effectiveness chart with immunities and special mechanics (Foresight ghost bypass, sandstorm typing)
sources: type_chart.md
tags: foundation, types

## Sprint 2
goal: Build game constants system — all battle constants, stat limits, level caps, type effectiveness scaling, shiny DVs, priority values
sources: game_constants.md
tags: foundation, constants

## Sprint 3 [QA]
goal: QA audit of sprints 1-2 — verify type chart correctness, constant values against pokecrystal ASM, boundary conditions
sources: type_chart.md, game_constants.md, implementation_gotchas.md
tags: qa

## Sprint 4
goal: Define all 251 species data structures — base stats, types, catch rate, base exp, growth rate, gender ratio, egg groups, hatch cycles for Pokemon #001-#125
sources: all_pokemon.md
tags: foundation, species

## Sprint 5
goal: Define species data for Pokemon #126-#251 — complete the full Pokedex of base stat blocks
sources: all_pokemon.md
tags: foundation, species

## Sprint 6 [QA]
goal: QA audit of sprints 4-5 — verify all 251 species base stats, types, catch rates match pokecrystal data
sources: all_pokemon.md, pokemon_trivia.md, faq.md
tags: qa

## Sprint 7
goal: Build all 251 move data structures — type, power, accuracy, PP, effect ID, effect chance, priority for moves #001-#125
sources: all_moves.md, move_details.md
tags: foundation, moves

## Sprint 8
goal: Complete move data for moves #126-#251 — priority values, critical hit moves, power tables, multi-hit distributions
sources: all_moves.md, move_details.md
tags: foundation, moves

## Sprint 9 [QA]
goal: QA audit of sprints 7-8 — verify all 251 moves match pokecrystal data, check priority table, crit move list, effect IDs
sources: all_moves.md, move_details.md, effect_commands_reference.md
tags: qa

## Sprint 10
goal: Build species learnsets — level-up moves, TM/HM compatibility, egg moves for all 251 Pokemon
sources: all_pokemon.md, learnset_by_move.md
tags: foundation, species, moves

## Sprint 11
goal: Build evolution chain data — all evolution methods (level, stone, trade, happiness, time-of-day) and family linkages
sources: evolution_chains.md, evolution_mechanics.md
tags: foundation, species, evolution

## Sprint 12 [QA]
goal: QA audit of sprints 10-11 — verify learnsets, TM compatibility, evolution chains, egg move legality
sources: all_pokemon.md, evolution_chains.md, learnset_by_move.md, evolution_mechanics.md
tags: qa

## Sprint 13
goal: Build all item data structures — prices, pockets, effects, TM/HM list, held item effects, type-boosting items
sources: all_items.md, item_details.md
tags: foundation, items

## Sprint 14
goal: Build memory layout and save data structures — WRAM/HRAM/SRAM layout, party struct, box struct, trainer card, options
sources: memory_map.md, game_constants.md
tags: foundation, memory

## Sprint 15 [QA]
goal: QA audit of sprints 13-14 — verify item data, pocket assignments, prices, memory layout against pokecrystal
sources: all_items.md, item_details.md, memory_map.md, game_constants.md
tags: qa

# ============================================================
# PHASE 2: CORE MATH & RNG (Sprints 16-24)
# ============================================================

## Sprint 16
goal: Implement DV system — 4 stored DVs, derived HP DV, shared Special DV, gender determination from Attack DV, shiny check
sources: stat_calculation.md, game_constants.md
tags: core, stats

## Sprint 17
goal: Implement stat calculation — HP formula, other stat formula, stat experience contribution, level-up recalculation
sources: stat_calculation.md, formulas_reference.md
tags: core, stats

## Sprint 18 [QA]
goal: QA audit of sprints 16-17 — verify stat formulas with known examples (Lv50 Mewtwo), DV edge cases, gender thresholds
sources: stat_calculation.md, formulas_reference.md, battle_scenarios.md
tags: qa

## Sprint 19
goal: Implement RNG system — hardware RNG equivalent (PRNG for recreation), battle PRNG, link-synchronized PRNG
sources: rng_mechanics.md, core_routines.md
tags: core, rng

## Sprint 20
goal: Implement stat stages — 7 stats, levels 1-13, multiplier table (25/100 to 400/100), stage application to raw stats
sources: stat_calculation.md, move_effects_complete.md, game_constants.md
tags: core, stats, battle

## Sprint 21 [QA]
goal: QA audit of sprints 19-20 — verify RNG distribution, stat stage multipliers, edge cases at min/max stages
sources: rng_mechanics.md, stat_calculation.md, formulas_reference.md, battle_scenarios.md
tags: qa

## Sprint 22
goal: Implement damage formula — phase 1 (base damage), phase 2 (physical/special split, screens), phase 5 (TruncateHL_BC), phase 6 (self-destruct)
sources: damage_formula.md, formulas_reference.md
tags: core, battle, damage

## Sprint 23
goal: Implement damage modifiers — critical hits, weather, badge boosts, STAB, type effectiveness (dual-type), damage variation (85-100%), item boosts, Thick Club/Light Ball/Metal Powder
sources: damage_formula.md, formulas_reference.md, type_chart.md
tags: core, battle, damage

## Sprint 24 [QA]
goal: QA audit of sprints 22-23 — verify damage calc with 36 battle scenarios, overflow behavior, screen interaction with crits
sources: damage_formula.md, battle_scenarios.md, formulas_reference.md, bugs_and_glitches.md
tags: qa

# ============================================================
# PHASE 3: BATTLE ENGINE CORE (Sprints 25-45)
# ============================================================

## Sprint 25
goal: Implement battle state machine — top-level flow (InitBattle -> DoBattle -> BattleTurn loop -> cleanup), per-turn reset of flags
sources: state_machine.md, battle_mechanics.md
tags: battle, state-machine

## Sprint 26
goal: Implement turn order determination — switching priority, move priority check, Quick Claw, speed comparison, speed ties, Vital Throw special case
sources: battle_mechanics.md, state_machine.md, move_details.md
tags: battle, turn-order

## Sprint 27 [QA]
goal: QA audit of sprints 25-26 — verify battle flow, turn order with priority moves, Quick Claw activation, speed tie randomness
sources: battle_mechanics.md, state_machine.md, battle_scenarios.md
tags: qa

## Sprint 28
goal: Implement pre-move checks (CheckTurn) — recharge, sleep, freeze, flinch, disable, confusion, attract, disabled move, paralysis in exact order
sources: battle_mechanics.md, battle_core_details.md, status_effects.md
tags: battle, pre-move

## Sprint 29
goal: Implement move execution pipeline — effect script system, command dispatch (DoMove), standard damaging move command sequence
sources: effect_commands_reference.md, battle_core_details.md, state_machine.md
tags: battle, move-execution

## Sprint 30 [QA]
goal: QA audit of sprints 28-29 — verify pre-move check order, sleep/freeze/para interactions, command sequence for basic moves
sources: battle_mechanics.md, battle_core_details.md, effect_commands_reference.md, battle_scenarios.md
tags: qa

## Sprint 31
goal: Implement non-volatile status effects — sleep (duration, counter, Snore/Sleep Talk bypass), freeze (thaw mechanics, self-thaw moves, fire-hit thaw)
sources: status_effects.md, move_effects_complete.md
tags: battle, status

## Sprint 32
goal: Implement remaining non-volatile statuses — paralysis (25% full para, speed quartering), burn (1/8 HP, attack halving, crit interaction), poison and toxic (escalating damage)
sources: status_effects.md, move_effects_complete.md, formulas_reference.md
tags: battle, status

## Sprint 33 [QA]
goal: QA audit of sprints 31-32 — verify all status durations, damage values, immunities (Fire can't burn, Electric can't para), toxic counter reset on switch
sources: status_effects.md, battle_scenarios.md, formulas_reference.md
tags: qa

## Sprint 34
goal: Implement volatile status conditions — confusion (2-5 turns, 50% self-hit, 40-power typeless), attract (50% immobilize), flinch, encore, disable
sources: status_effects.md, move_effects_complete.md
tags: battle, status

## Sprint 35
goal: Implement between-turn effects — residual damage order (PSN/BRN/Toxic -> Leech Seed -> Nightmare -> Curse), weather damage (sandstorm), future sight resolution
sources: state_machine.md, battle_core_details.md, status_effects.md
tags: battle, residual

## Sprint 36 [QA]
goal: QA audit of sprints 34-35 — verify confusion self-hit damage, attract check order, residual damage order, sandstorm type immunities
sources: status_effects.md, battle_scenarios.md, state_machine.md, battle_core_details.md
tags: qa

## Sprint 37
goal: Implement experience and level-up — exp formula (base exp, level scaling, trainer/wild multiplier, Lucky Egg), stat recalculation, move learning on level-up
sources: formulas_reference.md, battle_mechanics.md, all_pokemon.md
tags: battle, exp, level-up

## Sprint 38
goal: Implement switching mechanics — mid-battle switching, forced switching (Roar/Whirlwind), fainting replacement, Spikes on entry, Pursuit on switch
sources: battle_mechanics.md, battle_core_details.md, move_effects_complete.md
tags: battle, switching

## Sprint 39 [QA]
goal: QA audit of sprints 37-38 — verify exp formula, level-up move learning, switch timing, Spikes damage, Pursuit double damage on switch
sources: formulas_reference.md, battle_mechanics.md, battle_scenarios.md, move_effects_complete.md
tags: qa

## Sprint 40
goal: Implement wild battle specifics — wild encounter initiation, flee formula, catch formula (ball modifiers, status bonuses, HP factor), shiny wild Pokemon
sources: formulas_reference.md, battle_mechanics.md, overworld_engine.md
tags: battle, wild, catch

## Sprint 41
goal: Implement trainer battle framework — trainer data structure, reward money formula, rematch system, no-flee/no-catch rules, AI item usage slots
sources: all_trainers.md, battle_mechanics.md, formulas_reference.md
tags: battle, trainer

## Sprint 42 [QA]
goal: QA audit of sprints 40-41 — verify catch rate formula with various balls, flee formula, trainer money rewards, wild encounter rate
sources: formulas_reference.md, battle_scenarios.md, all_trainers.md
tags: qa

## Sprint 43
goal: Implement obedience system — OT ID check, badge-based level thresholds, disobedience behaviors (loaf, nap, use wrong move)
sources: battle_mechanics.md, battle_core_details.md
tags: battle, obedience

## Sprint 44
goal: Implement battle text system — all battle messages (used move, hit, miss, faint, status, weather, effectiveness), text buffer system
sources: game_text_strings.md, effect_commands_reference.md
tags: battle, text

## Sprint 45 [QA]
goal: QA audit of sprints 43-44 — verify obedience thresholds per badge, disobedience probability, battle message correctness and ordering
sources: battle_mechanics.md, game_text_strings.md, battle_scenarios.md
tags: qa

# ============================================================
# PHASE 4: MOVE EFFECTS — All 251 Moves (Sprints 46-78)
# ============================================================

## Sprint 46
goal: Implement status-inflicting move effects — sleep moves (Hypnosis, Sing, Sleep Powder, Lovely Kiss, Spore), poison (PoisonPowder, Toxic), burn (Will-O-Wisp analog via Fire moves)
sources: move_effects_complete.md, effect_commands_reference.md
tags: battle, move-effects

## Sprint 47
goal: Implement paralyze/freeze/confuse move effects — Thunder Wave, Stun Spore, Body Slam para chance, Blizzard/Ice Beam freeze chance, Confuse Ray, Supersonic, Swagger, Flatter
sources: move_effects_complete.md, effect_commands_reference.md
tags: battle, move-effects

## Sprint 48 [QA]
goal: QA audit of sprints 46-47 — verify status move accuracy, AI handicap (25% fail), Safeguard blocking, substitute blocking, type immunities
sources: move_effects_complete.md, battle_scenarios.md, status_effects.md
tags: qa

## Sprint 49
goal: Implement stat-raising moves — Swords Dance, Nasty Plot analog, Agility, Amnesia, Iron Defense analog, Growth, Minimize, stat stage limits
sources: move_effects_complete.md, effect_commands_reference.md
tags: battle, move-effects, stats

## Sprint 50
goal: Implement stat-lowering moves — Growl, Leer, Screech, String Shot, Charm, Sand-Attack, Flash, Cotton Spore, secondary stat drops (Acid, Psychic SpDef drop)
sources: move_effects_complete.md, effect_commands_reference.md
tags: battle, move-effects, stats

## Sprint 51 [QA]
goal: QA audit of sprints 49-50 — verify stat stage changes, Minimize flag, accuracy/evasion interaction, secondary effect chance (1/256 miss bug)
sources: move_effects_complete.md, battle_scenarios.md, bugs_and_glitches.md
tags: qa

## Sprint 52
goal: Implement multi-hit moves — DoubleSlap, Fury Swipes, Pin Missile (2-5 hit distribution: 37.5/37.5/12.5/12.5), Twineedle (2-hit + poison), Triple Kick
sources: move_effects_complete.md, move_details.md
tags: battle, move-effects

## Sprint 53
goal: Implement two-turn moves — Fly, Dig (semi-invulnerable), SolarBeam (charge + weather skip), Skull Bash, Razor Wind, Sky Attack
sources: move_effects_complete.md, effect_commands_reference.md, move_edge_cases.md
tags: battle, move-effects

## Sprint 54 [QA]
goal: QA audit of sprints 52-53 — verify multi-hit distribution, semi-invulnerable state (can be hit by specific moves), SolarBeam sun skip
sources: move_effects_complete.md, battle_scenarios.md, move_edge_cases.md
tags: qa

## Sprint 55
goal: Implement continuous/rampage moves — Rollout (5 turns, doubling damage, Defense Curl bonus), Fury Cutter (doubling), Outrage/Thrash/Petal Dance (2-3 turns + confusion)
sources: move_effects_complete.md, effect_commands_reference.md
tags: battle, move-effects

## Sprint 56
goal: Implement recoil and self-KO moves — Take Down, Double-Edge (25% recoil), Submission, Struggle, Self-Destruct/Explosion (halve defense), Destiny Bond
sources: move_effects_complete.md, effect_commands_reference.md, move_edge_cases.md
tags: battle, move-effects

## Sprint 57 [QA]
goal: QA audit of sprints 55-56 — verify Rollout damage scaling, rampage confusion, recoil calculation, Explosion defense halving, Destiny Bond timing
sources: move_effects_complete.md, battle_scenarios.md, move_edge_cases.md
tags: qa

## Sprint 58
goal: Implement HP recovery moves — Recover, Softboiled, Milk Drink, Moonlight/Morning Sun/Synthesis (weather-dependent), Rest (sleep + full heal)
sources: move_effects_complete.md, effect_commands_reference.md
tags: battle, move-effects

## Sprint 59
goal: Implement draining moves — Absorb, Mega Drain, Giga Drain (50% drain), Leech Life, Dream Eater (sleeping target only), Leech Seed (end-of-turn drain)
sources: move_effects_complete.md, effect_commands_reference.md, status_effects.md
tags: battle, move-effects

## Sprint 60 [QA]
goal: QA audit of sprints 58-59 — verify recovery amounts, weather effect on Moonlight/Synthesis, Dream Eater sleep check, Leech Seed + toxic interaction
sources: move_effects_complete.md, battle_scenarios.md, status_effects.md
tags: qa

## Sprint 61
goal: Implement protection moves — Protect, Detect (diminishing success), Endure (survive at 1 HP), Substitute (25% HP cost, blocks most moves)
sources: move_effects_complete.md, effect_commands_reference.md, move_edge_cases.md
tags: battle, move-effects

## Sprint 62
goal: Implement counter moves — Counter (reflect physical), Mirror Coat (reflect special), priority -1, damage-type checking
sources: move_effects_complete.md, move_edge_cases.md, effect_commands_reference.md
tags: battle, move-effects

## Sprint 63 [QA]
goal: QA audit of sprints 61-62 — verify Protect diminishing rate, Substitute HP threshold, Counter/Mirror Coat type checks, Endure + residual damage
sources: move_effects_complete.md, battle_scenarios.md, move_edge_cases.md, bugs_and_glitches.md
tags: qa

## Sprint 64
goal: Implement weather moves — Rain Dance, Sunny Day, Sandstorm; weather effects on damage, SolarBeam, Moonlight/Synthesis, Thunder accuracy
sources: move_effects_complete.md, damage_formula.md, formulas_reference.md
tags: battle, move-effects, weather

## Sprint 65
goal: Implement entry hazard and field moves — Spikes (1 layer, 12.5% on switch), Reflect, Light Screen (halve damage, 5 turns), Safeguard (5 turns, blocks status)
sources: move_effects_complete.md, effect_commands_reference.md
tags: battle, move-effects, field

## Sprint 66 [QA]
goal: QA audit of sprints 64-65 — verify weather durations, damage modifiers, Spikes damage on switch-in, screen overflow bug, Safeguard move list
sources: move_effects_complete.md, battle_scenarios.md, bugs_and_glitches.md
tags: qa

## Sprint 67
goal: Implement OHKO moves — Fissure, Horn Drill, Guillotine (30% accuracy, level check, fails if target faster)
sources: move_effects_complete.md, move_details.md
tags: battle, move-effects

## Sprint 68
goal: Implement fixed/variable damage moves — Seismic Toss/Night Shade (level = damage), Psywave (random 1-1.5x level), Super Fang (50% current HP), Dragon Rage (40 fixed), SonicBoom (20 fixed)
sources: move_effects_complete.md, effect_commands_reference.md
tags: battle, move-effects

## Sprint 69 [QA]
goal: QA audit of sprints 67-68 — verify OHKO accuracy/level check, fixed damage values, Psywave range, Super Fang minimum damage
sources: move_effects_complete.md, battle_scenarios.md, move_details.md
tags: qa

## Sprint 70
goal: Implement Baton Pass, Perish Song, Mean Look/Spider Web — pass/clear rules, trap mechanics, Perish count, interaction matrix
sources: move_edge_cases.md, move_effects_complete.md
tags: battle, move-effects, edge-cases

## Sprint 71
goal: Implement Sleep Talk, Snore, Hidden Power (type/power from DVs), Frustration/Return (happiness-based power), Present (random power/heal)
sources: move_edge_cases.md, move_effects_complete.md, move_details.md
tags: battle, move-effects, edge-cases

## Sprint 72 [QA]
goal: QA audit of sprints 70-71 — verify Baton Pass passing list, Perish Song + Mean Look combo, Hidden Power type formula, Sleep Talk exclusions
sources: move_edge_cases.md, battle_scenarios.md, move_effects_complete.md
tags: qa

## Sprint 73
goal: Implement copy/transform moves — Metronome (random move, exclusion list), Mimic, Sketch (permanent copy), Mirror Move, Transform
sources: move_edge_cases.md, move_effects_complete.md
tags: battle, move-effects

## Sprint 74
goal: Implement trapping/binding moves — Wrap, Bind, Fire Spin, Whirlpool, Clamp (2-5 turns, 1/16 HP per turn), Mean Look/Spider Web (prevent flee/switch)
sources: move_effects_complete.md, effect_commands_reference.md
tags: battle, move-effects

## Sprint 75 [QA]
goal: QA audit of sprints 73-74 — verify Metronome exclusion list, Sketch permanence, Transform stat copying, wrap damage and duration
sources: move_effects_complete.md, battle_scenarios.md, move_edge_cases.md
tags: qa

## Sprint 76
goal: Implement remaining special moves — Belly Drum (HP glitch), Psych Up, Conversion/Conversion2, Spite, Magnitude, Flail/Reversal, Pain Split, Curse (ghost vs non-ghost)
sources: move_effects_complete.md, move_edge_cases.md, bugs_and_glitches.md
tags: battle, move-effects

## Sprint 77
goal: Implement final move effects — Pay Day, Thief, Rapid Spin (clear hazards/traps), Future Sight (delayed damage), Lock-On/Mind Reader, Focus Energy, Beat Up, Heal Bell
sources: move_effects_complete.md, move_edge_cases.md, effect_commands_reference.md
tags: battle, move-effects

## Sprint 78 [QA]
goal: QA audit of sprints 76-77 — verify Belly Drum HP glitch, Curse ghost/non-ghost, Future Sight timing, Rapid Spin clears, all 251 moves implemented
sources: move_effects_complete.md, move_edge_cases.md, bugs_and_glitches.md, battle_scenarios.md
tags: qa

# ============================================================
# PHASE 5: AI SYSTEM (Sprints 79-90)
# ============================================================

## Sprint 79
goal: Implement AI scoring architecture — base score per move, 8 scoring layers, layer assignment per trainer class, lowest-score selection with random tiebreak
sources: ai_behavior.md, ai_scoring_details.md
tags: ai, battle

## Sprint 80
goal: Implement AI_Basic (layer 1) — dismiss redundant moves, status on statused target, Safeguard blocking, healing at full HP
sources: ai_behavior.md, ai_scoring_details.md
tags: ai, battle

## Sprint 81 [QA]
goal: QA audit of sprints 79-80 — verify scoring system, AI_Basic correctly dismisses useless moves, tiebreak randomness
sources: ai_behavior.md, ai_scoring_details.md, battle_scenarios.md
tags: qa

## Sprint 82
goal: Implement AI_Setup (layer 2) and AI_Types (layer 3) — encourage setup turn 1, type effectiveness scoring, immune move penalty
sources: ai_behavior.md, ai_scoring_details.md
tags: ai, battle

## Sprint 83
goal: Implement AI_Offensive (layer 4) and AI_Smart (layer 5) — discourage status moves, context-specific scoring (the complex one)
sources: ai_behavior.md, ai_scoring_details.md
tags: ai, battle

## Sprint 84 [QA]
goal: QA audit of sprints 82-83 — verify AI layer interactions, Smart layer context decisions, type effectiveness calculations
sources: ai_behavior.md, ai_scoring_details.md, battle_scenarios.md
tags: qa

## Sprint 85
goal: Implement AI_Cautious (layer 6), AI_Status (layer 7), AI_Risky (layer 8) — risk-averse, status-encouraging, high-risk behaviors
sources: ai_behavior.md, ai_scoring_details.md
tags: ai, battle

## Sprint 86
goal: Implement AI switching logic — when to switch, switch scoring, HP threshold, type disadvantage detection
sources: ai_behavior.md, ai_scoring_details.md
tags: ai, battle

## Sprint 87 [QA]
goal: QA audit of sprints 85-86 — verify all 8 AI layers, switching decisions, layer combinations per trainer class
sources: ai_behavior.md, ai_scoring_details.md, battle_scenarios.md
tags: qa

## Sprint 88
goal: Implement AI item usage — trainer item slots, potion thresholds, Full Restore logic, status healing prioritization
sources: ai_behavior.md, ai_scoring_details.md, ai_data_tables.md
tags: ai, battle, items

## Sprint 89
goal: Implement AI data tables — stat multiplier tables, critical hit chance tables, weather modifier tables, type matchup caching
sources: ai_data_tables.md, ai_scoring_details.md
tags: ai, battle, data

## Sprint 90 [QA]
goal: QA audit of sprints 88-89 — verify AI item usage timing, data table accuracy, full AI integration across all layers
sources: ai_behavior.md, ai_scoring_details.md, ai_data_tables.md, battle_scenarios.md
tags: qa

# ============================================================
# PHASE 6: OVERWORLD ENGINE (Sprints 91-111)
# ============================================================

## Sprint 91
goal: Implement overworld main loop — 3-state map machine (START/ENTER/HANDLE), per-frame processing, player event priority chain
sources: overworld_engine.md, state_machine.md
tags: overworld, engine

## Sprint 92
goal: Implement player movement — joypad input, 4-direction walking, running shoes equivalent, step counting, collision checking
sources: overworld_engine.md, collision_and_tilesets.md
tags: overworld, movement

## Sprint 93 [QA]
goal: QA audit of sprints 91-92 — verify overworld state transitions, movement responsiveness, collision detection accuracy
sources: overworld_engine.md, collision_and_tilesets.md, state_machine.md
tags: qa

## Sprint 94
goal: Implement collision system — tile permission types (LAND/WATER/WALL), warp tiles, ice sliding, forced movement tiles, pit/fall tiles
sources: collision_and_tilesets.md, overworld_engine.md
tags: overworld, collision

## Sprint 95
goal: Implement tileset system — tileset properties, animated tiles, palette assignments, CGB/SGB/Crystal color layouts
sources: collision_and_tilesets.md, sprites_and_animations.md, graphics_and_cutscenes.md
tags: overworld, tilesets, graphics

## Sprint 96 [QA]
goal: QA audit of sprints 94-95 — verify all collision types, warp tile behavior, tileset rendering, ice sliding mechanics
sources: collision_and_tilesets.md, overworld_engine.md, sprites_and_animations.md
tags: qa

## Sprint 97
goal: Implement sprite system — 102 overworld sprites (WALKING/STANDING/STILL types), sprite palettes, player sprites (Chris, Kris, bike, surf)
sources: sprites_and_animations.md, graphics_and_cutscenes.md
tags: overworld, sprites

## Sprint 98
goal: Implement NPC system — NPC movement types (wander, face, scripted), trainer line-of-sight, object visibility, map object handling
sources: overworld_engine.md, sprites_and_animations.md
tags: overworld, npcs

## Sprint 99 [QA]
goal: QA audit of sprints 97-98 — verify sprite rendering, NPC movement patterns, trainer sight range, object culling
sources: sprites_and_animations.md, overworld_engine.md
tags: qa

## Sprint 100
goal: Implement map connection system — cardinal direction connections between maps, border loading, seamless scrolling
sources: warp_connections.md, overworld_engine.md
tags: overworld, maps

## Sprint 101
goal: Implement warp system — all warp tile types (door, ladder, staircase, cave, carpet), warp event handling from 1,312 total warps
sources: warp_connections.md, collision_and_tilesets.md
tags: overworld, maps, warps

## Sprint 102 [QA]
goal: QA audit of sprints 100-101 — verify map connections load correctly, warp destinations match pokecrystal data, no stuck states
sources: warp_connections.md, overworld_engine.md, collision_and_tilesets.md
tags: qa

## Sprint 103
goal: Implement event scripting engine — ~140 event commands, flow control (jump/call/if), variable system, flag system, scene scripts
sources: scripting_commands.md, event_engine.md
tags: overworld, scripting

## Sprint 104
goal: Implement text and dialogue system — text commands, text speed options, text buffers, scrolling, player name insertion, line breaks
sources: scripting_commands.md, game_text_strings.md, menu_systems.md
tags: overworld, text

## Sprint 105 [QA]
goal: QA audit of sprints 103-104 — verify scripting command execution, flag set/check, text rendering, variable substitution
sources: scripting_commands.md, event_engine.md, game_text_strings.md
tags: qa

## Sprint 106
goal: Implement wild encounter system — grass/cave/water encounter checks, encounter rate, step cooldown, time-of-day encounter tables, repel
sources: overworld_engine.md, wild_encounters.md, formulas_reference.md
tags: overworld, encounters

## Sprint 107
goal: Implement fishing and headbutt tree encounters — Old Rod/Good Rod/Super Rod mechanics, headbutt tree tables, encounter probabilities
sources: wild_encounters.md, overworld_engine.md, formulas_reference.md
tags: overworld, encounters

## Sprint 108 [QA]
goal: QA audit of sprints 106-107 — verify encounter rates by area, time-of-day table switching, fishing mechanics, repel interaction
sources: wild_encounters.md, overworld_engine.md, formulas_reference.md
tags: qa

## Sprint 109
goal: Implement Surf and waterfall mechanics — HM Surf overworld usage, waterfall ascent, whirlpool passage, water collision tiles
sources: overworld_engine.md, collision_and_tilesets.md
tags: overworld, hm

## Sprint 110
goal: Implement remaining HM overworld effects — Cut (tall grass), Flash (dark caves), Strength (boulder pushing), Rock Smash, Fly (town warp)
sources: overworld_engine.md, collision_and_tilesets.md, item_use_mechanics.md
tags: overworld, hm

## Sprint 111 [QA]
goal: QA audit of sprints 109-110 — verify all HM field effects, badge requirements, boulder persistence, Fly destination list
sources: overworld_engine.md, collision_and_tilesets.md, item_use_mechanics.md
tags: qa

# ============================================================
# PHASE 7: JOHTO MAP CONTENT (Sprints 112-135)
# ============================================================

## Sprint 112
goal: Build New Bark Town + Route 29 + Cherrygrove City — maps, warps, NPCs, items, mart inventory
sources: johto_locations.md, warp_connections.md, johto_map_scripts.md
tags: maps, johto

## Sprint 113
goal: Build Route 30 + Route 31 + Violet City — Sprout Tower, Earl's Academy, gym, mart
sources: johto_locations.md, warp_connections.md, johto_map_scripts.md
tags: maps, johto

## Sprint 114 [QA]
goal: QA audit of sprints 112-113 — verify New Bark through Violet City connectivity, NPC placement, warp accuracy, mart inventories
sources: johto_locations.md, warp_connections.md, johto_map_scripts.md, game_progression.md
tags: qa

## Sprint 115
goal: Build Route 32 + Ruins of Alph exterior + Union Cave + Route 33 + Azalea Town — Slowpoke Well, Kurt's house, gym
sources: johto_locations.md, warp_connections.md, ruins_of_alph.md
tags: maps, johto

## Sprint 116
goal: Build Ilex Forest + Route 34 + Goldenrod City — Day Care, department store, Game Corner, gym, Radio Tower, underground
sources: johto_locations.md, warp_connections.md, johto_map_scripts.md
tags: maps, johto

## Sprint 117 [QA]
goal: QA audit of sprints 115-116 — verify Azalea through Goldenrod connectivity, department store floors, Game Corner, Day Care
sources: johto_locations.md, warp_connections.md, johto_map_scripts.md, game_progression.md
tags: qa

## Sprint 118
goal: Build Route 35 + National Park + Route 36 + Route 37 + Ecruteak City — Burned Tower, Tin Tower exterior, gym, Kimono Girls
sources: johto_locations.md, warp_connections.md, johto_map_scripts.md
tags: maps, johto

## Sprint 119
goal: Build Route 38 + Route 39 + Olivine City — lighthouse, gym, S.S. Aqua dock
sources: johto_locations.md, warp_connections.md, johto_map_scripts.md
tags: maps, johto

## Sprint 120 [QA]
goal: QA audit of sprints 118-119 — verify Ecruteak through Olivine connectivity, Burned Tower layout, lighthouse floors, dock warps
sources: johto_locations.md, warp_connections.md, johto_map_scripts.md, game_progression.md
tags: qa

## Sprint 121
goal: Build Route 40 + Route 41 + Cianwood City — pharmacy, gym, Suicune encounter spot
sources: johto_locations.md, warp_connections.md, johto_map_scripts.md
tags: maps, johto

## Sprint 122
goal: Build Route 42 + Mahogany Town + Route 43 + Lake of Rage — Team Rocket HQ, gym, Red Gyarados
sources: johto_locations.md, warp_connections.md, johto_map_scripts.md
tags: maps, johto

## Sprint 123 [QA]
goal: QA audit of sprints 121-122 — verify Cianwood through Lake of Rage, Rocket HQ floors, Red Gyarados encounter, gym layouts
sources: johto_locations.md, warp_connections.md, johto_map_scripts.md, game_progression.md
tags: qa

## Sprint 124
goal: Build Route 44 + Ice Path + Blackthorn City — gym, Dragon's Den, Move Deleter/Tutor
sources: johto_locations.md, warp_connections.md, johto_map_scripts.md
tags: maps, johto

## Sprint 125
goal: Build Route 45 + Route 46 + Dark Cave — connecting routes back to early game
sources: johto_locations.md, warp_connections.md, johto_map_scripts.md
tags: maps, johto

## Sprint 126 [QA]
goal: QA audit of sprints 124-125 — verify Ice Path puzzles, Blackthorn/Dragon's Den, Dark Cave connections, Route 45/46 drops
sources: johto_locations.md, warp_connections.md, johto_map_scripts.md, game_progression.md
tags: qa

## Sprint 127
goal: Build Victory Road + Indigo Plateau + Pokemon League — E4 chambers, Champion room, Hall of Fame
sources: johto_locations.md, warp_connections.md, johto_map_scripts.md
tags: maps, johto, e4

## Sprint 128
goal: Build Tin Tower + Whirl Islands — legendary encounter dungeons, multi-floor layouts, puzzle elements
sources: johto_locations.md, warp_connections.md, special_encounters.md
tags: maps, johto, dungeons

## Sprint 129 [QA]
goal: QA audit of sprints 127-128 — verify Victory Road, E4 room sequence, Tin Tower floors, Whirl Islands water maze
sources: johto_locations.md, warp_connections.md, game_progression.md, special_encounters.md
tags: qa

## Sprint 130
goal: Build Ruins of Alph interior — 4 puzzle chambers, inner chamber, 4 hidden rooms, 4 word rooms, Unown encounter unlock
sources: ruins_of_alph.md, warp_connections.md, johto_map_scripts.md
tags: maps, johto, ruins

## Sprint 131
goal: Build Mt. Mortar + Tohjo Falls + Route 26 + Route 27 — connecting dungeons, waterfalls
sources: johto_locations.md, warp_connections.md
tags: maps, johto, dungeons

## Sprint 132 [QA]
goal: QA audit of sprints 130-131 — verify Ruins of Alph puzzles and Unown unlocks, Mt. Mortar floors, Tohjo Falls route
sources: ruins_of_alph.md, johto_locations.md, warp_connections.md, game_progression.md
tags: qa

## Sprint 133
goal: Populate all Johto wild encounter tables — grass/cave/water/fishing by route, time-of-day variants, encounter rates
sources: wild_encounters.md, johto_locations.md
tags: maps, johto, encounters

## Sprint 134
goal: Populate all Johto trainer parties — route trainers, gym trainers, gym leaders, Team Rocket grunts and executives
sources: all_trainers.md, johto_locations.md
tags: maps, johto, trainers

## Sprint 135 [QA]
goal: QA audit of sprints 133-134 — verify encounter tables per route, trainer levels match progression, gym leader teams are correct
sources: wild_encounters.md, all_trainers.md, johto_locations.md, game_progression.md
tags: qa

# ============================================================
# PHASE 8: KANTO MAP CONTENT (Sprints 136-153)
# ============================================================

## Sprint 136
goal: Build Vermilion City + Route 6 + Saffron City — post-game entry point, Magnet Train station, gym
sources: kanto_locations.md, warp_connections.md, kanto_map_scripts.md
tags: maps, kanto

## Sprint 137
goal: Build Cerulean City + Route 24 + Route 25 + Power Plant — Misty sidequest, power restoration
sources: kanto_locations.md, warp_connections.md, kanto_map_scripts.md
tags: maps, kanto

## Sprint 138 [QA]
goal: QA audit of sprints 136-137 — verify Vermilion/Saffron/Cerulean connectivity, gym access, Power Plant sidequest
sources: kanto_locations.md, warp_connections.md, kanto_map_scripts.md, game_progression.md
tags: qa

## Sprint 139
goal: Build Celadon City + Route 7 + Route 8 + Lavender Town — department store, Radio Tower (Kanto), Pokemon Tower replacement
sources: kanto_locations.md, warp_connections.md, kanto_map_scripts.md
tags: maps, kanto

## Sprint 140
goal: Build Fuchsia City + Route 15 + Route 14 + Route 13 + Route 12 — Safari Zone warden, gym (Janine)
sources: kanto_locations.md, warp_connections.md, kanto_map_scripts.md
tags: maps, kanto

## Sprint 141 [QA]
goal: QA audit of sprints 139-140 — verify Celadon through Fuchsia connectivity, closed Safari Zone, Janine's gym
sources: kanto_locations.md, warp_connections.md, kanto_map_scripts.md, game_progression.md
tags: qa

## Sprint 142
goal: Build Pewter City + Route 3 + Route 4 + Mt. Moon + Cerulean Cave — Brock's gym, dungeon layouts
sources: kanto_locations.md, warp_connections.md, kanto_map_scripts.md
tags: maps, kanto

## Sprint 143
goal: Build Viridian City + Route 22 + Route 23 + Route 2 + Route 1 — Blue's gym, Trainer House, Viridian Forest (cut down)
sources: kanto_locations.md, warp_connections.md, kanto_map_scripts.md
tags: maps, kanto

## Sprint 144 [QA]
goal: QA audit of sprints 142-143 — verify Pewter through Viridian, Blue's gym access, Trainer House, cut-down forest
sources: kanto_locations.md, warp_connections.md, kanto_map_scripts.md, game_progression.md
tags: qa

## Sprint 145
goal: Build Pallet Town + Route 21 + Cinnabar Island (destroyed) + Seafoam Islands — Blaine relocated gym, Professor Oak
sources: kanto_locations.md, warp_connections.md, kanto_map_scripts.md
tags: maps, kanto

## Sprint 146
goal: Build Route 9 + Route 10 + Rock Tunnel + Route 11 + Diglett's Cave — connecting routes and dungeons
sources: kanto_locations.md, warp_connections.md, kanto_map_scripts.md
tags: maps, kanto

## Sprint 147 [QA]
goal: QA audit of sprints 145-146 — verify Pallet/Cinnabar/Seafoam, Rock Tunnel, Diglett's Cave connectivity
sources: kanto_locations.md, warp_connections.md, kanto_map_scripts.md, game_progression.md
tags: qa

## Sprint 148
goal: Build Mt. Silver — exterior, interior floors, summit (Red battle location), high-level wild encounters
sources: kanto_locations.md, warp_connections.md, special_encounters.md
tags: maps, kanto, endgame

## Sprint 149
goal: Populate all Kanto wild encounter tables — grass/cave/water/fishing by route, time-of-day, Kanto-specific species
sources: wild_encounters.md, kanto_locations.md
tags: maps, kanto, encounters

## Sprint 150 [QA]
goal: QA audit of sprints 148-149 — verify Mt. Silver layout, Red's location, all Kanto encounter tables
sources: wild_encounters.md, kanto_locations.md, warp_connections.md, special_encounters.md
tags: qa

## Sprint 151
goal: Populate all Kanto trainer parties — gym leaders (Brock through Blue), route trainers, rematches
sources: all_trainers.md, kanto_locations.md, gym_leader_rematches.md
tags: maps, kanto, trainers

## Sprint 152
goal: Build S.S. Aqua — ship interior, cabins, NPC battles, Olivine-Vermilion route
sources: kanto_locations.md, johto_locations.md, warp_connections.md, kanto_map_scripts.md
tags: maps, transport

## Sprint 153 [QA]
goal: QA audit of sprints 151-152 — verify all Kanto gym teams, S.S. Aqua cabins and trainers, Magnet Train
sources: all_trainers.md, kanto_locations.md, warp_connections.md, game_progression.md
tags: qa

# ============================================================
# PHASE 9: TRAINERS & SPECIAL ENCOUNTERS (Sprints 154-168)
# ============================================================

## Sprint 154
goal: Implement rival battle system — 8 rival encounters, team evolution based on starter choice, escalating levels
sources: all_trainers.md, game_progression.md, story_events_by_map.md
tags: trainers, rival

## Sprint 155
goal: Implement Elite Four + Champion Lance — 5 sequential battles, no healing between, Hall of Fame registration
sources: all_trainers.md, elite_four_strategies.md, game_progression.md
tags: trainers, e4

## Sprint 156 [QA]
goal: QA audit of sprints 154-155 — verify rival team progression per starter, E4 teams and levels, Lance's team, Hall of Fame
sources: all_trainers.md, elite_four_strategies.md, game_progression.md, battle_scenarios.md
tags: qa

## Sprint 157
goal: Implement Red battle — Mt. Silver summit, Lv81 Pikachu + full team, no dialogue, final boss mechanics
sources: all_trainers.md, special_encounters.md, kanto_locations.md
tags: trainers, endgame

## Sprint 158
goal: Implement Team Rocket encounters — Slowpoke Well, Goldenrod Radio Tower takeover, Mahogany base, executives (Proton, Petrel, Ariana, Archer)
sources: all_trainers.md, story_events_by_map.md, johto_map_scripts.md
tags: trainers, team-rocket

## Sprint 159 [QA]
goal: QA audit of sprints 157-158 — verify Red's team, Team Rocket executive teams and locations, event sequencing
sources: all_trainers.md, story_events_by_map.md, game_progression.md
tags: qa

## Sprint 160
goal: Implement legendary encounters — roaming beasts (Raikou, Entei), Suicune storyline (6 encounters), Ho-Oh, Lugia
sources: special_encounters.md, story_events_by_map.md
tags: encounters, legendaries

## Sprint 161
goal: Implement remaining special encounters — Celebi (GS Ball), Lapras (Union Cave Friday), Red Gyarados, Sudowoodo, Snorlax
sources: special_encounters.md, event_engine.md
tags: encounters, special

## Sprint 162 [QA]
goal: QA audit of sprints 160-161 — verify roaming mechanics (HP persists, route changing), Suicune triggers, legendary levels and movesets
sources: special_encounters.md, story_events_by_map.md, game_progression.md
tags: qa

## Sprint 163
goal: Implement gift Pokemon and in-game trades — Togepi egg, Eevee, starter trade-backs, Shuckie, Kenya, all NPC trades
sources: special_encounters.md, npc_dialogue.md, event_data_tables.md
tags: encounters, gifts, trades

## Sprint 164
goal: Implement hidden items — all 84 hidden items with coordinates, Itemfinder interaction, respawn rules
sources: hidden_items_complete.md, item_locations.md
tags: items, hidden

## Sprint 165 [QA]
goal: QA audit of sprints 163-164 — verify all gift Pokemon, trade species/requested, hidden item coordinates and Itemfinder
sources: special_encounters.md, hidden_items_complete.md, item_locations.md, npc_dialogue.md
tags: qa

## Sprint 166
goal: Implement phone rematch system — gym leader rematches, phone registration, day-of-week schedules, level scaling
sources: gym_leader_rematches.md, phone_engine.md, phone_call_content.md
tags: trainers, phone, rematches

## Sprint 167
goal: Implement item locations — all obtainable items by location, shop inventories, department store, special shops (underground, Celadon)
sources: item_locations.md, all_items.md, item_details.md
tags: items, locations

## Sprint 168 [QA]
goal: QA audit of sprints 166-167 — verify rematch schedules and teams, all item pickup locations, shop inventories match pokecrystal
sources: gym_leader_rematches.md, item_locations.md, all_items.md, game_progression.md
tags: qa

# ============================================================
# PHASE 10: ITEMS & POKEMON MANAGEMENT (Sprints 169-183)
# ============================================================

## Sprint 169
goal: Implement item use mechanics — healing items (Potion/Super/Hyper/Max/Full Restore), status cure items, Revive, Sacred Ash
sources: item_use_mechanics.md, item_details.md
tags: items, mechanics

## Sprint 170
goal: Implement battle items — X items (Attack, Defend, Speed, Special, Accuracy), Guard Spec, Dire Hit, stat stage application
sources: item_use_mechanics.md, item_details.md, all_items.md
tags: items, battle

## Sprint 171 [QA]
goal: QA audit of sprints 169-170 — verify healing values, status cure targeting, X item stat stages, Guard Spec duration
sources: item_use_mechanics.md, item_details.md, battle_scenarios.md
tags: qa

## Sprint 172
goal: Implement Poke Ball mechanics — ball modifiers (Great 1.5x, Ultra 2x, Master guaranteed), Apricorn balls (Level, Moon, Lure, Friend, Love, Fast, Heavy)
sources: item_use_mechanics.md, formulas_reference.md, item_details.md
tags: items, catch

## Sprint 173
goal: Implement held item effects — type-boosting items (10%), Leftovers (1/16 HP), berries (auto-cure, auto-heal), Scope Lens, King's Rock, Focus Band
sources: item_use_mechanics.md, item_details.md, all_items.md
tags: items, held

## Sprint 174 [QA]
goal: QA audit of sprints 172-173 — verify all ball catch modifiers, held item trigger conditions, berry auto-use timing, Leftovers rounding
sources: item_use_mechanics.md, formulas_reference.md, item_details.md
tags: qa

## Sprint 175
goal: Implement evolution items — evolution stones (Fire, Water, Thunder, Leaf, Sun, Moon), trade items (King's Rock, Metal Coat, Dragon Scale, Up-Grade)
sources: evolution_mechanics.md, item_use_mechanics.md, evolution_chains.md
tags: items, evolution

## Sprint 176
goal: Implement TM/HM system — all 50 TMs + 7 HMs, compatibility checks per species, move replacement, HM deletion restrictions (Move Deleter)
sources: all_items.md, all_pokemon.md, item_use_mechanics.md
tags: items, moves, tm-hm

## Sprint 177 [QA]
goal: QA audit of sprints 175-176 — verify all evolution stone triggers, trade evolution items, TM compatibility per species, HM restriction enforcement
sources: evolution_mechanics.md, all_items.md, all_pokemon.md
tags: qa

## Sprint 178
goal: Implement Bill's PC box system — 14 boxes, 20 Pokemon each, deposit/withdraw/move, box change save, mail handling
sources: pokemon_management.md, memory_map.md
tags: pokemon-management, pc

## Sprint 179
goal: Implement party management — party menu, move ordering, Pokemon summary screen, move learning/forgetting, nickname system
sources: pokemon_management.md, menu_systems.md
tags: pokemon-management, party

## Sprint 180 [QA]
goal: QA audit of sprints 178-179 — verify PC operations, box capacity, save on box change, party menu functions, move learning prompts
sources: pokemon_management.md, menu_systems.md
tags: qa

## Sprint 181
goal: Implement evolution system — all evolution methods (level, stone, trade, happiness day/night, trade+item), evolution animation, stat recalculation
sources: evolution_mechanics.md, evolution_chains.md, formulas_reference.md
tags: evolution, mechanics

## Sprint 182
goal: Implement breeding system — Day Care, egg group compatibility, egg production (step counter), DV inheritance, shiny breeding, egg moves
sources: breeding_mechanics.md, evolution_mechanics.md, event_data_tables.md
tags: breeding, mechanics

## Sprint 183 [QA]
goal: QA audit of sprints 181-182 — verify all evolution triggers, happiness thresholds, breeding compatibility, DV inheritance rules, Odd Egg
sources: evolution_mechanics.md, breeding_mechanics.md, event_data_tables.md, formulas_reference.md
tags: qa

# ============================================================
# PHASE 11: TIME SYSTEM & EVENTS (Sprints 184-195)
# ============================================================

## Sprint 184
goal: Implement time-of-day system — morning/day/night periods, palette changes, encounter table switching, time-sensitive NPCs
sources: time_system.md, rtc_engine.md
tags: time, overworld

## Sprint 185
goal: Implement day-of-week system — day tracking, daily events (berry trees, Lapras, Bug Contest), weekly reset mechanics
sources: time_system.md, event_engine.md, rtc_engine.md
tags: time, events

## Sprint 186 [QA]
goal: QA audit of sprints 184-185 — verify time period boundaries, palette transitions, day-of-week event triggers, daily reset
sources: time_system.md, rtc_engine.md, event_engine.md
tags: qa

## Sprint 187
goal: Implement happiness system — base happiness per species, gain events (walking, level-up, vitamins, grooming), loss events (faint), evolution threshold
sources: formulas_reference.md, event_data_tables.md, evolution_mechanics.md
tags: mechanics, happiness

## Sprint 188
goal: Implement Pokerus system — infection, spread mechanics (step-based), strain/duration, stat EV doubling effect
sources: event_engine.md, formulas_reference.md
tags: mechanics, pokerus

## Sprint 189 [QA]
goal: QA audit of sprints 187-188 — verify happiness gain/loss values, evolution threshold, Pokerus spread rules, strain system
sources: formulas_reference.md, event_data_tables.md, event_engine.md
tags: qa

## Sprint 190
goal: Implement story events part 1 — game start through Badge 4 (Elm intro, starter, rival battles 1-3, Team Rocket Slowpoke Well, Sudowoodo, Ecruteak events)
sources: story_events_by_map.md, game_progression.md, johto_map_scripts.md
tags: story, events, johto

## Sprint 191
goal: Implement story events part 2 — Badge 5 through Badge 8 (Jasmine/Amphy, Rocket Radio Tower takeover, Lake of Rage, Ice Path, Dragon's Den)
sources: story_events_by_map.md, game_progression.md, johto_map_scripts.md
tags: story, events, johto

## Sprint 192 [QA]
goal: QA audit of sprints 190-191 — verify all story flags, event sequencing, progression gates (badges, events), cutscene triggers
sources: story_events_by_map.md, game_progression.md, johto_map_scripts.md
tags: qa

## Sprint 193
goal: Implement Kanto story events — Power Plant restoration, Copycat's Clef Doll, Lost Item, EXPN Card, Kanto gym unlocks, Mt. Silver access
sources: story_events_by_map.md, game_progression.md, kanto_map_scripts.md
tags: story, events, kanto

## Sprint 194
goal: Implement Legendary Beast storyline — Burned Tower release event, Suicune 6-encounter chain, Clear Bell, Tin Tower access, Eusine battles
sources: story_events_by_map.md, special_encounters.md, johto_map_scripts.md
tags: story, legendaries

## Sprint 195 [QA]
goal: QA audit of sprints 193-194 — verify Kanto event chains, Power Plant fix, Legendary Beast triggers, Suicune encounters in order
sources: story_events_by_map.md, game_progression.md, special_encounters.md
tags: qa

# ============================================================
# PHASE 12: UI, MENUS, POKEGEAR (Sprints 196-210)
# ============================================================

## Sprint 196
goal: Implement start menu — Pokedex/Pokemon/Pack/Player/Save/Option/Exit, conditional visibility, options menu (text speed, battle scene/style, sound, frame)
sources: menu_systems.md, game_constants.md
tags: ui, menus

## Sprint 197
goal: Implement bag/pack system — 4 pockets (Items, Balls, Key Items, TMs/HMs), item use from menu, registered item (Select button), toss/give
sources: menu_systems.md, item_use_mechanics.md
tags: ui, items, menus

## Sprint 198 [QA]
goal: QA audit of sprints 196-197 — verify menu layout, option persistence in save, bag pocket sorting, item registration, key item behavior
sources: menu_systems.md, item_use_mechanics.md, all_items.md
tags: qa

## Sprint 199
goal: Implement Pokegear — Clock card, Map card (Johto/Kanto toggle, fly destinations), Phone card (10 contacts, call mechanics)
sources: pokegear.md, phone_engine.md
tags: ui, pokegear

## Sprint 200
goal: Implement phone system — call scheduling, incoming calls (hints, rematch offers, item gifts), trainer contact registration, call text content
sources: phone_engine.md, phone_call_content.md, pokegear.md
tags: ui, phone

## Sprint 201 [QA]
goal: QA audit of sprints 199-200 — verify Pokegear card unlocking, map display, phone contact list, call scheduling, rematch triggers
sources: pokegear.md, phone_engine.md, phone_call_content.md
tags: qa

## Sprint 202
goal: Implement radio system — 11 stations, Pokemon March/Lullaby (encounter rate mod), Oak's Pokemon Talk, Buena's Password, Team Rocket signal
sources: radio_system.md, pokegear.md
tags: ui, radio

## Sprint 203
goal: Implement Pokedex system — seen/caught tracking, search modes (type, name, habitat), Oak's rating system, area display
sources: pokedex_system.md, all_pokemon.md
tags: ui, pokedex

## Sprint 204 [QA]
goal: QA audit of sprints 202-203 — verify all radio stations, encounter rate modifiers from radio, Pokedex search accuracy, Oak's completion ratings
sources: radio_system.md, pokedex_system.md, pokegear.md
tags: qa

## Sprint 205
goal: Implement save system — save data structure, save/load/new game, box change saves, SRAM integrity check, continue screen
sources: menu_systems.md, memory_map.md
tags: ui, save

## Sprint 206
goal: Implement NPC dialogue system — key NPC text, move tutors (3 tutors: Headbutt, Flamethrower/Thunderbolt/Ice Beam, Elemental punches), name rater
sources: npc_dialogue.md, game_text_strings.md, scripting_commands.md
tags: ui, dialogue, npcs

## Sprint 207 [QA]
goal: QA audit of sprints 205-206 — verify save/load integrity, continue screen data, move tutor availability, NPC dialogue accuracy
sources: menu_systems.md, memory_map.md, npc_dialogue.md, game_text_strings.md
tags: qa

## Sprint 208
goal: Implement trainer card — badges display, money, Pokedex count, play time, ID number
sources: menu_systems.md, game_constants.md
tags: ui, trainer-card

## Sprint 209
goal: Implement Unown Pokedex mode — Unown form collection (26 letters), Ruins of Alph puzzle unlock tracking, special Unown word display
sources: ruins_of_alph.md, pokedex_system.md
tags: ui, pokedex, ruins

## Sprint 210 [QA]
goal: QA audit of sprints 208-209 — verify trainer card display, badge tracking, Unown form registration, Ruins of Alph Pokedex mode
sources: menu_systems.md, ruins_of_alph.md, pokedex_system.md
tags: qa

# ============================================================
# PHASE 13: MINI-GAMES & SPECIAL SYSTEMS (Sprints 211-228)
# ============================================================

## Sprint 211
goal: Implement slot machine — 3 reels (15 symbols each), payout table, bias/mode system, coin purchase, prize redemption
sources: mini_games.md, event_engine.md
tags: mini-game, game-corner

## Sprint 212
goal: Implement card flip game — 4x6 grid, bet types (single/column/row/pair), payout rates, coin costs
sources: mini_games.md
tags: mini-game, game-corner

## Sprint 213 [QA]
goal: QA audit of sprints 211-212 — verify slot machine odds and bias modes, card flip payouts, coin balance tracking
sources: mini_games.md, event_engine.md
tags: qa

## Sprint 214
goal: Implement Bug Catching Contest — 20-minute timer, Park Ball allocation, scoring formula, NPC contestant scoring, prize awards
sources: event_engine.md, mini_games.md, johto_map_scripts.md
tags: mini-game, contest

## Sprint 215
goal: Implement Unown puzzle — sliding tile puzzles (Kabuto, Omanyte, Aerodactyl, Ho-Oh), secret room unlocks, Unown letter group unlocking
sources: ruins_of_alph.md, mini_games.md
tags: mini-game, puzzle

## Sprint 216 [QA]
goal: QA audit of sprints 214-215 — verify contest scoring, NPC random scoring, prize distribution, puzzle solve detection, Unown unlocking
sources: event_engine.md, ruins_of_alph.md, mini_games.md
tags: qa

## Sprint 217
goal: Implement Battle Tower — 7-trainer streak, 70 BT trainers, 210 Pokemon builds, level 50/open rules, item clause, streak rewards
sources: battle_tower.md, event_engine.md
tags: battle-tower, endgame

## Sprint 218
goal: Implement Battle Tower Pokemon builds — all 210 BT Pokemon with species, moves, items, DVs; random selection per challenger
sources: battle_tower.md
tags: battle-tower, data

## Sprint 219 [QA]
goal: QA audit of sprints 217-218 — verify BT trainer selection, Pokemon build accuracy, streak counting, reward distribution
sources: battle_tower.md, event_engine.md
tags: qa

## Sprint 220
goal: Implement Mystery Gift system — IR communication protocol, daily limits (5 gifts/day), item vs decoration gifts, partner tracking
sources: mystery_gift_decorations.md, link_protocol.md
tags: mystery-gift, link

## Sprint 221
goal: Implement decoration system — 29 decorations, room placement (bed, carpet, plant, poster, doll, ornament), Trainer House customization
sources: mystery_gift_decorations.md
tags: decorations, player-house

## Sprint 222 [QA]
goal: QA audit of sprints 220-221 — verify Mystery Gift daily limits, decoration placement rules, room capacity, item delivery
sources: mystery_gift_decorations.md, link_protocol.md
tags: qa

## Sprint 223
goal: Implement link trading system — trade engine, evolution triggers on trade, item trade evolution, trade animation
sources: link_protocol.md, link_battle.md, evolution_mechanics.md
tags: link, trading

## Sprint 224
goal: Implement Time Capsule — Gen 1 trade restrictions (no Gen 2 moves/items/Pokemon), compatibility checks, conversion rules
sources: link_battle.md, link_protocol.md
tags: link, time-capsule

## Sprint 225 [QA]
goal: QA audit of sprints 223-224 — verify trade evolution triggers, Time Capsule restrictions, link protocol sync
sources: link_protocol.md, link_battle.md, evolution_mechanics.md
tags: qa

## Sprint 226
goal: Implement link battle system — 6v6 link battle rules, no bag items, synchronized PRNG, Colosseum compatibility
sources: link_battle.md, link_protocol.md, rng_mechanics.md
tags: link, battle

## Sprint 227
goal: Implement printer system — Game Boy Printer protocol, print Pokemon party, print Pokedex entries, print mail
sources: printer_system.md, link_protocol.md
tags: peripheral, printer

## Sprint 228 [QA]
goal: QA audit of sprints 226-227 — verify link battle sync, PRNG consistency, printer output formatting
sources: link_battle.md, link_protocol.md, printer_system.md, rng_mechanics.md
tags: qa

# ============================================================
# PHASE 14: GRAPHICS, MUSIC, CUTSCENES (Sprints 229-240)
# ============================================================

## Sprint 229
goal: Implement Pokemon sprite system — front/back sprites for all 251 species, 2bpp decompression, VRAM loading
sources: graphics_and_cutscenes.md, sprites_and_animations.md
tags: graphics, sprites

## Sprint 230
goal: Implement sprite animations — Pokemon stat screen animations (bounce, shake, stretch), per-species animation data
sources: sprites_and_animations.md, graphics_and_cutscenes.md
tags: graphics, animations

## Sprint 231 [QA]
goal: QA audit of sprints 229-230 — verify sprite loading for all 251 species, animation correctness, VRAM management
sources: sprites_and_animations.md, graphics_and_cutscenes.md
tags: qa

## Sprint 232
goal: Implement battle animations — move animations, status animations, weather visual effects, battle transition effects
sources: graphics_and_cutscenes.md, sprites_and_animations.md
tags: graphics, battle-anims

## Sprint 233
goal: Implement music system — audio engine, 103 music tracks, map-to-music assignments, battle music selection
sources: music_and_sound.md
tags: audio, music

## Sprint 234 [QA]
goal: QA audit of sprints 232-233 — verify battle animation timing, music track assignments per map, battle music triggers
sources: graphics_and_cutscenes.md, music_and_sound.md
tags: qa

## Sprint 235
goal: Implement sound effects — battle SFX (hit, faint, status, item), overworld SFX (door, ledge, bump), Pokemon cries
sources: music_and_sound.md, graphics_and_cutscenes.md
tags: audio, sfx

## Sprint 236
goal: Implement intro sequence — Crystal-specific intro animation, title screen, new game/continue/Mystery Gift menu
sources: graphics_and_cutscenes.md, menu_systems.md
tags: graphics, cutscene

## Sprint 237 [QA]
goal: QA audit of sprints 235-236 — verify SFX triggers, cry playback, intro animation sequence, title screen options
sources: music_and_sound.md, graphics_and_cutscenes.md, menu_systems.md
tags: qa

## Sprint 238
goal: Implement credits sequence — post-Champion credits, scrolling animation, staff listing, egg hatching sequence post-credits
sources: graphics_and_cutscenes.md
tags: graphics, cutscene

## Sprint 239
goal: Implement overworld visual effects — time-of-day palettes, weather overlays, flash darkness gradient, whiteout screen
sources: graphics_and_cutscenes.md, time_system.md, sprites_and_animations.md
tags: graphics, overworld

## Sprint 240 [QA]
goal: QA audit of sprints 238-239 — verify credits sequence, palette transitions, weather overlay rendering, flash cave darkness
sources: graphics_and_cutscenes.md, time_system.md, sprites_and_animations.md
tags: qa

# ============================================================
# PHASE 15: BUG RECREATION & EDGE CASES (Sprints 241-252)
# ============================================================

## Sprint 241
goal: Implement battle engine bugs — Thick Club/Light Ball overflow, Reflect/Light Screen overflow, 1/256 miss, Belly Drum HP glitch, Berserk Gene confusion
sources: bugs_and_glitches.md, implementation_gotchas.md
tags: bugs, battle

## Sprint 242
goal: Implement move-specific bugs — Counter/Mirror Coat item bug, disabled PP Up Struggle bypass, defense-lowering after Substitute break, Confusion item damage boost
sources: bugs_and_glitches.md, implementation_gotchas.md
tags: bugs, moves

## Sprint 243 [QA]
goal: QA audit of sprints 241-242 — verify all battle bugs match pokecrystal behavior exactly, test overflow conditions
sources: bugs_and_glitches.md, implementation_gotchas.md, battle_scenarios.md
tags: qa

## Sprint 244
goal: Implement remaining battle bugs — Perish Song + Spikes 0 HP, Beat Up desync, Rage stat wipe, Lock-On/Detect interaction, HP bar animation desync
sources: bugs_and_glitches.md, implementation_gotchas.md
tags: bugs, battle

## Sprint 245
goal: Implement overworld and system bugs — Coin Case arbitrary code, stone evolution stat exp, Day Care level-up move loss, map-related glitches
sources: bugs_and_glitches.md, design_flaws.md, implementation_gotchas.md
tags: bugs, overworld

## Sprint 246 [QA]
goal: QA audit of sprints 244-245 — verify all 92 documented bugs are implemented or intentionally excluded with justification
sources: bugs_and_glitches.md, design_flaws.md, implementation_gotchas.md
tags: qa

## Sprint 247
goal: Address design flaws — all 10 documented design flaws, decision on recreation vs. fix for each
sources: design_flaws.md, implementation_gotchas.md
tags: design-flaws

## Sprint 248
goal: Implement all 55 implementation gotchas — ASM-cited pitfalls for faithful recreation, edge case behaviors
sources: implementation_gotchas.md, bugs_and_glitches.md
tags: gotchas, edge-cases

## Sprint 249 [QA]
goal: QA audit of sprints 247-248 — verify design flaw decisions, gotcha implementations against ASM citations
sources: design_flaws.md, implementation_gotchas.md, bugs_and_glitches.md
tags: qa

## Sprint 250
goal: Implement core routines — key home/ routines (RNG, math helpers, text engine, memory copy/fill), utility functions
sources: core_routines.md, memory_map.md
tags: core, routines

## Sprint 251
goal: Implement remaining formulas — flee rate, friendship evolution, Magikarp length, egg step counter, Odd Egg shiny rates, day care level gain
sources: formulas_reference.md, event_data_tables.md
tags: formulas, edge-cases

## Sprint 252 [QA]
goal: QA audit of sprints 250-251 — verify utility routine correctness, formula edge cases, Magikarp length distribution
sources: core_routines.md, formulas_reference.md, event_data_tables.md
tags: qa

# ============================================================
# PHASE 16: FULL INTEGRATION TESTING (Sprints 253-264)
# ============================================================

## Sprint 253
goal: Integration test: New Bark Town through Violet City — full playthrough segment, starter selection, rival battle, Sprout Tower, Gym 1
sources: game_progression.md, gym_strategies.md, story_events_by_map.md
tags: integration, testing

## Sprint 254
goal: Integration test: Azalea through Goldenrod — Slowpoke Well, Ilex Forest, Bug Contest, Whitney's Miltank, Radio Tower
sources: game_progression.md, gym_strategies.md, story_events_by_map.md
tags: integration, testing

## Sprint 255 [QA]
goal: QA audit of sprints 253-254 — full early-game regression, verify progression gates, softlock detection, save/load mid-playthrough
sources: game_progression.md, gym_strategies.md, story_events_by_map.md, battle_scenarios.md
tags: qa

## Sprint 256
goal: Integration test: Ecruteak through Mahogany — Legendary Beasts release, Olivine-Cianwood loop, Team Rocket Mahogany base, Red Gyarados
sources: game_progression.md, gym_strategies.md, story_events_by_map.md
tags: integration, testing

## Sprint 257
goal: Integration test: Blackthorn through E4 — Dragon's Den, Victory Road, Elite Four gauntlet, Champion Lance, Hall of Fame
sources: game_progression.md, elite_four_strategies.md, story_events_by_map.md
tags: integration, testing

## Sprint 258 [QA]
goal: QA audit of sprints 256-257 — mid-to-late Johto regression, E4 difficulty balance, progression flag verification
sources: game_progression.md, elite_four_strategies.md, story_events_by_map.md
tags: qa

## Sprint 259
goal: Integration test: Kanto playthrough — Vermilion through all 8 Kanto gyms, Power Plant quest, Magnet Train unlock
sources: game_progression.md, gym_strategies.md, kanto_map_scripts.md
tags: integration, testing, kanto

## Sprint 260
goal: Integration test: Endgame content — Mt. Silver + Red battle, Battle Tower streaks, Legendary completion (all 5 legendaries)
sources: game_progression.md, special_encounters.md, battle_tower.md
tags: integration, testing, endgame

## Sprint 261 [QA]
goal: QA audit of sprints 259-260 — full Kanto regression, Red battle difficulty, Battle Tower AI, all legendary encounters
sources: game_progression.md, special_encounters.md, battle_tower.md
tags: qa

## Sprint 262
goal: Integration test: Speedrun route validation — verify any% glitchless route is completable, key split times reasonable
sources: speedrun_notes.md, game_progression.md
tags: integration, testing, speedrun

## Sprint 263
goal: Integration test: Competitive validation — verify competitive Pokemon are obtainable, movesets learnable, breeding produces correct results, team building viable
sources: competitive_gen2.md, competitive_pokemon.md, team_building.md
tags: integration, testing, competitive

## Sprint 264 [QA]
goal: Final QA audit — complete game regression, 205 QA pairs verification, all 81 data files consumed, softlock sweep, save corruption testing
sources: qa_pairs.md, faq.md, game_progression.md, bugs_and_glitches.md
tags: qa, final

# ============================================================
# DATA FILE COVERAGE VERIFICATION
# ============================================================
#
# species/all_pokemon.md           -> 4,5,6,10,12,133,176,203,263
# species/evolution_chains.md      -> 11,12,175,181
# species/learnset_by_move.md      -> 10,12
# species/competitive_pokemon.md   -> 263
# species/pokemon_trivia.md        -> 6
# species/faq.md                   -> 6,264
#
# moves/all_moves.md               -> 7,8,9
# moves/move_details.md            -> 7,8,9,52,67
#
# items/all_items.md               -> 13,15,167,170,176,197
# items/item_details.md            -> 13,15,167,170,172,173
#
# types/type_chart.md              -> 1,3,23
#
# trainers/all_trainers.md         -> 41,42,134,151,154,155,157,158
# trainers/gym_leader_rematches.md -> 151,166,168
#
# encounters/wild_encounters.md    -> 106,107,108,133,149
# encounters/special_encounters.md -> 128,148,157,160,161,163,194,260
# encounters/item_locations.md     -> 164,167
# encounters/battle_tower.md       -> 217,218,260
#
# mechanics/damage_formula.md      -> 22,23,24,64
# mechanics/stat_calculation.md    -> 16,17,18,20
# mechanics/battle_mechanics.md    -> 25,26,37,38,40,41,43
# mechanics/battle_core_details.md -> 28,29,35,38,43
# mechanics/battle_scenarios.md    -> 18,21,24,27,30,33,36,39,42,45,48,51,54,57,60,63,66,69,72,75,78
# mechanics/effect_commands_reference.md -> 9,29,46,47,49,50,52,53,56,58,59,61,62,64,65,68,74,77
# mechanics/status_effects.md      -> 28,31,32,33,34,36,48,59
# mechanics/move_edge_cases.md     -> 53,56,62,63,70,71,73,76
# mechanics/move_effects_complete.md -> 20,31,32,34,46,47,49,50,52,53,55,56,58,59,61,62,64,65,67,68,70,71,73,74,76,77
# mechanics/formulas_reference.md  -> 17,22,23,32,37,40,106,107,172,181,187,251
# mechanics/ai_behavior.md         -> 79,80,82,83,85,86,87
# mechanics/ai_scoring_details.md  -> 79,80,82,83,85,86,87,88,89,90
# mechanics/ai_data_tables.md      -> 88,89,90
# mechanics/breeding_mechanics.md  -> 182,183
# mechanics/evolution_mechanics.md -> 11,175,181,183,223
# mechanics/bugs_and_glitches.md   -> 24,51,63,66,76,78,241,242,244,245,246,248,249,264
# mechanics/design_flaws.md        -> 245,247,249
# mechanics/implementation_gotchas.md -> 3,241,242,245,247,248,249
# mechanics/rng_mechanics.md       -> 19,21,226
# mechanics/state_machine.md       -> 25,26,27,35,91
# mechanics/overworld_engine.md    -> 40,91,92,94,98,100,106,107,109,110
# mechanics/event_engine.md        -> 103,161,185,188,211,214,217
# mechanics/event_data_tables.md   -> 163,187,251
# mechanics/time_system.md         -> 184,185,239
# mechanics/rtc_engine.md          -> 184,185
# mechanics/item_use_mechanics.md  -> 110,169,170,172,175,176,197
# mechanics/menu_systems.md        -> 104,179,196,197,205,208,236
# mechanics/mini_games.md          -> 211,212,214,215
# mechanics/pokegear.md            -> 199,200,202
# mechanics/radio_system.md        -> 202
# mechanics/phone_engine.md        -> 166,200
# mechanics/link_battle.md         -> 224,226
# mechanics/link_protocol.md       -> 220,223,224,226,227
# mechanics/pokemon_management.md  -> 178,179
# mechanics/pokedex_system.md      -> 203,209
# mechanics/collision_and_tilesets.md -> 92,94,95,100,101,109,110
# mechanics/sprites_and_animations.md -> 95,97,98,229,230,232,239
# mechanics/graphics_and_cutscenes.md -> 95,229,230,232,235,236,238,239
# mechanics/game_constants.md      -> 2,3,14,16,196,208
# mechanics/game_text_strings.md   -> 44,104,206
# mechanics/memory_map.md          -> 14,15,178,205,250
# mechanics/core_routines.md       -> 19,250
# mechanics/scripting_commands.md  -> 103,206
# mechanics/mystery_gift_decorations.md -> 220,221
# mechanics/printer_system.md      -> 227
#
# maps/johto_locations.md          -> 112,113,115,116,118,119,121,122,124,125,127,128,131,133,134
# maps/johto_map_scripts.md        -> 112,113,115,116,118,119,121,122,124,130,134,158,190,191,214
# maps/kanto_locations.md          -> 136,137,139,140,142,143,145,146,148,149,151,157
# maps/kanto_map_scripts.md        -> 136,137,139,140,142,143,145,146,152,193
# maps/game_progression.md         -> 114,117,120,123,126,129,132,135,138,141,144,147,150,153,154,155,190,191,193,253,254,256,257,259,260,262,264
# maps/story_events_by_map.md      -> 154,158,160,190,191,193,194,253,254,256,257
# maps/hidden_items_complete.md    -> 164
# maps/warp_connections.md         -> 100,101,112,113,115,116,118,119,121,122,124,125,127,128,131,136,137,139,140,142,143,145,146,148,152
# maps/npc_dialogue.md             -> 163,206
# maps/phone_call_content.md       -> 166,200
# maps/music_and_sound.md          -> 233,235
# maps/ruins_of_alph.md            -> 115,130,209,215
#
# strategy/gym_strategies.md       -> 253,254,256,259
# strategy/elite_four_strategies.md -> 155,156,257
# strategy/team_building.md        -> 263
# strategy/speedrun_notes.md       -> 262
# strategy/competitive_gen2.md     -> 263
#
# meta/qa_pairs.md                 -> 264
#
# ALL 81 FILES COVERED
