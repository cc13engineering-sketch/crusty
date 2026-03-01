# UI & Sound Implementation Specification

**Covers**: Gap 3 (Extended DialogueQueue), Gap 5 (UiCanvas Tap Detection), Gap 8 (Persistent SoundScape Cues)
**Target**: Trap Links game on Crusty engine (Rust -> WASM -> Canvas 2D)

---

## Gap 3: Extended DialogueQueue -- Branching Conversations

### Problem Statement

The current `DialogueQueue` (`dialogue.rs`) supports three message types (`Dialogue`, `Notification`, `FloatingText`) that display passively and expire on a timer. There is no mechanism for:
- Presenting the player with a set of labeled choices
- Branching to different dialogue lines based on the chosen option
- Blocking the dialogue queue until the player makes a selection
- Emitting events when a choice is made (for game logic integration)

Trap Links needs branching dialogue for: NPC conversations (shop interactions, caddy party advice, save-point confirmations), level-up upgrade selection (choose 1 of 3 upgrades), and capture-attempt confirmation prompts.

### Current State Analysis

```
DialogueQueue
  messages: Vec<Message>
  push(msg) -> void
  tick(dt) -> void         // decrements remaining, removes expired
  active() -> Iterator     // yields non-expired messages
  clear() -> void

Message
  kind: MessageKind        // Dialogue | Notification | FloatingText
  text: String
  world_pos: Option<(f64, f64)>
  duration: f64
  remaining: f64
  color: Color
```

The tick loop in `engine.rs` calls `self.dialogue.tick(dt)` once per frame and renders messages in `render_hud()`. Dialogue messages render as bottom-of-screen text boxes. There is no interaction model -- messages are fire-and-forget.

### Design: Conversation System

The design adds a `Conversation` struct that sits alongside the existing `DialogueQueue`. A conversation is a directed graph of `DialogueNode`s. Each node is either a statement (auto-advance after display) or a choice (blocks until the player taps an option). The conversation runs independently from the existing message queue, so notifications and floating text continue to work during conversations.

### Struct/Enum/Trait Additions

All additions go in `dialogue.rs`.

```rust
/// Unique identifier for a dialogue node within a conversation.
pub type NodeId = u32;

/// A single option the player can choose.
#[derive(Clone, Debug)]
pub struct DialogueChoice {
    /// Display text for the choice button.
    pub label: String,
    /// The node to jump to when this choice is selected.
    /// None means "end the conversation".
    pub next: Option<NodeId>,
    /// Optional EventBus channel to emit when this choice is selected.
    pub event_channel: Option<String>,
    /// Optional key-value data to attach to the emitted event.
    pub event_data: Vec<(String, String)>,
}

/// A single node in the conversation graph.
#[derive(Clone, Debug)]
pub enum DialogueNode {
    /// Display a line of text, then auto-advance to `next` after `duration` seconds.
    /// If `next` is None, the conversation ends after this line.
    Statement {
        speaker: Option<String>,
        text: String,
        duration: f64,
        next: Option<NodeId>,
    },
    /// Display a prompt and a set of choices. Blocks until the player selects one.
    Choice {
        speaker: Option<String>,
        prompt: String,
        choices: Vec<DialogueChoice>,
    },
}

/// The state of an active conversation.
#[derive(Clone, Debug, PartialEq)]
pub enum ConversationState {
    /// Displaying a Statement node; counting down.
    Speaking { remaining: f64 },
    /// Waiting for the player to tap a choice.
    WaitingForChoice,
    /// The conversation has ended.
    Finished,
}

/// An active conversation session. Holds the node graph and current playback state.
#[derive(Clone, Debug)]
pub struct Conversation {
    /// All nodes in this conversation, keyed by NodeId.
    pub nodes: Vec<(NodeId, DialogueNode)>,
    /// The ID of the currently active node.
    pub current_node: NodeId,
    /// Playback state.
    pub state: ConversationState,
    /// Collected events to emit when choices are made.
    /// Drained by the engine each frame.
    pub pending_events: Vec<ConversationEvent>,
    /// Text color for the speaker name.
    pub speaker_color: Color,
    /// Text color for the body text.
    pub text_color: Color,
    /// Background color for the dialogue box.
    pub box_color: Color,
}

/// An event produced when the player makes a dialogue choice.
#[derive(Clone, Debug)]
pub struct ConversationEvent {
    pub channel: String,
    pub data: Vec<(String, String)>,
}
```

### Builder API

```rust
/// Fluent builder for constructing conversations.
#[derive(Clone, Debug)]
pub struct ConversationBuilder {
    nodes: Vec<(NodeId, DialogueNode)>,
    next_id: NodeId,
    speaker_color: Color,
    text_color: Color,
    box_color: Color,
}

impl ConversationBuilder {
    pub fn new() -> Self;

    /// Set the speaker name color.
    pub fn speaker_color(mut self, c: Color) -> Self;

    /// Set the body text color.
    pub fn text_color(mut self, c: Color) -> Self;

    /// Set the dialogue box background color.
    pub fn box_color(mut self, c: Color) -> Self;

    /// Add a Statement node. Returns the assigned NodeId.
    pub fn add_statement(
        &mut self,
        speaker: Option<&str>,
        text: &str,
        duration: f64,
        next: Option<NodeId>,
    ) -> NodeId;

    /// Add a Choice node. Returns the assigned NodeId.
    /// `choices` is a Vec of (label, next_node, optional_event_channel).
    pub fn add_choice(
        &mut self,
        speaker: Option<&str>,
        prompt: &str,
        choices: Vec<DialogueChoice>,
    ) -> NodeId;

    /// Build the Conversation, starting at the node with the lowest NodeId.
    pub fn build(self) -> Conversation;

    /// Build the Conversation, starting at a specific NodeId.
    pub fn build_from(self, start: NodeId) -> Conversation;
}
```

### Methods on Conversation

