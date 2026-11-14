//! Certify decoded coefficients and require the native side net to retain those curves.
#[path = "storage_capture.rs"]
mod capture;
#[path = "storage_incidence.rs"]
mod incidence;
#[cfg(test)]
#[path = "storage_tests.rs"]
mod tests;
#[path = "storage_trims.rs"]
mod trims;
use super::*;
use crate::surface_sketch::HistoryRole;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedCylindricalDepths, CertifiedCylindricalProfile, CertifiedCylindricalStorage,
    CylindricalDepthFailure, CylindricalProfileError, CylindricalProfileInterval,
    CylindricalProfileQuery, CylindricalStorageError, CylindricalStorageLimits,
    CylindricalStorageQuery, MetricBezier, certify_cylindrical_depths, certify_cylindrical_profile,
    certify_cylindrical_storage,
};
use std::collections::BTreeMap;

type Bindings<'a> = BTreeMap<BindingKey, &'a Shape>;

pub(super) struct StoredProof {
    pub span: CurveSpan,
    pub certificate: CertifiedCylindricalStorage,
    pub profile: CertifiedCylindricalProfile,
    pub depths: CertifiedCylindricalDepths,
    pub trims: trims::TrimEvidence,
}

struct OwnedCurve {
    poles: Vec<[f64; 3]>,
    weights: Vec<f64>,
}
impl OwnedCurve {
    fn borrowed(&self) -> MetricBezier<'_, 3> {
        MetricBezier {
            poles: &self.poles,
            weights: Some(&self.weights),
        }
    }
}

#[derive(Debug)]
pub(super) struct StorageFailure {
    key: BindingKey,
    cause: StorageCause,
}
#[derive(Debug)]
enum StorageCause {
    Native(Box<Failure>),
    Certificate(Box<CylindricalStorageError>),
    Profile(Box<CylindricalProfileError>),
    Depths(Box<CylindricalDepthFailure>),
    Trim {
        face: BindingKey,
        cause: Box<StorageFailure>,
    },
    Tolerance {
        actual: f64,
        maximum: f64,
    },
    Mismatch {
        field: &'static str,
        actual: Vec<u64>,
        expected: Vec<u64>,
    },
}
impl std::fmt::Display for StorageFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.cause {
            StorageCause::Native(cause) => write!(formatter, "{:?}: {cause}", self.key),
            StorageCause::Certificate(cause) => write!(formatter, "{:?}: {cause}", self.key),
            StorageCause::Profile(cause) => write!(formatter, "{:?}: {cause}", self.key),
            StorageCause::Depths(cause) => write!(formatter, "{:?}: {cause}", self.key),
            StorageCause::Tolerance { actual, maximum } => {
                tolerance_message(formatter, self.key, *actual, *maximum)
            }
            StorageCause::Trim { face, cause } => trim_message(formatter, self.key, *face, cause),
            StorageCause::Mismatch {
                field,
                actual,
                expected,
            } => mismatch_message(formatter, self.key, field, actual, expected),
        }
    }
}

fn trim_message(
    formatter: &mut std::fmt::Formatter<'_>,
    key: BindingKey,
    face: BindingKey,
    cause: &StorageFailure,
) -> std::fmt::Result {
    write!(formatter, "{key:?} on face={face:?}: {cause}")
}

fn mismatch_message(
    formatter: &mut std::fmt::Formatter<'_>,
    key: BindingKey,
    field: &str,
    actual: &[u64],
    expected: &[u64],
) -> std::fmt::Result {
    write!(
        formatter,
        "{key:?} {field}: bits/counts={actual:?}; expected {expected:?}"
    )
}

fn tolerance_message(
    formatter: &mut std::fmt::Formatter<'_>,
    key: BindingKey,
    actual: f64,
    maximum: f64,
) -> std::fmt::Result {
    write!(
        formatter,
        "{key:?}: retained tolerance={actual}; expected finite nonnegative tolerance <= allocated {maximum}"
    )
}

pub(super) fn certify(
    original: &Request,
    request: &Request,
    bindings: &[(BindingKey, Shape)],
) -> Result<Vec<StoredProof>, StorageFailure> {
    let indexed = index(bindings)?;
    capture::caps(request, &indexed)?;
    let authored = authored_index(original)?;
    request
        .loops
        .iter()
        .flatten()
        .map(|piece| certify_span(request, piece, &indexed, &authored))
        .collect()
}

fn index(bindings: &[(BindingKey, Shape)]) -> Result<Bindings<'_>, StorageFailure> {
    let mut indexed = Bindings::new();
    for (key, shape) in bindings {
        if indexed.insert(*key, shape).is_some() {
            return Err(mismatch(*key, "history membership", vec![2], vec![1]));
        }
    }
    Ok(indexed)
}

