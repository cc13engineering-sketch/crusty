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
                scripting::loader::load_world_file(&world_file, &mut eng.world, &mut eng.config);
                log::log(&format!("Loaded world '{}' with {} entities",
                    eng.config.name, eng.world.entity_count()));
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
