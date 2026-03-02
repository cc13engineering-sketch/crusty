# S-League: A Minigolf RPG — Game Design Document
(Formerly "Trap Links")

**Engine**: Crusty (Rust → WASM → Canvas 2D)
**Screen**: 480 × 720 portrait, tap-only mobile
**Genre**: Tile-based overworld RPG with minigolf-as-combat fight scenes

---

## 1. High Concept

You wander a hand-crafted overworld on foot, tile by tile, like a classic Pokémon game.
Hidden traps litter the world — spike tiles, monster dens, cursed spots.
Step on one and the screen irises inward: you are yanked into a FIGHT SCENE.

The fight scene is a self-contained minigolf hole. You are the ball.
The enemy is the hole (and everything guarding it).
Sink the shot in par or better to WIN. Fail and take damage. Die in the overworld.

There are no turn menus, no HP bars filling a menu screen.
Every single fight is a physical puzzle solved by your hands.

---

## 2. Overworld Design

### Screen Layout (480 × 720)
```
+------------------+
|  [MAP AREA]      |  600 × 600 world pixels, camera follows player
|                  |  TileMap: 32×32 px tiles, 60×60 grid = 1 screen-worth
|                  |  Multiple rooms connected via doors/paths
|                  |
+------------------+
| HP ■■■■■□  [BAG] |  120 px HUD strip at bottom
| LV 3  COINS 140  |
+------------------+
```

### Tile Types (using TileMap Custom IDs)
| ID | Name       | Effect                                       |
|----|------------|----------------------------------------------|
| 0  | Grass      | Walk freely                                  |
| 1  | Stone      | Walk freely (dungeon floors)                 |
| 2  | Wall       | Solid blocker                                |
| 3  | Water      | Impassable (unless you have Water Shoes)     |
| 4  | Sand       | Slow: move costs 2 steps instead of 1        |
| 5  | Ice        | Slide: keep moving until hitting a wall      |
| 6  | TrapFloor  | Triggers a fight on step (hidden or visible) |
| 7  | ExitTile   | Room transition                              |
| 8  | SavePoint  | Tap to save and heal 1 HP                   |
| 9  | ShopTile   | Tap to open shop                             |

### Regions (5 biomes, ~12 rooms each)
1. **Meadow Links** — Tutorial. Flat courses. Enemies: Moles, Rabbits.
2. **The Sandtraps** — Desert. Wind ZoneEffects in courses. Enemies: Scorpions, Sandworms.
3. **Frostway Greens** — Ice world. Frictionless walls. Enemies: Yetis, Ice Sprites.
4. **Cursed Caverns** — Cave dungeon. Dark rooms, TileMap tiles revealed by torch radius (Raycast). Enemies: Bats, Golems.
5. **The Final Course** — Floating sky islands. Moving platforms, anti-gravity zones. Boss: The Grand Caddy.

### Movement
- Tap a tile adjacent to the player to move there (4-directional).
- Walking animation: SpriteAnimator with "idle", "walk_n/s/e/w" clips.
- Pathfinding (A*) used for NPC movement only, not player.
- Camera: smooth lerp follow via CameraDirector.

### Trap Detection
- Each TrapFloor tile has a hidden `trap_id` encoded in its Custom tile ID (IDs 100–999 = traps).
- On player step: EventBus emits `"trap_triggered"` with `trap_id` payload.
- SceneManager pushes the FIGHT SCENE with the trap's data.
- Transition: iris-close wipe (ScreenFxStack iris effect, 0.4s) before push.

---

## 3. The Fight Scene — Core Loop

### What It Is
A minigolf hole. One shot at a time. Par is set per enemy type.
The course is 480 wide × 600 tall (leaving 120 px for the fight HUD at bottom).

```
+------------------+
|                  |
|   [GOLF COURSE]  |  Physics arena
|                  |  Walls, obstacles, the "hole" target
|   [BALL = YOU]   |
|                  |
+------------------+
| ●●●○○  AIM   PAR |  Fight HUD
| STROKE 1   [SHOT]|
+------------------+
```

### Shot Flow (one stroke)
1. **Aim Phase** (finger on screen):
   - Player places a finger anywhere on the lower half of the screen.
   - A trajectory guide arc appears from the ball: 3 ghost-dot trail (GhostTrail) showing projected path.
   - Drag direction = shot direction (opposite: drag LEFT to shoot RIGHT, like pulling a slingshot).
   - Drag distance = power. Max drag: 120 px = max power.
   - Power meter fills visually as a bar (UiCanvas arc/bar).

