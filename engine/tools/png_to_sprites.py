#!/usr/bin/env python3
"""
PNG Tilesheet → sprites.rs Converter

Converts a PNG spritesheet (with 4-color indexed GBC-style tiles) into
Rust const string declarations for use in the Crusty engine's sprites.rs.

The engine uses 16×16 tiles stored as 256-char strings where each char
is a palette index: '0'=transparent, '1'=light, '2'=medium, '3'=dark.
Palettes are applied at render time in render.rs.

Usage:
    python3 png_to_sprites.py <input.png> [--tile-size 16] [--cols 0] [--output sprites_gen.rs]

Arguments:
    input.png       Path to the PNG tilesheet
    --tile-size N   Tile size in pixels (default: 16, also supports 8)
    --cols N        Expected columns per row (0 = auto-detect from image width)
    --output FILE   Output file path (default: stdout)
    --preview       Print ASCII preview of each tile
    --threshold     Color clustering method: 'auto' (k-means), 'gray' (luminance quartiles)

The script auto-detects the 4 palette colors from the image by finding
the most common unique colors and mapping them to indices 0-3 sorted
by luminance (darkest=3, lightest=0).

For 8×8 source tiles, use --tile-size 8. The output will still be
256-char strings (upscaled 2x to 16×16) so no engine changes are needed.
"""

import sys
import argparse
from pathlib import Path

def luminance(r, g, b):
    """Perceived luminance (ITU-R BT.601)."""
    return 0.299 * r + 0.587 * g + 0.114 * b

def load_image(path):
    """Load PNG and return (width, height, pixels) where pixels is list of (r,g,b,a) tuples."""
    try:
        from PIL import Image
    except ImportError:
        print("ERROR: Pillow is required. Install with: pip3 install Pillow", file=sys.stderr)
        sys.exit(1)

    img = Image.open(path).convert("RGBA")
    w, h = img.size
    pixels = list(img.getdata())
    return w, h, pixels

def find_palette_colors(pixels, n=4):
    """Find the N most common colors, sorted by luminance (lightest first)."""
    from collections import Counter
    # Only count non-fully-transparent pixels
    color_counts = Counter()
    for r, g, b, a in pixels:
        if a > 128:  # skip transparent
            color_counts[(r, g, b)] += 1

    if len(color_counts) == 0:
        # All transparent
        return [(0, 0, 0), (85, 85, 85), (170, 170, 170), (255, 255, 255)]

    # Get top N colors
    top_colors = [c for c, _ in color_counts.most_common(n)]

    # If fewer than N colors, pad with interpolated values
    while len(top_colors) < n:
        top_colors.append((128, 128, 128))

    # Sort by luminance: index 0 = lightest (transparent), 3 = darkest
    top_colors.sort(key=lambda c: luminance(*c), reverse=True)
    return top_colors

def color_to_index(r, g, b, a, palette):
    """Map a pixel color to the nearest palette index (0-3)."""
    if a < 128:
        return 0  # transparent

    min_dist = float('inf')
    best_idx = 0
    for idx, (pr, pg, pb) in enumerate(palette):
        dist = (r - pr) ** 2 + (g - pg) ** 2 + (b - pb) ** 2
        if dist < min_dist:
            min_dist = dist
            best_idx = idx
    return best_idx

def extract_tile(pixels, img_w, tx, ty, tile_size):
    """Extract a tile_size x tile_size tile at grid position (tx, ty)."""
    tile_pixels = []
    for row in range(tile_size):
        for col in range(tile_size):
            px = tx * tile_size + col
            py = ty * tile_size + row
            idx = py * img_w + px
            tile_pixels.append(pixels[idx])
    return tile_pixels

def tile_to_string(tile_pixels, palette, tile_size, target_size=16):
    """Convert tile pixels to a palette-indexed string."""
    # First, map to indices at source resolution
    indices = []
    for r, g, b, a in tile_pixels:
        indices.append(color_to_index(r, g, b, a, palette))

    # If tile_size < target_size, upscale
    if tile_size < target_size:
        scale = target_size // tile_size
        upscaled = []
        for row in range(target_size):
            for col in range(target_size):
                src_row = row // scale
                src_col = col // scale
                upscaled.append(indices[src_row * tile_size + src_col])
        indices = upscaled

    return ''.join(str(i) for i in indices)

def is_blank_tile(tile_string):
    """Check if a tile is all one color (likely blank/empty)."""
    return len(set(tile_string)) <= 1

def main():
    parser = argparse.ArgumentParser(description="Convert PNG tilesheet to sprites.rs format")
    parser.add_argument("input", help="Input PNG file")
    parser.add_argument("--tile-size", type=int, default=16, help="Tile size in pixels (default: 16)")
    parser.add_argument("--cols", type=int, default=0, help="Columns per row (0=auto)")
    parser.add_argument("--output", type=str, default=None, help="Output file (default: stdout)")
    parser.add_argument("--preview", action="store_true", help="Print ASCII preview")
    parser.add_argument("--skip-blank", action="store_true", help="Skip blank/single-color tiles")
    parser.add_argument("--prefix", type=str, default="TILE", help="Const name prefix (default: TILE)")
    args = parser.parse_args()

    img_w, img_h, pixels = load_image(args.input)
    tile_size = args.tile_size

    cols = args.cols if args.cols > 0 else img_w // tile_size
    rows = img_h // tile_size

    print(f"Image: {img_w}x{img_h}, tile size: {tile_size}x{tile_size}", file=sys.stderr)
    print(f"Grid: {cols} cols x {rows} rows = {cols * rows} tiles", file=sys.stderr)

    # Find palette
    palette = find_palette_colors(pixels)
    print(f"Palette (lightest→darkest):", file=sys.stderr)
    for i, (r, g, b) in enumerate(palette):
        print(f"  {i}: RGB({r},{g},{b}) lum={luminance(r,g,b):.0f}", file=sys.stderr)

    # Extract and convert tiles
    output_lines = []
    output_lines.append(f"// Auto-generated from {Path(args.input).name}")
    output_lines.append(f"// {cols}x{rows} grid, {tile_size}px tiles → 16x16 sprites")
    output_lines.append("")

    tile_num = 0
    for ty in range(rows):
        for tx in range(cols):
            tile_pixels = extract_tile(pixels, img_w, tx, ty, tile_size)
            tile_str = tile_to_string(tile_pixels, palette, tile_size)

            if args.skip_blank and is_blank_tile(tile_str):
                continue

            name = f"{args.prefix}_{tile_num:03d}"
            output_lines.append(f'pub const {name}: &str = "{tile_str}";')

            if args.preview:
                # ASCII preview
                chars = " ░▒█"
                output_lines.append(f"// Preview of {name}:")
                for row in range(16):
                    line = ""
                    for col in range(16):
                        idx = int(tile_str[row * 16 + col])
                        line += chars[idx]
                    output_lines.append(f"// {line}")
                output_lines.append("")

            tile_num += 1

    output_text = "\n".join(output_lines) + "\n"

    if args.output:
        Path(args.output).write_text(output_text)
        print(f"Wrote {tile_num} tiles to {args.output}", file=sys.stderr)
    else:
        print(output_text)

    print(f"Total: {tile_num} tiles extracted", file=sys.stderr)

if __name__ == "__main__":
    main()
