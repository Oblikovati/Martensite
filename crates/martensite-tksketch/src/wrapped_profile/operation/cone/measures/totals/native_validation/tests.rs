//! Verdict controls exercise public report values without manufacturing geometry evidence.
use super::*;
use martensite_tkbo::body_check::{CheckMode, StageReport, StageVerdict, ValidationStage};

fn passed_report() -> ValidationReport {
    let stages = [
        (ValidationStage::Topology, CheckMode::Exact),
        (ValidationStage::Geometry, CheckMode::Sampled),
        (ValidationStage::SelfIntersection, CheckMode::NativeSearch),
        (ValidationStage::Tolerance, CheckMode::Exact),
    ];
    ValidationReport {
        stages: stages.map(|(stage, mode)| passed_stage(stage, mode)),
        census: Vec::new(),
        tolerance_budget: Some(1e-4),
    }
}

fn passed_stage(stage: ValidationStage, mode: CheckMode) -> StageReport {
    StageReport {
        stage,
        verdict: StageVerdict::Passed,
        mode,
        findings: Vec::new(),
    }
}

#[test]
fn valid_native_search_report_keeps_its_nonexact_modes() {
    let report = passed_report();
    let before = report.clone();
    assert!(report.is_valid());
    assert!(!report.is_exact());
    require_report(SurfaceRegionId(7), &report).unwrap();
    assert_eq!(report, before);
}

#[test]
fn every_nonpassing_stage_retains_its_original_report_and_region() {
    for index in 0..4 {
        refusal_case(index, StageVerdict::Failed);
        refusal_case(index, StageVerdict::Inconclusive);
        refusal_case(index, StageVerdict::NotRun);
    }
}

fn refusal_case(index: usize, verdict: StageVerdict) {
    let mut report = passed_report();
    report.stages[index].verdict = verdict;
    let failure = require_report(SurfaceRegionId(7), &report).unwrap_err();
    assert_eq!(failure.region, Some(SurfaceRegionId(7)));
    assert_eq!(failure.class, WrappedFailureClass::Incomplete);
    assert_eq!(failure.stage, WrappedStage::Construction);
    match failure.cause {
        ConeCause::ActualValidation(actual) => assert_eq!(*actual, report),
        other => panic!("expected original report, got {other:?}"),
    }
}
