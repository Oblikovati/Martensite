// SPDX-License-Identifier: GPL-2.0-only
//! Conic minimum classification corpus for #406.
use martensite_tkernel::precision;
use martensite_tkgeombase::extrema::{
    ExtPElC2d, GpPoint, GpVector, PointCurveExtrema, StationaryNature,
};
use martensite_tkmath::{
    elclib,
    gp::{Axis2d, Circle2, Direction2, Ellipse2, Hyperbola2, Parabola2, Point2},
};

fn axis(origin: [f64; 2], direction: [f64; 2]) -> Axis2d {
    Axis2d::new(
        Point2::new(origin[0], origin[1]),
        Direction2::new(direction[0], direction[1]).unwrap(),
    )
}
fn report<P: GpPoint>(
    name: &str,
    result: &PointCurveExtrema<P>,
    query: P,
    derivative: impl Fn(f64) -> (P, P::Vector, P::Vector),
) -> usize {
    let mut mismatches = 0;
    for index in 1..=result.nb_ext().unwrap() {
        let parameter = result.point(index).unwrap().parameter();
        let (position, tangent, second) = derivative(parameter);
        let variation = tangent.dot_with(&tangent) + query.vector_to(&position).dot_with(&second);
        assert!(
            variation != 0.,
            "case {name}: expected a nondegenerate stationary point"
        );
        mismatches += usize::from(result.is_min(index).unwrap() != (variation > 0.));
    }
    mismatches
}
fn assert_minima_count<P: GpPoint>(roots: &PointCurveExtrema<P>, expected: usize) {
    let count = (1..=roots.nb_ext().unwrap())
        .filter(|&index| roots.is_min(index).unwrap())
        .count();
    assert_eq!(count, expected);
}

fn circle() -> usize {
    let shape = Circle2::from_x_axis(axis([0.; 2], [1., 0.]), 1., true).unwrap();
    let query = Point2::new(2., 0.);
    let result = ExtPElC2d::with_circle(
        query,
        shape,
        precision::confusion(),
        0.,
        std::f64::consts::TAU,
    )
    .unwrap();
    assert_minima_count(&result, 1);
    report("circle", &result, query, |parameter| {
        elclib::d2(parameter, &shape)
    })
}
fn ellipse() -> usize {
    let shape =
        Ellipse2::from_x_axis(axis([2.7574, -6.9211], [0.6, 0.8]), 0.75793, 0.38063, true).unwrap();
    let query = Point2::new(2.9111111111111105, -6.288888888888888);
    let result = ExtPElC2d::with_ellipse(
        query,
        shape,
        precision::confusion(),
        0.,
        std::f64::consts::TAU,
    )
    .unwrap();
    assert_minima_count(&result, 1);
    report("rotated-ellipse", &result, query, |parameter| {
        elclib::d2(parameter, &shape)
    })
}
fn hyperbola() -> usize {
    let shape = Hyperbola2::from_x_axis(axis([0.; 2], [1., 0.]), 2., 1., true).unwrap();
    let query = Point2::new(3., 0.);
    let result = ExtPElC2d::with_hyperbola(query, shape, precision::confusion(), -2., 2.).unwrap();
    assert_minima_count(&result, 2);
    report("hyperbola", &result, query, |parameter| {
        elclib::d2(parameter, &shape)
    })
}
fn parabola() -> usize {
    let shape = Parabola2::from_mirror_axis(axis([0.; 2], [1., 0.]), 1., true).unwrap();
    let query = Point2::new(3., 0.);
    let result = ExtPElC2d::with_parabola(query, shape, precision::confusion(), -4., 4.).unwrap();
    assert_minima_count(&result, 2);
    report("parabola", &result, query, |parameter| {
        elclib::d2(parameter, &shape)
    })
}
#[test]
fn conic_stationary_roots_have_geometric_minimum_flags() {
    let mismatches = circle() + ellipse() + hyperbola() + parabola();
    assert_eq!(
        mismatches, 0,
        "SDK extrema flags: expected every nondegenerate root to have its geometric minimum classification"
    );
}

