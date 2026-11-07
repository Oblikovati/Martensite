//! Raw execution cannot release an invalid or incompletely checked candidate.
use super::boolean_coverage::{CoverageStage, OperationCoverage};
use super::{boolean_history, boolean_measurements, boolean_validation, bop_options};
use martensite_tkbo::bop_algo::{BOPAlgoBOP, ProgressCompletion};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{message::ProgressRange, precision};

#[path = "boolean_refusal.rs"]
mod refusal;
pub(super) use refusal::{
    BooleanCause, BooleanIssue, BooleanRequest, KernelDiagnostics, LineageRetention,
    OperandAuthority, RefusalClass, execution_status, kernel_issue, refused,
};

pub(super) struct CheckedLocalBody {
    shape: Shape,
    history: boolean_history::LocalHistory,
    checks: boolean_validation::ValidationReceipt,
    measurements: boolean_measurements::MeasurementReceipt,
    diagnostics: KernelDiagnostics,
    coverage: super::boolean_coverage::OperationCoverage,
}

impl CheckedLocalBody {
    #[cfg(test)]
    pub(super) fn append_test_only_prior_reports(mut self, sentinels: KernelDiagnostics) -> Self {
        // Retain reports only after genuine acquisition; no fabricated geometry.
        self.diagnostics = self.diagnostics.followed_by(sentinels);
        self
    }

    pub(super) fn diagnostics(&self) -> &KernelDiagnostics {
        // A successful raw run may still carry native warnings.
        &self.diagnostics
    }
    pub(super) fn shape(&self) -> &Shape {
        // Only this local wrapper follows all checks required by its declared mode.
        &self.shape
    }
    pub(super) fn into_shape(self) -> Shape {
        // Consuming retires the wrapper before its geometry leaves this construction step.
        self.shape
    }
    pub(super) fn history(&self) -> &boolean_history::LocalHistory {
        // Native handles stay inside the unpublished construction arena.
        &self.history
    }
    pub(super) fn checks(&self) -> &boolean_validation::ValidationReceipt {
        // The local check mode is not an exact-geometry certificate.
        &self.checks
    }
    pub(super) fn measurements(&self) -> &boolean_measurements::MeasurementReceipt {
        // Estimate and unknown evidence must accompany the measured values.
        &self.measurements
    }
}

/// Operands remain in the constructing session; no externally published shape is mutated.
/// Example: `execute_boolean(request, sphere, cube)?.shape()` only follows all required checks.
pub(super) fn execute_boolean(
    request: BooleanRequest,
    first: Shape,
    second: Shape,
) -> Result<CheckedLocalBody, BooleanIssue> {
    execute_boolean_on_progress(request, first, second, &ProgressRange::default())
}

pub(super) fn execute_boolean_on_progress(
    request: BooleanRequest,
    first: Shape,
    second: Shape,
    range: &ProgressRange<'_>,
) -> Result<CheckedLocalBody, BooleanIssue> {
    let raw = acquire_raw(request, &first, &second, range)?;
    admit_raw(request, raw)
}

#[cfg(test)]
pub(super) fn execute_with_measurements(
    request: BooleanRequest,
    first: Shape,
    second: Shape,
    source: &impl boolean_measurements::MeasurementSource,
) -> Result<CheckedLocalBody, BooleanIssue> {
    let raw = acquire_raw(request, &first, &second, &ProgressRange::default())?;
    admit_raw_with(request, raw, source)
}

struct RawBody {
    shape: Shape,
    history: Result<boolean_history::LocalHistory, BooleanIssue>,
    diagnostics: KernelDiagnostics,
    coverage: super::boolean_coverage::OperationCoverage,
}

fn acquire_raw(
    request: BooleanRequest,
    first: &Shape,
    second: &Shape,
    range: &ProgressRange<'_>,
) -> Result<RawBody, BooleanIssue> {
    let mut coverage = OperationCoverage::default();
    let (mut operation, diagnostics) =
        execute_observed(request, first, second, range, &mut coverage)?;
    let history = boolean_history::observe(request, &mut operation, &[first, second]);
    let history = observed_stage(history, CoverageStage::History, &mut coverage);
    Ok(RawBody {
        shape: operation.shape().clone(),
        history,
        diagnostics,
        coverage,
    })
}

