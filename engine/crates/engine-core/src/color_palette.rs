/// ColorPalette — Procedural Art Identity System
///
/// Provides semantic color roles, harmonious palette generation from a base hue
/// or seed, and a library of hand-crafted built-in palettes for immediate use.
/// All math uses f64. HashMap is keyed by string role names for simplicity.

use std::collections::HashMap;
use crate::rendering::color::Color;

// ─── Semantic Role Names ─────────────────────────────────────────────────────

/// The canonical string key for each semantic color role.
/// Use these constants with `ColorPalette::get_by_name` / `set_by_name`,
/// or use the typed `PaletteRole` enum with `get` / `set`.
pub const ROLE_BACKGROUND: &str = "background";
pub const ROLE_PRIMARY:    &str = "primary";
pub const ROLE_SECONDARY:  &str = "secondary";
pub const ROLE_ACCENT:     &str = "accent";
pub const ROLE_DANGER:     &str = "danger";
pub const ROLE_UI_TEXT:    &str = "ui_text";
pub const ROLE_UI_BG:      &str = "ui_bg";

/// All role name strings in a fixed order.
pub const ALL_ROLES: [&str; 7] = [
    ROLE_BACKGROUND,
    ROLE_PRIMARY,
    ROLE_SECONDARY,
    ROLE_ACCENT,
    ROLE_DANGER,
    ROLE_UI_TEXT,
    ROLE_UI_BG,
];

// ─── PaletteRole enum ─────────────────────────────────────────────────────────

/// Typed semantic color role. Maps 1-to-1 with the string keys in `ALL_ROLES`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PaletteRole {
    Background,
    Primary,
    Secondary,
    Accent,
    Danger,
    UiText,
    UiBg,
}

impl PaletteRole {
    /// Convert to the canonical HashMap key string.
    pub fn as_key(&self) -> &'static str {
        match self {
            PaletteRole::Background => ROLE_BACKGROUND,
            PaletteRole::Primary    => ROLE_PRIMARY,
            PaletteRole::Secondary  => ROLE_SECONDARY,
            PaletteRole::Accent     => ROLE_ACCENT,
            PaletteRole::Danger     => ROLE_DANGER,
            PaletteRole::UiText     => ROLE_UI_TEXT,
            PaletteRole::UiBg       => ROLE_UI_BG,
        }
    }
}

// ─── ColorScheme enum ─────────────────────────────────────────────────────────

/// Color harmony scheme used by `ColorPalette::from_hue`.
#[derive(Clone, Debug, PartialEq)]
pub enum ColorScheme {
    /// Two hues directly opposite on the color wheel (180°).
    Complementary,
    /// Three hues equally spaced (120° apart).
    Triadic,
    /// Hues adjacent on the wheel (±30° steps).
    Analogous,
    /// Base hue plus two hues flanking its complement (base, +150°, +210°).
    SplitComplementary,
}

// ─── ColorPalette struct ──────────────────────────────────────────────────────

/// A named collection of semantic colors covering all seven palette roles.
///
/// Colors are stored in a `HashMap<String, Color>` keyed by role name strings
/// (e.g. `"background"`, `"primary"`, …) so callers can look up by either
/// the typed `PaletteRole` enum or a raw `&str`.
#[derive(Clone, Debug)]
pub struct ColorPalette {
    pub name: String,
    pub colors: HashMap<String, Color>,
}

impl Default for ColorPalette {
    /// Returns a neutral dark-theme palette.
    fn default() -> Self {
        let mut p = Self::new("default");
        p.set(PaletteRole::Background, Color::from_rgba(18,  18,  28,  255));
        p.set(PaletteRole::Primary,    Color::from_rgba(90,  130, 210, 255));
        p.set(PaletteRole::Secondary,  Color::from_rgba(60,  90,  160, 255));
        p.set(PaletteRole::Accent,     Color::from_rgba(255, 200, 60,  255));
        p.set(PaletteRole::Danger,     Color::from_rgba(220, 60,  60,  255));
        p.set(PaletteRole::UiText,     Color::from_rgba(220, 220, 230, 255));
        p.set(PaletteRole::UiBg,       Color::from_rgba(30,  30,  45,  200));
        p
    }
}

