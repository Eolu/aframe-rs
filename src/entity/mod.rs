//! Module for the instantiaion of entities and primitives.

pub mod primitive;

use std::borrow::Cow;
use crate::{Attribute, ComponentVec, Htmlify, component::Component};

/// Defines the high-level API for describing entities, with one form for 
/// describing general entities and another for defining specific primitives.

/// Here's an example of a general entity definition:
/// ```ignore
/// entity!
/// {
///     attributes: ("id", "cube-rig"),
///     components: 
///     ("position", component::Position{x: 0.0, y: 2.5, z: -2.0}),
///     ("sound", component!
///     {
///         component::Sound,
///         src: Cow::Borrowed("#ambient_music"), 
///         volume: 0.5
///     }),
///     ("play-sound-on-event", component!
///     {
///         component::PlaySoundOnEvent,
///         mode: component::PlaySoundOnEventMode::ToggleStop, 
///         event: Cow::Borrowed("click")
///     }),
///     ("light", component!
///     {
///         component::Light,
///         light_type: component::LightType::Point
///         {
///             decay: 1.0,
///             distance: 50.0,
///             shadow: component::OptionalLocalShadow::NoCast{},
///         }, 
///         intensity: 0.0
///     }),
///     ("animation__mouseenter", component!
///     {
///         component::Animation,
///         property: Cow::Borrowed("light.intensity"),
///         to: Cow::Borrowed("1.0"),
///         start_events: component::List(Cow::Borrowed(&[Cow::Borrowed("mouseenter")])),
///         dur: 250
///     }),
///     ("animation__mouseleave", component!
///     {
///         component::Animation,
///         property: Cow::Borrowed("light.intensity"),
///         to: Cow::Borrowed("0.0"),
///         start_events: component::List(Cow::Borrowed(&[Cow::Borrowed("mouseleave")])),
///         dur: 250
///     }),
///     children: entity!
///     {
///         primitive: primitive::A_BOX,
///         attributes: ("id", "my-box"),
///         components:
///     }
/// },
/// ```
/// and here's an example of a primitive definition:
/// ```ignore
/// entity!
/// {
///     // This can also jsut a be a string: "a-box" 
///     primitive: primitive::A_BOX,
///     attributes: ("id", "my-box"),
///     components: 
/// }
/// ```
#[macro_export]
macro_rules! entity
{
    ( 
        $(attributes: $(($attr_id:literal, $attr_value:expr)),*)? $(,)?
        $(components: $(($cmp_id:literal, $cmp_value:expr)),*)? $(,)? 
        $(children: $($child:expr),*)? 
    ) => 
    {
        Entity::new
        (
            attributes_vec!
            {
                $($(($attr_id, $attr_value)),*)?
            },
            components_vec!
            {
                $($(($cmp_id, $cmp_value)),*)?
            },
            vec!
            {
                $($($child),*)?
            }
        )
    };
    ( 
        primitive: $name:expr,
        $(attributes: $(($attr_id:literal, $attr_value:expr)),*)? $(,)?
        $(components: $(($cmp_id:literal, $cmp_value:expr)),*)? $(,)? 
        $(children: $($child:expr),*)? 
    ) => 
    {
        Entity::new_primitive
        (
            std::borrow::Cow::Borrowed($name),
            attributes_vec!
            {
                $($(($attr_id, $attr_value)),*)?
            },
            components_vec!
            {
                $($(($cmp_id, $cmp_value)),*)?
            },
            vec!
            {
                $($($child),*)?
            }
        )
    }
}

/// Mid-level macro to create a vector of attributes
#[macro_export]
macro_rules! attributes_vec
{
    ( 
        $(($attr_id:literal, $attr_value:expr)),*
    ) => 
    {
        vec![ $(Attribute::new($attr_id, $attr_value)),* ]
    }
}

/// Mid-level macro to create a vector of components
#[macro_export]
macro_rules! components_vec
{
    ( 
        $(($cmp_id:literal, $cmp_value:expr)),* 
    ) => 
    {
        vec![ $(($cmp_id.into(), Box::new($cmp_value))),* ]
    }
}

/// Struct which represents an Aframe entity or primitive
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Entity
{
    primitive: Option<Cow<'static, str>>,
    attributes: Vec<Attribute>,
    components: ComponentVec,
    children: Vec<Entity>
}

impl Entity
{
    pub fn new(attributes: Vec<Attribute>, components: Vec<(Cow<'static, str>, Box<dyn Component>)>, children: Vec<Entity>) -> Self
    {
        Self { primitive: None, attributes, components: ComponentVec(components), children }
    }

    pub fn new_primitive(tag: Cow<'static, str>, attributes: Vec<Attribute>, components: Vec<(Cow<'static, str>, Box<dyn Component>)>, children: Vec<Entity>) -> Self
    {
        Self { primitive: Some(tag), attributes, components: ComponentVec(components), children }
    }

    pub fn with_components(components: Vec<(Cow<'static, str>, Box<dyn Component>)>) -> Self
    {
        Self { primitive: None, attributes: vec!(), components: ComponentVec(components), children: vec!() }
    }

    pub fn attributes(&self) -> &Vec<Attribute>
    {
        &self.attributes
    }

    pub fn attributes_mut(&mut self) -> &mut Vec<Attribute>
    {
        &mut self.attributes
    }

    pub fn components(&self) -> &Vec<(Cow<'static, str>, Box<dyn Component>)>
    {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut Vec<(Cow<'static, str>, Box<dyn Component>)>
    {
        &mut self.components
    }

    pub fn children(&self) -> &Vec<Entity>
    {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<Entity>
    {
        &mut self.children
    }

    pub fn tag(&self) -> Cow<'static, str>
    {
        match self.primitive
        {
            Some(ref tag) => tag.clone(),
            None => Cow::Borrowed(Self::TAG)
        }
    }
}

impl From<&(Cow<'static, str>, Box<dyn Component>)> for Attribute
{
    fn from((name, cmp): &(Cow<'static, str>, Box<dyn Component>)) -> Self 
    {
        Attribute 
        { 
            name: name.to_owned(), 
            value: format!("{}", cmp).into() 
        }
    }
}