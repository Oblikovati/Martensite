// SPDX-License-Identifier: GPL-2.0-only
//! Independent ideal-to-stored rational coefficient controls.
use super::*;
use std::cmp::Ordering;
fn request<'a>(poles: &'a [[f64; 3]], stored_weights: &'a [f64]) -> ConicalStorageQuery<'a> {
    ConicalStorageQuery {
        tangent: &[0., 0.],
        numerator: &[3., 3.],
        weights: &[1., 1.],
        semi_angle: std::f64::consts::FRAC_PI_6,
        center: 0.,
        depth: 0.,
        outward: true,
        stored_poles: poles,
        stored_weights,
    }
}
fn near_zero(request: &ConicalStorageQuery<'_>) {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    let result = storage_discrepancy(request, &mut arithmetic).unwrap();
    assert_eq!(
        DyadicBound::new(result.upper, -128)
            .compare_f64(1e-12)
            .unwrap(),
        Ordering::Less
    );
}
#[test]
fn bounded_stored_rounding_stays_near_the_ideal_edge() {
    let point = [1.5, 0., 3. * 3_f64.sqrt() / 2.];
    near_zero(&request(&[point, point], &[1., 1.]));
}
#[test]
fn modified_stored_pole_increases_the_bound() {
    let point = [1.5, 0., 3. * 3_f64.sqrt() / 2.];
    let shifted = [1.51, 0., point[2]];
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    let result =
        storage_discrepancy(&request(&[point, shifted], &[1., 1.]), &mut arithmetic).unwrap();
    let upper = DyadicBound::new(result.upper, -128);
    assert_eq!(upper.compare_f64(0.009999999).unwrap(), Ordering::Greater);
    assert_eq!(upper.compare_f64(0.010001).unwrap(), Ordering::Less);
}
#[test]
fn nonconstant_homogeneous_weights_preserve_constant_geometry() {
    let point = [1.5, 0., 3. * 3_f64.sqrt() / 2.];
    let poles = [point, point];
    let mut candidate = request(&poles, &[1., 2.]);
    candidate.numerator = &[3., 6.];
    candidate.weights = &[1., 2.];
    near_zero(&candidate);
}
#[test]
fn rotated_center_and_inward_depth_match_independent_sector_point() {
    let point = [0., 1.5 - 0.1 * 3_f64.sqrt(), 3. * 3_f64.sqrt() / 2. + 0.1];
    let poles = [point, point];
    let mut candidate = request(&poles, &[1., 1.]);
    candidate.center = std::f64::consts::FRAC_PI_2;
    candidate.depth = 0.2;
    candidate.outward = false;
    near_zero(&candidate);
}
#[test]
fn malformed_storage_refuses() {
    let poles = [[1.5, 0., 3.]; 2];
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    assert!(matches!(
        storage_discrepancy(&request(&poles, &[1.]), &mut arithmetic),
        Err(ConicalStorageFailure::Shape { .. })
    ));
}
#[test]
fn nonpositive_stored_weight_refuses() {
    let poles = [[1.5, 0., 3.]; 2];
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    assert!(matches!(
        storage_discrepancy(&request(&poles, &[1., -1.]), &mut arithmetic),
        Err(ConicalStorageFailure::Weight {
            role: "stored weight",
            ..
        })
    ));
}
#[test]
fn nonfinite_center_refuses() {
    let poles = [[1.5, 0., 3.]; 2];
    let mut candidate = request(&poles, &[1., 1.]);
    candidate.center = f64::NAN;
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    assert!(matches!(
        storage_discrepancy(&candidate, &mut arithmetic),
        Err(ConicalStorageFailure::Nonfinite {
            role: "center and depth",
            index: 0,
            ..
        })
    ));
}
#[test]
fn work_exhaustion_refuses_without_storage_certificate() {
    let point = [1.5, 0., 3.];
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: 0,
        },
        &mut cancelled,
    );
    assert!(matches!(
        storage_discrepancy(&request(&[point, point], &[1., 1.]), &mut arithmetic),
        Err(ConicalStorageFailure::Arithmetic(
            BoundedArithmeticFailure::WorkLimit { .. }
        ))
    ));
}

