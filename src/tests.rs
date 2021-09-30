use std::{borrow::Cow, collections::HashMap};
use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use crate::{*, component::{Position, Rotation}};

// Run with: wasm-pack test --firefox --headless
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen]
extern "C" 
{
    fn setTimeout(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
}

#[wasm_bindgen_test]
async fn test_register_component() 
{
    crate::init_aframe().await;

    let mut schema = HashMap::new();
    schema.insert("updateFreq", ComponentProperty::number(Some(500.0)));

    let fps = component_def!
    {
        dependencies: "text";
        schema: schema,
        init: js!(this.timeOfLastUpdate = 0.0;),
        tick: js!
        (time, delta =>>
            if (time - this.timeOfLastUpdate > this.data.updateFreq)
            {
                this.timeOfLastUpdate = time;
                this.el.setAttribute("text", "value", "FPS: " + (1000 / delta).toFixed(2));
            }
        ),
    };
    unsafe 
    {
        fps.register("fps");
    }
    console_log!("Registered fps component.");
}

#[wasm_bindgen_test]
async fn test_register_geometry() 
{
    crate::init_aframe().await;

    let mut schema = HashMap::new();
    schema.insert("depth", GeometryProperty::new(AframeVal::Float(1.0), Some(AframeVal::Float(0.0)), None, None));
    schema.insert("height", GeometryProperty::new(AframeVal::Float(1.0), Some(AframeVal::Float(0.0)), None, None));
    schema.insert("width", GeometryProperty::new(AframeVal::Float(1.0), Some(AframeVal::Float(0.0)), None, None));
    schema.insert("segmentsHeight", GeometryProperty::new(AframeVal::Int(1), Some(AframeVal::Int(1)), Some(AframeVal::Int(20)), Some("int")));
    schema.insert("segmentsWidth", GeometryProperty::new(AframeVal::Int(1), Some(AframeVal::Int(1)), Some(AframeVal::Int(20)), Some("int")));
    schema.insert("segmentsDepth", GeometryProperty::new(AframeVal::Int(1), Some(AframeVal::Int(1)), Some(AframeVal::Int(20)), Some("int")));

    let newbox = geometry_def!
    {
        schema: schema,
        init: js!(data =>> this.geometry = new THREE.BoxGeometry(data.width, data.height, data.depth);)
    };
    unsafe 
    {
        newbox.register("newbox");
    }
    console_log!("Registered newbox geometry.");
}

#[test]
fn entity_cmp()
{
    use crate::entity::*;

    let child_ent = Entity::new
    (
        vec!(Attribute::new(Cow::Borrowed("child_attr"), Cow::Borrowed("child_val"))), 
        vec!((Cow::Borrowed("child_cmp"), Box::new(component!(Rotation, x: 9.0, y: 55.0, z: 44.0)))), 
        vec!()
    );

    let ent = Entity::new
    (
        vec!(Attribute::new(Cow::Borrowed("test_attr"), Cow::Borrowed("test_val"))), 
        vec!((Cow::Borrowed("test_cmp"), Box::new(component!(Position, x: 1.0, y: 2.0, z: 3.0)))), 
        vec!(child_ent.clone())
    );

    assert_eq!(ent, Entity::new
    (
        vec!(Attribute::new(Cow::Borrowed("test_attr"), Cow::Borrowed("test_val"))), 
        vec!((Cow::Borrowed("test_cmp"), Box::new(component!(Position, x: 1.0, y: 2.0, z: 3.0)))), 
        vec!(child_ent)
    ));
    assert_ne!(ent, Entity::new
    (
        vec!(Attribute::new(Cow::Borrowed("test_attr_2"), Cow::Borrowed("test_val_2"))), 
        vec!((Cow::Borrowed("test_cmp_2"), Box::new(component!(Position, x: 1.0, y: 2.0, z: 3.0)))), 
        vec!()
    ));
}