//! The statistics overlay (GlFrameStatsPrs) against recorded dumps.
//!
//! The reference scene frame_stats_hud/hud_empty was recorded
//! on llvmpipe and on AMD radeonsi; each dump repeats byte-identically. The overlay shows the
//! structure, group and triangle counters of an empty 320x240 view at the native top-left
//! corner and text height, over the default black background.
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::{AspectNeutralWindow, AspectWindowMembers as _};
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tkv3d::v3d::V3dViewer;
use martensite_vulkan::{Graphic3dRenderDriver, VulkanOptions};

const NATIVE_LLVMPIPE: &[u8] = include_bytes!("fixtures/frame_stats_hud/hud-empty-llvmpipe.ppm");
const NATIVE_AMD: &[u8] = include_bytes!("fixtures/frame_stats_hud/hud-empty-amd.ppm");
const SIZE: (usize, usize) = (320, 240);

/// The RGB rows of a P6 dump; `ppm_rgb(bytes)`.
fn ppm_rgb(bytes: &[u8]) -> &[u8] {
    let header = format!("P6\n{} {}\n255\n", SIZE.0, SIZE.1);
    assert!(
        bytes.starts_with(header.as_bytes()),
        "expected a {header:?} header"
    );
    &bytes[header.len()..]
}

/// Performance update interval -1 with the structure, group and triangle counters
/// on an empty view, two repaints, then the RGB dump; `(device, rgb)`.
fn hud_dump() -> (String, Vec<u8>) {
    let options = VulkanOptions {
        validation: true,
        device_index: None,
    };
    let driver = Graphic3dRenderDriver::new(options).unwrap();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let view = viewer.create_view().unwrap();
    let view = view.get().unwrap();
    view.set_immediate_update(false);
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_virtual(true);
    window.set_size(SIZE.0 as i32, SIZE.1 as i32);
    view.set_window(&window.cast(), 0).unwrap();
    view.set_background_color(Color::new(0., 0., 0., ColorSpace::Rgb).unwrap());
    view.change_rendering_params(&mut |params| {
        params.stats.stats_update_interval = -1.;
        type Perf = Graphic3dRenderingParamsPerfCounters;
        params.stats.collected_stats = Graphic3dRenderingParamsPerfCounters(
            Perf::STRUCTURES.0 | Perf::GROUPS.0 | Perf::TRIANGLES.0,
        );
        params.stats.to_show_stats = true;
    });
    view.redraw();
    view.redraw();
    let image = ImagePixMap::default();
    image.set_top_down(true);
    assert!(image.init_zero(ImageFormat::Rgb, SIZE.0, SIZE.1, 0, 0));
    assert!(view.view().buffer_dump(&image, &Graphic3dBufferType::Rgb));
    let storage = image.data();
    let rgb = (0..SIZE.1)
        .flat_map(|y| storage.data.row(y).unwrap()[..SIZE.0 * 3].to_vec())
        .collect();
    let device = driver.device_name();
    view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    (device, rgb)
}

#[test]
fn shown_counters_overlay_matches_the_native_dump() {
    let (device, rgb) = hud_dump();
    let native = ppm_rgb(if device.contains("llvmpipe") {
        NATIVE_LLVMPIPE
    } else {
        NATIVE_AMD
    });
    let differing = rgb
        .as_chunks::<3>()
        .0
        .iter()
        .zip(native.as_chunks::<3>().0)
        .filter(|(a, b)| a != b)
        .count();
    println!("{{\"stats_hud_device\":{device:?},\"differing_pixels\":{differing}}}");
    assert_eq!(
        differing, 0,
        "device={device}; expected the native overlay byte for byte"
    );
}
