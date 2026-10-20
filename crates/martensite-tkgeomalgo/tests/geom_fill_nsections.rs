//! Tests of `GeomFill_NSections`.
#![allow(non_snake_case)]
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve};
use martensite_tkgeomalgo::geom_fill::GeomFillNSections;
use martensite_tkmath::gp::Point3;

/// Builds a simple degree-1 BSpline line segment from (0,0,0) to (1,0,0).
fn makeSimpleLinearCurve() -> Handle<GeomBSplineCurve> {
    let mut aPoles: Array1<'static, Point3> = Array1::with_bounds(1, 2).unwrap();
    aPoles[1] = Point3::new(0.0, 0.0, 0.0);
    aPoles[2] = Point3::new(1.0, 0.0, 0.0);

    let mut aKnots: Array1<'static, f64> = Array1::with_bounds(1, 2).unwrap();
    aKnots[1] = 0.0;
    aKnots[2] = 1.0;

    let mut aMults: Array1<'static, i32> = Array1::with_bounds(1, 2).unwrap();
    aMults[1] = 2;
    aMults[2] = 2;

    Handle::new(GeomBSplineCurve::new(&aPoles, &aKnots, &aMults, 1, false).unwrap())
}

// Tests that GeomFill_NSections with a single-curve sequence does not throw.
// The resulting BSplineSurface may be null for a degenerate single-section case,
// but no exception should be raised.
#[test]
fn GeomFill_NSectionsTest__OCC27875_SingleCurveDoesNotThrow() {
    let aCurve = makeSimpleLinearCurve();
    assert!(!aCurve.is_null());

    let aNC: Vec<Handle<dyn GeomCurve>> = vec![aCurve.cast()];

    // Must not throw even though result may be degenerate
    assert!(GeomFillNSections::new(&aNC).is_ok());
}
