//! Eval hyperboloid surface construction and evaluation contracts.
#![allow(non_snake_case)]
#[path = "common/mod.rs"]
mod common;
#[path = "geom_eval_surface_support/mod.rs"]
mod support;
use common::{EXPECT_NEAR, EXPECT_THROW};
use martensite_tkernel::precision;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkg3d::geom::{
    GeomGeometry, GeomSurface, GeomSurfaceD1, GeomSurfaceD2, GeomSurfaceD3,
};
use martensite_tkg3d::geom_eval::{GeomEvalHyperboloidSurface, HyperboloidSheetMode};
use martensite_tkmath::gp::{Axis3, Point3, Transform3, Vector3};
use std::f64::consts::PI;
use support::{EXPECT_VEC_NEAR, point_difference, quadric_residual, vector_difference};

const THE_FD_TOL: f64 = 1e-5;
const THE_FD_TOL_D3: f64 = 1e-4;

// `GeomEvalHyperboloidSurface(Axis3(), R1, R2[, Mode])`.
fn hyperboloid(r1: f64, r2: f64, mode: HyperboloidSheetMode) -> GeomEvalHyperboloidSurface {
    GeomEvalHyperboloidSurface::new(Axis3::default(), r1, r2, mode).unwrap()
}

const ONE: HyperboloidSheetMode = HyperboloidSheetMode::OneSheet;
const TWO: HyperboloidSheetMode = HyperboloidSheetMode::TwoSheets;

#[test]
fn hyperboloid_surface_construction_valid_params_one_sheet() {
    let aSurf = hyperboloid(2.0, 3.0, HyperboloidSheetMode::default());
    EXPECT_NEAR(aSurf.r1(), 2.0, precision::confusion());
    EXPECT_NEAR(aSurf.r2(), 3.0, precision::confusion());
    assert!(aSurf.mode() == ONE);
}

#[test]
fn hyperboloid_surface_construction_valid_params_two_sheets() {
    let aSurf = hyperboloid(2.0, 3.0, TWO);
    assert!(aSurf.mode() == TWO);
}

#[test]
fn hyperboloid_surface_construction_invalid_radii_throws() {
    let anAx3 = Axis3::default();
    let build = |r1: f64, r2: f64| GeomEvalHyperboloidSurface::new(anAx3, r1, r2, ONE);
    EXPECT_THROW(build(0.0, 1.0), "Standard_ConstructionError");
    EXPECT_THROW(build(1.0, 0.0), "Standard_ConstructionError");
    EXPECT_THROW(build(-1.0, 1.0), "Standard_ConstructionError");
    EXPECT_THROW(build(1.0, -1.0), "Standard_ConstructionError");
    EXPECT_THROW(build(-2.0, -3.0), "Standard_ConstructionError");
}

#[test]
fn hyperboloid_surface_setters_zero_radii_throw() {
    let aSurf = hyperboloid(2.0, 3.0, ONE);
    EXPECT_THROW(aSurf.set_r1(0.0), "Standard_ConstructionError");
    EXPECT_THROW(aSurf.set_r2(0.0), "Standard_ConstructionError");
}

#[test]
fn hyperboloid_surface_eval_d0_one_sheet_origin() {
    let (aR1, aR2) = (2.0, 3.0);
    let aP: Point3 = hyperboloid(aR1, aR2, ONE).eval_d0(0.0, 0.0).unwrap();
    EXPECT_NEAR(aP.x(), aR1, precision::confusion());
    EXPECT_NEAR(aP.y(), 0.0, precision::confusion());
    EXPECT_NEAR(aP.z(), 0.0, precision::confusion());
}

#[test]
fn hyperboloid_surface_eval_d0_one_sheet_half_pi() {
    let (aR1, aR2) = (2.0, 3.0);
    let aP: Point3 = hyperboloid(aR1, aR2, ONE).eval_d0(PI / 2.0, 0.0).unwrap();
    EXPECT_NEAR(aP.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aP.y(), aR1, precision::confusion());
    EXPECT_NEAR(aP.z(), 0.0, precision::confusion());
}

