//! Complete native F1 original attached sphere, degree rotation, fusion and naming pipeline.
use super::{
    naming_box_driver::real_argument,
    naming_draw_document::NamingDocumentCase,
    naming_draw_driver_registration::case_driver_table,
    naming_draw_modeling::{
        assert_no_errors, binary_operation, compute, function, initialize_logbook, reals,
        standard_box,
    },
    naming_draw_observations::{NamingShapeBindings, new_command},
    naming_draw_primitives::{attach_shape, cylinder, first_face_edge_attachment},
    naming_draw_selection::{solve_flat_from, test_selection},
    naming_function_graph::add_object,
    naming_transform_arguments::set_object_argument,
    support::shape_at,
    transform_checks::current,
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::{
    tdata_std::TDataStdUAttribute,
    tfunction::{TFunctionDriverTable, TFunctionFunction},
};
struct SphereFuseCase {
    lifecycle: NamingDocumentCase,
    table: Handle<TFunctionDriverTable>,
    bindings: NamingShapeBindings,
    object: Handle<TDataStdUAttribute>,
    functions: Vec<Handle<TFunctionFunction>>,
}
fn initial_box(lifecycle: NamingDocumentCase) -> SphereFuseCase {
    let table = case_driver_table();
    let mut bindings = NamingShapeBindings::for_case(&lifecycle);
    let (object, source) = standard_box(&table, &lifecycle.document, &mut bindings, false);
    SphereFuseCase {
        lifecycle,
        table,
        bindings,
        object,
        functions: vec![source],
    }
}
fn attached_cylinder(graph: &mut SphereFuseCase) -> Handle<TDataStdUAttribute> {
    new_command(&graph.lifecycle.document);
    graph.bindings.explode_face_edge("Box2", 1);
    let (axis, selection) = first_face_edge_attachment(
        &graph.lifecycle.document,
        &graph.object,
        &graph.functions[0],
    );
    let (object, cylinder) = cylinder(
        &graph.table,
        &graph.lifecycle.document,
        &axis,
        [50.0, 515.0],
    );
    graph.bindings.bind(&cylinder, "CylS").unwrap();
    graph.functions.extend([selection, cylinder]);
    object
}
fn translated_cut(graph: &mut SphereFuseCase, tool: &Handle<TDataStdUAttribute>) {
    new_command(&graph.lifecycle.document);
    let translation = function(tool, "56", "ParTranslation");
    reals(&translation, [100.0, 0.0, 0.0]);
    compute(&graph.table, &translation);
    graph.bindings.bind(&translation, "CylS").unwrap();
    graph.functions.push(translation);
    new_command(&graph.lifecycle.document);
    let cut = binary_operation(&graph.table, &graph.object, tool, ("48", "Cut"));
    graph.bindings.bind_after_erase(&cut, "S").unwrap();
    graph.functions.push(cut);
    graph.bindings.presentation().only("S");
    new_command(&graph.lifecycle.document);
}
fn centers(graph: &mut SphereFuseCase) -> [Handle<TDataStdUAttribute>; 4] {
    graph.bindings.explode("S", ShapeEnum::Vertex);
    let cut = current(&graph.functions[4]);
    [1, 2, 9, 11].map(|index| {
        let vertex = shape_at(&cut, ShapeEnum::Vertex, index);
        let (object, selection) =
            attach_shape(&graph.lifecycle.document, &graph.object, &vertex, false);
        graph.functions.push(selection);
        object
    })
}
fn sphere(
    graph: &mut SphereFuseCase,
    center: &Handle<TDataStdUAttribute>,
    name: &'static str,
) -> Handle<TDataStdUAttribute> {
    let object = add_object(&graph.lifecycle.document).unwrap();
    let sphere = function(&object, "44", "Sphere_Function");
    set_object_argument(&sphere, 1, center).unwrap();
    real_argument(&sphere, 2).unwrap().set_value(15.0).unwrap();
    compute(&graph.table, &sphere);
    graph.bindings.bind(&sphere, name).unwrap();
    graph.functions.push(sphere);
    object
}
fn spheres(graph: &mut SphereFuseCase) -> [Handle<TDataStdUAttribute>; 4] {
    let centers = centers(graph);
    std::array::from_fn(|index| {
        sphere(graph, &centers[index], ["Sp1", "Sp2", "Sp9", "Sp11"][index])
    })
}
fn rotate_sphere(
    graph: &mut SphereFuseCase,
    object: &Handle<TDataStdUAttribute>,
    name: &'static str,
) {
    let edge = shape_at(&current(&graph.functions[0]), ShapeEnum::Edge, 1);
    let (axis, selection) = attach_shape(&graph.lifecycle.document, &graph.object, &edge, false);
    graph.functions.push(selection);
    let rotation = function(object, "58", "ParRotationAroundLine");
    set_object_argument(&rotation, 2, &axis).unwrap();
    real_argument(&rotation, 1)
        .unwrap()
        .set_value(120.0 * (2.0 * std::f64::consts::PI / 360.0))
        .unwrap();
    compute(&graph.table, &rotation);
    graph.bindings.bind(&rotation, name).unwrap();
    graph.functions.push(rotation);
}
fn initial_f1(lifecycle: NamingDocumentCase) -> SphereFuseCase {
    let mut graph = initial_box(lifecycle);
    let cylinder = attached_cylinder(&mut graph);
    translated_cut(&mut graph, &cylinder);
    let spheres = spheres(&mut graph);
    graph.bindings.explode("Box2", ShapeEnum::Edge);
    rotate_sphere(&mut graph, &spheres[0], "Sp1");
    rotate_sphere(&mut graph, &spheres[1], "Sp2");
    for sphere in spheres {
        let fuse = binary_operation(&graph.table, &graph.object, &sphere, ("49", "Fuse"));
        graph.bindings.bind(&fuse, "S").unwrap();
        graph.functions.push(fuse);
    }
    graph
}
fn recompute(graph: &mut SphereFuseCase) {
    new_command(&graph.lifecycle.document);
    real_argument(&graph.functions[0], 1)
        .unwrap()
        .set_value(250.0)
        .unwrap();
    real_argument(&graph.functions[0], 3)
        .unwrap()
        .set_value(310.0)
        .unwrap();
    new_command(&graph.lifecycle.document);
    initialize_logbook(&graph.functions[0]);
    for index in [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 13, 14, 15, 16, 11, 12, 17, 18, 19, 20,
    ] {
        compute(&graph.table, &graph.functions[index]);
    }
    let result = &graph.functions[20];
    graph.bindings.bind_after_erase(result, "S").unwrap();
}
fn execute_f1(lifecycle: NamingDocumentCase) {
    let mut graph = initial_f1(lifecycle);
    let single = test_selection(&graph.lifecycle.document, &graph.object, false, false).unwrap();
    let multiple = test_selection(&graph.lifecycle.document, &graph.object, false, true).unwrap();
    assert_no_errors(&single);
    assert_no_errors(&multiple);
    println!(
        "{{\"event\":\"draw-f1-attempts\",\"single\":{},\"multiple\":{}}}",
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
fn caf_named_shape_f1_recomputes_every_single_and_multiple_selection() {
    execute_f1(NamingDocumentCase::new());
}

/// The tests/caf/driver/B1 body on the caf/begin document; the `caf_driver_cases`
/// suite runs it in its policy child.
pub(crate) fn caf_driver_b1_recomputes_case_graph_with_retained_begin_document(
    lifecycle: NamingDocumentCase,
) {
    execute_f1(lifecycle);
}
