use std::collections::HashMap;

/// Entry in the scene stack.
#[derive(Clone, Debug)]
pub struct SceneStackEntry {
    pub name: String,
}

/// Named scene registry with push/pop stack semantics.
/// Scenes are stored as .world file source strings.
#[derive(Clone, Debug)]
pub struct SceneManager {
    registry: HashMap<String, String>,
    stack: Vec<SceneStackEntry>,
}

impl SceneManager {
    pub fn new() -> Self {
        Self { registry: HashMap::new(), stack: Vec::new() }
    }

    /// Register a scene by name with its .world source.
    pub fn register(&mut self, name: &str, source: &str) {
        self.registry.insert(name.to_string(), source.to_string());
    }

    /// Remove a scene from the registry.
    pub fn unregister(&mut self, name: &str) -> bool {
        self.registry.remove(name).is_some()
    }

    /// Check if a scene is registered.
    pub fn has(&self, name: &str) -> bool {
        self.registry.contains_key(name)
    }

    /// Get the source for a named scene.
    pub fn get_source(&self, name: &str) -> Option<&str> {
        self.registry.get(name).map(|s| s.as_str())
    }

    /// Push a scene onto the stack. Returns false if scene not registered.
    pub fn push(&mut self, name: &str) -> bool {
        if self.registry.contains_key(name) {
            self.stack.push(SceneStackEntry { name: name.to_string() });
            true
        } else {
            false
        }
    }

    /// Pop the top scene. Returns the name of the popped scene.
    pub fn pop(&mut self) -> Option<String> {
        self.stack.pop().map(|e| e.name)
    }

    /// Get the current (top) scene name.
    pub fn current(&self) -> Option<&str> {
        self.stack.last().map(|e| e.name.as_str())
    }

    /// Get the current scene's source.
    pub fn current_source(&self) -> Option<&str> {
        self.current().and_then(|name| self.get_source(name))
    }

    /// Get the stack depth.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// Replace the current scene with a new one.
    pub fn replace(&mut self, name: &str) -> bool {
        if !self.registry.contains_key(name) {
            return false;
        }
        self.stack.pop();
        self.stack.push(SceneStackEntry { name: name.to_string() });
        true
    }

    /// Clear the entire stack.
    pub fn clear_stack(&mut self) {
        self.stack.clear();
    }

    /// Clear everything (registry + stack).
    pub fn clear(&mut self) {
        self.registry.clear();
        self.stack.clear();
    }

    /// List all registered scene names.
    pub fn scene_names(&self) -> Vec<&str> {
        self.registry.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_empty() {
        let sm = SceneManager::new();
        assert_eq!(sm.depth(), 0);
        assert!(sm.current().is_none());
    }

    #[test]
    fn register_and_has() {
        let mut sm = SceneManager::new();
        sm.register("level1", "world \"Level 1\" {}");
        assert!(sm.has("level1"));
        assert!(!sm.has("level2"));
    }

    #[test]
    fn get_source() {
        let mut sm = SceneManager::new();
        sm.register("test", "world \"Test\" {}");
        assert_eq!(sm.get_source("test"), Some("world \"Test\" {}"));
        assert_eq!(sm.get_source("missing"), None);
    }

    #[test]
    fn unregister() {
        let mut sm = SceneManager::new();
        sm.register("level1", "...");
        assert!(sm.unregister("level1"));
        assert!(!sm.has("level1"));
        assert!(!sm.unregister("level1"));
    }

    #[test]
    fn push_registered_scene() {
        let mut sm = SceneManager::new();
        sm.register("main", "...");
        assert!(sm.push("main"));
        assert_eq!(sm.depth(), 1);
        assert_eq!(sm.current(), Some("main"));
    }

    #[test]
    fn push_unregistered_fails() {
        let mut sm = SceneManager::new();
        assert!(!sm.push("nonexistent"));
        assert_eq!(sm.depth(), 0);
    }

    #[test]
    fn pop_returns_scene_name() {
        let mut sm = SceneManager::new();
        sm.register("a", "...");
        sm.push("a");
        assert_eq!(sm.pop(), Some("a".to_string()));
        assert_eq!(sm.depth(), 0);
    }

    #[test]
    fn pop_empty_returns_none() {
        let mut sm = SceneManager::new();
        assert_eq!(sm.pop(), None);
    }

    #[test]
    fn push_pop_stack_semantics() {
        let mut sm = SceneManager::new();
        sm.register("a", "...");
        sm.register("b", "...");
        sm.push("a");
        sm.push("b");
        assert_eq!(sm.current(), Some("b"));
        assert_eq!(sm.depth(), 2);
        sm.pop();
        assert_eq!(sm.current(), Some("a"));
        assert_eq!(sm.depth(), 1);
    }

    #[test]
    fn current_source_returns_world_data() {
        let mut sm = SceneManager::new();
        sm.register("level1", "world \"Level 1\" { bounds: 800 x 600 }");
        sm.push("level1");
        assert_eq!(sm.current_source(), Some("world \"Level 1\" { bounds: 800 x 600 }"));
    }

    #[test]
    fn replace_scene() {
        let mut sm = SceneManager::new();
        sm.register("a", "...");
        sm.register("b", "...");
        sm.push("a");
        assert!(sm.replace("b"));
        assert_eq!(sm.current(), Some("b"));
        assert_eq!(sm.depth(), 1);
    }

    #[test]
    fn replace_unregistered_fails() {
        let mut sm = SceneManager::new();
        sm.register("a", "...");
        sm.push("a");
        assert!(!sm.replace("missing"));
        assert_eq!(sm.current(), Some("a"));
    }

    #[test]
    fn clear_stack() {
        let mut sm = SceneManager::new();
        sm.register("a", "...");
        sm.register("b", "...");
        sm.push("a");
        sm.push("b");
        sm.clear_stack();
        assert_eq!(sm.depth(), 0);
        assert!(sm.has("a")); // registry unaffected
    }

    #[test]
    fn clear_everything() {
        let mut sm = SceneManager::new();
        sm.register("a", "...");
        sm.push("a");
        sm.clear();
        assert_eq!(sm.depth(), 0);
        assert!(!sm.has("a"));
    }

    #[test]
    fn scene_names_lists_all() {
        let mut sm = SceneManager::new();
        sm.register("alpha", "...");
        sm.register("beta", "...");
        let mut names = sm.scene_names();
        names.sort();
        assert_eq!(names, vec!["alpha", "beta"]);
    }

    #[test]
    fn clone_and_debug() {
        let mut sm = SceneManager::new();
        sm.register("x", "...");
        sm.push("x");
        let cloned = sm.clone();
        let debug = format!("{:?}", cloned);
        assert!(debug.contains("SceneManager"));
    }
}
