//! Complete native E3 kernel case, with source-bound document, registry and read/diagnostic acceptance.
use super::{
    naming_box_driver::real_argument,
    naming_draw_document::NamingDocumentCase,
    naming_draw_driver_registration::case_driver_table,
    naming_draw_modeling::{
        ModeledObject, assert_no_errors, compute, function, initial_box, reals,
    },
    naming_draw_observations::{NamingShapeBindings, new_command},
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
struct E3Graph {
    lifecycle: NamingDocumentCase,
    bindings: NamingShapeBindings,
    table: Handle<TFunctionDriverTable>,
    document: Handle<TDocStdDocument>,
    object: Handle<TDataStdUAttribute>,
    functions: [Handle<TFunctionFunction>; 4],
}
fn initial_boxes(
    table: &Handle<TFunctionDriverTable>,
    document: &Handle<TDocStdDocument>,
    bindings: &mut NamingShapeBindings,
) -> [ModeledObject; 2] {
    [
        initial_box(table, document, bindings, [190.0, 290.0, 390.0], true),
        initial_box(table, document, bindings, [120.0, 140.0, 150.0], false),
    ]
}
fn translated_cut(
    table: &Handle<TFunctionDriverTable>,
    document: &Handle<TDocStdDocument>,
    bindings: &mut NamingShapeBindings,
    object: &Handle<TDataStdUAttribute>,
    tool: &Handle<TDataStdUAttribute>,
) -> [Handle<TFunctionFunction>; 2] {
    new_command(document);
    let translation = function(tool, "56", "ParTranslation");
    reals(&translation, [150.0, -40.0, 270.0]);
    compute(table, &translation);
    bindings.bind(&translation, "Box").unwrap();
    new_command(document);
    let cut = function(object, "48", "Cut");
    set_object_argument(&cut, 1, tool).unwrap();
    compute(table, &cut);
    bindings.bind_after_erase(&cut, "S").unwrap();
    new_command(document);
    [translation, cut]
}
fn initial_e3(lifecycle: NamingDocumentCase) -> E3Graph {
    let table = case_driver_table();
    let document = lifecycle.document.clone();
    let mut bindings = NamingShapeBindings::for_case(&lifecycle);
    let [(object, first), (tool, second)] = initial_boxes(&table, &document, &mut bindings);
    let [translation, cut] = translated_cut(&table, &document, &mut bindings, &object, &tool);
    E3Graph {
        lifecycle,
        bindings,
        table,
        document,
        object,
        functions: [first, second, translation, cut],
    }
}
fn recompute(graph: &mut E3Graph) {
    new_command(&graph.document);
    real_argument(&graph.functions[0], 1)
        .unwrap()
        .set_value(220.0)
        .unwrap();
    new_command(&graph.document);
    TFunctionLogbook::set(&graph.functions[0].label())
        .unwrap()
        .clear()
        .unwrap();
    for function in &graph.functions {
        compute(&graph.table, function);
    }
    graph
        .bindings
        .bind_after_erase(&graph.functions[3], "S")
        .unwrap();
    graph.bindings.presentation().only("S");
}
fn execute_e3(lifecycle: NamingDocumentCase) {
    let mut graph = initial_e3(lifecycle);
    let single = test_selection(&graph.document, &graph.object, true, false).unwrap();
    let multiple = test_selection(&graph.document, &graph.object, false, true).unwrap();
    assert_no_errors(&single);
    assert_no_errors(&multiple);
    assert_eq!((single.attempted, multiple.attempted), (145, 5));
    recompute(&mut graph);
    assert!(
        solve_flat_from(&graph.table, &single.first)
            .unwrap()
            .is_empty()
    );
    new_command(&graph.document);
    graph.lifecycle.close();
}

#[test]
fn caf_named_shape_e3_recomputes_every_single_and_multiple_selection() {
    execute_e3(NamingDocumentCase::new());
}

/// The tests/caf/driver/A3 body on the caf/begin document; the `caf_driver_cases`
/// suite runs it in its policy child.
pub(crate) fn caf_driver_a3_recomputes_case_graph_with_retained_begin_document(
    lifecycle: NamingDocumentCase,
) {
    execute_e3(lifecycle);
}
