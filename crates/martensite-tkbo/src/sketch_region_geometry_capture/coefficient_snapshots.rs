// SPDX-License-Identifier: GPL-2.0-only
//! Bulk reads remain fresh and preserve the existing stored-coefficient capture contract (#449).
use super::*;
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg2d::geom2d::Geom2dBezierCurve;
use martensite_tkg3d::geom::GeomBezierCurve;
use martensite_tkmath::gp::{Point2, Point3};

fn spatial_source() -> Handle<GeomBezierCurve> {
    let points = [
        Point3::new(-0., 0., 2.),
        Point3::new(1., 2., 3.),
        Point3::new(2., 0., 4.),
    ];
    Handle::new(
        GeomBezierCurve::with_weights(
            &Array1::from_shared(&points, 1),
            &Array1::from_shared(&[1., 2., 1.], 1),
        )
        .unwrap(),
    )
}

#[test]
fn spatial_capture_retains_owned_bits_and_observes_later_atomic_edit() {
    let source = spatial_source();
    let first = coefficients(capture_space_curve(&source.clone().cast()).unwrap());
    source
        .set_pole_weighted(2, Point3::new(4., 5., 6.), 3.)
        .unwrap();
    let next = coefficients(capture_space_curve(&source.cast()).unwrap());
    assert_eq!(first.0[0][0].to_bits(), (-0.0_f64).to_bits());
    assert_eq!(first.0[1], [1., 2., 3.]);
    assert_eq!(first.1.unwrap(), vec![1., 2., 1.]);
    assert_eq!(next.0[1], [4., 5., 6.]);
    assert_eq!(next.1.unwrap(), vec![1., 3., 1.]);
}
#[test]
fn planar_capture_keeps_nonrational_weights_absent_and_remains_fresh() {
    let points = [
        Point2::new(-0., 0.),
        Point2::new(1., 2.),
        Point2::new(2., 0.),
    ];
    let source = Handle::new(Geom2dBezierCurve::new(&Array1::from_shared(&points, 1)).unwrap());
    let first = coefficients(capture_plane_curve(&source.clone().cast()).unwrap());
    source.set_pole(2, Point2::new(4., 5.)).unwrap();
    let next = coefficients(capture_plane_curve(&source.cast()).unwrap());
    assert_eq!(first.0[0][0].to_bits(), (-0.0_f64).to_bits());
    assert_eq!(first.0[1], [1., 2.]);
    assert_eq!(next.0[1], [4., 5.]);
    assert!(first.1.is_none() && next.1.is_none());
}
fn coefficients<const DIM: usize>(
    curve: CapturedMetricCurve<DIM>,
) -> (Vec<[f64; DIM]>, Option<Vec<f64>>) {
    let CapturedMetricCurve::Bezier { poles, weights } = curve else {
        panic!("expected captured Bezier coefficient values");
    };
    (poles, weights)
}
