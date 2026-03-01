pub mod transform;
pub mod rigidbody;
pub mod collider;
pub mod renderable;
pub mod force_field;
pub mod tags;
pub mod role;
pub mod lifetime;
pub mod game_state;
pub mod behavior;

pub use transform::Transform;
pub use rigidbody::RigidBody;
pub use collider::{Collider, ColliderShape};
pub use renderable::{Renderable, Visual};
pub use force_field::{ForceField, FieldType, Falloff};
pub use tags::Tags;
pub use role::Role;
pub use lifetime::Lifetime;
pub use game_state::GameState;
pub use behavior::{Behavior, BehaviorMode};

/// Trait for components to provide their own schema metadata.
pub trait SchemaInfo {
    fn schema_name() -> &'static str;
    fn schema() -> serde_json::Value;
}
