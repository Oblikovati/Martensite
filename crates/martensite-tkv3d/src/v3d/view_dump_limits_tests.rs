//! A dump from a view whose viewer was released fails typed instead of tiling forever.
use super::view_dump_limits::checked_dump_tile;
use super::{V3D_DUMP_WITHOUT_LIMITS, V3dImageDumpOptions, V3dViewer};
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectNeutralWindow;
use martensite_tkservice::image::ImagePixMap;
use martensite_tkservice::null_driver::NullGraphicDriver;

#[test]
fn dump_after_the_viewer_is_released_fails_typed() {
    let viewer = V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_size(16, 8);
    let view = viewer.create_view().unwrap();
    view.set_window(&window.cast(), 0).unwrap();
    drop(viewer);
    let options = V3dImageDumpOptions {
        width: 16,
        height: 8,
        tile_size: 4,
        ..Default::default()
    };
    let failure = view
        .view_base()
        .to_pix_map(&ImagePixMap::default(), &options)
        .unwrap_err();
    assert_eq!(failure.exception_type(), V3D_DUMP_WITHOUT_LIMITS);
    assert!(failure.message().contains("viewer=released"), "{failure:?}");
}

#[test]
fn dump_with_a_live_viewer_still_succeeds() {
    let viewer = V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_size(16, 8);
    let view = viewer.create_view().unwrap();
    view.set_window(&window.cast(), 0).unwrap();
    let options = V3dImageDumpOptions {
        width: 16,
        height: 8,
        ..Default::default()
    };
    let image = ImagePixMap::default();
    assert!(view.view_base().to_pix_map(&image, &options).is_ok());
    assert_eq!((image.width(), image.height()), (16, 8));
}

#[test]
fn zero_tiles_fail_typed_and_positive_tiles_pass() {
    assert!(checked_dump_tile(Vec2::new(1, 1)).is_ok());
    for tile in [Vec2::new(0, 0), Vec2::new(64, 0), Vec2::new(-1, 64)] {
        let failure = checked_dump_tile(tile).unwrap_err();
        assert_eq!(failure.exception_type(), V3D_DUMP_WITHOUT_LIMITS);
        assert!(
            failure
                .message()
                .contains(&format!("tile={}x{}", tile.x, tile.y))
        );
    }
}
