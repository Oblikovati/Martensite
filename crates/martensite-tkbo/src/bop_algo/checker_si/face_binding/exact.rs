// SPDX-License-Identifier: GPL-2.0-only
//! Released exact supplied-coefficient operations; no private arithmetic copy.
use super::{capture_curves::unsupported, types::*};
use martensite_tkmath::predicates::{compare_scalar_triple_product_3d, roots::metric_bounds::*};
use std::cmp::Ordering;

pub(super) fn identity() -> MetricEmbedding<3> {
    MetricEmbedding {
        linear: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
        origin: [0.; 3],
        scale: 1.,
    }
}
pub(super) fn require_invertible(placement: &AffinePlacement) -> Result<(), BindingFailure> {
    if !placement.scale.is_finite()
        || placement.scale == 0.
        || placement.origin.iter().any(|v| !v.is_finite())
    {
        return Err(unsupported(
            "affine placement",
            format!("{placement:?}; expected finite invertible affine map"),
        ));
    }
    let sign = compare_scalar_triple_product_3d(
        placement.linear[0],
        placement.linear[1],
        placement.linear[2],
    )
    .map_err(|cause| unsupported("affine determinant", format!("{cause}")))?;
    if sign == Ordering::Equal {
        return Err(unsupported("affine determinant", "zero; expected nonzero"));
    }
    Ok(())
}
pub(super) fn endpoint_matches(
    line: &OwnedLine<2>,
    parameter: f64,
    target: [f64; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<bool, BindingFailure> {
    let first = endpoint_chain(line, parameter);
    let second = MetricCurveChain {
        curve: MetricCurve::Line {
            origin: [target[0], target[1], 0.],
            direction: [0.; 3],
        },
        embedding: identity(),
        placements: &[],
    };
    Ok(compare_curve_chains(first, second, cancelled)? == CurveIdentityDecision::Identical)
}
fn endpoint_chain(line: &OwnedLine<2>, parameter: f64) -> MetricCurveChain<'static, 2> {
    MetricCurveChain {
        curve: MetricCurve::Line {
            origin: [parameter, 0.],
            direction: [0.; 2],
        },
        embedding: MetricEmbedding {
            linear: [[line.direction[0], 0.], [line.direction[1], 0.], [0., 0.]],
            origin: [line.origin[0], line.origin[1], 0.],
            scale: 1.,
        },
        placements: &[],
    }
}

pub(super) fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), BindingFailure> {
    if cancelled() {
        return Err(BindingFailure::Cancelled);
    }
    Ok(())
}

pub(super) fn squared_limit(
    limit: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<martensite_tkmath::predicates::roots::DyadicBound, BindingFailure> {
    // det(diag(limit,limit,1)) is the released exact scalar-product facility: no f64 square.
    let frame = MetricEmbedding {
        linear: [[limit, 0.], [0., limit], [0., 0.]],
        origin: [0.; 3],
        scale: 1.,
    };
    Ok(affine_prism_jacobian(
        frame,
        identity(),
        [0., 0., 1.],
        cancelled,
    )?)
}
