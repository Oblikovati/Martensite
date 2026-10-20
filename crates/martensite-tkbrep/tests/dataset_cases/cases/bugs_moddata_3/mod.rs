//! `bugs_moddata_3` case bodies whose
//! inputs are in the public dataset.
mod projections;
pub use projections::*;

use crate::support::{
    analysis::{gbounding2d, mk2dcurve_by_index, splitc12d, tocanon_curve, trinfo},
    checks::checkreal,
    commands::{mkcurve, mksurface, offset_surface, tclean},
    printed::g17,
    restore::{binrestore, restore_as},
    session::{Session, ViewStep, checkview_display, set},
};
use martensite_tkernel::{ncollection::Array1, precision, standard::Handle};
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dCurve as _, Adaptor3dSurface as _},
    geom::{GeomBSplineSurface, GeomCurve},
    geom_adaptor::{GeomAdaptorCurve, GeomAdaptorSurface},
};
use martensite_tkmath::geom_abs::Shape as Continuity;

/// The surface is neither C1 in U nor C1 in V
/// (`QCOMPARE(aSurf->IsCNu(1), false)`, the same in V, must print `OK`).
pub fn bug23683(s: &mut dyn Session) {
    let result = restore_as(s, "bug23683_a_31_draw", "result").surface();
    let surface = result.get().unwrap();
    assert!(
        !surface.is_cn_u(1).unwrap(),
        "Error : ucontinuity is incorrect"
    );
    assert!(
        !surface.is_cn_v(1).unwrap(),
        "Error : vcontinuity is incorrect"
    );
    checkview_display(s, Some("result"), "bug23683");
}

/// GeomAdaptor_Surface::D1 at (0, -6.337359082e-009) of the face
/// surface must not raise. The checkview finds no `result` variable.
pub fn bug23945(s: &mut dyn Session) {
    let f = restore_as(s, "bug23945_f.brep", "f").shape();
    let surface = set(s, "s", mksurface(&f));
    let adaptor = GeomAdaptorSurface::with_surface(&surface).unwrap();
    adaptor
        .eval_d1(0., -6.337359082e-009)
        .expect("GeomAdaptor_Surface::D1");
    checkview_display(s, None, "bug23945");
}

/// Geom_OffsetSurface on the periodic B-spline, then on it made U-non-periodic.
pub fn bug24621(s: &mut dyn Session) {
    let bs = restore_as(s, "bug24621_bs.draw", "bs").surface();
    set(
        s,
        "obs",
        offset_surface(&bs, -2.).expect("offset obs bs -2"),
    );
    let bspline = bs.cast::<GeomBSplineSurface>();
    bspline.get().unwrap().set_u_not_periodic().unwrap();
    let obs = offset_surface(&bs, -2.).expect("offset obs bs -2 after setunotperiodic");
    set(s, "obs", obs);
}

/// BndLib_Add2dCurve::Add box of the 2D curve, by `checkreal` (1e-4, 1e-4);
/// `restore` without a name names the curve after the file.
pub fn bug27890(s: &mut dyn Session) {
    let curve = restore_as(s, "bug27890_t1.draw", "bug27890_t1").curve2d();
    let [v1_x, v1_y, v2_x, v2_y] = gbounding2d(&curve);
    checkreal("v1_x", v1_x, -34.696157014910277, 1.0e-4, 0.0001);
    checkreal("v1_y", v1_y, -70.031982331514001, 1.0e-4, 0.0001);
    checkreal("v2_x", v2_x, -15.500706089827537, 1.0e-4, 0.0001);
    checkreal("v2_y", v2_y, -21.06204133417317, 1.0e-4, 0.0001);
    s.step(ViewStep::View2d([728, 20, 400, 400]));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug27890"));
}

/// `tclean -force` removes every triangulation and polygon on triangulation;
/// `trinfo` must then match each expected text.
pub fn bug28026(s: &mut dyn Session) {
    let b = restore_as(s, "bug28026_Ball.brep", "b").shape();
    tclean(&b, true);
    let output = trinfo(&b);
    for data in [
        "0 triangles",
        "0 nodes",
        "0 polygons on triangulation",
        "Maximal deflection 0",
    ] {
        assert!(
            output.contains(data),
            "Error: Not cleaned all trinfo in '{data}': {output}"
        );
    }
}

