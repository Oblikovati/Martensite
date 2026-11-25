use super::*;

#[test]
fn rounded_v2_selection_replays_original_bytes_history_budget_and_callbacks() {
    let input = request();
    let original = input.clone();
    let (previous, previous_polls) = previous(&input);
    let selection = WrappedConeExecutionSelection::RoundedProxyNativeSearchV1 {
        domain: WrappedConeDomainPolicy::PositiveXCommonChartV2,
    };
    let (selected, selected_polls) = selected(&input, selection);
    assert_eq!(selected_polls, previous_polls);
    assert_eq!(previous.execution_selection(), selection);
    assert_eq!(selected.execution_selection(), selection);
    assert_replay(&input, &previous, &selected);
    assert_eq!(input, original);
}
fn previous(input: &WrappedConeNormalRequest) -> (WrappedConeNormalResult, usize) {
    let mut polls = 0;
    let result = normal_extrude_cone_profile_with_domain_policy_and_cancel(
        input,
        WrappedConeDomainPolicy::PositiveXCommonChartV2,
        || {
            polls += 1;
            false
        },
    )
    .unwrap_or_else(|cause| panic!("original rounded V2: {cause:?}"));
    (result, polls)
}
fn selected(
    input: &WrappedConeNormalRequest,
    selection: WrappedConeExecutionSelection,
) -> (WrappedConeNormalResult, usize) {
    let mut polls = 0;
    let result = normal_extrude_cone_profile_with_selection_and_cancel(input, selection, || {
        polls += 1;
        false
    })
    .unwrap_or_else(|cause| panic!("selected rounded V2: {cause:?}"));
    (result, polls)
}
fn assert_replay(
    input: &WrappedConeNormalRequest,
    previous: &WrappedConeNormalResult,
    selected: &WrappedConeNormalResult,
) {
    assert_eq!(selected.encoded_brep(), previous.encoded_brep());
    assert_eq!(selected.history(), previous.history());
    assert_eq!(selected.budget(), previous.budget());
    assert_eq!(selected.domain_policy(), previous.domain_policy());
    assert!(previous.actual_self_intersection_coverage().is_none());
    assert!(selected.actual_self_intersection_coverage().is_none());
    assert_eq!(selected.support(), input.support);
    assert_eq!(selected.depths(), input.depths);
    assert_eq!(selected.options(), input.options);
    assert_eq!(selected.proof_options(), input.proof);
}
