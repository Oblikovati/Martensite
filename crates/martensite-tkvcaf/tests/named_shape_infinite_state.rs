//! Named-shape driver replacement retains the incoming-handle branch semantics.
mod support;
use martensite_tkcaf::tnaming::TNamingBuilder;
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::tdf::{TdfAttributeState, TdfLabel};
use martensite_tkmath::gp::Point3;
use martensite_tkv3d::{
    ais::{AisInteractiveObject, AisPoint, AisShape},
    prs_mgr::PrsMgrPresentableObjectMembers,
};
use martensite_tkvcaf::tprs_std::{TPrsStdDriver, TPrsStdNamedShapeDriver};

fn infinite_label() -> (
    Handle<martensite_tklcaf::tdoc_std::TDocStdDocument>,
    TdfLabel,
) {
    let document = support::new_document();
    let shape = support::make_box(1.0, 2.0, 3.0);
    shape.tshape().set_infinite(true);
    let named = support::named_shape(&document.main().unwrap(), &shape);
    (document, named.label())
}

fn update_infinite(object: &mut Handle<dyn AisInteractiveObject>) {
    let (_document, label) = infinite_label();
    assert!(
        TPrsStdNamedShapeDriver::new()
            .update(&label, object)
            .unwrap()
    );
    assert!(!object.cast::<AisShape>().is_null());
}

fn point_object() -> Handle<dyn AisInteractiveObject> {
    let point = Handle::new(martensite_tkg3d::geom::GeomCartesianPoint::from_pnt(
        Point3::default(),
    ));
    AisPoint::new(&point.cast()).unwrap().cast()
}

#[test]
fn replacing_a_non_shape_object_copies_the_named_shapes_infinite_state() {
    let mut object = point_object();
    let previous = object.object_address();
    update_infinite(&mut object);
    assert_ne!(object.object_address(), previous);
    assert!(object.is_infinite());
}

#[test]
fn building_from_a_null_object_keeps_the_new_shapes_default_finite_state() {
    let mut object = Handle::default();
    update_infinite(&mut object);
    assert!(!object.is_infinite());
}

#[test]
fn reusing_a_shape_object_updates_its_infinite_state_without_replacing_identity() {
    let mut object: Handle<dyn AisInteractiveObject> =
        AisShape::new(&support::make_box(2.0, 3.0, 4.0)).cast();
    let previous = object.object_address();
    update_infinite(&mut object);
    assert_eq!(object.object_address(), previous);
    assert!(object.is_infinite());
}

#[test]
fn changing_back_to_a_finite_named_shape_clears_the_kept_objects_infinite_state() {
    let (_document, label) = infinite_label();
    let mut object = point_object();
    let driver = TPrsStdNamedShapeDriver::new();
    assert!(driver.update(&label, &mut object).unwrap());
    assert!(object.is_infinite());
    let previous = object.object_address();
    let finite = support::make_box(4.0, 5.0, 6.0);
    TNamingBuilder::new(&label)
        .unwrap()
        .generated(&finite)
        .unwrap();
    assert!(driver.update(&label, &mut object).unwrap());
    assert_eq!(object.object_address(), previous);
    assert!(!object.is_infinite());
}
