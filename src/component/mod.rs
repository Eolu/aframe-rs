//! Module dealing with components in all forms, including registration,
//! representation, and instantiation.
//! 
//! ## High-level API
//! [component_def!](`component_def`)    
//! [component_struct!](`component_struct`)    
//! [component!](`component`)    
//! [simple_enum!](`simple_enum`)    
//! [complex_enum!](`complex_enum`)    
//!
//! ## Low-level API
//! A `component_struct` is simply a type that implements these 2 traits:
//!
//! ```ignore
//! pub trait Component: Display + std::fmt::Debug + std::any::Any
//! {
//!     fn clone(&self) -> Box<dyn Component>;
//!     fn eq(&self, other: &'static dyn Component) -> bool;
//!     fn as_map(&self) -> HashMap<Cow<'static, str>, Cow<'static, str>>;
//! }
//!
//! pub trait ConstDefault
//! {
//!     const DEFAULT: Self;
//! }
//! ```
//!
//! As long as `clone` provides a valid clone, `eq` provides a valid equality 
//! check, and `as_map` provides a serialization of keys to values that Aframe 
//! can understand, and a `DEFAULT` value is provided that can be evaluated at 
//! compile time, a struct is a valid component. 
//! 
//! A `ComponentReg` is slightly more complicated, but details on its low-level
//! API may be added here at a later date.

mod register;
mod instance;

pub use register::*;
pub use instance::*;

use std::borrow::Cow;
use crate::utils::*;
use crate::component_struct;
use crate::simple_enum;
use crate::complex_enum;

