// SPDX-License-Identifier: GPL-2.0-only
use super::super::{super::MetricBezier, *};
use super::BoundedArithmeticFailure;

const AUTHORED: [[f64; 2]; 2] = [[0., 3.], [0., 4.]];
const STORED: [[f64; 3]; 2] = [[1., 0., 3.], [1., 0., 4.]];
const COMBINED_LEVEL_63_1_EXPECTATION: &[&str] = &[
    "source_level: 63",
    "additional_level: 1",
    "combined level=64",
];

fn query() -> CylindricalProfileQuery<'static> {
    CylindricalProfileQuery {
        authored: &AUTHORED,
        stored: MetricBezier {
            poles: &STORED,
            weights: None,
        },
        interval: CylindricalProfileInterval::new(0, 0).unwrap(),
        support_radius: 2.,
        stored_radius: 1.,
        angular_origin: 0.,
        spatial_limit: 1e-6,
        options: Default::default(),
    }
}

fn assert_diagnostic(error: &impl std::fmt::Display, fragments: &[&str]) {
    let message = error.to_string();
    for fragment in fragments {
        assert!(
            message.contains(fragment),
            "{message:?} missing {fragment:?}"
        );
    }
}

#[test]
fn actual_authored_and_stored_count_refusals_name_their_ranges() {
    let authored = [[0., 3.]; 5];
    let stored = [[1., 0., 3.]; 11];
    let mut input = query();
    input.authored = &authored;
    let error = certify_cylindrical_profile(input, || false).unwrap_err();
    assert_diagnostic(&error, &["authored poles", "actual: 5", "2..=4"]);
    input = query();
    input.stored.poles = &stored;
    let error = certify_cylindrical_profile(input, || false).unwrap_err();
    assert_diagnostic(&error, &["stored poles", "actual: 11", "2..=10"]);
}

#[test]
fn actual_policy_refusal_preserves_values_and_states_supported_ranges() {
    let mut input = query();
    input.options.fractional_bits = 7;
    input.options.max_operations = 1_000_001;
    let error = certify_cylindrical_profile(input, || false).unwrap_err();
    assert_diagnostic(
        &error,
        &["fractional_bits: 7", "1000001", "8..=4096", "0..=1000000"],
    );
    assert_eq!(error.requested_options, input.options);
    assert!(matches!(error.cause, CylindricalProfileFailure::Policy(_)));
}

#[test]
fn actual_value_refusals_distinguish_positive_radius_and_nonnegative_limit() {
    let mut input = query();
    input.support_radius = 0.;
    let error = certify_cylindrical_profile(input, || false).unwrap_err();
    assert_diagnostic(
        &error,
        &["support radius", "value: 0.0", "finite value > 0"],
    );
    input = query();
    input.spatial_limit = -1.;
    let error = certify_cylindrical_profile(input, || false).unwrap_err();
    assert_diagnostic(
        &error,
        &["spatial limit", "value: -1.0", "finite value >= 0"],
    );
}

#[test]
fn actual_interval_refusal_names_both_index_and_level_constraints() {
    let error = CylindricalProfileInterval::new(4, 2).unwrap_err();
    let message = error.to_string();
    for fragment in ["index: 4", "level: 2", "level <= 63", "index < 2^level"] {
        assert!(
            message.contains(fragment),
            "{message:?} missing {fragment:?}"
        );
    }
}

#[test]
fn unknown_public_count_role_does_not_invent_a_supported_maximum() {
    let error = CylindricalProfileFailure::Count {
        role: "external coefficient role",
        actual: 7,
    };
    let message = error.to_string();
    assert!(message.contains("actual: 7"));
    assert!(message.contains("maximum unavailable"));
    assert!(!message.contains("2..=4"));
    assert!(!message.contains("2..=10"));
}

