// SPDX-License-Identifier: GPL-2.0-only
use super::{CubicEndJet, CubicEndJetFailure, CubicEndStencil, TangentDirectionBand};
use std::cmp::Ordering::{Equal, Greater, Less};

const INCOMING: [[f64; 3]; 2] = [[-1., 0., 0.], [0.; 3]];

#[test]
fn exact_curvature_witness_keeps_speed_and_spatial_limits_separate() {
    let straight = CubicEndJet::new(CubicEndStencil::Curvature([
        [0.; 3],
        [1., 0., 0.],
        [2., 0., 0.],
    ]))
    .unwrap();
    assert_eq!(straight.curvature(0.), Ok(Equal));
    let curved = CubicEndJet::new(CubicEndStencil::Curvature([
        [0.; 3],
        [1., 0., 0.],
        [2., 1., 0.],
    ]))
    .unwrap();
    let rounded = curved.rounded().unwrap();
    assert_eq!(rounded.first, [3., 0., 0.]);
    assert_eq!(rounded.second, Some([0., 6., 0.]));
    let nearest = 2_f64 / 3.;
    assert_eq!(curved.curvature(nearest), Ok(Greater));
    assert_eq!(curved.curvature(nearest.next_up()), Ok(Less));
    assert_eq!(curved.speed(3.), Ok(Equal));
    assert_eq!(curved.speed(3_f64.next_up()), Ok(Less));
}

#[test]
fn signed_tangent_uses_exact_sine_even_when_squared_tolerance_underflows() {
    let axis = CubicEndJet::new(CubicEndStencil::Tangent([[0.; 3], [1., 0., 0.]])).unwrap();
    let tiny = f64::MIN_POSITIVE;
    assert_eq!(
        axis.tangent(INCOMING, tiny),
        Ok(TangentDirectionBand {
            dot: Greater,
            sine: Less
        })
    );
    let reversed = [INCOMING[1], INCOMING[0]];
    assert_eq!(
        axis.tangent(reversed, tiny),
        Ok(TangentDirectionBand {
            dot: Less,
            sine: Less
        })
    );
    let three_four = CubicEndJet::new(CubicEndStencil::Tangent([[0.; 3], [4., 3., 0.]])).unwrap();
    assert_eq!(three_four.tangent(INCOMING, 0.6).unwrap().sine, Greater);
    assert_eq!(
        three_four
            .tangent(INCOMING, 0.6_f64.next_up())
            .unwrap()
            .sine,
        Less
    );
}

#[test]
fn endpoint_reversal_preserves_acceleration_after_large_pole_cancellation() {
    let large = f64::from_bits((1023_u64 + 60) << 52);
    let [a, b, c] = [[1., 0., 0.], [large, 1., 0.], [2. * large, 2., 0.]];
    let original = CubicEndJet::new(CubicEndStencil::Curvature([a, b, c]))
        .unwrap()
        .rounded()
        .unwrap();
    let reversed = CubicEndJet::new(CubicEndStencil::Curvature([c, b, a]))
        .unwrap()
        .rounded()
        .unwrap();
    assert_eq!(original.first, [3. * large, 3., 0.]);
    assert_eq!(reversed.first, [-3. * large, -3., 0.]);
    assert_eq!(original.second, Some([6., 0., 0.]));
    assert_eq!(original.second, reversed.second);
}

#[test]
fn unused_curvature_and_stationary_velocity_refuse_distinctly() {
    assert_eq!(
        CubicEndJet::new(CubicEndStencil::Tangent([[0.; 3], [-0.; 3]])),
        Err(CubicEndJetFailure::Stationary)
    );
    let jet = CubicEndJet::new(CubicEndStencil::Tangent([[0.; 3], [1., 0., 0.]])).unwrap();
    assert_eq!(jet.curvature(1.), Err(CubicEndJetFailure::MissingSecond));
    assert_eq!(
        jet.tangent([[0.; 3]; 2], 0.),
        Err(CubicEndJetFailure::DegenerateLine)
    );
}