2. **Release** (finger lifts):
   - GestureRecognizer: `on_touch_end` triggers shot.
   - Ball gets Impulse component: `force = direction * power_factor`.
   - Power factor: `drag_pixels / 120.0 * MAX_FORCE` where `MAX_FORCE = 900.0`.
   - So a full drag fires the ball at 900 force units.

3. **Ball in Motion**:
   - RigidBody physics runs at 60 Hz with CCD.
   - PhysicsMaterial on ball: `friction = 0.15, drag = 0.04` (rolls and slows naturally).
   - Ball bounces off Solid tile walls and obstacle entities.
   - ZoneEffect tiles add wind, conveyor, sand drag mid-flight.

4. **Shot Resolution**:
   - Ball enters the hole trigger zone (ZoneEffect, radius 12 px): HOLE IN — win.
   - Ball stops moving (speed < 2.0 px/frame for 0.5 s): MISSED — stroke counted, reset to last resting spot.
   - Stroke count increments. If strokes > par + 3: DEFEAT.

5. **Repeat until hole or defeat**.

---

## 4. Aiming Mechanics — Detailed

### Touch Input (GestureRecognizer wiring)
The fight scene registers raw touch, not the gesture recognizer, to get continuous drag:

- `on_touch_start(id, x, y)`: record `aim_origin = (x, y)`. Enter aim state.
- `on_touch_move(id, x, y)`: compute drag vector `delta = origin - current` (slingshot reverse).
  - `angle = atan2(delta.y, delta.x)`
  - `power = min(delta.length / 120.0, 1.0)`
  - Update aim line and power indicator every frame.
- `on_touch_end(id, x, y)`: fire shot if `power > 0.05` (ignore accidental micro-drags).

### Aim Line Rendering
- 3 ghost dots spaced 0.1 s of simulated travel time apart.
- Each dot sized by predicted velocity at that moment (large=fast, small=slowing).
- Dots fade out (alpha: 255 → 60) to show uncertainty at range.
- Dots skip through Solid tiles (showing the ball WILL bounce there).
- Color: white dots normally; red dots when any dot lands in a hazard zone.

### Why This Feels Good on Mobile
- The slingshot pull model means your finger never obscures the ball or target.
- You aim from the BOTTOM half of screen — thumb-reach zone on portrait phones.
- No tiny joystick knob to hit precisely. Any drag from anywhere in the lower half works.
- The aim line makes physics legible even for casual players.

---

## 5. Enemy Defenses as Golf Obstacles

Each enemy type generates a course via `ProceduralGen` seeded with `enemy_id XOR level_seed`.
This makes every encounter deterministic (same enemy at same level = same course) but varied across enemies.

### Obstacle Catalog

#### A. Static Walls (TileMap Solid tiles)
- Generated via dungeon-gen room carving, then stylized per biome.
- Meadow: rounded bumper shapes (circular Colliders around wall clusters).
- Cavern: jagged L-shapes and pillars.
- Course bounds are always fully walled. No "out of bounds" — the ball just keeps rolling.

#### B. Bumpers (Entity with circular Collider, `restitution = 1.4`)
- Glowing orbs. Hit one and the ball rebounds with extra speed (amplifying).
- Colors indicate strength: yellow (1.0x), orange (1.2x), red (1.4x restitution).
- Enemy: Pinball Mole — places 6–10 bumpers in a chaotic cluster around the hole.
- AutoJuice rule: OnCollision[ball, bumper] → particles(8, orange, 80px/s, 0.3s) + screen shake(2.0, 0.05s).

#### C. Moving Blockers (WaypointPath component)
- Rectangular or circular blockers that patrol back and forth.
- WaypointPath with PingPong mode, speed = 60–120 px/s.
- You must time your shot to slip through the gap.
- Enemy: Sandworm — one long worm body (3 linked segments via PhysicsJoint distance joints) sweeping across.
- Enemy: Patrol Guard — 2 rectangular blockers moving in opposite phase.

#### D. Sand Trap Zones (ZoneEffect, Custom ID)
- Circular regions that apply heavy drag: `drag_multiplier = 8.0`.
- Ball slows to almost nothing if it enters. Forces a re-shot from inside.
- Visual: brown stippled color fill, particle drift of sand motes.
- Enemy: Scorpion — buries 2–3 sand traps between tee and hole.

