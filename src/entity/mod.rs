mod primitive;

use std::borrow::Cow;
use crate::{Attribute, Htmlify, component::Component};


#[derive(Default, Debug)]
pub struct Entity
{
    attributes: Vec<Attribute>,
    components: Vec<(Cow<'static, str>, Box<dyn Component>)>,
    children: Vec<Entity>
}

impl PartialEq for Entity
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.attributes == other.attributes && 
        self.children == other.children &&
        self.components.len() == other.components.len() &&
        matches!(self.components.iter().enumerate().filter(|(i, (_name, cmp))| 
            cmp.eq(&*unsafe{std::mem::transmute::<&Entity, &'static Entity>(other)}
                .components[*i].1)).next(), None)
    }
}

impl Clone for Entity
{
    fn clone(&self) -> Self 
    {
        Self 
        { 
            attributes: self.attributes.clone(), 
            components: self.components.iter().map(|(name, cmp)| (name.clone(), (*cmp).clone())).collect(), 
            children: self.children.clone() 
        }
    }
}


impl Entity
{
    pub fn new(attributes: Vec<Attribute>, components: Vec<(Cow<'static, str>, Box<dyn Component>)>, children: Vec<Entity>) -> Self
    {
        Self { attributes, components, children }
    }

    pub fn with_components(components: Vec<(Cow<'static, str>, Box<dyn Component>)>) -> Self
    {
        Self { attributes: vec!(), components, children: vec!() }
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
}

impl Htmlify for Entity
{
    const TAG: &'static str = "a-entity";
    fn attributes(&self) -> Vec<Attribute>
    {
        self.components.iter()
            .map(Attribute::from)
            .chain(self.attributes.iter().map(Attribute::clone))
            .collect()
    }
    fn inner_html(&self) -> Cow<'static, str>
    {
        self.children.iter().map(|ent| ent.as_raw_html()).collect::<Vec<String>>().join("").into()
    }
}

impl From<&(Cow<'static, str>, Box<dyn Component>)> for Attribute
{
    fn from((name, cmp): &(Cow<'static, str>, Box<dyn Component>)) -> Self 
    {
        Attribute 
        { 
            name: name.to_owned(), 
            value: cmp.to_string().into() 
        }
    }
}