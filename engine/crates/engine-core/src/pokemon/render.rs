// AI-INSTRUCTIONS: Pokemon rendering utilities. Provides scaled sprite drawing, tile rendering,
// text rendering, UI elements (HP bars, menu boxes, dialogue boxes), and palette management.
// All rendering targets the engine Framebuffer. Uses a virtual GBC resolution (160x144) scaled
// to fill the actual framebuffer.

use crate::rendering::framebuffer::Framebuffer;
use crate::rendering::color::Color;
use crate::rendering::shapes;

// ─── GBC Color Palettes ─────────────────────────────────

/// 4-color palette (indexed 0-3, where 0 is transparent)
pub type Palette = [Color; 4];

/// Standard overworld palette (GBC green tones)
pub const PAL_OVERWORLD: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },         // 0: transparent
    Color { r: 200, g: 232, b: 152, a: 255 },  // 1: light green
    Color { r: 104, g: 168, b: 56, a: 255 },   // 2: medium green
    Color { r: 24, g: 64, b: 32, a: 255 },     // 3: dark green
];

/// Path/ground palette (brown tones)
pub const PAL_PATH: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 232, g: 208, b: 168, a: 255 },  // light tan
    Color { r: 192, g: 160, b: 112, a: 255 },  // medium brown
    Color { r: 104, g: 80, b: 48, a: 255 },    // dark brown
];

/// Building palette (gray tones)
pub const PAL_BUILDING: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 224, g: 224, b: 232, a: 255 },  // light gray
    Color { r: 160, g: 160, b: 176, a: 255 },  // medium gray
    Color { r: 64, g: 64, b: 80, a: 255 },     // dark gray
];

/// Water palette (blue tones)
pub const PAL_WATER: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 152, g: 200, b: 248, a: 255 },  // light blue
    Color { r: 72, g: 136, b: 232, a: 255 },   // medium blue
    Color { r: 24, g: 56, b: 152, a: 255 },    // dark blue
];

/// Pokemon Center palette (red/white)
pub const PAL_POKECENTER: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 248, g: 240, b: 240, a: 255 },  // white/cream
    Color { r: 240, g: 88, b: 72, a: 255 },    // red
    Color { r: 128, g: 32, b: 32, a: 255 },    // dark red
];

/// Lab palette
pub const PAL_LAB: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 232, g: 232, b: 248, a: 255 },  // light blue-white
    Color { r: 144, g: 152, b: 184, a: 255 },  // blue-gray
    Color { r: 56, g: 56, b: 88, a: 255 },     // dark blue-gray
];

/// Player sprite palette (red/blue outfit)
pub const PAL_PLAYER: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 248, g: 224, b: 176, a: 255 },  // skin tone
    Color { r: 200, g: 48, b: 48, a: 255 },    // red (jacket/hat)
    Color { r: 32, g: 32, b: 48, a: 255 },     // dark (hair/outline)
];

/// NPC palette - Professor Elm
pub const PAL_ELM: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 248, g: 248, b: 248, a: 255 },  // white (lab coat)
    Color { r: 160, g: 136, b: 96, a: 255 },   // brown (hair)
    Color { r: 40, g: 40, b: 56, a: 255 },     // dark
];

/// NPC palette - Mom
pub const PAL_MOM: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 248, g: 208, b: 176, a: 255 },  // skin
    Color { r: 208, g: 96, b: 128, a: 255 },   // pink (dress)
    Color { r: 80, g: 40, b: 56, a: 255 },     // dark
];

/// NPC palette - Nurse Joy
pub const PAL_NURSE: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 248, g: 200, b: 208, a: 255 },  // pink (hair)
    Color { r: 248, g: 248, b: 248, a: 255 },  // white (uniform)
    Color { r: 56, g: 56, b: 72, a: 255 },     // dark
];

/// NPC palette - generic youngster/lass
pub const PAL_NPC_GENERIC: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 248, g: 216, b: 176, a: 255 },  // skin
    Color { r: 80, g: 128, b: 200, a: 255 },   // blue (shirt)
    Color { r: 40, g: 48, b: 64, a: 255 },     // dark
];

/// Interior floor palette
pub const PAL_FLOOR: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 240, g: 224, b: 192, a: 255 },  // light wood
    Color { r: 200, g: 176, b: 136, a: 255 },  // medium wood
    Color { r: 120, g: 96, b: 64, a: 255 },    // dark wood
];

