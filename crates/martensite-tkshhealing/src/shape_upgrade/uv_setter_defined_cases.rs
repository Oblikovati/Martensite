//! Boundary observations supplement the universal two-branch source proof.
pub(super) struct UvSetterInput {
    pub u: i32,
    pub v: i32,
    pub prior_parts: i32,
    pub prior_mode: bool,
}
const INPUTS: &str = include_str!("../../tests/fixtures/behavior/uv_setter_defined_inputs.txt");
const EXPECTED: &str = include_str!("../../tests/fixtures/behavior/uv_setter_defined_expected.txt");

pub(super) fn defined_cases() -> Vec<(UvSetterInput, [f64; 6])> {
    assert_eq!(INPUTS.lines().count(), 848);
    assert_eq!(EXPECTED.lines().count(), 848);
    INPUTS
        .lines()
        .zip(EXPECTED.lines())
        .enumerate()
        .map(|(index, (input, output))| (parse_input(input), parse_output(index, output)))
        .collect()
}

fn parse_input(input: &str) -> UvSetterInput {
    let fields: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(fields.len(), 4);
    UvSetterInput {
        u: fields[0],
        v: fields[1],
        prior_parts: fields[2],
        prior_mode: fields[3] != 0,
    }
}

fn parse_output(index: usize, output: &str) -> [f64; 6] {
    let mut values = output.split_whitespace();
    assert_eq!(values.next().unwrap().parse::<usize>().unwrap(), index);
    let expected: Vec<f64> = values.map(|x| x.parse().unwrap()).collect();
    expected.try_into().unwrap()
}

pub(super) fn assert_overflow(error: &martensite_tkernel::standard::Failure, u: i32, v: i32) {
    assert_eq!(error.exception_type(), "Standard_OutOfRange");
    assert!(error.message().contains(&u.to_string()) && error.message().contains(&v.to_string()));
}

pub(super) fn expected_parts(input: &UvSetterInput) -> i32 {
    if input.u <= 0 || input.v <= 0 {
        return input.prior_parts;
    }
    i32::try_from(i64::from(input.u) * i64::from(input.v)).unwrap()
}

pub(super) const OVERFLOW: [(i32, i32); 4] = [
    (i32::MAX, 2),
    (2, i32::MAX),
    (46341, 46341),
    (i32::MAX, i32::MAX),
];
pub(super) const AREAS: [f64; 6] = [
    0.,
    -0.,
    12.5,
    f64::INFINITY,
    f64::NEG_INFINITY,
    f64::from_bits(0x7ff8_0000_0000_0123),
];