#### E. Wind Corridors (ZoneEffect force)
- Rectangular strips with constant lateral force: `force_x = ±200.0`.
- Visible as animated color bands (PropertyTween cycling hue).
- You must account for drift mid-shot or aim upwind.
- Enemy: Tempest Sprite — places a wind corridor across the entire width.

#### F. Ice Panels (PhysicsMaterial friction = 0.0)
- Tile regions with no friction. Ball slides forever until hitting a wall.
- Requires precise angle calculation — any error compounds.
- Enemy: Ice Golem — fills 60% of course floor with ice.

#### G. Spinning Blades (StateMachine: idle → spinning → idle)
- Circular entities that spin in place. Contact = instant bounce with random angle.
- On collision: short invincibility for the ball (EntityFlash, 0.5s white blink).
- Enemy: The Cogfather — 3–5 blades at varying RPMs.

#### H. Teleporters (SignalEmitter/Receiver pair)
- Two portals. Ball enters one, exits the other at matching velocity.
- SignalEmitter fires when ball enters zone, SignalReceiver repositions ball.
- Enemy: Warped Witch — 2 portal pairs, often misaligned with the hole exit.

#### I. Gravity Flip Zones (ZoneEffect, force_y = -600.0 instead of +600.0)
- Final biome only. Reverses gravity in a region.
- Your ball floats upward — you aim upside-down through this zone.
- Enemy: The Grand Caddy's final phase.

---

## 6. Damage Calculation

### Par System
Each enemy type has a BASE PAR. Par is the number of strokes the fight is designed to be winnable in.

| Enemy         | Base Par | Course Complexity |
|---------------|----------|-------------------|
| Mole          | 3        | Minimal obstacles |
| Rabbit        | 2        | Wide open, short  |
| Scorpion      | 4        | 2-3 sand traps    |
| Sandworm      | 5        | Moving blockers   |
| Ice Golem     | 4        | Ice floor maze    |
| Yeti          | 6        | Many bumpers      |
| Cogfather     | 5        | Spinning blades   |
| Warped Witch  | 7        | Portal maze       |
| Grand Caddy   | 10       | All combined      |

Par scales with player level: `effective_par = base_par + floor(player_level / 3)`.
This uses `LevelCurve` with a Step shape curve — par increases at levels 3, 6, 9, etc.

### Combat Outcome Table
| Strokes vs Par | Result       | Player HP Effect         | Rewards                    |
|----------------|--------------|--------------------------|----------------------------|
| Hole-in-One    | Eagle (−2)   | Heal 1 HP                | 3× gold, Bonus XP          |
| Par − 1        | Birdie (−1)  | No damage                | 2× gold, +XP               |
| Par            | Par          | No damage                | Normal gold, +XP           |
| Par + 1        | Bogey        | −1 HP                    | Half gold                  |
| Par + 2        | Double Bogey | −2 HP                    | No gold                    |
| Par + 3+       | Defeat       | −3 HP, forced retry      | No gold                    |
| Exact Par, any | Capture opportunity | — (see Section 8)  | —                          |

Player starts with 5 HP. Max HP increases with level (LevelCurve: `max_hp` curve, +1 HP per 2 levels).

### Why Fewer Strokes = More Damage Dealt
The framing is RPG: strokes are attacks. A Hole-in-One is a critical hit.
The enemy "dies" when the ball sinks. But your efficiency of solving it is your power level.
This creates tension: a greedy aggressive shot for Birdie vs. a safe route for Par.

---

## 7. Procedural Course Generation per Enemy Type

Each enemy type has a `CourseTemplate` struct that seeds `ProceduralGen`:

```
CourseTemplate {
    seed_offset: u64,        // XORed with level_seed
    wall_density: f64,       // 0.0–1.0 → dungeon gen cave_probability
    obstacle_types: Vec<ObstacleKind>,
    obstacle_count_range: (u32, u32),
    hole_placement: HolePlacement,  // TopCenter, TopRandom, TopLeft, etc.
    tee_placement: TeePlacement,    // BottomCenter, BottomRandom
    has_wind: bool,
    wind_strength: f64,
    biome_fx: BiomeFx,       // ice friction, sand drag, etc.
}
```

