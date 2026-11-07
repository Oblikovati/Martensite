//! Conversion failures retain predecessor evidence before any native mapping can be consumed.
use super::super::boolean_coverage::{CoverageStage, OperationCoverage};
use super::{
    BooleanIssue, BooleanRequest, CheckedLocalBody, LineageRetention, RawBody, Shape, admit_raw,
    boolean_history, kernel_issue, observed_stage, prior_lineage,
};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPINurbsConvert};

type Converted = (Shape, BRepBuilderAPINurbsConvert);

pub(in super::super) trait NurbsSource {
    fn convert(&self, request: BooleanRequest, input: &Shape) -> Result<Converted, BooleanIssue>;
    fn mapping(
        &self,
        request: BooleanRequest,
        input: &Shape,
        converter: &mut BRepBuilderAPINurbsConvert,
    ) -> Result<boolean_history::LocalHistory, BooleanIssue>;
}

pub(in super::super) struct ActualNurbsSource;
impl NurbsSource for ActualNurbsSource {
    fn convert(&self, request: BooleanRequest, input: &Shape) -> Result<Converted, BooleanIssue> {
        let mut converter = BRepBuilderAPINurbsConvert::with_shape(input, false)
            .map_err(|cause| kernel_issue(request, "NurbsConversion", cause))?;
        let shape = converter
            .shape()
            .map_err(|cause| kernel_issue(request, "NurbsConversion", cause))?
            .clone();
        Ok((shape, converter))
    }
    fn mapping(
        &self,
        request: BooleanRequest,
        input: &Shape,
        converter: &mut BRepBuilderAPINurbsConvert,
    ) -> Result<boolean_history::LocalHistory, BooleanIssue> {
        boolean_history::observe_nurbs(request, input, converter)
    }
}

/// Actual mapping precedes admission; a refusal never yields converted geometry.
/// Example: `convert_nurbs(request, previous)?` retains ancestry even on incomplete coverage.
pub(in super::super) fn convert_nurbs(
    request: BooleanRequest,
    previous: CheckedLocalBody,
) -> Result<CheckedLocalBody, BooleanIssue> {
    convert_with(request, previous, &ActualNurbsSource)
}

pub(in super::super) fn convert_with(
    request: BooleanRequest,
    previous: CheckedLocalBody,
    source: &impl NurbsSource,
) -> Result<CheckedLocalBody, BooleanIssue> {
    let mut coverage = OperationCoverage::default();
    let mut lineage = prior_lineage(&previous);
    let result = conversion_raw(request, previous, source, &mut coverage, &mut lineage);
    match result {
        Ok(raw) => admit_raw(request, raw).map_err(|issue| issue.with_lineage(lineage)),
        Err(issue) => Err(issue.with_coverage(coverage).with_lineage(lineage)),
    }
}

fn conversion_raw(
    request: BooleanRequest,
    previous: CheckedLocalBody,
    source: &impl NurbsSource,
    coverage: &mut OperationCoverage,
    lineage: &mut LineageRetention,
) -> Result<RawBody, BooleanIssue> {
    lineage.successor_state = "ConversionFailed";
    let (shape, mut converter) = observed_stage(
        source.convert(request, &previous.shape),
        CoverageStage::NurbsConversion,
        coverage,
    )
    .map_err(|issue| issue.with_diagnostics(previous.diagnostics.clone()))?;
    lineage.successor_state = "MappingFailed";
    let mapping = observed_stage(
        source.mapping(request, &previous.shape, &mut converter),
        CoverageStage::NurbsMap,
        coverage,
    )
    .map_err(|issue| issue.with_diagnostics(previous.diagnostics.clone()))?;
    Ok(compose_conversion(
        request, previous, shape, mapping, coverage, lineage,
    ))
}

fn compose_conversion(
    request: BooleanRequest,
    previous: CheckedLocalBody,
    shape: Shape,
    mapping: boolean_history::LocalHistory,
    coverage: &mut OperationCoverage,
    lineage: &mut LineageRetention,
) -> RawBody {
    lineage.successor = Some(mapping.observation());
    lineage.successor_state = "Observed";
    let history = boolean_history::compose(request, previous.history, mapping, Vec::new());
    lineage.composition = if history.is_ok() {
        "Completed"
    } else {
        "Failed"
    };
    let history = observed_stage(history, CoverageStage::History, coverage);
    RawBody {
        shape,
        history,
        diagnostics: previous.diagnostics,
        coverage: coverage.clone(),
    }
}
