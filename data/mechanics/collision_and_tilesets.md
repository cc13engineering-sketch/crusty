# Pokemon Crystal - Collision and Tilesets

Collision type data and tileset properties from pokecrystal data/collision/ and data/tilesets/.

Source: `data/collision/*.asm`, `data/tilesets.asm`, `data/tilesets/*.asm`

---

## Collision Tile Types

From `data/collision/collision_permissions.asm`. Every tile in the game has a collision byte. There are 3 base permission types:

| Permission | Value | Meaning |
|-----------|-------|---------|
| LAND_TILE | walkable | Player can walk on this tile |
| WATER_TILE | surfable | Requires Surf to traverse |
| WALL_TILE | impassable | Player cannot walk through |

Some tiles also have the TALK flag, meaning they can be interacted with (face the tile and press A).

---

## Collision Categories

### Floor/Walkable (LAND_TILE)
| Code | Name | Notes |
|------|------|-------|
| 0x00 | COLL_FLOOR | Standard walkable floor |
| 0x01-0x06 | Various floors | All walkable |
| 0x08 | COLL_CUT_08 | Cut-able grass tile |
| 0x10 | COLL_TALL_GRASS_10 | Triggers wild encounters |
| 0x14 | COLL_LONG_GRASS | Long grass (encounter tile) |
| 0x18 | COLL_TALL_GRASS | Tall grass (encounter tile) |
| 0x23 | COLL_ICE | Ice floor (player slides) |
| 0x40 | COLL_BRAKE | Stops sliding on ice |
| 0x41-0x44 | COLL_WALK_RIGHT/LEFT/UP/DOWN | Forced movement tiles |
| 0x48-0x4C | COLL_GRASS_48-4C | Various grass tiles |
| 0x50-0x53 | COLL_WALK_*_ALT | Alternative forced movement |
| 0x60 | COLL_PIT | Fall-through pit (dungeons) |
| 0x61 | COLL_VIRTUAL_BOY | Easter egg (virtual boy console) |

### Warp Tiles (LAND_TILE, trigger map transitions)
| Code | Name | Notes |
|------|------|-------|
| 0x70 | COLL_WARP_CARPET_DOWN | Warp with downward animation |
| 0x71 | COLL_DOOR | Standard door warp |
| 0x72 | COLL_LADDER | Ladder warp |
| 0x73 | COLL_STAIRCASE_73 | Staircase warp |
| 0x74 | COLL_CAVE_74 | Cave entrance warp |
| 0x7A | COLL_STAIRCASE | Standard staircase |
| 0x7B | COLL_CAVE | Cave opening |
| 0x7C | COLL_WARP_PANEL | Teleport panel (Silph Co style) |

### Water Tiles (WATER_TILE)
| Code | Name | Notes |
|------|------|-------|
| 0x20-0x21 | Water | Standard water |
| 0x24 | COLL_WHIRLPOOL | Whirlpool obstacle (HM06) |
| 0x27 | COLL_BUOY | Impassable water barrier |
| 0x29 | COLL_WATER | Standard water |
| 0x30-0x33 | COLL_WATERFALL_* | Waterfall tiles (HM07) |
| 0x38-0x3B | COLL_CURRENT_* | Water current (forced movement) |
| 0xC0-0xC7 | COLL_*_BUOY | Directional buoy barriers |

### Wall/Interactable Tiles (WALL_TILE)
| Code | Name | Notes |
|------|------|-------|
| 0x07 | COLL_WALL | Standard impassable wall |
| 0x12 | COLL_CUT_TREE | Cut tree (HM01 interaction) |
| 0x15 | COLL_HEADBUTT_TREE | Headbutt tree (TM02) |
| 0x90 | COLL_COUNTER | Shop counter (talk over) |
| 0x91 | COLL_BOOKSHELF | Bookshelf (triggers read text) |
| 0x93 | COLL_PC | Computer (triggers PC menu) |
| 0x94 | COLL_RADIO | Radio (triggers radio) |
| 0x95 | COLL_TOWN_MAP | Town map on wall |
| 0x96 | COLL_MART_SHELF | Merchandise shelf |
| 0x97 | COLL_TV | Television |
| 0x9D | COLL_WINDOW | Window |
| 0x9F | COLL_INCENSE_BURNER | Incense burner |

