/// S-League: Monster Collection RPG
///
/// A full-featured monster collection RPG built on the crusty engine's minigolf mechanics.
/// Battle system uses slingshot aiming (the "Spirit Strike" system) where you pull back
/// and release energy orbs to damage enemy monsters. Stylistically this is a Pokemon-like
/// monster collection game with towns, wild areas, shops, healing, and bosses.
///
/// Mobile-first 480x720 portrait. Tap/drag to navigate overworld, drag to aim in battle.

use crate::engine::Engine;
use crate::tilemap::{TileMap, Tile, TileType};
use crate::rendering::color::Color;
use crate::rendering::shapes;
use crate::rendering::text;
use crate::rendering::screen_fx::ScreenEffect;
use crate::sound::{SoundCommand, Waveform};

// ═══════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════

const TILE_SIZE: f64 = 16.0;
const MAP_W: usize = 30; // tiles wide
const MAP_H: usize = 45; // tiles tall
const WIDTH: f64 = 480.0;
const HEIGHT: f64 = 720.0;

// Camera for tilemap rendering
const TILEMAP_CAM_X: f64 = 240.0;
const TILEMAP_CAM_Y: f64 = 360.0;

// Game modes stored in global_state "mode"
const MODE_TITLE: f64 = 0.0;
const MODE_OVERWORLD: f64 = 1.0;
const MODE_BATTLE: f64 = 2.0;
const MODE_BATTLE_RESULT: f64 = 3.0;
const MODE_MENU: f64 = 4.0;
const MODE_DIALOGUE: f64 = 5.0;
const MODE_SHOP: f64 = 6.0;
const MODE_HEAL: f64 = 7.0;
const MODE_CATCH_ANIM: f64 = 8.0;
const MODE_TRANSITION: f64 = 9.0;

// Battle sub-phases
const BPHASE_INTRO: f64 = 0.0;
const BPHASE_PLAYER_AIM: f64 = 1.0;
const BPHASE_PLAYER_SHOT: f64 = 2.0;
const BPHASE_ENEMY_TURN: f64 = 3.0;
const _BPHASE_CATCH: f64 = 4.0;

// Tile custom IDs for overworld
const TILE_GRASS: u16 = 0;
const TILE_PATH: u16 = 1;
const TILE_WILD: u16 = 2;
const TILE_WATER: u16 = 3;
const TILE_BUILDING: u16 = 4;
const TILE_DOOR: u16 = 5;
const TILE_TREE: u16 = 6;
const TILE_FLOWER: u16 = 7;
const TILE_SAND: u16 = 8;
const TILE_CAVE: u16 = 9;
const TILE_SNOW: u16 = 10;
const TILE_DARK: u16 = 11;
const TILE_CRYSTAL: u16 = 12;
const TILE_HEAL: u16 = 13;
const TILE_SHOP: u16 = 14;
const TILE_BOSS: u16 = 15;

// Battle arena tile IDs
const TILE_ARENA: u16 = 50;
const _TILE_WALL: u16 = 51;
const TILE_HOLE: u16 = 52;
const TILE_BUMPER: u16 = 53;

// ═══════════════════════════════════════════════════════════════════════
// COLORS
// ═══════════════════════════════════════════════════════════════════════

const COL_BG: Color = Color { r: 20, g: 12, b: 28, a: 255 };
const COL_GRASS: Color = Color { r: 56, g: 128, b: 56, a: 255 };
const _COL_DARK_GRASS: Color = Color { r: 38, g: 92, b: 38, a: 255 };
const COL_PATH: Color = Color { r: 194, g: 178, b: 128, a: 255 };
const COL_WILD: Color = Color { r: 34, g: 120, b: 34, a: 255 };
const COL_WILD_ACCENT: Color = Color { r: 50, g: 160, b: 50, a: 255 };
const COL_WATER: Color = Color { r: 48, g: 96, b: 180, a: 255 };
const COL_BUILDING: Color = Color { r: 140, g: 120, b: 100, a: 255 };
const COL_DOOR: Color = Color { r: 120, g: 72, b: 40, a: 255 };
const _COL_TREE_TRUNK: Color = Color { r: 100, g: 70, b: 40, a: 255 };
const COL_TREE_TOP: Color = Color { r: 30, g: 100, b: 30, a: 255 };
const COL_FLOWER: Color = Color { r: 220, g: 80, b: 120, a: 255 };
const COL_SAND: Color = Color { r: 220, g: 200, b: 140, a: 255 };
const COL_CAVE: Color = Color { r: 80, g: 70, b: 60, a: 255 };
const COL_SNOW: Color = Color { r: 220, g: 230, b: 240, a: 255 };
const COL_DARK_TILE: Color = Color { r: 40, g: 30, b: 50, a: 255 };
const COL_CRYSTAL: Color = Color { r: 160, g: 200, b: 240, a: 255 };
const COL_HEAL: Color = Color { r: 255, g: 120, b: 120, a: 255 };
const COL_SHOP_TILE: Color = Color { r: 100, g: 140, b: 200, a: 255 };
const COL_BOSS_TILE: Color = Color { r: 180, g: 40, b: 40, a: 255 };

// UI colors
const COL_UI_BG: Color = Color { r: 24, g: 20, b: 37, a: 255 };
const COL_UI_BORDER: Color = Color { r: 80, g: 70, b: 100, a: 255 };
const COL_UI_TEXT: Color = Color { r: 230, g: 230, b: 230, a: 255 };
const COL_UI_HIGHLIGHT: Color = Color { r: 255, g: 220, b: 80, a: 255 };
const COL_HP_BAR: Color = Color { r: 80, g: 200, b: 80, a: 255 };
const COL_HP_BG: Color = Color { r: 60, g: 30, b: 30, a: 255 };
const COL_XP_BAR: Color = Color { r: 80, g: 120, b: 220, a: 255 };
const COL_PLAYER: Color = Color { r: 80, g: 180, b: 255, a: 255 };
const _COL_ENERGY_ORB: Color = Color { r: 255, g: 200, b: 60, a: 255 };
const COL_WHITE: Color = Color::WHITE;
const COL_BLACK: Color = Color::BLACK;

// ═══════════════════════════════════════════════════════════════════════
// ELEMENT TYPES & TYPE CHART
// ═══════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, PartialEq, Debug)]
enum Element {
    Normal,   // 0
    Fire,     // 1
    Water,    // 2
    Leaf,     // 3
    Electric, // 4
    Earth,    // 5
    Ice,      // 6
    Shadow,   // 7
    Light,    // 8
}

impl Element {
    fn index(self) -> usize {
        match self {
            Element::Normal => 0,
            Element::Fire => 1,
            Element::Water => 2,
            Element::Leaf => 3,
            Element::Electric => 4,
            Element::Earth => 5,
            Element::Ice => 6,
            Element::Shadow => 7,
            Element::Light => 8,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Element::Normal => "Normal",
            Element::Fire => "Fire",
            Element::Water => "Water",
            Element::Leaf => "Leaf",
            Element::Electric => "Electric",
            Element::Earth => "Earth",
            Element::Ice => "Ice",
            Element::Shadow => "Shadow",
            Element::Light => "Light",
        }
    }

    fn color(self) -> Color {
        match self {
            Element::Normal => Color { r: 168, g: 168, b: 120, a: 255 },
            Element::Fire => Color { r: 240, g: 80, b: 48, a: 255 },
            Element::Water => Color { r: 64, g: 144, b: 240, a: 255 },
            Element::Leaf => Color { r: 80, g: 200, b: 80, a: 255 },
            Element::Electric => Color { r: 248, g: 208, b: 48, a: 255 },
            Element::Earth => Color { r: 180, g: 140, b: 80, a: 255 },
            Element::Ice => Color { r: 140, g: 210, b: 240, a: 255 },
            Element::Shadow => Color { r: 100, g: 60, b: 140, a: 255 },
            Element::Light => Color { r: 255, g: 240, b: 180, a: 255 },
        }
    }
}

/// Type effectiveness: 1.5 = super effective, 0.5 = not effective, 1.0 = neutral
fn type_effectiveness(attacker: Element, defender: Element) -> f64 {
    // Rows = attacker, Cols = defender
    // Normal Fire Water Leaf Electric Earth Ice Shadow Light
    const CHART: [[f64; 9]; 9] = [
        // Normal attacks:
        [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        // Fire attacks:
        [1.0, 0.5, 0.5, 1.5, 1.0, 1.0, 1.5, 1.0, 1.0],
        // Water attacks:
        [1.0, 1.5, 0.5, 0.5, 1.0, 1.5, 1.0, 1.0, 1.0],
        // Leaf attacks:
        [1.0, 0.5, 1.5, 0.5, 1.0, 1.5, 0.5, 1.0, 1.0],
        // Electric attacks:
        [1.0, 1.0, 1.5, 0.5, 0.5, 0.5, 1.0, 1.0, 1.0],
        // Earth attacks:
        [1.0, 1.5, 1.0, 0.5, 1.5, 0.5, 1.0, 1.0, 1.0],
        // Ice attacks:
        [1.0, 0.5, 1.0, 1.5, 1.0, 1.0, 0.5, 1.0, 1.0],
        // Shadow attacks:
        [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.5],
        // Light attacks:
        [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.5, 0.5],
    ];
    CHART[attacker.index()][defender.index()]
}

// ═══════════════════════════════════════════════════════════════════════
// MONSTER SPECIES DATABASE
// ═══════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct MonsterSpecies {
    id: u8,
    name: &'static str,
    element: Element,
    base_hp: f64,
    base_atk: f64,
    base_def: f64,
    base_spd: f64,
    body_color: Color,
    accent_color: Color,
    catch_rate: f64, // 0.0-1.0 base catch probability
    evolves_to: Option<u8>, // species id to evolve into
    evolve_level: u8,
    desc: &'static str,
}

const NUM_SPECIES: usize = 24;

fn species_db() -> [MonsterSpecies; NUM_SPECIES] {
    [
        // ── Starter line (Normal) ──
        MonsterSpecies {
            id: 0, name: "Sproutail", element: Element::Leaf,
            base_hp: 45.0, base_atk: 12.0, base_def: 10.0, base_spd: 8.0,
            body_color: Color { r: 80, g: 180, b: 80, a: 255 },
            accent_color: Color { r: 40, g: 120, b: 40, a: 255 },
            catch_rate: 0.0, evolves_to: Some(1), evolve_level: 16,
            desc: "A small sprout creature",
        },
        MonsterSpecies {
            id: 1, name: "Thornvine", element: Element::Leaf,
            base_hp: 70.0, base_atk: 20.0, base_def: 16.0, base_spd: 12.0,
            body_color: Color { r: 60, g: 160, b: 60, a: 255 },
            accent_color: Color { r: 30, g: 100, b: 30, a: 255 },
            catch_rate: 0.0, evolves_to: None, evolve_level: 0,
            desc: "Evolved vine warrior",
        },
        // ── Fire line ──
        MonsterSpecies {
            id: 2, name: "Emberpup", element: Element::Fire,
            base_hp: 40.0, base_atk: 14.0, base_def: 8.0, base_spd: 10.0,
            body_color: Color { r: 240, g: 120, b: 60, a: 255 },
            accent_color: Color { r: 200, g: 60, b: 30, a: 255 },
            catch_rate: 0.4, evolves_to: Some(3), evolve_level: 18,
            desc: "A fiery young pup",
        },
        MonsterSpecies {
            id: 3, name: "Blazewolf", element: Element::Fire,
            base_hp: 65.0, base_atk: 24.0, base_def: 14.0, base_spd: 18.0,
            body_color: Color { r: 220, g: 80, b: 40, a: 255 },
            accent_color: Color { r: 180, g: 40, b: 20, a: 255 },
            catch_rate: 0.15, evolves_to: None, evolve_level: 0,
            desc: "Fierce flame wolf",
        },
        // ── Water line ──
        MonsterSpecies {
            id: 4, name: "Bubblefin", element: Element::Water,
            base_hp: 50.0, base_atk: 10.0, base_def: 12.0, base_spd: 9.0,
            body_color: Color { r: 80, g: 160, b: 240, a: 255 },
            accent_color: Color { r: 40, g: 100, b: 200, a: 255 },
            catch_rate: 0.45, evolves_to: Some(5), evolve_level: 16,
            desc: "A bubbly fish",
        },
        MonsterSpecies {
            id: 5, name: "Tidalord", element: Element::Water,
            base_hp: 80.0, base_atk: 18.0, base_def: 20.0, base_spd: 14.0,
            body_color: Color { r: 40, g: 100, b: 200, a: 255 },
            accent_color: Color { r: 20, g: 60, b: 140, a: 255 },
            catch_rate: 0.12, evolves_to: None, evolve_level: 0,
            desc: "Master of tides",
        },
        // ── Electric ──
        MonsterSpecies {
            id: 6, name: "Zapkit", element: Element::Electric,
            base_hp: 35.0, base_atk: 15.0, base_def: 7.0, base_spd: 14.0,
            body_color: Color { r: 248, g: 220, b: 80, a: 255 },
            accent_color: Color { r: 200, g: 160, b: 40, a: 255 },
            catch_rate: 0.35, evolves_to: Some(7), evolve_level: 20,
            desc: "Tiny electric critter",
        },
        MonsterSpecies {
            id: 7, name: "Voltiger", element: Element::Electric,
            base_hp: 60.0, base_atk: 25.0, base_def: 12.0, base_spd: 22.0,
            body_color: Color { r: 240, g: 200, b: 40, a: 255 },
            accent_color: Color { r: 200, g: 140, b: 20, a: 255 },
            catch_rate: 0.1, evolves_to: None, evolve_level: 0,
            desc: "Lightning tiger",
        },
        // ── Earth ──
        MonsterSpecies {
            id: 8, name: "Pebblet", element: Element::Earth,
            base_hp: 55.0, base_atk: 11.0, base_def: 16.0, base_spd: 5.0,
            body_color: Color { r: 180, g: 150, b: 100, a: 255 },
            accent_color: Color { r: 140, g: 110, b: 70, a: 255 },
            catch_rate: 0.5, evolves_to: Some(9), evolve_level: 18,
            desc: "A living pebble",
        },
        MonsterSpecies {
            id: 9, name: "Bouldox", element: Element::Earth,
            base_hp: 90.0, base_atk: 20.0, base_def: 28.0, base_spd: 8.0,
            body_color: Color { r: 160, g: 120, b: 80, a: 255 },
            accent_color: Color { r: 120, g: 80, b: 50, a: 255 },
            catch_rate: 0.1, evolves_to: None, evolve_level: 0,
            desc: "Immovable boulder ox",
        },
        // ── Ice ──
        MonsterSpecies {
            id: 10, name: "Frostkit", element: Element::Ice,
            base_hp: 38.0, base_atk: 13.0, base_def: 9.0, base_spd: 11.0,
            body_color: Color { r: 160, g: 220, b: 240, a: 255 },
            accent_color: Color { r: 100, g: 180, b: 220, a: 255 },
            catch_rate: 0.35, evolves_to: Some(11), evolve_level: 20,
            desc: "A tiny frost sprite",
        },
        MonsterSpecies {
            id: 11, name: "Glacirex", element: Element::Ice,
            base_hp: 68.0, base_atk: 22.0, base_def: 18.0, base_spd: 16.0,
            body_color: Color { r: 120, g: 190, b: 220, a: 255 },
            accent_color: Color { r: 80, g: 150, b: 200, a: 255 },
            catch_rate: 0.08, evolves_to: None, evolve_level: 0,
            desc: "Ice rex of the peaks",
        },
        // ── Shadow ──
        MonsterSpecies {
            id: 12, name: "Shadewisp", element: Element::Shadow,
            base_hp: 36.0, base_atk: 16.0, base_def: 8.0, base_spd: 13.0,
            body_color: Color { r: 100, g: 60, b: 140, a: 255 },
            accent_color: Color { r: 60, g: 30, b: 100, a: 255 },
            catch_rate: 0.3, evolves_to: Some(13), evolve_level: 22,
            desc: "A wispy shadow",
        },
        MonsterSpecies {
            id: 13, name: "Duskfiend", element: Element::Shadow,
            base_hp: 62.0, base_atk: 26.0, base_def: 14.0, base_spd: 20.0,
            body_color: Color { r: 80, g: 40, b: 120, a: 255 },
            accent_color: Color { r: 50, g: 20, b: 80, a: 255 },
            catch_rate: 0.06, evolves_to: None, evolve_level: 0,
            desc: "Terror of twilight",
        },
        // ── Light ──
        MonsterSpecies {
            id: 14, name: "Glimmer", element: Element::Light,
            base_hp: 42.0, base_atk: 14.0, base_def: 10.0, base_spd: 12.0,
            body_color: Color { r: 255, g: 240, b: 180, a: 255 },
            accent_color: Color { r: 220, g: 200, b: 140, a: 255 },
            catch_rate: 0.3, evolves_to: Some(15), evolve_level: 22,
            desc: "A tiny light being",
        },
        MonsterSpecies {
            id: 15, name: "Radiance", element: Element::Light,
            base_hp: 72.0, base_atk: 24.0, base_def: 16.0, base_spd: 18.0,
            body_color: Color { r: 255, g: 230, b: 150, a: 255 },
            accent_color: Color { r: 240, g: 200, b: 100, a: 255 },
            catch_rate: 0.06, evolves_to: None, evolve_level: 0,
            desc: "Blinding brilliance",
        },
        // ── Normal ──
        MonsterSpecies {
            id: 16, name: "Fluffmole", element: Element::Normal,
            base_hp: 48.0, base_atk: 10.0, base_def: 10.0, base_spd: 8.0,
            body_color: Color { r: 200, g: 180, b: 160, a: 255 },
            accent_color: Color { r: 160, g: 140, b: 120, a: 255 },
            catch_rate: 0.6, evolves_to: None, evolve_level: 0,
            desc: "Fluffy common mole",
        },
        MonsterSpecies {
            id: 17, name: "Scurratt", element: Element::Normal,
            base_hp: 38.0, base_atk: 12.0, base_def: 8.0, base_spd: 12.0,
            body_color: Color { r: 160, g: 140, b: 120, a: 255 },
            accent_color: Color { r: 120, g: 100, b: 80, a: 255 },
            catch_rate: 0.55, evolves_to: None, evolve_level: 0,
            desc: "Quick little rodent",
        },
        // ── Bosses (uncatchable) ──
        MonsterSpecies {
            id: 18, name: "Magmadon", element: Element::Fire,
            base_hp: 120.0, base_atk: 30.0, base_def: 22.0, base_spd: 12.0,
            body_color: Color { r: 200, g: 50, b: 30, a: 255 },
            accent_color: Color { r: 255, g: 120, b: 40, a: 255 },
            catch_rate: 0.0, evolves_to: None, evolve_level: 0,
            desc: "Guardian of Ember Hollow",
        },
        MonsterSpecies {
            id: 19, name: "Tsunadon", element: Element::Water,
            base_hp: 130.0, base_atk: 26.0, base_def: 28.0, base_spd: 14.0,
            body_color: Color { r: 20, g: 80, b: 180, a: 255 },
            accent_color: Color { r: 60, g: 140, b: 255, a: 255 },
            catch_rate: 0.0, evolves_to: None, evolve_level: 0,
            desc: "Lord of Coral Shore",
        },
        MonsterSpecies {
            id: 20, name: "Thunderex", element: Element::Electric,
            base_hp: 100.0, base_atk: 34.0, base_def: 18.0, base_spd: 24.0,
            body_color: Color { r: 240, g: 180, b: 20, a: 255 },
            accent_color: Color { r: 255, g: 255, b: 100, a: 255 },
            catch_rate: 0.0, evolves_to: None, evolve_level: 0,
            desc: "Warden of Sparkridge",
        },
        MonsterSpecies {
            id: 21, name: "Abyssking", element: Element::Shadow,
            base_hp: 140.0, base_atk: 32.0, base_def: 24.0, base_spd: 18.0,
            body_color: Color { r: 50, g: 20, b: 80, a: 255 },
            accent_color: Color { r: 120, g: 60, b: 180, a: 255 },
            catch_rate: 0.0, evolves_to: None, evolve_level: 0,
            desc: "Ruler of Shadow Vale",
        },
        MonsterSpecies {
            id: 22, name: "Crystalion", element: Element::Light,
            base_hp: 150.0, base_atk: 35.0, base_def: 30.0, base_spd: 20.0,
            body_color: Color { r: 240, g: 230, b: 255, a: 255 },
            accent_color: Color { r: 200, g: 180, b: 255, a: 255 },
            catch_rate: 0.0, evolves_to: None, evolve_level: 0,
            desc: "Final guardian",
        },
        // ── Rare catchable ──
        MonsterSpecies {
            id: 23, name: "Mossbear", element: Element::Leaf,
            base_hp: 60.0, base_atk: 18.0, base_def: 15.0, base_spd: 7.0,
            body_color: Color { r: 100, g: 160, b: 80, a: 255 },
            accent_color: Color { r: 60, g: 120, b: 40, a: 255 },
            catch_rate: 0.2, evolves_to: None, evolve_level: 0,
            desc: "Rare forest bear",
        },
    ]
}

