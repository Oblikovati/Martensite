//! Shared exact F5/F6/F7 point rectangle, closed Line3D and real prism command prefix.
use super::naming_function_values as values;
use super::{
    naming_draw_document::NamingDocumentCase,
    naming_draw_driver_registration::register_case_drivers,
    naming_draw_modeling::{compute, function, reals},
    naming_draw_observations::{
        NamingShapeBindings, new_command, observe_reference, observe_shape,
    },
    naming_draw_primitives::prism_with_reverse,
    naming_function_graph::add_object,
    naming_transform_arguments::set_object_argument,
};
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::{
    tdata_std::TDataStdUAttribute,
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionDriverTable, TFunctionFunction},
};
pub(super) struct PointPrismCase {
    pub lifecycle: NamingDocumentCase,
    pub table: Handle<TFunctionDriverTable>,
    pub bindings: NamingShapeBindings,
    pub object: Handle<TDataStdUAttribute>,
    pub point_objects: [Handle<TDataStdUAttribute>; 4],
    pub functions: Vec<Handle<TFunctionFunction>>,
}
type NamedPoint = (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>);
fn first_point_reads(
    bindings: &mut NamingShapeBindings,
    object: &Handle<TDataStdUAttribute>,
    function: &Handle<TFunctionFunction>,
) {
    bindings.bind(function, "P1").unwrap();
    observe_shape(bindings, "P1", "shape VERTEX FORWARD");
    observe_reference(object, "0:1:1:1:2");
}
fn rectangle_points(
    document: &Handle<TDocStdDocument>,
    table: &Handle<TFunctionDriverTable>,
    bindings: &mut NamingShapeBindings,
) -> [NamedPoint; 4] {
    let coordinates = [
        [0.0, 0.0, 0.0],
        [0.0, 200.0, 0.0],
        [200.0, 200.0, 0.0],
        [200.0, 0.0, 0.0],
    ];
    let points = std::array::from_fn(|index| {
        let object = add_object(document).unwrap();
        let function = function(&object, "5c", "PntXYZ_Function");
        reals(&function, coordinates[index]);
        compute(table, &function);
        if index == 0 {
            first_point_reads(bindings, &object, &function);
        }
        (object, function)
    });
    for (index, name) in [(1, "P2"), (2, "P3"), (3, "P4")] {
        bindings.bind(&points[index].1, name).unwrap();
    }
    points
}
fn closed_line(
    document: &Handle<TDocStdDocument>,
    table: &Handle<TFunctionDriverTable>,
    points: &[Handle<TDataStdUAttribute>; 4],
) -> NamedPoint {
    new_command(document);
    let object = add_object(document).unwrap();
    let function = function(&object, "5e", "Line3D_Function");
    values::integer_argument(&function, 1)
        .unwrap()
        .set_value(1)
        .unwrap();
    for (index, point) in points.iter().enumerate() {
        set_object_argument(&function, index as i32 + 2, point).unwrap();
    }
    values::integer_argument(&function, 2)
        .unwrap()
        .set_value(4)
        .unwrap();
    compute(table, &function);
    (object, function)
}
fn initial_rectangle(
    lifecycle: NamingDocumentCase,
) -> (PointPrismCase, Handle<TDataStdUAttribute>) {
    let table = Handle::new(TFunctionDriverTable::new());
    let mut bindings = NamingShapeBindings::for_case(&lifecycle);
    new_command(&lifecycle.document);
    register_case_drivers(&table, &["5c", "5d", "5e", "54"]);
    let points = rectangle_points(&lifecycle.document, &table, &mut bindings);
    let point_objects = points.clone().map(|(object, _)| object);
    let mut functions = Vec::from(points.map(|(_, function)| function));
    let (line_object, line) = closed_line(&lifecycle.document, &table, &point_objects);
    bindings.bind(&line, "L1").unwrap();
    functions.push(line);
    let graph = PointPrismCase {
        lifecycle,
        table,
        bindings,
        object: Handle::default(),
        point_objects,
        functions,
    };
    (graph, line_object)
}
/// Preserve the original prism reverse and registration stage; `point_prism_case(case,1,&["50","43","56"])`.
pub(super) fn point_prism_case(
    lifecycle: NamingDocumentCase,
    reverse: i32,
    driver_stage: &[&str],
) -> PointPrismCase {
    let (mut graph, line_object) = initial_rectangle(lifecycle);
    new_command(&graph.lifecycle.document);
    register_case_drivers(&graph.table, driver_stage);
    let (object, prism) = prism_with_reverse(
        &graph.table,
        &graph.lifecycle.document,
        &line_object,
        (300.0, reverse),
    );
    graph.bindings.bind(&prism, "Pr").unwrap();
    observe_reference(&object, "0:1:6:1:2");
    graph.object = object;
    graph.functions.push(prism);
    graph
}
/// Original PntOffset leaves z unchanged; `offset_first_point(&graph,[20.0,25.0])`.
pub(super) fn offset_first_point(graph: &PointPrismCase, offsets: [f64; 2]) {
    for (index, value) in offsets.into_iter().enumerate() {
        values::real_argument(&graph.functions[0], index as i32 + 1)
            .unwrap()
            .set_value(value)
            .unwrap();
    }
    assert_eq!(
        values::real_argument(&graph.functions[0], 3)
            .unwrap()
            .get()
            .unwrap()
            .get(),
        0.0
    );
}