fn certify_span(
    request: &Request,
    piece: &AuthoredPiece,
    bindings: &Bindings<'_>,
    authored: &BTreeMap<SurfaceCurveId, &[[f64; 2]]>,
) -> Result<StoredProof, StorageFailure> {
    let span = piece.span;
    let first_key = key(span, HistoryRole::FirstDepthEdge);
    let [a, b] = captured_span(piece, bindings)?;
    let trims = trims::certify(request, piece, bindings)?;
    let certificate = prove_captured(request, first_key, [&a, &b])?;
    let depths = prove_depths(request, first_key, &certificate)?;
    let source = authored
        .get(&span.curve)
        .copied()
        .ok_or_else(|| mismatch(first_key, "authored curve membership", vec![0], vec![1]))?;
    let profile = prove_profile(request, span, &a, source)?;
    Ok(StoredProof {
        span,
        certificate,
        profile,
        depths,
        trims,
    })
}

fn captured_span(
    piece: &AuthoredPiece,
    bindings: &Bindings<'_>,
) -> Result<[OwnedCurve; 2], StorageFailure> {
    let a = capture::curve(key(piece.span, HistoryRole::FirstDepthEdge), bindings)?;
    let b = capture::curve(key(piece.span, HistoryRole::LastDepthEdge), bindings)?;
    capture::side(key(piece.span, HistoryRole::SideFace), [&a, &b], bindings)?;
    incidence::span(piece, [&a, &b], bindings)?;
    Ok([a, b])
}

fn prove_captured(
    request: &Request,
    key: BindingKey,
    curves: [&OwnedCurve; 2],
) -> Result<CertifiedCylindricalStorage, StorageFailure> {
    let query = CylindricalStorageQuery {
        curves: curves.map(OwnedCurve::borrowed),
        radii: request.depths.map(|depth| request.radius + depth),
        limits: CylindricalStorageLimits::uniform(storage_share(request.tolerance)),
        options: Default::default(),
    };
    certify_cylindrical_storage(query, || false).map_err(|cause| StorageFailure {
        key,
        cause: StorageCause::Certificate(Box::new(cause)),
    })
}

fn key(span: CurveSpan, role: HistoryRole) -> BindingKey {
    BindingKey {
        source: Source::Span(span),
        role,
    }
}

fn lookup<'a>(key: BindingKey, bindings: &Bindings<'a>) -> Result<&'a Shape, StorageFailure> {
    bindings
        .get(&key)
        .copied()
        .ok_or_else(|| mismatch(key, "history membership", vec![0], vec![1]))
}

fn native<T>(key: BindingKey, result: Result<T, Failure>) -> Result<T, StorageFailure> {
    result.map_err(|cause| StorageFailure {
        key,
        cause: StorageCause::Native(Box::new(cause)),
    })
}

fn mismatch(
    key: BindingKey,
    field: &'static str,
    actual: Vec<u64>,
    expected: Vec<u64>,
) -> StorageFailure {
    StorageFailure {
        key,
        cause: StorageCause::Mismatch {
            field,
            actual,
            expected,
        },
    }
}

fn storage_share(tolerance: f64) -> f64 {
    let nearest = tolerance / 1000.;
    if nearest == 0. {
        return 0.;
    }
    nearest.next_down()
}

pub(super) fn authored_index(
    request: &Request,
) -> Result<BTreeMap<SurfaceCurveId, &[[f64; 2]]>, StorageFailure> {
    let mut result = BTreeMap::new();
    for piece in request.loops.iter().flatten() {
        if result
            .insert(piece.span.curve, piece.poles.as_slice())
            .is_some()
        {
            return Err(mismatch(
                key(piece.span, HistoryRole::FirstDepthEdge),
                "authored curve membership",
                vec![2],
                vec![1],
            ));
        }
    }
    Ok(result)
}

fn prove_profile(
    request: &Request,
    span: CurveSpan,
    curve: &OwnedCurve,
    authored: &[[f64; 2]],
) -> Result<CertifiedCylindricalProfile, StorageFailure> {
    let limit = (request.tolerance / 4.).next_down().max(0.);
    let wrap = |cause| StorageFailure {
        key: key(span, HistoryRole::FirstDepthEdge),
        cause: StorageCause::Profile(Box::new(cause)),
    };
    let options = Default::default();
    let interval =
        CylindricalProfileInterval::new(u64::from(span.index), span.level).map_err(|cause| {
            wrap(CylindricalProfileError {
                requested_options: options,
                requested_spatial_limit: limit,
                cause,
            })
        })?;
    let query = CylindricalProfileQuery {
        authored,
        stored: curve.borrowed(),
        interval,
        support_radius: request.radius,
        stored_radius: request.radius + request.depths[0],
        angular_origin: 0.,
        spatial_limit: limit,
        options,
    };
    certify_cylindrical_profile(query, || false).map_err(wrap)
}

fn prove_depths(
    request: &Request,
    key: BindingKey,
    storage: &CertifiedCylindricalStorage,
) -> Result<CertifiedCylindricalDepths, StorageFailure> {
    certify_cylindrical_depths(
        request.radius,
        request.depths,
        storage,
        storage_share(request.tolerance),
    )
    .map_err(|cause| StorageFailure {
        key,
        cause: StorageCause::Depths(Box::new(cause)),
    })
}
