// SPDX-License-Identifier: GPL-2.0-only
use super::{
    DyadicBound, HelixCylindricalSpanLaw, HelixSpanProgress, HelixSplineSource, HelixTraceBounds,
    HelixTraceFailure, LAW, LIMITS, ResultOf, TAU, certify_cylindrical_helix_span,
};

fn exact(value: f64) -> DyadicBound {
    DyadicBound::from_f64(value).unwrap()
}

fn controls(values: &[f64]) -> Vec<DyadicBound> {
    values.iter().copied().map(exact).collect()
}

fn certify(
    mode: HelixSpanProgress,
    axial: &[f64],
    degree: usize,
    poles: &[[f64; 3]],
) -> ResultOf<HelixTraceBounds> {
    let axial = controls(axial);
    let [first, last] = [0.25, 0.251];
    let knots = [vec![first; degree + 1], vec![last; degree + 1]].concat();
    let turns = [exact(0.), exact(0.001 / TAU)];
    let reference = HelixCylindricalSpanLaw {
        origin: LAW.origin,
        basis: LAW.basis,
        radius: 4.,
        turns: turns.each_ref(),
        axial_controls: &axial,
        native_parameter: [first, last],
        positive_rotation: true,
        progress: mode,
    };
    let retained = HelixSplineSource {
        degree,
        poles,
        knots: &knots,
        trim: [first, last],
    };
    certify_cylindrical_helix_span(reference, retained, LIMITS, || false)
}

#[test]
fn native_flat_requires_exact_planarity_even_with_spatial_budget_to_spare() {
    let poles = [[4., 0., 3.], [4., 0.004, 3.]];
    let proof = certify(HelixSpanProgress::Flat, &[3.], 1, &poles).unwrap();
    assert_eq!(proof.axial_derivative_lower, 0.);
    assert!(proof.position_error_upper <= LIMITS.max_position_error);
    let mut changed = poles;
    changed[1][2] = 3_f64.next_up();
    assert!(matches!(
        certify(HelixSpanProgress::Flat, &[3.], 1, &changed),
        Err(HelixTraceFailure::FlatProgress)
    ));
}

#[test]
fn native_stationary_endpoint_requires_positive_progress_inside_every_span() {
    let controls = [0., 0., 0., 0.05, 0.1];
    let poles = std::array::from_fn::<_, 5, _>(|i| [4., i as f64 * 0.001, controls[i]]);
    let proof = certify(HelixSpanProgress::Ramp, &controls, 4, &poles).unwrap();
    assert_eq!(proof.axial_derivative_lower, 0.);
    assert!(proof.position_error_upper <= LIMITS.max_position_error);
    let mut changed = poles;
    changed[1][2] = -f64::EPSILON;
    assert!(matches!(
        certify(HelixSpanProgress::Ramp, &controls, 4, &changed),
        Err(HelixTraceFailure::AxialProgress)
    ));
}
