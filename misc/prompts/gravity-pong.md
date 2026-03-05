# Gravity Pong — Crusty Engine Implementation Guide

> **Purpose**: Source of truth for implementing Gravity Pong as the first game built on the Crusty engine. Covers mechanics, physics, and visual design. No UI chrome, no level builder, no daily puzzle system.

---

## 1. Game Concept

Gravity Pong is a **single-player physics puzzle** where you guide colored particles into target zones using gravitational manipulation. Entities are pre-arranged by the level. Your tools are a **waypoint** (tap to place — nearby particles fly to it and lock, ignoring all forces) and a **slingshot** (grab a locked particle, pull back, release to launch). Hold and drag to preview your waypoint's reach radius before committing. The challenge is reading gravitational currents, using waypoints to reposition particles, and slingshotting them on paths that gravitational entities will bend toward the target.

**Core loop**: Observe the field → read the gravitational currents (visible through ambient field dust) → tap to place a waypoint that captures nearby particles → particles fly to waypoint (force-immune) and lock (enlarged) → grab a locked particle and pull back to aim using trajectory preview → release to sling it on a path toward the target → waypoint expires after 3s, releasing any remaining locked particles → repeat until all particles are scored.

---

## 2. Win/Loss Conditions

- **Win**: `totalScored >= goalTarget`
  - If `required_goals > 0`: goalTarget = that value
  - If `required_goals == 0`: goalTarget = total particle count (every particle must reach a target)
- **Loss**: All particles dead (consumed by black holes / supernovas) before goal is met
- **Score metric**: Time to completion (lower is better)

---

## 3. Entity Types

All positions use a **0–1000 normalized coordinate space** on both axes, scaled to viewport at runtime.

### 3.1 Gravity Well

Attracts particles via Plummer-softened gravitational force.

| Property | Type | Range | Default |
|----------|------|-------|---------|
| x, y | f64 | 0–1000 | required |
| strength | f64 | 1–100 | 50 |

**Force model** (Plummer softening):
```
F(r) = G * M * r / (r^2 + epsilon^2)^(3/2)
```
- `epsilon` = visual_radius * 0.7 (force peaks just inside visual edge, zero at center, 1/r^2 at distance)
- Strength maps linearly: value 1 → 900, value 100 → 7200
- No dead zone or hard clamping needed — Plummer model is smooth everywhere
- **Orbital circularization**: Instead of a tangential nudge hack, use angular momentum floor. Compute `v_circular` at the current radius from the Plummer potential. Blend the tangential velocity component toward `v_circular` at rate 0.03/frame. This replaces both the old tangential nudge and orbital drag with one physically-motivated mechanism.
- **Circularization rate**: 0.03 for gravity wells (allows eccentric fly-bys)

**Visual**: Deep blue core with cyan rim. Interior spiral arcs rotate slowly (1 rev / 6s). Rim pulses gently (alpha 80–120, 2s sine). 2–3 faint dashed concentric rings at softening radius, 2x, and 4x radius show influence zones.

### 3.2 Repulsor

Pushes particles away via Plummer-softened repulsive force. Same strength mapping as gravity wells (900–7200). No orbital circularization.

| Property | Type | Range | Default |
|----------|------|-------|---------|
| x, y | f64 | 0–1000 | required |
| strength | f64 | 1–100 | 50 |

- `epsilon` = visual_radius * 0.5 (tight repulsive core)

**Visual**: Warm amber core with gold radiating spokes (6 lines at 60-degree intervals). Spokes pulse outward (60%–100% length, 1.5s sine). Concentric rings use dashed pattern with wider gaps than wells.

### 3.3 Black Hole

Strong attractor with a kill zone that destroys particles on contact.

| Property | Type | Range | Default |
|----------|------|-------|---------|
| x, y | f64 | 0–1000 | required |
| horizon | f64 | 1–100 | 50 |

- **Fixed gravity**: 10800 (not value-driven — `horizon` controls size only)
- `epsilon` = kill_radius * 1.2 (force peaks at the event horizon rim)
- **Kill radius**: `25px * scaleFactor * (0.5 + (horizon-1)/99 * 1.5)`
- **Visual radius**: killRadius * 1.6
- **Circularization rate**: 0.08 (faster capture spirals than wells)
- Particles entering kill radius are destroyed, decrementing alive count
- **Tidal stretching**: Particles near a black hole are visually elongated along the radial direction proportional to the tidal force gradient. This communicates gravitational intensity without HUD.

**Visual**: Pure black filled circle (kill zone) — the only pure black element on screen. Bright orange-red accretion ring at visual radius (dashed, rotating 2px/frame). 3–4 cosmetic dot motes orbit at visual radius, slowly spiraling inward. Second fainter ring at killRadius * 2.0.

### 3.4 Target

Passive scoring zone. No force. Particles entering the target are consumed and scored.

| Property | Type | Range | Default |
|----------|------|-------|---------|
| x, y | f64 | 0–1000 | required |
| size | f64 | 1–100 | 50 |

- **Hit radius**: `30px * scaleFactor * (0.3 + (size-1)/99 * 2.2)`
- Uses swept circle-vs-circle collision (CCD) to prevent tunneling at high speeds
- Fires trigger (if tagged) on first hit only

