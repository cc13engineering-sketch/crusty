# Pokemon Crystal - Sprites and Animations

Overworld sprite data and animation system from pokecrystal data/sprites/ and data/sprite_anims/.

Source: `data/sprites/*.asm`, `data/sprite_anims/*.asm`

---

## Overworld Sprites (102 total)

From `data/sprites/sprites.asm`. Each sprite has: graphics pointer, tile count, movement type, and palette.

### Sprite Types
| Type | Behavior |
|------|----------|
| WALKING_SPRITE | Has 4-direction walking frames (12 tiles) |
| STANDING_SPRITE | Has facing frames but no walk animation (12 tiles) |
| STILL_SPRITE | Single static frame (4 tiles) |

### Palettes
| Palette | Color |
|---------|-------|
| PAL_OW_RED | Red tones |
| PAL_OW_BLUE | Blue tones |
| PAL_OW_GREEN | Green tones |
| PAL_OW_BROWN | Brown tones |
| PAL_OW_ROCK | Gray/stone |
| PAL_OW_TREE | Green (foliage) |
| PAL_OW_EMOTE | Special (silver trophy) |

### Player Sprites
| Sprite | Type | Palette | Notes |
|--------|------|---------|-------|
| Chris | Walking | Red | Male protagonist |
| Chris (Bike) | Walking | Red | Male on bicycle |
| Kris | Walking | Blue | Female protagonist (Crystal) |
| Kris (Bike) | Walking | Blue | Female on bicycle |
| Surf | Walking | Blue | Player surfing sprite |

### Named Characters
| Sprite | Type | Palette |
|--------|------|---------|
| Rival | Walking | Red |
| Prof. Oak | Walking | Brown |
| Red | Walking | Red |
| Blue | Walking | Red |
| Bill | Walking | Red |
| Mom | Walking | Red |
| Red's Mom | Walking | Red |
| Daisy | Walking | Blue |
| Prof. Elm | Walking | Brown |
| Kurt | Walking | Brown |
| Kurt (Outside) | Standing | Brown |
| Elder | Walking | Brown |
| Captain | Walking | Brown |

### Gym Leaders / Elite Four
| Sprite | Type | Palette |
|--------|------|---------|
| Falkner | Walking | Blue |
| Bugsy | Walking | Green |
| Whitney | Walking | Red |
| Morty | Walking | Brown |
| Chuck | Walking | Red |
| Jasmine | Walking | Green |
| Pryce | Walking | Brown |
| Clair | Walking | Red |
| Brock | Walking | Brown |
| Misty | Walking | Blue |
| Lt. Surge | Walking | Green |
| Erika | Walking | Green |
| Koga | Walking | Brown |
| Sabrina | Walking | Red |
| Blaine | Walking | Brown |
| Janine | Walking | Red |
| Will | Standing | Red |
| Karen | Standing | Blue |
| Bruno | Walking | Red |
| Lance | Walking | Red |

### Trainer Classes
| Sprite | Type | Palette |
|--------|------|---------|
| Cooltrainer M | Walking | Blue |
| Cooltrainer F | Walking | Blue |
| Bug Catcher | Walking | Blue |
| Twin | Walking | Red |
| Youngster | Walking | Blue |
| Lass | Walking | Red |
| Teacher | Walking | Red |
| Beauty | Walking | Blue |
| Super Nerd | Walking | Blue |
| Rocker | Walking | Green |
| Pokefan M | Walking | Brown |
| Pokefan F | Walking | Brown |
| Gramps | Walking | Brown |
| Granny | Walking | Brown |
| Swimmer Guy | Walking | Blue |
| Swimmer Girl | Walking | Blue |
| Rocket (Male) | Walking | Brown |
| Rocket (Female) | Walking | Brown |
| Scientist | Walking | Blue |
| Kimono Girl | Walking | Red |
| Sage | Walking | Brown |
| Gentleman | Walking | Blue |
| Black Belt | Walking | Brown |
| Officer | Walking | Blue |
| Sailor | Walking | Blue |
| Biker | Walking | Brown |
| Fisher | Walking | Blue |
| Fishing Guru | Walking | Blue |
| Cal | Walking | Brown |

