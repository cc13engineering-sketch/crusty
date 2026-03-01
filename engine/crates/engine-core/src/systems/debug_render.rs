/// SYSTEM: debug_render
/// READS: Transform, RigidBody, ForceField, Collider
/// WRITES: Framebuffer (overlays)
/// ORDER: runs last, only when debug_mode is true

use crate::ecs::World;
use crate::rendering::framebuffer::Framebuffer;
use crate::rendering::shapes;
use crate::rendering::color::Color;
use crate::engine::Camera;
use crate::components::{ColliderShape, FieldType};

pub fn run(world: &World, fb: &mut Framebuffer, camera: &Camera) {
    // Velocity vectors
    for (entity, rb) in world.rigidbodies.iter() {
        if rb.is_static { continue; }
        if let Some(t) = world.transforms.get(entity) {
            let (sx, sy) = camera.world_to_screen(t.x, t.y);
            let vscale = 0.1;
            let ex = sx as f64 + rb.vx * vscale;
            let ey = sy as f64 + rb.vy * vscale;
            let color = Color::from_rgba(255, 255, 0, 128);
            shapes::draw_line(fb, sx as f64, sy as f64, ex, ey, color);
        }
    }

    // Force field radii
    for (entity, ff) in world.force_fields.iter() {
        if let Some(t) = world.transforms.get(entity) {
            let (sx, sy) = camera.world_to_screen(t.x, t.y);
            let color = match &ff.field_type {
                FieldType::Attract => Color::from_rgba(0, 200, 255, 77),
                FieldType::Repel => Color::from_rgba(255, 80, 80, 77),
                FieldType::Vortex => Color::from_rgba(200, 80, 200, 77),
                FieldType::Directional { .. } => Color::from_rgba(200, 200, 200, 77),
            };
            shapes::draw_dashed_circle(fb, sx as f64, sy as f64, ff.radius, color, 8.0);
        }
    }

    // Collider wireframes
    for (entity, col) in world.colliders.iter() {
        if let Some(t) = world.transforms.get(entity) {
            let (sx, sy) = camera.world_to_screen(t.x, t.y);
            let color = Color::from_rgba(0, 255, 0, 77);
            match &col.shape {
                ColliderShape::Circle { radius } => {
                    shapes::draw_circle(fb, sx as f64, sy as f64, *radius, color);
                }
                ColliderShape::Rect { half_width, half_height } => {
                    shapes::draw_rect(
                        fb,
                        sx as f64 - half_width,
                        sy as f64 - half_height,
                        half_width * 2.0,
                        half_height * 2.0,
                        color,
                    );
                }
            }
        }
    }
}
