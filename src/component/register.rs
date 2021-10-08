//! Allows componenets to be registered in aframe. See the `component_def` macro for detailed docs.

use crate::sys::{registerComponent, registerGeometry};
use crate::utils::*;
use std::{borrow::Cow, collections::HashMap};
use serde::{Serialize};
use wasm_bindgen::{JsCast, prelude::*};

/// Top-level macro to define components. Usage resembles struct creation syntax.
/// The `js!` macro is available for writing inline javascript, and returns a
/// js_sys::Function object. This macro calls `into` on expressions passed into the 
/// fields expecting function, allowing the `js!` macro to be used as a catch-all.
/// Takes the optional fields described in the table below.
///
/// | field | syntax explanation | description |
/// |-------|--------------------|-------------|
/// | dependencies | strings separated by commas | names of components that must be initialized prior to this one |
/// | schema | A hashmap containing string keys and ComponentProperty values. Recommend the maplit crate | Describes component properties |
/// | multiple | boolean value | True to allow multiple components on a single entity |
/// | init | JsValue created from a js_sys::Function() | Called on initialization |
/// | update | JsValue created from a js_sys::Function(oldData) | Called whenever the component’s properties change |
/// | tick | JsValue created from a js_sys::Function(time, timeDelta) | Called on each tick or frame of the scene’s render loop |
/// | tock | JsValue created from a js_sys::Function(time, timeDelta, camera) | Identical to the tick method but invoked after the scene has rendered |
/// | remove | JsValue created from a js_sys::Function() | Called whenever the component is detached from the entity |
/// | pause | JsValue created from a js_sys::Function() | Called when the entity or scene pauses |
/// | play | JsValue created from a js_sys::Function() | Called when the entity or scene resumes |
/// | update_schema | JsValue created from a js_sys::Function(data) | if defined, is called on every update in order to check if the schema needs to be dynamically modified |
///
/// All parameteres are optional, although the order must be exactly as shown. 
/// `dependencies` should be a comma-separated list of strings followed by a 
/// semicolon. `schema` should be a HashMap with string keys and `AframeProperty` 
/// values. `multiple` is a boolean value. The rest are strings containing 
/// javascript code. A `js!` macro is provided to allow inline javascript code 
/// to be included in the Rust code (See the docs for the `js!` macro for 
/// caveats and limitations). Here's an example:
/// ```ignore
/// // Example: 
/// let some_component = component_def!
/// (
///     dependencies: "dependency1", "dependency2", some_string,
///     schema: hashmap!
///     {
///         "position" => AframeProperty::float("number", None),
///         "text" => AframeProperty::string("string", Some(Cow::Borrowed("x"))),
///         "autoplay" => AframeProperty::boolean("boolean", Some(true))
///     },
///     multiple: true,
///     init: js!
///     (
///         this.radians = Math.PI * 2; 
///         this.initalRotation = this.el.object3D.rotation.clone();
///     ),
///     update: js!(oldData =>> this.rotation = this.el.object3D.rotation;),
///     tick: js!
///     (time, delta =>>
///         if (this.data.autoplay)
///         {
///             var amount = this.data.radiansPerMillisecond * delta * this.data.speedMult;
///             if (this.data.axis.includes('x'))
///                 this.rotation.x = (this.rotation.x + amount) % this.radians;
///             if (this.data.axis.includes('y'))
///                 this.rotation.y = (this.rotation.y + amount) % this.radians;
///             if (this.data.axis.includes('z'))
///                 this.rotation.z = (this.rotation.z + amount) % this.radians;
///         }
///     ),
///     remove: js!(this.rotation.copy(this.initialRotation);),
///     pause: js!(this.data.autoplay = false;),
///     play: js!(this.data.autoplay = true;),
/// );
/// unsafe
/// {
///     some_component.register("component_name");
/// }
/// ```
#[macro_export]
macro_rules! component_def
{
    (
        $(dependencies: $($deps:expr),*;)? 
        $(schema: $schema:expr,)?
        $(multiple: $mult:expr,)? 
        $(init: $init:expr,)?
        $(update: $update:expr,)?
        $(tick: $tick:expr,)?
        $(tock: $tock:expr,)?
        $(remove: $remove:expr,)?
        $(pause: $pause:expr,)?
        $(play: $play:expr,)?
        $(update_schema: $update_schema:expr,)?
    ) => 
    {
        $crate::component::ComponentReg
        {
            $(schema: $schema,)?
            $(dependencies: std::borrow::Cow::Borrowed(&[$(std::borrow::Cow::Borrowed($deps)),*]),)?
            $(multiple: $mult,)?
            $(init: $init.into(),)?
            $(update: $update.into(),)?
            $(tick: $tick.into(),)?
            $(tock: $tock.into(),)?
            $(remove: $remove.into(),)?
            $(pause: $pause.into(),)?
            $(play: $play.into(),)?
            $(update_schema: $update_schema.into(),)?
            ..$crate::component::ComponentReg::default()
        }
    }
}

