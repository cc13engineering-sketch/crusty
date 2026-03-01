/// Save/Load system: captures and restores world state as JSON snapshots.
///
/// Captures all entity component data, global game state, and camera position.
/// Uses serde_json for serialization without requiring Serialize derives on
/// components (manual snapshot/restore for each component type).

use std::collections::HashMap;
use crate::ecs::Entity;

/// A snapshot of a single entity's components.
#[derive(Clone, Debug)]
pub struct EntitySnapshot {
    pub id: u64,
    pub name: Option<String>,
    pub components: HashMap<String, serde_json::Value>,
}

/// A snapshot of the entire world state.
#[derive(Clone, Debug)]
pub struct WorldSnapshot {
    pub entities: Vec<EntitySnapshot>,
    pub global_state: HashMap<String, f64>,
    pub global_strings: HashMap<String, String>,
    pub camera: (f64, f64, f64), // x, y, zoom
    pub timestamp: f64,
}

impl WorldSnapshot {
    /// Capture the current world state into a snapshot.
    pub fn capture(
        world: &crate::ecs::World,
        global_state: &crate::game_state::GameState,
        camera_x: f64,
        camera_y: f64,
        camera_zoom: f64,
        time: f64,
    ) -> Self {
        let mut entities = Vec::new();

        for entity in world.alive.iter() {
            let mut components = HashMap::new();

            // Transform
            if let Some(t) = world.transforms.get(*entity) {
                components.insert("Transform".to_string(), serde_json::json!({
                    "x": t.x, "y": t.y, "rotation": t.rotation, "scale": t.scale
                }));
            }

            // RigidBody
            if let Some(rb) = world.rigidbodies.get(*entity) {
                components.insert("RigidBody".to_string(), serde_json::json!({
                    "vx": rb.vx, "vy": rb.vy, "ax": rb.ax, "ay": rb.ay,
                    "mass": rb.mass, "restitution": rb.restitution,
                    "is_static": rb.is_static,
                    "damping": rb.damping
                }));
            }

            // GameState
            if let Some(gs) = world.game_states.get(*entity) {
                let values: HashMap<String, f64> = gs.values.iter()
                    .map(|(k, v)| (k.clone(), *v))
                    .collect();
                if !values.is_empty() {
                    components.insert("GameState".to_string(), serde_json::json!(values));
                }
            }

            // Tags
            if let Some(tags) = world.tags.get(*entity) {
                let tag_list: Vec<&str> = tags.values.iter().map(|s| s.as_str()).collect();
                if !tag_list.is_empty() {
                    components.insert("Tags".to_string(), serde_json::json!(tag_list));
                }
            }

            // Role
            if let Some(role) = world.roles.get(*entity) {
                components.insert("Role".to_string(), serde_json::json!({
                    "name": role.name, "intent": role.intent, "group": role.group
                }));
            }

            // StateMachine
            if let Some(sm) = world.state_machines.get(*entity) {
                components.insert("StateMachine".to_string(), serde_json::json!({
                    "current_state": sm.current_state,
                    "state_elapsed": sm.state_elapsed
                }));
            }

            let snap = EntitySnapshot {
                id: entity.0,
                name: world.names.get_name(*entity).map(|s| s.to_string()),
                components,
            };
            entities.push(snap);
        }

        // Global state — extract f64 and string entries from typed StateValue iter
        let mut global_f64 = HashMap::new();
        let mut global_str = HashMap::new();
        for (key, val) in global_state.iter() {
            match val {
                crate::game_state::StateValue::F64(v) => { global_f64.insert(key.to_string(), *v); }
                crate::game_state::StateValue::Str(s) => { global_str.insert(key.to_string(), s.clone()); }
                crate::game_state::StateValue::Bool(b) => { global_f64.insert(key.to_string(), if *b { 1.0 } else { 0.0 }); }
            }
        }

        WorldSnapshot {
            entities,
            global_state: global_f64,
            global_strings: global_str,
            camera: (camera_x, camera_y, camera_zoom),
            timestamp: time,
        }
    }