### NPC/Object Sprites
| Sprite | Type | Palette | Notes |
|--------|------|---------|-------|
| Nurse | Standing | Red | Pokemon Center nurse |
| Link Receptionist | Walking | Red | Trade/battle receptionist |
| Clerk | Walking | Green | Mart clerk |
| Pharmacist | Walking | Blue | Herbal medicine shop |
| Gym Guide | Walking | Blue | Gym info NPC |
| Receptionist | Walking | Blue | Generic receptionist |
| Gameboy Kid | Standing | Green | NPC with Game Boy |

### Pokemon/Object Sprites
| Sprite | Type | Palette | Notes |
|--------|------|---------|-------|
| Big Snorlax | Standing | Blue | Blocking route |
| Surfing Pikachu | Walking | Red | Mini-game sprite |
| Big Lapras | Standing | Blue | S.S. Aqua deck |
| Big Onix | Standing | Brown | Crystal intro |
| Slowpoke | Still | Red | Slowpoke Well |
| Sudowoodo | Standing | Green | Route 36 blocker |
| Suicune | Still | Blue | Roaming sprite |
| Entei | Still | Red | Roaming sprite |
| Raikou | Still | Red | Roaming sprite |
| Standing Youngster | Standing | Blue | |

### Item/Furniture Sprites
| Sprite | Type | Palette | Notes |
|--------|------|---------|-------|
| Poke Ball | Still | Red | Item on ground |
| Pokedex | Still | Brown | |
| Paper | Still | Blue | Notes/documents |
| Virtual Boy | Still | Red | Easter egg |
| Rock | Still | Rock | Strength boulder |
| Boulder | Still | Rock | Puzzle boulder |
| SNES | Still | Blue | Game console |
| Famicom | Still | Red | Game console |
| Fruit Tree | Still | Tree | Berry tree |
| Gold Trophy | Still | Brown | |
| Silver Trophy | Still | Emote | |
| N64 | Still | Brown | Game console |

---

## Emotes (12 total)

From `data/sprites/emotes.asm`. Displayed above NPCs/player heads:

| Emote | Name | Usage |
|-------|------|-------|
| 1 | Shock | ! exclamation (spotted by trainer) |
| 2 | Question | ? confusion |
| 3 | Happy | Musical notes (happy) |
| 4 | Sad | ... (disappointed) |
| 5 | Heart | Heart symbol (love/affection) |
| 6 | Bolt | Lightning bolt (anger/surprise) |
| 7 | Sleep | Zzz (sleeping) |
| 8 | Fish | Fish on line (fishing) |
| 9 | Jump Shadow | Shadow for jumping NPCs |
| 10 | Fishing Rod | Rod sprite during fishing |
| 11 | Boulder Dust | Dust cloud (Strength push) |
| 12 | Grass Rustle | Grass movement (headbutt) |

---

## Sprite Animation System

From `data/sprite_anims/`. The sprite animation system handles:

### Animation Objects (`objects.asm`)
Defines animation objects used in battle transitions, title screen, and special effects. Each object has:
- Sprite graphics reference
- Position and movement parameters
- Animation frameset reference

### Framesets (`framesets.asm`)
Defines sequences of animation frames. Each frameset is a list of:
- OAM tile index
- Duration (in frames)
- Next frame pointer (for looping)

### OAM Data (`oam.asm`)
Object Attribute Map data for positioning sprite tiles on screen. Each OAM entry has:
- Y position, X position
- Tile index
- Attributes (palette, flip flags, priority)

### Player Sprites (`player_sprites.asm`)
Maps player state to sprite graphics:
- Walking sprites (4 directions x 3 frames each)
- Bike sprites
- Surf sprite
- Gender-specific sprites (Chris vs Kris)

### Monster Sprites (`sprite_mons.asm`)
Maps Pokemon species to their overworld follower sprite (used for walking Pokemon in some events, like the Pikachu surfing mini-game sprite).
