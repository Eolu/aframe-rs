# aframe-rs

This is an [Aframe](https://aframe.io/) library for rust. It's still fairly experimental and a lot might change. I started writing this for a bit of fun to see if I could play with aframe from inside a [yew](https://yew.rs/) app. It started getting pretty large so I decided to abstract away all the yew-specific stuff and start making a library on its own. There's still a bunch missing and a bunch to do here, but what IS there is functional.

# Setup

### Initialization

This crate contains an `init` feature which may be enabled to allow initialization from an async function:

```rust,ignore

async fn app_main() -> Result<(), aframe::InitError>
{
    aframe::init_aframe().await?;
    // ... Now you can safely continue
}

```

You can also initialize simply by adding the Aframe script to your HTML header:

```html
<script src="https://aframe.io/releases/1.2.0/aframe.min.js"></script>
```

### Use

You can either use this crate's `Htmlify` trait to output raw html, or use the `yew-support` feature to create a yew componment (described lower in this readme) to output your actual Aframe scene.

# API    

## Scene    
Instantiating a scene:    
[scene!](https://docs.rs/aframe/*/aframe/macro.scene.html)    

## Components    
Defining a new component:    
[component_def!](https://docs.rs/aframe/*/aframe/macro.component_def.html)    

Declaring the structure of a defined component:    
[component_struct!](https://docs.rs/aframe/*/aframe/macro.component_struct.html)    
[simple_enum!](https://docs.rs/aframe/*/aframe/macro.simple_enum.html)    
[complex_enum!](https://docs.rs/aframe/*/aframe/macro.complex_enum.html)    

Instantiating a component struct:    
[component!](https://docs.rs/aframe/*/aframe/macro.component.html)    
 
See the [component](https://docs.rs/aframe/*/aframe/component/) module for more information and for 
pre-defined component structs.

## Custom Geometry
Defining a new custom geometry:    
[geometry_def!](https://docs.rs/aframe/*/aframe/macro.geometry_def.html)    

Not yet implemented:
* `geometry_struct!` macro to declare structure of custom geometry data
* `geometry!` macro to serve as a helper when instantiating custom geometry

## Entities & Primitives    
Instantiating an entity or defined primitive:    
[entity!](https://docs.rs/aframe/*/aframe/macro.entity.html)   

Defining a new primitive:    
[primitive!](https://docs.rs/aframe/*/aframe/macro.primitive.html)   

## Assets    

The `assets!` and `mixin!` macros are provided to define an `Assets` struct.   

[assets!](https://docs.rs/aframe/*/aframe/macro.assets.html)    
[mixin!](https://docs.rs/aframe/*/aframe/macro.mixin.html)    

## Shaders    

[Shader](https://docs.rs/aframe/*/aframe/shader/struct.Shader.html)   

## Htmlify    

The `Htmlify` trait is is to generate raw HTML from the structures provided in this crate. This may eventually be abstracted into a separate crate. (TBD: Is there a better crate in existence already?)    

[Htmlify](https://docs.rs/aframe/*/aframe/utils/htmlify/trait.Htmlify.html)    

## Sys API    

The lowest-level calls to Aframe are defined in the `sys` module:    

[registerPrimitive](https://docs.rs/aframe/*/aframe/sys/fn.registerPrimitive.html)    
[registerComponent](https://docs.rs/aframe/*/aframe/sys/fn.registerComponent.html)    
[registerShader](https://docs.rs/aframe/*/aframe/sys/fn.registerShader.html)    

## yew_support feature

The `yew_support` feature adds yew support to this crate. At its core, all this does is implement `From<&Scene> for Html` along with a few other conversions to yew's Html type.

See the [yew-ext module page](https://docs.rs/aframe/*/aframe/yew_ext/index.html)  for an example.

# WIP/Missing Features

* Event handling
* State handling
* Complete primitives implementation
* High-level support for custom geometry
* Access to Aframe utility functions
* Some component implementations are still accepting strings where they could accept enums or more specific structures

# Example

Below is a full example of how a scene is constructed in yew (this also serves of a valid example of how to use the `scene!` macro even outside of a yew context):

```rust,ignore
html!
{
    <Aframe scene = 
    { 
        // Using this contant to clean up some fluff in the code below.
        const CURSOR_COLOR: [(Cow<'static, str>, Cow<'static, str>); 1] = 
            [(Cow::Borrowed("color"), Cow::Borrowed("lightblue"))];
        scene!
        {
            // TODO: Some of these attributes are actually components, they need to be implemented in the library!
            attributes: ("inspector", "true"), ("embedded", "true"), ("cursor", "rayOrigin: mouse"),
                        ("mixin", "intersect_ray"), ("style", "min-height: 50px;"),
            assets: assets!
            {
                // Assume we have a few assets available to use
                Image::new("ramen", "/pics/ramen.png"),
                Image::new("noise", "/pics/noise.bmp"),
                // Create a mixin for shadows to know what to interact with
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
            // The camera rig
            entity!
            {
                attributes: ("id", "rig"),
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
                // This assumes the existence of a primitive registered as "ramen-cube"
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
                    // This assumes the existence of a shader registered as "strobe"
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
                    // This assumes the existence of a shader registered as "water"
                    shader: Cow::Borrowed("water"),
                    props: component::MaterialProps(Cow::Owned(vec!((Cow::Borrowed("transparent"), Cow::Borrowed("true")))))
                })
            }
        }
    } />
}
```
