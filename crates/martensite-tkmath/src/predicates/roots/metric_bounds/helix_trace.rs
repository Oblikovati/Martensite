// SPDX-License-Identifier: GPL-2.0-only
//! Whole-span evidence for a literal polynomial native spline against an authored helix.
mod model;
mod progress;

use super::{
    CylindricalProfileFailure, NativeSplineFailure, cylindrical_profile::algebra,
    rational_reference::native_spline::spatial_polynomial_pieces,
};
use crate::predicates::roots::DyadicBound;
use crate::predicates::roots::certificate_arithmetic::{
    bernstein_bounds as polynomial,
    bernstein_curve::bisect,
    bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
    bounded_failure::BoundedArithmeticFailure,
};
use model::PreparedLaw;
use progress::Progress;
use std::{error::Error, fmt};

/// Two independent literal scalars defining the ideal axial rise and total angle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HelixTraceLead {
    /// Axial rise per turn and number of turns, whose exact product gives the ideal height.
    PitchAndTurns([f64; 2]),
    /// Total axial rise, then number of turns.
    HeightAndTurns([f64; 2]),
    /// Axial rise per turn and total rise. Dividing total rise by pitch gives the exact turns.
    PitchAndHeight([f64; 2]),
}

impl HelixTraceLead {
    fn values(self) -> [f64; 2] {
        match self {
            Self::PitchAndTurns(pair) | Self::HeightAndTurns(pair) | Self::PitchAndHeight(pair) => {
                pair
            }
        }
    }
}

/// Literal law, traced with real angles and the original right-handed frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HelixTraceLaw {
    /// Center axis point at the opening station.
    pub origin: [f64; 3],
    /// Authored X, Y and Z frame columns. Z is also the progress direction.
    pub basis: [[f64; 3]; 3],
    /// Positive opening and closing radii.
    pub radii: [f64; 2],
    /// Original authored axial/rotation pair.
    pub lead: HelixTraceLead,
    /// True means positive rotation around the frame Z axis.
    pub positive_rotation: bool,
    /// Finite positive end of the actual spline's native interval `[0, parameter_end]`.
    pub parameter_end: f64,
}

/// Progress condition established on the actual retained polynomial.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelixSpanProgress {
    /// Strictly positive axial derivative on every knot interval.
    Advancing,
    /// A positive axial derivative inside each knot interval with stationary endpoints allowed.
    Ramp,
    /// A constant axial plane with strict progress along the endpoint chord.
    Flat,
}

/// Literal exact stations and planar axial controls of one ordered cylindrical piece.
#[derive(Clone, Copy, Debug)]
pub struct HelixCylindricalSpanLaw<'a> {
    /// Model-space center axis origin.
    pub origin: [f64; 3],
    /// Checked frame columns in X, Y, Z order.
    pub basis: [[f64; 3]; 3],
    /// Common positive cylindrical radius.
    pub radius: f64,
    /// Exact opening and closing turn stations.
    pub turns: [&'a DyadicBound; 2],
    /// Exact axial Bernstein controls in the full source span.
    pub axial_controls: &'a [DyadicBound],
    /// Actual active native parameter range.
    pub native_parameter: [f64; 2],
    /// True for rotation about positive frame Z.
    pub positive_rotation: bool,
    /// Required retained topology evidence for this span.
    pub progress: HelixSpanProgress,
}

/// Literal clamped, open, nonrational B-spline data read from a retained edge.
#[derive(Clone, Copy, Debug)]
pub struct HelixSplineSource<'a> {
    /// Polynomial degree from 1 to 8.
    pub degree: usize,
    /// Original XYZ poles.
    pub poles: &'a [[f64; 3]],
    /// Flat knot vector including repetitions.
    pub knots: &'a [f64],
    /// Actual edge trim, required to match the full proved domain.
    pub trim: [f64; 2],
}

/// Scoped spatial, representation and work limits for one trace proof.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HelixTraceLimits {
    /// Maximum admitted spatial L1 discrepancy in model units.
    pub max_position_error: f64,
    /// Maximum original native polynomial knot spans examined.
    pub max_native_spans: usize,
    /// Maximum proof cells examined, including bisections.
    pub max_cells: u32,
    /// Maximum fixed-grid operations across knot insertion and certification.
    pub max_operations: u32,
}

