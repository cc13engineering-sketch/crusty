# Pokemon Crystal -- Evolution Mechanics

Source: pokecrystal disassembly (engine/pokemon/evolve.asm), Bulbapedia.

---

## Evolution Methods in Gen 2

Five evolution methods exist, each handled by a specific code path in EvolvePokemon:

### 1. EVOLVE_LEVEL -- Level Up

The Pokemon evolves when it reaches a specific level.

```
condition: wTempMonLevel >= required_level
```

Checked on every level-up event. If the condition is met and the Pokemon isn't holding an Everstone, evolution proceeds.

**Examples**: Charmander -> Charmeleon (16), Charmeleon -> Charizard (36), Pidgey -> Pidgeotto (18)

### 2. EVOLVE_ITEM -- Evolutionary Stone

The Pokemon evolves when a specific item is used on it.

```
condition: wForceEvolution is set AND item matches
```

Stone evolutions are triggered by using the stone from the bag. They do NOT trigger on level-up.

**Stones and Pokemon**:
- Fire Stone: Vulpix -> Ninetales, Growlithe -> Arcanine, Eevee -> Flareon
- Water Stone: Poliwhirl -> Poliwrath, Shellder -> Cloyster, Staryu -> Starmie, Eevee -> Vaporeon
- Thunder Stone: Pikachu -> Raichu, Eevee -> Jolteon
- Leaf Stone: Gloom -> Vileplume, Weepinbell -> Victreebel, Exeggcute -> Exeggutor
- Moon Stone: Nidorina -> Nidoqueen, Nidorino -> Nidoking, Clefairy -> Clefable, Jigglypuff -> Wigglytuff
- Sun Stone: Gloom -> Bellossom, Sunkern -> Sunflora

### 3. EVOLVE_TRADE -- Trade Evolution

The Pokemon evolves when traded to another game.

```
condition: Trading is occurring
```

**Basic trade**: Kadabra -> Alakazam, Machoke -> Machamp, Graveler -> Golem, Haunter -> Gengar

**Trade with specific item** (the Pokemon must be holding the item when traded):
- Poliwhirl + King's Rock -> Politoed
- Slowpoke + King's Rock -> Slowking
- Onix + Metal Coat -> Steelix
- Scyther + Metal Coat -> Scizor
- Seadra + Dragon Scale -> Kingdra
- Porygon + Up-Grade -> Porygon2

The item is consumed during evolution.

### 4. EVOLVE_HAPPINESS -- Happiness Evolution

The Pokemon evolves when leveled up with high happiness (>= 220).

```
condition: wTempMonLevel increased (any level up) AND happiness >= 220
```

Three sub-variants exist:
- **Anytime**: Chansey -> Blissey (not actually happiness in Gen 2; Chansey evolves by trade with Lucky Punch... wait, no. Let me correct:)

Happiness evolutions:
- **Day only** (4:00 AM - 7:59 PM): Eevee -> Espeon
- **Night only** (8:00 PM - 3:59 AM): Eevee -> Umbreon
- **Anytime**: Golbat -> Crobat, Chansey -> Blissey, Pichu -> Pikachu, Cleffa -> Clefairy, Igglybuff -> Jigglypuff, Togepi -> Togetic

The happiness threshold is 220 (out of 255 maximum).

### 5. EVOLVE_STAT -- Stat-Based Evolution

Only used by Tyrogue:
- Tyrogue -> Hitmonlee: if Attack > Defense at level 20
- Tyrogue -> Hitmonchan: if Attack < Defense at level 20
- Tyrogue -> Hitmontop: if Attack = Defense at level 20

```
condition: level >= 20 AND stat comparison matches
```

---

## Happiness System

### Happiness Range
- 0 to 255 (unsigned byte)
- Default for caught/hatched Pokemon: varies by species (usually 70)
- Friend Ball caught Pokemon start at 200

### Happiness Modifiers

Actions that INCREASE happiness:
| Action | Amount (base) | Amount (happiness < 100) | Amount (100-199) | Amount (200+) |
|--------|---------------|--------------------------|-------------------|---------------|
| Level up | +5 | +5 | +3 | +2 |
| Walking 256 steps | +1 | +1 | +1 | +1 |
| Grooming (Daisy) | +3 | +3 | +3 | +1 |
| Vitamins (HP Up, etc.) | +5 | +5 | +3 | +2 |
| EV berries | +2 | +2 | +2 | +1 |
| Held Soothe Bell | - | Not in Gen 2 | - | - |

Actions that DECREASE happiness:
| Action | Amount |
|--------|--------|
| Fainting | -1 |
| Using bitter medicine (Energy Root, etc.) | -5 to -10 |
| Trading | Reset to base |

### Happiness and Moves
- **Return**: Power = Happiness * 10 / 25 (max 102 at 255 happiness)
- **Frustration**: Power = (255 - Happiness) * 10 / 25 (max 102 at 0 happiness)

---

## Everstone

- If the Pokemon holds an Everstone, evolution is prevented
- The evolution check fails silently (no message)
- Works for level-up, happiness, and stat-based evolutions
- Does NOT prevent trade evolutions or stone evolutions
- Implementation: `call IsMonHoldingEverstone; jp z, .dont_evolve`

---

## Evolution Process

When evolution is triggered:

1. Evolution animation plays
2. Species is changed to the evolved form
3. Stats are recalculated with new base stats (DVs and StatExp preserved)
4. Moves are checked: if the evolved species learns any moves at the current level, they are taught (with the option to replace old moves if all 4 slots are full)
5. Pokedex is updated (new species registered as caught)
6. Type is updated to the evolved form's types

---

## Prevented Evolution

- If the player presses B during the evolution animation, evolution is cancelled
- The Pokemon keeps its current form
- The evolution can be triggered again at the next level-up (or next applicable event)
- Cancelled evolutions do NOT permanently prevent evolution

---

## Baby Pokemon (Gen 2 introductions)

Gen 2 introduced baby Pokemon obtainable only through breeding:
- Pichu (from Pikachu/Raichu)
- Cleffa (from Clefairy/Clefable)
- Igglybuff (from Jigglypuff/Wigglytuff)
- Togepi (given as egg by Professor Elm's aide)
- Tyrogue (from Hitmonlee/Hitmonchan/Hitmontop)
- Smoochum (from Jynx)
- Elekid (from Electabuzz)
- Magby (from Magmar)

These all evolve via happiness (except Tyrogue, which uses stat comparison).
