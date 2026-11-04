//! User programs with more stages against recorded dumps: the built-in PhongShading
//! sample (opengl/data/shaderprog phong_box and phong_plastic), a
//! geometry stage (opengl/data/geom/geom1) and tessellation plus geometry stages
//! (opengl/data/geom/tess1). The inline GLSL replays from SPIR-V compiled offline by
//! the recording tool and keyed by the hash of its sources; the Phong sample is
//! Martensite's own GLSL (shaders/user), registered in every driver.
//!
//! Recorded on llvmpipe (SSE2/128) and AMD radeonsi; the recorded `phong` loads
//! PhongShading.vs/.fs.
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
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::Graphic3dRenderDriver;
use user_program_scene::*;

macro_rules! native_dumps {
    ($device:literal) => {
        [
            include_bytes!(concat!(
                "fixtures/user_programs/",
                $device,
                "/up-phong-box.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/user_programs/",
                $device,
                "/up-phong-plastic.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!("fixtures/user_programs/", $device, "/up-geom.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/user_programs/", $device, "/up-tess.ppm")).as_slice(),
        ]
    };
}
const NATIVE: NativeDumps = NativeDumps {
    names: &["up-phong-box", "up-phong-plastic", "up-geom", "up-tess"],
    llvmpipe: &native_dumps!("llvmpipe"),
    amd: &native_dumps!("amd"),
    dump_dir_variable: "MARTENSITE_USER_PROGRAM_STAGES_DUMP_DIR",
};

use Graphic3dTypeOfShaderObject::{Fragment, Geometry, TessControl, TessEvaluation, Vertex};
const GEOM_KEY: &str = "878b3f68bf89b8c0";
const GEOM_SPIRV: [&[u8]; 3] = [
    include_bytes!("fixtures/user_programs/spirv/878b3f68bf89b8c0.vert.spv"),
    include_bytes!("fixtures/user_programs/spirv/878b3f68bf89b8c0.geom.spv"),
    include_bytes!("fixtures/user_programs/spirv/878b3f68bf89b8c0.frag.spv"),
];
const TESS_KEY: &str = "bc14106fd1af8455";
const TESS_SPIRV: [&[u8]; 5] = [
    include_bytes!("fixtures/user_programs/spirv/bc14106fd1af8455.vert.spv"),
    include_bytes!("fixtures/user_programs/spirv/bc14106fd1af8455.tesc.spv"),
    include_bytes!("fixtures/user_programs/spirv/bc14106fd1af8455.tese.spv"),
    include_bytes!("fixtures/user_programs/spirv/bc14106fd1af8455.geom.spv"),
    include_bytes!("fixtures/user_programs/spirv/bc14106fd1af8455.frag.spv"),
];

/// The vertex, geometry and fragment program of geom1.
fn geom_program() -> Graphic3dShaderProgram {
    source_program(
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
    )
}

/// The program with every stage, tessellation included, of tess1.
fn tess_program() -> Graphic3dShaderProgram {
    source_program(
        "",
        &[
            (
                Vertex,
                include_str!("fixtures/user_programs/tess1.vert.glsl"),
            ),
            (
                TessControl,
                include_str!("fixtures/user_programs/tess1.tesc.glsl"),
            ),
            (
                TessEvaluation,
                include_str!("fixtures/user_programs/tess1.tese.glsl"),
            ),
            (
                Geometry,
                include_str!("fixtures/user_programs/tess1.geom.glsl"),
            ),
            (
                Fragment,
                include_str!("fixtures/user_programs/tess1.frag.glsl"),
            ),
        ],
    )
}

/// The first face, `b_1`.
fn first_face(shape: &martensite_tkbrep::topods::Shape) -> martensite_tkbrep::topods::Shape {
    let faces = martensite_tkbrep::topexp::Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape);
    faces.unwrap().current().clone()
}

fn phong_dumps(driver: &Handle<Graphic3dRenderDriver>, run: &DumpRun) -> Vec<Vec<u8>> {
    let (scene, object) = shaded_scene(driver, &unit_box(), run);
    let phong = Handle::new(martensite_vulkan::phong_shading_program());
    apply_program(&scene, &object, &phong);
    let mut dumps = vec![run.dump(&scene)];
    let plastic = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Plastified);
    scene
        .context
        .set_material(&object, &plastic, false)
        .unwrap();
    let green = Color::named(NamedColor::GREEN);
    scene.context.set_color(&object, green, false).unwrap();
    dumps.push(run.dump(&scene));
    dumps
}

fn stage_dumps(driver: &Handle<Graphic3dRenderDriver>, run: &DumpRun) -> Vec<Vec<u8>> {
    let mut dumps = phong_dumps(driver, run);
    register(driver, &geom_program(), GEOM_KEY, &GEOM_SPIRV);
    register(driver, &tess_program(), TESS_KEY, &TESS_SPIRV);
    let (scene, object) = shaded_scene(driver, &unit_box(), run);
    apply_program(&scene, &object, &Handle::new(geom_program()));
    dumps.push(run.dump(&scene));
    let (scene, object) = shaded_scene(driver, &first_face(&unit_box()), run);
    apply_program(&scene, &object, &Handle::new(tess_program()));
    dumps.push(run.dump(&scene));
    dumps
}

#[test]
fn phong_geometry_and_tessellation_programs_match_the_native_dumps() {
    let (device, report) = graded_dumps(&NATIVE, stage_dumps);
    let failing = grading::unrecorded_pixels(&report, &device, &[]);
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps or approved residues: {failing:?}"
    );
}
