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
/// to the document header and waiting for the script onload event. 
/// Current Aframe version: 1.2.0
#[cfg(feature = "init")]
pub async fn init_aframe() -> Result<(), InitError>
{
    const LINK: &'static str = "https://aframe.io/releases/1.2.0/aframe.min.js";
    
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use std::sync::{Arc, Mutex};
    use async_lock::Barrier;
    use futures::executor::block_on;

    let result: Arc<Mutex<Result<(), InitError>>> = Arc::new(Mutex::new(Err(InitError)));
    let barrier = Arc::new(Barrier::new(2));

    let result_outer = result.clone();
    let barrier_inner = barrier.clone();

    // Append Aframe to document
    let document = web_sys::window()
        .and_then(|win| win.document())
        .ok_or(InitError)?;
    let head = document.head()
        .ok_or(InitError)?;
    let script_element = document.create_element("script")
        .map_err(|_| InitError)?;
    let script_element = script_element.dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| InitError)?;
    head.append_child(&script_element)
        .map_err(|_| InitError)?;
    let closure = 
    {
        Closure::once(Box::new(move || 
        {
            *result.lock().unwrap() = Ok(());
            drop(result);
            block_on(barrier_inner.wait());
        }) as Box<dyn FnOnce()>)
    };
    script_element.set_onload(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    script_element.set_attribute("src", LINK)
        .map_err(|_| InitError)?;

    barrier.wait().await;
    Arc::try_unwrap(result_outer)
        .map_err(|_| InitError)
        .and_then(|mutex| mutex.into_inner().map_err(|_| InitError))
        .and_then(|result| result)
}

#[cfg(feature = "init")]
#[derive(Debug, Clone, Copy)]
pub struct InitError;

#[cfg(feature = "init")]
impl std::fmt::Display for InitError 
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        write!(f, "Failed to initialize")
    }
}

#[cfg(feature = "init")]
impl std::error::Error for InitError 
{
    fn description(&self) -> &str 
    {
        "Failed to initialize"
    }
}