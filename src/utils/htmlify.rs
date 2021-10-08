//! Module that defines the Htmlify trait and implements it for items in this
//! crate.

use std::{borrow::{Borrow, Cow}, fmt::Display};

use crate::{Asset, AssetItem, Assets, Audio, Entity, Image, Mixin, Scene, Video};

/// Trait used to generate HTML from aframe objects.
/// The `Htmlify` trait is defined for all components, entities, and scenes.
/// 
/// It contains the following required definition:
/// ```ignore
/// const TAG: &'static str;
/// ```
/// and the following optional definitions:
/// ```ignore
/// fn attributes(&self) -> Vec<Attribute>;
/// fn inner_html(&self) -> Cow<'static, str>;
/// ```
/// as well as the following definition which should not need to be implemented, but may on occasion be useful to be overridden:
/// ```ignore
/// fn as_raw_html(&self) -> String 
/// {
///     format!
///     (
///         "<{0} {2}> {1} </{0}>",
///         Self::TAG,
///         self.inner_html(),
///         self.attributes()
///             .iter()
///             .map(Attribute::to_string)
///             .collect::<Vec<String>>()
///             .join(" ")
///     )
/// }
/// ```
/// Finally, the following may be called to get a more structured js_sys type:
/// ```ignore
/// fn as_element(&self) -> Option<web_sys::Element>;
/// ```
pub trait Htmlify
{
    /// Sets the HTML tag associated when converting this to an element.
    fn tag(&self) -> &'static str { "" }
    /// Sets the attributes to include when converting this to an element.
    fn attributes(&self) -> Vec<Attribute>
    {
        vec!()
    }
    /// Get the inner HTML
    fn inner_html(&self) -> Vec<Box<dyn Htmlify>>
    {
        vec!()
    }
    /// Stringifies the inner HTML
    fn inner_html_as_string(&self) -> String
    {
        self.inner_html().iter().map(|e| e.as_raw_html()).collect::<Vec<String>>().join("")
    }
    /// Convert this to a raw string of HTML
    fn as_raw_html(&self) -> String 
    {
        format!
        (
            "<{0} {2}> {1} </{0}>",
            self.tag(),
            self.inner_html_as_string(),
            self.attributes()
                .iter()
                .map(Attribute::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
    /// Convert this into a [web_sys::Element]
    fn as_element(&self) -> Option<web_sys::Element>
    {
        let document = web_sys::window().and_then(|win| win.document())?;
        let element = document.create_element(self.tag()).ok()?;
        for attribute in self.attributes()
        {
            element.set_attribute(attribute.name.borrow(), attribute.value.borrow()).ok()?;
        }
        for inner in self.inner_html()
        {
            if let "__STRING_MARKER" = inner.tag()
            {
                element.append_with_str_1(&inner.as_raw_html()).ok()?;
            }
            else
            {
                element.append_with_node_1(inner.as_element()?.as_ref()).ok()?;
            }
        }
        Some(element)
    }
}

/// HTML Attribute wrapper, a simple key-value string pair
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Attribute
{
    pub name: Cow<'static, str>, 
    pub value: Cow<'static, str>
}
impl Attribute
{
    pub fn new(name: impl Into<Cow<'static, str>>, value: impl Into<Cow<'static, str>>) -> Self
    {
        Attribute { name: name.into(), value: value.into() }
    }
}
impl Display for Attribute
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        let value = self.value.to_string();
        if value.is_empty()
        {
            write!(f, "{}", self.name)
        }
        else
        {
            write!(f, "{}=\"{}\"", self.name, self.value)
        }
    }
}

impl Htmlify for Scene
{
    fn tag(&self) -> &'static str { "a-scene" }
    fn attributes(&self) -> Vec<Attribute>
    {
        self.components().iter()
            .map(Attribute::from)
            .chain(self.attributes().iter().map(Attribute::clone))
            .collect()
    }
    fn inner_html(&self) -> Vec<Box<dyn Htmlify>>
    {
        std::iter::once(Box::new(self.assets().clone()) as  Box<dyn Htmlify>)
            .chain(self.children().iter().map(|child| Box::new(child.clone()) as Box<dyn Htmlify>))
            .collect()
    }
}

impl Htmlify for Entity
{
    fn tag(&self) -> &'static str { "a-entity" }
    fn attributes(&self) -> Vec<Attribute>
    {
        self.components().iter()
            .map(Attribute::from)
            .chain(self.attributes().iter().map(Attribute::clone))
            .collect()
    }
    fn as_raw_html(&self) -> String 
    {
        format!
        (
            "<{0} {2}> {1} </{0}>",
            self.tag(),
            self.inner_html_as_string(),
            Htmlify::attributes(self)
                .iter()
                .map(Attribute::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
    fn inner_html(&self) -> Vec<Box<dyn Htmlify>>
    {
        self.children()
            .iter()
            .map(|child| Box::new(child.clone()) as Box<dyn Htmlify>)
            .collect()
    }
}

impl Htmlify for Assets
{
    fn tag(&self) -> &'static str { "a-assets" }
    fn attributes(&self) -> Vec<Attribute>
    {
        if self.timeout_ms <= 0
        {
            vec!()
        }
        else
        {
            vec!(Attribute::new("timeout", self.timeout_ms.to_string()))
        }
    }
    fn inner_html(&self) -> Vec<Box<dyn Htmlify>>
    {
        self.assets.iter().map(|asset| Box::new(asset.clone()) as Box<dyn Htmlify>).collect()
    }
}

impl Htmlify for Asset
{
    fn tag(&self) -> &'static str 
    { 
        match self
        {
            Asset::Item(i) => i.tag(),
            Asset::Image(i) => i.tag(),
            Asset::Video(i) => i.tag(),
            Asset::Audio(i) => i.tag(),
            Asset::Mixin(i) => i.tag(),
        }
    }
    fn attributes(&self) -> Vec<Attribute>
    {
        self.into()
    }
    fn as_raw_html(&self) -> String 
    {
        format!
        (
            "<{0} {2}> {1} </{0}>",
            self.tag(),
            self.inner_html_as_string(),
            self.attributes()
                .iter()
                .map(Attribute::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Htmlify for AssetItem
{
    fn tag(&self) -> &'static str { "a-asset-item" }
    fn attributes(&self) -> Vec<Attribute>
    {
        vec!
        (
            Attribute::new("id", self.id.clone()), 
            Attribute::new("src", self.src.clone()), 
        )
    }
}

impl Htmlify for Image
{
    fn tag(&self) -> &'static str { "img" }
    fn attributes(&self) -> Vec<Attribute>
    {
        vec!
        (
            Attribute::new("id", self.id.clone()), 
            Attribute::new("src", self.src.clone()), 
        )
    }
}

impl Htmlify for Video
{
    fn tag(&self) -> &'static str { "video" }
    fn attributes(&self) -> Vec<Attribute>
    {
        let mut attrs = vec!
        (
            Attribute::new("id", self.id.clone()), 
            Attribute::new("src", self.src.clone()), 
            Attribute::new("preload", self.preload.to_string())
        );
        if self.autoplay
        {
            attrs.push(Attribute::new("autoplay", "true"))
        }
        attrs
    }
}

impl Htmlify for Audio
{
    fn tag(&self) -> &'static str { "audio" }
    fn attributes(&self) -> Vec<Attribute>
    {
        let mut attrs = vec!
        (
            Attribute::new("id", self.id.clone()), 
            Attribute::new("src", self.src.clone()), 
            Attribute::new("preload", self.preload.to_string())
        );
        if self.autoplay
        {
            attrs.push(Attribute::new("autoplay", "true"))
        }
        attrs
    }
}

impl Htmlify for Mixin
{
    fn tag(&self) -> &'static str { "a-mixin" }
    fn attributes(&self) -> Vec<Attribute>
    {
        self.components.iter()
            .map(Attribute::from)
            .chain(std::iter::once(Attribute::new("id", self.id.clone())))
            .collect()
    }
}

impl Htmlify for &str
{
    /// Used as a sentinal value
    fn tag(&self) -> &'static str { "__STRING_MARKER" }
    fn as_raw_html(&self) -> String { self.to_string() }
    fn as_element(&self) -> Option<web_sys::Element> { None }
}