#[test]
fn hyperboloid_surface_eval_d0_two_sheets_origin() {
    let (aR1, aR2) = (2.0, 3.0);
    let aP: Point3 = hyperboloid(aR1, aR2, TWO).eval_d0(0.0, 0.0).unwrap();
    EXPECT_NEAR(aP.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aP.y(), 0.0, precision::confusion());
    EXPECT_NEAR(aP.z(), aR1, precision::confusion());
}

#[test]
fn hyperboloid_surface_eval_d1_consistent_with_d0() {
    let aSurf = hyperboloid(2.0, 3.0, ONE);
    let (aU, aV) = (1.0, 0.5);
    let h: f64 = precision::confusion();
    let aD1: GeomSurfaceD1 = aSurf.eval_d1(aU, aV).unwrap();
    let aPu1 = aSurf.eval_d0(aU + h, aV).unwrap();
    let aPu2 = aSurf.eval_d0(aU - h, aV).unwrap();
    let aPv1 = aSurf.eval_d0(aU, aV + h).unwrap();
    let aPv2 = aSurf.eval_d0(aU, aV - h).unwrap();
    EXPECT_VEC_NEAR(aD1.d1u, point_difference(aPu1, aPu2), THE_FD_TOL);
    EXPECT_VEC_NEAR(aD1.d1v, point_difference(aPv1, aPv2), THE_FD_TOL);
}

#[test]
fn hyperboloid_surface_bounds_periodicity() {
    let aSurf = hyperboloid(1.0, 1.0, ONE);
    let (aU1, aU2, _aV1, _aV2) = aSurf.bounds();
    EXPECT_NEAR(aU1, 0.0, precision::confusion());
    EXPECT_NEAR(aU2, 2.0 * PI, precision::confusion());
    assert!(aSurf.is_u_periodic());
    assert!(aSurf.is_u_closed().unwrap());
    assert!(!aSurf.is_v_periodic());
    assert!(!aSurf.is_v_closed().unwrap());
}

#[test]
fn hyperboloid_surface_iso_not_implemented() {
    let aSurf = hyperboloid(2.0, 3.0, ONE);
    EXPECT_THROW(aSurf.u_iso(0.5), "Standard_NotImplemented");
    EXPECT_THROW(aSurf.v_iso(0.5), "Standard_NotImplemented");
}

#[test]
fn hyperboloid_surface_reverse_not_implemented() {
    let aSurf = hyperboloid(2.0, 3.0, ONE);
    EXPECT_THROW(aSurf.u_reverse(), "Standard_NotImplemented");
    EXPECT_THROW(aSurf.v_reverse(), "Standard_NotImplemented");
    EXPECT_THROW(aSurf.u_reversed_parameter(0.5), "Standard_NotImplemented");
    EXPECT_THROW(aSurf.v_reversed_parameter(0.5), "Standard_NotImplemented");
}

// The residual of the implicit equation on the u in [0, 6), v in [-1, 1] grid.
fn expect_coefficients_satisfied(aSurf: &GeomEvalHyperboloidSurface) {
    let aCoefficients: [f64; 10] = aSurf.coefficients();
    let mut u: f64 = 0.0;
    while u < 6.0 {
        let mut v: f64 = -1.0;
        while v <= 1.0 {
            let aP = aSurf.eval_d0(u, v).unwrap();
            let aVal: f64 = quadric_residual(aCoefficients, aP);
            EXPECT_NEAR(aVal, 0.0, precision::intersection());
            v += 0.5;
        }
        u += 1.0;
    }
}

#[test]
fn hyperboloid_surface_coefficients_one_sheet_satisfied_at_eval_points() {
    expect_coefficients_satisfied(&hyperboloid(2.0, 3.0, ONE));
}

#[test]
fn hyperboloid_surface_coefficients_two_sheets_satisfied_at_eval_points() {
    expect_coefficients_satisfied(&hyperboloid(2.0, 3.0, TWO));
}

