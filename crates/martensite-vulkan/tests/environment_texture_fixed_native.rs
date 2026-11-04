//! The view's environment texture under the fixed-function option against native dumps.
//!
//! the recorded environment_texture_capture capture ran in its "fixed" mode (ffpEnable) on llvmpipe (SSE2/128) and AMD radeonsi: the environment texture
//! is bound as an ordinary fixed-function texture with GL_SPHERE_MAP generation and GL_DECAL
//! (GlSampler::applyGlobalTextureParams), over fixed-function lighting. The scene and dump
//! sequence are those of environment_texture_native.rs.
#[path = "support/native_dump_grading.rs"]
#[allow(dead_code)]
mod grading;
#[path = "support/environment_texture_scene.rs"]
mod scene;
#[path = "support/sphere_map_rim.rs"]
mod sphere_map_rim;
use martensite_tkernel::standard::Handle;
use martensite_vulkan::{
    Graphic3dRenderDriver, RenderLightingPipeline, RenderMeshEdgePipeline,
    RenderTextureCoordinatePipeline,
};
use scene::{DUMP_NAMES, SIZE, environment_texture_dumps};

macro_rules! native_dumps {
    ($device:literal) => {
        [
            include_bytes!(concat!(
                "fixtures/environment_texture_fixed/",
                $device,
                "/env-unlit.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/environment_texture_fixed/",
                $device,
                "/env-gouraud.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture_fixed/",
                $device,
                "/env-phong.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture_fixed/",
                $device,
                "/env-phong-facet.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture_fixed/",
                $device,
                "/env-pbr.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture_fixed/",
                $device,
                "/env-pbr-facet.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture_fixed/",
                $device,
                "/env-persp.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture_fixed/",
                $device,
                "/env-layer-off.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture_fixed/",
                $device,
                "/env-textured.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture_fixed/",
                $device,
                "/env-cleared.ppm"
            )),
        ]
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 10] = native_dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 10] = native_dumps!("amd");

fn fixed_driver() -> Handle<Graphic3dRenderDriver> {
    Graphic3dRenderDriver::with_render_pipelines(
        grading::validation_options(),
        RenderTextureCoordinatePipeline::GeneratedCompatibility,
        RenderMeshEdgePipeline::default(),
        RenderLightingPipeline::FixedFunction,
    )
    .unwrap()
}

/// Recorded failures as (device profile, dump, unexplained pixels, largest channel difference
/// among them): a dump may not exceed either; dumps not listed must be explained. Measured
/// 2026-10-29 after the attribute-rounding residue and the sphere-map rim rule (lavapipe):
/// - llvmpipe: one single-level step per dump that the bottom-up control keeps (as in the
///   programmable replay; compile path unproven), two in the perspective dumps.
/// - env-textured: the textured shape's Manual texture inherits GL_SPHERE_MAP from the
///   environment texture bound before it (GlContext::BindTextures applies a new texture
///   without resetting the previous one), so its own texture is sphere-mapped natively too.
/// - AMD (RADV vs radeonsi, cross-driver, no control): after the one-level shading bound, one
///   10-level texel step on the mapped faces of each shading dump and 24 pixels up to 4 levels
///   at the perspective sphere's rim (host sphere-map arithmetic vs radeonsi's FFP program).
const RECORDED: [(&str, &str, usize, u8); 18] = [
    ("llvmpipe", "env-unlit", 1, 1),
    ("llvmpipe", "env-gouraud", 1, 1),
    ("llvmpipe", "env-phong", 1, 1),
    ("llvmpipe", "env-phong-facet", 1, 1),
    ("llvmpipe", "env-pbr", 1, 1),
    ("llvmpipe", "env-pbr-facet", 1, 1),
    ("llvmpipe", "env-persp", 2, 1),
    ("llvmpipe", "env-layer-off", 1, 1),
    ("llvmpipe", "env-textured", 2, 1),
    ("llvmpipe", "env-cleared", 1, 1),
    ("AMD", "env-unlit", 1, 10),
    ("AMD", "env-gouraud", 1, 10),
    ("AMD", "env-phong", 1, 10),
    ("AMD", "env-phong-facet", 1, 10),
    ("AMD", "env-pbr", 1, 10),
    ("AMD", "env-pbr-facet", 1, 10),
    ("AMD", "env-persp", 24, 4),
    ("AMD", "env-textured", 24, 4),
];

/// (dump, unexplained pixels, largest channel difference among them) of every dump: on
/// lavapipe after the attribute-rounding residue and the sphere-map rim rule against the
/// bottom-up control of the same fixed-function pipelines; on AMD after the cross-driver bound.
fn unexplained_dumps(
    dumps: &[Vec<u8>],
    native: &[&[u8]; 10],
    control: Option<&[Vec<u8>]>,
) -> Vec<(&'static str, usize, u8)> {
    let graded = DUMP_NAMES.iter().enumerate().map(|(index, name)| {
        let (rust, expected) = (&dumps[index], grading::ppm_rgb(native[index], SIZE));
        let control = control.map(|dumps| dumps[index].as_slice());
        let (_, _, pixels) = grading::grade_dump(name, rust, expected, control, None, &[], SIZE.0);
        let mapped = |x, y| scene::sphere_mapped(index, (x, y), dumps, native);
        let rim =
            sphere_map_rim::grade_sphere_map_rim(pixels, rust, expected, control, SIZE.0, mapped);
        let largest = rim
            .0
            .iter()
            .flat_map(|p| p.2.iter().zip(&p.3).map(|(a, b)| a.abs_diff(*b)));
        (*name, rim.0.len(), largest.max().unwrap_or(0))
    });
    graded.collect()
}

#[test]
fn fixed_function_environment_texture_matches_the_native_dumps() {
    let driver = fixed_driver();
    let dumps = environment_texture_dumps(&driver);
    grading::write_dumps(
        "MARTENSITE_ENVIRONMENT_FIXED_DUMP_DIR",
        &DUMP_NAMES,
        &dumps,
        SIZE,
    );
    let device = driver.device_name();
    let native = match device.contains("llvmpipe") {
        true => NATIVE_LLVMPIPE,
        false => NATIVE_AMD,
    };
    let pipelines = (
        RenderTextureCoordinatePipeline::GeneratedCompatibility,
        RenderLightingPipeline::FixedFunction,
    );
    let control = grading::pipeline_rounding_control(&driver, pipelines, environment_texture_dumps);
    let report = unexplained_dumps(&dumps, &native, control.as_deref());
    println!("{{\"environment_fixed_device\":{device:?},\"unexplained\":{report:?}}}");
    let unrecorded: Vec<_> = report
        .into_iter()
        .filter(|&(name, count, largest)| {
            let allowed = RECORDED
                .iter()
                .find(|(profile, dump, ..)| device.contains(profile) && *dump == name)
                .map_or((0, 0), |&(_, _, count, delta)| (count, delta));
            count > allowed.0 || largest > allowed.1
        })
        .collect();
    assert!(
        unrecorded.is_empty(),
        "device={device}; unrecorded (dump, pixels, largest): {unrecorded:?}"
    );
}
