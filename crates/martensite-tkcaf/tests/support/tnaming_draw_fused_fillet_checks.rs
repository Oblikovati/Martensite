//! Complete E6 native original modeling, staged registry and unfiltered naming traversal.
use super::{
    naming_box_driver::real_argument,
    naming_draw_document::NamingDocumentCase,
    naming_draw_driver_registration::register_case_drivers,
    naming_draw_modeling::{
        assert_no_errors, binary_operation, box_reads, compute, function, initialize_logbook,
        prepare_box, reals,
    },
    naming_draw_observations::{NamingShapeBindings, new_command},
    naming_draw_primitives::{cylinder, edge_attachment},
    naming_draw_selection::{solve_flat_from, test_selection},
    naming_transform_arguments::set_object_argument,
};
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::{
    tdata_std::TDataStdUAttribute,
    tfunction::{TFunctionDriverTable, TFunctionFunction},
};
struct FusedFilletCase {
    lifecycle: NamingDocumentCase,
    table: Handle<TFunctionDriverTable>,
    bindings: NamingShapeBindings,
    object: Handle<TDataStdUAttribute>,
    functions: Vec<Handle<TFunctionFunction>>,
}
fn first_box(
    lifecycle: &NamingDocumentCase,
    table: &Handle<TFunctionDriverTable>,
    bindings: &mut NamingShapeBindings,
) -> (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>) {
    new_command(&lifecycle.document);
    register_case_drivers(table, &["43"]);
    let tool = super::naming_function_graph::add_object(&lifecycle.document).unwrap();
    let first = function(&tool, "43", "Box_Function");
    reals(&first, [130.0, 140.0, 150.0]);
    compute(table, &first);
    box_reads(bindings, &tool, &first, "Box1", Some("0:1:1:1:2"));
    (tool, first)
}
fn initial_boxes(lifecycle: NamingDocumentCase) -> (FusedFilletCase, Handle<TDataStdUAttribute>) {
    let table = Handle::new(TFunctionDriverTable::new());
    let mut bindings = NamingShapeBindings::for_case(&lifecycle);
    let (tool, first) = first_box(&lifecycle, &table, &mut bindings);
    let (object, second) = prepare_box(&lifecycle.document, [190.0, 290.0, 390.0], true);
    register_case_drivers(&table, &["43", "54", "45"]);
    compute(&table, &second);
    box_reads(&mut bindings, &object, &second, "Box2", Some("0:1:2:1:2"));
    (
        FusedFilletCase {
            lifecycle,
            table,
            bindings,
            object,
            functions: vec![first, second],
        },
        tool,
    )
}
fn translated_fuse(graph: &mut FusedFilletCase, tool: &Handle<TDataStdUAttribute>) {
    new_command(&graph.lifecycle.document);
    register_case_drivers(&graph.table, &["56", "57", "58"]);
    let translation = function(tool, "56", "ParTranslation");
    reals(&translation, [110.0, 130.0, 270.0]);
    compute(&graph.table, &translation);
    graph.bindings.bind(&translation, "Box1").unwrap();
    graph.functions.push(translation);
    register_case_drivers(&graph.table, &["49"]);
    let fuse = binary_operation(&graph.table, &graph.object, tool, ("49", "Fuse"));
    graph.bindings.bind(&fuse, "Box2").unwrap();
    graph.functions.push(fuse);
}
fn fused_axis_cylinder(graph: &mut FusedFilletCase) -> Handle<TDataStdUAttribute> {
    new_command(&graph.lifecycle.document);
    graph.bindings.explode_face_edge("Box2", 1);
    let (axis, selection) = edge_attachment(
        &graph.lifecycle.document,
        &graph.object,
        &graph.functions[3],
        (1, 1),
        false,
    );
    graph.functions.push(selection);
    register_case_drivers(&graph.table, &["54", "45"]);
    let document = &graph.lifecycle.document;
    let (tool, cylinder) = cylinder(&graph.table, document, &axis, [30.0, 315.0]);
    graph.bindings.bind(&cylinder, "S").unwrap();
    graph.functions.push(cylinder);
    tool
}
fn cut_cylinder(graph: &mut FusedFilletCase) {
    let tool = fused_axis_cylinder(graph);
    new_command(&graph.lifecycle.document);
    register_case_drivers(&graph.table, &["48"]);
    let cut = binary_operation(&graph.table, &graph.object, &tool, ("48", "Cut"));
    graph.bindings.bind(&cut, "S").unwrap();
    graph.functions.push(cut);
}
fn finish_fillet(graph: &mut FusedFilletCase) {
    new_command(&graph.lifecycle.document);
    graph.bindings.explode_face_edge("S", 3);
    let (path, selection) = edge_attachment(
        &graph.lifecycle.document,
        &graph.object,
        &graph.functions[6],
        (3, 6),
        false,
    );
    graph.functions.push(selection);
    register_case_drivers(&graph.table, &["53"]);
    let fillet = function(&graph.object, "53", "Fillet");
    real_argument(&fillet, 1).unwrap().set_value(25.0).unwrap();
    set_object_argument(&fillet, 3, &path).unwrap();
    compute(&graph.table, &fillet);
    graph.bindings.bind(&fillet, "S").unwrap();
    graph.functions.push(fillet);
}
fn initial_e6(lifecycle: NamingDocumentCase) -> FusedFilletCase {
    let (mut graph, tool) = initial_boxes(lifecycle);
    translated_fuse(&mut graph, &tool);
    cut_cylinder(&mut graph);
    finish_fillet(&mut graph);
    graph
}
fn recompute(graph: &mut FusedFilletCase) {
    new_command(&graph.lifecycle.document);
    real_argument(&graph.functions[1], 1)
        .unwrap()
        .set_value(210.0)
        .unwrap();
    new_command(&graph.lifecycle.document);
    initialize_logbook(&graph.functions[1]);
    for index in [1, 0, 2, 3, 4, 5, 6, 7, 8] {
        compute(&graph.table, &graph.functions[index]);
    }
    graph.bindings.bind(&graph.functions[8], "S").unwrap();
    graph.bindings.presentation().only("S");
}
fn execute_e6(lifecycle: NamingDocumentCase) {
    let mut graph = initial_e6(lifecycle);
    new_command(&graph.lifecycle.document);
    let single = test_selection(&graph.lifecycle.document, &graph.object, false, false).unwrap();
    let multiple = test_selection(&graph.lifecycle.document, &graph.object, false, true).unwrap();
    assert_no_errors(&single);
    assert_no_errors(&multiple);
    println!(
        "{{\"event\":\"draw-e6-attempts\",\"single\":{},\"multiple\":{}}}",
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
fn caf_named_shape_e6_recomputes_every_single_and_multiple_selection() {
    execute_e6(NamingDocumentCase::new());
}

/// The tests/caf/driver/A6 body on the caf/begin document; the `caf_driver_cases`
/// suite runs it in its policy child.
pub(crate) fn caf_driver_a6_recomputes_case_graph_with_retained_begin_document(
    lifecycle: NamingDocumentCase,
) {
    execute_e6(lifecycle);
}
