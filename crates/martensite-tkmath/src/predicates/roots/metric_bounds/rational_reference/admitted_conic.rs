// SPDX-License-Identifier: GPL-2.0-only
//! Analytic conics with an independently captured literal sewn representation.
use super::super::cylindrical_profile::trigonometric;
use super::*;

type PlanePoint = [Interval; 2];
#[cfg(test)]
mod tests;

/// Admitted affine frame. Each column is the product of the supplied axis and scale.
#[derive(Clone, Copy, Debug)]
pub struct AdmittedConicFrame {
    /// Original admitted center.
    pub center: [f64; 2],
    /// Admitted major and minor basis directions in developed coordinates.
    pub directions: [[f64; 2]; 2],
    /// Positive column scales in the admitted frame.
    pub scales: [f64; 2],
}

/// Rational parameter used by the sewn piece on its admitted native angular interval.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConicSpanParameter {
    /// `theta(t) = middle + 2 atan((2t - 1) tan(step/4))`.
    Symmetric,
    /// `theta(t) = first + 2 atan(t tan(step/2))`.
    FirstEndpoint,
}

/// Native angular interval associated with one sewn conic piece.
#[derive(Clone, Copy, Debug)]
pub enum AdmittedConicPhase {
    /// This piece occupies equal exact fractions of one admitted binary64 sweep.
    Sweep {
        /// Original admitted first angle.
        first: f64,
        /// Original admitted signed sweep.
        sweep: f64,
        /// Zero-based index in increasing authored parameter order.
        index: u8,
        /// Number of equal pieces covering the admitted sweep.
        count: u8,
        /// Parameter of the captured sewn rational piece.
        parameter: ConicSpanParameter,
    },
    /// Exact quarter of a closed conic. Zero is the native major-axis seam.
    Quarter(u8),
}

/// The actual weights and authored sewing points of one admitted analytic piece.
#[derive(Clone, Copy, Debug)]
pub struct AdmittedConicSpan {
    /// Admitted analytic frame.
    pub frame: AdmittedConicFrame,
    /// Authoritative native angle and parameter.
    pub phase: AdmittedConicPhase,
    /// Literal poles used by the planar region proof, including sewn endpoints.
    pub poles: [[f64; 2]; 3],
    /// Literal positive weights used by the planar region proof.
    pub weights: [f64; 3],
}

/// Bounds from the admitted analytic curve to its literal sewn planar curve.
pub struct CertifiedAdmittedConicSpan {
    /// Full-interval position and local derivative discrepancy and arithmetic cost.
    pub transport: RationalReferenceBounds,
    /// Outward error for Green area, also including the two endpoint connectors.
    pub area_transfer: f64,
    /// Maximum developed L1 error between analytic and the literal sewn endpoint.
    pub endpoint_error: f64,
}