fn get_species(id: u8) -> MonsterSpecies {
    let db = species_db();
    db[id as usize].clone()
}

// ═══════════════════════════════════════════════════════════════════════
// ZONE / AREA DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, PartialEq, Debug)]
enum Zone {
    PebbleTown,     // Starting town - heal + shop
    VerdantPath,    // Route 1 - Leaf/Normal wilds
    EmberHollow,    // Fire zone - boss Magmadon
    CoralShore,     // Water zone - boss Tsunadon
    Sparkridge,     // Electric zone - boss Thunderex
    DeepCave,       // Earth/Ice zone
    ShadowVale,     // Shadow zone - boss Abyssking
    CrystalSpire,   // Final zone - boss Crystalion
    Frostpeak,      // Ice bonus zone
}

impl Zone {
    fn name(self) -> &'static str {
        match self {
            Zone::PebbleTown => "Pebble Town",
            Zone::VerdantPath => "Verdant Path",
            Zone::EmberHollow => "Ember Hollow",
            Zone::CoralShore => "Coral Shore",
            Zone::Sparkridge => "Sparkridge",
            Zone::DeepCave => "Deep Cave",
            Zone::ShadowVale => "Shadow Vale",
            Zone::CrystalSpire => "Crystal Spire",
            Zone::Frostpeak => "Frostpeak",
        }
    }

    /// Wild monster encounter table: (species_id, min_level, max_level, weight)
    fn encounter_table(self) -> &'static [(u8, u8, u8, u8)] {
        match self {
            Zone::PebbleTown => &[],
            Zone::VerdantPath => &[
                (16, 2, 5, 40),   // Fluffmole
                (17, 2, 5, 30),   // Scurratt
                (0, 3, 6, 20),    // Sproutail (rare starter encounter)
                (23, 5, 7, 10),   // Mossbear (rare)
            ],
            Zone::EmberHollow => &[
                (2, 8, 12, 45),   // Emberpup
                (8, 8, 11, 30),   // Pebblet
                (17, 7, 10, 25),  // Scurratt
            ],
            Zone::CoralShore => &[
                (4, 10, 14, 45),  // Bubblefin
                (6, 10, 13, 30),  // Zapkit
                (16, 9, 12, 25),  // Fluffmole
            ],
            Zone::Sparkridge => &[
                (6, 14, 18, 40),  // Zapkit
                (2, 13, 17, 30),  // Emberpup
                (8, 14, 16, 30),  // Pebblet
            ],
            Zone::DeepCave => &[
                (8, 16, 20, 35),  // Pebblet
                (10, 16, 19, 35), // Frostkit
                (12, 17, 20, 30), // Shadewisp
            ],
            Zone::ShadowVale => &[
                (12, 20, 24, 50), // Shadewisp
                (14, 20, 23, 30), // Glimmer
                (17, 18, 22, 20), // Scurratt
            ],
            Zone::CrystalSpire => &[
                (14, 24, 28, 40), // Glimmer
                (12, 24, 27, 30), // Shadewisp
                (6, 22, 26, 30),  // Zapkit
            ],
            Zone::Frostpeak => &[
                (10, 18, 22, 50), // Frostkit
                (4, 18, 21, 30),  // Bubblefin
                (23, 20, 24, 20), // Mossbear
            ],
        }
    }

    fn boss(self) -> Option<(u8, u8)> {
        match self {
            Zone::EmberHollow => Some((18, 15)),   // Magmadon Lv15
            Zone::CoralShore => Some((19, 18)),    // Tsunadon Lv18
            Zone::Sparkridge => Some((20, 22)),    // Thunderex Lv22
            Zone::ShadowVale => Some((21, 26)),    // Abyssking Lv26
            Zone::CrystalSpire => Some((22, 30)),  // Crystalion Lv30
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn has_heal(self) -> bool {
        matches!(self, Zone::PebbleTown | Zone::DeepCave | Zone::Frostpeak)
    }

    #[allow(dead_code)]
    fn has_shop(self) -> bool {
        matches!(self, Zone::PebbleTown | Zone::CoralShore | Zone::Frostpeak)
    }

    /// Min level for wild encounters (for level scaling)
    #[allow(dead_code)]
    fn min_wild_level(self) -> u8 {
        match self {
            Zone::PebbleTown => 1,
            Zone::VerdantPath => 2,
            Zone::EmberHollow => 8,
            Zone::CoralShore => 10,
            Zone::Sparkridge => 14,
            Zone::DeepCave => 16,
            Zone::ShadowVale => 20,
            Zone::CrystalSpire => 24,
            Zone::Frostpeak => 18,
        }
    }
}

// Zone layout on a 3x3 grid for the overworld
// Each zone is 30x45 tiles but we pack them into one big map
// Actually, we use a single zone at a time with transitions
const _ALL_ZONES: [Zone; 9] = [
    Zone::PebbleTown,
    Zone::VerdantPath,
    Zone::EmberHollow,
    Zone::CoralShore,
    Zone::Sparkridge,
    Zone::DeepCave,
    Zone::ShadowVale,
    Zone::CrystalSpire,
    Zone::Frostpeak,
];

// ═══════════════════════════════════════════════════════════════════════
// ITEM DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════

const ITEM_SPIRIT_ORB: u8 = 0;  // Catches monsters
const ITEM_POTION: u8 = 1;       // Heals 30 HP
const ITEM_SUPER_POTION: u8 = 2; // Heals 60 HP
const ITEM_REVIVE: u8 = 3;       // Revives fainted monster
const ITEM_ULTRA_ORB: u8 = 4;    // Better catch rate

fn item_name(id: u8) -> &'static str {
    match id {
        0 => "Spirit Orb",
        1 => "Potion",
        2 => "Super Potion",
        3 => "Revive",
        4 => "Ultra Orb",
        _ => "???",
    }
}

fn item_price(id: u8) -> u32 {
    match id {
        0 => 200,
        1 => 100,
        2 => 250,
        3 => 500,
        4 => 600,
        _ => 0,
    }
}

// ═══════════════════════════════════════════════════════════════════════
// STATE KEYS (stored in engine.global_state)
// ═══════════════════════════════════════════════════════════════════════

// We use string keys in global_state for all persistent game data.
// This approach works because the engine's global_state is a HashMap<String, f64>.

const K_MODE: &str = "mode";
const K_ZONE: &str = "zone";
const K_PLAYER_X: &str = "px";
const K_PLAYER_Y: &str = "py";
const K_PLAYER_TX: &str = "ptx"; // target x (movement)
const K_PLAYER_TY: &str = "pty"; // target y
const K_PLAYER_MOVING: &str = "pmoving";
const K_PLAYER_DIR: &str = "pdir"; // 0=down,1=up,2=left,3=right
const K_PLAYER_ANIM: &str = "panim";
const K_GOLD: &str = "gold";
const K_BADGES: &str = "badges"; // bitfield of defeated bosses
const K_STEP_COUNT: &str = "steps";

// Team monsters (up to 6): species, level, hp, maxhp, xp
// Keys: "t0_species", "t0_level", "t0_hp", "t0_maxhp", "t0_xp", "t0_atk", "t0_def", "t0_spd"
const K_TEAM_SIZE: &str = "team_size";

// Inventory: "inv_0" = count of item 0, etc.
// Battle state
const K_BPHASE: &str = "bphase";
const K_BATTLE_TIMER: &str = "btimer";
const K_ACTIVE_MON: &str = "active_mon"; // index in team
const K_ENEMY_SPECIES: &str = "enemy_species";
const K_ENEMY_LEVEL: &str = "enemy_level";
const K_ENEMY_HP: &str = "enemy_hp";
const K_ENEMY_MAXHP: &str = "enemy_maxhp";
const K_ENEMY_ATK: &str = "enemy_atk";
const K_ENEMY_DEF: &str = "enemy_def";
const K_IS_BOSS: &str = "is_boss";
const K_BALL_X: &str = "ball_x";
const K_BALL_Y: &str = "ball_y";
const K_BALL_VX: &str = "ball_vx";
const K_BALL_VY: &str = "ball_vy";
const K_BALL_ACTIVE: &str = "ball_active";
const K_AIM_X: &str = "aim_x";
const K_AIM_Y: &str = "aim_y";
const K_AIMING: &str = "aiming";
const K_AIM_START_X: &str = "aim_sx";
const K_AIM_START_Y: &str = "aim_sy";
const K_STROKES: &str = "strokes";
const K_DIST_TO_HOLE: &str = "dist_to_hole";
const K_BEST_DIST: &str = "best_dist";
const K_WALL_BOUNCES: &str = "wall_bounces";
const K_EFFECTIVENESS: &str = "effectiveness"; // last hit effectiveness
const K_BATTLE_MSG: &str = "battle_msg"; // 0=none, 1=super effective, 2=not effective
const K_MSG_TIMER: &str = "msg_timer";
const K_DMG_POPUP: &str = "dmg_popup"; // damage number to display
const K_DMG_POPUP_X: &str = "dmg_popup_x";
const K_DMG_POPUP_Y: &str = "dmg_popup_y";
const K_DMG_POPUP_TIMER: &str = "dmg_popup_timer";
const K_DMG_CRIT: &str = "dmg_crit"; // 1.0 if last hit was critical
const K_COMBO: &str = "combo"; // consecutive hits in a row
const K_ENEMY_SHAKE: &str = "enemy_shake"; // shake timer on hit

// Dialogue
const K_DLG_ID: &str = "dlg_id";
const K_DLG_LINE: &str = "dlg_line";
const K_DLG_TIMER: &str = "dlg_timer";

// Shop/Heal
const K_SHOP_SEL: &str = "shop_sel";
const _K_MENU_SEL: &str = "menu_sel";

// Transition
const K_TRANS_TIMER: &str = "trans_timer";
const K_TRANS_TARGET: &str = "trans_target"; // zone to transition to
const K_TRANS_MODE: &str = "trans_mode"; // mode to transition to

// Catch animation
const K_CATCH_TIMER: &str = "catch_timer";
const K_CATCH_SUCCESS: &str = "catch_success";

// Walk encounter cooldown
const K_ENCOUNTER_CD: &str = "encounter_cd";

// Title screen
const K_TITLE_TIMER: &str = "title_timer";
const K_TITLE_SEL: &str = "title_sel";

// ═══════════════════════════════════════════════════════════════════════
// HELPER: State access shortcuts
// ═══════════════════════════════════════════════════════════════════════

fn gs(engine: &Engine, key: &str) -> f64 {
    engine.global_state.get_f64(key).unwrap_or(0.0)
}

fn ss(engine: &mut Engine, key: &str, val: f64) {
    engine.global_state.set_f64(key, val);
}

fn gs_team(engine: &Engine, slot: usize, field: &str) -> f64 {
    let key = format!("t{}_{}", slot, field);
    engine.global_state.get_f64(&key).unwrap_or(0.0)
}

fn ss_team(engine: &mut Engine, slot: usize, field: &str, val: f64) {
    let key = format!("t{}_{}", slot, field);
    engine.global_state.set_f64(&key, val);
}

fn gs_inv(engine: &Engine, item: u8) -> f64 {
    let key = format!("inv_{}", item);
    engine.global_state.get_f64(&key).unwrap_or(0.0)
}

fn ss_inv(engine: &mut Engine, item: u8, val: f64) {
    let key = format!("inv_{}", item);
    engine.global_state.set_f64(&key, val);
}

/// Simple pseudo-random using engine time + frame
fn rng(engine: &Engine) -> f64 {
    let seed = engine.time * 1000.0 + engine.frame as f64 * 7.13;
    ((seed * 12345.6789).sin() * 43758.5453).fract().abs()
}

fn rng_range(engine: &Engine, min: f64, max: f64) -> f64 {
    min + rng(engine) * (max - min)
}

fn _rng_seeded(seed: f64) -> f64 {
    ((seed * 12345.6789).sin() * 43758.5453).fract().abs()
}

// ═══════════════════════════════════════════════════════════════════════
// MONSTER STAT CALCULATIONS
// ═══════════════════════════════════════════════════════════════════════

fn calc_max_hp(base: f64, level: f64) -> f64 {
    (base + level * 3.0 + 10.0).floor()
}

fn calc_atk(base: f64, level: f64) -> f64 {
    (base + level * 1.5).floor()
}

fn calc_def(base: f64, level: f64) -> f64 {
    (base + level * 1.2).floor()
}

fn calc_spd(base: f64, level: f64) -> f64 {
    (base + level * 0.8).floor()
}

fn xp_for_level(level: f64) -> f64 {
    (level * level * 25.0).floor()  // Steeper curve to slow progression
}

fn xp_reward(enemy_level: f64) -> f64 {
    (enemy_level * 6.0 + 8.0).floor()  // Slightly reduced to match steeper curve
}

/// Set team slot stats from species + level
fn set_team_monster(engine: &mut Engine, slot: usize, species_id: u8, level: u8) {
    let sp = get_species(species_id);
    let lv = level as f64;
    let maxhp = calc_max_hp(sp.base_hp, lv);
    ss_team(engine, slot, "species", species_id as f64);
    ss_team(engine, slot, "level", lv);
    ss_team(engine, slot, "hp", maxhp);
    ss_team(engine, slot, "maxhp", maxhp);
    ss_team(engine, slot, "xp", 0.0);
    ss_team(engine, slot, "atk", calc_atk(sp.base_atk, lv));
    ss_team(engine, slot, "def", calc_def(sp.base_def, lv));
    ss_team(engine, slot, "spd", calc_spd(sp.base_spd, lv));
}

