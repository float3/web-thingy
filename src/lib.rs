mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, web-thingy!");
}

#[wasm_bindgen]
pub struct CursorPosition {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl CursorPosition {
    pub fn new() -> CursorPosition {
        CursorPosition { x: 0.0, y: 0.0 }
    }

    pub fn update_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}
