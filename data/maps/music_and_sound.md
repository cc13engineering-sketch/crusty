# Pokemon Crystal -- Complete Music and Sound Reference

Source: pokecrystal constants/music_constants.asm, constants/sfx_constants.asm, audio/ directory

---

## Complete Music Track Listing

103 music tracks total (indices $00-$66).

### Overworld -- Towns
| ID | Constant | Used For |
|----|----------|----------|
| $3C | MUSIC_NEW_BARK_TOWN | New Bark Town |
| $26 | MUSIC_CHERRYGROVE_CITY | Cherrygrove City, Mahogany Town |
| $2D | MUSIC_VIOLET_CITY | Violet City, Olivine City |
| $25 | MUSIC_AZALEA_TOWN | Azalea Town, Blackthorn City |
| $3D | MUSIC_GOLDENROD_CITY | Goldenrod City |
| $2C | MUSIC_ECRUTEAK_CITY | Ecruteak City, Cianwood City |
| $45 | MUSIC_LAKE_OF_RAGE | Lake of Rage, Routes 42-44 |
| $1C | MUSIC_PALLET_TOWN | Pallet Town |
| $15 | MUSIC_VIRIDIAN_CITY | Viridian, Pewter, Cerulean, Saffron, Cinnabar |
| $3E | MUSIC_VERMILION_CITY | Vermilion City |
| $16 | MUSIC_CELADON_CITY | Celadon City, Fuchsia City |
| $0E | MUSIC_LAVENDER_TOWN | Lavender Town |
| $46 | MUSIC_INDIGO_PLATEAU | Indigo Plateau, Route 28 |

### Overworld -- Routes
| ID | Constant | Used For |
|----|----------|----------|
| $34 | MUSIC_ROUTE_29 | Route 29 |
| $2B | MUSIC_ROUTE_30 | Routes 30, 31, 32, 33 |
| $35 | MUSIC_ROUTE_36 | Routes 34-37, 40-41, 45-46 |
| $47 | MUSIC_ROUTE_37 | Routes 38, 39 |
| $4D | MUSIC_ROUTE_26 | Routes 26, 27 |
| $02 | MUSIC_ROUTE_1 | Kanto Routes 1, 2 |
| $03 | MUSIC_ROUTE_3 | Kanto Routes 3-10 |
| $04 | MUSIC_ROUTE_12 | Kanto Routes 11-25 |

### Overworld -- Dungeons/Caves
| ID | Constant | Used For |
|----|----------|----------|
| $33 | MUSIC_DARK_CAVE | Dark Cave, Union Cave, Slowpoke Well |
| $28 | MUSIC_UNION_CAVE | (alternate cave theme) |
| $10 | MUSIC_MT_MOON | Mt. Moon, Mt. Mortar, Rock Tunnel |
| $42 | MUSIC_SPROUT_TOWER | Sprout Tower |
| $43 | MUSIC_BURNED_TOWER | Burned Tower |
| $41 | MUSIC_TIN_TOWER | Tin Tower |
| $44 | MUSIC_LIGHTHOUSE | Olivine Lighthouse |
| $55 | MUSIC_RUINS_OF_ALPH_INTERIOR | Ruins of Alph inside |
| $49 | MUSIC_DRAGONS_DEN | Dragon's Den |
| $48 | MUSIC_ROCKET_HIDEOUT | Team Rocket Hideout |
| $4F | MUSIC_VICTORY_ROAD | Victory Road |

### Overworld -- Special
| ID | Constant | Used For |
|----|----------|----------|
| $09 | MUSIC_POKEMON_CENTER | All Pokemon Centers |
| $1B | MUSIC_GYM | All Gyms |
| $12 | MUSIC_GAME_CORNER | Goldenrod Game Corner |
| $36 | MUSIC_SS_AQUA | SS Aqua |
| $05 | MUSIC_MAGNET_TRAIN | Magnet Train |
| $13 | MUSIC_BICYCLE | Bicycle riding |
| $21 | MUSIC_SURF | Surfing |
| $57 | MUSIC_DANCING_HALL | Ecruteak Dance Hall |
| $23 | MUSIC_NATIONAL_PARK | National Park |
| $1A | MUSIC_MT_MOON_SQUARE | Mt. Moon Square (Clefairy dance) |
| $56 | MUSIC_ROCKET_OVERTURE | Radio Tower occupied by Rockets |
| $65 | MUSIC_BATTLE_TOWER_LOBBY | Battle Tower lobby |

