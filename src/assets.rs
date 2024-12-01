//! Module for instantion of assets and mixins.

use std::borrow::Cow;
use crate::{Attribute, Component, ComponentVec, simple_enum};

/// Constructs an Assets object for use in a scene.
/// Example:
/// ```ignore
// assets!
/// {
///     Image::new("ramen", "/pics/ramen.png"),
///     Image::new("noise", "/pics/noise.bmp"),
///     Audio::new("ambient_music", "/audio/Ephemeral/Coin Machine.mp3"),
///     mixin!
///     {
///         "intersect_ray", 
///         ("raycaster", component!
///         {
///             RayCaster,
///             objects: List(Cow::Borrowed(&[Cow::Borrowed("#ramen-cube, #water")]))
///         })
///     }
/// },
/// ```
#[macro_export]
macro_rules! assets
{
    (timeout: $timeout:expr, $($asset:expr),*) => 
    {
        Assets::new($timeout, vec!($($asset),*))
    };
    ($($asset:expr),*) => 
    {
        assets!(timeout: 0, $($asset),*)
    }
}

/// Constructs an `AssetItem::Mixin` foir use in an `Assets` struct. 
/// See the [assets!](assets) macro for an example/
#[macro_export]
macro_rules! mixin
{
    ($id:expr, $(($cmp_id:literal, $cmp_value:expr)),*) => 
    {
        Mixin::new($id, $crate::components_vec!
        {
            $(($cmp_id, $cmp_value)),*
        })
    }
}

/// A collection of assets for use in a scene
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Assets
{
    pub(crate) timeout_ms: u32,
    pub(crate) assets: Vec<Asset>
}
impl Assets
{
    pub fn new(timeout_ms: u32, assets: Vec<Asset>) -> Self
    {
        Self { timeout_ms, assets }
    }
}

/// An individual asset or mixin
#[derive(Debug, Clone, PartialEq)]
pub enum Asset
{
    Item(AssetItem),
    Image(Image),
    Video(Video),
    Audio(Audio),
    Mixin(Mixin)
}
impl Asset
{
    pub fn src(&self) -> &Cow<'static, str>
    {
        match self
        {
            Asset::Item(item) => &item.src,
            Asset::Image(image) => &image.src,
            Asset::Video(video) => &video.src,
            Asset::Audio(audio) => &audio.src,
            Asset::Mixin(_) => &Cow::Borrowed(""),
        }
    }
}
impl From<&Asset> for Vec<Attribute>
{
    fn from(asset: &Asset) -> Self 
    {
        match asset
        {
            Asset::Item(item) => item.into(),
            Asset::Image(image) => image.into(),
            Asset::Video(video) => video.into(),
            Asset::Audio(audio) => audio.into(),
            Asset::Mixin(mixin) => mixin.into(),
        }
    }
}

/// An untyped asset
#[derive(Debug, Clone, PartialEq)]
pub struct AssetItem
{
    pub(crate) id: Cow<'static, str>,
    pub(crate) src: Cow<'static, str>
}
impl From<&AssetItem> for Vec<Attribute>
{
    fn from(item: &AssetItem) -> Self 
    {
        vec!(Attribute::new("id", item.id.clone()), Attribute::new("src", item.src.clone()))
    }
}
impl AssetItem
{
    pub fn new(id: impl Into<Cow<'static, str>>, src: impl Into<Cow<'static, str>>) -> Asset
    {
        Asset::Item(Self { id: id.into(), src: src.into() })
    }
}

/// An image asset
/// TODO: Support the full HTML img tag
#[derive(Debug, Clone, PartialEq)]
pub struct Image
{
    pub(crate) id: Cow<'static, str>,
    pub(crate) src: Cow<'static, str>
}
impl From<&Image> for Vec<Attribute>
{
    fn from(image: &Image) -> Self 
    {
        vec!(Attribute::new("id", image.id.clone()), Attribute::new("src", image.src.clone()))
    }
}
impl Image
{
    pub fn new(id: impl Into<Cow<'static, str>>, src: impl Into<Cow<'static, str>>) -> Asset
    {
        Asset::Image(Self { id: id.into(), src: src.into() })
    }
}

