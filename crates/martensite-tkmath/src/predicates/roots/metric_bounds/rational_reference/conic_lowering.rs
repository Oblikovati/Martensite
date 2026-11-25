//! An exact short affine conic arc and a certified binary64 rational representation.
use super::*;
use crate::predicates::roots::certificate_arithmetic::{ExactRatio, rounding::outward_binary64};
use std::cmp::Ordering;

/// An oriented segment whose endpoints are on the specified exact affine unit circle.
#[derive(Clone, Copy, Debug)]
pub struct RationalConicSegment {
    /// Exact binary64 translation in developed coordinates.
    pub center: [f64; 2],
    /// Exact affine columns from the unit-circle frame; their determinant must be positive.
    pub columns: [[f64; 2]; 2],
    /// Exact original endpoint coordinates, in authored parameter order.
    pub endpoints: [[f64; 2]; 2],
    /// Whether the native angle increases from the first endpoint to the second.
    pub increasing: bool,
}

/// Rounded rational coefficients and evidence relating them to the exact conic segment.
pub struct CertifiedRationalConicSegment {
    /// Literal rounded Cartesian poles. Endpoints retain the supplied binary64 bits.
    pub poles: [[f64; 2]; 3],
    /// Literal positive weights under the same stereographic parameter.
    pub weights: [f64; 3],
    /// Exact pole component enclosures for topology tests, indexed by pole and coordinate.
    pub control_enclosures: [[[f64; 2]; 2]; 3],
    /// Full-interval error between the exact source and the literal representation.
    pub transport: RationalReferenceBounds,
    /// Outward Green-area transfer error. Shared exact endpoints close the homotopy sides.
    pub area_transfer: f64,
}

/// The exact source or its bounded numeric transport could not be certified.
#[derive(Debug)]
#[non_exhaustive]
pub enum RationalConicSegmentFailure {
    /// At least one supplied binary64 coefficient was not finite.
    NonFinite(RationalConicSegment),
    /// Exact affine determinant must be strictly positive.
    Frame([[f64; 2]; 2]),
    /// One endpoint fails the exact affine unit-circle identity.
    Endpoint {
        /// First or last endpoint.
        index: usize,
        /// Actual endpoint coordinates on the developed plane.
        point: [f64; 2],
    },
    /// Endpoints and requested direction do not select a positive-length sweep of at most π/2.
    Sweep(RationalConicSegment),
    /// An exact coefficient has no finite binary64 enclosure.
    Rounding,
    /// The retained bounded arithmetic cause.
    Reference(RationalReferenceFailure),
}

type Lower<T> = Result<T, RationalConicSegmentFailure>;

impl std::fmt::Display for RationalConicSegmentFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "conic segment requires finite exact carrier endpoints and a positive short sweep: {self:?}"
        )
    }
}
impl std::error::Error for RationalConicSegmentFailure {}

