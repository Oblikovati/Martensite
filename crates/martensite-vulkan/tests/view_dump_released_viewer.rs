//! A tiled `ToPixMap` from a Vulkan view whose viewer was released.
//!
//! The view holds its viewer weakly; once the viewer is gone the driver limits read zero, the
//! Vulkan view still creates a (0x0) offscreen buffer, and the dump used to tile by zero
//! pixels forever. It now fails with the typed `V3dDumpWithoutLimits` failure, and the device
//! reports no validation message.
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::{AspectNeutralWindow, AspectWindowMembers as _};
use martensite_tkservice::image::ImagePixMap;
use martensite_tkv3d::v3d::{V3D_DUMP_WITHOUT_LIMITS, V3dImageDumpOptions, V3dViewer};
use martensite_vulkan::{Graphic3dRenderDriver, VulkanOptions};

#[test]
fn tiled_dump_after_the_viewer_is_released_fails_typed() {
    let options = VulkanOptions {
        validation: true,
        device_index: None,
    };
    let driver = Graphic3dRenderDriver::new(options).unwrap();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_size(40, 30);
    window.set_virtual(true);
    let view = viewer.create_view().unwrap();
    view.set_window(&window.cast(), 0).unwrap();
    drop(viewer);
    let dump = V3dImageDumpOptions {
        width: 40,
        height: 30,
        tile_size: 16,
        ..Default::default()
    };
    let result = view.view_base().to_pix_map(&ImagePixMap::default(), &dump);
    let failure = result.unwrap_err();
    assert_eq!(failure.exception_type(), V3D_DUMP_WITHOUT_LIMITS);
    view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
}
