//! 2D image backgrounds under the fixed-function option against native dumps.
//!
//! the recorded background_image capture ran in its "fixed" mode (ffpEnable) on llvmpipe (SSE2/128) and AMD radeonsi; the scene and dump sequence are those
//! of background_image_native.rs. Natively the fixed-function pipeline renders without sRGB
//! (GlContext::ToRenderSRGB) and draws the image quad with GL_MODULATE by the current
//! colour, which GlPrimitiveArray::Render sets to the background aspect's interior colour:
//! the default Graphic3dAspectFillArea3d colour, cyan, so the image loses its red channel.
//! Vertex gradients interpolate sRGB-encoded corner colours and the corner gradient writes its
//! linear mix unencoded. Both replays are byte-exact.
#[path = "support/native_dump_grading.rs"]
#[allow(dead_code)]
mod grading;
#[path = "support/background_image_scene.rs"]
mod scene;
use martensite_tkernel::standard::Handle;
use martensite_vulkan::{
    Graphic3dRenderDriver, RenderLightingPipeline, RenderMeshEdgePipeline,
    RenderTextureCoordinatePipeline,
};
use scene::{DUMP_NAMES, SIZE, background_dumps};

macro_rules! native_dumps {
    ($device:literal; $($name:literal),*) => {
        [$(include_bytes!(concat!("fixtures/background_image_fixed/", $device, "/", $name, ".ppm"))
            .as_slice()),*]
    };
}
macro_rules! device_dumps {
    ($device:literal) => {
        native_dumps!($device; "image-centered", "image-tiled", "image-stretch", "image-none",
            "image-stretch-tiles", "image-centered-tiles", "gradient-centered",
            "gradient-stretch", "gradient-none", "corner-centered", "image-cleared",
            "gradient-after-clear")
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 12] = device_dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 12] = device_dumps!("amd");

fn fixed_driver() -> Handle<Graphic3dRenderDriver> {
    Graphic3dRenderDriver::with_render_pipelines(
        grading::validation_options(),
        RenderTextureCoordinatePipeline::GeneratedCompatibility,
        RenderMeshEdgePipeline::default(),
        RenderLightingPipeline::FixedFunction,
    )
    .unwrap()
}

/// (dump, differing pixels, largest channel difference) of every dump that differs.
fn differing_dumps(dumps: &[Vec<u8>], native: &[&[u8]; 12]) -> Vec<(&'static str, usize, u8)> {
    let compared = DUMP_NAMES.iter().zip(dumps).zip(native);
    compared
        .map(|((name, rust), expected)| {
            let expected = grading::ppm_rgb(expected, SIZE);
            let pairs = rust
                .chunks(3)
                .zip(expected.chunks(3))
                .filter(|(a, b)| a != b);
            pairs.fold((*name, 0, 0), |(name, count, largest), (a, b)| {
                let delta = a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).max();
                (name, count + 1, largest.max(delta.unwrap_or(0)))
            })
        })
        .filter(|(_, count, _)| *count > 0)
        .collect()
}

#[test]
fn fixed_function_background_images_match_the_native_dumps() {
    let driver = fixed_driver();
    let (dumps, _) = background_dumps(&driver);
    grading::write_dumps(
        "MARTENSITE_BACKGROUND_FIXED_DUMP_DIR",
        &DUMP_NAMES,
        &dumps,
        SIZE,
    );
    let device = driver.device_name();
    let native = match device.contains("llvmpipe") {
        true => NATIVE_LLVMPIPE,
        false => NATIVE_AMD,
    };
    let differing = differing_dumps(&dumps, &native);
    assert!(
        differing.is_empty(),
        "device={device}; differing (dump, pixels, largest): {differing:?}"
    );
}