**Visual**: Green bullseye — concentric rings pulsing outward (+/- 3px, 2s sine). Score counter `totalScored/goalTarget` in center. Bright green rings, darker green center fill.

### 3.5 Wormhole

Bidirectional teleportation portal pair.

| Property | Type | Range | Default |
|----------|------|-------|---------|
| x1, y1 | f64 | 0–1000 | required |
| x2, y2 | f64 | 0–1000 | required |

- **Capture radius**: 25px * scaleFactor
- **Cooldown**: 500ms between re-entries (prevents oscillation)
- **Velocity handling**: Conserve speed magnitude. If mouths have facing normals, rotate velocity by the angle difference between entry and exit normals. Otherwise, preserve direction.
- **Weak attractor**: Each mouth exerts a weak Plummer attraction (1/4 of weakest gravity well in level) to "suck in" nearby particles, communicating the wormhole's presence.
- **Exit offset**: Place exiting particle at `capture_radius + particle_radius + 1px` along exit velocity direction to prevent immediate re-entry.
- Trail history cleared on teleport.
- Use swept circle-vs-circle for entry detection (prevents high-speed tunneling past mouths).

**Visual**: Both mouths share the same color (violet for first pair, teal for second, etc.). Double-circle outline with counter-rotating dashes creates spinning portal effect. Animated flow dots travel along `VisualConnection` between mouths, alternating direction every 2s. On teleport: both mouths flash bright for 100ms, flow line intensifies for 300ms. During cooldown, entered mouth dims to alpha 40.

### 3.6 Plasma Current

Directional force corridor along a line segment.

| Property | Type | Range | Default |
|----------|------|-------|---------|
| x1, y1 | f64 | 0–1000 | required (start) |
| x2, y2 | f64 | 0–1000 | required (end) |
| width | f64 | 10–300 | required |
| strength | f64 | 1–100 | required |

- Force pushes along the segment direction (from start toward end)
- **Gaussian falloff** (replaces linear): `falloff = exp(-(perpDist / sigma)^2)` where `sigma = halfWidth / 2.0`. Smooth everywhere — no discontinuity at corridor edge.
- Strength maps: value 1 → 0.3, value 100 → 2.0 (scaled)
- **Velocity-dependent coupling**: Particles already moving with the current feel less force; particles moving against feel more: `effective_strength = base * (1.0 - 0.5 * dot(normalize(v), current_dir))`. This creates a natural terminal velocity within the corridor.

**Visual**: Boundary lines in dim cyan. Animated flow dots (2px) move from start to end, speed proportional to strength. Dot density scales with strength (sparse at low, dense at high). Small emitter nozzle at start, thin perpendicular line at end.

### 3.7 Wall

Reflective line segment barrier.

| Property | Type | Range | Default |
|----------|------|-------|---------|
| x1, y1 | f64 | 0–1000 | required (start) |
| x2, y2 | f64 | 0–1000 | required (end) |
| restitution | f64 | 1–200 | 80 |

- Runtime restitution = value / 100 (so 80 → 0.8, 180 → 1.8)
- Values > 100 create **boost walls** that accelerate particles on bounce
- Uses **time-of-impact priority queue** for collision ordering (see Physics section 5.7)
- Velocity reflection: `v -= (1 + restitution) * v_normal * wall_normal` (only when particle is approaching the wall: `v_normal < 0`)
- **Sling immunity does NOT apply** to wall reflections — walls are kinematic constraints, always active

**Visual**: Normal walls: cool gray, 3px thickness, static. Boost walls: gray base + glowing green overlay (2px) with 6px dim green halo. Green intensity scales with restitution value. Pulses on 1s cycle; spikes to max brightness on particle hit for 100ms.

### 3.8 Supernova

Timed explosion that kills nearby particles.

| Property | Type | Range | Default |
|----------|------|-------|---------|
| x, y | f64 | 0–1000 | required |
| size | f64 | 1–100 | 50 |
| countdown | f64 | 1–30 | 5 |

- **Blast radius**: `originalRadius * 3.5`
- On detonation: kills ALL particles within blast radius
- No gravitational pull — purely a timed hazard
- Fires trigger (if tagged) on detonation
- Uses frame-based countdown (not wall-clock) for determinism

**Visual**: Star shape — 6 radiating spokes from center enclosed by faint circle. Hot white-yellow core, orange spokes. **Countdown ring**: thin red circle at blast radius that shrinks toward visual radius over countdown, with increasing alpha (40→200) and thickness (1→3px). Final 2s: core flashes rapidly (4Hz→8Hz). On detonation: white screen flash (alpha 80, 0.3s), screen shake (intensity 8, 0.3s), 30-40 orange-yellow cosmetic particles, expanding ring to blast radius over 150ms then fade.

### 3.9 Particle

The objects the player manipulates. Not a field entity.

| Property | Type | Range | Default |
|----------|------|-------|---------|
| x, y | f64 | 0–1000 | required (spawn pos) |
| vx, vy | f64 | -100–100 | 0 (initial velocity) |
| ax, ay | f64 | -100–100 | 0 (constant acceleration) |

- Each particle gets a random color within level color ranges
- Can be captured by waypoints, killed by hazards, or scored by targets

