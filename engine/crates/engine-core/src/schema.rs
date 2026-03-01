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
        ],
        "systems": [
            "lifecycle", "behavior",
            "force_accumulator", "integrator", "collision",
            "gameplay", "event_processor", "input_gameplay",
            "renderer", "debug_render"
        ],
        "engine_state": [
            "global_game_state", "timer_queue", "template_registry", "behavior_rules"
        ]
    });
    serde_json::to_string_pretty(&schema).unwrap()
}
