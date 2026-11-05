//! Remaining complete original numeric families, replayed through the shared operation registry.
#[allow(dead_code, unused_imports)]
#[path = "replay_operations/mod.rs"]
pub(super) mod replay_operations;
use replay_operations::buffered_tokens::BufferedTokens;
#[test]
fn brep_algo_0() {
    replay_fixture(include_str!("fixtures/full_numeric/brep_algo_0.tsv"), 78);
}
#[test]
fn brep_fill_advanced_evolved_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_advanced_evolved_0.tsv"),
        14,
    );
}
#[test]
fn brep_fill_compatible_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_compatible_0.tsv"),
        51,
    );
}
#[test]
fn brep_fill_draft_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_draft_0.tsv"),
        36,
    );
}
#[test]
fn brep_fill_evolved_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_evolved_0.tsv"),
        67,
    );
}
#[test]
fn brep_fill_filling_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_filling_0.tsv"),
        16,
    );
}
#[test]
fn brep_fill_generator_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_generator_0.tsv"),
        64,
    );
}
#[test]
fn brep_fill_location_law_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_location_law_0.tsv"),
        28,
    );
}
#[test]
fn brep_fill_multi_line_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_multi_line_0.tsv"),
        16,
    );
}
#[test]
fn brep_fill_offset_wire_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_offset_wire_0.tsv"),
        60,
    );
}
#[test]
fn brep_fill_package_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_package_0.tsv"),
        124,
    );
}
#[test]
fn brep_fill_pipe_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_pipe_0.tsv"),
        33,
    );
}
#[test]
fn brep_fill_pipe_shell_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_pipe_shell_0.tsv"),
        6,
    );
}
#[test]
fn brep_fill_pipe_shell_1() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_pipe_shell_1.tsv"),
        6,
    );
}
#[test]
fn brep_fill_pipe_shell_2() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_pipe_shell_2.tsv"),
        6,
    );
}
#[test]
fn brep_fill_pipe_shell_3() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_pipe_shell_3.tsv"),
        6,
    );
}
#[test]
fn brep_fill_pipe_shell_4() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_pipe_shell_4.tsv"),
        6,
    );
}
#[test]
fn brep_fill_pipe_shell_5() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_pipe_shell_5.tsv"),
        6,
    );
}
#[test]
fn brep_fill_pipe_shell_6() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_pipe_shell_6.tsv"),
        6,
    );
}
#[test]
fn brep_fill_pipe_shell_7() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_pipe_shell_7.tsv"),
        6,
    );
}
#[test]
fn brep_fill_pipe_shell_8() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_pipe_shell_8.tsv"),
        6,
    );
}
#[test]
fn brep_fill_section_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_section_0.tsv"),
        42,
    );
}
#[test]
fn brep_fill_section_law_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_section_law_0.tsv"),
        42,
    );
}
#[test]
fn brep_fill_sweep_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/brep_fill_sweep_0.tsv"),
        78,
    );
}
#[test]
fn brep_proj_0() {
    replay_fixture(include_str!("fixtures/full_numeric/brep_proj_0.tsv"), 19);
}
#[test]
fn top_ope_brep_build_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/top_ope_brep_build_0.tsv"),
        50,
    );
}
#[test]
fn top_ope_brep_build_tools_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/top_ope_brep_build_tools_0.tsv"),
        19,
    );
}
#[test]
fn top_ope_brep_ds_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/top_ope_brep_ds_0.tsv"),
        164,
    );
}
#[test]
fn top_ope_brep_ds_build_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/top_ope_brep_ds_build_0.tsv"),
        54,
    );
}
#[test]
fn top_ope_brep_ds_ops_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/top_ope_brep_ds_ops_0.tsv"),
        400,
    );
}
#[test]
fn top_ope_brep_ds_ops_1() {
    replay_fixture(
        include_str!("fixtures/full_numeric/top_ope_brep_ds_ops_1.tsv"),
        296,
    );
}
#[test]
fn top_ope_brep_tool_classifier_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/top_ope_brep_tool_classifier_0.tsv"),
        27,
    );
}
#[test]
fn top_ope_brep_tool_curve_tool_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/top_ope_brep_tool_curve_tool_0.tsv"),
        43,
    );
}
#[test]
fn top_ope_brep_tool_regu_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/top_ope_brep_tool_regu_0.tsv"),
        68,
    );
}
#[test]
fn top_ope_brep_tool_shapes_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/top_ope_brep_tool_shapes_0.tsv"),
        33,
    );
}
#[test]
fn top_ope_brep_tool_tool_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/top_ope_brep_tool_tool_0.tsv"),
        260,
    );
}
#[test]
fn top_ope_brep_tool_topology_0() {
    replay_fixture(
        include_str!("fixtures/full_numeric/top_ope_brep_tool_topology_0.tsv"),
        164,
    );
}

fn replay_fixture(fixture: &str, expected_count: usize) {
    assert_eq!(fixture.lines().count(), expected_count);
    for row in fixture.lines() {
        let (request, expected) = row.split_once('\t').expect("request and expected tokens");
        let actual = replay_request(request);
        assert_eq!(
            actual
                .split_whitespace()
                .map(canonical_token)
                .collect::<Vec<_>>(),
            expected
                .split_whitespace()
                .map(canonical_token)
                .collect::<Vec<_>>(),
            "{request}"
        );
    }
}
pub(super) fn replay_request(request: &str) -> String {
    let mut fields = request.split_whitespace();
    let operation = fields.next().expect("nonempty original request");
    let values: Vec<f64> = fields
        .map(|field| field.parse().expect("numeric field"))
        .collect();
    let (_, case) = replay_operations::cases::PROBE_CASES
        .iter()
        .find(|(name, _)| *name == operation)
        .expect("known original operation");
    let output = BufferedTokens::captured();
    case(&output, &values);
    output.finish()
}

fn canonical_token(token: &str) -> &str {
    let Ok(bits) = token.parse::<u64>() else {
        return token;
    };
    if token.len() >= 16 && f64::from_bits(bits).is_nan() {
        return "NaN";
    }
    token
}
#[test]
fn numeric_comparison_preserves_finite_bits_and_classifies_nan_payloads() {
    assert_eq!(canonical_token("9221120237041090560"), "NaN");
    assert_eq!(canonical_token("18444492273895866369"), "NaN");
    assert_eq!(
        canonical_token("9223372036854775808"),
        "9223372036854775808"
    );
    assert_eq!(
        canonical_token("9218868437227405312"),
        "9218868437227405312"
    );
    assert_eq!(
        canonical_token("4607182418800017408"),
        "4607182418800017408"
    );
    assert_eq!(canonical_token("Standard_Failure"), "Standard_Failure");
}
