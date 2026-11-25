// SPDX-License-Identifier: GPL-2.0-only
//! Certify disconnected selected material from one exact family of directed cycles.
use super::*;

/// One selected developed face with ordered outer and hole cycles.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectedPlanarFace {
    /// Counterclockwise outer cycle.
    pub outer: ProfileLoop,
    /// Clockwise holes contained exclusively by this face.
    pub holes: Vec<ProfileLoop>,
}

/// Solid material selection proved over one immutable, exact polynomial source family.
#[derive(Debug)]
pub struct CertifiedDirectedPlanarFamily {
    profile: SketchProfile,
    faces: Vec<DirectedPlanarFace>,
    resolution: Resolution,
    work: DirectedCapWork,
}
impl CertifiedDirectedPlanarFamily {
    /// Issue one exact proof for the selected disjoint outer cycles and holes.
    /// Example: `let family = CertifiedDirectedPlanarFamily::issue(profile, faces, resolution, policy, || false)?;`.
    pub fn issue(
        profile: SketchProfile,
        faces: Vec<DirectedPlanarFace>,
        resolution: Resolution,
        policy: DirectedCapPolicy,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<Self, DirectedCapFailure> {
        let work = certify(&profile, &faces, resolution, policy, &mut cancelled)?;
        Ok(Self {
            profile,
            faces,
            resolution,
            work,
        })
    }

    /// Original limits, entry loan and actual work; `let work = family.work();`.
    pub fn work(&self) -> DirectedCapWork {
        self.work
    }

    /// Rebind all exact source bits, cycle roles/order and admission policy.
    /// Example: `assert!(family.matches(&profile, &faces, resolution, policy));`.
    pub fn matches(
        &self,
        profile: &SketchProfile,
        faces: &[DirectedPlanarFace],
        resolution: Resolution,
        policy: DirectedCapPolicy,
    ) -> bool {
        self.resolution == resolution
            && self.work.requested == policy
            && self.faces == faces
            && source_matches(profile, &self.profile)
    }
}

fn certify(
    profile: &SketchProfile,
    faces: &[DirectedPlanarFace],
    resolution: Resolution,
    policy: DirectedCapPolicy,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<DirectedCapWork, DirectedCapFailure> {
    let mut stopped = false;
    let mut poll = || {
        stopped |= cancelled();
        stopped
    };
    let result = certify_with_cancel(profile, faces, resolution, policy, &mut poll);
    result.map_err(|mut failure| {
        failure.cancelled = stopped;
        failure
    })
}

fn certify_with_cancel(
    profile: &SketchProfile,
    faces: &[DirectedPlanarFace],
    resolution: Resolution,
    policy: DirectedCapPolicy,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<DirectedCapWork, DirectedCapFailure> {
    poll(cancelled).map_err(unobserved)?;
    preflight_family(profile, faces, policy).map_err(unobserved)?;
    poll(cancelled).map_err(unobserved)?;
    crate::profile_extrusion::admit_profile(profile, resolution)
        .map_err(|cause| unobserved(Cause::Profile(Box::new(cause))))?;
    let sources = cycle_roles(faces);
    prove(profile, sources, resolution, policy, cancelled)
}

fn preflight_family(
    profile: &SketchProfile,
    faces: &[DirectedPlanarFace],
    policy: DirectedCapPolicy,
) -> Result<(), Cause> {
    check_policy(policy)?;
    let count = profile_count(profile)?;
    if faces.is_empty() || faces.len() > count / 2 {
        return Err(Cause::Shape {
            field: "selected faces",
            actual: faces.len(),
            expected: count / 2,
        });
    }
    let mut uses = 0usize;
    for face in faces {
        uses += face_uses(&face.outer, &face.holes, policy, count)?;
    }
    complete_uses(uses, count)?;
    polynomial(profile)
}

fn cycle_roles(
    faces: &[DirectedPlanarFace],
) -> impl Iterator<Item = (&ProfileLoop, Option<usize>)> {
    let mut index = 0;
    faces.iter().flat_map(move |face| {
        let outer = index;
        index += 1 + face.holes.len();
        std::iter::once((&face.outer, None))
            .chain(face.holes.iter().map(move |hole| (hole, Some(outer))))
    })
}
