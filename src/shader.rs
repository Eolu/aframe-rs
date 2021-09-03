use crate::sys::registerShader;
use crate::utils::*;
use std::{borrow::Cow, collections::HashMap};
use serde::{Serialize, Serializer};
use wasm_bindgen::prelude::*;

/// Allows shaders to be passed to Aframe
#[derive(Serialize)]
pub struct Shader<'a, 'b, 'c>
{
    schema: HashMap<Cow<'a, str>, Property>,
    #[serde(rename = "vertexShader")] 
    vertex_shader: Cow<'b, str>,
    #[serde(rename = "fragmentShader")] 
    fragment_shader: Cow<'c, str>
}

impl<'a, 'b, 'c> Shader<'a, 'b, 'c>
{
    /// Define a new shader.
    pub fn new
    (
        schema: HashMap<Cow<'a, str>, Property>, 
        vertex_shader: Cow<'b, str>, 
        fragment_shader: Cow<'c, str>
    ) -> Self
    {
        Shader { schema, vertex_shader, fragment_shader }
    }

    /// Register a shader in aframe. Warning: Aframe must be initialized before this is called.
    pub unsafe fn register(&self, name: &str) -> Result<(), serde_json::error::Error>
    {
        registerShader(name, JsValue::from_serde(self)?);
        Ok(())
    }
}

#[derive(Serialize)]
pub struct Property
{
    #[serde(rename = "type")] 
    shader_type: &'static str,
    is: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<DefaultVal>
}

impl Property
{
    pub fn string(is: &'static str, default: Option<Cow<'static, str>>) -> Self
    {
        Property{ shader_type: "string", is, default: default.map(DefaultVal::Str) }
    }

    pub fn int(is: &'static str, default: Option<i64>) -> Self
    {
        Property{ shader_type: "int", is, default: default.map(DefaultVal::Int) }
    }

    pub fn map(is: &'static str, default: Option<Cow<'static, str>>) -> Self
    {
        Property{ shader_type: "map", is, default: default.map(DefaultVal::Str) }
    }

    pub fn number(is: &'static str, default: Option<f64>) -> Self
    {
        Property{ shader_type: "number", is, default: default.map(DefaultVal::Number) }
    }

    pub fn time(is: &'static str, default: Option<f64>) -> Self
    {
        Property{ shader_type: "time", is, default: default.map(DefaultVal::Number) }
    }

    pub fn vec2(is: &'static str, default: Option<Vector2>) -> Self
    {
        Property{ shader_type: "vec2", is, default: default.map(DefaultVal::Vec2) }
    }

    pub fn vec3(is: &'static str, default: Option<Vector3>) -> Self
    {
        Property{ shader_type: "vec3", is, default: default.map(DefaultVal::Vec3) }
    }

    pub fn vec4(is: &'static str, default: Option<Vector4>) -> Self
    {
        Property{ shader_type: "vec4", is, default: default.map(DefaultVal::Vec4) }
    }
}

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
