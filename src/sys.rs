//! Lower level FFI stuff. Mostly used internally, but exposed in case the abstractions
//! of this library are too restrictive.
//! Using this should not be necessary for the usage of this crate, but the 
//! public APIs have been provided while this crate is still feature-incomplete.

use wasm_bindgen::{JsCast, prelude::*};
use std::convert::TryFrom;
use js_sys::{Array, Object};
use once_cell::sync::Lazy;

static AFRAME: Lazy<Option<Aframe>> = Lazy::new(Aframe::get);

#[wasm_bindgen]
extern 
{
    /// [registering-a-primitive](https://aframe.io/docs/1.2.0/introduction/html-and-primitives.html#registering-a-primitive)
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerPrimitive(name: &str, definition: JsValue);

    /// [register-component-name-definition](https://aframe.io/docs/1.2.0/core/component.html#aframe-registercomponent-name-definition)
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerComponent(name: &str, data: JsValue);

    /// [registering-a-system](https://aframe.io/docs/1.2.0/core/systems.html#registering-a-system)
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerSystem(name: &str, data: JsValue);

    /// [register-a-custom-shader-material](https://aframe.io/docs/1.2.0/components/material.html#register-a-custom-shader-material)
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerShader(name: &str, data: JsValue);

    /// [register-a-custom-geometry](https://aframe.io/docs/1.2.0/components/geometry.html#register-a-custom-geometry)
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerGeometry(name: &str, data: JsValue);

    /// [aframe_properties_registerelement](https://aframe.io/docs/1.2.0/core/globals.html#aframe_properties_registerelement)
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerElement(name: &str, data: JsValue);
}

/// Access a field from an object
pub(crate) fn access_field(obj: &Object, field_name: &'static str) -> Option<JsValue>
{
        Object::entries(obj)
            .iter()
            .find(|e| e.dyn_ref::<Array>()
                .filter(|entry| entry
                    .iter()
                    .next()
                    .and_then(|key| key.as_string())
                    .filter(|key_str| key_str == field_name)
                    .is_some())
                .is_some())
}

/// Global [three.js](https://threejs.org/) object.
pub fn three_js() -> Option<JsValue>
{
    AFRAME.as_ref().and_then(|aframe| access_field(&aframe.0, "THREE"))
}

/// Object of registered components.
pub fn components() -> Option<JsValue>
{
    AFRAME.as_ref().and_then(|aframe| access_field(&aframe.0, "components"))
}

/// Object of registered geometries.
pub fn geometries() -> Option<JsValue>
{
    AFRAME.as_ref().and_then(|aframe| access_field(&aframe.0, "geometries"))
}

/// Object of registered primitives.
pub fn primitives() -> Option<JsValue>
{
    AFRAME.as_ref()
        .and_then(|aframe| access_field(&aframe.0, "primitives"))
        .and_then(|primitives| 
        {
            primitives.unchecked_into::<Array>()
                .iter()
                .skip(1)
                .next()
                .and_then(|primitives| access_field(primitives.unchecked_ref(), "primitives"))
        })
}

/// Object of registered shaders.
pub fn shaders() -> Option<JsValue>
{
    AFRAME.as_ref().and_then(|aframe| access_field(&aframe.0, "shaders"))
}

/// Object of registered systems.
pub fn systems() -> Option<JsValue>
{
    AFRAME.as_ref().and_then(|aframe| access_field(&aframe.0, "systems"))
}

/// Version of A-Frame build.
pub fn version() -> Option<JsValue>
{
    AFRAME.as_ref().and_then(|aframe| access_field(&aframe.0, "version"))
}

struct Aframe(Object);
unsafe impl Send for Aframe {}
unsafe impl Sync for Aframe {}

impl Aframe
{
    fn get() -> Option<Self>
    {
        web_sys::window()
            .ok_or("Failed to access window")
            .and_then(Aframe::try_from)
            .ok()
    }
}

impl TryFrom<web_sys::Window> for Aframe
{
    type Error = &'static str;

    fn try_from(window: web_sys::Window) -> Result<Self, Self::Error> 
    {
        window.get("AFRAME")
            .map(Aframe)
            .ok_or("Failed to access AFRAME global")
    }
}