**Visual**: Small filled circle (4–6px). Ghost trail via `GhostTrail` component (12 snapshots, 0.03s interval, 0.4s fade). Trail color matches particle, dims to alpha 60 at tail. When captured by waypoint: transitions to white over 100ms, faint line drawn to waypoint. When locked at waypoint: **enlarges to 1.5x scale** (grows over 100ms) and pulses gently (alpha 180–255, 1s sine), signaling it is ready to sling. **Tidal stretching near black holes**: sprite elongates along radial direction. **Three-body indicator**: trail shifts to gold/orange when in influence zone of 2+ attractors simultaneously (signals chaotic sensitivity).

---

## 4. Entity Modifiers

### 4.1 Phasing (Activation Cycling)

Any entity can cycle between active and dormant states.

| Pattern | Behavior |
|---------|----------|
| `on:3 off:2` | Active 3s, dormant 2s, repeating |
| `off:5` | Dormant 5s, then active forever |
| `on:4` | Active 4s, then dormant forever |

- Transition fades over 300ms
- **Active**: Full color, full alpha, full animation
- **Dormant**: Dashed outline only (no fill), alpha 30–40, desaturated color, no animation
- **Transitioning**: Alpha/color lerp over 300ms. Brief flash (alpha 255 for 1 frame) marks activation moment.
- Motion freezes when dormant

### 4.2 Motion

Entities can move along predefined paths. **Not allowed on**: wormhole, plasma-current, particle, wall.

**Orbit**: `orbit(cx, cy, period)` — circular motion around center point. Radius = initial distance from center. Period in seconds.

**Patrol**: `patrol(x2, y2, period)` — back-and-forth between start and endpoint. Uses cosine easing for smooth deceleration at endpoints.

**Visual hints**: Orbit paths shown as very faint dashed circle (alpha 15). Patrol paths shown as faint dashed line with chevrons at endpoints pointing in return direction.

**Note**: Moving gravity wells enable gravitational slingshot effects — particles passing behind a moving well's direction of travel gain energy, and those passing in front lose energy. Sub-stepping must update the well's position during the particle's physics step for this to work correctly.

### 4.3 Triggers

Tag-based cause-and-effect chains between entities.

- **Source** (`@tag`): Fires when the entity is activated (target hit, supernova detonation, black hole kill)
- **Listener** (`!tag`): Starts dormant. Activates when matching tag fires.
- **Allowed sources**: target, supernova, black-hole
- **Allowed listeners**: any entity type
- Activation resets phase and motion timers
- **Chains work**: target `@a` → supernova `!a @b` → black-hole `!b`

**Visual for listeners before trigger**: Very faint outline (alpha 15–20), small white "lock" square (3x3px) at entity position. On trigger fire: lock vanishes, brief radial burst of 4–6 white cosmetic particles, flash to full brightness, then settle into active appearance.

---

## 5. Physics System

### 5.1 Force Model — Plummer Softening

All radial forces (gravity wells, repulsors, black holes) use the **Plummer model**:

```
F(r) = G * M * r / (r^2 + epsilon^2)^(3/2)
```

This replaces the original dead zone + distance clamping approach. Advantages:
- Force is zero at r=0 (no dead zone needed)
- Force peaks at `r = epsilon / sqrt(2)`, then falls off as 1/r^2
- Smooth everywhere — no discontinuities
- `epsilon` per entity type controls the "softness" of the force peak

For repulsors, negate the force direction.

### 5.2 Orbital Mechanics — Angular Momentum Floor

Instead of a tangential nudge hack and separate orbital drag, use a unified circularization system:

```
v_radial = dot(v, r_hat) * r_hat
v_tangential = v - v_radial
v_circ = sqrt(G * M * r^2 / (r^2 + epsilon^2)^(3/2))

new_tang_magnitude = lerp(|v_tangential|, v_circ, circularization_rate * dt)
v_tangential = normalize(v_tangential) * new_tang_magnitude
v = v_radial + v_tangential
```

This naturally:
- Circularizes elliptical orbits (physically motivated as tidal dissipation)
- Preserves radial velocity (particles can still escape or spiral in)
- Replaces both tangential nudge AND orbital drag with one mechanism

### 5.3 Integration Method

Use **symplectic Euler** (semi-implicit Euler) — the same form the crusty engine already uses:

```
a = compute_all_forces(position)
velocity += a * dt
velocity = apply_damping(velocity, dt)
position += velocity * dt
```

This is first-order but symplectic (bounded energy error for conservative forces), simple to sub-step, and only requires one force evaluation per step.

### 5.4 Adaptive Sub-stepping

Replace the binary "4 substeps if near attractor, else 1" with **force-magnitude-based** sub-stepping:

```
max_accel = |accumulated_acceleration|
desired_dt = 0.5 * sqrt(2.0 * POSITION_TOLERANCE / max_accel)
substeps = ceil(FIXED_DT / desired_dt).clamp(1, 8)
```

Where `POSITION_TOLERANCE` = 2.0px (scaled). This adapts to any force strength, not just proximity. High-speed particles in plasma currents or near walls also get more substeps. Fallback CFL condition:

```
substeps = max(substeps, ceil(|velocity| * FIXED_DT / min_target_radius).clamp(1, 8))
```

This prevents tunneling through the smallest collision target.

