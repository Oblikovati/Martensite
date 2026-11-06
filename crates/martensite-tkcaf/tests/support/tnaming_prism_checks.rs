//! Independent native prism driver geometry, attachment history and recomputation checks.
use super::naming_function_values as values;
use super::{
    modeled_box,
    naming_box_driver::{real_argument, result_label},
    naming_draw_selection::setup,
    naming_function_graph::{add_function, add_object},
    naming_prism_driver::{NamingPrismDriver, execute_prism},
    naming_sweep_basis::{attachment_context, first_function, is_attachment},
    naming_transform_arguments::set_object_argument,
    transform_checks::current,
};
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkcaf::tnaming::{TNaming, TNamingEvolution, TNamingIterator};
use martensite_tkernel::{
    precision,
    standard::{Handle, Transient},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::TDataStdUAttribute,
    tdf::TdfAttributeState,
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionDriver, TFunctionFunction, TFunctionLogbook},
};
use martensite_tkmath::{gp::Transform3, toploc::Location};
use martensite_tktopalgo::brep_gprop::BRepGProp;
fn primitive(
    document: &Handle<TDocStdDocument>,
    basis: &Handle<TDataStdUAttribute>,
    height: f64,
) -> Handle<TFunctionFunction> {
    let object = add_object(document).unwrap();
    let guid = guid_from_text("12e94550-6dbc-11d4-b9c8-0060b0ee281b");
    let function = add_function(&object, &guid).unwrap();
    set_object_argument(&function, 1, basis).unwrap();
    real_argument(&function, 2)
        .unwrap()
        .set_value(height)
        .unwrap();
    values::integer_argument(&function, 3)
        .unwrap()
        .set_value(0)
        .unwrap();
    function
}
fn selected_kind(shape: &Shape, kind: ShapeEnum) -> Shape {
    Iterator::next(&mut Explorer::new(shape, kind, ShapeEnum::Shape).unwrap()).unwrap()
}
fn assert_volume(shape: &Shape, expected: f64) {
    let mut properties = GProps::default();
    BRepGProp::volume_properties(shape, &mut properties, false, false, false).unwrap();
    assert!((properties.mass() - expected).abs() < expected * 1e-12);
}
fn assert_names(function: &Handle<TFunctionFunction>, context: &Shape) {
    let result = result_label(function).unwrap();
    let root = TNamingIterator::from_label(&result).unwrap();
    assert_eq!(root.evolution().unwrap(), TNamingEvolution::Generated);
    assert!(root.old_shape().unwrap().is_same(context));
    assert_four_edge_generated_faces(&result);
    assert_caps(&result);
}
/// Verify every rectangle-edge generated face, not a nonempty subset; `assert_four_edge_generated_faces(&label)`.
pub(super) fn assert_four_edge_generated_faces(result: &martensite_tklcaf::tdf::TdfLabel) {
    let mut lateral = TNamingIterator::from_label(&result.find_child(1, false).unwrap()).unwrap();
    let mut count = 0;
    while lateral.more() {
        assert_eq!(
            lateral.old_shape().unwrap().shape_type().unwrap(),
            ShapeEnum::Edge
        );
        assert_eq!(
            lateral.new_shape().unwrap().shape_type().unwrap(),
            ShapeEnum::Face
        );
        count += 1;
        lateral.next().unwrap();
    }
    assert_eq!(count, 4);
}
fn assert_caps(result: &martensite_tklcaf::tdf::TdfLabel) {
    for tag in [2, 3] {
        let names = TNamingIterator::from_label(&result.find_child(tag, false).unwrap()).unwrap();
        assert_eq!(names.evolution().unwrap(), TNamingEvolution::Primitive);
        assert_eq!(
            names.new_shape().unwrap().shape_type().unwrap(),
            ShapeEnum::Face
        );
    }
    assert!(result.find_child(4, false).unwrap().is_null());
}
#[test]
fn attached_face_and_closed_wire_sweep_to_exact_prism_with_context_history() {
    for kind in [ShapeEnum::Face, ShapeEnum::Wire] {
        let (document, object, box_function) = modeled_box();
        let context = current(&box_function);
        let shape = selected_kind(&context, kind);
        let basis = setup::make_selection(&document, &object, &shape, false).unwrap();
        assert!(basis.selected);
        assert!(is_attachment(&basis.object).unwrap());
        assert!(!first_function(&basis.object).unwrap().is_null());
        assert!(!attachment_context(&basis.object).unwrap().is_empty());
        let prism = primitive(&document, &basis.object, 300.0);
        assert_eq!(execute_prism(&prism).unwrap(), 0);
        assert_volume(&current(&prism), 300.0 * 290.0 * 390.0);
        assert_names(&prism, &context);
        assert_volume(&current(&box_function), 190.0 * 290.0 * 390.0);
    }
}
fn actual_basis(
    document: &Handle<TDocStdDocument>,
    object: &Handle<TDataStdUAttribute>,
    context: &Shape,
    kind: ShapeEnum,
) -> Handle<TDataStdUAttribute> {
    let shape = selected_kind(context, kind);
    let selection = setup::make_selection(document, object, &shape, false).unwrap();
    assert!(selection.selected);
    assert!(selection.exception.is_none());
    selection.object
}
fn displace_prism(prism: &Handle<TFunctionFunction>) -> Location {
    let location = Location::new(Transform3::translation([10.0, 20.0, 30.0]));
    TNaming::displace(&result_label(prism).unwrap(), &location, true).unwrap();
    location
}
#[test]
fn recomputation_retains_location_resets_tags_and_accepts_any_nonzero_reverse() {
    let (document, object, box_function) = modeled_box();
    let context = current(&box_function);
    let basis = actual_basis(&document, &object, &context, ShapeEnum::Face);
    let prism = primitive(&document, &basis, 300.0);
    assert_eq!(execute_prism(&prism).unwrap(), 0);
    let location = displace_prism(&prism);
    real_argument(&prism, 2).unwrap().set_value(500.0).unwrap();
    values::integer_argument(&prism, 3)
        .unwrap()
        .set_value(-7)
        .unwrap();
    assert_eq!(execute_prism(&prism).unwrap(), 0);
    assert_eq!(current(&prism).location(), &location);
    assert_volume(&current(&prism), 500.0 * 290.0 * 390.0);
    assert!(current(&box_function).location().is_identity());
    assert_caps(&result_label(&prism).unwrap());
}
#[test]
fn unsupported_basis_and_confusion_height_preserve_wrong_argument_code() {
    let (document, object, box_function) = modeled_box();
    let shape = current(&box_function);
    for (kind, height) in [
        (ShapeEnum::Edge, 300.0),
        (ShapeEnum::Face, precision::confusion()),
    ] {
        let basis =
            setup::make_selection(&document, &object, &selected_kind(&shape, kind), false).unwrap();
        let prism = primitive(&document, &basis.object, height);
        assert_eq!(execute_prism(&prism).unwrap(), -1);
        assert_eq!(prism.get_failure(), 14);
    }
    let empty = add_object(&document).unwrap();
    let prism = primitive(&document, &empty, 300.0);
    assert_eq!(execute_prism(&prism).unwrap(), -1);
    assert_eq!(prism.get_failure(), 14);
}
#[test]
fn default_prism_driver_retains_native_rtti_and_noop_contracts() {
    let document = super::document();
    let driver = NamingPrismDriver::default();
    driver.init(&document.main().unwrap());
    assert_eq!(driver.dynamic_type().name(), "DNamingPrismDriver");
    let mut log = TFunctionLogbook::set(&document.main().unwrap()).unwrap();
    assert!(driver.must_execute(&log).unwrap());
    driver.validate(&log).unwrap();
    assert_eq!(driver.execute(&mut log).unwrap(), -1);
    assert!(log.is_empty());
    assert!(first_function(&Handle::default()).unwrap().is_null());
}

