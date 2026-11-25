use super::*;
fn originals(request: &WrappedConeNormalRequest) -> Vec<DevelopedRegionRequest> {
    input::capture(request, &mut || false).unwrap().originals
}
#[test]
fn chart_receipt_retains_source_policy_and_exactly_once_work() {
    let request = super::super::tests::request();
    let originals = originals(&request);
    let mut work = storage::Work::new(request.proof.arithmetic);
    let domain = certify(
        &ConeRun::from(&request),
        &originals,
        WrappedConeDomainPolicy::PositiveXCommonChartV2,
        &mut work,
        &mut || false,
    )
    .unwrap();
    let proof = domain.chart().unwrap();
    assert_chart_receipt(&request, &domain, proof, &work);
    assert_binding(&originals, proof);
}
fn assert_chart_receipt(
    request: &WrappedConeNormalRequest,
    domain: &DomainEvidence,
    proof: &ConicalChartDomainEvidence,
    work: &storage::Work,
) {
    assert_eq!(
        domain.policy(),
        WrappedConeDomainPolicy::PositiveXCommonChartV2
    );
    assert_eq!(proof.options(), request.proof.arithmetic);
    assert_eq!(proof.operations(), work.consumed);
    assert_eq!([proof.developed_turn(), proof.physical_turn()], [0, 0]);
    assert_eq!(
        proof.fit_center().to_bits(),
        request.support.chart_origin.to_bits()
    );
}
fn assert_binding(originals: &[DevelopedRegionRequest], proof: &ConicalChartDomainEvidence) {
    let spans = original_spans(originals);
    let mut query = ConicalChartDomainQuery {
        spans: &spans,
        semi_angle: originals_angle(),
        chart_origin: proof.chart_origin(),
        fit_center: proof.fit_center(),
        developed_turn: 0,
        physical_turn: 0,
        options: proof.options(),
    };
    assert!(proof.matches(&query));
    query.physical_turn = 1;
    assert!(!proof.matches(&query));
    query.physical_turn = 0;
    query.fit_center = proof.fit_center().next_up();
    assert!(!proof.matches(&query));
    query.fit_center = proof.fit_center();
    query.spans = &spans[1..];
    assert!(!proof.matches(&query));
}
fn originals_angle() -> f64 {
    super::super::tests::request().support.semi_angle
}
#[test]
fn domain_typed_refusal_precedes_accounting_failure_and_keeps_offender() {
    let request = super::super::tests::request();
    let originals = originals(&request);
    let span = originals[0].loops[0][0].span;
    let cause = ConicalChartDomainError {
        cause: ConicalChartDomainFailure::Sector(ConicalSectorFailure::Point {
            span: 0,
            pole: 0,
            value: [f64::NAN, 0.],
        }),
        operations: 1,
    };
    let mut work = storage::Work::new(ConeClearanceOptions {
        max_operations: 0,
        ..request.proof.arithmetic
    });
    let failure = charged(Err(cause), &originals, &mut work).err().unwrap();
    assert_eq!(failure.region, Some(originals[0].region));
    assert_eq!(failure.span, Some(span));
    assert!(matches!(failure.cause, ConeCause::Domain(_)));
}

#[test]
fn chart_boundary_reuse_refuses_missing_or_changed_domain_authority() {
    let (inputs, mut selection) = chart_selection();
    let prepared = &mut selection.prepared[0];
    assert!(matches_first(prepared, &inputs.originals[0]));
    let authority = prepared.chart_domain.take().unwrap();
    assert!(!matches_first(prepared, &inputs.originals[0]));
    prepared.chart_domain = Some(authority);
    let changed = changed_domain(&inputs.originals, prepared);
    prepared.chart_domain = Some(std::sync::Arc::new(changed));
    assert!(!matches_first(prepared, &inputs.originals[0]));
}
fn chart_selection() -> (Inputs, Selection) {
    let request = super::super::tests::request();
    let inputs = input::capture(&request, &mut || false).unwrap();
    let mut work = storage::Work::new(request.proof.arithmetic);
    let selected = prepare::select_with_policy(
        &ConeRun::from(&request),
        &inputs.originals,
        WrappedConeDomainPolicy::PositiveXCommonChartV2,
        &mut work,
        &mut || false,
    )
    .unwrap();
    (inputs, selected)
}
fn matches_first(prepared: &Prepared, original: &DevelopedRegionRequest) -> bool {
    let piece = &prepared.source.loops[0][0];
    let fit = &prepared.fits[&piece.span];
    let curves = fit.poles.each_ref().map(|curve| {
        crate::surface_sketch::wrapped_profile::storage::OwnedCurve {
            poles: curve.points.clone(),
            weights: curve.weights.clone(),
        }
    });
    let query = storage::boundary_query(original, prepared, piece, &curves).unwrap();
    prepared
        .boundary_metric(piece.span, fit.local_chart)
        .unwrap()
        .matches(&fit.boundary, &query)
}
fn changed_domain(
    originals: &[DevelopedRegionRequest],
    prepared: &Prepared,
) -> ConicalChartDomainEvidence {
    let original = prepared.chart_domain.as_deref().unwrap();
    let spans = original_spans(originals);
    let mut options = original.options();
    options.max_operations -= 1;
    certify_conical_chart_domain(
        ConicalChartDomainQuery {
            spans: &spans,
            semi_angle: prepared.semi_angle,
            chart_origin: original.chart_origin(),
            fit_center: original.fit_center(),
            developed_turn: original.developed_turn(),
            physical_turn: original.physical_turn(),
            options,
        },
        || false,
    )
    .unwrap()
}
