# Sprint 5 Implementation Plan: Route 31 + Route 31/Violet Gate + Violet City Exterior

> Frodo's phase-by-phase plan derived from Gandalf's SPRINT5_ARCHITECTURE.md.
> Each phase is self-contained. Engineers code directly from this document.

---

## Phase 1: data.rs — New Species, Moves, Items, Music (~80 lines)

### 1A. New Species ID Constants

Add after the `// --- Species ID Constants (Sprint 4 additions) ---` block (after line 796):

```rust
// --- Species ID Constants (Sprint 5 additions) ---
pub const BELLSPROUT: SpeciesId = 69;
pub const GASTLY: SpeciesId = 92;
```

### 1B. New Move ID Constants

Add after the `// --- Move ID Constants (Sprint 4 additions) ---` block (after line 780):

```rust
// --- Move ID Constants (Sprint 5 additions) ---
pub const MOVE_VINE_WHIP: MoveId = 22;
pub const MOVE_HYPNOSIS: MoveId = 95;
pub const MOVE_LICK: MoveId = 122;
pub const MOVE_GROWTH: MoveId = 74;
```

NOTE: Growth is needed for Bellsprout's learnset at level 6. Add it even though Bellsprout won't know it at encounter levels 4-5 — it's part of the canonical learnset and a level 6 Bellsprout should know it.

### 1C. New Item Constants

Add after the existing `ITEM_MYSTERY_EGG` line (after line 762):

```rust
// --- Item ID Constants (Sprint 5 additions) ---
pub const ITEM_PP_UP: u8 = 48;
pub const ITEM_RARE_CANDY: u8 = 43;
pub const ITEM_PRZ_CURE_BERRY: u8 = 54;
pub const ITEM_HYPER_POTION: u8 = 26;
```

### 1D. New Music Constants

Add after `MUSIC_JOHTO_TRAINER_BATTLE` (after line 842):

```rust
pub const MUSIC_VIOLET_CITY: u8 = 15;
pub const MUSIC_ROUTE_31: u8 = 16;
```

### 1E. Bellsprout Species Data

Add after the `SPINARAK_DATA` static (after line 478):

```rust
// Sprint 5 learnsets
static BELLSPROUT_LEARNSET: &[(u8, MoveId)] = &[
    (1, MOVE_VINE_WHIP), (6, MOVE_GROWTH),
];
static GASTLY_LEARNSET: &[(u8, MoveId)] = &[
    (1, MOVE_HYPNOSIS), (1, MOVE_LICK),
];

// Sprint 5 species data
static BELLSPROUT_DATA: SpeciesData = SpeciesData {
    id: BELLSPROUT, name: "BELLSPROUT", type1: PokemonType::Grass, type2: PokemonType::Poison,
    base_hp: 50, base_attack: 75, base_defense: 35, base_speed: 40,
    base_sp_attack: 70, base_sp_defense: 30, catch_rate: 255, base_exp: 84,
    growth_rate: GrowthRate::MediumSlow, learnset: BELLSPROUT_LEARNSET,
};
static GASTLY_DATA: SpeciesData = SpeciesData {
    id: GASTLY, name: "GASTLY", type1: PokemonType::Ghost, type2: PokemonType::Poison,
    base_hp: 30, base_attack: 35, base_defense: 30, base_speed: 80,
    base_sp_attack: 100, base_sp_defense: 35, catch_rate: 190, base_exp: 95,
    growth_rate: GrowthRate::MediumSlow, learnset: GASTLY_LEARNSET,
};
```

### 1F. New Move Data Statics

Add after `SUPERSONIC_DATA` static (after line 645):

```rust
// Sprint 5 move data
static VINE_WHIP_DATA: MoveData = MoveData {
    id: MOVE_VINE_WHIP, name: "VINE WHIP",
    move_type: PokemonType::Grass, power: 35, accuracy: 100, pp: 10, is_special: true,
};
static HYPNOSIS_DATA: MoveData = MoveData {
    id: MOVE_HYPNOSIS, name: "HYPNOSIS",
    move_type: PokemonType::Psychic, power: 0, accuracy: 60, pp: 20, is_special: false,
};
static LICK_DATA: MoveData = MoveData {
    id: MOVE_LICK, name: "LICK",
    move_type: PokemonType::Ghost, power: 20, accuracy: 100, pp: 30, is_special: false,
};
static GROWTH_DATA: MoveData = MoveData {
    id: MOVE_GROWTH, name: "GROWTH",
    move_type: PokemonType::Normal, power: 0, accuracy: 100, pp: 40, is_special: false,
};
```

### 1G. Update `species_data()` Match Arms

Add before the `_ => &CHIKORITA_DATA` fallback (line 499):

```rust
        // Sprint 5
        BELLSPROUT => &BELLSPROUT_DATA,
        GASTLY => &GASTLY_DATA,
```

### 1H. Update `move_data()` Match Arms

Add before the `_ => &TACKLE_DATA` fallback (line 668):

```rust
        // Sprint 5
        MOVE_VINE_WHIP => &VINE_WHIP_DATA,
        MOVE_HYPNOSIS => &HYPNOSIS_DATA,
        MOVE_LICK => &LICK_DATA,
        MOVE_GROWTH => &GROWTH_DATA,
```

### 1I. Update Import Line in maps.rs

The `use super::data::{...}` line in maps.rs (line 9-11) must add:

```rust
use super::data::{Direction, NpcState, SpeciesId,
    PIDGEY, RATTATA, SENTRET, HOOTHOOT, HOPPIP,
    CATERPIE, WEEDLE, ZUBAT, POLIWAG, LEDYBA, SPINARAK,
    BELLSPROUT, GASTLY,
    MUSIC_VIOLET_CITY, MUSIC_ROUTE_31};
```

### Phase 1 Tests

Add to `data.rs` `mod tests`:

```rust
    #[test]
    fn test_bellsprout_data() {
        let data = species_data(BELLSPROUT);
        assert_eq!(data.name, "BELLSPROUT");
        assert_eq!(data.type1, PokemonType::Grass);
        assert_eq!(data.type2, PokemonType::Poison);
        assert_eq!(data.base_hp, 50);
        assert_eq!(data.base_attack, 75);
        assert_eq!(data.base_sp_attack, 70);
        assert_eq!(data.catch_rate, 255);
        assert!(matches!(data.growth_rate, GrowthRate::MediumSlow));
    }

    #[test]
    fn test_gastly_data() {
        let data = species_data(GASTLY);
        assert_eq!(data.name, "GASTLY");
        assert_eq!(data.type1, PokemonType::Ghost);
        assert_eq!(data.type2, PokemonType::Poison);
        assert_eq!(data.base_hp, 30);
        assert_eq!(data.base_sp_attack, 100);
        assert_eq!(data.base_speed, 80);
        assert_eq!(data.catch_rate, 190);
    }

    #[test]
    fn test_bellsprout_learnset_at_level5() {
        let poke = Pokemon::new(BELLSPROUT, 5);
        let known: Vec<_> = poke.moves.iter().filter_map(|&m| m).collect();
        assert_eq!(known.len(), 1, "Level 5 Bellsprout should know only VineWhip");
        assert!(known.contains(&MOVE_VINE_WHIP));
    }

    #[test]
    fn test_gastly_learnset_at_level5() {
        let poke = Pokemon::new(GASTLY, 5);
        let known: Vec<_> = poke.moves.iter().filter_map(|&m| m).collect();
        assert_eq!(known.len(), 2, "Level 5 Gastly should know Hypnosis + Lick");
        assert!(known.contains(&MOVE_HYPNOSIS));
        assert!(known.contains(&MOVE_LICK));
    }

    #[test]
    fn test_vine_whip_is_special() {
        let data = move_data(MOVE_VINE_WHIP);
        assert!(data.is_special, "Vine Whip should be a special move (Grass type in Gen 2)");
        assert_eq!(data.power, 35);
        assert_eq!(data.move_type, PokemonType::Grass);
    }

    #[test]
    fn test_sprint5_moves_data() {
        let moves = [MOVE_VINE_WHIP, MOVE_HYPNOSIS, MOVE_LICK, MOVE_GROWTH];
        for &mv in &moves {
            let data = move_data(mv);
            assert!(!data.name.is_empty(), "Move {} should have a name", mv);
            assert!(data.pp > 0, "Move {} should have non-zero PP", mv);
        }
    }
```