/// Converting the second p-curve of the edge from C0 to C1 must not raise.
pub fn bug28230(s: &mut dyn Session) {
    let e = restore_as(s, "bug28230_edge.brep", "e").shape();
    let pc = set(s, "pc", mk2dcurve_by_index(&e, 2));
    splitc12d(&pc).expect("splitc12d pc 0");
}

/// The C1 intervals of the adapted curves on the ranges.
pub fn bug29745(s: &mut dyn Session) {
    let bc = restore_as(s, "bug29745_1", "bc").curve();
    let bc1 = restore_as(s, "bug29745_2", "bc1").curve();
    let ref1 = "NbIntervals: 2; 0 0.5 1 ";
    assert_eq!(intervals_text(&bc1, 0., 1.), ref1, "OCC29745 Faulty");
    let ref2 = "NbIntervals: 3; 0.10000000000000001 0.5 1 1.1000000000000001 ";
    assert_eq!(intervals_text(&bc1, 0.1, 1.1), ref2, "OCC29745 Faulty");
    let ref3 = "NbIntervals: 3; 0 4.1887902047863896 6.2831853071795898 7 ";
    assert_eq!(intervals_text(&bc, 0., 7.), ref3, "OCC29745 Faulty");
}

fn intervals_text(curve: &Handle<dyn GeomCurve>, t1: f64, t2: f64) -> String {
    let adaptor = GeomAdaptorCurve::with_range(curve, t1, t2).unwrap();
    let count = adaptor.nb_intervals(Continuity::C1).unwrap();
    let mut bounds = Array1::with_bounds(1, count + 1).unwrap();
    adaptor.intervals(&mut bounds, Continuity::C1).unwrap();
    let values: String = bounds
        .as_slice()
        .iter()
        .map(|value| g17(*value) + " ")
        .collect();
    format!("NbIntervals: {count}; {values}")
}

/// 0031402: Geom_BSplineSurface::Segment to [0, 1] x [0.5, 2/3]; the bounds by
/// `checkreal` (0.001, 0.0001).
pub fn bug31402_1(s: &mut dyn Session) {
    let face = restore_as(s, "bug31402.brep", "face").shape();
    let surface = set(s, "s", mksurface(&face));
    let bspline = surface.cast::<GeomBSplineSurface>();
    let bspline = bspline.get().expect("segsur needs a B-spline surface");
    let (v1, v2) = (0.49999999999999988898, 0.66666666666666662966);
    let tolerance = precision::p_confusion();
    bspline
        .segment((0.0, 1.0), (v1, v2), (tolerance, tolerance))
        .unwrap();
    let (u1, u2, v1, v2) = surface.get().unwrap().bounds();
    checkreal("U1", u1, 0., 0.001, 0.0001);
    checkreal("U2", u2, 1., 0.001, 0.0001);
    checkreal("V1", v1, 0.5, 0.001, 0.0001);
    checkreal("V2", v2, 0.66666667, 0.001, 0.0001);
}

/// 0031437: the offset surface at the singular point (0, 1) is 2.5 from the basis.
pub fn bug31437(s: &mut dyn Session) {
    let surface = restore_as(s, "bug31437.draw", "s").surface();
    let dist = 2.5;
    let result = offset_surface(&surface, -dist).expect("offset result s -2.5");
    let result = set(s, "result", result);
    let p1 = surface.get().unwrap().d0(0., 1.).unwrap();
    let p2 = result.get().unwrap().d0(0., 1.).unwrap();
    let cur_dist2 = p1.square_distance(&p2);
    assert!(
        (cur_dist2 - dist * dist).abs() < 1.0e-7,
        "Error: offset is wrong"
    );
    checkview_display(s, Some("result"), "bug31437");
}

/// 0033304: `REQUIRED ALL: Conversion failed`: the B-spline is not converted (and no
/// floating point signal is raised).
pub fn bug33304(s: &mut dyn Session) {
    let a = set(s, "a", binrestore("bug33304_bspl_curv.bbrep"));
    let c = set(s, "c", mkcurve(&a));
    assert!(
        !tocanon_curve(&c, precision::confusion()),
        "Conversion failed expected"
    );
}