    /// Serialize the snapshot to a JSON string.
    pub fn to_json(&self) -> String {
        let mut entity_values = Vec::new();
        for entity in &self.entities {
            entity_values.push(serde_json::json!({
                "id": entity.id,
                "name": entity.name,
                "components": entity.components,
            }));
        }

        let json = serde_json::json!({
            "entities": entity_values,
            "global_state": self.global_state,
            "global_strings": self.global_strings,
            "camera": {
                "x": self.camera.0,
                "y": self.camera.1,
                "zoom": self.camera.2,
            },
            "timestamp": self.timestamp,
        });

        serde_json::to_string_pretty(&json).unwrap_or_default()
    }

    /// Restore transform data from a snapshot into a world.
    /// Entities must already exist in the world — this restores component values.
    pub fn restore_transforms(&self, world: &mut crate::ecs::World) {
        for entity_snap in &self.entities {
            let entity = Entity(entity_snap.id);
            if !world.is_alive(entity) {
                continue;
            }

            if let Some(t_val) = entity_snap.components.get("Transform") {
                if let Some(t) = world.transforms.get_mut(entity) {
                    if let Some(x) = t_val.get("x").and_then(|v| v.as_f64()) { t.x = x; }
                    if let Some(y) = t_val.get("y").and_then(|v| v.as_f64()) { t.y = y; }
                    if let Some(r) = t_val.get("rotation").and_then(|v| v.as_f64()) { t.rotation = r; }
                    if let Some(s) = t_val.get("scale").and_then(|v| v.as_f64()) { t.scale = s; }
                }
            }
        }
    }

