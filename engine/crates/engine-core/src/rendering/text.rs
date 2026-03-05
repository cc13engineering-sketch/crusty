use super::framebuffer::Framebuffer;
use super::color::Color;

/// 5x7 bitmap font. Each char = 7 bytes (rows), low 5 bits = pixel columns (MSB=left).
/// Covers ASCII 32 (space) through 126 (~).
const FONT_DATA: [u8; 665] = [
    // 32: space
    0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
    // 33: !
    0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00100,
    // 34: "
    0b01010, 0b01010, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
    // 35: #
    0b01010, 0b11111, 0b01010, 0b01010, 0b11111, 0b01010, 0b00000,
    // 36: $
    0b00100, 0b01111, 0b10100, 0b01110, 0b00101, 0b11110, 0b00100,
    // 37: %
    0b11001, 0b11010, 0b00100, 0b00100, 0b01011, 0b10011, 0b00000,
    // 38: &
    0b01100, 0b10010, 0b01100, 0b10110, 0b10001, 0b10001, 0b01110,
    // 39: '
    0b00100, 0b00100, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
    // 40: (
    0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00100, 0b00010,
    // 41: )
    0b01000, 0b00100, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000,
    // 42: *
    0b00000, 0b00100, 0b10101, 0b01110, 0b10101, 0b00100, 0b00000,
    // 43: +
    0b00000, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000,
    // 44: ,
    0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00100, 0b01000,
    // 45: -
    0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000,
    // 46: .
    0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00100,
    // 47: /
    0b00001, 0b00010, 0b00100, 0b00100, 0b01000, 0b10000, 0b00000,
    // 48: 0
    0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
    // 49: 1
    0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
    // 50: 2
    0b01110, 0b10001, 0b00001, 0b00110, 0b01000, 0b10000, 0b11111,
    // 51: 3
    0b01110, 0b10001, 0b00001, 0b00110, 0b00001, 0b10001, 0b01110,
    // 52: 4
    0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
    // 53: 5
    0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110,
    // 54: 6
    0b01110, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
    // 55: 7
    0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
    // 56: 8
    0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
    // 57: 9
    0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110,
    // 58: :
    0b00000, 0b00000, 0b00100, 0b00000, 0b00100, 0b00000, 0b00000,
    // 59: ;
    0b00000, 0b00000, 0b00100, 0b00000, 0b00100, 0b00100, 0b01000,
    // 60: <
    0b00010, 0b00100, 0b01000, 0b10000, 0b01000, 0b00100, 0b00010,
    // 61: =
    0b00000, 0b00000, 0b11111, 0b00000, 0b11111, 0b00000, 0b00000,
    // 62: >
    0b10000, 0b01000, 0b00100, 0b00010, 0b00100, 0b01000, 0b10000,
    // 63: ?
    0b01110, 0b10001, 0b00010, 0b00100, 0b00100, 0b00000, 0b00100,
    // 64: @
    0b01110, 0b10001, 0b10111, 0b10101, 0b10110, 0b10000, 0b01110,
    // 65: A
    0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
    // 66: B
    0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
    // 67: C
    0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
    // 68: D
    0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
    // 69: E
    0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
    // 70: F
    0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
    // 71: G
    0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110,
    // 72: H
    0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
    // 73: I
    0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
    // 74: J
    0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100,
    // 75: K
    0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
    // 76: L
    0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
    // 77: M
    0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
    // 78: N
    0b10001, 0b11001, 0b10101, 0b10101, 0b10011, 0b10001, 0b10001,
    // 79: O
    0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
    // 80: P
    0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
    // 81: Q
    0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
    // 82: R
    0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
    // 83: S
    0b01110, 0b10001, 0b10000, 0b01110, 0b00001, 0b10001, 0b01110,
    // 84: T
    0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
    // 85: U
    0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
    // 86: V
    0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
    // 87: W
    0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001,
    // 88: X
    0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
    // 89: Y
    0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
    // 90: Z
    0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
    // 91: [
    0b01110, 0b01000, 0b01000, 0b01000, 0b01000, 0b01000, 0b01110,
    // 92: backslash
    0b10000, 0b01000, 0b00100, 0b00100, 0b00010, 0b00001, 0b00000,
    // 93: ]
    0b01110, 0b00010, 0b00010, 0b00010, 0b00010, 0b00010, 0b01110,
    // 94: ^
    0b00100, 0b01010, 0b10001, 0b00000, 0b00000, 0b00000, 0b00000,
    // 95: _
    0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111,
    // 96: `
    0b01000, 0b00100, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
    // 97: a
    0b00000, 0b00000, 0b01110, 0b00001, 0b01111, 0b10001, 0b01111,
    // 98: b
    0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b10001, 0b11110,
    // 99: c
    0b00000, 0b00000, 0b01110, 0b10000, 0b10000, 0b10001, 0b01110,
    // 100: d
    0b00001, 0b00001, 0b01111, 0b10001, 0b10001, 0b10001, 0b01111,
    // 101: e
    0b00000, 0b00000, 0b01110, 0b10001, 0b11111, 0b10000, 0b01110,
    // 102: f
    0b00110, 0b01001, 0b01000, 0b11110, 0b01000, 0b01000, 0b01000,
    // 103: g
    0b00000, 0b00000, 0b01111, 0b10001, 0b01111, 0b00001, 0b01110,
    // 104: h
    0b10000, 0b10000, 0b10110, 0b11001, 0b10001, 0b10001, 0b10001,
    // 105: i
    0b00100, 0b00000, 0b01100, 0b00100, 0b00100, 0b00100, 0b01110,
    // 106: j
    0b00010, 0b00000, 0b00110, 0b00010, 0b00010, 0b10010, 0b01100,
    // 107: k
    0b10000, 0b10000, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010,
    // 108: l
    0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
    // 109: m
    0b00000, 0b00000, 0b11010, 0b10101, 0b10101, 0b10001, 0b10001,
    // 110: n
    0b00000, 0b00000, 0b10110, 0b11001, 0b10001, 0b10001, 0b10001,
    // 111: o
    0b00000, 0b00000, 0b01110, 0b10001, 0b10001, 0b10001, 0b01110,
    // 112: p
    0b00000, 0b00000, 0b11110, 0b10001, 0b11110, 0b10000, 0b10000,
    // 113: q
    0b00000, 0b00000, 0b01111, 0b10001, 0b01111, 0b00001, 0b00001,
    // 114: r
    0b00000, 0b00000, 0b10110, 0b11001, 0b10000, 0b10000, 0b10000,
    // 115: s
    0b00000, 0b00000, 0b01111, 0b10000, 0b01110, 0b00001, 0b11110,
    // 116: t
    0b01000, 0b01000, 0b11110, 0b01000, 0b01000, 0b01001, 0b00110,
    // 117: u
    0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b10011, 0b01101,
    // 118: v
    0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
    // 119: w
    0b00000, 0b00000, 0b10001, 0b10001, 0b10101, 0b10101, 0b01010,
    // 120: x
    0b00000, 0b00000, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001,
    // 121: y
    0b00000, 0b00000, 0b10001, 0b10001, 0b01111, 0b00001, 0b01110,
    // 122: z
    0b00000, 0b00000, 0b11111, 0b00010, 0b00100, 0b01000, 0b11111,
    // 123: {
    0b00010, 0b00100, 0b00100, 0b01000, 0b00100, 0b00100, 0b00010,
    // 124: |
    0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
    // 125: }
    0b01000, 0b00100, 0b00100, 0b00010, 0b00100, 0b00100, 0b01000,
    // 126: ~
    0b00000, 0b00000, 0b01000, 0b10101, 0b00010, 0b00000, 0b00000,
];