fn execute_observed(
    request: BooleanRequest,
    first: &Shape,
    second: &Shape,
    range: &ProgressRange<'_>,
    coverage: &mut OperationCoverage,
) -> Result<(BOPAlgoBOP, KernelDiagnostics), BooleanIssue> {
    let result = run_operation(request, first, second, range).map_err(|issue| {
        coverage.refused(CoverageStage::Execution, issue.class, issue.to_string());
        issue.with_coverage(coverage.clone())
    })?;
    coverage.completed(CoverageStage::Execution, 1);
    Ok(result)
}

fn admit_raw(request: BooleanRequest, raw: RawBody) -> Result<CheckedLocalBody, BooleanIssue> {
    admit_raw_with(request, raw, &boolean_measurements::ActualMeasurements)
}

fn admit_raw_with(
    request: BooleanRequest,
    mut raw: RawBody,
    source: &impl boolean_measurements::MeasurementSource,
) -> Result<CheckedLocalBody, BooleanIssue> {
    let snapshot = raw
        .history
        .as_ref()
        .ok()
        .map(boolean_history::LocalHistory::observation);
    let components =
        checked_components(request, &raw.shape, raw.history, &mut raw.coverage, source);
    materialize_admission(
        raw.shape,
        raw.diagnostics,
        raw.coverage,
        snapshot,
        components,
    )
}

fn materialize_admission(
    shape: Shape,
    diagnostics: KernelDiagnostics,
    coverage: super::boolean_coverage::OperationCoverage,
    snapshot: Option<boolean_history::HistoryObservation>,
    components: Result<BodyComponents, BooleanIssue>,
) -> Result<CheckedLocalBody, BooleanIssue> {
    match components {
        Ok((checks, history, measurements)) => Ok(CheckedLocalBody {
            shape,
            history,
            checks,
            measurements,
            diagnostics,
            coverage,
        }),
        Err(issue) => Err(failed_components(issue, snapshot, diagnostics, coverage)),
    }
}

fn failed_components(
    issue: BooleanIssue,
    snapshot: Option<boolean_history::HistoryObservation>,
    diagnostics: KernelDiagnostics,
    coverage: OperationCoverage,
) -> BooleanIssue {
    issue
        .with_history(snapshot)
        .with_diagnostics(diagnostics)
        .with_coverage(coverage)
}

type BodyComponents = (
    boolean_validation::ValidationReceipt,
    boolean_history::LocalHistory,
    boolean_measurements::MeasurementReceipt,
);

fn checked_components(
    request: BooleanRequest,
    shape: &Shape,
    history: Result<boolean_history::LocalHistory, BooleanIssue>,
    coverage: &mut super::boolean_coverage::OperationCoverage,
    source: &impl boolean_measurements::MeasurementSource,
) -> Result<BodyComponents, BooleanIssue> {
    use super::boolean_coverage::CoverageStage;
    let checks = boolean_validation::validate_observed(request, shape, coverage)?;
    let history = observed_stage(history, CoverageStage::History, coverage)?;
    observed_stage(
        require_input_authority(request),
        CoverageStage::InputAuthority,
        coverage,
    )?;
    let measurements = boolean_measurements::measure_with(request, shape, coverage, source)?;
    Ok((checks, history, measurements))
}

fn observed_stage<T>(
    result: Result<T, BooleanIssue>,
    stage: super::boolean_coverage::CoverageStage,
    coverage: &mut super::boolean_coverage::OperationCoverage,
) -> Result<T, BooleanIssue> {
    match result {
        Ok(value) => {
            coverage.completed(stage, 1);
            Ok(value)
        }
        Err(issue) => {
            coverage.refused(stage, issue.class, issue.to_string());
            Err(issue)
        }
    }
}

fn run_operation(
    request: BooleanRequest,
    first: &Shape,
    second: &Shape,
    range: &ProgressRange<'_>,
) -> Result<(BOPAlgoBOP, KernelDiagnostics), BooleanIssue> {
    let mut operation = configured_operation(request, first, second);
    let completion =
        BOPAlgoBOP::perform_hooks_with_progress(&mut operation, range).map_err(|cause| {
            kernel_issue(request, "Execution", cause)
                .with_diagnostics(actual_diagnostics(&operation))
        })?;
    let diagnostics = actual_diagnostics(&operation);
    print!("{}{}", diagnostics.warnings, diagnostics.errors);
    match completed_candidate(request, &operation, completion) {
        Ok(()) => Ok((operation, diagnostics)),
        Err(issue) => Err(issue.with_diagnostics(diagnostics)),
    }
}

fn completed_candidate(
    request: BooleanRequest,
    operation: &BOPAlgoBOP,
    completion: ProgressCompletion,
) -> Result<(), BooleanIssue> {
    execution_status(request, "Execution", completion)?;
    require_candidate(request, operation)
}

