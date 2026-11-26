// SPDX-License-Identifier: GPL-2.0-only
//! Regression corpus for point-to-spline and point-to-Bezier interior extrema (#462).
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dCurve};
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkgeombase::extrema::{ExtPC, ExtPC2d, PointCurveClassification};
use martensite_tkmath::gp::{Point2, Point3};

fn array1_from_slice<T: Clone + Default>(values: &[T]) -> Array1<'static, T> {
    let mut array = Array1::with_bounds(1, values.len() as i32).unwrap();
    for (index, value) in values.iter().enumerate() {
        array[index as i32 + 1] = value.clone();
    }
    array
}

fn parabola_bspline_2d() -> Handle<dyn Geom2dCurve> {
    let poles = array1_from_slice(&[
        Point2::new(0.0, 0.0625),
        Point2::new(0.5, -2.4375),
        Point2::new(1.0, 95.0625),
    ]);
    let knots = array1_from_slice(&[0.0, 1.0]);
    let mults = array1_from_slice(&[3, 3]);
    Handle::new(Geom2dBSplineCurve::new(&poles, &knots, &mults, 2, false).unwrap()).cast()
}

fn collect_extrema_2d(extrema: &ExtPC2d<'_>) -> Vec<(f64, f64, bool)> {
    let count = extrema.nb_ext().unwrap();
    let mut out = Vec::with_capacity(count as usize);
    for i in 1..=count {
        let u = extrema.point(i).unwrap().parameter();
        let dist = extrema.square_distance(i).unwrap().sqrt();
        let is_min = extrema.is_min(i).unwrap();
        out.push((u, dist, is_min));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn certified_ext_2d<'a>(query: Point2, adaptor: &'a Geom2dAdaptorCurve) -> ExtPC2d<'a> {
    let mut extrema = ExtPC2d::new();
    extrema.set_classification_policy(PointCurveClassification::CertifiedSplineV2);
    extrema.initialize(adaptor, 0.0, 1.0, 1e-7).unwrap();
    extrema.perform(query).unwrap();
    extrema
}

#[test]
fn issue_462_quadratic_bspline_finds_both_interior_minima_and_maximum() {
    let curve = parabola_bspline_2d();
    let adaptor = Geom2dAdaptorCurve::with_curve(&curve).unwrap();
    let query = Point2::new(0.025, 0.0054);
    let extrema = certified_ext_2d(query, &adaptor);
    assert!(extrema.is_done());
    let roots = collect_extrema_2d(&extrema);
    assert_eq!(roots.len(), 3, "expected 3 stationary roots, got {roots:?}");
    let expected_min_dist = 0.005385164807134504_f64;
    assert!((roots[0].0 - 0.023).abs() < 1e-10);
    assert!((roots[0].1 - expected_min_dist).abs() < 1e-12);
    assert!(roots[0].2, "u = 0.023 must be a minimum");
    assert!((roots[1].0 - 0.025).abs() < 1e-10);
    assert!((roots[1].1 - 0.0054).abs() < 1e-12);
    assert!(!roots[1].2, "u = 0.025 must be a maximum");
    assert!((roots[2].0 - 0.027).abs() < 1e-10);
    assert!((roots[2].1 - expected_min_dist).abs() < 1e-12);
    assert!(roots[2].2, "u = 0.027 must be a minimum");
}

#[test]
fn closer_interior_root_pairs_are_both_found() {
    let curve = parabola_bspline_2d();
    let adaptor = Geom2dAdaptorCurve::with_curve(&curve).unwrap();
    // For C(u) = (u, 100(u - 0.025)^2) and Q = (0.025, 0.005 + 100*delta^2):
    // minima sit at u = 0.025 +/- delta.
    for delta in [5e-4_f64, 1e-4_f64] {
        let query = Point2::new(0.025, 0.005 + 100.0 * delta * delta);
        let extrema = certified_ext_2d(query, &adaptor);
        assert!(extrema.is_done());
        let roots = collect_extrema_2d(&extrema);
        assert_eq!(
            roots.len(),
            3,
            "delta={delta}: expected 3 stationary roots, got {roots:?}"
        );
        assert!((roots[0].0 - (0.025 - delta)).abs() < 1e-9);
        assert!(roots[0].2);
        assert!((roots[1].0 - 0.025).abs() < 1e-9);
        assert!(!roots[1].2);
        assert!((roots[2].0 - (0.025 + delta)).abs() < 1e-9);
        assert!(roots[2].2);
    }
}

