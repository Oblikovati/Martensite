// SPDX-License-Identifier: GPL-2.0-only
//! Actual intrinsic geometry, source ownership and complete incidence inventory.
use super::*;
/// Operation-issued member label; native identity is independently rebound.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConicalActualMemberId(#[doc = "Stable operation source label."] pub u64);
/// Operation-issued side label with a retained authored-source bijection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConicalActualSideId(#[doc = "Stable operation source label."] pub u64);
/// Operation-issued cap label; physical row identity is checked through incidence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConicalActualCapId(#[doc = "Stable operation source label."] pub u64);
/// Actual analytic cone carrier in the qualified common cardinal frame.
#[derive(Clone, Copy, Debug)]
pub struct ConicalActualCapCarrier {
    /// Stable cap source label.
    pub id: ConicalActualCapId,
    /// Owning member source label.
    pub member: ConicalActualMemberId,
    /// Origin, X, Y, axis; origin Z equals axial_origin bit for bit.
    pub frame: [[f64; 3]; 4],
    /// Actual cone radius at its reference origin.
    pub radius: f64,
    /// Actual reference-origin Z coordinate.
    pub axial_origin: f64,
    /// Actual cone semi-angle.
    pub semi_angle: f64,
}
/// Existing source-bound nearest boundary supplied to family gamma selection.
#[derive(Clone, Copy)]
pub struct ConicalDirectedGammaSpan<'a> {
    /// Side source label.
    pub id: ConicalActualSideId,
    /// Member source label.
    pub member: ConicalActualMemberId,
    /// Complete nearest query, before directed movement.
    pub previous: ConicalBoundaryQuery<'a>,
    /// Issued complete nearest proof.
    pub parent: &'a ConicalBoundaryEvidence,
    /// Issued source/domain authority.
    pub domain: &'a ConicalChartDomainEvidence,
    /// Actual cap labels in increasing physical-depth order.
    pub caps: [ConicalActualCapId; 2],
}
/// Family-wide deterministic separate-extrema bias request.
#[derive(Clone, Copy)]
pub struct ConicalDirectedGammaQuery<'a> {
    /// Every nearest source span, in deterministic operation order.
    pub spans: &'a [ConicalDirectedGammaSpan<'a>],
    /// Complete actual/planned carrier inventory, rebound after construction.
    pub caps: &'a [ConicalActualCapCarrier],
    /// Explicit source proof refinement.
    pub refinement: ConicalPrimitiveRefinement,
    /// Actual remaining arithmetic allowance.
    pub options: ConeClearanceOptions,
}
/// Declared selected contour role; TKBO separately binds the issued material proof.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConicalActualContourRole {
    /// Selected bounded outer boundary.
    Outer,
    /// Selected hole boundary.
    Hole,
}
/// Complete intrinsic cap-curve occurrence with actual native row incidence.
#[derive(Clone, Copy, Debug)]
pub struct ConicalActualUvEdge<'a> {
    /// Actual incident side label.
    pub side: ConicalActualSideId,
    /// Actual physical side row, zero or one.
    pub row: usize,
    /// Actual polynomial/rational UV control points, without sampling.
    pub poles: &'a [[f64; 2]],
    /// Actual positive weights, one per pole.
    pub weights: &'a [f64],
    /// Actual stored edge parameter range.
    pub range: [f64; 2],
    /// Native use reversal in the intrinsic face-Forward wire.
    pub reversed: bool,
}
/// Selected intrinsic wire, with complete ordered native uses.
#[derive(Clone, Copy, Debug)]
pub struct ConicalActualUvContour<'a> {
    /// Role from issued selected material, never native outer_wire.
    pub role: ConicalActualContourRole,
    /// Actual intrinsic face-Forward wire orientation, plus or minus one.
    pub wire_orientation: i8,
    /// Complete ordered curve uses.
    pub edges: &'a [ConicalActualUvEdge<'a>],
}
/// Actual cap source whose whole enclosing UV rectangle is certified.
#[derive(Clone, Copy, Debug)]
pub struct ConicalActualCapQuery<'a> {
    /// Actual analytic carrier.
    pub carrier: ConicalActualCapCarrier,
    /// Actual shell face orientation, plus or minus one.
    pub face_orientation: i8,
    /// Selected and independently rebound intrinsic contour inventory.
    pub contours: &'a [ConicalActualUvContour<'a>],
}
/// Final actual row query with an issued checked nearest-to-final transition.
#[derive(Clone, Copy)]
pub struct ConicalActualSideQuery<'a> {
    /// Stable side label.
    pub id: ConicalActualSideId,
    /// Stable owning member label.
    pub member: ConicalActualMemberId,
    /// Complete actual decoded final boundary query.
    pub current: ConicalBoundaryQuery<'a>,
    /// Issued checked transition, including actual captured row movements.
    pub transition: &'a ConicalBoundaryZRowEvidence,
    /// Matching source/domain authority.
    pub domain: &'a ConicalChartDomainEvidence,
    /// Actual cap labels at physical rows zero and one.
    pub caps: [ConicalActualCapId; 2],
    /// Actual shell face orientation, plus or minus one.
    pub orientation: i8,
}
/// Actual native shared-ruling incidence; geometry equality is checked independently.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConicalActualAdjacent {
    /// Distinct incident side labels.
    pub sides: [ConicalActualSideId; 2],
    /// Actual shared U endpoint for each side, zero or one.
    pub u_slots: [u8; 2],
}
/// Complete family numeric coverage request; TKBO supplies selected-material authority.
#[derive(Clone, Copy)]
pub struct ConicalActualPatchQuery<'a> {
    /// All actual sides.
    pub sides: &'a [ConicalActualSideQuery<'a>],
    /// All actual caps and complete intrinsic UV sources.
    pub caps: &'a [ConicalActualCapQuery<'a>],
    /// Complete actual native shared-ruling inventory.
    pub adjacent: &'a [ConicalActualAdjacent],
    /// Issued family bias authority.
    pub gamma: &'a ConicalDirectedGammaEvidence,
    /// Executed source/native primitive refinement.
    pub refinement: ConicalPrimitiveRefinement,
    /// Actual remaining arithmetic allowance.
    pub options: ConeClearanceOptions,
}
