use crate::sys::registerPrimitive;
use std::collections::HashMap;

use super::*;
use serde::{Serialize, Serializer};

// FIXME: This is untested and the component serialization is very likely flawed

#[derive(Serialize, Clone)]
pub struct PrimitiveReg
{
    #[serde(rename = "defaultComponents")] 
    default_components: HashMap<Cow<'static, str>, ComponentBox>,
    mappings: HashMap<Cow<'static, str>, Cow<'static, str>>
}

impl PrimitiveReg
{
    pub fn new
    (
        default_components: HashMap<Cow<'static, str>, ComponentBox>, 
        mappings: HashMap<Cow<'static, str>, Cow<'static, str>>
    ) -> Self
    {
        Self { default_components, mappings }
    }

    /// Register a primitive in aframe. Warning: Aframe must be initialized before this is called.
    pub unsafe fn register(&self, name: &str) -> Result<(), serde_json::error::Error>
    {
        registerPrimitive(name, wasm_bindgen::JsValue::from_serde(self)?);
        Ok(())
    }
}

#[repr(transparent)]
pub struct ComponentBox(Box<dyn Component>);

impl Clone for ComponentBox
{
    fn clone(&self) -> Self 
    {
        Self(self.0.clone())
    }
}

impl Serialize for ComponentBox
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer 
    {
        self.0.as_map().serialize(serializer)
    }
}