# S-League Game Module

Minigolf RPG demo. Pokemon-style overworld with random encounter minigolf fights. 480x720 portrait, touch + mouse.

## Overview

Two-mode game in a single Rust module (`sleague.rs`, ~1100 lines):
- **Overworld** (mode 0.0): Top-down tilemap walking, wild grass encounters
- **Fight** (mode 1.0): Slingshot minigolf against monsters

All state lives in `engine.global_state` (key-value store). No ECS entities used.

## Monsters

| ID | Name | Par | Max Strokes | Course Features |
|----|------|-----|-------------|-----------------|
| 0 | Mole | 3 | 6 | L-shaped walls + sand trap |
| 1 | Rabbit | 2 | 5 | Two small wall blocks |
| 2 | Scorpion | 4 | 7 | Corridor walls with gaps + large sand |
| 3 | Slime | 3 | 6 | Single vertical wall |

## Physics

- `BALL_RADIUS = 5.0` px, `HOLE_RADIUS = 8.0` px
- `MAX_POWER = 400.0` px/s, `DRAG = 1.8` (3x on sand)
- `RESTITUTION = 0.65`, `STOP_THRESHOLD = 8.0` px/s
- 4 physics substeps per frame
- 5-point collision check (center + 4 cardinal offsets)
- Wall bounce screen shake proportional to impact speed

## Scoring

- XP multiplier: eagle (3x), birdie (2x), par (1x), bogey (0.5x), worse (0.25x)
- Level up at `XP >= level^2 * 50`
- Even levels increase max HP by 1
- Failed fights cost 3 HP, mercy heal at 0 HP

## State Keys

### Common
`game_mode`, `player_hp`, `player_max_hp`, `player_level`, `player_xp`, `encounters_won`, `steps`

### Fight
`ball_x/y`, `ball_vx/vy`, `strokes`, `tl_phase` (0=aim, 1=moving, 2=win, 3=fail), `hole_x/y`, `start_x/y`, `aim_x/y`, `aim_active`, `monster_id`, `result_timer`, `dist_to_hole`, `best_dist`, `wall_bounces`

## Public API

```rust
setup(engine)              // Full game, overworld start
setup_fight_only(engine)   // Fight only (headless/CLI)
update(engine, dt)
render(engine)
on_pointer_down/move/up(engine, x, y)
dispatch_action(engine, action)  // Headless ScheduledAction routing

// AI scoring (operate on SimResult)
score_hole_completion(sim) -> f64     // 1.0 if sunk
score_stroke_efficiency(sim) -> f64   // par/strokes
score_proximity_to_hole(sim) -> f64   // 1 - dist/500
```
