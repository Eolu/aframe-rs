//! Lower level FFI stuff. Mostly used internally, but exposed in case the abstractions
//! of this library are too restrictive.
//! Using this should not be necessary for the usage of this crate, but the 
//! public APIs have been provided while this crate is still feature-incomplete.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern 
{
    /// [registering-a-primitive](https://aframe.io/docs/1.2.0/introduction/html-and-primitives.html#registering-a-primitive)
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerPrimitive(name: &str, definition: JsValue);
    /// [register-component-name-definition](https://aframe.io/docs/1.2.0/core/component.html#aframe-registercomponent-name-definition)
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerComponent(name: &str, data: JsValue);
    /// [register-a-custom-shader-material](https://aframe.io/docs/1.2.0/components/material.html#register-a-custom-shader-material)
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerShader(name: &str, data: JsValue);
}