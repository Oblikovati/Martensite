//! Complete native F4: both attached prisms, parameter edit, fusion and naming recompute.
use super::{
    naming_box_driver::real_argument,
    naming_draw_document::NamingDocumentCase,
    naming_draw_driver_registration::{register_case_drivers, sweep_driver_table},
    naming_draw_modeling::{assert_no_errors, compute, function, standard_box},
    naming_draw_observations::{NamingShapeBindings, new_command},
    naming_draw_primitives::{face_attachment, prism},
    naming_draw_selection::{solve_flat_from, test_selection},
    naming_transform_arguments::set_object_argument,
};
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::{
    tdata_std::TDataStdUAttribute,
    tdf::TdfAttributeState,
    tfunction::{TFunctionDriverTable, TFunctionFunction, TFunctionLogbook},
};
struct PrismFuseCase {
    lifecycle: NamingDocumentCase,
    table: Handle<TFunctionDriverTable>,
    bindings: NamingShapeBindings,
    object: Handle<TDataStdUAttribute>,
    functions: [Handle<TFunctionFunction>; 6],
}
fn extend_case_prism(function: &Handle<TFunctionFunction>) {
    let height = real_argument(function, 2).unwrap();
    height.set_value(500.0).unwrap();
}
fn first_prism(
    lifecycle: &NamingDocumentCase,
    table: &Handle<TFunctionDriverTable>,
    bindings: &mut NamingShapeBindings,
    box_object: &Handle<TDataStdUAttribute>,
    source: &Handle<TFunctionFunction>,
) -> (Handle<TDataStdUAttribute>, [Handle<TFunctionFunction>; 2]) {
    new_command(&lifecycle.document);
    bindings.explode("Box2", martensite_tkg3d::top_abs::ShapeEnum::Face);
    let (basis, selection) = face_attachment(&lifecycle.document, box_object, source, 1);
    let (object, function) = prism(table, &lifecycle.document, &basis, 300.0);
    bindings.bind(&function, "Pr1").unwrap();
    extend_case_prism(&function);
    compute(table, &function);
    bindings.bind(&function, "Pr1").unwrap();
    bindings.presentation().fit();
    (object, [selection, function])
}
fn second_prism(
    lifecycle: &NamingDocumentCase,
    table: &Handle<TFunctionDriverTable>,
    bindings: &mut NamingShapeBindings,
    first_object: &Handle<TDataStdUAttribute>,
    first_function: &Handle<TFunctionFunction>,
) -> (Handle<TDataStdUAttribute>, [Handle<TFunctionFunction>; 2]) {
    new_command(&lifecycle.document);
    bindings.explode("Pr1", martensite_tkg3d::top_abs::ShapeEnum::Face);
    let (basis, selection) = face_attachment(&lifecycle.document, first_object, first_function, 1);
    let (object, function) = prism(table, &lifecycle.document, &basis, 500.0);
    bindings.bind(&function, "Pr2").unwrap();
    (object, [selection, function])
}
fn prism_documents(
    lifecycle: NamingDocumentCase,
) -> (NamingDocumentCase, Handle<TFunctionDriverTable>) {
    let table = sweep_driver_table();
    (lifecycle, table)
}
fn initial_prisms(lifecycle: NamingDocumentCase) -> (PrismFuseCase, Handle<TDataStdUAttribute>) {
    let (lifecycle, table) = prism_documents(lifecycle);
    let mut bindings = NamingShapeBindings::for_case(&lifecycle);
    let (box_object, source) = standard_box(&table, &lifecycle.document, &mut bindings, true);
    let (object, [basis1, prism1]) =
        first_prism(&lifecycle, &table, &mut bindings, &box_object, &source);
    let (tool, [basis2, prism2]) =
        second_prism(&lifecycle, &table, &mut bindings, &object, &prism1);
    let functions = [source, basis1, prism1, basis2, prism2, Handle::default()];
    (
        PrismFuseCase {
            lifecycle,
            table,
            bindings,
            object,
            functions,
        },
        tool,
    )
}
fn initial_f4(lifecycle: NamingDocumentCase) -> PrismFuseCase {
    let (mut graph, tool) = initial_prisms(lifecycle);
    new_command(&graph.lifecycle.document);
    register_case_drivers(&graph.table, &["49"]);
    let fuse = function(&graph.object, "49", "Fuse");
    set_object_argument(&fuse, 1, &tool).unwrap();
    compute(&graph.table, &fuse);
    graph.bindings.bind(&fuse, "Pr1").unwrap();
    graph.functions[5] = fuse;
    graph
}
fn recompute(graph: &PrismFuseCase) {
    new_command(&graph.lifecycle.document);
    real_argument(&graph.functions[0], 3)
        .unwrap()
        .set_value(420.0)
        .unwrap();
    new_command(&graph.lifecycle.document);
    TFunctionLogbook::set(&graph.functions[0].label())
        .unwrap()
        .clear()
        .unwrap();
    for function in &graph.functions {
        compute(&graph.table, function);
    }
}
fn execute_f4(lifecycle: NamingDocumentCase) {
    let graph = initial_f4(lifecycle);
    new_command(&graph.lifecycle.document);
    let single = test_selection(&graph.lifecycle.document, &graph.object, false, false).unwrap();
    let multiple = test_selection(&graph.lifecycle.document, &graph.object, false, true).unwrap();
    assert_no_errors(&single);
    assert_no_errors(&multiple);
    println!(
        "{{\"event\":\"draw-f4-attempts\",\"single\":{},\"multiple\":{}}}",
        single.attempted, multiple.attempted
    );
    recompute(&graph);
    assert!(
        solve_flat_from(&graph.table, &single.first)
            .unwrap()
            .is_empty()
    );
    new_command(&graph.lifecycle.document);
    graph.lifecycle.close();
}

#[test]
fn caf_named_shape_f4_recomputes_every_single_and_multiple_selection() {
    execute_f4(NamingDocumentCase::new());
}

/// The tests/caf/driver/B4 body on the caf/begin document; the `caf_driver_cases`
/// suite runs it in its policy child.
pub(crate) fn caf_driver_b4_recomputes_case_graph_with_retained_begin_document(
    lifecycle: NamingDocumentCase,
) {
    execute_f4(lifecycle);
}
