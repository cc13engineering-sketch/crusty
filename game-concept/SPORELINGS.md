# SPORELINGS: The Hollow Root War
### A Tile-Based RPG with Minigolf Trap Encounters
### Built for the Crusty Engine — Mobile-First, 480×720 Portrait

---

## LOGLINE

A young mycelium tender must traverse the dying Underground Forest, solve ancient root-trap puzzles using a living spore-ball, recruit companion fungi, and confront the Blight Queen before she crystallizes the last Living Tree.

---

## WORLD & SETTING

### The Hollow Root

The world exists entirely underground — a vast network of root corridors, fungal caverns, bioluminescent grottos, and crystalline deep-caves beneath an ancient forest. The surface is never seen; rumors of it are myth. The Underground Forest was once a paradise of nutrient flow and spore trade between the seven Rootclans.

Three seasons ago, the Blight — a geometric crystalline infection — began spreading from the deepest caves, turning living mycelium into rigid, sharp mineral. The Rootclans fractured. Some fled upward. Some struck bargains with the Blight. Some simply went silent.

You are **Pip**, a junior spore-tender from the Velvet Clan, whose home burrow has just been swallowed by a Blight surge. You carry a single **Vitaspore** — a glowing living ball of compressed mycelium energy — and the knowledge that somewhere below is the Blight Queen's Heartcrystal. Destroy it, and the infection collapses.

### Tone

Stardew Valley warmth crossed with Hollow Knight melancholy. Cute character designs, surprisingly touching NPC stories, environmental tragedy that doesn't feel hopeless. The Underground feels lived-in: there are root-markets, spore-cafes, ancient libraries carved into mushroom-caps, and grumpy old fungi who have opinions about everything.

---

## CORE LOOP

### Overworld: Tile-Based Exploration

The overworld is a top-down tile map rendered at 480×720 in portrait orientation. Pip walks the root corridors. Tap-to-move: tap a destination, Pip pathfinds there via A*. The world is chunked into zones; each zone is procedurally generated from tuned parameters (see Zones section) but has fixed story landmarks.

**Encounter triggers:** Certain tile types are "trap tiles." Stepping on one triggers a scene transition into the Puzzle Scene. Trap density scales with zone danger rating and current player level.

**Overworld interactions:**
- Tap NPCs to open dialogue
- Tap items/chests to collect
- Tap zone exits to travel (unlocking conditions apply)
- Long-press any tile for a tooltip ("Root Corridor," "Blight Spore Cluster," etc.)

### Puzzle Scene: The Trap Encounter

When a trap triggers, a Pokémon-style transition (iris-wipe using SceneManager push) reveals the Puzzle Scene. This is a self-contained physics arena:

```
┌─────────────────────────────┐  ← 480px
│  [ENEMY PORTRAIT]  [HP bar] │
│  "BRACKENFANG has you!"     │
│─────────────────────────────│
│                             │
│    ┌──────────────────┐     │
│    │                  │     │
│    │   PUZZLE ARENA   │     │
│    │                  │     │
│    │     ●  (hole)    │     │
│    └──────────────────┘     │
│                             │
│  [BALL]  Shots: 3  Par: 2   │
│─────────────────────────────│
│  [AIM]  ──────────◉  [FIRE] │
│  Power: ████░░░  Dir: →↑    │
└─────────────────────────────┘
```

**Controls (tap-only):**
- **Aim**: Tap-and-drag from the ball; a dotted trajectory preview line (Raycast) shows predicted path
- **Power**: Drag distance from ball = power (clamped by MotionConstraint)
- **Fire**: Release drag to launch
- **Goal**: Get the Vitaspore ball into the enemy's "weak point hole" within par shots
- **Win**: Enemy takes damage. Reduce HP to 0 across multiple puzzle rounds to defeat the encounter.
- **Lose condition**: Run out of shots without holing out → Pip takes damage, puzzle resets

**Physics driving the puzzle:**
- `RigidBody` + CCD for the ball at high speeds
- `PhysicsMaterial` per tile: slick ice walls, sticky mushroom floors, bouncy chitin panels
- `ZoneEffect` regions: wind currents, drag pools, conveyor-belt roots
- `PhysicsJoint` for swinging obstacles: pendulum roots, spinning pinwheel barriers
- `WaypointPath` for moving platforms and patrolling blockers

**Enemy HP and multi-round structure:**
- Each enemy has 3-5 "HP segments." Each successful hole-out removes one segment.
- Between rounds the arena mutates: new obstacles appear, existing ones move, par count may change.
- Enemy has a "turn counter": if Pip takes too many shots total, enemy uses an ability (raises a new wall, reduces par, spawns a blocker).

---

## AREAS (7 Zones)

All zones use `ProceduralGen` noise + cellular automata with zone-specific seed offsets and parameters. Story landmarks are hard-placed on top.

---

### ZONE 1 — The Velvet Burrows
**Aesthetic:** Soft pink-brown tunnels, dense mycelium strands as decorative walls, gentle amber bioluminescence. Safe-feeling.

**Story Role:** Tutorial zone. Pip's destroyed home. First quest: find Elder Mosskin before fleeing.

**ProceduralGen params:**
- `Noise2D { octaves: 2, frequency: 0.04 }` — gentle rolling corridors, wide open paths
- `CellularAutomata { birth_limit: 4, death_limit: 3, iterations: 3 }` — organic blobs for mushroom clusters
- Trap density: 8% (low)
- Enemy pool: Spore Mites, Jelly Caps (tutorial enemies)