### Battle Music
| ID | Constant | Used For |
|----|----------|----------|
| $29 | MUSIC_JOHTO_WILD_BATTLE | Johto wild Pokemon |
| $4A | MUSIC_JOHTO_WILD_BATTLE_NIGHT | Johto wild Pokemon (night) |
| $08 | MUSIC_KANTO_WILD_BATTLE | Kanto wild Pokemon |
| $2A | MUSIC_JOHTO_TRAINER_BATTLE | Johto trainers |
| $07 | MUSIC_KANTO_TRAINER_BATTLE | Kanto trainers |
| $2E | MUSIC_JOHTO_GYM_LEADER_BATTLE | Johto gym leaders |
| $06 | MUSIC_KANTO_GYM_LEADER_BATTLE | Kanto gym leaders |
| $30 | MUSIC_RIVAL_BATTLE | Rival battles |
| $31 | MUSIC_ROCKET_BATTLE | Team Rocket Grunts (NOT Executives -- bug!) |
| $2F | MUSIC_CHAMPION_BATTLE | Champion (Lance/Red) |
| $64 | MUSIC_SUICUNE_BATTLE | Suicune |
| $63 | MUSIC_BATTLE_TOWER_THEME | Battle Tower |

### Victory Fanfares
| ID | Constant | Used For |
|----|----------|----------|
| $17 | MUSIC_TRAINER_VICTORY | Trainer defeated |
| $18 | MUSIC_WILD_VICTORY | Wild Pokemon caught/defeated |
| $19 | MUSIC_GYM_VICTORY | Gym leader defeated |

### Encounter Themes (brief stings)
| ID | Constant | Used For |
|----|----------|----------|
| $0A | MUSIC_HIKER_ENCOUNTER | Hiker |
| $0B | MUSIC_LASS_ENCOUNTER | Lass, Beauty |
| $0C | MUSIC_OFFICER_ENCOUNTER | Officer Jenny |
| $37 | MUSIC_YOUNGSTER_ENCOUNTER | Youngster, Bug Catcher |
| $38 | MUSIC_BEAUTY_ENCOUNTER | Beauty, Cooltrainer |
| $39 | MUSIC_ROCKET_ENCOUNTER | Team Rocket |
| $3A | MUSIC_POKEMANIAC_ENCOUNTER | Pokemaniac |
| $3B | MUSIC_SAGE_ENCOUNTER | Sage, Medium |
| $27 | MUSIC_KIMONO_ENCOUNTER | Kimono Girl |
| $1F | MUSIC_RIVAL_ENCOUNTER | Rival encounter |
| $61 | MUSIC_MYSTICALMAN_ENCOUNTER | Eusine encounter |

### Special/Story Music
| ID | Constant | Used For |
|----|----------|----------|
| $01 | MUSIC_TITLE | Title screen |
| $54 | MUSIC_MAIN_MENU | Main menu |
| $62 | MUSIC_CRYSTAL_OPENING | Crystal intro |
| $52 | MUSIC_GS_OPENING | Gold/Silver opening |
| $0D | MUSIC_HEAL | Pokemon Center heal |
| $22 | MUSIC_EVOLUTION | Evolution |
| $4C | MUSIC_CAPTURE | Pokemon caught |
| $14 | MUSIC_HALL_OF_FAME | Hall of Fame |
| $24 | MUSIC_CREDITS | Credits roll |
| $5C | MUSIC_POST_CREDITS | Post-credits |
| $1E | MUSIC_PROF_OAK | Prof. Oak theme |
| $32 | MUSIC_PROF_ELM | Prof. Elm theme |
| $4E | MUSIC_MOM | Mom theme |
| $20 | MUSIC_RIVAL_AFTER | After rival battle |
| $11 | MUSIC_SHOW_ME_AROUND | Tour guide theme |
| $5D | MUSIC_CLAIR | Clair's theme |

### Radio Music
| ID | Constant | Used For |
|----|----------|----------|
| $1D | MUSIC_POKEMON_TALK | Pokemon Talk radio |
| $3F | MUSIC_POKEMON_CHANNEL | Pokemon Channel |
| $40 | MUSIC_POKE_FLUTE_CHANNEL | Poke Flute Channel |
| $50 | MUSIC_POKEMON_LULLABY | Pokemon Lullaby |
| $51 | MUSIC_POKEMON_MARCH | Pokemon March |
| $4B | MUSIC_RUINS_OF_ALPH_RADIO | Unown radio signal |
| $5A | MUSIC_LAKE_OF_RAGE_ROCKET_RADIO | Rocket takeover radio |
| $60 | MUSIC_BUENAS_PASSWORD | Buena's Password |

### Contest/Event
| ID | Constant | Used For |
|----|----------|----------|
| $59 | MUSIC_BUG_CATCHING_CONTEST | Bug Catching Contest |
| $58 | MUSIC_BUG_CATCHING_CONTEST_RANKING | Contest results |

