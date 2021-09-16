use std::{borrow::Cow, fmt::Display};

use crate::{Asset, AssetItem, Assets, Audio, Entity, Image, Mixin, Scene, Video};

/// Trait used to generate HTML from aframe objects.
pub trait Htmlify
{
    const TAG: &'static str;
    fn attributes(&self) -> Vec<Attribute>
    {
        vec!()
    }
    fn as_raw_html(&self) -> String 
    {
        format!
        (
            "<{0} {2}> {1} </{0}>",
            Self::TAG,
            self.inner_html(),
            self.attributes()
                .iter()
                .map(Attribute::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
    fn inner_html(&self) -> Cow<'static, str>
    {
        Cow::Borrowed("")
    }
}

/// HTML Attribute wrapper
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
    const TAG: &'static str = "a-scene";
    fn attributes(&self) -> Vec<Attribute>
    {
        self.components().iter()
            .map(Attribute::from)
            .chain(self.attributes().iter().map(Attribute::clone))
            .collect()
    }
    fn inner_html(&self) -> Cow<'static, str>
    {
        std::iter::once(self.assets().as_raw_html())
            .chain(self.children()
                .iter()
                .map(|ent| ent.as_raw_html()))
            .collect::<Vec<String>>()
            .join("")
            .into()
    }
}

impl Htmlify for Entity
{
    const TAG: &'static str = "a-entity";
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
            self.inner_html(),
            Htmlify::attributes(self)
                .iter()
                .map(Attribute::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
    fn inner_html(&self) -> Cow<'static, str>
    {
        self.children().iter().map(|ent| ent.as_raw_html()).collect::<Vec<String>>().join("").into()
    }
}

impl Htmlify for Assets
{
    const TAG: &'static str = "a-assets";
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
    fn inner_html(&self) -> Cow<'static, str>
    {
        self.assets.iter().map(|a| a.as_raw_html()).collect::<Vec<String>>().join("").into()
    }
}

impl Htmlify for Asset
{
    /// This tag is unused
    const TAG: &'static str = "a-asset";
    fn attributes(&self) -> Vec<Attribute>
    {
        self.into()
    }
    fn as_raw_html(&self) -> String 
    {
        format!
        (
            "<{0} {2}> {1} </{0}>",
            match self
            {
                Self::Item(_) => AssetItem::TAG,
                Self::Image(_) => Image::TAG,
                Self::Video(_) => Video::TAG,
                Self::Audio(_) => Audio::TAG,
                Self::Mixin(_) => Mixin::TAG,
            },
            self.inner_html(),
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
    const TAG: &'static str = "a-asset-item";
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
    const TAG: &'static str = "img";
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
    const TAG: &'static str = "video";
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
    const TAG: &'static str = "audio";
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
    const TAG: &'static str = "a-mixin";
    fn attributes(&self) -> Vec<Attribute>
    {
        self.components.iter()
            .map(Attribute::from)
            .chain(std::iter::once(Attribute::new("id", self.id.clone())))
            .collect()
    }
}