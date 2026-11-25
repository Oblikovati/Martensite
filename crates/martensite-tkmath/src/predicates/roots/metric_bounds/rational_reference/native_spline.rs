// SPDX-License-Identifier: GPL-2.0-only
//! Outward native B-spline knot insertion with a same-parameter literal transport.
use super::*;
use partition::{Insertion, Span, check_knots, decompose};
mod partition;
#[cfg(test)]
mod tests;

const MAX_SPANS: usize = 64;

type GridPole = [Interval; 3];

/// Original literal clamped B-spline coefficients and native knots.
#[derive(Clone, Copy, Debug)]
pub struct NativeSplineSource<'a> {
    /// Degree in the supported range from 1 through 16.
    pub degree: usize,
    /// Cartesian poles in original order.
    pub poles: &'a [[f64; 2]],
    /// Original positive weights, or `None` for a polynomial spline.
    pub weights: Option<&'a [f64]>,
    /// Repeated native knots, including the two clamped endpoints.
    pub knots: &'a [f64],
}

impl NativeSplineSource<'_> {
    /// Check the bounded pole and flat-knot counts before reading their contents.
    /// Example: `source.check_extent()?;`.
    pub fn check_extent(self) -> NativeResult<()> {
        let [poles, knots] = [self.poles.len(), self.knots.len()];
        let degree = self.degree;
        if !(1..=16).contains(&degree)
            || poles <= degree
            || poles.checked_add(degree + 1) != Some(knots)
        {
            return Err(NativeSplineFailure::Knots {
                degree,
                poles,
                knots,
            });
        }
        let minimum_spans = (poles - 1).div_ceil(degree);
        if minimum_spans > MAX_SPANS {
            return Err(NativeSplineFailure::SpanCount {
                spans: minimum_spans,
                limit: MAX_SPANS,
            });
        }
        Ok(())
    }
}

/// Literal coefficients and an outward proof against an original native span.
pub struct CertifiedNativeSplineSpan {
    /// Exact native binary64 interval endpoints, without normalization.
    pub interval: [f64; 2],
    /// Cartesian Bézier poles with identical bits at all shared seams.
    pub poles: Vec<[f64; 2]>,
    /// Literal positive weights in the local affine parameter.
    pub weights: Option<Vec<f64>>,
    /// Outward control enclosures including both the exact and literal poles.
    pub control_enclosures: Vec<[[f64; 2]; 2]>,
    /// Position, local derivative and incremental arithmetic cost.
    pub transport: RationalReferenceBounds,
    /// Outward absolute Green-area homotopy strip contribution.
    pub area_transfer: f64,
}

/// A native B-spline could not be partitioned or bounded under the given policy.
#[derive(Debug)]
#[non_exhaustive]
pub enum NativeSplineFailure {
    /// Malformed, nonfinite, unclamped or discontinuous native knot vector.
    Knots {
        /// Requested native degree.
        degree: usize,
        /// Number of supplied poles.
        poles: usize,
        /// Number of supplied flat knots.
        knots: usize,
    },
    /// Certificate's supported work shape was exceeded.
    SpanCount {
        /// Lower bound on the number of required spans.
        spans: usize,
        /// Maximum number of spans.
        limit: usize,
    },
    /// No stable finite Cartesian rounding could be retained.
    Rounding {
        /// Bounds of the exact Cartesian coordinate.
        coordinate: [f64; 2],
        /// Literal homogeneous weight.
        weight: f64,
    },
    /// Retained bounded arithmetic or rational transport failure.
    Reference(RationalReferenceFailure),
}

impl std::fmt::Display for NativeSplineFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "native spline requires finite clamped knots and bounded positive-weight transport: {self:?}"
        )
    }
}
impl std::error::Error for NativeSplineFailure {}

impl From<Refusal> for NativeSplineFailure {
    fn from(cause: Refusal) -> Self {
        Self::Reference(cause)
    }
}

impl From<BoundedArithmeticFailure> for NativeSplineFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Reference(cause.into())
    }
}

type NativeResult<T> = Result<T, NativeSplineFailure>;

/// Replay the literal homogeneous binary64 B-spline decomposition.
/// Example: `let pieces = partition_homogeneous_bspline(degree, &knots, &poles)?;`.
pub fn partition_homogeneous_bspline(
    degree: usize,
    knots: &[f64],
    poles: &[[f64; 3]],
) -> NativeResult<Vec<Vec<[f64; 3]>>> {
    check_knots(degree, poles.len(), knots)?;
    Ok(decompose(degree, knots, poles, partition::Literal)
        .unwrap_or_else(|impossible| match impossible {})
        .into_iter()
        .map(|span| span.poles)
        .collect())
}

