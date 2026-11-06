//! Naming histories release topology when their standalone document is destroyed.
mod support;
use martensite_tkcaf::tnaming::TNamingBuilder;

#[test]
fn unnamed_box_topology_expires_after_last_owner_drops() {
    let shape = support::make_box(10., 10., 10.);
    let weak_shape = shape.tshape().downgrade();
    drop(shape);
    assert!(weak_shape.upgrade().is_null());
}

#[test]
fn named_box_topology_expires_after_document_drops() {
    let document = support::new_document();
    let shape = support::make_box(10., 10., 10.);
    let weak_shape = shape.tshape().downgrade();
    TNamingBuilder::new(&document.main().unwrap())
        .unwrap()
        .generated(&shape)
        .unwrap();
    drop(shape);
    drop(document);
    assert!(weak_shape.upgrade().is_null());
}
