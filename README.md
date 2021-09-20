# aframe-rs

This is an [Aframe](https://aframe.io/) library for rust. It's still fairly experimental and a lot might change. I started writing this for a bit of fun to see if I could play with aframe from inside a [yew](https://yew.rs/) app. It started getting pretty large so I decided to abstract away all the yew-specific stuff and start making a library on its own. There's still a bunch missing and a bunch to do here, but what IS there is functional.

# API

## Components

### High-level API

The high-level API for components is composed of 5 macros:
- `component_def!`: The `component_def!` macro allows the definition of new components. Its signature is as follows:
    ```rust
    component_def!
    (
        $(dependencies: $($deps:expr),*;)? 
        $(schema: $schema:expr,)?
        $(multiple: $mult:expr,)? 
        $(init: $init:expr,)?
        $(update: $update:expr,)?
        $(tick: $tick:expr,)?
        $(tock: $tock:expr,)?
        $(remove: $remove:expr,)?
        $(pause: $pause:expr,)?
        $(play: $play:expr,)?
        $(update_schema: $update_schema:expr,)?
    )
    ```
    All parameteres are optional, although the order must be exactly as shown. `dependencies` should be a comma-separated list of strings followed by a semicolon. `schema` should be a HashMap with string keys and `Property` values. `multiple` is a boolean value. The rest are strings containing javascript code. A `js!` macro is provided to allow inline javascript code to be included in the Rust code (See the docs for the `js!` macro for caveats and limitations). Here's an example:
    ```rust
    let spin = component_def!
    (
        dependencies: "rotation";
        schema: hashmap!
        {
            "radiansPerMillisecond" => Property::number(0.00116355333.into()),
            "speedMult" => Property::number(1.0.into()),
            "axis" => Property::string(Cow::Borrowed("x").into()),
            "autoplay" => Property::boolean(true.into())
        },
        multiple: true,
        init: js!
        (
            this.radians = Math.PI * 2; 
            this.initalRotation = this.el.object3D.rotation.clone();
        ),
        update: js!(oldData =>> this.rotation = this.el.object3D.rotation;),
        tick: js!
        (time, delta =>>
            if (this.data.autoplay)
            {
                var amount = this.data.radiansPerMillisecond * delta * this.data.speedMult;
                if (this.data.axis.includes('x'))
                    this.rotation.x = (this.rotation.x + amount) % this.radians;
                if (this.data.axis.includes('y'))
                    this.rotation.y = (this.rotation.y + amount) % this.radians;
                if (this.data.axis.includes('z'))
                    this.rotation.z = (this.rotation.z + amount) % this.radians;
            }
        ),
        remove: js!(this.rotation.copy(this.initialRotation);),
        pause: js!(this.data.autoplay = false;),
        play: js!(this.data.autoplay = true;),
    );
    ```
    This example defines a component which causes an entity's rotation to be updated on a loop, creating a spinning effect. This can then be registered in aframe via the following:
    ```rust
    unsafe
    {
        spin.register("spin");
    }
    ```
    We now have access to the `spin` component with the 4 properties defined in `schema`.
- `component_struct!`: While `component_def!` creates a component that Aframe can access from its own runtime, the `component_struct!` macro creates a Rust struct that mimics the internal details of that Aframe component. Component structs are already provided for Aframe's built-in components (WIP: not all components are defined yet. Once all aframe components are defined, calling `component_struct!` should only be necessary for locally-defined components. Once all components from Aframe have been defined, `component_def!` and `component_struct!` may be merged into a single macro to do the heavy-lifting of both at once). Its signature is as follows:
    ```rust
    macro_rules! component_struct
    {
        ($name:ident $(, $field:ident: $field_name:literal $ty:ty = $default:expr)*) => {...};
        ($name:ident $(:$alt:ident)? $fmt:expr $(, $field:ident: $field_name:literal $ty:ty = $default:expr)*) => {...}
    ```
    An example is as follows:
    ```rust
    component_struct!
    (Sound, 
        src: "src" Cow<'static, str> = Cow::Borrowed(""),
        autoplay: "autoplay" bool = false,
        positional: "positional" bool = true,
        volume: "volume" f32 = 1.0,
        looping: "loop" bool = false,
        additional_field_map: "" AdditionalFieldMap = AdditionalFieldMap { /* ... */ }
    );
    ```
- `component!`: The `component!` macro is a simple syntax-sugar for creating a specific instance of a component_struct. As every component_struct contains default values, this macro allows instantiation of a component without redundant re-definition of those default values. For example: `component!{component::Camera}` will create a `camera` component with all its fields set to default values, whereas `component!{component::Camera, active = false}` will create a `camera` component with all its fields set to default values except the `active` field.
- `simple_enum!` - The `simple_enum!` macro defines an enum in which each variant maps to a single string. This can be combined with `component_def!` to crate fields with a limited number of possiblities. For example:
    ```rust
    simple_enum!
    (Autoplay, 
        Null => "null", 
        True => "true", 
        False => "false"
    );
    component_struct!
    (Animation,
        // ...
        autoplay: "autoplay" Autoplay = Autoplay::Null,
        // ...
    );
    ```
- `complex_enum!` - The `complex_enum!` macro defines an enum in which each variant maps to an arbitrary number of fields which will themselves be flattened into fields of the component itself. For example:
    ```rust
    complex_enum!
    (AnimationLoop, 
        Amount "{}" => { looping: u32 },
        Forever "true" => {}
    );
    component_struct!
    (Animation,
        // ...
        looping: "loop" AnimationLoop = AnimationLoop::Amount{looping: 0},
        // ...
    );
    ```
    Another example is as follows:
    ```rust
    complex_enum!
    (GeometryPrimitive, 
        Box
        "primitive: box; width: {}; height: {}; depth: {}; segmentsWidth: {}; \
        segmentsHeight: {}; segmentsDepth: {}" => 
        {  
            width: f32,
            height: f32,
            depth: f32,
            segments_width: u32,
            segments_height: u32,
            segments_depth: u32
        },
        Circle
        "primitive: circle; radius: {}; segments: {}; \
        thetaStart: {}; thetaLength: {}" =>
        {
            radius: f32,
            segments: u32,
            theta_start: f32,
            theta_length: f32
        },
        // ...
    );
    component_struct!
    (Geometry,
        primitive: "" GeometryPrimitive = GeometryPrimitive::Box
        {
            width: 1.0,
            height: 1.0,
            depth: 1.0,
            segments_width: 1,
            segments_height: 1,
            segments_depth: 1,
        },
        // ...
    );
    ```

## Low-level API

A `component_struct` is simply a type that implements these 2 traits:

```rust
pub trait Component: Display + std::fmt::Debug + std::any::Any
{
    fn clone(&self) -> Box<dyn Component>;
    fn eq(&self, other: &'static dyn Component) -> bool;
    fn as_map(&self) -> HashMap<Cow<'static, str>, Cow<'static, str>>;
}

pub trait ConstDefault
{
    const DEFAULT: Self;
}
```

As long as `clone` provides a valid clone, `eq` provides a valid equality check, and `as_map` provides a serialization of keys to values that Aframe can understand, and a `DEFAULT` value is provided that can be evaluated at compile time, a struct is a valid component. 

A `component_reg` is slightly more complicated, but details on its low-level API may be added here at a later date.

## Entities

### High-level API

The `entity!` macro defines the high-level API for describing entities, with one form for describing general entities and another for defining specific primitives:

```rust
( 
    $(attributes: $(($attr_id:literal, $attr_value:expr)),*)? $(,)?
    $(components: $(($cmp_id:literal, $cmp_value:expr)),*)? $(,)? 
    $(children: $($child:expr),*)? 
)
```
and
```rust
( 
    primitive: $name:literal,
    $(attributes: $(($attr_id:literal, $attr_value:expr)),*)? $(,)?
    $(components: $(($cmp_id:literal, $cmp_value:expr)),*)? $(,)? 
    $(children: $($child:expr),*)? 
)
```
respectively.

Here's an example of a general entity definition:
```rust

entity!
{
    attributes: ("id", "cube-rig"),
    components: 
    ("position", component::Position{x: 0.0, y: 2.5, z: -2.0}),
    ("sound", component!
    {
        component::Sound,
        src: Cow::Borrowed("#ambient_music"), 
        volume: 0.5
    }),
    ("play-sound-on-event", component!
    {
        component::PlaySoundOnEvent,
        mode: component::PlaySoundOnEventMode::ToggleStop, 
        event: Cow::Borrowed("click")
    }),
    ("light", component!
    {
        component::Light,
        light_type: component::LightType::Point
        {
            decay: 1.0,
            distance: 50.0,
            shadow: component::OptionalLocalShadow::NoCast{},
        }, 
        intensity: 0.0
    }),
    ("animation__mouseenter", component!
    {
        component::Animation,
        property: Cow::Borrowed("light.intensity"),
        to: Cow::Borrowed("1.0"),
        start_events: component::List(Cow::Borrowed(&[Cow::Borrowed("mouseenter")])),
        dur: 250
    }),
    ("animation__mouseleave", component!
    {
        component::Animation,
        property: Cow::Borrowed("light.intensity"),
        to: Cow::Borrowed("0.0"),
        start_events: component::List(Cow::Borrowed(&[Cow::Borrowed("mouseleave")])),
        dur: 250
    }),
    children: entity!
    {
        primitive: "ramen-cube",
        attributes: ("id", "ramen-cube"),
        components:
    }
},
```
and here's an example of a primitive definition:
```rust
entity!
{
    primitive: "ramen-cube",
    attributes: ("id", "ramen-cube"),
    components: 
}
```

## Primitives

### High-level API

Primitives can be defined via the following macro signature:

```rust
(
    components: $(($name:expr, $cmp:expr)),*
    mappings: $(($map_name:expr, $map_expr:expr)),*
)
```

Here's an example of a primitive definition and a following registry with Aframe:

```rust
let prim = primitive!
{
    components: 
    ("position", component::Position{ x: 0.0, y: -2.0, z: -1.0 }),
    ("rotation", component::Rotation { x: 0.0, y: 45.0, z: 0.0  }),
    ("spin", component!(super::component::Spin, axis: super::component::Axis::Y)),
    ("geometry", component!(component::Geometry)),
    ("animation__click", component!
    { 
        component::Animation,
        property: Cow::Borrowed("rotation"),
        from: Cow::Borrowed("0 45 0"),
        to: Cow::Borrowed("0 405 0"),
        start_events: component::List(Cow::Borrowed(&[Cow::Borrowed("click")])),
        dur: 900,
        easing: component::Easing::EaseOutCubic
    }),
    ("shadow", component!(component::Shadow)),
    ("material", component!
    {
        component::Material, 
        props: component::MaterialProps(Cow::Owned(vec!((Cow::Borrowed("src"), Cow::Borrowed("#ramen")))))
    })
    mappings: 
    ("src", "material.src"), 
    ("depth", "geometry.depth"), 
    ("height", "geometry.height"), 
    ("width", "geometry.width")
};
unsafe
{
    match prim.register("ramen-cube")
    {
        Ok(_) => (),
        Err(err) => yew::services::ConsoleService::log(&format!("{:?}", err))
    }
}

```

## Shaders

## High-level API

The `Shader` struct provides all the tools necessary to define an Aframe shader. The [maplit](https://docs.rs/maplit/1.0.2/maplit/) crate is recommended for simplifying shader definitions. See below:

```rust
use maplit::*;
use aframe::shader::*;

pub const SIMPLE_VS: &str = include_str!("./SOME_PATH/glsl/simple.vs");
pub const STROBE_FS: &str = include_str!("./SOME_PATH/glsl/strobe.fs");

Shader::new
(
    hashmap!
    {
        "speedMult".into() => Property::number(IsUniform::Yes, 1.0.into()),
        "alpha".into() => Property::number(IsUniform::Yes, 1.0.into()),
        "alpha2".into() => Property::number(IsUniform::Yes, 1.0.into()),
        "color".into() => Property::color(IsUniform::Yes, color::BLACK.into()),
        "color2".into() => Property::color(IsUniform::Yes, color::WHITE.into()),
        "iTime".into() => Property::time(IsUniform::Yes, None)
    }, 
    SIMPLE_VS.into(),
    STROBE_FS.into()
    // Calling `register` will send this data to the AFRAME.registerShader function.
).register("strobe")?;
```

## Low-level API

TODO

## Sys API

The lowest-level calls to Aframe are defined in the `sys` module:

```rust
#[wasm_bindgen]
extern 
{
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerPrimitive(name: &str, definition: JsValue);
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerComponent(name: &str, data: JsValue);
    #[wasm_bindgen(js_namespace = AFRAME)]
    pub fn registerShader(name: &str, data: JsValue);
}
```

Using this should not be necessary for the usage of this crate, but the public APIs have been provided while this crate is still feature-incomplete.

## yew_support feature

The `yew_support` feature adds yew support to this crate. At its core, all this does is implement `From<&Scene> for Html`. This allows you to write a yew component as such:

```rust
static INIT: AtomicBool = AtomicBool::new(false);

#[derive(Clone, PartialEq, Properties)]
pub struct AframeProps
{
    scene: aframe::Scene
}

pub struct Aframe
{
    props: AframeProps
}

impl crate::utils::Component for Aframe 
{
    type Message = Msg;
    type Properties = AframeProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self 
    {
        // Register aframe stuff first time only
        if !INIT.load(Ordering::Relaxed)
        {
            unsafe 
            {
                // Code in this block registers shaders, components, and primitives with aframe
                shaders::register_shaders(); 
                component::register_components();
                primitive::register_primitives();
            }
            INIT.store(true, Ordering::Relaxed)
        }
        Self 
        { 
            props
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender 
    {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender 
    {
        false
    }

    fn view(&self) -> Html 
    {
        (&self.props.scene).into()
    }
}
```

Below is a full definition of how the scene is defined in yew:

```rust
html!
{
    <Aframe scene = 
    { 
        const CURSOR_COLOR: [(Cow<'static, str>, Cow<'static, str>); 1] = 
            [(Cow::Borrowed("color"), Cow::Borrowed("lightblue"))];
        scene!
        {
            // TODO: Some of these attributes are actually components
            attributes: ("inspector", "true"), ("embedded", "true"), ("cursor", "rayOrigin: mouse"),
                        ("mixin", "intersect_ray"), ("crawling-cursor", "target: #mouse-cursor"), 
                        ("style", "min-height: 50px;"),
            assets: assets!
            {
                Image::new("ramen", "/pics/ramen.png"),
                Image::new("noise", "/pics/noise.bmp"),
                Audio::new("ambient_music", "/audio/Ephemeral/Coin Machine.mp3"),
                mixin!
                {
                    "intersect_ray", 
                    ("raycaster", component!
                    {
                        RayCaster,
                        objects: List(Cow::Borrowed(&[Cow::Borrowed("#ramen-cube, #water")]))
                    })
                }
            },
            children: 
            // The mouse cursor
            entity!
            {
                // TODO: Make a constant for the fps & text components
                attributes: ("id", "mouse-cursor"), ("vr-mode-watcher", "true"), 
                            ("restrict-entity", "states: non-vr"),
                components: ("geometry", component!
                {
                    component::Geometry,
                    primitive: component::GeometryPrimitive::Ring
                    {
                        radius_inner: 0.06,
                        radius_outer: 0.2,
                        segments_theta: 32,
                        segments_phi: 8,
                        theta_start: 0.0,
                        theta_length: 360.0
                    }
                }),
                ("material", component!
                {
                    component::Material,
                    props: component::MaterialProps(Cow::Borrowed(&CURSOR_COLOR)),
                    opacity: 0.8
                })
            },
            // The camera rig
            entity!
            {
                attributes: ("id", "rig") /*, ("movement-controls", "true")*/,
                components: 
                ("position", component::Position { x: 0.0, y: 0.0, z: 0.0  }),
                ("geometry", component!
                {
                    component::Geometry,
                    primitive: component::GeometryPrimitive::Ring
                    {
                        radius_inner: 0.06,
                        radius_outer: 0.2,
                        segments_theta: 32,
                        segments_phi: 8,
                        theta_start: 0.0,
                        theta_length: 360.0
                    }
                }),
                ("material", component!
                {
                    component::Material,
                    props: component::MaterialProps(Cow::Borrowed(&CURSOR_COLOR)),
                    opacity: 0.8
                }),
                children: 
                    // The camera
                    entity!
                    {
                        attributes: ("id", "camera"), 
                        components: 
                            ("position", component::Position { x: 0.0, y: 1.8, z: 0.0  }),
                            ("camera", component!(component::Camera)),
                            ("look-controls", component!(component::LookControls))
                    }, 
                    
                    // FPS display
                    entity!
                    {
                        // TODO: Make a constant for the fps & text components
                        attributes: ("id", "fps-display"), ("text", "color: green; value: Text"),
                        components: 
                            ("position", component::Position { x: 0.0, y: 1.5, z: -1.0  }),
                            ("fps", component!(component::Fps))
                    }, 
                    
                    // Hands
                    entity!
                    {
                        // TODO: Some fancier way to add/build mixins
                        // TODO: Make a constant for all these components
                        attributes: ("id", "left-controller"), ("mixin", "intersect_ray"), ("vr-mode-watcher", "true"),
                                    ("restrict-entity", "states: vr"), ("laser-controls", "hand: left"), 
                                    ("crawling-cursor", "target: #vr-cursor"), ("line", "color: red; opacity: 0.75")
                    }, 
                    entity!
                    {
                        // TODO: Some fancier way to add/build mixins
                        // TODO: Make a constant for all these components
                        attributes: ("id", "right-controller"), ("mixin", "intersect_ray"), ("vr-mode-watcher", "true"),
                                    ("restrict-entity", "states: vr"), ("laser-controls", "hand: right"), 
                                    ("crawling-cursor", "target: #vr-cursor"), ("line", "color: red; opacity: 0.75")
                    }, 
    
                    // The vr cursor
                    entity!
                    {
                        // TODO: Make a constant for vr-mode-watcher & restrict-entity
                        attributes: ("id", "vr-cursor"), ("vr-mode-watcher", "true"), ("restrict-entity", "states: vr"),
                        components: ("geometry", component!
                        {
                            component::Geometry,
                            primitive: component::GeometryPrimitive::Ring
                            {
                                radius_inner: 0.06,
                                radius_outer: 0.2,
                                segments_theta: 32,
                                segments_phi: 8,
                                theta_start: 0.0,
                                theta_length: 360.0
                            }
                        }),
                        ("material", component!
                        {
                            component::Material,
                            props: component::MaterialProps(Cow::Borrowed(&CURSOR_COLOR)),
                            opacity: 0.7
                        })
                    }
            },
            entity!
            {
                attributes: ("id", "cube-rig"),
                components: 
                ("position", component::Position{x: 0.0, y: 2.5, z: -2.0}),
                ("sound", component!
                {
                    component::Sound,
                    src: Cow::Borrowed("#ambient_music"), 
                    volume: 0.5
                }),
                ("play-sound-on-event", component!
                {
                    component::PlaySoundOnEvent,
                    mode: component::PlaySoundOnEventMode::ToggleStop, 
                    event: Cow::Borrowed("click")
                }),
                ("light", component!
                {
                    component::Light,
                    light_type: component::LightType::Point
                    {
                        decay: 1.0,
                        distance: 50.0,
                        shadow: component::OptionalLocalShadow::NoCast{},
                    }, 
                    intensity: 0.0
                }),
                ("animation__mouseenter", component!
                {
                    component::Animation,
                    property: Cow::Borrowed("light.intensity"),
                    to: Cow::Borrowed("1.0"),
                    start_events: component::List(Cow::Borrowed(&[Cow::Borrowed("mouseenter")])),
                    dur: 250
                }),
                ("animation__mouseleave", component!
                {
                    component::Animation,
                    property: Cow::Borrowed("light.intensity"),
                    to: Cow::Borrowed("0.0"),
                    start_events: component::List(Cow::Borrowed(&[Cow::Borrowed("mouseleave")])),
                    dur: 250
                }),
                children: entity!
                {
                    primitive: "ramen-cube",
                    attributes: ("id", "ramen-cube"),
                    components: // None
                }
            },
    
            // Ambient light
            entity!
            {
                attributes: ("id", "ambient-light"),
                components: ("light", component!
                {
                    component::Light,
                    light_type: component::LightType::Ambient{},
                    color: color::GREY73,
                    intensity: 0.2
                })
            },
    
            // Directional light
            entity!
            {
                attributes: ("id", "directional-light"),
                components: 
                ("position", component::Position{ x: 0.5, y: 1.0, z: 1.0 }),
                ("light", component!
                {
                    component::Light,
                    light_type: component::LightType::Directional
                    {
                        shadow: component::OptionalDirectionalShadow::Cast
                        {
                            shadow: component!
                            {
                                component::DirectionalShadow
                            }
                        }
                    },
                    color: color::WHITE,
                    intensity: 0.1
                })
            },
            // The sky
            entity!
            {
                primitive: "a-sky",
                attributes: ("id", "sky"),
                components: ("material", component!
                {
                    component::Material, 
                    shader: Cow::Borrowed("strobe"),
                    props: component::MaterialProps(Cow::Owned(vec!
                    (
                        (Cow::Borrowed("color"), Cow::Borrowed("black")),
                        (Cow::Borrowed("color2"), Cow::Borrowed("#222222"))
                    )))
                })
            },
            // The ocean
            entity!
            {
                primitive: "a-ocean",
                attributes: ("id", "water"), ("depth", "100"), ("width", "100"), ("amplitude", "0.5"),
                components: ("material", component!
                {
                    component::Material, 
                    shader: Cow::Borrowed("water"),
                    props: component::MaterialProps(Cow::Owned(vec!((Cow::Borrowed("transparent"), Cow::Borrowed("true")))))
                })
            }
        }
    } />
}
```