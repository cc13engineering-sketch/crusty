/// SYSTEM: renderer
/// READS: Transform, Renderable, Collider (for auto-visuals)
/// WRITES: Framebuffer
/// ORDER: runs once per frame during rendering phase

use crate::ecs::World;
use crate::rendering::framebuffer::Framebuffer;
use crate::rendering::shapes;
use crate::rendering::sprite::SpriteSheet;
use crate::rendering::color::Color;
use crate::engine::{WorldConfig, Camera};
use crate::input::Input;
use crate::components::{Visual, ColliderShape};

/// Render entities only (no framebuffer clear). Called by engine after background clear.
pub fn run_entities_only(
    world: &World,
    fb: &mut Framebuffer,
    input: &Input,
    camera: &Camera,
    sprite_sheets: &[SpriteSheet],
) {
    render_drawables(world, fb, input, camera, sprite_sheets);
}

#[allow(dead_code)]
pub fn run(
    world: &World,
    fb: &mut Framebuffer,
    config: &WorldConfig,
    input: &Input,
    camera: &Camera,
    sprite_sheets: &[SpriteSheet],
) {
    fb.clear(config.background);
    render_drawables(world, fb, input, camera, sprite_sheets);
}

fn render_drawables(
    world: &World,
    fb: &mut Framebuffer,
    input: &Input,
    camera: &Camera,
    sprite_sheets: &[SpriteSheet],
) {
    // Collect drawable entities
    let mut drawables: Vec<(i32, f64, f64, DrawType)> = Vec::new();

    // Entities with Renderable
    for (entity, rend) in world.renderables.iter() {
        if !rend.visible { continue; }
        if let Some(t) = world.transforms.get(entity) {
            let (sx, sy) = camera.world_to_screen(t.x, t.y);
            drawables.push((rend.layer, sx as f64, sy as f64, DrawType::Visual(rend.visual.clone())));
        }
    }

    // Auto-visuals for entities with Collider but no Renderable
    for (entity, col) in world.colliders.iter() {
        if world.renderables.has(entity) { continue; }
        if let Some(t) = world.transforms.get(entity) {
            let (sx, sy) = camera.world_to_screen(t.x, t.y);
            let color = Color::from_rgba(0, 255, 0, 60);
            match &col.shape {
                ColliderShape::Circle { radius } => {
                    drawables.push((-1000, sx as f64, sy as f64,
                        DrawType::Visual(Visual::Circle { radius: *radius, color, filled: false })));
                }
                ColliderShape::Rect { half_width, half_height } => {
                    drawables.push((-1000, sx as f64, sy as f64,
                        DrawType::Visual(Visual::Rect {
                            width: half_width * 2.0, height: half_height * 2.0,
                            color, filled: false,
                        })));
                }
            }
        }
    }

    drawables.sort_by_key(|(layer, _, _, _)| *layer);

    for (_, x, y, draw_type) in &drawables {
        match draw_type {
            DrawType::Visual(vis) => match vis {
                Visual::Circle { radius, color, filled } => {
                    if *filled {
                        shapes::fill_circle(fb, *x, *y, *radius, *color);
                    } else {
                        shapes::draw_circle(fb, *x, *y, *radius, *color);
                    }
                }
                Visual::Rect { width, height, color, filled } => {
                    let rx = x - width / 2.0;
                    let ry = y - height / 2.0;
                    if *filled {
                        shapes::fill_rect(fb, rx, ry, *width, *height, *color);
                    } else {
                        shapes::draw_rect(fb, rx, ry, *width, *height, *color);
                    }
                }
                Visual::Line { x2, y2, color, thickness } => {
                    let (sx2, sy2) = camera.world_to_screen(*x2, *y2);
                    shapes::draw_line_thick(fb, *x, *y, sx2 as f64, sy2 as f64, *thickness, *color);
                }
                Visual::Sprite { sheet_id, tile_index } => {
                    if let Some(sheet) = sprite_sheets.get(*sheet_id as usize) {
                        // Center the tile on the entity position
                        let dx = *x as i32 - (sheet.tile_w as i32 / 2);
                        let dy = *y as i32 - (sheet.tile_h as i32 / 2);
                        sheet.draw_tile(fb, *tile_index, dx, dy);
                    }
                }
            }
        }
    }

    // Aim line while dragging
    if input.is_dragging {
        if let Some((start_x, start_y)) = input.drag_start {
            // Find player position
            for (entity, tag) in world.tags.iter() {
                if tag.has(crate::components::Tag::Player) {
                    if let Some(t) = world.transforms.get(entity) {
                        let (px, py) = camera.world_to_screen(t.x, t.y);
                        let dx = start_x - input.mouse_x;
                        let dy = start_y - input.mouse_y;
                        let aim_color = Color::from_rgba(255, 255, 255, 100);
                        shapes::draw_line(fb, px as f64, py as f64, px as f64 + dx, py as f64 + dy, aim_color);
                        break;
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
enum DrawType {
    Visual(Visual),
}
