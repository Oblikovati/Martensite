//! Real E2 prefix and independent translation/naming history checks.
use super::{
    execute_box, modeled_box, named_shape,
    naming_box_driver::{function_result, result_label},
    naming_function_graph::{add_function, add_selection, solve_selection},
    naming_transform::{
        execute_transform, make_along_line, make_mirror, make_rotation, make_translation,
    },
    selection_driver, set_box_dimensions,
};
use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods::Shape};
use martensite_tkcaf::tnaming::{TNamingEvolution, TNamingIterator, TNamingTool};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::TDataStdUAttribute,
    tdf::TdfAttributeState,
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionFunction, TFunctionLogbook},
};

pub(super) fn current(function: &Handle<TFunctionFunction>) -> Shape {
    let named = function_result(function).unwrap();
    if named.is_null() {
        return Shape::default();
    }
    TNamingTool::get_shape(&named).unwrap()
}
#[test]
fn function_shape_without_named_result_is_null() {
    let document = super::document();
    let object = super::naming_function_graph::add_object(&document).unwrap();
    let function = add_function(
        &object,
        &guid_from_text("12e94543-6dbc-11d4-b9c8-0060b0ee281b"),
    )
    .unwrap();
    assert!(current(&function).is_null());
}
fn bounds(shape: &Shape) -> [f64; 6] {
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for vertex in Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape).unwrap() {
        let p = BRepTool::pnt(&martensite_tkbrep::topods::vertex(&vertex).unwrap()).unwrap();
        for (axis, coordinate) in [p.x(), p.y(), p.z()].into_iter().enumerate() {
            min[axis] = min[axis].min(coordinate);
            max[axis] = max[axis].max(coordinate);
        }
    }
    [min[0], min[1], min[2], max[0], max[1], max[2]]
}
fn assert_pair(function: &Handle<TFunctionFunction>, old: &Shape, new: &Shape) {
    let result = result_label(function).unwrap();
    let pair = TNamingIterator::from_label(&result).unwrap();
    assert_eq!(pair.evolution().unwrap(), TNamingEvolution::Modify);
    assert!(pair.old_shape().unwrap().is_same(old));
    assert!(pair.new_shape().unwrap().is_same(new));
}

#[test]
fn xyz_translation_preserves_box_tags_and_old_new_history_on_recompute() {
    let (_document, object, box_function) = modeled_box();
    let source = current(&box_function);
    let translated = make_translation(&object, [150.0, 40.0, 90.0]).unwrap();
    let image = current(&translated);
    assert_eq!(bounds(&image), [150.0, 40.0, 90.0, 340.0, 330.0, 480.0]);
    assert_eq!(bounds(&source), [0.0, 0.0, 0.0, 190.0, 290.0, 390.0]);
    assert_pair(&translated, &source, &image);
    assert_primitive_face_history(&box_function, &translated);
    set_box_dimensions(&box_function, 290.0);
    execute_box(&box_function);
    assert_eq!(execute_transform(&translated).unwrap(), 0);
    assert_eq!(
        bounds(&current(&translated)),
        [150.0, 40.0, 90.0, 440.0, 330.0, 480.0]
    );
    assert_primitive_face_history(&box_function, &translated);
}
fn assert_primitive_face_history(
    source: &Handle<TFunctionFunction>,
    image: &Handle<TFunctionFunction>,
) {
    let source_label = result_label(source).unwrap();
    let image_label = result_label(image).unwrap();
    let log = TFunctionLogbook::set(&image.label()).unwrap();
    for tag in 1..=6 {
        let old = named_shape(&source_label.find_child(tag, false).unwrap());
        let label = image_label.find_child(tag, false).unwrap();
        let iterator = TNamingIterator::from_label(&label).unwrap();
        assert!(iterator.old_shape().unwrap().is_same(&old));
        assert_eq!(
            iterator.new_shape().unwrap().shape_type().unwrap(),
            ShapeEnum::Face
        );
        assert!(log.get_valid().contains(&label));
    }
}
pub(super) fn selected_subshape(
    document: &Handle<TDocStdDocument>,
    object: &Handle<TDataStdUAttribute>,
    shape: &Shape,
    face: usize,
    edge: Option<usize>,
) -> (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>) {
    let selected_face = super::support::shape_at(shape, ShapeEnum::Face, face);
    let selected = edge.map_or(selected_face.clone(), |index| {
        super::support::shape_at(&selected_face, ShapeEnum::Edge, index)
    });
    assert!(!selected.is_null());
    add_selection(document, object, &selected, true).unwrap()
}