/// Compare a sewn rational conic span to the admitted angle and carrier.
/// Example: `let proof = certify_admitted_conic_span(piece, Default::default(), || false)?;`.
pub fn certify_admitted_conic_span(
    piece: AdmittedConicSpan,
    options: CylindricalProfileOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Checked<CertifiedAdmittedConicSpan> {
    let policy = ArithmeticPolicy {
        fractional_bits: options.fractional_bits,
        max_operations: options.max_operations,
    };
    admit_policy([0., 0.], policy)?;
    let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
    let target = capture_literal(&piece, &mut arithmetic)?;
    let (source, connector) = capture_source(piece, &mut arithmetic)?;
    let endpoints = endpoint_bounds(&source, &target, connector, &mut arithmetic)?;
    let (mut transport, area) = rational_difference::transport(&source, &target, &mut arithmetic)?;
    let area_transfer = (area + endpoints.connector_area).next_up();
    transport.operations = arithmetic.operations();
    Ok(CertifiedAdmittedConicSpan {
        transport,
        area_transfer,
        endpoint_error: endpoints.max_gap,
    })
}

fn capture_literal(
    piece: &AdmittedConicSpan,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Homogeneous> {
    let source = MetricBezier {
        poles: &piece.poles,
        weights: Some(&piece.weights),
    };
    checked_shape(source, 3)?;
    capture(
        source,
        CylindricalProfileInterval::new(0, 0).map_err(Refusal::from)?,
        &piece.weights,
        arithmetic,
    )
}

struct Frame {
    center: PlanePoint,
    columns: [PlanePoint; 2],
}

impl Frame {
    fn capture(frame: AdmittedConicFrame, arithmetic: &mut Arithmetic<'_>) -> Checked<Self> {
        if !frame
            .scales
            .into_iter()
            .all(|scale| scale.is_finite() && scale > 0.)
        {
            return Err(Refusal::ConicFrame(frame));
        }
        let center = [
            arithmetic.scalar(frame.center[0])?,
            arithmetic.scalar(frame.center[1])?,
        ];
        let columns = [
            column(frame.directions[0], frame.scales[0], arithmetic)?,
            column(frame.directions[1], frame.scales[1], arithmetic)?,
        ];
        let main = arithmetic.multiply(&columns[0][0], &columns[1][1])?;
        let anti = arithmetic.multiply(&columns[1][0], &columns[0][1])?;
        if arithmetic.subtract(&main, &anti)?.lower <= 0.into() {
            return Err(Refusal::ConicFrame(frame));
        }
        Ok(Self { center, columns })
    }

    fn numerator(
        &self,
        unit: &PlanePoint,
        weight: &Interval,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Checked<PlanePoint> {
        let mut point = self.center.clone();
        for (axis, component) in point.iter_mut().enumerate() {
            let center = arithmetic.multiply(component, weight)?;
            let x = arithmetic.multiply(&self.columns[0][axis], &unit[0])?;
            let y = arithmetic.multiply(&self.columns[1][axis], &unit[1])?;
            let vector = arithmetic.add(&x, &y)?;
            *component = arithmetic.add(&center, &vector)?;
        }
        Ok(point)
    }
}

fn column(direction: [f64; 2], scale: f64, arithmetic: &mut Arithmetic<'_>) -> Checked<PlanePoint> {
    let x = arithmetic.scalar(direction[0])?;
    let y = arithmetic.scalar(direction[1])?;
    let scale = arithmetic.scalar(scale)?;
    Ok([
        arithmetic.multiply(&x, &scale)?,
        arithmetic.multiply(&y, &scale)?,
    ])
}

fn capture_source(
    piece: AdmittedConicSpan,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<(Homogeneous, PlanePoint)> {
    let frame = Frame::capture(piece.frame, arithmetic)?;
    let (unit, weights) = unit_bernstein(piece.phase, arithmetic)?;
    let [a, b, c] = unit;
    let [u, v, w] = &weights;
    let [ax, ay] = frame.numerator(&a, u, arithmetic)?;
    let [bx, by] = frame.numerator(&b, v, arithmetic)?;
    let [cx, cy] = frame.numerator(&c, w, arithmetic)?;
    let source = Homogeneous {
        numerator: [vec![ax, bx, cx], vec![ay, by, cy]],
        weight: weights.into(),
    };
    Ok((source, frame.center))
}

fn unit_bernstein(
    phase: AdmittedConicPhase,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<([PlanePoint; 3], [Interval; 3])> {
    match phase {
        AdmittedConicPhase::Quarter(first) => quarter(first, arithmetic),
        AdmittedConicPhase::Sweep {
            first,
            sweep,
            index,
            count,
            parameter,
        } => sweep_controls(first, sweep, index, count, parameter, arithmetic),
    }
}

fn quarter(
    first: u8,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<([PlanePoint; 3], [Interval; 3])> {
    let axes = [[1., 0.], [0., 1.], [-1., 0.], [0., -1.]];
    let [a, b] = [
        axes[usize::from(first % 4)],
        axes[usize::from((first % 4 + 1) % 4)],
    ];
    let a = [arithmetic.scalar(a[0])?, arithmetic.scalar(a[1])?];
    let b = [arithmetic.scalar(b[0])?, arithmetic.scalar(b[1])?];
    let one = arithmetic.scalar(1.)?;
    let two = arithmetic.scalar(2.)?;
    let middle = [arithmetic.add(&a[0], &b[0])?, arithmetic.add(&a[1], &b[1])?];
    let last = [
        arithmetic.multiply(&b[0], &two)?,
        arithmetic.multiply(&b[1], &two)?,
    ];
    Ok(([a, middle, last], [one.clone(), one, two]))
}

fn sweep_controls(
    first: f64,
    sweep: f64,
    index: u8,
    count: u8,
    parameter: ConicSpanParameter,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<([PlanePoint; 3], [Interval; 3])> {
    valid_phase(first, sweep, index, count)?;
    let trig = Series::capture(first, sweep, count, arithmetic)?;
    let index = 2 * u64::from(index);
    let a = trig.at(index, arithmetic)?;
    let b = trig.at(index + 2, arithmetic)?;
    let one = arithmetic.scalar(1.)?;
    let [cosine, sine] = trigonometric::scalar(&trig.half_step, arithmetic)?;
    match parameter {
        ConicSpanParameter::Symmetric => Ok((
            [a, trig.at(index + 1, arithmetic)?, b],
            [one.clone(), cosine, one],
        )),
        ConicSpanParameter::FirstEndpoint => first_endpoint(a, b, [cosine, sine], one, arithmetic),
    }
}

fn valid_phase(first: f64, sweep: f64, index: u8, count: u8) -> Checked<()> {
    let finite = first.is_finite() && sweep.is_finite();
    let split = (1..=5).contains(&count) && index < count;
    let angular = first.abs() <= 4.
        && sweep.abs() <= 7.
        && sweep != 0.
        && sweep.abs() / f64::from(count) <= 1.6;
    if finite && split && angular {
        return Ok(());
    }
    Err(Refusal::ConicPhase {
        first,
        sweep,
        index,
        count,
    })
}

struct Series {
    start: PlanePoint,
    half_step: Interval,
}

impl Series {
    fn capture(
        first: f64,
        sweep: f64,
        count: u8,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Checked<Self> {
        let first = arithmetic.scalar(first)?;
        let sweep = arithmetic.scalar(sweep)?;
        let start = trigonometric::scalar(&first, arithmetic)?;
        let half_step = arithmetic.fraction(&sweep, 1, 2 * u64::from(count))?;
        Ok(Self { start, half_step })
    }

    fn at(&self, numerator: u64, arithmetic: &mut Arithmetic<'_>) -> Checked<PlanePoint> {
        let delta = arithmetic.fraction(&self.half_step, numerator, 1)?;
        let [c, s] = trigonometric::scalar(&delta, arithmetic)?;
        let [x, y] = &self.start;
        let [xc, ys] = [arithmetic.multiply(x, &c)?, arithmetic.multiply(y, &s)?];
        let [xs, yc] = [arithmetic.multiply(x, &s)?, arithmetic.multiply(y, &c)?];
        Ok([arithmetic.subtract(&xc, &ys)?, arithmetic.add(&xs, &yc)?])
    }
}

fn first_endpoint(
    a: PlanePoint,
    b: PlanePoint,
    [cosine, sine]: PlanePoint,
    one: Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<([PlanePoint; 3], [Interval; 3])> {
    let q = arithmetic.divide(&sine, &cosine)?;
    let squared = arithmetic.multiply(&q, &q)?;
    let weight = arithmetic.add(&one, &squared)?;
    let dx = arithmetic.multiply(&a[1], &q)?;
    let dy = arithmetic.multiply(&a[0], &q)?;
    let middle = [
        arithmetic.subtract(&a[0], &dx)?,
        arithmetic.add(&a[1], &dy)?,
    ];
    let last = [
        arithmetic.multiply(&b[0], &weight)?,
        arithmetic.multiply(&b[1], &weight)?,
    ];
    Ok(([a, middle, last], [one.clone(), one, weight]))
}

struct Endpoints {
    max_gap: f64,
    connector_area: f64,
}

fn endpoint_bounds(
    source: &Homogeneous,
    target: &Homogeneous,
    center: PlanePoint,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Endpoints> {
    let mut total = arithmetic.zero();
    let mut max_gap: f64 = 0.;
    for index in [0, 2] {
        let a = cartesian(source, index, arithmetic)?;
        let b = cartesian(target, index, arithmetic)?;
        let delta = [
            arithmetic.subtract(&b[0], &a[0])?,
            arithmetic.subtract(&b[1], &a[1])?,
        ];
        let connector = connector_area(&a, &delta, &center, arithmetic)?;
        let gap = arithmetic.upper_f64(delta[0].magnitude() + delta[1].magnitude())?;
        max_gap = max_gap.max(gap);
        total = arithmetic.add(&total, &connector)?;
    }
    Ok(Endpoints {
        max_gap,
        connector_area: arithmetic.upper_f64(total.upper)?,
    })
}

fn cartesian(
    curve: &Homogeneous,
    index: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<PlanePoint> {
    let weight = &curve.weight[index];
    Ok([
        arithmetic.divide(&curve.numerator[0][index], weight)?,
        arithmetic.divide(&curve.numerator[1][index], weight)?,
    ])
}

fn connector_area(
    ideal: &PlanePoint,
    delta: &PlanePoint,
    center: &PlanePoint,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let x = arithmetic.subtract(&ideal[0], &center[0])?;
    let y = arithmetic.subtract(&ideal[1], &center[1])?;
    let left = arithmetic.multiply(&x, &delta[1])?;
    let right = arithmetic.multiply(&y, &delta[0])?;
    let area = arithmetic.subtract(&left, &right)?.magnitude();
    let abs = Interval {
        lower: area.clone(),
        upper: area,
    };
    Ok(arithmetic.fraction(&abs, 1, 2)?)
}
