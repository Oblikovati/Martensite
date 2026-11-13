//! Certify decoded coefficients and require the native side net to retain those curves.
#[path = "storage_capture.rs"]
mod capture;
#[cfg(test)]
#[path = "storage_tests.rs"]
mod tests;
use super::*;
use crate::surface_sketch::HistoryRole;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedCylindricalStorage, CylindricalStorageError, CylindricalStorageLimits,
    CylindricalStorageQuery, MetricBezier, certify_cylindrical_storage,
};
use std::collections::BTreeMap;

type Bindings<'a> = BTreeMap<BindingKey, &'a Shape>;

pub(super) struct StoredProof {
    pub span: CurveSpan,
    pub certificate: CertifiedCylindricalStorage,
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
            StorageCause::Mismatch {
                field,
                actual,
                expected,
            } => write!(
                formatter,
                "{:?} {field}: bits/counts={actual:?}; expected {expected:?}",
                self.key
            ),
        }
    }
}

pub(super) fn certify(
    request: &Request,
    bindings: &[(BindingKey, Shape)],
) -> Result<Vec<StoredProof>, StorageFailure> {
    let indexed = index(bindings)?;
    capture::caps(request, &indexed)?;
    request
        .loops
        .iter()
        .flatten()
        .map(|piece| certify_span(request, piece.span, &indexed))
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
    span: CurveSpan,
    bindings: &Bindings<'_>,
) -> Result<StoredProof, StorageFailure> {
    let first_key = key(span, HistoryRole::FirstDepthEdge);
    let a = capture::curve(first_key, bindings)?;
    let b = capture::curve(key(span, HistoryRole::LastDepthEdge), bindings)?;
    capture::side(key(span, HistoryRole::SideFace), [&a, &b], bindings)?;
    let certificate = prove_captured(request, first_key, [&a, &b])?;
    Ok(StoredProof { span, certificate })
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