```rust
impl Conversation {
    /// Get the currently active DialogueNode.
    pub fn current(&self) -> Option<&DialogueNode>;

    /// Advance the conversation timer by dt.
    /// For Statement nodes, decrements remaining time and auto-advances.
    /// For Choice nodes, does nothing (waits for select_choice).
    /// Returns true if the conversation is still active (not Finished).
    pub fn tick(&mut self, dt: f64) -> bool;

    /// Select a choice by index (0-based) in the current Choice node.
    /// Advances to the chosen node's `next` target.
    /// Returns None if the current node is not a Choice, or index is out of bounds.
    /// Returns Some(ConversationEvent) if the choice had an event_channel.
    pub fn select_choice(&mut self, index: usize) -> Option<ConversationEvent>;

    /// Whether the conversation has ended.
    pub fn is_finished(&self) -> bool;

    /// Drain all pending events (choice selections that emitted events).
    pub fn drain_events(&mut self) -> Vec<ConversationEvent>;

    /// Get the choice labels for the current node, if it is a Choice node.
    pub fn current_choices(&self) -> Option<Vec<&str>>;

    /// Get the speaker name for the current node, if any.
    pub fn current_speaker(&self) -> Option<&str>;

    /// Get the display text for the current node.
    pub fn current_text(&self) -> Option<&str>;
}
```

### Modifications to Existing Code

#### `DialogueQueue` (dialogue.rs)

Add a conversation slot:

```rust
pub struct DialogueQueue {
    pub messages: Vec<Message>,
    /// The currently active branching conversation, if any.
    pub conversation: Option<Conversation>,
}
```

Modify `DialogueQueue::new()`:
```rust
pub fn new() -> Self {
    Self {
        messages: Vec::new(),
        conversation: None,
    }
}
```

Add conversation management methods:
```rust
impl DialogueQueue {
    /// Start a branching conversation. Replaces any active conversation.
    pub fn start_conversation(&mut self, conv: Conversation) {
        self.conversation = Some(conv);
    }

    /// Whether a conversation is currently active (not finished).
    pub fn has_active_conversation(&self) -> bool {
        self.conversation.as_ref().map_or(false, |c| !c.is_finished())
    }

    /// Select a choice in the active conversation.
    /// Returns the ConversationEvent if the choice emitted one.
    pub fn select_choice(&mut self, index: usize) -> Option<ConversationEvent> {
        self.conversation.as_mut().and_then(|c| c.select_choice(index))
    }

    /// Get choice labels for the current conversation node.
    pub fn current_choices(&self) -> Option<Vec<&str>> {
        self.conversation.as_ref().and_then(|c| c.current_choices())
    }
}
```

Modify `DialogueQueue::tick()` to also tick the conversation:
```rust
pub fn tick(&mut self, dt: f64) {
    // Existing message ticking (unchanged)
    for msg in &mut self.messages {
        msg.remaining -= dt;
        if let MessageKind::FloatingText = msg.kind {
            if let Some((ref mut _wx, ref mut wy)) = msg.world_pos {
                *wy -= 20.0 * dt;
            }
        }
    }
    self.messages.retain(|m| m.remaining > 0.0);

    // Tick active conversation
    if let Some(ref mut conv) = self.conversation {
        conv.tick(dt);
        if conv.is_finished() {
            self.conversation = None;
        }
    }
}
```

Modify `DialogueQueue::clear()`:
```rust
pub fn clear(&mut self) {
    self.messages.clear();
    self.conversation = None;
}
```

#### `engine.rs` -- Tick Loop

After the existing `self.dialogue.tick(dt)` call, add conversation event draining:

```rust
// Drain conversation events into EventBus
if let Some(ref mut conv) = self.dialogue.conversation {
    for event in conv.drain_events() {
        let mut bus_event = crate::event_bus::BusEvent::new(&event.channel);
        for (k, v) in &event.data {
            bus_event = bus_event.with_text(k, v);
        }
        self.event_bus.publish(bus_event);
    }
}
```

#### `engine.rs` -- Render HUD

Add conversation rendering inside `render_hud()`, after the existing dialogue message rendering block. When a conversation is active, render:
1. The dialogue box (same position as `MessageKind::Dialogue`)
2. The speaker name (if present) in `speaker_color`
3. The body text in `text_color`
4. For Choice nodes: render each choice as a numbered button below the prompt

The choice buttons are rendered as `WidgetKind::Button` entries dynamically added to a temporary `UiCanvas` or rendered inline. The recommended approach is inline rendering with stored bounding rectangles for hit-testing (see Gap 5 integration below).

#### Integration with Gap 5 (UiCanvas tap detection)

When a `Conversation` is in `WaitingForChoice` state, the engine renders choice buttons and checks tap events against them. On tap hit:

```rust
// In tick(), after gesture processing:
if self.dialogue.has_active_conversation() {
    if let Some(choices) = self.dialogue.current_choices() {
        for gesture_event in self.event_bus.on("gesture:tap") {
            let tx = gesture_event.get_f64("x").unwrap_or(0.0);
            let ty = gesture_event.get_f64("y").unwrap_or(0.0);
            // Hit-test against choice button positions
            // (positions computed from screen layout: bottom of screen,
            //  stacked vertically, each button 440x30 px, centered)
            let base_y = (self.height as f64) - 44.0 - (choices.len() as f64 * 36.0);
            for (i, _label) in choices.iter().enumerate() {
                let btn_x = 20.0;
                let btn_y = base_y + (i as f64 * 36.0);
                let btn_w = 440.0;
                let btn_h = 30.0;
                if tx >= btn_x && tx < btn_x + btn_w
                    && ty >= btn_y && ty < btn_y + btn_h
                {
                    self.dialogue.select_choice(i);
                    // Play UI click sound
                    self.sound_palette.play("ui_click", &mut self.sound_queue);
                    break;
                }
            }
        }
    }
}
```

### Test Cases

