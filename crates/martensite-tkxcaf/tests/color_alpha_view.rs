//! Surface and curve colors with alpha are stored on a box, and its shaded view shows the lit face.
//! Reading these colors from a reopened stored document is not covered here.
#![cfg(target_os = "linux")]
#[path = "support/document_presentations.rs"]
mod document_presentations;
#[path = "support/document_view_capture.rs"]
#[allow(dead_code)] // This case always names its capture after the case.
mod document_view_capture;
mod support;

use document_presentations::{register_document_presentation_driver, show_explicit_label};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::MapOfShape;
use martensite_tkernel::quantity::{Color, ColorRgba, NamedColor};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdf::{TdfLabel, TdfTool};
use martensite_tkservice::graphic3d::Graphic3dBufferType;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tkv3d::v3d::V3dViewSource;
use martensite_tkxcaf::xcaf_doc::{XCAFDocColorTool, XCAFDocColorType, XCAFDocDocumentTool};
use support::document_viewer::{ViewerLifecycle, register_vulkan_factory};
use support::script_stream::ScriptStream;
use support::shape_document::XdeShapeRegressionContext;

const DOCUMENT_VIEW_NAME: &str = "Driver1/Document_D_First/View1";
const SHADED: i32 = 1;

fn distinct(shape: &Shape, kind: ShapeEnum) -> Vec<Shape> {
    let mut seen = MapOfShape::default();
    let mut parts = Vec::new();
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let current = explorer.current().clone();
        if seen.add(current.clone()) {
            parts.push(current);
        }
        explorer.next().unwrap();
    }
    parts
}

fn with_alpha(name: NamedColor, alpha: f32) -> ColorRgba {
    let mut color = ColorRgba::from_color(Color::named(name));
    color.set_alpha(alpha);
    color
}

fn set_color(tool: &XCAFDocColorTool, shape: &Shape, color: &ColorRgba, kind: XCAFDocColorType) {
    let found = tool.set_shape_color(shape, color, kind).unwrap();
    assert!(found, "the shape is not in the document");
}

/// Face colors go on the surface, edge colors on the curve, and one color has no user at all.
fn assign_colors(tool: &XCAFDocColorTool, unit: &Shape) -> TdfLabel {
    let faces = distinct(unit, ShapeEnum::Face);
    let edges = distinct(unit, ShapeEnum::Edge);
    let surface = XCAFDocColorType::ColorSurf;
    let curve = XCAFDocColorType::ColorCurv;
    let red = ColorRgba::from_color(Color::named(NamedColor::RED));
    set_color(tool, &faces[0], &red, surface);
    set_color(
        tool,
        &faces[1],
        &with_alpha(NamedColor::YELLOW, 0.3),
        surface,
    );
    set_color(tool, &edges[10], &with_alpha(NamedColor::WHITE, 0.2), curve);
    let cyan = ColorRgba::from_color(Color::named(NamedColor::CYAN));
    set_color(tool, &edges[9], &cyan, curve);
    let unused = ColorRgba::new(0.5, 0.5, 1., 0.1).unwrap();
    let added = tool.add_color(&unused).unwrap();
    assert_eq!(entry_of(&added), "0:1:2:5");
    added
}

fn entry_of(label: &TdfLabel) -> String {
    let mut entry = AsciiString::default();
    TdfTool::entry(label, &mut entry);
    String::from_utf8_lossy(entry.as_bytes()).into_owned()
}

/// The document before it is stored holds the values a native run prints for it.
fn assert_stored_colors(tool: &XCAFDocColorTool) {
    let expected = [
        ("RED", 1.),
        ("YELLOW", 0.3),
        ("WHITE", 0.2),
        ("CYAN", 1.),
        ("LIGHTSTEELBLUE2", 0.1),
    ];
    let labels = tool.get_colors().unwrap();
    assert_eq!(labels.len(), expected.len());
    for (label, (name, alpha)) in labels.iter().zip(expected) {
        let color = XCAFDocColorTool::get_color_rgba(label).unwrap().unwrap();
        assert_eq!(Color::string_name(color.rgb().name()), name);
        assert!(
            (f64::from(color.alpha()) - alpha).abs() <= 1e-7,
            "{name} alpha"
        );
    }
}

