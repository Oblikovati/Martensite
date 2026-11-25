// SPDX-License-Identifier: GPL-2.0-only
//! Issued selected material preserves the executed bounded-face authority.
mod known;
use crate::profile_extrusion::{
    PlanarIntegralEvidence, PlanarProofVersion, PlanarRegion, PlanarRegionSet, ProfileCurve,
    ProfileCurveId, ProfileLoop, RegionId, Resolution, SketchProfile,
};
pub use known::family::{CertifiedDirectedPlanarFamily, DirectedPlanarFace};
pub use known::{
    DirectedCapCause, DirectedCapFailure, DirectedCapPairCause, DirectedCapPolicy, DirectedCapWork,
};
use martensite_tkernel::standard::Failure;
use std::collections::BTreeMap;
use std::sync::Arc;

/// An executed bounded UV material, including its directed outer loop and holes.
/// Example: `let material = CertifiedPlanarMaterial::from_region(&regions, id)?;`.
#[derive(Clone, Debug)]
pub struct CertifiedPlanarMaterial(Arc<SelectedMaterial>);

#[derive(Debug)]
struct SelectedMaterial {
    authority: Arc<MaterialAuthority>,
    outer: ProfileLoop,
    holes: Vec<ProfileLoop>,
}
#[derive(Debug)]
struct MaterialAuthority {
    proof: IssuedMaterial,
    curves: BTreeMap<ProfileCurveId, usize>,
    periodic: bool,
}
#[derive(Debug)]
enum IssuedMaterial {
    Program(PlanarIntegralEvidence),
    Directed {
        source: SketchProfile,
        bits: Arc<[u64]>,
        work: [u64; 3],
    },
}
impl IssuedMaterial {
    fn profile(&self) -> &SketchProfile {
        match self {
            Self::Program(program) => &program.source,
            Self::Directed { source, .. } => source,
        }
    }
}

impl CertifiedPlanarMaterial {
    /// Retain an actually selected V5 face without repeating its planar proof.
    /// Example: `let selected = CertifiedPlanarMaterial::from_region(&set, region.id())?;`.
    pub fn from_region(set: &PlanarRegionSet, id: RegionId) -> Result<Self, Failure> {
        let region = set
            .region(id)
            .ok_or_else(|| rejected("selected region missing"))?;
        let authority = MaterialAuthority::capture(set, false)?;
        Ok(Self::selected(region, &authority))
    }

    /// Retain actual selected material in reflected coordinates `(angle, -slant)`.
    /// Reuse one verified program and curve index for the complete ordered selection.
    /// Example: `let materials = CertifiedPlanarMaterial::from_periodic_regions(&set, &ids)?;`.
    pub fn from_periodic_regions(
        set: &PlanarRegionSet,
        ids: &[RegionId],
    ) -> Result<Vec<Self>, Failure> {
        let regions: BTreeMap<_, _> = set
            .regions()
            .iter()
            .map(|region| (region.id(), region))
            .collect();
        let authority = MaterialAuthority::capture(set, true)?;
        ids.iter()
            .map(|id| {
                regions
                    .get(id)
                    .map(|region| Self::selected(region, &authority))
                    .ok_or_else(|| rejected("selected periodic region missing"))
            })
            .collect()
    }

