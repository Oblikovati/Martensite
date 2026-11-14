//! Require the existing ordered native pass without relabelling sampled or searched evidence.
use super::*;
use martensite_tkbo::body_check::{
    CheckMode, StageReport, StageVerdict, ValidationReport, ValidationStage,
};
const STAGES: [ValidationStage; 4] = [
    ValidationStage::Topology,
    ValidationStage::Geometry,
    ValidationStage::SelfIntersection,
    ValidationStage::Tolerance,
];
const MODES: [(ValidationStage, CheckMode); 3] = [
    (ValidationStage::Topology, CheckMode::Exact),
    (ValidationStage::Geometry, CheckMode::Sampled),
    (ValidationStage::SelfIntersection, CheckMode::NativeSearch),
];
pub(super) fn certify(
    region: SurfaceRegionId,
    report: &ValidationReport,
    tolerance: f64,
) -> Checked<()> {
    for (actual, expected) in report.stages.iter().zip(STAGES) {
        stage_pass(region, actual, expected)?;
    }
    if report.tolerance_budget.map(f64::to_bits) != Some(tolerance.to_bits()) {
        return Err(failure(region, ValidationStage::Tolerance));
    }
    if report.census.len() != 1 {
        return Err(failure(region, ValidationStage::Topology));
    }
    for (stage, mode) in MODES {
        require_mode(region, report, stage, mode)?;
    }
    Ok(())
}
fn stage_pass(
    region: SurfaceRegionId,
    actual: &StageReport,
    expected: ValidationStage,
) -> Checked<()> {
    if actual.stage != expected
        || actual.verdict != StageVerdict::Passed
        || !actual.findings.is_empty()
    {
        return Err(failure(region, expected));
    }
    Ok(())
}
fn require_mode(
    region: SurfaceRegionId,
    report: &ValidationReport,
    stage: ValidationStage,
    mode: CheckMode,
) -> Checked<()> {
    if report.stage(stage).mode != mode {
        return Err(failure(region, stage));
    }
    Ok(())
}
fn failure(region: SurfaceRegionId, stage: ValidationStage) -> BudgetFailure {
    BudgetFailure::Native { region, stage }
}
