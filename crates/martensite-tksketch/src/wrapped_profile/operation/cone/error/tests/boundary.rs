//! Bug #547: failure erasure must preserve formatting, concrete identity and borrowed work.
use super::*;
use crate::surface_sketch::wrapped_profile::api::error as public_error;

fn missing() -> ConeFailure {
    ConeFailure::new(
        Some(SurfaceRegionId(17)),
        None,
        WrappedFailureClass::Incomplete,
        ConeCause::Missing(SurfaceRegionId(17)),
    )
}
#[test]
fn cone_failure_boundary_pins_display_debug_and_source_depth() {
    let failure = missing().public();
    assert_eq!(
        failure.to_string(),
        "NormalExtrusion stage=Construction class=Incomplete entities=[Region(SurfaceRegionId(17))]: cone region=Some(SurfaceRegionId(17)) span=None: region=SurfaceRegionId(17); expected one retained owner"
    );
    assert_eq!(
        format!("{failure:?}"),
        "WrappedNormalFailure { operation: NormalExtrusion, stage: Construction, class: Incomplete, entities: [Region(SurfaceRegionId(17))], cause: Cone(ConeFailure { stage: Construction, class: Incomplete, region: Some(SurfaceRegionId(17)), span: None, cause: Missing(SurfaceRegionId(17)) }) }"
    );
    let outer = failure.source().unwrap();
    assert!(outer.downcast_ref::<public_error::Cause>().is_some());
    let concrete = retained_cone(&failure);
    assert!(matches!(
        concrete.cause,
        ConeCause::Missing(SurfaceRegionId(17))
    ));
    assert!(concrete.source().is_none());
    assert!(failure.planar_work_failure().is_none());
}
#[test]
fn cylinder_failure_boundary_keeps_legacy_display_debug_and_source_depth() {
    let failure = public_error::failure(
        WrappedStage::Admission,
        WrappedFailureClass::InvalidInput,
        Some(SurfaceRegionId(17)),
        public_error::Cause::Depths([2., -1.]),
    );
    assert_eq!(
        failure.to_string(),
        "NormalExtrusion stage=Admission class=InvalidInput entities=[Region(SurfaceRegionId(17))]: depths=[2.0, -1.0]; expected increasing finite signed normal depths"
    );
    assert_eq!(
        format!("{failure:?}"),
        "WrappedNormalFailure { operation: NormalExtrusion, stage: Admission, class: InvalidInput, entities: [Region(SurfaceRegionId(17))], cause: Depths([2.0, -1.0]) }"
    );
    let cause = failure.source().unwrap();
    assert!(cause.downcast_ref::<public_error::Cause>().is_some());
    assert!(cause.source().is_none());
    assert!(failure.planar_work_failure().is_none());
}
#[test]
fn cone_failure_boundary_retains_native_leaf_identity_and_alternate_debug() {
    let leaf = Box::new(Failure::new("native error boundary control"));
    let identity = leaf.as_ref() as *const Failure;
    let concrete = native_failure(leaf);
    let debug = format!("{concrete:#?}");
    let failure = concrete.public();
    let original = concrete_source(&failure);
    assert_eq!(format!("{original:#?}"), debug);
    assert!(original.downcast_ref::<ConeFailure>().is_some());
    let retained = original
        .source()
        .unwrap()
        .downcast_ref::<Failure>()
        .unwrap();
    assert_eq!(retained as *const Failure, identity);
    assert!(retained.source().is_none());
    assert_eq!(failure.class(), WrappedFailureClass::Native);
}
#[test]
fn cone_failure_boundary_borrows_original_planar_work_without_an_extra_source_layer() {
    let failure = cancelled_planar();
    let concrete = concrete_source(&failure);
    assert!(concrete.downcast_ref::<ConeFailure>().is_some());
    let work = concrete
        .source()
        .unwrap()
        .downcast_ref::<martensite_tkbo::profile_extrusion::PlanarRegionWorkFailure>()
        .unwrap();
    assert!(std::ptr::eq(work, failure.planar_work_failure().unwrap()));
    assert!(work.is_cancelled());
    assert_eq!(failure.class(), WrappedFailureClass::Cancelled);
}
#[test]
fn cone_failure_boundary_preserves_owned_send_and_sync() {
    fn require_shared<T: Send + Sync>() {}
    require_shared::<WrappedNormalFailure>();
    require_shared::<ConeFailure>();
}

fn concrete_source(failure: &WrappedNormalFailure) -> &(dyn Error + 'static) {
    failure.source().unwrap().source().unwrap()
}
fn native_failure(leaf: Box<Failure>) -> ConeFailure {
    ConeFailure::new(
        None,
        None,
        WrappedFailureClass::Incomplete,
        ConeCause::Native(leaf),
    )
}

fn retained_cone(failure: &WrappedNormalFailure) -> &ConeFailure {
    concrete_source(failure)
        .downcast_ref::<ConeFailure>()
        .unwrap()
}
fn cancelled_planar() -> WrappedNormalFailure {
    let request = super::super::super::tests::request();
    let inputs = input::capture(&request, &mut || false).unwrap();
    let cause = planar::capture_with_work_and_cancel(
        &inputs.originals,
        request.options.spatial_error,
        request.options.planar_work,
        || true,
    )
    .err()
    .unwrap();
    projected(ConeCause::Planar(Box::new(cause)))
}
