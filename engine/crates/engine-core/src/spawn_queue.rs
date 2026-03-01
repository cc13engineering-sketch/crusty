use crate::ecs::Entity;
use crate::components::*;

/// A deferred command to spawn a new entity at runtime.
#[derive(Clone, Debug)]
pub struct SpawnCommand {
    pub name: Option<String>,
    pub transform: Transform,
    pub rigidbody: Option<RigidBody>,
    pub collider: Option<Collider>,
    pub renderable: Option<Renderable>,
    pub tags: Option<Tags>,
    pub lifetime: Option<Lifetime>,
    pub game_state: Option<GameState>,
    pub behavior: Option<Behavior>,
}

impl SpawnCommand {
    /// Quick helper: create a minimal spawn at a position.
    pub fn at(x: f64, y: f64) -> Self {
        Self {
            name: None,
            transform: Transform { x, y, ..Default::default() },
            rigidbody: None,
            collider: None,
            renderable: None,
            tags: None,
            lifetime: None,
            game_state: None,
            behavior: None,
        }
    }
}

/// Queue of spawn and despawn commands processed once per frame.
#[derive(Default)]
pub struct SpawnQueue {
    pub spawns: Vec<SpawnCommand>,
    pub despawns: Vec<Entity>,
}

impl SpawnQueue {
    pub fn spawn(&mut self, cmd: SpawnCommand) {
        self.spawns.push(cmd);
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.despawns.push(entity);
    }

    pub fn clear(&mut self) {
        self.spawns.clear();
        self.despawns.clear();
    }
}
