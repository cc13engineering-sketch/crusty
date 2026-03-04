# Gravity Pong — Development Progress

**Branch**: `gravity-pong`
**Last session**: 2026-03-04

---

## What was done this session

### Physics Overhaul — "Make it fun"

Rewrote all core physics constants to fix the drag-dominated regime where particles moved through honey. The game was unplayable: balls took 2+ seconds to cross the screen at MAX_SPEED, gravity effects were invisible because drag killed all accumulated velocity, and waypoint capture was glacial.

#### Constants changed

| Constant | Before | After | Why |
|----------|--------|-------|-----|
| `MAX_SPEED` | 8.0 | 80.0 | 10x. World traversal ~0.2s instead of 2s |
| `BASE_DRAG` | 0.5 | 0.08 | 6x less. Particles coast and curve instead of sticking |
| `SPEED_DRAG` | 0.03 | 0.003 | 10x less. High-speed particles keep momentum |
| `REST_THRESHOLD` | 0.3 | 0.1 | Particles coast longer before stopping |
| `WELL_MIN_STRENGTH` | 900 | 2000 | Weakest wells actually attract now |
| `WELL_MAX_STRENGTH` | 7200 | 15000 | ~2x. Dramatic curved trajectories |
| `REPULSOR_MIN/MAX` | 900/7200 | 2000/15000 | Match well strengths |
| `BLACK_HOLE_GRAVITY` | 10800 | 25000 | Black holes are genuinely terrifying |
| `EDGE_RESTITUTION` | 0.8 | 0.85 | Bouncier walls, balls ricochet more |
| `WAYPOINT_TRAVEL_SPEED` | 3.0 | 15.0 | 5x faster snap to waypoint |
| `WAYPOINT_CAPTURE_RADIUS` | 200 | 150 | Slightly tighter (was 20% of world) |
| `SLING_MAX_SPEED` | 300 | 200 | Narrower gap with raised MAX_SPEED (2.5x vs 37.5x) |
| `SLING_DECAY_RATE` | 0.4 | 3.0 | Fast immunity decay (0.33s) — short burst, then gravity takes over |
| `WORMHOLE_GM` | 225 | 500 | Wormholes visibly tug nearby particles |

#### Key insight
The game had a binary feel: either *crawling* (gravity-driven, drag-crushed) or *rocketing* (sling with immunity). The fix narrows the gap — sling is 2.5x normal max instead of 37.5x, and gravity-driven particles actually move at meaningful speeds.

All 23 gravity_pong tests pass.

---

## What to do next session

### Juice features (proposed, not yet implemented)

1. **Impact Freeze** — Peggle-style micro-pause (6-8 frames) when particle scores. Slight zoom toward target. Last particle gets 30-frame celebration. *High impact, medium effort.*

2. **Combo Chain** — Consecutive scores within 2s: DOUBLE/TRIPLE/MEGA with escalating burst particles, expanding rings, screen shake. The `VisualEffect::ExpandingRing` already exists. *Medium effort.*

3. **Bumper Boost Walls** — Walls with restitution >1.0 (the `is_boost` field already exists on Wall but is never used). Adds strategic ricochet puzzle element. *Low effort.*

4. **Gravitational Lensing / Trajectory Preview** — Angry Birds-style dotted trajectory prediction during sling pull. Simulate ~90 frames forward with gravity. Makes gravity wells *readable* as tools, not mysterious forces. *High effort, high value.*

5. **Velocity Stretch** — Squash-and-stretch particles based on speed. Fast = teardrop, slow = circle. Speed lines above 100 units/frame. *Medium effort.*

6. **Bigger Sling Feedback** — 12 burst particles on sling release (currently 4), stronger screen shake, particle flash white on launch. *Low effort.*

### Structural improvements identified but not prioritized

- **Waypoint velocity semantics**: `update_waypoint` sets `p.vx/vy` as position deltas, not true velocity. Could cause trail/visual artifacts. Consider fixing if trail rendering looks wrong.
- **Sling immunity curve**: Currently linear decay. An ease-out curve (`immunity^2`) would keep high speed longer at start and drop off gently, avoiding abrupt "hit the wall" feeling.
- **Drag model simplification**: Could replace `BASE_DRAG + SPEED_DRAG * |v|` with a single linear drag coefficient. Less tuning surface, cleaner behavior. Only matters if current tuning still feels off after playtesting.

### Level rebalancing

All 10 levels were designed for the old sluggish physics. They likely need rebalancing — targets may be too easy to hit now that particles are faster and gravity is stronger. Playtest all levels and adjust:
- Target sizes
- Gravity well positions/strengths
- Wall placements
- Supernova countdowns (may be too generous now)

---

## Branch context

- `gravity-pong` — active development branch (20 commits ahead of main)
- `claude/review-game-engine-CaUoa` — old review branch that predates gravity pong, not relevant
- `main` — base branch, no gravity pong code