pub(super) struct E2TransformPrefix {
    pub document: Handle<TDocStdDocument>,
    pub object: Handle<TDataStdUAttribute>,
    pub box_function: Handle<TFunctionFunction>,
    pub transforms: [Handle<TFunctionFunction>; 4],
    pub selections: [Handle<TFunctionFunction>; 3],
}
pub(super) fn e2_transform_prefix() -> E2TransformPrefix {
    let (document, object, box_function) = modeled_box();
    let xyz = make_translation(&object, [150.0, 40.0, 90.0]).unwrap();
    let (line, line_selection) = selected_subshape(&document, &object, &current(&xyz), 1, Some(1));
    let along = make_along_line(&object, &line, 210.0).unwrap();
    let (rotation_line, rotation_selection) =
        selected_subshape(&document, &object, &current(&along), 1, Some(3));
    let rotated = make_rotation(&object, &rotation_line, 120.0).unwrap();
    let (plane, plane_selection) =
        selected_subshape(&document, &object, &current(&rotated), 3, None);
    let mirrored = make_mirror(&object, &plane).unwrap();
    E2TransformPrefix {
        document,
        object,
        box_function,
        transforms: [xyz, along, rotated, mirrored],
        selections: [line_selection, rotation_selection, plane_selection],
    }
}
#[test]
fn exact_e2_four_transform_prefix_recomputes_with_live_selected_axes_and_plane() {
    let prefix = e2_transform_prefix();
    recompute_prefix(
        &prefix.box_function,
        [
            &prefix.transforms[0],
            &prefix.transforms[1],
            &prefix.transforms[2],
            &prefix.transforms[3],
        ],
        [
            &prefix.selections[0],
            &prefix.selections[1],
            &prefix.selections[2],
        ],
    );
}

fn recompute_prefix(
    box_function: &Handle<TFunctionFunction>,
    transforms: [&Handle<TFunctionFunction>; 4],
    selections: [&Handle<TFunctionFunction>; 3],
) {
    let [xyz, along, rotated, mirrored] = transforms;
    let [line_selection, rotation_selection, plane_selection] = selections;
    let (driver, warnings) = selection_driver();
    set_box_dimensions(box_function, 290.0);
    execute_box(box_function);
    for (function, selection) in [
        (xyz, None),
        (along, Some(line_selection)),
        (rotated, Some(rotation_selection)),
        (mirrored, Some(plane_selection)),
    ] {
        recompute_transform(function, selection, &driver);
    }
    assert!(warnings.0.lock().unwrap().is_empty());
}

fn recompute_transform(
    function: &Handle<TFunctionFunction>,
    selection: Option<&Handle<TFunctionFunction>>,
    driver: &super::naming_selection_driver::NamingSelectionDriver,
) {
    if let Some(selection) = selection {
        assert!(!solve_selection(selection, driver).unwrap().is_null());
    }
    assert_eq!(execute_transform(function).unwrap(), 0);
    assert_eq!(current(function).shape_type().unwrap(), ShapeEnum::Solid);
}

#[test]
fn unsupported_transform_guid_and_invalid_axis_preserve_native_failure_codes() {
    let (_document, object, _) = modeled_box();
    let unsupported = add_function(
        &object,
        &guid_from_text("00000000-0000-0000-0000-000000000000"),
    )
    .unwrap();
    assert_eq!(execute_transform(&unsupported).unwrap(), -1);
    assert_eq!(unsupported.get_failure(), 15);
    let (_axis_document, axis_object, _) = modeled_box();
    let axis = add_function(
        &axis_object,
        &guid_from_text("12e94557-6dbc-11d4-b9c8-0060b0ee281b"),
    )
    .unwrap();
    assert_eq!(execute_transform(&axis).unwrap(), -1);
    assert_eq!(axis.get_failure(), 14);
}

#[test]
fn transformation_missing_function_or_predecessor_keeps_native_minus_one() {
    use martensite_tklcaf::tfunction::TFunctionDriver;
    let (_document, _object, function) = modeled_box();
    assert_eq!(execute_transform(&function).unwrap(), -1);
    assert_eq!(function.get_failure(), 0);
    let driver = super::naming_transform::NamingTransformDriver::default();
    driver.init(&function.label().root().unwrap());
    let mut log = TFunctionLogbook::set(&function.label()).unwrap();
    log.clear().unwrap();
    assert!(driver.must_execute(&log).unwrap());
    driver.validate(&log).unwrap();
    assert_eq!(driver.execute(&mut log).unwrap(), -1);
    assert!(log.is_empty());
}
