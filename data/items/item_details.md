# Pokemon Crystal - Item Details

Detailed item data not covered in all_items.md. Extracted from pokecrystal data/items/ (17 files).

Source: `data/items/*.asm`

---

## In-Game Item Descriptions

Every item has a 2-line description shown in the item menu. Key descriptions:

### Poke Balls
| Item | Description |
|------|------------|
| Master Ball | "The best BALL. It never misses." |
| Ultra Ball | "A BALL with a high rate of success." |
| Great Ball | "A BALL with a decent success rate." |
| Poke Ball | "An item for catching POKEMON." |
| Heavy Ball | "A BALL for catching heavy POKEMON." |
| Level Ball | "A BALL for lower-level POKEMON." |
| Lure Ball | "A BALL for POKEMON hooked by a ROD." |
| Fast Ball | "A BALL for catching fast POKEMON." |
| Friend Ball | "A BALL that makes POKEMON friendly." |
| Moon Ball | "A BALL for MOON STONE evolvers." |
| Love Ball | "For catching the opposite gender." |
| Park Ball | "The Bug-Catching Contest BALL." |
| GS Ball | "The mysterious BALL." |

### Held Items
| Item | Description |
|------|------------|
| BrightPowder | "Lowers the foe's accuracy. (HOLD)" |
| Lucky Punch | "Ups critical hit ratio of CHANSEY." |
| Metal Powder | "Raises DEFENSE of DITTO. (HOLD)" |
| Quick Claw | "Raises 1st strike ratio. (HOLD)" |
| King's Rock | "May make the foe flinch. (HOLD)" |
| Scope Lens | "Raises critical hit ratio. (HOLD)" |
| Focus Band | "May prevent fainting. (HOLD)" |
| Leftovers | "Restores HP during battle. (HOLD)" |
| Exp. Share | "Shares battle EXP. Points. (HOLD)" |
| Amulet Coin | "Doubles monetary earnings. (HOLD)" |
| Cleanse Tag | "Helps repel wild POKEMON. (HOLD)" |
| Smoke Ball | "Escape from wild POKEMON. (HOLD)" |
| Everstone | "Stops evolution. (HOLD)" |
| Lucky Egg | "Earns extra EXP. points. (HOLD)" |
| Thick Club | "A bone of some sort. Sell low." |
| Stick | "An ordinary stick. Sell low." |
| Light Ball | "An odd, electrical orb. (HOLD)" |
| Berserk Gene | "Boosts ATTACK but causes confusion." |
| Dragon Scale | "A rare dragon-type item." |
| Up-Grade | "A mysterious box made by SILPH CO." |

### Type-Boost Items (all "(HOLD)", +10% to type)
| Item | Type | Description |
|------|------|------------|
| Soft Sand | Ground | "Powers up ground-type moves." |
| Sharp Beak | Flying | "Powers up flying-type moves." |
| Poison Barb | Poison | "Powers up poison-type moves." |
| SilverPowder | Bug | "Powers up bug-type moves." |
| Mystic Water | Water | "Powers up water-type moves." |
| TwistedSpoon | Psychic | "Powers up psychic-type moves." |
| BlackBelt | Fighting | "Boosts fighting-type moves." |
| BlackGlasses | Dark | "Powers up dark-type moves." |
| Pink Bow | Normal | "Powers up normal-type moves." |
| Polkadot Bow | Normal | "Powers up normal-type moves." |
| NeverMeltIce | Ice | "Powers up ice-type moves." |
| Magnet | Electric | "Boosts electric-type moves." |
| Miracle Seed | Grass | "Powers up grass-type moves." |
| Charcoal | Fire | "Powers up fire-type moves." |
| Hard Stone | Rock | "Powers up rock-type moves." |
| Spell Tag | Ghost | "Powers up ghost-type moves." |
| Metal Coat | Steel | "Powers up steel-type moves." |
| Dragon Fang | Dragon | "Powers up dragon-type moves." |

### Berries (all "(HOLD)")
| Item | Description |
|------|------------|
| Berry | "A self-restore item. (10HP, HOLD)" |
| Gold Berry | "A self-restore item. (30HP, HOLD)" |
| PSNCureBerry | "A self-cure for poison." |
| PRZCureBerry | "A self-cure for paralysis." |
| Burnt Berry | "A self-cure for freezing." |
| Ice Berry | "A self-heal for a burn." |
| Bitter Berry | "A self-cure for confusion." |
| Mint Berry | "A self-awakening for sleep." |
| MiracleBerry | "Cures all status problems." |
| MysteryBerry | "A self-restore for PP." |