/// Whole-interval evidence with separate spatial, axial and work quantities.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HelixTraceBounds {
    /// Achieved outward same-parameter L1 position error in model units.
    pub position_error_upper: f64,
    /// Outward lower bound on the actual spline's world-axis derivative, including zero for flats or ramps.
    pub axial_derivative_lower: f64,
    /// Number of distinct original polynomial knot spans.
    pub native_spans: usize,
    /// Total proof cells examined.
    pub cells: u32,
    /// Total bounded arithmetic operations.
    pub operations: u32,
}

/// A refusal to assert whole-interval native source correspondence.
#[derive(Debug)]
#[non_exhaustive]
pub enum HelixTraceFailure {
    /// The literal law or its numeric limits are outside the admitted finite domain.
    InvalidLaw,
    /// The spline's literal representation dimensions or active trim do not match the request.
    InvalidSpline,
    /// Native span count exceeded the explicit bound.
    SpanLimit {
        /// Distinct spans requested.
        requested: usize,
        /// Maximum native spans admitted.
        limit: usize,
    },
    /// Adaptive proof exhausted its permitted cells without completing the enclosure.
    CellLimit {
        /// Proof cells already visited.
        completed: u32,
        /// Maximum proof cells admitted.
        limit: u32,
    },
    /// A cell cannot meet the requested trace limit within the bounded subdivision depth.
    PositionError {
        /// Outward residual bound for the unresolved cell.
        upper: f64,
        /// Maximum admitted spatial discrepancy.
        limit: f64,
    },
    /// A strictly advancing actual native axial coordinate could not be established.
    AxialProgress,
    /// The retained flat is outside one axial plane or cannot prove directed planar progress.
    FlatProgress,
    /// Refused native polynomial knot insertion.
    Spline(Box<NativeSplineFailure>),
    /// Outward arithmetic stopped before the proof completed.
    Arithmetic(Box<BoundedArithmeticFailure>),
    /// The reused rigorous trigonometric enclosure could not complete.
    Trigonometric(Box<CylindricalProfileFailure>),
    /// Caller cancellation was observed.
    Cancelled,
}

impl fmt::Display for HelixTraceFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            output,
            "bounded native helix trace could not complete: {self:?}"
        )
    }
}

impl Error for HelixTraceFailure {}

impl From<BoundedArithmeticFailure> for HelixTraceFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        match cause {
            BoundedArithmeticFailure::Cancelled => Self::Cancelled,
            other => Self::Arithmetic(Box::new(other)),
        }
    }
}

impl From<CylindricalProfileFailure> for HelixTraceFailure {
    fn from(cause: CylindricalProfileFailure) -> Self {
        match cause {
            CylindricalProfileFailure::Arithmetic(cause) => (*cause).into(),
            other => Self::Trigonometric(Box::new(other)),
        }
    }
}

fn insertion_failure(cause: NativeSplineFailure) -> HelixTraceFailure {
    use super::RationalReferenceFailure;
    match cause {
        NativeSplineFailure::Reference(RationalReferenceFailure::Arithmetic(cause)) => cause.into(),
        other => HelixTraceFailure::Spline(Box::new(other)),
    }
}

type ResultOf<T> = Result<T, HelixTraceFailure>;
type Poles = [Vec<Interval>; 3];
const MAX_LEVEL: u8 = 24;

/// Enclose the actual native spline's full trim and prove a simple monotone axial trace.
/// Use `certify_polynomial_helix_trace(law, spline, limits, || false)?` for an uncancellable caller.
pub fn certify_polynomial_helix_trace(
    law: HelixTraceLaw,
    spline: HelixSplineSource<'_>,
    limits: HelixTraceLimits,
    mut cancelled: impl FnMut() -> bool + Send,
) -> ResultOf<HelixTraceBounds> {
    if law
        .lead
        .values()
        .into_iter()
        .chain(law.radii)
        .chain([law.parameter_end])
        .any(|value| !value.is_finite() || value <= 0.)
    {
        return Err(HelixTraceFailure::InvalidLaw);
    }
    let extent = TraceExtent::new(law.origin, law.basis, [0., law.parameter_end]);
    certify(
        extent,
        spline,
        limits,
        HelixSpanProgress::Advancing,
        &mut cancelled,
        |arithmetic| PreparedLaw::prepare(law, arithmetic),
    )
}

