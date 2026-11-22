//! #592 keeps native descriptors without projecting embedded base-class objects.
use crate::{
    bop_ds::{CommonBlock, PaveBlock},
    int_tools::{Context, TopolTool},
};
use martensite_tkernel::standard::{Handle, RuntimeTyped, StandardTransient, Transient};
use martensite_tkg3d::{
    adaptor3d::Adaptor3dTopolTool,
    geom::{GeomCurve, GeomLine},
};
use martensite_tkmath::gp::Axis1;
use std::sync::Arc;

static_assertions::assert_impl_all!(Context: Send, Sync);
static_assertions::assert_impl_all!(PaveBlock: Send, Sync);
static_assertions::assert_impl_all!(CommonBlock: Send, Sync);
static_assertions::assert_impl_all!(TopolTool: Send, Sync);

fn assert_root_identity<T: Transient + RuntimeTyped>(value: T, name: &str) {
    let concrete = Handle::new(value);
    let erased = concrete.upcast::<dyn Transient>();
    assert_eq!(erased.dynamic_type().unwrap().name(), name);
    assert!(erased.is_kind_name("Standard_Transient"));
    assert_eq!(erased.object_address(), concrete.object_address());
    assert_eq!(
        erased.downcast::<T>().object_address(),
        concrete.object_address()
    );
    assert!(erased.downcast::<StandardTransient>().is_null());
}

#[test]
fn bookkeeping_free_classes_retain_native_names_and_actual_concrete_owners() {
    assert_root_identity(Context::default(), "IntToolsContext");
    assert_root_identity(PaveBlock::default(), "BOPDSPaveBlock");
    assert_root_identity(CommonBlock::default(), "BOPDSCommonBlock");
}

#[test]
fn topology_composition_retains_ancestry_without_a_sliced_owner() {
    let tool = Handle::new(TopolTool::default());
    let erased = tool.upcast::<dyn Transient>();
    assert_eq!(erased.dynamic_type().unwrap().name(), "IntToolsTopolTool");
    assert!(erased.is_kind(Adaptor3dTopolTool::static_type()));
    assert_eq!(
        erased.downcast::<TopolTool>().object_address(),
        tool.object_address()
    );
    assert!(erased.downcast::<Adaptor3dTopolTool>().is_null());
}

#[test]
fn curve_projector_cache_reuses_the_retained_allocation_across_views() {
    let context = Context::default();
    let curve = Handle::new(GeomLine::new(Axis1::default())).into_upcast::<dyn GeomCurve>();
    let projector = context.proj_pt(&curve).unwrap();
    let erased = curve.upcast::<dyn Transient>();
    let restored = erased.downcast::<GeomLine>().into_upcast::<dyn GeomCurve>();
    assert_eq!(curve.object_address(), restored.object_address());
    assert!(Arc::ptr_eq(
        &projector,
        &context.proj_pt(&restored).unwrap()
    ));
}
