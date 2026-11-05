//! Parent-kind refusal and located assembly refresh with the original document operations.
use super::*;
use martensite_tkernel::storage::format_general;
use martensite_tkgeombase::gprop::GProps;
use martensite_tklcaf::tdoc_std::TDocStdStandardApplication;
use martensite_tktopalgo::brep_gprop::BRepGProp;

pub(super) fn single_box_compound(part: &Shape) -> Shape {
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    Builder.add(&mut compound, part).unwrap();
    compound.into_shape()
}

pub(super) use super::support::shape_document::label_at_entry;

fn add_subshape_output(tool: &XCAFDocShapeTool, parent: &TdfLabel, child: &Shape) -> String {
    let label = tool.add_sub_shape(parent, child).unwrap();
    let mut entry = AsciiString::default();
    TdfTool::entry(&label, &mut entry);
    String::from_utf8(entry.as_bytes().to_vec()).unwrap()
}

fn refuse_assembly_subshape(application: &Handle<TDocStdStandardApplication>) {
    let part = support::make_box(1., 1., 1.);
    let compound = single_box_compound(&part);
    let context = XdeShapeRegressionContext::with_application(application);
    let parent = context.add_shape(&compound, true);
    assert_eq!(
        add_subshape_output(context.shape_tool(), &parent, &first_face(&compound)),
        ""
    );
    context.close();
}

fn refuse_reference_subshape(application: &Handle<TDocStdStandardApplication>) {
    let part = translated_unit_box();
    let context = XdeShapeRegressionContext::with_application(application);
    let parent = context.add_shape(&part, true);
    assert_eq!(
        add_subshape_output(context.shape_tool(), &parent, &first_face(&part)),
        ""
    );
    context.close();
}

pub(super) fn translated_unit_box() -> Shape {
    support::make_box(1., 1., 1.)
        .moved(&Location::new(Transform3::translation([1., 0., 0.])), false)
        .unwrap()
}

fn refuse_nested_subshape(application: &Handle<TDocStdStandardApplication>) {
    let part = support::make_box(1., 1., 1.);
    let context = XdeShapeRegressionContext::with_application(application);
    let parent = context.add_shape(&part, true);
    let face = first_face(&part);
    let child = context.shape_tool().add_sub_shape(&parent, &face).unwrap();
    let first_edge = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .current()
        .clone();
    assert_eq!(
        add_subshape_output(context.shape_tool(), &child, &first_edge),
        ""
    );
    context.close();
}

const PARENT_KIND_REFUSAL_CASE: &str = "document_regressions::subshape_and_assembly::assemblies_references_and_subshapes_refuse_nested_subshape_labels";

#[test]
fn assemblies_references_and_subshapes_refuse_nested_subshape_labels() {
    case_process_case::run(PARENT_KIND_REFUSAL_CASE, 1000, || {
        // OCC29890 reopens D on the same application after each independent parent check.
        let application = TDocStdStandardApplication::new();
        refuse_assembly_subshape(&application);
        refuse_reference_subshape(&application);
        refuse_nested_subshape(&application);
    });
}

const LOCATED_ASSEMBLY_REFRESH_CASE: &str =
    "document_regressions::subshape_and_assembly::located_assembly_refresh_uses_the_replaced_box";

#[test]
fn located_assembly_refresh_uses_the_replaced_box() {
    case_process_case::run(LOCATED_ASSEMBLY_REFRESH_CASE, 1000, || {
        // OCC29282 builds and locates the root before document construction.
        let part = support::make_box(1., 1., 1.);
        let compound = single_box_compound(&part)
            .moved(&Location::new(Transform3::translation([0., 1., 0.])), false)
            .unwrap();
        let context = XdeShapeRegressionContext::new();
        context.add_shape(&compound, true);
        let replacement = support::make_box(2., 1., 1.);
        let box_label = label_at_entry(&context, "0:1:1:3");
        context
            .shape_tool()
            .set_shape(&box_label, &replacement)
            .unwrap();
        context.shape_tool().update_assemblies().unwrap();
        assert_updated_mass(context.shape_tool());
        context.close();
    });
}

