//! A link assembly of a pin and two placed instances of one nut, with names and colors that differ
//! between a part and its instances.
use super::document_display::{
    DocumentDisplay, SavedView, fit_view, named, opened_view, redraw_document, save_view,
};
use super::solid_steps::{assert_counts, assert_valid_measures, children, translated};
use super::support::compound_shapes::compound_of;
use super::support::{new_document, shape_document::ensure_document_unit};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::quantity::{Color, ColorRgba, NamedColor};
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tklcaf::tdata_std::TDataStdName;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeOneAxis as _;
use martensite_tkxcaf::xcaf_doc::{XCAFDocColorTool, XCAFDocColorType, XCAFDocDocumentTool};

const EXPECTED_DUMP: &str = "ASSEMBLY COMPOUND 0:1:1:1 \"Link\" 
\tINSTANCE SOLID 0:1:1:1:1 (refers to 0:1:1:2) \"Pin instance\" 
\tINSTANCE SOLID 0:1:1:1:2 (refers to 0:1:1:3) \"Nut instance 1\" 
\tINSTANCE SOLID 0:1:1:1:3 (refers to 0:1:1:3) \"Nut instance 2\" 

PART SOLID 0:1:1:2 \"Pin\" 

PART SOLID 0:1:1:3 \"Nut\" 


Free Shapes: 1
ASSEMBLY COMPOUND  0:1:1:1 \"Link\" 
";

fn cylinder(radius: f64, height: f64) -> Shape {
    BRepPrimAPIMakeCylinder::new(radius, height)
        .solid()
        .unwrap()
        .into_shape()
}

fn set_name(label: &TdfLabel, name: &str) {
    TDataStdName::set(label, &ExtendedString::from_utf8(name)).unwrap();
}

fn set_named_color(tool: &XCAFDocColorTool, label: &TdfLabel, color: NamedColor) {
    let rgba = ColorRgba::from_color(Color::named(color));
    tool.set_color(label, &rgba, XCAFDocColorType::ColorGen)
        .unwrap();
}

struct LinkShapes {
    pin: Shape,
    nut: Shape,
    link: Shape,
}

/// Two instances of one nut, so the document finds one part with two placements.
fn link_shapes() -> LinkShapes {
    let pin = cylinder(1., 10.);
    let nut = cylinder(2., 2.);
    let nuts = children(&compound_of(&[nut.clone(), nut.clone()]));
    let link = compound_of(&[
        pin.clone(),
        translated(&nuts[0], 0., 0., 7.),
        translated(&nuts[1], 0., 0., 1.),
    ]);
    LinkShapes { pin, nut, link }
}

fn name_and_color(color_tool: &XCAFDocColorTool, link: &TdfLabel, pin: &TdfLabel, nut: &TdfLabel) {
    let instance = |tag| link.find_child(tag, false).unwrap();
    set_name(pin, "Pin");
    set_name(nut, "Nut");
    set_name(link, "Link");
    set_name(&instance(1), "Pin instance");
    set_name(&instance(2), "Nut instance 1");
    set_name(&instance(3), "Nut instance 2");
    set_named_color(color_tool, pin, NamedColor::WHITE);
    set_named_color(color_tool, nut, NamedColor::GRAY10);
    set_named_color(color_tool, &instance(2), NamedColor::RED);
    set_named_color(color_tool, &instance(3), NamedColor::GREEN);
}

/// Build, inspect and show the document.
pub fn run() {
    let shapes = link_shapes();
    assert_counts("pin", &shapes.pin, [2, 3, 3, 3, 1, 1, 0, 0, 13]);
    assert_counts("link", &shapes.link, [4, 6, 6, 6, 2, 2, 0, 1, 27]);
    assert_valid_measures("link", &shapes.link, 81.6814, 169.646);
    let (_application, document) = new_document("XCAF");
    ensure_document_unit(&document);
    let main = document.main().unwrap();
    let shape_handle = XCAFDocDocumentTool::shape_tool(&main).unwrap();
    let shape_tool = shape_handle.get().unwrap();
    let link_label = shape_tool.add_shape(&shapes.link, true, true).unwrap();
    let pin_label = shape_tool.find_shape_label(&shapes.pin, false).unwrap();
    let nut_label = shape_tool.find_shape_label(&shapes.nut, false).unwrap();
    let color_handle = XCAFDocDocumentTool::color_tool(&main).unwrap();
    let color_tool = color_handle.get().unwrap();
    name_and_color(color_tool, &link_label, &pin_label, &nut_label);
    let instance = |tag| link_label.find_child(tag, false).unwrap();
    assert_document(shape_tool, color_tool, &pin_label, &nut_label, &instance);
    let lifecycle = opened_view();
    let shown = DocumentDisplay::new(true).show_free_shapes(&lifecycle, "D", shape_tool);
    assert_eq!(
        shown,
        [
            "D:Link/Pin instance",
            "D:Link/Nut instance 1",
            "D:Link/Nut instance 2"
        ]
    );
    redraw_document(&lifecycle);
    fit_view(&lifecycle);
    assert_image(&save_view(&lifecycle, "link_assembly"));
}

fn assert_document(
    shape_tool: &martensite_tkxcaf::xcaf_doc::XCAFDocShapeTool,
    color_tool: &XCAFDocColorTool,
    pin: &TdfLabel,
    nut: &TdfLabel,
    instance: &dyn Fn(i32) -> TdfLabel,
) {
    let mut dump = Vec::new();
    shape_tool.dump_shapes(&mut dump, false).unwrap();
    assert_eq!(String::from_utf8(dump).unwrap().trim_start(), EXPECTED_DUMP);
    let color = |label: &TdfLabel| {
        named(
            XCAFDocColorTool::get_label_color(label, XCAFDocColorType::ColorGen)
                .unwrap()
                .map(|rgba| *rgba.rgb()),
        )
    };
    assert_eq!(color(pin), Some("WHITE"));
    assert_eq!(color(nut), Some("GRAY10"));
    assert_eq!(color(&instance(1)), None);
    assert_eq!(color(&instance(2)), Some("RED"));
    assert_eq!(color(&instance(3)), Some("GREEN"));
    assert_eq!(color_tool.get_colors().unwrap().len(), 4);
}

fn assert_image(image: &SavedView) {
    const BLACK: [u8; 3] = [0, 0, 0];
    const PIN: [u8; 3] = [179, 179, 179];
    image.assert_probes([
        PIN,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        PIN,
        [9, 168, 9],
        BLACK,
        BLACK,
        BLACK,
        BLACK,
    ]);
}