### Generation Steps (using ProceduralGen module)
1. Run `cellular_automata` 3–4 times on a 30×37 tile grid to carve cave walls.
   - `cave_probability = wall_density` (0.35 for open courses, 0.55 for maze-like).
   - `birth_limit = 4`, `survival_limit = 3`.
2. Place tee at bottom center (guaranteed 3×3 open area).
3. Place hole at position determined by `HolePlacement`, verified open by BFS.
4. Run Pathfinding (A*) from tee to hole. If no path exists, carve one.
5. Seed obstacles using `SeededRng(enemy_id XOR level_seed XOR attempt_number)`.
   - Place each obstacle, verify minimum clearance from tee (48 px) and hole (36 px).
6. Apply biome `ZoneEffect` tiles (wind/ice/sand) using `Noise2D` to make organic regions.
7. Validate: run 20 simulated shots with random angles/powers. At least 1 must reach the hole. If none do, remove most restrictive obstacle and retry.

### Enemy-Specific Course Flavors
- **Mole**: `wall_density = 0.35`. Course feels like a real golf hole: one fairway, light rough, one bunker.
- **Sandworm**: Generated CORRIDOR — two long parallel walls with patrolling body segments. No branching paths.
- **Cogfather**: Open arena, `wall_density = 0.2`. Obstacles are all blades, minimal terrain.
- **Warped Witch**: `wall_density = 0.50` plus 2 portal pairs. Maze-ish with shortcuts via portals.
- **Grand Caddy (boss)**: FIXED hand-crafted course, not procedural. 10-par epic with phase-shifts: at stroke 4, new walls descend. At stroke 7, gravity flips.

---

## 8. The Pokémon Element — Capture Mechanics

### The Caddy System (Party)
After a fight you can CAPTURE defeated enemies and add them to your CADDY PARTY (max 4 active).
Caddies give you passive bonuses during fights.

### Capture Mechanic
After sinking the ball, a Capture Window opens (3 seconds):
- A "CAPTURE" button appears (big tap target, bottom center).
- Tap it to attempt capture. Success chance = `base_capture_rate × (1 / strokes_taken_ratio)`.
- `strokes_taken_ratio = actual_strokes / par`. Par or better = 1.0. Double bogey = 2.0.
- Example: Par fight vs Mole (base rate 60%): tap immediately → 60%. Birdie → min(60% × 2, 90%) = 90%.
- Failure: enemy "escapes" — no capture. You still win the fight.

### Capture Animation
- Ball flies back out of hole toward a glowing trap icon (Coroutine-driven).
- Enemy sprite appears from hole, gets sucked into a TRAP DISC item (like a Pokéball but a golf disc).
- ScreenFxStack flash (gold, 0.3s) + particle burst (gold, 80 count).

### Caddy Party (up to 4 active)
Caddies are managed on the party screen (tap BAG in HUD, then CADDIES tab).
Each captured caddy occupies one slot and provides a PASSIVE ABILITY during fights:

| Caddy         | Passive Ability                                                   |
|---------------|-------------------------------------------------------------------|
| Mole          | "Tunneler": ball passes through one wall segment per fight       |
| Rabbit        | "Quick Draw": aim-line shows 5 ghost dots instead of 3           |
| Scorpion      | "Venom Shot": first miss still counts as ½ stroke (rounded up)  |
| Sandworm      | "Worm's Eye": moving blockers briefly pause when shot is fired   |
| Ice Golem     | "Ice Breaker": ball gets +20% speed on ice tiles instead of drag |
| Yeti          | "Bumper Buddy": bumpers grant +1 shot power on hit (max 1/shot)  |
| Cogfather     | "Gear Head": spinning blades slow to 50% speed during your fight |
| Warped Witch  | "Portal Sense": portal exit shown on aim line as a dot          |

### Species Rarity & Variants
Each enemy species has 3 color variants (Normal, Shiny, Gilded):
- **Normal**: standard capture rate.
- **Shiny** (1 in 50 encounters): palette-swapped (ColorPalette module swap). Capture rate halved but caddy passive is 50% stronger.
- **Gilded** (1 in 200 encounters): gold-tinted. Unique passive: "Birdie or better = +1 bonus stroke removed from total." Catchable only with a Gilded Disc item.

Variant is determined by `SeededRng(trap_id XOR global_step_counter)`.

---

## 9. Power-Ups and Special Shots