impl ColorPalette {
    // ─── Core API ────────────────────────────────────────────────────────────

    /// Create an empty palette with the given name.
    /// All seven roles default to fully-transparent black until set.
    pub fn new(name: &str) -> Self {
        let mut colors = HashMap::with_capacity(ALL_ROLES.len());
        for role in &ALL_ROLES {
            colors.insert(role.to_string(), Color::TRANSPARENT);
        }
        Self { name: name.to_string(), colors }
    }

    /// Get the color for a typed role. Returns transparent black if the role
    /// is somehow missing (should never happen for a properly constructed palette).
    pub fn get(&self, role: PaletteRole) -> Color {
        self.colors.get(role.as_key()).copied().unwrap_or(Color::TRANSPARENT)
    }

    /// Set the color for a typed role.
    pub fn set(&mut self, role: PaletteRole, color: Color) {
        self.colors.insert(role.as_key().to_string(), color);
    }

    /// Get by raw string key. Returns `None` if key is not a recognised role.
    pub fn get_by_name(&self, key: &str) -> Option<Color> {
        self.colors.get(key).copied()
    }

    /// Set by raw string key. Silently does nothing if key is not recognised.
    pub fn set_by_name(&mut self, key: &str, color: Color) {
        if self.colors.contains_key(key) {
            self.colors.insert(key.to_string(), color);
        }
    }

    // ─── Generation from hue ─────────────────────────────────────────────────

    /// Generate a harmonious palette from a base hue (0–360 degrees on the
    /// HSL colour wheel) using the requested colour harmony scheme.
    ///
    /// Saturation and lightness are chosen for each role to produce good
    /// contrast across background / content / UI layers.
    pub fn from_hue(name: &str, base_hue: f64, scheme: ColorScheme) -> Self {
        let h0 = base_hue % 360.0;

        // Derive up to four hues from the scheme.
        let (h1, h2, h3) = match scheme {
            ColorScheme::Complementary => {
                let comp = (h0 + 180.0) % 360.0;
                (comp, (h0 + 30.0) % 360.0, (comp + 30.0) % 360.0)
            }
            ColorScheme::Triadic => {
                let t1 = (h0 + 120.0) % 360.0;
                let t2 = (h0 + 240.0) % 360.0;
                (t1, t2, (h0 + 60.0) % 360.0)
            }
            ColorScheme::Analogous => {
                let a1 = (h0 + 30.0) % 360.0;
                let a2 = (h0 - 30.0 + 360.0) % 360.0;
                (a1, a2, (h0 + 60.0) % 360.0)
            }
            ColorScheme::SplitComplementary => {
                let s1 = (h0 + 150.0) % 360.0;
                let s2 = (h0 + 210.0) % 360.0;
                (s1, s2, (h0 + 180.0) % 360.0)
            }
        };

        let mut p = Self::new(name);
        // Background: very dark, desaturated version of base hue
        p.set(PaletteRole::Background, hsl_to_rgb(h0,  0.20, 0.08));
        // Primary: vivid base hue
        p.set(PaletteRole::Primary,    hsl_to_rgb(h0,  0.75, 0.55));
        // Secondary: second harmony hue, slightly muted
        p.set(PaletteRole::Secondary,  hsl_to_rgb(h1,  0.60, 0.45));
        // Accent: third harmony hue, bright
        p.set(PaletteRole::Accent,     hsl_to_rgb(h2,  0.90, 0.65));
        // Danger: always a red-shifted hue regardless of scheme
        p.set(PaletteRole::Danger,     hsl_to_rgb(h3.min(30.0).max(0.0) + (h0 * 0.0),
                                                   0.85, 0.55));
        // Use a dedicated danger-red derived from the scheme's fourth hue
        // (override with a more readable red approach)
        p.set(PaletteRole::Danger,     hsl_to_rgb(5.0 + (h3 % 20.0), 0.85, 0.55));
        // UI Text: near-white, very light tint of base hue
        p.set(PaletteRole::UiText,     hsl_to_rgb(h0,  0.15, 0.90));
        // UI Background: dark semi-transparent overlay tinted to base hue
        p.set(PaletteRole::UiBg,       hsl_to_rgb(h0,  0.25, 0.12).with_alpha(210));
        p
    }

