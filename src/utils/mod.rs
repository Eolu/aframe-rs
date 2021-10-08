//! The `js!` macro, vectors and other useful utility modules.

pub mod color;
pub mod htmlify;

pub use ::htmlify::*;
pub use const_default::ConstDefault;
use js_sys::{Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use std::{borrow::Cow, fmt::Display};
use serde::{Serialize, Serializer};
pub use js_sys::Function;

/// Allows a javascript function to be defined inline. Accepts 2 forms of syntax:
/// `js!(<js code>);`
/// `js!(arg1, arg2, arg3 =>> <js code>)`
/// There are some limitations: 
/// - `===` and `!==` cannot be parsed correctly, use `==` and `!=` instead.
/// - String literals must be double-quoted, not single-quoted.
/// - Statements missing a terminating a semi-colon may not parse correctly.
#[macro_export]
macro_rules! js
{
    ($($arg:ident),* =>> $($tt:tt)*) => 
    {
        $crate::utils::Function::new_with_args(stringify!($($arg), *), stringify!($($tt)*))
    };
    ($($tt:tt)*) => 
    {
        $crate::utils::Function::new_no_args(stringify!($($tt)*))
    }
}

/// A 2-dimensional vector
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Vector2
{
    pub x: f64,
    pub y: f64
}
impl ConstDefault for Vector2
{
    const DEFAULT: Vector2 = Vector2 { x: 0.0, y: 0.0 };
}

/// A 3-dimensional vector
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Vector3
{
    pub x: f64,
    pub y: f64,
    pub z: f64
}
impl ConstDefault for Vector3
{
    const DEFAULT: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
}

/// A 4-dimensional vector
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Vector4
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}
impl ConstDefault for Vector4
{
    const DEFAULT: Vector4 = Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
}

impl Display for Vector2
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl Display for Vector3
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Display for Vector4
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        write!(f, "{} {} {} {}", self.x, self.y, self.z, self.w)
    }
}

/// Helper function to attach JsFunctions to a serialized JsValue
pub(crate) fn define_property(src: &Object, name: &str, value: &Object)
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

/// A property used for some registrations in Aframe. 
/// Contains the type string and the default value.
#[derive(Serialize, Clone)]
pub struct AframeProperty
{
    #[serde(rename = "type")] 
    component_type: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<AframeVal>
}

impl AframeProperty
{
    pub fn array(default: Option<Vec<Cow<'static, str>>>) -> Self
    {
        Self { component_type: "array", default: default.map(AframeVal::Array) }
    }

    pub fn asset(default: Option<Cow<'static, str>>) -> Self
    {
        Self { component_type: "asset", default: default.map(AframeVal::Str) }
    }

    pub fn audio(default: Option<Cow<'static, str>>) -> Self
    {
        Self { component_type: "audio", default: default.map(AframeVal::Str) }
    }

    pub fn boolean(default: Option<bool>) -> Self
    {
        Self { component_type: "boolean", default: default.map(AframeVal::Bool) }
    }

    pub fn color(default: Option<Cow<'static, str>>) -> Self
    {
        Self { component_type: "color", default: default.map(AframeVal::Str) }
    }

    pub fn int(default: Option<i64>) -> Self
    {
        Self { component_type: "int", default: default.map(AframeVal::Int) }
    }

    pub fn map(default: Option<Cow<'static, str>>) -> Self
    {
        Self { component_type: "map", default: default.map(AframeVal::Str) }
    }

    pub fn model(default: Option<Cow<'static, str>>) -> Self
    {
        Self { component_type: "model", default: default.map(AframeVal::Str) }
    }

    pub fn number(default: Option<f32>) -> Self
    {
        Self { component_type: "number", default: default.map(AframeVal::Float) }
    }

    pub fn selector(default: Option<Cow<'static, str>>) -> Self
    {
        Self { component_type: "selector", default: default.map(AframeVal::Str) }
    }

    pub fn selector_all(default: Option<Cow<'static, str>>) -> Self
    {
        Self { component_type: "selectorAll", default: default.map(AframeVal::Str) }
    }

    pub fn string(default: Option<Cow<'static, str>>) -> Self
    {
        Self { component_type: "string", default: default.map(AframeVal::Str) }
    }

    pub fn vec2(default: Option<Vector2>) -> Self
    {
        Self { component_type: "vec2", default: default.map(AframeVal::Vec2) }
    }

    pub fn vec3(default: Option<Vector3>) -> Self
    {
        Self { component_type: "vec3", default: default.map(AframeVal::Vec3) }
    }

    pub fn vec4(default: Option<Vector4>) -> Self
    {
        Self { component_type: "vec4", default: default.map(AframeVal::Vec4) }
    }
}

#[derive(Clone)]
pub enum AframeVal
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

impl Serialize for AframeVal
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