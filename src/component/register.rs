//! Allows componenets to be registered in aframe. See the `component_def` macro for detailed docs.

use crate::sys::registerComponent;
use crate::utils::*;
use std::{borrow::Cow, collections::HashMap};

use serde::{Serialize, Serializer};
use wasm_bindgen::{JsCast, prelude::*};
use js_sys::{Object, Reflect};

/// Top-level macro to define components. Usage resembles struct creation syntax.
/// The `js!` macro is available for writing inline javascript, and returns a
/// js_sys::Function object. This macro calls `into` on expressions passed into the 
/// fields expecting function, allowing the `js!` macro to be used as a catch-all.
/// Takes the optional fields described in the table below.
///
/// | field | syntax explanation | description |
/// |-------|--------------------|-------------|
/// | dependencies | strings separated by commas | names of components that must be initialized prior to this one |
/// | schema | A hashmap containing string keys and SchemaProperty values. Recommend the maplit crate | Describes component properties |
/// | multiple | boolean value | True to allow multiple components on a single entity |
/// | init | JsValue created from a js_sys::Function() | Called on initialization |
/// | update | JsValue created from a js_sys::Function(oldData) | Called whenever the component’s properties change |
/// | tick | JsValue created from a js_sys::Function(time, timeDelta) | Called called on each tick or frame of the scene’s render loop |
/// | tock | JsValue created from a js_sys::Function(time, timeDelta, camera) | Identical to the tick method but invoked after the scene has rendered |
/// | remove | JsValue created from a js_sys::Function() | Called whenever the component is detached from the entity |
/// | pause | JsValue created from a js_sys::Function() | Called when the entity or scene pauses |
/// | play | JsValue created from a js_sys::Function() | Called when the entity or scene resumes |
/// | update_schema | JsValue created from a js_sys::Function(data) | if defined, is called on every update in order to check if the schema needs to be dynamically modified |
///
/// ```
/// // Example: 
/// component_def!
/// (
///     dependencies: "dependency1", "dependency2", some_string,
///     schema: hashmap!
///     {
///         "position" => SchemaProperty::float("number", None),
///         "text" => SchemaProperty::string("string", Some(Cow::Borrowed("x"))),
///         "autoplay" => SchemaProperty::boolean("boolean", Some(true))
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
/// ```
#[macro_export]
macro_rules! component_def
{
    (
        $(dependencies: $($deps:expr),*,)? 
        $(schema: $schema:expr,)?
        $(multiple $mult:expr,)? 
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
            $(dependencies: std::borrow::Cow::Borrowed(&[$($deps.into()),*]),)?
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

/// Component registration definition. 
#[derive(Serialize, Clone)]
pub struct ComponentReg
{
    schema: HashMap<&'static str, SchemaProperty>,
    dependencies: Cow<'static, [Cow<'static, str>]>,
    multiple: bool,
    // TODO: events: HashMap<Cow<'static, str>, Function(event)>
    #[serde(skip)] init: JsValue,
    #[serde(skip)] update: JsValue,
    #[serde(skip)] tick: JsValue, 
    #[serde(skip)] tock: JsValue,
    #[serde(skip)] remove: JsValue,
    #[serde(skip)] pause: JsValue,
    #[serde(skip)] play: JsValue,
    #[serde(skip)] update_schema: JsValue
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
        ComponentReg::define_property(js_value.unchecked_ref(), "init", (cmr.init).unchecked_ref());
        ComponentReg::define_property(js_value.unchecked_ref(), "update", (cmr.update).unchecked_ref());
        ComponentReg::define_property(js_value.unchecked_ref(), "tick", (cmr.tick).unchecked_ref());
        ComponentReg::define_property(js_value.unchecked_ref(), "tock", (cmr.tock).unchecked_ref());
        ComponentReg::define_property(js_value.unchecked_ref(), "remove", (cmr.remove).unchecked_ref());
        ComponentReg::define_property(js_value.unchecked_ref(), "pause", (cmr.pause).unchecked_ref());
        ComponentReg::define_property(js_value.unchecked_ref(), "play", (cmr.play).unchecked_ref());
        ComponentReg::define_property(js_value.unchecked_ref(), "update_schema", (cmr.update_schema).unchecked_ref());
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

    fn define_property(src: &Object, name: &str, value: &Object)
    {
        if src.unchecked_ref::<JsValue>() != &JsValue::UNDEFINED
        {
            #[allow(unused_unsafe)]
            unsafe
            {
                Reflect::set(src, &JsValue::from_str(name), value).expect(&format!("Failed to define property on: {}", name));
            }
        }
    }
}

#[derive(Serialize, Clone)]
pub struct SchemaProperty
{
    #[serde(rename = "type")] 
    component_type: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<DefaultVal>
}

impl SchemaProperty
{
    pub fn array(default: Option<Vec<Cow<'static, str>>>) -> Self
    {
        SchemaProperty{ component_type: "array", default: default.map(DefaultVal::Array) }
    }

    pub fn asset(default: Option<Cow<'static, str>>) -> Self
    {
        SchemaProperty{ component_type: "asset", default: default.map(DefaultVal::Str) }
    }

    pub fn audio(default: Option<Cow<'static, str>>) -> Self
    {
        SchemaProperty{ component_type: "audio", default: default.map(DefaultVal::Str) }
    }

    pub fn boolean(default: Option<bool>) -> Self
    {
        SchemaProperty{ component_type: "boolean", default: default.map(DefaultVal::Bool) }
    }

    pub fn color(default: Option<Cow<'static, str>>) -> Self
    {
        SchemaProperty{ component_type: "color", default: default.map(DefaultVal::Str) }
    }

    pub fn int(default: Option<i64>) -> Self
    {
        SchemaProperty{ component_type: "int", default: default.map(DefaultVal::Int) }
    }

    pub fn map(default: Option<Cow<'static, str>>) -> Self
    {
        SchemaProperty{ component_type: "map", default: default.map(DefaultVal::Str) }
    }

    pub fn model(default: Option<Cow<'static, str>>) -> Self
    {
        SchemaProperty{ component_type: "model", default: default.map(DefaultVal::Str) }
    }

    pub fn number(default: Option<f32>) -> Self
    {
        SchemaProperty{ component_type: "number", default: default.map(DefaultVal::Float) }
    }

    pub fn string(default: Option<Cow<'static, str>>) -> Self
    {
        SchemaProperty{ component_type: "string", default: default.map(DefaultVal::Str) }
    }

    pub fn vec2(default: Option<Vector2>) -> Self
    {
        SchemaProperty{ component_type: "vec2", default: default.map(DefaultVal::Vec2) }
    }

    pub fn vec3(default: Option<Vector3>) -> Self
    {
        SchemaProperty{ component_type: "vec3", default: default.map(DefaultVal::Vec3) }
    }

    pub fn vec4(default: Option<Vector4>) -> Self
    {
        SchemaProperty{ component_type: "vec4", default: default.map(DefaultVal::Vec4) }
    }
}

#[derive(Clone)]
pub enum DefaultVal
{
    Array(Vec<Cow<'static, str>>),
    Bool(bool),
    Int(i64),
    Str(Cow<'static, str>),
    Float(f32),
    Vec2(Vector2),
    Vec3(Vector3),
    Vec4(Vector4)
}

impl Serialize for DefaultVal
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
    {
        match self
        {
            Self::Str(s) => s.serialize(serializer),
            Self::Float(f) => f.serialize(serializer),
            Self::Vec2(vec) => vec.serialize(serializer),
            Self::Vec3(vec) => vec.serialize(serializer),
            Self::Vec4(vec) => vec.serialize(serializer),
            Self::Array(arr) => arr.serialize(serializer),
            Self::Bool(b) => b.serialize(serializer),
            Self::Int(i) => i.serialize(serializer),
        }
    }
}
