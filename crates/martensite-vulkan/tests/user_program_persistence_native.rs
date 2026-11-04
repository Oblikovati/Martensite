//! User programs under transform persistence against recorded dumps.
//!
//! The reference scene user_programs/user_programs_trsfpers was recorded on llvmpipe (SSE2/128) and AMD radeonsi: a 40x60x80 box, fitted and rotated, made
//! zoom-persistent about the origin, drawn without and with the uniforms program of
//! The reference scene user_programs, zoomed out, then made zoom-rotate-persistent about (20, 0, 0).
//! GlStructure::Render applies Graphic3dTransformPers to the context's model-world,
//! world-view and projection before the program reads occModelWorldMatrix,
//! occWorldViewMatrix and occProjectionMatrix, so the program sees the persistent matrices.
#[path = "support/native_dump_grading.rs"]
mod grading;
#[allow(dead_code)] // Only the view, fit and dump helpers serve this case.
#[path = "support/raytrace_scene.rs"]
mod raytrace_scene;
#[path = "support/shift_control.rs"]
mod shift_control;
#[path = "case_host/snap_pixels.rs"]
mod snap_pixels;
#[allow(dead_code)] // Only the frame recorder serves the snap coverage rule here.
#[path = "case_host/tie_edges.rs"]
mod tie_edges;
#[allow(dead_code)] // Only the scene, program and grading helpers.
#[path = "support/user_program_scene.rs"]
mod user_program_scene;
#[path = "support/view_snap_pixels.rs"]
mod view_snap_pixels;
use martensite_tkernel::ncollection::{Mat3, Mat4, Vec2, Vec3, Vec4};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::*;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::AisInteractiveObject;
use martensite_vulkan::Graphic3dRenderDriver;
use user_program_scene::*;

const NAMES: [&str; 4] = [
    "up-zoom-normal",
    "up-zoom-program",
    "up-zoom-zoomed",
    "up-zoomrotate-program",
];

macro_rules! native_dumps {
    ($device:literal) => {
        [
            include_bytes!(concat!(
                "fixtures/user_programs/",
                $device,
                "/up-zoom-normal.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/user_programs/",
                $device,
                "/up-zoom-program.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/user_programs/",
                $device,
                "/up-zoom-zoomed.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/user_programs/",
                $device,
                "/up-zoomrotate-program.ppm"
            ))
            .as_slice(),
        ]
    };
}
const NATIVE: NativeDumps = NativeDumps {
    names: &NAMES,
    llvmpipe: &native_dumps!("llvmpipe"),
    amd: &native_dumps!("amd"),
    dump_dir_variable: "MARTENSITE_USER_PROGRAM_PERSISTENCE_DUMP_DIR",
};

const UNIFORMS_KEY: &str = "ced4d8ee5ebda03a";
const UNIFORMS_SPIRV: [&[u8]; 2] = [
    include_bytes!("fixtures/user_programs/spirv/ced4d8ee5ebda03a.vert.spv"),
    include_bytes!("fixtures/user_programs/spirv/ced4d8ee5ebda03a.frag.spv"),
];

/// A 40x60x80 box.
fn wide_box() -> martensite_tkbrep::topods::Shape {
    let made = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::at_point(
        Point3::new(0., 0., 0.),
        40.,
        60.,
        80.,
    );
    made.unwrap().shape().unwrap().clone()
}

/// The uniforms program, uModelTrsf translated by 10 in x.
fn translated_program() -> Handle<Graphic3dShaderProgram> {
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
    let mut model = Mat4::default();
    model.set_value(0, 3, 10.);
    program.push_variable_mat4("uModelTrsf", model);
    program.push_variable_mat3("uNormTrsf", Mat3::default());
    program.push_variable_vec4("uColor", Vec4::new(0.2, 0.6, 1.0, 1.0));
    program.push_variable_vec3("uLightDir", Vec3::new(0., 0., 1.));
    program.push_variable_vec2("uScale", Vec2::new(1., 1.));
    Handle::new(program)
}

/// Apply transform persistence `mode` anchored at (x, y, z) to the displayed box.
fn persist(
    scene: &raytrace_scene::RaytraceScene,
    object: &Handle<dyn AisInteractiveObject>,
    mode: Graphic3dTransModeFlags,
    anchor: Point3,
) {
    let pers = Graphic3dTransformPers::with_point(mode, &anchor).unwrap();
    let pers: Handle<dyn Graphic3dTransformPersSource> = Handle::new(pers).cast();
    scene
        .context
        .set_transform_persistence(object, &pers)
        .unwrap();
}

/// A dump: V3dView::Redraw fits the depth range to the persistent bounds (AutoZFit) before
/// the frame is drawn.
fn vdump(scene: &raytrace_scene::RaytraceScene, run: &DumpRun) -> Vec<u8> {
    scene.view().auto_z_fit().unwrap();
    run.dump(scene)
}

fn persistence_dumps(driver: &Handle<Graphic3dRenderDriver>, run: &DumpRun) -> Vec<Vec<u8>> {
    let program = translated_program();
    register(driver, &program, UNIFORMS_KEY, &UNIFORMS_SPIRV);
    let (scene, object) = shaded_scene(driver, &wide_box(), run);
    let zoom = Graphic3dTransModeFlags::ZOOM_PERS;
    persist(&scene, &object, zoom, Point3::new(0., 0., 0.));
    let mut dumps = vec![vdump(&scene, run)];
    apply_program(&scene, &object, &program);
    dumps.push(vdump(&scene, run));
    scene.view().set_zoom(0.8, true).unwrap();
    dumps.push(vdump(&scene, run));
    let zoom_rotate = Graphic3dTransModeFlags::ZOOM_ROTATE_PERS;
    persist(&scene, &object, zoom_rotate, Point3::new(20., 0., 0.));
    dumps.push(vdump(&scene, run));
    dumps
}

#[test]
fn user_programs_under_transform_persistence_match_the_native_dumps() {
    let (device, report) = graded_dumps(&NATIVE, persistence_dumps);
    let failing = grading::unrecorded_pixels(&report, &device, &[]);
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps or approved residues: {failing:?}"
    );
}