### 5.5 Damping Model — Continuous Velocity-Dependent Drag

Replace the three-tier per-frame multiplier with continuous, frame-rate-independent drag:

```
let speed = |v|;
let effective_drag = BASE_DRAG + SPEED_DRAG * speed;
let factor = (-effective_drag * dt).exp();
v *= factor;

if speed * factor < REST_THRESHOLD {
    v = (0.0, 0.0);
}
```

Constants:
- `BASE_DRAG` = 0.5 (replaces SETTLE_DRAG)
- `SPEED_DRAG` = 0.03 (replaces high-speed DRAG)
- `REST_THRESHOLD` = 0.3 (same snap-to-rest)

Frame-rate independent via `exp(-k * dt)` instead of `multiplier^frames`.

**Waypoint force immunity**: Particles captured by a waypoint bypass the drag model entirely — they move at constant speed toward the waypoint (see section 6.3). On release, particles start from rest and the standard drag model applies normally.

**Sling immunity** simplifies to a single decaying scalar:

```rust
struct SlingState {
    immunity: f64,      // 1.0 = full, 0.0 = none
    decay_rate: f64,    // per-second
    launch_speed: f64,  // initial speed (for scaling)
}

// Per frame:
let speed_ratio = |v| / sling.launch_speed;
let effective_decay = sling.decay_rate * (1.0 + 2.0 * (1.0 - speed_ratio).max(0.0));
sling.immunity = (sling.immunity - effective_decay * dt).max(0.0);
effective_drag = base_drag * (1.0 - sling.immunity);
```

### 5.6 Speed Cap

Hard velocity ceiling: `maxSpeed (8) * scaleFactor`. Velocity is clamped (not damped) to this limit after all force application.

### 5.7 Collision System — Time-of-Impact Priority Queue

Replace iterative wall re-checking with sorted sweep:

```
remaining_dt = dt
for iteration in 0..MAX_BOUNCES (5):
    earliest_hit = None

    for each wall:
        if let Some(hit) = sweep_circle_vs_line(pos, pos + vel * remaining_dt, radius, wall):
            if hit.t < earliest_hit.t: earliest_hit = (hit, wall)

    for each target:
        if let Some(hit) = sweep_circle_vs_circle(pos, vel, remaining_dt, target):
            if hit.t < earliest_hit.t: earliest_hit = (hit, target)

    if no hit:
        position += velocity * remaining_dt
        break

    position = earliest_hit.contact
    remaining_dt *= (1.0 - earliest_hit.t)
    apply_reflection_or_consumption(earliest_hit)
```

This handles arbitrary bounce chains in temporal order and resolves the "wall AND target in same frame" ambiguity naturally.

### 5.8 Edge Bouncing

Particles reflect off screen boundaries with `restitution = 0.8`. Position is mirrored past the boundary.

### 5.9 Force Suppression — Waypoint Capture Override

Particles captured by a waypoint have **all field forces fully suppressed** — this is a binary state, not a blend:

- **Captured**: All gravitational, repulsive, and plasma forces = 0. Drag = 0. Particle moves at constant velocity toward waypoint.
- **Not captured**: All forces apply normally.

There is no smooth transition or suppression radius. Capture is determined at waypoint placement time (see section 6.3), and force immunity persists until the waypoint expires or is replaced.

### 5.10 Force Immunity Scope

**Waypoint capture** fully suppresses these forces:
- Gravity well attraction
- Black hole attraction
- Repulsor repulsion
- Plasma current forces
- Continuous drag (BASE_DRAG + SPEED_DRAG)
- Edge bouncing (captured particles do not bounce off screen edges)

Waypoint capture **does NOT suppress**:
- Wall reflections (kinematic constraints — captured particles still bounce off walls)
- Target consumption (scoring — captured particles can still be scored mid-flight)
- Black hole kill zone (hazard — flying into it is a meaningful outcome)
- Wormhole teleportation (transport — captured particles entering a wormhole mouth are teleported, then continue toward waypoint from exit)

**Sling immunity** attenuates these forces (scaled by immunity value):
- Gravity well attraction
- Black hole attraction
- Repulsor repulsion
- Plasma current forces

Sling immunity **does NOT apply** to:
- Wall reflections (kinematic constraints)
- Edge bouncing (boundary constraints)
- Target consumption (scoring)
- Black hole kill zone (hazard — slung into it = meaningful outcome)

### 5.11 Determinism

- All entity iteration must use `sorted_entities()` (by entity ID)
- Force accumulation order must be deterministic — sort force sources by entity ID
- Supernova countdowns use frame-based timing (not wall-clock `performance.now()`)
- Collision events processed in time-of-impact order (section 5.7)
- Consider `libm` crate for cross-platform deterministic transcendentals

---

## 6. Player Interaction

### 6.1 Waypoint System

One of the player's two tools is placing **waypoints** — positional attractors that capture nearby particles. (The other tool is the slingshot — see section 6.4.)

- **Tap** anywhere on the playfield to place a waypoint at that position
- **Only one waypoint** can exist at a time — placing a new one destroys the previous
- **Timeout**: Waypoints expire after **3 seconds** (frame-based, 180 frames at 60Hz for determinism)
- When a waypoint expires or is replaced, all captured particles are **released** at their current position with zero velocity
- Waypoints can be placed anywhere, including inside entities

