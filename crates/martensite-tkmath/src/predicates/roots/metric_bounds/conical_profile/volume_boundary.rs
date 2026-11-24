// SPDX-License-Identifier: GPL-2.0-only
//! Conditional signed boundary integral; material and topology remain caller obligations.
use super::inverse::*;
use super::quadrature::*;
use super::*;

pub(super) type SourceBits = Vec<Vec<Vec<[u64; 2]>>>;
pub(super) struct Contour<'a> {
    pub(super) spans: Vec<&'a [[f64; 2]]>,
    pub(super) reverse: bool,
}
#[derive(Debug)]
pub(super) enum BoundaryFailure {
    Options(SourceBoundFailure),
    ContourCount(usize),
    TotalSpanCount(usize),
    SpanCount {
        contour: usize,
        count: usize,
    },
    PoleCount {
        contour: usize,
        span: usize,
        count: usize,
    },
    NonFinite {
        contour: usize,
        span: usize,
        pole: usize,
        axis: usize,
        value: f64,
    },
    Open {
        contour: usize,
        span: usize,
        end: [f64; 2],
        next: [f64; 2],
    },
    Span {
        contour: usize,
        span: usize,
        cause: SourceBoundFailure,
    },
    Arithmetic(BoundedArithmeticFailure),
    Trigonometric(crate::predicates::roots::metric_bounds::CylindricalProfileFailure),
    InvalidSupport(f64),
    InvalidDepths([f64; 2]),
    UnresolvedArea,
    UnresolvedInverse,
    InvalidClearanceLevel(u8),
    DepthClearance {
        contour: usize,
        span: usize,
        cause: Box<super::ConicalDepthFailure>,
    },
}
impl std::fmt::Display for BoundaryFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Options(cause) => write!(out, "boundary options: {cause}"),
            Self::ContourCount(count) => write!(out, "contours={count}; expected 1..=128"),
            Self::TotalSpanCount(count) => write!(out, "total spans={count}; expected <= 128"),
            Self::SpanCount { .. } => self.span_count_message(out),
            Self::PoleCount { .. } => self.pole_count_message(out),
            Self::NonFinite { .. } => self.nonfinite_message(out),
            Self::Open { .. } => self.open_message(out),
            Self::Span { .. } => self.span_message(out),
            Self::Arithmetic(cause) => write!(out, "boundary arithmetic: {cause:?}"),
            Self::Trigonometric(cause) => write!(out, "support trigonometry: {cause:?}"),
            Self::InvalidSupport(value) => support_message(out, *value),
            Self::InvalidDepths(depths) => depths_message(out, *depths),
            Self::UnresolvedArea => out.write_str("signed area lower bound is not positive"),
            Self::UnresolvedInverse => inverse_message(out),
            Self::InvalidClearanceLevel(level) => clearance_level_message(out, *level),
            Self::DepthClearance { .. } => self.depth_clearance_message(out),
        }
    }
}
impl BoundaryFailure {
    fn span_count_message(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self::SpanCount { contour, count } = self else {
            return Err(std::fmt::Error);
        };
        write!(out, "contour={contour}, spans={count}; expected nonempty")
    }
    fn pole_count_message(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self::PoleCount {
            contour,
            span,
            count,
        } = self
        else {
            return Err(std::fmt::Error);
        };
        write!(
            out,
            "contour={contour}, span={span}, poles={count}; expected 2..=4"
        )
    }
    fn nonfinite_message(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self::NonFinite {
            contour,
            span,
            pole,
            axis,
            value,
        } = self
        else {
            return Err(std::fmt::Error);
        };
        write!(
            out,
            "contour={contour}, span={span}, pole={pole}, axis={axis}, value={value}; expected finite"
        )
    }
    fn open_message(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self::Open {
            contour,
            span,
            end,
            next,
        } = self
        else {
            return Err(std::fmt::Error);
        };
        write!(
            out,
            "contour={contour}, span={span}, end={end:?}; expected next start={next:?}"
        )
    }
    fn span_message(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self::Span {
            contour,
            span,
            cause,
        } = self
        else {
            return Err(std::fmt::Error);
        };
        write!(out, "contour={contour}, span={span}: {cause}")
    }
    fn depth_clearance_message(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self::DepthClearance {
            contour,
            span,
            cause,
        } = self
        else {
            return Err(std::fmt::Error);
        };
        write!(
            out,
            "contour={contour}, span={span}, depth clearance: {cause:?}"
        )
    }
}
fn support_message(out: &mut std::fmt::Formatter<'_>, value: f64) -> std::fmt::Result {
    write!(out, "semi-angle={value}; expected 0 < angle < pi/2")
}
fn depths_message(out: &mut std::fmt::Formatter<'_>, depths: [f64; 2]) -> std::fmt::Result {
    write!(
        out,
        "depths={depths:?}; expected finite increasing endpoints"
    )
}
fn clearance_level_message(out: &mut std::fmt::Formatter<'_>, level: u8) -> std::fmt::Result {
    write!(out, "clearance level={level}; expected <= 64")
}
pub(super) fn source_bits(contours: &[Contour<'_>]) -> SourceBits {
    contours
        .iter()
        .map(|contour| {
            contour
                .spans
                .iter()
                .map(|span| span.iter().map(|p| p.map(f64::to_bits)).collect())
                .collect()
        })
        .collect()
}
pub(super) fn admit_contours(
    contours: &[Contour<'_>],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), BoundaryFailure> {
    if contours.is_empty() || contours.len() > 128 {
        return Err(BoundaryFailure::ContourCount(contours.len()));
    }
    let count: usize = contours.iter().map(|contour| contour.spans.len()).sum();
    if count > 128 {
        return Err(BoundaryFailure::TotalSpanCount(count));
    }
    for (index, contour) in contours.iter().enumerate() {
        admit_contour(index, contour, arithmetic)?;
    }
    Ok(())
}
pub(super) fn admit_contour(
    index: usize,
    contour: &Contour<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), BoundaryFailure> {
    if contour.spans.is_empty() {
        return Err(BoundaryFailure::SpanCount {
            contour: index,
            count: 0,
        });
    }
    for (span, poles) in contour.spans.iter().enumerate() {
        arithmetic
            .check_cancelled()
            .map_err(BoundaryFailure::Arithmetic)?;
        admit_poles(index, span, poles)?;
    }
    admit_closure(index, contour)
}
pub(super) fn admit_poles(
    contour: usize,
    span: usize,
    poles: &[[f64; 2]],
) -> Result<(), BoundaryFailure> {
    if !(2..=4).contains(&poles.len()) {
        return Err(BoundaryFailure::PoleCount {
            contour,
            span,
            count: poles.len(),
        });
    }
    admit_coordinates(contour, span, poles)
}
pub(super) fn sum_contours(
    contours: &[Contour<'_>],
    level: u8,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, BoundaryFailure> {
    let mut total = arithmetic.zero();
    for (index, contour) in contours.iter().enumerate() {
        let value = sum_contour(index, contour, level, arithmetic)?;
        total = arithmetic
            .add(&total, &value)
            .map_err(BoundaryFailure::Arithmetic)?;
    }
    Ok(total)
}
pub(super) fn sum_contour(
    index: usize,
    contour: &Contour<'_>,
    level: u8,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, BoundaryFailure> {
    let mut total = arithmetic.zero();
    for (span, poles) in contour.spans.iter().enumerate() {
        let value =
            integrate_span(poles, level, arithmetic).map_err(|cause| BoundaryFailure::Span {
                contour: index,
                span,
                cause,
            })?;
        total = arithmetic
            .add(&total, &value)
            .map_err(BoundaryFailure::Arithmetic)?;
    }
    if contour.reverse {
        total = arithmetic
            .subtract(&arithmetic.zero(), &total)
            .map_err(BoundaryFailure::Arithmetic)?;
    }
    Ok(total)
}

pub use super::volume::ConicalAuthoredContour;
pub use super::volume::ConicalAuthoredFlux;
pub use super::volume::ConicalAuthoredFluxError;
pub use super::volume::ConicalAuthoredFluxQuery;
pub use super::volume::ConicalAuthoredFluxStage;
pub use super::volume::certify_conical_authored_flux;

fn admit_closure(index: usize, contour: &Contour<'_>) -> Result<(), BoundaryFailure> {
    for (span, poles) in contour.spans.iter().enumerate() {
        let end = poles[poles.len() - 1];
        let next = contour.spans[(span + 1) % contour.spans.len()][0];
        if end != next {
            return Err(BoundaryFailure::Open {
                contour: index,
                span,
                end,
                next,
            });
        }
    }
    Ok(())
}

fn admit_coordinates(
    contour: usize,
    span: usize,
    poles: &[[f64; 2]],
) -> Result<(), BoundaryFailure> {
    for (pole, point) in poles.iter().enumerate() {
        admit_point(contour, span, pole, point)?;
    }
    Ok(())
}

fn admit_point(
    contour: usize,
    span: usize,
    pole: usize,
    point: &[f64; 2],
) -> Result<(), BoundaryFailure> {
    for (axis, value) in point.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(BoundaryFailure::NonFinite {
                contour,
                span,
                pole,
                axis,
                value,
            });
        }
    }
    Ok(())
}

fn inverse_message(out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    out.write_str("inverse-slant integral lower bound is not positive")
}
