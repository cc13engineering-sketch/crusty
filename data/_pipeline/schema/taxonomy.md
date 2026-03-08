# Pokemon Crystal Vector DB — Classification Taxonomy

> Complete hierarchical taxonomy for categorizing all Pokemon Crystal knowledge.
> Every chunk is classified into one category + subcategory, and tagged from the
> standardized vocabulary below.

---

## 1. Top-Level Domains (Categories)

```
battle_system       — Everything about combat: damage, status, weather, AI, priority
pokemon_data        — Species stats, learnsets, evolutions, egg groups, types
moves_data          — Move definitions, effects, interactions, TM/HM assignments
items_data          — All items: held items, key items, balls, berries, medicine
overworld           — Maps, routes, encounters, NPCs, warps, HM obstacles
story_progression   — Plot events, flags, prerequisites, progression gates
trainer_data        — Trainer teams, classes, AI, gym leaders, E4, rival
type_system         — Type chart, STAB, type-based item boosts, physical/special split
meta_game           — Strategy, team building, competitive, speedrun, glitches
side_features       — Pokegear, phone, radio, bug contest, decorations, Game Corner
audio_visual        — Music, sound effects, sprites, palettes, animations
```

---

## 2. Subcategories Per Domain

### battle_system
```
damage_calc         — Core damage formula, modifiers, rounding
critical_hits       — Crit rate calculation, crit stages, high-crit moves
accuracy_evasion    — Accuracy check, evasion stages, always-hit moves, OHKO formula
stat_stages         — Stat modifier system (+1 through +6, -1 through -6)
status_effects      — Burn, poison, sleep, freeze, paralysis, confusion, infatuation
weather             — Rain, sun, sandstorm — effects on moves, damage, abilities
priority            — Move priority brackets, speed ties, Quick Claw
switching           — Switching mechanics, trapping, Baton Pass, Pursuit
multi_turn          — Fly, Dig, Solar Beam, Rollout, Fury Cutter, charge moves
recoil_drain        — Recoil damage, HP drain, Struggle, crash damage
protection          — Protect, Detect, Endure, successive use odds
field_effects       — Reflect, Light Screen, Safeguard, Spikes, weather
fainting            — Faint mechanics, EXP distribution, forced switches
ai_behavior         — Trainer AI scoring, item usage, switching logic
held_items_battle   — In-battle held item effects, berries, type boosters
```

### pokemon_data
```
base_stats          — HP, Atk, Def, SpA, SpD, Spe for each species
typing              — Type assignments per species (dual-type handling)
level_up_learnset   — Moves learned by level-up
tm_hm_learnset      — TM and HM compatibility
egg_moves           — Moves inherited via breeding
evolution_methods    — Level, item, trade, happiness, stat-based evolution
evolution_chains     — Full family trees (Eevee -> 5 branches, etc.)
egg_groups          — Breeding compatibility groups
gender_ratios       — Male/female distribution per species
growth_rates        — EXP curves: fast, medium-fast, medium-slow, slow
catch_rates         — Catch rate values and capture formula
stat_experience     — Stat EXP (EVs) system, vitamin limits
happiness           — Happiness system, sources of happiness gain/loss
shiny               — Shiny odds, DV-based determination
unown               — Unown forms, puzzle, wall patterns
pokedex_entries     — Dex text, height, weight, category
```

### moves_data
```
move_stats          — Base power, accuracy, PP, type, priority
move_effects        — Effect descriptions and resolution
status_moves        — Non-damaging moves (stat changes, status, field)
damaging_moves      — Physical/special attacks with power > 0
signature_moves     — Moves exclusive or near-exclusive to certain species
hm_moves            — HM-specific moves and field effects (Cut, Surf, etc.)
tm_list             — TM numbers, moves, locations, costs
pp_system           — PP, PP Up, max PP
move_interactions   — How specific moves interact (Sleep Talk + rest moves, etc.)
```

### items_data
```
held_items          — Items Pokemon can hold in battle
pokeballs           — Ball types and catch rate modifiers
medicine            — Potions, status heals, revives
vitamins            — HP Up, Protein, Iron, Calcium, Carbos
evolution_stones    — Fire/Water/Thunder/Leaf/Moon/Sun Stone
berries             — Gen 2 berries (held item berries)
key_items           — Story-required items, HMs, quest items
tm_hm_items         — TM/HM as inventory items
mail                — Mail types and mechanics
decorations         — Room decorations (Mystery Gift)
mart_inventory      — What each Poke Mart sells and prices
item_locations      — Where every item is found (overworld, gift, purchase)
```

