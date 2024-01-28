//! Module to provide tools for shader registration as well as built-in shader 
//! constants.

pub const PORTAL: &'static str = "portal";
pub const FLAT: &'static str = "flat";
pub const STANDARD: &'static str = "standard";
pub const SDF: &'static str = "sdf";
pub const MSDF: &'static str = "msdf";

use crate::sys::registerShader;
use crate::utils::*;
use std::{borrow::Cow, collections::HashMap};
use serde::{Serialize, Serializer};

/// Provides all the tools necessary to define an Aframe shader. 
/// The [maplit](https://docs.rs/maplit/1.0.2/maplit/) crate is recommended 
/// for simplifying shader definitions. See below:
/// ```ignore
/// use maplit::*;
/// use aframe::shader::*;
///
/// pub const SIMPLE_VS: &str = include_str!("./SOME_PATH/glsl/simple.vs");
/// pub const STROBE_FS: &str = include_str!("./SOME_PATH/glsl/strobe.fs");
///
/// Shader::new
/// (
///     hashmap!
///     {
///         "speedMult".into() => ShaderProperty::number(IsUniform::Yes, 1.0.into()),
///         "alpha".into() => ShaderProperty::number(IsUniform::Yes, 1.0.into()),
///         "alpha2".into() => ShaderProperty::number(IsUniform::Yes, 1.0.into()),
///         "color".into() => ShaderProperty::color(IsUniform::Yes, color::BLACK.into()),
///         "color2".into() => ShaderProperty::color(IsUniform::Yes, color::WHITE.into()),
///         "iTime".into() => ShaderProperty::time(IsUniform::Yes, None)
///     }, 
///     SIMPLE_VS.into(),
///     STROBE_FS.into()
///     // Calling `register` will send this data to the AFRAME.registerShader function.
/// ).register("strobe")?;
/// ```
#[derive(Serialize)]
pub struct Shader<'a, 'b, 'c>
{
    pub schema: HashMap<Cow<'a, str>, ShaderProperty>,
    #[serde(rename = "vertexShader")] 
    pub vertex_shader: Cow<'b, str>,
    #[serde(rename = "fragmentShader")] 
    pub fragment_shader: Cow<'c, str>
}

impl<'a, 'b, 'c> Shader<'a, 'b, 'c>
{
    /// Define a new shader.
    pub fn new
    (
        schema: HashMap<Cow<'a, str>, ShaderProperty>, 
        vertex_shader: Cow<'b, str>, 
        fragment_shader: Cow<'c, str>
    ) -> Self
    {
        Shader { schema, vertex_shader, fragment_shader }
    }

    /// Register a shader in aframe. Warning: Aframe must be initialized before this is called.
    pub unsafe fn register(&self, name: &str) -> Result<(), serde_wasm_bindgen::Error>
    {
        registerShader(name, serde_wasm_bindgen::to_value(self)?);
        Ok(())
    }
}

/// A property for a shader. This includes the shader type, whether or not this 
/// property is a uniform, and an optional default value.
#[derive(Serialize)]
pub struct ShaderProperty
{
    #[serde(rename = "type")] 
    pub shader_type: &'static str,
    #[serde(skip_serializing_if = "IsUniform::not_uniform")]
    pub is: IsUniform,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<DefaultVal>
}

impl ShaderProperty
{
    pub fn color(is: IsUniform, default: Option<color::Rgb>) -> Self
    {
        ShaderProperty{ shader_type: "color", is, default: default.map(color::Rgb::into).map(DefaultVal::Str) }
    }

    pub fn array(is: IsUniform, default: Option<Vector3>) -> Self
    {
        ShaderProperty{ shader_type: "array", is, default: default.map(DefaultVal::Vec3) }
    }

    pub fn int(is: IsUniform, default: Option<i64>) -> Self
    {
        ShaderProperty{ shader_type: "int", is, default: default.map(DefaultVal::Int) }
    }

    pub fn number(is: IsUniform, default: Option<f64>) -> Self
    {
        ShaderProperty{ shader_type: "number", is, default: default.map(DefaultVal::Number) }
    }

    pub fn map(is: IsUniform, default: Option<Cow<'static, str>>) -> Self
    {
        ShaderProperty{ shader_type: "map", is, default: default.map(DefaultVal::Str) }
    }

    pub fn time(is: IsUniform, default: Option<f64>) -> Self
    {
        ShaderProperty{ shader_type: "time", is, default: default.map(DefaultVal::Number) }
    }

    pub fn vec2(is: IsUniform, default: Option<Vector2>) -> Self
    {
        ShaderProperty{ shader_type: "vec2", is, default: default.map(DefaultVal::Vec2) }
    }

    pub fn vec3(is: IsUniform, default: Option<Vector3>) -> Self
    {
        ShaderProperty{ shader_type: "vec3", is, default: default.map(DefaultVal::Vec3) }
    }

    pub fn vec4(is: IsUniform, default: Option<Vector4>) -> Self
    {
        ShaderProperty{ shader_type: "vec4", is, default: default.map(DefaultVal::Vec4) }
    }
}

/// Wrapper for possible uniform default values
pub enum DefaultVal
{
    Str(Cow<'static, str>),
    Number(f64),
    Int(i64),
    Vec2(Vector2),
    Vec3(Vector3),
    Vec4(Vector4),
}

impl Serialize for DefaultVal
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
    {
        match self
        {
            Self::Str(s) => s.serialize(serializer),
            Self::Number(n) => n.serialize(serializer),
            Self::Int(n) => n.serialize(serializer),
            Self::Vec2(vec) => vec.serialize(serializer),
            Self::Vec3(vec) => vec.serialize(serializer),
            Self::Vec4(vec) => vec.serialize(serializer)
        }
    }
}

/// Enum to make asking whether or not a property is a uniform more readable.
pub enum IsUniform
{
    Yes, No
}
impl IsUniform
{
    fn not_uniform(&self) -> bool
    {
        matches!(self, Self::No)
    }
}
impl Serialize for IsUniform
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
    {
        match self
        {
            Self::Yes => "uniform".serialize(serializer),
            Self::No => "".serialize(serializer)
        }
    }
}