// SPDX-License-Identifier: GPL-2.0-only
//! Keep symbolic source intervals independent of numeric root enclosures.
use super::*;

/// One of the two requested normal-depth endpoints.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum PeriodicDepth {
    /// First requested depth.
    First,
    /// Last requested depth.
    Last,
}

/// Direction of a transverse crossing in the authored angular lift.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum SheetTransition {
    /// The contour advances from the lower sheet to the next sheet.
    Increasing,
    /// The contour returns from the higher sheet to the preceding sheet.
    Decreasing,
}

impl SheetTransition {
    pub(in crate::surface_sketch::full_turn::hole) fn oriented(self, reversed: bool) -> Self {
        match (self, reversed) {
            (Self::Increasing, false) | (Self::Decreasing, true) => Self::Increasing,
            (Self::Decreasing, false) | (Self::Increasing, true) => Self::Decreasing,
        }
    }
}

/// Identity of a unique, transverse authored crossing, independent of B-rep ordering.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct PeriodicCrossingId {
    /// Original contour curve.
    pub curve: SurfaceCurveId,
    /// Cut lift relative to the authored zero-arc meridian.
    pub turn: i64,
    /// Zero-based ordinal among distinct roots on this authored curve and signed cut.
    pub ordinal: u8,
    /// Forward authored-curve direction across the lift.
    pub transition: SheetTransition,
}

/// Numerical enclosure of the exact source intersection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PeriodicCrossing {
    /// Persistent authored address.
    pub id: PeriodicCrossingId,
    /// Outward enclosure in the original source parameter, normalized to `[0,1]`.
    /// For a conic quarter, this is its exact stereographic parameter.
    pub normalized_parameter: [f64; 2],
    /// Exact map from the reported stereographic interval to an authored analytic angle.
    pub conic_map: Option<WrappedConicParameterMap>,
}

/// One endpoint of a retained interval on the original directed contour.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum PeriodicSourceEnd {
    /// Original starting vertex.
    Start,
    /// A fitter's exact interior dyadic endpoint in the original curve.
    Dyadic(WrappedSourcePoint),
    /// Exact interior stereographic parameter on a selected analytic conic quarter.
    ConicDyadic {
        /// Reduced dyadic coordinate on the authored quarter.
        point: WrappedSourcePoint,
        /// Exact conversion to the original native angle.
        map: WrappedConicParameterMap,
    },
    /// Certified cut parameter.
    Cut(PeriodicCrossingId),
    /// Original ending vertex.
    End,
}

/// Physical endpoint of one surviving section of the construction seam.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum PeriodicSeamEnd {
    /// The seam reaches an authored whole-band rim.
    Rim(SurfaceCurveId),
    /// The seam reaches a certified aperture crossing.
    Crossing(PeriodicCrossingId),
}

/// Complete typed identity of a native image in the split periodic band.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[non_exhaustive]
pub enum PeriodicImageKey {
    /// A one-to-one retained semantic image.
    Retained(HistoryKey),
    /// An unchanged fitted source span or source vertex.
    WrappedRetained(WrappedHistoryKey),
    /// A fragment of an authored aperture curve, in source traversal order.
    AperturePiece {
        /// Authored boundary being subdivided.
        curve: SurfaceCurveId,
        /// Requested depth surface.
        depth: PeriodicDepth,
        /// Oriented source interval, referring to the crossing report where needed.
        interval: [PeriodicSourceEnd; 2],
    },
    /// A period seam segment with two opposite native face uses.
    SeamSegment {
        /// Owner of the periodic face.
        region: SurfaceRegionId,
        /// Requested depth surface.
        depth: PeriodicDepth,
        /// Geometrically bound semantic ends, in the authored meridian direction.
        ends: [PeriodicSeamEnd; 2],
    },
    /// The vertex shared by both aperture pieces and the corresponding stock seam.
    CrossingVertex {
        /// Requested depth surface.
        depth: PeriodicDepth,
        /// Certified authored source intersection.
        crossing: PeriodicCrossingId,
    },
}

pub(super) fn crossings(
    hole: tool::Contour,
    turn: i64,
    [lower, upper]: [[f64; 2]; 2],
) -> Checked<[PeriodicCrossing; 2]> {
    let lower = crossing(hole.boundaries[0], turn, SheetTransition::Increasing, lower);
    let upper = crossing(hole.boundaries[2], turn, SheetTransition::Decreasing, upper);
    Ok([lower?, upper?])
}

fn crossing(
    curve: SurfaceCurveId,
    turn: i64,
    transition: SheetTransition,
    [low, high]: [f64; 2],
) -> Checked<PeriodicCrossing> {
    if !low.is_finite() || !high.is_finite() || !(0. < low && low <= high && high < 1.) {
        return Err(incomplete(
            "interior directed aperture source parameter",
            format!("{curve:?}: {low}..{high}"),
        ));
    }
    Ok(PeriodicCrossing {
        id: PeriodicCrossingId {
            curve,
            turn,
            ordinal: 0,
            transition,
        },
        normalized_parameter: [low, high],
        conic_map: None,
    })
}

impl PeriodicDepth {
    pub(in crate::surface_sketch) fn index(self) -> usize {
        usize::from(self == Self::Last)
    }
}
