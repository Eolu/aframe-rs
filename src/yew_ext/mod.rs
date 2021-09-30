//! Support for yew. See the below example:
//! ```rust,ignore
//! static INIT: AtomicBool = AtomicBool::new(false);
//!
//! #[derive(Clone, PartialEq, Properties)]
//! pub struct AframeProps
//! {
//!     scene: aframe::Scene
//! }
//!
//! pub struct Aframe
//! {
//!     props: AframeProps
//! }
//!
//! impl crate::utils::Component for Aframe 
//! {
//!     type Message = Msg;
//!     type Properties = AframeProps;
//!
//!     fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self 
//!     {
//!         // Register aframe stuff first time only
//!         if !INIT.load(Ordering::Relaxed)
//!         {
//!             unsafe 
//!             {
//!                // Code in this block registers shaders, components, and primitives with aframe
//!                 shaders::register_shaders(); 
//!                 component::register_components();
//!                 primitive::register_primitives();
//!             }
//!             INIT.store(true, Ordering::Relaxed)
//!         }
//!         Self 
//!         { 
//!             props
//!         }
//!     }
//!
//!     fn update(&mut self, _: Self::Message) -> ShouldRender 
//!     {
//!         true
//!     }
//!
//!     fn change(&mut self, _props: Self::Properties) -> ShouldRender 
//!     {
//!         false
//!     }
//!
//!     fn view(&self) -> Html 
//!     {
//!         (&self.props.scene).into()
//!     }
//! }
//! ```

use yew::Html;
use yew::html;
use crate::Entity;
use crate::Scene;

mod raw_html;

impl From<&Scene> for Html
{
    fn from(scene: &Scene) -> Self 
    {
        html!{{raw_html::RawHtml::from(scene)}}
    }
}

impl From<&Entity> for Html
{
    fn from(entity: &Entity) -> Self 
    {
        html!{{raw_html::RawHtml::from(entity)}}
    }
}