### Ledge/Hop Tiles (LAND_TILE, one-way)
| Code | Name | Notes |
|------|------|-------|
| 0xA0 | COLL_HOP_RIGHT | Hop ledge facing right |
| 0xA1 | COLL_HOP_LEFT | Hop ledge facing left |
| 0xA2 | COLL_HOP_UP | Hop ledge facing up |
| 0xA3 | COLL_HOP_DOWN | Hop ledge facing down |
| 0xA4-0xA7 | COLL_HOP_DOWN_RIGHT etc. | Diagonal ledges |

### One-Way Walls (LAND_TILE, block from one direction)
| Code | Name | Notes |
|------|------|-------|
| 0xB0 | COLL_RIGHT_WALL | Blocks rightward movement |
| 0xB1 | COLL_LEFT_WALL | Blocks leftward movement |
| 0xB2 | COLL_UP_WALL | Blocks upward movement |
| 0xB3 | COLL_DOWN_WALL | Blocks downward movement |
| 0xB4-0xB7 | Diagonal walls | Corner blocking |

---

## Collision-Triggered Scripts

From `data/collision/collision_stdscripts.asm`. Walking into these tiles triggers a standard script:

| Tile | Script | Effect |
|------|--------|--------|
| COLL_BOOKSHELF | MagazineBookshelfScript | "Plenty of Pokemon magazines!" (or variant) |
| COLL_PC | PCScript | Opens Pokemon Center PC |
| COLL_RADIO | Radio1Script | Turns on radio |
| COLL_TOWN_MAP | TownMapScript | Shows town map |
| COLL_MART_SHELF | MerchandiseShelfScript | "Merchandise shelf" text |
| COLL_TV | TVScript | Displays TV text |
| COLL_WINDOW | WindowScript | Window description |
| COLL_INCENSE_BURNER | IncenseBurnerScript | Incense burner description |

---

## Field Move Blocks (Cut and Whirlpool)

From `data/collision/field_move_blocks.asm`. When you use Cut or Whirlpool, the game replaces one map block with another:

### Cut Trees
| Tileset | Facing Block | Replacement | Animation |
|---------|-------------|-------------|-----------|
| Johto | Grass ($03) | Cleared ($02) | Grass cut |
| Johto | Tree ($5B) | Stump ($3C) | Tree fall |
| Johto | Tree ($5F) | Stump ($3D) | Tree fall |
| Johto | Tree ($63) | Stump ($3F) | Tree fall |
| Johto | Tree ($67) | Stump ($3E) | Tree fall |
| Kanto | Grass ($0B) | Cleared ($0A) | Grass cut |
| Kanto | Tree (5 variants) | Stump variants | Tree fall |
| Park | Grass ($13, $03) | Cleared ($03, $04) | Grass cut |
| Forest | Tree ($0F) | Cleared ($17) | Tree fall |

### Whirlpool
| Tileset | Facing Block | Replacement |
|---------|-------------|-------------|
| Johto | Whirlpool ($07) | Calm water ($36) |

---

## Tilesets

The game uses multiple tilesets for different map environments. Each tileset defines the visual tiles, collision data, and palette mapping.

Key tilesets referenced in field_move_blocks:
- **TILESET_JOHTO** — Standard outdoor Johto tileset
- **TILESET_JOHTO_MODERN** — Modern buildings in Johto
- **TILESET_KANTO** — Kanto outdoor tileset
- **TILESET_PARK** — National Park tileset
- **TILESET_FOREST** — Ilex Forest / forest areas

Additional tilesets from `data/tilesets.asm` (37 total tileset definition files):
- Cave, Gate, House, Lab, Facility, Port, Pokecenter, Mart
- Radio Tower, Lighthouse, Ruins, Ice Path, Underground
- Ho-Oh/Lugia towers, Mansion, Game Corner, Battle Tower
- And more specialized interior/exterior tilesets