/// A video asset
#[derive(Debug, Clone, PartialEq)]
pub struct Video
{
    pub(crate) id: Cow<'static, str>,
    pub(crate) src: Cow<'static, str>,
    pub(crate) autoplay: bool,
    pub(crate) preload: Preload
}
impl From<&Video> for Vec<Attribute>
{
    fn from(video: &Video) -> Self 
    {
        vec!
        (
            Attribute::new("id", video.id.clone()), 
            Attribute::new("src", video.src.clone()), 
            Attribute::new("autoplay", video.autoplay.to_string()), 
            Attribute::new("preload", video.preload.to_string())
        )
    }
}
impl Video
{
    pub fn new(id: impl Into<Cow<'static, str>>, src: impl Into<Cow<'static, str>>) -> Asset
    {
        Asset::Video(Self { id: id.into(), src: src.into(), autoplay: false, preload: Preload::None })
    }
    pub fn autoplay(id: impl Into<Cow<'static, str>>, src: impl Into<Cow<'static, str>>) -> Asset
    {
        Asset::Video(Self { id: id.into(), src: src.into(), autoplay: true, preload: Preload::None })
    }
    pub fn preload(id: impl Into<Cow<'static, str>>, src: impl Into<Cow<'static, str>>, preload: Preload) -> Asset
    {
        Asset::Video(Self { id: id.into(), src: src.into(), autoplay: false, preload })
    }
}

/// An audio asset
#[derive(Debug, Clone, PartialEq)]
pub struct Audio
{
    pub(crate) id: Cow<'static, str>,
    pub(crate) src: Cow<'static, str>,
    pub(crate) autoplay: bool,
    pub(crate) preload: Preload
}
impl From<&Audio> for Vec<Attribute>
{
    fn from(audio: &Audio) -> Self 
    {
        let mut vec = vec!
        (
            Attribute::new("id", audio.id.clone()), 
            Attribute::new("src", audio.src.clone()), 
        );
        if audio.autoplay
        {
            vec.push(Attribute::new("autoplay", "true"));
        }
        match audio.preload
        {
            Preload::None => (),
            preload => vec.push(Attribute::new("preload", preload.to_string()))
        }
        vec
    }
}
impl Audio
{
    pub fn new(id: impl Into<Cow<'static, str>>, src: impl Into<Cow<'static, str>>) -> Asset
    {
        Asset::Audio(Self { id: id.into(), src: src.into(), autoplay: false, preload: Preload::None })
    }
    pub fn autoplay(id: impl Into<Cow<'static, str>>, src: impl Into<Cow<'static, str>>) -> Asset
    {
        Asset::Audio(Self { id: id.into(), src: src.into(), autoplay: true, preload: Preload::None })
    }
    pub fn preload(id: impl Into<Cow<'static, str>>, src: impl Into<Cow<'static, str>>, preload: Preload) -> Asset
    {
        Asset::Audio(Self { id: id.into(), src: src.into(), autoplay: false, preload })
    }
}

/// [mixins](https://aframe.io/docs/1.6.0/core/mixins.html)
#[derive(Debug, Clone, PartialEq)]
pub struct Mixin
{
    pub(crate) id: Cow<'static, str>,
    pub(crate) components: ComponentVec
}
impl From<&Mixin> for Vec<Attribute>
{
    fn from(mixin: &Mixin) -> Self 
    {
        std::iter::once(Attribute::new("id", mixin.id.clone()))
            .chain(mixin.components.iter().map(crate::component::cmp_to_attr))
            .collect()
    }
}
impl Mixin
{
    pub fn new(id: impl Into<Cow<'static, str>>, components: Vec<(Cow<'static, str>, Box<dyn Component>)>) -> Asset
    {
        Asset::Mixin(Self { id: id.into(), components: ComponentVec(components) })
    }
}

simple_enum!
(
    /// Preload behavior for audio and video assets. Ignored if autoplay is set
    Preload, 
    Auto => "auto", 
    Metadata => "metadata", 
    None => "none"
);