**Test 1: Statement auto-advance**
Create a conversation with two Statement nodes (A -> B). Tick past A's duration. Verify the current node advances to B. Tick past B's duration. Verify conversation is Finished.

**Test 2: Choice blocks until selection**
Create a conversation with a Choice node with 3 options. Tick for 10 seconds. Verify state remains `WaitingForChoice`. Call `select_choice(1)`. Verify the conversation advances to the node specified by choice index 1's `next` field.

**Test 3: Choice emits ConversationEvent**
Create a Choice node where option 0 has `event_channel = Some("shop_buy")` and `event_data = [("item", "potion")]`. Select choice 0. Drain events. Verify exactly one `ConversationEvent` with channel `"shop_buy"` and data key `"item"` = `"potion"`.

**Test 4: Branching graph**
Build a 4-node conversation: Node 0 (Statement -> 1), Node 1 (Choice: option A -> 2, option B -> 3), Node 2 (Statement -> None), Node 3 (Statement -> None). Select option B at node 1. Verify conversation reaches node 3 and then finishes. Repeat selecting option A. Verify it reaches node 2.

**Test 5: Conversation ends on None next**
Create a single Statement node with `next: None`. Tick past duration. Verify `is_finished()` returns true and `conversation` field on `DialogueQueue` is set to `None` after the next `DialogueQueue::tick()`.

**Test 6: Builder assigns sequential NodeIds**
Use `ConversationBuilder` to add 3 nodes. Verify returned IDs are 0, 1, 2. Build and verify the start node is 0.

**Test 7: select_choice out of bounds returns None**
Create a Choice node with 2 options. Call `select_choice(5)`. Verify it returns `None` and state remains `WaitingForChoice`.

**Test 8: select_choice on Statement node returns None**
Create a Statement node. Call `select_choice(0)` while it is active. Verify it returns `None`.

**Test 9: DialogueQueue tick advances conversation**
Push a conversation into `DialogueQueue`. Call `dialogue.tick(dt)` with enough time to expire the first Statement. Verify the conversation advanced.

**Test 10: DialogueQueue clear removes conversation**
Start a conversation. Call `dialogue.clear()`. Verify `has_active_conversation()` returns false.

### Estimated Lines of Code

| Component | Lines |
|-----------|-------|
| `DialogueChoice`, `DialogueNode`, `ConversationState`, `ConversationEvent` structs/enums | ~55 |
| `Conversation` struct + impl (tick, select_choice, accessors) | ~120 |
| `ConversationBuilder` struct + impl | ~80 |
| `DialogueQueue` modifications (new field, new methods, modified tick/clear) | ~40 |
| `engine.rs` modifications (event draining, choice tap handling, rendering) | ~70 |
| Tests (10 tests) | ~200 |
| **Total** | **~565** |

---

## Gap 5: UiCanvas Tap Detection

### Problem Statement

`UiCanvas` already has a `hit_test(x, y, width, height) -> Option<String>` method that returns the `action` string of the first visible `Button` widget hit at the given screen coordinate. However, the engine tick loop does not wire tap gestures to this hit-test, and there is no callback/event mechanism to notify game code when a button is tapped. The gap is in the **integration layer**, not the primitive itself.

Trap Links needs this for: Fight HUD buttons (SHOT, SPECIAL, item slots, CAPTURE), overworld HUD (BAG button), shop menus, and level-up choice buttons.

### Current State Analysis

`UiCanvas` in `ui_canvas.rs`:
- `hit_test(&self, x, y, width, height) -> Option<String>` -- already implemented correctly
- `WidgetKind::Button` -- already has `label`, `action`, `width`, `height`, `color`, `text_color`
- `render()` -- already draws buttons with background rect, border, centered label
- Missing: no connection between `GestureRecognizer` tap events and `hit_test()`
- Missing: no EventBus emission on button tap
- Missing: no visual feedback (pressed state) on buttons
- Missing: no way to disable a button without hiding it

In `engine.rs`:
- Gestures are processed and published to EventBus as `"gesture:tap"`, `"gesture:double_tap"`, etc.
- `UiCanvas` is rendered... nowhere in the tick loop currently (the `render_hud()` method is hardcoded). The `ui_canvas` field exists on `Engine` but `ui_canvas.render()` is never called in `tick()`.

### Design: Tap-to-Action Pipeline

The design adds:
1. A `disabled` field on `UiWidget` for graying out buttons without hiding them
2. A `pressed_timer` for visual press feedback
3. Automatic tap-to-hit-test wiring in the engine tick loop
4. EventBus emission of `"ui:button_tap"` events with the action string as payload
5. Visual press feedback (darkened button color for ~0.1s after tap)

### Struct/Enum/Trait Additions

In `ui_canvas.rs`:

```rust
/// Tap result returned when a button is hit.
#[derive(Clone, Debug, PartialEq)]
pub struct TapResult {
    /// The widget id that was tapped.
    pub widget_id: String,
    /// The action string from the Button widget.
    pub action: String,
}
```

### Modifications to Existing Code

#### `UiWidget` (ui_canvas.rs)

Add fields:

```rust
pub struct UiWidget {
    pub id: String,
    pub kind: WidgetKind,
    pub anchor: Anchor,
    pub offset: (f64, f64),
    pub visible: bool,
    /// When true, the widget is drawn grayed out and does not respond to taps.
    pub disabled: bool,
    /// Countdown timer for visual press feedback (seconds remaining).
    pub pressed_timer: f64,
}
```

Modify `UiWidget::new()`:
```rust
pub fn new(id: &str, kind: WidgetKind, anchor: Anchor, offset: (f64, f64)) -> Self {
    Self {
        id: id.to_string(),
        kind,
        anchor,
        offset,
        visible: true,
        disabled: false,
        pressed_timer: 0.0,
    }
}
```

#### `UiCanvas` (ui_canvas.rs)

Modify `hit_test()` to also check `disabled`:

```rust
pub fn hit_test(&self, x: f64, y: f64, width: f64, height: f64) -> Option<TapResult> {
    for widget in &self.widgets {
        if !widget.visible || widget.disabled {
            continue;
        }
        if let WidgetKind::Button { action, width: bw, height: bh, .. } = &widget.kind {
            let (wx, wy) = resolve_position(&widget.anchor, widget.offset, width, height);
            let wx = wx as f64;
            let wy = wy as f64;
            if x >= wx && x < wx + bw && y >= wy && y < wy + bh {
                return Some(TapResult {
                    widget_id: widget.id.clone(),
                    action: action.clone(),
                });
            }
        }
    }
    None
}
```

**Note on backward compatibility**: The return type changes from `Option<String>` to `Option<TapResult>`. Callers that only need the action string can access `result.action`. Since there are no external callers (only tests), this is a safe change. Alternatively, keep `hit_test` returning `Option<String>` and add a new `hit_test_full` method. The recommended approach is to change the return type, since the existing tests are internal and easily updated.

Add a new method to trigger press feedback:

```rust
impl UiCanvas {
    /// Mark a button as pressed (starts visual feedback timer).
    pub fn press(&mut self, widget_id: &str) {
        if let Some(w) = self.get_mut(widget_id) {
            w.pressed_timer = 0.1; // 100ms feedback
        }
    }

    /// Tick all widget timers (press feedback countdown).
    pub fn tick(&mut self, dt: f64) {
        for widget in &mut self.widgets {
            if widget.pressed_timer > 0.0 {
                widget.pressed_timer -= dt;
                if widget.pressed_timer < 0.0 {
                    widget.pressed_timer = 0.0;
                }
            }
        }
    }
}
```

#### `UiCanvas::render()` (ui_canvas.rs)

Modify the Button rendering branch to support disabled state and press feedback:

```rust
WidgetKind::Button { label, width: bw, height: bh, color, text_color, .. } => {
    let render_color = if widget.disabled {
        // Grayed out: reduce all channels by 60%
        Color::from_rgba(
            (color.r as f64 * 0.4) as u8,
            (color.g as f64 * 0.4) as u8,
            (color.b as f64 * 0.4) as u8,
            color.a,
        )
    } else if widget.pressed_timer > 0.0 {
        // Pressed: brighten by 40%
        Color::from_rgba(
            (color.r as f64 * 1.4).min(255.0) as u8,
            (color.g as f64 * 1.4).min(255.0) as u8,
            (color.b as f64 * 1.4).min(255.0) as u8,
            color.a,
        )
    } else {
        *color
    };

    let render_text_color = if widget.disabled {
        Color::from_rgba(
            (text_color.r as f64 * 0.5) as u8,
            (text_color.g as f64 * 0.5) as u8,
            (text_color.b as f64 * 0.5) as u8,
            text_color.a,
        )
    } else {
        *text_color
    };

    // Background rect
    shapes::fill_rect(fb, x as f64, y as f64, *bw, *bh, render_color);
    // Border
    shapes::draw_rect(fb, x as f64, y as f64, *bw, *bh, render_text_color);
    // Centered label text
    let tw = text::text_width(label, 1);
    let text_x = x + (*bw as i32 - tw) / 2;
    let text_y = y + (*bh as i32 - 7) / 2;
    text::draw_text(fb, text_x, text_y, label, render_text_color, 1);
}
```

#### `engine.rs` -- Tick Loop Modifications

Add UiCanvas tap detection after gesture processing, and add rendering call:

```rust
// --- In tick(), after gesture processing block ---

// UiCanvas: check tap gestures against button widgets
for event in self.event_bus.on("gesture:tap").collect::<Vec<_>>() {
    let tx = event.get_f64("x").unwrap_or(0.0);
    let ty = event.get_f64("y").unwrap_or(0.0);
    if let Some(tap_result) = self.ui_canvas.hit_test(
        tx, ty, self.width as f64, self.height as f64
    ) {
        // Emit a bus event for the tapped button
        self.event_bus.publish(
            crate::event_bus::BusEvent::new("ui:button_tap")
                .with_text("action", &tap_result.action)
                .with_text("widget_id", &tap_result.widget_id)
                .with_f64("x", tx)
                .with_f64("y", ty)
        );
        // Trigger press visual feedback
        self.ui_canvas.press(&tap_result.widget_id);
        // Play UI click sound
        self.sound_palette.play("ui_click", &mut self.sound_queue);
    }
}

// Tick UiCanvas timers (press feedback)
self.ui_canvas.tick(dt);
```

Add UiCanvas rendering in the rendering section, after HUD and before screen effects:

```rust
// UiCanvas overlay (after HUD, before screen effects)
self.ui_canvas.render(
    &mut self.framebuffer,
    &self.global_state,
    self.width as f64,
    self.height as f64,
);
```

#### `engine.rs` -- reset_game_state()

The existing `self.ui_canvas.clear()` call is sufficient. No change needed.

### Event Flow Summary

```
GestureRecognizer::on_touch_end()
  -> Gesture::Tap { x, y }
    -> engine tick(): EventBus.publish("gesture:tap", x, y)
      -> engine tick(): UiCanvas.hit_test(x, y)
        -> TapResult { widget_id, action }
          -> EventBus.publish("ui:button_tap", action, widget_id, x, y)
          -> UiCanvas.press(widget_id)  // visual feedback
          -> SoundPalette.play("ui_click")
```

Game code listens on `EventBus.on("ui:button_tap")` and reads the `"action"` payload to determine what to do.

### Test Cases

**Test 1: hit_test returns TapResult with widget_id and action**
Create a UiCanvas with a Button (id="btn1", action="fire"). Hit-test at a point inside the button. Verify returned `TapResult` has `widget_id = "btn1"` and `action = "fire"`.

**Test 2: Disabled button not hit-testable**
Create a Button widget, set `disabled = true`. Hit-test at a point inside it. Verify `None` is returned.

**Test 3: Invisible button not hit-testable**
(Existing test already covers this -- `hit_test_invisible_button_ignored`.) Verify it still passes with the new `TapResult` return type.

