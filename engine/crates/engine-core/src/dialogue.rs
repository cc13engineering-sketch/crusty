use crate::rendering::color::Color;

/// The visual style / placement category of a message.
#[derive(Clone, Debug)]
pub enum MessageKind {
    /// Full dialogue box at the bottom of the screen.
    Dialogue,
    /// Short toast notification at the top-center.
    Notification,
    /// Text that floats at a world position and drifts upward.
    FloatingText,
}

/// A single in-game message (dialogue line, notification, or floating text).
#[derive(Clone, Debug)]
pub struct Message {
    pub kind: MessageKind,
    pub text: String,
    /// World-space position for FloatingText. Ignored for Dialogue / Notification.
    pub world_pos: Option<(f64, f64)>,
    /// Total lifetime in seconds.
    pub duration: f64,
    /// Seconds remaining before this message expires.
    pub remaining: f64,
    /// Text color.
    pub color: Color,
}

/// A queue of active in-game messages. Messages are ticked each frame and
/// automatically removed when their remaining time runs out.
#[derive(Clone, Debug)]
pub struct DialogueQueue {
    pub messages: Vec<Message>,
}

impl DialogueQueue {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    /// Push a new message onto the queue.
    pub fn push(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    /// Advance all message timers by `dt` and remove expired ones.
    /// FloatingText messages also drift upward over time.
    pub fn tick(&mut self, dt: f64) {
        for msg in &mut self.messages {
            msg.remaining -= dt;

            // FloatingText drifts upward
            if let MessageKind::FloatingText = msg.kind {
                if let Some((ref mut _wx, ref mut wy)) = msg.world_pos {
                    *wy -= 20.0 * dt; // 20 pixels/sec upward drift
                }
            }
        }
        self.messages.retain(|m| m.remaining > 0.0);
    }

    /// Iterate over currently active (non-expired) messages.
    pub fn active(&self) -> impl Iterator<Item = &Message> {
        self.messages.iter().filter(|m| m.remaining > 0.0)
    }

    /// Remove all messages.
    pub fn clear(&mut self) {
        self.messages.clear();
    }
}