#[test]
fn generic_ellipse_projection_matches_occt_witness_and_preserves_replay_geometry() {
    use martensite_tkernel::standard::Handle;
    use martensite_tkg2d::{geom2d::Geom2dEllipse, geom2d_adaptor::Geom2dAdaptorCurve};
    use martensite_tkgeombase::extrema::{ExtPC2d, PointCurveClassification};
    let shape =
        Ellipse2::from_x_axis(axis([2.7574, -6.9211], [0.6, 0.8]), 0.75793, 0.38063, true).unwrap();
    let geometry = Handle::new(Geom2dEllipse::from_elips2d(shape));
    let adaptor = Geom2dAdaptorCurve::with_curve(&geometry.cast()).unwrap();
    let query = Point2::new(2.9111111111111105, -6.288888888888888);
    let corrected = ExtPC2d::with_curve(query, &adaptor, precision::confusion()).unwrap();
    let mut replay = ExtPC2d::new();
    replay.set_classification_policy(PointCurveClassification::LegacyV1);
    replay
        .initialize(&adaptor, 0., std::f64::consts::TAU, precision::confusion())
        .unwrap();
    replay.perform(query).unwrap();
    assert_ellipse_projection_witness(&corrected);
    assert_legacy_geometry_unchanged(&corrected, &replay);
}

fn assert_ellipse_projection_witness(corrected: &martensite_tkgeombase::extrema::ExtPC2d<'_>) {
    let minimum = (1..=corrected.nb_ext().unwrap())
        .find(|&index| corrected.is_min(index).unwrap())
        .unwrap();
    let foot = corrected.point(minimum).unwrap();
    assert!((foot.parameter() - 0.682_817_513_323_967_3).abs() < precision::p_confusion());
    assert!(
        foot.value()
            .distance(Point2::new(2.918064003780093, -6.306597278227931))
            < precision::confusion()
    );
    assert!(
        (corrected.square_distance(minimum).unwrap().sqrt() - 0.019024451883025126).abs()
            < precision::confusion()
    );
}

fn assert_legacy_geometry_unchanged(
    corrected: &martensite_tkgeombase::extrema::ExtPC2d<'_>,
    replay: &martensite_tkgeombase::extrema::ExtPC2d<'_>,
) {
    assert_eq!(corrected.nb_ext().unwrap(), replay.nb_ext().unwrap());
    for index in 1..=corrected.nb_ext().unwrap() {
        assert_eq!(
            corrected.point(index).unwrap(),
            replay.point(index).unwrap()
        );
        assert_eq!(
            corrected.square_distance(index).unwrap().to_bits(),
            replay.square_distance(index).unwrap().to_bits()
        );
        assert!(!replay.is_min(index).unwrap());
    }
}

#[test]
fn parabola_degenerate_and_unresolved_second_variation_are_named_declines() {
    let shape = Parabola2::from_mirror_axis(axis([0.; 2], [1., 0.]), 1., true).unwrap();
    for x in [2., 2. + 1.0e-12, 2. - 1.0e-12] {
        let roots =
            ExtPElC2d::with_parabola(Point2::new(x, 0.), shape, precision::confusion(), -4., 4.)
                .unwrap();
        assert!(roots.is_done());
        for index in 1..=roots.nb_ext().unwrap() {
            assert!(matches!(
                roots.classification(index).unwrap(),
                StationaryNature::Unresolved(_)
            ));
            assert_eq!(
                roots.is_min(index).unwrap_err().exception_type(),
                "Extrema_UncertifiedStationaryRoot"
            );
            assert!(roots.square_distance(index).unwrap().is_finite());
        }
    }
}

#[test]
fn reflected_parameter_frames_and_near_degenerate_roots_keep_classification() {
    for sense in [true, false] {
        for x in [2. - 1.0e-5, 2. + 1.0e-5, 3.] {
            let shape = Parabola2::from_mirror_axis(axis([0.; 2], [1., 0.]), 1., sense).unwrap();
            let query = Point2::new(x, 0.);
            let first =
                ExtPElC2d::with_parabola(query, shape, precision::confusion(), -4., 4.).unwrap();
            assert_eq!(
                report("reflected-parabola", &first, query, |u| elclib::d2(
                    u, &shape
                )),
                0
            );
            for _ in 0..8 {
                let repeat =
                    ExtPElC2d::with_parabola(query, shape, precision::confusion(), -4., 4.)
                        .unwrap();
                assert_eq!(format!("{first:?}"), format!("{repeat:?}"));
            }
        }
    }
}