    // ─── Generation from seed ────────────────────────────────────────────────

    /// Generate a deterministic palette from a 64-bit seed.
    /// The same seed always produces the same palette.
    pub fn from_seed(name: &str, seed: u64) -> Self {
        // Simple LCG to derive palette parameters from the seed.
        let hue = lcg_f64(seed, 0) * 360.0;
        let scheme_idx = (lcg_f64(seed, 1) * 4.0) as u32 % 4;
        let scheme = match scheme_idx {
            0 => ColorScheme::Complementary,
            1 => ColorScheme::Triadic,
            2 => ColorScheme::Analogous,
            _ => ColorScheme::SplitComplementary,
        };
        // Vary saturation and lightness slightly per seed
        let sat_bias  = 0.6 + lcg_f64(seed, 2) * 0.3; // 0.6–0.9
        let lit_bias  = 0.45 + lcg_f64(seed, 3) * 0.15; // 0.45–0.60

        let h0 = hue;
        let h1 = match scheme {
            ColorScheme::Complementary      => (h0 + 180.0) % 360.0,
            ColorScheme::Triadic            => (h0 + 120.0) % 360.0,
            ColorScheme::Analogous          => (h0 + 30.0)  % 360.0,
            ColorScheme::SplitComplementary => (h0 + 150.0) % 360.0,
        };
        let h2 = match scheme {
            ColorScheme::Complementary      => (h0 + 30.0)  % 360.0,
            ColorScheme::Triadic            => (h0 + 240.0) % 360.0,
            ColorScheme::Analogous          => (h0 - 30.0 + 360.0) % 360.0,
            ColorScheme::SplitComplementary => (h0 + 210.0) % 360.0,
        };

        let mut p = Self::new(name);
        p.set(PaletteRole::Background, hsl_to_rgb(h0, 0.20,              0.08));
        p.set(PaletteRole::Primary,    hsl_to_rgb(h0, sat_bias,           lit_bias));
        p.set(PaletteRole::Secondary,  hsl_to_rgb(h1, sat_bias * 0.85,    lit_bias * 0.85));
        p.set(PaletteRole::Accent,     hsl_to_rgb(h2, sat_bias.min(0.95), lit_bias + 0.10));
        p.set(PaletteRole::Danger,     hsl_to_rgb(5.0 + (lcg_f64(seed, 4) * 15.0), 0.85, 0.55));
        p.set(PaletteRole::UiText,     hsl_to_rgb(h0, 0.10, 0.92));
        p.set(PaletteRole::UiBg,       hsl_to_rgb(h0, 0.25, 0.10).with_alpha(210));
        p
    }

    // ─── Built-in palettes ───────────────────────────────────────────────────

    /// Deep underground neon glow aesthetic.
    pub fn neon_cave() -> Self {
        let mut p = Self::new("neon_cave");
        p.set(PaletteRole::Background, Color::from_rgba(8,   4,   18,  255));
        p.set(PaletteRole::Primary,    Color::from_rgba(190, 0,   255, 255));
        p.set(PaletteRole::Secondary,  Color::from_rgba(0,   200, 255, 255));
        p.set(PaletteRole::Accent,     Color::from_rgba(255, 60,  180, 255));
        p.set(PaletteRole::Danger,     Color::from_rgba(255, 30,  60,  255));
        p.set(PaletteRole::UiText,     Color::from_rgba(230, 210, 255, 255));
        p.set(PaletteRole::UiBg,       Color::from_rgba(20,  10,  40,  200));
        p
    }

