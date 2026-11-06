//! An initially empty assembly publishes its translated component on update.
use super::subshape_and_assembly::label_at_entry;
use super::*;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::Vertex;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

const CASE_COUNTS: [(ShapeEnum, usize); 5] = [
    (ShapeEnum::Vertex, 8),
    (ShapeEnum::Edge, 12),
    (ShapeEnum::Face, 6),
    (ShapeEnum::Shell, 1),
    (ShapeEnum::Solid, 1),
];
const TRANSLATED_TEN_CUBE_CORNERS: [[f64; 3]; 8] = [
    [10., 0., 0.],
    [10., 0., 10.],
    [10., 10., 0.],
    [10., 10., 10.],
    [20., 0., 0.],
    [20., 0., 10.],
    [20., 10., 0.],
    [20., 10., 10.],
];

fn populate_case_empty_assembly(context: &XdeShapeRegressionContext) -> TdfLabel {
    context.shape_tool().new_shape().unwrap();
    let box_shape = support::make_box(10., 10., 10.);
    context.add_shape(&box_shape, false);
    let placement = Location::new(Transform3::translation([10., 0., 0.]));
    let located = box_shape.moved(&placement, false).unwrap();
    let assembly = label_at_entry(context, "0:1:1:1");
    let component = context
        .shape_tool()
        .add_component_shape(&assembly, &located, false)
        .unwrap();
    assert!(!component.is_null());
    assembly
}

fn checked_assembly_shape(assembly: &TdfLabel) -> Shape {
    let mut result = Shape::default();
    assert!(XCAFDocShapeTool::get_shape(assembly, &mut result).unwrap());
    assert!(
        BRepCheckAnalyzer::new(&result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    result
}

fn assert_case_counts(shape: &Shape) {
    let mut unique = IndexedMapOfShape::default();
    topexp::map_shapes(shape, &mut unique, true, true);
    assert_eq!(unique.extent(), 35);
    for (kind, expected) in CASE_COUNTS {
        let count = unique
            .iter()
            .filter(|candidate| candidate.shape_type().unwrap() == kind)
            .count();
        assert_eq!(count, expected, "original {kind:?} count");
    }
}

fn assert_located_box_vertices(shape: &Shape) {
    let mut vertices = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Vertex, &mut vertices).unwrap();
    let mut corners = vertices
        .iter()
        .map(|vertex| {
            let point = BRepTool::pnt(&Vertex::unchecked(vertex.clone())).unwrap();
            [point.x(), point.y(), point.z()]
        })
        .collect::<Vec<[f64; 3]>>();
    corners.sort_by(|left, right| left.partial_cmp(right).unwrap());
    assert_eq!(corners, TRANSLATED_TEN_CUBE_CORNERS);
}

const EMPTY_ASSEMBLY_REFRESH_CASE: &str =
    "document_regressions::empty_assembly::empty_assembly_update_preserves_the_case_translated_box";

#[test]
fn empty_assembly_update_preserves_the_case_translated_box() {
    case_process_case::run(EMPTY_ASSEMBLY_REFRESH_CASE, 1000, || {
        let context = XdeShapeRegressionContext::new();
        let assembly = populate_case_empty_assembly(&context);
        let _ = checked_assembly_shape(&assembly);
        context.shape_tool().update_assemblies().unwrap();
        let updated = checked_assembly_shape(&assembly);
        assert_case_counts(&updated);
        assert_located_box_vertices(&updated);
        context.close();
    });
}
