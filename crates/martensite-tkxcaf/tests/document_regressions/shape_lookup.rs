//! Shape lookups and linked-assembly updates retain original document entries.
use super::subshape_and_assembly::label_at_entry;
use super::*;

fn document_entry(label: &TdfLabel) -> String {
    let mut entry = AsciiString::default();
    TdfTool::entry(label, &mut entry);
    String::from_utf8(entry.as_bytes().to_vec()).unwrap()
}

pub(super) use super::support::compound_shapes::compound_of;

fn translated_copy(part: &Shape, offset: [f64; 3]) -> Shape {
    part.clone()
        .moved(&Location::new(Transform3::translation(offset)), false)
        .unwrap()
}

const MAIN_SHAPE_LOOKUP_CASE: &str = "document_regressions::shape_lookup::main_shape_lookup_uses_definition_location_and_preserves_top_level_order";

#[test]
fn main_shape_lookup_uses_definition_location_and_preserves_top_level_order() {
    case_process_case::run(MAIN_SHAPE_LOOKUP_CASE, 1000, || {
        // OCC30727 copies and locates the part before the document is opened.
        let part = support::make_box(1., 1., 1.);
        let located = translated_copy(&part, [2., 0., 0.]);
        let context = XdeShapeRegressionContext::new();
        context.add_shape(&located, true);
        let located_face = first_face(&located);
        let faces = first_two_faces(&part);
        assert_main_shape_entry(context.shape_tool(), &faces[0], "0:1:1:2");
        assert_main_shape_entry(context.shape_tool(), &located_face, "");
        let parent = label_at_entry(&context, "0:1:1:2");
        context
            .shape_tool()
            .add_sub_shape(&parent, &faces[0])
            .unwrap();
        context.add_shape(&compound_of(&faces), true);
        assert_top_level_entries(context.shape_tool());
        context.close();
    });
}

fn first_two_faces(part: &Shape) -> [Shape; 2] {
    let mut faces = Explorer::new(part, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    let first = faces.current().clone();
    faces.next().unwrap();
    [first, faces.current().clone()]
}

fn assert_main_shape_entry(tool: &XCAFDocShapeTool, face: &Shape, expected: &str) {
    assert_eq!(
        document_entry(&tool.find_main_shape(face).unwrap()),
        expected
    );
}

fn assert_top_level_entries(tool: &XCAFDocShapeTool) {
    let entries = tool
        .get_shapes()
        .unwrap()
        .iter()
        .map(|label| format!("{} ", document_entry(label)))
        .collect::<String>();
    assert_eq!(entries, "0:1:1:1 0:1:1:2 0:1:1:3 0:1:1:4 0:1:1:5 ");
}

pub(super) fn corner_unit_box(x: f64) -> Shape {
    BRepPrimAPIMakeBox::at_point(Point3::new(x, 0., 0.), 1., 1., 1.)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape()
}

fn linked_box_assemblies() -> Shape {
    let first = support::make_box(1., 1., 1.);
    let second = corner_unit_box(2.);
    let third = corner_unit_box(-2.);
    let boxes = compound_of(&[first, second, third]);
    let upper_copy = translated_copy(&boxes, [0., 2., 0.]);
    let lower_copy = translated_copy(&boxes, [0., -2., 0.]);
    let upper = compound_of(&[upper_copy]);
    let lower = compound_of(&[lower_copy]);
    compound_of(&[upper, lower])
}

const LINKED_ASSEMBLIES_REFRESH_CASE: &str = "document_regressions::shape_lookup::linked_assemblies_refresh_both_instances_after_removing_a_box";

#[test]
fn linked_assemblies_refresh_both_instances_after_removing_a_box() {
    case_process_case::run(LINKED_ASSEMBLIES_REFRESH_CASE, 1000, || {
        // OCC31517 finishes both linked branches before creating the document.
        let assemblies = linked_box_assemblies();
        let context = XdeShapeRegressionContext::new();
        context.add_shape(&assemblies, true);
        let component = label_at_entry(&context, "0:1:1:3:1");
        context.shape_tool().remove_component(&component).unwrap();
        let definition = label_at_entry(&context, "0:1:1:4");
        context
            .shape_tool()
            .remove_shape(&definition, true)
            .unwrap();
        context.shape_tool().update_assemblies().unwrap();
        let result = context.shape_tool().get_one_shape().unwrap();
        assert_exploded_box_names(&result);
        context.close();
    });
}

fn assert_exploded_box_names(result: &Shape) {
    let count = exploded_solid_count(result);
    let names = (1..=count)
        .map(|index| format!("upd_{index}"))
        .collect::<Vec<_>>()
        .join(" ");
    assert_eq!(names, "upd_1 upd_2 upd_3 upd_4");
}

fn exploded_solid_count(shape: &Shape) -> usize {
    let mut unique = IndexedMapOfShape::default();
    for solid in Explorer::new(shape, ShapeEnum::Solid, ShapeEnum::Shape).unwrap() {
        unique.add(solid);
    }
    unique.extent()
}
