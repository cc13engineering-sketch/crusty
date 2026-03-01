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
    pub physics_material: Option<PhysicsMaterial>,
    pub impulse: Option<Impulse>,
    pub motion_constraint: Option<MotionConstraint>,
    pub zone_effect: Option<ZoneEffect>,
    pub property_tween: Option<PropertyTween>,
    pub entity_flash: Option<EntityFlash>,
    pub ghost_trail: Option<GhostTrail>,
    pub time_scale: Option<TimeScale>,
    pub active: Option<Active>,
    pub waypoint_path: Option<WaypointPath>,
    pub signal_emitter: Option<SignalEmitter>,
    pub signal_receiver: Option<SignalReceiver>,
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
            physics_material: None,
            impulse: None,
            motion_constraint: None,
            zone_effect: None,
            property_tween: None,
            entity_flash: None,
            ghost_trail: None,
            time_scale: None,
            active: None,
            waypoint_path: None,
            signal_emitter: None,
            signal_receiver: None,
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
