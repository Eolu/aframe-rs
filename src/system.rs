use std::collections::HashMap;

use crate::sys::{registerSystem};
use crate::utils::*;
use serde::Serialize;
use wasm_bindgen::{JsCast, prelude::*};


/// Top-level macro to define systems. Usage resembles struct creation syntax.
/// The `js!` macro is available for writing inline javascript, and returns a
/// js_sys::Function object. This macro calls `into` on expressions passed into the 
/// fields expecting function, allowing the `js!` macro to be used as a catch-all.
/// Takes the optional fields described in the table below.
///
/// | field | syntax explanation | description |
/// |-------|--------------------|-------------|
/// | schema | A hashmap containing string keys and ComponentProperty values. Recommend the maplit crate | Describes component properties |
/// | init | JsValue created from a js_sys::Function() | Called on initialization |
/// | pause | JsValue created from a js_sys::Function() | Called when the entity or scene pauses |
/// | play | JsValue created from a js_sys::Function() | Called when the entity or scene resumes |
/// | tick | JsValue created from a js_sys::Function(time, timeDelta) | Called on each tick or frame of the scene’s render loop |
/// | properties | name: JsValue, ... | Additional comma-separated functions or data with any valid name may be specified |
///
/// All parameteres are optional, although the order must be exactly as shown. 
/// `schema` should be a HashMap with string keys and `AframeProperty` 
/// values. The rest are strings containing  javascript code. A `js!` macro 
/// is provided to allow inline javascript code to be included in the Rust code
/// (See the docs for the `js!` macro for caveats and limitations). Here's an 
/// example:
/// ```ignore
/// // Example: 
/// let some_system = system_def!
/// (
///     schema: hashmap!
///     {
///         "some_float" => AframeProperty::float("number", None),
///         "some_text" => AframeProperty::string("string", Some(Cow::Borrowed("init")))
///     },
///     init: js!
///     (
///         this.data.some_float = 1.0; 
///         this.data.some_text = "I'm a bit of text";
///     ),
///     tick: js!
///     (time, delta =>>
///         this.data.some_float = this.data.some_float + 1.0;
///     ),
///     pause: js!(this.data.some_text = "paused!";),
///     play: js!(this.data.some_text = "playing!";),
///     properties:
///         reset_me: js!(this.data.some_float = 0.0;)
/// );
/// unsafe
/// {
///     some_system.register("system_name");
/// }
/// ```
#[macro_export]
macro_rules! system_def
{
    (
        $(schema: $schema:expr,)?
        $(init: $init:expr,)?
        $(pause: $pause:expr,)?
        $(play: $play:expr,)?
        $(tick: $tick:expr,)?
        $(properties: $($fn_name:ident: $func:expr),*)?
    ) => 
    {
        $crate::system::SystemReg
        {
            $(schema: $schema,)?
            $(init: $init.into(),)?
            $(pause: $pause.into(),)?
            $(play: $play.into(),)?
            $(tick: $tick.into(),)?
            $(properties: 
            {
                let mut props = std::collections::HashMap::new();
                $(
                    props.insert(stringify!($fn_name), $func.into());
                )*
                props
            },)?
            ..$crate::system::SystemReg::default()
        }
    }
}

/// System registration definition. All JsValues should be derived from [`js_sys::Function`]
#[derive(Serialize, Clone)]
pub struct SystemReg
{
    pub schema: HashMap<&'static str, AframeProperty>,
    #[serde(skip)] pub init: JsValue,
    #[serde(skip)] pub pause: JsValue,
    #[serde(skip)] pub play: JsValue,
    #[serde(skip)] pub tick: JsValue,
    #[serde(skip)] pub properties: HashMap<&'static str, JsValue>
}
impl Default for SystemReg
{
    fn default() -> Self 
    {
        let empty_fn: JsValue = js_sys::Function::default().into();
        Self
        {
            schema: HashMap::new(),
            init: empty_fn.clone(),
            pause: empty_fn.clone(),
            play: empty_fn.clone(),
            tick: empty_fn,
            properties: HashMap::new()
        }
    }
}
impl From<&SystemReg> for JsValue
{
    fn from(sysr: &SystemReg) -> Self 
    {
        let js_value = serde_wasm_bindgen::to_value(sysr).expect("Failed to convert SystemReg into JsObject");
        define_property(js_value.unchecked_ref(), "init", (sysr.init).unchecked_ref());
        define_property(js_value.unchecked_ref(), "pause", (sysr.pause).unchecked_ref());
        define_property(js_value.unchecked_ref(), "play", (sysr.play).unchecked_ref());
        define_property(js_value.unchecked_ref(), "tick", (sysr.tick).unchecked_ref());
        for (k, v) in sysr.properties.iter()
        {
            define_property(js_value.unchecked_ref(), k, v.unchecked_ref());
        }
        js_value
    }
}
impl SystemReg
{
    /// Register a system in aframe. Warning: Aframe must be initialized before this is called.
    pub unsafe fn register(self, name: &str)
    {
        registerSystem(name, (&self).into());
    }
}