use crate::{devtools::Devtools, runtime::Runtime};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Chrome;

    #[wasm_bindgen(method, getter)]
    pub fn runtime(this: &Chrome) -> Runtime;

    #[wasm_bindgen(method, getter)]
    pub fn devtools(this: &Chrome) -> Devtools;
}
