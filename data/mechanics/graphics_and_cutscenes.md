# Pokemon Crystal -- Graphics and Cutscenes

Source: pokecrystal disassembly (engine/gfx/*.asm, engine/movie/*.asm)

---

## Graphics Engine (engine/gfx/)

### Sprite Loading (load_pics.asm)
- Pokemon sprites loaded from compressed 2bpp data
- Front and back sprites stored separately
- Decompression into VRAM tile data
- Sprites are 7x7 tiles (56x56 pixels) for Pokemon
- Trainer sprites vary in size

### Pic Animation (pic_animation.asm)
- Pokemon sprite animations on the stats screen and Pokedex
- Frame-based animation system
- Each species has predefined animation data
- Animation types: bounce, shake, stretch, etc.

### Mon Icons (mon_icons.asm)
- Small 2-frame animated icons used in party menu, PC
- Each species has a 16x16 pixel icon
- Icon pointers in data/icon_pointers.asm
- 2-frame animation alternates with a timer

### Color System

**CGB Layouts (cgb_layouts.asm):**
- Defines palette layouts for Game Boy Color
- Each screen/context has a layout ID (SCGB_* constants)
- Layouts include: battle, party menu, Pokedex, overworld, PC, etc.

**Crystal Layouts (crystal_layouts.asm):**
- Crystal-exclusive palette configurations
- Enhanced color schemes compared to Gold/Silver

**SGB Layouts (sgb_layouts.asm):**
- Super Game Boy palette layouts
- Simpler palette system (4 palettes only)
- SGB border and enhancement packets

**Color Palettes (color.asm):**
- DMG (original Game Boy) to CGB palette conversion
- DmgToCgbBGPals: converts DMG background palette format to CGB
- DmgToCgbObjPals: converts DMG object palette format to CGB

### DMA Transfer (dma_transfer.asm)
- Hardware DMA for OAM (sprite) data transfer
- Runs during VBlank for glitch-free sprite updates
- Standard Game Boy DMA routine

### Font Loading (load_font.asm, load_overworld_font.asm)
- LoadStandardFont: loads the standard text font tiles
- LoadFontsExtra: loads additional character tiles
- load_overworld_font.asm: loads font optimized for map display

### Player Graphics (player_gfx.asm)
- Player character sprite loading
- Male/Female sprite selection based on wPlayerGender
- Handles overworld sprite and menu sprite

### Trade Mon Frontpic (trademon_frontpic.asm)
- Loads the front sprite of a Pokemon during trade animations
- Handles decompression and palette assignment

### Place Graphic (place_graphic.asm)
- General-purpose graphic placement utility
- Places decompressed tiles at specified screen coordinates

### Load Push OAM (load_push_oam.asm)
- Loads OAM data for hardware sprite rendering
- Push OAM routine copies shadow OAM to hardware OAM during VBlank

---

## Movie/Cutscene System (engine/movie/)

### Intro Sequence (intro.asm)
- Game startup animation
- Shows Suicune running, Unown flying
- Crystal-exclusive intro (different from Gold/Silver)
- Multiple animation phases with sprite manipulation
- Music: MUSIC_CRYSTAL_OPENING

### Title Screen (title.asm)
- Suicune on the title with crystal effects
- "POKEMON" logo and "Crystal Version" text
- Press START prompt
- Cycles through demo Pokemon if idle

### Unused Title (unused_title.asm)
- Leftover Gold/Silver title screen code
- Not displayed in Crystal but code remains

### Splash Screen (splash.asm)
- Game Freak logo animation
- Star sparkle effect
- Displayed on boot before intro

### GBC Only Check (gbc_only.asm)
- Checks if running on Game Boy Color hardware
- Displays "This game requires Game Boy Color" on original DMG
- Crystal is GBC-exclusive

### Evolution Animation (evolution_animation.asm)
- Morphing animation between pre-evolution and evolved form
- White flash transition effect
- Palette cycling during morph
- "Congratulations!" sequence
- Can be cancelled by pressing B (returns to pre-evolution)

### Trade Animation (trade_animation.asm)
- Cable trade visual sequence
- Pokemon sent up through cable visual
- Receiving Pokemon comes down through cable
- Link cable imagery with sparkle effects
- Different animation for Time Capsule trades

### Credits Sequence (credits.asm)

**Credits Flow:**
1. Staff credits scroll with names
2. Pokemon sprites displayed alongside credits
3. Player character walking/biking scenes
4. Final statistics displayed

**Credits Data:**
- data/credits_script.asm: defines the sequence of events
- data/credits_strings.asm: contains staff names and titles

### Init Hall of Fame Credits (init_hof_credits.asm)
- Sets up the Hall of Fame display
- Prepares palette and graphics data for credits
- Loads party Pokemon data for the victory display

---

## Sprite Animation System (engine/sprite_anims/)

### Overview
- General-purpose sprite animation engine
- Used for overworld NPCs, battle effects, menu cursors
- Each animation has: frames, timing, movement patterns

### Animation Objects
- SpriteAnimStruct: data structure per animation instance
- Fields: index, position (x/y), offset (x/y), tile ID, attributes, jumptable index, variables

### Animation Types Used
- Slots Golem (falls and rolls across screen)
- Slots Chansey (walks across, drops egg)
- Slots egg (falls after Chansey)
- Memory game cursor
- Various battle animation objects
- NPC walking/turning animations

---

## Tileset System (engine/tilesets/)

### Tileset Loading
- Each map area uses a tileset defining visual appearance
- Tilesets contain: tile graphics, collision data, palette info
- data/tilesets.asm: master tileset table
- data/tilesets/: per-tileset data files

### Map Rendering
- Maps composed of metatiles (2x2 or 4x4 tile blocks)
- data/maps/blocks.asm: block data for each map
- data/maps/attributes.asm: per-block attributes (collision, terrain type)
- Blocks reference tiles from the current tileset

---

## Printer System (engine/printer/)

### Game Boy Printer Support
- Print Pokemon data, Pokedex entries, mail
- Printer communication protocol
- Print quality controlled by Options menu PRINT setting
- Requires Game Boy Printer accessory

---

## Debug Graphics (engine/debug/)

### Debug Tools
- Debugging graphics and test screens
- Not accessible in retail builds
- May include tile viewers, palette testers
