/// Input action mapping: abstracts raw key/mouse inputs into named game actions.
///
/// Instead of checking `input.keys_held.contains("Space")`, systems check
/// `input_map.is_action_pressed("jump")`. Bindings are configurable at runtime.

use std::collections::HashMap;

/// A physical input source that can be bound to an action.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InputSource {
    /// A keyboard key code (e.g., "Space", "ArrowUp", "KeyW").
    Key(String),
    /// A mouse button (0=left, 1=middle, 2=right).
    MouseButton(u32),
}

/// An action binding: one action can have multiple input sources.
#[derive(Clone, Debug)]
pub struct ActionBinding {
    pub sources: Vec<InputSource>,
}

impl ActionBinding {
    pub fn new() -> Self {
        Self { sources: Vec::new() }
    }

    pub fn key(mut self, key: &str) -> Self {
        self.sources.push(InputSource::Key(key.to_string()));
        self
    }

    pub fn mouse_button(mut self, button: u32) -> Self {
        self.sources.push(InputSource::MouseButton(button));
        self
    }
}

/// Axis binding: maps two actions (positive/negative) to a -1..+1 range.
#[derive(Clone, Debug)]
pub struct AxisBinding {
    pub positive_sources: Vec<InputSource>,
    pub negative_sources: Vec<InputSource>,
}

impl AxisBinding {
    pub fn new() -> Self {
        Self {
            positive_sources: Vec::new(),
            negative_sources: Vec::new(),
        }
    }

    pub fn positive_key(mut self, key: &str) -> Self {
        self.positive_sources.push(InputSource::Key(key.to_string()));
        self
    }

    pub fn negative_key(mut self, key: &str) -> Self {
        self.negative_sources.push(InputSource::Key(key.to_string()));
        self
    }
}

/// The input map: maps named actions and axes to physical inputs.
#[derive(Clone, Debug, Default)]
pub struct InputMap {
    actions: HashMap<String, ActionBinding>,
    axes: HashMap<String, AxisBinding>,
}