---

## Phase 2: maps.rs — New MapId Variants + Map Builders (~500 lines)

### 2A. New MapId Variants

Replace the existing `MapId` enum (lines 27-51) with:

```rust
#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub enum MapId {
    // Sprint 1 (existing)
    NewBarkTown,
    PlayersHouse1F,
    PlayersHouse2F,
    ElmsLab,
    ElmsHouse,
    PlayersNeighborsHouse,
    Route29,
    Route27,
    // Sprint 2 (new)
    Route29Route46Gate,
    CherrygroveCity,
    CherrygrovePokecenter1F,
    CherrygroveMart,
    GuideGentsHouse,
    CherrygroveGymSpeechHouse,
    CherrygroveEvolutionSpeechHouse,
    Route46,
    Route30,
    // Sprint 4 (new)
    Route30BerryHouse,
    MrPokemonsHouse,
    Route31,
    // Sprint 5 (new)
    Route31VioletGate,
    VioletCity,
    VioletMart,
    VioletGym,
    EarlsPokemonAcademy,
    VioletNicknameSpeechHouse,
    VioletPokecenter1F,
    VioletKylesHouse,
    SproutTower1F,
    DarkCaveVioletEntrance,
    Route32,
    Route36,
}
```

**CRITICAL**: The ordering must be preserved — Sprint 1, then Sprint 2, then Sprint 4, then Sprint 5 — because `SceneState` indexes by `map as usize`. Sprint 5 maps are appended at the end.

### 2B. Update `load_map()` Dispatcher

Replace the existing `load_map` function to add all new entries. Change the `MapId::Route31` arm from `build_route31_stub()` to `build_route31()`, and add all new arms:

```rust
pub fn load_map(id: MapId) -> MapData {
    match id {
        // ... all existing arms unchanged ...
        MapId::Route31              => build_route31(),  // CHANGED: was build_route31_stub()
        // Sprint 5 (new)
        MapId::Route31VioletGate    => build_route31_violet_gate(),
        MapId::VioletCity           => build_violet_city(),
        MapId::VioletMart           => build_violet_city_stub(MapId::VioletMart, "VIOLET MART", MapId::VioletCity, 0),
        MapId::VioletGym            => build_violet_city_stub(MapId::VioletGym, "VIOLET GYM", MapId::VioletCity, 1),
        MapId::EarlsPokemonAcademy  => build_violet_city_stub(MapId::EarlsPokemonAcademy, "EARL'S POKEMON ACADEMY", MapId::VioletCity, 2),
        MapId::VioletNicknameSpeechHouse => build_violet_city_stub(MapId::VioletNicknameSpeechHouse, "VIOLET NICKNAME HOUSE", MapId::VioletCity, 3),
        MapId::VioletPokecenter1F   => build_violet_city_stub(MapId::VioletPokecenter1F, "VIOLET POKEMON CENTER", MapId::VioletCity, 4),
        MapId::VioletKylesHouse     => build_violet_city_stub(MapId::VioletKylesHouse, "KYLE'S HOUSE", MapId::VioletCity, 5),
        MapId::SproutTower1F        => build_violet_city_stub(MapId::SproutTower1F, "SPROUT TOWER 1F", MapId::VioletCity, 6),
        MapId::DarkCaveVioletEntrance => build_dark_cave_violet_entrance_stub(),
        MapId::Route32              => build_connection_stub(MapId::Route32, "ROUTE 32"),
        MapId::Route36              => build_connection_stub(MapId::Route36, "ROUTE 36"),
    }
}
```

### 2C. Route 31 Wild Encounter Table

Add a new function (after the existing `build_route30_encounters()` helper or near the Route 31 builder):

```rust
fn build_route31_encounters() -> WildEncounterTable {
    WildEncounterTable {
        encounter_rate: 10,
        morning: vec![
            WildSlot { species: LEDYBA,     level: 4 },
            WildSlot { species: CATERPIE,   level: 4 },
            WildSlot { species: BELLSPROUT,  level: 5 },
            WildSlot { species: PIDGEY,     level: 5 },
            WildSlot { species: WEEDLE,     level: 4 },
            WildSlot { species: HOPPIP,     level: 5 },
            WildSlot { species: HOPPIP,     level: 5 },
        ],
        day: vec![
            WildSlot { species: PIDGEY,     level: 4 },
            WildSlot { species: CATERPIE,   level: 4 },
            WildSlot { species: BELLSPROUT,  level: 5 },
            WildSlot { species: PIDGEY,     level: 5 },
            WildSlot { species: WEEDLE,     level: 4 },
            WildSlot { species: HOPPIP,     level: 5 },
            WildSlot { species: HOPPIP,     level: 5 },
        ],
        night: vec![
            WildSlot { species: SPINARAK,   level: 4 },
            WildSlot { species: POLIWAG,    level: 4 },
            WildSlot { species: BELLSPROUT,  level: 5 },
            WildSlot { species: HOOTHOOT,   level: 5 },
            WildSlot { species: ZUBAT,      level: 4 },
            WildSlot { species: GASTLY,     level: 5 },
            WildSlot { species: GASTLY,     level: 5 },
        ],
    }
}
```

Source: `pokecrystal-master/data/wild/johto_grass.asm` lines 1293-1318.

### 2D. Replace `build_route31_stub()` with Full `build_route31()`

Delete `build_route31_stub()` (lines 1180-1202) and replace with:

```rust
// ── Route 31 (40 x 18) ─────────────────────────────────────────────────────
// Source: pokecrystal-master/maps/Route31.asm, map_const ROUTE_31, 20, 9

fn build_route31() -> MapData {
    let (w, h) = (40i32, 18i32);
    let total = (w * h) as usize;
    let mut tiles = vec![0u8; total];
    let mut col = vec![C_FLOOR; total];

    // Perimeter walls (top/bottom)
    for x in 0..w {
        set_tile(&mut tiles, &mut col, w, x, 0, 2, C_WALL);
        set_tile(&mut tiles, &mut col, w, x, 17, 2, C_WALL);
    }

    // Tree/wall blocks along top and bottom corridors
    for x in 0..4 { for y in 0..8 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    for x in 6..12 { for y in 0..3 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    for x in 24..30 { for y in 0..3 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    for x in 0..4 { for y in 10..18 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    for x in 6..14 { for y in 15..18 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    for x in 24..30 { for y in 15..18 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }

    // Dark Cave entrance wall area (east side)
    for x in 32..40 { for y in 0..4 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    for x in 36..40 { for y in 4..8 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }

    // Grass patches
    for x in 12..20 { for y in 4..8 { set_tile(&mut tiles, &mut col, w, x, y, 7, C_GRASS); } }
    for x in 8..14 { for y in 10..14 { set_tile(&mut tiles, &mut col, w, x, y, 7, C_GRASS); } }
    for x in 20..28 { for y in 10..14 { set_tile(&mut tiles, &mut col, w, x, y, 7, C_GRASS); } }

    // Gate warps (west side)
    set_tile(&mut tiles, &mut col, w, 4, 6, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 4, 7, 3, C_WARP);
    // Dark Cave entrance warp
    set_tile(&mut tiles, &mut col, w, 34, 5, 3, C_WARP);

    MapData {
        id: MapId::Route31,
        name: "ROUTE 31",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            // warp idx 0: gate upper
            WarpDef { x: 4, y: 6, dest_map: MapId::Route31VioletGate, dest_warp_id: 2 },
            // warp idx 1: gate lower
            WarpDef { x: 4, y: 7, dest_map: MapId::Route31VioletGate, dest_warp_id: 3 },
            // warp idx 2: Dark Cave entrance
            WarpDef { x: 34, y: 5, dest_map: MapId::DarkCaveVioletEntrance, dest_warp_id: 0 },
        ],
        npcs: vec![
            // idx 0: Fisher (Kenya mail NPC) at (17,7), StandingDown
            NpcDef { x: 17, y: 7, sprite_id: 4, move_type: NpcMoveType::Standing(Direction::Down),
                script_id: 403, event_flag: None, event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "KENYA_FISHER", trainer_range: None },
            // idx 1: Youngster at (9,5), SpinRandom
            NpcDef { x: 9, y: 5, sprite_id: 15, move_type: NpcMoveType::SpinRandom,
                script_id: 404, event_flag: None, event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "YOUNGSTER", trainer_range: None },
            // idx 2: Bug Catcher Wade at (21,13), StandingLeft, trainer range=5
            NpcDef { x: 21, y: 13, sprite_id: 18, move_type: NpcMoveType::Standing(Direction::Left),
                script_id: 402, event_flag: Some(44), event_flag_show: false, palette: 0,
                facing: Direction::Left, name: "BUG_CATCHER_WADE", trainer_range: Some(5) },
            // idx 3: CooltrainerM at (33,8), SpinRandom
            NpcDef { x: 33, y: 8, sprite_id: 14, move_type: NpcMoveType::SpinRandom,
                script_id: 405, event_flag: None, event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "COOLTRAINER_M", trainer_range: None },
            // idx 4: Fruit Tree at (16,7)
            NpcDef { x: 16, y: 7, sprite_id: 16, move_type: NpcMoveType::Still,
                script_id: 406, event_flag: None, event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "FRUIT_TREE", trainer_range: None },
            // idx 5: Potion PokeBall at (29,5) -- hidden when EVENT_ROUTE_31_POTION set
            NpcDef { x: 29, y: 5, sprite_id: 6, move_type: NpcMoveType::Still,
                script_id: 407, event_flag: Some(46), event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "POTION_BALL", trainer_range: None },
            // idx 6: Poke Ball PokeBall at (19,15) -- hidden when EVENT_ROUTE_31_POKE_BALL set
            NpcDef { x: 19, y: 15, sprite_id: 6, move_type: NpcMoveType::Still,
                script_id: 408, event_flag: Some(47), event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "POKE_BALL_BALL", trainer_range: None },
        ],
        bg_events: vec![
            BgEvent { x: 7, y: 5, kind: BgEventKind::Read, script_id: 400 },
            BgEvent { x: 31, y: 5, kind: BgEventKind::Read, script_id: 401 },
        ],
        coord_events: vec![],
        wild_encounters: Some(build_route31_encounters()),
        connections: MapConnections {
            north: None,
            south: Some(MapConnection { direction: Direction::Down, dest_map: MapId::Route30, offset: 10 }),
            east: None,
            west: Some(MapConnection { direction: Direction::Left, dest_map: MapId::VioletCity, offset: -9 }),
        },
        music_id: MUSIC_ROUTE_31,
    }
}
```

### 2E. Route 31/Violet Gate Builder

Add new function:

```rust
// ── Route 31 Violet Gate (10 x 8) ──────────────────────────────────────────
// Source: pokecrystal-master/maps/Route31VioletGate.asm

fn build_route31_violet_gate() -> MapData {
    let (w, h) = (10i32, 8i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);

    // West exit warps (to Violet City)
    set_tile(&mut tiles, &mut col, w, 0, 4, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 0, 5, 3, C_WARP);
    // East exit warps (to Route 31)
    set_tile(&mut tiles, &mut col, w, 9, 4, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, 9, 5, 3, C_WARP);

    MapData {
        id: MapId::Route31VioletGate,
        name: "ROUTE 31 GATE",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            // warp idx 0: west exit upper -> VioletCity warp 7
            WarpDef { x: 0, y: 4, dest_map: MapId::VioletCity, dest_warp_id: 7 },
            // warp idx 1: west exit lower -> VioletCity warp 8
            WarpDef { x: 0, y: 5, dest_map: MapId::VioletCity, dest_warp_id: 8 },
            // warp idx 2: east exit upper -> Route31 warp 0
            WarpDef { x: 9, y: 4, dest_map: MapId::Route31, dest_warp_id: 0 },
            // warp idx 3: east exit lower -> Route31 warp 1
            WarpDef { x: 9, y: 5, dest_map: MapId::Route31, dest_warp_id: 1 },
        ],
        npcs: vec![
            // idx 0: Officer at (5,2), StandingDown
            NpcDef { x: 5, y: 2, sprite_id: 8, move_type: NpcMoveType::Standing(Direction::Down),
                script_id: 420, event_flag: None, event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "OFFICER", trainer_range: None },
            // idx 1: CooltrainerF at (1,2), SpinRandom
            NpcDef { x: 1, y: 2, sprite_id: 19, move_type: NpcMoveType::SpinRandom,
                script_id: 421, event_flag: None, event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "COOLTRAINER_F", trainer_range: None },
        ],
        coord_events: vec![],
        bg_events: vec![],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: MUSIC_ROUTE_31,
    }
}
```

### 2F. Violet City Builder

Add new function:

```rust
// ── Violet City (40 x 36) ──────────────────────────────────────────────────
// Source: pokecrystal-master/maps/VioletCity.asm, map_const VIOLET_CITY, 20, 18

fn build_violet_city() -> MapData {
    let (w, h) = (40i32, 36i32);
    let total = (w * h) as usize;
    let mut tiles = vec![1u8; total]; // path default
    let mut col = vec![C_FLOOR; total];

    // Perimeter walls (top/bottom)
    for x in 0..w {
        set_tile(&mut tiles, &mut col, w, x, 0, 2, C_WALL);
        set_tile(&mut tiles, &mut col, w, x, 35, 2, C_WALL);
    }
    // Left border wall
    for y in 0..h { set_tile(&mut tiles, &mut col, w, 0, y, 2, C_WALL); }

    // Building blocks (approximate)
    // Violet Mart area: rows 13-16, cols 7-12
    for x in 7..13 { for y in 13..16 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    // Violet Gym area: rows 13-16, cols 16-21
    for x in 16..22 { for y in 13..16 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    // Earl's Academy area: rows 13-16, cols 28-33
    for x in 28..34 { for y in 13..16 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    // Nickname Speech House: rows 11-14, cols 1-5
    for x in 1..6 { for y in 11..14 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    // Pokecenter area: rows 21-24, cols 29-34
    for x in 29..35 { for y in 21..24 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    // Kyle's House: rows 25-28, cols 19-24
    for x in 19..25 { for y in 25..28 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }
    // Sprout Tower area: rows 1-4, cols 21-26
    for x in 21..27 { for y in 1..4 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WALL); } }

    // Water tiles (pond near Sprout Tower)
    for x in 15..20 { for y in 2..6 { set_tile(&mut tiles, &mut col, w, x, y, 2, C_WATER); } }

    // Warp tiles for building entrances
    set_tile(&mut tiles, &mut col, w, 9, 17, 3, C_WARP);   // Mart
    set_tile(&mut tiles, &mut col, w, 18, 17, 3, C_WARP);  // Gym
    set_tile(&mut tiles, &mut col, w, 30, 17, 3, C_WARP);  // Academy
    set_tile(&mut tiles, &mut col, w, 3, 15, 3, C_WARP);   // Nickname House
    set_tile(&mut tiles, &mut col, w, 31, 25, 3, C_WARP);  // Pokecenter
    set_tile(&mut tiles, &mut col, w, 21, 29, 3, C_WARP);  // Kyle's House
    set_tile(&mut tiles, &mut col, w, 23, 5, 3, C_WARP);   // Sprout Tower
    set_tile(&mut tiles, &mut col, w, 39, 24, 3, C_WARP);  // East gate upper
    set_tile(&mut tiles, &mut col, w, 39, 25, 3, C_WARP);  // East gate lower

    MapData {
        id: MapId::VioletCity,
        name: "VIOLET CITY",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            // warp idx 0: Violet Mart
            WarpDef { x: 9, y: 17, dest_map: MapId::VioletMart, dest_warp_id: 0 },
            // warp idx 1: Violet Gym
            WarpDef { x: 18, y: 17, dest_map: MapId::VioletGym, dest_warp_id: 0 },
            // warp idx 2: Earl's Academy
            WarpDef { x: 30, y: 17, dest_map: MapId::EarlsPokemonAcademy, dest_warp_id: 0 },
            // warp idx 3: Nickname Speech House
            WarpDef { x: 3, y: 15, dest_map: MapId::VioletNicknameSpeechHouse, dest_warp_id: 0 },
            // warp idx 4: Pokecenter
            WarpDef { x: 31, y: 25, dest_map: MapId::VioletPokecenter1F, dest_warp_id: 0 },
            // warp idx 5: Kyle's House
            WarpDef { x: 21, y: 29, dest_map: MapId::VioletKylesHouse, dest_warp_id: 0 },
            // warp idx 6: Sprout Tower
            WarpDef { x: 23, y: 5, dest_map: MapId::SproutTower1F, dest_warp_id: 0 },
            // warp idx 7: East gate upper -> Route31VioletGate warp 0
            WarpDef { x: 39, y: 24, dest_map: MapId::Route31VioletGate, dest_warp_id: 0 },
            // warp idx 8: East gate lower -> Route31VioletGate warp 1
            WarpDef { x: 39, y: 25, dest_map: MapId::Route31VioletGate, dest_warp_id: 1 },
        ],
        npcs: vec![
            // idx 0: Earl at (13,16), SpinRandom -- conditional on EVENT_VIOLET_CITY_EARL
            NpcDef { x: 13, y: 16, sprite_id: 4, move_type: NpcMoveType::SpinRandom,
                script_id: 430, event_flag: Some(48), event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "EARL", trainer_range: None },
            // idx 1: Lass at (28,28), SpinRandom
            NpcDef { x: 28, y: 28, sprite_id: 20, move_type: NpcMoveType::SpinRandom,
                script_id: 431, event_flag: None, event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "LASS", trainer_range: None },
            // idx 2: Super Nerd at (24,14), SpinRandom
            NpcDef { x: 24, y: 14, sprite_id: 21, move_type: NpcMoveType::SpinRandom,
                script_id: 432, event_flag: None, event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "SUPER_NERD", trainer_range: None },
            // idx 3: Gramps at (17,20), WalkLeftRight
            NpcDef { x: 17, y: 20, sprite_id: 22, move_type: NpcMoveType::WalkLeftRight,
                script_id: 433, event_flag: None, event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "GRAMPS", trainer_range: None },
            // idx 4: Youngster at (5,18), SpinRandom
            NpcDef { x: 5, y: 18, sprite_id: 15, move_type: NpcMoveType::SpinRandom,
                script_id: 434, event_flag: None, event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "YOUNGSTER", trainer_range: None },
            // idx 5: Fruit Tree at (14,29)
            NpcDef { x: 14, y: 29, sprite_id: 16, move_type: NpcMoveType::Still,
                script_id: 435, event_flag: None, event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "FRUIT_TREE", trainer_range: None },
            // idx 6: PP Up PokeBall at (4,1) -- hidden when EVENT_VIOLET_CITY_PP_UP set
            NpcDef { x: 4, y: 1, sprite_id: 6, move_type: NpcMoveType::Still,
                script_id: 436, event_flag: Some(49), event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "PP_UP_BALL", trainer_range: None },
            // idx 7: Rare Candy PokeBall at (35,5) -- hidden when EVENT_VIOLET_CITY_RARE_CANDY set
            NpcDef { x: 35, y: 5, sprite_id: 6, move_type: NpcMoveType::Still,
                script_id: 437, event_flag: Some(50), event_flag_show: false, palette: 0,
                facing: Direction::Down, name: "RARE_CANDY_BALL", trainer_range: None },
        ],
        coord_events: vec![],
        bg_events: vec![
            BgEvent { x: 24, y: 20, kind: BgEventKind::Read, script_id: 440 },
            BgEvent { x: 15, y: 17, kind: BgEventKind::Read, script_id: 441 },
            BgEvent { x: 24, y: 8, kind: BgEventKind::Read, script_id: 442 },
            BgEvent { x: 27, y: 17, kind: BgEventKind::Read, script_id: 443 },
            BgEvent { x: 32, y: 25, kind: BgEventKind::Read, script_id: 444 },
            BgEvent { x: 10, y: 17, kind: BgEventKind::Read, script_id: 445 },
            // Hidden Hyper Potion: item event
            BgEvent { x: 37, y: 14, kind: BgEventKind::Read, script_id: 446 },
        ],
        wild_encounters: None,
        connections: MapConnections {
            north: None,
            south: Some(MapConnection { direction: Direction::Down, dest_map: MapId::Route32, offset: 0 }),
            east: Some(MapConnection { direction: Direction::Right, dest_map: MapId::Route31, offset: 9 }),
            west: Some(MapConnection { direction: Direction::Left, dest_map: MapId::Route36, offset: 0 }),
        },
        music_id: MUSIC_VIOLET_CITY,
    }
}
```

### 2G. Violet City Stub Building Helper

Add a generic stub building function for Violet City interiors:

```rust
/// Generic stub interior for Violet City buildings.
/// Creates an 8x8 (or 10x8 for pokecenter/gym) room with a single return warp.
fn build_violet_city_stub(id: MapId, name: &'static str, return_map: MapId, return_warp_id: u8) -> MapData {
    let (w, h) = match id {
        MapId::VioletGym | MapId::SproutTower1F => (10i32, 10i32),
        MapId::VioletPokecenter1F => (10i32, 8i32),
        _ => (8i32, 8i32),
    };
    let (mut tiles, mut col) = fill_room(w, h, 4);
    // Return warp at bottom-center
    let warp_x = w / 2;
    let warp_y = h - 1;
    set_tile(&mut tiles, &mut col, w, warp_x, warp_y, 3, C_WARP);
    set_tile(&mut tiles, &mut col, w, warp_x + 1, warp_y, 3, C_WARP);

    MapData {
        id,
        name,
        width: w, height: h, tiles, collision: col,
        warps: vec![
            WarpDef { x: warp_x, y: warp_y, dest_map: return_map, dest_warp_id: return_warp_id },
            WarpDef { x: warp_x + 1, y: warp_y, dest_map: return_map, dest_warp_id: return_warp_id },
        ],
        npcs: vec![],
        coord_events: vec![],
        bg_events: vec![],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: MUSIC_VIOLET_CITY,
    }
}
```

### 2H. Dark Cave Stub + Connection Target Stubs