**Unlock:** Starting zone, always open.

**Landmarks:**
- Pip's Home Burrow (destroyed, intro cutscene)
- Elder Mosskin's Tea Shop (NPC hub, shop unlocks here)
- The First Root Door (gate to Zone 2, requires 1 Rootkey)

**Color palette:** `#C8956C`, `#E8C87A`, `#7A5C3E`, `#F0E0C0`

---

### ZONE 2 — The Drip Caverns
**Aesthetic:** Hanging stalactites, constant dripping water, cyan luminescent pools, slick wet floors.

**Story Role:** First real danger. The Rootclan of Damp have gone missing. Their abandoned settlement is here.

**ProceduralGen params:**
- `Noise2D { octaves: 3, frequency: 0.06 }` — more jagged, narrower corridors
- `CellularAutomata { birth_limit: 5, death_limit: 2, iterations: 4 }` — creates stalactite patterns if applied vertically
- Trap density: 15%
- Enemy pool: Stalactite Lurkers, Slime Hounds, Dripweavers

**Puzzle physics introduced:** `PhysicsMaterial.friction = 0.1` (wet floors), `ZoneEffect::Drag` pools, bouncing off curved stalactite walls.

**Unlock:** 1 Rootkey from Zone 1 boss.

**Landmarks:**
- The Damp Settlement (ruined, lore tablets)
- Merchants Drippo & Fenn (traveling shop duo, appear in later zones too)
- The Siphon Gate (deep cave entrance, locked until Zone 4 item)

---

### ZONE 3 — The Chitin Market
**Aesthetic:** Warm amber, orange lanterns, dense NPC population. Insect-harvested panels form the walls. Chaotic and busy.

**Story Role:** Major hub zone. Three Rootclans converge here in uneasy truce. Political intrigue, side quests, shop upgrades.

**ProceduralGen params:**
- Grid-heavy layout (reduced noise, more structured) to suggest built architecture
- `Noise2D { octaves: 1, frequency: 0.02 }` — large flat rooms connected by narrow passages
- Trap density: 5% (populated zone — traps are hidden more carefully)
- Enemy pool: Blight Scouts (first Blight enemies), Chitin Bandits (rogue merchants)

**Puzzle physics introduced:** `PhysicsJoint::Hinge` spinning market stall barriers, `ZoneEffect::Conveyor` root-belt systems.

**Unlock:** Completing the Damp Settlement quest in Zone 2.

**Landmarks:**
- The Grand Sporex (main market square, multiple shops)
- The Rootclan Council Chamber (story: clans arguing, Pip must broker a deal)
- Blight Containment Wall (ominous — cracks visible, foreshadowing Zone 5)
- The Deep Archive (lore library — NPC Scholar Frillian runs it)

---

### ZONE 4 — The Amber Veins
**Aesthetic:** Fossilized tree sap channels, deep gold and dark brown. Ancient, geological timescale. Fossils of unknown creatures in the walls.

**Story Role:** The Rootclans used to mine healing amber here. The Blight has begun crystallizing the veins. Race to extract the last Pure Amber before the zone seals.

**ProceduralGen params:**
- `Noise2D { octaves: 4, frequency: 0.08, persistence: 0.4 }` — tight, maze-like vein corridors
- Trap density: 22%
- Enemy pool: Amber Golems, Fossil Crawlers, Blight Crystallizers

**Puzzle physics introduced:** `ZoneEffect::Wind` (amber gas jets), sticky `PhysicsMaterial.friction = 3.0` walls (ball sticks then releases), environmental hazard tiles that activate mid-round.

**Unlock:** The Siphon Gate key from Zone 2 + Pure Amber fragment (Zone 3 quest reward).

**Landmarks:**
- The Great Amber Column (zone centerpiece, pure amber pillar)
- Old Miner's Ghost Camp (NPC Gramble, gives equipment upgrade quest)
- The Vein Collapse (scripted disaster — cutscene, triggers on story flag)

---

### ZONE 5 — The Blight Frontier
**Aesthetic:** Sharp geometry, desaturated colors, crystalline spires jutting from organic walls. Everything is half-converted. Unsettling beauty.

**Story Role:** The war front. The Blight is actively spreading here. Some infected fungi are still sentient and can be talked to. Moral choices: cure them (hard) or fight through them (easy).

**ProceduralGen params:**
- `Noise2D { octaves: 2, frequency: 0.05 }` combined with `DensityField` overlay for blight spread visualization
- `CellularAutomata { birth_limit: 6, death_limit: 1, iterations: 5 }` — aggressive, spiky growth patterns
- Trap density: 30%
- Enemy pool: Blight Husks (infected Rootclan members), Crystal Sentinels, Void Leeches

**Puzzle physics introduced:** Blight crystal walls that GROW during the puzzle (using WaypointPath-driven tile spawner), magnetic zones (`ZoneEffect::Wind` inward attraction), shatter tiles that break after one ball contact.

**Unlock:** Completing the Amber Veins storyline + Zone 4 boss defeat.

**Landmarks:**
- The Last Outpost (NPC soldiers, shop with military-grade items)
- The Infected Elder (choice: save them with Pure Amber, or fight through — different rewards)
- Threshold Gate (final zone entrance, requires all 5 Rootclan seals)

---

### ZONE 6 — The Deep Root Cathedral
**Aesthetic:** Massive scale, ancient architecture — roots the size of buildings, stained glass panels of spore-light, religious/spiritual tone. The oldest part of the Underground.

