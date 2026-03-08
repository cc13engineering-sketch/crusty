# Pokemon Crystal - Special Encounters

All non-standard Pokemon encounters: legendaries, gifts, trades, prizes, and one-time events.

Source: pokecrystal disassembly (maps/*.asm, data/events/npc_trades.asm, constants/).

---

## Legendary Pokemon

### Suicune (Crystal Storyline)

**Location:** Multiple encounters, final battle at Tin Tower
**Level:** 40
**Mechanics:** In Pokemon Crystal (unlike Gold/Silver), Suicune has a unique storyline. You encounter it at:
1. Burned Tower (flees, alongside Raikou and Entei)
2. Cianwood City (cutscene, flees)
3. Route 42 (cutscene, flees)
4. Vermilion City (cutscene, flees)
5. Route 14 (cutscene, flees)
6. **Tin Tower B1F** (static battle, Lv40) — requires Clear Bell + all 3 Legendary Beasts seen

**Catch Rate:** 3 (hardest tier)
**Moves:** Rain Dance, Gust, Aurora Beam, Mist
**Held Item:** None

### Raikou (Roaming)

**Location:** Roaming Johto (after Burned Tower event)
**Level:** 40
**Mechanics:** Appears randomly in Johto grass areas. Flees immediately on encounter. Track via Pokedex after first sighting. Its HP and status persist between encounters.
- Changes route every time you change routes
- Cannot encounter it on water or in caves
- If KOed, it is gone forever (no respawn)

**Catch Rate:** 3
**Moves:** Roar, Thunder Shock, Reflect, Crunch
**Tips:** Use Mean Look/Spider Web to trap, or use a Master Ball. False Swipe + Sleep is optimal.

### Entei (Roaming)

**Location:** Roaming Johto (after Burned Tower event)
**Level:** 40
**Mechanics:** Same roaming mechanics as Raikou.

**Catch Rate:** 3
**Moves:** Roar, Fire Spin, Stomp, Flamethrower
**Tips:** Same strategy as Raikou. Roar will force you out, so use a faster Pokemon with Mean Look.

### Lugia

**Location:** Whirl Islands B2F
**Level:** 60
**Requirements:** Silver Wing (from Radio Tower Director after Team Rocket event)
**Catch Rate:** 3
**Moves:** Aeroblast, Safeguard, Gust, Recover

### Ho-Oh

**Location:** Tin Tower 7F
**Level:** 60
**Requirements:** Rainbow Wing (from Radio Tower Director after Team Rocket event, Crystal only) + Clear Bell
**Catch Rate:** 3
**Moves:** Sacred Fire, Safeguard, Gust, Recover

### Celebi (GS Ball Event)

**Location:** Ilex Forest Shrine
**Level:** 30
**Requirements:** GS Ball (originally Japan-only mobile event; available on Virtual Console worldwide)
**Mechanics:** Take GS Ball to Kurt in Azalea Town. After one day, he returns it. Place it on the shrine in Ilex Forest. Celebi appears.
**Catch Rate:** 45 (much easier than other legendaries)
**Moves:** Leech Seed, Recover, Heal Bell, Safeguard

---

## Roaming Mechanics (Raikou & Entei)

- Both are released when you fall through the Burned Tower floor
- They roam Johto routes (grass encounters only)
- Each time you enter a new route, they move to a random adjacent route
- If you enter their current route, there's a chance to encounter them in grass
- They always attempt to flee on the first turn (Roar forces you out too)
- HP, status, and catch attempts persist across encounters
- If you KO either one, they disappear permanently
- Using Fly/Teleport to a route they're on works — they don't move when you Fly
- The Pokedex map feature shows their current location after first sighting

---

## Gift Pokemon

### Starter Pokemon
**Location:** New Bark Town (Professor Elm's Lab)
**Level:** 5
**Choose one:**
- **Cyndaquil** — Fire type
- **Totodile** — Water type
- **Chikorita** — Grass type

### Togepi Egg
**Location:** Violet City Pokemon Center
**Given by:** Mystery Egg from Mr. Pokemon (Route 30), hatched by Professor Elm's aide
**Level:** 5 (when hatched)
**Egg moves vary:** Togepi hatches knowing Growl, Charm, and 2 random moves from parents (but this is the scripted egg)

### Eevee
**Location:** Goldenrod City (Bill's house, after meeting him in Ecruteak)
**Level:** 20
**Note:** Only one per save file. Choose evolution wisely.

### Shuckle (Mania)
**Location:** Cianwood City (Kirk/Mania's house)
**Level:** 15
**Held Item:** Berry
**Note:** He asks you to take care of it. He may ask for it back later — say no to keep it.

### Spearow (Kenya)
**Location:** Route 35 gate
**Given by:** Guard in the gate
**Level:** 10
**Held Item:** Mail (letter to deliver to Route 31)
**OT:** Webster
**Note:** Deliver the mail to the guy on Route 31, or keep Kenya. If you remove the mail, you can keep the Spearow but can't deliver the letter.

### Tyrogue
**Location:** Mt. Mortar (deep interior)
**Given by:** Blackbelt Kiyo (after defeating him)
**Level:** 10
**Note:** Evolves at Level 20 based on Atk vs Def stats: Hitmonlee (Atk > Def), Hitmonchan (Atk < Def), Hitmontop (Atk = Def)

### Dratini
**Location:** Dragon's Den (after answering Elder's quiz correctly)
**Level:** 15
**Special:** If you answer ALL questions correctly, the Dratini knows ExtremeSpeed (the only way to get ExtremeSpeed Dratini in the game)
**Note:** The "correct" answers are the ones about caring for Pokemon, not power

### Red Gyarados
**Location:** Lake of Rage
**Level:** 30
**Type:** Water/Flying
**Note:** Always Shiny (guaranteed). Static encounter — battle or flee, but it's unique. Gives Red Scale when caught (trade to Mr. Pokemon for Exp. Share).

### Sudowoodo
**Location:** Route 36 (blocking the path)
**Level:** 20
**Requirements:** SquirtBottle from Goldenrod flower shop (need Whitney badge)
**Note:** One per save file. Use SquirtBottle to make it attack.

### Snorlax
**Location:** Route 11 (Kanto, blocking Diglett's Cave entrance)
**Level:** 50
**Requirements:** PokeFlute channel on the radio (from Lavender Tower upgrade)
**Note:** One per save file. Tune the radio to the PokeFlute channel while standing next to it.

### Odd Egg (Crystal Exclusive)
**Location:** Pokemon Communication Center (replaces Goldenrod Pokemon Center 2F in Crystal)
**Level:** 5 (when hatched)
**Contains one of:** Pichu, Cleffa, Igglybuff, Tyrogue, Smoochum, Elekid, Magby
**Special:** 14% chance of being Shiny (compared to normal 1/8192)

---

## In-Game Trades

All NPC trades in Pokemon Crystal. Data from pokecrystal npc_trades.asm.

| You Give | You Receive | Nickname | Held Item | OT Name | Location |
|----------|------------|----------|-----------|---------|----------|
| Abra | Machop | MUSCLE | Gold Berry | MIKE | Goldenrod Dept Store 5F |
| Bellsprout | Onix | ROCKY | Bitter Berry | KYLE | Violet City (house near gym) |
| Krabby | Voltorb | VOLTY | PrzCureBerry | TIM | Olivine City |
| Dragonair | Dodrio | DORIS | Smoke Ball | EMY | Blackthorn City (female player only) |
| Haunter | Xatu | PAUL | MysteryBerry | CHRIS | Cianwood City |
| Chansey | Aerodactyl | AEROY | Gold Berry | KIM | Route 14 |
| Dugtrio | Magneton | MAGGIE | Metal Coat | FOREST | Power Plant |

**Notes:**
- Trade #4 (Dragonair -> Dodrio) requires a female player character
- Traded Pokemon gain boosted EXP (1.5x)
- All traded Pokemon have preset DVs and OT IDs

---

## Game Corner Prizes (Goldenrod)

### Pokemon Prizes (Counter)
| Pokemon | Cost (Coins) | Level |
|---------|-------------|-------|
| Abra | 100 | 5 |
| Cubone | 800 | 15 |
| Wobbuffet | 1500 | 15 |

### TM Prizes
| TM | Move | Cost (Coins) |
|----|------|--------------|
| TM25 | Thunder | 5500 |
| TM14 | Blizzard | 5500 |
| TM38 | Fire Blast | 5500 |
| TM32 | Double Team | 1500 |
| TM29 | Psychic | 3500 |
| TM15 | Hyper Beam | 7500 |

---

## Bug Catching Contest Prizes

**Location:** National Park (Tuesday, Thursday, Saturday)
**Time:** 9:00 AM - 6:00 PM
**Rules:** 20 Sport Balls, 20 minutes, keep one caught Pokemon

### Prizes by Placement
| Place | Prize |
|-------|-------|
| 1st | Sun Stone |
| 2nd | Everstone |
| 3rd | Gold Berry |
| Runner-up | Berry |

### Available Pokemon (Contest Area Only)
| Pokemon | Level | Rarity |
|---------|-------|--------|
| Caterpie | 7-18 | Common |
| Metapod | 9-18 | Common |
| Butterfree | 12-15 | Uncommon |
| Weedle | 7-18 | Common |
| Kakuna | 9-18 | Common |
| Beedrill | 12-15 | Uncommon |
| Paras | 10-18 | Uncommon |
| Venonat | 10-16 | Uncommon |
| Scyther | 13-14 | Rare |
| Pinsir | 13-14 | Rare |

**Scoring:** Higher HP, level, and rarity = more points. Scyther and Pinsir almost always win 1st place if caught.

---

## Celadon Game Corner Prizes (Kanto)

### Pokemon Prizes
| Pokemon | Cost (Coins) |
|---------|-------------|
| Mr. Mime | 3333 |
| Eevee | 6666 |
| Porygon | 9999 |

### TM Prizes
| TM | Move | Cost (Coins) |
|----|------|--------------|
| TM25 | Thunder | 5500 |
| TM14 | Blizzard | 5500 |
| TM38 | Fire Blast | 5500 |

---

## Move Tutors

Three one-time move tutors teach powerful moves for free:

| Move | Location | Available |
|------|----------|-----------|
| Flamethrower | Goldenrod City Game Corner (man at counter) | Wednesday/Saturday |
| Thunderbolt | Goldenrod City Game Corner (man at counter) | Wednesday/Saturday |
| Ice Beam | Goldenrod City Game Corner (man at counter) | Wednesday/Saturday |

**Note:** Each tutor can only teach one Pokemon per playthrough. Choose wisely.

---

## Unique One-Time Pokemon

### Lapras
**Location:** Union Cave B2F
**Level:** 20
**Available:** Friday only (Surf required)
**Note:** Respawns every Friday if KOed or fled

### Electrode (Rocket HQ)
**Location:** Team Rocket HQ, Mahogany Town (3 Electrodes guarding generators)
**Level:** 23
**Note:** You must battle them to destroy the generators. Can catch or KO.

---

## Version Exclusives

### Crystal-Only Content (vs Gold/Silver)
- Suicune storyline (static encounter instead of roaming)
- Eusine character (Suicune researcher)
- Battle Tower in Olivine City
- Buena's Password (Radio Tower, daily)
- Odd Egg from Communication Center
- Celebi event (GS Ball, originally mobile-only)
- Move Tutors (Flamethrower, Thunderbolt, Ice Beam)
- Animated Pokemon sprites (front sprites animate in battle)
- Expanded Ruins of Alph events
- New Pokemon availability changes (e.g., Phanpy/Teddiursa availability differs)

### Gold-Only Pokemon (not in Crystal wild)
- Mankey, Primeape (Route 42)
- Growlithe (Route 35, 36, 37)
- Spinarak, Ariados (Route 2, 30, 31, 37)
- Teddiursa, Ursaring (Route 45, Mt. Silver)
- Gligar (Route 45)
- Mantine (Route 41)

### Silver-Only Pokemon (not in Crystal wild)
- Vulpix, Ninetales (Route 36, 37)
- Meowth, Persian (Route 39)
- Ledyba, Ledian (Route 2, 30, 31, 37)
- Phanpy, Donphan (Route 45, Mt. Silver)
- Skarmory (Route 45)
- Delibird (Ice Path)
