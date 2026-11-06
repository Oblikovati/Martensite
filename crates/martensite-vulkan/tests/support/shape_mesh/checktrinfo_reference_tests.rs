use super::*;
fn reference(token: &str) -> CountExpectation {
    CountExpectation::from_checktrinfo_reference(token, 0., 1e-7).unwrap()
}
#[test]
fn case_literal_zero_disables_deflection_but_decimal_zero_checks_it() {
    assert_eq!(reference("0"), CountExpectation::Any);
    assert_eq!(
        reference("0.0"),
        CountExpectation::Near {
            value: 0.,
            tolerance_absolute: 0.,
            tolerance_relative: 1e-7,
        }
    );
}
#[test]
fn literal_one_requires_presence_while_decimal_one_requires_equality() {
    assert_eq!(reference("1"), CountExpectation::Present);
    assert!(matches!(
        reference("1.0"),
        CountExpectation::Near { value: 1., .. }
    ));
}
#[test]
fn boolean_words_and_unambiguous_prefixes_preserve_native_dispatch() {
    for word in [
        "false", "FALSE", "f", "fa", "fal", "fals", "0", "n", "no", "of", "off",
    ] {
        assert_eq!(reference(word), CountExpectation::Any, "{word}");
    }
    for word in [
        "true", "TRUE", "t", "tr", "tru", "1", "y", "ye", "yes", "on",
    ] {
        assert_eq!(reference(word), CountExpectation::Present, "{word}");
    }
}
#[test]
fn numeric_lexemes_are_not_coerced_to_boolean_after_parsing() {
    for (token, value) in [
        ("+0", 0.),
        ("-0", -0.),
        ("00", 0.),
        ("01", 1.),
        ("010", 8.),
        ("0x10", 16.),
        ("0o10", 8.),
        ("0b10", 2.),
        (" 0 ", 0.),
        ("2", 2.),
        ("1e0", 1.),
    ] {
        assert!(
            matches!(reference(token), CountExpectation::Near { value: actual, .. } if actual==value),
            "{token}"
        );
    }
}
#[test]
fn exact_case_counts_and_authored_relative_tolerance_are_retained() {
    assert!(matches!(
        reference("2"),
        CountExpectation::Near { value: 2., .. }
    ));
    assert!(matches!(
        reference("4"),
        CountExpectation::Near { value: 4., .. }
    ));
    assert_eq!(
        reference("0.1164052220738387"),
        CountExpectation::Near {
            value: 0.1164052220738387,
            tolerance_absolute: 0.,
            tolerance_relative: 1e-7,
        }
    );
}
#[test]
fn unsupported_or_ambiguous_references_refuse_instead_of_guessing() {
    for token in ["", "o", "08", "NaN", "Inf", "trueish", "!2", "$expected"] {
        let error = CountExpectation::from_checktrinfo_reference(token, 0., 0.).unwrap_err();
        assert!(error.message().contains(&format!("{token:?}")));
        assert!(
            error
                .message()
                .contains("expected a nonempty boolean spelling or finite numeric literal")
        );
    }
}