fn level_up_monster(engine: &mut Engine, slot: usize) {
    let species_id = gs_team(engine, slot, "species") as u8;
    let level = gs_team(engine, slot, "level") + 1.0;
    let sp = get_species(species_id);
    let old_maxhp = gs_team(engine, slot, "maxhp");
    let new_maxhp = calc_max_hp(sp.base_hp, level);
    let hp_gain = new_maxhp - old_maxhp;
    let current_hp = gs_team(engine, slot, "hp");

    ss_team(engine, slot, "level", level);
    ss_team(engine, slot, "maxhp", new_maxhp);
    ss_team(engine, slot, "hp", (current_hp + hp_gain).min(new_maxhp));
    ss_team(engine, slot, "atk", calc_atk(sp.base_atk, level));
    ss_team(engine, slot, "def", calc_def(sp.base_def, level));
    ss_team(engine, slot, "spd", calc_spd(sp.base_spd, level));

    // Check evolution
    if let Some(evo_id) = sp.evolves_to {
        if level >= sp.evolve_level as f64 {
            let evo = get_species(evo_id);
            let evo_maxhp = calc_max_hp(evo.base_hp, level);
            let evo_hp = gs_team(engine, slot, "hp") + (evo_maxhp - new_maxhp).max(0.0);
            ss_team(engine, slot, "species", evo_id as f64);
            ss_team(engine, slot, "maxhp", evo_maxhp);
            ss_team(engine, slot, "hp", evo_hp.min(evo_maxhp));
            ss_team(engine, slot, "atk", calc_atk(evo.base_atk, level));
            ss_team(engine, slot, "def", calc_def(evo.base_def, level));
            ss_team(engine, slot, "spd", calc_spd(evo.base_spd, level));
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// MAP GENERATION
// ═══════════════════════════════════════════════════════════════════════

fn build_zone_map(zone: Zone) -> TileMap {
    let mut tm = TileMap::new(MAP_W, MAP_H, TILE_SIZE);

    match zone {
        Zone::PebbleTown => build_town_map(&mut tm),
        Zone::VerdantPath => build_verdant_path(&mut tm),
        Zone::EmberHollow => build_ember_hollow(&mut tm),
        Zone::CoralShore => build_coral_shore(&mut tm),
        Zone::Sparkridge => build_sparkridge(&mut tm),
        Zone::DeepCave => build_deep_cave(&mut tm),
        Zone::ShadowVale => build_shadow_vale(&mut tm),
        Zone::CrystalSpire => build_crystal_spire(&mut tm),
        Zone::Frostpeak => build_frostpeak(&mut tm),
    }

    tm
}

fn fill_rect_tiles(tm: &mut TileMap, x: usize, y: usize, w: usize, h: usize, tile: Tile) {
    for ty in y..(y + h).min(MAP_H) {
        for tx in x..(x + w).min(MAP_W) {
            tm.set(tx, ty, tile.clone());
        }
    }
}

fn build_town_map(tm: &mut TileMap) {
    // Fill with grass
    fill_rect_tiles(tm, 0, 0, MAP_W, MAP_H, Tile::custom(TILE_GRASS, COL_GRASS));

    // Town square path
    fill_rect_tiles(tm, 10, 15, 10, 15, Tile::custom(TILE_PATH, COL_PATH));

    // Main road (vertical)
    fill_rect_tiles(tm, 13, 0, 4, MAP_H, Tile::custom(TILE_PATH, COL_PATH));

    // Cross road (horizontal)
    fill_rect_tiles(tm, 0, 22, MAP_W, 3, Tile::custom(TILE_PATH, COL_PATH));

    // Heal center (top-left area)
    fill_rect_tiles(tm, 4, 16, 5, 4, Tile::custom(TILE_BUILDING, COL_BUILDING));
    tm.set(6, 19, Tile::custom(TILE_HEAL, COL_HEAL));

    // Shop (top-right area)
    fill_rect_tiles(tm, 21, 16, 5, 4, Tile::custom(TILE_BUILDING, COL_BUILDING));
    tm.set(23, 19, Tile::custom(TILE_SHOP, COL_SHOP_TILE));

    // Professor's house (top)
    fill_rect_tiles(tm, 12, 5, 6, 5, Tile::custom(TILE_BUILDING, COL_BUILDING));
    tm.set(14, 9, Tile::custom(TILE_DOOR, COL_DOOR));

    // Decorative trees
    for &(tx, ty) in &[(2, 10), (8, 12), (22, 10), (26, 12), (5, 30), (24, 32),
                        (1, 3), (28, 3), (1, 40), (28, 40)] {
        tm.set(tx, ty, Tile::custom(TILE_TREE, COL_TREE_TOP));
    }

    // Flowers
    for &(tx, ty) in &[(10, 12), (19, 12), (12, 28), (17, 28)] {
        tm.set(tx, ty, Tile::custom(TILE_FLOWER, COL_FLOWER));
    }

    // Water pond
    fill_rect_tiles(tm, 22, 28, 5, 4, Tile::custom(TILE_WATER, COL_WATER));

    // Walls (border)
    for x in 0..MAP_W {
        tm.set(x, 0, Tile::solid(COL_TREE_TOP));
        tm.set(x, MAP_H - 1, Tile::solid(COL_TREE_TOP));
    }
    for y in 0..MAP_H {
        tm.set(0, y, Tile::solid(COL_TREE_TOP));
        tm.set(MAP_W - 1, y, Tile::solid(COL_TREE_TOP));
    }

    // Exits: South leads to Verdant Path
    fill_rect_tiles(tm, 13, MAP_H - 1, 4, 1, Tile::custom(TILE_PATH, COL_PATH));
}

fn build_verdant_path(tm: &mut TileMap) {
    fill_rect_tiles(tm, 0, 0, MAP_W, MAP_H, Tile::custom(TILE_GRASS, COL_GRASS));

    // Main path winding down
    fill_rect_tiles(tm, 13, 0, 4, 12, Tile::custom(TILE_PATH, COL_PATH));
    fill_rect_tiles(tm, 8, 10, 12, 3, Tile::custom(TILE_PATH, COL_PATH));
    fill_rect_tiles(tm, 8, 12, 4, 10, Tile::custom(TILE_PATH, COL_PATH));
    fill_rect_tiles(tm, 8, 20, 12, 3, Tile::custom(TILE_PATH, COL_PATH));
    fill_rect_tiles(tm, 17, 22, 4, 10, Tile::custom(TILE_PATH, COL_PATH));
    fill_rect_tiles(tm, 10, 30, 11, 3, Tile::custom(TILE_PATH, COL_PATH));
    fill_rect_tiles(tm, 10, 32, 4, 10, Tile::custom(TILE_PATH, COL_PATH));
    fill_rect_tiles(tm, 13, MAP_H - 2, 4, 2, Tile::custom(TILE_PATH, COL_PATH));

    // Wild grass patches
    fill_rect_tiles(tm, 2, 4, 8, 6, Tile::custom(TILE_WILD, COL_WILD));
    fill_rect_tiles(tm, 20, 4, 8, 6, Tile::custom(TILE_WILD, COL_WILD));
    fill_rect_tiles(tm, 2, 14, 5, 6, Tile::custom(TILE_WILD, COL_WILD));
    fill_rect_tiles(tm, 22, 14, 6, 6, Tile::custom(TILE_WILD, COL_WILD));
    fill_rect_tiles(tm, 2, 34, 6, 6, Tile::custom(TILE_WILD, COL_WILD));
    fill_rect_tiles(tm, 22, 34, 6, 6, Tile::custom(TILE_WILD, COL_WILD));

    // Trees scattered
    for &(tx, ty) in &[(1, 1), (5, 12), (25, 12), (28, 1), (1, 25), (27, 25),
                        (3, 42), (26, 42), (15, 15), (14, 25)] {
        tm.set(tx, ty, Tile::custom(TILE_TREE, COL_TREE_TOP));
    }

    // Border
    for x in 0..MAP_W {
        tm.set(x, 0, Tile::solid(COL_TREE_TOP));
        tm.set(x, MAP_H - 1, Tile::solid(COL_TREE_TOP));
    }
    for y in 0..MAP_H {
        tm.set(0, y, Tile::solid(COL_TREE_TOP));
        tm.set(MAP_W - 1, y, Tile::solid(COL_TREE_TOP));
    }

    // North exit to Pebble Town
    fill_rect_tiles(tm, 13, 0, 4, 1, Tile::custom(TILE_PATH, COL_PATH));
    // South exit to Ember Hollow
    fill_rect_tiles(tm, 13, MAP_H - 1, 4, 1, Tile::custom(TILE_PATH, COL_PATH));
    // East exit to Coral Shore
    fill_rect_tiles(tm, MAP_W - 1, 21, 1, 3, Tile::custom(TILE_PATH, COL_PATH));
}

fn build_ember_hollow(tm: &mut TileMap) {
    fill_rect_tiles(tm, 0, 0, MAP_W, MAP_H, Tile::custom(TILE_SAND, COL_SAND));

    // Lava rocks (dark cave-like)
    fill_rect_tiles(tm, 3, 5, 6, 4, Tile::custom(TILE_CAVE, COL_CAVE));
    fill_rect_tiles(tm, 20, 8, 7, 5, Tile::custom(TILE_CAVE, COL_CAVE));
    fill_rect_tiles(tm, 10, 20, 10, 3, Tile::custom(TILE_CAVE, COL_CAVE));

    // Wild fire encounters
    fill_rect_tiles(tm, 4, 12, 8, 6, Tile::custom(TILE_WILD, Color { r: 180, g: 80, b: 40, a: 255 }));
    fill_rect_tiles(tm, 18, 16, 8, 6, Tile::custom(TILE_WILD, Color { r: 180, g: 80, b: 40, a: 255 }));
    fill_rect_tiles(tm, 8, 30, 14, 6, Tile::custom(TILE_WILD, Color { r: 180, g: 80, b: 40, a: 255 }));

    // Path
    fill_rect_tiles(tm, 13, 0, 4, MAP_H, Tile::custom(TILE_PATH, Color { r: 180, g: 160, b: 100, a: 255 }));

    // Boss platform
    fill_rect_tiles(tm, 11, 38, 8, 4, Tile::custom(TILE_BOSS, COL_BOSS_TILE));

    // Border
    for x in 0..MAP_W { tm.set(x, 0, Tile::solid(COL_CAVE)); tm.set(x, MAP_H - 1, Tile::solid(COL_CAVE)); }
    for y in 0..MAP_H { tm.set(0, y, Tile::solid(COL_CAVE)); tm.set(MAP_W - 1, y, Tile::solid(COL_CAVE)); }

    // North exit
    fill_rect_tiles(tm, 13, 0, 4, 1, Tile::custom(TILE_PATH, COL_PATH));
    // East exit to Deep Cave
    fill_rect_tiles(tm, MAP_W - 1, 21, 1, 3, Tile::custom(TILE_PATH, COL_PATH));
}

fn build_coral_shore(tm: &mut TileMap) {
    // Beach + water
    fill_rect_tiles(tm, 0, 0, MAP_W, 15, Tile::custom(TILE_WATER, COL_WATER));
    fill_rect_tiles(tm, 0, 15, MAP_W, MAP_H - 15, Tile::custom(TILE_SAND, COL_SAND));

    // Shore line
    fill_rect_tiles(tm, 0, 13, MAP_W, 4, Tile::custom(TILE_SAND, Color { r: 240, g: 220, b: 170, a: 255 }));

    // Path
    fill_rect_tiles(tm, 0, 21, MAP_W, 3, Tile::custom(TILE_PATH, COL_PATH));
    fill_rect_tiles(tm, 13, 15, 4, 30, Tile::custom(TILE_PATH, COL_PATH));

    // Wild areas
    fill_rect_tiles(tm, 3, 17, 8, 4, Tile::custom(TILE_WILD, Color { r: 40, g: 120, b: 180, a: 255 }));
    fill_rect_tiles(tm, 20, 17, 8, 4, Tile::custom(TILE_WILD, Color { r: 40, g: 120, b: 180, a: 255 }));
    fill_rect_tiles(tm, 5, 30, 8, 5, Tile::custom(TILE_WILD, Color { r: 40, g: 120, b: 180, a: 255 }));
    fill_rect_tiles(tm, 18, 30, 8, 5, Tile::custom(TILE_WILD, Color { r: 40, g: 120, b: 180, a: 255 }));

    // Shop
    fill_rect_tiles(tm, 3, 26, 5, 3, Tile::custom(TILE_BUILDING, COL_BUILDING));
    tm.set(5, 28, Tile::custom(TILE_SHOP, COL_SHOP_TILE));

    // Boss
    fill_rect_tiles(tm, 11, 38, 8, 4, Tile::custom(TILE_BOSS, COL_BOSS_TILE));

    // Border
    for x in 0..MAP_W { tm.set(x, 0, Tile::solid(COL_WATER)); tm.set(x, MAP_H - 1, Tile::solid(COL_BUILDING)); }
    for y in 0..MAP_H { tm.set(0, y, Tile::solid(COL_WATER)); tm.set(MAP_W - 1, y, Tile::solid(COL_BUILDING)); }

    // West exit
    fill_rect_tiles(tm, 0, 21, 1, 3, Tile::custom(TILE_PATH, COL_PATH));
    // South exit to Sparkridge
    fill_rect_tiles(tm, 13, MAP_H - 1, 4, 1, Tile::custom(TILE_PATH, COL_PATH));
}

fn build_sparkridge(tm: &mut TileMap) {
    fill_rect_tiles(tm, 0, 0, MAP_W, MAP_H, Tile::custom(TILE_GRASS, Color { r: 80, g: 100, b: 60, a: 255 }));

    // Rocky terrain
    fill_rect_tiles(tm, 2, 4, 4, 3, Tile::solid(Color { r: 120, g: 110, b: 100, a: 255 }));
    fill_rect_tiles(tm, 24, 8, 4, 4, Tile::solid(Color { r: 120, g: 110, b: 100, a: 255 }));
    fill_rect_tiles(tm, 5, 25, 3, 3, Tile::solid(Color { r: 120, g: 110, b: 100, a: 255 }));

    // Path
    fill_rect_tiles(tm, 13, 0, 4, MAP_H, Tile::custom(TILE_PATH, COL_PATH));
    fill_rect_tiles(tm, 5, 15, 20, 3, Tile::custom(TILE_PATH, COL_PATH));

    // Wild areas
    fill_rect_tiles(tm, 3, 8, 8, 6, Tile::custom(TILE_WILD, Color { r: 200, g: 180, b: 60, a: 255 }));
    fill_rect_tiles(tm, 19, 20, 8, 6, Tile::custom(TILE_WILD, Color { r: 200, g: 180, b: 60, a: 255 }));
    fill_rect_tiles(tm, 3, 30, 8, 5, Tile::custom(TILE_WILD, Color { r: 200, g: 180, b: 60, a: 255 }));

    // Boss
    fill_rect_tiles(tm, 11, 38, 8, 4, Tile::custom(TILE_BOSS, COL_BOSS_TILE));

    // Border
    for x in 0..MAP_W { tm.set(x, 0, Tile::solid(Color { r: 100, g: 90, b: 80, a: 255 })); tm.set(x, MAP_H - 1, Tile::solid(Color { r: 100, g: 90, b: 80, a: 255 })); }
    for y in 0..MAP_H { tm.set(0, y, Tile::solid(Color { r: 100, g: 90, b: 80, a: 255 })); tm.set(MAP_W - 1, y, Tile::solid(Color { r: 100, g: 90, b: 80, a: 255 })); }

    // North exit
    fill_rect_tiles(tm, 13, 0, 4, 1, Tile::custom(TILE_PATH, COL_PATH));
    // South to Shadow Vale
    fill_rect_tiles(tm, 13, MAP_H - 1, 4, 1, Tile::custom(TILE_PATH, COL_PATH));
    // East to Frostpeak
    fill_rect_tiles(tm, MAP_W - 1, 15, 1, 3, Tile::custom(TILE_PATH, COL_PATH));
}

fn build_deep_cave(tm: &mut TileMap) {
    fill_rect_tiles(tm, 0, 0, MAP_W, MAP_H, Tile::custom(TILE_CAVE, COL_CAVE));

    // Corridors
    fill_rect_tiles(tm, 5, 0, 4, 20, Tile::custom(TILE_PATH, Color { r: 120, g: 100, b: 80, a: 255 }));
    fill_rect_tiles(tm, 5, 18, 20, 4, Tile::custom(TILE_PATH, Color { r: 120, g: 100, b: 80, a: 255 }));
    fill_rect_tiles(tm, 22, 18, 4, 20, Tile::custom(TILE_PATH, Color { r: 120, g: 100, b: 80, a: 255 }));
    fill_rect_tiles(tm, 10, 36, 14, 4, Tile::custom(TILE_PATH, Color { r: 120, g: 100, b: 80, a: 255 }));

    // Wild areas (in caverns)
    fill_rect_tiles(tm, 10, 5, 8, 6, Tile::custom(TILE_WILD, Color { r: 100, g: 80, b: 60, a: 255 }));
    fill_rect_tiles(tm, 12, 26, 8, 6, Tile::custom(TILE_WILD, Color { r: 100, g: 80, b: 60, a: 255 }));

    // Crystals
    for &(tx, ty) in &[(3, 10), (15, 3), (26, 10), (8, 30), (25, 30)] {
        tm.set(tx, ty, Tile::custom(TILE_CRYSTAL, COL_CRYSTAL));
    }

    // Heal point
    tm.set(7, 10, Tile::custom(TILE_HEAL, COL_HEAL));

    // Border
    for x in 0..MAP_W { tm.set(x, 0, Tile::solid(COL_CAVE)); tm.set(x, MAP_H - 1, Tile::solid(COL_CAVE)); }
    for y in 0..MAP_H { tm.set(0, y, Tile::solid(COL_CAVE)); tm.set(MAP_W - 1, y, Tile::solid(COL_CAVE)); }

    // West exit
    fill_rect_tiles(tm, 0, 18, 1, 4, Tile::custom(TILE_PATH, COL_PATH));
}

fn build_shadow_vale(tm: &mut TileMap) {
    fill_rect_tiles(tm, 0, 0, MAP_W, MAP_H, Tile::custom(TILE_DARK, COL_DARK_TILE));

    // Eerie path
    fill_rect_tiles(tm, 13, 0, 4, MAP_H, Tile::custom(TILE_PATH, Color { r: 80, g: 60, b: 90, a: 255 }));
    fill_rect_tiles(tm, 5, 20, 20, 3, Tile::custom(TILE_PATH, Color { r: 80, g: 60, b: 90, a: 255 }));

    // Wild shadow areas
    fill_rect_tiles(tm, 2, 6, 8, 8, Tile::custom(TILE_WILD, Color { r: 60, g: 40, b: 80, a: 255 }));
    fill_rect_tiles(tm, 20, 6, 8, 8, Tile::custom(TILE_WILD, Color { r: 60, g: 40, b: 80, a: 255 }));
    fill_rect_tiles(tm, 3, 28, 8, 6, Tile::custom(TILE_WILD, Color { r: 60, g: 40, b: 80, a: 255 }));
    fill_rect_tiles(tm, 19, 28, 8, 6, Tile::custom(TILE_WILD, Color { r: 60, g: 40, b: 80, a: 255 }));

    // Boss
    fill_rect_tiles(tm, 11, 38, 8, 4, Tile::custom(TILE_BOSS, COL_BOSS_TILE));

    // Dead trees
    for &(tx, ty) in &[(4, 4), (25, 4), (2, 20), (27, 20), (10, 35), (20, 35)] {
        tm.set(tx, ty, Tile::custom(TILE_TREE, Color { r: 60, g: 40, b: 50, a: 255 }));
    }

    // Border
    for x in 0..MAP_W { tm.set(x, 0, Tile::solid(Color { r: 30, g: 20, b: 40, a: 255 })); tm.set(x, MAP_H - 1, Tile::solid(Color { r: 30, g: 20, b: 40, a: 255 })); }
    for y in 0..MAP_H { tm.set(0, y, Tile::solid(Color { r: 30, g: 20, b: 40, a: 255 })); tm.set(MAP_W - 1, y, Tile::solid(Color { r: 30, g: 20, b: 40, a: 255 })); }

    // North exit
    fill_rect_tiles(tm, 13, 0, 4, 1, Tile::custom(TILE_PATH, COL_PATH));
    // South exit to Crystal Spire
    fill_rect_tiles(tm, 13, MAP_H - 1, 4, 1, Tile::custom(TILE_PATH, COL_PATH));
}

fn build_crystal_spire(tm: &mut TileMap) {
    fill_rect_tiles(tm, 0, 0, MAP_W, MAP_H, Tile::custom(TILE_CRYSTAL, Color { r: 200, g: 210, b: 230, a: 255 }));

    // Crystal path
    fill_rect_tiles(tm, 13, 0, 4, MAP_H, Tile::custom(TILE_PATH, Color { r: 220, g: 220, b: 240, a: 255 }));
    fill_rect_tiles(tm, 5, 15, 20, 3, Tile::custom(TILE_PATH, Color { r: 220, g: 220, b: 240, a: 255 }));
    fill_rect_tiles(tm, 5, 30, 20, 3, Tile::custom(TILE_PATH, Color { r: 220, g: 220, b: 240, a: 255 }));

    // Wild areas
    fill_rect_tiles(tm, 2, 5, 8, 8, Tile::custom(TILE_WILD, Color { r: 180, g: 190, b: 220, a: 255 }));
    fill_rect_tiles(tm, 20, 5, 8, 8, Tile::custom(TILE_WILD, Color { r: 180, g: 190, b: 220, a: 255 }));
    fill_rect_tiles(tm, 3, 20, 8, 6, Tile::custom(TILE_WILD, Color { r: 180, g: 190, b: 220, a: 255 }));
    fill_rect_tiles(tm, 19, 20, 8, 6, Tile::custom(TILE_WILD, Color { r: 180, g: 190, b: 220, a: 255 }));

    // Boss at the top
    fill_rect_tiles(tm, 11, 38, 8, 4, Tile::custom(TILE_BOSS, COL_BOSS_TILE));

    // Crystals
    for &(tx, ty) in &[(4, 3), (25, 3), (2, 18), (27, 18), (8, 35), (22, 35)] {
        tm.set(tx, ty, Tile::custom(TILE_CRYSTAL, Color { r: 230, g: 240, b: 255, a: 255 }));
    }

    // Border
    for x in 0..MAP_W { tm.set(x, 0, Tile::solid(Color { r: 180, g: 190, b: 210, a: 255 })); tm.set(x, MAP_H - 1, Tile::solid(Color { r: 180, g: 190, b: 210, a: 255 })); }
    for y in 0..MAP_H { tm.set(0, y, Tile::solid(Color { r: 180, g: 190, b: 210, a: 255 })); tm.set(MAP_W - 1, y, Tile::solid(Color { r: 180, g: 190, b: 210, a: 255 })); }

    // North exit
    fill_rect_tiles(tm, 13, 0, 4, 1, Tile::custom(TILE_PATH, COL_PATH));
}

fn build_frostpeak(tm: &mut TileMap) {
    fill_rect_tiles(tm, 0, 0, MAP_W, MAP_H, Tile::custom(TILE_SNOW, COL_SNOW));

    // Icy paths
    fill_rect_tiles(tm, 13, 0, 4, MAP_H, Tile::custom(TILE_PATH, Color { r: 200, g: 210, b: 220, a: 255 }));
    fill_rect_tiles(tm, 5, 20, 20, 3, Tile::custom(TILE_PATH, Color { r: 200, g: 210, b: 220, a: 255 }));

    // Wild icy areas
    fill_rect_tiles(tm, 2, 5, 8, 8, Tile::custom(TILE_WILD, Color { r: 180, g: 210, b: 230, a: 255 }));
    fill_rect_tiles(tm, 20, 5, 8, 8, Tile::custom(TILE_WILD, Color { r: 180, g: 210, b: 230, a: 255 }));
    fill_rect_tiles(tm, 4, 30, 8, 6, Tile::custom(TILE_WILD, Color { r: 180, g: 210, b: 230, a: 255 }));
    fill_rect_tiles(tm, 18, 30, 8, 6, Tile::custom(TILE_WILD, Color { r: 180, g: 210, b: 230, a: 255 }));

    // Frozen trees
    for &(tx, ty) in &[(4, 3), (25, 3), (2, 18), (27, 18)] {
        tm.set(tx, ty, Tile::custom(TILE_TREE, Color { r: 150, g: 180, b: 200, a: 255 }));
    }

    // Heal point
    fill_rect_tiles(tm, 3, 20, 2, 2, Tile::custom(TILE_BUILDING, COL_BUILDING));
    tm.set(4, 21, Tile::custom(TILE_HEAL, COL_HEAL));

    // Shop
    fill_rect_tiles(tm, 24, 20, 4, 3, Tile::custom(TILE_BUILDING, COL_BUILDING));
    tm.set(26, 22, Tile::custom(TILE_SHOP, COL_SHOP_TILE));

    // Border
    for x in 0..MAP_W { tm.set(x, 0, Tile::solid(COL_SNOW)); tm.set(x, MAP_H - 1, Tile::solid(COL_SNOW)); }
    for y in 0..MAP_H { tm.set(0, y, Tile::solid(COL_SNOW)); tm.set(MAP_W - 1, y, Tile::solid(COL_SNOW)); }

    // West exit
    fill_rect_tiles(tm, 0, 20, 1, 3, Tile::custom(TILE_PATH, COL_PATH));
}

// ═══════════════════════════════════════════════════════════════════════
// BATTLE ARENA GENERATION
// ═══════════════════════════════════════════════════════════════════════

fn build_battle_arena(engine: &mut Engine, enemy_element: Element) {
    let mut tm = TileMap::new(MAP_W, MAP_H, TILE_SIZE);
    let arena_col = match enemy_element {
        Element::Fire => Color { r: 60, g: 30, b: 20, a: 255 },
        Element::Water => Color { r: 20, g: 40, b: 70, a: 255 },
        Element::Leaf => Color { r: 20, g: 50, b: 20, a: 255 },
        Element::Electric => Color { r: 50, g: 50, b: 20, a: 255 },
        Element::Earth => Color { r: 50, g: 40, b: 30, a: 255 },
        Element::Ice => Color { r: 40, g: 50, b: 60, a: 255 },
        Element::Shadow => Color { r: 25, g: 15, b: 35, a: 255 },
        Element::Light => Color { r: 55, g: 55, b: 45, a: 255 },
        _ => Color { r: 40, g: 40, b: 40, a: 255 },
    };
    let wall_col = Color { r: arena_col.r + 30, g: arena_col.g + 30, b: arena_col.b + 30, a: 255 };

    // Fill arena floor
    fill_rect_tiles(&mut tm, 0, 0, MAP_W, MAP_H, Tile::custom(TILE_ARENA, arena_col));

    // Walls around border
    for x in 0..MAP_W {
        tm.set(x, 0, Tile::solid(wall_col));
        tm.set(x, 1, Tile::solid(wall_col));
        tm.set(x, MAP_H - 1, Tile::solid(wall_col));
        tm.set(x, MAP_H - 2, Tile::solid(wall_col));
    }
    for y in 0..MAP_H {
        tm.set(0, y, Tile::solid(wall_col));
        tm.set(1, y, Tile::solid(wall_col));
        tm.set(MAP_W - 1, y, Tile::solid(wall_col));
        tm.set(MAP_W - 2, y, Tile::solid(wall_col));
    }

    // Target zone (where enemy monster stands - the "hole")
    fill_rect_tiles(&mut tm, 13, 5, 4, 4, Tile::custom(TILE_HOLE, enemy_element.color()));

    // Bumpers (obstacles based on element)
    let bumper_col = Color { r: wall_col.r + 20, g: wall_col.g + 20, b: wall_col.b + 20, a: 255 };
    match enemy_element {
        Element::Fire | Element::Earth => {
            // Scattered rocks
            for &(bx, by) in &[(6, 15), (23, 15), (10, 25), (19, 25), (8, 10), (21, 10)] {
                fill_rect_tiles(&mut tm, bx, by, 2, 2, Tile::custom(TILE_BUMPER, bumper_col));
            }
        }
        Element::Water | Element::Ice => {
            // Pools
            fill_rect_tiles(&mut tm, 5, 12, 3, 3, Tile::custom(TILE_BUMPER, COL_WATER));
            fill_rect_tiles(&mut tm, 22, 12, 3, 3, Tile::custom(TILE_BUMPER, COL_WATER));
            fill_rect_tiles(&mut tm, 12, 20, 6, 2, Tile::custom(TILE_BUMPER, COL_WATER));
        }
        _ => {
            // Default layout
            fill_rect_tiles(&mut tm, 7, 12, 2, 2, Tile::custom(TILE_BUMPER, bumper_col));
            fill_rect_tiles(&mut tm, 21, 12, 2, 2, Tile::custom(TILE_BUMPER, bumper_col));
            fill_rect_tiles(&mut tm, 14, 22, 2, 2, Tile::custom(TILE_BUMPER, bumper_col));
        }
    }

    engine.tilemap = Some(tm);
}

// ═══════════════════════════════════════════════════════════════════════
// DIALOGUE DATA
// ═══════════════════════════════════════════════════════════════════════

fn get_dialogue(id: u32) -> &'static [&'static str] {
    match id {
        // Professor intro (first time)
        0 => &[
            "Welcome to the world of",
            "Spirit Creatures!",
            "I'm Professor Oak-...",
            "Er, Professor Pebble.",
            "Take this Sproutail.",
            "Explore the world and",
            "collect Spirit Creatures!",
            "Defeat the 5 Guardians",
            "to become Champion!",
        ],
        // Heal center
        1 => &[
            "Welcome to the Spirit",
            "Center! Your creatures",
            "are fully healed!",
        ],
        // Shop greeting
        2 => &[
            "Welcome to the shop!",
            "What would you like?",
        ],
        // Boss defeated
        3 => &[
            "The Guardian has fallen!",
            "You earned a Badge!",
            "New paths are open.",
        ],
        // Wild area hint
        4 => &[
            "Tall grass ahead!",
            "Wild creatures lurk",
            "in these areas.",
        ],
        // Victory
        5 => &[
            "You defeated all five",
            "Guardians! You are the",
            "S-League Champion!",
            "Congratulations!",
        ],
        // Generic NPC
        6 => &[
            "The Spirit Creatures",
            "grow stronger when you",
            "battle with them!",
        ],
        // Zone-specific hints
        7 => &[
            "The tall grass hides",
            "many creatures. Walk",
            "through it to find them!",
            "Each zone has unique",
            "species to collect.",
        ],
        8 => &[
            "Fire creatures dwell in",
            "Ember Hollow. They fear",
            "Water-type attacks.",
            "Type advantages deal",
            "much more damage!",
        ],
        9 => &[
            "I heard there's a rare",
            "creature deep in the",
            "caves. It only appears",
            "to strong trainers!",
        ],
        10 => &[
            "The Guardian of each",
            "zone guards a Badge.",
            "You can't flee from a",
            "Guardian battle!",
            "Come prepared!",
        ],
        11 => &[
            "Spirit Orbs are used to",
            "catch wild creatures.",
            "Ultra Orbs have a much",
            "higher catch rate!",
        ],
        12 => &[
            "The Crystal Spire is",
            "the final challenge.",
            "Only those with all",
            "five Badges may enter.",
        ],
        13 => &[
            "When your creatures",
            "faint, visit a Spirit",
            "Center to heal them.",
            "Potions can also help!",
        ],
        14 => &[
            "Hit the enemy with",
            "speed! Faster impacts",
            "deal more damage.",
            "Aim for critical hits!",
        ],
        15 => &[
            "Your Sproutail will",
            "evolve into Thornvine",
            "at level 16! Keep",
            "battling to grow!",
        ],
        _ => &["..."],
    }
}

// ═══════════════════════════════════════════════════════════════════════
// ZONE TRANSITIONS & CONNECTIONS
// ═══════════════════════════════════════════════════════════════════════

fn zone_from_f64(v: f64) -> Zone {
    match v as u32 {
        0 => Zone::PebbleTown,
        1 => Zone::VerdantPath,
        2 => Zone::EmberHollow,
        3 => Zone::CoralShore,
        4 => Zone::Sparkridge,
        5 => Zone::DeepCave,
        6 => Zone::ShadowVale,
        7 => Zone::CrystalSpire,
        8 => Zone::Frostpeak,
        _ => Zone::PebbleTown,
    }
}

fn zone_to_f64(z: Zone) -> f64 {
    match z {
        Zone::PebbleTown => 0.0,
        Zone::VerdantPath => 1.0,
        Zone::EmberHollow => 2.0,
        Zone::CoralShore => 3.0,
        Zone::Sparkridge => 4.0,
        Zone::DeepCave => 5.0,
        Zone::ShadowVale => 6.0,
        Zone::CrystalSpire => 7.0,
        Zone::Frostpeak => 8.0,
    }
}

/// Check edges of zone to see if player walks to exit. Returns (target_zone, spawn_x, spawn_y).
fn check_zone_exit(zone: Zone, px: f64, py: f64, badges: u32) -> Option<(Zone, f64, f64)> {
    let tx = (px / TILE_SIZE) as usize;
    let ty = (py / TILE_SIZE) as usize;

    match zone {
        Zone::PebbleTown => {
            if ty >= MAP_H - 2 && tx >= 13 && tx <= 16 {
                return Some((Zone::VerdantPath, 15.0 * TILE_SIZE, 2.0 * TILE_SIZE));
            }
        }
        Zone::VerdantPath => {
            if ty <= 1 && tx >= 13 && tx <= 16 {
                return Some((Zone::PebbleTown, 15.0 * TILE_SIZE, (MAP_H as f64 - 3.0) * TILE_SIZE));
            }
            if ty >= MAP_H - 2 && tx >= 13 && tx <= 16 {
                return Some((Zone::EmberHollow, 15.0 * TILE_SIZE, 2.0 * TILE_SIZE));
            }
            if tx >= MAP_W - 2 && ty >= 21 && ty <= 23 {
                return Some((Zone::CoralShore, 2.0 * TILE_SIZE, 22.0 * TILE_SIZE));
            }
        }
        Zone::EmberHollow => {
            if ty <= 1 && tx >= 13 && tx <= 16 {
                return Some((Zone::VerdantPath, 15.0 * TILE_SIZE, (MAP_H as f64 - 3.0) * TILE_SIZE));
            }
            if tx >= MAP_W - 2 && ty >= 21 && ty <= 23 {
                if badges >= 1 { // Need Ember badge
                    return Some((Zone::DeepCave, 2.0 * TILE_SIZE, 20.0 * TILE_SIZE));
                }
            }
        }
        Zone::CoralShore => {
            if tx <= 1 && ty >= 21 && ty <= 23 {
                return Some((Zone::VerdantPath, (MAP_W as f64 - 3.0) * TILE_SIZE, 22.0 * TILE_SIZE));
            }
            if ty >= MAP_H - 2 && tx >= 13 && tx <= 16 {
                if badges >= 2 { // Need Coral badge
                    return Some((Zone::Sparkridge, 15.0 * TILE_SIZE, 2.0 * TILE_SIZE));
                }
            }
        }
        Zone::Sparkridge => {
            if ty <= 1 && tx >= 13 && tx <= 16 {
                return Some((Zone::CoralShore, 15.0 * TILE_SIZE, (MAP_H as f64 - 3.0) * TILE_SIZE));
            }
            if ty >= MAP_H - 2 && tx >= 13 && tx <= 16 {
                if badges >= 3 {
                    return Some((Zone::ShadowVale, 15.0 * TILE_SIZE, 2.0 * TILE_SIZE));
                }
            }
            if tx >= MAP_W - 2 && ty >= 15 && ty <= 17 {
                return Some((Zone::Frostpeak, 2.0 * TILE_SIZE, 21.0 * TILE_SIZE));
            }
        }
        Zone::DeepCave => {
            if tx <= 1 && ty >= 18 && ty <= 21 {
                return Some((Zone::EmberHollow, (MAP_W as f64 - 3.0) * TILE_SIZE, 22.0 * TILE_SIZE));
            }
        }
        Zone::ShadowVale => {
            if ty <= 1 && tx >= 13 && tx <= 16 {
                return Some((Zone::Sparkridge, 15.0 * TILE_SIZE, (MAP_H as f64 - 3.0) * TILE_SIZE));
            }
            if ty >= MAP_H - 2 && tx >= 13 && tx <= 16 {
                if badges >= 4 {
                    return Some((Zone::CrystalSpire, 15.0 * TILE_SIZE, 2.0 * TILE_SIZE));
                }
            }
        }
        Zone::CrystalSpire => {
            if ty <= 1 && tx >= 13 && tx <= 16 {
                return Some((Zone::ShadowVale, 15.0 * TILE_SIZE, (MAP_H as f64 - 3.0) * TILE_SIZE));
            }
        }
        Zone::Frostpeak => {
            if tx <= 1 && ty >= 20 && ty <= 22 {
                return Some((Zone::Sparkridge, (MAP_W as f64 - 3.0) * TILE_SIZE, 16.0 * TILE_SIZE));
            }
        }
    }
    None
}

// ═══════════════════════════════════════════════════════════════════════
// SETUP
// ═══════════════════════════════════════════════════════════════════════

pub fn setup(engine: &mut Engine) {
    // Start on title screen
    ss(engine, K_MODE, MODE_TITLE);
    ss(engine, K_TITLE_TIMER, 0.0);
    ss(engine, K_TITLE_SEL, 0.0);

    // Initialize player
    ss(engine, K_ZONE, zone_to_f64(Zone::PebbleTown));
    ss(engine, K_PLAYER_X, 15.0 * TILE_SIZE);
    ss(engine, K_PLAYER_Y, 24.0 * TILE_SIZE);
    ss(engine, K_PLAYER_DIR, 0.0);
    ss(engine, K_GOLD, 500.0);
    ss(engine, K_BADGES, 0.0);
    ss(engine, K_STEP_COUNT, 0.0);

    // Give starter monster: Sproutail Lv5
    ss(engine, K_TEAM_SIZE, 1.0);
    set_team_monster(engine, 0, 0, 5);

    // Starting inventory
    ss_inv(engine, ITEM_SPIRIT_ORB, 5.0);
    ss_inv(engine, ITEM_POTION, 3.0);

    // Build initial map
    engine.tilemap = Some(build_zone_map(Zone::PebbleTown));

    // Play intro dialogue
    ss(engine, K_MODE, MODE_DIALOGUE);
    ss(engine, K_DLG_ID, 0.0);
    ss(engine, K_DLG_LINE, 0.0);
    ss(engine, K_DLG_TIMER, 0.0);

    // Title music
    engine.sound_queue.push(SoundCommand::StartLoop {
        id: "bgm".to_string(),
        frequency: 220.0,
        volume: 0.08,
        waveform: Waveform::Triangle,
    });
}

pub fn setup_fight_only(engine: &mut Engine) {
    // Quick setup for headless battle testing
    ss(engine, K_MODE, MODE_BATTLE);
    ss(engine, K_BPHASE, BPHASE_PLAYER_AIM);
    ss(engine, K_TEAM_SIZE, 1.0);
    set_team_monster(engine, 0, 0, 10);
    ss(engine, K_ACTIVE_MON, 0.0);

    // Spawn a test enemy
    let sp = get_species(2);
    let lv = 8.0;
    ss(engine, K_ENEMY_SPECIES, 2.0);
    ss(engine, K_ENEMY_LEVEL, lv);
    ss(engine, K_ENEMY_HP, calc_max_hp(sp.base_hp, lv));
    ss(engine, K_ENEMY_MAXHP, calc_max_hp(sp.base_hp, lv));
    ss(engine, K_ENEMY_ATK, calc_atk(sp.base_atk, lv));
    ss(engine, K_ENEMY_DEF, calc_def(sp.base_def, lv));
    ss(engine, K_IS_BOSS, 0.0);

    // Ball start position
    ss(engine, K_BALL_X, 15.0 * TILE_SIZE);
    ss(engine, K_BALL_Y, 35.0 * TILE_SIZE);
    ss(engine, K_BALL_ACTIVE, 0.0);
    ss(engine, K_STROKES, 0.0);

    // Distance tracking for headless tests
    let target_cx = 15.0 * TILE_SIZE;
    let target_cy = 7.0 * TILE_SIZE;
    let dx = 15.0 * TILE_SIZE - target_cx;
    let dy = 35.0 * TILE_SIZE - target_cy;
    let initial_dist = (dx * dx + dy * dy).sqrt();
    ss(engine, K_DIST_TO_HOLE, initial_dist);
    ss(engine, K_BEST_DIST, initial_dist);
    ss(engine, K_WALL_BOUNCES, 0.0);

    build_battle_arena(engine, sp.element);
}

// ═══════════════════════════════════════════════════════════════════════
// SOUND HELPERS
// ═══════════════════════════════════════════════════════════════════════

fn play_hit_sound(engine: &mut Engine) {
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 200.0, duration: 0.12, volume: 0.4,
        waveform: Waveform::Square, attack: 0.005, decay: 0.1,
    });
    engine.sound_queue.push(SoundCommand::PlayNoise {
        duration: 0.08, volume: 0.25, filter_freq: 2000.0,
    });
}

