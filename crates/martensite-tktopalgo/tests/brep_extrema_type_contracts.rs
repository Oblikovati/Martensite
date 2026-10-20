//! Native exception and descriptor contracts of distance-support objects.
use martensite_tkernel::standard::{RuntimeTyped, Transient};
use martensite_tktopalgo::brep_extrema::{BRepExtremaTriangleSet, uncompatible_shape};

#[test]
fn incompatible_support_is_caught_as_a_domain_error() {
    let failure = uncompatible_shape("support IsVertex; expected EDGE");
    assert!(failure.is_kind("Standard_DomainError"));
    assert!(failure.is_kind("Standard_Failure"));
    assert!(!failure.is_kind("Standard_RangeError"));
    assert_eq!(failure.message(), "support IsVertex; expected EDGE");
}

#[test]
fn triangle_descriptor_inherits_the_named_bvh_base() {
    let triangles = BRepExtremaTriangleSet::new();
    let descriptor = triangles.dynamic_type();
    assert_eq!(descriptor.name(), "BRepExtrema_TriangleSet");
    assert_eq!(BRepExtremaTriangleSet::TYPE_NAME, descriptor.name());
    assert_eq!(descriptor.parent().unwrap().name(), "BVH_ObjectTransient");
    assert!(std::ptr::eq(
        descriptor,
        BRepExtremaTriangleSet::static_type()
    ));
    assert!(triangles.is_kind_name("BVH_ObjectTransient"));
    assert!(triangles.is_kind_name("Standard_Transient"));
    assert!(!triangles.is_kind_name("BVH_PrimitiveSet3d"));
}

#[test]
fn explicit_stack_constructor_retains_stack_and_domain_base() {
    use martensite_tktopalgo::brep_extrema::uncompatible_shape_with_stack;
    let failure = uncompatible_shape_with_stack("support IsVertex", "caller trace");
    assert_eq!(failure.exception_type(), "BRepExtrema_UnCompatibleShape");
    assert_eq!(failure.message(), "support IsVertex");
    assert_eq!(failure.stack_string(), "caller trace");
    assert!(failure.is_kind("Standard_DomainError"));
    assert!(failure.is_kind("Standard_Failure"));
}

#[test]
fn bvh_base_handle_preserves_properties_and_concrete_identity() {
    use martensite_tkernel::standard::Handle;
    use martensite_tkmath::bvh::{BvhObjectTransientSource, BvhProperties, BvhTransform};
    use std::sync::Arc;
    let concrete = Handle::new(BRepExtremaTriangleSet::new());
    let base: Handle<dyn BvhObjectTransientSource> = concrete.cast();
    let properties: Arc<dyn BvhProperties> = Arc::new(BvhTransform::<f64>::default());
    base.set_properties(Some(properties.clone()));
    assert!(Arc::ptr_eq(&concrete.properties().unwrap(), &properties));
    assert!(!base.is_dirty());
    base.set_properties(None);
    assert!(concrete.properties().is_none());
    assert!(!base.cast::<BRepExtremaTriangleSet>().is_null());
}

#[test]
fn bvh_base_handle_dirty_flag_is_consumed_by_concrete_tree_rebuild() {
    use martensite_tkernel::standard::Handle;
    use martensite_tkmath::bvh::BvhObjectTransientSource;
    let concrete = Handle::new(BRepExtremaTriangleSet::new());
    let base: Handle<dyn BvhObjectTransientSource> = concrete.cast();
    assert!(!base.is_dirty());
    base.mark_dirty();
    assert!(concrete.is_dirty());
    concrete.bvh().unwrap();
    assert!(!base.is_dirty());
}
