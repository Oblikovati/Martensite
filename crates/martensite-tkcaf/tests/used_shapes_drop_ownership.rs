//! Final dynamic attribute ownership clears map keys without deleting caller-owned references.
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Shape, Vertex},
};
use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingRefShape, TNamingUsedShapes};
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::tdf::{TdfAttribute, TdfData};
use std::sync::Arc;

fn detached_used_shapes() -> Handle<TNamingUsedShapes> {
    let framework = TdfData::new();
    TNamingBuilder::new(&framework.root()).unwrap();
    let mut attached = Handle::<TNamingUsedShapes>::default();
    assert!(
        framework
            .root()
            .find_attribute_as(&TNamingUsedShapes::get_id(), &mut attached)
            .unwrap()
    );
    attached.new_empty().cast()
}
fn vertex_shape() -> Shape {
    let mut vertex = Vertex::default();
    BRepBuilder::default().make_vertex(&mut vertex);
    vertex.into_shape()
}
fn assert_map_owners(used: &TNamingUsedShapes, shape: &Shape, reference: &Arc<TNamingRefShape>) {
    assert_eq!(used.map().len(), 1);
    assert!(Arc::ptr_eq(used.map().seek(shape).unwrap(), reference));
    assert_eq!(Arc::strong_count(reference), 2);
    assert_eq!(shape.tshape().ref_count(), 3);
}

#[test]
fn last_dynamic_attribute_owner_releases_map_keys_and_preserves_external_reference() {
    let used = detached_used_shapes();
    let weak_used = used.downgrade();
    let shape = vertex_shape();
    let reference = Arc::new(TNamingRefShape::new(&shape));
    assert_eq!(shape.tshape().ref_count(), 2);
    used.map().bind(shape.clone(), reference.clone());
    let dynamic: Handle<dyn TdfAttribute> = used.cast();
    drop(used);
    assert_map_owners(&dynamic.cast::<TNamingUsedShapes>(), &shape, &reference);
    drop(dynamic);
    assert!(weak_used.upgrade().is_null());
    release_external_reference(shape, reference);
}
fn release_external_reference(shape: Shape, reference: Arc<TNamingRefShape>) {
    let weak_topology = shape.tshape().downgrade();
    assert_eq!(Arc::strong_count(&reference), 1);
    assert_eq!(shape.tshape().ref_count(), 2);
    assert!(reference.shape().is_same(&shape));
    drop(shape);
    assert!(!weak_topology.upgrade().is_null());
    drop(reference);
    assert!(weak_topology.upgrade().is_null());
}
