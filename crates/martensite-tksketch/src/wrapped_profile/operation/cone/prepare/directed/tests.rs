use super::*;
#[test]
fn directed_mode_requires_actual_requested_program_material_authority() {
    let request = super::super::super::tests::request();
    let failure = admit(
        &ConeRun::from(&request),
        WrappedConeExecutionSelection::DirectedCommonChartRequireCertifiedV1,
    )
    .unwrap_err();
    assert_eq!(failure.stage, WrappedStage::Admission);
    assert_eq!(failure.class, WrappedFailureClass::Unsupported);
    assert!(
        matches!(failure.cause, ConeCause::ExecutionProof(actual) if *actual == request.proof.planar_proof)
    );
}
#[test]
fn rounded_modes_preserve_their_existing_planar_policy_admission() {
    let request = super::super::super::tests::request();
    for domain in [
        WrappedConeDomainPolicy::LegacySectorV1,
        WrappedConeDomainPolicy::PositiveXCommonChartV2,
    ] {
        assert!(
            admit(
                &ConeRun::from(&request),
                WrappedConeExecutionSelection::RoundedProxyNativeSearchV1 { domain }
            )
            .is_ok()
        );
    }
}
#[test]
fn actual_original_refusal_outranks_later_ledger_failure() {
    let options = ConeClearanceOptions {
        max_operations: 1,
        ..Default::default()
    };
    let query = empty_query(options);
    let result = certify_conical_directed_gamma(query, || false);
    assert!(result.is_err());
    let mut work = Work::new(options);
    let charged = work.charge(SurfaceRegionId(9), None, 2);
    let inventory = Inventory {
        sides: BTreeMap::new(),
        caps: BTreeMap::new(),
    };
    let failure = gamma::finish(result, charged, query, &inventory).unwrap_err();
    assert!(
        matches!(&failure.cause, ConeCause::ActualPatch(cause) if matches!(cause.cause.as_ref(), ConicalActualPatchFailure::Shape { actual: 0, .. }))
    );
    assert_eq!(failure.public().class(), WrappedFailureClass::Unsupported);
    assert_eq!(work.consumed, 0);
}

fn empty_query(options: ConeClearanceOptions) -> ConicalDirectedGammaQuery<'static> {
    ConicalDirectedGammaQuery {
        spans: &[],
        caps: &[],
        refinement: ConicalPrimitiveRefinement::HalfSplitV1,
        options,
    }
}

#[test]
fn exhausted_valid_context_refuses_before_lending_a_zero_math_policy() {
    let options = ConeClearanceOptions {
        max_operations: 1,
        ..Default::default()
    };
    let mut work = Work::new(options);
    let region = SurfaceRegionId(7);
    work.charge(region, None, 1).unwrap();
    let failure = work.remaining_checked(region, None).unwrap_err();
    assert_eq!(failure.region, Some(region));
    assert_eq!(failure.public().class(), WrappedFailureClass::Incomplete);
    assert_eq!(work.consumed, 1);
}
