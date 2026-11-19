//! #560: non-passing reports cannot publish otherwise genuine legacy cone evidence.
use super::*;
use crate::surface_sketch::HistorySource;
use crate::surface_sketch::wrapped_profile::{family::Control, operation as driver};
use driver::stages::{OwnedFamily, SupportStages};
use martensite_tkbo::body_check::{CheckMode, StageVerdict, ValidationStage};

type Family = OwnedFamily<Selection, Geometry>;

#[test]
fn nonpassing_native_reports_cannot_complete_legacy_cone_publication() {
    let cases = [
        (ValidationStage::Geometry, StageVerdict::Failed),
        (
            ValidationStage::SelfIntersection,
            StageVerdict::Inconclusive,
        ),
        (ValidationStage::Tolerance, StageVerdict::NotRun),
    ];
    let published = cases
        .into_iter()
        .filter(|(stage, verdict)| observed_case(*stage, *verdict))
        .count();
    assert_eq!(
        published, 0,
        "non-passing native reports reached successful publication"
    );
}

fn observed_case(stage: ValidationStage, verdict: StageVerdict) -> bool {
    let request = driver::cone::tests::request();
    let inputs = input::capture(&request, &mut || false).unwrap();
    let (mut family, mut context) = constructed(&request, &inputs);
    baseline_report(&family);
    inject_report(&mut family, stage, verdict);
    let result = budget_and_publish(&request, inputs, family, &mut context);
    observation(stage, verdict, result)
}

fn constructed(request: &WrappedConeNormalRequest, inputs: &Inputs) -> (Family, storage::Work) {
    let support = legacy_stages(request);
    support.limits(&inputs.originals).unwrap();
    let mut context = support.context();
    let mut poll = |_| false;
    let mut control = Control::new(&mut poll);
    let family = driver::execute::construct_with_context(
        &support,
        &inputs.originals,
        &mut control,
        &mut context,
    )
    .unwrap();
    (family, context)
}

fn baseline_report(family: &Family) {
    let report = &family.members[&SurfaceRegionId(1)].native.report;
    assert!(
        report.is_valid(),
        "baseline report was already non-passing: {report:?}"
    );
    assert!(!report.is_exact());
    assert_eq!(
        report.stage(ValidationStage::SelfIntersection).mode,
        CheckMode::NativeSearch
    );
    eprintln!(
        r#"{{"event":"native_report_baseline","valid":true,"exact":false,"si_mode":"NativeSearch"}}"#
    );
}

fn inject_report(family: &mut Family, stage: ValidationStage, verdict: StageVerdict) {
    let member = family.members.get_mut(&SurfaceRegionId(1)).unwrap();
    let report = &mut member.native.report;
    report.stages[stage as usize].verdict = verdict;
    for later in report.stages.iter_mut().skip(stage as usize + 1) {
        later.verdict = StageVerdict::NotRun;
    }
    assert!(!report.is_valid());
}

fn budget_and_publish(
    request: &WrappedConeNormalRequest,
    inputs: Inputs,
    family: Family,
    context: &mut storage::Work,
) -> Checked<WrappedConeNormalResult> {
    let support = legacy_stages(request);
    let expected = family.members[&SurfaceRegionId(1)].native.report.clone();
    let prior_work = (context.consumed, context.integration_cells);
    let budget = support
        .budget(&inputs.originals, &family, (), context, &mut || false)
        .map_err(|cause| {
            assert_eq!((context.consumed, context.integration_cells), prior_work);
            retained_failure(cause, &expected)
        })?;
    publish(
        request,
        inputs,
        ConeBudgeted { family, budget },
        &mut || false,
    )
}

fn observation(
    stage: ValidationStage,
    verdict: StageVerdict,
    result: Checked<WrappedConeNormalResult>,
) -> bool {
    match result {
        Ok(result) => published_report(stage, verdict, result),
        Err(cause) => {
            assert_eq!(cause.class(), WrappedFailureClass::Incomplete);
            assert!(
                cause
                    .entities()
                    .contains(&HistorySource::Region(SurfaceRegionId(1)))
            );
            eprintln!(
                r#"{{"event":"native_report_fault","stage":"{stage:?}","verdict":"{verdict:?}","published":false}}"#
            );
            false
        }
    }
}

fn published_report(
    stage: ValidationStage,
    verdict: StageVerdict,
    result: WrappedConeNormalResult,
) -> bool {
    let report = &result.reports()[&SurfaceRegionId(1)].validation;
    assert!(!report.is_valid());
    assert_eq!(report.stage(stage).verdict, verdict);
    eprintln!(
        r#"{{"event":"native_report_fault","stage":"{stage:?}","verdict":"{verdict:?}","published":true}}"#
    );
    true
}

fn legacy_stages(request: &WrappedConeNormalRequest) -> stages::ConeStages<'_> {
    stages::ConeStages {
        request,
        execution_selection: WrappedConeExecutionSelection::RoundedProxyNativeSearchV1 {
            domain: WrappedConeDomainPolicy::LegacySectorV1,
        },
    }
}

fn retained_failure(
    cause: ConeFailure,
    expected: &martensite_tkbo::body_check::ValidationReport,
) -> WrappedNormalFailure {
    assert_eq!(cause.region, Some(SurfaceRegionId(1)));
    assert_eq!(cause.class, WrappedFailureClass::Incomplete);
    match &cause.cause {
        ConeCause::ActualValidation(report) => assert_eq!(report.as_ref(), expected),
        other => panic!("expected retained ordered report, got {other:?}"),
    }
    cause.public()
}
