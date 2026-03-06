# Unit Newtypes & Advanced Compiler Patterns

Deep research into `WorldCoord`, `ScreenPx`, `Radians`, `Seconds` newtypes plus advanced Rust patterns.

---

## Unit Newtypes — Full Landscape

### Semantic categories of f64 usage (~655 locations)

| Category | ~Count | Files | Newtype candidate |
|----------|--------|-------|-------------------|
| **World coordinates** (game-space position) | 120 | transform.rs, gravity_pong, aim_preview, components | `WorldCoord(f64)` |
| **Screen/pixel coordinates** | 100 | framebuffer.rs, shapes.rs, layers.rs, ui_canvas | `ScreenPx(f64)` |
| **Time durations** (dt, timers, decay) | 180 | engine.rs, timers.rs, lifetime.rs, tween, particles | `Seconds(f64)` |
| **Radians** (rotation, angles) | 25 | transform.rs (RESERVED), gravity_pong trajectories | `Radians(f64)` — defer |
| **Velocities** (vx/vy) | 40 | rigidbody.rs, gravity_pong physics | Keep as f64 |
| **Dimensionless** (scale, opacity, ratios) | 100+ | Everywhere | Not worth wrapping |
| **Audio** (Hz, volume 0-1) | 30 | sound.rs | Too niche |
| **Game-specific** (score, health, strength) | 60 | game_state, gravity_pong components | Not spatial |

### Conversion boundaries (where types change)

1. **World → Screen**: `rendering/layers.rs` camera transform, `shapes.rs` draw calls. Natural `Into<ScreenPx>` boundary.
2. **Seconds creation**: `engine.rs` FIXED_DT, timer callbacks. Parse from f64 at system boundaries.
3. **Radians creation**: Currently RESERVED in Transform. Only active in gravity_pong angle calculations.

### Operator requirements per type

| Type | Add | Sub | Mul<f64> | Div<f64> | Neg | Cmp | Mul<Self> |
|------|-----|-----|----------|----------|-----|-----|-----------|
| WorldCoord | yes | yes | yes (scale) | yes | yes | yes | no |
| ScreenPx | yes | yes | yes | yes | yes | yes | no |
| Seconds | yes | yes | yes (scale) | yes | no | yes | no |
| Radians | yes | yes | yes | no | yes | no | no |

A single `unit_newtype!` macro can generate all trait impls.

### Recommended macro

```rust
macro_rules! unit_newtype {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        #[repr(transparent)]
        pub struct $name(pub f64);

        impl std::ops::Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self { $name(self.0 + rhs.0) }
        }
        impl std::ops::Sub for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self { $name(self.0 - rhs.0) }
        }
        impl std::ops::Mul<f64> for $name {
            type Output = Self;
            fn mul(self, rhs: f64) -> Self { $name(self.0 * rhs) }
        }
        impl std::ops::Div<f64> for $name {
            type Output = Self;
            fn div(self, rhs: f64) -> Self { $name(self.0 / rhs) }
        }
        impl std::ops::Neg for $name {
            type Output = Self;
            fn neg(self) -> Self { $name(-self.0) }
        }
        impl $name {
            pub const ZERO: Self = $name(0.0);
            pub fn raw(self) -> f64 { self.0 }
        }
    };
}
```

### Blast radius estimate

**Phase 1: Seconds (highest safety gain, moderate blast)**
- ~180 locations, ~15 files
- Primary: engine.rs (FIXED_DT, accumulator, dt params), timers.rs, lifetime.rs, property_tween.rs, particles.rs
- Key risk: Seconds * Seconds makes no physical sense — catches bugs like `dt * dt` without a unit

**Phase 2: WorldCoord / ScreenPx (biggest refactor)**
- ~220 locations, ~25 files
- Transform x/y → WorldCoord, framebuffer pixel ops → ScreenPx
- Rendering boundary is the natural conversion point
- Key risk: Adding WorldCoord to ScreenPx is a compile error — prevents coordinate space confusion