/// Interior furniture palette
pub const PAL_FURNITURE: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 216, g: 200, b: 168, a: 255 },
    Color { r: 152, g: 128, b: 88, a: 255 },
    Color { r: 72, g: 56, b: 40, a: 255 },
];

/// Flower palette
pub const PAL_FLOWER: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 248, g: 248, b: 120, a: 255 },  // yellow flower
    Color { r: 200, g: 232, b: 152, a: 255 },  // light green (grass)
    Color { r: 104, g: 168, b: 56, a: 255 },   // dark green
];

/// Sign palette
pub const PAL_SIGN: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 216, g: 192, b: 144, a: 255 },  // wood
    Color { r: 200, g: 232, b: 152, a: 255 },  // grass bg
    Color { r: 80, g: 64, b: 40, a: 255 },     // dark wood
];

/// Battle UI palette (dark themed)
pub const PAL_BATTLE_BG: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 248, g: 248, b: 248, a: 255 },  // white
    Color { r: 168, g: 168, b: 176, a: 255 },  // gray
    Color { r: 48, g: 48, b: 56, a: 255 },     // dark
];

/// Fence palette
pub const PAL_FENCE: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 232, g: 216, b: 176, a: 255 },  // light wood
    Color { r: 200, g: 232, b: 152, a: 255 },  // grass
    Color { r: 120, g: 96, b: 56, a: 255 },    // dark wood
];

/// Ledge palette
pub const PAL_LEDGE: Palette = [
    Color { r: 0, g: 0, b: 0, a: 0 },
    Color { r: 200, g: 232, b: 152, a: 255 },  // light green
    Color { r: 136, g: 184, b: 88, a: 255 },   // medium green
    Color { r: 72, g: 112, b: 40, a: 255 },    // dark green
];

/// Black tile palette (borders)
pub const PAL_BLACK: Palette = [
    Color { r: 8, g: 8, b: 16, a: 255 },
    Color { r: 8, g: 8, b: 16, a: 255 },
    Color { r: 8, g: 8, b: 16, a: 255 },
    Color { r: 8, g: 8, b: 16, a: 255 },
];

// ─── Rendering State ────────────────────────────────────

/// Holds the computed scale and offset for rendering
#[derive(Clone)]
pub struct RenderContext {
    pub scale: u32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub fb_width: u32,
    pub fb_height: u32,
    /// Virtual screen dimensions in GBC pixels
    pub virt_w: u32,
    pub virt_h: u32,
}

impl RenderContext {
    pub fn new(fb_width: u32, fb_height: u32) -> Self {
        let virt_w: u32 = 160;
        let virt_h: u32 = 144;
        let scale_x = fb_width / virt_w;
        let scale_y = fb_height / virt_h;
        let scale = scale_x.min(scale_y).max(1);
        let offset_x = ((fb_width - virt_w * scale) / 2) as i32;
        let offset_y = ((fb_height - virt_h * scale) / 2) as i32;
        RenderContext { scale, offset_x, offset_y, fb_width, fb_height, virt_w, virt_h }
    }

    /// Convert virtual pixel coordinates to framebuffer coordinates
    pub fn to_fb(&self, vx: i32, vy: i32) -> (i32, i32) {
        (
            self.offset_x + vx * self.scale as i32,
            self.offset_y + vy * self.scale as i32,
        )
    }
}

// ─── Sprite Decoding ────────────────────────────────────

/// Decode a sprite string (chars '0'-'3') into a Vec<u8>
pub fn decode_sprite(data: &str) -> Vec<u8> {
    data.bytes()
        .filter(|b| *b >= b'0' && *b <= b'3')
        .map(|b| b - b'0')
        .collect()
}

// ─── Drawing Functions ──────────────────────────────────

