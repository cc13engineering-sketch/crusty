# Pokemon Crystal - Radio System

All radio stations, frequencies, content, and mechanics. Source: pokecrystal radio_constants.asm, radio.asm, buenas_passwords.asm.

---

## Overview

The Radio is a Pokegear feature unlocked by obtaining the Radio Card from the Goldenrod Radio Tower quiz. Stations are accessed via a dial interface. Johto stations are available in Johto; Kanto stations require the EXPN Card upgrade from Lavender Radio Tower.

---

## Radio Stations

### 1. Oak's Pokemon Talk (OAKS_POKEMON_TALK)
**Availability:** Johto (Radio Card)
**Music:** MUSIC_POKEMON_TALK
**Content:** Professor Oak and DJ Mary discuss Pokemon found on various routes. They describe a random Pokemon with random adverbs and adjectives.

**Routes discussed:**
Route 29, 46, 30, 32, 34, 35, 37, 38, 39, 42, 43, 44, 45, 36, 31

**Format:** "OAK: [Pokemon] is a [adverb] [adjective] Pokemon!" with randomized word selection from pools of 16 adverbs and 16 adjectives.

**Gameplay use:** Tells you which Pokemon can be found on which routes. Useful for tracking wild encounters.

---

### 2. Pokedex Show (POKEDEX_SHOW)
**Availability:** Johto (Radio Card)
**Music:** MUSIC_POKEMON_CENTER
**Content:** DJ Ben describes random Pokemon from the Pokedex with their Pokedex entries and classification.

---

### 3. Pokemon Music (POKEMON_MUSIC)
**Availability:** Johto (Radio Card)
**Music:** MUSIC_TITLE
**Content:** Pokemon-themed music channel. Plays the game's title theme as background music. DJ Fern hosts.

**Special:** After obtaining the EXPN Card, this channel can also play:
- **Pokemon March** (MUSIC_POKEMON_MARCH) — plays during the day, increases wild encounter rate
- **Pokemon Lullaby** (MUSIC_POKEMON_LULLABY) — plays at night, decreases wild encounter rate

---

### 4. Lucky Channel (LUCKY_CHANNEL)
**Availability:** Johto (Radio Card)
**Music:** MUSIC_GAME_CORNER
**Content:** Reed hosts a weekly lottery. A random 5-digit number is drawn every Friday. Match digits from the RIGHT of your Pokemon's OT ID numbers.

**Prizes:**
| Matching Digits | Prize |
|----------------|-------|
| Last 2 digits | PP Up |
| Last 3 digits | Exp. Share |
| Last 4 digits | Max Revive |
| All 5 digits | Master Ball |

**Tips:** Trade Pokemon with many different OT IDs to maximize your chances.

---

### 5. Buena's Password (BUENAS_PASSWORD) [Crystal Only]
**Availability:** Johto, Crystal only
**Music:** MUSIC_BUENAS_PASSWORD
**Time:** 6:00 PM - 12:00 AM (night only)
**Content:** Buena gives a password from one of 11 categories with 3 options each. Go to Radio Tower 2F and tell her the password to earn Blue Card points.

**Password Categories and Options:**

| Category | Option 1 | Option 2 | Option 3 | Points |
|----------|----------|----------|----------|--------|
| Johto Starters | Cyndaquil | Totodile | Chikorita | 10 |
| Beverages | Fresh Water | Soda Pop | Lemonade | 12 |
| Healing Items | Potion | Antidote | Parlyz Heal | 12 |
| Balls | Poke Ball | Great Ball | Ultra Ball | 12 |
| Pokemon 1 | Pikachu | Rattata | Geodude | 10 |
| Pokemon 2 | Hoothoot | Spinarak | Drowzee | 10 |
| Johto Towns | New Bark Town | Cherrygrove City | Azalea Town | 16 |
| Types | Flying | Bug | Grass | 6 |
| Moves | Tackle | Growl | Mud-Slap | 12 |
| X Items | X Attack | X Defend | X Speed | 12 |
| Radio Stations | Pokemon Talk | Pokemon Music | Lucky Channel | 13 |