impl InputMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a named action with its bindings.
    pub fn bind_action(&mut self, name: &str, binding: ActionBinding) {
        self.actions.insert(name.to_string(), binding);
    }

    /// Register a named axis with its bindings.
    pub fn bind_axis(&mut self, name: &str, binding: AxisBinding) {
        self.axes.insert(name.to_string(), binding);
    }

    /// Unbind an action.
    pub fn unbind_action(&mut self, name: &str) {
        self.actions.remove(name);
    }

    /// Unbind an axis.
    pub fn unbind_axis(&mut self, name: &str) {
        self.axes.remove(name);
    }

    /// Check if an action is currently held (any bound source is active).
    pub fn is_action_held(&self, name: &str, input: &crate::input::Input) -> bool {
        if let Some(binding) = self.actions.get(name) {
            binding.sources.iter().any(|source| match source {
                InputSource::Key(key) => input.keys_held.contains(key.as_str()),
                InputSource::MouseButton(btn) => input.mouse_buttons_held.contains(btn),
            })
        } else {
            false
        }
    }

    /// Check if an action was just pressed this frame.
    pub fn is_action_pressed(&self, name: &str, input: &crate::input::Input) -> bool {
        if let Some(binding) = self.actions.get(name) {
            binding.sources.iter().any(|source| match source {
                InputSource::Key(key) => input.keys_pressed.contains(key.as_str()),
                InputSource::MouseButton(btn) => input.mouse_buttons_pressed.contains(btn),
            })
        } else {
            false
        }
    }

    /// Check if an action was just released this frame.
    pub fn is_action_released(&self, name: &str, input: &crate::input::Input) -> bool {
        if let Some(binding) = self.actions.get(name) {
            binding.sources.iter().any(|source| match source {
                InputSource::Key(key) => input.keys_released.contains(key.as_str()),
                InputSource::MouseButton(_btn) => false, // Mouse release tracking not yet in Input
            })
        } else {
            false
        }
    }

    /// Get the value of a named axis (-1.0 to 1.0).
    /// Returns 0.0 if neither positive nor negative sources are active.
    pub fn axis_value(&self, name: &str, input: &crate::input::Input) -> f64 {
        if let Some(binding) = self.axes.get(name) {
            let pos = binding.positive_sources.iter().any(|s| match s {
                InputSource::Key(key) => input.keys_held.contains(key.as_str()),
                InputSource::MouseButton(btn) => input.mouse_buttons_held.contains(btn),
            });
            let neg = binding.negative_sources.iter().any(|s| match s {
                InputSource::Key(key) => input.keys_held.contains(key.as_str()),
                InputSource::MouseButton(btn) => input.mouse_buttons_held.contains(btn),
            });
            match (pos, neg) {
                (true, false) => 1.0,
                (false, true) => -1.0,
                _ => 0.0, // both or neither
            }
        } else {
            0.0
        }
    }

    /// List all bound action names.
    pub fn action_names(&self) -> Vec<&str> {
        self.actions.keys().map(|s| s.as_str()).collect()
    }

    /// List all bound axis names.
    pub fn axis_names(&self) -> Vec<&str> {
        self.axes.keys().map(|s| s.as_str()).collect()
    }

    /// Get the bindings for an action.
    pub fn get_action(&self, name: &str) -> Option<&ActionBinding> {
        self.actions.get(name)
    }

    /// Create a default WASD + arrows + space game input map.
    pub fn default_game_map() -> Self {
        let mut map = Self::new();
        map.bind_action("jump", ActionBinding::new().key("Space").key("ArrowUp").key("KeyW"));
        map.bind_action("fire", ActionBinding::new().mouse_button(0).key("KeyX"));
        map.bind_action("interact", ActionBinding::new().key("KeyE").key("Enter"));
        map.bind_action("pause", ActionBinding::new().key("Escape").key("KeyP"));

        map.bind_axis("horizontal", AxisBinding::new()
            .positive_key("ArrowRight").positive_key("KeyD")
            .negative_key("ArrowLeft").negative_key("KeyA"));
        map.bind_axis("vertical", AxisBinding::new()
            .positive_key("ArrowDown").positive_key("KeyS")
            .negative_key("ArrowUp").negative_key("KeyW"));
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::Input;

    fn input_with_key_held(key: &str) -> Input {
        let mut input = Input::new();
        input.on_key_down(key.to_string());
        input
    }

    fn input_with_key_pressed(key: &str) -> Input {
        let mut input = Input::new();
        input.on_key_down(key.to_string());
        input
    }

    #[test]
    fn new_map_is_empty() {
        let map = InputMap::new();
        assert!(map.action_names().is_empty());
        assert!(map.axis_names().is_empty());
    }

    #[test]
    fn bind_and_check_action() {
        let mut map = InputMap::new();
        map.bind_action("jump", ActionBinding::new().key("Space"));

        let input = input_with_key_held("Space");
        assert!(map.is_action_held("jump", &input));
    }

    #[test]
    fn action_not_held_when_wrong_key() {
        let mut map = InputMap::new();
        map.bind_action("jump", ActionBinding::new().key("Space"));

        let input = input_with_key_held("Enter");
        assert!(!map.is_action_held("jump", &input));
    }

    #[test]
    fn multiple_bindings_for_action() {
        let mut map = InputMap::new();
        map.bind_action("jump", ActionBinding::new().key("Space").key("ArrowUp"));

        let input = input_with_key_held("ArrowUp");
        assert!(map.is_action_held("jump", &input));
    }

    #[test]
    fn action_pressed_this_frame() {
        let mut map = InputMap::new();
        map.bind_action("fire", ActionBinding::new().key("KeyX"));

        let input = input_with_key_pressed("KeyX");
        assert!(map.is_action_pressed("fire", &input));
    }

    #[test]
    fn unbound_action_returns_false() {
        let map = InputMap::new();
        let input = input_with_key_held("Space");
        assert!(!map.is_action_held("nonexistent", &input));
        assert!(!map.is_action_pressed("nonexistent", &input));
    }

    #[test]
    fn axis_positive() {
        let mut map = InputMap::new();
        map.bind_axis("horizontal", AxisBinding::new()
            .positive_key("ArrowRight")
            .negative_key("ArrowLeft"));

        let input = input_with_key_held("ArrowRight");
        assert_eq!(map.axis_value("horizontal", &input), 1.0);
    }

    #[test]
    fn axis_negative() {
        let mut map = InputMap::new();
        map.bind_axis("horizontal", AxisBinding::new()
            .positive_key("ArrowRight")
            .negative_key("ArrowLeft"));

        let input = input_with_key_held("ArrowLeft");
        assert_eq!(map.axis_value("horizontal", &input), -1.0);
    }

    #[test]
    fn axis_both_cancel_to_zero() {
        let mut map = InputMap::new();
        map.bind_axis("horizontal", AxisBinding::new()
            .positive_key("ArrowRight")
            .negative_key("ArrowLeft"));

        let mut input = Input::new();
        input.on_key_down("ArrowRight".to_string());
        input.on_key_down("ArrowLeft".to_string());
        assert_eq!(map.axis_value("horizontal", &input), 0.0);
    }

    #[test]
    fn axis_neither_is_zero() {
        let mut map = InputMap::new();
        map.bind_axis("horizontal", AxisBinding::new()
            .positive_key("ArrowRight")
            .negative_key("ArrowLeft"));

        let input = Input::new();
        assert_eq!(map.axis_value("horizontal", &input), 0.0);
    }

    #[test]
    fn unbound_axis_returns_zero() {
        let map = InputMap::new();
        let input = Input::new();
        assert_eq!(map.axis_value("nonexistent", &input), 0.0);
    }

    #[test]
    fn unbind_action() {
        let mut map = InputMap::new();
        map.bind_action("jump", ActionBinding::new().key("Space"));
        map.unbind_action("jump");

        let input = input_with_key_held("Space");
        assert!(!map.is_action_held("jump", &input));
    }

    #[test]
    fn default_game_map_has_standard_bindings() {
        let map = InputMap::default_game_map();
        assert!(map.get_action("jump").is_some());
        assert!(map.get_action("fire").is_some());
        assert!(map.get_action("interact").is_some());
        assert!(map.get_action("pause").is_some());
        assert!(map.axis_names().contains(&"horizontal"));
        assert!(map.axis_names().contains(&"vertical"));
    }

    #[test]
    fn default_map_jump_works_with_space() {
        let map = InputMap::default_game_map();
        let input = input_with_key_held("Space");
        assert!(map.is_action_held("jump", &input));
    }

    #[test]
    fn action_names_lists_all() {
        let mut map = InputMap::new();
        map.bind_action("a", ActionBinding::new().key("KeyA"));
        map.bind_action("b", ActionBinding::new().key("KeyB"));
        let names = map.action_names();
        assert_eq!(names.len(), 2);
    }

    #[test]
    fn mouse_button_binding() {
        let mut map = InputMap::new();
        map.bind_action("fire", ActionBinding::new().mouse_button(0));

        let mut input = Input::new();
        input.on_mouse_down(100.0, 100.0, 0);
        assert!(map.is_action_held("fire", &input));
        assert!(map.is_action_pressed("fire", &input));
    }
}
