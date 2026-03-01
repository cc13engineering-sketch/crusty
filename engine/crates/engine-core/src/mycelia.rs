/// GAME MODULE: Mycelia — Ascent
/// A fungal network grows upward through procedurally generated caves toward light.
/// Mobile-first: tap empty cave tiles near your network to grow.
/// Engine features: ProceduralGen, TileMap, DensityField, FlowNetwork, GraphNode,
/// VisualConnection, EnvironmentClock, PathFinding.

use crate::engine::Engine;
use crate::ecs::Entity;
use crate::tilemap::{TileMap, Tile, TileType};
use crate::rendering::color::Color;
use crate::rendering::shapes;
use crate::rendering::text;
use crate::procedural_gen::{SeededRng, Noise2D, cellular_automata};
use crate::components::graph_node::GraphNode;
use crate::components::Transform;
use crate::environment_clock::{EnvironmentClock, TimeCycle};

// ─── Constants ──────────────────────────────────────────────────────────

const TILE_SIZE: f64 = 16.0;
const CAVE_W: usize = 30;
const CAVE_H: usize = 45;
const WORLD_W: f64 = CAVE_W as f64 * TILE_SIZE; // 480
const WORLD_H: f64 = CAVE_H as f64 * TILE_SIZE; // 720

// Colors
const COL_CAVE_WALL: Color = Color { r: 52, g: 38, b: 28, a: 255 };
const COL_CAVE_WALL_LIGHT: Color = Color { r: 62, g: 48, b: 36, a: 255 };
const COL_BACKGROUND: Color = Color { r: 8, g: 8, b: 18, a: 255 };
const COL_NUTRIENT: Color = Color { r: 200, g: 144, b: 64, a: 255 };
const COL_NODE: Color = Color { r: 64, g: 216, b: 96, a: 255 };
const COL_ROOT: Color = Color { r: 224, g: 192, b: 64, a: 255 };
const COL_CONNECTION: Color = Color { r: 48, g: 160, b: 80, a: 180 };
const COL_BLIGHT: Color = Color { r: 180, g: 30, b: 30, a: 100 };
const COL_SURFACE: Color = Color { r: 120, g: 200, b: 255, a: 255 };

// Gameplay
const GROW_COST: f64 = 8.0;
const GROW_RANGE: f64 = 3.5; // max tiles away from existing node
const ENERGY_START: f64 = 30.0;
const ENERGY_MAX: f64 = 100.0;
const NUTRIENT_ENERGY: f64 = 15.0;
const BLIGHT_SPEED: f64 = 2.0; // pixels per second
const ROOT_PRODUCTION: f64 = 3.0; // energy per second
const NODE_RADIUS: f64 = 4.0;
const ROOT_RADIUS: f64 = 6.0;

// Nutrient tile marker
const NUTRIENT_TILE_ID: u16 = 1;

// ─── Game State (stored in Engine's global_state) ───────────────────────

// Keys for global_state
const K_ENERGY: &str = "energy";
const K_BLIGHT_Y: &str = "blight_y";
const K_SCORE: &str = "score";
const K_PHASE: &str = "phase"; // 0=playing, 1=won, 2=lost
const K_ROOT_ID: &str = "root_id";
const K_NODE_COUNT: &str = "node_count";
const K_HIGHEST_Y: &str = "highest_y";
const K_TIME: &str = "game_time";

// ─── Setup ──────────────────────────────────────────────────────────────

