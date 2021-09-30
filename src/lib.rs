#![doc = include_str!("../README.md")]

#[cfg(test)]
pub mod tests;

pub mod sys;
pub mod shader;
pub mod component;
pub mod utils;
pub mod entity;
pub mod scene;
pub mod assets;
#[cfg(feature = "yew-support")]
pub mod yew_ext;

pub use shader::*;
pub use component::*;
pub use utils::*;
pub use entity::*;
pub use scene::*;
pub use assets::*;
#[cfg(feature = "yew-support")]
pub use crate::yew_ext::*;

/// Async function which initializes aframe by adding the aframe script tag
/// to the document header and waiting 1 second. 
/// Current Aframe version: 1.2.0
#[cfg(feature = "init")]
pub async fn init_aframe()
{
    const LINK: &'static str = "https://aframe.io/releases/1.2.0/aframe.min.js";
    
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use std::sync::{Once, Arc};
    use async_lock::Barrier;
    use futures::executor::block_on;

    static INIT: Once = Once::new();
    if INIT.is_completed()
    {
        return;
    }

    let barrier = Arc::new(Barrier::new(2));

    INIT.call_once(|| 
    {
        // Append Aframe to document
        if let Some(document) = web_sys::window().and_then(|win| win.document())
        {
            if let Some(head) = document.head()
            {
                if let Ok(script_element) = document.create_element("script")
                {
                    if let Ok(script_element) = script_element.dyn_into::<web_sys::HtmlElement>()
                    {
                        head.append_child(&script_element).unwrap();
                        let closure = 
                        {
                            let barrier = barrier.clone();
                            Closure::wrap(Box::new(move || 
                            {
                                block_on(barrier.wait());
                            }) as Box<dyn FnMut()>)
                        };
                        script_element.set_onload(Some(closure.as_ref().unchecked_ref()));
                        closure.forget();
                        if let Ok(_) = script_element.set_attribute("src", LINK)
                        {
                            // Success!
                        }
                    }
                }
            }
        }
    });

    barrier.wait().await;
}