### Inventory Items (ResourceInventory)
Player carries consumable items. Max stack per item: 9. Tap item in fight HUD to use before shooting.

| Item            | Count | Effect                                                          |
|-----------------|-------|-----------------------------------------------------------------|
| Curve Stone     | 3     | Next shot curves 15° mid-flight after 0.3s (Coroutine + force) |
| Ghost Ball      | 2     | Next shot passes through 1 obstacle (Collider tag ignore)      |
| Multi-Ball      | 1     | Shoots 3 balls simultaneously (spread ±10°). First to sink wins|
| Power Spike     | 3     | Next shot max power × 1.4                                       |
| Slow Zone       | 2     | Places a ZoneEffect drag zone on the course for 5 seconds      |
| Freeze Frame    | 2     | Pauses all moving blockers for 3 seconds                        |
| Magnet Hole     | 1     | Hole attracts ball within 40 px radius for 1 shot              |
| Sand Wedge      | 3     | Removes one sand trap tile from the course                      |
| Lucky Bounce    | 2     | Next wall bounce reflects toward hole (PropertyTween angle adj) |

Items drop from fights: better performance = higher item drop chance.
LevelCurve used to scale item drop rates with progression.

### Special Shots (Unlocked via Progression)
At levels 5, 10, 15 the player unlocks a SIGNATURE SHOT — a rechargeable special, not consumable.
One use per fight. Tap the SPECIAL button in fight HUD (glows when charged).

| Level | Name          | Mechanic                                                        |
|-------|---------------|-----------------------------------------------------------------|
| 5     | Ricochet Rush | Ball ignores normal friction for 2s, bounces 6× before stopping|
| 10    | Lob Shot      | Ball follows high parabolic arc, ignoring ground obstacles      |
| 15    | Warp Drive    | Teleports ball to click position (within 200 px radius)         |

Signature shot recharges after every 3 fights (EventBus counter).

---

## 10. Scoring, XP, and Progression

### XP Formula
```
xp_earned = base_xp × stroke_multiplier × caddy_bonus
base_xp   = enemy_base_xp[enemy_type]          // Mole=10, Yeti=40, Grand Caddy=200
stroke_multiplier:
  Eagle (−2):    3.0×
  Birdie (−1):   2.0×
  Par:           1.0×
  Bogey:         0.5×
  Double Bogey:  0.25×
  Defeat:        0.0×
caddy_bonus = 1.0 + (0.1 × active_caddies)     // max +40% with 4 caddies
```

### Level-Up (using LevelCurve)
XP thresholds follow an EaseIn curve:
- Level 2: 50 XP
- Level 5: 200 XP
- Level 10: 800 XP
- Level 15: 2500 XP
- Level 20: 6000 XP

On level-up: screen flash (gold), DialogueQueue notification "LEVEL UP! Now Lv X".
Player chooses one of 3 randomly offered upgrades (tap to select):
- +1 Max HP
- +1 Caddy Slot (max 6)
- +1 Item Carry Slot (max 5 item types)
- Unlock/upgrade a Signature Shot
- +10% XP multiplier (permanent)

---

## 11. Visual Style

### Color Palette (ColorPalette module)
Two 8-color palettes swap on biome transition:

**Meadow Links**:   `#7EC850 #5AA22A #F5E642 #D4603A #8B4513 #2B6FA8 #F5F5DC #1A1A1A`
**The Sandtraps**:  `#E8C96A #C4923A #8B4513 #D44A1E #7A3515 #A0C4FF #F5E8C8 #1A0E00`
**Frostway Greens**: `#A8D8F0 #5AB4D8 #FFFFFF #6890B0 #38607A #F0F8FF #C8E8F8 #0A1A28`
**Cursed Caverns**: `#2A1A3A #5A3A6A #8A5A8A #4A2A5A #C85A8A #8A1A3A #F0C0D0 #0A0A0A`
**Final Course**:   `#1A0A2A #6A3A9A #C870F0 #F0A830 #A030F0 #30F0C8 #F0F0FF #000010`

Biome palette transition: 0.6s PropertyTween on all color values via ColorPalette lerp.

### Entity Visuals (Renderable shapes, no external sprites required)
All entities drawn with primitive shapes (shapes module):