**Story Role:** The Rootclans built their civilization around the roots of the Living Tree above. The Cathedral holds the Tree's lowest root-tips. The Blight is here too, but has not yet reached the Tree. Last safe stronghold — and where the five Rootclan leaders must reunite for the final ritual.

**ProceduralGen params:**
- Mostly hand-designed (low procedural noise) — cathedral feel requires intentional layout
- `Noise2D { octaves: 1, frequency: 0.01 }` — subtle variation in "stained glass" light patches
- Trap density: 12% (holy ground — traps are ancient protective mechanisms, not Blight)
- Enemy pool: Ancient Guardians (defending the Cathedral), Corrupted Acolytes (Blight-touched believers)

**Puzzle physics introduced:** Gravity-flip tiles (using `ZoneEffect::Wind` pointing up), holy-light zones that boost ball speed, multi-layer arenas (ball must pass through vertical gates on different Y levels).

**Unlock:** All five Rootclan seals.

**Landmarks:**
- The Rootclan Altar (final ritual site — story climax setup)
- Hall of the Founders (lore murals, NPC historian)
- The Root Descent (entrance to Zone 7)

---

### ZONE 7 — The Heartcrystal Depths
**Aesthetic:** Pure Blight — everything geometric, angular, cold blue-white and black. No organic material. Crystalline labyrinths. Final dungeon.

**Story Role:** The Blight Queen's domain. Linear path to her chamber. No shops, no NPCs. Just Pip, the Vitaspore, and whatever items were brought.

**ProceduralGen params:**
- No procedural generation — fully hand-designed dungeon
- Three fixed sub-areas: The Crystal Maze (navigation puzzle), The Echo Chambers (gauntlet of repeat encounters), The Heartcrystal Chamber (boss arena)
- Trap density: 45% — every other room is a trap

**Unlock:** Completing the Cathedral ritual (story gate — automatic after Zone 6 boss).

---

## ENEMY TYPES (15 Total)

Each enemy has: a **puzzle layout template**, a **signature mechanic**, and **lore flavor**.

---

### Tier 1 — Tutorial & Zone 1

**1. SPORE MITE**
- *Lore:* Tiny, round, perpetually startled. Lives in mycelium fluff. Completely harmless in groups... until it isn't.
- *Puzzle layout:* Single straight lane, hole at the far end. Par 1. Pure tutorial — teaches aim and power.
- *Signature mechanic:* None. Stationary. Clean introduction.
- *HP:* 2 segments
- *Enemy turn:* Releases a puff of spores (adds one wall tile to the lane)

**2. JELLY CAP**
- *Lore:* A mushroom cap filled with luminescent jelly. Wobbles dramatically. Actually quite shy.
- *Puzzle layout:* Wide arena with a central jelly pillar (bouncy `PhysicsMaterial`). Hole is behind the pillar — must use a bounce shot.
- *Signature mechanic:* Jelly Wall — one wall is super-bouncy, requires angled shots.
- *HP:* 2 segments
- *Enemy turn:* Repositions the jelly pillar one step toward the ball

---

### Tier 2 — Zone 2 Enemies

**3. STALACTITE LURKER**
- *Lore:* Hangs upside down from cave ceilings, drops suddenly. Has never understood why everyone finds this rude.
- *Puzzle layout:* Stalactites as vertical obstacles that periodically lower and raise (`WaypointPath::PingPong`). Hole is in the back row.
- *Signature mechanic:* Timed obstacles — must shoot through a gap in the stalactite cycle.
- *HP:* 3 segments

**4. SLIME HOUND**
- *Lore:* A loyal but confused creature made of cave slime. Wants to fetch your ball but doesn't understand why you need it back.
- *Puzzle layout:* Arena divided by two `ZoneEffect::Drag` slime pools. Path requires threading through narrow non-slime corridors.
- *Signature mechanic:* Slime pools — ball dramatically slows, requires high-power shot to escape
- *HP:* 3 segments
- *Enemy turn:* Expands one slime pool by two tiles

**5. DRIPWEAVER**
- *Lore:* A spider-like creature that weaves nets from condensed water droplets. Considers the nets art.
- *Puzzle layout:* Multiple water-thread barriers with gaps. Barrier positions shuffle between rounds using `StateMachine`.
- *Signature mechanic:* Net barriers — touching one costs one shot (ball must retry from current position)
- *HP:* 3 segments

---

### Tier 3 — Zone 3-4 Enemies

**6. BLIGHT SCOUT**
- *Lore:* An advance agent of the Blight. Small, fast, crystalline. The first sign something is very wrong.
- *Puzzle layout:* Open arena with three rotating crystal pillars (`PhysicsJoint::Hinge` spinners). Hole is exposed between their rotation cycles.
- *Signature mechanic:* Spinning barriers — rotation speed increases each round
- *HP:* 4 segments
- *Enemy turn:* Adds one more rotating crystal to the field

**7. CHITIN BANDIT**
- *Lore:* A rogue merchant who decided trading was for suckers. Has a great laugh. Actually kind of likeable.
- *Puzzle layout:* Divided arena — ball must pass through a one-way valve (chitin flap). Two-section puzzle: get ball from left chamber to right chamber via the valve, then into the hole.
- *Signature mechanic:* Two-chamber layout requires planning the full trajectory before shooting
- *HP:* 3 segments

**8. AMBER GOLEM**
- *Lore:* Ancient guardian, crystallized amber from a forgotten age. Moves slowly but cannot be stopped.
- *Puzzle layout:* Huge arena. A massive slow-moving Amber Golem entity (`WaypointPath::Loop`) blocks the center. Must time shots around its path.
- *Signature mechanic:* Moving blocker — the Golem entity is a solid `Collider` block that moves through the arena
- *HP:* 5 segments
- *Enemy turn:* Golem speeds up for the next round

