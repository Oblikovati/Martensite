//! Complete native E7/E8 original analytic sweep and ordered naming pipelines.
use super::naming_function_values as values;
use super::{
    naming_box_driver::real_argument,
    naming_draw_document::NamingDocumentCase,
    naming_draw_driver_registration::{register_case_drivers, sweep_driver_table},
    naming_draw_modeling::{
        ModeledObject, assert_no_errors, binary_operation, compute, function, initialize_logbook,
        reals, standard_box,
    },
    naming_draw_observations::{NamingShapeBindings, new_command},
    naming_draw_primitives::{edge_attachment, face_attachment, prism},
    naming_draw_selection::{solve_flat_from, test_selection},
    naming_function_graph::add_object,
    naming_transform_arguments::set_object_argument,
};
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::{
    tdata_std::TDataStdUAttribute,
    tfunction::{TFunctionDriverTable, TFunctionFunction},
};
struct RevolutionCase {
    lifecycle: NamingDocumentCase,
    table: Handle<TFunctionDriverTable>,
    bindings: NamingShapeBindings,
    object: Handle<TDataStdUAttribute>,
    functions: Vec<Handle<TFunctionFunction>>,
}
fn sweep_case(lifecycle: NamingDocumentCase) -> RevolutionCase {
    let table = sweep_driver_table();
    let mut bindings = NamingShapeBindings::for_case(&lifecycle);
    let (object, source) = standard_box(&table, &lifecycle.document, &mut bindings, true);
    RevolutionCase {
        lifecycle,
        table,
        bindings,
        object,
        functions: vec![source],
    }
}
fn revol_arguments(
    function: &Handle<TFunctionFunction>,
    basis: &Handle<TDataStdUAttribute>,
    axis: &Handle<TDataStdUAttribute>,
    full: bool,
) {
    set_object_argument(function, 1, basis).unwrap();
    set_object_argument(function, 2, axis).unwrap();
    if !full {
        real_argument(function, 3)
            .unwrap()
            .set_value(120.0 * (2.0 * std::f64::consts::PI / 360.0))
            .unwrap();
        values::integer_argument(function, 4)
            .unwrap()
            .set_value(0)
            .unwrap();
    }
}
fn revol_function(
    graph: &RevolutionCase,
    basis: &Handle<TDataStdUAttribute>,
    axis: &Handle<TDataStdUAttribute>,
    full: bool,
) -> (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>) {
    let object = add_object(&graph.lifecycle.document).unwrap();
    let kind = if full {
        ("51", "FulRevol_Function")
    } else {
        ("52", "SecRevol_Function")
    };
    let function = function(&object, kind.0, kind.1);
    revol_arguments(&function, basis, axis, full);
    compute(&graph.table, &function);
    (object, function)
}
fn revolution_attachments(
    graph: &mut RevolutionCase,
    source: &Handle<TDataStdUAttribute>,
    source_function: &Handle<TFunctionFunction>,
    indices: (usize, usize),
) -> (Handle<TDataStdUAttribute>, Handle<TDataStdUAttribute>) {
    let document = &graph.lifecycle.document;
    let (basis, basis_selection) = face_attachment(document, source, source_function, indices.0);
    let (axis, axis_selection) = edge_attachment(document, source, source_function, indices, false);
    graph.functions.extend([basis_selection, axis_selection]);
    (basis, axis)
}
fn attached_revolution(
    graph: &mut RevolutionCase,
    source: &Handle<TDataStdUAttribute>,
    source_function: &Handle<TFunctionFunction>,
    indices: (usize, usize),
    full: bool,
    name: &'static str,
    source_name: &str,
) -> (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>) {
    graph.bindings.explode_face_edge(source_name, indices.0);
    let (basis, axis) = revolution_attachments(graph, source, source_function, indices);
    let (object, function) = revol_function(graph, &basis, &axis, full);
    graph.bindings.bind(&function, name).unwrap();
    graph.functions.push(function.clone());
    (object, function)
}
fn first_prism(graph: &mut RevolutionCase) -> Handle<TDataStdUAttribute> {
    graph
        .bindings
        .explode("Box2", martensite_tkg3d::top_abs::ShapeEnum::Face);
    let document = &graph.lifecycle.document;
    let (basis, selection) = face_attachment(document, &graph.object, &graph.functions[0], 1);
    let (object, function) = prism(&graph.table, &graph.lifecycle.document, &basis, 300.0);
    graph.bindings.bind(&function, "Pr").unwrap();
    real_argument(&function, 2)
        .unwrap()
        .set_value(500.0)
        .unwrap();
    compute(&graph.table, &function);
    graph.bindings.bind(&function, "Pr").unwrap();
    graph.bindings.presentation().fit();
    graph.functions.extend([selection, function]);
    object
}
fn translate_full_revolution(graph: &mut RevolutionCase, object: &Handle<TDataStdUAttribute>) {
    new_command(&graph.lifecycle.document);
    let translation = function(object, "56", "ParTranslation");
    reals(&translation, [-250.0, 0.0, 0.0]);
    compute(&graph.table, &translation);
    graph.bindings.bind(&translation, "R1").unwrap();
    graph.functions.push(translation);
}
fn fuse(
    graph: &mut RevolutionCase,
    object: &Handle<TDataStdUAttribute>,
    tool: &Handle<TDataStdUAttribute>,
    name: &'static str,
) {
    let fuse = binary_operation(&graph.table, object, tool, ("49", "Fuse"));
    graph.bindings.bind(&fuse, name).unwrap();
    graph.functions.push(fuse);
}
fn initial_e7(lifecycle: NamingDocumentCase) -> RevolutionCase {
    let mut graph = sweep_case(lifecycle);
    let prism = first_prism(&mut graph);
    let source = graph.object.clone();
    let source_function = graph.functions[0].clone();
    let (full, _) = attached_revolution(
        &mut graph,
        &source,
        &source_function,
        (3, 3),
        true,
        "R1",
        "Box2",
    );
    translate_full_revolution(&mut graph, &full);
    fuse_full_e7(&mut graph, &prism, &full, &source_function);
    finish_e7(&mut graph, &source, &source_function, &prism);
    graph
}
fn fuse_full_e7(
    graph: &mut RevolutionCase,
    prism: &Handle<TDataStdUAttribute>,
    full: &Handle<TDataStdUAttribute>,
    source_function: &Handle<TFunctionFunction>,
) {
    register_case_drivers(&graph.table, &["49"]);
    fuse(graph, prism, full, "Pr");
    graph.bindings.bind(source_function, "Box2").unwrap();
}
fn finish_e7(
    graph: &mut RevolutionCase,
    source: &Handle<TDataStdUAttribute>,
    source_function: &Handle<TFunctionFunction>,
    prism: &Handle<TDataStdUAttribute>,
) {
    let (section, _) =
        attached_revolution(graph, source, source_function, (2, 3), false, "R2", "Box2");
    fuse(graph, source, &section, "Box2");
    fuse(graph, source, prism, "Box2");
}
fn first_e8_revolution(
    graph: &mut RevolutionCase,
) -> (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>) {
    let source = graph.object.clone();
    let source_function = graph.functions[0].clone();
    let (first, first_function) = attached_revolution(
        graph,
        &source,
        &source_function,
        (3, 2),
        false,
        "R1",
        "Box2",
    );
    graph.bindings.presentation().only("R1");
    (first, first_function)
}
fn next_e8_revolution(
    graph: &mut RevolutionCase,
    source: &ModeledObject,
    name: &str,
) -> ModeledObject {
    let (object, function) =
        attached_revolution(graph, &source.0, &source.1, (5, 2), false, "R2", name);
    (object, function)
}
fn initial_e8(lifecycle: NamingDocumentCase) -> RevolutionCase {
    let mut graph = sweep_case(lifecycle);
    let first = first_e8_revolution(&mut graph);
    let second = next_e8_revolution(&mut graph, &first, "R1");
    let third = next_e8_revolution(&mut graph, &second, "R2");
    register_case_drivers(&graph.table, &["49"]);
    fuse(&mut graph, &first.0, &second.0, "R1");
    fuse(&mut graph, &first.0, &third.0, "R1");
    graph
}
fn recompute(graph: &mut RevolutionCase, name: &'static str) {
    new_command(&graph.lifecycle.document);
    real_argument(&graph.functions[0], 3)
        .unwrap()
        .set_value(420.0)
        .unwrap();
    new_command(&graph.lifecycle.document);
    initialize_logbook(&graph.functions[0]);
    for function in &graph.functions {
        compute(&graph.table, function);
    }
    graph
        .bindings
        .bind(graph.functions.last().unwrap(), name)
        .unwrap();
}
fn execute_case(mut graph: RevolutionCase, name: &'static str) {
    new_command(&graph.lifecycle.document);
    let single = test_selection(&graph.lifecycle.document, &graph.object, false, false).unwrap();
    let multiple = test_selection(&graph.lifecycle.document, &graph.object, false, true).unwrap();
    assert_no_errors(&single);
    assert_no_errors(&multiple);
    println!(
        "{{\"event\":\"draw-revolution-chain-attempts\",\"result\":\"{name}\",\"single\":{},\"multiple\":{}}}",
        single.attempted, multiple.attempted
    );
    recompute(&mut graph, name);
    assert!(
        solve_flat_from(&graph.table, &single.first)
            .unwrap()
            .is_empty()
    );
    new_command(&graph.lifecycle.document);
    graph.lifecycle.close();
}
#[test]
fn caf_named_shape_e7_recomputes_every_single_and_multiple_selection() {
    execute_case(initial_e7(NamingDocumentCase::new()), "Box2");
}
#[test]
fn caf_named_shape_e8_recomputes_every_case_box_selection_after_revolutions() {
    execute_case(initial_e8(NamingDocumentCase::new()), "R1");
}

/// The tests/caf/driver/A7 body on the caf/begin document; the `caf_driver_cases`
/// suite runs it in its policy child.
pub(crate) fn caf_driver_a7_recomputes_case_graph_with_retained_begin_document(
    lifecycle: NamingDocumentCase,
) {
    execute_case(initial_e7(lifecycle), "Box2");
}

/// The tests/caf/driver/A8 body on the caf/begin document; the `caf_driver_cases`
/// suite runs it in its policy child.
pub(crate) fn caf_driver_a8_recomputes_case_graph_with_retained_begin_document(
    lifecycle: NamingDocumentCase,
) {
    execute_case(initial_e8(lifecycle), "R1");
}
