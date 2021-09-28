use std::
{
    borrow::Cow, 
    collections::HashMap, 
    fmt::{self, Display, Formatter}, 
    ops::{Deref, DerefMut}
};

pub trait Component: Display + std::fmt::Debug + std::any::Any
{
    fn clone(&self) -> Box<dyn Component>;
    fn eq(&self, other: &'static dyn Component) -> bool;
    fn as_map(&self) -> HashMap<Cow<'static, str>, Cow<'static, str>>;
}

impl serde::Serialize for Box<dyn Component>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer 
    {
        self.as_map().serialize(serializer)
    }
}

impl Clone for Box<dyn Component>
{
    fn clone(&self) -> Self 
    {
        Component::clone(&**self)
    }
}

/// A vector containing a tuple of components along with their property name
#[derive(Default, Debug)]
#[repr(transparent)]
pub struct ComponentVec(pub Vec<(Cow<'static, str>, Box<dyn Component>)>);

impl Deref for ComponentVec
{
    type Target = Vec<(Cow<'static, str>, Box<dyn Component>)>;
    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl DerefMut for ComponentVec
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.0
    }
}

impl PartialEq for ComponentVec
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.0.len() == other.0.len() &&
        matches!(self.0.iter().enumerate().filter(|(i, (_name, cmp))| 
            cmp.eq(&*unsafe{std::mem::transmute::<&ComponentVec, &'static ComponentVec>(other)}
                .0[*i].1)).next(), None)
    }
}

impl Clone for ComponentVec
{
    fn clone(&self) -> Self 
    {
        Self(self.0.iter().map(|(name, cmp)| (name.clone(), (*cmp).clone())).collect())
    }
}

/// While `component_def!` creates a component that Aframe can access from its 
/// own runtime, the `component_struct!` macro creates a Rust struct that mimics
/// the internal details of that Aframe component. Component structs are already
/// provided for Aframe's built-in components (WIP: not all components are defined
/// yet. Once all aframe components are defined, calling `component_struct!` 
/// should only be necessary for locally-defined components. Once all components
/// from Aframe have been defined, `component_def!` and `component_struct!` may
/// be merged into a single macro to do the heavy-lifting of both at once). The
/// component must be already registered in aframe before this struct may be used 
/// (although constructing it before that is safe). There are 2 variation of 
/// syntax provided, depending on the desired resulting `Display` implementation.
/// 
/// ```ignore
/// use aframe::component_struct;
/// 
/// // Example 1, uses hard-coded display implementation:
/// component_struct!
/// {
///     /// Doc comment for StructName
///     StructName, 
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
    ($(#[$outer:meta])* $name:ident $(, $field:ident: $field_name:literal $ty:ty = $default:expr)*) => 
    {
        component_struct!($(#[$outer])* $name concat!($($field_name, ": {};"),*) $(, $field: $field_name $ty = $default)*);
    };
    ($(#[$outer:meta])* $name:ident $(:$alt:ident)? $fmt:expr $(, $field:ident: $field_name:literal $ty:ty = $default:expr)*) => 
    {
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq, serde::Serialize)]
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
                if stringify!($($alt)?).len() < 2
                {
                    $(
                        if self.$field != Self::DEFAULT.$field
                        {
                            if $field_name.len() <= 1
                            {
                                write!(f, "{};", self.$field)?;
                            }
                            else
                            {
                                write!(f, concat!($field_name, ": {};"), self.$field)?;
                            }
                        }
                    )*
                    Ok(())
                }
                else
                {
                    write!(f, $fmt, $(self.$field),*)
                }
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
            fn as_map(&self) -> std::collections::HashMap<Cow<'static, str>, Cow<'static, str>>
            {
                #[allow(unused_mut)]
                let mut map = std::collections::HashMap::new();
                $( if $field_name.len() < 1
                {
                    let mut inner_map = std::collections::HashMap::new();
                    for (k, v) in self.$field
                        .to_string()
                        .split(";")
                        .map(str::trim)
                        .filter_map(|s| s.split_once(":"))
                    {
                        inner_map.insert
                        (
                            k.trim().to_owned().into(), 
                            v.trim().to_owned().into()
                        );
                    }
                    map.extend(inner_map);
                }
                else
                {
                    map.insert($field_name.into(), self.$field.to_string().into());
                })*
                map
            }
        }
    }
}

/// A macro to instantiate a component. Mimics struct creation syntax, but allows
/// any number of fields to be left out (in which case defaults will be used). 
/// Note that the ability to leave out fields does not extend to struct_like
/// enum variants created in this macro. For example: `component!{component::Camera}` 
/// will create a `camera` component with all its fields set to default values, 
/// whereas `component!{component::Camera, active = false}` will create a 
/// `camera` component with all its fields set to default values except the 
/// `active` field.
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

/// Defines an enum in which each variant maps to a single string (via a 
/// `Display` implementation). This can be combined with `component_def!` 
/// to crate fields with a limited number of possiblities.
/// ```ignore
/// // For example:
/// use aframe::simple_enum;
/// 
/// simple_enum!
/// (Autoplay, 
///     Null => "null", 
///     True => "true", 
///     False => "false"
/// );
/// component_struct!
/// (Animation,
///     // ...
///     autoplay: "autoplay" Autoplay = Autoplay::Null,
///     // ...
/// );
/// ```
#[macro_export]
macro_rules! simple_enum
{
    ($(#[$outer:meta])* $name:ident $(, $variant:ident => $s:literal)*) => 
    {
        $(#[$outer])* 
        #[derive(Clone, Copy, PartialEq, Debug, serde::Serialize)]
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

/// A macro to define an enum in which each variant maps to an arbitrary number 
/// of fields which will themselves be flattened into fields of the component 
/// itself. Works similarly to the `component_struct!` macro.
/// ```ignore
/// // For example:
/// use aframe::complex_enum;
/// 
/// complex_enum!
/// (AnimationLoop, 
///     Amount "{}" => { looping: u32 },
///     Forever "true" => {}
/// );
/// component_struct!
/// (Animation,
///     // ...
///     looping: "loop" AnimationLoop = AnimationLoop::Amount{looping: 0},
///     // ...
/// );
/// 
/// // Another example is as follows:
/// 
/// complex_enum!
/// (
///     /// Doc comment for GeometryPrimitive enum
///     GeometryPrimitive, 
///     Box
///     "primitive: box; width: {}; height: {}; depth: {}; segmentsWidth: {}; \
///     segmentsHeight: {}; segmentsDepth: {}" => 
///     {  
///         width: f32,
///         height: f32,
///         depth: f32,
///         segments_width: u32,
///         segments_height: u32,
///         segments_depth: u32
///     },
///     Circle
///     "primitive: circle; radius: {}; segments: {}; \
///     thetaStart: {}; thetaLength: {}" =>
///     {
///         radius: f32,
///         segments: u32,
///         theta_start: f32,
///         theta_length: f32
///     },
///     // ...
/// );
/// component_struct!
/// (
///     /// This doc comment will be captured
///     Geometry,
///     primitive: "" GeometryPrimitive = GeometryPrimitive::Box
///     {
///         width: 1.0,
///         height: 1.0,
///         depth: 1.0,
///         segments_width: 1,
///         segments_height: 1,
///         segments_depth: 1,
///     },
///     // ...
/// );
/// ```
#[macro_export]
macro_rules! complex_enum
{
    ($(#[$outer:meta])* $name:ident $(, $variant:ident $fmt:expr => { $($field:ident: $ty:ty),* })*) => 
    {
        $(#[$outer])* 
        #[derive(Debug, Clone, PartialEq, serde::Serialize)]
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
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct List<T: Display + ToOwned + std::fmt::Debug + Clone + PartialEq + serde::Serialize + 'static> 
(pub Cow<'static, [T]>)
where [T]: ToOwned, <[T] as ToOwned>::Owned: std::fmt::Debug;

impl<T: Display + ToOwned + 'static + std::fmt::Debug + Clone + PartialEq + serde::Serialize> Display for List<T>
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

impl<T: Display + ToOwned + std::fmt::Debug + 'static + Clone + PartialEq + serde::Serialize> List<T>
where [T]: ToOwned, <[T] as ToOwned>::Owned: std::fmt::Debug
{
    pub const DEFAULT: List<T> = List(Cow::Borrowed(&[]));
}
