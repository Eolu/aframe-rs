use std::{borrow::Cow, collections::HashMap, sync::atomic::{AtomicBool, Ordering}};
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

async fn init_aframe_tests()
{
    // Once this has happened, don't do it again
    static INIT: AtomicBool = AtomicBool::new(false);
    if !INIT.load(Ordering::Relaxed)
    {
        crate::init_aframe().await.unwrap();
        INIT.store(true, Ordering::Relaxed);
    }
}

#[wasm_bindgen_test]
async fn test_register_component() 
{
    init_aframe_tests().await;

    let mut schema = HashMap::new();
    schema.insert("updateFreq", AframeProperty::number(Some(500.0)));

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
    init_aframe_tests().await;

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

#[wasm_bindgen_test]
async fn test_register_system() 
{
    init_aframe_tests().await;

    let mut schema = HashMap::new();
    schema.insert("some_float", AframeProperty::number(None));
    schema.insert("some_text", AframeProperty::string(Some(Cow::Borrowed("init"))));

    let my_sys = system_def!
    {
        schema: schema,
        init: js!
        (
            this.data.some_float = 1.0; 
            this.data.some_text = "I'm a bit of text";
        ),
        pause: js!(this.data.some_text = "paused!";),
        play: js!(this.data.some_text = "playing!";),
        tick: js!
        (time, delta =>>
            this.data.some_float = this.data.some_float + 1.0;
        ),
        properties:
            reset_me: js!(this.data.some_float = 0.0;)
    };
    unsafe 
    {
        my_sys.register("my-sys");
    }
    console_log!("Registered my-sys system.");
}

#[wasm_bindgen_test]
async fn test_globals_access() 
{
    init_aframe_tests().await;

    console_log!("THREE global: {:?}", sys::three_js().expect("THREE global access failed!"));
    let _components = sys::components().expect("components access failed!");
    // console_log!("registered components: {:?}", components);
    let _geometries = sys::geometries().expect("geometries access failed!");
    // console_log!("registered geometries: {:?}", geometries);
    console_log!("registered primitives: {:?}", sys::primitives().expect("primitives access failed!"));
    let _shaders = sys::shaders().expect("shaders access failed!");
    // console_log!("registered shaders: {:?}", shaders);
    console_log!("registered systems: {:?}", sys::systems().expect("systems access failed!"));
    console_log!("utils: {:?}", sys::utils().expect("utils access failed!"));
    console_log!("device: {:?}", sys::device().expect("device access failed!"));
    // console_log!("check_headset_connected: {:?}", sys::checkHeadsetConnected());
    // console_log!("is_gear_vr: {}", sys::isGearVR());
    // console_log!("is_oculus_go: {:?}", sys::isOculusGo());
    // console_log!("is_mobile: {:?}", sys::isMobile());
    console_log!("Aframe version: {:?}", sys::version().expect("version access failed!"));
    console_log!("Globals access test complete.");
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