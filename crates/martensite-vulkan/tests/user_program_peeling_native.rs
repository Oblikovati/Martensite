//! A blended user program in OIT frames against recorded dumps.
//!
//! The reference scene user_programs/user_programs_peeling was recorded on llvmpipe (SSE2/128) and AMD radeonsi at 256x256: the uniforms program of
//! the user_programs case (GLSL 1.10 header, uColor alpha 0.5) on a half-transparent box in
//! front of an opaque red box, without OIT, with weighted OIT and with depth peeling of four
//! and two layers. The program was set without OIT, so it has one output and writes
//! `gl_FragColor`, which GL writes into every draw buffer: under weighted OIT into the colour
//! and coverage targets, under depth peeling into the depth, front and back targets of every
//! pass (GlLayerList::renderTransparent). The peeled colour then depends on the layer
//! count and its alpha saturates, so the program box hides the opaque box behind it.
#[allow(dead_code)] // Only the shape reader serves the session helpers.
#[path = "support/dataset_shape.rs"]
mod dataset_shape;
#[path = "support/native_dump_grading.rs"]
mod grading;
#[allow(dead_code)] // Only the boxes of this case.
#[path = "case_host/primitive_shapes.rs"]
mod primitive_shapes;
#[allow(dead_code)] // Only SIZE serves this case.
#[path = "support/raytrace_scene.rs"]
mod raytrace_scene;
#[path = "support/shift_control.rs"]
mod shift_control;
#[path = "case_host/snap_pixels.rs"]
mod snap_pixels;
#[allow(dead_code)] // Only the frame recorder serves the snap coverage rule here.
#[path = "case_host/tie_edges.rs"]
mod tie_edges;
#[allow(dead_code)] // Only the dump run, registration and grading serve this case.
#[path = "support/user_program_scene.rs"]
mod user_program_scene;
#[allow(dead_code)] // Only the session, display and program helpers.
#[path = "support/user_program_session.rs"]
mod user_program_session;
#[path = "support/view_snap_pixels.rs"]
mod view_snap_pixels;
use martensite_tkernel::ncollection::{Mat3, Mat4, Vec2, Vec3, Vec4};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::Graphic3dRenderDriver;
use user_program_scene::*;
use user_program_session::*;

const NAMES: [&str; 4] = [
    "up-blend-unordered",
    "up-blend-weighted",
    "up-blend-peeling",
    "up-blend-peeling2",
];
macro_rules! native_dumps {
    ($device:literal) => {
        [
            include_bytes!(concat!(
                "fixtures/user_programs/",
                $device,
                "/up-blend-unordered.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/user_programs/",
                $device,
                "/up-blend-weighted.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/user_programs/",
                $device,
                "/up-blend-peeling.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/user_programs/",
                $device,
                "/up-blend-peeling2.ppm"
            ))
            .as_slice(),
        ]
    };
}
const NATIVE: NativeDumps = NativeDumps {
    names: &NAMES,
    llvmpipe: &native_dumps!("llvmpipe"),
    amd: &native_dumps!("amd"),
    dump_dir_variable: "MARTENSITE_USER_PROGRAM_PEELING_DUMP_DIR",
};

const UNIFORMS_KEY: &str = "ced4d8ee5ebda03a";
const UNIFORMS_SPIRV: [&[u8]; 2] = [
    include_bytes!("fixtures/user_programs/spirv/ced4d8ee5ebda03a.vert.spv"),
    include_bytes!("fixtures/user_programs/spirv/ced4d8ee5ebda03a.frag.spv"),
];

/// The uniforms program with a half-transparent uColor.
fn blended_program() -> Handle<Graphic3dShaderProgram> {
    let program = source_program(
        "#version 110",
        &[
            (
                Graphic3dTypeOfShaderObject::Vertex,
                include_str!("fixtures/user_programs/uniforms.vert.glsl"),
            ),
            (
                Graphic3dTypeOfShaderObject::Fragment,
                include_str!("fixtures/user_programs/uniforms.frag.glsl"),
            ),
        ],
    );
    program.push_variable_mat4("uModelTrsf", Mat4::default());
    program.push_variable_mat3("uNormTrsf", Mat3::default());
    program.push_variable_vec4("uColor", Vec4::new(0.2, 0.6, 1.0, 0.5));
    program.push_variable_vec3("uLightDir", Vec3::new(0., 0., 1.));
    program.push_variable_vec2("uScale", Vec2::new(1., 1.));
    Handle::new(program)
}

/// Set the OIT method, with `layers` peeling layers; weighted keeps the default factor 0.
fn oit(session: &UserProgramSession, method: Graphic3dRenderTransparentMethod, layers: i32) {
    session.view().change_rendering_params(&mut |params| {
        params.transparency_method = method;
        params.nb_oit_depth_peeling_layers = layers;
    });
}

fn peeling_dumps(driver: &Handle<Graphic3dRenderDriver>, run: &DumpRun) -> Vec<Vec<u8>> {
    let program = blended_program();
    register(driver, &program, UNIFORMS_KEY, &UNIFORMS_SPIRV);
    let session = UserProgramSession::new(driver);
    session.shaded_by_default();
    session.axo();
    configure_peeling_boxes(&session, &program);
    use Graphic3dRenderTransparentMethod::{BlendOit, BlendUnordered, DepthPeelingOit};
    let mut dumps = Vec::new();
    for (method, layers) in [
        (BlendUnordered, 4),
        (BlendOit, 4),
        (DepthPeelingOit, 4),
        (DepthPeelingOit, 2),
    ] {
        oit(&session, method, layers);
        dumps.push(session.dump(run));
    }
    dumps
}

fn configure_peeling_boxes(session: &UserProgramSession, program: &Handle<Graphic3dShaderProgram>) {
    let (opaque, programmed) = peeling_objects(session);
    session.fit();
    session.rotate(0.2);
    let red = Color::named(NamedColor::RED);
    session.context.set_color(&opaque, red, false).unwrap();
    session
        .context
        .set_transparency(&programmed, 0.5, false)
        .unwrap();
    set_fill_program(&session.context, &[&programmed], || program.clone());
}

type PeelingObject = Handle<dyn martensite_tkv3d::ais::AisInteractiveObject>;
fn peeling_objects(session: &UserProgramSession) -> (PeelingObject, PeelingObject) {
    let behind = primitive_shapes::box_at([-1., -1., -1.], [4., 4., 0.5]).unwrap();
    let opaque = session.display(&behind);
    let programmed = session.display(&user_program_scene::unit_box());
    (opaque, programmed)
}

#[test]
fn a_gl_frag_color_program_draws_its_colour_into_every_oit_target() {
    let (device, report) = graded_dumps(&NATIVE, peeling_dumps);
    let failing = grading::unrecorded_pixels(&report, &device, &[]);
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps or approved residues: {failing:?}"
    );
}