```rust
/// Dark Cave Violet Entrance stub -- minimal room with return warp to Route 31.
fn build_dark_cave_violet_entrance_stub() -> MapData {
    let (w, h) = (10i32, 10i32);
    let (mut tiles, mut col) = fill_room(w, h, 4);
    set_tile(&mut tiles, &mut col, w, 5, 9, 3, C_WARP);

    MapData {
        id: MapId::DarkCaveVioletEntrance,
        name: "DARK CAVE",
        width: w, height: h, tiles, collision: col,
        warps: vec![
            // warp idx 0: return to Route 31 warp 2
            WarpDef { x: 5, y: 9, dest_map: MapId::Route31, dest_warp_id: 2 },
        ],
        npcs: vec![],
        coord_events: vec![],
        bg_events: vec![],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: 0,
    }
}

/// Generic connection stub -- minimal open map for south/west targets.
fn build_connection_stub(id: MapId, name: &'static str) -> MapData {
    let (w, h) = (40i32, 20i32);
    let tiles = vec![0u8; (w * h) as usize];
    let col = vec![C_FLOOR; (w * h) as usize];

    MapData {
        id,
        name,
        width: w, height: h, tiles, collision: col,
        warps: vec![],
        npcs: vec![],
        coord_events: vec![],
        bg_events: vec![],
        wild_encounters: None,
        connections: MapConnections::none(),
        music_id: 0,
    }
}
```

### Phase 2 Tests

Add to `maps.rs` `mod tests`:

```rust
    #[test]
    fn test_sprint5_maps_load() {
        let ids = [
            MapId::Route31, MapId::Route31VioletGate, MapId::VioletCity,
            MapId::VioletMart, MapId::VioletGym, MapId::EarlsPokemonAcademy,
            MapId::VioletNicknameSpeechHouse, MapId::VioletPokecenter1F,
            MapId::VioletKylesHouse, MapId::SproutTower1F,
            MapId::DarkCaveVioletEntrance, MapId::Route32, MapId::Route36,
        ];
        for &id in &ids {
            let m = load_map(id);
            assert!(m.width > 0, "Map {:?} has zero width", id);
            assert!(m.height > 0, "Map {:?} has zero height", id);
            assert_eq!(m.tiles.len(), (m.width * m.height) as usize, "tiles mismatch {:?}", id);
            assert_eq!(m.collision.len(), (m.width * m.height) as usize, "collision mismatch {:?}", id);
        }
    }

    #[test]
    fn test_route31_dimensions() {
        let m = load_map(MapId::Route31);
        assert_eq!(m.width, 40);
        assert_eq!(m.height, 18);
    }

    #[test]
    fn test_route31_violet_gate_dimensions() {
        let m = load_map(MapId::Route31VioletGate);
        assert_eq!(m.width, 10);
        assert_eq!(m.height, 8);
    }

    #[test]
    fn test_violet_city_dimensions() {
        let m = load_map(MapId::VioletCity);
        assert_eq!(m.width, 40);
        assert_eq!(m.height, 36);
    }

    #[test]
    fn test_route31_has_encounters() {
        let m = load_map(MapId::Route31);
        assert!(m.wild_encounters.is_some(), "Route 31 should have wild encounters");
        let table = m.wild_encounters.unwrap();
        assert_eq!(table.morning.len(), 7);
        assert_eq!(table.day.len(), 7);
        assert_eq!(table.night.len(), 7);
        assert_eq!(table.encounter_rate, 10);
    }

    #[test]
    fn test_route31_npc_count() {
        let m = load_map(MapId::Route31);
        assert_eq!(m.npcs.len(), 7, "Route 31 should have 7 NPCs");
    }

    #[test]
    fn test_violet_city_npc_count() {
        let m = load_map(MapId::VioletCity);
        assert_eq!(m.npcs.len(), 8, "Violet City should have 8 NPCs");
    }

    #[test]
    fn test_violet_city_warp_count() {
        let m = load_map(MapId::VioletCity);
        assert_eq!(m.warps.len(), 9, "Violet City should have 9 warps");
    }

    #[test]
    fn test_route31_gate_warp_count() {
        let m = load_map(MapId::Route31VioletGate);
        assert_eq!(m.warps.len(), 4, "Gate should have 4 warps");
    }

    #[test]
    fn test_sprint5_warp_bidirectional() {
        // Route 31 <-> Route 31 Violet Gate
        let r31 = load_map(MapId::Route31);
        let gate = load_map(MapId::Route31VioletGate);
        let vc = load_map(MapId::VioletCity);

        // Route31 warp 0 -> Gate warp 2, and Gate warp 2 -> Route31 warp 0
        assert_eq!(r31.warps[0].dest_map, MapId::Route31VioletGate);
        assert_eq!(r31.warps[0].dest_warp_id, 2);
        assert_eq!(gate.warps[2].dest_map, MapId::Route31);
        assert_eq!(gate.warps[2].dest_warp_id, 0);

        // Route31 warp 1 -> Gate warp 3, and Gate warp 3 -> Route31 warp 1
        assert_eq!(r31.warps[1].dest_map, MapId::Route31VioletGate);
        assert_eq!(r31.warps[1].dest_warp_id, 3);
        assert_eq!(gate.warps[3].dest_map, MapId::Route31);
        assert_eq!(gate.warps[3].dest_warp_id, 1);

        // Gate warp 0 -> VioletCity warp 7, and VioletCity warp 7 -> Gate warp 0
        assert_eq!(gate.warps[0].dest_map, MapId::VioletCity);
        assert_eq!(gate.warps[0].dest_warp_id, 7);
        assert_eq!(vc.warps[7].dest_map, MapId::Route31VioletGate);
        assert_eq!(vc.warps[7].dest_warp_id, 0);

        // Gate warp 1 -> VioletCity warp 8, and VioletCity warp 8 -> Gate warp 1
        assert_eq!(gate.warps[1].dest_map, MapId::VioletCity);
        assert_eq!(gate.warps[1].dest_warp_id, 8);
        assert_eq!(vc.warps[8].dest_map, MapId::Route31VioletGate);
        assert_eq!(vc.warps[8].dest_warp_id, 1);
    }

    #[test]
    fn test_violet_city_stubs_have_return_warps() {
        let stub_maps = [
            MapId::VioletMart, MapId::VioletGym, MapId::EarlsPokemonAcademy,
            MapId::VioletNicknameSpeechHouse, MapId::VioletPokecenter1F,
            MapId::VioletKylesHouse, MapId::SproutTower1F,
        ];
        for &id in &stub_maps {
            let m = load_map(id);
            assert!(!m.warps.is_empty(), "Stub {:?} should have at least one warp", id);
            let has_return = m.warps.iter().any(|w| w.dest_map == MapId::VioletCity);
            assert!(has_return, "Stub {:?} should warp back to VioletCity", id);
        }
    }

    #[test]
    fn test_dark_cave_stub_returns_to_route31() {
        let m = load_map(MapId::DarkCaveVioletEntrance);
        assert_eq!(m.warps.len(), 1);
        assert_eq!(m.warps[0].dest_map, MapId::Route31);
        assert_eq!(m.warps[0].dest_warp_id, 2);
    }

    #[test]
    fn test_route31_wade_trainer() {
        let m = load_map(MapId::Route31);
        let wade = m.npcs.iter().find(|n| n.name == "BUG_CATCHER_WADE");
        assert!(wade.is_some(), "Route 31 should have Bug Catcher Wade");
        let wade = wade.unwrap();
        assert_eq!(wade.trainer_range, Some(5), "Wade should have sight range 5");
        assert_eq!(wade.event_flag, Some(44), "Wade beaten flag should be EVENT_BEAT_BUG_CATCHER_WADE (44)");
    }
```

---

## Phase 3: events.rs — New Event Flags, Script IDs, Script Builders (~200 lines)

### 3A. New Event Flag Constants

Add after `EVENT_PLAYERS_NEIGHBORS_HOUSE_NEIGHBOR` (line 67):

