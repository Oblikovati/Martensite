//! Compose source-bound O2 derivative bounds with decoded shared incidence.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedCylindricalCone, CylindricalConeError, CylindricalConeQuery, CylindricalConeSource,
    CylindricalProfileFailure, CylindricalProfileInterval, certify_cylindrical_cone,
};
use std::collections::BTreeMap;
#[cfg(test)]
#[path = "planar_cones_tests.rs"]
pub(super) mod tests;

#[path = "planar_cones_binding.rs"]
mod binding;
pub(super) use binding::bind;

type ProofIndex<'a> = BTreeMap<CurveSpan, &'a storage::StoredProof>;
type AuthoredIndex<'a> = BTreeMap<SurfaceCurveId, &'a [[f64; 2]]>;

#[derive(Default)]
pub(super) struct ConeEvidence {
    pub pieces: Vec<SpanCone>,
    pub joins: Vec<SpanCone>,
}
pub(super) struct SpanCone {
    pub spans: Vec<CurveSpan>,
    pub certificate: CertifiedCylindricalCone,
}
#[derive(Debug)]
pub(super) enum ConeFailure {
    Authored(Box<storage::StorageFailure>),
    Binding {
        spans: Vec<CurveSpan>,
        field: &'static str,
        actual: usize,
        expected: usize,
    },
    Membership {
        span: CurveSpan,
        count: usize,
    },
    Interval {
        span: CurveSpan,
        cause: CylindricalProfileFailure,
    },
    Certificate {
        spans: Vec<CurveSpan>,
        cause: CylindricalConeError,
    },
}
impl std::fmt::Display for ConeFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authored(cause) => write!(f, "cone authored source: {cause}"),
            Self::Membership { span, count } => write!(
                f,
                "cone span={span:?}, membership={count}; expected one source and one retained O2 proof"
            ),
            Self::Interval { span, cause } => write!(f, "cone span={span:?}: {cause}"),
            Self::Certificate { spans, cause } => write!(f, "cone spans={spans:?}: {cause}"),
            _ => self.binding_message(f),
        }
    }
}

impl ConeFailure {
    fn binding_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binding {
                spans,
                field,
                actual,
                expected,
            } => write!(
                f,
                "cone spans={spans:?} {field}={actual}; expected {expected}"
            ),
            _ => write!(f, "{self:?}"),
        }
    }
}

pub(super) fn certify(
    original: &Request,
    prepared: &Request,
    stored: &[storage::StoredProof],
) -> Result<ConeEvidence, ConeFailure> {
    certify_with_cancel(original, prepared, stored, || false)
}
pub(super) fn certify_with_cancel(
    original: &Request,
    prepared: &Request,
    stored: &[storage::StoredProof],
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConeEvidence, ConeFailure> {
    let authored = storage::authored_index(original)
        .map_err(|cause| ConeFailure::Authored(Box::new(cause)))?;
    let proofs = proof_index(stored)?;
    let mut evidence = ConeEvidence::default();
    let sources = ConeSources::new(original.radius, &authored, &proofs);
    for contour in &prepared.loops {
        contour_cones(contour, &sources, &mut evidence, &mut cancelled)?;
    }
    Ok(evidence)
}

fn contour_cones(
    contour: &[AuthoredPiece],
    sources: &ConeSources<'_, '_>,
    evidence: &mut ConeEvidence,
    cancelled: &mut CancelPoll<'_>,
) -> Result<(), ConeFailure> {
    for (i, piece) in contour.iter().enumerate() {
        let next = contour[(i + 1) % contour.len()].span;
        let adjacent = [piece.span, next];
        let regular = sources.certify(&[piece.span], &mut *cancelled)?;
        let join = sources.certify(&adjacent, &mut *cancelled)?;
        evidence.pieces.push(regular);
        evidence.joins.push(join);
    }
    Ok(())
}

struct ConeSources<'a, 'source> {
    radius: f64,
    authored: &'a AuthoredIndex<'source>,
    proofs: &'a ProofIndex<'source>,
}
impl<'a, 'source> ConeSources<'a, 'source> {
    fn new(
        radius: f64,
        authored: &'a AuthoredIndex<'source>,
        proofs: &'a ProofIndex<'source>,
    ) -> Self {
        Self {
            radius,
            authored,
            proofs,
        }
    }
    fn certify(
        &self,
        spans: &[CurveSpan],
        cancelled: &mut CancelPoll<'_>,
    ) -> Result<SpanCone, ConeFailure> {
        prove(spans, self.radius, self.authored, self.proofs, cancelled)
    }
}

fn proof_index(stored: &[storage::StoredProof]) -> Result<ProofIndex<'_>, ConeFailure> {
    let mut result = BTreeMap::new();
    for proof in stored {
        if result.insert(proof.span, proof).is_some() {
            return Err(membership(proof.span, 2));
        }
    }
    Ok(result)
}