**9. FOSSIL CRAWLER**
- *Lore:* A prehistoric creature re-animated by ambient Blight energy. Confused by everything. Attacks by accident.
- *Puzzle layout:* Arena with breakable fossil tiles. Ball must shatter certain tiles to open the path to the hole. Order matters.
- *Signature mechanic:* Tile destruction puzzles — breaking wrong tiles collapses a path
- *HP:* 4 segments

---

### Tier 4 — Zone 5-6 Enemies

**10. BLIGHT HUSK**
- *Lore:* A converted Rootclan member, still faintly aware. Fights you but their attacks seem hesitant. One of the game's most emotionally striking encounters.
- *Puzzle layout:* Blight crystal walls that grow one tile per shot (`Coroutine`-driven tile spawner). The hole slowly fills in — must complete quickly.
- *Signature mechanic:* Shrinking arena — time pressure creates urgency
- *HP:* 4 segments
- *Enemy turn:* Spawns a new crystal wall segment

**11. CRYSTAL SENTINEL**
- *Lore:* A Blight creation, not a converted being. Pure machine logic. Completely remorseless. Very tidy.
- *Puzzle layout:* Reflective crystal walls that deflect the ball at perfect angles. Hole is only reachable via a multi-bounce path.
- *Signature mechanic:* Reflection puzzles — like billiards, must calculate bounces (trajectory preview via `Raycast`)
- *HP:* 5 segments
- *Enemy turn:* Rotates one wall 45 degrees, breaking the previous solution

**12. VOID LEECH**
- *Lore:* Born in the deepest Blight zones. Drinks light. Has never been full. Deeply relatable.
- *Puzzle layout:* Arena with three gravity wells (`ZoneEffect::Wind` pointing inward toward entity positions). Ball must arc between them.
- *Signature mechanic:* Gravity-well navigation — requires careful power control and angle selection
- *HP:* 4 segments

**13. ANCIENT GUARDIAN**
- *Lore:* Programmed by the Cathedral's founders to protect the sacred roots. Has been waiting 400 years. Very thorough.
- *Puzzle layout:* Rotating arena — the entire puzzle board rotates slowly each round using a `Coroutine`. What was left is now down. Par adjusts.
- *Signature mechanic:* Rotating arena orientation — player must mentally reorient each round
- *HP:* 5 segments

---

### Tier 5 — Rare/Special

**14. ECHO SHADE**
- *Lore:* A creature made of resonant sound trapped in crystal. Appears only in the Echo Chambers of Zone 7. Mirrors your shots back at you.
- *Puzzle layout:* Mirrored arena. A copy of your ball launches simultaneously from the opposite side after each shot. Both balls are live; yours must reach the hole before the echo does (echo reaches an "anti-hole" — if echo scores, Pip takes damage).
- *Signature mechanic:* Racing against your own shot copy — requires efficient paths
- *HP:* 5 segments

**15. SPORE TITAN** *(rare encounter, random spawn 2% chance in Zones 4-6)*
- *Lore:* A god-sized spore creature. Hasn't noticed you specifically. You are an inconvenience.
- *Puzzle layout:* Massive arena, multiple holes to unlock in sequence. Ball must complete a 4-step relay: enter hole A → ball relaunches → enter hole B → relaunches → final hole C.
- *Signature mechanic:* Relay puzzle — each sub-hole completion changes the arena configuration for the next stage
- *HP:* 6 segments
- *Special reward:* Drops a Titan's Spore (unique equipment modifier)

---

## BOSS ENCOUNTERS (5 Bosses)

Bosses are accessed via zone boss rooms. Unlike regular encounters which use `SceneManager::push` for a clean exit, boss encounters are persistent — the boss has complex multi-phase behavior driven by `StateMachine` and `Coroutines`.

**Boss puzzle structure differences:**
- 3 distinct phases with full arena redesigns between them
- Boss has a "counter move" it uses every N shots (not just on timeout)
- Music shifts (SoundScape intensity layers)
- Win condition sometimes changes between phases
- First attempt is always "scripted" — hints guide you; subsequent runs are harder

---

### BOSS 1 — MOSSKIN'S BANE (Zone 1 Boss)
*Location:* The Root Door Chamber

*Lore:* A massive Trap-Root, ancient as the Burrows themselves, animated by decades of absorbed frustration from unwanted visitors. Elder Mosskin respects it. You must not damage the Root — you must redirect it.

*Puzzle concept:* The ball must not hit the boss directly. Instead, hit special "pressure nodes" on the arena walls in sequence. Each node lit = one HP segment removed. Nodes shift positions between phases.

*Phase 1:* Three nodes, wide-open arena. Easy geometry, low par.
*Phase 2:* Four nodes, Root begins swinging through arena (`PhysicsJoint::Rope`). Nodes only stay active for 3 seconds before moving.
*Phase 3:* Five nodes. Root moves faster. Two nodes must be hit in a single shot via bounce.

*Teaches:* Multi-bounce and planning ahead.
*Reward:* Rootkey #1, "Tendril" ball modifier (ball can curve very slightly on command).

---

### BOSS 2 — THE SIPHON QUEEN (Zone 2 Boss)
*Location:* The Siphon Gate Chamber

*Lore:* A crystallized jellyfish-like creature that has claimed the cavern's drainage network. She pulls everything toward her center. Including your ball. Especially your ball.