fn play_element_hit_sound(engine: &mut Engine, elem: Element, is_crit: bool) {
    let base_vol = if is_crit { 0.5 } else { 0.35 };
    match elem {
        Element::Fire => {
            engine.sound_queue.push(SoundCommand::PlayTone {
                frequency: 180.0, duration: 0.15, volume: base_vol,
                waveform: Waveform::Sawtooth, attack: 0.005, decay: 0.12,
            });
            engine.sound_queue.push(SoundCommand::PlayNoise {
                duration: 0.1, volume: 0.3, filter_freq: 3000.0,
            });
        }
        Element::Water => {
            engine.sound_queue.push(SoundCommand::PlayTone {
                frequency: 300.0, duration: 0.2, volume: base_vol,
                waveform: Waveform::Sine, attack: 0.01, decay: 0.15,
            });
            engine.sound_queue.push(SoundCommand::PlayTone {
                frequency: 450.0, duration: 0.1, volume: base_vol * 0.5,
                waveform: Waveform::Sine, attack: 0.05, decay: 0.05,
            });
        }
        Element::Electric => {
            engine.sound_queue.push(SoundCommand::PlayTone {
                frequency: 800.0, duration: 0.08, volume: base_vol,
                waveform: Waveform::Square, attack: 0.002, decay: 0.06,
            });
            engine.sound_queue.push(SoundCommand::PlayNoise {
                duration: 0.05, volume: 0.4, filter_freq: 6000.0,
            });
        }
        Element::Ice => {
            engine.sound_queue.push(SoundCommand::PlayTone {
                frequency: 600.0, duration: 0.15, volume: base_vol,
                waveform: Waveform::Triangle, attack: 0.005, decay: 0.12,
            });
            engine.sound_queue.push(SoundCommand::PlayTone {
                frequency: 900.0, duration: 0.1, volume: base_vol * 0.4,
                waveform: Waveform::Sine, attack: 0.03, decay: 0.07,
            });
        }
        Element::Shadow => {
            engine.sound_queue.push(SoundCommand::PlayTone {
                frequency: 80.0, duration: 0.2, volume: base_vol,
                waveform: Waveform::Sawtooth, attack: 0.01, decay: 0.18,
            });
            engine.sound_queue.push(SoundCommand::PlayNoise {
                duration: 0.15, volume: 0.2, filter_freq: 800.0,
            });
        }
        Element::Light => {
            engine.sound_queue.push(SoundCommand::PlayTone {
                frequency: 1047.0, duration: 0.12, volume: base_vol,
                waveform: Waveform::Sine, attack: 0.005, decay: 0.1,
            });
            engine.sound_queue.push(SoundCommand::PlayTone {
                frequency: 1319.0, duration: 0.1, volume: base_vol * 0.6,
                waveform: Waveform::Sine, attack: 0.02, decay: 0.08,
            });
        }
        _ => {
            play_hit_sound(engine);
        }
    }
    if is_crit {
        engine.sound_queue.push(SoundCommand::PlayTone {
            frequency: 523.0, duration: 0.1, volume: 0.3,
            waveform: Waveform::Triangle, attack: 0.005, decay: 0.08,
        });
    }
}

fn play_super_effective_sound(engine: &mut Engine) {
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 880.0, duration: 0.15, volume: 0.35,
        waveform: Waveform::Sine, attack: 0.01, decay: 0.12,
    });
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 1320.0, duration: 0.15, volume: 0.25,
        waveform: Waveform::Sine, attack: 0.05, decay: 0.1,
    });
}

fn play_catch_sound(engine: &mut Engine) {
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 440.0, duration: 0.2, volume: 0.3,
        waveform: Waveform::Triangle, attack: 0.01, decay: 0.15,
    });
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 660.0, duration: 0.2, volume: 0.25,
        waveform: Waveform::Sine, attack: 0.1, decay: 0.1,
    });
}

fn play_level_up_sound(engine: &mut Engine) {
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 523.0, duration: 0.15, volume: 0.3,
        waveform: Waveform::Triangle, attack: 0.01, decay: 0.12,
    });
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 659.0, duration: 0.15, volume: 0.3,
        waveform: Waveform::Triangle, attack: 0.08, decay: 0.07,
    });
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 784.0, duration: 0.2, volume: 0.35,
        waveform: Waveform::Triangle, attack: 0.15, decay: 0.05,
    });
}

fn play_encounter_sound(engine: &mut Engine) {
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 330.0, duration: 0.15, volume: 0.4,
        waveform: Waveform::Square, attack: 0.005, decay: 0.12,
    });
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 440.0, duration: 0.15, volume: 0.35,
        waveform: Waveform::Square, attack: 0.08, decay: 0.07,
    });
}

fn play_ui_click(engine: &mut Engine) {
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 600.0, duration: 0.05, volume: 0.25,
        waveform: Waveform::Triangle, attack: 0.002, decay: 0.04,
    });
}

fn play_heal_sound(engine: &mut Engine) {
    for i in 0..3 {
        engine.sound_queue.push(SoundCommand::PlayTone {
            frequency: 440.0 + i as f64 * 220.0, duration: 0.2, volume: 0.25,
            waveform: Waveform::Sine, attack: 0.02 + i as f64 * 0.06, decay: 0.1,
        });
    }
}