```rust
// Sprint 5 event flags
pub const EVENT_BEAT_BUG_CATCHER_WADE: u16 = 44;
pub const EVENT_WADE_ASKED_FOR_PHONE_NUMBER: u16 = 45;
pub const EVENT_ROUTE_31_POTION: u16 = 46;
pub const EVENT_ROUTE_31_POKE_BALL: u16 = 47;
pub const EVENT_VIOLET_CITY_EARL: u16 = 48;
pub const EVENT_VIOLET_CITY_PP_UP: u16 = 49;
pub const EVENT_VIOLET_CITY_RARE_CANDY: u16 = 50;
pub const EVENT_VIOLET_CITY_HIDDEN_HYPER_POTION: u16 = 51;
pub const EVENT_ENGINE_FLYPOINT_VIOLET: u16 = 52;
pub const EVENT_GOT_TM50_NIGHTMARE: u16 = 53;
pub const EVENT_GOT_KENYA: u16 = 54;
pub const EVENT_GAVE_KENYA: u16 = 55;
pub const EVENT_TALKED_TO_MOM_AFTER_MYSTERY_EGG_QUEST: u16 = 56;
pub const EVENT_EARLS_ACADEMY_EARL: u16 = 57;
```

### 3B. New Script ID Constants

Add after `SCRIPT_MR_POKEMON_COINS` (line 207):

```rust
// Sprint 5: Route 31 scripts
pub const SCRIPT_ROUTE31_SIGN: u16 = 400;
pub const SCRIPT_DARK_CAVE_SIGN: u16 = 401;
pub const SCRIPT_TRAINER_WADE: u16 = 402;
pub const SCRIPT_ROUTE31_MAIL_RECIPIENT: u16 = 403;
pub const SCRIPT_ROUTE31_YOUNGSTER: u16 = 404;
pub const SCRIPT_ROUTE31_COOLTRAINER_M: u16 = 405;
pub const SCRIPT_ROUTE31_FRUIT_TREE: u16 = 406;
pub const SCRIPT_ROUTE31_POTION: u16 = 407;
pub const SCRIPT_ROUTE31_POKE_BALL: u16 = 408;

// Sprint 5: Route 31 Violet Gate scripts
pub const SCRIPT_GATE_OFFICER_VIOLET: u16 = 420;
pub const SCRIPT_GATE_COOLTRAINER_F_VIOLET: u16 = 421;

// Sprint 5: Violet City scripts
pub const SCRIPT_VIOLET_CITY_EARL: u16 = 430;
pub const SCRIPT_VIOLET_CITY_LASS: u16 = 431;
pub const SCRIPT_VIOLET_CITY_SUPER_NERD: u16 = 432;
pub const SCRIPT_VIOLET_CITY_GRAMPS: u16 = 433;
pub const SCRIPT_VIOLET_CITY_YOUNGSTER: u16 = 434;
pub const SCRIPT_VIOLET_CITY_FRUIT_TREE: u16 = 435;
pub const SCRIPT_VIOLET_CITY_PP_UP: u16 = 436;
pub const SCRIPT_VIOLET_CITY_RARE_CANDY: u16 = 437;
pub const SCRIPT_VIOLET_CITY_SIGN: u16 = 440;
pub const SCRIPT_VIOLET_GYM_SIGN: u16 = 441;
pub const SCRIPT_SPROUT_TOWER_SIGN: u16 = 442;
pub const SCRIPT_EARLS_ACADEMY_SIGN: u16 = 443;
pub const SCRIPT_VIOLET_POKECENTER_SIGN: u16 = 444;
pub const SCRIPT_VIOLET_MART_SIGN: u16 = 445;
pub const SCRIPT_VIOLET_CITY_HIDDEN_HYPER_POTION: u16 = 446;
```

### 3C. Update events.rs Import Line

Add new imports at top of events.rs (line 9-17):

```rust
use super::data::{
    BattleType, Direction, Emote, NpcState, PlayerState, Pokemon, SpeciesId,
    ITEM_BERRY, ITEM_MYSTIC_WATER, ITEM_POTION, ITEM_MYSTERY_EGG,
    ITEM_PP_UP, ITEM_RARE_CANDY, ITEM_PRZ_CURE_BERRY, ITEM_POKE_BALL, ITEM_HYPER_POTION,
    CYNDAQUIL, TOTODILE, CHIKORITA,
    CATERPIE, PIDGEY, RATTATA, WEEDLE,
    MUSIC_SHOW_ME_AROUND, MUSIC_RIVAL_ENCOUNTER, MUSIC_RIVAL_AFTER,
    MUSIC_PROF_OAK,
    HOPPIP,
};
```

### 3D. Update `get_script()` Registry

Add to the `get_script()` match (before the `_ => vec![ScriptStep::End]` fallback at line 835):

```rust
        // Sprint 5: Route 31 scripts
        SCRIPT_ROUTE31_SIGN => simple_text("ROUTE 31\nVIOLET CITY - CHERRYGROVE CITY"),
        SCRIPT_DARK_CAVE_SIGN => simple_text("DARK CAVE"),
        SCRIPT_TRAINER_WADE => build_trainer_wade_script(),
        SCRIPT_ROUTE31_MAIL_RECIPIENT => build_route31_mail_recipient_script(),
        SCRIPT_ROUTE31_YOUNGSTER => simple_text("YOUNGSTER: There are lots of\nPOKeMON in DARK CAVE!"),
        SCRIPT_ROUTE31_COOLTRAINER_M => simple_text("COOLTRAINER: DARK CAVE is pitch\nblack. You need FLASH to see."),
        SCRIPT_ROUTE31_FRUIT_TREE => build_fruit_tree_script(),
        SCRIPT_ROUTE31_POTION => build_route31_potion_script(),
        SCRIPT_ROUTE31_POKE_BALL => build_route31_poke_ball_script(),

        // Sprint 5: Route 31 Violet Gate scripts
        SCRIPT_GATE_OFFICER_VIOLET => simple_text("OFFICER: Did you visit\nSPROUT TOWER?"),
        SCRIPT_GATE_COOLTRAINER_F_VIOLET => simple_text("COOLTRAINER: I came too far out.\nI have to go back."),

        // Sprint 5: Violet City scripts
        SCRIPT_VIOLET_CITY_EARL => build_violet_city_earl_script(),
        SCRIPT_VIOLET_CITY_LASS => simple_text("LASS: There are ghosts in\nSPROUT TOWER! Normal-type\nmoves don't work on them!"),
        SCRIPT_VIOLET_CITY_SUPER_NERD => simple_text("SUPER NERD: If you beat the GYM\nLEADER, it's prime time!"),
        SCRIPT_VIOLET_CITY_GRAMPS => simple_text("GRAMPS: FALKNER inherited the\nGYM from his father."),
        SCRIPT_VIOLET_CITY_YOUNGSTER => simple_text("YOUNGSTER: There's a wiggly tree\nup ahead that won't budge!"),
        SCRIPT_VIOLET_CITY_FRUIT_TREE => build_fruit_tree_script(),
        SCRIPT_VIOLET_CITY_PP_UP => build_violet_city_pp_up_script(),
        SCRIPT_VIOLET_CITY_RARE_CANDY => build_violet_city_rare_candy_script(),
        SCRIPT_VIOLET_CITY_SIGN => simple_text("VIOLET CITY\nThe City of Nostalgic Scents"),
        SCRIPT_VIOLET_GYM_SIGN => simple_text("VIOLET CITY POKeMON GYM\nLEADER: FALKNER\n\"The Elegant Master of\nFlying POKeMON!\""),
        SCRIPT_SPROUT_TOWER_SIGN => simple_text("SPROUT TOWER"),
        SCRIPT_EARLS_ACADEMY_SIGN => simple_text("EARL'S POKeMON ACADEMY"),
        SCRIPT_VIOLET_POKECENTER_SIGN => simple_text("POKEMON CENTER"),
        SCRIPT_VIOLET_MART_SIGN => simple_text("VIOLET MART"),
        SCRIPT_VIOLET_CITY_HIDDEN_HYPER_POTION => build_violet_city_hidden_hyper_potion_script(),
```