/// Draw a scaled indexed sprite onto the framebuffer.
/// sprite_data: decoded pixel indices (0-3)
/// w, h: sprite dimensions in pixels
/// dx, dy: destination in FRAMEBUFFER coordinates (not virtual)
/// scale: pixels per sprite pixel
/// palette: color lookup
pub fn draw_sprite(
    fb: &mut Framebuffer,
    sprite_data: &[u8],
    w: usize,
    h: usize,
    dx: i32,
    dy: i32,
    scale: u32,
    palette: &Palette,
) {
    let s = scale as i32;
    let fbw = fb.width as i32;
    let fbh = fb.height as i32;

    for sy in 0..h {
        let py = dy + sy as i32 * s;
        if py + s <= 0 || py >= fbh { continue; }
        for sx in 0..w {
            let px = dx + sx as i32 * s;
            if px + s <= 0 || px >= fbw { continue; }
            let idx = sy * w + sx;
            if idx >= sprite_data.len() { continue; }
            let ci = sprite_data[idx];
            if ci == 0 { continue; } // transparent
            let color = palette[ci as usize & 3];
            if color.a == 0 { continue; }
            // Fill the scaled pixel block
            for fy in 0..s {
                let row = py + fy;
                if row < 0 || row >= fbh { continue; }
                for fx in 0..s {
                    let col = px + fx;
                    if col < 0 || col >= fbw { continue; }
                    fb.set_pixel(col, row, color);
                }
            }
        }
    }
}

/// Draw a sprite from a string constant (decodes then draws)
pub fn draw_sprite_str(
    fb: &mut Framebuffer,
    sprite_str: &str,
    w: usize,
    h: usize,
    dx: i32,
    dy: i32,
    scale: u32,
    palette: &Palette,
) {
    let data = decode_sprite(sprite_str);
    draw_sprite(fb, &data, w, h, dx, dy, scale, palette);
}

/// Get the palette for a tile ID
pub fn tile_palette(tile_id: u8) -> &'static Palette {
    match tile_id {
        0 => &PAL_OVERWORLD,      // grass
        1 => &PAL_OVERWORLD,      // tall grass (darker variant handled by sprite)
        2 => &PAL_PATH,           // path
        3 => &PAL_OVERWORLD,      // tree top
        4 => &PAL_OVERWORLD,      // tree bottom
        5 | 6 => &PAL_WATER,     // water
        7 => &PAL_BUILDING,       // building wall
        8 => &PAL_BUILDING,       // building roof
        9 => &PAL_BUILDING,       // door
        10 => &PAL_FENCE,         // fence
        11 => &PAL_FLOWER,        // flower
        12 => &PAL_POKECENTER,    // pokecenter roof
        13 => &PAL_POKECENTER,    // pokecenter wall
        14 => &PAL_POKECENTER,    // pokecenter door
        15 => &PAL_LAB,           // lab wall
        16 => &PAL_LAB,           // lab roof
        17 => &PAL_SIGN,          // sign
        18 => &PAL_LEDGE,         // ledge
        19 => &PAL_FLOOR,         // floor
        20 => &PAL_FURNITURE,     // table
        21 => &PAL_FURNITURE,     // bookshelf
        22 => &PAL_FURNITURE,     // PC
        23 => &PAL_POKECENTER,    // heal machine
        24 => &PAL_BLACK,         // black
        _ => &PAL_OVERWORLD,
    }
}

/// Get the palette for an NPC sprite ID
pub fn npc_palette(sprite_id: u8) -> &'static Palette {
    match sprite_id {
        0 => &PAL_ELM,
        1 => &PAL_MOM,
        2 => &PAL_NPC_GENERIC,
        3 => &PAL_NPC_GENERIC,
        4 => &PAL_NURSE,
        5 => &PAL_NPC_GENERIC,
        _ => &PAL_NPC_GENERIC,
    }
}

// ─── Text Rendering ─────────────────────────────────────

