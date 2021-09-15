mod register;
mod instance;

pub use register::*;
pub use instance::*;

use std::borrow::Cow;
use crate::utils::*;
use crate::component_struct;
use crate::simple_enum;
use crate::complex_enum;

component_struct!(Position "{} {} {}", x: f32 = 0.0, y: f32 = 0.0, z: f32 = 0.0);
component_struct!(Rotation "{} {} {}", x: f32 = 0.0, y: f32 = 0.0, z: f32 = 0.0);
component_struct!
(Sound, 
    src: "src" Cow<'static, str> = Cow::Borrowed(""),
    autoplay: "autoplay" bool = false,
    positional: "positional" bool = true,
    volume: "volume" f32 = 1.0,
    looping: "loop" bool = false
);
component_struct!
(Light "{}; color: {}; intensity: {}", 
    light_type: LightType = LightType::Directional { shadow: OptionalDirectionalShadow::NoCast{} },
    color: color::Rgb = color::WHITE,
    intensity: f32 = 1.0
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
    shadow_map_width: "shadowMapWdith" u32 = 512,
    shadow_camera_fov: "shadowCameraFov" f32 = 50.0
);
component_struct!
(DirectionalShadow, 
    shadow_bias: "shadowBias" f64 = 0.0,
    shadow_camera_far: "shadowCameraFar" f32 = 500.0,
    shadow_camera_near: "shadowCameraNear" f32 = 0.5,
    shadow_camera_visible: "shadowCameraVisible" bool = false,
    shadow_map_height: "shadowMapHeight" u32 = 512,
    shadow_map_width: "shadowMapWdith" u32 = 512,
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
(Geometry "{}; skipCache: {}",
    primitive: GeometryPrimitive = GeometryPrimitive::Box
    {
        width: 1.0,
        height: 1.0,
        depth: 1.0,
        segments_width: 1,
        segments_height: 1,
        segments_depth: 1,
    },
    skip_cache: bool = false
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
(Material 
    "alphaTest: {}; depthTest: {}; flatShading: {}; npot: {}; \
    offset: {}; opacity: {}; repeat: {}; shader: {}; \
    side: {}; transparent: {}; vertexColors: {}; visible: {}; \
    blending: {}; dithering: {}; {}", 
    alpha_test: f32 = 0.0,
    depth_test: bool = true,
    flat_shading: bool = false,
    npot: bool = false,
    offset: Vector2 = Vector2 { x: 0.0, y: 0.0 },
    opacity: f32 = 1.0,
    repeat: Vector2 = Vector2 { x: 1.0, y: 1.0 },
    shader: Cow<'static, str> = Cow::Borrowed("standard"),
    side: MaterialSide = MaterialSide::Front,
    transparent: bool = false,
    vertex_colors: VertexColors = VertexColors::None,
    visible: bool = true,
    blending: Blending = Blending::Normal,
    dithering: bool = true,
    props: MaterialProps = MaterialProps::DEFAULT
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
#[derive(Debug, Clone, PartialEq)]
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