### 3E. Script Builder Functions

Add these functions after the existing Sprint 4 script builders (after `build_mr_pokemon_talk_script`):

```rust
// ── Sprint 5 Script Builders ────────────────────────────────────────────────

fn build_trainer_wade_script() -> Vec<ScriptStep> {
    vec![
        // Wade NPC index is 2 in Route 31
        ScriptStep::FacingPlayer { npc_idx: 2 },
        ScriptStep::ShowText("WADE: I caught a bunch of\nPOKeMON! Let me battle with you!".to_string()),
        ScriptStep::LoadTrainerParty {
            party: vec![(CATERPIE, 2), (CATERPIE, 2), (WEEDLE, 3), (CATERPIE, 2)],
            beaten_flag: EVENT_BEAT_BUG_CATCHER_WADE,
        },
        ScriptStep::StartBattle { battle_type: BattleType::Normal },
        ScriptStep::ShowText("WADE: I need to catch stronger\nPOKeMON...".to_string()),
        ScriptStep::ShowText("WADE: Can I have your phone\nnumber? I'll call if I spot\nsome rare POKeMON!".to_string()),
        ScriptStep::SetEvent(EVENT_WADE_ASKED_FOR_PHONE_NUMBER),
        ScriptStep::End,
    ]
}

fn build_route31_mail_recipient_script() -> Vec<ScriptStep> {
    // Stub: the full Kenya sidequest requires Route 35.
    // For Sprint 5, just show the sleepy man dialogue.
    vec![
        ScriptStep::FacingPlayer { npc_idx: 0 },
        ScriptStep::ShowText("FISHER: ...I'm waiting for\nsomeone to deliver MAIL to me.\n...Zzz...".to_string()),
        ScriptStep::End,
    ]
}

fn build_route31_potion_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::CheckEvent { flag: EVENT_ROUTE_31_POTION, jump_if_true: 3 },
        ScriptStep::GiveItem { item_id: ITEM_POTION, count: 1 },
        ScriptStep::SetEvent(EVENT_ROUTE_31_POTION),
        // jump target 3:
        ScriptStep::ShowText("Found a POTION!".to_string()),
        ScriptStep::End,
    ]
}

fn build_route31_poke_ball_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::CheckEvent { flag: EVENT_ROUTE_31_POKE_BALL, jump_if_true: 3 },
        ScriptStep::GiveItem { item_id: ITEM_POKE_BALL, count: 1 },
        ScriptStep::SetEvent(EVENT_ROUTE_31_POKE_BALL),
        // jump target 3:
        ScriptStep::ShowText("Found a POKe BALL!".to_string()),
        ScriptStep::End,
    ]
}

fn build_violet_city_earl_script() -> Vec<ScriptStep> {
    // Sprint 5 simplification: Earl dialogue + flag, no complex movement sequence
    vec![
        ScriptStep::FacingPlayer { npc_idx: 0 },
        ScriptStep::CheckEvent { flag: EVENT_EARLS_ACADEMY_EARL, jump_if_true: 5 },
        ScriptStep::ShowText("EARL: I'm EARL! Want me to\nteach you about POKeMON?".to_string()),
        ScriptStep::ShowText("EARL: Follow me!".to_string()),
        ScriptStep::SetEvent(EVENT_EARLS_ACADEMY_EARL),
        // jump target 5:
        ScriptStep::ShowText("EARL: The POKeMON ACADEMY is\nwhere you learn about battling!".to_string()),
        ScriptStep::End,
    ]
}

fn build_violet_city_pp_up_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::CheckEvent { flag: EVENT_VIOLET_CITY_PP_UP, jump_if_true: 3 },
        ScriptStep::GiveItem { item_id: ITEM_PP_UP, count: 1 },
        ScriptStep::SetEvent(EVENT_VIOLET_CITY_PP_UP),
        // jump target 3:
        ScriptStep::ShowText("Found a PP UP!".to_string()),
        ScriptStep::End,
    ]
}

fn build_violet_city_rare_candy_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::CheckEvent { flag: EVENT_VIOLET_CITY_RARE_CANDY, jump_if_true: 3 },
        ScriptStep::GiveItem { item_id: ITEM_RARE_CANDY, count: 1 },
        ScriptStep::SetEvent(EVENT_VIOLET_CITY_RARE_CANDY),
        // jump target 3:
        ScriptStep::ShowText("Found a RARE CANDY!".to_string()),
        ScriptStep::End,
    ]
}

fn build_violet_city_hidden_hyper_potion_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::CheckEvent { flag: EVENT_VIOLET_CITY_HIDDEN_HYPER_POTION, jump_if_true: 3 },
        ScriptStep::GiveItem { item_id: ITEM_HYPER_POTION, count: 1 },
        ScriptStep::SetEvent(EVENT_VIOLET_CITY_HIDDEN_HYPER_POTION),
        // jump target 3:
        ScriptStep::ShowText("Found a HYPER POTION!".to_string()),
        ScriptStep::End,
    ]
}
```

### Phase 3 Tests

Add to `events.rs` `mod tests`:

```rust
    #[test]
    fn test_sprint5_event_flags() {
        let mut flags = EventFlags::new();
        assert!(!flags.has(EVENT_BEAT_BUG_CATCHER_WADE));
        flags.set(EVENT_BEAT_BUG_CATCHER_WADE);
        assert!(flags.has(EVENT_BEAT_BUG_CATCHER_WADE));
        assert!(!flags.has(EVENT_ENGINE_FLYPOINT_VIOLET));
        flags.set(EVENT_ENGINE_FLYPOINT_VIOLET);
        assert!(flags.has(EVENT_ENGINE_FLYPOINT_VIOLET));
    }

    #[test]
    fn test_wade_script_loads() {
        let steps = get_script(SCRIPT_TRAINER_WADE);
        assert!(steps.len() > 3, "Wade script should have multiple steps");
        // Check it has a LoadTrainerParty step
        let has_party = steps.iter().any(|s| matches!(s, ScriptStep::LoadTrainerParty { .. }));
        assert!(has_party, "Wade script should load trainer party");
    }

    #[test]
    fn test_wade_party_composition() {
        let steps = get_script(SCRIPT_TRAINER_WADE);
        for step in &steps {
            if let ScriptStep::LoadTrainerParty { party, beaten_flag } = step {
                assert_eq!(party.len(), 4, "Wade should have 4 Pokemon");
                assert_eq!(party[0], (CATERPIE, 2));
                assert_eq!(party[1], (CATERPIE, 2));
                assert_eq!(party[2], (WEEDLE, 3));
                assert_eq!(party[3], (CATERPIE, 2));
                assert_eq!(*beaten_flag, EVENT_BEAT_BUG_CATCHER_WADE);
                return;
            }
        }
        panic!("Wade script should contain LoadTrainerParty step");
    }

    #[test]
    fn test_sprint5_sign_scripts_load() {
        let sign_ids = [
            SCRIPT_ROUTE31_SIGN, SCRIPT_DARK_CAVE_SIGN,
            SCRIPT_VIOLET_CITY_SIGN, SCRIPT_VIOLET_GYM_SIGN,
            SCRIPT_SPROUT_TOWER_SIGN, SCRIPT_EARLS_ACADEMY_SIGN,
            SCRIPT_VIOLET_POKECENTER_SIGN, SCRIPT_VIOLET_MART_SIGN,
        ];
        for &id in &sign_ids {
            let steps = get_script(id);
            assert!(steps.len() >= 2, "Sign script {} should have at least ShowText + End", id);
            assert!(matches!(steps[0], ScriptStep::ShowText(_)), "Sign script {} should start with ShowText", id);
        }
    }

    #[test]
    fn test_item_ball_scripts_check_flags() {
        let item_scripts = [
            SCRIPT_ROUTE31_POTION, SCRIPT_ROUTE31_POKE_BALL,
            SCRIPT_VIOLET_CITY_PP_UP, SCRIPT_VIOLET_CITY_RARE_CANDY,
            SCRIPT_VIOLET_CITY_HIDDEN_HYPER_POTION,
        ];
        for &id in &item_scripts {
            let steps = get_script(id);
            let has_check = steps.iter().any(|s| matches!(s, ScriptStep::CheckEvent { .. }));
            assert!(has_check, "Item script {} should check event flag", id);
            let has_give = steps.iter().any(|s| matches!(s, ScriptStep::GiveItem { .. }));
            assert!(has_give, "Item script {} should give an item", id);
            let has_set = steps.iter().any(|s| matches!(s, ScriptStep::SetEvent(_)));
            assert!(has_set, "Item script {} should set event flag", id);
        }
    }
```