fn play_boss_intro(engine: &mut Engine) {
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 110.0, duration: 0.4, volume: 0.5,
        waveform: Waveform::Sawtooth, attack: 0.01, decay: 0.35,
    });
    engine.sound_queue.push(SoundCommand::PlayNoise {
        duration: 0.3, volume: 0.3, filter_freq: 500.0,
    });
}

fn play_victory_sound(engine: &mut Engine) {
    let notes = [523.0, 659.0, 784.0, 1047.0];
    for (i, &freq) in notes.iter().enumerate() {
        engine.sound_queue.push(SoundCommand::PlayTone {
            frequency: freq, duration: 0.25, volume: 0.3,
            waveform: Waveform::Triangle, attack: 0.01 + i as f64 * 0.05, decay: 0.15,
        });
    }
}

// ═══════════════════════════════════════════════════════════════════════
// INPUT HANDLERS
// ═══════════════════════════════════════════════════════════════════════

pub fn on_pointer_down(engine: &mut Engine, x: f64, y: f64) {
    let mode = gs(engine, K_MODE);

    if mode == MODE_TITLE {
        // Check button areas
        if y > 400.0 && y < 450.0 && x > 140.0 && x < 340.0 {
            // New Game
            play_ui_click(engine);
            start_new_game(engine);
        }
        return;
    }

    if mode == MODE_DIALOGUE {
        advance_dialogue(engine);
        return;
    }

    if mode == MODE_OVERWORLD {
        // Tap to set movement target
        let cam_x = gs(engine, K_PLAYER_X) - WIDTH / 2.0;
        let cam_y = gs(engine, K_PLAYER_Y) - HEIGHT / 2.0;
        let world_x = x + cam_x;
        let world_y = y + cam_y;
        ss(engine, K_PLAYER_TX, world_x);
        ss(engine, K_PLAYER_TY, world_y);
        ss(engine, K_PLAYER_MOVING, 1.0);
        return;
    }

    if mode == MODE_BATTLE {
        let bphase = gs(engine, K_BPHASE);
        if bphase == BPHASE_PLAYER_AIM && gs(engine, K_BALL_ACTIVE) == 0.0 {
            // Check if tapping battle menu buttons (bottom HUD)
            if y > HEIGHT - 120.0 {
                // Menu buttons
                if x < WIDTH / 3.0 {
                    // Attack (start aiming)
                    ss(engine, K_AIMING, 1.0);
                    ss(engine, K_AIM_START_X, x);
                    ss(engine, K_AIM_START_Y, y);
                    ss(engine, K_AIM_X, x);
                    ss(engine, K_AIM_Y, y);
                } else if x < 2.0 * WIDTH / 3.0 {
                    // Catch
                    try_catch(engine);
                } else {
                    // Items / Run
                    try_run(engine);
                }
                return;
            }
            // Start aiming in the arena area
            ss(engine, K_AIMING, 1.0);
            ss(engine, K_AIM_START_X, x);
            ss(engine, K_AIM_START_Y, y);
            ss(engine, K_AIM_X, x);
            ss(engine, K_AIM_Y, y);
        }
        return;
    }

    if mode == MODE_BATTLE_RESULT {
        // Tap to dismiss
        let timer = gs(engine, K_BATTLE_TIMER);
        if timer > 1.0 {
            end_battle(engine, true);
        }
        return;
    }

    if mode == MODE_SHOP {
        handle_shop_tap(engine, x, y);
        return;
    }

    if mode == MODE_HEAL {
        // Tap to dismiss heal
        ss(engine, K_MODE, MODE_OVERWORLD);
        return;
    }

    if mode == MODE_MENU {
        handle_menu_tap(engine, x, y);
        return;
    }

    if mode == MODE_CATCH_ANIM {
        // Wait for animation
        return;
    }
}

pub fn on_pointer_move(engine: &mut Engine, x: f64, y: f64) {
    let mode = gs(engine, K_MODE);
    if mode == MODE_BATTLE && gs(engine, K_AIMING) == 1.0 {
        ss(engine, K_AIM_X, x);
        ss(engine, K_AIM_Y, y);
    }
    if mode == MODE_OVERWORLD && gs(engine, K_PLAYER_MOVING) == 1.0 {
        let cam_x = gs(engine, K_PLAYER_X) - WIDTH / 2.0;
        let cam_y = gs(engine, K_PLAYER_Y) - HEIGHT / 2.0;
        ss(engine, K_PLAYER_TX, x + cam_x);
        ss(engine, K_PLAYER_TY, y + cam_y);
    }
}

pub fn on_pointer_up(engine: &mut Engine, x: f64, y: f64) {
    let mode = gs(engine, K_MODE);

    if mode == MODE_BATTLE && gs(engine, K_AIMING) == 1.0 {
        ss(engine, K_AIMING, 0.0);
        // Fire the energy orb
        let sx = gs(engine, K_AIM_START_X);
        let sy = gs(engine, K_AIM_START_Y);
        let dx = sx - x;
        let dy = sy - y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist > 15.0 {
            let power = (dist * 12.0).min(1800.0);
            let angle = dy.atan2(dx);
            let vx = angle.cos() * power;
            let vy = angle.sin() * power;
            ss(engine, K_BALL_VX, vx);
            ss(engine, K_BALL_VY, vy);
            ss(engine, K_BALL_ACTIVE, 1.0);
            ss(engine, K_BPHASE, BPHASE_PLAYER_SHOT);
            let strokes = gs(engine, K_STROKES);
            ss(engine, K_STROKES, strokes + 1.0);

            // Shot sound
            engine.sound_queue.push(SoundCommand::PlayTone {
                frequency: 300.0 + power * 0.5, duration: 0.1, volume: 0.3,
                waveform: Waveform::Sine, attack: 0.005, decay: 0.08,
            });
        }
    }

    if mode == MODE_OVERWORLD {
        ss(engine, K_PLAYER_MOVING, 0.0);
    }
}

fn start_new_game(engine: &mut Engine) {
    ss(engine, K_MODE, MODE_DIALOGUE);
    ss(engine, K_DLG_ID, 0.0);
    ss(engine, K_DLG_LINE, 0.0);
    ss(engine, K_DLG_TIMER, 0.0);
}

fn advance_dialogue(engine: &mut Engine) {
    play_ui_click(engine);
    let id = gs(engine, K_DLG_ID) as u32;
    let line = gs(engine, K_DLG_LINE) as usize;
    let lines = get_dialogue(id);
    if line + 1 < lines.len() {
        ss(engine, K_DLG_LINE, (line + 1) as f64);
    } else {
        // Dialogue finished
        ss(engine, K_MODE, MODE_OVERWORLD);
    }
}

fn try_catch(engine: &mut Engine) {
    let orbs = gs_inv(engine, ITEM_SPIRIT_ORB);
    let ultra = gs_inv(engine, ITEM_ULTRA_ORB);
    if orbs <= 0.0 && ultra <= 0.0 {
        return; // No orbs
    }

    if gs(engine, K_IS_BOSS) == 1.0 {
        // Can't catch bosses
        ss(engine, K_BATTLE_MSG, 3.0); // "Can't catch!"
        ss(engine, K_MSG_TIMER, 1.5);
        return;
    }

    // Use ultra orb first if available
    let catch_bonus = if ultra > 0.0 {
        ss_inv(engine, ITEM_ULTRA_ORB, ultra - 1.0);
        0.3
    } else {
        ss_inv(engine, ITEM_SPIRIT_ORB, orbs - 1.0);
        0.0
    };

    let species_id = gs(engine, K_ENEMY_SPECIES) as u8;
    let sp = get_species(species_id);
    let hp_ratio = gs(engine, K_ENEMY_HP) / gs(engine, K_ENEMY_MAXHP);
    let catch_chance = (sp.catch_rate + catch_bonus) * (1.5 - hp_ratio);

    play_catch_sound(engine);
    ss(engine, K_MODE, MODE_CATCH_ANIM);
    ss(engine, K_CATCH_TIMER, 0.0);

    let roll = rng(engine);
    ss(engine, K_CATCH_SUCCESS, if roll < catch_chance { 1.0 } else { 0.0 });
}

fn try_run(engine: &mut Engine) {
    if gs(engine, K_IS_BOSS) == 1.0 {
        ss(engine, K_BATTLE_MSG, 4.0); // "Can't run from guardian!"
        ss(engine, K_MSG_TIMER, 1.5);
        return;
    }
    // Run away
    play_ui_click(engine);
    end_battle(engine, false);
}

fn handle_shop_tap(engine: &mut Engine, _x: f64, y: f64) {
    let gold = gs(engine, K_GOLD) as u32;

    // Shop layout: items listed vertically
    let items = [ITEM_SPIRIT_ORB, ITEM_POTION, ITEM_SUPER_POTION, ITEM_REVIVE, ITEM_ULTRA_ORB];
    let start_y = 200.0;
    let item_h = 60.0;

    for (i, &item) in items.iter().enumerate() {
        let iy = start_y + i as f64 * item_h;
        if y >= iy && y < iy + item_h {
            let price = item_price(item);
            if gold >= price {
                ss(engine, K_GOLD, (gold - price) as f64);
                let count = gs_inv(engine, item);
                ss_inv(engine, item, count + 1.0);
                play_ui_click(engine);
            }
            return;
        }
    }

    // Exit button at bottom
    if y > HEIGHT - 80.0 {
        play_ui_click(engine);
        ss(engine, K_MODE, MODE_OVERWORLD);
    }
}

fn handle_menu_tap(engine: &mut Engine, _x: f64, y: f64) {
    // Menu: team info, items, back
    if y > HEIGHT - 80.0 {
        play_ui_click(engine);
        ss(engine, K_MODE, MODE_OVERWORLD);
    }
    // Use potion on first injured monster
    if y > 300.0 && y < 360.0 {
        use_potion(engine);
    }
}

