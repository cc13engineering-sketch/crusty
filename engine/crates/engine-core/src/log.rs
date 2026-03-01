/// Platform-agnostic logging. USE THIS everywhere instead of direct web_sys::console calls.

pub fn log(msg: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(msg));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("{}", msg);
    }
}

pub fn warn(msg: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::warn_1(&wasm_bindgen::JsValue::from_str(msg));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        eprintln!("WARN: {}", msg);
    }
}

pub fn error(msg: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(msg));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        eprintln!("ERROR: {}", msg);
    }
}
