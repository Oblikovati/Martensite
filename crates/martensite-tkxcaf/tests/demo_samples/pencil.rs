//! A pencil: a hexagonal body cut by a cone around a lead, with engraved text and colored faces.
use super::document_display::{
    DocumentDisplay, SavedView, fit_view, named, opened_view, redraw_document, save_view,
};
use super::solid_steps::{
    assert_counts, assert_valid_measures, boolean, children, distinct, rotated, translated,
};
use super::support::compound_shapes::compound_of;
use super::support::{new_document, shape_document::ensure_document_unit};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::String8;
use martensite_tkernel::quantity::{Color, ColorRgba, NamedColor};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Point3, Vector3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeOneAxis as _;
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCone, BRepPrimAPIMakeCylinder, BRepPrimAPIMakePrism,
    BRepPrimAPIMakeSphere,
};
use martensite_tkservice::graphic3d::{
    Graphic3dHorizontalTextAlignment, Graphic3dVerticalTextAlignment,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::std_prs::{StdPrsBRepFont, StdPrsBRepFontTextOptions};
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocColorTool, XCAFDocColorType, XCAFDocDocumentTool, XCAFDocShapeTool,
};

const FREE_SERIF: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../external/martensite-test-data/fonts/freefont-20211204/FreeSerif.ttf"
);
const X_AXIS: [f64; 3] = [1., 0., 0.];
const Y_AXIS: [f64; 3] = [0., 1., 0.];

/// A plate intersected with two turned copies of itself. The first argument of each intersection
/// is what the plate compound holds, so the operation sees the unturned plate three times.
fn hexagonal_prism() -> Shape {
    let plate = BRepPrimAPIMakeBox::at_point(Point3::new(0., -20., -10.), 100., 40., 20.)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape();
    let plates = compound_of(&[plate.clone(), plate.clone(), plate.clone()]);
    let copies = children(&plates);
    let first = rotated(&copies[0], X_AXIS, 60.);
    let second = rotated(&copies[1], X_AXIS, -60.);
    let one = boolean(&plates, &first, Operation::Common);
    boolean(&one, &second, Operation::Common)
}

/// The bare cylinder and the lead, a cylinder with a rounded end.
fn cylinder_and_lead() -> (Shape, Shape) {
    let cylinder = BRepPrimAPIMakeCylinder::new(4., 100.)
        .solid()
        .unwrap()
        .into_shape();
    let cylinder = rotated(&cylinder, Y_AXIS, 90.);
    let sphere = BRepPrimAPIMakeSphere::new(1.4)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape();
    let sphere = translated(&sphere, 99.2, 0., 0.);
    let lead = boolean(&cylinder, &sphere, Operation::Fuse);
    (cylinder, lead)
}

fn sharpening_cone() -> Shape {
    let cone = BRepPrimAPIMakeCone::new(60., 0.5, 101.)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape();
    rotated(&cone, Y_AXIS, 90.)
}

/// The text's outline in FreeSerif, by path, so the result does not depend on the fonts installed
/// on the machine. The expected counts and measures below hold for this font.
fn engraving() -> Shape {
    let font = StdPrsBRepFont::new();
    assert!(
        font.init(
            &String8::from_unicode_units(FREE_SERIF.as_bytes(), -1),
            10.,
            0
        ),
        "no font at {FREE_SERIF}"
    );
    let options = StdPrsBRepFontTextOptions {
        pen: martensite_tkmath::gp::Axis3::new(
            Point3::new(0., 0., 0.),
            martensite_tkmath::gp::Direction3::new(0., 0., 1.).unwrap(),
            martensite_tkmath::gp::Direction3::new(1., 0., 0.).unwrap(),
        )
        .unwrap(),
        horizontal_alignment: Graphic3dHorizontalTextAlignment::Left,
        vertical_alignment: Graphic3dVerticalTextAlignment::Bottom,
        to_concatenate_contours: false,
    };
    let text = "CAD Assistant";
    let shape = font.render_text_with(&String8::from_unicode_units(text.as_bytes(), -1), &options);
    assert!(!shape.is_null(), "no outline for {text:?}");
    shape
}

fn color_faces(tool: &XCAFDocColorTool, faces: &[Shape], tags: &[usize], color: &ColorRgba) {
    for tag in tags {
        assert!(
            tool.set_shape_color(&faces[tag - 1], color, XCAFDocColorType::ColorGen)
                .unwrap(),
            "face {tag} is not in the document"
        );
    }
}

fn hex_color(text: &str) -> ColorRgba {
    ColorRgba::from_hex(text, false).unwrap_or_else(|| panic!("{text} is not a hex color"))
}

struct PencilSolids {
    body: Shape,
    core: Shape,
}