**Test 4: press() sets pressed_timer**
Create a UiCanvas with a Button. Call `press("btn_id")`. Verify the widget's `pressed_timer` is 0.1.

**Test 5: tick() decrements pressed_timer**
Set `pressed_timer = 0.1`. Call `tick(0.05)`. Verify `pressed_timer` is approximately 0.05. Call `tick(0.06)`. Verify `pressed_timer` is 0.0 (clamped, not negative).

**Test 6: Multiple buttons hit-test returns first match**
Create two overlapping buttons (Button A at (0,0) 100x100, Button B at (50,50) 100x100). Hit-test at (75, 75). Verify Button A is returned (first in widget list).

**Test 7: hit_test with non-button widgets returns None**
Create a UiCanvas with only Label and Bar widgets. Hit-test at any position. Verify `None` is returned.

**Test 8: Disabled button renders differently (visual check via pixel inspection)**
Create a Button with color (100, 100, 100, 255). Set `disabled = true`. Render to a Framebuffer. Check that the pixel at the button center has RGB values around (40, 40, 40) (60% reduction).

**Test 9: Pressed button renders differently (visual check via pixel inspection)**
Create a Button with color (100, 100, 100, 255). Set `pressed_timer = 0.05`. Render to a Framebuffer. Check that the pixel at the button center has RGB values around (140, 140, 140) (40% increase).

**Test 10: hit_test respects anchor positioning**
Create a Button anchored at `BottomCenter` with offset (-40, -40) and size 80x30. On an 800x600 screen, the button should be at (360, 560). Hit-test at (400, 575). Verify hit. Hit-test at (300, 575). Verify miss.

### Estimated Lines of Code

| Component | Lines |
|-----------|-------|
| `TapResult` struct | ~8 |
| `UiWidget` field additions (`disabled`, `pressed_timer`) | ~6 |
| `UiWidget::new()` modification | ~2 |
| `UiCanvas::hit_test()` modification (return `TapResult`, check `disabled`) | ~8 (net change) |
| `UiCanvas::press()` method | ~8 |
| `UiCanvas::tick()` method | ~12 |
| `UiCanvas::render()` Button branch modification (disabled + pressed colors) | ~30 |
| `engine.rs` tap-to-hit-test wiring | ~20 |
| `engine.rs` UiCanvas rendering call | ~6 |
| Test updates (existing tests adapted for `TapResult`) | ~20 |
| New tests (10 tests) | ~180 |
| **Total** | **~300** |

---

## Gap 8: Persistent SoundScape Cues via AutoJuice

### Problem Statement

The `AutoJuiceSystem` already collects sound palette names in its `sound_queue: Vec<String>` field when effects fire. The `SoundPalette` already has one-shot sound profiles (e.g., `"impact"`, `"pickup"`, `"ui_click"`). The `SoundCommandQueue` already serializes commands to JSON for the JS audio driver.

However, there is **no code in the engine tick loop that reads `auto_juice.sound_queue` and plays those sounds through `sound_palette` into `sound_queue`**. The pipeline is:

```
AutoJuice.apply() -> sound_queue: Vec<String> accumulates palette names
                     [MISSING BRIDGE]
SoundPalette.play(name, &mut SoundCommandQueue)
SoundCommandQueue.drain_json() -> JS audio driver
```

Additionally, the default `SoundPalette` does not include the game-specific sound profiles that Trap Links needs (e.g., `"shot_fire"`, `"bounce_wall"`, `"hole_in"`, `"trap_trigger"`, etc.).

### Current State Analysis

`AutoJuiceSystem` (`auto_juice.rs`):
- `sound_queue: Vec<String>` -- populated in `fire_effect_inner()` when `effect.sound` is Some
- Cleared at the start of each `apply()` call
- Never read by any other code

`SoundPalette` (`sound.rs`):
- `play(event_name, &mut SoundCommandQueue) -> bool` -- looks up profile, pushes commands
- `default_palette()` -- has 6 profiles: `"impact"`, `"pickup"`, `"explosion"`, `"ui_click"`, `"ambient_wind"`, `"game_over"`
- `register(name, commands)` -- allows custom profile registration

`SoundCommandQueue` (`sound.rs`):
- `push(cmd)` -- adds a command
- `drain_json() -> String` -- serialized and consumed by JS via WASM binding `drain_sound_commands()`

`Engine` (`engine.rs`):
- Has `sound_queue: SoundCommandQueue` and `sound_palette: SoundPalette`
- Has `auto_juice: AutoJuiceSystem`
- `auto_juice.apply()` is never called in `tick()` (the system exists but is not wired into the tick loop)
- `drain_sound_commands()` WASM binding calls `eng.sound_queue.drain_json()`

### Design: AutoJuice-to-Sound Bridge + Trap Links Palette

The design adds:
1. A bridge in the engine tick loop that drains `auto_juice.sound_queue` and plays each name through `sound_palette` into `sound_queue`
2. Wiring `auto_juice.apply()` into the tick loop (currently missing entirely)
3. A `trap_links_palette()` constructor on `SoundPalette` with all game-specific sound profiles
4. A `SoundPalette::merge()` method to combine palettes
5. A convenience `Engine::play_sound(name)` method for game code to trigger sounds directly

### Struct/Enum/Trait Additions

In `sound.rs`:

