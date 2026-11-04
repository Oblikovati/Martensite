use super::{source_coordinate_token, source_point_diagnostics};

#[test]
fn source_coordinate_character_class_keeps_numeric_words_only() {
    for word in ["1", "-0", "+2.4", "1e-06", "1E+08"] {
        assert!(source_coordinate_token(word), "numeric token {word:?}");
    }
    for word in ["", "nan", "inf", "Point:", "1 2", "(1)"] {
        assert!(!source_coordinate_token(word), "nonmatching token {word:?}");
    }
}

#[test]
fn all_three_source_targets_compare_the_actual_formatted_tokens() {
    for (point, target) in [
        ([1., 2., 3.], "1 2 3"),
        ([1., 2., 0.], "1 2 0"),
        ([1., 1.8383, 0.], "1 1.8383 0"),
    ] {
        assert!(source_point_diagnostics(&[point], target).is_empty());
    }
}

#[test]
fn source_precision_is_string_equality_not_an_added_metric_tolerance() {
    assert!(source_point_diagnostics(&[[1., 1.8382999, 0.]], "1 1.8383 0").is_empty());
    assert_eq!(
        source_point_diagnostics(&[[1., 1.8384, 0.]], "1 1.8383 0"),
        ["Error: Calculate point (1 1.8384 0) is not equal to target (1 1.8383 0)"]
    );
}

#[test]
fn all_actual_matching_rows_are_compared_in_rank_order() {
    assert_eq!(
        source_point_diagnostics(&[[1., 2., 3.], [1., 2., 4.], [0., 2., 3.]], "1 2 3"),
        [
            "Error: Calculate point (1 2 4) is not equal to target (1 2 3)",
            "Error: Calculate point (0 2 3) is not equal to target (1 2 3)"
        ]
    );
}

#[test]
fn empty_or_nonmatching_point_stream_does_not_invent_source_assertions() {
    assert!(source_point_diagnostics(&[], "1 2 3").is_empty());
    assert!(source_point_diagnostics(&[[f64::NAN, 2., 3.]], "1 2 3").is_empty());
    assert!(source_point_diagnostics(&[[1., f64::INFINITY, 3.]], "1 2 3").is_empty());
}

#[test]
fn signed_zero_is_not_rewritten_before_the_source_string_comparison() {
    assert_eq!(
        source_point_diagnostics(&[[1., 2., -0.]], "1 2 0"),
        ["Error: Calculate point (1 2 -0) is not equal to target (1 2 0)"]
    );
}