fn assert_updated_mass(tool: &XCAFDocShapeTool) {
    let result = tool.get_one_shape().unwrap();
    let mut properties = GProps::default();
    BRepGProp::volume_properties(&result, &mut properties, false, false, false).unwrap();
    assert_eq!(format_general(properties.mass(), 6), "2");
}

fn free_shape_count_from_dump(tool: &XCAFDocShapeTool) -> i32 {
    let mut bytes = Vec::new();
    tool.dump_shapes(&mut bytes, false).unwrap();
    let output = String::from_utf8(bytes).unwrap();
    output
        .split("Free Shapes: ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

fn remove_located_box(context: &XdeShapeRegressionContext, completely: bool, expected: i32) {
    context.add_shape(&translated_unit_box(), true);
    let label = label_at_entry(context, "0:1:1:1");
    context
        .shape_tool()
        .remove_shape(&label, completely)
        .unwrap();
    assert_eq!(free_shape_count_from_dump(context.shape_tool()), expected);
    // OCC23047 leaves D registered after the original count predicate.
}

const RETAIN_FREE_DEFINITION_CASE: &str = "document_regressions::subshape_and_assembly::removing_only_the_located_instance_keeps_one_free_definition";

#[test]
fn removing_only_the_located_instance_keeps_one_free_definition() {
    case_process_case::run(RETAIN_FREE_DEFINITION_CASE, 1000, || {
        let context = XdeShapeRegressionContext::new();
        remove_located_box(&context, false, 1);
    });
}

const REMOVE_FREE_DEFINITION_CASE: &str = "document_regressions::subshape_and_assembly::removing_the_located_instance_completely_keeps_no_free_definition";

#[test]
fn removing_the_located_instance_completely_keeps_no_free_definition() {
    case_process_case::run(REMOVE_FREE_DEFINITION_CASE, 1000, || {
        let context = XdeShapeRegressionContext::new();
        remove_located_box(&context, true, 0);
    });
}

fn assert_active_document_registered(
    application: &Handle<martensite_tklcaf::tdoc_std::TDocStdStandardApplication>,
    context: &XdeShapeRegressionContext,
    previous: &[Handle<TDocStdDocument>],
) {
    use martensite_tklcaf::tdoc_std::TDocStdApplication;
    assert_eq!(application.nb_documents().unwrap(), 1);
    assert_eq!(
        application.get_document(1).unwrap().as_ptr(),
        context.document.as_ptr()
    );
    assert!(
        previous
            .iter()
            .all(|document| document.as_ptr() != context.document.as_ptr())
    );
    assert!(context.shape_tool().get_shapes().unwrap().is_empty());
}

#[test]
fn shared_application_reopens_three_independent_shape_documents() {
    use martensite_tklcaf::tdoc_std::{TDocStdApplication, TDocStdStandardApplication};
    let application = TDocStdStandardApplication::new();
    let mut previous: Vec<Handle<TDocStdDocument>> = Vec::new();
    for _ in 0..3 {
        let context = XdeShapeRegressionContext::with_application(&application);
        assert_active_document_registered(&application, &context, &previous);
        context.add_shape(&support::make_box(1., 1., 1.), true);
        previous.push(context.document.clone());
        context.close();
        assert_eq!(application.nb_documents().unwrap(), 0);
    }
}

pub(super) fn assert_document_kept_in_session(
    application: &Handle<TDocStdStandardApplication>,
    document: &Handle<TDocStdDocument>,
) {
    use martensite_tklcaf::tdoc_std::TDocStdApplication;
    assert_eq!(application.nb_documents().unwrap(), 1);
    assert_eq!(
        application.get_document(1).unwrap().as_ptr(),
        document.as_ptr()
    );
    let owner = TDocStdDocument::get(&document.main().unwrap()).unwrap();
    assert_eq!(owner.as_ptr(), document.as_ptr());
}

#[test]
fn removal_predicates_keep_document_registered_without_explicit_close() {
    for (completely, expected) in [(false, 1), (true, 0)] {
        let application = TDocStdStandardApplication::new();
        let context = XdeShapeRegressionContext::with_application(&application);
        let document = context.document.clone();
        remove_located_box(&context, completely, expected);
        assert_document_kept_in_session(&application, &document);
        drop(context);
        assert_document_kept_in_session(&application, &document);
    }
}
