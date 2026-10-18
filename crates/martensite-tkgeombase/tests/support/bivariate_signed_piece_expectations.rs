//! Expected stores share one independent owner, so overlapping role tails are derived once.
use super::*;
const CONSTANT_WEIGHTS: [f64; 4] = [
    0.2564561626532326,
    0.221824096603541,
    0.15724713746926033,
    0.07157938446051357,
];
const QUADRATIC_WEIGHTS: [f64; 4] = [
    -0.25778313522636853,
    -0.04252004995346777,
    0.1589358116831741,
    0.1413673734966622,
];
fn folded_projection(weights: &[f64; 4]) -> f64 {
    let mut coefficient = 0.;
    for weight in weights {
        coefficient += 2. * weight;
    }
    coefficient
}
fn independent_projection() -> [f64; 3] {
    let constant = folded_projection(&CONSTANT_WEIGHTS);
    let quadratic = folded_projection(&QUADRATIC_WEIGHTS);
    [
        constant * std::f64::consts::FRAC_1_SQRT_2,
        quadratic.abs() * 2.5_f64.sqrt(),
        ((quadratic * quadratic) / 2.).sqrt(),
    ]
}
fn expected_owner(prefix: bool) -> Vec<f64> {
    let mut owner = initial_reals();
    owner[64..67].fill(0.);
    owner[448..450].copy_from_slice(&[0., 1.]);
    let [coefficient, maximum, mean] = independent_projection();
    for index in if prefix { &[0, -1, 0][..] } else { &[0][..] } {
        let fold = (*index * 5) as isize;
        let error = *index as isize;
        owner[(192 + fold) as usize..(197 + fold) as usize].copy_from_slice(&[0., 2., 2., 2., 2.]);
        owner[(256 + fold) as usize..(261 + fold) as usize].fill(0.);
        owner[(512 + error) as usize] = maximum;
        owner[(576 + error) as usize] = mean;
        owner[(64 + *index * 3) as usize] = coefficient;
    }
    owner
}
pub(super) fn check_window(owner: &[f64], origin: usize, read: impl Fn(i32) -> f64) {
    for index in -8..12 {
        let expected = owner[(origin as i32 + index) as usize];
        assert_eq!(
            read(index).to_bits(),
            expected.to_bits(),
            "origin={origin},index={index}"
        );
    }
}
pub(super) fn check_curve(frame: &Frame, owner: &[f64]) {
    for index in -8..128 {
        assert_eq!(
            frame.caller_curve_value_signed(index).unwrap().to_bits(),
            owner[(64 + index) as usize].to_bits()
        );
        if index >= 0 {
            assert_eq!(
                frame.caller_curve_value(index as usize).unwrap().to_bits(),
                owner[(64 + index) as usize].to_bits()
            );
        }
    }
}
fn check_integer_owner(frame: &Frame, iso: i32, prefix: bool) {
    let mut expected = initial_integers(iso);
    expected[41] = 1;
    expected[42] = 0;
    expected[64] = 1;
    if prefix {
        expected[63] = 1;
    }
    assert_eq!(&frame.integer_values()[32..70], &expected[32..70]);
}
pub(super) fn check_published_owner(frame: &Frame, iso: i32, prefix: bool) {
    let owner = expected_owner(prefix);
    check_integer_owner(frame, iso, prefix);
    check_curve(frame, &owner);
    check_window(&owner, 192, |i| {
        frame.folded_output(Folded::SymmetricSum, i).unwrap()
    });
    check_window(&owner, 256, |i| {
        frame
            .folded_output(Folded::AntisymmetricDifference, i)
            .unwrap()
    });
    check_window(&owner, 320, |i| {
        frame.endpoint_output(Endpoint::Lower, i).unwrap()
    });
    check_window(&owner, 384, |i| {
        frame.endpoint_output(Endpoint::Upper, i).unwrap()
    });
    check_window(&owner, 448, |i| frame.cut_parameter(i).unwrap());
    check_window(&owner, 512, |i| {
        frame.error_output(ErrorRole::Maximum, i).unwrap()
    });
    check_window(&owner, 576, |i| {
        frame.error_output(ErrorRole::RootMeanSquare, i).unwrap()
    });
    assert_eq!(
        frame.coefficient_count(-1).unwrap(),
        if prefix { 1 } else { -7 }
    );
}