- **Player**: 14 px white circle with colored ring (ring color = active caddy primary color).
- **Ball (fight)**: 10 px white circle, GhostTrail of 6 fading dots when in motion.
- **Hole**: 16 px dark circle, 3 px gold ring, tiny flag sprite (triangle + line in shapes).
- **Bumpers**: 18–24 px circles, colored per strength, EntityFlash on hit.
- **Moving Blockers**: 8×32 px filled rectangles.
- **Sand Traps**: Filled circles (brown, semi-transparent at alpha 180).
- **Enemies (overworld)**: 16 px colored circles with unique shape accents (Mole = small triangles on top, Yeti = spiky outline via shapes arcs).

### Fight Scene Camera
CameraDirector: static camera (no scroll) during fights.
Course fits exactly in 480×600. No zoom needed for normal courses.
Grand Caddy boss fight: CameraDirector zooms out 0.8× to show full extended course.

### Post-FX Usage
- Vignette: always on at 15% intensity during fights (focus feeling).
- Scanlines: on in Cursed Caverns biome (retro horror feel).
- Screen shake (PostFxConfig): triggered by AutoJuice on bumper hit (intensity 2.0, 0.05s) and on defeat (intensity 8.0, 0.3s).

---

## 12. Sound Design (SoundScape module)

All audio is synthesized (no audio files shipped). SoundScape uses named palette strings.

| Event                   | Sound Cue Name     | Character                          |
|-------------------------|--------------------|------------------------------------|
| Shot fired              | `"shot_fire"`      | Whoosh + spring release            |
| Ball bounce (wall)      | `"bounce_wall"`    | Low thock                          |
| Ball bounce (bumper)    | `"bounce_bumper"`  | Bright ping                        |
| Ball in sand trap       | `"sand_drag"`      | Muffled scraping                   |
| Ball enters hole        | `"hole_in"`        | Ascending arpeggio + chime         |
| Defeat                  | `"fight_defeat"`   | Descending wail                    |
| Eagle (hole-in-one –2)  | `"eagle"`          | Fanfare 5-note                     |
| Capture success         | `"capture"`        | Magical sparkle sweep              |
| Level-up                | `"level_up"`       | 8-bit ascending chime              |
| Overworld step          | `"step_grass/stone"`| Soft footstep (biome-specific)    |
| Trap trigger            | `"trap_trigger"`   | Dramatic sting                     |

AutoJuice wires all fight-scene sounds automatically from its rule table.

---

## 13. Engine Module Mapping

| Game System                    | Engine Module(s) Used                                        |
|--------------------------------|--------------------------------------------------------------|
| Overworld tile grid            | TileMap (32px tiles), CameraDirector (follow), Pathfinding  |
| Player movement                | Transform, RigidBody (kinematic), Input/GestureRecognizer   |
| Trap detection                 | TileMap Custom tiles, EventBus, SceneManager                |
| Fight scene physics            | RigidBody, Collider (CCD), PhysicsMaterial, ZoneEffect      |
| Shot aiming                    | GestureRecognizer (raw touch drag), Impulse component       |
| Aim line preview               | GhostTrail OR custom shape dots each frame                  |
| Moving blockers                | WaypointPath (ping-pong), RigidBody kinematic               |
| Bumpers                        | Collider (restitution > 1), AutoJuice collision rules       |
| Wind / sand / ice zones        | ZoneEffect (force/drag override per tile/zone)              |
| Spinning blades                | StateMachine (idle/spin), Transform rotation                |
| Portals                        | SignalEmitter / SignalReceiver, Coroutine (reposition)      |
| Gravity flip                   | ZoneEffect (negative force_y)                               |
| Procedural course gen          | ProceduralGen (cellular automata + dungeon), SeededRng      |
| Enemy AI (overworld)           | BehaviorAI (wander/chase rules), Pathfinding                |
| Dialogue / notifications       | DialogueQueue (Notification kind, FloatingText on score)    |
| Damage and HP                  | Custom ResourceInventory (HP resource, max_capacity = max_hp)|
| XP and leveling                | ResourceInventory (XP resource) + LevelCurve               |
| Item inventory                 | ResourceInventory (per-item-type slots)                     |
| Caddy party                    | Tags + Save/Load (caddy IDs and variants stored)            |
| Capture mechanic               | Coroutine (3-second window), EventBus ("capture_attempt")   |
| Visual effects                 | AutoJuice, EntityFlash, ParticlePool, ScreenFxStack         |
| Scene transitions              | SceneManager push/pop, ScreenFxStack (iris effect)          |
| Biome palette swap             | ColorPalette + PropertyTween                                |
| Difficulty scaling             | LevelCurve (enemy HP/par/reward curves)                     |
| Save / Load                    | Save/Load (world snapshot: player pos, HP, XP, items, caddies)|
| Boss course (Grand Caddy)      | StateMachine (course phases), Coroutine (wall descent)      |
| Sound                          | SoundScape + AutoJuice sound rules                          |
| UI (HUD, buttons)              | UiCanvas (HP bar, stroke counter, item row, capture button) |

