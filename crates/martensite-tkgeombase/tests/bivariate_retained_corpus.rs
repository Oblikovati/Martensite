//! The original approximation protocol stays in one fresh process and its original order.
#[path = "support/construction_corpus/mod.rs"]
mod construction_corpus;
const REQUESTS: &str = include_str!("fixtures/bivariate_legacy/requests.txt");
const ANSWERS: &str = include_str!("fixtures/bivariate_legacy/answers.txt");
const FAMILIES: &[(&str, usize)] = &[
    ("aa_mma1her", 5),
    ("aa_mma2roo", 5),
    ("aa_mma2jmx", 5),
    ("aa_mmapptt", 15),
    ("aa_mma2ac1", 4),
    ("aa_mma2ac2", 3),
    ("aa_mma2ac3", 3),
    ("aa_mma2can", 6),
    ("aa_mma2cdi", 8),
    ("aa_mma2ds1", 7),
    ("aa_mma2ce1", 15),
    ("aa_mma2fnc", 11),
    ("aa_mma2fx6", 4),
    ("aa_mma2fnc_mut_order", 1),
    ("aa_mma2ds1_mut_state", 1),
    ("aa_context", 10),
    ("aa_grid", 4),
    ("aa_patch", 7),
    ("aa_approx", 15),
];

fn check_frozen_scope() {
    assert_eq!(REQUESTS.lines().count(), 129);
    assert_eq!(ANSWERS.lines().count(), 129);
    assert_eq!(
        construction_corpus::replay::fixture_hash([REQUESTS].into_iter()),
        14139705285282157215
    );
    assert_eq!(
        construction_corpus::replay::fixture_hash([ANSWERS].into_iter()),
        17538326652504145611
    );
    for &(family, expected) in FAMILIES {
        assert_eq!(
            REQUESTS
                .lines()
                .filter(|row| row.split_whitespace().next() == Some(family))
                .count(),
            expected
        );
    }
}

#[test]
fn nineteen_families_preserve_all_requests_outputs_and_order() {
    check_frozen_scope();
    for (request, expected) in REQUESTS.lines().zip(ANSWERS.split_inclusive('\n')) {
        let family = request.split_whitespace().next().unwrap();
        assert!(construction_corpus::replay::checked_request(request, expected, family).is_some());
    }
}

#[test]
fn linear_hermite_basis_interpolates_both_endpoints_exactly() {
    let actual = construction_corpus::run_values("aa_mma1her", &[0.0]).unwrap();
    let coefficients: Vec<f64> = actual
        .split_whitespace()
        .skip(1)
        .map(|value| f64::from_bits(value.parse().unwrap()))
        .collect();
    assert_eq!(coefficients, [0.5, -0.5, 0.5, 0.5]);
    assert_eq!(coefficients[0] - coefficients[1], 1.0);
    assert_eq!(coefficients[0] + coefficients[1], 0.0);
    assert_eq!(coefficients[2] - coefficients[3], 0.0);
    assert_eq!(coefficients[2] + coefficients[3], 1.0);
}

#[test]
fn unsupported_projection_degree_preserves_every_initialized_slot() {
    let actual = construction_corpus::run_values("aa_mmapptt", &[8.0, 8.0, 0.0]).unwrap();
    let mut tokens = actual.split_whitespace();
    assert_eq!(tokens.next(), Some("31"));
    let slots: Vec<u64> = tokens.map(|value| value.parse().unwrap()).collect();
    assert_eq!(slots.len(), 45);
    assert!(slots.iter().all(|bits| *bits == 7.5_f64.to_bits()));
}