**Blue Card Prize Exchange:**

| Item | Points Required |
|------|----------------|
| Ultra Ball | 2 |
| Full Restore | 2 |
| Nugget | 3 |
| Rare Candy | 3 |
| Protein | 5 |
| Iron | 5 |
| Carbos | 5 |
| Calcium | 5 |
| HP Up | 5 |

---

### 6. Places and People (PLACES_AND_PEOPLE)
**Availability:** Johto (Radio Card)
**Music:** MUSIC_VIRIDIAN_CITY
**Content:** Lily hosts a show describing locations and notable trainers in Kanto. Random places and people are featured with adjective descriptors.

**Places featured:**
Pallet Town, Route 22, Pewter City, Cerulean City, Route 12, Route 11, Route 16, Route 14, Cinnabar Island

**Hidden people (NOT discussed until conditions met):**
- Elite Four (Will, Bruno, Karen, Koga, Lance) — hidden until beaten
- Kanto Gym Leaders (Brock, Misty, Lt. Surge, Erika, Janine, Sabrina, Blaine, Blue) — hidden until E4 beaten
- Special characters (Rival, Professor Oak, Cal, Red) — hidden until Kanto beaten

---

### 7. Let's All Sing (LETS_ALL_SING)
**Availability:** Johto (Radio Card)
**Music:** MUSIC_BICYCLE
**Content:** Sing-along music show.

---

### 8. Team Rocket Radio (ROCKET_RADIO)
**Availability:** Replaces normal stations during Radio Tower takeover event
**Music:** MUSIC_ROCKET_OVERTURE
**Content:** Team Rocket propaganda broadcast during the Goldenrod Radio Tower takeover. Calls for Giovanni to return. Cannot be turned off during the event.

**Special behavior:** When the Radio Tower is occupied by Team Rocket, the RADIO_TOWER_MUSIC flag overrides normal music with the Rocket broadcast music. All Radio Tower floors use this special flag.

---

### 9. PokeFlute Radio (POKE_FLUTE_RADIO)
**Availability:** Kanto (EXPN Card required)
**Music:** MUSIC_POKE_FLUTE_CHANNEL
**Content:** Plays the PokeFlute melody.

**Gameplay use:** Required to wake up Snorlax blocking the entrance to Diglett's Cave on Route 11. Stand next to Snorlax and tune to this channel.

---

### 10. Unown Radio (UNOWN_RADIO)
**Availability:** Only inside Ruins of Alph
**Music:** MUSIC_RUINS_OF_ALPH_RADIO
**Content:** Mysterious static/signal that can only be received inside the Ruins of Alph. No dialogue — just atmospheric sound.

**Gameplay use:** Atmospheric only. No direct gameplay effect. Adds to the mysterious atmosphere of the ruins.

---

### 11. Evolution Radio (EVOLUTION_RADIO)
**Availability:** Special (related to Lake of Rage forced evolution signal)
**Music:** MUSIC_LAKE_OF_RAGE_ROCKET_RADIO
**Content:** The Rocket radio signal that was forcing Magikarp to evolve at Lake of Rage. Part of the Team Rocket Mahogany Town storyline.

---

## Radio Card Acquisition

### Johto Radio Card
**Location:** Goldenrod Radio Tower 1F
**How:** Answer the receptionist's quiz correctly (5 questions about Pokemon)
**Unlocks:** All Johto radio stations

### EXPN Card (Kanto Expansion)
**Location:** Lavender Radio Tower
**How:** Fix the Power Plant generator (Machine Part quest), then talk to the manager in Lavender
**Unlocks:** Kanto-specific stations (PokeFlute) and enhanced Pokemon Music channel features

---

## Technical Details

- 11 radio channels total (NUM_RADIO_CHANNELS = 11)
- Channels have multiple internal segments for dialogue progression
- Radio plays its channel music, replacing overworld map music
- When exiting the radio, map music resumes (RESTART_MAP_MUSIC = 0xFE)
- The Radio Tower has a special bit flag (RADIO_TOWER_MUSIC) that overrides music during the Team Rocket event
