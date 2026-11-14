//! Certify every nonadjacent authored pair before native construction, then bind retained O2 sources.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedCylindricalClearance, CertifiedCylindricalPeriodicSlab, CylindricalClearanceDomain,
    CylindricalClearanceError, CylindricalClearanceQuery, CylindricalClearanceSource,
    certify_cylindrical_clearance,
};
use std::collections::BTreeSet;
#[cfg(test)]
#[path = "planar_clearance_tests.rs"]
mod tests;

pub(super) struct OriginalClearance {
    pub spans: Vec<CurveSpan>,
    pub certificate: CertifiedCylindricalClearance,
    pub periodic: CertifiedCylindricalPeriodicSlab,
}
#[derive(Debug)]
pub(super) enum ClearanceFailure {
    Authored(Box<storage::StorageFailure>),
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
        }
    }
}

pub(super) fn certify(
    original: &Request,
    prepared: &Request,
) -> Result<OriginalClearance, ClearanceFailure> {
    let authored = storage::authored_index(original)
        .map_err(|cause| ClearanceFailure::Authored(Box::new(cause)))?;
    let spans = prepared_spans(prepared);
    let sources = sources(&spans, &authored)?;
    let excluded = excluded_pairs(prepared);
    let query = clearance_query(original, &sources, &excluded);
    let certificate = certify_input(query, &spans)?;
    let periodic = certify_periodic(&certificate, &spans)?;
    Ok(OriginalClearance {
        spans,
        certificate,
        periodic,
    })
}

fn certify_periodic(
    certificate: &CertifiedCylindricalClearance,
    spans: &[CurveSpan],
) -> Result<CertifiedCylindricalPeriodicSlab, ClearanceFailure> {
    certificate
        .certify_periodic_slab(|| false)
        .map_err(|cause| ClearanceFailure::Certificate {
            spans: spans.to_vec(),
            cause: Box::new(cause),
        })
}

fn prepared_spans(prepared: &Request) -> Vec<CurveSpan> {
    prepared
        .loops
        .iter()
        .flatten()
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

fn excluded_pairs(prepared: &Request) -> Vec<[usize; 2]> {
    let mut offset = 0;
    let mut result = BTreeSet::new();
    for contour in &prepared.loops {
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
