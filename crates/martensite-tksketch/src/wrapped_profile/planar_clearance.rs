//! Certify every nonadjacent authored pair before native construction, then bind retained O2 sources.
use super::*;
#[path = "planar_clearance_family.rs"]
mod family;
#[path = "planar_intervals.rs"]
pub(super) mod intervals;
pub(super) use family::bind_many;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedCylindricalClearance, CertifiedCylindricalPeriodicClearance, ConeClearanceOptions,
    CylindricalClearanceDomain, CylindricalClearanceError, CylindricalClearanceOptions,
    CylindricalClearanceQuery, CylindricalClearanceSource, CylindricalClearanceWorkError,
    certify_cylindrical_clearance_with_work,
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
    OriginalBinding,
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
            _ => self.original_message(f),
        }
    }
}

impl ClearanceFailure {
    fn original_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OriginalBinding => write!(
                f,
                "original curves/chart or periodic source changed; expected the complete recorded boundary relation"
            ),
            _ => write!(f, "{self:?}"),
        }
    }
}

#[cfg(test)]
pub(super) fn certify(
    original: &Request,
    prepared: &Request,
) -> Result<OriginalClearance, ClearanceFailure> {
    certify_many(
        std::slice::from_ref(original),
        std::slice::from_ref(prepared),
    )
}
#[cfg(test)]
pub(super) fn certify_many(
    originals: &[Request],
    prepared: &[Request],
) -> Result<OriginalClearance, ClearanceFailure> {
    certify_many_with_cancel(originals, prepared, || false)
}
pub(super) fn certify_many_with_cancel(
    originals: &[Request],
    prepared: &[Request],
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<OriginalClearance, ClearanceFailure> {
    certify_family(originals, prepared, None, &mut cancelled).map_err(|error| {
        debug_assert!(
            error.operations <= 2 * CylindricalClearanceOptions::default().max_operations
        );
        error.cause
    })
}

#[cfg(test)]
pub(super) fn certify_many_with_limit(
    originals: &[Request],
    prepared: &[Request],
    arithmetic: ConeClearanceOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<OriginalClearance, ClearanceWorkFailure> {
    certify_family(originals, prepared, Some(arithmetic), &mut cancelled)
}

#[derive(Debug)]
pub(super) struct ClearanceWorkFailure {
    pub cause: ClearanceFailure,
    pub operations: u32,
}

impl From<ClearanceFailure> for ClearanceWorkFailure {
    fn from(cause: ClearanceFailure) -> Self {
        Self {
            cause,
            operations: 0,
        }
    }
}

fn certify_family(
    originals: &[Request],
    prepared: &[Request],
    arithmetic: Option<ConeClearanceOptions>,
    cancelled: &mut CancelPoll<'_>,
) -> Result<OriginalClearance, ClearanceWorkFailure> {
    let first = originals.first().ok_or(ClearanceFailure::EmptySelection)?;
    selection_binding(originals, prepared)?;
    let authored = authored_collection(originals)?;
    let spans = prepared_spans(prepared);
    if !intervals::complete(&spans, &authored) {
        return Err(ClearanceFailure::OriginalBinding.into());
    }
    let sources = sources(&spans, &authored)?;
    let excluded = excluded_pairs(prepared);
    let mut query = clearance_query(first, &sources, &excluded);
    query.options = clearance_options(arithmetic);
    let certificate = certify_cylindrical_clearance_with_work(query, &mut *cancelled)
        .map_err(|error| work_failure(&spans, 0, error))?;
    complete_boundary(spans, certificate, arithmetic, cancelled)
}

fn clearance_options(arithmetic: Option<ConeClearanceOptions>) -> CylindricalClearanceOptions {
    arithmetic.map_or_else(Default::default, |options| CylindricalClearanceOptions {
        fractional_bits: options.fractional_bits,
        max_operations: options.max_operations,
        ..Default::default()
    })
}

fn complete_boundary(
    spans: Vec<CurveSpan>,
    certificate: CertifiedCylindricalClearance,
    arithmetic: Option<ConeClearanceOptions>,
    cancelled: &mut CancelPoll<'_>,
) -> Result<OriginalClearance, ClearanceWorkFailure> {
    let spent = certificate.operations();
    let limit = arithmetic.map_or(certificate.requested_options().max_operations, |policy| {
        policy.max_operations - spent
    });
    let periodic = certificate
        .certify_periodic_clearance_with_work(16, limit, cancelled)
        .map_err(|error| work_failure(&spans, spent, error))?;
    Ok(OriginalClearance {
        spans,
        certificate,
        periodic,
    })
}

fn work_failure(
    spans: &[CurveSpan],
    prior_operations: u32,
    error: CylindricalClearanceWorkError,
) -> ClearanceWorkFailure {
    ClearanceWorkFailure {
        operations: prior_operations + error.operations,
        cause: ClearanceFailure::Certificate {
            spans: spans.to_vec(),
            cause: Box::new(error.cause),
        },
    }
}

pub(super) fn bind_originals(
    evidence: &OriginalClearance,
    originals: &[Request],
    prepared: &[Request],
) -> Result<(), ClearanceFailure> {
    selection_binding(originals, prepared)?;
    let first = originals.first().ok_or(ClearanceFailure::EmptySelection)?;
    let authored = authored_collection(originals)?;
    let expected = prepared_spans(prepared);
    if !intervals::complete(&expected, &authored) {
        return Err(ClearanceFailure::OriginalBinding);
    }
    let sources = sources(&expected, &authored)?;
    if !original_matches(evidence, first, &expected, &sources, prepared) {
        return Err(ClearanceFailure::OriginalBinding);
    }
    Ok(())
}

fn original_matches(
    evidence: &OriginalClearance,
    first: &Request,
    expected: &[CurveSpan],
    sources: &[CylindricalClearanceSource<'_>],
    prepared: &[Request],
) -> bool {
    let c = &evidence.certificate;
    evidence.spans == expected
        && c.angular_origin().to_bits() == 0f64.to_bits()
        && c.matches_sources(sources)
        && c.support_radius().to_bits() == first.radius.to_bits()
        && c.clearance_lower_bound().to_bits() == (4. * first.tolerance).to_bits()
        && c.excluded_pairs() == excluded_pairs(prepared)
        && evidence.periodic.matches_boundary(c)
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

#[cfg(test)]
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
