//! Use the restored native chart when an attached profile is recomputed (#420).
use super::*;
use crate::surface_sketch::{CylinderSupport, NormalSense};
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;

impl CylinderFaceAttachment {
    /// Resolve a right-handed extrusion chart from the frozen selected cylinder.
    /// The supplied seam defines zero arc length. The normal is relative to the radius.
    /// For an indirect face, reverse native height before using this chart. Its angular seam
    /// stays in native angular coordinates. The face trims remain part of the attachment.
    /// Example: `attachment.extrusion_support(revision, 0., NormalSense::Outward)?`.
    pub fn extrusion_support(
        &self,
        revision: SupportRevision,
        seam: f64,
        normal: NormalSense,
    ) -> Checked<CylinderSupport> {
        self.resolve_chart(revision, seam, normal)
            .map(|(_, chart, _)| chart)
    }

    pub(in crate::surface_sketch) fn resolve_chart(
        &self,
        revision: SupportRevision,
        seam: f64,
        normal: NormalSense,
    ) -> Checked<(DetachedCylinderSupport, CylinderSupport, bool)> {
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
        let (chart, reflected) = support(&attempt, &selected.face, seam, normal)?;
        Ok((selected, chart, reflected))
    }
}

fn support(
    attempt: &Attempt,
    face: &Face,
    seam: f64,
    normal: NormalSense,
) -> Checked<(CylinderSupport, bool)> {
    let adaptor = attempt.native(
        "selected cylinder adaptor",
        BRepAdaptorSurface::with_face(face, false),
    )?;
    let cylinder = attempt.native("selected cylinder geometry", adaptor.cylinder())?;
    let frame = cylinder.position();
    let reflected = !attempt.native("selected cylinder handedness", cylinder.direct())?;
    let axis = *frame.direction().coordinates();
    let axis = if reflected {
        axis.map(|component| -component)
    } else {
        axis
    };
    Ok((
        CylinderSupport {
            origin: *frame.location().coordinates(),
            axis,
            radial: *frame.x_direction().coordinates(),
            radius: cylinder.radius(),
            chart_origin: seam,
            normal,
        },
        reflected,
    ))
}

#[cfg(test)]
mod tests;