### overworld
```
cities_towns        — City/town maps, buildings, features
routes              — Route connections, terrain, obstacles
dungeons            — Caves, towers, forests (multi-floor)
buildings           — Gyms, Poke Centers, Marts, houses
connections         — Map adjacency graph
warps               — Doors, stairs, cave exits, fly points
hm_obstacles        — Cut trees, Surf water, Strength boulders, Whirlpool, Waterfall
npc_dialogue        — Important NPC text and triggers
item_pickups        — Overworld item ball locations
hidden_items        — Itemfinder-revealed items
time_of_day         — Morning/day/night effects on overworld
day_of_week         — Day-specific events (Buena's Password, Goldenrod lottery)
```

### story_progression
```
main_quest          — Required story events in order
gym_progression     — Gym order, requirements, badge effects
rival_encounters    — Rival battle triggers and teams per encounter
team_rocket         — Rocket plot events (Slowpoke Well, Radio Tower, etc.)
legendary_events    — Ho-Oh, Lugia, Suicune, Celebi encounters
post_game           — Kanto portion, Red battle, post-E4 content
phone_calls         — Story-relevant phone triggers
flag_system         — Internal game flags and how events set them
```

### trainer_data
```
gym_leaders         — 16 gym leader teams (8 Johto + 8 Kanto)
elite_four          — E4 + Champion Lance teams
rival               — All rival encounters (7+ battles)
route_trainers      — Non-boss trainers on routes and caves
rematch_trainers    — Trainers with phone-triggered rematches
trainer_ai          — AI difficulty, type-specific strategies
trainer_items       — Items used by trainers in battle
battle_tower        — Battle Tower teams and rules
```

### type_system
```
type_chart          — Full 17x17 effectiveness table (incl. Steel/Dark)
offensive_matchups  — Each type's super-effective / not-very / immune targets
defensive_matchups  — Each type's weaknesses / resistances / immunities
dual_type           — How dual typing stacks effectiveness
stab                — Same-Type Attack Bonus rules
physical_special    — Which types are physical vs. special in Gen 2
type_items          — Items that boost specific types (Charcoal, Mystic Water, etc.)
```

### meta_game
```
team_building       — Recommended team compositions for story mode
boss_strategies     — How to beat specific gym leaders / E4 / Red
routing             — Optimal play order, when to grind, where to train
competitive_sets    — Stadium 2 / link battle viable movesets
speedrun_strats     — Speedrun-relevant tech and routing
glitches_exploits   — Known glitches (Coin Case, cloning, etc.)
```

### side_features
```
pokegear            — Phone, radio, map features
bug_contest         — Bug Catching Contest rules, scoring, prizes
game_corner         — Goldenrod Game Corner, Voltorb Flip, prizes
breeding            — Day Care, egg hatching, move inheritance
happiness_events    — Haircut brothers, vitamins, friendship berries
mystery_gift        — Mystery Gift mechanics and decorations
trainer_house       — Viridian Trainer House battles
safari_zone         — (Not in Crystal, but worth noting absence)
```

### audio_visual
```
music_tracks        — Route music, battle themes, city themes
sound_effects       — Move SFX, menu sounds
pokemon_sprites     — Front/back sprites, shiny palettes
trainer_sprites     — Trainer class artwork
overworld_sprites   — NPC and player overworld graphics
animations          — Battle move animations
palettes            — Color palettes by Pokemon, SGB/GBC differences
```

---

## 3. Standardized Tag Vocabulary

Tags provide fine-grained retrieval beyond category/subcategory. Use only tags from this list.

### Entity Tags (what the chunk is about)
```
# Pokemon-specific
starter, legendary, mythical, pseudo_legendary, baby_pokemon, trade_evolution
johto_native, kanto_native, gen2_new, unown

# Move-specific
physical, special, status, contact, sound_based, punch_move
high_priority, negative_priority, never_miss
multi_hit, two_turn, recoil, drain, self_destruct
weather_boosted, field_effect

# Type tags (use as needed)
normal, fire, water, electric, grass, ice, fighting, poison
ground, flying, psychic, bug, rock, ghost, dragon, dark, steel

# Trainer-specific
gym_leader, elite_four, champion, rival, rocket_grunt, rocket_admin
sage, lass, youngster, hiker, swimmer, bug_catcher

# Location-specific
johto, kanto, indoor, outdoor, cave, tower, forest, ocean
early_game, mid_game, late_game, post_game

# Mechanic-specific
damage, accuracy, evasion, critical, weather, priority, switching
status_condition, stat_boost, stat_drop, field_effect
turn_order, faint, experience, capture
```

### Qualifier Tags (how to use the chunk)
```
formula, table, list, step_by_step, edge_case, exception
beginner, intermediate, advanced
verified_asm, derived, approximate
```