/// Prove one actual native polynomial span against exact ordered cylindrical stations.
/// Use `certify_cylindrical_helix_span(law, source, limits, || false)?` for an uncancellable caller.
pub fn certify_cylindrical_helix_span(
    law: HelixCylindricalSpanLaw<'_>,
    spline: HelixSplineSource<'_>,
    limits: HelixTraceLimits,
    mut cancelled: impl FnMut() -> bool + Send,
) -> ResultOf<HelixTraceBounds> {
    let controls = match law.progress {
        HelixSpanProgress::Advancing => 2,
        HelixSpanProgress::Ramp => 5,
        HelixSpanProgress::Flat => 1,
    };
    if !law.radius.is_finite()
        || law.radius <= 0.
        || law.turns[0] >= law.turns[1]
        || law.axial_controls.len() != controls
        || law.axial_controls.windows(2).any(|pair| pair[0] > pair[1])
    {
        return Err(HelixTraceFailure::InvalidLaw);
    }
    let extent = TraceExtent::new(law.origin, law.basis, law.native_parameter);
    certify(
        extent,
        spline,
        limits,
        law.progress,
        &mut cancelled,
        |arithmetic| PreparedLaw::prepare_cylindrical(law, arithmetic),
    )
}

fn certify(
    extent: TraceExtent,
    spline: HelixSplineSource<'_>,
    limits: HelixTraceLimits,
    mode: HelixSpanProgress,
    cancelled: &mut (impl FnMut() -> bool + Send),
    prepare: impl FnOnce(&mut Arithmetic<'_>) -> ResultOf<PreparedLaw>,
) -> ResultOf<HelixTraceBounds> {
    let native_spans = admit(extent, spline, limits)?;
    let policy = ArithmeticPolicy {
        fractional_bits: 128,
        max_operations: limits.max_operations,
    };
    let mut arithmetic = Arithmetic::new(policy, cancelled);
    arithmetic.check_cancelled()?;
    let model = prepare(&mut arithmetic)?;
    let pieces =
        spatial_polynomial_pieces(spline.degree, spline.knots, spline.poles, &mut arithmetic)
            .map_err(insertion_failure)?;
    let progress = Progress::new(mode, spline, &model, &mut arithmetic)?;
    let mut proof = Proof {
        limits,
        model,
        arithmetic,
        cells: 0,
        progress,
        position: 0.,
    };
    let axial = proof.verify(pieces)?;
    proof.arithmetic.check_cancelled()?;
    Ok(HelixTraceBounds {
        position_error_upper: proof.position,
        axial_derivative_lower: axial,
        native_spans,
        cells: proof.cells,
        operations: proof.arithmetic.operations(),
    })
}

#[derive(Clone, Copy)]
struct TraceExtent {
    origin: [f64; 3],
    basis: [[f64; 3]; 3],
    parameter: [f64; 2],
}
impl TraceExtent {
    fn new(origin: [f64; 3], basis: [[f64; 3]; 3], parameter: [f64; 2]) -> Self {
        Self {
            origin,
            basis,
            parameter,
        }
    }
}

fn admit(
    extent: TraceExtent,
    spline: HelixSplineSource<'_>,
    limits: HelixTraceLimits,
) -> ResultOf<usize> {
    let [first, last] = extent.parameter;
    if !first.is_finite()
        || !last.is_finite()
        || first < 0.
        || last <= first
        || !limits.max_position_error.is_finite()
        || limits.max_position_error < 0.
    {
        return Err(HelixTraceFailure::InvalidLaw);
    }
    let [x, y, z] = extent.origin;
    let [x_axis, y_axis, z_axis] = extent.basis;
    if crate::gp::Axis2::from_stored_coefficients(
        crate::gp::Point3::new(x, y, z),
        z_axis,
        x_axis,
        y_axis,
    )
    .is_err()
    {
        return Err(HelixTraceFailure::InvalidLaw);
    }
    if !(1..=8).contains(&spline.degree)
        || spline.poles.len() <= spline.degree
        || spline.poles.len().checked_add(spline.degree + 1) != Some(spline.knots.len())
        || spline.trim != extent.parameter
        || spline.knots.first() != Some(&first)
        || spline.knots.last() != Some(&last)
    {
        return Err(HelixTraceFailure::InvalidSpline);
    }
    let lower = (spline.poles.len() - 1).div_ceil(spline.degree);
    let limit = limits.max_native_spans;
    if lower > limit {
        return Err(HelixTraceFailure::SpanLimit {
            requested: lower,
            limit,
        });
    }
    let count = spline
        .knots
        .windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count();
    if count > limit {
        return Err(HelixTraceFailure::SpanLimit {
            requested: count,
            limit,
        });
    }
    Ok(count)
}

struct Proof<'a> {
    limits: HelixTraceLimits,
    model: PreparedLaw,
    arithmetic: Arithmetic<'a>,
    cells: u32,
    position: f64,
    progress: Progress,
}

