//! Native traversal establishes membership only; caller identities never come from its order.
use super::*;
use martensite_tkbrep::{brep_adaptor::BRepAdaptorSurface, topexp::Explorer};
use martensite_tkg3d::{
    adaptor3d::Adaptor3dSurface,
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::geom_abs::SurfaceType;
pub(super) fn selection(attempt: &Attempt, body: &Shape, face: &Face) -> Checked<()> {
    face_use(attempt, body, face)?;
    cylinder(attempt, face)
}
pub(super) fn face_use(attempt: &Attempt, body: &Shape, face: &Face) -> Checked<()> {
    let (occurrences, orientation) = membership(attempt, body, face)?;
    if occurrences != 1 {
        return Err(attempt.reject(AttachmentFailureCause::Membership { occurrences }));
    }
    if orientation != Some(face.orientation()) {
        return Err(attempt.reject(AttachmentFailureCause::Orientation {
            actual: face.orientation(),
            expected: orientation,
        }));
    }
    Ok(())
}
pub(super) fn membership(
    attempt: &Attempt,
    body: &Shape,
    face: &Face,
) -> Checked<(usize, Option<Orientation>)> {
    let mut explorer = attempt.native(
        "body faces",
        Explorer::new(body, ShapeEnum::Face, ShapeEnum::Shape),
    )?;
    let (mut count, mut orientation) = (0, None);
    while explorer.more() {
        if explorer.current().is_same(face) {
            count += 1;
            orientation = Some(explorer.current().orientation());
        }
        attempt.native("next body face", explorer.next())?;
    }
    Ok((count, orientation))
}
fn cylinder(attempt: &Attempt, face: &Face) -> Checked<()> {
    let actual = surface_type(attempt, face)?;
    if actual != SurfaceType::Cylinder {
        return Err(attempt.reject(AttachmentFailureCause::Surface { actual }));
    }
    Ok(())
}
pub(super) fn surface_type(attempt: &Attempt, face: &Face) -> Checked<SurfaceType> {
    let adaptor = attempt.native(
        "selected surface",
        BRepAdaptorSurface::with_face(face, false),
    )?;
    attempt.native("surface classification", adaptor.get_type())
}
pub(super) fn validation(
    attempt: &Attempt,
    body: &Shape,
    resolution: &Resolution,
) -> Checked<ValidationReport> {
    let report = martensite_tkbo::body_check::validate_body(body, resolution).map_err(|cause| {
        attempt.reject(AttachmentFailureCause::Resolution {
            field: cause.field,
            value: cause.value,
        })
    })?;
    if !report.is_valid() {
        return Err(attempt.reject(AttachmentFailureCause::Validation(Box::new(report))));
    }
    Ok(report)
}
