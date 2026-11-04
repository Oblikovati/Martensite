//! Corner gradient backgrounds (getColoredQuadProgram) against native dumps.
//!
//! The reference scene colored_quad/colored_quad was recorded on llvmpipe
//! (SSE2/128) and AMD radeonsi: a 320x240 view with RED/BLUE Corner1..4, ORANGE/STEELBLUE
//! Corner1 and Corner3, then a shaded box over an ORANGE/STEELBLUE Corner2 background.
#[path = "support/native_dump_grading.rs"]
// The per-pixel recorded-failure helpers are unused here.
#[allow(dead_code)]
mod grading;
use grading::{DumpReport, validation_options};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::aspect::{
    AspectGradientFillMethod, AspectNeutralWindow, AspectWindowMembers as _,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::*;
use martensite_tkv3d::prs_mgr::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;

const SIZE: (usize, usize) = (320, 240);
macro_rules! dumps {
    ($device:literal, [$($name:literal),*]) => {
        [$(include_bytes!(concat!("fixtures/colored_quad/", $device, "/", $name, ".ppm")).as_slice()),*]
    };
}
macro_rules! named_dumps {
    ($device:literal) => {
        dumps!(
            $device,
            [
                "cq-red-blue-corner1",
                "cq-red-blue-corner2",
                "cq-red-blue-corner3",
                "cq-red-blue-corner4",
                "cq-orange-steelblue-corner1",
                "cq-orange-steelblue-corner3",
                "cq-box-corner2"
            ]
        )
    };
}
/// Dumps in capture order.
const DUMP_NAMES: [&str; 7] = [
    "cq-red-blue-corner1",
    "cq-red-blue-corner2",
    "cq-red-blue-corner3",
    "cq-red-blue-corner4",
    "cq-orange-steelblue-corner1",
    "cq-orange-steelblue-corner3",
    "cq-box-corner2",
];
const NATIVE_LLVMPIPE: [&[u8]; 7] = named_dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 7] = named_dumps!("amd");

/// A 320x240 view of the capture with its viewer and context.
struct ColoredQuadScene {
    context: Handle<AisInteractiveContext>,
    view: Handle<dyn V3dViewSource>,
    _viewer: Handle<V3dViewer>,
    _window: Handle<AspectNeutralWindow>,
}
impl ColoredQuadScene {
    fn new(driver: &Handle<Graphic3dRenderDriver>) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        viewer.set_default_lights().unwrap();
        viewer.set_light_on().unwrap();
        let context = AisInteractiveContext::new(&viewer).unwrap();
        let window = Handle::new(AspectNeutralWindow::default());
        window.set_size(SIZE.0 as i32, SIZE.1 as i32);
        window.set_virtual(true);
        let view = viewer.create_view().unwrap();
        view.set_window(&window.cast(), 0).unwrap();
        Self {
            context,
            view,
            _viewer: viewer,
            _window: window,
        }
    }
    fn view(&self) -> &V3dView {
        self.view.get().unwrap().view_base()
    }
    /// Dump a gradient background of `colors` filled with `method`.
    fn gradient_dump(&self, colors: [NamedColor; 2], method: AspectGradientFillMethod) -> Vec<u8> {
        let [first, second] = colors.map(Color::named);
        self.view()
            .set_bg_gradient_colors(first, second, method, false);
        self.view.redraw();
        grading::view_rgb(self.view(), SIZE)
    }
    /// Display a shaded unit box and fit the view.
    fn display_box(&self) {
        let point = Point3::new(0., 0., 0.);
        let mut maker =
            martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::at_point(point, 1., 1., 1.)
                .unwrap();
        let shape: Handle<dyn AisInteractiveObject> =
            AisShape::new(&maker.shape().unwrap().clone()).cast();
        shape.get().unwrap().set_display_mode(1);
        self.context
            .display_with_modes(&shape, 1, 0, false, PrsMgrDisplayStatus::None)
            .unwrap();
        self.view().fit_all(0.01, false).unwrap();
    }
}

fn colored_quad_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    use AspectGradientFillMethod::{Corner1, Corner2, Corner3, Corner4};
    let scene = ColoredQuadScene::new(driver);
    let red_blue = [NamedColor::RED, NamedColor::BLUE];
    let orange_steel = [NamedColor::ORANGE, NamedColor::STEELBLUE];
    let mut dumps: Vec<Vec<u8>> = [Corner1, Corner2, Corner3, Corner4]
        .into_iter()
        .map(|method| scene.gradient_dump(red_blue, method))
        .collect();
    for method in [Corner1, Corner3] {
        dumps.push(scene.gradient_dump(orange_steel, method));
    }
    scene.display_box();
    dumps.push(scene.gradient_dump(orange_steel, Corner2));
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

fn graded_colored_quads() -> (String, Vec<DumpReport>) {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let dumps = colored_quad_dumps(&driver);
    let control = grading::rounding_control(&driver, colored_quad_dumps);
    let device = driver.device_name();
    let native = if device.contains("llvmpipe") {
        NATIVE_LLVMPIPE
    } else {
        NATIVE_AMD
    };
    grading::write_dumps(
        "MARTENSITE_COLORED_QUAD_DUMP_DIR",
        &DUMP_NAMES,
        &dumps,
        SIZE,
    );
    if let Some(control) = &control {
        grading::write_dumps(
            "MARTENSITE_COLORED_QUAD_CONTROL_DIR",
            &DUMP_NAMES,
            control,
            SIZE,
        );
    }
    let report: Vec<DumpReport> = dumps
        .iter()
        .enumerate()
        .map(|(index, rgb)| {
            let expected = grading::ppm_rgb(native[index], SIZE);
            let control = control.as_ref().map(|dumps| dumps[index].as_slice());
            grading::grade_dump(DUMP_NAMES[index], rgb, expected, control, None, &[], SIZE.0)
        })
        .collect();
    let counts: Vec<_> = report
        .iter()
        .map(|(name, differing, pixels)| (*name, *differing, pixels.len()))
        .collect();
    println!("{{\"colored_quad_device\":{device:?},\"differing_unexplained\":{counts:?}}}");
    (device, report)
}

#[test]
fn corner_gradients_match_the_native_dumps() {
    let (device, report) = graded_colored_quads();
    let failing: Vec<_> = report
        .iter()
        .filter(|(_, _, pixels)| !pixels.is_empty())
        .map(|(name, _, pixels)| (*name, pixels.len(), pixels.first().cloned()))
        .collect();
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps or approved residues: {failing:?}"
    );
}
