//! GlView::ToFlipOutput and SetImmediateModeDrawToFront on real Vulkan frames.
//!
//! Expected values come from the recorded view_flip_output capture run on the pinned
//! native OpenGl driver under llvmpipe and AMD radeonsi, which agree: the flag
//! starts false and round-trips; a flipped frame reverses its rows exactly only without MSAA
//! and when an intermediate scene buffer is used (immediate drawing to the front buffer, or
//! OIT); V3dView::ToPixMap turns immediate front drawing off, so its dumps flip only with
//! OIT. Every other case leaves the dump byte-identical. The native output is kept in
//! fixtures/view_flip_output/native.txt (build: g++ -std=c++17 against run with DISPLAY=:1).
#[allow(dead_code)]
#[path = "support/recording_device.rs"]
mod recording_device;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod viewer_scene;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_vulkan::{Graphic3dRenderDriver, VulkanOptions};
use viewer_scene::ViewerScene;

/// One native row: (pixmap dump, msaa, oit, draw to front) -> rows reversed.
type NativeFlipRow = ((bool, i32, bool, bool), bool);

/// The native table, identical on llvmpipe and AMD.
const NATIVE_FLIP_TABLE: [NativeFlipRow; 16] = [
    ((true, 0, false, true), false),
    ((true, 0, false, false), false),
    ((true, 0, true, true), true),
    ((true, 0, true, false), true),
    ((true, 4, false, true), false),
    ((true, 4, false, false), false),
    ((true, 4, true, true), false),
    ((true, 4, true, false), false),
    ((false, 0, false, true), true),
    ((false, 0, false, false), false),
    ((false, 0, true, true), true),
    ((false, 0, true, false), true),
    ((false, 4, false, true), false),
    ((false, 4, false, false), false),
    ((false, 4, true, true), false),
    ((false, 4, true, false), false),
];

fn configure(scene: &ViewerScene, msaa: i32, oit: bool, draw_to_front: bool) {
    let method = if oit {
        Graphic3dRenderTransparentMethod::BlendOit
    } else {
        Graphic3dRenderTransparentMethod::BlendUnordered
    };
    scene
        .view
        .get()
        .unwrap()
        .change_rendering_params(&mut |params| {
            params.nb_msaa_samples = msaa;
            params.transparency_method = method;
        });
    scene
        .cview()
        .set_immediate_mode_draw_to_front(draw_to_front);
}

fn dump(scene: &ViewerScene, pixmap: bool) -> Vec<u8> {
    let image = ImagePixMap::default();
    image.set_top_down(true);
    if pixmap {
        let view = scene.view.get().unwrap();
        assert!(
            view.to_pix_map_sized(&image, 64, 64, Graphic3dBufferType::Rgba, true)
                .unwrap()
        );
    } else {
        scene.render_view().try_redraw().unwrap();
        assert!(image.init_zero(ImageFormat::Rgba, 64, 64, 0, 0));
        assert!(
            scene
                .cview()
                .buffer_dump(&image, &Graphic3dBufferType::Rgba)
        );
    }
    let storage = image.data();
    (0..64)
        .flat_map(|y| storage.data.row(y).unwrap()[..256].to_vec())
        .collect()
}

fn reversed_rows(rows: &[u8]) -> Vec<u8> {
    rows.rchunks_exact(256).flatten().copied().collect()
}

fn assert_native_flip_row(scene: &ViewerScene, row: NativeFlipRow) {
    let ((pixmap, msaa, oit, draw_to_front), reversed) = row;
    configure(scene, msaa, oit, draw_to_front);
    scene.cview().set_to_flip_output(false);
    let plain = dump(scene, pixmap);
    assert_ne!(
        plain,
        reversed_rows(&plain),
        "{row:?}: scene must be vertically asymmetric"
    );
    scene.cview().set_to_flip_output(true);
    let flipped = dump(scene, pixmap);
    scene.cview().set_to_flip_output(false);
    let expected = if reversed {
        reversed_rows(&plain)
    } else {
        plain
    };
    assert!(
        flipped == expected,
        "{row:?}: flipped dump differs from native contract"
    );
}

fn recording_scene() -> ViewerScene {
    let driver =
        Graphic3dRenderDriver::with_backend(recording_device::RecordingDevice::default(), 128)
            .unwrap();
    ViewerScene::new(&driver)
}

#[test]
fn flip_output_starts_false_and_round_trips_like_the_native_view() {
    let scene = recording_scene();
    let view = scene.cview();
    assert!(!view.to_flip_output());
    view.set_to_flip_output(true);
    assert!(view.to_flip_output());
    view.set_to_flip_output(false);
    assert!(!view.to_flip_output());
}

#[test]
fn immediate_draw_to_front_starts_enabled_and_returns_the_previous_mode() {
    let scene = recording_scene();
    let view = scene.cview();
    assert!(view.set_immediate_mode_draw_to_front(false));
    assert!(!view.set_immediate_mode_draw_to_front(true));
    assert!(view.set_immediate_mode_draw_to_front(true));
}

#[test]
fn real_vulkan_flip_output_matches_the_native_dump_table() {
    let options = VulkanOptions {
        validation: true,
        device_index: None,
    };
    let driver: Handle<Graphic3dRenderDriver> = Graphic3dRenderDriver::new(options).unwrap();
    println!("{{\"flip_output_device\":{:?}}}", driver.device_name());
    let scene = ViewerScene::new(&driver);
    for row in NATIVE_FLIP_TABLE {
        assert_native_flip_row(&scene, row);
    }
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
}
