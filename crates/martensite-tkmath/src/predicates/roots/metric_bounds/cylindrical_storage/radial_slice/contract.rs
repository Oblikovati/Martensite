// SPDX-License-Identifier: GPL-2.0-only
use super::*;

/// Common-height ruled patch and a direction in its developed `(radius * angle, height)` chart.
#[derive(Clone, Copy)]
pub struct CylindricalSliceQuery<'a> {
    /// Original rational Bézier rails in first-to-far depth order, in the +Z frame at the origin.
    pub rails: [MetricBezier<'a, 3>; 2],
    /// Declared direction of radial motion from first to far.
    pub sense: CylindricalRadialSense,
    /// Positive constant multiplier on angular coordinates.
    pub chart_radius: f64,
    /// Nonzero finite vector for the fixed-radius projection.
    pub direction: [f64; 2],
    /// Finite closed normalized-depth interval. Extrapolation is explicitly included in this proof.
    pub depth: [f64; 2],
    /// Explicit arithmetic limits. The proof also requires degree at most twelve.
    pub options: CylindricalStorageOptions,
}

/// Whole-domain directional slope receipt bound to the supplied raw rail and query inputs.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalSlice {
    source: Vec<u64>,
    sense: CylindricalRadialSense,
    direction: [u64; 2],
    depth: [u64; 2],
    lower: [f64; 2],
    options: CylindricalStorageOptions,
    operations: u32,
}

impl CertifiedCylindricalSlice {
    pub(super) fn new(query: CylindricalSliceQuery<'_>, lower: [f64; 2], operations: u32) -> Self {
        Self {
            source: source(query),
            sense: query.sense,
            direction: query.direction.map(f64::to_bits),
            depth: query.depth.map(f64::to_bits),
            lower,
            options: query.options,
            operations,
        }
    }

    /// Strict positive projected derivative lower bound. Example: `proof.derivative_lower()`.
    pub fn derivative_lower(&self) -> f64 {
        self.lower[0]
    }

    /// Signed squared-radius derivative lower bound across the declared depth interval.
    /// Example: `proof.signed_squared_radius_derivative_lower()`.
    pub fn signed_squared_radius_derivative_lower(&self) -> f64 {
        self.lower[1]
    }

    /// Original unmodified work limits. Example: `proof.requested_options()`.
    pub fn requested_options(&self) -> CylindricalStorageOptions {
        self.options
    }

    /// Charged interval operations. Example: `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }

    /// Transfer this exact signed radial bound to the unit-depth patch in both depth directions.
    /// The complete unit interval must lie inside the original proved interval. The present
    /// query must retain the identical rails, chart and direction. Reversing the depth parameter
    /// and the signed sense together preserves the bound: d|S(1-s)|²/ds = -d|S|²/ds at 1-s.
    /// Example: `let [from_near, from_far] = proof.unit_radial_sweeps(query)?;`.
    pub fn unit_radial_sweeps(
        &self,
        query: CylindricalSliceQuery<'_>,
    ) -> Option<[CertifiedCylindricalRadialSweep; 2]> {
        if query.depth[0] > 0. || query.depth[1] < 1. || !self.matches_source(query) {
            return None;
        }
        let first = CylindricalRadialSweepQuery {
            rails: query.rails,
            axis_origin: [0.; 2],
            sense: query.sense,
            minimum: 0.,
            options: self.options,
        };
        let last = CylindricalRadialSweepQuery {
            rails: [query.rails[1], query.rails[0]],
            sense: match query.sense {
                CylindricalRadialSense::Outward => CylindricalRadialSense::Inward,
                CylindricalRadialSense::Inward => CylindricalRadialSense::Outward,
            },
            ..first
        };
        Some([first, last].map(|source| {
            CertifiedCylindricalRadialSweep::new(
                source,
                self.signed_squared_radius_derivative_lower(),
                self.operations,
            )
        }))
    }

    /// Compare exact source words independently of proof work. Example: `proof.matches_source(query)`.
    pub fn matches_source(&self, query: CylindricalSliceQuery<'_>) -> bool {
        admit(CylindricalSliceQuery {
            options: self.options,
            ..query
        })
        .is_ok()
            && self.source == source(query)
            && self.sense == query.sense
            && self.direction == query.direction.map(f64::to_bits)
            && self.depth == query.depth.map(f64::to_bits)
    }
}