    /// Warm gradient twilight horizon.
    pub fn sunset() -> Self {
        let mut p = Self::new("sunset");
        p.set(PaletteRole::Background, Color::from_rgba(15,  5,   30,  255));
        p.set(PaletteRole::Primary,    Color::from_rgba(255, 100, 30,  255));
        p.set(PaletteRole::Secondary,  Color::from_rgba(255, 50,  100, 255));
        p.set(PaletteRole::Accent,     Color::from_rgba(255, 220, 60,  255));
        p.set(PaletteRole::Danger,     Color::from_rgba(220, 20,  20,  255));
        p.set(PaletteRole::UiText,     Color::from_rgba(255, 240, 220, 255));
        p.set(PaletteRole::UiBg,       Color::from_rgba(40,  15,  10,  200));
        p
    }

    /// Cold crystalline cavern.
    pub fn ice_dungeon() -> Self {
        let mut p = Self::new("ice_dungeon");
        p.set(PaletteRole::Background, Color::from_rgba(5,   10,  25,  255));
        p.set(PaletteRole::Primary,    Color::from_rgba(130, 200, 255, 255));
        p.set(PaletteRole::Secondary,  Color::from_rgba(60,  130, 210, 255));
        p.set(PaletteRole::Accent,     Color::from_rgba(200, 240, 255, 255));
        p.set(PaletteRole::Danger,     Color::from_rgba(200, 80,  80,  255));
        p.set(PaletteRole::UiText,     Color::from_rgba(210, 235, 255, 255));
        p.set(PaletteRole::UiBg,       Color::from_rgba(10,  20,  50,  200));
        p
    }

    /// Sickly green bioluminescent swamp.
    pub fn toxic_swamp() -> Self {
        let mut p = Self::new("toxic_swamp");
        p.set(PaletteRole::Background, Color::from_rgba(5,   15,  5,   255));
        p.set(PaletteRole::Primary,    Color::from_rgba(60,  200, 40,  255));
        p.set(PaletteRole::Secondary,  Color::from_rgba(30,  130, 60,  255));
        p.set(PaletteRole::Accent,     Color::from_rgba(180, 255, 30,  255));
        p.set(PaletteRole::Danger,     Color::from_rgba(200, 60,  20,  255));
        p.set(PaletteRole::UiText,     Color::from_rgba(200, 255, 190, 255));
        p.set(PaletteRole::UiBg,       Color::from_rgba(10,  25,  10,  200));
        p
    }

    /// Abyssal deep-sea bioluminescence.
    pub fn ocean_depths() -> Self {
        let mut p = Self::new("ocean_depths");
        p.set(PaletteRole::Background, Color::from_rgba(2,   5,   20,  255));
        p.set(PaletteRole::Primary,    Color::from_rgba(0,   100, 200, 255));
        p.set(PaletteRole::Secondary,  Color::from_rgba(0,   160, 180, 255));
        p.set(PaletteRole::Accent,     Color::from_rgba(60,  220, 240, 255));
        p.set(PaletteRole::Danger,     Color::from_rgba(200, 40,  80,  255));
        p.set(PaletteRole::UiText,     Color::from_rgba(180, 230, 255, 255));
        p.set(PaletteRole::UiBg,       Color::from_rgba(5,   10,  35,  200));
        p
    }

    /// Scorched lava flows and ember glow.
    pub fn volcanic() -> Self {
        let mut p = Self::new("volcanic");
        p.set(PaletteRole::Background, Color::from_rgba(10,  3,   2,   255));
        p.set(PaletteRole::Primary,    Color::from_rgba(220, 80,  20,  255));
        p.set(PaletteRole::Secondary,  Color::from_rgba(160, 40,  10,  255));
        p.set(PaletteRole::Accent,     Color::from_rgba(255, 200, 30,  255));
        p.set(PaletteRole::Danger,     Color::from_rgba(255, 30,  10,  255));
        p.set(PaletteRole::UiText,     Color::from_rgba(255, 220, 190, 255));
        p.set(PaletteRole::UiBg,       Color::from_rgba(25,  8,   5,   200));
        p
    }
}

