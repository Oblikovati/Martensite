//! Certify every nonadjacent authored pair before native construction, then bind retained O2 sources.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedCylindricalClearance, CertifiedCylindricalPeriodicClearance,
    CylindricalClearanceDomain, CylindricalClearanceError, CylindricalClearanceQuery,
    CylindricalClearanceSource, certify_cylindrical_clearance,
};
use std::collections::BTreeSet;
#[cfg(test)]
#[path = "planar_clearance_tests.rs"]
mod tests;

pub(super) struct OriginalClearance {
    pub spans: Vec<CurveSpan>,
    pub certificate: CertifiedCylindricalClearance,
    pub periodic: CertifiedCylindricalPeriodicClearance,
}
#[derive(Debug)]
pub(super) enum ClearanceFailure {
    Authored(Box<storage::StorageFailure>),
    EmptySelection,
    SelectionBinding {
        actual: Vec<SurfaceRegionId>,
        expected: Vec<SurfaceRegionId>,
    },
    DuplicateSource(SurfaceCurveId),
    Sources(Box<planar_cones::ConeFailure>),
    Certificate {
        spans: Vec<CurveSpan>,
        cause: Box<CylindricalClearanceError>,
    },
    Binding {
        expected: Vec<CurveSpan>,
        actual: Vec<CurveSpan>,
    },
}
impl std::fmt::Display for ClearanceFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authored(cause) => write!(f, "clearance authored sources: {cause}"),
            Self::Sources(cause) => write!(f, "clearance source binding: {cause}"),
            Self::Certificate { spans, cause } => write!(f, "clearance spans={spans:?}: {cause}"),
            Self::Binding { expected, actual } => write!(
                f,
                "clearance source intervals={actual:?}; expected original intervals={expected:?} with matching retained O2 coefficients"
            ),
            _ => self.selection_message(f),
        }
    }
}

impl ClearanceFailure {
    fn selection_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SelectionBinding { actual, expected } => write!(
                f,
                "clearance regions={actual:?}; expected prepared owners={expected:?}"
            ),
            Self::EmptySelection => write!(
                f,
                "clearance regions=0; expected at least one original region"
            ),
            Self::DuplicateSource(curve) => write!(
                f,
                "clearance curve={curve:?}; expected globally unique authored curve IDs"
            ),
            _ => write!(f, "{self:?}"),
        }
    }
}

pub(super) fn certify(
    original: &Request,
    prepared: &Request,
) -> Result<OriginalClearance, ClearanceFailure> {
    certify_many(
        std::slice::from_ref(original),
        std::slice::from_ref(prepared),
    )
}
pub(super) fn certify_many(
    originals: &[Request],
    prepared: &[Request],
) -> Result<OriginalClearance, ClearanceFailure> {
    let first = originals.first().ok_or(ClearanceFailure::EmptySelection)?;
    selection_binding(originals, prepared)?;
    let authored = authored_collection(originals)?;
    let spans = prepared_spans(prepared);
    let sources = sources(&spans, &authored)?;
    let excluded = excluded_pairs(prepared);
    let query = clearance_query(first, &sources, &excluded);
    let certificate = certify_input(query, &spans)?;
    let periodic = certify_periodic(&certificate, &spans)?;
    Ok(OriginalClearance {
        spans,
        certificate,
        periodic,
    })
}
fn selection_binding(originals: &[Request], prepared: &[Request]) -> Result<(), ClearanceFailure> {
    let expected: Vec<_> = originals.iter().map(|r| r.region).collect();
    let actual: Vec<_> = prepared.iter().map(|r| r.region).collect();
    if actual != expected {
        return Err(ClearanceFailure::SelectionBinding { actual, expected });
    }
    Ok(())
}

fn authored_collection(
    originals: &[Request],
) -> Result<std::collections::BTreeMap<SurfaceCurveId, &[[f64; 2]]>, ClearanceFailure> {
    let mut result = std::collections::BTreeMap::new();
    for original in originals {
        let indexed = storage::authored_index(original)
            .map_err(|cause| ClearanceFailure::Authored(Box::new(cause)))?;
        for (curve, poles) in indexed {
            if result.insert(curve, poles).is_some() {
                return Err(ClearanceFailure::DuplicateSource(curve));
            }
        }
    }
    Ok(result)
}

fn certify_periodic(
    certificate: &CertifiedCylindricalClearance,
    spans: &[CurveSpan],
) -> Result<CertifiedCylindricalPeriodicClearance, ClearanceFailure> {
    certificate
        .certify_periodic_clearance(16, || false)
        .map_err(|cause| ClearanceFailure::Certificate {
            spans: spans.to_vec(),
            cause: Box::new(cause),
        })
}

fn prepared_spans(prepared: &[Request]) -> Vec<CurveSpan> {
    prepared
        .iter()
        .flat_map(|request| request.loops.iter().flatten())
        .map(|piece| piece.span)
        .collect()
}

fn clearance_query<'a>(
    original: &Request,
    sources: &'a [CylindricalClearanceSource<'a>],
    excluded: &'a [[usize; 2]],
) -> CylindricalClearanceQuery<'a> {
    CylindricalClearanceQuery {
        sources,
        support_radius: original.radius,
        angular_origin: 0.,
        domain: CylindricalClearanceDomain::Within { excluded },
        requested_clearance: 4. * original.tolerance,
        options: Default::default(),
    }
}

fn certify_input(
    query: CylindricalClearanceQuery<'_>,
    spans: &[CurveSpan],
) -> Result<CertifiedCylindricalClearance, ClearanceFailure> {
    certify_cylindrical_clearance(query, || false).map_err(|cause| ClearanceFailure::Certificate {
        spans: spans.to_vec(),
        cause: Box::new(cause),
    })
}

fn sources<'a>(
    spans: &[CurveSpan],
    authored: &std::collections::BTreeMap<SurfaceCurveId, &'a [[f64; 2]]>,
) -> Result<Vec<CylindricalClearanceSource<'a>>, ClearanceFailure> {
    spans
        .iter()
        .map(|span| {
            let (authored, interval) = planar_cones::original_source(*span, authored)
                .map_err(|cause| ClearanceFailure::Sources(Box::new(cause)))?;
            Ok(CylindricalClearanceSource { authored, interval })
        })
        .collect()
}

fn excluded_pairs(prepared: &[Request]) -> Vec<[usize; 2]> {
    let mut offset = 0;
    let mut result = BTreeSet::new();
    for contour in prepared.iter().flat_map(|request| &request.loops) {
        for i in 0..contour.len() {
            let mut pair = [offset + i, offset + (i + 1) % contour.len()];
            pair.sort();
            result.insert(pair);
        }
        offset += contour.len();
    }
    result.into_iter().collect()
}

pub(super) fn bind(
    evidence: &OriginalClearance,
    original: &Request,
    prepared: &Request,
    stored: &[storage::StoredProof],
) -> Result<(), ClearanceFailure> {
    let (spans, sources) = planar_cones::source_collection(original, prepared, stored)
        .map_err(|cause| ClearanceFailure::Sources(Box::new(cause)))?;
    if spans != evidence.spans
        || !evidence.certificate.matches_profiles(&sources)
        || !evidence.periodic.matches_profiles(&sources)
    {
        return Err(ClearanceFailure::Binding {
            expected: evidence.spans.clone(),
            actual: spans,
        });
    }
    Ok(())
}