---

## Phase 4: mod.rs — Integration (~30 lines)

### 4A. Violet City Map Callback

In `check_map_callbacks()` (around line 460-477), add a new arm before `_ => {}`:

```rust
            MapId::VioletCity => {
                self.event_flags.set(EVENT_ENGINE_FLYPOINT_VIOLET);
            }
```

### 4B. Update `warp_to_last_pokecenter()`

Replace the existing `warp_to_last_pokecenter()` method (around line 450-458):

```rust
    fn warp_to_last_pokecenter(&mut self) {
        let dest = if self.event_flags.has(EVENT_ENGINE_FLYPOINT_VIOLET) {
            MapId::VioletPokecenter1F
        } else if self.event_flags.has(EVENT_ENGINE_FLYPOINT_CHERRYGROVE) {
            MapId::CherrygrovePokecenter1F
        } else {
            MapId::ElmsLab
        };
        self.change_map(dest, 0);
        self.phase = GamePhase::Overworld;
    }
```

### Phase 4 Tests

Add to `mod.rs` `mod tests`:

```rust
    #[test]
    fn test_violet_city_flypoint_set_on_entry() {
        let mut sim = PokemonV2Sim::with_state(
            MapId::VioletCity, 20, 20, vec![Pokemon::new(155, 5)],
        );
        sim.check_map_callbacks();
        assert!(sim.event_flags.has(EVENT_ENGINE_FLYPOINT_VIOLET),
            "Entering Violet City should set flypoint flag");
    }

    #[test]
    fn test_blackout_routes_to_violet_pokecenter() {
        let mut sim = PokemonV2Sim::with_state(
            MapId::VioletCity, 20, 20, vec![Pokemon::new(155, 5)],
        );
        sim.event_flags.set(EVENT_ENGINE_FLYPOINT_VIOLET);
        sim.event_flags.set(EVENT_ENGINE_FLYPOINT_CHERRYGROVE);
        sim.warp_to_last_pokecenter();
        assert_eq!(sim.current_map_id, MapId::VioletPokecenter1F,
            "Blackout with Violet flypoint should go to Violet Pokecenter");
    }

    #[test]
    fn test_all_sprint5_maps_load_from_mod() {
        let ids = [
            MapId::Route31, MapId::Route31VioletGate, MapId::VioletCity,
            MapId::VioletMart, MapId::VioletGym, MapId::EarlsPokemonAcademy,
            MapId::VioletNicknameSpeechHouse, MapId::VioletPokecenter1F,
            MapId::VioletKylesHouse, MapId::SproutTower1F,
            MapId::DarkCaveVioletEntrance, MapId::Route32, MapId::Route36,
        ];
        for &id in &ids {
            let m = load_map(id);
            assert!(m.width > 0);
            assert!(m.height > 0);
        }
    }

    #[test]
    fn test_route31_to_gate_path_exists() {
        // Verify Route 31 -> Gate warp chain works
        let r31 = load_map(MapId::Route31);
        assert_eq!(r31.warps[0].dest_map, MapId::Route31VioletGate);
        let gate = load_map(MapId::Route31VioletGate);
        assert_eq!(gate.warps[0].dest_map, MapId::VioletCity);
    }
```

---

## Phase 5: Polish + QA

### 5A. Compilation Check

Run `cargo test` from `engine/` directory. Fix any compilation errors.

### 5B. Verify All Map Loads

Ensure all 33 MapId variants (21 existing + 12 new) are dispatched in `load_map()`.

### 5C. Warp Bidirectional Audit

Verify every warp has a matching return warp:
- Route31 warps 0,1 <-> Gate warps 2,3
- Gate warps 0,1 <-> VioletCity warps 7,8
- Route31 warp 2 <-> DarkCaveVioletEntrance warp 0
- VioletCity warps 0-6 <-> stub building warps 0

### 5D. Encounter Table Audit

Verify Route 31 encounter table matches pokecrystal exactly:
- 7 slots per time period
- encounter_rate = 10
- morning: Ledyba/4, Caterpie/4, Bellsprout/5, Pidgey/5, Weedle/4, Hoppip/5, Hoppip/5
- day: Pidgey/4, Caterpie/4, Bellsprout/5, Pidgey/5, Weedle/4, Hoppip/5, Hoppip/5
- night: Spinarak/4, Poliwag/4, Bellsprout/5, Hoothoot/5, Zubat/4, Gastly/5, Gastly/5

### 5E. Wade Trainer Audit

Verify Wade's party matches pokecrystal `parties.asm` BUG_CATCHER WADE:
- Caterpie Lv2
- Caterpie Lv2
- Weedle Lv3
- Caterpie Lv2

### 5F. Full Test Run

```bash
cd engine && cargo test 2>&1 | tail -20
```

All existing tests must still pass. New Sprint 5 tests must pass. Target: 0 failures.

---

## Summary of Changes

| File | Est. Lines | Key Changes |
|------|-----------|-------------|
| `data.rs` | ~80 | 2 species (Bellsprout, Gastly), 4 moves (VineWhip, Hypnosis, Lick, Growth), 4 item constants, 2 music constants |
| `maps.rs` | ~500 | 12 new MapId variants, build_route31() full implementation, build_route31_violet_gate(), build_violet_city(), build_violet_city_stub() x7, build_dark_cave_stub(), build_connection_stub() x2, build_route31_encounters(), load_map() dispatcher |
| `events.rs` | ~200 | 14 event flags, 27 script IDs, build_trainer_wade_script(), build_route31_mail_recipient_script(), 7 item ball scripts, earl script, get_script() registry |
| `mod.rs` | ~30 | VioletCity callback, warp_to_last_pokecenter update |
| **Total** | **~810** | |

---

## Warp Topology Summary (Quick Reference)

```
Route30 --[south connection]--> Route31
Route31 warp 0 (4,6) <--> Route31VioletGate warp 2 (9,4)
Route31 warp 1 (4,7) <--> Route31VioletGate warp 3 (9,5)
Route31VioletGate warp 0 (0,4) <--> VioletCity warp 7 (39,24)
Route31VioletGate warp 1 (0,5) <--> VioletCity warp 8 (39,25)
Route31 warp 2 (34,5) <--> DarkCaveVioletEntrance warp 0 (5,9)
VioletCity warp 0 (9,17) <--> VioletMart warp 0
VioletCity warp 1 (18,17) <--> VioletGym warp 0
VioletCity warp 2 (30,17) <--> EarlsPokemonAcademy warp 0
VioletCity warp 3 (3,15) <--> VioletNicknameSpeechHouse warp 0
VioletCity warp 4 (31,25) <--> VioletPokecenter1F warp 0
VioletCity warp 5 (21,29) <--> VioletKylesHouse warp 0
VioletCity warp 6 (23,5) <--> SproutTower1F warp 0
VioletCity --[south connection]--> Route32 (stub)
VioletCity --[east connection]--> Route31 (offset 9)
VioletCity --[west connection]--> Route36 (stub)
```
