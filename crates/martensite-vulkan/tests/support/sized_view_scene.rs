//! A V3d view of a black virtual window whose dumps replay the 1040x1040 recorded captures
//! (the recorded graduated_trihedron capture, martensite_axial_scale): each dump is
//! fingerprinted and compared with the fixture line of the device's native driver (llvmpipe for
//! lavapipe, radeonsi for RADV). `XW_DUMP=<dir>` also writes each dump as `<label>.ppm`.
use super::device_options;
use super::rgb_fingerprint::{Fingerprint, fingerprint, native_fingerprints};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::{
    aspect::{AspectNeutralWindow, AspectWindowMembers},
    graphic3d::*,
    image::ImagePixMap,
};
use martensite_tkv3d::v3d::{V3dView, V3dViewSource, V3dViewer};
use martensite_vulkan::{Graphic3dRenderDriver, Graphic3dRenderView};

/// Wider than 1024 px, so radeonsi snaps vertices to 1/256 pixel like RADV (see the probes).
pub const SIZE: i32 = 1040;

pub struct SizedViewScene {
    pub driver: Handle<Graphic3dRenderDriver>,
    pub viewer: Handle<V3dViewer>,
    view: Handle<dyn V3dViewSource>,
    _window: Handle<AspectNeutralWindow>,
    native: &'static str,
}

impl SizedViewScene {
    /// A view replaying the fingerprints of `native`, with or without auto z-fit.
    pub fn new(native: &'static str, auto_z_fit: bool) -> Self {
        let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
        Self::with_driver(native, auto_z_fit, driver)
    }

    /// The same view on a caller-built driver, e.g. a control frame convention.
    pub fn with_driver(
        native: &'static str,
        auto_z_fit: bool,
        driver: Handle<Graphic3dRenderDriver>,
    ) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        let view = viewer.create_view().unwrap();
        let v3d = view.get().unwrap();
        v3d.set_immediate_update(false);
        let window = Handle::new(AspectNeutralWindow::default());
        window.set_virtual(true);
        window.set_size(SIZE, SIZE);
        v3d.set_window(&window.cast(), 0).unwrap();
        v3d.set_background_color(Color::named(NamedColor::BLACK));
        v3d.set_auto_z_fit_mode(auto_z_fit, 1.).unwrap();
        Self {
            driver,
            viewer,
            view,
            _window: window,
            native,
        }
    }

    pub fn v3d(&self) -> &V3dView {
        self.view.get().unwrap()
    }

    pub fn render_view(&self) -> Handle<Graphic3dRenderView> {
        self.v3d().view().cast()
    }

    /// A new structure of this viewer; display it to draw it.
    pub fn structure(&self) -> Handle<Graphic3dStructure> {
        Graphic3dStructure::new(&self.viewer.structure_manager(), &Handle::default()).unwrap()
    }

    /// Redraw, dump and fingerprint the top-down RGB image; no validation message may appear.
    pub fn dump(&self, label: &str) -> Fingerprint {
        fingerprint(&self.dump_rgb(label))
    }

    /// Redraw and dump the top-down RGB image; no validation message may appear.
    pub fn dump_rgb(&self, label: &str) -> Vec<u8> {
        let view = self.v3d();
        view.redraw();
        let image = ImagePixMap::default();
        image.set_top_down(true);
        let dumped = view.to_pix_map_sized(&image, SIZE, SIZE, Graphic3dBufferType::Rgba, true);
        assert!(dumped.unwrap());
        let rgba = image.data().data.data().to_vec();
        let rgb: Vec<u8> = rgba.chunks(4).flat_map(|p| p[..3].to_vec()).collect();
        if let Ok(dir) = std::env::var("XW_DUMP") {
            let mut ppm = format!("P6\n{SIZE} {SIZE}\n255\n").into_bytes();
            ppm.extend(&rgb);
            std::fs::write(format!("{dir}/{label}.ppm"), ppm).unwrap();
        }
        assert!(self.driver.validation_errors().unwrap().is_empty());
        rgb
    }

    /// The dump's mismatch against the native line of `label`, if any.
    pub fn mismatch(&self, label: &str) -> Option<String> {
        self.rgb_mismatch(label, &self.dump_rgb(label))
    }

    /// The mismatch of an RGB dump against the native line of `label`, if any.
    pub fn rgb_mismatch(&self, label: &str, rgb: &[u8]) -> Option<String> {
        let actual = fingerprint(rgb);
        (actual != native_fingerprints(self.native, label)[self.native_index()]).then(|| {
            format!(
                "{label}: {} colours, fnv {}",
                actual.counts.len(),
                actual.hash
            )
        })
    }

    /// Whether the device is RADV, graded against radeonsi.
    pub fn is_radv(&self) -> bool {
        self.native_index() == 1
    }

    /// The native fixture line of this device's driver: llvmpipe for lavapipe, radeonsi for RADV.
    pub fn native_index(&self) -> usize {
        let identity = self.driver.driver_identity().unwrap();
        match identity.driver_name.as_str() {
            "llvmpipe" => 0,
            "radv" => 1,
            other => panic!("no native fixture for driver {other:?}"),
        }
    }
}

/// A primitive array of `count` vertices without edges, bounds or attributes.
pub fn array(
    kind: Graphic3dTypeOfPrimitiveArray,
    count: i32,
) -> Handle<Graphic3dArrayOfPrimitives> {
    Handle::new(Graphic3dArrayOfPrimitives::new(kind, count, 0, 0, 0).unwrap())
}