/// Pokemon-style bitmap font (5x7 per character, uppercase + digits + punctuation)
/// Returns glyph data for a character as a &[u8] (35 bytes, 5 wide x 7 tall)
fn get_glyph(ch: char) -> Option<[u8; 35]> {
    let ch = ch.to_ascii_uppercase();
    // Simple bitmap font definitions - each is 5 columns x 7 rows
    Some(match ch {
        'A' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 1,1,1,1,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1],
        'B' => [1,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 1,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 1,1,1,1,0],
        'C' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,1, 0,1,1,1,0],
        'D' => [1,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,1,1,1,0],
        'E' => [1,1,1,1,1, 1,0,0,0,0, 1,0,0,0,0, 1,1,1,1,0, 1,0,0,0,0, 1,0,0,0,0, 1,1,1,1,1],
        'F' => [1,1,1,1,1, 1,0,0,0,0, 1,0,0,0,0, 1,1,1,1,0, 1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,0],
        'G' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,0, 1,0,1,1,1, 1,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        'H' => [1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,1,1,1,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1],
        'I' => [0,1,1,1,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,1,1,1,0],
        'J' => [0,0,1,1,1, 0,0,0,1,0, 0,0,0,1,0, 0,0,0,1,0, 0,0,0,1,0, 1,0,0,1,0, 0,1,1,0,0],
        'K' => [1,0,0,0,1, 1,0,0,1,0, 1,0,1,0,0, 1,1,0,0,0, 1,0,1,0,0, 1,0,0,1,0, 1,0,0,0,1],
        'L' => [1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,0, 1,1,1,1,1],
        'M' => [1,0,0,0,1, 1,1,0,1,1, 1,0,1,0,1, 1,0,1,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1],
        'N' => [1,0,0,0,1, 1,1,0,0,1, 1,0,1,0,1, 1,0,0,1,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1],
        'O' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        'P' => [1,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 1,1,1,1,0, 1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,0],
        'Q' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,1,0,1, 1,0,0,1,0, 0,1,1,0,1],
        'R' => [1,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 1,1,1,1,0, 1,0,1,0,0, 1,0,0,1,0, 1,0,0,0,1],
        'S' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,0, 0,1,1,1,0, 0,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        'T' => [1,1,1,1,1, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0],
        'U' => [1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        'V' => [1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 0,1,0,1,0, 0,1,0,1,0, 0,0,1,0,0],
        'W' => [1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,1,0,1, 1,0,1,0,1, 1,1,0,1,1, 1,0,0,0,1],
        'X' => [1,0,0,0,1, 1,0,0,0,1, 0,1,0,1,0, 0,0,1,0,0, 0,1,0,1,0, 1,0,0,0,1, 1,0,0,0,1],
        'Y' => [1,0,0,0,1, 1,0,0,0,1, 0,1,0,1,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0],
        'Z' => [1,1,1,1,1, 0,0,0,0,1, 0,0,0,1,0, 0,0,1,0,0, 0,1,0,0,0, 1,0,0,0,0, 1,1,1,1,1],
        '0' => [0,1,1,1,0, 1,0,0,1,1, 1,0,1,0,1, 1,0,1,0,1, 1,0,1,0,1, 1,1,0,0,1, 0,1,1,1,0],
        '1' => [0,0,1,0,0, 0,1,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,1,1,1,0],
        '2' => [0,1,1,1,0, 1,0,0,0,1, 0,0,0,0,1, 0,0,0,1,0, 0,0,1,0,0, 0,1,0,0,0, 1,1,1,1,1],
        '3' => [0,1,1,1,0, 1,0,0,0,1, 0,0,0,0,1, 0,0,1,1,0, 0,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        '4' => [0,0,0,1,0, 0,0,1,1,0, 0,1,0,1,0, 1,0,0,1,0, 1,1,1,1,1, 0,0,0,1,0, 0,0,0,1,0],
        '5' => [1,1,1,1,1, 1,0,0,0,0, 1,1,1,1,0, 0,0,0,0,1, 0,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        '6' => [0,1,1,1,0, 1,0,0,0,0, 1,0,0,0,0, 1,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        '7' => [1,1,1,1,1, 0,0,0,0,1, 0,0,0,1,0, 0,0,0,1,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0],
        '8' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        '9' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 0,1,1,1,1, 0,0,0,0,1, 0,0,0,0,1, 0,1,1,1,0],
        ' ' => [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        '!' => [0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,0,0,0, 0,0,1,0,0],
        '?' => [0,1,1,1,0, 1,0,0,0,1, 0,0,0,0,1, 0,0,0,1,0, 0,0,1,0,0, 0,0,0,0,0, 0,0,1,0,0],
        '.' => [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,1,0,0],
        ',' => [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,1,0,0, 0,1,0,0,0],
        '-' => [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 1,1,1,1,1, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        ':' => [0,0,0,0,0, 0,0,1,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,1,0,0, 0,0,0,0,0],
        '/' => [0,0,0,0,1, 0,0,0,0,1, 0,0,0,1,0, 0,0,1,0,0, 0,1,0,0,0, 1,0,0,0,0, 1,0,0,0,0],
        '\'' => [0,0,1,0,0, 0,0,1,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        _ => return None,
    })
}

/// Draw text at virtual GBC coordinates using the Pokemon-style bitmap font.
/// Returns the width in virtual pixels of the rendered text.
pub fn draw_text_pkmn(
    fb: &mut Framebuffer,
    ctx: &RenderContext,
    text: &str,
    vx: i32,
    vy: i32,
    color: Color,
) -> i32 {
    let scale = ctx.scale;
    let char_w = 6; // 5 pixels + 1 spacing
    let mut cx = vx;

    for ch in text.chars() {
        if let Some(glyph) = get_glyph(ch) {
            let (fbx, fby) = ctx.to_fb(cx, vy);
            for row in 0..7 {
                for col in 0..5 {
                    if glyph[row * 5 + col] != 0 {
                        let px = fbx + col as i32 * scale as i32;
                        let py = fby + row as i32 * scale as i32;
                        for dy in 0..scale as i32 {
                            for dx in 0..scale as i32 {
                                let fx = px + dx;
                                let fy = py + dy;
                                if fx >= 0 && fx < fb.width as i32 && fy >= 0 && fy < fb.height as i32 {
                                    fb.set_pixel(fx, fy, color);
                                }
                            }
                        }
                    }
                }
            }
        }
        cx += char_w;
    }
    cx - vx
}

/// Draw text with a shadow (dark outline behind lighter text)
pub fn draw_text_shadowed(
    fb: &mut Framebuffer,
    ctx: &RenderContext,
    text: &str,
    vx: i32,
    vy: i32,
    fg: Color,
    shadow: Color,
) {
    draw_text_pkmn(fb, ctx, text, vx + 1, vy + 1, shadow);
    draw_text_pkmn(fb, ctx, text, vx, vy, fg);
}

// ─── UI Elements ────────────────────────────────────────

/// Draw a Pokemon-style text/menu box (bordered rectangle)
/// x, y, w, h are in virtual GBC pixels
pub fn draw_text_box(
    fb: &mut Framebuffer,
    ctx: &RenderContext,
    vx: i32,
    vy: i32,
    vw: i32,
    vh: i32,
) {
    let scale = ctx.scale;
    let s = scale as i32;
    let (fx, fy) = ctx.to_fb(vx, vy);
    let fw = vw * s;
    let fh = vh * s;

    let bg = Color::from_rgba(248, 248, 248, 255);
    let border = Color::from_rgba(40, 40, 48, 255);
    let inner_border = Color::from_rgba(96, 96, 112, 255);

    // Fill background
    shapes::fill_rect(fb, fx as f64, fy as f64, fw as f64, fh as f64, bg);

    // Outer border (2 virtual pixels thick)
    let bw = (2 * s) as f64;
    // Top
    shapes::fill_rect(fb, fx as f64, fy as f64, fw as f64, bw, border);
    // Bottom
    shapes::fill_rect(fb, fx as f64, (fy + fh) as f64 - bw, fw as f64, bw, border);
    // Left
    shapes::fill_rect(fb, fx as f64, fy as f64, bw, fh as f64, border);
    // Right
    shapes::fill_rect(fb, (fx + fw) as f64 - bw, fy as f64, bw, fh as f64, border);

    // Inner border line (1 virtual pixel inside)
    let iw = s as f64;
    let inset = 2 * s;
    // Top
    shapes::fill_rect(fb, (fx + inset) as f64, (fy + inset) as f64, (fw - 2 * inset) as f64, iw, inner_border);
    // Bottom
    shapes::fill_rect(fb, (fx + inset) as f64, (fy + fh - inset) as f64 - iw, (fw - 2 * inset) as f64, iw, inner_border);
    // Left
    shapes::fill_rect(fb, (fx + inset) as f64, (fy + inset) as f64, iw, (fh - 2 * inset) as f64, inner_border);
    // Right
    shapes::fill_rect(fb, (fx + fw - inset) as f64 - iw, (fy + inset) as f64, iw, (fh - 2 * inset) as f64, inner_border);
}

/// Draw an HP bar at virtual coordinates
/// x, y: virtual GBC pixels (top-left of bar)
/// w: bar width in virtual pixels
/// current, max: HP values
pub fn draw_hp_bar(
    fb: &mut Framebuffer,
    ctx: &RenderContext,
    vx: i32,
    vy: i32,
    vw: i32,
    current: u16,
    max: u16,
) {
    let scale = ctx.scale;
    let s = scale as i32;
    let (fx, fy) = ctx.to_fb(vx, vy);
    let bar_h = 2 * s; // 2 virtual pixels tall
    let full_w = vw * s;

    // Background (dark)
    shapes::fill_rect(fb, fx as f64, fy as f64, full_w as f64, bar_h as f64,
        Color::from_rgba(40, 40, 48, 255));

    // HP fill
    let ratio = if max > 0 { current as f64 / max as f64 } else { 0.0 };
    let fill_w = ((full_w as f64) * ratio) as i32;

    let hp_color = if ratio > 0.5 {
        Color::from_rgba(72, 208, 72, 255) // green
    } else if ratio > 0.2 {
        Color::from_rgba(248, 208, 48, 255) // yellow
    } else {
        Color::from_rgba(240, 64, 48, 255) // red
    };

    if fill_w > 0 {
        shapes::fill_rect(fb, fx as f64, fy as f64, fill_w as f64, bar_h as f64, hp_color);
    }

    // "HP" label
    draw_text_pkmn(fb, ctx, "HP", vx - 14, vy - 1, Color::from_rgba(248, 176, 48, 255));
}

/// Draw an EXP bar
pub fn draw_exp_bar(
    fb: &mut Framebuffer,
    ctx: &RenderContext,
    vx: i32,
    vy: i32,
    vw: i32,
    current: u32,
    needed: u32,
) {
    let scale = ctx.scale;
    let s = scale as i32;
    let (fx, fy) = ctx.to_fb(vx, vy);
    let bar_h = 2 * s;
    let full_w = vw * s;

    // Background
    shapes::fill_rect(fb, fx as f64, fy as f64, full_w as f64, bar_h as f64,
        Color::from_rgba(40, 40, 48, 255));

    // Fill (cyan)
    let ratio = if needed > 0 { current as f64 / needed as f64 } else { 0.0 };
    let fill_w = ((full_w as f64) * ratio.min(1.0)) as i32;
    if fill_w > 0 {
        shapes::fill_rect(fb, fx as f64, fy as f64, fill_w as f64, bar_h as f64,
            Color::from_rgba(64, 200, 248, 255));
    }
}

/// Draw a right-pointing arrow cursor at virtual coordinates
pub fn draw_cursor(
    fb: &mut Framebuffer,
    ctx: &RenderContext,
    vx: i32,
    vy: i32,
    color: Color,
) {
    let scale = ctx.scale;
    let s = scale as i32;
    let (fx, fy) = ctx.to_fb(vx, vy);

    // Simple right-pointing triangle (4x7 virtual pixels)
    for row in 0..7 {
        let w = match row {
            0 | 6 => 1,
            1 | 5 => 2,
            2 | 4 => 3,
            3 => 4,
            _ => 0,
        };
        for col in 0..w {
            for dy in 0..s {
                for dx in 0..s {
                    let px = fx + col * s + dx;
                    let py = fy + row * s + dy;
                    if px >= 0 && px < fb.width as i32 && py >= 0 && py < fb.height as i32 {
                        fb.set_pixel(px, py, color);
                    }
                }
            }
        }
    }
}

/// Fill the entire virtual screen with a solid color
pub fn fill_virtual_screen(fb: &mut Framebuffer, ctx: &RenderContext, color: Color) {
    let (fx, fy) = ctx.to_fb(0, 0);
    let fw = ctx.virt_w as i32 * ctx.scale as i32;
    let fh = ctx.virt_h as i32 * ctx.scale as i32;
    shapes::fill_rect(fb, fx as f64, fy as f64, fw as f64, fh as f64, color);
}

/// Draw a filled rectangle in virtual coordinates
pub fn fill_rect_v(
    fb: &mut Framebuffer,
    ctx: &RenderContext,
    vx: i32,
    vy: i32,
    vw: i32,
    vh: i32,
    color: Color,
) {
    let s = ctx.scale as i32;
    let (fx, fy) = ctx.to_fb(vx, vy);
    shapes::fill_rect(fb, fx as f64, fy as f64, (vw * s) as f64, (vh * s) as f64, color);
}
