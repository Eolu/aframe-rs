use std::{borrow::Cow, fmt::{self, Display, Formatter}};

pub trait Component: Display + std::fmt::Debug + std::any::Any
{
    fn clone(&self) -> Box<dyn Component>;
    fn eq(&self, other: &'static dyn Component) -> bool;
}

/// A macro to define a component struct. The component must be already registered 
/// in aframe before this struct may be used (although constructing it before that
/// is safe). There are 2 variation of syntax provided, depending on the desired 
/// resulting `Display` implementation.
/// 
/// ```ignore
/// use aframe::component_struct;
/// 
/// // Example 1, uses hard-coded display implementation:
/// component_struct!{StructName, 
///     field_1: "field1Name" f32 = 1.5,
///     field_2: "field2Name" bool = false
/// }
///
/// // This will display as: "field1Name: 1.5; field2Name: false"
///
/// // Example 2, uses custom display implementation:
/// component_struct!{Vec3 "{} {} {}", 
///     x: f32 = 1.0,
///     y: f32 = 1.5,
///     z: f32 = 2.0
/// }
/// 
/// // This will display as "1.0 1.5 2.0"
/// ```
/// When using items defined with this macro or with the `complex_enum!` macro 
/// as fields, a custom display implementation may be used to flatten out the
/// nested properties and print correctly as a single semicolon-separated list
/// of properties. 
#[macro_export]
macro_rules! component_struct
{
    ($name:ident $(, $field:ident: $field_name:literal $ty:ty = $default:expr)*) => 
    {
        component_struct!($name concat!($($field_name, ": {};"),*) $(, $field: $ty = $default)*);
    };
    ($name:ident $fmt:expr $(, $field:ident: $ty:ty = $default:expr)*) => 
    {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name
        {
            $(
                pub $field: $ty
            ),*
        }
        impl std::fmt::Display for $name
        {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
            {
                write!(f, $fmt, $(self.$field),*)
            }
        }
        impl ConstDefault for $name
        {
            const DEFAULT: Self = Self 
            {
                $($field: $default),*
            };
        }
        impl Component for $name 
        {
            fn clone(&self) -> Box<dyn Component>
            {
                Box::new(Clone::clone(self))
            }
            fn eq(&self, other: &'static dyn Component) -> bool
            {
                match (&&*other as &dyn std::any::Any).downcast_ref::<&&$name>()
                {
                    Some(other) => self == **other,
                    None => false
                }
            }
        }
    }
}

/// A macro to instantiate a component. Mimics struct creation syntax, but allows
/// any number of fields to be left out (in which case defaults will be used). 
/// Note that the ability to leave out fields does not extend to struct_like
/// enum variants created in this macro.
/// ```ignore
/// // For example:
/// use aframe::component;
/// 
/// component!(component::Light,
///     light_type: component::LightType::Point
///     {
///         decay: 1.0,
///         distance: 50.0,
///         shadow: component::OptionalLocalShadow::NoCast{},
///     }, 
///     intensity: 0.0
/// );
/// ```
#[macro_export]
macro_rules! component
{
    ($($cmp:ident)::* $(, $field:ident: $val:expr)*) => 
    {
        $($cmp)::*
        {
            $($field: $val,)*
            ..$($cmp)::*::DEFAULT
        }
    }
}

/// A macro to define a simple enum which implements Display.
/// ```ignore
/// // For example:
/// use aframe::simple_enum;
/// 
/// simple_enum!
/// (
///     Axis, 
///     X => "x", 
///     Y => "y", 
///     Z => "z"
/// );
/// ```
#[macro_export]
macro_rules! simple_enum
{
    ($name:ident $(, $variant:ident => $s:literal)*) => 
    {
        #[derive(Clone, Copy, PartialEq, Debug)]
        pub enum $name {$($variant),* }
        impl std::fmt::Display for $name
        {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
            {
                write!(f, "{}", match self { $(Self::$variant => $s),* })
            }
        }
    }
}

/// A macro to define an enum in which each field is struct-like. Works similarly
/// to the `component_struct!` macro.
/// ```ignore
/// // For example:
/// use aframe::complex_enum;
/// 
/// complex_enum!{LightType, 
///     Ambient "type: ambient; " => {},
///     Directional "type: directional; {}" => { shadow: Shadow },
///     Hemisphere "type: hemisphere; groundColor: {}" => { ground_color: color::Color  },
///     Point "type: point; decay: {}; distance: {}; {}" => 
///     { 
///         decay: f32, 
///         distance: f32,
///         shadow: Shadow
///     },
///     Spot "type: spot; angle: {}; decay: {}; distance: {}; penumbra: {}; target: {}; {}" =>
///     {
///         angle: i32,
///         decay: f32,
///         distance: f32,
///         penumbra: f32,
///         target: Cow<'static, str>,
///         shadow: Shadow
///     }
/// }
/// ```
#[macro_export]
macro_rules! complex_enum
{
    ($name:ident $(, $variant:ident $fmt:expr => { $($field:ident: $ty:ty),* })*) => 
    {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $name 
        {
            $($variant { $($field: $ty),* }),*
        }
        impl std::fmt::Display for $name
        {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
            {
                match self
                {
                    $(Self::$variant { $($field),* } => write!(f, $fmt, $($field),*)),*
                }
            }
        }
    }
}

/// The type here may look daunting, but all this is just to allow you to create
/// a `Cow<'static, [T]>` field in a component.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq)]
pub struct List<T: Display + ToOwned + std::fmt::Debug + Clone + PartialEq + 'static> 
(pub Cow<'static, [T]>)
where [T]: ToOwned, <[T] as ToOwned>::Owned: std::fmt::Debug;

impl<T: Display + ToOwned + 'static + std::fmt::Debug + Clone + PartialEq> Display for List<T>
where [T]: ToOwned, <[T] as ToOwned>::Owned: std::fmt::Debug
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result 
    {
        let len = self.0.len();
        for (i, item) in self.0.iter().enumerate()
        {
            if i < len - 1
            {
                write!(f, "{},", item)?;
            }
            else
            {
                std::fmt::Display::fmt(&item, f)?;
            }
        }
        Ok(())
    }
}

impl<T: Display + ToOwned + std::fmt::Debug + 'static + Clone + PartialEq> List<T>
where [T]: ToOwned, <[T] as ToOwned>::Owned: std::fmt::Debug
{
    pub const DEFAULT: List<T> = List(Cow::Borrowed(&[]));
}
