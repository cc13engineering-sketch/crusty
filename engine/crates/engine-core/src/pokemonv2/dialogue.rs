// AI-INSTRUCTIONS: pokemonv2/dialogue.rs — Standalone dialogue for simple NPC/sign text.
// Leaf module — no sibling imports. Used for non-scripted interactions.

/// What happens when dialogue finishes.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DialogueAction {
    Resume, // return to overworld
}

/// Standalone NPC/sign dialogue state.
#[derive(Clone, Debug)]
pub struct DialogueState {
    pub lines: Vec<String>,
    pub current_line: usize,
    pub char_index: usize,
    pub timer: f64,
    pub on_complete: DialogueAction,
}

impl DialogueState {
    const CHARS_PER_SECOND: f64 = 40.0;
    const MAX_LINE_LEN: usize = 26;

    pub fn new(text: &str) -> Self {
        let lines = wrap_text(text, Self::MAX_LINE_LEN);
        Self {
            lines,
            current_line: 0,
            char_index: 0,
            timer: 0.0,
            on_complete: DialogueAction::Resume,
        }
    }

    /// Advance one frame. Returns true while still active.
    pub fn step(&mut self, confirm_pressed: bool) -> bool {
        if self.lines.is_empty() {
            return false;
        }

        let current_len = self.lines.get(self.current_line).map_or(0, |l| l.len());
        let fully_shown = self.char_index >= current_len;

        if fully_shown && confirm_pressed {
            // Advance to next page or finish
            if self.current_line + 1 < self.lines.len() {
                self.current_line += 1;
                self.char_index = 0;
                self.timer = 0.0;
            } else {
                return false;
            }
        } else if !fully_shown {
            // Advance typewriter
            self.timer += 1.0 / 60.0;
            let chars_shown = (self.timer * Self::CHARS_PER_SECOND) as usize;
            self.char_index = chars_shown.min(current_len);

            // Confirm skips typewriter
            if confirm_pressed {
                self.char_index = current_len;
            }
        }

        true
    }

    /// Current visible text.
    pub fn visible_text(&self) -> &str {
        self.lines.get(self.current_line)
            .map(|l| &l[..self.char_index.min(l.len())])
            .unwrap_or("")
    }
}

fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for raw_line in text.split('\n') {
        let words: Vec<&str> = raw_line.split_whitespace().collect();
        let mut current = String::new();
        for word in words {
            if current.is_empty() {
                current.push_str(word);
            } else if current.len() + 1 + word.len() <= max_chars {
                current.push(' ');
                current.push_str(word);
            } else {
                lines.push(current.clone());
                current = word.to_string();
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialogue_creation() {
        let d = DialogueState::new("Hello, world!");
        assert!(!d.lines.is_empty());
    }

    #[test]
    fn test_dialogue_step_advances() {
        let mut d = DialogueState::new("Hi!");
        // Initially not done
        let active = d.step(false);
        assert!(active);
        // Fast-forward to end of line
        d.char_index = d.lines[0].len();
        // Confirm dismisses
        let active = d.step(true);
        assert!(!active);
    }

    #[test]
    fn test_wrap_text() {
        let lines = super::wrap_text("This is a very long line that should be wrapped", 20);
        for line in &lines {
            assert!(line.len() <= 20, "Line too long: '{}'", line);
        }
    }
}
