//! Frozen cone receipts bind actual retained coefficients to original authored spans.
use super::super::super::storage as retained;
use super::*;
mod allocation;
mod binding;
mod boundary;
#[cfg(test)]
pub(super) use boundary::cap_join_spans;
pub(super) use boundary::{prepared_query, query as boundary_query};
mod capture;
mod trims;
mod work;
pub(super) use work::Work;
type Bindings<'a> = BTreeMap<BindingKey, &'a Shape>;
pub(super) struct Stored {
    pub shape: Shape,
    pub coverage: coverage::CoverageEvidence,
    pub spans: Vec<SpanProof>,
    pub carriers: [ConicalTrimCarrier; 2],
    pub joins: ConicalJoinsEvidence,
    pub capture_operations: u32,
}
pub(in crate::surface_sketch::wrapped_profile) struct SpanProof {
    pub span: CurveSpan,
    pub boundary: std::sync::Arc<ConicalBoundaryEvidence>,
    pub storage: allocation::StorageEvidence,
    pub trims: [CertifiedConicalTrim; 2],
    pub developed_boundary: Option<ConicalDevelopedBoundaryEvidence>,
    pub cap_pcurves: usize,
    pub side_pcurves: usize,
    pub maximum_tolerance: f64,
    pub curves: [retained::OwnedCurve; 2],
    pub pcurves: [Vec<[f64; 2]>; 2],
    pub side_reversed: bool,
    pub cap_reversed: [bool; 2],
}
impl SpanProof {
    pub(super) fn flux_span(&self) -> AnalyticConicalFluxSpan<'_> {
        AnalyticConicalFluxSpan {
            boundaries: self.curves.each_ref().map(retained::OwnedCurve::borrowed),
            cap_pcurves: self.pcurves.each_ref().map(Vec::as_slice),
            side_reversed: self.side_reversed,
            cap_reversed: self.cap_reversed,
        }
    }
}
pub(super) fn capture(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    view: coverage::MemberView,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Stored> {
    let started = work.consumed;
    let captured = capture_spans(original, prepared, &view, work, cancelled)?;
    let joins = boundary::joins(original, prepared, &captured.spans, work, cancelled)?;
    Ok(Stored {
        shape: view.shape,
        coverage: captured.coverage,
        spans: captured.spans,
        carriers: captured.carriers,
        joins,
        capture_operations: work.consumed - started,
    })
}

struct CapturedSpans {
    coverage: coverage::CoverageEvidence,
    spans: Vec<SpanProof>,
    carriers: [ConicalTrimCarrier; 2],
}

fn capture_spans(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    view: &coverage::MemberView,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<CapturedSpans> {
    let (coverage, occurrences) = graph(original.region, prepared, view)?;
    let bindings = captured_index(original.region, &view.bindings)?;
    retain_actual_bindings(prepared, view, work);
    let carriers = capture::carriers(prepared, &bindings)?;
    let source = capture::SourceCapture {
        original,
        prepared,
        bindings: &bindings,
        carriers,
        occurrences: &occurrences,
    };
    let spans = capture_proofs(&source, work, cancelled)?;
    Ok(CapturedSpans {
        coverage,
        spans,
        carriers,
    })
}

fn graph(
    region: SurfaceRegionId,
    prepared: &Prepared,
    view: &coverage::MemberView,
) -> ConeChecked<(coverage::CoverageEvidence, coverage::Occurrences)> {
    coverage::certify_developed_oriented(
        &prepared.source,
        &view.shape,
        &view.bindings,
        super::super::topology::DevelopedMapHandedness::Reversing,
    )
    .map_err(|cause| failure(region, None, ConeCause::Coverage(Box::new(cause))))
}

fn captured_index(
    region: SurfaceRegionId,
    bindings: &[(BindingKey, Shape)],
) -> ConeChecked<Bindings<'_>> {
    retained::index(bindings).map_err(|cause| captured(region, cause))
}

fn capture_proofs(
    source: &capture::SourceCapture<'_>,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Vec<SpanProof>> {
    source
        .prepared
        .source
        .loops
        .iter()
        .flatten()
        .map(|piece| source.span(piece, work, cancelled))
        .collect()
}

pub(super) fn failure(
    region: SurfaceRegionId,
    span: Option<CurveSpan>,
    cause: ConeCause,
) -> ConeFailure {
    ConeFailure::new(Some(region), span, WrappedFailureClass::Incomplete, cause)
        .at(WrappedStage::Construction)
}
pub(super) fn captured(region: SurfaceRegionId, cause: retained::StorageFailure) -> ConeFailure {
    let span = match cause.key.source {
        Source::Span(span) => Some(span),
        _ => None,
    };
    failure(region, span, ConeCause::Storage(Box::new(cause)))
}
pub(super) fn mismatch(
    region: SurfaceRegionId,
    span: Option<CurveSpan>,
    field: &'static str,
    actual: Vec<u64>,
    expected: Vec<u64>,
) -> ConeFailure {
    failure(
        region,
        span,
        ConeCause::Binding {
            field,
            actual,
            expected,
        },
    )
}
pub(super) fn bind(
    originals: &[DevelopedRegionRequest],
    selection: &Selection,
    stored: &BTreeMap<SurfaceRegionId, Stored>,
) -> ConeChecked<()> {
    binding::certify(originals, selection, stored)
}

fn retain_actual_bindings(prepared: &Prepared, view: &coverage::MemberView, work: &mut Work) {
    if domain::directed(prepared) {
        work.actual_bindings
            .insert(prepared.source.region, view.bindings.clone());
    }
}