    /// Prove explicitly ordered, nonperiodic polynomial cycles before issuing opaque material.
    /// Example: `CertifiedPlanarMaterial::from_directed_cycles_v1(profile, outer, holes, resolution, policy, || false)?`.
    pub fn from_directed_cycles_v1(
        profile: SketchProfile,
        outer: ProfileLoop,
        holes: Vec<ProfileLoop>,
        resolution: Resolution,
        policy: DirectedCapPolicy,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<(Self, DirectedCapWork), DirectedCapFailure> {
        let mut stopped = false;
        let mut poll = || {
            stopped |= cancelled();
            stopped
        };
        let sealed = (|| {
            if poll() {
                return Err(known::unobserved(DirectedCapCause::Cancelled));
            }
            known::preflight(&profile, &outer, &holes, policy).map_err(known::unobserved)?;
            if poll() {
                return Err(known::unobserved(DirectedCapCause::Cancelled));
            }
            crate::profile_extrusion::admit_profile(&profile, resolution)
                .map_err(|cause| known::unobserved(DirectedCapCause::Profile(Box::new(cause))))?;
            known::seal(&profile, &outer, &holes, resolution, policy, &mut poll)
        })()
        .map_err(|mut failure| {
            failure.cancelled = stopped;
            failure
        })?;
        let curves = profile
            .curves
            .iter()
            .enumerate()
            .map(|(i, c)| (c.id(), i))
            .collect();
        let authority = Arc::new(MaterialAuthority {
            proof: IssuedMaterial::Directed {
                source: profile,
                bits: sealed.bits,
                work: sealed.work.loan(),
            },
            curves,
            periodic: false,
        });
        let selected = Self(Arc::new(SelectedMaterial {
            authority,
            outer,
            holes,
        }));
        Ok((selected, sealed.work))
    }

    /// Rebind the original directed selected face, including each hole and the full source profile.
    /// Example: `assert!(material.matches_directed_cycles(&profile, &outer, &holes));`.
    pub fn matches_directed_cycles(
        &self,
        profile: &SketchProfile,
        outer: &ProfileLoop,
        holes: &[ProfileLoop],
    ) -> bool {
        matches!(self.0.authority.proof, IssuedMaterial::Directed { .. })
            && self.matches_profile(profile)
            && &self.0.outer == outer
            && self.0.holes == holes
    }

    fn selected(region: &PlanarRegion, authority: &Arc<MaterialAuthority>) -> Self {
        Self(Arc::new(SelectedMaterial {
            authority: Arc::clone(authority),
            outer: region.outer().clone(),
            holes: region.holes().to_vec(),
        }))
    }

    /// Replay the complete original polynomial profile binding.
    /// Example: `assert!(material.matches_profile(&profile));`.
    pub fn matches_profile(&self, profile: &SketchProfile) -> bool {
        match &self.0.authority.proof {
            IssuedMaterial::Program(program) => program.matches_profile(profile),
            IssuedMaterial::Directed { source, .. } => known::source_matches(profile, source),
        }
    }
}
impl MaterialAuthority {
    fn capture(set: &PlanarRegionSet, periodic: bool) -> Result<Arc<Self>, Failure> {
        let program = set
            .integral_program_evidence()
            .ok_or_else(|| rejected("V5 program missing"))?;
        if !matches!(program.requested_proof_version().request_version(), 5 | 6)
            || !program.matches_profile(&program.source)
        {
            return Err(rejected("issued planar source/lineage mismatch"));
        }
        let curves = program
            .source
            .curves
            .iter()
            .enumerate()
            .map(|(index, curve)| (curve.id(), index))
            .collect();
        Ok(Arc::new(Self {
            proof: IssuedMaterial::Program(program.clone()),
            curves,
            periodic,
        }))
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
    pub(super) fn source(&self) -> MaterialSource {
        let authority = &self.0.authority;
        let (version, work, coefficients) = match &authority.proof {
            IssuedMaterial::Program(program) => (
                Some(program.requested_proof_version()),
                [
                    program.entry_allowance(),
                    program.granted_subintervals(),
                    program.integration_subintervals(),
                ],
                Arc::clone(&program.source_bits),
            ),
            IssuedMaterial::Directed { bits, work, .. } => (None, *work, Arc::clone(bits)),
        };
        MaterialSource {
            version,
            work,
            coefficients,
            periodic: authority.periodic,
        }
    }
    pub(super) fn curve(&self, id: ProfileCurveId) -> Option<&ProfileCurve> {
        let authority = &self.0.authority;
        authority
            .curves
            .get(&id)
            .and_then(|&index| authority.proof.profile().curves.get(index))
    }
    pub(super) fn project(&self, [u, v]: [f64; 2]) -> [u64; 2] {
        if self.0.authority.periodic {
            [u, -v].map(f64::to_bits)
        } else {
            [v, u].map(f64::to_bits)
        }
    }
    pub(super) fn loops(&self) -> impl Iterator<Item = &ProfileLoop> {
        std::iter::once(&self.0.outer).chain(&self.0.holes)
    }
    pub(super) fn loop_role(&self, actual: &ProfileLoop) -> Option<usize> {
        self.loops()
            .position(|expected| reversed_cycle(&expected.uses, &actual.uses))
    }
}
#[derive(Clone)]
pub(super) struct MaterialSource {
    version: Option<PlanarProofVersion>,
    work: [u64; 3],
    coefficients: Arc<[u64]>,
    periodic: bool,
}
impl PartialEq for MaterialSource {
    fn eq(&self, other: &Self) -> bool {
        self.periodic == other.periodic
            && self.version == other.version
            && self.work == other.work
            && (Arc::ptr_eq(&self.coefficients, &other.coefficients)
                || self.coefficients.as_ref() == other.coefficients.as_ref())
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

    #[test]
    fn material_source_requires_equal_bits_and_work_across_independent_captures() {
        let capture = |bits: u64, consumed: u64| MaterialSource {
            version: Some(PlanarProofVersion::GeometricCompositeV6 {
                max_components: 4,
                max_total_subintervals: 16,
            }),
            work: [16, 16, consumed],
            coefficients: Arc::new([bits]),
            periodic: false,
        };
        let original = capture(0, 1);
        let rebuilt = capture(0, 1);
        assert!(!Arc::ptr_eq(&original.coefficients, &rebuilt.coefficients));
        assert!(original == rebuilt);
        assert!(original != capture((-0.0_f64).to_bits(), 1));
        assert!(original != capture(0, 2));
    }
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