fn require_candidate(request: BooleanRequest, operation: &BOPAlgoBOP) -> Result<(), BooleanIssue> {
    if !operation.options().has_errors() && !operation.shape().is_null() {
        return Ok(());
    }
    let actual = format!(
        "errors={} null={}",
        operation.options().has_errors(),
        operation.shape().is_null()
    );
    Err(refused(
        request,
        "Execution",
        actual,
        "non-null result without native errors",
        RefusalClass::Failed,
    ))
}

fn require_input_authority(request: BooleanRequest) -> Result<(), BooleanIssue> {
    if matches!(request.authority, OperandAuthority::ProceduralSource) {
        return Ok(());
    }
    Err(refused(
        request,
        "InputAuthority",
        "actual opened face and consumed glyph provider unobserved".into(),
        "separately governed actual font provider witness",
        RefusalClass::Incomplete,
    ))
}

fn configured_operation(request: BooleanRequest, first: &Shape, second: &Shape) -> BOPAlgoBOP {
    let mut operation = BOPAlgoBOP::new();
    operation.add_argument(first);
    operation.add_tool(second);
    bop_options::direct_options(&mut operation, request.operation, precision::confusion());
    operation
}

/// Continue only an unpublished body, composing actual histories before returning its successor.
/// Example: `continue_boolean(request, previous, tool)?` consumes the predecessor.
pub(super) fn continue_boolean(
    request: BooleanRequest,
    previous: CheckedLocalBody,
    tool: Shape,
) -> Result<CheckedLocalBody, BooleanIssue> {
    let lineage = prior_lineage(&previous);
    let raw = acquire_raw(request, &previous.shape, &tool, &ProgressRange::default()).map_err(
        |issue| {
            issue
                .prepend_diagnostics(previous.diagnostics.clone())
                .with_lineage(lineage.clone())
        },
    )?;
    let (mut raw, lineage) = compose_successor(request, previous.history, raw, lineage);
    raw.diagnostics = previous.diagnostics.followed_by(raw.diagnostics);
    admit_raw(request, raw).map_err(|issue| issue.with_lineage(lineage))
}

fn compose_successor(
    request: BooleanRequest,
    previous: boolean_history::LocalHistory,
    mut raw: RawBody,
    mut lineage: LineageRetention,
) -> (RawBody, LineageRetention) {
    observe_successor_lineage(&raw, &mut lineage);
    raw.history = raw.history.and_then(|next| {
        let extra = next.tool_origins(request);
        boolean_history::compose(request, previous, next, extra)
    });
    record_composition(&mut raw, &mut lineage);
    (raw, lineage)
}

fn observe_successor_lineage(raw: &RawBody, lineage: &mut LineageRetention) {
    lineage.successor = raw
        .history
        .as_ref()
        .ok()
        .map(boolean_history::LocalHistory::observation);
    lineage.successor_state = if raw.history.is_ok() {
        "Observed"
    } else {
        "ObservationFailed"
    };
}

fn record_composition(raw: &mut RawBody, lineage: &mut LineageRetention) {
    lineage.composition = if lineage.successor.is_none() {
        "Unattempted"
    } else if raw.history.is_ok() {
        "Completed"
    } else {
        "Failed"
    };
    use super::boolean_coverage::CoverageStage;
    match &raw.history {
        Ok(_) => raw.coverage.completed(CoverageStage::History, 1),
        Err(issue) => raw
            .coverage
            .refused(CoverageStage::History, issue.class, issue.to_string()),
    }
}

fn prior_lineage(previous: &CheckedLocalBody) -> LineageRetention {
    LineageRetention {
        prior: Some(previous.history.observation()),
        prior_coverage: Some(previous.coverage.clone()),
        successor: None,
        successor_state: "Unobserved",
        composition: "Unattempted",
    }
}

#[path = "boolean_conversion.rs"]
mod conversion;
pub(super) use conversion::convert_nurbs;
#[cfg(test)]
pub(super) use conversion::{ActualNurbsSource, NurbsSource, convert_with};

fn actual_diagnostics(operation: &BOPAlgoBOP) -> KernelDiagnostics {
    use martensite_tkernel::message::Gravity;
    let report = operation.options().report();
    KernelDiagnostics {
        warnings: super::bop_alerts::gravity_alert_text(report, Gravity::Warning, "Warning: "),
        errors: super::bop_alerts::gravity_alert_text(report, Gravity::Fail, "Error: "),
    }
}
