//! `RenderContextState::dump_json` writes the native `GlContext::DumpJson`
//! for the context a user element sees while it renders: device limits, viewport, polygon
//! offset, face culling, alpha-to-coverage, resolution, render scale and the model-world,
//! world-view and projection stacks of a default V3d view with auto z-fit.
//!
//! Expected lines come from the recorded context_dump_json capture (a white unlit triangle and a
//! dumping element in a 64x64 virtual window), kept in fixtures/context_dump_json/native.txt
//!  run with DISPLAY=:1) on llvmpipe; radeonsi differs only in two ray-tracing
//! flags and the address of the GL colour mask. Before comparing:
//! - the GL-only fields, which the Vulkan dump does not write, are removed from the native lines
//!   (texture clamp and point-sprite origin enumerations, GL version, GL_NORMALIZE, render and
//!   polygon modes, read buffer, default VAO, colour-mask pointer, GL debug context);
//! - the ray-tracing flags are native's on llvmpipe (the recorded driver); on RADV the atomic
//!   adaptive-sampling and the textures flags are radeonsi's, from
//!   fixtures/driver_limits/native.txt (radeonsi has no GL_NV_shader_atomic_float and RADV no
//!   float image atomics; radeonsi has bindless textures and RADV textures ray-traced
//!   materials, user decision 2026-10-29);
//! - the device limits are the selected Vulkan device's, whose agreement with native is
//!   witnessed by driver_limits_native.rs (combined texture units are a different budget).
#[path = "../examples/support/device_options.rs"]
mod device_options;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkservice::aspect::{AspectInteriorStyle, AspectNeutralWindow, AspectWindowMembers};
use martensite_tkservice::graphic3d::{Graphic3dTypeOfPrimitiveArray as Kind, *};
use martensite_tkv3d::v3d::V3dViewer;
use martensite_vulkan::*;
use std::sync::{Arc, Mutex};

const NATIVE: &str = include_str!("fixtures/context_dump_json/native.txt");
const GL_ONLY: [&str; 11] = [
    "TexClamp",
    "GlVerMajor",
    "GlVerMinor",
    "IsGlNormalizeEnabled",
    "PointSpriteOrig",
    "RenderMode",
    "PolygonMode",
    "ReadBuffer",
    "DefaultVao",
    "ColorMask",
    "IsGlDebugCtx",
];
/// The radeonsi context limits of the driver-limits reference (its `== host` section).
const NATIVE_LIMITS: &str = include_str!("fixtures/driver_limits/native.txt");

/// The radeonsi value of a context limit; `radeonsi_limit("HasRayTracing")`.
fn radeonsi_limit(name: &str) -> i32 {
    let host = &NATIVE_LIMITS[NATIVE_LIMITS.find("== host").expect("radeonsi section")..];
    let key = format!("context {name}=");
    host.lines()
        .find_map(|line| line.strip_prefix(&key))
        .and_then(|value| value.parse().ok())
        .unwrap_or_else(|| panic!("received no integer {key}; expected one"))
}

/// The ray-tracing flags that differ from the recorded llvmpipe lines on this device.
fn ray_tracing_fields(device: &str) -> Vec<(&'static str, i32)> {
    if device.contains("llvmpipe") {
        return Vec::new();
    }
    [
        "HasRayTracingTextures",
        "HasRayTracingAdaptiveSamplingAtomic",
    ]
    .into_iter()
    .map(|name| (name, radeonsi_limit(name)))
    .collect()
}

/// Replace (`Some`) or remove (`None`) the numeric field `key` of a flat dump line.
fn set_field(line: &str, key: &str, value: Option<i32>) -> String {
    let pattern = format!(", \"{key}\": ");
    let start = line
        .find(&pattern)
        .unwrap_or_else(|| panic!("native field {key} in {line}"));
    let rest = &line[start + pattern.len()..];
    let end = start + pattern.len() + rest.find([',', '}']).unwrap_or(rest.len());
    let replacement = value.map_or(String::new(), |v| format!("{pattern}{v}"));
    format!("{}{replacement}{}", &line[..start], &line[end..])
}

