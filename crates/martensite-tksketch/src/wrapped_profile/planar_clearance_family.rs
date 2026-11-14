//! Compose every decoded member with the one global boundary relation.
use super::*;
use std::collections::BTreeMap;
pub(in super::super) fn bind_many(
    evidence: &OriginalClearance,
    originals: &[Request],
    prepared: &[Request],
    stored: &BTreeMap<SurfaceRegionId, &[storage::StoredProof]>,
) -> Result<(), ClearanceFailure> {
    bind_originals(evidence, originals, prepared)?;
    owners(originals, stored)?;
    let mut spans = Vec::new();
    let mut sources = Vec::new();
    for (original, retained) in originals.iter().zip(prepared) {
        let (member_spans, member_sources) =
            planar_cones::source_collection(original, retained, stored[&original.region])
                .map_err(|cause| ClearanceFailure::Sources(Box::new(cause)))?;
        spans.extend(member_spans);
        sources.extend(member_sources);
    }
    matches(evidence, spans, &sources)
}
fn owners(
    originals: &[Request],
    stored: &BTreeMap<SurfaceRegionId, &[storage::StoredProof]>,
) -> Result<(), ClearanceFailure> {
    let mut expected: Vec<_> = originals.iter().map(|r| r.region).collect();
    expected.sort();
    let actual: Vec<_> = stored.keys().copied().collect();
    if actual != expected {
        return Err(ClearanceFailure::SelectionBinding { actual, expected });
    }
    Ok(())
}
fn matches(
    evidence: &OriginalClearance,
    spans: Vec<CurveSpan>,
    sources: &[martensite_tkmath::predicates::roots::metric_bounds::CylindricalConeSource<'_>],
) -> Result<(), ClearanceFailure> {
    if spans != evidence.spans
        || !evidence.certificate.matches_profiles(sources)
        || !evidence.periodic.matches_profiles(sources)
    {
        return Err(ClearanceFailure::Binding {
            expected: evidence.spans.clone(),
            actual: spans,
        });
    }
    Ok(())
}
