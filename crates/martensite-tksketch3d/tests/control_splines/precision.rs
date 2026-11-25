// SPDX-License-Identifier: GPL-2.0-only
use super::{rows, sid, snapshot};

const FAR: f64 = f64::from_bits(1083_u64 << 52);

#[test]
fn a_far_first_pole_keeps_the_other_endpoint_jet_and_its_neighborhood() {
    let source = snapshot([[FAR, 0., 0.], [0., 1., 0.], [1., 2., 0.], [2., 3., 0.]]);
    let spline = source.control_spline(sid(5)).unwrap();
    assert_eq!(
        rows(spline.jet(1.).unwrap()),
        [[2., 3., 0.], [3., 3., 0.], [0.; 3]]
    );
    assert_eq!(spline.first_derivative(1.).unwrap().x(), 3.);
    let parameter = 1_f64.next_down();
    let left = 1. - parameter;
    let jet = spline.jet(parameter).unwrap();
    // For these controls D1.x = 3[1 - (FAR+1)(1-t)^2] and D2.x = 6(FAR+1)(1-t).
    let speed_x = 3. * (1. - FAR * left * left);
    assert!((jet.first.x() - speed_x).abs() <= 4. * f64::EPSILON);
    assert_eq!(jet.second.x(), 6. * FAR * left);
}

#[test]
fn a_tiny_nonzero_parameter_keeps_its_first_interior_pole_contribution() {
    let source = snapshot([[0., 0., 0.], [FAR, 1., 0.], [1., 2., 0.], [2., 3., 0.]]);
    let spline = source.control_spline(sid(5)).unwrap();
    let parameter = f64::from_bits(968_u64 << 52);
    // Here 3*t*FAR is 96. The remaining Bernstein terms leave it within half an ulp of 96.
    let value = spline.value(parameter).unwrap();
    assert_eq!(value.x(), 96.);
    assert_eq!(value.y(), 3. * parameter);
    assert_eq!(spline.jet(parameter).unwrap().position, value);
}

#[test]
fn opposing_far_poles_keep_the_interior_first_derivative_residual() {
    let source = snapshot([[FAR, 0., 0.], [0., 1., 0.], [1., 2., 0.], [FAR, 3., 0.]]);
    let spline = source.control_spline(sid(5)).unwrap();
    // At t=1/2 the derivative is 3/4 times (-FAR + 2 + FAR - 1).
    let expected = [0.75, 3., 0.];
    assert_eq!(
        spline.first_derivative(0.5).unwrap().xyz().coordinates(),
        &expected
    );
    assert_eq!(rows(spline.jet(0.5).unwrap())[1], expected);
}

#[test]
fn endpoint_acceleration_rounds_only_after_large_poles_cancel() {
    let source = snapshot([
        [0., 0., 0.],
        [2. * FAR, 1., 0.],
        [FAR, 2., 0.],
        [1., 3., 0.],
    ]);
    let spline = source.control_spline(sid(5)).unwrap();
    // The terminal acceleration is 6(P3 - 2P2 + P1).
    assert_eq!(rows(spline.jet(1.).unwrap())[2], [6., 0., 0.]);
}
