//! Module that implements the Htmlify trait for items in this crate.

use std::borrow::Cow;
use crate::{Asset, AssetItem, Assets, Audio, Entity, Image, Mixin, Scene, Video};
use htmlify::*;

impl Htmlify for Scene
{
    fn tag(&self) -> Cow<'static, str> { Cow::Borrowed("a-scene") }
    fn attributes(&self) -> Vec<Attribute>
    {
        self.components().iter()
            .map(crate::component::cmp_to_attr)
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
    fn tag(&self) -> Cow<'static, str> { Cow::Borrowed("a-entity") }
    fn attributes(&self) -> Vec<Attribute>
    {
        self.components().iter()
            .map(crate::component::cmp_to_attr)
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
    fn tag(&self) -> Cow<'static, str> { Cow::Borrowed("a-assets") }
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
    fn tag(&self) -> Cow<'static, str>
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
    fn tag(&self) -> Cow<'static, str> { Cow::Borrowed("a-asset-item") }
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
    fn tag(&self) -> Cow<'static, str> { Cow::Borrowed("img") }
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
    fn tag(&self) -> Cow<'static, str> { Cow::Borrowed("video") }
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
    fn tag(&self) -> Cow<'static, str> { Cow::Borrowed("audio") }
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
    fn tag(&self) -> Cow<'static, str> { Cow::Borrowed("a-mixin") }
    fn attributes(&self) -> Vec<Attribute>
    {
        self.components.iter()
            .map(crate::component::cmp_to_attr)
            .chain(std::iter::once(Attribute::new("id", self.id.clone())))
            .collect()
    }
}