*Puzzle concept:* Central gravity well (`ZoneEffect::Wind` at extreme intensity) makes all shots curve toward the boss. The hole is BEHIND the boss — must orbit the ball around the gravity well without it being swallowed.

*Phase 1:* Moderate pull. Wide berth possible. Par 2.
*Phase 2:* Pull intensifies. Safe orbit path is narrower. A second weaker gravity well appears at the opposite wall.
*Phase 3:* Both wells are active at high intensity. Ball must slingshot around the boss using her own gravity. Par 3, but you can do it in 2 if you're daring (secret achievement).

*Reward:* Rootkey #2, "Drift" ball modifier (reduced pull from gravity zones — 40% resistance).

---

### BOSS 3 — MERCHANT KING FERROUS (Zone 3 Boss)
*Location:* The Grand Sporex Black Market

*Lore:* A corrupt merchant who secretly sells Blight artifacts. Theatrical, overdressed, fights using purchased defenses. A delicious villain who refuses to take the fight seriously until Phase 3.

*Phase 1:* "Just business." Talks during the fight via `DialogueQueue::FloatingText`. Arena full of bouncy market stalls. Hole is easy to find, hard to reach cleanly. He deploys one "hired goon" blocker (moves randomly, `BehaviorAI::RandomWalk`).
*Phase 2:* "You've ruined my quarterly projections." Three goon blockers. Arena gets purchase-price labels on every element (visual gag). Par reduced by one. He starts deflecting shots with a "bribe wall" (`SignalEmitter` trigger).
*Phase 3:* "Fine. Personal." Drops the theatrical act. Full Blight artifact deployment: crystal growers, gravity shards, rotating blades. His "true" puzzle — very tight, high skill ceiling.

*Reward:* Rootkey #3, "Commerce" passive (all future shop items 15% cheaper).

---

### BOSS 4 — THE CRYSTALLIZER (Zone 5 Boss)
*Location:* The Threshold Gate Chamber

*Lore:* Not a creature — a Blight process given physical form. The mechanism by which living things become crystal. It is not malicious. It simply converts.

*Puzzle concept:* The arena is ACTIVELY CRYSTALLIZING during the fight. Every shot, more tiles become solid crystal walls. The arena shrinks. There is no "reset" between phases — the crystals you couldn't avoid accumulate. Each phase is harder because of Phase 1's consequences.

*Phase 1:* Wide arena, one crystal grows per shot. Hole at far end. Comfortable par 2.
*Phase 2:* Arena is ~70% what it was. Two crystals grow per shot. Hole moves. Par 2.
*Phase 3:* Arena is ~40% of original. Three crystals per shot. Hole position changes each shot. Must complete in par 1. (Think: the ball must go IN immediately, no exploratory shots.)

*The design lesson:* This boss punishes waste. Every unnecessary shot has permanent consequences. It recontextualizes everything the player has learned.

*Reward:* Rootkey #4, "Anti-Blight" coating (ball cannot be slowed by Blight effects — immunity to crystal-zone deceleration).

---

### BOSS 5 — THE BLIGHT QUEEN (Zone 7 Final Boss)
*Location:* The Heartcrystal Chamber

*Lore:* She was once a Rootclan Elder named Verruca, who went too deep seeking a cure for a sickness that killed her clan. The Blight offered her a bargain: crystallize yourself, and you can stop the pain. She did. The Blight used her as its consciousness. She is both villain and victim. The game's dialogue with her is the emotional heart of the story.

*Puzzle concept:* Three distinct phase arenas, each referencing a previous zone.

*Phase 1 — "The Memory of the Velvet Burrows":* Familiar warm colors, but wrong. Mycelium walls are crystallizing mid-fight. Pip sees home being destroyed again. Mechanically: Mosskin's Bane style node-hitting (callback to Boss 1), but nodes are infected-looking and pulsing.

*Phase 2 — "The Siphon Maze":* The arena becomes the Siphon Queen's gravity-well structure (callback to Boss 2), combined with the Crystallizer's growing walls. Gravity + shrinking. The Queen begins speaking to Pip through `DialogueQueue` during this phase. "You know what I lost. Don't pretend you don't feel it too." Emotionally manipulative. Mechanically ruthless.

*Phase 3 — "The Heartcrystal":* The Queen's true form. The arena is the inside of a massive crystal lattice. The ball must ricochet through a precise multi-bounce path to reach the Heartcrystal at the center. But the path exists — it was designed to be found, a hidden beauty inside all the destruction. Par 4. If you complete it in 2 (the "perfect solution"), you unlock the True Ending: the Queen's consciousness separates from the Blight, and Verruca is laid to rest — the Blight continues spreading but slower, and Pip has hope. Normal ending: Heartcrystal shatters, Blight collapses, but something important is lost with it.

*Reward:* Victory. The game's ending sequence. Credits roll over the Underground Forest slowly regrowing, lit by the Vitaspore's glow.

---

## ITEMS & EQUIPMENT

Items modify the Vitaspore ball, Pip's stats, or the puzzle rules. Divided into three categories: Ball Modifiers, Pip Abilities, and Consumables.

### Ball Modifiers (Equip one at a time, found/purchased)

