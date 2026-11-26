// SPDX-License-Identifier: GPL-2.0-only
//! Bounded exact Bernstein trim evidence for two synchronized planar cubic paths.
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
type Net = [Uv; 4];

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
        poll(&mut cancelled)?;
        check_options(options)?;
        let (projected, lifted, bound) = self.cubic_nets(world, uv, options.distance_limit)?;
        let monotone = common_monotone(&projected, &lifted).ok_or(Failure::Monotone)?;
        let certify = |net, path, cancelled: &mut _| {
            self.certify_cubic_path(net, path, options.max_subdivisions, cancelled)
        };
        let subdivisions = [
            certify(projected, Path::WorldProjection, &mut cancelled)?,
            certify(lifted, Path::UvLift, &mut cancelled)?,
        ];
        poll(&mut cancelled)?;
        Ok(PlanarCubicCertificate {
            options,
            monotone,
            subdivisions,
            distance_bound: bound,
        })
    }

    fn cubic_nets(
        &self,
        world: [[f64; 3]; 4],
        uv: [[f64; 2]; 4],
        limit: f64,
    ) -> Result<(Net, Net, f64), Attachment> {
        let projected = array_result(world.map(|pole| self.world.project(pole)))?;
        let lifted = array_result(std::array::from_fn(|i| {
            self.world.proposed(uv[i], world[i])
        }))?;
        // Distance to the plane cannot exceed distance to this same-chart lift.
        let pair_bound = distance_bound(&lifted, limit)?;
        Ok((projected.map(|p| p.uv), lifted.map(|p| p.uv), pair_bound))
    }

    fn certify_cubic_path(
        &self,
        controls: Net,
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

    fn contains_cubic_span(
        &self,
        controls: &Net,
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

    fn reject_cubic_ends(
        &self,
        span: &Span,
        path: Path,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), Failure> {
        for (i, parameter) in [0, 3].into_iter().zip(span.parameter) {
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

fn array_result<T>([a, b, c, d]: [Result<T, Attachment>; 4]) -> Result<[T; 4], Attachment> {
    Ok([a?, b?, c?, d?])
}

fn distance_bound(controls: &[Projection; 4], limit: f64) -> Result<f64, Attachment> {
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

fn common_monotone(projected: &Net, lifted: &Net) -> Option<PlanarMonotoneAxis> {
    use PlanarMonotoneAxis::*;
    for (axis, signs) in [[IncreasingU, DecreasingU], [IncreasingV, DecreasingV]]
        .into_iter()
        .enumerate()
    {
        let sign = |net: &Net| net[1][axis].cmp(&net[0][axis]);
        let direction = sign(projected);
        let directed = |net: &Net| {
            net.windows(2)
                .all(|pair| pair[1][axis].cmp(&pair[0][axis]) == direction)
        };
        if direction != Ordering::Equal && directed(projected) && directed(lifted) {
            return Some(signs[usize::from(direction == Ordering::Less)]);
        }
    }
    None
}

struct Span {
    controls: Net,
    parameter: [f64; 2],
    depth: u8,
}

impl Span {
    fn split(self) -> [Self; 2] {
        let [a, b, c, d] = self.controls;
        let [ab, bc, cd] = [midpoint(&a, &b), midpoint(&b, &c), midpoint(&c, &d)];
        let [abc, bcd] = [midpoint(&ab, &bc), midpoint(&bc, &cd)];
        let middle = midpoint(&abc, &bcd);
        let [lo, hi] = self.parameter;
        let mid = (lo + hi) * 0.5;
        let depth = self.depth + 1;
        [
            Self {
                controls: [a, ab, abc, middle.clone()],
                parameter: [lo, mid],
                depth,
            },
            Self {
                controls: [middle, bcd, cd, d],
                parameter: [mid, hi],
                depth,
            },
        ]
    }
}

fn midpoint(a: &Uv, b: &Uv) -> Uv {
    let half = R::from_finite(0.5);
    std::array::from_fn(|i| a[i].add(&b[i]).multiply(&half))
}
