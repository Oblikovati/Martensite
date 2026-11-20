//! Conditional flux and authored normal volume remain distinct measured receipts.
use super::super::stages::OwnedFamily;
use super::*;
use martensite_tkmath::predicates::roots::DyadicBound;
mod authored;
mod comparison;
pub(super) use comparison::member_volume;
#[cfg(test)]
pub(super) use comparison::{ComparisonBounds, certify as comparison};
mod flux;
mod reference;
mod totals;
pub(super) fn geometry(
    request: &WrappedConeNormalRequest,
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    stored: storage::Stored,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<(Geometry, Shape)> {
    let started = work.consumed;
    let material = material::certify(request, original, prepared, &stored, work, &mut *cancelled)?;
    let reference_volume = reference::certify(request, prepared, work, cancelled)?;
    let flux = flux::certify(request, original.region, &stored, work, cancelled)?;
    let authored = authored_for_chart(request, original, prepared, &flux, work, cancelled)?;
    let arithmetic_operations = stored.capture_operations + work.consumed - started;
    let geometry = Geometry {
        coverage: stored.coverage,
        spans: stored.spans,
        flux,
        reference_volume,
        authored,
        joins: stored.joins,
        arithmetic_operations,
        material,
    };
    Ok((geometry, stored.shape))
}
pub(super) fn budget(
    request: &WrappedConeNormalRequest,
    family: &OwnedFamily<Selection, Geometry>,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Budget> {
    if family.selection.executed_selection.is_directed() {
        actual::validate_family(family, work)?;
    }
    totals::certify(request, family, work, cancelled)
}
pub(super) fn spatial(spans: &[storage::SpanProof]) -> ConeChecked<f64> {
    let mut maximum: f64 = 0.;
    for proof in spans {
        let total = upper(proof.boundary.total())?;
        let trace = proof
            .trims
            .iter()
            .map(|trim| trim.residual_bounds()[2])
            .fold(0., f64::max);
        let composed = if total == 0. || trace == 0. {
            total + trace
        } else {
            (total + trace).next_up()
        };
        maximum = maximum.max(composed).max(proof.maximum_tolerance);
    }
    Ok(maximum)
}
pub(super) fn upper(value: &DyadicBound) -> ConeChecked<f64> {
    value
        .binary64_bounds()
        .map(|bound| bound[1])
        .ok_or_else(|| {
            scalar(
                "dyadic outward bound",
                f64::INFINITY,
                "finite binary64 outward enclosure",
            )
        })
}
pub(super) fn range(bounds: [DyadicBound; 2]) -> ConeChecked<WrappedCertifiedRange> {
    let lower = bounds[0]
        .binary64_bounds()
        .map(|value| value[0])
        .ok_or_else(|| {
            scalar(
                "dyadic lower bound",
                f64::NEG_INFINITY,
                "finite binary64 outward enclosure",
            )
        })?;
    Ok(WrappedCertifiedRange {
        lower,
        upper: upper(&bounds[1])?,
    })
}
fn scalar(field: &'static str, actual: f64, expected: &'static str) -> ConeFailure {
    ConeFailure::new(
        None,
        None,
        WrappedFailureClass::Incomplete,
        ConeCause::Scalar {
            field,
            actual,
            expected,
        },
    )
    .at(WrappedStage::Publication)
}

fn authored_for_chart(
    request: &WrappedConeNormalRequest,
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    flux: &CertifiedAnalyticConicalFlux,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConicalAuthoredFlux> {
    authored::certify(
        request,
        original,
        prepared.chart_domain.as_deref(),
        flux.enclosure(),
        work,
        cancelled,
    )
}