fn device_fields(limits: &RenderDeviceLimits) -> [(&'static str, i32); 9] {
    [
        ("AnisoMax", limits.max_degree_of_anisotropy()),
        ("MaxTexDim", limits.max_texture_size()),
        ("MaxTexCombined", limits.max_combined_texture_units()),
        ("MaxDumpSizeX", limits.max_dump_size_x()),
        ("MaxDumpSizeY", limits.max_dump_size_y()),
        ("MaxClipPlanes", limits.max_clip_planes()),
        ("MaxMsaaSamples", limits.max_msaa_samples()),
        ("MaxDrawBuffers", limits.max_draw_buffers()),
        ("MaxColorAttachments", limits.max_color_attachments()),
    ]
}

fn native_lines(device: &str, limits: &RenderDeviceLimits) -> Vec<String> {
    NATIVE
        .lines()
        .map(|line| {
            let mut line = line.to_owned();
            line = GL_ONLY.iter().fold(line, |l, key| set_field(&l, key, None));
            line = ray_tracing_fields(device)
                .iter()
                .fold(line, |l, (key, value)| set_field(&l, key, Some(*value)));
            device_fields(limits)
                .iter()
                .fold(line, |l, (key, value)| set_field(&l, key, Some(*value)))
        })
        .collect()
}

/// Dumps the context it sees on its first render, at depth -1 and 1.
#[derive(Default)]
struct DumpingElement {
    lines: Mutex<Vec<String>>,
}
impl RenderElement for DumpingElement {
    fn render(&self, context: &mut RenderElementContext<'_>) -> Result<(), RenderError> {
        let mut lines = self.lines.lock().unwrap();
        if lines.is_empty() {
            for depth in [-1, 1] {
                let mut stream = DumpStream::default();
                context
                    .context_state()
                    .dump_json(&mut stream, depth)
                    .unwrap();
                lines.push(format!("element_context {depth} {}", stream.text()));
            }
        }
        Ok(())
    }
}

fn triangle() -> Handle<Graphic3dArrayOfPrimitives> {
    let array = Handle::new(Graphic3dArrayOfPrimitives::new(Kind::Triangles, 3, 0, 0, 0).unwrap());
    for [x, y] in [[-10., -10.], [10., -10.], [0., 10.]] {
        array.add_vertex(x, y, 0.).unwrap();
    }
    array
}

/// The probe scene on the selected device; returns the element's dump lines.
fn replay(driver: &Handle<Graphic3dRenderDriver>) -> Vec<String> {
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let view = viewer.create_view().unwrap();
    view.set_immediate_update(false);
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_virtual(true);
    window.set_size(64, 64);
    view.set_window(&window.cast(), 0).unwrap();
    let structure =
        Graphic3dStructure::new(&viewer.structure_manager(), &Handle::default()).unwrap();
    let group = structure.new_group().unwrap();
    let fill = Handle::new(Graphic3dAspectFillArea3d::default());
    fill.set_interior_style(AspectInteriorStyle::Solid);
    fill.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    fill.set_interior_color(Color::named(NamedColor::WHITE));
    group.set_group_primitives_aspect(&fill.cast::<dyn Graphic3dAspectsSource>());
    group.add_primitive_array(&triangle(), true);
    let element = Arc::new(DumpingElement::default());
    let draw_group = group.cast::<Graphic3dDrawGroup>();
    draw_group.get().unwrap().add_element(element.clone());
    structure.display();
    // V3dView::Redraw fits Z first; the driver redraw reports what the frame refused.
    view.redraw();
    let cview = view.get().unwrap().view();
    cview.cast::<Graphic3dRenderView>().try_redraw().unwrap();
    element.lines.lock().unwrap().clone()
}

#[test]
fn element_context_dumps_the_native_context_json() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let limits = driver.device_limits().expect("a Vulkan device");
    let expected = native_lines(&driver.device_name(), &limits);
    let actual = replay(&driver);
    assert_eq!(actual.len(), expected.len(), "{actual:#?}");
    for (index, (actual, expected)) in actual.iter().zip(&expected).enumerate() {
        assert_eq!(actual, expected, "dump line {index}");
    }
}
