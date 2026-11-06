//! Replays of the renderer references that print the recorded rgb_fingerprint capture lines:
//! `<label> fnv=<FNV-1a 64 of the top-down 64x64 RGB dump> <rrggbb>=<pixel count>...`.
use super::device_options;
use super::scene::ViewerScene;
use martensite_tkernel::{
    quantity::{Color, ColorSpace},
    standard::Handle,
};
use martensite_tkservice::{graphic3d::*, image::ImagePixMap};
use martensite_vulkan::Graphic3dRenderDriver;
use std::collections::BTreeMap;

/// FNV-1a 64 of the RGB bytes and the per-colour pixel counts.
#[derive(Debug, PartialEq)]
pub struct Fingerprint {
    pub hash: String,
    pub counts: BTreeMap<String, usize>,
}

/// Fingerprint packed RGB bytes.
pub fn fingerprint(rgb: &[u8]) -> Fingerprint {
    let mut hash = 1469598103934665603u64;
    let mut counts = BTreeMap::new();
    for pixel in rgb.chunks(3) {
        *counts
            .entry(format!("{:02x}{:02x}{:02x}", pixel[0], pixel[1], pixel[2]))
            .or_insert(0) += 1;
        for &byte in pixel {
            hash = (hash ^ u64::from(byte)).wrapping_mul(1099511628211);
        }
    }
    Fingerprint {
        hash: format!("{hash:016x}"),
        counts,
    }
}

fn parse_fingerprint(line: &str) -> Fingerprint {
    let mut fields = line
        .split_whitespace()
        .skip_while(|field| !field.starts_with("fnv="));
    let hash = fields.next().unwrap().trim_start_matches("fnv=").to_owned();
    let counts = fields
        .map(|field| {
            let (color, count) = field.split_once('=').unwrap();
            (color.to_owned(), count.parse().unwrap())
        })
        .collect();
    Fingerprint { hash, counts }
}

/// The fingerprints labelled `label` in a fixture: llvmpipe first, then AMD radeonsi.
pub fn native_fingerprints(fixture: &str, label: &str) -> [Fingerprint; 2] {
    let key = format!("{label} fnv=");
    let found: Vec<_> = fixture
        .lines()
        .filter(|line| line.starts_with(&key))
        .map(parse_fingerprint)
        .collect();
    found
        .try_into()
        .unwrap_or_else(|v: Vec<_>| panic!("{} lines for {label}; expected sw and host", v.len()))
}

/// A 64x64 viewer with a black background and 1 world unit per pixel, as the probes set it.
pub struct FingerprintScene {
    pub driver: Handle<Graphic3dRenderDriver>,
    pub scene: ViewerScene,
}

impl FingerprintScene {
    pub fn new() -> Self {
        let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
        let scene = ViewerScene::new(&driver);
        scene.structure.erase();
        let view = scene.view.get().unwrap();
        view.set_background_color(Color::new(0., 0., 0., ColorSpace::Rgb).unwrap());
        view.camera().set_scale(64.);
        Self { driver, scene }
    }

    /// A new structure of this viewer; display it to draw it.
    pub fn structure(&self) -> Handle<Graphic3dStructure> {
        Graphic3dStructure::new(&self.scene.viewer.structure_manager(), &Handle::default()).unwrap()
    }

    /// Change the rendering parameters, redraw and fingerprint the top-down RGB dump.
    pub fn dump(&self, params: &mut dyn FnMut(&mut Graphic3dRenderingParams)) -> Fingerprint {
        let view = self.scene.view.get().unwrap();
        view.change_rendering_params(params);
        self.scene.render_view().try_redraw().unwrap();
        let image = ImagePixMap::default();
        image.set_top_down(true);
        let dumped = view.to_pix_map_sized(&image, 64, 64, Graphic3dBufferType::Rgba, true);
        assert!(dumped.unwrap());
        let rgba = image.data().data.data().to_vec();
        let rgb: Vec<u8> = rgba.chunks(4).flat_map(|p| p[..3].to_vec()).collect();
        assert!(self.driver.validation_errors().unwrap().is_empty());
        fingerprint(&rgb)
    }
}