/// Certify knot insertion and rounded Bézier transport for a literal native B-spline.
/// Example: `let pieces = lower_native_bspline(source, Default::default(), || false)?;`.
pub fn lower_native_bspline(
    source: NativeSplineSource<'_>,
    options: CylindricalProfileOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> NativeResult<Vec<CertifiedNativeSplineSpan>> {
    let policy = ArithmeticPolicy {
        fractional_bits: options.fractional_bits,
        max_operations: options.max_operations,
    };
    admit_policy([0., 0.], policy)?;
    let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
    arithmetic.check_cancelled()?;
    admit_spline(source)?;
    let literal = input_poles(source, &mut arithmetic)?;
    let pieces = decompose(source.degree, source.knots, &literal, &mut arithmetic)?;
    certify_pieces(source, pieces, &mut arithmetic)
}

fn admit_spline(source: NativeSplineSource<'_>) -> NativeResult<()> {
    source.check_extent()?;
    check_knots(source.degree, source.poles.len(), source.knots)?;
    let spans = source
        .knots
        .windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count();
    if spans > MAX_SPANS {
        return Err(NativeSplineFailure::SpanCount {
            spans,
            limit: MAX_SPANS,
        });
    }
    if let Some(weights) = source.weights {
        if weights.len() != source.poles.len() {
            return Err(Refusal::Shape {
                source: source.poles.len(),
                weights: Some(weights.len()),
                reference: 0,
            }
            .into());
        }
        for (index, &value) in weights.iter().enumerate() {
            if !value.is_finite() || value <= 0. {
                return Err(Refusal::Weight { index, value }.into());
            }
        }
    }
    Ok(())
}

pub(in crate::predicates::roots::metric_bounds) type SpatialSplinePiece =
    ([f64; 2], Vec<[Interval; 3]>);

pub(in crate::predicates::roots::metric_bounds) fn spatial_polynomial_pieces(
    degree: usize,
    knots: &[f64],
    poles: &[[f64; 3]],
    arithmetic: &mut Arithmetic<'_>,
) -> NativeResult<Vec<SpatialSplinePiece>> {
    check_knots(degree, poles.len(), knots)?;
    let mut coefficients = Vec::with_capacity(poles.len());
    for &[x, y, z] in poles {
        coefficients.push([
            arithmetic.scalar(x)?,
            arithmetic.scalar(y)?,
            arithmetic.scalar(z)?,
        ]);
    }
    let spans = decompose(degree, knots, &coefficients, arithmetic)?;
    Ok(spans
        .into_iter()
        .map(|span| (span.interval, span.poles))
        .collect())
}

fn input_poles(
    source: NativeSplineSource<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> NativeResult<Vec<GridPole>> {
    source
        .poles
        .iter()
        .enumerate()
        .map(|(index, &[x, y])| {
            let w = arithmetic.scalar(source.weights.map_or(1., |weights| weights[index]))?;
            let x = arithmetic.scalar(x)?;
            let y = arithmetic.scalar(y)?;
            Ok([
                arithmetic.multiply(&x, &w)?,
                arithmetic.multiply(&y, &w)?,
                w,
            ])
        })
        .collect()
}

impl Insertion<GridPole, (Interval, Interval), BoundedArithmeticFailure> for &mut Arithmetic<'_> {
    fn ratio(
        &mut self,
        first: f64,
        upper: f64,
        knot: f64,
    ) -> Result<(Interval, Interval), BoundedArithmeticFailure> {
        let first = self.scalar(first)?;
        let upper = self.scalar(upper)?;
        let knot = self.scalar(knot)?;
        let width = self.subtract(&upper, &first)?;
        let a = self.subtract(&knot, &first)?;
        let b = self.subtract(&upper, &knot)?;
        Ok((self.divide(&a, &width)?, self.divide(&b, &width)?))
    }

    fn blend(
        &mut self,
        (alpha, beta): &(Interval, Interval),
        a: &GridPole,
        b: &GridPole,
    ) -> Result<GridPole, BoundedArithmeticFailure> {
        let mut p = [self.zero(), self.zero(), self.zero()];
        for (index, axis) in p.iter_mut().enumerate() {
            let x = self.multiply(alpha, &a[index])?;
            let y = self.multiply(beta, &b[index])?;
            *axis = self.add(&x, &y)?;
        }
        Ok(p)
    }
}

fn certify_pieces(
    source: NativeSplineSource<'_>,
    pieces: Vec<Span<GridPole>>,
    arithmetic: &mut Arithmetic<'_>,
) -> NativeResult<Vec<CertifiedNativeSplineSpan>> {
    let count = pieces.len();
    let mut accounted = 0;
    pieces
        .into_iter()
        .enumerate()
        .map(|(index, piece)| {
            let ends = [
                (index == 0).then(|| source.poles[0]),
                (index + 1 == count).then(|| source.poles[source.poles.len() - 1]),
            ];
            let mut span = certify_piece(piece, ends, source.weights.is_some(), arithmetic)?;
            span.transport.operations -= accounted;
            accounted = arithmetic.operations();
            Ok(span)
        })
        .collect()
}

fn certify_piece(
    piece: Span<GridPole>,
    ends: [Option<[f64; 2]>; 2],
    rational: bool,
    arithmetic: &mut Arithmetic<'_>,
) -> NativeResult<CertifiedNativeSplineSpan> {
    let literal = round_poles(&piece.poles, ends, rational, arithmetic)?;
    let original = homogeneous(piece.poles);
    let (transport, area_transfer) = compare_literal(&original, &literal, arithmetic)?;
    Ok(CertifiedNativeSplineSpan {
        interval: piece.interval,
        poles: literal.poles,
        weights: rational.then_some(literal.weights),
        control_enclosures: literal.enclosures,
        transport,
        area_transfer,
    })
}

struct LiteralPoles {
    poles: Vec<[f64; 2]>,
    weights: Vec<f64>,
    enclosures: Vec<[[f64; 2]; 2]>,
}

fn round_poles(
    grid: &[GridPole],
    ends: [Option<[f64; 2]>; 2],
    rational: bool,
    arithmetic: &mut Arithmetic<'_>,
) -> NativeResult<LiteralPoles> {
    let mut literal = LiteralPoles {
        poles: Vec::new(),
        weights: Vec::new(),
        enclosures: Vec::new(),
    };
    for (index, [x, y, w]) in grid.iter().enumerate() {
        let xy = [arithmetic.divide(x, w)?, arithmetic.divide(y, w)?];
        let weight = if rational {
            outward(w, arithmetic)?[0]
        } else {
            1.
        };
        let mut enclosure = [outward(&xy[0], arithmetic)?, outward(&xy[1], arithmetic)?];
        let endpoint = if index == 0 {
            ends[0]
        } else if index + 1 == grid.len() {
            ends[1]
        } else {
            None
        };
        let pole = literal_pole(endpoint, &mut enclosure, weight)?;
        literal.poles.push(pole);
        literal.weights.push(weight);
        literal.enclosures.push(enclosure);
    }
    Ok(literal)
}

fn literal_pole(
    endpoint: Option<[f64; 2]>,
    enclosure: &mut [[f64; 2]; 2],
    weight: f64,
) -> NativeResult<[f64; 2]> {
    if !(weight.is_finite() && weight > 0.) {
        return Err(NativeSplineFailure::Rounding {
            coordinate: enclosure[0],
            weight,
        });
    }
    if let Some(pole) = endpoint {
        return Ok(pole);
    }
    let point = [
        stable_round(enclosure[0], weight)?,
        stable_round(enclosure[1], weight)?,
    ];
    for (bound, value) in enclosure.iter_mut().zip(point) {
        *bound = [bound[0].min(value), bound[1].max(value)];
    }
    Ok(point)
}

fn stable_round(coordinate: [f64; 2], weight: f64) -> NativeResult<f64> {
    // TKBO's planar replay converts through a homogeneous pole. Choose a fixed
    // point so the stored literal planar proof and the transport certify one curve.
    let [a, b] = coordinate.map(|value| (value * weight) / weight);
    [a, b]
        .into_iter()
        .find(|&value| {
            value.is_finite() && ((value * weight) / weight).to_bits() == value.to_bits()
        })
        .ok_or(NativeSplineFailure::Rounding { coordinate, weight })
}

fn outward(interval: &Interval, arithmetic: &Arithmetic<'_>) -> NativeResult<[f64; 2]> {
    Ok([
        -arithmetic.upper_f64(-&interval.lower)?,
        arithmetic.upper_f64(interval.upper.clone())?,
    ])
}

fn homogeneous(poles: Vec<GridPole>) -> Homogeneous {
    let mut coefficients = [Vec::new(), Vec::new(), Vec::new()];
    for pole in poles {
        for (axis, value) in coefficients.iter_mut().zip(pole) {
            axis.push(value);
        }
    }
    let [x, y, weight] = coefficients;
    Homogeneous {
        numerator: [x, y],
        weight,
    }
}

fn compare_literal(
    original: &Homogeneous,
    literal: &LiteralPoles,
    arithmetic: &mut Arithmetic<'_>,
) -> NativeResult<(RationalReferenceBounds, f64)> {
    let target = capture(
        MetricBezier {
            poles: &literal.poles,
            weights: Some(&literal.weights),
        },
        CylindricalProfileInterval::new(0, 0).map_err(Refusal::from)?,
        &literal.weights,
        arithmetic,
    )?;
    Ok(rational_difference::transport(
        original, &target, arithmetic,
    )?)
}
