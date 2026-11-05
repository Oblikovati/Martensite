//! Complete native F5 point-profile, multiple-prism, sphere cut and naming pipeline.
use super::naming_draw_document::NamingDocumentCase;
use super::{
    naming_box_driver::real_argument,
    naming_draw_driver_registration::register_case_drivers,
    naming_draw_modeling::{assert_no_errors, binary_operation, compute, function},
    naming_draw_observations::new_command,
    naming_draw_point_prism::{PointPrismCase, offset_first_point, point_prism_case},
    naming_draw_primitives::{attach_shape, prism},
    naming_draw_selection::{solve_flat_from, test_selection},
    naming_function_graph::add_object,
    naming_transform_arguments::set_object_argument,
    support::shape_at,
    transform_checks::current,
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdata_std::TDataStdUAttribute;
fn prism_bases(graph: &mut PointPrismCase) -> [Handle<TDataStdUAttribute>; 2] {
    new_command(&graph.lifecycle.document);
    graph.bindings.explode("Pr", ShapeEnum::Face);
    let source = current(&graph.functions[5]);
    [1, 4].map(|index| {
        let face = shape_at(&source, ShapeEnum::Face, index);
        let (object, selection) =
            attach_shape(&graph.lifecycle.document, &graph.object, &face, true);
        graph.functions.push(selection);
        object
    })
}
fn additional_prisms(graph: &mut PointPrismCase) -> [Handle<TDataStdUAttribute>; 2] {
    let bases = prism_bases(graph);
    std::array::from_fn(|index| {
        let (object, function) = prism(
            &graph.table,
            &graph.lifecycle.document,
            &bases[index],
            [300.0, 500.0][index],
        );
        graph
            .bindings
            .bind(&function, ["Pr2", "Pr3"][index])
            .unwrap();
        graph.functions.push(function);
        object
    })
}
fn fused_prisms(graph: &mut PointPrismCase, tools: [Handle<TDataStdUAttribute>; 2]) {
    new_command(&graph.lifecycle.document);
    register_case_drivers(&graph.table, &["49", "44", "48"]);
    for tool in tools {
        let fuse = binary_operation(&graph.table, &graph.object, &tool, ("49", "Fuse"));
        graph.bindings.bind(&fuse, "Pr").unwrap();
        graph.functions.push(fuse);
    }
}
fn sphere_cut(graph: &mut PointPrismCase) {
    let sphere_object = add_object(&graph.lifecycle.document).unwrap();
    let sphere = function(&sphere_object, "44", "Sphere_Function");
    set_object_argument(&sphere, 1, &graph.point_objects[2]).unwrap();
    real_argument(&sphere, 2).unwrap().set_value(150.0).unwrap();
    new_command(&graph.lifecycle.document);
    compute(&graph.table, &sphere);
    graph.bindings.bind(&sphere, "Sp").unwrap();
    graph.functions.push(sphere);
    new_command(&graph.lifecycle.document);
    let cut = binary_operation(&graph.table, &graph.object, &sphere_object, ("48", "Cut"));
    graph.bindings.bind(&cut, "Pr").unwrap();
    graph.bindings.presentation().only("Pr");
    graph.functions.push(cut);
}
fn initial_f5(lifecycle: NamingDocumentCase) -> PointPrismCase {
    let mut graph = point_prism_case(lifecycle, 0, &["50"]);
    let tools = additional_prisms(&mut graph);
    fused_prisms(&mut graph, tools);
    sphere_cut(&mut graph);
    graph
}
fn recompute(graph: &mut PointPrismCase) {
    offset_first_point(graph, [50.0, 30.0]);
    new_command(&graph.lifecycle.document);
    for index in [0, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13] {
        compute(&graph.table, &graph.functions[index]);
    }
    graph
        .bindings
        .bind_after_erase(&graph.functions[13], "Pr")
        .unwrap();
}
fn execute_f5(lifecycle: NamingDocumentCase) {
    let mut graph = initial_f5(lifecycle);
    new_command(&graph.lifecycle.document);
    let single = test_selection(&graph.lifecycle.document, &graph.object, false, false).unwrap();
    let multiple = test_selection(&graph.lifecycle.document, &graph.object, false, true).unwrap();
    assert_no_errors(&single);
    assert_no_errors(&multiple);
    println!(
        "{{\"event\":\"draw-f5-attempts\",\"single\":{},\"multiple\":{}}}",
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
fn caf_named_shape_f5_recomputes_every_single_and_multiple_selection() {
    execute_f5(NamingDocumentCase::new());
}

/// The tests/caf/driver/B5 body on the caf/begin document; the `caf_driver_cases`
/// suite runs it in its policy child.
pub(crate) fn caf_driver_b5_recomputes_case_graph_with_retained_begin_document(
    lifecycle: NamingDocumentCase,
) {
    execute_f5(lifecycle);
}
