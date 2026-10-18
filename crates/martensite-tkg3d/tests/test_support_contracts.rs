//! Authored boundary checks preserve the geometry test support contracts.
#[path = "common/mod.rs"]
mod common;
use common::{CreateUniformParams, EXPECT_DOUBLE_EQ, EXPECT_NEAR, EXPECT_THROW};
use martensite_tkernel::standard::Failure;

#[test]
fn owned_ulp_assertion_keeps_signed_zero_and_four_ulp_boundary() {
    EXPECT_DOUBLE_EQ(0., -0.);
    EXPECT_DOUBLE_EQ(f64::from_bits(1), f64::from_bits((1 << 63) | 1));
    EXPECT_DOUBLE_EQ(1., f64::from_bits(1_f64.to_bits() + 4));
    assert!(
        std::panic::catch_unwind(|| EXPECT_DOUBLE_EQ(1., f64::from_bits(1_f64.to_bits() + 5)))
            .is_err()
    );
    assert!(std::panic::catch_unwind(|| EXPECT_DOUBLE_EQ(f64::NAN, f64::NAN)).is_err());
}

#[test]
fn owned_absolute_near_assertion_keeps_inclusive_boundary() {
    EXPECT_NEAR(1., 1.25, 0.25);
    assert!(std::panic::catch_unwind(|| EXPECT_NEAR(1., 1.5, 0.25)).is_err());
    assert!(std::panic::catch_unwind(|| EXPECT_NEAR(f64::NAN, 1., 0.25)).is_err());
}

#[test]
fn owned_exception_assertion_requires_the_exact_type_and_a_failure() {
    EXPECT_THROW::<()>(
        Err(Failure::out_of_range("index=2; expected 0..1")),
        "Standard_OutOfRange",
    );
    assert!(std::panic::catch_unwind(|| EXPECT_THROW(Ok(()), "Standard_OutOfRange")).is_err());
    assert!(
        std::panic::catch_unwind(|| EXPECT_THROW::<()>(
            Err(Failure::out_of_range("index=2; expected 0..1")),
            "Standard_DomainError"
        ))
        .is_err()
    );
}

#[test]
fn owned_uniform_parameters_keep_one_based_bounds_and_exact_dyadic_values() {
    let parameters = CreateUniformParams(-2., 2., 5);
    assert_eq!(parameters.lower(), 1);
    assert_eq!(parameters.upper(), 5);
    assert_eq!(parameters.as_slice(), [-2., -1., 0., 1., 2.]);
}