    /// Restore game state values from snapshot.
    pub fn restore_game_states(&self, world: &mut crate::ecs::World) {
        for entity_snap in &self.entities {
            let entity = Entity(entity_snap.id);
            if !world.is_alive(entity) {
                continue;
            }

            if let Some(gs_val) = entity_snap.components.get("GameState") {
                if let Some(obj) = gs_val.as_object() {
                    if let Some(gs) = world.game_states.get_mut(entity) {
                        for (key, val) in obj {
                            if let Some(f) = val.as_f64() {
                                gs.set(key, f);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Restore global game state from snapshot.
    pub fn restore_global_state(&self, global_state: &mut crate::game_state::GameState) {
        global_state.clear();
        for (key, value) in &self.global_state {
            global_state.set_f64(key, *value);
        }
        for (key, value) in &self.global_strings {
            global_state.set_str(key, value);
        }
    }

    /// Number of entities captured.
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Total number of component entries captured.
    pub fn component_count(&self) -> usize {
        self.entities.iter().map(|e| e.components.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::World;
    use crate::components::*;
    use crate::game_state::GameState as GlobalGameState;

    #[test]
    fn capture_empty_world() {
        let world = World::new();
        let gs = GlobalGameState::new();
        let snap = WorldSnapshot::capture(&world, &gs, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(snap.entity_count(), 0);
        assert_eq!(snap.component_count(), 0);
    }

    #[test]
    fn capture_entity_with_transform() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 10.0, y: 20.0, rotation: 0.5, scale: 2.0 });
        let gs = GlobalGameState::new();

        let snap = WorldSnapshot::capture(&world, &gs, 0.0, 0.0, 1.0, 42.0);
        assert_eq!(snap.entity_count(), 1);
        assert_eq!(snap.timestamp, 42.0);
        let entity_snap = &snap.entities[0];
        assert!(entity_snap.components.contains_key("Transform"));
    }

    #[test]
    fn capture_entity_with_game_state() {
        let mut world = World::new();
        let e = world.spawn();
        let mut entity_gs = crate::components::game_state::GameState::new();
        entity_gs.set("health", 100.0);
        entity_gs.set("mana", 50.0);
        world.game_states.insert(e, entity_gs);
        let gs = GlobalGameState::new();

        let snap = WorldSnapshot::capture(&world, &gs, 0.0, 0.0, 1.0, 0.0);
        let entity_snap = &snap.entities[0];
        assert!(entity_snap.components.contains_key("GameState"));
    }

    #[test]
    fn capture_global_state() {
        let world = World::new();
        let mut gs = GlobalGameState::new();
        gs.set_f64("score", 999.0);
        gs.set_str("level", "dungeon");

        let snap = WorldSnapshot::capture(&world, &gs, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(*snap.global_state.get("score").unwrap(), 999.0);
        assert_eq!(snap.global_strings.get("level").unwrap(), "dungeon");
    }

    #[test]
    fn capture_camera_state() {
        let world = World::new();
        let gs = GlobalGameState::new();
        let snap = WorldSnapshot::capture(&world, &gs, 100.0, 200.0, 2.5, 0.0);
        assert_eq!(snap.camera, (100.0, 200.0, 2.5));
    }

    #[test]
    fn to_json_is_valid() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 5.0, y: 10.0, rotation: 0.0, scale: 1.0 });
        let gs = GlobalGameState::new();

        let snap = WorldSnapshot::capture(&world, &gs, 0.0, 0.0, 1.0, 0.0);
        let json = snap.to_json();
        assert!(!json.is_empty());
        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.get("entities").is_some());
        assert!(parsed.get("camera").is_some());
    }

    #[test]
    fn restore_transforms() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform { x: 10.0, y: 20.0, rotation: 0.0, scale: 1.0 });
        let gs = GlobalGameState::new();

        let snap = WorldSnapshot::capture(&world, &gs, 0.0, 0.0, 1.0, 0.0);

        // Modify transforms
        world.transforms.get_mut(e).unwrap().x = 999.0;

        // Restore
        snap.restore_transforms(&mut world);
        assert_eq!(world.transforms.get(e).unwrap().x, 10.0);
    }

    #[test]
    fn restore_game_states() {
        let mut world = World::new();
        let e = world.spawn();
        let mut entity_gs = crate::components::game_state::GameState::new();
        entity_gs.set("health", 100.0);
        world.game_states.insert(e, entity_gs);
        let gs = GlobalGameState::new();

        let snap = WorldSnapshot::capture(&world, &gs, 0.0, 0.0, 1.0, 0.0);

        // Modify
        world.game_states.get_mut(e).unwrap().set("health", 0.0);

        // Restore
        snap.restore_game_states(&mut world);
        assert_eq!(world.game_states.get(e).unwrap().get("health"), 100.0);
    }

    #[test]
    fn restore_global_state() {
        let world = World::new();
        let mut gs = GlobalGameState::new();
        gs.set_f64("score", 500.0);
        gs.set_str("level", "forest");

        let snap = WorldSnapshot::capture(&world, &gs, 0.0, 0.0, 1.0, 0.0);

        // Modify
        gs.set_f64("score", 0.0);
        gs.set_str("level", "");

        // Restore
        snap.restore_global_state(&mut gs);
        assert_eq!(gs.get_f64("score"), Some(500.0));
        assert_eq!(gs.get_str("level"), Some("forest"));
    }

    #[test]
    fn named_entity_captured() {
        let mut world = World::new();
        let e = world.spawn_named("hero");
        world.transforms.insert(e, Transform::default());
        let gs = GlobalGameState::new();

        let snap = WorldSnapshot::capture(&world, &gs, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(snap.entities[0].name.as_deref(), Some("hero"));
    }

    #[test]
    fn component_count_accurate() {
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform::default());
        world.tags.insert(e, Tags::new(&["player"]));
        let gs = GlobalGameState::new();

        let snap = WorldSnapshot::capture(&world, &gs, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(snap.component_count(), 2);
    }
}
