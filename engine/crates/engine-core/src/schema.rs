use crate::components::*;

pub fn generate_schema() -> String {
    let schema = serde_json::json!({
        "engine": "sling-rpg",
        "version": "0.1.0",
        "components": [
            { "name": Transform::schema_name(), "schema": Transform::schema() },
            { "name": RigidBody::schema_name(), "schema": RigidBody::schema() },
            { "name": Collider::schema_name(), "schema": Collider::schema() },
            { "name": Renderable::schema_name(), "schema": Renderable::schema() },
            { "name": ForceField::schema_name(), "schema": ForceField::schema() },
            { "name": Tags::schema_name(), "schema": Tags::schema() },
            { "name": Role::schema_name(), "schema": Role::schema() },
        ],
        "systems": [
            "force_accumulator", "integrator", "collision",
            "event_processor", "input_gameplay", "renderer", "debug_render"
        ]
    });
    serde_json::to_string_pretty(&schema).unwrap()
}
