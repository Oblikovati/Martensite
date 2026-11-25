// SPDX-License-Identifier: GPL-2.0-only
use super::*;

fn padded_binary64(value: f64, point: usize, axis: usize) -> Result<BigInt, PairSourceFailure> {
    if !value.is_finite() {
        return Err(PairSourceFailure::Coordinate(NonFiniteCoordinate {
            point,
            axis,
            value,
        }));
    }
    let bits = value.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as usize;
    let fraction = bits & ((1_u64 << 52) - 1);
    let significand = fraction | if exponent == 0 { 0 } else { 1_u64 << 52 };
    let integer = BigInt::from(significand) << exponent.saturating_sub(1);
    Ok(if bits >> 63 == 0 { integer } else { -integer })
}

fn original(
    source: &OwnedBezierSource,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<HomogeneousPolynomials, PairSourceFailure> {
    let unit = padded_binary64(1., 0, 0).unwrap();
    let mut components = [vec![], vec![], vec![]];
    for (index, &[x, y]) in source.controls.iter().enumerate() {
        poll(cancelled).map_err(PairSourceFailure::Algebra)?;
        let x = padded_binary64(x, index, 0)?;
        let y = padded_binary64(y, index, 1)?;
        let value = source.weights.as_ref().map_or(1., |weights| weights[index]);
        let weight = padded_binary64(value, index, 0)
            .map_err(|_| PairSourceFailure::Weight { index, value })?;
        for (coordinate, component) in [&x, &y, &unit].into_iter().zip(&mut components) {
            component.push(coordinate * &weight);
        }
    }
    let mut polynomials = vec![];
    for component in components {
        let powers = bernstein::power_coefficients(component, cancelled)
            .map_err(PairSourceFailure::Algebra)?;
        polynomials.push(ExactCoefficientPolynomial::new(powers));
    }
    let components = polynomials.try_into().unwrap();
    Ok(HomogeneousPolynomials { components }.reduce_shared_content())
}

#[test]
fn dyadic_conversion_matches_padded_arithmetic_for_binary64_extremes() {
    let controls = [
        [-0., 0.],
        [f64::from_bits(1), -f64::MIN_POSITIVE],
        [f64::MAX, -f64::MAX],
    ];
    let unit_weights = [1., 1., 1.];
    let signed_weights = [-0., -f64::MAX, f64::from_bits(3)];
    let zero_weights = [0., -0., 0.];
    for weights in [
        None,
        Some(&unit_weights[..]),
        Some(&signed_weights[..]),
        Some(&zero_weights[..]),
    ] {
        compare(&controls, weights);
    }
    compare(&[[0., -0.]; 3], None);
    compare(&[[0., -0.]; 3], Some(&[-f64::MAX, -0., -2.]));
}

#[test]
fn dyadic_conversion_matches_padded_arithmetic_on_generated_profiles() {
    let mut state = 0x4d617274656e7369_u64;
    let mut finite = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        f64::from_bits(if state & 0x7ff0_0000_0000_0000 == 0x7ff0_0000_0000_0000 {
            state ^ 0x0010_0000_0000_0000
        } else {
            state
        })
    };
    for i in 0..128 {
        let count = 1 + i % 7;
        let controls: Vec<_> = (0..count).map(|_| [finite(), finite()]).collect();
        let weights: Vec<_> = (0..count).map(|_| finite()).collect();
        compare(&controls, None);
        compare(&controls, Some(&weights));
    }
}

fn compare(controls: &[[f64; 2]], weights: Option<&[f64]>) {
    let source = OwnedBezierSource::capture(controls, weights, [0., 1.]).unwrap();
    assert_eq!(
        source.homogeneous(&mut || false),
        original(&source, &mut || false)
    );
}

#[test]
fn dyadic_conversion_preserves_every_cancellation_checkpoint() {
    let controls = [[-0., 0.25], [1.25, -0.5], [2., 8.], [-1., 10.]];
    let weights = [1., 0.625, -2., 0.];
    for weights in [None, Some(&weights[..])] {
        let source = OwnedBezierSource::capture(&controls, weights, [0., 1.]).unwrap();
        let mut total = 0;
        original(&source, &mut || {
            total += 1;
            false
        })
        .unwrap();
        assert_eq!(total, 34);
        for stop in 1..=total + 1 {
            let (mut old_polls, mut new_polls) = (0, 0);
            let old = original(&source, &mut || {
                old_polls += 1;
                old_polls == stop
            });
            let new = source.homogeneous(&mut || {
                new_polls += 1;
                new_polls == stop
            });
            assert_eq!(new, old, "checkpoint {stop}");
            assert_eq!(new_polls, old_polls, "checkpoint {stop}");
        }
    }
}

#[test]
fn dyadic_conversion_retains_coordinate_then_weight_failure_order_and_payload() {
    let nan = f64::from_bits(0xfff8_0000_1234_5678);
    let controls = [[0., 0.], [1., nan], [f64::INFINITY, 1.]];
    let source =
        OwnedBezierSource::capture(&controls, Some(&[1., -f64::INFINITY, nan]), [0., 1.]).unwrap();
    let old = original(&source, &mut || false).unwrap_err();
    let new = source.homogeneous(&mut || false).unwrap_err();
    for error in [old, new] {
        let PairSourceFailure::Coordinate(error) = error else {
            panic!("{error:?}");
        };
        assert_eq!(
            (error.point, error.axis, error.value.to_bits()),
            (1, 1, nan.to_bits())
        );
    }
    let source =
        OwnedBezierSource::capture(&[[0., 0.], [1., 2.]], Some(&[1., nan]), [0., 1.]).unwrap();
    let old = original(&source, &mut || false).unwrap_err();
    let new = source.homogeneous(&mut || false).unwrap_err();
    for error in [old, new] {
        let PairSourceFailure::Weight { index, value } = error else {
            panic!("{error:?}");
        };
        assert_eq!((index, value.to_bits()), (1, nan.to_bits()));
    }
}