/// Convert a component to an attribute
pub fn cmp_to_attr((name, cmp): &(Cow<'static, str>, Box<dyn Component>)) -> Attribute 
{
    Attribute 
    { 
        name: name.to_owned(), 
        value: format!("{}", cmp).into() 
    }
}

component_struct!
(
    /// [animation](https://aframe.io/docs/1.6.0/components/animation.html)
    Animation,
    property: "property" Cow<'static, str> = Cow::Borrowed(""),
    is_raw_property: "isRawProperty" bool = false,
    from: "from" Cow<'static, str> = Cow::Borrowed("null"),
    to: "to" Cow<'static, str> = Cow::Borrowed("null"),
    delay: "delay" u64 = 0,
    dir: "dir" AnimationDirection = AnimationDirection::Normal,
    dur: "dur" u64 = 1000,
    easing: "easing" Easing = Easing::EaseInQuad,
    elasticity: "elasticity" u32 = 400,
    looping: "loop" AnimationLoop = AnimationLoop::Amount{looping: 0},
    round: "round" bool = false,
    start_events: "startEvents" List<Cow<'static, str>> = List::DEFAULT,
    pause_events: "pauseEvents" List<Cow<'static, str>> = List::DEFAULT,
    resume_events: "resumeEvents" List<Cow<'static, str>> = List::DEFAULT,
    autoplay: "autoplay" Autoplay = Autoplay::Null,
    enabled: "enabled" bool = true
);
complex_enum!
(
    /// [animation#loop](https://aframe.io/docs/1.6.0/components/animation.html#api_loop)
    AnimationLoop, 
    Amount "{}" => { looping: u32 },
    Forever "true" => {}
);
simple_enum!
(
    /// [animation#autoplay](https://aframe.io/docs/1.6.0/components/animation.html#api_autoplay)
    Autoplay, 
    Null => "null", 
    True => "true", 
    False => "false"
);
simple_enum!
(
    /// [animation#dir](https://aframe.io/docs/1.6.0/components/animation.html#api_dir)
    AnimationDirection, 
    Normal => "normal", 
    Reverse => "reverse", 
    Alternate => "alternate"
);
simple_enum!
(
    /// [animation#easings](https://aframe.io/docs/1.6.0/components/animation.html#easings)
    Easing, 
    EaseInQuad => "easeInQuad",
    EaseInCubic => "easeInCubic",
    EaseInQuart => "easeInQuart",
    EaseInQuint => "easeInQuint",
    EaseInSine => "easeInSine",
    EaseInExpo => "easeInExpo",
    EaseInCirc => "easeInCirc",
    EaseInBack => "easeInBack",
    EaseInElastic => "easeInElastic",
    EaseOutQuad => "easeOutQuad",
    EaseOutCubic => "easeOutCubic",
    EaseOutQuart => "easeOutQuart",
    EaseOutQuint => "easeOutQuint",
    EaseOutSine => "easeOutSine",
    EaseOutExpo => "easeOutExpo",
    EaseOutCirc => "easeOutCirc",
    EaseOutBack => "easeOutBack",
    EaseOutElastic => "easeOutElastic",
    EaseInOutQuad => "easeInOutQuad",
    EaseInOutCubic => "easeInOutCubic",
    EaseInOutQuart => "easeInOutQuart",
    EaseInOutQuint => "easeInOutQuint",
    EaseInOutSine => "easeInOutSine",
    EaseInOutExpo => "easeInOutExpo",
    EaseInOutCirc => "easeInOutCirc",
    EaseInOutBack => "easeInOutBack",
    EaseInOutElastic => "easeInOutElastic",
    Linear => "linear"
);
component_struct!
(
    /// [background](https://aframe.io/docs/1.6.0/components/background.html)
    Background, 
    color: "color" color::Rgb = color::BLACK,
    transparent: "transparent" bool = false
);
component_struct!
{
    /// [camera](https://aframe.io/docs/1.6.0/components/camera.html)
    Camera,
    active: "active" bool = true,
    far: "far" u32 = 10000,
    fov: "fov" f32 = 80.0,
    near: "near" f32 = 0.05,
    spectator: "spectator" bool = false,
    zoom: "zoom" f32 = 1.0
}
component_struct!
(
    /// [cursor](https://aframe.io/docs/1.6.0/components/cursor.html)
    Cursor, 
    // TODO: True event handling
    down_events: "downEvents" List<Cow<'static, str>> = List::DEFAULT,
    fuse: "fuse" bool = false,
    fuse_timeout: "fuseTimeout" u64 = 1500,
    mouse_cursor_styles_enabled: "mousecursorstylesenabled" bool = true,
    ray_origin: "rayOrigin" RayOrigin = RayOrigin::Entity,
    up_events: "upEvents" List<Cow<'static, str>> = List::DEFAULT
);
simple_enum!
{
    /// Where the intersection ray is cast from (i.e.,entity or mouse).
    RayOrigin,
    Mouse => "mouse",
    Entity => "entity"
}
component_struct!
(
    /// [daydream-controls](https://aframe.io/docs/1.6.0/components/daydream-controls.html)
    DaydreamControls, 
    arm_model: "armModel" bool = true,
    botton_color: "bottonColor" color::Rgb = color::BLACK,
    button_touched_color: "buttonTouchedColor" color::Rgb = color::GREY47,
    button_highlight_color: "buttonHighlightColor" color::Rgb = color::WHITE,
    hand: "hand" Hand = Hand::None,
    model: "model" bool = true,
    orientation_offset: "orientationOffset" Vector3 = Vector3::DEFAULT
);
simple_enum!
{
    /// Set hand that will be tracked (i.e., right, left).
    Hand,
    Right => "right",
    Left => "left",
    None => ""
}
component_struct!
(
    /// [device-orientation-permission-ui](https://aframe.io/docs/1.6.0/components/device-orientation-permission-ui.html)
    DeviceOrientationPermissionUI, 
    enabled: "enabled" bool = true,
    deny_button_text: "denyButtonText" Cow<'static, str> = Cow::Borrowed("Deny"),
    allow_button_text: "allowButtonText" Cow<'static, str> = Cow::Borrowed("Allow"),
    cancel_button_text: "cancelButtonText" Cow<'static, str> = Cow::Borrowed("Cancel"),
    device_motion_message: "deviceMotionMessage" Cow<'static, str> = Cow::Borrowed("Enable Device Motion"),
    mobile_desktop_message: "mobiledestkopmessage" Cow<'static, str> = Cow::Borrowed("Switch to Mobile Browsing"),
    https_message: "httpsMessage" Cow<'static, str> = Cow::Borrowed("Switch to HTTPS")
);
component_struct!
(
    /// [embedded](https://aframe.io/docs/1.6.0/components/embedded.html)
    Embedded
);
component_struct!
(
    /// [fog](https://aframe.io/docs/1.6.0/components/fog.html)
    Fog, 
    fog_type: "" FogType = FogType::Linear { near: 1, far: 1000 },
    color: "color" color::Rgb = color::BLACK
);
complex_enum!
{
    /// Type of fog distribution. Can be linear or exponential.
    FogType,
    Linear "type: linear; near: {}; far: {}" => { near: u64, far: u64 },
    Exponential "type: exponential; density: {}" => { density: f64}
}
component_struct!
(
    /// [gearvr-controls](https://aframe.io/docs/1.6.0/components/gearvr-controls.html)
    GearVRControls, 
    arm_model: "armModel" bool = true,
    button_color: "buttonColor" color::Rgb = color::BLACK,
    button_touched_color: "buttonTouchedColor" color::Rgb = color::GREY47,
    button_highlight_color: "buttonHighlightColor" color::Rgb = color::WHITE,
    hand: "hand" Hand = Hand::None,
    model: "model" bool = true,
    orientation_offset: "orientationOffset" Vector3 = Vector3::DEFAULT
);
component_struct!
(
    /// [geometry](https://aframe.io/docs/1.6.0/components/geometry.html)
    Geometry,
    primitive: "" GeometryPrimitive = GeometryPrimitive::Box
    {
        width: 1.0,
        height: 1.0,
        depth: 1.0,
        segments_width: 1,
        segments_height: 1,
        segments_depth: 1,
    },
    skip_cache: "skipCache" bool = false
);
complex_enum!
(
    /// [geometry#built-in-geometries](https://aframe.io/docs/1.6.0/components/geometry.html#built-in-geometries)
    GeometryPrimitive, 
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
    Cone
    "primitive: cone; height: {}; openEnded: {}; \
                    radiusBottom: {}; radiusTop: {}; segmentsRadial: {}; \
                    segmentsHeight: {}; thetaStart: {}; thetaLength: {}" =>
    {
        height: f32,
        open_ended: bool,
        radius_bottom: f32,
        radius_top: f32,
        segments_radial: u32,
        segments_height: u32,
        theta_start: f32,
        theta_length: f32
    },
    Cylinder
    "primitive: cylinder; radius: {}; height: {}; \
                    segmentsRadial: {}; segmentsHeight: {}; openEnded: {}; \
                    thetaStart: {}; thetaLength: {}" =>
    {
        radius: f32,
        height: f32,
        segments_radial: u32,
        segments_height: u32,
        open_ended: bool,
        theta_start: f32,
        theta_length: f32
    },
    Dodecahedron
    "primitive: dodecahedron; radius: {}" => { radius: f32 },
    Octahedron
    "primitive: octahedron; radius: {}" => { radius: f32 },
    Plane
    "primitive: plane; width: {}; height: {}; \
    segmentsWidth: {}; segmentsHeight: {}" =>
    {
        width: f32,
        height: f32,
        segments_width: u32,
        segments_height: u32
    },
    Ring
    "primitive: ring; radiusInner: {}; radiusOuter: {}; \
    segmentsTheta: {}; segmentsPhi: {}; thetaStart: {}; \
    thetaLength: {}" =>
    {
        radius_inner: f32,
        radius_outer: f32,
        segments_theta: u32,
        segments_phi: u32,
        theta_start: f32,
        theta_length: f32
    },
    Sphere
    "primitive: sphere; radius: {}; segmentsWidth: {}; \
    segmentsHeight: {}; phiStart: {}; phiLength: {}; \
    thetaStart: {}; thetaLength: {}" =>
    {
        radius: f32,
        segments_width: u32,
        segments_height: u32,
        phi_start: f32,
        phi_length: f32,
        theta_start: f32,
        theta_length: f32
    },
    Tetrahedron
    "primitive: tetrahedron; radius: {}" => { radius: f32 },
    Torus
    "primitive: torus; radius: {}; radiusTubular: {}; \
    segmentsRadial: {}; segmentsTubular: {}; arc: {}" =>
    {
        radius: f32,
        radius_tubular: f32,
        segments_radial: u32,
        segments_tubular: u32,
        arc: f32
    },
    TorusKnot
    "primitive: torusKnot; radius: {}; radiusTubular: {}; \
    segmentsRadial: {}; segmentsTubular: {}; p: {}; q: {}" =>
    {
        radius: f32,
        radius_tubular: f32,
        segments_radial: u32,
        segments_tubular: u32,
        p: u32,
        q: u32
    },
    Triangle
    "primitive: triangle; vertexA: {}; vertexB: {}; vertexC: {}" =>
    {
        vertex_a: Vector3,
        vertex_b: Vector3,
        vertex_c: Vector3
    },
    // TODO: A true high-level implementation of this needs to be done. This
    // implementation is just a placeholder.
    Custom
    "{}" =>
    {
        data: Cow<'static, str>
    }
);
component_struct!
(
    /// [gltf-model](https://aframe.io/docs/1.6.0/components/gltf-model.html)
    GltfModel, 
    string: "" Cow<'static, str> = Cow::Borrowed("")
);
component_struct!
(
    /// [hand-controls](https://aframe.io/docs/1.6.0/components/hand-controls.html)
    HandControls, 
    color: "color" color::Rgb = color::WHITE,
    hand: "hand" Hand = Hand::Left,
    hand_model_style: "handModelStyle" HandModelStyle = HandModelStyle::LowPoly
);
simple_enum!
{
    /// Style of the hand 3D model loaded. Can be lowPoly, highPoly or toon.
    HandModelStyle,
    LowPoly => "lowPoly",
    HighPoly => "highPoly",
    Toon => "toon"
}
component_struct!
(
    /// [hand-controls](https://aframe.io/docs/1.6.0/components/hand-controls.html)
    HandTrackingControls, 
    hand: "hand" Hand = Hand::Left,
    model_color: "modelColor" color::Rgb = color::WHITE,
    model_style: "modelStyle" ModelStyle = ModelStyle::Mesh
);
simple_enum!
{
    /// Mesh representing the hand or dots matching the joints
    ModelStyle,
    Mesh => "mesh",
    Dots => "dots"
}
component_struct!
(
    /// [laser-controls](https://aframe.io/docs/1.6.0/components/laser-controls.html)
    LaserControls, 
    enter_vr: "enterVR" bool = true
);
component_struct!
(
    /// [keyboard-shortcuts](https://aframe.io/docs/1.6.0/components/keyboard-shortcuts.html)
    KeyboardShortcuts, 
    hand: "hand" Hand = Hand::Left,
    model: "model" bool = true,
    default_model_color: "defaultModelColor" color::Rgb = color::WHITE
);
component_struct!
(
    /// [layer](https://aframe.io/docs/1.6.0/components/layer.html)
    Layer, 
    layer_type: "type" LayerType = LayerType::Quad,
    src: "src" Cow<'static, str> = Cow::Borrowed(""),
    rotate_cubemap: "rotateCubemap" bool = false
);
simple_enum!
{
    /// quad, monocubemap or stereocubemap
    LayerType,
    Quad => "quad",
    MonoCubeMap => "monocubemap",
    StereoCubeMap => "stereocubemap"
}
component_struct!
(
    /// [light](https://aframe.io/docs/1.6.0/components/light.html)
    Light, 
    light_type: "" LightType = LightType::Directional { shadow: OptionalDirectionalShadow::NoCast{} },
    color: "color" color::Rgb = color::WHITE,
    intensity: "intensity" f32 = 1.0
);
complex_enum!
(
    /// [light-types](https://aframe.io/docs/1.6.0/components/light.html#light-types)
    LightType, 
    Ambient "type: ambient; " => {},
    Directional "type: directional; {}" => { shadow: OptionalDirectionalShadow },
    Hemisphere "type: hemisphere; groundColor: {}" => { ground_color: color::Rgb  },
    Point "type: point; decay: {}; distance: {}; {}" => 
    { 
        decay: f32, 
        distance: f32,
        shadow: OptionalLocalShadow
    },
    Spot "type: spot; angle: {}; decay: {}; distance: {}; penumbra: {}; target: {}; {}" =>
    {
        angle: i32,
        decay: f32,
        distance: f32,
        penumbra: f32,
        target: Cow<'static, str>,
        shadow: OptionalLocalShadow
    }
);
component_struct!
(
    /// [light#shadow](https://aframe.io/docs/1.6.0/components/light.html#configuring-shadows)
    LocalShadow, 
    shadow_bias: "shadowBias" f64 = 0.0,
    shadow_camera_far: "shadowCameraFar" f32 = 500.0,
    shadow_camera_near: "shadowCameraNear" f32 = 0.5,
    shadow_camera_visible: "shadowCameraVisible" bool = false,
    shadow_map_height: "shadowMapHeight" u32 = 512,
    shadow_map_width: "shadowMapWidth" u32 = 512,
    shadow_camera_fov: "shadowCameraFov" f32 = 50.0
);
component_struct!
(
    /// [light#shadow](https://aframe.io/docs/1.6.0/components/light.html#configuring-shadows)
    DirectionalShadow, 
    shadow_bias: "shadowBias" f64 = 0.0,
    shadow_camera_far: "shadowCameraFar" f32 = 500.0,
    shadow_camera_near: "shadowCameraNear" f32 = 0.5,
    shadow_camera_visible: "shadowCameraVisible" bool = false,
    shadow_map_height: "shadowMapHeight" u32 = 512,
    shadow_map_width: "shadowMapWidth" u32 = 512,
    shadow_camera_bottom: "shadowCameraBottom" f32 = -5.0,
    shadow_camera_left: "shadowCameraLeft" f32 = -5.0,
    shadow_camera_right: "shadowCameraRight" f32 = 5.0,
    shadow_camera_top: "shadowCameraTop" f32 = 5.0
);
complex_enum!
(
    /// Shadow used for point and spot lights
    OptionalLocalShadow, 
    Cast "castShadow: true; {}" => { shadow: LocalShadow },
    NoCast "castShadow: false;" => {}
);
complex_enum!
(
    /// Shadow used for directional lights
    OptionalDirectionalShadow, 
    Cast "castShadow: true; {}" => { shadow: DirectionalShadow },
    NoCast "castShadow: false;" => {}
);
component_struct!
(
    /// [line](https://aframe.io/docs/1.6.0/components/line.html)
    Line, 
    start: "start" Vector3 = Vector3::DEFAULT,
    end: "end" Vector3 = Vector3::DEFAULT,
    color: "color" color::Rgb = color::Rgb::new(116, 190, 193),
    opacity: "opacity" f64 = 1.0,
    visible: "visible" bool = true
);
component_struct!
(
    /// [link](https://aframe.io/docs/1.6.0/components/link.html)
    Link, 
    background_color: "backgroundColor" color::Rgb = color::RED,
    border_color: "borderColor" color::Rgb = color::WHITE,
    highlighted: "highlighted" bool = false,
    highlighted_color: "highlightedColor" color::Rgb = color::Rgb::new(36, 202, 255),
    href: "href" Cow<'static, str> = Cow::Borrowed(""),
    image: "image" Cow<'static, str> = Cow::Borrowed(""),
    on: "on" Cow<'static, str> = Cow::Borrowed("click"),
    peek_mode: "peekMode" bool = false,
    title: "title" Cow<'static, str> = Cow::Borrowed(""),
    title_color: "titleColor" color::Rgb = color::WHITE,
    visual_aspect_enabled: "visualAspectEnabled" bool = true
);
component_struct!
(
    /// [loading-screen](https://aframe.io/docs/1.6.0/components/loading-screen.html)
    LoadingScreen, 
    dots_color: "dotsColor" color::Rgb = color::WHITE,
    background_color: "backgroundColor" color::Rgb = color::Rgb::new(36, 202, 255),
    enabled: "enabled" bool = true
);
component_struct!
{
    /// [look-controls](https://aframe.io/docs/1.6.0/components/look-controls.html)
    LookControls,
    enabled: "enabled" bool = true,
    hmd_enabled: "hmdEnabled" bool = true,
    reverse_mouse_drag: "reverseMouseDrag" bool = false,
    reverse_touch_drag: "reverseTouchDrag" bool = false,
    touch_enabled: "touchEnabled" bool = true,
    mouse_enabled: "mouseEnabled" bool = true,
    pointer_lock_enabled: "pointerLockEnabled" bool = false,
    magic_window_tracking_enabled: "magicWindowTrackingEnabled" bool = true
}
component_struct!
{
    /// [magicleap-controls](https://aframe.io/docs/1.6.0/components/magicleap-controls.html)
    MagicLeapControls,
    hand: "hand" Hand = Hand::None,
    model: "model" bool = true,
    orientation_offset: "orientationOffset" Vector3 = Vector3::DEFAULT
}
component_struct!
(
    /// [material](https://aframe.io/docs/1.6.0/components/material.html)
    Material, 
    alpha_test: "alphaTest" f32 = 0.0,
    depth_test: "depthTest" bool = true,
    flat_shading: "flatShading" bool = false,
    npot: "npot" bool = false,
    offset: "offset" Vector2 = Vector2 { x: 0.0, y: 0.0 },
    opacity: "opacity" f32 = 1.0,
    repeat: "repeat" Vector2 = Vector2 { x: 1.0, y: 1.0 },
    shader: "shader" Cow<'static, str> = Cow::Borrowed("standard"),
    side: "side" MaterialSide = MaterialSide::Front,
    transparent: "transparent" bool = false,
    vertex_colors: "vertexColors" VertexColors = VertexColors::None,
    visible: "visible" bool = true,
    blending: "blending" Blending = Blending::Normal,
    dithering: "dithering" bool = true,
    props: "" MaterialProps = MaterialProps::DEFAULT
);
simple_enum!
(
    /// [material#side](https://aframe.io/docs/1.6.0/components/material.html#properties_side)
    MaterialSide, 
    Front => "front", 
    Back => "back", 
    Double => "double"
);
simple_enum!
(
    /// [material#vertexcolors](https://aframe.io/docs/1.6.0/components/material.html#properties_vertexcolors)
    VertexColors, 
    None => "none", 
    Vertex => "vertex", 
    Face => "face"
);
simple_enum!
(
    /// [material#blending](https://aframe.io/docs/1.6.0/components/material.html#properties_blending)
    Blending, 
    None => "none", 
    Normal => "normal", 
    Additive => "additive", 
    Subtractive => "subtractive", 
    Multiply => "multiply"
);

/// Additional properties for the Material component. Contains a slice or vector
/// of property names to property values.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[repr(transparent)]
pub struct MaterialProps(pub Cow<'static, [(Cow<'static, str>, Cow<'static, str>)]>);
impl MaterialProps
{
    pub const DEFAULT: Self = MaterialProps(Cow::Borrowed(&[]));
}
impl std::fmt::Display for MaterialProps
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        for s in self.0.iter().map(|(k, v)| format!("{}: {}; ", k, v))
        {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}
component_struct!
(
    /// [obj-model](https://aframe.io/docs/1.6.0/components/obj-model.html)
    ObjModel, 
    obj: "obj" Cow<'static, str> = Cow::Borrowed(""),
    mtl: "mtl" Cow<'static, str> = Cow::Borrowed("")
);
component_struct!
(
    /// [oculus-go-controls](https://aframe.io/docs/1.6.0/components/oculus-go-controls.html)
    OculusGoControls, 
    arm_model: "armModel" bool = true,
    button_color: "buttonColor" color::Rgb = color::BLACK,
    button_touched_color: "buttonTouchedColor" color::Rgb = color::GREY47,
    button_highlight_color: "buttonHighlightColor" color::Rgb = color::WHITE,
    hand: "hand" Hand = Hand::None,
    model: "model" bool = true,
    orientation_offset: "orientationOffset" Vector3 = Vector3::DEFAULT
);
component_struct!
(
    /// [oculus-touch-controls](https://aframe.io/docs/1.6.0/components/oculus-touch-controls.html)
    OculusTouchControls,
    hand: "hand" Hand = Hand::Left,
    model: "model" bool = true,
    orientation_offset: "orientationOffset" Vector3 = Vector3::DEFAULT
);
component_struct!
(
    /// [pool](https://aframe.io/docs/1.6.0/components/pool.html)
    Pool,
    container: "container" Cow<'static, str> = Cow::Borrowed(""),
    dynamic: "dynamic" bool = false,
    mixin: "mixin" Cow<'static, str> = Cow::Borrowed(""),
    size: "size" u32 = 0
);
component_struct!
(
    /// [position](https://aframe.io/docs/1.6.0/components/position.html)
    Position :alt "{} {} {}", x: "x" f32 = 0.0, y: "y" f32 = 0.0, z: "z" f32 = 0.0
);
component_struct!
{
    /// [renderer](https://aframe.io/docs/1.6.0/components/renderer.html)
    Renderer,
    antialias: "antialias" Antialias = Antialias::Auto,
    color_management: "colorManagement" bool = false,
    high_refresh_rate: "highRefreshRate" bool = false,
    foveation_level: "foveationLevel" u8 = 0,
    sort_objects: "sortObjects" bool = false,
    physically_correct_lights: "physicallyCorrectLights" bool = false,
    max_canvas_width: "maxCanvasWidth" u32 = 1920,
    max_canvas_height: "maxCanvasHeight" u32 = 1920,
    logarithmic_depth_buffer: "logarithmicDepthBuffer" LogarithmicDepthBuffer = LogarithmicDepthBuffer::Auto,
    precision: "precision" Precision = Precision::High,
    alpha: "alpha" bool = true
}
simple_enum!
{
    /// Whether to perform antialiasing. If auto, antialiasing is disabled on mobile.
    Antialias,
    Auto => "auto",
    True => "true",
    False => "false"
}
simple_enum!
{
    /// Whether to use a logarithmic depth buffer.
    LogarithmicDepthBuffer,
    Auto => "auto",
    True => "true",
    False => "false"
}
simple_enum!
{
    /// Fragment shader precision : low, medium or high.
    Precision,
    Low => "low",
    Medium => "medium",
    High => "high"
}
component_struct!
{
    /// [raycaster](https://aframe.io/docs/1.6.0/components/raycaster.html)
    RayCaster,
    auto_refresh: "autoRefresh" bool = true,
    direction: "direction" Vector3 = Vector3 { x: 0.0, y: 0.0, z: -1.0 },
    enabled: "enabled" bool = true,
    far: "far" DistancePlane = DistancePlane::Infinity{},
    interval: "interval" u32 = 0,
    line_color: "lineColor" color::Rgb = color::WHITE,
    line_opacity: "lineOpacity" color::Rgb = color::WHITE,
    near: "near" DistancePlane = DistancePlane::Distance{distance: 0.0},
    objects: "objects" List<Cow<'static, str>> = List(Cow::Borrowed(&[Cow::Borrowed("null")])),
    origin: "origin" Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 },
    show_line: "showLine" bool = false,
    use_world_coordinates: "useWorldCoordinates" bool = false
}
complex_enum! 
{
    /// [raycaster#far](https://aframe.io/docs/1.6.0/components/raycaster.html#properties_far)
    DistancePlane,
    Infinity "Infinity" => {},
    Distance "{}" => {distance: f32}
}
component_struct!
(
    /// [rotation](https://aframe.io/docs/1.6.0/components/rotation.html)
    Rotation :alt "{} {} {}", x: "x" f32 = 0.0, y: "y" f32 = 0.0, z: "z" f32 = 0.0
);
component_struct!
(
    /// [scale](https://aframe.io/docs/1.6.0/components/scale.html)
    Scale :alt "{} {} {}", x: "x" f32 = 1.0, y: "y" f32 = 1.0, z: "z" f32 = 1.0
);
component_struct!
{
    /// [screenshot](https://aframe.io/docs/1.6.0/components/screenshot.html)
    Screenshot,
    width: "width" u32 = 4096,
    height: "height" u32 = 2048
}
component_struct!
(
    /// [shadow](https://aframe.io/docs/1.6.0/components/shadow.html)
    Shadow, 
    cast: "cast" bool = true,
    receive: "receive" bool = true
);
component_struct!
(
    /// [sound](https://aframe.io/docs/1.6.0/components/sound.html)
    Sound, 
    src: "src" Cow<'static, str> = Cow::Borrowed(""),
    autoplay: "autoplay" bool = false,
    positional: "positional" bool = true,
    volume: "volume" f32 = 1.0,
    looping: "loop" bool = false
);
component_struct!
(
    /// [stats](https://aframe.io/docs/1.6.0/components/stats.html)
    Stats
);
component_struct!
(
    /// [text](https://aframe.io/docs/1.6.0/components/text.html)
    Text, 
    align: "align" TextAlignment = TextAlignment::Left,
    alpha_test: "alphaTest" f32 = 0.5,
    anchor: "anchor" TextAnchor = TextAnchor::Center,
    baseline: "baseline" TextBaseline = TextBaseline::Center,
    color: "color" color::Rgb = color::WHITE,
    font: "font" Cow<'static, str> = Cow::Borrowed("default"),
    font_image: "fontImage" Cow<'static, str> = Cow::Borrowed("default"),
    height: "height" u32 = 0,
    letter_spacing: "letterSpacing" u32 = 0,
    line_height: "lineHeight" u32 = 0,
    opacity: "opacity" f64 = 1.0,
    shader: "shader" Cow<'static, str> = Cow::Borrowed("sdf"),
    side: "side" TextSide = TextSide::Front,
    tab_size: "tabSize" u32 = 4,
    transparent: "transparent" bool = true,
    value: "value" Cow<'static, str> = Cow::Borrowed(""),
    white_space: "whiteSpace" TextWhiteSpace = TextWhiteSpace::Normal,
    width: "width" u32 = 0,
    wrap_count: "wrapCount" u32 = 40,
    wrap_pixels: "wrapPixels" u32 = 0,
    x_offset: "xOffset" f64 = 0.0,
    y_offset: "yOffset" f64 = 0.001
);
simple_enum!
{
    /// Multi-line text alignment (left, center, right).
    TextAlignment,
    Left => "left",
    Center => "center",
    Right => "right"
}
simple_enum!
{
    /// Horizontal positioning (left, center, right, align).
    TextAnchor,
    Left => "left",
    Center => "center",
    Right => "right",
    Align => "align"
}
simple_enum!
{
    /// Vertical positioning (top, center, bottom).
    TextBaseline,
    Top => "top",
    Center => "center",
    Bottom => "bottom"
}
simple_enum!
{
    /// Side to render. (front, back, double)
    TextSide,
    Front => "front",
    Back => "back",
    Double => "double"
}
simple_enum!
{
    /// How whitespace should be handled (i.e., normal, pre, nowrap).
    TextWhiteSpace,
    Normal => "normal",
    Pre => "pre",
    NoWrap => "nowrap"
}
component_struct!
(
    /// [tracked-controls](https://aframe.io/docs/1.6.0/components/tracked-controls.html)
    TrackedControls, 
    arm_model: "armModel" bool = true,
    auto_hide: "autoHide" bool = true,
    controller: "controller" u32 = 0,
    id: "id" Cow<'static, str> = Cow::Borrowed(""),
    id_prefix: "idPrefix" Cow<'static, str> = Cow::Borrowed(""),
    head_element: "headElement" Cow<'static, str> = Cow::Borrowed(""),
    hand: "hand" Hand = Hand::Right,
    orientation_offset: "orientationOffset" Vector3 = Vector3::DEFAULT
);
component_struct!
(
    /// [visible](https://aframe.io/docs/1.6.0/components/visible.html)
    Visible :alt "{}", visible: "visible" bool = true
);
component_struct!
(
    /// [vive-controls](https://aframe.io/docs/1.6.0/components/vive-controls.html)
    ViveControls, 
    button_color: "buttonColor" color::Rgb = color::GREY98,
    button_highlight_color: "buttonHighlightColor" color::Rgb = color::Rgb::new(34, 209, 238),
    hand: "hand" Hand = Hand::Left,
    model: "model" bool = true,
    orientation_offset: "orientationOffset" Vector3 = Vector3::DEFAULT
);
component_struct!
(
    /// [vive-focus-controls](https://aframe.io/docs/1.6.0/components/vive-focus-controls.html)
    ViveFocusControls, 
    arm_model: "armModel" bool = true,
    button_touched_color: "buttonTouchedColor" color::Rgb = color::GREY47,
    button_highlight_color: "buttonHighlightColor" color::Rgb = color::WHITE,
    hand: "hand" Hand = Hand::None,
    model: "model" bool = true,
    orientation_offset: "orientationOffset" Vector3 = Vector3::DEFAULT
);
component_struct!
(
    /// [vr-mode-ui](https://aframe.io/docs/1.6.0/components/vr-mode-ui.html)
    VrModeUi, 
    enabled: "enabled" bool = true,
    enter_vr_button: "enterVRButton" Cow<'static, str> = Cow::Borrowed(""),
    enter_ar_button: "enterARButton" Cow<'static, str> = Cow::Borrowed("")
);
component_struct!
(
    /// [wasd-controls](https://aframe.io/docs/1.6.0/components/wasd-controls.html)
    WasdControls, 
    acceleration: "acceleration" u32 = 65,
    ad_axis: "adAxis" WasdAxis = WasdAxis::X,
    ad_inverted: "adInverted" bool = false,
    enabled: "enabled" bool = true,
    fly: "fly" bool = false,
    ws_axis: "wsAxis" WasdAxis = WasdAxis::Z,
    ws_inverted: "wsInverted" bool = false
);
simple_enum!
{
    /// Axis that the A and D keys act upon.
    WasdAxis,
    X => "x",
    Y => "y",
    Z => "z"
}
component_struct!
(
    /// [webxr](https://aframe.io/docs/1.6.0/components/webxr.html)
    WebXR, 
    reference_space_type: "referenceSpaceType" ReferenceSpaceType = ReferenceSpaceType::LocalFloor,
    required_features: "requiredFeatures" List<Cow<'static, str>> = List::DEFAULT,
    optional_features: "optionalFeatures"  List<Cow<'static, str>> = List::DEFAULT,
    overlay_element: "overlayElement" Cow<'static, str> = Cow::Borrowed("null")
);
simple_enum!
{
    /// The scene’s reference space type for camera and controllers.
    ReferenceSpaceType,
    Viewer => "viewer",
    Local => "local",
    LocalFloor => "local-floor",
    BoundedFloor => "bounded-floor",
    Unbounded => "unbounded"
}
component_struct!
(
    /// [windows-motion-controls](https://aframe.io/docs/1.6.0/components/windows-motion-controls.html)
    WindowsMotionControls, 
    hand: "hand" Hand = Hand::Left,
    pair: "pair" u32 = 0,
    model: "model" bool = true,
    hide_disconnected: "hideDisconnected" bool = true
);
