//! #592 retains naming-map identities and actual attribute owners across typed views.
use crate::{
    tdata_xtd::{TDataXtdPattern, TDataXtdPatternStd},
    tnaming::{TNamingCopyShape, TNamingNamedShape, TNamingTransientMap},
};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTVertex, BRepTool},
    topods::{Shape, Vertex, vertex},
};
use martensite_tkernel::standard::{Handle, RuntimeTyped, Transient};
use martensite_tklcaf::tdf::{TdfAttribute, TdfData};
use martensite_tkmath::gp::Point3;

fn source_vertex() -> Shape {
    let mut vertex = Vertex::default();
    BRepBuilder::default()
        .make_vertex_at(&mut vertex, Point3::new(1., 2., 3.), 1e-7)
        .unwrap();
    vertex.into_shape()
}

#[test]
fn naming_copy_reuses_actual_brep_owners_in_the_erased_identity_map() {
    let source = source_vertex();
    let mut map = TNamingTransientMap::default();
    let _: &(dyn Send + Sync) = &map;
    let (mut first, mut second) = (Shape::default(), Shape::default());
    TNamingCopyShape::copy_tool(&source, &mut map, &mut first).unwrap();
    let count = map.extent();
    TNamingCopyShape::copy_tool(&source, &mut map, &mut second).unwrap();
    assert_eq!(map.extent(), count);
    assert!(first.is_same(&second));
    assert_classic_copy(&source, &first, &map);
}

fn assert_classic_copy(source: &Shape, copy: &Shape, map: &TNamingTransientMap) {
    let key = source.tshape().upcast::<dyn Transient>();
    assert!(!copy.is_same(source));
    assert!(copy.tshape().downcast::<BRepTVertex>().is_some());
    assert_eq!(
        map.seek(&key).unwrap().object_address(),
        copy.tshape().object_address()
    );
    assert_eq!(
        BRepTool::point(&vertex(copy).unwrap()).unwrap(),
        Point3::new(1., 2., 3.)
    );
}

#[test]
fn pattern_interface_upcast_preserves_label_binding_and_concrete_identity() {
    let data = TdfData::new();
    let pattern = TDataXtdPatternStd::set(&data.root()).unwrap();
    let virtual_pattern = pattern.upcast::<dyn TDataXtdPattern>();
    let attribute = virtual_pattern.into_upcast::<dyn TdfAttribute>();
    assert_eq!(attribute.object_address(), pattern.object_address());
    assert_eq!(attribute.id(), pattern.id());
    assert_eq!(attribute.label(), data.root());
    assert_eq!(
        attribute.downcast::<TDataXtdPatternStd>().object_address(),
        pattern.object_address()
    );
    assert!(attribute.is_kind(TDataXtdPatternStd::static_type()));
}

#[test]
fn named_shape_attribute_keeps_its_native_descriptor_and_actual_rust_owner() {
    let named = Handle::new(TNamingNamedShape::new());
    let attribute = named.upcast::<dyn TdfAttribute>();
    let _: &(dyn Send + Sync) = &named;
    assert_eq!(
        attribute.dynamic_type().unwrap().name(),
        "TNamingNamedShape"
    );
    assert_eq!(attribute.id(), TNamingNamedShape::get_id());
    assert_eq!(
        attribute.downcast::<TNamingNamedShape>().object_address(),
        named.object_address()
    );
}
