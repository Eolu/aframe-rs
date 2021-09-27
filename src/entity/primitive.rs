use crate::sys::registerPrimitive;
use std::collections::HashMap;

use super::*;
use serde::Serialize;

/// Top-level macro to define a new primitive.
/// ```ignore
/// let prim = primitive!
/// {
///     components: 
///     ("position", component::Position{ x: 0.0, y: -2.0, z: -1.0 }),
///     ("rotation", component::Rotation { x: 0.0, y: 45.0, z: 0.0  }),
///     ("geometry", component!(component::Geometry)),
///     ("animation__click", component!
///     { 
///         component::Animation,
///         property: Cow::Borrowed("rotation"),
///         from: Cow::Borrowed("0 45 0"),
///         to: Cow::Borrowed("0 405 0"),
///         start_events: component::List(Cow::Borrowed(&[Cow::Borrowed("click")])),
///         dur: 900,
///         easing: component::Easing::EaseOutCubic
///     }),
///     ("shadow", component!(component::Shadow)),
///     ("material", component!
///     {
///         component::Material, 
///         props: component::MaterialProps(Cow::Owned(vec!((Cow::Borrowed("src"), Cow::Borrowed("#ramen")))))
///     })
///     mappings: 
///     ("src", "material.src"), 
///     ("depth", "geometry.depth"), 
///     ("height", "geometry.height"), 
///     ("width", "geometry.width")
/// };
/// unsafe
/// {
///     match prim.register("ramen-cube")
///     {
///         Ok(_) => (),
///         Err(err) => yew::services::ConsoleService::log(&format!("{:?}", err))
///     }
/// }
/// ```
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

/// Contains primitive definition which may be registered to Aframe
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