#[test]
fn lifted_circle_roots_have_expected_minima() {
    use martensite_tkgeombase::extrema::ExtPElC;
    use martensite_tkmath::gp::{Axis2, Point3};
    let query = Point3::new(3., 0., 0.7);
    let planar_circle = Circle2::from_x_axis(axis([0.; 2], [1., 0.]), 1., true).unwrap();
    let circle = elclib::to3d_circle(Axis2::default(), planar_circle).unwrap();
    let roots = ExtPElC::with_circle(
        query,
        circle,
        precision::confusion(),
        0.,
        std::f64::consts::TAU,
    )
    .unwrap();
    assert_eq!(
        report("3d-circle", &roots, query, |u| elclib::d2(u, &circle)),
        0
    );
    assert_minima_count(&roots, 1);
}

#[test]
fn lifted_ellipse_roots_have_expected_minima() {
    use martensite_tkgeombase::extrema::ExtPElC;
    use martensite_tkmath::gp::{Axis2, Point3};
    let query = Point3::new(3., 0., 0.7);
    let planar_ellipse = Ellipse2::from_x_axis(axis([0.; 2], [1., 0.]), 2., 1., true).unwrap();
    let ellipse = elclib::to3d_ellipse(Axis2::default(), planar_ellipse).unwrap();
    let roots = ExtPElC::with_ellipse(
        query,
        ellipse,
        precision::confusion(),
        0.,
        std::f64::consts::TAU,
    )
    .unwrap();
    assert_eq!(
        report("3d-ellipse", &roots, query, |u| elclib::d2(u, &ellipse)),
        0
    );
    assert_minima_count(&roots, 1);
}

#[test]
fn lifted_hyperbola_roots_have_expected_minima() {
    use martensite_tkgeombase::extrema::ExtPElC;
    use martensite_tkmath::gp::{Axis2, Point3};
    let query = Point3::new(3., 0., 0.7);
    let planar_hyperbola = Hyperbola2::from_x_axis(axis([0.; 2], [1., 0.]), 2., 1., true).unwrap();
    let hyperbola = elclib::to3d_hyperbola(Axis2::default(), planar_hyperbola).unwrap();
    let roots = ExtPElC::with_hyperbola(query, hyperbola, precision::confusion(), -2., 2.).unwrap();
    assert_eq!(
        report("3d-hyperbola", &roots, query, |u| elclib::d2(u, &hyperbola)),
        0
    );
    assert_minima_count(&roots, 2);
}

#[test]
fn lifted_parabola_roots_have_expected_minima() {
    use martensite_tkgeombase::extrema::ExtPElC;
    use martensite_tkmath::gp::{Axis2, Point3};
    let query = Point3::new(3., 0., 0.7);
    let planar_parabola = Parabola2::from_mirror_axis(axis([0.; 2], [1., 0.]), 1., true).unwrap();
    let parabola = elclib::to3d_parabola(Axis2::default(), planar_parabola).unwrap();
    let roots = ExtPElC::with_parabola(query, parabola, precision::confusion(), -4., 4.).unwrap();
    assert_eq!(
        report("3d-parabola", &roots, query, |u| elclib::d2(u, &parabola)),
        0
    );
    assert_minima_count(&roots, 2);
}

#[test]
fn generated_ellipses_have_two_minima_under_parameter_reversal() {
    for sense in [true, false] {
        for major in [1. + 1.0e-5, 2., 100.] {
            let shape = Ellipse2::from_x_axis(axis([0.; 2], [0.6, 0.8]), major, 1., sense).unwrap();
            let roots = ExtPElC2d::with_ellipse(
                Point2::default(),
                shape,
                precision::confusion(),
                0.,
                std::f64::consts::TAU,
            )
            .unwrap();
            assert_eq!(roots.nb_ext().unwrap(), 4);
            assert_eq!(
                (1..=4)
                    .filter(|&index| roots.is_min(index).unwrap())
                    .count(),
                2
            );
            for index in 1..=4 {
                let squared = roots.square_distance(index).unwrap();
                let expected = if roots.is_min(index).unwrap() {
                    1.
                } else {
                    major * major
                };
                assert!((squared - expected).abs() < precision::confusion() * major);
            }
        }
    }
}