```rust
impl SoundPalette {
    /// Merge another palette into this one. Existing profiles with the same
    /// name are overwritten by the incoming palette.
    pub fn merge(&mut self, other: &SoundPalette) {
        for (name, commands) in &other.profiles {
            self.profiles.insert(name.clone(), commands.clone());
        }
    }

    /// Build a palette with Trap Links game-specific sound profiles.
    pub fn trap_links_palette() -> Self {
        let mut palette = Self::new();

        // "shot_fire" -- whoosh + spring release
        palette.register("shot_fire", vec![
            SoundCommand::PlayTone {
                frequency: 300.0,
                duration: 0.12,
                volume: 0.5,
                waveform: Waveform::Triangle,
                attack: 0.005,
                decay: 0.1,
            },
            SoundCommand::PlayNoise {
                duration: 0.08,
                volume: 0.25,
                filter_freq: 3000.0,
            },
        ]);

        // "bounce_wall" -- low thock
        palette.register("bounce_wall", vec![
            SoundCommand::PlayTone {
                frequency: 120.0,
                duration: 0.08,
                volume: 0.5,
                waveform: Waveform::Square,
                attack: 0.003,
                decay: 0.06,
            },
        ]);

        // "bounce_bumper" -- bright ping
        palette.register("bounce_bumper", vec![
            SoundCommand::PlayTone {
                frequency: 1200.0,
                duration: 0.1,
                volume: 0.45,
                waveform: Waveform::Sine,
                attack: 0.002,
                decay: 0.08,
            },
            SoundCommand::PlayTone {
                frequency: 1800.0,
                duration: 0.08,
                volume: 0.25,
                waveform: Waveform::Sine,
                attack: 0.01,
                decay: 0.06,
            },
        ]);

        // "sand_drag" -- muffled scraping
        palette.register("sand_drag", vec![
            SoundCommand::PlayNoise {
                duration: 0.3,
                volume: 0.2,
                filter_freq: 600.0,
            },
        ]);

        // "hole_in" -- ascending arpeggio + chime
        palette.register("hole_in", vec![
            SoundCommand::PlayTone {
                frequency: 523.0,   // C5
                duration: 0.12,
                volume: 0.4,
                waveform: Waveform::Sine,
                attack: 0.005,
                decay: 0.1,
            },
            SoundCommand::PlayTone {
                frequency: 659.0,   // E5
                duration: 0.12,
                volume: 0.4,
                waveform: Waveform::Sine,
                attack: 0.06,
                decay: 0.1,
            },
            SoundCommand::PlayTone {
                frequency: 784.0,   // G5
                duration: 0.12,
                volume: 0.4,
                waveform: Waveform::Sine,
                attack: 0.12,
                decay: 0.1,
            },
            SoundCommand::PlayTone {
                frequency: 1047.0,  // C6
                duration: 0.25,
                volume: 0.5,
                waveform: Waveform::Sine,
                attack: 0.18,
                decay: 0.2,
            },
        ]);

        // "fight_defeat" -- descending wail
        palette.register("fight_defeat", vec![
            SoundCommand::PlayTone {
                frequency: 440.0,
                duration: 0.25,
                volume: 0.5,
                waveform: Waveform::Sawtooth,
                attack: 0.01,
                decay: 0.2,
            },
            SoundCommand::PlayTone {
                frequency: 330.0,
                duration: 0.3,
                volume: 0.5,
                waveform: Waveform::Sawtooth,
                attack: 0.1,
                decay: 0.25,
            },
            SoundCommand::PlayTone {
                frequency: 220.0,
                duration: 0.4,
                volume: 0.6,
                waveform: Waveform::Sawtooth,
                attack: 0.15,
                decay: 0.35,
            },
        ]);

        // "eagle" -- fanfare 5-note
        palette.register("eagle", vec![
            SoundCommand::PlayTone {
                frequency: 523.0, duration: 0.1, volume: 0.5,
                waveform: Waveform::Triangle, attack: 0.005, decay: 0.08,
            },
            SoundCommand::PlayTone {
                frequency: 659.0, duration: 0.1, volume: 0.5,
                waveform: Waveform::Triangle, attack: 0.05, decay: 0.08,
            },
            SoundCommand::PlayTone {
                frequency: 784.0, duration: 0.1, volume: 0.5,
                waveform: Waveform::Triangle, attack: 0.1, decay: 0.08,
            },
            SoundCommand::PlayTone {
                frequency: 1047.0, duration: 0.1, volume: 0.5,
                waveform: Waveform::Triangle, attack: 0.15, decay: 0.08,
            },
            SoundCommand::PlayTone {
                frequency: 1319.0, duration: 0.3, volume: 0.6,
                waveform: Waveform::Triangle, attack: 0.2, decay: 0.25,
            },
        ]);

        // "capture" -- magical sparkle sweep
        palette.register("capture", vec![
            SoundCommand::PlayTone {
                frequency: 880.0, duration: 0.1, volume: 0.3,
                waveform: Waveform::Sine, attack: 0.005, decay: 0.08,
            },
            SoundCommand::PlayTone {
                frequency: 1320.0, duration: 0.1, volume: 0.3,
                waveform: Waveform::Sine, attack: 0.05, decay: 0.08,
            },
            SoundCommand::PlayTone {
                frequency: 1760.0, duration: 0.15, volume: 0.35,
                waveform: Waveform::Sine, attack: 0.1, decay: 0.12,
            },
            SoundCommand::PlayNoise {
                duration: 0.2, volume: 0.15, filter_freq: 6000.0,
            },
        ]);

        // "level_up" -- 8-bit ascending chime
        palette.register("level_up", vec![
            SoundCommand::PlayTone {
                frequency: 440.0, duration: 0.08, volume: 0.4,
                waveform: Waveform::Square, attack: 0.005, decay: 0.06,
            },
            SoundCommand::PlayTone {
                frequency: 554.0, duration: 0.08, volume: 0.4,
                waveform: Waveform::Square, attack: 0.04, decay: 0.06,
            },
            SoundCommand::PlayTone {
                frequency: 659.0, duration: 0.08, volume: 0.4,
                waveform: Waveform::Square, attack: 0.08, decay: 0.06,
            },
            SoundCommand::PlayTone {
                frequency: 880.0, duration: 0.2, volume: 0.5,
                waveform: Waveform::Square, attack: 0.12, decay: 0.15,
            },
        ]);

        // "step_grass" -- soft footstep
        palette.register("step_grass", vec![
            SoundCommand::PlayNoise {
                duration: 0.06, volume: 0.1, filter_freq: 800.0,
            },
        ]);

        // "step_stone" -- harder footstep
        palette.register("step_stone", vec![
            SoundCommand::PlayTone {
                frequency: 200.0, duration: 0.04, volume: 0.15,
                waveform: Waveform::Square, attack: 0.002, decay: 0.03,
            },
        ]);

        // "trap_trigger" -- dramatic sting
        palette.register("trap_trigger", vec![
            SoundCommand::PlayTone {
                frequency: 180.0, duration: 0.15, volume: 0.7,
                waveform: Waveform::Sawtooth, attack: 0.005, decay: 0.12,
            },
            SoundCommand::PlayTone {
                frequency: 90.0, duration: 0.3, volume: 0.6,
                waveform: Waveform::Square, attack: 0.05, decay: 0.25,
            },
            SoundCommand::PlayNoise {
                duration: 0.15, volume: 0.35, filter_freq: 1200.0,
            },
        ]);

        palette
    }
}
```