### Mail (all "(HOLD)")
| Item | Description |
|------|------------|
| Flower Mail | "Flower-print MAIL." |
| Surf Mail | "LAPRAS-print MAIL." |
| LiteBlue Mail | "DRATINI-print MAIL." |
| Portrait Mail | "MAIL featuring the holder's likeness." |
| Lovely Mail | "Heart-print MAIL." |
| Eon Mail | "EEVEE-print MAIL." |
| Morph Mail | "DITTO-print MAIL." |
| BlueSky Mail | "Sky-print MAIL." |
| Music Mail | "NATU-print MAIL." |
| Mirage Mail | "MEW-print MAIL." |

### Key Items
| Item | Description |
|------|------------|
| Bicycle | "A collapsible bike for fast movement." |
| Coin Case | "Holds up to 9,999 game coins." |
| Itemfinder | "Checks for unseen items in the area." |
| Red Scale | "A scale from the red GYARADOS." |
| SecretPotion | "Fully heals any POKEMON." |
| S.S.Ticket | "A ticket for the S.S.AQUA." |
| Mystery Egg | "An EGG obtained from MR.POKEMON." |
| Clear Bell | "Makes a gentle ringing." |
| Silver Wing | "A strange, silver-colored feather." |
| Card Key | "Opens shutters in the RADIO TOWER." |
| Machine Part | "A machine part for the POWER PLANT." |
| Egg Ticket | "May use at Goldenrod trade corner." |
| Lost Item | "The POKE DOLL lost by the COPYCAT." |
| Basement Key | "Opens doors." |
| Pass | "A ticket for the MAGNET TRAIN." |
| SquirtBottle | "A bottle used for watering plants." |
| Rainbow Wing | "A mystical feather of rainbow colors." |
| Blue Card | "Card to save points." |
| Normal Box | "Open it and see what's inside." |
| Gorgeous Box | "Open it and see what's inside." |

---

## HP Healing Values (Exact)

From `data/items/heal_hp.asm`:

| Item | HP Restored |
|------|------------|
| Potion | 20 |
| Super Potion | 50 |
| Hyper Potion | 200 |
| Max Potion | Full HP |
| Full Restore | Full HP + status |
| Fresh Water | 50 |
| Soda Pop | 60 |
| Lemonade | 80 |
| MooMoo Milk | 100 |
| RageCandyBar | 20 |
| Berry | 10 (held, auto at <50% HP) |
| Gold Berry | 30 (held, auto at <50% HP) |
| Berry Juice | 20 |
| EnergyPowder | 50 (bitter, -happiness) |
| Energy Root | 200 (bitter, -happiness) |

## Status Healing Items

From `data/items/heal_status.asm`:

| Item | Cures | Context |
|------|-------|---------|
| Antidote | Poison | Menu use |
| Burn Heal | Burn | Menu use |
| Ice Heal | Freeze | Menu use |
| Awakening | Sleep | Menu use |
| Parlyz Heal | Paralysis | Menu use |
| Full Heal | All status | Menu use |
| Full Restore | All status | Menu use |
| Heal Powder | All status | Menu use (bitter) |
| PSNCureBerry | Poison | Held item, auto-use |
| PRZCureBerry | Paralysis | Held item, auto-use |
| Burnt Berry | Freeze | Held item, auto-use |
| Ice Berry | Burn | Held item, auto-use |
| Mint Berry | Sleep | Held item, auto-use |
| MiracleBerry | All status | Held item, auto-use |

---

## X Stat Items

From `data/items/x_stats.asm`:

| Item | Stat Boosted | Effect |
|------|-------------|--------|
| X Attack | Attack | +1 stage in battle |
| X Defend | Defense | +1 stage in battle |
| X Speed | Speed | +1 stage in battle |
| X Special | Sp. Attack | +1 stage in battle |

Note: X Accuracy and Dire Hit are handled separately (accuracy/crit ratio).

---

## Apricorn-to-Ball Mappings

From `data/items/apricorn_balls.asm`:

| Apricorn | Ball | Special Effect |
|----------|------|---------------|
| Red Apricorn | Level Ball | Better catch rate if your mon is higher level |
| Blue Apricorn | Lure Ball | Better catch rate on fished Pokemon |
| Yellow Apricorn | Moon Ball | Better catch rate on Moon Stone evolvers |
| Green Apricorn | Friend Ball | Caught Pokemon starts with high friendship |
| White Apricorn | Fast Ball | Better catch rate on fast/fleeing Pokemon |
| Black Apricorn | Heavy Ball | Better catch rate on heavy Pokemon |
| Pink Apricorn | Love Ball | Better catch rate on opposite gender |

---

## Fruit Trees (Berry Locations)

From `data/items/fruit_trees.asm`:

### Johto
| Location | Berry |
|----------|-------|
| Route 29 | Berry |
| Route 30 (1) | Berry |
| Route 38 | Berry |
| Route 46 (1) | Berry |
| Route 30 (2) | PSNCureBerry |
| Route 33 | PSNCureBerry |
| Route 31 | Bitter Berry |
| Route 43 | Bitter Berry |
| Violet City | PRZCureBerry |
| Route 46 (2) | PRZCureBerry |
| Route 35 | MysteryBerry |
| Route 45 | MysteryBerry |
| Route 36 | Ice Berry |
| Route 26 | Ice Berry |
| Route 39 | Mint Berry |
| Route 44 | Burnt Berry |
| Route 37 (1) | Red Apricorn |
| Route 37 (2) | Blue Apricorn |
| Route 37 (3) | Black Apricorn |
| Azalea Town | White Apricorn |
| Route 42 (1) | Pink Apricorn |
| Route 42 (2) | Green Apricorn |
| Route 42 (3) | Yellow Apricorn |

### Kanto
| Location | Berry |
|----------|-------|
| Route 11 | Berry |
| Route 2 | PSNCureBerry |
| Route 1 | Bitter Berry |
| Route 8 | PRZCureBerry |
| Pewter City (1) | Ice Berry |
| Pewter City (2) | Mint Berry |
| Fuchsia City | Burnt Berry |

---

## Special Shop Inventories

### Bargain Shop (Goldenrod Underground, daily)

| Item | Price (discounted) | Normal Price |
|------|-------------------|-------------|
| Nugget | 4,500 | 5,000 (sell) |
| Pearl | 650 | 700 (sell) |
| Big Pearl | 3,500 | 3,750 (sell) |
| Stardust | 900 | 1,000 (sell) |
| Star Piece | 4,600 | 4,900 (sell) |

### Buena's Prize Exchange (Radio show points)

| Item | Points Required |
|------|----------------|
| Ultra Ball | 2 |
| Full Restore | 2 |
| Nugget | 3 |
| Rare Candy | 3 |
| Protein | 5 |
| Iron | 5 |
| Carbos | 5 |
| Calcium | 5 |
| HP Up | 5 |

### Rooftop Sale (Goldenrod Dept Store, occasional)

**Sale 1:**
| Item | Price (discounted) |
|------|-------------------|
| Poke Ball | 150 |
| Great Ball | 500 |
| Super Potion | 500 |
| Full Heal | 500 |
| Revive | 1,200 |

**Sale 2:**
| Item | Price (discounted) |
|------|-------------------|
| Hyper Potion | 1,000 |
| Full Restore | 2,000 |
| Full Heal | 500 |
| Ultra Ball | 1,000 |
| Protein | 7,800 |

### Mahogany Fake Shop (before Rockets cleared)
| Item | Notes |
|------|-------|
| TinyMushroom | Rocket front |
| SlowpokeTail | Rocket front |
| Poke Ball | |
| Potion | |

---

## All Mart Inventories

### Johto Marts
| City | Items |
|------|-------|
| Cherrygrove (pre-Dex) | Potion, Antidote, Parlyz Heal, Awakening |
| Cherrygrove (post-Dex) | Poke Ball, Potion, Antidote, Parlyz Heal, Awakening |
| Violet | Poke Ball, Potion, Escape Rope, Antidote, Parlyz Heal, Awakening, X Defend, X Attack, X Speed, Flower Mail |
| Azalea | Charcoal, Poke Ball, Potion, Super Potion, Escape Rope, Repel, Antidote, Parlyz Heal, Flower Mail |
| Cianwood | Potion, Super Potion, Hyper Potion, Full Heal, Revive |
| Goldenrod 2F-1 | Potion, Super Potion, Antidote, Parlyz Heal, Awakening, Burn Heal, Ice Heal |
| Goldenrod 2F-2 | Poke Ball, Great Ball, Escape Rope, Repel, Revive, Full Heal, Poke Doll, Flower Mail |
| Goldenrod 3F | X Speed, X Special, X Defend, X Attack, Dire Hit, Guard Spec., X Accuracy |
| Goldenrod 4F | Protein, Iron, Carbos, Calcium, HP Up |
| Goldenrod 5F (varies by progress) | TM ThunderPunch, TM Fire Punch, TM Ice Punch, (+TM Headbutt, +TM Rock Smash) |
| Olivine | Great Ball, Super Potion, Hyper Potion, Antidote, Parlyz Heal, Awakening, Ice Heal, Super Repel, Surf Mail |
| Ecruteak | Poke Ball, Great Ball, Potion, Super Potion, Antidote, Parlyz Heal, Awakening, Burn Heal, Ice Heal, Revive |
| Mahogany (post-Rockets) | RageCandyBar, Great Ball, Super Potion, Hyper Potion, Antidote, Parlyz Heal, Super Repel, Revive, Flower Mail |
| Blackthorn | Great Ball, Ultra Ball, Hyper Potion, Max Potion, Full Heal, Revive, Max Repel, X Defend, X Attack |

