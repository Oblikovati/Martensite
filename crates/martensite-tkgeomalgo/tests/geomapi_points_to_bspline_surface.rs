//! Tests of `GeomAPI_PointsToBSplineSurface`.
#![allow(non_snake_case)]
use martensite_tkernel::ncollection::Array2;
use martensite_tkernel::precision;
use martensite_tkgeomalgo::geom_api::PointsToBSplineSurface;
use martensite_tkmath::geom_abs::Shape;

#[test]
fn GeomAPI_PointsToBSplineSurface_Test__FailedDegenerateRebuildResetsDoneState() {
    let mut aValidZPoints: Array2<'static, f64> = Array2::with_bounds(1, 2, 1, 2).unwrap();
    aValidZPoints.set_value(1, 1, 0.0).unwrap();
    aValidZPoints.set_value(1, 2, 1.0).unwrap();
    aValidZPoints.set_value(2, 1, 1.0).unwrap();
    aValidZPoints.set_value(2, 2, 0.0).unwrap();

    let mut anApprox = PointsToBSplineSurface::new();
    // EXPECT_NO_THROW
    assert!(
        anApprox
            .init_heights(
                &aValidZPoints,
                (0.0, 1.0, 0.0, 1.0),
                (3, 5, Shape::C2, precision::confusion())
            )
            .is_ok()
    );
    assert!(anApprox.is_done());

    let mut aZPoints: Array2<'static, f64> = Array2::with_bounds(1, 3, 1, 1).unwrap();
    aZPoints.set_value(1, 1, 0.0).unwrap();
    aZPoints.set_value(2, 1, 1.0).unwrap();
    aZPoints.set_value(3, 1, 0.0).unwrap();

    // EXPECT_THROW(..., StdFail_NotDone)
    let thrown = anApprox
        .init_heights(
            &aZPoints,
            (0.0, 1.0, 0.0, 0.0),
            (3, 5, Shape::C2, precision::confusion()),
        )
        .unwrap_err();
    assert_eq!(thrown.exception_type(), "StdFail_NotDone");
    assert!(!anApprox.is_done());
}
