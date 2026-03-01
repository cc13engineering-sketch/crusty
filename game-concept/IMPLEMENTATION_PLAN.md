# Trap Links — Engine Gap Implementation Plan

Based on pre-audit of all affected engine source files.

---

## Gap Priority (Round 5 vs Round 6)

### Round 5 — Physics & Core (4 gaps)
| Gap | File | Change | LOC |
|-----|------|--------|-----|
| **Gap 7: Restitution** | collision.rs | Wire `PhysicsMaterial.restitution_override` into CCD response | ~15 |
| **Gap 4: Scene Isolation** | scene_manager.rs, engine.rs | Add World snapshot save/restore on push/pop | ~120 |
| **Gap 1: Multi-layer TileMap** | tilemap.rs | Add `layers: Vec<TileLayer>` with per-layer render | ~100 |
| **Gap 6: Aim Preview** | New: aim_preview.rs | Shot trajectory preview (simulate physics, draw dots) | ~80 |

### Round 6 — UI & Content (4 gaps)
| Gap | File | Change | LOC |
|-----|------|--------|-----|
| **Gap 5: UI Tap Actions** | ui_canvas.rs, engine.rs | EventBus integration for button taps | ~40 |
| **Gap 3: Dialogue Branching** | dialogue.rs | Add Choice variant with options + callback | ~90 |
| **Gap 2: Sprite Tiles** | tilemap.rs, rendering | Sprite sheet rendering per tile | ~60 |
| **Gap 8: One-shot SFX** | sound.rs, auto_juice.rs | AutoJuice → SoundQueue wiring for fight sounds | ~30 |

---

## Detailed Specifications

### Gap 7: Restitution Override (TRIVIAL)
**Current**: `collision.rs:169` uses `snap.restitution.min(other.restitution)` from RigidBody only.
**Fix**: Check `PhysicsMaterial.restitution_override` first:
```rust
let e_a = world.physics_materials.get(&entity_a)
    .and_then(|pm| pm.restitution_override)
    .unwrap_or(snap_a.restitution);
let e_b = world.physics_materials.get(&entity_b)
    .and_then(|pm| pm.restitution_override)
    .unwrap_or(snap_b.restitution);
let e = e_a.min(e_b);
```

### Gap 4: Scene Isolation
**Current**: SceneManager stores only name→source mappings, no state.
**Add**: `WorldSnapshot` that saves/restores all 26 ComponentStores.
- On `push()`: snapshot current World into stack entry
- On `pop()`: restore World from stack entry
- Engine gets `snapshot_world()` and `restore_world()` methods

### Gap 1: Multi-layer TileMap
**Current**: Single flat `Vec<Tile>` grid.
**Add**: `TileLayer` wrapper, `MultiTileMap` with ordered layers.
- Background layer: terrain (grass, stone, sand)
- Object layer: traps, items, NPCs
- render() iterates layers bottom-to-top

### Gap 6: Aim Preview
**New module**: Simulate ball physics forward N steps, record positions, draw ghost dots.
- Input: ball position, velocity direction, power
- Output: Vec of (x, y) positions at 0.1s intervals
- Uses same CCD sweep functions for accurate prediction
- Render as fading white dots (3-5 dots)

### Gap 5: UI Tap Actions → EventBus
**Current**: `hit_test()` returns `Option<String>` action.
**Add**: In engine tick, if mouse_up in UI area, run hit_test, emit EventBus event.

### Gap 3: Dialogue Branching
**Add**: `MessageKind::Choice` with `Vec<(String, String)>` (label, action).
- Renders as buttons below dialogue text
- Selection emits EventBus event with chosen action

### Gap 2: Sprite Tiles
**Current**: `sprite_index: Option<u16>` exists on Tile but unused in render.
**Add**: Accept optional sprite sheet data (pixel data + tile dimensions).
- If tile has sprite_index and sheet provided, blit sprite instead of fill_rect.

### Gap 8: One-shot SFX via AutoJuice
**Current**: AutoJuice has `JuiceEffect::Sound { cue }` and SoundPalette has `play()`.
**Verify**: This may already work — AutoJuice fires sound cue which calls SoundPalette.play() which enqueues PlayTone commands. Check if wiring is complete.

---

## Test Count Targets
- Round 5: ~40 new tests (restitution: 8, scene isolation: 12, multi-layer tilemap: 12, aim preview: 8)
- Round 6: ~30 new tests (UI tap: 6, dialogue branching: 10, sprite tiles: 8, SFX wiring: 6)
- Total target: 1063 + 70 = ~1133 tests
