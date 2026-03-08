# World Area Data Format

Each area file is JSON with the following structure:

```json
{
  "area_id": "new_bark_town",
  "area_name": "New Bark Town",
  "area_type": "town|route|dungeon|building|cave|forest|lake|special",
  "pokecrystal_map_id": "NEW_BARK_TOWN",
  "width_blocks": 10,
  "height_blocks": 9,
  "width_tiles": 20,
  "height_tiles": 18,
  "tileset": "johto",
  "connections": [
    {
      "direction": "west|east|north|south",
      "target_area_id": "route_29",
      "offset": 0,
      "requires": null
    }
  ],
  "warps": [
    {
      "x": 5, "y": 3,
      "target_area_id": "players_house_1f",
      "target_warp_id": 0
    }
  ],
  "tile_layout": {
    "legend": {
      ".": "walkable_ground",
      "#": "wall/obstacle",
      "T": "tree",
      "~": "water",
      "G": "tall_grass",
      "H": "house_footprint",
      "D": "door",
      "F": "fence",
      "L": "ledge_south",
      "P": "path",
      "S": "sign",
      "R": "rock",
      "B": "bridge",
      "C": "cave_entrance",
      "W": "whirlpool",
      "I": "ice",
      "M": "mart",
      "K": "pokecenter",
      "^": "stairs_up",
      "v": "stairs_down",
      "X": "blocked",
      "E": "exit_mat",
      "N": "npc_position",
      "!": "item_ball",
      "?": "hidden_item",
      "@": "player_start"
    },
    "rows": [
      "####################",
      "#TTTT..HH..HH..TTTT"
    ]
  },
  "npcs": [
    {
      "id": "npc_1",
      "name": "Mom",
      "x": 5, "y": 3,
      "facing": "down",
      "movement": "standing|walk_around|look_around",
      "script_type": "dialogue|trainer|shopkeeper|healer",
      "dialogue_summary": "Saves money, gives Pokegear at start"
    }
  ],
  "trainers": [
    {
      "class": "Youngster",
      "name": "Joey",
      "x": 10, "y": 5,
      "facing": "left",
      "line_of_sight": 3,
      "team": [{"species": "Rattata", "level": 4}],
      "prize_money": 64,
      "rematch": false
    }
  ],
  "items": [
    {
      "name": "Potion",
      "x": 12, "y": 7,
      "hidden": false,
      "requires": null
    }
  ],
  "wild_encounters": {
    "grass": {
      "encounter_rate": 25,
      "morning": [{"species": "Pidgey", "level_min": 2, "level_max": 4, "rate": 55}],
      "day": [],
      "night": []
    },
    "water": { "encounter_rate": 15, "mons": [] },
    "fishing": { "old_rod": [], "good_rod": [], "super_rod": [] }
  },
  "events": [
    {
      "trigger": "first_visit",
      "description": "Guide Gent gives tour of city"
    }
  ],
  "notes": "Additional implementation notes"
}
```

## Block vs Tile dimensions
- pokecrystal stores dimensions in **blocks** (2x2 tiles each)
- width_tiles = width_blocks * 2
- height_tiles = height_blocks * 2
- Tile layouts should use width_tiles x height_tiles grid

## Area types
- **town**: Outdoor town/city map
- **route**: Outdoor route between locations
- **dungeon**: Multi-floor indoor dungeon (each floor is a separate area file)
- **building**: Unique interior building (gym, shop, house)
- **cave**: Cave area
- **forest**: Forest area
- **lake**: Lake/water area
- **special**: Special purpose area (E4 rooms, etc.)
