//! The 2D image background scene of the recorded background_image capture, shared by the
//! programmable and fixed-function replays.
use martensite_tkernel::quantity::{Color, ColorSpace, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::{
    AspectFillMethod, AspectGradientFillMethod, AspectNeutralWindow, AspectWindowMembers as _,
};
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;

pub const SIZE: (usize, usize) = (203, 151);
/// The tile edge of the tiled dumps.
const TILE: i32 = 64;
/// Dumps in capture order.
pub const DUMP_NAMES: [&str; 12] = [
    "image-centered",
    "image-tiled",
    "image-stretch",
    "image-none",
    "image-stretch-tiles",
    "image-centered-tiles",
    "gradient-centered",
    "gradient-stretch",
    "gradient-none",
    "corner-centered",
    "image-cleared",
    "gradient-after-clear",
];
/// The capture's 37x23 RGB texture: red and green ramps over a 4-texel blue checker, in the
/// logical rows of a bottom-up pixmap (ImagePixMap's default row order).
fn background_texture() -> Handle<dyn Graphic3dTextureMap> {
    let mut logical = Vec::with_capacity(37 * 23 * 3);
    for row in 0..23_u32 {
        for column in 0..37_u32 {
            let blue = ((column >> 2) ^ (row >> 2)) & 1 != 0;
            logical.extend([
                (column * 7) as u8,
                (row * 11) as u8,
                if blue { 230 } else { 20 },
            ]);
        }
    }
    let memory: Vec<u8> = logical.chunks(37 * 3).rev().flatten().copied().collect();
    let image = Handle::new(ImagePixMap::default());
    assert!(image.init_wrapper(ImageFormat::Rgb, Some(memory), 37, 23, 37 * 3));
    image.set_top_down(false);
    Handle::new(Graphic3dTexture2D::from_pix_map(image)).cast()
}

/// The capture's view and the viewer and window it keeps alive (a view without its viewer
/// reads no driver limits, so `ToPixMap` would tile by zero pixels).
struct BackgroundScene {
    view: Handle<dyn V3dViewSource>,
    _viewer: Handle<V3dViewer>,
    _window: Handle<AspectNeutralWindow>,
    styles: Vec<String>,
}
impl BackgroundScene {
    fn new(driver: &Handle<Graphic3dRenderDriver>) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        viewer.set_default_background_color(Color::named(NamedColor::BLACK));
        let window = Handle::new(AspectNeutralWindow::default());
        window.set_size(SIZE.0 as i32, SIZE.1 as i32);
        window.set_virtual(true);
        let view = viewer.create_view().unwrap();
        view.set_window(&window.cast(), 0).unwrap();
        let mut scene = Self {
            view,
            _viewer: viewer,
            _window: window,
            styles: Vec::new(),
        };
        scene.record_style("initial");
        scene
    }
    fn view(&self) -> &V3dView {
        self.view.get().unwrap().view_base()
    }
    fn record_style(&mut self, name: &str) {
        let style = self.view().view().background_image_style() as i32;
        self.styles.push(format!("{name} {style}"));
    }
    fn style(&self, style: AspectFillMethod) {
        self.view().set_bg_image_style(style, false);
    }
    fn gradient(&self, method: AspectGradientFillMethod) {
        let first = Color::new(0.9, 0.2, 0.1, ColorSpace::Rgb).unwrap();
        let second = Color::new(0.1, 0.3, 0.8, ColorSpace::Rgb).unwrap();
        self.view()
            .set_bg_gradient_colors(first, second, method, false);
    }
    /// `ToPixMap` at the window size, in `tile`-pixel tiles when positive.
    fn dump(&mut self, name: &str, tile: i32) -> Vec<u8> {
        self.view.redraw();
        let image = ImagePixMap::default();
        image.set_top_down(true);
        let options = V3dImageDumpOptions {
            width: SIZE.0 as i32,
            height: SIZE.1 as i32,
            tile_size: tile,
            ..Default::default()
        };
        assert!(self.view().to_pix_map(&image, &options).unwrap(), "{name}");
        self.record_style(name);
        let storage = image.data();
        (0..SIZE.1)
            .flat_map(|y| storage.data.row(y).unwrap()[..SIZE.0 * 3].to_vec())
            .collect()
    }
}

fn fill_dumps(scene: &mut BackgroundScene, dumps: &mut Vec<Vec<u8>>) {
    let texture = background_texture();
    let view = scene.view();
    view.set_background_image(&texture, AspectFillMethod::Centered, false);
    dumps.push(scene.dump("image-centered", 0));
    for (style, name) in [
        (AspectFillMethod::Tiled, "image-tiled"),
        (AspectFillMethod::Stretch, "image-stretch"),
        (AspectFillMethod::None, "image-none"),
    ] {
        scene.style(style);
        dumps.push(scene.dump(name, 0));
    }
    scene.style(AspectFillMethod::Stretch);
    dumps.push(scene.dump("image-stretch-tiles", TILE));
    scene.style(AspectFillMethod::Centered);
    dumps.push(scene.dump("image-centered-tiles", TILE));
}
fn gradient_dumps(scene: &mut BackgroundScene, dumps: &mut Vec<Vec<u8>>) {
    scene.gradient(AspectGradientFillMethod::Vertical);
    dumps.push(scene.dump("gradient-centered", 0));
    scene.style(AspectFillMethod::Stretch);
    dumps.push(scene.dump("gradient-stretch", 0));
    scene.style(AspectFillMethod::None);
    dumps.push(scene.dump("gradient-none", 0));
    scene.gradient(AspectGradientFillMethod::Corner3);
    scene.style(AspectFillMethod::Centered);
    dumps.push(scene.dump("corner-centered", 0));
    let view = scene.view();
    view.set_background_image(&Handle::default(), AspectFillMethod::Centered, false);
    dumps.push(scene.dump("image-cleared", 0));
    scene.gradient(AspectGradientFillMethod::Horizontal);
    dumps.push(scene.dump("gradient-after-clear", 0));
}

/// Run the capture on `driver`: every dump in `DUMP_NAMES` order and the style lines.
pub fn background_dumps(driver: &Handle<Graphic3dRenderDriver>) -> (Vec<Vec<u8>>, Vec<String>) {
    let mut scene = BackgroundScene::new(driver);
    let mut dumps = Vec::new();
    fill_dumps(&mut scene, &mut dumps);
    gradient_dumps(&mut scene, &mut dumps);
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    (dumps, scene.styles)
}
