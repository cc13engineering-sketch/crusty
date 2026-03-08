# Pokemon Crystal -- Pokedex System

Source: pokecrystal disassembly (engine/pokedex/*.asm)

---

## Pokedex Modes

### Display States (from pokedex.asm)

The Pokedex operates as a state machine with these states:

| State | Description |
|-------|-------------|
| DEXSTATE_MAIN_SCR | Main listing screen |
| DEXSTATE_UPDATE_MAIN_SCR | Refresh main listing |
| DEXSTATE_DEX_ENTRY_SCR | Individual entry view |
| DEXSTATE_UPDATE_DEX_ENTRY_SCR | Refresh entry view |
| DEXSTATE_REINIT_DEX_ENTRY_SCR | Re-initialize entry |
| DEXSTATE_SEARCH_SCR | Search/filter screen |
| DEXSTATE_UPDATE_SEARCH_SCR | Refresh search |
| DEXSTATE_OPTION_SCR | Options/mode select |
| DEXSTATE_UPDATE_OPTION_SCR | Refresh options |
| DEXSTATE_SEARCH_RESULTS_SCR | Search results list |
| DEXSTATE_UPDATE_SEARCH_RESULTS_SCR | Refresh results |
| DEXSTATE_UNOWN_MODE | Unown catalog |
| DEXSTATE_UPDATE_UNOWN_MODE | Refresh Unown catalog |
| DEXSTATE_EXIT | Exit Pokedex |

### Listing Modes

**New Pokedex Order** -- Pokemon ordered by Johto Pokedex number (data/pokemon/dex_order_new.asm)
**Old Pokedex Order** -- Pokemon ordered by National Pokedex number (Kanto-first, 1-251)
**Alphabetical Order** -- Pokemon sorted A-Z (data/pokemon/dex_order_alpha.asm)

The current mode is stored in wCurDexMode and persists between Pokedex sessions via wLastDexMode.

---

## Pokedex Entry Screen

### Entry Display (pokedex_2.asm, pokedex_3.asm)

Each Pokemon entry shows:

**Seen but not caught:**
- Species name
- Silhouette sprite
- "Seen" indicator

**Caught:**
- Species name
- Full-color sprite
- Species category (e.g., "Seed Pokemon")
- Height (ft/in)
- Weight (lbs)
- Pokedex description text
- Cry playback option

### Dex Entry Data
- Entry text stored in data/pokemon/dex_entries/ (one file per species)
- Entry pointers in data/pokemon/dex_entry_pointers.asm
- Each entry contains: species name, height, weight, description text

---

## New Pokedex Entry (new_pokedex_entry.asm)

### Triggered When
- A new species is caught for the first time
- "New Pokedex data will be added" message
- Full entry displayed automatically

### Process
1. Species registered as caught in Pokedex flags
2. Entry screen displayed with species info
3. Player presses A/B to dismiss

---

## Search/Sort Functionality

### Search Options (DEXSTATE_SEARCH_SCR)
- **By Type:** Filter Pokemon by type (data/types/search_types.asm, search_strings.asm)
- **Alphabetical:** Sort A-Z

### Search Results
- Filtered list displayed in scrollable format
- Only shows seen/caught Pokemon matching criteria
- Results count displayed

---

## Unown Mode (unown_dex.asm)

### Unlock Condition
- STATUSFLAGS_UNOWN_DEX_F must be set in wStatusFlags
- Unlocked by catching specific Unown forms
- Stored in wUnlockedUnownMode

### Display
- Shows all 26 Unown forms (A-Z)
- Each form displayed as its unique letter glyph
- Only caught forms are shown (uncaught forms blank)

### Unown Data
- data/pokemon/unown_pic_pointers.asm: sprite pointers for each form
- data/pokemon/unown_words.asm: associated words for each Unown letter

---

## Pokedex Flags

### Seen/Caught Tracking
- Each Pokemon has two flags: Seen and Caught
- 251 species, stored as bit arrays
- SetSeenAndCaughtMon: sets both flags
- CheckCaughtMon: returns whether species is caught
- CheckSeenMon: returns whether species is seen

### Pokedex Counts
- Total Seen count and Total Caught count
- Displayed on Trainer Card and Pokedex main screen
- Professor Oak's rating based on caught count

---

## Area Display

### Where Pokemon Can Be Found
- Displays map locations where a species appears
- Based on encounter table data
- Shows markers on the Town Map for each location
- Different markers for: Grass, Water, Cave encounters
- Landmark system (data/maps/landmarks.asm) used for location names

---

## Pokedex Rating

### Professor Oak's PC (engine/events/prof_oaks_pc.asm)
- Accessible from any Pokemon Center PC
- Oak evaluates your Pokedex completion
- Different messages based on caught count thresholds
- Catching all 251 Pokemon triggers a special congratulatory message

### Rating Thresholds
Multiple tiers of commentary based on number of species caught, encouraging the player to continue collecting.

---

## Cursor and Navigation

### Main List Navigation
- Up/Down: scroll through Pokemon list
- A: view selected entry
- B: exit Pokedex
- START: search options
- SELECT: mode options

### Cursor Position Memory
- wPrevDexEntry: remembers last viewed entry
- wDexListingScrollOffset: remembers scroll position
- Cursor returns to previous position when re-entering Pokedex

### Scrolling
- List shows 7 entries at a time
- Scroll offset tracked in wDexListingScrollOffset
- Smooth scrolling through species list
