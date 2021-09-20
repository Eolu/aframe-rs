use crate::sys::registerPrimitive;
use std::collections::HashMap;

use super::*;
use serde::{Serialize, Serializer};

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
        default_components: HashMap<Cow<'static, str>, Box<dyn Component>>, 
        mappings: HashMap<Cow<'static, str>, Cow<'static, str>>
    ) -> Self
    {
        // This transmute is safe because ComponentBox is #[repr(transparent)]
        Self { default_components: unsafe 
        { 
            std::mem::transmute::
            <
                HashMap<Cow<'static, str>, Box<dyn Component>>,
                HashMap<Cow<'static, str>, ComponentBox>
            >(default_components) 
        }, mappings }
    }

    /// Register a primitive in aframe. Warning: Aframe must be initialized before this is called.
    pub unsafe fn register(&self, name: &str) -> Result<(), serde_json::error::Error>
    {
        registerPrimitive(name, wasm_bindgen::JsValue::from_serde(self)?);
        Ok(())
    }
}

#[repr(transparent)]
struct ComponentBox(Box<dyn Component>);

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