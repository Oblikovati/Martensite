//! Shared native attachment and cylinder construction, preserving command-host explorer order.
use super::naming_function_values as values;
use super::{
    naming_box_driver::real_argument,
    naming_draw_modeling::{compute, function},
    naming_draw_selection::setup,
    naming_function_graph::add_object,
    naming_transform_arguments::set_object_argument,
    support::shape_at,
    transform_checks::current,
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::{
    tdata_std::TDataStdUAttribute,
    tdf::TdfAttributeState,
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionDriverTable, TFunctionFunction},
};
pub(super) fn first_face_edge_attachment(
    document: &Handle<TDocStdDocument>,
    object: &Handle<TDataStdUAttribute>,
    source: &Handle<TFunctionFunction>,
) -> (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>) {
    edge_attachment(document, object, source, (1, 1), true)
}
pub(super) fn edge_attachment(
    document: &Handle<TDocStdDocument>,
    object: &Handle<TDataStdUAttribute>,
    source: &Handle<TFunctionFunction>,
    indices: (usize, usize),
    keep: bool,
) -> (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>) {
    let face = shape_at(&current(source), ShapeEnum::Face, indices.0);
    let edge = shape_at(&face, ShapeEnum::Edge, indices.1);
    attach_shape(document, object, &edge, keep)
}
pub(super) fn face_attachment(
    document: &Handle<TDocStdDocument>,
    object: &Handle<TDataStdUAttribute>,
    source: &Handle<TFunctionFunction>,
    index: usize,
) -> (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>) {
    let face = shape_at(&current(source), ShapeEnum::Face, index);
    attach_shape(document, object, &face, false)
}
pub(super) fn attach_shape(
    document: &Handle<TDocStdDocument>,
    object: &Handle<TDataStdUAttribute>,
    shape: &martensite_tkbrep::topods::Shape,
    keep: bool,
) -> (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>) {
    let attachment = setup::make_selection(document, object, shape, keep).unwrap();
    assert!(attachment.selected);
    assert!(attachment.exception.is_none());
    let function = values::function_on(&attachment.object.label().find_child(1, false).unwrap())
        .unwrap()
        .unwrap();
    (attachment.object, function)
}
pub(super) fn cylinder(
    table: &Handle<TFunctionDriverTable>,
    document: &Handle<TDocStdDocument>,
    axis: &Handle<TDataStdUAttribute>,
    sizes: [f64; 2],
) -> (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>) {
    let object = add_object(document).unwrap();
    let cylinder = function(&object, "45", "Cyl_Function");
    real_argument(&cylinder, 1)
        .unwrap()
        .set_value(sizes[0])
        .unwrap();
    real_argument(&cylinder, 2)
        .unwrap()
        .set_value(sizes[1])
        .unwrap();
    set_object_argument(&cylinder, 3, axis).unwrap();
    compute(table, &cylinder);
    (object, cylinder)
}

pub(super) fn prism(
    table: &Handle<TFunctionDriverTable>,
    document: &Handle<TDocStdDocument>,
    basis: &Handle<TDataStdUAttribute>,
    height: f64,
) -> (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>) {
    prism_with_reverse(table, document, basis, (height, 0))
}
pub(super) fn prism_with_reverse(
    table: &Handle<TFunctionDriverTable>,
    document: &Handle<TDocStdDocument>,
    basis: &Handle<TDataStdUAttribute>,
    options: (f64, i32),
) -> (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>) {
    let object = add_object(document).unwrap();
    let function = function(&object, "50", "Prism_Function");
    set_object_argument(&function, 1, basis).unwrap();
    real_argument(&function, 2)
        .unwrap()
        .set_value(options.0)
        .unwrap();
    values::integer_argument(&function, 3)
        .unwrap()
        .set_value(options.1)
        .unwrap();
    compute(table, &function);
    (object, function)
}
