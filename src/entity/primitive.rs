use crate::sys::registerPrimitive;
use std::collections::HashMap;

use super::*;
use serde::{Serialize, Serializer};

/// Macro to create a new primitive
#[macro_export]
macro_rules! primitive
{
    (
        components: $(($name:expr, $cmp:expr)),*
        mappings: $(($map_name:expr, $map_expr:expr)),*
    ) => 
    {
        {
            use std::collections::HashMap;
            let mut components: HashMap<std::borrow::Cow<'static, str>, Box<dyn Component>> = HashMap::new();
            $(components.insert($name.into(), Box::new($cmp));)*
            let mut mappings = HashMap::new();
            $(mappings.insert($map_name.into(), $map_expr.into());)*
            PrimitiveReg::new(components, mappings)
        }
    }
}

/// Contains primitive definition which may be registered to aframe
#[derive(Serialize, Clone)]
pub struct PrimitiveReg
{
    #[serde(rename = "defaultComponents")] 
    default_components: HashMap<Cow<'static, str>, Box<dyn Component>>,
    mappings: HashMap<Cow<'static, str>, Cow<'static, str>>
}

impl PrimitiveReg
{
    /// Create a new primitive definition. This is more easily done with the 
    /// `primitive!` macro.
    pub fn new
    (
        default_components: HashMap<Cow<'static, str>, Box<dyn Component>>, 
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