| Item | Effect | Source |
|---|---|---|
| **Tendril Wrap** | Ball can curve ±15° mid-flight (tap left/right during flight) | Boss 1 reward |
| **Drift Coat** | 40% resistance to gravity-well pull | Boss 2 reward |
| **Sporeblast Core** | On first wall contact, ball explodes into 3 sub-balls for 0.5s | Zone 4 shop |
| **Chitin Shell** | Ball can break one destructible tile per shot at no extra cost | Zone 3 shop |
| **Velvet Membrane** | Ball sticks to ANY surface for 1 second before releasing — lets player aim mid-bounce | Zone 5 shop |
| **Amber Heart** | Ball glows — reveals hidden path tiles invisible to normal vision | Zone 4 quest reward |
| **Echo Skin** | Ball leaves a ghost trail (`GhostTrail`) — trail has mild physical collision, blocks Blight growth | Zone 6 quest |
| **Iron Core** | Ball is much heavier — hard to pull by gravity wells, smashes through breakable tiles, but loses range | Zone 5 merchant |
| **Moonspore Lining** | Ball is much lighter — travels 50% further per same power, but gravity wells are devastating | Zone 2 shop |
| **Titan's Spore** | Massive ball — plugs entire corridors, guaranteed wall bounces, but struggles to enter small holes | Rare drop from Spore Titan |

### Pip Abilities (Permanent unlocks from quests/story)

| Ability | Effect | Source |
|---|---|---|
| **Second Shot** | Once per encounter, undo the last shot (costs 1 Amber Shard) | Zone 3 Council quest |
| **Root Sense** | See trap tiles on the overworld highlighted in orange | Elder Mosskin quest |
| **Quick Mend** | After winning an encounter, heal 1 HP | Zone 4 Gramble quest |
| **Spore Surge** | Once per encounter, increase ball power cap by 30% for one shot | Zone 5 outpost quest |
| **Anti-Blight** | Immunity to Blight zone deceleration on ball | Boss 4 reward |
| **Tender's Eye** | Trajectory preview extends to show 2 bounces instead of 1 | Zone 6 Scholar Frillian quest |

### Consumables (Limited inventory, found in chests / purchased)

| Item | Effect | Cost/Source |
|---|---|---|
| **Amber Shard** | Powers the Second Shot ability; also currency for some shops | 12g or chest drops |
| **Spore Puff** | Adds 1 extra shot to current puzzle (immediately) | 30g |
| **Root Tea** | Heals Pip 1 HP (used in overworld) | 20g, Elder Mosskin's shop |
| **Clarity Leaf** | Extends trajectory preview for the next shot only | 15g |
| **Blight Ward** | Prevents one Blight crystal growth on next enemy turn | 25g |
| **Lucky Spore** | Next shot counts for -1 shot toward par | 40g, rare |

---

## NPCS AND DIALOGUE

All dialogue is rendered via `DialogueQueue::Dialogue` at the portrait bottom, with the NPC's sprite portrait displayed. Each NPC has 3-5 dialogue trees (initial meeting, after progress, after zone boss, after final boss, optional quest chain).

### ELDER MOSSKIN
*Role:* First mentor, hub shopkeeper, comic relief
*Location:* Zone 1 → later relocates to Zone 6 Cathedral

*Voice:* Grandmotherly, highly opinionated about tea, secretly devastated by the Blight.

Sample dialogue (first meeting):
> "Oh! Oh, a surviving Velvet child! Pip, is it? Well. Come inside. I was just making my third cup and — yes, yes the Blight is terrible, we can discuss that while the kettle's on."

Sample dialogue (after Zone 3):
> "You negotiated with the Chitin Clans? Actual negotiation? Not shouting and hoping? I may have underestimated you, dear."

*Shop inventory:* Root Tea, Clarity Leaf, modest equipment. Upgrades stock as zones progress.

---

### DRIPPO & FENN
*Role:* Traveling merchant duo, appear in Zones 2, 3, 5, 6
*Voice:* Drippo (tall, anxious) and Fenn (short, wildly optimistic). Bicker constantly. Devoted to each other.

Sample dialogue (first meeting, Zone 2):
> **Drippo:** "We are NOT taking on another client after what happened in the Amber Veins —"
> **Fenn:** "That was technically fine! Nobody lost an important limb!"
> **Drippo:** "...Hello, traveler. We are DEFINITELY OPEN FOR BUSINESS."

*Shop:* Rotating stock of consumables and mid-tier ball modifiers. Stock differs each zone.

---

### SCHOLAR FRILLIAN
*Role:* Lore librarian, quest giver, exposition engine — but a funny, passionate one
*Location:* Zone 3 Deep Archive, then Zone 6

Sample dialogue:
> "The Blight is not evil, strictly speaking! Evil implies intent! It is more like... a geological process that is personally inconvenient to everyone who isn't a crystal."

*Quest chain:* Collect 5 lore tablets from fallen Rootclan settlements → reward: Tender's Eye ability + full backstory of Verruca/Blight Queen.

---

### GRAMBLE
*Role:* Retired miner, gruff exterior, soft interior
*Location:* Zone 4 Old Miner's Camp

Sample dialogue:
> "You want the old chitin drill? Fine. But you're coming back and telling me what's happened to the Great Column. Nobody tells me anything since my knees went."

*Quest:* Help clear Blight from his mine section → reward: Chitin Shell modifier, Quick Mend ability.

---

### THE INFECTED ELDER (Zone 5)
*Role:* Moral choice NPC, partially converted to Blight
*Dialogue condition:* Changes based on whether player has Pure Amber from Zone 4

With Pure Amber:
> "I... I can feel it. The cold is losing... some kind of warmth is returning... tell Mosskin that I... I'm sorry about the tea I borrowed in '43. I never gave it back."

Without Pure Amber:
> "Run, small tender. There's nothing here that remembers you."

