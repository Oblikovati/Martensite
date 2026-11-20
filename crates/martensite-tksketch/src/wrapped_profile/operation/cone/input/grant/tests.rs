use super::*;

fn selection() -> WrappedConeExecutionSelection {
    WrappedConeExecutionSelection::DirectedFamilyChartRequireCertifiedV2 {
        arithmetic: WrappedConeArithmeticGrantPolicy::InputBoundV1,
    }
}
fn captured() -> (WrappedConeNormalRequest, Inputs) {
    let request = crate::surface_sketch::wrapped_profile::operation::cone::tests::request();
    let inputs = super::super::capture(&request, &mut || false).unwrap();
    (request, inputs)
}

#[test]
fn allowance_has_original_floor_input_scaling_and_hard_ceiling() {
    assert_eq!(input_ceiling(1, 0).unwrap(), 1_000_000);
    assert_eq!(input_ceiling(239, 8).unwrap(), 8_810_496);
    assert_eq!(input_ceiling(4096, 8).unwrap(), 16_000_000);
    assert!(input_ceiling(239, 9).is_err());
    assert!(input_ceiling(u32::MAX, 8).is_err());
}

#[test]
fn source_count_counts_occurrences_and_refuses_empty_or_oversized_input() {
    let (_, mut inputs) = captured();
    assert_eq!(source_count(&inputs.originals).unwrap(), 8);
    let piece = inputs.originals[0].loops[0][0].clone();
    inputs.originals[0].loops = vec![vec![piece.clone(); 4096]];
    assert_eq!(source_count(&inputs.originals).unwrap(), 4096);
    inputs.originals[0].loops[0].push(piece);
    assert!(source_count(&inputs.originals).is_err());
    assert!(source_count(&[]).is_err());
}

#[test]
fn original_selection_keeps_old_admission_and_no_extra_source_snapshot() {
    let (request, inputs) = captured();
    let original = WrappedConeExecutionSelection::DirectedCommonChartRequireCertifiedV1;
    assert_eq!(admission_ceiling(original), 1_000_000);
    assert!(
        capture(&request, original, &inputs.originals)
            .unwrap()
            .is_none()
    );
    assert_eq!(admission_ceiling(selection()), 16_000_000);
}

#[test]
fn requested_aggregate_is_preserved_and_never_clamped_to_input_ceiling() {
    let (mut request, inputs) = captured();
    request.proof.arithmetic.max_operations = 1_000_000;
    let grant = capture(&request, selection(), &inputs.originals)
        .unwrap()
        .unwrap();
    assert_eq!(grant.receipt.requested_operations, 1_000_000);
    assert_eq!(grant.receipt.input_ceiling, 1_000_000);
    request.proof.arithmetic.max_operations += 1;
    assert!(capture(&request, selection(), &inputs.originals).is_err());
    assert_eq!(request.proof.arithmetic.max_operations, 1_000_001);
}

#[test]
fn snapshot_rejects_raw_source_edits_order_and_loop_partition_changes() {
    let (request, mut inputs) = captured();
    let grant = capture(&request, selection(), &inputs.originals)
        .unwrap()
        .unwrap();
    grant
        .validate(&request, selection(), &inputs.originals)
        .unwrap();
    inputs.originals[0].loops[0].swap(0, 1);
    assert!(
        grant
            .validate(&request, selection(), &inputs.originals)
            .is_err()
    );
    inputs.originals[0].loops[0].swap(0, 1);
    let tail = inputs.originals[0].loops[0].split_off(2);
    inputs.originals[0].loops.insert(1, tail);
    assert!(
        grant
            .validate(&request, selection(), &inputs.originals)
            .is_err()
    );
    let tail = inputs.originals[0].loops.remove(1);
    inputs.originals[0].loops[0].extend(tail);
    inputs.originals[0].loops[0][0].poles[0][0] = 4_f64.next_up();
    assert!(
        grant
            .validate(&request, selection(), &inputs.originals)
            .is_err()
    );
}

#[test]
fn snapshot_rejects_selector_limits_and_source_metadata_mutations() {
    let (mut request, mut inputs) = captured();
    let grant = capture(&request, selection(), &inputs.originals)
        .unwrap()
        .unwrap();
    let original = WrappedConeExecutionSelection::DirectedCommonChartRequireCertifiedV1;
    assert!(
        grant
            .validate(&request, original, &inputs.originals)
            .is_err()
    );
    request.options.maximum_fit_level -= 1;
    assert!(
        grant
            .validate(&request, selection(), &inputs.originals)
            .is_err()
    );
    request.options.maximum_fit_level += 1;
    inputs.originals[0].depths[0] = inputs.originals[0].depths[0].next_up();
    assert!(
        grant
            .validate(&request, selection(), &inputs.originals)
            .is_err()
    );
}

#[test]
fn receipt_and_selector_are_shareable_immutable_values() {
    static_assertions::assert_impl_all!(WrappedConeArithmeticGrantReceipt: Send, Sync);
    static_assertions::assert_impl_all!(WrappedConeArithmeticGrantPolicy: Send, Sync);
    let (request, inputs) = captured();
    let grant = capture(&request, selection(), &inputs.originals)
        .unwrap()
        .unwrap();
    let mut receipt = grant.receipt;
    receipt.requested_operations = 0;
    assert_ne!(receipt, grant.receipt);
    grant
        .validate(&request, selection(), &inputs.originals)
        .unwrap();
}

#[test]
fn invalid_input_bound_proof_options_report_the_selected_ceiling() {
    let mut proof = WrappedConeProofOptions::default();
    proof.arithmetic.max_operations = 16_000_001;
    assert_ceiling_diagnostic(proof);
    proof.arithmetic.max_operations = 8_810_496;
    proof.arithmetic.fractional_bits = 7;
    assert_ceiling_diagnostic(proof);
}
fn assert_ceiling_diagnostic(proof: WrappedConeProofOptions) {
    let failure = super::super::policy::admit_with_ceiling(proof, admission_ceiling(selection()))
        .unwrap_err()
        .public();
    assert_eq!(failure.stage(), WrappedStage::Admission);
    assert_eq!(failure.class(), WrappedFailureClass::InvalidInput);
    assert!(
        failure
            .to_string()
            .contains("aggregate arithmetic <=16000000")
    );
}
