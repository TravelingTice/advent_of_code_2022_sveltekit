use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_message(message: String) -> String {
    message
}