fn source(query: CylindricalSliceQuery<'_>) -> Vec<u64> {
    types::source_bits(query.rails, [query.chart_radius, 0.])
}

/// Admission, arithmetic or strict directional slope prevented a complete receipt.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum CylindricalSliceRefusal {
    /// Invalid explicit arithmetic limits or retained raw Bézier controls.
    Admission(CylindricalStorageFailure),
    /// Radius, nonzero direction, or ordered finite depth interval was invalid.
    Chart {
        /// Requested constant angular scale.
        radius: f64,
        /// Requested projected direction.
        direction: [f64; 2],
        /// Requested depth interval.
        depth: [f64; 2],
    },
    /// The matched rails differ in number of control points.
    RailCount([usize; 2]),
    /// The matched rails differed in raw height or effective weight.
    RailControl {
        /// Zero-based offending control point.
        index: usize,
        /// Actual heights before interpolation.
        heights: [f64; 2],
        /// Actual effective positive weights (one when omitted).
        weights: [f64; 2],
    },
    /// The source rails, ascending radii, positional receipt, or depth cover did not bind.
    RadialCoverSource {
        /// Actual ascending-radius inputs of the proposed storage certificate.
        radii: [f64; 2],
        /// Declared family-wide radial allowances used to form the common interval.
        allowance: [f64; 2],
        /// Declared depth interval.
        depth: [f64; 2],
    },
    /// At least one depth extension could not cover the common radial interval.
    RadialCoverMargin {
        /// Zero-based index in ascending-radius order.
        radius_index: usize,
        /// Conservative upper bound on the required squared-radius excursion.
        required: f64,
        /// Conservative lower bound on the achieved squared-radius excursion.
        achieved: f64,
    },
    /// Bounded arithmetic stopped, including an unresolved signed radial denominator.
    Arithmetic(BoundedArithmeticFailure),
    /// Strict positive direction could not be certified at both depth boundaries.
    Nonpositive {
        /// Conservative lower bound of the projected derivative.
        direction: f64,
        /// Conservative lower bound of the signed squared-radius derivative.
        radial: f64,
    },
}

impl From<BoundedArithmeticFailure> for CylindricalSliceRefusal {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}

impl From<CylindricalStorageFailure> for CylindricalSliceRefusal {
    fn from(cause: CylindricalStorageFailure) -> Self {
        Self::Admission(cause)
    }
}

impl std::fmt::Display for CylindricalSliceRefusal {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "cylindrical fixed-radius slice refused {self:?}; expected common-height weighted rails and a strict signed directional derivative"
        )
    }
}

impl std::error::Error for CylindricalSliceRefusal {}

pub(super) fn admit(query: CylindricalSliceQuery<'_>) -> Checked<()> {
    admission::admit_policy(query.options)?;
    for (depth, rail) in query.rails.into_iter().enumerate() {
        admission::admit_curve(rail, depth, query.options.max_degree.min(12))?;
    }
    if !valid_chart(query) {
        return Err(CylindricalSliceRefusal::Chart {
            radius: query.chart_radius,
            direction: query.direction,
            depth: query.depth,
        });
    }
    common_height_and_weight(query.rails)
}

fn valid_chart(query: CylindricalSliceQuery<'_>) -> bool {
    query.chart_radius.is_finite()
        && query.chart_radius > 0.
        && query.direction.iter().all(|v| v.is_finite())
        && query.direction != [0.; 2]
        && query.depth.iter().all(|v| v.is_finite())
        && query.depth[0] < query.depth[1]
}

fn common_height_and_weight(rails: [MetricBezier<'_, 3>; 2]) -> Checked<()> {
    let count = rails.map(|rail| rail.poles.len());
    if count[0] != count[1] {
        return Err(CylindricalSliceRefusal::RailCount(count));
    }
    for index in 0..count[0] {
        let heights = rails.map(|rail| rail.poles[index][2]);
        let weights = rails.map(|rail| rail.weights.map_or(1., |weights| weights[index]));
        if heights[0].to_bits() != heights[1].to_bits() || weights[0] != weights[1] {
            return Err(CylindricalSliceRefusal::RailControl {
                index,
                heights,
                weights,
            });
        }
    }
    Ok(())
}

const _: fn() = || {
    fn shared<T: Send + Sync>() {}
    shared::<CylindricalSliceQuery<'static>>();
    shared::<CertifiedCylindricalSlice>();
    shared::<CylindricalSliceRefusal>();
};
