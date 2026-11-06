//! The typed refusals of user-program draws the renderer does not link: a blended program
//! whose outputs native leaves undefined in an OIT frame. Native writes `gl_FragColor` (GLSL
//! below 1.30, one output) into every OIT target, which tests/user_program_peeling_native.rs
//! replays; a program with declared outputs writes location 0 only, so without the weighted
//! output its coverage target, and under depth peeling its front and back targets, are
//! undefined. Each frame fails with a RenderError naming the offending state.
#[allow(dead_code)] // Only the shape reader.
#[path = "support/dataset_shape.rs"]
mod dataset_shape;
#[path = "support/native_dump_grading.rs"]
#[allow(dead_code)] // Only the validation options.
mod grading;
#[allow(dead_code)] // Only SIZE.
#[path = "support/raytrace_scene.rs"]
mod raytrace_scene;
#[allow(dead_code)] // Unused by the refusals.
#[path = "support/shift_control.rs"]
mod shift_control;
#[allow(dead_code)] // Unused by the refusals.
#[path = "case_host/snap_pixels.rs"]
mod snap_pixels;
#[allow(dead_code)] // Unused by the refusals.
#[path = "case_host/tie_edges.rs"]
mod tie_edges;
#[allow(dead_code)] // Only the box, program and registration helpers.
#[path = "support/user_program_scene.rs"]
mod user_program_scene;
#[allow(dead_code)] // Only the session, display and program helpers.
#[path = "support/user_program_session.rs"]
mod user_program_session;
#[allow(dead_code)] // Unused by the refusals.
#[path = "support/view_snap_pixels.rs"]
mod view_snap_pixels;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::{Graphic3dRenderDriver, Graphic3dRenderView};
use user_program_scene::{register, source_program, unit_box};
use user_program_session::*;

const GEOM_KEY: &str = "878b3f68bf89b8c0";
const GEOM_SPIRV: [&[u8]; 3] = [
    include_bytes!("fixtures/user_programs/spirv/878b3f68bf89b8c0.vert.spv"),
    include_bytes!("fixtures/user_programs/spirv/878b3f68bf89b8c0.geom.spv"),
    include_bytes!("fixtures/user_programs/spirv/878b3f68bf89b8c0.frag.spv"),
];

/// geom1 of opengl/data/geom: a geometry stage makes native compile it as `#version 150`, so
/// its fragment colour is a declared output at location 0.
fn geom_program() -> Handle<Graphic3dShaderProgram> {
    use Graphic3dTypeOfShaderObject::{Fragment, Geometry, Vertex};
    Handle::new(source_program(
        "",
        &[
            (
                Vertex,
                include_str!("fixtures/user_programs/geom1.vert.glsl"),
            ),
            (
                Geometry,
                include_str!("fixtures/user_programs/geom1.geom.glsl"),
            ),
            (
                Fragment,
                include_str!("fixtures/user_programs/geom1.frag.glsl"),
            ),
        ],
    ))
}

/// A half-transparent shaded box drawn with `program` in a `method` frame; `(session, object)`.
fn blended_box(
    driver: &Handle<Graphic3dRenderDriver>,
    program: fn() -> Handle<Graphic3dShaderProgram>,
    method: Graphic3dRenderTransparentMethod,
) -> UserProgramSession {
    let session = UserProgramSession::new(driver);
    session.shaded_by_default();
    session.axo();
    let object = session.display(&unit_box());
    session.fit();
    set_fill_program(&session.context, &[&object], program);
    let context = &session.context;
    context.set_transparency(&object, 0.5, false).unwrap();
    session.view().change_rendering_params(&mut |params| {
        params.transparency_method = method;
    });
    session
}

/// The error of redrawing the session's view.
fn redraw_error(session: &UserProgramSession) -> String {
    let base = session.view_handle().get().unwrap();
    let render: Handle<Graphic3dRenderView> = base.view().cast();
    let error = render
        .try_redraw()
        .expect_err("a refused user-program frame");
    format!("{error:?}")
}

#[test]
fn a_declared_output_program_without_the_weighted_output_is_refused_in_an_oit_frame() {
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let program = geom_program();
    register(&driver, &program, GEOM_KEY, &GEOM_SPIRV);
    let session = blended_box(
        &driver,
        geom_program,
        Graphic3dRenderTransparentMethod::BlendOit,
    );
    let error = redraw_error(&session);
    assert!(
        error.contains("a blended user program without the weighted OIT output in an OIT frame"),
        "expected the OIT refusal, got {error}"
    );
    // The same blended program draws in a frame without OIT.
    session.view().change_rendering_params(&mut |params| {
        params.transparency_method = Graphic3dRenderTransparentMethod::BlendUnordered;
    });
    let base = session.view_handle().get().unwrap();
    let render: Handle<Graphic3dRenderView> = base.view().cast();
    render
        .try_redraw()
        .expect("a blended user program without OIT");
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
}

#[test]
fn declared_output_programs_are_refused_in_a_depth_peeling_frame() {
    use Graphic3dRenderTransparentMethod::DepthPeelingOit;
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let program = geom_program();
    register(&driver, &program, GEOM_KEY, &GEOM_SPIRV);
    // The weighted-output PhongShading program declares two outputs (draw buffers).
    let weighted = || {
        let program = martensite_vulkan::phong_shading_program();
        program.set_nb_fragment_outputs(2);
        program.set_oit_output(Graphic3dRenderTransparentMethod::BlendOit);
        Handle::new(program)
    };
    for program in [geom_program as fn() -> _, weighted] {
        let session = blended_box(&driver, program, DepthPeelingOit);
        let error = redraw_error(&session);
        assert!(
            error.contains(
                "a blended user program with declared fragment outputs in a depth-peeling frame"
            ),
            "expected the depth-peeling refusal, got {error}"
        );
    }
}
