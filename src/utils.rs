use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    /// Output a `&str` to the browser console
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(msg: &str);
}

/// Set `console_error_panic_hook` once. This allows to get better error
/// messages in the browser console if the WASM program panics.
/// More info: [https://github.com/rustwasm/console_error_panic_hook#readme]()
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    
    console_error_panic_hook::set_once();
}
