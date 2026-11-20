// SPDX-License-Identifier: GPL-2.0-only
//! The checked cap clearance separates the entire interpolation by the triangle inequality.
use super::*;
mod volume;
use crate::predicates::roots::metric_bounds::{
    CertifiedCylindricalClearance, CertifiedCylindricalPeriodicClearance, CylindricalClearanceScope,
};
pub use volume::{
    ConicalCapNormalError, ConicalCapNormalEvidence, ConicalCapVolumeError,
    ConicalCapVolumeEvidence, ConicalCapVolumeFailure,
};

/// Which complete boundary relation must stay strictly separated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConicalCapClearanceRelation {
    /// Distinct nonadjacent spans in the same unwrapped cover.
    Nonadjacent,
    /// All spans against nonzero periodic images.
    Periodic,
}

/// Failure to prove separation for the entire authored-to-cap interpolation.
#[derive(Debug)]
pub enum ConicalCapHomotopyFailure {
    /// The input join certificate used a different reference representation.
    Joins,
    /// Clearance differs in scope, periodic datum, cap control bits, or adjacency exclusions.
    Binding(ConicalCapClearanceRelation),
    /// The clearance lower bound does not exceed twice the motion upper bound.
    Gap {
        /// Relation whose separation remains unproved.
        relation: ConicalCapClearanceRelation,
        /// Certified initial separation.
        available: DyadicBound,
        /// Twice the maximum certified cap-coordinate motion.
        required: DyadicBound,
    },
}
impl std::fmt::Display for ConicalCapHomotopyFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Joins => write!(out, "join certificate; expected decoded cap reference"),
            Self::Binding(relation) => write!(
                out,
                "{relation:?} clearance; expected the same reflected cap controls, adjacency exclusions and unit angular period scale"
            ),
            Self::Gap {
                relation,
                available,
                required,
            } => write!(
                out,
                "{relation:?} clearance={available}; expected separation strictly above {required}"
            ),
        }
    }
}
impl std::error::Error for ConicalCapHomotopyFailure {}

/// Proven adjacent joins and strictly positive boundary separation throughout interpolation.
/// Periodic filled-material separation also requires a checked disjoint fill at an endpoint.
pub struct ConicalCapHomotopyEvidence<'a> {
    cap: &'a CapBinding,
    margins: [DyadicBound; 2],
}
impl ConicalCapHomotopyEvidence<'_> {
    /// Maximum motion in the numerical unwrapped angle/slant coordinates.
    /// Example: `let upper = proof.motion_upper_bound();`.
    pub fn motion_upper_bound(&self) -> &DyadicBound {
        &self.cap.motion
    }

    /// Certified residual gaps for the nonadjacent and periodic relations, respectively.
    /// Example: `let [nonadjacent, periodic] = proof.separation_lower_bounds();`.
    pub fn separation_lower_bounds(&self) -> &[DyadicBound; 2] {
        &self.margins
    }
}
impl ConicalJoinsEvidence {
    /// Compose the decoded cap joins with complete cap clearance in the reflected coordinates
    /// `(angle, -slant)`. Unit angular scale gives the true period `2π`.
    /// Example: `let proof = joins.certify_cap_homotopy(&clearance, &periodic)?;`.
    pub fn certify_cap_homotopy(
        &self,
        clearance: &CertifiedCylindricalClearance,
        periodic: &CertifiedCylindricalPeriodicClearance,
    ) -> Result<ConicalCapHomotopyEvidence<'_>, ConicalCapHomotopyFailure> {
        use ConicalCapClearanceRelation::{Nonadjacent, Periodic};
        let cap = self
            .cap_charts
            .as_deref()
            .ok_or(ConicalCapHomotopyFailure::Joins)?;
        admit_clearance(clearance, cap, &self.contour_lengths)?;
        if !periodic.matches_boundary(clearance) {
            return Err(ConicalCapHomotopyFailure::Binding(Periodic));
        }
        let within = DyadicBound::from_f64(clearance.clearance_lower_bound())
            .map_err(|_| ConicalCapHomotopyFailure::Binding(Nonadjacent))?;
        let required = cap.motion.scale_pow2(1);
        let margins = [
            separation(Nonadjacent, &within, &required)?,
            separation(Periodic, periodic.separation_lower_bound(), &required)?,
        ];
        Ok(ConicalCapHomotopyEvidence { cap, margins })
    }
}

fn admit_clearance(
    clearance: &CertifiedCylindricalClearance,
    cap: &CapBinding,
    lengths: &[usize],
) -> Result<(), ConicalCapHomotopyFailure> {
    let same_chart = clearance.support_radius().to_bits() == 1_f64.to_bits()
        && clearance.angular_origin().to_bits() == 0_f64.to_bits();
    let reflected = cap
        .spans
        .iter()
        .map(|span| span.uv.iter().map(|&[u, v]| [u, v ^ (1_u64 << 63)]));
    if !same_chart
        || clearance.scope() != CylindricalClearanceScope::Within
        || !clearance
            .excluded_pairs()
            .iter()
            .copied()
            .eq(adjacencies(lengths))
        || !clearance.matches_whole_control_bits(reflected)
    {
        return Err(ConicalCapHomotopyFailure::Binding(
            ConicalCapClearanceRelation::Nonadjacent,
        ));
    }
    Ok(())
}

fn adjacencies(lengths: &[usize]) -> impl Iterator<Item = [usize; 2]> + '_ {
    lengths
        .iter()
        .scan(0, |start, &length| {
            let begin = *start;
            *start += length;
            Some((begin, length))
        })
        .flat_map(|(start, length)| {
            let last = start + length.saturating_sub(1);
            let first = (length > 1).then_some([start, start + 1]);
            let closing = (length > 2).then_some([start, last]);
            first
                .into_iter()
                .chain(closing)
                .chain((start + 1..last).map(|i| [i, i + 1]))
        })
}

fn separation(
    relation: ConicalCapClearanceRelation,
    available: &DyadicBound,
    required: &DyadicBound,
) -> Result<DyadicBound, ConicalCapHomotopyFailure> {
    if available <= required {
        return Err(ConicalCapHomotopyFailure::Gap {
            relation,
            available: available.clone(),
            required: required.clone(),
        });
    }
    Ok(available.difference(required))
}

#[cfg(test)]
mod tests;
