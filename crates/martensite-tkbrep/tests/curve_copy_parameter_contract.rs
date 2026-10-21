//! Copy bound dispatch and exact typed-error transport through extension curves.
use super::curve_update_rebinding::rebinding_curve::RebindingParameterCurve;
use super::curve3d_copy_callbacks::space_curve::RebindingSpaceCurve;
use martensite_tkbrep::brep::{BRepCurveRepresentation, BRepGCurve};
use martensite_tkernel::standard::{Failure, Handle};
type CopyOutcome = Result<Handle<dyn BRepCurveRepresentation>, Failure>;
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkg3d::geom::{GeomCurve, GeomLine};
use martensite_tkmath::gp::{Axis1, Axis2d, Point2, Point3};
use std::sync::{Arc, atomic::AtomicUsize};

#[test]
fn ordinary_planar_bound_hooks_preserve_values() {
    let planar = Geom2dLine::new(Axis2d::default());
    assert_eq!(
        planar.try_first_parameter().unwrap(),
        planar.first_parameter()
    );
    assert_eq!(
        planar.try_last_parameter().unwrap(),
        planar.last_parameter()
    );
}

#[test]
fn ordinary_spatial_bound_hooks_preserve_values() {
    let spatial = GeomLine::new(Axis1::default());
    assert_eq!(
        spatial.try_first_parameter().unwrap(),
        spatial.first_parameter()
    );
    assert_eq!(
        spatial.try_last_parameter().unwrap(),
        spatial.last_parameter()
    );
}

#[test]
fn extension_bound_hooks_transport_raw_typed_failure_and_stack() {
    let planar = RebindingParameterCurve::new(
        Arc::new(AtomicUsize::new(0)),
        Arc::new(|u| Ok(Point2::new(u, 0.))),
    );
    planar.set_fallible_query(Arc::new(|_| Err(bound_failure())));
    assert_bound_failure(planar.try_first_parameter().unwrap_err());
    assert_bound_failure(planar.try_last_parameter().unwrap_err());
}

#[test]
fn spatial_extension_bound_hooks_transport_raw_typed_failure_and_stack() {
    let spatial = RebindingSpaceCurve::new(
        Arc::new(AtomicUsize::new(0)),
        Arc::new(|u| Ok(Point3::new(u, 0., 0.))),
    );
    spatial.set_fallible_query(Arc::new(|_| Err(bound_failure())));
    assert_bound_failure(spatial.try_first_parameter().unwrap_err());
    assert_bound_failure(spatial.try_last_parameter().unwrap_err());
}

fn bound_failure() -> Failure {
    Failure::with_type_bytes("Standard_RangeError", b"bound\xffvalue", b"owned-stack")
}

fn assert_bound_failure(error: Failure) {
    assert_eq!(error.exception_type(), "Standard_RangeError");
    assert_eq!(error.what(), b"bound\xffvalue");
    assert_eq!(error.stack_bytes(), b"owned-stack");
}

#[path = "curve_copy_parameter_contract/fixtures.rs"]
mod fixtures;
use fixtures::{RepresentationKind, SourceFixture};
use martensite_tkernel::standard::SharedState;
use std::sync::atomic::Ordering;

#[test]
fn spatial_copy_bound_queries_follow_linux_profile_and_transport_errors() {
    verify_failure_matrix(RepresentationKind::Space);
}

#[test]
fn surface_copy_bound_queries_follow_linux_profile_and_transport_errors() {
    verify_failure_matrix(RepresentationKind::Surface);
}

#[test]
fn seam_copy_bound_queries_follow_linux_profile_and_transport_errors() {
    verify_failure_matrix(RepresentationKind::Seam);
}

fn verify_failure_matrix(kind: RepresentationKind) {
    for mode in ["first", "last", "both", "none"] {
        let fixture = SourceFixture::new(kind);
        fixture.prepare();
        let trace = Arc::new(SharedState::new(Vec::new()));
        fixtures::failure_query(&fixture, mode, trace.clone());
        let copied = fixture.source.copy();
        verify_copy_result(mode, &fixture, copied);
        verify_failure_trace(mode, &fixture, &trace.read());
    }
}

fn verify_failure_trace(mode: &str, fixture: &SourceFixture, trace: &[&str]) {
    let expected = if mode == "none" || mode == "first" {
        vec!["last", "first"]
    } else {
        vec!["last"]
    };
    assert_eq!(trace, expected, "failure mode {mode}");
    let translation = fixture
        .source
        .location()
        .transformation()
        .translation_part();
    assert_eq!(translation.x(), expected.len() as f64);
}

fn verify_copy_result(mode: &str, fixture: &SourceFixture, copied: CopyOutcome) {
    if mode == "none" {
        let copied = copied.unwrap();
        assert!(fixture.bound_to(copied.get().unwrap()));
        assert_eq!(copied.location(), fixture.source.location());
        let updates = if fixture.source.is_curve3d() { 0 } else { 2 };
        assert_eq!(fixture.evaluations.load(Ordering::SeqCst), updates);
        return;
    }
    assert_bound_failure(copied.expect_err("bound exception must prevent result publication"));
    assert_eq!(fixture.evaluations.load(Ordering::SeqCst), 0);
    assert_eq!(fixture.source.range(), (2., 4.));
}

#[test]
fn spatial_copy_reloads_replacement_with_a_different_parameter_domain() {
    verify_replacement(RepresentationKind::Space, false);
}

#[test]
fn surface_copy_reloads_replacement_with_a_different_parameter_domain() {
    verify_replacement(RepresentationKind::Surface, false);
}

#[test]
fn seam_copy_reloads_replacement_with_a_different_parameter_domain() {
    verify_replacement(RepresentationKind::Seam, false);
}

fn verify_replacement(kind: RepresentationKind, fail_replacement: bool) {
    let original = SourceFixture::new(kind);
    let replacement = SourceFixture::new(kind);
    original.prepare();
    let trace = Arc::new(SharedState::new(Vec::new()));
    fixtures::replacement_queries(&original, &replacement, trace.clone(), fail_replacement);
    let copied = original.source.copy();
    assert_eq!(*trace.read(), ["old.last", "new.first"]);
    verify_replacement_result(&original, &replacement, copied, fail_replacement);
}

fn verify_replacement_result(
    original: &SourceFixture,
    replacement: &SourceFixture,
    copied: CopyOutcome,
    fail: bool,
) {
    if fail {
        assert_bound_failure(copied.expect_err("replacement query must prevent publication"));
        assert!(replacement.bound_to(original.source.get().unwrap()));
        assert_eq!(replacement.evaluations.load(Ordering::SeqCst), 0);
        return;
    }
    let copied = copied.unwrap();
    assert!(replacement.bound_to(copied.get().unwrap()));
    assert!(replacement.bound_to(original.source.get().unwrap()));
    assert_eq!(copied.cast::<dyn BRepGCurve>().range(), (2., 4.));
}

#[test]
fn spatial_copy_preserves_rebinding_before_replacement_query_failure() {
    verify_replacement(RepresentationKind::Space, true);
}

#[test]
fn surface_copy_preserves_rebinding_before_replacement_query_failure() {
    verify_replacement(RepresentationKind::Surface, true);
}

#[test]
fn seam_copy_preserves_rebinding_before_replacement_query_failure() {
    verify_replacement(RepresentationKind::Seam, true);
}