---

### VERRUCA (The Blight Queen, Phase 2 dialogue)
*Role:* Final antagonist, genuinely tragic figure
*Delivery:* Lines appear as `DialogueQueue::FloatingText` mid-puzzle, timed to near-misses

> "Every time you almost made it. Every time you told yourself, one more try. That's what I told myself, too."

> "I didn't choose to become this. I chose to stop hurting. Do you think those are different things?"

> "Your ball is made of life. Life doesn't survive down here. I can show you something better."

---

## PROGRESSION GATES

Progression is gated by **Rootkeys** (boss drops), **story flags** (quest completions), and **ability prerequisites** (some paths require specific Pip abilities to navigate).

```
Zone 1 (open)
  └→ [1 Rootkey] → Zone 2
       └→ [Damp Settlement Quest complete] → Zone 3
            └→ [Council Quest complete + 2 Rootkeys] → Zone 4
                 └→ [Vein Quest complete + Siphon Gate key + 3 Rootkeys] → Zone 5
                      └→ [All 5 Rootclan seals] → Zone 6
                           └→ [Cathedral Ritual complete + 4 Rootkeys] → Zone 7
```

**Side gates (optional paths):**
- The Siphon Gate in Zone 2 (bonus lore area, requires Zone 4 item — accessible out-of-order on revisit)
- The Fossil Gallery in Zone 4 (requires Amber Heart ability to see hidden entrance)
- The Cathedral Undercroft in Zone 6 (requires all 5 lore tablets from Frillian quest)
- The Echo Chambers in Zone 7 have a skip path if player solved the Echo Shade encounter on first attempt

---

## PROGRESSION SYSTEMS

### Player Level (LevelCurve integration)

Pip gains EXP from completing puzzle encounters. Level is used to scale:

```rust
// LevelCurve configs (pseudocode, production values)
"trap_density_encounter_rate": Linear[(1, 0.8), (5, 1.0), (10, 1.2)]  // harder encounters as you level
"enemy_hp_segments": EaseIn[(1, 2.0), (10, 5.0)]                       // more segments at high levels
"enemy_turn_frequency": EaseOut[(1, 5), (10, 3)]                       // enemy acts more often
"consumable_drop_rate": Linear[(1, 0.15), (10, 0.08)]                  // fewer freebies later
```

Level is NOT a power gate — it scales encounter difficulty, not whether you CAN access zones. The world is about keys and story, not grinding.

### Gold and Shopping

Gold drops from encounters (scaled: 5-30g based on enemy tier). Shops update inventory at zone transitions. The economy is deliberately tight — players must choose between consumables (safety net) and equipment (power).

### Pip HP System

Pip has 5 HP. Failing a puzzle costs 1 HP. Reaching 0 HP shows the Game Over screen (`GameFlow::GameOver` state), then respawns Pip at the last rest site (zone entrance mushroom bench) with 2 HP. No permadeath. The game is meant to be finished.

### Save System

`Save/Load` snapshots: current zone, Pip position, HP, gold, equipped modifier, inventory, completed quest flags, story flags, unlocked zones. Auto-save on zone transition + manual save at rest sites.

---

## ENDGAME

### Victory Condition

Defeat the Blight Queen. Two endings:

**Standard Ending:** Heartcrystal shattered (normal), Blight begins slow recession. The Underground Forest will recover in a generation. Pip is thanked by the surviving Rootclans. Elder Mosskin cries and pretends she has something in her eye.

**True Ending:** Complete Phase 3 of the Blight Queen in par 2 or better (the "perfect solution"). Verruca's consciousness separates cleanly from the Blight before the Heartcrystal shatters. She dies peacefully. The Blight recession is faster. A late-game cutscene shows a small seedling of light breaking through the Underground ceiling — the Living Tree is regrowing a new root, pointing toward the surface. The final dialogue is Pip, looking up at it: "Maybe someday."

### Post-Game Replayability

