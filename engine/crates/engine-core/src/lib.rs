use wasm_bindgen::prelude::*;

mod ecs;
mod components;
mod systems;
mod rendering;
mod physics;
pub mod scripting;
mod events;
mod input;
pub mod log;
pub mod engine;
pub mod schema;
mod spawn_queue;
pub mod game_state;
pub mod timers;
pub mod templates;
pub mod behavior;
pub mod dialogue;
pub mod scene_manager;
pub mod tilemap;
pub mod raycast;
pub mod spatial_query;
pub mod entity_pool;
pub mod pathfinding;
pub mod event_bus;
pub mod input_map;
pub mod save_load;

#[cfg(test)]
mod tests;

use engine::Engine;

thread_local! {
    static ENGINE: std::cell::RefCell<Option<Engine>> = std::cell::RefCell::new(None);
}

fn with_engine<F, R>(f: F) -> R
where F: FnOnce(&mut Engine) -> R {
    ENGINE.with(|e| {
        let mut borrow = e.borrow_mut();
        f(borrow.as_mut().expect("Engine not initialized"))
    })
}

#[wasm_bindgen]
pub fn init(width: u32, height: u32) {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    ENGINE.with(|e| {
        *e.borrow_mut() = Some(Engine::new(width, height));
    });
}

#[wasm_bindgen]
pub fn tick(dt_ms: f64) {
    with_engine(|eng| eng.tick(dt_ms / 1000.0));
}

#[wasm_bindgen]
pub fn framebuffer_ptr() -> usize {
    with_engine(|eng| eng.framebuffer.ptr())
}

#[wasm_bindgen]
pub fn framebuffer_len() -> usize {
    with_engine(|eng| eng.framebuffer.len())
}

#[wasm_bindgen]
pub fn key_down(code: String) {
    with_engine(|eng| eng.input.on_key_down(code));
}

#[wasm_bindgen]
pub fn key_up(code: String) {
    with_engine(|eng| eng.input.on_key_up(code));
}

#[wasm_bindgen]
pub fn mouse_move(x: f64, y: f64) {
    with_engine(|eng| eng.input.on_mouse_move(x, y));
}

#[wasm_bindgen]
pub fn mouse_down(x: f64, y: f64, button: u32) {
    with_engine(|eng| eng.input.on_mouse_down(x, y, button));
}

#[wasm_bindgen]
pub fn mouse_up(x: f64, y: f64, button: u32) {
    with_engine(|eng| eng.input.on_mouse_up(x, y, button));
}

#[wasm_bindgen]
pub fn load_world(source: String) {
    with_engine(|eng| {
        match scripting::parser::parse_world(&source) {
            Ok(world_file) => {
                // Reset everything first (clears spawner timers, particles, etc.)
                eng.reset_game_state();

                // Full load: entities + templates + state + timers + rules
                // (load_world_full also clears and repopulates these from the .world file)
                scripting::loader::load_world_full(
                    &world_file, &mut eng.world, &mut eng.config,
                    &mut eng.global_state, &mut eng.timers,
                    &mut eng.templates, &mut eng.rules,
                );
                log::log(&format!(
                    "Loaded world '{}' with {} entities, {} templates, {} timers, {} rules",
                    eng.config.name, eng.world.entity_count(),
                    eng.templates.len(), eng.timers.len(), eng.rules.len(),
                ));

                // Initialize starfield for space-themed games
                let name_lower = eng.config.name.to_lowercase();
                if name_lower.contains("space") || name_lower.contains("asteroid")
                    || name_lower.contains("star") || name_lower.contains("cosmic")
                {
                    let (bw, bh) = eng.config.bounds;
                    eng.starfield = Some(rendering::starfield::Starfield::generate(
                        42, bw, bh, 200,
                    ));
                    eng.post_fx.vignette_strength = 0.6;
                }
            }
            Err(e) => {
                log::error(&format!("World parse error: {}", e));
            }
        }
    });
}

#[wasm_bindgen]
pub fn get_schema() -> String {
    schema::generate_schema()
}

// ─── Game State WASM API ─────────────────────────────────────────────

/// Get the entire global game state as a JSON string.
#[wasm_bindgen]
pub fn get_game_state() -> String {
    with_engine(|eng| eng.global_state.to_json())
}

/// Set a numeric game state value.
#[wasm_bindgen]
pub fn set_game_state_f64(key: String, value: f64) {
    with_engine(|eng| eng.global_state.set_f64(&key, value));
}

/// Get a numeric game state value (returns 0 if not found).
#[wasm_bindgen]
pub fn get_game_state_f64(key: String) -> f64 {
    with_engine(|eng| eng.global_state.get_f64(&key).unwrap_or(0.0))
}

// ─── Runtime Spawn WASM API ──────────────────────────────────────────

/// Spawn an entity from a named template at the given position.
/// Returns the entity ID, or 0 if the template was not found.
#[wasm_bindgen]
pub fn spawn_template(name: String, x: f64, y: f64) -> u64 {
    with_engine(|eng| {
        if let Some(entity) = eng.templates.spawn(&name, &mut eng.world, Some((x, y))) {
            entity.0
        } else {
            log::warn(&format!("spawn_template: template '{}' not found", name));
            0
        }
    })
}

// ─── Timer WASM API ──────────────────────────────────────────────────

/// Start a one-shot timer. When it fires, behavior rules can react to it.
#[wasm_bindgen]
pub fn start_timer(name: String, delay: f64) {
    with_engine(|eng| {
        eng.timers.add(timers::Timer::one_shot(&name, delay));
    });
}

/// Start a repeating timer.
#[wasm_bindgen]
pub fn start_repeating_timer(name: String, delay: f64, interval: f64) {
    with_engine(|eng| {
        eng.timers.add(timers::Timer::repeating(&name, delay, interval));
    });
}

/// Cancel a timer by name.
#[wasm_bindgen]
pub fn cancel_timer(name: String) {
    with_engine(|eng| { eng.timers.cancel(&name); });
}
