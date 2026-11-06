//! The dimension presentation remains on its fixed document child label.
use super::shape_lookup::{compound_of, corner_unit_box};
use super::subshape_and_assembly::label_at_entry;
use super::*;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tkxcaf::xcaf_dim_tol_objects::{
    XCAFDimTolObjectsDimensionObject, XCAFDimTolObjectsDimensionType,
};
use martensite_tkxcaf::xcaf_doc::XCAFDocDimension;

fn dimension_input_labels(context: &XdeShapeRegressionContext) -> [TdfLabel; 2] {
    let part = support::make_box(1., 1., 1.);
    let compound = compound_of(&[part.clone(), corner_unit_box(2.)]);
    let parent = context.add_shape(&compound, false);
    let simple = context.add_shape(&part, true);
    let face = first_face(&part);
    context.shape_tool().add_sub_shape(&parent, &face).unwrap();
    context.shape_tool().add_sub_shape(&simple, &face).unwrap();
    [parent, simple]
}

fn dimension_attribute(context: &XdeShapeRegressionContext) -> Handle<XCAFDocDimension> {
    let [first, second] = dimension_input_labels(context);
    let tool = XCAFDocDocumentTool::dim_tol_tool(&context.document.main().unwrap()).unwrap();
    let label = tool.get().unwrap().add_dimension().unwrap();
    tool.get()
        .unwrap()
        .set_dimension_pair(&first, &second, &label)
        .unwrap();
    assert_eq!(label, label_at_entry(context, "0:1:4:1"));
    XCAFDocDimension::set(&label).unwrap()
}

fn edit_dimension(
    attribute: &XCAFDocDimension,
    edit: impl FnOnce(&XCAFDimTolObjectsDimensionObject),
) {
    let object = attribute.get_object().unwrap();
    edit(object.get().unwrap());
    attribute.set_object(&object).unwrap();
}

fn configure_dimension(attribute: &XCAFDocDimension) {
    edit_dimension(attribute, |object| {
        object.set_type(XCAFDimTolObjectsDimensionType::LocationLinearDistance)
    });
    edit_dimension(attribute, |object| object.set_value(6.));
    edit_dimension(attribute, |object| {
        object.set_lower_tol_value(-0.001);
        object.set_upper_tol_value(0.002);
    });
    edit_dimension(attribute, |object| {
        object.set_point(&dimension_vertex_point(10.));
        object.set_point2(&dimension_vertex_point(20.));
    });
}

fn dimension_vertex_point(coordinate: f64) -> Point3 {
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(coordinate, coordinate, coordinate))
        .unwrap()
        .vertex()
        .unwrap();
    BRepTool::pnt(&vertex).unwrap()
}

fn assert_presentation_topology(shape: &Shape) {
    for (kind, expected) in [
        (ShapeEnum::Vertex, 8),
        (ShapeEnum::Edge, 12),
        (ShapeEnum::Wire, 6),
        (ShapeEnum::Face, 6),
    ] {
        let mut unique = IndexedMapOfShape::default();
        for entity in Explorer::new(shape, kind, ShapeEnum::Shape).unwrap() {
            unique.add(entity);
        }
        assert_eq!(unique.extent(), expected);
    }
}

#[test]
fn dimension_presentation_stays_on_child_fifteen_with_case_area() {
    super::case_process_case::run(
        "document_regressions::dimension_presentation::dimension_presentation_stays_on_child_fifteen_with_case_area",
        1000,
        case_dimension_presentation,
    );
}

fn case_dimension_presentation() {
    let context = XdeShapeRegressionContext::new();
    let attribute = dimension_attribute(&context);
    configure_dimension(attribute.get().unwrap());
    store_dimension_presentation(attribute.get().unwrap());
    let label = label_at_entry(&context, "0:1:4:1:15");
    let presentation = XCAFDocShapeTool::shape_of(&label).unwrap();
    assert_presentation_topology(&presentation);
    let mut properties = GProps::default();
    BRepGProp::surface_properties_eps(&presentation, &mut properties, 1e-2, false).unwrap();
    assert_ne!(properties.mass(), 0.);
    assert!(((1.5 - properties.mass()) / 1.5).abs() <= 1e-2);
    context.close();
}

fn store_dimension_presentation(attribute: &XCAFDocDimension) {
    edit_dimension(attribute, |object| {
        object.set_presentation(
            &support::make_box(0.5, 0.5, 0.5),
            &Handle::new(HAsciiString::new("temp presentation")),
        )
    });
}
