//! Module for the registration of primitives and for constants that map to
//! Aframe's built-in primitives.

use crate::sys::registerPrimitive;
use std::collections::HashMap;

use super::*;
use serde::Serialize;

/// https://aframe.io/docs/1.2.0/primitives/a-box.html
pub const A_BOX: &'static str = "a-box";
/// https://aframe.io/docs/1.2.0/primitives/a-camera.html
pub const A_CAMERA: &'static str = "a-camera";
/// https://aframe.io/docs/1.2.0/primitives/a-circle.html
pub const A_CIRCLE: &'static str = "a-circle";
/// https://aframe.io/docs/1.2.0/primitives/a-cone.html
pub const A_CONE: &'static str = "a-cone";
/// https://aframe.io/docs/1.2.0/primitives/a-cursor.html
pub const A_CURSOR: &'static str = "a-cursor";
/// https://aframe.io/docs/1.2.0/primitives/a-curvedimage.html
pub const A_CURVEDIMAGE: &'static str = "a-curvedimage";
/// https://aframe.io/docs/1.2.0/primitives/a-cylinder.html
pub const A_CYLINDER: &'static str = "a-cylinder";
/// https://aframe.io/docs/1.2.0/primitives/a-dodecahedron.html
pub const A_DODECAHEDRON: &'static str = "a-dodecahedron";
/// https://aframe.io/docs/1.2.0/primitives/a-gltf-model.html
pub const A_GLTF_MODEL: &'static str = "a-gltf-model";
/// https://aframe.io/docs/1.2.0/primitives/a-icosahedron.html
pub const A_ICOSAHEDRON: &'static str = "a-icosahedron";
/// https://aframe.io/docs/1.2.0/primitives/a-image.html
pub const A_IMAGE: &'static str = "a-image";
/// https://aframe.io/docs/1.2.0/primitives/a-light.html
pub const A_LIGHT: &'static str = "a-light";
/// https://aframe.io/docs/1.2.0/primitives/a-link.html
pub const A_LINK: &'static str = "a-link";
/// https://aframe.io/docs/1.2.0/primitives/a-obj-model.html
pub const A_OBJ_MODEL: &'static str = "a-obj-model";
/// https://aframe.io/docs/1.2.0/primitives/a-octahedron.html
pub const A_OCTAHEDRON: &'static str = "a-octahedron";
/// https://aframe.io/docs/1.2.0/primitives/a-plane.html
pub const A_PLANE: &'static str = "a-plane";
/// https://aframe.io/docs/1.2.0/primitives/a-ring.html
pub const A_RING: &'static str = "a-ring";
/// https://aframe.io/docs/1.2.0/primitives/a-sky.html
pub const A_SKY: &'static str = "a-sky";
/// https://aframe.io/docs/1.2.0/primitives/a-sound.html
pub const A_SOUND: &'static str = "a-sound";
/// https://aframe.io/docs/1.2.0/primitives/a-sphere.html
pub const A_SPHERE: &'static str = "a-sphere";
/// https://aframe.io/docs/1.2.0/primitives/a-tetrahedron.html
pub const A_TETRAHEDRON: &'static str = "a-tetrahedron";
/// https://aframe.io/docs/1.2.0/primitives/a-text.html
pub const A_TEXT: &'static str = "a-text";
/// https://aframe.io/docs/1.2.0/primitives/a-torus-knot.html
pub const A_TORUS_KNOT: &'static str = "a-torus-knot";
/// https://aframe.io/docs/1.2.0/primitives/a-torus.html
pub const A_TORUS: &'static str = "a-torus";
/// https://aframe.io/docs/1.2.0/primitives/a-triangle.html
pub const A_TRIANGLE: &'static str = "a-triangle";
/// https://aframe.io/docs/1.2.0/primitives/a-video.html
pub const A_VIDEO: &'static str = "a-video";
/// https://aframe.io/docs/1.2.0/primitives/a-videosphere.html
pub const A_VIDEOSPHERE: &'static str = "a-videosphere";

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