fn pencil_solids() -> PencilSolids {
    let prism = hexagonal_prism();
    assert_counts("prism", &prism, [12, 18, 8, 8, 1, 1, 0, 1, 49]);
    assert_valid_measures("prism", &prism, 34641., 7621.02);
    let (cylinder, lead) = cylinder_and_lead();
    assert_counts("lead", &lead, [4, 6, 6, 5, 1, 1, 0, 1, 24]);
    assert_valid_measures("lead", &lead, 5027.91, 2614.94);
    let cone = sharpening_cone();
    let body = boolean(&prism, &cone, Operation::Common);
    let body = boolean(&body, &cylinder, Operation::Cut);
    let core = boolean(&lead, &cone, Operation::Common);
    let text = translated(&engraving(), 10., -4., 10.);
    assert_counts("text", &text, [226, 226, 12, 9, 0, 0, 0, 2, 475]);
    let raised = BRepPrimAPIMakePrism::new(&text, Vector3::new(0., 0., -1.), false, true)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    assert_counts(
        "raised text",
        &raised,
        [226, 561, 347, 344, 13, 13, 0, 2, 1506],
    );
    let body = boolean(&body, &raised, Operation::Fuse);
    assert_counts("body", &body, [241, 249, 39, 22, 1, 1, 0, 1, 554]);
    assert_valid_measures("body", &body, 26261.3, 9086.51);
    assert_counts("core", &core, [4, 7, 5, 5, 1, 1, 0, 1, 24]);
    assert_valid_measures("core", &core, 4890.95, 2536.03);
    PencilSolids { body, core }
}

/// Build, inspect and show the document.
pub fn run() {
    let solids = pencil_solids();
    let body_solid = distinct(&solids.body, ShapeEnum::Solid).remove(0);
    let core_solid = distinct(&solids.core, ShapeEnum::Solid).remove(0);
    let body_faces = distinct(&body_solid, ShapeEnum::Face);
    let core_faces = distinct(&core_solid, ShapeEnum::Face);
    assert_eq!((body_faces.len(), core_faces.len()), (26, 5));
    let (_application, document) = new_document("BinXCAF");
    ensure_document_unit(&document);
    let main = document.main().unwrap();
    let shapes = XCAFDocDocumentTool::shape_tool(&main).unwrap();
    let shape_tool = shapes.get().unwrap();
    shape_tool.add_shape(&body_solid, true, true).unwrap();
    shape_tool.add_shape(&core_solid, true, true).unwrap();
    let colors = XCAFDocDocumentTool::color_tool(&main).unwrap();
    let color_tool = colors.get().unwrap();
    color_document(color_tool, &body_faces, &core_faces, &core_solid);
    assert_color_names(color_tool);
    let root = shape_tool.get_one_shape().unwrap();
    assert_counts("document", &root, [244, 255, 44, 27, 2, 2, 0, 1, 575]);
    assert_valid_measures("document", &root, 31152.3, 11622.5);
    let image = show_on_white(shape_tool);
    assert_image(&image);
}

fn show_on_white(shape_tool: &XCAFDocShapeTool) -> SavedView {
    let lifecycle = opened_view();
    let shown = DocumentDisplay::new(true).show_free_shapes(&lifecycle, "D", shape_tool);
    assert_eq!(shown, ["D:SOLID", "D:SOLID_1"]);
    redraw_document(&lifecycle);
    fit_view(&lifecycle);
    let view = lifecycle.current_view();
    let view_ref = view.get().unwrap();
    view_ref.change_rendering_params(&mut |params| params.nb_msaa_samples = 8);
    view_ref.set_background_color(Color::named(NamedColor::WHITE));
    view_ref.redraw();
    save_view(&lifecycle, "pencil")
}

fn assert_image(image: &SavedView) {
    const WHITE: [u8; 3] = [255, 255, 255];
    const BODY: [u8; 3] = [9, 9, 168];
    image.assert_probes([
        [12, 12, 178],
        BODY,
        WHITE,
        WHITE,
        [151, 83, 66],
        WHITE,
        WHITE,
        [240, 240, 245],
        BODY,
        WHITE,
        WHITE,
    ]);
}

fn color_document(
    tool: &XCAFDocColorTool,
    body_faces: &[Shape],
    core_faces: &[Shape],
    core_solid: &Shape,
) {
    let blue = ColorRgba::from_color(Color::named(NamedColor::BLUE));
    let all_faces: Vec<usize> = (1..=26).collect();
    color_faces(tool, body_faces, &all_faces, &blue);
    color_faces(tool, body_faces, &[1, 9], &hex_color("E68066"));
    let text_faces: Vec<usize> = (10..=22).collect();
    color_faces(tool, body_faces, &text_faces, &hex_color("99B300"));
    let lead = hex_color("1A1AFF");
    assert!(
        tool.set_shape_color(core_solid, &lead, XCAFDocColorType::ColorGen)
            .unwrap()
    );
    color_faces(tool, core_faces, &[1, 2, 3, 4, 5], &lead);
}

fn assert_color_names(tool: &XCAFDocColorTool) {
    let names: Vec<&str> = tool
        .get_colors()
        .unwrap()
        .iter()
        .map(|label| named(XCAFDocColorTool::get_color(label).unwrap()).unwrap())
        .collect();
    assert_eq!(names, ["BLUE", "SALMON2", "YELLOW4", "BLUE"]);
}