impl Proof<'_> {
    fn verify(
        &mut self,
        pieces: Vec<super::rational_reference::native_spline::SpatialSplinePiece>,
    ) -> ResultOf<f64> {
        let mut axial = f64::INFINITY;
        for (range, xyz) in pieces {
            let a = self.arithmetic.scalar(range[0])?;
            let b = self.arithmetic.scalar(range[1])?;
            let width = self.arithmetic.subtract(&b, &a)?;
            let parameters = [
                self.model.fraction(&a, &mut self.arithmetic)?,
                self.model.fraction(&b, &mut self.arithmetic)?,
            ];
            let poles = self.model.centered(xyz, &mut self.arithmetic)?;
            axial = axial.min(self.progress.check(
                &poles,
                &width,
                &self.model,
                &mut self.arithmetic,
            )?);
            self.check_cell(poles, parameters, 0)?;
        }
        if !axial.is_finite() || axial < 0. {
            return Err(HelixTraceFailure::AxialProgress);
        }
        Ok(axial)
    }

    fn check_cell(&mut self, poles: Poles, parameters: [Interval; 2], level: u8) -> ResultOf<()> {
        self.arithmetic.check_cancelled()?;
        if self.cells == self.limits.max_cells {
            return Err(HelixTraceFailure::CellLimit {
                completed: self.cells,
                limit: self.limits.max_cells,
            });
        }
        self.cells += 1;
        let upper = self.residual(&poles, &parameters)?;
        let limit = self.limits.max_position_error;
        if upper <= limit {
            self.position = self.position.max(upper);
            return Ok(());
        }
        if level == MAX_LEVEL {
            return Err(HelixTraceFailure::PositionError { upper, limit });
        }
        let [x, y, z] = poles;
        let [xl, xr] = bisect(&x, &mut self.arithmetic)?;
        let [yl, yr] = bisect(&y, &mut self.arithmetic)?;
        let [zl, zr] = bisect(&z, &mut self.arithmetic)?;
        let [a, b] = parameters;
        let sum = self.arithmetic.add(&a, &b)?;
        let middle = self.arithmetic.fraction(&sum, 1, 2)?;
        self.check_cell([xl, yl, zl], [a, middle.clone()], level + 1)?;
        self.check_cell([xr, yr, zr], [middle, b], level + 1)
    }

    fn residual(&mut self, native: &Poles, parameters: &[Interval; 2]) -> ResultOf<f64> {
        let ideal = self.model.ideal(parameters, &mut self.arithmetic)?;
        let mut error = 0.into();
        for (actual, reference) in native.iter().zip(ideal) {
            let difference = algebra::difference(actual, &reference, &mut self.arithmetic)?;
            error += polynomial::magnitude(&difference);
        }
        Ok(self.arithmetic.upper_f64(error)?)
    }
}

#[cfg(test)]
mod tests;
