// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use std::f64::consts::TAU;

const LAW: HelixTraceLaw = HelixTraceLaw {
    origin: [0.; 3],
    basis: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
    radii: [4.; 2],
    lead: HelixTraceLead::HeightAndTurns([0.1, 0.001 / TAU]),
    positive_rotation: true,
    parameter_end: 0.001,
};
const LIMITS: HelixTraceLimits = HelixTraceLimits {
    max_position_error: 3e-6,
    max_native_spans: 2,
    max_cells: 64,
    max_operations: 300_000,
};

fn source(poles: &[[f64; 3]; 2]) -> HelixSplineSource<'_> {
    HelixSplineSource {
        degree: 1,
        poles,
        knots: &[0., 0., 0.001, 0.001],
        trim: [0., 0.001],
    }
}

#[test]
fn short_linear_carrier_encloses_analytic_radial_sag_and_native_axial_progress() {
    let poles = [[4., 0., 0.], [4., 0.004, 0.1]];
    let proof = certify_polynomial_helix_trace(LAW, source(&poles), LIMITS, || false).unwrap();
    assert!(proof.position_error_upper >= 4. * (1. - martensite_cmath::cos(0.001)));
    assert!(proof.position_error_upper <= LIMITS.max_position_error);
    assert!(
        proof.axial_derivative_lower > 99. && proof.axial_derivative_lower <= 100.00000000000001
    );
    assert_eq!(proof.native_spans, 1);
}

#[test]
fn measured_shape_and_limits_cannot_substitute_for_retained_active_trim() {
    let poles = [[4., 0., 0.], [4., 0.004, 0.1]];
    let mut wrong_trim = source(&poles);
    wrong_trim.trim[1] = 0.0005;
    assert!(matches!(
        certify_polynomial_helix_trace(LAW, wrong_trim, LIMITS, || false),
        Err(HelixTraceFailure::InvalidSpline)
    ));
    let reversed = [[4., 0., 0.1], [4., 0.004, 0.]];
    assert!(matches!(
        certify_polynomial_helix_trace(LAW, source(&reversed), LIMITS, || false),
        Err(HelixTraceFailure::AxialProgress)
    ));
    let mismatch = [[4.2, 0., 0.], [4.2, 0.004, 0.1]];
    let short = HelixTraceLimits {
        max_cells: 1,
        ..LIMITS
    };
    assert!(matches!(
        certify_polynomial_helix_trace(LAW, source(&mismatch), short, || false),
        Err(HelixTraceFailure::CellLimit {
            completed: 1,
            limit: 1
        })
    ));
    assert!(matches!(
        certify_polynomial_helix_trace(LAW, source(&poles), LIMITS, || true),
        Err(HelixTraceFailure::Cancelled)
    ));
    let work = HelixTraceLimits {
        max_operations: 0,
        ..LIMITS
    };
    assert!(
        matches!(certify_polynomial_helix_trace(LAW, source(&poles), work, || false), Err(HelixTraceFailure::Arithmetic(cause)) if matches!(*cause, BoundedArithmeticFailure::WorkLimit { .. }))
    );
}

#[test]
fn exact_authored_pitch_product_cannot_be_replaced_with_its_rounded_height() {
    let factor = 1_f64.next_up();
    let rounded_height = factor * factor;
    let exact_difference = martensite_cmath::powi(2., -104);
    let radius = martensite_cmath::powi(2., -120);
    let parameter_end = TAU * factor;
    let source = HelixSplineSource {
        degree: 1,
        poles: &[[radius, 0., 0.], [radius, 0., rounded_height]],
        knots: &[0., 0., parameter_end, parameter_end],
        trim: [0., parameter_end],
    };
    let exact = HelixTraceLaw {
        radii: [radius; 2],
        lead: HelixTraceLead::PitchAndTurns([factor; 2]),
        parameter_end,
        ..LAW
    };
    let rounded = HelixTraceLaw {
        lead: HelixTraceLead::HeightAndTurns([rounded_height, factor]),
        ..exact
    };
    let narrow = HelixTraceLimits {
        max_position_error: exact_difference / 4.,
        ..LIMITS
    };
    let broad = HelixTraceLimits {
        max_position_error: exact_difference * 4.,
        ..LIMITS
    };
    let proved = certify_polynomial_helix_trace(exact, source, broad, || false).unwrap();
    assert!(proved.position_error_upper >= exact_difference);
    assert!(certify_polynomial_helix_trace(rounded, source, narrow, || false).is_ok());
    assert!(matches!(
        certify_polynomial_helix_trace(exact, source, narrow, || false),
        Err(HelixTraceFailure::PositionError { .. })
    ));
}
