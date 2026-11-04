//! `RenderFrameBuffer` follows a view's offscreen framebuffer (`Graphic3dCView` FBO calls and the
//! `GlFrameBuffer` queries): allocated, viewport and initial sizes, viewport changes
//! inside, beyond and below the allocation, refused sizes, release and foreign handles.
//!
//! Expected lines come from the recorded view_framebuffer capture, identical on llvmpipe
//! and AMD radeonsi, kept in fixtures/view_framebuffer/native.txt run
//! with DISPLAY=:1). The hardware limit there is 16384; this driver's image limit is 128, so the
//! refused size is checked against it. Native `EstimatedDataSize` counts its GL textures
//! (`bytes=`); the Vulkan images are allocated per captured frame and are not compared.
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

const NATIVE: &str = include_str!("fixtures/view_framebuffer/native.txt");

fn dims(label: &str, view: &dyn Graphic3dCView, fbo: &Handle<StandardTransient>) -> String {
    // Native leaves the outputs untouched (-7) for a null or foreign handle.
    let d = if fbo.cast::<RenderFrameBuffer>().is_null() {
        [-7; 4]
    } else {
        view.fbo_get_dimensions(fbo)
    };
    format!("dims {label} {} {} {} {}", d[0], d[1], d[2], d[3])
}

/// The probe line without the native GL texture estimate (`bytes=`).
fn fbo(label: &str, fbo: &Handle<StandardTransient>) -> String {
    let handle = fbo.cast::<RenderFrameBuffer>();
    let Some(f) = handle.get() else {
        return format!("fbo {label} null");
    };
    format!(
        "fbo {label} size={},{} vp={},{} init={},{} valid={} samples={} colors={} color={} \
         depth={} color_rb={} depth_rb={}",
        f.size_x(),
        f.size_y(),
        f.viewport_size_x(),
        f.viewport_size_y(),
        f.init_viewport_size_x(),
        f.init_viewport_size_y(),
        i32::from(f.is_valid()),
        f.nb_samples(),
        f.nb_color_buffers(),
        i32::from(f.has_color()),
        i32::from(f.has_depth()),
        i32::from(f.is_color_render_buffer()),
        i32::from(f.is_depth_stencil_render_buffer()),
    )
}

fn without_bytes(line: &str) -> String {
    match line.find(" bytes=") {
        Some(start) => {
            let end = line[start + 1..]
                .find(' ')
                .map_or(line.len(), |e| start + 1 + e);
            format!("{}{}", &line[..start], &line[end..])
        }
        None => line.to_owned(),
    }
}

#[test]
fn view_framebuffers_follow_the_native_queries() {
    let device = recording_device::RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(device, 128).unwrap();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let v3d = viewer.create_view().unwrap();
    view_setup::configure_view(&v3d);
    let cview: Handle<dyn Graphic3dCView> = v3d.get().unwrap().view();
    let view: &dyn Graphic3dCView = cview.get().unwrap();
    let mut lines = vec![];
    let mut target = view.fbo_create(64, 48);
    for (label, size) in [
        ("created", None),
        ("smaller", Some((32, 16))),
        ("larger", Some((128, 100))),
        ("non_positive", Some((0, -1))),
    ] {
        if let Some((w, h)) = size {
            view.fbo_change_viewport(&target, w, h);
        }
        lines.push(fbo(label, &target));
        lines.push(dims(label, view, &target));
    }
    view.fbo_release(&mut target);
    lines.push(format!("released null={}", i32::from(target.is_null())));
    lines.push(dims("released", view, &target));
    lines.push(fbo("zero", &view.fbo_create(0, 0)));
    lines.push(fbo("negative", &view.fbo_create(-4, 8)));
    lines.push(fbo("huge", &view.fbo_create(129, 8)));
    lines.push(fbo("one", &view.fbo_create(1, 1)));
    let mut other = Handle::new(StandardTransient::default());
    lines.push(dims("foreign", view, &other));
    view.fbo_release(&mut other);
    lines.push(format!(
        "foreign_released null={}",
        i32::from(other.is_null())
    ));
    let expected: Vec<String> = NATIVE.lines().map(without_bytes).collect();
    for (actual, expected) in lines.iter().zip(&expected) {
        assert_eq!(actual, expected);
    }
    assert_eq!(lines.len(), expected.len());
}
