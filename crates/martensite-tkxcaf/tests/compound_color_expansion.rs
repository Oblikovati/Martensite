//! Expansion keeps independently assigned colors on the original three boxes.
mod support;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkxcaf::xcaf_doc::{XCAFDocColorTool, XCAFDocColorType, XCAFDocEditor};
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocShapeTool};
use support::compound_shapes::compound_of;
use support::shape_document::{XdeShapeRegressionContext, label_at_entry};

fn three_coincident_boxes() -> [Shape; 3] {
    std::array::from_fn(|_| support::make_box(10., 10., 10.))
}

fn assign_compound_colors(context: &XdeShapeRegressionContext, boxes: &[Shape; 3]) {
    let colors = XCAFDocDocumentTool::color_tool(&context.document.main().unwrap()).unwrap();
    for (shape, rgb) in boxes.iter().zip([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]) {
        let color = Color::new(rgb[0], rgb[1], rgb[2], ColorSpace::Srgb).unwrap();
        assert!(
            colors
                .get()
                .unwrap()
                .set_shape_color_rgb(shape, &color, XCAFDocColorType::ColorGen)
                .unwrap()
        );
    }
}

fn assert_color_name(context: &XdeShapeRegressionContext, entry: &str, expected: &str) {
    let label = label_at_entry(context, entry);
    let color = XCAFDocColorTool::get_label_color_rgb(&label, XCAFDocColorType::ColorGen)
        .unwrap()
        .unwrap();
    assert!(Color::string_name(color.name()).contains(expected));
}

#[test]
fn expanded_compound_keeps_red_green_blue_part_colors() {
    support::case_policy::run_with_case_policy(
        "expanded_compound_keeps_red_green_blue_part_colors",
        case_compound_color_expansion,
    );
}

fn case_compound_color_expansion() {
    let context = XdeShapeRegressionContext::new();
    let boxes = three_coincident_boxes();
    context.add_shape(&compound_of(&boxes), false);
    assign_compound_colors(&context, &boxes);
    assert!(XCAFDocEditor::expand(&context.document.main().unwrap(), false).unwrap());
    assert_expanded_compound_dump(context.shape_tool());
    assert_color_name(&context, "0:1:1:2", "RED");
    assert_color_name(&context, "0:1:1:3", "GREEN");
    assert_color_name(&context, "0:1:1:4", "BLUE");
    context.close();
}

fn assert_expanded_compound_dump(tool: &XCAFDocShapeTool) {
    let mut dump = Vec::new();
    tool.dump_shapes(&mut dump, false).unwrap();
    assert!(
        String::from_utf8(dump)
            .unwrap()
            .contains("ASSEMBLY COMPOUND 0:1:1:1 \"COMPOUND\"")
    );
}
