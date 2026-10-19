//! Package-local assertion and geometric fixture contracts with independent expectations.
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use common::{CreateUniformParams, EXPECT_DOUBLE_EQ, EXPECT_NEAR, EXPECT_THROW, modeling_circle};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Point2;

#[test]
fn double_equality_accepts_four_ulps_and_signed_zero() {
    EXPECT_DOUBLE_EQ(1.0, f64::from_bits(1.0f64.to_bits() + 4));
    EXPECT_DOUBLE_EQ(0.0, -0.0);
}
#[test]
fn double_equality_rejects_five_ulps_and_nan() {
    assert!(
        std::panic::catch_unwind(|| EXPECT_DOUBLE_EQ(1.0, f64::from_bits(1.0f64.to_bits() + 5)))
            .is_err()
    );
    assert!(std::panic::catch_unwind(|| EXPECT_DOUBLE_EQ(f64::NAN, f64::NAN)).is_err());
}
#[test]
fn near_preserves_boundary_and_rejects_outside_and_nan() {
    EXPECT_NEAR(1.0, 1.5, 0.5);
    assert!(std::panic::catch_unwind(|| EXPECT_NEAR(1.0, 1.5, 0.25)).is_err());
    assert!(std::panic::catch_unwind(|| EXPECT_NEAR(f64::NAN, 0.0, 1.0)).is_err());
}
#[test]
fn typed_exception_assertion_preserves_exact_type() {
    EXPECT_THROW::<()>(Err(Failure::out_of_range("fixture")), "Standard_OutOfRange");
    assert!(
        std::panic::catch_unwind(|| EXPECT_THROW::<()>(
            Err(Failure::out_of_range("fixture")),
            "Standard_DomainError"
        ))
        .is_err()
    );
    assert!(std::panic::catch_unwind(|| EXPECT_THROW(Ok(()), "Standard_OutOfRange")).is_err());
}
#[test]
fn uniform_parameters_keep_one_based_closed_form_grid() {
    let grid = CreateUniformParams(1.0, 2.0, 3);
    assert_eq!((grid.lower(), grid.upper()), (1, 3));
    assert_eq!([grid[1], grid[2], grid[3]], [1.0, 1.5, 2.0]);
}
#[test]
fn planar_circle_fixture_uses_direct_cartesian_axes() {
    let circle = modeling_circle(Point2::new(3.0, -4.0), 2.0);
    let point = circle.get().unwrap().value(0.0).unwrap();
    assert_eq!((point.x(), point.y()), (5.0, -4.0));
}
