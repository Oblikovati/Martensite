//! Use the restored native chart when an attached profile is recomputed (#420).
use super::*;
use crate::surface_sketch::{CylinderSupport, NormalSense};
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;

impl CylinderFaceAttachment {
    /// Resolve a right-handed extrusion chart from the frozen selected cylinder.
    /// The supplied seam defines zero arc length. The normal is relative to the radius.
    /// This returns the analytic support. The face trims remain part of the attachment.
    /// Example: `attachment.extrusion_support(revision, 0., NormalSense::Outward)?`.
    pub fn extrusion_support(
        &self,
        revision: SupportRevision,
        seam: f64,
        normal: NormalSense,
    ) -> Checked<CylinderSupport> {
        let attempt = Attempt::new(
            AttachmentOperation::Develop,
            self.record.id,
            self.record.revision,
        );
        attempt.revision(revision)?;
        if !seam.is_finite() {
            return Err(attempt.reject(AttachmentFailureCause::ChartValue {
                field: "seam angle",
                value: seam,
            }));
        }
        let selected = codec::decode(&attempt, &self.record.brep)?;
        support(&attempt, &selected.face, seam, normal)
    }
}

fn support(
    attempt: &Attempt,
    face: &Face,
    seam: f64,
    normal: NormalSense,
) -> Checked<CylinderSupport> {
    let adaptor = attempt.native(
        "selected cylinder adaptor",
        BRepAdaptorSurface::with_face(face, false),
    )?;
    let cylinder = attempt.native("selected cylinder geometry", adaptor.cylinder())?;
    let frame = cylinder.position();
    if !attempt.native("selected cylinder handedness", cylinder.direct())? {
        return attempt.native(
            "selected cylinder chart",
            Err(Failure::new(
                "indirect native cylinder frame; expected right-handed axis and radial directions",
            )),
        );
    }
    Ok(CylinderSupport {
        origin: *frame.location().coordinates(),
        axis: *frame.direction().coordinates(),
        radial: *frame.x_direction().coordinates(),
        radius: cylinder.radius(),
        chart_origin: seam,
        normal,
    })
}

#[cfg(test)]
mod tests;
