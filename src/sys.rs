//! Lower level FFI stuff. Mostly used internally, but exposed in case the abstractions
//! of this library are too restrictive.
//! Using this should not be necessary for the usage of this crate, but the 
//! public APIs have been provided while this crate is still feature-incomplete.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern 
{
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerPrimitive(name: &str, definition: JsValue);
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerComponent(name: &str, data: JsValue);
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerShader(name: &str, data: JsValue);
}