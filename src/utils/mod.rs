pub mod color;

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
        js_sys::Function::new_with_args(stringify!($($arg), *), stringify!($($tt)*))
    };
    ($($tt:tt)*) => 
    {
        js_sys::Function::new_no_args(stringify!($($tt)*))
    }
}

use std::fmt::Display;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Vector2
{
    pub x: f64,
    pub y: f64
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Vector3
{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Vector4
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
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