use std::borrow::Cow;
use web_sys::Node;
use yew::virtual_dom::VNode;
use yew::{prelude::*, Component, ComponentLink, Html, ShouldRender};
use crate::{Htmlify, Attribute};

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props
{
    pub tag: String,
    pub attributes: Vec<Attribute>,
    pub html: Cow<'static, str>
}

pub struct RawHtml
{
    props: Props,
}

impl RawHtml
{
    pub fn from<T: Htmlify>(t: &T) -> Html
    {
        html!
        {
            <RawHtml tag=t.tag().to_string() attributes={t.attributes()} html={t.inner_html_as_string()} />
        }
    }
}

impl Component for RawHtml 
{
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self 
    {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender 
    {
        unreachable!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender 
    {
        if self.props != props 
        {
            self.props = props;
            true
        } 
        else 
        {
            false
        }
    }

    fn view(&self) -> Html 
    {
        VNode::VRef(Node::from
        ({
            let element = web_sys::window().unwrap().document().unwrap().create_element(&self.props.tag).unwrap();
            for attr in self.props.attributes.iter()
            {
                element.set_attribute(&attr.name, &attr.value).unwrap();
            }
            element.set_inner_html(&self.props.html);
            element
        }))
    }
}