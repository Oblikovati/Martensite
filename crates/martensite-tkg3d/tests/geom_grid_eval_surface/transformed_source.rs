//! #592: the capability borrows an evaluator and retains the native descriptor alias.
use super::*;
use martensite_tkg3d::geom_adaptor::{GeomAdaptorTransformedSurface, GeomTransformedSurfaceSource};
use martensite_tkmath::gp::Transform3;

fn assert_send_sync<T: ?Sized + Send + Sync>() {}

fn assert_source_contract() {
    assert_send_sync::<dyn GeomTransformedSurfaceSource>();
    assert_send_sync::<GeomAdaptorTransformedSurface>();
    assert!(std::ptr::eq(
        <dyn GeomTransformedSurfaceSource>::static_type(),
        GeomAdaptorTransformedSurface::static_type()
    ));
}

#[test]
fn transformed_source_retains_owner_descriptor_and_borrow_without_extra_counts() {
    assert_source_contract();
    let geometry = PlaneZ(0., 0., 0.).into_upcast();
    let owner = Handle::new(
        GeomAdaptorTransformedSurface::with_surface(&geometry, Transform3::default()).unwrap(),
    );
    let source = owner.upcast::<dyn GeomTransformedSurfaceSource>();
    assert_eq!(owner.object_address(), source.object_address());
    assert!(std::ptr::eq(source.transformed_surface(), &*owner));
    assert_eq!(owner.ref_count(), 2);
    let grid = GeomGridEvalSurface::from_adaptor(&*source).unwrap();
    assert!(matches!(
        grid.evaluator(),
        martensite_tkg3d::geom_grid_eval::GeomGridEvalSurfaceEvaluator::Plane(_)
    ));
    assert_eq!(owner.ref_count(), 2);
}

#[test]
fn plain_surface_adaptor_does_not_claim_an_unowned_transformed_component() {
    let geometry = PlaneZ(0., 0., 0.).into_upcast();
    let owner = Handle::new(GeomAdaptorSurface::with_surface(&geometry).unwrap());
    assert!(owner.cast::<dyn GeomTransformedSurfaceSource>().is_null());
    assert_eq!(owner.ref_count(), 1);
    let grid = GeomGridEvalSurface::from_adaptor(&*owner).unwrap();
    assert!(matches!(
        grid.evaluator(),
        martensite_tkg3d::geom_grid_eval::GeomGridEvalSurfaceEvaluator::Plane(_)
    ));
}