### Modifications to Existing Code

#### `engine.rs` -- Tick Loop: Wire AutoJuice.apply()

Currently `auto_juice.apply()` is never called in `tick()`. Add the call after gameplay systems but before rendering. The call requires collecting tags, spawn/despawn tags, and providing the various effect sinks.

```rust
// --- After gameplay systems, before rendering ---

// AutoJuice: evaluate trigger rules and fire effects
{
    // Snapshot spawn/despawn tags for this frame
    // (These would be populated by the lifecycle system;
    //  for now, pass empty slices -- game code populates via EventBus.)
    let spawn_tags: Vec<String> = Vec::new();
    let despawn_tags: Vec<String> = Vec::new();

    let tags_lookup = |entity: crate::ecs::Entity| -> Vec<String> {
        self.world.tags.get(entity)
            .map(|t| t.all().iter().map(|s| s.to_string()).collect())
            .unwrap_or_default()
    };

    self.auto_juice.apply(
        &self.events,
        &self.event_bus,
        &tags_lookup,
        &spawn_tags,
        &despawn_tags,
        &mut self.particles,
        &mut self.post_fx,
        &mut self.screen_fx,
        self.frame,
    );
}
```

#### `engine.rs` -- Tick Loop: AutoJuice Sound Bridge

Add immediately after the `auto_juice.apply()` block:

```rust
// AutoJuice -> SoundPalette -> SoundCommandQueue bridge
for sound_name in &self.auto_juice.sound_queue {
    self.sound_palette.play(sound_name, &mut self.sound_queue);
}
```

This is the critical missing bridge. It reads each palette name accumulated by AutoJuice during `apply()` and plays it through the palette into the command queue, which is then drained to JSON by the JS audio driver via `drain_sound_commands()`.

#### `engine.rs` -- AutoJuice Freeze Frame Integration

Add freeze frame check at the top of `tick()`:

```rust
pub fn tick(&mut self, dt: f64) {
    let dt = dt.min(MAX_FRAME_DT);

    // AutoJuice freeze frames: skip game logic while frozen
    if self.auto_juice.tick_freeze(dt) {
        // Still render, but skip physics and game logic
        // (rendering block runs below)
        self.render_frozen_frame();
        return;
    }

    // ... rest of tick
}
```

Where `render_frozen_frame()` re-renders the last frame state without advancing game logic. This is a simplified path -- for the MVP, we can simply check freeze at the top and skip the physics step:

```rust
// Simpler alternative: just gate the physics accumulator
let effective_dt = if self.auto_juice.freeze_remaining > 0.0 {
    self.auto_juice.tick_freeze(dt);
    0.0 // freeze game time
} else {
    dt
};
```

#### `Engine` -- Convenience Method

```rust
impl Engine {
    /// Play a named sound profile. Convenience wrapper for game code.
    pub fn play_sound(&mut self, name: &str) -> bool {
        self.sound_palette.play(name, &mut self.sound_queue)
    }
}
```

#### `engine.rs` -- Engine::new() Palette Initialization

Merge the Trap Links palette into the default palette at construction:

```rust
// In Engine::new():
let mut sound_palette = crate::sound::SoundPalette::default_palette();
sound_palette.merge(&crate::sound::SoundPalette::trap_links_palette());
// ... assign to self.sound_palette
```

Alternatively, game-specific code can call `engine.sound_palette.merge(...)` during initialization rather than hardcoding it in the engine constructor. This is preferable for engine generality. The `trap_links_palette()` method lives in `sound.rs` but is only called by the game module.

### Event Flow Summary

```
AutoJuice Rule triggers (collision/spawn/despawn/bus event)
  -> AutoJuice.apply() fires JuiceEffect
    -> effect.sound = Some("bounce_bumper")
      -> auto_juice.sound_queue.push("bounce_bumper")

Engine tick() bridge:
  -> for name in auto_juice.sound_queue:
       sound_palette.play(name, &mut sound_queue)
         -> SoundCommandQueue gets PlayTone/PlayNoise commands

JS side (each frame):
  -> drain_sound_commands() returns JSON array
    -> Web Audio API processes each command
```

### Test Cases

**Test 1: AutoJuice sound_queue feeds into SoundCommandQueue**
Create an `AutoJuiceSystem` with a rule that has `.with_sound("impact")`. Fire the rule (via an OnEvent trigger). Verify `auto_juice.sound_queue` contains `"impact"`. Then simulate the bridge: call `sound_palette.play("impact", &mut sound_queue)`. Verify `sound_queue.len() == 2` (impact has 2 commands: PlayTone + PlayNoise).

**Test 2: SoundPalette::merge combines profiles**
Create palette A with profile "a". Create palette B with profiles "b" and "c". Call `a.merge(&b)`. Verify palette A now has profiles "a", "b", "c". Verify `a.len() == 3`.