fn prove(
    spans: &[CurveSpan],
    radius: f64,
    authored: &AuthoredIndex<'_>,
    proofs: &ProofIndex<'_>,
    cancelled: &mut CancelPoll<'_>,
) -> Result<SpanCone, ConeFailure> {
    let sources = spans
        .iter()
        .map(|span| source(*span, radius, authored, proofs))
        .collect::<Result<Vec<_>, _>>()?;
    certify_sources(spans, &sources, cancelled)
}

fn certify_sources(
    spans: &[CurveSpan],
    sources: &[CylindricalConeSource<'_>],
    cancelled: &mut CancelPoll<'_>,
) -> Result<SpanCone, ConeFailure> {
    let query = CylindricalConeQuery {
        sources,
        options: Default::default(),
    };
    let certificate =
        certify_cylindrical_cone(query, cancelled).map_err(|cause| ConeFailure::Certificate {
            spans: spans.to_vec(),
            cause,
        })?;
    Ok(SpanCone {
        spans: spans.to_vec(),
        certificate,
    })
}

fn source<'a>(
    span: CurveSpan,
    radius: f64,
    authored: &AuthoredIndex<'a>,
    proofs: &ProofIndex<'a>,
) -> Result<CylindricalConeSource<'a>, ConeFailure> {
    let missing = || membership(span, 0);
    let (authored, interval) = original_source(span, authored)?;
    let proof = proofs.get(&span).copied().ok_or_else(missing)?;
    Ok(CylindricalConeSource {
        authored,
        interval,
        support_radius: radius,
        angular_origin: 0.,
        certificate: &proof.profile,
    })
}

fn membership(span: CurveSpan, count: usize) -> ConeFailure {
    ConeFailure::Membership { span, count }
}

pub(super) fn original_source<'a>(
    span: CurveSpan,
    authored: &AuthoredIndex<'a>,
) -> Result<(&'a [[f64; 2]], CylindricalProfileInterval), ConeFailure> {
    let poles = authored
        .get(&span.curve)
        .copied()
        .ok_or_else(|| membership(span, 0))?;
    let interval = CylindricalProfileInterval::new(u64::from(span.index), span.level)
        .map_err(|cause| ConeFailure::Interval { span, cause })?;
    Ok((poles, interval))
}

pub(super) fn source_collection<'a>(
    original: &'a Request,
    prepared: &Request,
    stored: &'a [storage::StoredProof],
) -> Result<(Vec<CurveSpan>, Vec<CylindricalConeSource<'a>>), ConeFailure> {
    let authored = storage::authored_index(original)
        .map_err(|cause| ConeFailure::Authored(Box::new(cause)))?;
    let proofs = proof_index(stored)?;
    let spans: Vec<_> = prepared
        .loops
        .iter()
        .flatten()
        .map(|piece| piece.span)
        .collect();
    let sources = spans
        .iter()
        .map(|span| source(*span, original.radius, &authored, &proofs))
        .collect::<Result<_, _>>()?;
    Ok((spans, sources))
}
