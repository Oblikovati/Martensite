//! Read raw values before a native self-check can overwrite them (#490).
use super::{
    CheckMode, Finding, FindingKind, Severity, StageReport, StageVerdict, TopologyRef,
    ValidationReport, ValidationStage, ledger::Ledger, naming::TopologyNames,
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

pub(super) fn inspect(
    names: &TopologyNames,
    root: TopologyRef,
    budget: Option<f64>,
) -> Option<ValidationReport> {
    let (findings, verdict) = match raw_findings(names) {
        Ok(findings) if findings.is_empty() => return None,
        Ok(findings) => (findings, StageVerdict::Failed),
        Err(cause) => (vec![unreadable(root, cause)], StageVerdict::Inconclusive),
    };
    let mut report = Ledger::new().finish(Vec::new(), budget);
    report.stages[ValidationStage::Tolerance as usize] = StageReport {
        stage: ValidationStage::Tolerance,
        verdict,
        mode: CheckMode::Exact,
        findings,
    };
    Some(report)
}
fn raw_findings(names: &TopologyNames) -> Result<Vec<Finding>, Failure> {
    let mut findings = Vec::new();
    for kind in [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face] {
        for shape in names.records(kind) {
            findings.extend(invalid_value(shape, names)?);
        }
    }
    Ok(findings)
}
fn invalid_value(
    shape: &martensite_tkbrep::topods::Shape,
    names: &TopologyNames,
) -> Result<Option<Finding>, Failure> {
    let tolerance = super::tolerance::raw_tolerance(shape)?;
    if tolerance.is_finite() && tolerance >= 0. {
        return Ok(None);
    }
    Ok(Some(Finding {
        severity: Severity::Defect,
        kind: FindingKind::InvalidTolerance { tolerance },
        subject: names.name(shape),
        context: None,
    }))
}

fn unreadable(root: TopologyRef, cause: Failure) -> Finding {
    Finding {
        severity: Severity::Inconclusive,
        kind: FindingKind::CheckerFailure {
            message: cause.message().to_owned(),
        },
        subject: root,
        context: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unreadable_storage_is_inconclusive_with_its_cause() {
        let root = TopologyRef {
            kind: ShapeEnum::Solid,
            index: Some(0),
        };
        let cause = Failure::with_type("Standard_TypeMismatch", "expected BRep vertex storage", "");
        let finding = unreadable(root, cause);
        assert_eq!(finding.subject, root);
        assert_eq!(finding.severity, Severity::Inconclusive);
        assert_eq!(
            finding.kind,
            FindingKind::CheckerFailure {
                message: "expected BRep vertex storage".to_owned(),
            }
        );
    }
}
