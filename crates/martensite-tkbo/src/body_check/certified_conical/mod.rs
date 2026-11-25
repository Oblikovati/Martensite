// SPDX-License-Identifier: GPL-2.0-only
//! Native binding is separate from the opaque mathematical family certificate.
mod binding;
mod census;
#[cfg(test)]
mod error_tests;
mod material;
pub use census::ConicalNativeCensus;
mod native;
mod ordered;
mod query;
use crate::profile_extrusion::ProfileCurveId;
pub use binding::{CertifiedConicalBody, bind_conical_actual_family};
use martensite_tkbrep::topods::{Face, Shape};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::predicates::roots::metric_bounds::*;
pub use material::{
    CertifiedDirectedPlanarFamily, CertifiedPlanarMaterial, DirectedCapCause, DirectedCapFailure,
    DirectedCapPairCause, DirectedCapPolicy, DirectedCapWork, DirectedPlanarFace,
};
pub use ordered::{validate_body_with_certified_conical, validate_body_with_transported_conical};
mod transport;

/// Explicit source-to-native member mapping, used only during operation qualification.
/// Example: `let captured = capture_conical_actual_family(&members)?;`.
pub struct ConicalNativeMember {
    /// Stable source member label.
    pub id: ConicalActualMemberId,
    /// Current native member root.
    pub shape: Shape,
    /// Every source side and its actual native face.
    pub sides: Vec<(ConicalActualSideId, Face)>,
    /// Every source cap and its selected material authority.
    pub caps: Vec<ConicalNativeCap>,
}
/// Source-keyed native cap and its issued compact planar material.
/// Example: pass this mapping to `capture_conical_actual_family`.
pub struct ConicalNativeCap {
    /// Stable source cap label.
    pub id: ConicalActualCapId,
    /// Actual native face.
    pub face: Face,
    /// Executed selected bounded UV material.
    pub material: CertifiedPlanarMaterial,
    /// Exhaustive selected-profile curve to source-side mapping.
    pub curves: Vec<(ProfileCurveId, ConicalActualSideId)>,
}
/// A native or source binding refusal with the actual source entities.
#[derive(Debug)]
pub struct ConicalBindingError {
    /// Owning source member, if admission reached it.
    pub member: Option<ConicalActualMemberId>,
    /// Offending side, if present.
    pub side: Option<ConicalActualSideId>,
    /// Offending cap, if present.
    pub cap: Option<ConicalActualCapId>,
    /// Original native/binding cause.
    pub cause: Box<Failure>,
}
impl std::fmt::Display for ConicalBindingError {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            output,
            "conical native binding member={:?} side={:?} cap={:?}: {}",
            self.member, self.side, self.cap, self.cause
        )
    }
}
impl std::error::Error for ConicalBindingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.cause.as_ref())
    }
}
/// Operation-local independent capture; native handles never enter the issued body receipt.
pub struct ConicalNativeGeometry {
    members: Vec<MemberGeometry>,
    adjacent: Vec<ConicalActualAdjacent>,
    side_orientations: std::collections::BTreeMap<u64, i8>,
}
struct MemberGeometry {
    id: ConicalActualMemberId,
    sides: Vec<native::Side>,
    caps: Vec<native::Cap>,
    materials: Vec<(u64, material::MaterialSource)>,
    adjacency: std::ops::Range<usize>,
}
/// Decode every native face and incidence against explicit source/material bindings.
/// Example: `let actual = capture_conical_actual_family(&members)?;`.
pub fn capture_conical_actual_family(
    members: &[ConicalNativeMember],
) -> Result<ConicalNativeGeometry, ConicalBindingError> {
    binding::capture(members)
}
