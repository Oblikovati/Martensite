//! Integer powers preserve the specified sequence of binary64 operations (#201).
//! Expected words come from exact rational products and reciprocals, rounding after
//! every operation. NaN signs and payloads are deliberately outside this contract.

const VECTORS: [&str; 2] = [
    include_str!("fixtures/powi_sequence/cases_00.txt"),
    include_str!("fixtures/powi_sequence/cases_01.txt"),
];

#[test]
fn integer_power_matches_independent_operation_sequence() {
    let mut count: usize = 0;
    for row in VECTORS.into_iter().flat_map(str::lines) {
        check_sequence_row(row);
        count += 1;
    }
    assert_eq!(count, 700, "expected all 700 integer-power cases");
}

fn check_sequence_row(row: &str) {
    let (word, power, expected): (u64, i32, &str) = parse_sequence_row(row);
    let actual: f64 = martensite_cmath::powi(f64::from_bits(word), power);
    if expected == "nan" {
        assert!(actual.is_nan(), "row {row:?}; expected NaN, got {actual:?}");
        return;
    }
    let expected: u64 = parse_sequence_word(expected);
    assert_eq!(actual.to_bits(), expected, "integer-power row {row:?}");
}

fn parse_sequence_row(row: &str) -> (u64, i32, &str) {
    let fields: Vec<&str> = row.split_whitespace().collect();
    assert_eq!(
        fields.len(),
        3,
        "row {row:?}; expected input exponent result"
    );
    let word: u64 = parse_sequence_word(fields[0]);
    let power: i32 = fields[1]
        .parse()
        .unwrap_or_else(|_| panic!("exponent {:?}; expected signed i32", fields[1]));
    (word, power, fields[2])
}

fn parse_sequence_word(token: &str) -> u64 {
    u64::from_str_radix(token, 16)
        .unwrap_or_else(|_| panic!("word {token:?}; expected hexadecimal binary64 bits"))
}

#[test]
fn reciprocal_follows_positive_power_overflow() {
    // The specified positive power overflows before inversion. Replacing this
    // sequence with correctly rounded full power would change its contract.
    for exponent in [-1024, -1074] {
        assert_eq!(martensite_cmath::powi(2.0, exponent).to_bits(), 0);
    }
    assert_eq!(martensite_cmath::powi(-0.0, -3), f64::NEG_INFINITY);
    assert_eq!(martensite_cmath::powi(-1.0, i32::MIN), 1.0);
}