### Kanto Marts
| City | Items |
|------|-------|
| Viridian | Ultra Ball, Hyper Potion, Full Heal, Revive, Antidote, Parlyz Heal, Awakening, Burn Heal, Flower Mail |
| Pewter | Great Ball, Super Potion, Super Repel, Antidote, Parlyz Heal, Awakening, Burn Heal |
| Cerulean | Great Ball, Ultra Ball, Super Potion, Super Repel, Full Heal, X Defend, X Attack, Dire Hit, Surf Mail |
| Lavender | Great Ball, Potion, Super Potion, Max Repel, Antidote, Parlyz Heal, Awakening, Burn Heal |
| Vermilion | Ultra Ball, Super Potion, Hyper Potion, Revive, Parlyz Heal, Awakening, Burn Heal, LiteBlue Mail |
| Celadon 2F-1 | Potion, Super Potion, Hyper Potion, Max Potion, Revive, Super Repel, Max Repel |
| Celadon 2F-2 | Poke Ball, Great Ball, Ultra Ball, Escape Rope, Full Heal, Antidote, Burn Heal, Ice Heal, Awakening, Parlyz Heal |
| Celadon 3F | TM Hidden Power, TM Sunny Day, TM Protect, TM Rain Dance, TM Sandstorm |
| Celadon 4F | Poke Doll, Lovely Mail, Surf Mail |
| Celadon 5F-1 | HP Up, Protein, Iron, Carbos, Calcium |
| Celadon 5F-2 | X Accuracy, Guard Spec., Dire Hit, X Attack, X Defend, X Speed, X Special |
| Fuchsia | Great Ball, Ultra Ball, Super Potion, Hyper Potion, Full Heal, Max Repel, Flower Mail |
| Saffron | Great Ball, Ultra Ball, Hyper Potion, Max Potion, Full Heal, X Attack, X Defend, Flower Mail |
| Mt. Moon Square | Poke Doll, Fresh Water, Soda Pop, Lemonade, Repel, Portrait Mail |
| Indigo Plateau | Ultra Ball, Max Repel, Hyper Potion, Max Potion, Full Restore, Revive, Full Heal |
| Underground | EnergyPowder, Energy Root, Heal Powder, Revival Herb |

---

## Mom's Shopping (Savings System)

From `data/items/mom_phone.asm`. Mom buys items with your savings:

### Phase 1 (Initial)
| Trigger Amount | Cost | Item |
|---------------|------|------|
| 0 | 600 | Super Potion |
| 0 | 90 | Antidote |
| 0 | 180 | Poke Ball |
| 0 | 450 | Escape Rope |
| 0 | 500 | Great Ball |

### Phase 2 (Progressive)
| Savings Reach | Cost | Item/Decoration |
|--------------|------|-----------------|
| 900 | 600 | Super Potion |
| 4,000 | 270 | Repel |
| 7,000 | 600 | Super Potion |
| 10,000 | 1,800 | Charmander Doll (decoration) |
| 15,000 | 3,000 | Moon Stone |
| 19,000 | 600 | Super Potion |
| 30,000 | 4,800 | Clefairy Doll (decoration) |
| 40,000 | 900 | Hyper Potion |
| 50,000 | 8,000 | Pikachu Doll (decoration) |
| 100,000 | 22,800 | Big Snorlax Doll (decoration) |

---

## Mystery Gift Items

From `data/items/mystery_gift_items.asm`. Random item pool for Mystery Gift:

Berry, PRZCureBerry, Mint Berry, Ice Berry, Burnt Berry, PSNCureBerry, Guard Spec., X Defend, X Attack, Bitter Berry, Dire Hit, X Special, X Accuracy, Eon Mail, Morph Mail, Music Mail, MiracleBerry, Gold Berry, Revive, Great Ball, Super Repel, Max Repel, Elixer, Ether, Water Stone, Fire Stone, Leaf Stone, Thunderstone, Max Ether, Max Elixer, Max Revive, Scope Lens, HP Up, PP Up, Rare Candy, BlueSky Mail, Mirage Mail

---

## Time Capsule Catch Rate Items

From `data/items/catch_rate_items.asm`. When trading from RBY, certain catch rates are mapped to specific items:

| Catch Rate | Becomes Item |
|-----------|-------------|
| 0x19 | Leftovers |
| 0x2D | Bitter Berry |
| 0x32 | Gold Berry |
| 0x5A, 0x64, 0x78, 0x87, 0xBE, 0xC3, 0xDC, 0xFA, 0xFF | Berry |
