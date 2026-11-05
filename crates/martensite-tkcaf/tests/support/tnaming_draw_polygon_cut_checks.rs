//! Complete native F6/F7 profile intersection/cylinder cut and exact selection contexts.
use super::naming_draw_document::NamingDocumentCase;
use super::{
    naming_draw_driver_registration::register_case_drivers,
    naming_draw_modeling::{
        assert_no_errors, binary_operation, compute, function, prepare_box, reals,
    },
    naming_draw_observations::new_command,
    naming_draw_point_prism::{PointPrismCase, offset_first_point, point_prism_case},
    naming_draw_primitives::{attach_shape, cylinder},
    naming_draw_selection::{solve_flat_from, test_selection},
    support::shape_at,
    transform_checks::current,
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::{tdata_std::TDataStdUAttribute, tfunction::TFunctionFunction};
fn translated_box(graph: &mut PointPrismCase) -> Handle<TDataStdUAttribute> {
    let (object, box_function) =
        prepare_box(&graph.lifecycle.document, [400.0, 140.0, 600.0], false);
    compute(&graph.table, &box_function);
    graph.bindings.bind(&box_function, "Box").unwrap();
    graph.functions.push(box_function);
    new_command(&graph.lifecycle.document);
    let translation = function(&object, "56", "ParTranslation");
    reals(&translation, [-100.0, -20.0, -100.0]);
    compute(&graph.table, &translation);
    graph.bindings.bind(&translation, "Box").unwrap();
    graph.functions.push(translation);
    object
}
fn common_box(graph: &mut PointPrismCase, tool: &Handle<TDataStdUAttribute>) {
    new_command(&graph.lifecycle.document);
    register_case_drivers(&graph.table, &["4a", "48", "49"]);
    let common = binary_operation(&graph.table, &graph.object, tool, ("4a", "Common"));
    graph.bindings.bind(&common, "Pr").unwrap();
    graph.functions.push(common);
}
fn attached_cylinder(graph: &mut PointPrismCase) -> Handle<TDataStdUAttribute> {
    new_command(&graph.lifecycle.document);
    graph.bindings.explode("Pr", ShapeEnum::Edge);
    register_case_drivers(&graph.table, &["54", "45"]);
    let edge = shape_at(&current(&graph.functions[8]), ShapeEnum::Edge, 6);
    let (axis, selection) = attach_shape(&graph.lifecycle.document, &graph.object, &edge, false);
    graph.functions.push(selection);
    let (object, cylinder) = cylinder(
        &graph.table,
        &graph.lifecycle.document,
        &axis,
        [70.0, 515.0],
    );
    graph.bindings.bind(&cylinder, "CylS").unwrap();
    graph.functions.push(cylinder);
    object
}
fn translated_cut(
    graph: &mut PointPrismCase,
    cylinder: &Handle<TDataStdUAttribute>,
    separate_cut: bool,
) {
    new_command(&graph.lifecycle.document);
    let translation = function(cylinder, "56", "ParTranslation");
    reals(&translation, [-100.0, 200.0, 170.0]);
    compute(&graph.table, &translation);
    graph.bindings.bind(&translation, "CylS").unwrap();
    graph.functions.push(translation);
    if separate_cut {
        new_command(&graph.lifecycle.document);
    }
    let cut = binary_operation(&graph.table, &graph.object, cylinder, ("48", "Cut"));
    graph.bindings.bind(&cut, "Pr").unwrap();
    graph.bindings.presentation().only("Pr");
    graph.functions.push(cut);
}
fn initial_case(lifecycle: NamingDocumentCase, separate_cut: bool) -> PointPrismCase {
    let mut graph = point_prism_case(lifecycle, 1, &["50", "43", "56"]);
    let box_object = translated_box(&mut graph);
    common_box(&mut graph, &box_object);
    let cylinder = attached_cylinder(&mut graph);
    translated_cut(&mut graph, &cylinder, separate_cut);
    graph
}
fn recompute(graph: &mut PointPrismCase, offsets: [f64; 2], new_commands: bool) {
    if new_commands {
        new_command(&graph.lifecycle.document);
    }
    offset_first_point(graph, offsets);
    if new_commands {
        new_command(&graph.lifecycle.document);
    }
    for index in [0, 4, 5, 8, 9, 10, 11, 12] {
        compute(&graph.table, &graph.functions[index]);
    }
    graph
        .bindings
        .bind_after_erase(&graph.functions[12], "Pr")
        .unwrap();
}
fn execute_f6(lifecycle: NamingDocumentCase, offsets: [f64; 2], require_solve: bool) {
    let mut graph = initial_case(lifecycle, false);
    new_command(&graph.lifecycle.document);
    let single = test_selection(&graph.lifecycle.document, &graph.object, false, false).unwrap();
    let multiple = test_selection(&graph.lifecycle.document, &graph.object, false, true).unwrap();
    assert_no_errors(&single);
    assert_no_errors(&multiple);
    println!(
        "{{\"event\":\"draw-f6-attempts\",\"single\":{},\"multiple\":{}}}",
        single.attempted, multiple.attempted
    );
    recompute(&mut graph, offsets, true);
    let solved = solve_flat_from(&graph.table, &single.first);
    if require_solve {
        assert!(solved.unwrap().is_empty());
    }
    new_command(&graph.lifecycle.document);
    graph.lifecycle.close();
}
fn case_edge_display(bindings: &mut super::naming_draw_observations::NamingShapeBindings) {
    bindings.explode("Pr", ShapeEnum::Edge);
    let names: Vec<String> = (1..=17).map(|index| format!("Pr_{index}")).collect();
    let borrowed: Vec<&str> = names.iter().map(String::as_str).collect();
    bindings.presentation().erase(&borrowed);
    bindings.presentation().display(&["Pr_8", "Pr_9"]);
}

fn four_selections(graph: &mut PointPrismCase) -> [Handle<TFunctionFunction>; 4] {
    case_edge_display(&mut graph.bindings);
    let cut = current(&graph.functions[12]);
    new_command(&graph.lifecycle.document);
    [(8, false), (9, true), (11, true), (11, false)].map(|(index, keep)| {
        let edge = shape_at(&cut, ShapeEnum::Edge, index);
        let (_, selection) = attach_shape(&graph.lifecycle.document, &graph.object, &edge, keep);
        selection
    })
}
// At native, KeepOrientation0 changes across identical processes; the source does not test it.
fn assert_observed_selection_metadata(
    graph: &mut PointPrismCase,
    selections: &[Handle<TFunctionFunction>; 4],
) {
    for (selection, (name, keep)) in selections.iter().zip([
        ("Sel8", false),
        ("Sel9", true),
        ("Sel11", true),
        ("Sel12", false),
    ]) {
        graph.bindings.bind(selection, name).unwrap();
        let actual = graph.bindings.whatis(name).unwrap();
        if keep {
            assert_eq!(actual, "shape EDGE REVERSED Modified Orientable");
            continue;
        }
        assert!(matches!(
            actual.as_str(),
            "shape EDGE FORWARD Modified Orientable" | "shape EDGE REVERSED Modified Orientable"
        ));
    }
}
fn execute_f7(lifecycle: NamingDocumentCase) -> (PointPrismCase, [Handle<TFunctionFunction>; 4]) {
    let mut graph = initial_case(lifecycle, true);
    let selections = four_selections(&mut graph);
    recompute(&mut graph, [40.0, 25.0], false);
    for selection in &selections {
        compute(&graph.table, selection);
    }
    new_command(&graph.lifecycle.document);
    (graph, selections)
}
#[test]
fn caf_named_shape_f7_recomputes_four_case_edge_selections() {
    let (graph, _selections) = execute_f7(NamingDocumentCase::new());
    graph.lifecycle.close();
}
#[test]
fn f7_additional_observation_matches_current_edge_types_flags_and_orientation_policy() {
    let (mut graph, selections) = execute_f7(NamingDocumentCase::new());
    assert_observed_selection_metadata(&mut graph, &selections);
    graph.lifecycle.close();
}

#[test]
fn caf_named_shape_f6_recomputes_every_single_and_multiple_selection() {
    execute_f6(NamingDocumentCase::new(), [20.0, 25.0], true);
}

/// The tests/caf/driver/B6 body on the caf/begin document; the `caf_driver_cases`
/// suite runs it in its policy child.
pub(crate) fn caf_driver_b6_retains_case_offsets_and_caught_solve_outcome(
    lifecycle: NamingDocumentCase,
) {
    // Unlike F6, the original B6 uses offsets25,15 and deliberately catches SolveFlatFrom.
    execute_f6(lifecycle, [25.0, 15.0], false);
}
/// The tests/caf/driver/B7 body on the caf/begin document; the `caf_driver_cases`
/// suite runs it in its policy child.
pub(crate) fn caf_driver_b7_recomputes_four_case_edge_selections_with_retained_begin_document(
    lifecycle: NamingDocumentCase,
) {
    let (graph, _selections) = execute_f7(lifecycle);
    graph.lifecycle.close();
}

fn unique_vertex_count(shape: &martensite_tkbrep::topods::Shape) -> usize {
    let mut vertices = martensite_tkbrep::toptools::MapOfShape::default();
    for vertex in super::support::explore(shape, ShapeEnum::Vertex) {
        vertices.add(vertex);
    }
    vertices.extent()
}
/// Retain the document through the original checked result and its rendered capture.
/// This fixture is thread-confined because its operation-local document graph uses shared mutable labels.
pub struct Bug21707Scene {
    graph: PointPrismCase,
    result: martensite_tkbrep::topods::Shape,
}
impl Bug21707Scene {
    /// The actual named cut result; `let shape = scene.result();`.
    pub fn result(&self) -> martensite_tkbrep::topods::Shape {
        self.result.clone()
    }
    /// Close only after every original observation; `scene.close();`.
    pub fn close(self) {
        self.graph.lifecycle.close();
    }
    /// Consume the same original scene for result-only capture; `scene.case_view()`.
    pub fn case_view(&self) {
        let presentation = self.graph.bindings.presentation();
        presentation.result_view("bug21707");
    }
}
/// Build and check the actual function graph; `let scene = bug21707_checked_scene();`.
pub fn bug21707_checked_scene() -> Bug21707Scene {
    let lifecycle = NamingDocumentCase::standalone();
    checked_bug_scene(lifecycle)
}

/// Retain actual source UI state through the original frame; `bug21707_presented_scene()`.
pub fn bug21707_presented_scene() -> Bug21707Scene {
    let lifecycle = NamingDocumentCase::standalone().with_presentation();
    lifecycle.presentation.isos(12);
    checked_bug_scene(lifecycle)
}

fn checked_bug_scene(lifecycle: NamingDocumentCase) -> Bug21707Scene {
    let mut graph = initial_case(lifecycle, false);
    graph.bindings.explode("Pr", ShapeEnum::Vertex);
    graph.bindings.presentation().only("Pr");
    let result = current(&graph.functions[12]);
    for index in [8, 10] {
        let vertex = shape_at(&result, ShapeEnum::Vertex, index);
        let (_object, _selection) =
            attach_shape(&graph.lifecycle.document, &graph.object, &vertex, false);
    }
    graph.bindings.bind(&graph.functions[12], "result").unwrap();
    assert_eq!(unique_vertex_count(&result), 10);
    Bug21707Scene { graph, result }
}
#[test]
fn bug21707_case_kernel_graph_keeps_ten_vertices_and_both_vertex_attachments() {
    bug21707_checked_scene().close();
}
