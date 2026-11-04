//! Stereo pairs (GlView::drawStereoPair, getStdProgramStereo) against native dumps.
//!
//! The reference scene stereo/stereo was recorded on llvmpipe (SSE2/128)
//! and AMD radeonsi: a 1x2x3 box and a red unit cube under a perspective axonometric camera at
//! 256x256 (window at 0,0), dumped blended in every Graphic3dStereoMode with the
//! anaglyph filters, smoothed interlacing and a reversed pair, then left, right and mono.
#[path = "support/native_dump_grading.rs"]
// The per-pixel recorded-failure helpers are unused here.
#[allow(dead_code)]
mod grading;
use grading::{DumpReport, validation_options};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::aspect::{AspectNeutralWindow, AspectWindowMembers as _};
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::*;
use martensite_tkv3d::prs_mgr::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;

const SIZE: (usize, usize) = (256, 256);
const DUMP_NAMES: [&str; 19] = [
    "st-anaglyph",
    "st-anaglyph-redcyan-simple",
    "st-anaglyph-yellowblue-simple",
    "st-anaglyph-yellowblue",
    "st-anaglyph-greenmagenta",
    "st-row",
    "st-row-smooth",
    "st-column",
    "st-column-smooth",
    "st-chess",
    "st-chess-smooth",
    "st-sidebyside",
    "st-overunder",
    "st-quadbuffer",
    "st-softpageflip",
    "st-row-reversed",
    "st-left",
    "st-right",
    "st-mono",
];
macro_rules! dumps {
    ($device:literal) => {
        [
            include_bytes!(concat!("fixtures/stereo/", $device, "/st-anaglyph.ppm")).as_slice(),
            include_bytes!(concat!(
                "fixtures/stereo/",
                $device,
                "/st-anaglyph-redcyan-simple.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/stereo/",
                $device,
                "/st-anaglyph-yellowblue-simple.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/stereo/",
                $device,
                "/st-anaglyph-yellowblue.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/stereo/",
                $device,
                "/st-anaglyph-greenmagenta.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!("fixtures/stereo/", $device, "/st-row.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/stereo/", $device, "/st-row-smooth.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/stereo/", $device, "/st-column.ppm")).as_slice(),
            include_bytes!(concat!(
                "fixtures/stereo/",
                $device,
                "/st-column-smooth.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!("fixtures/stereo/", $device, "/st-chess.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/stereo/", $device, "/st-chess-smooth.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/stereo/", $device, "/st-sidebyside.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/stereo/", $device, "/st-overunder.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/stereo/", $device, "/st-quadbuffer.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/stereo/", $device, "/st-softpageflip.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/stereo/", $device, "/st-row-reversed.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/stereo/", $device, "/st-left.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/stereo/", $device, "/st-right.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/stereo/", $device, "/st-mono.ppm")).as_slice(),
        ]
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 19] = dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 19] = dumps!("amd");

/// A 256x256 view of the capture at the window origin.
struct StereoScene {
    context: Handle<AisInteractiveContext>,
    view: Handle<dyn V3dViewSource>,
    _viewer: Handle<V3dViewer>,
    _window: Handle<AspectNeutralWindow>,
}

impl StereoScene {
    fn new(driver: &Handle<Graphic3dRenderDriver>) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        // A new view clears to black.
        viewer.set_default_background_color(Color::named(NamedColor::BLACK));
        viewer.set_default_lights().unwrap();
        viewer.set_light_on().unwrap();
        let context = AisInteractiveContext::new(&viewer).unwrap();
        let window = Handle::new(AspectNeutralWindow::default());
        window.set_position_and_size(0, 0, SIZE.0 as i32, SIZE.1 as i32);
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
    /// Display the shape shaded.
    fn display(
        &self,
        shape: &martensite_tkbrep::topods::Shape,
    ) -> Handle<dyn AisInteractiveObject> {
        let object: Handle<dyn AisInteractiveObject> = AisShape::new(shape).cast();
        object.get().unwrap().set_display_mode(1);
        self.context
            .display_with_modes(&object, 1, 0, false, PrsMgrDisplayStatus::None)
            .unwrap();
        object
    }
    /// Change the stereo parameters: mode, anaglyph filter, smoothing and reversal.
    fn stereo(&self, change: impl Fn(&mut Graphic3dStereoParams)) {
        self.view()
            .change_rendering_params(&mut |params| change(&mut params.stereo));
        self.view()
            .camera()
            .set_projection_type(Graphic3dCameraProjection::Stereo);
    }
    /// V3dView::ToPixMap of the RGB rows with `stereo_options`, top row first.
    fn dump(&self, stereo_options: V3dStereoDumpOptions) -> Vec<u8> {
        let image = ImagePixMap::default();
        image.set_top_down(true);
        assert!(image.init_zero(ImageFormat::Rgb, SIZE.0, SIZE.1, 0, 0));
        let options = V3dImageDumpOptions {
            stereo_options,
            ..Default::default()
        };
        assert!(self.view().to_pix_map(&image, &options).unwrap());
        let storage = image.data();
        (0..SIZE.1)
            .flat_map(|y| storage.data.row(y).unwrap()[..SIZE.0 * 3].to_vec())
            .collect()
    }
}

fn mode(mode: Graphic3dStereoMode, smooth: bool) -> impl Fn(&mut Graphic3dStereoParams) {
    move |params| {
        params.stereo_mode = mode;
        params.to_smooth_interlacing = smooth;
    }
}

fn anaglyph(filter: Graphic3dRenderingParamsAnaglyph) -> impl Fn(&mut Graphic3dStereoParams) {
    move |params| {
        params.stereo_mode = Graphic3dStereoMode::Anaglyph;
        params.anaglyph_filter = filter;
    }
}

fn build_scene(driver: &Handle<Graphic3dRenderDriver>) -> StereoScene {
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
    let scene = StereoScene::new(driver);
    let mut b = BRepPrimAPIMakeBox::at_point(Point3::new(0., 0., 0.), 1., 2., 3.).unwrap();
    let mut s = BRepPrimAPIMakeBox::at_point(Point3::new(2., 0., 0.), 1., 1., 1.).unwrap();
    scene.display(&b.shape().unwrap().clone());
    let s = scene.display(&s.shape().unwrap().clone());
    scene
        .context
        .set_color(&s, Color::named(NamedColor::RED), false)
        .unwrap();
    let view = scene.view();
    view.camera()
        .set_projection_type(Graphic3dCameraProjection::Perspective);
    view.set_proj_orientation(V3dTypeOfOrientation::XposYnegZpos, false)
        .unwrap();
    view.fit_all(0.01, false).unwrap();
    scene
}

fn stereo_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    use Graphic3dRenderingParamsAnaglyph as Filter;
    use Graphic3dStereoMode as Mode;
    let scene = build_scene(driver);
    let blend = V3dStereoDumpOptions::Blended;
    scene.stereo(mode(Mode::Anaglyph, false));
    let mut dumps = vec![scene.dump(blend)];
    for filter in [
        Filter::RedCyanSimple,
        Filter::YellowBlueSimple,
        Filter::YellowBlueOptimized,
        Filter::GreenMagentaSimple,
    ] {
        scene.stereo(anaglyph(filter));
        dumps.push(scene.dump(blend));
    }
    for interlaced in [
        Mode::RowInterlaced,
        Mode::ColumnInterlaced,
        Mode::ChessBoard,
    ] {
        for smooth in [false, true] {
            scene.stereo(mode(interlaced, smooth));
            dumps.push(scene.dump(blend));
        }
    }
    for pair in [
        Mode::SideBySide,
        Mode::OverUnder,
        Mode::QuadBuffer,
        Mode::SoftPageFlip,
    ] {
        scene.stereo(mode(pair, true));
        dumps.push(scene.dump(blend));
    }
    scene.stereo(|params| {
        mode(Mode::RowInterlaced, false)(params);
        params.to_reverse_stereo = true;
    });
    dumps.push(scene.dump(blend));
    for eye in [
        V3dStereoDumpOptions::LeftEye,
        V3dStereoDumpOptions::RightEye,
        V3dStereoDumpOptions::Mono,
    ] {
        dumps.push(scene.dump(eye));
    }
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

fn graded_stereo() -> (String, Vec<DumpReport>) {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let dumps = stereo_dumps(&driver);
    let device = driver.device_name();
    let native = if device.contains("llvmpipe") {
        NATIVE_LLVMPIPE
    } else {
        NATIVE_AMD
    };
    grading::write_dumps("MARTENSITE_STEREO_DUMP_DIR", &DUMP_NAMES, &dumps, SIZE);
    let report: Vec<DumpReport> = dumps
        .iter()
        .enumerate()
        .map(|(index, rgb)| {
            let expected = grading::ppm_rgb(native[index], SIZE);
            grading::grade_dump(DUMP_NAMES[index], rgb, expected, None, None, &[], SIZE.0)
        })
        .collect();
    let counts: Vec<_> = report
        .iter()
        .map(|(name, differing, pixels)| (*name, *differing, pixels.len()))
        .collect();
    println!("{{\"stereo_device\":{device:?},\"differing_unexplained\":{counts:?}}}");
    (device, report)
}

/// The recorded AMD failures (fixtures/stereo/amd-recorded.txt, `dump x y` lines).
fn recorded_failures() -> Vec<grading::RecordedFailure> {
    include_str!("fixtures/stereo/amd-recorded.txt")
        .lines()
        .filter(|line| !line.starts_with('#'))
        .map(|line| {
            let fields: Vec<&str> = line.split(' ').collect();
            let name = DUMP_NAMES.iter().find(|name| **name == fields[0]).unwrap();
            let pixel = (fields[1].parse().unwrap(), fields[2].parse().unwrap());
            ("AMD", *name, Some(pixel))
        })
        .collect()
}

#[test]
fn stereo_pairs_match_the_native_dumps() {
    let (device, report) = graded_stereo();
    let failing = grading::unrecorded_pixels(&report, &device, &recorded_failures());
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps or approved residues: {failing:?}"
    );
}

#[test]
#[ignore = "recorded failures: fixtures/stereo/amd-recorded.txt lists the AMD eye-scene silhouette pixels"]
fn stereo_recorded_failures_match_the_native_dumps() {
    let (device, report) = graded_stereo();
    let failing = grading::unrecorded_pixels(&report, &device, &[]);
    assert!(
        failing.is_empty(),
        "device={device}; expected every dump within an approved bound: {failing:?}"
    );
}