#[test]
fn actual_work_refusal_keeps_its_original_typed_arithmetic_cause() {
    let mut input = query();
    input.options.max_operations = 0;
    let error = certify_cylindrical_profile(input, || false).unwrap_err();
    assert_diagnostic(
        &error,
        &["completed: 0", "limit: 0", "unchanged work budget"],
    );
    assert!(matches!(
        error.cause,
        CylindricalProfileFailure::Arithmetic(cause)
            if matches!(*cause, BoundedArithmeticFailure::WorkLimit { completed: 0, limit: 0 })
    ));
}

#[test]
fn actual_nonfinite_coordinate_refusal_names_the_finite_constraint() {
    let stored = [[f64::NAN, 0., 3.], [1., 0., 4.]];
    let mut input = query();
    input.stored.poles = &stored;
    let error = certify_cylindrical_profile(input, || false).unwrap_err();
    assert_diagnostic(
        &error,
        &["stored coordinate", "value: NaN", "finite binary64 value"],
    );
}

#[test]
fn actual_retained_weight_count_refusal_names_the_pole_relationship() {
    let weights = [1.];
    let mut input = query();
    input.stored.weights = Some(&weights);
    let error = certify_cylindrical_profile(input, || false).unwrap_err();
    assert_diagnostic(
        &error,
        &["poles: 2", "weights: 1", "one retained weight per pole"],
    );
}

fn fit_query(source_level: u8, max_level: u8, max_operations: u32) -> CylindricalFitQuery<'static> {
    CylindricalFitQuery {
        authored: &AUTHORED,
        interval: CylindricalProfileInterval::new(0, source_level).unwrap(),
        support_radius: 2.,
        angular_origin: 0.,
        spatial_limit: 1e-6,
        max_level,
        max_operations,
    }
}

#[test]
fn combined_planning_level_refusal_reports_both_inputs_and_their_sum() {
    let input = fit_query(63, 1, 100_000);
    let mut polls = 0;
    let error = plan_cylindrical_fit(input, || {
        polls += 1;
        false
    })
    .unwrap_err();
    assert_diagnostic(&error, COMBINED_LEVEL_63_1_EXPECTATION);
    assert_diagnostic(&error, &["source_level + additional_level <= 63"]);
    assert_eq!(polls, 0);
    assert_eq!(error.requested_options.max_operations, input.max_operations);
}

#[test]
fn maximum_valid_source_interval_still_accepts_a_zero_additional_level() {
    let input = fit_query(63, 0, 100_000);
    let plan = plan_cylindrical_fit(input, || false).unwrap();
    assert_eq!(plan.subdivision_level(), 0);
    assert_eq!(plan.ideal_developed_error_bound(), 0.);
}

#[test]
fn source_interval_constructor_refuses_its_own_invalid_level_before_planning() {
    let error = CylindricalProfileInterval::new(0, 64).unwrap_err();
    assert_diagnostic(&error, &["index: 0", "level: 64", "level <= 63"]);
    assert!(matches!(
        error,
        CylindricalProfileFailure::Interval {
            index: 0,
            level: 64
        }
    ));
}

#[test]
fn unary_planning_work_policy_refusal_preserves_its_original_variant() {
    let input = fit_query(0, 1, 1_000_001);
    let error = plan_cylindrical_fit(input, || false).unwrap_err();
    assert_diagnostic(
        &error,
        &[
            "maximum_level: 1",
            "maximum_operations: 1000001",
            "0..=1000000",
        ],
    );
    assert!(matches!(
        error.cause,
        CylindricalProfileFailure::FitPolicy { .. }
    ));
}

#[test]
fn combined_level_refusal_precedes_an_invalid_work_allowance() {
    let error = plan_cylindrical_fit(fit_query(63, 1, 1_000_001), || false).unwrap_err();
    assert_diagnostic(&error, COMBINED_LEVEL_63_1_EXPECTATION);
    assert_eq!(error.requested_options.max_operations, 1_000_001);
}
