//! Reserved datum children are cleared; the source's other setters address a shape.
use super::*;
use martensite_tklcaf::tdata_std::TDataStdInteger;
use martensite_tklcaf::tdf::TdfTagSource;
use martensite_tkxcaf::xcaf_doc::{XCAFDocDatum, XCAFDocDimension, XCAFDocGeomTolerance};

fn child_with_integer(parent: &TdfLabel, tag: i32) -> TdfLabel {
    for _ in 1..tag {
        TdfTagSource::new_child_of(parent).unwrap();
    }
    let child = TdfTagSource::new_child_of(parent).unwrap();
    TDataStdInteger::set(&child, 100).unwrap();
    child
}

fn assert_integer_unchanged(label: &TdfLabel) {
    let mut integer = Handle::<TDataStdInteger>::default();
    assert!(
        label
            .find_attribute_as(&TDataStdInteger::get_id(), &mut integer)
            .unwrap()
    );
    assert_eq!(integer.get().unwrap().get(), 100);
}

fn set_shape_tolerance_value(label: &TdfLabel) {
    let mut attribute = Handle::<XCAFDocGeomTolerance>::default();
    if label
        .find_attribute_as(&XCAFDocGeomTolerance::get_id(), &mut attribute)
        .unwrap()
    {
        let object = attribute.get().unwrap().get_object().unwrap();
        object.get().unwrap().set_value(0.5);
        attribute.get().unwrap().set_object(&object).unwrap();
    }
}

fn set_shape_dimension_value(label: &TdfLabel) {
    let mut attribute = Handle::<XCAFDocDimension>::default();
    if label
        .find_attribute_as(&XCAFDocDimension::get_id(), &mut attribute)
        .unwrap()
    {
        let object = attribute.get().unwrap().get_object().unwrap();
        object.get().unwrap().set_value(0.5);
        attribute.get().unwrap().set_object(&object).unwrap();
    }
}

fn assert_datum_child_survives(
    tool: &martensite_tkxcaf::xcaf_doc::XCAFDocDimTolTool,
    shape: &TdfLabel,
) {
    let label = tool.add_datum().unwrap();
    tool.set_datum_shapes(std::slice::from_ref(shape), &label)
        .unwrap();
    let child = child_with_integer(&label, 20);
    let attribute = XCAFDocDatum::set(&label).unwrap();
    let object = attribute.get().unwrap().get_object().unwrap();
    object.get().unwrap().set_position(2);
    attribute.get().unwrap().set_object(&object).unwrap();
    assert_integer_unchanged(&child);
}

const RESERVED_ATTRIBUTE_CHILD_CASE: &str = "document_regressions::reserved_children::reserved_setters_preserve_the_case_unreserved_child_integers";

#[test]
fn reserved_setters_preserve_the_case_unreserved_child_integers() {
    case_process_case::run(RESERVED_ATTRIBUTE_CHILD_CASE, 1000, || {
        // OCC29854 creates the shape before opening its unmanaged document.
        let box_shape = support::make_box(10., 10., 10.);
        let document = TDocStdDocument::new(&ExtendedString::from_utf8("dummy")).unwrap();
        ensure_document_unit(&document);
        let shapes = XCAFDocDocumentTool::shape_tool(&document.main().unwrap()).unwrap();
        let shape = shapes
            .get()
            .unwrap()
            .add_shape(&box_shape, true, true)
            .unwrap();
        let tool = XCAFDocDocumentTool::dim_tol_tool(&document.main().unwrap()).unwrap();
        let tool = tool.get().unwrap();
        assert_datum_child_survives(tool, &shape);
        assert_tolerance_child_survives(tool, &shape);
        assert_dimension_child_survives(tool, &shape);
    });
}

fn assert_tolerance_child_survives(
    tool: &martensite_tkxcaf::xcaf_doc::XCAFDocDimTolTool,
    shape: &TdfLabel,
) {
    let tolerance = tool.add_geom_tolerance().unwrap();
    tool.set_geom_tolerance(shape, &tolerance).unwrap();
    let tolerance_child = child_with_integer(&tolerance, 19);
    set_shape_tolerance_value(shape);
    assert_integer_unchanged(&tolerance_child);
}

fn assert_dimension_child_survives(
    tool: &martensite_tkxcaf::xcaf_doc::XCAFDocDimTolTool,
    shape: &TdfLabel,
) {
    let dimension = tool.add_dimension().unwrap();
    tool.set_dimension(shape, &dimension).unwrap();
    let dimension_child = child_with_integer(&dimension, 19);
    set_shape_dimension_value(shape);
    assert_integer_unchanged(&dimension_child);
}