---

## 14. What Makes This Fun on Mobile — Design Rationale

### Fun Principle 1: One Meaningful Decision Per Shot
Each stroke has a genuine trade-off: safe angle (guaranteed not to roll into sand) vs. risky angle (possible birdie). With par-3 holes, there are at most 6 shots before defeat. Every shot counts.

### Fun Principle 2: The Slingshot Model Fits Thumbs
Portrait orientation, slingshot drag from anywhere in the lower 40% of screen. Your thumb never needs to reach the top of the screen. The aim line previews the full trajectory so you're not guessing.

### Fun Principle 3: Short Sessions
Every fight is 20–120 seconds. The overworld between fights is calm exploration (no time pressure). This "tension / release" alternation is perfect for mobile: pick up for one fight, put down.

### Fun Principle 4: Enemy Variety Through Obstacle Flavors
Eight distinct obstacle types mean every new enemy type you encounter in a new biome genuinely changes how you think. Scorpion fights are avoidance puzzles. Cogfather fights are reaction-timing puzzles. No two encounters feel the same.

### Fun Principle 5: The Caddy Party Creates Identity
Which caddies you catch changes your playstyle. A Rabbit caddy (extra aim dots) benefits careful players. A Yeti caddy (bumper boost) rewards risky shots. Collecting a full party and tuning it is the "build" system — achievable without deep menus.

### Fun Principle 6: Skill Expression at All Levels
Beginners: tap roughly, use items, accept bogeys, still progress.
Intermediate: learn optimal angles, time blockers, go for birdies.
Expert: Hole-in-one hunting, Shiny captures, no-item clears, challenge runs (par only or lose).

---

## 15. Missing Engine Features (Gaps for Phase 3)

These features are needed but not yet in the engine. Phase 5–6 innovation rounds should address them:

1. **Multi-layer TileMap**: Overworld needs a background (terrain) layer + an entity/object layer. Current TileMap is single-layer.
2. **Sprite sheet support for overworld tiles**: TileMap tiles have `sprite_index` field but no sprite rendering path in `TileMap::render`. Need sprite-per-tile rendering.
3. **Extended DialogueQueue**: NPC conversations need branching (player choice). Current DialogueQueue has no choice/branch support.
4. **Physics isolation per scene**: Fight scene physics must not bleed into overworld. SceneManager push/pop should carry isolated World snapshots. Currently all physics is global.
5. **UiCanvas tap detection**: UiCanvas draws UI but has no hit-test / tap callback for buttons. Fight HUD buttons (SHOT, SPECIAL, item row) need tap regions.
6. **Circular arc aim line**: GhostTrail is entity-attached. Need a standalone "preview shot path" that draws projected arc without a real entity. Could be a new rendering primitive or a temporary entity trick.
7. **Ball-specific collision response**: CCD collision handles all bodies uniformly. Need per-entity `restitution` coefficient respected in collision resolution (currently PhysicsMaterial has friction/drag but not restitution for arbitrary-pair response).
8. **Persistent SoundScape cues**: SoundScape currently plays ambient loops. Need one-shot SFX triggered by AutoJuice.

---

## 16. MVP Scope (What to Build First)

For a shippable prototype:

**Overworld**: 1 biome (Meadow Links), 1 room (15×15 tiles), 6 trap tiles, 1 save point, 1 exit. Player can walk and trigger traps.

**Fight Scene**: 1 enemy type (Mole), procedurally generated par-3 course, static walls only (no moving obstacles), sand drag zone. Shot aiming with slingshot model. Par tracking, HP loss on bogey, win on hole.

**Capture**: Always available after fight. Fixed 50% success rate (no stat formula yet). Captured Mole gives Tunneler passive (pass through 1 wall).

**HUD**: HP bar (5 pips), stroke counter, par display.

