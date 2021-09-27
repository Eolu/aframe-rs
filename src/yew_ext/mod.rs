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