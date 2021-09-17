mod register;
mod instance;

pub use register::*;
pub use instance::*;

use std::borrow::Cow;
use crate::utils::*;
use crate::component_struct;
use crate::simple_enum;
use crate::complex_enum;

component_struct!(Position :alt "{} {} {}", x: "x" f32 = 0.0, y: "y" f32 = 0.0, z: "z" f32 = 0.0);
component_struct!(Rotation :alt "{} {} {}", x: "x" f32 = 0.0, y: "y" f32 = 0.0, z: "z" f32 = 0.0);
component_struct!
(Sound, 
    src: "src" Cow<'static, str> = Cow::Borrowed(""),
    autoplay: "autoplay" bool = false,
    positional: "positional" bool = true,
    volume: "volume" f32 = 1.0,
    looping: "loop" bool = false
);
component_struct!
(Light, 
    light_type: "" LightType = LightType::Directional { shadow: OptionalDirectionalShadow::NoCast{} },
    color: "color" color::Rgb = color::WHITE,
    intensity: "intensity" f32 = 1.0
);
complex_enum!
(LightType, 
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
(LocalShadow, 
    shadow_bias: "shadowBias" f64 = 0.0,
    shadow_camera_far: "shadowCameraFar" f32 = 500.0,
    shadow_camera_near: "shadowCameraNear" f32 = 0.5,
    shadow_camera_visible: "shadowCameraVisible" bool = false,
    shadow_map_height: "shadowMapHeight" u32 = 512,
    shadow_map_width: "shadowMapWidth" u32 = 512,
    shadow_camera_fov: "shadowCameraFov" f32 = 50.0
);
component_struct!
(DirectionalShadow, 
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
(OptionalLocalShadow, 
    Cast "castShadow: true; {}" => { shadow: LocalShadow },
    NoCast "castShadow: false;" => {}
);
complex_enum!
(OptionalDirectionalShadow, 
    Cast "castShadow: true; {}" => { shadow: DirectionalShadow },
    NoCast "castShadow: false;" => {}
);

component_struct!
(Animation,
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
(AnimationLoop, 
    Amount "{}" => { looping: u32 },
    Forever "true" => {}
);
simple_enum!
(Autoplay, 
    Null => "null", 
    True => "true", 
    False => "false"
);
simple_enum!
(AnimationDirection, 
    Normal => "normal", 
    Reverse => "reverse", 
    Alternate => "alternate"
);
simple_enum!
(Easing, 
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
{
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
    DistancePlane,
    Infinity "Infinity" => {},
    Distance "{}" => {distance: f32}
}
component_struct!
{
    Camera,
    active: "active" bool = true,
    far: "far" u32 = 10000,
    fov: "fov" f32 = 80.0,
    near: "near" f32 = 0.05,
    spectator: "spectator" bool = false,
    zoom: "zoom" f32 = 1.0
}
component_struct!
{
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
    skip_cache: "skipCache" bool = false
);
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
    }
    //Custom(CustomGeometry)
);
component_struct!
(Shadow, 
    cast: "cast" bool = true,
    receive: "receive" bool = true
);
component_struct!
(Material, 
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
(MaterialSide, 
    Front => "front", 
    Back => "back", 
    Double => "double"
);
simple_enum!
(VertexColors, 
    None => "none", 
    Vertex => "vertex", 
    Face => "face"
);
simple_enum!
(Blending, 
    None => "none", 
    Normal => "normal", 
    Additive => "additive", 
    Subtractive => "subtractive", 
    Multiply => "multiply"
);
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