#[test]
fn generic_recompute_forwards_local_uncertainty_and_recovers() {
    use martensite_tkernel::standard::Handle;
    use martensite_tkg2d::{geom2d::Geom2dParabola, geom2d_adaptor::Geom2dAdaptorCurve};
    use martensite_tkgeombase::extrema::ExtPC2d;
    let shape = Parabola2::from_mirror_axis(axis([0.; 2], [1., 0.]), 1., true).unwrap();
    let geometry = Handle::new(Geom2dParabola::from_parab2d(shape));
    let adaptor = Geom2dAdaptorCurve::with_range(&geometry.cast(), -4., 4.).unwrap();
    let mut solver =
        ExtPC2d::with_curve(Point2::new(3., 0.), &adaptor, precision::confusion()).unwrap();
    assert!(solver.is_done());
    solver.perform(Point2::new(2., 0.)).unwrap();
    assert!(solver.is_done());
    assert_eq!(solver.nb_ext().unwrap(), 1);
    assert!(matches!(
        solver.classification(1).unwrap(),
        StationaryNature::Unresolved(_)
    ));
    assert_eq!(
        solver.is_min(1).unwrap_err().exception_type(),
        "Extrema_UncertifiedStationaryRoot"
    );
    assert_eq!(solver.square_distance(1).unwrap(), 4.);
    solver.perform(Point2::new(3., 0.)).unwrap();
    for index in 1..=solver.nb_ext().unwrap() {
        assert!(solver.is_min(index).is_ok());
    }
}

#[test]
fn one_uncertain_root_does_not_discard_other_ellipse_roots() {
    let shape = Ellipse2::from_x_axis(axis([0.; 2], [1., 0.]), 2., 1., true).unwrap();
    let roots = ExtPElC2d::with_ellipse(
        Point2::new(1.5, 0.),
        shape,
        precision::confusion(),
        0.,
        std::f64::consts::TAU,
    )
    .unwrap();
    let mut unresolved = 0;
    let mut certified = 0;
    for index in 1..=roots.nb_ext().unwrap() {
        match roots.classification(index).unwrap() {
            StationaryNature::Unresolved(_) => {
                unresolved += 1;
                assert!(roots.is_min(index).is_err());
            }
            _ => {
                certified += 1;
                assert!(roots.is_min(index).is_ok());
            }
        }
        assert!(roots.square_distance(index).unwrap().is_finite());
    }
    assert!(unresolved > 0 && certified > 0);
}

#[test]
fn circle_and_hyperbola_minima_survive_parameter_reversal() {
    for sense in [true, false] {
        assert_reversed_circle(sense);
        assert_reversed_hyperbola(sense);
    }
}

fn assert_reversed_circle(sense: bool) {
    let shape = Circle2::from_x_axis(axis([0.; 2], [0.6, 0.8]), 1., sense).unwrap();
    let query = Point2::new(1.2, 1.6);
    let roots = ExtPElC2d::with_circle(
        query,
        shape,
        precision::confusion(),
        0.,
        std::f64::consts::TAU,
    )
    .unwrap();
    assert_minima_count(&roots, 1);
    assert_eq!(
        report("reversed-circle", &roots, query, |u| elclib::d2(u, &shape)),
        0
    );
}

fn assert_reversed_hyperbola(sense: bool) {
    let shape = Hyperbola2::from_x_axis(axis([0.; 2], [0.6, 0.8]), 2., 1., sense).unwrap();
    let query = Point2::new(1.8, 2.4);
    let roots = ExtPElC2d::with_hyperbola(query, shape, precision::confusion(), -2., 2.).unwrap();
    assert_minima_count(&roots, 2);
    assert_eq!(
        report("reversed-hyperbola", &roots, query, |u| elclib::d2(
            u, &shape
        )),
        0
    );
}