/// Top-level macro to define custom geometries. Syntax resemles but is simpler
/// than the `component_def!` macro.
/// The `js!` macro is available for writing inline javascript, and returns a
/// js_sys::Function object. This macro calls `into` on expressions passed into the 
/// fields expecting function, allowing the `js!` macro to be used as a catch-all.
/// Takes the optional fields described in the table below.
///
/// | field | syntax explanation | description |
/// |-------|--------------------|-------------|
/// | schema | A hashmap containing string keys and GeometryProperty values. Recommend the maplit crate | Describes custom geometry properties |
/// | init | JsValue created from a js_sys::Function() | Called on initialization |
///
/// All parameteres are optional, although leaving either out may not result in 
/// a meaningful geometry definition.
/// ```ignore
/// // Example (this is an exact replica of the builtin `box` geometry): 
/// let newbox = geometry_def!
/// {
///     schema: hashmap!
///     {
///         "depth" => GeometryProperty::new(AframeVal::Float(1.0), Some(AframeVal::Float(0.0)), None, None),
///         "height" => GeometryProperty::new(AframeVal::Float(1.0), Some(AframeVal::Float(0.0)), None, None),
///         "width" => GeometryProperty::new(AframeVal::Float(1.0), Some(AframeVal::Float(0.0)), None, None),
///         "segmentsHeight" => GeometryProperty::new(AframeVal::Int(1), Some(AframeVal::Int(1)), Some(AframeVal::Int(20)), Some("int")),
///         "segmentsWidth" => GeometryProperty::new(AframeVal::Int(1), Some(AframeVal::Int(1)), Some(AframeVal::Int(20)), Some("int")),
///         "segmentsDepth" => GeometryProperty::new(AframeVal::Int(1), Some(AframeVal::Int(1)), Some(AframeVal::Int(20)), Some("int")),
///     },
///     init: js!(data =>> this.geometry = new THREE.BoxGeometry(data.width, data.height, data.depth);)
/// };
/// unsafe
/// {
///     newbox.register("newbox");
/// }
/// ```
#[macro_export]
macro_rules! geometry_def
{
    (
        $(schema: $schema:expr,)?
        $(init: $init:expr)?
    ) => 
    {
        $crate::component::GeometryReg
        {
            $(schema: $schema,)?
            $(init: $init.into(),)?
            ..$crate::component::GeometryReg::default()
        }
    }
}

/// Component registration definition. All JsValues should be derived from [`js_sys::Function`]
#[derive(Serialize, Clone)]
pub struct ComponentReg
{
    pub schema: HashMap<&'static str, AframeProperty>,
    pub dependencies: Cow<'static, [Cow<'static, str>]>,
    pub multiple: bool,
    // TODO: events: HashMap<Cow<'static, str>, Function(event)>
    #[serde(skip)] pub init: JsValue,
    #[serde(skip)] pub update: JsValue,
    #[serde(skip)] pub tick: JsValue, 
    #[serde(skip)] pub tock: JsValue,
    #[serde(skip)] pub remove: JsValue,
    #[serde(skip)] pub pause: JsValue,
    #[serde(skip)] pub play: JsValue,
    #[serde(skip)] pub update_schema: JsValue
}
impl Default for ComponentReg
{
    fn default() -> Self 
    {
        let empty_fn: JsValue = js_sys::Function::default().into();
        Self
        {
            schema: HashMap::new(),
            dependencies: Cow::Borrowed(&[]),
            multiple: false,
            init: empty_fn.clone(),
            update: empty_fn.clone(),
            tick: empty_fn.clone(),
            tock: empty_fn.clone(),
            remove: empty_fn.clone(),
            pause: empty_fn.clone(),
            play: empty_fn.clone(),
            update_schema: empty_fn
        }
    }
}
impl From<&ComponentReg> for JsValue
{
    fn from(cmr: &ComponentReg) -> Self 
    {
        let js_value = JsValue::from_serde(cmr).expect("Failed to convert ComponentReg into JsObject");
        define_property(js_value.unchecked_ref(), "init", (cmr.init).unchecked_ref());
        define_property(js_value.unchecked_ref(), "update", (cmr.update).unchecked_ref());
        define_property(js_value.unchecked_ref(), "tick", (cmr.tick).unchecked_ref());
        define_property(js_value.unchecked_ref(), "tock", (cmr.tock).unchecked_ref());
        define_property(js_value.unchecked_ref(), "remove", (cmr.remove).unchecked_ref());
        define_property(js_value.unchecked_ref(), "pause", (cmr.pause).unchecked_ref());
        define_property(js_value.unchecked_ref(), "play", (cmr.play).unchecked_ref());
        define_property(js_value.unchecked_ref(), "update_schema", (cmr.update_schema).unchecked_ref());
        js_value
    }
}
impl ComponentReg
{
    /// Register a component in aframe. Warning: Aframe must be initialized before this is called.
    pub unsafe fn register(self, name: &str)
    {
        registerComponent(name, (&self).into());
    }
}

/// Geometry registration definition. The `init` JsValue should be derived from [`js_sys::Function`]
#[derive(Serialize, Clone)]
pub struct GeometryReg
{
    pub schema: HashMap<&'static str, GeometryProperty>,
    #[serde(skip)] pub init: JsValue,
}
impl Default for GeometryReg
{
    fn default() -> Self 
    {
        Self
        {
            schema: HashMap::new(),
            init: js_sys::Function::default().into(),
        }
    }
}
impl From<&GeometryReg> for JsValue
{
    fn from(cmr: &GeometryReg) -> Self 
    {
        let js_value = JsValue::from_serde(cmr).expect("Failed to convert GeometryReg into JsObject");
        define_property(js_value.unchecked_ref(), "init", (cmr.init).unchecked_ref());
        js_value
    }
}
impl GeometryReg
{
    /// Register a custom geometry in aframe. Warning: Aframe must be initialized before this is called.
    pub unsafe fn register(self, name: &str)
    {
        registerGeometry(name, (&self).into());
    }
}

/// A property for a GeometryReg
#[derive(Serialize, Clone)]
pub struct GeometryProperty
{
    default: AframeVal,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<AframeVal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<AframeVal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")] 
    component_type: Option<&'static str>
}

impl GeometryProperty
{
    pub fn new(default: AframeVal, min: Option<AframeVal>, max: Option<AframeVal>, component_type: Option<&'static str>) -> Self
    {
        GeometryProperty{ default, component_type, min, max }
    }
}