### 6.2 Waypoint Reach Preview (Hold & Drag)

Hold and drag (without releasing) to preview the waypoint's capture radius before committing:

- **On touch-down + hold (>150ms without release)**: Enter preview mode
- **Preview circle**: Displays the waypoint capture radius centered on the current finger/cursor position
- **Capture radius**: `WAYPOINT_CAPTURE_RADIUS` (200px * scaleFactor)
- **Particles within radius**: Highlighted (brief alpha pulse, faint connecting line to cursor) to show which particles would be captured
- **Drag to reposition**: The preview follows the finger/cursor in real time
- **Release**: Places the waypoint at the release position (transition from preview to active waypoint)
- **Quick tap** (<150ms): Immediately places waypoint without preview

**Preview visual**: Dashed white circle at capture radius (alpha 40), pulsing slowly (alpha 30–50, 1s sine). Particles within radius get a faint white connecting line (alpha 20) and a subtle brightness boost. Center crosshair marker (8px, alpha 60).

### 6.3 Waypoint Particle Capture

When a waypoint is active, it captures and moves particles within its reach:

- **Capture condition**: Any particle whose center is within `WAYPOINT_CAPTURE_RADIUS` of the waypoint position at the moment the waypoint is placed
- **Capture is instant** — evaluated once when the waypoint is created (not continuously)
- **Captured particle behavior**:
  - All forces are **suppressed** (gravity wells, repulsors, black holes, plasma currents, drag — everything)
  - Particle moves toward the waypoint at **constant speed** (`WAYPOINT_TRAVEL_SPEED` = 3.0 * scaleFactor)
  - Movement is a straight line from current position to waypoint position
  - **No acceleration, no drag, no deflection** — pure constant-velocity linear motion
  - Walls still reflect captured particles (kinematic constraint, same as sling immunity rule)
  - Black hole kill zones still kill captured particles (hazard, always active)
  - Target zones still consume captured particles (scoring, always active)
- **Arrival / lock**: When a captured particle reaches the waypoint position (within 2px), it **locks in place** — velocity set to zero, position snapped to waypoint
  - Locked particles remain force-immune and stationary until the waypoint expires or is replaced
- **Release**: When the waypoint expires (3s timeout) or the player places a new waypoint:
  - All captured/locked particles are released at their current position
  - Velocity is set to zero (particles resume being affected by field forces from rest)
  - Particles naturally begin drifting under whatever gravitational field exists at their release point

**Waypoint visual**:
- **Active waypoint marker**: Small white diamond (6px) with 4 short radiating lines (compass pattern), pulsing gently (alpha 150–220, 1.5s sine)
- **Timeout indicator**: Thin white ring around waypoint that shrinks from capture radius to 0 over the 3s lifetime, communicating remaining time. Alpha fades from 60 to 20 as it shrinks.
- **Capture lines**: Faint white lines (alpha 30) from each captured particle to the waypoint, visible while particles are in transit. Fade out once particles lock.
- **On expire**: Waypoint marker fades out over 100ms, 4–6 tiny white cosmetic particles disperse outward
- **On place**: Brief white flash at waypoint position (alpha 180, 50ms fade), subtle radial pulse ring expanding to capture radius over 200ms (alpha 40→0)

### 6.4 Slingshot (from Locked Particles)

Locked particles (those that have arrived at a waypoint) can be slung directly — no separate charge step.

**Initiate sling**: Touch and hold within 5px of a **locked** particle → particle is selected for slinging. The particle visually enlarges (scale 1.0→1.5x over 100ms) when locked at the waypoint to indicate it is slingable.

**Sling**: Drag the locked particle backward (up to 150px pull). Release launches in the opposite direction.
- The particle is **released from the waypoint** on launch (no longer captured/locked)
- Speed: `2 + (298) * sqrt(normalized_pull)` (ease-out curve from pull distance)
- Dead zone: < 5px pull → snap back, particle stays locked at waypoint
- Post-launch: sling immunity (see 5.5, 5.10)
- Only **locked** particles can be slung — free-flying or in-transit particles cannot

**Sling visuals**:
- **Elastic band**: Two thin bowed lines from particle to anchor (thickness 1→3px scaling with pull distance)
- **Aim preview**: 8–12 dotted trajectory prediction along the elastic, incorporating all active gravitational forces. Each dot is a hollow circle (2px, alpha fading from 180 to 40). Preview extends ~1.5s of simulated time — enough to be helpful, short enough to preserve puzzle challenge.
  - If a predicted dot enters a black hole kill zone: red X, stop drawing further dots
  - If a predicted dot enters a target: green ring
- **Direction arrow**: Small arrow at particle pointing in launch direction (opposite to pull). Length scales with pull distance.
- **Release**: Elastic snaps to zero, particle shrinks back to normal size (1.5x→1.0x over 50ms), flashes white 1 frame, 4 cosmetic particles spray backward (pull direction), micro-shake (intensity 2.0, 0.04s).

---

## 7. Visual Design System

### 7.1 Field Dust — Gravitational Current Visualization

The most important visual system. 200–400 tiny ambient particles ("field dust") drift across the playfield according to gravitational forces at their positions. These are purely cosmetic, rendered on a background layer.

