# Leaning on the Compiler More

Two-perspective audit: game/UI logic expert + Rust compiler expert. Every recommendation below is **zero or near-zero runtime cost** unless noted.

---

## Tier 1 — High Impact, Low Effort

### 1. Kill stringly-typed events & sounds
**Now:** `BusEvent::new("collision")`, `sound_palette.play("impact", &mut queue)`
**Problem:** Typos compile fine, fail silently at runtime.
**Fix:** Exhaustive enums.

```rust
pub enum EventChannel { Collision, TriggerEnter, TimerFired, ScoreChanged }
pub enum SoundEvent  { Impact { intensity: f64 }, Pickup, Ambience { freq: f64 } }
```

Compiler catches: misspelled channels, unhandled new event types, wrong payload shapes.
**Cost:** Zero. Enums dispatch as efficiently as string matches (faster, actually).

### 2. Kill stringly-typed tags & roles
**Now:** `Tags { values: Vec<String> }`, `Role { name: String, intent: String }`
**Problem:** `"eemy"` compiles, never matches `"enemy"`.
**Fix:**

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tag { Enemy, Boss, Bullet, Trigger, Player, Wall }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RoleName { Player, NPC, Projectile, Obstacle }
```

Behavior rules (`Condition::Collision { tag_a: Tag, tag_b: Tag }`) get exhaustive matching for free.
**Cost:** Zero.

### 3. Newtype key codes (input boundary)
**Now:** `keys_held: HashSet<String>` — typo `"Sapce"` silently ignored.
**Fix:**

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyCode { Space, KeyA, KeyW, KeyS, KeyD, ArrowUp, ArrowDown, ... }
```

Parse `String → KeyCode` once at the WASM boundary. Everything downstream is typed.
**Cost:** Zero (faster than hashing strings).

### 4. `#[must_use]` on side-effect returns
**Now:** `drain_json()`, `play()`, `emit()` return values sometimes ignored.
**Fix:** Sprinkle `#[must_use]` on methods where dropping the return value is likely a bug.
**Cost:** Zero. 15 minutes of work.

---

## Tier 2 — High Impact, Medium Effort

### 5. Unit newtypes for coordinates & time
**Now:** Everything is bare `f64`. World coords, screen pixels, radians, seconds — all interchangeable.
**Problem:** `PARTICLE_RADIUS` (5.0, visual) vs `PARTICLE_WORLD_RADIUS` (8.0, physics) — nothing prevents mixing them.
**Fix:**

```rust
#[derive(Clone, Copy, Debug)] #[repr(transparent)]
pub struct WorldCoord(pub f64);

#[derive(Clone, Copy, Debug)] #[repr(transparent)]
pub struct ScreenPx(pub f64);

#[derive(Clone, Copy, Debug)] #[repr(transparent)]
pub struct Radians(pub f64);

#[derive(Clone, Copy, Debug)] #[repr(transparent)]
pub struct Seconds(pub f64);
```

Implement `Add`, `Sub`, `Mul<f64>` etc. via a macro. `WorldCoord + ScreenPx` is a compile error.
**Cost:** Zero runtime (`#[repr(transparent)]`). Medium refactor effort — touches Transform, physics, rendering.
**Tradeoff:** Operator impls add boilerplate. A `unit_newtype!` macro keeps it manageable.

### 6. Typed game state keys
**Now:** `global_state.set_f64("score", 100.0)` — string key, untyped value.
**Option A (simple):** Replace HashMap with a plain struct per game:

```rust
pub struct GravityPongState { pub score: f64, pub lives: i32, pub level: u8 }
```

**Option B (generic):** Typed key trait:

```rust
pub trait StateKey { type Value: Clone; fn name() -> &'static str; }
struct Score; impl StateKey for Score { type Value = f64; ... }
```

**Cost:** Option A is zero-cost and dead simple. Option B adds trait machinery.
**Recommendation:** Option A for game-specific state; keep the dynamic HashMap only for editor/tooling.

### 7. State machine — enum over strings
**Now:** `StateMachine::new("idle")`, transitions checked by string comparison.
**Fix:** Per-game state enum with exhaustive transitions:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PongPhase { Attract, Countdown, Playing, Paused, GameOver }
```

Full typestate (phantom types) is possible but high-friction for a game that hot-swaps states. A simple enum with `match` (no `_` arm) gets 90% of the benefit.
**Cost:** Zero.

---

## Tier 3 — Nice to Have

### 8. Seal the `Simulation` trait
Prevents misimplementation if the engine is ever a library. Private supertrait:

```rust
mod sealed { pub trait Sealed {} }
pub trait Simulation: sealed::Sealed { ... }
```

**Cost:** Zero. Low urgency since all implementations are in-crate today.

### 9. Newtype entity roles
**Now:** `Entity(pub u64)` — a paddle ID and a ball ID are the same type.
**Fix:** `struct PaddleId(Entity)`, `struct BallId(Entity)` — prevents passing a ball where a paddle is expected.
**Cost:** Zero. Moderate refactor.

### 10. Const fn for magic constants
**Now:** Gravity well strength is computed at runtime from constants.
**Fix:** `const fn compute_gm(strength_pct: f64) -> f64 { ... }` — moves computation to compile time where possible.
**Cost:** Zero. Micro-optimization; low priority.

### 11. Rendering layer IDs as a newtype/enum

```rust
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LayerId(i32);
impl LayerId {
    pub const BACKGROUND: Self = Self(-10);
    pub const ENTITIES: Self = Self(0);
    pub const UI: Self = Self(100);
}
```

**Cost:** Zero. Prevents confusion between layer IDs and other integers.

---

## What NOT to do (compiler overkill)

| Idea | Why skip it |
|------|-------------|
| Phantom-typed framebuffer (Active/Inactive) | Adds ownership gymnastics; rendering bugs are visual, not crashes |
| Typestate builder for BusEvent | Current builder is simple and correct; typestate adds struct proliferation |
| Replace RefCell thread-locals | Necessary for WASM FFI; already the right pattern |
| Const-generic field grid size | Loses runtime flexibility for negligible gain |
| Encapsulate Transform fields behind getters | Internal engine code; public fields are fine |

---

## Already done well

- `ComponentStore<T>` has no unnecessary `T: Default` bound
- `CompareOp`, `Waveform` use exhaustive enums with no `_` catch-all
- Entity is a Copy newtype (not raw u64 everywhere)
- No deep clones in hot paths
- Destructure-at-top pattern avoids borrow conflicts

---

## Suggested migration order

1. **Tags/Roles → enums** (most string-matching bugs live here)
2. **EventChannel → enum** (central communication hub)
3. **KeyCode → enum** (input boundary, one parse point)
4. **SoundEvent → enum** (small surface area)
5. **Game state → per-game struct** (removes dynamic typing per game)
6. **Unit newtypes** (biggest refactor, save for a focused sprint)
7. **`#[must_use]`** (sprinkle anytime, no migration needed)