const CHAR_W: u32 = 5;
const CHAR_H: u32 = 7;
const CHAR_SPACING: u32 = 1;

/// Check if a font pixel is set at (row, col) within a glyph at the given offset.
fn font_bit(offset: usize, row: u32, col: u32) -> bool {
    if row >= CHAR_H || col >= CHAR_W { return false; }
    FONT_DATA[offset + row as usize] & (1 << (CHAR_W - 1 - col)) != 0
}

pub fn draw_char(fb: &mut Framebuffer, x: i32, y: i32, c: char, color: Color, scale: u32) {
    let idx = c as usize;
    if idx < 32 || idx > 126 { return; }
    let offset = (idx - 32) * CHAR_H as usize;

    // Anti-aliased fringe for scale >= 2: 1px soft edge around glyph outlines
    if scale >= 2 {
        let edge_a = (color.a as u32 * 2 / 5).min(255) as u8; // ~40%
        let edge = Color { r: color.r, g: color.g, b: color.b, a: edge_a };
        let corner_a = (color.a as u32 / 5).min(255) as u8; // ~20%
        let corner = Color { r: color.r, g: color.g, b: color.b, a: corner_a };
        for row in 0..CHAR_H {
            for col in 0..CHAR_W {
                if !font_bit(offset, row, col) { continue; }
                let bx = x + (col * scale) as i32;
                let by = y + (row * scale) as i32;
                let s = scale as i32;
                let has_l = font_bit(offset, row, col.wrapping_sub(1));
                let has_r = font_bit(offset, row, col + 1);
                let has_u = font_bit(offset, row.wrapping_sub(1), col);
                let has_d = font_bit(offset, row + 1, col);
                // Cardinal edges
                if !has_l {
                    for sy in 0..s { fb.set_pixel_blended(bx - 1, by + sy, edge); }
                }
                if !has_r {
                    for sy in 0..s { fb.set_pixel_blended(bx + s, by + sy, edge); }
                }
                if !has_u {
                    for sx in 0..s { fb.set_pixel_blended(bx + sx, by - 1, edge); }
                }
                if !has_d {
                    for sx in 0..s { fb.set_pixel_blended(bx + sx, by + s, edge); }
                }
                // Diagonal corners (where both cardinal neighbors are off)
                if !has_l && !has_u { fb.set_pixel_blended(bx - 1, by - 1, corner); }
                if !has_r && !has_u { fb.set_pixel_blended(bx + s, by - 1, corner); }
                if !has_l && !has_d { fb.set_pixel_blended(bx - 1, by + s, corner); }
                if !has_r && !has_d { fb.set_pixel_blended(bx + s, by + s, corner); }
            }
        }
    }

    // Main solid pixels
    for row in 0..CHAR_H {
        let bits = FONT_DATA[offset + row as usize];
        for col in 0..CHAR_W {
            if bits & (1 << (CHAR_W - 1 - col)) != 0 {
                for sy in 0..scale {
                    for sx in 0..scale {
                        let px = x + (col * scale + sx) as i32;
                        let py = y + (row * scale + sy) as i32;
                        if color.a == 255 {
                            fb.set_pixel(px, py, color);
                        } else {
                            fb.set_pixel_blended(px, py, color);
                        }
                    }
                }
            }
        }
    }
}

pub fn draw_text(fb: &mut Framebuffer, x: i32, y: i32, text: &str, color: Color, scale: u32) {
    let scale = scale.max(1);
    let advance = (CHAR_W + CHAR_SPACING) * scale;
    for (i, c) in text.chars().enumerate() {
        draw_char(fb, x + (i as u32 * advance) as i32, y, c, color, scale);
    }
}

pub fn text_width(text: &str, scale: u32) -> i32 {
    let scale = scale.max(1) as i32;
    let n = text.len() as i32;
    if n == 0 { return 0; }
    n * CHAR_W as i32 * scale + (n - 1) * CHAR_SPACING as i32 * scale
}

pub fn draw_text_centered(fb: &mut Framebuffer, cx: i32, cy: i32, text: &str, color: Color, scale: u32) {
    let w = text_width(text, scale);
    let h = CHAR_H as i32 * scale.max(1) as i32;
    draw_text(fb, cx - w / 2, cy - h / 2, text, color, scale);
}
