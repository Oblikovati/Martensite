//! Independent closed forms and transport contracts supplement the frozen baselines.
use super::{common, print, run_request};
use martensite_tkernel::standard::Failure;

#[test]
fn binary16_unit_two_and_signed_zero_have_closed_form_encodings() {
    assert_eq!(
        run_request("sv_half 15360 2").unwrap(),
        "4607182418800017408 16384 \n"
    );
    assert_eq!(
        run_request("sv_half 32768 -0").unwrap(),
        "9223372036854775808 32768 \n"
    );
}

#[test]
fn rectangular_grid_rounds_to_its_exact_cartesian_lattice() {
    // Unit, axis-aligned lattice: (.4,1.6) maps to (0,2) for Compute and Hit.
    assert_eq!(
        run_request("sv_rgrid 1 1 0 0 0 0 0 0.4 1.6").unwrap(),
        "0 4611686018427387904 0 4611686018427387904 \n"
    );
}

#[test]
fn blank_and_unknown_requests_preserve_the_cli_contract() {
    assert_eq!(run_request(" ").unwrap(), "");
    assert_eq!(
        run_request("unknown_operation 0").unwrap_err(),
        "Unknown probe operation: unknown_operation"
    );
}

#[test]
fn outputs_are_operation_local_and_exceptions_keep_their_types() {
    let first = print::Output::default();
    let second = print::Output::default();
    print::int(&first, 7);
    print::flag(&second, true);
    print::attempt(&first, || {
        Err(Failure::with_message(
            "Standard_OutOfRange",
            "value 7 expected 0..1",
        ))
    });
    print::show(&second, Ok::<i32, Failure>(2), |n| print::int(&second, n));
    print::end(&first);
    print::end(&second);
    assert_eq!(first.finish(), "7 Standard_OutOfRange \n");
    assert_eq!(second.finish(), "1 2 \n");
}

#[test]
fn captured_text_keeps_hex_bytes_before_its_exception() {
    let output = print::Output::default();
    common::captured(&output, |text| {
        text.push_str("a\n");
        Err(Failure::with_message(
            "Standard_DomainError",
            "value -1 expected nonnegative",
        ))
    });
    assert_eq!(output.finish(), "t610a Standard_DomainError ");
}

#[test]
fn only_named_rendering_objects_are_masked_in_the_original_protocol() {
    let value = br#""ChartSize": {"x": 2, "inner": {"y": 3}}, "Keep": {"value": 4}"#;
    assert_eq!(
        super::params::masked(value, "ChartSize"),
        br#""ChartSize": ?, "Keep": {"value": 4}"#
    );
    assert_eq!(super::params::masked(value, "Missing"), value);
}
