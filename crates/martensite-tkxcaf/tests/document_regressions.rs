//! Native document regressions for aggregate shapes, parent-scoped subshapes and layers.
#[path = "support/headless_document.rs"]
mod case_process_case;
mod support;

#[path = "document_regressions/subshape_and_assembly.rs"]
mod subshape_and_assembly;

#[path = "document_regressions/shape_dump.rs"]
mod shape_dump;

#[path = "document_regressions/shape_lookup.rs"]
mod shape_lookup;

#[path = "document_regressions/dimension_presentation.rs"]
mod dimension_presentation;

#[path = "document_regressions/reserved_children.rs"]
mod reserved_children;

#[path = "document_regressions/saved_view_definition.rs"]
mod saved_view_definition;

use martensite_tkbrep::{
    topexp::{self, Explorer},
    topods::{Builder, Compound, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::{
    standard::Handle,
    tcollection::{AsciiString, ExtendedString},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::{
    tdf::{TdfLabel, TdfTool},
    tdoc_std::TDocStdDocument,
};
use martensite_tkmath::{
    gp::{Point3, Transform3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocShapeTool};

use support::shape_document::{XdeShapeRegressionContext, ensure_document_unit};

const AGGREGATE_BOX_PARTS_CASE: &str =
    "document_regressions::one_shape_collects_two_translated_box_parts";
const FACE_LAYER_LOOKUP_CASE: &str =
    "document_regressions::layer_lookup_by_face_finds_the_registered_subshape";

fn topology_counts(shape: &Shape) -> (usize, usize) {
    let mut shapes = IndexedMapOfShape::default();
    topexp::map_shapes(shape, &mut shapes, true, true);
    let compounds = (1..=shapes.extent())
        .filter(|&index| {
            shapes.find_key(index).unwrap().shape_type().unwrap() == ShapeEnum::Compound
        })
        .count();
    (shapes.extent(), compounds)
}

#[test]
fn one_shape_collects_two_translated_box_parts() {
    case_process_case::run(AGGREGATE_BOX_PARTS_CASE, 1000, || {
        let context = XdeShapeRegressionContext::new();
        let shapes = context.shape_tool();
        context.add_shape(&support::make_box(10., 10., 10.), true);
        assert_eq!(topology_counts(&shapes.get_one_shape().unwrap()).0, 34);
        let translated = support::make_box(10., 10., 10.)
            .moved(&Location::new(Transform3::translation([20., 0., 0.])), true)
            .unwrap();
        context.add_shape(&translated, true);
        assert_eq!(topology_counts(&shapes.get_one_shape().unwrap()), (69, 1));
        context.close();
    });
}

fn first_face(shape: &Shape) -> Shape {
    let faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    assert!(faces.more());
    faces.current().clone()
}

#[test]
fn layer_lookup_by_face_finds_the_registered_subshape() {
    case_process_case::run(FACE_LAYER_LOOKUP_CASE, 1000, || {
        let context = XdeShapeRegressionContext::new();
        let box_shape = support::make_box(1., 1., 1.);
        let face = first_face(&box_shape);
        let part = context.add_shape(&box_shape, true);
        let subshape = context.shape_tool().add_sub_shape(&part, &face).unwrap();
        let layers = XCAFDocDocumentTool::layer_tool(&context.document.main().unwrap()).unwrap();
        let layer_name = ExtendedString::from_utf8("myLayer");
        layers.add_layer(&layer_name).unwrap();
        layers.set_layer(&subshape, &layer_name, false).unwrap();
        let output = layer_query_output(&layers.get_shape_layers(&face).unwrap());
        assert_eq!(output, "\"myLayer\" ");
        context.close();
    });
}

fn layer_query_output(layers: &[ExtendedString]) -> String {
    layers
        .iter()
        .map(|name| format!("\"{}\" ", name.to_utf8()))
        .collect()
}

fn part_and_compound() -> (Shape, Shape) {
    let part = support::make_box(1., 1., 1.);
    let other = BRepPrimAPIMakeBox::at_point(Point3::new(2., 0., 0.), 1., 1., 1.)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape();
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    let mut shape = compound.into_shape();
    Builder.add(&mut shape, &part).unwrap();
    Builder.add(&mut shape, &other).unwrap();
    (part, shape)
}

fn assert_subshape_entry(tool: &XCAFDocShapeTool, parent: &TdfLabel, face: &Shape, expected: &str) {
    let mut found = TdfLabel::default();
    assert!(tool.find_sub_shape(parent, face, &mut found).unwrap());
    let mut entry = AsciiString::default();
    TdfTool::entry(&found, &mut entry);
    assert_eq!(entry, expected);
}

fn register_subshape_parents(
    context: &XdeShapeRegressionContext,
    part: &Shape,
    compound: &Shape,
) -> (TdfLabel, TdfLabel, TdfLabel, Shape) {
    let tool = context.shape_tool();
    let compound_label = context.add_shape(compound, false);
    let part_label = context.add_shape(part, true);
    let face = first_face(part);
    let first = tool.add_sub_shape(&compound_label, &face).unwrap();
    tool.add_sub_shape(&part_label, &face).unwrap();
    (compound_label, part_label, first, face)
}

const PARENT_SUBSHAPE_FORGET_CASE: &str =
    "document_regressions::forgetting_one_parent_subshape_preserves_the_other_parent_mapping";

#[test]
fn forgetting_one_parent_subshape_preserves_the_other_parent_mapping() {
    case_process_case::run(PARENT_SUBSHAPE_FORGET_CASE, 1000, || {
        // OCC29821 constructs both shape owners before the document exists.
        let (part, compound) = part_and_compound();
        let context = XdeShapeRegressionContext::new();
        let tool = context.shape_tool();
        let (compound_label, part_label, first, face) =
            register_subshape_parents(&context, &part, &compound);
        assert_subshape_entry(tool, &compound_label, &face, "0:1:1:1:1");
        assert_subshape_entry(tool, &part_label, &face, "0:1:1:2:1");
        first.forget_all_attributes(true).unwrap();
        assert!(
            !tool
                .find_sub_shape(&compound_label, &face, &mut TdfLabel::default())
                .unwrap()
        );
        assert_subshape_entry(tool, &part_label, &face, "0:1:1:2:1");
        context.close();
    });
}

#[path = "document_regressions/empty_assembly.rs"]
mod empty_assembly;
