// SPDX-License-Identifier: GPL-2.0-only
//! Bound an ideal sweep and two stored edges under one original spatial limit.
use super::spatial;
use super::storage;
use super::*;

pub(super) struct CombinedRequest<'a> {
    pub(super) spatial: spatial::SpatialRequest<'a>,
    pub(super) edges: [storage::ConicalStorageQuery<'a>; 2],
}
pub(super) struct CombinedEvidence {
    pub(super) retained: Option<std::sync::Arc<spatial::RetainedSpatial>>,
    pub(super) total: Interval,
    pub(super) ideal: Interval,
    pub(super) original: Interval,
    pub(super) anchor: super::original::AnchorEvidence,
    pub(super) angular_derivative: Interval,
    pub(super) original_angular_derivative: Interval,
    pub(super) total_angular_derivative: Interval,
    pub(super) storage: [Interval; 2],
    pub(super) operations: u32,
}
#[derive(Debug)]
pub(super) struct BitsMismatch {
    pub(super) actual_len: usize,
    pub(super) expected_len: usize,
    pub(super) index: usize,
    pub(super) actual: Option<u64>,
    pub(super) expected: Option<u64>,
}
#[derive(Debug)]
pub(super) enum CombinedFailure {
    Binding {
        depth: usize,
        field: &'static str,
        mismatch: BitsMismatch,
    },
    Spatial(spatial::SpatialFailure),
    Storage {
        depth: usize,
        cause: storage::ConicalStorageFailure,
    },
    Arithmetic(BoundedArithmeticFailure),
    Uncertified {
        upper: DyadicBound,
        limit: f64,
    },
}
impl From<BoundedArithmeticFailure> for CombinedFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}
impl std::fmt::Display for CombinedFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binding {
                depth,
                field,
                mismatch,
            } => binding_message(f, *depth, field, mismatch),
            Self::Spatial(cause) => write!(f, "spatial: {cause}"),
            Self::Storage { depth, cause } => write!(f, "storage depth={depth}: {cause}"),
            Self::Arithmetic(cause) => write!(f, "arithmetic: {cause:?}"),
            Self::Uncertified { upper, limit } => write!(
                f,
                "upper={upper:?}, limit={limit}; expected bound at most limit"
            ),
        }
    }
}
pub(super) fn binding_message(
    f: &mut std::fmt::Formatter<'_>,
    depth: usize,
    field: &str,
    mismatch: &BitsMismatch,
) -> std::fmt::Result {
    write!(
        f,
        "depth={depth}, field={field}, actual_len={}, expected_len={}, index={}, actual={:?}, expected={:?}; expected matching bits",
        mismatch.actual_len,
        mismatch.expected_len,
        mismatch.index,
        mismatch.actual,
        mismatch.expected
    )
}
pub(super) fn certify_combined(
    request: &CombinedRequest<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<CombinedEvidence, CombinedFailure> {
    certify_combined_metric(
        request,
        None,
        ConicalPrimitiveRefinement::UnsplitV1,
        arithmetic,
    )
}
pub(super) fn certify_combined_in_chart_selected(
    request: &CombinedRequest<'_>,
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<CombinedEvidence, CombinedFailure> {
    certify_combined_metric(request, Some(domain), refinement, arithmetic)
}
fn certify_combined_metric(
    request: &CombinedRequest<'_>,
    domain: Option<&ConicalChartDomainEvidence>,
    refinement: ConicalPrimitiveRefinement,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<CombinedEvidence, CombinedFailure> {
    admit_binding(request)?;
    let (spatial, retained) = spatial_metric(&request.spatial, domain, refinement, arithmetic)?;
    let [first, last] = storage_metric(request, domain, arithmetic)?;
    let maximum = storage_maximum(&first, &last);
    let total = total_bound(request, &spatial, &maximum, arithmetic)?;
    let total_derivative = total_derivative(request, &spatial, arithmetic)?;
    accept(
        retained,
        total,
        spatial,
        total_derivative,
        [first, last],
        request.spatial.limit,
        arithmetic,
    )
}
fn storage_metric(
    request: &CombinedRequest<'_>,
    domain: Option<&ConicalChartDomainEvidence>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Interval; 2], CombinedFailure> {
    if let Some(domain) = domain {
        return super::storage_pair::certify(request, domain, arithmetic);
    }
    let first = storage::storage_discrepancy(&request.edges[0], arithmetic)
        .map_err(|cause| CombinedFailure::Storage { depth: 0, cause })?;
    let last = storage::storage_discrepancy(&request.edges[1], arithmetic)
        .map_err(|cause| CombinedFailure::Storage { depth: 1, cause })?;
    Ok([first, last])
}
fn spatial_metric(
    request: &spatial::SpatialRequest<'_>,
    domain: Option<&ConicalChartDomainEvidence>,
    refinement: ConicalPrimitiveRefinement,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<
    (
        spatial::SpatialEvidence,
        Option<std::sync::Arc<spatial::RetainedSpatial>>,
    ),
    CombinedFailure,
> {
    if let Some(domain) = domain {
        let capture =
            spatial::certify_spatial_in_chart_selected(request, domain, refinement, arithmetic)
                .map_err(CombinedFailure::Spatial)?;
        let evidence = capture.evidence.clone();
        return Ok((evidence, Some(std::sync::Arc::new(capture))));
    }
    let evidence =
        spatial::certify_spatial_span(request, arithmetic).map_err(CombinedFailure::Spatial)?;
    Ok((evidence, None))
}
pub(super) fn admit_binding(request: &CombinedRequest<'_>) -> Result<(), CombinedFailure> {
    for (depth, edge) in request.edges.iter().enumerate() {
        bind_one(&request.spatial, edge, depth)?;
    }
    bind_values(
        1,
        "stored weights",
        request.edges[1].stored_weights,
        request.edges[0].stored_weights,
    )?;
    Ok(())
}
pub(super) fn bind_one(
    spatial: &spatial::SpatialRequest<'_>,
    edge: &storage::ConicalStorageQuery<'_>,
    depth: usize,
) -> Result<(), CombinedFailure> {
    bind_vectors(spatial, edge, depth)?;
    bind_scalars(spatial, edge, depth)
}
pub(super) fn bind_vectors(
    spatial: &spatial::SpatialRequest<'_>,
    edge: &storage::ConicalStorageQuery<'_>,
    depth: usize,
) -> Result<(), CombinedFailure> {
    for (field, actual, expected) in [
        ("tangent", edge.tangent, spatial.angular_fit),
        ("numerator", edge.numerator, spatial.slant_numerator),
        ("weights", edge.weights, spatial.weights),
    ] {
        bind_values(depth, field, actual, expected)?;
    }
    Ok(())
}
pub(super) fn bind_scalars(
    spatial: &spatial::SpatialRequest<'_>,
    edge: &storage::ConicalStorageQuery<'_>,
    depth: usize,
) -> Result<(), CombinedFailure> {
    for (field, actual, expected) in [
        ("semi-angle", edge.semi_angle, spatial.anchor.semi_angle),
        ("center", edge.center, spatial.anchor.fit_center),
        ("depth", edge.depth, spatial.depths[depth]),
    ] {
        bind_values(depth, field, &[actual], &[expected])?;
    }
    bind_values(
        depth,
        "normal sense",
        &[u8::from(edge.outward) as f64],
        &[u8::from(spatial.outward) as f64],
    )
}
pub(super) fn bind_values(
    depth: usize,
    field: &'static str,
    actual: &[f64],
    expected: &[f64],
) -> Result<(), CombinedFailure> {
    if let Some(mismatch) = first_mismatch(actual, expected) {
        return Err(CombinedFailure::Binding {
            depth,
            field,
            mismatch,
        });
    }
    Ok(())
}
pub(super) fn first_mismatch(actual: &[f64], expected: &[f64]) -> Option<BitsMismatch> {
    let index = actual
        .iter()
        .zip(expected)
        .position(|(a, b)| a.to_bits() != b.to_bits())
        .or_else(|| (actual.len() != expected.len()).then_some(actual.len().min(expected.len())))?;
    Some(BitsMismatch {
        actual_len: actual.len(),
        expected_len: expected.len(),
        index,
        actual: actual.get(index).map(|value| value.to_bits()),
        expected: expected.get(index).map(|value| value.to_bits()),
    })
}

pub(super) fn accept(
    retained: Option<std::sync::Arc<spatial::RetainedSpatial>>,
    total: Interval,
    spatial: spatial::SpatialEvidence,
    total_angular_derivative: Interval,
    storage: [Interval; 2],
    limit: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<CombinedEvidence, CombinedFailure> {
    let requested = arithmetic.scalar(limit)?;
    if total.upper > requested.lower {
        return Err(CombinedFailure::Uncertified {
            upper: DyadicBound::new(total.upper, -i32::from(arithmetic.fractional_bits())),
            limit,
        });
    }
    Ok(CombinedEvidence {
        retained,
        total,
        ideal: spatial.bound,
        original: spatial.original,
        anchor: spatial.anchor,
        angular_derivative: spatial.angular_derivative,
        original_angular_derivative: spatial.original_angular_derivative,
        total_angular_derivative,
        storage,
        operations: arithmetic.operations(),
    })
}

impl std::error::Error for CombinedFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Spatial(cause) => Some(cause),
            _ => None,
        }
    }
}

fn total_bound(
    request: &CombinedRequest<'_>,
    spatial: &spatial::SpatialEvidence,
    maximum: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CombinedFailure> {
    if request.spatial.original.is_some() {
        let ideal_and_original = arithmetic.add(&spatial.bound, &spatial.original)?;
        return Ok(arithmetic.add(&ideal_and_original, maximum)?);
    }
    Ok(arithmetic.add(&spatial.bound, maximum)?)
}
fn total_derivative(
    request: &CombinedRequest<'_>,
    spatial: &spatial::SpatialEvidence,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CombinedFailure> {
    if request.spatial.original.is_some() {
        return Ok(arithmetic.add(
            &spatial.angular_derivative,
            &spatial.original_angular_derivative,
        )?);
    }
    Ok(spatial.angular_derivative.clone())
}

fn storage_maximum(first: &Interval, last: &Interval) -> Interval {
    Interval {
        lower: 0.into(),
        upper: first.upper.clone().max(last.upper.clone()),
    }
}