---

## 4. Entity Relationship Model

These are the core relationships between entity types in the knowledge base.
Use these for populating `related_entities` and enabling graph traversal.

### Primary Relationships

```
Pokemon --learns--> Move          (via level-up, TM/HM, egg move, tutor)
Pokemon --evolves_to--> Pokemon   (level, item, trade, happiness)
Pokemon --has_type--> Type        (primary, secondary)
Pokemon --found_at--> Location    (wild encounter)
Pokemon --in_egg_group--> EggGroup
Pokemon --holds--> Item           (wild held item)

Move --has_type--> Type
Move --has_effect--> MoveEffect
Move --governed_by--> Mechanic

Trainer --uses--> Pokemon
Trainer --located_at--> Location
Trainer --awards--> Badge         (gym leaders)
Trainer --part_of--> StoryEvent

Location --connects_to--> Location
Location --contains--> Item       (overworld pickup)
Location --has_encounter--> WildEncounter
Location --has_trainer--> Trainer
Location --requires--> HMMove     (to access)
Location --in_region--> Region

Item --enables--> Evolution
Item --boosts--> Type             (held type boosters)
Item --heals--> StatusCondition
Item --sold_at--> Location

StoryEvent --requires--> StoryEvent  (prerequisite chain)
StoryEvent --sets--> Flag
StoryEvent --occurs_at--> Location
StoryEvent --involves--> Trainer
StoryEvent --grants--> Item

Type --super_effective_against--> Type
Type --not_effective_against--> Type
Type --immune_to--> Type
Type --category--> PhysicalOrSpecial  (Gen 2 split)
```

### Relationship Cardinality
```
Pokemon : Move          = many-to-many (a Pokemon learns many moves; a move is learned by many Pokemon)
Pokemon : Type          = one-to-many (1-2 types per Pokemon)
Pokemon : Location      = many-to-many (multiple Pokemon per route, Pokemon on multiple routes)
Trainer : Pokemon       = one-to-many (per battle instance)
Location : Location     = many-to-many (bidirectional connections)
Move : MoveEffect       = many-to-one (many moves share EFFECT_NORMAL_HIT)
StoryEvent : StoryEvent = one-to-many (prerequisite chains)
```

### Inverse Relationship Lookup

For every forward relationship, store the inverse for bidirectional traversal:

| Forward | Inverse |
|---|---|
| Pokemon learns Move | Move learned_by Pokemon |
| Pokemon evolves_to Pokemon | Pokemon evolves_from Pokemon |
| Pokemon found_at Location | Location has_encounter Pokemon |
| Trainer uses Pokemon | Pokemon used_by Trainer |
| Trainer located_at Location | Location has_trainer Trainer |
| Item enables Evolution | Evolution requires Item |
| StoryEvent requires StoryEvent | StoryEvent unlocks StoryEvent |

---

## 5. Entity ID Reference Tables

### Type IDs
```
normal, fire, water, electric, grass, ice, fighting, poison,
ground, flying, psychic, bug, rock, ghost, dragon, dark, steel
```
(17 types total — matches pokecrystal constants)

### Trainer Class IDs
```
gym_leader, elite_four, champion, rival,
rocket_grunt, rocket_executive,
ace_trainer, beauty, bird_keeper, blackbelt, bug_catcher,
burglar, camper, cooltrainer, fisher, gentleman,
hiker, juggler, lass, medium, picnicker,
pokefan, pokemaniac, psychic_trainer, sage, sailor,
schoolboy, scientist, skier, super_nerd, swimmer,
teacher, twins, youngster
```

### Region / Map Group IDs
```
# Johto
new_bark_town, cherrygrove_city, violet_city, azalea_town,
goldenrod_city, ecruteak_city, olivine_city, cianwood_city,
mahogany_town, blackthorn_city,
route_29, route_30, route_31, route_32, route_33,
route_34, route_35, route_36, route_37, route_38,
route_39, route_40, route_41, route_42, route_43,
route_44, route_45, route_46,
sprout_tower, ruins_of_alph, union_cave, slowpoke_well,
ilex_forest, national_park, burned_tower, tin_tower,
whirl_islands, mt_mortar, lake_of_rage, ice_path,
dragons_den, dark_cave, silver_cave,
# Kanto
pallet_town, viridian_city, pewter_city, cerulean_city,
vermilion_city, lavender_town, celadon_city, fuchsia_city,
saffron_city, cinnabar_island, indigo_plateau,
route_1, route_2, ..., route_28,
mt_moon, rock_tunnel, power_plant, seafoam_islands,
pokemon_tower, victory_road, digletts_cave, tohjo_falls
```