#[test]
fn varying_angle_and_uniformly_scaled_native_weights_match() {
    let z = 3. * 3_f64.sqrt() / 2.;
    let poles = [[1.5, 0., z], [1.5, 1.5, z], [0., 1.5, z]];
    let mut candidate = request(&poles, &[2., 2., 4.]);
    candidate.tangent = &[0., 1.];
    candidate.numerator = &[3.];
    candidate.weights = &[1.];
    near_zero(&candidate);
}
#[test]
fn degree_elevated_native_curve_remains_close() {
    let z = 3. * 3_f64.sqrt() / 2.;
    let poles = [[1.5, 0., z], [1.5, 1., z], [0.75, 1.5, z], [0., 1.5, z]];
    let mut candidate = request(&poles, &[1., 1., 4. / 3., 2.]);
    candidate.tangent = &[0., 1.];
    candidate.numerator = &[3.];
    candidate.weights = &[1.];
    near_zero(&candidate);
}
#[test]
fn maximum_supported_degrees_are_bounded() {
    let point = [1.5, 0., 3. * 3_f64.sqrt() / 2.];
    let poles = [point; 16];
    let weights = [1.; 16];
    let mut candidate = request(&poles, &weights);
    candidate.tangent = &[0.; 4];
    candidate.numerator = &[3.; 10];
    candidate.weights = &[1.; 10];
    near_zero(&candidate);
}
#[test]
fn invalid_semi_angle_and_nonfinite_pole_refuse() {
    let poles = [[1.5, 0., 3.], [1.5, 0., f64::NAN]];
    let mut candidate = request(&poles, &[1., 1.]);
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    candidate.semi_angle = 0.;
    assert!(matches!(
        storage_discrepancy(&candidate, &mut arithmetic),
        Err(ConicalStorageFailure::Support { semi_angle: 0. })
    ));
    candidate.semi_angle = 0.5;
    assert!(
        matches!(storage_discrepancy(&candidate, &mut arithmetic), Err(ConicalStorageFailure::Nonfinite { role: "stored pole", index: 1, axis: 2, bits }) if f64::from_bits(bits).is_nan())
    );
}
#[test]
fn tiny_unresolved_stored_weight_refuses() {
    let point = [1.5, 0., 3.];
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    assert!(matches!(
        storage_discrepancy(
            &request(&[point, point], &[f64::MIN_POSITIVE; 2]),
            &mut arithmetic
        ),
        Err(ConicalStorageFailure::Weight {
            role: "stored weight",
            ..
        })
    ));
}

fn before_residual(request: &ConicalStorageQuery<'_>, arithmetic: &mut Arithmetic<'_>) {
    admit_storage(request).unwrap();
    prepare_storage(request, arithmetic).unwrap();
}
#[test]
fn work_exhaustion_inside_storage_residual_is_typed() {
    let point = [1.5, 0., 3.];
    let poles = [point, point];
    let candidate = request(&poles, &[1., 1.]);
    let mut available = || false;
    let mut full = Arithmetic::new(ArithmeticPolicy::default(), &mut available);
    before_residual(&candidate, &mut full);
    let limit = full.operations();
    let mut available = || false;
    let mut limited = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: limit,
        },
        &mut available,
    );
    assert!(
        matches!(storage_discrepancy(&candidate, &mut limited), Err(ConicalStorageFailure::Arithmetic(BoundedArithmeticFailure::WorkLimit { completed, .. })) if completed == limit)
    );
}
fn polls_before_residual(candidate: &ConicalStorageQuery<'_>) -> usize {
    let mut polls = 0;
    let mut counting = || {
        polls += 1;
        false
    };
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut counting);
    before_residual(candidate, &mut arithmetic);
    polls
}
#[test]
fn cancellation_inside_storage_residual_is_typed() {
    let point = [1.5, 0., 3.];
    let poles = [point, point];
    let candidate = request(&poles, &[1., 1.]);
    let permitted = polls_before_residual(&candidate);
    let mut polls = 0;
    let mut cancelled = || {
        polls += 1;
        polls > permitted
    };
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    assert!(matches!(
        storage_discrepancy(&candidate, &mut arithmetic),
        Err(ConicalStorageFailure::Arithmetic(
            BoundedArithmeticFailure::Cancelled
        ))
    ));
}

fn decoded_native_fixture() -> (Vec<[f64; 3]>, Vec<f64>) {
    let fixture = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/features/surface-normal-sketches/references/conical-native-edge-v1.txt"
    ));
    let rows: Vec<Vec<f64>> = fixture
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| f64::from_bits(u64::from_str_radix(word, 16).unwrap()))
                .collect()
        })
        .collect();
    let poles = rows.iter().map(|row| [row[0], row[1], row[2]]).collect();
    let weights = rows.iter().map(|row| row[3]).collect();
    (poles, weights)
}
#[test]
fn actual_decoded_native_fixture_has_small_storage_error() {
    let (poles, weights) = decoded_native_fixture();
    let mut candidate = request(&poles, &weights);
    candidate.tangent = &[0., 1.];
    near_zero(&candidate);
}
#[test]
fn actual_decoded_native_fixture_detects_endpoint_mutation() {
    let (mut poles, weights) = decoded_native_fixture();
    poles.last_mut().unwrap()[0] += 0.01;
    let mut candidate = request(&poles, &weights);
    candidate.tangent = &[0., 1.];
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    let result = storage_discrepancy(&candidate, &mut arithmetic).unwrap();
    assert_eq!(
        DyadicBound::new(result.upper, -128)
            .compare_f64(0.009999999)
            .unwrap(),
        Ordering::Greater
    );
}
