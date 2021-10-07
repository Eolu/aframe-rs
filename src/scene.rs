//! The scene construct, the top-level container for all other Aframe structures.

use std::borrow::Cow;
use crate::{ComponentVec, Assets, Attribute, component::Component, entity::*};

/// Provided to define a `Scene` struct.
/// ```ignore
/// let aframe_scene = scene!
/// {
///     attributes: ("style", "min-height: 50px;"),
///
///     assets: assets!
///     {
///         Image::new("image-name", "/my-image.png")
///     }
///
///     components: ("embedded", component!(Embedded)),
///
///     children:
///     entity!
///     {
///         attributes: ("id", "test-entity"),
///         components: ("position", component!(component::Position)),
///     }
/// }
/// ```
#[macro_export]
macro_rules! scene
{
    ( 
        $(attributes: $(($attr_id:literal, $attr_value:expr)),*)? $(,)?
        assets: $assets:expr,
        $(components: $(($cmp_id:literal, $cmp_value:expr)),*)? $(,)? 
        $(children: $($child:expr),*)? 
    ) => 
    {
        Scene::new
        (
            $crate::attributes_vec!
            {
                $($(($attr_id, $attr_value)),*)?
            },
            $assets,
            $crate::components_vec!
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Scene
{
    attributes: Vec<Attribute>,
    assets: Assets,
    components: ComponentVec,
    children: Vec<Entity>
}

impl Scene
{
    pub fn new(attributes: Vec<Attribute>, assets: Assets, components: Vec<(Cow<'static, str>, Box<dyn Component>)>, children: Vec<Entity>) -> Self
    {
        Self { attributes, assets, components: ComponentVec(components), children }
    }

    pub fn with_components(components: Vec<(Cow<'static, str>, Box<dyn Component>)>) -> Self
    {
        Self { attributes: vec!(), assets: Assets::default(), components: ComponentVec(components), children: vec!() }
    }

    pub fn assets(&self) -> &Assets
    {
        &self.assets
    }

    pub fn assets_mut(&mut self) -> &mut Assets
    {
        &mut self.assets
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