#[test]
fn nonattachment_face_value_uses_primitive_root_naming() {
    let (document, object, box_function) = modeled_box();
    let base = result_label(&box_function)
        .unwrap()
        .find_child(1, false)
        .unwrap();
    martensite_tklcaf::tdf::TdfReference::set(&object.label(), &base).unwrap();
    assert!(!is_attachment(&object).unwrap());
    let prism = primitive(&document, &object, 300.0);
    assert_eq!(execute_prism(&prism).unwrap(), 0);
    assert_volume(&current(&prism), 300.0 * 190.0 * 290.0);
    let names = TNamingIterator::from_label(&result_label(&prism).unwrap()).unwrap();
    assert_eq!(names.evolution().unwrap(), TNamingEvolution::Primitive);
}
#[test]
fn external_attachment_is_recognized_but_has_no_internal_context() {
    let (document, object, box_function) = modeled_box();
    let face = selected_kind(&current(&box_function), ShapeEnum::Face);
    let basis = setup::make_selection(&document, &object, &face, false).unwrap();
    first_function(&basis.object)
        .unwrap()
        .set_driver_guid(&guid_from_text("12e94555-6dbc-11d4-b9c8-0060b0ee281b"))
        .unwrap();
    assert!(is_attachment(&basis.object).unwrap());
    assert!(attachment_context(&basis.object).unwrap().is_null());
    let prism = primitive(&document, &basis.object, 300.0);
    assert_eq!(execute_prism(&prism).unwrap(), -1);
    assert_eq!(prism.get_failure(), 14);
}
fn axis_components(axis: martensite_tkmath::gp::Axis1) -> [f64; 3] {
    let direction = axis.direction();
    [direction.x(), direction.y(), direction.z()]
}
#[test]
fn sweep_axis_tracks_forward_and_reversed_planar_face_normals() {
    let maker = martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace::from_plane(
        martensite_tkmath::gp::Plane3::default(),
    )
    .unwrap();
    let mut face = (*maker.face().unwrap()).clone();
    let forward = super::naming_sweep_basis::sweep_axis(&face).unwrap();
    assert_eq!(axis_components(forward), [0.0, 0.0, 1.0]);
    face.reverse();
    let reversed = super::naming_sweep_basis::sweep_axis(&face).unwrap();
    assert_eq!(axis_components(reversed), [0.0, 0.0, -1.0]);
}