**Test 3: SoundPalette::merge overwrites duplicates**
Create palette A with profile "x" (1 command). Create palette B with profile "x" (3 commands). Call `a.merge(&b)`. Play "x" from A into a queue. Verify queue has 3 commands (B's version won).

**Test 4: trap_links_palette has all required profiles**
Call `SoundPalette::trap_links_palette()`. Verify it has all 12 profiles: `"shot_fire"`, `"bounce_wall"`, `"bounce_bumper"`, `"sand_drag"`, `"hole_in"`, `"fight_defeat"`, `"eagle"`, `"capture"`, `"level_up"`, `"step_grass"`, `"step_stone"`, `"trap_trigger"`.

**Test 5: Engine::play_sound convenience method**
(Integration test.) Create an Engine. Call `engine.play_sound("ui_click")`. Verify `engine.sound_queue.len() > 0`. Drain JSON and verify it contains a PlayTone command.

**Test 6: Unknown sound name does not panic**
Call `sound_palette.play("nonexistent_sfx", &mut queue)`. Verify it returns false and queue remains empty. Verify no panic.

**Test 7: Multiple sounds in one frame accumulate**
Fire two AutoJuice rules in one frame, each with a different sound. Verify `auto_juice.sound_queue` has 2 entries. Bridge them to sound_queue. Verify sound_queue has the combined command count.

**Test 8: AutoJuice sound_queue is cleared each apply()**
Call `auto_juice.apply()` once (fires a sound). Verify `sound_queue` has 1 entry. Call `apply()` again with no matching events. Verify `sound_queue` is empty (cleared at start of apply).

**Test 9: Each trap_links profile produces valid JSON**
For each profile in `trap_links_palette()`, play it into a queue, drain_json(), parse with serde_json. Verify each produces a valid JSON array with at least 1 command.

**Test 10: Merged palette retains default profiles**
Start with `default_palette()`. Merge `trap_links_palette()`. Verify original profiles still exist: `"impact"`, `"pickup"`, `"explosion"`, `"ui_click"`, `"ambient_wind"`, `"game_over"`. Verify total profile count is 6 (default) + 12 (trap_links) = 18.

### Estimated Lines of Code

| Component | Lines |
|-----------|-------|
| `SoundPalette::merge()` method | ~8 |
| `SoundPalette::trap_links_palette()` (12 profiles) | ~180 |
| `engine.rs` AutoJuice.apply() wiring | ~20 |
| `engine.rs` AutoJuice-to-Sound bridge | ~5 |
| `engine.rs` freeze frame integration | ~10 |
| `Engine::play_sound()` convenience method | ~5 |
| Tests (10 tests) | ~150 |
| **Total** | **~378** |

---

## Cross-Gap Integration Summary

### How the Three Gaps Compose Together

The three gaps form a vertical slice of the Trap Links interaction model:

1. **Player taps a button** (Gap 5: UiCanvas tap detection)
   - GestureRecognizer detects Tap -> EventBus `"gesture:tap"`
   - Engine hit-tests against UiCanvas buttons
   - Emits `"ui:button_tap"` with action string
   - Plays `"ui_click"` sound (Gap 8 bridge)

2. **Button triggers a dialogue choice** (Gap 3: Extended DialogueQueue)
   - For example, tapping "TALK" on an NPC opens a Conversation
   - Conversation renders choice buttons inline
   - Player taps a choice button
   - Choice emits a ConversationEvent -> EventBus
   - Game logic reacts (e.g., open shop, give item)

3. **Game event triggers a sound** (Gap 8: SoundScape cues)
   - Collision with a bumper fires AutoJuice rule
   - AutoJuice accumulates `"bounce_bumper"` in sound_queue
   - Bridge plays it through SoundPalette into SoundCommandQueue
   - JS audio driver receives PlayTone commands via drain_json()

### Dependency Order for Implementation

1. **Gap 5 first** -- UiCanvas tap detection is the foundation. Both Gap 3 (choice buttons) and Gap 8 (ui_click sound on button tap) depend on it.
2. **Gap 8 second** -- Sound bridge is standalone but enhances Gap 5 (button click sounds) and is needed before Gap 3 (choice selection sounds).
3. **Gap 3 last** -- Branching dialogue depends on both tap detection (for choice buttons) and sound (for selection feedback).

### Total Estimated Lines of Code

| Gap | Lines |
|-----|-------|
| Gap 3: Extended DialogueQueue | ~565 |
| Gap 5: UiCanvas Tap Detection | ~300 |
| Gap 8: Persistent SoundScape Cues | ~378 |
| **Grand Total** | **~1,243** |

### Files Modified

| File | Gaps |
|------|------|
| `engine/crates/engine-core/src/dialogue.rs` | Gap 3 |
| `engine/crates/engine-core/src/ui_canvas.rs` | Gap 5 |
| `engine/crates/engine-core/src/sound.rs` | Gap 8 |
| `engine/crates/engine-core/src/auto_juice.rs` | Gap 8 (no code changes, just wiring) |
| `engine/crates/engine-core/src/engine.rs` | Gaps 3, 5, 8 |

### New Public API Surface

```rust
// Gap 3
dialogue::NodeId                          // type alias u32
dialogue::DialogueChoice                  // struct
dialogue::DialogueNode                    // enum (Statement, Choice)
dialogue::ConversationState               // enum
dialogue::Conversation                    // struct
dialogue::ConversationEvent               // struct
dialogue::ConversationBuilder             // struct
DialogueQueue::start_conversation()       // method
DialogueQueue::has_active_conversation()  // method
DialogueQueue::select_choice()            // method
DialogueQueue::current_choices()          // method

// Gap 5
ui_canvas::TapResult                      // struct
UiWidget::disabled                        // field
UiWidget::pressed_timer                   // field
UiCanvas::press()                         // method
UiCanvas::tick()                          // method
// hit_test() return type changes from Option<String> to Option<TapResult>

// Gap 8
SoundPalette::merge()                     // method
SoundPalette::trap_links_palette()        // constructor
Engine::play_sound()                      // method
```
