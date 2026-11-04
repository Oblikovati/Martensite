//! Registered user programs against recorded dumps: the
//! opengl/data/shaderprog/uniforms case, whose inline GLSL replays from the SPIR-V compiled
//! offline by the recording tool and keyed by the hash of its sources (user decision
//! 2026-10-28, "User shader programs: SPIR-V only"), and the typed refusal of GLSL source that
//! has no registered SPIR-V.
//!
//! The reference scene user_programs/user_programs was recorded
//! on llvmpipe (SSE2/128) and AMD radeonsi.
#[path = "support/native_dump_grading.rs"]
mod grading;
#[allow(dead_code)] // Only the view, fit and dump helpers serve these cases.
#[path = "support/raytrace_scene.rs"]
mod raytrace_scene;
#[path = "support/shift_control.rs"]
mod shift_control;
#[path = "case_host/snap_pixels.rs"]
mod snap_pixels;
#[allow(dead_code)] // Only the frame recorder serves the snap coverage rule here.
#[path = "case_host/tie_edges.rs"]
mod tie_edges;
#[path = "support/user_program_scene.rs"]
mod user_program_scene;
#[path = "support/view_snap_pixels.rs"]
mod view_snap_pixels;
use martensite_tkernel::ncollection::{Mat3, Mat4, Vec2, Vec3, Vec4};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::Graphic3dRenderDriver;
use user_program_scene::*;

macro_rules! native_dumps {
    ($device:literal) => {
        [
            include_bytes!(concat!(
                "fixtures/user_programs/",
                $device,
                "/up-normal.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/user_programs/",
                $device,
                "/up-identity.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/user_programs/",
                $device,
                "/up-translated.ppm"
            ))
            .as_slice(),
        ]
    };
}
const NATIVE: NativeDumps = NativeDumps {
    names: &["up-normal", "up-identity", "up-translated"],
    llvmpipe: &native_dumps!("llvmpipe"),
    amd: &native_dumps!("amd"),
    dump_dir_variable: "MARTENSITE_USER_PROGRAMS_DUMP_DIR",
};

const UNIFORMS_KEY: &str = "ced4d8ee5ebda03a";
const UNIFORMS_SPIRV: [&[u8]; 2] = [
    include_bytes!("fixtures/user_programs/spirv/ced4d8ee5ebda03a.vert.spv"),
    include_bytes!("fixtures/user_programs/spirv/ced4d8ee5ebda03a.frag.spv"),
];

/// uModelTrsf, uColor, uLightDir and the uniform uScale of one program.
type UniformVariables = (Mat4<f32>, [f32; 4], [f32; 3], f32);

/// The vertex and fragment program (header `#version 110`) with the program variables.
fn uniforms_program(variables: UniformVariables) -> Handle<Graphic3dShaderProgram> {
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
    let (model, color, light, scale) = variables;
    program.push_variable_mat4("uModelTrsf", model);
    program.push_variable_mat3("uNormTrsf", Mat3::default());
    program.push_variable_vec4("uColor", Vec4::new(color[0], color[1], color[2], color[3]));
    program.push_variable_vec3("uLightDir", Vec3::new(light[0], light[1], light[2]));
    program.push_variable_vec2("uScale", Vec2::new(scale, scale));
    Handle::new(program)
}

/// uModelTrsf with rows as written, the translation `x`.
fn translated(x: f32) -> Mat4<f32> {
    let mut matrix = Mat4::default();
    matrix.set_value(0, 3, x);
    matrix
}

fn user_program_dumps(driver: &Handle<Graphic3dRenderDriver>, run: &DumpRun) -> Vec<Vec<u8>> {
    let probe = uniforms_program((Mat4::default(), [0.; 4], [0.; 3], 1.));
    register(driver, &probe, UNIFORMS_KEY, &UNIFORMS_SPIRV);
    let (scene, object) = shaded_scene(driver, &unit_box(), run);
    let mut dumps = vec![run.dump(&scene)];
    let identity = (Mat4::default(), [0.2, 0.6, 1.0, 1.0], [0., 0., 1.], 1.);
    apply_program(&scene, &object, &uniforms_program(identity));
    dumps.push(run.dump(&scene));
    let moved = (translated(0.5), [1.0, 0.4, 0.2, 1.0], [1., 1., 1.], 0.8);
    apply_program(&scene, &object, &uniforms_program(moved));
    dumps.push(run.dump(&scene));
    dumps
}

#[test]
fn registered_user_programs_match_the_native_dumps() {
    let (device, report) = graded_dumps(&NATIVE, user_program_dumps);
    let failing = grading::unrecorded_pixels(&report, &device, &[]);
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps or approved residues: {failing:?}"
    );
}

#[test]
fn unregistered_glsl_source_is_refused_with_the_typed_failure() {
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let (scene, object) = shaded_scene(&driver, &unit_box(), &DumpRun::plain());
    let program = uniforms_program((Mat4::default(), [1.; 4], [0., 0., 1.], 1.));
    apply_program(&scene, &object, &program);
    let render: Handle<martensite_vulkan::Graphic3dRenderView> =
        scene.view.get().unwrap().view().cast();
    let error = render.try_redraw().err();
    let failure = error
        .as_ref()
        .and_then(|error| error.user_program_failure());
    assert!(
        matches!(
            failure,
            Some(martensite_vulkan::RenderUserProgramFailure::GlslSourceUnsupported { key })
                if key.to_string() == UNIFORMS_KEY
        ),
        "expected GlslSourceUnsupported for {UNIFORMS_KEY}, got {error:?}"
    );
}