After credits, a **New Journey+** mode unlocks:
- All zones procedurally regenerated with new seeds
- Enemy positions randomized across zone-appropriate tile pools
- Boss arenas get one new mechanic each
- Two new "hidden" enemies appear in the rotation: Echo Shade (now available in Zones 4-6, not just 7) and a new enemy, the **Mirror Golem** (reflects the ball's previous trajectory — requires breaking your own patterns)
- Achievement system (tracked via `Save/Load` global strings): "Par Perfection" (every encounter at or under par), "Pacifist" (cure all Blight Husks), "Collector" (find all lore tablets), "Speed Root" (complete without buying any consumables)

### The Challenge Rooms

Unlocked after Zone 3, seven "Challenge Rooms" appear as optional purple-marked tiles in each zone:
- Rooms from defeated enemies but with par reduced by 1 (elite mode)
- Three-star rating (par / par-1 / hole-in-one where possible)
- Three-star completion rewards cosmetic ball modifiers (glow colors, trail effects using `GhostTrail` color params)
- Full clear of all challenge rooms unlocks the "Vitaspore Legend" cosmetic skin: a rainbow prismatic ball with a starfield trail

---

## ENGINE MAPPING: FEATURE-TO-SYSTEM

How each major game element maps to the Crusty engine:

| Game Feature | Engine System |
|---|---|
| Overworld tile exploration | `TileMap` + `Pathfinding` (A* tap-to-move) |
| Zone procedural generation | `ProceduralGen` (Noise2D + CellularAutomata + DungeonGen) |
| Encounter trap tiles | `TileMap::Custom(TRAP_ID)` + tile-collision detection |
| Scene transition (encounter start) | `SceneManager::push` + iris-wipe `ScreenFxStack` |
| Ball physics | `RigidBody` + CCD + `PhysicsMaterial` per surface |
| Trajectory preview | `Raycast` circle-cast, dotted line drawn each frame |
| Zone effects (wind/gravity/drag) | `ZoneEffect` with Wind/Drag modes |
| Rotating obstacles | `PhysicsJoint::Hinge` + `WaypointPath` |
| Moving platforms | `WaypointPath::PingPong` |
| Growing crystal walls | `Coroutine` step-sequence tile spawner |
| Arena rotation (Guardian) | `Coroutine` + `CameraDirector` pan |
| Enemy HP segments | `ResourceInventory` (bounded HP resource) |
| Enemy turn behavior | `BehaviorAI` + `StateMachine` |
| Multi-phase boss behavior | `StateMachine` (3-state FSM with transitions) |
| NPC dialogue | `DialogueQueue::Dialogue` + portrait sprite |
| Mid-puzzle boss speech | `DialogueQueue::FloatingText` |
| Damage flash | `EntityFlash` hit-flash |
| Ball trail | `GhostTrail` ring buffer |
| Particle effects (explosion) | `ParticleSystem` burst |
| Zone ambience | `SoundScape` layered audio |
| Encounter juice (win/lose) | `AutoJuice` + `ScreenFxStack::Shake` |
| Player progression | `LevelCurve` difficulty scaling |
| Inventory | `ResourceInventory` + `Save/Load` |
| Quest flags | `GameState` global string map |
| Minimap | `UiCanvas` overlay layer |
| Challenge room ratings | `GameState` + `Save/Load` |

---

## VISUAL DESIGN

### Color Palettes (per zone, fed to `ColorPalette`)

```
Zone 1 (Velvet Burrows):   bg=#2A1A0E  wall=#7A5C3E  accent=#E8C87A  glow=#C87A50
Zone 2 (Drip Caverns):     bg=#0A1A22  wall=#1E4A5A  accent=#5ACFDC  glow=#2A8A9A
Zone 3 (Chitin Market):    bg=#1A1005  wall=#5A3A10  accent=#E08020  glow=#FFA040
Zone 4 (Amber Veins):      bg=#1A0E00  wall=#7A4A10  accent=#D4A030  glow=#E8C060
Zone 5 (Blight Frontier):  bg=#050A12  wall=#1E2A40  accent=#8AB0E0  glow=#4080D0
Zone 6 (Cathedral):        bg=#050510  wall=#302860  accent=#A080E0  glow=#D0B0FF
Zone 7 (Heartcrystal):     bg=#000005  wall=#0A0A20  accent=#60A0FF  glow=#B0D0FF
```

### Vitaspore Ball Visual

Warm amber-yellow core, soft glow ring (post-fx vignette inverted — inner light), ghost trail in warm orange. On modifier equip, the ball's glow color shifts to reflect the modifier. Chitin Shell = brown ring, Moonspore = silver shimmer, Titan's Spore = giant purple ball.

### Typography

All text via `BitmapText` in the engine's bitmap font renderer. Zone name announcements use a large center-screen font with a zone-color tint and the iris-open transition.

---

## SESSION FLOW: WHAT 30+ MINUTES ACTUALLY LOOKS LIKE

**Minutes 0-5:** Zone 1. Tutorial. Meet Mosskin. Learn tapping. First two encounters (Spore Mite, Jelly Cap). Player feels smart immediately — both are solvable on first attempt with minor thought. Pip gains a Clarity Leaf from a chest. Boss 1 fight: Mosskin's Bane. Engaging, feels fair, likely 2-3 attempts. Rootkey 1 earned.

**Minutes 5-12:** Zone 2. Atmosphere shifts cooler. Three new enemies with three new mechanics — slime pools, stalactites, net barriers. Player picks up Moonspore Lining from Drippo/Fenn. Makes a tricky gravity encounter very manageable. Mid-zone discovery: the abandoned Damp Settlement (lore + sadness). Boss 2 gravity fight — challenging. Second attempt most likely.

**Minutes 12-18:** Zone 3 market. This is the "breathe" moment — lots of NPCs, shopping, quests accepted, Frillian met. The Chitin Bandit encounter teaches two-chamber thinking. Boss 3 Merchant King — first genuinely funny boss, theatrical, eventually hard. Player has now seen: steering (Tendril), gravity (Drift), two-phase layout (Bandit), growing obstacles.

**Minutes 18-25:** Zone 4. The game gets serious. Crystal Sentinels demand real angle math. The Amber Golem tests timing. Mid-zone: the Vein Collapse scripted sequence — sudden darkness, physics chaos, Pip slides down a new shaft. Zone 5 gate opens. Player is invested in the story.

**Minutes 25-30+:** Zone 5. The Blight Husks are the emotional gut-punch. The shrinking arena Blight Husk encounter — first time the player might actually fail 3+ times. The Crystallizer boss is a revelation: past choices mattered. Player buys Blight Ward from the Last Outpost, thinks carefully before each shot. They are playing differently than they were in Zone 1.

**30+ minutes onward:** The Cathedral, the Echo Shade encounter, the Blight Queen's three phases, the realization that the final puzzle has a beautiful solution inside it. Credits. Either satisfaction or determination to find the True Ending.

---

*SPORELINGS: The Hollow Root War. A game about losing things you love, and finding the path through anyway. Built in Rust, played on your phone during a commute, finished with more feelings than you expected.*
