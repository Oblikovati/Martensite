//! Complete source-bound E4/E5 cylinder-cut naming command pipelines.
use super::{
    naming_box_driver::real_argument,
    naming_draw_document::NamingDocumentCase,
    naming_draw_driver_registration::case_driver_table,
    naming_draw_modeling::{assert_no_errors, compute, function, initial_box, reals},
    naming_draw_observations::{NamingShapeBindings, new_command},
    naming_draw_primitives::{cylinder, first_face_edge_attachment},
    naming_draw_selection::{solve_flat_from, test_selection},
    naming_transform_arguments::set_object_argument,
};
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::{
    tdata_std::TDataStdUAttribute,
    tdf::TdfAttributeState,
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionDriverTable, TFunctionFunction, TFunctionLogbook},
};
struct CylinderCutCase {
    lifecycle: NamingDocumentCase,
    table: Handle<TFunctionDriverTable>,
    bindings: NamingShapeBindings,
    object: Handle<TDataStdUAttribute>,
    functions: [Handle<TFunctionFunction>; 5],
}
fn translated_cylinder(
    table: &Handle<TFunctionDriverTable>,
    document: &Handle<TDocStdDocument>,
    bindings: &mut NamingShapeBindings,
    tool: &Handle<TDataStdUAttribute>,
) -> Handle<TFunctionFunction> {
    new_command(document);
    let translation = function(tool, "56", "ParTranslation");
    reals(&translation, [100.0, 0.0, 0.0]);
    compute(table, &translation);
    bindings.bind(&translation, "CylS").unwrap();
    new_command(document);
    translation
}
fn translated_cut(
    table: &Handle<TFunctionDriverTable>,
    document: &Handle<TDocStdDocument>,
    bindings: &mut NamingShapeBindings,
    source: &Handle<TDataStdUAttribute>,
    tool: &Handle<TDataStdUAttribute>,
) -> [Handle<TFunctionFunction>; 2] {
    let translation = translated_cylinder(table, document, bindings, tool);
    let cut = function(source, "48", "Cut");
    set_object_argument(&cut, 1, tool).unwrap();
    compute(table, &cut);
    bindings.bind_after_erase(&cut, "S").unwrap();
    bindings.presentation().only("S");
    new_command(document);
    [translation, cut]
}
fn cylinder_cut(lifecycle: NamingDocumentCase, height: f64) -> CylinderCutCase {
    let document = &lifecycle.document;
    let table = case_driver_table();
    let mut bindings = NamingShapeBindings::for_case(&lifecycle);
    let (object, source) =
        initial_box(&table, document, &mut bindings, [190.0, 290.0, 390.0], true);
    new_command(document);
    bindings.explode_face_edge("Box2", 1);
    let (axis, selection) = first_face_edge_attachment(document, &object, &source);
    let (tool, cylinder) = cylinder(&table, document, &axis, [70.0, height]);
    bindings.bind(&cylinder, "CylS").unwrap();
    let [translation, cut] = translated_cut(&table, document, &mut bindings, &object, &tool);
    CylinderCutCase {
        lifecycle,
        table,
        bindings,
        object,
        functions: [source, selection, cylinder, translation, cut],
    }
}
fn recompute(graph: &mut CylinderCutCase, dx: f64) {
    let document = &graph.lifecycle.document;
    new_command(document);
    real_argument(&graph.functions[0], 1)
        .unwrap()
        .set_value(dx)
        .unwrap();
    new_command(document);
    TFunctionLogbook::set(&graph.functions[0].label())
        .unwrap()
        .clear()
        .unwrap();
    for function in &graph.functions {
        compute(&graph.table, function);
    }
    let result = &graph.functions[4];
    graph.bindings.bind_after_erase(result, "S").unwrap();
    graph.bindings.presentation().only("S");
}
fn execute_case(lifecycle: NamingDocumentCase, height: f64, dx: f64, multiple: bool) {
    let mut graph = cylinder_cut(lifecycle, height);
    let single = test_selection(&graph.lifecycle.document, &graph.object, false, false).unwrap();
    assert_no_errors(&single);
    if multiple {
        let batch = test_selection(&graph.lifecycle.document, &graph.object, false, true).unwrap();
        assert_no_errors(&batch);
    }
    println!(
        "{{\"event\":\"draw-cylinder-cut-attempts\",\"height\":{height},\"single\":{}}}",
        single.attempted
    );
    recompute(&mut graph, dx);
    assert!(
        solve_flat_from(&graph.table, &single.first)
            .unwrap()
            .is_empty()
    );
    new_command(&graph.lifecycle.document);
    graph.lifecycle.close();
}
#[test]
fn caf_named_shape_e4_recomputes_every_single_and_multiple_selection() {
    execute_case(NamingDocumentCase::new(), 515.0, 250.0, true);
}
#[test]
fn caf_named_shape_e5_recomputes_every_single_selection() {
    execute_case(NamingDocumentCase::new(), 275.0, 220.0, false);
}

/// The tests/caf/driver/A4 body on the caf/begin document; the `caf_driver_cases`
/// suite runs it in its policy child.
pub(crate) fn caf_driver_a4_recomputes_case_graph_with_retained_begin_document(
    lifecycle: NamingDocumentCase,
) {
    execute_case(lifecycle, 515.0, 250.0, true);
}

/// The tests/caf/driver/A5 body on the caf/begin document; the `caf_driver_cases`
/// suite runs it in its policy child.
pub(crate) fn caf_driver_a5_recomputes_case_graph_with_retained_begin_document(
    lifecycle: NamingDocumentCase,
) {
    execute_case(lifecycle, 275.0, 220.0, false);
}