- Each mote: 1–2px, alpha 40–60
- Each frame: compute net gravitational acceleration at mote position, apply to mote velocity with heavy damping (0.85/frame)
- Motes naturally cluster near attractors (emergent density = strength visualization)
- When speed > 1.5 px/frame: draw as 2–3px line in velocity direction (streak)
- Color by dominant force: pale blue for attraction, pale amber for repulsion
- Respawn when consumed by black hole, exiting screen, or after random 3–8s lifetime
- Update in batches (50/frame across 4–8 frames) for performance

This transforms the game from "launching into invisible forces" to "reading visible currents."

### 7.2 Render Layer Architecture

| Layer | Name | Contents |
|-------|------|----------|
| 0 | Background | Dark blue-black (#0a0a14) + mild vignette (0.3) |
| 10 | Field Visualization | Field dust motes, concentric influence rings |
| 20 | Connections | Wormhole flow lines, plasma corridor boundaries, motion path hints |
| 30 | Entities | All game entities |
| 40 | Particles | Gameplay particles + ghost trails |
| 50 | Effects | Cosmetic bursts, sparks, flashes |
| 60 | Interaction UI | Waypoint marker, timeout ring, capture radius preview, capture lines, sling elastic, aim preview |
| 70 | HUD | Score, timer |

### 7.3 Color Palette — Semantic Encoding

| Color Family | Meaning | Hex Range |
|---|---|---|
| Blue/Cyan | Attraction | #1a3a6a → #4af0ff |
| Amber/Gold | Repulsion | #6a4a1a → #ffd040 |
| Red/Orange | Danger | #ff2200 → #ff8800 |
| Green | Success | #004020 → #00ff80 |
| Violet/Teal | Transport | #aa44ff, #44ffaa |
| White | Player action | #ffffff |
| Gray | Neutral structure | #444455 → #888899 |

Players internalize: "blue pulls, gold pushes, red kills, green wins, purple teleports."

### 7.4 Feedback Events

**Particle scores**:
- 30ms time-pause (2 frames)
- Target flashes green→white→green over 100ms
- 12–16 cosmetic particles burst outward in particle's color (ring pattern, speed 80–120, life 0.4s)
- Subtle green screen flash (alpha 30, 0.15s)
- On level-winning score: 50ms freeze, stronger flash, 24–32 particles

**Particle consumed by black hole**:
- 150ms death spiral animation: particle shrinks 100%→1px, trail tightens, color shifts to orange-red
- 8 dark red/orange particles burst INWARD (contracting burst — visually distinct from scoring)
- Screen shake intensity 3.0, 0.1s
- Dark red screen tint, intensity 0.15, 0.2s

**Wall bounce**:
- 4–6 tiny sparks at contact point along reflection normal
- Boost walls: 8–12 bright green sparks, wall flashes full brightness 50ms, micro-shake 1.5 / 0.05s

**Waypoint placed**:
- White flash at waypoint position (alpha 180, 50ms)
- Radial pulse ring expanding to capture radius over 200ms (alpha 40→0)
- Captured particles briefly flash white (50ms), then show faint connecting line to waypoint

**Waypoint expired**:
- Waypoint marker fades out over 100ms
- 4–6 tiny white cosmetic particles disperse outward
- Captured particles resume normal color over 100ms

**Particle locks at waypoint**:
- Subtle white pulse at lock position (alpha 100, 80ms)
- Connecting line fades out over 100ms

### 7.5 Gravitational Lensing (Post-Processing)

Subtle screen-space displacement near strong attractors:

```
For each pixel near an attractor:
  displacement = lensing_strength * r_hat / (r^2 + epsilon^2)
  sampled_color = framebuffer[pixel + displacement]
```

Max displacement: 2–5 pixels. Applied as a post-fx pass. Subtle warping near gravity wells, stronger near black holes. Communicates field presence through environmental distortion.

### 7.6 Visual Density Management

- Gameplay-critical elements (particles, targets) always fully opaque
- Cosmetic elements (field dust, path hints, connections) at alpha 15–50 — naturally recede when overlapping important elements
- 1px dark outline around each entity prevents silhouette merging
- Stagger pulsing animations with random phase offsets per entity (prevents visual strobing when entities cluster)
- Adaptive field dust: reduce spawning in regions with 3+ overlapping entity influence zones

---

## 8. Mapping to Crusty ECS

### Components Needed

| Gravity Pong Concept | Crusty Component | Notes |
|---|---|---|
| Entity position | `Transform` | x, y fields |
| Particle physics | `RigidBody` | vx, vy, ax, ay, damping, restitution |
| Gravity well/repulsor/black hole | `ForceField` | Use Attract/Repel types, InverseSquare falloff |
| Collision zones | `Collider` | Circle shape for wells/targets/holes, line for walls |
| Kill zones | Custom component | `BlackHoleKillZone { kill_radius, epsilon }` |
| Targets | Custom component | `ScoringZone { radius, hits }` |
| Wormholes | Custom component | `WormholeLink { partner: Entity, cooldown_remaining }` |
| Plasma currents | Custom component | `PlasmaStream { dx, dy, half_width, sigma, strength }` |
| Walls | Custom component | `WallSegment { normal, length, restitution }` |
| Supernovas | Custom component | `TimedExplosion { countdown, elapsed, blast_radius, exploded }` |
| Particles | `Tags` with "particle" tag + `RigidBody` |
| Phasing | Custom component | `PhaseCycle { on_duration, off_duration, elapsed, alpha }` |
| Motion paths | Custom component or use `WaypointPath` / `Behavior::Orbit` |
| Triggers | `SignalEmitter` + `SignalReceiver` | Map @tag → channel |
| Waypoint state | Custom component | `Waypoint { position, remaining_frames, captured_entities }` |
| Waypoint capture | Custom component | `WaypointCaptured { waypoint: Entity, locked: bool }` |
| Sling state | Custom component | `SlingState { immunity, decay_rate, launch_speed }` |
| Game score | `GameState` (global) | `totalScored`, `goalTarget`, `aliveCount` |
| Field dust | `Tags` with "field_dust" tag + minimal `RigidBody` | Cosmetic-only layer |
| Tidal stretch | Custom component | `TidalStretch { stretch_x, stretch_y }` | Visual only |

### Systems Needed

| System | Phase | Description |
|---|---|---|
| `supernova_system` | Simulation | Frame-based countdown + detonation |
| `phase_system` | Simulation | Entity active/dormant cycling with alpha fade |
| `entity_motion_system` | Simulation | Orbit and patrol path updates |
| `trigger_system` | Simulation | Fire and receive tag-based triggers |
| `gravity_force_system` | Physics | Plummer-softened attraction/repulsion |
| `circularization_system` | Physics | Angular momentum floor near attractors |
| `plasma_force_system` | Physics | Gaussian-profile directional corridor forces |
| `collision_system` | Physics | TOI priority queue: walls + targets + edges |
| `wormhole_system` | PostPhysics | Teleportation with cooldown + velocity rotation |
| `waypoint_input_system` | Input | Tap/hold detection, waypoint placement, preview mode |
| `waypoint_capture_system` | Physics | Move captured particles toward waypoint at constant speed, handle locking |
| `waypoint_timeout_system` | Simulation | Frame-based countdown, expire and release particles |
| `sling_system` | Input | Locked particle sling detection, launch, immunity decay |
| `win_loss_system` | Simulation | Check score vs goal, check alive count |
| `field_dust_system` | Rendering | Update cosmetic field dust motes |
| `tidal_stretch_system` | Rendering | Compute visual elongation near attractors |
| `aim_preview_system` | Rendering | Trajectory prediction during sling pull-back |

### Simulation Trait Implementation

```rust
pub struct GravityPong {
    // Level definition
    // Score tracking
    // Player interaction state
}

impl Simulation for GravityPong {
    fn setup(&mut self, engine: &mut Engine) {
        // Parse level definition
        // Spawn all entities with appropriate components
        // Spawn field dust motes
        // Initialize score tracking in GameState
    }

    fn step(&mut self, engine: &mut Engine) {
        // 1. Process player input (tap → place waypoint, hold → preview, sling locked particles)
        // 2. Run waypoint timeout (decrement frames, expire, release particles)
        // 3. Run entity phase cycling
        // 4. Run entity motion (orbit, patrol)
        // 5. Run supernova countdowns
        // 6. Check triggers
        // 7. Check win/loss conditions
        // Physics (gravity, collision, waypoint capture, sling immunity) handled by engine systems
    }

    fn render(&self, engine: &mut Engine) {
        // Layer 0: Background
        // Layer 10: Field dust + influence rings
        // Layer 20: Wormhole connections, plasma boundaries, motion paths
        // Layer 30: All game entities
        // Layer 40: Particles + ghost trails
        // Layer 50: Effects (bursts, sparks)
        // Layer 60: Waypoint UI + sling elastic + aim preview
        // Layer 70: HUD (timer, score counter)
        // Post-fx: Gravitational lensing, vignette
    }
}
```

---

## 9. Physics Constants Reference

| Constant | Value | Category |
|----------|-------|----------|
| FIXED_DT | 1/60s | Engine timing |
| WELL_MIN_STRENGTH | 900 | Gravity wells |
| WELL_MAX_STRENGTH | 7200 | Gravity wells |
| WELL_EPSILON_FACTOR | 0.7 | Plummer softening (× visual radius) |
| WELL_CIRC_RATE | 0.03 | Orbit circularization blend rate |
| BLACK_HOLE_GRAVITY | 10800 | Black holes |
| BLACK_HOLE_EPSILON_FACTOR | 1.2 | Plummer softening (× kill radius) |
| BLACK_HOLE_CIRC_RATE | 0.08 | Capture spiral rate |
| REPULSOR_MIN_STRENGTH | 900 | Repulsors |
| REPULSOR_MAX_STRENGTH | 7200 | Repulsors |
| REPULSOR_EPSILON_FACTOR | 0.5 | Plummer softening (× visual radius) |
| BASE_DRAG | 0.5 | Continuous drag (linear term) |
| SPEED_DRAG | 0.03 | Continuous drag (quadratic term) |
| REST_THRESHOLD | 0.3 | Snap-to-rest speed |
| EDGE_RESTITUTION | 0.8 | Screen bounce |
| MAX_SPEED | 8.0 (scaled) | Hard velocity cap |
| POSITION_TOLERANCE | 2.0px | Adaptive sub-step threshold |
| MAX_SUBSTEPS | 8 | Sub-step cap |
| MAX_BOUNCES | 5 | Collision iteration cap |
| PLASMA_MIN_FORCE | 0.3 | Plasma currents |
| PLASMA_MAX_FORCE | 2.0 | Plasma currents |
| SUPERNOVA_BLAST_SCALE | 3.5 | Explosion radius multiplier |
| WORMHOLE_COOLDOWN | 500ms | Teleport cooldown |
| WAYPOINT_CAPTURE_RADIUS | 200px (scaled) | Waypoint particle capture radius |
| WAYPOINT_TRAVEL_SPEED | 3.0 (scaled) | Constant speed for captured particles |
| WAYPOINT_TIMEOUT_FRAMES | 180 | Waypoint lifetime (3s at 60Hz) |
| WAYPOINT_PREVIEW_HOLD_MS | 150ms | Hold duration to enter preview mode |
| WAYPOINT_LOCK_SCALE | 1.5 | Particle enlargement when locked |
| SLING_MIN_SPEED | 2.0 | Launch floor |
| SLING_MAX_SPEED | 300.0 | Launch ceiling |
| SLING_MAX_PULL | 150px | Pull distance cap |
| SLING_DECAY_RATE | 0.4 | Immunity decay per second |
| PHASE_FADE_MS | 300ms | Phasing transition duration |
| WALL_DEFAULT_RESTITUTION | 0.8 | Wall bounce default |
| FIELD_DUST_COUNT | 300 | Cosmetic field dust motes |
| FIELD_DUST_DAMPING | 0.85 | Mote velocity damping per frame |
| LENSING_MAX_DISPLACEMENT | 4px | Post-fx gravitational lensing |

---

## 10. Implementation Order

### Phase 1: Core Physics
1. Particle spawning with Transform + RigidBody
2. Gravity well force system (Plummer softening)
3. Edge bouncing
4. Speed cap + continuous drag model
5. Basic target collision (CCD sweep, consume particle, increment score)
6. Win/loss detection

### Phase 2: Advanced Forces
7. Repulsor force system
8. Black hole force + kill zone
9. Angular momentum floor / circularization system
10. Adaptive force-magnitude-based sub-stepping

### Phase 3: Obstacles & Transport
11. Wall collision via TOI priority queue
12. Wormhole teleportation with cooldown + velocity handling
13. Plasma current with Gaussian profile + velocity coupling
14. Supernova frame-based countdown + detonation

### Phase 4: Player Interaction
15. Waypoint placement system (tap to place, one at a time, 3s timeout)
16. Waypoint capture system (capture particles within radius, constant-speed travel, locking)
17. Waypoint reach preview (hold & drag to see capture radius before placing)
18. Slingshot system (grab locked particle, pull-back, launch, decaying immunity scalar)
19. Waypoint force immunity (suppress all forces on captured/locked particles)

### Phase 5: Level Dynamics
20. Entity phasing (on/off cycling with alpha fade)
21. Entity motion (orbit + patrol, with gravitational slingshot support)
22. Trigger system (@tag → !tag chains)
23. Level loading from definition format

### Phase 6: Visual Polish
24. Field dust system (gravitational current visualization)
25. Entity visual language (shapes, colors, animations per type)
26. Waypoint visuals (marker, timeout ring, capture lines, preview circle)
27. Locked particle visuals (enlarged scale, pulse, sling-ready indicator)
28. Sling aim preview (physics-aware trajectory dots)
29. Feedback events (score bursts, death spirals, bounce sparks, waypoint place/expire)
30. Phasing/trigger visual states (dormant ghosting, lock indicators)
31. Wormhole connection lines + plasma flow dots
32. Tidal stretching near black holes
33. Gravitational lensing post-fx
34. Render layer ordering + visual density management

---

## 11. Example Level Definitions

### Simple (1 target, 1 well, wall barrier, 7 particles)
```
target:500:220:70
gravity-well:500:520:50
wall:150:650:850:650:80
particle:200:850
particle:400:900
particle:500:950
particle:600:900
particle:800:850
```

### Chain Reaction (trigger: target hit activates wells)
```
target:500:200:60 @chain
gravity-well:300:550:50 !chain
gravity-well:700:550:50 !chain
particle:420:870
particle:460:900
particle:500:880
particle:540:900
particle:580:870
```

### Complex Multi-Wave (walls, plasma, orbiting wells, triggers, supernovas)
```
target:500:50:40 @wave1
wall:200:130:800:130:80
wall:200:130:200:870:80
wall:800:130:800:870:80
wall:200:870:800:870:80
plasma-current:500:870:500:720:60:70
plasma-current:200:550:350:400:50:60
gravity-well:300:450:60:orbit:500:450:12
gravity-well:700:450:60:orbit:500:450:12
repulsor:500:550:40:on3:off2
wormhole:665:50:780:150
supernova:500:550:60:8 @wave2 !wave1
black-hole:350:200:30 !wave2
repulsor:250:300:50:patrol:250:700:10 !wave2
gravity-well:500:150:70:on2:off2 !wave2
particle:500:800
particle:400:830
particle:600:830
```