// ─── HSL Utilities ────────────────────────────────────────────────────────────

/// Convert HSL (hue 0–360, saturation 0–1, lightness 0–1) to an opaque Color.
/// Uses the standard two-chroma-band formula with no external deps.
fn hsl_to_rgb(h: f64, s: f64, l: f64) -> Color {
    // Clamp inputs
    let h = ((h % 360.0) + 360.0) % 360.0;
    let s = s.max(0.0).min(1.0);
    let l = l.max(0.0).min(1.0);

    if s == 0.0 {
        // Achromatic
        let v = (l * 255.0).round() as u8;
        return Color::from_rgba(v, v, v, 255);
    }

    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;

    let r = hue_to_channel(p, q, h / 360.0 + 1.0 / 3.0);
    let g = hue_to_channel(p, q, h / 360.0);
    let b = hue_to_channel(p, q, h / 360.0 - 1.0 / 3.0);

    Color::from_rgba(
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
        255,
    )
}

/// Helper for the hue-to-RGB channel conversion used in `hsl_to_rgb`.
fn hue_to_channel(p: f64, q: f64, mut t: f64) -> f64 {
    if t < 0.0 { t += 1.0; }
    if t > 1.0 { t -= 1.0; }
    if t < 1.0 / 6.0 { return p + (q - p) * 6.0 * t; }
    if t < 1.0 / 2.0 { return q; }
    if t < 2.0 / 3.0 { return p + (q - p) * (2.0 / 3.0 - t) * 6.0; }
    p
}

/// Convert a Color to HSL (hue 0–360, saturation 0–1, lightness 0–1).
#[cfg(test)]
fn rgb_to_hsl(color: Color) -> (f64, f64, f64) {
    let r = color.r as f64 / 255.0;
    let g = color.g as f64 / 255.0;
    let b = color.b as f64 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if (max - min).abs() < 1e-10 {
        // Achromatic
        return (0.0, 0.0, l);
    }

    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };

    let h = if (max - r).abs() < 1e-10 {
        let seg = (g - b) / d + if g < b { 6.0 } else { 0.0 };
        seg / 6.0
    } else if (max - g).abs() < 1e-10 {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };

    (h * 360.0, s, l)
}

