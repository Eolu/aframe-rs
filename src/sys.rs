//! Lower level FFI stuff. Mostly used internally, but exposed in case the abstractions
//! of this library are too restrictive.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern 
{
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerComponent(name: &str, data: JsValue);
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerShader(name: &str, data: JsValue);
}