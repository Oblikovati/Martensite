use super::*;
use state_rows::{FrustumCriterionText, frustum_criterion_text, frustum_point_from_row};

fn criterion_fixture() -> FrustumCriterionText {
    FrustumCriterionText {
        depth: 1.807106781,
        distance: 0.0000123,
        point: [1.807106781, -0.0000123, -0.],
        normal: [0.; 3],
    }
}

fn wire_rows_fixture(edge_first: bool) -> Vec<String> {
    let edge = [
        "  e Depth: 0.5 Distance: 2 Point: 10 3 0 (Select3DSensitiveSegment)",
        "                       Detected Shape: BRep_TEdge",
    ];
    let wire = [
        "  b_5 Depth: 0.5 Distance: 9 Point: 10 3 0 (Select3DSensitiveWire)",
        "                       Detected Shape: TopoDS_TWire",
        "                       Detected Child: Select3DSensitiveSegment",
    ];
    let mut rows = vec!["Detected entities:".to_owned()];
    let groups: [&[&str]; 2] = if edge_first {
        [&edge, &wire]
    } else {
        [&wire, &edge]
    };
    rows.extend(groups.into_iter().flatten().map(|row| (*row).to_owned()));
    rows
}

#[test]
fn source_percent_g_roundtrip_keeps_six_digits_exponents_and_negative_zero() {
    let hit = criterion_fixture();
    let text = frustum_criterion_text(hit);
    assert_eq!(
        text,
        " Depth: 1.80711 Distance: 1.23e-05 Point: 1.80711 -1.23e-05 -0"
    );
    let point = frustum_point_from_row(&text).unwrap();
    assert_eq!(point[0], 1.80711);
    assert_ne!(point[0].to_bits(), hit.point[0].to_bits());
    assert_eq!(point[1], -0.0000123);
    assert_eq!(point[2].to_bits(), (-0_f64).to_bits());
}

#[test]
fn normal_rows_preserve_native_presence_threshold_and_promoted_float_values() {
    let mut hit = criterion_fixture();
    hit.normal = [0., 0., 0.00001];
    assert!(!frustum_criterion_text(hit).contains(" Normal:"));
    hit.normal = [0., 0., 1.];
    assert!(frustum_criterion_text(hit).ends_with(" Normal: 0 0 1"));
}

#[test]
fn missing_or_malformed_point_tokens_refuse_without_comment_coordinate_fallback() {
    for row in ["no Point tokens", "Point: 1 2", "Point: nope 2 3"] {
        let error = frustum_point_from_row(row).unwrap_err();
        assert!(error.to_string().contains("expected"));
    }
}

#[test]
fn real_string_comparison_ignores_distance_and_preserves_both_rank_orders() {
    for edge_first in [true, false] {
        let rows = wire_rows_fixture(edge_first);
        assert!(wire_depth::frustum_wire_diagnostics(&rows).is_empty());
        let parsed = wire_depth::frustum_parse_characteristics(&rows[1]);
        assert_eq!(&parsed[1..3], ["0.5", "10 3 0"]);
    }
}

#[test]
fn characteristic_failures_keep_both_case_messages_before_type_checks() {
    let mut rows = wire_rows_fixture(true);
    rows[3] = rows[3]
        .replace("Depth: 0.5", "Depth: 0.6")
        .replace("10 3 0", "10 4 0");
    let lines = wire_depth::frustum_wire_diagnostics(&rows);
    assert_eq!(
        lines,
        [
            "Characteristics are not equal at value nb: 2",
            "The values are: 0.5 and 0.6",
            "ERROR",
            "",
            "Characteristics are not equal at value nb: 3",
            "The values are: 10 3 0 and 10 4 0",
            "ERROR",
            "",
        ]
    );
}

#[test]
fn all_five_type_predicates_refuse_changed_sensitive_shape_or_child_rows() {
    let missing = state_rows::frustum_runtime_name(None, "missing-sensitive").unwrap_err();
    assert!(missing.message().contains("runtime type=None"));
    for index in [1, 2, 3, 4, 5] {
        let mut rows = wire_rows_fixture(true);
        rows[index] = rows[index]
            .replace("Sensitive", "WrongSensitive")
            .replace("Shape:", "WrongShape:");
        let lines = wire_depth::frustum_wire_diagnostics(&rows);
        assert!(
            lines.iter().any(|line| line == "ERROR"),
            "changed row {index}"
        );
        assert!(lines.iter().any(|line| line.starts_with("Must be:")));
    }
}

#[test]
fn missing_wire_child_cannot_be_silently_reported_as_expected_segment() {
    let rows = wire_rows_fixture(true);
    let missing = &rows[..rows.len() - 1];
    let lines = wire_depth::frustum_wire_diagnostics(missing);
    assert_eq!(lines[0], "Wrong type of wire's inner sensitive! Value is: ");
    assert_eq!(lines.last().unwrap(), "ERROR");
}