pub fn setup(engine: &mut Engine, seed: u64) {
    engine.debug_mode = false;
    engine.config.name = "Mycelia: Ascent".into();
    engine.config.bounds = (WORLD_W, WORLD_H);
    engine.config.background = COL_BACKGROUND;

    // Generate cave
    let mut tilemap = TileMap::new(CAVE_W, CAVE_H, TILE_SIZE);
    let mut rng = SeededRng::new(seed);

    // Cellular automata for organic cave walls
    cellular_automata(
        &mut tilemap, &mut rng,
        0.48,  // fill chance
        5,     // iterations
        5,     // birth threshold
        4,     // death threshold
        COL_CAVE_WALL,
    );

    // Add variation to wall colors using noise
    let noise = Noise2D::new(seed.wrapping_add(7));
    for y in 0..CAVE_H {
        for x in 0..CAVE_W {
            if tilemap.is_solid(x, y) {
                let n = noise.sample(x as f64 * 0.3, y as f64 * 0.3);
                if n > 0.55 {
                    tilemap.set(x, y, Tile::solid(COL_CAVE_WALL_LIGHT));
                }
            }
        }
    }

    // Ensure borders are solid walls
    for x in 0..CAVE_W {
        tilemap.set(x, 0, Tile::solid(COL_CAVE_WALL));
        tilemap.set(x, CAVE_H - 1, Tile::solid(COL_CAVE_WALL));
    }
    for y in 0..CAVE_H {
        tilemap.set(0, y, Tile::solid(COL_CAVE_WALL));
        tilemap.set(CAVE_W - 1, y, Tile::solid(COL_CAVE_WALL));
    }

    // Carve starting chamber at bottom
    let start_cx = CAVE_W / 2;
    let start_cy = CAVE_H - 5;
    for dy in -2i32..=2 {
        for dx in -3i32..=3 {
            let x = (start_cx as i32 + dx) as usize;
            let y = (start_cy as i32 + dy) as usize;
            if x > 0 && x < CAVE_W - 1 && y > 0 && y < CAVE_H - 1 {
                tilemap.set(x, y, Tile::empty());
            }
        }
    }

    // Carve surface opening at top
    for x in (CAVE_W / 2 - 3)..(CAVE_W / 2 + 4) {
        for y in 1..4 {
            tilemap.set(x, y, Tile::empty());
        }
    }

    // Place nutrient deposits in empty spaces (~12% of open tiles)
    for y in 2..CAVE_H - 2 {
        for x in 2..CAVE_W - 2 {
            if !tilemap.is_solid(x, y) && rng.chance(0.12) {
                tilemap.set(x, y, Tile::custom(NUTRIENT_TILE_ID, COL_NUTRIENT));
            }
        }
    }

    // Make surface tiles sky-colored
    for x in 0..CAVE_W {
        if !tilemap.is_solid(x, 0) {
            tilemap.set(x, 0, Tile::custom(2, COL_SURFACE));
        }
    }

    engine.tilemap = Some(tilemap);

    // Set up environment clock (pulse cycle)
    engine.environment_clock = EnvironmentClock::new();
    engine.environment_clock.add_cycle(TimeCycle::new(
        "pulse",
        vec![
            ("surge", 4.0),  // fast energy flow
            ("rest", 6.0),   // slow energy flow
        ],
    ));

    // Spawn root node entity
    let root = engine.world.spawn();
    let (root_wx, root_wy) = {
        let tm = engine.tilemap.as_ref().unwrap();
        tm.tile_to_world(start_cx, start_cy)
    };
    engine.world.transforms.insert(root, Transform {
        x: root_wx, y: root_wy, rotation: 0.0, scale: 1.0,
    });
    engine.world.tags.insert(root, crate::components::Tags::new(&["node", "root"]));
    engine.world.graph_nodes.insert(root, GraphNode::new());

    // Initialize game state
    engine.global_state.set_f64(K_ENERGY, ENERGY_START);
    engine.global_state.set_f64(K_BLIGHT_Y, WORLD_H + 40.0); // starts off-screen below
    engine.global_state.set_f64(K_SCORE, 0.0);
    engine.global_state.set_f64(K_PHASE, 0.0);
    engine.global_state.set_f64(K_ROOT_ID, root.0 as f64);
    engine.global_state.set_f64(K_NODE_COUNT, 1.0);
    engine.global_state.set_f64(K_HIGHEST_Y, root_wy);
    engine.global_state.set_f64(K_TIME, 0.0);

    // Camera: start looking at root
    engine.camera.x = root_wx - (engine.width as f64) / 2.0;
    engine.camera.y = root_wy - (engine.height as f64) * 0.7;
    engine.camera.smoothing = 0.3;
    engine.camera.zoom = 1.0;

    // Post-FX: subtle vignette
    engine.post_fx.vignette_strength = 0.5;
}

// ─── Per-frame Update ───────────────────────────────────────────────────

