use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start(element_id: String) {
    println!("{}", element_id);
}
