pub mod primitive;

use std::borrow::Cow;
use crate::{Attribute, ComponentVec, Htmlify, component::Component};

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
            children_vec!
            {
                $($($child),*)?
            }
        )
    };
    ( 
        primitive: $name:literal,
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
            children_vec!
            {
                $($($child),*)?
            }
        )
    }
}

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

#[macro_export]
macro_rules! children_vec
{
    ( 
        $($child:expr),*
    ) => 
    {
        vec![ $($child),* ]
    }
}

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