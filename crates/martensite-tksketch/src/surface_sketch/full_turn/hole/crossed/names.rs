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

/// Identity of a unique, transverse authored crossing, independent of B-rep ordering.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct PeriodicCrossingId {
    /// Original contour curve.
    pub curve: SurfaceCurveId,
    /// Cut lift relative to the authored zero-arc meridian.
    pub turn: i64,
    /// Traversal direction across the lift.
    pub transition: SheetTransition,
}

/// Numerical enclosure of the exact source intersection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PeriodicCrossing {
    /// Persistent authored address.
    pub id: PeriodicCrossingId,
    /// Outward enclosure in the original directed curve's normalized `[0,1]` interval.
    pub normalized_parameter: [f64; 2],
}

/// One endpoint of a retained interval on the original directed contour.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum PeriodicSourceEnd {
    /// Original starting vertex.
    Start,
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

pub(super) fn crossings(hole: &CylinderBandHole) -> Checked<[PeriodicCrossing; 2]> {
    let [a, b] = hole.arc_length;
    let width = b - a;
    let lower = crossing(hole.boundaries[0], SheetTransition::Increasing, -a, width);
    let upper = crossing(hole.boundaries[2], SheetTransition::Decreasing, b, width);
    Ok([lower?, upper?])
}

fn crossing(
    curve: SurfaceCurveId,
    transition: SheetTransition,
    offset: f64,
    width: f64,
) -> Checked<PeriodicCrossing> {
    // The real denominator is the exact difference of the input binary64 numbers.
    // The enclosing predecessor/successor make both final quotients outward.
    let low = (offset / width.next_up()).next_down();
    let high = (offset / width.next_down()).next_up();
    if !low.is_finite() || !high.is_finite() || !(0. < low && low <= high && high < 1.) {
        return Err(incomplete(
            "interior directed aperture source parameter",
            format!("{curve:?}: {low}..{high}"),
        ));
    }
    Ok(PeriodicCrossing {
        id: PeriodicCrossingId {
            curve,
            turn: 0,
            transition,
        },
        normalized_parameter: [low, high],
    })
}

impl PeriodicDepth {
    pub(in crate::surface_sketch) fn index(self) -> usize {
        usize::from(self == Self::Last)
    }
}
