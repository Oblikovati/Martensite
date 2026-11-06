//! Complete native E1 with staged original registry and both real edge attachments.
use super::{
    naming_box_driver::real_argument,
    naming_draw_document::NamingDocumentCase,
    naming_draw_driver_registration::register_case_drivers,
    naming_draw_modeling::{assert_no_errors, compute, function, initial_box_reads, prepare_box},
    naming_draw_observations::{NamingShapeBindings, new_command},
    naming_draw_primitives::{cylinder, first_face_edge_attachment},
    naming_draw_selection::{solve_flat_from, test_selection},
    naming_transform_arguments::set_object_argument,
};
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::{
    tdata_std::TDataStdUAttribute,
    tdf::TdfAttributeState,
    tfunction::{TFunctionDriverTable, TFunctionFunction, TFunctionLogbook},
};
struct FilletCutCase {
    lifecycle: NamingDocumentCase,
    table: Handle<TFunctionDriverTable>,
    bindings: NamingShapeBindings,
    object: Handle<TDataStdUAttribute>,
    functions: [Handle<TFunctionFunction>; 6],
}
fn source_box(
    lifecycle: &NamingDocumentCase,
    table: &Handle<TFunctionDriverTable>,
    bindings: &mut NamingShapeBindings,
) -> (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>) {
    let (object, function) = prepare_box(&lifecycle.document, [190.0, 290.0, 390.0], true);
    register_case_drivers(table, &["43", "54", "45"]);
    compute(table, &function);
    initial_box_reads(bindings, &object, &function, true);
    (object, function)
}
fn cut(graph: &mut FilletCutCase, tool: &Handle<TDataStdUAttribute>) -> Handle<TFunctionFunction> {
    new_command(&graph.lifecycle.document);
    register_case_drivers(&graph.table, &["48"]);
    let cut = function(&graph.object, "48", "Cut");
    set_object_argument(&cut, 1, tool).unwrap();
    compute(&graph.table, &cut);
    graph.bindings.bind(&cut, "S").unwrap();
    cut
}
fn fillet(graph: &mut FilletCutCase) -> [Handle<TFunctionFunction>; 2] {
    new_command(&graph.lifecycle.document);
    graph.bindings.explode_face_edge("S", 1);
    let (path, selection) = first_face_edge_attachment(
        &graph.lifecycle.document,
        &graph.object,
        &graph.functions[3],
    );
    register_case_drivers(&graph.table, &["53"]);
    let fillet = function(&graph.object, "53", "Fillet");
    real_argument(&fillet, 1).unwrap().set_value(25.0).unwrap();
    set_object_argument(&fillet, 3, &path).unwrap();
    compute(&graph.table, &fillet);
    graph.bindings.bind(&fillet, "S").unwrap();
    [selection, fillet]
}
fn fillet_cut_graph(
    lifecycle: NamingDocumentCase,
    table: Handle<TFunctionDriverTable>,
    bindings: NamingShapeBindings,
    object: Handle<TDataStdUAttribute>,
    prefix: [Handle<TFunctionFunction>; 3],
) -> FilletCutCase {
    let mut functions = std::array::from_fn(|_| Handle::default());
    functions[..3].clone_from_slice(&prefix);
    FilletCutCase {
        lifecycle,
        table,
        bindings,
        object,
        functions,
    }
}
fn e1_modeling_prefix(
    lifecycle: NamingDocumentCase,
) -> (FilletCutCase, Handle<TDataStdUAttribute>) {
    let table = Handle::new(TFunctionDriverTable::new());
    let mut bindings = NamingShapeBindings::for_case(&lifecycle);
    let (object, source) = source_box(&lifecycle, &table, &mut bindings);
    new_command(&lifecycle.document);
    bindings.explode_face_edge("Box2", 1);
    let (axis, selection) = first_face_edge_attachment(&lifecycle.document, &object, &source);
    register_case_drivers(&table, &["54", "45"]);
    let (tool, cylinder) = cylinder(&table, &lifecycle.document, &axis, [30.0, 315.0]);
    let graph = fillet_cut_graph(
        lifecycle,
        table,
        bindings,
        object,
        [source, selection, cylinder],
    );
    (graph, tool)
}
fn initial_e1(lifecycle: NamingDocumentCase) -> FilletCutCase {
    let (mut graph, tool) = e1_modeling_prefix(lifecycle);
    graph.functions[3] = cut(&mut graph, &tool);
    let [selection, fillet] = fillet(&mut graph);
    graph.functions[4] = selection;
    graph.functions[5] = fillet;
    graph
}
fn recompute(graph: &mut FilletCutCase) {
    new_command(&graph.lifecycle.document);
    real_argument(&graph.functions[0], 1)
        .unwrap()
        .set_value(290.0)
        .unwrap();
    new_command(&graph.lifecycle.document);
    TFunctionLogbook::set(&graph.functions[0].label())
        .unwrap()
        .clear()
        .unwrap();
    for function in &graph.functions {
        compute(&graph.table, function);
    }
    graph.bindings.bind(&graph.functions[5], "S").unwrap();
    graph.bindings.presentation().only("S");
}
fn execute_e1(lifecycle: NamingDocumentCase) {
    let mut graph = initial_e1(lifecycle);
    new_command(&graph.lifecycle.document);
    let single = test_selection(&graph.lifecycle.document, &graph.object, false, false).unwrap();
    let multiple = test_selection(&graph.lifecycle.document, &graph.object, false, true).unwrap();
    assert_no_errors(&single);
    assert_no_errors(&multiple);
    println!(
        "{{\"event\":\"draw-e1-attempts\",\"single\":{},\"multiple\":{}}}",
        single.attempted, multiple.attempted
    );
    recompute(&mut graph);
    assert!(
        solve_flat_from(&graph.table, &single.first)
            .unwrap()
            .is_empty()
    );
    new_command(&graph.lifecycle.document);
    graph.lifecycle.close();
}

#[test]
fn caf_named_shape_e1_recomputes_every_single_and_multiple_selection() {
    execute_e1(NamingDocumentCase::new());
}

/// The tests/caf/driver/A1 body on the caf/begin document; the `caf_driver_cases`
/// suite runs it in its policy child.
pub(crate) fn caf_driver_a1_recomputes_case_graph_with_retained_begin_document(
    lifecycle: NamingDocumentCase,
) {
    execute_e1(lifecycle);
}
