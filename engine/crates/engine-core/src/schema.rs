use crate::components::*;

pub fn generate_schema() -> String {
    let schema = serde_json::json!({
        "engine": "sling-rpg",
        "version": "0.2.0",
        "components": [
            { "name": Transform::schema_name(), "schema": Transform::schema() },
            { "name": RigidBody::schema_name(), "schema": RigidBody::schema() },
            { "name": Collider::schema_name(), "schema": Collider::schema() },
            { "name": Renderable::schema_name(), "schema": Renderable::schema() },
            { "name": ForceField::schema_name(), "schema": ForceField::schema() },
            { "name": Tags::schema_name(), "schema": Tags::schema() },
            { "name": Role::schema_name(), "schema": Role::schema() },
            { "name": Lifetime::schema_name(), "schema": Lifetime::schema() },
            { "name": GameState::schema_name(), "schema": GameState::schema() },
            { "name": Behavior::schema_name(), "schema": Behavior::schema() },
            { "name": PhysicsMaterial::schema_name(), "schema": PhysicsMaterial::schema() },
            { "name": Impulse::schema_name(), "schema": Impulse::schema() },
            { "name": MotionConstraint::schema_name(), "schema": MotionConstraint::schema() },
            { "name": ZoneEffect::schema_name(), "schema": ZoneEffect::schema() },
            { "name": PropertyTween::schema_name(), "schema": PropertyTween::schema() },
            { "name": EntityFlash::schema_name(), "schema": EntityFlash::schema() },
            { "name": GhostTrail::schema_name(), "schema": GhostTrail::schema() },
            { "name": TimeScale::schema_name(), "schema": TimeScale::schema() },
            { "name": Active::schema_name(), "schema": Active::schema() },
            { "name": WaypointPath::schema_name(), "schema": WaypointPath::schema() },
            { "name": SignalEmitter::schema_name(), "schema": SignalEmitter::schema() },
            { "name": SignalReceiver::schema_name(), "schema": SignalReceiver::schema() },
            { "name": Parent::schema_name(), "schema": Parent::schema() },
            { "name": Children::schema_name(), "schema": Children::schema() },
            { "name": WorldTransform::schema_name(), "schema": WorldTransform::schema() },
            { "name": StateMachine::schema_name(), "schema": StateMachine::schema() },
            { "name": Coroutine::schema_name(), "schema": Coroutine::schema() },
        ],
        "systems": [
            "lifecycle", "hierarchy", "signal", "state_machine", "coroutine",
            "behavior", "tween", "flash", "ghost_trail", "waypoint",
            "force_accumulator", "integrator", "collision",
            "gameplay", "event_processor", "input_gameplay",
            "renderer", "debug_render"
        ],
        "engine_state": [
            "global_game_state", "timer_queue", "template_registry", "behavior_rules",
            "scene_manager", "screen_fx_stack", "tilemap", "spatial_query", "entity_pool"
        ]
    });
    serde_json::to_string_pretty(&schema).unwrap()
}