fn use_potion(engine: &mut Engine) {
    let potions = gs_inv(engine, ITEM_POTION);
    let supers = gs_inv(engine, ITEM_SUPER_POTION);
    if potions <= 0.0 && supers <= 0.0 { return; }

    let team_size = gs(engine, K_TEAM_SIZE) as usize;
    for i in 0..team_size {
        let hp = gs_team(engine, i, "hp");
        let maxhp = gs_team(engine, i, "maxhp");
        if hp > 0.0 && hp < maxhp {
            let (heal_amount, use_super) = if supers > 0.0 { (60.0, true) } else { (30.0, false) };
            ss_team(engine, i, "hp", (hp + heal_amount).min(maxhp));
            if use_super {
                ss_inv(engine, ITEM_SUPER_POTION, supers - 1.0);
            } else {
                ss_inv(engine, ITEM_POTION, potions - 1.0);
            }
            play_heal_sound(engine);
            return;
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// UPDATE LOOP
// ═══════════════════════════════════════════════════════════════════════

pub fn update(engine: &mut Engine, dt: f64) {
    // Normalize dt to seconds: headless runner passes seconds (~0.017),
    // browser JS passes milliseconds (~16.7) from performance.now() deltas.
    let dt = if dt > 1.0 { dt / 1000.0 } else { dt };

    let mode = gs(engine, K_MODE);

    if mode == MODE_TITLE {
        let t = gs(engine, K_TITLE_TIMER) + dt;
        ss(engine, K_TITLE_TIMER, t);
        return;
    }

    if mode == MODE_DIALOGUE {
        let t = gs(engine, K_DLG_TIMER) + dt;
        ss(engine, K_DLG_TIMER, t);
        return;
    }

    if mode == MODE_OVERWORLD {
        update_overworld(engine, dt);
        return;
    }

    if mode == MODE_BATTLE {
        update_battle(engine, dt);
        return;
    }

    if mode == MODE_BATTLE_RESULT {
        let t = gs(engine, K_BATTLE_TIMER) + dt;
        ss(engine, K_BATTLE_TIMER, t);
        return;
    }

    if mode == MODE_CATCH_ANIM {
        update_catch_anim(engine, dt);
        return;
    }

    if mode == MODE_TRANSITION {
        update_transition(engine, dt);
        return;
    }

    // Shop, Heal, Menu: no continuous update needed (event-driven)
    // But update message timer
    let msg_t = gs(engine, K_MSG_TIMER);
    if msg_t > 0.0 {
        ss(engine, K_MSG_TIMER, (msg_t - dt).max(0.0));
    }
}

fn update_overworld(engine: &mut Engine, dt: f64) {
    let px = gs(engine, K_PLAYER_X);
    let py = gs(engine, K_PLAYER_Y);
    let moving = gs(engine, K_PLAYER_MOVING);

    // Animation timer
    let anim = gs(engine, K_PLAYER_ANIM) + dt;
    ss(engine, K_PLAYER_ANIM, anim);

    if moving == 1.0 {
        let tx = gs(engine, K_PLAYER_TX);
        let ty = gs(engine, K_PLAYER_TY);
        let dx = tx - px;
        let dy = ty - py;
        let dist = (dx * dx + dy * dy).sqrt();
        let speed = 120.0 * dt;

        if dist > speed {
            let nx = px + (dx / dist) * speed;
            let ny = py + (dy / dist) * speed;

            // Direction
            if dx.abs() > dy.abs() {
                ss(engine, K_PLAYER_DIR, if dx > 0.0 { 3.0 } else { 2.0 });
            } else {
                ss(engine, K_PLAYER_DIR, if dy > 0.0 { 0.0 } else { 1.0 });
            }

            // Collision check
            if let Some(ref tm) = engine.tilemap {
                let tile_nx = (nx / TILE_SIZE) as usize;
                let tile_ny = (ny / TILE_SIZE) as usize;
                if tile_nx < MAP_W && tile_ny < MAP_H {
                    if let Some(tile) = tm.get(tile_nx, tile_ny) {
                        match tile.tile_type {
                            TileType::Solid => {
                                ss(engine, K_PLAYER_MOVING, 0.0);
                                return;
                            }
                            _ => {}
                        }
                    }
                }
            }

            ss(engine, K_PLAYER_X, nx);
            ss(engine, K_PLAYER_Y, ny);

            // Step counting for encounters
            let steps = gs(engine, K_STEP_COUNT) + speed / TILE_SIZE;
            ss(engine, K_STEP_COUNT, steps);

            // Check for encounters in wild grass
            let enc_cd = gs(engine, K_ENCOUNTER_CD);
            if enc_cd > 0.0 {
                ss(engine, K_ENCOUNTER_CD, enc_cd - dt);
            } else {
                check_wild_encounter(engine, nx, ny);
            }
        } else {
            ss(engine, K_PLAYER_MOVING, 0.0);
        }
    }

    // Check zone exits
    let zone = zone_from_f64(gs(engine, K_ZONE));
    let badges = gs(engine, K_BADGES) as u32;
    let cpx = gs(engine, K_PLAYER_X);
    let cpy = gs(engine, K_PLAYER_Y);
    if let Some((target, spawn_x, spawn_y)) = check_zone_exit(zone, cpx, cpy, badges) {
        start_zone_transition(engine, target, spawn_x, spawn_y);
    }

    // Check special tiles
    check_special_tiles(engine);
}

fn check_wild_encounter(engine: &mut Engine, px: f64, py: f64) {
    let zone = zone_from_f64(gs(engine, K_ZONE));
    let tile_x = (px / TILE_SIZE) as usize;
    let tile_y = (py / TILE_SIZE) as usize;

    if let Some(ref tm) = engine.tilemap {
        if tile_x < MAP_W && tile_y < MAP_H {
            if let Some(tile) = tm.get(tile_x, tile_y) {
            if let TileType::Custom(id) = tile.tile_type {
                if id == TILE_WILD {
                    // Random encounter chance
                    let r = rng(engine);
                    if r < 0.02 { // ~2% per movement check
                        let table = zone.encounter_table();
                        if !table.is_empty() {
                            start_wild_encounter(engine, zone);
                        }
                    }
                }
            }
            }
        }
    }
}

fn start_wild_encounter(engine: &mut Engine, zone: Zone) {
    let table = zone.encounter_table();
    if table.is_empty() { return; }

    // Weighted random selection
    let total_weight: u32 = table.iter().map(|e| e.3 as u32).sum();
    let mut roll = (rng(engine) * total_weight as f64) as u32;
    let mut selected = &table[0];
    for entry in table {
        if roll < entry.3 as u32 {
            selected = entry;
            break;
        }
        roll -= entry.3 as u32;
    }

    let (species_id, min_lv, max_lv, _) = *selected;
    let level = min_lv + ((rng(engine) * (max_lv - min_lv + 1) as f64) as u8).min(max_lv - min_lv);

    start_battle(engine, species_id, level, false);
}

fn check_special_tiles(engine: &mut Engine) {
    let px = gs(engine, K_PLAYER_X);
    let py = gs(engine, K_PLAYER_Y);
    let tile_x = (px / TILE_SIZE) as usize;
    let tile_y = (py / TILE_SIZE) as usize;

    if let Some(ref tm) = engine.tilemap {
        if tile_x < MAP_W && tile_y < MAP_H {
            if let Some(tile) = tm.get(tile_x, tile_y) {
            if let TileType::Custom(id) = tile.tile_type {
                match id {
                    TILE_HEAL => {
                        // Heal all monsters
                        let team_size = gs(engine, K_TEAM_SIZE) as usize;
                        for i in 0..team_size {
                            let maxhp = gs_team(engine, i, "maxhp");
                            ss_team(engine, i, "hp", maxhp);
                        }
                        play_heal_sound(engine);
                        ss(engine, K_MODE, MODE_DIALOGUE);
                        ss(engine, K_DLG_ID, 1.0);
                        ss(engine, K_DLG_LINE, 0.0);
                        // Move player off the tile
                        ss(engine, K_PLAYER_Y, py + TILE_SIZE);
                    }
                    TILE_SHOP => {
                        play_ui_click(engine);
                        ss(engine, K_MODE, MODE_SHOP);
                        ss(engine, K_SHOP_SEL, 0.0);
                        ss(engine, K_PLAYER_Y, py + TILE_SIZE);
                    }
                    TILE_BOSS => {
                        let zone = zone_from_f64(gs(engine, K_ZONE));
                        if let Some((boss_id, boss_lv)) = zone.boss() {
                            // Check if already defeated
                            let zone_bit = match zone {
                                Zone::EmberHollow => 1,
                                Zone::CoralShore => 2,
                                Zone::Sparkridge => 4,
                                Zone::ShadowVale => 8,
                                Zone::CrystalSpire => 16,
                                _ => 0,
                            };
                            let badges = gs(engine, K_BADGES) as u32;
                            if badges & zone_bit == 0 {
                                start_battle(engine, boss_id, boss_lv, true);
                            }
                        }
                    }
                    _ => {}
                }
            }
            }
        }
    }
}

fn start_battle(engine: &mut Engine, species_id: u8, level: u8, is_boss: bool) {
    play_encounter_sound(engine);
    engine.screen_fx.push(ScreenEffect::Flash {
        color: COL_WHITE,
        intensity: 0.8,
    }, 0.4);

    let sp = get_species(species_id);
    let lv = level as f64;
    ss(engine, K_MODE, MODE_BATTLE);
    ss(engine, K_BPHASE, BPHASE_INTRO);
    ss(engine, K_BATTLE_TIMER, 0.0);
    ss(engine, K_ENEMY_SPECIES, species_id as f64);
    ss(engine, K_ENEMY_LEVEL, lv);
    ss(engine, K_ENEMY_HP, calc_max_hp(sp.base_hp, lv));
    ss(engine, K_ENEMY_MAXHP, calc_max_hp(sp.base_hp, lv));
    ss(engine, K_ENEMY_ATK, calc_atk(sp.base_atk, lv));
    ss(engine, K_ENEMY_DEF, calc_def(sp.base_def, lv));
    ss(engine, K_IS_BOSS, if is_boss { 1.0 } else { 0.0 });
    ss(engine, K_BALL_ACTIVE, 0.0);
    ss(engine, K_AIMING, 0.0);
    ss(engine, K_STROKES, 0.0);
    ss(engine, K_BATTLE_MSG, 0.0);
    ss(engine, K_MSG_TIMER, 0.0);
    ss(engine, K_ENCOUNTER_CD, 3.0); // cooldown after battle

    // Find first alive monster in team
    let team_size = gs(engine, K_TEAM_SIZE) as usize;
    let mut active = 0;
    for i in 0..team_size {
        if gs_team(engine, i, "hp") > 0.0 {
            active = i;
            break;
        }
    }
    ss(engine, K_ACTIVE_MON, active as f64);

    // Ball starting position
    ss(engine, K_BALL_X, 15.0 * TILE_SIZE);
    ss(engine, K_BALL_Y, 35.0 * TILE_SIZE);

    if is_boss {
        play_boss_intro(engine);
    }

    // Build battle arena
    build_battle_arena(engine, sp.element);
}

fn start_zone_transition(engine: &mut Engine, target: Zone, spawn_x: f64, spawn_y: f64) {
    ss(engine, K_MODE, MODE_TRANSITION);
    ss(engine, K_TRANS_TIMER, 0.0);
    ss(engine, K_TRANS_TARGET, zone_to_f64(target));
    ss(engine, K_TRANS_MODE, MODE_OVERWORLD);
    ss(engine, K_PLAYER_TX, spawn_x);
    ss(engine, K_PLAYER_TY, spawn_y);

    engine.screen_fx.push(ScreenEffect::Flash {
        color: COL_BLACK,
        intensity: 1.0,
    }, 0.6);
}

fn update_transition(engine: &mut Engine, dt: f64) {
    let t = gs(engine, K_TRANS_TIMER) + dt;
    ss(engine, K_TRANS_TIMER, t);

    if t > 0.3 {
        // Halfway through - swap the map
        let target = zone_from_f64(gs(engine, K_TRANS_TARGET));
        ss(engine, K_ZONE, zone_to_f64(target));
        engine.tilemap = Some(build_zone_map(target));

        // Move player to spawn point
        ss(engine, K_PLAYER_X, gs(engine, K_PLAYER_TX));
        ss(engine, K_PLAYER_Y, gs(engine, K_PLAYER_TY));
        ss(engine, K_PLAYER_MOVING, 0.0);
    }

    if t > 0.6 {
        ss(engine, K_MODE, MODE_OVERWORLD);
    }
}

fn update_catch_anim(engine: &mut Engine, dt: f64) {
    let t = gs(engine, K_CATCH_TIMER) + dt;
    ss(engine, K_CATCH_TIMER, t);

    if t > 2.0 {
        let success = gs(engine, K_CATCH_SUCCESS) == 1.0;
        if success {
            // Add to team
            let team_size = gs(engine, K_TEAM_SIZE) as usize;
            if team_size < 6 {
                let species_id = gs(engine, K_ENEMY_SPECIES) as u8;
                let level = gs(engine, K_ENEMY_LEVEL) as u8;
                set_team_monster(engine, team_size, species_id, level);
                ss(engine, K_TEAM_SIZE, (team_size + 1) as f64);
                play_catch_sound(engine);
            }
            end_battle(engine, true);
        } else {
            // Failed catch - back to battle
            ss(engine, K_MODE, MODE_BATTLE);
            ss(engine, K_BPHASE, BPHASE_ENEMY_TURN);
            ss(engine, K_BATTLE_TIMER, 0.0);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// BATTLE UPDATE
// ═══════════════════════════════════════════════════════════════════════

fn update_battle(engine: &mut Engine, dt: f64) {
    let bphase = gs(engine, K_BPHASE);
    let timer = gs(engine, K_BATTLE_TIMER) + dt;
    ss(engine, K_BATTLE_TIMER, timer);

    // Update message timer
    let msg_t = gs(engine, K_MSG_TIMER);
    if msg_t > 0.0 {
        ss(engine, K_MSG_TIMER, (msg_t - dt).max(0.0));
    }

    // Update damage popup timer
    let popup_t = gs(engine, K_DMG_POPUP_TIMER);
    if popup_t > 0.0 {
        ss(engine, K_DMG_POPUP_TIMER, (popup_t - dt).max(0.0));
    }

    // Update enemy shake timer
    let shake_t = gs(engine, K_ENEMY_SHAKE);
    if shake_t > 0.0 {
        ss(engine, K_ENEMY_SHAKE, (shake_t - dt).max(0.0));
    }

    if bphase == BPHASE_INTRO {
        if timer > 1.5 {
            ss(engine, K_BPHASE, BPHASE_PLAYER_AIM);
            ss(engine, K_BATTLE_TIMER, 0.0);
        }
        return;
    }

    if bphase == BPHASE_PLAYER_SHOT {
        update_ball_physics(engine, dt);
        return;
    }

    if bphase == BPHASE_ENEMY_TURN {
        if timer > 1.0 {
            do_enemy_attack(engine);
            ss(engine, K_BPHASE, BPHASE_PLAYER_AIM);
            ss(engine, K_BATTLE_TIMER, 0.0);
        }
        return;
    }
}

fn update_ball_physics(engine: &mut Engine, dt: f64) {
    if gs(engine, K_BALL_ACTIVE) == 0.0 { return; }

    let mut bx = gs(engine, K_BALL_X);
    let mut by = gs(engine, K_BALL_Y);
    let mut vx = gs(engine, K_BALL_VX);
    let mut vy = gs(engine, K_BALL_VY);

    // Physics step
    let friction = 0.98;
    vx *= friction;
    vy *= friction;

    bx += vx * dt;
    by += vy * dt;

    // Wall bouncing
    let min_x = 2.0 * TILE_SIZE;
    let max_x = (MAP_W as f64 - 2.0) * TILE_SIZE;
    let min_y = 2.0 * TILE_SIZE;
    let max_y = (MAP_H as f64 - 2.0) * TILE_SIZE;

    if bx < min_x { bx = min_x; vx = -vx * 0.8; play_wall_bounce(engine); let wb = gs(engine, K_WALL_BOUNCES); ss(engine, K_WALL_BOUNCES, wb + 1.0); }
    if bx > max_x { bx = max_x; vx = -vx * 0.8; play_wall_bounce(engine); let wb = gs(engine, K_WALL_BOUNCES); ss(engine, K_WALL_BOUNCES, wb + 1.0); }
    if by < min_y { by = min_y; vy = -vy * 0.8; play_wall_bounce(engine); let wb = gs(engine, K_WALL_BOUNCES); ss(engine, K_WALL_BOUNCES, wb + 1.0); }
    if by > max_y { by = max_y; vy = -vy * 0.8; play_wall_bounce(engine); let wb = gs(engine, K_WALL_BOUNCES); ss(engine, K_WALL_BOUNCES, wb + 1.0); }

    // Bumper collision
    if let Some(ref tm) = engine.tilemap {
        let tile_x = (bx / TILE_SIZE) as usize;
        let tile_y = (by / TILE_SIZE) as usize;
        if tile_x < MAP_W && tile_y < MAP_H {
            if let Some(tile) = tm.get(tile_x, tile_y) {
            if let TileType::Custom(TILE_BUMPER) = tile.tile_type {
                // Bounce off bumper
                let center_x = (tile_x as f64 + 0.5) * TILE_SIZE;
                let center_y = (tile_y as f64 + 0.5) * TILE_SIZE;
                let dx = bx - center_x;
                let dy = by - center_y;
                let dist = (dx * dx + dy * dy).sqrt().max(0.1);
                vx = (dx / dist) * 300.0;
                vy = (dy / dist) * 300.0;
                play_wall_bounce(engine);
            }
            }
        }
    }

    // Track distance to target for headless testing
    let target_cx = 15.0 * TILE_SIZE;
    let target_cy = 7.0 * TILE_SIZE;
    let dx = bx - target_cx;
    let dy = by - target_cy;
    let dist = (dx * dx + dy * dy).sqrt();
    ss(engine, K_DIST_TO_HOLE, dist);
    let best = gs(engine, K_BEST_DIST);
    if dist < best { ss(engine, K_BEST_DIST, dist); }

    // Check if hit the target zone (enemy)
    let hit_radius = 2.5 * TILE_SIZE;

    if dist < hit_radius {
        // HIT! Calculate damage
        let active = gs(engine, K_ACTIVE_MON) as usize;
        let atk = gs_team(engine, active, "atk");
        let def = gs(engine, K_ENEMY_DEF);

        // Element effectiveness
        let active_species = gs_team(engine, active, "species") as u8;
        let enemy_species = gs(engine, K_ENEMY_SPECIES) as u8;
        let atk_elem = get_species(active_species).element;
        let def_elem = get_species(enemy_species).element;
        let eff = type_effectiveness(atk_elem, def_elem);

        // Speed of impact affects damage
        let speed = (vx * vx + vy * vy).sqrt();
        let speed_mult = (speed / 200.0).min(2.0).max(0.5);

        // Critical hit: 15% chance, 1.5x damage
        let crit_roll = rng(engine);
        let is_crit = crit_roll < 0.15;
        let crit_mult = if is_crit { 1.5 } else { 1.0 };

        // Damage variance: ±15%
        let variance = 0.85 + rng(engine) * 0.30;

        let raw_dmg = (atk * 2.0 - def * 0.8) * eff * speed_mult * crit_mult * variance;
        let damage = raw_dmg.max(1.0).floor();

        let enemy_hp = gs(engine, K_ENEMY_HP);
        let new_hp = (enemy_hp - damage).max(0.0);
        ss(engine, K_ENEMY_HP, new_hp);

        // Damage popup
        ss(engine, K_DMG_POPUP, damage);
        ss(engine, K_DMG_POPUP_X, bx);
        ss(engine, K_DMG_POPUP_Y, by);
        ss(engine, K_DMG_POPUP_TIMER, 1.0);
        ss(engine, K_DMG_CRIT, if is_crit { 1.0 } else { 0.0 });

        // Combo tracking
        let combo = gs(engine, K_COMBO) + 1.0;
        ss(engine, K_COMBO, combo);

        // Enemy shake on hit
        ss(engine, K_ENEMY_SHAKE, 0.3);

        // Sound varies by element
        play_element_hit_sound(engine, atk_elem, is_crit);

        // Show effectiveness message
        if is_crit && eff > 1.0 {
            ss(engine, K_BATTLE_MSG, 5.0); // "Critical! Super effective!"
            ss(engine, K_MSG_TIMER, 2.0);
            play_super_effective_sound(engine);
            engine.screen_fx.push(ScreenEffect::Flash {
                color: atk_elem.color(), intensity: 0.6,
            }, 0.4);
        } else if eff > 1.0 {
            ss(engine, K_BATTLE_MSG, 1.0);
            ss(engine, K_MSG_TIMER, 1.5);
            play_super_effective_sound(engine);
            engine.screen_fx.push(ScreenEffect::Flash {
                color: atk_elem.color(), intensity: 0.4,
            }, 0.3);
        } else if is_crit {
            ss(engine, K_BATTLE_MSG, 6.0); // "Critical hit!"
            ss(engine, K_MSG_TIMER, 1.5);
            engine.screen_fx.push(ScreenEffect::Flash {
                color: COL_WHITE, intensity: 0.3,
            }, 0.2);
        } else if eff < 1.0 {
            ss(engine, K_BATTLE_MSG, 2.0);
            ss(engine, K_MSG_TIMER, 1.5);
        }
        ss(engine, K_EFFECTIVENESS, eff);

        // Ball stops
        ss(engine, K_BALL_ACTIVE, 0.0);

        // Check if enemy fainted
        if new_hp <= 0.0 {
            on_enemy_defeated(engine);
        } else {
            // Enemy turn
            ss(engine, K_BPHASE, BPHASE_ENEMY_TURN);
            ss(engine, K_BATTLE_TIMER, 0.0);
        }
        return;
    }

    // Check if ball stopped (low velocity)
    let speed = (vx * vx + vy * vy).sqrt();
    if speed < 5.0 {
        ss(engine, K_BALL_ACTIVE, 0.0);
        ss(engine, K_COMBO, 0.0); // Reset combo on miss
        // Missed - enemy turn
        ss(engine, K_BPHASE, BPHASE_ENEMY_TURN);
        ss(engine, K_BATTLE_TIMER, 0.0);
        return;
    }

    ss(engine, K_BALL_X, bx);
    ss(engine, K_BALL_Y, by);
    ss(engine, K_BALL_VX, vx);
    ss(engine, K_BALL_VY, vy);
}

fn play_wall_bounce(engine: &mut Engine) {
    engine.sound_queue.push(SoundCommand::PlayTone {
        frequency: 150.0, duration: 0.06, volume: 0.2,
        waveform: Waveform::Square, attack: 0.002, decay: 0.05,
    });
}

fn do_enemy_attack(engine: &mut Engine) {
    let enemy_atk = gs(engine, K_ENEMY_ATK);
    let active = gs(engine, K_ACTIVE_MON) as usize;
    let def = gs_team(engine, active, "def");

    // Element effectiveness for enemy attacks too
    let enemy_species = gs(engine, K_ENEMY_SPECIES) as u8;
    let active_species = gs_team(engine, active, "species") as u8;
    let atk_elem = get_species(enemy_species).element;
    let def_elem = get_species(active_species).element;
    let eff = type_effectiveness(atk_elem, def_elem);

    // Damage variance ±15%
    let variance = rng_range(engine, 0.85, 1.15);

    let raw_dmg = (enemy_atk * 1.5 - def * 0.6) * eff * variance;
    let damage = raw_dmg.max(1.0).floor();

    let hp = gs_team(engine, active, "hp");
    let new_hp = (hp - damage).max(0.0);
    ss_team(engine, active, "hp", new_hp);

    play_element_hit_sound(engine, atk_elem, false);
    engine.screen_fx.push(ScreenEffect::Flash {
        color: atk_elem.color().with_alpha(200),
        intensity: 0.3,
    }, 0.2);

    // Show enemy effectiveness message
    if eff > 1.0 {
        ss(engine, K_BATTLE_MSG, 1.0);
        ss(engine, K_MSG_TIMER, 1.0);
    } else if eff < 1.0 {
        ss(engine, K_BATTLE_MSG, 2.0);
        ss(engine, K_MSG_TIMER, 1.0);
    }

    if new_hp <= 0.0 {
        // Current monster fainted - find next alive
        let team_size = gs(engine, K_TEAM_SIZE) as usize;
        let mut found = false;
        for i in 0..team_size {
            if gs_team(engine, i, "hp") > 0.0 {
                ss(engine, K_ACTIVE_MON, i as f64);
                found = true;
                break;
            }
        }
        if !found {
            // All fainted - game over, heal and return to town
            on_player_defeated(engine);
        }
    }

    // Reset ball position for next shot
    ss(engine, K_BALL_X, 15.0 * TILE_SIZE);
    ss(engine, K_BALL_Y, 35.0 * TILE_SIZE);
    ss(engine, K_BALL_VX, 0.0);
    ss(engine, K_BALL_VY, 0.0);
}

fn on_enemy_defeated(engine: &mut Engine) {
    play_victory_sound(engine);

    // XP reward
    let enemy_lv = gs(engine, K_ENEMY_LEVEL);
    let xp_gain = xp_reward(enemy_lv);
    let active = gs(engine, K_ACTIVE_MON) as usize;
    let current_xp = gs_team(engine, active, "xp");
    let new_xp = current_xp + xp_gain;
    ss_team(engine, active, "xp", new_xp);

    // Check level up
    let current_level = gs_team(engine, active, "level");
    let needed = xp_for_level(current_level + 1.0);
    if new_xp >= needed {
        level_up_monster(engine, active);
        ss_team(engine, active, "xp", 0.0);
        play_level_up_sound(engine);
    }

    // Gold reward
    let gold = gs(engine, K_GOLD);
    let gold_gain = (enemy_lv * 5.0 + 10.0).floor();
    ss(engine, K_GOLD, gold + gold_gain);

    // Boss badge
    if gs(engine, K_IS_BOSS) == 1.0 {
        let zone = zone_from_f64(gs(engine, K_ZONE));
        let zone_bit = match zone {
            Zone::EmberHollow => 1,
            Zone::CoralShore => 2,
            Zone::Sparkridge => 4,
            Zone::ShadowVale => 8,
            Zone::CrystalSpire => 16,
            _ => 0,
        };
        let badges = gs(engine, K_BADGES) as u32;
        ss(engine, K_BADGES, (badges | zone_bit) as f64);

        // Check if all badges collected
        if (badges | zone_bit) == 0b11111 {
            ss(engine, K_MODE, MODE_DIALOGUE);
            ss(engine, K_DLG_ID, 5.0);
            ss(engine, K_DLG_LINE, 0.0);
            return;
        }
    }

    ss(engine, K_MODE, MODE_BATTLE_RESULT);
    ss(engine, K_BATTLE_TIMER, 0.0);
}

fn on_player_defeated(engine: &mut Engine) {
    // Heal team to half HP and teleport back to Pebble Town
    let team_size = gs(engine, K_TEAM_SIZE) as usize;
    for i in 0..team_size {
        let maxhp = gs_team(engine, i, "maxhp");
        ss_team(engine, i, "hp", (maxhp / 2.0).floor().max(1.0));
    }

    // Lose some gold
    let gold = gs(engine, K_GOLD);
    ss(engine, K_GOLD, (gold * 0.7).floor());

    // Teleport to town
    ss(engine, K_ZONE, zone_to_f64(Zone::PebbleTown));
    engine.tilemap = Some(build_zone_map(Zone::PebbleTown));
    ss(engine, K_PLAYER_X, 15.0 * TILE_SIZE);
    ss(engine, K_PLAYER_Y, 24.0 * TILE_SIZE);
    ss(engine, K_MODE, MODE_OVERWORLD);

    engine.screen_fx.push(ScreenEffect::Desaturate { amount: 0.8 }, 1.0);
}

fn end_battle(engine: &mut Engine, victory: bool) {
    // Restore overworld map
    let zone = zone_from_f64(gs(engine, K_ZONE));
    engine.tilemap = Some(build_zone_map(zone));
    ss(engine, K_MODE, MODE_OVERWORLD);
    ss(engine, K_PLAYER_MOVING, 0.0);
    ss(engine, K_ENCOUNTER_CD, 3.0);

    if !victory {
        // Ran away - no penalty
    }
}

// ═══════════════════════════════════════════════════════════════════════
// RENDER
// ═══════════════════════════════════════════════════════════════════════

pub fn render(engine: &mut Engine) {
    let mode = gs(engine, K_MODE);
    engine.framebuffer.clear(COL_BG);

    if mode == MODE_TITLE {
        render_title(engine);
    } else if mode == MODE_OVERWORLD || mode == MODE_TRANSITION {
        render_overworld(engine);
    } else if mode == MODE_BATTLE || mode == MODE_BATTLE_RESULT {
        render_battle(engine);
    } else if mode == MODE_DIALOGUE {
        if gs(engine, K_DLG_ID) == 0.0 && gs(engine, K_DLG_LINE) < 1.0 {
            render_title(engine);
        } else {
            render_overworld(engine);
        }
        render_dialogue(engine);
    } else if mode == MODE_SHOP {
        render_overworld(engine);
        render_shop(engine);
    } else if mode == MODE_HEAL {
        render_overworld(engine);
        render_heal(engine);
    } else if mode == MODE_MENU {
        render_menu(engine);
    } else if mode == MODE_CATCH_ANIM {
        render_battle(engine);
        render_catch_anim(engine);
    }

    engine.screen_fx.apply(&mut engine.framebuffer);
}

fn render_title(engine: &mut Engine) {
    let t = gs(engine, K_TITLE_TIMER);
    let fb = &mut engine.framebuffer;

    for y in 0..HEIGHT as i32 {
        let ratio = y as f64 / HEIGHT;
        let r = (20.0 + ratio * 40.0) as u8;
        let g = (12.0 + ratio * 20.0) as u8;
        let b = (40.0 + ratio * 60.0) as u8;
        shapes::fill_rect(fb, 0.0, y as f64, WIDTH, 1.0, Color { r, g, b, a: 255 });
    }

    for i in 0..20 {
        let px = ((t * 20.0 + i as f64 * 73.0) % WIDTH).abs();
        let py = ((t * 15.0 + i as f64 * 47.0) % HEIGHT).abs();
        let size = 2.0 + (t + i as f64).sin().abs() * 3.0;
        let alpha = (100.0 + ((t * 2.0 + i as f64).sin() * 80.0)) as u8;
        shapes::fill_circle(fb, px, py, size, Color { r: 200, g: 180, b: 255, a: alpha });
    }

    text::draw_text_centered(fb, 240, 180, "S - L E A G U E", COL_UI_HIGHLIGHT, 4);
    text::draw_text_centered(fb, 240, 230, "Spirit Collection RPG", COL_UI_TEXT, 2);

    let bob = (t * 2.0).sin() * 5.0;
    shapes::fill_circle(fb, 160.0, 320.0 + bob, 25.0, Color { r: 80, g: 200, b: 80, a: 200 });
    shapes::fill_circle(fb, 240.0, 310.0 - bob, 25.0, Color { r: 240, g: 100, b: 60, a: 200 });
    shapes::fill_circle(fb, 320.0, 320.0 + bob, 25.0, Color { r: 80, g: 160, b: 240, a: 200 });

    let btn_y = 420.0;
    shapes::fill_rect(fb, 140.0, btn_y, 200.0, 40.0, COL_UI_BG);
    shapes::draw_rect(fb, 140.0, btn_y, 200.0, 40.0, COL_UI_HIGHLIGHT);
    text::draw_text_centered(fb, 240, (btn_y + 14.0) as i32, "NEW GAME", COL_UI_HIGHLIGHT, 2);

    text::draw_text_centered(fb, 240, 650, "Tap to begin!", COL_UI_TEXT, 1);
    text::draw_text_centered(fb, 240, 680, "Built with Crusty Engine", Color { r: 120, g: 120, b: 140, a: 255 }, 1);
}

fn render_overworld(engine: &mut Engine) {
    let px = gs(engine, K_PLAYER_X);
    let py = gs(engine, K_PLAYER_Y);
    let cam_x = px;
    let cam_y = py;

    if let Some(ref tm) = engine.tilemap {
        tm.render(&mut engine.framebuffer, cam_x, cam_y, 1.0, WIDTH as u32, HEIGHT as u32);
    }

    render_wild_grass_overlay(engine, cam_x, cam_y);
    render_player(engine, cam_x, cam_y);
    render_zone_banner(engine);
    render_overworld_hud(engine);
}

fn render_wild_grass_overlay(engine: &mut Engine, cam_x: f64, cam_y: f64) {
    let t = engine.time;
    let start_tx = ((cam_x - WIDTH / 2.0) / TILE_SIZE).floor() as i32;
    let end_tx = ((cam_x + WIDTH / 2.0) / TILE_SIZE).ceil() as i32;
    let start_ty = ((cam_y - HEIGHT / 2.0) / TILE_SIZE).floor() as i32;
    let end_ty = ((cam_y + HEIGHT / 2.0) / TILE_SIZE).ceil() as i32;

    // Collect wild tile positions first (to avoid borrow conflict)
    let mut wild_tiles: Vec<(i32, i32)> = Vec::new();
    if let Some(ref tm) = engine.tilemap {
        for ty in start_ty..=end_ty {
            for tx in start_tx..=end_tx {
                if tx >= 0 && ty >= 0 && (tx as usize) < MAP_W && (ty as usize) < MAP_H {
                    if let Some(tile) = tm.get(tx as usize, ty as usize) {
                        if let TileType::Custom(TILE_WILD) = tile.tile_type {
                            wild_tiles.push((tx, ty));
                        }
                    }
                }
            }
        }
    }

    let fb = &mut engine.framebuffer;
    for (tx, ty) in wild_tiles {
        let screen_x = (tx as f64 * TILE_SIZE) - cam_x + WIDTH / 2.0;
        let screen_y = (ty as f64 * TILE_SIZE) - cam_y + HEIGHT / 2.0;
        let wave = (t * 3.0 + tx as f64 * 0.7 + ty as f64 * 1.1).sin() * 3.0;
        shapes::draw_line(fb,
            screen_x + 4.0, screen_y + TILE_SIZE,
            screen_x + 4.0 + wave, screen_y + 4.0,
            COL_WILD_ACCENT);
        shapes::draw_line(fb,
            screen_x + 10.0, screen_y + TILE_SIZE,
            screen_x + 10.0 - wave, screen_y + 2.0,
            COL_WILD_ACCENT);
    }
}

fn render_player(engine: &mut Engine, cam_x: f64, cam_y: f64) {
    let px = gs(engine, K_PLAYER_X);
    let py = gs(engine, K_PLAYER_Y);
    let dir = gs(engine, K_PLAYER_DIR) as u32;
    let anim = gs(engine, K_PLAYER_ANIM);
    let moving = gs(engine, K_PLAYER_MOVING);
    let fb = &mut engine.framebuffer;

    let sx = px - cam_x + WIDTH / 2.0;
    let sy = py - cam_y + HEIGHT / 2.0;
    let bob = if moving == 1.0 { (anim * 8.0).sin() * 2.0 } else { 0.0 };

    shapes::fill_circle(fb, sx, sy - 4.0 + bob, 7.0, COL_PLAYER);
    shapes::fill_circle(fb, sx, sy - 12.0 + bob, 5.0, Color { r: 255, g: 220, b: 180, a: 255 });

    let (ex, ey) = match dir {
        0 => (0.0, 2.0), 1 => (0.0, -2.0), 2 => (-2.0, 0.0), 3 => (2.0, 0.0), _ => (0.0, 2.0),
    };
    shapes::fill_circle(fb, sx + ex - 1.5, sy - 12.0 + ey + bob, 1.0, COL_BLACK);
    shapes::fill_circle(fb, sx + ex + 1.5, sy - 12.0 + ey + bob, 1.0, COL_BLACK);
}

fn render_zone_banner(engine: &mut Engine) {
    let zone = zone_from_f64(gs(engine, K_ZONE));
    let badges = gs(engine, K_BADGES) as u32;
    let badge_count = badges.count_ones();
    let zone_name = zone.name();
    let badge_text = format!("Badges: {}/5", badge_count);
    let fb = &mut engine.framebuffer;

    shapes::fill_rect(fb, 0.0, 0.0, WIDTH, 24.0, Color { r: 0, g: 0, b: 0, a: 160 });
    text::draw_text_centered(fb, 240, 8, zone_name, COL_UI_TEXT, 1);
    text::draw_text(fb, 370, 8, &badge_text, COL_UI_HIGHLIGHT, 1);
}

fn render_overworld_hud(engine: &mut Engine) {
    let active = gs(engine, K_ACTIVE_MON) as usize;
    let species_id = gs_team(engine, active, "species") as u8;
    let sp = get_species(species_id);
    let level = gs_team(engine, active, "level") as u32;
    let hp = gs_team(engine, active, "hp");
    let maxhp = gs_team(engine, active, "maxhp");
    let gold = gs(engine, K_GOLD) as u32;
    let team_size = gs(engine, K_TEAM_SIZE) as usize;
    let mut team_hp = [0.0f64; 6];
    for i in 0..team_size.min(6) {
        team_hp[i] = gs_team(engine, i, "hp");
    }
    let fb = &mut engine.framebuffer;

    let hud_y = HEIGHT - 40.0;
    shapes::fill_rect(fb, 0.0, hud_y, WIDTH, 40.0, Color { r: 0, g: 0, b: 0, a: 180 });

    let info = format!("{} Lv{}", sp.name, level);
    text::draw_text(fb, 10, (hud_y + 5.0) as i32, &info, COL_UI_TEXT, 1);

    let bar_x = 10.0;
    let bar_y = hud_y + 20.0;
    let bar_w = 150.0;
    let bar_h = 8.0;
    shapes::fill_rect(fb, bar_x, bar_y, bar_w, bar_h, COL_HP_BG);
    let hp_ratio = (hp / maxhp).max(0.0).min(1.0);
    let hp_col = if hp_ratio > 0.5 { COL_HP_BAR }
        else if hp_ratio > 0.25 { Color { r: 220, g: 180, b: 40, a: 255 } }
        else { Color { r: 220, g: 60, b: 40, a: 255 } };
    shapes::fill_rect(fb, bar_x, bar_y, bar_w * hp_ratio, bar_h, hp_col);
    let hp_text = format!("{}/{}", hp as u32, maxhp as u32);
    text::draw_text(fb, (bar_x + bar_w + 5.0) as i32, bar_y as i32, &hp_text, COL_UI_TEXT, 1);

    let gold_text = format!("${}", gold);
    text::draw_text(fb, 380, (hud_y + 5.0) as i32, &gold_text, COL_UI_HIGHLIGHT, 1);

    for i in 0..team_size.min(6) {
        let ix = 380.0 + i as f64 * 14.0;
        let iy = hud_y + 22.0;
        let col = if team_hp[i] > 0.0 { COL_HP_BAR } else { Color { r: 100, g: 40, b: 40, a: 255 } };
        shapes::fill_circle(fb, ix + 5.0, iy + 4.0, 4.0, col);
    }
}

fn render_battle(engine: &mut Engine) {
    // Pre-read all state
    let bphase = gs(engine, K_BPHASE);
    let timer = gs(engine, K_BATTLE_TIMER);
    let mode = gs(engine, K_MODE);
    let time = engine.time;
    let enemy_species_id = gs(engine, K_ENEMY_SPECIES) as u8;
    let sp = get_species(enemy_species_id);
    let enemy_hp = gs(engine, K_ENEMY_HP);
    let enemy_maxhp = gs(engine, K_ENEMY_MAXHP);
    let enemy_level = gs(engine, K_ENEMY_LEVEL) as u32;
    let is_boss = gs(engine, K_IS_BOSS) == 1.0;
    let ball_active = gs(engine, K_BALL_ACTIVE) == 1.0;
    let ball_x = gs(engine, K_BALL_X);
    let ball_y = gs(engine, K_BALL_Y);
    let ball_vx = gs(engine, K_BALL_VX);
    let ball_vy = gs(engine, K_BALL_VY);
    let aiming = gs(engine, K_AIMING) == 1.0;
    let aim_sx = gs(engine, K_AIM_START_X);
    let aim_sy = gs(engine, K_AIM_START_Y);
    let aim_x = gs(engine, K_AIM_X);
    let aim_y = gs(engine, K_AIM_Y);
    let active_idx = gs(engine, K_ACTIVE_MON) as usize;
    let active_species_id = gs_team(engine, active_idx, "species") as u8;
    let active_sp = get_species(active_species_id);
    let active_lv = gs_team(engine, active_idx, "level") as u32;
    let active_hp = gs_team(engine, active_idx, "hp");
    let active_maxhp = gs_team(engine, active_idx, "maxhp");
    let active_xp = gs_team(engine, active_idx, "xp");
    let battle_msg = gs(engine, K_BATTLE_MSG) as u32;
    let msg_timer = gs(engine, K_MSG_TIMER);
    let orb_count = gs_inv(engine, ITEM_SPIRIT_ORB) as u32 + gs_inv(engine, ITEM_ULTRA_ORB) as u32;
    let dmg_popup = gs(engine, K_DMG_POPUP);
    let dmg_popup_x = gs(engine, K_DMG_POPUP_X);
    let dmg_popup_y = gs(engine, K_DMG_POPUP_Y);
    let dmg_popup_timer = gs(engine, K_DMG_POPUP_TIMER);
    let dmg_crit = gs(engine, K_DMG_CRIT) == 1.0;
    let combo = gs(engine, K_COMBO) as u32;
    let enemy_shake_timer = gs(engine, K_ENEMY_SHAKE);

    // Render tilemap
    if let Some(ref tm) = engine.tilemap {
        tm.render(&mut engine.framebuffer, TILEMAP_CAM_X, TILEMAP_CAM_Y, 1.0, WIDTH as u32, HEIGHT as u32);
    }

    let fb = &mut engine.framebuffer;

    // Enemy monster with hit shake
    let target_cx = 240.0;
    let target_cy = 7.0 * TILE_SIZE;
    let shake = if enemy_shake_timer > 0.0 {
        (enemy_shake_timer * 60.0).sin() * 6.0 * enemy_shake_timer
    } else if bphase == BPHASE_ENEMY_TURN {
        (timer * 20.0).sin() * 2.0
    } else { 0.0 };
    let bob = (time * 2.0).sin() * 3.0;
    render_monster_sprite(fb, target_cx + shake, target_cy + bob, &sp, false);

    // Enemy HP bar
    let bar_w = 120.0;
    let bar_x = target_cx - bar_w / 2.0;
    let bar_y = target_cy - 50.0;
    shapes::fill_rect(fb, bar_x - 1.0, bar_y - 1.0, bar_w + 2.0, 10.0, COL_UI_BORDER);
    shapes::fill_rect(fb, bar_x, bar_y, bar_w, 8.0, COL_HP_BG);
    let hp_ratio = (enemy_hp / enemy_maxhp).max(0.0).min(1.0);
    shapes::fill_rect(fb, bar_x, bar_y, bar_w * hp_ratio, 8.0, COL_HP_BAR);
    let enemy_name = format!("{} Lv{}", sp.name, enemy_level);
    text::draw_text_centered(fb, target_cx as i32, (bar_y - 14.0) as i32, &enemy_name, COL_UI_TEXT, 1);
    let hp_text = format!("{}/{}", enemy_hp as u32, enemy_maxhp as u32);
    text::draw_text_centered(fb, target_cx as i32, (bar_y + 12.0) as i32, &hp_text, COL_UI_TEXT, 1);
    text::draw_text_centered(fb, target_cx as i32, (bar_y - 26.0) as i32, sp.element.name(), sp.element.color(), 1);

    if is_boss {
        text::draw_text_centered(fb, target_cx as i32, (bar_y - 38.0) as i32, "GUARDIAN", COL_BOSS_TILE, 2);
    }

    // Energy orb with enhanced trail
    let orb_col = active_sp.element.color();
    if ball_active {
        let screen_x = ball_x - TILEMAP_CAM_X + WIDTH / 2.0;
        let screen_y = ball_y - TILEMAP_CAM_Y + HEIGHT / 2.0;
        let speed = (ball_vx * ball_vx + ball_vy * ball_vy).sqrt();

        // Multi-segment trail
        if speed > 30.0 {
            let norm = speed.max(1.0);
            let nvx = ball_vx / norm;
            let nvy = ball_vy / norm;
            for i in 1..=5 {
                let t = i as f64 * 3.0;
                let tx = screen_x - nvx * t;
                let ty = screen_y - nvy * t;
                let alpha = (180.0 - i as f64 * 30.0).max(20.0) as u8;
                let r = (5.0 - i as f64 * 0.6).max(1.0);
                shapes::fill_circle(fb, tx, ty, r, orb_col.with_alpha(alpha));
            }
        }

        // Glow
        shapes::fill_circle(fb, screen_x, screen_y, 10.0, orb_col.with_alpha(60));
        shapes::fill_circle(fb, screen_x, screen_y, 7.0, orb_col);
        shapes::fill_circle(fb, screen_x, screen_y, 3.0, COL_WHITE);
    } else if bphase == BPHASE_PLAYER_AIM {
        let screen_x = ball_x - TILEMAP_CAM_X + WIDTH / 2.0;
        let screen_y = ball_y - TILEMAP_CAM_Y + HEIGHT / 2.0;
        shapes::fill_circle(fb, screen_x, screen_y, 6.0, orb_col);
        shapes::fill_circle(fb, screen_x, screen_y, 3.0, COL_WHITE);

        if aiming {
            let dx = aim_sx - aim_x;
            let dy = aim_sy - aim_y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist > 5.0 {
                let power = (dist * 12.0).min(1800.0);
                let angle = dy.atan2(dx);
                let arrow_len = (power / 1800.0 * 80.0).min(80.0);
                let ax2 = screen_x + angle.cos() * arrow_len;
                let ay2 = screen_y + angle.sin() * arrow_len;

                // Color shifts from yellow to red with power
                let power_ratio = power / 1800.0;
                let arrow_col = Color {
                    r: 255, g: (220.0 * (1.0 - power_ratio * 0.7)) as u8,
                    b: (80.0 * (1.0 - power_ratio)) as u8, a: 255,
                };
                shapes::draw_line_thick(fb, screen_x, screen_y, ax2, ay2, 3.0, arrow_col);
                let power_pct = (power_ratio * 100.0) as u32;
                let pow_text = format!("{}%", power_pct);
                text::draw_text(fb, (screen_x + 15.0) as i32, (screen_y - 10.0) as i32, &pow_text, arrow_col, 1);
            }
        }
    }

    // Player monster panel
    let panel_y = HEIGHT - 130.0;
    shapes::fill_rect(fb, 0.0, panel_y, WIDTH, 130.0, Color { r: 0, g: 0, b: 0, a: 200 });
    shapes::draw_line(fb, 0.0, panel_y, WIDTH, panel_y, COL_UI_BORDER);
    render_monster_sprite(fb, 50.0, panel_y + 35.0, &active_sp, true);

    let info = format!("{} Lv{}", active_sp.name, active_lv);
    text::draw_text(fb, 90, (panel_y + 10.0) as i32, &info, COL_UI_TEXT, 1);
    text::draw_text(fb, 90, (panel_y + 24.0) as i32, active_sp.element.name(), active_sp.element.color(), 1);

    let pbar_x = 90.0;
    let pbar_y = panel_y + 38.0;
    shapes::fill_rect(fb, pbar_x, pbar_y, 120.0, 8.0, COL_HP_BG);
    let php_ratio = (active_hp / active_maxhp).max(0.0).min(1.0);
    shapes::fill_rect(fb, pbar_x, pbar_y, 120.0 * php_ratio, 8.0, COL_HP_BAR);
    let php_text = format!("{}/{}", active_hp as u32, active_maxhp as u32);
    text::draw_text(fb, pbar_x as i32, (pbar_y + 12.0) as i32, &php_text, COL_UI_TEXT, 1);

    let xp_needed = xp_for_level(active_lv as f64 + 1.0);
    let xp_ratio = if xp_needed > 0.0 { (active_xp / xp_needed).min(1.0) } else { 0.0 };
    let xpbar_y = pbar_y + 24.0;
    shapes::fill_rect(fb, pbar_x, xpbar_y, 120.0, 4.0, Color { r: 30, g: 30, b: 60, a: 255 });
    shapes::fill_rect(fb, pbar_x, xpbar_y, 120.0 * xp_ratio, 4.0, COL_XP_BAR);

    // Action buttons
    if bphase == BPHASE_PLAYER_AIM {
        let btn_y = panel_y + 75.0;
        let btn_w = WIDTH / 3.0;
        let btn_h = 40.0;

        shapes::fill_rect(fb, 0.0, btn_y, btn_w, btn_h, Color { r: 60, g: 40, b: 20, a: 255 });
        shapes::draw_rect(fb, 0.0, btn_y, btn_w, btn_h, COL_UI_HIGHLIGHT);
        text::draw_text_centered(fb, (btn_w / 2.0) as i32, (btn_y + 14.0) as i32, "STRIKE", COL_UI_HIGHLIGHT, 1);

        shapes::fill_rect(fb, btn_w, btn_y, btn_w, btn_h, Color { r: 20, g: 40, b: 60, a: 255 });
        shapes::draw_rect(fb, btn_w, btn_y, btn_w, btn_h, COL_UI_TEXT);
        let catch_text = format!("CATCH({})", orb_count);
        text::draw_text_centered(fb, (btn_w * 1.5) as i32, (btn_y + 14.0) as i32, &catch_text, COL_UI_TEXT, 1);

        shapes::fill_rect(fb, btn_w * 2.0, btn_y, btn_w, btn_h, Color { r: 40, g: 20, b: 20, a: 255 });
        shapes::draw_rect(fb, btn_w * 2.0, btn_y, btn_w, btn_h, COL_UI_TEXT);
        text::draw_text_centered(fb, (btn_w * 2.5) as i32, (btn_y + 14.0) as i32, "RUN", COL_UI_TEXT, 1);
    }

    // Battle intro
    if bphase == BPHASE_INTRO {
        let alpha = ((1.5 - timer) / 1.5 * 200.0).max(0.0) as u8;
        shapes::fill_rect(fb, 0.0, 250.0, WIDTH, 60.0, Color { r: 0, g: 0, b: 0, a: alpha });
        let intro_text = if is_boss {
            format!("Guardian {} appears!", sp.name)
        } else {
            format!("A wild {} appeared!", sp.name)
        };
        text::draw_text_centered(fb, 240, 272, &intro_text, COL_UI_TEXT, 2);
    }

    // Effectiveness message
    if battle_msg > 0 && msg_timer > 0.0 {
        let alpha = (msg_timer / 1.5 * 255.0).min(255.0) as u8;
        let (msg_text, msg_col) = match battle_msg {
            1 => ("Super effective!", Color { r: 255, g: 200, b: 60, a: alpha }),
            2 => ("Not very effective...", Color { r: 120, g: 120, b: 140, a: alpha }),
            3 => ("Can't catch Guardians!", Color { r: 255, g: 100, b: 100, a: alpha }),
            4 => ("Can't run from Guardian!", Color { r: 255, g: 100, b: 100, a: alpha }),
            5 => ("CRITICAL! Super effective!", Color { r: 255, g: 160, b: 40, a: alpha }),
            6 => ("Critical hit!", Color { r: 255, g: 255, b: 200, a: alpha }),
            _ => ("", COL_UI_TEXT),
        };
        if !msg_text.is_empty() {
            text::draw_text_centered(fb, 240, 280, msg_text, msg_col, 2);
        }
    }

    // Damage popup (floating number)
    if dmg_popup_timer > 0.0 && dmg_popup > 0.0 {
        let screen_x = dmg_popup_x - TILEMAP_CAM_X + WIDTH / 2.0;
        let screen_y = dmg_popup_y - TILEMAP_CAM_Y + HEIGHT / 2.0;
        let float_offset = (1.0 - dmg_popup_timer) * 40.0;
        let alpha = (dmg_popup_timer * 255.0).min(255.0) as u8;
        let dmg_text = format!("{}", dmg_popup as u32);
        let scale = if dmg_crit { 3 } else { 2 };
        let col = if dmg_crit {
            Color { r: 255, g: 220, b: 60, a: alpha }
        } else {
            Color { r: 255, g: 255, b: 255, a: alpha }
        };
        text::draw_text_centered(fb, screen_x as i32, (screen_y - float_offset) as i32, &dmg_text, col, scale);
        if dmg_crit {
            text::draw_text_centered(fb, screen_x as i32, (screen_y - float_offset - 20.0) as i32, "CRIT!", Color { r: 255, g: 180, b: 40, a: alpha }, 1);
        }
    }

    // Combo counter
    if combo >= 2 {
        let combo_text = format!("{} HIT COMBO!", combo);
        let pulse = (time * 4.0).sin() * 0.3 + 0.7;
        let alpha = (pulse * 255.0) as u8;
        text::draw_text_centered(fb, 240, 220, &combo_text, Color { r: 255, g: 200, b: 100, a: alpha }, 2);
    }

    // Victory overlay
    if mode == MODE_BATTLE_RESULT {
        shapes::fill_rect(fb, 80.0, 250.0, 320.0, 120.0, Color { r: 0, g: 0, b: 0, a: 220 });
        shapes::draw_rect(fb, 80.0, 250.0, 320.0, 120.0, COL_UI_HIGHLIGHT);
        text::draw_text_centered(fb, 240, 270, "VICTORY!", COL_UI_HIGHLIGHT, 3);
        let enemy_lv = enemy_level as f64;
        let xp = xp_reward(enemy_lv);
        let gold_gain = (enemy_lv * 5.0 + 10.0).floor();
        let reward_text = format!("+{} XP  +${}", xp as u32, gold_gain as u32);
        text::draw_text_centered(fb, 240, 310, &reward_text, COL_UI_TEXT, 1);
        if is_boss {
            text::draw_text_centered(fb, 240, 340, "Badge earned!", COL_UI_HIGHLIGHT, 2);
        }
        text::draw_text_centered(fb, 240, 360, "Tap to continue", COL_UI_TEXT, 1);
    }
}

fn render_monster_sprite(fb: &mut crate::rendering::framebuffer::Framebuffer, cx: f64, cy: f64, sp: &MonsterSpecies, small: bool) {
    let scale = if small { 0.6 } else { 1.0 };
    let r = 20.0 * scale;

    shapes::fill_circle(fb, cx, cy, r, sp.body_color);

    match sp.element {
        Element::Fire => {
            shapes::fill_circle(fb, cx - r * 0.5, cy - r * 0.8, r * 0.3, sp.accent_color);
            shapes::fill_circle(fb, cx + r * 0.5, cy - r * 0.8, r * 0.3, sp.accent_color);
            shapes::fill_circle(fb, cx, cy - r, r * 0.4, Color { r: 255, g: 200, b: 60, a: 255 });
        }
        Element::Water => {
            shapes::fill_circle(fb, cx, cy - r * 0.3, r * 0.6, sp.accent_color);
            shapes::fill_circle(fb, cx - r * 0.4, cy + r * 0.4, r * 0.2, Color { r: 200, g: 230, b: 255, a: 180 });
        }
        Element::Leaf => {
            shapes::fill_circle(fb, cx - r * 0.7, cy - r * 0.7, r * 0.3, sp.accent_color);
            shapes::fill_circle(fb, cx + r * 0.7, cy - r * 0.7, r * 0.3, sp.accent_color);
        }
        Element::Electric => {
            shapes::draw_line_thick(fb, cx - r * 0.5, cy - r, cx - r * 0.2, cy - r * 0.5, 2.0, Color { r: 255, g: 255, b: 100, a: 255 });
            shapes::draw_line_thick(fb, cx + r * 0.5, cy - r, cx + r * 0.2, cy - r * 0.5, 2.0, Color { r: 255, g: 255, b: 100, a: 255 });
        }
        Element::Earth => {
            shapes::fill_circle(fb, cx - r * 0.3, cy - r * 0.2, r * 0.15, sp.accent_color);
            shapes::fill_circle(fb, cx + r * 0.4, cy + r * 0.1, r * 0.15, sp.accent_color);
            shapes::fill_circle(fb, cx, cy + r * 0.3, r * 0.15, sp.accent_color);
        }
        Element::Ice => {
            shapes::fill_circle(fb, cx, cy - r * 0.9, r * 0.25, Color { r: 200, g: 240, b: 255, a: 255 });
            shapes::fill_circle(fb, cx - r * 0.6, cy - r * 0.5, r * 0.2, Color { r: 200, g: 240, b: 255, a: 255 });
            shapes::fill_circle(fb, cx + r * 0.6, cy - r * 0.5, r * 0.2, Color { r: 200, g: 240, b: 255, a: 255 });
        }
        Element::Shadow => {
            shapes::fill_circle(fb, cx - r * 0.8, cy, r * 0.2, sp.accent_color);
            shapes::fill_circle(fb, cx + r * 0.8, cy, r * 0.2, sp.accent_color);
            shapes::fill_circle(fb, cx, cy + r * 0.9, r * 0.15, sp.accent_color);
        }
        Element::Light => {
            for i in 0..6 {
                let angle = i as f64 * std::f64::consts::PI / 3.0;
                let x2 = cx + angle.cos() * r * 1.3;
                let y2 = cy + angle.sin() * r * 1.3;
                shapes::draw_line(fb, cx, cy, x2, y2, Color { r: 255, g: 240, b: 180, a: 100 });
            }
        }
        _ => {}
    }

    let eye_y = cy - r * 0.15;
    let eye_r = r * 0.12;
    shapes::fill_circle(fb, cx - r * 0.3, eye_y, eye_r + 1.0, COL_WHITE);
    shapes::fill_circle(fb, cx + r * 0.3, eye_y, eye_r + 1.0, COL_WHITE);
    shapes::fill_circle(fb, cx - r * 0.3, eye_y, eye_r, COL_BLACK);
    shapes::fill_circle(fb, cx + r * 0.3, eye_y, eye_r, COL_BLACK);
    shapes::draw_line(fb, cx - r * 0.2, cy + r * 0.2, cx + r * 0.2, cy + r * 0.2, sp.accent_color);
}

fn render_dialogue(engine: &mut Engine) {
    let id = gs(engine, K_DLG_ID) as u32;
    let line = gs(engine, K_DLG_LINE) as usize;
    let lines = get_dialogue(id);
    let time = engine.time;
    let fb = &mut engine.framebuffer;

    let box_y = HEIGHT - 160.0;
    shapes::fill_rect(fb, 20.0, box_y, WIDTH - 40.0, 140.0, Color { r: 10, g: 10, b: 20, a: 230 });
    shapes::draw_rect(fb, 20.0, box_y, WIDTH - 40.0, 140.0, COL_UI_BORDER);

    if line < lines.len() {
        if line > 0 {
            text::draw_text(fb, 40, (box_y + 15.0) as i32, lines[line - 1], Color { r: 120, g: 120, b: 140, a: 255 }, 2);
        }
        text::draw_text(fb, 40, (box_y + 45.0) as i32, lines[line], COL_UI_TEXT, 2);
    }

    let blink = (time * 3.0).sin() > 0.0;
    if blink {
        text::draw_text(fb, 380, (box_y + 110.0) as i32, ">>>", COL_UI_HIGHLIGHT, 1);
    }

    let total = lines.len();
    let dot_y = box_y + 120.0;
    for i in 0..total {
        let dot_x = 40.0 + i as f64 * 12.0;
        let col = if i <= line { COL_UI_HIGHLIGHT } else { COL_UI_BORDER };
        shapes::fill_circle(fb, dot_x, dot_y, 3.0, col);
    }
}

fn render_shop(engine: &mut Engine) {
    let gold = gs(engine, K_GOLD) as u32;
    let items = [ITEM_SPIRIT_ORB, ITEM_POTION, ITEM_SUPER_POTION, ITEM_REVIVE, ITEM_ULTRA_ORB];
    let mut owned = [0u32; 5];
    for (i, &item) in items.iter().enumerate() {
        owned[i] = gs_inv(engine, item) as u32;
    }
    let fb = &mut engine.framebuffer;

    shapes::fill_rect(fb, 40.0, 60.0, WIDTH - 80.0, HEIGHT - 120.0, Color { r: 15, g: 15, b: 30, a: 240 });
    shapes::draw_rect(fb, 40.0, 60.0, WIDTH - 80.0, HEIGHT - 120.0, COL_UI_BORDER);
    text::draw_text_centered(fb, 240, 85, "SPIRIT SHOP", COL_UI_HIGHLIGHT, 2);
    let gold_text = format!("Gold: ${}", gold);
    text::draw_text_centered(fb, 240, 115, &gold_text, COL_UI_TEXT, 1);
    shapes::draw_line(fb, 60.0, 140.0, WIDTH - 60.0, 140.0, COL_UI_BORDER);

    let start_y = 160.0;
    let item_h = 60.0;
    for (i, &item) in items.iter().enumerate() {
        let iy = start_y + i as f64 * item_h;
        let price = item_price(item);
        let can_buy = gold >= price;
        let bg_col = if can_buy { Color { r: 30, g: 30, b: 50, a: 255 } }
            else { Color { r: 20, g: 15, b: 25, a: 255 } };
        shapes::fill_rect(fb, 60.0, iy, WIDTH - 120.0, item_h - 4.0, bg_col);
        shapes::draw_rect(fb, 60.0, iy, WIDTH - 120.0, item_h - 4.0, COL_UI_BORDER);
        let name_col = if can_buy { COL_UI_TEXT } else { Color { r: 100, g: 100, b: 120, a: 255 } };
        text::draw_text(fb, 80, (iy + 10.0) as i32, item_name(item), name_col, 2);
        let price_text = format!("${}", price);
        text::draw_text(fb, 300, (iy + 10.0) as i32, &price_text, COL_UI_HIGHLIGHT, 1);
        let owned_text = format!("Own: {}", owned[i]);
        text::draw_text(fb, 300, (iy + 30.0) as i32, &owned_text, COL_UI_TEXT, 1);
    }

    let exit_y = HEIGHT - 90.0;
    shapes::fill_rect(fb, 160.0, exit_y, 160.0, 35.0, Color { r: 60, g: 30, b: 30, a: 255 });
    shapes::draw_rect(fb, 160.0, exit_y, 160.0, 35.0, COL_UI_TEXT);
    text::draw_text_centered(fb, 240, (exit_y + 10.0) as i32, "EXIT", COL_UI_TEXT, 2);
}

fn render_heal(engine: &mut Engine) {
    let fb = &mut engine.framebuffer;
    shapes::fill_rect(fb, 80.0, 250.0, 320.0, 100.0, Color { r: 10, g: 10, b: 20, a: 230 });
    shapes::draw_rect(fb, 80.0, 250.0, 320.0, 100.0, COL_HEAL);
    text::draw_text_centered(fb, 240, 275, "All creatures healed!", COL_HEAL, 2);
    text::draw_text_centered(fb, 240, 310, "Tap to continue", COL_UI_TEXT, 1);
}

fn render_menu(engine: &mut Engine) {
    let team_size = gs(engine, K_TEAM_SIZE) as usize;

    // Pre-read all team data
    struct MonInfo { sp: MonsterSpecies, level: u32, hp: f64, maxhp: f64 }
    let mut team_info: Vec<MonInfo> = Vec::new();
    for i in 0..team_size.min(6) {
        let sid = gs_team(engine, i, "species") as u8;
        team_info.push(MonInfo {
            sp: get_species(sid),
            level: gs_team(engine, i, "level") as u32,
            hp: gs_team(engine, i, "hp"),
            maxhp: gs_team(engine, i, "maxhp"),
        });
    }

    let items_arr = [ITEM_POTION, ITEM_SUPER_POTION, ITEM_SPIRIT_ORB, ITEM_ULTRA_ORB, ITEM_REVIVE];
    let mut item_counts = [0u32; 5];
    for (i, &item) in items_arr.iter().enumerate() {
        item_counts[i] = gs_inv(engine, item) as u32;
    }
    let potions = gs_inv(engine, ITEM_POTION) as u32 + gs_inv(engine, ITEM_SUPER_POTION) as u32;

    let fb = &mut engine.framebuffer;

    shapes::fill_rect(fb, 20.0, 20.0, WIDTH - 40.0, HEIGHT - 40.0, Color { r: 15, g: 12, b: 25, a: 245 });
    shapes::draw_rect(fb, 20.0, 20.0, WIDTH - 40.0, HEIGHT - 40.0, COL_UI_BORDER);
    text::draw_text_centered(fb, 240, 50, "TEAM", COL_UI_HIGHLIGHT, 3);

    for (i, info) in team_info.iter().enumerate() {
        let y_pos = 90.0 + i as f64 * 80.0;
        let bg = if info.hp > 0.0 { Color { r: 30, g: 30, b: 45, a: 255 } }
            else { Color { r: 40, g: 20, b: 20, a: 255 } };
        shapes::fill_rect(fb, 40.0, y_pos, WIDTH - 80.0, 70.0, bg);
        shapes::draw_rect(fb, 40.0, y_pos, WIDTH - 80.0, 70.0, COL_UI_BORDER);
        render_monster_sprite(fb, 80.0, y_pos + 35.0, &info.sp, true);
        let name = format!("{} Lv{}", info.sp.name, info.level);
        text::draw_text(fb, 110, (y_pos + 10.0) as i32, &name, COL_UI_TEXT, 2);
        text::draw_text(fb, 110, (y_pos + 30.0) as i32, info.sp.element.name(), info.sp.element.color(), 1);
        shapes::fill_rect(fb, 110.0, y_pos + 45.0, 200.0, 8.0, COL_HP_BG);
        let ratio = (info.hp / info.maxhp).max(0.0).min(1.0);
        shapes::fill_rect(fb, 110.0, y_pos + 45.0, 200.0 * ratio, 8.0, COL_HP_BAR);
        let hp_text = format!("{}/{}", info.hp as u32, info.maxhp as u32);
        text::draw_text(fb, 320, (y_pos + 42.0) as i32, &hp_text, COL_UI_TEXT, 1);
    }

    let items_y = 90.0 + team_info.len().min(6) as f64 * 80.0 + 10.0;
    text::draw_text(fb, 50, items_y as i32, "Items:", COL_UI_HIGHLIGHT, 1);
    let mut ix = 50.0;
    for (i, &item) in items_arr.iter().enumerate() {
        if item_counts[i] > 0 {
            let txt = format!("{}x{} ", item_name(item), item_counts[i]);
            text::draw_text(fb, ix as i32, (items_y + 16.0) as i32, &txt, COL_UI_TEXT, 1);
            ix += text::text_width(&txt, 1) as f64 + 5.0;
        }
    }

    if potions > 0 {
        shapes::fill_rect(fb, 120.0, 300.0, 240.0, 35.0, Color { r: 40, g: 60, b: 40, a: 255 });
        shapes::draw_rect(fb, 120.0, 300.0, 240.0, 35.0, COL_HP_BAR);
        text::draw_text_centered(fb, 240, 310, "Use Potion", COL_HP_BAR, 2);
    }

    let exit_y = HEIGHT - 80.0;
    shapes::fill_rect(fb, 160.0, exit_y, 160.0, 35.0, Color { r: 60, g: 30, b: 30, a: 255 });
    shapes::draw_rect(fb, 160.0, exit_y, 160.0, 35.0, COL_UI_TEXT);
    text::draw_text_centered(fb, 240, (exit_y + 10.0) as i32, "BACK", COL_UI_TEXT, 2);
}

fn render_catch_anim(engine: &mut Engine) {
    let t = gs(engine, K_CATCH_TIMER);
    let success = gs(engine, K_CATCH_SUCCESS) == 1.0;
    let fb = &mut engine.framebuffer;

    let orb_x = 240.0;
    let orb_y = if t < 0.5 {
        HEIGHT - 100.0 - t * 400.0
    } else {
        HEIGHT - 300.0 + (t - 0.5).sin() * 20.0
    };

    shapes::fill_circle(fb, orb_x, orb_y, 12.0, Color { r: 200, g: 200, b: 255, a: 255 });
    shapes::fill_circle(fb, orb_x, orb_y, 8.0, Color { r: 255, g: 255, b: 255, a: 255 });
    shapes::draw_circle(fb, orb_x, orb_y, 12.0, COL_UI_BORDER);

    if t > 1.0 && t < 2.0 {
        let shakes = ((t - 1.0) * 6.0) as u32;
        if shakes < 3 {
            let shake_text = ".".repeat(shakes as usize + 1);
            text::draw_text_centered(fb, 240, 350, &shake_text, COL_UI_TEXT, 3);
        }
    }

    if t > 2.0 {
        if success {
            text::draw_text_centered(fb, 240, 350, "CAUGHT!", COL_UI_HIGHLIGHT, 3);
        } else {
            text::draw_text_centered(fb, 240, 350, "It broke free!", COL_UI_TEXT, 2);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// HEADLESS TESTING API
// ═══════════════════════════════════════════════════════════════════════

pub fn dispatch_action(engine: &mut Engine, action: &crate::headless::ScheduledAction) {
    match action {
        crate::headless::ScheduledAction::PointerDown { x, y, .. } => on_pointer_down(engine, *x, *y),
        crate::headless::ScheduledAction::PointerMove { x, y, .. } => on_pointer_move(engine, *x, *y),
        crate::headless::ScheduledAction::PointerUp { x, y, .. } => on_pointer_up(engine, *x, *y),
    }
}

pub fn score_hole_completion(sim: &crate::headless::SimResult) -> f64 {
    let enemy_hp = sim.get_f64("enemy_hp").unwrap_or(100.0);
    let enemy_maxhp = sim.get_f64("enemy_maxhp").unwrap_or(100.0);
    if enemy_maxhp <= 0.0 { return 0.0; }
    (1.0 - (enemy_hp / enemy_maxhp)).max(0.0).min(1.0)
}

pub fn score_stroke_efficiency(sim: &crate::headless::SimResult) -> f64 {
    let strokes = sim.get_f64("strokes").unwrap_or(10.0);
    if strokes <= 0.0 { return 1.0; }
    (1.0 / strokes).min(1.0)
}

pub fn score_proximity_to_hole(sim: &crate::headless::SimResult) -> f64 {
    let bx = sim.get_f64("ball_x").unwrap_or(240.0);
    let by = sim.get_f64("ball_y").unwrap_or(560.0);
    let target_x = 15.0 * TILE_SIZE;
    let target_y = 7.0 * TILE_SIZE;
    let dx = bx - target_x;
    let dy = by - target_y;
    let dist = (dx * dx + dy * dy).sqrt();
    (1.0 - dist / 400.0).max(0.0).min(1.0)
}
