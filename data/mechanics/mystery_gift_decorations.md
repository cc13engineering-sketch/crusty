# Pokemon Crystal — Mystery Gift and Decoration System

Source: pokecrystal disassembly (engine/link/mystery_gift.asm, engine/overworld/decorations.asm, data/decorations/, data/items/mystery_gift_items.asm)

---

## Mystery Gift Overview

Mystery Gift is a feature in Pokemon Crystal that uses the Game Boy Color's infrared port to exchange gifts between two cartridges, or between a cartridge and a Pokemon Pikachu 2 pedometer.

**How to access:** Select "Mystery Gift" from the main menu (unlocked after visiting the Goldenrod Dept Store 5F and talking to the girl near the stairs).

### Communication Protocol
- Uses infrared (IR) communication
- Both players select Mystery Gift from the main menu simultaneously
- One device becomes IR_RECEIVER (1), the other IR_SENDER (2)
- Region check: REGION_PREFIX = $96, REGION_CODE = $90 (USA)
- Error handling: checksum verification, timeout detection, cancellation support

Source: `engine/link/mystery_gift.asm`

---

## Mystery Gift Rules

### Daily Limits
- **Maximum 5 gifts per day** (MAX_MYSTERY_GIFT_PARTNERS)
  - Tracked in sNumDailyMysteryGiftPartnerIDs
  - Reset daily via the Mystery Gift daily timer (DoMysteryGiftIfDayHasPassed in time.asm)
- **Maximum 1 gift from the same partner per day**
  - Partner IDs stored in sDailyMysteryGiftPartnerIDs
  - Checked by .CheckAlreadyGotAGiftFromThatPerson

### Gift Types
Gifts are either items or decorations:
1. If wMysteryGiftPartnerSentDeco is set: decoration gift
2. Otherwise: item gift

### Item Retrieval
- Items aren't given directly — they're stored in sBackupMysteryGiftItem
- Message: "receive gift at counter" (player must visit a Pokemon Center 2F)
- If there's already an uncollected gift, new gifts can't be received until the previous one is claimed

### Pokemon Pikachu 2 Compatibility
- Compatible with Pokemon Pikachu 2 pedometer (POKEMON_PIKACHU_2_VERSION)
- Pikachu 2 connection skips the 5-per-day check and partner dupe check
- Different version handling for items vs saved data

---

## Mystery Gift Item Pool

Items are selected from MysteryGiftItems table (37 items total):

### Berries
| Item |
|------|
| Berry |
| PRZCureBerry |
| Mint Berry |
| Ice Berry |
| Burnt Berry |
| PSNCureBerry |
| Bitter Berry |
| MiracleBerry |
| Gold Berry |

### Battle Items
| Item |
|------|
| Guard Spec. |
| X Defend |
| X Attack |
| Dire Hit |
| X Special |
| X Accuracy |

### Mail
| Item |
|------|
| Eon Mail |
| Morph Mail |
| Music Mail |
| BlueSky Mail |
| Mirage Mail |

### Consumables
| Item |
|------|
| Revive |
| Great Ball |
| Super Repel |
| Max Repel |
| Elixer |
| Ether |
| Max Ether |
| Max Elixer |
| Max Revive |

### Evolution Stones
| Item |
|------|
| Water Stone |
| Fire Stone |
| Leaf Stone |
| Thunderstone |

### Rare Items
| Item |
|------|
| Scope Lens |
| HP Up |
| PP Up |
| Rare Candy |

Source: `data/items/mystery_gift_items.asm`

---

## Mystery Gift Decoration Pool

Decorations received via Mystery Gift (37 decorations total):

### Dolls (Small)
| Decoration |
|-----------|
| Jigglypuff Doll |
| Poliwag Doll |
| Diglett Doll |
| Staryu Doll |
| Magikarp Doll |
| Oddish Doll |
| Gengar Doll |
| Shellder Doll |
| Grimer Doll |
| Voltorb Doll |
| Weedle Doll |
| Geodude Doll |
| Machop Doll |
| Bulbasaur Doll |
| Squirtle Doll |
| Tentacool Doll |
| Surf Pikachu Doll |
| Unown Doll |