**Phase 3: Radians (defer)**
- ~25 locations, mostly gravity_pong
- Transform.rotation is currently "RESERVED" — activate only when rotation goes live

### Friction points

1. **Physics inner loops**: Intermediate calculations where everything is world-coords. Wrapping/unwrapping adds noise. Mitigation: work on `.0` directly inside physics functions, wrap at API boundaries.
2. **Velocity type**: Should `vx`/`vy` be `WorldCoord` or a separate `Velocity` type? Recommendation: keep as f64 inside RigidBody. Velocity = WorldCoord/Seconds is complex; not worth the machinery.
3. **Sound frequency**: Hz is not worth a newtype — too niche, only used in sound.rs.
4. **Dimensionless multipliers**: Scale (0-1), opacity (0-1), damping factors. These are ratios — wrapping them would be pure noise.

---

## Advanced Compiler Patterns

### Tier 1: Do now (high ROI, low effort)

**1. Seal Simulation, Policy, SchemaInfo traits**
```rust
mod sealed { pub trait Sealed {} }
pub trait Simulation: sealed::Sealed { ... }
// impl sealed::Sealed for GravityPong {}
// impl sealed::Sealed for DemoBall {}
// impl sealed::Sealed for ChordRepsSim {}
```
- Effort: 30 min
- Prevents external misimplementation of the contract
- Only relevant if engine-core is published; low urgency now

**2. Tighten pub visibility in lib.rs**
- Only 2 uses of `pub(crate)` in entire codebase
- `ecs`, `components`, `systems`, `rendering`, `physics`, `events`, `input` are all `pub mod` but should be internal
- Re-export narrowly: `pub use engine::Engine;` etc.
- Effort: 2-3 hours (audit)
- Protects future refactoring of internals

**3. `const fn` on Color and SeededRng**
```rust
// color.rs — enable compile-time color constants
pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color { ... }

// rng.rs — enable const seed initialization
pub const fn new(seed: u64) -> SeededRng { ... }
```
- Effort: 30 min
- Enables `const PLAYER_COLOR: Color = Color::from_rgba(...)` patterns

### Tier 2: Nice to have

**4. Reuse Vec allocations in hot paths**
- `systems/renderer.rs`: `Vec<(i32, f64, f64, DrawType)>` allocated every frame
- `systems/collision.rs`: 4 Vecs allocated per physics step
- Move to Engine fields, `clear()` and reuse
- Effort: 4-5 hours
- Impact: removes ~100 small allocs/frame (small entities, but free win)

**5. `#[non_exhaustive]` on extensible enums**
- Candidates: `SystemPhase`, `Waveform`, `ScreenEffect`, `TransitionKind`, `ColliderShape`
- Only relevant when published as a library
- Defer until then

### Not recommended

| Pattern | Why skip |
|---------|----------|
| Typestate on Engine/Framebuffer | Lifecycle enforced by calling patterns, not types. Adds ceremony without preventing real bugs. |
| Typestate on EventBuilder | Current builder is simple; typestate adds struct proliferation. |
| Replace RefCell thread-locals | Required for WASM FFI. Already optimal. |
| Const-generic field grid | Loses runtime flexibility for negligible gain. |
| Monomorphize Box<dyn> | Only 1 Box<dyn> usage in entire codebase (lib.rs SIM). No vtable bloat. |

### Already done well
- `ComponentStore<T>` avoids unnecessary `T: Default` bound
- `CompareOp`, `Waveform` use exhaustive enums (no `_` catch-all)
- No deep clones in hot paths
- String→enum migration (Tags, EventChannel, KeyCode, SoundEvent) now in progress

---

## Suggested rollout

1. **Seconds newtype** — highest safety-to-effort ratio, catches time unit bugs
2. **WorldCoord + ScreenPx** — biggest blast radius, save for a focused sprint
3. **Radians** — defer until rotation is activated
4. **pub(crate) tightening** — do alongside any major refactor
5. **Sealed traits** — do when publishing engine-core as a library
