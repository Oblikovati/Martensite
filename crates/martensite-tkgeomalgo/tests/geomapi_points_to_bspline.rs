//! Tests of `GeomAPI_PointsToBSpline`.
#![allow(non_snake_case)]
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkgeomalgo::geom_api::PointsToBSpline;
use martensite_tkmath::geom_abs::Shape;
use martensite_tkmath::gp::Point3;

#[test]
fn GeomAPI_PointsToBSpline_Test__DegenerateExplicitParametersResetDoneState() {
    let mut aPoints: Array1<'static, Point3> = Array1::with_bounds(1, 3).unwrap();
    aPoints[1] = Point3::new(0.0, 0.0, 0.0);
    aPoints[2] = Point3::new(1.0, 0.5, 0.0);
    aPoints[3] = Point3::new(2.0, 0.0, 0.0);

    let mut aParameters: Array1<'static, f64> = Array1::with_bounds(1, 3).unwrap();
    aParameters[1] = 2.0;
    aParameters[2] = 2.0;
    aParameters[3] = 2.0;

    let mut anApprox =
        PointsToBSpline::with_points(&aPoints, 3, 5, Shape::C2, precision::confusion()).unwrap();
    assert!(anApprox.is_done());

    // EXPECT_NO_THROW
    assert!(
        anApprox
            .init_parameters(
                &aPoints,
                &aParameters,
                3,
                5,
                Shape::C2,
                precision::confusion()
            )
            .is_ok()
    );
    assert!(!anApprox.is_done());
}
