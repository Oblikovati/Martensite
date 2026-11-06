//! Actual fillet driver prerequisite; no substitute for the complete E2 chain.
use super::{
    document, modeled_box,
    naming_box_driver::{real_argument, result_label},
    naming_fillet::{NamingFilletDriver, execute_fillet, make_fillet},
    naming_function_graph::add_function,
    transform_checks::{current, selected_subshape},
};
use martensite_tkbrep::{topexp::Explorer, toptools::MapOfShape};
use martensite_tkcaf::tnaming::{TNamingEvolution, TNamingIterator};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::{
    guid_from_text,
    tfunction::{TFunctionDriver, TFunctionLogbook},
};

#[test]
fn real_box_edge_fillet_has_native_generated_and_modified_face_history() {
    let (document, object, box_function) = modeled_box();
    let source = current(&box_function);
    let (path, _selection) = selected_subshape(&document, &object, &source, 1, Some(3));
    let function = make_fillet(&object, &path).unwrap();
    let result = current(&function);
    assert_eq!(result.shape_type().unwrap(), ShapeEnum::Solid);
    assert_seven_faces(&result);
    let label = result_label(&function).unwrap();
    assert_fillet_history(&label, &source);
}

fn assert_seven_faces(result: &martensite_tkbrep::topods::Shape) {
    let mut faces = MapOfShape::default();
    for face in Explorer::new(result, ShapeEnum::Face, ShapeEnum::Shape).unwrap() {
        faces.add(face);
    }
    assert_eq!(faces.extent(), 7);
}

fn assert_fillet_history(
    label: &martensite_tklcaf::tdf::TdfLabel,
    source: &martensite_tkbrep::topods::Shape,
) {
    let root = TNamingIterator::from_label(label).unwrap();
    assert_eq!(root.evolution().unwrap(), TNamingEvolution::Modify);
    assert!(root.old_shape().unwrap().is_same(source));
    assert!(
        TNamingIterator::from_label(&label.find_child(1, false).unwrap())
            .unwrap()
            .more()
    );
    assert!(
        TNamingIterator::from_label(&label.find_child(2, false).unwrap())
            .unwrap()
            .more()
    );
}

#[test]
fn native_small_radius_is_rejected_before_missing_path() {
    let (_document, object, _box) = modeled_box();
    let function = add_function(
        &object,
        &guid_from_text("12e94553-6dbc-11d4-b9c8-0060b0ee281b"),
    )
    .unwrap();
    real_argument(&function, 1).unwrap().set_value(0.0).unwrap();
    assert_eq!(execute_fillet(&function).unwrap(), -1);
    assert_eq!(function.get_failure(), 14);
}

#[test]
fn fillet_missing_function_is_silent_with_native_execution_defaults() {
    let document = document();
    let label = document.main().unwrap();
    let driver = NamingFilletDriver::default();
    driver.init(&label);
    let mut log = TFunctionLogbook::set(&label).unwrap();
    assert!(driver.must_execute(&log).unwrap());
    driver.validate(&log).unwrap();
    assert_eq!(driver.execute(&mut log).unwrap(), -1);
    assert!(log.is_empty());
    assert!(
        super::naming_transform_arguments::previous_function(&Handle::default())
            .unwrap()
            .is_null()
    );
}
