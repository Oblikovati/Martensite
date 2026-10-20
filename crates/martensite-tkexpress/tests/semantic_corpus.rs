//! Frozen expression semantics, replayed directly through Rust operations.
#![allow(clippy::unit_arg)] // The shared semantic driver records successful void steps.
#[path = "semantic_corpus/support/mod.rs"]
mod semantic_support;

const REQUESTS: &str = include_str!("semantic_corpus/fixtures/inputs.txt");
const EXPECTED: &str = include_str!("semantic_corpus/fixtures/expected.txt");

type SemanticOperation = fn(&[&str]) -> String;

fn replay_family(operation: &str, execute: SemanticOperation, expected_count: usize) {
    assert_eq!(REQUESTS.lines().count(), EXPECTED.lines().count());
    let mut count = 0;
    for (index, (request, expected)) in REQUESTS.lines().zip(EXPECTED.lines()).enumerate() {
        let mut fields = request.split_whitespace();
        if fields.next() != Some(operation) {
            continue;
        }
        assert_semantic_request(request, expected, index, execute);
        count += 1;
    }
    assert_eq!(
        count, expected_count,
        "expected complete {operation} family"
    );
}

fn assert_semantic_request(
    request: &str,
    expected: &str,
    index: usize,
    execute: SemanticOperation,
) {
    let arguments = request.split_whitespace().skip(1).collect::<Vec<_>>();
    let actual = execute(&arguments);
    assert_eq!(
        canonical_nan_line(&actual),
        canonical_nan_line(expected),
        "semantic request {}: {request}",
        index + 1
    );
}

fn canonical_nan_line(line: &str) -> String {
    line.split(' ')
        .map(canonical_nan_token)
        .collect::<Vec<_>>()
        .join(" ")
}

fn canonical_nan_token(token: &str) -> &str {
    if !(16..=20).contains(&token.len()) || !token.bytes().all(|byte| byte.is_ascii_digit()) {
        return token;
    }
    match token.parse::<u64>() {
        Ok(bits) if f64::from_bits(bits).is_nan() => "NaN",
        _ => token,
    }
}

#[test]
fn interpreter_sequences_and_seeded_invalid_tokens_replay() {
    replay_family("ex_text", semantic_support::text::text_case, 3318);
}

#[test]
fn unary_functions_special_values_and_random_values_replay() {
    replay_family("ex_fold", semantic_support::fold::fold_case, 2200);
}

#[test]
fn binary_operators_special_values_and_random_values_replay() {
    replay_family("ex_fold2", semantic_support::fold::fold2_case, 1800);
}

#[test]
fn relation_structure_and_mutations_replay() {
    replay_family("ex_rel", semantic_support::rel::rel_case, 13);
}

#[test]
fn direct_expression_api_scenarios_replay() {
    replay_family("ex_case", semantic_support::cases::case_case, 9);
}

#[test]
fn numeric_comparison_preserves_finite_infinity_and_signed_zero_bits() {
    for bits in [
        0_u64,
        1_u64 << 63,
        1_f64.to_bits(),
        f64::INFINITY.to_bits(),
        f64::NEG_INFINITY.to_bits(),
    ] {
        let token = bits.to_string();
        assert_eq!(canonical_nan_token(&token), token);
    }
    assert_ne!(
        canonical_nan_line(" 0"),
        canonical_nan_line(" 9223372036854775808")
    );
    assert_ne!(canonical_nan_line(" 0"), canonical_nan_line("0"));
}

#[test]
fn numeric_comparison_normalizes_only_nan_classification() {
    for bits in [0x7ff8_0000_0000_0001_u64, 0xfff8_0000_0000_0001] {
        assert_eq!(canonical_nan_token(&bits.to_string()), "NaN");
    }
    for text in [
        "x9221120237041090561",
        "!Standard_Failure",
        "18446744073709551616",
    ] {
        assert_eq!(canonical_nan_token(text), text);
    }
}

#[test]
fn completed_semantic_lines_reset_the_buffer() {
    let mut line = semantic_support::common::SemanticLine::default();
    line.int(7);
    assert_eq!(line.end(), " 7");
    assert_eq!(line.end(), "");
    line.boolean(false);
    assert_eq!(line.end(), " 0");
}

#[test]
fn inverse_hyperbolic_constants_keep_the_owned_negative_nan_text() {
    for encoded in [
        "x61636f736828302e323529",
        "x41436f736828302e323529",
        "x6174616e68283229",
        "x4154616e68283229",
    ] {
        let result = semantic_support::text::text_case(&[encoded]);
        let fields = result.split(' ').collect::<Vec<_>>();
        for index in [3, 8, 9, 10] {
            assert_eq!(fields[index], "x2d6e616e");
        }
    }
}
