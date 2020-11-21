
pub mod automata;
pub mod board;
pub mod colour;
pub mod mutation;
pub mod population;
pub mod tournament;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn bonjour() {
    log("bonjour I am a teapot");
}
