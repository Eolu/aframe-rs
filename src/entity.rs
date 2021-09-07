use std::borrow::Cow;
use crate::{Attribute, Htmlify, component::Component};


#[derive(Default, Debug)]
pub struct Entity
{
    components: Vec<(Cow<'static, str>, Box<dyn Component>)>,
    children: Vec<Entity>
}


impl Entity
{
    pub fn with_components(components: Vec<(Cow<'static, str>, Box<dyn Component>)>) -> Self
    {
        Self { components, children: vec!() }
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
        self.components.iter().map(|(name, cmp)| Attribute 
        { 
            name: name.to_owned(), 
            value: cmp.to_string().into() 
        }).collect()
    }
    fn inner_html(&self) -> Cow<'static, str>
    {
        self.children.iter().map(|ent| ent.as_raw_html()).collect::<Vec<String>>().join("").into()
    }
}