/// Deterministic LCG-based pseudo-random f64 in [0, 1) for palette generation.
/// `index` salts the seed to produce independent values per call site.
fn lcg_f64(seed: u64, index: u64) -> f64 {
    // LCG constants from Knuth (MMIX)
    const A: u64 = 6364136223846793005;
    const C: u64 = 1442695040888963407;
    let v = seed.wrapping_mul(A).wrapping_add(C).wrapping_add(index.wrapping_mul(2862933555777941757));
    // Mix further with a second LCG step
    let v = v.wrapping_mul(A).wrapping_add(C);
    // Map to [0, 1)
    (v >> 11) as f64 / (1u64 << 53) as f64
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── HSL round-trip ───────────────────────────────────────────────────────

    #[test]
    fn hsl_roundtrip_pure_red() {
        let original = Color::from_rgba(255, 0, 0, 255);
        let (h, s, l) = rgb_to_hsl(original);
        let recovered = hsl_to_rgb(h, s, l);
        assert_eq!(recovered.r, original.r);
        assert_eq!(recovered.g, original.g);
        assert_eq!(recovered.b, original.b);
    }

    #[test]
    fn hsl_roundtrip_mid_color() {
        // Mid-range color: allow ±2 per channel rounding tolerance.
        let original = Color::from_rgba(100, 150, 200, 255);
        let (h, s, l) = rgb_to_hsl(original);
        let recovered = hsl_to_rgb(h, s, l);
        let diff = |a: u8, b: u8| (a as i32 - b as i32).unsigned_abs();
        assert!(diff(recovered.r, original.r) <= 2, "r channel drift: {} vs {}", recovered.r, original.r);
        assert!(diff(recovered.g, original.g) <= 2, "g channel drift: {} vs {}", recovered.g, original.g);
        assert!(diff(recovered.b, original.b) <= 2, "b channel drift: {} vs {}", recovered.b, original.b);
    }

    #[test]
    fn hsl_roundtrip_achromatic_grey() {
        let original = Color::from_rgba(128, 128, 128, 255);
        let (h, s, l) = rgb_to_hsl(original);
        assert!(s.abs() < 1e-6, "grey should have zero saturation");
        let recovered = hsl_to_rgb(h, s, l);
        let diff = |a: u8, b: u8| (a as i32 - b as i32).unsigned_abs();
        assert!(diff(recovered.r, original.r) <= 1);
        assert!(diff(recovered.g, original.g) <= 1);
        assert!(diff(recovered.b, original.b) <= 1);
    }

    #[test]
    fn hsl_roundtrip_black_and_white() {
        for original in [Color::BLACK, Color::WHITE] {
            let (h, s, l) = rgb_to_hsl(original);
            let recovered = hsl_to_rgb(h, s, l);
            assert_eq!(recovered.r, original.r);
            assert_eq!(recovered.g, original.g);
            assert_eq!(recovered.b, original.b);
        }
    }

    // ── hsl_to_rgb edge cases ────────────────────────────────────────────────

    #[test]
    fn hsl_to_rgb_hue_wraps_360() {
        // hue 0 and hue 360 should produce the same red
        let a = hsl_to_rgb(0.0, 1.0, 0.5);
        let b = hsl_to_rgb(360.0, 1.0, 0.5);
        assert_eq!(a, b);
    }

    #[test]
    fn hsl_to_rgb_zero_saturation_is_grey() {
        let c = hsl_to_rgb(200.0, 0.0, 0.5);
        // All channels should be equal (grey)
        assert_eq!(c.r, c.g);
        assert_eq!(c.g, c.b);
    }

    // ── Seed determinism ─────────────────────────────────────────────────────

    #[test]
    fn seed_determinism_same_seed_same_palette() {
        let a = ColorPalette::from_seed("test", 42);
        let b = ColorPalette::from_seed("test", 42);
        for role in &ALL_ROLES {
            assert_eq!(
                a.colors[*role], b.colors[*role],
                "role '{}' differed between same-seed palettes", role
            );
        }
    }

    #[test]
    fn seed_determinism_different_seeds_differ() {
        let a = ColorPalette::from_seed("a", 1);
        let b = ColorPalette::from_seed("b", 999999);
        // At least one role should differ
        let any_diff = ALL_ROLES.iter().any(|r| a.colors[*r] != b.colors[*r]);
        assert!(any_diff, "different seeds produced identical palettes");
    }

    // ── All built-in palettes have all 7 roles ────────────────────────────────

    fn assert_all_roles_present(p: &ColorPalette, palette_name: &str) {
        for role in &ALL_ROLES {
            assert!(
                p.colors.contains_key(*role),
                "palette '{}' missing role '{}'", palette_name, role
            );
        }
        assert_eq!(p.colors.len(), 7, "palette '{}' should have exactly 7 roles", palette_name);
    }

    #[test]
    fn builtin_neon_cave_has_all_roles() {
        assert_all_roles_present(&ColorPalette::neon_cave(), "neon_cave");
    }

    #[test]
    fn builtin_sunset_has_all_roles() {
        assert_all_roles_present(&ColorPalette::sunset(), "sunset");
    }

    #[test]
    fn builtin_ice_dungeon_has_all_roles() {
        assert_all_roles_present(&ColorPalette::ice_dungeon(), "ice_dungeon");
    }

    #[test]
    fn builtin_toxic_swamp_has_all_roles() {
        assert_all_roles_present(&ColorPalette::toxic_swamp(), "toxic_swamp");
    }

    #[test]
    fn builtin_ocean_depths_has_all_roles() {
        assert_all_roles_present(&ColorPalette::ocean_depths(), "ocean_depths");
    }

    #[test]
    fn builtin_volcanic_has_all_roles() {
        assert_all_roles_present(&ColorPalette::volcanic(), "volcanic");
    }

    #[test]
    fn default_palette_has_all_roles() {
        assert_all_roles_present(&ColorPalette::default(), "default");
    }

    // ── from_hue produces distinct colours per scheme ────────────────────────

    fn count_distinct(p: &ColorPalette) -> usize {
        let mut seen: Vec<Color> = Vec::new();
        for role in &ALL_ROLES {
            let c = p.colors[*role];
            if !seen.contains(&c) {
                seen.push(c);
            }
        }
        seen.len()
    }

    #[test]
    fn from_hue_complementary_has_distinct_colors() {
        let p = ColorPalette::from_hue("c", 210.0, ColorScheme::Complementary);
        assert!(count_distinct(&p) >= 4, "complementary palette should have at least 4 distinct colors");
    }

    #[test]
    fn from_hue_triadic_has_distinct_colors() {
        let p = ColorPalette::from_hue("t", 30.0, ColorScheme::Triadic);
        assert!(count_distinct(&p) >= 4, "triadic palette should have at least 4 distinct colors");
    }

    #[test]
    fn from_hue_analogous_has_distinct_colors() {
        let p = ColorPalette::from_hue("a", 120.0, ColorScheme::Analogous);
        assert!(count_distinct(&p) >= 4, "analogous palette should have at least 4 distinct colors");
    }

    #[test]
    fn from_hue_split_complementary_has_distinct_colors() {
        let p = ColorPalette::from_hue("s", 60.0, ColorScheme::SplitComplementary);
        assert!(count_distinct(&p) >= 4, "split-complementary palette should have at least 4 distinct colors");
    }

    // ── get / set ────────────────────────────────────────────────────────────

    #[test]
    fn get_returns_set_color() {
        let mut p = ColorPalette::new("g");
        p.set(PaletteRole::Primary, Color::RED);
        assert_eq!(p.get(PaletteRole::Primary), Color::RED);
    }

    #[test]
    fn set_overwrites_previous_value() {
        let mut p = ColorPalette::new("o");
        p.set(PaletteRole::Accent, Color::BLUE);
        p.set(PaletteRole::Accent, Color::GREEN);
        assert_eq!(p.get(PaletteRole::Accent), Color::GREEN);
    }

    #[test]
    fn get_by_name_returns_correct_color() {
        let mut p = ColorPalette::new("n");
        p.set(PaletteRole::Danger, Color::RED);
        assert_eq!(p.get_by_name("danger"), Some(Color::RED));
    }

    #[test]
    fn get_by_name_unknown_key_returns_none() {
        let p = ColorPalette::new("x");
        assert_eq!(p.get_by_name("nonexistent_key"), None);
    }

    #[test]
    fn set_by_name_updates_existing_role() {
        let mut p = ColorPalette::new("y");
        p.set_by_name("primary", Color::WHITE);
        assert_eq!(p.get(PaletteRole::Primary), Color::WHITE);
    }

    // ── from_hue has all 7 roles ─────────────────────────────────────────────

    #[test]
    fn from_hue_has_all_roles() {
        let p = ColorPalette::from_hue("h", 180.0, ColorScheme::Complementary);
        assert_all_roles_present(&p, "from_hue");
    }

    // ── from_seed has all 7 roles ────────────────────────────────────────────

    #[test]
    fn from_seed_has_all_roles() {
        let p = ColorPalette::from_seed("s", 12345);
        assert_all_roles_present(&p, "from_seed");
    }
}
