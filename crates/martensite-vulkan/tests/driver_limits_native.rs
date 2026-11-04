//! GlGraphicDriver::InquireLimit and the GlContext getters behind it, on real devices.
//!
//! Native values come from the recorded driver_limits capture on llvmpipe
//! and AMD radeonsi, kept in fixtures/driver_limits/native.txt run with
//! DISPLAY=:1, LIBGL_ALWAYS_SOFTWARE=1 for llvmpipe). Limits whose OpenGL and Vulkan meanings
//! agree must equal the native value on both drivers. Combined texture units and the MSAA
//! ceiling are device budgets that differ between APIs; they are checked against the Vulkan
//! limits that `vulkaninfo` reported for the same devices on 2026-10-27.
#[path = "../examples/support/device_options.rs"]
mod device_options;
use martensite_tkservice::graphic3d::{Graphic3dGraphicDriver, Graphic3dTypeOfLimit as Limit};
use martensite_vulkan::Graphic3dRenderDriver;

const NATIVE: &str = include_str!("fixtures/driver_limits/native.txt");
/// VkDriverId values of the qualified drivers.
const DRIVER_ID_MESA_RADV: u32 = 3;
const DRIVER_ID_MESA_LLVMPIPE: u32 = 13;

/// One native `<phase> <name>=<value>` entry; both native drivers must agree on it.
fn native_value(phase: &str, name: &str) -> i32 {
    let key = format!("{phase} {name}=");
    let values: Vec<i32> = NATIVE
        .lines()
        .filter_map(|line| line.strip_prefix(&key))
        .map(|value| value.parse().expect("integer native value"))
        .collect();
    assert_eq!(
        values.len(),
        2,
        "received {values:?} for {key}; expected sw and host"
    );
    assert_eq!(values[0], values[1], "native drivers disagree on {key}");
    values[0]
}

/// The `<phase> <name>=` entry of one native section: `sw` (llvmpipe) or `host` (radeonsi).
fn native_section_value(section: &str, phase: &str, name: &str) -> i32 {
    let start = NATIVE
        .find(&format!("== {section}\n"))
        .unwrap_or_else(|| panic!("native section {section}"));
    let key = format!("{phase} {name}=");
    NATIVE[start..]
        .lines()
        .find_map(|line| line.strip_prefix(&key))
        .and_then(|value| value.parse().ok())
        .unwrap_or_else(|| panic!("received no integer {key} in {section}; expected one"))
}

/// Vulkan budgets recorded by `vulkaninfo`: (combined image samplers, highest MSAA count).
fn vulkan_budget(driver_id: u32) -> (i32, i32) {
    match driver_id {
        DRIVER_ID_MESA_LLVMPIPE => (1_000_000, 8),
        DRIVER_ID_MESA_RADV => (8_388_606, 8),
        other => panic!("received driver id {other}; expected lavapipe (13) or RADV (3)"),
    }
}

#[test]
fn shared_meaning_limits_equal_the_native_driver() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    for (limit, name) in [
        (Limit::MaxNbLights, "MaxNbLights"),
        (Limit::MaxNbClipPlanes, "MaxNbClipPlanes"),
        (Limit::MaxNbViews, "MaxNbViews"),
        (Limit::MaxTextureSize, "MaxTextureSize"),
        (Limit::MaxViewDumpSizeX, "MaxViewDumpSizeX"),
        (Limit::MaxViewDumpSizeY, "MaxViewDumpSizeY"),
        (Limit::HasPBR, "HasPBR"),
        (Limit::HasSRGB, "HasSRGB"),
        (Limit::HasBlendedOit, "HasBlendedOit"),
        (Limit::HasFlatShading, "HasFlatShading"),
        (Limit::IsWorkaroundFBO, "IsWorkaroundFBO"),
        (Limit::HasMeshEdges, "HasMeshEdges"),
        (Limit::HasRayTracing, "HasRayTracing"),
        (
            Limit::HasRayTracingAdaptiveSampling,
            "HasRayTracingAdaptiveSampling",
        ),
    ] {
        let expected = native_value("context", name);
        assert_eq!(driver.inquire_limit(limit), expected, "{name}");
    }
    assert!(driver.validation_errors().unwrap().is_empty());
}

