/// RGBA color. Used throughout the engine for rendering and configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Parse "#RRGGBB" or "#RRGGBBAA". Strips leading '#' if present.
    pub fn from_hex(s: &str) -> Option<Self> {
        let s = s.strip_prefix('#').unwrap_or(s);
        let len = s.len();
        if len != 6 && len != 8 {
            return None;
        }
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        let a = if len == 8 {
            u8::from_str_radix(&s[6..8], 16).ok()?
        } else {
            255
        };
        Some(Self { r, g, b, a })
    }

    pub fn with_alpha(self, a: u8) -> Self {
        Self { a, ..self }
    }

    /// Linear interpolation between two colors. t=0 returns a, t=1 returns b.
    pub fn lerp(a: Color, b: Color, t: f64) -> Color {
        let t = t.max(0.0).min(1.0);
        let inv = 1.0 - t;
        Color {
            r: (a.r as f64 * inv + b.r as f64 * t).round() as u8,
            g: (a.g as f64 * inv + b.g as f64 * t).round() as u8,
            b: (a.b as f64 * inv + b.b as f64 * t).round() as u8,
            a: (a.a as f64 * inv + b.a as f64 * t).round() as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Constants ---

    #[test]
    fn constant_black() {
        assert_eq!(Color::BLACK, Color { r: 0, g: 0, b: 0, a: 255 });
    }

    #[test]
    fn constant_white() {
        assert_eq!(Color::WHITE, Color { r: 255, g: 255, b: 255, a: 255 });
    }

    #[test]
    fn constant_red() {
        assert_eq!(Color::RED, Color { r: 255, g: 0, b: 0, a: 255 });
    }

    #[test]
    fn constant_green() {
        assert_eq!(Color::GREEN, Color { r: 0, g: 255, b: 0, a: 255 });
    }

    #[test]
    fn constant_blue() {
        assert_eq!(Color::BLUE, Color { r: 0, g: 0, b: 255, a: 255 });
    }

    #[test]
    fn constant_transparent() {
        assert_eq!(Color::TRANSPARENT, Color { r: 0, g: 0, b: 0, a: 0 });
    }

    // --- from_rgba ---

    #[test]
    fn from_rgba_constructs_correctly() {
        let c = Color::from_rgba(10, 20, 30, 40);
        assert_eq!(c.r, 10);
        assert_eq!(c.g, 20);
        assert_eq!(c.b, 30);
        assert_eq!(c.a, 40);
    }

    #[test]
    fn from_rgba_boundary_values() {
        let c = Color::from_rgba(0, 0, 0, 0);
        assert_eq!(c, Color { r: 0, g: 0, b: 0, a: 0 });

        let c = Color::from_rgba(255, 255, 255, 255);
        assert_eq!(c, Color { r: 255, g: 255, b: 255, a: 255 });
    }

    // --- from_hex with "#RRGGBB" (6-char) ---

    #[test]
    fn from_hex_6_char_with_hash() {
        let c = Color::from_hex("#FF8040").unwrap();
        assert_eq!(c.r, 0xFF);
        assert_eq!(c.g, 0x80);
        assert_eq!(c.b, 0x40);
        assert_eq!(c.a, 255); // default alpha
    }

    #[test]
    fn from_hex_6_char_black() {
        let c = Color::from_hex("#000000").unwrap();
        assert_eq!(c, Color { r: 0, g: 0, b: 0, a: 255 });
    }

    #[test]
    fn from_hex_6_char_white() {
        let c = Color::from_hex("#FFFFFF").unwrap();
        assert_eq!(c, Color { r: 255, g: 255, b: 255, a: 255 });
    }

    // --- from_hex with "#RRGGBBAA" (8-char) ---

    #[test]
    fn from_hex_8_char_with_hash() {
        let c = Color::from_hex("#FF804080").unwrap();
        assert_eq!(c.r, 0xFF);
        assert_eq!(c.g, 0x80);
        assert_eq!(c.b, 0x40);
        assert_eq!(c.a, 0x80);
    }

    #[test]
    fn from_hex_8_char_fully_transparent() {
        let c = Color::from_hex("#00000000").unwrap();
        assert_eq!(c, Color { r: 0, g: 0, b: 0, a: 0 });
    }

    // --- from_hex without # prefix ---

    #[test]
    fn from_hex_no_hash_6_char() {
        let c = Color::from_hex("AB12CD").unwrap();
        assert_eq!(c.r, 0xAB);
        assert_eq!(c.g, 0x12);
        assert_eq!(c.b, 0xCD);
        assert_eq!(c.a, 255);
    }

    #[test]
    fn from_hex_no_hash_8_char() {
        let c = Color::from_hex("AB12CD7F").unwrap();
        assert_eq!(c.r, 0xAB);
        assert_eq!(c.g, 0x12);
        assert_eq!(c.b, 0xCD);
        assert_eq!(c.a, 0x7F);
    }

    // --- from_hex with invalid input ---

    #[test]
    fn from_hex_too_short() {
        assert!(Color::from_hex("#FFF").is_none());
        assert!(Color::from_hex("#FFFF").is_none());
        assert!(Color::from_hex("#FFFFF").is_none());
    }

    #[test]
    fn from_hex_too_long() {
        assert!(Color::from_hex("#FFFFFFFFF").is_none());
    }

    #[test]
    fn from_hex_invalid_hex_chars() {
        assert!(Color::from_hex("#GGHHII").is_none());
        assert!(Color::from_hex("#ZZZZZZ").is_none());
    }

    #[test]
    fn from_hex_empty_string() {
        assert!(Color::from_hex("").is_none());
        assert!(Color::from_hex("#").is_none());
    }

    // --- with_alpha ---

    #[test]
    fn with_alpha_modifies_only_alpha() {
        let c = Color::from_rgba(10, 20, 30, 255);
        let c2 = c.with_alpha(100);
        assert_eq!(c2.r, 10);
        assert_eq!(c2.g, 20);
        assert_eq!(c2.b, 30);
        assert_eq!(c2.a, 100);
    }

    #[test]
    fn with_alpha_does_not_mutate_original() {
        let c = Color::from_rgba(10, 20, 30, 255);
        let _c2 = c.with_alpha(0);
        assert_eq!(c.a, 255); // original unchanged (Copy semantics)
    }

    // --- PartialEq ---

    #[test]
    fn partial_eq_same_colors() {
        let a = Color::from_rgba(1, 2, 3, 4);
        let b = Color::from_rgba(1, 2, 3, 4);
        assert_eq!(a, b);
    }

    #[test]
    fn partial_eq_different_colors() {
        let a = Color::from_rgba(1, 2, 3, 4);
        let b = Color::from_rgba(1, 2, 3, 5);
        assert_ne!(a, b);
    }

    #[test]
    fn partial_eq_all_channels_matter() {
        let base = Color::from_rgba(10, 20, 30, 40);
        assert_ne!(base, Color::from_rgba(11, 20, 30, 40)); // r differs
        assert_ne!(base, Color::from_rgba(10, 21, 30, 40)); // g differs
        assert_ne!(base, Color::from_rgba(10, 20, 31, 40)); // b differs
        assert_ne!(base, Color::from_rgba(10, 20, 30, 41)); // a differs
    }

    // --- Clone / Copy ---

    #[test]
    fn color_is_copy() {
        let c = Color::from_rgba(1, 2, 3, 4);
        let c2 = c; // Copy
        assert_eq!(c, c2); // original still usable
    }

    #[test]
    fn from_hex_lowercase() {
        let c = Color::from_hex("#ff8040").unwrap();
        assert_eq!(c.r, 0xFF);
        assert_eq!(c.g, 0x80);
        assert_eq!(c.b, 0x40);
    }
}
