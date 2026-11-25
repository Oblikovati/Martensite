//! Required ordered coverage is checked before a directed member can be retained.
use super::*;
use martensite_tkbo::body_check::{CheckMode, ValidationReport, ValidationStage};
pub(super) fn require(
    original: &DevelopedRegionRequest,
    report: &ValidationReport,
) -> ConeChecked<()> {
    if !admitted(report, original.tolerance) {
        return Err(storage::failure(
            original.region,
            None,
            ConeCause::ActualValidation(Box::new(report.clone())),
        ));
    }
    Ok(())
}
fn admitted(report: &ValidationReport, tolerance: f64) -> bool {
    let stages = [
        ValidationStage::Topology,
        ValidationStage::Geometry,
        ValidationStage::SelfIntersection,
        ValidationStage::Tolerance,
    ];
    report.is_valid()
        && report
            .stages
            .iter()
            .zip(stages)
            .all(|(actual, stage)| actual.stage == stage && actual.findings.is_empty())
        && report.stage(ValidationStage::Topology).mode == CheckMode::Exact
        && report.stage(ValidationStage::Geometry).mode == CheckMode::Sampled
        && report.stage(ValidationStage::SelfIntersection).mode == CheckMode::Exact
        && report.tolerance_budget.map(f64::to_bits) == Some(tolerance.to_bits())
        && report.census.len() == 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkbo::body_check::{EulerCensus, StageReport, StageVerdict, TopologyRef};
    use martensite_tkg3d::top_abs::ShapeEnum;
    fn report() -> ValidationReport {
        ValidationReport {
            stages: stages(),
            census: vec![census()],
            tolerance_budget: Some(1e-4),
        }
    }
    fn stages() -> [StageReport; 4] {
        [
            ValidationStage::Topology,
            ValidationStage::Geometry,
            ValidationStage::SelfIntersection,
            ValidationStage::Tolerance,
        ]
        .map(|stage| StageReport {
            stage,
            verdict: StageVerdict::Passed,
            findings: Vec::new(),
            mode: if stage == ValidationStage::Geometry {
                CheckMode::Sampled
            } else {
                CheckMode::Exact
            },
        })
    }
    fn census() -> EulerCensus {
        EulerCensus {
            solid: TopologyRef {
                kind: ShapeEnum::Solid,
                index: Some(0),
            },
            vertices: 8,
            edges: 12,
            faces: 6,
            loops: 6,
            shells: 1,
            genus: Some(0),
        }
    }

    #[test]
    fn directed_policy_requires_exact_si_without_promoting_sampled_geometry() {
        let mut report = report();
        assert!(admitted(&report, 1e-4));
        assert!(!report.is_exact());
        report.stages[2].mode = CheckMode::NativeSearch;
        assert!(!admitted(&report, 1e-4));
        report.stages[2].mode = CheckMode::Exact;
        report.stages[2].verdict = StageVerdict::Inconclusive;
        assert!(!admitted(&report, 1e-4));
    }
    #[test]
    fn missing_reordered_or_wrong_budget_reports_do_not_publish() {
        let mut missing = report();
        missing.stages[3].verdict = StageVerdict::NotRun;
        assert!(!admitted(&missing, 1e-4));
        let mut reordered = report();
        reordered.stages.swap(0, 1);
        assert!(!admitted(&reordered, 1e-4));
        assert!(!admitted(&report(), 1e-5));
        let mut absent = report();
        absent.census.clear();
        assert!(!admitted(&absent, 1e-4));
    }
}