### Mobile Adapter (Crystal Japan)
| ID | Constant | Used For |
|----|----------|----------|
| $5E | MUSIC_MOBILE_ADAPTER_MENU | Mobile adapter menu |
| $5F | MUSIC_MOBILE_ADAPTER | Mobile adapter |
| $66 | MUSIC_MOBILE_CENTER | Mobile center |
| $5B | MUSIC_PRINTER | GB Printer |

---

## Audio Engine Architecture

### Channels
- Channels 1-4: Music (Square 1, Square 2, Wave, Noise)
- Channels 5-8: Sound effects (same hardware, override music)

### Tempo System
- Tempo value stored as 16-bit number
- BPM = 19200 / tempo_value
- Common tempo 150 BPM = tempo value 128
- Note lengths are in "ticks"; a tick's duration depends on tempo and note_type speed

### Duty Cycles (Square channels 1-2)
- 0 = 12.5% (_______.) -- thin, hollow
- 1 = 25% (______.. ) -- medium
- 2 = 50% (____....) -- classic square
- 3 = 75% (__......) -- same as 25%

### Wave Channel (3)
- 32 4-bit samples define the waveform
- 16 predefined wave instruments in audio/wave_samples.asm
- Volume: 0=mute, 1=100%, 2=50%, 3=25%

### Noise Channel (4)
- Used for drums and percussion
- 6 drum kits available (toggle_noise 0-5)
- 12 drum instruments per kit

### Pokemon Cries
- Based on music tracks with modified pitch and length
- wCryPitch and wCryLength control the transformation
- Each species has a base cry + pitch/length parameters in data/pokemon/cries.asm

---

## Sound Effects Catalog (Key SFX)

### UI/Menu
| ID | Constant | Description |
|----|----------|-------------|
| $06 | SFX_MENU | Menu cursor move |
| $07 | SFX_READ_TEXT | Text printing sound |
| $25 | SFX_SAVE | Save game |
| $01 | SFX_ITEM | Item obtained |
| $04 | SFX_POTION | Use potion |
| $05 | SFX_FULL_HEAL | Use Full Heal |
| $19 | SFX_WRONG | Error/wrong input |

### Battle
| ID | Constant | Description |
|----|----------|-------------|
| $28 | SFX_THROW_BALL | Throw Poke Ball |
| $29 | SFX_BALL_POOF | Ball opens/closes |
| $2A | SFX_FAINT | Pokemon faints |
| $2B | SFX_RUN | Run from battle |
| $2E | SFX_PECK | Peck attack |
| $31 | SFX_POUND | Pound attack |
| $34 | SFX_MEGA_PUNCH | Strong punch |
| $38 | SFX_CUT | Cut attack |
| $3F | SFX_HEADBUTT | Headbutt |
| $41 | SFX_TACKLE | Tackle |
| $4D | SFX_THUNDER | Thunder |
| $50 | SFX_EMBER | Ember |
| $52 | SFX_HYDRO_PUMP | Hydro Pump |
| $53 | SFX_SURF | Surf |
| $57 | SFX_PSYCHIC | Psychic |
| $5D | SFX_HYPER_BEAM | Hyper Beam |

### Field
| ID | Constant | Description |
|----|----------|-------------|
| $16 | SFX_JUMP_OVER_LEDGE | Ledge hop |
| $17 | SFX_GRASS_RUSTLE | Enter tall grass |
| $18 | SFX_FLY | Fly HM |
| $1B | SFX_STRENGTH | Push boulder |
| $1F | SFX_ENTER_DOOR | Enter building |
| $23 | SFX_EXIT_BUILDING | Exit building |
| $24 | SFX_BUMP | Walk into wall |
| $10 | SFX_ESCAPE_ROPE | Use Escape Rope |

### Special
| ID | Constant | Description |
|----|----------|-------------|
| $02 | SFX_CAUGHT_MON | Caught Pokemon |
| $0D | SFX_BOOT_PC | Turn on PC |
| $0E | SFX_SHUT_DOWN_PC | Turn off PC |
| $2C | SFX_SLOT_MACHINE_START | Slot machine |
| $2D | SFX_FANFARE | General fanfare |
| $5E | SFX_SHINE | Shiny/sparkle |

---

## Audio Bugs (from docs)

1. **Slot machine payout SFX cut off**: Inverted check plays sound every frame instead of every 8th frame
2. **Rocket battle music missing**: Executives and Scientists don't trigger MUSIC_ROCKET_BATTLE
3. **No bump noise on tile $3E**: Standing direction indexes into edge warp table incorrectly
4. **Entei cry distortion**: Playing Entei's Pokedex cry can distort Raikou/Suicune cries
5. **SFX_RUN incorrect**: Uses PlaySFX instead of WaitPlaySFX for wild flee