/// Lower a short exact affine conic segment under the native stereographic map.
/// If `u0` and `u1` are its exact affine unit endpoints, the native angle is
/// `arg(u0) + 2 atan(t * cross(u0,u1)/(1 + dot(u0,u1)))`.
/// Example: `let piece = lower_rational_conic_segment(segment, Default::default(), || false)?;`.
pub fn lower_rational_conic_segment(
    segment: RationalConicSegment,
    options: CylindricalProfileOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Lower<CertifiedRationalConicSegment> {
    let policy = ArithmeticPolicy {
        fractional_bits: options.fractional_bits,
        max_operations: options.max_operations,
    };
    admit_policy([0., 0.], policy).map_err(RationalConicSegmentFailure::Reference)?;
    let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
    // Check cancellation before constructing the fixed-size exact source ratios.
    arithmetic.scalar(0.).map_err(refusal)?;
    let exact = exact_coefficients(segment)?;
    output(segment.endpoints, exact, &mut arithmetic)
}

struct ExactControls {
    poles: [[ExactRatio; 2]; 3],
    weights: [ExactRatio; 3],
}

type Point = [ExactRatio; 2];

fn exact_coefficients(source: RationalConicSegment) -> Lower<ExactControls> {
    let values = source
        .center
        .into_iter()
        .chain(source.columns.into_iter().flatten())
        .chain(source.endpoints.into_iter().flatten());
    if !values.into_iter().all(f64::is_finite) {
        return Err(RationalConicSegmentFailure::NonFinite(source));
    }
    let center = source.center.map(ExactRatio::from_finite);
    let columns = source.columns.map(|p| p.map(ExactRatio::from_finite));
    let endpoints = source.endpoints.map(|p| p.map(ExactRatio::from_finite));
    let determinant = cross(&columns[0], &columns[1]);
    if determinant.sign() != Ordering::Greater {
        return Err(RationalConicSegmentFailure::Frame(source.columns));
    }
    let unit = unit_endpoints(&source, &endpoints, &center, &columns, &determinant)?;
    let q = short_slope(&unit, source)?;
    let middle = tangent_control(&endpoints[0], &columns, &unit[0], &q);
    let [start, end] = endpoints;
    let one = ExactRatio::from_finite(1.);
    Ok(ExactControls {
        poles: [start, middle, end],
        weights: [one.clone(), one.clone(), one.add(&q.multiply(&q))],
    })
}

fn unit_endpoints(
    source: &RationalConicSegment,
    endpoints: &[Point; 2],
    center: &Point,
    columns: &[Point; 2],
    determinant: &ExactRatio,
) -> Lower<[Point; 2]> {
    let unit = endpoints.each_ref().map(|point| {
        let local: Point = std::array::from_fn(|axis| point[axis].subtract(&center[axis]));
        [
            cross(&local, &columns[1]).divide(determinant),
            cross(&columns[0], &local).divide(determinant),
        ]
    });
    let one = ExactRatio::from_finite(1.);
    for (index, coordinates) in unit.iter().enumerate() {
        if dot(coordinates, coordinates) != one {
            return Err(RationalConicSegmentFailure::Endpoint {
                index,
                point: source.endpoints[index],
            });
        }
    }
    Ok(unit)
}

fn short_slope(unit: &[Point; 2], source: RationalConicSegment) -> Lower<ExactRatio> {
    let angle_dot = dot(&unit[0], &unit[1]);
    let angle_cross = cross(&unit[0], &unit[1]);
    let sense = if source.increasing {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    if angle_dot.sign() == Ordering::Less || angle_cross.sign() != sense {
        return Err(RationalConicSegmentFailure::Sweep(source));
    }
    Ok(angle_cross.divide(&ExactRatio::from_finite(1.).add(&angle_dot)))
}

fn tangent_control(start: &Point, columns: &[Point; 2], unit: &Point, q: &ExactRatio) -> Point {
    std::array::from_fn(|axis| {
        let tangent = columns[1][axis]
            .multiply(&unit[0])
            .subtract(&columns[0][axis].multiply(&unit[1]));
        start[axis].add(&q.multiply(&tangent))
    })
}

fn cross(a: &Point, b: &Point) -> ExactRatio {
    a[0].multiply(&b[1]).subtract(&a[1].multiply(&b[0]))
}

fn dot(a: &Point, b: &Point) -> ExactRatio {
    a[0].multiply(&b[0]).add(&a[1].multiply(&b[1]))
}

fn output(
    endpoints: [[f64; 2]; 2],
    exact: ExactControls,
    arithmetic: &mut Arithmetic<'_>,
) -> Lower<CertifiedRationalConicSegment> {
    let mut control_enclosures = [[[0.; 2]; 2]; 3];
    for (bound, point) in control_enclosures.iter_mut().zip(&exact.poles) {
        [bound[0], bound[1]] = [rounded(&point[0])?, rounded(&point[1])?];
    }
    let poles = [
        endpoints[0],
        control_enclosures[1].map(|bound| bound[0]),
        endpoints[1],
    ];
    let weights = [1., 1., rounded(&exact.weights[2])?[0]];
    let original = capture_exact(&exact, arithmetic)?;
    let target = capture(
        MetricBezier {
            poles: &poles,
            weights: Some(&weights),
        },
        CylindricalProfileInterval::new(0, 0)
            .map_err(|cause| RationalConicSegmentFailure::Reference(cause.into()))?,
        &weights,
        arithmetic,
    )
    .map_err(RationalConicSegmentFailure::Reference)?;
    let (transport, area_transfer) = rational_difference::transport(&original, &target, arithmetic)
        .map_err(RationalConicSegmentFailure::Reference)?;
    Ok(CertifiedRationalConicSegment {
        poles,
        weights,
        control_enclosures,
        transport,
        area_transfer,
    })
}

fn capture_exact(exact: &ExactControls, arithmetic: &mut Arithmetic<'_>) -> Lower<Homogeneous> {
    let mut numerator = [Vec::with_capacity(3), Vec::with_capacity(3)];
    let mut weights = Vec::with_capacity(3);
    for (pole, weight) in exact.poles.iter().zip(&exact.weights) {
        for (coefficients, coordinate) in numerator.iter_mut().zip(pole) {
            coefficients.push(
                arithmetic
                    .rational(&coordinate.multiply(weight))
                    .map_err(refusal)?,
            );
        }
        weights.push(arithmetic.rational(weight).map_err(refusal)?);
    }
    Ok(Homogeneous {
        numerator,
        weight: weights,
    })
}

fn rounded(value: &ExactRatio) -> Lower<[f64; 2]> {
    outward_binary64(value).ok_or(RationalConicSegmentFailure::Rounding)
}

fn refusal(error: BoundedArithmeticFailure) -> RationalConicSegmentFailure {
    RationalConicSegmentFailure::Reference(error.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARC: RationalConicSegment = RationalConicSegment {
        center: [4., 0.],
        columns: [[5. / 128., 0.], [0., 5. / 128.]],
        endpoints: [[4. + 5. / 128., 0.], [4. + 4. / 128., 3. / 128.]],
        increasing: true,
    };

    #[test]
    fn non_dyadic_half_angle_is_a_small_certified_transport() {
        let fitted = lower_rational_conic_segment(ARC, Default::default(), || false).unwrap();
        assert_eq!([fitted.poles[0], fitted.poles[2]], ARC.endpoints);
        let [lo, hi] = fitted.control_enclosures[1][1];
        assert!(lo <= 5. / 384. && 5. / 384. <= hi && lo < hi);
        assert!((fitted.weights[2] - 10. / 9.).abs() <= f64::EPSILON);
        assert!(fitted.transport.position > 0. && fitted.transport.position < 1e-13);
        assert!(fitted.transport.derivative > 0. && fitted.transport.derivative < 1e-12);
    }

    #[test]
    fn direction_off_carrier_and_exhaustion_fail_typed() {
        let reversed = RationalConicSegment {
            increasing: false,
            ..ARC
        };
        assert!(matches!(
            lower_rational_conic_segment(reversed, Default::default(), || false),
            Err(RationalConicSegmentFailure::Sweep(_))
        ));
        let mut off = ARC;
        off.endpoints[1][1] = off.endpoints[1][1].next_up();
        assert!(matches!(
            lower_rational_conic_segment(off, Default::default(), || false),
            Err(RationalConicSegmentFailure::Endpoint { index: 1, .. })
        ));
        let low = CylindricalProfileOptions {
            max_operations: 1,
            ..Default::default()
        };
        assert!(matches!(
            lower_rational_conic_segment(ARC, low, || false),
            Err(RationalConicSegmentFailure::Reference(
                RationalReferenceFailure::Arithmetic(_)
            ))
        ));
    }
}