**Progression**: XP earned per fight, level-up notification (no upgrade choice yet). Save/Load player HP and position.

This MVP is completable in ~2 hours of play and validates every core loop.

---

## 17. Equipment System (from Pocket Links proposal)

Beyond consumable items, the player collects permanent equipment that modifies their golf ball and abilities.

### Balls (8 types, unlocked by level)
| Ball | Unlock | Effect |
|------|--------|--------|
| Standard | Start | No modifier |
| Rubber Ball | Lv 3 | +20% restitution on wall bounces |
| Heavy Ball | Lv 5 | +15% max force, −10% max speed |
| Spin Ball | Lv 7 | Curves 5° naturally (tunable via aim) |
| Ghost Ball | Lv 10 | Passes through first obstacle hit |
| Magnet Ball | Lv 12 | Slight pull toward hole within 60 px |
| Splitter Ball | Lv 15 | Splits into 2 on first bounce |
| Golden Ball | Lv 18 | +50% gold from all fights |

### Clubs (12 types, found in chests or bought)
Clubs modify shot physics. Equip one at a time.
- **Putter**: default, no bonus
- **Driver**: +30% max force, −10% accuracy (aim wobble)
- **Wedge**: shots arc higher, ignoring ground obstacles for 40% of travel
- **Eagle Club**: −1 effective par for damage calculation
- **Ice Pick**: ball ignores ice friction penalty
- **Sand Wedge Pro**: ball ignores sand drag penalty
- **Boomerang Club**: ball returns to tee if it doesn't sink (no stroke penalty)

### Charms (equip up to 2)
Passive effects that don't change shot physics:
- **Albatross Charm**: first shot each fight is always Birdie or better
- **Lucky Charm**: +15% capture rate
- **Shield Charm**: block 1 HP damage per fight
- **XP Charm**: +25% XP from all fights
- **Scout Charm**: reveal hidden traps in a 3-tile radius on overworld

Equipment is stored in `ResourceInventory` with named slots and persisted via `Save/Load`.

---

## 18. Endgame & Replayability

### Post-Game Content
After defeating The Grand Caddy:
- **New Game+**: replay with all equipment, enemies scaled +50% difficulty via `LevelCurve`
- **Daily Links**: procedurally generated daily challenge course using `SeededRng(date_seed)` — one 9-hole marathon, global leaderboard for minimum total strokes
- **Shiny Hunting**: complete the Shiny Caddy collection (8 shiny variants, 1-in-50 encounter rate)
- **Gilded Hunt**: ultra-rare Gilded variants (1-in-200) require special Gilded Disc items found in post-game chests
- **Challenge Mode**: par-only-or-lose runs, no-item clears, speed runs (timer via `EnvironmentClock`)

### Estimated Play Time
| Content | Time |
|---------|------|
| Main story (5 biomes × ~45 min) | ~4 hours |
| Side quests and NPC challenges | ~1 hour |
| Chest puzzles and exploration | ~30 min |
| Random encounters | ~1-2 hours |
| **First playthrough total** | **~7-8 hours** |
| Post-game (Daily Links, Shiny hunting) | Indefinite |

---

## 19. Design Pillars

1. **Skill Over Stats** — A highly skilled player can beat higher-level enemies by mastering bounce geometry. Stats matter but never override a well-aimed hole-in-one.
2. **One-Thumb Playable** — Every interaction is a single-point touch gesture. No multi-finger required. Portrait orientation, thumb-reach zone aiming.
3. **Mode Contrast** — Overworld is calm, warm, exploratory. Fight scenes are tense, focused, electric. The palette and sound swap makes transitions feel like shifting gears.
4. **Engine Composition, Not Extension** — Every mechanic maps to existing Crusty modules. The design is shaped around what the engine provides.
5. **Predictable Physics = Learnable Skill** — Consistent restitution values and CCD collision guarantee the ball always bounces as expected. The aim preview removes randomness from aiming.
6. **Short Sessions, Deep Mastery** — Each fight is 20-120 seconds. Pick up for one fight, put down. But mastering optimal angles and timing creates genuine depth.
7. **Deterministic Seeded Generation** — Every course is reproducible from its seed. Same enemy at same level = same course. This enables fair difficulty and speedrun competition.

---

*Document version 2.0 — Synthesized from 3 competing proposals (Pocket Links, Trap Links, TrapQuest) during Innovation Games Round 3.*