pub fn update(engine: &mut Engine, dt: f64) {
    let phase = engine.global_state.get_f64(K_PHASE).unwrap_or(0.0);
    if phase != 0.0 { return; } // game over

    let mut energy = engine.global_state.get_f64(K_ENERGY).unwrap_or(0.0);
    let mut blight_y = engine.global_state.get_f64(K_BLIGHT_Y).unwrap_or(WORLD_H);
    let game_time = engine.global_state.get_f64(K_TIME).unwrap_or(0.0) + dt;
    engine.global_state.set_f64(K_TIME, game_time);

    // Check pulse phase for energy production rate
    let is_surge = engine.environment_clock.phase("pulse")
        .map_or(false, |p| p == "surge");
    let production_mult = if is_surge { 2.0 } else { 0.8 };

    // Root produces energy
    energy = (energy + ROOT_PRODUCTION * production_mult * dt).min(ENERGY_MAX);

    // Blight rises (starts after 10 seconds, accelerates)
    if game_time > 10.0 {
        let acceleration = 1.0 + (game_time - 10.0) * 0.005;
        blight_y -= BLIGHT_SPEED * acceleration * dt;
    }

    // Check for nodes consumed by blight
    let mut dead_nodes: Vec<Entity> = Vec::new();
    for (entity, tags) in engine.world.tags.iter() {
        if tags.has("node") {
            if let Some(t) = engine.world.transforms.get(entity) {
                if t.y > blight_y {
                    dead_nodes.push(entity);
                }
            }
        }
    }

    for entity in &dead_nodes {
        let is_root = engine.world.tags.get(*entity)
            .map_or(false, |t| t.has("root"));
        if is_root {
            // Game over — blight reached root
            engine.global_state.set_f64(K_PHASE, 2.0);
            engine.screen_fx.push(
                crate::rendering::screen_fx::ScreenEffect::Flash {
                    color: Color::from_rgba(180, 30, 30, 255),
                    intensity: 0.8,
                },
                1.0,
            );
            engine.post_fx.shake_remaining = 0.5;
            engine.post_fx.shake_intensity = 8.0;
        }
        // Remove graph edges pointing to this node
        for (_, graph) in engine.world.graph_nodes.iter_mut() {
            graph.remove_edges_to(*entity);
        }
        engine.world.despawn(*entity);
    }

    // Check nutrients collected — nodes on nutrient tiles
    if let Some(ref mut tm) = engine.tilemap {
        for (entity, tags) in engine.world.tags.iter() {
            if tags.has("node") {
                if let Some(t) = engine.world.transforms.get(entity) {
                    if let Some((tx, ty)) = tm.world_to_tile(t.x, t.y) {
                        if let Some(tile) = tm.get(tx, ty) {
                            if tile.tile_type == TileType::Custom(NUTRIENT_TILE_ID) {
                                energy = (energy + NUTRIENT_ENERGY * dt * 0.5).min(ENERGY_MAX);
                                // Slowly consume the nutrient (fade it)
                                let faded = Color::from_rgba(
                                    (COL_NUTRIENT.r as f64 * 0.98) as u8,
                                    (COL_NUTRIENT.g as f64 * 0.98) as u8,
                                    (COL_NUTRIENT.b as f64 * 0.98) as u8,
                                    COL_NUTRIENT.a,
                                );
                                // After enough time the nutrient is consumed
                                let current_r = tile.color.r;
                                if current_r < 60 {
                                    tm.set(tx, ty, Tile::empty());
                                    let score = engine.global_state.get_f64(K_SCORE).unwrap_or(0.0);
                                    engine.global_state.set_f64(K_SCORE, score + 50.0);
                                } else {
                                    tm.set(tx, ty, Tile::custom(NUTRIENT_TILE_ID, faded));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Check win condition — any node reaches top 3 rows
    let highest_y = engine.global_state.get_f64(K_HIGHEST_Y).unwrap_or(WORLD_H);
    if highest_y < TILE_SIZE * 4.0 {
        engine.global_state.set_f64(K_PHASE, 1.0);
        engine.screen_fx.push(
            crate::rendering::screen_fx::ScreenEffect::Flash {
                color: Color::from_rgba(120, 255, 180, 255),
                intensity: 0.6,
            },
            1.5,
        );
    }

    // Update camera target: follow highest node, biased upward
    let camera_target_y = highest_y - (engine.height as f64) * 0.4;
    let camera_target_x = WORLD_W / 2.0 - (engine.width as f64) / 2.0;
    let smooth = (dt / engine.camera.smoothing.max(0.01)).min(1.0);
    engine.camera.x += (camera_target_x - engine.camera.x) * smooth;
    engine.camera.y += (camera_target_y - engine.camera.y) * smooth;

    // Clamp camera to world
    engine.camera.y = engine.camera.y
        .max(-(engine.height as f64) * 0.2)
        .min(WORLD_H - (engine.height as f64) * 0.5);

    engine.global_state.set_f64(K_ENERGY, energy);
    engine.global_state.set_f64(K_BLIGHT_Y, blight_y);
}

// ─── Tap Handler ────────────────────────────────────────────────────────

pub fn on_tap(engine: &mut Engine, screen_x: f64, screen_y: f64) -> bool {
    let phase = engine.global_state.get_f64(K_PHASE).unwrap_or(0.0);

    // Handle restart on game over
    if phase != 0.0 {
        return true; // Signal to JS to restart
    }

    let energy = engine.global_state.get_f64(K_ENERGY).unwrap_or(0.0);
    if energy < GROW_COST {
        return false;
    }

    // Convert screen position to world position
    let world_x = screen_x + engine.camera.x;
    let world_y = screen_y + engine.camera.y;

    // Check the tap is on an empty tile
    let (tile_x, tile_y) = match engine.tilemap.as_ref().and_then(|tm| tm.world_to_tile(world_x, world_y)) {
        Some(pos) => pos,
        None => return false,
    };

    // Must be empty or nutrient tile
    if let Some(tm) = &engine.tilemap {
        if tm.is_solid(tile_x, tile_y) {
            return false;
        }
    }

    // Find nearest existing node within growth range
    let (target_wx, target_wy) = engine.tilemap.as_ref().unwrap().tile_to_world(tile_x, tile_y);
    let max_dist = GROW_RANGE * TILE_SIZE;
    let max_dist_sq = max_dist * max_dist;

    let mut nearest_node: Option<(Entity, f64)> = None;
    for (entity, tags) in engine.world.tags.iter() {
        if tags.has("node") {
            if let Some(t) = engine.world.transforms.get(entity) {
                let dx = t.x - target_wx;
                let dy = t.y - target_wy;
                let dist_sq = dx * dx + dy * dy;
                if dist_sq < max_dist_sq && dist_sq > TILE_SIZE * 0.5 {
                    if nearest_node.is_none() || dist_sq < nearest_node.unwrap().1 {
                        nearest_node = Some((entity, dist_sq));
                    }
                }
            }
        }
    }

    let parent = match nearest_node {
        Some((e, _)) => e,
        None => return false, // no nearby node
    };

    // Check path isn't through solid tiles (simple line-of-sight)
    if !check_line_clear(engine, parent, target_wx, target_wy) {
        return false;
    }

    // Grow new node
    let new_node = engine.world.spawn();
    engine.world.transforms.insert(new_node, Transform {
        x: target_wx, y: target_wy, rotation: 0.0, scale: 1.0,
    });
    engine.world.tags.insert(new_node, crate::components::Tags::new(&["node"]));

    // Set up graph edge from parent to new node
    let mut graph = GraphNode::new();
    graph.add_edge(parent, "connection", 1.0, true);
    engine.world.graph_nodes.insert(new_node, graph);

    // Also add reverse edge on parent
    if let Some(parent_graph) = engine.world.graph_nodes.get_mut(parent) {
        parent_graph.add_edge(new_node, "connection", 1.0, true);
    }

    // Deduct energy
    let new_energy = energy - GROW_COST;
    engine.global_state.set_f64(K_ENERGY, new_energy);

    // Update score
    let score = engine.global_state.get_f64(K_SCORE).unwrap_or(0.0);
    engine.global_state.set_f64(K_SCORE, score + 10.0);

    // Update node count
    let count = engine.global_state.get_f64(K_NODE_COUNT).unwrap_or(0.0);
    engine.global_state.set_f64(K_NODE_COUNT, count + 1.0);

    // Update highest Y (lower Y = higher on screen)
    let highest = engine.global_state.get_f64(K_HIGHEST_Y).unwrap_or(WORLD_H);
    if target_wy < highest {
        engine.global_state.set_f64(K_HIGHEST_Y, target_wy);
    }

    // Growth feedback particle burst
    engine.particles.spawn_burst(
        target_wx, target_wy,
        6, 10.0, 30.0,
        0.4, 2.0, 0.5,
        COL_NODE,
        Color::from_rgba(40, 160, 80, 0),
        engine.frame,
    );

    false
}

/// Simple line-of-sight check: walk tiles from parent to target.
fn check_line_clear(engine: &Engine, parent: Entity, target_x: f64, target_y: f64) -> bool {
    let tm = match &engine.tilemap {
        Some(tm) => tm,
        None => return false,
    };

    let (px, py) = match engine.world.transforms.get(parent) {
        Some(t) => (t.x, t.y),
        None => return false,
    };

    // Step along line in small increments
    let dx = target_x - px;
    let dy = target_y - py;
    let dist = (dx * dx + dy * dy).sqrt();
    let steps = (dist / (TILE_SIZE * 0.5)).ceil() as usize;
    if steps == 0 { return true; }

    for i in 1..steps {
        let t = i as f64 / steps as f64;
        let wx = px + dx * t;
        let wy = py + dy * t;
        if tm.is_solid_at_world(wx, wy) {
            return false;
        }
    }

    true
}

// ─── Custom Rendering ───────────────────────────────────────────────────

pub fn render(engine: &mut Engine) {
    let sw = engine.width as f64;
    let sh = engine.height as f64;
    let cam_x = engine.camera.x;
    let cam_y = engine.camera.y;

    // Render tilemap
    if let Some(ref tilemap) = engine.tilemap {
        // Camera center for tilemap render
        let cam_cx = cam_x + sw / 2.0;
        let cam_cy = cam_y + sh / 2.0;
        tilemap.render(
            &mut engine.framebuffer,
            cam_cx, cam_cy,
            engine.camera.zoom,
            engine.width, engine.height,
        );
    }

    // Render connections between nodes (before nodes so lines are behind)
    let edges: Vec<(f64, f64, f64, f64)> = {
        let mut result = Vec::new();
        for (entity, graph) in engine.world.graph_nodes.iter() {
            if let Some(t1) = engine.world.transforms.get(entity) {
                for edge in &graph.edges {
                    // Only render each edge once (from lower entity ID)
                    if entity.0 < edge.target.0 {
                        if let Some(t2) = engine.world.transforms.get(edge.target) {
                            result.push((t1.x, t1.y, t2.x, t2.y));
                        }
                    }
                }
            }
        }
        result
    };

    let game_time = engine.global_state.get_f64(K_TIME).unwrap_or(0.0);
    for (x1, y1, x2, y2) in &edges {
        let sx1 = (x1 - cam_x) as f64;
        let sy1 = (y1 - cam_y) as f64;
        let sx2 = (x2 - cam_x) as f64;
        let sy2 = (y2 - cam_y) as f64;

        // Pulse the connection color
        let pulse = ((game_time * 3.0).sin() * 0.5 + 0.5) as f64;
        let base_g = COL_CONNECTION.g as f64;
        let g = (base_g + pulse * 40.0).min(255.0) as u8;
        let conn_color = Color::from_rgba(COL_CONNECTION.r, g, COL_CONNECTION.b, COL_CONNECTION.a);

        shapes::draw_line_thick(
            &mut engine.framebuffer,
            sx1, sy1, sx2, sy2,
            2.0, conn_color,
        );
    }

    // Render nodes
    for (entity, tags) in engine.world.tags.iter() {
        if tags.has("node") {
            if let Some(t) = engine.world.transforms.get(entity) {
                let sx = (t.x - cam_x) as f64;
                let sy = (t.y - cam_y) as f64;

                let is_root = tags.has("root");
                let (radius, color) = if is_root {
                    (ROOT_RADIUS, COL_ROOT)
                } else {
                    (NODE_RADIUS, COL_NODE)
                };

                // Subtle pulse
                let pulse = ((game_time * 2.0 + t.x * 0.1).sin() * 0.5 + 0.5) * 1.5;
                let r = radius + pulse;

                shapes::fill_circle(&mut engine.framebuffer, sx, sy, r, color);

                // Glow effect (larger, translucent circle)
                let glow = Color::from_rgba(color.r, color.g, color.b, 40);
                shapes::fill_circle(&mut engine.framebuffer, sx, sy, r + 3.0, glow);
            }
        }
    }

    // Render blight overlay
    let blight_y = engine.global_state.get_f64(K_BLIGHT_Y).unwrap_or(WORLD_H);
    let blight_screen_y = (blight_y - cam_y) as f64;
    if blight_screen_y < sh {
        let y_start = blight_screen_y.max(0.0);
        let blight_h = sh - y_start;
        if blight_h > 0.0 {
            shapes::fill_rect(
                &mut engine.framebuffer,
                0.0, y_start,
                sw, blight_h,
                COL_BLIGHT,
            );
            // Gradient edge at top of blight
            for i in 0..8 {
                let gy = y_start - i as f64;
                if gy >= 0.0 {
                    let alpha = ((8 - i) as f64 / 8.0 * COL_BLIGHT.a as f64) as u8;
                    shapes::fill_rect(
                        &mut engine.framebuffer,
                        0.0, gy, sw, 1.0,
                        Color::from_rgba(COL_BLIGHT.r, COL_BLIGHT.g, COL_BLIGHT.b, alpha / 2),
                    );
                }
            }
        }
    }

    // Render particles (managed by engine tick)
    engine.particles.render(&mut engine.framebuffer, &engine.camera);

    // Render HUD
    render_hud(engine);
}

fn render_hud(engine: &mut Engine) {
    let sw = engine.width as i32;
    let sh = engine.height as i32;
    let energy = engine.global_state.get_f64(K_ENERGY).unwrap_or(0.0);
    let score = engine.global_state.get_f64(K_SCORE).unwrap_or(0.0) as i64;
    let node_count = engine.global_state.get_f64(K_NODE_COUNT).unwrap_or(0.0) as i64;
    let highest_y = engine.global_state.get_f64(K_HIGHEST_Y).unwrap_or(WORLD_H);
    let phase = engine.global_state.get_f64(K_PHASE).unwrap_or(0.0);

    // Depth indicator (how far from surface)
    let depth_tiles = (highest_y / TILE_SIZE).ceil() as i64;
    let depth_text = format!("DEPTH {}", depth_tiles);
    text::draw_text(&mut engine.framebuffer, 8, 8, &depth_text, Color::from_rgba(150, 200, 255, 200), 1);

    // Score
    let score_text = format!("SCORE {}", score);
    let stw = text::text_width(&score_text, 1);
    text::draw_text(&mut engine.framebuffer, sw - stw - 8, 8, &score_text, Color::from_rgba(255, 220, 100, 200), 1);

    // Node count
    let nodes_text = format!("NODES {}", node_count);
    text::draw_text(&mut engine.framebuffer, 8, 22, &nodes_text, Color::from_rgba(100, 200, 130, 180), 1);

    // Energy bar
    let bar_x = 8;
    let bar_y = sh - 20;
    let bar_w = 120;
    let bar_h = 8;
    text::draw_text(&mut engine.framebuffer, bar_x, bar_y - 12, "ENERGY", Color::from_rgba(200, 200, 200, 180), 1);
    shapes::fill_rect(
        &mut engine.framebuffer,
        bar_x as f64, bar_y as f64,
        bar_w as f64, bar_h as f64,
        Color::from_rgba(30, 30, 30, 200),
    );
    let fill_w = ((energy / ENERGY_MAX) * bar_w as f64) as i32;
    if fill_w > 0 {
        let bar_color = if energy > GROW_COST {
            Color::from_rgba(64, 216, 96, 255)
        } else {
            Color::from_rgba(200, 80, 80, 255)
        };
        shapes::fill_rect(
            &mut engine.framebuffer,
            bar_x as f64, bar_y as f64,
            fill_w as f64, bar_h as f64,
            bar_color,
        );
    }

    // Pulse phase indicator
    let is_surge = engine.environment_clock.phase("pulse")
        .map_or(false, |p| p == "surge");
    let phase_text = if is_surge { "SURGE" } else { "REST" };
    let phase_color = if is_surge {
        Color::from_rgba(100, 255, 150, 200)
    } else {
        Color::from_rgba(150, 150, 180, 150)
    };
    text::draw_text(&mut engine.framebuffer, bar_x + bar_w + 8, bar_y - 4, phase_text, phase_color, 1);

    // Game over / win overlays
    if phase == 1.0 {
        // Won
        text::draw_text_centered(
            &mut engine.framebuffer,
            sw / 2, sh / 2 - 30,
            "SURFACE REACHED",
            Color::from_rgba(120, 255, 180, 255), 3,
        );
        text::draw_text_centered(
            &mut engine.framebuffer,
            sw / 2, sh / 2 + 10,
            &format!("SCORE: {}", score),
            Color::WHITE, 2,
        );
        if (engine.frame / 30) % 2 == 0 {
            text::draw_text_centered(
                &mut engine.framebuffer,
                sw / 2, sh / 2 + 40,
                "TAP TO PLAY AGAIN",
                Color::from_rgba(180, 180, 180, 200), 1,
            );
        }
    } else if phase == 2.0 {
        // Lost
        text::draw_text_centered(
            &mut engine.framebuffer,
            sw / 2, sh / 2 - 30,
            "NETWORK CONSUMED",
            Color::from_rgba(255, 80, 80, 255), 3,
        );
        text::draw_text_centered(
            &mut engine.framebuffer,
            sw / 2, sh / 2 + 10,
            &format!("SCORE: {}", score),
            Color::WHITE, 2,
        );
        if (engine.frame / 30) % 2 == 0 {
            text::draw_text_centered(
                &mut engine.framebuffer,
                sw / 2, sh / 2 + 40,
                "TAP TO PLAY AGAIN",
                Color::from_rgba(180, 180, 180, 200), 1,
            );
        }
    } else {
        // Playing — show tap hint at start
        let game_time = engine.global_state.get_f64(K_TIME).unwrap_or(0.0);
        if game_time < 5.0 {
            let alpha = ((5.0 - game_time) / 2.0).min(1.0).max(0.0);
            let hint_color = Color::from_rgba(200, 200, 200, (180.0 * alpha) as u8);
            text::draw_text_centered(
                &mut engine.framebuffer,
                sw / 2, sh / 2 - 60,
                "TAP EMPTY SPACE TO GROW",
                hint_color, 1,
            );
        }
    }
}

// ─── Get Game State as JSON ─────────────────────────────────────────────

pub fn get_state(engine: &Engine) -> String {
    let energy = engine.global_state.get_f64(K_ENERGY).unwrap_or(0.0);
    let score = engine.global_state.get_f64(K_SCORE).unwrap_or(0.0);
    let phase = engine.global_state.get_f64(K_PHASE).unwrap_or(0.0);
    let nodes = engine.global_state.get_f64(K_NODE_COUNT).unwrap_or(0.0);
    let highest = engine.global_state.get_f64(K_HIGHEST_Y).unwrap_or(0.0);

    format!(
        r#"{{"energy":{:.1},"score":{},"phase":{},"nodes":{},"depth":{:.0}}}"#,
        energy, score as i64, phase as i32, nodes as i64, highest / TILE_SIZE,
    )
}

// ─── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_engine() -> Engine {
        Engine::new(480, 720)
    }

    #[test]
    fn setup_creates_tilemap() {
        let mut engine = make_engine();
        setup(&mut engine, 42);
        assert!(engine.tilemap.is_some());
        let tm = engine.tilemap.as_ref().unwrap();
        assert_eq!(tm.width, CAVE_W);
        assert_eq!(tm.height, CAVE_H);
    }

    #[test]
    fn setup_creates_root_node() {
        let mut engine = make_engine();
        setup(&mut engine, 42);
        let root_id = engine.global_state.get_f64(K_ROOT_ID).unwrap() as u64;
        let root = Entity(root_id);
        assert!(engine.world.tags.get(root).unwrap().has("root"));
        assert!(engine.world.tags.get(root).unwrap().has("node"));
    }

    #[test]
    fn setup_initializes_game_state() {
        let mut engine = make_engine();
        setup(&mut engine, 42);
        assert_eq!(engine.global_state.get_f64(K_ENERGY).unwrap(), ENERGY_START);
        assert_eq!(engine.global_state.get_f64(K_PHASE).unwrap(), 0.0);
        assert_eq!(engine.global_state.get_f64(K_NODE_COUNT).unwrap(), 1.0);
    }

    #[test]
    fn setup_carves_starting_chamber() {
        let mut engine = make_engine();
        setup(&mut engine, 42);
        let tm = engine.tilemap.as_ref().unwrap();
        // Center of starting chamber should be empty
        assert!(!tm.is_solid(CAVE_W / 2, CAVE_H - 5));
    }

    #[test]
    fn setup_places_nutrients() {
        let mut engine = make_engine();
        setup(&mut engine, 42);
        let tm = engine.tilemap.as_ref().unwrap();
        let nutrients = tm.tiles.iter()
            .filter(|t| t.tile_type == TileType::Custom(NUTRIENT_TILE_ID))
            .count();
        assert!(nutrients > 0, "Should have nutrient tiles");
    }

    #[test]
    fn tap_near_root_grows_node() {
        let mut engine = make_engine();
        setup(&mut engine, 42);

        let root_id = engine.global_state.get_f64(K_ROOT_ID).unwrap() as u64;
        let root_pos = engine.world.transforms.get(Entity(root_id)).unwrap().clone();

        // Collect candidate positions first to avoid borrow conflicts
        let candidates: Vec<(f64, f64)> = {
            let tm = engine.tilemap.as_ref().unwrap();
            let (root_tx, root_ty) = tm.world_to_tile(root_pos.x, root_pos.y).unwrap();
            let mut v = Vec::new();
            for dy in -2i32..=2 {
                for dx in -2i32..=2 {
                    if dx == 0 && dy == 0 { continue; }
                    let tx = (root_tx as i32 + dx) as usize;
                    let ty = (root_ty as i32 + dy) as usize;
                    if !tm.is_solid(tx, ty) {
                        let (wx, wy) = tm.tile_to_world(tx, ty);
                        v.push((wx, wy));
                    }
                }
            }
            v
        };

        let mut grew = false;
        for (wx, wy) in candidates {
            let sx = wx - engine.camera.x;
            let sy = wy - engine.camera.y;
            on_tap(&mut engine, sx, sy);
            let count = engine.global_state.get_f64(K_NODE_COUNT).unwrap();
            if count > 1.0 {
                grew = true;
                break;
            }
        }
        assert!(grew, "Should have grown at least one node near root");
    }

    #[test]
    fn tap_on_solid_does_nothing() {
        let mut engine = make_engine();
        setup(&mut engine, 42);

        // Tap on a border wall (always solid)
        let (wx, wy) = engine.tilemap.as_ref().unwrap().tile_to_world(0, 0);
        let sx = wx - engine.camera.x;
        let sy = wy - engine.camera.y;
        on_tap(&mut engine, sx, sy);

        assert_eq!(engine.global_state.get_f64(K_NODE_COUNT).unwrap(), 1.0);
    }

    #[test]
    fn tap_too_far_does_nothing() {
        let mut engine = make_engine();
        setup(&mut engine, 42);

        // Tap far from root
        let sx = 10.0 - engine.camera.x;
        let sy = 10.0 - engine.camera.y;
        on_tap(&mut engine, sx, sy);

        assert_eq!(engine.global_state.get_f64(K_NODE_COUNT).unwrap(), 1.0);
    }

    #[test]
    fn grow_costs_energy() {
        let mut engine = make_engine();
        setup(&mut engine, 42);
        let initial_energy = engine.global_state.get_f64(K_ENERGY).unwrap();

        // Collect candidate positions first
        let root_id = engine.global_state.get_f64(K_ROOT_ID).unwrap() as u64;
        let root_pos = engine.world.transforms.get(Entity(root_id)).unwrap().clone();
        let candidates: Vec<(f64, f64)> = {
            let tm = engine.tilemap.as_ref().unwrap();
            let (root_tx, root_ty) = tm.world_to_tile(root_pos.x, root_pos.y).unwrap();
            let mut v = Vec::new();
            for dy in -2i32..=2 {
                for dx in -2i32..=2 {
                    if dx == 0 && dy == 0 { continue; }
                    let tx = (root_tx as i32 + dx) as usize;
                    let ty = (root_ty as i32 + dy) as usize;
                    if !tm.is_solid(tx, ty) {
                        let (wx, wy) = tm.tile_to_world(tx, ty);
                        v.push((wx, wy));
                    }
                }
            }
            v
        };

        for (wx, wy) in candidates {
            let sx = wx - engine.camera.x;
            let sy = wy - engine.camera.y;
            on_tap(&mut engine, sx, sy);
            let count = engine.global_state.get_f64(K_NODE_COUNT).unwrap();
            if count > 1.0 {
                let new_energy = engine.global_state.get_f64(K_ENERGY).unwrap();
                assert!((new_energy - (initial_energy - GROW_COST)).abs() < 0.01,
                    "Energy should decrease by GROW_COST");
                return;
            }
        }
    }

    #[test]
    fn update_advances_blight_after_delay() {
        let mut engine = make_engine();
        setup(&mut engine, 42);
        let initial_blight = engine.global_state.get_f64(K_BLIGHT_Y).unwrap();

        // Advance past blight delay
        for _ in 0..700 {
            update(&mut engine, 1.0 / 60.0);
        }

        let new_blight = engine.global_state.get_f64(K_BLIGHT_Y).unwrap();
        assert!(new_blight < initial_blight, "Blight should have risen after 10+ seconds");
    }

    #[test]
    fn update_produces_energy() {
        let mut engine = make_engine();
        setup(&mut engine, 42);
        engine.global_state.set_f64(K_ENERGY, 10.0);

        update(&mut engine, 1.0);

        let energy = engine.global_state.get_f64(K_ENERGY).unwrap();
        assert!(energy > 10.0, "Root should produce energy over time");
    }

    #[test]
    fn get_state_returns_json() {
        let mut engine = make_engine();
        setup(&mut engine, 42);
        let state = get_state(&engine);
        assert!(state.contains("\"energy\""));
        assert!(state.contains("\"score\""));
        assert!(state.contains("\"phase\""));
    }

    #[test]
    fn check_line_clear_returns_true_for_open() {
        let mut engine = make_engine();
        setup(&mut engine, 42);

        let root_id = engine.global_state.get_f64(K_ROOT_ID).unwrap() as u64;
        let root = Entity(root_id);
        let root_pos = engine.world.transforms.get(root).unwrap();

        // Check line to nearby point (should be clear in starting chamber)
        assert!(check_line_clear(&engine, root, root_pos.x + TILE_SIZE, root_pos.y));
    }

    #[test]
    fn render_does_not_panic() {
        let mut engine = make_engine();
        setup(&mut engine, 42);
        render(&mut engine);
    }
}