/// The per-driver ray-tracing flags equal the native driver of the same device. The atomic
/// adaptive-sampling flag is the device's float image atomics: llvmpipe has
/// GL_NV_shader_atomic_float and lavapipe shaderImageFloat32AtomicAdd, radeonsi and RADV
/// neither. HasRayTracingTextures is bindless textures natively: llvmpipe 0 and radeonsi 1;
/// lavapipe reports 0 and RADV 1 (user decision 2026-10-29, ray-tracing textures per driver).
#[test]
fn per_driver_ray_tracing_flags_equal_the_native_driver_of_the_same_device() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let section = match driver.device_limits().map(|_| driver.device_name()) {
        Some(name) if name.contains("llvmpipe") => "sw",
        _ => "host",
    };
    for (limit, name) in [
        (
            Limit::HasRayTracingAdaptiveSamplingAtomic,
            "HasRayTracingAdaptiveSamplingAtomic",
        ),
        (Limit::HasRayTracingTextures, "HasRayTracingTextures"),
    ] {
        let expected = native_section_value(section, "context", name);
        assert_eq!(driver.inquire_limit(limit), expected, "{section} {name}");
    }
}

#[test]
fn context_getters_equal_the_native_context() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let limits = driver.device_limits().expect("a Vulkan device");
    for (value, name) in [
        (limits.max_clip_planes(), "MaxClipPlanes"),
        (limits.max_texture_size(), "MaxTextureSize"),
        (limits.max_dump_size_x(), "MaxDumpSizeX"),
        (limits.max_dump_size_y(), "MaxDumpSizeY"),
        (limits.max_color_attachments(), "MaxColorAttachments"),
        (limits.max_draw_buffers(), "MaxDrawBuffers"),
        (limits.max_degree_of_anisotropy(), "MaxDegreeOfAnisotropy"),
        (i32::from(limits.has_srgb()), "HasSRGB"),
        (
            i32::from(limits.has_texture_multisampling()),
            "HasTextureMultisampling",
        ),
        (i32::from(limits.has_flat_shading()), "hasFlatShading"),
        (i32::from(limits.has_mesh_edges()), "hasGeometryStage"),
    ] {
        assert_eq!(value, native_value("getter", name), "{name}");
    }
    assert!(driver.validation_errors().unwrap().is_empty());
}

#[test]
fn device_budgets_equal_the_vulkan_profile() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let (combined, msaa) = vulkan_budget(driver.driver_identity().unwrap().driver_id);
    let limits = driver.device_limits().expect("a Vulkan device");
    assert_eq!(limits.max_combined_texture_units(), combined);
    assert_eq!(
        driver.inquire_limit(Limit::MaxCombinedTextureUnits),
        combined
    );
    assert_eq!(limits.max_msaa_samples(), msaa);
    assert_eq!(driver.inquire_limit(Limit::MaxMsaa), msaa);
    assert!(driver.validation_errors().unwrap().is_empty());
}

/// Named fake backend: it has no Vulkan device and must never be asked to render.
struct DevicelessBackend;
impl martensite_vulkan::RenderBackend for DevicelessBackend {
    fn render(
        &mut self,
        _: &martensite_vulkan::RenderFrame,
    ) -> Result<martensite_vulkan::FrameCapture, martensite_vulkan::RenderError> {
        panic!("a limit query submitted a frame; expected property-only lookup")
    }
}

#[test]
fn deviceless_driver_answers_like_the_native_null_context() {
    let driver = Graphic3dRenderDriver::with_backend(DevicelessBackend, 64).unwrap();
    assert!(driver.device_limits().is_none());
    for (limit, name) in [
        (Limit::MaxNbLights, "MaxNbLights"),
        (Limit::MaxNbClipPlanes, "MaxNbClipPlanes"),
        (Limit::MaxNbViews, "MaxNbViews"),
        (Limit::MaxCombinedTextureUnits, "MaxCombinedTextureUnits"),
        (Limit::MaxMsaa, "MaxMsaa"),
        (Limit::HasPBR, "HasPBR"),
        (Limit::HasSRGB, "HasSRGB"),
        (Limit::HasBlendedOit, "HasBlendedOit"),
        (Limit::HasFlatShading, "HasFlatShading"),
        (Limit::IsWorkaroundFBO, "IsWorkaroundFBO"),
        (Limit::HasMeshEdges, "HasMeshEdges"),
    ] {
        assert_eq!(
            driver.inquire_limit(limit),
            native_value("no_context", name),
            "{name}"
        );
    }
}
