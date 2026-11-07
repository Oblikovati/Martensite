//! Real predecessors and actual successor observations survive every selected refusal seam.
use super::boolean_coverage::{CoverageStage as Stage, OperationCoverage, StageState};
use super::boolean_history::{HistoryObservation, LocalHistory};
use super::boolean_outcome::{
    ActualNurbsSource, BooleanIssue, BooleanRequest, CheckedLocalBody, NurbsSource,
    OperandAuthority, RefusalClass, continue_boolean, convert_with, execute_boolean, refused,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::topods::Shape;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPINurbsConvert;

#[path = "boolean_report_fixture.rs"]
mod report_fixture;

fn request() -> BooleanRequest {
    BooleanRequest {
        case: "bop4_retained_receipts",
        ordinal: 2,
        operation: Operation::Cut,
        operands: ["previous", "tool"],
        authority: OperandAuthority::ProceduralSource,
    }
}

fn genuine_union() -> CheckedLocalBody {
    let mut initial = request();
    initial.ordinal = 1;
    initial.operation = Operation::Fuse;
    initial.operands = ["b0", "b1"];
    let first = crate::primitive_shapes::box_sized([2.; 3]).unwrap();
    let second = crate::primitive_shapes::box_at([1., 0., 0.], [2.; 3]).unwrap();
    execute_boolean(initial, first, second).expect("real union must pass existing ordered checks")
}

fn refusal(result: Result<CheckedLocalBody, BooleanIssue>) -> BooleanIssue {
    match result {
        Ok(_) => panic!("refused successor escaped; expected no geometry"),
        Err(issue) => issue,
    }
}

fn initial_bindings(snapshot: &HistoryObservation) {
    for binding in ["b0", "b1"] {
        assert!(
            snapshot
                .relations
                .iter()
                .any(|relation| relation.binding == binding)
        );
    }
}

fn prior_retained(issue: &BooleanIssue, prior: &HistoryObservation) {
    let lineage = issue.lineage();
    let actual = lineage
        .prior
        .as_ref()
        .expect("predecessor observation must survive");
    assert_eq!(format!("{actual:?}"), format!("{prior:?}"));
    initial_bindings(actual);
    let coverage = lineage.prior_coverage.as_ref().unwrap();
    assert_eq!(
        coverage.stages[&Stage::FiniteMeasurements].state,
        StageState::Completed
    );
}

#[test]
fn failed_successor_preserves_prior_without_invented_final_history() {
    let (previous, prior_reports) = report_fixture::append_prior_sentinels(genuine_union());
    let prior = previous.history().observation();
    let issue = refusal(continue_boolean(request(), previous, Shape::default()));
    prior_retained(&issue, &prior);
    report_fixture::assert_failed_successor_reports(&issue, &prior_reports);
    assert_eq!(issue.stage, "Execution");
    assert!(issue.history_observation().is_none());
    assert!(issue.lineage().successor.is_none());
    assert_eq!(issue.lineage().successor_state, "Unobserved");
    assert_eq!(issue.lineage().composition, "Unattempted");
    assert_eq!(
        issue.coverage().stages[&Stage::Topology].state,
        StageState::Unattempted
    );
}

#[test]
fn completed_empty_successor_retains_actual_composed_history() {
    let previous = genuine_union();
    let prior = previous.history().observation();
    let tool = previous.shape().clone();
    let issue = refusal(continue_boolean(request(), previous, tool));
    assert_eq!(issue.class, RefusalClass::CompletedEmpty);
    prior_retained(&issue, &prior);
    composed_retained(&issue);
    assert_eq!(
        issue.coverage().stages[&Stage::Execution].state,
        StageState::Completed
    );
    assert_eq!(
        issue.coverage().stages[&Stage::Geometry].state,
        StageState::Unattempted
    );
}

fn composed_retained(issue: &BooleanIssue) {
    let snapshot = issue
        .history_observation()
        .expect("actual composed ancestry must survive");
    initial_bindings(snapshot);
    assert_eq!(snapshot.steps, ["BOP", "BOP"]);
    assert!(
        snapshot
            .relations
            .iter()
            .any(|relation| relation.binding == "tool")
    );
    assert!(
        snapshot
            .relations
            .iter()
            .all(|relation| relation.binding != "previous")
    );
    check_composition_role(issue);
}

fn check_composition_role(issue: &BooleanIssue) {
    assert!(issue.lineage().successor.is_some());
    assert_eq!(issue.lineage().successor_state, "Observed");
    assert_eq!(issue.lineage().composition, "Completed");
}

#[test]
fn missing_input_authority_retains_completed_checks_and_unattempted_measurement() {
    receipt_thread_safety();
    let previous = genuine_union();
    let prior = previous.history().observation();
    let mut next = request();
    next.authority = OperandAuthority::FontProviderUnobserved;
    let tool = crate::primitive_shapes::box_at([1., 0., 0.], [1.; 3]).unwrap();
    let issue = refusal(continue_boolean(next, previous, tool));
    assert_eq!(issue.stage, "InputAuthority");
    assert_eq!(issue.class, RefusalClass::Incomplete);
    prior_retained(&issue, &prior);
    composed_retained(&issue);
    check_authority_coverage(issue.coverage());
}

fn check_authority_coverage(coverage: &OperationCoverage) {
    for stage in [
        Stage::Topology,
        Stage::Geometry,
        Stage::SelfIntersection,
        Stage::Tolerance,
        Stage::History,
    ] {
        assert_eq!(
            coverage.stages[&stage].state,
            StageState::Completed,
            "{stage:?}"
        );
        assert!(coverage.stages[&stage].observed_items > 0);
    }
    check_unattempted_measurements(coverage);
}

fn check_unattempted_measurements(coverage: &OperationCoverage) {
    assert!(
        coverage.stages[&Stage::InputAuthority]
            .detail
            .as_ref()
            .unwrap()
            .contains("actual opened face")
    );
    assert_eq!(
        coverage.stages[&Stage::InputAuthority].state,
        StageState::Missing
    );
    assert_eq!(
        coverage.stages[&Stage::SurfaceMeasurement].state,
        StageState::Unattempted
    );
    assert_eq!(
        coverage.stages[&Stage::VolumeMeasurement].state,
        StageState::Unattempted
    );
    assert!(coverage.self_check.is_some());
    assert!(coverage.surface.is_none() && coverage.volume.is_none());
}

struct FailingNurbsConstructor;
impl NurbsSource for FailingNurbsConstructor {
    fn convert(
        &self,
        request: BooleanRequest,
        _: &Shape,
    ) -> Result<(Shape, BRepBuilderAPINurbsConvert), BooleanIssue> {
        Err(forced_refusal(request, "NurbsConversion"))
    }
    fn mapping(
        &self,
        _: BooleanRequest,
        _: &Shape,
        _: &mut BRepBuilderAPINurbsConvert,
    ) -> Result<LocalHistory, BooleanIssue> {
        panic!("mapping attempted after failed constructor; expected early refusal")
    }
}

struct FailingNurbsMap;
impl NurbsSource for FailingNurbsMap {
    fn convert(
        &self,
        request: BooleanRequest,
        shape: &Shape,
    ) -> Result<(Shape, BRepBuilderAPINurbsConvert), BooleanIssue> {
        ActualNurbsSource.convert(request, shape)
    }
    fn mapping(
        &self,
        request: BooleanRequest,
        _: &Shape,
        _: &mut BRepBuilderAPINurbsConvert,
    ) -> Result<LocalHistory, BooleanIssue> {
        Err(forced_refusal(request, "NurbsMap"))
    }
}

fn forced_refusal(request: BooleanRequest, stage: &'static str) -> BooleanIssue {
    refused(
        request,
        stage,
        "named failure-only dependency".into(),
        "actual completed conversion/map/measurement",
        RefusalClass::Incomplete,
    )
}

#[test]
fn conversion_constructor_refusal_retains_predecessor_receipts() {
    let (previous, prior_reports) = report_fixture::append_prior_sentinels(genuine_union());
    let prior = previous.history().observation();
    let issue = refusal(convert_with(request(), previous, &FailingNurbsConstructor));
    prior_retained(&issue, &prior);
    report_fixture::assert_conversion_refusal_reports(&issue, &prior_reports);
    assert_eq!(issue.stage, "NurbsConversion");
    assert_eq!(
        issue.coverage().stages[&Stage::NurbsConversion].state,
        StageState::Missing
    );
    assert_eq!(
        issue.coverage().stages[&Stage::NurbsMap].state,
        StageState::Unattempted
    );
    assert_eq!(issue.lineage().successor_state, "ConversionFailed");
    assert!(issue.history_observation().is_none() && issue.lineage().successor.is_none());
}

#[test]
fn conversion_mapping_refusal_retains_predecessor_after_actual_conversion() {
    let (previous, prior_reports) = report_fixture::append_prior_sentinels(genuine_union());
    let prior = previous.history().observation();
    let issue = refusal(convert_with(request(), previous, &FailingNurbsMap));
    prior_retained(&issue, &prior);
    report_fixture::assert_conversion_refusal_reports(&issue, &prior_reports);
    assert_eq!(issue.stage, "NurbsMap");
    assert_eq!(
        issue.coverage().stages[&Stage::NurbsConversion].state,
        StageState::Completed
    );
    assert_eq!(
        issue.coverage().stages[&Stage::NurbsMap].state,
        StageState::Missing
    );
    assert_eq!(issue.lineage().successor_state, "MappingFailed");
    assert!(issue.history_observation().is_none() && issue.lineage().successor.is_none());
}

#[derive(Default)]
struct FailingVolumeMeasurement {
    observed: std::cell::Cell<Option<martensite_tktopalgo::brep_gprop::SurfacePropertyResult>>,
}
impl super::boolean_measurements::MeasurementSource for FailingVolumeMeasurement {
    fn surface(
        &self,
        request: BooleanRequest,
        shape: &Shape,
    ) -> Result<martensite_tktopalgo::brep_gprop::SurfacePropertyResult, BooleanIssue> {
        use super::boolean_measurements::ActualMeasurements;
        let surface = ActualMeasurements.surface(request, shape)?;
        self.observed.set(Some(surface));
        Ok(surface)
    }
    fn volume(&self, request: BooleanRequest, _: &Shape) -> Result<f64, BooleanIssue> {
        Err(forced_refusal(request, "Measurement"))
    }
}

#[test]
fn failed_volume_retains_actual_surface_options_and_error_category() {
    use super::boolean_outcome::execute_with_measurements;
    let first = crate::primitive_shapes::box_sized([2.; 3]).unwrap();
    let second = crate::primitive_shapes::box_at([1., 0., 0.], [2.; 3]).unwrap();
    let mut operation = request();
    operation.operation = Operation::Fuse;
    let source = FailingVolumeMeasurement::default();
    let issue = refusal(execute_with_measurements(operation, first, second, &source));
    check_partial_measurement(&issue, &source);
}

fn check_partial_measurement(issue: &BooleanIssue, source: &FailingVolumeMeasurement) {
    let coverage = issue.coverage();
    let surface = coverage
        .surface
        .expect("actual surface must survive later volume refusal");
    assert!((surface.properties().mass() - 32.).abs() < 1e-7);
    assert_eq!(surface.options(), coverage.surface_options);
    assert_eq!(Some(surface), source.observed.get());
    assert_eq!(coverage.volume_error, "Unknown");
    assert_eq!(coverage.volume_options, (false, false, false));
    assert!(coverage.volume.is_none());
    check_partial_measurement_stages(issue);
}

fn check_partial_measurement_stages(issue: &BooleanIssue) {
    let coverage = issue.coverage();
    assert_eq!(
        coverage.stages[&Stage::SurfaceMeasurement].state,
        StageState::Completed
    );
    assert_eq!(
        coverage.stages[&Stage::VolumeMeasurement].state,
        StageState::Missing
    );
    assert_eq!(
        coverage.stages[&Stage::FiniteMeasurements].state,
        StageState::Unattempted
    );
    assert!(issue.history_observation().is_some());
    assert_eq!(
        coverage.stages[&Stage::InputAuthority].state,
        StageState::Completed
    );
}

fn receipt_thread_safety() {
    fn assert_value<T: Send + Sync>() {}
    assert_value::<OperationCoverage>();
    assert_value::<super::boolean_outcome::LineageRetention>();
    assert_value::<HistoryObservation>();
}
