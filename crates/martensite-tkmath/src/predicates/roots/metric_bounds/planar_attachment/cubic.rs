// SPDX-License-Identifier: GPL-2.0-only
//! Bounded exact Bernstein trim evidence for synchronized planar polynomial paths.
use super::{
    CertifiedPlanarAttachment, ExactRatio as R, PlanarAttachmentError as Attachment,
    PlanarAttachmentLocation as Location, PlanarCurvePath as Path,
    chart::{Projection, Uv},
    outward_sqrt, poll, trim,
};
use std::{cmp::Ordering, error::Error, fmt};

const MAX_SUBDIVISIONS: u32 = 4096;
const MAX_DEPTH: u8 = 32;

/// Independent spatial and exact trimming work limits for a paired planar cubic.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarCubicOptions {
    /// Inclusive world Euclidean position error limit.
    pub distance_limit: f64,
    /// Maximum completed half subdivisions per path, between zero and 4096.
    /// Exact parameter cells also have a fixed maximum dyadic depth of 32.
    pub max_subdivisions: u32,
}

/// One original chart coordinate proved strictly monotone for both full spans.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanarMonotoneAxis {
    /// Increasing U.
    IncreasingU,
    /// Decreasing U.
    DecreasingU,
    /// Increasing V.
    IncreasingV,
    /// Decreasing V.
    DecreasingV,
}

/// Complete native cubic trim and synchronized-pair evidence.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarCubicCertificate {
    /// The original requested limits.
    pub options: PlanarCubicOptions,
    /// Common injective direction in the original chart.
    pub monotone: PlanarMonotoneAxis,
    /// Completed subdivisions for world projection and UV lift, respectively.
    pub subdivisions: [u32; 2],
    /// Whole-span Euclidean error between the actual spatial curve and lifted pcurve.
    pub distance_bound: f64,
}

/// The same exact trim and synchronized-pair evidence for a stored quadratic.
pub type PlanarQuadraticCertificate = PlanarCubicCertificate;

/// Why the bounded whole-trim cubic certificate could not complete.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum PlanarCubicProofFailure {
    /// The caller requested unsupported subdivision work.
    InvalidSubdivisionLimit(u32),
    /// Both paths require the same strictly monotone original chart direction.
    Monotone,
    /// An exact value proves entry to the excluded trim.
    Trim {
        /// The actual path with the certified excluded point.
        path: Path,
        /// Exact excluded location.
        location: Location,
        /// Exact dyadic parameter of the excluded point.
        parameter: f64,
    },
    /// The allowed work ended before every span was certified.
    Incomplete {
        /// The actual path with an unresolved subspan.
        path: Path,
        /// Exact dyadic bounds of the unresolved subspan.
        parameter: [f64; 2],
        /// Completed half subdivisions on this path.
        subdivisions: u32,
    },
    /// Original chart, error-limit or cancellation cause.
    Attachment(Attachment),
}

impl fmt::Display for PlanarCubicProofFailure {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSubdivisionLimit(limit) => write!(
                out,
                "cubic trim subdivision limit {limit} must be <= {MAX_SUBDIVISIONS}"
            ),
            Self::Monotone => {
                out.write_str("both planar cubic paths require a common strictly monotone U or V")
            }
            Self::Trim {
                path,
                location,
                parameter,
            } => write!(out, "{path:?} enters {location:?} at {parameter}"),
            Self::Incomplete {
                path,
                parameter,
                subdivisions,
            } => write!(
                out,
                "{path:?} trim on {parameter:?} remains incomplete after {subdivisions} subdivisions"
            ),
            Self::Attachment(error) => error.fmt(out),
        }
    }
}

impl Error for PlanarCubicProofFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Attachment(cause) => Some(cause),
            _ => None,
        }
    }
}

impl From<Attachment> for PlanarCubicProofFailure {
    fn from(cause: Attachment) -> Self {
        Self::Attachment(cause)
    }
}

type Failure = PlanarCubicProofFailure;
type Net<const N: usize> = [Uv; N];
type ProjectionCollector<const N: usize> =
    fn([Result<Projection, Attachment>; N]) -> Result<[Projection; N], Attachment>;

