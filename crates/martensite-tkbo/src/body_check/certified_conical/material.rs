// SPDX-License-Identifier: GPL-2.0-only
//! Issued selected material preserves the executed bounded-face authority.
use crate::profile_extrusion::{
    PlanarIntegralEvidence, PlanarRegionSet, ProfileLoop, RegionId, SketchProfile,
};
use martensite_tkernel::standard::Failure;

/// An executed bounded UV material, including its directed outer loop and holes.
/// Example: `let material = CertifiedPlanarMaterial::from_region(&regions, id)?;`.
#[derive(Clone, Debug)]
pub struct CertifiedPlanarMaterial {
    pub(super) program: PlanarIntegralEvidence,
    pub(super) outer: ProfileLoop,
    pub(super) holes: Vec<ProfileLoop>,
}
impl CertifiedPlanarMaterial {
    /// Retain an actually selected V5 face without repeating its planar proof.
    /// Example: `let selected = CertifiedPlanarMaterial::from_region(&set, region.id())?;`.
    pub fn from_region(set: &PlanarRegionSet, id: RegionId) -> Result<Self, Failure> {
        let region = set
            .region(id)
            .ok_or_else(|| rejected("selected region missing"))?;
        let program = set
            .integral_program_evidence()
            .ok_or_else(|| rejected("V5 program missing"))?;
        if !matches!(program.requested_proof_version().request_version(), 5 | 6)
            || !program.matches_profile(&program.source)
        {
            return Err(rejected("issued planar source/lineage mismatch"));
        }
        Ok(Self {
            program: program.clone(),
            outer: region.outer().clone(),
            holes: region.holes().to_vec(),
        })
    }
    /// Replay the complete original polynomial profile binding.
    /// Example: `assert!(material.matches_profile(&profile));`.
    pub fn matches_profile(&self, profile: &SketchProfile) -> bool {
        self.program.matches_profile(profile)
    }
}
fn rejected(cause: &str) -> Failure {
    Failure::with_type(
        "CertifiedPlanarMaterialRejected",
        cause,
        "expected an issued source-bound V5 bounded material",
    )
}

impl CertifiedPlanarMaterial {
    pub(super) fn source_bits(&self) -> Vec<u64> {
        let version = self.program.requested_proof_version();
        let mut bits = vec![
            u64::from(version.request_version()),
            self.program.entry_allowance(),
            self.program.granted_subintervals(),
            self.program.integration_subintervals(),
        ];
        if let Some(options) = version.integral_options() {
            bits.extend([
                u64::from(options.max_components),
                options.max_total_subintervals,
            ]);
        }
        bits.extend(self.program.source_bits.iter().copied());
        bits
    }
    pub(super) fn profile(&self) -> &SketchProfile {
        &self.program.source
    }
    pub(super) fn loops(&self) -> impl Iterator<Item = &ProfileLoop> {
        std::iter::once(&self.outer).chain(&self.holes)
    }
    pub(super) fn loop_role(&self, actual: &ProfileLoop) -> Option<usize> {
        self.loops()
            .position(|expected| reversed_cycle(&expected.uses, &actual.uses))
    }
}
fn reversed_cycle(
    expected: &[crate::profile_extrusion::LoopUse],
    actual: &[crate::profile_extrusion::LoopUse],
) -> bool {
    if expected.is_empty() || expected.len() != actual.len() {
        return false;
    }
    (0..actual.len()).any(|offset| {
        expected.iter().rev().enumerate().all(|(index, entry)| {
            let candidate = actual[(index + offset) % actual.len()];
            entry.curve == candidate.curve && entry.reversed != candidate.reversed
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile_extrusion::{LoopUse, ProfileCurveId};
    fn cycle() -> Vec<LoopUse> {
        (1..=3)
            .map(|id| LoopUse {
                curve: ProfileCurveId(id),
                reversed: false,
            })
            .collect()
    }
    #[test]
    fn coordinate_swap_requires_complete_reversed_directed_cycle() {
        let expected = cycle();
        let mut actual: Vec<_> = expected
            .iter()
            .rev()
            .map(|entry| LoopUse {
                reversed: true,
                ..*entry
            })
            .collect();
        actual.rotate_left(1);
        assert!(reversed_cycle(&expected, &actual));
        actual[0].reversed = false;
        assert!(!reversed_cycle(&expected, &actual));
        actual.pop();
        assert!(!reversed_cycle(&expected, &actual));
    }
    #[test]
    fn original_direction_and_duplicate_identity_do_not_select_filled_material() {
        let expected = cycle();
        assert!(!reversed_cycle(&expected, &expected));
        let actual = vec![
            LoopUse {
                curve: ProfileCurveId(1),
                reversed: true
            };
            3
        ];
        assert!(!reversed_cycle(&expected, &actual));
        assert!(!reversed_cycle(&[], &[]));
    }
}