#[test]
fn reversed_orientation_and_split_knots_preserve_all_interior_minima() {
    // Reversed parameterization: u -> 1 - u places the stationary triple at 0.973, 0.975, 0.977.
    let reversed_poles = array1_from_slice(&[
        Point2::new(1.0, 95.0625),
        Point2::new(0.5, -2.4375),
        Point2::new(0.0, 0.0625),
    ]);
    let knots = array1_from_slice(&[0.0, 1.0]);
    let mults = array1_from_slice(&[3, 3]);
    let reversed: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dBSplineCurve::new(&reversed_poles, &knots, &mults, 2, false).unwrap())
            .cast();
    let adaptor = Geom2dAdaptorCurve::with_curve(&reversed).unwrap();
    let extrema = certified_ext_2d(Point2::new(0.025, 0.0054), &adaptor);
    let roots = collect_extrema_2d(&extrema);
    assert_eq!(roots.len(), 3);
    assert!((roots[0].0 - 0.973).abs() < 1e-10 && roots[0].2);
    assert!((roots[1].0 - 0.975).abs() < 1e-10 && !roots[1].2);
    assert!((roots[2].0 - 0.977).abs() < 1e-10 && roots[2].2);

    // Non-uniform knot split at u = 0.025 (two spans [0, 0.025] and [0.025, 1]).
    let split = Geom2dBSplineCurve::new(
        &array1_from_slice(&[
            Point2::new(0.0, 0.0625),
            Point2::new(0.5, -2.4375),
            Point2::new(1.0, 95.0625),
        ]),
        &knots,
        &mults,
        2,
        false,
    )
    .unwrap();
    split.insert_knot(0.025, 1, 1e-12).unwrap();
    let split_handle: Handle<dyn Geom2dCurve> = Handle::new(split).cast();
    let split_adaptor = Geom2dAdaptorCurve::with_curve(&split_handle).unwrap();
    let split_ext = certified_ext_2d(Point2::new(0.025, 0.0054), &split_adaptor);
    let split_roots = collect_extrema_2d(&split_ext);
    assert_eq!(split_roots.len(), 3, "split_roots={split_roots:?}");
    assert!(
        (split_roots[0].0 - 0.023).abs() < 1e-10 && split_roots[0].2,
        "split_roots={split_roots:?}"
    );
    assert!(
        (split_roots[1].0 - 0.025).abs() < 1e-10 && !split_roots[1].2,
        "split_roots={split_roots:?}"
    );
    assert!(
        (split_roots[2].0 - 0.027).abs() < 1e-10 && split_roots[2].2,
        "split_roots={split_roots:?}"
    );
}

#[test]
fn bezier_and_3d_bspline_curves_also_certify_all_interior_minima() {
    let poles_2d = array1_from_slice(&[
        Point2::new(0.0, 0.0625),
        Point2::new(0.5, -2.4375),
        Point2::new(1.0, 95.0625),
    ]);
    let bezier: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dBezierCurve::new(&poles_2d).unwrap()).cast();
    let bez_adaptor = Geom2dAdaptorCurve::with_curve(&bezier).unwrap();
    let bez_ext = certified_ext_2d(Point2::new(0.025, 0.0054), &bez_adaptor);
    let bez_roots = collect_extrema_2d(&bez_ext);
    assert_eq!(bez_roots.len(), 3);
    assert!(bez_roots[0].2 && !bez_roots[1].2 && bez_roots[2].2);

    let poles_3d = array1_from_slice(&[
        Point3::new(0.0, 0.0625, 1.0),
        Point3::new(0.5, -2.4375, 1.0),
        Point3::new(1.0, 95.0625, 1.0),
    ]);
    let knots = array1_from_slice(&[0.0, 1.0]);
    let mults = array1_from_slice(&[3, 3]);
    let curve_3d: Handle<dyn GeomCurve> =
        Handle::new(GeomBSplineCurve::new(&poles_3d, &knots, &mults, 2, false).unwrap()).cast();
    let adaptor_3d = GeomAdaptorCurve::with_curve(&curve_3d).unwrap();
    let mut ext_3d = ExtPC::new();
    ext_3d.set_classification_policy(PointCurveClassification::CertifiedSplineV2);
    ext_3d.initialize(&adaptor_3d, 0.0, 1.0, 1e-7).unwrap();
    ext_3d.perform(Point3::new(0.025, 0.0054, 1.0)).unwrap();
    assert_eq!(ext_3d.nb_ext().unwrap(), 3);
}

#[test]
fn legacy_v1_policy_remains_selectable_for_replay() {
    let curve = parabola_bspline_2d();
    let adaptor = Geom2dAdaptorCurve::with_curve(&curve).unwrap();
    let mut legacy = ExtPC2d::new();
    legacy.set_classification_policy(PointCurveClassification::LegacyV1);
    legacy.initialize(&adaptor, 0.0, 1.0, 1e-7).unwrap();
    legacy.perform(Point2::new(0.025, 0.0054)).unwrap();
    assert!(legacy.is_done());
    assert_eq!(legacy.nb_ext().unwrap(), 1);
}