fn pixel_color_name(view: &dyn V3dViewSource, x: i32, y: i32) -> &'static str {
    let (width, height) = view.window().get().unwrap().size();
    let image = ImagePixMap::default();
    assert!(image.init_trash(ImageFormat::Rgb, width as usize, height as usize, 0));
    let dumped = view
        .to_pix_map_sized(&image, width, height, Graphic3dBufferType::Rgb, true)
        .unwrap();
    assert!(dumped, "no image of the view");
    let color = image.pixel_color(x, y, true).unwrap();
    Color::string_name(color.rgb().name())
}

fn preserve_requested_artifact(bytes: &[u8]) {
    let Some(directory) = std::env::var_os("MARTENSITE_DOCUMENT_ARTIFACT_DIR") else {
        return;
    };
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(
        std::path::Path::new(&directory).join("bug28641_first.ppm"),
        bytes,
    )
    .unwrap();
}

fn show_and_check(lifecycle: &ViewerLifecycle, label: &TdfLabel, stream: &mut ScriptStream) {
    stream.lines([
        "XShow D_First",
        "pload OPENGL",
        "vfit",
        "vsetdispmode 1",
        "vdump image",
    ]);
    show_explicit_label(lifecycle, DOCUMENT_VIEW_NAME, label);
    let view = lifecycle.current_view();
    let view_ref = view.get().unwrap();
    view_ref.fit_all(0.01, false).unwrap();
    view_ref.redraw();
    let context = lifecycle.interactive_context();
    assert_eq!(context.nb_selected(), 0);
    context.set_default_display_mode(SHADED, true).unwrap();
    context.update_current_viewer();
    let bytes = document_view_capture::read_case_saved_view(&view, "bug28641_first");
    assert_eq!(&bytes[..15], b"P6\n409 409\n255\n");
    preserve_requested_artifact(&bytes);
    stream.line("vreadpixel 300 200 rgb name");
    let pixel = pixel_color_name(view_ref, 300, 200);
    stream.line(pixel);
    assert_eq!(pixel, "GRAY63");
}

fn numbered_names(prefix: &str, count: usize) -> String {
    let names: Vec<String> = (1..=count)
        .map(|index| format!("{prefix}_{index}"))
        .collect();
    names.join(" ")
}

fn alpha_color_scenario() {
    register_vulkan_factory();
    register_document_presentation_driver();
    let mut stream = ScriptStream::default();
    stream.lines([
        "cpulimit 1000",
        "CPU and elapsed time limit set to 1000 seconds",
        "pload OCAF XDE",
        "========",
        "0028641: Data Exchange - Support alpha-channel of color",
        "========",
        "pload OCAF",
        "NewDocument D_First BinXCAF",
        "document D_First created",
    ]);
    let context = XdeShapeRegressionContext::new();
    stream.line(entry_of(&context.document.main().unwrap()));
    let unit = support::make_box(1., 1., 1.);
    stream.lines(["box b 1 1 1", "explode b e"]);
    stream.line(numbered_names("b", distinct(&unit, ShapeEnum::Edge).len()));
    stream.line("explode b f");
    stream.line(numbered_names("b", distinct(&unit, ShapeEnum::Face).len()));
    stream.line("XAddShape D_First b");
    let label = context.add_shape(&unit, true);
    stream.line(entry_of(&label));
    let main = context.document.main().unwrap();
    let colors = XCAFDocDocumentTool::color_tool(&main).unwrap();
    let color_tool = colors.get().unwrap();
    stream.lines([
        "XSetColor D_First b_1 RED s",
        "XSetColor D_First b_2 YELLOW 0.3 s",
        "XSetColor D_First b_11 WHITE 0.2 c",
        "XSetColor D_First b_10 CYAN c",
        "XAddColor D_First 0.5 0.5 1 0.1",
    ]);
    let added = assign_colors(color_tool, &unit);
    stream.line(entry_of(&added));
    assert_stored_colors(color_tool);
    let lifecycle = ViewerLifecycle::new(true);
    show_and_check(&lifecycle, &label, &mut stream);
    stream.save("bug28641");
    context.close();
    lifecycle.remove(DOCUMENT_VIEW_NAME, true).unwrap();
    assert!(lifecycle.state().views.borrow().is_empty());
}

#[test]
fn surface_and_curve_colors_with_alpha_keep_the_lit_face_gray() {
    support::case_policy::run_with_case_policy(
        "surface_and_curve_colors_with_alpha_keep_the_lit_face_gray",
        alpha_color_scenario,
    );
}