#[test]
fn hyperboloid_surface_transform_not_implemented() {
    let aSurf = hyperboloid(2.0, 3.0, ONE);
    let mut aTrsf = Transform3::default();
    aTrsf.set_translation(Vector3::new(1.0, 2.0, 3.0));
    EXPECT_THROW(aSurf.transform(aTrsf), "Standard_NotImplemented");
    EXPECT_THROW(aSurf.transformed(aTrsf), "Standard_NotImplemented");
}

#[test]
fn hyperboloid_surface_copy_independent() {
    let aSurf = hyperboloid(2.0, 3.0, ONE);
    let aCopy = aSurf.copy().unwrap();
    assert!(!aCopy.is_null());
    let aCopySurf = aCopy.cast::<GeomEvalHyperboloidSurface>();
    assert!(!aCopySurf.is_null());
    EXPECT_NEAR(aCopySurf.r1(), 2.0, precision::confusion());
    EXPECT_NEAR(aCopySurf.r2(), 3.0, precision::confusion());
}

#[test]
fn hyperboloid_surface_dump_json_no_crash() {
    let aSurf = hyperboloid(2.0, 3.0, ONE);
    let mut aSS = DumpStream::default();
    assert!(aSurf.dump_json(&mut aSS, -1).is_ok());
}

// D2 against central differences of D1 at (aU, aV).
fn expect_d2_consistent(aSurf: &GeomEvalHyperboloidSurface, aU: f64, aV: f64) {
    let h: f64 = precision::confusion();
    let aD2: GeomSurfaceD2 = aSurf.eval_d2(aU, aV).unwrap();
    let aD1Uf = aSurf.eval_d1(aU + h, aV).unwrap();
    let aD1Ub = aSurf.eval_d1(aU - h, aV).unwrap();
    let aD1Vf = aSurf.eval_d1(aU, aV + h).unwrap();
    let aD1Vb = aSurf.eval_d1(aU, aV - h).unwrap();
    EXPECT_VEC_NEAR(aD2.d2u, vector_difference(aD1Uf.d1u, aD1Ub.d1u), THE_FD_TOL);
    EXPECT_VEC_NEAR(aD2.d2v, vector_difference(aD1Vf.d1v, aD1Vb.d1v), THE_FD_TOL);
    EXPECT_VEC_NEAR(
        aD2.d2uv,
        vector_difference(aD1Vf.d1u, aD1Vb.d1u),
        THE_FD_TOL,
    );
}

#[test]
fn hyperboloid_surface_eval_d2_consistent_with_d1() {
    expect_d2_consistent(&hyperboloid(2.0, 3.0, ONE), 0.7, 0.5);
}

#[test]
fn hyperboloid_surface_eval_d3_consistent_with_d2() {
    let aSurf = hyperboloid(2.0, 3.0, ONE);
    let (aU, aV) = (1.0, 0.3);
    let h: f64 = precision::confusion();
    let aD3: GeomSurfaceD3 = aSurf.eval_d3(aU, aV).unwrap();
    let aD2Uf = aSurf.eval_d2(aU + h, aV).unwrap();
    let aD2Ub = aSurf.eval_d2(aU - h, aV).unwrap();
    let aD2Vf = aSurf.eval_d2(aU, aV + h).unwrap();
    let aD2Vb = aSurf.eval_d2(aU, aV - h).unwrap();
    EXPECT_VEC_NEAR(
        aD3.d3u,
        vector_difference(aD2Uf.d2u, aD2Ub.d2u),
        THE_FD_TOL_D3,
    );
    EXPECT_VEC_NEAR(
        aD3.d3v,
        vector_difference(aD2Vf.d2v, aD2Vb.d2v),
        THE_FD_TOL_D3,
    );
    EXPECT_VEC_NEAR(
        aD3.d3uuv,
        vector_difference(aD2Vf.d2u, aD2Vb.d2u),
        THE_FD_TOL_D3,
    );
    EXPECT_VEC_NEAR(
        aD3.d3uvv,
        vector_difference(aD2Uf.d2v, aD2Ub.d2v),
        THE_FD_TOL_D3,
    );
}

#[test]
fn hyperboloid_surface_eval_d2_two_sheets_consistent_with_d1() {
    expect_d2_consistent(&hyperboloid(2.0, 3.0, TWO), 0.7, 0.5);
}
