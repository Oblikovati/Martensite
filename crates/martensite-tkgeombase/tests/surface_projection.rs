//! Geom proj lib contracts.
// The reference values keep the defined literals digit for digit.
#![allow(non_snake_case, clippy::excessive_precision)]
mod common;
use common::prelude::*;
use martensite_tkg3d::geom::{GeomHyperbola, GeomParabola, GeomPlane};
use martensite_tkgeombase::gcpnts::AbscissaPoint as GCPnts_AbscissaPoint;
use martensite_tkgeombase::geom_proj_lib as GeomProjLib;
use martensite_tkmath::gp::Plane3;

fn makeOCC31661Plane() -> Handle<GeomPlane> {
    Handle::new(GeomPlane::from_pln(
        Plane3::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
    ))
}

fn makeOCC31661CurveAxis() -> Axis2 {
    Axis2::with_x_direction(
        Point3::new(0.0, 0.0, 0.0),
        dir3(1.0, 1.0, 1.0),
        dir3(2.0, 0.0, -2.0),
    )
    .unwrap()
}

// `aCurve->DynamicType()->Name()`.
fn dynamicTypeName(aCurve: &Handle<dyn GeomCurve>) -> &'static str {
    aCurve.dynamic_type().unwrap().name()
}

// Migrated from tests/the regression fixture Projection of a parabola onto
// the XY plane must preserve its conic type and the trimmed parameter range.
#[test]
fn case_1_project_parabola_on_plane() {
    let aParabola: Handle<dyn GeomCurve> =
        Handle::new(GeomParabola::new(makeOCC31661CurveAxis(), 10.0).unwrap()).cast();
    let aPlane: Handle<GeomPlane> = makeOCC31661Plane();

    let aProjected: Handle<dyn GeomCurve> =
        GeomProjLib::project_on_plane(&aParabola, &aPlane, dir3(0.0, 0.0, 1.0), false).unwrap();
    assert!(!aProjected.is_null());
    assert_eq!(dynamicTypeName(&aProjected), "Geom_Parabola");

    let aTrimmed: Handle<dyn GeomCurve> =
        Handle::new(GeomTrimmedCurve::new(&aParabola, -100.0, 100.0, true, true).unwrap()).cast();
    let aProjectedTrimmed: Handle<dyn GeomCurve> =
        GeomProjLib::project_on_plane(&aTrimmed, &aPlane, dir3(0.0, 0.0, 1.0), false).unwrap();
    assert!(!aProjectedTrimmed.is_null());
    assert_eq!(dynamicTypeName(&aProjectedTrimmed), "Geom_TrimmedCurve");

    let anAdaptor = GeomAdaptorCurve::with_curve(&aProjectedTrimmed).unwrap();
    EXPECT_NEAR(
        GCPnts_AbscissaPoint::length_with_tolerance(&anAdaptor, precision::confusion()).unwrap(),
        408.40363195229503,
        2.0e-5,
    );
    // The parameter checks retain geometry command's references; the wider bound reflects
    // the relative numerical tolerance used by its checkreal command.
    let aCurve: &dyn GeomCurve = aProjectedTrimmed.get().unwrap();
    EXPECT_NEAR(aCurve.first_parameter(), -91.077748943768597, 2.0e-6);
    EXPECT_NEAR(aCurve.last_parameter(), 72.221567418462357, 2.0e-6);
}

// Migrated from tests/the regression fixture Projection of a hyperbola
// must not crash and must retain the hyperbolic conic type after trimming.
#[test]
fn case_2_project_hyperbola_on_plane() {
    let aHyperbola: Handle<dyn GeomCurve> =
        Handle::new(GeomHyperbola::new(makeOCC31661CurveAxis(), 10.0, 10.0).unwrap()).cast();
    let aPlane: Handle<GeomPlane> = makeOCC31661Plane();

    let aProjected: Handle<dyn GeomCurve> =
        GeomProjLib::project_on_plane(&aHyperbola, &aPlane, dir3(0.0, 0.0, 1.0), false).unwrap();
    assert!(!aProjected.is_null());
    assert_eq!(dynamicTypeName(&aProjected), "Geom_Hyperbola");

    let aTrimmed: Handle<dyn GeomCurve> =
        Handle::new(GeomTrimmedCurve::new(&aHyperbola, -5.0, 5.0, true, true).unwrap()).cast();
    let aProjectedTrimmed: Handle<dyn GeomCurve> =
        GeomProjLib::project_on_plane(&aTrimmed, &aPlane, dir3(0.0, 0.0, 1.0), false).unwrap();
    assert!(!aProjectedTrimmed.is_null());
    assert_eq!(dynamicTypeName(&aProjectedTrimmed), "Geom_TrimmedCurve");

    let anAdaptor = GeomAdaptorCurve::with_curve(&aProjectedTrimmed).unwrap();
    EXPECT_NEAR(
        GCPnts_AbscissaPoint::length_with_tolerance(&anAdaptor, precision::confusion()).unwrap(),
        1664.3732976598988,
        2.0e-5,
    );
    let aCurve: &dyn GeomCurve = aProjectedTrimmed.get().unwrap();
    EXPECT_NEAR(aCurve.first_parameter(), -5.23179933356147, 1.0e-7);
    EXPECT_NEAR(aCurve.last_parameter(), 4.76820064934972, 1.0e-7);
}