impl CertifiedPlanarAttachment {
    /// Prove two original unweighted cubic nets over the common closed [0, 1] span.
    /// Example: `face.certify_cubic_curve(world_poles, uv_poles, limits, || false)?`.
    pub fn certify_cubic_curve(
        &self,
        world: [[f64; 3]; 4],
        uv: [[f64; 2]; 4],
        options: PlanarCubicOptions,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarCubicCertificate, Failure> {
        self.certify_polynomial(world, uv, options, &mut cancelled, cubic_projection)
    }

    /// Prove two original unweighted quadratic nets over the common closed interval.
    /// Example: `face.certify_quadratic_curve(world_poles, uv_poles, limits, || false)?`.
    pub fn certify_quadratic_curve(
        &self,
        world: [[f64; 3]; 3],
        uv: [[f64; 2]; 3],
        options: PlanarCubicOptions,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarQuadraticCertificate, Failure> {
        self.certify_polynomial(world, uv, options, &mut cancelled, quadratic_projection)
    }

    fn certify_polynomial<const N: usize>(
        &self,
        world: [[f64; 3]; N],
        uv: [[f64; 2]; N],
        options: PlanarCubicOptions,
        cancelled: &mut impl FnMut() -> bool,
        collect: ProjectionCollector<N>,
    ) -> Result<PlanarCubicCertificate, Failure> {
        poll(cancelled)?;
        check_options(options)?;
        let (projected, lifted, bound) =
            self.polynomial_nets(world, uv, options.distance_limit, collect)?;
        let monotone = common_monotone(&projected, &lifted).ok_or(Failure::Monotone)?;
        let certify = |net, path, cancelled: &mut _| {
            self.certify_polynomial_path(net, path, options.max_subdivisions, cancelled)
        };
        let subdivisions = [
            certify(projected, Path::WorldProjection, cancelled)?,
            certify(lifted, Path::UvLift, cancelled)?,
        ];
        poll(cancelled)?;
        Ok(PlanarCubicCertificate {
            options,
            monotone,
            subdivisions,
            distance_bound: bound,
        })
    }

    fn polynomial_nets<const N: usize>(
        &self,
        world: [[f64; 3]; N],
        uv: [[f64; 2]; N],
        limit: f64,
        collect: ProjectionCollector<N>,
    ) -> Result<(Net<N>, Net<N>, f64), Attachment> {
        let projected = collect(world.map(|pole| self.world.project(pole)))?;
        let lifted = collect(std::array::from_fn(|i| {
            self.world.proposed(uv[i], world[i])
        }))?;
        // Distance to the plane cannot exceed distance to this same-chart lift.
        let pair_bound = distance_bound(&lifted, limit)?;
        Ok((projected.map(|p| p.uv), lifted.map(|p| p.uv), pair_bound))
    }

    fn certify_polynomial_path<const N: usize>(
        &self,
        controls: Net<N>,
        path: Path,
        limit: u32,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<u32, Failure> {
        let mut pending = vec![Span {
            controls,
            parameter: [0., 1.],
            depth: 0,
        }];
        let mut subdivisions = 0;
        while let Some(span) = pending.pop() {
            poll(cancelled)?;
            if self.contains_cubic_span(&span.controls, cancelled)? {
                continue;
            }
            self.reject_cubic_ends(&span, path, cancelled)?;
            if subdivisions == limit || span.depth == MAX_DEPTH {
                return Err(Failure::Incomplete {
                    path,
                    parameter: span.parameter,
                    subdivisions,
                });
            }
            let [first, second] = span.split();
            pending.extend([second, first]);
            subdivisions += 1;
        }
        Ok(subdivisions)
    }

    fn contains_cubic_span<const N: usize>(
        &self,
        controls: &Net<N>,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<bool, Attachment> {
        if !self.outer.bernstein_contains(controls, cancelled)? {
            return Ok(false);
        }
        match &self.hole {
            Some(hole) => hole.bernstein_excludes_open(controls, cancelled),
            None => Ok(true),
        }
    }

    fn reject_cubic_ends<const N: usize>(
        &self,
        span: &Span<N>,
        path: Path,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), Failure> {
        for (i, parameter) in [0, N - 1].into_iter().zip(span.parameter) {
            let location = trim::locate(
                &self.outer,
                self.hole.as_ref(),
                &span.controls[i],
                cancelled,
            )?;
            if matches!(location, Location::OutsideOuter | Location::HoleInterior) {
                return Err(Failure::Trim {
                    path,
                    location,
                    parameter,
                });
            }
        }
        Ok(())
    }
}

fn check_options(options: PlanarCubicOptions) -> Result<(), Failure> {
    if options.max_subdivisions > MAX_SUBDIVISIONS {
        return Err(Failure::InvalidSubdivisionLimit(options.max_subdivisions));
    }
    if !options.distance_limit.is_finite() || options.distance_limit < 0. {
        return Err(Attachment::DistanceLimit(options.distance_limit).into());
    }
    Ok(())
}

fn cubic_projection(
    [a, b, c, d]: [Result<Projection, Attachment>; 4],
) -> Result<[Projection; 4], Attachment> {
    Ok([a?, b?, c?, d?])
}

fn quadratic_projection(
    [a, b, c]: [Result<Projection, Attachment>; 3],
) -> Result<[Projection; 3], Attachment> {
    Ok([a?, b?, c?])
}

fn distance_bound(controls: &[Projection], limit: f64) -> Result<f64, Attachment> {
    let cap = R::from_finite(limit);
    let mut maximum = R::from_finite(0.);
    for control in controls {
        maximum = maximum.max(control.distance_squared.clone());
    }
    // Both curves have the same nonnegative Bernstein weights. Their residual net
    // bounds the pointwise Euclidean norm without a point-to-curve search.
    let bound = if maximum.sign() == Ordering::Equal {
        Some(0.)
    } else {
        outward_sqrt(&maximum)
    };
    if maximum > cap.multiply(&cap) {
        return Err(Attachment::OffSupport { bound, limit });
    }
    bound.ok_or(Attachment::Unrepresentable)
}

fn common_monotone<const N: usize>(
    projected: &Net<N>,
    lifted: &Net<N>,
) -> Option<PlanarMonotoneAxis> {
    use PlanarMonotoneAxis::*;
    for (axis, signs) in [[IncreasingU, DecreasingU], [IncreasingV, DecreasingV]]
        .into_iter()
        .enumerate()
    {
        let sign = |net: &Net<N>| net[1][axis].cmp(&net[0][axis]);
        let direction = sign(projected);
        let directed = |net: &Net<N>| {
            net.windows(2)
                .all(|pair| pair[1][axis].cmp(&pair[0][axis]) == direction)
        };
        if direction != Ordering::Equal && directed(projected) && directed(lifted) {
            return Some(signs[usize::from(direction == Ordering::Less)]);
        }
    }
    None
}

struct Span<const N: usize> {
    controls: Net<N>,
    parameter: [f64; 2],
    depth: u8,
}

impl<const N: usize> Span<N> {
    fn split(self) -> [Self; 2] {
        let [left, right] = bisect(self.controls);
        let [lo, hi] = self.parameter;
        let mid = (lo + hi) * 0.5;
        let depth = self.depth + 1;
        [
            Self {
                controls: left,
                parameter: [lo, mid],
                depth,
            },
            Self {
                controls: right,
                parameter: [mid, hi],
                depth,
            },
        ]
    }
}

fn bisect<const N: usize>(mut work: Net<N>) -> [Net<N>; 2] {
    let mut left = work.clone();
    let mut right = work.clone();
    for i in 1..N {
        for start in 0..(N - i) {
            work[start] = midpoint(&work[start], &work[start + 1]);
        }
        left[i] = work[0].clone();
        right[N - 1 - i] = work[N - 1 - i].clone();
    }
    [left, right]
}

fn midpoint(a: &Uv, b: &Uv) -> Uv {
    let half = R::from_finite(0.5);
    std::array::from_fn(|i| a[i].add(&b[i]).multiply(&half))
}