### Big Dolls
| Decoration |
|-----------|
| Big Onix Doll |
| Big Lapras Doll |

### Posters
| Decoration |
|-----------|
| Clefairy Poster |
| Jigglypuff Poster |
| Pikachu Poster |

### Game Consoles
| Decoration |
|-----------|
| SNES |
| Famicom |
| N64 |
| Virtual Boy |

### Plants
| Decoration |
|-----------|
| Magna Plant |
| Tropic Plant |
| Jumbo Plant |

### Beds
| Decoration |
|-----------|
| Pink Bed |
| Polkadot Bed |
| Pikachu Bed |

### Carpets
| Decoration |
|-----------|
| Red Carpet |
| Blue Carpet |
| Yellow Carpet |
| Green Carpet |

Source: `data/decorations/mystery_gift_decos.asm`

---

## Decoration System

The player's room in New Bark Town can be customized with decorations. Decorations are organized into 7 categories, each with a single active slot.

### Decoration Categories

| Category | Max Active | Description |
|----------|-----------|-------------|
| Bed | 1 | Changes the bed in the player's room |
| Carpet | 1 | Floor covering |
| Plant | 1 | Decorative plant |
| Poster | 1 | Wall poster |
| Game Console | 1 | Console on the desk (can be interacted with) |
| Ornament | 1 | Small doll displayed on shelf |
| Big Doll | 1 | Large doll displayed on the floor |

Only one decoration per category can be active at a time. Selecting a new decoration in a category replaces the old one.

### Default Room
- Default bed: standard bed (no decoration flag)
- No carpet, plant, poster, console, ornament, or big doll by default

### Obtaining Decorations

**Mystery Gift:** Primary method. 37 decorations available through the Mystery Gift pool.

**Mom's Shopping:** Mom occasionally calls saying she bought a decoration with your savings. She shops at the Goldenrod Dept Store.

**Purchased:** Some decorations can be bought at the Goldenrod Dept Store 5F.

### Decoration Interaction
- Game consoles: Interacting plays a short animation/game reference
- SNES: "A classic system!"
- N64: References to N64 games
- Virtual Boy: References Virtual Boy
- Famicom: "A system from a faraway land"

### Decoration Flags
Each decoration has a DECOFLAG_* constant. The flags track which decorations have been received. Duplicates are handled: if a Mystery Gift decoration has already been received (flag set), the system falls through to give an item instead.

Source: `engine/overworld/decorations.asm`

---

## Trainer House Connection

Mystery Gift also connects to the Trainer House in Viridian City:
- After a successful Mystery Gift exchange, the partner's trainer data is saved
- This trainer appears in the Viridian City Trainer House for battle
- Trainer data includes: name, party composition
- Stored in sMysteryGiftTrainerHouseFlag, sMysteryGiftPartnerName, sMysteryGiftTrainer

Source: `engine/link/mystery_gift.asm` — .SaveMysteryGiftTrainerName

---

## GS Ball Event (Crystal)

The GS Ball is a special event item connected to Mystery Gift infrastructure:
- BackupGSBallFlag / RestoreGSBallFlag called during Mystery Gift operations
- The GS Ball triggers the Celebi event at the Ilex Forest shrine
- In the original Japanese Crystal, this was distributed via mobile adapter
- International releases had it distributed through special events
- The Virtual Console re-release makes it available through normal gameplay

Source: References in `engine/link/mystery_gift.asm` lines 103-105

---

## Mystery Gift Error Handling

| Error | Description |
|-------|-------------|
| MG_WRONG_CHECKSUM | Data integrity failure |
| MG_TIMED_OUT | Communication timeout |
| MG_CANCELED | User pressed B to cancel |
| MG_WRONG_PREFIX | Region/version mismatch |

On communication error, the game automatically retries (jp DoMysteryGift).
On cancellation, displays "Link has been canceled" and exits.

Source: `engine/link/mystery_gift.asm` — error flag definitions and handlers
