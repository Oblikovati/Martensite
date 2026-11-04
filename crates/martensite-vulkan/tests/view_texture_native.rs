//! `RenderTexture` answers texture queries on a view framebuffer's images (colour and depth of a
//! `Graphic3dCView::FBOCreate` framebuffer): validity, size, samples, mipmaps, alpha, row order,
//! point sprite and revision, after creation, after a viewport change and for small requests.
//!
//! Expected lines come from the recorded view_texture capture, identical on llvmpipe and
//! AMD radeonsi, kept in fixtures/view_texture/native.txt run with
//! DISPLAY=:1). The GL target, format and sized-format enumerations, the GL pixel size and the
//! memory estimate are removed before comparing: they name GL objects (native
//! `PixelSizeOfPixelFormat` does not know GL_SRGB8_ALPHA8 and counts it as 1 byte).
#[allow(dead_code)]
#[path = "support/recording_device.rs"]
mod recording_device;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::standard::{Handle, StandardTransient};
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::v3d::V3dViewer;
use martensite_vulkan::*;

const NATIVE: &str = include_str!("fixtures/view_texture/native.txt");
const GL_FIELDS: [&str; 5] = ["target", "format", "sized", "bytes", "pixel"];

fn without_gl_fields(line: &str) -> String {
    line.split(' ')
        .filter(|word| {
            !GL_FIELDS
                .iter()
                .any(|key| word.starts_with(&format!("{key}=")))
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn texture(label: &str, texture: Option<RenderTexture>) -> String {
    let Some(t) = texture else {
        return format!("texture {label} null");
    };
    let [x, y, z] = t.size();
    format!(
        "texture {label} valid={} size={x},{y},{z} samples={} max_mip={} mipmaps={} alpha={} \
         top_down={} point_sprite={} revision={}",
        i32::from(t.is_valid()),
        t.nb_samples(),
        t.max_mipmap_level(),
        i32::from(t.has_mipmaps()),
        i32::from(t.is_alpha()),
        i32::from(t.is_top_down()),
        i32::from(t.is_point_sprite()),
        t.revision(),
    )
}

fn textures(label: &str, target: &Handle<StandardTransient>) -> Vec<String> {
    let handle = target.cast::<RenderFrameBuffer>();
    let fbo = handle.get().expect("a driver framebuffer");
    vec![
        format!("fbo {label} colors={}", fbo.nb_color_buffers()),
        texture(&format!("{label}_color"), fbo.color_texture(0)),
        texture(&format!("{label}_depth"), Some(fbo.depth_stencil_texture())),
    ]
}

#[test]
fn framebuffer_textures_follow_the_native_queries() {
    let device = recording_device::RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(device, 128).unwrap();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let v3d = viewer.create_view().unwrap();
    view_setup::configure_view(&v3d);
    let cview: Handle<dyn Graphic3dCView> = v3d.get().unwrap().view();
    let view: &dyn Graphic3dCView = cview.get().unwrap();
    let target = view.fbo_create(64, 48);
    let mut lines = textures("created", &target);
    view.fbo_change_viewport(&target, 32, 16);
    lines.extend(textures("smaller", &target));
    lines.extend(textures("zero", &view.fbo_create(0, 0)));
    lines.extend(textures("one", &view.fbo_create(1, 1)));
    let expected: Vec<String> = NATIVE.lines().map(without_gl_fields).collect();
    for (actual, expected) in lines.iter().zip(&expected) {
        assert_eq!(actual, expected);
    }
    assert_eq!(lines.len(), expected.len());
}

#[test]
fn only_the_first_colour_texture_exists() {
    let device = recording_device::RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(device, 128).unwrap();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let v3d = viewer.create_view().unwrap();
    view_setup::configure_view(&v3d);
    let cview: Handle<dyn Graphic3dCView> = v3d.get().unwrap().view();
    let target = cview.get().unwrap().fbo_create(8, 8);
    let fbo = target.cast::<RenderFrameBuffer>();
    assert!(fbo.get().unwrap().color_texture(1).is_none());
}
