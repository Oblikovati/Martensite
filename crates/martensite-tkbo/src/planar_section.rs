// SPDX-License-Identifier: GPL-2.0-only
//! Checked native Section between one linear convex planar face and a named work-plane.
mod failure;
mod geometry;
mod native;
mod topology;
mod types;

pub use failure::{PlanarSectionCause, PlanarSectionFailure};
pub use types::*;

use martensite_tkernel::message::ProgressRange;
use martensite_tkmath::predicates::roots::metric_bounds::{
    PlanarSectionError, PlanarSectionTrim, certify_planar_section_trim,
};

type Checked<T> = Result<T, PlanarSectionCause>;

/// Intersect one named, located face with an unbounded named work-plane.
/// Example: `section_face_with_work_plane(source, plane, options, &progress)?`.
///
/// The caller keeps the source revision stable until return. The private Section owns
/// copies of its operands and the published line contains detached value evidence only.
pub fn section_face_with_work_plane(
    source: PlanarSectionFace<'_>,
    plane: NamedSectionPlane,
    options: PlanarSectionOptions,
    progress: &ProgressRange,
) -> Result<PlanarSectionResult, PlanarSectionFailure> {
    let parents = SectionParents {
        face: source.stamp,
        work_plane: plane.stamp,
    };
    match options.behavior {
        PlanarSectionBehavior::ConvexLinearV1 => {
            intersect_convex_linear(source, plane, options, progress)
        }
    }
    .map_err(|cause| PlanarSectionFailure { parents, cause })
}

fn intersect_convex_linear(
    source: PlanarSectionFace<'_>,
    plane: NamedSectionPlane,
    options: PlanarSectionOptions,
    progress: &ProgressRange,
) -> Checked<PlanarSectionResult> {
    poll(progress)?;
    let settings = options.validate()?;
    let observed = topology::observe(source, progress)?;
    let inputs = [
        observed.support.value,
        geometry::unplaced_chart(plane.plane),
    ];
    let trim = certify_planar_section_trim(inputs, observed.reversed, &observed.border, || {
        progress.user_break()
    })
    .map_err(|cause| observed.trim_failure(cause))?;
    poll(progress)?;
    let native_builds = u8::from(matches!(trim, PlanarSectionTrim::Segment(_)));
    let outcome = qualify(trim, source.face, &observed, plane, settings, progress)?;
    poll(progress)?;
    Ok(PlanarSectionResult {
        parents: SectionParents {
            face: source.stamp,
            work_plane: plane.stamp,
        },
        face_orientation: source.face.orientation(),
        settings,
        work: PlanarSectionWork {
            trim_uses: observed.border.len(),
            captured_cells: observed.captured_cells,
            native_builds,
        },
        outcome,
    })
}

fn qualify(
    trim: PlanarSectionTrim,
    source: &martensite_tkbrep::topods::Face,
    observed: &topology::ObservedFace,
    plane: NamedSectionPlane,
    settings: PlanarSectionOptions,
    progress: &ProgressRange,
) -> Checked<PlanarSectionOutcome> {
    match trim {
        PlanarSectionTrim::Empty(reason) => Ok(PlanarSectionOutcome::Empty(reason)),
        PlanarSectionTrim::CoincidentSupports => Err(PlanarSectionCause::CoincidentSupports),
        PlanarSectionTrim::BoundaryOverlap(index) => {
            Err(PlanarSectionCause::BoundaryOverlap(observed.uses[index].id))
        }
        PlanarSectionTrim::PointOnly(index) => {
            Err(PlanarSectionCause::PointOnly(observed.vertex_origin(index)))
        }
        PlanarSectionTrim::Segment(witness) => {
            let candidate = native::run(source, observed, &witness, plane.plane, progress)?;
            let evidence = witness.certify_native(
                candidate.space,
                candidate.vertices,
                candidate.pcurves,
                settings.max_position_error,
                || progress.user_break(),
            )?;
            Ok(PlanarSectionOutcome::Segment(Box::new(
                PlanarSectionSegment {
                    native_fuzzy_length: candidate.native_fuzzy_length,
                    origin: evidence.anchors.map(|anchor| observed.origin(anchor)),
                    evidence,
                },
            )))
        }
    }
}

fn poll(progress: &ProgressRange) -> Checked<()> {
    if progress.user_break() {
        return Err(PlanarSectionCause::Cancelled);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
