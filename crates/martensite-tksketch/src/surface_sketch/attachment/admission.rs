//! Native traversal establishes membership only; caller identities never come from its order.
use super::*;
use martensite_tkbrep::{brep::access, brep_adaptor::BRepAdaptorSurface, topexp::Explorer};
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
/// Reject raw NaN, infinite, or negative stored tolerances before `BinShapeSet`
/// normalizes them to `Precision::Confusion()` (#513).
pub(super) fn raw_tolerances(attempt: &Attempt, body: &Shape) -> Checked<()> {
    for kind in [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face] {
        let mut explorer = attempt.native(
            "raw tolerance scan",
            Explorer::new(body, kind, ShapeEnum::Shape),
        )?;
        while explorer.more() {
            let entity = explorer.current();
            let raw = match kind {
                ShapeEnum::Vertex => attempt
                    .native(
                        "raw vertex tolerance",
                        access::vertex_tshape(entity, "attachment"),
                    )?
                    .tolerance(),
                ShapeEnum::Edge => attempt
                    .native(
                        "raw edge tolerance",
                        access::edge_tshape(entity, "attachment"),
                    )?
                    .tolerance(),
                ShapeEnum::Face => attempt
                    .native(
                        "raw face tolerance",
                        access::face_tshape(entity, "attachment"),
                    )?
                    .tolerance(),
                _ => 0.0,
            };
            if !raw.is_finite() || raw < 0.0 {
                return Err(attempt.reject(AttachmentFailureCause::Resolution {
                    field: "raw_tolerance",
                    value: raw,
                }));
            }
            attempt.native("next tolerance entity", explorer.next())?;
        }
    }
    Ok(())
}

pub(super) fn validation(
    attempt: &Attempt,
    body: &Shape,
    resolution: &Resolution,
) -> Checked<ValidationReport> {
    raw_tolerances(attempt, body)?;
    // #509: Report caps are nonfatal observations; run required validity stages with
    // NoLimit so topology/geometry/SI/consistency defects still fail, then collect
    // any ToleranceAboveBudget findings under the caller's Report cap.
    if matches!(
        resolution.cap,
        martensite_tkbo::profile_extrusion::ToleranceCap::Report(_)
    ) {
        let mut required = *resolution;
        required.cap = martensite_tkbo::profile_extrusion::ToleranceCap::NoLimit;
        let required_report =
            martensite_tkbo::body_check::validate_body(body, &required).map_err(|cause| {
                attempt.reject(AttachmentFailureCause::Resolution {
                    field: cause.field,
                    value: cause.value,
                })
            })?;
        if !required_report.is_valid() {
            return Err(attempt.reject(AttachmentFailureCause::Validation(Box::new(
                required_report,
            ))));
        }
        return martensite_tkbo::body_check::validate_body(body, resolution).map_err(|cause| {
            attempt.reject(AttachmentFailureCause::Resolution {
                field: cause.field,
                value: cause.value,
            })
        });
    }
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
